//! Acceptance and integration tests for `phase2_sprint6_deep2` ("Implement isolated base repositories").
//!
//! Complies with docs/plan/phase2_sprint6_deep2.md, docs/business/be-core.md (§3.1, §3.6, §3.8, §3.9, §1549),
//! and docs/db.dbml.
//!
//! Per backend module wiring rule in docs/plan/phase2_sprint6_deep2.md §12:
//! - This sprint does not edit any mod.rs or lib.rs.
//! - The test target loads owned implementation files via `#[path = "..."]`.

#[path = "../src/storage/db.rs"]
pub mod db;

#[path = "../src/storage/migration.rs"]
pub mod migration;

#[path = "../src/storage/draft_repo.rs"]
pub mod draft_repo;

#[path = "../src/storage/catalog_repo.rs"]
pub mod catalog_repo;

#[path = "../src/storage/workflow_repo.rs"]
pub mod workflow_repo;

#[path = "../src/storage/temp_store.rs"]
pub mod temp_store;

pub mod storage {
    pub use crate::catalog_repo;
    pub use crate::db;
    pub use crate::draft_repo;
    pub use crate::migration;
    pub use crate::temp_store;
    pub use crate::workflow_repo;
}

use rusqlite::Connection;
use std::path::PathBuf;
use std::process::Command;
use tempfile::TempDir;

use storage::catalog_repo::{CatalogRepo, CatalogRepoError, SqliteCatalogRepo};
use storage::db::{configure_connection, DbError, ErrorCode};
use storage::draft_repo::{
    DraftRepo, DraftRepoError, DraftSessionStatus, PendingPatchStatus, SqliteDraftRepo,
};
use storage::migration::{run_migrations, verify_schema_and_integrity};
use storage::temp_store::{TempStore, TempStoreError};
use storage::workflow_repo::{
    SqliteWorkflowRepo, WorkflowReadRepo, WorkflowRepoError,
};

const BASELINE_COMMIT: &str = "57a462efb953a665e3673b15e6cc33622a4f2d39";
const PREDECESSOR_P2S2_COMMIT: &str = "f69f1ab0e3d3d34853ddf887cac139cde6d64b0d";
const PREDECESSOR_P2S3_COMMIT: &str = "d51da679c193f9ef7697bdae4807490472396746";

/// Helper to verify predecessor gate and baseline lineage (`phase2_sprint6_deep2-F0`).
pub fn verify_p2s6_dispatch_preconditions(
    expected_baseline: &str,
    actual_baseline: &str,
    p2s2_accepted: bool,
    p2s3_accepted: bool,
) -> Result<(), DbError> {
    if !p2s2_accepted {
        return Err(DbError::ContractBlocked(
            "predecessor gate_phase2_sprint2_deep1 is absent or not ACCEPTED".to_string(),
        ));
    }
    if !p2s3_accepted {
        return Err(DbError::ContractBlocked(
            "predecessor gate_phase2_sprint3_deep1 is absent or not ACCEPTED".to_string(),
        ));
    }
    if expected_baseline != actual_baseline {
        return Err(DbError::ContractBlocked(format!(
            "baseline SHA drift detected: expected {expected_baseline}, got {actual_baseline}"
        )));
    }
    Ok(())
}

/// Helper to initialize a disposable test SQLite database with migrations 0001, 0002, 0003 applied.
fn setup_disposable_db(temp_dir: &TempDir) -> (PathBuf, Connection) {
    let db_path = temp_dir.path().join("test_merlink.sqlite3");
    let mut conn = Connection::open(&db_path).expect("open disposable sqlite");
    configure_connection(&conn).expect("configure PRAGMAs");
    run_migrations(&mut conn).expect("run migrations 0001..0003");
    verify_schema_and_integrity(&conn).expect("verify schema and integrity");
    (db_path, conn)
}

