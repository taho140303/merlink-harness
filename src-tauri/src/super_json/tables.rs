//! Per-table row contracts: allowed columns, required columns, and row validation.

use serde_json::Value;

use super::schema;

fn field_path(table: &str, index: usize, column: &str) -> String {
    format!("{table}[{index}].{column}")
}

/// Draft rows are incremental: required columns, `count == rows.len()`, the
/// characters-primary rule and the cross-table document rules are all skipped.
/// Complete rows are the whole truth for a table and must satisfy every rule.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RowMode {
    Draft,
    Complete,
}

/// Rewrites `_json_index` on every object of a `rows` array to its position.
/// The tool owns this value; a model-supplied value is always overwritten.
pub fn normalize_json_index(rows: &mut Value) {
    if let Some(items) = rows.as_array_mut() {
        for (index, item) in items.iter_mut().enumerate() {
            if let Some(object) = item.as_object_mut() {
                object.insert("_json_index".to_owned(), Value::from(index));
            }
        }
    }
}

/// Draft references that point at a table which can hold rows in the document.
/// A `<column>_json_index` is range-checked only for these pairs; any other
/// draft reference (e.g. `style_preset_id_json_index`) only has to be an int.
pub const REF_TARGETS: &[(&str, &str, &str)] = &[
    ("chapters", "arc_id", "arcs"),
    ("scenes", "chapter_id", "chapters"),
    ("scenes", "world_setting_id", "world_settings"),
    ("scene_characters", "scene_id", "scenes"),
    ("scene_characters", "character_id", "characters"),
    ("relationships", "from_character_id", "characters"),
    ("relationships", "to_character_id", "characters"),
    ("relationship_roles", "relationship_id", "relationships"),
    (
        "relationship_roles",
        "category_id",
        "relationship_categories",
    ),
    ("relationship_motifs", "relationship_id", "relationships"),
    ("relationship_states", "parent_id", "relationships"),
    ("scene_versions", "parent_id", "scenes"),
    ("themes", "arc_id", "arcs"),
    ("themes", "parent_theme_id", "themes"),
    ("theme_arguments", "theme_id", "themes"),
    ("theme_arguments", "character_id", "characters"),
    ("project_motifs", "arc_id", "arcs"),
    ("project_motifs", "chapter_id", "chapters"),
    ("lore_entries", "world_setting_id", "world_settings"),
    ("power_tiers", "power_system_id", "power_systems"),
    ("power_abilities", "power_system_id", "power_systems"),
    ("power_abilities", "tier_id", "power_tiers"),
    ("memories", "source_scene_id", "scenes"),
    ("memory_revisions", "source_scene_id", "scenes"),
    ("memory_references", "memory_id", "memories"),
    ("memory_emotions", "memory_id", "memories"),
    ("memory_connections", "source_memory_id", "memories"),
    ("memory_connections", "target_memory_id", "memories"),
    ("memory_revisions", "memory_id", "memories"),
    ("character_appearances", "character_id", "characters"),
    (
        "character_distinguishing_marks",
        "character_id",
        "characters",
    ),
    ("appearance_options", "group_id", "appearance_option_groups"),
    ("character_competences", "character_id", "characters"),
    ("character_competences", "domain_id", "competence_domains"),
    ("character_knowledge_sources", "character_id", "characters"),
    (
        "competence_capabilities",
        "competence_id",
        "character_competences",
    ),
    (
        "competence_limitations",
        "competence_id",
        "character_competences",
    ),
    ("world_settings", "parent_id", "world_settings"),
];

fn ref_target(table: &str, column: &str) -> Option<&'static str> {
    REF_TARGETS
        .iter()
        .find(|(known_table, known_column, _)| *known_table == table && *known_column == column)
        .map(|(_, _, target)| *target)
}

fn target_rows_len(document: &Value, target: &str) -> usize {
    schema::countable_entry(document, target)
        .and_then(|entry| entry.get("rows"))
        .and_then(Value::as_array)
        .map_or(0, Vec::len)
}

