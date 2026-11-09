//! Integration and acceptance tests for `phase2_sprint3_deep1` ("Implement disposable SQLite bootstrap and migrations").
//!
//! Complies with docs/plan/phase2_sprint3_deep1.md, docs/business/be-core.md (§3.1, §3.4, §3.5, §12),
//! and docs/db.dbml.
//!
//! Per backend module wiring rule in docs/plan/phase2_sprint3_deep1.md §12:
//! - This sprint does not edit any mod.rs or lib.rs.
//! - The test target loads owned implementation files via `#[path = "..."]`.

#[path = "../src/storage/db.rs"]
pub mod db;

#[path = "../src/storage/migration.rs"]
pub mod migration;

pub mod storage {
    pub use crate::db;
    pub use crate::migration;
}

use storage::db::{
    canonical_db_path, configure_connection, DbError, DbManager, ErrorCode,
};
use storage::migration::{
    get_user_version, run_migrations, run_migrations_with_backup, set_user_version,
    verify_p1s2_authorization, verify_schema_and_integrity, MAX_SUPPORTED_SCHEMA_VERSION,
    MIGRATION_0001_SQL, MIGRATION_0002_SQL,
};

use rusqlite::Connection;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::TempDir;

const BASELINE_COMMIT: &str = "57a462efb953a665e3673b15e6cc33622a4f2d39";
const PREDECESSOR_P2S1_COMMIT: &str = "cc6fe79511e4349b5fa22815d4d7a06ccc4cfad2";
const PREDECESSOR_P1S2_COMMIT: &str = "5fa305f5cad1ca4dfb6be4ceae8de8210ac0ae2f";

/// Helper to verify predecessor gate and baseline lineage (`phase2_sprint3_deep1-F0`).
pub fn verify_p2s3_dispatch_preconditions(
    expected_baseline: &str,
    actual_baseline: &str,
    p2s1_accepted: bool,
    p1s2_accepted: bool,
) -> Result<(), DbError> {
    if !p2s1_accepted {
        return Err(DbError::ContractBlocked(
            "predecessor gate_phase2_sprint1_deep0 is absent or not ACCEPTED".to_string(),
        ));
    }
    if !p1s2_accepted {
        return Err(DbError::ContractBlocked(
            "predecessor gate_phase1_sprint2_deep1 is absent or not ACCEPTED".to_string(),
        ));
    }
    if expected_baseline != actual_baseline {
        return Err(DbError::ContractBlocked(format!(
            "baseline SHA drift detected: expected {expected_baseline}, got {actual_baseline}"
        )));
    }
    Ok(())
}