/// Seed fixture data for testing isolation across draft sessions.
fn seed_test_data(conn: &Connection) {
    // 1. Projects and Sessions
    conn.execute(
        "INSERT INTO projects (id, code, display_name, status, language) VALUES
         ('018e3a00-0000-7000-8000-000000000010', 'proj_alpha', 'Project Alpha', 'PLANNING', 'VI'),
         ('018e3a00-0000-7000-8000-000000000020', 'proj_beta', 'Project Beta', 'DRAFTING', 'EN')",
        [],
    ).expect("seed projects");

    conn.execute(
        "INSERT INTO sessions (id, project_id, display_name) VALUES
         ('018e3a00-0000-7000-8000-000000000011', '018e3a00-0000-7000-8000-000000000010', 'Session 1'),
         ('018e3a00-0000-7000-8000-000000000021', '018e3a00-0000-7000-8000-000000000020', 'Session 2')",
        [],
    ).expect("seed sessions");

    // 2. Draft sessions: Draft A and Draft B
    conn.execute(
        "INSERT INTO draft_sessions (id, session_id, project_id, head_version_seq, status, created_at, updated_at) VALUES
         ('018e3a00-0000-7000-8000-0000000000a0', '018e3a00-0000-7000-8000-000000000011', '018e3a00-0000-7000-8000-000000000010', 2, 'editing', '2026-03-01T00:00:00Z', '2026-03-01T01:00:00Z'),
         ('018e3a00-0000-7000-8000-0000000000b0', '018e3a00-0000-7000-8000-000000000021', '018e3a00-0000-7000-8000-000000000020', 1, 'committing', '2026-03-01T02:00:00Z', '2026-03-01T02:30:00Z')",
        [],
    ).expect("seed draft_sessions");

    // 3. Draft versions for Draft A (seq 1, seq 2) and Draft B (seq 1)
    conn.execute(
        "INSERT INTO draft_versions (id, draft_session_id, seq, seal_key, phase, subphase, data_json, data_hash, reason, parent_seq, created_at) VALUES
         ('018e3a00-0000-7000-8000-0000000000a1', '018e3a00-0000-7000-8000-0000000000a0', 1, 'seal_a_1', 1, NULL, '{\"project\":{\"name\":\"Alpha\"}}', 'hash_a_1', 'manual_edit', NULL, '2026-03-01T00:10:00Z'),
         ('018e3a00-0000-7000-8000-0000000000a2', '018e3a00-0000-7000-8000-0000000000a0', 2, 'seal_a_2', 1, NULL, '{\"project\":{\"name\":\"Alpha Updated\"}}', 'hash_a_2', 'ai_patch', 1, '2026-03-01T00:50:00Z'),
         ('018e3a00-0000-7000-8000-0000000000b1', '018e3a00-0000-7000-8000-0000000000b0', 1, 'seal_b_1', 2, NULL, '{\"project\":{\"name\":\"Beta\"}}', 'hash_b_1', 'manual_edit', NULL, '2026-03-01T02:10:00Z')",
        [],
    ).expect("seed draft_versions");

    // 4. Pending patches for Draft A and Draft B
    conn.execute(
        "INSERT INTO pending_patches (id, draft_session_id, base_version_seq, target_type, target_code, field_name, operation, previous_value, new_value, target_source, eligibility_json, status, created_at) VALUES
         ('018e3a00-0000-7000-8000-0000000000a3', '018e3a00-0000-7000-8000-0000000000a0', 2, 'character', 'char_hero', 'name', 'set', '\"Hero\"', '\"Super Hero\"', 'user_edited', '{\"eligible\":true}', 'pending', '2026-03-01T01:00:00Z'),
         ('018e3a00-0000-7000-8000-0000000000a4', '018e3a00-0000-7000-8000-0000000000a0', 2, 'character', 'char_villain', 'role', 'set', '\"Rival\"', '\"Villain\"', 'ai_generated', NULL, 'confirmed', '2026-03-01T01:05:00Z'),
         ('018e3a00-0000-7000-8000-0000000000b3', '018e3a00-0000-7000-8000-0000000000b0', 1, 'world_setting', 'loc_capital', 'access', 'set', '\"PUBLIC\"', '\"RESTRICTED\"', 'user', NULL, 'pending', '2026-03-01T02:20:00Z')",
        [],
    ).expect("seed pending_patches");

    // 5. Catalog Entities: Motifs, Characters, World Settings, Lore, Prompt Blocks, Style Presets
    conn.execute(
        "INSERT INTO motifs (id, code, display_name, category, version, is_system, description, min_chapter, max_chapter) VALUES
         ('018e3a00-0000-7000-8000-0000000000m1', 'motif_journey', 'Hero Journey', 'character_arc', 'v1', 1, 'A classic journey motif', 1, 100),
         ('018e3a00-0000-7000-8000-0000000000m2', 'motif_betrayal', 'Sudden Betrayal', 'plot', 'v1', 0, 'A plot twist of betrayal', 5, 50)",
        [],
    ).expect("seed motifs");

    conn.execute(
        "INSERT INTO characters (id, code, display_name, nickname, birth, species, biological_sex, gender_identity, gender_expression, deleted_at) VALUES
         ('018e3a00-0000-7000-8000-0000000000c1', 'char_hero', 'Arthur Pendragon', 'King', 'Year 500', 'Human', 'MALE', 'MALE', 'MALE', NULL),
         ('018e3a00-0000-7000-8000-0000000000c2', 'char_deleted', 'Old Hermit', 'Hermit', 'Year 450', 'Human', 'NONE', 'NONE', 'NONE', '2026-03-01T00:00:00Z')",
        [],
    ).expect("seed characters");

    conn.execute(
        "INSERT INTO world_settings (id, code, project_id, display_name, type, parent_id, description, access, created_at) VALUES
         ('018e3a00-0000-7000-8000-0000000000w1', 'ws_camelot', '018e3a00-0000-7000-8000-000000000010', 'Camelot Castle', 'FACILITY', NULL, 'Seat of the High King', 'PUBLIC', '2026-03-01T00:00:00Z'),
         ('018e3a00-0000-7000-8000-0000000000w2', 'ws_avalon', '018e3a00-0000-7000-8000-000000000010', 'Avalon Isle', 'REGION', NULL, 'Sacred mystic island', 'SECRET', '2026-03-01T00:00:00Z')",
        [],
    ).expect("seed world_settings");

    conn.execute(
        "INSERT INTO lore_entries (id, code, type, display_name, description, world_setting_id, data) VALUES
         ('018e3a00-0000-7000-8000-0000000000l1', 'lore_excalibur', 'ITEM', 'Excalibur Sword', 'Legendary sword of power', '018e3a00-0000-7000-8000-0000000000w1', '{\"power\":100}')",
        [],
    ).expect("seed lore_entries");

    conn.execute(
        "INSERT INTO prompt_blocks (id, prompt_code, display_name, heading_level, content_format, content) VALUES
         ('018e3a00-0000-7000-8000-0000000000p1', 'pb_system_core', 'System Core', 1, 'MARKDOWN', 'You are the Merlink authoring assistant.'),
         ('018e3a00-0000-7000-8000-0000000000p2', 'pb_world_probe', 'World Probe Block', 2, 'JSON', '{\"scope\":\"world_setting\"}')",
        [],
    ).expect("seed prompt_blocks");

    conn.execute(
        "INSERT INTO style_presets (id, code, category, tone_pace, tone_emotion, is_system) VALUES
         ('018e3a00-0000-7000-8000-0000000000s1', 'style_epic', 'fantasy', 'MODERATE', 'AWE', 1),
         ('018e3a00-0000-7000-8000-0000000000s2', 'style_noir', 'mystery', 'FAST', 'TENSE', 0)",
        [],
    ).expect("seed style_presets");

    // 6. Workflow Receipts: Round receipt and Task receipt for Draft A
    conn.execute(
        "INSERT INTO workflow_round_receipts (
            id, draft_session_id, workflow_run_id, plan_generation, round_index,
            round_seal_key, task_count, parallel_limit, plan_digest, request_digest,
            base_version_seq, base_data_hash, final_data_hash, resulting_version_seq, created_at
         ) VALUES (
            '018e3a00-0000-7000-8000-0000000000r1',
            '018e3a00-0000-7000-8000-0000000000a0',
            'run_alpha_01', 1, 0, 'seal_round_a_0', 1, 1, 'plan_dig_1', 'req_dig_1',
            1, 'hash_a_1', 'hash_a_2', 2, '2026-03-01T00:50:00Z'
         )",
        [],
    ).expect("seed workflow_round_receipts");

    conn.execute(
        "INSERT INTO workflow_task_receipts (
            id, draft_session_id, workflow_run_id, plan_generation, task_identity,
            input_scope_fingerprint, output_scope_fingerprint, base_version_seq,
            resulting_version_seq, had_mutations, round_receipt_id, task_index,
            evidence_json, created_at
         ) VALUES (
            '018e3a00-0000-7000-8000-0000000000t1',
            '018e3a00-0000-7000-8000-0000000000a0',
            'run_alpha_01', 1, 'task_world_probe_01',
            'fp_in_01', 'fp_out_01', 1, 2, 1,
            '018e3a00-0000-7000-8000-0000000000r1', 0,
            '{\"summary\":\"Probed camelot castle\",\"mutations\":1}', '2026-03-01T00:49:00Z'
         )",
        [],
    ).expect("seed workflow_task_receipts");
}

