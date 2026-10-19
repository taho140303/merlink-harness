use std::{
    collections::HashSet,
    future::Future,
    pin::Pin,
    sync::OnceLock,
};

use serde_json::{Value, json};

use crate::models::{FetchedModel, MessageEnvelope, ToolCall, TokenUsage};

pub fn parse_context_window(value: &str) -> u64 {
    let value = value.trim().to_ascii_uppercase();
    let (number, multiplier) = match value.chars().last() {
        Some('K') => (&value[..value.len() - 1], 1_000_u64),
        Some('M') => (&value[..value.len() - 1], 1_000_000_u64),
        _ => (value.as_str(), 1_u64),
    };
    number.parse::<u64>().ok().and_then(|n| n.checked_mul(multiplier)).filter(|n| *n > 0).unwrap_or(128_000)
}

pub struct AdapterReply {
    pub content: String,
    pub tokens: TokenUsage,
    /// Non-empty when the model wants tools run instead of (or before) answering.
    pub tool_calls: Vec<ToolCall>,
    /// The model's own thinking, when the provider exposes it (Sprint 11.12 · D9).
    /// Empty/whitespace reasoning is normalised to `None`.
    pub reasoning: Option<String>,
}

/// Constructors used by the agent-loop tests to script backend replies.
#[cfg(test)]
impl AdapterReply {
    pub fn text(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            tokens: TokenUsage::default(),
            tool_calls: Vec::new(),
            reasoning: None,
        }
    }

    pub fn with_tool_calls(content: impl Into<String>, tool_calls: Vec<ToolCall>) -> Self {
        Self {
            content: content.into(),
            tokens: TokenUsage::default(),
            tool_calls,
            reasoning: None,
        }
    }

    /// Test-only: the same reply, carrying reasoning content (D9).
    pub fn with_reasoning(mut self, reasoning: impl Into<String>) -> Self {
        self.reasoning = Some(reasoning.into());
        self
    }
}

/// Everything one backend round needs. Owned so the returned future can be
/// `Send` and outlive the borrow of the caller's slices.
#[derive(Clone, Debug)]
pub struct ChatRequest {
    pub kind: String,
    pub base_url: String,
    pub api_key: String,
    pub model: String,
    pub max_tokens: String,
    pub system: String,
    pub messages: Vec<MessageEnvelope>,
    pub tools: Vec<Value>,
}

pub type BoxedFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// Indirection that lets the agent loop run against a scripted backend in tests
/// without a network or an API key. Hand-rolled to avoid an `async-trait`
/// dependency.
///
/// `Send + Sync` is required because Tauri commands must return `Send` futures.
pub trait LlmBackend: Send + Sync {
    fn complete<'a>(&'a self, request: ChatRequest) -> BoxedFuture<'a, Result<AdapterReply, String>>;
}

#[derive(Default)]
pub struct HttpBackend;

impl LlmBackend for HttpBackend {
    fn complete<'a>(&'a self, request: ChatRequest) -> BoxedFuture<'a, Result<AdapterReply, String>> {
        Box::pin(async move {
            if request.base_url.trim().is_empty() {
                return Err("Provider base URL is missing".into());
            }
            match request.kind.as_str() {
                "anthropic" => call_anthropic(&request).await,
                "openai" | "custom" => call_openai(&request).await,
                other => Err(format!("Unsupported provider kind: {other}")),
            }
        })
    }
}

pub fn openai_endpoint(base_url: &str) -> String {
    let base = base_url.trim_end_matches('/');
    if base.ends_with("/chat/completions") {
        base.to_owned()
    } else {
        format!("{base}/chat/completions")
    }
}

fn anthropic_endpoint(base_url: &str) -> String {
    let base = base_url.trim_end_matches('/');
    if base.ends_with("/v1/messages") {
        base.to_owned()
    } else if base.ends_with("/v1") {
        format!("{base}/messages")
    } else {
        format!("{base}/v1/messages")
    }
}

/// Derive model discovery from the API root, including completed chat URLs.
fn models_endpoint(base_url: &str, kind: &str) -> String {
    let base = base_url.trim_end_matches('/');
    if kind == "anthropic" {
        let base = base.strip_suffix("/messages").unwrap_or(base);
        if base.ends_with("/models") {
            base.to_owned()
        } else if base.ends_with("/v1") {
            format!("{base}/models")
        } else {
            format!("{base}/v1/models")
        }
    } else {
        let base = base.strip_suffix("/chat/completions").unwrap_or(base);
        if base.ends_with("/models") {
            base.to_owned()
        } else {
            format!("{base}/models")
        }
    }
}

fn output_limit(value: &str) -> u64 {
    let trimmed = value.trim().to_uppercase();
    let (number, multiplier) = if let Some(prefix) = trimmed.strip_suffix('K') {
        (prefix, 1_000)
    } else if let Some(prefix) = trimmed.strip_suffix('M') {
        (prefix, 1_000_000)
    } else {
        (trimmed.as_str(), 1)
    };
    number
        .parse::<u64>()
        .map(|value| value.saturating_mul(multiplier).clamp(1, 128_000))
        .unwrap_or(4_000)
}

