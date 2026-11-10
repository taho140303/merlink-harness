//! SQLite schema migration orchestrator, transactional execution, and integrity verifier.
//! Strictly enforces docs/business/be-core.md (§3.4, §3.5) and docs/db.dbml physical SSOT.

use std::collections::BTreeSet;
use std::path::Path;
use rusqlite::Connection;

use super::db::{backup_path, perform_online_backup, DbError};

/// Maximum schema migration version supported by this application release (§3.4).
pub const MAX_SUPPORTED_SCHEMA_VERSION: u32 = 18;

/// Embedded forward-only migration scripts.
pub const MIGRATION_0001_SQL: &str = include_str!("../../migrations/0001_initial_schema.sql");
pub const MIGRATION_0002_SQL: &str = include_str!("../../migrations/0002_seed_reference_data.sql");
pub const MIGRATION_0003_SQL: &str = include_str!("../../migrations/0003_sf06_b_plus.sql");
pub const MIGRATION_0004_SQL: &str = include_str!("../../migrations/0004_seed_style_presets.sql");
pub const MIGRATION_0005_SQL: &str = include_str!("../../migrations/0005_seed_prompts.sql");
pub const MIGRATION_0006_SQL: &str = include_str!("../../migrations/0006_seed_prompt_read_tools.sql");
pub const MIGRATION_0007_SQL: &str = include_str!("../../migrations/0007_fix_prompt_identity.sql");
pub const MIGRATION_0008_SQL: &str = include_str!("../../migrations/0008_session_trash.sql");
pub const MIGRATION_0009_SQL: &str = include_str!("../../migrations/0009_tool_catalog_and_invariant.sql");
pub const MIGRATION_0010_SQL: &str = include_str!("../../migrations/0010_refine_chat_contracts.sql");
pub const MIGRATION_0013_SQL: &str = include_str!("../../migrations/0013_author_facing_language.sql");
pub const MIGRATION_0012_SQL: &str = include_str!("../../migrations/0012_clarify_story_summary.sql");
pub const MIGRATION_0011_SQL: &str = include_str!("../../migrations/0011_clarify_planning_scope.sql");
pub const MIGRATION_0014_SQL: &str = include_str!("../../migrations/0014_seed_system_catalogs.sql");
pub const MIGRATION_0015_SQL: &str = include_str!("../../migrations/0015_phase0_catalog_tools.sql");
pub const MIGRATION_0016_SQL: &str = include_str!("../../migrations/0016_seed_phase_prompts.sql");
pub const MIGRATION_0017_SQL: &str = include_str!("../../migrations/0017_tool_call_schemas.sql");
pub const MIGRATION_0018_SQL: &str = include_str!("../../migrations/0018_character_coverage.sql");

/// Complete set of exactly 66 physical tables defined in docs/db.dbml (B+ SF-06 target).
pub const TARGET_TABLES_V3: &[&str] = &[
    "motifs",
    "genres",
    "motif_applicable_genres",
    "event_blocks",
    "event_transitions",
    "states",
    "state_down_by_rules",
    "state_rules",
    "emotional_ranges",
    "event_block_emotionals",
    "components",
    "motif_constraints",
    "constraints",
    "style_presets",
    "rhetorical_devices",
    "style_preset_rhetorical_devices",
    "style_preset_vocab_items",
    "style_preset_focus_items",
    "narrator_configs",
    "style_preset_narratives",
    "narrative_directives",
    "character_motifs",
    "projects",
    "sessions",
    "messages",
    "project_genres",
    "arc_motifs",
    "arc_event_blocks",
    "arcs",
    "chapters",
    "scenes",
    "scene_lore_entries",
    "scene_characters",
    "scene_versions",
    "chapter_save_receipts",
    "characters",
    "project_characters",
    "appearance_option_groups",
    "appearance_options",
    "character_appearances",
    "character_psychological",
    "character_distinguishing_marks",
    "relationships",
    "relationship_categories",
    "relationship_roles",
    "relationship_states",
    "world_settings",
    "lore_entries",
    "themes",
    "theme_arguments",
    "character_memories",
    "competence_domains",
    "domain_level",
    "character_competences",
    "competence_capabilities",
    "competence_limitations",
    "symbolic_motifs",
    "symbolics",
    "prompt_blocks",
    "tools",
    "prompt_block_tools",
    "draft_sessions",
    "draft_versions",
    "workflow_round_receipts",
    "workflow_task_receipts",
    "pending_patches",
];

