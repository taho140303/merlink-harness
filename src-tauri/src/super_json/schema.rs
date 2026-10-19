//! The documented skeleton, derived writable paths/units, and per-field type rules.
//!
//! The super JSON is a project-rooted document: one `project` block (13 fixed
//! leaves) plus 34 schema tables. Each table contributes exactly two writable
//! paths (`count` and `rows`); `fields` is a read-only per-row contract.
//! `fields` is deliberately absent from [`valid_paths`]: the model reads it with
//! `super_json_get` (table selector) and then replaces `rows` wholesale.

use serde_json::Value;
use std::sync::OnceLock;

/// The canonical skeleton lives in the docs so the specification and the code
/// can never drift apart.
const SKELETON_JSON: &str = include_str!("../../../docs/schema/runtime/super.json");

/// Root-level keys that carry documentation, not data.
const NON_DATA_KEYS: [&str; 1] = ["$comment"];

/// Parsed skeleton, with documentation keys removed so the file we write to a
/// user's project directory contains data only.
pub fn skeleton() -> Value {
    let mut document: Value =
        serde_json::from_str(SKELETON_JSON).expect("embedded super.json must be valid JSON");
    if let Some(object) = document.as_object_mut() {
        for key in NON_DATA_KEYS {
            object.remove(key);
        }
    }
    document
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Group {
    Require,
    Optional,
}

impl Group {
    pub fn as_str(self) -> &'static str {
        match self {
            Group::Require => "require",
            Group::Optional => "optional",
        }
    }

    fn from_str(text: &str) -> Option<Self> {
        match text {
            "require" => Some(Group::Require),
            "optional" => Some(Group::Optional),
            _ => None,
        }
    }
}

/// One writable address. `TableSelector` is read-only and exists only for
/// `super_json_get`; it returns the whole table entry.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PathRef<'a> {
    Project { group: Group, rest: &'a str },
    SchemaCount { group: Group, table: &'a str },
    SchemaRows { group: Group, table: &'a str },
    TableSelector { group: Group, table: &'a str },
}

/// Parses the stable, index-free grammar:
/// `require.project.<leaf>`, `require.schema.<table>.count|rows`,
/// and the read-only `require.schema.<table>`.
pub fn parse_path(path: &str) -> Option<PathRef<'_>> {
    let (group_text, rest) = path.split_once('.')?;
    let group = Group::from_str(group_text)?;
    if let Some(leaf) = rest.strip_prefix("project.") {
        if leaf.is_empty() || leaf.starts_with('.') {
            return None;
        }
        return Some(PathRef::Project { group, rest: leaf });
    }
    let tail = rest.strip_prefix("schema.")?;
    let (table, suffix) = match tail.split_once('.') {
        Some((table, suffix)) => (table, Some(suffix)),
        None => (tail, None),
    };
    if table.is_empty() {
        return None;
    }
    match suffix {
        Some("count") => Some(PathRef::SchemaCount { group, table }),
        Some("rows") => Some(PathRef::SchemaRows { group, table }),
        Some(_) => None,
        None => Some(PathRef::TableSelector { group, table }),
    }
}

fn project_object(document: &Value, group: Group) -> Option<&Value> {
    document.get(group.as_str())?.get("project")
}

fn collect_leaves(value: &Value, prefix: &str, out: &mut Vec<String>) {
    match value {
        Value::Object(map) => {
            for (key, child) in map {
                if key.starts_with('$') {
                    continue;
                }
                let path = if prefix.is_empty() {
                    key.clone()
                } else {
                    format!("{prefix}.{key}")
                };
                collect_leaves(child, &path, out);
            }
        }
        _ => out.push(prefix.to_owned()),
    }
}

/// Project leaves (`plan.arc.count`, …) for one group, sorted.
pub fn project_leaves(group: Group) -> Vec<String> {
    let document = skeleton();
    let mut out = Vec::new();
    if let Some(project) = project_object(&document, group) {
        collect_leaves(project, "", &mut out);
    }
    out.sort();
    out
}

/// Every writable path: 13 project leaves + 34 tables × {count, rows}.
pub fn valid_paths() -> &'static [String] {
    static CACHE: OnceLock<Vec<String>> = OnceLock::new();
    CACHE.get_or_init(|| {
        let document = skeleton();
        let mut out = Vec::new();
        for group in [Group::Require, Group::Optional] {
            for leaf in project_leaves(group) {
                out.push(format!("{}.project.{leaf}", group.as_str()));
            }
            if let Some(entries) = document[group.as_str()]["schema"].as_array() {
                for entry in entries {
                    let Some(table) = entry["table"].as_str() else {
                        continue;
                    };
                    if entry.get("count").is_some() {
                        out.push(format!("{}.schema.{table}.count", group.as_str()));
                    }
                    if entry.get("rows").is_some() {
                        out.push(format!("{}.schema.{table}.rows", group.as_str()));
                    }
                }
            }
        }
        out.sort();
        out
    })
}

fn units(group: Group) -> Vec<String> {
    let document = skeleton();
    let mut out = Vec::new();
    for leaf in project_leaves(group) {
        out.push(format!("{}.project.{leaf}", group.as_str()));
    }
    if let Some(entries) = document[group.as_str()]["schema"].as_array() {
        for entry in entries {
            let Some(table) = entry["table"].as_str() else {
                continue;
            };
            if entry.get("count").is_some() {
                out.push(format!("{}.schema.{table}", group.as_str()));
            }
        }
    }
    out.sort();
    out
}

/// Blocking decision units for `require` (project leaves + tables).
pub fn required_paths() -> &'static [String] {
    static CACHE: OnceLock<Vec<String>> = OnceLock::new();
    CACHE.get_or_init(|| units(Group::Require))
}

