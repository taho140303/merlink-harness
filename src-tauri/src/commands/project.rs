use crate::{db, models::ProjectSummary, runtime};

#[tauri::command]
pub(crate) fn list_projects() -> Result<Vec<ProjectSummary>, String> {
    let connection = db::open()?;
    db::list_project_summaries(&connection)
}

#[tauri::command]
pub(crate) fn get_project(project_id: String) -> Result<Option<ProjectSummary>, String> {
    let connection = db::open()?;
    db::project_summary(&connection, &project_id)
}

#[tauri::command]
pub(crate) fn execute_tool(
    tool_name: String,
    args: serde_json::Value,
    project_id: String,
    session_id: String,
) -> Result<serde_json::Value, String> {
    runtime::execute_tool_internal(&tool_name, args, &project_id, &session_id)
}
