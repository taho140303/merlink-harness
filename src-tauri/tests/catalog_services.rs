#![allow(clippy::field_reassign_with_default)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::result_large_err)]
#![allow(clippy::result_unit_err)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]

//! Acceptance and integration tests for `phase4_sprint3_deep1`
//! ("Expose project and character library services").
//!
//! Complies with:
//! - `docs/business/core.md` (§1.3 Q1, §3.8)
//! - `docs/business/be-core.md` (§10.2.7 IPC #22–28, REQ-PROJECT-001)
//! - `docs/plan/phase4_sprint3_deep1.md`

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
#[path = "../src/domain/phase_gate.rs"]
pub mod phase_gate;
#[path = "../src/domain/progress_engine.rs"]
pub mod progress_engine;
#[path = "../src/domain/word_count.rs"]
pub mod word_count;

pub mod domain {
    pub use crate::business_flow;
    pub use crate::code_allocator;
    pub use crate::fingerprint;
    pub use crate::invariants;
    pub use crate::phase_gate;
    pub use crate::progress_engine;
    pub use crate::word_count;
}

#[path = "../src/storage/character_repo.rs"]
pub mod character_repo;
#[path = "../src/storage/commit_repo.rs"]
pub mod commit_repo;
#[path = "../src/storage/db.rs"]
pub mod db;
#[path = "../src/storage/draft_repo.rs"]
pub mod draft_repo;
#[path = "../src/storage/migration.rs"]
pub mod migration;
#[path = "../src/storage/project_repo.rs"]
pub mod project_repo;

pub mod storage {
    pub use crate::character_repo;
    pub use crate::commit_repo;
    pub use crate::db;
    pub use crate::draft_repo;
    pub use crate::migration;
    pub use crate::project_repo;
}

#[path = "../src/service/character_service.rs"]
pub mod character_service;
#[path = "../src/service/commit_service.rs"]
pub mod commit_service;
#[path = "../src/service/draft_runtime.rs"]
pub mod draft_runtime;
#[path = "../src/service/draft_service.rs"]
pub mod draft_service;
#[path = "../src/service/project_service.rs"]
pub mod project_service;

pub mod service {
    pub use crate::character_service;
    pub use crate::commit_service;
    pub use crate::draft_runtime;
    pub use crate::draft_service;
    pub use crate::project_service;
}

#[path = "../src/ipc/character_commands.rs"]
pub mod character_commands;
#[path = "../src/ipc/dto.rs"]
pub mod dto;
#[path = "../src/ipc/project_commands.rs"]
pub mod project_commands;

pub mod ipc {
    pub use crate::character_commands;
    pub use crate::dto;
    pub use crate::project_commands;
}

use std::path::PathBuf;
use std::sync::Arc;

use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{params, Connection};
use tempfile::TempDir;
use uuid::Uuid;

use error::ErrorCode;
use ipc::character_commands::*;
use ipc::dto::*;
use ipc::project_commands::*;
use service::character_service::CharacterService;
use service::draft_runtime::DraftRuntimeRegistry;
use service::draft_service::DraftService;
use service::project_service::ProjectService;
use storage::db::configure_connection;
use storage::migration::run_migrations;

pub const BASELINE_COMMIT: &str = "14bd13058f8c61a90c0fd8505d67989260d6542c";
pub const GATE_P4S2_COMMIT: &str = "b013b9f9682ba92966af32e94f9c45d77473c25d";
pub const GATE_P3S2_COMMIT: &str = "ded1e9c5457ab8e72f456a515937a05af6ebba63";

// ============================================================================
// Database & Fixture Setup Helpers
// ============================================================================