/// Fillable units for `optional` (project leaves + tables).
pub fn optional_paths() -> &'static [String] {
    static CACHE: OnceLock<Vec<String>> = OnceLock::new();
    CACHE.get_or_init(|| units(Group::Optional))
}

/// Unique table names in skeleton order (first appearance), 34 entries.
#[allow(dead_code)]
pub fn tables() -> &'static [String] {
    static CACHE: OnceLock<Vec<String>> = OnceLock::new();
    CACHE.get_or_init(|| {
        let document = skeleton();
        let mut out = Vec::new();
        for group in [Group::Require, Group::Optional] {
            if let Some(entries) = document[group.as_str()]["schema"].as_array() {
                for entry in entries {
                    if let Some(table) = entry["table"].as_str()
                        && !out.iter().any(|known| known == table)
                    {
                        out.push(table.to_owned());
                    }
                }
            }
        }
        out
    })
}

/// The group that owns a table's `count`/`rows` entry.
pub fn countable_group(table: &str) -> Option<Group> {
    let document = skeleton();
    for group in [Group::Require, Group::Optional] {
        if let Some(entries) = document[group.as_str()]["schema"].as_array() {
            for entry in entries {
                if entry["table"].as_str() == Some(table) && entry.get("count").is_some() {
                    return Some(group);
                }
            }
        }
    }
    None
}

/// The entry that owns `count`/`rows` for a table (at most one, in one group).
pub fn countable_entry<'a>(document: &'a Value, table: &str) -> Option<&'a Value> {
    for group in [Group::Require, Group::Optional] {
        if let Some(entries) = document[group.as_str()]["schema"].as_array() {
            for entry in entries {
                if entry["table"].as_str() == Some(table) && entry.get("count").is_some() {
                    return Some(entry);
                }
            }
        }
    }
    None
}

/// First entry for a table, used by the read-only table selector.
pub fn table_entry<'a>(document: &'a Value, table: &str) -> Option<&'a Value> {
    for group in [Group::Require, Group::Optional] {
        if let Some(entries) = document[group.as_str()]["schema"].as_array() {
            for entry in entries {
                if entry["table"].as_str() == Some(table) {
                    return Some(entry);
                }
            }
        }
    }
    None
}

