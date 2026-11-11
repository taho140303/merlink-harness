//! Atomically seal B+ Workflow Round in TX-04 (`REQ-SF06-003`, `REQ-TX-004`, `be-core.md` §502, §6.4.5, §1510–§1535).
//!
//! Enforces:
//! 1. Single `BEGIN IMMEDIATE ... COMMIT` transaction boundary for all mutations.
//! 2. Round header inserted into `workflow_round_receipts` with canonical digests.
//! 3. Task receipts and evidence inserted into `workflow_task_receipts`.
//! 4. Insert-Once semantics: strictly `INSERT ... ON CONFLICT DO NOTHING`, absolutely never `INSERT OR REPLACE` (Q3.2, Q3.3).
//! 5. Retry Before Version Creation: look up existing receipts by `(draft_session_id, workflow_run_id, plan_generation, task_identity)`.
//! 6. Max 1 Draft Version created only on net-round state mutation (`net_round_state_changed == true`).
//!    Zero-net mutation rounds persist round header and receipts with `had_mutations = 0, resulting_version_seq = NULL`
//!    WITHOUT creating a fake `draft_versions` record.
//! 7. Pure WorkingState replay: strictly excludes `pending_patches` rows from sealed version state.
//! 8. Monotonic head version sequence update on `draft_sessions` when a new version is created.

use chrono::Utc;
use rusqlite::{params, Connection, OptionalExtension, TransactionBehavior};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use crate::domain::business_flow::BusinessFlowData;
use crate::domain::fingerprint::state_data_hash;
use crate::domain::sf06::codec::{
    decode_evidence, replay, replay_evidence, EvidenceJsonV2, PatchOp, PatchV2, CODEC_VERSION_V2,
    SCHEMA_VERSION_V2,
};
use crate::domain::sf06::manifest::{
    compute_plan_digest, compute_request_digest, ManifestTask, PlanDigestInput, RequestDigestInput,
};
use crate::error::{ErrorCode, IpcError};

/// Specification of a planned task within a workflow round.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlannedTaskSpec {
    pub phase: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subphase: Option<i32>,
    #[serde(rename = "taskKind", alias = "kind")]
    pub task_kind: String,
    #[serde(rename = "assignedScope")]
    pub assigned_scope: Value,
    #[serde(default)]
    pub dependencies: Vec<String>,
    #[serde(
        rename = "taskIdentity",
        alias = "deterministicIdentity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub deterministic_identity: Option<String>,
}

/// Computes the canonical scope identity string for a planned task specification.
pub fn canonical_task_identity(spec: &PlannedTaskSpec) -> String {
    if let Some(ref id) = spec.deterministic_identity {
        let trimmed = id.trim();
        if !trimmed.is_empty() {
            return trimmed.to_string();
        }
    }
    if let Some(sub) = spec.subphase {
        format!(
            "task:p{}_s{}:{}:{}",
            spec.phase, sub, spec.task_kind, spec.assigned_scope
        )
    } else {
        format!(
            "task:p{}:{}:{}",
            spec.phase, spec.task_kind, spec.assigned_scope
        )
    }
}

/// An executed task outcome submitted for round sealing.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompletedTaskOutcome {
    pub task_index: usize,
    pub task_spec: PlannedTaskSpec,
    pub task_identity: String,
    pub input_scope_fingerprint: String,
    pub output_scope_fingerprint: String,
    pub has_applied_write: bool,
    pub applied_write_count: u32,
    #[serde(default)]
    pub applied_mutations: Vec<PatchOp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub artifact: Option<Value>,
}

/// Authorized plan context for verifying round integrity before mutation (`be-core.md` §6.4.5).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorizedRoundPlanContext {
    pub workflow_run_id: String,
    pub plan_generation: i64,
    pub phase: i32,
    pub subphase: Option<i32>,
    pub round_index: i64,
    pub parallel_limit: i64,
    pub authorized_tasks: std::collections::BTreeMap<usize, PlannedTaskSpec>,
}

