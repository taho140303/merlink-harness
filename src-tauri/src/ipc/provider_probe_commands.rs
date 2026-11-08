//! Auxiliary provider diagnostics IPC commands (outside the 40-command master catalog).
//!
//! - `test_provider_connection`: verifies endpoint reachability + credentials.
//! - `fetch_provider_models`: lists models served by an endpoint.
//!
//! Strategy: `GET {base}/models` first; if the endpoint has no listing route
//! (404/405) fall back to a minimal 1-token completion on the chosen protocol.
//! The API key comes from the request (form value) or the credential manager and is
//! never echoed back; all error text passes through the redactor.

use crate::ipc::config_commands::RuntimeConfigState;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::Arc;
use std::time::Duration;
use ts_rs::TS;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct ProbeProviderReq {
    pub provider_id: String,
    #[serde(default)]
    pub endpoint_url: Option<String>,
    #[serde(default)]
    pub model_name: Option<String>,
    /// `openai-completions` (default) | `openai-responses` | `anthropic-messages`
    #[serde(default)]
    pub protocol: Option<String>,
    #[serde(default, skip_serializing)]
    pub api_key: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct ProbeProviderResultDto {
    pub ok: bool,
    /// `connected` | `auth_failed` | `unreachable` | `error`
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    pub models: Vec<String>,
}

impl ProbeProviderResultDto {
    fn fail(status: &str, code: &str, msg: impl Into<String>) -> Self {
        Self {
            ok: false,
            status: status.into(),
            error: Some(msg.into()),
            code: Some(code.into()),
            models: vec![],
        }
    }
}

fn extract_models(body: &Value) -> Vec<String> {
    let arr = body
        .get("data")
        .or_else(|| body.get("models"))
        .and_then(Value::as_array)
        .or_else(|| body.as_array());
    let mut out: Vec<String> = arr
        .map(|items| {
            items
                .iter()
                .filter_map(|it| match it {
                    Value::String(s) => Some(s.clone()),
                    Value::Object(_) => it
                        .get("id")
                        .or_else(|| it.get("name"))
                        .or_else(|| it.get("model"))
                        .and_then(Value::as_str)
                        .map(|s| s.trim_start_matches("models/").to_string()),
                    _ => None,
                })
                .collect()
        })
        .unwrap_or_default();
    out.sort();
    out.dedup();
    out
}

fn probe_paths(endpoint: &str, provider_id: &str) -> (String, String) {
    let base = endpoint.trim().trim_end_matches('/');
    if provider_id == "ollama" {
        let root = base.strip_suffix("/api/chat").or_else(|| base.strip_suffix("/api")).unwrap_or(base);
        return (format!("{root}/api/tags"), format!("{root}/api/chat"));
    }
    let root = base.strip_suffix("/chat/completions").or_else(|| base.strip_suffix("/responses")).or_else(|| base.strip_suffix("/messages")).unwrap_or(base);
    (format!("{root}/models"), format!("{root}/chat/completions"))
}

pub async fn probe_provider(
    state: &RuntimeConfigState,
    req: &ProbeProviderReq,
    require_model_list: bool,
) -> ProbeProviderResultDto {
    let provider_id = req.provider_id.trim();
    let stored = state.providers.read().get(provider_id).cloned();

    let endpoint = req
        .endpoint_url
        .clone()
        .filter(|s| !s.trim().is_empty())
        .or_else(|| stored.as_ref().and_then(|p| p.endpoint_url.clone()));
    let Some(endpoint) = endpoint else {
        return ProbeProviderResultDto::fail("error", "validation_failed", "Endpoint URL is required");
    };
    let base = endpoint.trim().trim_end_matches('/').to_string();
    match url::Url::parse(&base) {
        Ok(u) if u.scheme() == "http" || u.scheme() == "https" => {}
        _ => {
            return ProbeProviderResultDto::fail(
                "error",
                "validation_failed",
                "Endpoint URL must be a valid http(s) URL",
            )
        }
    }

    let api_key = req
        .api_key
        .clone()
        .filter(|k| !k.trim().is_empty())
        .or_else(|| state.credential_manager.get_api_key(provider_id));
    if let Some(k) = &api_key {
        state.redactor.register_secret(k);
    }

    let protocol = req.protocol.as_deref().unwrap_or("openai-completions");
    let anthropic = protocol == "anthropic-messages";
    let client = match reqwest::Client::builder()
        .timeout(Duration::from_secs(15))
        .build()
    {
        Ok(c) => c,
        Err(e) => return ProbeProviderResultDto::fail("error", "internal_error", e.to_string()),
    };

    let auth = |rb: reqwest::RequestBuilder| -> reqwest::RequestBuilder {
        let rb = if anthropic {
            rb.header("anthropic-version", "2023-06-01")
        } else {
            rb
        };
        match &api_key {
            Some(k) if anthropic => rb.header("x-api-key", k),
            Some(k) => rb.bearer_auth(k),
            None => rb,
        }
    };

    let map_send_err = |e: reqwest::Error| {
        let msg = state.redactor.redact(&e.to_string());
        ProbeProviderResultDto::fail("unreachable", "provider_unreachable", msg)
    };

    // Normalize configured adapter endpoints before probing (Ollama uses /api/tags).
    let (models_url, completion_url) = probe_paths(&base, provider_id);
    let resp = match auth(client.get(models_url)).send().await {
        Ok(r) => r,
        Err(e) => return map_send_err(e),
    };
    let status = resp.status();
    if status.is_success() {
        let body: Value = resp.json().await.unwrap_or(Value::Null);
        let models = extract_models(&body);
        return ProbeProviderResultDto {
            ok: true,
            status: "connected".into(),
            error: None,
            code: None,
            models,
        };
    }
    if status.as_u16() == 401 || status.as_u16() == 403 {
        return ProbeProviderResultDto::fail(
            "auth_failed",
            "provider_auth_failed",
            format!("Authentication failed (HTTP {})", status.as_u16()),
        );
    }
    if !(status.as_u16() == 404 || status.as_u16() == 405 || status.as_u16() == 501) {
        let body = resp.text().await.unwrap_or_default();
        let snippet: String = body.chars().take(200).collect();
        return ProbeProviderResultDto::fail(
            "error",
            "provider_error",
            state
                .redactor
                .redact(&format!("HTTP {}: {}", status.as_u16(), snippet)),
        );
    }
    if require_model_list {
        return ProbeProviderResultDto::fail(
            "error",
            "models_unsupported",
            "Endpoint does not expose a model listing; add model ids manually",
        );
    }

    // 2) Fallback: minimal completion
    let model = req
        .model_name
        .clone()
        .filter(|s| !s.trim().is_empty())
        .or_else(|| stored.and_then(|p| p.model_name));
    let Some(model) = model else {
        return ProbeProviderResultDto::fail(
            "error",
            "validation_failed",
            "No /models route and no model name set for a test completion",
        );
    };
    let (path, body) = match protocol {
        "openai-responses" => (
            "responses",
            json!({"model": model, "input": "ping", "max_output_tokens": 16}),
        ),
        "anthropic-messages" => (
            "messages",
            json!({"model": model, "max_tokens": 1, "messages": [{"role": "user", "content": "ping"}]}),
        ),
        _ => (
            "chat/completions",
            json!({"model": model, "max_tokens": 1, "messages": [{"role": "user", "content": "ping"}]}),
        ),
    };
    let url = if provider_id == "ollama" { completion_url } else if path == "chat/completions" { completion_url } else { format!("{base}/{path}") };
    let body = if provider_id == "ollama" {
        json!({"model": model, "stream": false, "messages": [{"role": "user", "content": "ping"}]})
    } else { body };
    let resp = match auth(client.post(url).json(&body)).send().await {
        Ok(r) => r,
        Err(e) => return map_send_err(e),
    };
    let status = resp.status();
    if status.is_success() {
        return ProbeProviderResultDto {
            ok: true,
            status: "connected".into(),
            error: None,
            code: None,
            models: vec![],
        };
    }
    if status.as_u16() == 401 || status.as_u16() == 403 {
        return ProbeProviderResultDto::fail(
            "auth_failed",
            "provider_auth_failed",
            format!("Authentication failed (HTTP {})", status.as_u16()),
        );
    }
    let text = resp.text().await.unwrap_or_default();
    let snippet: String = text.chars().take(200).collect();
    ProbeProviderResultDto::fail(
        "error",
        "provider_error",
        state
            .redactor
            .redact(&format!("HTTP {}: {}", status.as_u16(), snippet)),
    )
}

#[tauri::command]
pub async fn test_provider_connection(
    state: tauri::State<'_, Arc<RuntimeConfigState>>,
    req: ProbeProviderReq,
) -> Result<ProbeProviderResultDto, crate::error::IpcError> {
    Ok(probe_provider(&state, &req, false).await)
}

#[tauri::command]
pub async fn fetch_provider_models(
    state: tauri::State<'_, Arc<RuntimeConfigState>>,
    req: ProbeProviderReq,
) -> Result<ProbeProviderResultDto, crate::error::IpcError> {
    Ok(probe_provider(&state, &req, true).await)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_openai_and_gemini_shapes() {
        let a = json!({"data": [{"id": "b"}, {"id": "a"}]});
        assert_eq!(extract_models(&a), vec!["a", "b"]);
        let g = json!({"models": [{"name": "models/gemini-x"}]});
        assert_eq!(extract_models(&g), vec!["gemini-x"]);
        assert_eq!(extract_models(&json!(["m1", "m1"])), vec!["m1"]);
        assert_eq!(extract_models(&json!({"models": [{"name": "llama3.2"}]})), vec!["llama3.2"]);
    }

    #[test]
    fn configured_provider_endpoints_have_correct_probe_routes() {
        assert_eq!(probe_paths("http://127.0.0.1:11434/api/chat", "ollama"),
            ("http://127.0.0.1:11434/api/tags".into(), "http://127.0.0.1:11434/api/chat".into()));
        assert_eq!(probe_paths("https://api.opencode.ai/v1/chat/completions", "opencode"),
            ("https://api.opencode.ai/v1/models".into(), "https://api.opencode.ai/v1/chat/completions".into()));
    }
}
