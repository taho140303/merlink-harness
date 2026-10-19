//! Session rows: create-or-ignore, project lookup, and conditional binding.

use rusqlite::{Connection, params};

use super::now;

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
    use super::{bind_session_to_project_if_unbound, ensure_session, project_for_session};

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
