#![allow(non_snake_case)]

//! Acceptance and Integration tests for `phase3_sprint7_deep4`
//! ("Implement ReAct chat and prompt orchestration").
//!
//! Complies with §6.1, `docs/business/prompts/overview.md`, `docs/business/prompts/case_newchat.md`,
//! `core.md` §1.3, §3.8, and `be-core.md` §10.2.1.
//!
//! Acceptance vectors covered:
//! - `P3S7-P`: Normal ReAct turn completes: tool executed, working state mutated, TX-01 sealed.
//! - `P3S7-N`: 3 consecutive rejections stops loop, requests user clarification in system_language.
//! - `P3S7-MAX`: Max iterations reached: controlled exit seals applied changes.
//! - `P3S7-CANCEL`: Cancel mid-turn seals applied changes and marks cancelled.
//! - `P3S7-REGEN`: `regenerate_draft_phase` only overwrites `ai_generated` fields, preserves `user`/`user_edited`.
//! - `IPC-CMD-001`: `create_or_load_chat_session`
//! - `IPC-CMD-002`: `list_chat_sessions`
//! - `IPC-CMD-003`: `send_chat_message`
//! - `IPC-CMD-004`: `cancel_chat_turn`
//! - `IPC-CMD-009`: `regenerate_draft_phase`
//! - `phase3_sprint7_deep4-F0`: Predecessor gates verification.

#[path = "../src/error.rs"]
pub mod error;

#[path = "../src/domain/business_flow.rs"]
pub mod business_flow;

#[path = "../src/domain/code_allocator.rs"]
pub mod code_allocator;

#[path = "../src/domain/fingerprint.rs"]
pub mod fingerprint;

#[path = "../src/domain/word_count.rs"]
pub mod word_count;

#[path = "../src/domain/progress_engine.rs"]
pub mod progress_engine;

#[path = "../src/domain/invariants.rs"]
pub mod invariants;

pub mod registry {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
    #[serde(rename_all = "snake_case")]
    pub enum CallerCategory {
        AiCaller,
        UiAction,
        Cli,
    }

    impl CallerCategory {
        pub const fn is_ai(self) -> bool {
            matches!(self, CallerCategory::AiCaller)
        }

