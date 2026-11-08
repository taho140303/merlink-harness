//! Model Configuration & Credentials IPC commands (`REQ-CMD-038`, `REQ-CMD-039`, `REQ-CMD-040`, `§10.2.10`, `CI-10`).
//!
//! Handlers:
//! - IPC #38: `get_runtime_and_provider_config`
//! - IPC #39: `update_runtime_and_provider_config`
//! - IPC #40: `set_provider_api_key`

use crate::error::{ErrorCode, IpcError};
use crate::ipc::dto::{
    GetRuntimeConfigReq, ProviderStatusDto, RuntimeAndProviderConfigDto, SetProviderApiKeyReq,
    SetProviderApiKeyResponseDto, SubagentConfigDto, UpdateRuntimeConfigReq,
};
use crate::providers::credentials::{CredentialManager, CredentialStorageMode};
use crate::providers::redaction::Redactor;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use crate::providers::traits::{LlmProvider, ProviderChatRequest, ProviderChatResponse, ProviderError, ProviderStreamChunk, ProviderStreamRequest, ProviderStreamSummary};
use tokio_util::sync::CancellationToken;
use tokio::sync::mpsc;

/// In-memory runtime state for provider settings and runtime subagent options (`CI-10`).
#[derive(Debug, Clone)]
pub struct ProviderRuntimeSettings {
    pub provider_id: String,
    pub name: String,
    pub endpoint_url: Option<String>,
    pub model_name: Option<String>,
}

/// Shared runtime configuration state across IPC calls.
#[derive(Debug)]
pub struct RuntimeConfigState {
    pub subagent_config: RwLock<SubagentConfigDto>,
    pub active_provider_id: RwLock<String>,
    pub providers: RwLock<HashMap<String, ProviderRuntimeSettings>>,
    pub credential_manager: CredentialManager,
    pub redactor: Arc<Redactor>,
}

impl Default for RuntimeConfigState {
    fn default() -> Self {
        Self::new(CredentialManager::new(), Arc::new(Redactor::new()))
    }
}

impl RuntimeConfigState {
    pub fn new(credential_manager: CredentialManager, redactor: Arc<Redactor>) -> Self {
        Self {
            subagent_config: RwLock::new(SubagentConfigDto::default()),
            active_provider_id: RwLock::new(String::new()),
            providers: RwLock::new(HashMap::new()),
            credential_manager,
            redactor,
        }
    }

    /// Resolve the selected provider for both chat and scene generation.
    pub fn selected_provider(&self) -> Result<Arc<dyn crate::providers::traits::LlmProvider>, IpcError> {
        let id = self.active_provider_id.read().clone();
        if id.is_empty() {
            return Err(IpcError::new(ErrorCode::ValidationFailed, "No active AI provider configured"));
        }
        let settings = self.providers.read().get(&id).cloned().ok_or_else(|| {
            IpcError::new(ErrorCode::ValidationFailed, "Unsupported active provider")
        })?;
        let endpoint = settings.endpoint_url.filter(|s| !s.trim().is_empty()).ok_or_else(|| {
            IpcError::new(ErrorCode::ValidationFailed, "Selected provider endpoint is missing")
        })?;
        let model = settings.model_name.filter(|s| !s.trim().is_empty()).ok_or_else(|| {
            IpcError::new(ErrorCode::ValidationFailed, "Selected provider model is missing")
        })?;
        let adapter: Arc<dyn crate::providers::traits::LlmProvider> = match id.as_str() {
            "ollama" => Arc::new(crate::providers::ollama::OllamaProvider::new(
                normalize_provider_endpoint(&endpoint, "api/chat")?, false,
            )),
            "opencode" => Arc::new(crate::providers::opencode::OpenCodeProvider::new(
                normalize_provider_endpoint(&endpoint, "chat/completions")?,
                self.credential_manager.get_api_key(&id),
            )),
            // Any other (custom) provider is treated as OpenAI-compatible.
            _ => Arc::new(crate::providers::opencode::OpenCodeProvider::new(
                normalize_provider_endpoint(&endpoint, "chat/completions")?,
                self.credential_manager.get_api_key(&id),
            )),
        };
        Ok(Arc::new(SelectedModelProvider { adapter, model }))
    }

    /// Build current provider status list.
    pub fn build_provider_statuses(&self) -> Vec<ProviderStatusDto> {
        let active_id = self.active_provider_id.read().clone();
        let providers_guard = self.providers.read();

        let mut list = Vec::new();
        for (id, p) in providers_guard.iter() {
            let has_key = self.credential_manager.has_api_key(id);
            list.push(ProviderStatusDto {
                provider_id: id.clone(),
                name: p.name.clone(),
                is_active: *id == active_id,
                has_api_key: has_key,
                endpoint_url: p.endpoint_url.clone(),
                model_name: p.model_name.clone(),
            });
        }

        // Sort deterministically by provider_id
        list.sort_by(|a, b| a.provider_id.cmp(&b.provider_id));
        list
    }

