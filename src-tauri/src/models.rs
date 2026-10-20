use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct InputTypes {
    pub text: bool,
    pub image: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelConfig {
    pub id: String,
    pub name: String,
    pub context_window: String,
    pub max_tokens: String,
    pub input_types: InputTypes,
    pub reasoning_level: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub api_key: String,
    pub base_url: String,
    pub models: Vec<ModelConfig>,
    pub default_model_id: String,
}

/// One model as reported by the provider's model-list endpoint (Sprint 11.5 · D2).
///
/// `name` is the provider's `display_name` when it sends one, otherwise the id
/// itself, so the UI always has a label to render.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FetchedModel {
    pub id: String,
    pub name: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TokenUsage {
    pub input_tokens: u64,
    pub output_tokens: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MessageMeta {
    pub model: String,
    pub provider: String,
    pub session_id: String,
    pub project_id: String,
    pub prompt_template_id: String,
    pub prompt_hash: String,
    pub tokens: TokenUsage,
    pub latency_ms: u64,
    pub created_at: String,
}

/// A tool call as requested by the model, normalised across the OpenAI
/// (`tool_calls`) and Anthropic (`tool_use`) wire formats.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ToolCall {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub arguments: serde_json::Value,
}

/// Display-ready sidebar summary for a persisted conversation.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProjectSummary {
    pub id: String,
    pub title: String,
    pub genres: Vec<String>,
    pub created_at: String,
    pub session_count: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SessionSummary {
    pub id: String,
    /// Empty string means the session has not been bound to a project.
    pub project_id: String,
    pub project_title: Option<String>,
    pub title: String,
    pub created_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MessageEnvelope {
    pub id: String,
    pub role: String,
    pub content: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<MessageMeta>,
    /// Set on assistant messages that request tools.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,
    /// Set on `role: "tool"` messages, pairing them with the request.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
    /// Tool name, for `role: "tool"` messages.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The model's own thinking for the turn (Sprint 11.12 · D9). Only the final
    /// assistant message of a turn carries it; intermediate messages do not.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<String>,
}

impl MessageEnvelope {
    pub fn new(id: impl Into<String>, role: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            role: role.into(),
            content: content.into(),
            meta: None,
            tool_calls: None,
            tool_call_id: None,
            name: None,
            reasoning: None,
        }
    }

    /// A tool result fed back to the model. Written as a compact JSON string so
    /// the provider sees valid JSON rather than a Rust `Debug` rendering.
    pub fn tool_result(
        id: impl Into<String>,
        tool_call_id: impl Into<String>,
        name: impl Into<String>,
        result: &serde_json::Value,
    ) -> Self {
        Self {
            id: id.into(),
            role: "tool".into(),
            content: serde_json::to_string(result).unwrap_or_else(|_| "{}".into()),
            meta: None,
            tool_calls: None,
            tool_call_id: Some(tool_call_id.into()),
            name: Some(name.into()),
            reasoning: None,
        }
    }
}

/// Convenience constructor that only tests need so far.
#[cfg(test)]
impl MessageEnvelope {
    pub fn user(id: impl Into<String>, content: impl Into<String>) -> Self {
        Self::new(id, "user", content)
    }
}

/// What one chat turn produced (D4).
///
/// `project_id` is the project the session belongs to *after* the turn, so the
/// frontend can rebind instead of guessing: the first message of a brand new chat
/// can create the project, and the answer already reports it.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatTurn {
    pub project_id: String,
    pub messages: Vec<MessageEnvelope>,
}

