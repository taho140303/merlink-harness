//! Shared HTTP plumbing for provider calls.
//!
//! Chat and model discovery use separate clients so each keeps its own timeout.

use std::{sync::OnceLock, time::Duration};

use serde_json::Value;

/* ------------------------------------------------------------------ */
/* HTTP calls                                                          */
/* ------------------------------------------------------------------ */

#[derive(Clone, Copy)]
pub(super) enum TransportFailure {
    Connect,
    Request,
    Other,
}

pub(super) fn retryable_transport(failure: TransportFailure) -> bool {
    matches!(
        failure,
        TransportFailure::Connect | TransportFailure::Request
    )
}

fn transport_failure(error: &reqwest::Error) -> TransportFailure {
    match (error.is_connect(), error.is_request()) {
        (true, _) => TransportFailure::Connect,
        (_, true) => TransportFailure::Request,
        _ => TransportFailure::Other,
    }
}

pub(super) fn connection_error(provider: &str, base_url: &str, detail: &str) -> String {
    format!(
        "Không kết nối được tới {provider} tại {base_url}: {detail}. Kiểm tra gateway còn chạy không, rồi thử lại."
    )
}

pub(super) async fn send_chat_request(
    request: reqwest::RequestBuilder,
    provider: &str,
    base_url: &str,
    failure_prefix: &str,
) -> Result<reqwest::Response, String> {
    let retry_request = request.try_clone();
    match request.send().await {
        Ok(response) => Ok(response),
        Err(error) if retryable_transport(transport_failure(&error)) => {
            let Some(retry_request) = retry_request else {
                return Err(connection_error(provider, base_url, &error.to_string()));
            };
            tokio::time::sleep(Duration::from_millis(300)).await;
            retry_request.send().await.map_err(|retry_error| {
                connection_error(provider, base_url, &retry_error.to_string())
            })
        }
        Err(error) => Err(format!("{failure_prefix}: {error}")),
    }
}

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
    CHAT_CLIENT
        .get_or_init(|| {
            reqwest::Client::builder()
                .timeout(Duration::from_secs(120))
                .pool_idle_timeout(Duration::from_secs(20))
                .tcp_keepalive(Duration::from_secs(15))
                .build()
                .map_err(|error| error.to_string())
        })
        .clone()
}

/// Model discovery retains its shorter 30-second timeout.
pub(super) fn list_client() -> Result<reqwest::Client, String> {
    LIST_CLIENT
        .get_or_init(|| {
            reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .map_err(|error| error.to_string())
        })
        .clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection_error_names_provider_and_base_url() {
        let message = connection_error(
            "OpenAI-compatible",
            "http://127.0.0.1:8045/v1",
            "connection reset",
        );

        assert!(message.contains("OpenAI-compatible"));
        assert!(message.contains("http://127.0.0.1:8045/v1"));
        assert!(message.contains("Kiểm tra gateway còn chạy không, rồi thử lại."));
        assert!(!message.contains('–'));
        assert!(!message.contains('—'));
        assert!(!message.chars().any(is_emoji));
    }

    #[test]
    fn test_retryable_transport_only_includes_connect_and_request() {
        assert!(retryable_transport(TransportFailure::Connect));
        assert!(retryable_transport(TransportFailure::Request));
        assert!(!retryable_transport(TransportFailure::Other));
    }

    fn is_emoji(character: char) -> bool {
        matches!(character as u32, 0x1F000..=0x1FAFF | 0x2600..=0x27BF)
    }
}
