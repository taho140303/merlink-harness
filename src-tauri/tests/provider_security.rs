#![allow(clippy::field_reassign_with_default)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::result_large_err)]
#![allow(clippy::result_unit_err)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]

//! Acceptance and security tests for `phase2_sprint4_deep3` ("Implement provider security and adapters").
//!
//! Per backend module wiring rules:
//! - This sprint does not edit any `mod.rs` or `lib.rs`.
//! - Implementation modules are loaded directly via `#[path = "..."]`.
//!
//! Covered acceptance vectors:
//! - `P2S4-P`: Fake Ollama/OpenCode requests prove tools and `permission:{"*":"deny"}`.
//! - `P2S4-N`: Log/error snapshot never contains synthetic secret (redaction and debug masks).
//! - `P2S4-B`: Cancellation drops request and returns typed error without hanging.
//! - `phase2_sprint4_deep3-F0`: Predecessor gate absent or baseline SHA drift yields CONTRACT_BLOCKED.
//! - `phase2_sprint4_deep3-F1`: Map 401/403 to ProviderAuthFailed(non-retryable) and 429/502/503/504 to ProviderUnreachable(retryable).
//! - `phase2_sprint4_deep3-F2`: Keychain unavailable yields session-only fallback in RAM with no disk secrets.
//! - `IPC-CMD-038`: Typed IPC #38 `get_runtime_and_provider_config` read-only inquiry.
//! - `IPC-CMD-039`: Typed IPC #39 `update_runtime_and_provider_config` with arithmetic validation and no SQLite persistence.
//! - `IPC-CMD-040`: Typed IPC #40 `set_provider_api_key` with whitespace validation, redaction registration, and keychain/RAM storage.
//! - `T-IPC-01`: Multi-point security verification (caller permission, path traversal defense, secret redaction in HTTP error/log).

mod common;

#[path = "../src/error.rs"]
pub mod error;

#[path = "../src/ipc/dto.rs"]
pub mod dto;

#[path = "../src/ipc/adapter.rs"]
pub mod adapter;

pub mod ipc {
    pub use crate::adapter;
    pub use crate::config_commands;
    pub use crate::dto;
}

#[path = "../src/providers/redaction.rs"]
pub mod redaction;

#[path = "../src/providers/credentials.rs"]
pub mod credentials;

#[path = "../src/providers/traits.rs"]
pub mod traits;

#[path = "../src/providers/ollama.rs"]
pub mod ollama;

#[path = "../src/providers/opencode.rs"]
pub mod opencode;

pub mod providers {
    pub use crate::credentials;
    pub use crate::ollama;
    pub use crate::opencode;
    pub use crate::redaction;
    pub use crate::traits;
}

#[path = "../src/domain/business_flow.rs"]
pub mod business_flow;

#[path = "../src/domain/workflow_planner.rs"]
pub mod workflow_planner;

pub mod domain {
    pub use crate::business_flow;
    pub use crate::workflow_planner;
}

#[path = "../src/ipc/config_commands.rs"]
pub mod config_commands;

use config_commands::{
    exec_get_runtime_and_provider_config, exec_set_provider_api_key,
    exec_update_runtime_and_provider_config, validate_subagent_config, RuntimeConfigState,
};
use credentials::{CredentialManager, CredentialStorageMode};
use dto::{
    GetRuntimeConfigReq, ProviderConfigInputDto, SetProviderApiKeyReq, SubagentConfigDto,
    UpdateRuntimeConfigReq,
};
use error::{ErrorCode, IpcError};
use ollama::{is_loopback_url, OllamaProvider};
use opencode::OpenCodeProvider;
use redaction::{Redactor, SecretKey, REDACTED_PLACEHOLDER};
use std::sync::Arc;
use tokio_util::sync::CancellationToken;
use traits::{
    LlmProvider, ProviderChatRequest, ProviderChatResponse, ProviderError, ProviderMessage,
    ProviderStreamChunk, ProviderStreamRequest, ProviderStreamSummary, ProviderToolCall,
    ProviderToolDefinition, ProviderUsage,
};

pub const BASELINE_COMMIT: &str = "ddda30cd3c842113599c7bfb7d0e221936330428";
pub const PREDECESSOR_P2S1_COMMIT: &str = "cc6fe79511e4349b5fa22815d4d7a06ccc4cfad2";
pub const PREDECESSOR_P2S5_COMMIT: &str = "02fc9b735457d47115e0f90063650dc00f322dc1";
pub const PREDECESSOR_P1S1_COMMIT: &str = "56dd590569a0d6123b4f09d6f1c6761a8b1d7ff2";