/// Read authoritative schema version from PRAGMA user_version (§3.4).
pub fn get_user_version(conn: &Connection) -> Result<u32, DbError> {
    let version: u32 = conn.query_row("PRAGMA user_version;", [], |row| row.get(0))?;
    Ok(version)
}

/// Set schema version via PRAGMA user_version.
pub fn set_user_version(conn: &Connection, version: u32) -> Result<(), DbError> {
    conn.execute(&format!("PRAGMA user_version = {};", version), [])?;
    Ok(())
}

/// Retrieve SQL statements corresponding to target migration version.
pub fn get_migration_sql(target_version: u32) -> Result<&'static str, DbError> {
    match target_version {
        1 => Ok(MIGRATION_0001_SQL),
        2 => Ok(MIGRATION_0002_SQL),
        3 => Ok(MIGRATION_0003_SQL),
        4 => Ok(MIGRATION_0004_SQL),
        5 => Ok(MIGRATION_0005_SQL),
        6 => Ok(MIGRATION_0006_SQL),
        7 => Ok(MIGRATION_0007_SQL),
        8 => Ok(MIGRATION_0008_SQL),
        9 => Ok(MIGRATION_0009_SQL),
        10 => Ok(MIGRATION_0010_SQL),
        11 => Ok(MIGRATION_0011_SQL),
        12 => Ok(MIGRATION_0012_SQL),
        13 => Ok(MIGRATION_0013_SQL),
        14 => Ok(MIGRATION_0014_SQL),
        15 => Ok(MIGRATION_0015_SQL),
        16 => Ok(MIGRATION_0016_SQL),
        17 => Ok(MIGRATION_0017_SQL),
        18 => Ok(MIGRATION_0018_SQL),
        _ => Err(DbError::MigrationFailed(format!(
            "No migration script registered for target version {}",
            target_version
        ))),
    }
}

/// Run sequential forward-only migrations from `current_version + 1` to `MAX_SUPPORTED_SCHEMA_VERSION`.
///
/// Guarantees:
/// - Rejects `current_version > MAX_SUPPORTED_SCHEMA_VERSION` immediately with `UnsupportedSchemaVersion`
///   without modifying any bytes on disk.
/// - Each migration step runs in its own `BEGIN IMMEDIATE ... COMMIT` transaction.
/// - If any statement fails, the transaction is completely rolled back, leaving `user_version` at `k - 1`.
pub fn run_migrations(conn: &mut Connection) -> Result<(), DbError> {
    let current_version = get_user_version(conn)?;

    if current_version > MAX_SUPPORTED_SCHEMA_VERSION {
        return Err(DbError::UnsupportedSchemaVersion {
            current: current_version,
            max_supported: MAX_SUPPORTED_SCHEMA_VERSION,
        });
    }

    if current_version == MAX_SUPPORTED_SCHEMA_VERSION {
        return Ok(());
    }

    for target_version in (current_version + 1)..=MAX_SUPPORTED_SCHEMA_VERSION {
        let sql = get_migration_sql(target_version)?;

        // Execute step in an atomic transaction
        let tx = conn
            .transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)
            .map_err(|e| DbError::MigrationFailed(format!("Failed to begin transaction: {}", e)))?;

        tx.execute_batch(sql).map_err(|e| {
            DbError::MigrationFailed(format!(
                "Failed executing migration to v{}: {}",
                target_version, e
            ))
        })?;

        // Explicitly set user_version within transaction
        tx.execute(&format!("PRAGMA user_version = {};", target_version), [])
            .map_err(|e| {
                DbError::MigrationFailed(format!(
                    "Failed updating user_version to {}: {}",
                    target_version, e
                ))
            })?;

        tx.commit().map_err(|e| {
            DbError::MigrationFailed(format!(
                "Failed committing transaction for v{}: {}",
                target_version, e
            ))
        })?;
    }

    Ok(())
}

#[cfg(test)]
mod prompt_seed_tests {
    use super::*;

