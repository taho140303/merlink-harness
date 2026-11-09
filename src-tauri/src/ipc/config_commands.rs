//! Model Configuration & Credentials IPC commands (`REQ-CMD-038`, `REQ-CMD-039`, `REQ-CMD-040`, `§10.2.10`, `CI-10`).
//!
//! Handlers:
//! - IPC #38: `get_runtime_and_provider_config`
//! - IPC #39: `update_runtime_and_provider_config`
//! - IPC #40: `set_provider_api_key`

use crate::domain::workflow_planner::WorkflowConfig;
use crate::error::{ErrorCode, IpcError};
use crate::ipc::dto::{
    GetRuntimeConfigReq, ProviderStatusDto, RuntimeAndProviderConfigDto, SetProviderApiKeyReq,
    SetProviderApiKeyResponseDto, SubagentConfigDto, UpdateRuntimeConfigReq,
};
use crate::providers::credentials::{CredentialManager, CredentialStorageMode};
use crate::providers::redaction::Redactor;
use parking_lot::RwLock;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Write;
use std::path::PathBuf;
use std::sync::Arc;
use crate::providers::traits::{LlmProvider, ProviderChatRequest, ProviderChatResponse, ProviderError, ProviderStreamChunk, ProviderStreamRequest, ProviderStreamSummary};
use tokio_util::sync::CancellationToken;
use tokio::sync::mpsc;

/// In-memory runtime state for provider settings and runtime subagent options (`CI-10`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderRuntimeSettings {
    pub provider_id: String,
    pub name: String,
    pub endpoint_url: Option<String>,
    pub model_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct PersistedRuntimeConfig {
    version: u8,
    subagent_config: SubagentConfigDto,
    active_provider_id: String,
    providers: HashMap<String, ProviderRuntimeSettings>,
}

/// Shared runtime configuration state across IPC calls.
#[derive(Debug)]
pub struct RuntimeConfigState {
    pub subagent_config: RwLock<SubagentConfigDto>,
    pub active_provider_id: RwLock<String>,
    pub providers: RwLock<HashMap<String, ProviderRuntimeSettings>>,
    pub credential_manager: CredentialManager,
    pub redactor: Arc<Redactor>,
    persistence_path: Option<PathBuf>,
    update_lock: Mutex<()>,
}

impl Default for RuntimeConfigState {
    fn default() -> Self {
        Self::new(CredentialManager::new(), Arc::new(Redactor::new()))
    }
}

impl RuntimeConfigState {
    pub fn new(credential_manager: CredentialManager, redactor: Arc<Redactor>) -> Self {
        // No providers or models are preselected; users add their own configuration.
        let providers = HashMap::new();
        Self {
            subagent_config: RwLock::new(SubagentConfigDto::default()),
            active_provider_id: RwLock::new(String::new()),
            providers: RwLock::new(providers),
            credential_manager,
            redactor,
            persistence_path: None,
            update_lock: Mutex::new(()),
        }
    }

    /// Load non-secret provider settings from the app data directory.
    /// Credentials remain exclusively in the OS Keychain or session memory.
    pub fn with_persistence(path: PathBuf) -> Result<Self, IpcError> {
        let state = Self::default();
        if path.exists() {
            let bytes = std::fs::read(&path)
                .map_err(|e| IpcError::new(ErrorCode::InternalError, format!("Cannot read provider settings: {e}")))?;
            let saved: PersistedRuntimeConfig = serde_json::from_slice(&bytes)
                .map_err(|e| IpcError::new(ErrorCode::InternalError, format!("Invalid provider settings: {e}")))?;
            if saved.version != 1 {
                return Err(IpcError::new(ErrorCode::ValidationFailed, "Unsupported provider settings version"));
            }
            validate_subagent_config(&saved.subagent_config)?;
            if !saved.active_provider_id.is_empty() && !saved.providers.contains_key(&saved.active_provider_id) {
                return Err(IpcError::new(ErrorCode::ValidationFailed, "Active provider is missing from saved settings"));
            }
            *state.subagent_config.write() = saved.subagent_config;
            *state.active_provider_id.write() = saved.active_provider_id;
            *state.providers.write() = saved.providers;
        }
        Ok(Self { persistence_path: Some(path), ..state })
    }

