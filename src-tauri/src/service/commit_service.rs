//! Commit service orchestration (`TX-05`, IPC #19, #20, #21).
//!
//! Enforces:
//! - Pre-commit validation and preview generation (`REQ-CMD-019`).
//! - Atomic commit gated by `CallerContext::UiAction` with strict rollback on failure (`REQ-CMD-020`, `TX-05`).
//! - Idempotent returns when committing an already-committed draft.
//! - Written scenes gating on draft reopening (`REQ-CMD-021`, `DraftReopenForbidden`).

use chrono::Utc;
use rusqlite::params;
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::business_flow::BusinessFlowData;
use crate::domain::invariants::validate_draft_for_commit;
use crate::error::{ErrorCode, IpcError};
use crate::ipc::dto::{
    CommitDraftResponseDto, CommitPreviewDto, CommitValidationResultDto, PendingPatchDto,
    PendingPatchesSummaryDto, PlanSummaryDto, ReopenCommittedDraftResponseDto, ValidationIssueDto,
};
use crate::service::draft_runtime::{DraftRuntimeRegistry, DraftRuntimeStatus, WorkingState, DRAFT_LOCK_TIMEOUT};
use crate::storage::commit_repo::{commit_draft_tx, reopen_committed_draft, CommitRepoError};
use crate::storage::db::DbPool;

/// Caller classification for commit operations (`CI-04`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CallerContext {
    /// Human user action via frontend UI (`"ui_action"`).
    UiAction,
    /// Automated AI caller / model turn (`"ai"`). Strictly forbidden for committing draft.
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

/// Orchestration service for draft commit, preview, and reopening.
pub struct CommitService {
    pool: DbPool,
    registry: Arc<DraftRuntimeRegistry>,
}

impl CommitService {
    pub fn new(pool: DbPool, registry: Arc<DraftRuntimeRegistry>) -> Self {
        Self { pool, registry }
    }

    pub fn pool(&self) -> &DbPool {
        &self.pool
    }

    pub fn registry(&self) -> &Arc<DraftRuntimeRegistry> {
        &self.registry
    }

