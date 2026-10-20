//! Locked, atomic read/write access to one project's `super.json`, plus the
//! one-way migration from the flat v1 document (25 leaves) to v2.

use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    sync::{Arc, Mutex, OnceLock},
};

use serde_json::{Map, Value, json};

use super::{
    schema::{self, Group, PathRef},
    tables::{self, RowMode},
};
use crate::paths;

/// Error text for an unknown table. The valid list matters: models often guess a
/// selector (`require.schema.characters`) instead of the bare table name.
fn unknown_table(table: &str) -> String {
    format!(
        "Unknown super json table: {table}. Use the bare table name, one of: {}",
        schema::tables().join(", ")
    )
}

fn file_path(project_id: &str) -> Result<PathBuf, String> {
    paths::super_json_path(project_id)
}

/// Process-wide per-project serialization. This protects only threads in this app
/// process; a second app process can still race on the same filesystem document.
fn project_lock(project_id: &str) -> Arc<Mutex<()>> {
    static LOCKS: OnceLock<Mutex<HashMap<String, Arc<Mutex<()>>>>> = OnceLock::new();
    let registry = LOCKS.get_or_init(|| Mutex::new(HashMap::new()));
    let mut locks = registry.lock().unwrap_or_else(|poison| poison.into_inner());
    Arc::clone(
        locks
            .entry(project_id.to_owned())
            .or_insert_with(|| Arc::new(Mutex::new(()))),
    )
}

/// Reads an existing super json file. Missing file → `Ok(None)`. Never creates
/// a project directory or a skeleton, unlike [`load_or_init`].
pub fn load_existing(project_id: &str) -> Result<Option<Value>, String> {
    let path = file_path(project_id)?;
    match fs::read_to_string(&path) {
        Ok(raw) => serde_json::from_str(&raw)
            .map(Some)
            .map_err(|error| format!("Invalid super json at {}: {error}", path.display())),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(format!(
            "Cannot read super json at {}: {error}",
            path.display()
        )),
    }
}

/// Reads the project's super json, creating it from the skeleton on first use.
pub fn load_or_init(project_id: &str) -> Result<Value, String> {
    let lock = project_lock(project_id);
    let _guard = lock.lock().unwrap_or_else(|poison| poison.into_inner());
    load_or_init_unlocked(project_id)
}

/// The caller must already hold the project's lock.
fn load_or_init_unlocked(project_id: &str) -> Result<Value, String> {
    let path = file_path(project_id)?;
    match fs::read_to_string(&path) {
        Ok(raw) => {
            let mut document: Value = serde_json::from_str(&raw)
                .map_err(|error| format!("Invalid super json at {}: {error}", path.display()))?;
            let migrated_v1 = migrate_if_needed(&path, &raw, &mut document)?;
            let migrated_groups = migrate_groups_if_needed(&mut document);
            let migrated_v3 = migrate_v3_if_needed(&path, &raw, &mut document)?;
            if migrated_v1 || migrated_groups || migrated_v3 {
                save_unlocked(project_id, &document, false)?;
            }
            Ok(document)
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            let document = schema::skeleton();
            save_unlocked(project_id, &document, true)?;
            Ok(document)
        }
        Err(error) => Err(format!("Cannot read super json: {error}")),
    }
}

/// The caller must already hold the project's lock. Exclusive saves publish via
/// hard link, which fails if `super.json` exists instead of replacing it.
fn save_unlocked(project_id: &str, document: &Value, exclusive: bool) -> Result<(), String> {
    let path = file_path(project_id)?;
    let parent = path.parent().ok_or("Invalid super json path")?;
    fs::create_dir_all(parent)
        .map_err(|error| format!("Cannot create project directory: {error}"))?;

    let bytes = serde_json::to_vec_pretty(document)
        .map_err(|error| format!("Cannot encode super json: {error}"))?;
    let tmp = parent.join(format!(
        ".super.{}.{}.tmp",
        std::process::id(),
        chrono::Utc::now().timestamp_nanos_opt().unwrap_or_default()
    ));

    let result = (|| -> Result<(), String> {
        #[cfg(unix)]
        {
            use std::{io::Write, os::unix::fs::OpenOptionsExt};
            let mut file = fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .mode(0o600)
                .open(&tmp)
                .map_err(|error| format!("Cannot create super json: {error}"))?;
            file.write_all(&bytes)
                .map_err(|error| format!("Cannot write super json: {error}"))?;
            file.sync_all()
                .map_err(|error| format!("Cannot sync super json: {error}"))?;
        }
        #[cfg(not(unix))]
        {
            fs::write(&tmp, &bytes).map_err(|error| format!("Cannot write super json: {error}"))?;
        }
        if exclusive {
            fs::hard_link(&tmp, &path).map_err(|error| {
                if error.kind() == std::io::ErrorKind::AlreadyExists {
                    format!("Super json already exists at {}", path.display())
                } else {
                    format!("Cannot initialize super json: {error}")
                }
            })?;
            let _ = fs::remove_file(&tmp);
            Ok(())
        } else {
            fs::rename(&tmp, &path).map_err(|error| format!("Cannot replace super json: {error}"))
        }
    })();

    if result.is_err() {
        let _ = fs::remove_file(&tmp);
    }
    result
}

