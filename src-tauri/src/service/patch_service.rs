//! Patch Service owning TX-08 (Persist Proposal) and TX-02 (Resolve Pending Patches)
//! (`REQ-PATCH-001`..`004`, `REQ-TX-002`, `REQ-TX-008`, `be-core.md` §3.6, §5.3.2, §1538–§1547).

use parking_lot::RwLock;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{params, OptionalExtension, TransactionBehavior};
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::cascade_delete::{execute_delete, CascadeOutcome};
use crate::domain::fingerprint::state_data_hash;
use crate::domain::progress_engine::evaluate_progress;
use crate::domain::tools::registry::CallerCategory;
use crate::error::{ErrorCode, IpcError};
use crate::ipc::dto::{
    ConfirmAllPatchesResponseDto, DraftVersionSummaryDto, PatchResolutionDto, PendingPatchDto,
    PendingPatchListDto, ProgressSummaryDto, SkippedPatchDto,
};
use crate::service::draft_runtime::{
    DraftRuntimeRegistry, DraftRuntimeStatus, WorkingState, DRAFT_LOCK_TIMEOUT,
};
use crate::service::draft_service::{DraftService, SealVersionParams};
use crate::storage::pending_repo::{
    apply_field_mutation, evaluate_proposal, expire_patch, find_by_id, list_by_draft,
    list_pending_by_draft, persist_proposal_tx, update_status_cas, PersistProposalParams,
    PersistProposalResult, ProposalEvaluationResult,
};

/// Helper constructing `ProgressSummaryDto` from working state.
pub fn compute_progress_dto(ws: &WorkingState) -> ProgressSummaryDto {
    let eval = evaluate_progress(&ws.business_flow_data);
    let completed = (eval.confirmed_count + eval.ai_suggested_count) as u32;
    ProgressSummaryDto {
        current_phase: ws.phase,
        current_subphase: ws.subphase,
        completed_attributes: completed,
        total_attributes: 46,
        progress_percentage: (completed as f32 / 46.0) * 100.0,
        is_require_complete: eval.is_require_complete,
        can_advance_phase: if ws.phase == 0 {
            eval.is_require_complete
        } else {
            true
        },
        confirmed_count: Some(eval.confirmed_count as u32),
        ai_inferred_count: Some(eval.ai_suggested_count as u32),
    }
}

/// Simulation hook for failure injection during transactional testing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PatchErrorInjectionPoint {
    None,
    Tx08FailBeforeCommit,
    Tx02FailBeforeCommit,
}

/// Service managing the lifecycle of pending proposal patches (TX-08 and TX-02).
pub struct PatchService {
    pool: Pool<SqliteConnectionManager>,
    registry: Arc<DraftRuntimeRegistry>,
    draft_service: Arc<DraftService>,
    error_injection: RwLock<PatchErrorInjectionPoint>,
}

impl PatchService {
    pub fn new(
        pool: Pool<SqliteConnectionManager>,
        registry: Arc<DraftRuntimeRegistry>,
        draft_service: Arc<DraftService>,
    ) -> Self {
        Self {
            pool,
            registry,
            draft_service,
            error_injection: RwLock::new(PatchErrorInjectionPoint::None),
        }
    }

    pub fn set_error_injection(&self, point: PatchErrorInjectionPoint) {
        *self.error_injection.write() = point;
    }

    pub fn pool(&self) -> &Pool<SqliteConnectionManager> {
        &self.pool
    }

    pub fn registry(&self) -> &Arc<DraftRuntimeRegistry> {
        &self.registry
    }

    pub fn draft_service(&self) -> &Arc<DraftService> {
        &self.draft_service
    }

    // ========================================================================
    // TX-08: Persist Pending Proposal (tool executor)
    // ========================================================================