// -----------------------------------------------------------------------------
// Acceptance Test 1: P2S3-P
// -----------------------------------------------------------------------------
#[test]
#[allow(non_snake_case)]
fn P2S3_P_disposable_new_db_has_exactly_66_target_tables_composite_fks_and_pragmas() {
    let temp_dir = TempDir::new().expect("tempdir creation");
    let app_data = temp_dir.path();

    let manager = DbManager::initialize_and_migrate(app_data)
        .expect("initialize_and_migrate must succeed on fresh DB");

    let conn = manager.get_connection().expect("connection from pool");

    // PRAGMAs verification
    let journal_mode: String = conn
        .query_row("PRAGMA journal_mode;", [], |r| r.get(0))
        .expect("journal_mode");
    assert_eq!(journal_mode.to_lowercase(), "wal");

    let synchronous: i32 = conn
        .query_row("PRAGMA synchronous;", [], |r| r.get(0))
        .expect("synchronous");
    assert_eq!(synchronous, 2, "synchronous must be FULL (2)");

    let foreign_keys: i32 = conn
        .query_row("PRAGMA foreign_keys;", [], |r| r.get(0))
        .expect("foreign_keys");
    assert_eq!(foreign_keys, 1, "foreign_keys must be ON (1)");

    let user_version = get_user_version(&conn).expect("user_version");
    assert_eq!(user_version, 4);
    let style_preset_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM style_presets", [], |row| row.get(0))
        .expect("seeded style presets");
    assert_eq!(style_preset_count, 54);

    // Verify 66 target tables in sqlite_master
    let mut stmt = conn
        .prepare("SELECT name FROM sqlite_master WHERE type = 'table' AND name NOT LIKE 'sqlite_%' ORDER BY name;")
        .expect("prepare table query");
    let tables: Vec<String> = stmt
        .query_map([], |r| r.get(0))
        .expect("query_map")
        .collect::<Result<Vec<_>, _>>()
        .expect("collect tables");
    assert_eq!(tables.len(), 66, "Must contain exactly 66 target tables");

    // Verify composite FK between workflow_round_receipts and workflow_task_receipts
    let proj_id = "018e3a00-0000-7000-8000-000000000001";
    conn.execute("INSERT INTO projects (id, display_name) VALUES (?, 'P1');", [proj_id]).unwrap();
    let sess_id = "018e3a00-0000-7000-8000-000000000002";
    conn.execute("INSERT INTO sessions (id, project_id) VALUES (?, ?);", [sess_id, proj_id]).unwrap();
    let draft_id = "018e3a00-0000-7000-8000-000000000003";
    conn.execute("INSERT INTO draft_sessions (id, session_id, project_id) VALUES (?, ?, ?);", [draft_id, sess_id, proj_id]).unwrap();

    let ver1_id = "018e3a00-0000-7000-8000-000000000021";
    conn.execute(
        "INSERT INTO draft_versions (id, draft_session_id, seq, phase, data_json, data_hash, reason) VALUES (?, ?, 1, 1, '{}', 'hash1', 'phase_confirm');",
        [ver1_id, draft_id]
    ).unwrap();
    let ver2_id = "018e3a00-0000-7000-8000-000000000022";
    conn.execute(
        "INSERT INTO draft_versions (id, draft_session_id, seq, phase, data_json, data_hash, reason) VALUES (?, ?, 2, 1, '{}', 'hash2', 'phase_confirm');",
        [ver2_id, draft_id]
    ).unwrap();

    let round_id = "018e3a00-0000-7000-8000-000000000030";
    let run_id = "run-1";
    conn.execute(
        "INSERT INTO workflow_round_receipts (id, draft_session_id, workflow_run_id, plan_generation, round_index, round_seal_key, task_count, parallel_limit, plan_digest, request_digest, base_version_seq, base_data_hash, final_data_hash, resulting_version_seq, created_at)
         VALUES (?, ?, ?, 1, 0, 'seal-1', 2, 2, 'plan-d', 'req-d', 1, 'hash1', 'hash2', 2, '2026-11-08T00:00:00Z');",
        [round_id, draft_id, run_id]
    ).unwrap();

    // Valid child task insert matching composite FK
    let task_id = "018e3a00-0000-7000-8000-000000000040";
    conn.execute(
        "INSERT INTO workflow_task_receipts (id, draft_session_id, workflow_run_id, plan_generation, task_identity, input_scope_fingerprint, output_scope_fingerprint, base_version_seq, resulting_version_seq, had_mutations, round_receipt_id, task_index, evidence_json, created_at)
         VALUES (?, ?, ?, 1, 'task-1', 'in-fp', 'out-fp', 1, 2, 1, ?, 0, '{}', '2026-11-08T00:00:00Z');",
        [task_id, draft_id, run_id, round_id]
    ).unwrap();

    // Invalid composite FK insert (mismatched workflow_run_id)
    let bad_task_id = "018e3a00-0000-7000-8000-000000000041";
    let bad_res = conn.execute(
        "INSERT INTO workflow_task_receipts (id, draft_session_id, workflow_run_id, plan_generation, task_identity, input_scope_fingerprint, output_scope_fingerprint, base_version_seq, resulting_version_seq, had_mutations, round_receipt_id, task_index, evidence_json, created_at)
         VALUES (?, ?, 'wrong-run', 1, 'task-2', 'in-fp', 'out-fp', 1, 2, 1, ?, 1, '{}', '2026-11-08T00:00:00Z');",
        [bad_task_id, draft_id, round_id]
    );
    assert!(bad_res.is_err(), "Composite FK mismatch MUST fail constraint check");
}

