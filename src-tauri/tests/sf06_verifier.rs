//! Acceptance, contract, and classifier tests for `phase3_sprint9_deep2`
//! ("Verify historical B+ rounds read-only").
//!
//! Per the backend module wiring rule in `docs/plan/phase3_sprint9_deep2.md` §12:
//! - This sprint does not edit any `mod.rs` or `lib.rs`.
//! - This test target loads owned implementation and predecessor modules via `#[path = "..."]`.
//!
//! Covered acceptance vectors:
//! - `phase3_sprint9_deep2-F0`: Predecessor gates & baseline ancestry validation.
//! - `phase3_sprint9_deep2-F1`: Index gap / N+1 / hash mismatch fails closed (`Integrity::Corrupt`).
//! - `phase3_sprint9_deep2-F2`: Restore-diverged injection (`Applicability::RestoreDiverged`).
//! - `P3S9-P`: Persisted zero-net header + children verifies after restart.
//! - `P3S9-N`: Index gap or N+1 child fails closed despite SQLite FK pass.
//! - `P3S9-B`: Legacy NULL evidence yields `Indeterminate`, no backfill.
//! - `REC-01` through `REC-15`: Acceptance vectors per `be-core.md` §12.4.

#[path = "../src/error.rs"]
pub mod error;

#[path = "../src/domain/business_flow.rs"]
pub mod business_flow;

#[path = "../src/domain/fingerprint.rs"]
pub mod fingerprint;

#[path = "../src/domain/sf06/manifest.rs"]
pub mod manifest;

#[path = "../src/domain/sf06/codec.rs"]
pub mod codec;

#[path = "../src/storage/db.rs"]
pub mod db;

#[path = "../src/storage/migration.rs"]
pub mod migration;

#[path = "../src/service/receipt_verifier.rs"]
pub mod receipt_verifier;

pub mod domain {
    pub mod sf06 {
        pub use crate::codec;
        pub use crate::manifest;
    }
    pub use crate::business_flow;
    pub use crate::fingerprint;
}

pub mod service {
    pub use crate::receipt_verifier;
}

pub mod storage {
    pub use crate::db;
    pub use crate::migration;
}

use codec::*;
use manifest::*;
use business_flow::*;
use fingerprint::*;
use receipt_verifier::*;
use rusqlite::{params, Connection};
use serde_json::json;
use std::process::Command;
use tempfile::TempDir;

pub const BASELINE_COMMIT: &str = "57a462efb953a665e3673b15e6cc33622a4f2d39";
pub const PREDECESSOR_P3S8_COMMIT: &str = "95e93a229860bab6473e9777291c2e68edce3215";
pub const PREDECESSOR_P3S2_COMMIT: &str = "ded1e9c5457ab8e72f456a515937a05af6ebba63";
pub const PREDECESSOR_P3S3_COMMIT: &str = "9f8bb7f0c43ba79d22072fb73f026a54343e4363";

// -----------------------------------------------------------------------------
// Test Fixture Setup Helpers
// -----------------------------------------------------------------------------

fn setup_test_db() -> (Connection, TempDir) {
    let temp_dir = TempDir::new().expect("failed to create temp dir");
    let db_path = temp_dir.path().join("test_merlink.db");
    let mut conn = Connection::open(&db_path).expect("failed to open sqlite db");
    storage::db::configure_connection(&conn).expect("failed to configure sqlite");
    storage::migration::run_migrations(&mut conn).expect("failed to run migrations");
    (conn, temp_dir)
}

fn reopen_test_db(temp_dir: &TempDir) -> Connection {
    let db_path = temp_dir.path().join("test_merlink.db");
    let conn = Connection::open(&db_path).expect("failed to reopen sqlite db");
    storage::db::configure_connection(&conn).expect("failed to configure reopened sqlite");
    conn
}

fn seed_test_session(conn: &Connection, draft_session_id: &str) {
    let proj_id = "018e3a00-0000-7000-8000-000000000001";
    let sess_id = "018e3a00-0000-7000-8000-000000000002";
    conn.execute(
        "INSERT OR IGNORE INTO projects (id, display_name) VALUES (?, 'Test Project');",
        [proj_id],
    ).unwrap();
    conn.execute(
        "INSERT OR IGNORE INTO sessions (id, project_id) VALUES (?, ?);",
        [sess_id, proj_id],
    ).unwrap();
    conn.execute(
        "INSERT OR IGNORE INTO draft_sessions (id, session_id, project_id, head_version_seq) VALUES (?, ?, ?, 1);",
        [draft_session_id, sess_id, proj_id],
    ).unwrap();
}

fn seed_draft_version(
    conn: &Connection,
    draft_session_id: &str,
    seq: i64,
    state: &BusinessFlowData,
    reason: &str,
    parent_seq: Option<i64>,
    seal_key: Option<&str>,
) -> String {
    let ver_id = format!("018e3a00-0000-7000-8000-{:012x}", seq);
    let state_json = serde_json::to_string(state).unwrap();
    let hash = state_data_hash(state).unwrap();
    conn.execute(
        "INSERT INTO draft_versions (id, draft_session_id, seq, seal_key, phase, subphase, data_json, data_hash, reason, parent_seq, created_at)
         VALUES (?1, ?2, ?3, ?4, 1, NULL, ?5, ?6, ?7, ?8, '2026-11-08T00:00:00Z');",
        params![ver_id, draft_session_id, seq, seal_key, state_json, hash, reason, parent_seq],
    ).unwrap();

    // Update draft_sessions head_version_seq
    conn.execute(
        "UPDATE draft_sessions SET head_version_seq = ?1 WHERE id = ?2",
        params![seq, draft_session_id],
    ).unwrap();

    hash
}

// -----------------------------------------------------------------------------
// Dispatch Preconditions Helper (`phase3_sprint9_deep2-F0`)
// -----------------------------------------------------------------------------

pub fn verify_p3s9_dispatch_preconditions(
    baseline_commit: &str,
    current_head: &str,
    gate_p3s8: bool,
    gate_p3s2: bool,
    gate_p3s3: bool,
) -> Result<(), String> {
    if !gate_p3s8 {
        return Err("CONTRACT_BLOCKED: predecessor gate_phase3_sprint8_deep1 is absent".to_string());
    }
    if !gate_p3s2 {
        return Err("CONTRACT_BLOCKED: predecessor gate_phase3_sprint2_deep0 is absent".to_string());
    }
    if !gate_p3s3 {
        return Err("CONTRACT_BLOCKED: predecessor gate_phase3_sprint3_deep0 is absent".to_string());
    }

    // 1. Check baseline commit ancestry
    let out_baseline = Command::new("git")
        .args(["merge-base", "--is-ancestor", baseline_commit, current_head])
        .output();
    match out_baseline {
        Ok(out) if out.status.success() => {}
        _ => {
            return Err(format!(
                "CONTRACT_BLOCKED: baseline commit {baseline_commit} is not an ancestor of {current_head}"
            ));
        }
    }

    // 2. Check predecessor P3S8 ancestry or commit existence
    let out_p3s8 = Command::new("git")
        .args(["merge-base", "--is-ancestor", PREDECESSOR_P3S8_COMMIT, current_head])
        .output();
    let p3s8_ok = match out_p3s8 {
        Ok(out) if out.status.success() => true,
        _ => {
            let cat = Command::new("git")
                .args(["cat-file", "-e", &format!("{PREDECESSOR_P3S8_COMMIT}^{{commit}}")])
                .output();
            matches!(cat, Ok(out) if out.status.success())
        }
    };
    if !p3s8_ok {
        return Err(format!(
            "CONTRACT_BLOCKED: predecessor commit {PREDECESSOR_P3S8_COMMIT} is not an ancestor of {current_head}"
        ));
    }

    // 3. Check predecessor P3S2 ancestry or commit existence
    let out_p3s2 = Command::new("git")
        .args(["merge-base", "--is-ancestor", PREDECESSOR_P3S2_COMMIT, current_head])
        .output();
    let p3s2_ok = match out_p3s2 {
        Ok(out) if out.status.success() => true,
        _ => {
            let cat = Command::new("git")
                .args(["cat-file", "-e", &format!("{PREDECESSOR_P3S2_COMMIT}^{{commit}}")])
                .output();
            matches!(cat, Ok(out) if out.status.success())
        }
    };
    if !p3s2_ok {
        return Err(format!(
            "CONTRACT_BLOCKED: predecessor commit {PREDECESSOR_P3S2_COMMIT} is not an ancestor of {current_head}"
        ));
    }

    // 4. Check predecessor P3S3 commit exists in git
    let out_p3s3 = Command::new("git")
        .args(["rev-parse", "--verify", PREDECESSOR_P3S3_COMMIT])
        .output();
    match out_p3s3 {
        Ok(out) if out.status.success() => Ok(()),
        _ => Err(format!(
            "CONTRACT_BLOCKED: predecessor commit {PREDECESSOR_P3S3_COMMIT} not found in repository"
        )),
    }
}

// -----------------------------------------------------------------------------
// Acceptance Tests
// -----------------------------------------------------------------------------

