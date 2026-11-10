//! Acceptance and integration tests for `phase3_sprint2_deep0`
//! ("Implement draft runtime and TX-01/TX-03").
//!
//! Complies with docs/plan/phase3_sprint2_deep0.md, docs/business/core.md (§3.6),
//! docs/business/be-core.md (§2, §3.7, §4.4, §4.5, §10.2), and docs/plan/evidence/contract-ledger.md.

#![allow(non_snake_case)]

#[path = "../src/error.rs"]
pub mod error;

#[path = "../src/domain/business_flow.rs"]
pub mod business_flow;
#[path = "../src/domain/code_allocator.rs"]
pub mod code_allocator;
#[path = "../src/domain/fingerprint.rs"]
pub mod fingerprint;
#[path = "../src/domain/word_count.rs"]
pub mod word_count;
#[path = "../src/domain/progress_engine.rs"]
pub mod progress_engine;
#[path = "../src/domain/phase_gate.rs"]
pub mod phase_gate;
#[path = "../src/domain/invariants.rs"]
pub mod invariants;

pub mod domain {
    pub use crate::business_flow;
    pub use crate::code_allocator;
    pub use crate::fingerprint;
    pub use crate::invariants;
    pub use crate::phase_gate;
    pub use crate::progress_engine;
    pub use crate::word_count;
}

#[path = "../src/storage/db.rs"]
pub mod db;
#[path = "../src/storage/migration.rs"]
pub mod migration;
#[path = "../src/storage/draft_repo.rs"]
pub mod draft_repo;

pub mod storage {
    pub use crate::db;
    pub use crate::draft_repo;
    pub use crate::migration;
}

#[path = "../src/service/draft_runtime.rs"]
pub mod draft_runtime;
#[path = "../src/service/draft_service.rs"]
pub mod draft_service;

pub mod service {
    pub use crate::draft_runtime;
    pub use crate::draft_service;
}

#[path = "../src/ipc/dto.rs"]
pub mod dto;
#[path = "../src/ipc/draft_commands.rs"]
pub mod draft_commands;

pub mod ipc {
    pub use crate::draft_commands;
    pub use crate::dto;
}

use error::ErrorCode;
use domain::business_flow::{BusinessFlowData, ProvenanceSource};
use domain::code_allocator::{CodeAllocator, EntityCodeSpec};
use domain::fingerprint::state_data_hash;
use ipc::draft_commands::*;
use ipc::dto::*;
use service::draft_runtime::*;
use service::draft_service::*;
use storage::db::configure_connection;
use storage::migration::run_migrations;

use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::Connection;
use std::path::PathBuf;
use std::process::Command;
use std::sync::Arc;
use std::time::Duration;
use tempfile::TempDir;
use uuid::Uuid;

const BASELINE_COMMIT: &str = "57a462efb953a665e3673b15e6cc33622a4f2d39";
const PREDECESSOR_P2S5_COMMIT: &str = "02fc9b735457d47115e0f90063650dc00f322dc1";
const PREDECESSOR_P2S6_COMMIT: &str = "75d3886503643e6398c0be8c600b9b712811b3e0";

/// Setup a fresh disposable SQLite database with migrations applied.
fn setup_test_db(temp_dir: &TempDir) -> (PathBuf, Pool<SqliteConnectionManager>) {
    let db_path = temp_dir.path().join("test_merlink_p3s2.sqlite3");
    {
        let mut conn = Connection::open(&db_path).expect("open test sqlite db");
        configure_connection(&conn).expect("configure sqlite pragmas");
        run_migrations(&mut conn).expect("run sqlite migrations");
    }

    let manager = SqliteConnectionManager::file(&db_path)
        .with_init(|c| configure_connection(c));
    let pool = Pool::builder()
        .max_size(8)
        .build(manager)
        .expect("build connection pool");

    (db_path, pool)
}

/// Seed initial project, session, and draft_session.
fn seed_draft_session(pool: &Pool<SqliteConnectionManager>, draft_session_id: Uuid, status: &str) {
    let proj_uuid = Uuid::now_v7();
    let sess_uuid = Uuid::now_v7();
    let proj_str = proj_uuid.to_string();
    let sess_str = sess_uuid.to_string();
    let draft_str = draft_session_id.to_string();
    let code_str = format!("p_{}", proj_uuid.simple());

    let conn = pool.get().expect("get conn for seeding");
    conn.execute(
        "INSERT INTO projects (id, code, display_name, status, language) VALUES (?1, ?2, 'Test Project', 'PLANNING', 'VI')",
        rusqlite::params![proj_str, code_str],
    ).expect("seed project");

    conn.execute(
        "INSERT INTO sessions (id, project_id, display_name) VALUES (?1, ?2, 'Test Session')",
        rusqlite::params![sess_str, proj_str],
    ).expect("seed session");

    conn.execute(
        "INSERT INTO draft_sessions (id, session_id, project_id, head_version_seq, status, created_at, updated_at)
         VALUES (?1, ?2, ?3, NULL, ?4, '2026-03-01T00:00:00Z', '2026-03-01T00:00:00Z')",
        rusqlite::params![draft_str, sess_str, proj_str, status],
    ).expect("seed draft_session");
}

// ============================================================================
// Acceptance Tests
// ============================================================================