    /// Persist or deduplicate a proposal patch under SQLite `BEGIN IMMEDIATE` boundary (TX-08).
    ///
    /// Invariants (`be-core.md` §3.6, §5.3.2, §1538):
    /// - Runs under outer `operation_lock(draft)` borrowed from workflow / turn orchestrator (never re-locks).
    /// - `BEGIN IMMEDIATE`, commits before returning `patch_id`.
    /// - Does not hold SQLite TX across network awaits.
    /// - Independent rollback: if TX-04 subsequently fails, committed proposal in TX-08 survives.
    pub fn persist_proposal(
        &self,
        params: &PersistProposalParams,
    ) -> Result<PersistProposalResult, IpcError> {
        let mut conn = self.pool.get().map_err(|e| {
            IpcError::new(
                ErrorCode::DatabaseError,
                format!("Failed to acquire connection from pool: {}", e),
            )
        })?;

        // Optional check against working state if loaded in runtime
        let ws_snapshot = self
            .registry
            .get(&params.draft_session_id)
            .map(|entry| entry.get_snapshot());

        let tx = conn
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|e| {
                IpcError::new(
                    ErrorCode::DatabaseError,
                    format!("Failed to begin IMMEDIATE transaction for TX-08: {}", e),
                )
            })?;

        let res = persist_proposal_tx(
            &tx,
            params,
            ws_snapshot.as_ref().map(|s| &s.business_flow_data),
        )?;

        // Failure injection point check
        if *self.error_injection.read() == PatchErrorInjectionPoint::Tx08FailBeforeCommit {
            return Err(IpcError::new(
                ErrorCode::DatabaseError,
                "Injected failure before TX-08 commit",
            ));
        }

        tx.commit().map_err(|e| {
            IpcError::new(
                ErrorCode::DatabaseError,
                format!("Failed to commit TX-08: {}", e),
            )
        })?;

        // Update in-memory WorkingState.pending_patches if entry is registered
        if let Some(entry) = self.registry.get(&params.draft_session_id) {
            let p_record = {
                let conn_read = self.pool.get().ok();
                conn_read.and_then(|c| find_by_id(&c, &res.patch_id).ok().flatten())
            };
            if let Some(rec) = p_record {
                entry.mutate_working_state(|ws| {
                    if let Some(idx) = ws.pending_patches.iter().position(|p| p.id == rec.id) {
                        ws.pending_patches[idx] = rec.into();
                    } else {
                        ws.pending_patches.push(rec.into());
                    }
                });
            }
        }

