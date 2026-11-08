//! Historical B+ Round Verifier (`REQ-SF06-002`, `REQ-SF06-004`, `be-core.md` §1534–§1549, `sf06-owner-ratification.md` §38–§42).
//!
//! Provides pure read-only verification of sealed historical workflow rounds from SQLite and RAM:
//! - Strictly read-only: no writes, no LLM provider calls, no external tools, no temporary store.
//! - Separates 3 orthogonal outcome dimensions:
//!   1. `Integrity`: `Verified` | `Corrupt` | `Indeterminate`
//!   2. `Attestation`: `Attested` | `Unknown`
//!   3. `Applicability`: `Applicable` | `RestoreDiverged` | `Inapplicable`

use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::domain::business_flow::BusinessFlowData;
use crate::domain::fingerprint::{state_data_hash, states_are_semantically_equal};
use crate::domain::sf06::codec::{decode_evidence, replay_evidence, EvidenceJsonV2};
use crate::domain::sf06::manifest::{
    compute_plan_digest, compute_request_digest, ManifestTask, PlanDigestInput, RequestDigestInput,
};

/// Dimensional outcome 1: Internal structural and cryptographic integrity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Integrity {
    /// Internal integrity fully verified: exact child membership, digests, pure patch replay, final hash.
    Verified,
    /// Integrity check failed: missing/extra child receipt, index gap, hash mismatch, corrupted payload.
    Corrupt,
    /// Cannot determine integrity: legacy receipt with NULL evidence, or unsupported codec version.
    Indeterminate,
}

/// Dimensional outcome 2: Authorization attestation under trusted backend writer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Attestation {
    /// Committed by authorized backend writer under verified protocol.
    Attested,
    /// Unknown or unverified authorization (e.g. legacy receipt, corrupted evidence).
    Unknown,
}

/// Type alias for authorization attestation per SSOT terminology.
pub type AuthorizationAttestation = Attestation;

/// Dimensional outcome 3: Applicability to the current workspace head version.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Applicability {
    /// Output state is applicable to or matches the current workspace head.
    Applicable,
    /// The round lies on an abandoned branch severed by a Restore operation (REC-10, GV9).
    RestoreDiverged,
    /// Inapplicable for other reasons (e.g. head has advanced unproven, baseline missing, corrupt).
    Inapplicable,
}

/// Complete 3-dimensional verification outcome.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistoricalRoundVerificationResult {
    pub integrity: Integrity,
    pub attestation: Attestation,
    pub authorization_attestation: Attestation,
    pub applicability: Applicability,
    pub details: Option<String>,
}

impl HistoricalRoundVerificationResult {
    /// Constructs a full verification outcome.
    pub fn new(
        integrity: Integrity,
        attestation: Attestation,
        applicability: Applicability,
        details: Option<String>,
    ) -> Self {
        Self {
            integrity,
            attestation,
            authorization_attestation: attestation,
            applicability,
            details,
        }
    }

    /// Helper for corrupted verification result.
    pub fn corrupt(reason: impl Into<String>) -> Self {
        Self::new(
            Integrity::Corrupt,
            Attestation::Unknown,
            Applicability::Inapplicable,
            Some(reason.into()),
        )
    }

    /// Helper for indeterminate verification result (e.g. legacy NULL evidence or unsupported codec).
    pub fn indeterminate(reason: impl Into<String>) -> Self {
        Self::new(
            Integrity::Indeterminate,
            Attestation::Unknown,
            Applicability::Inapplicable,
            Some(reason.into()),
        )
    }

    /// Returns true if internal integrity is verified.
    pub fn is_verified(&self) -> bool {
        self.integrity == Integrity::Verified
    }

    /// Returns true if attestation is attested.
    pub fn is_attested(&self) -> bool {
        self.attestation == Attestation::Attested
    }

    /// Returns true if applicability is applicable to current head.
    pub fn is_applicable(&self) -> bool {
        self.applicability == Applicability::Applicable
    }
}

