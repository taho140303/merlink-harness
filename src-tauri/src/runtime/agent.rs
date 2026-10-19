//! The agent turn: model -> tools -> model, capped at [`MAX_TOOL_ROUNDS`].
use super::events::{ChannelSink, ProgressSink, elapsed_ms};
use super::prompt::assemble_prompt;
use super::session::{build_meta, derive_session_id, effective_project};
use crate::adapters::{ChatRequest, HttpBackend, LlmBackend};
use crate::models::{AgentEvent, ChatTurn, MessageEnvelope, ProviderConfig, TokenUsage};
use crate::{db, store, tools};
use chrono::Utc;
use serde_json::json;
use sha2::{Digest, Sha256};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;
/// Upper bound on model -> tool -> model round trips for a single user turn.
pub const MAX_TOOL_ROUNDS: usize = 5;
pub const STOP_MARKER: &str = "[stopped: reached 5 tool rounds]";
fn hash_prompt(prompt: &str) -> String {
    Sha256::digest(prompt.as_bytes())
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}
/// Millisecond timestamps are not unique inside one tool round, so a process
/// wide counter keeps ids distinct.
fn new_id(prefix: &str) -> String {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    format!(
        "{prefix}-{}-{}-{}",
        Utc::now().timestamp_millis(),
        std::process::id(),
        COUNTER.fetch_add(1, Ordering::Relaxed)
    )
}
fn demo_reply(model_name: &str, provider_name: &str) -> String {
    format!(
        "[Demo mode] Merlink is ready with {model_name} via {provider_name}. \
         Add an API key in ~/.merlink/config.json to send real requests."
    )
}
pub async fn send_chat_internal(
    provider_id: String,
    model_id: String,
    messages: Vec<MessageEnvelope>,
    project_id: String,
    session_id: String,
    channel: tauri::ipc::Channel<AgentEvent>,
) -> Result<ChatTurn, String> {
    let providers = store::load_providers()?;
    let sink = ChannelSink(channel);
    run_agent_turn(
        &HttpBackend,
        &providers,
        &provider_id,
        &model_id,
        &messages,
        &project_id,
        &session_id,
        &sink,
    )
    .await
}