// -----------------------------------------------------------------------------
// Acceptance Test: phase2_sprint6_deep2_F0
// -----------------------------------------------------------------------------
#[test]
#[allow(non_snake_case)]
fn phase2_sprint6_deep2_F0_predecessor_gate_absent_or_baseline_sha_drift_is_contract_blocked() {
    // 1. Missing predecessor gate p2s2 -> CONTRACT_BLOCKED
    let err1 = verify_p2s6_dispatch_preconditions(BASELINE_COMMIT, BASELINE_COMMIT, false, true).unwrap_err();
    assert_eq!(err1.error_code(), ErrorCode::ContractBlocked);

    // 2. Missing predecessor gate p2s3 -> CONTRACT_BLOCKED
    let err2 = verify_p2s6_dispatch_preconditions(BASELINE_COMMIT, BASELINE_COMMIT, true, false).unwrap_err();
    assert_eq!(err2.error_code(), ErrorCode::ContractBlocked);

    // 3. Baseline SHA drift -> CONTRACT_BLOCKED
    let err_drift = verify_p2s6_dispatch_preconditions(BASELINE_COMMIT, "deadbeef1234", true, true).unwrap_err();
    assert_eq!(err_drift.error_code(), ErrorCode::ContractBlocked);

    // 4. Positive verification in repository: git checks pass
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

    let p2s2_check = Command::new("git")
        .args(["cat-file", "-e", &format!("{PREDECESSOR_P2S2_COMMIT}^{{commit}}")])
        .current_dir(&repo_root)
        .status()
        .expect("git cat-file p2s2 execution");
    assert!(p2s2_check.success(), "Predecessor P2S2 commit must exist");

    let p2s3_check = Command::new("git")
        .args(["cat-file", "-e", &format!("{PREDECESSOR_P2S3_COMMIT}^{{commit}}")])
        .current_dir(&repo_root)
        .status()
        .expect("git cat-file p2s3 execution");
    assert!(p2s3_check.success(), "Predecessor P2S3 commit must exist");

    assert!(verify_p2s6_dispatch_preconditions(BASELINE_COMMIT, BASELINE_COMMIT, true, true).is_ok());
}