/* ------------------------------------------------------------------ */
/* Internal history -> provider wire format                            */
/* ------------------------------------------------------------------ */

/// OpenAI: assistant messages carry `tool_calls`, results come back as
/// `role: "tool"` paired by `tool_call_id`.
fn openai_turns(messages: &[MessageEnvelope]) -> Vec<Value> {
    messages
        .iter()
        .filter_map(|message| match message.role.as_str() {
            "user" | "system" => Some(json!({"role": message.role, "content": message.content})),
            "assistant" => match message.tool_calls.as_deref() {
                Some(calls) if !calls.is_empty() => Some(json!({
                    "role": "assistant",
                    "content": message.content,
                    "tool_calls": calls
                        .iter()
                        .map(|call| json!({
                            "id": call.id,
                            "type": "function",
                            "function": {
                                "name": call.name,
                                // The wire format wants a JSON *string* here.
                                "arguments": call.arguments.to_string(),
                            }
                        }))
                        .collect::<Vec<_>>(),
                })),
                _ => Some(json!({"role": "assistant", "content": message.content})),
            },
            "tool" => Some(json!({
                "role": "tool",
                "tool_call_id": message.tool_call_id.clone().unwrap_or_default(),
                "content": message.content,
            })),
            _ => None,
        })
        .collect()
}

/// Anthropic: assistant content blocks (`tool_use`) and user blocks
/// (`tool_result`). Consecutive tool results are merged into one user turn.
fn anthropic_turns(messages: &[MessageEnvelope]) -> Vec<Value> {
    let mut turns: Vec<Value> = Vec::new();
    for message in messages {
        match message.role.as_str() {
            "user" | "system" => turns.push(json!({"role": "user", "content": message.content})),
            "assistant" => {
                let mut blocks: Vec<Value> = Vec::new();
                if !message.content.is_empty() {
                    blocks.push(json!({"type": "text", "text": message.content}));
                }
                for call in message.tool_calls.as_deref().unwrap_or_default() {
                    blocks.push(json!({
                        "type": "tool_use",
                        "id": call.id,
                        "name": call.name,
                        "input": call.arguments,
                    }));
                }
                if blocks.is_empty() {
                    blocks.push(json!({"type": "text", "text": ""}));
                }
                turns.push(json!({"role": "assistant", "content": blocks}));
            }
            "tool" => {
                let block = json!({
                    "type": "tool_result",
                    "tool_use_id": message.tool_call_id.clone().unwrap_or_default(),
                    "content": message.content,
                });
                let merged = turns.last_mut().is_some_and(|last| {
                    if last["role"] != "user" || !last["content"].is_array() {
                        return false;
                    }
                    match last["content"].as_array_mut() {
                        Some(items) => {
                            items.push(block.clone());
                            true
                        }
                        None => false,
                    }
                });
                if !merged {
                    turns.push(json!({"role": "user", "content": [block]}));
                }
            }
            _ => {}
        }
    }
    turns
}

/// `tools` in OpenAI shape -> Anthropic shape (`input_schema`).
fn anthropic_tools(tools: &[Value]) -> Vec<Value> {
    tools
        .iter()
        .filter_map(|spec| {
            let function = spec.get("function")?;
            Some(json!({
                "name": function.get("name")?.as_str()?,
                "description": function.get("description").and_then(Value::as_str).unwrap_or(""),
                "input_schema": function
                    .get("parameters")
                    .cloned()
                    .unwrap_or_else(|| json!({"type": "object", "properties": {}})),
            }))
        })
        .collect()
}

/* ------------------------------------------------------------------ */
/* Response parsing (pure, so it is testable without a network)        */
/* ------------------------------------------------------------------ */