#[test]
#[allow(non_snake_case)]
fn phase3_sprint9_deep2_F0_predecessor_gates_and_baseline_ancestry() {
    // 1. Missing gate P3S8
    let err_p3s8 = verify_p3s9_dispatch_preconditions(BASELINE_COMMIT, "HEAD", false, true, true).unwrap_err();
    assert!(err_p3s8.contains("CONTRACT_BLOCKED"));
    assert!(err_p3s8.contains("gate_phase3_sprint8_deep1"));

    // 2. Missing gate P3S2
    let err_p3s2 = verify_p3s9_dispatch_preconditions(BASELINE_COMMIT, "HEAD", true, false, true).unwrap_err();
    assert!(err_p3s2.contains("CONTRACT_BLOCKED"));
    assert!(err_p3s2.contains("gate_phase3_sprint2_deep0"));

    // 3. Missing gate P3S3
    let err_p3s3 = verify_p3s9_dispatch_preconditions(BASELINE_COMMIT, "HEAD", true, true, false).unwrap_err();
    assert!(err_p3s3.contains("CONTRACT_BLOCKED"));
    assert!(err_p3s3.contains("gate_phase3_sprint3_deep0"));

    // 4. Baseline drift simulation
    let bad_baseline = "0000000000000000000000000000000000000000";
    let err_drift = verify_p3s9_dispatch_preconditions(bad_baseline, "HEAD", true, true, true).unwrap_err();
    assert!(err_drift.contains("CONTRACT_BLOCKED"));

    // 5. Live repository check against HEAD
    let live_res = verify_p3s9_dispatch_preconditions(BASELINE_COMMIT, "HEAD", true, true, true);
    assert!(live_res.is_ok(), "Preconditions must be satisfied for current commit");
}

#[test]
#[allow(non_snake_case)]
fn P3S9_P_persisted_zero_net_header_and_children_verifies_after_restart() {
    let (conn, temp_dir) = setup_test_db();
    let draft_session_id = "018e3a00-0000-7000-8000-000000000010";
    seed_test_session(&conn, draft_session_id);

    // Initial state S0
    let mut s0 = BusinessFlowData::new();
    s0.counters.insert("test_counter".to_string(), 1);
    let s0_hash = seed_draft_version(&conn, draft_session_id, 1, &s0, "phase_confirm", None, None);

    // T1: 1 -> 2 (s0 -> s1)
    let mut s1 = s0.clone();
    s1.counters.insert("test_counter".to_string(), 2);
    let s1_hash = state_data_hash(&s1).unwrap();
    let patch_t1 = canonical_diff(&s0, &s1).unwrap();

    // T2: 2 -> 1 (s1 -> s2 = s0, net zero)
    let mut s2 = s1.clone();
    s2.counters.insert("test_counter".to_string(), 1);
    let s2_hash = state_data_hash(&s2).unwrap();
    let patch_t2 = canonical_diff(&s1, &s2).unwrap();

    let round_id = "018e3a00-0000-7000-8000-000000000020";
    let run_id = "018e3a00-0000-7000-8000-000000000021";
    let round_seal_key = format!("wf_round:{}:1:0", run_id);

    let ev1 = EvidenceJsonV2 {
        schema_version: 2,
        codec_version: "v2".to_string(),
        planner_version: "v1".to_string(),
        policy_version: "v1".to_string(),
        task_spec: json!({ "taskIdentity": "p1:s1:k1:scope1" }),
        pre_state_hash: s0_hash.clone(),
        post_state_hash: s1_hash.clone(),
        had_applied_writes: true,
        effective_state_changed: true,
        patch: patch_t1.ops,
        artifact: None,
    };

    let ev2 = EvidenceJsonV2 {
        schema_version: 2,
        codec_version: "v2".to_string(),
        planner_version: "v1".to_string(),
        policy_version: "v1".to_string(),
        task_spec: json!({ "taskIdentity": "p1:s1:k2:scope2" }),
        pre_state_hash: s1_hash.clone(),
        post_state_hash: s2_hash.clone(),
        had_applied_writes: true,
        effective_state_changed: true,
        patch: patch_t2.ops,
        artifact: None,
    };

    let manifest_tasks = vec![
        ManifestTask::new(0, "p1:s1:k1:scope1", ev1.task_spec.clone(), "v1", "v1", "v2"),
        ManifestTask::new(1, "p1:s1:k2:scope2", ev2.task_spec.clone(), "v1", "v1", "v2"),
    ];

    let plan_input = PlanDigestInput {
        draft_session_id: draft_session_id.to_string(),
        workflow_run_id: run_id.to_string(),
        plan_generation: 1,
        round_index: 0,
        task_count: 2,
        parallel_limit: 2,
        tasks: manifest_tasks,
    };
    let plan_digest = compute_plan_digest(&plan_input).unwrap();

    let req_input = RequestDigestInput {
        draft_session_id: draft_session_id.to_string(),
        workflow_run_id: run_id.to_string(),
        plan_generation: 1,
        round_index: 0,
        task_count: 2,
        parallel_limit: 2,
        plan_digest: plan_digest.clone(),
        base_version_seq: 1,
        base_data_hash: s0_hash.clone(),
    };
    let request_digest = compute_request_digest(&req_input).unwrap();

    // Insert round receipt (zero-net: resulting_version_seq IS NULL, base_data_hash == final_data_hash)
    conn.execute(
        "INSERT INTO workflow_round_receipts (
            id, draft_session_id, workflow_run_id, plan_generation, round_index,
            round_seal_key, task_count, parallel_limit, plan_digest, request_digest,
            base_version_seq, base_data_hash, final_data_hash, resulting_version_seq, created_at
         ) VALUES (?1, ?2, ?3, 1, 0, ?4, 2, 2, ?5, ?6, 1, ?7, ?7, NULL, '2026-11-08T00:00:00Z');",
        params![round_id, draft_session_id, run_id, round_seal_key, plan_digest, request_digest, s0_hash],
    ).unwrap();

    // Insert child task receipts
    conn.execute(
        "INSERT INTO workflow_task_receipts (
            id, draft_session_id, workflow_run_id, plan_generation, task_identity,
            input_scope_fingerprint, output_scope_fingerprint, base_version_seq, resulting_version_seq,
            had_mutations, round_receipt_id, task_index, evidence_json, created_at
         ) VALUES ('018e3a00-0000-7000-8000-000000000031', ?1, ?2, 1, 'p1:s1:k1:scope1',
                   'in-fp-1', 'out-fp-1', 1, NULL, 0, ?3, 0, ?4, '2026-11-08T00:00:00Z');",
        params![draft_session_id, run_id, round_id, serde_json::to_string(&ev1).unwrap()],
    ).unwrap();

    conn.execute(
        "INSERT INTO workflow_task_receipts (
            id, draft_session_id, workflow_run_id, plan_generation, task_identity,
            input_scope_fingerprint, output_scope_fingerprint, base_version_seq, resulting_version_seq,
            had_mutations, round_receipt_id, task_index, evidence_json, created_at
         ) VALUES ('018e3a00-0000-7000-8000-000000000032', ?1, ?2, 1, 'p1:s1:k2:scope2',
                   'in-fp-2', 'out-fp-2', 1, NULL, 0, ?3, 1, ?4, '2026-11-08T00:00:00Z');",
        params![draft_session_id, run_id, round_id, serde_json::to_string(&ev2).unwrap()],
    ).unwrap();

    drop(conn);

    // Simulate process restart by reopening fresh DB connection from disk
    let conn_restarted = reopen_test_db(&temp_dir);

    // Verify historical round
    let result = verify_historical_round(&conn_restarted, round_id, &["v2"], 1);
    assert_eq!(result.integrity, Integrity::Verified);
    assert_eq!(result.attestation, Attestation::Attested);
    assert_eq!(result.applicability, Applicability::Applicable);
    assert!(result.is_verified());
    assert!(result.is_attested());
    assert!(result.is_applicable());
}

#[test]
#[allow(non_snake_case)]
fn P3S9_N_and_phase3_sprint9_deep2_F1_index_gap_and_hash_mismatch_fails_closed() {
    let (conn, _temp_dir) = setup_test_db();
    let draft_session_id = "018e3a00-0000-7000-8000-000000000010";
    seed_test_session(&conn, draft_session_id);

    let mut s0 = BusinessFlowData::new();
    s0.counters.insert("test_counter".to_string(), 10);
    let s0_hash = seed_draft_version(&conn, draft_session_id, 1, &s0, "phase_confirm", None, None);

    let round_id = "018e3a00-0000-7000-8000-000000000020";
    let run_id = "018e3a00-0000-7000-8000-000000000021";
    let round_seal_key = format!("wf_round:{}:1:0", run_id);

    // Case 1: Task count mismatch (header specifies 2 tasks, but only 1 child exists)
    conn.execute(
        "INSERT INTO workflow_round_receipts (
            id, draft_session_id, workflow_run_id, plan_generation, round_index,
            round_seal_key, task_count, parallel_limit, plan_digest, request_digest,
            base_version_seq, base_data_hash, final_data_hash, resulting_version_seq, created_at
         ) VALUES (?1, ?2, ?3, 1, 0, ?4, 2, 2, 'dummy-plan', 'dummy-req', 1, ?5, ?5, NULL, '2026-11-08T00:00:00Z');",
        params![round_id, draft_session_id, run_id, round_seal_key, s0_hash],
    ).unwrap();

    let ev1 = EvidenceJsonV2 {
        schema_version: 2,
        codec_version: "v2".to_string(),
        planner_version: "v1".to_string(),
        policy_version: "v1".to_string(),
        task_spec: json!({ "taskIdentity": "p1:s1:k1:scope1" }),
        pre_state_hash: s0_hash.clone(),
        post_state_hash: s0_hash.clone(),
        had_applied_writes: false,
        effective_state_changed: false,
        patch: vec![],
        artifact: None,
    };

    conn.execute(
        "INSERT INTO workflow_task_receipts (
            id, draft_session_id, workflow_run_id, plan_generation, task_identity,
            input_scope_fingerprint, output_scope_fingerprint, base_version_seq, resulting_version_seq,
            had_mutations, round_receipt_id, task_index, evidence_json, created_at
         ) VALUES ('018e3a00-0000-7000-8000-000000000031', ?1, ?2, 1, 'p1:s1:k1:scope1',
                   'in-fp-1', 'out-fp-1', 1, NULL, 0, ?3, 0, ?4, '2026-11-08T00:00:00Z');",
        params![draft_session_id, run_id, round_id, serde_json::to_string(&ev1).unwrap()],
    ).unwrap();

    let res_count_mismatch = verify_historical_round(&conn, round_id, &["v2"], 1);
    assert_eq!(res_count_mismatch.integrity, Integrity::Corrupt);
    assert!(res_count_mismatch.details.unwrap().contains("Task count mismatch"));

    // Case 2: Index gap: insert second task with task_index = 5 instead of 1 (valid in SQLite schema)
    conn.execute(
        "INSERT INTO workflow_task_receipts (
            id, draft_session_id, workflow_run_id, plan_generation, task_identity,
            input_scope_fingerprint, output_scope_fingerprint, base_version_seq, resulting_version_seq,
            had_mutations, round_receipt_id, task_index, evidence_json, created_at
         ) VALUES ('018e3a00-0000-7000-8000-000000000032', ?1, ?2, 1, 'p1:s1:k2:scope2',
                   'in-fp-2', 'out-fp-2', 1, NULL, 0, ?3, 5, ?4, '2026-11-08T00:00:00Z');",
        params![draft_session_id, run_id, round_id, serde_json::to_string(&ev1).unwrap()],
    ).unwrap();

    let res_index_gap = verify_historical_round(&conn, round_id, &["v2"], 1);
    assert_eq!(res_index_gap.integrity, Integrity::Corrupt);
    assert!(res_index_gap.details.unwrap().contains("Task index gap/mismatch"));
}