// -----------------------------------------------------------------------------
// Acceptance Test: phase2_sprint6_deep2_F1
// Missing schema blocks until P2S3 gate; no fallback JSON database
// -----------------------------------------------------------------------------
#[test]
#[allow(non_snake_case)]
fn phase2_sprint6_deep2_F1_missing_schema_blocks_without_fallback() {
    let temp_dir = TempDir::new().unwrap();
    let unmigrated_db = temp_dir.path().join("unmigrated.sqlite3");
    let conn = Connection::open(&unmigrated_db).unwrap();
    configure_connection(&conn).unwrap();

    let valid_uuid = "018e3a00-0000-7000-8000-0000000000a0";

    // 1. Draft repo returns MissingSchema on unmigrated database
    let draft_repo = SqliteDraftRepo::new(&conn);
    let res = draft_repo.find_session_by_id(valid_uuid);
    assert!(matches!(res, Err(DraftRepoError::MissingSchema(_))));

    let res_ver = draft_repo.list_versions(valid_uuid, None);
    assert!(matches!(res_ver, Err(DraftRepoError::MissingSchema(_))));

    // 2. Catalog repo returns MissingSchema on unmigrated database
    let catalog_repo = SqliteCatalogRepo::new(&conn);
    let res_motif = catalog_repo.get_motif_by_code("motif_journey");
    assert!(matches!(res_motif, Err(CatalogRepoError::MissingSchema(_))));

    let res_genres = catalog_repo.list_genres();
    assert!(matches!(res_genres, Err(CatalogRepoError::MissingSchema(_))));

    // 3. Workflow repo returns MissingSchema on unmigrated database
    let workflow_repo = SqliteWorkflowRepo::new(&conn);
    let res_round = workflow_repo.list_round_receipts(valid_uuid, "run_1");
    assert!(matches!(res_round, Err(WorkflowRepoError::MissingSchema(_))));
}

