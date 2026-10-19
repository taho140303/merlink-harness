use super::*;

use crate::adapters::{AdapterReply, BoxedFuture, HttpBackend, ScriptedBackend};
use crate::models::{AgentEvent, ChatTurn, MessageEnvelope, ProviderConfig, ToolCall};
use crate::runtime::{
    NoopSink, ProgressSink, RecordingSink, assemble_prompt, session_messages,
    super_json_progress_for_session,
};
use crate::{db, store};
use serde_json::{Value, json};
use std::fs;
use std::time::{Duration, Instant};

fn event_tag(event: &AgentEvent) -> &'static str {
    match event {
        AgentEvent::TurnStarted { .. } => "turn_started",
        AgentEvent::RoundStarted { .. } => "round_started",
        AgentEvent::ToolStarted { .. } => "tool_started",
        AgentEvent::ToolFinished { .. } => "tool_finished",
        AgentEvent::RoundFinished { .. } => "round_finished",
        AgentEvent::TurnFinished { .. } => "turn_finished",
    }
}

/// A backend that sleeps on every call, so round and turn durations are
/// measurably above zero and the streaming tests can prove an event arrived
/// while a later round was still in flight.
struct SleepingBackend {
    replies: std::sync::Mutex<std::collections::VecDeque<AdapterReply>>,
    sleep: Duration,
}

impl SleepingBackend {
    fn new(replies: Vec<AdapterReply>, sleep: Duration) -> Self {
        Self {
            replies: std::sync::Mutex::new(replies.into()),
            sleep,
        }
    }
}

impl LlmBackend for SleepingBackend {
    fn complete<'a>(
        &'a self,
        _request: ChatRequest,
    ) -> BoxedFuture<'a, Result<AdapterReply, String>> {
        Box::pin(async move {
            std::thread::sleep(self.sleep);
            self.replies
                .lock()
                .unwrap()
                .pop_front()
                .ok_or_else(|| "SleepingBackend ran out of scripted replies".to_string())
        })
    }
}

fn message(id: &str, role: &str, content: &str) -> MessageEnvelope {
    MessageEnvelope::new(id, role, content)
}

fn keyed_providers() -> Vec<ProviderConfig> {
    let mut providers = store::catalog_fixture();
    providers[0].api_key = "test-key".into();
    providers
}

fn tool_call(id: &str, path: &str, value: Value) -> ToolCall {
    ToolCall {
        id: id.into(),
        name: "super_json_set".into(),
        arguments: json!({"path": path, "value": value}),
    }
}

fn project_create_call(id: &str, name: &str) -> ToolCall {
    ToolCall {
        id: id.into(),
        name: "project_create".into(),
        arguments: json!({"name": name}),
    }
}

#[test]
fn test_agent_turn_rejects_history_id_from_another_session() {
    let _home = crate::test_support::TempHome::new("runtime-cross-session-history");
    let providers = store::catalog_fixture();
    let backend = ScriptedBackend::default();
    let original = MessageEnvelope::user("shared-history-id", "original");
    tauri::async_runtime::block_on(run_agent_turn(
        &backend,
        &providers,
        "openai",
        "gpt-4o",
        &[original],
        "",
        "session-a",
        &NoopSink,
    ))
    .unwrap();

    let error = tauri::async_runtime::block_on(run_agent_turn(
        &backend,
        &providers,
        "openai",
        "gpt-4o",
        &[MessageEnvelope::user("shared-history-id", "replacement")],
        "",
        "session-b",
        &NoopSink,
    ))
    .unwrap_err();

    assert_eq!(
        error,
        "Cannot append message 'shared-history-id': ID already belongs to session 'session-a', not requested session 'session-b'"
    );
    assert_eq!(
        session_messages("session-a").unwrap()[0].content,
        "original"
    );
    assert!(session_messages("session-b").unwrap().is_empty());
}

#[test]
fn test_mock_chat_without_key_returns_metadata() {
    let _home = crate::test_support::TempHome::new("runtime-demo");
    let providers = store::catalog_fixture();
    let messages = vec![MessageEnvelope::user("first-message", "Hello")];

    let turn = tauri::async_runtime::block_on(run_agent_turn(
        &ScriptedBackend::default(),
        &providers,
        "openai",
        "gpt-4o",
        &messages,
        "default",
        "sess-demo",
        &NoopSink,
    ))
    .unwrap();

    assert_eq!(
        turn.project_id, "default",
        "an explicitly passed project id seeds a brand new session"
    );
    let appended = turn.messages;
    assert_eq!(appended.len(), 1, "demo mode answers in one message");
    assert_eq!(appended[0].role, "assistant");
    assert!(appended[0].content.contains("Demo mode"));
    let meta = appended[0].meta.clone().unwrap();
    assert_eq!(meta.session_id, "sess-demo");
    assert_eq!(meta.project_id, "default");
    assert_eq!(meta.prompt_hash.len(), 64);
}

/// The heart of the sprint: model asks for a tool, the tool writes the super
/// json, the model then answers.
#[test]
fn test_agent_loop_executes_tool_then_finishes() {
    let _home = crate::test_support::TempHome::new("runtime-loop");
    let providers = keyed_providers();
    let backend = ScriptedBackend::new(vec![
        AdapterReply::with_tool_calls(
            "",
            vec![tool_call(
                "call_1",
                "require.project.name",
                json!("Nửa Đêm"),
            )],
        ),
        AdapterReply::text("Đã đặt tên project thành công."),
    ]);

    let messages = vec![MessageEnvelope::user("u1", "Đặt tên project là Nửa Đêm")];
    let turn = tauri::async_runtime::block_on(run_agent_turn(
        &backend,
        &providers,
        "openai",
        "gpt-4o",
        &messages,
        "default",
        "sess-loop",
        &NoopSink,
    ))
    .unwrap();
    let appended = turn.messages;

    assert_eq!(appended.len(), 3);
    assert_eq!(
        appended.iter().map(|m| m.role.as_str()).collect::<Vec<_>>(),
        vec!["assistant", "tool", "assistant"]
    );
    assert_eq!(
        appended[0].tool_calls.as_ref().unwrap()[0].name,
        "super_json_set"
    );
    assert_eq!(appended[1].tool_call_id.as_deref(), Some("call_1"));
    assert_eq!(appended[1].name.as_deref(), Some("super_json_set"));
    assert_eq!(appended[2].content, "Đã đặt tên project thành công.");
    assert!(appended[2].meta.is_some(), "final answer carries metadata");

    // The tool actually mutated the project super json on disk.
    assert_eq!(
        crate::super_json::get("default", "require.project.name").unwrap(),
        json!("Nửa Đêm")
    );

    // The provider saw the tool result echoed back before answering again.
    assert_eq!(backend.request_count(), 2);
    let second = &backend.requests()[1].messages;
    assert_eq!(second.last().unwrap().role, "tool");

    // Everything is persisted, including the tool round trip.
    let stored = session_messages("sess-loop").unwrap();
    assert_eq!(stored.len(), 4, "user + assistant + tool + assistant");
    assert_eq!(stored[0].role, "user");
    assert_eq!(stored[2].role, "tool");
}

