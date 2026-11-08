//! Chat & Prompt Orchestrator implementing ReAct single-turn loop,
//! prompt composition, and version sealing boundaries (§6.1, `docs/business/prompts/overview.md`, `core.md` §3.8).
//!
//! Enforces:
//! 1. Prompt composer (§6.1): Distinguishes `system_language` (UI communication language, "vi" | "en")
//!    from `project.language` (draft creative content language). Injects prompt blocks JIT.
//! 2. ReAct turn loop (`run_turn_loop`):
//!    - Strictly NO SQLite transaction held across network await (`CI-03`).
//!    - Max iterations limit (default 10) and consecutive rejections limit (3 consecutive rejections
//!      stops tool loop, skips remaining calls with `turn_aborted`, and prompts user for clarification).
//!    - Tool execution gated by ToolRegistry authorization.
//!    - Protected provenance preservation: AI tools attempting to overwrite `user` or `user_edited`
//!      fields emit pending proposals (TX-08) rather than direct state mutation.
//! 3. Turn conclusion:
//!    - If mutations were `applied` (`has_applied_write == true`), seals draft version via TX-01
//!      (`seal_key = "turn:{turn_id}"`, reason = `"ai_patch"`).
//!    - Retries at most once on seal failure (`CI-03`); if still failing, marks `SealPendingRetry`
//!      and returns `ErrorCode::VersionSealFailed`.
//!    - On cancellation: If mutations were applied, still seals TX-01 for the applied subset.
//! 4. Phase regeneration (`regenerate_draft_phase`):
//!    - Only overwrites fields where `source == 'ai_generated'`.
//!    - Strictly preserves `user` and `user_edited` fields.
//!    - Seals draft version via TX-01 with `reason = 'regenerate'`, `seal_key = "regenerate:{turn_id}"`.

use chrono::Utc;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeSet;
use std::fmt::Write as _;
use std::sync::Arc;
use std::time::Duration;
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

use crate::domain::business_flow::{BusinessFlowData, ProvenanceSource};
use crate::domain::fingerprint::state_data_hash;
use crate::domain::tools::registry::{CallerCategory, ToolRegistry};
use crate::domain::tools::write_tools::{
    register_all_write_tools, ExecutionGuard, ToolCall, ToolExecutor, ToolOutcome,
};
use crate::error::{ErrorCode, IpcError};
use crate::ipc::dto::{
    ChatMessageDto, ChatStreamEvent, DraftVersionSummaryDto, PendingPatchDto,
    SendChatTurnResponseDto, ToolCallItemDto,
};
use crate::providers::traits::{
    LlmProvider, ProviderChatRequest, ProviderMessage, ProviderToolDefinition,
};
use crate::service::draft_runtime::{
    DraftRuntimeEntry, DraftRuntimeRegistry, DRAFT_LOCK_TIMEOUT,
};
use crate::service::draft_service::{DraftService, SealVersionParams};
use crate::service::patch_service::{compute_progress_dto, PatchService};
use crate::storage::pending_repo::{persist_proposal_tx, PersistProposalParams};

/// Format specification for a prompt block.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum PromptBlockFormat {
    Json,
    Markdown,
}

impl Default for PromptBlockFormat {
    fn default() -> Self {
        Self::Markdown
    }
}

/// Dynamic prompt block retrieved from database or default seed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptBlock {
    pub id: Uuid,
    pub prompt_code: String,
    pub display_name: String,
    pub heading_level: u8,
    pub content_format: PromptBlockFormat,
    pub content: String,
}

/// Composed prompt specification including text and available tool definitions.
#[derive(Debug, Clone)]
pub struct RunSpec {
    pub prompt: String,
    pub tools: Vec<ProviderToolDefinition>,
}

/// In-memory state tracking for an active ReAct turn loop.
#[derive(Debug, Default)]
pub struct TurnState {
    pub has_applied_write: bool,
    pub consecutive_rejected_count: usize,
    pub all_tool_calls: Vec<ToolCallItemDto>,
    pub assistant_text: String,
}

/// Configuration parameters for ReAct orchestration turn loop.
#[derive(Debug, Clone)]
pub struct OrchestratorConfig {
    pub max_iterations: usize,
    pub consecutive_rejected_limit: usize,
    pub draft_lock_timeout: Duration,
}

impl Default for OrchestratorConfig {
    fn default() -> Self {
        Self {
            max_iterations: 10,
            consecutive_rejected_limit: 3,
            draft_lock_timeout: DRAFT_LOCK_TIMEOUT,
        }
    }
}

/// Prompt Composer managing prompt block assembly, tool schema retrieval, and template interpolation.
pub struct PromptComposer {
    pool: Pool<SqliteConnectionManager>,
}

impl PromptComposer {
    pub fn new(pool: Pool<SqliteConnectionManager>) -> Self {
        Self { pool }
    }

