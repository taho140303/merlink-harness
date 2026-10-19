//! Shared HTTP plumbing for provider calls.
//!
//! Chat and model discovery use separate clients so each keeps its own timeout.

use std::sync::OnceLock;

use serde_json::Value;

/* ------------------------------------------------------------------ */
/* HTTP calls                                                          */
/* ------------------------------------------------------------------ */

pub(super) async fn checked_json(response: reqwest::Response) -> Result<Value, String> {
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

pub(super) fn client() -> Result<reqwest::Client, String> {
    CHAT_CLIENT.get_or_init(|| reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(120))
        .build()
        .map_err(|error| error.to_string())).clone()
}

/// Model discovery retains its shorter 30-second timeout.
pub(super) fn list_client() -> Result<reqwest::Client, String> {
    LIST_CLIENT.get_or_init(|| reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|error| error.to_string())).clone()
}