#[test]
fn test_agent_loop_provider_error_keeps_partial_transcript() {
    let _home = crate::test_support::TempHome::new("runtime-partial-failure");
    let providers = keyed_providers();
    let backend = ScriptedBackend::new(vec![AdapterReply::with_tool_calls(
        "",
        vec![ToolCall {
            id: "call_echo".into(),
            name: "echo".into(),
            arguments: json!({"value": "persisted"}),
        }],
    )]);
    let session = "sess-partial-failure";
    let error = tauri::async_runtime::block_on(run_agent_turn(
        &backend,
        &providers,
        "openai",
        "gpt-4o",
        &[MessageEnvelope::user("partial-user", "echo this")],
        "",
        session,
        &NoopSink,
    ))
    .unwrap_err();
    assert_eq!(error, "ScriptedBackend ran out of scripted replies");
    assert_eq!(backend.request_count(), 2);

    let connection = db::open().unwrap();
    let stored = db::list_messages(&connection, session).unwrap();
    assert_eq!(
        stored.len(),
        3,
        "user, assistant tool call, and tool result remain durable"
    );
    assert_eq!(
        stored
            .iter()
            .map(|message| message.role.as_str())
            .collect::<Vec<_>>(),
        ["user", "assistant", "tool"]
    );
    assert_eq!(stored[0].id, "partial-user");
    assert_eq!(stored[1].tool_calls.as_ref().unwrap()[0].id, "call_echo");
    assert_eq!(stored[2].tool_call_id.as_deref(), Some("call_echo"));
    assert_eq!(
        serde_json::from_str::<Value>(&stored[2].content).unwrap()["value"],
        json!("persisted")
    );
}

/// Sprint 11.12 · D9: reasoning from every round is concatenated onto the
/// final message only; intermediate assistant/tool messages stay clean.
#[test]
fn test_agent_loop_attaches_reasoning_to_final_message() {
    let _home = crate::test_support::TempHome::new("runtime-reasoning");
    let providers = keyed_providers();
    let backend = ScriptedBackend::new(vec![
        AdapterReply::with_tool_calls(
            "",
            vec![tool_call(
                "call_1",
                "require.project.name",
                json!("Nửa Đêm"),
            )],
        )
        .with_reasoning("Vòng 1: người dùng đã nêu tên truyện."),
        AdapterReply::text("Đã ghi nhận tên truyện.").with_reasoning("Vòng 2: xác nhận."),
    ]);

    let messages = vec![MessageEnvelope::user("u1", "Đặt tên project là Nửa Đêm")];
    let turn = tauri::async_runtime::block_on(run_agent_turn(
        &backend,
        &providers,
        "openai",
        "gpt-4o",
        &messages,
        "default",
        "sess-reasoning",
        &NoopSink,
    ))
    .unwrap();
    let appended = turn.messages;
    assert_eq!(appended.len(), 3);

    // Intermediate messages never carry reasoning ...
    assert_eq!(appended[0].reasoning, None, "tool-request message");
    assert_eq!(appended[1].reasoning, None, "tool result message");
    // ... only the final message does, with both rounds concatenated.
    assert_eq!(
        appended[2].reasoning.as_deref(),
        Some("Vòng 1: người dùng đã nêu tên truyện.\n\nVòng 2: xác nhận."),
        "the final message must carry every round's reasoning"
    );
    assert!(
        appended[2].meta.is_some(),
        "reasoning rides on the meta message"
    );

    // Reasoning is display-only: it is never handed back to the model.
    for request in backend.requests() {
        for message in request.messages {
            assert_eq!(
                message.reasoning, None,
                "reasoning must not go back on the wire"
            );
        }
    }

    // It is persisted with the final message, so a reload can show it too.
    let stored = session_messages("sess-reasoning").unwrap();
    let last = stored.last().unwrap();
    assert_eq!(
        last.reasoning.as_deref(),
        Some("Vòng 1: người dùng đã nêu tên truyện.\n\nVòng 2: xác nhận.")
    );
}

/// A failing tool must be reported to the model, not abort the turn.
#[test]
fn test_agent_loop_survives_tool_error() {
    let _home = crate::test_support::TempHome::new("runtime-tool-error");
    let providers = keyed_providers();
    let backend = ScriptedBackend::new(vec![
        AdapterReply::with_tool_calls(
            "",
            vec![tool_call(
                "call_bad",
                "require.project.plan.arc.count",
                json!("5"),
            )],
        ),
        AdapterReply::text("Xin lỗi, tôi cần một con số."),
    ]);

    let messages = vec![MessageEnvelope::user("u1", "Chia 5 arc")];
    let turn = tauri::async_runtime::block_on(run_agent_turn(
        &backend,
        &providers,
        "openai",
        "gpt-4o",
        &messages,
        "default",
        "sess-error",
        &NoopSink,
    ))
    .unwrap();
    let appended = turn.messages;

    assert_eq!(appended.len(), 3);
    let tool_message = &appended[1];
    assert_eq!(tool_message.role, "tool");
    assert!(
        tool_message.content.contains("\"error\""),
        "tool error must be handed back as JSON: {}",
        tool_message.content
    );
    assert_eq!(appended[2].content, "Xin lỗi, tôi cần một con số.");
    // The rejected value was never written.
    assert!(
        crate::super_json::get("default", "require.project.plan.arc.count")
            .unwrap()
            .is_null()
    );
}