    /// Load a prompt block from SQLite `prompt_blocks` or fall back to standard built-in defaults.
    pub fn get_prompt_block(&self, prompt_code: &str) -> Result<PromptBlock, IpcError> {
        let conn = self
            .pool
            .get()
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

        let mut stmt = conn
            .prepare(
                "SELECT id, prompt_code, display_name, heading_level, content_format, content
                 FROM prompt_blocks
                 WHERE prompt_code = ?1",
            )
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

        let mut rows = stmt
            .query(params![prompt_code])
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

        if let Some(row) = rows
            .next()
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?
        {
            let id_str: String = row.get(0).unwrap_or_default();
            let prompt_id = Uuid::parse_str(&id_str).unwrap_or_else(|_| Uuid::new_v4());
            let code: String = row.get(1).unwrap_or_default();
            let display_name: String = row.get(2).unwrap_or_default();
            let heading_level: i64 = row.get(3).unwrap_or(2);
            let format_str: String = row.get(4).unwrap_or_else(|_| "MARKDOWN".to_string());
            let content: String = row.get(5).unwrap_or_default();

            let content_format = if format_str.eq_ignore_ascii_case("JSON") {
                PromptBlockFormat::Json
            } else {
                PromptBlockFormat::Markdown
            };

            return Ok(PromptBlock {
                id: prompt_id,
                prompt_code: code,
                display_name,
                heading_level: heading_level.clamp(1, 6) as u8,
                content_format,
                content,
            });
        }

        // Built-in fallback blocks when DB has not seeded prompt_blocks
        self.get_fallback_prompt_block(prompt_code)
    }

    fn get_fallback_prompt_block(&self, prompt_code: &str) -> Result<PromptBlock, IpcError> {
        match prompt_code {
            "merlink_core" => Ok(PromptBlock {
                id: Uuid::new_v4(),
                prompt_code: "merlink_core".to_string(),
                display_name: "NGUYÊN TẮC CỐT LÕI (BẤT BIẾN)".to_string(),
                heading_level: 2,
                content_format: PromptBlockFormat::Markdown,
                content: "1. Quy ước Định danh: Trường `code` là slug định danh nội bộ, TUYỆT ĐỐI KHÔNG hiển thị `code` trên giao diện người dùng. Dùng `display_name` hoặc `nickname`.\n2. Ngôn ngữ & Giọng điệu: Giao tiếp với người dùng bằng `{{system_language}}`. Nội dung sáng tác ghi vào State dùng `{{project.language}}`.\n3. Cấm kỵ tuyệt đối: Không sử dụng emoji icon và không được dùng kí tự \"—\" trong bất kỳ output nào.\n4. Thu thập theo mọi phase: Bóc tách và ghi qua tool ngay trong lượt mọi thông tin người dùng nêu.\n5. Không tự ghi đè dữ liệu user hoặc user_edited.".to_string(),
            }),
            "planning_mode" => Ok(PromptBlock {
                id: Uuid::new_v4(),
                prompt_code: "planning_mode".to_string(),
                display_name: "PHÂN BIỆT PLANNING MODE VÀ WRITING MODE (BẤT BIẾN)".to_string(),
                heading_level: 2,
                content_format: PromptBlockFormat::Markdown,
                content: "1. Luồng chat mới mặc định phục vụ Planning Mode.\n2. Không sinh nội dung chương hoàn chỉnh khi plan chưa được commit.\n3. Không tự viết hàng loạt chương.\n4. Không tự chuyển sang Writing Mode.".to_string(),
            }),
            "tool_library" => Ok(PromptBlock {
                id: Uuid::new_v4(),
                prompt_code: "tool_library".to_string(),
                display_name: "THƯ VIỆN TOOL".to_string(),
                heading_level: 2,
                content_format: PromptBlockFormat::Markdown,
                content: "Chỉ gọi các tool đã được cấp qua function calling để đọc và cập nhật dữ liệu. Khi gọi tool ghi nhiều trường (`add_`, `set_`), luôn khai báo `declared_sources` theo từng trường: trường người dùng nói trực tiếp gắn `user`, trường AI tự suy luận gắn `ai_generated`. Khi gọi tool xóa (`delete_*`), khai báo `declared_source` scalar.".to_string(),
            }),
            _ => Ok(PromptBlock {
                id: Uuid::new_v4(),
                prompt_code: prompt_code.to_string(),
                display_name: format!("PROMPT BLOCK: {}", prompt_code),
                heading_level: 2,
                content_format: PromptBlockFormat::Markdown,
                content: format!("Prompt block {} content instructions.", prompt_code),
            }),
        }
    }

    /// Retrieve tool definitions associated with a prompt block from `prompt_block_tools` JOIN `tools`.
    pub fn get_tools_by_prompt(&self, prompt_code: &str) -> Result<Vec<ProviderToolDefinition>, IpcError> {
        let conn = self
            .pool
            .get()
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

        let mut stmt = conn
            .prepare(
                "SELECT t.code, t.description, t.input_schema
                 FROM prompt_blocks pb
                 JOIN prompt_block_tools pbt ON pb.id = pbt.prompt_block_id
                 JOIN tools t ON pbt.tool_id = t.id
                 WHERE pb.prompt_code = ?1",
            )
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

        let mut rows = stmt
            .query(params![prompt_code])
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

        let mut tools = Vec::new();
        while let Some(row) = rows
            .next()
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?
        {
            let code: String = row.get(0).unwrap_or_default();
            let desc: Option<String> = row.get(1).unwrap_or(None);
            let schema_str: Option<String> = row.get(2).unwrap_or(None);
            let parameters = schema_str
                .and_then(|s| serde_json::from_str(&s).ok())
                .unwrap_or_else(|| serde_json::json!({ "type": "object", "properties": {} }));

            tools.push(ProviderToolDefinition {
                name: code,
                description: desc.unwrap_or_default(),
                parameters,
            });
        }

        Ok(tools)
    }