#[test]
#[allow(non_snake_case)]
fn P3S9_B_and_REC_02_legacy_null_evidence_yields_indeterminate() {
    let (conn, _temp_dir) = setup_test_db();
    let draft_session_id = "018e3a00-0000-7000-8000-000000000010";
    seed_test_session(&conn, draft_session_id);

    let s0 = BusinessFlowData::new();
    let s0_hash = seed_draft_version(&conn, draft_session_id, 1, &s0, "phase_confirm", None, None);

    let round_id = "018e3a00-0000-7000-8000-000000000020";
    let run_id = "018e3a00-0000-7000-8000-000000000021";
    let round_seal_key = format!("wf_round:{}:1:0", run_id);

    conn.execute(
        "INSERT INTO workflow_round_receipts (
            id, draft_session_id, workflow_run_id, plan_generation, round_index,
            round_seal_key, task_count, parallel_limit, plan_digest, request_digest,
            base_version_seq, base_data_hash, final_data_hash, resulting_version_seq, created_at
         ) VALUES (?1, ?2, ?3, 1, 0, ?4, 1, 1, 'dummy-plan', 'dummy-req', 1, ?5, ?5, NULL, '2026-11-08T00:00:00Z');",
        params![round_id, draft_session_id, run_id, round_seal_key, s0_hash],
    ).unwrap();

    // Disable foreign key and check enforcement to simulate legacy NULL evidence in SQLite
    conn.execute("PRAGMA ignore_check_constraints = ON;", []).unwrap();
    conn.execute(
        "INSERT INTO workflow_task_receipts (
            id, draft_session_id, workflow_run_id, plan_generation, task_identity,
            input_scope_fingerprint, output_scope_fingerprint, base_version_seq, resulting_version_seq,
            had_mutations, round_receipt_id, task_index, evidence_json, created_at
         ) VALUES ('018e3a00-0000-7000-8000-000000000031', ?1, ?2, 1, 'p1:s1:k1:scope1',
                   'in-fp-1', 'out-fp-1', 1, NULL, 0, ?3, 0, NULL, '2026-11-08T00:00:00Z');",
        params![draft_session_id, run_id, round_id],
    ).unwrap();
    conn.execute("PRAGMA ignore_check_constraints = OFF;", []).unwrap();

    let res = verify_historical_round(&conn, round_id, &["v2"], 1);
    assert_eq!(res.integrity, Integrity::Indeterminate);
    assert_eq!(res.attestation, Attestation::Unknown);
    assert!(res.details.unwrap().contains("NULL evidence_json"));
}

#[test]
#[allow(non_snake_case)]
fn phase3_sprint9_deep2_F2_and_REC_10_restore_diverged_injection() {
    let (conn, _temp_dir) = setup_test_db();
    let draft_session_id = "018e3a00-0000-7000-8000-000000000010";
    seed_test_session(&conn, draft_session_id);

    // V1: baseline
    let mut s1 = BusinessFlowData::new();
    s1.counters.insert("test_counter".to_string(), 1);
    let s1_hash = seed_draft_version(&conn, draft_session_id, 1, &s1, "phase_confirm", None, None);

    // V2: mutating round executed from V1 creating V2
    let mut s2 = s1.clone();
    s2.counters.insert("test_counter".to_string(), 2);
    let patch_t1 = canonical_diff(&s1, &s2).unwrap();
    let round_seal_key = "wf_round:018e3a00-0000-7000-8000-000000000021:1:0";
    let s2_hash = seed_draft_version(&conn, draft_session_id, 2, &s2, "workflow_round", Some(1), Some(round_seal_key));

    let round_id = "018e3a00-0000-7000-8000-000000000020";
    let run_id = "018e3a00-0000-7000-8000-000000000021";

    let ev1 = EvidenceJsonV2 {
        schema_version: 2,
        codec_version: "v2".to_string(),
        planner_version: "v1".to_string(),
        policy_version: "v1".to_string(),
        task_spec: json!({ "taskIdentity": "p1:s1:k1:scope1" }),
        pre_state_hash: s1_hash.clone(),
        post_state_hash: s2_hash.clone(),
        had_applied_writes: true,
        effective_state_changed: true,
        patch: patch_t1.ops,
        artifact: None,
    };

    let manifest_tasks = vec![
        ManifestTask::new(0, "p1:s1:k1:scope1", ev1.task_spec.clone(), "v1", "v1", "v2"),
    ];

    let plan_input = PlanDigestInput {
        draft_session_id: draft_session_id.to_string(),
        workflow_run_id: run_id.to_string(),
        plan_generation: 1,
        round_index: 0,
        task_count: 1,
        parallel_limit: 1,
        tasks: manifest_tasks,
    };
    let plan_digest = compute_plan_digest(&plan_input).unwrap();

    let req_input = RequestDigestInput {
        draft_session_id: draft_session_id.to_string(),
        workflow_run_id: run_id.to_string(),
        plan_generation: 1,
        round_index: 0,
        task_count: 1,
        parallel_limit: 1,
        plan_digest: plan_digest.clone(),
        base_version_seq: 1,
        base_data_hash: s1_hash.clone(),
    };
    let request_digest = compute_request_digest(&req_input).unwrap();

    conn.execute(
        "INSERT INTO workflow_round_receipts (
            id, draft_session_id, workflow_run_id, plan_generation, round_index,
            round_seal_key, task_count, parallel_limit, plan_digest, request_digest,
            base_version_seq, base_data_hash, final_data_hash, resulting_version_seq, created_at
         ) VALUES (?1, ?2, ?3, 1, 0, ?4, 1, 1, ?5, ?6, 1, ?7, ?8, 2, '2026-11-08T00:00:00Z');",
        params![round_id, draft_session_id, run_id, round_seal_key, plan_digest, request_digest, s1_hash, s2_hash],
    ).unwrap();

    conn.execute(
        "INSERT INTO workflow_task_receipts (
            id, draft_session_id, workflow_run_id, plan_generation, task_identity,
            input_scope_fingerprint, output_scope_fingerprint, base_version_seq, resulting_version_seq,
            had_mutations, round_receipt_id, task_index, evidence_json, created_at
         ) VALUES ('018e3a00-0000-7000-8000-000000000031', ?1, ?2, 1, 'p1:s1:k1:scope1',
                   'in-fp-1', 'out-fp-1', 1, 2, 1, ?3, 0, ?4, '2026-11-08T00:00:00Z');",
        params![draft_session_id, run_id, round_id, serde_json::to_string(&ev1).unwrap()],
    ).unwrap();

    // Now inject Restore: Restore back to V1 creating V3 (parent_seq = 1, reason = 'restore')
    seed_draft_version(&conn, draft_session_id, 3, &s1, "restore", Some(1), Some("restore_seal_key_3"));

    // Head is now 3. Verify historical round V2:
    // Internal integrity must be Verified, attestation must be Attested, but applicability must be RestoreDiverged!
    let res = verify_historical_round(&conn, round_id, &["v2"], 3);
    assert_eq!(res.integrity, Integrity::Verified);
    assert_eq!(res.attestation, Attestation::Attested);
    assert_eq!(res.applicability, Applicability::RestoreDiverged);
    assert!(res.details.unwrap().contains("severed by Restore"));
}