pub fn parse_openai_response(body: &Value) -> Result<AdapterReply, String> {
    let message = body
        .pointer("/choices/0/message")
        .ok_or("Provider response missing choices[0].message")?;

    // `content` is legitimately empty (or null) when the model only asks for tools.
    let content = message
        .get("content")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_owned();

    let mut tool_calls = Vec::new();
    let mut ids = HashSet::new();
    if let Some(raw_calls) = message.get("tool_calls") {
        let calls = raw_calls.as_array().ok_or("OpenAI tool_calls must be an array")?;
        for (index, call) in calls.iter().enumerate() {
            let id = call.get("id").and_then(Value::as_str)
                .filter(|value| !value.trim().is_empty())
                .ok_or_else(|| format!("OpenAI tool call {index} missing id"))?;
            if !ids.insert(id) {
                return Err(format!("OpenAI tool call {index} duplicate id {id}"));
            }
            let function = call.get("function")
                .ok_or_else(|| format!("OpenAI tool call {id} missing function.name"))?;
            let name = function.get("name").and_then(Value::as_str)
                .filter(|value| !value.trim().is_empty())
                .ok_or_else(|| format!("OpenAI tool call {id} missing function.name"))?;
            let arguments = match function.get("arguments") {
                Some(Value::String(raw)) => serde_json::from_str(raw)
                    .map_err(|error| format!("OpenAI tool call {id} invalid arguments: {error}"))?,
                Some(value @ (Value::Object(_) | Value::Array(_) | Value::Null)) => value.clone(),
                Some(_) => return Err(format!("OpenAI tool call {id} invalid arguments type")),
                None => return Err(format!("OpenAI tool call {id} missing arguments")),
            };
            tool_calls.push(ToolCall { id: id.to_owned(), name: name.to_owned(), arguments });
        }
    }

    if content.trim().is_empty() && tool_calls.is_empty() {
        return Err("Provider returned neither content nor tool calls".into());
    }

    let usage = &body["usage"];
    // Sprint 11.12 · D9: DeepSeek-compatible endpoints expose the model's
    // thinking as `reasoning_content`; a blank string means "no reasoning".
    let reasoning = message
        .get("reasoning_content")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|text| !text.is_empty())
        .map(ToOwned::to_owned);
    Ok(AdapterReply {
        content,
        tokens: TokenUsage {
            input_tokens: usage["prompt_tokens"].as_u64().unwrap_or(0),
            output_tokens: usage["completion_tokens"].as_u64().unwrap_or(0),
        },
        tool_calls,
        reasoning,
    })
}

pub fn parse_anthropic_response(body: &Value) -> Result<AdapterReply, String> {
    let blocks = body["content"]
        .as_array()
        .ok_or("Provider response missing content array")?;

    let mut text = Vec::new();
    let mut tool_calls = Vec::new();
    for block in blocks {
        match block.get("type").and_then(Value::as_str) {
            Some("text") => {
                if let Some(value) = block.get("text").and_then(Value::as_str) {
                    text.push(value.to_owned());
                }
            }
            Some("tool_use") => {
                let id = block.get("id").and_then(Value::as_str)
                    .filter(|value| !value.trim().is_empty())
                    .ok_or_else(|| format!("Anthropic tool_use block {} missing id", tool_calls.len()))?;
                let name = block.get("name").and_then(Value::as_str)
                    .filter(|value| !value.trim().is_empty())
                    .ok_or_else(|| format!("Anthropic tool_use {id} missing name"))?;
                tool_calls.push(ToolCall {
                    id: id.to_owned(),
                    name: name.to_owned(),
                    arguments: block.get("input").cloned().unwrap_or(Value::Null),
                });
            }
            _ => {}
        }
    }

    let content = text.join("\n");
    if content.trim().is_empty() && tool_calls.is_empty() {
        return Err("Provider returned no text content or tool use".into());
    }

    let usage = &body["usage"];
    Ok(AdapterReply {
        content,
        tokens: TokenUsage {
            input_tokens: usage["input_tokens"].as_u64().unwrap_or(0),
            output_tokens: usage["output_tokens"].as_u64().unwrap_or(0),
        },
        tool_calls,
        // D9: extended thinking is not enabled for Anthropic in this sprint.
        reasoning: None,
    })
}

/* ------------------------------------------------------------------ */
/* Model list parsing (pure, so it is testable without a network)      */
/* ------------------------------------------------------------------ */

/// Extracts the model list from a provider response (Sprint 11.5 · D4).
///
/// Both wire formats the app speaks put the collection under `data[]` with the
/// identifier in `id` (OpenAI `/v1/models`, Anthropic `/v1/models`), so `kind`
/// does not change the extraction — it is kept for signature symmetry with
/// [`fetch_models`] and is deliberately unused.
///
/// A bare JSON array is accepted too, because OpenAI-compatible proxies in the
/// wild often return the array without the `{"object":"list"}` envelope. Entries
/// without a usable `id` are skipped rather than failing the whole call, but an
/// empty result is an error: an empty model list is never useful to the UI.
pub fn parse_models_response(body: &Value, _kind: &str) -> Result<Vec<FetchedModel>, String> {
    let entries = if let Some(array) = body.get("data").and_then(Value::as_array) {
        array
    } else if let Some(array) = body.as_array() {
        array
    } else {
        return Err("Provider response contains no model list (expected a 'data' array)".into());
    };

    let models: Vec<FetchedModel> = entries
        .iter()
        .filter_map(|entry| {
            let id = entry.get("id").and_then(Value::as_str)?.trim();
            if id.is_empty() {
                return None;
            }
            // D2: `display_name` wins over `name`, both fall back to the id.
            let name = ["display_name", "name"]
                .iter()
                .find_map(|key| entry.get(*key).and_then(Value::as_str))
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .unwrap_or(id);
            Some(FetchedModel {
                id: id.to_owned(),
                name: name.to_owned(),
            })
        })
        .collect();

    if models.is_empty() {
        return Err("Provider returned no models".into());
    }
    Ok(models)
}