    /// Compose prompt text and deduplicated tool definitions across an ordered sequence of prompt block codes.
    pub fn compose(
        &self,
        script: &[&str],
        system_language: &str,
        project_language: &str,
    ) -> Result<RunSpec, IpcError> {
        let mut blocks = Vec::with_capacity(script.len());
        let mut seen = BTreeSet::new();
        let mut tools = Vec::new();

        let human_system_lang = match system_language {
            "vi" => "Tiếng Việt",
            "en" => "English",
            other => other,
        };

        let human_project_lang = if project_language.is_empty() {
            "Tiếng Việt"
        } else {
            project_language
        };

        for &code in script {
            let block = self.get_prompt_block(code)?;

            if !(1..=6).contains(&block.heading_level) {
                return Err(IpcError::new(
                    ErrorCode::ValidationFailed,
                    format!("Invalid heading level {} for block {}", block.heading_level, code),
                ));
            }

            let mut out = String::new();
            write!(
                out,
                "{} {}\n\n",
                "#".repeat(block.heading_level as usize),
                block.display_name
            )
            .map_err(|e| IpcError::new(ErrorCode::InternalError, e.to_string()))?;

            let processed_content = block
                .content
                .replace("{{system_language}}", human_system_lang)
                .replace("{{project.language}}", human_project_lang);

            match block.content_format {
                PromptBlockFormat::Markdown => {
                    write!(out, "{}", processed_content)
                        .map_err(|e| IpcError::new(ErrorCode::InternalError, e.to_string()))?;
                }
                PromptBlockFormat::Json => {
                    write!(out, "```json\n{}\n```", processed_content)
                        .map_err(|e| IpcError::new(ErrorCode::InternalError, e.to_string()))?;
                }
            }

            blocks.push(out.trim_end().to_string());

            let block_tools = self.get_tools_by_prompt(code)?;
            for t in block_tools {
                if seen.insert(t.name.clone()) {
                    tools.push(t);
                }
            }
        }

        // If tools are still empty, populate standard planning tool definitions
        if tools.is_empty() {
            tools = Self::get_standard_planning_tools();
        }

        Ok(RunSpec {
            prompt: blocks.join("\n\n"),
            tools,
        })
    }

    /// Default tool definitions corresponding to standard write tools.
    pub fn get_standard_planning_tools() -> Vec<ProviderToolDefinition> {
        vec![
            ProviderToolDefinition {
                name: "set_character".to_string(),
                description: "Update character fields in draft working state".to_string(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "code": { "type": "string" },
                        "display_name": { "type": "string" },
                        "nickname": { "type": "string" },
                        "gender": { "type": "string" },
                        "age": { "type": "integer" },
                        "role": { "type": "string" },
                        "declared_sources": { "type": "object" }
                    },
                    "required": ["code"]
                }),
            },
            ProviderToolDefinition {
                name: "add_character".to_string(),
                description: "Add a new character to draft working state".to_string(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "code": { "type": "string" },
                        "display_name": { "type": "string" },
                        "role": { "type": "string" },
                        "declared_sources": { "type": "object" }
                    },
                    "required": ["code", "display_name"]
                }),
            },
            ProviderToolDefinition {
                name: "delete_character".to_string(),
                description: "Delete a character from draft working state".to_string(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "code": { "type": "string" },
                        "declared_source": { "type": "string" }
                    },
                    "required": ["code"]
                }),
            },
            ProviderToolDefinition {
                name: "set_project".to_string(),
                description: "Update project metadata in draft working state".to_string(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "display_name": { "type": "string" },
                        "language": { "type": "string" },
                        "logline": { "type": "string" },
                        "target": { "type": "object" },
                        "declared_sources": { "type": "object" }
                    }
                }),
            },
            ProviderToolDefinition {
                name: "set_world_setting".to_string(),
                description: "Update world setting fields in draft working state".to_string(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "code": { "type": "string" },
                        "name": { "type": "string" },
                        "content": { "type": "string" },
                        "declared_sources": { "type": "object" }
                    },
                    "required": ["code"]
                }),
            },
            ProviderToolDefinition {
                name: "set_arc".to_string(),
                description: "Update arc fields in draft working state".to_string(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "code": { "type": "string" },
                        "name": { "type": "string" },
                        "declared_sources": { "type": "object" }
                    },
                    "required": ["code"]
                }),
            },
        ]
    }
}

/// Central application service orchestrating ReAct AI chat turns, prompt composition,
/// session lifecycle, and version sealing.
pub struct ChatOrchestrator {
    pool: Pool<SqliteConnectionManager>,
    registry: Arc<DraftRuntimeRegistry>,
    draft_service: Arc<DraftService>,
    patch_service: Arc<PatchService>,
    composer: PromptComposer,
    tool_registry: ToolRegistry,
    config: OrchestratorConfig,
}

