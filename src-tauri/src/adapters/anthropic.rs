//! Anthropic Messages wire format: history conversion, response parsing, and the
//! HTTP call itself.

use serde_json::{Value, json};

use crate::models::{MessageEnvelope, TokenUsage, ToolCall};

use super::{
    AdapterReply, ChatRequest,
    endpoints::{anthropic_endpoint, output_limit},
    http::{checked_json, client},
};

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
                let id = block
                    .get("id")
                    .and_then(Value::as_str)
                    .filter(|value| !value.trim().is_empty())
                    .ok_or_else(|| {
                        format!("Anthropic tool_use block {} missing id", tool_calls.len())
                    })?;
                let name = block
                    .get("name")
                    .and_then(Value::as_str)
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
/* HTTP call                                                           */
/* ------------------------------------------------------------------ */

pub(super) async fn call_anthropic(request: &ChatRequest) -> Result<AdapterReply, String> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anthropic_turns_merge_tool_results() {
        let mut assistant = MessageEnvelope::new("a1", "assistant", "đang xử lý");
        assistant.tool_calls = Some(vec![
            ToolCall {
                id: "toolu_1".into(),
                name: "super_json_get".into(),
                arguments: json!({"path": "require.project.name"}),
            },
            ToolCall {
                id: "toolu_2".into(),
                name: "super_json_missing".into(),
                arguments: json!({}),
            },
        ]);
        let first =
            MessageEnvelope::tool_result("t1", "toolu_1", "super_json_get", &json!({"ok": true}));
        let second = MessageEnvelope::tool_result(
            "t2",
            "toolu_2",
            "super_json_missing",
            &json!({"missing": []}),
        );

        let turns = anthropic_turns(&[
            MessageEnvelope::new("u1", "user", "hi"),
            assistant,
            first,
            second,
        ]);

        assert_eq!(
            turns.len(),
            3,
            "two tool results must merge into one user turn"
        );
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
                 "input": {"path": "require.project.name", "value": "X"}}
            ],
            "usage": {"input_tokens": 10, "output_tokens": 5}
        });
        let reply = parse_anthropic_response(&body).unwrap();
        assert_eq!(reply.content, "để tôi đặt tên");
        assert_eq!(reply.tool_calls[0].id, "toolu_1");
        assert_eq!(reply.tool_calls[0].arguments["value"], json!("X"));
        assert_eq!(reply.tokens.output_tokens, 5);
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
            let error = parse_anthropic_response(&body)
                .err()
                .expect("malformed call must fail");
            assert!(error.contains(expected), "{error}");
        }
        let body = json!({"content":[
            {"type":"tool_use","id":"one","name":"set"},
            {"type":"tool_use","id":"two","name":"set","input":[1,2]}
        ]});
        let reply = parse_anthropic_response(&body).unwrap();
        assert_eq!(reply.tool_calls[0].arguments, Value::Null);
        assert_eq!(reply.tool_calls[1].arguments, json!([1, 2]));
    }
}
