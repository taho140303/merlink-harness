use crate::runtime;

#[tauri::command]
pub(crate) fn execute_tool(
    tool_name: String,
    args: serde_json::Value,
    project_id: String,
    session_id: String,
) -> Result<serde_json::Value, String> {
    runtime::execute_tool_internal(&tool_name, args, &project_id, &session_id)
}