impl ChatOrchestrator {
    pub fn new(
        pool: Pool<SqliteConnectionManager>,
        registry: Arc<DraftRuntimeRegistry>,
        draft_service: Arc<DraftService>,
        patch_service: Arc<PatchService>,
    ) -> Self {
        let mut tool_registry = ToolRegistry::new();
        register_all_write_tools(&mut tool_registry);

        Self {
            composer: PromptComposer::new(pool.clone()),
            pool,
            registry,
            draft_service,
            patch_service,
            tool_registry,
            config: OrchestratorConfig::default(),
        }
    }

    pub fn with_config(mut self, config: OrchestratorConfig) -> Self {
        self.config = config;
        self
    }

    pub fn pool(&self) -> &Pool<SqliteConnectionManager> {
        &self.pool
    }

    pub fn registry(&self) -> &Arc<DraftRuntimeRegistry> {
        &self.registry
    }

    pub fn draft_service(&self) -> &Arc<DraftService> {
        &self.draft_service
    }

    pub fn patch_service(&self) -> &Arc<PatchService> {
        &self.patch_service
    }

    pub fn composer(&self) -> &PromptComposer {
        &self.composer
    }

    pub fn tool_registry(&self) -> &ToolRegistry {
        &self.tool_registry
    }

    // ========================================================================
    // ReAct Turn Execution Boundary
    // ========================================================================

    /// Executes a single AI turn over the ReAct loop (§6.1, `docs/business/prompts/overview.md`).
    pub async fn run_ai_turn(
        &self,
        draft_session_id: Uuid,
        session_id: &str,
        user_message_id: Uuid,
        turn_id: Uuid,
        user_prompt: &str,
        system_language: &str,
        provider: Arc<dyn LlmProvider>,
        stream_tx: Option<tokio::sync::mpsc::Sender<ChatStreamEvent>>,
        cancel_token: CancellationToken,
    ) -> Result<SendChatTurnResponseDto, IpcError> {
        let entry = self
            .registry
            .get(&draft_session_id)
            .ok_or_else(|| IpcError::new(ErrorCode::EntityNotFound, "Draft session not in runtime"))?;

        // 1. Acquire draft operation lock with timeout (30s)
        let _draft_guard = entry
            .acquire_operation_lock("run_ai_turn", self.config.draft_lock_timeout)
            .await?;

        // Register cancel token
        *entry.active_cancel_token.lock().unwrap() = Some(cancel_token.clone());

        // Emit TurnStarted event
        if let Some(ref tx) = stream_tx {
            let _ = tx
                .send(ChatStreamEvent::TurnStarted {
                    turn_id,
                    user_message_id,
                })
                .await;
        }

        // 2. Prompt composition
        let ws_snapshot = entry.get_snapshot();
        let project_lang = ws_snapshot
            .business_flow_data
            .require
            .project
            .language
            .value
            .as_deref()
            .unwrap_or("vi");

        let script = ["merlink_core", "planning_mode", "tool_library"];
        let composed = self
            .composer
            .compose(&script, system_language, project_lang)?;

        // 3. Assemble conversation history for provider request
        let mut messages = vec![ProviderMessage::system(composed.prompt.clone())];

        // Inject up to 10 recent messages from SQLite for context
        let history = self.fetch_recent_session_messages(session_id, user_message_id, 10)?;
        for hist_msg in history {
            if hist_msg.role == "user" {
                messages.push(ProviderMessage::user(hist_msg.content));
            } else if hist_msg.role == "assistant" {
                messages.push(ProviderMessage::assistant(hist_msg.content));
            }
        }

        // Append current user goal
        messages.push(ProviderMessage::user(user_prompt.to_string()));

        // 4. Run ReAct Turn Loop
        let mut state = TurnState::default();
        let loop_result = self
            .run_turn_loop(
                &entry,
                draft_session_id,
                turn_id,
                &composed,
                &mut messages,
                &mut state,
                system_language,
                provider.as_ref(),
                stream_tx.as_ref(),
                cancel_token.clone(),
            )
            .await;

        // Clear active cancel token
        *entry.active_cancel_token.lock().unwrap() = None;

        // Check if loop failed fatally
        let assistant_text = match loop_result {
            Ok(txt) => txt,
            Err(err) => {
                if let Some(ref tx) = stream_tx {
                    let _ = tx
                        .send(ChatStreamEvent::TurnError {
                            turn_id,
                            error: err.clone(),
                        })
                        .await;
                }

                // If error occurred but we already have applied mutations, still seal TX-01!
                if state.has_applied_write {
                    let _ = self.seal_turn_version(&entry, draft_session_id, turn_id, user_message_id);
                }
                return Err(err);
            }
        };

        // 5. Turn Finalization & TX-01 Sealing
        let mut sealed_version = None;
        if state.has_applied_write {
            let sealed = self.seal_turn_version(&entry, draft_session_id, turn_id, user_message_id)?;
            sealed_version = Some(sealed);
        }

        let assistant_message_id = Uuid::new_v4();

        // Save assistant response to SQLite `messages` table
        let payload_json = serde_json::json!({
            "tool_calls": state.all_tool_calls,
        });
        self.save_message(
            assistant_message_id,
            session_id,
            turn_id,
            "assistant",
            &assistant_text,
            Some(&payload_json),
        )?;

        // Emit TurnCompleted event
        if let Some(ref tx) = stream_tx {
            let _ = tx
                .send(ChatStreamEvent::TurnCompleted {
                    turn_id,
                    assistant_message_id,
                    sealed_version: sealed_version.clone(),
                })
                .await;
        }

        let current_ws = entry.get_snapshot();
        let progress = compute_progress_dto(&current_ws);

        Ok(SendChatTurnResponseDto {
            turn_id,
            user_message_id,
            assistant_message_id,
            assistant_text,
            sealed_version,
            progress,
            pending_patches: current_ws.pending_patches.clone(),
        })
    }