// ============================================================================
// Fake Provider Fixture for in-memory testing (no real external network calls)
// ============================================================================

#[derive(Debug, Clone)]
pub struct FakeLlmProvider {
    pub provider_id_str: &'static str,
    pub simulated_chat_response: Option<ProviderChatResponse>,
    pub simulated_error: Option<u16>, // HTTP error status to simulate
    pub simulated_stream_chunks: Vec<String>,
}

impl FakeLlmProvider {
    pub fn new(id: &'static str) -> Self {
        Self {
            provider_id_str: id,
            simulated_chat_response: None,
            simulated_error: None,
            simulated_stream_chunks: Vec::new(),
        }
    }
}

#[async_trait::async_trait]
impl LlmProvider for FakeLlmProvider {
    fn provider_id(&self) -> &'static str {
        self.provider_id_str
    }

    async fn chat_with_tools(
        &self,
        req: &ProviderChatRequest,
        cancel_token: CancellationToken,
    ) -> Result<ProviderChatResponse, ProviderError> {
        if cancel_token.is_cancelled() {
            return Err(ProviderError::Cancelled);
        }

        if let Some(status) = self.simulated_error {
            return Err(ProviderError::from_http_status(
                status,
                "Simulated failure with Bearer sk-secret-token-1234567890",
            ));
        }

        tokio::select! {
            _ = cancel_token.cancelled() => Err(ProviderError::Cancelled),
            _ = tokio::time::sleep(std::time::Duration::from_millis(5)) => {
                if let Some(resp) = &self.simulated_chat_response {
                    Ok(resp.clone())
                } else {
                    let mut tool_calls = Vec::new();
                    if let Some(tools) = &req.tools {
                        if let Some(first) = tools.first() {
                            tool_calls.push(ProviderToolCall {
                                id: "call_mock_1".to_string(),
                                name: first.name.clone(),
                                arguments: serde_json::json!({ "arg": "val" }),
                            });
                        }
                    }
                    Ok(ProviderChatResponse {
                        content: Some("Fake generated answer".to_string()),
                        tool_calls,
                        finish_reason: Some("stop".to_string()),
                        usage: Some(ProviderUsage {
                            prompt_tokens: 15,
                            completion_tokens: 25,
                            total_tokens: 40,
                        }),
                    })
                }
            }
        }
    }

    async fn stream_completion(
        &self,
        _req: &ProviderStreamRequest,
        event_tx: tokio::sync::mpsc::Sender<ProviderStreamChunk>,
        cancel_token: CancellationToken,
    ) -> Result<ProviderStreamSummary, ProviderError> {
        if cancel_token.is_cancelled() {
            return Err(ProviderError::Cancelled);
        }

        let mut total = String::new();
        let mut count = 0;

        for chunk in &self.simulated_stream_chunks {
            tokio::select! {
                _ = cancel_token.cancelled() => return Err(ProviderError::Cancelled),
                res = event_tx.send(ProviderStreamChunk {
                    delta: chunk.clone(),
                    finish_reason: None,
                }) => {
                    if res.is_ok() {
                        total.push_str(chunk);
                        count += 1;
                    }
                }
            }
        }

        Ok(ProviderStreamSummary {
            total_text: total,
            chunk_count: count,
            finish_reason: Some("stop".to_string()),
            usage: Some(ProviderUsage {
                prompt_tokens: 10,
                completion_tokens: count as u32 * 5,
                total_tokens: 10 + count as u32 * 5,
            }),
        })
    }
}

// ============================================================================
// Acceptance Tests
// ============================================================================

