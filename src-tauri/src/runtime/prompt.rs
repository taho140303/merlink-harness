//! System prompt assembly.

const SYSTEM_TEMPLATE: &str = include_str!("../../prompts/harness_system.md");

/// What `{project_state}` says while the chat has no project yet. Worded as an
/// instruction because that is how the model reads it.
const NO_PROJECT_STATE: &str =
    "(none yet - call project_create once the author says what they want to write)";

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
    SYSTEM_TEMPLATE
        .replace("{provider_name}", provider_name)
        .replace("{model_name}", model_name)
        .replace("{session_id}", session_id)
        .replace("{project_state}", project_state)
        .replace("{required_total}", &required_total)
        .replace("{optional_total}", &optional_total)
}