// -----------------------------------------------------------------------------
// Acceptance Test: phase2_sprint6_deep2_F2
// Traversal and foreign-draft queries return typed rejection with zero writes
// -----------------------------------------------------------------------------
#[test]
#[allow(non_snake_case)]
fn phase2_sprint6_deep2_F2_traversal_foreign_draft_query_typed_reject() {
    let temp_dir = TempDir::new().unwrap();
    let (_db_path, conn) = setup_disposable_db(&temp_dir);
    seed_test_data(&conn);

    let draft_a = "018e3a00-0000-7000-8000-0000000000a0";
    let draft_b = "018e3a00-0000-7000-8000-0000000000b0";
    let patch_b = "018e3a00-0000-7000-8000-0000000000b3"; // belongs to Draft B
    let round_a = "018e3a00-0000-7000-8000-0000000000r1"; // belongs to Draft A

    let draft_repo = SqliteDraftRepo::new(&conn);
    let workflow_repo = SqliteWorkflowRepo::new(&conn);

    // 1. Cross-draft version query: Draft A caller attempting to read Draft B target
    let err_cross_ver = draft_repo.query_scoped_version_record(draft_a, draft_b, 1).unwrap_err();
    assert!(matches!(
        err_cross_ver,
        DraftRepoError::ForeignDraftAccess { ref caller, ref target }
        if caller == draft_a && target == draft_b
    ));

    // 2. Draft A caller attempting to read Draft B's patch
    let err_cross_patch = draft_repo.query_scoped_patch(draft_a, patch_b).unwrap_err();
    assert!(matches!(
        err_cross_patch,
        DraftRepoError::ForeignDraftAccess { ref caller, ref target }
        if caller == draft_a && target == draft_b
    ));

    // 3. Workflow cross-draft round receipt query: Draft B caller attempting to read Draft A's round
    let err_cross_round = workflow_repo.query_scoped_round_receipt(draft_b, draft_a, round_a).unwrap_err();
    assert!(matches!(
        err_cross_round,
        WorkflowRepoError::ForeignDraftAccess { ref caller, ref target }
        if caller == draft_b && target == draft_a
    ));

    // 4. Invalid UUID / Traversal injection into draft session IDs
    assert!(matches!(
        draft_repo.find_session_by_id("../etc/passwd"),
        Err(DraftRepoError::InvalidSessionId(_))
    ));
    assert!(matches!(
        workflow_repo.find_round_receipt_by_id("../invalid", round_a),
        Err(WorkflowRepoError::InvalidSessionId(_))
    ));

    // 5. Zero writes verification: count of draft versions and round receipts is unaltered
    let count_versions: i64 = conn.query_row("SELECT count(*) FROM draft_versions", [], |r| r.get(0)).unwrap();
    assert_eq!(count_versions, 3);

    let count_rounds: i64 = conn.query_row("SELECT count(*) FROM workflow_round_receipts", [], |r| r.get(0)).unwrap();
    assert_eq!(count_rounds, 1);
}