        Ok(res)
    }

    // ========================================================================
    // IPC #12: List Pending Patches
    // ========================================================================

    /// Queries pending patches for a draft session and performs lazy expiration check (§5.3.2).
    pub fn list_pending_patches(
        &self,
        draft_session_id: &Uuid,
    ) -> Result<PendingPatchListDto, IpcError> {
        let entry = self
            .draft_service
            .get_or_load_runtime_entry(*draft_session_id)?;
        let snapshot = entry.get_snapshot();

        let conn = self.pool.get().map_err(|e| {
            IpcError::new(
                ErrorCode::DatabaseError,
                format!("Failed to acquire connection: {}", e),
            )
        })?;

        let records = list_by_draft(&conn, draft_session_id).map_err(|e| {
            IpcError::new(
                ErrorCode::DatabaseError,
                format!("Failed to query pending_patches: {}", e),
            )
        })?;

        let now = chrono::Utc::now().to_rfc3339();
        let mut items = Vec::with_capacity(records.len());

        for mut record in records {
            if record.status == "pending" {
                // Perform lazy expiration check
                if let ProposalEvaluationResult::Expired { reason } = evaluate_proposal(&record, &snapshot.business_flow_data) {
                    let _ = expire_patch(&conn, &record.id, &now);
                    record.status = "expired".to_string();
                    let mut dto: PendingPatchDto = record.into();
                    dto.reason = Some(reason);
                    items.push(dto);
                    continue;
                }
            }
            items.push(record.into());
        }

        Ok(PendingPatchListDto { items })
    }

    // ========================================================================
    // TX-02: Confirm Pending Patch (IPC #13)
    // ========================================================================

    /// Confirm a single pending proposal patch under `operation_lock` and TX-02.
    ///
    /// Invariants (`be-core.md` §3.6, §5.3.2, §1538–§1547, IPC #13):
    /// - Requires `CallerContext::UiAction` (caller checks enforced in IPC layer).
    /// - CAS on status: if already `confirmed`, returns `already_confirmed` without creating new version.
    /// - If `expired` (or becomes expired on lazy check), returns `status: "expired"`.
    /// - If missing dependency evidence on workflow origin: returns `ErrorCode::ProposalEligibilityIndeterminate`,
    ///   retaining `pending` status in SQLite.
    /// - If valid: applies mutation to `WorkingState` (`source = 'user_edited'`), marks `status = 'confirmed'`,
    ///   and seals exactly 1 draft version `ai_patch` (`seal_key = "patch_confirm:{patchId}"`).
    pub async fn confirm_pending_patch(
        &self,
        draft_session_id: Uuid,
        patch_id: Uuid,
    ) -> Result<PatchResolutionDto, IpcError> {
        let entry = self
            .draft_service
            .get_or_load_runtime_entry(draft_session_id)?;

        // Acquire Level 1 operation_lock
        let _guard = entry
            .acquire_operation_lock("confirm_pending_patch", DRAFT_LOCK_TIMEOUT)
            .await?;

        let snapshot = entry.get_snapshot();
        if snapshot.status == DraftRuntimeStatus::Committing
            || snapshot.status == DraftRuntimeStatus::Committed
        {
            return Err(IpcError::new(
                ErrorCode::DraftNotEditable,
                format!(
                    "Draft {} is in non-editable state: {}",
                    draft_session_id, snapshot.status
                ),
            ));
        }

        let mut conn = self
            .pool
            .get()
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

        // TX-02 boundary
        let tx = conn
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

        let record = find_by_id(&tx, &patch_id)
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?
            .ok_or_else(|| {
                IpcError::new(
                    ErrorCode::EntityNotFound,
                    format!("Patch {} not found in draft {}", patch_id, draft_session_id),
                )
            })?;

        let now = chrono::Utc::now().to_rfc3339();

        // 1. CAS: Check already confirmed
        if record.status == "confirmed" {
            tx.commit()
                .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
            return Ok(PatchResolutionDto {
                status: "already_confirmed".to_string(),
                sealed_version: None,
                progress: compute_progress_dto(&snapshot),
            });
        }

        // 2. CAS: Check already rejected
        if record.status == "rejected" {
            tx.commit()
                .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
            return Ok(PatchResolutionDto {
                status: "already_rejected".to_string(),
                sealed_version: None,
                progress: compute_progress_dto(&snapshot),
            });
        }

        // 3. CAS: Check already expired
        if record.status == "expired" {
            tx.commit()
                .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
            return Ok(PatchResolutionDto {
                status: "expired".to_string(),
                sealed_version: None,
                progress: compute_progress_dto(&snapshot),
            });
        }

        // 4. Status must be pending -> evaluate proposal on current working state
        let eval_result = evaluate_proposal(&record, &snapshot.business_flow_data);

        match eval_result {
            ProposalEvaluationResult::Indeterminate { reason } => {
                // Fail closed: retain pending status, rollback TX-02
                let _ = tx.rollback();
                return Err(IpcError::new(
                    ErrorCode::ProposalEligibilityIndeterminate,
                    format!(
                        "Proposal {} eligibility indeterminate: {}",
                        patch_id, reason
                    ),
                ));
            }
            ProposalEvaluationResult::Expired { reason } => {
                // Update status to expired in DB
                expire_patch(&tx, &patch_id, &now)
                    .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
                tx.commit()
                    .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

                entry.mutate_working_state(|ws| {
                    if let Some(p) = ws.pending_patches.iter_mut().find(|p| p.id == patch_id) {
                        p.status = "expired".to_string();
                        p.reason = Some(reason);
                    }
                });

                return Ok(PatchResolutionDto {
                    status: "expired".to_string(),
                    sealed_version: None,
                    progress: compute_progress_dto(&snapshot),
                });
            }
            ProposalEvaluationResult::Eligible => {}
        }

        // 5. Apply patch to WorkingState clone
        let mut candidate = snapshot.business_flow_data.clone();

        if record.operation == "set" {
            let field = record.field_name.as_deref().unwrap_or_default();
            let new_val = record
                .new_value
                .as_ref()
                .unwrap_or(&serde_json::Value::Null);
            apply_field_mutation(
                &mut candidate,
                &record.target_type,
                &record.target_code,
                field,
                new_val,
            )
            .map_err(|e| IpcError::new(ErrorCode::ValidationFailed, e))?;
        } else if record.operation == "delete" {
            let outcome = execute_delete(
                &candidate,
                CallerCategory::UiAction,
                &record.target_type,
                &record.target_code,
                None,
            );
            match outcome {
                CascadeOutcome::Applied { mutated_state, .. } => {
                    candidate = *mutated_state;
                }
                CascadeOutcome::Rejected { error, .. } => {
                    return Err(IpcError::new(
                        ErrorCode::ValidationFailed,
                        format!("Delete rejection: {}", error),
                    ));
                }
                _ => {
                    return Err(IpcError::new(
                        ErrorCode::ValidationFailed,
                        "Delete could not be applied",
                    ));
                }
            }
        }

        // 6. Update pending_patches status to confirmed in SQLite
        update_status_cas(&tx, &patch_id, "pending", "confirmed", &now)
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

        // Failure injection point check
        if *self.error_injection.read() == PatchErrorInjectionPoint::Tx02FailBeforeCommit {
            let _ = tx.rollback();
            return Err(IpcError::new(
                ErrorCode::DatabaseError,
                "Injected failure before TX-02 commit",
            ));
        }

        tx.commit()
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

        // 7. Update in-memory WorkingState
        entry.mutate_working_state(|ws| {
            ws.business_flow_data = candidate.clone();
            if let Some(p) = ws.pending_patches.iter_mut().find(|p| p.id == patch_id) {
                p.status = "confirmed".to_string();
            }
        });

        // 8. Seal exactly 1 draft version `ai_patch` with seal_key = "patch_confirm:{patchId}"
        let data_hash = state_data_hash(&candidate)
            .map_err(|e| IpcError::new(ErrorCode::ValidationFailed, e.to_string()))?;
        let data_json = serde_json::to_string(&candidate)
            .map_err(|e| IpcError::new(ErrorCode::ValidationFailed, e.to_string()))?;

        let seal_key = format!("patch_confirm:{}", patch_id);
        let seal_params = SealVersionParams {
            draft_session_id,
            seal_key,
            reason: "ai_patch".to_string(),
            phase: snapshot.phase,
            subphase: snapshot.subphase,
            data_json,
            data_hash,
            parent_seq: snapshot.head_version_seq,
            turn_id: None,
            chat_message_id: None,
        };

        let version_summary = self.draft_service.seal_version(&seal_params)?;
        let final_snapshot = entry.get_snapshot();

        Ok(PatchResolutionDto {
            status: "confirmed".to_string(),
            sealed_version: Some(version_summary),
            progress: compute_progress_dto(&final_snapshot),
        })
    }

    // ========================================================================
    // TX-02: Reject Pending Patch (IPC #14)
    // ========================================================================

    /// Reject a single pending proposal patch under `operation_lock` and TX-02.
    ///
    /// Invariants (`be-core.md` §3.6, §5.3.2, IPC #14):
    /// - Requires `CallerContext::UiAction`.
    /// - CAS on status: if already `rejected`, returns `already_rejected`.
    /// - Sets `status = 'rejected'`, `resolved_at = now()`.
    /// - Does NOT mutate WorkingState and does NOT create `draft_versions`.
    pub async fn reject_pending_patch(
        &self,
        draft_session_id: Uuid,
        patch_id: Uuid,
    ) -> Result<PatchResolutionDto, IpcError> {
        let entry = self
            .draft_service
            .get_or_load_runtime_entry(draft_session_id)?;

        let _guard = entry
            .acquire_operation_lock("reject_pending_patch", DRAFT_LOCK_TIMEOUT)
            .await?;

        let snapshot = entry.get_snapshot();
        if snapshot.status == DraftRuntimeStatus::Committing
            || snapshot.status == DraftRuntimeStatus::Committed
        {
            return Err(IpcError::new(
                ErrorCode::DraftNotEditable,
                format!("Draft {} is non-editable", draft_session_id),
            ));
        }

        let mut conn = self
            .pool
            .get()
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

        let tx = conn
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

        let record = find_by_id(&tx, &patch_id)
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?
            .ok_or_else(|| {
                IpcError::new(
                    ErrorCode::EntityNotFound,
                    format!("Patch {} not found in draft {}", patch_id, draft_session_id),
                )
            })?;

        let now = chrono::Utc::now().to_rfc3339();

        if record.status == "rejected" {
            tx.commit()
                .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
            return Ok(PatchResolutionDto {
                status: "already_rejected".to_string(),
                sealed_version: None,
                progress: compute_progress_dto(&snapshot),
            });
        }

        if record.status == "confirmed" {
            tx.commit()
                .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
            return Ok(PatchResolutionDto {
                status: "already_confirmed".to_string(),
                sealed_version: None,
                progress: compute_progress_dto(&snapshot),
            });
        }

        // Update status to rejected
        update_status_cas(&tx, &patch_id, "pending", "rejected", &now)
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

        tx.commit()
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

        // Update in-memory WorkingState
        entry.mutate_working_state(|ws| {
            if let Some(p) = ws.pending_patches.iter_mut().find(|p| p.id == patch_id) {
                p.status = "rejected".to_string();
            }
        });

        Ok(PatchResolutionDto {
            status: "rejected".to_string(),
            sealed_version: None,
            progress: compute_progress_dto(&snapshot),
        })
    }

    // ========================================================================
    // TX-02: Confirm All Pending Patches (IPC #15)
    // ========================================================================

    /// Confirm all pending patches under `operation_lock` and TX-02.
    ///
    /// Invariants (`be-core.md` §3.6, §5.3.2, §1538–§1547, IPC #15):
    /// - Requires `CallerContext::UiAction`.
    /// - Iterates pending patches sorted by `created_at ASC, id ASC`.
    /// - Revalidates both data validity and confirmation eligibility on the cumulative candidate state.
    /// - Invalid / expired patches are marked `expired` and added to `skipped` without rolling back the batch.
    /// - Missing dependency evidence on workflow origin pushes to `skipped` with indeterminate reason.
    /// - Valid patches applied to candidate with `source = 'user_edited'`.
    /// - If `applied.len() > 0`: seals exactly 1 draft version `ai_patch` (`seal_key = "patch_batch:{clientRequestId}"`).
    /// - If `applied.is_empty()`: no draft version created.
    /// - Idempotent by `client_request_id`.
    pub async fn confirm_all_pending_patches(
        &self,
        draft_session_id: Uuid,
        client_request_id: String,
    ) -> Result<ConfirmAllPatchesResponseDto, IpcError> {
        let entry = self
            .draft_service
            .get_or_load_runtime_entry(draft_session_id)?;

        let _guard = entry
            .acquire_operation_lock("confirm_all_pending_patches", DRAFT_LOCK_TIMEOUT)
            .await?;

        let snapshot = entry.get_snapshot();
        if snapshot.status == DraftRuntimeStatus::Committing
            || snapshot.status == DraftRuntimeStatus::Committed
        {
            return Err(IpcError::new(
                ErrorCode::DraftNotEditable,
                format!("Draft {} is non-editable", draft_session_id),
            ));
        }

        // Idempotency check: see if batch version already sealed
        let seal_key = format!("patch_batch:{}", client_request_id);
        {
            let conn_check = self
                .pool
                .get()
                .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
            let mut check_stmt = conn_check
                .prepare(
                    "SELECT seq, phase, subphase, reason, parent_seq, created_at
                 FROM draft_versions
                 WHERE draft_session_id = ?1 AND seal_key = ?2",
                )
                .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

            let existing_opt = check_stmt
                .query_row(params![draft_session_id.to_string(), seal_key], |row| {
                    let seq: i32 = row.get(0)?;
                    let phase: i32 = row.get(1)?;
                    let subphase: Option<i32> = row.get(2)?;
                    let reason: String = row.get(3)?;
                    let parent_seq: Option<i32> = row.get(4)?;
                    let created_at: String = row.get(5)?;

                    Ok(DraftVersionSummaryDto {
                        seq,
                        display_label: format!("v.{:02}", seq),
                        phase,
                        subphase,
                        reason,
                        parent_seq,
                        created_at,
                        data_hash: None,
                    })
                })
                .optional()
                .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

            if let Some(existing_summary) = existing_opt {
                return Ok(ConfirmAllPatchesResponseDto {
                    applied: Vec::new(),
                    skipped: Vec::new(),
                    sealed_version: Some(existing_summary),
                    progress: compute_progress_dto(&snapshot),
                });
            }
        }

        let mut conn = self
            .pool
            .get()
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

        let tx = conn
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

        let pending_patches = list_pending_by_draft(&tx, &draft_session_id)
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

        let mut applied: Vec<Uuid> = Vec::new();
        let mut skipped: Vec<SkippedPatchDto> = Vec::new();
        let mut candidate = snapshot.business_flow_data.clone();
        let now = chrono::Utc::now().to_rfc3339();

        for record in pending_patches {
            // Revalidate on cumulative candidate state
            let eval = evaluate_proposal(&record, &candidate);

            match eval {
                ProposalEvaluationResult::Indeterminate { reason } => {
                    // Indeterminate: skip, do not expire, retain pending
                    skipped.push(SkippedPatchDto {
                        patch_id: record.id,
                        reason: format!("ProposalEligibilityIndeterminate: {}", reason),
                    });
                }
                ProposalEvaluationResult::Expired { reason } => {
                    // Invalid or expired on cumulative state -> expire in DB
                    let _ = expire_patch(&tx, &record.id, &now);
                    skipped.push(SkippedPatchDto {
                        patch_id: record.id,
                        reason,
                    });
                }
                ProposalEvaluationResult::Eligible => {
                    // Apply to cumulative candidate state
                    let apply_res = if record.operation == "set" {
                        let field = record.field_name.as_deref().unwrap_or_default();
                        let new_val = record
                            .new_value
                            .as_ref()
                            .unwrap_or(&serde_json::Value::Null);
                        apply_field_mutation(
                            &mut candidate,
                            &record.target_type,
                            &record.target_code,
                            field,
                            new_val,
                        )
                    } else if record.operation == "delete" {
                        let outcome = execute_delete(
                            &candidate,
                            CallerCategory::UiAction,
                            &record.target_type,
                            &record.target_code,
                            None,
                        );
                        match outcome {
                            CascadeOutcome::Applied { mutated_state, .. } => {
                                candidate = *mutated_state;
                                Ok(())
                            }
                            CascadeOutcome::Rejected { error, .. } => Err(error),
                            _ => Err("Delete could not be applied".to_string()),
                        }
                    } else {
                        Err(format!("Unsupported operation '{}'", record.operation))
                    };

                    match apply_res {
                        Ok(()) => {
                            let _ =
                                update_status_cas(&tx, &record.id, "pending", "confirmed", &now);
                            applied.push(record.id);
                        }
                        Err(e) => {
                            let _ = expire_patch(&tx, &record.id, &now);
                            skipped.push(SkippedPatchDto {
                                patch_id: record.id,
                                reason: format!("Failed to apply to cumulative candidate: {}", e),
                            });
                        }
                    }
                }
            }
        }

        // Failure injection point check
        if *self.error_injection.read() == PatchErrorInjectionPoint::Tx02FailBeforeCommit {
            let _ = tx.rollback();
            return Err(IpcError::new(
                ErrorCode::DatabaseError,
                "Injected failure before TX-02 commit in confirm_all",
            ));
        }

        tx.commit()
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

        // Update in-memory WorkingState
        entry.mutate_working_state(|ws| {
            if !applied.is_empty() {
                ws.business_flow_data = candidate.clone();
            }
            for p in &mut ws.pending_patches {
                if applied.contains(&p.id) {
                    p.status = "confirmed".to_string();
                } else if skipped
                    .iter()
                    .any(|s| s.patch_id == p.id && !s.reason.contains("Indeterminate"))
                {
                    p.status = "expired".to_string();
                }
            }
        });

        // Seal version if applied > 0
        let sealed_version = if !applied.is_empty() {
            let data_hash = state_data_hash(&candidate)
                .map_err(|e| IpcError::new(ErrorCode::ValidationFailed, e.to_string()))?;
            let data_json = serde_json::to_string(&candidate)
                .map_err(|e| IpcError::new(ErrorCode::ValidationFailed, e.to_string()))?;

            let seal_params = SealVersionParams {
                draft_session_id,
                seal_key,
                reason: "ai_patch".to_string(),
                phase: snapshot.phase,
                subphase: snapshot.subphase,
                data_json,
                data_hash,
                parent_seq: snapshot.head_version_seq,
                turn_id: None,
                chat_message_id: None,
            };

            Some(self.draft_service.seal_version(&seal_params)?)
        } else {
            None
        };

        let final_snapshot = entry.get_snapshot();

        Ok(ConfirmAllPatchesResponseDto {
            applied,
            skipped,
            sealed_version,
            progress: compute_progress_dto(&final_snapshot),
        })
    }
}
