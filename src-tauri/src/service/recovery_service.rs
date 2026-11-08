//! Deterministic Crash & Startup Recovery Service (REQ-ARCH-002, REQ-OPS-001, be-core.md §11, T-REC-01).
//!
//! Provides deterministic crash recovery during startup and session reloading:
//! 1. Scenario 1 (Interrupted Draft Commits): Resets `status = 'committing'` sessions to `'editing'`.
//! 2. Scenario 2 (Interrupted Turn / Ambiguous Sealing):
//!    - Loads working state strictly from latest sealed `draft_versions`. Never replays unsealed tool calls.
//!    - Detects unsealed turns with applied tool writes, raising `interruptedTurnAlert = true`.
//!    - Lazy-expires pending patches whose target code is absent in restored working state.
//! 3. Scenario 3 & 5 (Corrupt Evidence / Workflow):
//!    - Corrupt or stale evidence receipts transition workflow to `RequiresReplan`, NEVER `ValidCompleted` or auto-skipped.
//! 4. Scenario 4 (Temporary Cache Hygiene):
//!    - Scans `temp_root` and wipes temporary report files whose fingerprint does not match the sealed version.
//! 5. Scenario 7 (Ambiguous Save Chapter):
//!    - Recovers durable receipts via `chapter_save_receipts`.

use std::fs;
use std::path::Path;
use std::sync::Arc;

use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use crate::domain::business_flow::BusinessFlowData;
use crate::domain::fingerprint::state_data_hash;
use crate::domain::sf06::codec::decode_evidence;
use crate::error::{ErrorCode, IpcError};
use crate::ipc::dto::SaveChapterResponse;
use crate::service::draft_runtime::{DraftRuntimeRegistry, DraftRuntimeStatus, WorkingState};
use crate::storage::db::DbPool;

/// Information about a single draft session processed during startup recovery.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoveredSessionInfo {
    pub session_id: String,
    pub draft_session_id: Uuid,
    pub status: String,
    pub head_version_seq: Option<i64>,
    pub interrupted_turn_alert: bool,
    pub expired_patches_count: usize,
}

/// Comprehensive startup recovery audit report.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StartupRecoveryReport {
    pub reset_committing_sessions_count: usize,
    pub interrupted_turns_detected: usize,
    pub cleaned_temp_files_count: usize,
    pub recovered_sessions: Vec<RecoveredSessionInfo>,
}

/// Workflow recovery decision for historical evidence verification (P6S2-N, phase6_sprint2_deep0-F2).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkflowRecoveryDecision {
    RequiresReplan { reason: String },
    ValidEvidence,
}

/// Validates historical evidence for a workflow run, ensuring corrupt or stale evidence
/// transitions the workflow to `RequiresReplan`, NEVER `ValidCompleted` or auto-skipped.
pub fn evaluate_evidence_integrity(
    evidence_json: Option<&str>,
    expected_fingerprint: Option<&str>,
) -> WorkflowRecoveryDecision {
    let evidence_str = match evidence_json {
        Some(s) if !s.trim().is_empty() => s.trim(),
        _ => {
            return WorkflowRecoveryDecision::RequiresReplan {
                reason: "insufficient_task_evidence".to_string(),
            };
        }
    };

    let parsed: Value = match serde_json::from_str(evidence_str) {
        Ok(v) => v,
        Err(_) => {
            return WorkflowRecoveryDecision::RequiresReplan {
                reason: "corrupt_evidence_json".to_string(),
            };
        }
    };

    if decode_evidence(&parsed).is_err() {
        return WorkflowRecoveryDecision::RequiresReplan {
            reason: "invalid_evidence_codec".to_string(),
        };
    }

    if let Some(expected) = expected_fingerprint {
        let extracted_fingerprint = parsed
            .get("fingerprint")
            .or_else(|| parsed.get("output_scope_fingerprint"))
            .or_else(|| parsed.get("data_hash"))
            .and_then(|v| v.as_str());

        if let Some(actual) = extracted_fingerprint {
            if actual != expected {
                return WorkflowRecoveryDecision::RequiresReplan {
                    reason: "evidence_fingerprint_mismatch".to_string(),
                };
            }
        }
    }

    WorkflowRecoveryDecision::ValidEvidence
}

