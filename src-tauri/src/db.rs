use rusqlite::{Connection, params};

use crate::{models::MessageEnvelope, paths};

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
    let connection =
        Connection::open(&path).map_err(|error| format!("Cannot open database: {error}"))?;
    connection
        .execute_batch(
            "PRAGMA journal_mode=WAL;\n\
             PRAGMA foreign_keys=ON;\n\
             PRAGMA busy_timeout=5000;\n",
        )
        .map_err(|error| format!("Cannot configure database: {error}"))?;
    connection
        .execute_batch(SCHEMA)
        .map_err(|error| format!("Cannot create schema: {error}"))?;
    Ok(connection)
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

/// Creates a project row. The caller (`project_create`) has already resolved a
/// unique id, so this never needs to overwrite an existing title.
pub fn create_project(
    connection: &Connection,
    project_id: &str,
    title: &str,
) -> Result<(), String> {
    connection
        .execute(
            "INSERT OR IGNORE INTO projects (id, title, created_at) VALUES (?1, ?2, ?3)",
            params![project_id, title, now()],
        )
        .map_err(|error| format!("Cannot create project: {error}"))?;
    Ok(())
}

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

/// Binds a session to a project (D1).
///
/// Written as an upsert: the agent loop always creates the session row first, but
/// `project_create` is also reachable through the `execute_tool` command, where a
/// caller-supplied session id might not exist yet. Creating the row here keeps
/// the binding from silently going nowhere.
pub fn bind_session_to_project(
    connection: &Connection,
    session_id: &str,
    project_id: &str,
) -> Result<(), String> {
    connection
        .execute(
            "INSERT INTO sessions (id, project_id, title, created_at) VALUES (?1, ?2, NULL, ?3) \
             ON CONFLICT(id) DO UPDATE SET project_id = excluded.project_id",
            params![session_id, project_id, now()],
        )
        .map_err(|error| format!("Cannot bind session to project: {error}"))?;
    Ok(())
}

pub fn append_message(
    connection: &Connection,
    session_id: &str,
    message: &MessageEnvelope,
) -> Result<(), String> {
    let payload = serde_json::to_string(message)
        .map_err(|error| format!("Cannot encode message: {error}"))?;
    connection
        .execute(
            "INSERT OR REPLACE INTO messages \
             (id, session_id, role, content, payload_json, created_at) \
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
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
            "SELECT role, content, payload_json FROM messages \
             WHERE session_id = ?1 ORDER BY rowid ASC",
        )
        .map_err(|error| format!("Cannot read messages: {error}"))?;
    let rows = statement
        .query_map(params![session_id], |row| {
            let role: String = row.get(0)?;
            let content: String = row.get(1)?;
            let payload: Option<String> = row.get(2)?;
            Ok((role, content, payload))
        })
        .map_err(|error| format!("Cannot read messages: {error}"))?;

    let mut messages = Vec::new();
    for row in rows {
        let (role, content, payload) =
            row.map_err(|error| format!("Cannot read message row: {error}"))?;
        messages.push(match payload {
            Some(raw) => serde_json::from_str::<MessageEnvelope>(&raw).unwrap_or_else(|_| {
                MessageEnvelope::new(format!("recovered-{}", messages.len()), role, content)
            }),
            None => MessageEnvelope::new(format!("recovered-{}", messages.len()), role, content),
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
        let rows = statement.query_map([], |row| row.get::<_, String>(0)).unwrap();
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
        let tool = MessageEnvelope::tool_result(
            "m3",
            "call_1",
            "super_json_set",
            &json!({"ok": true}),
        );
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
        assert_eq!(listed[1].tool_calls.as_ref().unwrap()[0].name, "super_json_set");
        assert_eq!(listed[2].tool_call_id.as_deref(), Some("call_1"));
        assert_eq!(listed[2].name.as_deref(), Some("super_json_set"));

        // Sessions are isolated from each other.
        assert!(list_messages(&connection, "sess-other").unwrap().is_empty());
    }

    #[test]
    fn test_ensure_helpers_are_idempotent() {
        let _home = crate::test_support::TempHome::new("db-idempotent");
        let connection = open().unwrap();
        ensure_project(&connection, "default", "First title").unwrap();
        ensure_project(&connection, "default", "Second title").unwrap();
        let title: String = connection
            .query_row("SELECT title FROM projects WHERE id = 'default'", [], |row| {
                row.get(0)
            })
            .unwrap();
        assert_eq!(title, "First title");
    }
}
