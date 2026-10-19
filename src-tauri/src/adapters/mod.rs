//! LLM provider adapters.
//!
//! [`LlmBackend`] isolates the agent loop from provider wire formats;
//! [`HttpBackend`] is production and submodules hold endpoint, wire, and HTTP code.

mod anthropic;
mod endpoints;
mod http;
mod openai;

#[cfg(test)]
mod scripted;

use std::{future::Future, pin::Pin};

use serde_json::Value;

use crate::models::{FetchedModel, MessageEnvelope, TokenUsage, ToolCall};

use anthropic::call_anthropic;
use endpoints::models_endpoint;
use http::{checked_json, list_client};
use openai::call_openai;

#[allow(unused_imports)]
pub use anthropic::parse_anthropic_response;
#[allow(unused_imports)]
pub use endpoints::{openai_endpoint, parse_context_window};
#[allow(unused_imports)]
pub use openai::parse_openai_response;
#[cfg(test)]
pub use scripted::ScriptedBackend;

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
    fn scripted(content: String, tool_calls: Vec<ToolCall>) -> Self {
        Self {
            content,
            tokens: TokenUsage::default(),
            tool_calls,
            reasoning: None,
        }
    }

    pub fn text(content: impl Into<String>) -> Self {
        Self::scripted(content.into(), Vec::new())
    }

    pub fn with_tool_calls(content: impl Into<String>, tool_calls: Vec<ToolCall>) -> Self {
        Self::scripted(content.into(), tool_calls)
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
/// dependency; `Send + Sync` is required because Tauri commands return `Send`
/// futures.
pub trait LlmBackend: Send + Sync {
    fn complete<'a>(
        &'a self,
        request: ChatRequest,
    ) -> BoxedFuture<'a, Result<AdapterReply, String>>;
}

#[derive(Default)]
pub struct HttpBackend;

impl LlmBackend for HttpBackend {
    fn complete<'a>(
        &'a self,
        request: ChatRequest,
    ) -> BoxedFuture<'a, Result<AdapterReply, String>> {
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

/// Extracts the model list from a provider response (Sprint 11.5 · D4).
///
/// Both wire formats put the collection under `data[]`, and a bare JSON array is
/// accepted because proxies often omit the envelope. Entries without a usable
/// `id` are skipped; an empty list is an error.
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

/// Asks the provider for its model list (Sprint 11.5 · D1/D3).
///
/// Runs in Rust so the API key stays native; blank inputs fail before any network.
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
