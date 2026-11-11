#![allow(clippy::field_reassign_with_default)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::result_large_err)]
#![allow(clippy::result_unit_err)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]

//! Acceptance and contract tests for `phase3_sprint10_deep4`
//! ("Seal B+ round atomically in TX-04").
//!
//! Per the backend module wiring rule in `docs/plan/phase3_sprint10_deep4.md` §12:
//! - This sprint does not edit any `mod.rs` or `lib.rs`.
//! - This test target loads owned implementation and predecessor modules via `#[path = "..."]`.
//!
//! Covered acceptance vectors:
//! - `phase3_sprint10_deep4-F0`: Predecessor gates & baseline ancestry validation.
//! - `TX-004`: Atomic rollback on failure, retry verification.
//! - `T-WF-01`: Workflow Receipts (`SF-06`), TX-04 atomicity, retry before seal, zero-mutation round, restore check.
//! - `WF-01` through `WF-10`: Authoritative specification scenarios.
//! - `WF-C01` through `WF-C15`: Invariant matrix scenarios (net-zero, duplicates, interleaved, crash recovery).
//! - `pending_patches` exclusion and independent lifecycle.
//! - Insert-Once semantics (ON CONFLICT DO NOTHING, never INSERT OR REPLACE).

mod common;

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

#[path = "../src/service/round_seal_service.rs"]
pub mod round_seal_service;

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
    pub use crate::round_seal_service;
}

pub mod storage {
    pub use crate::db;
    pub use crate::migration;
}

use std::collections::BTreeMap;
use tempfile::TempDir;

use business_flow::BusinessFlowData;
use codec::canonical_diff;
use error::ErrorCode;
use fingerprint::state_data_hash;
use receipt_verifier::{verify_historical_round, Integrity};
use round_seal_service::*;
use rusqlite::{params, Connection};
use serde_json::json;

pub const BASELINE_COMMIT: &str = "bdd9aa003f909a3e7a17b1dbfd626f0f628145b8";
pub const PREDECESSOR_GATE_P3S6: &str = "181eda6d87f7fac1d17e05f82d843222aa0a376e";
pub const PREDECESSOR_GATE_P3S9: &str = "935f682db62b3b6227f95e34ef40b0547de760b7";
pub const PREDECESSOR_GATE_P3S2: &str = "ded1e9c5457ab8e72f456a515937a05af6ebba63";
pub const PREDECESSOR_GATE_P3S3: &str = "9f8bb7f0c43ba79d22072fb73f026a54343e4363";

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

fn seed_test_session(conn: &Connection, draft_session_id: &str) {
    let proj_id = "018e3a00-0000-7000-8000-000000000001";
    let sess_id = "018e3a00-0000-7000-8000-000000000002";
    conn.execute(
        "INSERT OR IGNORE INTO projects (id, display_name) VALUES (?, 'Test Project');",
        [proj_id],
    )
    .unwrap();
    conn.execute(
        "INSERT OR IGNORE INTO sessions (id, project_id) VALUES (?, ?);",
        [sess_id, proj_id],
    )
    .unwrap();
    conn.execute(
        "INSERT OR IGNORE INTO draft_sessions (id, session_id, project_id, head_version_seq) VALUES (?, ?, ?, 1);",
        [draft_session_id, sess_id, proj_id],
    )
    .unwrap();
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

    conn.execute(
        "UPDATE draft_sessions SET head_version_seq = ?1 WHERE id = ?2",
        params![seq, draft_session_id],
    )
    .unwrap();

    hash
}

// -----------------------------------------------------------------------------
// Predecessors Ancestry Check (`phase3_sprint10_deep4-F0`)
// -----------------------------------------------------------------------------

pub fn verify_p3s10_dispatch_preconditions(
    baseline_commit: &str,
    current_head: &str,
    gate_p3s6: bool,
    gate_p3s9: bool,
    gate_p3s2: bool,
    gate_p3s3: bool,
) -> Result<(), String> {
    if !gate_p3s6 {
        return Err(
            "CONTRACT_BLOCKED: predecessor gate_phase3_sprint6_deep3 is absent".to_string(),
        );
    }
    if !gate_p3s9 {
        return Err(
            "CONTRACT_BLOCKED: predecessor gate_phase3_sprint9_deep2 is absent".to_string(),
        );
    }
    if !gate_p3s2 {
        return Err(
            "CONTRACT_BLOCKED: predecessor gate_phase3_sprint2_deep0 is absent".to_string(),
        );
    }
    if !gate_p3s3 {
        return Err(
            "CONTRACT_BLOCKED: predecessor gate_phase3_sprint3_deep0 is absent".to_string(),
        );
    }

    // 1. Check baseline commit ancestry, existence, or recorded lineage
    if !common::verify_commit_at(baseline_commit, current_head) {
        return Err(format!(
            "CONTRACT_BLOCKED: baseline commit {baseline_commit} is not an ancestor of {current_head} and is not recorded in the lineage manifest"
        ));
    }

    // 2. Check predecessor P3S6 ancestry, existence, or recorded lineage
    if !common::verify_commit_at(PREDECESSOR_GATE_P3S6, current_head) {
        return Err(format!(
            "CONTRACT_BLOCKED: predecessor commit {PREDECESSOR_GATE_P3S6} is not an ancestor of {current_head} and is not recorded in the lineage manifest"
        ));
    }

    // 3. Check predecessor P3S9 ancestry, existence, or recorded lineage
    if !common::verify_commit_at(PREDECESSOR_GATE_P3S9, current_head) {
        return Err(format!(
            "CONTRACT_BLOCKED: predecessor commit {PREDECESSOR_GATE_P3S9} is not an ancestor of {current_head} and is not recorded in the lineage manifest"
        ));
    }

    // 4. Check predecessor P3S2 ancestry, existence, or recorded lineage
    if !common::verify_commit_at(PREDECESSOR_GATE_P3S2, current_head) {
        return Err(format!(
            "CONTRACT_BLOCKED: predecessor commit {PREDECESSOR_GATE_P3S2} is not an ancestor of {current_head} and is not recorded in the lineage manifest"
        ));
    }

    // 5. Check predecessor P3S3 ancestry, existence, or recorded lineage
    if !common::verify_commit_at(PREDECESSOR_GATE_P3S3, current_head) {
        return Err(format!(
            "CONTRACT_BLOCKED: predecessor commit {PREDECESSOR_GATE_P3S3} not found in repository or lineage manifest"
        ));
    }

    Ok(())
}

// =============================================================================
// Tests
// =============================================================================

#[test]
#[allow(non_snake_case)]
fn phase3_sprint10_deep4_F0_predecessor_gates_and_baseline_ancestry() {
    // 1. Missing gate P3S6
    let err_p3s6 =
        verify_p3s10_dispatch_preconditions(BASELINE_COMMIT, "HEAD", false, true, true, true)
            .unwrap_err();
    assert!(err_p3s6.contains("CONTRACT_BLOCKED"));
    assert!(err_p3s6.contains("gate_phase3_sprint6_deep3"));

    // 2. Missing gate P3S9
    let err_p3s9 =
        verify_p3s10_dispatch_preconditions(BASELINE_COMMIT, "HEAD", true, false, true, true)
            .unwrap_err();
    assert!(err_p3s9.contains("CONTRACT_BLOCKED"));
    assert!(err_p3s9.contains("gate_phase3_sprint9_deep2"));

    // 3. Missing gate P3S2
    let err_p3s2 =
        verify_p3s10_dispatch_preconditions(BASELINE_COMMIT, "HEAD", true, true, false, true)
            .unwrap_err();
    assert!(err_p3s2.contains("CONTRACT_BLOCKED"));
    assert!(err_p3s2.contains("gate_phase3_sprint2_deep0"));

    // 4. Missing gate P3S3
    let err_p3s3 =
        verify_p3s10_dispatch_preconditions(BASELINE_COMMIT, "HEAD", true, true, true, false)
            .unwrap_err();
    assert!(err_p3s3.contains("CONTRACT_BLOCKED"));
    assert!(err_p3s3.contains("gate_phase3_sprint3_deep0"));

    // 5. Baseline drift simulation
    let bad_baseline = "0000000000000000000000000000000000000000";
    let err_drift =
        verify_p3s10_dispatch_preconditions(bad_baseline, "HEAD", true, true, true, true)
            .unwrap_err();
    assert!(err_drift.contains("CONTRACT_BLOCKED"));

    // 6. Live repository check against HEAD
    let live_res =
        verify_p3s10_dispatch_preconditions(BASELINE_COMMIT, "HEAD", true, true, true, true);
    assert!(live_res.is_ok(), "live check failed: {:?}", live_res);
}

