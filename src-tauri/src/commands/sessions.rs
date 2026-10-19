use crate::{db, models::SessionSummary};

#[tauri::command]
pub(crate) fn list_sessions() -> Result<Vec<SessionSummary>, String> {
    db::list_sessions(&db::open()?)
}

#[tauri::command]
pub(crate) fn get_session(session_id: String) -> Result<Option<SessionSummary>, String> {
    db::get_session(&db::open()?, &session_id)
}

#[tauri::command]
pub(crate) fn rename_session(session_id: String, title: String) -> Result<SessionSummary, String> {
    db::rename_session(&db::open()?, &session_id, &title)
}

#[tauri::command]
pub(crate) fn delete_session(session_id: String) -> Result<bool, String> {
    db::delete_session(&db::open()?, &session_id)
}