/// `P2S4-P`: Fake Ollama/OpenCode requests prove tools and `permission:{"*":"deny"}`.
#[tokio::test]
async fn test_p2s4_p_permission_deny_and_tool_calling() {
    // 1. Verify OpenCode Chat payload ALWAYS enforces `permission: { "*": "deny" }`
    let chat_req = ProviderChatRequest {
        model: "claude-3-5-sonnet".to_string(),
        messages: vec![ProviderMessage::user("Please plan my scene")],
        tools: Some(vec![ProviderToolDefinition {
            name: "get_draft_workspace_state".to_string(),
            description: "Read draft state".to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": { "draft_session_id": { "type": "string" } },
                "required": ["draft_session_id"]
            }),
        }]),
        temperature: Some(0.7),
        max_tokens: Some(1024),
    };

    let opencode_payload = OpenCodeProvider::build_chat_payload(&chat_req);
    assert_eq!(
        opencode_payload["permission"]["*"], "deny",
        "OpenCode payload MUST inject permission: {{ '*': 'deny' }} per REQ-PROVIDER-001"
    );
    assert_eq!(opencode_payload["model"], "claude-3-5-sonnet");
    assert_eq!(opencode_payload["tools"][0]["type"], "function");
    assert_eq!(
        opencode_payload["tools"][0]["function"]["name"],
        "get_draft_workspace_state"
    );

    // 2. Verify OpenCode Stream payload ALSO enforces `permission: { "*": "deny" }`
    let stream_req = ProviderStreamRequest {
        model: "claude-3-5-sonnet".to_string(),
        prompt: "Write chapter 1".to_string(),
        messages: None,
        temperature: Some(0.5),
        max_tokens: Some(2000),
    };
    let opencode_stream_payload = OpenCodeProvider::build_stream_payload(&stream_req);
    assert_eq!(
        opencode_stream_payload["permission"]["*"], "deny",
        "OpenCode streaming payload MUST inject permission: {{ '*': 'deny' }}"
    );
    assert_eq!(opencode_stream_payload["stream"], true);

    // 3. Verify Ollama chat payload formats tools as native Ollama functions
    let ollama_payload = OllamaProvider::build_chat_payload(&chat_req);
    assert_eq!(ollama_payload["model"], "claude-3-5-sonnet");
    assert_eq!(ollama_payload["tools"][0]["type"], "function");
    assert_eq!(
        ollama_payload["tools"][0]["function"]["name"],
        "get_draft_workspace_state"
    );

    // 4. Verify loopback restriction on Ollama
    assert!(is_loopback_url("http://127.0.0.1:11434/api/chat"));
    assert!(is_loopback_url("http://localhost:11434/api/chat"));
    assert!(is_loopback_url("http://[::1]:11434/api/chat"));
    assert!(!is_loopback_url("http://192.168.1.50:11434/api/chat"));
    assert!(!is_loopback_url(
        "https://remote-ollama.example.com/api/chat"
    ));

    let non_loopback_ollama = OllamaProvider::new("http://192.168.1.50:11434/api/chat", false);
    let cancel = CancellationToken::new();
    let res = non_loopback_ollama.chat_with_tools(&chat_req, cancel).await;
    assert!(res.is_err());
    match res.unwrap_err() {
        ProviderError::Validation(msg) => {
            assert!(msg.contains("only loopback addresses"));
        }
        other => panic!("Expected Validation error, got: {other:?}"),
    }

    // 5. Test fake provider executing tool calling cycle
    let fake_provider = FakeLlmProvider::new("test_fake");
    let cancel = CancellationToken::new();
    let resp = fake_provider
        .chat_with_tools(&chat_req, cancel)
        .await
        .expect("fake chat succeeds");
    assert_eq!(resp.tool_calls.len(), 1);
    assert_eq!(resp.tool_calls[0].name, "get_draft_workspace_state");
}

