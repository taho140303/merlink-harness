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
        .map(|n| n.clamp(1, 100) as i64)
        .unwrap_or(20)
}

fn search_offset(args: &Value) -> i64 {
    args.get("offset")
        .and_then(Value::as_u64)
        .unwrap_or(0) as i64
}

fn page_hint(tool: &str) -> String {
    format!(
        "List mode returns only codes. Call {tool} with code=<code> to read full details; pass query?, limit?, offset? for paging."
    )
}

/// Genre list/detail. List mode is paginated and returns codes only so a large catalog never floods
/// the context; the agent fetches a specific genre by `code` when it needs the full record.
pub fn search_genre(conn: &Connection, args: &Value) -> Result<Value> {
    if let Some(code) = args.get("code").and_then(Value::as_str).map(str::trim).filter(|c| !c.is_empty()) {
        let item = conn
            .query_row(
                "SELECT code, display_name, description FROM genres WHERE code = ?1 LIMIT 1",
                [code],
                |row| {
                    Ok(json!({
                        "code": row.get::<_, String>(0)?,
                        "display_name": row.get::<_, Option<String>>(1)?,
                        "description": row.get::<_, Option<String>>(2)?,
                    }))
                },
            )
            .optional()?;
        return Ok(match item {
            Some(item) => json!({"status": "applied", "detail": true, "total": 1, "items": [item]}),
            None => json!({"status": "applied", "detail": true, "total": 0, "items": [], "error": "unknown_genre"}),
        });
    }

    let query = args.get("query").and_then(Value::as_str).unwrap_or("").trim();
    let pattern = format!("%{}%", escape_like(query));
    let total: i64 = conn.query_row(
        "SELECT COUNT(*) FROM genres
         WHERE code IS NOT NULL AND (code LIKE ?1 ESCAPE '\\' OR display_name LIKE ?1 ESCAPE '\\')",
        [&pattern],
        |row| row.get(0),
    )?;
    let limit = search_limit(args);
    let offset = search_offset(args);
    let mut stmt = conn.prepare(
        "SELECT code FROM genres
         WHERE code IS NOT NULL AND (code LIKE ?1 ESCAPE '\\' OR display_name LIKE ?1 ESCAPE '\\')
         ORDER BY code COLLATE NOCASE LIMIT ?2 OFFSET ?3",
    )?;
    let rows = stmt.query_map(params![pattern, limit, offset], |row| {
        Ok(json!({"code": row.get::<_, String>(0)?}))
    })?;
    let items = rows.collect::<Result<Vec<_>>>()?;
    let next_offset = if offset + limit < total { Some(offset + limit) } else { None };
    Ok(json!({
        "status": "applied",
        "total": total,
        "limit": limit,
        "offset": offset,
        "next_offset": next_offset,
        "items": items,
        "hint": page_hint("search_genre"),
    }))
}

