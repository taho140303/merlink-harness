//! OpenCode / Cloud OpenAI-Compatible Adapter (`REQ-PROVIDER-001`, `§9.1.1`).
//!
//! STRICT SECURITY INVARIANT (REQ-PROVIDER-001, P2S4-P):
//! MUST ALWAYS inject `permission: { "*": "deny" }` into every outbound request body
//! to strictly forbid remote arbitrary code/tool execution by the model server.

use crate::providers::redaction::{redact_sensitive_strings, SecretKey};
use crate::providers::traits::{
    LlmProvider, ProviderChatRequest, ProviderChatResponse, ProviderError, ProviderMessage,
    ProviderStreamChunk, ProviderStreamRequest, ProviderStreamSummary, ProviderToolCall,
    ProviderUsage,
};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use reqwest::Client;
use serde::Deserialize;
use std::time::Duration;
use tokio_util::sync::CancellationToken;

pub const DEFAULT_OPENCODE_ENDPOINT: &str = "https://api.opencode.ai/v1/chat/completions";
pub const DEFAULT_CONNECT_TIMEOUT: Duration = Duration::from_secs(10);
pub const DEFAULT_FIRST_BYTE_TIMEOUT: Duration = Duration::from_secs(60);
pub const DEFAULT_IDLE_STREAM_TIMEOUT: Duration = Duration::from_secs(30);

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
struct OpenCodeToolCallFunction {
    name: String,
    arguments: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
struct OpenCodeToolCall {
    id: String,
    #[serde(rename = "type")]
    call_type: Option<String>,
    function: OpenCodeToolCallFunction,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
struct OpenCodeChoiceMessage {
    role: Option<String>,
    content: Option<String>,
    tool_calls: Option<Vec<OpenCodeToolCall>>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
struct OpenCodeChoice {
    index: usize,
    message: Option<OpenCodeChoiceMessage>,
    finish_reason: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
struct OpenCodeUsage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
struct OpenCodeChatResponseBody {
    id: Option<String>,
    choices: Vec<OpenCodeChoice>,
    usage: Option<OpenCodeUsage>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
struct OpenCodeStreamDelta {
    content: Option<String>,
    role: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
struct OpenCodeStreamChoice {
    index: usize,
    delta: Option<OpenCodeStreamDelta>,
    finish_reason: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
struct OpenCodeStreamChunkBody {
    id: Option<String>,
    choices: Vec<OpenCodeStreamChoice>,
    usage: Option<OpenCodeUsage>,
}

/// OpenCode / Cloud OpenAI-Compatible Provider Adapter.
#[derive(Debug, Clone)]
pub struct OpenCodeProvider {
    endpoint: String,
    api_key: Option<SecretKey>,
    client: Client,
}

impl Default for OpenCodeProvider {
    fn default() -> Self {
        Self::new(DEFAULT_OPENCODE_ENDPOINT, None)
    }
}

impl OpenCodeProvider {
    pub fn new(endpoint: impl Into<String>, api_key: Option<String>) -> Self {
        let client = Client::builder()
            .connect_timeout(DEFAULT_CONNECT_TIMEOUT)
            .timeout(DEFAULT_FIRST_BYTE_TIMEOUT)
            .build()
            .unwrap_or_else(|_| Client::new());

        Self {
            endpoint: endpoint.into(),
            api_key: api_key.map(SecretKey::new),
            client,
        }
    }

    pub fn with_client(
        endpoint: impl Into<String>,
        api_key: Option<String>,
        client: Client,
    ) -> Self {
        Self {
            endpoint: endpoint.into(),
            api_key: api_key.map(SecretKey::new),
            client,
        }
    }

    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    /// Build JSON payload for OpenCode chat request.
    ///
    /// CRITICAL: Injects `permission: { "*": "deny" }` in all cases (`REQ-PROVIDER-001`, `P2S4-P`).
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

        serde_json::json!({
            "model": req.model,
            "messages": req.messages,
            "tools": tools_value,
            "temperature": req.temperature,
            "max_tokens": req.max_tokens,
            // CRITICAL INVARIANT: Disable all server-side permissions
            "permission": {
                "*": "deny"
            }
        })
    }

