use rusqlite::{Connection, ErrorCode, params};

use crate::{models::MessageEnvelope, paths};

const SCHEMA_VERSION: i64 = 1;

const SCHEMA: &str = r#"
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
                .execute_batch(&format!("PRAGMA user_version = {SCHEMA_VERSION};"))
                .map_err(|error| format!("Cannot set database schema version: {error}"))?;
        }
        SCHEMA_VERSION => {
            transaction
                .execute_batch(SCHEMA)
                .map_err(|error| format!("Cannot create schema: {error}"))?;
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

fn now() -> String {
    chrono::Utc::now().to_rfc3339()
}

pub fn ensure_project(
    connection: &Connection,
    project_id: &str,
    title: &str,
) -> Result<(), String> {
    connection
        .execute(
            "INSERT OR IGNORE INTO projects (id, title, created_at) VALUES (?1, ?2, ?3)",
            params![project_id, title, now()],
        )
        .map_err(|error| format!("Cannot ensure project: {error}"))?;
    Ok(())
}

/// Reserve an ID without changing an existing project's title. Only an ID
/// primary-key conflict is a retryable collision; other constraints are errors.
pub fn create_project(
    connection: &Connection,
    project_id: &str,
    title: &str,
) -> Result<bool, String> {
    match connection.execute(
        "INSERT INTO projects (id, title, created_at) VALUES (?1, ?2, ?3)",
        params![project_id, title, now()],
    ) {
        Ok(_) => Ok(true),
        Err(rusqlite::Error::SqliteFailure(error, _))
            if error.code == ErrorCode::ConstraintViolation
                && error.extended_code == rusqlite::ffi::SQLITE_CONSTRAINT_PRIMARYKEY =>
        {
            Ok(false)
        }
        Err(error) => Err(format!("Cannot create project: {error}")),
    }
}

#[cfg(test)]
pub fn list_projects(connection: &Connection) -> Result<Vec<String>, String> {
    let mut statement = connection
        .prepare("SELECT id FROM projects ORDER BY created_at ASC, rowid ASC")
        .map_err(|error| format!("Cannot list projects: {error}"))?;
    let rows = statement
        .query_map([], |row| row.get::<_, String>(0))
        .map_err(|error| format!("Cannot list projects: {error}"))?;
    let mut ids = Vec::new();
    for row in rows {
        ids.push(row.map_err(|error| format!("Cannot read project row: {error}"))?);
    }
    Ok(ids)
}

/// `sessions.project_id` is `NOT NULL`, and an unbound session stores the empty
/// string (D1). `ensure_session` therefore accepts an empty `project_id`: the row
/// is still created, but no project is implied by it.
///
/// `INSERT OR IGNORE` is deliberate. A session is the single source of truth for
/// "which project this chat belongs to", so a later turn that arrives with an
/// empty `project_id` must not unbind a session that already has one.
pub fn ensure_session(
    connection: &Connection,
    session_id: &str,
    project_id: &str,
) -> Result<(), String> {
    connection
        .execute(
            "INSERT OR IGNORE INTO sessions (id, project_id, title, created_at) \
             VALUES (?1, ?2, NULL, ?3)",
            params![session_id, project_id, now()],
        )
        .map_err(|error| format!("Cannot ensure session: {error}"))?;
    Ok(())
}

/// The project a session belongs to, or `None` when the session is unknown or
/// not bound yet. An unbound session stores `''`; that is normalised to `None` so
/// callers never have to tell "no project" from "empty project id".
pub fn project_for_session(
    connection: &Connection,
    session_id: &str,
) -> Result<Option<String>, String> {
    let result = connection.query_row(
        "SELECT project_id FROM sessions WHERE id = ?1",
        params![session_id],
        |row| row.get::<_, String>(0),
    );
    let project_id = match result {
        Ok(value) => value,
        Err(rusqlite::Error::QueryReturnedNoRows) => return Ok(None),
        Err(error) => return Err(format!("Cannot read session: {error}")),
    };
    Ok(Some(project_id).filter(|value| !value.trim().is_empty()))
}

/// Insert a missing session or bind an unbound one without ever replacing its
/// existing project. `execute_tool` may receive a session not yet in SQLite.
pub fn bind_session_to_project_if_unbound(
    connection: &Connection,
    session_id: &str,
    project_id: &str,
) -> Result<bool, String> {
    let changed = connection
        .execute(
            "INSERT INTO sessions (id, project_id, title, created_at) VALUES (?1, ?2, NULL, ?3) \
             ON CONFLICT(id) DO UPDATE SET project_id = excluded.project_id \
             WHERE sessions.project_id IS NULL OR sessions.project_id = ''",
            params![session_id, project_id, now()],
        )
        .map_err(|error| format!("Cannot bind session to project: {error}"))?;
    Ok(changed != 0)
}

pub fn append_message(
    connection: &Connection,
    session_id: &str,
    message: &MessageEnvelope,
) -> Result<(), String> {
    let payload = serde_json::to_string(message)
        .map_err(|error| format!("Cannot encode message: {error}"))?;
    let changed = connection
        .execute(
            "INSERT INTO messages (id, session_id, role, content, payload_json, created_at) \
             VALUES (?1, ?2, ?3, ?4, ?5, ?6) \
             ON CONFLICT(id) DO UPDATE SET \
                 role = excluded.role, \
                 content = excluded.content, \
                 payload_json = excluded.payload_json \
             WHERE messages.session_id = excluded.session_id",
            params![
                message.id,
                session_id,
                message.role,
                message.content,
                payload,
                now()
            ],
        )
        .map_err(|error| format!("Cannot append message: {error}"))?;
    if changed == 0 {
        let stored_session: String = connection
            .query_row(
                "SELECT session_id FROM messages WHERE id = ?1",
                params![message.id],
                |row| row.get(0),
            )
            .map_err(|error| {
                format!("Cannot read conflicting message '{}': {error}", message.id)
            })?;
        return Err(format!(
            "Cannot append message '{}': ID already belongs to session '{}', not requested session '{}'",
            message.id, stored_session, session_id
        ));
    }
    Ok(())
}

/// Returns the transcript in insertion order.
///
/// `created_at` has millisecond resolution and a tool loop can append several
/// messages inside the same millisecond, so ordering uses SQLite's implicit
/// `rowid` instead.
pub fn list_messages(
    connection: &Connection,
    session_id: &str,
) -> Result<Vec<MessageEnvelope>, String> {
    let mut statement = connection
        .prepare(
            "SELECT id, role, content, payload_json FROM messages \
             WHERE session_id = ?1 ORDER BY rowid ASC",
        )
        .map_err(|error| format!("Cannot read messages: {error}"))?;
    let rows = statement
        .query_map(params![session_id], |row| {
            let id: String = row.get(0)?;
            let role: String = row.get(1)?;
            let content: String = row.get(2)?;
            let payload: Option<String> = row.get(3)?;
            Ok((id, role, content, payload))
        })
        .map_err(|error| format!("Cannot read messages: {error}"))?;

    let mut messages = Vec::new();
    for row in rows {
        let (id, role, content, payload) =
            row.map_err(|error| format!("Cannot read message row: {error}"))?;
        messages.push(match payload {
            Some(raw) => {
                let mut message =
                    serde_json::from_str::<MessageEnvelope>(&raw).map_err(|error| {
                        format!("Cannot decode message '{id}' in session '{session_id}': {error}")
                    })?;
                // The primary key, not the JSON copy, is authoritative.
                message.id = id;
                message
            }
            None => MessageEnvelope::new(id, role, content),
        });
    }
    Ok(messages)
}

/// Message count for a session. Only the tests need it so far; kept next to
/// `list_messages` because they share the same query shape.
#[cfg(test)]
pub fn count_messages(connection: &Connection, session_id: &str) -> Result<u64, String> {
    // rusqlite has no `FromSql` for u64, so read an i64 and widen.
    let count: i64 = connection
        .query_row(
            "SELECT COUNT(*) FROM messages WHERE session_id = ?1",
            params![session_id],
            |row| row.get(0),
        )
        .map_err(|error| format!("Cannot count messages: {error}"))?;
    Ok(u64::try_from(count).unwrap_or(0))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{MessageEnvelope, ToolCall};
    use serde_json::json;

    fn table_names(connection: &Connection) -> Vec<String> {
        let mut statement = connection
            .prepare("SELECT name FROM sqlite_master WHERE type = 'table' ORDER BY name")
            .unwrap();
        let rows = statement
            .query_map([], |row| row.get::<_, String>(0))
            .unwrap();
        rows.map(Result::unwrap).collect()
    }

    #[test]
    fn test_db_message_round_trip_and_tables() {
        let _home = crate::test_support::TempHome::new("db");
        let connection = open().unwrap();

        let tables = table_names(&connection);
        for expected in ["projects", "sessions", "messages"] {
            assert!(tables.contains(&expected.to_string()), "missing {expected}");
        }

        ensure_project(&connection, "default", "Default project").unwrap();
        ensure_session(&connection, "sess-1", "default").unwrap();

        let user = MessageEnvelope::user("m1", "Đặt tên project là Nửa Đêm");
        let mut assistant = MessageEnvelope::new("m2", "assistant", "");
        assistant.tool_calls = Some(vec![ToolCall {
            id: "call_1".into(),
            name: "super_json_set".into(),
            arguments: json!({"path": "require.overview.name", "value": "Nửa Đêm"}),
        }]);
        let tool =
            MessageEnvelope::tool_result("m3", "call_1", "super_json_set", &json!({"ok": true}));
        let final_message = MessageEnvelope::new("m4", "assistant", "Đã đặt tên xong.");

        for message in [&user, &assistant, &tool, &final_message] {
            append_message(&connection, "sess-1", message).unwrap();
        }

        let listed = list_messages(&connection, "sess-1").unwrap();
        assert_eq!(listed.len(), 4);
        assert_eq!(
            listed.iter().map(|m| m.role.as_str()).collect::<Vec<_>>(),
            vec!["user", "assistant", "tool", "assistant"]
        );
        assert_eq!(count_messages(&connection, "sess-1").unwrap(), 4);

        // Tool wiring must survive the SQLite round trip.
        assert_eq!(
            listed[1].tool_calls.as_ref().unwrap()[0].name,
            "super_json_set"
        );
        assert_eq!(listed[2].tool_call_id.as_deref(), Some("call_1"));
        assert_eq!(listed[2].name.as_deref(), Some("super_json_set"));

        // Sessions are isolated from each other.
        assert!(list_messages(&connection, "sess-other").unwrap().is_empty());
    }

    #[test]
    fn test_append_message_replay_preserves_count_order_and_created_at() {
        let _home = crate::test_support::TempHome::new("db-message-replay");
        let connection = open().unwrap();
        for id in ["first", "middle", "last"] {
            append_message(&connection, "session-a", &MessageEnvelope::user(id, id)).unwrap();
        }
        connection
            .execute(
                "UPDATE messages SET created_at = 'original-timestamp' WHERE id = 'middle'",
                [],
            )
            .unwrap();
        let before: Vec<(i64, String)> = connection
            .prepare("SELECT rowid, id FROM messages ORDER BY rowid ASC")
            .unwrap()
            .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))
            .unwrap()
            .map(Result::unwrap)
            .collect();

        for _ in 0..2 {
            append_message(
                &connection,
                "session-a",
                &MessageEnvelope::user("middle", "updated"),
            )
            .unwrap();
        }

        let after: Vec<(i64, String)> = connection
            .prepare("SELECT rowid, id FROM messages ORDER BY rowid ASC")
            .unwrap()
            .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))
            .unwrap()
            .map(Result::unwrap)
            .collect();
        assert_eq!(count_messages(&connection, "session-a").unwrap(), 3);
        assert_eq!(
            after, before,
            "replay must keep each rowid and transcript order"
        );
        assert_eq!(
            list_messages(&connection, "session-a")
                .unwrap()
                .iter()
                .map(|message| message.id.as_str())
                .collect::<Vec<_>>(),
            vec!["first", "middle", "last"]
        );
        let (created_at, content): (String, String) = connection
            .query_row(
                "SELECT created_at, content FROM messages WHERE id = 'middle'",
                [],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .unwrap();
        assert_eq!(created_at, "original-timestamp");
        assert_eq!(content, "updated");
    }

    #[test]
    fn test_append_message_rejects_cross_session_duplicate() {
        let _home = crate::test_support::TempHome::new("db-cross-session-id");
        let connection = open().unwrap();
        append_message(
            &connection,
            "session-a",
            &MessageEnvelope::user("shared-id", "original"),
        )
        .unwrap();

        let error = append_message(
            &connection,
            "session-b",
            &MessageEnvelope::user("shared-id", "replacement"),
        )
        .unwrap_err();

        assert_eq!(
            error,
            "Cannot append message 'shared-id': ID already belongs to session 'session-a', not requested session 'session-b'"
        );
        assert_eq!(count_messages(&connection, "session-a").unwrap(), 1);
        assert_eq!(count_messages(&connection, "session-b").unwrap(), 0);
        assert_eq!(
            list_messages(&connection, "session-a").unwrap()[0].content,
            "original"
        );
    }

    #[test]
    fn test_list_messages_reconstructs_null_payload_with_stored_id() {
        let _home = crate::test_support::TempHome::new("db-legacy-null-payload");
        let connection = open().unwrap();
        connection
            .execute(
                "INSERT INTO messages (id, session_id, role, content, payload_json, created_at) \
                 VALUES ('legacy-id', 'session-a', 'tool', 'legacy content', NULL, 'original')",
                [],
            )
            .unwrap();

        let listed = list_messages(&connection, "session-a").unwrap();
        assert_eq!(listed.len(), 1);
        assert_eq!(listed[0].id, "legacy-id");
        assert_eq!(listed[0].role, "tool");
        assert_eq!(listed[0].content, "legacy content");
    }

    #[test]
    fn test_list_messages_reports_corrupt_payload_instead_of_recovery() {
        let _home = crate::test_support::TempHome::new("db-corrupt-payload");
        let connection = open().unwrap();
        connection
            .execute(
                "INSERT INTO messages (id, session_id, role, content, payload_json, created_at) \
                 VALUES ('corrupt-id', 'session-a', 'user', 'original', '{invalid', 'original')",
                [],
            )
            .unwrap();

        let error = list_messages(&connection, "session-a").unwrap_err();
        assert!(error.contains("corrupt-id"), "{error}");
        assert!(error.contains("session-a"), "{error}");
    }

    #[test]
    fn test_list_messages_uses_stored_id_not_payload_id() {
        let _home = crate::test_support::TempHome::new("db-mismatched-payload-id");
        let connection = open().unwrap();
        append_message(
            &connection,
            "session-a",
            &MessageEnvelope::user("stored-id", "original"),
        )
        .unwrap();
        let payload =
            serde_json::to_string(&MessageEnvelope::user("wrong-id", "original")).unwrap();
        connection
            .execute(
                "UPDATE messages SET payload_json = ?1 WHERE id = 'stored-id'",
                params![payload],
            )
            .unwrap();
        assert_eq!(
            list_messages(&connection, "session-a").unwrap()[0].id,
            "stored-id"
        );
    }

    #[test]
    fn test_open_migrates_v0_and_reopening_is_idempotent() {
        let _home = crate::test_support::TempHome::new("db-schema-version");
        let connection = open().unwrap();
        let version: i64 = connection
            .query_row("PRAGMA user_version", [], |row| row.get(0))
            .unwrap();
        assert_eq!(version, 1);
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
        assert_eq!(version, 1);
        assert_eq!(
            list_messages(&connection, "session-a").unwrap()[0].id,
            "one"
        );
    }

    #[test]
    fn test_open_rejects_newer_schema_without_modifying_it() {
        let _home = crate::test_support::TempHome::new("db-newer-schema");
        let connection = open().unwrap();
        connection
            .execute_batch("PRAGMA user_version = 2;")
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
        assert_eq!(version, 2);
    }

    #[test]
    fn test_create_project_reports_id_conflict_without_changing_row() {
        let _home = crate::test_support::TempHome::new("db-project-conflict");
        let connection = open().unwrap();
        assert!(create_project(&connection, "same", "First title").unwrap());
        assert!(!create_project(&connection, "same", "Second title").unwrap());
        let titles: Vec<String> = connection
            .prepare("SELECT title FROM projects WHERE id = 'same'")
            .unwrap()
            .query_map([], |row| row.get(0))
            .unwrap()
            .map(Result::unwrap)
            .collect();
        assert_eq!(titles, ["First title"]);
    }

    #[test]
    fn test_conditional_bind_preserves_existing_project_and_inserts_missing_session() {
        let _home = crate::test_support::TempHome::new("db-conditional-bind");
        let connection = open().unwrap();
        assert!(bind_session_to_project_if_unbound(&connection, "missing", "first").unwrap());
        assert!(!bind_session_to_project_if_unbound(&connection, "missing", "second").unwrap());
        assert_eq!(project_for_session(&connection, "missing").unwrap().as_deref(), Some("first"));
        ensure_session(&connection, "unbound", "").unwrap();
        assert!(bind_session_to_project_if_unbound(&connection, "unbound", "third").unwrap());
        assert_eq!(project_for_session(&connection, "unbound").unwrap().as_deref(), Some("third"));
    }

    #[test]
    fn test_ensure_helpers_are_idempotent() {
        let _home = crate::test_support::TempHome::new("db-idempotent");
        let connection = open().unwrap();
        ensure_project(&connection, "default", "First title").unwrap();
        ensure_project(&connection, "default", "Second title").unwrap();
        let title: String = connection
            .query_row(
                "SELECT title FROM projects WHERE id = 'default'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(title, "First title");
    }
}
