//! System prompt assembly.

const SYSTEM_TEMPLATE: &str = include_str!("../../prompts/harness_system.md");

/// Phase 2 rules. Appended only once every required unit is filled, because
/// before that the chat is still collecting and must not switch to outlining.
const OUTLINE_PLAYBOOK: &str = include_str!("../../prompts/outline_playbook.md");

/// What `{project_state}` says while the chat has no project yet. Worded as an
/// instruction because that is how the model reads it.
const NO_PROJECT_STATE: &str =
    "(none yet - call project_create once the author says what they want to write)";

/// What `{super_json}` says while the chat has no project yet.
const NO_SUPER_JSON: &str = "(chưa có project)";

/// The whole project document, pretty-printed, for the prompt (D3).
///
/// It is read straight from disk and only when the file exists: a prompt must
/// never create a ghost project. An unparsable file is embedded raw so the model
/// sees what the author actually has instead of an empty prompt.
fn embedded_document(project_id: &str) -> String {
    if project_id.trim().is_empty() {
        return NO_SUPER_JSON.to_owned();
    }
    let Ok(path) = crate::paths::super_json_path(project_id) else {
        return NO_SUPER_JSON.to_owned();
    };
    let Ok(raw) = std::fs::read_to_string(&path) else {
        return NO_SUPER_JSON.to_owned();
    };
    match serde_json::from_str::<serde_json::Value>(&raw) {
        Ok(document) => serde_json::to_string_pretty(&document).unwrap_or(raw),
        Err(_) => raw,
    }
}

pub fn assemble_prompt(
    provider_name: &str,
    model_name: &str,
    session_id: &str,
    project_id: &str,
) -> String {
    // An empty project id means "this chat has no project yet" (D1); the prompt
    // says so explicitly so the model knows to call `project_create`.
    let project_state = if project_id.trim().is_empty() {
        NO_PROJECT_STATE
    } else {
        project_id
    };
    let required_total = crate::super_json::required_paths().len().to_string();
    let optional_total = crate::super_json::optional_paths().len().to_string();
    // D3: `{super_json}` is substituted last, so a JSON string containing another
    // placeholder can never be replaced by a later `.replace` pass.
    let super_json = embedded_document(project_id);
    let prompt = SYSTEM_TEMPLATE
        .replace("{provider_name}", provider_name)
        .replace("{model_name}", model_name)
        .replace("{session_id}", session_id)
        .replace("{project_state}", project_state)
        .replace("{required_total}", &required_total)
        .replace("{optional_total}", &optional_total)
        .replace("{super_json}", &super_json);
    if phase_two_ready(project_id) {
        return format!("{prompt}\n\n{OUTLINE_PLAYBOOK}");
    }
    prompt
}

/// `true` when the project exists on disk and every required unit is filled, the
/// same gate the outline button uses. A missing file is never created here.
fn phase_two_ready(project_id: &str) -> bool {
    if project_id.trim().is_empty() {
        return false;
    }
    let Ok(path) = crate::paths::super_json_path(project_id) else {
        return false;
    };
    if !path.exists() {
        return false;
    }
    crate::super_json::missing_required(project_id)
        .map(|missing| missing.is_empty())
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_prompt_embeds_the_live_super_json() {
        let _home = crate::test_support::TempHome::new("prompt-live-json");
        crate::super_json::init_project("p", None).unwrap();
        crate::super_json::set("p", "require.project.name", json!("Nửa Đêm")).unwrap();

        let prompt = assemble_prompt("OpenAI", "gpt-4o", "s", "p");
        assert!(prompt.contains("\"Nửa Đêm\""), "the title is missing");
        assert!(prompt.contains("\"require\""), "the document is missing");
        assert!(
            !prompt.contains("{super_json}"),
            "the placeholder must be substituted"
        );
    }

    /// Fills the 12 required units so the phase-2 playbook must appear.
    fn fill_required(project: &str) {
        for (path, value) in [
            ("require.project.name", json!("Nửa Đêm")),
            ("require.project.language", json!("VI")),
            ("require.project.plan.arc.count", json!(3)),
            ("require.project.plan.chapter.count", json!(12)),
            ("require.project.plan.scene.count", json!(36)),
            ("require.project.plan.word.count", json!(90000)),
        ] {
            crate::super_json::set(project, path, value).unwrap();
        }
        for (table, rows) in [
            (
                "project_genres",
                json!([{ "genre_id": "trinh thám", "is_primary": true }]),
            ),
            ("arcs", json!([{ "name": "Mở", "position": 1 }])),
            ("characters", json!([{ "name": "An", "is_primary": true }])),
            (
                "project_motifs",
                json!([{ "motif_id": "den-bao", "scope": "project" }]),
            ),
            (
                "themes",
                json!([{ "scope": "PROJECT", "question": "Tự do là gì?" }]),
            ),
            (
                "theme_arguments",
                json!([{ "theme_id_json_index": 0, "character_id_json_index": 0, "role": "PRIMARY" }]),
            ),
        ] {
            let path = format!("require.schema.{table}.rows");
            crate::super_json::set(project, &path, rows).unwrap();
            let path = format!("require.schema.{table}.count");
            crate::super_json::set(project, &path, json!(1)).unwrap();
        }
    }

    #[test]
    fn test_prompt_appends_the_playbook_once_required_is_complete() {
        let _home = crate::test_support::TempHome::new("prompt-playbook");
        crate::super_json::init_project("p", None).unwrap();
        let early = assemble_prompt("OpenAI", "gpt-4o", "s", "p");
        assert!(
            !early.contains("# Outline playbook"),
            "the playbook must wait for the required half"
        );

        fill_required("p");
        let prompt = assemble_prompt("OpenAI", "gpt-4o", "s", "p");
        assert!(
            prompt.contains("# Outline playbook"),
            "phase 2 rules are missing once the plan is complete"
        );
        assert!(
            prompt.matches("# Outline playbook").count() == 1,
            "the playbook must be appended exactly once"
        );
        assert!(!prompt.contains("{super_json}"));
    }

    #[test]
    fn test_prompt_without_project_does_not_create_a_file() {
        let _home = crate::test_support::TempHome::new("prompt-no-project");
        let prompt = assemble_prompt("OpenAI", "gpt-4o", "s", "");
        assert!(prompt.contains("(chưa có project)"));
        let path = crate::paths::super_json_path("p").unwrap();
        assert!(
            !path.exists(),
            "assembling a prompt must not create {}",
            path.display()
        );
    }
}