#[test]
fn test_agent_loop_caps_at_max_rounds() {
    let _home = crate::test_support::TempHome::new("runtime-cap");
    let providers = keyed_providers();
    let prose = "This pre-tool prose must not become a final answer";
    let mut endless: Vec<_> = (0..MAX_TOOL_ROUNDS - 1)
        .map(|index| {
            AdapterReply::with_tool_calls(
                "",
                vec![ToolCall {
                    id: format!("call_{index}"),
                    name: "echo".into(),
                    arguments: json!({"round": index}),
                }],
            )
        })
        .collect();
    endless.push(
        AdapterReply::with_tool_calls(prose, vec![project_create_call("call_create", "Nửa Đêm")])
            .with_reasoning("Reasoning from the final round"),
    );
    let backend = ScriptedBackend::new(endless);
    let sink = RecordingSink::default();

    let messages = vec![MessageEnvelope::user("u1", "đặt tên mãi")];
    let turn = tauri::async_runtime::block_on(run_agent_turn(
        &backend, &providers, "openai", "gpt-4o", &messages, "", "sess-cap", &sink,
    ))
    .unwrap();
    let appended = &turn.messages;

    assert_eq!(backend.request_count(), MAX_TOOL_ROUNDS);
    let last = appended.last().unwrap();
    assert_eq!(last.role, "assistant");
    assert!(
        last.content.ends_with(STOP_MARKER),
        "last content must end with the stop marker: {}",
        last.content
    );
    assert_eq!(last.content, STOP_MARKER);
    assert!(
        !last.content.contains(prose),
        "pre-tool prose must not be recycled"
    );
    assert_eq!(
        last.reasoning.as_deref(),
        Some("Reasoning from the final round")
    );
    assert_eq!(turn.project_id, "n-a-m");
    assert_eq!(last.meta.as_ref().unwrap().project_id, turn.project_id);
    assert_eq!(
        session_messages("sess-cap")
            .unwrap()
            .last()
            .unwrap()
            .content,
        STOP_MARKER
    );
    assert!(matches!(
        sink.events().last().unwrap(),
        AgentEvent::TurnFinished { project_id, rounds, .. }
            if project_id == &turn.project_id && *rounds == MAX_TOOL_ROUNDS as u32
    ));
    assert!(
        last.tool_calls.is_none(),
        "the stop message requests no tools"
    );
    // 5 rounds x (assistant + tool) + the stop message.
    assert_eq!(appended.len(), MAX_TOOL_ROUNDS * 2 + 1);
}

#[test]
fn test_run_agent_turn_validates_provider_and_model() {
    let _home = crate::test_support::TempHome::new("runtime-validate");
    let providers = keyed_providers();
    let messages = vec![MessageEnvelope::user("u1", "hi")];

    let unknown_provider = tauri::async_runtime::block_on(run_agent_turn(
        &ScriptedBackend::default(),
        &providers,
        "nope",
        "gpt-4o",
        &messages,
        "default",
        "s",
        &NoopSink,
    ))
    .unwrap_err();
    assert_eq!(unknown_provider, "Provider not found");

    let unknown_model = tauri::async_runtime::block_on(run_agent_turn(
        &ScriptedBackend::default(),
        &providers,
        "openai",
        "nope",
        &messages,
        "default",
        "s",
        &NoopSink,
    ))
    .unwrap_err();
    assert_eq!(unknown_model, "Model does not belong to provider");

    let no_user = tauri::async_runtime::block_on(run_agent_turn(
        &ScriptedBackend::default(),
        &providers,
        "openai",
        "gpt-4o",
        &[message("a1", "assistant", "hello")],
        "default",
        "s",
        &NoopSink,
    ))
    .unwrap_err();
    assert_eq!(no_user, "A user message is required");
}

#[test]
fn test_system_prompt_hash_is_stable() {
    let prompt = assemble_prompt("OpenAI", "gpt-4o", "session", "project");
    assert_eq!(hash_prompt(&prompt).len(), 64);
    assert!(!prompt.contains("{provider_name}"));
    assert!(prompt.contains("Runtime: OpenAI / gpt-4o"));
}

/// Sprint 11.12 · D6: the prompt caps questions per reply and forbids
/// turning one field into a menu of sub-questions.
#[test]
fn test_system_prompt_limits_questions_per_turn() {
    let prompt = assemble_prompt("OpenAI", "gpt-4o", "s", "p");
    assert!(
        prompt.contains("# Asking the author"),
        "the prompt has no asking-the-author section"
    );
    assert!(
        prompt.contains("Ask **at most three questions per reply**"),
        "the three-question cap is missing"
    );
    assert!(
        prompt.contains("One decision per question."),
        "the one-question-per-field rule is missing"
    );
    assert!(
        prompt.contains("Never re-ask a field that is already set"),
        "the no-re-ask rule is missing"
    );
    assert!(
        !prompt.contains("Derive `name` from what the author just said"),
        "the old invent-a-title instruction is still in the prompt"
    );
}

/// Sprint 11.12 · D7: the prompt forbids deriving or inventing values, and
/// tells the model to create an unnamed project instead of making a title up.
#[test]
fn test_system_prompt_forbids_inventing_values() {
    let prompt = assemble_prompt("OpenAI", "gpt-4o", "s", "p");
    assert!(
        prompt.contains("# Only write what the author actually said"),
        "the write-only-what-was-said section is missing"
    );
    for expected in [
        "writes **exactly one** field with **exactly the value the author stated**",
        "Never invent a value to fill a field",
        "the only thing you may write is",
        "`require.project.plan.chapter.count` = 120",
        "`project_create` **without** a name",
        "**Never make one up.**",
        "When it is unclear which field the author meant, ask instead of writing.",
    ] {
        assert!(
            prompt.contains(expected),
            "the prompt is missing {expected:?}"
        );
    }
}

/// The prompt has to say out loud that there is no project yet, otherwise the
/// model has no reason to call `project_create` (D2/D5).
fn assert_prompt_reports_project_state() {
    let none = assemble_prompt("OpenAI", "gpt-4o", "s", "");
    assert!(
        !none.contains("{project_state}"),
        "placeholder must be substituted"
    );
    let header = none
        .lines()
        .find(|line| line.starts_with("Project:"))
        .expect("the prompt has a Project: identity line");
    assert!(
        header.starts_with("Project: (none yet - call project_create"),
        "unexpected identity line: {header}"
    );

    let bound = assemble_prompt("OpenAI", "gpt-4o", "s", "nua-dem");
    let header = bound
        .lines()
        .find(|line| line.starts_with("Project:"))
        .expect("the prompt has a Project: identity line");
    assert_eq!(header, "Project: nua-dem");
}