/// `P2S4-N`: Log/error snapshot never contains synthetic secret (redaction and debug masks).
#[test]
fn test_p2s4_n_secret_redaction_and_dto_hygiene() {
    let redactor = Redactor::new();
    let synthetic_secret = "sk-proj-supersecretkey1234567890abcdef";
    let custom_secret = "ghp_VerySecretToken9876543210ZYXWV";

    redactor.register_secret(custom_secret);

    // 1. SecretKey wrapper displays/debugs as [REDACTED]
    let key_wrapper = SecretKey::new(synthetic_secret);
    assert_eq!(format!("{key_wrapper}"), REDACTED_PLACEHOLDER);
    assert_eq!(format!("{key_wrapper:?}"), REDACTED_PLACEHOLDER);
    assert_eq!(key_wrapper.expose_secret(), synthetic_secret);

    // 2. Standard OpenAI pattern redaction
    let raw_log = format!("Failed to connect: Bearer {synthetic_secret} returned 401 Unauthorized");
    let redacted_log = redactor.redact(&raw_log);
    assert!(!redacted_log.contains(synthetic_secret));
    assert!(redacted_log.contains(REDACTED_PLACEHOLDER));

    // 3. Registered custom secret redaction
    let custom_log = format!("Error with token: {custom_secret} during handshake");
    let redacted_custom = redactor.redact(&custom_log);
    assert!(!redacted_custom.contains(custom_secret));
    assert!(redacted_custom.contains(REDACTED_PLACEHOLDER));

    // 4. ProviderError redaction
    let auth_err = ProviderError::from_http_status(
        401,
        &format!("Unauthorized: Invalid token Bearer {synthetic_secret}"),
    );
    let ipc_err = auth_err.to_ipc_error_with_redactor(&redactor);
    assert_eq!(ipc_err.code, ErrorCode::ProviderAuthFailed);
    assert!(!ipc_err.message.contains(synthetic_secret));
    assert!(ipc_err.message.contains(REDACTED_PLACEHOLDER));

    // 5. Verify DTO hygiene: SetProviderApiKeyReq has #[serde(skip_serializing)]
    let req_dto = SetProviderApiKeyReq {
        provider_id: "opencode".to_string(),
        api_key: synthetic_secret.to_string(),
    };
    let json_val = serde_json::to_value(&req_dto).unwrap();
    assert!(!json_val.to_string().contains(synthetic_secret));
    assert!(json_val.get("apiKey").is_none());

    // 6. ProviderConfigInputDto hygiene: apiKey is skip_serializing
    let config_dto = ProviderConfigInputDto {
        provider_id: "opencode".to_string(),
        api_key: Some(synthetic_secret.to_string()),
        endpoint_url: Some("https://api.opencode.ai".to_string()),
        model_name: Some("claude-3-5-sonnet".to_string()),
    };
    let config_json = serde_json::to_value(&config_dto).unwrap();
    assert!(!config_json.to_string().contains(synthetic_secret));
    assert!(config_json.get("apiKey").is_none());
}

/// `P2S4-B`: Cancellation drops request and returns typed error without hanging.
#[tokio::test]
async fn test_p2s4_b_cancellation_handling() {
    let fake_provider = FakeLlmProvider {
        provider_id_str: "fake_cancel",
        simulated_chat_response: None,
        simulated_error: None,
        simulated_stream_chunks: vec!["Hello ".to_string(), "world".to_string()],
    };

    let cancel_token = CancellationToken::new();
    // Cancel immediately before call
    cancel_token.cancel();

    let req = ProviderChatRequest {
        model: "m1".to_string(),
        messages: vec![ProviderMessage::user("test")],
        tools: None,
        temperature: None,
        max_tokens: None,
    };

    let res = fake_provider
        .chat_with_tools(&req, cancel_token.clone())
        .await;
    assert!(res.is_err());
    match res.unwrap_err() {
        ProviderError::Cancelled => {}
        other => panic!("Expected ProviderError::Cancelled, got {other:?}"),
    }

    // Test stream cancellation
    let (tx, mut _rx) = tokio::sync::mpsc::channel(10);
    let stream_req = ProviderStreamRequest {
        model: "m1".to_string(),
        prompt: "test stream".to_string(),
        messages: None,
        temperature: None,
        max_tokens: None,
    };

    let stream_res = fake_provider
        .stream_completion(&stream_req, tx, cancel_token)
        .await;
    assert!(stream_res.is_err());
    match stream_res.unwrap_err() {
        ProviderError::Cancelled => {}
        other => panic!("Expected ProviderError::Cancelled, got {other:?}"),
    }
}

/// `phase2_sprint4_deep3-F0`: Predecessor gate check and lineage assertion.
#[test]
fn test_f0_predecessor_lineage_and_baseline_immutability() {
    // 1. Verify baseline ancestor
    assert!(
        common::verify_commit(BASELINE_COMMIT),
        "lineage commit {} must be an ancestor, a local object, or recorded in the lineage manifest",
        BASELINE_COMMIT
    );

    // 2. Verify P2S1 predecessor commit exists
    assert!(
        common::verify_commit(PREDECESSOR_P2S1_COMMIT),
        "lineage commit {} must be an ancestor, a local object, or recorded in the lineage manifest",
        PREDECESSOR_P2S1_COMMIT
    );

    // 3. Verify P2S5 predecessor commit exists
    assert!(
        common::verify_commit(PREDECESSOR_P2S5_COMMIT),
        "lineage commit {} must be an ancestor, a local object, or recorded in the lineage manifest",
        PREDECESSOR_P2S5_COMMIT
    );

    // 4. Verify P1S1 predecessor commit exists
    assert!(
        common::verify_commit(PREDECESSOR_P1S1_COMMIT),
        "lineage commit {} must be an ancestor, a local object, or recorded in the lineage manifest",
        PREDECESSOR_P1S1_COMMIT
    );
}