/// One read-only view of a table: merged `fields` across both groups plus the
/// `count`/`rows` owned by the countable entry. This is what `super_json_get`
/// returns for a table selector.
pub fn merged_table_entry(document: &Value, table: &str) -> Option<Value> {
    let mut fields = serde_json::Map::new();
    let mut found = false;
    for group in [Group::Require, Group::Optional] {
        if let Some(entries) = document[group.as_str()]["schema"].as_array() {
            for entry in entries {
                if entry["table"].as_str() == Some(table) {
                    found = true;
                    if let Some(map) = entry["fields"].as_object() {
                        for (key, value) in map {
                            fields.insert(key.clone(), value.clone());
                        }
                    }
                }
            }
        }
    }
    if !found {
        return None;
    }
    let countable = countable_entry(document, table);
    Some(serde_json::json!({
        "table": table,
        "count": countable.and_then(|entry| entry.get("count")).cloned().unwrap_or(Value::Null),
        "fields": Value::Object(fields),
        "rows": countable.and_then(|entry| entry.get("rows")).cloned().unwrap_or_else(|| Value::Array(Vec::new())),
    }))
}

/// Union of `fields` keys for a table across both groups.
pub fn table_fields(table: &str) -> Vec<String> {
    let document = skeleton();
    let mut out: Vec<String> = Vec::new();
    for group in [Group::Require, Group::Optional] {
        if let Some(entries) = document[group.as_str()]["schema"].as_array() {
            for entry in entries {
                if entry["table"].as_str() == Some(table)
                    && let Some(fields) = entry["fields"].as_object()
                {
                    for key in fields.keys() {
                        if !out.iter().any(|known| known == key) {
                            out.push(key.clone());
                        }
                    }
                }
            }
        }
    }
    out
}

/// Fields every row must carry, independent of the table's group. A table that
/// moves from require to optional must not silently lose its row contract.
const REQUIRED_ROW_FIELDS: [(&str, &[&str]); 13] = [
    ("project_genres", &["genre_id", "is_primary"]),
    ("arcs", &["name", "position"]),
    ("characters", &["name", "is_primary"]),
    ("project_motifs", &["motif_id", "scope"]),
    ("themes", &["scope", "question"]),
    ("theme_arguments", &["theme_id", "character_id", "role"]),
    ("chapters", &["name", "position"]),
    ("scenes", &["name", "chapter_id", "position"]),
    ("scene_characters", &["scene_id", "character_id", "is_pov"]),
    ("relationships", &["from_character_id", "to_character_id"]),
    (
        "relationship_roles",
        &["relationship_id", "category_id", "is_primary"],
    ),
    ("world_settings", &["name", "type"]),
    ("power_systems", &["name", "source"]),
];

pub fn required_row_fields(table: &str) -> Vec<String> {
    REQUIRED_ROW_FIELDS
        .iter()
        .find(|(known, _)| *known == table)
        .map(|(_, fields)| fields.iter().map(|field| (*field).to_owned()).collect())
        .unwrap_or_default()
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FieldKind {
    Text,
    PositiveInt,
    NonNegativeInt,
    Bool,
    Number,
    Json,
    Rows,
    Choice(&'static [&'static str]),
}

const LANGUAGES: &[&str] = &["VI", "EN", "JA", "ZH_HANT", "ZH_HANS"];
const THEME_SCOPES: &[&str] = &["PROJECT", "ARC"];
const THEME_RESOLUTIONS: &[&str] = &["RESOLVED", "OPEN", "AMBIGUOUS"];
const THEME_ROLES: &[&str] = &["PRIMARY", "ANTITHESIS", "CAUTIONARY", "WITNESS"];
const THEME_ANSWER_MODES: &[&str] = &["VERBAL", "ACTION", "OUTCOME", "IMPLICIT"];
const PROJECT_MOTIF_SCOPES: &[&str] = &["project", "arc", "chapter"];
const LOCATION_TYPES: &[&str] = &[
    "REALM", "WORLD", "REGION", "COUNTRY", "CITY", "TOWN", "FACILITY", "STREET", "ROOM",
];
const LOCATION_ACCESS: &[&str] = &[
    "PUBLIC",
    "PRIVATE",
    "SECRET",
    "LOCKED",
    "GUARDED",
    "RESTRICTED",
];
const LORE_TYPES: &[&str] = &["CULTURE", "HISTORY", "FACTION", "ITEM"];
const MEMORY_TYPES: &[&str] = &["EPISODIC", "SEMANTIC", "PROCEDURAL"];
const MEMORY_CERTAINTY: &[&str] = &["CERTAIN", "PROBABLE", "UNCERTAIN"];
const MEMORY_SOURCES: &[&str] = &["FIRSTHAND", "HEARSAY", "INFERENCE"];
const MEMORY_STATUSES: &[&str] = &["ACTIVE", "SUPPRESSED", "FORGOTTEN", "DISTORTED"];
const SCENE_SOURCES: &[&str] = &["AI_GENERATED", "HUMAN_EDITED", "RESTORE"];
const HIERARCHY: &[&str] = &["SUPERIOR", "EQUAL", "INFERIOR"];
const DIMENSION_LEVELS: &[&str] = &["NONE", "LOW", "MODERATE", "HIGH", "ABSOLUTE"];
const CATEGORY_GROUPS: &[&str] = &[
    "FAMILY",
    "ROMANTIC",
    "SOCIAL",
    "PROFESSIONAL",
    "ANTAGONISTIC",
];
const GENDERS: &[&str] = &["MALE", "FEMALE"];
const MEASUREMENT_UNITS: &[&str] = &["MM", "CM", "M", "HA", "KM", "ML", "L", "MG", "G", "KG"];
const CERTAINTY_LEVELS: &[&str] = &["LOW", "MEDIUM", "HIGH"];
const KNOWLEDGE_SOURCE_TYPES: &[&str] = &["INITIAL", "LORE"];