/* ------------------------------------------------------------------ */
/* HTTP calls                                                          */
/* ------------------------------------------------------------------ */

async fn checked_json(response: reqwest::Response) -> Result<Value, String> {
    let status = response.status();
    let body = response
        .text()
        .await
        .map_err(|error| format!("Unable to read provider response: {error}"))?;
    if !status.is_success() {
        let detail = serde_json::from_str::<Value>(&body)
            .ok()
            .and_then(|value| {
                value
                    .pointer("/error/message")
                    .or_else(|| value.get("message"))
                    .and_then(Value::as_str)
                    .map(ToOwned::to_owned)
            })
            .unwrap_or_else(|| "Provider rejected the request".into());
        return Err(format!(
            "Provider HTTP {status}: {}",
            detail.chars().take(300).collect::<String>()
        ));
    }
    serde_json::from_str(&body).map_err(|error| format!("Invalid provider response: {error}"))
}

static CHAT_CLIENT: OnceLock<Result<reqwest::Client, String>> = OnceLock::new();
static LIST_CLIENT: OnceLock<Result<reqwest::Client, String>> = OnceLock::new();

fn client() -> Result<reqwest::Client, String> {
    CHAT_CLIENT.get_or_init(|| reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(120))
        .build()
        .map_err(|error| error.to_string())).clone()
}

/// Model discovery retains its shorter 30-second timeout.
fn list_client() -> Result<reqwest::Client, String> {
    LIST_CLIENT.get_or_init(|| reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|error| error.to_string())).clone()
}

/// Asks the provider for its model list (Sprint 11.5 · D1/D3).
///
/// Runs in Rust on purpose: the webview would hit CORS, and the API key must stay
/// on the native side. Blank inputs are rejected before any network call so a
/// half-filled form reports a clear message instead of a connection error.
pub async fn fetch_models(
    kind: &str,
    base_url: &str,
    api_key: &str,
) -> Result<Vec<FetchedModel>, String> {
    if base_url.trim().is_empty() {
        return Err("Provider base URL is missing".into());
    }
    if api_key.trim().is_empty() {
        return Err("Provider API key is missing".into());
    }
    let endpoint = models_endpoint(base_url, kind);
    let request = match kind {
        "anthropic" => list_client()?
            .get(&endpoint)
            .header("x-api-key", api_key)
            .header("anthropic-version", "2023-06-01"),
        "openai" | "custom" => list_client()?.get(&endpoint).bearer_auth(api_key),
        other => return Err(format!("Unsupported provider kind: {other}")),
    };

    let response = request
        .send()
        .await
        .map_err(|error| format!("Model list request to {endpoint} failed: {error}"))?;
    // `checked_json` turns a non-2xx answer into "Provider HTTP <status>: <detail>",
    // which is exactly the message the dialog shows under the button (D5).
    parse_models_response(&checked_json(response).await?, kind)
}

async fn call_openai(request: &ChatRequest) -> Result<AdapterReply, String> {
    let mut turns = vec![json!({"role": "system", "content": request.system})];
    turns.extend(openai_turns(&request.messages));

    let mut body = json!({"model": request.model, "messages": turns});
    if !request.tools.is_empty() {
        body["tools"] = Value::Array(request.tools.clone());
        body["tool_choice"] = json!("auto");
    }

    let response = client()?
        .post(openai_endpoint(&request.base_url))
        .bearer_auth(&request.api_key)
        .json(&body)
        .send()
        .await
        .map_err(|error| format!("OpenAI-compatible request failed: {error}"))?;
    parse_openai_response(&checked_json(response).await?)
}

async fn call_anthropic(request: &ChatRequest) -> Result<AdapterReply, String> {
    let mut body = json!({
        "model": request.model,
        "system": request.system,
        "messages": anthropic_turns(&request.messages),
        "max_tokens": output_limit(&request.max_tokens),
    });
    if !request.tools.is_empty() {
        body["tools"] = Value::Array(anthropic_tools(&request.tools));
    }

    let response = client()?
        .post(anthropic_endpoint(&request.base_url))
        .header("x-api-key", &request.api_key)
        .header("anthropic-version", "2023-06-01")
        .json(&body)
        .send()
        .await
        .map_err(|error| format!("Anthropic request failed: {error}"))?;
    parse_anthropic_response(&checked_json(response).await?)
}

/* ------------------------------------------------------------------ */
/* Test backend                                                        */
/* ------------------------------------------------------------------ */

/// Replays canned replies so the agent loop can be exercised offline.
#[cfg(test)]
#[derive(Default)]
pub struct ScriptedBackend {
    replies: std::sync::Mutex<std::collections::VecDeque<AdapterReply>>,
    requests: std::sync::Mutex<Vec<ChatRequest>>,
}

