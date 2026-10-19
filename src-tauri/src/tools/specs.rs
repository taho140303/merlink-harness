//! Provider-facing JSON schemas for the five tools.

use serde_json::{Value, json};

use crate::super_json;

/// JSON-Schema definitions handed to the provider.
///
/// `path` is an `enum` of the exact whitelist rather than a free string: an
/// endpoint test showed the model inventing `project.name` when it had to guess.
pub fn tool_specs() -> Vec<Value> {
    let paths: Vec<Value> = super_json::valid_paths().iter().map(|path| json!(path)).collect();
    let required = super_json::required_paths();
    let required_list = required.join(", ");
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
                    named the work, because the project is created unnamed and require.overview.name \
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
                "description": "Read the project's super json. Omit 'path' to read the whole document.",
                "parameters": {
                    "type": "object",
                    "properties": { "path": path_property() },
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
                "description": "List the required super json paths that are still unset. \
                    Use it to decide what to ask the user next. An empty list means the outline \
                    step may start.",
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
        assert_eq!(allowed.len(), 25);
        assert!(allowed.contains(&json!("require.overview.name")));
        assert!(allowed.contains(&json!("optional.world.setting-scale")));
    }
}