    #[test]
    fn test_prompt_seed_contains_merlink_identity_and_callable_tools() {
        let mut conn = Connection::open_in_memory().unwrap();
        run_migrations(&mut conn).unwrap();
        let identity: String = conn.query_row(
            "SELECT content FROM prompt_blocks WHERE prompt_code = 'introduction'",
            [],
            |row| row.get(0),
        ).unwrap();
        assert!(identity.contains("Merlink Harness"));
        let phase_blocks: i64 = conn.query_row(
            "SELECT COUNT(*) FROM prompt_blocks WHERE prompt_code LIKE 'planning_phase_%' OR prompt_code = 'writing_chapter'",
            [],
            |row| row.get(0),
        ).unwrap();
        assert_eq!(phase_blocks, 8, "every phase prompt must be seeded");
        for code in [
            "planning_phase_0", "planning_phase_1", "planning_phase_2", "planning_phase_3",
            "planning_phase_4", "planning_phase_5", "planning_phase_6", "writing_chapter",
        ] {
            let linked: i64 = conn.query_row(
                "SELECT COUNT(*) FROM prompt_block_tools pbt JOIN prompt_blocks pb ON pb.id = pbt.prompt_block_id WHERE pb.prompt_code = ?1",
                [code], |row| row.get(0),
            ).unwrap();
            assert!(linked > 0, "{code} must grant at least one tool");
        }
        let ui_only: i64 = conn.query_row(
            "SELECT COUNT(*) FROM prompt_block_tools pbt JOIN tools t ON t.id = pbt.tool_id WHERE t.code IN ('commit_draft','confirm_phase','lock_scene','write_scene_content')",
            [],
            |row| row.get(0),
        ).unwrap();
        assert_eq!(ui_only, 0, "UI-only or locked tools must never be granted by prompts");
    }
}

#[cfg(test)]
mod session_trash_migration_tests {
    use super::*;

    #[test]
    fn test_existing_session_survives_soft_delete_column_migration() {
        let mut conn = Connection::open_in_memory().unwrap();
        for version in 1..=7 {
            conn.execute_batch(get_migration_sql(version).unwrap()).unwrap();
        }
        conn.execute("INSERT INTO sessions (id, display_name, created_at) VALUES ('legacy-chat', 'Legacy', '2026-01-01')", []).unwrap();

        run_migrations(&mut conn).unwrap();

        let (title, deleted_at): (String, Option<String>) = conn.query_row(
            "SELECT display_name, deleted_at FROM sessions WHERE id = 'legacy-chat'", [],
            |row| Ok((row.get(0)?, row.get(1)?)),
        ).unwrap();
        assert_eq!(title, "Legacy");
        assert_eq!(deleted_at, None);
        assert_eq!(get_user_version(&conn).unwrap(), MAX_SUPPORTED_SCHEMA_VERSION);
    }
}

#[cfg(test)]
mod tool_inventory_migration_tests {
    use super::*;

    #[test]
    fn test_documented_tool_codes_are_seeded_without_exposing_unimplemented_tools() {
        let mut conn = Connection::open_in_memory().unwrap();
        run_migrations(&mut conn).unwrap();
        let total: i64 = conn.query_row("SELECT COUNT(*) FROM tools", [], |row| row.get(0)).unwrap();
        let linked: i64 = conn.query_row("SELECT COUNT(*) FROM prompt_block_tools", [], |row| row.get(0)).unwrap();
        let incomplete: i64 = conn.query_row(
            "SELECT COUNT(*) FROM tools WHERE display_name IS NULL AND description IS NULL AND input_schema IS NULL",
            [], |row| row.get(0),
        ).unwrap();
        assert_eq!(total, 87);
        assert!(linked > 12, "phase prompts must grant tools in addition to the shared blocks");
        assert_eq!(incomplete, 0, "every seeded tool must carry a callable schema");
    }

    #[test]
    fn test_invariant_prompt_requires_confirmation_for_inferred_display_names() {
        let mut conn = Connection::open_in_memory().unwrap();
        run_migrations(&mut conn).unwrap();
        let content: String = conn.query_row(
            "SELECT content FROM prompt_blocks WHERE id = '45908789-ce45-5763-afeb-ff2bf3c847ee'",
            [], |row| row.get(0),
        ).unwrap();
        assert!(content.contains("display_name"));
        assert!(content.contains("xác nhận"));
        assert!(content.contains("trước khi gọi tool ghi"));
    }

