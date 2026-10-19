use crate::{adapters, models, store};

#[tauri::command]
pub(crate) fn load_providers() -> Result<Vec<models::ProviderConfig>, String> {
    store::load_providers()
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
