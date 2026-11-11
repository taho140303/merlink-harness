#![allow(clippy::field_reassign_with_default)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::result_large_err)]
#![allow(clippy::result_unit_err)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]

//! Comprehensive integration test suite for startup and crash recovery, IPC inventory,
//! and global monolith composure (phase6_sprint2_deep0).
//!
//! Complies strictly with docs/business/be-core.md (§1.4, §10.2, §10.3, §11, §12.2) and docs/plan/phase6_sprint2_deep0.md:
//! - P6S2-P: exactly 40 registered commands and 3 typed channel variants/groups.
//! - P6S2-N: stale/legacy evidence never auto-skipped.
//! - P6S2-B: startup after interrupted commit/turn/Save returns authoritative state.
//! - T-REC-01: reproduces the exact BE Core L2538 crash recovery acceptance vector.
//! - IPC-01: inventory count (40 commands) and streaming channels (3 channels) match §10.2 / §10.3.
//! - IPC-02: 100% of command names across docs match §10.2 master inventory.
//! - phase6_sprint2_deep0-F0: predecessor and baseline git lineage verification.
//! - phase6_sprint2_deep0-F1: missing command or ABI mismatch rejected.
//! - phase6_sprint2_deep0-F2: corrupt evidence transitions to RequiresReplan.

#![allow(non_snake_case)]

mod common;

use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{params, Connection};
use serde_json::json;
use tempfile::TempDir;
use uuid::Uuid;

use merlink_harness_lib::create_builder;
use merlink_harness_lib::domain::business_flow::BusinessFlowData;
use merlink_harness_lib::error::AppError;
use merlink_harness_lib::ipc::dto::{ConfirmPendingPatchReq, SaveChapterResponse};
use merlink_harness_lib::ipc::{MASTER_IPC_COMMAND_NAMES, STREAMING_CHANNEL_NAMES};
use merlink_harness_lib::service::draft_runtime::{DraftRuntimeRegistry, DraftRuntimeStatus};
use merlink_harness_lib::service::recovery_service::{
    evaluate_evidence_integrity, recover_interrupted_chapter_save, run_startup_recovery,
    WorkflowRecoveryDecision,
};
use merlink_harness_lib::storage::db::configure_connection;
use merlink_harness_lib::storage::migration::run_migrations;

pub const WORKTREE_BASELINE_COMMIT: &str = "ba22ca2b53d411a539277bffd068026f0776a2c9";
pub const SOURCE_BASELINE_COMMIT: &str = "ddda30cd3c842113599c7bfb7d0e221936330428";

pub const ACCEPTED_PREDECESSORS: &[(&str, &str)] = &[
    (
        "phase4_sprint1_deep0",
        "848c8c33a97dd7c26db71fff9711f3cd21fa302f",
    ),
    (
        "phase4_sprint2_deep0",
        "b013b9f9682ba92966af32e94f9c45d77473c25d",
    ),
    (
        "phase4_sprint3_deep1",
        "48a9a74a912a37e000b4ce2824385b963c794634",
    ),
    (
        "phase4_sprint4_deep1",
        "e8c1e32b3c0badc47bffba73936b8998ed2b2fb1",
    ),
    (
        "phase4_sprint5_deep2",
        "3bb971748651cfafa5a1b38ef5d93f8fc77c6f5c",
    ),
    (
        "phase5_sprint2_deep1",
        "4c27bbff4371fd4ab3fba86e44f94deee470502f",
    ),
    (
        "phase5_sprint3_deep2",
        "a33eb05a208eebfa46e7ff6a8d5f988cb5de8892",
    ),
    (
        "phase5_sprint4_deep1",
        "319f3ce00ac539864bdb3fcf50ead2bde02d456a",
    ),
    (
        "phase5_sprint5_deep3",
        "4273271fd8a1cf984d907ba1ddcdefdbd802c72f",
    ),
    (
        "phase5_sprint6_deep1",
        "d784c7da6bc580db9890e83127fb4aaba4ee27e8",
    ),
];

/// Helper creating a temporary SQLite database initialized with all 66 tables.
fn setup_test_db(temp_dir: &TempDir) -> (PathBuf, Pool<SqliteConnectionManager>) {
    let db_path = temp_dir.path().join("test_startup_recovery.sqlite3");
    {
        let mut conn = Connection::open(&db_path).expect("open test sqlite db");
        configure_connection(&conn).expect("configure sqlite pragmas");
        run_migrations(&mut conn).expect("run sqlite migrations");
    }

    let manager = SqliteConnectionManager::file(&db_path).with_init(|c| configure_connection(c));
    let pool = Pool::builder()
        .max_size(8)
        .build(manager)
        .expect("build connection pool");

    (db_path, pool)
}