    /// Internal ReAct loop implementation (`docs/business/prompts/overview.md`).
    async fn run_turn_loop(
        &self,
        entry: &Arc<DraftRuntimeEntry>,
        draft_session_id: Uuid,
        turn_id: Uuid,
        composed: &RunSpec,
        messages: &mut Vec<ProviderMessage>,
        state: &mut TurnState,
        system_language: &str,
        provider: &dyn LlmProvider,
        stream_tx: Option<&tokio::sync::mpsc::Sender<ChatStreamEvent>>,
        cancel_token: CancellationToken,
    ) -> Result<String, IpcError> {
        let max_iter = self.config.max_iterations;
        let rejected_limit = self.config.consecutive_rejected_limit;

        for _ in 0..max_iter {
            if cancel_token.is_cancelled() {
                return Ok("Turn cancelled by user".to_string());
            }

            let req = ProviderChatRequest {
                model: "chat".to_string(),
                messages: messages.clone(),
                tools: Some(composed.tools.clone()),
                temperature: None,
                max_tokens: None,
            };

            // Network call: strictly NO SQLite transaction held across this await
            let resp = match provider.chat_with_tools(&req, cancel_token.clone()).await {
                Ok(r) => r,
                Err(err) => {
                    if cancel_token.is_cancelled() {
                        return Ok("Turn cancelled by user".to_string());
                    }
                    return Err(err.to_ipc_error());
                }
            };

            if let Some(ref content) = resp.content {
                if !content.is_empty() {
                    state.assistant_text = content.clone();
                    if let Some(tx) = stream_tx {
                        let _ = tx
                            .send(ChatStreamEvent::AssistantDelta {
                                turn_id,
                                delta: content.clone(),
                            })
                            .await;
                    }
                }
            }

            if resp.tool_calls.is_empty() {
                // LLM completed its turn without calling any further tools
                return Ok(state.assistant_text.clone());
            }

            // Append assistant tool-calling message to history
            messages.push(ProviderMessage {
                role: "assistant".to_string(),
                content: resp.content.clone().unwrap_or_default(),
                tool_calls: Some(resp.tool_calls.clone()),
                tool_call_id: None,
            });

            let mut remaining_calls = resp.tool_calls.into_iter();
            while let Some(call) = remaining_calls.next() {
                if cancel_token.is_cancelled() {
                    return Ok("Turn cancelled by user".to_string());
                }

                if let Some(tx) = stream_tx {
                    let _ = tx
                        .send(ChatStreamEvent::ToolCallStarted {
                            turn_id,
                            tool_call_id: call.id.clone(),
                            tool_code: call.name.clone(),
                        })
                        .await;
                }

                let start_time = std::time::Instant::now();
                let tool_name = call.name.clone();

                // Gated check: Authorize tool against ToolRegistry
                let tool_meta = self.tool_registry.authorize(&tool_name, CallerCategory::AiCaller);

                let (outcome, execution_time_ms) = match tool_meta {
                    Err(ErrorCode::UnauthorizedCaller) => (
                        ToolOutcome::rejected("unauthorized_caller"),
                        start_time.elapsed().as_millis() as u64,
                    ),
                    Err(_) => (
                        ToolOutcome::rejected("tool_not_found"),
                        start_time.elapsed().as_millis() as u64,
                    ),
                    Ok(meta) => {
                        let current_snap = entry.get_snapshot();
                        let guard = ExecutionGuard::with_session(
                            &current_snap.business_flow_data,
                            draft_session_id.to_string(),
                        );

                        let mut tool_req = ToolCall::new(
                            tool_name.clone(),
                            CallerCategory::AiCaller,
                            call.arguments.clone(),
                        );

                        // Extract declared sources for provenance protection
                        if let Some(map) = call
                            .arguments
                            .get("declared_sources")
                            .and_then(|v| v.as_object())
                        {
                            let mut btree = std::collections::BTreeMap::new();
                            for (k, v) in map {
                                if let Ok(src) = serde_json::from_value::<ProvenanceSource>(v.clone()) {
                                    btree.insert(k.clone(), src);
                                }
                            }
                            tool_req.declared_sources = Some(btree);
                        } else if let Some(src_val) = call.arguments.get("declared_source") {
                            if let Ok(src) = serde_json::from_value::<ProvenanceSource>(src_val.clone()) {
                                tool_req.declared_source = Some(src);
                            }
                        }

                        let outcome = ToolExecutor::execute(&tool_req, &guard);
                        let elapsed = start_time.elapsed().as_millis() as u64;

                        // If tool writes draft state and produced applied mutations, update working state
                        let is_write = meta.writes_draft_state;
                        let applied = outcome.is_applied();

                        if is_write && applied {
                            if let Some(mutated) = outcome.mutated_state() {
                                entry.mutate_working_state(|ws| {
                                    ws.business_flow_data = mutated.clone();
                                });
                                state.has_applied_write = true;
                            }
                        }

                        // If tool produced pending proposals (protected provenance), persist via TX-08
                        if !outcome.pending_patches().is_empty() {
                            if let Ok(mut conn) = self.pool.get() {
                                let cur_ws = entry.get_snapshot();
                                for proposal in outcome.pending_patches() {
                                    let params = PersistProposalParams {
                                        draft_session_id,
                                        base_version_seq: cur_ws.head_version_seq,
                                        target_type: proposal.target_type.clone(),
                                        target_code: proposal.target_code.clone(),
                                        field_name: proposal.field_name.clone(),
                                        operation: proposal.operation.clone(),
                                        previous_value: proposal.previous_value.clone(),
                                        new_value: proposal.new_value.clone(),
                                        target_source: Some("user".to_string()),
                                        eligibility_json: None,
                                        require_new_origin: false,
                                    };
                                    if let Ok(res) = persist_proposal_tx(
                                        &mut conn,
                                        &params,
                                        Some(&cur_ws.business_flow_data),
                                    ) {
                                        entry.mutate_working_state(|ws| {
                                            ws.pending_patches.push(PendingPatchDto {
                                                id: res.patch_id,
                                                draft_session_id,
                                                target_code: proposal.target_code.clone(),
                                                field_path: proposal.field_name.clone().unwrap_or_default(),
                                                operation: proposal.operation.clone(),
                                                previous_value: proposal.previous_value.clone(),
                                                proposed_value: proposal
                                                    .new_value
                                                    .clone()
                                                    .unwrap_or(Value::Null),
                                                status: "pending".to_string(),
                                                reason: None,
                                                created_at: Utc::now().to_rfc3339(),
                                            });
                                        });
                                    }
                                }
                            }
                        }

                        (outcome, elapsed)
                    }
                };

                let outcome_status = outcome.status.clone();
                let outcome_json = serde_json::to_value(&outcome)
                    .unwrap_or_else(|_| serde_json::json!({ "status": outcome_status }));

                // Track consecutive rejections
                if outcome_status == "rejected" {
                    state.consecutive_rejected_count += 1;
                } else {
                    state.consecutive_rejected_count = 0;
                }

                // Record tool call audit
                state.all_tool_calls.push(ToolCallItemDto {
                    id: call.id.clone(),
                    name: tool_name.clone(),
                    parameters: call.arguments.clone(),
                    result: Some(outcome_json.clone()),
                    execution_time_ms: Some(execution_time_ms),
                    status: if outcome.is_applied() {
                        "success".to_string()
                    } else if outcome.is_rejected() {
                        "rejected".to_string()
                    } else {
                        "pending".to_string()
                    },
                });

                if let Some(tx) = stream_tx {
                    let _ = tx
                        .send(ChatStreamEvent::ToolCallCompleted {
                            turn_id,
                            tool_call_id: call.id.clone(),
                            tool_code: tool_name.clone(),
                            status: outcome_status,
                            has_applied_write: state.has_applied_write,
                        })
                        .await;

                    let cur_ws = entry.get_snapshot();
                    let _ = tx
                        .send(ChatStreamEvent::WorkingStateUpdated {
                            turn_id,
                            progress: compute_progress_dto(&cur_ws),
                            pending_patch_count: cur_ws.pending_patches.len(),
                        })
                        .await;
                }

                // Append observation message to conversation history
                messages.push(ProviderMessage::tool_result(
                    call.id.clone(),
                    outcome_json.to_string(),
                ));

                // Guardrail: Consecutive rejections exceeded limit -> abort remaining calls and stop loop
                if state.consecutive_rejected_count >= rejected_limit {
                    for skipped in remaining_calls {
                        let aborted_val =
                            serde_json::json!({ "status": "rejected", "error": "turn_aborted" });
                        messages.push(ProviderMessage::tool_result(
                            skipped.id.clone(),
                            aborted_val.to_string(),
                        ));
                        state.all_tool_calls.push(ToolCallItemDto {
                            id: skipped.id,
                            name: skipped.name,
                            parameters: skipped.arguments,
                            result: Some(aborted_val),
                            execution_time_ms: Some(0),
                            status: "rejected".to_string(),
                        });
                    }

                    // Final turn without tools: ask user for clarification in system_language
                    let final_req = ProviderChatRequest {
                        model: "chat".to_string(),
                        messages: messages.clone(),
                        tools: None,
                        temperature: None,
                        max_tokens: None,
                    };

                    if let Ok(final_resp) = provider.chat_with_tools(&final_req, cancel_token.clone()).await {
                        if let Some(txt) = final_resp.content {
                            if !txt.is_empty() {
                                state.assistant_text = txt;
                            }
                        }
                    }

                    if state.assistant_text.is_empty() {
                        state.assistant_text = if system_language == "vi" {
                            "Đã gặp lỗi liên tiếp khi thực thi công cụ. Vui lòng cung cấp thêm thông tin hoặc làm rõ yêu cầu để tiếp tục.".to_string()
                        } else {
                            "Encountered consecutive errors while executing tools. Please clarify your request to continue.".to_string()
                        };
                    }

                    return Ok(state.assistant_text.clone());
                }
            }
        }

        // Controlled exit when reaching max iterations
        if state.assistant_text.is_empty() {
            state.assistant_text = if system_language == "vi" {
                "Đã đạt giới hạn số lượt tương tác trong lượt này. Các thay đổi hợp lệ đã được lưu trữ.".to_string()
            } else {
                "Maximum iterations reached for this turn. Valid changes have been saved.".to_string()
            };
        }

        Ok(state.assistant_text.clone())
    }

