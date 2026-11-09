//! Draft Service owning TX-01 (Seal Draft Version) and TX-03 (Restore Draft Version)
//! (`REQ-DRAFT-001`, `REQ-TX-001`, `REQ-TX-003`, `be-core.md` §3.7, §4.4, §4.5, `CI-03`).

use crate::domain::business_flow::BusinessFlowData;
use crate::domain::progress_engine::evaluate_progress;
use crate::error::{ErrorCode, IpcError};
use crate::ipc::dto::{
    DraftVersionListDto, DraftVersionSummaryDto, DraftWorkspaceStateDto,
    PendingPatchDto, ProgressSummaryDto, RestoreVersionResponseDto,
};
use crate::service::draft_runtime::{
    DraftRuntimeEntry, DraftRuntimeRegistry, DraftRuntimeStatus, WorkingState, DRAFT_LOCK_TIMEOUT,
};
use parking_lot::RwLock;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{params, TransactionBehavior};
use std::sync::Arc;
use uuid::Uuid;

/// Parameters for sealing a draft version (TX-01).
#[derive(Debug, Clone)]
pub struct SealVersionParams {
    pub draft_session_id: Uuid,
    pub seal_key: String,
    pub reason: String,
    pub phase: i32,
    pub subphase: Option<i32>,
    pub data_json: String,
    pub data_hash: String,
    pub parent_seq: Option<i32>,
    pub turn_id: Option<Uuid>,
    pub chat_message_id: Option<Uuid>,
}

/// Parameters for restoring a draft version (TX-03).
#[derive(Debug, Clone)]
pub struct RestoreVersionParams {
    pub draft_session_id: Uuid,
    pub target_seq: i32,
    pub client_request_id: String,
}

/// Simulation hook for failure injection during transactional testing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorInjectionPoint {
    None,
    Tx01FailOnce,
    Tx01AlwaysFail,
    BeforeTx01Commit,
    Tx03AlwaysFail,
    BeforeTx03Commit,
}

/// Core application service managing durable draft version sealing and restoration.
pub struct DraftService {
    pool: Pool<SqliteConnectionManager>,
    registry: Arc<DraftRuntimeRegistry>,
    error_injection: RwLock<ErrorInjectionPoint>,
    fail_once_counter: RwLock<u32>,
}

impl DraftService {
    pub fn new(pool: Pool<SqliteConnectionManager>, registry: Arc<DraftRuntimeRegistry>) -> Self {
        Self {
            pool,
            registry,
            error_injection: RwLock::new(ErrorInjectionPoint::None),
            fail_once_counter: RwLock::new(0),
        }
    }

    pub fn registry(&self) -> &Arc<DraftRuntimeRegistry> {
        &self.registry
    }

    pub fn pool(&self) -> &Pool<SqliteConnectionManager> {
        &self.pool
    }

    pub fn set_error_injection(&self, point: ErrorInjectionPoint) {
        *self.error_injection.write() = point;
        *self.fail_once_counter.write() = 0;
    }

    // ========================================================================
    // TX-01: Seal Draft Version
    // ========================================================================

    /// Executes TX-01 with automatic retry on transient SQLite errors.
    ///
    /// Invariants:
    /// - `BEGIN IMMEDIATE` transaction boundary.
    /// - Idempotent by `(draft_session_id, seal_key)`: identical payload returns existing version.
    /// - Mismatched payload for existing seal_key returns `ErrorCode::SealKeyPayloadMismatch`.
    /// - Retries at most once; if still failing, returns `ErrorCode::VersionSealFailed`
    ///   and marks in-memory status as `SealPendingRetry` (`CI-03`).
    pub fn seal_version(&self, params: &SealVersionParams) -> Result<DraftVersionSummaryDto, IpcError> {
        // Attempt 1:
        let first_res = self.execute_seal_version_tx(params);
        match first_res {
            Ok(dto) => Ok(dto),
            Err(err) => {
                // Non-retryable conflict errors
                if err.code == ErrorCode::SealKeyPayloadMismatch
                    || err.code == ErrorCode::DraftNotEditable
                    || err.code == ErrorCode::EntityNotFound
                    || err.code == ErrorCode::ValidationFailed
                {
                    return Err(err);
                }

                // Attempt 2 (retry once):
                match self.execute_seal_version_tx(params) {
                    Ok(dto) => Ok(dto),
                    Err(_retry_err) => {
                        // Mark in-memory runtime status as SealPendingRetry without mutating SQLite (CI-03)
                        if let Some(entry) = self.registry.get(&params.draft_session_id) {
                            entry.set_runtime_status(DraftRuntimeStatus::SealPendingRetry);
                        }
                        Err(IpcError::new(
                            ErrorCode::VersionSealFailed,
                            "Failed to seal draft version after retry attempt",
                        ))
                    }
                }
            }
        }
    }

