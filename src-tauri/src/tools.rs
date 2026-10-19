use rusqlite::Connection;
use serde_json::{Value, json};

use crate::{db, paths, super_json};

/// Tools exposed to the model. `project_create` is listed first because the other
/// four are unusable until a chat has a project.
pub const TOOL_NAMES: [&str; 5] = [
    "project_create",
    "super_json_get",
    "super_json_set",
    "super_json_missing",
    "echo",
];

/// Prefix shared by every tool that needs a project (D2).
const NO_PROJECT: &str = "No project yet. Call project_create first.";

/// Vietnamese label for every writable super json path (D3). The list is
/// exhaustive over the 25 leaf paths; `test_status_message_covers_all_25_paths`
/// guards that it stays that way.
const PATH_LABELS: [(&str, &str); 25] = [
    ("require.overview.name", "tên truyện"),
    ("require.overview.logline.trigger-event", "sự kiện khởi động"),
    ("require.overview.logline.main-character", "nhân vật chính"),
    ("require.overview.logline.action", "hành động chính"),
    ("require.overview.logline.risk", "rủi ro"),
    ("require.overview.logline.major-loss", "mất mát lớn"),
    ("require.overview.plan.arc.count", "số arc"),
    ("require.overview.plan.chapter.count", "số chương"),
    ("require.overview.plan.word.count", "số từ"),
    ("require.genre.primary", "thể loại chính"),
    ("require.theme.question", "câu hỏi chủ đề"),
    ("optional.overview.description", "mô tả truyện"),
    ("optional.overview.author", "tác giả"),
    ("optional.overview.language", "ngôn ngữ"),
    ("optional.overview.target-audience", "đối tượng độc giả"),
    ("optional.overview.age-rating", "độ tuổi"),
    ("optional.overview.plan.arc.tolerance", "sai lệch số arc"),
    ("optional.overview.plan.chapter.tolerance", "sai lệch số chương"),
    ("optional.overview.plan.word.tolerance", "sai lệch số từ"),
    ("optional.genre.secondary", "thể loại phụ"),
    ("optional.theme.resolution-type", "kiểu giải quyết chủ đề"),
    ("optional.theme.resolution", "câu trả lời chủ đề"),
    ("optional.theme.message", "thông điệp"),
    ("optional.world.has-power-system", "có hệ thống sức mạnh"),
    ("optional.world.setting-scale", "quy mô thế giới"),
];

/// The label for one super json path, or `None` when it is outside the map.
fn path_label(path: &str) -> Option<&'static str> {
    PATH_LABELS
        .iter()
        .find(|(known, _)| *known == path)
        .map(|(_, label)| *label)
}

/// The single source of truth for field labels (Sprint 11.12 · D8).
///
/// `lib.rs` exposes this through the `super_json_labels` command so the frontend
/// can phrase the tool log in Vietnamese without keeping a second copy of the map.
pub fn field_labels() -> Vec<(&'static str, &'static str)> {
    PATH_LABELS.to_vec()
}

/// A ready-to-render Vietnamese phase message for one tool call (D3). It is the
/// `status` carried by both `ToolStarted` and `ToolFinished`, so the UI line never
/// changes wording while the tool runs.
pub fn status_message(name: &str, args: &Value) -> String {
    match name {
        // Sprint 11.12 · D7: an unnamed project must not claim to record a title
        // it never received — the live frame has to stay literally true.
        "project_create" => {
            let named = args
                .get("name")
                .and_then(Value::as_str)
                .map(str::trim)
                .is_some_and(|value| !value.is_empty());
            if named {
                "Đang ghi nhận tên truyện và tạo project".to_owned()
            } else {
                "Đang tạo project".to_owned()
            }
        }
        "super_json_missing" => "Đang kiểm tra các trường còn thiếu".to_owned(),
        "super_json_get" => "Đang đọc siêu JSON".to_owned(),
        "super_json_set" => {
            let path = args.get("path").and_then(Value::as_str).unwrap_or("");
            match path_label(path) {
                Some(label) => format!("Đang ghi nhận {label}"),
                // Unreachable for the 25 whitelisted paths; fall back to the path
                // itself rather than inventing a label.
                None => format!("Đang ghi nhận {path}"),
            }
        }
        "echo" => "Đang kiểm tra kết nối tool".to_owned(),
        other => format!("Đang chạy {other}"),
    }
}