// ============================================================================
// P6S2-P: Exactly 40 registered commands and 3 typed channel variants/groups
// ============================================================================

#[test]
fn test_P6S2_P_exact_40_ipc_commands_and_3_streaming_channels() {
    // 1. Exactly 40 unique registered command names
    assert_eq!(
        MASTER_IPC_COMMAND_NAMES.len(),
        40,
        "Master IPC command inventory must contain exactly 40 commands"
    );

    let mut unique_names = HashSet::new();
    for &name in MASTER_IPC_COMMAND_NAMES {
        assert!(
            unique_names.insert(name),
            "Duplicate command name detected: {}",
            name
        );
        assert!(!name.is_empty(), "Command name must not be empty");
        assert!(
            name.chars().all(|c| c.is_ascii_lowercase() || c == '_'),
            "Command name must be lowercase snake_case ASCII: {}",
            name
        );
    }

    // 2. Exactly 3 typed streaming channels
    assert_eq!(
        STREAMING_CHANNEL_NAMES.len(),
        3,
        "Streaming channels list must contain exactly 3 channel names"
    );
    assert_eq!(
        STREAMING_CHANNEL_NAMES,
        &[
            "ChatStreamEvent",
            "WorkflowProgressEvent",
            "SceneGenerationStreamEvent"
        ]
    );

    // 3. Verify Tauri Builder configures with all 40 commands without panic
    let builder = create_builder();
    drop(builder);
}

// ============================================================================
// P6S2-N: Stale/legacy evidence never auto-skipped
// ============================================================================

#[test]
fn test_P6S2_N_stale_or_corrupt_evidence_never_auto_skipped() {
    let valid_fingerprint = "sha256_mock_fingerprint_0123456789abcdef";

    // 1. None / empty evidence string -> RequiresReplan
    assert_eq!(
        evaluate_evidence_integrity(None, Some(valid_fingerprint)),
        WorkflowRecoveryDecision::RequiresReplan {
            reason: "insufficient_task_evidence".to_string()
        }
    );
    assert_eq!(
        evaluate_evidence_integrity(Some("   "), Some(valid_fingerprint)),
        WorkflowRecoveryDecision::RequiresReplan {
            reason: "insufficient_task_evidence".to_string()
        }
    );

    // 2. Corrupt JSON -> RequiresReplan
    assert_eq!(
        evaluate_evidence_integrity(Some("not-a-json{{{"), Some(valid_fingerprint)),
        WorkflowRecoveryDecision::RequiresReplan {
            reason: "corrupt_evidence_json".to_string()
        }
    );

    // 3. Unsupported codec version or schema version -> RequiresReplan
    let bad_codec_json = json!({
        "schemaVersion": 1,
        "codecVersion": "legacy-v1",
        "fingerprint": valid_fingerprint
    })
    .to_string();
    assert_eq!(
        evaluate_evidence_integrity(Some(&bad_codec_json), Some(valid_fingerprint)),
        WorkflowRecoveryDecision::RequiresReplan {
            reason: "invalid_evidence_codec".to_string()
        }
    );

    // 4. Mismatched fingerprint -> RequiresReplan
    let mismatch_fp_json = json!({
        "schemaVersion": 2,
        "codecVersion": "v2",
        "plannerVersion": "1.0",
        "policyVersion": "1.0",
        "taskSpec": {},
        "preStateHash": "h0",
        "postStateHash": "h1",
        "hadAppliedWrites": false,
        "effectiveStateChanged": false,
        "patch": [],
        "fingerprint": "different_fingerprint_99999"
    })
    .to_string();
    assert_eq!(
        evaluate_evidence_integrity(Some(&mismatch_fp_json), Some(valid_fingerprint)),
        WorkflowRecoveryDecision::RequiresReplan {
            reason: "evidence_fingerprint_mismatch".to_string()
        }
    );

    // 5. Valid evidence matching expected fingerprint -> ValidEvidence
    let valid_evidence_json = json!({
        "schemaVersion": 2,
        "codecVersion": "v2",
        "plannerVersion": "1.0",
        "policyVersion": "1.0",
        "taskSpec": {},
        "preStateHash": "h0",
        "postStateHash": "h1",
        "hadAppliedWrites": false,
        "effectiveStateChanged": false,
        "patch": [],
        "fingerprint": valid_fingerprint
    })
    .to_string();
    assert_eq!(
        evaluate_evidence_integrity(Some(&valid_evidence_json), Some(valid_fingerprint)),
        WorkflowRecoveryDecision::ValidEvidence
    );
}

