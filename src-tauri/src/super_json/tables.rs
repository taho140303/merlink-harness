//! Per-table row contracts: allowed columns, required columns, and row validation.

use serde_json::Value;

use super::schema;

fn field_path(table: &str, index: usize, column: &str) -> String {
    format!("{table}[{index}].{column}")
}

/// Validates one `rows` array against its table contract.
///
/// `count` is the declared cardinality, when already set. The caller sets
/// `count` and `rows` in either order, so a `None` count only skips the
/// equality check; `progress` still refuses to tick a table until both agree.
pub fn validate_rows(table: &str, rows: &Value, count: Option<i64>) -> Result<(), String> {
    let Some(items) = rows.as_array() else {
        return Err(format!(
            "Invalid value for {table}.rows: expected an array of objects"
        ));
    };
    if let Some(count) = count {
        if count < 0 {
            return Err(format!(
                "Invalid value for {table}.count: must be 0 or greater"
            ));
        }
        if items.len() as i64 != count {
            return Err(format!(
                "Invalid value for {table}.rows: count is {count} but rows has {} object(s)",
                items.len()
            ));
        }
    }

    let allowed = schema::table_fields(table);
    if allowed.is_empty() {
        return Err(format!("Unknown super json table: {table}"));
    }
    let required = schema::required_row_fields(table);

    for (index, item) in items.iter().enumerate() {
        let Some(object) = item.as_object() else {
            return Err(format!(
                "Invalid value for {table}.rows: row {index} must be an object"
            ));
        };
        for (column, value) in object {
            if !allowed.iter().any(|known| known == column) {
                return Err(format!(
                    "Invalid value for {table}.rows: row {index} has unknown field {column:?}; \
                     allowed fields are: {}",
                    allowed.join(", ")
                ));
            }
            let kind = schema::column_kind(table, column);
            schema::validate(kind, &field_path(table, index, column), value)?;
        }
        for column in &required {
            if !object.contains_key(column) {
                return Err(format!(
                    "Invalid value for {table}.rows: row {index} is missing required field {column:?}"
                ));
            }
        }
    }

    if table == "characters" {
        let primaries = items
            .iter()
            .filter(|item| item["is_primary"].as_bool() == Some(true))
            .count();
        if !items.is_empty() && primaries != 1 {
            return Err(format!(
                "Invalid value for characters.rows: exactly one character must have \
                 is_primary = true (found {primaries})"
            ));
        }
    }

    Ok(())
}

/// Cross-table rules that make a table "not yet complete" without rejecting the
/// write itself (drafting is incremental).
fn document_rule(document: &Value, table: &str, rows: &Value) -> bool {
    let items: &[Value] = rows.as_array().map_or(&[], Vec::as_slice);
    match table {
        "project_motifs" => items
            .iter()
            .any(|row| row.get("scope").and_then(Value::as_str) == Some("project")),
        "scene_characters" => {
            let scenes = schema::countable_entry(document, "scenes")
                .and_then(|entry| entry.get("rows"))
                .and_then(Value::as_array);
            match scenes {
                Some(scenes) if !scenes.is_empty() => items
                    .iter()
                    .any(|row| row.get("is_pov").and_then(Value::as_bool) == Some(true)),
                _ => true,
            }
        }
        _ => true,
    }
}

/// A valid, non-empty row array for a required table.
pub fn is_table_complete(document: &Value, table: &str) -> Result<bool, String> {
    let Some(entry) = schema::countable_entry(document, table) else {
        return Ok(false);
    };
    let Some(count) = entry.get("count").and_then(Value::as_i64) else {
        return Ok(false);
    };
    if count <= 0 {
        return Ok(false);
    }
    let Some(rows) = entry.get("rows") else {
        return Ok(false);
    };
    if rows
        .as_array()
        .is_none_or(|items| items.len() as i64 != count)
    {
        return Ok(false);
    }
    validate_rows(table, rows, Some(count))?;
    Ok(document_rule(document, table, rows))
}