/// The super json path a tool call addresses, when it has one. Used for the
/// `path` field of `ToolStarted` so the UI can label the phase precisely.
pub fn tool_path(name: &str, args: &Value) -> Option<String> {
    match name {
        "super_json_set" | "super_json_get" => args
            .get("path")
            .and_then(Value::as_str)
            .map(str::to_owned),
        _ => None,
    }
}

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

/// Guards the tools that cannot do anything without a project (D2).
fn project_or_error(project_id: &str) -> Result<&str, String> {
    let trimmed = project_id.trim();
    if trimmed.is_empty() {
        return Err(format!(
            "{NO_PROJECT} This chat is not bound to a project, so its super json cannot be \
             read or written."
        ));
    }
    Ok(trimmed)
}

/// D3.3: lowercase, every character outside `[a-z0-9]` becomes `-`, runs of `-`
/// collapse, edges are trimmed; an empty result becomes `"project"`. The result
/// only contains `[a-z0-9-]`, so it always survives `paths::safe_segment`.
fn slugify(name: &str) -> String {
    let mut slug = String::new();
    for character in name.trim().to_lowercase().chars() {
        if character.is_ascii_lowercase() || character.is_ascii_digit() {
            slug.push(character);
        } else if !slug.is_empty() && !slug.ends_with('-') {
            slug.push('-');
        }
    }
    let slug = slug.trim_end_matches('-').to_owned();
    if slug.is_empty() { "project".to_owned() } else { slug }
}

/// The slug, plus a `-2`, `-3`, … suffix while the id is taken.
fn unique_project_id(connection: &Connection, name: &str) -> Result<String, String> {
    let base = slugify(name);
    let existing = db::list_projects(connection)?;
    let mut candidate = base.clone();
    let mut suffix = 2;
    while existing.iter().any(|id| id == &candidate) {
        candidate = format!("{base}-{suffix}");
        suffix += 1;
    }
    // The id becomes a directory name, so prove it passes the path guard too.
    let _ = paths::project_dir(&candidate)?;
    Ok(candidate)
}

