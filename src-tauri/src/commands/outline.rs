use crate::{db, models, outline, store};
use outline::stage::Stage;
use serde_json::Value;
use std::sync::{Arc, atomic::AtomicBool};

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
pub(crate) fn outline_status(session_id: String) -> Result<models::OutlineStatus, String> {
    outline::status(&session_id)
}
#[tauri::command]
pub(crate) fn outline_start(
    project_id: String,
    session_id: String,
) -> Result<models::OutlineStatus, String> {
    outline::start(&project_id, &session_id)
}
#[tauri::command]
pub(crate) async fn outline_run(
    session_id: String,
    provider_id: String,
    model_id: String,
    stage: String,
    range: Option<String>,
    channel: tauri::ipc::Channel<models::OutlineEvent>,
) -> Result<models::OutlineRunResult, String> {
    let (provider, model) = resolve_provider_model(&provider_id, &model_id)?;
    let stage = Stage::parse(&stage)?;
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
    let forward = tokio::spawn(async move {
        while let Some(event) = rx.recv().await {
            let _ = channel.send(event);
        }
    });
    let result = outline::run(
        &session_id,
        &provider,
        &model,
        stage,
        range.as_deref(),
        tx,
        Arc::new(AtomicBool::new(false)),
    )
    .await;
    let _ = forward.await;
    result
}
#[tauri::command]
pub(crate) fn outline_items(
    session_id: String,
    stage: String,
) -> Result<Vec<models::OutlineItem>, String> {
    outline::items(&session_id, Stage::parse(&stage)?)
}
#[tauri::command]
pub(crate) async fn outline_regenerate(
    session_id: String,
    provider_id: String,
    model_id: String,
    stage: String,
    key: String,
    note: String,
) -> Result<models::OutlineRunResult, String> {
    let (p, m) = resolve_provider_model(&provider_id, &model_id)?;
    outline::regenerate(&session_id, &p, &m, Stage::parse(&stage)?, &key, &note).await
}
#[tauri::command]
pub(crate) fn outline_commit(
    session_id: String,
    stage: String,
    keys: Option<Vec<String>>,
) -> Result<models::CommitReport, String> {
    outline::commit(&session_id, Stage::parse(&stage)?, keys.as_deref())
}
#[tauri::command]
pub(crate) fn outline_accept(
    session_id: String,
    stage: String,
    keys: Option<Vec<String>>,
) -> Result<u32, String> {
    outline::accept(&session_id, Stage::parse(&stage)?, keys.as_deref())
}

#[tauri::command]
pub(crate) fn outline_reject(
    session_id: String,
    stage: String,
    keys: Option<Vec<String>>,
) -> Result<u32, String> {
    outline::reject(&session_id, Stage::parse(&stage)?, keys.as_deref())
}

#[tauri::command]
pub(crate) fn outline_cancel(session_id: String) -> Result<(), String> {
    outline::cancel(&session_id)
}
fn resolve_provider_model(
    provider_id: &str,
    model_id: &str,
) -> Result<(models::ProviderConfig, models::ModelConfig), String> {
    let providers = store::load_providers()?;
    let provider = providers
        .iter()
        .find(|p| p.id == provider_id)
        .ok_or("Provider not found")?;
    let model = provider
        .models
        .iter()
        .find(|m| m.id == model_id)
        .ok_or("Model does not belong to provider")?;
    Ok((provider.clone(), model.clone()))
}