// -----------------------------------------------------------------------------
// Acceptance Test: P2S6-P
// Repositories return exact scoped records from disposable seeded DB
// -----------------------------------------------------------------------------
#[test]
#[allow(non_snake_case)]
fn P2S6_P_repositories_return_exact_scoped_records_from_disposable_seeded_db() {
    let temp_dir = TempDir::new().unwrap();
    let (_db_path, conn) = setup_disposable_db(&temp_dir);
    seed_test_data(&conn);

    let draft_a = "018e3a00-0000-7000-8000-0000000000a0";
    let draft_b = "018e3a00-0000-7000-8000-0000000000b0";

    // 1. Test DraftRepo
    let draft_repo = SqliteDraftRepo::new(&conn);

    let session_a = draft_repo.find_session_by_id(draft_a).unwrap().expect("session A found");
    assert_eq!(session_a.id, draft_a);
    assert_eq!(session_a.head_version_seq, Some(2));
    assert_eq!(session_a.status, DraftSessionStatus::Editing);

    let status_b = draft_repo.get_session_status(draft_b).unwrap().expect("status B found");
    assert_eq!(status_b, DraftSessionStatus::Committing);

    let head_a = draft_repo.get_head_version(draft_a).unwrap().expect("head A found");
    assert_eq!(head_a.seq, 2);
    assert_eq!(head_a.data_hash, "hash_a_2");

    let ver_a_1 = draft_repo.get_version_by_seq(draft_a, 1).unwrap().expect("ver A.1 found");
    assert_eq!(ver_a_1.seal_key, Some("seal_a_1".to_string()));

    let ver_by_seal = draft_repo.find_version_by_seal_key(draft_a, "seal_a_2").unwrap().expect("found by seal");
    assert_eq!(ver_by_seal.seq, 2);

    let versions_a = draft_repo.list_versions(draft_a, None).unwrap();
    assert_eq!(versions_a.len(), 2);
    assert_eq!(versions_a[0].seq, 1);
    assert_eq!(versions_a[1].seq, 2);

    let versions_b = draft_repo.list_versions(draft_b, None).unwrap();
    assert_eq!(versions_b.len(), 1);
    assert_eq!(versions_b[0].seq, 1);

    // Pending patches scoping
    let patches_a = draft_repo.list_pending_patches(draft_a, None).unwrap();
    assert_eq!(patches_a.len(), 2);

    let pending_only_a = draft_repo.list_pending_patches(draft_a, Some(PendingPatchStatus::Pending)).unwrap();
    assert_eq!(pending_only_a.len(), 1);
    assert_eq!(pending_only_a[0].target_code, "char_hero");

    let confirmed_only_a = draft_repo.list_pending_patches(draft_a, Some(PendingPatchStatus::Confirmed)).unwrap();
    assert_eq!(confirmed_only_a.len(), 1);
    assert_eq!(confirmed_only_a[0].target_code, "char_villain");

    let patch_b_records = draft_repo.list_pending_patches(draft_b, None).unwrap();
    assert_eq!(patch_b_records.len(), 1);
    assert_eq!(patch_b_records[0].target_code, "loc_capital");

    // 2. Test CatalogRepo
    let catalog_repo = SqliteCatalogRepo::new(&conn);

    // Reference genres from migration
    let genres = catalog_repo.list_genres().unwrap();
    assert!(!genres.is_empty());
    let fantasy = catalog_repo.get_genre_by_code("fantasy").unwrap().expect("fantasy genre");
    assert_eq!(fantasy.display_name, Some("Kỳ Ảo".to_string()));

    // Motifs
    let motif = catalog_repo.get_motif_by_code("motif_journey").unwrap().expect("motif journey");
    assert_eq!(motif.display_name, Some("Hero Journey".to_string()));
    assert!(motif.is_system);

    let arc_motifs = catalog_repo.list_motifs(Some("character_arc")).unwrap();
    assert_eq!(arc_motifs.len(), 3);

    // Characters (include_deleted check)
    let active_chars = catalog_repo.list_characters(false).unwrap();
    assert_eq!(active_chars.len(), 1);
    assert_eq!(active_chars[0].code, Some("char_hero".to_string()));

    let all_chars = catalog_repo.list_characters(true).unwrap();
    assert_eq!(all_chars.len(), 2);

    // World settings
    let ws = catalog_repo.get_world_setting_by_code("ws_camelot").unwrap().expect("camelot");
    assert_eq!(ws.display_name, "Camelot Castle");
    assert_eq!(ws.setting_type, "FACILITY");

    // Lore entries
    let lore = catalog_repo.get_lore_entry_by_code("lore_excalibur").unwrap().expect("excalibur");
    assert_eq!(lore.entry_type, "ITEM");

    // Prompt blocks
    let pb = catalog_repo.get_prompt_block("pb_system_core").unwrap().expect("system core pb");
    assert_eq!(pb.heading_level, 1);
    assert_eq!(pb.content_format, "MARKDOWN");

    // Style presets
    let sp = catalog_repo.get_style_preset_by_code("style_epic").unwrap().expect("epic style");
    assert_eq!(sp.tone_pace, Some("MODERATE".to_string()));
    assert!(sp.is_system);

    // 3. Test WorkflowReadRepo
    let workflow_repo = SqliteWorkflowRepo::new(&conn);

    let round = workflow_repo
        .find_round_receipt(draft_a, "run_alpha_01", 1, 0)
        .unwrap()
        .expect("round receipt found");
    assert_eq!(round.round_seal_key, "seal_round_a_0");
    assert_eq!(round.base_version_seq, 1);
    assert_eq!(round.resulting_version_seq, Some(2));

    let rounds_a = workflow_repo.list_round_receipts(draft_a, "run_alpha_01").unwrap();
    assert_eq!(rounds_a.len(), 1);

    let rounds_b = workflow_repo.list_round_receipts(draft_b, "run_alpha_01").unwrap();
    assert_eq!(rounds_b.len(), 0); // Scoped to draft_b: zero rounds for run_alpha_01

    let task = workflow_repo
        .find_task_receipt(draft_a, "run_alpha_01", 1, "task_world_probe_01")
        .unwrap()
        .expect("task receipt found");
    assert_eq!(task.input_scope_fingerprint, "fp_in_01");
    assert!(task.had_mutations);
    assert_eq!(task.task_index, Some(0));

    let tasks_for_round = workflow_repo
        .list_task_receipts_for_round(draft_a, "018e3a00-0000-7000-8000-0000000000r1")
        .unwrap();
    assert_eq!(tasks_for_round.len(), 1);
    assert_eq!(tasks_for_round[0].task_identity, "task_world_probe_01");

    // 4. Test TempStore
    let temp_root = temp_dir.path().join("test_temp_root");
    let temp_store = TempStore::new(&temp_root);

    let artifact = temp_store
        .write_phase_report(draft_a, 3, "probe_hash_123", "# Probe Report\nFound Castle.")
        .unwrap();
    assert_eq!(artifact.phase, 3);
    assert_eq!(artifact.task_hash, "probe_hash_123");

    let read_back = temp_store.read_phase_report(draft_a, 3, "probe_hash_123").unwrap();
    assert_eq!(read_back, "# Probe Report\nFound Castle.");

    let reports = temp_store.read_phase_reports(draft_a, 3).unwrap();
    assert_eq!(reports.len(), 1);
    assert_eq!(reports[0].0, "probe_hash_123");
    assert_eq!(reports[0].1, "# Probe Report\nFound Castle.");

    // Clean session
    temp_store.clean_session(draft_a).unwrap();
    let after_clean = temp_store.read_phase_reports(draft_a, 3).unwrap();
    assert!(after_clean.is_empty());
}