/// Executes one tool call.
///
/// `session_id` scopes `project_create`: the binding it writes is what every
/// later turn reads the effective project from (D1).
pub fn execute_tool(
    name: &str,
    args: &Value,
    project_id: &str,
    session_id: &str,
) -> Result<Value, String> {
    match name {
        "echo" => Ok(args.clone()),
        "project_create" => {
            // Sprint 11.12 · D7: the title is optional. A missing or blank name
            // creates an unnamed project instead of failing — and, crucially,
            // instead of inventing a title the author never gave.
            let requested_name = match args.get("name") {
                None | Some(Value::Null) => None,
                Some(Value::String(text)) => {
                    let trimmed = text.trim();
                    (!trimmed.is_empty()).then(|| trimmed.to_owned())
                }
                // A non-string is still a malformed call, not "no name".
                Some(_) => {
                    return Err(
                        "project_create requires a non-empty string 'name' when it is provided"
                            .to_owned(),
                    );
                }
            };

            let connection = db::open()?;
            // D3.2: a chat owns at most one project, so a repeat call is refused
            // instead of silently creating a second one.
            if let Some(existing) = db::project_for_session(&connection, session_id)? {
                return Err(format!("This chat already belongs to project {existing}"));
            }

            // An unnamed project takes a `du-an` id and keeps the documented
            // display title, but does not touch `require.overview.name`.
            let (project_id, display_title) = match &requested_name {
                Some(name) => (unique_project_id(&connection, name)?, name.clone()),
                None => (
                    unique_project_id(&connection, "du-an")?,
                    "Dự án chưa đặt tên".to_owned(),
                ),
            };
            db::create_project(&connection, &project_id, &display_title)?;
            super_json::load_or_init(&project_id)?;
            if let Some(name) = &requested_name {
                // D3.5: the author never has to be asked for the title twice.
                super_json::set(
                    &project_id,
                    "require.overview.name",
                    Value::String(name.clone()),
                )?;
            }
            db::bind_session_to_project(&connection, session_id, &project_id)?;

            let missing = super_json::missing_required(&project_id)?;
            let mut result = json!({
                "ok": true,
                "project_id": project_id,
                "missing_required": missing,
            });
            // D7: `name` appears only when the author actually named the work.
            if let Some(name) = requested_name {
                result["name"] = json!(name);
            }
            Ok(result)
        }
        "super_json_get" => {
            let project_id = project_or_error(project_id)?;
            match args.get("path").and_then(Value::as_str) {
                Some(path) => super_json::get(project_id, path).map(|value| {
                    json!({ "path": path, "value": value })
                }),
                None => super_json::load_or_init(project_id),
            }
        }
        "super_json_set" => {
            let project_id = project_or_error(project_id)?;
            let path = args
                .get("path")
                .and_then(Value::as_str)
                .ok_or("super_json_set requires a string 'path'")?;
            let value = args
                .get("value")
                .ok_or("super_json_set requires a 'value'")?;
            let applied = super_json::set(project_id, path, value.clone())?;
            let missing = super_json::missing_required(project_id)?;
            Ok(json!({
                "ok": true,
                "path": path,
                "value": applied,
                "missing_required": missing,
            }))
        }
        "super_json_missing" => {
            let project_id = project_or_error(project_id)?;
            let missing = super_json::missing_required(project_id)?;
            Ok(json!({
                "missing": missing,
                "required_total": super_json::required_paths().len(),
            }))
        }
        other => Err(format!(
            "Unknown tool: {other}. Available tools: {}",
            TOOL_NAMES.join(", ")
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{db, paths};
    use serde_json::Value;
    use std::fs;

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

    /// D3: every one of the 25 whitelisted paths must yield a non-empty phase
    /// message, and the labelled ones must read like real Vietnamese.
    #[test]
    fn test_status_message_covers_all_25_paths() {
        let _home = crate::test_support::TempHome::new("tools-status-paths");
        let paths = crate::super_json::valid_paths();
        assert_eq!(paths.len(), 25, "the whitelist must stay at 25 paths");
        assert_eq!(PATH_LABELS.len(), 25, "the label map must cover the whitelist");

        // Every label in the map is non-empty and belongs to a real path.
        for (path, label) in PATH_LABELS {
            assert!(!label.trim().is_empty(), "{path} has an empty label");
            assert!(
                paths.iter().any(|known| known == path),
                "label map mentions a non-whitelisted path: {path}"
            );
        }

        for path in paths {
            let message =
                status_message("super_json_set", &json!({"path": path, "value": "x"}));
            assert!(!message.trim().is_empty(), "{path} produced no message");
            assert!(
                message.starts_with("Đang ghi nhận "),
                "{path} produced an unexpected message: {message}"
            );
            // A whitelisted path must never fall back to itself.
            assert_ne!(
                message,
                format!("Đang ghi nhận {path}"),
                "{path} is missing from PATH_LABELS"
            );
        }

        // At least these five labels must be the expected, meaningful Vietnamese.
        for (path, expected) in [
            ("require.overview.name", "tên truyện"),
            ("require.overview.logline.trigger-event", "sự kiện khởi động"),
            ("require.genre.primary", "thể loại chính"),
            ("require.theme.question", "câu hỏi chủ đề"),
            ("require.overview.plan.arc.count", "số arc"),
            ("optional.world.has-power-system", "có hệ thống sức mạnh"),
        ] {
            let message = status_message("super_json_set", &json!({"path": path, "value": "x"}));
            assert!(
                message.contains(expected),
                "{path} message {message:?} must contain {expected:?}"
            );
        }

        // A path outside the map would fall back to itself (cannot happen with the
        // 25 valid paths, but the fallback must still be non-empty).
        let fallback = status_message("super_json_set", &json!({"path": "require.bogus"}));
        assert_eq!(fallback, "Đang ghi nhận require.bogus");
    }

    /// D3: the non-`super_json_set` tools have fixed messages, and an unknown
    /// tool names itself.
    #[test]
    fn test_status_message_for_meta_tools() {
        let _home = crate::test_support::TempHome::new("tools-status-meta");
        assert_eq!(
            status_message("project_create", &json!({"name": "Nửa Đêm"})),
            "Đang ghi nhận tên truyện và tạo project"
        );
        // Sprint 11.12 · D7: unnamed creation says only what it does.
        assert_eq!(
            status_message("project_create", &json!({})),
            "Đang tạo project"
        );
        assert_eq!(
            status_message("project_create", &json!({"name": "   "})),
            "Đang tạo project"
        );
        assert_eq!(
            status_message("super_json_missing", &json!({})),
            "Đang kiểm tra các trường còn thiếu"
        );
        assert_eq!(
            status_message("super_json_get", &json!({"path": "require.overview.name"})),
            "Đang đọc siêu JSON"
        );
        assert_eq!(
            status_message("echo", &json!({"message": "hi"})),
            "Đang kiểm tra kết nối tool"
        );
        assert_eq!(
            status_message("some_future_tool", &json!({})),
            "Đang chạy some_future_tool"
        );

        // `tool_path` only reports a path for the tools that address one.
        assert_eq!(
            tool_path("super_json_set", &json!({"path": "require.overview.name"})).as_deref(),
            Some("require.overview.name")
        );
        assert_eq!(
            tool_path("super_json_get", &json!({"path": "require.genre.primary"})).as_deref(),
            Some("require.genre.primary")
        );
        assert!(tool_path("project_create", &json!({"name": "x"})).is_none());
        assert!(tool_path("super_json_missing", &json!({})).is_none());
    }

    #[test]
    fn test_execute_tool_round_trip_and_unknown_tool() {
        let _home = crate::test_support::TempHome::new("tools");

        assert_eq!(
            execute_tool("echo", &json!({"message": "hi"}), "default", "s").unwrap(),
            json!({"message": "hi"})
        );

        let missing = execute_tool("super_json_missing", &json!({}), "default", "s").unwrap();
        assert_eq!(missing["required_total"], json!(11));
        assert_eq!(missing["missing"].as_array().unwrap().len(), 11);

        let applied = execute_tool(
            "super_json_set",
            &json!({"path": "require.overview.name", "value": "Nửa Đêm"}),
            "default",
            "s",
        )
        .unwrap();
        assert_eq!(applied["ok"], json!(true));
        assert_eq!(applied["missing_required"].as_array().unwrap().len(), 10);

        let read = execute_tool(
            "super_json_get",
            &json!({"path": "require.overview.name"}),
            "default",
            "s",
        )
        .unwrap();
        assert_eq!(read["value"], json!("Nửa Đêm"));

        let whole = execute_tool("super_json_get", &json!({}), "default", "s").unwrap();
        assert!(whole["require"]["overview"]["logline"].is_object());

        assert!(
            execute_tool("nope", &json!({}), "default", "s")
                .unwrap_err()
                .contains("Unknown tool: nope")
        );
    }

    #[test]
    fn test_execute_tool_surfaces_validation_errors() {
        let _home = crate::test_support::TempHome::new("tools-errors");

        let error = execute_tool(
            "super_json_set",
            &json!({"path": "require.overview.plan.arc.count", "value": "5"}),
            "default",
            "s",
        )
        .unwrap_err();
        assert!(error.contains("positive integer"), "{error}");

        assert!(
            execute_tool("super_json_set", &json!({"path": "require.overview.name"}), "default", "s")
                .unwrap_err()
                .contains("requires a 'value'")
        );
        assert!(
            execute_tool("super_json_set", &json!({"value": 1}), "default", "s")
                .unwrap_err()
                .contains("requires a string 'path'")
        );
    }

    /// Acceptance: the tool creates the project, seeds `super.json`, writes the
    /// name and binds the session (D3 steps 3–6).
    #[test]
    fn test_project_create_tool_binds_session_and_seeds_super_json() {
        let home = crate::test_support::TempHome::new("tools-create");
        let connection = db::open().unwrap();
        db::ensure_session(&connection, "sess-new", "").unwrap();
        assert_eq!(
            db::project_for_session(&connection, "sess-new").unwrap(),
            None,
            "a fresh session has no project"
        );

        // Before the project exists the header reports "no project" (D7).
        let before = crate::runtime::super_json_progress_for_session("sess-new").unwrap();
        assert!(!before.has_project);
        assert_eq!(before.label, "Chưa có project");
        assert_eq!(before.percent, 0);

        let result =
            execute_tool("project_create", &json!({"name": "Nửa Đêm"}), "", "sess-new").unwrap();

        // Every non-`[a-z0-9]` character becomes `-` (D3.3), diacritics included.
        assert_eq!(result["project_id"], json!("n-a-m"));
        assert_eq!(result["name"], json!("Nửa Đêm"));
        assert_eq!(result["ok"], json!(true));
        assert_eq!(
            result["missing_required"].as_array().unwrap().len(),
            10,
            "project_create fills require.overview.name, so 10 remain"
        );

        // The session is bound to the new project.
        assert_eq!(
            db::project_for_session(&connection, "sess-new").unwrap().as_deref(),
            Some("n-a-m")
        );
        // The project row exists exactly once.
        assert_eq!(db::list_projects(&connection).unwrap(), vec!["n-a-m".to_string()]);

        // `projects/<id>/super.json` exists, was seeded from the skeleton and
        // already carries the name.
        let path = paths::super_json_path("n-a-m").unwrap();
        assert_eq!(
            path,
            home.path().join("projects").join("n-a-m").join("super.json"),
            "the super json must live under the project directory"
        );
        assert!(path.exists(), "super.json missing at {}", path.display());
        let on_disk: Value =
            serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
        assert_eq!(on_disk["require"]["overview"]["name"], json!("Nửa Đêm"));
        assert_eq!(on_disk["require"]["overview"]["logline"]["action"], Value::Null);
        assert_eq!(on_disk["optional"]["genre"]["secondary"], json!([]));
        assert!(on_disk.get("$comment").is_none(), "docs keys stay out of user data");

        // Reading it back through the tools agrees with the file.
        let read = execute_tool(
            "super_json_get",
            &json!({"path": "require.overview.name"}),
            "n-a-m",
            "sess-new",
        )
        .unwrap();
        assert_eq!(read["value"], json!("Nửa Đêm"));

        // The header now follows the binding: 1 of 11 required paths is filled, so
        // the bar sits at round(70 * 1 / 11) = 6%.
        let after = crate::runtime::super_json_progress_for_session("sess-new").unwrap();
        assert!(after.has_project);
        assert_eq!(after.project_id, "n-a-m");
        assert_eq!(after.filled_required, 1);
        assert_eq!(after.required_total, 11);
        assert_eq!(after.filled_optional, 0);
        assert_eq!(after.optional_total, 14);
        assert_eq!(after.percent, 6);
        assert_eq!(after.label, "Đang thu thập thông tin");
    }

    /// D3 step 2: one project per chat, so a repeat call is refused.
    #[test]
    fn test_project_create_rejects_second_project_for_same_session() {
        let _home = crate::test_support::TempHome::new("tools-create-twice");
        let connection = db::open().unwrap();
        db::ensure_session(&connection, "sess-one", "").unwrap();

        let first =
            execute_tool("project_create", &json!({"name": "Truyện A"}), "", "sess-one").unwrap();
        assert_eq!(first["project_id"], json!("truy-n-a"));

        let error = execute_tool("project_create", &json!({"name": "Truyện B"}), "", "sess-one")
            .unwrap_err();
        assert_eq!(error, "This chat already belongs to project truy-n-a");

        // Nothing was created by the rejected call.
        assert_eq!(db::list_projects(&connection).unwrap().len(), 1);
        assert_eq!(
            db::project_for_session(&connection, "sess-one").unwrap().as_deref(),
            Some("truy-n-a")
        );
        assert!(!paths::project_dir("truy-n-b").unwrap().exists());

        // A malformed argument is rejected before anything is written. A missing
        // or blank `name` is no longer malformed (D7): only a non-string is.
        for bad in [json!({"name": 7}), json!({"name": ["Nửa Đêm"]})] {
            let error = execute_tool("project_create", &bad, "", "sess-two").unwrap_err();
            assert!(error.contains("requires a non-empty string 'name'"), "{bad}: {error}");
        }
        // Nothing was created by any of the rejected calls.
        assert_eq!(db::list_projects(&connection).unwrap(), vec!["truy-n-a".to_string()]);
    }

    /// D3 step 3: the slug is unique, and it always survives the path guard.
    #[test]
    fn test_project_create_slug_is_unique_and_path_safe() {
        let _home = crate::test_support::TempHome::new("tools-slug");
        let connection = db::open().unwrap();
        for session in ["sess-a", "sess-b", "sess-c", "sess-d"] {
            db::ensure_session(&connection, session, "").unwrap();
        }

        // Same title in two chats: the second id needs a suffix.
        let first =
            execute_tool("project_create", &json!({"name": "Nửa Đêm"}), "", "sess-a").unwrap();
        let second =
            execute_tool("project_create", &json!({"name": "Nửa Đêm"}), "", "sess-b").unwrap();
        let first_id = first["project_id"].as_str().unwrap().to_owned();
        let second_id = second["project_id"].as_str().unwrap().to_owned();

        assert_eq!(first_id, "n-a-m", "diacritics are not [a-z0-9] (D3.3)");
        assert_ne!(first_id, second_id);
        assert_eq!(second_id, "n-a-m-2", "the second project takes a -2 suffix");
        // Both ids pass `paths::safe_segment`. It is private, and `project_dir` is
        // its only caller and the function that actually builds the path, so it is
        // the real guard to assert on.
        for id in [&first_id, &second_id] {
            assert!(paths::project_dir(id).is_ok(), "{id} must pass the path guard");
            assert!(paths::super_json_path(id).is_ok(), "{id} must pass the path guard");
            assert!(
                id.chars().all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '-'),
                "{id} must only contain [a-z0-9-]"
            );
        }

        // Vietnamese letters, quotes and punctuation all collapse into single `-`.
        let messy =
            execute_tool("project_create", &json!({"name": "Truyện: \"Kẻ Mù\"?!"}), "", "sess-c")
                .unwrap();
        let messy_id = messy["project_id"].as_str().unwrap();
        assert_eq!(messy_id, "truy-n-k-m");
        assert!(paths::project_dir(messy_id).is_ok());

        // A title with nothing slug-able falls back to "project".
        let empty = execute_tool("project_create", &json!({"name": "…???"}), "", "sess-d").unwrap();
        assert_eq!(empty["project_id"], json!("project"));
        assert!(paths::project_dir("project").is_ok());
    }

    /// Acceptance: `super_json_*` is unusable until the chat has a project (D2).
    #[test]
    fn test_super_json_tools_fail_without_project() {        let home = crate::test_support::TempHome::new("tools-no-project");

        for (name, args) in [
            (
                "super_json_set",
                json!({"path": "require.overview.name", "value": "Nửa Đêm"}),
            ),
            ("super_json_get", json!({})),
            ("super_json_get", json!({"path": "require.overview.name"})),
            ("super_json_missing", json!({})),
        ] {
            let error = execute_tool(name, &args, "", "sess-unbound").unwrap_err();
            assert!(
                error.starts_with("No project yet. Call project_create first."),
                "{name} must point at project_create: {error}"
            );
        }

        // Nothing was created on disk for an unbound chat — no super json, and no
        // project directory either.
        assert!(!home.path().join("projects").exists());
        assert!(!paths::project_dir("default").unwrap().join("super.json").exists());

        // `project_create` itself is exactly what unblocks them.
        let created =
            execute_tool("project_create", &json!({"name": "Sau khi tạo"}), "", "sess-unbound")
                .unwrap();
        let project_id = created["project_id"].as_str().unwrap().to_owned();
        assert_eq!(project_id, "sau-khi-t-o");
        let applied = execute_tool(
            "super_json_set",
            &json!({"path": "require.genre.primary", "value": "trinh thám"}),
            &project_id,
            "sess-unbound",
        )
        .unwrap();
        assert_eq!(applied["ok"], json!(true));
        assert!(
            execute_tool("super_json_missing", &json!({}), &project_id, "sess-unbound").is_ok(),
            "after project_create the super json tools work"
        );
    }

    /// Sprint 11.12 · D8: the command source covers every one of the 25 paths.
    #[test]
    fn test_labels_command_covers_all_25_paths() {
        let _home = crate::test_support::TempHome::new("tools-labels");
        let labels = field_labels();
        assert_eq!(labels.len(), 25, "the label map must cover all 25 paths");
        assert_eq!(
            labels.len(),
            crate::super_json::valid_paths().len(),
            "the label map and the whitelist drifted"
        );

        // Every path is present exactly once, and every label is real Vietnamese.
        for path in crate::super_json::valid_paths() {
            let matches: Vec<&str> = labels
                .iter()
                .filter(|(known, _)| known == path)
                .map(|(_, label)| *label)
                .collect();
            assert_eq!(matches.len(), 1, "{path} must have exactly one label");
            assert!(!matches[0].trim().is_empty(), "{path} has an empty label");
        }
        let by_path: std::collections::BTreeMap<&str, &str> = labels.iter().copied().collect();
        assert_eq!(by_path["require.overview.name"], "tên truyện");
        assert_eq!(by_path["require.overview.plan.word.count"], "số từ");
        assert_eq!(by_path["optional.world.setting-scale"], "quy mô thế giới");
    }

    /// Acceptance: `project_create` without a name creates an unnamed project,
    /// takes a `du-an` id and leaves `require.overview.name` untouched (D7).
    #[test]
    fn test_project_create_without_name_leaves_title_unset() {
        let _home = crate::test_support::TempHome::new("tools-unnamed");
        let connection = db::open().unwrap();
        db::ensure_session(&connection, "sess-unnamed", "").unwrap();

        let created = execute_tool("project_create", &json!({}), "", "sess-unnamed").unwrap();
        assert_eq!(created["ok"], json!(true));
        assert_eq!(created["project_id"], json!("du-an"));
        assert!(
            created.get("name").is_none(),
            "an unnamed project must not report a name: {created}"
        );
        assert_eq!(
            created["missing_required"].as_array().unwrap().len(),
            11,
            "no title was written, so all 11 required fields are still missing"
        );

        // The row exists under the documented display title ...
        let titles: Vec<String> = connection
            .prepare("SELECT title FROM projects WHERE id = 'du-an'")
            .unwrap()
            .query_map([], |row| row.get(0))
            .unwrap()
            .map(|row| row.unwrap())
            .collect();
        assert_eq!(titles, vec!["Dự án chưa đặt tên".to_string()]);

        // ... the super json exists and its name stayed null ...
        let path = paths::super_json_path("du-an").unwrap();
        let document: Value =
            serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
        assert_eq!(document["require"]["overview"]["name"], Value::Null);
        assert!(
            crate::super_json::missing_required("du-an")
                .unwrap()
                .contains(&"require.overview.name".to_string()),
            "an unnamed project must still report the title as missing"
        );

        // ... and the session is bound, so the super json tools work.
        assert_eq!(
            db::project_for_session(&connection, "sess-unnamed").unwrap().as_deref(),
            Some("du-an")
        );
        let applied = execute_tool(
            "super_json_set",
            &json!({"path": "require.genre.primary", "value": "trinh thám"}),
            "du-an",
            "sess-unnamed",
        )
        .unwrap();
        assert_eq!(applied["ok"], json!(true));

        // A second unnamed project takes the `du-an-2` id, and a blank name is
        // treated exactly like a missing one.
        db::ensure_session(&connection, "sess-unnamed-2", "").unwrap();
        let second = execute_tool("project_create", &json!({"name": "   "}), "", "sess-unnamed-2")
            .unwrap();
        assert_eq!(second["project_id"], json!("du-an-2"));
        assert!(second.get("name").is_none(), "{second}");
        let second_path = paths::super_json_path("du-an-2").unwrap();
        let second_document: Value =
            serde_json::from_str(&fs::read_to_string(&second_path).unwrap()).unwrap();
        assert_eq!(second_document["require"]["overview"]["name"], Value::Null);
    }
}