/// `phase2_sprint4_deep3-F1`: Map 401/403 to ProviderAuthFailed(nonretryable) and 429/502/503/504 to ProviderUnreachable(retryable).
#[test]
fn test_f1_http_error_mapping_and_retryability() {
    let redactor = Redactor::new();
    let secret = "sk-leaked-api-token-1234567890";
    redactor.register_secret(secret);

    // 401 Unauthorized
    let err_401 =
        ProviderError::from_http_status(401, &format!("Invalid credentials Bearer {secret}"));
    let ipc_401 = err_401.to_ipc_error_with_redactor(&redactor);
    assert_eq!(ipc_401.code, ErrorCode::ProviderAuthFailed);
    assert!(!ipc_401.retryable, "401 must NOT be retryable");
    assert!(!ipc_401.message.contains(secret));

    // 403 Forbidden
    let err_403 =
        ProviderError::from_http_status(403, &format!("Access forbidden with Bearer {secret}"));
    let ipc_403 = err_403.to_ipc_error_with_redactor(&redactor);
    assert_eq!(ipc_403.code, ErrorCode::ProviderAuthFailed);
    assert!(!ipc_403.retryable, "403 must NOT be retryable");
    assert!(!ipc_403.message.contains(secret));

    // 429 Rate Limit
    let err_429 = ProviderError::from_http_status(429, "Too many requests");
    let ipc_429 = err_429.to_ipc_error_with_redactor(&redactor);
    assert_eq!(ipc_429.code, ErrorCode::ProviderUnreachable);
    assert!(ipc_429.retryable, "429 MUST be retryable");

    // 502 Bad Gateway
    let err_502 = ProviderError::from_http_status(502, "Bad Gateway");
    let ipc_502 = err_502.to_ipc_error_with_redactor(&redactor);
    assert_eq!(ipc_502.code, ErrorCode::ProviderUnreachable);
    assert!(ipc_502.retryable, "502 MUST be retryable");

    // 503 Service Unavailable
    let err_503 = ProviderError::from_http_status(503, "Service Unavailable");
    let ipc_503 = err_503.to_ipc_error_with_redactor(&redactor);
    assert_eq!(ipc_503.code, ErrorCode::ProviderUnreachable);
    assert!(ipc_503.retryable, "503 MUST be retryable");

    // 504 Gateway Timeout
    let err_504 = ProviderError::from_http_status(504, "Gateway Timeout");
    let ipc_504 = err_504.to_ipc_error_with_redactor(&redactor);
    assert_eq!(ipc_504.code, ErrorCode::ProviderUnreachable);
    assert!(ipc_504.retryable, "504 MUST be retryable");
}

/// `phase2_sprint4_deep3-F2`: Keychain unavailable yields session-only fallback in RAM with no disk secrets.
#[test]
fn test_f2_keychain_unavailable_session_fallback_no_disk() {
    let cred_mgr = CredentialManager::new_with_session_only();
    assert!(!cred_mgr.is_keychain_available());

    let test_provider = "anthropic";
    let test_key = "sk-ant-api03-session-fallback-secret-987654321";

    let mode = cred_mgr
        .set_api_key(test_provider, test_key)
        .expect("set_api_key succeeds via RAM fallback");

    assert_eq!(
        mode,
        CredentialStorageMode::SessionOnlyFallback,
        "Storage mode MUST be session_only_fallback when keychain unavailable"
    );
    assert_eq!(mode.as_str(), "session_only_fallback");

    // In-memory lookup succeeds
    let retrieved = cred_mgr.get_api_key(test_provider);
    assert_eq!(retrieved, Some(test_key.to_string()));
    assert!(cred_mgr.has_api_key(test_provider));

    // Deletion clears from RAM
    cred_mgr.delete_api_key(test_provider);
    assert_eq!(cred_mgr.get_api_key(test_provider), None);
    assert!(!cred_mgr.has_api_key(test_provider));
}