/// Initialize once, in one atomic publish, without replacing an existing file.
/// A successful return means this call created the file; callers may compensate
/// a later transaction failure by removing only that file.
pub fn init_project(project_id: &str, name: Option<&str>) -> Result<(), String> {
    let lock = project_lock(project_id);
    let _guard = lock.lock().unwrap_or_else(|poison| poison.into_inner());
    let mut document = schema::skeleton();
    if let Some(name) = name {
        if name.trim().is_empty() {
            return Err("Invalid value for require.project.name: must not be empty".to_owned());
        }
        document["require"]["project"]["name"] = Value::String(name.to_owned());
    }
    save_unlocked(project_id, &document, true)
}

fn project_mut<'a>(document: &'a mut Value, group: Group, rest: &str) -> Option<&'a mut Value> {
    let mut cursor = document.get_mut(group.as_str())?.get_mut("project")?;
    for segment in rest.split('.') {
        cursor = cursor.get_mut(segment)?;
    }
    Some(cursor)
}

fn project_ref<'a>(document: &'a Value, group: Group, rest: &str) -> Option<&'a Value> {
    let mut cursor = document.get(group.as_str())?.get("project")?;
    for segment in rest.split('.') {
        cursor = cursor.get(segment)?;
    }
    Some(cursor)
}

fn schema_entry_mut<'a>(
    document: &'a mut Value,
    group: Group,
    table: &str,
    writable: bool,
) -> Option<&'a mut Value> {
    document
        .get_mut(group.as_str())?
        .get_mut("schema")?
        .as_array_mut()?
        .iter_mut()
        .find(|entry| {
            entry["table"].as_str() == Some(table) && (!writable || entry.get("count").is_some())
        })
}

pub fn get(project_id: &str, path: &str) -> Result<Value, String> {
    let parsed =
        schema::parse_path(path).ok_or_else(|| format!("Unknown super json path: {path}"))?;
    if !matches!(parsed, PathRef::TableSelector { .. })
        && !schema::valid_paths().iter().any(|known| known == path)
    {
        return Err(format!("Unknown super json path: {path}"));
    }
    let document = load_or_init(project_id)?;
    match parsed {
        PathRef::Project { group, rest } => project_ref(&document, group, rest)
            .cloned()
            .ok_or_else(|| format!("Path not found: {path}")),
        PathRef::TableSelector { table, .. } => {
            schema::merged_table_entry(&document, table).ok_or_else(|| unknown_table(table))
        }
        PathRef::SchemaCount { group, table } | PathRef::SchemaRows { group, table } => {
            let field = if matches!(parsed, PathRef::SchemaCount { .. }) {
                "count"
            } else {
                "rows"
            };
            let mut cursor = document
                .get(group.as_str())
                .and_then(|group| group.get("schema"))
                .and_then(Value::as_array)
                .and_then(|entries| {
                    entries
                        .iter()
                        .find(|entry| entry["table"].as_str() == Some(table))
                })
                .and_then(|entry| entry.get(field));
            if cursor.is_none() && field == "count" {
                cursor = schema::table_entry(&document, table).and_then(|entry| entry.get("count"));
            }
            cursor
                .cloned()
                .ok_or_else(|| format!("Path not found: {path}"))
        }
    }
}

/// Sets exactly one writable field. Never creates new fields.
pub fn set(project_id: &str, path: &str, mut value: Value) -> Result<Value, String> {
    let parsed = schema::parse_path(path).ok_or_else(|| {
        format!(
            "Unknown super json path: {path}. Valid paths are: {}",
            schema::valid_paths().join(", ")
        )
    })?;

    if let PathRef::TableSelector { .. } = parsed {
        return Err(format!(
            "Path {path} is read-only; use a table selector only with super_json_get"
        ));
    }

    let kind =
        schema::field_kind(path).ok_or_else(|| format!("Unknown super json path: {path}"))?;
    match kind {
        schema::FieldKind::Rows => {}
        other => schema::validate(other, path, &value)?,
    }

    let lock = project_lock(project_id);
    let _guard = lock.lock().unwrap_or_else(|poison| poison.into_inner());
    let mut document = load_or_init_unlocked(project_id)?;

    match parsed {
        PathRef::Project { group, rest } => {
            let slot = project_mut(&mut document, group, rest)
                .ok_or_else(|| format!("Path not found: {path}"))?;
            *slot = value.clone();
        }
        PathRef::SchemaCount { group, table } => {
            let count = value
                .as_i64()
                .ok_or_else(|| format!("Invalid value for {path}: expected an integer"))?;
            if group == Group::Require && count == 0 {
                return Err(format!(
                    "Invalid value for {path}: a required table needs at least one row"
                ));
            }
            let entry = schema_entry_mut(&mut document, group, table, true)
                .ok_or_else(|| format!("Path not found: {path}"))?;
            if let Some(rows) = entry.get("rows").and_then(Value::as_array)
                && !rows.is_empty()
                && rows.len() as i64 != count
            {
                return Err(format!(
                    "Invalid value for {path}: rows has {} object(s), not {count}",
                    rows.len()
                ));
            }
            entry["count"] = Value::from(count);
        }
        PathRef::SchemaRows { group, table } => {
            let existing_count = schema_entry_mut(&mut document, group, table, true)
                .and_then(|entry| entry.get("count"))
                .and_then(Value::as_i64);
            tables::normalize_json_index(&mut value);
            tables::validate_rows(
                table,
                &value,
                existing_count,
                Some(&document),
                RowMode::Complete,
            )?;
            let entry = schema_entry_mut(&mut document, group, table, true)
                .ok_or_else(|| format!("Path not found: {path}"))?;
            entry["rows"] = value.clone();
        }
        PathRef::TableSelector { .. } => unreachable!(),
    }

    save_unlocked(project_id, &document, false)?;
    Ok(value)
}

