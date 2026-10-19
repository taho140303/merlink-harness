//! Provider-facing JSON schemas for the seven tools.

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
    let tables: Vec<Value> = super_json::schema_tables()
        .iter()
        .map(|table| json!(table))
        .collect();
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
                "description": "Replace one whole whitelisted value of the project's super json. \
                    Bulk writes only: for one new row use `super_json_add_row`, for one field of an \
                    existing row use `super_json_add_field`. Call this whenever the user states a \
                    value instead of merely replying in prose. `count` is an integer; `rows` is the \
                    complete array of objects whose keys come from that table's read-only `fields`. \
                    `count` must equal `rows.length`; for characters exactly one row carries \
                    `is_primary: true`. `fields` itself is never writable. Counts must be numbers, \
                    not strings. Unknown paths are rejected.",
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
                "name": "super_json_add_row",
                "description": "Append ONE draft row to a table, without reading or replacing the \
                    existing rows. The tool assigns `_json_index` (the row's 0-based position) and \
                    manages `count`; never write `count` after an add. A draft row may omit required \
                    fields, which `super_json_add_field` can fill later. When the target row has no \
                    id yet, point at it with `<field>_json_index` instead of inventing an id; never \
                    put a name, slug or code where an id is expected.",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "table": {
                            "type": "string",
                            "enum": tables,
                            "description": "One of the 34 schema tables."
                        },
                        "row": {
                            "type": "object",
                            "description": "The row to append. Keys must come from that table's \
                                read-only `fields`; required keys may be left out in a draft row."
                        }
                    },
                    "required": ["table", "row"]
                }
            }
        }),
        json!({
            "type": "function",
            "function": {
                "name": "super_json_add_field",
                "description": "Set exactly ONE field of one existing row. Read the table first and \
                    pass the row's `_json_index`. Rejects the synthetic `_json_index` itself and any \
                    key outside the table's read-only `fields`. Use this instead of replacing \
                    `rows` when the author adds a single detail.",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "table": {
                            "type": "string",
                            "enum": tables,
                            "description": "One of the 34 schema tables."
                        },
                        "json_index": {
                            "type": "integer",
                            "minimum": 0,
                            "description": "0-based position in that table's `rows`."
                        },
                        "field": {
                            "type": "string",
                            "description": "Column name from that table's read-only `fields`."
                        },
                        "value": {
                            "description": "New value. Type must match the field: string, \
                                integer >= 0, boolean, or one of the allowed enum values."
                        }
                    },
                    "required": ["table", "json_index", "field", "value"]
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

    #[test]
    fn test_tool_specs_cover_seven_tools() {
        let specs = tool_specs();
        assert_eq!(TOOL_NAMES.len(), 7);
        assert_eq!(specs.len(), 7);
        for name in ["super_json_add_row", "super_json_add_field"] {
            let spec = specs
                .iter()
                .find(|spec| spec["function"]["name"] == name)
                .unwrap_or_else(|| panic!("missing spec for {name}"));
            let tables = spec["function"]["parameters"]["properties"]["table"]["enum"]
                .as_array()
                .unwrap();
            assert_eq!(tables.len(), 34, "{name} must expose all 34 tables");
            assert!(tables.contains(&json!("characters")));
            assert!(tables.contains(&json!("competence_limitations")));
        }
        let add_row = specs
            .iter()
            .find(|spec| spec["function"]["name"] == "super_json_add_row")
            .unwrap();
        assert!(
            add_row["function"]["description"]
                .as_str()
                .unwrap()
                .contains("_json_index")
        );
        let set = specs
            .iter()
            .find(|spec| spec["function"]["name"] == "super_json_set")
            .unwrap();
        assert!(
            set["function"]["description"]
                .as_str()
                .unwrap()
                .contains("Bulk writes only")
        );
    }
}