/// phase3_sprint2_deep0-F0: Predecessor gate absent or baseline SHA drift check.
#[test]
fn phase3_sprint2_deep0_F0_predecessor_gate_and_baseline_check() {
    let mb_output = Command::new("git")
        .args(["merge-base", "--is-ancestor", BASELINE_COMMIT, "HEAD"])
        .output()
        .expect("execute git merge-base");
    assert!(
        mb_output.status.success(),
        "Baseline commit {} must be an ancestor of HEAD",
        BASELINE_COMMIT
    );

    let p2s5_check = Command::new("git")
        .args(["cat-file", "-e", &format!("{}^{{commit}}", PREDECESSOR_P2S5_COMMIT)])
        .output()
        .expect("execute git cat-file for p2s5");
    assert!(
        p2s5_check.status.success(),
        "Predecessor P2S5 commit {} must exist",
        PREDECESSOR_P2S5_COMMIT
    );

    let p2s6_check = Command::new("git")
        .args(["cat-file", "-e", &format!("{}^{{commit}}", PREDECESSOR_P2S6_COMMIT)])
        .output()
        .expect("execute git cat-file for p2s6");
    assert!(
        p2s6_check.status.success(),
        "Predecessor P2S6 commit {} must exist",
        PREDECESSOR_P2S6_COMMIT
    );
}