/// Appends one draft row to a table and returns `(json_index, count)`.
///
/// The tool owns `_json_index`; the count follows D2: an unset count becomes
/// `rows.len()`, a count that matched the old length follows the append, and a
/// count already ahead of the rows (a declared plan) is left alone.
pub fn add_row(project_id: &str, table: &str, row: Value) -> Result<(i64, i64), String> {
    if schema::table_fields(table).is_empty() {
        return Err(unknown_table(table));
    }
    if !row.is_object() {
        return Err(format!(
            "Invalid value for {table}.rows: row must be an object"
        ));
    }
    let group = schema::countable_group(table).ok_or_else(|| unknown_table(table))?;

    let lock = project_lock(project_id);
    let _guard = lock.lock().unwrap_or_else(|poison| poison.into_inner());
    let mut document = load_or_init_unlocked(project_id)?;

    let (old_rows, current_count) = {
        let entry = schema_entry_mut(&mut document, group, table, true)
            .ok_or_else(|| unknown_table(table))?;
        (
            entry
                .get("rows")
                .and_then(Value::as_array)
                .cloned()
                .unwrap_or_default(),
            entry.get("count").and_then(Value::as_i64),
        )
    };
    let json_index = old_rows.len() as i64;
    let count = match current_count {
        // A count ahead of the rows is a plan the author declared; keep it.
        Some(current) if current != json_index => current,
        // Unset, or exactly in sync: the append advances the count.
        _ => json_index + 1,
    };

    let mut appended = row;
    appended["_json_index"] = Value::from(json_index);
    let mut candidate = Value::Array(old_rows);
    candidate
        .as_array_mut()
        .expect("built as an array")
        .push(appended);
    tables::validate_rows(
        table,
        &candidate,
        Some(count),
        Some(&document),
        RowMode::Draft,
    )?;

    let entry =
        schema_entry_mut(&mut document, group, table, true).ok_or_else(|| unknown_table(table))?;
    entry["count"] = Value::from(count);
    entry["rows"] = candidate;

    save_unlocked(project_id, &document, false)?;
    Ok((json_index, count))
}

/// Sets exactly one field of one existing row.
pub fn add_field(
    project_id: &str,
    table: &str,
    json_index: i64,
    field: &str,
    value: Value,
) -> Result<(), String> {
    let allowed = schema::table_fields(table);
    if allowed.is_empty() {
        return Err(unknown_table(table));
    }
    if field == "_json_index" {
        return Err("_json_index is assigned by the tool and cannot be written".to_owned());
    }
    if !allowed.iter().any(|known| known == field) {
        return Err(format!(
            "Unknown field {field:?} for {table}; allowed fields are: {}",
            allowed.join(", ")
        ));
    }
    let required = schema::required_row_fields(table);
    if value.is_null() {
        // Optional columns may be cleared with an explicit null; a required column
        // cannot be nulled (the completeness rules would reject the table anyway).
        if required.iter().any(|known| known == field) {
            return Err(format!(
                "Invalid value for {table}.rows[{json_index}].{field}: a required field cannot be null"
            ));
        }
    } else {
        let kind = schema::column_kind(table, field);
        schema::validate(kind, &format!("{table}.rows[{json_index}].{field}"), &value)?;
    }

    let group = schema::countable_group(table).ok_or_else(|| unknown_table(table))?;
    let lock = project_lock(project_id);
    let _guard = lock.lock().unwrap_or_else(|poison| poison.into_inner());
    let mut document = load_or_init_unlocked(project_id)?;

    {
        let entry =
            schema::countable_entry(&document, table).ok_or_else(|| unknown_table(table))?;
        let rows = entry
            .get("rows")
            .and_then(Value::as_array)
            .ok_or_else(|| format!("Path not found: {table}.rows"))?;
        if json_index < 0 || json_index as usize >= rows.len() {
            return Err(format!("Row {json_index} not found in {table}.rows"));
        }
        tables::validate_draft_reference(
            table,
            json_index as usize,
            field,
            &value,
            Some(&document),
        )?;
    }

    let entry =
        schema_entry_mut(&mut document, group, table, true).ok_or_else(|| unknown_table(table))?;
    let rows = entry
        .get_mut("rows")
        .and_then(Value::as_array_mut)
        .ok_or_else(|| format!("Path not found: {table}.rows"))?;
    let object = rows[json_index as usize].as_object_mut().ok_or_else(|| {
        format!("Invalid value for {table}.rows: row {json_index} must be an object")
    })?;
    object.insert(field.to_owned(), value);

    save_unlocked(project_id, &document, false)?;
    Ok(())
}

/// Soft warnings for the project's current document (D4).
pub fn warnings(project_id: &str) -> Result<Vec<String>, String> {
    let document = load_or_init(project_id)?;
    Ok(tables::warnings(&document))
}

// ─────────────────────────────────────────────────────────────────────────
// v1 → v2 migration
// ─────────────────────────────────────────────────────────────────────────

fn old_value<'a>(document: &'a Value, group: &str, path: &str) -> Option<&'a Value> {
    let mut cursor = document.get(group)?;
    for segment in path.split('.') {
        cursor = cursor.get(segment)?;
    }
    Some(cursor)
}