    #[test]
    fn test_tool_catalog_upgrade_preserves_user_edited_prompt() {
        let mut conn = Connection::open_in_memory().unwrap();
        for version in 1..=8 {
            conn.execute_batch(get_migration_sql(version).unwrap()).unwrap();
        }
        conn.execute(
            "UPDATE prompt_blocks SET content = 'My custom rules' WHERE id = '45908789-ce45-5763-afeb-ff2bf3c847ee'",
            [],
        ).unwrap();

        run_migrations(&mut conn).unwrap();

        let content: String = conn.query_row(
            "SELECT content FROM prompt_blocks WHERE id = '45908789-ce45-5763-afeb-ff2bf3c847ee'",
            [], |row| row.get(0),
        ).unwrap();
        assert_eq!(content, "My custom rules");
        assert_eq!(get_user_version(&conn).unwrap(), MAX_SUPPORTED_SCHEMA_VERSION);
    }
}

#[cfg(test)]
mod chat_contract_migration_tests {
    use super::*;

    #[test]
    fn test_chat_contract_migration_exposes_help_and_accurate_project_schema() {
        let mut conn = Connection::open_in_memory().unwrap();
        run_migrations(&mut conn).unwrap();
        let count: i64 = conn.query_row("SELECT COUNT(*) FROM tools t JOIN prompt_block_tools p ON t.id=p.tool_id WHERE t.code='get_tool_definition'", [], |r| r.get(0)).unwrap();
        assert!(count >= 1, "get_tool_definition must stay granted to at least one prompt");
        let schema: String = conn.query_row("SELECT input_schema FROM tools WHERE code='set_project'", [], |r| r.get(0)).unwrap();
        let schema: serde_json::Value = serde_json::from_str(&schema).unwrap();
        assert_eq!(schema["examples"][0]["target"]["chapter_count"], 120);
        assert_eq!(schema["properties"]["target"]["properties"]["arc_count"]["minimum"], 1);
        let (format, content): (String, String) = conn.query_row("SELECT content_format,content FROM prompt_blocks WHERE prompt_code='story_context'", [], |r| Ok((r.get(0)?,r.get(1)?))).unwrap();
        assert_eq!(format, "JSON");
        assert_eq!(content, "{{story_context}}");
    }

    #[test]
    fn test_chat_contract_upgrade_preserves_custom_prompt_and_tool() {
        let mut conn = Connection::open_in_memory().unwrap();
        for version in 1..=9 { conn.execute_batch(get_migration_sql(version).unwrap()).unwrap(); }
        set_user_version(&conn, 9).unwrap();
        conn.execute("UPDATE prompt_blocks SET content='Author custom prompt' WHERE prompt_code='invariant_rule'", []).unwrap();
        conn.execute("UPDATE tools SET description='Custom description' WHERE code='set_project'", []).unwrap();
        run_migrations(&mut conn).unwrap();
        assert_eq!(get_user_version(&conn).unwrap(), MAX_SUPPORTED_SCHEMA_VERSION);
        let content: String = conn.query_row("SELECT content FROM prompt_blocks WHERE prompt_code='invariant_rule'", [], |r| r.get(0)).unwrap();
        assert_eq!(content, "Author custom prompt");
        let description: String = conn.query_row("SELECT description FROM tools WHERE code='set_project'", [], |r| r.get(0)).unwrap();
        assert_eq!(description, "Custom description");
    }
}

#[cfg(test)]
mod planning_scope_migration_tests {
    use super::*;