    fn persist_config(&self, config: &PersistedRuntimeConfig) -> Result<(), IpcError> {
        let Some(path) = &self.persistence_path else { return Ok(()); };
        let bytes = serde_json::to_vec(config)
            .map_err(|e| IpcError::new(ErrorCode::InternalError, format!("Cannot encode provider settings: {e}")))?;
        let parent = path.parent().ok_or_else(|| IpcError::new(ErrorCode::InternalError, "Provider settings path has no parent"))?;
        std::fs::create_dir_all(parent)
            .map_err(|e| IpcError::new(ErrorCode::InternalError, format!("Cannot create settings directory: {e}")))?;
        let temp_path = path.with_extension(format!("tmp-{}", uuid::Uuid::new_v4()));
        let mut options = std::fs::OpenOptions::new();
        options.write(true).create_new(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(0o600);
        }
        let result = (|| -> std::io::Result<()> {
            let mut file = options.open(&temp_path)?;
            file.write_all(&bytes)?;
            file.sync_all()?;
            std::fs::rename(&temp_path, path)?;
            Ok(())
        })();
        if let Err(err) = result {
            let _ = std::fs::remove_file(&temp_path);
            return Err(IpcError::new(ErrorCode::InternalError, format!("Cannot save provider settings: {err}")));
        }
        Ok(())
    }

    /// Resolve the selected provider for both chat and scene generation.
    pub fn selected_provider(&self) -> Result<Arc<dyn LlmProvider>, IpcError> {
        let id = self.active_provider_id.read().clone();
        self.provider_for_model(&id, None)
    }

    /// Resolve the saved worker choice; `chat` follows the current active provider/model.
    pub fn resolve_subagent_model(&self) -> Result<(String, String), IpcError> {
        let choice = self.subagent_config.read().model.clone();
        let (id, override_model) = if choice == "chat" {
            (self.active_provider_id.read().clone(), None)
        } else {
            let (provider_id, model_id) = choice.split_once('/').ok_or_else(||
                IpcError::new(ErrorCode::ValidationFailed, "Subagent model must be provider/model"))?;
            if provider_id.trim().is_empty() || model_id.trim().is_empty() {
                return Err(IpcError::new(ErrorCode::ValidationFailed, "Subagent provider and model must be non-empty"));
            }
            (provider_id.to_string(), Some(model_id.to_string()))
        };
        let settings = self.providers.read().get(&id).cloned().ok_or_else(||
            IpcError::new(ErrorCode::ValidationFailed, "Subagent provider is not configured"))?;
        let model = override_model.or(settings.model_name).filter(|s| !s.trim().is_empty()).ok_or_else(||
            IpcError::new(ErrorCode::ValidationFailed, "Subagent model is not configured"))?;
        Ok((id, model))
    }

    pub fn subagent_provider(&self) -> Result<Arc<dyn LlmProvider>, IpcError> {
        let (id, model) = self.resolve_subagent_model()?;
        self.provider_for_model(&id, Some(model))
    }