/// Conversion helper trait allowing `current_head_seq` to be passed as `i64`, `i32`, `Option<i64>`, or `Option<i32>`.
pub trait IntoHeadSeq {
    fn into_head_seq(self) -> Option<i64>;
}

impl IntoHeadSeq for i64 {
    fn into_head_seq(self) -> Option<i64> {
        Some(self)
    }
}

impl IntoHeadSeq for i32 {
    fn into_head_seq(self) -> Option<i64> {
        Some(self as i64)
    }
}

impl IntoHeadSeq for Option<i64> {
    fn into_head_seq(self) -> Option<i64> {
        self
    }
}

impl IntoHeadSeq for Option<i32> {
    fn into_head_seq(self) -> Option<i64> {
        self.map(|s| s as i64)
    }
}

/// Internal struct representing a row from `workflow_round_receipts`.
#[derive(Debug)]
struct RoundHeaderRow {
    id: String,
    draft_session_id: String,
    workflow_run_id: String,
    plan_generation: i32,
    round_index: i32,
    round_seal_key: String,
    task_count: i32,
    parallel_limit: i32,
    plan_digest: String,
    request_digest: String,
    base_version_seq: i64,
    base_data_hash: String,
    final_data_hash: String,
    resulting_version_seq: Option<i64>,
}

/// Internal struct representing a row from `workflow_task_receipts`.
#[allow(dead_code)]
#[derive(Debug)]
struct TaskReceiptRow {
    id: String,
    draft_session_id: String,
    workflow_run_id: String,
    plan_generation: i32,
    task_identity: String,
    input_scope_fingerprint: String,
    output_scope_fingerprint: String,
    base_version_seq: i64,
    resulting_version_seq: Option<i64>,
    had_mutations: i32,
    round_receipt_id: Option<String>,
    task_index: Option<i32>,
    evidence_json: Option<String>,
}