/// Motif list/detail with the same pagination and code-only list contract as genres.
pub fn search_motif(conn: &Connection, args: &Value) -> Result<Value> {
    if let Some(code) = args.get("code").and_then(Value::as_str).map(str::trim).filter(|c| !c.is_empty()) {
        let item = conn
            .query_row(
                "SELECT code, display_name, category, description, min_chapter, max_chapter
                 FROM motifs WHERE code = ?1 LIMIT 1",
                [code],
                |row| {
                    Ok(json!({
                        "code": row.get::<_, String>(0)?,
                        "display_name": row.get::<_, Option<String>>(1)?,
                        "category": row.get::<_, Option<String>>(2)?,
                        "description": row.get::<_, Option<String>>(3)?,
                        "min_chapter": row.get::<_, Option<i64>>(4)?,
                        "max_chapter": row.get::<_, Option<i64>>(5)?,
                    }))
                },
            )
            .optional()?;
        return Ok(match item {
            Some(item) => json!({"status": "applied", "detail": true, "total": 1, "items": [item]}),
            None => json!({"status": "applied", "detail": true, "total": 0, "items": [], "error": "unknown_motif"}),
        });
    }

    let query = args.get("query").and_then(Value::as_str).unwrap_or("").trim();
    let category = args.get("category").and_then(Value::as_str);
    let pattern = format!("%{}%", escape_like(query));
    let total: i64 = conn.query_row(
        "SELECT COUNT(*) FROM motifs
         WHERE code IS NOT NULL AND (?2 IS NULL OR category = ?2)
         AND (code LIKE ?1 ESCAPE '\\' OR display_name LIKE ?1 ESCAPE '\\')",
        params![pattern, category],
        |row| row.get(0),
    )?;
    let limit = search_limit(args);
    let offset = search_offset(args);
    let mut stmt = conn.prepare(
        "SELECT code FROM motifs
         WHERE code IS NOT NULL AND (?2 IS NULL OR category = ?2)
         AND (code LIKE ?1 ESCAPE '\\' OR display_name LIKE ?1 ESCAPE '\\')
         ORDER BY code COLLATE NOCASE LIMIT ?3 OFFSET ?4",
    )?;
    let rows = stmt.query_map(params![pattern, category, limit, offset], |row| {
        Ok(json!({"code": row.get::<_, String>(0)?}))
    })?;
    let items = rows.collect::<Result<Vec<_>>>()?;
    let next_offset = if offset + limit < total { Some(offset + limit) } else { None };
    Ok(json!({
        "status": "applied",
        "total": total,
        "limit": limit,
        "offset": offset,
        "next_offset": next_offset,
        "items": items,
        "hint": page_hint("search_motif"),
    }))
}

/// Generic paginated catalog search for the `search_*` read tools. List mode returns codes only
/// (`limit`/`offset`, default 20, max 100); passing `code` returns the full row. Keeping the list
/// payload small lets the model page through catalogs without flooding the context.
pub fn search_catalog(conn: &Connection, tool: &str, args: &Value) -> Result<Value> {
    let (table, label_column, filter_column): (&str, &str, Option<&str>) = match tool {
        "search_narrator_config" => ("narrator_configs", "display_name", None),
        "search_style_preset" => ("style_presets", "category", Some("category")),
        "search_competence_domain" => ("competence_domains", "display_name", None),
        "search_relationship_category" => ("relationship_categories", "category", Some("category")),
        "search_appearance_option" => ("appearance_options", "display_name", None),
        "search_event_block" => ("event_blocks", "code", None),
        "search_character" => ("characters", "display_name", None),
        _ => return Ok(json!({"status": "rejected", "error": "catalog_tool_unknown" })),
    };

    if let Some(code) = args.get("code").and_then(Value::as_str).map(str::trim).filter(|c| !c.is_empty()) {
        let mut stmt = conn.prepare(&format!("SELECT * FROM {table} WHERE code = ?1 LIMIT 1"))?;
        let columns: Vec<String> = stmt.column_names().iter().map(|name| name.to_string()).collect();
        let item = stmt.query_row([code], |row| {
            let mut object = serde_json::Map::new();
            for (index, column) in columns.iter().enumerate() {
                let value: rusqlite::types::Value = row.get(index)?;
                object.insert(column.clone(), sqlite_value_to_json(value));
            }
            Ok(Value::Object(object))
        }).optional()?;
        return Ok(match item {
            Some(item) => json!({"status":"applied","detail":true,"total":1,"items":[item]}),
            None => json!({"status":"applied","detail":true,"total":0,"items":[],"error":"unknown_code"}),
        });
    }

    let query = args.get("query").and_then(Value::as_str).unwrap_or("").trim();
    let pattern = format!("%{}%", escape_like(query));
    let category = args.get("category").and_then(Value::as_str);
    let label_expr = if label_column == "code" { "code".to_string() } else { format!("coalesce({label_column}, code)") };
    let category_clause = if filter_column.is_some() { " AND (?2 IS NULL OR category = ?2)" } else { " AND ?2 IS NULL" };
    let base = format!(
        "WHERE code IS NOT NULL AND (code LIKE ?1 ESCAPE '\\' OR {label_expr} LIKE ?1 ESCAPE '\\'){category_clause}"
    );
    let total: i64 = conn.query_row(
        &format!("SELECT COUNT(*) FROM {table} {base}"),
        params![pattern, category],
        |row| row.get(0),
    )?;
    let limit = search_limit(args);
    let offset = search_offset(args);
    let mut stmt = conn.prepare(&format!("SELECT code FROM {table} {base} ORDER BY code COLLATE NOCASE LIMIT ?3 OFFSET ?4"))?;
    let rows = stmt.query_map(params![pattern, category, limit, offset], |row| {
        Ok(json!({"code": row.get::<_, String>(0)?}))
    })?;
    let items = rows.collect::<Result<Vec<_>>>()?;
    let next_offset = if offset + limit < total { Some(offset + limit) } else { None };
    Ok(json!({
        "status":"applied",
        "total": total,
        "limit": limit,
        "offset": offset,
        "next_offset": next_offset,
        "items": items,
        "hint": page_hint(tool),
    }))
}

