//! Pending Patches IPC Commands (#12, #13, #14, #15)
//! (`REQ-CMD-012`..`015`, `REQ-TX-002`, `be-core.md` §10.2.4, `harness/tools.md` §3.3).

use std::sync::Arc;

use crate::error::{ErrorCode, IpcError};
use crate::ipc::dto::{
    ConfirmAllPatchesResponseDto, ConfirmAllPendingPatchesReq, ConfirmPendingPatchReq,
    ListPendingPatchesReq, PatchResolutionDto, PendingPatchListDto, RejectPendingPatchReq,
};
use crate::service::patch_service::PatchService;

/// Caller category invoking IPC patch commands (`be-core.md` §10.2.4, `harness/tools.md` §3.3).
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CallerContext {
    /// Human user action via frontend UI (`"ui_action"`).
    UiAction,
    /// Automated AI caller / model turn (`"ai"`). Strictly forbidden for patch resolution.
    Ai,
}

impl CallerContext {
    pub const fn is_ai(self) -> bool {
        matches!(self, CallerContext::Ai)
    }

    pub fn require_ui_action(self, cmd_name: &str) -> Result<(), IpcError> {
        if self.is_ai() {
            Err(IpcError::new(
                ErrorCode::UnauthorizedCaller,
                format!(
                    "Command '{}' is restricted to UI/human operator (CallerContext::UiAction required)",
                    cmd_name
                ),
            ))
        } else {
            Ok(())
        }
    }
}

// ============================================================================
// IPC #12: list_pending_patches
// ============================================================================

/// Direct execution of IPC #12 `list_pending_patches`.
///
/// Invariants (`be-core.md` §10.2.4):
/// - Evaluates each pending patch against the current working state (lazy expiration check).
/// - If expired, updates `status = 'expired'` in SQLite and returns updated preview.
/// - Errors: `ErrorCode::EntityNotFound`, `ErrorCode::DatabaseError`.
pub fn exec_list_pending_patches(
    service: &PatchService,
    req: &ListPendingPatchesReq,
) -> Result<PendingPatchListDto, IpcError> {
    service.list_pending_patches(&req.draft_session_id)
}

#[tauri::command]
pub async fn list_pending_patches(
    service: tauri::State<'_, Arc<PatchService>>,
    req: ListPendingPatchesReq,
) -> Result<PendingPatchListDto, IpcError> {
    exec_list_pending_patches(&service, &req)
}

// ============================================================================
// IPC #13: confirm_pending_patch
// ============================================================================

/// Direct execution of IPC #13 `confirm_pending_patch`.
///
/// Invariants (`be-core.md` §10.2.4, IPC #13):
/// - Gated by `CallerContext::UiAction` (returns `ErrorCode::UnauthorizedCaller` if invoked by AI).
/// - Runs under `operation_lock` and `TX-02` (`BEGIN IMMEDIATE`).
/// - CAS on `patch.status`: idempotent `already_confirmed` if already confirmed.
/// - If valid: applies mutation to `WorkingState` (`source = 'user_edited'`), marks `confirmed`,
///   and seals exactly 1 version `ai_patch` (`seal_key = "patch_confirm:{patchId}"`).
pub async fn exec_confirm_pending_patch(
    service: &PatchService,
    req: &ConfirmPendingPatchReq,
    caller: CallerContext,
) -> Result<PatchResolutionDto, IpcError> {
    caller.require_ui_action("confirm_pending_patch")?;
    service
        .confirm_pending_patch(req.draft_session_id, req.patch_id)
        .await
}

#[tauri::command]
pub async fn confirm_pending_patch(
    service: tauri::State<'_, Arc<PatchService>>,
    req: ConfirmPendingPatchReq,
) -> Result<PatchResolutionDto, IpcError> {
    exec_confirm_pending_patch(&service, &req, CallerContext::UiAction).await
}

// ============================================================================
// IPC #14: reject_pending_patch
// ============================================================================

/// Direct execution of IPC #14 `reject_pending_patch`.
///
/// Invariants (`be-core.md` §10.2.4, IPC #14):
/// - Gated by `CallerContext::UiAction` (returns `ErrorCode::UnauthorizedCaller` if invoked by AI).
/// - Runs under `operation_lock` and `TX-02` (`BEGIN IMMEDIATE`).
/// - Sets `status = 'rejected'`, `resolved_at = now()`.
/// - Does NOT mutate WorkingState and does NOT create `draft_versions`.
pub async fn exec_reject_pending_patch(
    service: &PatchService,
    req: &RejectPendingPatchReq,
    caller: CallerContext,
) -> Result<PatchResolutionDto, IpcError> {
    caller.require_ui_action("reject_pending_patch")?;
    service
        .reject_pending_patch(req.draft_session_id, req.patch_id)
        .await
}

#[tauri::command]
pub async fn reject_pending_patch(
    service: tauri::State<'_, Arc<PatchService>>,
    req: RejectPendingPatchReq,
) -> Result<PatchResolutionDto, IpcError> {
    exec_reject_pending_patch(&service, &req, CallerContext::UiAction).await
}

// ============================================================================
// IPC #15: confirm_all_pending_patches
// ============================================================================

/// Direct execution of IPC #15 `confirm_all_pending_patches`.
///
/// Invariants (`be-core.md` §10.2.4, IPC #15):
/// - Gated by `CallerContext::UiAction` (returns `ErrorCode::UnauthorizedCaller` if invoked by AI).
/// - Runs under `operation_lock` and `TX-02` (`BEGIN IMMEDIATE`).
/// - Evaluates pending patches sequentially ordered by `created_at ASC, id ASC` on cumulative candidate state.
/// - Invalid/expired patches moved to `skipped` without rolling back the whole batch.
/// - Valid patches applied to candidate with `source = 'user_edited'`.
/// - Seals exactly 1 draft version `ai_patch` (`seal_key = "patch_batch:{clientRequestId}"`) if `applied.len() > 0`.
/// - Idempotent by `client_request_id`.
pub async fn exec_confirm_all_pending_patches(
    service: &PatchService,
    req: &ConfirmAllPendingPatchesReq,
    caller: CallerContext,
) -> Result<ConfirmAllPatchesResponseDto, IpcError> {
    caller.require_ui_action("confirm_all_pending_patches")?;
    service
        .confirm_all_pending_patches(req.draft_session_id, req.client_request_id.clone())
        .await
}

#[tauri::command]
pub async fn confirm_all_pending_patches(
    service: tauri::State<'_, Arc<PatchService>>,
    req: ConfirmAllPendingPatchesReq,
) -> Result<ConfirmAllPatchesResponseDto, IpcError> {
    exec_confirm_all_pending_patches(&service, &req, CallerContext::UiAction).await
}