/// Runs one user turn: model -> tools -> model, until the model stops asking for
/// tools or [`MAX_TOOL_ROUNDS`] is reached.
///
/// Returns only the messages produced by this turn, plus the project the session
/// belongs to when the turn ends (D4) — the first message of an empty chat can
/// create that project.
///
/// `sink` receives the live [`AgentEvent`] stream (D4/D5): every step is emitted
/// *as it happens*, before `run_agent_turn` returns, so the UI can render phases
/// while it waits.
#[allow(clippy::too_many_arguments)]
pub async fn run_agent_turn(
    backend: &dyn LlmBackend,
    providers: &[ProviderConfig],
    provider_id: &str,
    model_id: &str,
    messages: &[MessageEnvelope],
    project_id: &str,
    session_id: &str,
    sink: &dyn ProgressSink,
) -> Result<ChatTurn, String> {
    let turn_started = Instant::now();
    let provider = providers
        .iter()
        .find(|item| item.id == provider_id)
        .ok_or("Provider not found")?;
    let model = provider
        .models
        .iter()
        .find(|item| item.id == model_id)
        .ok_or("Model does not belong to provider")?;
    if !messages.iter().any(|message| message.role == "user") {
        return Err("A user message is required".into());
    }

    let session_id = if session_id.trim().is_empty() {
        derive_session_id(messages)
    } else {
        session_id.to_owned()
    };

    // D4: first event of the turn, emitted before any work happens.
    sink.emit(AgentEvent::TurnStarted {
        session_id: session_id.clone(),
        round_budget: MAX_TOOL_ROUNDS as u32,
    });

    // Persisting the incoming history is idempotent: message ids are stable and
    // the insert is an upsert.
    let connection = db::open()?;
    // D1: the session owns the binding. `ensure_session` accepts an empty project
    // id (a chat with no project yet) and never overwrites an existing binding,
    // so a later turn sending "" cannot unbind a session.
    db::ensure_session(&connection, &session_id, project_id.trim())?;
    let mut effective_project_id = effective_project(&connection, &session_id)?;
    // D1: a chat with no project gets no project invented for it — and therefore
    // no super json either (D2).
    if !effective_project_id.is_empty() {
        db::ensure_project(&connection, &effective_project_id, &effective_project_id)?;
    }
    for message in messages {
        db::append_message(&connection, &session_id, message)?;
    }

    let system = assemble_prompt(
        &provider.name,
        &model.name,
        &session_id,
        &effective_project_id,
    );
    let prompt_hash = hash_prompt(&system);
    let mut appended: Vec<MessageEnvelope> = Vec::new();

    // No API key: keep the long-standing demo behaviour (plain text, no tools).
    if provider.api_key.trim().is_empty() {
        let mut reply = MessageEnvelope::new(
            new_id("assistant"),
            "assistant",
            demo_reply(&model.name, &provider.name),
        );
        reply.meta = Some(build_meta(
            provider,
            &model.name,
            &session_id,
            &effective_project_id,
            &prompt_hash,
            TokenUsage::default(),
            elapsed_ms(turn_started) as u128,
        ));
        db::append_message(&connection, &session_id, &reply)?;
        appended.push(reply);
        // D4: even the demo path closes its turn with `TurnFinished`.
        let project_id = effective_project(&connection, &session_id)?;
        sink.emit(AgentEvent::TurnFinished {
            project_id: project_id.clone(),
            duration_ms: elapsed_ms(turn_started),
            rounds: 0,
        });
        return Ok(ChatTurn {
            project_id,
            messages: appended,
        });
    }

    let tool_specs = tools::tool_specs();
    let mut conversation: Vec<MessageEnvelope> = messages.to_vec();
    let mut last_tokens = TokenUsage::default();
    // D9: reasoning is accumulated across every round and attached only to the
    // final message of the turn (the one carrying `meta`).
    let mut turn_reasoning = String::new();

    for round_index in 0..MAX_TOOL_ROUNDS {
        // 1-based for the UI: "Đang suy nghĩ (vòng 1)".
        let round = (round_index + 1) as u32;
        sink.emit(AgentEvent::RoundStarted { round });
        let round_started = Instant::now();
        // D5: recomputed per round, so a project created by the previous round is
        // already part of the system prompt the model reads next.
        effective_project_id = effective_project(&connection, &session_id)?;
        let request = ChatRequest {
            kind: provider.kind.clone(),
            base_url: provider.base_url.clone(),
            api_key: provider.api_key.clone(),
            model: model.id.clone(),
            max_tokens: model.max_tokens.clone(),
            system: assemble_prompt(
                &provider.name,
                &model.name,
                &session_id,
                &effective_project_id,
            ),
            messages: conversation.clone(),
            tools: tool_specs.clone(),
        };
        let reply = match backend.complete(request).await {
            Ok(reply) => reply,
            Err(error) => {
                // A dead backend still closes the turn so the UI clears its phases.
                sink.emit(AgentEvent::TurnFinished {
                    project_id: effective_project_id.clone(),
                    duration_ms: elapsed_ms(turn_started),
                    rounds: round,
                });
                return Err(error);
            }
        };
        last_tokens = reply.tokens.clone();
        // D9: every round's thinking belongs to the same turn, so concatenate.
        if let Some(reasoning) = reply
            .reasoning
            .as_deref()
            .map(str::trim)
            .filter(|reasoning| !reasoning.is_empty())
        {
            if !turn_reasoning.is_empty() {
                turn_reasoning.push_str("\n\n");
            }
            turn_reasoning.push_str(reasoning);
        }

        let mut assistant =
            MessageEnvelope::new(new_id("assistant"), "assistant", reply.content.clone());
        if !reply.tool_calls.is_empty() {
            assistant.tool_calls = Some(reply.tool_calls.clone());
        }
        db::append_message(&connection, &session_id, &assistant)?;
        conversation.push(assistant.clone());
        appended.push(assistant);

        if reply.tool_calls.is_empty() {
            // D5: the round is over before the answer is finalised.
            sink.emit(AgentEvent::RoundFinished {
                round,
                duration_ms: elapsed_ms(round_started),
                tool_calls: 0,
            });
            // Final answer: attach the run metadata the UI renders.
            if let Some(message) = appended.last_mut() {
                message.meta = Some(build_meta(
                    provider,
                    &model.name,
                    &session_id,
                    &effective_project_id,
                    &prompt_hash,
                    reply.tokens,
                    elapsed_ms(turn_started) as u128,
                ));
                // D9: the final message is the only one that carries reasoning.
                message.reasoning = (!turn_reasoning.is_empty()).then(|| turn_reasoning.clone());
                db::append_message(&connection, &session_id, message)?;
            }
            let project_id = effective_project(&connection, &session_id)?;
            sink.emit(AgentEvent::TurnFinished {
                project_id: project_id.clone(),
                duration_ms: elapsed_ms(turn_started),
                rounds: round,
            });
            return Ok(ChatTurn {
                project_id,
                messages: appended,
            });
        }

        for call in &reply.tool_calls {
            // D5: recomputed *before every tool call*, not once per turn — without
            // this, the project `project_create` made in round 1 would not be
            // visible to the `super_json_set` calls of round 2.
            effective_project_id = effective_project(&connection, &session_id)?;
            // D3/D4: one Vietnamese status per call, shared by the start and finish
            // events so the UI line keeps its wording.
            let status = tools::status_message(&call.name, &call.arguments);
            let path = tools::tool_path(&call.name, &call.arguments);
            sink.emit(AgentEvent::ToolStarted {
                id: call.id.clone(),
                name: call.name.clone(),
                path,
                status: status.clone(),
            });
            let tool_started = Instant::now();
            let outcome = tools::execute_tool(
                &call.name,
                &call.arguments,
                &effective_project_id,
                &session_id,
            );
            let duration_ms = elapsed_ms(tool_started);
            let (result, ok) = match outcome {
                Ok(value) => (value, true),
                // A failing tool must not kill the turn: the model gets the error
                // back and can correct itself.
                Err(message) => (json!({ "error": message }), false),
            };
            // D6: the long-standing stderr log now carries the measured duration.
            eprintln!(
                "[merlink] tool {} ok={} ms={} args={} result={}",
                call.name, ok, duration_ms, call.arguments, result
            );
            let tool_message =
                MessageEnvelope::tool_result(new_id("tool"), &call.id, &call.name, &result);
            // A finished tool is truthful only after its result is durable.
            if let Err(error) = db::append_message(&connection, &session_id, &tool_message) {
                sink.emit(AgentEvent::ToolFinished {
                    id: call.id.clone(),
                    name: call.name.clone(),
                    ok: false,
                    duration_ms,
                    status,
                });
                return Err(format!(
                    "Tool '{}' ran but its result could not be stored: {error}",
                    call.name
                ));
            }
            conversation.push(tool_message.clone());
            appended.push(tool_message);
            sink.emit(AgentEvent::ToolFinished {
                id: call.id.clone(),
                name: call.name.clone(),
                ok,
                duration_ms,
                status,
            });
        }

        sink.emit(AgentEvent::RoundFinished {
            round,
            duration_ms: elapsed_ms(round_started),
            tool_calls: reply.tool_calls.len() as u32,
        });
    }

    let project_id = effective_project(&connection, &session_id)?;
    let mut stop_message = MessageEnvelope::new(new_id("assistant"), "assistant", STOP_MARKER);
    stop_message.meta = Some(build_meta(
        provider,
        &model.name,
        &session_id,
        &project_id,
        &prompt_hash,
        last_tokens,
        elapsed_ms(turn_started) as u128,
    ));
    // D9: the round-cap path carries the accumulated reasoning too.
    stop_message.reasoning = (!turn_reasoning.is_empty()).then(|| turn_reasoning.clone());
    db::append_message(&connection, &session_id, &stop_message)?;
    appended.push(stop_message);
    // D4: the cap path also closes with `TurnFinished`.
    sink.emit(AgentEvent::TurnFinished {
        project_id: project_id.clone(),
        duration_ms: elapsed_ms(turn_started),
        rounds: MAX_TOOL_ROUNDS as u32,
    });
    Ok(ChatTurn {
        project_id,
        messages: appended,
    })
}
#[cfg(test)]
mod tests;