    /// IPC #19: `get_commit_confirmation_preview`.
    ///
    /// Reads WorkingState, counts pending patches, executes pure validation,
    /// and calculates whether confirmation is currently permitted (`canConfirm`).
    pub fn get_commit_confirmation_preview(
        &self,
        draft_session_id: &Uuid,
    ) -> Result<CommitPreviewDto, IpcError> {
        let conn = self.pool.get().map_err(|e| {
            IpcError::new(ErrorCode::DatabaseError, format!("Failed to acquire connection: {}", e))
        })?;

        // 1. Fetch draft session info
        let mut session_stmt = conn
            .prepare("SELECT status, head_version_seq FROM draft_sessions WHERE id = ?1")
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

        let mut session_rows = session_stmt
            .query(params![draft_session_id.to_string()])
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

        let (status, head_seq_opt): (String, Option<i64>) = if let Some(row) = session_rows
            .next()
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?
        {
            let s: Option<String> = row.get(0).map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
            let h: Option<i64> = row.get(1).map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
            (s.unwrap_or_else(|| "editing".to_string()), h)
        } else {
            return Err(IpcError::new(
                ErrorCode::EntityNotFound,
                format!("Draft session '{}' not found", draft_session_id),
            ));
        };

        // 2. Fetch BusinessFlowData (from registry snapshot if available, else from draft_versions)
        let business_flow_data = if let Some(entry) = self.registry.get(draft_session_id) {
            let snap = entry.get_snapshot();
            snap.business_flow_data.clone()
        } else if let Some(seq) = head_seq_opt {
            let mut ver_stmt = conn
                .prepare("SELECT data_json FROM draft_versions WHERE draft_session_id = ?1 AND seq = ?2")
                .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

            let mut ver_rows = ver_stmt
                .query(params![draft_session_id.to_string(), seq])
                .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

            if let Some(row) = ver_rows
                .next()
                .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?
            {
                let data_json: String = row.get(0).map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
                BusinessFlowData::from_json_str_strict(&data_json).map_err(|e| {
                    IpcError::new(
                        ErrorCode::ValidationFailed,
                        format!("Failed to parse draft version state: {}", e),
                    )
                })?
            } else {
                BusinessFlowData::new()
            }
        } else {
            BusinessFlowData::new()
        };

        // 3. Query pending patches
        let mut patch_stmt = conn
            .prepare(
                "SELECT id, draft_session_id, target_code, field_name, operation, previous_value, new_value, status, created_at
                 FROM pending_patches
                 WHERE draft_session_id = ?1 AND status = 'pending'
                 ORDER BY created_at ASC",
            )
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

        let patch_rows = patch_stmt
            .query_map(params![draft_session_id.to_string()], |row| {
                let id_str: String = row.get(0)?;
                let ds_id_str: String = row.get(1)?;
                let target_code: String = row.get(2)?;
                let field_path: Option<String> = row.get(3)?;
                let op: String = row.get(4)?;
                let prev_val: Option<String> = row.get(5)?;
                let new_val: Option<String> = row.get(6)?;
                let p_status: String = row.get(7)?;
                let created_at: Option<String> = row.get(8)?;

                let parsed_id = Uuid::parse_str(&id_str).unwrap_or_default();
                let parsed_ds_id = Uuid::parse_str(&ds_id_str).unwrap_or_default();
                let prev_json = prev_val.and_then(|s| serde_json::from_str(&s).ok());
                let proposed_json = new_val
                    .and_then(|s| serde_json::from_str(&s).ok())
                    .unwrap_or(serde_json::Value::Null);

                Ok(PendingPatchDto {
                    id: parsed_id,
                    draft_session_id: parsed_ds_id,
                    target_code,
                    field_path: field_path.unwrap_or_default(),
                    operation: op,
                    previous_value: prev_json,
                    proposed_value: proposed_json,
                    status: p_status,
                    reason: None,
                    created_at: created_at.unwrap_or_default(),
                })
            })
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

        let mut pending_items = Vec::new();
        for pr in patch_rows {
            pending_items.push(pr.map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?);
        }
        let pending_count = pending_items.len();

        // 4. Run pure validation
        let report = validate_draft_for_commit(&business_flow_data, pending_count > 0);
        let validation_status = if report.issues.is_empty() {
            "ready".to_string()
        } else {
            "blocked".to_string()
        };

        let issues: Vec<ValidationIssueDto> = report
            .issues
            .into_iter()
            .map(|i| ValidationIssueDto {
                field: i.path,
                message: i.message,
                severity: "error".to_string(),
            })
            .collect();

        // 5. Build plan summary
        let character_count = business_flow_data.require.character.len() as u32;
        let arc_count = business_flow_data.optional.overview_phase.arcs.len() as u32;
        let chapter_count = business_flow_data
            .optional
            .chapter_phase
            .arcs
            .iter()
            .map(|a| a.chapters.len() as u32)
            .sum();
        let world_setting_count = business_flow_data
            .optional
            .world_building_phase
            .world_settings
            .len() as u32;
        let lore_entry_count = business_flow_data
            .optional
            .world_building_phase
            .world_settings
            .iter()
            .map(|ws| ws.lore_entries.len() as u32)
            .sum();

        let plan_summary = PlanSummaryDto {
            project_name: business_flow_data
                .require
                .project
                .display_name
                .value
                .clone()
                .unwrap_or_default(),
            target_arcs: business_flow_data.require.project.target.arc_count.value.unwrap_or(0),
            target_chapters: business_flow_data.require.project.target.chapter_count.value.unwrap_or(0),
            target_words_per_chapter: business_flow_data
                .require
                .project
                .target
                .word_per_chapter_count
                .value
                .unwrap_or(0),
            character_count,
            arc_count,
            chapter_count,
            world_setting_count,
            lore_entry_count,
        };

        // 6. canConfirm criteria
        let can_confirm = status == "editing" && validation_status == "ready" && pending_count == 0;

        Ok(CommitPreviewDto {
            draft_session_id: *draft_session_id,
            status,
            plan_summary,
            validation: CommitValidationResultDto {
                status: validation_status,
                issues,
            },
            pending_patches: PendingPatchesSummaryDto {
                count: pending_count,
                items: pending_items,
            },
            can_confirm,
        })
    }

