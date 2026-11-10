//! Read-only catalog queries and code validation for chat tools.
//!
//! Use a short-lived SQLite connection; callers must not hold it across an LLM await.

use rusqlite::{params, Connection, OptionalExtension, Result};
use serde_json::{json, Value};

/// Return true only for a nonempty exact genre code present in the reference catalog.
pub fn validate_genre_code(conn: &Connection, code: &str) -> Result<bool> {
    exists_by_code(conn, "genres", code)
}

/// Return true only for a character-arc motif's exact code.
/// Plot, symbolic and relationship motifs cannot be assigned to a character.
pub fn validate_motif_code(conn: &Connection, code: &str) -> Result<bool> {
    if code.is_empty() || code.trim() != code {
        return Ok(false);
    }
    conn.query_row(
        "SELECT 1 FROM motifs WHERE code = ?1 AND category = 'character_arc' LIMIT 1",
        [code],
        |_| Ok(()),
    )
    .optional()
    .map(|found| found.is_some())
}

fn exists_by_code(conn: &Connection, table: &str, code: &str) -> Result<bool> {
    if code.is_empty() || code.trim() != code {
        return Ok(false);
    }
    // `table` is exclusively supplied by the two constant-name wrappers above.
    conn.query_row(
        &format!("SELECT 1 FROM {table} WHERE code = ?1 LIMIT 1"),
        [code],
        |_| Ok(()),
    )
    .optional()
    .map(|found| found.is_some())
}

fn search_limit(args: &Value) -> i64 {
    args.get("limit")
        .and_then(Value::as_u64)
        .map(|n| n.clamp(1, 50) as i64)
        .unwrap_or(10)
}

/// Search exact catalog codes and human-readable names, case-insensitively.
/// `query` and `limit` are optional; an empty query lists the first page.
/// Items retain `code` so the model can pass an actual catalog key to write tools.
pub fn search_genre(conn: &Connection, args: &Value) -> Result<Value> {
    let query = args.get("query").and_then(Value::as_str).unwrap_or("").trim();
    let pattern = format!("%{}%", escape_like(query));
    let mut stmt = conn.prepare(
        "SELECT code, display_name, description FROM genres
         WHERE code IS NOT NULL AND (code LIKE ?1 ESCAPE '\\' OR display_name LIKE ?1 ESCAPE '\\')
         ORDER BY code COLLATE NOCASE LIMIT ?2",
    )?;
    let rows = stmt.query_map(params![pattern, search_limit(args)], |row| {
        Ok(json!({
            "code": row.get::<_, String>(0)?,
            "display_name": row.get::<_, Option<String>>(1)?,
            "description": row.get::<_, Option<String>>(2)?,
        }))
    })?;
    let items = rows.collect::<Result<Vec<_>>>()?;
    Ok(json!({"status": "applied", "items": items}))
}

/// Search motif codes and labels; `category` optionally restricts to a catalog category.
pub fn search_motif(conn: &Connection, args: &Value) -> Result<Value> {
    let query = args.get("query").and_then(Value::as_str).unwrap_or("").trim();
    let category = args.get("category").and_then(Value::as_str);
    let pattern = format!("%{}%", escape_like(query));
    let mut stmt = conn.prepare(
        "SELECT code, display_name, category, description, min_chapter, max_chapter FROM motifs
         WHERE code IS NOT NULL AND (?2 IS NULL OR category = ?2)
         AND (code LIKE ?1 ESCAPE '\\' OR display_name LIKE ?1 ESCAPE '\\')
         ORDER BY code COLLATE NOCASE LIMIT ?3",
    )?;
    let rows = stmt.query_map(params![pattern, category, search_limit(args)], |row| {
        Ok(json!({
            "code": row.get::<_, String>(0)?,
            "display_name": row.get::<_, Option<String>>(1)?,
            "category": row.get::<_, Option<String>>(2)?,
            "description": row.get::<_, Option<String>>(3)?,
            "min_chapter": row.get::<_, Option<i64>>(4)?,
            "max_chapter": row.get::<_, Option<i64>>(5)?,
        }))
    })?;
    let items = rows.collect::<Result<Vec<_>>>()?;
    Ok(json!({"status": "applied", "items": items}))
}

// A search term is literal: SQLite LIKE wildcards supplied by the model must not broaden it.
fn escape_like(query: &str) -> String {
    query.replace('\\', "\\\\").replace('%', "\\%").replace('_', "\\_")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn catalog() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(
            "CREATE TABLE genres (code TEXT, display_name TEXT, description TEXT);
             CREATE TABLE motifs (code TEXT, display_name TEXT, category TEXT, description TEXT, min_chapter INTEGER, max_chapter INTEGER);
             INSERT INTO genres VALUES ('fantasy', 'Kỳ Ảo', 'Magic'), ('romance', 'Lãng Mạn', 'Love');
             INSERT INTO motifs VALUES ('hero_journey', 'Hero Journey', 'plot', 'A journey', 10, 30), ('hero_arc', 'Hero Arc', 'character_arc', 'Growth', 5, 20);",
        ).unwrap();
        conn
    }

    #[test]
    fn test_validate_codes_exact_and_unknown() {
        let conn = catalog();
        assert!(validate_genre_code(&conn, "fantasy").unwrap());
        assert!(!validate_genre_code(&conn, "Fantasy").unwrap());
        assert!(!validate_genre_code(&conn, "fantasy ").unwrap());
        assert!(!validate_genre_code(&conn, "unknown").unwrap());
        assert!(validate_motif_code(&conn, "hero_arc").unwrap());
        assert!(!validate_motif_code(&conn, "hero_journey").unwrap());
        assert!(!validate_motif_code(&conn, "hero").unwrap());
    }

    #[test]
    fn test_search_genre_filters_names_and_escapes_wildcards() {
        let conn = catalog();
        let result = search_genre(&conn, &json!({"query":"Kỳ"})).unwrap();
        assert_eq!(result["items"][0]["code"], "fantasy");
        assert_eq!(result["items"].as_array().unwrap().len(), 1);
        assert!(search_genre(&conn, &json!({"query":"%"})).unwrap()["items"].as_array().unwrap().is_empty());
    }

    #[test]
    fn test_search_motif_category_and_limit() {
        let conn = catalog();
        let result = search_motif(&conn, &json!({"query":"hero", "category":"character_arc", "limit":1})).unwrap();
        assert_eq!(result["items"].as_array().unwrap().len(), 1);
        assert_eq!(result["items"][0]["code"], "hero_arc");
        assert!(search_motif(&conn, &json!({"category":"relationship"})).unwrap()["items"].as_array().unwrap().is_empty());
    }
}
