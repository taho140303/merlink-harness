//! Session-scoped queries: the bound project, the tool execution guard, super
//! json reads, persisted messages, context usage, and progress.

use chrono::Utc;

use crate::{
    adapters, db,
    models::{
        ContextUsage, MessageEnvelope, MessageMeta, ProviderConfig, SuperJsonProgress, TokenUsage,
    },
    store, tools,
};

use super::prompt::assemble_prompt;

/// The project a session currently belongs to; empty means "not bound yet" (D1).
pub(super) fn effective_project(
    connection: &rusqlite::Connection,
    session_id: &str,
) -> Result<String, String> {
    Ok(db::project_for_session(connection, session_id)?.unwrap_or_default())
}

pub(super) fn derive_session_id(messages: &[MessageEnvelope]) -> String {
    messages
        .first()
        .map(|message| {
            message
                .meta
                .as_ref()
                .map(|meta| meta.session_id.as_str())
                .filter(|id| !id.is_empty())
                .unwrap_or(message.id.as_str())
        })
        .unwrap_or("session-default")
        .to_owned()
}

#[allow(clippy::too_many_arguments)]
pub(super) fn build_meta(
    provider: &ProviderConfig,
    model_name: &str,
    session_id: &str,
    project_id: &str,
    prompt_hash: &str,
    tokens: TokenUsage,
    latency_ms: u128,
) -> MessageMeta {
    MessageMeta {
        model: model_name.to_owned(),
        provider: provider.name.clone(),
        session_id: session_id.to_owned(),
        project_id: project_id.to_owned(),
        prompt_template_id: "harness_system.v1".into(),
        prompt_hash: prompt_hash.to_owned(),
        tokens,
        latency_ms: latency_ms.min(u128::from(u64::MAX)) as u64,
        created_at: Utc::now().to_rfc3339(),
    }
}

/// The public tool command must never accept a caller-selected project for a
/// session-bound super JSON operation. The agent loop already resolves its own
/// project from the session before calling `tools::execute_tool` directly.
pub fn execute_tool_internal(
    tool_name: &str,
    args: serde_json::Value,
    project_id: &str,
    session_id: &str,
) -> Result<serde_json::Value, String> {
    let bound_project = if matches!(
        tool_name,
        "super_json_get" | "super_json_set" | "super_json_missing"
    ) {
        let connection = db::open()?;
        let bound = db::project_for_session(&connection, session_id)?
            .filter(|id| !id.is_empty())
            .ok_or("This chat is not bound to a project. Call project_create first.")?;
        if !project_id.is_empty() && project_id != bound {
            return Err("Project does not belong to this session".into());
        }
        bound
    } else {
        project_id.to_owned()
    };
    tools::execute_tool(tool_name, &args, &bound_project, session_id)
}

/// Reads may initialise a missing document only for a project that exists in
/// SQLite; a typo must not leave a ghost directory or plausible empty data.
pub fn super_json_document(project_id: &str) -> Result<serde_json::Value, String> {
    if project_id.trim().is_empty() {
        return Err("Project ID must not be empty".into());
    }
    let connection = db::open()?;
    if !db::project_exists(&connection, project_id)? {
        return Err(format!("Project not found: {project_id}"));
    }
    crate::super_json::load_or_init(project_id)
}

/// The `Chi tiết` dialog's prose summary (Sprint 11.11 · D2): a template fill
/// over the project's super json, generated here and never by a model.
pub fn super_json_summary_for_project(project_id: &str) -> Result<String, String> {
    let document = super_json_document(project_id)?;
    Ok(crate::super_json::summary(&document))
}

pub fn session_messages(session_id: &str) -> Result<Vec<MessageEnvelope>, String> {
    let connection = db::open()?;
    db::list_messages(&connection, session_id)
}

/// Estimated context usage for the selected model (Sprint 11.10).
pub fn context_usage_for_session(
    session_id: &str,
    provider_id: &str,
    model_id: &str,
) -> Result<crate::models::ContextUsage, String> {
    let providers = store::load_providers()?;
    let Some(provider) = providers.iter().find(|p| p.id == provider_id) else {
        return Ok(ContextUsage::default());
    };
    let Some(model) = provider.models.iter().find(|m| m.id == model_id) else {
        return Ok(ContextUsage::default());
    };
    let connection = db::open()?;
    let project_id = effective_project(&connection, session_id)?;
    let prompt = assemble_prompt(&provider.name, &model.name, session_id, &project_id);
    let tools_json = serde_json::to_string(&tools::tool_specs()).map_err(|e| e.to_string())?;
    let messages = db::list_messages(&connection, session_id)?;
    let chars = |s: &str| -> u64 { (s.chars().count() as u64).div_ceil(4) };
    let system_tokens = chars(&prompt);
    let tool_tokens = chars(&tools_json);
    let message_tokens = messages
        .iter()
        .map(|m| {
            chars(&m.content)
                + m.tool_calls
                    .as_ref()
                    .map(|calls| {
                        calls
                            .iter()
                            .map(|call| chars(&serde_json::to_string(call).unwrap_or_default()))
                            .sum::<u64>()
                    })
                    .unwrap_or(0)
        })
        .sum::<u64>();
    let used_tokens = system_tokens + tool_tokens + message_tokens;
    let total_tokens = adapters::parse_context_window(&model.context_window);
    let percent =
        ((100.0 * used_tokens as f64 / total_tokens as f64).round() as u64).min(100) as u8;
    let measured_input_tokens = messages
        .iter()
        .rev()
        .find(|m| m.role == "assistant")
        .and_then(|m| m.meta.as_ref())
        .map(|meta| meta.tokens.input_tokens);
    Ok(ContextUsage {
        has_model: true,
        total_tokens,
        used_tokens,
        percent,
        system_tokens,
        tool_tokens,
        message_tokens,
        measured_input_tokens,
    })
}

/// Progress for the header bar (D7). The session's project is resolved here, so
/// the frontend never has to guess which project the numbers belong to.
pub fn super_json_progress_for_session(session_id: &str) -> Result<SuperJsonProgress, String> {
    let connection = db::open()?;
    let project_id = effective_project(&connection, session_id)?;
    crate::super_json::progress(&project_id)
}

#[cfg(test)]
mod tests;
