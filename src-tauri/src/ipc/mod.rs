//! Typed IPC commands, DTOs, and read-model adapters module root (REQ-ARCH-001, REQ-IPC-002, P6S2).

pub mod adapter;
pub mod character_commands;
pub mod chat_commands;
pub mod commit_commands;
pub mod config_commands;
pub mod draft_commands;
pub mod dto;
pub mod patch_commands;
pub mod project_commands;
pub mod provider_probe_commands;
pub mod system_prompt_commands;
pub mod workflow_commands;
pub mod writing_generation_commands;
pub mod writing_persistence_commands;

use std::sync::Arc;
use tokio::sync::mpsc;
use uuid::Uuid;

use crate::error::{ErrorCode, IpcError};
use crate::ipc::dto::{ChatStreamEvent, SendChatMessageReq, SendChatTurnResponseDto};
use crate::service::chat_orchestrator::ChatOrchestrator;

/// Master inventory of all 40 typed IPC commands in Merlink Harness (§10.2).
pub const MASTER_IPC_COMMAND_NAMES: &[&str] = &[
    // Group 1: Chat & Session (4)
    "create_or_load_chat_session",
    "list_chat_sessions",
    "send_chat_message",
    "cancel_chat_turn",
    // Group 2: Draft, Working State & Progress (5)
    "get_draft_workspace_state",
    "get_progress_detail",
    "apply_manual_draft_edit",
    "confirm_phase",
    "regenerate_draft_phase",
    // Group 3: Versioning & Restore (2)
    "list_draft_versions",
    "restore_draft_version",
    // Group 4: Pending Patches (4)
    "list_pending_patches",
    "confirm_pending_patch",
    "reject_pending_patch",
    "confirm_all_pending_patches",
    // Group 5: Segmented Workflow (3)
    "start_or_resume_workflow",
    "cancel_workflow",
    "get_workflow_status",
    // Group 6: Start / Commit Draft (3)
    "get_commit_confirmation_preview",
    "commit_draft",
    "reopen_committed_draft",
    // Group 7: Projects & Creator Mode (3)
    "list_projects",
    "get_project_detail",
    "delete_project",
    // Group 8: Character Library (4)
    "list_characters",
    "get_character_detail",
    "create_or_update_library_character",
    "link_library_character_to_draft",
    // Group 9: Writing Mode (9)
    "get_chapter_writing_state",
    "set_scene_lock",
    "set_chapter_scenes_lock_bulk",
    "start_scene_generation",
    "cancel_scene_generation",
    "respond_edit_scope_expansion",
    "list_scene_versions",
    "save_chapter",
    "get_chapter_save_receipt",
    // Group 10: Model Config & Credentials (3)
    "get_runtime_and_provider_config",
    "update_runtime_and_provider_config",
    "set_provider_api_key",
];

/// Typed streaming channel event payload names (§10.3).
pub const STREAMING_CHANNEL_NAMES: &[&str] = &[
    "ChatStreamEvent",
    "WorkflowProgressEvent",
    "SceneGenerationStreamEvent",
];

// Re-export command handlers
pub use character_commands::{
    create_or_update_library_character, get_character_detail, link_library_character_to_draft,
    list_characters,
};
pub use chat_commands::{
    cancel_chat_turn, create_or_load_chat_session, list_chat_sessions, regenerate_draft_phase,
};
pub use commit_commands::{commit_draft, get_commit_confirmation_preview, reopen_committed_draft};
pub use config_commands::{
    get_runtime_and_provider_config, set_provider_api_key, update_runtime_and_provider_config,
};
pub use draft_commands::{
    apply_manual_draft_edit, confirm_phase, get_draft_workspace_state, get_progress_detail,
    list_draft_versions, restore_draft_version,
};
pub use patch_commands::{
    confirm_all_pending_patches, confirm_pending_patch, list_pending_patches, reject_pending_patch,
};
pub use project_commands::{delete_project, get_project_detail, list_projects};
pub use workflow_commands::{cancel_workflow, get_workflow_status, start_or_resume_workflow};
pub use writing_generation_commands::{
    cancel_scene_generation, respond_edit_scope_expansion, start_scene_generation,
};
pub use writing_persistence_commands::{
    get_chapter_save_receipt, get_chapter_writing_state, list_scene_versions, save_chapter,
    set_chapter_scenes_lock_bulk, set_scene_lock,
};

/// IPC #3 `send_chat_message` with streaming channel support (§10.2.1, §10.3.1).
#[tauri::command]
pub async fn send_chat_message(
    orchestrator: tauri::State<'_, Arc<ChatOrchestrator>>,
    config: tauri::State<'_, Arc<crate::ipc::config_commands::RuntimeConfigState>>,
    req: SendChatMessageReq,
    on_event: tauri::ipc::Channel<ChatStreamEvent>,
) -> Result<SendChatTurnResponseDto, IpcError> {
    let provider = config.selected_provider()?;
    let (tx, mut rx) = mpsc::channel(100);
    let channel_clone = on_event.clone();
    tokio::spawn(async move {
        while let Some(evt) = rx.recv().await {
            let _ = channel_clone.send(evt);
        }
    });

    let (draft_session_id, turn_id, user_message_id) = {
        let conn = orchestrator
            .pool()
            .get()
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

        let mut stmt = conn
            .prepare("SELECT id FROM draft_sessions WHERE session_id = ?1")
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

        let draft_session_id_str: String = stmt
            .query_row(rusqlite::params![req.session_id], |row| row.get(0))
            .map_err(|_| {
                IpcError::new(
                    ErrorCode::EntityNotFound,
                    format!(
                        "Draft session for session_id '{}' not found",
                        req.session_id
                    ),
                )
            })?;

        let draft_session_id = Uuid::parse_str(&draft_session_id_str)
            .map_err(|e| IpcError::new(ErrorCode::InternalError, e.to_string()))?;

        let turn_id = Uuid::new_v4();
        let user_message_id = Uuid::new_v4();

        let payload = req
            .feedback_context
            .as_ref()
            .map(|f| serde_json::to_value(f).unwrap_or(serde_json::Value::Null));

        orchestrator.save_message(
            user_message_id,
            &req.session_id,
            turn_id,
            "user",
            &req.content,
            payload.as_ref(),
        )?;

        (draft_session_id, turn_id, user_message_id)
    };

    orchestrator
        .run_ai_turn(
            draft_session_id,
            &req.session_id,
            user_message_id,
            turn_id,
            &req.content,
            &req.system_language,
            provider,
            Some(tx),
            tokio_util::sync::CancellationToken::new(),
        )
        .await
}