/// Request payload to seal a workflow round in TX-04.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowRoundSealRequest {
    pub draft_session_id: String,
    pub workflow_run_id: String,
    pub plan_generation: i64,
    pub round_index: i64,
    pub round_seal_key: String,
    pub phase: i32,
    pub subphase: Option<i32>,
    pub parallel_limit: i64,
    pub base_version_seq: i64,
    pub base_data_hash: String,
    pub completed_tasks: Vec<CompletedTaskOutcome>,
    pub has_applied_write: bool,
}

/// Mutation and execution telemetry for an individual task.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskMutationTelemetry {
    pub task_identity: String,
    pub has_applied_write: bool,
    pub applied_write_count: u32,
    pub effective_task_state_changed: bool,
    pub persisted_had_mutations: bool,
    pub resulting_version_seq: Option<i64>,
}

/// Durable receipt row for a workflow task in `workflow_task_receipts`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowTaskReceiptRow {
    pub id: String,
    pub draft_session_id: String,
    pub workflow_run_id: String,
    pub plan_generation: i64,
    pub task_identity: String,
    pub input_scope_fingerprint: String,
    pub output_scope_fingerprint: String,
    pub base_version_seq: i64,
    pub resulting_version_seq: Option<i64>,
    pub had_mutations: bool,
    pub round_receipt_id: Option<String>,
    pub task_index: Option<i64>,
    pub evidence_json: Option<String>,
    pub created_at: Option<String>,
}

/// Complete response from sealing a workflow round in TX-04.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowRoundSealResponse {
    pub round_receipt_id: Option<String>,
    pub sealed_version_seq: Option<i64>,
    pub receipts: Vec<WorkflowTaskReceiptRow>,
    pub task_telemetry: Vec<TaskMutationTelemetry>,
    pub any_applied_write: bool,
    pub net_round_state_changed: bool,
    pub durable_version_created: bool,
    pub was_idempotent_replay: bool,
}

/// Helper to validate round tasks before applying any mutations (`PATCH-02`, `be-core.md` §6.4.5).
pub fn validate_round_tasks_pre_mutation<'a>(
    req: &'a WorkflowRoundSealRequest,
    plan_ctx: &AuthorizedRoundPlanContext,
) -> Result<Vec<&'a CompletedTaskOutcome>, IpcError> {
    // 1. Validate workflow run, plan generation, phase/subphase, and round index metadata
    if !req.workflow_run_id.trim().eq_ignore_ascii_case(plan_ctx.workflow_run_id.trim())
        || req.plan_generation != plan_ctx.plan_generation
        || req.phase != plan_ctx.phase
        || req.subphase != plan_ctx.subphase
        || req.round_index != plan_ctx.round_index
    {
        return Err(IpcError::new(
            ErrorCode::ValidationFailed,
            "Workflow round seal request metadata does not match the active authorized plan context",
        ));
    }

    if (req.phase == 5 && !matches!(req.subphase, Some(1..=3)))
        || (req.phase != 5 && req.subphase.is_some())
    {
        return Err(IpcError::new(
            ErrorCode::ValidationFailed,
            "Invalid phase/subphase combination in workflow round request",
        ));
    }

    // WF-01: Empty completed tasks handling
    if req.completed_tasks.is_empty() {
        if req.has_applied_write {
            return Err(IpcError::new(
                ErrorCode::ValidationFailed,
                "has_applied_write is true but completed_tasks is empty",
            ));
        }
        return Ok(vec![]);
    }

    // 2. Global uniqueness of task_identity and task_index using HashSet (WF-C05, WF-C06, WF-C07)
    let mut seen_identities: std::collections::HashSet<String> =
        std::collections::HashSet::with_capacity(req.completed_tasks.len());
    let mut seen_indices: std::collections::HashSet<usize> =
        std::collections::HashSet::with_capacity(req.completed_tasks.len());

    for task in &req.completed_tasks {
        let trimmed_id = task.task_identity.trim();
        if trimmed_id.is_empty() {
            return Err(IpcError::new(
                ErrorCode::ValidationFailed,
                "Missing or empty task_identity in completed_tasks",
            ));
        }

        if !seen_identities.insert(task.task_identity.clone()) {
            return Err(IpcError::new(
                ErrorCode::ValidationFailed,
                format!(
                    "Duplicate task_identity '{}' detected in completed_tasks",
                    task.task_identity
                ),
            ));
        }

        if !seen_indices.insert(task.task_index) {
            return Err(IpcError::new(
                ErrorCode::ValidationFailed,
                format!(
                    "Duplicate task_index {} detected in completed_tasks",
                    task.task_index
                ),
            ));
        }

        // 3. Phase and subphase must match round request
        if task.task_spec.phase != req.phase || task.task_spec.subphase != req.subphase {
            return Err(IpcError::new(
                ErrorCode::ValidationFailed,
                format!(
                    "Task '{}' phase/subphase does not match round phase/subphase",
                    task.task_identity
                ),
            ));
        }

        let expected_canonical_id = canonical_task_identity(&task.task_spec);
        if task.task_identity != expected_canonical_id {
            return Err(IpcError::new(
                ErrorCode::ValidationFailed,
                format!(
                    "Task identity '{}' does not match canonical scope identity '{}'",
                    task.task_identity, expected_canonical_id
                ),
            ));
        }

        // 4. Authorized task check
        match plan_ctx.authorized_tasks.get(&task.task_index) {
            Some(authorized_spec)
                if canonical_task_identity(authorized_spec) == task.task_identity => {}
            _ => {
                return Err(IpcError::new(
                    ErrorCode::ValidationFailed,
                    format!(
                        "Task (index={}, id='{}') is outside the currently authorized plan for round {}",
                        task.task_index, task.task_identity, req.round_index
                    ),
                ));
            }
        }

        // 5. Inconsistency between has_applied_write, applied_write_count and applied_mutations
        if task.has_applied_write
            != (!task.applied_mutations.is_empty() && task.applied_write_count > 0)
        {
            return Err(IpcError::new(
                ErrorCode::ValidationFailed,
                format!(
                    "Inconsistent applied mutation flags on task '{}'",
                    task.task_identity
                ),
            ));
        }
    }

    // 6. Preserve deterministic execution order: sort by task_index ASC
    let mut ordered_tasks: Vec<&'a CompletedTaskOutcome> = req.completed_tasks.iter().collect();
    ordered_tasks.sort_by_key(|t| t.task_index);

    Ok(ordered_tasks)
}