    /// Inner single-transaction execution of TX-01 within `BEGIN IMMEDIATE`.
    fn execute_seal_version_tx(&self, req: &SealVersionParams) -> Result<DraftVersionSummaryDto, IpcError> {
        let mut conn = self.pool.get().map_err(|e| {
            IpcError::new(
                ErrorCode::DatabaseError,
                format!("Failed to acquire connection from pool: {}", e),
            )
        })?;

        let tx = conn
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|e| {
                IpcError::new(
                    ErrorCode::DatabaseError,
                    format!("Failed to begin IMMEDIATE transaction for TX-01: {}", e),
                )
            })?;

        // Step 1: Check idempotency according to (draft_session_id, seal_key) [SF-01, T-IDEM-01]
        let existing_version_opt = {
            let mut check_seal = tx
                .prepare(
                    "SELECT id, seq, phase, subphase, reason, parent_seq, created_at, data_hash
                     FROM draft_versions
                     WHERE draft_session_id = ?1 AND seal_key = ?2",
                )
                .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

            let mut rows = check_seal
                .query(params![req.draft_session_id.to_string(), req.seal_key])
                .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

            if let Some(row) = rows
                .next()
                .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?
            {
                let existing_seq: i64 = row.get(1).map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
                let existing_phase: Option<i64> = row.get(2).map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
                let existing_subphase: Option<i64> = row.get(3).map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
                let existing_reason: String = row.get(4).map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
                let existing_parent_seq: Option<i64> = row.get(5).map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
                let existing_created_at: String = row.get(6).map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
                let existing_hash: String = row.get(7).map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

                Some((
                    existing_seq,
                    existing_phase,
                    existing_subphase,
                    existing_reason,
                    existing_parent_seq,
                    existing_created_at,
                    existing_hash,
                ))
            } else {
                None
            }
        };

        if let Some((
            existing_seq,
            existing_phase,
            existing_subphase,
            existing_reason,
            existing_parent_seq,
            existing_created_at,
            existing_hash,
        )) = existing_version_opt
        {
            if existing_hash == req.data_hash {
                tx.commit()
                    .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
                return Ok(DraftVersionSummaryDto {
                    seq: existing_seq as i32,
                    display_label: format!("v.{:02}", existing_seq),
                    phase: existing_phase.map(|p| p as i32).unwrap_or(0),
                    subphase: existing_subphase.map(|s| s as i32),
                    reason: existing_reason,
                    parent_seq: existing_parent_seq.map(|s| s as i32),
                    created_at: existing_created_at,
                    data_hash: Some(existing_hash),
                });
            } else {
                return Err(IpcError::new(
                    ErrorCode::SealKeyPayloadMismatch,
                    format!(
                        "Idempotency conflict: seal_key '{}' already exists with different data_hash",
                        req.seal_key
                    ),
                ));
            }
        }

        // Step 2: Read current draft_sessions row
        let (session_status, head_version_seq): (Option<String>, Option<i64>) = {
            let mut check_session = tx
                .prepare("SELECT status, head_version_seq FROM draft_sessions WHERE id = ?1")
                .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

            let mut s_rows = check_session
                .query(params![req.draft_session_id.to_string()])
                .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

            match s_rows
                .next()
                .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?
            {
                Some(r) => (
                    r.get(0).map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?,
                    r.get(1).map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?,
                ),
                None => {
                    return Err(IpcError::new(
                        ErrorCode::EntityNotFound,
                        format!("Draft session '{}' not found", req.draft_session_id),
                    ));
                }
            }
        };