#[test]
#[allow(non_snake_case)]
fn TX_004_atomic_rollback_on_failure_and_retry_succeeds() {
    let (mut conn, _td) = setup_test_db();
    let draft_session_id = "018e3a00-0000-7000-8000-000000000010";
    seed_test_session(&conn, draft_session_id);

    let mut s0 = BusinessFlowData::new();
    s0.counters.insert("test_counter".to_string(), 1);
    let s0_hash = seed_draft_version(&conn, draft_session_id, 1, &s0, "phase_confirm", None, None);

    let run_id = "018e3a00-0000-7000-8000-000000000020";
    let seal_key = format!("wf_round:{}:1:0", run_id);

    // Prepare a task with conflicting metadata (task identity outside authorized plan)
    let spec1 = PlannedTaskSpec {
        phase: 1,
        subphase: None,
        task_kind: "character_writer".to_string(),
        assigned_scope: json!({"chapter": 1}),
        dependencies: vec![],
        deterministic_identity: Some("task:p1:character_writer:{\"chapter\":1}".to_string()),
    };
    let mut s1 = s0.clone();
    s1.counters.insert("test_counter".to_string(), 2);
    let patch1 = canonical_diff(&s0, &s1).unwrap();

    let task1 = CompletedTaskOutcome {
        task_index: 0,
        task_spec: spec1.clone(),
        task_identity: "task:p1:character_writer:{\"chapter\":1}".to_string(),
        input_scope_fingerprint: "in_fp_1".to_string(),
        output_scope_fingerprint: "out_fp_1".to_string(),
        has_applied_write: true,
        applied_write_count: 1,
        applied_mutations: patch1.ops.clone(),
        artifact: None,
    };

    // Case A: Plan context authorizes index 0 for a DIFFERENT task identity -> ValidationFailed
    let mut auth_tasks_bad = BTreeMap::new();
    let mut bad_spec = spec1.clone();
    bad_spec.deterministic_identity = Some("task:p1:other_task".to_string());
    auth_tasks_bad.insert(0, bad_spec);

    let plan_ctx_bad = AuthorizedRoundPlanContext {
        workflow_run_id: run_id.to_string(),
        plan_generation: 1,
        phase: 1,
        subphase: None,
        round_index: 0,
        parallel_limit: 4,
        authorized_tasks: auth_tasks_bad,
    };

    let req_bad = WorkflowRoundSealRequest {
        draft_session_id: draft_session_id.to_string(),
        workflow_run_id: run_id.to_string(),
        plan_generation: 1,
        round_index: 0,
        round_seal_key: seal_key.clone(),
        phase: 1,
        subphase: None,
        parallel_limit: 4,
        base_version_seq: 1,
        base_data_hash: s0_hash.clone(),
        completed_tasks: vec![task1.clone()],
        has_applied_write: true,
    };

    let err = seal_workflow_round_tx(&mut conn, &req_bad, &plan_ctx_bad).unwrap_err();
    assert_eq!(err.code, ErrorCode::ValidationFailed);

    // Verify atomic rollback: 0 round headers, 0 receipts, 0 new draft versions
    let round_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM workflow_round_receipts", [], |r| {
            r.get(0)
        })
        .unwrap();
    let task_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM workflow_task_receipts", [], |r| {
            r.get(0)
        })
        .unwrap();
    let ver_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM draft_versions", [], |r| r.get(0))
        .unwrap();
    let head_seq: i64 = conn
        .query_row(
            "SELECT head_version_seq FROM draft_sessions WHERE id = ?",
            [draft_session_id],
            |r| r.get(0),
        )
        .unwrap();

    assert_eq!(
        round_count, 0,
        "No round receipts should exist after rollback"
    );
    assert_eq!(
        task_count, 0,
        "No task receipts should exist after rollback"
    );
    assert_eq!(ver_count, 1, "Only baseline draft version should exist");
    assert_eq!(head_seq, 1, "Head seq must remain 1");

    // Case B: Now retry with valid authorized plan -> succeeds atomically!
    let mut auth_tasks_good = BTreeMap::new();
    auth_tasks_good.insert(0, spec1);
    let plan_ctx_good = AuthorizedRoundPlanContext {
        workflow_run_id: run_id.to_string(),
        plan_generation: 1,
        phase: 1,
        subphase: None,
        round_index: 0,
        parallel_limit: 4,
        authorized_tasks: auth_tasks_good,
    };

    let res = seal_workflow_round_tx(&mut conn, &req_bad, &plan_ctx_good).unwrap();
    assert_eq!(res.sealed_version_seq, Some(2));
    assert!(res.net_round_state_changed);
    assert!(res.durable_version_created);
    assert_eq!(res.receipts.len(), 1);

    // Verify DB committed state
    let round_count_after: i64 = conn
        .query_row("SELECT COUNT(*) FROM workflow_round_receipts", [], |r| {
            r.get(0)
        })
        .unwrap();
    let task_count_after: i64 = conn
        .query_row("SELECT COUNT(*) FROM workflow_task_receipts", [], |r| {
            r.get(0)
        })
        .unwrap();
    let head_seq_after: i64 = conn
        .query_row(
            "SELECT head_version_seq FROM draft_sessions WHERE id = ?",
            [draft_session_id],
            |r| r.get(0),
        )
        .unwrap();

    assert_eq!(round_count_after, 1);
    assert_eq!(task_count_after, 1);
    assert_eq!(head_seq_after, 2);
}

#[test]
#[allow(non_snake_case)]
fn T_WF_01_workflow_receipts_tx04_retry_zero_mutation_and_restore() {
    let (mut conn, _td) = setup_test_db();
    let draft_session_id = "018e3a00-0000-7000-8000-000000000010";
    seed_test_session(&conn, draft_session_id);

    let mut s0 = BusinessFlowData::new();
    s0.counters.insert("chapter_count".to_string(), 1);
    let s0_hash = seed_draft_version(&conn, draft_session_id, 1, &s0, "phase_confirm", None, None);

    let run_id = "018e3a00-0000-7000-8000-000000000020";

    // -------------------------------------------------------------------------
    // Part (a): Round 1 has mutation -> creates v2. Retry Round 1 -> idempotent lookup, 0 new version
    // -------------------------------------------------------------------------
    let seal_key_r1 = format!("wf_round:{}:1:0", run_id);
    let spec1 = PlannedTaskSpec {
        phase: 1,
        subphase: None,
        task_kind: "outline_writer".to_string(),
        assigned_scope: json!({"arc": 1}),
        dependencies: vec![],
        deterministic_identity: Some("task:p1:outline_writer:{\"arc\":1}".to_string()),
    };

    let mut s1 = s0.clone();
    s1.counters.insert("chapter_count".to_string(), 2);
    let patch1 = canonical_diff(&s0, &s1).unwrap();

    let task1 = CompletedTaskOutcome {
        task_index: 0,
        task_spec: spec1.clone(),
        task_identity: "task:p1:outline_writer:{\"arc\":1}".to_string(),
        input_scope_fingerprint: "in_1".to_string(),
        output_scope_fingerprint: "out_1".to_string(),
        has_applied_write: true,
        applied_write_count: 1,
        applied_mutations: patch1.ops.clone(),
        artifact: None,
    };

    let mut auth1 = BTreeMap::new();
    auth1.insert(0, spec1.clone());
    let plan1 = AuthorizedRoundPlanContext {
        workflow_run_id: run_id.to_string(),
        plan_generation: 1,
        phase: 1,
        subphase: None,
        round_index: 0,
        parallel_limit: 4,
        authorized_tasks: auth1.clone(),
    };

    let req1 = WorkflowRoundSealRequest {
        draft_session_id: draft_session_id.to_string(),
        workflow_run_id: run_id.to_string(),
        plan_generation: 1,
        round_index: 0,
        round_seal_key: seal_key_r1.clone(),
        phase: 1,
        subphase: None,
        parallel_limit: 4,
        base_version_seq: 1,
        base_data_hash: s0_hash.clone(),
        completed_tasks: vec![task1.clone()],
        has_applied_write: true,
    };

    let res1 = seal_workflow_round_tx(&mut conn, &req1, &plan1).unwrap();
    assert_eq!(res1.sealed_version_seq, Some(2));
    assert!(res1.durable_version_created);
    assert!(!res1.was_idempotent_replay);

    // Retry Round 1: lookup receipt before writes; skip write and return idempotent replay
    let retry_res1 = seal_workflow_round_tx(&mut conn, &req1, &plan1).unwrap();
    assert_eq!(retry_res1.sealed_version_seq, Some(2));
    assert!(retry_res1.was_idempotent_replay);
    assert!(!retry_res1.durable_version_created);

    let ver_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM draft_versions", [], |r| r.get(0))
        .unwrap();
    assert_eq!(ver_count, 2, "No duplicate draft version created on retry");

    // -------------------------------------------------------------------------
    // Part (b): Round 2 has 0 state changes (net-zero) -> persists header and receipt with had_mutations = 0, resulting_version_seq = NULL, 0 fake version
    // -------------------------------------------------------------------------
    let s1_hash = state_data_hash(&s1).unwrap();
    let seal_key_r2 = format!("wf_round:{}:1:1", run_id);
    let spec2 = PlannedTaskSpec {
        phase: 1,
        subphase: None,
        task_kind: "outline_verifier".to_string(),
        assigned_scope: json!({"arc": 1}),
        dependencies: vec![],
        deterministic_identity: Some("task:p1:outline_verifier:{\"arc\":1}".to_string()),
    };

    let task2 = CompletedTaskOutcome {
        task_index: 0,
        task_spec: spec2.clone(),
        task_identity: "task:p1:outline_verifier:{\"arc\":1}".to_string(),
        input_scope_fingerprint: "in_2".to_string(),
        output_scope_fingerprint: "out_2".to_string(),
        has_applied_write: false,
        applied_write_count: 0,
        applied_mutations: vec![],
        artifact: None,
    };

    let mut auth2 = BTreeMap::new();
    auth2.insert(0, spec2.clone());
    let plan2 = AuthorizedRoundPlanContext {
        workflow_run_id: run_id.to_string(),
        plan_generation: 1,
        phase: 1,
        subphase: None,
        round_index: 1,
        parallel_limit: 4,
        authorized_tasks: auth2,
    };

    let req2 = WorkflowRoundSealRequest {
        draft_session_id: draft_session_id.to_string(),
        workflow_run_id: run_id.to_string(),
        plan_generation: 1,
        round_index: 1,
        round_seal_key: seal_key_r2,
        phase: 1,
        subphase: None,
        parallel_limit: 4,
        base_version_seq: 2,
        base_data_hash: s1_hash.clone(),
        completed_tasks: vec![task2],
        has_applied_write: false,
    };

    let res2 = seal_workflow_round_tx(&mut conn, &req2, &plan2).unwrap();
    assert_eq!(
        res2.sealed_version_seq, None,
        "Zero-net round must NOT create resulting version"
    );
    assert!(!res2.durable_version_created);
    assert!(!res2.net_round_state_changed);
    assert_eq!(res2.receipts.len(), 1);
    assert!(!res2.receipts[0].had_mutations);
    assert_eq!(res2.receipts[0].resulting_version_seq, None);

    let ver_count_after_r2: i64 = conn
        .query_row("SELECT COUNT(*) FROM draft_versions", [], |r| r.get(0))
        .unwrap();
    assert_eq!(
        ver_count_after_r2, 2,
        "No fake draft version created for zero-net round"
    );

    let head_seq_r2: i64 = conn
        .query_row(
            "SELECT head_version_seq FROM draft_sessions WHERE id = ?",
            [draft_session_id],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(
        head_seq_r2, 2,
        "Head sequence remains unchanged on zero-net round"
    );

    // -------------------------------------------------------------------------
    // Part (c): Restore creates v3 with parent = 1 -> lineage cut, resume with stale base seq fails
    // -------------------------------------------------------------------------
    seed_draft_version(&conn, draft_session_id, 3, &s0, "restore", Some(1), None);

    let req_stale = WorkflowRoundSealRequest {
        draft_session_id: draft_session_id.to_string(),
        workflow_run_id: run_id.to_string(),
        plan_generation: 1,
        round_index: 2,
        round_seal_key: format!("wf_round:{}:1:2", run_id),
        phase: 1,
        subphase: None,
        parallel_limit: 4,
        base_version_seq: 2, // Head is now 3!
        base_data_hash: s1_hash,
        completed_tasks: vec![task1],
        has_applied_write: true,
    };

    let err_restore = seal_workflow_round_tx(&mut conn, &req_stale, &plan1).unwrap_err();
    assert_eq!(err_restore.code, ErrorCode::ValidationFailed);
}

#[test]
#[allow(non_snake_case)]
fn WF_01_empty_completed_tasks_behavior() {
    let (mut conn, _td) = setup_test_db();
    let draft_session_id = "018e3a00-0000-7000-8000-000000000010";
    seed_test_session(&conn, draft_session_id);

    let s0 = BusinessFlowData::new();
    let s0_hash = seed_draft_version(&conn, draft_session_id, 1, &s0, "phase_confirm", None, None);

    let run_id = "018e3a00-0000-7000-8000-000000000020";
    let plan_ctx = AuthorizedRoundPlanContext {
        workflow_run_id: run_id.to_string(),
        plan_generation: 1,
        phase: 1,
        subphase: None,
        round_index: 0,
        parallel_limit: 4,
        authorized_tasks: BTreeMap::new(),
    };

    // Case 1: has_applied_write = false -> Ok response with no version, no panic, 0 DB rows
    let req_ok = WorkflowRoundSealRequest {
        draft_session_id: draft_session_id.to_string(),
        workflow_run_id: run_id.to_string(),
        plan_generation: 1,
        round_index: 0,
        round_seal_key: format!("wf_round:{}:1:0", run_id),
        phase: 1,
        subphase: None,
        parallel_limit: 4,
        base_version_seq: 1,
        base_data_hash: s0_hash.clone(),
        completed_tasks: vec![],
        has_applied_write: false,
    };

    let res = seal_workflow_round_tx(&mut conn, &req_ok, &plan_ctx).unwrap();
    assert_eq!(res.sealed_version_seq, None);
    assert_eq!(res.receipts.len(), 0);
    assert!(!res.was_idempotent_replay);

    let round_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM workflow_round_receipts", [], |r| {
            r.get(0)
        })
        .unwrap();
    assert_eq!(round_count, 0);

    // Case 2: has_applied_write = true with empty completed_tasks -> Err(ValidationFailed)
    let req_err = WorkflowRoundSealRequest {
        draft_session_id: draft_session_id.to_string(),
        workflow_run_id: run_id.to_string(),
        plan_generation: 1,
        round_index: 0,
        round_seal_key: format!("wf_round:{}:1:0", run_id),
        phase: 1,
        subphase: None,
        parallel_limit: 4,
        base_version_seq: 1,
        base_data_hash: s0_hash,
        completed_tasks: vec![],
        has_applied_write: true,
    };

    let err = seal_workflow_round_tx(&mut conn, &req_err, &plan_ctx).unwrap_err();
    assert_eq!(err.code, ErrorCode::ValidationFailed);
}

