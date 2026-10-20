use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use serde_json::Value;

use crate::adapters::{ChatRequest, LlmBackend};
use crate::models::{MessageEnvelope, ModelConfig, ProviderConfig, TokenUsage};

const COMPLETION_TIMEOUT: Duration = Duration::from_secs(180);
const RATE_LIMIT_BACKOFF: Duration = Duration::from_secs(2);
const MAX_RATE_LIMIT_RETRIES: usize = 2;
static MESSAGE_ID: AtomicU64 = AtomicU64::new(1);

/// Text and token usage returned by one non-tool provider call.
#[derive(Clone, Debug)]
pub struct Completion {
    pub text: String,
    pub usage: Option<TokenUsage>,
}

/// Calls a provider once for a system/user prompt, without enabling tools.
pub async fn complete(
    backend: &dyn LlmBackend,
    provider: &ProviderConfig,
    model: &ModelConfig,
    system: &str,
    user: &str,
) -> Result<Completion, String> {
    let request = ChatRequest {
        kind: provider.kind.clone(),
        base_url: provider.base_url.clone(),
        api_key: provider.api_key.clone(),
        model: model.id.clone(),
        max_tokens: model.max_tokens.clone(),
        system: system.to_owned(),
        messages: vec![MessageEnvelope::new(
            format!("outline-{}", MESSAGE_ID.fetch_add(1, Ordering::Relaxed)),
            "user",
            user,
        )],
        tools: Vec::new(),
    };

    let mut retries = 0;
    let reply = loop {
        match tokio::time::timeout(COMPLETION_TIMEOUT, backend.complete(request.clone())).await {
            Ok(Ok(reply)) => break reply,
            Ok(Err(error)) if error.contains("429") && retries < MAX_RATE_LIMIT_RETRIES => {
                retries += 1;
                tokio::time::sleep(RATE_LIMIT_BACKOFF).await;
            }
            Ok(Err(error)) => return Err(error),
            Err(_) => return Err("Provider completion timed out after 180 seconds".into()),
        }
    };

    if reply.content.trim().is_empty() {
        return Err("Provider returned an empty completion".into());
    }

    Ok(Completion {
        text: reply.content,
        usage: Some(reply.tokens),
    })
}

/// Extracts exactly one JSON object from plain, fenced, or surrounding prose text.
pub fn extract_json(text: &str) -> Result<Value, String> {
    let text = fenced_content(text).unwrap_or(text);
    let bytes = text.as_bytes();
    let mut objects = Vec::new();
    let mut index = 0;

    while index < bytes.len() {
        if bytes[index] != b'{' {
            index += 1;
            continue;
        }

        let start = index;
        let mut depth = 0usize;
        let mut in_string = false;
        let mut escaped = false;
        let mut end = None;
        while index < bytes.len() {
            let byte = bytes[index];
            if in_string {
                if escaped {
                    escaped = false;
                } else if byte == b'\\' {
                    escaped = true;
                } else if byte == b'"' {
                    in_string = false;
                }
            } else {
                match byte {
                    b'"' => in_string = true,
                    b'{' => depth += 1,
                    b'}' => {
                        depth -= 1;
                        if depth == 0 {
                            end = Some(index + 1);
                            break;
                        }
                    }
                    _ => {}
                }
            }
            index += 1;
        }

        match end {
            Some(end) => {
                objects.push((start, &text[start..end]));
                index = end;
            }
            None => {
                let error_offset = start;
                return Err(format!(
                    "Malformed JSON object at offset {error_offset}: unbalanced braces or string"
                ));
            }
        }
    }

    if objects.is_empty() {
        return Err("No JSON object found".into());
    }
    if objects.len() != 1 {
        return Err(format!(
            "Expected exactly one JSON object, found {}",
            objects.len()
        ));
    }

    let (start, object) = objects[0];
    serde_json::from_str(object).map_err(|error| {
        let offset = json_error_offset(object, &error);
        format!("Invalid JSON at offset {}: {error}", start + offset)
    })
}

