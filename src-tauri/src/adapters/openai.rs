//! OpenAI-compatible wire format: history conversion, response parsing, and the
//! HTTP call itself.

use std::collections::HashSet;

use serde_json::{Value, json};

use crate::models::{MessageEnvelope, TokenUsage, ToolCall};

use super::{
    AdapterReply, ChatRequest,
    endpoints::openai_endpoint,
    http::{checked_json, client},
};

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
        let calls = raw_calls
            .as_array()
            .ok_or("OpenAI tool_calls must be an array")?;
        for (index, call) in calls.iter().enumerate() {
            let id = call
                .get("id")
                .and_then(Value::as_str)
                .filter(|value| !value.trim().is_empty())
                .ok_or_else(|| format!("OpenAI tool call {index} missing id"))?;
            if !ids.insert(id) {
                return Err(format!("OpenAI tool call {index} duplicate id {id}"));
            }
            let function = call
                .get("function")
                .ok_or_else(|| format!("OpenAI tool call {id} missing function.name"))?;
            let name = function
                .get("name")
                .and_then(Value::as_str)
                .filter(|value| !value.trim().is_empty())
                .ok_or_else(|| format!("OpenAI tool call {id} missing function.name"))?;
            let arguments = match function.get("arguments") {
                Some(Value::String(raw)) => serde_json::from_str(raw)
                    .map_err(|error| format!("OpenAI tool call {id} invalid arguments: {error}"))?,
                Some(value @ (Value::Object(_) | Value::Array(_) | Value::Null)) => value.clone(),
                Some(_) => return Err(format!("OpenAI tool call {id} invalid arguments type")),
                None => return Err(format!("OpenAI tool call {id} missing arguments")),
            };
            tool_calls.push(ToolCall {
                id: id.to_owned(),
                name: name.to_owned(),
                arguments,
            });
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

/* ------------------------------------------------------------------ */
/* HTTP call                                                           */
/* ------------------------------------------------------------------ */

pub(super) async fn call_openai(request: &ChatRequest) -> Result<AdapterReply, String> {
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

#[cfg(test)]
mod tests {
    use super::super::parse_anthropic_response;
    use super::*;

    fn message(id: &str, role: &str, content: &str) -> MessageEnvelope {
        MessageEnvelope::new(id, role, content)
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
                            "arguments": "{\"path\":\"require.project.name\",\"value\":\"Nửa Đêm\"}"
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
            json!("require.project.name")
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
        assert_eq!(
            reply.reasoning.as_deref(),
            Some("Tôi sẽ gọi super_json_set.")
        );
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
            arguments: json!({"path": "require.project.name", "value": "X"}),
        }]);
        let tool =
            MessageEnvelope::tool_result("t1", "call_1", "super_json_set", &json!({"ok": true}));

        let turns = openai_turns(&[message("u1", "user", "hi"), assistant, tool]);
        assert_eq!(turns[0]["role"], "user");
        assert_eq!(
            turns[1]["tool_calls"][0]["function"]["name"],
            "super_json_set"
        );
        // Arguments must be a JSON string on the wire, not an object.
        assert!(turns[1]["tool_calls"][0]["function"]["arguments"].is_string());
        assert_eq!(turns[2]["role"], "tool");
        assert_eq!(turns[2]["tool_call_id"], "call_1");
    }

    #[test]
    fn test_openai_rejects_malformed_tool_calls() {
        for (call, expected) in [
            (json!({"function":{"name":"set","arguments":"{}"}}), "id"),
            (
                json!({"id":"  ","function":{"name":"set","arguments":"{}"}}),
                "id",
            ),
            (json!({"id":"one","function":{"arguments":"{}"}}), "name"),
            (
                json!({"id":"one","function":{"name":"  ","arguments":"{}"}}),
                "name",
            ),
            (
                json!({"id":"one","function":{"name":"set","arguments":"{not json"}}),
                "arguments",
            ),
            (json!({"id":"one","function":{"name":"set"}}), "arguments"),
            (
                json!({"id":"one","function":{"name":"set","arguments":42}}),
                "arguments",
            ),
        ] {
            let body = json!({"choices":[{"message":{"content":"ok","tool_calls":[call]}}]});
            let error = parse_openai_response(&body)
                .err()
                .expect("malformed call must fail");
            assert!(error.contains(expected), "{error}");
        }
        let duplicate = json!({"choices":[{"message":{"content":"ok","tool_calls":[
            {"id":"one","function":{"name":"set","arguments":"{}"}},
            {"id":"one","function":{"name":"get","arguments":{}}}
        ]}}]});
        assert!(
            parse_openai_response(&duplicate)
                .err()
                .expect("duplicate must fail")
                .contains("one")
        );
        let object = json!({"choices":[{"message":{"tool_calls":[
            {"id":"one","function":{"name":"set","arguments":{"path":"x"}}}
        ]}}]});
        assert_eq!(
            parse_openai_response(&object).unwrap().tool_calls[0].arguments,
            json!({"path":"x"})
        );
    }
}