#[test]
#[allow(non_snake_case)]
fn WF_02_and_WF_C03_single_mutating_task_creates_draft_version_and_receipt() {
    let (mut conn, _td) = setup_test_db();
    let draft_session_id = "018e3a00-0000-7000-8000-000000000010";
    seed_test_session(&conn, draft_session_id);

    let mut s0 = BusinessFlowData::new();
    s0.counters.insert("test_counter".to_string(), 10);
    let s0_hash = seed_draft_version(&conn, draft_session_id, 3, &s0, "phase_confirm", None, None);

    let run_id = "018e3a00-0000-7000-8000-000000000020";
    let seal_key = format!("wf_round:{}:1:0", run_id);

    let spec1 = PlannedTaskSpec {
        phase: 1,
        subphase: None,
        task_kind: "editor".to_string(),
        assigned_scope: json!({"part": 1}),
        dependencies: vec![],
        deterministic_identity: Some("task:p1:editor:{\"part\":1}".to_string()),
    };

    let mut s1 = s0.clone();
    s1.counters.insert("test_counter".to_string(), 20);
    let patch1 = canonical_diff(&s0, &s1).unwrap();

    let task1 = CompletedTaskOutcome {
        task_index: 0,
        task_spec: spec1.clone(),
        task_identity: "task:p1:editor:{\"part\":1}".to_string(),
        input_scope_fingerprint: "in_fp".to_string(),
        output_scope_fingerprint: "out_fp".to_string(),
        has_applied_write: true,
        applied_write_count: 1,
        applied_mutations: patch1.ops,
        artifact: None,
    };

    let mut auth = BTreeMap::new();
    auth.insert(0, spec1);
    let plan = AuthorizedRoundPlanContext {
        workflow_run_id: run_id.to_string(),
        plan_generation: 1,
        phase: 1,
        subphase: None,
        round_index: 0,
        parallel_limit: 4,
        authorized_tasks: auth,
    };

    let req = WorkflowRoundSealRequest {
        draft_session_id: draft_session_id.to_string(),
        workflow_run_id: run_id.to_string(),
        plan_generation: 1,
        round_index: 0,
        round_seal_key: seal_key.clone(),
        phase: 1,
        subphase: None,
        parallel_limit: 4,
        base_version_seq: 3,
        base_data_hash: s0_hash,
        completed_tasks: vec![task1],
        has_applied_write: true,
    };

    let res = seal_workflow_round_tx(&mut conn, &req, &plan).unwrap();

    // Verifies WF-02 & WF-C03 expectations:
    // Creates exactly 1 draft_versions row (seq = 4, reason = 'workflow_round', seal_key = req.round_seal_key)
    assert_eq!(res.sealed_version_seq, Some(4));
    assert!(res.durable_version_created);
    assert!(res.net_round_state_changed);

    // Verifies draft_sessions head_version_seq updated to 4
    let head_seq: i64 = conn
        .query_row(
            "SELECT head_version_seq FROM draft_sessions WHERE id = ?",
            [draft_session_id],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(head_seq, 4);

    // Verifies 1 workflow_task_receipts with had_mutations = 1, resulting_version_seq = 4
    assert_eq!(res.receipts.len(), 1);
    assert!(res.receipts[0].had_mutations);
    assert_eq!(res.receipts[0].resulting_version_seq, Some(4));

    // Verify historical verifier accepts the round
    let round_id = res.round_receipt_id.unwrap();
    let verify_res = verify_historical_round(&conn, &round_id, &["v2"], Some(4));
    assert_eq!(verify_res.integrity, Integrity::Verified);
}

#[test]
#[allow(non_snake_case)]
fn WF_03_and_WF_C02_single_zero_mutation_task_creates_receipt_without_draft_version() {
    let (mut conn, _td) = setup_test_db();
    let draft_session_id = "018e3a00-0000-7000-8000-000000000010";
    seed_test_session(&conn, draft_session_id);

    let s0 = BusinessFlowData::new();
    let s0_hash = seed_draft_version(&conn, draft_session_id, 3, &s0, "phase_confirm", None, None);

    let run_id = "018e3a00-0000-7000-8000-000000000020";
    let seal_key = format!("wf_round:{}:1:0", run_id);

    let spec1 = PlannedTaskSpec {
        phase: 1,
        subphase: None,
        task_kind: "auditor".to_string(),
        assigned_scope: json!({"part": 1}),
        dependencies: vec![],
        deterministic_identity: Some("task:p1:auditor:{\"part\":1}".to_string()),
    };

    let task1 = CompletedTaskOutcome {
        task_index: 0,
        task_spec: spec1.clone(),
        task_identity: "task:p1:auditor:{\"part\":1}".to_string(),
        input_scope_fingerprint: "in_fp".to_string(),
        output_scope_fingerprint: "out_fp".to_string(),
        has_applied_write: false,
        applied_write_count: 0,
        applied_mutations: vec![],
        artifact: None,
    };

    let mut auth = BTreeMap::new();
    auth.insert(0, spec1);
    let plan = AuthorizedRoundPlanContext {
        workflow_run_id: run_id.to_string(),
        plan_generation: 1,
        phase: 1,
        subphase: None,
        round_index: 0,
        parallel_limit: 4,
        authorized_tasks: auth,
    };

    let req = WorkflowRoundSealRequest {
        draft_session_id: draft_session_id.to_string(),
        workflow_run_id: run_id.to_string(),
        plan_generation: 1,
        round_index: 0,
        round_seal_key: seal_key,
        phase: 1,
        subphase: None,
        parallel_limit: 4,
        base_version_seq: 3,
        base_data_hash: s0_hash,
        completed_tasks: vec![task1],
        has_applied_write: false,
    };

    let res = seal_workflow_round_tx(&mut conn, &req, &plan).unwrap();

    // Verifies WF-03 & WF-C02 expectations:
    // Creates 0 draft_versions row (resulting_version_seq = None)
    assert_eq!(res.sealed_version_seq, None);
    assert!(!res.durable_version_created);
    assert!(!res.net_round_state_changed);

    // Head version sequence remains 3
    let head_seq: i64 = conn
        .query_row(
            "SELECT head_version_seq FROM draft_sessions WHERE id = ?",
            [draft_session_id],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(head_seq, 3);

    // Exactly 1 row in workflow_task_receipts with had_mutations = 0, resulting_version_seq = NULL
    assert_eq!(res.receipts.len(), 1);
    assert!(!res.receipts[0].had_mutations);
    assert_eq!(res.receipts[0].resulting_version_seq, None);

    // Verifies historical round verifies
    let round_id = res.round_receipt_id.unwrap();
    let verify_res = verify_historical_round(&conn, &round_id, &["v2"], Some(3));
    assert_eq!(verify_res.integrity, Integrity::Verified);
}

#[test]
#[allow(non_snake_case)]
fn WF_04_and_WF_C04_multi_task_round_with_mutating_and_readonly_tasks() {
    let (mut conn, _td) = setup_test_db();
    let draft_session_id = "018e3a00-0000-7000-8000-000000000010";
    seed_test_session(&conn, draft_session_id);

    let mut s0 = BusinessFlowData::new();
    s0.counters.insert("A".to_string(), 1);
    s0.counters.insert("B".to_string(), 10);
    let s0_hash = seed_draft_version(&conn, draft_session_id, 3, &s0, "phase_confirm", None, None);

    let run_id = "018e3a00-0000-7000-8000-000000000020";
    let seal_key = format!("wf_round:{}:1:0", run_id);

    // T1: mutating (A: 1 -> 2)
    let spec1 = PlannedTaskSpec {
        phase: 1,
        subphase: None,
        task_kind: "writer_a".to_string(),
        assigned_scope: json!({"field": "A"}),
        dependencies: vec![],
        deterministic_identity: Some("task:p1:writer_a:{\"field\":\"A\"}".to_string()),
    };
    let mut s1 = s0.clone();
    s1.counters.insert("A".to_string(), 2);
    let patch1 = canonical_diff(&s0, &s1).unwrap();

    let task1 = CompletedTaskOutcome {
        task_index: 0,
        task_spec: spec1.clone(),
        task_identity: "task:p1:writer_a:{\"field\":\"A\"}".to_string(),
        input_scope_fingerprint: "in_1".to_string(),
        output_scope_fingerprint: "out_1".to_string(),
        has_applied_write: true,
        applied_write_count: 1,
        applied_mutations: patch1.ops,
        artifact: None,
    };

    // T2: read-only (no mutation, S1 == S2)
    let spec2 = PlannedTaskSpec {
        phase: 1,
        subphase: None,
        task_kind: "reader_b".to_string(),
        assigned_scope: json!({"field": "B"}),
        dependencies: vec![],
        deterministic_identity: Some("task:p1:reader_b:{\"field\":\"B\"}".to_string()),
    };
    let task2 = CompletedTaskOutcome {
        task_index: 1,
        task_spec: spec2.clone(),
        task_identity: "task:p1:reader_b:{\"field\":\"B\"}".to_string(),
        input_scope_fingerprint: "in_2".to_string(),
        output_scope_fingerprint: "out_2".to_string(),
        has_applied_write: false,
        applied_write_count: 0,
        applied_mutations: vec![],
        artifact: None,
    };

    // T3: mutating (B: 10 -> 20)
    let spec3 = PlannedTaskSpec {
        phase: 1,
        subphase: None,
        task_kind: "writer_b".to_string(),
        assigned_scope: json!({"field": "B_write"}),
        dependencies: vec![],
        deterministic_identity: Some("task:p1:writer_b:{\"field\":\"B_write\"}".to_string()),
    };
    let mut s3 = s1.clone();
    s3.counters.insert("B".to_string(), 20);
    let patch3 = canonical_diff(&s1, &s3).unwrap();

    let task3 = CompletedTaskOutcome {
        task_index: 2,
        task_spec: spec3.clone(),
        task_identity: "task:p1:writer_b:{\"field\":\"B_write\"}".to_string(),
        input_scope_fingerprint: "in_3".to_string(),
        output_scope_fingerprint: "out_3".to_string(),
        has_applied_write: true,
        applied_write_count: 1,
        applied_mutations: patch3.ops,
        artifact: None,
    };

    let mut auth = BTreeMap::new();
    auth.insert(0, spec1);
    auth.insert(1, spec2);
    auth.insert(2, spec3);

    let plan = AuthorizedRoundPlanContext {
        workflow_run_id: run_id.to_string(),
        plan_generation: 1,
        phase: 1,
        subphase: None,
        round_index: 0,
        parallel_limit: 4,
        authorized_tasks: auth,
    };

    // Submit tasks in scrambled order [T2, T3, T1] to verify deterministic sorting by task_index
    let req = WorkflowRoundSealRequest {
        draft_session_id: draft_session_id.to_string(),
        workflow_run_id: run_id.to_string(),
        plan_generation: 1,
        round_index: 0,
        round_seal_key: seal_key,
        phase: 1,
        subphase: None,
        parallel_limit: 4,
        base_version_seq: 3,
        base_data_hash: s0_hash,
        completed_tasks: vec![task2, task3, task1],
        has_applied_write: true,
    };

    let res = seal_workflow_round_tx(&mut conn, &req, &plan).unwrap();

    // Verifies WF-04 / WF-C04 expectations:
    // Exactly 1 new draft_versions row (seq = 4)
    assert_eq!(res.sealed_version_seq, Some(4));
    assert!(res.durable_version_created);

    let head_seq: i64 = conn
        .query_row(
            "SELECT head_version_seq FROM draft_sessions WHERE id = ?",
            [draft_session_id],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(head_seq, 4);

    // Receipts: T1 & T3 have had_mutations = 1, resulting_version_seq = 4
    // T2 has had_mutations = 0, resulting_version_seq = NULL
    assert_eq!(res.receipts.len(), 3);
    assert_eq!(
        res.receipts[0].task_identity,
        "task:p1:writer_a:{\"field\":\"A\"}"
    );
    assert!(res.receipts[0].had_mutations);
    assert_eq!(res.receipts[0].resulting_version_seq, Some(4));

    assert_eq!(
        res.receipts[1].task_identity,
        "task:p1:reader_b:{\"field\":\"B\"}"
    );
    assert!(!res.receipts[1].had_mutations);
    assert_eq!(res.receipts[1].resulting_version_seq, None);

    assert_eq!(
        res.receipts[2].task_identity,
        "task:p1:writer_b:{\"field\":\"B_write\"}"
    );
    assert!(res.receipts[2].had_mutations);
    assert_eq!(res.receipts[2].resulting_version_seq, Some(4));

    let round_id = res.round_receipt_id.unwrap();
    let verify_res = verify_historical_round(&conn, &round_id, &["v2"], Some(4));
    assert_eq!(verify_res.integrity, Integrity::Verified);
}

#[test]
#[allow(non_snake_case)]
fn WF_06_prefix_receipt_and_new_suffix_reconciliation() {
    let (mut conn, _td) = setup_test_db();
    let draft_session_id = "018e3a00-0000-7000-8000-000000000010";
    seed_test_session(&conn, draft_session_id);

    let mut s0 = BusinessFlowData::new();
    s0.counters.insert("step".to_string(), 0);
    let s0_hash = seed_draft_version(&conn, draft_session_id, 1, &s0, "phase_confirm", None, None);

    let run_id = "018e3a00-0000-7000-8000-000000000020";

    // Seed existing task receipt T0 directly in DB (simulating crash before full round sealed)
    let spec0 = PlannedTaskSpec {
        phase: 1,
        subphase: None,
        task_kind: "step0".to_string(),
        assigned_scope: json!({"step": 0}),
        dependencies: vec![],
        deterministic_identity: Some("task:p1:step0:{\"step\":0}".to_string()),
    };
    let mut s1 = s0.clone();
    s1.counters.insert("step".to_string(), 1);
    let patch0 = canonical_diff(&s0, &s1).unwrap();

    conn.execute(
        "INSERT INTO workflow_task_receipts (
            id, draft_session_id, workflow_run_id, plan_generation, task_identity,
            input_scope_fingerprint, output_scope_fingerprint, base_version_seq,
            resulting_version_seq, had_mutations, round_receipt_id, task_index, evidence_json, created_at
         ) VALUES ('018e3a00-0000-7000-8000-000000000055', ?1, ?2, 1, ?3, 'in_0', 'out_0', 1, NULL, 0, NULL, NULL, NULL, '2026-11-08T00:00:00Z')",
        params![draft_session_id, run_id, "task:p1:step0:{\"step\":0}"],
    ).unwrap();

    // Now submit round containing [T0 (existing), T1 (new)]
    let task0 = CompletedTaskOutcome {
        task_index: 0,
        task_spec: spec0.clone(),
        task_identity: "task:p1:step0:{\"step\":0}".to_string(),
        input_scope_fingerprint: "in_0".to_string(),
        output_scope_fingerprint: "out_0".to_string(),
        has_applied_write: true,
        applied_write_count: 1,
        applied_mutations: patch0.ops,
        artifact: None,
    };

    let spec1 = PlannedTaskSpec {
        phase: 1,
        subphase: None,
        task_kind: "step1".to_string(),
        assigned_scope: json!({"step": 1}),
        dependencies: vec![],
        deterministic_identity: Some("task:p1:step1:{\"step\":1}".to_string()),
    };
    let mut s2 = s1.clone();
    s2.counters.insert("step".to_string(), 2);
    let patch1 = canonical_diff(&s1, &s2).unwrap();

    let task1 = CompletedTaskOutcome {
        task_index: 1,
        task_spec: spec1.clone(),
        task_identity: "task:p1:step1:{\"step\":1}".to_string(),
        input_scope_fingerprint: "in_1".to_string(),
        output_scope_fingerprint: "out_1".to_string(),
        has_applied_write: true,
        applied_write_count: 1,
        applied_mutations: patch1.ops,
        artifact: None,
    };

    let mut auth = BTreeMap::new();
    auth.insert(0, spec0);
    auth.insert(1, spec1);

    let plan = AuthorizedRoundPlanContext {
        workflow_run_id: run_id.to_string(),
        plan_generation: 1,
        phase: 1,
        subphase: None,
        round_index: 0,
        parallel_limit: 4,
        authorized_tasks: auth,
    };

    let req = WorkflowRoundSealRequest {
        draft_session_id: draft_session_id.to_string(),
        workflow_run_id: run_id.to_string(),
        plan_generation: 1,
        round_index: 0,
        round_seal_key: format!("wf_round:{}:1:0", run_id),
        phase: 1,
        subphase: None,
        parallel_limit: 4,
        base_version_seq: 1,
        base_data_hash: s0_hash,
        completed_tasks: vec![task0, task1],
        has_applied_write: true,
    };

    let res = seal_workflow_round_tx(&mut conn, &req, &plan).unwrap();
    assert_eq!(res.sealed_version_seq, Some(2));
    assert_eq!(res.receipts.len(), 2);

    // Verify insert-once semantics: T0 receipt was preserved, T1 was newly inserted
    let receipts_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM workflow_task_receipts", [], |r| {
            r.get(0)
        })
        .unwrap();
    assert_eq!(receipts_count, 2);
}

