use crate::{models, runtime};

#[tauri::command]
pub(crate) async fn send_chat(
    provider_id: String,
    model_id: String,
    messages: Vec<models::MessageEnvelope>,
    project_id: String,
    session_id: String,
    channel: tauri::ipc::Channel<models::AgentEvent>,
) -> Result<models::ChatTurn, String> {
    runtime::send_chat_internal(
        provider_id,
        model_id,
        messages,
        project_id,
        session_id,
        channel,
    )
    .await
}

#[tauri::command]
pub(crate) fn context_usage(
    session_id: String,
    provider_id: String,
    model_id: String,
) -> Result<models::ContextUsage, String> {
    runtime::context_usage_for_session(&session_id, &provider_id, &model_id)
}

#[tauri::command]
pub(crate) fn list_messages(session_id: String) -> Result<Vec<models::MessageEnvelope>, String> {
    runtime::session_messages(&session_id)
}
