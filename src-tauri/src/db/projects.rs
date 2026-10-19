//! Project rows: ensure, existence check, create, and test-only listing.

use rusqlite::{Connection, ErrorCode, params};
use serde_json::Value;

use super::now;
use crate::{models::ProjectSummary, super_json};

fn preferred_title(document: &Value, db_title: &str) -> String {
    document["require"]["project"]["name"]
        .as_str()
        .map(str::trim)
        .filter(|name| !name.is_empty())
        .unwrap_or(db_title)
        .to_owned()
}

fn extract_genres(document: &Value) -> Vec<String> {
    let rows = ["require", "optional"]
        .iter()
        .filter_map(|group| document[*group]["schema"].as_array())
        .flat_map(|entries| entries.iter())
        .find(|entry| entry["table"].as_str() == Some("project_genres"))
        .and_then(|entry| entry["rows"].as_array());
    let Some(rows) = rows else { return Vec::new() };
    let mut unique = std::collections::HashMap::<String, (String, bool, usize)>::new();
    for (index, row) in rows.iter().enumerate() {
        let Some(name) = row["genre_id"]
            .as_str()
            .map(str::trim)
            .filter(|name| !name.is_empty())
        else {
            continue;
        };
        let is_primary = row["is_primary"].as_bool() == Some(true);
        let key = name.to_lowercase();
        match unique.get_mut(&key) {
            Some((stored_name, stored_primary, stored_index)) if is_primary && !*stored_primary => {
                *stored_name = name.to_owned();
                *stored_primary = true;
                *stored_index = index;
            }
            Some(_) => {}
            None => {
                unique.insert(key, (name.to_owned(), is_primary, index));
            }
        }
    }
    let mut genres: Vec<_> = unique.into_values().collect();
    genres.sort_by_key(|(_, is_primary, index)| (!*is_primary, *index));
    genres.into_iter().map(|(name, _, _)| name).collect()
}

fn summary_from_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<(String, String, String, u32)> {
    Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
}

fn build_summary(
    id: String,
    db_title: String,
    created_at: String,
    session_count: u32,
) -> Result<ProjectSummary, String> {
    let metadata = match super_json::load_existing(&id) {
        Ok(document) => document,
        // A malformed document is isolated to this project and uses the same
        // fallback as a missing document. Other read/path errors stay visible.
        Err(error) if error.starts_with("Invalid super json at ") => None,
        Err(error) => return Err(error),
    };
    Ok(ProjectSummary {
        title: metadata
            .as_ref()
            .map(|document| preferred_title(document, &db_title))
            .unwrap_or(db_title),
        genres: metadata.as_ref().map(extract_genres).unwrap_or_default(),
        id,
        created_at,
        session_count,
    })
}

pub fn list_project_summaries(connection: &Connection) -> Result<Vec<ProjectSummary>, String> {
    let mut statement = connection
        .prepare(
            "SELECT p.id, p.title, p.created_at, COALESCE(s.session_count, 0) \
         FROM projects p LEFT JOIN (SELECT project_id, COUNT(*) AS session_count \
         FROM sessions GROUP BY project_id) s ON s.project_id = p.id \
         ORDER BY p.created_at DESC, p.rowid DESC",
        )
        .map_err(|error| format!("Cannot prepare project list: {error}"))?;
    let rows = statement
        .query_map([], summary_from_row)
        .map_err(|error| format!("Cannot list projects: {error}"))?;
    let records = rows
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| format!("Cannot read projects: {error}"))?;
    records
        .into_iter()
        .map(|(id, title, created, count)| build_summary(id, title, created, count))
        .collect()
}

pub fn project_summary(
    connection: &Connection,
    project_id: &str,
) -> Result<Option<ProjectSummary>, String> {
    let record = connection.query_row(
        "SELECT p.id, p.title, p.created_at, (SELECT COUNT(*) FROM sessions s WHERE s.project_id = p.id) \
         FROM projects p WHERE p.id = ?1", [project_id], summary_from_row,
    );
    match record {
        Ok((id, title, created, count)) => build_summary(id, title, created, count).map(Some),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(error) => Err(format!("Cannot read project: {error}")),
    }
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
    use super::{
        create_project, ensure_project, list_project_summaries, list_projects, project_exists,
        project_summary,
    };

    #[test]
    fn test_list_project_summaries_orders_newest_first() {
        let _home = crate::test_support::TempHome::new("db-project-summary-order");
        let connection = open().unwrap();
        connection.execute("INSERT INTO projects (id,title,created_at) VALUES ('older','Older','2025-01-01'),('newer','Newer','2025-02-01')", []).unwrap();
        let summaries = list_project_summaries(&connection).unwrap();
        assert_eq!(
            summaries
                .iter()
                .map(|item| item.id.as_str())
                .collect::<Vec<_>>(),
            ["newer", "older"]
        );
    }

    #[test]
    fn test_project_summary_prefers_super_json_name_and_primary_genres() {
        let home = crate::test_support::TempHome::new("db-project-summary-metadata");
        let connection = open().unwrap();
        ensure_project(&connection, "meta", "DB title").unwrap();
        let path = home.path().join("projects/meta/super.json");
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(&path, r#"{"require":{"project":{"name":" File title "},"schema":[{"table":"project_genres","rows":[{"genre_id":" mystery ","is_primary":false},{"genre_id":"Fantasy","is_primary":true},{"genre_id":"MYSTERY","is_primary":true},{"genre_id":"Sci-Fi","is_primary":false}] }]}}"#).unwrap();
        let summary = project_summary(&connection, "meta").unwrap().unwrap();
        assert_eq!(summary.title, "File title");
        assert_eq!(summary.genres, ["Fantasy", "MYSTERY", "Sci-Fi"]);
    }

    #[test]
    fn test_project_summary_falls_back_when_super_json_is_corrupt() {
        let home = crate::test_support::TempHome::new("db-project-summary-corrupt");
        let connection = open().unwrap();
        ensure_project(&connection, "broken", "DB title").unwrap();
        let path = home.path().join("projects/broken/super.json");
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, "not-json").unwrap();
        let summary = project_summary(&connection, "broken").unwrap().unwrap();
        assert_eq!(summary.title, "DB title");
        assert!(summary.genres.is_empty());
    }

    #[test]
    fn test_project_summary_falls_back_when_super_json_is_missing() {
        let _home = crate::test_support::TempHome::new("db-project-summary-missing");
        let connection = open().unwrap();
        ensure_project(&connection, "missing", "DB title").unwrap();
        let summary = project_summary(&connection, "missing").unwrap().unwrap();
        assert_eq!(summary.title, "DB title");
        assert!(summary.genres.is_empty());
    }

    #[test]
    fn test_project_summary_counts_sessions() {
        let _home = crate::test_support::TempHome::new("db-project-summary-count");
        let connection = open().unwrap();
        ensure_project(&connection, "counted", "Counted").unwrap();
        connection.execute("INSERT INTO sessions (id,project_id,title,created_at) VALUES ('s1','counted',NULL,'2025-01-01'),('s2','counted',NULL,'2025-01-02')", []).unwrap();
        assert_eq!(
            project_summary(&connection, "counted")
                .unwrap()
                .unwrap()
                .session_count,
            2
        );
    }

    #[test]
    fn test_project_summary_returns_none_for_unknown_project() {
        let _home = crate::test_support::TempHome::new("db-project-summary-none");
        let connection = open().unwrap();
        assert!(project_summary(&connection, "unknown").unwrap().is_none());
    }

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