/// Type rule for one project leaf.
fn project_kind(rest: &str) -> Option<FieldKind> {
    Some(match rest {
        "name" | "author" | "summarize" | "target-audience" => FieldKind::Text,
        "language" => FieldKind::Choice(LANGUAGES),
        "plan.arc.count" | "plan.chapter.count" | "plan.scene.count" | "plan.word.count" => {
            FieldKind::PositiveInt
        }
        "plan.arc.tolerance"
        | "plan.chapter.tolerance"
        | "plan.scene.tolerance"
        | "plan.word.tolerance" => FieldKind::NonNegativeInt,
        _ => return None,
    })
}

/// Table-aware type rule for one row column. Everything not listed falls back
/// to `Text`, the dominant DBML type (varchar/text/uuid FK).
pub fn column_kind(table: &str, column: &str) -> FieldKind {
    match column {
        "is_primary" | "is_system" | "is_pov" => return FieldKind::Bool,
        "position" | "rank" | "height" | "weight" | "word_count" | "version_number"
        | "revision_number" | "count" => return FieldKind::NonNegativeInt,
        "significance" | "intensity" => return FieldKind::Number,
        "data" => return FieldKind::Json,
        _ => {}
    }

    match (table, column) {
        ("themes", "scope") => FieldKind::Choice(THEME_SCOPES),
        ("project_motifs", "scope") => FieldKind::Choice(PROJECT_MOTIF_SCOPES),
        ("themes", "resolution_type") => FieldKind::Choice(THEME_RESOLUTIONS),
        ("theme_arguments", "role") => FieldKind::Choice(THEME_ROLES),
        ("theme_arguments", "answer_mode") => FieldKind::Choice(THEME_ANSWER_MODES),
        ("world_settings", "type") => FieldKind::Choice(LOCATION_TYPES),
        ("world_settings", "access") => FieldKind::Choice(LOCATION_ACCESS),
        ("lore_entries", "type") => FieldKind::Choice(LORE_TYPES),
        ("memories", "type") => FieldKind::Choice(MEMORY_TYPES),
        ("memories", "certainty") => FieldKind::Choice(MEMORY_CERTAINTY),
        ("memories", "knowledge_source") => FieldKind::Choice(MEMORY_SOURCES),
        ("memories", "status") => FieldKind::Choice(MEMORY_STATUSES),
        ("scene_versions", "scene_source") => FieldKind::Choice(SCENE_SOURCES),
        ("relationship_roles", "hierarchy") => FieldKind::Choice(HIERARCHY),
        ("relationship_categories", "category") => FieldKind::Choice(CATEGORY_GROUPS),
        ("relationship_states", "trust")
        | ("relationship_states", "affection")
        | ("relationship_states", "hostility")
        | ("relationship_states", "fear")
        | ("relationship_states", "respect")
        | ("relationship_states", "familiarity")
        | ("relationship_states", "dependency") => FieldKind::Choice(DIMENSION_LEVELS),
        ("characters", "biological_sex")
        | ("characters", "gender_identity")
        | ("characters", "gender_expression") => FieldKind::Choice(GENDERS),
        ("character_appearances", "height_unit") | ("character_appearances", "weight_unit") => {
            FieldKind::Choice(MEASUREMENT_UNITS)
        }
        ("character_knowledge_sources", "type") => FieldKind::Choice(KNOWLEDGE_SOURCE_TYPES),
        ("character_knowledge_sources", "certainty") | ("character_competences", "reliability") => {
            FieldKind::Choice(CERTAINTY_LEVELS)
        }
        _ => FieldKind::Text,
    }
}