    /// Current credential storage mode string ("os_keychain" | "session_only_fallback").
    pub fn credential_storage_mode(&self) -> &'static str {
        if self.credential_manager.is_keychain_available() {
            CredentialStorageMode::OsKeychain.as_str()
        } else {
            CredentialStorageMode::SessionOnlyFallback.as_str()
        }
    }
}

fn normalize_provider_endpoint(raw: &str, route: &str) -> Result<String, IpcError> {
    let url = url::Url::parse(raw.trim()).map_err(|_| IpcError::new(ErrorCode::ValidationFailed, "Invalid provider endpoint URL"))?;
    if !matches!(url.scheme(), "http" | "https") || url.host_str().is_none() {
        return Err(IpcError::new(ErrorCode::ValidationFailed, "Invalid provider endpoint URL"));
    }
    let base = raw.trim().trim_end_matches('/');
    if base.ends_with(&format!("/{route}")) { return Ok(base.to_string()); }
    let prefix = if route == "api/chat" { "api" } else { "v1" };
    if base.ends_with(&format!("/{prefix}")) {
        return Ok(format!("{base}/{}", route.strip_prefix(&format!("{prefix}/")).unwrap_or(route)));
    }
    Ok(format!("{base}/{route}"))
}

struct SelectedModelProvider {
    adapter: Arc<dyn LlmProvider>,
    model: String,
}

#[async_trait::async_trait]
impl LlmProvider for SelectedModelProvider {
    fn provider_id(&self) -> &'static str { self.adapter.provider_id() }

    async fn chat_with_tools(&self, req: &ProviderChatRequest, cancel: CancellationToken) -> Result<ProviderChatResponse, ProviderError> {
        let mut req = req.clone();
        req.model = self.model.clone();
        self.adapter.chat_with_tools(&req, cancel).await
    }

    async fn stream_completion(&self, req: &ProviderStreamRequest, tx: mpsc::Sender<ProviderStreamChunk>, cancel: CancellationToken) -> Result<ProviderStreamSummary, ProviderError> {
        let mut req = req.clone();
        req.model = self.model.clone();
        self.adapter.stream_completion(&req, tx, cancel).await
    }
}

#[cfg(test)]
mod provider_selection_tests {
    use super::*;

    #[test]
    fn test_selected_provider_uses_configured_model_and_endpoint() {
        let state = RuntimeConfigState::default();
        assert!(state.selected_provider().is_err());
        state.providers.write().insert(
            "ollama".to_string(),
            ProviderRuntimeSettings {
                provider_id: "ollama".to_string(),
                name: "Ollama (Native Local)".to_string(),
                endpoint_url: Some("http://127.0.0.1:11434".to_string()),
                model_name: Some("llama3.2".to_string()),
            },
        );
        state.providers.write().insert(
            "opencode".to_string(),
            ProviderRuntimeSettings {
                provider_id: "opencode".to_string(),
                name: "OpenCode / Cloud AI".to_string(),
                endpoint_url: Some("https://api.opencode.ai/v1/chat/completions".to_string()),
                model_name: Some("claude-3-5-sonnet".to_string()),
            },
        );
        *state.active_provider_id.write() = "ollama".into();
        let provider = state.selected_provider().unwrap();
        assert_eq!(provider.provider_id(), "ollama");
        *state.active_provider_id.write() = "opencode".into();
        assert_eq!(state.selected_provider().unwrap().provider_id(), "opencode");
        *state.active_provider_id.write() = "unsupported".into();
        assert!(state.selected_provider().is_err());
        assert_eq!(normalize_provider_endpoint("http://127.0.0.1:11434", "api/chat").unwrap(), "http://127.0.0.1:11434/api/chat");
        assert_eq!(normalize_provider_endpoint("https://example.com/v1/chat/completions", "chat/completions").unwrap(), "https://example.com/v1/chat/completions");
    }
}

/// Validate subagent arithmetic invariants (`subagentConfig.ts:20-51` and `core.md` §3.9).
/// - `1 <= maxSubagents <= 6`
/// - `0 <= outlineLookback < outlineCapacity`
pub fn validate_subagent_config(cfg: &SubagentConfigDto) -> Result<(), IpcError> {
    if cfg.max_subagents < 1 || cfg.max_subagents > 6 {
        return Err(IpcError::new(
            ErrorCode::ValidationFailed,
            format!(
                "maxSubagents must be between 1 and 6 inclusive, got {}",
                cfg.max_subagents
            ),
        ));
    }

    if cfg.outline_lookback >= cfg.outline_capacity {
        return Err(IpcError::new(
            ErrorCode::ValidationFailed,
            format!(
                "outlineLookback ({}) must be strictly less than outlineCapacity ({})",
                cfg.outline_lookback, cfg.outline_capacity
            ),
        ));
    }

    Ok(())
}

// ============================================================================
// Core Execution Logic (Pure Rust Fixture Compatible)
// ============================================================================