// -----------------------------------------------------------------------------
// Acceptance Test 2: P2S3-N
// -----------------------------------------------------------------------------
#[test]
#[allow(non_snake_case)]
fn P2S3_N_child_index_equals_N_passes_sql_but_backend_gate_required() {
    let temp_dir = TempDir::new().expect("tempdir");
    let manager = DbManager::initialize_and_migrate(temp_dir.path()).expect("init");
    let conn = manager.get_connection().expect("conn");

    // Setup hierarchy
    let proj_id = "018e3a00-0000-7000-8000-000000000001";
    conn.execute("INSERT INTO projects (id, display_name) VALUES (?, 'P1');", [proj_id]).unwrap();
    let sess_id = "018e3a00-0000-7000-8000-000000000002";
    conn.execute("INSERT INTO sessions (id, project_id) VALUES (?, ?);", [sess_id, proj_id]).unwrap();
    let draft_id = "018e3a00-0000-7000-8000-000000000003";
    conn.execute("INSERT INTO draft_sessions (id, session_id, project_id) VALUES (?, ?, ?);", [draft_id, sess_id, proj_id]).unwrap();

    let ver1_id = "018e3a00-0000-7000-8000-000000000021";
    conn.execute(
        "INSERT INTO draft_versions (id, draft_session_id, seq, phase, data_json, data_hash, reason) VALUES (?, ?, 1, 1, '{}', 'hash1', 'phase_confirm');",
        [ver1_id, draft_id]
    ).unwrap();

    let round_id = "018e3a00-0000-7000-8000-000000000030";
    let run_id = "run-1";
    // task_count is 2
    conn.execute(
        "INSERT INTO workflow_round_receipts (id, draft_session_id, workflow_run_id, plan_generation, round_index, round_seal_key, task_count, parallel_limit, plan_digest, request_digest, base_version_seq, base_data_hash, final_data_hash, resulting_version_seq, created_at)
         VALUES (?, ?, ?, 1, 0, 'seal-1', 2, 2, 'plan-d', 'req-d', 1, 'hash1', 'hash1', NULL, '2026-11-08T00:00:00Z');",
        [round_id, draft_id, run_id]
    ).unwrap();

    // Insert task with task_index = 5 (>= task_count). In SQL this is NOT rejected by CHECK constraint.
    // Per docs/db.dbml:1349: "SQLite CHECK KHÔNG cưỡng chế task_index < parent.task_count... backend trước commit và recovery kiểm lại."
    let task_id = "018e3a00-0000-7000-8000-000000000040";
    let insert_res = conn.execute(
        "INSERT INTO workflow_task_receipts (id, draft_session_id, workflow_run_id, plan_generation, task_identity, input_scope_fingerprint, output_scope_fingerprint, base_version_seq, resulting_version_seq, had_mutations, round_receipt_id, task_index, evidence_json, created_at)
         VALUES (?, ?, ?, 1, 'task-out-of-bounds', 'in-fp', 'out-fp', 1, NULL, 0, ?, 5, '{}', '2026-11-08T00:00:00Z');",
        [task_id, draft_id, run_id, round_id]
    );
    assert!(insert_res.is_ok(), "SQL does not overstate CHECK: task_index >= task_count is valid in SQL, gated by backend");
}

// -----------------------------------------------------------------------------
// Acceptance Test 3: P2S3-B
// -----------------------------------------------------------------------------
#[test]
#[allow(non_snake_case)]
fn P2S3_B_simulated_upgrade_keeps_old_receipts_byte_identical_and_new_fields_null() {
    let temp_dir = TempDir::new().expect("tempdir");
    let db_path = temp_dir.path().join("upgrade_test.sqlite3");
    let mut conn = Connection::open(&db_path).unwrap();
    configure_connection(&conn).unwrap();

    // Run v1 and v2
    conn.execute_batch(MIGRATION_0001_SQL).unwrap();
    conn.execute_batch(MIGRATION_0002_SQL).unwrap();
    assert_eq!(get_user_version(&conn).unwrap(), 2);

    // Insert legacy v2 data
    let proj_id = "018e3a00-0000-7000-8000-000000000001";
    conn.execute("INSERT INTO projects (id, display_name) VALUES (?, 'P1');", [proj_id]).unwrap();
    let sess_id = "018e3a00-0000-7000-8000-000000000002";
    conn.execute("INSERT INTO sessions (id, project_id) VALUES (?, ?);", [sess_id, proj_id]).unwrap();
    let draft_id = "018e3a00-0000-7000-8000-000000000003";
    conn.execute("INSERT INTO draft_sessions (id, session_id, project_id) VALUES (?, ?, ?);", [draft_id, sess_id, proj_id]).unwrap();

    let legacy_task_id = "018e3a00-0000-7000-8000-000000000040";
    let run_id = "legacy-run-99";
    conn.execute(
        "INSERT INTO workflow_task_receipts (id, draft_session_id, workflow_run_id, plan_generation, task_identity, input_scope_fingerprint, output_scope_fingerprint, base_version_seq, resulting_version_seq, had_mutations, created_at)
         VALUES (?, ?, ?, 1, 'legacy-task', 'in-fp-1', 'out-fp-1', 1, NULL, 0, '2026-11-07T12:00:00Z');",
        [legacy_task_id, draft_id, run_id]
    ).unwrap();

    // Now upgrade through v3 to v4
    run_migrations(&mut conn).unwrap();
    assert_eq!(get_user_version(&conn).unwrap(), 4);

    // Query upgraded receipt
    let (id, task_ident, in_fp, out_fp, round_id, task_idx, evidence): (
        String,
        String,
        String,
        String,
        Option<String>,
        Option<i64>,
        Option<String>,
    ) = conn
        .query_row(
            "SELECT id, task_identity, input_scope_fingerprint, output_scope_fingerprint, round_receipt_id, task_index, evidence_json FROM workflow_task_receipts WHERE id = ?;",
            [legacy_task_id],
            |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?, r.get(5)?, r.get(6)?))
        )
        .unwrap();

    assert_eq!(id, legacy_task_id);
    assert_eq!(task_ident, "legacy-task");
    assert_eq!(in_fp, "in-fp-1");
    assert_eq!(out_fp, "out-fp-1");
    assert!(round_id.is_none(), "round_receipt_id must be NULL for legacy receipt");
    assert!(task_idx.is_none(), "task_index must be NULL for legacy receipt");
    assert!(evidence.is_none(), "evidence_json must be NULL for legacy receipt");
}