fn old_text(document: &Value, group: &str, path: &str) -> Option<String> {
    old_value(document, group, path)
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|text| !text.is_empty())
        .map(str::to_owned)
}

fn copy_project(
    document: &mut Value,
    old: &Value,
    group: Group,
    old_group: &str,
    old_path: &str,
    new_leaf: &str,
) {
    if let Some(text) = old_text(old, old_group, old_path)
        && let Some(slot) = project_mut(document, group, new_leaf)
    {
        *slot = Value::String(text);
    }
}

fn copy_int(
    document: &mut Value,
    old: &Value,
    group: Group,
    old_group: &str,
    old_path: &str,
    new_leaf: &str,
) {
    if let Some(number) = old_value(old, old_group, old_path).and_then(Value::as_i64)
        && let Some(slot) = project_mut(document, group, new_leaf)
    {
        *slot = Value::from(number);
    }
}

fn write_table(document: &mut Value, table: &str, count: i64, rows: Vec<Value>) {
    let has_countable = schema::countable_entry(document, table).is_some();
    if !has_countable {
        return;
    }
    for group in [Group::Require, Group::Optional] {
        if let Some(entry) = schema_entry_mut(document, group, table, true) {
            entry["count"] = Value::from(count);
            entry["rows"] = Value::Array(rows);
            return;
        }
    }
}

/// Detects the flat v1 document and rewrites it in place. The original bytes are
/// copied to `super.v1.json` once, before the first overwrite.
fn migrate_if_needed(path: &Path, raw: &str, document: &mut Value) -> Result<bool, String> {
    let is_v1 = document["require"].get("overview").is_some()
        && document["require"].get("project").is_none();
    if !is_v1 {
        return Ok(false);
    }

    let backup = path.with_file_name("super.v1.json");
    if !backup.exists() {
        write_private(&backup, raw.as_bytes())?;
    }

    let old = document.clone();
    let mut migrated = schema::skeleton();

    copy_project(
        &mut migrated,
        &old,
        Group::Require,
        "require",
        "overview.name",
        "name",
    );
    copy_project(
        &mut migrated,
        &old,
        Group::Require,
        "optional",
        "overview.language",
        "language",
    );
    copy_project(
        &mut migrated,
        &old,
        Group::Optional,
        "optional",
        "overview.author",
        "author",
    );
    copy_project(
        &mut migrated,
        &old,
        Group::Optional,
        "optional",
        "overview.description",
        "summarize",
    );
    copy_project(
        &mut migrated,
        &old,
        Group::Optional,
        "optional",
        "overview.target-audience",
        "target-audience",
    );

    for leaf in ["arc", "chapter", "word"] {
        copy_int(
            &mut migrated,
            &old,
            Group::Require,
            "require",
            &format!("overview.plan.{leaf}.count"),
            &format!("plan.{leaf}.count"),
        );
        copy_int(
            &mut migrated,
            &old,
            Group::Optional,
            "optional",
            &format!("overview.plan.{leaf}.tolerance"),
            &format!("plan.{leaf}.tolerance"),
        );
    }

    let mut genre_rows = Vec::new();
    if let Some(primary) = old_text(&old, "require", "genre.primary") {
        genre_rows.push(json!({ "genre_id": primary, "is_primary": true }));
    }
    if let Some(secondary) =
        old_value(&old, "optional", "genre.secondary").and_then(Value::as_array)
    {
        for item in secondary {
            if let Some(value) = item.as_str().map(str::trim).filter(|text| !text.is_empty()) {
                genre_rows.push(json!({ "genre_id": value, "is_primary": false }));
            }
        }
    }
    if !genre_rows.is_empty() {
        let count = genre_rows.len() as i64;
        write_table(&mut migrated, "project_genres", count, genre_rows);
    }

    let mut theme_row = Map::new();
    theme_row.insert("scope".to_owned(), json!("PROJECT"));
    let has_theme = old_text(&old, "require", "theme.question");
    if let Some(question) = has_theme.clone() {
        theme_row.insert("question".to_owned(), json!(question));
    }
    if let Some(kind) = old_text(&old, "optional", "theme.resolution-type") {
        theme_row.insert("resolution_type".to_owned(), json!(kind));
    }
    if let Some(resolution) = old_text(&old, "optional", "theme.resolution") {
        theme_row.insert("resolution".to_owned(), json!(resolution));
    }
    if let Some(message) = old_text(&old, "optional", "theme.message") {
        theme_row.insert("message".to_owned(), json!(message));
    }
    if has_theme.is_some() {
        write_table(&mut migrated, "themes", 1, vec![Value::Object(theme_row)]);
    }

    if let Some(scale) = old_text(&old, "optional", "world.setting-scale") {
        write_table(
            &mut migrated,
            "world_settings",
            1,
            vec![json!({ "type": scale })],
        );
    }
    if let Some(has_power) =
        old_value(&old, "optional", "world.has-power-system").and_then(Value::as_bool)
    {
        write_table(
            &mut migrated,
            "power_systems",
            i64::from(has_power),
            Vec::new(),
        );
    }

    *document = migrated;
    Ok(true)
}

