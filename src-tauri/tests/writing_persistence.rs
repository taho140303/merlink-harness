#![allow(clippy::field_reassign_with_default)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::result_large_err)]
#![allow(clippy::result_unit_err)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]

//! Acceptance and integration tests for `phase4_sprint4_deep1`
//! ("Persist Writing read-model locks and chapter Save").
//!
//! Complies with docs/business/core.md (§1.5, §1.6), docs/business/be-core.md (§8.1..§8.5, §10.2.9),
//! and docs/plan/evidence/contract-ledger.md (`CI-06`, `CI-08`, `SF-02`, `SF-07`).

#![allow(non_snake_case)]

mod common;

#[path = "../src/error.rs"]
pub mod error;

#[path = "../src/domain/business_flow.rs"]
pub mod business_flow;
#[path = "../src/domain/code_allocator.rs"]
pub mod code_allocator;
#[path = "../src/domain/fingerprint.rs"]
pub mod fingerprint;
#[path = "../src/domain/invariants.rs"]
pub mod invariants;
#[path = "../src/domain/progress_engine.rs"]
pub mod progress_engine;
#[path = "../src/domain/word_count.rs"]
pub mod word_count;

pub mod domain {
    pub use crate::business_flow;
    pub use crate::code_allocator;
    pub use crate::fingerprint;
    pub use crate::invariants;
    pub use crate::progress_engine;
    pub use crate::word_count;
}

#[path = "../src/storage/db.rs"]
pub mod db;
#[path = "../src/storage/migration.rs"]
pub mod migration;
#[path = "../src/storage/writing_repo.rs"]
pub mod writing_repo;

pub mod storage {
    pub use crate::db;
    pub use crate::migration;
    pub use crate::writing_repo;
}

#[path = "../src/service/writing_persistence.rs"]
pub mod writing_persistence;

pub mod service {
    pub use crate::writing_persistence;
}

#[path = "../src/ipc/dto.rs"]
pub mod dto;
#[path = "../src/ipc/writing_persistence_commands.rs"]
pub mod writing_persistence_commands;

pub mod ipc {
    pub use crate::dto;
    pub use crate::writing_persistence_commands;
}

use chrono::Utc;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{params, Connection};
use std::path::PathBuf;
use tempfile::TempDir;
use uuid::Uuid;

use error::ErrorCode;
use ipc::dto::*;
use ipc::writing_persistence_commands::*;
use service::writing_persistence::{CallerContext, WritingPersistenceService};
use storage::db::configure_connection;
use storage::migration::run_migrations;

// ============================================================================
// Test Fixture Setup Helpers
// ============================================================================