/// Acceptance: the first message of a chat with no project creates it, later
/// rounds of the same turn already see it, and the project lands on disk (D5).
#[test]
fn test_agent_loop_creates_project_on_first_message() {
    assert_prompt_reports_project_state();
    let home = crate::test_support::TempHome::new("runtime-create-project");
    let providers = keyed_providers();
    let backend = ScriptedBackend::new(vec![
        AdapterReply::with_tool_calls("", vec![project_create_call("call_create", "Nửa Đêm")]),
        // Round 2 runs in the same turn: without D5 the effective project would
        // still be empty here and this call would fail.
        AdapterReply::with_tool_calls(
            "",
            vec![tool_call(
                "call_set",
                "require.schema.project_genres.count",
                json!(1),
            )],
        ),
        AdapterReply::text("Đã tạo project Nửa Đêm."),
    ]);

    let messages = vec![MessageEnvelope::user(
        "u1",
        "Tôi muốn viết truyện trinh thám về một thợ đồng hồ mù",
    )];
    let turn = tauri::async_runtime::block_on(run_agent_turn(
        &backend,
        &providers,
        "openai",
        "gpt-4o",
        &messages,
        "",
        "sess-create",
        &NoopSink,
    ))
    .unwrap();

    assert_eq!(
        turn.project_id, "n-a-m",
        "the turn reports the project it created"
    );
    assert_eq!(
        turn.messages[0].tool_calls.as_ref().unwrap()[0].name,
        "project_create"
    );

    // The session row carries the binding, which is what later turns read.
    let connection = db::open().unwrap();
    assert_eq!(
        db::project_for_session(&connection, "sess-create")
            .unwrap()
            .as_deref(),
        Some("n-a-m")
    );

    // Round 1's tool result reports the new id; round 2's `super_json_set`
    // succeeded against it.
    let created: Value = serde_json::from_str(&turn.messages[1].content).unwrap();
    assert_eq!(created["project_id"], json!("n-a-m"));
    assert_eq!(created["name"], json!("Nửa Đêm"));
    assert_eq!(created["missing_required"].as_array().unwrap().len(), 17);
    let applied: Value = serde_json::from_str(&turn.messages[3].content).unwrap();
    assert_eq!(applied["ok"], json!(true), "{applied}");
    assert_eq!(applied["value"], json!(1));

    // `projects/<id>/super.json` exists with the name already set.
    let path = crate::paths::super_json_path("n-a-m").unwrap();
    assert!(path.starts_with(home.path()), "{}", path.display());
    assert!(path.exists(), "super.json missing at {}", path.display());
    let on_disk: Value = serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
    assert_eq!(on_disk["require"]["project"]["name"], json!("Nửa Đêm"));
    assert_eq!(on_disk["require"]["schema"][0]["count"], json!(1));

    // The second round's system prompt already names the project (D5).
    let requests = backend.requests();
    assert!(requests[0].system.contains("(none yet"), "round 1 prompt");
    assert!(
        requests[1].system.contains("Project: n-a-m"),
        "round 2 prompt"
    );
}

/// D3 step 2, end to end: the second `project_create` of a turn is refused, the
/// turn still finishes, and only one project exists.
#[test]
fn test_agent_loop_second_project_creation_is_rejected() {
    let _home = crate::test_support::TempHome::new("runtime-create-twice");
    let providers = keyed_providers();
    let backend = ScriptedBackend::new(vec![
        AdapterReply::with_tool_calls("", vec![project_create_call("call_1", "Nửa Đêm")]),
        AdapterReply::with_tool_calls("", vec![project_create_call("call_2", "Truyện khác")]),
        AdapterReply::text("Chat này đã có project rồi."),
    ]);

    let messages = vec![MessageEnvelope::user("u1", "viết truyện trinh thám")];
    let turn = tauri::async_runtime::block_on(run_agent_turn(
        &backend,
        &providers,
        "openai",
        "gpt-4o",
        &messages,
        "",
        "sess-twice",
        &NoopSink,
    ))
    .unwrap();

    assert_eq!(turn.project_id, "n-a-m");
    let first: Value = serde_json::from_str(&turn.messages[1].content).unwrap();
    assert_eq!(first["ok"], json!(true));
    let second: Value = serde_json::from_str(&turn.messages[3].content).unwrap();
    assert_eq!(
        second["error"].as_str().unwrap(),
        "This chat already belongs to project n-a-m"
    );
    // The failure is handed back to the model as data, not raised as an error.
    assert_eq!(turn.messages[4].content, "Chat này đã có project rồi.");

    let connection = db::open().unwrap();
    assert_eq!(
        db::list_projects(&connection).unwrap(),
        vec!["n-a-m".to_string()],
        "the rejected call must not create a second project"
    );
    assert!(!crate::paths::project_dir("truy-n-kh-c").unwrap().exists());
}

/// D2/D4: the exact order the UI relies on, and one `ToolStarted` per real
/// tool call.
#[test]
fn test_agent_loop_emits_ordered_events() {
    let _home = crate::test_support::TempHome::new("runtime-events-order");
    let providers = keyed_providers();
    let backend = ScriptedBackend::new(vec![
        AdapterReply::with_tool_calls("", vec![project_create_call("call_a", "Nửa Đêm")]),
        AdapterReply::with_tool_calls(
            "",
            vec![tool_call(
                "call_b",
                "require.schema.project_genres.count",
                json!(1),
            )],
        ),
        AdapterReply::text("Xong."),
    ]);
    let sink = RecordingSink::default();

    let messages = vec![MessageEnvelope::user("u1", "viết truyện trinh thám")];
    let turn = tauri::async_runtime::block_on(run_agent_turn(
        &backend,
        &providers,
        "openai",
        "gpt-4o",
        &messages,
        "",
        "sess-ordered",
        &sink,
    ))
    .unwrap();
    assert_eq!(turn.project_id, "n-a-m");

    let events = sink.events();
    let tags: Vec<&str> = events.iter().map(event_tag).collect();
    assert_eq!(
        tags,
        vec![
            "turn_started",
            "round_started",
            "tool_started",
            "tool_finished",
            "round_finished",
            "round_started",
            "tool_started",
            "tool_finished",
            "round_finished",
            "round_started",
            "round_finished",
            "turn_finished",
        ]
    );

    let tool_starts = events
        .iter()
        .filter(|event| matches!(event, AgentEvent::ToolStarted { .. }))
        .count();
    let tool_calls: usize = turn
        .messages
        .iter()
        .filter_map(|message| message.tool_calls.as_ref())
        .map(Vec::len)
        .sum();
    assert_eq!(tool_starts, tool_calls);
    assert_eq!(tool_starts, 2);

    // Rounds are 1-based and every `RoundFinished` pairs with its start.
    let rounds: Vec<u32> = events
        .iter()
        .filter_map(|event| match event {
            AgentEvent::RoundStarted { round } => Some(*round),
            _ => None,
        })
        .collect();
    assert_eq!(rounds, vec![1, 2, 3]);
}

