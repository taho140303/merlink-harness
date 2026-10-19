use crate::{models, runtime, tools};

#[tauri::command]
pub(crate) fn super_json_read(project_id: String) -> Result<serde_json::Value, String> {
    runtime::super_json_document(&project_id)
}

/// Progress for the chat header (D7). Computed in Rust: the frontend renders the
/// numbers, it never derives them.
#[tauri::command]
pub(crate) fn super_json_progress(session_id: String) -> Result<models::SuperJsonProgress, String> {
    runtime::super_json_progress_for_session(&session_id)
}

/// The `Chi tiết` dialog's six-paragraph summary (Sprint 11.11 · D2/D5).
///
/// Called from JS with **camelCase** keys (`{ projectId }`); `tauri-macros`
/// lower-camel-cases the Rust parameter name, so `project_id` is rejected with
/// "missing required key".
#[tauri::command]
pub(crate) fn super_json_summary(project_id: String) -> Result<String, String> {
    runtime::super_json_summary_for_project(&project_id)
}

/// `path -> Vietnamese label` for the tool-log dialog (Sprint 11.12 · D8).
///
/// The only source is `tools::PATH_LABELS`; the frontend never hardcodes a second
/// map. Takes no arguments, so JS calls `safeInvoke("super_json_labels")`.
#[tauri::command]
pub(crate) fn super_json_labels() -> std::collections::BTreeMap<String, String> {
    tools::field_labels()
        .into_iter()
        .map(|(path, label)| (path.to_owned(), label.to_owned()))
        .collect()
}