    fn provider_for_model(&self, id: &str, override_model: Option<String>) -> Result<Arc<dyn LlmProvider>, IpcError> {
        if id.is_empty() {
            return Err(IpcError::new(ErrorCode::ValidationFailed, "No active AI provider configured"));
        }
        let settings = self.providers.read().get(id).cloned().ok_or_else(|| {
            IpcError::new(ErrorCode::ValidationFailed, "Unsupported active provider")
        })?;
        let endpoint = settings.endpoint_url.filter(|s| !s.trim().is_empty()).ok_or_else(|| {
            IpcError::new(ErrorCode::ValidationFailed, "Selected provider endpoint is missing")
        })?;
        let model = override_model.or(settings.model_name).filter(|s| !s.trim().is_empty()).ok_or_else(|| {
            IpcError::new(ErrorCode::ValidationFailed, "Selected provider model is missing")
        })?;
        let adapter: Arc<dyn crate::providers::traits::LlmProvider> = match id {
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
    fn test_provider_config_persists_across_restart_without_secret() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("provider-config.json");
        let state = RuntimeConfigState::with_persistence(path.clone()).unwrap();
        exec_update_runtime_and_provider_config(&state, &UpdateRuntimeConfigReq {
            active_provider_id: Some("custom".into()),
            endpoint_url: Some("https://example.com/v1".into()),
            model_name: Some("story-model".into()),
            ..UpdateRuntimeConfigReq::default()
        }).unwrap();
        drop(state);

        let restored = RuntimeConfigState::with_persistence(path.clone()).unwrap();
        assert_eq!(&*restored.active_provider_id.read(), "custom");
        assert_eq!(restored.providers.read()["custom"].model_name.as_deref(), Some("story-model"));
        assert!(!std::fs::read_to_string(path).unwrap().contains("api_key"));
    }

    #[test]
    fn test_subagent_config_maps_every_workflow_unit() {
        let cfg = SubagentConfigDto {
            max_subagents: 2,
            outline_capacity: 80,
            outline_lookback: 8,
            context_margin: 3,
            chapters_per_subagent: 12,
            lore_records_per_subagent: 7,
            characters_per_subagent: 2,
            relationship_pairs_per_subagent: 4,
            ..SubagentConfigDto::default()
        };
        let planner = workflow_config_from_subagent(&cfg).unwrap();
        assert_eq!(planner.max_parallel, 2);
        assert_eq!(planner.outline_cap, 80);
        assert_eq!(planner.outline_lookback, 8);
        assert_eq!(planner.context_margin, 3);
        assert_eq!(planner.unit_chapter, 12);
        assert_eq!(planner.unit_world_lore, 7);
        assert_eq!(planner.unit_character, 2);
        assert_eq!(planner.unit_relationship, 4);
    }

    #[test]
    fn test_subagent_config_survives_restart_and_rejects_invalid_sizes() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("provider-config.json");
        let state = RuntimeConfigState::with_persistence(path.clone()).unwrap();
        let saved = SubagentConfigDto {
            max_subagents: 4, rejected_threshold: 5, outline_capacity: 75,
            outline_lookback: 12, context_margin: 3, chapters_per_subagent: 15,
            lore_records_per_subagent: 6, characters_per_subagent: 2,
            relationship_pairs_per_subagent: 7, model: "chat".into(),
        };
        exec_update_runtime_and_provider_config(&state, &UpdateRuntimeConfigReq {
            subagent_config: Some(saved.clone()), ..UpdateRuntimeConfigReq::default()
        }).unwrap();
        let restored = RuntimeConfigState::with_persistence(path).unwrap();
        assert_eq!(*restored.subagent_config.read(), saved);
        let bad = SubagentConfigDto { chapters_per_subagent: 0, ..saved.clone() };
        assert!(validate_subagent_config(&bad).is_err());
        let overflow = SubagentConfigDto { outline_capacity: i32::MAX as u32 + 1, ..saved };
        assert!(validate_subagent_config(&overflow).is_err());
    }

    #[test]
    fn test_subagent_model_resolution_fails_closed_on_missing_provider() {
        let state = RuntimeConfigState::default();
        assert!(state.subagent_provider().is_err());
        state.providers.write().insert("custom".into(), ProviderRuntimeSettings {
            provider_id: "custom".into(), name: "Custom".into(),
            endpoint_url: Some("https://example.com/v1".into()),
            model_name: Some("chat-model".into()),
        });
        *state.active_provider_id.write() = "custom".into();
        assert_eq!(state.resolve_subagent_model().unwrap(), ("custom".into(), "chat-model".into()));
        state.subagent_config.write().model = "custom/worker-model".into();
        assert_eq!(state.resolve_subagent_model().unwrap(), ("custom".into(), "worker-model".into()));
        assert_eq!(state.subagent_provider().unwrap().provider_id(), "opencode");
        state.subagent_config.write().model = "missing/worker-model".into();
        assert!(state.subagent_provider().is_err());
    }