fn sqlite_value_to_json(value: rusqlite::types::Value) -> Value {
    use rusqlite::types::Value as SqlValue;
    match value {
        SqlValue::Null => Value::Null,
        SqlValue::Integer(number) => json!(number),
        SqlValue::Real(number) => json!(number),
        SqlValue::Text(text) => json!(text),
        SqlValue::Blob(_) => Value::Null,
    }
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
             CREATE TABLE narrator_configs (code TEXT, display_name TEXT, description TEXT, person TEXT, pov_scope TEXT);
             INSERT INTO genres VALUES ('fantasy', 'Kỳ Ảo', 'Magic'), ('romance', 'Lãng Mạn', 'Love');
             INSERT INTO motifs VALUES ('hero_journey', 'Hero Journey', 'plot', 'A journey', 10, 30), ('hero_arc', 'Hero Arc', 'character_arc', 'Growth', 5, 20);
             INSERT INTO narrator_configs VALUES ('third_limited', 'Third Limited', 'One POV', 'third', 'limited'), ('first_person', 'First Person', 'I', 'first', 'single');",
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
    fn test_list_is_paginated_codes_only_and_detail_by_code() {
        let conn = catalog();
        let first = search_genre(&conn, &json!({"limit": 1})).unwrap();
        assert_eq!(first["total"], 2);
        assert_eq!(first["items"].as_array().unwrap().len(), 1);
        assert_eq!(first["items"][0]["code"], "fantasy");
        assert!(first["items"][0].get("display_name").is_none(), "list mode must only return codes");
        assert_eq!(first["next_offset"], 1);

        let second = search_genre(&conn, &json!({"limit": 1, "offset": 1})).unwrap();
        assert_eq!(second["items"][0]["code"], "romance");

        let detail = search_genre(&conn, &json!({"code": "fantasy"})).unwrap();
        assert_eq!(detail["items"][0]["display_name"], "Kỳ Ảo");
        assert_eq!(detail["items"][0]["description"], "Magic");

        let missing = search_genre(&conn, &json!({"code": "unknown"})).unwrap();
        assert_eq!(missing["total"], 0);
    }

    #[test]
    fn test_generic_catalog_is_paginated_and_details_load_by_code() {
        let conn = catalog();
        let page = search_catalog(&conn, "search_narrator_config", &json!({"limit": 1})).unwrap();
        assert_eq!(page["total"], 2);
        assert_eq!(page["items"][0]["code"], "first_person");
        assert!(page["items"][0].get("display_name").is_none());
        assert_eq!(page["next_offset"], 1);

        let next = search_catalog(&conn, "search_narrator_config", &json!({"limit": 1, "offset": 1})).unwrap();
        assert_eq!(next["items"][0]["code"], "third_limited");

        let detail = search_catalog(&conn, "search_narrator_config", &json!({"code": "third_limited"})).unwrap();
        assert_eq!(detail["items"][0]["display_name"], "Third Limited");
        assert_eq!(detail["items"][0]["pov_scope"], "limited");
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