#[test]
#[allow(non_snake_case)]
fn REC_01_t2_depends_on_t1_uses_intermediate_s1_fingerprint() {
    let (conn, _temp_dir) = setup_test_db();
    let draft_session_id = "018e3a00-0000-7000-8000-000000000010";
    seed_test_session(&conn, draft_session_id);

    let mut s0 = BusinessFlowData::new();
    s0.counters.insert("A".to_string(), 1);
    s0.counters.insert("B".to_string(), 0);
    let s0_hash = seed_draft_version(&conn, draft_session_id, 1, &s0, "phase_confirm", None, None);

    // T1: A 1 -> 2
    let mut s1 = s0.clone();
    s1.counters.insert("A".to_string(), 2);
    let s1_hash = state_data_hash(&s1).unwrap();
    let patch1 = canonical_diff(&s0, &s1).unwrap();

    // T2: B = A + 1 = 3
    let mut s2 = s1.clone();
    s2.counters.insert("B".to_string(), 3);
    let s2_hash = state_data_hash(&s2).unwrap();
    let patch2 = canonical_diff(&s1, &s2).unwrap();

    let round_id = "018e3a00-0000-7000-8000-000000000020";
    let run_id = "018e3a00-0000-7000-8000-000000000021";
    let round_seal_key = format!("wf_round:{}:1:0", run_id);
    seed_draft_version(&conn, draft_session_id, 2, &s2, "workflow_round", Some(1), Some(&round_seal_key));

    // Construct ev2 but intentionally use S0 hash instead of S1 hash for pre_state_hash
    let ev1 = EvidenceJsonV2 {
        schema_version: 2,
        codec_version: "v2".to_string(),
        planner_version: "v1".to_string(),
        policy_version: "v1".to_string(),
        task_spec: json!({ "taskIdentity": "t1" }),
        pre_state_hash: s0_hash.clone(),
        post_state_hash: s1_hash.clone(),
        had_applied_writes: true,
        effective_state_changed: true,
        patch: patch1.ops,
        artifact: None,
    };

    let bad_ev2 = EvidenceJsonV2 {
        schema_version: 2,
        codec_version: "v2".to_string(),
        planner_version: "v1".to_string(),
        policy_version: "v1".to_string(),
        task_spec: json!({ "taskIdentity": "t2" }),
        pre_state_hash: s0_hash.clone(), // INCORRECT: should be s1_hash
        post_state_hash: s2_hash.clone(),
        had_applied_writes: true,
        effective_state_changed: true,
        patch: patch2.ops,
        artifact: None,
    };

    let manifest_tasks = vec![
        ManifestTask::new(0, "t1", ev1.task_spec.clone(), "v1", "v1", "v2"),
        ManifestTask::new(1, "t2", bad_ev2.task_spec.clone(), "v1", "v1", "v2"),
    ];

    let plan_input = PlanDigestInput {
        draft_session_id: draft_session_id.to_string(),
        workflow_run_id: run_id.to_string(),
        plan_generation: 1,
        round_index: 0,
        task_count: 2,
        parallel_limit: 2,
        tasks: manifest_tasks,
    };
    let plan_digest = compute_plan_digest(&plan_input).unwrap();

    let req_input = RequestDigestInput {
        draft_session_id: draft_session_id.to_string(),
        workflow_run_id: run_id.to_string(),
        plan_generation: 1,
        round_index: 0,
        task_count: 2,
        parallel_limit: 2,
        plan_digest: plan_digest.clone(),
        base_version_seq: 1,
        base_data_hash: s0_hash.clone(),
    };
    let request_digest = compute_request_digest(&req_input).unwrap();

    conn.execute(
        "INSERT INTO workflow_round_receipts (
            id, draft_session_id, workflow_run_id, plan_generation, round_index,
            round_seal_key, task_count, parallel_limit, plan_digest, request_digest,
            base_version_seq, base_data_hash, final_data_hash, resulting_version_seq, created_at
         ) VALUES (?1, ?2, ?3, 1, 0, ?4, 2, 2, ?5, ?6, 1, ?7, ?8, 2, '2026-11-08T00:00:00Z');",
        params![round_id, draft_session_id, run_id, round_seal_key, plan_digest, request_digest, s0_hash, s2_hash],
    ).unwrap();

    conn.execute(
        "INSERT INTO workflow_task_receipts (
            id, draft_session_id, workflow_run_id, plan_generation, task_identity,
            input_scope_fingerprint, output_scope_fingerprint, base_version_seq, resulting_version_seq,
            had_mutations, round_receipt_id, task_index, evidence_json, created_at
         ) VALUES ('018e3a00-0000-7000-8000-000000000031', ?1, ?2, 1, 't1',
                   'in-fp-1', 'out-fp-1', 1, 2, 1, ?3, 0, ?4, '2026-11-08T00:00:00Z');",
        params![draft_session_id, run_id, round_id, serde_json::to_string(&ev1).unwrap()],
    ).unwrap();

    conn.execute(
        "INSERT INTO workflow_task_receipts (
            id, draft_session_id, workflow_run_id, plan_generation, task_identity,
            input_scope_fingerprint, output_scope_fingerprint, base_version_seq, resulting_version_seq,
            had_mutations, round_receipt_id, task_index, evidence_json, created_at
         ) VALUES ('018e3a00-0000-7000-8000-000000000032', ?1, ?2, 1, 't2',
                   'in-fp-2', 'out-fp-2', 1, 2, 1, ?3, 1, ?4, '2026-11-08T00:00:00Z');",
        params![draft_session_id, run_id, round_id, serde_json::to_string(&bad_ev2).unwrap()],
    ).unwrap();

    // Hash chain verification must fail closed because T2 uses S0 instead of S1
    let res = verify_historical_round(&conn, round_id, &["v2"], 2);
    assert_eq!(res.integrity, Integrity::Corrupt);
    assert!(res.details.unwrap().contains("pre_state_hash chain broken"));
}

#[test]
#[allow(non_snake_case)]
fn REC_03_net_zero_round_with_writes_is_attested() {
    let (conn, _temp_dir) = setup_test_db();
    let draft_session_id = "018e3a00-0000-7000-8000-000000000010";
    seed_test_session(&conn, draft_session_id);

    let mut s0 = BusinessFlowData::new();
    s0.counters.insert("test_counter".to_string(), 5);
    let s0_hash = seed_draft_version(&conn, draft_session_id, 1, &s0, "phase_confirm", None, None);

    let mut s1 = s0.clone();
    s1.counters.insert("test_counter".to_string(), 10);
    let s1_hash = state_data_hash(&s1).unwrap();
    let patch1 = canonical_diff(&s0, &s1).unwrap();

    let mut s2 = s1.clone();
    s2.counters.insert("test_counter".to_string(), 5);
    let s2_hash = state_data_hash(&s2).unwrap();
    let patch2 = canonical_diff(&s1, &s2).unwrap();

    let round_id = "018e3a00-0000-7000-8000-000000000020";
    let run_id = "018e3a00-0000-7000-8000-000000000021";
    let round_seal_key = format!("wf_round:{}:1:0", run_id);

    let ev1 = EvidenceJsonV2 {
        schema_version: 2,
        codec_version: "v2".to_string(),
        planner_version: "v1".to_string(),
        policy_version: "v1".to_string(),
        task_spec: json!({ "taskIdentity": "t1" }),
        pre_state_hash: s0_hash.clone(),
        post_state_hash: s1_hash.clone(),
        had_applied_writes: true,
        effective_state_changed: true,
        patch: patch1.ops,
        artifact: None,
    };

    let ev2 = EvidenceJsonV2 {
        schema_version: 2,
        codec_version: "v2".to_string(),
        planner_version: "v1".to_string(),
        policy_version: "v1".to_string(),
        task_spec: json!({ "taskIdentity": "t2" }),
        pre_state_hash: s1_hash.clone(),
        post_state_hash: s2_hash.clone(),
        had_applied_writes: true,
        effective_state_changed: true,
        patch: patch2.ops,
        artifact: None,
    };

    let manifest_tasks = vec![
        ManifestTask::new(0, "t1", ev1.task_spec.clone(), "v1", "v1", "v2"),
        ManifestTask::new(1, "t2", ev2.task_spec.clone(), "v1", "v1", "v2"),
    ];

    let plan_input = PlanDigestInput {
        draft_session_id: draft_session_id.to_string(),
        workflow_run_id: run_id.to_string(),
        plan_generation: 1,
        round_index: 0,
        task_count: 2,
        parallel_limit: 2,
        tasks: manifest_tasks,
    };
    let plan_digest = compute_plan_digest(&plan_input).unwrap();

    let req_input = RequestDigestInput {
        draft_session_id: draft_session_id.to_string(),
        workflow_run_id: run_id.to_string(),
        plan_generation: 1,
        round_index: 0,
        task_count: 2,
        parallel_limit: 2,
        plan_digest: plan_digest.clone(),
        base_version_seq: 1,
        base_data_hash: s0_hash.clone(),
    };
    let request_digest = compute_request_digest(&req_input).unwrap();

    conn.execute(
        "INSERT INTO workflow_round_receipts (
            id, draft_session_id, workflow_run_id, plan_generation, round_index,
            round_seal_key, task_count, parallel_limit, plan_digest, request_digest,
            base_version_seq, base_data_hash, final_data_hash, resulting_version_seq, created_at
         ) VALUES (?1, ?2, ?3, 1, 0, ?4, 2, 2, ?5, ?6, 1, ?7, ?7, NULL, '2026-11-08T00:00:00Z');",
        params![round_id, draft_session_id, run_id, round_seal_key, plan_digest, request_digest, s0_hash],
    ).unwrap();

    conn.execute(
        "INSERT INTO workflow_task_receipts (
            id, draft_session_id, workflow_run_id, plan_generation, task_identity,
            input_scope_fingerprint, output_scope_fingerprint, base_version_seq, resulting_version_seq,
            had_mutations, round_receipt_id, task_index, evidence_json, created_at
         ) VALUES ('018e3a00-0000-7000-8000-000000000031', ?1, ?2, 1, 't1',
                   'in-fp-1', 'out-fp-1', 1, NULL, 0, ?3, 0, ?4, '2026-11-08T00:00:00Z');",
        params![draft_session_id, run_id, round_id, serde_json::to_string(&ev1).unwrap()],
    ).unwrap();

    conn.execute(
        "INSERT INTO workflow_task_receipts (
            id, draft_session_id, workflow_run_id, plan_generation, task_identity,
            input_scope_fingerprint, output_scope_fingerprint, base_version_seq, resulting_version_seq,
            had_mutations, round_receipt_id, task_index, evidence_json, created_at
         ) VALUES ('018e3a00-0000-7000-8000-000000000032', ?1, ?2, 1, 't2',
                   'in-fp-2', 'out-fp-2', 1, NULL, 0, ?3, 1, ?4, '2026-11-08T00:00:00Z');",
        params![draft_session_id, run_id, round_id, serde_json::to_string(&ev2).unwrap()],
    ).unwrap();

    let res = verify_historical_round(&conn, round_id, &["v2"], 1);
    assert_eq!(res.integrity, Integrity::Verified);
    assert_eq!(res.attestation, Attestation::Attested);
    assert_eq!(res.applicability, Applicability::Applicable);
}