    /// Durable version sealing under TX-01 (`seal_key = "turn:{turn_id}"`).
    fn seal_turn_version(
        &self,
        entry: &Arc<DraftRuntimeEntry>,
        draft_session_id: Uuid,
        turn_id: Uuid,
        user_message_id: Uuid,
    ) -> Result<DraftVersionSummaryDto, IpcError> {
        let ws = entry.get_snapshot();
        let data_json = serde_json::to_string(&ws.business_flow_data)
            .map_err(|e| IpcError::new(ErrorCode::ValidationFailed, e.to_string()))?;
        let data_hash = state_data_hash(&ws.business_flow_data)
            .map_err(|e| IpcError::new(ErrorCode::ValidationFailed, e.to_string()))?;

        let seal_params = SealVersionParams {
            draft_session_id,
            seal_key: format!("turn:{}", turn_id),
            reason: "ai_patch".to_string(),
            phase: ws.phase,
            subphase: ws.subphase,
            data_json,
            data_hash,
            parent_seq: ws.head_version_seq,
            turn_id: Some(turn_id),
            chat_message_id: Some(user_message_id),
        };

        self.draft_service.seal_version(&seal_params)
    }

    // ========================================================================
    // IPC #9: Phase Regeneration Boundary
    // ========================================================================

    /// Regenerates a draft phase under TX-01 (`reason = 'regenerate'`, `seal_key = "regenerate:{turn_id}"`).
    ///
    /// Invariant:
    /// - ONLY overwrites fields where `source == 'ai_generated'`.
    /// - STRICTLY preserves fields where `source == 'user'` or `'user_edited'`.
    pub async fn regenerate_draft_phase(
        &self,
        draft_session_id: Uuid,
        turn_id: Uuid,
        phase: i32,
        subphase: Option<i32>,
        candidate_override: Option<BusinessFlowData>,
    ) -> Result<DraftVersionSummaryDto, IpcError> {
        let entry = self
            .registry
            .get(&draft_session_id)
            .ok_or_else(|| IpcError::new(ErrorCode::EntityNotFound, "Draft session not in runtime"))?;

        let _draft_guard = entry
            .acquire_operation_lock("regenerate_draft_phase", self.config.draft_lock_timeout)
            .await?;

        // Selective field mutation respecting protected provenance
        entry.mutate_working_state(|ws| {
            if let Some(candidate) = candidate_override {
                Self::merge_phase_respecting_provenance(
                    &mut ws.business_flow_data,
                    &candidate,
                    phase,
                    subphase,
                );
            } else {
                Self::reset_ai_generated_phase_fields(
                    &mut ws.business_flow_data,
                    phase,
                    subphase,
                );
            }
            ws.phase = phase;
            ws.subphase = subphase;
        });

        let ws = entry.get_snapshot();
        let data_json = serde_json::to_string(&ws.business_flow_data)
            .map_err(|e| IpcError::new(ErrorCode::ValidationFailed, e.to_string()))?;
        let data_hash = state_data_hash(&ws.business_flow_data)
            .map_err(|e| IpcError::new(ErrorCode::ValidationFailed, e.to_string()))?;

        let seal_params = SealVersionParams {
            draft_session_id,
            seal_key: format!("regenerate:{}", turn_id),
            reason: "regenerate".to_string(),
            phase,
            subphase,
            data_json,
            data_hash,
            parent_seq: ws.head_version_seq,
            turn_id: Some(turn_id),
            chat_message_id: None,
        };

        self.draft_service.seal_version(&seal_params)
    }