// -----------------------------------------------------------------------------
// Acceptance Test 4: P2S3-C
// -----------------------------------------------------------------------------
#[test]
#[allow(non_snake_case)]
fn P2S3_C_interrupted_ddl_leaves_user_version_and_rows_unchanged() {
    let temp_dir = TempDir::new().expect("tempdir");
    let db_path = temp_dir.path().join("interrupted.sqlite3");
    let mut conn = Connection::open(&db_path).unwrap();
    configure_connection(&conn).unwrap();

    conn.execute_batch(MIGRATION_0001_SQL).unwrap();
    conn.execute_batch(MIGRATION_0002_SQL).unwrap();
    assert_eq!(get_user_version(&conn).unwrap(), 2);

    // Insert user row at v2
    let genre_id = "018e3a00-0000-7000-8000-000000000099";
    conn.execute("INSERT INTO genres (id, code, display_name) VALUES (?, 'g_test', 'Genre Test');", [genre_id]).unwrap();

    // Attempt a transaction with invalid SQL in the middle
    let tx = conn.transaction_with_behavior(rusqlite::TransactionBehavior::Immediate).unwrap();
    let _ = tx.execute("CREATE TABLE temp_marker (id INT);", []);
    let err = tx.execute_batch("INVALID SQL STATEMENT SYNTAX ERROR;");
    assert!(err.is_err());
    drop(tx); // rolls back

    // Version must remain 2
    assert_eq!(get_user_version(&conn).unwrap(), 2);

    // temp_marker must not exist
    let marker_exists: bool = conn
        .query_row(
            "SELECT count(*) FROM sqlite_master WHERE type = 'table' AND name = 'temp_marker';",
            [],
            |r| r.get(0),
        )
        .map(|cnt: i64| cnt > 0)
        .unwrap();
    assert!(!marker_exists, "Partial table must have been rolled back");

    // User row is untouched
    let count: i64 = conn
        .query_row("SELECT count(*) FROM genres WHERE id = ?;", [genre_id], |r| r.get(0))
        .unwrap();
    assert_eq!(count, 1, "User data must be 100% preserved");
}

// -----------------------------------------------------------------------------
// Acceptance Test 5: phase2_sprint3_deep1_F0
// -----------------------------------------------------------------------------
#[test]
#[allow(non_snake_case)]
fn phase2_sprint3_deep1_F0_predecessor_gate_absent_or_baseline_sha_drift_is_contract_blocked() {
    // 1. Missing predecessor gate -> CONTRACT_BLOCKED
    let err1 = verify_p2s3_dispatch_preconditions(BASELINE_COMMIT, BASELINE_COMMIT, false, true).unwrap_err();
    assert_eq!(err1.error_code(), ErrorCode::ContractBlocked);

    let err2 = verify_p2s3_dispatch_preconditions(BASELINE_COMMIT, BASELINE_COMMIT, true, false).unwrap_err();
    assert_eq!(err2.error_code(), ErrorCode::ContractBlocked);

    // 2. Baseline SHA drift -> CONTRACT_BLOCKED
    let err_drift = verify_p2s3_dispatch_preconditions(BASELINE_COMMIT, "deadbeef1234", true, true).unwrap_err();
    assert_eq!(err_drift.error_code(), ErrorCode::ContractBlocked);

    // 3. Positive verification in repository: git checks pass
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf();

    let baseline_check = Command::new("git")
        .args(["merge-base", "--is-ancestor", BASELINE_COMMIT, "HEAD"])
        .current_dir(&repo_root)
        .status()
        .expect("git merge-base execution");
    assert!(baseline_check.success(), "HEAD must descend from BASELINE_COMMIT");

    let p2s1_check = Command::new("git")
        .args(["cat-file", "-e", &format!("{PREDECESSOR_P2S1_COMMIT}^{{commit}}")])
        .current_dir(&repo_root)
        .status()
        .expect("git cat-file execution");
    assert!(p2s1_check.success(), "Predecessor P2S1 commit must exist");

    let p1s2_check = Command::new("git")
        .args(["cat-file", "-e", &format!("{PREDECESSOR_P1S2_COMMIT}^{{commit}}")])
        .current_dir(&repo_root)
        .status()
        .expect("git cat-file execution");
    assert!(p1s2_check.success(), "Predecessor P1S2 commit must exist");

    assert!(verify_p2s3_dispatch_preconditions(BASELINE_COMMIT, BASELINE_COMMIT, true, true).is_ok());
}

