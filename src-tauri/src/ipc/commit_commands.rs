//! Draft Commit & Reopening IPC Commands (#19, #20, #21)
//! (`REQ-CMD-019`..`021`, `be-core.md` §10.2.6, `CI-04`).

use std::sync::Arc;

use crate::error::IpcError;
use crate::ipc::dto::{
    CommitDraftReq, CommitDraftResponseDto, CommitPreviewDto, GetCommitPreviewReq,
    ReopenCommittedDraftReq, ReopenCommittedDraftResponseDto,
};
pub use crate::service::commit_service::CallerContext;
use crate::service::commit_service::CommitService;

// ============================================================================
// IPC #19: get_commit_confirmation_preview
// ============================================================================

/// Direct execution of IPC #19 `get_commit_confirmation_preview`.
pub fn exec_get_commit_confirmation_preview(
    service: &CommitService,
    req: &GetCommitPreviewReq,
) -> Result<CommitPreviewDto, IpcError> {
    service.get_commit_confirmation_preview(&req.draft_session_id)
}

#[tauri::command]
pub async fn get_commit_confirmation_preview(
    service: tauri::State<'_, Arc<CommitService>>,
    req: GetCommitPreviewReq,
) -> Result<CommitPreviewDto, IpcError> {
    exec_get_commit_confirmation_preview(&service, &req)
}

// ============================================================================
// IPC #20: commit_draft
// ============================================================================

/// Direct execution of IPC #20 `commit_draft` with caller context check.
pub async fn exec_commit_draft(
    service: &CommitService,
    req: &CommitDraftReq,
    caller: CallerContext,
) -> Result<CommitDraftResponseDto, IpcError> {
    service.commit_draft(&req.draft_session_id, caller).await
}

#[tauri::command]
pub async fn commit_draft(
    service: tauri::State<'_, Arc<CommitService>>,
    req: CommitDraftReq,
) -> Result<CommitDraftResponseDto, IpcError> {
    exec_commit_draft(&service, &req, CallerContext::UiAction).await
}

// ============================================================================
// IPC #21: reopen_committed_draft
// ============================================================================

/// Direct execution of IPC #21 `reopen_committed_draft`.
pub async fn exec_reopen_committed_draft(
    service: &CommitService,
    req: &ReopenCommittedDraftReq,
) -> Result<ReopenCommittedDraftResponseDto, IpcError> {
    service
        .reopen_committed_draft(&req.project_id, &req.active_session_id)
        .await
}

#[tauri::command]
pub async fn reopen_committed_draft(
    service: tauri::State<'_, Arc<CommitService>>,
    req: ReopenCommittedDraftReq,
) -> Result<ReopenCommittedDraftResponseDto, IpcError> {
    exec_reopen_committed_draft(&service, &req).await
}