#[test]
fn test_tool_finished_observes_persisted_result() {
    struct InspectingSink {
        session: &'static str,
        persisted_at_finish: std::sync::Mutex<Vec<bool>>,
    }
    impl ProgressSink for InspectingSink {
        fn emit(&self, event: AgentEvent) {
            if let AgentEvent::ToolFinished { id, .. } = event {
                let connection = db::open().unwrap();
                let found = db::list_messages(&connection, self.session)
                    .unwrap()
                    .iter()
                    .any(|message| {
                        message.role == "tool" && message.tool_call_id.as_deref() == Some(&id)
                    });
                self.persisted_at_finish.lock().unwrap().push(found);
            }
        }
    }
    let _home = crate::test_support::TempHome::new("runtime-tool-finish-persisted");
    let backend = ScriptedBackend::new(vec![
        AdapterReply::with_tool_calls(
            "",
            vec![ToolCall {
                id: "call_echo".into(),
                name: "echo".into(),
                arguments: json!({"value": "stored"}),
            }],
        ),
        AdapterReply::text("done"),
    ]);
    let sink = InspectingSink {
        session: "sess-tool-finish-persisted",
        persisted_at_finish: std::sync::Mutex::new(Vec::new()),
    };
    tauri::async_runtime::block_on(run_agent_turn(
        &backend,
        &keyed_providers(),
        "openai",
        "gpt-4o",
        &[MessageEnvelope::user("u1", "echo")],
        "",
        sink.session,
        &sink,
    ))
    .unwrap();
    assert_eq!(*sink.persisted_at_finish.lock().unwrap(), vec![true]);
}

#[test]
fn test_tool_result_append_failure_reports_tool_ran_but_result_not_stored() {
    struct BlockingSink {
        events: std::sync::Mutex<Vec<AgentEvent>>,
    }
    impl ProgressSink for BlockingSink {
        fn emit(&self, event: AgentEvent) {
            if matches!(event, AgentEvent::ToolStarted { .. }) {
                // A trigger forces only the tool-result insert to fail, after
                // the user and the assistant call have already been stored.
                db::open()
                    .unwrap()
                    .execute_batch(
                        "CREATE TRIGGER block_tool_results BEFORE INSERT ON messages \
                         WHEN NEW.role = 'tool' BEGIN SELECT RAISE(ABORT, 'tool result blocked'); END;",
                    )
                    .unwrap();
            }
            self.events.lock().unwrap().push(event);
        }
    }
    let _home = crate::test_support::TempHome::new("runtime-tool-result-append-failure");
    let backend = ScriptedBackend::new(vec![AdapterReply::with_tool_calls(
        "",
        vec![ToolCall {
            id: "call_echo".into(),
            name: "echo".into(),
            arguments: json!({"value": "ran"}),
        }],
    )]);
    let sink = BlockingSink {
        events: std::sync::Mutex::new(Vec::new()),
    };
    let error = tauri::async_runtime::block_on(run_agent_turn(
        &backend,
        &keyed_providers(),
        "openai",
        "gpt-4o",
        &[MessageEnvelope::user("u1", "echo")],
        "",
        "sess-blocked-result",
        &sink,
    ))
    .unwrap_err();
    assert!(
        error.contains("ran but its result could not be stored"),
        "{error}"
    );
    let events = sink.events.lock().unwrap();
    let started_status = events
        .iter()
        .find_map(|event| match event {
            AgentEvent::ToolStarted { status, .. } => Some(status),
            _ => None,
        })
        .unwrap();
    let (ok, finished_status) = events
        .iter()
        .find_map(|event| match event {
            AgentEvent::ToolFinished { ok, status, .. } => Some((ok, status)),
            _ => None,
        })
        .unwrap();
    assert!(!ok);
    assert_eq!(started_status, finished_status);
    let connection = db::open().unwrap();
    let stored = db::list_messages(&connection, "sess-blocked-result").unwrap();
    assert_eq!(
        stored
            .iter()
            .map(|message| message.role.as_str())
            .collect::<Vec<_>>(),
        ["user", "assistant"]
    );
}

/// D5: `duration_ms` is a real measurement, not a constant. `echo` clones its
/// argument, so a large payload makes the elapsed time provably non-zero.
#[test]
fn test_tool_finished_event_carries_duration() {
    let _home = crate::test_support::TempHome::new("runtime-tool-duration");
    let providers = keyed_providers();
    let payload = "x".repeat(24 * 1024 * 1024);
    let backend = ScriptedBackend::new(vec![
        AdapterReply::with_tool_calls(
            "",
            vec![ToolCall {
                id: "call_echo".into(),
                name: "echo".into(),
                arguments: json!({ "blob": payload }),
            }],
        ),
        AdapterReply::text("xong"),
    ]);
    let sink = RecordingSink::default();

    let messages = vec![MessageEnvelope::user("u1", "echo thử")];
    tauri::async_runtime::block_on(run_agent_turn(
        &backend,
        &providers,
        "openai",
        "gpt-4o",
        &messages,
        "",
        "sess-tool-duration",
        &sink,
    ))
    .unwrap();

    let finished: Vec<(bool, u64)> = sink
        .events()
        .iter()
        .filter_map(|event| match event {
            AgentEvent::ToolFinished {
                ok, duration_ms, ..
            } => Some((*ok, *duration_ms)),
            _ => None,
        })
        .collect();
    assert_eq!(finished.len(), 1, "exactly one tool ran");
    let (ok, duration_ms) = finished[0];
    assert!(ok, "echo always succeeds");
    assert!(
        duration_ms > 0,
        "a tool that does real work must report a measured duration, got {duration_ms}ms"
    );
}

