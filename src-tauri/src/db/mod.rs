//! SQLite schema, migration, and connection setup.

mod messages;
mod outline;
mod projects;
mod sessions;

#[allow(unused_imports)]
#[cfg(test)]
pub use messages::count_messages;
pub use messages::{append_message, list_messages};
#[allow(unused_imports)]
pub use outline::{
    OutlineRun, OutlineStoredItem, create_run, delete_items_for_shard, list_items, run_for_session,
    set_item_status, update_run_stage, update_run_status, update_run_usage, upsert_items,
};
#[cfg(test)]
pub use projects::list_projects;
pub use projects::{
    create_project, ensure_project, list_project_summaries, project_exists, project_summary,
};
pub use sessions::{
    bind_session_to_project_if_unbound, delete_session, ensure_session, get_session, list_sessions,
    project_for_session, rename_session,
};

use crate::paths;
use rusqlite::Connection;

pub(super) const SCHEMA_VERSION: i64 = 2;

pub(super) const SCHEMA: &str = r#"
CREATE TABLE IF NOT EXISTS projects (
    id         TEXT PRIMARY KEY,
    title      TEXT NOT NULL,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS sessions (
    id         TEXT PRIMARY KEY,
    project_id TEXT NOT NULL,
    title      TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS messages (
    id           TEXT PRIMARY KEY,
    session_id   TEXT NOT NULL,
    role         TEXT NOT NULL,
    content      TEXT NOT NULL,
    payload_json TEXT,
    created_at   TEXT NOT NULL
);

-- rowid cannot appear in an index definition; ordering by rowid still works.
CREATE INDEX IF NOT EXISTS idx_messages_session ON messages (session_id);
"#;

pub(super) const OUTLINE_SCHEMA: &str = r#"
CREATE TABLE IF NOT EXISTS outline_runs (
    id TEXT PRIMARY KEY,
    project_id TEXT NOT NULL,
    session_id TEXT NOT NULL,
    stage TEXT NOT NULL,
    status TEXT NOT NULL,
    calls INTEGER NOT NULL DEFAULT 0,
    tokens_in INTEGER NOT NULL DEFAULT 0,
    tokens_out INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS outline_items (
    id TEXT PRIMARY KEY,
    run_id TEXT NOT NULL,
    stage TEXT NOT NULL,
    lane TEXT NOT NULL,
    shard TEXT NOT NULL DEFAULT '',
    key TEXT NOT NULL,
    position INTEGER NOT NULL,
    payload_json TEXT NOT NULL,
    status TEXT NOT NULL,
    error TEXT,
    created_at TEXT NOT NULL,
    UNIQUE (run_id, stage, key)
);
CREATE INDEX IF NOT EXISTS idx_outline_items_run ON outline_items (run_id, stage, status);
CREATE INDEX IF NOT EXISTS idx_outline_runs_project ON outline_runs (project_id, session_id);
"#;

/// Opens (creating if needed) the local SQLite database.
///
/// A connection per operation is fine here: WAL plus a busy timeout keeps
/// concurrent writers safe, and no connection is ever held across an `await`.
pub fn open() -> Result<Connection, String> {
    let path = paths::db_path()?;
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|error| format!("Cannot create data directory: {error}"))?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let _ = std::fs::set_permissions(parent, std::fs::Permissions::from_mode(0o700));
        }
    }
    let mut connection =
        Connection::open(&path).map_err(|error| format!("Cannot open database: {error}"))?;
    connection
        .execute_batch("PRAGMA foreign_keys=ON; PRAGMA busy_timeout=5000;")
        .map_err(|error| format!("Cannot configure database: {error}"))?;
    migrate(&mut connection)?;
    connection
        .execute_batch("PRAGMA journal_mode=WAL;")
        .map_err(|error| format!("Cannot configure database: {error}"))?;
    Ok(connection)
}