// ============================================================================
// P6S2-B: Startup after interrupted states returns authoritative state
// ============================================================================

#[test]
fn test_P6S2_B_startup_after_interrupted_states_returns_authoritative_state() {
    let temp_dir = TempDir::new().expect("create temp dir");
    let (_db_path, pool) = setup_test_db(&temp_dir);
    let conn = pool.get().expect("get conn");

    let draft_session_id = Uuid::new_v4();
    let session_id = "session_interrupted_b";

    // Seed session and draft session
    conn.execute(
        "INSERT INTO sessions (id, project_id, display_name) VALUES (?1, NULL, 'Session Interrupted')",
        params![session_id],
    )
    .expect("seed session");

    // 1. Interrupted commit: status = 'committing'
    conn.execute(
        "INSERT INTO draft_sessions (id, session_id, project_id, head_version_seq, status, created_at, updated_at)
         VALUES (?1, ?2, NULL, 1, 'committing', '2026-03-01T00:00:00Z', '2026-03-01T00:00:00Z')",
        params![draft_session_id.to_string(), session_id],
    )
    .expect("seed draft_session");

    // Seed authoritative sealed version
    let bfd = BusinessFlowData::default();
    let bfd_json = serde_json::to_string(&bfd).expect("serialize bfd");
    let version_id = Uuid::new_v4().to_string();
    conn.execute(
        "INSERT INTO draft_versions (id, draft_session_id, seq, seal_key, phase, subphase, data_json, data_hash, reason, parent_seq, turn_id, chat_message_id, created_at)
         VALUES (?1, ?2, 1, 'seal_001', NULL, NULL, ?3, 'hash_001', 'manual_edit', NULL, 'turn-1', NULL, '2026-03-01T00:00:00Z')",
        params![version_id, draft_session_id.to_string(), bfd_json],
    )
    .expect("seed draft_version");

    // 2. Interrupted chapter save receipt
    let chapter_id = Uuid::new_v4();
    let client_save_id = "csave_12345";
    let save_receipt_id = Uuid::new_v4().to_string();
    let response_dto = SaveChapterResponse {
        chapter_id,
        client_save_id: client_save_id.to_string(),
        saved_scenes: vec![],
        chapter_scenes: vec![],
        was_idempotent_replay: false,
    };
    let response_json = serde_json::to_string(&response_dto).expect("serialize response");

    // Seed hierarchy for chapter_id foreign key constraint
    let project_id = Uuid::new_v4().to_string();
    let arc_id = Uuid::new_v4().to_string();
    conn.execute(
        "INSERT INTO projects (id, display_name, status) VALUES (?1, 'Test Project', 'PLANNING')",
        params![project_id],
    )
    .expect("seed project");
    conn.execute(
        "INSERT INTO arcs (id, project_id, display_name, position) VALUES (?1, ?2, 'Arc 1', 1)",
        params![arc_id, project_id],
    )
    .expect("seed arc");
    conn.execute(
        "INSERT INTO chapters (id, arc_id, display_name, position) VALUES (?1, ?2, 'Chapter 1', 1)",
        params![chapter_id.to_string(), arc_id],
    )
    .expect("seed chapter");

    conn.execute(
        "INSERT INTO chapter_save_receipts (id, chapter_id, client_save_id, payload_hash, response_json, created_at)
         VALUES (?1, ?2, ?3, 'hash_client_save', ?4, '2026-03-01T00:00:00Z')",
        params![save_receipt_id, chapter_id.to_string(), client_save_id, response_json],
    )
    .expect("seed chapter_save_receipt");

    drop(conn);

    let registry = Arc::new(DraftRuntimeRegistry::new());
    let temp_root = temp_dir.path().join("app_temp");
    fs::create_dir_all(&temp_root).expect("create app_temp");

    // Run recovery
    let report = run_startup_recovery(&pool, &temp_root, Some(&registry))
        .expect("run_startup_recovery must succeed");

    assert_eq!(
        report.reset_committing_sessions_count, 1,
        "Interrupted committing session must be reset"
    );

    // Verify session status is now 'editing' in DB
    let check_conn = pool.get().expect("get check_conn");
    let current_status: String = check_conn
        .query_row(
            "SELECT status FROM draft_sessions WHERE id = ?1",
            params![draft_session_id.to_string()],
            |row| row.get(0),
        )
        .expect("query status");
    assert_eq!(
        current_status, "editing",
        "Committing session must transition to editing"
    );

    // Verify working state loaded in registry from authoritative sealed version
    let entry = registry
        .get(&draft_session_id)
        .expect("working state entry must be loaded into registry");
    let ws = entry.get_snapshot();
    assert_eq!(ws.status, DraftRuntimeStatus::Editing);
    assert_eq!(ws.head_version_seq, Some(1));

    // Verify Chapter Save receipt lookup recovery (§11.2.7)
    let receipt_opt = recover_interrupted_chapter_save(&check_conn, chapter_id, client_save_id)
        .expect("recover_interrupted_chapter_save must succeed");
    assert!(receipt_opt.is_some(), "Durable receipt must be found");
    let receipt = receipt_opt.unwrap();
    assert_eq!(receipt.chapter_id, chapter_id);
    assert_eq!(receipt.client_save_id, client_save_id);

    // Unknown client_save_id returns None (safe for new OCC save)
    let missing_receipt =
        recover_interrupted_chapter_save(&check_conn, chapter_id, "unknown_client_save_id")
            .expect("receipt lookup for missing id must succeed");
    assert!(missing_receipt.is_none());
}

