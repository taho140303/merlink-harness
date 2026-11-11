//! Ollama Native Adapter (`REQ-PROVIDER-001`, `§9.1.1`).
//!
//! Default endpoint: `http://127.0.0.1:11434/api/chat`.
//! Restricts connections to loopback (localhost/127.0.0.1/[::1]) by default.

use crate::providers::redaction::redact_sensitive_strings;
use crate::providers::traits::{
    LlmProvider, ProviderChatRequest, ProviderChatResponse, ProviderError, ProviderMessage,
    ProviderStreamChunk, ProviderStreamRequest, ProviderStreamSummary, ProviderToolCall,
    ProviderUsage,
};
use reqwest::Client;
use serde::Deserialize;
use std::time::Duration;
use tokio_util::sync::CancellationToken;
use url::Url;

pub const DEFAULT_OLLAMA_ENDPOINT: &str = "http://127.0.0.1:11434/api/chat";
pub const DEFAULT_CONNECT_TIMEOUT: Duration = Duration::from_secs(10);
pub const DEFAULT_FIRST_BYTE_TIMEOUT: Duration = Duration::from_secs(60);
pub const DEFAULT_IDLE_STREAM_TIMEOUT: Duration = Duration::from_secs(30);

/// Check if a URL points to a local loopback address (`localhost`, `127.0.0.1`, `::1`).
pub fn is_loopback_url(endpoint: &str) -> bool {
    if let Ok(parsed) = Url::parse(endpoint) {
        if let Some(host) = parsed.host_str() {
            return host == "localhost" || host == "127.0.0.1" || host == "::1" || host == "[::1]";
        }
    }
    false
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
struct OllamaResponseToolCallFunction {
    name: String,
    arguments: serde_json::Value,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
struct OllamaResponseToolCall {
    #[serde(default)]
    id: Option<String>,
    function: OllamaResponseToolCallFunction,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
struct OllamaResponseMessage {
    #[serde(default)]
    content: Option<String>,
    #[serde(default)]
    tool_calls: Option<Vec<OllamaResponseToolCall>>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
struct OllamaChatResponseBody {
    #[serde(default)]
    message: Option<OllamaResponseMessage>,
    #[serde(default)]
    done: bool,
    #[serde(default)]
    prompt_eval_count: Option<u32>,
    #[serde(default)]
    eval_count: Option<u32>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
struct OllamaStreamChunkBody {
    #[serde(default)]
    message: Option<OllamaResponseMessage>,
    #[serde(default)]
    done: bool,
    #[serde(default)]
    prompt_eval_count: Option<u32>,
    #[serde(default)]
    eval_count: Option<u32>,
}

/// Ollama Local Model Adapter.
#[derive(Debug, Clone)]
pub struct OllamaProvider {
    endpoint: String,
    client: Client,
    allow_non_loopback: bool,
}

impl Default for OllamaProvider {
    fn default() -> Self {
        Self::new(DEFAULT_OLLAMA_ENDPOINT, false)
    }
}

impl OllamaProvider {
    pub fn new(endpoint: impl Into<String>, allow_non_loopback: bool) -> Self {
        let ep = endpoint.into();
        let client = Client::builder()
            .connect_timeout(DEFAULT_CONNECT_TIMEOUT)
            .timeout(DEFAULT_FIRST_BYTE_TIMEOUT)
            .build()
            .unwrap_or_else(|_| Client::new());

        Self {
            endpoint: ep,
            client,
            allow_non_loopback,
        }
    }

    pub fn with_client(
        endpoint: impl Into<String>,
        client: Client,
        allow_non_loopback: bool,
    ) -> Self {
        Self {
            endpoint: endpoint.into(),
            client,
            allow_non_loopback,
        }
    }

    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    fn validate_endpoint(&self) -> Result<(), ProviderError> {
        if !self.allow_non_loopback && !is_loopback_url(&self.endpoint) {
            return Err(ProviderError::Validation(format!(
                "Ollama endpoint '{}' rejected: only loopback addresses (localhost/127.0.0.1/[::1]) are permitted by default",
                self.endpoint
            )));
        }
        Ok(())
    }

    /// Build JSON payload for Ollama chat request (exposed for test inspections without network calls).
    pub fn build_chat_payload(req: &ProviderChatRequest) -> serde_json::Value {
        let tools_value = req.tools.as_ref().map(|tools| {
            serde_json::Value::Array(
                tools
                    .iter()
                    .map(|t| {
                        serde_json::json!({
                            "type": "function",
                            "function": {
                                "name": t.name,
                                "description": t.description,
                                "parameters": t.parameters
                            }
                        })
                    })
                    .collect(),
            )
        });

        let formatted_messages: Vec<serde_json::Value> = req
            .messages
            .iter()
            .map(|m| {
                let mut map = serde_json::Map::new();
                map.insert(
                    "role".to_string(),
                    serde_json::Value::String(m.role.clone()),
                );
                map.insert(
                    "content".to_string(),
                    serde_json::Value::String(m.content.clone()),
                );

                if let Some(ref name) = m.name {
                    map.insert("name".to_string(), serde_json::Value::String(name.clone()));
                }

                if let Some(ref tid) = m.tool_call_id {
                    map.insert(
                        "tool_call_id".to_string(),
                        serde_json::Value::String(tid.clone()),
                    );
                }

                if let Some(ref tcs) = m.tool_calls {
                    let formatted_tcs: Vec<serde_json::Value> = tcs
                        .iter()
                        .map(|tc| {
                            serde_json::json!({
                                "id": tc.id,
                                "type": "function",
                                "function": {
                                    "name": tc.name,
                                    "arguments": tc.arguments
                                }
                            })
                        })
                        .collect();
                    map.insert(
                        "tool_calls".to_string(),
                        serde_json::Value::Array(formatted_tcs),
                    );
                }

                serde_json::Value::Object(map)
            })
            .collect();

        serde_json::json!({
            "model": req.model,
            "messages": formatted_messages,
            "stream": false,
            "tools": tools_value
        })
    }
}

#[async_trait::async_trait]
impl LlmProvider for OllamaProvider {
    fn provider_id(&self) -> &'static str {
        "ollama"
    }

    async fn chat_with_tools(
        &self,
        req: &ProviderChatRequest,
        cancel_token: CancellationToken,
    ) -> Result<ProviderChatResponse, ProviderError> {
        self.validate_endpoint()?;

        let body = Self::build_chat_payload(req);

        let request = self.client.post(&self.endpoint).json(&body);

        let response = tokio::select! {
            _ = cancel_token.cancelled() => {
                return Err(ProviderError::Cancelled);
            }
            res = request.send() => {
                match res {
                    Ok(resp) => resp,
                    Err(err) => {
                        let msg = redact_sensitive_strings(&err.to_string());
                        if err.is_timeout() || err.is_connect() {
                            return Err(ProviderError::Unreachable { status: None, message: msg });
                        }
                        return Err(ProviderError::Internal(msg));
                    }
                }
            }
        };

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(ProviderError::from_http_status(
                status.as_u16(),
                &error_text,
            ));
        }

        let resp_body: OllamaChatResponseBody = response.json().await.map_err(|err| {
            ProviderError::Protocol(format!(
                "Failed to parse Ollama response: {}",
                redact_sensitive_strings(&err.to_string())
            ))
        })?;

        let mut tool_calls = Vec::new();
        let mut content = None;

        if let Some(msg) = resp_body.message {
            content = msg.content;
            if let Some(tcs) = msg.tool_calls {
                for (idx, tc) in tcs.into_iter().enumerate() {
                    let id = tc.id.unwrap_or_else(|| format!("call_{idx}"));
                    tool_calls.push(ProviderToolCall {
                        id,
                        name: tc.function.name,
                        arguments: tc.function.arguments,
                    });
                }
            }
        }

        let usage = Some(ProviderUsage {
            prompt_tokens: resp_body.prompt_eval_count.unwrap_or(0),
            completion_tokens: resp_body.eval_count.unwrap_or(0),
            total_tokens: resp_body.prompt_eval_count.unwrap_or(0)
                + resp_body.eval_count.unwrap_or(0),
        });

        let finish_reason = if tool_calls.is_empty() {
            "stop".to_string()
        } else {
            "tool_calls".to_string()
        };

        Ok(ProviderChatResponse {
            content,
            tool_calls,
            finish_reason: Some(finish_reason),
            usage,
        })
    }

    async fn stream_completion(
        &self,
        req: &ProviderStreamRequest,
        event_tx: tokio::sync::mpsc::Sender<ProviderStreamChunk>,
        cancel_token: CancellationToken,
    ) -> Result<ProviderStreamSummary, ProviderError> {
        self.validate_endpoint()?;

        let messages = match &req.messages {
            Some(m) => m.clone(),
            None => vec![ProviderMessage::user(&req.prompt)],
        };

        let body = serde_json::json!({
            "model": req.model,
            "messages": messages,
            "stream": true
        });

        let request = self.client.post(&self.endpoint).json(&body);

        let response = tokio::select! {
            _ = cancel_token.cancelled() => {
                return Err(ProviderError::Cancelled);
            }
            res = request.send() => {
                match res {
                    Ok(resp) => resp,
                    Err(err) => {
                        let msg = redact_sensitive_strings(&err.to_string());
                        if err.is_timeout() || err.is_connect() {
                            return Err(ProviderError::Unreachable { status: None, message: msg });
                        }
                        return Err(ProviderError::Internal(msg));
                    }
                }
            }
        };

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(ProviderError::from_http_status(
                status.as_u16(),
                &error_text,
            ));
        }

        let mut total_text = String::new();
        let mut chunk_count = 0;
        let mut finish_reason = None;
        let mut usage = None;

        use futures_util::StreamExt;
        let mut byte_stream = response.bytes_stream();

        let mut line_buffer = String::new();

        loop {
            let next_item = tokio::select! {
                _ = cancel_token.cancelled() => {
                    return Err(ProviderError::Cancelled);
                }
                item = byte_stream.next() => item,
            };

            match next_item {
                Some(Ok(bytes)) => {
                    let text = String::from_utf8_lossy(&bytes);
                    line_buffer.push_str(&text);

                    while let Some(pos) = line_buffer.find('\n') {
                        let line = line_buffer[..pos].trim().to_string();
                        line_buffer = line_buffer[pos + 1..].to_string();

                        if line.is_empty() {
                            continue;
                        }

                        if let Ok(chunk_body) = serde_json::from_str::<OllamaStreamChunkBody>(&line)
                        {
                            if let Some(msg) = chunk_body.message {
                                if let Some(delta) = msg.content {
                                    if !delta.is_empty() {
                                        total_text.push_str(&delta);
                                        chunk_count += 1;
                                        let _ = event_tx
                                            .send(ProviderStreamChunk {
                                                delta,
                                                finish_reason: None,
                                            })
                                            .await;
                                    }
                                }
                            }

                            if chunk_body.done {
                                finish_reason = Some("stop".to_string());
                                usage = Some(ProviderUsage {
                                    prompt_tokens: chunk_body.prompt_eval_count.unwrap_or(0),
                                    completion_tokens: chunk_body.eval_count.unwrap_or(0),
                                    total_tokens: chunk_body.prompt_eval_count.unwrap_or(0)
                                        + chunk_body.eval_count.unwrap_or(0),
                                });
                            }
                        }
                    }
                }
                Some(Err(err)) => {
                    let msg = redact_sensitive_strings(&err.to_string());
                    return Err(ProviderError::Unreachable {
                        status: None,
                        message: msg,
                    });
                }
                None => break,
            }
        }

        Ok(ProviderStreamSummary {
            total_text,
            chunk_count,
            finish_reason,
            usage,
        })
    }
}