/// Range-checks one `<field>_json_index` value against the document. `add_field`
/// calls this directly for a single-row write; `validate_rows` calls it per
/// column. Non-reference columns are ignored.
pub fn validate_draft_reference(
    table: &str,
    index: usize,
    column: &str,
    value: &Value,
    document: Option<&Value>,
) -> Result<(), String> {
    let Some(base) = column.strip_suffix("_json_index") else {
        return Ok(());
    };
    // `_json_index` is the tool-assigned position, not a pointer elsewhere.
    if base.is_empty() {
        return Ok(());
    }
    let Some(reference) = value.as_i64() else {
        return Ok(());
    };
    let Some(target) = ref_target(table, base) else {
        return Ok(());
    };
    let Some(document) = document else {
        return Ok(());
    };
    let len = target_rows_len(document, target);
    if reference >= len as i64 {
        return Err(format!(
            "Invalid value for {}: draft reference {reference} is out of range for {target}.rows \
             (has {len} row(s))",
            field_path(table, index, column)
        ));
    }
    Ok(())
}

/// Validates one `rows` array against its table contract.
///
/// `count` is the declared cardinality, when already set. The caller sets
/// `count` and `rows` in either order, so a `None` count only skips the
/// equality check; `progress` still refuses to tick a table until both agree.
///
/// `document` is the document *before* this write. It is required for
/// range-checking draft references and for the Complete-mode cross-table rules;
/// callers with no document context (`None`) still get type and key checks.
pub fn validate_rows(
    table: &str,
    rows: &Value,
    count: Option<i64>,
    document: Option<&Value>,
    mode: RowMode,
) -> Result<(), String> {
    let Some(items) = rows.as_array() else {
        return Err(format!(
            "Invalid value for {table}.rows: expected an array of objects"
        ));
    };

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
            // An explicit null means "not set": type rules apply only to real
            // values. A required column that is null is caught below as missing.
            if value.is_null() {
                continue;
            }
            let kind = schema::column_kind(table, column);
            schema::validate(kind, &field_path(table, index, column), value)?;
            validate_draft_reference(table, index, column, value, document)?;
        }
        if mode == RowMode::Complete {
            for column in &required {
                // A null value is the same as a missing column.
                if object.get(column).is_some_and(|value| !value.is_null()) {
                    continue;
                }
                // D1: a draft reference replaces the required `_id` column.
                let reference = format!("{column}_json_index");
                if column.ends_with("_id")
                    && object.get(&reference).is_some_and(|value| !value.is_null())
                {
                    continue;
                }
                return Err(format!(
                    "Invalid value for {table}.rows: row {index} is missing required field \
                     {column:?} (or its draft reference {reference:?})"
                ));
            }
        }
    }

    if mode == RowMode::Complete {
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
        if let Some(document) = document
            && let Some(error) = document_rule_error(document, table, rows)
        {
            return Err(error);
        }
    }

    Ok(())
}

/// Cross-table rules that make a table "not yet complete" without rejecting the
/// write itself (drafting is incremental).
fn document_rule(document: &Value, table: &str, rows: &Value) -> bool {
    document_rule_error(document, table, rows).is_none()
}

fn document_rule_error(document: &Value, table: &str, rows: &Value) -> Option<String> {
    let items: &[Value] = rows.as_array().map_or(&[], Vec::as_slice);
    match table {
        "project_motifs" => {
            let has_project = items
                .iter()
                .any(|row| row.get("scope").and_then(Value::as_str) == Some("project"));
            (!has_project).then(|| {
                "Invalid value for project_motifs.rows: at least one row must have \
                 scope = \"project\""
                    .to_owned()
            })
        }
        "scene_characters" => {
            let scenes = schema::countable_entry(document, "scenes")
                .and_then(|entry| entry.get("rows"))
                .and_then(Value::as_array);
            match scenes {
                Some(scenes)
                    if !scenes.is_empty()
                        && !items.iter().any(|row| {
                            row.get("is_pov").and_then(Value::as_bool) == Some(true)
                        }) =>
                {
                    Some(format!(
                        "Invalid value for scene_characters.rows: at least one row must have \
                         is_pov = true while scenes has {} row(s)",
                        scenes.len()
                    ))
                }
                _ => None,
            }
        }
        _ => None,
    }
}