/// D5: the per-round duration includes the model call, so a sleeping backend
/// must be visible in `RoundFinished`.
#[test]
fn test_round_finished_event_carries_duration() {
    let _home = crate::test_support::TempHome::new("runtime-round-duration");
    let providers = keyed_providers();
    let backend = SleepingBackend::new(vec![AdapterReply::text("xong")], Duration::from_millis(45));
    let sink = RecordingSink::default();

    let messages = vec![MessageEnvelope::user("u1", "hi")];
    tauri::async_runtime::block_on(run_agent_turn(
        &backend,
        &providers,
        "openai",
        "gpt-4o",
        &messages,
        "",
        "sess-round-duration",
        &sink,
    ))
    .unwrap();

    let durations: Vec<u64> = sink
        .events()
        .iter()
        .filter_map(|event| match event {
            AgentEvent::RoundFinished { duration_ms, .. } => Some(*duration_ms),
            _ => None,
        })
        .collect();
    assert_eq!(durations.len(), 1);
    assert!(
        durations[0] >= 40,
        "the round duration must include the 45ms model call, got {}ms",
        durations[0]
    );
}

/// D2/D4: `TurnFinished` reports the project the turn ended on and how many
/// model rounds it actually used.
#[test]
fn test_turn_finished_reports_project_and_rounds() {
    let _home = crate::test_support::TempHome::new("runtime-turn-finished");
    let providers = keyed_providers();
    let backend = ScriptedBackend::new(vec![
        AdapterReply::with_tool_calls("", vec![project_create_call("call_create", "Nửa Đêm")]),
        AdapterReply::with_tool_calls(
            "",
            vec![tool_call(
                "call_set",
                "require.schema.project_genres.count",
                json!(1),
            )],
        ),
        AdapterReply::text("Đã tạo project."),
    ]);
    let sink = RecordingSink::default();

    let messages = vec![MessageEnvelope::user("u1", "viết truyện trinh thám")];
    let turn = tauri::async_runtime::block_on(run_agent_turn(
        &backend,
        &providers,
        "openai",
        "gpt-4o",
        &messages,
        "",
        "sess-turn-finished",
        &sink,
    ))
    .unwrap();

    let finished = sink
        .events()
        .into_iter()
        .find_map(|event| match event {
            AgentEvent::TurnFinished {
                project_id,
                duration_ms,
                rounds,
            } => Some((project_id, duration_ms, rounds)),
            _ => None,
        })
        .expect("a turn must end with TurnFinished");
    assert_eq!(finished.0, turn.project_id);
    assert_eq!(finished.0, "n-a-m");
    assert_eq!(finished.2, 3, "three model rounds actually ran");
    assert!(
        finished.1 < 60_000,
        "sanity: the turn did not take a minute"
    );
}

/// The acceptance proof of real streaming: the first `ToolStarted` must be
/// recorded while a later model round is still running, i.e. well before
/// `run_agent_turn` returns.
#[test]
fn test_events_are_emitted_before_the_turn_returns() {
    let _home = crate::test_support::TempHome::new("runtime-events-early");
    let providers = keyed_providers();
    // Both rounds sleep 80ms, so the first tool event lands ~80ms before the
    // turn ends instead of being batched at the end.
    let backend = SleepingBackend::new(
        vec![
            AdapterReply::with_tool_calls("", vec![project_create_call("call_create", "Nửa Đêm")]),
            AdapterReply::text("Đã tạo project."),
        ],
        Duration::from_millis(80),
    );
    let sink = RecordingSink::default();

    let messages = vec![MessageEnvelope::user("u1", "viết truyện")];
    tauri::async_runtime::block_on(run_agent_turn(
        &backend,
        &providers,
        "openai",
        "gpt-4o",
        &messages,
        "",
        "sess-streaming",
        &sink,
    ))
    .unwrap();
    let returned = Instant::now();

    let first_tool = sink
        .first_instant(|event| matches!(event, AgentEvent::ToolStarted { .. }))
        .expect("the first tool must emit ToolStarted");
    assert!(
        first_tool < returned,
        "the event must be recorded before the turn returns"
    );
    let gap = returned.duration_since(first_tool);
    assert!(
        gap >= Duration::from_millis(40),
        "the first ToolStarted must arrive while the turn is still running (round 2 was still \
         in flight); gap was {gap:?}"
    );
}

/// A failing tool still announces itself, reports `ok: false`, and lets the
/// turn finish normally.
#[test]
fn test_failed_tool_emits_tool_finished_with_ok_false() {
    let _home = crate::test_support::TempHome::new("runtime-failed-tool");
    let providers = keyed_providers();
    let backend = ScriptedBackend::new(vec![
        AdapterReply::with_tool_calls(
            "",
            vec![tool_call(
                "call_bad",
                "require.project.plan.arc.count",
                json!("5"),
            )],
        ),
        AdapterReply::text("Cần một con số."),
    ]);
    let sink = RecordingSink::default();

    let messages = vec![MessageEnvelope::user("u1", "chia 5 arc")];
    let turn = tauri::async_runtime::block_on(run_agent_turn(
        &backend,
        &providers,
        "openai",
        "gpt-4o",
        &messages,
        "",
        "sess-failed-tool",
        &sink,
    ))
    .unwrap();
    assert_eq!(turn.messages.last().unwrap().content, "Cần một con số.");

    let events = sink.events();
    assert_eq!(event_tag(events.last().unwrap()), "turn_finished");
    let finished: Vec<(bool, u64)> = events
        .iter()
        .filter_map(|event| match event {
            AgentEvent::ToolFinished {
                ok, duration_ms, ..
            } => Some((*ok, *duration_ms)),
            _ => None,
        })
        .collect();
    assert_eq!(finished.len(), 1);
    assert!(!finished[0].0, "the rejected set must report ok = false");
}