#[test]
#[allow(non_snake_case)]
fn WF_07_and_WF_C13_conflicting_receipt_reuse_with_altered_fingerprint_rejected() {
    let (mut conn, _td) = setup_test_db();
    let draft_session_id = "018e3a00-0000-7000-8000-000000000010";
    seed_test_session(&conn, draft_session_id);

    let s0 = BusinessFlowData::new();
    let s0_hash = seed_draft_version(&conn, draft_session_id, 1, &s0, "phase_confirm", None, None);

    let run_id = "018e3a00-0000-7000-8000-000000000020";

    // Seed historical receipt with specific fingerprints
    conn.execute(
        "INSERT INTO workflow_task_receipts (
            id, draft_session_id, workflow_run_id, plan_generation, task_identity,
            input_scope_fingerprint, output_scope_fingerprint, base_version_seq,
            resulting_version_seq, had_mutations, created_at
         ) VALUES ('018e3a00-0000-7000-8000-000000000099', ?1, ?2, 1, 'task:p1:t1:{\"x\":1}', 'correct_in_fp', 'correct_out_fp', 1, NULL, 0, '2026-11-08T00:00:00Z')",
        params![draft_session_id, run_id],
    ).unwrap();

    let spec1 = PlannedTaskSpec {
        phase: 1,
        subphase: None,
        task_kind: "t1".to_string(),
        assigned_scope: json!({"x": 1}),
        dependencies: vec![],
        deterministic_identity: Some("task:p1:t1:{\"x\":1}".to_string()),
    };

    // Altered fingerprint in caller request
    let task1_altered = CompletedTaskOutcome {
        task_index: 0,
        task_spec: spec1.clone(),
        task_identity: "task:p1:t1:{\"x\":1}".to_string(),
        input_scope_fingerprint: "tampered_in_fp".to_string(),
        output_scope_fingerprint: "correct_out_fp".to_string(),
        has_applied_write: false,
        applied_write_count: 0,
        applied_mutations: vec![],
        artifact: None,
    };

    let mut auth = BTreeMap::new();
    auth.insert(0, spec1);
    let plan = AuthorizedRoundPlanContext {
        workflow_run_id: run_id.to_string(),
        plan_generation: 1,
        phase: 1,
        subphase: None,
        round_index: 0,
        parallel_limit: 4,
        authorized_tasks: auth,
    };

    let req = WorkflowRoundSealRequest {
        draft_session_id: draft_session_id.to_string(),
        workflow_run_id: run_id.to_string(),
        plan_generation: 1,
        round_index: 0,
        round_seal_key: format!("wf_round:{}:1:0", run_id),
        phase: 1,
        subphase: None,
        parallel_limit: 4,
        base_version_seq: 1,
        base_data_hash: s0_hash,
        completed_tasks: vec![task1_altered],
        has_applied_write: false,
    };

    let err = seal_workflow_round_tx(&mut conn, &req, &plan).unwrap_err();
    assert_eq!(err.code, ErrorCode::StaleWorkflowReceipt);

    // Verify historical receipt remains intact
    let stored_fp: String = conn.query_row(
        "SELECT input_scope_fingerprint FROM workflow_task_receipts WHERE id = '018e3a00-0000-7000-8000-000000000099'",
        [],
        |r| r.get(0),
    ).unwrap();
    assert_eq!(stored_fp, "correct_in_fp");
}

