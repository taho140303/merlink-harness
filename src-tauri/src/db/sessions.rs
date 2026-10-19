//! Session rows: create-or-ignore, project lookup, and conditional binding.

use rusqlite::{Connection, params};

use super::now;
use crate::models::SessionSummary;

fn display_title(stored: Option<String>, first_user: Option<String>) -> String {
    if let Some(title) = stored
        .map(|title| title.trim().to_string())
        .filter(|title| !title.is_empty())
    {
        return title;
    }
    let collapsed = first_user
        .unwrap_or_default()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    if collapsed.is_empty() {
        return "New conversation".into();
    }
    let mut chars = collapsed.chars();
    let prefix: String = chars.by_ref().take(60).collect();
    if chars.next().is_some() {
        format!("{prefix}…")
    } else {
        prefix
    }
}

const SESSION_SUMMARY_SELECT: &str = "SELECT s.id, s.project_id, p.title, s.title, s.created_at, \
    (SELECT m.content FROM messages m WHERE m.session_id = s.id AND m.role = 'user' ORDER BY m.rowid ASC LIMIT 1) \
    FROM sessions s LEFT JOIN projects p ON p.id = s.project_id AND s.project_id <> ''";

fn read_summary(row: &rusqlite::Row<'_>) -> rusqlite::Result<SessionSummary> {
    let stored: Option<String> = row.get(3)?;
    let first_user: Option<String> = row.get(5)?;
    Ok(SessionSummary {
        id: row.get(0)?,
        project_id: row.get(1)?,
        project_title: row.get(2)?,
        title: display_title(stored, first_user),
        created_at: row.get(4)?,
    })
}

pub fn list_sessions(connection: &Connection) -> Result<Vec<SessionSummary>, String> {
    let mut statement = connection
        .prepare(&format!(
            "{SESSION_SUMMARY_SELECT} ORDER BY s.created_at DESC, s.rowid DESC"
        ))
        .map_err(|error| format!("Cannot prepare session list: {error}"))?;
    let rows = statement
        .query_map([], read_summary)
        .map_err(|error| format!("Cannot list sessions: {error}"))?;
    rows.collect::<Result<Vec<_>, _>>()
        .map_err(|error| format!("Cannot read sessions: {error}"))
}