    /// IPC #20: `commit_draft` (TX-05).
    ///
    /// Executes atomic insertion into the 21 relational tables under `operation_lock`.
    /// - Restricted to `CallerContext::UiAction`.
    /// - Idempotent if already committed.
    /// - Rejects if pending patches exist (`PendingPatchesExist`) or validation fails (`ValidationFailed`).
    /// - Full rollback to `editing` on any collision or relational error.
    pub async fn commit_draft(
        &self,
        draft_session_id: &Uuid,
        caller: CallerContext,
    ) -> Result<CommitDraftResponseDto, IpcError> {
        caller.require_ui_action("commit_draft")?;

        let mut conn = self.pool.get().map_err(|e| {
            IpcError::new(ErrorCode::DatabaseError, format!("Failed to acquire connection: {}", e))
        })?;

        // 1. Fetch draft session info
        let (session_id, project_id_opt, head_seq_opt, status) = {
            let mut session_stmt = conn
                .prepare("SELECT session_id, project_id, head_version_seq, status FROM draft_sessions WHERE id = ?1")
                .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

            let mut rows = session_stmt
                .query(params![draft_session_id.to_string()])
                .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

            if let Some(row) = rows
                .next()
                .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?
            {
                let s_id: String = row.get(0).map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
                let p_id: Option<String> = row.get(1).map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
                let h_seq: Option<i64> = row.get(2).map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
                let st: Option<String> = row.get(3).map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
                (s_id, p_id, h_seq, st.unwrap_or_else(|| "editing".to_string()))
            } else {
                return Err(IpcError::new(
                    ErrorCode::EntityNotFound,
                    format!("Draft session '{}' not found", draft_session_id),
                ));
            }
        };

        // 2. Idempotency check: if already committed, return immediately
        if status == "committed" {
            let pid = project_id_opt
                .as_deref()
                .and_then(|s| Uuid::parse_str(s).ok())
                .unwrap_or(*draft_session_id);
            return Ok(CommitDraftResponseDto {
                status: "committed".to_string(),
                project_id: pid,
                redirect_path: format!("/projects/{}", pid),
            });
        }

        // 3. Verify no pending patches exist
        let pending_count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM pending_patches WHERE draft_session_id = ?1 AND status = 'pending'",
                params![draft_session_id.to_string()],
                |row| row.get(0),
            )
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

        if pending_count > 0 {
            return Err(IpcError::new(
                ErrorCode::PendingPatchesExist,
                format!(
                    "Cannot commit draft: {} unresolved pending patch(es) exist",
                    pending_count
                ),
            ));
        }

        // 4. Retrieve BusinessFlowData
        let business_flow_data = if let Some(entry) = self.registry.get(draft_session_id) {
            let snap = entry.get_snapshot();
            snap.business_flow_data.clone()
        } else if let Some(seq) = head_seq_opt {
            let data_json: String = conn
                .query_row(
                    "SELECT data_json FROM draft_versions WHERE draft_session_id = ?1 AND seq = ?2",
                    params![draft_session_id.to_string(), seq],
                    |row| row.get(0),
                )
                .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
            BusinessFlowData::from_json_str_strict(&data_json).map_err(|e| {
                IpcError::new(
                    ErrorCode::ValidationFailed,
                    format!("Invalid draft version data: {}", e),
                )
            })?
        } else {
            return Err(IpcError::new(
                ErrorCode::ValidationFailed,
                "Draft session has no sealed versions to commit",
            ));
        };

        // 5. Pure validation check
        let report = validate_draft_for_commit(&business_flow_data, false);
        if !report.is_valid_for_commit || !report.issues.is_empty() {
            return Err(IpcError::with_details(
                ErrorCode::ValidationFailed,
                "Draft validation failed for commit",
                serde_json::json!({ "issues": report.issues }),
            ));
        }