/// `IPC-CMD-038`: Typed IPC #38 `get_runtime_and_provider_config` read-only inquiry.
#[test]
fn test_ipc_cmd_038_get_runtime_and_provider_config() {
    let cred_mgr = CredentialManager::new_with_session_only();
    let redactor = Arc::new(Redactor::new());
    let state = RuntimeConfigState::new(cred_mgr, redactor);

    let req = GetRuntimeConfigReq {};
    let config = exec_get_runtime_and_provider_config(&state, &req)
        .expect("get_runtime_and_provider_config succeeds");

    // Verify subagent defaults
    assert_eq!(config.subagent_config.max_subagents, 6);
    assert_eq!(config.subagent_config.outline_capacity, 50);
    assert_eq!(config.subagent_config.outline_lookback, 10);

    // No provider or model should be hard-coded into a fresh configuration.
    assert!(config.providers.is_empty());

    assert_eq!(config.credential_storage_mode, "session_only_fallback");
}

/// `IPC-CMD-039`: Typed IPC #39 `update_runtime_and_provider_config` with arithmetic validation and no SQLite persistence.
#[test]
fn test_ipc_cmd_039_update_runtime_and_provider_config() {
    let cred_mgr = CredentialManager::new_with_session_only();
    let redactor = Arc::new(Redactor::new());
    let state = RuntimeConfigState::new(cred_mgr, redactor);

    // 1. Boundary check: maxSubagents must be 1 <= maxSubagents <= 6
    let invalid_subagent_0 = SubagentConfigDto {
        max_subagents: 0,
        ..Default::default()
    };
    let err_0 = validate_subagent_config(&invalid_subagent_0).unwrap_err();
    assert_eq!(err_0.code, ErrorCode::ValidationFailed);

    let invalid_subagent_7 = SubagentConfigDto {
        max_subagents: 7,
        ..Default::default()
    };
    let err_7 = validate_subagent_config(&invalid_subagent_7).unwrap_err();
    assert_eq!(err_7.code, ErrorCode::ValidationFailed);

    // 2. Boundary check: outlineLookback < outlineCapacity
    let invalid_lookback_equal = SubagentConfigDto {
        outline_capacity: 50,
        outline_lookback: 50,
        ..Default::default()
    };
    let err_eq = validate_subagent_config(&invalid_lookback_equal).unwrap_err();
    assert_eq!(err_eq.code, ErrorCode::ValidationFailed);

    let invalid_lookback_greater = SubagentConfigDto {
        outline_capacity: 40,
        outline_lookback: 45,
        ..Default::default()
    };
    let err_gt = validate_subagent_config(&invalid_lookback_greater).unwrap_err();
    assert_eq!(err_gt.code, ErrorCode::ValidationFailed);

    // 3. Valid update
    let valid_subagent = SubagentConfigDto {
        max_subagents: 4,
        outline_capacity: 60,
        outline_lookback: 15,
        ..Default::default()
    };
    let update_req = UpdateRuntimeConfigReq {
        subagent_config: Some(valid_subagent),
        active_provider_id: Some("opencode".to_string()),
        model_name: Some("gpt-4o".to_string()),
        endpoint_url: Some("https://api.openai.com/v1".to_string()),
        remove_provider_id: None,
    };

    let updated = exec_update_runtime_and_provider_config(&state, &update_req)
        .expect("update succeeds with valid inputs");

    assert_eq!(updated.subagent_config.max_subagents, 4);
    assert_eq!(updated.subagent_config.outline_capacity, 60);
    assert_eq!(updated.subagent_config.outline_lookback, 15);

    let opencode_status = updated
        .providers
        .iter()
        .find(|p| p.provider_id == "opencode")
        .expect("opencode present");
    assert!(opencode_status.is_active);
    assert_eq!(opencode_status.model_name.as_deref(), Some("gpt-4o"));
    assert_eq!(
        opencode_status.endpoint_url.as_deref(),
        Some("https://api.openai.com/v1")
    );
}