#[test]
#[allow(non_snake_case)]
fn REC_04_had_mutations_zero_preserves_evidence_replay() {
    let (conn, _temp_dir) = setup_test_db();
    let draft_session_id = "018e3a00-0000-7000-8000-000000000010";
    seed_test_session(&conn, draft_session_id);

    let s0 = BusinessFlowData::new();
    let s0_hash = seed_draft_version(&conn, draft_session_id, 1, &s0, "phase_confirm", None, None);

    let round_id = "018e3a00-0000-7000-8000-000000000020";
    let run_id = "018e3a00-0000-7000-8000-000000000021";
    let round_seal_key = format!("wf_round:{}:1:0", run_id);

    // Task had_applied_writes is true, but effective_state_changed is false (empty patch)
    let ev1 = EvidenceJsonV2 {
        schema_version: 2,
        codec_version: "v2".to_string(),
        planner_version: "v1".to_string(),
        policy_version: "v1".to_string(),
        task_spec: json!({ "taskIdentity": "t1" }),
        pre_state_hash: s0_hash.clone(),
        post_state_hash: s0_hash.clone(),
        had_applied_writes: true,
        effective_state_changed: false,
        patch: vec![],
        artifact: None,
    };

    let manifest_tasks = vec![
        ManifestTask::new(0, "t1", ev1.task_spec.clone(), "v1", "v1", "v2"),
    ];

    let plan_input = PlanDigestInput {
        draft_session_id: draft_session_id.to_string(),
        workflow_run_id: run_id.to_string(),
        plan_generation: 1,
        round_index: 0,
        task_count: 1,
        parallel_limit: 1,
        tasks: manifest_tasks,
    };
    let plan_digest = compute_plan_digest(&plan_input).unwrap();

    let req_input = RequestDigestInput {
        draft_session_id: draft_session_id.to_string(),
        workflow_run_id: run_id.to_string(),
        plan_generation: 1,
        round_index: 0,
        task_count: 1,
        parallel_limit: 1,
        plan_digest: plan_digest.clone(),
        base_version_seq: 1,
        base_data_hash: s0_hash.clone(),
    };
    let request_digest = compute_request_digest(&req_input).unwrap();

    conn.execute(
        "INSERT INTO workflow_round_receipts (
            id, draft_session_id, workflow_run_id, plan_generation, round_index,
            round_seal_key, task_count, parallel_limit, plan_digest, request_digest,
            base_version_seq, base_data_hash, final_data_hash, resulting_version_seq, created_at
         ) VALUES (?1, ?2, ?3, 1, 0, ?4, 1, 1, ?5, ?6, 1, ?7, ?7, NULL, '2026-11-08T00:00:00Z');",
        params![round_id, draft_session_id, run_id, round_seal_key, plan_digest, request_digest, s0_hash],
    ).unwrap();

    conn.execute(
        "INSERT INTO workflow_task_receipts (
            id, draft_session_id, workflow_run_id, plan_generation, task_identity,
            input_scope_fingerprint, output_scope_fingerprint, base_version_seq, resulting_version_seq,
            had_mutations, round_receipt_id, task_index, evidence_json, created_at
         ) VALUES ('018e3a00-0000-7000-8000-000000000031', ?1, ?2, 1, 't1',
                   'in-fp-1', 'out-fp-1', 1, NULL, 0, ?3, 0, ?4, '2026-11-08T00:00:00Z');",
        params![draft_session_id, run_id, round_id, serde_json::to_string(&ev1).unwrap()],
    ).unwrap();

    let res = verify_historical_round(&conn, round_id, &["v2"], 1);
    assert_eq!(res.integrity, Integrity::Verified);
    assert_eq!(res.attestation, Attestation::Attested);
    assert_eq!(res.applicability, Applicability::Applicable);
}

#[test]
#[allow(non_snake_case)]
fn REC_05_same_process_retry_is_deterministic_and_idempotent() {
    let (conn, _temp_dir) = setup_test_db();
    let draft_session_id = "018e3a00-0000-7000-8000-000000000010";
    seed_test_session(&conn, draft_session_id);

    let s0 = BusinessFlowData::new();
    let s0_hash = seed_draft_version(&conn, draft_session_id, 1, &s0, "phase_confirm", None, None);

    let round_id = "018e3a00-0000-7000-8000-000000000020";
    let run_id = "018e3a00-0000-7000-8000-000000000021";
    let round_seal_key = format!("wf_round:{}:1:0", run_id);

    let ev1 = EvidenceJsonV2 {
        schema_version: 2,
        codec_version: "v2".to_string(),
        planner_version: "v1".to_string(),
        policy_version: "v1".to_string(),
        task_spec: json!({ "taskIdentity": "t1" }),
        pre_state_hash: s0_hash.clone(),
        post_state_hash: s0_hash.clone(),
        had_applied_writes: false,
        effective_state_changed: false,
        patch: vec![],
        artifact: None,
    };

    let manifest_tasks = vec![
        ManifestTask::new(0, "t1", ev1.task_spec.clone(), "v1", "v1", "v2"),
    ];

    let plan_input = PlanDigestInput {
        draft_session_id: draft_session_id.to_string(),
        workflow_run_id: run_id.to_string(),
        plan_generation: 1,
        round_index: 0,
        task_count: 1,
        parallel_limit: 1,
        tasks: manifest_tasks,
    };
    let plan_digest = compute_plan_digest(&plan_input).unwrap();

    let req_input = RequestDigestInput {
        draft_session_id: draft_session_id.to_string(),
        workflow_run_id: run_id.to_string(),
        plan_generation: 1,
        round_index: 0,
        task_count: 1,
        parallel_limit: 1,
        plan_digest: plan_digest.clone(),
        base_version_seq: 1,
        base_data_hash: s0_hash.clone(),
    };
    let request_digest = compute_request_digest(&req_input).unwrap();

    conn.execute(
        "INSERT INTO workflow_round_receipts (
            id, draft_session_id, workflow_run_id, plan_generation, round_index,
            round_seal_key, task_count, parallel_limit, plan_digest, request_digest,
            base_version_seq, base_data_hash, final_data_hash, resulting_version_seq, created_at
         ) VALUES (?1, ?2, ?3, 1, 0, ?4, 1, 1, ?5, ?6, 1, ?7, ?7, NULL, '2026-11-08T00:00:00Z');",
        params![round_id, draft_session_id, run_id, round_seal_key, plan_digest, request_digest, s0_hash],
    ).unwrap();

    conn.execute(
        "INSERT INTO workflow_task_receipts (
            id, draft_session_id, workflow_run_id, plan_generation, task_identity,
            input_scope_fingerprint, output_scope_fingerprint, base_version_seq, resulting_version_seq,
            had_mutations, round_receipt_id, task_index, evidence_json, created_at
         ) VALUES ('018e3a00-0000-7000-8000-000000000031', ?1, ?2, 1, 't1',
                   'in-fp-1', 'out-fp-1', 1, NULL, 0, ?3, 0, ?4, '2026-11-08T00:00:00Z');",
        params![draft_session_id, run_id, round_id, serde_json::to_string(&ev1).unwrap()],
    ).unwrap();

    let res1 = verify_historical_round(&conn, round_id, &["v2"], 1);
    let res2 = verify_historical_round(&conn, round_id, &["v2"], 1);
    assert_eq!(res1, res2, "Repeated verifications must produce exact identical results");
}

