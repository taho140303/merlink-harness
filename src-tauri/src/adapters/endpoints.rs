//! Provider endpoint normalisation and token-budget parsing.
//!
//! Everything in this module is pure: it rewrites user-supplied base URLs and
//! human-readable budget strings into the values the wire format expects.

pub fn parse_context_window(value: &str) -> u64 {
    let value = value.trim().to_ascii_uppercase();
    let (number, multiplier) = match value.chars().last() {
        Some('K') => (&value[..value.len() - 1], 1_000_u64),
        Some('M') => (&value[..value.len() - 1], 1_000_000_u64),
        _ => (value.as_str(), 1_u64),
    };
    number.parse::<u64>().ok().and_then(|n| n.checked_mul(multiplier)).filter(|n| *n > 0).unwrap_or(128_000)
}

pub fn openai_endpoint(base_url: &str) -> String {
    let base = base_url.trim_end_matches('/');
    if base.ends_with("/chat/completions") {
        base.to_owned()
    } else {
        format!("{base}/chat/completions")
    }
}

pub(super) fn anthropic_endpoint(base_url: &str) -> String {
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
pub(super) fn models_endpoint(base_url: &str, kind: &str) -> String {
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

pub(super) fn output_limit(value: &str) -> u64 {
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

#[cfg(test)]
mod tests {
    use super::super::{fetch_models, parse_models_response};
    use super::*;
    use serde_json::json;

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