/// Type rule for a writable path, or `None` when the path is not writable.
pub fn field_kind(path: &str) -> Option<FieldKind> {
    if let Some(rest) = path
        .strip_prefix("require.project.")
        .or_else(|| path.strip_prefix("optional.project."))
    {
        return project_kind(rest);
    }
    match parse_path(path)? {
        PathRef::SchemaCount { .. } => Some(FieldKind::NonNegativeInt),
        PathRef::SchemaRows { .. } => Some(FieldKind::Rows),
        PathRef::TableSelector { .. } | PathRef::Project { .. } => None,
    }
}

pub(super) fn validate(kind: FieldKind, path: &str, value: &Value) -> Result<(), String> {
    let type_error = |expected: &str| {
        format!(
            "Invalid value for {path}: expected {expected}, got {}",
            describe(value)
        )
    };
    match kind {
        FieldKind::Text => match value.as_str() {
            Some(text) if !text.trim().is_empty() => Ok(()),
            Some(_) => Err(format!("Invalid value for {path}: must not be empty")),
            None => Err(type_error("a non-empty string")),
        },
        FieldKind::PositiveInt => match value.as_i64() {
            Some(number) if number > 0 => Ok(()),
            Some(_) => Err(format!("Invalid value for {path}: must be greater than 0")),
            None => Err(type_error("a positive integer")),
        },
        FieldKind::NonNegativeInt => match value.as_i64() {
            Some(number) if number >= 0 => Ok(()),
            Some(_) => Err(format!("Invalid value for {path}: must be 0 or greater")),
            None => Err(type_error("an integer >= 0")),
        },
        FieldKind::Bool => match value.as_bool() {
            Some(_) => Ok(()),
            None => Err(type_error("a boolean")),
        },
        FieldKind::Number => match value.as_f64() {
            Some(number) if (0.0..=1.0).contains(&number) => Ok(()),
            Some(_) => Err(format!("Invalid value for {path}: must be between 0 and 1")),
            None => Err(type_error("a number between 0 and 1")),
        },
        FieldKind::Json => {
            if value.is_object() || value.is_array() || value.is_string() {
                Ok(())
            } else {
                Err(type_error("a JSON object, array, or string"))
            }
        }
        FieldKind::Rows => Err(format!(
            "Invalid value for {path}: rows are validated by the table contract"
        )),
        FieldKind::Choice(allowed) => match value.as_str() {
            Some(text) if allowed.contains(&text) => Ok(()),
            Some(text) => Err(format!(
                "Invalid value for {path}: {text:?} is not one of {}",
                allowed.join(", ")
            )),
            None => Err(type_error(&format!("one of {}", allowed.join(", ")))),
        },
    }
}