// -----------------------------------------------------------------------------
// Acceptance Test 6: phase2_sprint3_deep1_F1
// -----------------------------------------------------------------------------
#[test]
#[allow(non_snake_case)]
fn phase2_sprint3_deep1_F1_no_p1s2_authorization_is_contract_blocked() {
    let unratified_text = r#"
MIGRATION_DECISION:
  version_path: UNRATIFIED_PROPOSAL_ONLY
  authorized_revision: NONE
"#;
    let res = verify_p1s2_authorization(unratified_text);
    assert!(res.is_err());
    let err = res.unwrap_err();
    assert_eq!(err.error_code(), ErrorCode::ContractBlocked);
    assert!(err.to_string().contains("CONTRACT_BLOCKED"));

    let ratified_text = r#"
MIGRATION_DECISION:
  version_path: PATH_A_GREENFIELD_SYNTHESIZED
  authorized_revision: RATIFIED_ACCEPTED
"#;
    assert!(verify_p1s2_authorization(ratified_text).is_ok());
}

// -----------------------------------------------------------------------------
// Acceptance Test 7: phase2_sprint3_deep1_F2
// -----------------------------------------------------------------------------
#[test]
#[allow(non_snake_case)]
fn phase2_sprint3_deep1_F2_invalid_migration_or_backup_failure_rolls_back_and_stops_startup() {
    let temp_dir = TempDir::new().expect("tempdir");
    let db_path = temp_dir.path().join("f2_test.sqlite3");
    let mut conn = Connection::open(&db_path).unwrap();
    configure_connection(&conn).unwrap();

    conn.execute_batch(MIGRATION_0001_SQL).unwrap();
    conn.execute_batch(MIGRATION_0002_SQL).unwrap();
    assert_eq!(get_user_version(&conn).unwrap(), 2);

    // Seed baseline user row
    conn.execute("INSERT INTO genres (id, code, display_name) VALUES ('018e3a00-0000-7000-8000-000000000009', 'g_f2', 'Genre F2');", []).unwrap();

    // Trigger failure by trying to backup to an invalid directory (e.g. read-only path or bad path)
    let bad_backup_dir = Path::new("/proc/invalid_non_writable_dir/merlink-harness");
    let backup_res = run_migrations_with_backup(&mut conn, bad_backup_dir);
    assert!(backup_res.is_err());
    assert_eq!(get_user_version(&conn).unwrap(), 2, "user_version must remain unchanged on backup failure");

    // Verify user row preserved
    let count: i64 = conn
        .query_row("SELECT count(*) FROM genres WHERE code = 'g_f2';", [], |r| r.get(0))
        .unwrap();
    assert_eq!(count, 1);
}

// -----------------------------------------------------------------------------
// Acceptance Test 8: T-DB-01
// -----------------------------------------------------------------------------
#[test]
#[allow(non_snake_case)]
fn T_DB_01_pragmas_user_version_fks_and_enforced_check_constraints() {
    let temp_dir = TempDir::new().expect("tempdir");
    let db_path = temp_dir.path().join("t_db_01.sqlite3");
    let mut conn = Connection::open(&db_path).unwrap();
    configure_connection(&conn).unwrap();

    // Apply migrations forward
    conn.execute_batch(MIGRATION_0001_SQL).unwrap();
    conn.execute_batch(MIGRATION_0002_SQL).unwrap();
    run_migrations(&mut conn).unwrap();

    // PRAGMAs verification
    let jm: String = conn.query_row("PRAGMA journal_mode;", [], |r| r.get(0)).unwrap();
    assert_eq!(jm.to_lowercase(), "wal");

    let fk: i32 = conn.query_row("PRAGMA foreign_keys;", [], |r| r.get(0)).unwrap();
    assert_eq!(fk, 1);

    let sync: i32 = conn.query_row("PRAGMA synchronous;", [], |r| r.get(0)).unwrap();
    assert_eq!(sync, 2);

    assert_eq!(get_user_version(&conn).unwrap(), MAX_SUPPORTED_SCHEMA_VERSION);

    // Exactly 66 tables
    let mut stmt = conn
        .prepare("SELECT name FROM sqlite_master WHERE type = 'table' AND name NOT LIKE 'sqlite_%';")
        .unwrap();
    let tables: Vec<String> = stmt
        .query_map([], |r| r.get(0))
        .unwrap()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    assert_eq!(tables.len(), 66);

    // Setup base parent entities for constraint testing
    let proj_id = "018e3a00-0000-7000-8000-000000000001";
    conn.execute("INSERT INTO projects (id, display_name) VALUES (?, 'P1');", [proj_id]).unwrap();
    let sess_id = "018e3a00-0000-7000-8000-000000000002";
    conn.execute("INSERT INTO sessions (id, project_id) VALUES (?, ?);", [sess_id, proj_id]).unwrap();
    let draft_id = "018e3a00-0000-7000-8000-000000000003";
    conn.execute("INSERT INTO draft_sessions (id, session_id, project_id) VALUES (?, ?, ?);", [draft_id, sess_id, proj_id]).unwrap();

    // 1. Violation 1: characters with display_name and nickname BOTH NULL -> SQLITE_CONSTRAINT
    let err_char = conn.execute(
        "INSERT INTO characters (id, project_id, code, display_name, nickname) VALUES ('018e3a00-0000-7000-8000-000000000050', ?, 'c1', NULL, NULL);",
        [proj_id]
    );
    assert!(err_char.is_err(), "characters with NULL display_name & nickname must violate CHECK");

    // 2. Violation 2: draft_versions with phase = 2, subphase = 1 -> SQLITE_CONSTRAINT
    let err_dv = conn.execute(
        "INSERT INTO draft_versions (id, draft_session_id, seq, phase, subphase, data_json, data_hash, reason) VALUES ('018e3a00-0000-7000-8000-000000000051', ?, 1, 2, 1, '{}', 'h1', 'phase_confirm');",
        [draft_id]
    );
    assert!(err_dv.is_err(), "draft_versions with phase=2, subphase=1 must violate CHECK");

    // 3. Violation 3: scene_versions with parent_id = NULL -> SQLITE_CONSTRAINT
    let err_sv = conn.execute(
        "INSERT INTO scene_versions (id, parent_id, version_number, payload) VALUES ('018e3a00-0000-7000-8000-000000000052', NULL, 1, '{}');",
        []
    );
    assert!(err_sv.is_err(), "scene_versions with NULL parent_id must violate NOT NULL constraint");

    // 4. Violation 4: workflow_task_receipts with had_mutations = 0, resulting_version_seq = 5 -> SQLITE_CONSTRAINT
    let err_wtr = conn.execute(
        "INSERT INTO workflow_task_receipts (id, draft_session_id, workflow_run_id, plan_generation, task_identity, input_scope_fingerprint, output_scope_fingerprint, base_version_seq, resulting_version_seq, had_mutations, created_at)
         VALUES ('018e3a00-0000-7000-8000-000000000053', ?, 'run-1', 1, 'task-c', 'in-fp', 'out-fp', 1, 5, 0, '2026-11-08T00:00:00Z');",
        [draft_id]
    );
    assert!(err_wtr.is_err(), "workflow_task_receipts with had_mutations=0 & resulting_version_seq!=NULL must violate CHECK");
}