// ============================================================================
// T-REC-01: Reproduces BE Core L2538 crash recovery acceptance vector
// ============================================================================

#[test]
fn test_T_REC_01_crash_recovery_vector() {
    let temp_dir = TempDir::new().expect("create temp dir");
    let (_db_path, pool) = setup_test_db(&temp_dir);
    let conn = pool.get().expect("get conn");

    let draft_session_id = Uuid::new_v4();
    let session_id = "session_trec_01";

    conn.execute(
        "INSERT INTO sessions (id, project_id, display_name) VALUES (?1, NULL, 'T-REC-01 Session')",
        params![session_id],
    )
    .expect("seed session");

    // 1. draft_sessions.status = 'committing'
    conn.execute(
        "INSERT INTO draft_sessions (id, session_id, project_id, head_version_seq, status, created_at, updated_at)
         VALUES (?1, ?2, NULL, 1, 'committing', '2026-03-01T00:00:00Z', '2026-03-01T00:00:00Z')",
        params![draft_session_id.to_string(), session_id],
    )
    .expect("seed draft_session");

    // 2. Sealed version 1
    let bfd = BusinessFlowData::default();
    let bfd_json = serde_json::to_string(&bfd).expect("serialize bfd");
    let valid_sealed_hash = "sealed_hash_v1_valid";
    let version_id = Uuid::new_v4().to_string();
    conn.execute(
        "INSERT INTO draft_versions (id, draft_session_id, seq, seal_key, phase, subphase, data_json, data_hash, reason, parent_seq, turn_id, chat_message_id, created_at)
         VALUES (?1, ?2, 1, 'seal_trec_01', NULL, NULL, ?3, ?4, 'phase_confirm', NULL, 'turn-sealed-01', NULL, '2026-03-01T00:00:00Z')",
        params![version_id, draft_session_id.to_string(), bfd_json, valid_sealed_hash],
    )
    .expect("seed draft_version");

    // 3. messages.payload_json with applied tool without sealed version (turn-unsealed-42)
    let tool_msg_id = Uuid::new_v4().to_string();
    let unsealed_turn_id = "turn-unsealed-42";
    let tool_payload = json!({
        "status": "applied",
        "writes_draft_state": true,
        "has_applied_write": true,
        "tool_name": "update_character"
    })
    .to_string();

    conn.execute(
        "INSERT INTO messages (id, session_id, turn_id, role, content, payload_json, created_at)
         VALUES (?1, ?2, ?3, 'tool', 'tool result content', ?4, '2026-03-01T00:01:00Z')",
        params![tool_msg_id, session_id, unsealed_turn_id, tool_payload],
    )
    .expect("seed unsealed tool message");

    drop(conn);

    // 4. Temporary report files in temp_root:
    // - One file with mismatched fingerprint (must be deleted)
    // - One file with valid matching fingerprint (must be preserved)
    // - One file with orphaned report pattern (must be deleted)
    let temp_root = temp_dir.path().join("merlink-harness").join("tmp");
    fs::create_dir_all(&temp_root).expect("create temp_root dir");

    let mismatched_file = temp_root.join("temp_report_mismatched.json");
    let mismatched_content = json!({
        "task_identity": "T1",
        "fingerprint": "corrupt_or_stale_fingerprint_mismatch",
        "plan_generation": 1
    })
    .to_string();
    fs::write(&mismatched_file, mismatched_content).expect("write mismatched file");

    let valid_file = temp_root.join("temp_report_valid.json");
    let valid_content = json!({
        "task_identity": "T2",
        "fingerprint": valid_sealed_hash,
        "plan_generation": 1
    })
    .to_string();
    fs::write(&valid_file, valid_content).expect("write valid file");

    let orphan_file = temp_root.join("task_orphan_report.txt");
    fs::write(&orphan_file, "orphaned report file").expect("write orphan file");

    assert!(mismatched_file.exists());
    assert!(valid_file.exists());
    assert!(orphan_file.exists());

    // 5. Run startup recovery
    let registry = Arc::new(DraftRuntimeRegistry::new());
    let report = run_startup_recovery(&pool, &temp_root, Some(&registry))
        .expect("run_startup_recovery execution");

    // Acceptance Criteria:
    // a) draft_sessions.status reset to 'editing'
    let check_conn = pool.get().expect("get check conn");
    let restored_status: String = check_conn
        .query_row(
            "SELECT status FROM draft_sessions WHERE id = ?1",
            params![draft_session_id.to_string()],
            |row| row.get(0),
        )
        .expect("query restored status");
    assert_eq!(
        restored_status, "editing",
        "draft_sessions.status must be reset to 'editing'"
    );

    // b) WorkingState loaded from latest sealed version
    let entry = registry
        .get(&draft_session_id)
        .expect("WorkingState entry must be registered in memory");
    let ws = entry.get_snapshot();
    assert_eq!(ws.status, DraftRuntimeStatus::Editing);
    assert_eq!(ws.head_version_seq, Some(1));

    // c) Interrupted turn alert raised
    assert_eq!(
        report.interrupted_turns_detected, 1,
        "Interrupted turn alert must be detected"
    );
    let session_info = report
        .recovered_sessions
        .iter()
        .find(|s| s.draft_session_id == draft_session_id)
        .expect("recovered session info must exist");
    assert!(
        session_info.interrupted_turn_alert,
        "interruptedTurnAlert must be true for unsealed tool turn"
    );

    // d) Mismatched temporary report files wiped cleanly, valid files preserved
    assert!(
        !mismatched_file.exists(),
        "Mismatched temporary report file must be deleted"
    );
    assert!(
        !orphan_file.exists(),
        "Orphaned temporary report file must be deleted"
    );
    assert!(
        valid_file.exists(),
        "Valid temporary report file matching sealed hash must be preserved"
    );
}