/// Super json completeness, computed in Rust (D6/D7) so the header bar and the
/// outline button read one source of truth.
///
/// `percent` uses two stops: the 12 required units cover 0 → 70%, the 35 optional
/// units cover 70 → 100%. `required_total`/`optional_total` are always 12/35; the
/// `filled_*` counters are the ones that drop to zero without a project.
/// Live progress of one agent turn, streamed to the UI over a Tauri `Channel`
/// (D2). Serialised with an internal `type` tag so TypeScript can build a
/// discriminated union:
///
/// ```json
/// {"type":"tool_started","id":"call_1","name":"super_json_set","path":null,"status":"..."}
/// ```
#[derive(Clone, Debug, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AgentEvent {
    /// First event of a turn. `round_budget` is [`crate::runtime::MAX_TOOL_ROUNDS`].
    TurnStarted {
        session_id: String,
        round_budget: u32,
    },
    /// Emitted immediately before each model call. `round` is 1-based.
    RoundStarted { round: u32 },
    /// Emitted immediately before a tool runs. `status` is the Vietnamese phase
    /// message from `tools::status_message`.
    ToolStarted {
        id: String,
        name: String,
        #[serde(default)]
        path: Option<String>,
        status: String,
    },
    /// Emitted after a tool runs, carrying the real measured duration (D5).
    ToolFinished {
        id: String,
        name: String,
        ok: bool,
        duration_ms: u64,
        status: String,
    },
    /// Emitted after every model call of a round, with the real duration (D5).
    RoundFinished {
        round: u32,
        duration_ms: u64,
        tool_calls: u32,
    },
    /// Last event of a turn, emitted before `run_agent_turn` returns — on the
    /// demo path, the 5-round cap and the tool-error path alike.
    TurnFinished {
        project_id: String,
        duration_ms: u64,
        rounds: u32,
    },
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ContextUsage {
    pub has_model: bool,
    pub total_tokens: u64,
    pub used_tokens: u64,
    pub percent: u8,
    pub system_tokens: u64,
    pub tool_tokens: u64,
    pub message_tokens: u64,
    pub measured_input_tokens: Option<u64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SuperJsonProgress {
    pub has_project: bool,
    pub project_id: String,
    pub percent: u8,
    pub label: String,
    pub filled_required: u32,
    pub required_total: u32,
    pub filled_optional: u32,
    pub optional_total: u32,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct OutlineStatus {
    pub ok: bool,
    pub stage: String,
    pub active: bool,
    pub run_status: String,
    pub stage_status: String,
    pub counts: serde_json::Value,
    pub next_stage: Option<String>,
    pub blockers: Vec<String>,
    pub usage: serde_json::Value,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OutlineItem {
    pub key: String,
    pub lane: String,
    pub position: u32,
    pub status: String,
    pub title: String,
    pub error: Option<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OutlineRunResult {
    pub stage: String,
    pub shards: u32,
    pub calls: u32,
    pub items: u32,
    pub failures: u32,
    pub tokens_in: u64,
    pub tokens_out: u64,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CommitReport {
    pub committed: usize,
    pub skipped: usize,
    pub missing: Vec<String>,
    pub warnings: Vec<String>,
}
#[derive(Clone, Debug, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum OutlineEvent {
    RunStarted {
        stage: String,
    },
    ShardStarted {
        key: String,
        lane: String,
    },
    ShardFinished {
        key: String,
        lane: String,
        ok: bool,
        error: Option<String>,
    },
    ItemsReady {
        stage: String,
        count: u32,
    },
    RunFinished {
        stage: String,
        failures: u32,
    },
    RunFailed {
        error: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Old payloads (written before tools existed) must still deserialize.
    #[test]
    fn test_message_envelope_tolerates_legacy_payload() {
        let legacy = r#"{"id":"m1","role":"user","content":"hi"}"#;
        let parsed: MessageEnvelope = serde_json::from_str(legacy).unwrap();
        assert_eq!(parsed.role, "user");
        assert!(parsed.tool_calls.is_none());
        assert!(parsed.tool_call_id.is_none());
    }

    /// D2: the internal tag must appear as `"type"` so the frontend can build a
    /// discriminated union, and every variant must stick to snake_case.
    #[test]
    fn test_agent_event_serializes_with_type_tag() {
        let _home = crate::test_support::TempHome::new("models-events");

        let events = [
            AgentEvent::TurnStarted {
                session_id: "sess-1".into(),
                round_budget: 5,
            },
            AgentEvent::RoundStarted { round: 2 },
            AgentEvent::ToolStarted {
                id: "call_1".into(),
                name: "super_json_set".into(),
                path: Some("require.project.name".into()),
                status: "Đang ghi nhận tên truyện".into(),
            },
            AgentEvent::ToolFinished {
                id: "call_1".into(),
                name: "super_json_set".into(),
                ok: true,
                duration_ms: 12,
                status: "Đang ghi nhận tên truyện".into(),
            },
            AgentEvent::RoundFinished {
                round: 2,
                duration_ms: 340,
                tool_calls: 1,
            },
            AgentEvent::TurnFinished {
                project_id: "nua-dem".into(),
                duration_ms: 900,
                rounds: 2,
            },
        ];
        let expected_tags = [
            "turn_started",
            "round_started",
            "tool_started",
            "tool_finished",
            "round_finished",
            "turn_finished",
        ];

        for (event, tag) in events.iter().zip(expected_tags) {
            let encoded = serde_json::to_value(event).unwrap();
            assert_eq!(
                encoded["type"], tag,
                "internal tag must serialise as 'type': {encoded}"
            );
        }

        // The fields the UI reads must keep their snake_case names.
        let tool = serde_json::to_value(&events[2]).unwrap();
        assert_eq!(tool["id"], "call_1");
        assert_eq!(tool["path"], "require.project.name");
        let finished = serde_json::to_value(&events[4]).unwrap();
        assert_eq!(finished["duration_ms"], 340);
        assert_eq!(finished["tool_calls"], 1);
    }

    /// An assistant message that only requests a tool has empty content but must
    /// still round-trip with its tool calls intact.
    #[test]
    fn test_tool_call_round_trip_with_empty_content() {
        let mut message = MessageEnvelope::new("a1", "assistant", "");
        message.tool_calls = Some(vec![ToolCall {
            id: "call_1".into(),
            name: "super_json_set".into(),
            arguments: serde_json::json!({"path": "require.project.name", "value": "X"}),
        }]);
        let encoded = serde_json::to_string(&message).unwrap();
        let decoded: MessageEnvelope = serde_json::from_str(&encoded).unwrap();
        assert_eq!(decoded.content, "");
        assert_eq!(decoded.tool_calls.unwrap()[0].name, "super_json_set");
    }
}