/// Optional tables may legitimately declare zero rows ("none").
pub fn is_optional_table_complete(document: &Value, table: &str) -> Result<bool, String> {
    let Some(entry) = schema::countable_entry(document, table) else {
        return Ok(false);
    };
    let Some(count) = entry.get("count").and_then(Value::as_i64) else {
        return Ok(false);
    };
    if count < 0 {
        return Ok(false);
    }
    let Some(rows) = entry.get("rows") else {
        return Ok(false);
    };
    if rows
        .as_array()
        .is_none_or(|items| items.len() as i64 != count)
    {
        return Ok(false);
    }
    validate_rows(table, rows, Some(count))?;
    Ok(document_rule(document, table, rows))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_three_characters_with_one_primary_is_valid() {
        let rows = json!([
            { "name": "Hoàng", "is_primary": true },
            { "name": "Phong", "is_primary": false },
            { "name": "Linh", "is_primary": false }
        ]);
        validate_rows("characters", &rows, Some(3)).unwrap();
    }

    #[test]
    fn test_rows_reject_unknown_field() {
        let rows = json!([{ "name": "Hoàng", "is_primary": true, "archetype": "hero" }]);
        let error = validate_rows("characters", &rows, Some(1)).unwrap_err();
        assert!(error.contains("unknown field"), "{error}");
    }

    #[test]
    fn test_rows_reject_count_mismatch() {
        let rows = json!([{ "name": "Hoàng", "is_primary": true }]);
        let error = validate_rows("characters", &rows, Some(3)).unwrap_err();
        assert!(error.contains("count is 3"), "{error}");
    }

    #[test]
    fn test_characters_require_exactly_one_primary() {
        let rows = json!([
            { "name": "Hoàng", "is_primary": false },
            { "name": "Phong", "is_primary": false }
        ]);
        let error = validate_rows("characters", &rows, Some(2)).unwrap_err();
        assert!(error.contains("exactly one"), "{error}");
    }

    #[test]
    fn test_scenes_require_chapter_id() {
        let rows = json!([{ "name": "Mở đầu", "position": 1 }]);
        let error = validate_rows("scenes", &rows, Some(1)).unwrap_err();
        assert!(error.contains("chapter_id"), "{error}");
    }

    fn document_with(table: &str, count: i64, rows: Value) -> Value {
        let mut document = schema::skeleton();
        for group in ["require", "optional"] {
            if let Some(entries) = document[group]["schema"].as_array_mut()
                && let Some(entry) = entries
                    .iter_mut()
                    .find(|entry| entry["table"] == table && entry.get("count").is_some())
            {
                entry["count"] = json!(count);
                entry["rows"] = rows;
                return document;
            }
        }
        panic!("no countable entry for {table}");
    }

    #[test]
    fn test_project_motifs_need_a_project_scope_row() {
        let document = document_with(
            "project_motifs",
            1,
            json!([{ "motif_id": "mo-tif", "scope": "arc", "arc_id": "a1" }]),
        );
        assert!(!is_table_complete(&document, "project_motifs").unwrap());

        let document = document_with(
            "project_motifs",
            1,
            json!([{ "motif_id": "mo-tif", "scope": "project" }]),
        );
        assert!(is_table_complete(&document, "project_motifs").unwrap());
    }

    #[test]
    fn test_scene_characters_need_a_pov_row_when_scenes_exist() {
        let mut document = document_with(
            "scenes",
            1,
            json!([{ "name": "Mở đầu", "chapter_id": "c1", "position": 1 }]),
        );
        let scene_characters = document["optional"]["schema"]
            .as_array_mut()
            .unwrap()
            .iter_mut()
            .find(|entry| entry["table"] == "scene_characters")
            .unwrap();
        scene_characters["count"] = json!(1);
        scene_characters["rows"] =
            json!([{ "scene_id": "s1", "character_id": "c1", "is_pov": false }]);
        assert!(!is_optional_table_complete(&document, "scene_characters").unwrap());

        let scene_characters = document["optional"]["schema"]
            .as_array_mut()
            .unwrap()
            .iter_mut()
            .find(|entry| entry["table"] == "scene_characters")
            .unwrap();
        scene_characters["rows"] =
            json!([{ "scene_id": "s1", "character_id": "c1", "is_pov": true }]);
        assert!(is_optional_table_complete(&document, "scene_characters").unwrap());
    }
}