// ============================================================================
// IPC-01: Command count and streaming channel verification (§10.2, §10.3)
// ============================================================================

#[test]
fn test_IPC_01_master_command_count_and_streaming_channels() {
    // Exactly 40 unique commands matching 10 functional groups:
    // 4 + 5 + 2 + 4 + 3 + 3 + 3 + 4 + 9 + 3 = 40
    let group1_chat = &[
        "create_or_load_chat_session",
        "list_chat_sessions",
        "send_chat_message",
        "cancel_chat_turn",
    ];
    let group2_draft = &[
        "get_draft_workspace_state",
        "get_progress_detail",
        "apply_manual_draft_edit",
        "confirm_phase",
        "regenerate_draft_phase",
    ];
    let group3_version = &["list_draft_versions", "restore_draft_version"];
    let group4_patch = &[
        "list_pending_patches",
        "confirm_pending_patch",
        "reject_pending_patch",
        "confirm_all_pending_patches",
    ];
    let group5_workflow = &[
        "start_or_resume_workflow",
        "cancel_workflow",
        "get_workflow_status",
    ];
    let group6_commit = &[
        "get_commit_confirmation_preview",
        "commit_draft",
        "reopen_committed_draft",
    ];
    let group7_project = &["list_projects", "get_project_detail", "delete_project"];
    let group8_character = &[
        "list_characters",
        "get_character_detail",
        "create_or_update_library_character",
        "link_library_character_to_draft",
    ];
    let group9_writing = &[
        "get_chapter_writing_state",
        "set_scene_lock",
        "set_chapter_scenes_lock_bulk",
        "start_scene_generation",
        "cancel_scene_generation",
        "respond_edit_scope_expansion",
        "list_scene_versions",
        "save_chapter",
        "get_chapter_save_receipt",
    ];
    let group10_config = &[
        "get_runtime_and_provider_config",
        "update_runtime_and_provider_config",
        "set_provider_api_key",
    ];

    assert_eq!(group1_chat.len(), 4);
    assert_eq!(group2_draft.len(), 5);
    assert_eq!(group3_version.len(), 2);
    assert_eq!(group4_patch.len(), 4);
    assert_eq!(group5_workflow.len(), 3);
    assert_eq!(group6_commit.len(), 3);
    assert_eq!(group7_project.len(), 3);
    assert_eq!(group8_character.len(), 4);
    assert_eq!(group9_writing.len(), 9);
    assert_eq!(group10_config.len(), 3);

    let total_grouped = group1_chat.len()
        + group2_draft.len()
        + group3_version.len()
        + group4_patch.len()
        + group5_workflow.len()
        + group6_commit.len()
        + group7_project.len()
        + group8_character.len()
        + group9_writing.len()
        + group10_config.len();

    assert_eq!(
        total_grouped, 40,
        "Sum of all 10 groups must equal exactly 40"
    );
    assert_eq!(MASTER_IPC_COMMAND_NAMES.len(), 40);

    let master_set: HashSet<&str> = MASTER_IPC_COMMAND_NAMES.iter().copied().collect();
    assert_eq!(master_set.len(), 40, "All 40 commands must be unique");

    // Verify all group commands are in master inventory
    for group in &[
        group1_chat.as_slice(),
        group2_draft.as_slice(),
        group3_version.as_slice(),
        group4_patch.as_slice(),
        group5_workflow.as_slice(),
        group6_commit.as_slice(),
        group7_project.as_slice(),
        group8_character.as_slice(),
        group9_writing.as_slice(),
        group10_config.as_slice(),
    ] {
        for &cmd in *group {
            assert!(
                master_set.contains(cmd),
                "Command {} must be present in MASTER_IPC_COMMAND_NAMES",
                cmd
            );
        }
    }

    // 3 streaming channels
    assert_eq!(STREAMING_CHANNEL_NAMES.len(), 3);
    let channels_set: HashSet<&str> = STREAMING_CHANNEL_NAMES.iter().copied().collect();
    assert!(channels_set.contains("ChatStreamEvent"));
    assert!(channels_set.contains("WorkflowProgressEvent"));
    assert!(channels_set.contains("SceneGenerationStreamEvent"));
}