#[cfg(test)]
impl ScriptedBackend {
    pub fn new(replies: Vec<AdapterReply>) -> Self {
        Self {
            replies: std::sync::Mutex::new(replies.into()),
            requests: std::sync::Mutex::new(Vec::new()),
        }
    }

    pub fn request_count(&self) -> usize {
        self.requests.lock().unwrap().len()
    }

    pub fn requests(&self) -> Vec<ChatRequest> {
        self.requests.lock().unwrap().clone()
    }
}

#[cfg(test)]
impl LlmBackend for ScriptedBackend {
    fn complete<'a>(&'a self, request: ChatRequest) -> BoxedFuture<'a, Result<AdapterReply, String>> {
        Box::pin(async move {
            self.requests.lock().unwrap().push(request);
            self.replies
                .lock()
                .unwrap()
                .pop_front()
                .ok_or_else(|| "ScriptedBackend ran out of scripted replies".to_string())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_context_window_k_and_m() {
        assert_eq!(parse_context_window("128K"), 128_000);
        assert_eq!(parse_context_window("1M"), 1_000_000);
        assert_eq!(parse_context_window("128000"), 128_000);
    }

    #[test]
    fn test_parse_context_window_falls_back_to_128k() {
        assert_eq!(parse_context_window("invalid"), 128_000);
        assert_eq!(parse_context_window("0"), 128_000);
    }

    fn message(id: &str, role: &str, content: &str) -> MessageEnvelope {
        MessageEnvelope::new(id, role, content)
    }

    #[test]
    fn test_output_limit_suffixes() {
        assert_eq!(output_limit("4K"), 4_000);
        assert_eq!(output_limit("320K"), 128_000);
        assert_eq!(output_limit("garbage"), 4_000);
    }

    #[test]
    fn test_anthropic_endpoint_normalizes_version() {
        assert_eq!(
            anthropic_endpoint("https://api.anthropic.com"),
            "https://api.anthropic.com/v1/messages"
        );
        assert_eq!(
            anthropic_endpoint("https://api.anthropic.com/v1"),
            "https://api.anthropic.com/v1/messages"
        );
    }

    /// A tool-only turn has empty content. Treating that as an error would break
    /// the entire loop, and real proxies also send `reasoning_content` alongside.
    #[test]
    fn test_openai_parses_tool_calls_with_empty_content() {
        let body = json!({
            "choices": [{
                "finish_reason": "tool_calls",
                "message": {
                    "role": "assistant",
                    "content": "",
                    "reasoning_content": "the model thinking out loud",
                    "tool_calls": [{
                        "id": "call_75f994f2dd751cdc",
                        "type": "function",
                        "function": {
                            "name": "super_json_set",
                            "arguments": "{\"path\":\"require.overview.name\",\"value\":\"Nửa Đêm\"}"
                        }
                    }]
                }
            }],
            "usage": {"prompt_tokens": 176, "completion_tokens": 29}
        });

        let reply = parse_openai_response(&body).unwrap();
        assert_eq!(reply.content, "");
        assert_eq!(
            reply.reasoning.as_deref(),
            Some("the model thinking out loud"),
            "the fixture already carries reasoning_content (D9)"
        );
        assert_eq!(reply.tool_calls.len(), 1);
        assert_eq!(reply.tool_calls[0].name, "super_json_set");
        assert_eq!(
            reply.tool_calls[0].arguments["path"],
            json!("require.overview.name")
        );
        assert_eq!(reply.tokens.input_tokens, 176);
    }

    /// Sprint 11.12 · D9: `reasoning_content` is captured so the UI can show the
    /// "Đã suy nghĩ" panel, and blank reasoning reads as absent.
    #[test]
    fn test_openai_parses_reasoning_content() {
        let body = json!({
            "choices": [{
                "message": {
                    "role": "assistant",
                    "content": "Đã ghi nhận.",
                    "reasoning_content": "  Người dùng nói 120 chương, tôi chỉ được ghi số chương.  ",
                }
            }]
        });
        let reply = parse_openai_response(&body).unwrap();
        assert_eq!(
            reply.reasoning.as_deref(),
            Some("Người dùng nói 120 chương, tôi chỉ được ghi số chương."),
            "reasoning content must be trimmed and kept"
        );

        // A tool-only reply can still carry reasoning.
        let tool_only = json!({
            "choices": [{
                "message": {
                    "role": "assistant",
                    "content": "",
                    "reasoning_content": "Tôi sẽ gọi super_json_set.",
                    "tool_calls": [{
                        "id": "call_1",
                        "type": "function",
                        "function": {"name": "super_json_set", "arguments": "{}"}
                    }]
                }
            }]
        });
        let reply = parse_openai_response(&tool_only).unwrap();
        assert_eq!(reply.reasoning.as_deref(), Some("Tôi sẽ gọi super_json_set."));
        assert_eq!(reply.tool_calls.len(), 1);

        // Blank / missing reasoning is `None`, never an empty panel.
        for message in [
            json!({"role": "assistant", "content": "hi", "reasoning_content": ""}),
            json!({"role": "assistant", "content": "hi", "reasoning_content": "   "}),
            json!({"role": "assistant", "content": "hi"}),
            json!({"role": "assistant", "content": "hi", "reasoning_content": null}),
        ] {
            let body = json!({"choices": [{"message": message}]});
            assert_eq!(parse_openai_response(&body).unwrap().reasoning, None);
        }

        // Anthropic leaves reasoning unset in this sprint (D9).
        let anthropic = parse_anthropic_response(&json!({
            "content": [{"type": "text", "text": "xin chào"}],
            "usage": {"input_tokens": 1, "output_tokens": 1}
        }))
        .unwrap();
        assert_eq!(anthropic.reasoning, None);
    }

    #[test]
    fn test_openai_parses_plain_text_and_rejects_empty() {
        let body = json!({"choices": [{"message": {"role": "assistant", "content": "xin chào"}}]});
        assert_eq!(parse_openai_response(&body).unwrap().content, "xin chào");

        let empty = json!({"choices": [{"message": {"role": "assistant", "content": ""}}]});
        assert!(parse_openai_response(&empty).is_err());
    }

    #[test]
    fn test_openai_turns_emit_tool_wire_format() {
        let mut assistant = message("a1", "assistant", "");
        assistant.tool_calls = Some(vec![ToolCall {
            id: "call_1".into(),
            name: "super_json_set".into(),
            arguments: json!({"path": "require.overview.name", "value": "X"}),
        }]);
        let tool = MessageEnvelope::tool_result("t1", "call_1", "super_json_set", &json!({"ok": true}));

        let turns = openai_turns(&[message("u1", "user", "hi"), assistant, tool]);
        assert_eq!(turns[0]["role"], "user");
        assert_eq!(turns[1]["tool_calls"][0]["function"]["name"], "super_json_set");
        // Arguments must be a JSON string on the wire, not an object.
        assert!(turns[1]["tool_calls"][0]["function"]["arguments"].is_string());
        assert_eq!(turns[2]["role"], "tool");
        assert_eq!(turns[2]["tool_call_id"], "call_1");
    }

    #[test]
    fn test_anthropic_turns_merge_tool_results() {
        let mut assistant = message("a1", "assistant", "đang xử lý");
        assistant.tool_calls = Some(vec![
            ToolCall {
                id: "toolu_1".into(),
                name: "super_json_get".into(),
                arguments: json!({"path": "require.overview.name"}),
            },
            ToolCall {
                id: "toolu_2".into(),
                name: "super_json_missing".into(),
                arguments: json!({}),
            },
        ]);
        let first = MessageEnvelope::tool_result("t1", "toolu_1", "super_json_get", &json!({"ok": true}));
        let second =
            MessageEnvelope::tool_result("t2", "toolu_2", "super_json_missing", &json!({"missing": []}));

        let turns = anthropic_turns(&[message("u1", "user", "hi"), assistant, first, second]);

        assert_eq!(turns.len(), 3, "two tool results must merge into one user turn");
        assert_eq!(turns[1]["content"][0]["type"], "text");
        assert_eq!(turns[1]["content"][1]["type"], "tool_use");
        assert_eq!(turns[1]["content"][1]["id"], "toolu_1");
        assert_eq!(turns[1]["content"][2]["id"], "toolu_2");
        assert_eq!(turns[2]["role"], "user");
        let results = turns[2]["content"].as_array().unwrap();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0]["type"], "tool_result");
        assert_eq!(results[0]["tool_use_id"], "toolu_1");
        assert_eq!(results[1]["tool_use_id"], "toolu_2");
    }

    #[test]
    fn test_anthropic_tool_spec_conversion() {
        let converted = anthropic_tools(&[json!({
            "type": "function",
            "function": {
                "name": "super_json_set",
                "description": "set a field",
                "parameters": {"type": "object", "properties": {"path": {"type": "string"}}}
            }
        })]);
        assert_eq!(converted[0]["name"], "super_json_set");
        assert!(converted[0].get("function").is_none());
        assert_eq!(converted[0]["input_schema"]["type"], "object");
    }

    #[test]
    fn test_parse_anthropic_response_with_tool_use() {
        let body = json!({
            "content": [
                {"type": "text", "text": "để tôi đặt tên"},
                {"type": "tool_use", "id": "toolu_1", "name": "super_json_set",
                 "input": {"path": "require.overview.name", "value": "X"}}
            ],
            "usage": {"input_tokens": 10, "output_tokens": 5}
        });
        let reply = parse_anthropic_response(&body).unwrap();
        assert_eq!(reply.content, "để tôi đặt tên");
        assert_eq!(reply.tool_calls[0].id, "toolu_1");
        assert_eq!(reply.tool_calls[0].arguments["value"], json!("X"));
        assert_eq!(reply.tokens.output_tokens, 5);
    }

    /* ---------------------------------------------------------------- */
    /* Sprint 11.5 · model list (D3/D4)                                 */
    /* ---------------------------------------------------------------- */

    /// The real shape of the dev endpoint on 127.0.0.1:8045 (`object`/`data[]`).
    #[test]
    fn test_parse_openai_models_response() {
        let body = json!({
            "object": "list",
            "data": [
                {"id": "gpt-4o", "object": "model", "created": 1706745600, "owned_by": "antigravity"},
                {"id": "gpt-4o-mini", "object": "model", "created": 1706745600},
                {"id": "claude-opus-4", "object": "model", "created": 1706745600},
            ]
        });

        let models = parse_models_response(&body, "openai").unwrap();
        assert_eq!(models.len(), 3);
        assert_eq!(models[0].id, "gpt-4o");
        // D2: no `display_name` / `name` field -> the id is the label.
        assert_eq!(models[0].name, "gpt-4o");
        assert_eq!(models[1].id, "gpt-4o-mini");
        assert_eq!(models[2].id, "claude-opus-4");
    }

    /// D2: `display_name` is the preferred label, `name` is the fallback.
    #[test]
    fn test_parse_models_response_prefers_display_name() {
        let body = json!({
            "data": [
                {"id": "claude-sonnet-4-5", "display_name": "Claude Sonnet 4.5"},
                {"id": "glm-4.6", "name": "GLM 4.6"},
                {"id": "bare-model"},
            ]
        });

        let models = parse_models_response(&body, "anthropic").unwrap();
        assert_eq!(models[0].name, "Claude Sonnet 4.5");
        assert_eq!(models[1].name, "GLM 4.6");
        assert_eq!(models[2].name, "bare-model", "falls back to the id");
    }

    /// OpenAI-compatible proxies often answer with the array itself.
    #[test]
    fn test_parse_models_response_accepts_bare_array() {
        let body = json!([{"id": "llama3.2", "name": "Llama 3.2"}, {"id": "qwen3"}]);

        let models = parse_models_response(&body, "custom").unwrap();
        assert_eq!(
            models.iter().map(|m| m.id.as_str()).collect::<Vec<_>>(),
            vec!["llama3.2", "qwen3"]
        );
        assert_eq!(models[0].name, "Llama 3.2");
    }

    /// Neither `data[]` nor a bare array: the UI must get an error, not an empty
    /// list that would silently look like "no new models".
    #[test]
    fn test_parse_models_response_rejects_garbage() {
        for body in [
            json!({}),
            json!({"object": "list"}),
            json!({"data": "not-an-array"}),
            json!({"error": {"message": "Invalid token"}}),
            json!("just a string"),
        ] {
            let error = parse_models_response(&body, "openai").unwrap_err();
            assert!(
                error.contains("no model list"),
                "unexpected error for {body}: {error}"
            );
        }
    }

    /// An entry with no usable id is skipped; the rest of the list still arrives.
    #[test]
    fn test_parse_models_response_skips_entries_without_id() {
        let body = json!({
            "data": [
                {"name": "no id at all"},
                {"id": "", "name": "empty id"},
                {"id": "   ", "name": "blank id"},
                {"id": "  gpt-4o  ", "name": "  GPT-4o  "},
                {"id": 42},
            ]
        });

        let models = parse_models_response(&body, "openai").unwrap();
        assert_eq!(models.len(), 1, "only the one real entry survives: {models:?}");
        assert_eq!(models[0].id, "gpt-4o", "ids are trimmed");
        assert_eq!(models[0].name, "GPT-4o", "names are trimmed");
    }

    #[test]
    fn test_parse_models_response_errors_on_empty_list() {
        for body in [json!({"object": "list", "data": []}), json!([])] {
            assert_eq!(
                parse_models_response(&body, "openai").unwrap_err(),
                "Provider returned no models"
            );
        }
    }

    /// D3: `/v1/models` for Anthropic, `/models` for everything OpenAI-compatible.
    #[test]
    fn test_models_endpoint_for_openai_and_anthropic() {
        assert_eq!(
            models_endpoint("http://127.0.0.1:8045/v1", "openai"),
            "http://127.0.0.1:8045/v1/models"
        );
        assert_eq!(
            models_endpoint("https://api.openai.com/v1/", "openai"),
            "https://api.openai.com/v1/models",
            "a trailing slash must not double up"
        );
        assert_eq!(
            models_endpoint("https://api.example.com/v1", "custom"),
            "https://api.example.com/v1/models"
        );
        assert_eq!(
            models_endpoint("https://api.anthropic.com", "anthropic"),
            "https://api.anthropic.com/v1/models"
        );
        assert_eq!(
            models_endpoint("https://api.anthropic.com/v1", "anthropic"),
            "https://api.anthropic.com/v1/models",
            "an existing version segment must not be duplicated"
        );
    }

    #[test]
    fn test_models_endpoint_completed_urls_and_slashes() {
        for (kind, base, expected) in [
            ("openai", "https://api.example.com", "https://api.example.com/models"),
            ("openai", "https://api.example.com/v1/chat/completions", "https://api.example.com/v1/models"),
            ("custom", "https://api.example.com/v1/chat/completions/", "https://api.example.com/v1/models"),
            ("openai", "https://api.example.com/v1/models/", "https://api.example.com/v1/models"),
            ("anthropic", "https://api.example.com/v1/messages", "https://api.example.com/v1/models"),
            ("anthropic", "https://api.example.com/v1/messages/", "https://api.example.com/v1/models"),
            ("anthropic", "https://api.example.com/v1/models/", "https://api.example.com/v1/models"),
            ("anthropic", "https://api.example.com/messages", "https://api.example.com/v1/models"),
        ] {
            assert_eq!(models_endpoint(base, kind), expected, "{kind} {base}");
        }
    }

    #[test]
    fn test_openai_rejects_malformed_tool_calls() {
        for (call, expected) in [
            (json!({"function":{"name":"set","arguments":"{}"}}), "id"),
            (json!({"id":"  ","function":{"name":"set","arguments":"{}"}}), "id"),
            (json!({"id":"one","function":{"arguments":"{}"}}), "name"),
            (json!({"id":"one","function":{"name":"  ","arguments":"{}"}}), "name"),
            (json!({"id":"one","function":{"name":"set","arguments":"{not json"}}), "arguments"),
            (json!({"id":"one","function":{"name":"set"}}), "arguments"),
            (json!({"id":"one","function":{"name":"set","arguments":42}}), "arguments"),
        ] {
            let body = json!({"choices":[{"message":{"content":"ok","tool_calls":[call]}}]});
            let error = parse_openai_response(&body).err().expect("malformed call must fail");
            assert!(error.contains(expected), "{error}");
        }
        let duplicate = json!({"choices":[{"message":{"content":"ok","tool_calls":[
            {"id":"one","function":{"name":"set","arguments":"{}"}},
            {"id":"one","function":{"name":"get","arguments":{}}}
        ]}}]});
        assert!(parse_openai_response(&duplicate).err().expect("duplicate must fail").contains("one"));
        let object = json!({"choices":[{"message":{"tool_calls":[
            {"id":"one","function":{"name":"set","arguments":{"path":"x"}}}
        ]}}]});
        assert_eq!(parse_openai_response(&object).unwrap().tool_calls[0].arguments, json!({"path":"x"}));
    }

    #[test]
    fn test_anthropic_rejects_missing_or_blank_tool_identity() {
        for (block, expected) in [
            (json!({"type":"tool_use","name":"set"}), "id"),
            (json!({"type":"tool_use","id":"  ","name":"set"}), "id"),
            (json!({"type":"tool_use","id":"one"}), "name"),
            (json!({"type":"tool_use","id":"one","name":"  "}), "name"),
        ] {
            let body = json!({"content":[{"type":"text","text":"ok"},block]});
            let error = parse_anthropic_response(&body).err().expect("malformed call must fail");
            assert!(error.contains(expected), "{error}");
        }
        let body = json!({"content":[
            {"type":"tool_use","id":"one","name":"set"},
            {"type":"tool_use","id":"two","name":"set","input":[1,2]}
        ]});
        let reply = parse_anthropic_response(&body).unwrap();
        assert_eq!(reply.tool_calls[0].arguments, Value::Null);
        assert_eq!(reply.tool_calls[1].arguments, json!([1,2]));
    }

    /// D3: blank inputs fail immediately, and no request is attempted (there is no
    /// network in `cargo test` at all, so a network attempt would hang or error
    /// differently).
    #[test]
    fn test_fetch_models_rejects_blank_inputs() {
        tauri::async_runtime::block_on(async {
            assert_eq!(
                fetch_models("openai", "", "sk-key").await.unwrap_err(),
                "Provider base URL is missing"
            );
            assert_eq!(
                fetch_models("openai", "   ", "sk-key").await.unwrap_err(),
                "Provider base URL is missing"
            );
            assert_eq!(
                fetch_models("openai", "http://127.0.0.1:8045/v1", "")
                    .await
                    .unwrap_err(),
                "Provider API key is missing"
            );
            assert_eq!(
                fetch_models("openai", "http://127.0.0.1:8045/v1", "  ")
                    .await
                    .unwrap_err(),
                "Provider API key is missing"
            );
            assert_eq!(
                fetch_models("martian", "http://127.0.0.1:8045/v1", "sk-key")
                    .await
                    .unwrap_err(),
                "Unsupported provider kind: martian"
            );
        });
    }
}