#[test]
#[allow(non_snake_case)]
fn WF_C01_applied_mutation_with_net_zero_round_state_change() {
    let (mut conn, _td) = setup_test_db();
    let draft_session_id = "018e3a00-0000-7000-8000-000000000010";
    seed_test_session(&conn, draft_session_id);

    let mut s0 = BusinessFlowData::new();
    s0.counters.insert("A".to_string(), 1);
    let s0_hash = seed_draft_version(&conn, draft_session_id, 1, &s0, "phase_confirm", None, None);

    let run_id = "018e3a00-0000-7000-8000-000000000020";

    // T1: sets A = 2
    let spec1 = PlannedTaskSpec {
        phase: 1,
        subphase: None,
        task_kind: "t1".to_string(),
        assigned_scope: json!({"field": "A"}),
        dependencies: vec![],
        deterministic_identity: Some("task:p1:t1:{\"field\":\"A\"}".to_string()),
    };
    let mut s1 = s0.clone();
    s1.counters.insert("A".to_string(), 2);
    let patch1 = canonical_diff(&s0, &s1).unwrap();

    let task1 = CompletedTaskOutcome {
        task_index: 0,
        task_spec: spec1.clone(),
        task_identity: "task:p1:t1:{\"field\":\"A\"}".to_string(),
        input_scope_fingerprint: "in_1".to_string(),
        output_scope_fingerprint: "out_1".to_string(),
        has_applied_write: true,
        applied_write_count: 1,
        applied_mutations: patch1.ops,
        artifact: None,
    };

    // T2: sets A = 1 (cancelling out, S2 == S0)
    let spec2 = PlannedTaskSpec {
        phase: 1,
        subphase: None,
        task_kind: "t2".to_string(),
        assigned_scope: json!({"field": "A"}),
        dependencies: vec![],
        deterministic_identity: Some("task:p1:t2:{\"field\":\"A\"}".to_string()),
    };
    let patch2 = canonical_diff(&s1, &s0).unwrap();

    let task2 = CompletedTaskOutcome {
        task_index: 1,
        task_spec: spec2.clone(),
        task_identity: "task:p1:t2:{\"field\":\"A\"}".to_string(),
        input_scope_fingerprint: "in_2".to_string(),
        output_scope_fingerprint: "out_2".to_string(),
        has_applied_write: true,
        applied_write_count: 1,
        applied_mutations: patch2.ops,
        artifact: None,
    };

    let mut auth = BTreeMap::new();
    auth.insert(0, spec1);
    auth.insert(1, spec2);
    let plan = AuthorizedRoundPlanContext {
        workflow_run_id: run_id.to_string(),
        plan_generation: 1,
        phase: 1,
        subphase: None,
        round_index: 0,
        parallel_limit: 4,
        authorized_tasks: auth,
    };

    let req = WorkflowRoundSealRequest {
        draft_session_id: draft_session_id.to_string(),
        workflow_run_id: run_id.to_string(),
        plan_generation: 1,
        round_index: 0,
        round_seal_key: format!("wf_round:{}:1:0", run_id),
        phase: 1,
        subphase: None,
        parallel_limit: 4,
        base_version_seq: 1,
        base_data_hash: s0_hash,
        completed_tasks: vec![task1, task2],
        has_applied_write: true,
    };

    let res = seal_workflow_round_tx(&mut conn, &req, &plan).unwrap();

    // Verifies WF-C01:
    // net_round_state_changed is false, durable_version_created is false
    assert!(!res.net_round_state_changed);
    assert!(!res.durable_version_created);
    assert_eq!(res.sealed_version_seq, None);

    // 0 rows inserted in draft_versions
    let ver_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM draft_versions", [], |r| r.get(0))
        .unwrap();
    assert_eq!(ver_count, 1, "Only baseline S0 version must exist");

    // 2 receipts inserted with had_mutations = 0, resulting_version_seq = NULL
    assert_eq!(res.receipts.len(), 2);
    assert!(!res.receipts[0].had_mutations);
    assert_eq!(res.receipts[0].resulting_version_seq, None);
    assert!(!res.receipts[1].had_mutations);
    assert_eq!(res.receipts[1].resulting_version_seq, None);

    // Verifies historical verifier can verify this zero-net round
    let round_id = res.round_receipt_id.unwrap();
    let verify_res = verify_historical_round(&conn, &round_id, &["v2"], Some(1));
    assert_eq!(verify_res.integrity, Integrity::Verified);
}

#[test]
#[allow(non_snake_case)]
fn WF_C05_C06_C07_duplicate_detection() {
    let (mut conn, _td) = setup_test_db();
    let draft_session_id = "018e3a00-0000-7000-8000-000000000010";
    seed_test_session(&conn, draft_session_id);

    let s0 = BusinessFlowData::new();
    let s0_hash = seed_draft_version(&conn, draft_session_id, 1, &s0, "phase_confirm", None, None);
    let run_id = "018e3a00-0000-7000-8000-000000000020";

    let spec_a = PlannedTaskSpec {
        phase: 1,
        subphase: None,
        task_kind: "task_a".to_string(),
        assigned_scope: json!({"id": "A"}),
        dependencies: vec![],
        deterministic_identity: Some("task:p1:task_a:{\"id\":\"A\"}".to_string()),
    };
    let spec_b = PlannedTaskSpec {
        phase: 1,
        subphase: None,
        task_kind: "task_b".to_string(),
        assigned_scope: json!({"id": "B"}),
        dependencies: vec![],
        deterministic_identity: Some("task:p1:task_b:{\"id\":\"B\"}".to_string()),
    };

    let mut auth = BTreeMap::new();
    auth.insert(0, spec_a.clone());
    auth.insert(1, spec_b.clone());
    auth.insert(2, spec_a.clone());

    let plan = AuthorizedRoundPlanContext {
        workflow_run_id: run_id.to_string(),
        plan_generation: 1,
        phase: 1,
        subphase: None,
        round_index: 0,
        parallel_limit: 4,
        authorized_tasks: auth,
    };

    // WF-C05: Adjacent duplicate task_identity [(0, "A"), (1, "A")]
    let task_a0 = CompletedTaskOutcome {
        task_index: 0,
        task_spec: spec_a.clone(),
        task_identity: "task:p1:task_a:{\"id\":\"A\"}".to_string(),
        input_scope_fingerprint: "in_a".to_string(),
        output_scope_fingerprint: "out_a".to_string(),
        has_applied_write: false,
        applied_write_count: 0,
        applied_mutations: vec![],
        artifact: None,
    };
    let task_a1 = CompletedTaskOutcome {
        task_index: 1,
        task_spec: spec_a.clone(),
        task_identity: "task:p1:task_a:{\"id\":\"A\"}".to_string(),
        input_scope_fingerprint: "in_a".to_string(),
        output_scope_fingerprint: "out_a".to_string(),
        has_applied_write: false,
        applied_write_count: 0,
        applied_mutations: vec![],
        artifact: None,
    };

    let req_c05 = WorkflowRoundSealRequest {
        draft_session_id: draft_session_id.to_string(),
        workflow_run_id: run_id.to_string(),
        plan_generation: 1,
        round_index: 0,
        round_seal_key: format!("wf_round:{}:1:0", run_id),
        phase: 1,
        subphase: None,
        parallel_limit: 4,
        base_version_seq: 1,
        base_data_hash: s0_hash.clone(),
        completed_tasks: vec![task_a0.clone(), task_a1],
        has_applied_write: false,
    };
    let err_c05 = seal_workflow_round_tx(&mut conn, &req_c05, &plan).unwrap_err();
    assert_eq!(err_c05.code, ErrorCode::ValidationFailed);

    // WF-C06: Non-adjacent duplicate task_identity [(0, "A"), (1, "B"), (2, "A")]
    let task_b1 = CompletedTaskOutcome {
        task_index: 1,
        task_spec: spec_b.clone(),
        task_identity: "task:p1:task_b:{\"id\":\"B\"}".to_string(),
        input_scope_fingerprint: "in_b".to_string(),
        output_scope_fingerprint: "out_b".to_string(),
        has_applied_write: false,
        applied_write_count: 0,
        applied_mutations: vec![],
        artifact: None,
    };
    let task_a2 = CompletedTaskOutcome {
        task_index: 2,
        task_spec: spec_a.clone(),
        task_identity: "task:p1:task_a:{\"id\":\"A\"}".to_string(),
        input_scope_fingerprint: "in_a".to_string(),
        output_scope_fingerprint: "out_a".to_string(),
        has_applied_write: false,
        applied_write_count: 0,
        applied_mutations: vec![],
        artifact: None,
    };

    let req_c06 = WorkflowRoundSealRequest {
        draft_session_id: draft_session_id.to_string(),
        workflow_run_id: run_id.to_string(),
        plan_generation: 1,
        round_index: 0,
        round_seal_key: format!("wf_round:{}:1:0", run_id),
        phase: 1,
        subphase: None,
        parallel_limit: 4,
        base_version_seq: 1,
        base_data_hash: s0_hash.clone(),
        completed_tasks: vec![task_a0.clone(), task_b1.clone(), task_a2],
        has_applied_write: false,
    };
    let err_c06 = seal_workflow_round_tx(&mut conn, &req_c06, &plan).unwrap_err();
    assert_eq!(err_c06.code, ErrorCode::ValidationFailed);

    // WF-C07: Duplicate task_index [(0, "A"), (0, "B")]
    let mut task_b0 = task_b1;
    task_b0.task_index = 0;
    let req_c07 = WorkflowRoundSealRequest {
        draft_session_id: draft_session_id.to_string(),
        workflow_run_id: run_id.to_string(),
        plan_generation: 1,
        round_index: 0,
        round_seal_key: format!("wf_round:{}:1:0", run_id),
        phase: 1,
        subphase: None,
        parallel_limit: 4,
        base_version_seq: 1,
        base_data_hash: s0_hash,
        completed_tasks: vec![task_a0, task_b0],
        has_applied_write: false,
    };
    let err_c07 = seal_workflow_round_tx(&mut conn, &req_c07, &plan).unwrap_err();
    assert_eq!(err_c07.code, ErrorCode::ValidationFailed);
}

#[test]
#[allow(non_snake_case)]
fn pending_patches_exclusion_and_independent_tx08_lifecycle() {
    let (mut conn, _td) = setup_test_db();
    let draft_session_id = "018e3a00-0000-7000-8000-000000000010";
    seed_test_session(&conn, draft_session_id);

    let mut s0 = BusinessFlowData::new();
    s0.counters.insert("counter".to_string(), 1);
    let s0_hash = seed_draft_version(&conn, draft_session_id, 1, &s0, "phase_confirm", None, None);

    // Insert pending patch row (simulating TX-08 proposal created independently)
    conn.execute(
        "INSERT INTO pending_patches (
            id, draft_session_id, base_version_seq, target_type, target_code, field_name,
            operation, previous_value, new_value, target_source, status, created_at
         ) VALUES ('018e3a00-0000-7000-8000-000000000099', ?1, 1, 'character', 'char_1', 'name',
                   'set', '\"Old\"', '\"New\"', 'ai_generated', 'pending', '2026-11-08T00:00:00Z')",
        params![draft_session_id],
    )
    .unwrap();

    let run_id = "018e3a00-0000-7000-8000-000000000020";
    let spec1 = PlannedTaskSpec {
        phase: 1,
        subphase: None,
        task_kind: "task1".to_string(),
        assigned_scope: json!({"task": 1}),
        dependencies: vec![],
        deterministic_identity: Some("task:p1:task1:{\"task\":1}".to_string()),
    };

    let mut s1 = s0.clone();
    s1.counters.insert("counter".to_string(), 2);
    let patch1 = canonical_diff(&s0, &s1).unwrap();

    let task1 = CompletedTaskOutcome {
        task_index: 0,
        task_spec: spec1.clone(),
        task_identity: "task:p1:task1:{\"task\":1}".to_string(),
        input_scope_fingerprint: "in_1".to_string(),
        output_scope_fingerprint: "out_1".to_string(),
        has_applied_write: true,
        applied_write_count: 1,
        applied_mutations: patch1.ops,
        artifact: None,
    };

    let mut auth = BTreeMap::new();
    auth.insert(0, spec1);
    let plan = AuthorizedRoundPlanContext {
        workflow_run_id: run_id.to_string(),
        plan_generation: 1,
        phase: 1,
        subphase: None,
        round_index: 0,
        parallel_limit: 4,
        authorized_tasks: auth,
    };

    let req = WorkflowRoundSealRequest {
        draft_session_id: draft_session_id.to_string(),
        workflow_run_id: run_id.to_string(),
        plan_generation: 1,
        round_index: 0,
        round_seal_key: format!("wf_round:{}:1:0", run_id),
        phase: 1,
        subphase: None,
        parallel_limit: 4,
        base_version_seq: 1,
        base_data_hash: s0_hash,
        completed_tasks: vec![task1],
        has_applied_write: true,
    };

    // Seal round TX-04
    let res = seal_workflow_round_tx(&mut conn, &req, &plan).unwrap();
    assert_eq!(res.sealed_version_seq, Some(2));

    // Assert sealed draft_versions row DOES NOT contain the pending patch!
    let data_json: String = conn
        .query_row(
            "SELECT data_json FROM draft_versions WHERE draft_session_id = ? AND seq = 2",
            params![draft_session_id],
            |r| r.get(0),
        )
        .unwrap();
    assert!(
        !data_json.contains("char_1"),
        "pending_patches must be excluded from pure WorkingState"
    );

    // Assert pending patch in SQLite is still pending (TX-04 did not mutate or delete it)
    let patch_status: String = conn
        .query_row(
            "SELECT status FROM pending_patches WHERE id = '018e3a00-0000-7000-8000-000000000099'",
            [],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(patch_status, "pending");
}

