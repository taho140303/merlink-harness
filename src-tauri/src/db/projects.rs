//! Project rows: ensure, existence check, create, and test-only listing.

use rusqlite::{Connection, ErrorCode, params};

use super::now;

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

/// Check the project row without creating a project or document.
pub fn project_exists(connection: &Connection, project_id: &str) -> Result<bool, String> {
    connection
        .query_row(
            "SELECT EXISTS(SELECT 1 FROM projects WHERE id = ?1)",
            params![project_id],
            |row| row.get(0),
        )
        .map_err(|error| format!("Cannot check project: {error}"))
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

#[cfg(test)]
mod tests {
    use super::super::open;
    use super::{create_project, ensure_project, list_projects, project_exists};

    #[test]
    fn test_project_exists_is_read_only() {
        let _home = crate::test_support::TempHome::new("db-project-exists");
        let connection = open().unwrap();
        assert!(!project_exists(&connection, "ghost").unwrap());
        assert!(list_projects(&connection).unwrap().is_empty());
        ensure_project(&connection, "real", "Real project").unwrap();
        assert!(project_exists(&connection, "real").unwrap());
        assert!(!project_exists(&connection, "ghost").unwrap());
        assert_eq!(list_projects(&connection).unwrap(), ["real"]);
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
