//! Pure snake_case <-> camelCase and domain-to-UI projection adapters (`REQ-IPC-001`, `Q-C9`).
//!
//! Provides bidirectional lossless transformation between:
//! - Canonical snake_case storage representations (SQLite, `draft_versions.data_json`, `BusinessFlowData`)
//! - CamelCase UI consumption models used across React components (`DraftWorkspace`, `ChapterDetailView`, `ProjectDetailView`, `CharacterListView`).

use serde_json::{Map, Value};

/// Convert a single string identifier from snake_case to camelCase.
pub fn snake_to_camel(s: &str) -> String {
    if let Some(rest) = s.strip_prefix('_') {
        let mut inner = snake_to_camel(rest);
        if let Some(first_char) = inner.chars().next() {
            let lower_first = first_char.to_lowercase().to_string();
            inner.replace_range(0..first_char.len_utf8(), &lower_first);
        }
        return format!("_{}", inner);
    }
    let mut result = String::with_capacity(s.len());
    let mut capitalize_next = false;
    for (i, c) in s.chars().enumerate() {
        if c == '_' {
            capitalize_next = true;
        } else if capitalize_next {
            result.extend(c.to_uppercase());
            capitalize_next = false;
        } else if i == 0 {
            result.extend(c.to_lowercase());
        } else {
            result.push(c);
        }
    }
    result
}

/// Convert a single string identifier from camelCase to snake_case.
pub fn camel_to_snake(s: &str) -> String {
    if let Some(rest) = s.strip_prefix('_') {
        return format!("_{}", camel_to_snake(rest));
    }
    let mut result = String::with_capacity(s.len() + 4);
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() {
            if i > 0 {
                result.push('_');
            }
            result.extend(c.to_lowercase());
        } else {
            result.push(c);
        }
    }
    result
}

/// Recursively transform all JSON object keys from snake_case to camelCase.
///
/// Preserves:
/// - Array ordering
/// - Primitive values and types (strings, numbers, booleans, null)
/// - Special keys (e.g. `_counters` prefix preserved)
pub fn to_camel_case_value(val: &Value) -> Value {
    match val {
        Value::Object(map) => {
            let mut new_map = Map::with_capacity(map.len());
            for (k, v) in map {
                let new_key = snake_to_camel(k);
                new_map.insert(new_key, to_camel_case_value(v));
            }
            Value::Object(new_map)
        }
        Value::Array(arr) => Value::Array(arr.iter().map(to_camel_case_value).collect()),
        other => other.clone(),
    }
}

/// Recursively transform all JSON object keys from camelCase to snake_case.
///
/// Preserves:
/// - Array ordering
/// - Primitive values and types
/// - Special keys (e.g. `_counters`)
pub fn to_snake_case_value(val: &Value) -> Value {
    match val {
        Value::Object(map) => {
            let mut new_map = Map::with_capacity(map.len());
            for (k, v) in map {
                let new_key = camel_to_snake(k);
                new_map.insert(new_key, to_snake_case_value(v));
            }
            Value::Object(new_map)
        }
        Value::Array(arr) => Value::Array(arr.iter().map(to_snake_case_value).collect()),
        other => other.clone(),
    }
}

/// Adapt raw `BusinessFlowData` JSON into camelCase `draftOverviewUi` representation.
pub fn adapt_business_flow_to_camel(val: &Value) -> Value {
    to_camel_case_value(val)
}

/// Adapt camelCase draft state back to canonical snake_case `BusinessFlowData` JSON.
pub fn adapt_business_flow_from_camel(val: &Value) -> Value {
    to_snake_case_value(val)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_snake_to_camel_and_back() {
        assert_eq!(snake_to_camel("draft_session_id"), "draftSessionId");
        assert_eq!(camel_to_snake("draftSessionId"), "draft_session_id");
        assert_eq!(snake_to_camel("arc_count"), "arcCount");
        assert_eq!(camel_to_snake("arcCount"), "arc_count");
        assert_eq!(snake_to_camel("_counters"), "_counters");
        assert_eq!(camel_to_snake("_counters"), "_counters");
    }

    #[test]
    fn test_json_roundtrip_adaptation() {
        let original = json!({
            "draft_session_id": "11111111-1111-1111-1111-111111111111",
            "phase": 1,
            "subphase": null,
            "require": {
                "project": {
                    "display_name": "Test Novel",
                    "target": {
                        "arc_count": 3,
                        "chapter_count": 10
                    }
                },
                "characters": [
                    { "code": "char-1", "display_name": "Alice" },
                    { "code": "char-2", "display_name": "Bob" }
                ]
            },
            "_counters": {
                "arc": 3
            }
        });

        let camel = to_camel_case_value(&original);
        assert!(camel.get("draftSessionId").is_some());
        assert!(camel["require"]["project"]["target"]
            .get("arcCount")
            .is_some());
        assert_eq!(camel["require"]["characters"][0]["displayName"], "Alice");

        let back_to_snake = to_snake_case_value(&camel);
        assert_eq!(original, back_to_snake);
    }
}