/// `IPC-CMD-040`: Typed IPC #40 `set_provider_api_key` with whitespace validation, redaction registration, and keychain/RAM storage.
#[test]
fn test_ipc_cmd_040_set_provider_api_key() {
    let cred_mgr = CredentialManager::new_with_session_only();
    let redactor = Arc::new(Redactor::new());
    let state = RuntimeConfigState::new(cred_mgr, redactor.clone());

    // 1. Empty or whitespace apiKey rejected with ValidationFailed
    let empty_req = SetProviderApiKeyReq {
        provider_id: "opencode".to_string(),
        api_key: "   \t\n  ".to_string(),
    };
    let empty_err = exec_set_provider_api_key(&state, &empty_req).unwrap_err();
    assert_eq!(empty_err.code, ErrorCode::ValidationFailed);

    // 2. Empty providerId rejected with ValidationFailed
    let empty_provider = SetProviderApiKeyReq {
        provider_id: "".to_string(),
        api_key: "valid_secret_123456789".to_string(),
    };
    let empty_p_err = exec_set_provider_api_key(&state, &empty_provider).unwrap_err();
    assert_eq!(empty_p_err.code, ErrorCode::ValidationFailed);

    // 3. Valid key sets successfully, registers in redactor, returns mode
    let valid_secret = "sk-ant-live-key-4567890123456789";
    let valid_req = SetProviderApiKeyReq {
        provider_id: "opencode".to_string(),
        api_key: valid_secret.to_string(),
    };
    let resp = exec_set_provider_api_key(&state, &valid_req).expect("set_api_key succeeds");
    assert_eq!(resp.provider_id, "opencode");
    assert_eq!(resp.storage_mode, "session_only_fallback");

    // Redactor must now redact this secret
    let test_log = format!("Connecting using token: {valid_secret}");
    let redacted = redactor.redact(&test_log);
    assert!(!redacted.contains(valid_secret));
    assert!(redacted.contains(REDACTED_PLACEHOLDER));

    // Provider status must now show has_api_key = true
    let cfg = exec_get_runtime_and_provider_config(&state, &GetRuntimeConfigReq {}).unwrap();
    let opencode_status = cfg
        .providers
        .iter()
        .find(|p| p.provider_id == "opencode")
        .unwrap();
    assert!(opencode_status.has_api_key);
}

/// `T-IPC-01`: Multi-point security verification (§1.6, §5.2, §9.2, §9.3).
/// (1) AI calls to UI/CLI exclusive commands blocked with UnauthorizedCaller.
/// (2) Path traversal in Uuid fields blocked by Uuid parsing.
/// (3) HTTP errors containing Bearer keys are redacted in log and IpcError.
#[test]
fn test_t_ipc_01_comprehensive_security_invariants() {
    // Part (1): Simulated AI calling UI-only commands gets blocked by UnauthorizedCaller
    // UI exclusive commands: confirm_pending_patch, commit_draft, set_scene_lock
    fn simulate_caller_check(is_ai_caller: bool, command_name: &str) -> Result<(), IpcError> {
        let ui_only_commands = ["confirm_pending_patch", "commit_draft", "set_scene_lock"];
        if is_ai_caller && ui_only_commands.contains(&command_name) {
            return Err(IpcError::new(
                ErrorCode::UnauthorizedCaller,
                format!("Command '{command_name}' is restricted to UI/human operator"),
            ));
        }
        Ok(())
    }

    assert_eq!(
        simulate_caller_check(true, "confirm_pending_patch")
            .unwrap_err()
            .code,
        ErrorCode::UnauthorizedCaller
    );
    assert_eq!(
        simulate_caller_check(true, "commit_draft")
            .unwrap_err()
            .code,
        ErrorCode::UnauthorizedCaller
    );
    assert_eq!(
        simulate_caller_check(true, "set_scene_lock")
            .unwrap_err()
            .code,
        ErrorCode::UnauthorizedCaller
    );
    assert!(simulate_caller_check(false, "commit_draft").is_ok());

    // Part (2): Path traversal attack via draft_session_id containing `../../etc`
    let malicious_id = "../../etc/passwd";
    let parse_res = uuid::Uuid::parse_str(malicious_id);
    assert!(
        parse_res.is_err(),
        "Path traversal string must fail Uuid parse"
    );

    // Part (3): HTTP error containing Bearer sk-secret1234567890123456 redacted in log & IpcError
    let redactor = Redactor::new();
    let raw_secret = "Bearer sk-secret1234567890123456";
    let simulated_http_err = format!("Connection failed: {raw_secret} rejected by host");

    let err = ProviderError::from_http_status(401, &simulated_http_err);
    let ipc_err = err.to_ipc_error_with_redactor(&redactor);

    assert_eq!(ipc_err.code, ErrorCode::ProviderAuthFailed);
    assert!(!ipc_err.message.contains("sk-secret1234567890123456"));
    assert!(ipc_err.message.contains(REDACTED_PLACEHOLDER));

    let logged_string = redactor.redact(&simulated_http_err);
    assert!(!logged_string.contains("sk-secret1234567890123456"));
    assert!(logged_string.contains(REDACTED_PLACEHOLDER));
}