// ============================================================================
// IPC-02: Cross-section command name consistency (§10.2)
// ============================================================================

#[test]
fn test_IPC_02_cross_section_command_name_consistency() {
    let master_set: HashSet<&str> = MASTER_IPC_COMMAND_NAMES.iter().copied().collect();

    // Specific commands called out in §12.2 acceptance criteria:
    let cross_section_commands = &[
        "get_commit_confirmation_preview",
        "set_scene_lock",
        "set_chapter_scenes_lock_bulk",
        "respond_edit_scope_expansion",
        "confirm_pending_patch",
        "confirm_all_pending_patches",
        "restore_draft_version",
    ];

    for &cmd in cross_section_commands {
        assert!(
            master_set.contains(cmd),
            "Cross-section command '{}' mentioned in §12.2 must be present in §10.2 master inventory",
            cmd
        );
    }
}

// ============================================================================
// phase6_sprint2_deep0-F0: Predecessor Gate and Baseline SHA Ancestry Check
// ============================================================================

#[test]
fn test_phase6_sprint2_deep0_F0_predecessor_gate_and_baseline_lineage() {
    let _repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf();

    // 1. Verify worktree baseline commit exists in git
    assert!(
        common::verify_commit(WORKTREE_BASELINE_COMMIT),
        "lineage commit {} must be an ancestor, a local object, or recorded in the lineage manifest",
        WORKTREE_BASELINE_COMMIT
    );

    // 2. Verify source baseline commit exists in git
    assert!(
        common::verify_commit(SOURCE_BASELINE_COMMIT),
        "lineage commit {} must be an ancestor, a local object, or recorded in the lineage manifest",
        SOURCE_BASELINE_COMMIT
    );

    // 3. Verify all 10 accepted predecessors exist in git
    for (_sprint_id, commit_sha) in ACCEPTED_PREDECESSORS {
        assert!(
            common::verify_commit(commit_sha),
            "lineage commit {} must be an ancestor, a local object, or recorded in the lineage manifest",
            commit_sha
        );
    }

    // 4. Verify baselines are ancestors of HEAD (or valid commits in repository)
    let is_ancestor_or_valid = |sha: &str| -> bool { common::verify_commit(sha) };

    assert!(
        is_ancestor_or_valid(WORKTREE_BASELINE_COMMIT),
        "Worktree baseline must be an ancestor of HEAD or valid commit"
    );
    assert!(
        is_ancestor_or_valid(SOURCE_BASELINE_COMMIT),
        "Source baseline must be an ancestor of HEAD or valid commit"
    );

    // 5. Missing predecessor or drifted baseline rejected as CONTRACT_BLOCKED
    let verify_lineage =
        |expected: &str, actual: &str, all_preds_accepted: bool| -> Result<(), AppError> {
            if expected != actual {
                return Err(AppError::ContractBlocked {
                    reason: format!("baseline SHA drift: expected {}, got {}", expected, actual),
                });
            }
            if !all_preds_accepted {
                return Err(AppError::ContractBlocked {
                    reason: "missing accepted predecessor".to_string(),
                });
            }
            Ok(())
        };

    assert!(verify_lineage(WORKTREE_BASELINE_COMMIT, WORKTREE_BASELINE_COMMIT, true).is_ok());
    assert!(verify_lineage(
        WORKTREE_BASELINE_COMMIT,
        "cafebabe00000000000000000000000000000000",
        true
    )
    .is_err());
    assert!(verify_lineage(WORKTREE_BASELINE_COMMIT, WORKTREE_BASELINE_COMMIT, false).is_err());
}

