//! Writing persistence service orchestration (`REQ-CMD-029`..`031`, `REQ-CMD-035`..`037`).
//!
//! Orchestrates:
//! - Writing Mode chapter read model query (`get_chapter_writing_state`, IPC #29).
//! - User-exclusive scene lock toggle (`set_scene_lock`, IPC #30, `TX-06`).
//! - User-exclusive chapter bulk lock toggle (`set_chapter_scenes_lock_bulk`, IPC #31, `TX-06`).
//! - Scene version history listing (`list_scene_versions`, IPC #35).
//! - Atomic chapter save with OCC and 5-property idempotency (`save_chapter`, IPC #36, `TX-07`).
//! - Idempotency receipt retrieval (`get_chapter_save_receipt`, IPC #37).

use crate::error::{ErrorCode, IpcError};
use crate::ipc::dto::{
    BulkSceneLockStateDto, ChapterWritingReadModelDto, GetChapterSaveReceiptReq,
    GetChapterWritingStateReq, ListSceneVersionsReq, SaveChapterRequest, SaveChapterResponse,
    SaveReceiptLookupDto, SceneLockStateDto, SceneVersionListDto, SetChapterScenesLockBulkReq,
    SetSceneLockReq,
};
use crate::storage::db::DbPool;
use crate::storage::writing_repo;

/// Caller classification for writing operations (`LOCK-01`, `LOCK-02`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CallerContext {
    /// Human user action via frontend UI (`"ui_action"`).
    UiAction,
    /// Automated AI caller / model turn (`"ai"`). Strictly forbidden for locking operations.
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

/// Orchestration service for Writing Mode persistence and read model queries.
pub struct WritingPersistenceService {
    pool: DbPool,
}

impl WritingPersistenceService {
    /// Constructs a new `WritingPersistenceService` with the given database pool.
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    /// Accessor for the underlying database pool.
    pub fn pool(&self) -> &DbPool {
        &self.pool
    }

    /// IPC #29: `get_chapter_writing_state`.
    ///
    /// Verifies Committed-Plan Gate (`WR-01`) and returns full chapter read model.
    pub fn get_chapter_writing_state(
        &self,
        req: &GetChapterWritingStateReq,
    ) -> Result<ChapterWritingReadModelDto, IpcError> {
        let conn = self
            .pool
            .get()
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
        writing_repo::get_chapter_writing_state(&conn, req.project_id, req.chapter_id)
    }

    /// IPC #30: `set_scene_lock`.
    ///
    /// Gated by `CallerContext::UiAction` (`LOCK-01`, `LOCK-02`). Executes `TX-06` in `BEGIN IMMEDIATE`.
    pub fn set_scene_lock(
        &self,
        req: &SetSceneLockReq,
        caller: CallerContext,
    ) -> Result<SceneLockStateDto, IpcError> {
        caller.require_ui_action("set_scene_lock")?;
        let mut conn = self
            .pool
            .get()
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
        writing_repo::set_scene_lock_tx(
            &mut conn,
            req.project_id,
            req.chapter_id,
            req.scene_id,
            req.desired_locked,
        )
    }

    /// IPC #31: `set_chapter_scenes_lock_bulk`.
    ///
    /// Gated by `CallerContext::UiAction` (`LOCK-01`). Executes `TX-06` bulk update in `BEGIN IMMEDIATE`.
    pub fn set_chapter_scenes_lock_bulk(
        &self,
        req: &SetChapterScenesLockBulkReq,
        caller: CallerContext,
    ) -> Result<BulkSceneLockStateDto, IpcError> {
        caller.require_ui_action("set_chapter_scenes_lock_bulk")?;
        let mut conn = self
            .pool
            .get()
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
        writing_repo::set_chapter_scenes_lock_bulk_tx(
            &mut conn,
            req.project_id,
            req.chapter_id,
            req.is_locked,
        )
    }

    /// IPC #35: `list_scene_versions`.
    ///
    /// Read-only historical version list (`CI-08`, `SF-07`).
    pub fn list_scene_versions(
        &self,
        req: &ListSceneVersionsReq,
    ) -> Result<SceneVersionListDto, IpcError> {
        let conn = self
            .pool
            .get()
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
        writing_repo::list_scene_versions(&conn, req.project_id, req.chapter_id, req.scene_id)
    }

    /// IPC #36: `save_chapter`.
    ///
    /// Executes `TX-07` in `BEGIN IMMEDIATE` with OCC and 5-property idempotency.
    pub fn save_chapter(&self, req: &SaveChapterRequest) -> Result<SaveChapterResponse, IpcError> {
        let mut conn = self
            .pool
            .get()
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
        writing_repo::save_chapter_tx(&mut conn, req)
    }

    /// IPC #37: `get_chapter_save_receipt`.
    ///
    /// Read-only receipt lookup for ambiguous result recovery (`Q4.7`).
    pub fn get_chapter_save_receipt(
        &self,
        req: &GetChapterSaveReceiptReq,
    ) -> Result<SaveReceiptLookupDto, IpcError> {
        let conn = self
            .pool
            .get()
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
        writing_repo::get_chapter_save_receipt(&conn, req.chapter_id, &req.client_save_id)
    }
}