        // 6. Acquire operation_lock
        let entry = self
            .registry
            .get_or_register(*draft_session_id, WorkingState::new(*draft_session_id));
        let _guard = match tokio::time::timeout(DRAFT_LOCK_TIMEOUT, entry.operation_lock.lock()).await {
            Ok(g) => g,
            Err(_) => return Err(IpcError::new(ErrorCode::DraftBusy, "Operation lock timeout")),
        };

        // 7. Transition status to 'committing'
        let now_committing = Utc::now().to_rfc3339();
        conn.execute(
            "UPDATE draft_sessions SET status = 'committing', updated_at = ?1 WHERE id = ?2",
            params![now_committing, draft_session_id.to_string()],
        )
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
        entry.set_runtime_status(DraftRuntimeStatus::Committing);

        // 8. Execute TX-05 atomic commit
        let commit_res = commit_draft_tx(&mut conn, draft_session_id, &session_id, &business_flow_data);

        match commit_res {
            Ok(project_id) => {
                entry.set_runtime_status(DraftRuntimeStatus::Committed);
                Ok(CommitDraftResponseDto {
                    status: "committed".to_string(),
                    project_id,
                    redirect_path: format!("/projects/{}", project_id),
                })
            }
            Err(err) => {
                // Step: rollback status to 'editing' on failure
                let now_editing = Utc::now().to_rfc3339();
                let _ = conn.execute(
                    "UPDATE draft_sessions SET status = 'editing', updated_at = ?1 WHERE id = ?2",
                    params![now_editing, draft_session_id.to_string()],
                );
                entry.set_runtime_status(DraftRuntimeStatus::Editing);

                match err {
                    CommitRepoError::CodeCollision(conflicts) => Err(IpcError::with_details(
                        ErrorCode::CodeCollisionOnCommit,
                        "Entity code collision on commit",
                        serde_json::json!({ "conflicts": conflicts }),
                    )),
                    CommitRepoError::DraftReopenForbidden => Err(IpcError::new(
                        ErrorCode::DraftReopenForbidden,
                        "Draft reopen forbidden",
                    )),
                    CommitRepoError::NotFound(msg) | CommitRepoError::EntityNotFound(msg) => {
                        Err(IpcError::new(ErrorCode::EntityNotFound, msg))
                    }
                    CommitRepoError::Database(e) => Err(IpcError::new(
                        ErrorCode::DatabaseError,
                        format!("Database error during TX-05 commit: {}", e),
                    )),
                    CommitRepoError::InvalidState(msg) => {
                        Err(IpcError::new(ErrorCode::ValidationFailed, msg))
                    }
                }
            }
        }
    }

    /// IPC #21: `reopen_committed_draft`.
    ///
    /// Reopens an existing committed draft if and only if no scene content has been written (`be-core.md` §7.3).
    pub async fn reopen_committed_draft(
        &self,
        project_id: &Uuid,
        active_session_id: &str,
    ) -> Result<ReopenCommittedDraftResponseDto, IpcError> {
        let mut conn = self.pool.get().map_err(|e| {
            IpcError::new(ErrorCode::DatabaseError, format!("Failed to acquire connection: {}", e))
        })?;

        match reopen_committed_draft(&mut conn, project_id, active_session_id) {
            Ok(draft_session_id) => {
                if let Some(entry) = self.registry.get(&draft_session_id) {
                    entry.set_runtime_status(DraftRuntimeStatus::Editing);
                }
                Ok(ReopenCommittedDraftResponseDto {
                    draft_session_id,
                    status: "editing".to_string(),
                })
            }
            Err(CommitRepoError::DraftReopenForbidden) => Err(IpcError::new(
                ErrorCode::DraftReopenForbidden,
                "Cannot reopen draft: project already has written scene content",
            )),
            Err(CommitRepoError::NotFound(msg)) => Err(IpcError::new(ErrorCode::EntityNotFound, msg)),
            Err(CommitRepoError::Database(e)) => Err(IpcError::new(
                ErrorCode::DatabaseError,
                format!("Database error during draft reopen: {}", e),
            )),
            Err(err) => Err(IpcError::new(ErrorCode::InternalError, err.to_string())),
        }
    }
}