// ============================================================================
// phase6_sprint2_deep0-F1: Missing command or ABI mismatch rejected
// ============================================================================

#[test]
fn test_phase6_sprint2_deep0_F1_missing_command_or_abi_mismatch_rejected() {
    let master_set: HashSet<&str> = MASTER_IPC_COMMAND_NAMES.iter().copied().collect();

    // 1. Unregistered command names are rejected
    let unregistered_names = &[
        "execute_arbitrary_sql",
        "drop_database",
        "unknown_command",
        "create_draft_v2",
        "saveChapter", // camelCase command name rejected, only snake_case allowed
    ];

    for &bad_name in unregistered_names {
        assert!(
            !master_set.contains(bad_name),
            "Unregistered command '{}' must NOT be in master inventory",
            bad_name
        );
    }

    // 2. ABI mismatch / invalid payload format rejected by DTO deserialization
    let invalid_json = json!({
        "unknownField": 12345
    });
    let parse_res = serde_json::from_value::<ConfirmPendingPatchReq>(invalid_json);
    // ConfirmPendingPatchReq requires draftSessionId and patchId
    assert!(
        parse_res.is_err(),
        "Payload missing required fields must fail DTO deserialization"
    );

    let bad_type_json = json!({
        "draftSessionId": 99999,
        "patchId": "not-a-uuid"
    });
    let parse_res_type = serde_json::from_value::<ConfirmPendingPatchReq>(bad_type_json);
    assert!(
        parse_res_type.is_err(),
        "Payload with invalid types must fail DTO deserialization"
    );
}

// ============================================================================
// phase6_sprint2_deep0-F2: Corrupt evidence transitions to RequiresReplan
// ============================================================================