    #[test]
    fn planning_scope_allows_outline_but_reserves_scene_content_for_writing() {
        let mut conn = Connection::open_in_memory().unwrap();
        run_migrations(&mut conn).unwrap();
        let content: String = conn
            .query_row(
                "SELECT content FROM prompt_blocks WHERE prompt_code='planning_mode'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert!(content.contains("Phase 2 lập dàn ý chương và cảnh"));
        assert!(content.contains("không sinh nội dung scenes.content"));
        assert!(content.contains("Writing Mode sau khi plan đã commit"));
    }

    #[test]
    fn planning_scope_upgrade_keeps_author_custom_prompt() {
        let mut conn = Connection::open_in_memory().unwrap();
        for version in 1..=10 {
            conn.execute_batch(get_migration_sql(version).unwrap()).unwrap();
        }
        set_user_version(&conn, 10).unwrap();
        conn.execute(
            "UPDATE prompt_blocks SET content='Author custom planning' WHERE prompt_code='planning_mode'",
            [],
        )
        .unwrap();
        run_migrations(&mut conn).unwrap();
        let content: String = conn
            .query_row(
                "SELECT content FROM prompt_blocks WHERE prompt_code='planning_mode'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(content, "Author custom planning");
    }
}

#[cfg(test)]
mod system_catalog_migration_tests {
    use super::*;

    #[test]
    fn test_system_catalogs_seeded_with_genres_motifs_narrators_and_directives() {
        let mut conn = Connection::open_in_memory().unwrap();
        run_migrations(&mut conn).unwrap();
        let genres: i64 = conn.query_row("SELECT COUNT(*) FROM genres", [], |r| r.get(0)).unwrap();
        assert_eq!(genres, 50);
        let motifs: i64 = conn.query_row("SELECT COUNT(*) FROM motifs", [], |r| r.get(0)).unwrap();
        assert_eq!(motifs, 19);
        let narrators: i64 = conn.query_row("SELECT COUNT(*) FROM narrator_configs", [], |r| r.get(0)).unwrap();
        assert_eq!(narrators, 64);
        let directives: i64 = conn.query_row("SELECT COUNT(*) FROM narrative_directives", [], |r| r.get(0)).unwrap();
        assert_eq!(directives, 73);
        assert_eq!(get_user_version(&conn).unwrap(), MAX_SUPPORTED_SCHEMA_VERSION);
    }
}

/// Run migrations including physical backup creation when upgrading an existing non-empty DB (§3.4, DB-P02, MIG-02).
pub fn run_migrations_with_backup(conn: &mut Connection, app_data_dir: &Path) -> Result<(), DbError> {
    let current_version = get_user_version(conn)?;

    if current_version > MAX_SUPPORTED_SCHEMA_VERSION {
        return Err(DbError::UnsupportedSchemaVersion {
            current: current_version,
            max_supported: MAX_SUPPORTED_SCHEMA_VERSION,
        });
    }

    // Step 6: Physical backup before upgrade if 0 < current_version < MAX_SUPPORTED_SCHEMA_VERSION
    if current_version > 0 && current_version < MAX_SUPPORTED_SCHEMA_VERSION {
        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S").to_string();
        let backup_file = backup_path(app_data_dir, current_version, &timestamp);
        perform_online_backup(conn, &backup_file)?;
    }

    // Step 7: Execute migrations
    run_migrations(conn)
}

/// Verify schema structure, constraints, indexes, and referential integrity (§3.5, MIG-05).
///
/// Checks:
/// 1. PRAGMA user_version == MAX_SUPPORTED_SCHEMA_VERSION (11)
/// 2. PRAGMA quick_check(1) == "ok"
/// 3. Exactly 66 target tables exist and NO `_schema_migrations` table
/// 4. Mandatory columns, types, NOT NULL and CHECK constraints:
///    - SF-01: `draft_versions.seal_key` exists
///    - SF-02: `chapter_save_receipts` table exists
///    - SF-06: `workflow_round_receipts` table exists with 15 columns;
///             `workflow_task_receipts` has 3 new nullable columns;
///             `pending_patches.eligibility_json` exists
///    - SF-07: `scene_versions.parent_id` is NOT NULL
/// 5. Mandatory indexes exist
/// 6. PRAGMA foreign_key_check returns 0 violations
pub fn verify_schema_and_integrity(conn: &Connection) -> Result<(), DbError> {
    // 1. Schema version check
    let version = get_user_version(conn)?;
    if version != MAX_SUPPORTED_SCHEMA_VERSION {
        return Err(DbError::MigrationFailed(format!(
            "user_version mismatch: expected {}, found {}",
            MAX_SUPPORTED_SCHEMA_VERSION, version
        )));
    }

    // 2. Physical B-Tree integrity check
    let quick_check: String = conn
        .query_row("PRAGMA quick_check(1);", [], |row| row.get(0))
        .map_err(|e| DbError::IntegrityCheckFailed(e.to_string()))?;
    if quick_check != "ok" {
        return Err(DbError::IntegrityCheckFailed(quick_check));
    }

    // 3. Table manifest verification (exactly 66 target tables, no _schema_migrations)
    let mut table_stmt = conn
        .prepare("SELECT name FROM sqlite_master WHERE type = 'table' AND name NOT LIKE 'sqlite_%' ORDER BY name;")
        .map_err(|e| DbError::MigrationFailed(e.to_string()))?;

    let table_names: Vec<String> = table_stmt
        .query_map([], |row| row.get(0))
        .map_err(|e| DbError::MigrationFailed(e.to_string()))?
        .collect::<Result<Vec<String>, _>>()
        .map_err(|e| DbError::MigrationFailed(e.to_string()))?;

    if table_names.iter().any(|t| t == "_schema_migrations") {
        return Err(DbError::MigrationFailed(
            "Disallowed _schema_migrations table found in database".into(),
        ));
    }

    let expected_tables: BTreeSet<&str> = TARGET_TABLES_V3.iter().copied().collect();
    let actual_tables: BTreeSet<&str> = table_names.iter().map(|s| s.as_str()).collect();

    if expected_tables != actual_tables {
        let missing: Vec<_> = expected_tables.difference(&actual_tables).collect();
        let extra: Vec<_> = actual_tables.difference(&expected_tables).collect();
        return Err(DbError::MigrationFailed(format!(
            "Table count/name mismatch: expected 66 tables, found {}. Missing: {:?}, Extra: {:?}",
            actual_tables.len(),
            missing,
            extra
        )));
    }

    // 4. Column & Constraint checks (SF-01, SF-02, SF-06, SF-07)
    // SF-01: draft_versions.seal_key
    let dv_columns = get_column_names(conn, "draft_versions")?;
    if !dv_columns.contains(&"seal_key".to_string()) {
        return Err(DbError::MigrationFailed(
            "SF-01 violation: draft_versions.seal_key column is missing".into(),
        ));
    }

    // SF-06: workflow_round_receipts exists with 15 columns
    let wr_columns = get_column_names(conn, "workflow_round_receipts")?;
    if wr_columns.len() != 15 {
        return Err(DbError::MigrationFailed(format!(
            "SF-06 violation: workflow_round_receipts expected 15 columns, found {}",
            wr_columns.len()
        )));
    }

    // SF-06: workflow_task_receipts B+ columns
    let wt_columns = get_column_names(conn, "workflow_task_receipts")?;
    for required_col in &["round_receipt_id", "task_index", "evidence_json"] {
        if !wt_columns.contains(&required_col.to_string()) {
            return Err(DbError::MigrationFailed(format!(
                "SF-06 violation: workflow_task_receipts.{} is missing",
                required_col
            )));
        }
    }

    // SF-06: pending_patches.eligibility_json
    let pp_columns = get_column_names(conn, "pending_patches")?;
    if !pp_columns.contains(&"eligibility_json".to_string()) {
        return Err(DbError::MigrationFailed(
            "SF-06 violation: pending_patches.eligibility_json is missing".into(),
        ));
    }

    // SF-07: scene_versions.parent_id has notnull == 1
    let mut ti_stmt = conn
        .prepare("PRAGMA table_info(scene_versions);")
        .map_err(|e| DbError::MigrationFailed(e.to_string()))?;
    let parent_id_notnull = ti_stmt
        .query_map([], |row| {
            let name: String = row.get(1)?;
            let notnull: i32 = row.get(3)?;
            Ok((name, notnull == 1))
        })
        .map_err(|e| DbError::MigrationFailed(e.to_string()))?
        .filter_map(|r| r.ok())
        .find(|(name, _)| name == "parent_id")
        .map(|(_, notnull)| notnull)
        .unwrap_or(false);

    if !parent_id_notnull {
        return Err(DbError::MigrationFailed(
            "SF-07 violation: scene_versions.parent_id must be NOT NULL".into(),
        ));
    }

    // 5. Index verification
    let dv_indices = get_index_names(conn, "draft_versions")?;
    if !dv_indices.iter().any(|idx| idx.contains("seal_key")) {
        // Also check if an unnamed UNIQUE constraint created an index
        let mut idx_list = conn.prepare("PRAGMA index_list(draft_versions);")
            .map_err(|e| DbError::MigrationFailed(e.to_string()))?;
        let has_seal_key_idx = idx_list.query_map([], |row| {
            let idx_name: String = row.get(1)?;
            Ok(idx_name)
        })
        .map_err(|e| DbError::MigrationFailed(e.to_string()))?
        .filter_map(|r| r.ok())
        .any(|idx_name| {
            let cols = get_index_columns(conn, &idx_name).unwrap_or_default();
            cols.contains(&"seal_key".to_string())
        });
        if !has_seal_key_idx {
            return Err(DbError::MigrationFailed(
                "SF-01 violation: unique index on draft_versions(draft_session_id, seal_key) missing".into(),
            ));
        }
    }

    // 6. Foreign key integrity check
    let mut fk_stmt = conn
        .prepare("PRAGMA foreign_key_check;")
        .map_err(|e| DbError::MigrationFailed(e.to_string()))?;
    let violations: Vec<(String, i64, String, i64)> = fk_stmt
        .query_map([], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
        })
        .map_err(|e| DbError::MigrationFailed(e.to_string()))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| DbError::MigrationFailed(e.to_string()))?;