/// D4: a chat without an API key answers in demo mode but still brackets the
/// turn with `TurnStarted`/`TurnFinished`.
#[test]
fn test_demo_mode_still_emits_turn_events() {
    let _home = crate::test_support::TempHome::new("runtime-demo-events");
    let providers = store::catalog_fixture();
    let sink = RecordingSink::default();

    let messages = vec![MessageEnvelope::user("u1", "hello")];
    let turn = tauri::async_runtime::block_on(run_agent_turn(
        &ScriptedBackend::default(),
        &providers,
        "openai",
        "gpt-4o",
        &messages,
        "default",
        "sess-demo-events",
        &sink,
    ))
    .unwrap();
    assert!(turn.messages[0].content.contains("Demo mode"));

    let events = sink.events();
    assert_eq!(
        events.len(),
        2,
        "demo mode has no rounds or tools: {events:?}"
    );
    assert_eq!(event_tag(&events[0]), "turn_started");
    assert_eq!(event_tag(&events[1]), "turn_finished");
    match &events[1] {
        AgentEvent::TurnFinished { rounds, .. } => assert_eq!(*rounds, 0),
        other => panic!("expected TurnFinished, got {other:?}"),
    }
}

/// The dev endpoint's key is never committed (Sprint 12 · Q3).
/// Export `MERLINK_E2E_API_KEY` to run the `#[ignore]` network tests; the
/// fallback is a placeholder that will simply be rejected by the endpoint.
fn e2e_api_key() -> String {
    std::env::var("MERLINK_E2E_API_KEY").unwrap_or_else(|_| "local-dev-placeholder".into())
}

fn agtron_providers() -> Vec<ProviderConfig> {
    // Sprint 11.13 · D2: there is no shipped catalog to borrow a provider
    // from any more, so the fixture builds its own provider outright.
    vec![ProviderConfig {
        id: "agtron".into(),
        name: "Agtron (local)".into(),
        kind: "openai".into(),
        api_key: e2e_api_key(),
        base_url: "http://127.0.0.1:8045/v1".into(),
        models: vec![crate::models::ModelConfig {
            id: "gpt-4o".into(),
            name: "gpt-4o".into(),
            context_window: "128K".into(),
            max_tokens: "4K".into(),
            input_types: crate::models::InputTypes {
                text: true,
                image: false,
            },
            reasoning_level: false,
        }],
        default_model_id: "gpt-4o".into(),
    }]
}

/// Prints the real per-tool / per-round / per-turn timing table the sprint
/// report quotes. Every number comes from the `AgentEvent` stream, i.e. from
/// the same measurement the UI receives (D5).
fn print_timing_table(label: &str, events: &[AgentEvent]) {
    eprintln!("--- timing table: {label} ---");
    eprintln!(
        "{:<30} {:<46} {:>7} {:>5}",
        "tool name", "status", "ms", "ok"
    );
    for event in events {
        if let AgentEvent::ToolFinished {
            name,
            ok,
            duration_ms,
            status,
            ..
        } = event
        {
            eprintln!("{name:<30} {status:<46} {duration_ms:>7} {ok:>5}");
        }
    }
    eprintln!("{:<30} {:>7} {:>7}", "round", "ms", "tools");
    for event in events {
        if let AgentEvent::RoundFinished {
            round,
            duration_ms,
            tool_calls,
        } = event
        {
            eprintln!("{round:<30} {duration_ms:>7} {tool_calls:>7}");
        }
    }
    let rounds = events
        .iter()
        .filter(|event| matches!(event, AgentEvent::RoundStarted { .. }))
        .count();
    let tool_calls = events
        .iter()
        .filter(|event| matches!(event, AgentEvent::ToolStarted { .. }))
        .count();
    match events.last() {
        Some(AgentEvent::TurnFinished {
            duration_ms,
            rounds: reported,
            project_id,
        }) => eprintln!(
            "turn total: {duration_ms}ms | rounds={reported} (started={rounds}) | tool_calls={tool_calls} | project_id={project_id}"
        ),
        _ => eprintln!("turn total: (no TurnFinished) | rounds={rounds} | tool_calls={tool_calls}"),
    }
}

async fn real_turn(
    providers: &[ProviderConfig],
    session: &str,
    instruction: &str,
) -> (ChatTurn, Vec<AgentEvent>) {
    // Message ids must be unique per turn: `append_message` is an upsert, so a
    // repeated id would replace the earlier row instead of appending to it.
    let stamp = chrono::Utc::now().timestamp_nanos_opt().unwrap_or_default();
    let messages = vec![MessageEnvelope::user(
        format!("u-{session}-{stamp}"),
        instruction,
    )];
    let sink = RecordingSink::default();
    // No project id on purpose: the session owns the binding (D1), and the
    // first turn of a fresh chat has none.
    let turn = run_agent_turn(
        &HttpBackend,
        providers,
        "agtron",
        "gpt-4o",
        &messages,
        "",
        session,
        &sink,
    )
    .await
    .expect("real endpoint turn failed");
    (turn, sink.events())
}