fn describe(value: &Value) -> String {
    match value {
        Value::Null => "null".into(),
        Value::Bool(inner) => format!("boolean {inner}"),
        Value::Number(inner) => format!("number {inner}"),
        Value::String(inner) => format!("string {inner:?}"),
        Value::Array(_) => "an array".into(),
        Value::Object(_) => "an object".into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_skeleton_has_81_paths_and_34_tables() {
        assert_eq!(valid_paths().len(), 81, "13 project + 34×2");
        assert_eq!(tables().len(), 34);
        assert!(valid_paths().contains(&"require.project.name".to_owned()));
        assert!(valid_paths().contains(&"require.schema.characters.rows".to_owned()));
        assert!(valid_paths().contains(&"require.schema.themes.count".to_owned()));
    }

    #[test]
    fn test_required_units_are_12_and_optional_35() {
        assert_eq!(required_paths().len(), 12);
        assert_eq!(optional_paths().len(), 35);
        assert!(required_paths().contains(&"require.schema.characters".to_owned()));
        assert!(required_paths().contains(&"require.schema.project_motifs".to_owned()));
        assert!(!required_paths().contains(&"require.schema.scenes".to_owned()));
        for table in [
            "scenes",
            "chapters",
            "scene_characters",
            "relationships",
            "relationship_roles",
            "world_settings",
            "power_systems",
        ] {
            assert!(
                optional_paths().contains(&format!("optional.schema.{table}")),
                "{table} must be optional"
            );
        }
        assert!(optional_paths().contains(&"optional.schema.appearance_options".to_owned()));
    }

    #[test]
    fn test_one_entry_per_table() {
        let document = skeleton();
        assert_eq!(tables().len(), 34);
        for group in [Group::Require, Group::Optional] {
            for entry in document[group.as_str()]["schema"].as_array().unwrap() {
                let table = entry["table"].as_str().unwrap();
                let occurrences = [Group::Require, Group::Optional]
                    .iter()
                    .flat_map(|group| {
                        document[group.as_str()]["schema"]
                            .as_array()
                            .unwrap()
                            .iter()
                            .filter(move |entry| entry["table"] == table)
                    })
                    .count();
                assert_eq!(occurrences, 1, "{table} must have exactly one entry");
                assert!(entry.get("count").is_some(), "{table} needs count");
                assert!(entry.get("rows").is_some(), "{table} needs rows");
            }
        }
    }

    #[test]
    fn test_project_motifs_declares_scope_and_anchors() {
        let required = required_row_fields("project_motifs");
        for field in ["motif_id", "scope"] {
            assert!(required.contains(&field.to_owned()), "missing {field}");
        }
        let document = skeleton();
        let entry = document["require"]["schema"]
            .as_array()
            .unwrap()
            .iter()
            .find(|entry| entry["table"] == "project_motifs")
            .unwrap();
        assert_eq!(
            entry["fields"],
            serde_json::json!({
                "motif_id": null,
                "scope": null,
                "arc_id": null,
                "chapter_id": null
            })
        );
        assert_eq!(
            column_kind("project_motifs", "scope"),
            FieldKind::Choice(&["project", "arc", "chapter"])
        );
    }

    #[test]
    fn test_every_valid_path_has_a_type_rule() {
        for path in valid_paths() {
            assert!(field_kind(path).is_some(), "missing type rule for {path}");
        }
    }

    #[test]
    fn test_scenes_chapter_id_is_a_required_row_field() {
        let required = required_row_fields("scenes");
        for field in ["name", "chapter_id", "position"] {
            assert!(required.contains(&field.to_owned()), "missing {field}");
        }
        let document = skeleton();
        let entries: Vec<&Value> = ["require", "optional"]
            .iter()
            .flat_map(|group| document[group]["schema"].as_array().unwrap())
            .filter(|entry| entry["table"] == "scenes")
            .collect();
        assert_eq!(entries.len(), 1, "scenes must have exactly one entry");
        let optional = document["optional"]["schema"].as_array().unwrap();
        assert!(
            optional.iter().any(|entry| entry["table"] == "scenes"),
            "scenes must live in optional"
        );
        assert!(entries[0]["fields"].get("chapter_id").is_some());
    }

    #[test]
    fn test_parse_path_grammar() {
        assert_eq!(
            parse_path("require.schema.characters.rows"),
            Some(PathRef::SchemaRows {
                group: Group::Require,
                table: "characters"
            })
        );
        assert_eq!(
            parse_path("require.schema.characters"),
            Some(PathRef::TableSelector {
                group: Group::Require,
                table: "characters"
            })
        );
        assert_eq!(parse_path("optional.schema.characters.fields"), None);
        assert_eq!(
            parse_path("require.project.plan.arc.count"),
            Some(PathRef::Project {
                group: Group::Require,
                rest: "plan.arc.count"
            })
        );
    }

    #[test]
    fn test_characters_have_is_primary_synthetic_field() {
        assert!(table_fields("characters").contains(&"is_primary".to_owned()));
    }
}
