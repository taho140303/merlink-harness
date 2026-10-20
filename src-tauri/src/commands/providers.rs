use crate::{adapters, models, store};

#[tauri::command]
pub(crate) async fn load_providers() -> Result<Vec<models::ProviderConfig>, String> {
    let mut providers = store::load_providers()?;
    // If Ollama is running, add a local provider once with the models actually
    // installed on this machine. Persisting it makes it available to chat,
    // outline, and writing flows that resolve providers from the shared config.
    if let Some(local) = adapters::detect_local_ollama().await
        && !providers.iter().any(|provider| provider.id == local.id)
    {
        providers.push(local);
        store::save_providers(&providers)?;
    }
    Ok(providers)
}

#[tauri::command]
pub(crate) fn save_providers(providers: Vec<models::ProviderConfig>) -> Result<(), String> {
    store::save_providers(&providers)
}

/// Lists the models a provider serves (Sprint 11.5 · D1).
///
/// Called from JS with **camelCase** keys (`providerKind`, `baseUrl`, `apiKey`) —
/// `tauri-macros` lower-camel-cases every Rust parameter name, and snake_case keys
/// fail with "missing required key" (D11).
#[tauri::command]
pub(crate) async fn fetch_models(
    provider_kind: String,
    base_url: String,
    api_key: String,
) -> Result<Vec<models::FetchedModel>, String> {
    adapters::fetch_models(&provider_kind, &base_url, &api_key).await
}