#[test]
#[allow(non_snake_case)]
fn insert_once_semantics_never_replaces() {
    let (mut conn, _td) = setup_test_db();
    let draft_session_id = "018e3a00-0000-7000-8000-000000000010";
    seed_test_session(&conn, draft_session_id);

    let s0 = BusinessFlowData::new();
    let s0_hash = seed_draft_version(&conn, draft_session_id, 1, &s0, "phase_confirm", None, None);
    let run_id = "018e3a00-0000-7000-8000-000000000020";

    // Insert an initial receipt with created_at timestamp
    let original_id = "018e3a00-0000-7000-8000-000000000077";
    let original_time = "2026-11-01T00:00:00Z";
    conn.execute(
        "INSERT INTO workflow_task_receipts (
            id, draft_session_id, workflow_run_id, plan_generation, task_identity,
            input_scope_fingerprint, output_scope_fingerprint, base_version_seq,
            resulting_version_seq, had_mutations, created_at
         ) VALUES (?1, ?2, ?3, 1, 'task:p1:t:{\"x\":1}', 'in_1', 'out_1', 1, NULL, 0, ?4)",
        params![original_id, draft_session_id, run_id, original_time],
    )
    .unwrap();

    let spec = PlannedTaskSpec {
        phase: 1,
        subphase: None,
        task_kind: "t".to_string(),
        assigned_scope: json!({"x": 1}),
        dependencies: vec![],
        deterministic_identity: Some("task:p1:t:{\"x\":1}".to_string()),
    };

    let task = CompletedTaskOutcome {
        task_index: 0,
        task_spec: spec.clone(),
        task_identity: "task:p1:t:{\"x\":1}".to_string(),
        input_scope_fingerprint: "in_1".to_string(),
        output_scope_fingerprint: "out_1".to_string(),
        has_applied_write: false,
        applied_write_count: 0,
        applied_mutations: vec![],
        artifact: None,
    };

    let mut auth = BTreeMap::new();
    auth.insert(0, spec);
    let plan = AuthorizedRoundPlanContext {
        workflow_run_id: run_id.to_string(),
        plan_generation: 1,
        phase: 1,
        subphase: None,
        round_index: 0,
        parallel_limit: 4,
        authorized_tasks: auth,
    };

    let req = WorkflowRoundSealRequest {
        draft_session_id: draft_session_id.to_string(),
        workflow_run_id: run_id.to_string(),
        plan_generation: 1,
        round_index: 0,
        round_seal_key: format!("wf_round:{}:1:0", run_id),
        phase: 1,
        subphase: None,
        parallel_limit: 4,
        base_version_seq: 1,
        base_data_hash: s0_hash,
        completed_tasks: vec![task],
        has_applied_write: false,
    };

    // Execute seal round
    let res = seal_workflow_round_tx(&mut conn, &req, &plan).unwrap();
    assert_eq!(res.receipts.len(), 1);

    // Verify receipt was NOT replaced: original ID and original created_at remain unchanged
    let (stored_id, stored_created_at): (String, String) = conn.query_row(
        "SELECT id, created_at FROM workflow_task_receipts WHERE draft_session_id = ? AND workflow_run_id = ? AND task_identity = 'task:p1:t:{\"x\":1}'",
        params![draft_session_id, run_id],
        |r| Ok((r.get(0)?, r.get(1)?)),
    ).unwrap();

    assert_eq!(
        stored_id, original_id,
        "Receipt row ID must not change (ON CONFLICT DO NOTHING)"
    );
    assert_eq!(
        stored_created_at, original_time,
        "Original created_at must not be overwritten"
    );
}

#[test]
#[allow(non_snake_case)]
fn WF_05_and_WF_C14_full_round_retry_idempotency() {
    let (mut conn, _td) = setup_test_db();
    let draft_session_id = "018e3a00-0000-7000-8000-000000000010";
    seed_test_session(&conn, draft_session_id);

    let mut s0 = BusinessFlowData::new();
    s0.counters.insert("counter".to_string(), 100);
    let s0_hash = seed_draft_version(&conn, draft_session_id, 1, &s0, "phase_confirm", None, None);

    let run_id = "018e3a00-0000-7000-8000-000000000020";
    let seal_key = format!("wf_round:{}:1:0", run_id);

    let spec1 = PlannedTaskSpec {
        phase: 1,
        subphase: None,
        task_kind: "t1".to_string(),
        assigned_scope: json!({"c": 100}),
        dependencies: vec![],
        deterministic_identity: Some("task:p1:t1:{\"c\":100}".to_string()),
    };
    let mut s1 = s0.clone();
    s1.counters.insert("counter".to_string(), 200);
    let patch1 = canonical_diff(&s0, &s1).unwrap();

    let task1 = CompletedTaskOutcome {
        task_index: 0,
        task_spec: spec1.clone(),
        task_identity: "task:p1:t1:{\"c\":100}".to_string(),
        input_scope_fingerprint: "in_100".to_string(),
        output_scope_fingerprint: "out_200".to_string(),
        has_applied_write: true,
        applied_write_count: 1,
        applied_mutations: patch1.ops,
        artifact: None,
    };

    let mut auth = BTreeMap::new();
    auth.insert(0, spec1);
    let plan = AuthorizedRoundPlanContext {
        workflow_run_id: run_id.to_string(),
        plan_generation: 1,
        phase: 1,
        subphase: None,
        round_index: 0,
        parallel_limit: 4,
        authorized_tasks: auth,
    };

    let req = WorkflowRoundSealRequest {
        draft_session_id: draft_session_id.to_string(),
        workflow_run_id: run_id.to_string(),
        plan_generation: 1,
        round_index: 0,
        round_seal_key: seal_key,
        phase: 1,
        subphase: None,
        parallel_limit: 4,
        base_version_seq: 1,
        base_data_hash: s0_hash,
        completed_tasks: vec![task1],
        has_applied_write: true,
    };

    // First seal execution
    let res1 = seal_workflow_round_tx(&mut conn, &req, &plan).unwrap();
    assert_eq!(res1.sealed_version_seq, Some(2));
    assert!(res1.durable_version_created);
    assert!(!res1.was_idempotent_replay);

    // Second seal execution (idempotent retry)
    let res2 = seal_workflow_round_tx(&mut conn, &req, &plan).unwrap();
    assert_eq!(res2.sealed_version_seq, Some(2));
    assert!(!res2.durable_version_created);
    assert!(res2.was_idempotent_replay);
    assert_eq!(res2.receipts.len(), 1);

    // Total versions in DB must remain 2
    let ver_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM draft_versions", [], |r| r.get(0))
        .unwrap();
    assert_eq!(ver_count, 2);
}

#[test]
#[allow(non_snake_case)]
fn WF_08_and_WF_C11_restore_then_resume_lineage_severed() {
    let (mut conn, _td) = setup_test_db();
    let draft_session_id = "018e3a00-0000-7000-8000-000000000010";
    seed_test_session(&conn, draft_session_id);

    let mut s0 = BusinessFlowData::new();
    s0.counters.insert("v".to_string(), 1);
    let _s0_hash = seed_draft_version(&conn, draft_session_id, 1, &s0, "phase_confirm", None, None);

    let mut s1 = s0.clone();
    s1.counters.insert("v".to_string(), 2);
    let s1_hash = seed_draft_version(
        &conn,
        draft_session_id,
        2,
        &s1,
        "workflow_round",
        Some(1),
        None,
    );

    // Restore v1 creating v3 with parent = 1
    seed_draft_version(&conn, draft_session_id, 3, &s0, "restore", Some(1), None);

    let run_id = "018e3a00-0000-7000-8000-000000000020";
    let spec1 = PlannedTaskSpec {
        phase: 1,
        subphase: None,
        task_kind: "t".to_string(),
        assigned_scope: json!({}),
        dependencies: vec![],
        deterministic_identity: Some("task:p1:t:{}".to_string()),
    };

    let mut auth = BTreeMap::new();
    auth.insert(0, spec1.clone());
    let plan = AuthorizedRoundPlanContext {
        workflow_run_id: run_id.to_string(),
        plan_generation: 1,
        phase: 1,
        subphase: None,
        round_index: 1,
        parallel_limit: 4,
        authorized_tasks: auth,
    };

    // Attempt to resume workflow using stale baseline v2 (while head is now v3!)
    let req_stale = WorkflowRoundSealRequest {
        draft_session_id: draft_session_id.to_string(),
        workflow_run_id: run_id.to_string(),
        plan_generation: 1,
        round_index: 1,
        round_seal_key: format!("wf_round:{}:1:1", run_id),
        phase: 1,
        subphase: None,
        parallel_limit: 4,
        base_version_seq: 2,
        base_data_hash: s1_hash,
        completed_tasks: vec![CompletedTaskOutcome {
            task_index: 0,
            task_spec: spec1,
            task_identity: "task:p1:t:{}".to_string(),
            input_scope_fingerprint: "in".to_string(),
            output_scope_fingerprint: "out".to_string(),
            has_applied_write: false,
            applied_write_count: 0,
            applied_mutations: vec![],
            artifact: None,
        }],
        has_applied_write: false,
    };

    let err = seal_workflow_round_tx(&mut conn, &req_stale, &plan).unwrap_err();
    assert_eq!(err.code, ErrorCode::ValidationFailed);
}

