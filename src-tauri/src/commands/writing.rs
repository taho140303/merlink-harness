use std::sync::{Arc, atomic::AtomicBool};

use serde_json::{Value, json};

use crate::{models, store, writing};

/// One scene addressed by chapter and position, the key the plan uses.
fn scene_pairs(scenes: &Value) -> Result<Vec<(u64, u64)>, String> {
    let entries = scenes
        .as_array()
        .ok_or("Cần mảng 'scenes' gồm {chapter, position}")?;
    let mut pairs = Vec::with_capacity(entries.len());
    for entry in entries {
        let chapter = entry
            .get("chapter")
            .and_then(Value::as_u64)
            .ok_or("Mỗi cảnh cần 'chapter'")?;
        let position = entry
            .get("position")
            .and_then(Value::as_u64)
            .ok_or("Mỗi cảnh cần 'position'")?;
        pairs.push((chapter, position));
    }
    Ok(pairs)
}

fn resolve_provider_model(
    provider_id: &str,
    model_id: &str,
) -> Result<(models::ProviderConfig, models::ModelConfig), String> {
    let providers = store::load_providers()?;
    let provider = providers
        .iter()
        .find(|provider| provider.id == provider_id)
        .ok_or("Provider not found")?;
    let model = provider
        .models
        .iter()
        .find(|model| model.id == model_id)
        .ok_or("Model does not belong to provider")?;
    Ok((provider.clone(), model.clone()))
}

#[tauri::command]
pub(crate) fn writing_status(project_id: String) -> Result<Value, String> {
    writing::status(&project_id)
}

#[tauri::command]
pub(crate) async fn write_scene(
    project_id: String,
    provider_id: String,
    model_id: String,
    chapter: u64,
    position: u64,
    note: Option<String>,
) -> Result<Value, String> {
    let (provider, model) = resolve_provider_model(&provider_id, &model_id)?;
    let (sender, _receiver) = tokio::sync::mpsc::unbounded_channel();
    let report = writing::write_scene(
        Arc::new(crate::adapters::HttpBackend),
        &provider,
        &model,
        &project_id,
        chapter,
        position,
        note.as_deref(),
        sender,
    )
    .await?;
    Ok(json!({
        "scene": report.scene,
        "path": report.path,
        "word_count": report.word_count,
        "within_budget": report.within_budget,
        "clean": report.clean,
        "violations": report.violations,
    }))
}

#[tauri::command]
pub(crate) async fn rewrite_scenes(
    project_id: String,
    provider_id: String,
    model_id: String,
    scenes: Value,
    note: Option<String>,
) -> Result<Value, String> {
    let (provider, model) = resolve_provider_model(&provider_id, &model_id)?;
    let pairs = scene_pairs(&scenes)?;
    let reports = writing::rewrite_selection(
        Arc::new(crate::adapters::HttpBackend),
        &provider,
        &model,
        &project_id,
        &pairs,
        note.as_deref(),
        Arc::new(AtomicBool::new(false)),
    )
    .await?;
    Ok(Value::Array(
        reports
            .into_iter()
            .map(|report| {
                json!({
                    "scene": report.scene,
                    "path": report.path,
                    "word_count": report.word_count,
                    "within_budget": report.within_budget,
                    "clean": report.clean,
                })
            })
            .collect(),
    ))
}

#[tauri::command]
pub(crate) fn lock_scene(
    project_id: String,
    chapter: u64,
    position: u64,
    locked: bool,
) -> Result<(), String> {
    writing::set_locked(&project_id, chapter, position, locked)
}
