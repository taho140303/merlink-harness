//! Writing persistence IPC commands (#29, #30, #31, #35, #36, #37)
//! (`REQ-CMD-029`..`031`, `REQ-CMD-035`..`037`, `be-core.md` §10.2.9).

use std::sync::Arc;

use crate::error::IpcError;
use crate::ipc::dto::{
    BulkSceneLockStateDto, ChapterWritingReadModelDto, GetChapterSaveReceiptReq,
    GetChapterWritingStateReq, ListSceneVersionsReq, SaveChapterRequest, SaveChapterResponse,
    SaveReceiptLookupDto, SceneLockStateDto, SceneVersionListDto, SetChapterScenesLockBulkReq,
    SetSceneLockReq,
};
pub use crate::service::writing_persistence::CallerContext;
use crate::service::writing_persistence::WritingPersistenceService;

// ============================================================================
// IPC #29: get_chapter_writing_state
// ============================================================================

/// Direct execution of IPC #29 `get_chapter_writing_state`.
pub fn exec_get_chapter_writing_state(
    service: &WritingPersistenceService,
    req: &GetChapterWritingStateReq,
) -> Result<ChapterWritingReadModelDto, IpcError> {
    service.get_chapter_writing_state(req)
}

#[tauri::command]
pub async fn get_chapter_writing_state(
    service: tauri::State<'_, Arc<WritingPersistenceService>>,
    req: GetChapterWritingStateReq,
) -> Result<ChapterWritingReadModelDto, IpcError> {
    exec_get_chapter_writing_state(&service, &req)
}

// ============================================================================
// IPC #30: set_scene_lock
// ============================================================================

/// Direct execution of IPC #30 `set_scene_lock` with caller context check.
pub fn exec_set_scene_lock(
    service: &WritingPersistenceService,
    req: &SetSceneLockReq,
    caller: CallerContext,
) -> Result<SceneLockStateDto, IpcError> {
    service.set_scene_lock(req, caller)
}

#[tauri::command]
pub async fn set_scene_lock(
    service: tauri::State<'_, Arc<WritingPersistenceService>>,
    req: SetSceneLockReq,
) -> Result<SceneLockStateDto, IpcError> {
    exec_set_scene_lock(&service, &req, CallerContext::UiAction)
}

// ============================================================================
// IPC #31: set_chapter_scenes_lock_bulk
// ============================================================================

/// Direct execution of IPC #31 `set_chapter_scenes_lock_bulk` with caller context check.
pub fn exec_set_chapter_scenes_lock_bulk(
    service: &WritingPersistenceService,
    req: &SetChapterScenesLockBulkReq,
    caller: CallerContext,
) -> Result<BulkSceneLockStateDto, IpcError> {
    service.set_chapter_scenes_lock_bulk(req, caller)
}

#[tauri::command]
pub async fn set_chapter_scenes_lock_bulk(
    service: tauri::State<'_, Arc<WritingPersistenceService>>,
    req: SetChapterScenesLockBulkReq,
) -> Result<BulkSceneLockStateDto, IpcError> {
    exec_set_chapter_scenes_lock_bulk(&service, &req, CallerContext::UiAction)
}

// ============================================================================
// IPC #35: list_scene_versions
// ============================================================================

/// Direct execution of IPC #35 `list_scene_versions`.
pub fn exec_list_scene_versions(
    service: &WritingPersistenceService,
    req: &ListSceneVersionsReq,
) -> Result<SceneVersionListDto, IpcError> {
    service.list_scene_versions(req)
}

#[tauri::command]
pub async fn list_scene_versions(
    service: tauri::State<'_, Arc<WritingPersistenceService>>,
    req: ListSceneVersionsReq,
) -> Result<SceneVersionListDto, IpcError> {
    exec_list_scene_versions(&service, &req)
}

// ============================================================================
// IPC #36: save_chapter
// ============================================================================

/// Direct execution of IPC #36 `save_chapter`.
pub fn exec_save_chapter(
    service: &WritingPersistenceService,
    req: &SaveChapterRequest,
) -> Result<SaveChapterResponse, IpcError> {
    service.save_chapter(req)
}

#[tauri::command]
pub async fn save_chapter(
    service: tauri::State<'_, Arc<WritingPersistenceService>>,
    req: SaveChapterRequest,
) -> Result<SaveChapterResponse, IpcError> {
    exec_save_chapter(&service, &req)
}

// ============================================================================
// IPC #37: get_chapter_save_receipt
// ============================================================================

/// Direct execution of IPC #37 `get_chapter_save_receipt`.
pub fn exec_get_chapter_save_receipt(
    service: &WritingPersistenceService,
    req: &GetChapterSaveReceiptReq,
) -> Result<SaveReceiptLookupDto, IpcError> {
    service.get_chapter_save_receipt(req)
}

#[tauri::command]
pub async fn get_chapter_save_receipt(
    service: tauri::State<'_, Arc<WritingPersistenceService>>,
    req: GetChapterSaveReceiptReq,
) -> Result<SaveReceiptLookupDto, IpcError> {
    exec_get_chapter_save_receipt(&service, &req)
}