    /// Build JSON payload for OpenCode stream completion request.
    ///
    /// CRITICAL: Injects `permission: { "*": "deny" }` and `stream: true`.
    pub fn build_stream_payload(req: &ProviderStreamRequest) -> serde_json::Value {
        let messages = match &req.messages {
            Some(m) => m.clone(),
            None => vec![ProviderMessage::user(&req.prompt)],
        };

        serde_json::json!({
            "model": req.model,
            "messages": messages,
            "stream": true,
            "temperature": req.temperature,
            "max_tokens": req.max_tokens,
            // CRITICAL INVARIANT: Disable all server-side permissions
            "permission": {
                "*": "deny"
            }
        })
    }

    fn prepare_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        if let Some(key) = &self.api_key {
            let auth_str = format!("Bearer {}", key.expose_secret());
            if let Ok(val) = HeaderValue::from_str(&auth_str) {
                headers.insert(AUTHORIZATION, val);
            }
        }
        headers
    }
}

#[async_trait::async_trait]
impl LlmProvider for OpenCodeProvider {
    fn provider_id(&self) -> &'static str {
        "opencode"
    }

    async fn chat_with_tools(
        &self,
        req: &ProviderChatRequest,
        cancel_token: CancellationToken,
    ) -> Result<ProviderChatResponse, ProviderError> {
        let body = Self::build_chat_payload(req);
        let headers = self.prepare_headers();

        let request = self.client.post(&self.endpoint).headers(headers).json(&body);

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
            return Err(ProviderError::from_http_status(status.as_u16(), &error_text));
        }

        let resp_body: OpenCodeChatResponseBody = response.json().await.map_err(|err| {
            ProviderError::Protocol(format!(
                "Failed to parse OpenCode response: {}",
                redact_sensitive_strings(&err.to_string())
            ))
        })?;

        let choice = resp_body
            .choices
            .into_iter()
            .next()
            .ok_or_else(|| ProviderError::Protocol("Empty choices from OpenCode".to_string()))?;

        let mut tool_calls = Vec::new();
        let mut content = None;

        if let Some(msg) = choice.message {
            content = msg.content;
            if let Some(tcs) = msg.tool_calls {
                for tc in tcs {
                    let parsed_args = serde_json::from_str(&tc.function.arguments)
                        .unwrap_or_else(|_| serde_json::json!({ "raw": tc.function.arguments }));
                    tool_calls.push(ProviderToolCall {
                        id: tc.id,
                        name: tc.function.name,
                        arguments: parsed_args,
                    });
                }
            }
        }

        let usage = resp_body.usage.map(|u| ProviderUsage {
            prompt_tokens: u.prompt_tokens,
            completion_tokens: u.completion_tokens,
            total_tokens: u.total_tokens,
        });

        Ok(ProviderChatResponse {
            content,
            tool_calls,
            finish_reason: choice.finish_reason,
            usage,
        })
    }

    async fn stream_completion(
        &self,
        req: &ProviderStreamRequest,
        event_tx: tokio::sync::mpsc::Sender<ProviderStreamChunk>,
        cancel_token: CancellationToken,
    ) -> Result<ProviderStreamSummary, ProviderError> {
        let body = Self::build_stream_payload(req);
        let headers = self.prepare_headers();

        let request = self.client.post(&self.endpoint).headers(headers).json(&body);

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
            return Err(ProviderError::from_http_status(status.as_u16(), &error_text));
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

                        // SSE parsing: `data: ...`
                        let payload = if let Some(stripped) = line.strip_prefix("data:") {
                            stripped.trim()
                        } else {
                            line.as_str()
                        };

                        if payload == "[DONE]" {
                            if finish_reason.is_none() {
                                finish_reason = Some("stop".to_string());
                            }
                            break;
                        }

                        if let Ok(chunk_body) = serde_json::from_str::<OpenCodeStreamChunkBody>(payload) {
                            if let Some(ch) = chunk_body.choices.into_iter().next() {
                                if let Some(delta) = ch.delta {
                                    if let Some(content) = delta.content {
                                        if !content.is_empty() {
                                            total_text.push_str(&content);
                                            chunk_count += 1;
                                            let _ = event_tx
                                                .send(ProviderStreamChunk {
                                                    delta: content,
                                                    finish_reason: ch.finish_reason.clone(),
                                                })
                                                .await;
                                        }
                                    }
                                }
                                if ch.finish_reason.is_some() {
                                    finish_reason = ch.finish_reason;
                                }
                            }

                            if let Some(u) = chunk_body.usage {
                                usage = Some(ProviderUsage {
                                    prompt_tokens: u.prompt_tokens,
                                    completion_tokens: u.completion_tokens,
                                    total_tokens: u.total_tokens,
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