pub fn get_session(
    connection: &Connection,
    session_id: &str,
) -> Result<Option<SessionSummary>, String> {
    let mut statement = connection
        .prepare(&format!("{SESSION_SUMMARY_SELECT} WHERE s.id = ?1"))
        .map_err(|error| format!("Cannot prepare session lookup: {error}"))?;
    match statement.query_row([session_id], read_summary) {
        Ok(summary) => Ok(Some(summary)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(error) => Err(format!("Cannot read session: {error}")),
    }
}

pub fn rename_session(
    connection: &Connection,
    session_id: &str,
    title: &str,
) -> Result<SessionSummary, String> {
    let title = title.trim();
    if title.is_empty() {
        return Err("Session title cannot be empty".into());
    }
    if title.chars().count() > 120 {
        return Err("Session title is too long (max 120 characters)".into());
    }
    let changed = connection
        .execute(
            "UPDATE sessions SET title = ?1 WHERE id = ?2",
            params![title, session_id],
        )
        .map_err(|error| format!("Cannot rename session: {error}"))?;
    if changed == 0 {
        return Err(format!("Session not found: {session_id}"));
    }
    get_session(connection, session_id)?.ok_or_else(|| format!("Session not found: {session_id}"))
}

pub fn delete_session(connection: &Connection, session_id: &str) -> Result<bool, String> {
    let transaction = connection
        .unchecked_transaction()
        .map_err(|error| format!("Cannot start session deletion: {error}"))?;
    let exists: bool = transaction
        .query_row(
            "SELECT EXISTS(SELECT 1 FROM sessions WHERE id = ?1)",
            [session_id],
            |row| row.get(0),
        )
        .map_err(|error| format!("Cannot check session: {error}"))?;
    if !exists {
        return Ok(false);
    }
    transaction
        .execute("DELETE FROM messages WHERE session_id = ?1", [session_id])
        .map_err(|error| format!("Cannot delete session messages: {error}"))?;
    transaction
        .execute("DELETE FROM sessions WHERE id = ?1", [session_id])
        .map_err(|error| format!("Cannot delete session: {error}"))?;
    transaction
        .commit()
        .map_err(|error| format!("Cannot commit session deletion: {error}"))?;
    Ok(true)
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

#[cfg(test)]
mod tests {
    use super::super::open;
    use super::{
        bind_session_to_project_if_unbound, delete_session, ensure_session, get_session,
        list_sessions, project_for_session, rename_session,
    };
    fn add_message(
        connection: &rusqlite::Connection,
        session: &str,
        id: &str,
        role: &str,
        content: &str,
    ) {
        connection.execute("INSERT INTO messages (id, session_id, role, content, payload_json, created_at) VALUES (?1, ?2, ?3, ?4, NULL, ?5)", rusqlite::params![id, session, role, content, super::super::now()]).unwrap();
    }

    #[test]
    fn test_list_sessions_orders_by_created_at_desc_and_falls_back_to_first_user_message() {
        let _home = crate::test_support::TempHome::new("session-list-order");
        let connection = open().unwrap();
        connection.execute("INSERT INTO sessions (id, project_id, title, created_at) VALUES ('old', '', NULL, '2024-01-01'), ('new', '', NULL, '2024-02-01'), ('tie-a', '', 'stored', '2024-03-01'), ('tie-b', '', NULL, '2024-03-01')", []).unwrap();
        add_message(
            &connection,
            "old",
            "u1",
            "user",
            "  first\n   user   text  ",
        );
        add_message(&connection, "old", "a1", "assistant", "ignore");
        add_message(&connection, "old", "u2", "user", "later user");
        add_message(&connection, "tie-b", "u3", "user", &"x".repeat(61));
        let list = list_sessions(&connection).unwrap();
        assert_eq!(
            list.iter().map(|s| s.id.as_str()).collect::<Vec<_>>(),
            ["tie-b", "tie-a", "new", "old"]
        );
        assert_eq!(list[3].title, "first user text");
        assert_eq!(list[2].title, "New conversation");
        assert_eq!(list[1].title, "stored");
        assert_eq!(list[0].title, format!("{}…", "x".repeat(60)));
    }

    #[test]
    fn test_list_sessions_includes_project_title_and_unbound_group() {
        let _home = crate::test_support::TempHome::new("session-project-title");
        let connection = open().unwrap();
        connection.execute("INSERT INTO projects (id, title, created_at) VALUES ('p1', 'Project One', '2024-01-01')", []).unwrap();
        connection.execute("INSERT INTO sessions (id, project_id, title, created_at) VALUES ('bound', 'p1', NULL, '2024-01-01'), ('unbound', '', NULL, '2024-01-02'), ('orphan', 'gone', NULL, '2024-01-03')", []).unwrap();
        let list = list_sessions(&connection).unwrap();
        assert_eq!(list[0].project_title, None);
        assert_eq!(list[0].project_id, "gone");
        assert_eq!(list[1].project_id, "");
        assert_eq!(list[1].project_title, None);
        assert_eq!(list[2].project_title.as_deref(), Some("Project One"));
    }

    #[test]
    fn test_rename_session_trims_and_rejects_empty_or_too_long() {
        let _home = crate::test_support::TempHome::new("session-rename");
        let connection = open().unwrap();
        ensure_session(&connection, "rename", "").unwrap();
        assert_eq!(
            rename_session(&connection, "rename", "  a title  ")
                .unwrap()
                .title,
            "a title"
        );
        assert_eq!(
            rename_session(&connection, "rename", " \t ").unwrap_err(),
            "Session title cannot be empty"
        );
        assert_eq!(
            rename_session(&connection, "rename", &"x".repeat(121)).unwrap_err(),
            "Session title is too long (max 120 characters)"
        );
    }

    #[test]
    fn test_delete_session_removes_messages_and_returns_false_when_missing() {
        let _home = crate::test_support::TempHome::new("session-delete");
        let connection = open().unwrap();
        ensure_session(&connection, "delete", "").unwrap();
        add_message(&connection, "delete", "u1", "user", "hello");
        add_message(&connection, "other", "u2", "user", "keep");
        assert!(delete_session(&connection, "delete").unwrap());
        assert!(!delete_session(&connection, "delete").unwrap());
        assert_eq!(
            super::super::count_messages(&connection, "delete").unwrap(),
            0
        );
        assert_eq!(
            super::super::count_messages(&connection, "other").unwrap(),
            1
        );
    }

    #[test]
    fn test_get_session_returns_none_for_unknown_without_creating_a_row() {
        let _home = crate::test_support::TempHome::new("session-get-unknown");
        let connection = open().unwrap();
        assert!(get_session(&connection, "missing").unwrap().is_none());
        let count: i64 = connection
            .query_row("SELECT COUNT(*) FROM sessions", [], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 0);
    }

    #[test]
    fn test_conditional_bind_preserves_existing_project_and_inserts_missing_session() {
        let _home = crate::test_support::TempHome::new("db-conditional-bind");
        let connection = open().unwrap();
        assert!(bind_session_to_project_if_unbound(&connection, "missing", "first").unwrap());
        assert!(!bind_session_to_project_if_unbound(&connection, "missing", "second").unwrap());
        assert_eq!(
            project_for_session(&connection, "missing")
                .unwrap()
                .as_deref(),
            Some("first")
        );
        ensure_session(&connection, "unbound", "").unwrap();
        assert!(bind_session_to_project_if_unbound(&connection, "unbound", "third").unwrap());
        assert_eq!(
            project_for_session(&connection, "unbound")
                .unwrap()
                .as_deref(),
            Some("third")
        );
    }
}