// -----------------------------------------------------------------------------
// Acceptance Test: P2S6-N
// ../ path rejected; temp report cannot mark task complete
// -----------------------------------------------------------------------------
#[test]
#[allow(non_snake_case)]
fn P2S6_N_path_traversal_rejected_and_temp_report_cannot_mark_task_complete() {
    let temp_dir = TempDir::new().unwrap();
    let temp_root = temp_dir.path().join("test_temp_root");
    let temp_store = TempStore::new(&temp_root);

    let valid_uuid = "018e3a00-0000-7000-8000-0000000000a0";

    // 1. Path traversal via draft_session_id
    assert!(matches!(
        temp_store.resolve_report_path("../etc", 1, "task1"),
        Err(TempStoreError::PathTraversal(_))
    ));
    assert!(matches!(
        temp_store.resolve_report_path("018e3a00-0000-7000-8000-0000000000a0/../../bad", 1, "task1"),
        Err(TempStoreError::PathTraversal(_))
    ));

    // 2. Path traversal via task_hash
    assert!(matches!(
        temp_store.resolve_report_path(valid_uuid, 1, "../evil_hash"),
        Err(TempStoreError::PathTraversal(_))
    ));
    assert!(matches!(
        temp_store.resolve_report_path(valid_uuid, 1, "task/sub"),
        Err(TempStoreError::PathTraversal(_))
    ));

    // 3. Invalid phase boundaries
    assert!(matches!(
        temp_store.resolve_report_path(valid_uuid, 0, "task1"),
        Err(TempStoreError::InvalidPhase { phase: 0 })
    ));
    assert!(matches!(
        temp_store.resolve_report_path(valid_uuid, 7, "task1"),
        Err(TempStoreError::InvalidPhase { phase: 7 })
    ));

    // 4. Temp report cannot mark task complete (REQ-DB-004, be-core.md §1549)
    let artifact = temp_store
        .write_phase_report(valid_uuid, 1, "task_dummy", "dummy report content")
        .unwrap();

    // Invariant: ephemeral artifact CANNOT serve as durable completion proof
    assert!(!artifact.is_durable_completion_proof());

    let verify_res = temp_store.verify_durable_completion(&artifact);
    assert_eq!(verify_res, Err(TempStoreError::NonAuthoritativeProof));
}