/// Verifies a historical workflow round purely from SQLite and RAM (`REQ-SF06-002`, `REQ-SF06-004`).
///
/// Invariants enforced:
/// 1. Reads `workflow_round_receipts` by `round_receipt_id`.
/// 2. Verifies exact child task receipt set with `task_index` spanning `{0..task_count - 1}`.
///    Any gap, missing index, or extra child (N+1) fails closed as `Integrity::Corrupt`.
/// 3. Legacy child with `evidence_json IS NULL` returns `Integrity::Indeterminate` and `Attestation::Unknown`
///    without backfill or fake verification (`REC-02`, `P3S9-B`).
/// 4. Reconstructs manifest tasks from child evidence, recomputes `plan_digest` and `request_digest`,
///    and verifies byte-exact equality against header digests.
/// 5. Replays pure patches sequentially starting from S0 (`draft_versions.data_json` matching `base_data_hash`),
///    checking pre-state and post-state hashes at each step, and matching final state to `final_data_hash`.
/// 6. Verifies resulting `draft_versions` row (if non-null) matches final state, seal key, and reason.
/// 7. Checks current-head applicability: if round is on a branch severed by Restore, returns
///    `Applicability::RestoreDiverged` (`P3S9-C`, `phase3_sprint9_deep2-F2`, `REC-10`).
/// 8. Zero-net rounds (`resulting_version_seq IS NULL` and `base_data_hash == final_data_hash`) verify
///    all evidence patches deterministically after restart (`P3S9-P`, `REC-03`).
pub fn verify_historical_round(
    conn: &Connection,
    round_receipt_id: &str,
    supported_codecs: &[&str],
    current_head_seq: impl IntoHeadSeq,
) -> HistoricalRoundVerificationResult {
    // -------------------------------------------------------------------------
    // Step 1: Read and validate round header from workflow_round_receipts
    // -------------------------------------------------------------------------
    let header_opt: Option<RoundHeaderRow> = match conn.query_row(
        "SELECT id, draft_session_id, workflow_run_id, plan_generation, round_index, \
                round_seal_key, task_count, parallel_limit, plan_digest, request_digest, \
                base_version_seq, base_data_hash, final_data_hash, resulting_version_seq \
         FROM workflow_round_receipts WHERE id = ?1",
        params![round_receipt_id],
        |row| {
            Ok(RoundHeaderRow {
                id: row.get(0)?,
                draft_session_id: row.get(1)?,
                workflow_run_id: row.get(2)?,
                plan_generation: row.get(3)?,
                round_index: row.get(4)?,
                round_seal_key: row.get(5)?,
                task_count: row.get(6)?,
                parallel_limit: row.get(7)?,
                plan_digest: row.get(8)?,
                request_digest: row.get(9)?,
                base_version_seq: row.get(10)?,
                base_data_hash: row.get(11)?,
                final_data_hash: row.get(12)?,
                resulting_version_seq: row.get(13)?,
            })
        },
    ).optional() {
        Ok(opt) => opt,
        Err(e) => return HistoricalRoundVerificationResult::corrupt(format!("Failed to query round header: {e}")),
    };

    let header = match header_opt {
        Some(h) => h,
        None => return HistoricalRoundVerificationResult::corrupt("Round header not found in workflow_round_receipts"),
    };

    // Header field sanity checks
    if header.plan_generation <= 0 {
        return HistoricalRoundVerificationResult::corrupt("plan_generation must be > 0");
    }
    if header.round_index < 0 {
        return HistoricalRoundVerificationResult::corrupt("round_index must be >= 0");
    }
    if header.task_count <= 0 {
        return HistoricalRoundVerificationResult::corrupt("task_count must be >= 1");
    }
    if header.parallel_limit <= 0 {
        return HistoricalRoundVerificationResult::corrupt("parallel_limit must be >= 1");
    }
    if (header.task_count as u32) > (header.parallel_limit as u32) {
        return HistoricalRoundVerificationResult::corrupt("task_count exceeds frozen parallel_limit (N <= P violation)");
    }

    // Verify round_seal_key format: wf_round:<canonical UUID>:<generation>:<round_index>
    let expected_seal_key = format!(
        "wf_round:{}:{}:{}",
        header.workflow_run_id.trim().to_ascii_lowercase(),
        header.plan_generation,
        header.round_index
    );
    if header.round_seal_key.trim().to_ascii_lowercase() != expected_seal_key {
        return HistoricalRoundVerificationResult::corrupt(format!(
            "round_seal_key mismatch: expected '{}', found '{}'",
            expected_seal_key, header.round_seal_key
        ));
    }

    // Invariant: zero-net round has resulting_version_seq IS NULL AND base_data_hash == final_data_hash
    match header.resulting_version_seq {
        None => {
            if header.base_data_hash != header.final_data_hash {
                return HistoricalRoundVerificationResult::corrupt(
                    "Zero-net round (resulting_version_seq IS NULL) must have base_data_hash == final_data_hash",
                );
            }
        }
        Some(_) => {
            if header.base_data_hash == header.final_data_hash {
                return HistoricalRoundVerificationResult::corrupt(
                    "Mutating round (resulting_version_seq IS NOT NULL) must have base_data_hash <> final_data_hash",
                );
            }
        }
    }

    // -------------------------------------------------------------------------
    // Step 2: Retrieve and verify S0 base snapshot from draft_versions
    // -------------------------------------------------------------------------
    let s0_row: Option<(String, String)> = match conn.query_row(
        "SELECT data_json, data_hash FROM draft_versions WHERE draft_session_id = ?1 AND seq = ?2",
        params![header.draft_session_id, header.base_version_seq],
        |row| Ok((row.get(0)?, row.get(1)?)),
    ).optional() {
        Ok(opt) => opt,
        Err(e) => return HistoricalRoundVerificationResult::corrupt(format!("Failed to query S0 draft_version: {e}")),
    };

    let (s0_json, s0_stored_hash) = match s0_row {
        Some(pair) => pair,
        None => return HistoricalRoundVerificationResult::corrupt(format!(
            "Base draft version seq {} not found for session {}",
            header.base_version_seq, header.draft_session_id
        )),
    };

    let s0: BusinessFlowData = match serde_json::from_str(&s0_json) {
        Ok(data) => data,
        Err(e) => return HistoricalRoundVerificationResult::corrupt(format!("S0 data_json deserialization error: {e}")),
    };

    let s0_computed_hash = match state_data_hash(&s0) {
        Ok(h) => h,
        Err(e) => return HistoricalRoundVerificationResult::corrupt(format!("S0 state_data_hash computation error: {e}")),
    };

    if s0_computed_hash != header.base_data_hash || s0_computed_hash != s0_stored_hash {
        return HistoricalRoundVerificationResult::corrupt(format!(
            "S0 hash mismatch: computed '{}', header '{}', stored '{}'",
            s0_computed_hash, header.base_data_hash, s0_stored_hash
        ));
    }

    // -------------------------------------------------------------------------
    // Step 3: Retrieve child task receipts and verify exact index membership
    // -------------------------------------------------------------------------
    let mut stmt = match conn.prepare(
        "SELECT id, draft_session_id, workflow_run_id, plan_generation, task_identity, \
                input_scope_fingerprint, output_scope_fingerprint, base_version_seq, \
                resulting_version_seq, had_mutations, round_receipt_id, task_index, evidence_json \
         FROM workflow_task_receipts WHERE round_receipt_id = ?1 ORDER BY task_index ASC",
    ) {
        Ok(s) => s,
        Err(e) => return HistoricalRoundVerificationResult::corrupt(format!("Failed to prepare child receipts query: {e}")),
    };

    let child_rows_res: Result<Vec<TaskReceiptRow>, _> = stmt.query_map(params![header.id], |row| {
        Ok(TaskReceiptRow {
            id: row.get(0)?,
            draft_session_id: row.get(1)?,
            workflow_run_id: row.get(2)?,
            plan_generation: row.get(3)?,
            task_identity: row.get(4)?,
            input_scope_fingerprint: row.get(5)?,
            output_scope_fingerprint: row.get(6)?,
            base_version_seq: row.get(7)?,
            resulting_version_seq: row.get(8)?,
            had_mutations: row.get(9)?,
            round_receipt_id: row.get(10)?,
            task_index: row.get(11)?,
            evidence_json: row.get(12)?,
        })
    }).and_then(|mapped| mapped.collect());

    let child_rows = match child_rows_res {
        Ok(rows) => rows,
        Err(e) => return HistoricalRoundVerificationResult::corrupt(format!("Failed to read child receipts: {e}")),
    };

    // Exact child membership check: must have exactly header.task_count children
    if child_rows.len() != header.task_count as usize {
        return HistoricalRoundVerificationResult::corrupt(format!(
            "Task count mismatch (P3S9-N, F1): header specifies {} tasks, found {} in workflow_task_receipts",
            header.task_count,
            child_rows.len()
        ));
    }

    // Check strict contiguous index set {0..N-1}
    for (expected_idx, child) in child_rows.iter().enumerate() {
        match child.task_index {
            Some(idx) if idx == expected_idx as i32 => {}
            Some(idx) => {
                return HistoricalRoundVerificationResult::corrupt(format!(
                    "Task index gap/mismatch at position {}: expected index {}, found {}",
                    expected_idx, expected_idx, idx
                ));
            }
            None => {
                return HistoricalRoundVerificationResult::corrupt(format!(
                    "Task index is NULL at position {}",
                    expected_idx
                ));
            }
        }

        // Composite FK column checks
        if child.draft_session_id != header.draft_session_id
            || child.workflow_run_id != header.workflow_run_id
            || child.plan_generation != header.plan_generation
            || child.base_version_seq != header.base_version_seq
        {
            return HistoricalRoundVerificationResult::corrupt(format!(
                "Child task receipt composite attribute mismatch at index {}",
                expected_idx
            ));
        }
    }

    // -------------------------------------------------------------------------
    // Step 4: Validate evidence and extract manifest tasks
    // -------------------------------------------------------------------------
    let mut manifest_tasks: Vec<ManifestTask> = Vec::with_capacity(child_rows.len());
    let mut decoded_evidences: Vec<EvidenceJsonV2> = Vec::with_capacity(child_rows.len());

    for (idx, child) in child_rows.iter().enumerate() {
        let evidence_raw = match &child.evidence_json {
            Some(raw) if !raw.trim().is_empty() => raw,
            _ => {
                // Legacy child without evidence_json or NULL evidence fails closed as Indeterminate
                // (REC-02, P3S9-B): no backfill, no fake verification.
                return HistoricalRoundVerificationResult::indeterminate(format!(
                    "Legacy child task receipt at index {} has NULL evidence_json (REC-02, P3S9-B)",
                    idx
                ));
            }
        };

        let evidence_val: Value = match serde_json::from_str(evidence_raw) {
            Ok(v) => v,
            Err(e) => {
                return HistoricalRoundVerificationResult::corrupt(format!(
                    "Malformed evidence_json at index {}: {}",
                    idx, e
                ));
            }
        };

        // Codec compatibility check
        let codec_ver = evidence_val
            .get("codecVersion")
            .and_then(Value::as_str)
            .unwrap_or("");

        if !supported_codecs.contains(&codec_ver) {
            return HistoricalRoundVerificationResult::indeterminate(format!(
                "Unsupported evidence codec '{}' at task index {} (supported: {:?})",
                codec_ver, idx, supported_codecs
            ));
        }

        let evidence = match decode_evidence(&evidence_val) {
            Ok(ev) => ev,
            Err(e) => {
                return HistoricalRoundVerificationResult::indeterminate(format!(
                    "Failed to decode evidence at index {}: {:?}",
                    idx, e
                ));
            }
        };

        // Validate taskIdentity inside taskSpec matches child row task_identity
        if let Some(spec_identity) = evidence.task_spec.get("taskIdentity").and_then(Value::as_str) {
            if spec_identity != child.task_identity {
                return HistoricalRoundVerificationResult::corrupt(format!(
                    "Task identity mismatch at index {}: child row has '{}', task_spec has '{}'",
                    idx, child.task_identity, spec_identity
                ));
            }
        }

        manifest_tasks.push(ManifestTask::new(
            idx as u32,
            child.task_identity.clone(),
            evidence.task_spec.clone(),
            evidence.planner_version.clone(),
            evidence.policy_version.clone(),
            evidence.codec_version.clone(),
        ));

        decoded_evidences.push(evidence);
    }

    // -------------------------------------------------------------------------
    // Step 5: Verify plan_digest and request_digest
    // -------------------------------------------------------------------------
    let plan_input = PlanDigestInput {
        draft_session_id: header.draft_session_id.clone(),
        workflow_run_id: header.workflow_run_id.clone(),
        plan_generation: header.plan_generation,
        round_index: header.round_index,
        task_count: header.task_count as u32,
        parallel_limit: header.parallel_limit as u32,
        tasks: manifest_tasks,
    };

    let computed_plan_digest = match compute_plan_digest(&plan_input) {
        Ok(d) => d,
        Err(e) => {
            return HistoricalRoundVerificationResult::corrupt(format!(
                "Failed to compute plan digest: {e}"
            ));
        }
    };

    if computed_plan_digest != header.plan_digest {
        return HistoricalRoundVerificationResult::corrupt(format!(
            "plan_digest mismatch: computed '{}', header '{}'",
            computed_plan_digest, header.plan_digest
        ));
    }

    let req_input = RequestDigestInput {
        draft_session_id: header.draft_session_id.clone(),
        workflow_run_id: header.workflow_run_id.clone(),
        plan_generation: header.plan_generation,
        round_index: header.round_index,
        task_count: header.task_count as u32,
        parallel_limit: header.parallel_limit as u32,
        plan_digest: header.plan_digest.clone(),
        base_version_seq: header.base_version_seq,
        base_data_hash: header.base_data_hash.clone(),
    };

    let computed_req_digest = match compute_request_digest(&req_input) {
        Ok(d) => d,
        Err(e) => {
            return HistoricalRoundVerificationResult::corrupt(format!(
                "Failed to compute request digest: {e}"
            ));
        }
    };

    if computed_req_digest != header.request_digest {
        return HistoricalRoundVerificationResult::corrupt(format!(
            "request_digest mismatch: computed '{}', header '{}'",
            computed_req_digest, header.request_digest
        ));
    }

    // -------------------------------------------------------------------------
    // Step 6: Pure Sequential Replay and Hash Chain Verification
    // -------------------------------------------------------------------------
    let mut current_state = s0.clone();
    let mut current_hash = header.base_data_hash.clone();

    for (idx, evidence) in decoded_evidences.iter().enumerate() {
        if evidence.pre_state_hash != current_hash {
            return HistoricalRoundVerificationResult::corrupt(format!(
                "pre_state_hash chain broken at task index {}: expected '{}', found '{}'",
                idx, current_hash, evidence.pre_state_hash
            ));
        }

        let next_state = match replay_evidence(&current_state, evidence) {
            Ok(s) => s,
            Err(e) => {
                return HistoricalRoundVerificationResult::corrupt(format!(
                    "Replay failed at task index {}: {:?}",
                    idx, e
                ));
            }
        };

        let next_hash = match state_data_hash(&next_state) {
            Ok(h) => h,
            Err(e) => {
                return HistoricalRoundVerificationResult::corrupt(format!(
                    "Failed to compute state_data_hash at task index {}: {:?}",
                    idx, e
                ));
            }
        };

        if next_hash != evidence.post_state_hash {
            return HistoricalRoundVerificationResult::corrupt(format!(
                "post_state_hash mismatch at task index {}: replayed state hash is '{}', evidence has '{}'",
                idx, next_hash, evidence.post_state_hash
            ));
        }

        current_state = next_state;
        current_hash = next_hash;
    }

    if current_hash != header.final_data_hash {
        return HistoricalRoundVerificationResult::corrupt(format!(
            "final_data_hash mismatch after replaying all tasks: replayed '{}', header '{}'",
            current_hash, header.final_data_hash
        ));
    }

    // -------------------------------------------------------------------------
    // Step 7: Verify Resulting Draft Version (if present) or Zero-Net Equality
    // -------------------------------------------------------------------------
    match header.resulting_version_seq {
        Some(res_seq) => {
            let res_row_opt: Option<(String, String, String, i64, String)> = match conn.query_row(
                "SELECT data_hash, seal_key, reason, parent_seq, data_json \
                 FROM draft_versions WHERE draft_session_id = ?1 AND seq = ?2",
                params![header.draft_session_id, res_seq],
                |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?)),
            ).optional() {
                Ok(opt) => opt,
                Err(e) => return HistoricalRoundVerificationResult::corrupt(format!("Failed to query resulting draft_version: {e}")),
            };

            let (stored_hash, seal_key, reason, parent_seq, data_json) = match res_row_opt {
                Some(row) => row,
                None => return HistoricalRoundVerificationResult::corrupt(format!(
                    "Resulting draft version seq {} missing from draft_versions",
                    res_seq
                )),
            };

            if stored_hash != header.final_data_hash {
                return HistoricalRoundVerificationResult::corrupt(format!(
                    "Resulting draft_version data_hash mismatch: stored '{}', header final '{}'",
                    stored_hash, header.final_data_hash
                ));
            }
            if seal_key != header.round_seal_key {
                return HistoricalRoundVerificationResult::corrupt(format!(
                    "Resulting draft_version seal_key mismatch: stored '{}', header '{}'",
                    seal_key, header.round_seal_key
                ));
            }
            if reason != "workflow_round" {
                return HistoricalRoundVerificationResult::corrupt(format!(
                    "Resulting draft_version reason mismatch: expected 'workflow_round', found '{}'",
                    reason
                ));
            }
            if parent_seq != header.base_version_seq {
                return HistoricalRoundVerificationResult::corrupt(format!(
                    "Resulting draft_version parent_seq mismatch: expected {}, found {}",
                    header.base_version_seq, parent_seq
                ));
            }

            let parsed_final: BusinessFlowData = match serde_json::from_str(&data_json) {
                Ok(p) => p,
                Err(e) => return HistoricalRoundVerificationResult::corrupt(format!("Resulting draft_version data_json invalid: {e}")),
            };

            if !matches!(states_are_semantically_equal(&parsed_final, &current_state), Ok(true)) {
                return HistoricalRoundVerificationResult::corrupt(
                    "Resulting draft_version state is not semantically equal to replayed final state",
                );
            }
        }
        None => {
            // Zero-net: replayed state must be semantically equal to initial S0
            if !matches!(states_are_semantically_equal(&s0, &current_state), Ok(true)) {
                return HistoricalRoundVerificationResult::corrupt(
                    "Zero-net round replayed state is not semantically equal to initial S0",
                );
            }
        }
    }

    // Internal integrity is verified and attested by trusted writer
    let integrity = Integrity::Verified;
    let attestation = Attestation::Attested;

    // -------------------------------------------------------------------------
    // Step 8: Evaluate Current-Head Applicability (Dimension 3)
    // -------------------------------------------------------------------------
    let target_seq = header.resulting_version_seq.unwrap_or(header.base_version_seq);

    // Resolve head seq: either passed by caller or queried from draft_sessions
    let head_seq = match current_head_seq.into_head_seq() {
        Some(s) => s,
        None => {
            let session_head: Option<i64> = conn.query_row(
                "SELECT head_version_seq FROM draft_sessions WHERE id = ?1",
                params![header.draft_session_id],
                |row| row.get(0),
            ).optional().unwrap_or(None).flatten();

            match session_head {
                Some(s) => s,
                None => {
                    return HistoricalRoundVerificationResult::new(
                        integrity,
                        attestation,
                        Applicability::Inapplicable,
                        Some("Draft session has no active head version".to_string()),
                    );
                }
            }
        }
    };

    if head_seq == target_seq {
        return HistoricalRoundVerificationResult::new(
            integrity,
            attestation,
            Applicability::Applicable,
            Some("Round result matches current workspace head".to_string()),
        );
    }

    // If head_seq != target_seq, trace the lineage backward from head_seq to check
    // whether target_seq is an ancestor of head_seq or if a Restore operation severed it.
    let mut curr_lineage = head_seq;
    let mut is_in_lineage = false;
    let mut visited = std::collections::HashSet::new();

    while curr_lineage > 0 && visited.insert(curr_lineage) {
        if curr_lineage == target_seq {
            is_in_lineage = true;
            break;
        }

        let parent_opt: Option<i64> = conn.query_row(
            "SELECT parent_seq FROM draft_versions WHERE draft_session_id = ?1 AND seq = ?2",
            params![header.draft_session_id, curr_lineage],
            |row| row.get(0),
        ).optional().unwrap_or(None).flatten();

        match parent_opt {
            Some(p) => curr_lineage = p,
            None => break,
        }
    }

    if is_in_lineage {
        // Head advanced past target_seq along the valid ancestor chain
        HistoricalRoundVerificationResult::new(
            integrity,
            attestation,
            Applicability::Inapplicable,
            Some(format!(
                "Head has advanced to seq {} beyond round target seq {}",
                head_seq, target_seq
            )),
        )
    } else {
        // Target seq is NOT in head lineage. Check if any Restore operation occurred in this draft session.
        let restore_count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM draft_versions WHERE draft_session_id = ?1 AND reason = 'restore'",
            params![header.draft_session_id],
            |row| row.get(0),
        ).unwrap_or(0);

        if restore_count > 0 {
            // Severed by Restore: historical verified but inapplicable (REC-10, GV9, P3S9-C, F2)
            HistoricalRoundVerificationResult::new(
                integrity,
                attestation,
                Applicability::RestoreDiverged,
                Some(format!(
                    "Round target seq {} is on an abandoned branch severed by Restore (current head: {})",
                    target_seq, head_seq
                )),
            )
        } else {
            HistoricalRoundVerificationResult::new(
                integrity,
                attestation,
                Applicability::Inapplicable,
                Some(format!(
                    "Round target seq {} is not in current head {} lineage",
                    target_seq, head_seq
                )),
            )
        }
    }
}