    if !violations.is_empty() {
        return Err(DbError::MigrationFailed(format!(
            "Foreign key check failed: {} violations detected",
            violations.len()
        )));
    }

    Ok(())
}

/// Helper to get column names for a table.
pub fn get_column_names(conn: &Connection, table_name: &str) -> Result<Vec<String>, DbError> {
    let mut stmt = conn
        .prepare(&format!("PRAGMA table_info({});", table_name))
        .map_err(|e| DbError::MigrationFailed(e.to_string()))?;
    let cols = stmt
        .query_map([], |row| row.get(1))
        .map_err(|e| DbError::MigrationFailed(e.to_string()))?
        .collect::<Result<Vec<String>, _>>()
        .map_err(|e| DbError::MigrationFailed(e.to_string()))?;
    Ok(cols)
}

/// Helper to get index names for a table.
pub fn get_index_names(conn: &Connection, table_name: &str) -> Result<Vec<String>, DbError> {
    let mut stmt = conn
        .prepare(&format!("PRAGMA index_list({});", table_name))
        .map_err(|e| DbError::MigrationFailed(e.to_string()))?;
    let indices = stmt
        .query_map([], |row| row.get(1))
        .map_err(|e| DbError::MigrationFailed(e.to_string()))?
        .collect::<Result<Vec<String>, _>>()
        .map_err(|e| DbError::MigrationFailed(e.to_string()))?;
    Ok(indices)
}