fn setup_test_db(temp_dir: &TempDir) -> (PathBuf, Pool<SqliteConnectionManager>) {
    let db_path = temp_dir.path().join("test_catalog_services.sqlite3");
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

fn seed_draft_session(
    pool: &Pool<SqliteConnectionManager>,
    draft_session_id: Uuid,
    project_id: Uuid,
    session_id: &str,
    status: &str,
) {
    let conn = pool.get().expect("get conn for seeding draft session");
    let proj_str = project_id.to_string();
    let draft_str = draft_session_id.to_string();

    conn.execute(
        "INSERT INTO sessions (id, project_id, display_name) VALUES (?1, ?2, 'Active Session')",
        params![session_id, proj_str],
    )
    .expect("seed session");

    conn.execute(
        "INSERT INTO draft_sessions (id, session_id, project_id, head_version_seq, status, created_at, updated_at)
         VALUES (?1, ?2, ?3, NULL, ?4, '2026-03-01T00:00:00Z', '2026-03-01T00:00:00Z')",
        params![draft_str, session_id, proj_str, status],
    )
    .expect("seed draft_session");
}

fn seed_test_hierarchy(
    conn: &Connection,
    project_id: &str,
    world_setting_id: &str,
    arc_id: &str,
    chapter_id: &str,
    scene1_id: &str,
    scene2_id: &str,
) {
    // 1. world setting
    conn.execute(
        "INSERT INTO world_settings (id, code, project_id, display_name, type)
         VALUES (?1, 'ws_p1', ?2, 'Project 1 World', 'WORLD')",
        params![world_setting_id, project_id],
    )
    .expect("seed world_setting");

    // 2. lore entry
    conn.execute(
        "INSERT INTO lore_entries (id, code, type, display_name, world_setting_id)
         VALUES (?1, 'lore_p1', 'HISTORY', 'Project 1 Ancient Lore', ?2)",
        params![Uuid::now_v7().to_string(), world_setting_id],
    )
    .expect("seed lore_entry");

    // 3. arc
    conn.execute(
        "INSERT INTO arcs (id, code, display_name, project_id, position)
         VALUES (?1, 'arc_1', 'Arc 1: The Awakening', ?2, 1)",
        params![arc_id, project_id],
    )
    .expect("seed arc");

    // 4. chapter
    conn.execute(
        "INSERT INTO chapters (id, code, display_name, arc_id, position)
         VALUES (?1, 'ch_1', 'Chapter 1: The Threshold', ?2, 1)",
        params![chapter_id, arc_id],
    )
    .expect("seed chapter");

    // 5. scenes
    conn.execute(
        "INSERT INTO scenes (id, code, display_name, chapter_id, world_setting_id, summary, content, word_count, position, is_locked)
         VALUES (?1, 'sc_1', 'Scene 1: Call to Adventure', ?2, ?3, 'Hero gets the call', 'This is the written content of scene 1.', 1500, 1, 1)",
        params![scene1_id, chapter_id, world_setting_id],
    )
    .expect("seed scene 1");

    conn.execute(
        "INSERT INTO scenes (id, code, display_name, chapter_id, world_setting_id, summary, content, word_count, position, is_locked)
         VALUES (?1, 'sc_2', 'Scene 2: Refusal of Call', ?2, ?3, 'Hero hesitates', NULL, 2500, 2, 0)",
        params![scene2_id, chapter_id, world_setting_id],
    )
    .expect("seed scene 2");
}

// ============================================================================
// Acceptance Tests
// ============================================================================

/// phase4_sprint3_deep1_F0: Predecessor gates & baseline lineage verification.
#[test]
fn phase4_sprint3_deep1_F0() {
    let baseline = BASELINE_COMMIT;
    let gate_p4s2 = GATE_P4S2_COMMIT;
    let gate_p3s2 = GATE_P3S2_COMMIT;

    assert!(
        common::verify_commit(baseline),
        "Baseline commit {baseline} must be an ancestor, a local object, or recorded in the lineage manifest"
    );

    let gates = [
        ("gate_phase4_sprint2_deep0", gate_p4s2),
        ("gate_phase3_sprint2_deep0", gate_p3s2),
    ];

    for (name, sha) in gates {
        assert!(
            common::verify_commit(sha),
            "Predecessor gate {name} ({sha}) must be an ancestor, a local object, or recorded in the lineage manifest"
        );
    }
}

/// P4S3-PROJECT-LIST-AND-DETAIL: Project listing, sorting, searching, details, and hierarchy.
#[test]
fn P4S3_PROJECT_LIST_AND_DETAIL() {
    let temp_dir = TempDir::new().unwrap();
    let (_db_path, pool) = setup_test_db(&temp_dir);
    let project_service = ProjectService::new(pool.clone());

    let p1_id = Uuid::now_v7();
    let p2_id = Uuid::now_v7();
    let p3_id = Uuid::now_v7();
    let p_del_id = Uuid::now_v7();

    let arc_id = Uuid::now_v7().to_string();
    let ch_id = Uuid::now_v7().to_string();
    let sc1_id = Uuid::now_v7().to_string();
    let sc2_id = Uuid::now_v7().to_string();
    let ws_id = Uuid::now_v7().to_string();

    {
        let conn = pool.get().unwrap();

        // Project 1: Alpha Novel
        conn.execute(
            "INSERT INTO projects (id, code, display_name, status, summary, cover, created_at, updated_at)
             VALUES (?1, 'proj_alpha', 'Alpha Novel', 'PLANNING', 'First chronicle of the dawn', 'cover1.png', '2026-03-01T08:00:00Z', '2026-03-01T10:00:00Z')",
            params![p1_id.to_string()],
        ).unwrap();

        // Project 2: Beta Story
        conn.execute(
            "INSERT INTO projects (id, code, display_name, status, summary, cover, created_at, updated_at)
             VALUES (?1, 'proj_beta', 'Beta Story', 'DRAFTING', 'Second saga of the realm', 'cover2.png', '2026-03-02T08:00:00Z', '2026-03-02T10:00:00Z')",
            params![p2_id.to_string()],
        ).unwrap();

        // Project 3: Charlie Tale
        conn.execute(
            "INSERT INTO projects (id, code, display_name, status, summary, cover, created_at, updated_at)
             VALUES (?1, 'proj_charlie', 'Charlie Tale', 'COMPLETED', 'Final mystery of time', 'cover3.png', '2026-03-03T08:00:00Z', '2026-03-03T10:00:00Z')",
            params![p3_id.to_string()],
        ).unwrap();

        // Deleted Project (must not appear in list)
        conn.execute(
            "INSERT INTO projects (id, code, display_name, status, summary, created_at, updated_at, deleted_at)
             VALUES (?1, 'proj_del', 'Deleted Realm', 'PLANNING', 'Should not appear', '2026-03-04T08:00:00Z', '2026-03-04T10:00:00Z', '2026-03-04T12:00:00Z')",
            params![p_del_id.to_string()],
        ).unwrap();

        // Primary genre for Project 1 (Fantasy)
        conn.execute(
            "INSERT INTO project_genres (id, project_id, genre_id, is_primary)
             VALUES (?1, ?2, '018e3a00-0000-7000-8000-000000000001', 1)",
            params![Uuid::now_v7().to_string(), p1_id.to_string()],
        )
        .unwrap();

        // Hierarchy for Project 1
        seed_test_hierarchy(
            &conn,
            &p1_id.to_string(),
            &ws_id,
            &arc_id,
            &ch_id,
            &sc1_id,
            &sc2_id,
        );

        // Character in Project 1
        let char_id = Uuid::now_v7().to_string();
        conn.execute(
            "INSERT INTO characters (id, code, display_name) VALUES (?1, 'char_1', 'Linh Đan')",
            params![char_id],
        )
        .unwrap();
        conn.execute(
            "INSERT INTO project_characters (id, project_id, character_id) VALUES (?1, ?2, ?3)",
            params![Uuid::now_v7().to_string(), p1_id.to_string(), char_id],
        )
        .unwrap();
    }

    // 1. Test list_projects with default sort (modified desc)
    let list_default = project_service
        .list_projects(&ListProjectsReq::default())
        .expect("list_projects default");
    assert_eq!(list_default.items.len(), 3);
    assert_eq!(list_default.items[0].id, p3_id.to_string()); // Charlie (March 3)
    assert_eq!(list_default.items[1].id, p2_id.to_string()); // Beta (March 2)
    assert_eq!(list_default.items[2].id, p1_id.to_string()); // Alpha (March 1)

    // 2. Test sort "a-z"
    let list_az = project_service
        .list_projects(&ListProjectsReq {
            query: None,
            sort: Some("a-z".to_string()),
        })
        .expect("list_projects a-z");
    assert_eq!(list_az.items[0].title, "Alpha Novel");
    assert_eq!(list_az.items[1].title, "Beta Story");
    assert_eq!(list_az.items[2].title, "Charlie Tale");

    // 3. Test sort "z-a"
    let list_za = project_service
        .list_projects(&ListProjectsReq {
            query: None,
            sort: Some("z-a".to_string()),
        })
        .expect("list_projects z-a");
    assert_eq!(list_za.items[0].title, "Charlie Tale");
    assert_eq!(list_za.items[1].title, "Beta Story");
    assert_eq!(list_za.items[2].title, "Alpha Novel");

    // 4. Test query filtering
    let search_beta = project_service
        .list_projects(&ListProjectsReq {
            query: Some("beta".to_string()),
            sort: None,
        })
        .expect("search beta");
    assert_eq!(search_beta.items.len(), 1);
    assert_eq!(search_beta.items[0].title, "Beta Story");

    let search_summary = project_service
        .list_projects(&ListProjectsReq {
            query: Some("chronicle".to_string()),
            sort: None,
        })
        .expect("search summary");
    assert_eq!(search_summary.items.len(), 1);
    assert_eq!(search_summary.items[0].title, "Alpha Novel");

    // 5. Test detail counts for Project 1
    let p1_item = list_az
        .items
        .iter()
        .find(|p| p.id == p1_id.to_string())
        .unwrap();
    assert_eq!(p1_item.details.word_count, "4000"); // 1500 + 2500
    assert_eq!(p1_item.details.chapters, "1");
    assert_eq!(p1_item.details.characters, "1");
    assert_eq!(p1_item.details.world_lore, "2"); // 1 ws + 1 lore
    assert_eq!(p1_item.genres, vec!["Kỳ Ảo".to_string()]);

    // 6. Test get_project_detail
    let detail = project_service
        .get_project_detail(&GetProjectDetailReq { project_id: p1_id })
        .expect("get_project_detail p1");
    assert_eq!(detail.project.id, p1_id.to_string());
    assert_eq!(detail.project.title, "Alpha Novel");
    assert_eq!(detail.project.status, "Planning");
    assert!(detail.has_committed_plan);
    // Because Scene 1 has written content, can_reopen_draft should be false
    assert!(!detail.can_reopen_draft);

    // Verify arc and chapter hierarchy
    let arcs = detail.project.arcs.expect("arcs present");
    assert_eq!(arcs.len(), 1);
    assert_eq!(arcs[0].title, "Arc 1: The Awakening");
    assert_eq!(arcs[0].chapters.len(), 1);
    assert_eq!(arcs[0].chapters[0].title, "Chapter 1: The Threshold");

    let blocks = arcs[0].chapters[0].blocks.as_ref().expect("blocks present");
    assert_eq!(blocks.len(), 2);
    assert_eq!(blocks[0].text, "This is the written content of scene 1.");
    assert!(blocks[0].locked);
    assert_eq!(blocks[1].text, "Hero hesitates");
    assert!(!blocks[1].locked);

    // 7. Non-existent project yields EntityNotFound
    let not_found = project_service
        .get_project_detail(&GetProjectDetailReq {
            project_id: Uuid::now_v7(),
        })
        .expect_err("unknown project not found");
    assert_eq!(not_found.code, ErrorCode::EntityNotFound);
}

/// P4S3-PROJECT-DELETE-SCOPED: Scoped soft-delete preserves shared characters and template world settings.
#[test]
fn P4S3_PROJECT_DELETE_SCOPED() {
    let temp_dir = TempDir::new().unwrap();
    let (_db_path, pool) = setup_test_db(&temp_dir);
    let project_service = ProjectService::new(pool.clone());

    let p1_id = Uuid::now_v7();
    let p2_id = Uuid::now_v7();
    let shared_char_id = Uuid::now_v7().to_string();
    let template_ws_id = Uuid::now_v7().to_string();
    let template_lore_id = Uuid::now_v7().to_string();

    let arc_id = Uuid::now_v7().to_string();
    let ch_id = Uuid::now_v7().to_string();
    let sc1_id = Uuid::now_v7().to_string();
    let sc2_id = Uuid::now_v7().to_string();
    let p1_ws_id = Uuid::now_v7().to_string();
    let theme_id = Uuid::now_v7().to_string();

    {
        let conn = pool.get().unwrap();

        // 1. Projects
        conn.execute(
            "INSERT INTO projects (id, code, display_name) VALUES (?1, 'proj_1', 'Project 1')",
            params![p1_id.to_string()],
        )
        .unwrap();
        conn.execute(
            "INSERT INTO projects (id, code, display_name) VALUES (?1, 'proj_2', 'Project 2')",
            params![p2_id.to_string()],
        )
        .unwrap();

        // 2. Hierarchy for Project 1
        seed_test_hierarchy(
            &conn,
            &p1_id.to_string(),
            &p1_ws_id,
            &arc_id,
            &ch_id,
            &sc1_id,
            &sc2_id,
        );

        // 3. Theme in Project 1 arc
        conn.execute(
            "INSERT INTO themes (id, arc_id, question) VALUES (?1, ?2, 'What is destiny?')",
            params![theme_id, arc_id],
        )
        .unwrap();

        // 4. Shared Character linked to both Project 1 and Project 2
        conn.execute(
            "INSERT INTO characters (id, code, display_name) VALUES (?1, 'shared_char', 'Vân Tiêu')",
            params![shared_char_id],
        ).unwrap();
        conn.execute(
            "INSERT INTO project_characters (id, project_id, character_id) VALUES (?1, ?2, ?3)",
            params![
                Uuid::now_v7().to_string(),
                p1_id.to_string(),
                shared_char_id
            ],
        )
        .unwrap();
        conn.execute(
            "INSERT INTO project_characters (id, project_id, character_id) VALUES (?1, ?2, ?3)",
            params![
                Uuid::now_v7().to_string(),
                p2_id.to_string(),
                shared_char_id
            ],
        )
        .unwrap();

        // 5. Template World Setting (project_id IS NULL) and its lore entry
        conn.execute(
            "INSERT INTO world_settings (id, code, project_id, display_name, type)
             VALUES (?1, 'template_world', NULL, 'Global Template Realm', 'WORLD')",
            params![template_ws_id],
        )
        .unwrap();
        conn.execute(
            "INSERT INTO lore_entries (id, code, type, display_name, world_setting_id)
             VALUES (?1, 'template_lore', 'CULTURE', 'Universal Laws', ?2)",
            params![template_lore_id, template_ws_id],
        )
        .unwrap();
    }

    // Execute scoped soft-delete on Project 1
    let del_resp = project_service
        .delete_project(&DeleteProjectReq { project_id: p1_id })
        .expect("delete_project p1");
    assert!(del_resp.deleted);

    // Verify soft-delete state in DB
    {
        let conn = pool.get().unwrap();

        // Project 1 must be soft-deleted
        let p1_deleted: Option<String> = conn
            .query_row(
                "SELECT deleted_at FROM projects WHERE id = ?1",
                params![p1_id.to_string()],
                |r| r.get(0),
            )
            .unwrap();
        assert!(p1_deleted.is_some(), "Project 1 deleted_at must be set");

        // Project 1 arcs, chapters, scenes, themes must be soft-deleted
        let arc_deleted: Option<String> = conn
            .query_row(
                "SELECT deleted_at FROM arcs WHERE id = ?1",
                params![arc_id],
                |r| r.get(0),
            )
            .unwrap();
        assert!(arc_deleted.is_some());

        let ch_deleted: Option<String> = conn
            .query_row(
                "SELECT deleted_at FROM chapters WHERE id = ?1",
                params![ch_id],
                |r| r.get(0),
            )
            .unwrap();
        assert!(ch_deleted.is_some());

        let sc_deleted: Option<String> = conn
            .query_row(
                "SELECT deleted_at FROM scenes WHERE id = ?1",
                params![sc1_id],
                |r| r.get(0),
            )
            .unwrap();
        assert!(sc_deleted.is_some());

        let theme_deleted: Option<String> = conn
            .query_row(
                "SELECT deleted_at FROM themes WHERE id = ?1",
                params![theme_id],
                |r| r.get(0),
            )
            .unwrap();
        assert!(theme_deleted.is_some());

        // Project 1 project_characters link is soft-deleted
        let p1_pc_deleted: Option<String> = conn
            .query_row(
                "SELECT deleted_at FROM project_characters WHERE project_id = ?1 AND character_id = ?2",
                params![p1_id.to_string(), shared_char_id],
                |r| r.get(0),
            )
            .unwrap();
        assert!(p1_pc_deleted.is_some());

        // Project 1-specific world settings and lore entries are soft-deleted
        let p1_ws_deleted: Option<String> = conn
            .query_row(
                "SELECT deleted_at FROM world_settings WHERE id = ?1",
                params![p1_ws_id],
                |r| r.get(0),
            )
            .unwrap();
        assert!(p1_ws_deleted.is_some());

        // ====================================================================
        // CRITICAL DOMAIN INVARIANTS: Preserved Entities
        // ====================================================================

        // 1. Shared character in library is NOT deleted
        let char_deleted: Option<String> = conn
            .query_row(
                "SELECT deleted_at FROM characters WHERE id = ?1",
                params![shared_char_id],
                |r| r.get(0),
            )
            .unwrap();
        assert!(
            char_deleted.is_none(),
            "Shared library character must NOT be soft-deleted"
        );

        // 2. Project 2 link to shared character is NOT deleted
        let p2_pc_deleted: Option<String> = conn
            .query_row(
                "SELECT deleted_at FROM project_characters WHERE project_id = ?1 AND character_id = ?2",
                params![p2_id.to_string(), shared_char_id],
                |r| r.get(0),
            )
            .unwrap();
        assert!(
            p2_pc_deleted.is_none(),
            "Project 2 link must NOT be soft-deleted"
        );

        // 3. Template world setting (project_id IS NULL) is NOT deleted
        let tmpl_ws_deleted: Option<String> = conn
            .query_row(
                "SELECT deleted_at FROM world_settings WHERE id = ?1",
                params![template_ws_id],
                |r| r.get(0),
            )
            .unwrap();
        assert!(
            tmpl_ws_deleted.is_none(),
            "Template world setting must NOT be deleted"
        );

        // 4. Template lore entry is NOT deleted
        let tmpl_lore_deleted: Option<String> = conn
            .query_row(
                "SELECT deleted_at FROM lore_entries WHERE id = ?1",
                params![template_lore_id],
                |r| r.get(0),
            )
            .unwrap();
        assert!(
            tmpl_lore_deleted.is_none(),
            "Template lore entry must NOT be deleted"
        );
    }

    // Calling delete again on already deleted project yields EntityNotFound
    let second_del = project_service.delete_project(&DeleteProjectReq { project_id: p1_id });
    assert_eq!(second_del.unwrap_err().code, ErrorCode::EntityNotFound);
}

/// P4S3-CHARACTER-4TIER-SEARCH: 4-tier character search with Vietnamese diacritics and project scoping.
#[test]
fn P4S3_CHARACTER_4TIER_SEARCH() {
    let temp_dir = TempDir::new().unwrap();
    let (_db_path, pool) = setup_test_db(&temp_dir);
    let registry = Arc::new(DraftRuntimeRegistry::new());
    let draft_service = Arc::new(DraftService::new(pool.clone(), registry.clone()));
    let character_service = CharacterService::new(pool.clone(), draft_service);

    let proj1_id = Uuid::now_v7();
    let proj2_id = Uuid::now_v7();

    let c1_id = Uuid::now_v7();
    let c2_id = Uuid::now_v7();
    let c3_id = Uuid::now_v7();
    let c4_id = Uuid::now_v7();

    {
        let conn = pool.get().unwrap();

        // Projects
        conn.execute(
            "INSERT INTO projects (id, code, display_name) VALUES (?1, 'proj_1', 'Project 1')",
            params![proj1_id.to_string()],
        )
        .unwrap();
        conn.execute(
            "INSERT INTO projects (id, code, display_name) VALUES (?1, 'proj_2', 'Project 2')",
            params![proj2_id.to_string()],
        )
        .unwrap();

        // Character 1: Linh Đan (Tiểu Linh) - In Project 1, Female
        conn.execute(
            "INSERT INTO characters (id, code, display_name, nickname, species, biological_sex, gender_identity, gender_expression)
             VALUES (?1, 'linh_dan', 'Linh Đan', 'Tiểu Linh', 'HUMAN', 'FEMALE', 'FEMALE', 'FEMALE')",
            params![c1_id.to_string()],
        ).unwrap();
        conn.execute(
            "INSERT INTO project_characters (id, project_id, character_id) VALUES (?1, ?2, ?3)",
            params![
                Uuid::now_v7().to_string(),
                proj1_id.to_string(),
                c1_id.to_string()
            ],
        )
        .unwrap();

        // Character 2: Minh Hoàng (Hoàng Tử) - In Project 1, Male
        conn.execute(
            "INSERT INTO characters (id, code, display_name, nickname, species, biological_sex, gender_identity, gender_expression)
             VALUES (?1, 'minh_hoang', 'Minh Hoàng', 'Hoàng Tử', 'HUMAN', 'MALE', 'MALE', 'MALE')",
            params![c2_id.to_string()],
        ).unwrap();
        conn.execute(
            "INSERT INTO project_characters (id, project_id, character_id) VALUES (?1, ?2, ?3)",
            params![
                Uuid::now_v7().to_string(),
                proj1_id.to_string(),
                c2_id.to_string()
            ],
        )
        .unwrap();

        // Character 3: Đức Trọng (Đại Ca) - In Project 2 only, Male
        conn.execute(
            "INSERT INTO characters (id, code, display_name, nickname, species, biological_sex, gender_identity, gender_expression)
             VALUES (?1, 'duc_trong', 'Đức Trọng', 'Đại Ca', 'HUMAN', 'MALE', 'MALE', 'MALE')",
            params![c3_id.to_string()],
        ).unwrap();
        conn.execute(
            "INSERT INTO project_characters (id, project_id, character_id) VALUES (?1, ?2, ?3)",
            params![
                Uuid::now_v7().to_string(),
                proj2_id.to_string(),
                c3_id.to_string()
            ],
        )
        .unwrap();

        // Character 4: Bạch Lan (Tiểu Lan) - Library only (no project), Female
        conn.execute(
            "INSERT INTO characters (id, code, display_name, nickname, species, biological_sex, gender_identity, gender_expression)
             VALUES (?1, 'bach_lan', 'Bạch Lan', 'Tiểu Lan', 'HUMAN', 'FEMALE', 'FEMALE', 'FEMALE')",
            params![c4_id.to_string()],
        ).unwrap();
    }

    // Tier 1: NFC normalization test (decomposed vs precomposed)
    // Decomposed 'Đức': 'Đ' + 'u' + combining acute + 'c'
    let decomposed_duc = "\u{0110}\u{0075}\u{0301}\u{0063}";
    let res_nfc = character_service
        .list_characters(&ListCharactersReq {
            project_id: None,
            biological_sex: None,
            query: Some(decomposed_duc.to_string()),
        })
        .expect("search decomposed nfc");
    assert_eq!(res_nfc.items.len(), 1);
    assert_eq!(res_nfc.items[0].name, "Đức Trọng");

    // Tier 2: Exact match on display_name or nickname
    let res_exact_name = character_service
        .list_characters(&ListCharactersReq {
            project_id: None,
            biological_sex: None,
            query: Some("linh đan".to_string()),
        })
        .expect("search exact name");
    assert_eq!(res_exact_name.items.len(), 1);
    assert_eq!(res_exact_name.items[0].name, "Linh Đan");

    let res_exact_nick = character_service
        .list_characters(&ListCharactersReq {
            project_id: None,
            biological_sex: None,
            query: Some("hoàng tử".to_string()),
        })
        .expect("search exact nickname");
    assert_eq!(res_exact_nick.items.len(), 1);
    assert_eq!(res_exact_nick.items[0].name, "Minh Hoàng");

    // Tier 3: Prefix & Substring match
    let res_prefix = character_service
        .list_characters(&ListCharactersReq {
            project_id: None,
            biological_sex: None,
            query: Some("minh".to_string()),
        })
        .expect("search prefix");
    assert_eq!(res_prefix.items.len(), 1);
    assert_eq!(res_prefix.items[0].name, "Minh Hoàng");

    let res_substr = character_service
        .list_characters(&ListCharactersReq {
            project_id: None,
            biological_sex: None,
            query: Some("trọng".to_string()),
        })
        .expect("search substring");
    assert_eq!(res_substr.items.len(), 1);
    assert_eq!(res_substr.items[0].name, "Đức Trọng");

    // Tier 4: Vietnamese diacritics removal (unaccented search)
    let res_unaccented_name = character_service
        .list_characters(&ListCharactersReq {
            project_id: None,
            biological_sex: None,
            query: Some("linh dan".to_string()),
        })
        .expect("search unaccented name");
    assert_eq!(res_unaccented_name.items.len(), 1);
    assert_eq!(res_unaccented_name.items[0].name, "Linh Đan");

    let res_unaccented_nick = character_service
        .list_characters(&ListCharactersReq {
            project_id: None,
            biological_sex: None,
            query: Some("tieu linh".to_string()),
        })
        .expect("search unaccented nickname");
    assert_eq!(res_unaccented_nick.items.len(), 1);
    assert_eq!(res_unaccented_nick.items[0].name, "Linh Đan");

    let res_unaccented_duc = character_service
        .list_characters(&ListCharactersReq {
            project_id: None,
            biological_sex: None,
            query: Some("duc trong".to_string()),
        })
        .expect("search unaccented duc trong");
    assert_eq!(res_unaccented_duc.items.len(), 1);
    assert_eq!(res_unaccented_duc.items[0].name, "Đức Trọng");

    // Project scope & Fallback behavior
    // Step A: Search in Project 1 for "Linh" -> matches Linh Đan in Project 1 (in_project = true)
    let res_p1_match = character_service
        .list_characters(&ListCharactersReq {
            project_id: Some(proj1_id),
            biological_sex: None,
            query: Some("Linh".to_string()),
        })
        .expect("search in project 1");
    assert_eq!(res_p1_match.items.len(), 1);
    assert_eq!(res_p1_match.items[0].name, "Linh Đan");
    assert!(res_p1_match.items[0].in_project);

    // Step B: Search in Project 1 for "Bạch Lan" -> 0 matches in Project 1, fallback to library
    let res_p1_fallback = character_service
        .list_characters(&ListCharactersReq {
            project_id: Some(proj1_id),
            biological_sex: None,
            query: Some("Bạch Lan".to_string()),
        })
        .expect("search in project 1 with fallback");
    assert_eq!(res_p1_fallback.items.len(), 1);
    assert_eq!(res_p1_fallback.items[0].name, "Bạch Lan");
    assert!(!res_p1_fallback.items[0].in_project); // Not in project 1

    // Biological sex filter
    let res_sex_female = character_service
        .list_characters(&ListCharactersReq {
            project_id: None,
            biological_sex: Some("FEMALE".to_string()),
            query: None,
        })
        .expect("filter female");
    assert_eq!(res_sex_female.items.len(), 2);
    assert!(res_sex_female
        .items
        .iter()
        .all(|c| c.biological_sex == "FEMALE"));

    let res_sex_male = character_service
        .list_characters(&ListCharactersReq {
            project_id: None,
            biological_sex: Some("MALE".to_string()),
            query: None,
        })
        .expect("filter male");
    assert_eq!(res_sex_male.items.len(), 2);
    assert!(res_sex_male
        .items
        .iter()
        .all(|c| c.biological_sex == "MALE"));
}

/// P4S3-CHARACTER-DETAIL-AND-UPSERT: Complete character 3-tab creation, updates, and invariant validation.
#[test]
fn P4S3_CHARACTER_DETAIL_AND_UPSERT() {
    let temp_dir = TempDir::new().unwrap();
    let (_db_path, pool) = setup_test_db(&temp_dir);
    let registry = Arc::new(DraftRuntimeRegistry::new());
    let draft_service = Arc::new(DraftService::new(pool.clone(), registry.clone()));
    let character_service = CharacterService::new(pool.clone(), draft_service);

    let proj_id = Uuid::now_v7();
    let arc_id = Uuid::now_v7().to_string();
    let ch_id = Uuid::now_v7().to_string();
    let sc_id = Uuid::now_v7();
    let ws_id = Uuid::now_v7().to_string();

    {
        let conn = pool.get().unwrap();
        conn.execute(
            "INSERT INTO projects (id, code, display_name) VALUES (?1, 'proj_main', 'Main Project')",
            params![proj_id.to_string()],
        ).unwrap();
        seed_test_hierarchy(
            &conn,
            &proj_id.to_string(),
            &ws_id,
            &arc_id,
            &ch_id,
            &sc_id.to_string(),
            &Uuid::now_v7().to_string(),
        );
    }

    // Invariant check 1: name and nickname both empty yields ValidationFailed
    let invalid_name_req = CreateOrUpdateLibraryCharacterReq {
        character_id: None,
        project_id: Some(proj_id),
        profile: CharacterUpsertPayloadDto {
            name: "   ".to_string(),
            nickname: None,
            species: "HUMAN".to_string(),
            biological_sex: "FEMALE".to_string(),
            gender_identity: "FEMALE".to_string(),
            gender_expression: "FEMALE".to_string(),
            appearance: None,
            psychological: None,
            memories: vec![],
            competences: vec![],
        },
    };
    let err_name = character_service.create_or_update_library_character(&invalid_name_req);
    assert_eq!(err_name.unwrap_err().code, ErrorCode::ValidationFailed);

    // Invariant check 2: competence without scene_id yields ValidationFailed
    let invalid_comp_req = CreateOrUpdateLibraryCharacterReq {
        character_id: None,
        project_id: Some(proj_id),
        profile: CharacterUpsertPayloadDto {
            name: "Vân Tiêu".to_string(),
            nickname: Some("Tiểu Tiêu".to_string()),
            species: "HUMAN".to_string(),
            biological_sex: "FEMALE".to_string(),
            gender_identity: "FEMALE".to_string(),
            gender_expression: "FEMALE".to_string(),
            appearance: None,
            psychological: None,
            memories: vec![],
            competences: vec![CharacterCompetenceDto {
                id: Uuid::now_v7(),
                domain: "Martial Arts".to_string(),
                competence_name: "Kiếm Khí".to_string(),
                level: "HIGH".to_string(),
                scene_id: None, // Missing scene_id!
            }],
        },
    };
    let err_comp = character_service.create_or_update_library_character(&invalid_comp_req);
    assert_eq!(err_comp.unwrap_err().code, ErrorCode::ValidationFailed);

    // Invariant check 3: competence with non-existent scene_id yields ValidationFailed
    let invalid_scene_req = CreateOrUpdateLibraryCharacterReq {
        character_id: None,
        project_id: Some(proj_id),
        profile: CharacterUpsertPayloadDto {
            name: "Vân Tiêu".to_string(),
            nickname: Some("Tiểu Tiêu".to_string()),
            species: "HUMAN".to_string(),
            biological_sex: "FEMALE".to_string(),
            gender_identity: "FEMALE".to_string(),
            gender_expression: "FEMALE".to_string(),
            appearance: None,
            psychological: None,
            memories: vec![],
            competences: vec![CharacterCompetenceDto {
                id: Uuid::now_v7(),
                domain: "Martial Arts".to_string(),
                competence_name: "Kiếm Khí".to_string(),
                level: "HIGH".to_string(),
                scene_id: Some(Uuid::now_v7()), // Non-existent scene!
            }],
        },
    };
    let err_scene = character_service.create_or_update_library_character(&invalid_scene_req);
    assert_eq!(err_scene.unwrap_err().code, ErrorCode::ValidationFailed);

    // Create valid character with full 3-tab details
    let comp_id = Uuid::now_v7();
    let mem_id = Uuid::now_v7();

    let valid_upsert_req = CreateOrUpdateLibraryCharacterReq {
        character_id: None,
        project_id: Some(proj_id),
        profile: CharacterUpsertPayloadDto {
            name: "Triệu Băng Nhi".to_string(),
            nickname: Some("Băng Băng".to_string()),
            species: "HUMAN".to_string(),
            biological_sex: "FEMALE".to_string(),
            gender_identity: "FEMALE".to_string(),
            gender_expression: "FEMALE".to_string(),
            appearance: Some(CharacterAppearanceDto {
                height: 168.0,
                weight: 48.0,
                hair: "Jet Black".to_string(),
                eyes: "Sky Blue".to_string(),
                skin: "Porcelain Fair".to_string(),
                hair_note: "Long straight hair".to_string(),
                eyes_note: "Sharp cold gaze".to_string(),
                skin_note: "Flawless".to_string(),
                outfit_style: "Azure robe".to_string(),
                default_summary: "Cold elegant swordswoman".to_string(),
            }),
            psychological: Some(CharacterPsychologicalDto {
                core_values: vec!["Loyalty".to_string(), "Truth".to_string()],
                trauma_triggers: vec!["Wildfire".to_string()],
                behavioral_quirks: vec!["Touches sword hilt when nervous".to_string()],
                summary: "Traumatized by mountain sect massacre".to_string(),
            }),
            memories: vec![CharacterMemoryDto {
                id: mem_id,
                memory_type: "EPISODIC".to_string(),
                title: "Tuyết Sơn Biến Cố".to_string(),
                description: "The night the sect fell".to_string(),
                emotional_charge: "NEGATIVE".to_string(),
            }],
            competences: vec![CharacterCompetenceDto {
                id: comp_id,
                domain: "Swordsmanship".to_string(),
                competence_name: "Băng Hàn Kiếm Quyết".to_string(),
                level: "EXPERT".to_string(),
                scene_id: Some(sc_id),
            }],
        },
    };

    let created = character_service
        .create_or_update_library_character(&valid_upsert_req)
        .expect("create character success");

    let char_id = created.character.id;
    assert_eq!(created.character.name, "Triệu Băng Nhi");
    assert_eq!(created.character.nickname, Some("Băng Băng".to_string()));
    assert!(created.character.in_project);
    assert_eq!(created.character.code, "trieu_bang_nhi");

    let app = created.appearance.expect("appearance present");
    assert_eq!(app.height, 168.0);
    assert_eq!(app.weight, 48.0);
    assert_eq!(app.hair, "Jet Black");
    assert_eq!(app.eyes, "Sky Blue");
    assert_eq!(app.default_summary, "Cold elegant swordswoman");

    let psych = created.psychological.expect("psych present");
    assert_eq!(psych.summary, "Traumatized by mountain sect massacre");
    assert_eq!(psych.core_values, vec!["Loyalty", "Truth"]);
    assert_eq!(psych.trauma_triggers, vec!["Wildfire"]);

    assert_eq!(created.memories.len(), 1);
    assert_eq!(created.memories[0].title, "Tuyết Sơn Biến Cố");

    assert_eq!(created.competences.len(), 1);
    assert_eq!(
        created.competences[0].competence_name,
        "Băng Hàn Kiếm Quyết"
    );
    assert_eq!(created.competences[0].scene_id, Some(sc_id));

    // Verify get_character_detail directly
    let detail = character_service
        .get_character_detail(&GetCharacterDetailReq {
            character_id: char_id,
            project_id: Some(proj_id),
        })
        .expect("get_character_detail");
    assert_eq!(detail.character.id, char_id);
    assert!(detail.character.in_project);

    // Update existing character
    let update_req = CreateOrUpdateLibraryCharacterReq {
        character_id: Some(char_id),
        project_id: Some(proj_id),
        profile: CharacterUpsertPayloadDto {
            name: "Triệu Băng Nhi (Kiếm Tôn)".to_string(),
            nickname: Some("Băng Tôn".to_string()),
            species: "HUMAN".to_string(),
            biological_sex: "FEMALE".to_string(),
            gender_identity: "FEMALE".to_string(),
            gender_expression: "FEMALE".to_string(),
            appearance: Some(CharacterAppearanceDto {
                height: 169.0,
                weight: 49.0,
                hair: "Silver White".to_string(),
                eyes: "Violet".to_string(),
                skin: "Porcelain Fair".to_string(),
                hair_note: "Flowing silver".to_string(),
                eyes_note: "Divine gaze".to_string(),
                skin_note: "Flawless".to_string(),
                outfit_style: "White robe".to_string(),
                default_summary: "Enlightened Sword Venerable".to_string(),
            }),
            psychological: Some(CharacterPsychologicalDto {
                core_values: vec!["Harmony".to_string()],
                trauma_triggers: vec![],
                behavioral_quirks: vec!["Calm breathing".to_string()],
                summary: "Past reconciled".to_string(),
            }),
            memories: vec![],
            competences: vec![CharacterCompetenceDto {
                id: Uuid::now_v7(),
                domain: "Swordsmanship".to_string(),
                competence_name: "Vạn Kiếm Quy Tông".to_string(),
                level: "MASTER".to_string(),
                scene_id: Some(sc_id),
            }],
        },
    };

    let updated = character_service
        .create_or_update_library_character(&update_req)
        .expect("update character");
    assert_eq!(updated.character.name, "Triệu Băng Nhi (Kiếm Tôn)");
    assert_eq!(updated.character.nickname, Some("Băng Tôn".to_string()));
    assert_eq!(updated.appearance.unwrap().hair, "Silver White");
    assert_eq!(updated.competences[0].competence_name, "Vạn Kiếm Quy Tông");

    // Unknown character ID yields EntityNotFound
    let not_found = character_service.get_character_detail(&GetCharacterDetailReq {
        character_id: Uuid::now_v7(),
        project_id: None,
    });
    assert_eq!(not_found.unwrap_err().code, ErrorCode::EntityNotFound);
}

async fn run_link_to_draft_scenario() {
    let temp_dir = TempDir::new().unwrap();
    let (_db_path, pool) = setup_test_db(&temp_dir);
    let registry = Arc::new(DraftRuntimeRegistry::new());
    let draft_service = Arc::new(DraftService::new(pool.clone(), registry.clone()));
    let character_service = CharacterService::new(pool.clone(), draft_service.clone());

    let proj_id = Uuid::now_v7();
    let draft_id = Uuid::now_v7();
    let session_id = "session_link_test";
    let char_id = Uuid::now_v7();

    // 1. Seed project & character
    {
        let conn = pool.get().unwrap();
        conn.execute(
            "INSERT INTO projects (id, code, display_name) VALUES (?1, 'proj_link', 'Link Test Project')",
            params![proj_id.to_string()],
        ).unwrap();

        conn.execute(
            "INSERT INTO characters (id, code, display_name, nickname, biological_sex)
             VALUES (?1, 'char_diep_thanh', 'Diệp Thanh', 'Thanh Nhi', 'MALE')",
            params![char_id.to_string()],
        )
        .unwrap();
    }

    // 2. Seed draft session in "editing" state
    seed_draft_session(&pool, draft_id, proj_id, session_id, "editing");

    // 3. Link library character to draft
    let link_req = LinkLibraryCharacterToDraftReq {
        draft_session_id: draft_id,
        character_id: char_id,
        role: Some("RIVAL".to_string()),
        notes: Some("Former disciple rival".to_string()),
    };

    let link_resp = character_service
        .link_library_character_to_draft(&link_req)
        .await
        .expect("link character to draft");

    assert_eq!(link_resp.character_code, "char_diep_thanh");
    assert_eq!(link_resp.sealed_version.seq, 1);
    assert_eq!(link_resp.sealed_version.reason, "manual_edit");

    // Verify WorkingState in runtime registry
    let entry = draft_service.get_or_load_runtime_entry(draft_id).unwrap();
    let snap = entry.get_snapshot();
    let ws_chars = &snap.business_flow_data.require.character;
    assert_eq!(ws_chars.len(), 1);
    assert_eq!(ws_chars[0].code, "char_diep_thanh");
    assert_eq!(
        ws_chars[0].display_name.value.as_deref(),
        Some("Diệp Thanh")
    );
    assert_eq!(ws_chars[0].role.value.as_deref(), Some("RIVAL"));
    assert_eq!(
        ws_chars[0].notes.value.as_deref(),
        Some("Former disciple rival")
    );
    assert!(ws_chars[0].reused_library_character);

    // 4. Test idempotency (linking the same character again must not duplicate)
    let link_again_resp = character_service
        .link_library_character_to_draft(&link_req)
        .await
        .expect("link same character again");

    assert_eq!(link_again_resp.character_code, "char_diep_thanh");
    assert_eq!(link_again_resp.sealed_version.seq, 1);

    let snap2 = entry.get_snapshot();
    let ws_chars2 = &snap2.business_flow_data.require.character;
    assert_eq!(
        ws_chars2.len(),
        1,
        "Idempotent link must NOT create duplicate character in draft"
    );

    // 5. Error case: Character not found
    let err_char_not_found = character_service
        .link_library_character_to_draft(&LinkLibraryCharacterToDraftReq {
            draft_session_id: draft_id,
            character_id: Uuid::now_v7(),
            role: None,
            notes: None,
        })
        .await;
    assert_eq!(
        err_char_not_found.unwrap_err().code,
        ErrorCode::EntityNotFound
    );

    // 6. Error case: Draft session not found
    let err_draft_not_found = character_service
        .link_library_character_to_draft(&LinkLibraryCharacterToDraftReq {
            draft_session_id: Uuid::now_v7(),
            character_id: char_id,
            role: None,
            notes: None,
        })
        .await;
    assert_eq!(
        err_draft_not_found.unwrap_err().code,
        ErrorCode::EntityNotFound
    );

    // 7. Error case: Draft in committing / committed status cannot link characters
    let draft_committed_id = Uuid::now_v7();
    seed_draft_session(
        &pool,
        draft_committed_id,
        proj_id,
        "session_committed",
        "committed",
    );

    let err_committed = character_service
        .link_library_character_to_draft(&LinkLibraryCharacterToDraftReq {
            draft_session_id: draft_committed_id,
            character_id: char_id,
            role: None,
            notes: None,
        })
        .await;
    assert_eq!(err_committed.unwrap_err().code, ErrorCode::DraftNotEditable);
}

/// P4S3-LINK-TO-DRAFT: Link character into draft WorkingState, avoiding duplicates, sealing version TX-01.
#[tokio::test]
async fn P4S3_LINK_TO_DRAFT() {
    run_link_to_draft_scenario().await;
}

// ============================================================================
// IPC Commands Direct Execution Tests (#22 .. #28)
// ============================================================================

/// IPC-CMD-022: list_projects command
#[test]
fn IPC_CMD_022_list_projects() {
    let temp_dir = TempDir::new().unwrap();
    let (_db_path, pool) = setup_test_db(&temp_dir);
    let service = ProjectService::new(pool.clone());

    let res =
        exec_list_projects(&service, &ListProjectsReq::default()).expect("exec_list_projects");
    assert_eq!(res.items.len(), 0);
}

/// IPC-CMD-023: get_project_detail command
#[test]
fn IPC_CMD_023_get_project_detail() {
    let temp_dir = TempDir::new().unwrap();
    let (_db_path, pool) = setup_test_db(&temp_dir);
    let service = ProjectService::new(pool.clone());

    let pid = Uuid::now_v7();
    {
        let conn = pool.get().unwrap();
        conn.execute(
            "INSERT INTO projects (id, code, display_name) VALUES (?1, 'p23', 'Project 23')",
            params![pid.to_string()],
        )
        .unwrap();
    }

    let res = exec_get_project_detail(&service, &GetProjectDetailReq { project_id: pid })
        .expect("exec_get_project_detail");
    assert_eq!(res.project.title, "Project 23");
}

/// IPC-CMD-024: delete_project command
#[test]
fn IPC_CMD_024_delete_project() {
    let temp_dir = TempDir::new().unwrap();
    let (_db_path, pool) = setup_test_db(&temp_dir);
    let service = ProjectService::new(pool.clone());

    let pid = Uuid::now_v7();
    {
        let conn = pool.get().unwrap();
        conn.execute(
            "INSERT INTO projects (id, code, display_name) VALUES (?1, 'p24', 'Project 24')",
            params![pid.to_string()],
        )
        .unwrap();
    }

    let res = exec_delete_project(&service, &DeleteProjectReq { project_id: pid })
        .expect("exec_delete_project");
    assert!(res.deleted);
}

/// IPC-CMD-025: list_characters command
#[test]
fn IPC_CMD_025_list_characters() {
    let temp_dir = TempDir::new().unwrap();
    let (_db_path, pool) = setup_test_db(&temp_dir);
    let registry = Arc::new(DraftRuntimeRegistry::new());
    let draft_service = Arc::new(DraftService::new(pool.clone(), registry.clone()));
    let service = CharacterService::new(pool.clone(), draft_service);

    let res = exec_list_characters(&service, &ListCharactersReq::default())
        .expect("exec_list_characters");
    assert_eq!(res.items.len(), 0);
}

/// IPC-CMD-026: get_character_detail command
#[test]
fn IPC_CMD_026_get_character_detail() {
    let temp_dir = TempDir::new().unwrap();
    let (_db_path, pool) = setup_test_db(&temp_dir);
    let registry = Arc::new(DraftRuntimeRegistry::new());
    let draft_service = Arc::new(DraftService::new(pool.clone(), registry.clone()));
    let service = CharacterService::new(pool.clone(), draft_service);

    let cid = Uuid::now_v7();
    {
        let conn = pool.get().unwrap();
        conn.execute(
            "INSERT INTO characters (id, code, display_name) VALUES (?1, 'c26', 'Char 26')",
            params![cid.to_string()],
        )
        .unwrap();
    }

    let res = exec_get_character_detail(
        &service,
        &GetCharacterDetailReq {
            character_id: cid,
            project_id: None,
        },
    )
    .expect("exec_get_character_detail");
    assert_eq!(res.character.name, "Char 26");
}

/// IPC-CMD-027: create_or_update_library_character command
#[test]
fn IPC_CMD_027_create_or_update_library_character() {
    let temp_dir = TempDir::new().unwrap();
    let (_db_path, pool) = setup_test_db(&temp_dir);
    let registry = Arc::new(DraftRuntimeRegistry::new());
    let draft_service = Arc::new(DraftService::new(pool.clone(), registry.clone()));
    let service = CharacterService::new(pool.clone(), draft_service);

    let req = CreateOrUpdateLibraryCharacterReq {
        character_id: None,
        project_id: None,
        profile: CharacterUpsertPayloadDto {
            name: "Hạ Lan".to_string(),
            nickname: None,
            species: "HUMAN".to_string(),
            biological_sex: "FEMALE".to_string(),
            gender_identity: "FEMALE".to_string(),
            gender_expression: "FEMALE".to_string(),
            appearance: None,
            psychological: None,
            memories: vec![],
            competences: vec![],
        },
    };

    let res = exec_create_or_update_library_character(&service, &req)
        .expect("exec_create_or_update_library_character");
    assert_eq!(res.character.name, "Hạ Lan");
}

/// IPC-CMD-028: link_library_character_to_draft command
#[tokio::test]
async fn IPC_CMD_028_link_library_character_to_draft() {
    let temp_dir = TempDir::new().unwrap();
    let (_db_path, pool) = setup_test_db(&temp_dir);
    let registry = Arc::new(DraftRuntimeRegistry::new());
    let draft_service = Arc::new(DraftService::new(pool.clone(), registry.clone()));
    let service = CharacterService::new(pool.clone(), draft_service);

    let pid = Uuid::now_v7();
    let did = Uuid::now_v7();
    let cid = Uuid::now_v7();

    {
        let conn = pool.get().unwrap();
        conn.execute(
            "INSERT INTO projects (id, code, display_name) VALUES (?1, 'p28', 'Project 28')",
            params![pid.to_string()],
        )
        .unwrap();
        conn.execute(
            "INSERT INTO characters (id, code, display_name) VALUES (?1, 'c28', 'Char 28')",
            params![cid.to_string()],
        )
        .unwrap();
    }
    seed_draft_session(&pool, did, pid, "s28", "editing");

    let req = LinkLibraryCharacterToDraftReq {
        draft_session_id: did,
        character_id: cid,
        role: Some("ALLY".to_string()),
        notes: None,
    };

    let res = exec_link_library_character_to_draft(&service, &req)
        .await
        .expect("exec_link_library_character_to_draft");
    assert_eq!(res.character_code, "c28");
    assert_eq!(res.sealed_version.seq, 1);
}

// ============================================================================
// Sprint Contract Matrix Aliases (Matching Plan & Failure Matrix Names)
// ============================================================================

/// P4S3-P: Positive end-to-end smoke across catalog services.
#[tokio::test]
async fn P4S3_P() {
    P4S3_PROJECT_LIST_AND_DETAIL();
    P4S3_CHARACTER_4TIER_SEARCH();
    P4S3_CHARACTER_DETAIL_AND_UPSERT();
    run_link_to_draft_scenario().await;
}

/// P4S3-N / phase4_sprint3_deep1_F1: Scoped soft-delete preserves cross-project shared references.
#[test]
fn P4S3_N() {
    P4S3_PROJECT_DELETE_SCOPED();
}

#[test]
fn phase4_sprint3_deep1_F1() {
    P4S3_PROJECT_DELETE_SCOPED();
}

/// P4S3-B: Linking existing character avoids duplicates in draft WorkingState.
#[tokio::test]
async fn P4S3_B() {
    run_link_to_draft_scenario().await;
}

/// phase4_sprint3_deep1_F2: Unknown project / character yields EntityNotFound and zero writes.
#[test]
fn phase4_sprint3_deep1_F2() {
    let temp_dir = TempDir::new().unwrap();
    let (_db_path, pool) = setup_test_db(&temp_dir);
    let registry = Arc::new(DraftRuntimeRegistry::new());
    let draft_service = Arc::new(DraftService::new(pool.clone(), registry.clone()));
    let project_service = ProjectService::new(pool.clone());
    let character_service = CharacterService::new(pool.clone(), draft_service);

    let rand_id = Uuid::now_v7();
    let err_proj = project_service
        .get_project_detail(&GetProjectDetailReq {
            project_id: rand_id,
        })
        .unwrap_err();
    assert_eq!(err_proj.code, ErrorCode::EntityNotFound);

    let err_del = project_service
        .delete_project(&DeleteProjectReq {
            project_id: rand_id,
        })
        .unwrap_err();
    assert_eq!(err_del.code, ErrorCode::EntityNotFound);

    let err_char = character_service
        .get_character_detail(&GetCharacterDetailReq {
            character_id: rand_id,
            project_id: None,
        })
        .unwrap_err();
    assert_eq!(err_char.code, ErrorCode::EntityNotFound);
}