/// Normalises a v2 document — tables split across groups, or a second
/// fields-only entry — into v2.1: exactly one entry per table, in its countable
/// group, carrying the v2.1 `fields` while `count`/`rows` survive unchanged.
fn migrate_groups_if_needed(document: &mut Value) -> bool {
    let before = document.clone();
    let skeleton = schema::skeleton();

    for table in schema::tables() {
        let Some(belongs) = schema::countable_group(table) else {
            continue;
        };
        let mut found = Vec::new();
        let mut count = Value::Null;
        let mut rows = Value::Array(Vec::new());
        for group in [Group::Require, Group::Optional] {
            for entry in schema_entries(document, group) {
                if entry["table"].as_str() != Some(table.as_str()) {
                    continue;
                }
                found.push(group);
                if entry.get("count").is_some() {
                    count = entry.get("count").cloned().unwrap_or(Value::Null);
                    rows = entry
                        .get("rows")
                        .cloned()
                        .unwrap_or_else(|| Value::Array(Vec::new()));
                }
            }
        }
        if found.len() == 1 && found[0] == belongs {
            continue;
        }

        for group in [Group::Require, Group::Optional] {
            if let Some(entries) = schema_entries_mut(document, group) {
                entries.retain(|entry| entry["table"].as_str() != Some(table.as_str()));
            }
        }
        let fields = schema::table_entry(&skeleton, table)
            .and_then(|entry| entry.get("fields"))
            .cloned()
            .unwrap_or_else(|| Value::Object(Map::new()));
        let entry = json!({
            "table": table,
            "count": count,
            "fields": fields,
            "rows": rows,
        });
        if let Some(entries) = schema_entries_mut(document, belongs) {
            entries.push(entry);
        }
    }

    let order = schema::tables();
    for group in [Group::Require, Group::Optional] {
        if let Some(entries) = schema_entries_mut(document, group) {
            entries.sort_by_key(|entry| {
                order
                    .iter()
                    .position(|known| known == entry["table"].as_str().unwrap_or_default())
                    .unwrap_or(usize::MAX)
            });
        }
    }

    *document != before
}

/// Migrates a v2.1 document to v3:
/// - Refreshes `fields` in each schema table using skeleton v3, keeping `count`/`rows` intact.
/// - Sorts `schema` according to `schema::tables()`.
/// - Updates top-level `$comment` to the v3 description from skeleton.
/// - Writes `super.v2.json` backup before the first rewrite.
/// - Idempotent: returns false if already v3, and never creates backup a second time.
fn migrate_v3_if_needed(path: &Path, raw: &str, document: &mut Value) -> Result<bool, String> {
    let before = document.clone();
    let skeleton = schema::skeleton();

    // Refresh `fields` for every table and ensure order matches schema::tables()
    for group in [Group::Require, Group::Optional] {
        let Some(entries) = schema_entries_mut(document, group) else {
            continue;
        };
        for entry in entries.iter_mut() {
            let Some(table_name) = entry["table"].as_str() else {
                continue;
            };
            if let Some(skeleton_fields) =
                schema::table_entry(&skeleton, table_name).and_then(|e| e.get("fields"))
            {
                entry["fields"] = skeleton_fields.clone();
            }
        }
        let order = schema::tables();
        entries.sort_by_key(|entry| {
            order
                .iter()
                .position(|known| known == entry["table"].as_str().unwrap_or_default())
                .unwrap_or(usize::MAX)
        });
    }

    // Refresh $comment if present in skeleton
    if let Some(comment) = skeleton
        .get("$comment")
        .filter(|&c| document.get("$comment") != Some(c))
    {
        document["$comment"] = comment.clone();
    }

    if *document == before {
        return Ok(false);
    }

    let backup = path.with_file_name("super.v2.json");
    if !backup.exists() {
        write_private(&backup, raw.as_bytes())?;
    }

    Ok(true)
}

fn schema_entries(document: &Value, group: Group) -> &[Value] {
    document[group.as_str()]["schema"]
        .as_array()
        .map_or(&[], Vec::as_slice)
}

fn schema_entries_mut(document: &mut Value, group: Group) -> Option<&mut Vec<Value>> {
    document
        .get_mut(group.as_str())?
        .get_mut("schema")?
        .as_array_mut()
}