fn migrate(connection: &mut Connection) -> Result<(), String> {
    let transaction = connection
        .transaction()
        .map_err(|error| format!("Cannot start schema migration: {error}"))?;
    let version: i64 = transaction
        .query_row("PRAGMA user_version", [], |row| row.get(0))
        .map_err(|error| format!("Cannot read database schema version: {error}"))?;
    match version {
        0 => {
            // Existing message rowids (including those shifted by historical
            // REPLACE writes) are left untouched; there is no data backfill.
            transaction
                .execute_batch(SCHEMA)
                .map_err(|error| format!("Cannot create schema: {error}"))?;
            transaction
                .execute_batch(OUTLINE_SCHEMA)
                .map_err(|error| format!("Cannot create outline schema: {error}"))?;
            transaction
                .execute_batch(&format!("PRAGMA user_version = {SCHEMA_VERSION};"))
                .map_err(|error| format!("Cannot set database schema version: {error}"))?;
        }
        1 => {
            transaction
                .execute_batch(OUTLINE_SCHEMA)
                .map_err(|error| format!("Cannot create outline schema: {error}"))?;
            transaction
                .execute_batch("PRAGMA user_version = 2;")
                .map_err(|error| format!("Cannot set database schema version: {error}"))?;
        }
        SCHEMA_VERSION => {
            transaction
                .execute_batch(SCHEMA)
                .map_err(|error| format!("Cannot create schema: {error}"))?;
            transaction
                .execute_batch(OUTLINE_SCHEMA)
                .map_err(|error| format!("Cannot create outline schema: {error}"))?;
        }
        _ => {
            return Err(format!(
                "database was created by a newer version ({version})"
            ));
        }
    }
    transaction
        .commit()
        .map_err(|error| format!("Cannot commit schema migration: {error}"))
}

pub(super) fn now() -> String {
    chrono::Utc::now().to_rfc3339()
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::MessageEnvelope;
    #[test]
    fn test_open_migrates_v0_and_reopening_is_idempotent() {
        let _home = crate::test_support::TempHome::new("db-schema-version");
        let connection = open().unwrap();
        let version: i64 = connection
            .query_row("PRAGMA user_version", [], |row| row.get(0))
            .unwrap();
        assert_eq!(version, 2);
        append_message(
            &connection,
            "session-a",
            &MessageEnvelope::user("one", "original"),
        )
        .unwrap();
        drop(connection);
        let connection = open().unwrap();
        let version: i64 = connection
            .query_row("PRAGMA user_version", [], |row| row.get(0))
            .unwrap();
        assert_eq!(version, 2);
        assert_eq!(
            list_messages(&connection, "session-a").unwrap()[0].id,
            "one"
        );
    }
    #[test]
    fn test_open_migrates_v1_database_to_v2() {
        let _home = crate::test_support::TempHome::new("db-v1-to-v2");
        let path = paths::db_path().unwrap();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }
        let legacy = Connection::open(&path).unwrap();
        legacy.execute_batch(SCHEMA).unwrap();
        legacy
            .execute(
                "INSERT INTO projects (id, title, created_at) VALUES ('kept', 'Kept', 'now')",
                [],
            )
            .unwrap();
        legacy.execute_batch("PRAGMA user_version = 1;").unwrap();
        drop(legacy);
        let connection = open().unwrap();
        let version: i64 = connection
            .query_row("PRAGMA user_version", [], |row| row.get(0))
            .unwrap();
        assert_eq!(version, 2);
        let title: String = connection
            .query_row("SELECT title FROM projects WHERE id='kept'", [], |row| {
                row.get(0)
            })
            .unwrap();
        assert_eq!(title, "Kept");
        for table in ["outline_runs", "outline_items"] {
            let exists: bool = connection
                .query_row(
                    "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE type='table' AND name=?1)",
                    [table],
                    |row| row.get(0),
                )
                .unwrap();
            assert!(exists, "missing {table}");
        }
    }

    #[test]
    fn test_open_rejects_newer_schema_without_modifying_it() {
        let _home = crate::test_support::TempHome::new("db-newer-schema");
        let connection = open().unwrap();
        connection
            .execute_batch("PRAGMA user_version = 3;")
            .unwrap();
        drop(connection);
        let error = open().unwrap_err();
        assert!(
            error.contains("database was created by a newer version"),
            "{error}"
        );
        let connection = Connection::open(paths::db_path().unwrap()).unwrap();
        let version: i64 = connection
            .query_row("PRAGMA user_version", [], |row| row.get(0))
            .unwrap();
        assert_eq!(version, 3);
    }
}