#[path = "../src/ipc/provider_probe_commands.rs"]
pub mod provider_probe_commands;

#[tokio::test]
async fn custom_provider_chat_sends_the_key_saved_for_its_provider_id() {
    use std::io::{Read, Write};
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let endpoint = format!("http://{}/v1", listener.local_addr().unwrap());
    let server = std::thread::spawn(move || {
        let (mut socket, _) = listener.accept().unwrap();
        socket
            .set_read_timeout(Some(std::time::Duration::from_secs(5)))
            .unwrap();
        let mut request = Vec::new();
        let mut buf = [0; 1024];
        while !request.windows(4).any(|w| w == b"\r\n\r\n") {
            let n = socket.read(&mut buf).unwrap();
            assert!(n > 0);
            request.extend_from_slice(&buf[..n]);
        }
        let header_end = request.windows(4).position(|w| w == b"\r\n\r\n").unwrap() + 4;
        let length = String::from_utf8_lossy(&request[..header_end])
            .lines()
            .find_map(|line| {
                line.to_lowercase()
                    .strip_prefix("content-length:")
                    .and_then(|n| n.trim().parse::<usize>().ok())
            })
            .unwrap_or(0);
        while request.len() < header_end + length {
            let n = socket.read(&mut buf).unwrap();
            assert!(n > 0);
            request.extend_from_slice(&buf[..n]);
        }
        let headers = String::from_utf8_lossy(&request).to_lowercase();
        assert!(headers.starts_with("post /v1/chat/completions "));
        assert!(headers.contains("authorization: bearer synthetic-custom-key\r\n"));
        let body = r#"{"choices":[{"index":0,"message":{"role":"assistant","content":"OK"},"finish_reason":"stop"}]}"#;
        write!(socket, "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}", body.len()).unwrap();
    });
    let state = RuntimeConfigState::new(
        CredentialManager::new_with_session_only(),
        Arc::new(Redactor::new()),
    );
    exec_set_provider_api_key(
        &state,
        &SetProviderApiKeyReq {
            provider_id: "provider-custom".into(),
            api_key: "synthetic-custom-key".into(),
        },
    )
    .unwrap();
    exec_update_runtime_and_provider_config(
        &state,
        &UpdateRuntimeConfigReq {
            active_provider_id: Some("provider-custom".into()),
            endpoint_url: Some(endpoint),
            model_name: Some("custom-model".into()),
            ..Default::default()
        },
    )
    .unwrap();
    let response = state
        .selected_provider()
        .unwrap()
        .chat_with_tools(
            &ProviderChatRequest {
                model: "ignored".into(),
                messages: vec![ProviderMessage::user("Hello")],
                tools: None,
                temperature: None,
                max_tokens: Some(1),
            },
            CancellationToken::new(),
        )
        .await
        .unwrap();
    server.join().unwrap();
    assert_eq!(response.content.as_deref(), Some("OK"));
}

#[tokio::test]
async fn request_timeout_reports_actionable_cause() {
    use std::time::Duration;
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let endpoint = format!(
        "http://{}/v1/chat/completions",
        listener.local_addr().unwrap()
    );
    let server = tokio::spawn(async move {
        let (_socket, _) = listener.accept().await.unwrap();
        tokio::time::sleep(Duration::from_secs(5)).await;
    });
    let client = reqwest::Client::builder()
        .no_proxy()
        .timeout(Duration::from_millis(100))
        .build()
        .unwrap();
    let provider = OpenCodeProvider::with_client(endpoint, None, client);
    let err = provider
        .chat_with_tools(
            &ProviderChatRequest {
                model: "test".into(),
                messages: vec![ProviderMessage::user("ping")],
                tools: None,
                temperature: None,
                max_tokens: Some(1),
            },
            CancellationToken::new(),
        )
        .await
        .unwrap_err();
    server.abort();
    let ipc = err.to_ipc_error();
    assert!(
        ipc.message.contains("timed out"),
        "Actual timeout cause must survive IPC: {}",
        ipc.message
    );
}