fn write_private(path: &Path, bytes: &[u8]) -> Result<(), String> {
    #[cfg(unix)]
    {
        use std::{io::Write, os::unix::fs::OpenOptionsExt};
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o600)
            .open(path)
            .map_err(|error| format!("Cannot create migration backup: {error}"))?;
        file.write_all(bytes)
            .map_err(|error| format!("Cannot write migration backup: {error}"))?;
        file.sync_all()
            .map_err(|error| format!("Cannot sync migration backup: {error}"))
    }
    #[cfg(not(unix))]
    {
        fs::write(path, bytes).map_err(|error| format!("Cannot write migration backup: {error}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::super_json::{missing_required, required_paths};

    #[test]
    fn test_load_existing_does_not_create_a_missing_project() {
        let home = crate::test_support::TempHome::new("super-load-existing-missing");
        let directory = home.path().join("projects").join("missing");
        assert_eq!(load_existing("missing").unwrap(), None);
        assert!(!directory.exists());
    }

    #[test]
    fn test_load_existing_reads_an_existing_document() {
        let home = crate::test_support::TempHome::new("super-load-existing-present");
        let path = home
            .path()
            .join("projects")
            .join("existing")
            .join("super.json");
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(&path, r#"{"require":{"project":{"name":"Existing"}}}"#).unwrap();
        let document = load_existing("existing").unwrap().unwrap();
        assert_eq!(document["require"]["project"]["name"], "Existing");
    }

    #[test]
    fn test_super_json_skeleton_and_set_round_trip() {
        let _home = crate::test_support::TempHome::new("super-round-trip");

        let fresh = load_or_init("default").unwrap();
        assert_eq!(fresh, schema::skeleton());

        set("default", "require.project.name", json!("Nửa Đêm")).unwrap();
        set("default", "require.schema.characters.count", json!(2)).unwrap();
        set(
            "default",
            "require.schema.characters.rows",
            json!([
                { "name": "Hoàng", "is_primary": true },
                { "name": "Linh", "is_primary": false }
            ]),
        )
        .unwrap();

        assert_eq!(
            get("default", "require.project.name").unwrap(),
            json!("Nửa Đêm")
        );
        assert_eq!(
            get("default", "require.schema.characters.count").unwrap(),
            json!(2)
        );
        let entry = get("default", "require.schema.characters").unwrap();
        assert_eq!(entry["rows"][1]["name"], json!("Linh"));

        let missing = missing_required("default").unwrap();
        assert_eq!(missing.len(), required_paths().len() - 2);
        assert!(!missing.contains(&"require.project.name".to_string()));
        assert!(!missing.contains(&"require.schema.characters".to_string()));
    }

    #[test]
    fn test_rows_reject_count_mismatch_and_unknown_field() {
        let _home = crate::test_support::TempHome::new("super-rows");
        load_or_init("default").unwrap();
        set("default", "require.project.name", json!("T")).unwrap();
        set("default", "require.schema.characters.count", json!(3)).unwrap();

        let error = set(
            "default",
            "require.schema.characters.rows",
            json!([{ "name": "Hoàng", "is_primary": true }]),
        )
        .unwrap_err();
        assert!(error.contains("count is 3"), "{error}");

        let error = set(
            "default",
            "require.schema.characters.rows",
            json!([
                { "name": "Hoàng", "is_primary": true, "superpower": "flying" },
                { "name": "Phong", "is_primary": false },
                { "name": "Linh", "is_primary": false }
            ]),
        )
        .unwrap_err();
        assert!(error.contains("unknown field"), "{error}");
    }

    #[test]
    fn test_migration_from_v1_document_moves_mapped_leaves() {
        let home = crate::test_support::TempHome::new("super-migrate");
        let project = "old";
        let path = paths::super_json_path(project).unwrap();
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        let old = json!({
            "require": {
                "overview": {
                    "name": "Nửa Đêm",
                    "plan": { "arc": { "count": 5 }, "chapter": { "count": 120 }, "word": { "count": 90000 } }
                },
                "genre": { "primary": "trinh thám" },
                "theme": { "question": "ký ức có làm nên con người?" }
            },
            "optional": {
                "overview": { "language": "VI", "author": "A", "description": "Tóm tắt" },
                "genre": { "secondary": ["huyền bí"] },
                "theme": { "resolution-type": "OPEN", "message": "thông điệp" },
                "world": { "has-power-system": false, "setting-scale": "CITY" }
            }
        });
        fs::write(&path, serde_json::to_vec_pretty(&old).unwrap()).unwrap();

        let document = load_or_init(project).unwrap();
        assert_eq!(document["require"]["project"]["name"], json!("Nửa Đêm"));
        assert_eq!(document["require"]["project"]["language"], json!("VI"));
        assert_eq!(
            document["require"]["project"]["plan"]["arc"]["count"],
            json!(5)
        );
        assert_eq!(
            document["require"]["schema"][0]["rows"][0]["genre_id"],
            json!("trinh thám")
        );
        assert!(path.with_file_name("super.v1.json").exists());

        let again = load_or_init(project).unwrap();
        assert_eq!(again, document, "migration must be idempotent");
        let _ = home;
    }

    #[test]
    fn test_migration_from_v2_groups_to_v2_1() {
        let _home = crate::test_support::TempHome::new("super-migrate-groups");
        let project = "v2";
        let path = paths::super_json_path(project).unwrap();
        fs::create_dir_all(path.parent().unwrap()).unwrap();

        let skeleton = schema::skeleton();
        let mut v2 = skeleton.clone();
        let take = |document: &mut Value, group: &str, table: &str| -> Value {
            let entries = document[group]["schema"].as_array_mut().unwrap();
            let index = entries
                .iter()
                .position(|entry| entry["table"] == table)
                .unwrap();
            entries.remove(index)
        };

        // v2: `project_motifs` lived in optional with only `motif_id`.
        let mut motifs = take(&mut v2, "require", "project_motifs");
        motifs["fields"] = json!({ "motif_id": null });
        v2["optional"]["schema"]
            .as_array_mut()
            .unwrap()
            .push(motifs);
        // v2: `scenes` lived in require, carrying real data.
        let mut scenes = take(&mut v2, "optional", "scenes");
        scenes["count"] = json!(1);
        scenes["rows"] = json!([{ "name": "Tiếng chuông", "chapter_id": "c1", "position": 1 }]);
        v2["require"]["schema"].as_array_mut().unwrap().push(scenes);
        // v2: `characters` was two entries, the second fields-only.
        v2["optional"]["schema"]
            .as_array_mut()
            .unwrap()
            .push(json!({
                "table": "characters",
                "fields": { "birth": null, "species": null }
            }));
        fs::write(&path, serde_json::to_vec_pretty(&v2).unwrap()).unwrap();

        let document = load_or_init(project).unwrap();
        for table in schema::tables() {
            let entries = ["require", "optional"]
                .iter()
                .filter(|group| {
                    document[group]["schema"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .any(|entry| entry["table"].as_str() == Some(table.as_str()))
                })
                .count();
            assert_eq!(entries, 1, "{table} must have exactly one entry");
        }
        assert_eq!(document["require"]["schema"].as_array().unwrap().len(), 6);
        assert_eq!(document["optional"]["schema"].as_array().unwrap().len(), 28);

        let motifs = schema::countable_entry(&document, "project_motifs").unwrap();
        assert_eq!(
            motifs["fields"],
            skeleton["require"]["schema"]
                .as_array()
                .unwrap()
                .iter()
                .find(|entry| entry["table"] == "project_motifs")
                .unwrap()["fields"]
        );
        assert_eq!(motifs["count"], Value::Null);
        assert_eq!(
            schema::countable_group("project_motifs"),
            Some(Group::Require)
        );

        let scenes = schema::countable_entry(&document, "scenes").unwrap();
        assert_eq!(scenes["count"], json!(1));
        assert_eq!(scenes["rows"][0]["name"], json!("Tiếng chuông"));
        assert_eq!(
            scenes["fields"],
            skeleton["optional"]["schema"]
                .as_array()
                .unwrap()
                .iter()
                .find(|entry| entry["table"] == "scenes")
                .unwrap()["fields"]
        );
        assert_eq!(schema::countable_group("scenes"), Some(Group::Optional));

        let on_disk: Value = serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
        assert_eq!(on_disk, document);

        let again = load_or_init(project).unwrap();
        assert_eq!(again, document, "migration must be idempotent");
    }

    #[test]
    fn test_migration_v2_to_v3_is_idempotent() {
        let _home = crate::test_support::TempHome::new("super-migrate-v2-to-v3");
        let project = "v2-proj";
        let path = paths::super_json_path(project).unwrap();
        fs::create_dir_all(path.parent().unwrap()).unwrap();

        // Create a v2.1 super.json (missing v3 fields in `fields` like genre_code, beats, etc.)
        let skeleton = schema::skeleton();
        let mut v2_doc = skeleton.clone();
        v2_doc["$comment"] = json!("SUPER JSON v2.1");

        // Strip a newly added field from scenes.fields and chapters.fields to make it strictly v2.1
        for group in ["require", "optional"] {
            for entry in v2_doc[group]["schema"].as_array_mut().unwrap() {
                if entry["table"] == "scenes" {
                    let fields = entry["fields"].as_object_mut().unwrap();
                    fields.remove("directives");
                    fields.remove("rules");
                    fields.remove("status");
                }
                if entry["table"] == "chapters" {
                    let fields = entry["fields"].as_object_mut().unwrap();
                    fields.remove("beats");
                    fields.remove("scene_count");
                }
            }
        }
        // Add a row to scenes to ensure rows survive migration
        let scenes_entry = v2_doc["optional"]["schema"]
            .as_array_mut()
            .unwrap()
            .iter_mut()
            .find(|e| e["table"] == "scenes")
            .unwrap();
        scenes_entry["count"] = json!(1);
        scenes_entry["rows"] = json!([{ "name": "Cảnh 1", "position": 1 }]);

        let v2_bytes = serde_json::to_vec_pretty(&v2_doc).unwrap();
        fs::write(&path, &v2_bytes).unwrap();

        let backup = path.with_file_name("super.v2.json");
        assert!(!backup.exists());

        // First load triggers migration to v3
        let migrated = load_or_init(project).unwrap();
        assert!(
            backup.exists(),
            "backup super.v2.json must exist after first migration"
        );
        let backup_content = fs::read(&backup).unwrap();
        assert_eq!(
            backup_content, v2_bytes,
            "backup must match original v2.1 bytes"
        );

        // Verify v3 fields were refreshed
        let scenes = schema::countable_entry(&migrated, "scenes").unwrap();
        assert!(scenes["fields"].get("directives").is_some());
        assert!(scenes["fields"].get("rules").is_some());
        assert!(scenes["fields"].get("status").is_some());
        assert_eq!(scenes["count"], json!(1));
        assert_eq!(scenes["rows"][0]["name"], json!("Cảnh 1"));

        let disk_bytes_after_first = fs::read(&path).unwrap();

        // Second load: idempotent check
        let second = load_or_init(project).unwrap();
        assert_eq!(migrated, second);
        let disk_bytes_after_second = fs::read(&path).unwrap();
        assert_eq!(
            disk_bytes_after_first, disk_bytes_after_second,
            "load lần hai không đổi byte nào"
        );
    }

    #[test]
    fn test_super_json_rejects_unknown_path() {
        let _home = crate::test_support::TempHome::new("super-unknown");
        for bad in ["require.overview.name", "project.name", "", "require"] {
            let error = set("default", bad, json!("x")).unwrap_err();
            assert!(
                error.contains("Unknown super json path"),
                "unexpected error for {bad}: {error}"
            );
        }
    }

    #[test]
    fn test_super_json_rejects_wrong_type() {
        let _home = crate::test_support::TempHome::new("super-type");
        load_or_init("default").unwrap();
        assert!(set("default", "require.project.plan.arc.count", json!("5")).is_err());
        assert!(set("default", "require.project.plan.arc.count", json!(0)).is_err());
        assert!(set("default", "optional.project.plan.arc.tolerance", json!(0)).is_ok());
        assert!(set("default", "require.project.name", json!("   ")).is_err());
        assert!(set("default", "require.project.language", json!("vi")).is_err());
        assert!(set("default", "require.project.language", json!("VI")).is_ok());
    }

    #[test]
    fn test_add_row_appends_and_tracks_count() {
        let _home = crate::test_support::TempHome::new("super-add-row");
        load_or_init("p").unwrap();

        let (index, count) = add_row(
            "p",
            "characters",
            json!({ "name": "Hoàng", "is_primary": true }),
        )
        .unwrap();
        assert_eq!((index, count), (0, 1));

        let entry = get("p", "require.schema.characters").unwrap();
        assert_eq!(entry["count"], json!(1));
        assert_eq!(entry["rows"][0]["_json_index"], json!(0));

        // A count already equal to the old length advances with the append.
        let (index, count) = add_row(
            "p",
            "characters",
            json!({ "name": "Phong", "is_primary": false }),
        )
        .unwrap();
        assert_eq!((index, count), (1, 2));
        let entry = get("p", "require.schema.characters").unwrap();
        assert_eq!(entry["rows"][1]["_json_index"], json!(1));
        assert_eq!(entry["rows"][1]["name"], json!("Phong"));

        // Draft rows only need to be objects, and the table must exist.
        let error = add_row("p", "nope", json!({})).unwrap_err();
        assert!(error.contains("Unknown super json table: nope"), "{error}");
        assert!(
            error.contains("one of:"),
            "the error must list the valid tables: {error}"
        );
        let error = add_row("p", "characters", json!("Hoàng")).unwrap_err();
        assert!(error.contains("must be an object"), "{error}");
    }

    #[test]
    fn test_add_row_keeps_planned_count() {
        let _home = crate::test_support::TempHome::new("super-add-row-plan");
        load_or_init("p").unwrap();
        set("p", "require.schema.characters.count", json!(5)).unwrap();

        let (index, count) = add_row("p", "characters", json!({ "name": "Hoàng" })).unwrap();
        assert_eq!((index, count), (0, 5), "a planned count must survive");

        // Draft mode accepts a row that still misses a required field.
        let entry = get("p", "require.schema.characters").unwrap();
        assert_eq!(entry["count"], json!(5));
        assert_eq!(entry["rows"].as_array().unwrap().len(), 1);
        assert_eq!(entry["rows"][0]["name"], json!("Hoàng"));
    }

    #[test]
    fn test_add_field_updates_one_row() {
        let _home = crate::test_support::TempHome::new("super-add-field");
        load_or_init("p").unwrap();
        add_row(
            "p",
            "characters",
            json!({ "name": "Hoàng", "is_primary": true }),
        )
        .unwrap();

        add_field("p", "characters", 0, "birth", json!("Mùa đông")).unwrap();
        let entry = get("p", "require.schema.characters").unwrap();
        assert_eq!(entry["rows"][0]["birth"], json!("Mùa đông"));
        assert_eq!(entry["rows"][0]["name"], json!("Hoàng"));

        let error = add_field("p", "characters", 0, "_json_index", json!(1)).unwrap_err();
        assert!(error.contains("cannot be written"), "{error}");
        let error = add_field("p", "characters", 0, "superpower", json!("flying")).unwrap_err();
        assert!(error.contains("Unknown field"), "{error}");
        let error = add_field("p", "characters", 9, "name", json!("Linh")).unwrap_err();
        assert_eq!(error, "Row 9 not found in characters.rows");
        let error = add_field("p", "nope", 0, "name", json!("Linh")).unwrap_err();
        assert!(error.contains("Unknown super json table: nope"), "{error}");
    }

    #[test]
    fn test_add_field_accepts_null_only_on_optional_columns() {
        let _home = crate::test_support::TempHome::new("super-add-field-null");
        load_or_init("p").unwrap();
        set("p", "optional.schema.scenes.count", json!(1)).unwrap();
        set(
            "p",
            "optional.schema.scenes.rows",
            json!([{ "name": "Mở đầu", "chapter_id": "c1", "position": 1 }]),
        )
        .unwrap();

        // An explicit null clears an optional column (world_setting_id may be empty).
        add_field("p", "scenes", 0, "world_setting_id", Value::Null).unwrap();
        let entry = get("p", "optional.schema.scenes").unwrap();
        assert_eq!(entry["rows"][0]["world_setting_id"], Value::Null);

        // A required column cannot be nulled.
        let error = add_field("p", "scenes", 0, "name", Value::Null).unwrap_err();
        assert!(error.contains("required field cannot be null"), "{error}");
    }

    #[test]
    fn test_set_rows_renumbers_json_index() {
        let _home = crate::test_support::TempHome::new("super-renumber");
        load_or_init("p").unwrap();
        set("p", "require.schema.characters.count", json!(2)).unwrap();
        set(
            "p",
            "require.schema.characters.rows",
            json!([
                { "name": "Hoàng", "is_primary": true, "_json_index": 9 },
                { "name": "Linh", "is_primary": false, "_json_index": 7 }
            ]),
        )
        .unwrap();

        let entry = get("p", "require.schema.characters").unwrap();
        assert_eq!(entry["rows"][0]["_json_index"], json!(0));
        assert_eq!(entry["rows"][1]["_json_index"], json!(1));
    }
}
