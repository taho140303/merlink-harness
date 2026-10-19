//! Provider-facing JSON schemas for the five tools.

use serde_json::{Value, json};

use crate::super_json;

/// JSON-Schema definitions handed to the provider.
///
/// `path` is an `enum` of the exact whitelist rather than a free string: an
/// endpoint test showed the model inventing `project.name` when it had to guess.
pub fn tool_specs() -> Vec<Value> {
    let paths: Vec<Value> = super_json::valid_paths()
        .iter()
        .map(|path| json!(path))
        .collect();
    let mut read_paths = paths.clone();
    for table in super_json::schema_tables() {
        if let Some(group) = super_json::schema_table_group(table) {
            read_paths.push(json!(format!("{}.schema.{table}", group.as_str())));
        }
    }
    let required = super_json::required_paths();
    let required_list = required.join(", ");
    let read_path_property = || {
        json!({
            "type": "string",
            "enum": read_paths,
            "description": "Readable path: a project leaf, a `.count`/`.rows` path, or a table \
                selector like `require.schema.characters`, which returns the merged field \
                contract plus count and rows."
        })
    };
    let path_property = || {
        json!({
            "type": "string",
            "enum": paths,
            "description": format!(
                "Dot path of the field to address. These {} paths are required and block the \
                 outline step until set: {}",
                required.len(),
                required_list
            )
        })
    };

    vec![
        json!({
            "type": "function",
            "function": {
                "name": "project_create",
                "description": "Create this chat's project. Call it exactly ONCE, as soon as the \
                    author has said what they want to write, before answering in prose. The optional \
                    'name' is the title the author actually stated: omit it when the author has not \
                    named the work, because the project is created unnamed and require.project.name \
                    stays empty until the author gives a title. Never invent a title. Every \
                    super_json_* tool fails until the project exists.",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "name": {
                            "type": "string",
                            "description": "Optional title of the work, exactly as the author stated \
                                it. Omit it (or send \"\") when the author has not named the work; \
                                the project is created with a 'du-an' id and no title is written."
                        }
                    },
                    "required": []
                }
            }
        }),
        json!({
            "type": "function",
            "function": {
                "name": "super_json_get",
                "description": "Read the project's super json. Omit 'path' to read the whole \
                    document. A table selector such as `require.schema.characters` returns that \
                    table's read-only `fields` contract plus its `count` and `rows`.",
                "parameters": {
                    "type": "object",
                    "properties": { "path": read_path_property() },
                    "required": []
                }
            }
        }),
        json!({
            "type": "function",
            "function": {
                "name": "super_json_set",
                "description": "Set exactly one whitelisted field of the project's super json. \
                    Call this whenever the user states a value instead of merely replying in prose. \
                    `count` is an integer; `rows` is an array of objects whose keys come from that \
                    table's read-only `fields`. `count` must equal `rows.length`; for characters \
                    exactly one row carries `is_primary: true`. `fields` itself is never writable. \
                    Counts must be numbers, not strings. Unknown paths are rejected.",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "path": path_property(),
                        "value": {
                            "description": "New value. Type must match the field: string, positive \
                                integer, integer >= 0, boolean, array of strings, or one of the \
                                allowed enum values."
                        }
                    },
                    "required": ["path", "value"]
                }
            }
        }),
        json!({
            "type": "function",
            "function": {
                "name": "super_json_missing",
                "description": "List the required super json units that are still unset: a project \
                    leaf or `require.schema.<table>`. Use it to decide what to ask the user next. \
                    An empty list means the outline step may start.",
                "parameters": {
                    "type": "object",
                    "properties": {},
                    "required": []
                }
            }
        }),
        json!({
            "type": "function",
            "function": {
                "name": "echo",
                "description": "Returns its arguments unchanged. Diagnostics only.",
                "parameters": {
                    "type": "object",
                    "properties": { "message": { "type": "string" } },
                    "required": []
                }
            }
        }),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tools::TOOL_NAMES;
    use serde_json::json;

    #[test]
    fn test_tool_specs_cover_all_tools_and_expose_full_whitelist() {
        let specs = tool_specs();
        assert_eq!(specs.len(), TOOL_NAMES.len());
        let names: Vec<&str> = specs
            .iter()
            .map(|spec| spec["function"]["name"].as_str().unwrap())
            .collect();
        for name in TOOL_NAMES {
            assert!(names.contains(&name), "missing spec for {name}");
        }

        let set_spec = specs
            .iter()
            .find(|spec| spec["function"]["name"] == "super_json_set")
            .unwrap();
        let allowed = set_spec["function"]["parameters"]["properties"]["path"]["enum"]
            .as_array()
            .unwrap();
        assert_eq!(allowed.len(), 81);
        assert!(allowed.contains(&json!("require.project.name")));
        assert!(allowed.contains(&json!("require.schema.characters.rows")));
        assert!(allowed.contains(&json!("optional.schema.appearance_options.count")));

        let get_spec = specs
            .iter()
            .find(|spec| spec["function"]["name"] == "super_json_get")
            .unwrap();
        let readable = get_spec["function"]["parameters"]["properties"]["path"]["enum"]
            .as_array()
            .unwrap();
        assert_eq!(
            readable.len(),
            81 + 34,
            "get also accepts 34 table selectors"
        );
        assert!(readable.contains(&json!("require.schema.characters")));
        assert!(readable.contains(&json!("optional.schema.lore_entries")));
    }
}
