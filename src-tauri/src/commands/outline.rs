use serde_json::Value;

use crate::{db, outline};

#[tauri::command]
pub(crate) fn outline_anchor(project_id: String) -> Result<Value, String> {
    if project_id.trim().is_empty() {
        return Err("Project ID must not be empty".into());
    }
    let connection = db::open()?;
    if !db::project_exists(&connection, &project_id)? {
        return Err(format!("Project not found: {project_id}"));
    }
    outline::anchor_for_project(&project_id)
}

#[tauri::command]
pub(crate) fn outline_status(session_id: String) -> Result<Value, String> {
    if session_id.trim().is_empty() {
        return Err("Session ID must not be empty".into());
    }
    let connection = db::open()?;
    let project_id = db::project_for_session(&connection, &session_id)?.unwrap_or_default();
    Ok(serde_json::json!({
        "ok": true,
        "stage": "seed",
        "active": false,
        "note": "outline engine arrives in sprint 17.2",
        "project_id": project_id
    }))
}