#[test]
#[allow(non_snake_case)]
fn REC_06_lost_ipc_response_verifies_independent_of_request_strings() {
    let (conn, _temp_dir) = setup_test_db();
    let draft_session_id = "018e3a00-0000-7000-8000-000000000010";
    seed_test_session(&conn, draft_session_id);

    let s0 = BusinessFlowData::new();
    let s0_hash = seed_draft_version(&conn, draft_session_id, 1, &s0, "phase_confirm", None, None);

    let round_id = "018e3a00-0000-7000-8000-000000000020";
    let run_id = "018e3a00-0000-7000-8000-000000000021";
    let round_seal_key = format!("wf_round:{}:1:0", run_id);

    let ev1 = EvidenceJsonV2 {
        schema_version: 2,
        codec_version: "v2".to_string(),
        planner_version: "v1".to_string(),
        policy_version: "v1".to_string(),
        task_spec: json!({ "taskIdentity": "t1" }),
        pre_state_hash: s0_hash.clone(),
        post_state_hash: s0_hash.clone(),
        had_applied_writes: false,
        effective_state_changed: false,
        patch: vec![],
        artifact: None,
    };

    let manifest_tasks = vec![
        ManifestTask::new(0, "t1", ev1.task_spec.clone(), "v1", "v1", "v2"),
    ];

    let plan_input = PlanDigestInput {
        draft_session_id: draft_session_id.to_string(),
        workflow_run_id: run_id.to_string(),
        plan_generation: 1,
        round_index: 0,
        task_count: 1,
        parallel_limit: 1,
        tasks: manifest_tasks,
    };
    let plan_digest = compute_plan_digest(&plan_input).unwrap();

    let req_input = RequestDigestInput {
        draft_session_id: draft_session_id.to_string(),
        workflow_run_id: run_id.to_string(),
        plan_generation: 1,
        round_index: 0,
        task_count: 1,
        parallel_limit: 1,
        plan_digest: plan_digest.clone(),
        base_version_seq: 1,
        base_data_hash: s0_hash.clone(),
    };
    let request_digest = compute_request_digest(&req_input).unwrap();

    conn.execute(
        "INSERT INTO workflow_round_receipts (
            id, draft_session_id, workflow_run_id, plan_generation, round_index,
            round_seal_key, task_count, parallel_limit, plan_digest, request_digest,
            base_version_seq, base_data_hash, final_data_hash, resulting_version_seq, created_at
         ) VALUES (?1, ?2, ?3, 1, 0, ?4, 1, 1, ?5, ?6, 1, ?7, ?7, NULL, '2026-11-08T00:00:00Z');",
        params![round_id, draft_session_id, run_id, round_seal_key, plan_digest, request_digest, s0_hash],
    ).unwrap();

    conn.execute(
        "INSERT INTO workflow_task_receipts (
            id, draft_session_id, workflow_run_id, plan_generation, task_identity,
            input_scope_fingerprint, output_scope_fingerprint, base_version_seq, resulting_version_seq,
            had_mutations, round_receipt_id, task_index, evidence_json, created_at
         ) VALUES ('018e3a00-0000-7000-8000-000000000031', ?1, ?2, 1, 't1',
                   'in-fp-1', 'out-fp-1', 1, NULL, 0, ?3, 0, ?4, '2026-11-08T00:00:00Z');",
        params![draft_session_id, run_id, round_id, serde_json::to_string(&ev1).unwrap()],
    ).unwrap();

    // Verifier looks up strictly by round receipt ID without needing client request strings
    let res = verify_historical_round(&conn, round_id, &["v2"], 1);
    assert_eq!(res.integrity, Integrity::Verified);
    assert_eq!(res.attestation, Attestation::Attested);
}

#[test]
#[allow(non_snake_case)]
fn REC_07_and_REC_08_crash_before_vs_after_tx04_commit() {
    let (conn, _temp_dir) = setup_test_db();

    // REC-08: Crash before TX-04 commit -> round does not exist in SQLite
    let res_crash_before = verify_historical_round(&conn, "018e3a00-0000-7000-8000-000000000099", &["v2"], 1);
    assert_eq!(res_crash_before.integrity, Integrity::Corrupt);
    assert!(res_crash_before.details.unwrap().contains("Round header not found"));

    // REC-07: Crash after TX-04 commit -> round committed, verifies completely
    let draft_session_id = "018e3a00-0000-7000-8000-000000000010";
    seed_test_session(&conn, draft_session_id);
    let s0 = BusinessFlowData::new();
    let s0_hash = seed_draft_version(&conn, draft_session_id, 1, &s0, "phase_confirm", None, None);

    let round_id = "018e3a00-0000-7000-8000-000000000020";
    let run_id = "018e3a00-0000-7000-8000-000000000021";
    let round_seal_key = format!("wf_round:{}:1:0", run_id);

    let ev1 = EvidenceJsonV2 {
        schema_version: 2,
        codec_version: "v2".to_string(),
        planner_version: "v1".to_string(),
        policy_version: "v1".to_string(),
        task_spec: json!({ "taskIdentity": "t1" }),
        pre_state_hash: s0_hash.clone(),
        post_state_hash: s0_hash.clone(),
        had_applied_writes: false,
        effective_state_changed: false,
        patch: vec![],
        artifact: None,
    };

    let manifest_tasks = vec![
        ManifestTask::new(0, "t1", ev1.task_spec.clone(), "v1", "v1", "v2"),
    ];

    let plan_input = PlanDigestInput {
        draft_session_id: draft_session_id.to_string(),
        workflow_run_id: run_id.to_string(),
        plan_generation: 1,
        round_index: 0,
        task_count: 1,
        parallel_limit: 1,
        tasks: manifest_tasks,
    };
    let plan_digest = compute_plan_digest(&plan_input).unwrap();

    let req_input = RequestDigestInput {
        draft_session_id: draft_session_id.to_string(),
        workflow_run_id: run_id.to_string(),
        plan_generation: 1,
        round_index: 0,
        task_count: 1,
        parallel_limit: 1,
        plan_digest: plan_digest.clone(),
        base_version_seq: 1,
        base_data_hash: s0_hash.clone(),
    };
    let request_digest = compute_request_digest(&req_input).unwrap();

    conn.execute(
        "INSERT INTO workflow_round_receipts (
            id, draft_session_id, workflow_run_id, plan_generation, round_index,
            round_seal_key, task_count, parallel_limit, plan_digest, request_digest,
            base_version_seq, base_data_hash, final_data_hash, resulting_version_seq, created_at
         ) VALUES (?1, ?2, ?3, 1, 0, ?4, 1, 1, ?5, ?6, 1, ?7, ?7, NULL, '2026-11-08T00:00:00Z');",
        params![round_id, draft_session_id, run_id, round_seal_key, plan_digest, request_digest, s0_hash],
    ).unwrap();

    conn.execute(
        "INSERT INTO workflow_task_receipts (
            id, draft_session_id, workflow_run_id, plan_generation, task_identity,
            input_scope_fingerprint, output_scope_fingerprint, base_version_seq, resulting_version_seq,
            had_mutations, round_receipt_id, task_index, evidence_json, created_at
         ) VALUES ('018e3a00-0000-7000-8000-000000000031', ?1, ?2, 1, 't1',
                   'in-fp-1', 'out-fp-1', 1, NULL, 0, ?3, 0, ?4, '2026-11-08T00:00:00Z');",
        params![draft_session_id, run_id, round_id, serde_json::to_string(&ev1).unwrap()],
    ).unwrap();

    let res_crash_after = verify_historical_round(&conn, round_id, &["v2"], 1);
    assert_eq!(res_crash_after.integrity, Integrity::Verified);
    assert_eq!(res_crash_after.attestation, Attestation::Attested);
}

#[test]
#[allow(non_snake_case)]
fn REC_09_subsequent_downstream_task_overwrite_does_not_falsely_invalidate_history() {
    let (conn, _temp_dir) = setup_test_db();
    let draft_session_id = "018e3a00-0000-7000-8000-000000000010";
    seed_test_session(&conn, draft_session_id);

    let mut s0 = BusinessFlowData::new();
    s0.counters.insert("test_counter".to_string(), 1);
    let s0_hash = seed_draft_version(&conn, draft_session_id, 1, &s0, "phase_confirm", None, None);

    // Round 1 creates V2
    let mut s1 = s0.clone();
    s1.counters.insert("test_counter".to_string(), 2);
    let patch1 = canonical_diff(&s0, &s1).unwrap();
    let round1_id = "018e3a00-0000-7000-8000-000000000020";
    let run1_id = "018e3a00-0000-7000-8000-000000000021";
    let round1_seal_key = format!("wf_round:{}:1:0", run1_id);
    let s1_hash = seed_draft_version(&conn, draft_session_id, 2, &s1, "workflow_round", Some(1), Some(&round1_seal_key));

    let ev1 = EvidenceJsonV2 {
        schema_version: 2,
        codec_version: "v2".to_string(),
        planner_version: "v1".to_string(),
        policy_version: "v1".to_string(),
        task_spec: json!({ "taskIdentity": "t1" }),
        pre_state_hash: s0_hash.clone(),
        post_state_hash: s1_hash.clone(),
        had_applied_writes: true,
        effective_state_changed: true,
        patch: patch1.ops,
        artifact: None,
    };

    let manifest_tasks = vec![
        ManifestTask::new(0, "t1", ev1.task_spec.clone(), "v1", "v1", "v2"),
    ];

    let plan_input = PlanDigestInput {
        draft_session_id: draft_session_id.to_string(),
        workflow_run_id: run1_id.to_string(),
        plan_generation: 1,
        round_index: 0,
        task_count: 1,
        parallel_limit: 1,
        tasks: manifest_tasks,
    };
    let plan_digest = compute_plan_digest(&plan_input).unwrap();

    let req_input = RequestDigestInput {
        draft_session_id: draft_session_id.to_string(),
        workflow_run_id: run1_id.to_string(),
        plan_generation: 1,
        round_index: 0,
        task_count: 1,
        parallel_limit: 1,
        plan_digest: plan_digest.clone(),
        base_version_seq: 1,
        base_data_hash: s0_hash.clone(),
    };
    let request_digest = compute_request_digest(&req_input).unwrap();

    conn.execute(
        "INSERT INTO workflow_round_receipts (
            id, draft_session_id, workflow_run_id, plan_generation, round_index,
            round_seal_key, task_count, parallel_limit, plan_digest, request_digest,
            base_version_seq, base_data_hash, final_data_hash, resulting_version_seq, created_at
         ) VALUES (?1, ?2, ?3, 1, 0, ?4, 1, 1, ?5, ?6, 1, ?7, ?8, 2, '2026-11-08T00:00:00Z');",
        params![round1_id, draft_session_id, run1_id, round1_seal_key, plan_digest, request_digest, s0_hash, s1_hash],
    ).unwrap();

    conn.execute(
        "INSERT INTO workflow_task_receipts (
            id, draft_session_id, workflow_run_id, plan_generation, task_identity,
            input_scope_fingerprint, output_scope_fingerprint, base_version_seq, resulting_version_seq,
            had_mutations, round_receipt_id, task_index, evidence_json, created_at
         ) VALUES ('018e3a00-0000-7000-8000-000000000031', ?1, ?2, 1, 't1',
                   'in-fp-1', 'out-fp-1', 1, 2, 1, ?3, 0, ?4, '2026-11-08T00:00:00Z');",
        params![draft_session_id, run1_id, round1_id, serde_json::to_string(&ev1).unwrap()],
    ).unwrap();

    // Subsequent operation overwrites test_counter creating V3
    let mut s2 = s1.clone();
    s2.counters.insert("test_counter".to_string(), 999);
    seed_draft_version(&conn, draft_session_id, 3, &s2, "manual_edit", Some(2), None);

    // Current head is V3. Verify Round 1 (output was V2):
    // Must remain Verified historically, but applicability is Inapplicable (head advanced)
    let res = verify_historical_round(&conn, round1_id, &["v2"], 3);
    assert_eq!(res.integrity, Integrity::Verified);
    assert_eq!(res.attestation, Attestation::Attested);
    assert_eq!(res.applicability, Applicability::Inapplicable);
}