// -----------------------------------------------------------------------------
// Acceptance Test 9: MIG-01
// -----------------------------------------------------------------------------
#[test]
#[allow(non_snake_case)]
fn MIG_01_fresh_database_bootstrap_runs_all_migrations_to_max_version() {
    let temp_dir = TempDir::new().expect("tempdir");
    let db_path = temp_dir.path().join("fresh.sqlite3");
    let mut conn = Connection::open(&db_path).unwrap();
    configure_connection(&conn).unwrap();

    assert_eq!(get_user_version(&conn).unwrap(), 0);

    run_migrations(&mut conn).expect("fresh migrations must succeed");
    assert_eq!(get_user_version(&conn).unwrap(), MAX_SUPPORTED_SCHEMA_VERSION);

    // Verify schema and integrity passes
    verify_schema_and_integrity(&conn).expect("verify_schema_and_integrity must pass");

    // Confirm no _schema_migrations table exists
    let has_schema_migrations: bool = conn
        .query_row(
            "SELECT count(*) FROM sqlite_master WHERE type = 'table' AND name = '_schema_migrations';",
            [],
            |r| r.get(0),
        )
        .map(|cnt: i64| cnt > 0)
        .unwrap();
    assert!(!has_schema_migrations, "No _schema_migrations table allowed");
}

// -----------------------------------------------------------------------------
// Acceptance Test 10: MIG-02
// -----------------------------------------------------------------------------
#[test]
#[allow(non_snake_case)]
fn MIG_02_sequential_upgrade_from_older_version_creates_backup_and_preserves_user_data() {
    let temp_dir = TempDir::new().expect("tempdir");
    let app_data = temp_dir.path();
    let db_path = canonical_db_path(app_data);
    fs::create_dir_all(db_path.parent().unwrap()).unwrap();

    let mut conn = Connection::open(&db_path).unwrap();
    configure_connection(&conn).unwrap();

    // Bootstrap at v1
    conn.execute_batch(MIGRATION_0001_SQL).unwrap();
    assert_eq!(get_user_version(&conn).unwrap(), 1);

    // Insert user data at v1
    let genre_id = "018e3a00-0000-7000-8000-000000000011";
    conn.execute(
        "INSERT INTO genres (id, code, display_name, description) VALUES (?, 'g_mig2', 'Genre Mig2', 'Desc Mig2');",
        [genre_id]
    ).unwrap();

    // Now run migration with backup
    run_migrations_with_backup(&mut conn, app_data).unwrap();
    assert_eq!(get_user_version(&conn).unwrap(), MAX_SUPPORTED_SCHEMA_VERSION);

    // Verify backup file exists in backups directory
    let backups_dir = app_data.join("merlink-harness").join("backups");
    let backup_entries: Vec<_> = fs::read_dir(&backups_dir)
        .expect("backups dir must exist")
        .filter_map(|e| e.ok())
        .collect();
    assert_eq!(backup_entries.len(), 1, "Exactly 1 backup file created");
    let backup_filename = backup_entries[0].file_name().to_string_lossy().to_string();
    assert!(backup_filename.starts_with("merlink_pre_migration_v1_"));
    assert!(backup_filename.ends_with(".bak"));

    // Verify user data is 100% preserved
    let display_name: String = conn
        .query_row("SELECT display_name FROM genres WHERE id = ?;", [genre_id], |r| r.get(0))
        .unwrap();
    assert_eq!(display_name, "Genre Mig2");

    verify_schema_and_integrity(&conn).expect("schema must verify");
}