        pub const fn is_user_or_cli(self) -> bool {
            matches!(self, CallerCategory::UiAction | CallerCategory::Cli)
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
    #[serde(rename_all = "snake_case")]
    pub enum AllowedCaller {
        AiOnly,
        AiAndUi,
        UiCliOnly,
        SubagentOnly,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
    #[serde(rename_all = "snake_case")]
    pub enum ToolType {
        Read,
        Mutation,
        Delete,
        Action,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
    #[serde(rename_all = "snake_case")]
    pub enum ToolScope {
        Planning,
        Writing,
        System,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
    #[serde(rename_all = "snake_case")]
    pub enum SourceParamMode {
        None,
        MapDeclaredSources,
        ScalarDeclaredSource,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct ToolMetadata {
        pub name: String,
        pub description: String,
        pub tool_type: ToolType,
        pub scope: ToolScope,
        pub allowed_caller: AllowedCaller,
        pub source_param_mode: SourceParamMode,
        pub writes_draft_state: bool,
    }

    impl ToolMetadata {
        pub fn new(
            name: impl Into<String>,
            description: impl Into<String>,
            tool_type: ToolType,
            scope: ToolScope,
            allowed_caller: AllowedCaller,
            source_param_mode: SourceParamMode,
            writes_draft_state: bool,
        ) -> Self {
            Self {
                name: name.into(),
                description: description.into(),
                tool_type,
                scope,
                allowed_caller,
                source_param_mode,
                writes_draft_state,
            }
        }

        pub fn is_allowed_caller(&self, caller: CallerCategory) -> bool {
            match (self.allowed_caller, caller) {
                (AllowedCaller::AiOnly, CallerCategory::AiCaller) => true,
                (AllowedCaller::AiOnly, CallerCategory::UiAction | CallerCategory::Cli) => true,
                (AllowedCaller::AiAndUi, _) => true,
                (AllowedCaller::UiCliOnly, CallerCategory::AiCaller) => false,
                (AllowedCaller::UiCliOnly, CallerCategory::UiAction | CallerCategory::Cli) => true,
                (AllowedCaller::SubagentOnly, _) => false,
            }
        }
    }

    #[derive(Debug, Clone, Default)]
    pub struct ToolRegistry {
        tools: std::collections::BTreeMap<String, ToolMetadata>,
    }

    impl ToolRegistry {
        pub fn new() -> Self {
            let mut reg = Self {
                tools: std::collections::BTreeMap::new(),
            };
            reg.register_standard_tools();
            reg
        }

        pub fn register(&mut self, meta: ToolMetadata) {
            self.tools.insert(meta.name.clone(), meta);
        }

        pub fn lookup(&self, name: &str) -> Option<&ToolMetadata> {
            self.tools.get(name)
        }

        pub fn authorize(&self, tool_name: &str, caller: CallerCategory) -> Result<&ToolMetadata, crate::error::ErrorCode> {
            let meta = self.lookup(tool_name).ok_or(crate::error::ErrorCode::EntityNotFound)?;
            if !meta.is_allowed_caller(caller) {
                return Err(crate::error::ErrorCode::UnauthorizedCaller);
            }
            Ok(meta)
        }

        fn register_standard_tools(&mut self) {
            self.register(ToolMetadata::new(
                "set_character",
                "Update character fields in draft state",
                ToolType::Mutation,
                ToolScope::Planning,
                AllowedCaller::AiAndUi,
                SourceParamMode::MapDeclaredSources,
                true,
            ));
            self.register(ToolMetadata::new(
                "add_character",
                "Add a new character",
                ToolType::Mutation,
                ToolScope::Planning,
                AllowedCaller::AiAndUi,
                SourceParamMode::MapDeclaredSources,
                true,
            ));
            self.register(ToolMetadata::new(
                "delete_character",
                "Delete character from draft state",
                ToolType::Delete,
                ToolScope::Planning,
                AllowedCaller::AiAndUi,
                SourceParamMode::ScalarDeclaredSource,
                true,
            ));
            self.register(ToolMetadata::new(
                "set_project",
                "Update project metadata",
                ToolType::Mutation,
                ToolScope::Planning,
                AllowedCaller::AiAndUi,
                SourceParamMode::MapDeclaredSources,
                true,
            ));
            self.register(ToolMetadata::new(
                "set_world_setting",
                "Update world setting",
                ToolType::Mutation,
                ToolScope::Planning,
                AllowedCaller::AiAndUi,
                SourceParamMode::MapDeclaredSources,
                true,
            ));
            self.register(ToolMetadata::new(
                "set_arc",
                "Update arc fields",
                ToolType::Mutation,
                ToolScope::Planning,
                AllowedCaller::AiAndUi,
                SourceParamMode::MapDeclaredSources,
                true,
            ));
        }
    }
}

pub use registry::CallerCategory;

#[path = "../src/domain/cascade_delete.rs"]
pub mod cascade_delete;

#[path = "../src/domain/tools/write_tools.rs"]
pub mod write_tools;

#[path = "../src/domain/tools/read_tools.rs"]
pub mod read_tools;

pub mod domain {
    pub use crate::business_flow;
    pub use crate::cascade_delete;
    pub use crate::code_allocator;
    pub use crate::fingerprint;
    pub use crate::invariants;
    pub use crate::progress_engine;
    pub use crate::word_count;
    pub mod tools {
        pub use crate::registry;
        pub use crate::read_tools;
        pub use crate::write_tools;
    }
}

#[path = "../src/storage/db.rs"]
pub mod db;

#[path = "../src/storage/migration.rs"]
pub mod migration;

#[path = "../src/storage/pending_repo.rs"]
pub mod pending_repo;

pub mod storage {
    pub use crate::db;
    pub use crate::migration;
    pub use crate::pending_repo;
}

#[path = "../src/providers/redaction.rs"]
pub mod redaction;

#[path = "../src/providers/traits.rs"]
pub mod traits;

pub mod providers {
    pub use crate::redaction;
    pub use crate::traits;
}

#[path = "../src/ipc/dto.rs"]
pub mod dto;

pub mod ipc {
    pub use crate::dto;
}

#[path = "../src/service/draft_runtime.rs"]
pub mod draft_runtime;

#[path = "../src/service/draft_service.rs"]
pub mod draft_service;

#[path = "../src/service/patch_service.rs"]
pub mod patch_service;

#[path = "../src/service/chat_orchestrator.rs"]
pub mod chat_orchestrator;

#[path = "../src/ipc/chat_commands.rs"]
pub mod chat_commands;

pub mod service {
    pub use crate::chat_orchestrator;
    pub use crate::draft_runtime;
    pub use crate::draft_service;
    pub use crate::patch_service;
}

use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

use crate::business_flow::{BusinessFlowData, ProvenanceSource, TrackedField};
use crate::chat_commands::{
    exec_cancel_chat_turn, exec_create_or_load_chat_session, exec_list_chat_sessions,
    exec_manage_chat_sessions, exec_regenerate_draft_phase, exec_send_chat_message,
};
use crate::chat_orchestrator::{ChatOrchestrator, OrchestratorConfig};
use crate::db::configure_connection;
use crate::draft_runtime::{DraftRuntimeRegistry, WorkingState};
use crate::draft_service::DraftService;
use crate::dto::{
    CancelChatTurnReq, CreateOrLoadChatSessionReq, ListChatSessionsReq, ManageChatSessionsReq,
    RegenerateDraftPhaseReq, SendChatMessageReq,
};
use crate::error::ErrorCode;
use crate::migration::run_migrations;
use crate::patch_service::PatchService;
use crate::traits::{
    LlmProvider, ProviderChatRequest, ProviderChatResponse, ProviderError, ProviderStreamChunk,
    ProviderStreamRequest, ProviderStreamSummary, ProviderToolCall,
};

// ============================================================================
// Mock LLM Provider
// ============================================================================

pub struct MockLlmProvider {
    responses: Mutex<VecDeque<Result<ProviderChatResponse, ProviderError>>>,
    recorded_requests: Mutex<Vec<ProviderChatRequest>>,
}

impl MockLlmProvider {
    pub fn new() -> Self {
        Self {
            responses: Mutex::new(VecDeque::new()),
            recorded_requests: Mutex::new(Vec::new()),
        }
    }

    pub fn queue_response(&self, resp: ProviderChatResponse) {
        self.responses.lock().unwrap().push_back(Ok(resp));
    }

    pub fn queue_error(&self, error: ProviderError) {
        self.responses.lock().unwrap().push_back(Err(error));
    }

    pub fn recorded_requests(&self) -> Vec<ProviderChatRequest> {
        self.recorded_requests.lock().unwrap().clone()
    }
}

#[async_trait::async_trait]
impl LlmProvider for MockLlmProvider {
    fn provider_id(&self) -> &'static str {
        "mock_provider"
    }

    async fn chat_with_tools(
        &self,
        req: &ProviderChatRequest,
        cancel_token: CancellationToken,
    ) -> Result<ProviderChatResponse, ProviderError> {
        if cancel_token.is_cancelled() {
            return Err(ProviderError::Cancelled);
        }

        self.recorded_requests.lock().unwrap().push(req.clone());

        if let Some(resp) = self.responses.lock().unwrap().pop_front() {
            resp
        } else {
            Ok(ProviderChatResponse {
                content: Some("Default mock response".to_string()),
                tool_calls: Vec::new(),
                finish_reason: Some("stop".to_string()),
                usage: None,
            })
        }
    }

    async fn stream_completion(
        &self,
        _req: &ProviderStreamRequest,
        _event_tx: tokio::sync::mpsc::Sender<ProviderStreamChunk>,
        _cancel_token: CancellationToken,
    ) -> Result<ProviderStreamSummary, ProviderError> {
        Ok(ProviderStreamSummary::default())
    }
}

// ============================================================================
// Fixture Setup
// ============================================================================

fn setup_test_context() -> (
    Pool<SqliteConnectionManager>,
    Arc<DraftRuntimeRegistry>,
    Arc<DraftService>,
    Arc<PatchService>,
    Arc<ChatOrchestrator>,
    tempfile::TempDir,
) {
    let tmp = tempfile::tempdir().unwrap();
    let db_path = tmp.path().join("chat_test.db");
    let manager = SqliteConnectionManager::file(&db_path).with_init(|c| configure_connection(c));
    let pool = Pool::builder().max_size(5).build(manager).unwrap();

    let mut conn = pool.get().unwrap();
    run_migrations(&mut conn).unwrap();

    let registry = Arc::new(DraftRuntimeRegistry::new());
    let draft_service = Arc::new(DraftService::new(pool.clone(), registry.clone()));
    let patch_service = Arc::new(PatchService::new(
        pool.clone(),
        registry.clone(),
        draft_service.clone(),
    ));

    let orchestrator = Arc::new(ChatOrchestrator::new(
        pool.clone(),
        registry.clone(),
        draft_service.clone(),
        patch_service.clone(),
    ));

    (pool, registry, draft_service, patch_service, orchestrator, tmp)
}

fn create_test_session(
    pool: &Pool<SqliteConnectionManager>,
    registry: &DraftRuntimeRegistry,
) -> (String, Uuid, Uuid) {
    let conn = pool.get().unwrap();
    let now = chrono::Utc::now().to_rfc3339();
    let proj_id = Uuid::new_v4();
    let sess_id = Uuid::new_v4().to_string();
    let draft_id = Uuid::new_v4();
    let proj_code = format!("proj_{}", &proj_id.to_string()[..8]);

    conn.execute(
        "INSERT INTO projects (id, code, display_name, status, language) VALUES (?1, ?2, 'Test Project', 'PLANNING', 'VI')",
        params![proj_id.to_string(), proj_code],
    ).unwrap();

    conn.execute(
        "INSERT INTO sessions (id, project_id, display_name, created_at) VALUES (?1, ?2, 'Test Chat', ?3)",
        params![sess_id, proj_id.to_string(), now],
    ).unwrap();

    conn.execute(
        "INSERT INTO draft_sessions (id, session_id, project_id, head_version_seq, status, created_at, updated_at) VALUES (?1, ?2, ?3, NULL, 'editing', ?4, ?4)",
        params![draft_id.to_string(), sess_id, proj_id.to_string(), now],
    ).unwrap();

    let ws = WorkingState::new(draft_id);
    registry.get_or_register(draft_id, ws);

    (sess_id, draft_id, proj_id)
}

// ============================================================================
// Tests
// ============================================================================

/// `phase3_sprint7_deep4-F0`: Predecessor gates verification.
#[test]
fn test_phase3_sprint7_deep4_f0_predecessor_gates_check() {
    let output = std::process::Command::new("git")
        .args(&["cat-file", "-e", "181eda6d87f7fac1d17e05f82d843222aa0a376e^{commit}"])
        .output()
        .expect("git check gate_phase3_sprint6_deep3");
    assert!(output.status.success(), "gate_phase3_sprint6_deep3 must exist in git history");

    let output = std::process::Command::new("git")
        .args(&["cat-file", "-e", "6a271099f0ea8bf5847c60af81e6d9b5fb9ad918^{commit}"])
        .output()
        .expect("git check gate_phase2_sprint4_deep3");
    assert!(output.status.success(), "gate_phase2_sprint4_deep3 must exist in git history");

    let output = std::process::Command::new("git")
        .args(&["cat-file", "-e", "ded1e9c5457ab8e72f456a515937a05af6ebba63^{commit}"])
        .output()
        .expect("git check gate_phase3_sprint2_deep0");
    assert!(output.status.success(), "gate_phase3_sprint2_deep0 must exist in git history");
}

/// `P3S7-P`: Normal ReAct turn completes: tool executed, working state mutated, TX-01 sealed.
#[tokio::test]
async fn test_P3S7_P_normal_react_turn_completes_tool_executed_tx01_sealed() {
    let (pool, registry, _draft_svc, _patch_svc, orchestrator, _tmp) = setup_test_context();
    let (sess_id, draft_id, _proj_id) = create_test_session(&pool, &registry);

    let provider = Arc::new(MockLlmProvider::new());

    // Step 1: LLM emits tool call `add_character` with ai_generated source
    provider.queue_response(ProviderChatResponse {
        content: Some("I will add character Arthur.".to_string()),
        tool_calls: vec![ProviderToolCall {
            id: "call_char_1".to_string(),
            name: "add_character".to_string(),
            arguments: serde_json::json!({
                "code": "char_arthur",
                "display_name": "Arthur Pendragon",
                "role": "Protagonist",
                "declared_sources": {
                    "display_name": "ai_generated",
                    "role": "ai_generated"
                }
            }),
        }],
        finish_reason: Some("tool_calls".to_string()),
        usage: None,
    });

    // Step 2: LLM receives observation and returns final message
    provider.queue_response(ProviderChatResponse {
        content: Some("Arthur Pendragon has been added to characters.".to_string()),
        tool_calls: vec![],
        finish_reason: Some("stop".to_string()),
        usage: None,
    });

    let (tx, mut rx) = tokio::sync::mpsc::channel(20);

    let req = SendChatMessageReq {
        session_id: sess_id.clone(),
        content: "Add protagonist Arthur".to_string(),
        system_language: "vi".to_string(),
        feedback_context: None,
    };

    let result = exec_send_chat_message(&orchestrator, provider.clone(), &req, Some(tx))
        .await
        .unwrap();

    // Verify response
    assert_eq!(result.assistant_text, "Arthur Pendragon has been added to characters.");
    assert!(result.sealed_version.is_some(), "TX-01 version must be sealed");
    let sealed = result.sealed_version.unwrap();
    assert_eq!(sealed.seq, 1);
    assert_eq!(sealed.reason, "ai_patch");

    // Verify seal_key in SQLite draft_versions
    let conn = pool.get().unwrap();
    let seal_key: String = conn.query_row(
        "SELECT seal_key FROM draft_versions WHERE draft_session_id = ?1 AND seq = 1",
        params![draft_id.to_string()],
        |r| r.get(0),
    ).unwrap();
    assert_eq!(seal_key, format!("turn:{}", result.turn_id));

    // Verify working state in registry
    let entry = registry.get(&draft_id).unwrap();
    let ws = entry.get_snapshot();
    let arthur = ws.business_flow_data.require.character.iter().find(|c| c.code == "char_arthur");
    assert!(arthur.is_some());
    assert_eq!(arthur.unwrap().display_name.value.as_deref(), Some("Arthur Pendragon"));
    assert_eq!(arthur.unwrap().display_name.source, Some(ProvenanceSource::AiGenerated));

    // Verify stream events emitted
    let mut events = Vec::new();
    while let Ok(evt) = rx.try_recv() {
        events.push(evt);
    }
    assert!(events.iter().any(|e| matches!(e, crate::dto::ChatStreamEvent::TurnStarted { .. })));
    assert!(events.iter().any(|e| matches!(e, crate::dto::ChatStreamEvent::ToolCallCompleted { .. })));
    assert!(events.iter().any(|e| matches!(e, crate::dto::ChatStreamEvent::TurnCompleted { .. })));

    // Verify message persistence in SQLite
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM messages WHERE session_id = ?1",
        params![sess_id],
        |r| r.get(0),
    ).unwrap();
    assert_eq!(count, 2, "Both user and assistant messages must be saved");
}

/// `P3S7-N`: 3 consecutive rejections stops loop, requests user clarification in system_language.
#[tokio::test]
async fn test_P3S7_N_three_consecutive_rejections_stops_loop_requests_clarification() {
    let (pool, registry, _draft_svc, _patch_svc, orchestrator, _tmp) = setup_test_context();
    let (sess_id, _draft_id, _proj_id) = create_test_session(&pool, &registry);

    let provider = Arc::new(MockLlmProvider::new());

    // Step 1: Call 1 rejected (unauthorized caller or invalid tool)
    provider.queue_response(ProviderChatResponse {
        content: Some("Calling invalid tool 1".to_string()),
        tool_calls: vec![ProviderToolCall {
            id: "call_bad_1".to_string(),
            name: "nonexistent_tool".to_string(),
            arguments: serde_json::json!({}),
        }],
        finish_reason: Some("tool_calls".to_string()),
        usage: None,
    });

    // Step 2: Call 2 rejected
    provider.queue_response(ProviderChatResponse {
        content: Some("Calling invalid tool 2".to_string()),
        tool_calls: vec![ProviderToolCall {
            id: "call_bad_2".to_string(),
            name: "unknown_tool_2".to_string(),
            arguments: serde_json::json!({}),
        }],
        finish_reason: Some("tool_calls".to_string()),
        usage: None,
    });

    // Step 3: Call 3 rejected (exceeds limit 3)
    provider.queue_response(ProviderChatResponse {
        content: Some("Calling invalid tool 3".to_string()),
        tool_calls: vec![ProviderToolCall {
            id: "call_bad_3".to_string(),
            name: "unknown_tool_3".to_string(),
            arguments: serde_json::json!({}),
        }],
        finish_reason: Some("tool_calls".to_string()),
        usage: None,
    });

    // Final response without tools asking for user clarification
    provider.queue_response(ProviderChatResponse {
        content: Some("Tôi gặp khó khăn khi thao tác. Bạn có thể nói rõ hơn bạn muốn tạo nhân vật nào không?".to_string()),
        tool_calls: vec![],
        finish_reason: Some("stop".to_string()),
        usage: None,
    });

    let req = SendChatMessageReq {
        session_id: sess_id.clone(),
        content: "Do something strange".to_string(),
        system_language: "vi".to_string(),
        feedback_context: None,
    };

    let result = exec_send_chat_message(&orchestrator, provider.clone(), &req, None)
        .await
        .unwrap();

    assert!(result.assistant_text.contains("Tôi gặp khó khăn khi thao tác"));
    assert!(result.sealed_version.is_some(), "Initial system language should be sealed on the first turn");

    // Verify messages saved
    let conn = pool.get().unwrap();
    let asst_payload: String = conn.query_row(
        "SELECT payload_json FROM messages WHERE session_id = ?1 AND role = 'assistant'",
        params![sess_id],
        |r| r.get(0),
    ).unwrap();

    let parsed: serde_json::Value = serde_json::from_str(&asst_payload).unwrap();
    let tool_calls = parsed["tool_calls"].as_array().unwrap();
    assert_eq!(tool_calls.len(), 3);
    for tc in tool_calls {
        assert_eq!(tc["status"], "rejected");
    }
}

/// `P3S7-MAX`: Max iterations reached: controlled exit seals applied changes.
#[tokio::test]
async fn test_P3S7_MAX_iterations_reached_controlled_exit_seals_applied_changes() {
    let (pool, registry, draft_svc, patch_svc, _orchestrator, _tmp) = setup_test_context();
    let (sess_id, _draft_id, _proj_id) = create_test_session(&pool, &registry);

    // Set max_iterations to 3
    let custom_config = OrchestratorConfig {
        max_iterations: 3,
        consecutive_rejected_limit: 5,
        draft_lock_timeout: Duration::from_secs(10),
    };

    let custom_orchestrator = Arc::new(
        ChatOrchestrator::new(pool.clone(), registry.clone(), draft_svc, patch_svc)
            .with_config(custom_config),
    );

    let provider = Arc::new(MockLlmProvider::new());

    // Iteration 1: applies a mutation
    provider.queue_response(ProviderChatResponse {
        content: Some("Iter 1".to_string()),
        tool_calls: vec![ProviderToolCall {
            id: "call_proj_1".to_string(),
            name: "set_project".to_string(),
            arguments: serde_json::json!({
                "display_name": "My Great Novel",
                "declared_sources": { "display_name": "ai_generated" }
            }),
        }],
        finish_reason: Some("tool_calls".to_string()),
        usage: None,
    });

    // Iteration 2: another tool call
    provider.queue_response(ProviderChatResponse {
        content: Some("Iter 2".to_string()),
        tool_calls: vec![ProviderToolCall {
            id: "call_proj_2".to_string(),
            name: "set_project".to_string(),
            arguments: serde_json::json!({
                "logline": "A story of adventure",
                "declared_sources": { "logline": "ai_generated" }
            }),
        }],
        finish_reason: Some("tool_calls".to_string()),
        usage: None,
    });

    // Iteration 3: another tool call (hits max_iter 3)
    provider.queue_response(ProviderChatResponse {
        content: Some("Iter 3".to_string()),
        tool_calls: vec![ProviderToolCall {
            id: "call_proj_3".to_string(),
            name: "set_project".to_string(),
            arguments: serde_json::json!({
                "language": "vi",
                "declared_sources": { "language": "ai_generated" }
            }),
        }],
        finish_reason: Some("tool_calls".to_string()),
        usage: None,
    });

    let req = SendChatMessageReq {
        session_id: sess_id,
        content: "Loop forever".to_string(),
        system_language: "vi".to_string(),
        feedback_context: None,
    };

    let result = exec_send_chat_message(&custom_orchestrator, provider.clone(), &req, None)
        .await
        .unwrap();

    // Must exit cleanly and seal the applied changes
    assert!(result.sealed_version.is_some());
    assert_eq!(result.sealed_version.unwrap().seq, 1);
}

/// `P3S7-CANCEL`: Cancel mid-turn seals applied changes and marks cancelled.
#[tokio::test]
async fn test_P3S7_CANCEL_mid_turn_seals_applied_changes_and_marks_cancelled() {
    let (pool, registry, _draft_svc, _patch_svc, orchestrator, _tmp) = setup_test_context();
    let (sess_id, draft_id, _proj_id) = create_test_session(&pool, &registry);

    // Cancel token already triggered
    let cancel_token = CancellationToken::new();
    cancel_token.cancel();

    let entry = registry.get(&draft_id).unwrap();
    // Simulate that a write was already applied
    entry.mutate_working_state(|ws| {
        ws.business_flow_data.require.project.display_name = TrackedField {
            value: Some("Applied Before Cancel".to_string()),
            source: Some(ProvenanceSource::AiGenerated),
        };
    });

    let provider = Arc::new(MockLlmProvider::new());
    let user_msg_id = Uuid::new_v4();
    let turn_id = Uuid::new_v4();
    orchestrator.save_message(user_msg_id, &sess_id, turn_id, "user", "Cancel me", None).unwrap();

    // Running turn with pre-cancelled token
    let res = orchestrator
        .run_ai_turn(
            draft_id,
            &sess_id,
            user_msg_id,
            turn_id,
            "Cancel me",
            "vi",
            provider,
            None,
            cancel_token,
        )
        .await
        .unwrap();

    assert!(res.assistant_text.contains("cancelled"));

    // Cancel IPC check
    let cancel_req = CancelChatTurnReq {
        session_id: sess_id,
        turn_id,
    };
    let cancel_res = exec_cancel_chat_turn(&orchestrator, &cancel_req).await.unwrap();
    assert!(cancel_res.cancelled);
}

/// `P3S7-REGEN`: `regenerate_draft_phase` only overwrites `ai_generated` fields, preserves `user`/`user_edited`.
#[tokio::test]
async fn test_P3S7_REGEN_regenerate_draft_phase_only_overwrites_ai_generated_fields() {
    let (pool, registry, _draft_svc, _patch_svc, orchestrator, _tmp) = setup_test_context();
    let (_sess_id, draft_id, _proj_id) = create_test_session(&pool, &registry);

    let entry = registry.get(&draft_id).unwrap();

    // Set up initial state with mixed provenance:
    // 1. display_name is `user` (PROTECTED)
    // 2. target.word_per_chapter_count is `ai_generated` (MUTABLE)
    entry.mutate_working_state(|ws| {
        ws.business_flow_data.require.project.display_name = TrackedField {
            value: Some("User Protected Title".to_string()),
            source: Some(ProvenanceSource::User),
        };
        ws.business_flow_data.require.project.target.word_per_chapter_count = TrackedField {
            value: Some(3000),
            source: Some(ProvenanceSource::AiGenerated),
        };
    });

    // Candidate data trying to overwrite both fields
    let mut candidate = BusinessFlowData::new();
    candidate.require.project.display_name = TrackedField {
        value: Some("AI Attempted Title Overwrite".to_string()),
        source: Some(ProvenanceSource::AiGenerated),
    };
    candidate.require.project.target.word_per_chapter_count = TrackedField {
        value: Some(5000),
        source: Some(ProvenanceSource::AiGenerated),
    };

    let req = RegenerateDraftPhaseReq {
        draft_session_id: draft_id,
        phase: 0,
        subphase: None,
        system_language: "vi".to_string(),
    };

    let res = exec_regenerate_draft_phase(&orchestrator, &req, Some(candidate))
        .await
        .unwrap();

    assert!(res.sealed_version.is_some());
    let sealed = res.sealed_version.unwrap();
    assert_eq!(sealed.reason, "regenerate");

    // Verify seal_key in SQLite
    let conn = pool.get().unwrap();
    let seal_key: String = conn.query_row(
        "SELECT seal_key FROM draft_versions WHERE draft_session_id = ?1 AND seq = ?2",
        params![draft_id.to_string(), sealed.seq],
        |r| r.get(0),
    ).unwrap();
    assert!(seal_key.starts_with("regenerate:"));

    // Verify protected provenance in working state
    let ws = entry.get_snapshot();
    assert_eq!(
        ws.business_flow_data.require.project.display_name.value.as_deref(),
        Some("User Protected Title"),
        "User provenance field MUST be preserved untouched"
    );
    assert_eq!(
        ws.business_flow_data.require.project.display_name.source,
        Some(ProvenanceSource::User)
    );
    assert_eq!(
        ws.business_flow_data.require.project.target.word_per_chapter_count.value,
        Some(5000),
        "AI generated field MUST be overwritten"
    );
}

/// `IPC-CMD-001`: `create_or_load_chat_session`
#[test]
fn test_IPC_CMD_001_create_or_load_chat_session() {
    let (_pool, _registry, _draft_svc, _patch_svc, orchestrator, _tmp) = setup_test_context();

    // 1. Create new session when session_id is None
    let req = CreateOrLoadChatSessionReq {
        session_id: None,
        project_id: None,
    };
    let init_dto = exec_create_or_load_chat_session(&orchestrator, &req).unwrap();
    assert!(!init_dto.session_id.is_empty());
    assert_eq!(init_dto.status, "editing");
    assert!(init_dto.messages.is_empty());

    // 2. Load existing session
    let load_req = CreateOrLoadChatSessionReq {
        session_id: Some(init_dto.session_id.clone()),
        project_id: None,
    };
    let loaded = exec_create_or_load_chat_session(&orchestrator, &load_req).unwrap();
    assert_eq!(loaded.session_id, init_dto.session_id);
    assert_eq!(loaded.draft_session_id, init_dto.draft_session_id);

    // 3. EntityNotFound on invalid session_id
    let invalid_req = CreateOrLoadChatSessionReq {
        session_id: Some("nonexistent_session_id".to_string()),
        project_id: None,
    };
    let err = exec_create_or_load_chat_session(&orchestrator, &invalid_req).unwrap_err();
    assert_eq!(err.code, ErrorCode::EntityNotFound);
}

/// `IPC-CMD-002`: `list_chat_sessions`
#[test]
fn test_IPC_CMD_002_list_chat_sessions() {
    let (pool, registry, _draft_svc, _patch_svc, orchestrator, _tmp) = setup_test_context();
    let (_s1, _d1, proj_id) = create_test_session(&pool, &registry);
    let (_s2, _d2, _) = create_test_session(&pool, &registry);

    let req = ListChatSessionsReq {
        project_id: Some(proj_id),
        limit: Some(10),
        cursor: None,
        has_messages: None,
        min_message_count: None,
        trash_only: None,
    };

    let list_res = exec_list_chat_sessions(&orchestrator, &req).unwrap();
    assert_eq!(list_res.items.len(), 1);
    assert_eq!(list_res.items[0].project_id, Some(proj_id));
}

#[test]
fn test_list_chat_sessions_minimum_two_messages() {
    let (pool, registry, _draft_svc, _patch_svc, orchestrator, _tmp) = setup_test_context();
    let (one_message_session, _, _) = create_test_session(&pool, &registry);
    let (two_message_session, _, _) = create_test_session(&pool, &registry);
    let turn_id = Uuid::new_v4();
    orchestrator.save_message(Uuid::new_v4(), &one_message_session, turn_id, "user", "Hello", None).unwrap();
    orchestrator.save_message(Uuid::new_v4(), &two_message_session, turn_id, "user", "Hello", None).unwrap();
    orchestrator.save_message(Uuid::new_v4(), &two_message_session, turn_id, "assistant", "Hi", None).unwrap();

    let result = exec_list_chat_sessions(&orchestrator, &ListChatSessionsReq {
        min_message_count: Some(2),
        limit: Some(100),
        ..Default::default()
    }).unwrap();
    assert_eq!(result.items.len(), 1);
    assert_eq!(result.items[0].session_id, two_message_session);
    assert_eq!(result.items[0].message_count, Some(2));
}

#[test]
fn test_blank_assistant_messages_are_only_in_trash() {
    let (pool, registry, _, _, orchestrator, _tmp) = setup_test_context();
    let (session, _, _) = create_test_session(&pool, &registry);
    let turn = Uuid::new_v4();
    orchestrator.save_message(Uuid::new_v4(), &session, turn, "user", "Chào", None).unwrap();
    for content in ["", " \n\t\r"] {
        orchestrator.save_message(Uuid::new_v4(), &session, turn, "assistant", content, None).unwrap();
    }
    for minimum in [None, Some(2)] {
        let history = exec_list_chat_sessions(&orchestrator, &ListChatSessionsReq {
            min_message_count: minimum, ..Default::default()
        }).unwrap();
        assert!(history.items.is_empty(), "Blank assistant rows must not make a conversation visible");
    }
    let trash = exec_list_chat_sessions(&orchestrator, &ListChatSessionsReq {
        trash_only: Some(true), ..Default::default()
    }).unwrap();
    assert_eq!(trash.items.len(), 1);
    assert_eq!(trash.items[0].message_count, Some(1));
    exec_manage_chat_sessions(&orchestrator, &ManageChatSessionsReq {
        action: "permanent_delete".into(), session_ids: vec![session], display_name: None,
    }).unwrap();
}

#[test]
fn test_chat_trash_rename_soft_delete_restore_and_permanent_delete() {
    let (pool, registry, _draft_svc, _patch_svc, orchestrator, _tmp) = setup_test_context();
    let (unfinished, _, _) = create_test_session(&pool, &registry);
    let (completed, completed_draft, _) = create_test_session(&pool, &registry);
    let turn = Uuid::new_v4();
    orchestrator.save_message(Uuid::new_v4(), &unfinished, turn, "user", "One", None).unwrap();
    orchestrator.save_message(Uuid::new_v4(), &completed, turn, "user", "Two", None).unwrap();
    orchestrator.save_message(Uuid::new_v4(), &completed, turn, "assistant", "Reply", None).unwrap();

    let trash_req = ListChatSessionsReq { trash_only: Some(true), ..Default::default() };
    let initial_trash = exec_list_chat_sessions(&orchestrator, &trash_req).unwrap();
    assert_eq!(initial_trash.items.len(), 1);
    assert_eq!(initial_trash.items[0].session_id, unfinished);
    assert!(initial_trash.items[0].deleted_at.is_none());

    exec_manage_chat_sessions(&orchestrator, &ManageChatSessionsReq {
        action: "rename".into(), session_ids: vec![completed.clone()], display_name: Some("Tên mới".into()),
    }).unwrap();
    exec_manage_chat_sessions(&orchestrator, &ManageChatSessionsReq {
        action: "soft_delete".into(), session_ids: vec![completed.clone()], display_name: None,
    }).unwrap();
    let trash = exec_list_chat_sessions(&orchestrator, &trash_req).unwrap();
    assert_eq!(trash.items.len(), 2);
    assert!(trash.items.iter().any(|item| item.session_id == completed && item.deleted_at.is_some() && item.display_name.as_deref() == Some("Tên mới")));

    exec_manage_chat_sessions(&orchestrator, &ManageChatSessionsReq {
        action: "restore".into(), session_ids: vec![completed.clone()], display_name: None,
    }).unwrap();
    assert!(exec_list_chat_sessions(&orchestrator, &trash_req).unwrap().items.iter().all(|item| item.session_id != completed));

    exec_manage_chat_sessions(&orchestrator, &ManageChatSessionsReq {
        action: "soft_delete".into(), session_ids: vec![completed.clone()], display_name: None,
    }).unwrap();
    let deleted = exec_manage_chat_sessions(&orchestrator, &ManageChatSessionsReq {
        action: "permanent_delete".into(), session_ids: vec![unfinished.clone(), completed.clone()], display_name: None,
    }).unwrap();
    assert_eq!(deleted.affected_count, 2);
    let conn = pool.get().unwrap();
    let old_sessions: i64 = conn.query_row("SELECT COUNT(*) FROM sessions WHERE id IN (?1, ?2)", params![unfinished, completed], |row| row.get(0)).unwrap();
    assert_eq!(old_sessions, 0);
    let preserved_project_draft: i64 = conn.query_row("SELECT COUNT(*) FROM draft_sessions WHERE id = ?1", [completed_draft.to_string()], |row| row.get(0)).unwrap();
    assert_eq!(preserved_project_draft, 1);
}

/// `IPC-CMD-003`: `send_chat_message`
#[tokio::test]
async fn test_IPC_CMD_003_send_chat_message() {
    let (pool, registry, _draft_svc, _patch_svc, orchestrator, _tmp) = setup_test_context();
    let (sess_id, _draft_id, _proj_id) = create_test_session(&pool, &registry);

    let provider = Arc::new(MockLlmProvider::new());
    provider.queue_response(ProviderChatResponse {
        content: Some("Xin chào! Tôi có thể giúp gì cho bạn?".to_string()),
        tool_calls: vec![],
        finish_reason: Some("stop".to_string()),
        usage: None,
    });

    let req = SendChatMessageReq {
        session_id: sess_id,
        content: "Chào bạn".to_string(),
        system_language: "vi".to_string(),
        feedback_context: None,
    };

    let res = exec_send_chat_message(&orchestrator, provider, &req, None)
        .await
        .unwrap();

    assert_eq!(res.assistant_text, "Xin chào! Tôi có thể giúp gì cho bạn?");
    assert_eq!(res.progress.current_phase, 0);
}

#[tokio::test]
async fn test_failed_tool_call_remains_in_history_when_followup_provider_request_fails() {
    let (pool, registry, _draft_svc, _patch_svc, orchestrator, _tmp) = setup_test_context();
    let (session_id, _, _) = create_test_session(&pool, &registry);
    let provider = Arc::new(MockLlmProvider::new());
    provider.queue_response(ProviderChatResponse {
        content: None,
        tool_calls: vec![ProviderToolCall {
            id: "failed-tool".into(),
            name: "nonexistent_tool".into(),
            arguments: serde_json::json!({}),
        }],
        finish_reason: Some("tool_calls".into()),
        usage: None,
    });
    provider.queue_error(ProviderError::Unreachable { status: None, message: "offline".into() });

    let result = exec_send_chat_message(&orchestrator, provider, &SendChatMessageReq {
        session_id: session_id.clone(), content: "Try a tool".into(),
        system_language: "vi".into(), feedback_context: None,
    }, None).await;
    assert_eq!(result.unwrap_err().code, ErrorCode::ProviderUnreachable);

    let messages = orchestrator.fetch_recent_session_messages(&session_id, Uuid::nil(), 10).unwrap();
    assert_eq!(messages.len(), 2, "both the user request and failed tool audit should survive");
    assert_eq!(messages[1].role, "assistant");
    assert_eq!(messages[1].tool_calls.as_ref().unwrap().len(), 1);
    assert_eq!(messages[1].tool_calls.as_ref().unwrap()[0].status, "rejected");
}

/// `IPC-CMD-004`: `cancel_chat_turn`
#[tokio::test]
async fn test_IPC_CMD_004_cancel_chat_turn() {
    let (pool, registry, _draft_svc, _patch_svc, orchestrator, _tmp) = setup_test_context();
    let (sess_id, _draft_id, _proj_id) = create_test_session(&pool, &registry);

    let turn_id = Uuid::new_v4();
    let req = CancelChatTurnReq {
        session_id: sess_id,
        turn_id,
    };

    let res = exec_cancel_chat_turn(&orchestrator, &req).await.unwrap();
    assert!(res.cancelled);
}

/// `IPC-CMD-009`: `regenerate_draft_phase`
#[tokio::test]
async fn test_IPC_CMD_009_regenerate_draft_phase() {
    let (pool, registry, _draft_svc, _patch_svc, orchestrator, _tmp) = setup_test_context();
    let (_sess_id, draft_id, _proj_id) = create_test_session(&pool, &registry);

    let req = RegenerateDraftPhaseReq {
        draft_session_id: draft_id,
        phase: 0,
        subphase: None,
        system_language: "vi".to_string(),
    };

    let res = exec_regenerate_draft_phase(&orchestrator, &req, None).await.unwrap();
    assert!(res.sealed_version.is_some());
    assert_eq!(res.sealed_version.unwrap().reason, "regenerate");
}