#[test]
#[allow(non_snake_case)]
fn REC_11_prefix_with_missing_suffix_fails_closed() {
    let (conn, _temp_dir) = setup_test_db();
    let draft_session_id = "018e3a00-0000-7000-8000-000000000010";
    seed_test_session(&conn, draft_session_id);

    let s0 = BusinessFlowData::new();
    let s0_hash = seed_draft_version(&conn, draft_session_id, 1, &s0, "phase_confirm", None, None);

    let round_id = "018e3a00-0000-7000-8000-000000000020";
    let run_id = "018e3a00-0000-7000-8000-000000000021";
    let round_seal_key = format!("wf_round:{}:1:0", run_id);

    // Header specifies task_count = 3 (prefix + suffix)
    conn.execute(
        "INSERT INTO workflow_round_receipts (
            id, draft_session_id, workflow_run_id, plan_generation, round_index,
            round_seal_key, task_count, parallel_limit, plan_digest, request_digest,
            base_version_seq, base_data_hash, final_data_hash, resulting_version_seq, created_at
         ) VALUES (?1, ?2, ?3, 1, 0, ?4, 3, 3, 'plan-d', 'req-d', 1, ?5, ?5, NULL, '2026-11-08T00:00:00Z');",
        params![round_id, draft_session_id, run_id, round_seal_key, s0_hash],
    ).unwrap();

    let ev1 = EvidenceJsonV2 {
        schema_version: 2,
        codec_version: "v2".to_string(),
        planner_version: "v1".to_string(),
        policy_version: "v1".to_string(),
        task_spec: json!({ "taskIdentity": "t1" }),
        pre_state_hash: s0_hash.clone(),
        post_state_hash: s0_hash.clone(),
        had_applied_writes: false,
        effective_state_changed: false,
        patch: vec![],
        artifact: None,
    };

    // Only prefix task 0 is committed; task 1 and 2 are missing
    conn.execute(
        "INSERT INTO workflow_task_receipts (
            id, draft_session_id, workflow_run_id, plan_generation, task_identity,
            input_scope_fingerprint, output_scope_fingerprint, base_version_seq, resulting_version_seq,
            had_mutations, round_receipt_id, task_index, evidence_json, created_at
         ) VALUES ('018e3a00-0000-7000-8000-000000000031', ?1, ?2, 1, 't1',
                   'in-fp-1', 'out-fp-1', 1, NULL, 0, ?3, 0, ?4, '2026-11-08T00:00:00Z');",
        params![draft_session_id, run_id, round_id, serde_json::to_string(&ev1).unwrap()],
    ).unwrap();

    let res = verify_historical_round(&conn, round_id, &["v2"], 1);
    assert_eq!(res.integrity, Integrity::Corrupt);
    assert!(res.details.unwrap().contains("Task count mismatch"));
}

#[test]
#[allow(non_snake_case)]
fn REC_12_interleaved_or_out_of_order_task_indices_rejected() {
    let (conn, _temp_dir) = setup_test_db();
    let draft_session_id = "018e3a00-0000-7000-8000-000000000010";
    seed_test_session(&conn, draft_session_id);

    let s0 = BusinessFlowData::new();
    let s0_hash = seed_draft_version(&conn, draft_session_id, 1, &s0, "phase_confirm", None, None);

    let round_id = "018e3a00-0000-7000-8000-000000000020";
    let run_id = "018e3a00-0000-7000-8000-000000000021";
    let round_seal_key = format!("wf_round:{}:1:0", run_id);

    conn.execute(
        "INSERT INTO workflow_round_receipts (
            id, draft_session_id, workflow_run_id, plan_generation, round_index,
            round_seal_key, task_count, parallel_limit, plan_digest, request_digest,
            base_version_seq, base_data_hash, final_data_hash, resulting_version_seq, created_at
         ) VALUES (?1, ?2, ?3, 1, 0, ?4, 2, 2, 'plan-d', 'req-d', 1, ?5, ?5, NULL, '2026-11-08T00:00:00Z');",
        params![round_id, draft_session_id, run_id, round_seal_key, s0_hash],
    ).unwrap();

    let ev1 = EvidenceJsonV2 {
        schema_version: 2,
        codec_version: "v2".to_string(),
        planner_version: "v1".to_string(),
        policy_version: "v1".to_string(),
        task_spec: json!({ "taskIdentity": "t1" }),
        pre_state_hash: s0_hash.clone(),
        post_state_hash: s0_hash.clone(),
        had_applied_writes: false,
        effective_state_changed: false,
        patch: vec![],
        artifact: None,
    };

    // Insert task 1 and task 2 (indices 1 and 2 instead of 0 and 1)
    conn.execute(
        "INSERT INTO workflow_task_receipts (
            id, draft_session_id, workflow_run_id, plan_generation, task_identity,
            input_scope_fingerprint, output_scope_fingerprint, base_version_seq, resulting_version_seq,
            had_mutations, round_receipt_id, task_index, evidence_json, created_at
         ) VALUES ('018e3a00-0000-7000-8000-000000000031', ?1, ?2, 1, 't1',
                   'in-fp-1', 'out-fp-1', 1, NULL, 0, ?3, 1, ?4, '2026-11-08T00:00:00Z');",
        params![draft_session_id, run_id, round_id, serde_json::to_string(&ev1).unwrap()],
    ).unwrap();

    conn.execute(
        "INSERT INTO workflow_task_receipts (
            id, draft_session_id, workflow_run_id, plan_generation, task_identity,
            input_scope_fingerprint, output_scope_fingerprint, base_version_seq, resulting_version_seq,
            had_mutations, round_receipt_id, task_index, evidence_json, created_at
         ) VALUES ('018e3a00-0000-7000-8000-000000000032', ?1, ?2, 1, 't2',
                   'in-fp-2', 'out-fp-2', 1, NULL, 0, ?3, 2, ?4, '2026-11-08T00:00:00Z');",
        params![draft_session_id, run_id, round_id, serde_json::to_string(&ev1).unwrap()],
    ).unwrap();

    let res = verify_historical_round(&conn, round_id, &["v2"], 1);
    assert_eq!(res.integrity, Integrity::Corrupt);
    assert!(res.details.unwrap().contains("Task index gap/mismatch"));
}