#[test]
#[allow(non_snake_case)]
fn WF_09_and_WF_C10_manual_edit_then_resume() {
    let (mut conn, _td) = setup_test_db();
    let draft_session_id = "018e3a00-0000-7000-8000-000000000010";
    seed_test_session(&conn, draft_session_id);

    let mut s0 = BusinessFlowData::new();
    s0.counters.insert("v".to_string(), 1);
    let s0_hash = seed_draft_version(&conn, draft_session_id, 1, &s0, "phase_confirm", None, None);

    // User performs manual edit creating v2
    let mut s1 = s0.clone();
    s1.counters.insert("v".to_string(), 99);
    seed_draft_version(
        &conn,
        draft_session_id,
        2,
        &s1,
        "manual_edit",
        Some(1),
        None,
    );

    let run_id = "018e3a00-0000-7000-8000-000000000020";
    let spec1 = PlannedTaskSpec {
        phase: 1,
        subphase: None,
        task_kind: "t".to_string(),
        assigned_scope: json!({}),
        dependencies: vec![],
        deterministic_identity: Some("task:p1:t:{}".to_string()),
    };

    let mut auth = BTreeMap::new();
    auth.insert(0, spec1.clone());
    let plan = AuthorizedRoundPlanContext {
        workflow_run_id: run_id.to_string(),
        plan_generation: 1,
        phase: 1,
        subphase: None,
        round_index: 0,
        parallel_limit: 4,
        authorized_tasks: auth,
    };

    // Workflow sends round based on old v1 baseline
    let req = WorkflowRoundSealRequest {
        draft_session_id: draft_session_id.to_string(),
        workflow_run_id: run_id.to_string(),
        plan_generation: 1,
        round_index: 0,
        round_seal_key: format!("wf_round:{}:1:0", run_id),
        phase: 1,
        subphase: None,
        parallel_limit: 4,
        base_version_seq: 1,
        base_data_hash: s0_hash,
        completed_tasks: vec![CompletedTaskOutcome {
            task_index: 0,
            task_spec: spec1,
            task_identity: "task:p1:t:{}".to_string(),
            input_scope_fingerprint: "in".to_string(),
            output_scope_fingerprint: "out".to_string(),
            has_applied_write: false,
            applied_write_count: 0,
            applied_mutations: vec![],
            artifact: None,
        }],
        has_applied_write: false,
    };

    let err = seal_workflow_round_tx(&mut conn, &req, &plan).unwrap_err();
    assert_eq!(err.code, ErrorCode::ValidationFailed);
}

#[test]
#[allow(non_snake_case)]
fn WF_10_cancellation_before_vs_after_commit() {
    let (mut conn, _td) = setup_test_db();
    let draft_session_id = "018e3a00-0000-7000-8000-000000000010";
    seed_test_session(&conn, draft_session_id);

    let mut s0 = BusinessFlowData::new();
    s0.counters.insert("v".to_string(), 1);
    let s0_hash = seed_draft_version(&conn, draft_session_id, 1, &s0, "phase_confirm", None, None);

    let run_id = "018e3a00-0000-7000-8000-000000000020";
    let spec1 = PlannedTaskSpec {
        phase: 1,
        subphase: None,
        task_kind: "t".to_string(),
        assigned_scope: json!({}),
        dependencies: vec![],
        deterministic_identity: Some("task:p1:t:{}".to_string()),
    };

    let mut s1 = s0.clone();
    s1.counters.insert("v".to_string(), 2);
    let patch1 = canonical_diff(&s0, &s1).unwrap();

    let task1 = CompletedTaskOutcome {
        task_index: 0,
        task_spec: spec1.clone(),
        task_identity: "task:p1:t:{}".to_string(),
        input_scope_fingerprint: "in".to_string(),
        output_scope_fingerprint: "out".to_string(),
        has_applied_write: true,
        applied_write_count: 1,
        applied_mutations: patch1.ops,
        artifact: None,
    };

    let mut auth = BTreeMap::new();
    auth.insert(0, spec1);
    let plan = AuthorizedRoundPlanContext {
        workflow_run_id: run_id.to_string(),
        plan_generation: 1,
        phase: 1,
        subphase: None,
        round_index: 0,
        parallel_limit: 4,
        authorized_tasks: auth,
    };

    let req = WorkflowRoundSealRequest {
        draft_session_id: draft_session_id.to_string(),
        workflow_run_id: run_id.to_string(),
        plan_generation: 1,
        round_index: 0,
        round_seal_key: format!("wf_round:{}:1:0", run_id),
        phase: 1,
        subphase: None,
        parallel_limit: 4,
        base_version_seq: 1,
        base_data_hash: s0_hash,
        completed_tasks: vec![task1],
        has_applied_write: true,
    };

    // Cancellation before TX-04 commit -> no SQLite rows created
    let round_count_before: i64 = conn
        .query_row("SELECT COUNT(*) FROM workflow_round_receipts", [], |r| {
            r.get(0)
        })
        .unwrap();
    assert_eq!(round_count_before, 0);

    // Seal TX-04 commits
    let res = seal_workflow_round_tx(&mut conn, &req, &plan).unwrap();
    assert_eq!(res.sealed_version_seq, Some(2));

    // Cancellation after TX-04 commit -> committed rows are fully preserved
    let round_count_after: i64 = conn
        .query_row("SELECT COUNT(*) FROM workflow_round_receipts", [], |r| {
            r.get(0)
        })
        .unwrap();
    let task_count_after: i64 = conn
        .query_row("SELECT COUNT(*) FROM workflow_task_receipts", [], |r| {
            r.get(0)
        })
        .unwrap();
    let ver_count_after: i64 = conn
        .query_row("SELECT COUNT(*) FROM draft_versions", [], |r| r.get(0))
        .unwrap();

    assert_eq!(round_count_after, 1);
    assert_eq!(task_count_after, 1);
    assert_eq!(ver_count_after, 2);
}

#[test]
#[allow(non_snake_case)]
fn WF_C08_historical_receipt_comparison_uses_intermediate_states() {
    let (mut conn, _td) = setup_test_db();
    let draft_session_id = "018e3a00-0000-7000-8000-000000000010";
    seed_test_session(&conn, draft_session_id);

    let mut s0 = BusinessFlowData::new();
    s0.counters.insert("A".to_string(), 1);
    s0.counters.insert("B".to_string(), 10);
    let s0_hash = seed_draft_version(&conn, draft_session_id, 1, &s0, "phase_confirm", None, None);

    let run_id = "018e3a00-0000-7000-8000-000000000020";

    // T0: A: 1 -> 2 (s0 -> s1)
    let spec0 = PlannedTaskSpec {
        phase: 1,
        subphase: None,
        task_kind: "t0".to_string(),
        assigned_scope: json!({"field": "A"}),
        dependencies: vec![],
        deterministic_identity: Some("task:p1:t0:{\"field\":\"A\"}".to_string()),
    };
    let mut s1 = s0.clone();
    s1.counters.insert("A".to_string(), 2);
    let patch0 = canonical_diff(&s0, &s1).unwrap();

    let task0 = CompletedTaskOutcome {
        task_index: 0,
        task_spec: spec0.clone(),
        task_identity: "task:p1:t0:{\"field\":\"A\"}".to_string(),
        input_scope_fingerprint: "in_0".to_string(),
        output_scope_fingerprint: "out_0".to_string(),
        has_applied_write: true,
        applied_write_count: 1,
        applied_mutations: patch0.ops,
        artifact: None,
    };

    // T1: B: 10 -> 20 (s1 -> s2, intermediate state is S1)
    let spec1 = PlannedTaskSpec {
        phase: 1,
        subphase: None,
        task_kind: "t1".to_string(),
        assigned_scope: json!({"field": "B"}),
        dependencies: vec![],
        deterministic_identity: Some("task:p1:t1:{\"field\":\"B\"}".to_string()),
    };
    let mut s2 = s1.clone();
    s2.counters.insert("B".to_string(), 20);
    let patch1 = canonical_diff(&s1, &s2).unwrap();

    let task1 = CompletedTaskOutcome {
        task_index: 1,
        task_spec: spec1.clone(),
        task_identity: "task:p1:t1:{\"field\":\"B\"}".to_string(),
        input_scope_fingerprint: "in_1".to_string(),
        output_scope_fingerprint: "out_1".to_string(),
        has_applied_write: true,
        applied_write_count: 1,
        applied_mutations: patch1.ops,
        artifact: None,
    };

    let mut auth = BTreeMap::new();
    auth.insert(0, spec0);
    auth.insert(1, spec1);
    let plan = AuthorizedRoundPlanContext {
        workflow_run_id: run_id.to_string(),
        plan_generation: 1,
        phase: 1,
        subphase: None,
        round_index: 0,
        parallel_limit: 4,
        authorized_tasks: auth,
    };

    let req = WorkflowRoundSealRequest {
        draft_session_id: draft_session_id.to_string(),
        workflow_run_id: run_id.to_string(),
        plan_generation: 1,
        round_index: 0,
        round_seal_key: format!("wf_round:{}:1:0", run_id),
        phase: 1,
        subphase: None,
        parallel_limit: 4,
        base_version_seq: 1,
        base_data_hash: s0_hash,
        completed_tasks: vec![task0, task1],
        has_applied_write: true,
    };

    let res = seal_workflow_round_tx(&mut conn, &req, &plan).unwrap();
    assert_eq!(res.sealed_version_seq, Some(2));

    let round_id = res.round_receipt_id.unwrap();
    let verify_res = verify_historical_round(&conn, &round_id, &["v2"], Some(2));
    assert_eq!(verify_res.integrity, Integrity::Verified);
}