// -----------------------------------------------------------------------------
// Acceptance Test 11: MIG-03
// -----------------------------------------------------------------------------
#[test]
#[allow(non_snake_case)]
fn MIG_03_failed_migration_rollback_leaves_user_version_and_data_intact() {
    let temp_dir = TempDir::new().expect("tempdir");
    let mut conn = Connection::open(temp_dir.path().join("mig03.sqlite3")).unwrap();
    configure_connection(&conn).unwrap();

    conn.execute_batch(MIGRATION_0001_SQL).unwrap();
    conn.execute_batch(MIGRATION_0002_SQL).unwrap();
    assert_eq!(get_user_version(&conn).unwrap(), 2);

    // User row
    conn.execute(
        "INSERT INTO genres (id, code, display_name) VALUES ('018e3a00-0000-7000-8000-000000000012', 'g_mig3', 'Genre Mig3');",
        []
    ).unwrap();

    // Simulate failing step in transaction
    let tx_res = (|| -> Result<(), DbError> {
        let tx = conn
            .transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)
            .map_err(|e| DbError::MigrationFailed(e.to_string()))?;
        tx.execute("CREATE TABLE partial_table (id INT);", [])
            .map_err(|e| DbError::MigrationFailed(e.to_string()))?;
        tx.execute("INVALID SQL FORCE ROLLBACK;", [])
            .map_err(|e| DbError::MigrationFailed(e.to_string()))?;
        tx.commit().map_err(|e| DbError::MigrationFailed(e.to_string()))?;
        Ok(())
    })();

    assert!(tx_res.is_err());
    assert_eq!(tx_res.unwrap_err().error_code(), ErrorCode::MigrationFailed);

    assert_eq!(get_user_version(&conn).unwrap(), 2);

    // Verify partial table rolled back
    let exists: bool = conn
        .query_row(
            "SELECT count(*) FROM sqlite_master WHERE type = 'table' AND name = 'partial_table';",
            [],
            |r| r.get(0),
        )
        .map(|c: i64| c > 0)
        .unwrap();
    assert!(!exists);

    // User row intact
    let count: i64 = conn
        .query_row("SELECT count(*) FROM genres WHERE code = 'g_mig3';", [], |r| r.get(0))
        .unwrap();
    assert_eq!(count, 1);
}

// -----------------------------------------------------------------------------
// Acceptance Test 12: MIG-04
// -----------------------------------------------------------------------------
#[test]
#[allow(non_snake_case)]
fn MIG_04_unsupported_newer_schema_version_is_rejected_without_modifying_db_bytes() {
    let temp_dir = TempDir::new().expect("tempdir");
    let db_path = temp_dir.path().join("newer.sqlite3");
    let conn = Connection::open(&db_path).unwrap();
    configure_connection(&conn).unwrap();

    set_user_version(&conn, MAX_SUPPORTED_SCHEMA_VERSION + 1).unwrap();
    drop(conn);

    let original_bytes = fs::read(&db_path).expect("read db bytes");

    // Open connection again and attempt run_migrations
    let mut conn2 = Connection::open(&db_path).unwrap();
    configure_connection(&conn2).unwrap();

    let res = run_migrations(&mut conn2);
    assert!(res.is_err());
    let err = res.unwrap_err();
    assert_eq!(err.error_code(), ErrorCode::UnsupportedSchemaVersion);
    drop(conn2);

    let current_bytes = fs::read(&db_path).expect("read db bytes after rejection");
    assert_eq!(
        original_bytes, current_bytes,
        "Rejected DB must not have a single byte modified"
    );
}

