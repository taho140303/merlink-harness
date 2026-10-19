//! The documented skeleton, the derived leaf lists, and the per-field type rules.

use std::sync::OnceLock;

use serde_json::{Map, Value};

/// The canonical skeleton lives in the docs so the specification and the code
/// can never drift apart. `docs/design/super-json.md` explains every field.
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
                let is_empty_object = child.as_object().is_some_and(Map::is_empty);
                if is_empty_object {
                    out.push(path);
                } else {
                    collect_leaves(child, &path, out);
                }
            }
        }
        // nulls, scalars and arrays are all leaves.
        _ => out.push(prefix.to_string()),
    }
}

fn leaves_of(section: &str) -> Vec<String> {
    let document = skeleton();
    let mut out = Vec::new();
    if let Some(value) = document.get(section) {
        collect_leaves(value, section, &mut out);
    }
    out.sort();
    out
}

/// Every field the agent is allowed to touch, derived from the skeleton itself
/// rather than duplicated as a hand-written constant.
pub fn valid_paths() -> &'static [String] {
    static CACHE: OnceLock<Vec<String>> = OnceLock::new();
    CACHE.get_or_init(|| {
        let mut all = Vec::new();
        collect_leaves(&skeleton(), "", &mut all);
        all.sort();
        all
    })
}

/// Fields that block the "write outline" step until they are filled.
pub fn required_paths() -> &'static [String] {
    static CACHE: OnceLock<Vec<String>> = OnceLock::new();
    CACHE.get_or_init(|| leaves_of("require"))
}

/// The remaining fields: fillable at any time, and the ones that take the header
/// progress bar from 70% to 100%.
pub fn optional_paths() -> &'static [String] {
    static CACHE: OnceLock<Vec<String>> = OnceLock::new();
    CACHE.get_or_init(|| leaves_of("optional"))
}

#[derive(Clone, Copy)]
pub(super) enum FieldKind {
    Text,
    PositiveInt,
    NonNegativeInt,
    Bool,
    TextArray,
    Choice(&'static [&'static str]),
}

/// Type rules per field. Values start as `null`, so the type cannot be inferred
/// from the skeleton — it has to be declared. `test_every_valid_path_has_a_type_rule`
/// keeps this table in sync with `super.json`.
pub(super) fn field_kind(path: &str) -> Option<FieldKind> {
    use FieldKind::*;
    Some(match path {
        "require.overview.name"
        | "require.overview.logline.trigger-event"
        | "require.overview.logline.main-character"
        | "require.overview.logline.action"
        | "require.overview.logline.risk"
        | "require.overview.logline.major-loss"
        | "require.genre.primary"
        | "require.theme.question"
        | "optional.overview.description"
        | "optional.overview.author"
        | "optional.overview.target-audience"
        | "optional.overview.age-rating"
        | "optional.theme.resolution"
        | "optional.theme.message" => Text,

        "require.overview.plan.arc.count"
        | "require.overview.plan.chapter.count"
        | "require.overview.plan.word.count" => PositiveInt,

        "optional.overview.plan.arc.tolerance"
        | "optional.overview.plan.chapter.tolerance"
        | "optional.overview.plan.word.tolerance" => NonNegativeInt,

        "optional.overview.language" => Choice(&["VI", "EN", "JA", "ZH_HANT", "ZH_HANS"]),
        "optional.theme.resolution-type" => Choice(&["RESOLVED", "OPEN", "AMBIGUOUS"]),
        "optional.world.setting-scale" => Choice(&[
            "REALM", "WORLD", "REGION", "COUNTRY", "CITY", "TOWN", "FACILITY", "STREET", "ROOM",
        ]),

        "optional.world.has-power-system" => Bool,
        "optional.genre.secondary" => TextArray,

        _ => return None,
    })
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
            // Deliberately rejects "5": models like to send numbers as strings.
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
        FieldKind::TextArray => match value.as_array() {
            Some(items) => {
                if items.iter().all(|item| {
                    item.as_str().is_some_and(|text| !text.trim().is_empty())
                }) {
                    Ok(())
                } else {
                    Err(format!(
                        "Invalid value for {path}: every item must be a non-empty string"
                    ))
                }
            }
            None => Err(type_error("an array of strings")),
        },
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
    use crate::super_json::missing_required;

    #[test]
    fn test_super_json_required_path_count_is_eleven() {
        let _home = crate::test_support::TempHome::new("super-count");
        assert_eq!(valid_paths().len(), 25, "valid path count drifted");
        assert_eq!(
            required_paths().len(),
            11,
            "required path count drifted: {:?}",
            required_paths()
        );
        assert_eq!(
            missing_required("default").unwrap().len(),
            11,
            "a fresh project must report all 11 required paths as missing"
        );
        // Every required path must also be a valid path.
        for path in required_paths() {
            assert!(valid_paths().iter().any(|known| known == path), "{path}");
        }
    }

    /// Guards the hand-written type table against `super.json` edits.
    #[test]
    fn test_every_valid_path_has_a_type_rule() {
        for path in valid_paths() {
            assert!(
                field_kind(path).is_some(),
                "no type rule declared for {path} — super.json and field_kind() drifted"
            );
        }
        assert_eq!(valid_paths().len(), 25);
    }
}