    /// Selectively merges candidate phase data into current `BusinessFlowData`,
    /// strictly protecting user and user_edited fields.
    pub fn merge_phase_respecting_provenance(
        target: &mut BusinessFlowData,
        candidate: &BusinessFlowData,
        phase: i32,
        _subphase: Option<i32>,
    ) {
        match phase {
            0 => {
                // Project metadata
                if !target.require.project.display_name.source.unwrap_or(ProvenanceSource::Incomplete).is_protected_from_ai_overwrite() {
                    target.require.project.display_name = candidate.require.project.display_name.clone();
                }
                if !target.require.project.language.source.unwrap_or(ProvenanceSource::Incomplete).is_protected_from_ai_overwrite() {
                    target.require.project.language = candidate.require.project.language.clone();
                }
                if !target.require.project.target.word_per_chapter_count.source.unwrap_or(ProvenanceSource::Incomplete).is_protected_from_ai_overwrite() {
                    target.require.project.target.word_per_chapter_count = candidate.require.project.target.word_per_chapter_count.clone();
                }
                if !target.require.project.target.chapter_count.source.unwrap_or(ProvenanceSource::Incomplete).is_protected_from_ai_overwrite() {
                    target.require.project.target.chapter_count = candidate.require.project.target.chapter_count.clone();
                }

                // Characters: preserve user-created characters, update or add ai_generated ones
                for cand_char in &candidate.require.character {
                    if let Some(target_char) = target.require.character.iter_mut().find(|c| c.code == cand_char.code) {
                        if !target_char.display_name.source.unwrap_or(ProvenanceSource::Incomplete).is_protected_from_ai_overwrite() {
                            target_char.display_name = cand_char.display_name.clone();
                        }
                        if !target_char.role.source.unwrap_or(ProvenanceSource::Incomplete).is_protected_from_ai_overwrite() {
                            target_char.role = cand_char.role.clone();
                        }
                    } else {
                        target.require.character.push(cand_char.clone());
                    }
                }
            }
            1 => {
                // Overview phase
                for cand_arc in &candidate.optional.overview_phase.arcs {
                    if let Some(t_arc) = target.optional.overview_phase.arcs.iter_mut().find(|a| a.code == cand_arc.code) {
                        if !t_arc.display_name.source.unwrap_or(ProvenanceSource::Incomplete).is_protected_from_ai_overwrite() {
                            t_arc.display_name = cand_arc.display_name.clone();
                        }
                    } else {
                        target.optional.overview_phase.arcs.push(cand_arc.clone());
                    }
                }
            }
            _ => {}
        }
    }