#[test]
#[allow(non_snake_case)]
fn REC_13_caller_forged_mutations_hash_rejected() {
    let (conn, _temp_dir) = setup_test_db();
    let draft_session_id = "018e3a00-0000-7000-8000-000000000010";
    seed_test_session(&conn, draft_session_id);

    let mut s0 = BusinessFlowData::new();
    s0.counters.insert("test_counter".to_string(), 1);
    let s0_hash = seed_draft_version(&conn, draft_session_id, 1, &s0, "phase_confirm", None, None);

    let mut s1 = s0.clone();
    s1.counters.insert("test_counter".to_string(), 2);
    let patch1 = canonical_diff(&s0, &s1).unwrap();

    let round_id = "018e3a00-0000-7000-8000-000000000020";
    let run_id = "018e3a00-0000-7000-8000-000000000021";
    let round_seal_key = format!("wf_round:{}:1:0", run_id);
    seed_draft_version(&conn, draft_session_id, 2, &s1, "workflow_round", Some(1), Some(&round_seal_key));

    // Forged post_state_hash declared by untrusted caller
    let fake_post_hash = "sha256:ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff";

    let ev1 = EvidenceJsonV2 {
        schema_version: 2,
        codec_version: "v2".to_string(),
        planner_version: "v1".to_string(),
        policy_version: "v1".to_string(),
        task_spec: json!({ "taskIdentity": "t1" }),
        pre_state_hash: s0_hash.clone(),
        post_state_hash: fake_post_hash.to_string(),
        had_applied_writes: true,
        effective_state_changed: true,
        patch: patch1.ops,
        artifact: None,
    };

    let manifest_tasks = vec![
        ManifestTask::new(0, "t1", ev1.task_spec.clone(), "v1", "v1", "v2"),
    ];

    let plan_input = PlanDigestInput {
        draft_session_id: draft_session_id.to_string(),
        workflow_run_id: run_id.to_string(),
        plan_generation: 1,
        round_index: 0,
        task_count: 1,
        parallel_limit: 1,
        tasks: manifest_tasks,
    };
    let plan_digest = compute_plan_digest(&plan_input).unwrap();

    let req_input = RequestDigestInput {
        draft_session_id: draft_session_id.to_string(),
        workflow_run_id: run_id.to_string(),
        plan_generation: 1,
        round_index: 0,
        task_count: 1,
        parallel_limit: 1,
        plan_digest: plan_digest.clone(),
        base_version_seq: 1,
        base_data_hash: s0_hash.clone(),
    };
    let request_digest = compute_request_digest(&req_input).unwrap();

    conn.execute(
        "INSERT INTO workflow_round_receipts (
            id, draft_session_id, workflow_run_id, plan_generation, round_index,
            round_seal_key, task_count, parallel_limit, plan_digest, request_digest,
            base_version_seq, base_data_hash, final_data_hash, resulting_version_seq, created_at
         ) VALUES (?1, ?2, ?3, 1, 0, ?4, 1, 1, ?5, ?6, 1, ?7, ?8, 2, '2026-11-08T00:00:00Z');",
        params![round_id, draft_session_id, run_id, round_seal_key, plan_digest, request_digest, s0_hash, fake_post_hash],
    ).unwrap();

    conn.execute(
        "INSERT INTO workflow_task_receipts (
            id, draft_session_id, workflow_run_id, plan_generation, task_identity,
            input_scope_fingerprint, output_scope_fingerprint, base_version_seq, resulting_version_seq,
            had_mutations, round_receipt_id, task_index, evidence_json, created_at
         ) VALUES ('018e3a00-0000-7000-8000-000000000031', ?1, ?2, 1, 't1',
                   'in-fp-1', 'out-fp-1', 1, 2, 1, ?3, 0, ?4, '2026-11-08T00:00:00Z');",
        params![draft_session_id, run_id, round_id, serde_json::to_string(&ev1).unwrap()],
    ).unwrap();

    let res = verify_historical_round(&conn, round_id, &["v2"], 2);
    assert_eq!(res.integrity, Integrity::Corrupt);
    assert!(
        res.details.as_ref().unwrap().contains("PatchPreconditionFailed")
            || res.details.as_ref().unwrap().contains("post_state_hash mismatch"),
        "Actual details: {:?}",
        res.details
    );
}

#[test]
#[allow(non_snake_case)]
fn REC_14_missing_or_corrupted_intermediate_evidence_requires_replan() {
    let (conn, _temp_dir) = setup_test_db();
    let draft_session_id = "018e3a00-0000-7000-8000-000000000010";
    seed_test_session(&conn, draft_session_id);

    let s0 = BusinessFlowData::new();
    let s0_hash = seed_draft_version(&conn, draft_session_id, 1, &s0, "phase_confirm", None, None);

    let round_id = "018e3a00-0000-7000-8000-000000000020";
    let run_id = "018e3a00-0000-7000-8000-000000000021";
    let round_seal_key = format!("wf_round:{}:1:0", run_id);

    conn.execute(
        "INSERT INTO workflow_round_receipts (
            id, draft_session_id, workflow_run_id, plan_generation, round_index,
            round_seal_key, task_count, parallel_limit, plan_digest, request_digest,
            base_version_seq, base_data_hash, final_data_hash, resulting_version_seq, created_at
         ) VALUES (?1, ?2, ?3, 1, 0, ?4, 1, 1, 'plan-d', 'req-d', 1, ?5, ?5, NULL, '2026-11-08T00:00:00Z');",
        params![round_id, draft_session_id, run_id, round_seal_key, s0_hash],
    ).unwrap();

    // Disable check constraints to simulate non-JSON corruption in SQLite
    conn.execute("PRAGMA ignore_check_constraints = ON;", []).unwrap();
    conn.execute(
        "INSERT INTO workflow_task_receipts (
            id, draft_session_id, workflow_run_id, plan_generation, task_identity,
            input_scope_fingerprint, output_scope_fingerprint, base_version_seq, resulting_version_seq,
            had_mutations, round_receipt_id, task_index, evidence_json, created_at
         ) VALUES ('018e3a00-0000-7000-8000-000000000031', ?1, ?2, 1, 't1',
                   'in-fp-1', 'out-fp-1', 1, NULL, 0, ?3, 0, '{\"broken_json\":', '2026-11-08T00:00:00Z');",
        params![draft_session_id, run_id, round_id],
    ).unwrap();
    conn.execute("PRAGMA ignore_check_constraints = OFF;", []).unwrap();

    let res = verify_historical_round(&conn, round_id, &["v2"], 1);
    assert_eq!(res.integrity, Integrity::Corrupt);
    assert!(res.details.unwrap().contains("Malformed evidence_json"));
}

#[test]
#[allow(non_snake_case)]
fn REC_15_full_round_retry_replays_durable_patches_without_duplication() {
    let (conn, _temp_dir) = setup_test_db();
    let draft_session_id = "018e3a00-0000-7000-8000-000000000010";
    seed_test_session(&conn, draft_session_id);

    let mut s0 = BusinessFlowData::new();
    s0.counters.insert("test_counter".to_string(), 1);
    let s0_hash = seed_draft_version(&conn, draft_session_id, 1, &s0, "phase_confirm", None, None);

    let mut s1 = s0.clone();
    s1.counters.insert("test_counter".to_string(), 2);
    let patch1 = canonical_diff(&s0, &s1).unwrap();
    let round_id = "018e3a00-0000-7000-8000-000000000020";
    let run_id = "018e3a00-0000-7000-8000-000000000021";
    let round_seal_key = format!("wf_round:{}:1:0", run_id);
    let s1_hash = seed_draft_version(&conn, draft_session_id, 2, &s1, "workflow_round", Some(1), Some(&round_seal_key));

    let ev1 = EvidenceJsonV2 {
        schema_version: 2,
        codec_version: "v2".to_string(),
        planner_version: "v1".to_string(),
        policy_version: "v1".to_string(),
        task_spec: json!({ "taskIdentity": "t1" }),
        pre_state_hash: s0_hash.clone(),
        post_state_hash: s1_hash.clone(),
        had_applied_writes: true,
        effective_state_changed: true,
        patch: patch1.ops,
        artifact: None,
    };

    let manifest_tasks = vec![
        ManifestTask::new(0, "t1", ev1.task_spec.clone(), "v1", "v1", "v2"),
    ];

    let plan_input = PlanDigestInput {
        draft_session_id: draft_session_id.to_string(),
        workflow_run_id: run_id.to_string(),
        plan_generation: 1,
        round_index: 0,
        task_count: 1,
        parallel_limit: 1,
        tasks: manifest_tasks,
    };
    let plan_digest = compute_plan_digest(&plan_input).unwrap();

    let req_input = RequestDigestInput {
        draft_session_id: draft_session_id.to_string(),
        workflow_run_id: run_id.to_string(),
        plan_generation: 1,
        round_index: 0,
        task_count: 1,
        parallel_limit: 1,
        plan_digest: plan_digest.clone(),
        base_version_seq: 1,
        base_data_hash: s0_hash.clone(),
    };
    let request_digest = compute_request_digest(&req_input).unwrap();

    conn.execute(
        "INSERT INTO workflow_round_receipts (
            id, draft_session_id, workflow_run_id, plan_generation, round_index,
            round_seal_key, task_count, parallel_limit, plan_digest, request_digest,
            base_version_seq, base_data_hash, final_data_hash, resulting_version_seq, created_at
         ) VALUES (?1, ?2, ?3, 1, 0, ?4, 1, 1, ?5, ?6, 1, ?7, ?8, 2, '2026-11-08T00:00:00Z');",
        params![round_id, draft_session_id, run_id, round_seal_key, plan_digest, request_digest, s0_hash, s1_hash],
    ).unwrap();

    conn.execute(
        "INSERT INTO workflow_task_receipts (
            id, draft_session_id, workflow_run_id, plan_generation, task_identity,
            input_scope_fingerprint, output_scope_fingerprint, base_version_seq, resulting_version_seq,
            had_mutations, round_receipt_id, task_index, evidence_json, created_at
         ) VALUES ('018e3a00-0000-7000-8000-000000000031', ?1, ?2, 1, 't1',
                   'in-fp-1', 'out-fp-1', 1, 2, 1, ?3, 0, ?4, '2026-11-08T00:00:00Z');",
        params![draft_session_id, run_id, round_id, serde_json::to_string(&ev1).unwrap()],
    ).unwrap();

    let count_before: i64 = conn.query_row("SELECT COUNT(*) FROM draft_versions", [], |r| r.get(0)).unwrap();

    let res = verify_historical_round(&conn, round_id, &["v2"], 2);
    assert_eq!(res.integrity, Integrity::Verified);
    assert_eq!(res.attestation, Attestation::Attested);
    assert_eq!(res.applicability, Applicability::Applicable);

    let count_after: i64 = conn.query_row("SELECT COUNT(*) FROM draft_versions", [], |r| r.get(0)).unwrap();
    assert_eq!(count_before, count_after, "Verification must be strictly read-only and create zero new versions");
}