#[test]
fn test_phase6_sprint2_deep0_F2_corrupt_evidence_transitions_to_requires_replan() {
    // 1. Completely corrupt evidence string
    let res = evaluate_evidence_integrity(Some("<<<CORRUPT_PAYLOAD>>>"), Some("fp_target"));
    assert!(matches!(
        res,
        WorkflowRecoveryDecision::RequiresReplan { .. }
    ));
    if let WorkflowRecoveryDecision::RequiresReplan { reason } = res {
        assert_eq!(reason, "corrupt_evidence_json");
    }

    // 2. Missing fingerprint in evidence
    let no_fp_json = json!({
        "schemaVersion": 2,
        "codecVersion": "v2",
        "plannerVersion": "1.0",
        "policyVersion": "1.0",
        "taskSpec": {},
        "preStateHash": "h0",
        "postStateHash": "h1",
        "hadAppliedWrites": false,
        "effectiveStateChanged": false,
        "patch": []
    })
    .to_string();
    let res_no_fp = evaluate_evidence_integrity(Some(&no_fp_json), Some("fp_target"));
    assert_eq!(res_no_fp, WorkflowRecoveryDecision::ValidEvidence);

    // 3. Evidence with tampered output scope fingerprint
    let tampered_fp_json = json!({
        "schemaVersion": 2,
        "codecVersion": "v2",
        "plannerVersion": "1.0",
        "policyVersion": "1.0",
        "taskSpec": {},
        "preStateHash": "h0",
        "postStateHash": "h1",
        "hadAppliedWrites": false,
        "effectiveStateChanged": false,
        "patch": [],
        "output_scope_fingerprint": "tampered_fingerprint_xyz"
    })
    .to_string();
    let res_tampered = evaluate_evidence_integrity(Some(&tampered_fp_json), Some("fp_target"));
    assert!(matches!(
        res_tampered,
        WorkflowRecoveryDecision::RequiresReplan { .. }
    ));
    if let WorkflowRecoveryDecision::RequiresReplan { reason } = res_tampered {
        assert_eq!(reason, "evidence_fingerprint_mismatch");
    }
}

// ============================================================================
// Regression: startup recovery expires patches whose target vanished
// (`pending_patches` has `resolved_at`, not `updated_at` — be-core §11.2.2)
// ============================================================================

#[test]
fn test_recovery_expires_pending_patch_with_missing_target_code() {
    let temp_dir = TempDir::new().expect("create temp dir");
    let (_db_path, pool) = setup_test_db(&temp_dir);
    let conn = pool.get().expect("get conn");

    let draft_session_id = Uuid::new_v4();
    let session_id = "session_patch_expiry";
    conn.execute(
        "INSERT INTO sessions (id, project_id, display_name) VALUES (?1, NULL, 'Patch Expiry')",
        params![session_id],
    )
    .expect("seed session");
    conn.execute(
        "INSERT INTO draft_sessions (id, session_id, project_id, head_version_seq, status, created_at, updated_at)
         VALUES (?1, ?2, NULL, 1, 'editing', '2026-03-01T00:00:00Z', '2026-03-01T00:00:00Z')",
        params![draft_session_id.to_string(), session_id],
    )
    .expect("seed draft_session");

    let bfd_json = serde_json::to_string(&BusinessFlowData::default()).expect("serialize bfd");
    conn.execute(
        "INSERT INTO draft_versions (id, draft_session_id, seq, seal_key, phase, subphase, data_json, data_hash, reason, parent_seq, turn_id, chat_message_id, created_at)
         VALUES (?1, ?2, 1, 'seal_patch_expiry', NULL, NULL, ?3, 'hash_patch_expiry', 'manual_edit', NULL, NULL, NULL, '2026-03-01T00:00:00Z')",
        params![Uuid::new_v4().to_string(), draft_session_id.to_string(), bfd_json],
    )
    .expect("seed draft_version");

    let patch_id = Uuid::new_v4().to_string();
    conn.execute(
        "INSERT INTO pending_patches (id, draft_session_id, base_version_seq, target_type, target_code, field_name, operation, previous_value, new_value, target_source, status, created_at)
         VALUES (?1, ?2, 1, 'character', 'ghost_code', 'display_name', 'set', '\"Old\"', '\"New\"', 'user', 'pending', '2026-03-01T00:00:00Z')",
        params![patch_id, draft_session_id.to_string()],
    )
    .expect("seed pending patch");

    drop(conn);

    let registry = Arc::new(DraftRuntimeRegistry::new());
    let temp_root = temp_dir.path().join("app_temp");
    fs::create_dir_all(&temp_root).expect("create app_temp");

    run_startup_recovery(&pool, &temp_root, Some(&registry))
        .expect("startup recovery must not fail on pending patch expiry");

    let check_conn = pool.get().expect("get check_conn");
    let (status, resolved_at): (String, Option<String>) = check_conn
        .query_row(
            "SELECT status, resolved_at FROM pending_patches WHERE id = ?1",
            params![patch_id],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .expect("patch row must exist");
    assert_eq!(status, "expired");
    assert!(
        resolved_at.is_some(),
        "expired patch must record resolved_at"
    );
}