    /// Resets all ai_generated fields in the phase, leaving user and user_edited fields intact.
    pub fn reset_ai_generated_phase_fields(
        target: &mut BusinessFlowData,
        phase: i32,
        _subphase: Option<i32>,
    ) {
        if phase == 0 {
            if target.require.project.display_name.source == Some(ProvenanceSource::AiGenerated) {
                target.require.project.display_name = crate::domain::business_flow::TrackedField::unset();
            }
            if target.require.project.target.word_per_chapter_count.source == Some(ProvenanceSource::AiGenerated) {
                target.require.project.target.word_per_chapter_count = crate::domain::business_flow::TrackedField::unset();
            }
            target.require.character.retain(|c| {
                c.display_name.source.map(|s| s.is_protected_from_ai_overwrite()).unwrap_or(false)
            });
        }
    }

    // ========================================================================
    // SQLite Persistence Helpers
    // ========================================================================

    pub fn save_message(
        &self,
        id: Uuid,
        session_id: &str,
        turn_id: Uuid,
        role: &str,
        content: &str,
        payload_json: Option<&Value>,
    ) -> Result<(), IpcError> {
        let conn = self
            .pool
            .get()
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

        let now = Utc::now().to_rfc3339();
        let payload_str = payload_json.map(|v| v.to_string());

        conn.execute(
            "INSERT INTO messages (id, session_id, turn_id, role, content, payload_json, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                id.to_string(),
                session_id,
                turn_id.to_string(),
                role,
                content,
                payload_str,
                now
            ],
        )
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

        Ok(())
    }

    pub fn fetch_recent_session_messages(
        &self,
        session_id: &str,
        exclude_id: Uuid,
        limit: usize,
    ) -> Result<Vec<ChatMessageDto>, IpcError> {
        let conn = self
            .pool
            .get()
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

        let mut stmt = conn
            .prepare(
                "SELECT id, role, content, payload_json, created_at
                 FROM messages
                 WHERE session_id = ?1 AND id != ?2
                 ORDER BY created_at ASC
                 LIMIT ?3",
            )
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

        let mut rows = stmt
            .query(params![session_id, exclude_id.to_string(), limit as i64])
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

        let mut items = Vec::new();
        while let Some(row) = rows
            .next()
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?
        {
            let id: String = row.get(0).unwrap_or_default();
            let role: String = row.get(1).unwrap_or_default();
            let content: String = row.get(2).unwrap_or_default();
            let payload_str: Option<String> = row.get(3).unwrap_or(None);
            let created_at: String = row.get(4).unwrap_or_default();

            let tool_calls = payload_str
                .as_deref()
                .and_then(|s| serde_json::from_str::<Value>(s).ok())
                .and_then(|v| v.get("tool_calls").cloned())
                .and_then(|v| serde_json::from_value::<Vec<ToolCallItemDto>>(v).ok());

            items.push(ChatMessageDto {
                id,
                role,
                content,
                thinking_seconds: None,
                thinking_details: None,
                is_thinking: None,
                tool_calls,
                timestamp: created_at,
            });
        }

        Ok(items)
    }
}