/// Executes TX-04 transaction boundary to seal a workflow round.
#[allow(clippy::type_complexity)]
pub fn seal_workflow_round_tx(
    conn: &mut Connection,
    req: &WorkflowRoundSealRequest,
    plan_ctx: &AuthorizedRoundPlanContext,
) -> Result<WorkflowRoundSealResponse, IpcError> {
    // 1. Pre-mutation validation
    let tasks = validate_round_tasks_pre_mutation(req, plan_ctx)?;

    // WF-01: Empty completed tasks without applied writes returns immediately
    if tasks.is_empty() {
        return Ok(WorkflowRoundSealResponse {
            round_receipt_id: None,
            sealed_version_seq: None,
            receipts: Vec::new(),
            task_telemetry: Vec::new(),
            any_applied_write: false,
            net_round_state_changed: false,
            durable_version_created: false,
            was_idempotent_replay: false,
        });
    }

    // 2. Open single atomic BEGIN IMMEDIATE ... COMMIT boundary
    let tx = conn
        .transaction_with_behavior(TransactionBehavior::Immediate)
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

    // 3. Lookup existing round header (Retry Before Version Creation & Idempotent Replay)
    let existing_round_opt: Option<(String, String, i64, String, String, Option<i64>)> = tx
        .query_row(
            "SELECT id, round_seal_key, base_version_seq, base_data_hash, final_data_hash, resulting_version_seq
             FROM workflow_round_receipts
             WHERE draft_session_id = ?1 AND (round_seal_key = ?2 OR (workflow_run_id = ?3 AND plan_generation = ?4 AND round_index = ?5))",
            params![
                req.draft_session_id,
                req.round_seal_key,
                req.workflow_run_id,
                req.plan_generation,
                req.round_index,
            ],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?, row.get(5)?)),
        )
        .optional()
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

    if let Some((
        existing_round_id,
        existing_seal_key,
        existing_base_seq,
        existing_base_hash,
        _existing_final_hash,
        existing_res_seq,
    )) = existing_round_opt
    {
        // Conflict detection: if seal key or base version mismatches, reject
        if existing_seal_key != req.round_seal_key {
            return Err(IpcError::new(
                ErrorCode::SealKeyPayloadMismatch,
                format!(
                    "Existing round has conflicting seal key '{}' != requested '{}'",
                    existing_seal_key, req.round_seal_key
                ),
            ));
        }

        if existing_base_seq != req.base_version_seq || existing_base_hash != req.base_data_hash {
            return Err(IpcError::new(
                ErrorCode::SealKeyPayloadMismatch,
                "Existing round header base_version_seq or base_data_hash does not match retry request",
            ));
        }

        // Query existing child receipts for idempotent replay
        let mut child_stmt = tx
            .prepare(
                "SELECT id, draft_session_id, workflow_run_id, plan_generation, task_identity,
                        input_scope_fingerprint, output_scope_fingerprint, base_version_seq,
                        resulting_version_seq, had_mutations, round_receipt_id, task_index,
                        evidence_json, created_at
                 FROM workflow_task_receipts
                 WHERE round_receipt_id = ?1
                 ORDER BY task_index ASC",
            )
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

        let receipts = child_stmt
            .query_map(params![existing_round_id], |row| {
                let had_mut_int: i64 = row.get(9)?;
                Ok(WorkflowTaskReceiptRow {
                    id: row.get(0)?,
                    draft_session_id: row.get(1)?,
                    workflow_run_id: row.get(2)?,
                    plan_generation: row.get(3)?,
                    task_identity: row.get(4)?,
                    input_scope_fingerprint: row.get(5)?,
                    output_scope_fingerprint: row.get(6)?,
                    base_version_seq: row.get(7)?,
                    resulting_version_seq: row.get(8)?,
                    had_mutations: had_mut_int == 1,
                    round_receipt_id: row.get(10)?,
                    task_index: row.get(11)?,
                    evidence_json: row.get(12)?,
                    created_at: row.get(13)?,
                })
            })
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

        let telemetry: Vec<TaskMutationTelemetry> = receipts
            .iter()
            .map(|r| TaskMutationTelemetry {
                task_identity: r.task_identity.clone(),
                has_applied_write: r.had_mutations,
                applied_write_count: if r.had_mutations { 1 } else { 0 },
                effective_task_state_changed: r.had_mutations,
                persisted_had_mutations: r.had_mutations,
                resulting_version_seq: r.resulting_version_seq,
            })
            .collect();

        // Idempotent replay: zero new writes, returns historical committed outcome
        return Ok(WorkflowRoundSealResponse {
            round_receipt_id: Some(existing_round_id),
            sealed_version_seq: existing_res_seq,
            receipts,
            task_telemetry: telemetry,
            any_applied_write: req.has_applied_write,
            net_round_state_changed: existing_res_seq.is_some(),
            durable_version_created: false,
            was_idempotent_replay: true,
        });
    }

    // 4. Check existing task receipts and reconcile prefix/suffix (Approach B - Task-level reconciliation)
    let mut task_has_existing_receipt = Vec::with_capacity(tasks.len());
    let mut existing_receipt_rows: Vec<WorkflowTaskReceiptRow> = Vec::new();

    for task in &tasks {
        let existing_task_row: Option<(
            String,
            String,
            String,
            i64,
            Option<i64>,
            Option<String>,
            Option<i64>,
            Option<String>,
            Option<String>,
        )> = tx
            .query_row(
                "SELECT id, input_scope_fingerprint, output_scope_fingerprint, had_mutations,
                        resulting_version_seq, round_receipt_id, task_index, evidence_json, created_at
                 FROM workflow_task_receipts
                 WHERE draft_session_id = ?1 AND workflow_run_id = ?2 AND plan_generation = ?3 AND task_identity = ?4",
                params![
                    req.draft_session_id,
                    req.workflow_run_id,
                    req.plan_generation,
                    task.task_identity,
                ],
                |row| {
                    Ok((
                        row.get(0)?,
                        row.get(1)?,
                        row.get(2)?,
                        row.get(3)?,
                        row.get(4)?,
                        row.get(5)?,
                        row.get(6)?,
                        row.get(7)?,
                        row.get(8)?,
                    ))
                },
            )
            .optional()
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

        if let Some((
            existing_id,
            existing_in_fp,
            existing_out_fp,
            had_mut_int,
            res_seq,
            round_rcpt_id,
            task_idx,
            ev_json,
            created,
        )) = existing_task_row
        {
            // WF-07, WF-C13: Conflicting receipt reuse with altered fingerprint rejected
            if existing_in_fp != task.input_scope_fingerprint
                || existing_out_fp != task.output_scope_fingerprint
            {
                return Err(IpcError::new(
                    ErrorCode::StaleWorkflowReceipt,
                    format!(
                        "Conflicting receipt for task '{}': stored fingerprints ({}, {}) mismatch request ({}, {})",
                        task.task_identity, existing_in_fp, existing_out_fp, task.input_scope_fingerprint, task.output_scope_fingerprint
                    ),
                ));
            }

            task_has_existing_receipt.push(true);
            existing_receipt_rows.push(WorkflowTaskReceiptRow {
                id: existing_id,
                draft_session_id: req.draft_session_id.clone(),
                workflow_run_id: req.workflow_run_id.clone(),
                plan_generation: req.plan_generation,
                task_identity: task.task_identity.clone(),
                input_scope_fingerprint: existing_in_fp,
                output_scope_fingerprint: existing_out_fp,
                base_version_seq: req.base_version_seq,
                resulting_version_seq: res_seq,
                had_mutations: had_mut_int == 1,
                round_receipt_id: round_rcpt_id,
                task_index: task_idx,
                evidence_json: ev_json,
                created_at: created,
            });
        } else {
            task_has_existing_receipt.push(false);
        }
    }

    // Verify prefix structure (WF-06, REC-11, REC-12):
    // Existing receipts must form a contiguous prefix 0..k-1.
    // An existing receipt cannot follow a new task.
    let mut seen_new_task = false;
    for (i, has_existing) in task_has_existing_receipt.iter().enumerate() {
        if *has_existing {
            if seen_new_task {
                return Err(IpcError::new(
                    ErrorCode::ValidationFailed,
                    format!(
                        "Interleaved task structure (REC-12): task at index {} has an existing receipt after new tasks",
                        i
                    ),
                ));
            }
        } else {
            seen_new_task = true;
        }
    }

    // 5. Verify draft session head version and baseline version
    let session_head_seq: i64 = tx
        .query_row(
            "SELECT head_version_seq FROM draft_sessions WHERE id = ?1",
            params![req.draft_session_id],
            |row| row.get(0),
        )
        .map_err(|e| {
            IpcError::new(
                ErrorCode::EntityNotFound,
                format!("Draft session '{}' not found: {}", req.draft_session_id, e),
            )
        })?;

    // Only genuinely new round demands head == sealed baseline
    if seen_new_task && session_head_seq != req.base_version_seq {
        return Err(IpcError::new(
            ErrorCode::ValidationFailed,
            format!(
                "Draft session head version {} does not match round base version {}",
                session_head_seq, req.base_version_seq
            ),
        ));
    }

    // Load S0 baseline version from draft_versions
    let (s0_data_json, s0_data_hash): (String, String) = tx
        .query_row(
            "SELECT data_json, data_hash FROM draft_versions WHERE draft_session_id = ?1 AND seq = ?2",
            params![req.draft_session_id, req.base_version_seq],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .map_err(|e| {
            IpcError::new(
                ErrorCode::EntityNotFound,
                format!(
                    "Base draft version seq {} not found for session '{}': {}",
                    req.base_version_seq, req.draft_session_id, e
                ),
            )
        })?;

    if s0_data_hash != req.base_data_hash {
        return Err(IpcError::new(
            ErrorCode::ValidationFailed,
            format!(
                "Base data hash mismatch: stored '{}', request '{}'",
                s0_data_hash, req.base_data_hash
            ),
        ));
    }

    // Pure WorkingState: deserialize purely from S0 data_json (strictly excludes pending_patches)
    let s0 = BusinessFlowData::from_json_str_strict(&s0_data_json).map_err(|e| {
        IpcError::new(
            ErrorCode::ValidationFailed,
            format!("Failed to parse S0 BusinessFlowData: {e}"),
        )
    })?;

    // 6. Sequential Pure Replay & Fingerprint Verification
    let mut current_state = s0.clone();
    let mut current_hash = req.base_data_hash.clone();
    let mut task_telemetries = Vec::with_capacity(tasks.len());
    let mut new_task_evidences = Vec::with_capacity(tasks.len());
    let mut new_task_receipts = Vec::with_capacity(tasks.len());

    for (idx, task) in tasks.iter().enumerate() {
        let is_existing = task_has_existing_receipt[idx];

        if is_existing {
            // Replay from existing durable evidence if available, or from task applied mutations
            let existing_receipt = &existing_receipt_rows[idx];
            let mut replayed = false;
            if let Some(ref ev_str) = existing_receipt.evidence_json {
                if let Ok(ev_val) = serde_json::from_str::<Value>(ev_str) {
                    if let Ok(ev) = decode_evidence(&ev_val) {
                        if let Ok(next_state) = replay_evidence(&current_state, &ev) {
                            current_hash = ev.post_state_hash;
                            current_state = next_state;
                            replayed = true;
                        }
                    }
                }
            }

            if !replayed && !task.applied_mutations.is_empty() {
                let patch = PatchV2 {
                    ops: task.applied_mutations.clone(),
                };
                if let Ok(next_state) = replay(&current_state, &patch) {
                    if let Ok(post_hash) = state_data_hash(&next_state) {
                        current_state = next_state;
                        current_hash = post_hash;
                    }
                }
            }

            task_telemetries.push(TaskMutationTelemetry {
                task_identity: task.task_identity.clone(),
                has_applied_write: existing_receipt.had_mutations,
                applied_write_count: if existing_receipt.had_mutations { 1 } else { 0 },
                effective_task_state_changed: existing_receipt.had_mutations,
                persisted_had_mutations: existing_receipt.had_mutations,
                resulting_version_seq: existing_receipt.resulting_version_seq,
            });
        } else {
            // Apply new task's mutations
            let s_before = current_state.clone();
            let pre_hash = current_hash.clone();

            let patch = PatchV2 {
                ops: task.applied_mutations.clone(),
            };

            let s_after = replay(&s_before, &patch).map_err(|e| {
                IpcError::new(
                    e,
                    format!(
                        "Failed to apply mutations for task '{}' at index {}",
                        task.task_identity, idx
                    ),
                )
            })?;

            let post_hash = state_data_hash(&s_after).map_err(|e| {
                IpcError::new(
                    ErrorCode::ValidationFailed,
                    format!("Failed to compute state hash at task index {}: {}", idx, e),
                )
            })?;

            let effective_task_state_changed = pre_hash != post_hash;

            let evidence = EvidenceJsonV2 {
                schema_version: SCHEMA_VERSION_V2,
                codec_version: CODEC_VERSION_V2.to_string(),
                planner_version: "merlink.v2".to_string(),
                policy_version: "merlink.v2".to_string(),
                task_spec: serde_json::to_value(&task.task_spec).map_err(|e| {
                    IpcError::new(
                        ErrorCode::ValidationFailed,
                        format!("Failed to serialize task_spec at index {}: {}", idx, e),
                    )
                })?,
                pre_state_hash: pre_hash,
                post_state_hash: post_hash.clone(),
                had_applied_writes: task.has_applied_write,
                effective_state_changed: effective_task_state_changed,
                patch: task.applied_mutations.clone(),
                artifact: task.artifact.clone(),
            };

            current_state = s_after;
            current_hash = post_hash;

            new_task_evidences.push(evidence);
            new_task_receipts.push((idx, *task, effective_task_state_changed));

            task_telemetries.push(TaskMutationTelemetry {
                task_identity: task.task_identity.clone(),
                has_applied_write: task.has_applied_write,
                applied_write_count: task.applied_write_count,
                effective_task_state_changed,
                persisted_had_mutations: false, // Updated below after version decision
                resulting_version_seq: None,
            });
        }
    }

    let final_data_hash = current_hash;
    let net_round_state_changed = final_data_hash != req.base_data_hash;
    let now_iso = Utc::now().to_rfc3339();

    // 7. Draft Version Creation (Atomic Version Creation Decision)
    let resulting_version_seq: Option<i64>;

    if net_round_state_changed {
        let next_seq = session_head_seq + 1;
        resulting_version_seq = Some(next_seq);

        let new_version_id = Uuid::now_v7().to_string();
        let final_state_json = serde_json::to_string(&current_state).map_err(|e| {
            IpcError::new(
                ErrorCode::ValidationFailed,
                format!("Failed to serialize final WorkingState: {e}"),
            )
        })?;

        // Insert new draft_versions row BEFORE workflow_round_receipts (FK dependency)
        tx.execute(
            "INSERT INTO draft_versions (
                id, draft_session_id, seq, seal_key, phase, subphase,
                data_json, data_hash, reason, parent_seq, turn_id, chat_message_id, created_at
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
            params![
                new_version_id,
                req.draft_session_id,
                next_seq,
                req.round_seal_key,
                req.phase,
                req.subphase,
                final_state_json,
                final_data_hash,
                "workflow_round",
                req.base_version_seq,
                Option::<String>::None,
                Option::<String>::None,
                now_iso,
            ],
        )
        .map_err(|e| {
            IpcError::new(
                ErrorCode::DatabaseError,
                format!("Failed to insert draft_versions row: {e}"),
            )
        })?;

        // Monotonic head version update on draft_sessions
        tx.execute(
            "UPDATE draft_sessions SET head_version_seq = ?1, updated_at = ?2 WHERE id = ?3",
            params![next_seq, now_iso, req.draft_session_id],
        )
        .map_err(|e| {
            IpcError::new(
                ErrorCode::DatabaseError,
                format!("Failed to update draft_sessions head_version_seq: {e}"),
            )
        })?;
    } else {
        // ZERO-NET MUTATION ROUND:
        // Do NOT create draft_versions row.
        // Do NOT update draft_sessions head_version_seq.
        resulting_version_seq = None;
    }

    // 8. Compute plan_digest and request_digest
    let manifest_tasks: Vec<ManifestTask> = tasks
        .iter()
        .enumerate()
        .map(|(idx, task)| {
            ManifestTask::new(
                idx as u32,
                task.task_identity.clone(),
                serde_json::to_value(&task.task_spec).unwrap_or(Value::Null),
                "merlink.v2",
                "merlink.v2",
                CODEC_VERSION_V2,
            )
        })
        .collect();

    let plan_input = PlanDigestInput {
        draft_session_id: req.draft_session_id.clone(),
        workflow_run_id: req.workflow_run_id.clone(),
        plan_generation: req.plan_generation as i32,
        round_index: req.round_index as i32,
        task_count: tasks.len() as u32,
        parallel_limit: req.parallel_limit as u32,
        tasks: manifest_tasks,
    };

    let plan_digest = compute_plan_digest(&plan_input).map_err(|e| {
        IpcError::new(
            ErrorCode::ValidationFailed,
            format!("Failed to compute plan digest: {e}"),
        )
    })?;

    let req_input = RequestDigestInput {
        draft_session_id: req.draft_session_id.clone(),
        workflow_run_id: req.workflow_run_id.clone(),
        plan_generation: req.plan_generation as i32,
        round_index: req.round_index as i32,
        task_count: tasks.len() as u32,
        parallel_limit: req.parallel_limit as u32,
        plan_digest: plan_digest.clone(),
        base_version_seq: req.base_version_seq,
        base_data_hash: req.base_data_hash.clone(),
    };

    let request_digest = compute_request_digest(&req_input).map_err(|e| {
        IpcError::new(
            ErrorCode::ValidationFailed,
            format!("Failed to compute request digest: {e}"),
        )
    })?;

    // 9. Insert round header into workflow_round_receipts
    let round_receipt_id = Uuid::now_v7().to_string();

    tx.execute(
        "INSERT INTO workflow_round_receipts (
            id, draft_session_id, workflow_run_id, plan_generation, round_index,
            round_seal_key, task_count, parallel_limit, plan_digest, request_digest,
            base_version_seq, base_data_hash, final_data_hash, resulting_version_seq, created_at
         ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)",
        params![
            round_receipt_id,
            req.draft_session_id,
            req.workflow_run_id,
            req.plan_generation,
            req.round_index,
            req.round_seal_key,
            tasks.len() as i64,
            req.parallel_limit,
            plan_digest,
            request_digest,
            req.base_version_seq,
            req.base_data_hash,
            final_data_hash,
            resulting_version_seq,
            now_iso,
        ],
    )
    .map_err(|e| {
        IpcError::new(
            ErrorCode::DatabaseError,
            format!("Failed to insert workflow_round_receipts header: {e}"),
        )
    })?;

    // 10. Insert task receipts into workflow_task_receipts (Insert-Once Semantics - Q3.2, Q3.3)
    // STRICTLY INSERT ... ON CONFLICT DO NOTHING, absolutely never INSERT OR REPLACE!
    let mut all_receipts = existing_receipt_rows;

    for ((task_pos, task, effective_state_changed), evidence) in
        new_task_receipts.into_iter().zip(new_task_evidences)
    {
        let task_had_mutations = effective_state_changed && net_round_state_changed;
        let task_resulting_seq = if task_had_mutations {
            resulting_version_seq
        } else {
            None
        };

        let evidence_json_str = serde_json::to_string(&evidence).map_err(|e| {
            IpcError::new(
                ErrorCode::ValidationFailed,
                format!("Failed to serialize evidence_json: {e}"),
            )
        })?;

        let task_receipt_id = Uuid::now_v7().to_string();

        tx.execute(
            "INSERT INTO workflow_task_receipts (
                id, draft_session_id, workflow_run_id, plan_generation, task_identity,
                input_scope_fingerprint, output_scope_fingerprint, base_version_seq,
                resulting_version_seq, had_mutations, round_receipt_id, task_index,
                evidence_json, created_at
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)
             ON CONFLICT DO NOTHING",
            params![
                task_receipt_id,
                req.draft_session_id,
                req.workflow_run_id,
                req.plan_generation,
                task.task_identity,
                task.input_scope_fingerprint,
                task.output_scope_fingerprint,
                req.base_version_seq,
                task_resulting_seq,
                if task_had_mutations { 1 } else { 0 },
                round_receipt_id,
                task_pos as i64,
                evidence_json_str,
                now_iso,
            ],
        )
        .map_err(|e| {
            IpcError::new(
                ErrorCode::DatabaseError,
                format!("Failed to insert workflow_task_receipts row: {e}"),
            )
        })?;

        // Update telemetry
        if let Some(t) = task_telemetries
            .iter_mut()
            .find(|t| t.task_identity == task.task_identity)
        {
            t.persisted_had_mutations = task_had_mutations;
            t.resulting_version_seq = task_resulting_seq;
        }

        all_receipts.push(WorkflowTaskReceiptRow {
            id: task_receipt_id,
            draft_session_id: req.draft_session_id.clone(),
            workflow_run_id: req.workflow_run_id.clone(),
            plan_generation: req.plan_generation,
            task_identity: task.task_identity.clone(),
            input_scope_fingerprint: task.input_scope_fingerprint.clone(),
            output_scope_fingerprint: task.output_scope_fingerprint.clone(),
            base_version_seq: req.base_version_seq,
            resulting_version_seq: task_resulting_seq,
            had_mutations: task_had_mutations,
            round_receipt_id: Some(round_receipt_id.clone()),
            task_index: Some(task_pos as i64),
            evidence_json: Some(evidence_json_str),
            created_at: Some(now_iso.clone()),
        });
    }

    // 11. Atomic Commit TX-04
    tx.commit().map_err(|e| {
        IpcError::new(
            ErrorCode::DatabaseError,
            format!("Failed to commit TX-04: {e}"),
        )
    })?;

    Ok(WorkflowRoundSealResponse {
        round_receipt_id: Some(round_receipt_id),
        sealed_version_seq: resulting_version_seq,
        receipts: all_receipts,
        task_telemetry: task_telemetries,
        any_applied_write: req.has_applied_write,
        net_round_state_changed,
        durable_version_created: resulting_version_seq.is_some(),
        was_idempotent_replay: false,
    })
}