    #[test]
    fn test_removed_provider_stays_removed_after_restart() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("provider-config.json");
        let state = RuntimeConfigState::with_persistence(path.clone()).unwrap();
        exec_update_runtime_and_provider_config(&state, &UpdateRuntimeConfigReq {
            active_provider_id: Some("custom".into()),
            endpoint_url: Some("https://example.com/v1".into()),
            model_name: Some("story-model".into()),
            ..UpdateRuntimeConfigReq::default()
        }).unwrap();
        exec_update_runtime_and_provider_config(&state, &UpdateRuntimeConfigReq {
            remove_provider_id: Some("custom".into()),
            ..UpdateRuntimeConfigReq::default()
        }).unwrap();
        let restored = RuntimeConfigState::with_persistence(path).unwrap();
        assert!(restored.providers.read().is_empty());
        assert!(restored.active_provider_id.read().is_empty());
    }

    #[test]
    fn test_selected_provider_uses_configured_model_and_endpoint() {
        let state = RuntimeConfigState::default();
        assert!(state.providers.read().is_empty());
        assert!(state.selected_provider().is_err());

        state.providers.write().insert(
            "opencode".into(),
            ProviderRuntimeSettings {
                provider_id: "opencode".into(),
                name: "My Provider".into(),
                endpoint_url: Some("https://example.com/v1".into()),
                model_name: Some("my-model".into()),
            },
        );
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

    if cfg.rejected_threshold == 0 || cfg.outline_capacity == 0 || cfg.chapters_per_subagent == 0
        || cfg.lore_records_per_subagent == 0 || cfg.characters_per_subagent == 0
        || cfg.relationship_pairs_per_subagent == 0 {
        return Err(IpcError::new(ErrorCode::ValidationFailed, "Subagent thresholds and task sizes must be positive"));
    }
    if [cfg.outline_capacity, cfg.outline_lookback, cfg.context_margin, cfg.chapters_per_subagent,
        cfg.lore_records_per_subagent, cfg.characters_per_subagent, cfg.relationship_pairs_per_subagent]
        .iter().any(|&value| value > i32::MAX as u32) {
        return Err(IpcError::new(ErrorCode::ValidationFailed, "Subagent task sizes exceed planner limits"));
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

/// Convert a validated saved UI configuration into the planner's frozen per-run values.
pub fn workflow_config_from_subagent(cfg: &SubagentConfigDto) -> Result<WorkflowConfig, IpcError> {
    validate_subagent_config(cfg)?;
    let config = WorkflowConfig {
        max_parallel: cfg.max_subagents as usize,
        outline_cap: cfg.outline_capacity as i32,
        outline_lookback: cfg.outline_lookback as i32,
        context_margin: cfg.context_margin as i32,
        unit_chapter: cfg.chapters_per_subagent as i32,
        unit_world_lore: cfg.lore_records_per_subagent as i32,
        unit_character: cfg.characters_per_subagent as i32,
        unit_relationship: cfg.relationship_pairs_per_subagent as i32,
        ..WorkflowConfig::default()
    };
    config.validate().map_err(|error| IpcError::new(ErrorCode::ValidationFailed, error.to_string()))?;
    Ok(config)
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
    let _guard = state.update_lock.lock();
    let mut subagent_config = state.subagent_config.read().clone();
    let mut active_id = state.active_provider_id.read().clone();
    let mut providers = state.providers.read().clone();

    if let Some(subagent_cfg) = &req.subagent_config {
        validate_subagent_config(subagent_cfg)?;
        subagent_config = subagent_cfg.clone();
    }

    let removed_id = req.remove_provider_id.as_deref().map(str::trim).filter(|id| !id.is_empty());
    if let Some(id) = removed_id {
        if req.active_provider_id.as_deref().map(str::trim) == Some(id) {
            return Err(IpcError::new(ErrorCode::ValidationFailed, "Cannot select and remove the same provider"));
        }
        providers.remove(id);
        if active_id == id { active_id.clear(); }
    }

    if let Some(requested_active_id) = &req.active_provider_id {
        let trimmed = requested_active_id.trim();
        if trimmed.is_empty() {
            return Err(IpcError::new(
                ErrorCode::ValidationFailed,
                "activeProviderId cannot be empty",
            ));
        }
        // Custom (OpenAI-compatible) providers are registered on first save.
        providers.entry(trimmed.to_string()).or_insert_with(|| ProviderRuntimeSettings {
            provider_id: trimmed.to_string(),
            name: trimmed.to_string(),
            endpoint_url: None,
            model_name: None,
        });
        active_id = trimmed.to_string();
    }

    if req.model_name.is_some() || req.endpoint_url.is_some() {
        let entry = providers.get_mut(&active_id).ok_or_else(|| {
            IpcError::new(ErrorCode::ValidationFailed, "Unsupported active provider")
        })?;

        if let Some(model) = &req.model_name {
            entry.model_name = Some(model.clone());
        }
        if let Some(url) = &req.endpoint_url {
            entry.endpoint_url = Some(url.clone());
        }
    }

    state.persist_config(&PersistedRuntimeConfig {
        version: 1,
        subagent_config: subagent_config.clone(),
        active_provider_id: active_id.clone(),
        providers: providers.clone(),
    })?;
    *state.subagent_config.write() = subagent_config.clone();
    *state.active_provider_id.write() = active_id;
    *state.providers.write() = providers;
    if let Some(id) = removed_id {
        state.credential_manager.delete_api_key(id);
    }

    // Return current state snapshot
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

    // A key saved before the endpoint/model should still appear as a user-added provider.
    state
        .providers
        .write()
        .entry(trimmed_provider.to_string())
        .or_insert_with(|| ProviderRuntimeSettings {
            provider_id: trimmed_provider.to_string(),
            name: trimmed_provider.to_string(),
            endpoint_url: None,
            model_name: None,
        });

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
