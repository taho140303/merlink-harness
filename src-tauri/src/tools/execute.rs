//! One entry point that dispatches every tool call.

use rusqlite::TransactionBehavior;
use serde_json::{Value, json};

use crate::{db, paths, super_json};

use super::TOOL_NAMES;
use super::project::{project_or_error, unique_project_id};

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

            let mut connection = db::open()?;
            let mut created_file = None;
            let mut created_directory = None;
            // SQLite and the filesystem are not one transaction. Roll back the
            // reservation/binding first (by dropping a failed transaction), then
            // unlink only the JSON file and empty directory this call created.
            // A crash or failed cleanup can still leave an orphan file/directory;
            // a crash before commit can leave the file without a project row.
            let creation = (|| -> Result<(String, Vec<String>), String> {
                // Acquire the SQLite writer before the session pre-check: a
                // deferred read transaction can otherwise fail on promotion to
                // writer when another session reserves the same slug concurrently.
                let transaction = connection
                    .transaction_with_behavior(TransactionBehavior::Immediate)
                    .map_err(|error| format!("Cannot start project transaction: {error}"))?;
                // Preserve the existing repeat-call error; the conditional bind
                // below is the authority if another path binds the session.
                if let Some(existing) = db::project_for_session(&transaction, session_id)? {
                    return Err(format!("This chat already belongs to project {existing}"));
                }

                let (base_name, display_title) = match &requested_name {
                    Some(name) => (name.as_str(), name.as_str()),
                    None => ("du-an", "Dự án chưa đặt tên"),
                };
                let project_id = unique_project_id(&transaction, base_name, display_title)?;
                let directory = paths::project_dir(&project_id)?;
                std::fs::create_dir_all(directory.parent().ok_or("Invalid project directory")?)
                    .map_err(|error| format!("Cannot create projects directory: {error}"))?;
                match std::fs::create_dir(&directory) {
                    Ok(()) => created_directory = Some(directory),
                    Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {}
                    Err(error) => return Err(format!("Cannot create project directory: {error}")),
                }
                super_json::init_project(&project_id, requested_name.as_deref())?;
                created_file = Some(paths::super_json_path(&project_id)?);
                if !db::bind_session_to_project_if_unbound(&transaction, session_id, &project_id)? {
                    return Err(
                        db::project_for_session(&transaction, session_id)?.map_or_else(
                            || "This chat already belongs to a project".to_owned(),
                            |existing| format!("This chat already belongs to project {existing}"),
                        ),
                    );
                }
                let missing = super_json::missing_required(&project_id)?;
                transaction
                    .commit()
                    .map_err(|error| format!("Cannot commit project transaction: {error}"))?;
                Ok((project_id, missing))
            })();
            let (project_id, missing) = match creation {
                Ok(result) => result,
                Err(mut error) => {
                    if let Some(path) = created_file
                        && let Err(cleanup_error) = std::fs::remove_file(&path)
                    {
                        error.push_str(&format!(
                            "; cannot remove newly created super json: {cleanup_error}"
                        ));
                    }
                    // Non-recursive: never remove a directory populated after
                    // we created it (or a pre-existing directory).
                    if let Some(directory) = created_directory
                        && let Err(cleanup_error) = std::fs::remove_dir(&directory)
                    {
                        error.push_str(&format!(
                            "; cannot remove newly created project directory: {cleanup_error}"
                        ));
                    }
                    return Err(error);
                }
            };

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
                Some(path) => super_json::get(project_id, path)
                    .map(|value| json!({ "path": path, "value": value })),
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
            let warnings = super_json::warnings(project_id)?;
            Ok(json!({
                "ok": true,
                "path": path,
                "value": applied,
                "missing_required": missing,
                "warnings": warnings,
            }))
        }
        "super_json_add_row" => {
            let project_id = project_or_error(project_id)?;
            let table = args
                .get("table")
                .and_then(Value::as_str)
                .ok_or("super_json_add_row requires a string 'table'")?;
            let row = args
                .get("row")
                .ok_or("super_json_add_row requires a 'row' object")?;
            let (json_index, count) = super_json::add_row(project_id, table, row.clone())?;
            let mut stored = row.clone();
            stored["_json_index"] = json!(json_index);
            let missing = super_json::missing_required(project_id)?;
            let warnings = super_json::warnings(project_id)?;
            Ok(json!({
                "ok": true,
                "table": table,
                "json_index": json_index,
                "count": count,
                "row": stored,
                "missing_required": missing,
                "warnings": warnings,
            }))
        }
        "super_json_add_field" => {
            let project_id = project_or_error(project_id)?;
            let table = args
                .get("table")
                .and_then(Value::as_str)
                .ok_or("super_json_add_field requires a string 'table'")?;
            let json_index = args
                .get("json_index")
                .and_then(Value::as_i64)
                .ok_or("super_json_add_field requires an integer 'json_index'")?;
            let field = args
                .get("field")
                .and_then(Value::as_str)
                .ok_or("super_json_add_field requires a string 'field'")?;
            let value = args
                .get("value")
                .ok_or("super_json_add_field requires a 'value'")?;
            super_json::add_field(project_id, table, json_index, field, value.clone())?;
            let missing = super_json::missing_required(project_id)?;
            let warnings = super_json::warnings(project_id)?;
            Ok(json!({
                "ok": true,
                "table": table,
                "json_index": json_index,
                "field": field,
                "value": value,
                "missing_required": missing,
                "warnings": warnings,
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
        "outline" => {
            let project_id = project_or_error(project_id)?;
            let action = match args.get("action").and_then(Value::as_str) {
                Some(act) => act,
                None => return Err("Missing required argument `action` for tool `outline`".into()),
            };
            match action {
                "status" => Ok(json!({
                    "ok": true,
                    "stage": "seed",
                    "active": false,
                    "note": "outline engine arrives in sprint 17.2",
                    "project_id": project_id,
                })),
                "anchor" => {
                    let doc = super_json::load_or_init(project_id)?;
                    let text = crate::outline::anchor(&doc);
                    let compact = crate::outline::anchor_compact(&doc);
                    let json = crate::outline::anchor_json(&doc);
                    Ok(json!({
                        "ok": true,
                        "project_id": project_id,
                        "text": text,
                        "compact": compact,
                        "json": json,
                    }))
                }
                other => Err(format!(
                    "Unknown outline action: {other}. Available actions: status, anchor"
                )),
            }
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
    fn test_outline_tool_is_registered() {
        let _home = crate::test_support::TempHome::new("tools-outline");
        assert!(TOOL_NAMES.contains(&"outline"));
        let status_res =
            execute_tool("outline", &json!({"action": "status"}), "proj1", "s").unwrap();
        assert_eq!(status_res["ok"], json!(true));
        assert_eq!(status_res["stage"], json!("seed"));
        assert_eq!(status_res["active"], json!(false));
        assert_eq!(status_res["project_id"], json!("proj1"));

        let anchor_res =
            execute_tool("outline", &json!({"action": "anchor"}), "proj1", "s").unwrap();
        assert_eq!(anchor_res["ok"], json!(true));
        assert_eq!(anchor_res["project_id"], json!("proj1"));
        assert!(anchor_res["text"].as_str().unwrap().contains("Dự án"));
        assert!(anchor_res["json"].is_object());
    }

    #[test]
    fn test_execute_tool_round_trip_and_unknown_tool() {
        let _home = crate::test_support::TempHome::new("tools");

        assert_eq!(
            execute_tool("echo", &json!({"message": "hi"}), "default", "s").unwrap(),
            json!({"message": "hi"})
        );

        let missing = execute_tool("super_json_missing", &json!({}), "default", "s").unwrap();
        assert_eq!(missing["required_total"], json!(12));
        assert_eq!(missing["missing"].as_array().unwrap().len(), 12);

        let applied = execute_tool(
            "super_json_set",
            &json!({"path": "require.project.name", "value": "Nửa Đêm"}),
            "default",
            "s",
        )
        .unwrap();
        assert_eq!(applied["ok"], json!(true));
        assert_eq!(applied["missing_required"].as_array().unwrap().len(), 11);

        let read = execute_tool(
            "super_json_get",
            &json!({"path": "require.project.name"}),
            "default",
            "s",
        )
        .unwrap();
        assert_eq!(read["value"], json!("Nửa Đêm"));

        let whole = execute_tool("super_json_get", &json!({}), "default", "s").unwrap();
        assert!(whole["require"]["project"].is_object());

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
            &json!({"path": "require.project.plan.arc.count", "value": "5"}),
            "default",
            "s",
        )
        .unwrap_err();
        assert!(error.contains("positive integer"), "{error}");

        assert!(
            execute_tool(
                "super_json_set",
                &json!({"path": "require.project.name"}),
                "default",
                "s"
            )
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

        let result = execute_tool(
            "project_create",
            &json!({"name": "Nửa Đêm"}),
            "",
            "sess-new",
        )
        .unwrap();

        // Every non-`[a-z0-9]` character becomes `-` (D3.3), diacritics included.
        assert_eq!(result["project_id"], json!("n-a-m"));
        assert_eq!(result["name"], json!("Nửa Đêm"));
        assert_eq!(result["ok"], json!(true));
        assert_eq!(
            result["missing_required"].as_array().unwrap().len(),
            11,
            "project_create fills require.project.name, so 11 remain"
        );

        // The session is bound to the new project.
        assert_eq!(
            db::project_for_session(&connection, "sess-new")
                .unwrap()
                .as_deref(),
            Some("n-a-m")
        );
        // The project row exists exactly once.
        assert_eq!(
            db::list_projects(&connection).unwrap(),
            vec!["n-a-m".to_string()]
        );

        // `projects/<id>/super.json` exists, was seeded from the skeleton and
        // already carries the name.
        let path = paths::super_json_path("n-a-m").unwrap();
        assert_eq!(
            path,
            home.path()
                .join("projects")
                .join("n-a-m")
                .join("super.json"),
            "the super json must live under the project directory"
        );
        assert!(path.exists(), "super.json missing at {}", path.display());
        let on_disk: Value = serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
        assert_eq!(on_disk["require"]["project"]["name"], json!("Nửa Đêm"));
        assert_eq!(on_disk["require"]["project"]["language"], Value::Null);
        assert_eq!(on_disk["optional"]["project"]["author"], Value::Null);
        assert!(
            on_disk.get("$comment").is_none(),
            "docs keys stay out of user data"
        );

        // Reading it back through the tools agrees with the file.
        let read = execute_tool(
            "super_json_get",
            &json!({"path": "require.project.name"}),
            "n-a-m",
            "sess-new",
        )
        .unwrap();
        assert_eq!(read["value"], json!("Nửa Đêm"));

        // The header now follows the binding: 1 of 12 required units is filled, so
        // the bar sits at round(70 * 1 / 12) = 6%.
        let after = crate::runtime::super_json_progress_for_session("sess-new").unwrap();
        assert!(after.has_project);
        assert_eq!(after.project_id, "n-a-m");
        assert_eq!(after.filled_required, 1);
        assert_eq!(after.required_total, 12);
        assert_eq!(after.filled_optional, 0);
        assert_eq!(after.optional_total, 35);
        assert_eq!(after.percent, 6);
        assert_eq!(after.label, "Đang thu thập thông tin");
    }

    /// D3 step 2: one project per chat, so a repeat call is refused.
    #[test]
    fn test_project_create_rejects_second_project_for_same_session() {
        let _home = crate::test_support::TempHome::new("tools-create-twice");
        let connection = db::open().unwrap();
        db::ensure_session(&connection, "sess-one", "").unwrap();

        let first = execute_tool(
            "project_create",
            &json!({"name": "Truyện A"}),
            "",
            "sess-one",
        )
        .unwrap();
        assert_eq!(first["project_id"], json!("truy-n-a"));

        let error = execute_tool(
            "project_create",
            &json!({"name": "Truyện B"}),
            "",
            "sess-one",
        )
        .unwrap_err();
        assert_eq!(error, "This chat already belongs to project truy-n-a");

        // Nothing was created by the rejected call.
        assert_eq!(db::list_projects(&connection).unwrap().len(), 1);
        assert_eq!(
            db::project_for_session(&connection, "sess-one")
                .unwrap()
                .as_deref(),
            Some("truy-n-a")
        );
        assert!(!paths::project_dir("truy-n-b").unwrap().exists());

        // A malformed argument is rejected before anything is written. A missing
        // or blank `name` is no longer malformed (D7): only a non-string is.
        for bad in [json!({"name": 7}), json!({"name": ["Nửa Đêm"]})] {
            let error = execute_tool("project_create", &bad, "", "sess-two").unwrap_err();
            assert!(
                error.contains("requires a non-empty string 'name'"),
                "{bad}: {error}"
            );
        }
        // Nothing was created by any of the rejected calls.
        assert_eq!(
            db::list_projects(&connection).unwrap(),
            vec!["truy-n-a".to_string()]
        );
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
        for id in [&first_id, &second_id] {
            let document: Value = serde_json::from_str(
                &fs::read_to_string(paths::super_json_path(id).unwrap()).unwrap(),
            )
            .unwrap();
            assert_eq!(document["require"]["project"]["name"], json!("Nửa Đêm"));
        }
        // Both ids pass `paths::safe_segment`. It is private, and `project_dir` is
        // its only caller and the function that actually builds the path, so it is
        // the real guard to assert on.
        for id in [&first_id, &second_id] {
            assert!(
                paths::project_dir(id).is_ok(),
                "{id} must pass the path guard"
            );
            assert!(
                paths::super_json_path(id).is_ok(),
                "{id} must pass the path guard"
            );
            assert!(
                id.chars()
                    .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '-'),
                "{id} must only contain [a-z0-9-]"
            );
        }

        // Vietnamese letters, quotes and punctuation all collapse into single `-`.
        let messy = execute_tool(
            "project_create",
            &json!({"name": "Truyện: \"Kẻ Mù\"?!"}),
            "",
            "sess-c",
        )
        .unwrap();
        let messy_id = messy["project_id"].as_str().unwrap();
        assert_eq!(messy_id, "truy-n-k-m");
        assert!(paths::project_dir(messy_id).is_ok());

        // A title with nothing slug-able falls back to "project".
        let empty = execute_tool("project_create", &json!({"name": "…???"}), "", "sess-d").unwrap();
        assert_eq!(empty["project_id"], json!("project"));
        assert!(paths::project_dir("project").is_ok());
    }

    #[test]
    fn test_project_create_json_failure_rolls_back_and_preserves_existing_directory() {
        let _home = crate::test_support::TempHome::new("tools-create-json-failure");
        let connection = db::open().unwrap();
        let directory = paths::project_dir("blocked").unwrap();
        fs::create_dir_all(&directory).unwrap();
        let file = directory.join("super.json");
        fs::write(&file, "original contents").unwrap();

        let error = execute_tool(
            "project_create",
            &json!({"name": "blocked"}),
            "",
            "new-session",
        )
        .unwrap_err();
        assert!(error.contains("already exists"), "{error}");
        assert!(db::list_projects(&connection).unwrap().is_empty());
        assert_eq!(
            db::project_for_session(&connection, "new-session").unwrap(),
            None
        );
        assert_eq!(fs::read_to_string(file).unwrap(), "original contents");
    }

    /// Acceptance: `super_json_*` is unusable until the chat has a project (D2).
    #[test]
    fn test_super_json_tools_fail_without_project() {
        let home = crate::test_support::TempHome::new("tools-no-project");

        for (name, args) in [
            (
                "super_json_set",
                json!({"path": "require.project.name", "value": "Nửa Đêm"}),
            ),
            ("super_json_get", json!({})),
            ("super_json_get", json!({"path": "require.project.name"})),
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
        assert!(
            !paths::project_dir("default")
                .unwrap()
                .join("super.json")
                .exists()
        );

        // `project_create` itself is exactly what unblocks them.
        let created = execute_tool(
            "project_create",
            &json!({"name": "Sau khi tạo"}),
            "",
            "sess-unbound",
        )
        .unwrap();
        let project_id = created["project_id"].as_str().unwrap().to_owned();
        assert_eq!(project_id, "sau-khi-t-o");
        let applied = execute_tool(
            "super_json_set",
            &json!({"path": "require.schema.project_genres.count", "value": 1}),
            &project_id,
            "sess-unbound",
        )
        .unwrap();
        assert_eq!(applied["ok"], json!(true));
        assert!(
            execute_tool(
                "super_json_missing",
                &json!({}),
                &project_id,
                "sess-unbound"
            )
            .is_ok(),
            "after project_create the super json tools work"
        );
    }

    /// Acceptance: `project_create` without a name creates an unnamed project,
    /// takes a `du-an` id and leaves `require.project.name` untouched (D7).
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
            12,
            "no title was written, so all 12 required units are still missing"
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
        let document: Value = serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
        assert_eq!(document["require"]["project"]["name"], Value::Null);
        assert!(
            crate::super_json::missing_required("du-an")
                .unwrap()
                .contains(&"require.project.name".to_string()),
            "an unnamed project must still report the title as missing"
        );

        // ... and the session is bound, so the super json tools work.
        assert_eq!(
            db::project_for_session(&connection, "sess-unnamed")
                .unwrap()
                .as_deref(),
            Some("du-an")
        );
        let applied = execute_tool(
            "super_json_set",
            &json!({"path": "require.schema.project_genres.count", "value": 1}),
            "du-an",
            "sess-unnamed",
        )
        .unwrap();
        assert_eq!(applied["ok"], json!(true));

        // A second unnamed project takes the `du-an-2` id, and a blank name is
        // treated exactly like a missing one.
        db::ensure_session(&connection, "sess-unnamed-2", "").unwrap();
        let second = execute_tool(
            "project_create",
            &json!({"name": "   "}),
            "",
            "sess-unnamed-2",
        )
        .unwrap();
        assert_eq!(second["project_id"], json!("du-an-2"));
        assert!(second.get("name").is_none(), "{second}");
        let second_path = paths::super_json_path("du-an-2").unwrap();
        let second_document: Value =
            serde_json::from_str(&fs::read_to_string(&second_path).unwrap()).unwrap();
        assert_eq!(second_document["require"]["project"]["name"], Value::Null);
    }

    #[test]
    fn test_execute_add_row_and_add_field_round_trip() {
        let _home = crate::test_support::TempHome::new("tools-add-row-field");
        let created = execute_tool(
            "project_create",
            &json!({"name": "Nửa Đêm"}),
            "",
            "sess-add-row",
        )
        .unwrap();
        let project_id = created["project_id"].as_str().unwrap();
        let before = created["missing_required"].as_array().unwrap().len();

        let added = execute_tool(
            "super_json_add_row",
            &json!({"table": "characters", "row": {"name": "Hoàng", "is_primary": true}}),
            project_id,
            "sess-add-row",
        )
        .unwrap();
        assert_eq!(added["ok"], json!(true));
        assert_eq!(added["table"], json!("characters"));
        assert_eq!(added["json_index"], json!(0));
        assert_eq!(added["count"], json!(1));
        assert_eq!(added["row"]["_json_index"], json!(0));
        assert_eq!(added["row"]["name"], json!("Hoàng"));
        assert_eq!(
            added["missing_required"].as_array().unwrap().len(),
            before - 1,
            "characters became complete, so one required unit disappeared"
        );
        assert!(added["warnings"].is_array());

        let field = execute_tool(
            "super_json_add_field",
            &json!({
                "table": "characters",
                "json_index": 0,
                "field": "birth",
                "value": "Mùa đông"
            }),
            project_id,
            "sess-add-row",
        )
        .unwrap();
        assert_eq!(field["ok"], json!(true));
        assert_eq!(field["table"], json!("characters"));
        assert_eq!(field["json_index"], json!(0));
        assert_eq!(field["field"], json!("birth"));
        assert_eq!(field["value"], json!("Mùa đông"));
        assert!(field["warnings"].is_array());

        let applied = execute_tool(
            "super_json_set",
            &json!({"path": "require.project.language", "value": "VI"}),
            project_id,
            "sess-add-row",
        )
        .unwrap();
        assert!(applied["warnings"].is_array(), "{applied}");
    }
}