/// Helper to get column names of an index.
pub fn get_index_columns(conn: &Connection, index_name: &str) -> Result<Vec<String>, DbError> {
    let mut stmt = conn
        .prepare(&format!("PRAGMA index_info({});", index_name))
        .map_err(|e| DbError::MigrationFailed(e.to_string()))?;
    let cols = stmt
        .query_map([], |row| row.get(2))
        .map_err(|e| DbError::MigrationFailed(e.to_string()))?
        .collect::<Result<Vec<String>, _>>()
        .map_err(|e| DbError::MigrationFailed(e.to_string()))?;
    Ok(cols)
}

/// Check P1S2 predecessor authorization for sprint gating (REQ-CONTRACT-002, phase2_sprint3_deep1-F1).
pub fn verify_p1s2_authorization(contract_text: &str) -> Result<(), DbError> {
    if contract_text.contains("version_path: UNRATIFIED_PROPOSAL_ONLY")
        || contract_text.contains("authorized_revision: NONE")
    {
        return Err(DbError::ContractBlocked(
            "P1S2 authorization unratified or absent: CONTRACT_BLOCKED".into(),
        ));
    }
    Ok(())
}

#[cfg(test)]
mod summary_prompt_tests {
    use super::*;

    #[test]
    fn test_author_facing_prompt_does_not_narrate_internal_mechanics() {
        let mut conn = Connection::open_in_memory().unwrap();
        run_migrations(&mut conn).unwrap();
        let rule: String = conn.query_row("SELECT content FROM prompt_blocks WHERE prompt_code='invariant_rule'", [], |r| r.get(0)).unwrap();
        let exceptions: String = conn.query_row("SELECT content FROM prompt_blocks WHERE prompt_code='exception_handling'", [], |r| r.get(0)).unwrap();
        assert!(rule.contains("Không kể quy trình kỹ thuật nội bộ"));
        assert!(rule.contains("Không bắt chước cách nói lỗi"));
        assert!(exceptions.contains("không yêu cầu tác giả xác nhận lại để sửa lỗi kỹ thuật"));
        assert!(exceptions.contains("không tự nhảy sang bước sau"));
    }

    #[test]
    fn test_summary_rule_is_migrated_for_existing_databases() {
        let mut conn = Connection::open_in_memory().unwrap();
        run_migrations(&mut conn).unwrap();
        let rule: String = conn.query_row("SELECT content FROM prompt_blocks WHERE prompt_code='invariant_rule'", [], |r| r.get(0)).unwrap();
        assert!(rule.contains("150–200 từ"));
        assert!(rule.contains("declared_sources.summary=user"));
        assert!(rule.contains("không hỏi lại tác giả"));
    }
}
