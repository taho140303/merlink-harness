//! Writing generation IPC commands (#32, #33, #34)
//! (`REQ-CMD-032`..`034`, `be-core.md` §10.2.9, §10.3.3).

use std::sync::Arc;
use tokio::sync::mpsc;

use crate::error::IpcError;
use crate::ipc::dto::{
    CancelSceneGenerationReq, CancelSceneGenerationResponseDto,
    RespondEditScopeExpansionReq, RespondEditScopeExpansionResponseDto,
    SceneGenerationResultDto, SceneGenerationStreamEvent, StartSceneGenerationReq,
};
use crate::service::writing_generation::WritingGenerationService;

// ============================================================================
// IPC #32: start_scene_generation
// ============================================================================

/// Direct execution of IPC #32 `start_scene_generation`.
pub async fn exec_start_scene_generation(
    service: &WritingGenerationService,
    req: &StartSceneGenerationReq,
    event_tx: Option<mpsc::Sender<SceneGenerationStreamEvent>>,
) -> Result<SceneGenerationResultDto, IpcError> {
    service.start_scene_generation(req, event_tx).await
}

#[tauri::command]
pub async fn start_scene_generation(
    service: tauri::State<'_, Arc<WritingGenerationService>>,
    config: tauri::State<'_, Arc<crate::ipc::config_commands::RuntimeConfigState>>,
    req: StartSceneGenerationReq,
    on_event: tauri::ipc::Channel<SceneGenerationStreamEvent>,
) -> Result<SceneGenerationResultDto, IpcError> {
    let provider = config.selected_provider()?;
    let (tx, mut rx) = mpsc::channel(100);
    let channel_clone = on_event.clone();
    tokio::spawn(async move {
        while let Some(evt) = rx.recv().await {
            let _ = channel_clone.send(evt);
        }
    });
    service.start_scene_generation_with_provider(&req, provider, Some(tx)).await
}

// ============================================================================
// IPC #33: cancel_scene_generation
// ============================================================================

/// Direct execution of IPC #33 `cancel_scene_generation`.
pub async fn exec_cancel_scene_generation(
    service: &WritingGenerationService,
    req: &CancelSceneGenerationReq,
) -> Result<CancelSceneGenerationResponseDto, IpcError> {
    service.cancel_scene_generation(req).await
}

#[tauri::command]
pub async fn cancel_scene_generation(
    service: tauri::State<'_, Arc<WritingGenerationService>>,
    req: CancelSceneGenerationReq,
) -> Result<CancelSceneGenerationResponseDto, IpcError> {
    exec_cancel_scene_generation(&service, &req).await
}

// ============================================================================
// IPC #34: respond_edit_scope_expansion
// ============================================================================

/// Direct execution of IPC #34 `respond_edit_scope_expansion`.
pub async fn exec_respond_edit_scope_expansion(
    service: &WritingGenerationService,
    req: &RespondEditScopeExpansionReq,
) -> Result<RespondEditScopeExpansionResponseDto, IpcError> {
    service.respond_edit_scope_expansion(req).await
}

#[tauri::command]
pub async fn respond_edit_scope_expansion(
    service: tauri::State<'_, Arc<WritingGenerationService>>,
    req: RespondEditScopeExpansionReq,
) -> Result<RespondEditScopeExpansionResponseDto, IpcError> {
    exec_respond_edit_scope_expansion(&service, &req).await
}