/// Helper recovering ambiguous Save Chapter response via `chapter_save_receipts` (§11.2.7, SF-02).
pub fn recover_interrupted_chapter_save(
    conn: &Connection,
    chapter_id: Uuid,
    client_save_id: &str,
) -> Result<Option<SaveChapterResponse>, IpcError> {
    let mut stmt = conn
        .prepare("SELECT response_json FROM chapter_save_receipts WHERE chapter_id = ?1 AND client_save_id = ?2")
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

    let receipt_json: Option<String> = stmt
        .query_row(params![chapter_id.to_string(), client_save_id], |row| {
            row.get(0)
        })
        .ok();

    if let Some(json_str) = receipt_json {
        let resp: SaveChapterResponse = serde_json::from_str(&json_str)
            .map_err(|e| IpcError::new(ErrorCode::InternalError, e.to_string()))?;
        Ok(Some(resp))
    } else {
        Ok(None)
    }
}

/// Executes primary startup and crash recovery procedure (REQ-ARCH-002, REQ-OPS-001, be-core.md §11, T-REC-01).
pub fn run_startup_recovery(
    pool: &DbPool,
    temp_root: &Path,
    registry: Option<&Arc<DraftRuntimeRegistry>>,
) -> Result<StartupRecoveryReport, IpcError> {
    let conn = pool
        .get()
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

    let mut report = StartupRecoveryReport::default();

    // ========================================================================
    // Step 1: Interrupted Draft Commits (§11.2.1)
    // ========================================================================
    let now = chrono::Utc::now().to_rfc3339();
    let reset_count = conn
        .execute(
            "UPDATE draft_sessions
             SET status = 'editing', updated_at = ?1
             WHERE status = 'committing'",
            params![now],
        )
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
    report.reset_committing_sessions_count = reset_count;

    // ========================================================================
    // Step 2 & 3: Working State Restoration, Interrupted Turn Detection & Patch Expiration (§11.2.2)
    // ========================================================================
    let mut session_stmt = conn
        .prepare("SELECT id, session_id, head_version_seq, status FROM draft_sessions")
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

    let session_rows = session_stmt
        .query_map([], |row| {
            let id_str: String = row.get(0)?;
            let session_id: String = row.get(1)?;
            let head_seq: Option<i64> = row.get(2)?;
            let status: String = row.get(3)?;
            Ok((id_str, session_id, head_seq, status))
        })
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

    let mut valid_fingerprints: std::collections::HashSet<String> =
        std::collections::HashSet::new();

    for row_res in session_rows {
        let (id_str, session_id, head_seq, status_str) =
            row_res.map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

        let draft_session_id = Uuid::parse_str(&id_str)
            .map_err(|e| IpcError::new(ErrorCode::InternalError, e.to_string()))?;

        let mut restored_bfd: Option<BusinessFlowData> = None;

        // Restore WorkingState strictly from latest sealed version
        if let Some(seq) = head_seq {
            let mut v_stmt = conn
                .prepare(
                    "SELECT data_json, data_hash, phase, subphase FROM draft_versions
                     WHERE draft_session_id = ?1 AND seq = ?2",
                )
                .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

            let mut v_rows = v_stmt
                .query(params![draft_session_id.to_string(), seq])
                .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

            if let Some(v_row) = v_rows
                .next()
                .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?
            {
                let data_json: String = v_row.get(0).unwrap_or_default();
                let data_hash: Option<String> = v_row.get(1).ok();
                if let Some(dh) = data_hash {
                    if !dh.is_empty() {
                        valid_fingerprints.insert(dh);
                    }
                }
                if let Ok(bfd) = BusinessFlowData::from_json_str_strict(&data_json) {
                    if let Ok(fp) = state_data_hash(&bfd) {
                        valid_fingerprints.insert(fp);
                    }
                    restored_bfd = Some(bfd);
                }
            }
        }

        // Detect unsealed messages with applied tool writes (interrupted turn detection)
        let mut msg_stmt = conn
            .prepare(
                "SELECT DISTINCT m.turn_id, m.payload_json, m.role
                 FROM messages m
                 WHERE m.session_id = ?1 AND m.turn_id IS NOT NULL",
            )
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

        let msg_rows = msg_stmt
            .query_map(params![session_id], |row| {
                let turn_id_str: Option<String> = row.get(0)?;
                let payload_json: Option<String> = row.get(1)?;
                let role: String = row.get(2)?;
                Ok((turn_id_str, payload_json, role))
            })
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

        let mut has_interrupted_turn = false;

        for m in msg_rows {
            let (turn_id_str, payload_json, role) =
                m.map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

            let is_applied_write = if role == "tool" {
                if let Some(p_str) = payload_json.as_deref() {
                    if let Ok(val) = serde_json::from_str::<Value>(p_str) {
                        let writes_draft = val
                            .get("writes_draft_state")
                            .and_then(|v| v.as_bool().or_else(|| v.as_i64().map(|n| n == 1)))
                            .unwrap_or(false);
                        let has_applied = val
                            .get("has_applied_write")
                            .and_then(|v| v.as_bool().or_else(|| v.as_i64().map(|n| n == 1)))
                            .unwrap_or(false);
                        let status_applied = val
                            .get("status")
                            .and_then(|v| v.as_str())
                            .map(|s| s == "applied")
                            .unwrap_or(false);

                        (writes_draft && has_applied) || status_applied
                    } else {
                        false
                    }
                } else {
                    false
                }
            } else {
                false
            };

            if is_applied_write {
                if let Some(turn_str) = turn_id_str {
                    let mut check_v_stmt = conn
                        .prepare(
                            "SELECT 1 FROM draft_versions
                             WHERE draft_session_id = ?1 AND turn_id = ?2",
                        )
                        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

                    let version_exists = check_v_stmt
                        .exists(params![draft_session_id.to_string(), turn_str])
                        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

                    if !version_exists {
                        has_interrupted_turn = true;
                        break;
                    }
                }
            }
        }

        if has_interrupted_turn {
            report.interrupted_turns_detected += 1;
        }

        // Lazy-expire pending patches targeting non-existent elements
        let mut expired_patches_count = 0;
        if let Some(ref bfd) = restored_bfd {
            let bfd_json = serde_json::to_string(bfd).unwrap_or_default();

            let mut patch_stmt = conn
                .prepare(
                    "SELECT id, target_code FROM pending_patches
                     WHERE draft_session_id = ?1 AND status = 'pending'",
                )
                .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

            let patch_rows = patch_stmt
                .query_map(params![draft_session_id.to_string()], |row| {
                    let patch_id: String = row.get(0)?;
                    let target_code: Option<String> = row.get(1)?;
                    Ok((patch_id, target_code))
                })
                .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

            for p_res in patch_rows {
                let (patch_id, target_code) =
                    p_res.map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

                let is_valid_target = if let Some(code) = target_code {
                    bfd_json.contains(&code)
                } else {
                    true
                };

                if !is_valid_target {
                    conn.execute(
                        "UPDATE pending_patches SET status = 'expired', updated_at = ?1 WHERE id = ?2",
                        params![now, patch_id],
                    )
                    .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
                    expired_patches_count += 1;
                }
            }
        }

        // Populate in-memory registry if provided
        if let Some(reg) = registry {
            let mut ws = WorkingState::new(draft_session_id);
            ws.status = match status_str.as_str() {
                "committed" => DraftRuntimeStatus::Committed,
                "committing" => DraftRuntimeStatus::Editing,
                _ => DraftRuntimeStatus::Editing,
            };
            ws.head_version_seq = head_seq.map(|s| s as i32);
            if let Some(bfd) = restored_bfd {
                ws.business_flow_data = bfd;
            }
            reg.get_or_register(draft_session_id, ws);
        }

        report.recovered_sessions.push(RecoveredSessionInfo {
            session_id,
            draft_session_id,
            status: status_str,
            head_version_seq: head_seq,
            interrupted_turn_alert: has_interrupted_turn,
            expired_patches_count,
        });
    }

    // ========================================================================
    // Step 4: Temporary Report Cache Cleanup (§11.2.5, T-REC-01)
    // ========================================================================
    if temp_root.exists() {
        let mut files_to_clean = Vec::new();
        collect_temp_files(temp_root, &mut files_to_clean);

        for file_path in files_to_clean {
            let mut should_delete = false;

            if let Ok(content) = fs::read_to_string(&file_path) {
                if let Ok(val) = serde_json::from_str::<Value>(&content) {
                    let fp = val
                        .get("fingerprint")
                        .or_else(|| val.get("data_hash"))
                        .or_else(|| val.get("sealed_fingerprint"))
                        .and_then(|v| v.as_str());

                    if let Some(f) = fp {
                        let matches_any = valid_fingerprints.contains(f);
                        if !matches_any {
                            should_delete = true;
                        }
                    }
                } else if content.contains("fingerprint") {
                    let matches_any = valid_fingerprints
                        .iter()
                        .any(|vfp| content.contains(vfp));
                    if !matches_any {
                        should_delete = true;
                    }
                } else {
                    let file_name = file_path.file_name().and_then(|s| s.to_str()).unwrap_or("");
                    if file_name.starts_with("task_")
                        || file_name.contains("report")
                        || file_name.contains("mismatched")
                    {
                        should_delete = true;
                    }
                }
            } else {
                should_delete = true;
            }

            if should_delete {
                if let Ok(()) = fs::remove_file(&file_path) {
                    report.cleaned_temp_files_count += 1;
                }
            }
        }
    }

    Ok(report)
}

fn collect_temp_files(dir: &Path, acc: &mut Vec<std::path::PathBuf>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                acc.push(path);
            } else if path.is_dir() {
                collect_temp_files(&path, acc);
            }
        }
    }
}