        let status_str = session_status.as_deref().unwrap_or("editing");
        if status_str != "editing" {
            return Err(IpcError::new(
                ErrorCode::DraftNotEditable,
                format!("Draft session is not in editing state (current: {})", status_str),
            ));
        }

        // Step 3: Prevent duplicate version if data_hash is identical with head
        // (except for phase_confirm which always seals phase advancement)
        let head_match_opt = if let Some(head_seq) = head_version_seq {
            let mut check_head = tx
                .prepare(
                    "SELECT id, seq, phase, subphase, reason, parent_seq, created_at, data_hash
                     FROM draft_versions
                     WHERE draft_session_id = ?1 AND seq = ?2",
                )
                .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

            let mut h_rows = check_head
                .query(params![req.draft_session_id.to_string(), head_seq])
                .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

            if let Some(h_row) = h_rows
                .next()
                .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?
            {
                let h_phase: Option<i64> = h_row.get(2).map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
                let h_subphase: Option<i64> = h_row.get(3).map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
                let h_reason: String = h_row.get(4).map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
                let h_parent_seq: Option<i64> = h_row.get(5).map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
                let h_created_at: String = h_row.get(6).map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
                let h_hash: String = h_row.get(7).map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

                if h_hash == req.data_hash
                    && h_phase == Some(req.phase as i64)
                    && h_subphase == req.subphase.map(|s| s as i64)
                    && req.reason != "phase_confirm"
                {
                    Some((head_seq, h_phase, h_subphase, h_reason, h_parent_seq, h_created_at, h_hash))
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };

        if let Some((head_seq, h_phase, h_subphase, h_reason, h_parent_seq, h_created_at, h_hash)) = head_match_opt {
            tx.commit()
                .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
            return Ok(DraftVersionSummaryDto {
                seq: head_seq as i32,
                display_label: format!("v.{:02}", head_seq),
                phase: h_phase.map(|p| p as i32).unwrap_or(0),
                subphase: h_subphase.map(|s| s as i32),
                reason: h_reason,
                parent_seq: h_parent_seq.map(|s| s as i32),
                created_at: h_created_at,
                data_hash: Some(h_hash),
            });
        }

        // Test error injection check
        {
            let inj = *self.error_injection.read();
            match inj {
                ErrorInjectionPoint::Tx01FailOnce => {
                    let mut cnt = self.fail_once_counter.write();
                    if *cnt == 0 {
                        *cnt += 1;
                        return Err(IpcError::new(ErrorCode::DatabaseError, "Injected transient TX-01 error"));
                    }
                }
                ErrorInjectionPoint::Tx01AlwaysFail | ErrorInjectionPoint::BeforeTx01Commit => {
                    return Err(IpcError::new(ErrorCode::DatabaseError, "Injected persistent TX-01 error"));
                }
                _ => {}
            }
        }

        // Step 4: Allocate next seq and insert new draft_versions row
        let next_seq = head_version_seq.unwrap_or(0) + 1;
        let version_id = Uuid::now_v7().to_string();
        let now_iso = chrono::Utc::now().to_rfc3339();
        let parent_seq = req.parent_seq.map(|s| s as i64).or(head_version_seq);

        tx.execute(
            "INSERT INTO draft_versions (
                id, draft_session_id, seq, seal_key, phase, subphase,
                data_json, data_hash, reason, parent_seq, turn_id, chat_message_id, created_at
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
            params![
                version_id,
                req.draft_session_id.to_string(),
                next_seq,
                req.seal_key,
                req.phase,
                req.subphase,
                req.data_json,
                req.data_hash,
                req.reason,
                parent_seq,
                req.turn_id.map(|u| u.to_string()),
                req.chat_message_id.map(|u| u.to_string()),
                now_iso,
            ],
        )
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, format!("Insert draft_versions failed: {}", e)))?;