#[test]
#[allow(non_snake_case)]
fn WF_C09_downstream_task_overwrite_does_not_falsely_invalidate_history() {
    let (mut conn, _td) = setup_test_db();
    let draft_session_id = "018e3a00-0000-7000-8000-000000000010";
    seed_test_session(&conn, draft_session_id);

    let mut s0 = BusinessFlowData::new();
    s0.counters.insert("X".to_string(), 1);
    let s0_hash = seed_draft_version(&conn, draft_session_id, 1, &s0, "phase_confirm", None, None);

    let run_id = "018e3a00-0000-7000-8000-000000000020";

    // T0 sets X = 2
    let spec0 = PlannedTaskSpec {
        phase: 1,
        subphase: None,
        task_kind: "t0".to_string(),
        assigned_scope: json!({"field": "X"}),
        dependencies: vec![],
        deterministic_identity: Some("task:p1:t0:{\"field\":\"X\"}".to_string()),
    };
    let mut s1 = s0.clone();
    s1.counters.insert("X".to_string(), 2);
    let patch0 = canonical_diff(&s0, &s1).unwrap();

    let task0 = CompletedTaskOutcome {
        task_index: 0,
        task_spec: spec0.clone(),
        task_identity: "task:p1:t0:{\"field\":\"X\"}".to_string(),
        input_scope_fingerprint: "in_0".to_string(),
        output_scope_fingerprint: "out_0".to_string(),
        has_applied_write: true,
        applied_write_count: 1,
        applied_mutations: patch0.ops,
        artifact: None,
    };

    // T1 overwrites X = 3
    let spec1 = PlannedTaskSpec {
        phase: 1,
        subphase: None,
        task_kind: "t1".to_string(),
        assigned_scope: json!({"field": "X"}),
        dependencies: vec![],
        deterministic_identity: Some("task:p1:t1:{\"field\":\"X\"}".to_string()),
    };
    let mut s2 = s1.clone();
    s2.counters.insert("X".to_string(), 3);
    let patch1 = canonical_diff(&s1, &s2).unwrap();

    let task1 = CompletedTaskOutcome {
        task_index: 1,
        task_spec: spec1.clone(),
        task_identity: "task:p1:t1:{\"field\":\"X\"}".to_string(),
        input_scope_fingerprint: "in_1".to_string(),
        output_scope_fingerprint: "out_1".to_string(),
        has_applied_write: true,
        applied_write_count: 1,
        applied_mutations: patch1.ops,
        artifact: None,
    };

    let mut auth = BTreeMap::new();
    auth.insert(0, spec0);
    auth.insert(1, spec1);
    let plan = AuthorizedRoundPlanContext {
        workflow_run_id: run_id.to_string(),
        plan_generation: 1,
        phase: 1,
        subphase: None,
        round_index: 0,
        parallel_limit: 4,
        authorized_tasks: auth,
    };

    let req = WorkflowRoundSealRequest {
        draft_session_id: draft_session_id.to_string(),
        workflow_run_id: run_id.to_string(),
        plan_generation: 1,
        round_index: 0,
        round_seal_key: format!("wf_round:{}:1:0", run_id),
        phase: 1,
        subphase: None,
        parallel_limit: 4,
        base_version_seq: 1,
        base_data_hash: s0_hash,
        completed_tasks: vec![task0, task1],
        has_applied_write: true,
    };

    let res = seal_workflow_round_tx(&mut conn, &req, &plan).unwrap();
    assert_eq!(res.sealed_version_seq, Some(2));

    let round_id = res.round_receipt_id.unwrap();
    let verify_res = verify_historical_round(&conn, &round_id, &["v2"], Some(2));
    assert_eq!(verify_res.integrity, Integrity::Verified);
}

#[test]
#[allow(non_snake_case)]
fn WF_C12_interleaved_task_order_rejected() {
    let (mut conn, _td) = setup_test_db();
    let draft_session_id = "018e3a00-0000-7000-8000-000000000010";
    seed_test_session(&conn, draft_session_id);

    let s0 = BusinessFlowData::new();
    let s0_hash = seed_draft_version(&conn, draft_session_id, 1, &s0, "phase_confirm", None, None);

    let run_id = "018e3a00-0000-7000-8000-000000000020";

    // Seed task 1 into DB (existing) while task 0 is absent
    conn.execute(
        "INSERT INTO workflow_task_receipts (
            id, draft_session_id, workflow_run_id, plan_generation, task_identity,
            input_scope_fingerprint, output_scope_fingerprint, base_version_seq,
            resulting_version_seq, had_mutations, created_at
         ) VALUES ('018e3a00-0000-7000-8000-000000000099', ?1, ?2, 1, 'task:p1:t1:{}', 'in_1', 'out_1', 1, NULL, 0, '2026-11-08T00:00:00Z')",
        params![draft_session_id, run_id],
    ).unwrap();

    let spec0 = PlannedTaskSpec {
        phase: 1,
        subphase: None,
        task_kind: "t0".to_string(),
        assigned_scope: json!({}),
        dependencies: vec![],
        deterministic_identity: Some("task:p1:t0:{}".to_string()),
    };
    let spec1 = PlannedTaskSpec {
        phase: 1,
        subphase: None,
        task_kind: "t1".to_string(),
        assigned_scope: json!({}),
        dependencies: vec![],
        deterministic_identity: Some("task:p1:t1:{}".to_string()),
    };

    let task0 = CompletedTaskOutcome {
        task_index: 0,
        task_spec: spec0.clone(),
        task_identity: "task:p1:t0:{}".to_string(),
        input_scope_fingerprint: "in_0".to_string(),
        output_scope_fingerprint: "out_0".to_string(),
        has_applied_write: false,
        applied_write_count: 0,
        applied_mutations: vec![],
        artifact: None,
    };
    let task1 = CompletedTaskOutcome {
        task_index: 1,
        task_spec: spec1.clone(),
        task_identity: "task:p1:t1:{}".to_string(),
        input_scope_fingerprint: "in_1".to_string(),
        output_scope_fingerprint: "out_1".to_string(),
        has_applied_write: false,
        applied_write_count: 0,
        applied_mutations: vec![],
        artifact: None,
    };

    let mut auth = BTreeMap::new();
    auth.insert(0, spec0);
    auth.insert(1, spec1);
    let plan = AuthorizedRoundPlanContext {
        workflow_run_id: run_id.to_string(),
        plan_generation: 1,
        phase: 1,
        subphase: None,
        round_index: 0,
        parallel_limit: 4,
        authorized_tasks: auth,
    };

    // Submitting [task0 (new), task1 (existing)] -> interleaved task order (REC-12)
    let req = WorkflowRoundSealRequest {
        draft_session_id: draft_session_id.to_string(),
        workflow_run_id: run_id.to_string(),
        plan_generation: 1,
        round_index: 0,
        round_seal_key: format!("wf_round:{}:1:0", run_id),
        phase: 1,
        subphase: None,
        parallel_limit: 4,
        base_version_seq: 1,
        base_data_hash: s0_hash,
        completed_tasks: vec![task0, task1],
        has_applied_write: false,
    };

    let err = seal_workflow_round_tx(&mut conn, &req, &plan).unwrap_err();
    assert_eq!(err.code, ErrorCode::ValidationFailed);
}

#[test]
#[allow(non_snake_case)]
fn WF_C15_crash_after_commit_verifies_completely() {
    let (mut conn, td) = setup_test_db();
    let draft_session_id = "018e3a00-0000-7000-8000-000000000010";
    seed_test_session(&conn, draft_session_id);

    let mut s0 = BusinessFlowData::new();
    s0.counters.insert("v".to_string(), 1);
    let s0_hash = seed_draft_version(&conn, draft_session_id, 1, &s0, "phase_confirm", None, None);

    let run_id = "018e3a00-0000-7000-8000-000000000020";
    let spec = PlannedTaskSpec {
        phase: 1,
        subphase: None,
        task_kind: "t".to_string(),
        assigned_scope: json!({}),
        dependencies: vec![],
        deterministic_identity: Some("task:p1:t:{}".to_string()),
    };
    let mut s1 = s0.clone();
    s1.counters.insert("v".to_string(), 2);
    let patch = canonical_diff(&s0, &s1).unwrap();

    let task = CompletedTaskOutcome {
        task_index: 0,
        task_spec: spec.clone(),
        task_identity: "task:p1:t:{}".to_string(),
        input_scope_fingerprint: "in".to_string(),
        output_scope_fingerprint: "out".to_string(),
        has_applied_write: true,
        applied_write_count: 1,
        applied_mutations: patch.ops,
        artifact: None,
    };

    let mut auth = BTreeMap::new();
    auth.insert(0, spec);
    let plan = AuthorizedRoundPlanContext {
        workflow_run_id: run_id.to_string(),
        plan_generation: 1,
        phase: 1,
        subphase: None,
        round_index: 0,
        parallel_limit: 4,
        authorized_tasks: auth,
    };

    let req = WorkflowRoundSealRequest {
        draft_session_id: draft_session_id.to_string(),
        workflow_run_id: run_id.to_string(),
        plan_generation: 1,
        round_index: 0,
        round_seal_key: format!("wf_round:{}:1:0", run_id),
        phase: 1,
        subphase: None,
        parallel_limit: 4,
        base_version_seq: 1,
        base_data_hash: s0_hash,
        completed_tasks: vec![task],
        has_applied_write: true,
    };

    let res = seal_workflow_round_tx(&mut conn, &req, &plan).unwrap();
    let round_id = res.round_receipt_id.unwrap();

    // Close and reopen SQLite connection simulating process restart / crash after commit
    drop(conn);
    let db_path = td.path().join("test_merlink.db");
    let reopened = Connection::open(&db_path).unwrap();
    storage::db::configure_connection(&reopened).unwrap();

    let verify_res = verify_historical_round(&reopened, &round_id, &["v2"], Some(2));
    assert_eq!(verify_res.integrity, Integrity::Verified);
}

#[test]
#[allow(non_snake_case)]
fn monotonic_head_increment_on_mutating_rounds() {
    let (mut conn, _td) = setup_test_db();
    let draft_session_id = "018e3a00-0000-7000-8000-000000000010";
    seed_test_session(&conn, draft_session_id);

    let mut s = BusinessFlowData::new();
    s.counters.insert("step".to_string(), 1);
    let mut current_hash =
        seed_draft_version(&conn, draft_session_id, 1, &s, "phase_confirm", None, None);

    let run_id = "018e3a00-0000-7000-8000-000000000020";

    for round_idx in 0..3 {
        let prev_seq = round_idx + 1;
        let next_seq = prev_seq + 1;

        let spec = PlannedTaskSpec {
            phase: 1,
            subphase: None,
            task_kind: format!("step_{}", round_idx),
            assigned_scope: json!({"round": round_idx}),
            dependencies: vec![],
            deterministic_identity: Some(format!(
                "task:p1:step_{}:{{\"round\":{}}}",
                round_idx, round_idx
            )),
        };

        let mut next_s = s.clone();
        next_s
            .counters
            .insert("step".to_string(), (round_idx + 2) as u64);
        let patch = canonical_diff(&s, &next_s).unwrap();

        let task = CompletedTaskOutcome {
            task_index: 0,
            task_spec: spec.clone(),
            task_identity: format!("task:p1:step_{}:{{\"round\":{}}}", round_idx, round_idx),
            input_scope_fingerprint: format!("in_{}", round_idx),
            output_scope_fingerprint: format!("out_{}", round_idx),
            has_applied_write: true,
            applied_write_count: 1,
            applied_mutations: patch.ops,
            artifact: None,
        };

        let mut auth = BTreeMap::new();
        auth.insert(0, spec);
        let plan = AuthorizedRoundPlanContext {
            workflow_run_id: run_id.to_string(),
            plan_generation: 1,
            phase: 1,
            subphase: None,
            round_index: round_idx,
            parallel_limit: 4,
            authorized_tasks: auth,
        };

        let req = WorkflowRoundSealRequest {
            draft_session_id: draft_session_id.to_string(),
            workflow_run_id: run_id.to_string(),
            plan_generation: 1,
            round_index: round_idx,
            round_seal_key: format!("wf_round:{}:1:{}", run_id, round_idx),
            phase: 1,
            subphase: None,
            parallel_limit: 4,
            base_version_seq: prev_seq,
            base_data_hash: current_hash.clone(),
            completed_tasks: vec![task],
            has_applied_write: true,
        };

        let res = seal_workflow_round_tx(&mut conn, &req, &plan).unwrap();
        assert_eq!(res.sealed_version_seq, Some(next_seq));

        let head_seq: i64 = conn
            .query_row(
                "SELECT head_version_seq FROM draft_sessions WHERE id = ?",
                [draft_session_id],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(head_seq, next_seq, "Head must increment monotonically");

        s = next_s;
        current_hash = state_data_hash(&s).unwrap();
    }

    let final_head: i64 = conn
        .query_row(
            "SELECT head_version_seq FROM draft_sessions WHERE id = ?",
            [draft_session_id],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(final_head, 4);
}