fn setup_test_db(temp_dir: &TempDir) -> (PathBuf, Pool<SqliteConnectionManager>) {
    let db_path = temp_dir.path().join("test_merlink_p4s4.sqlite3");
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

/// Seeds a test project, arc, chapter, and scenes.
///
/// If `is_committed` is true, sets `projects.status = 'DRAFTING'` and adds a committed `draft_sessions`.
/// If false, sets `projects.status = 'PLANNING'` and `draft_sessions.status = 'editing'`.
fn seed_test_writing_project(conn: &Connection, is_committed: bool) -> (Uuid, Uuid, Vec<Uuid>) {
    let project_id = Uuid::now_v7();
    let arc_id = Uuid::now_v7();
    let chapter_id = Uuid::now_v7();
    let ws_id = Uuid::now_v7();
    let session_id = format!("sess_{}", Uuid::now_v7().simple());
    let draft_id = Uuid::now_v7();

    let now = Utc::now().to_rfc3339();
    let status = if is_committed { "DRAFTING" } else { "PLANNING" };

    conn.execute(
        "INSERT INTO projects (id, code, display_name, summary, status, language, created_at, updated_at)
         VALUES (?1, ?2, 'Thần Đạo Đan Tôn', 'Hành trình luyện đan tu tiên', ?3, 'VI', ?4, ?4)",
        params![project_id.to_string(), format!("p_{}", project_id.simple()), status, now],
    )
    .expect("seed project");

    conn.execute(
        "INSERT INTO sessions (id, project_id, display_name) VALUES (?1, ?2, 'Active Session')",
        params![session_id, project_id.to_string()],
    )
    .expect("seed session");

    let draft_status = if is_committed { "committed" } else { "editing" };
    conn.execute(
        "INSERT INTO draft_sessions (id, session_id, project_id, status, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?5)",
        params![
            draft_id.to_string(),
            session_id,
            project_id.to_string(),
            draft_status,
            now
        ],
    )
    .expect("seed draft_session");

    conn.execute(
        "INSERT INTO world_settings (id, code, project_id, display_name, type)
         VALUES (?1, ?2, ?3, 'Cổ Giới Đan Tông', 'REALM')",
        params![
            ws_id.to_string(),
            format!("ws_{}", ws_id.simple()),
            project_id.to_string()
        ],
    )
    .expect("seed world_settings");

    conn.execute(
        "INSERT INTO arcs (id, code, display_name, project_id, position)
         VALUES (?1, ?2, 'Quyển 1: Xuất Thế', ?3, 1)",
        params![
            arc_id.to_string(),
            format!("arc_{}", arc_id.simple()),
            project_id.to_string()
        ],
    )
    .expect("seed arc");

    conn.execute(
        "INSERT INTO chapters (id, code, display_name, arc_id, position, summary, created_at, updated_at)
         VALUES (?1, ?2, 'Chương 1: Trùng Sinh', ?3, 1, 'Lăng Hàn trọng sinh tại Thương Lãng Đại Lục', ?4, ?4)",
        params![chapter_id.to_string(), format!("ch_{}", chapter_id.simple()), arc_id.to_string(), now],
    )
    .expect("seed chapter");

    let mut scene_ids = Vec::new();
    for i in 1..=3 {
        let s_id = Uuid::now_v7();
        scene_ids.push(s_id);
        conn.execute(
            "INSERT INTO scenes (id, code, display_name, chapter_id, world_setting_id, summary, content, word_count, position, is_locked)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, NULL, 0, ?7, 0)",
            params![
                s_id.to_string(),
                format!("scn_{}", s_id.simple()),
                format!("Cảnh {}: Khởi đầu", i),
                chapter_id.to_string(),
                ws_id.to_string(),
                format!("Tóm tắt cảnh {}", i),
                i,
            ],
        )
        .expect("seed scene");
    }

    (project_id, chapter_id, scene_ids)
}

// ============================================================================
// Acceptance Tests
// ============================================================================

/// `phase4_sprint4_deep1-F0`: Predecessor gate verification.
#[test]
fn test_phase4_sprint4_deep1_F0_predecessor_gates() {
    let baseline = "14bd13058f8c61a90c0fd8505d67989260d6542c";
    let p1 = "b013b9f9682ba92966af32e94f9c45d77473c25d"; // gate_phase4_sprint2_deep0
    let p2 = "02fc9b735457d47115e0f90063650dc00f322dc1"; // gate_phase2_sprint5_deep2

    let verify_commit = |sha: &str| common::verify_commit(sha);

    assert!(
        verify_commit(baseline),
        "Baseline commit ({}) must exist",
        baseline
    );
    assert!(
        verify_commit(p1),
        "Predecessor gate_phase4_sprint2_deep0 ({}) must exist",
        p1
    );
    assert!(
        verify_commit(p2),
        "Predecessor gate_phase2_sprint5_deep2 ({}) must exist",
        p2
    );
}

/// `T-SAVE-01`: Save chapter with clean locked scene, idempotent replay, payload mismatch,
/// dirty locked abort, and OCC mismatch.
#[test]
fn test_T_SAVE_01_chapter_save_comprehensive_matrix() {
    let temp_dir = TempDir::new().unwrap();
    let (_path, pool) = setup_test_db(&temp_dir);
    let service = WritingPersistenceService::new(pool.clone());

    let (project_id, chapter_id, scene_ids) = {
        let conn = pool.get().unwrap();
        seed_test_writing_project(&conn, true)
    };
    let s1 = scene_ids[0];
    let s2 = scene_ids[1];
    let s3 = scene_ids[2];

    // Lock Scene 2 (clean locked scene)
    let lock_res = exec_set_scene_lock(
        &service,
        &SetSceneLockReq {
            project_id,
            chapter_id,
            scene_id: s2,
            desired_locked: true,
        },
        CallerContext::UiAction,
    );
    assert!(lock_res.is_ok());
    assert!(lock_res.unwrap().is_locked);

    let client_save_id = "client-save-001".to_string();

    // ------------------------------------------------------------------------
    // (1) Scene 1 dirty (unlocked), Scene 2 clean (locked) -> call save_chapter(C_1)
    // ------------------------------------------------------------------------
    let dirty_items = vec![DirtySceneSaveItem {
        scene_id: s1,
        expected_version_number: 0,
        content: "Lăng Hàn mở mắt ra, phát hiện mình đã trùng sinh một vạn năm sau.".to_string(),
        scene_source: "USER".to_string(),
        summary: Some("Lăng Hàn tỉnh lại".to_string()),
    }];

    let save_req = SaveChapterRequest {
        project_id,
        chapter_id,
        client_save_id: client_save_id.clone(),
        dirty_scenes: dirty_items.clone(),
    };

    let save_resp = exec_save_chapter(&service, &save_req).expect("save_chapter should succeed");
    assert_eq!(save_resp.chapter_id, chapter_id);
    assert_eq!(save_resp.client_save_id, client_save_id);
    assert!(!save_resp.was_idempotent_replay);
    assert_eq!(save_resp.saved_scenes.len(), 1);
    assert_eq!(save_resp.saved_scenes[0].scene_id, s1);
    assert_eq!(save_resp.saved_scenes[0].new_version_number, 1);
    assert_eq!(save_resp.saved_scenes[0].word_count, 15);

    // Verify exactly 1 scene_versions created and 1 chapter_save_receipts created
    {
        let conn = pool.get().unwrap();
        let sv_count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM scene_versions WHERE parent_id = ?1",
                params![s1.to_string()],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(
            sv_count, 1,
            "Exactly 1 scene version should be created for s1"
        );

        let receipt_count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM chapter_save_receipts WHERE chapter_id = ?1 AND client_save_id = ?2",
                params![chapter_id.to_string(), client_save_id],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(receipt_count, 1, "Exactly 1 receipt should exist");
    }

    // ------------------------------------------------------------------------
    // (2) Re-invoke save_chapter with same clientSaveId and identical payload
    // ------------------------------------------------------------------------
    let replay_resp =
        exec_save_chapter(&service, &save_req).expect("idempotent replay should succeed");
    assert!(
        replay_resp.was_idempotent_replay,
        "was_idempotent_replay must be true on identical replay"
    );
    assert_eq!(replay_resp.saved_scenes.len(), 1);
    assert_eq!(replay_resp.saved_scenes[0].new_version_number, 1);

    // Verify NO second version created in DB
    {
        let conn = pool.get().unwrap();
        let sv_count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM scene_versions WHERE parent_id = ?1",
                params![s1.to_string()],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(
            sv_count, 1,
            "No duplicate scene version should be created on replay"
        );
    }

    // ------------------------------------------------------------------------
    // (3) Re-invoke save_chapter with same clientSaveId but different payload
    // ------------------------------------------------------------------------
    let mismatch_req = SaveChapterRequest {
        project_id,
        chapter_id,
        client_save_id: client_save_id.clone(),
        dirty_scenes: vec![DirtySceneSaveItem {
            scene_id: s1,
            expected_version_number: 1,
            content: "Nội dung hoàn toàn khác biệt gây xung đột hash".to_string(),
            scene_source: "USER".to_string(),
            summary: None,
        }],
    };
    let mismatch_err =
        exec_save_chapter(&service, &mismatch_req).expect_err("mismatch payload must fail");
    assert_eq!(mismatch_err.code, ErrorCode::IdempotencyKeyPayloadMismatch);

    // ------------------------------------------------------------------------
    // (4) Call save_chapter with Scene 2 (locked) in dirty_scenes -> rolls back all
    // ------------------------------------------------------------------------
    let locked_save_req = SaveChapterRequest {
        project_id,
        chapter_id,
        client_save_id: "client-save-locked-attempt".to_string(),
        dirty_scenes: vec![
            DirtySceneSaveItem {
                scene_id: s1,
                expected_version_number: 1,
                content: "Nội dung hợp lệ của s1".to_string(),
                scene_source: "USER".to_string(),
                summary: None,
            },
            DirtySceneSaveItem {
                scene_id: s2, // Locked!
                expected_version_number: 0,
                content: "Cố tình sửa cảnh đang bị khóa".to_string(),
                scene_source: "USER".to_string(),
                summary: None,
            },
        ],
    };
    let locked_err =
        exec_save_chapter(&service, &locked_save_req).expect_err("dirty locked scene must abort");
    assert_eq!(locked_err.code, ErrorCode::SceneLocked);

    // Verify atomic rollback: s1 was not updated to version 2!
    {
        let conn = pool.get().unwrap();
        let s1_max_ver: i32 = conn
            .query_row(
                "SELECT COALESCE(MAX(version_number), 0) FROM scene_versions WHERE parent_id = ?1",
                params![s1.to_string()],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(s1_max_ver, 1, "s1 must remain at version 1 after rollback");
    }

    // ------------------------------------------------------------------------
    // (5) Call save_chapter with Scene 3 with stale expected_version_number -> OCC mismatch
    // ------------------------------------------------------------------------
    let occ_req = SaveChapterRequest {
        project_id,
        chapter_id,
        client_save_id: "client-save-occ-attempt".to_string(),
        dirty_scenes: vec![DirtySceneSaveItem {
            scene_id: s3,
            expected_version_number: 42, // Stale! DB has 0
            content: "Nội dung cảnh 3 với OCC sai".to_string(),
            scene_source: "USER".to_string(),
            summary: None,
        }],
    };
    let occ_err = exec_save_chapter(&service, &occ_req).expect_err("stale OCC must abort");
    assert_eq!(occ_err.code, ErrorCode::OccVersionMismatch);
}

/// `P4S4-CLEAN-LOCKED-DOES-NOT-BLOCK`: Clean scenes that are locked do not block saving valid dirty scenes.
#[test]
fn test_P4S4_clean_locked_does_not_block() {
    let temp_dir = TempDir::new().unwrap();
    let (_path, pool) = setup_test_db(&temp_dir);
    let service = WritingPersistenceService::new(pool.clone());

    let (project_id, chapter_id, scene_ids) = {
        let conn = pool.get().unwrap();
        seed_test_writing_project(&conn, true)
    };
    let s1 = scene_ids[0];
    let s2 = scene_ids[1];
    let s3 = scene_ids[2];

    // Lock s1 and s3 (both clean)
    exec_set_scene_lock(
        &service,
        &SetSceneLockReq {
            project_id,
            chapter_id,
            scene_id: s1,
            desired_locked: true,
        },
        CallerContext::UiAction,
    )
    .unwrap();
    exec_set_scene_lock(
        &service,
        &SetSceneLockReq {
            project_id,
            chapter_id,
            scene_id: s3,
            desired_locked: true,
        },
        CallerContext::UiAction,
    )
    .unwrap();

    // Save only s2 (dirty and unlocked)
    let resp = exec_save_chapter(
        &service,
        &SaveChapterRequest {
            project_id,
            chapter_id,
            client_save_id: "save-s2-only".to_string(),
            dirty_scenes: vec![DirtySceneSaveItem {
                scene_id: s2,
                expected_version_number: 0,
                content: "Cảnh 2 được lưu thành công dù cảnh 1 và 3 đang khóa.".to_string(),
                scene_source: "AI".to_string(),
                summary: None,
            }],
        },
    )
    .expect("save s2 should succeed despite s1 and s3 being locked");

    assert_eq!(resp.saved_scenes.len(), 1);
    assert_eq!(resp.saved_scenes[0].scene_id, s2);
    assert_eq!(resp.saved_scenes[0].new_version_number, 1);
}

/// `P4S4-DIRTY-LOCKED-ABORTS-ALL`: When any dirty scene is locked, the entire Save transaction rolls back.
#[test]
fn test_P4S4_dirty_locked_aborts_all() {
    let temp_dir = TempDir::new().unwrap();
    let (_path, pool) = setup_test_db(&temp_dir);
    let service = WritingPersistenceService::new(pool.clone());

    let (project_id, chapter_id, scene_ids) = {
        let conn = pool.get().unwrap();
        seed_test_writing_project(&conn, true)
    };
    let s1 = scene_ids[0];
    let s2 = scene_ids[1];

    // Lock s2
    exec_set_scene_lock(
        &service,
        &SetSceneLockReq {
            project_id,
            chapter_id,
            scene_id: s2,
            desired_locked: true,
        },
        CallerContext::UiAction,
    )
    .unwrap();

    // Try to save both s1 (unlocked) and s2 (locked)
    let err = exec_save_chapter(
        &service,
        &SaveChapterRequest {
            project_id,
            chapter_id,
            client_save_id: "abort-attempt-1".to_string(),
            dirty_scenes: vec![
                DirtySceneSaveItem {
                    scene_id: s1,
                    expected_version_number: 0,
                    content: "Nội dung cảnh 1".to_string(),
                    scene_source: "USER".to_string(),
                    summary: None,
                },
                DirtySceneSaveItem {
                    scene_id: s2,
                    expected_version_number: 0,
                    content: "Nội dung cảnh 2 bị khóa".to_string(),
                    scene_source: "USER".to_string(),
                    summary: None,
                },
            ],
        },
    )
    .expect_err("must abort due to s2 being locked");

    assert_eq!(err.code, ErrorCode::SceneLocked);

    // Verify s1 has no versions and content is NULL
    let conn = pool.get().unwrap();
    let s1_ver_count: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM scene_versions WHERE parent_id = ?1",
            params![s1.to_string()],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(
        s1_ver_count, 0,
        "No versions should exist for s1 after rollback"
    );

    let s1_content: Option<String> = conn
        .query_row(
            "SELECT content FROM scenes WHERE id = ?1",
            params![s1.to_string()],
            |r| r.get(0),
        )
        .unwrap();
    assert!(s1_content.is_none(), "s1 content must remain NULL");
}

/// `P4S4-OCC-MISMATCH`: expected_version_number != MAX(version_number) returns OccVersionMismatch and rolls back.
#[test]
fn test_P4S4_occ_mismatch() {
    let temp_dir = TempDir::new().unwrap();
    let (_path, pool) = setup_test_db(&temp_dir);
    let service = WritingPersistenceService::new(pool.clone());

    let (project_id, chapter_id, scene_ids) = {
        let conn = pool.get().unwrap();
        seed_test_writing_project(&conn, true)
    };
    let s1 = scene_ids[0];

    // First save: creates version 1
    exec_save_chapter(
        &service,
        &SaveChapterRequest {
            project_id,
            chapter_id,
            client_save_id: "occ-save-v1".to_string(),
            dirty_scenes: vec![DirtySceneSaveItem {
                scene_id: s1,
                expected_version_number: 0,
                content: "Bản thảo ban đầu".to_string(),
                scene_source: "USER".to_string(),
                summary: None,
            }],
        },
    )
    .unwrap();

    // Second save with expected_version_number = 0 (stale, current is 1)
    let err = exec_save_chapter(
        &service,
        &SaveChapterRequest {
            project_id,
            chapter_id,
            client_save_id: "occ-save-v2-stale".to_string(),
            dirty_scenes: vec![DirtySceneSaveItem {
                scene_id: s1,
                expected_version_number: 0,
                content: "Bản sửa đổi trên nền cũ".to_string(),
                scene_source: "USER".to_string(),
                summary: None,
            }],
        },
    )
    .expect_err("must return OccVersionMismatch");

    assert_eq!(err.code, ErrorCode::OccVersionMismatch);
}

/// `P4S4-IDEMPOTENT-REPLAY`: Repeated invocation with same clientSaveId returns was_idempotent_replay = true.
#[test]
fn test_P4S4_idempotent_replay() {
    let temp_dir = TempDir::new().unwrap();
    let (_path, pool) = setup_test_db(&temp_dir);
    let service = WritingPersistenceService::new(pool.clone());

    let (project_id, chapter_id, scene_ids) = {
        let conn = pool.get().unwrap();
        seed_test_writing_project(&conn, true)
    };
    let s1 = scene_ids[0];

    let req = SaveChapterRequest {
        project_id,
        chapter_id,
        client_save_id: "idem-test-key".to_string(),
        dirty_scenes: vec![DirtySceneSaveItem {
            scene_id: s1,
            expected_version_number: 0,
            content: "Nội dung kiểm tra idempotency".to_string(),
            scene_source: "USER".to_string(),
            summary: None,
        }],
    };

    let r1 = exec_save_chapter(&service, &req).unwrap();
    assert!(!r1.was_idempotent_replay);

    let r2 = exec_save_chapter(&service, &req).unwrap();
    assert!(r2.was_idempotent_replay);
    assert_eq!(r1.chapter_id, r2.chapter_id);
    assert_eq!(r1.client_save_id, r2.client_save_id);
    assert_eq!(r1.saved_scenes, r2.saved_scenes);
}

/// `P4S4-IDEMPOTENT-PAYLOAD-MISMATCH`: Re-using clientSaveId with different content returns IdempotencyKeyPayloadMismatch.
#[test]
fn test_P4S4_idempotent_payload_mismatch() {
    let temp_dir = TempDir::new().unwrap();
    let (_path, pool) = setup_test_db(&temp_dir);
    let service = WritingPersistenceService::new(pool.clone());

    let (project_id, chapter_id, scene_ids) = {
        let conn = pool.get().unwrap();
        seed_test_writing_project(&conn, true)
    };
    let s1 = scene_ids[0];

    let req1 = SaveChapterRequest {
        project_id,
        chapter_id,
        client_save_id: "shared-save-id".to_string(),
        dirty_scenes: vec![DirtySceneSaveItem {
            scene_id: s1,
            expected_version_number: 0,
            content: "Nội dung bản A".to_string(),
            scene_source: "USER".to_string(),
            summary: None,
        }],
    };
    exec_save_chapter(&service, &req1).unwrap();

    let req2 = SaveChapterRequest {
        project_id,
        chapter_id,
        client_save_id: "shared-save-id".to_string(),
        dirty_scenes: vec![DirtySceneSaveItem {
            scene_id: s1,
            expected_version_number: 0,
            content: "Nội dung bản B (khác hoàn toàn)".to_string(),
            scene_source: "USER".to_string(),
            summary: None,
        }],
    };
    let err = exec_save_chapter(&service, &req2).expect_err("must return payload mismatch");
    assert_eq!(err.code, ErrorCode::IdempotencyKeyPayloadMismatch);
}

// ============================================================================
// IPC Command Level Tests
// ============================================================================

/// `IPC-CMD-029`: `get_chapter_writing_state`.
#[test]
fn test_IPC_CMD_029_get_chapter_writing_state() {
    let temp_dir = TempDir::new().unwrap();
    let (_path, pool) = setup_test_db(&temp_dir);
    let service = WritingPersistenceService::new(pool.clone());

    // 1. Success case: committed project
    let (project_id, chapter_id, scene_ids) = {
        let conn = pool.get().unwrap();
        seed_test_writing_project(&conn, true)
    };

    let state = exec_get_chapter_writing_state(
        &service,
        &GetChapterWritingStateReq {
            project_id,
            chapter_id,
        },
    )
    .expect("get_chapter_writing_state should succeed");

    assert_eq!(state.project_id, project_id);
    assert_eq!(state.chapter_id, chapter_id);
    assert_eq!(state.chapter_title, "Chương 1: Trùng Sinh");
    assert_eq!(state.position, 1);
    assert_eq!(state.scenes.len(), 3);

    // Verify ordering by position ASC and fields
    for (idx, scene) in state.scenes.iter().enumerate() {
        assert_eq!(scene.id, scene_ids[idx]);
        assert_eq!(scene.position, (idx + 1) as i32);
        assert_eq!(scene.head_version_number, 0);
        assert!(!scene.is_locked);
    }

    // 2. Gate check: uncommitted project plan returns ValidationFailed
    let (uncommitted_proj_id, uncommitted_chap_id, _) = {
        let conn = pool.get().unwrap();
        seed_test_writing_project(&conn, false)
    };

    let gate_err = exec_get_chapter_writing_state(
        &service,
        &GetChapterWritingStateReq {
            project_id: uncommitted_proj_id,
            chapter_id: uncommitted_chap_id,
        },
    )
    .expect_err("uncommitted plan must fail gate");

    assert_eq!(gate_err.code, ErrorCode::ValidationFailed);

    // 3. EntityNotFound: nonexistent chapter
    let not_found_err = exec_get_chapter_writing_state(
        &service,
        &GetChapterWritingStateReq {
            project_id,
            chapter_id: Uuid::now_v7(),
        },
    )
    .expect_err("nonexistent chapter must return EntityNotFound");

    assert_eq!(not_found_err.code, ErrorCode::EntityNotFound);
}

/// `IPC-CMD-030`: `set_scene_lock`.
#[test]
fn test_IPC_CMD_030_set_scene_lock() {
    let temp_dir = TempDir::new().unwrap();
    let (_path, pool) = setup_test_db(&temp_dir);
    let service = WritingPersistenceService::new(pool.clone());

    let (project_id, chapter_id, scene_ids) = {
        let conn = pool.get().unwrap();
        seed_test_writing_project(&conn, true)
    };
    let s1 = scene_ids[0];

    // 1. Rejection of AI caller
    let ai_err = exec_set_scene_lock(
        &service,
        &SetSceneLockReq {
            project_id,
            chapter_id,
            scene_id: s1,
            desired_locked: true,
        },
        CallerContext::Ai,
    )
    .expect_err("AI caller must be rejected");
    assert_eq!(ai_err.code, ErrorCode::UnauthorizedCaller);

    // 2. Successful lock by UI operator
    let lock_res = exec_set_scene_lock(
        &service,
        &SetSceneLockReq {
            project_id,
            chapter_id,
            scene_id: s1,
            desired_locked: true,
        },
        CallerContext::UiAction,
    )
    .expect("UI lock should succeed");

    assert_eq!(lock_res.scene_id, s1);
    assert!(lock_res.is_locked);
    assert!(lock_res.invalidated_generations.is_empty());

    // Verify in DB that is_locked is persisted and no scene_versions created
    {
        let conn = pool.get().unwrap();
        let is_locked: i32 = conn
            .query_row(
                "SELECT is_locked FROM scenes WHERE id = ?1",
                params![s1.to_string()],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(is_locked, 1);

        let sv_count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM scene_versions WHERE parent_id = ?1",
                params![s1.to_string()],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(sv_count, 0, "Lock toggle must NOT create scene_versions");
    }

    // 3. Successful unlock
    let unlock_res = exec_set_scene_lock(
        &service,
        &SetSceneLockReq {
            project_id,
            chapter_id,
            scene_id: s1,
            desired_locked: false,
        },
        CallerContext::UiAction,
    )
    .expect("UI unlock should succeed");
    assert!(!unlock_res.is_locked);

    // 4. Nonexistent scene returns EntityNotFound
    let not_found_err = exec_set_scene_lock(
        &service,
        &SetSceneLockReq {
            project_id,
            chapter_id,
            scene_id: Uuid::now_v7(),
            desired_locked: true,
        },
        CallerContext::UiAction,
    )
    .expect_err("nonexistent scene must return EntityNotFound");
    assert_eq!(not_found_err.code, ErrorCode::EntityNotFound);
}

/// `IPC-CMD-031`: `set_chapter_scenes_lock_bulk`.
#[test]
fn test_IPC_CMD_031_set_chapter_scenes_lock_bulk() {
    let temp_dir = TempDir::new().unwrap();
    let (_path, pool) = setup_test_db(&temp_dir);
    let service = WritingPersistenceService::new(pool.clone());

    let (project_id, chapter_id, scene_ids) = {
        let conn = pool.get().unwrap();
        seed_test_writing_project(&conn, true)
    };

    // 1. Rejection of AI caller
    let ai_err = exec_set_chapter_scenes_lock_bulk(
        &service,
        &SetChapterScenesLockBulkReq {
            project_id,
            chapter_id,
            is_locked: true,
        },
        CallerContext::Ai,
    )
    .expect_err("AI caller must be rejected");
    assert_eq!(ai_err.code, ErrorCode::UnauthorizedCaller);

    // 2. Lock All via UiAction
    let bulk_lock = exec_set_chapter_scenes_lock_bulk(
        &service,
        &SetChapterScenesLockBulkReq {
            project_id,
            chapter_id,
            is_locked: true,
        },
        CallerContext::UiAction,
    )
    .expect("bulk lock should succeed");

    assert_eq!(bulk_lock.chapter_id, chapter_id);
    assert_eq!(bulk_lock.scenes.len(), 3);
    for scene in &bulk_lock.scenes {
        assert!(scene.is_locked);
    }

    // Verify all 3 in DB
    {
        let conn = pool.get().unwrap();
        for sid in &scene_ids {
            let is_locked: i32 = conn
                .query_row(
                    "SELECT is_locked FROM scenes WHERE id = ?1",
                    params![sid.to_string()],
                    |r| r.get(0),
                )
                .unwrap();
            assert_eq!(is_locked, 1);
        }
    }

    // 3. Unlock All via UiAction
    let bulk_unlock = exec_set_chapter_scenes_lock_bulk(
        &service,
        &SetChapterScenesLockBulkReq {
            project_id,
            chapter_id,
            is_locked: false,
        },
        CallerContext::UiAction,
    )
    .expect("bulk unlock should succeed");

    for scene in &bulk_unlock.scenes {
        assert!(!scene.is_locked);
    }

    // 4. Nonexistent chapter returns EntityNotFound
    let not_found_err = exec_set_chapter_scenes_lock_bulk(
        &service,
        &SetChapterScenesLockBulkReq {
            project_id,
            chapter_id: Uuid::now_v7(),
            is_locked: true,
        },
        CallerContext::UiAction,
    )
    .expect_err("nonexistent chapter must return EntityNotFound");
    assert_eq!(not_found_err.code, ErrorCode::EntityNotFound);
}

/// `IPC-CMD-035`: `list_scene_versions`.
#[test]
fn test_IPC_CMD_035_list_scene_versions() {
    let temp_dir = TempDir::new().unwrap();
    let (_path, pool) = setup_test_db(&temp_dir);
    let service = WritingPersistenceService::new(pool.clone());

    let (project_id, chapter_id, scene_ids) = {
        let conn = pool.get().unwrap();
        seed_test_writing_project(&conn, true)
    };
    let s1 = scene_ids[0];

    // Initial check: 0 versions
    let list_v0 = exec_list_scene_versions(
        &service,
        &ListSceneVersionsReq {
            project_id,
            chapter_id,
            scene_id: s1,
        },
    )
    .expect("list scene versions should succeed");
    assert_eq!(list_v0.head_version_number, 0);
    assert!(list_v0.versions.is_empty());

    // Save version 1
    exec_save_chapter(
        &service,
        &SaveChapterRequest {
            project_id,
            chapter_id,
            client_save_id: "ver-1-save".to_string(),
            dirty_scenes: vec![DirtySceneSaveItem {
                scene_id: s1,
                expected_version_number: 0,
                content: "Nội dung phiên bản 1".to_string(),
                scene_source: "USER".to_string(),
                summary: Some("Bản 1".to_string()),
            }],
        },
    )
    .unwrap();

    // Save version 2
    exec_save_chapter(
        &service,
        &SaveChapterRequest {
            project_id,
            chapter_id,
            client_save_id: "ver-2-save".to_string(),
            dirty_scenes: vec![DirtySceneSaveItem {
                scene_id: s1,
                expected_version_number: 1,
                content: "Nội dung phiên bản 2 cập nhật chi tiết hơn".to_string(),
                scene_source: "AI".to_string(),
                summary: Some("Bản 2".to_string()),
            }],
        },
    )
    .unwrap();

    // List versions: must return [v2, v1] in DESC order
    let list_res = exec_list_scene_versions(
        &service,
        &ListSceneVersionsReq {
            project_id,
            chapter_id,
            scene_id: s1,
        },
    )
    .expect("list scene versions should succeed");

    assert_eq!(list_res.scene_id, s1);
    assert_eq!(list_res.head_version_number, 2);
    assert_eq!(list_res.versions.len(), 2);

    assert_eq!(list_res.versions[0].version_number, 2);
    assert_eq!(
        list_res.versions[0].content,
        "Nội dung phiên bản 2 cập nhật chi tiết hơn"
    );
    assert_eq!(list_res.versions[0].scene_source, "AI");
    assert_eq!(list_res.versions[0].summary.as_deref(), Some("Bản 2"));

    assert_eq!(list_res.versions[1].version_number, 1);
    assert_eq!(list_res.versions[1].content, "Nội dung phiên bản 1");
    assert_eq!(list_res.versions[1].scene_source, "USER");
    assert_eq!(list_res.versions[1].summary.as_deref(), Some("Bản 1"));

    // Nonexistent scene returns EntityNotFound
    let not_found_err = exec_list_scene_versions(
        &service,
        &ListSceneVersionsReq {
            project_id,
            chapter_id,
            scene_id: Uuid::now_v7(),
        },
    )
    .expect_err("nonexistent scene must return EntityNotFound");
    assert_eq!(not_found_err.code, ErrorCode::EntityNotFound);
}

/// `IPC-CMD-036`: `save_chapter` with empty dirty scenes.
#[test]
fn test_IPC_CMD_036_save_chapter_empty_dirty_scenes() {
    let temp_dir = TempDir::new().unwrap();
    let (_path, pool) = setup_test_db(&temp_dir);
    let service = WritingPersistenceService::new(pool.clone());

    let (project_id, chapter_id, _) = {
        let conn = pool.get().unwrap();
        seed_test_writing_project(&conn, true)
    };

    let resp = exec_save_chapter(
        &service,
        &SaveChapterRequest {
            project_id,
            chapter_id,
            client_save_id: "empty-save-1".to_string(),
            dirty_scenes: vec![],
        },
    )
    .expect("empty dirty scenes should succeed without versions");

    assert_eq!(resp.chapter_id, chapter_id);
    assert!(resp.saved_scenes.is_empty());
    assert_eq!(resp.chapter_scenes.len(), 3);
    assert!(!resp.was_idempotent_replay);
}

/// `IPC-CMD-037`: `get_chapter_save_receipt`.
#[test]
fn test_IPC_CMD_037_get_chapter_save_receipt() {
    let temp_dir = TempDir::new().unwrap();
    let (_path, pool) = setup_test_db(&temp_dir);
    let service = WritingPersistenceService::new(pool.clone());

    let (project_id, chapter_id, scene_ids) = {
        let conn = pool.get().unwrap();
        seed_test_writing_project(&conn, true)
    };
    let s1 = scene_ids[0];

    // 1. Query receipt before saving -> found = false
    let lookup_pre = exec_get_chapter_save_receipt(
        &service,
        &GetChapterSaveReceiptReq {
            chapter_id,
            client_save_id: "receipt-lookup-1".to_string(),
        },
    )
    .expect("receipt lookup should succeed");
    assert!(!lookup_pre.found);
    assert!(lookup_pre.receipt.is_none());
    assert_eq!(lookup_pre.current_chapter_scenes.len(), 3);

    // 2. Perform Save
    exec_save_chapter(
        &service,
        &SaveChapterRequest {
            project_id,
            chapter_id,
            client_save_id: "receipt-lookup-1".to_string(),
            dirty_scenes: vec![DirtySceneSaveItem {
                scene_id: s1,
                expected_version_number: 0,
                content: "Nội dung lưu biên nhận".to_string(),
                scene_source: "RESTORE".to_string(),
                summary: None,
            }],
        },
    )
    .unwrap();

    // 3. Query receipt after saving -> found = true
    let lookup_post = exec_get_chapter_save_receipt(
        &service,
        &GetChapterSaveReceiptReq {
            chapter_id,
            client_save_id: "receipt-lookup-1".to_string(),
        },
    )
    .expect("receipt lookup should succeed");
    assert!(lookup_post.found);
    let receipt = lookup_post.receipt.expect("receipt must be present");
    assert_eq!(receipt.chapter_id, chapter_id);
    assert_eq!(receipt.client_save_id, "receipt-lookup-1");
    assert_eq!(receipt.saved_scenes.len(), 1);
    assert_eq!(receipt.saved_scenes[0].scene_id, s1);
    assert_eq!(receipt.saved_scenes[0].new_version_number, 1);

    // 4. Nonexistent chapter returns EntityNotFound
    let not_found_err = exec_get_chapter_save_receipt(
        &service,
        &GetChapterSaveReceiptReq {
            chapter_id: Uuid::now_v7(),
            client_save_id: "any-save-id".to_string(),
        },
    )
    .expect_err("nonexistent chapter must return EntityNotFound");
    assert_eq!(not_found_err.code, ErrorCode::EntityNotFound);
}