// -----------------------------------------------------------------------------
// Acceptance Test: P2S6-B
// Injected SQL error does not expose partial result
// -----------------------------------------------------------------------------
#[test]
#[allow(non_snake_case)]
fn P2S6_B_injected_sql_error_does_not_expose_partial_result() {
    let temp_dir = TempDir::new().unwrap();
    let (_db_path, conn) = setup_disposable_db(&temp_dir);
    seed_test_data(&conn);

    let draft_a = "018e3a00-0000-7000-8000-0000000000a0";

    // 1. Simulate SQL injection / corrupt syntax by dropping a table in a nested transaction
    // or querying with a broken connection state.
    conn.execute("DROP TABLE pending_patches", []).expect("drop table to simulate failure");

    let draft_repo = SqliteDraftRepo::new(&conn);

    // List pending patches must fail with MissingSchema / Sqlite error and return NO partial list
    let res = draft_repo.list_pending_patches(draft_a, None);
    assert!(res.is_err(), "must fail cleanly on dropped/corrupt table");
    assert!(matches!(res.unwrap_err(), DraftRepoError::MissingSchema(_)));

    // Ensure other unaffected queries execute cleanly without state corruption
    let session = draft_repo.find_session_by_id(draft_a).unwrap().expect("session exists");
    assert_eq!(session.id, draft_a);
}