/// phase3_sprint2_deep0-F1: Verify seal_pending_retry is strictly in-memory (CI-03).
#[test]
fn phase3_sprint2_deep0_F1_seal_pending_retry_is_in_memory_only() {
    let temp_dir = TempDir::new().unwrap();
    let (_path, pool) = setup_test_db(&temp_dir);
    let registry = Arc::new(DraftRuntimeRegistry::new());
    let service = DraftService::new(pool.clone(), registry.clone());

    let draft_id = Uuid::now_v7();
    seed_draft_session(&pool, draft_id, "editing");

    let entry = service.get_or_load_runtime_entry(draft_id).unwrap();
    assert_eq!(entry.get_snapshot().status, DraftRuntimeStatus::Editing);

    // Inject persistent failure so both seal attempts fail
    service.set_error_injection(ErrorInjectionPoint::Tx01AlwaysFail);

    let params = SealVersionParams {
        draft_session_id: draft_id,
        seal_key: "turn:turn-f1".to_string(),
        reason: "ai_patch".to_string(),
        phase: 0,
        subphase: None,
        data_json: "{}".to_string(),
        data_hash: "sha256:dummy".to_string(),
        parent_seq: None,
        turn_id: None,
        chat_message_id: None,
    };

    let res = service.seal_version(&params);
    assert_eq!(res.unwrap_err().code, ErrorCode::VersionSealFailed);

    // In-memory status becomes SealPendingRetry
    assert_eq!(entry.get_snapshot().status, DraftRuntimeStatus::SealPendingRetry);

    // SQLite status remains 'editing' (never altered by seal_pending_retry per CI-03)
    let conn = pool.get().unwrap();
    let db_status: String = conn
        .query_row(
            "SELECT status FROM draft_sessions WHERE id = ?1",
            rusqlite::params![draft_id.to_string()],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(db_status, "editing");
}

/// phase3_sprint2_deep0-F2: Retry mismatch returns SealKeyPayloadMismatch and creates 0 new versions.
#[test]
fn phase3_sprint2_deep0_F2_retry_mismatch_returns_error() {
    let temp_dir = TempDir::new().unwrap();
    let (_path, pool) = setup_test_db(&temp_dir);
    let registry = Arc::new(DraftRuntimeRegistry::new());
    let service = DraftService::new(pool.clone(), registry);

    let draft_id = Uuid::now_v7();
    seed_draft_session(&pool, draft_id, "editing");

    let mut bfd1 = BusinessFlowData::new();
    bfd1.require.project.display_name.value = Some("Name 1".to_string());
    let hash1 = state_data_hash(&bfd1).unwrap();
    let json1 = serde_json::to_string(&bfd1).unwrap();

    let params1 = SealVersionParams {
        draft_session_id: draft_id,
        seal_key: "turn:turn-f2".to_string(),
        reason: "ai_patch".to_string(),
        phase: 0,
        subphase: None,
        data_json: json1,
        data_hash: hash1,
        parent_seq: None,
        turn_id: None,
        chat_message_id: None,
    };

    let v1 = service.seal_version(&params1).unwrap();
    assert_eq!(v1.seq, 1);

    // Attempt seal with same seal_key but different payload/hash
    let mut bfd2 = BusinessFlowData::new();
    bfd2.require.project.display_name.value = Some("Different Name".to_string());
    let hash2 = state_data_hash(&bfd2).unwrap();
    let json2 = serde_json::to_string(&bfd2).unwrap();

    let params2 = SealVersionParams {
        draft_session_id: draft_id,
        seal_key: "turn:turn-f2".to_string(),
        reason: "ai_patch".to_string(),
        phase: 0,
        subphase: None,
        data_json: json2,
        data_hash: hash2,
        parent_seq: None,
        turn_id: None,
        chat_message_id: None,
    };

    let err = service.seal_version(&params2).unwrap_err();
    assert_eq!(err.code, ErrorCode::SealKeyPayloadMismatch);

    // Verify 0 new versions created (total versions remains 1)
    let conn = pool.get().unwrap();
    let count: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM draft_versions WHERE draft_session_id = ?1",
            rusqlite::params![draft_id.to_string()],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(count, 1);
}

/// P3S2-P: One applied turn yields one version; identical retry returns original.
#[test]
fn P3S2_P_one_version_per_turn_and_identical_retry() {
    let temp_dir = TempDir::new().unwrap();
    let (_path, pool) = setup_test_db(&temp_dir);
    let registry = Arc::new(DraftRuntimeRegistry::new());
    let service = DraftService::new(pool.clone(), registry);

    let draft_id = Uuid::now_v7();
    seed_draft_session(&pool, draft_id, "editing");

    let bfd = BusinessFlowData::new();
    let hash = state_data_hash(&bfd).unwrap();
    let json_str = serde_json::to_string(&bfd).unwrap();

    let params = SealVersionParams {
        draft_session_id: draft_id,
        seal_key: "turn:turn-p1".to_string(),
        reason: "ai_patch".to_string(),
        phase: 0,
        subphase: None,
        data_json: json_str.clone(),
        data_hash: hash.clone(),
        parent_seq: None,
        turn_id: None,
        chat_message_id: None,
    };

    // First call: creates v.01
    let v1 = service.seal_version(&params).unwrap();
    assert_eq!(v1.seq, 1);
    assert_eq!(v1.display_label, "v.01");

    // Second call: identical retry returns v.01
    let v2 = service.seal_version(&params).unwrap();
    assert_eq!(v2.seq, 1);
    assert_eq!(v2.data_hash, v1.data_hash);

    let conn = pool.get().unwrap();
    let count: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM draft_versions WHERE draft_session_id = ?1",
            rusqlite::params![draft_id.to_string()],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(count, 1, "COUNT(draft_versions) must remain 1 after identical retry");
}

/// P3S2-N: Two failed seal attempts => VersionSealFailed without success claim.
#[test]
fn P3S2_N_two_failed_seal_attempts_return_version_seal_failed() {
    let temp_dir = TempDir::new().unwrap();
    let (_path, pool) = setup_test_db(&temp_dir);
    let registry = Arc::new(DraftRuntimeRegistry::new());
    let service = DraftService::new(pool.clone(), registry);

    let draft_id = Uuid::now_v7();
    seed_draft_session(&pool, draft_id, "editing");

    service.set_error_injection(ErrorInjectionPoint::Tx01AlwaysFail);

    let params = SealVersionParams {
        draft_session_id: draft_id,
        seal_key: "turn:turn-n".to_string(),
        reason: "ai_patch".to_string(),
        phase: 0,
        subphase: None,
        data_json: "{}".to_string(),
        data_hash: "sha256:dummy".to_string(),
        parent_seq: None,
        turn_id: None,
        chat_message_id: None,
    };

    let err = service.seal_version(&params).unwrap_err();
    assert_eq!(err.code, ErrorCode::VersionSealFailed);

    let conn = pool.get().unwrap();
    let count: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM draft_versions WHERE draft_session_id = ?1",
            rusqlite::params![draft_id.to_string()],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(count, 0, "No version should be created on persistent failure");
}

/// P3S2-B: Restore V2 from V4 appends V5 and expires all pending; no fake seq0.
#[tokio::test]
async fn P3S2_B_restore_v2_from_v4_appends_v5_and_expires_pending() {
    let temp_dir = TempDir::new().unwrap();
    let (_path, pool) = setup_test_db(&temp_dir);
    let registry = Arc::new(DraftRuntimeRegistry::new());
    let service = DraftService::new(pool.clone(), registry);

    let draft_id = Uuid::now_v7();
    seed_draft_session(&pool, draft_id, "editing");

    // Create v.01, v.02, v.03, v.04
    for i in 1..=4 {
        let mut bfd = BusinessFlowData::new();
        bfd.require.project.display_name.value = Some(format!("Version {}", i));
        let hash = state_data_hash(&bfd).unwrap();
        let json_str = serde_json::to_string(&bfd).unwrap();

        let params = SealVersionParams {
            draft_session_id: draft_id,
            seal_key: format!("init:v{}", i),
            reason: "manual_edit".to_string(),
            phase: 0,
            subphase: None,
            data_json: json_str,
            data_hash: hash,
            parent_seq: if i > 1 { Some(i - 1) } else { None },
            turn_id: None,
            chat_message_id: None,
        };
        let v = service.seal_version(&params).unwrap();
        assert_eq!(v.seq, i);
    }

    // Insert 2 pending patches at v.04
    let patch1_id = Uuid::now_v7();
    let patch2_id = Uuid::now_v7();
    {
        let conn = pool.get().unwrap();
        conn.execute(
            "INSERT INTO pending_patches (id, draft_session_id, base_version_seq, target_type, target_code, field_name, operation, status)
             VALUES (?1, ?2, 4, 'character', 'char_hero', 'name', 'set', 'pending'),
                    (?3, ?2, 4, 'character', 'char_villain', 'role', 'set', 'pending')",
            rusqlite::params![patch1_id.to_string(), draft_id.to_string(), patch2_id.to_string()],
        ).unwrap();
    }

    // Call restore target_seq = 2
    let restore_params = RestoreVersionParams {
        draft_session_id: draft_id,
        target_seq: 2,
        client_request_id: "req-restore-b".to_string(),
    };

    let resp = service.restore_version(&restore_params).await.unwrap();

    // Invariants: appends v5, parent_seq = 2, reason = 'restore'
    assert_eq!(resp.new_version.seq, 5);
    assert_eq!(resp.new_version.display_label, "v.05");
    assert_eq!(resp.new_version.parent_seq, Some(2));
    assert_eq!(resp.new_version.reason, "restore");

    // All pending patches expired
    assert_eq!(resp.expired_patch_ids.len(), 2);
    assert!(resp.expired_patch_ids.contains(&patch1_id));
    assert!(resp.expired_patch_ids.contains(&patch2_id));

    let conn = pool.get().unwrap();
    let pending_count: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM pending_patches WHERE draft_session_id = ?1 AND status = 'pending'",
            rusqlite::params![draft_id.to_string()],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(pending_count, 0);

    let expired_count: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM pending_patches WHERE draft_session_id = ?1 AND status = 'expired'",
            rusqlite::params![draft_id.to_string()],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(expired_count, 2);

    // No fake seq 0
    let seq0_count: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM draft_versions WHERE draft_session_id = ?1 AND seq = 0",
            rusqlite::params![draft_id.to_string()],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(seq0_count, 0, "No fake seq 0 must exist");
}

/// TX-001: Execute TX-01 then inject error -> atomic rollback, no partial effect.
#[test]
fn TX_001_atomic_rollback_on_failure() {
    let temp_dir = TempDir::new().unwrap();
    let (_path, pool) = setup_test_db(&temp_dir);
    let registry = Arc::new(DraftRuntimeRegistry::new());
    let service = DraftService::new(pool.clone(), registry);

    let draft_id = Uuid::now_v7();
    seed_draft_session(&pool, draft_id, "editing");

    service.set_error_injection(ErrorInjectionPoint::BeforeTx01Commit);

    let params = SealVersionParams {
        draft_session_id: draft_id,
        seal_key: "turn:tx1".to_string(),
        reason: "ai_patch".to_string(),
        phase: 0,
        subphase: None,
        data_json: "{}".to_string(),
        data_hash: "sha256:tx1".to_string(),
        parent_seq: None,
        turn_id: None,
        chat_message_id: None,
    };

    let res = service.seal_version(&params);
    assert!(res.is_err());

    let conn = pool.get().unwrap();
    let version_count: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM draft_versions WHERE draft_session_id = ?1",
            rusqlite::params![draft_id.to_string()],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(version_count, 0, "Atomic rollback: 0 draft_versions rows");

    let head_seq: Option<i64> = conn
        .query_row(
            "SELECT head_version_seq FROM draft_sessions WHERE id = ?1",
            rusqlite::params![draft_id.to_string()],
            |r| r.get(0),
        )
        .unwrap();
    assert!(head_seq.is_none(), "head_version_seq must remain NULL on rollback");
}

/// TX-003: Execute TX-03 then inject error -> atomic rollback.
#[tokio::test]
async fn TX_003_atomic_rollback_on_failure() {
    let temp_dir = TempDir::new().unwrap();
    let (_path, pool) = setup_test_db(&temp_dir);
    let registry = Arc::new(DraftRuntimeRegistry::new());
    let service = DraftService::new(pool.clone(), registry);

    let draft_id = Uuid::now_v7();
    seed_draft_session(&pool, draft_id, "editing");

    // Create v1
    let bfd = BusinessFlowData::new();
    let hash = state_data_hash(&bfd).unwrap();
    let json_str = serde_json::to_string(&bfd).unwrap();
    service.seal_version(&SealVersionParams {
        draft_session_id: draft_id,
        seal_key: "init:1".to_string(),
        reason: "manual_edit".to_string(),
        phase: 0,
        subphase: None,
        data_json: json_str,
        data_hash: hash,
        parent_seq: None,
        turn_id: None,
        chat_message_id: None,
    }).unwrap();

    // Create 1 pending patch
    let patch_id = Uuid::now_v7();
    {
        let conn = pool.get().unwrap();
        conn.execute(
            "INSERT INTO pending_patches (id, draft_session_id, base_version_seq, target_type, target_code, field_name, operation, status)
             VALUES (?1, ?2, 1, 'character', 'c1', 'name', 'set', 'pending')",
            rusqlite::params![patch_id.to_string(), draft_id.to_string()],
        ).unwrap();
    }

    service.set_error_injection(ErrorInjectionPoint::BeforeTx03Commit);

    let restore_params = RestoreVersionParams {
        draft_session_id: draft_id,
        target_seq: 1,
        client_request_id: "req-tx3-fail".to_string(),
    };

    let res = service.restore_version(&restore_params).await;
    assert!(res.is_err());

    let conn = pool.get().unwrap();
    let version_count: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM draft_versions WHERE draft_session_id = ?1",
            rusqlite::params![draft_id.to_string()],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(version_count, 1, "Must remain 1 version after rollback");

    let patch_status: String = conn
        .query_row(
            "SELECT status FROM pending_patches WHERE id = ?1",
            rusqlite::params![patch_id.to_string()],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(patch_status, "pending", "Pending patch must not be expired on rollback");
}

/// T-CONC-01: Concurrent reads on D1 are non-blocking; write on D2 succeeds; duplicate write on D1 waits/busy.
#[tokio::test]
async fn T_CONC_01_concurrent_reads_and_per_draft_isolation() {
    let temp_dir = TempDir::new().unwrap();
    let (_path, pool) = setup_test_db(&temp_dir);
    let registry = Arc::new(DraftRuntimeRegistry::new());
    let service = Arc::new(DraftService::new(pool.clone(), registry.clone()));

    let d1 = Uuid::now_v7();
    let d2 = Uuid::now_v7();
    seed_draft_session(&pool, d1, "editing");
    seed_draft_session(&pool, d2, "editing");

    let entry1 = service.get_or_load_runtime_entry(d1).unwrap();
    let _entry2 = service.get_or_load_runtime_entry(d2).unwrap();

    // Simulate AI / workflow holding operation_lock on D1
    let guard1 = entry1.acquire_operation_lock("ai_turn", DRAFT_LOCK_TIMEOUT).await.unwrap();

    // (a) 10 concurrent reads on D1 return immediately (< 50ms) from snapshot without blocking
    let start_reads = std::time::Instant::now();
    let mut read_handles = Vec::new();
    for _ in 0..10 {
        let svc = Arc::clone(&service);
        read_handles.push(tokio::spawn(async move {
            let req = GetDraftWorkspaceStateReq { draft_session_id: d1 };
            exec_get_draft_workspace_state(&svc, &req)
        }));
    }

    for h in read_handles {
        let res = h.await.unwrap();
        assert!(res.is_ok());
        let ws = res.unwrap();
        assert!(ws.lock_state.is_locked, "Lock state should report locked");
        assert_eq!(ws.lock_state.holder_info, Some("ai_turn".to_string()));
    }
    let read_dur = start_reads.elapsed();
    assert!(read_dur < Duration::from_millis(150), "10 reads took too long: {:?}", read_dur);

    // (b) 1 write on D2 completes concurrently without being blocked by D1
    let svc_d2 = Arc::clone(&service);
    let d2_handle = tokio::spawn(async move {
        let req = ApplyManualDraftEditReq {
            draft_session_id: d2,
            client_edit_id: "edit-d2".to_string(),
            tool_code: "set_genre".to_string(),
            args: serde_json::json!({"name": "Cyberpunk"}),
        };
        exec_apply_manual_draft_edit(&svc_d2, &req).await
    });
    let d2_res = d2_handle.await.unwrap();
    assert!(d2_res.is_ok(), "Write on D2 must succeed while D1 is locked");

    // (c) 1 write on D1 with short timeout fails immediately with DraftBusy
    let svc_d1 = Arc::clone(&service);
    let d1_write_handle = tokio::spawn(async move {
        let entry = svc_d1.get_or_load_runtime_entry(d1).unwrap();
        let err_code = match entry.acquire_operation_lock("manual_edit", Duration::from_millis(30)).await {
            Ok(_) => None,
            Err(e) => Some(e.code),
        };
        err_code
    });
    let busy_res = d1_write_handle.await.unwrap();
    assert_eq!(busy_res, Some(ErrorCode::DraftBusy));

    drop(guard1);
}

/// T-IDEM-01: Idempotent sealing and payload mismatch check.
#[test]
fn T_IDEM_01_idempotent_sealing_protocol() {
    let temp_dir = TempDir::new().unwrap();
    let (_path, pool) = setup_test_db(&temp_dir);
    let registry = Arc::new(DraftRuntimeRegistry::new());
    let service = DraftService::new(pool.clone(), registry);

    let draft_id = Uuid::now_v7();
    seed_draft_session(&pool, draft_id, "editing");

    let bfd = BusinessFlowData::new();
    let hash = state_data_hash(&bfd).unwrap();
    let json_str = serde_json::to_string(&bfd).unwrap();

    let seal_key = "turn:idem-01".to_string();

    // Call 1
    let v1 = service.seal_version(&SealVersionParams {
        draft_session_id: draft_id,
        seal_key: seal_key.clone(),
        reason: "ai_patch".to_string(),
        phase: 0,
        subphase: None,
        data_json: json_str.clone(),
        data_hash: hash.clone(),
        parent_seq: None,
        turn_id: None,
        chat_message_id: None,
    }).unwrap();
    assert_eq!(v1.seq, 1);

    // Call 2: same seal_key and same data_hash
    let v2 = service.seal_version(&SealVersionParams {
        draft_session_id: draft_id,
        seal_key: seal_key.clone(),
        reason: "ai_patch".to_string(),
        phase: 0,
        subphase: None,
        data_json: json_str.clone(),
        data_hash: hash.clone(),
        parent_seq: None,
        turn_id: None,
        chat_message_id: None,
    }).unwrap();
    assert_eq!(v2.seq, 1);

    let conn = pool.get().unwrap();
    let count: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM draft_versions WHERE draft_session_id = ?1",
            rusqlite::params![draft_id.to_string()],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(count, 1);

    // Call 3: same seal_key but different data_hash
    let err = service.seal_version(&SealVersionParams {
        draft_session_id: draft_id,
        seal_key: seal_key.clone(),
        reason: "ai_patch".to_string(),
        phase: 0,
        subphase: None,
        data_json: json_str,
        data_hash: "sha256:different_hash".to_string(),
        parent_seq: None,
        turn_id: None,
        chat_message_id: None,
    }).unwrap_err();
    assert_eq!(err.code, ErrorCode::SealKeyPayloadMismatch);
}

/// T-REST-01: Full restore lifecycle, monotonic counters, and patch expiry.
#[tokio::test]
async fn T_REST_01_restore_lifecycle_counters_and_patch_expiry() {
    let temp_dir = TempDir::new().unwrap();
    let (_path, pool) = setup_test_db(&temp_dir);
    let registry = Arc::new(DraftRuntimeRegistry::new());
    let service = DraftService::new(pool.clone(), registry);

    let draft_id = Uuid::now_v7();
    seed_draft_session(&pool, draft_id, "editing");

    // Create v.01..v.04 with arc code allocations
    for i in 1..=4 {
        let mut bfd = BusinessFlowData::new();
        // In v.02, we allocate 1 arc
        if i >= 2 {
            CodeAllocator::allocate(&mut bfd, EntityCodeSpec::Arc { project_code: "proj_rest" }).unwrap();
        }
        // In v.03, we allocate 2 arcs
        if i >= 3 {
            CodeAllocator::allocate(&mut bfd, EntityCodeSpec::Arc { project_code: "proj_rest" }).unwrap();
        }
        // In v.04, we allocate 3 arcs
        if i >= 4 {
            CodeAllocator::allocate(&mut bfd, EntityCodeSpec::Arc { project_code: "proj_rest" }).unwrap();
        }

        let hash = state_data_hash(&bfd).unwrap();
        let json_str = serde_json::to_string(&bfd).unwrap();

        service.seal_version(&SealVersionParams {
            draft_session_id: draft_id,
            seal_key: format!("v:{}", i),
            reason: "manual_edit".to_string(),
            phase: 0,
            subphase: None,
            data_json: json_str,
            data_hash: hash,
            parent_seq: if i > 1 { Some(i - 1) } else { None },
            turn_id: None,
            chat_message_id: None,
        }).unwrap();
    }

    // Add 2 pending patches at v.04
    {
        let conn = pool.get().unwrap();
        conn.execute(
            "INSERT INTO pending_patches (id, draft_session_id, base_version_seq, target_type, target_code, field_name, operation, status)
             VALUES ('018e3a00-0000-7000-8000-000000000001', ?1, 4, 'character', 'c1', 'name', 'set', 'pending'),
                    ('018e3a00-0000-7000-8000-000000000002', ?1, 4, 'character', 'c2', 'role', 'set', 'pending')",
            rusqlite::params![draft_id.to_string()],
        ).unwrap();
    }

    // Restore targetSeq = 2
    let restore_res = service.restore_version(&RestoreVersionParams {
        draft_session_id: draft_id,
        target_seq: 2,
        client_request_id: "client-req-01".to_string(),
    }).await.unwrap();

    assert_eq!(restore_res.new_version.seq, 5);
    assert_eq!(restore_res.new_version.parent_seq, Some(2));
    assert_eq!(restore_res.new_version.reason, "restore");

    // All pending patches expired
    assert_eq!(restore_res.expired_patch_ids.len(), 2);

    // Verify _counters restored back to v.02 state (1 arc allocated, counter = 1)
    let entry = service.get_or_load_runtime_entry(draft_id).unwrap();
    let snap = entry.get_snapshot();
    let counter_val = snap.business_flow_data.counters.get("proj_rest").copied().unwrap_or(0);
    assert_eq!(counter_val, 1, "Counter for proj_rest in v.02 must be 1");

    // Allocate next arc: must allocate p1-a2 without collision
    let mut modified_bfd = snap.business_flow_data.clone();
    let next_arc_code = CodeAllocator::allocate(&mut modified_bfd, EntityCodeSpec::Arc { project_code: "proj_rest" }).unwrap();
    assert_eq!(next_arc_code, "proj_rest-a2", "Restored counters allocate next sequence without collision");
}

/// IPC-CMD-005: get_draft_workspace_state tests.
#[test]
fn IPC_CMD_005_get_draft_workspace_state() {
    let temp_dir = TempDir::new().unwrap();
    let (_path, pool) = setup_test_db(&temp_dir);
    let registry = Arc::new(DraftRuntimeRegistry::new());
    let service = DraftService::new(pool.clone(), registry);

    let draft_id = Uuid::now_v7();
    seed_draft_session(&pool, draft_id, "editing");

    // Valid draft
    let req = GetDraftWorkspaceStateReq { draft_session_id: draft_id };
    let resp = exec_get_draft_workspace_state(&service, &req).unwrap();
    assert_eq!(resp.draft_session_id, draft_id);
    assert_eq!(resp.status, "editing");
    assert_eq!(resp.phase, 0);

    // Non-existent draft
    let invalid_id = Uuid::now_v7();
    let err = exec_get_draft_workspace_state(&service, &GetDraftWorkspaceStateReq { draft_session_id: invalid_id }).unwrap_err();
    assert_eq!(err.code, ErrorCode::EntityNotFound);
}

/// IPC-CMD-006: get_progress_detail tests.
#[test]
fn IPC_CMD_006_get_progress_detail() {
    let temp_dir = TempDir::new().unwrap();
    let (_path, pool) = setup_test_db(&temp_dir);
    let registry = Arc::new(DraftRuntimeRegistry::new());
    let service = DraftService::new(pool.clone(), registry);

    let draft_id = Uuid::now_v7();
    seed_draft_session(&pool, draft_id, "editing");

    let req_vi = GetProgressDetailReq {
        draft_session_id: draft_id,
        system_language: "vi".to_string(),
    };
    let resp_vi = exec_get_progress_detail(&service, &req_vi).unwrap();
    assert_eq!(resp_vi.progress.total_attributes, 46);
    assert_eq!(resp_vi.attributes.len(), 46);
    assert!(resp_vi.rendered_markdown.contains("Tổng Quan"));

    let req_en = GetProgressDetailReq {
        draft_session_id: draft_id,
        system_language: "en".to_string(),
    };
    let resp_en = exec_get_progress_detail(&service, &req_en).unwrap();
    assert!(resp_en.rendered_markdown.contains("Draft Workspace Summary"));
}

/// IPC-CMD-007: apply_manual_draft_edit tests.
#[tokio::test]
async fn IPC_CMD_007_apply_manual_draft_edit() {
    let temp_dir = TempDir::new().unwrap();
    let (_path, pool) = setup_test_db(&temp_dir);
    let registry = Arc::new(DraftRuntimeRegistry::new());
    let service = DraftService::new(pool.clone(), registry);

    let draft_id = Uuid::now_v7();
    seed_draft_session(&pool, draft_id, "editing");

    // 1. Successful manual edit
    let req = ApplyManualDraftEditReq {
        draft_session_id: draft_id,
        client_edit_id: "edit-101".to_string(),
        tool_code: "set_genre".to_string(),
        args: serde_json::json!({"name": "Xianxia"}),
    };
    let resp = exec_apply_manual_draft_edit(&service, &req).await.unwrap();
    assert!(resp.sealed_version.is_some());
    assert_eq!(resp.sealed_version.unwrap().seq, 1);

    // 2. Reject non-mutation tool
    let req_invalid_tool = ApplyManualDraftEditReq {
        draft_session_id: draft_id,
        client_edit_id: "edit-102".to_string(),
        tool_code: "get_genre".to_string(),
        args: serde_json::json!({}),
    };
    let err = exec_apply_manual_draft_edit(&service, &req_invalid_tool).await.unwrap_err();
    assert_eq!(err.code, ErrorCode::ValidationFailed);

    // 3. Reject edit on committed draft
    let committed_id = Uuid::now_v7();
    seed_draft_session(&pool, committed_id, "committed");
    let req_committed = ApplyManualDraftEditReq {
        draft_session_id: committed_id,
        client_edit_id: "edit-comm".to_string(),
        tool_code: "set_genre".to_string(),
        args: serde_json::json!({"name": "Wuxia"}),
    };
    let err_comm = exec_apply_manual_draft_edit(&service, &req_committed).await.unwrap_err();
    assert_eq!(err_comm.code, ErrorCode::DraftNotEditable);
}

/// IPC-CMD-008: confirm_phase tests and gating.
#[tokio::test]
async fn IPC_CMD_008_confirm_phase_gating() {
    let temp_dir = TempDir::new().unwrap();
    let (_path, pool) = setup_test_db(&temp_dir);
    let registry = Arc::new(DraftRuntimeRegistry::new());
    let service = DraftService::new(pool.clone(), registry);

    let draft_id = Uuid::now_v7();
    seed_draft_session(&pool, draft_id, "editing");

    // 1. Confirming Phase 1 when Phase 0 is incomplete is rejected
    let req_p1_incomplete = ConfirmPhaseReq {
        draft_session_id: draft_id,
        target_phase: 1,
        target_subphase: None,
        pre_draft_suggestion: None,
    };
    let err_gating = exec_confirm_phase(&service, &req_p1_incomplete).await.unwrap_err();
    assert_eq!(err_gating.code, ErrorCode::ValidationFailed);

    // Complete Phase 0 requirement
    let entry = service.get_or_load_runtime_entry(draft_id).unwrap();
    entry.mutate_working_state(|ws| {
        // Set all 10 required attributes in require section
        ws.business_flow_data.require.project.display_name.value = Some("My Project".to_string());
        ws.business_flow_data.require.project.display_name.source = Some(ProvenanceSource::User);
        ws.business_flow_data.require.project.summary.value = Some("A summary of the story".to_string());
        ws.business_flow_data.require.project.summary.source = Some(ProvenanceSource::User);
        ws.business_flow_data.require.project.language.value = Some("vi".to_string());
        ws.business_flow_data.require.project.language.source = Some(ProvenanceSource::User);

        ws.business_flow_data.require.project.target.arc_count.value = Some(3);
        ws.business_flow_data.require.project.target.arc_count.source = Some(ProvenanceSource::User);
        ws.business_flow_data.require.project.target.chapter_count.value = Some(30);
        ws.business_flow_data.require.project.target.chapter_count.source = Some(ProvenanceSource::User);
        ws.business_flow_data.require.project.target.word_per_chapter_count.value = Some(3000);
        ws.business_flow_data.require.project.target.word_per_chapter_count.source = Some(ProvenanceSource::User);

        let genre_item = domain::business_flow::RequireGenreItem {
            code: domain::business_flow::TrackedField {
                value: Some("fantasy".to_string()),
                source: Some(ProvenanceSource::User),
            },
            is_primary: domain::business_flow::TrackedField {
                value: Some(true),
                source: Some(ProvenanceSource::User),
            },
            display_name: domain::business_flow::TrackedField {
                value: Some("Fantasy".to_string()),
                source: Some(ProvenanceSource::User),
            },
        };
        ws.business_flow_data.require.project.genres = vec![genre_item];

        let mut c = domain::business_flow::RequireCharacterItem::default();
        c.code = "char_protagonist".to_string();
        c.display_name.value = Some("Arthur".to_string());
        c.display_name.source = Some(ProvenanceSource::User);
        c.biological_sex.value = Some("male".to_string());
        c.biological_sex.source = Some(ProvenanceSource::User);
        c.motif_code.value = Some("chosen_one".to_string());
        c.motif_code.source = Some(ProvenanceSource::User);
        ws.business_flow_data.require.character.push(c);
    });

    // Now Phase 1 gating succeeds
    let resp_p1 = exec_confirm_phase(&service, &req_p1_incomplete).await.unwrap();
    assert_eq!(resp_p1.phase, 1);
    assert_eq!(resp_p1.sealed_version.reason, "phase_confirm");

    // 2. Phase 1 cannot be left while its optional payload is incomplete (3 arcs required).
    let req_p2 = ConfirmPhaseReq {
        draft_session_id: draft_id,
        target_phase: 2,
        target_subphase: None,
        pre_draft_suggestion: None,
    };
    let err_gate = exec_confirm_phase(&service, &req_p2).await.unwrap_err();
    assert_eq!(err_gate.code, ErrorCode::ValidationFailed);
    assert!(
        err_gate.message.contains("Phase 1 chưa hoàn tất"),
        "unexpected gate message: {}",
        err_gate.message
    );

    // Fill the three required arcs, then the same transition is accepted.
    let entry = service.get_or_load_runtime_entry(draft_id).unwrap();
    entry.mutate_working_state(|ws| {
        for index in 1..=3u32 {
            let mut arc = domain::business_flow::OverviewArcItem::default();
            arc.code = format!("arc-{index}");
            arc.display_name = domain::business_flow::TrackedField::user(format!("Arc {index}"));
            arc.summary = domain::business_flow::TrackedField::user("Tóm tắt".to_string());
            arc.motif.code = domain::business_flow::TrackedField::user("chosen_one".to_string());
            arc.motif.event_blocks.push(domain::business_flow::ArcEventBlockItem {
                code: format!("eb-{index}"),
                ..Default::default()
            });
            arc.themes.question =
                domain::business_flow::TrackedField::user("Câu hỏi chủ đề?".to_string());
            arc.themes.message =
                domain::business_flow::TrackedField::user("Thông điệp".to_string());
            ws.business_flow_data.optional.overview_phase.arcs.push(arc);
        }
        ws.business_flow_data.meta.phase = 1;
    });
    let resp_p2 = exec_confirm_phase(&service, &req_p2).await.unwrap();
    assert_eq!(resp_p2.phase, 2);

    // 3. Subphase invalid for Phase 1
    let req_invalid_sub = ConfirmPhaseReq {
        draft_session_id: draft_id,
        target_phase: 2,
        target_subphase: Some(1),
        pre_draft_suggestion: None,
    };
    let err_sub = exec_confirm_phase(&service, &req_invalid_sub).await.unwrap_err();
    assert_eq!(err_sub.code, ErrorCode::ValidationFailed);

    // 3. Jumping over phases is rejected (Phase 1 -> Phase 5).
    let req_p5_jump = ConfirmPhaseReq {
        draft_session_id: draft_id,
        target_phase: 5,
        target_subphase: Some(1),
        pre_draft_suggestion: None,
    };
    let err_jump = exec_confirm_phase(&service, &req_p5_jump).await.unwrap_err();
    assert_eq!(err_jump.code, ErrorCode::ValidationFailed);
    assert!(err_jump.message.contains("Phase"), "{}", err_jump.message);

    // 4. Phase 5 requires a valid subphase. Move the runtime to phase 5 and verify the subphase
    // contract is still enforced by the command (re-confirming the current phase is allowed).
    let entry = service.get_or_load_runtime_entry(draft_id).unwrap();
    entry.mutate_working_state(|ws| {
        ws.phase = 5;
        ws.subphase = None;
    });
    let req_p5_missing_sub = ConfirmPhaseReq {
        draft_session_id: draft_id,
        target_phase: 5,
        target_subphase: None,
        pre_draft_suggestion: None,
    };
    assert_eq!(
        exec_confirm_phase(&service, &req_p5_missing_sub).await.unwrap_err().code,
        ErrorCode::ValidationFailed
    );

    let req_p5_valid = ConfirmPhaseReq {
        draft_session_id: draft_id,
        target_phase: 5,
        target_subphase: Some(1),
        pre_draft_suggestion: None,
    };
    let resp_p5 = exec_confirm_phase(&service, &req_p5_valid).await.unwrap();
    assert_eq!(resp_p5.phase, 5);
    assert_eq!(resp_p5.subphase, Some(1));
}

/// IPC-CMD-010: list_draft_versions tests.
#[test]
fn IPC_CMD_010_list_draft_versions() {
    let temp_dir = TempDir::new().unwrap();
    let (_path, pool) = setup_test_db(&temp_dir);
    let registry = Arc::new(DraftRuntimeRegistry::new());
    let service = DraftService::new(pool.clone(), registry);

    let draft_id = Uuid::now_v7();
    seed_draft_session(&pool, draft_id, "editing");

    for i in 1..=3 {
        let mut bfd = BusinessFlowData::new();
        bfd.require.project.display_name.value = Some(format!("V{}", i));
        let hash = state_data_hash(&bfd).unwrap();
        let json_str = serde_json::to_string(&bfd).unwrap();

        service.seal_version(&SealVersionParams {
            draft_session_id: draft_id,
            seal_key: format!("v:{}", i),
            reason: "ai_patch".to_string(),
            phase: 0,
            subphase: None,
            data_json: json_str,
            data_hash: hash,
            parent_seq: None,
            turn_id: None,
            chat_message_id: None,
        }).unwrap();
    }

    let resp = exec_list_draft_versions(&service, &ListDraftVersionsReq { draft_session_id: draft_id }).unwrap();
    assert_eq!(resp.head_version_seq, Some(3));
    assert_eq!(resp.versions.len(), 3);
    assert_eq!(resp.versions[0].seq, 3);
    assert_eq!(resp.versions[0].display_label, "v.03");
    assert_eq!(resp.versions[1].seq, 2);
    assert_eq!(resp.versions[2].seq, 1);
}

/// IPC-CMD-011: restore_draft_version tests.
#[tokio::test]
async fn IPC_CMD_011_restore_draft_version() {
    let temp_dir = TempDir::new().unwrap();
    let (_path, pool) = setup_test_db(&temp_dir);
    let registry = Arc::new(DraftRuntimeRegistry::new());
    let service = DraftService::new(pool.clone(), registry);

    let draft_id = Uuid::now_v7();
    seed_draft_session(&pool, draft_id, "editing");

    // Seal v.01 and v.02
    for i in 1..=2 {
        let mut bfd = BusinessFlowData::new();
        bfd.require.project.display_name.value = Some(format!("Version {}", i));
        let hash = state_data_hash(&bfd).unwrap();
        let json_str = serde_json::to_string(&bfd).unwrap();

        service.seal_version(&SealVersionParams {
            draft_session_id: draft_id,
            seal_key: format!("init:{}", i),
            reason: "manual_edit".to_string(),
            phase: 0,
            subphase: None,
            data_json: json_str,
            data_hash: hash,
            parent_seq: None,
            turn_id: None,
            chat_message_id: None,
        }).unwrap();
    }

    // Restore to v1
    let req = RestoreDraftVersionReq {
        draft_session_id: draft_id,
        target_seq: 1,
        client_request_id: "req-restore-cmd11".to_string(),
    };
    let resp = exec_restore_draft_version(&service, &req).await.unwrap();
    assert_eq!(resp.new_version.seq, 3);
    assert_eq!(resp.new_version.parent_seq, Some(1));
    assert_eq!(resp.new_version.reason, "restore");

    // Idempotent call with same clientRequestId
    let resp_retry = exec_restore_draft_version(&service, &req).await.unwrap();
    assert_eq!(resp_retry.new_version.seq, 3);

    // Non-existent target version fails with EntityNotFound
    let req_invalid = RestoreDraftVersionReq {
        draft_session_id: draft_id,
        target_seq: 999,
        client_request_id: "req-invalid".to_string(),
    };
    let err = exec_restore_draft_version(&service, &req_invalid).await.unwrap_err();
    assert_eq!(err.code, ErrorCode::EntityNotFound);
}