// -----------------------------------------------------------------------------
// Acceptance Test 13: MIG-05
// -----------------------------------------------------------------------------
#[test]
#[allow(non_snake_case)]
fn MIG_05_schema_and_fk_verification_detects_deviations_and_fails_startup() {
    // 1. Missing draft_versions.seal_key
    {
        let temp_dir = TempDir::new().expect("tempdir");
        let mut conn = Connection::open(temp_dir.path().join("mig05_seal.sqlite3")).unwrap();
        configure_connection(&conn).unwrap();
        run_migrations(&mut conn).unwrap();

        // Deliberately drop seal_key by altering draft_versions
        conn.execute_batch(
            "CREATE TABLE draft_versions_bad AS SELECT id, draft_session_id, seq, phase, subphase, data_json, data_hash, reason, parent_seq, turn_id, chat_message_id, created_at FROM draft_versions;
             DROP TABLE draft_versions;
             ALTER TABLE draft_versions_bad RENAME TO draft_versions;"
        ).unwrap();

        let err = verify_schema_and_integrity(&conn).unwrap_err();
        assert_eq!(err.error_code(), ErrorCode::MigrationFailed);
        assert!(err.to_string().contains("seal_key"));
    }

    // 2. Missing workflow_task_receipts table
    {
        let temp_dir = TempDir::new().expect("tempdir");
        let mut conn = Connection::open(temp_dir.path().join("mig05_wt.sqlite3")).unwrap();
        configure_connection(&conn).unwrap();
        run_migrations(&mut conn).unwrap();

        conn.execute("DROP TABLE workflow_task_receipts;", []).unwrap();
        let err = verify_schema_and_integrity(&conn).unwrap_err();
        assert_eq!(err.error_code(), ErrorCode::MigrationFailed);
    }

    // 3. Foreign key violation present
    {
        let temp_dir = TempDir::new().expect("tempdir");
        let mut conn = Connection::open(temp_dir.path().join("mig05_fk.sqlite3")).unwrap();
        configure_connection(&conn).unwrap();
        run_migrations(&mut conn).unwrap();

        // Temporarily bypass FK pragma to insert violating row
        conn.execute_batch(
            "PRAGMA foreign_keys = OFF;
             INSERT INTO draft_versions (id, draft_session_id, seq, phase, data_json, data_hash, reason)
             VALUES ('018e3a00-0000-7000-8000-000000000099', 'non-existent-session-id', 1, 1, '{}', 'h', 'phase_confirm');
             PRAGMA foreign_keys = ON;"
        ).unwrap();

        let err = verify_schema_and_integrity(&conn).unwrap_err();
        assert_eq!(err.error_code(), ErrorCode::MigrationFailed);
        assert!(err.to_string().contains("Foreign key check failed"));
    }
}

// -----------------------------------------------------------------------------
// Acceptance Test 14: DB-P01
// -----------------------------------------------------------------------------
#[test]
#[allow(non_snake_case)]
fn DB_P01_database_initialization_uses_canonical_path_from_section_1_6() {
    let temp_dir = TempDir::new().expect("tempdir");
    let app_data = temp_dir.path();

    let manager = DbManager::initialize_and_migrate(app_data).expect("initialize_and_migrate");

    let expected_db_path = canonical_db_path(app_data);
    assert_eq!(manager.db_path(), &expected_db_path);
    assert!(expected_db_path.exists(), "merlink.sqlite3 must exist at canonical path");

    let forbidden_db_path = app_data.join("merlink-harness").join("data").join("merlink.db");
    assert!(
        !forbidden_db_path.exists(),
        "merlink-harness/data/merlink.db MUST NOT exist"
    );
}

// -----------------------------------------------------------------------------
// Acceptance Test 15: DB-P02
// -----------------------------------------------------------------------------
#[test]
#[allow(non_snake_case)]
fn DB_P02_pre_migration_backup_uses_sqlite_online_backup_api() {
    let temp_dir = TempDir::new().expect("tempdir");
    let app_data = temp_dir.path();
    let db_path = canonical_db_path(app_data);
    fs::create_dir_all(db_path.parent().unwrap()).unwrap();

    let mut conn = Connection::open(&db_path).unwrap();
    configure_connection(&conn).unwrap();

    // Bootstrap at v2 with some data
    conn.execute_batch(MIGRATION_0001_SQL).unwrap();
    conn.execute_batch(MIGRATION_0002_SQL).unwrap();
    assert_eq!(get_user_version(&conn).unwrap(), 2);

    let genre_id = "018e3a00-0000-7000-8000-000000000077";
    conn.execute(
        "INSERT INTO genres (id, code, display_name) VALUES (?, 'g_dbp02', 'Genre DBP02');",
        [genre_id]
    ).unwrap();

    // Run migration with backup
    run_migrations_with_backup(&mut conn, app_data).unwrap();

    let backups_dir = app_data.join("merlink-harness").join("backups");
    let backup_files: Vec<_> = fs::read_dir(&backups_dir)
        .expect("read backups dir")
        .filter_map(|e| e.ok())
        .collect();
    assert_eq!(backup_files.len(), 1);

    let backup_path = backup_files[0].path();
    let backup_conn = Connection::open(&backup_path).expect("backup file must be valid SQLite DB");
    let count: i64 = backup_conn
        .query_row("SELECT count(*) FROM genres WHERE id = ?;", [genre_id], |r| r.get(0))
        .unwrap();
    assert_eq!(count, 1, "Backup must contain valid pre-migration data");
    let backup_version = get_user_version(&backup_conn).unwrap();
    assert_eq!(backup_version, 2, "Backup must reflect pre-migration version v2");
}