        // Step 5: Update draft_sessions head_version_seq
        tx.execute(
            "UPDATE draft_sessions SET head_version_seq = ?1, updated_at = ?2 WHERE id = ?3",
            params![next_seq, now_iso, req.draft_session_id.to_string()],
        )
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, format!("Update draft_sessions failed: {}", e)))?;

        tx.commit()
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, format!("Commit TX-01 failed: {}", e)))?;

        // Step 6: Synchronize in-memory WorkingState
        if let Some(entry) = self.registry.get(&req.draft_session_id) {
            entry.mutate_working_state(|ws| {
                ws.head_version_seq = Some(next_seq as i32);
                ws.phase = req.phase;
                ws.subphase = req.subphase;
                if let Ok(bfd) = BusinessFlowData::from_json_str_strict(&req.data_json) {
                    ws.business_flow_data = bfd;
                }
                ws.status = DraftRuntimeStatus::Editing;
            });
        }

        Ok(DraftVersionSummaryDto {
            seq: next_seq as i32,
            display_label: format!("v.{:02}", next_seq),
            phase: req.phase,
            subphase: req.subphase,
            reason: req.reason.clone(),
            parent_seq: parent_seq.map(|s| s as i32),
            created_at: now_iso,
            data_hash: Some(req.data_hash.clone()),
        })
    }

    // ========================================================================
    // TX-03: Restore Draft Version
    // ========================================================================

    /// Executes TX-03 inside `BEGIN IMMEDIATE` boundary.
    ///
    /// Invariants (`core.md` §1.3, §3.6, `be-core.md` §4.5, `CI-03`, `T-REST-01`, `P3S2-B`):
    /// - Always appends a new `draft_versions` record with `seq = head + 1`, `reason = 'restore'`,
    ///   `parent_seq = target_seq`.
    /// - Atomically restores `BusinessFlowData`, `phase`, `subphase`, and `_counters`.
    /// - Atomically transitions all `pending_patches` in `'pending'` status to `'expired'`.
    /// - Idempotent by `client_request_id`: repeated calls with identical `(client_request_id, target_seq)`
    ///   return the existing restored version.
    pub async fn restore_version(
        &self,
        params: &RestoreVersionParams,
    ) -> Result<RestoreVersionResponseDto, IpcError> {
        let entry = self.get_or_load_runtime_entry(params.draft_session_id)?;

        // Hold per-draft operation_lock (Level 1)
        let _guard = entry
            .acquire_operation_lock("restore_draft_version", DRAFT_LOCK_TIMEOUT)
            .await?;

        self.execute_restore_version_tx(params, &entry)
    }

    fn execute_restore_version_tx(
        &self,
        req: &RestoreVersionParams,
        entry: &Arc<DraftRuntimeEntry>,
    ) -> Result<RestoreVersionResponseDto, IpcError> {
        let mut conn = self.pool.get().map_err(|e| {
            IpcError::new(
                ErrorCode::DatabaseError,
                format!("Failed to acquire connection from pool: {}", e),
            )
        })?;

        let tx = conn
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|e| {
                IpcError::new(
                    ErrorCode::DatabaseError,
                    format!("Failed to begin IMMEDIATE transaction for TX-03: {}", e),
                )
            })?;

        // 1. Verify draft_sessions status
        let (session_status, head_version_seq): (Option<String>, Option<i64>) = {
            let mut check_session = tx
                .prepare("SELECT status, head_version_seq FROM draft_sessions WHERE id = ?1")
                .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

            let mut s_rows = check_session
                .query(params![req.draft_session_id.to_string()])
                .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

            match s_rows
                .next()
                .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?
            {
                Some(r) => (
                    r.get(0).map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?,
                    r.get(1).map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?,
                ),
                None => {
                    return Err(IpcError::new(
                        ErrorCode::EntityNotFound,
                        format!("Draft session '{}' not found", req.draft_session_id),
                    ));
                }
            }
        };

        let status_str = session_status.as_deref().unwrap_or("editing");
        if status_str != "editing" {
            return Err(IpcError::new(
                ErrorCode::DraftNotEditable,
                format!("Draft session is not in editing state (current: {})", status_str),
            ));
        }

        // 2. Idempotency check: seal_key = format!("restore:{}:{}", client_request_id, target_seq)
        let seal_key = format!("restore:{}:{}", req.client_request_id, req.target_seq);
        let existing_restore_opt = {
            let mut check_seal = tx
                .prepare(
                    "SELECT id, seq, phase, subphase, reason, parent_seq, created_at, data_hash
                     FROM draft_versions
                     WHERE draft_session_id = ?1 AND seal_key = ?2",
                )
                .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

            let mut seal_rows = check_seal
                .query(params![req.draft_session_id.to_string(), seal_key])
                .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

            if let Some(row) = seal_rows
                .next()
                .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?
            {
                let existing_seq: i64 = row.get(1).map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
                let existing_phase: Option<i64> = row.get(2).map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
                let existing_subphase: Option<i64> = row.get(3).map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
                let existing_reason: String = row.get(4).map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
                let existing_parent_seq: Option<i64> = row.get(5).map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
                let existing_created_at: String = row.get(6).map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
                let existing_hash: String = row.get(7).map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

                Some((
                    existing_seq,
                    existing_phase,
                    existing_subphase,
                    existing_reason,
                    existing_parent_seq,
                    existing_created_at,
                    existing_hash,
                ))
            } else {
                None
            }
        };

        if let Some((
            existing_seq,
            existing_phase,
            existing_subphase,
            existing_reason,
            existing_parent_seq,
            existing_created_at,
            existing_hash,
        )) = existing_restore_opt
        {
            if existing_parent_seq == Some(req.target_seq as i64) {
                tx.commit()
                    .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
                let ws_snapshot = self.get_workspace_state(&req.draft_session_id)?;
                return Ok(RestoreVersionResponseDto {
                    new_version: DraftVersionSummaryDto {
                        seq: existing_seq as i32,
                        display_label: format!("v.{:02}", existing_seq),
                        phase: existing_phase.map(|p| p as i32).unwrap_or(0),
                        subphase: existing_subphase.map(|s| s as i32),
                        reason: existing_reason,
                        parent_seq: existing_parent_seq.map(|s| s as i32),
                        created_at: existing_created_at,
                        data_hash: Some(existing_hash),
                    },
                    restored_phase: existing_phase.map(|p| p as i32).unwrap_or(0),
                    restored_subphase: existing_subphase.map(|s| s as i32),
                    expired_patch_ids: Vec::new(),
                    working_state: ws_snapshot,
                });
            } else {
                return Err(IpcError::new(
                    ErrorCode::SealKeyPayloadMismatch,
                    "Idempotency conflict: seal_key already used with different target_seq",
                ));
            }
        }

        // 3. Look up target version record
        let (target_phase, target_subphase, target_data_json, target_data_hash): (
            Option<i64>,
            Option<i64>,
            String,
            String,
        ) = {
            let mut target_stmt = tx
                .prepare(
                    "SELECT phase, subphase, data_json, data_hash
                     FROM draft_versions
                     WHERE draft_session_id = ?1 AND seq = ?2",
                )
                .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

            let mut t_rows = target_stmt
                .query(params![req.draft_session_id.to_string(), req.target_seq])
                .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

            match t_rows
                .next()
                .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?
            {
                Some(r) => (
                    r.get(0).map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?,
                    r.get(1).map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?,
                    r.get(2).map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?,
                    r.get(3).map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?,
                ),
                None => {
                    return Err(IpcError::new(
                        ErrorCode::EntityNotFound,
                        format!(
                            "Target version {} not found for draft {}",
                            req.target_seq, req.draft_session_id
                        ),
                    ));
                }
            }
        };

        // Handle legacy phase nullable (CI-03 3d): fallback to phase 0 if NULL
        let restored_phase = target_phase.map(|p| p as i32).unwrap_or(0);
        let restored_subphase = if restored_phase == 5 {
            target_subphase.map(|s| s as i32)
        } else {
            None
        };

        // Test error injection check
        {
            let inj = *self.error_injection.read();
            if inj == ErrorInjectionPoint::Tx03AlwaysFail || inj == ErrorInjectionPoint::BeforeTx03Commit {
                return Err(IpcError::new(ErrorCode::DatabaseError, "Injected TX-03 error"));
            }
        }

        // 4. Append new draft_versions row (seq = head + 1, reason = 'restore', parent_seq = targetSeq)
        let next_seq = head_version_seq.unwrap_or(0) + 1;
        let new_version_id = Uuid::now_v7().to_string();
        let now_iso = chrono::Utc::now().to_rfc3339();

        tx.execute(
            "INSERT INTO draft_versions (
                id, draft_session_id, seq, seal_key, phase, subphase,
                data_json, data_hash, reason, parent_seq, turn_id, chat_message_id, created_at
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
            params![
                new_version_id,
                req.draft_session_id.to_string(),
                next_seq,
                seal_key,
                restored_phase,
                restored_subphase,
                target_data_json,
                target_data_hash,
                "restore",
                req.target_seq as i64,
                Option::<String>::None,
                Option::<String>::None,
                now_iso,
            ],
        )
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, format!("Insert restore version failed: {}", e)))?;

        // 5. Update draft_sessions head_version_seq
        tx.execute(
            "UPDATE draft_sessions SET head_version_seq = ?1, updated_at = ?2 WHERE id = ?3",
            params![next_seq, now_iso, req.draft_session_id.to_string()],
        )
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, format!("Update head_version_seq failed: {}", e)))?;

        // 6. Expire all pending_patches for this draft_session_id with status = 'pending'
        let expired_patch_ids = {
            let mut find_pending = tx
                .prepare("SELECT id FROM pending_patches WHERE draft_session_id = ?1 AND status = 'pending'")
                .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

            let p_rows = find_pending
                .query_map(params![req.draft_session_id.to_string()], |r| {
                    let id_str: String = r.get(0)?;
                    Ok(id_str)
                })
                .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

            let mut list = Vec::new();
            for r in p_rows {
                if let Ok(id_str) = r {
                    if let Ok(u) = Uuid::parse_str(&id_str) {
                        list.push(u);
                    }
                }
            }
            list
        };

        tx.execute(
            "UPDATE pending_patches SET status = 'expired', resolved_at = ?1 WHERE draft_session_id = ?2 AND status = 'pending'",
            params![now_iso, req.draft_session_id.to_string()],
        )
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, format!("Expire pending patches failed: {}", e)))?;

        tx.commit()
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, format!("Commit TX-03 failed: {}", e)))?;

        // 7. Parse restored BusinessFlowData and update in-memory WorkingState
        let parsed_bfd = BusinessFlowData::from_json_str_strict(&target_data_json).map_err(|e| {
            IpcError::new(
                ErrorCode::DatabaseError,
                format!("Restored state JSON syntax error: {}", e),
            )
        })?;

        let expired_ids_set = expired_patch_ids.clone();
        entry.mutate_working_state(|ws| {
            ws.phase = restored_phase;
            ws.subphase = restored_subphase;
            ws.head_version_seq = Some(next_seq as i32);
            ws.business_flow_data = parsed_bfd;
            ws.status = DraftRuntimeStatus::Editing;
            for p in &mut ws.pending_patches {
                if expired_ids_set.contains(&p.id) {
                    p.status = "expired".to_string();
                }
            }
        });

        let ws_dto = self.get_workspace_state(&req.draft_session_id)?;

        let version_summary = DraftVersionSummaryDto {
            seq: next_seq as i32,
            display_label: format!("v.{:02}", next_seq),
            phase: restored_phase,
            subphase: restored_subphase,
            reason: "restore".to_string(),
            parent_seq: Some(req.target_seq),
            created_at: now_iso,
            data_hash: Some(target_data_hash),
        };

        Ok(RestoreVersionResponseDto {
            new_version: version_summary,
            restored_phase,
            restored_subphase,
            expired_patch_ids,
            working_state: ws_dto,
        })
    }

    // ========================================================================
    // State Queries and Workspace Assembly
    // ========================================================================

    /// Assembles non-blocking `DraftWorkspaceStateDto` snapshot (`IPC #5`).
    pub fn get_workspace_state(&self, draft_session_id: &Uuid) -> Result<DraftWorkspaceStateDto, IpcError> {
        let entry = self.get_or_load_runtime_entry(*draft_session_id)?;
        let snapshot = entry.get_snapshot();
        let lock_state = entry.get_lock_status();

        let progress_eval = evaluate_progress(&snapshot.business_flow_data);
        let completed = (progress_eval.confirmed_count + progress_eval.ai_suggested_count) as u32;
        let progress = ProgressSummaryDto {
            current_phase: snapshot.phase,
            current_subphase: snapshot.subphase,
            completed_attributes: completed,
            total_attributes: 46,
            progress_percentage: (completed as f32 / 46.0) * 100.0,
            is_require_complete: progress_eval.is_require_complete,
            can_advance_phase: if snapshot.phase == 0 {
                progress_eval.is_require_complete
            } else {
                true
            },
            confirmed_count: Some(progress_eval.confirmed_count as u32),
            ai_inferred_count: Some(progress_eval.ai_suggested_count as u32),
        };

        let bfd_json = serde_json::to_value(&snapshot.business_flow_data).unwrap_or(serde_json::json!({}));

        Ok(DraftWorkspaceStateDto {
            draft_session_id: *draft_session_id,
            status: snapshot.status.as_str().to_string(),
            phase: snapshot.phase,
            subphase: snapshot.subphase,
            head_version_seq: snapshot.head_version_seq,
            business_flow_data: bfd_json,
            draft_overview_ui: snapshot.draft_overview_ui.clone(),
            progress,
            pending_patches: snapshot.pending_patches.clone(),
            event_log: snapshot.event_log.clone(),
            lock_state,
        })
    }

    /// Queries draft version list ordered by `seq DESC` (`IPC #10`).
    pub fn list_versions(&self, draft_session_id: &Uuid) -> Result<DraftVersionListDto, IpcError> {
        let conn = self.pool.get().map_err(|e| {
            IpcError::new(
                ErrorCode::DatabaseError,
                format!("Failed to acquire connection: {}", e),
            )
        })?;

        // Verify draft_session exists
        let head_version_seq: Option<i64> = {
            let mut session_stmt = conn
                .prepare("SELECT head_version_seq FROM draft_sessions WHERE id = ?1")
                .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

            let mut s_rows = session_stmt
                .query(params![draft_session_id.to_string()])
                .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

            match s_rows
                .next()
                .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?
            {
                Some(r) => r.get(0).map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?,
                None => {
                    return Err(IpcError::new(
                        ErrorCode::EntityNotFound,
                        format!("Draft session '{}' not found", draft_session_id),
                    ));
                }
            }
        };

        let mut stmt = conn
            .prepare(
                "SELECT seq, phase, subphase, reason, parent_seq, created_at, data_hash
                 FROM draft_versions
                 WHERE draft_session_id = ?1
                 ORDER BY seq DESC",
            )
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

        let rows = stmt
            .query_map(params![draft_session_id.to_string()], |row| {
                let seq: i64 = row.get(0)?;
                let phase: Option<i64> = row.get(1)?;
                let subphase: Option<i64> = row.get(2)?;
                let reason: String = row.get(3)?;
                let parent_seq: Option<i64> = row.get(4)?;
                let created_at: String = row.get(5)?;
                let data_hash: Option<String> = row.get(6)?;

                Ok(DraftVersionSummaryDto {
                    seq: seq as i32,
                    display_label: format!("v.{:02}", seq),
                    phase: phase.map(|p| p as i32).unwrap_or(0),
                    subphase: subphase.map(|s| s as i32),
                    reason,
                    parent_seq: parent_seq.map(|s| s as i32),
                    created_at,
                    data_hash,
                })
            })
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

        let mut versions = Vec::new();
        for r in rows {
            versions.push(r.map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?);
        }

        Ok(DraftVersionListDto {
            head_version_seq: head_version_seq.map(|s| s as i32),
            versions,
        })
    }

    /// Loads or retrieves an active `DraftRuntimeEntry` for `draft_session_id`.
    pub fn get_or_load_runtime_entry(&self, draft_session_id: Uuid) -> Result<Arc<DraftRuntimeEntry>, IpcError> {
        if let Some(entry) = self.registry.get(&draft_session_id) {
            return Ok(entry);
        }

        let conn = self.pool.get().map_err(|e| {
            IpcError::new(
                ErrorCode::DatabaseError,
                format!("Failed to acquire connection: {}", e),
            )
        })?;

        let (status_str, head_seq): (Option<String>, Option<i64>) = {
            let mut session_stmt = conn
                .prepare("SELECT status, head_version_seq FROM draft_sessions WHERE id = ?1")
                .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

            let mut s_rows = session_stmt
                .query(params![draft_session_id.to_string()])
                .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

            match s_rows
                .next()
                .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?
            {
                Some(r) => (
                    r.get(0).map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?,
                    r.get(1).map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?,
                ),
                None => {
                    return Err(IpcError::new(
                        ErrorCode::EntityNotFound,
                        format!("Draft session '{}' not found", draft_session_id),
                    ));
                }
            }
        };

        let runtime_status = status_str
            .as_deref()
            .and_then(DraftRuntimeStatus::from_str)
            .unwrap_or(DraftRuntimeStatus::Editing);

        let mut ws = WorkingState::new(draft_session_id);
        ws.status = runtime_status;

        // If head version exists, load BusinessFlowData from SQLite
        if let Some(seq) = head_seq {
            let mut v_stmt = conn
                .prepare("SELECT phase, subphase, data_json FROM draft_versions WHERE draft_session_id = ?1 AND seq = ?2")
                .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

            let mut v_rows = v_stmt
                .query(params![draft_session_id.to_string(), seq])
                .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

            if let Some(vr) = v_rows
                .next()
                .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?
            {
                let phase: Option<i64> = vr.get(0).map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
                let subphase: Option<i64> = vr.get(1).map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
                let data_json: String = vr.get(2).map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

                ws.phase = phase.map(|p| p as i32).unwrap_or(0);
                ws.subphase = subphase.map(|s| s as i32);
                ws.head_version_seq = Some(seq as i32);
                if let Ok(bfd) = BusinessFlowData::from_json_str_strict(&data_json) {
                    ws.business_flow_data = bfd;
                }
            }
        }

        // Load pending patches
        let mut p_stmt = conn
            .prepare(
                "SELECT id, target_code, field_name, operation, previous_value, new_value, status, created_at
                 FROM pending_patches
                 WHERE draft_session_id = ?1
                 ORDER BY created_at ASC",
            )
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

        let p_rows = p_stmt
            .query_map(params![draft_session_id.to_string()], |row| {
                let id_str: String = row.get(0)?;
                let target_code: String = row.get(1)?;
                let field_path: Option<String> = row.get(2)?;
                let op: String = row.get(3)?;
                let prev_json: Option<String> = row.get(4)?;
                let new_json: Option<String> = row.get(5)?;
                let st: Option<String> = row.get(6)?;
                let created_at: Option<String> = row.get(7)?;

                let id = Uuid::parse_str(&id_str).unwrap_or_default();
                let previous_value = prev_json.and_then(|s| serde_json::from_str(&s).ok());
                let proposed_value = new_json.and_then(|s| serde_json::from_str(&s).ok()).unwrap_or(serde_json::Value::Null);

                Ok(PendingPatchDto {
                    id,
                    draft_session_id,
                    target_code,
                    field_path: field_path.unwrap_or_default(),
                    operation: op,
                    previous_value,
                    proposed_value,
                    status: st.unwrap_or_else(|| "pending".to_string()),
                    reason: None,
                    created_at: created_at.unwrap_or_default(),
                })
            })
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

        for pr in p_rows {
            if let Ok(dto) = pr {
                ws.pending_patches.push(dto);
            }
        }

        Ok(self.registry.get_or_register(draft_session_id, ws))
    }
}