fn fenced_content(text: &str) -> Option<&str> {
    let opening = text.find("```")?;
    let after_open = opening + 3;
    let closing_relative = text[after_open..].find("```")?;
    let mut content = &text[after_open..after_open + closing_relative];
    let trimmed = content.trim_start();
    if let Some(after_json) = trimmed
        .strip_prefix("json")
        .filter(|rest| rest.starts_with([' ', '\t', '\r', '\n']))
    {
        content = after_json;
    }
    Some(content)
}

fn json_error_offset(text: &str, error: &serde_json::Error) -> usize {
    let line = error.line().saturating_sub(1);
    let column = error.column().saturating_sub(1);
    text.split('\n')
        .take(line)
        .map(|part| part.len() + 1)
        .sum::<usize>()
        + column
}

#[cfg(test)]
mod tests {
    use crate::adapters::{AdapterReply, ScriptedBackend};
    use crate::models::{InputTypes, ModelConfig, ProviderConfig, TokenUsage, ToolCall};
    use crate::runtime::{complete, extract_json};
    use serde_json::{Value, json};

    fn provider() -> ProviderConfig {
        ProviderConfig {
            id: "provider-1".into(),
            name: "Test provider".into(),
            kind: "openai".into(),
            api_key: "test-key".into(),
            base_url: "https://example.invalid".into(),
            models: Vec::new(),
            default_model_id: "model-1".into(),
        }
    }

    fn model() -> ModelConfig {
        ModelConfig {
            id: "model-1".into(),
            name: "Test model".into(),
            context_window: "8192".into(),
            max_tokens: "321".into(),
            input_types: InputTypes {
                text: true,
                image: false,
            },
            reasoning_level: false,
        }
    }

    #[test]
    fn test_complete_returns_text_and_usage_without_tools() {
        let backend = ScriptedBackend::new(vec![AdapterReply {
            content: "  {\"ok\":true}  ".into(),
            tokens: TokenUsage {
                input_tokens: 17,
                output_tokens: 9,
            },
            tool_calls: vec![ToolCall {
                id: "ignored-call".into(),
                name: "ignored_tool".into(),
                arguments: json!({}),
            }],
            reasoning: None,
        }]);

        let completion = tauri::async_runtime::block_on(complete(
            &backend,
            &provider(),
            &model(),
            "system prompt",
            "user prompt",
        ))
        .unwrap();

        assert_eq!(completion.text, "  {\"ok\":true}  ");
        let usage = completion.usage.unwrap();
        assert_eq!(usage.input_tokens, 17);
        assert_eq!(usage.output_tokens, 9);

        let requests = backend.requests();
        assert_eq!(requests.len(), 1);
        assert!(requests[0].tools.is_empty());
        assert_eq!(requests[0].system, "system prompt");
        assert_eq!(requests[0].max_tokens, "321");
        assert_eq!(requests[0].messages.len(), 1);
        assert_eq!(requests[0].messages[0].role, "user");
        assert_eq!(requests[0].messages[0].content, "user prompt");
    }

    #[test]
    fn test_complete_rejects_empty_text() {
        let backend = ScriptedBackend::new(vec![AdapterReply::text(" \n\t")]);
        let error = tauri::async_runtime::block_on(complete(
            &backend,
            &provider(),
            &model(),
            "system",
            "user",
        ))
        .unwrap_err();
        assert_eq!(error, "Provider returned an empty completion");
    }

    #[test]
    fn test_extract_json_accepts_plain_and_fenced() {
        for (text, expected) in [
            (r#"{"ok":true}"#, json!({"ok": true})),
            ("```json\n{\"ok\":true}\n```", json!({"ok": true})),
            ("Output: {\"count\":2} done.", json!({"count": 2})),
        ] {
            assert_eq!(extract_json(text).unwrap(), expected);
        }
    }

    #[test]
    fn test_extract_json_handles_braces_inside_strings() {
        let text =
            r#"before {"text":"braces { and } and escaped quote \" then }","ok":true} after"#;
        let parsed = extract_json(text).unwrap();
        assert_eq!(parsed["text"], "braces { and } and escaped quote \" then }");
        assert_eq!(parsed["ok"], Value::Bool(true));
    }

    #[test]
    fn test_extract_json_rejects_malformed() {
        let error = extract_json(r#"preface {"ok":, "bad":true} trailing"#).unwrap_err();
        assert!(error.contains("Invalid JSON"), "{error}");
        assert!(error.contains("offset"), "{error}");
    }
}