/// Online end-to-end against the local dev endpoint. Ignored by default so
/// `cargo test` never needs a network, and it *requires* `MERLINK_HOME` so
/// the artifacts can be inspected afterwards:
///
/// ```text
/// MERLINK_HOME=/tmp/merlink-e2e cargo test -- --ignored --nocapture real_endpoint
/// ```
#[test]
#[ignore = "requires the local dev endpoint on 127.0.0.1:8045 and MERLINK_HOME"]
fn test_real_endpoint_agent_loop() {
    let home = crate::test_support::RequiredHome::new();
    eprintln!("--- e2e home: {} ---", home.root.display());
    let providers = agtron_providers();
    let session = "sess-real-project";

    tauri::async_runtime::block_on(async {
        // Turn 1: the chat has no project, so the model must create one from
        // the prompt itself (D3). This is the sprint's D8 measurement turn.
        let (first, first_events) = real_turn(
            &providers,
            session,
            "Tôi muốn viết truyện trinh thám về một thợ đồng hồ mù",
        )
        .await;
        print_timing_table("turn 1 (open project) - D8 measurement", &first_events);
        eprintln!(
            "--- D8 measurement: opening turn used {} model rounds / {} tool calls \
             (baseline from the bug report: 3 rounds / 5 tool calls) ---",
            first_events
                .iter()
                .filter(|event| matches!(event, AgentEvent::RoundStarted { .. }))
                .count(),
            first_events
                .iter()
                .filter(|event| matches!(event, AgentEvent::ToolStarted { .. }))
                .count(),
        );
        eprintln!("--- turn 1 project_id = {:?} ---", first.project_id);

        assert!(
            first.messages.iter().any(|message| message
                .tool_calls
                .as_ref()
                .is_some_and(|calls| calls.iter().any(|call| call.name == "project_create"))),
            "the model never called project_create: {:#?}",
            first.messages
        );
        assert!(!first.project_id.is_empty(), "the turn reported no project");
        let project_id = first.project_id.clone();

        // The project directory and its super json exist on disk, with the
        // name already filled in.
        let super_json_path = crate::paths::super_json_path(&project_id).unwrap();
        assert!(
            super_json_path.exists(),
            "super.json missing at {}",
            super_json_path.display()
        );
        let on_disk: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(&super_json_path).unwrap()).unwrap();
        eprintln!(
            "--- {} ---\n{}",
            super_json_path.display(),
            serde_json::to_string_pretty(&on_disk).unwrap()
        );
        assert!(
            !on_disk["require"]["project"]["name"].is_null(),
            "require.project.name is still null: {on_disk}"
        );

        // Real turns to fill the remaining required fields. Each turn may use
        // up to MAX_TOOL_ROUNDS model -> tool round trips.
        for (index, instruction) in [
            "Logline: sự kiện mở đầu là chiếc đồng hồ cuối cùng ngừng chạy, nhân vật chính là \
             một người thợ đồng hồ mù, hành động là truy tìm người đã chế tạo ra nó, rủi ro là \
             đánh mất ký ức, mất mát lớn là đứa con gái duy nhất. Ghi tất cả vào super json.",
            "Kế hoạch truyện: 5 arc, 120 chương, khoảng 90000 từ. Thể loại chính là trinh thám. \
             Câu hỏi chủ đề là ký ức có làm nên con người hay không. Ghi hết vào super json.",
            "Hãy gọi super_json_missing rồi điền nốt tất cả các trường bắt buộc còn thiếu trong \
             super json bằng giá trị hợp lý.",
        ]
        .into_iter()
        .enumerate()
        {
            let (turn, events) = real_turn(&providers, session, instruction).await;
            print_timing_table(&format!("turn {}", index + 2), &events);
            assert_eq!(turn.project_id, project_id, "the binding must not move");
        }

        let progress = super_json_progress_for_session(session).unwrap();
        eprintln!("--- progress {project_id} ---\n{progress:#?}");
        let missing = crate::super_json::missing_required(&project_id).unwrap();
        eprintln!("--- still missing ({}): {missing:#?} ---", missing.len());
        let final_document: serde_json::Value = serde_json::from_str(
            &fs::read_to_string(crate::paths::super_json_path(&project_id).unwrap()).unwrap(),
        )
        .unwrap();
        eprintln!(
            "--- {} ---\n{}",
            crate::paths::super_json_path(&project_id)
                .unwrap()
                .display(),
            serde_json::to_string_pretty(&final_document).unwrap()
        );

        // The command the header calls, on the same session.
        let command_progress = crate::super_json_progress(session.to_owned()).unwrap();
        assert_eq!(
            command_progress.filled_required, 11,
            "required fields: {command_progress:#?}"
        );
        assert_eq!(command_progress.percent, 70);
        assert_eq!(command_progress.label, "Độ mạch lạc tạm chấp nhận");
        assert_eq!(progress.filled_required, 11);

        // What the UI reads after a page reload: the persisted transcript,
        // including the tool round trips.
        let stored = session_messages(session).unwrap();
        eprintln!(
            "--- reload transcript {session}: {} messages ---",
            stored.len()
        );
        assert_eq!(stored[0].role, "user");
        assert!(stored.iter().any(|message| message.role == "tool"));
        assert!(stored.iter().any(|message| message.tool_calls.is_some()));
        assert!(stored.last().unwrap().meta.is_some());

        // A second chat creating a project with the same title must not collide.
        let (other, other_events) = real_turn(
            &providers,
            "sess-real-project-2",
            "Tôi cũng muốn viết một truyện trinh thám về một thợ đồng hồ mù",
        )
        .await;
        print_timing_table("second chat (open project)", &other_events);
        assert!(!other.project_id.is_empty());
        assert_ne!(
            other.project_id, project_id,
            "two chats must not share one project directory"
        );
    });
}

/// Online end-to-end for the real model list (Sprint 11.5, acceptance).
///
/// ```text
/// MERLINK_HOME=/tmp/merlink-e2e cargo test -- --ignored --nocapture real_endpoint_fetch
/// ```
#[test]
#[ignore = "requires the local dev endpoint on 127.0.0.1:8045"]
fn test_real_endpoint_fetch_models() {
    let models = tauri::async_runtime::block_on(crate::adapters::fetch_models(
        "openai",
        "http://127.0.0.1:8045/v1",
        &e2e_api_key(),
    ))
    .expect("real fetch_models failed");

    eprintln!("--- fetch_models(openai, http://127.0.0.1:8045/v1) ---");
    eprintln!("--- {} models returned ---", models.len());
    for model in models.iter().take(12) {
        eprintln!("{:<34} {}", model.id, model.name);
    }
    eprintln!("--- ... ({} more) ---", models.len().saturating_sub(12));

    assert!(!models.is_empty(), "the real endpoint returned no models");
    assert!(
        models.iter().any(|model| model.id == "gpt-4o"),
        "the real list must contain gpt-4o: {:?}",
        models.iter().map(|m| &m.id).collect::<Vec<_>>()
    );
    // D2: every entry has a usable label, and ids are unique.
    assert!(models.iter().all(|model| !model.name.trim().is_empty()));
    let mut ids: Vec<&str> = models.iter().map(|model| model.id.as_str()).collect();
    ids.sort_unstable();
    let unique = ids.len();
    ids.dedup();
    assert_eq!(ids.len(), unique, "the provider list must not repeat ids");
}

#[test]
#[ignore = "requires the local dev endpoint on 127.0.0.1:8045"]
fn test_real_endpoint_fetch_models_bad_key() {
    let error = tauri::async_runtime::block_on(crate::adapters::fetch_models(
        "openai",
        "http://127.0.0.1:8045/v1",
        "invalid-key-for-403-test",
    ))
    .expect_err("a wrong API key must not return a model list");

    eprintln!("--- fetch_models with a wrong api_key ---");
    eprintln!("{error}");

    assert!(
        error.contains("403") || error.contains("401"),
        "the error must carry the HTTP status: {error}"
    );
}