/// Execute IPC #38: `get_runtime_and_provider_config`
pub fn exec_get_runtime_and_provider_config(
    state: &RuntimeConfigState,
    _req: &GetRuntimeConfigReq,
) -> Result<RuntimeAndProviderConfigDto, IpcError> {
    let subagent_config = state.subagent_config.read().clone();
    let providers = state.build_provider_statuses();
    let credential_storage_mode = state.credential_storage_mode().to_string();

    Ok(RuntimeAndProviderConfigDto {
        subagent_config,
        providers,
        credential_storage_mode,
    })
}

/// Execute IPC #39: `update_runtime_and_provider_config`
pub fn exec_update_runtime_and_provider_config(
    state: &RuntimeConfigState,
    req: &UpdateRuntimeConfigReq,
) -> Result<RuntimeAndProviderConfigDto, IpcError> {
    if let Some(subagent_cfg) = &req.subagent_config {
        validate_subagent_config(subagent_cfg)?;
        *state.subagent_config.write() = subagent_cfg.clone();
    }

    if let Some(active_id) = &req.active_provider_id {
        let trimmed = active_id.trim();
        if trimmed.is_empty() {
            return Err(IpcError::new(
                ErrorCode::ValidationFailed,
                "activeProviderId cannot be empty",
            ));
        }
        {
            // Custom (OpenAI-compatible) providers are registered on first save.
            let mut providers_guard = state.providers.write();
            providers_guard.entry(trimmed.to_string()).or_insert_with(|| ProviderRuntimeSettings {
                provider_id: trimmed.to_string(),
                name: trimmed.to_string(),
                endpoint_url: None,
                model_name: None,
            });
        }
        *state.active_provider_id.write() = trimmed.to_string();
    }

    let active_id = state.active_provider_id.read().clone();

    if req.model_name.is_some() || req.endpoint_url.is_some() {
        let mut providers_guard = state.providers.write();
        let entry = providers_guard.get_mut(&active_id).ok_or_else(|| {
            IpcError::new(ErrorCode::ValidationFailed, "Unsupported active provider")
        })?;

        if let Some(model) = &req.model_name {
            entry.model_name = Some(model.clone());
        }
        if let Some(url) = &req.endpoint_url {
            entry.endpoint_url = Some(url.clone());
        }
    }

    // Return current state snapshot
    let subagent_config = state.subagent_config.read().clone();
    let providers = state.build_provider_statuses();
    let credential_storage_mode = state.credential_storage_mode().to_string();

    Ok(RuntimeAndProviderConfigDto {
        subagent_config,
        providers,
        credential_storage_mode,
    })
}

/// Execute IPC #40: `set_provider_api_key`
pub fn exec_set_provider_api_key(
    state: &RuntimeConfigState,
    req: &SetProviderApiKeyReq,
) -> Result<SetProviderApiKeyResponseDto, IpcError> {
    let trimmed_key = req.api_key.trim();
    if trimmed_key.is_empty() {
        return Err(IpcError::new(
            ErrorCode::ValidationFailed,
            "apiKey cannot be empty or whitespace",
        ));
    }

    let trimmed_provider = req.provider_id.trim();
    if trimmed_provider.is_empty() {
        return Err(IpcError::new(
            ErrorCode::ValidationFailed,
            "providerId cannot be empty or whitespace",
        ));
    }

    // 1. MUST register secret into redaction filter before storing (`§10.2.10`, `T-IPC-01`)
    state.redactor.register_secret(trimmed_key);

    // 2. Persist to OS Keychain or SessionKeyStore RAM fallback
    let mode = state
        .credential_manager
        .set_api_key(trimmed_provider, trimmed_key)
        .map_err(|err| IpcError::new(ErrorCode::InternalError, err))?;

    Ok(SetProviderApiKeyResponseDto {
        provider_id: trimmed_provider.to_string(),
        storage_mode: mode.as_str().to_string(),
    })
}

// ============================================================================
// Tauri v2 Command Handlers
// ============================================================================

#[tauri::command]
pub async fn get_runtime_and_provider_config(
    state: tauri::State<'_, Arc<RuntimeConfigState>>,
    req: GetRuntimeConfigReq,
) -> Result<RuntimeAndProviderConfigDto, IpcError> {
    exec_get_runtime_and_provider_config(&state, &req)
}

#[tauri::command]
pub async fn update_runtime_and_provider_config(
    state: tauri::State<'_, Arc<RuntimeConfigState>>,
    req: UpdateRuntimeConfigReq,
) -> Result<RuntimeAndProviderConfigDto, IpcError> {
    exec_update_runtime_and_provider_config(&state, &req)
}

#[tauri::command]
pub async fn set_provider_api_key(
    state: tauri::State<'_, Arc<RuntimeConfigState>>,
    req: SetProviderApiKeyReq,
) -> Result<SetProviderApiKeyResponseDto, IpcError> {
    exec_set_provider_api_key(&state, &req)
}