/// Soft warnings for a loaded document (D4). They never reject a write.
pub fn warnings(document: &Value) -> Vec<String> {
    let mut out = Vec::new();
    let scenes_len = schema::countable_entry(document, "scenes")
        .and_then(|entry| entry.get("rows"))
        .and_then(Value::as_array)
        .map_or(0, Vec::len);
    if scenes_len > 0 {
        let has_pov = schema::countable_entry(document, "scene_characters")
            .and_then(|entry| entry.get("rows"))
            .and_then(Value::as_array)
            .is_some_and(|rows| {
                rows.iter()
                    .any(|row| row.get("is_pov").and_then(Value::as_bool) == Some(true))
            });
        if !has_pov {
            out.push(format!(
                "scene_characters needs at least one is_pov row while scenes has {scenes_len} row(s)"
            ));
        }
    }
    let has_project_motif = schema::countable_entry(document, "project_motifs")
        .and_then(|entry| entry.get("rows"))
        .and_then(Value::as_array)
        .is_some_and(|rows| {
            rows.iter()
                .any(|row| row.get("scope").and_then(Value::as_str) == Some("project"))
        });
    if !has_project_motif {
        out.push("project_motifs needs at least one row with scope = \"project\"".to_owned());
    }
    out
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
    // A cross-table rule that fails means "not complete yet", not "invalid
    // write": validate_rows would reject it in Complete mode, so stop first.
    if !document_rule(document, table, rows) {
        return Ok(false);
    }
    validate_rows(table, rows, Some(count), Some(document), RowMode::Complete)?;
    Ok(true)
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
    // A cross-table rule that fails means "not complete yet", not "invalid
    // write": validate_rows would reject it in Complete mode, so stop first.
    if !document_rule(document, table, rows) {
        return Ok(false);
    }
    validate_rows(table, rows, Some(count), Some(document), RowMode::Complete)?;
    Ok(true)
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
        validate_rows("characters", &rows, Some(3), None, RowMode::Complete).unwrap();
    }

    #[test]
    fn test_rows_reject_unknown_field() {
        let rows = json!([{ "name": "Hoàng", "is_primary": true, "superpower": "flying" }]);
        let error =
            validate_rows("characters", &rows, Some(1), None, RowMode::Complete).unwrap_err();
        assert!(error.contains("unknown field"), "{error}");
    }

    #[test]
    fn test_rows_reject_count_mismatch() {
        let rows = json!([{ "name": "Hoàng", "is_primary": true }]);
        let error =
            validate_rows("characters", &rows, Some(3), None, RowMode::Complete).unwrap_err();
        assert!(error.contains("count is 3"), "{error}");
    }

    #[test]
    fn test_characters_require_exactly_one_primary() {
        let rows = json!([
            { "name": "Hoàng", "is_primary": false },
            { "name": "Phong", "is_primary": false }
        ]);
        let error =
            validate_rows("characters", &rows, Some(2), None, RowMode::Complete).unwrap_err();
        assert!(error.contains("exactly one"), "{error}");
    }

    #[test]
    fn test_scenes_require_chapter_id() {
        let rows = json!([{ "name": "Mở đầu", "position": 1 }]);
        let error = validate_rows("scenes", &rows, Some(1), None, RowMode::Complete).unwrap_err();
        assert!(error.contains("chapter_id"), "{error}");
    }

    #[test]
    fn test_null_optional_field_is_allowed() {
        // The fields contract shows `null`; an explicit null on an optional column
        // means "not set" (`world_setting_id` is allowed to be empty).
        let rows = json!([{
            "name": "Mở đầu",
            "chapter_id": "c1",
            "position": 1,
            "world_setting_id": Value::Null
        }]);
        validate_rows("scenes", &rows, Some(1), None, RowMode::Complete).unwrap();
    }

    #[test]
    fn test_null_required_field_is_rejected() {
        let rows = json!([{ "name": Value::Null, "is_primary": true }]);
        let error =
            validate_rows("characters", &rows, Some(1), None, RowMode::Complete).unwrap_err();
        assert!(error.contains("missing required field"), "{error}");
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

    #[test]
    fn test_ref_satisfies_required_id() {
        let document = document_with(
            "characters",
            2,
            json!([
                { "name": "Hoàng", "is_primary": true },
                { "name": "Phong", "is_primary": false }
            ]),
        );
        // D1: two draft references stand in for the two required `_id` columns.
        let rows = json!([
            { "from_character_id_json_index": 0, "to_character_id_json_index": 1 }
        ]);
        validate_rows(
            "relationships",
            &rows,
            Some(1),
            Some(&document),
            RowMode::Complete,
        )
        .unwrap();

        // Without either form, the error mentions both choices.
        let missing = json!([{ "from_character_id": "c1" }]);
        let error = validate_rows(
            "relationships",
            &missing,
            Some(1),
            Some(&document),
            RowMode::Complete,
        )
        .unwrap_err();
        assert!(error.contains("to_character_id"), "{error}");
        assert!(error.contains("to_character_id_json_index"), "{error}");
    }

    #[test]
    fn test_ref_out_of_range_is_rejected() {
        let document = document_with(
            "characters",
            2,
            json!([
                { "name": "Hoàng", "is_primary": true },
                { "name": "Phong", "is_primary": false }
            ]),
        );
        let rows = json!([
            { "from_character_id_json_index": 0, "to_character_id_json_index": 2 }
        ]);
        let error = validate_rows(
            "relationships",
            &rows,
            Some(1),
            Some(&document),
            RowMode::Complete,
        )
        .unwrap_err();
        assert!(error.contains("out of range"), "{error}");
        assert!(error.contains("characters.rows"), "{error}");

        // Without a document there is no range to check, only the integer type.
        validate_rows("relationships", &rows, Some(1), None, RowMode::Complete).unwrap();
        // A target outside the draft-capable tables is only an integer check.
        let preset = json!([{ "name": "Mở đầu", "position": 1, "chapter_id": "c1",
                              "style_preset_id_json_index": 99 }]);
        validate_rows(
            "scenes",
            &preset,
            Some(1),
            Some(&document),
            RowMode::Complete,
        )
        .unwrap();
    }

    #[test]
    fn test_json_index_is_normalised() {
        let mut rows = json!([
            { "name": "Hoàng", "is_primary": true, "_json_index": 9 },
            { "name": "Phong", "is_primary": false }
        ]);
        normalize_json_index(&mut rows);
        assert_eq!(rows[0]["_json_index"], json!(0));
        assert_eq!(rows[1]["_json_index"], json!(1));
        // The synthetic column is allowed by the row contract.
        validate_rows("characters", &rows, Some(2), None, RowMode::Complete).unwrap();
    }

    #[test]
    fn test_draft_allows_partial_rows() {
        let rows = json!([{ "name": "Hoàng" }]);
        validate_rows("characters", &rows, Some(1), None, RowMode::Draft).unwrap();
        let error =
            validate_rows("characters", &rows, Some(1), None, RowMode::Complete).unwrap_err();
        assert!(error.contains("is_primary"), "{error}");
        // Complete also enforces count == rows.len().
        validate_rows("characters", &rows, Some(5), None, RowMode::Draft).unwrap();
        let full = json!([{ "name": "Hoàng", "is_primary": true }]);
        validate_rows("characters", &full, Some(5), None, RowMode::Draft).unwrap();
        let error =
            validate_rows("characters", &full, Some(5), None, RowMode::Complete).unwrap_err();
        assert!(error.contains("count is 5"), "{error}");
    }

    #[test]
    fn test_warnings_report_missing_pov_and_project_motif() {
        let document = schema::skeleton();
        assert_eq!(
            warnings(&document),
            vec!["project_motifs needs at least one row with scope = \"project\"".to_owned()]
        );

        let mut document = document_with(
            "scenes",
            1,
            json!([{ "name": "Mở đầu", "chapter_id": "c1", "position": 1 }]),
        );
        document["optional"]["schema"]
            .as_array_mut()
            .unwrap()
            .iter_mut()
            .find(|entry| entry["table"] == "scene_characters")
            .unwrap()["rows"] = json!([
            { "scene_id": "s1", "character_id": "c1", "is_pov": false }
        ]);
        assert_eq!(
            warnings(&document),
            vec![
                "scene_characters needs at least one is_pov row while scenes has 1 row(s)"
                    .to_owned(),
                "project_motifs needs at least one row with scope = \"project\"".to_owned()
            ]
        );

        document["require"]["schema"]
            .as_array_mut()
            .unwrap()
            .iter_mut()
            .find(|entry| entry["table"] == "project_motifs")
            .unwrap()["rows"] = json!([{ "motif_id": "m1", "scope": "project" }]);
        document["optional"]["schema"]
            .as_array_mut()
            .unwrap()
            .iter_mut()
            .find(|entry| entry["table"] == "scene_characters")
            .unwrap()["rows"] = json!([
            { "scene_id": "s1", "character_id": "c1", "is_pov": true }
        ]);
        assert!(warnings(&document).is_empty());
    }
}
