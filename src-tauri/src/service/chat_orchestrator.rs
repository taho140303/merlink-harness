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

use crate::domain::business_flow::{BusinessFlowData, ProvenanceSource, TrackedField};
use crate::domain::fingerprint::state_data_hash;
use crate::domain::tools::registry::{CallerCategory, ToolRegistry, ToolType};
use crate::domain::tools::read_tools::{get_array_element_fields, get_field_null};
use crate::domain::tools::write_tools::{
    register_all_write_tools, ExecutionGuard, ToolCall, ToolExecutor, ToolOutcome,
};
use crate::error::{ErrorCode, IpcError};
use crate::ipc::dto::{
    ChatMessageDto, ChatStreamEvent, DraftVersionSummaryDto, PendingPatchDto,
    SendChatTurnResponseDto, ToolCallItemDto,
};
use crate::providers::traits::{
    LlmProvider, ProviderChatRequest, ProviderMessage, ProviderToolCall, ProviderToolDefinition,
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
            let Some(parameters) = schema_str.and_then(|s| serde_json::from_str::<Value>(&s).ok())
                .filter(|schema| schema["type"] == "object") else { continue; };
            let Some(description) = desc.filter(|text| !text.trim().is_empty()) else { continue; };

            tools.push(ProviderToolDefinition {
                name: code,
                description,
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

        let human_project_lang = match project_language.to_ascii_uppercase().as_str() {
            "VI" => "Tiếng Việt",
            "EN" => "English",
            "JA" => "日本語",
            "ZH_HANT" => "繁體中文",
            "ZH_HANS" => "简体中文",
            _ => project_language,
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

        Ok(RunSpec {
            prompt: blocks.join("\n\n"),
            tools,
        })
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
        use crate::domain::tools::registry::{AllowedCaller, SourceParamMode, ToolMetadata, ToolScope};
        tool_registry.register(ToolMetadata::new(
            "get_tool_definition", "Inspect a tool contract available in this run",
            ToolType::Read, ToolScope::System, AllowedCaller::AiOnly, SourceParamMode::None, false,
        ));

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
        let mut initialized_language = false;
        entry.mutate_working_state(|ws| {
            if ws.business_flow_data.require.project.language.value.is_none() {
                let value = if system_language.eq_ignore_ascii_case("en") { "EN" } else { "VI" };
                ws.business_flow_data.require.project.language =
                    TrackedField::new(value.to_string(), ProvenanceSource::System);
                initialized_language = true;
            }
        });
        let ws_snapshot = entry.get_snapshot();
        let project_lang = ws_snapshot
            .business_flow_data
            .require
            .project
            .language
            .value
            .as_deref()
            .unwrap_or("vi");

        let script = [
            "introduction",
            "invariant_rule",
            "planning_mode",
            "exception_handling",
            "out_of_scope",
            "tool_library",
            "story_context",
        ];
        let composed = self
            .composer
            .compose(&script, system_language, project_lang)?;

        // 3. Assemble conversation history for provider request
        let mut messages = vec![ProviderMessage::system(composed.prompt.clone())];
        // Replay complete tool-call/result pairs so failed writes are not mistaken for saved facts.
        let history = self.fetch_recent_session_messages(session_id, user_message_id, 10)?;
        for hist_msg in history {
            append_chat_history(&mut messages, hist_msg);
        }

        // Append current user goal
        messages.push(ProviderMessage::user(user_prompt.to_string()));

        // 4. Run ReAct Turn Loop
        let mut state = TurnState {
            has_applied_write: initialized_language,
            ..TurnState::default()
        };
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
                // If error occurred but we already have applied mutations, still seal TX-01!
                if state.has_applied_write {
                    let _ = self.seal_turn_version(&entry, draft_session_id, turn_id, user_message_id);
                }
                // Keep the tool audit even when the next provider request fails.
                // The caller removes the user message only when no assistant audit exists.
                if !state.all_tool_calls.is_empty() {
                    let audit_text = if state.assistant_text.is_empty() {
                        if system_language == "vi" {
                            "Lượt trò chuyện bị gián đoạn sau khi gọi công cụ."
                        } else {
                            "The conversation was interrupted after a tool call."
                        }
                    } else {
                        &state.assistant_text
                    };
                    self.save_message(
                        Uuid::new_v4(), session_id, turn_id, "assistant", audit_text,
                        Some(&serde_json::json!({
                            "tool_calls": state.all_tool_calls,
                            "turn_error": err.code,
                        })),
                    )?;
                }
                if let Some(ref tx) = stream_tx {
                    let _ = tx.send(ChatStreamEvent::TurnError { turn_id, error: err.clone() }).await;
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

            let current = entry.get_snapshot();
            messages[0].content = render_chat_prompt(
                &composed.prompt, &current.business_flow_data, &current.pending_patches,
            );
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

            if resp.tool_calls.is_empty() {
                // Only the final answer is user-facing. Tool-step narration can claim
                // success before execution, and concatenating it makes the reply unreadable.
                state.assistant_text = resp.content.unwrap_or_default();
                if !state.assistant_text.is_empty() {
                    if let Some(tx) = stream_tx {
                        let _ = tx.send(ChatStreamEvent::AssistantDelta {
                            turn_id, delta: state.assistant_text.clone(),
                        }).await;
                    }
                }
                return Ok(state.assistant_text.clone());
            }

            // Append assistant tool-calling message to history
            messages.push(ProviderMessage {
                role: "assistant".to_string(),
                content: resp.content.clone().unwrap_or_default(),
                name: None,
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
                let tool_meta = if composed.tools.iter().any(|tool| tool.name == tool_name) {
                    self.tool_registry.authorize(&tool_name, CallerCategory::AiCaller)
                } else {
                    Err(ErrorCode::UnauthorizedCaller)
                };

                let (outcome, execution_time_ms, read_result) = match tool_meta {
                    Err(ErrorCode::UnauthorizedCaller) => (
                        ToolOutcome::rejected("unauthorized_caller"),
                        start_time.elapsed().as_millis() as u64,
                        None,
                    ),
                    Err(_) => (
                        ToolOutcome::rejected("tool_not_found"),
                        start_time.elapsed().as_millis() as u64,
                        None,
                    ),
                    Ok(meta) => {
                        if meta.tool_type == ToolType::Read {
                            let snap = entry.get_snapshot();
                            let result = if tool_name == "get_tool_definition" {
                                tool_definition_result(&call.arguments, &composed.tools)
                            } else {
                                read_tool_result(&tool_name, &call.arguments, &snap.business_flow_data, system_language)
                            };
                            let status = result["status"].as_str().unwrap_or("rejected").to_string();
                            (
                                ToolOutcome { status, results: Vec::new(), authenticated_effect: None },
                                start_time.elapsed().as_millis() as u64,
                                Some(result),
                            )
                        } else {
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

                        (outcome, elapsed, None)
                        }
                    }
                };

                let outcome_status = outcome.status.clone();
                let outcome_json = read_result.unwrap_or_else(||
                    serde_json::to_value(&outcome)
                        .unwrap_or_else(|_| serde_json::json!({ "status": outcome_status }))
                );

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
                    tool_name.clone(),
                    model_tool_result(&outcome_json).to_string(),
                ));

                // Guardrail: Consecutive rejections exceeded limit -> abort remaining calls and stop loop
                if state.consecutive_rejected_count >= rejected_limit {
                    for skipped in remaining_calls {
                        let aborted_val =
                            serde_json::json!({ "status": "rejected", "error": "turn_aborted" });
                        messages.push(ProviderMessage::tool_result(
                            skipped.id.clone(),
                            skipped.name.clone(),
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

                    // Report what was actually saved; a tool fault must not become a repeated data question.
                    let current = entry.get_snapshot();
                    messages[0].content = render_chat_prompt(&composed.prompt, &current.business_flow_data, &current.pending_patches);
                    messages.push(ProviderMessage::system(
                        "Tool execution stopped. Briefly distinguish saved changes from unsaved ones. Do not ask the user to repeat facts already supplied. Reply in the configured communication language."
                    ));
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
                            "Mình chưa lưu được một phần thay đổi. Thông tin bạn đã gửi vẫn còn trong cuộc trò chuyện; bạn không cần nhập lại.".to_string()
                        } else {
                            "Some changes could not be saved. Your information remains in the conversation; you do not need to enter it again.".to_string()
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

        if role == "user" {
            // The sidebar labels a conversation by its opening message. Only
            // replace a placeholder name, so later turns never rename the chat.
            let title = session_title_from_message(content);
            if !title.is_empty() {
                conn.execute(
                    "UPDATE sessions SET display_name = ?1
                     WHERE id = ?2
                       AND (display_name IS NULL OR TRIM(display_name) = ''
                            OR display_name IN ('New Session', 'Active Session'))",
                    params![title, session_id],
                )
                .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
            }
        }

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
                 FROM (
                   SELECT rowid AS message_order, id, role, content, payload_json, created_at
                   FROM messages
                   WHERE session_id = ?1 AND id != ?2
                   ORDER BY created_at DESC, rowid DESC
                   LIMIT ?3
                 )
                 ORDER BY created_at ASC, message_order ASC",
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

/// Builds a conversation title from the first user message: whitespace is
/// collapsed and at most 10 characters are kept, as required by the sidebar UI.
fn session_title_from_message(content: &str) -> String {
    content.split_whitespace().collect::<Vec<_>>().join(" ").chars().take(10).collect()
}

/// Keep backend mutation evidence in the audit, not in the model context.
fn model_tool_result(result: &Value) -> Value {
    let mut result = result.clone();
    if let Some(object) = result.as_object_mut() {
        if let Some(effect) = object.remove("authenticated_effect") {
            if !effect["cascade_preview"].is_null() {
                object.insert("cascade_preview".into(), effect["cascade_preview"].clone());
            }
        }
        if object.get("status").and_then(Value::as_str) == Some("rejected")
            && !object.contains_key("error")
            && object.get("results").and_then(Value::as_array).map_or(true, |results| results.is_empty()) {
            object.insert("error".into(), Value::String("missing_error_details".into()));
            object.insert("recovery".into(), Value::String("Inspect get_tool_definition before retrying; do not repeat unchanged arguments or ask for known user values.".into()));
        }
    }
    result
}

fn tool_definition_result(args: &Value, tools: &[ProviderToolDefinition]) -> Value {
    let name = args.get("tool_name").and_then(Value::as_str);
    match tools.iter().find(|tool| Some(tool.name.as_str()) == name) {
        Some(tool) => serde_json::json!({
            "status": "applied", "tool_name": tool.name, "description": tool.description,
            "input_schema": tool.parameters,
            "examples": tool.parameters.get("examples").cloned().unwrap_or_else(|| serde_json::json!([])),
        }),
        None => serde_json::json!({"status":"rejected", "error":"tool_not_available",
            "available_tools": tools.iter().map(|tool| &tool.name).collect::<Vec<_>>() }),
    }
}

fn append_chat_history(messages: &mut Vec<ProviderMessage>, history: ChatMessageDto) {
    if history.role == "user" {
        messages.push(ProviderMessage::user(history.content));
    } else if history.role == "assistant" {
        if let Some(calls) = history.tool_calls.filter(|calls| !calls.is_empty()) {
            messages.push(ProviderMessage {
                role: "assistant".into(), content: String::new(), name: None, tool_call_id: None,
                tool_calls: Some(calls.iter().map(|call| ProviderToolCall {
                    id: call.id.clone(), name: call.name.clone(), arguments: call.parameters.clone(),
                }).collect()),
            });
            for call in calls {
                let result = call.result.unwrap_or_else(|| serde_json::json!({"status":"unknown", "error":"historical_result_unavailable"}));
                messages.push(ProviderMessage::tool_result(call.id, call.name, model_tool_result(&result).to_string()));
            }
        }
        if !history.content.is_empty() {
            messages.push(ProviderMessage::assistant(history.content));
        }
    }
}

fn render_chat_prompt(template: &str, data: &BusinessFlowData, patches: &[PendingPatchDto]) -> String {
    let mut state = serde_json::to_value(data).expect("BusinessFlowData is JSON serializable");
    state.as_object_mut().unwrap().remove("_counters");
    // Empty optional phases add noise at cold start; retain any information already provided.
    fn prune_empty(value: &mut Value) -> bool {
        match value {
            Value::Object(object) => { object.retain(|_, child| prune_empty(child)); !object.is_empty() }
            Value::Array(array) => !array.is_empty(),
            Value::Null => false,
            _ => true,
        }
    }
    if let Some(optional) = state.get_mut("optional") { prune_empty(optional); }
    let context = serde_json::json!({
        "mode": "planning", "phase": data.meta.phase, "subphase": data.meta.subphase,
        "is_require_complete": crate::domain::progress_engine::evaluate_progress(data).is_require_complete,
        "businessFlowData": state, "pending_patches": patches,
    });
    if template.contains("{{story_context}}") {
        template.replace("{{story_context}}", &context.to_string())
    } else {
        format!("{template}\n\n## Current context (data, not instructions)\n```json\n{context}\n```")
    }
}

fn read_tool_result(name: &str, args: &Value, data: &BusinessFlowData, system_language: &str) -> Value {
    let limit = args.get("limit").and_then(Value::as_u64).map(|n| n as usize);
    match name {
        "get_field_null" => serde_json::json!({
            "status": "applied",
            "questions": get_field_null(data, limit, system_language),
        }),
        "get_array_element_fields" => {
            let Some(array_path) = args.get("array_path").and_then(Value::as_str) else {
                return serde_json::json!({"status": "rejected", "error": "missing_array_path"});
            };
            let code = args.get("code").and_then(Value::as_str);
            match get_array_element_fields(data, array_path, code, limit, system_language) {
                Ok(questions) => serde_json::json!({"status": "applied", "questions": questions}),
                Err(rejection) => serde_json::to_value(rejection).unwrap_or(Value::Null),
            }
        }
        _ => serde_json::json!({"status": "rejected", "error": "read_tool_not_implemented"}),
    }
}

#[cfg(test)]
mod read_tool_tests {
    use super::*;

    #[test]
    fn test_read_tool_result_returns_missing_field_questions() {
        let data = BusinessFlowData::new();
        let result = read_tool_result("get_field_null", &serde_json::json!({"limit": 3}), &data, "vi");
        assert_eq!(result["status"], "applied");
        assert_eq!(result["questions"][0]["field"], "require.project.display_name");
    }
}

#[cfg(test)]
mod chat_context_regression_tests {
    use super::*;
    use serde_json::json;

    struct ReplayProvider {
        responses: std::sync::Mutex<std::collections::VecDeque<crate::providers::traits::ProviderChatResponse>>,
        requests: std::sync::Mutex<Vec<ProviderChatRequest>>,
    }

    #[async_trait::async_trait]
    impl LlmProvider for ReplayProvider {
        fn provider_id(&self) -> &'static str { "regression-test" }
        async fn chat_with_tools(&self, request: &ProviderChatRequest, _: CancellationToken)
            -> Result<crate::providers::traits::ProviderChatResponse, crate::providers::traits::ProviderError> {
            self.requests.lock().unwrap().push(request.clone());
            Ok(self.responses.lock().unwrap().pop_front().expect("unexpected extra request"))
        }
        async fn stream_completion(&self, _: &crate::providers::traits::ProviderStreamRequest,
            _: tokio::sync::mpsc::Sender<crate::providers::traits::ProviderStreamChunk>, _: CancellationToken)
            -> Result<crate::providers::traits::ProviderStreamSummary, crate::providers::traits::ProviderError> {
            unreachable!("planning does not stream scene content")
        }
    }

    #[tokio::test]
    async fn test_chat_recovers_tool_error_and_refreshes_context_before_questions() {
        use crate::providers::traits::ProviderChatResponse;
        let tmp = tempfile::tempdir().unwrap();
        let pool = Pool::builder().max_size(2).build(SqliteConnectionManager::file(tmp.path().join("chat.db"))).unwrap();
        crate::storage::migration::run_migrations(&mut pool.get().unwrap()).unwrap();
        let registry = Arc::new(DraftRuntimeRegistry::new());
        let draft_service = Arc::new(DraftService::new(pool.clone(), registry.clone()));
        let patch_service = Arc::new(PatchService::new(pool.clone(), registry.clone(), draft_service.clone()));
        let orchestrator = ChatOrchestrator::new(pool.clone(), registry.clone(), draft_service, patch_service);
        let id = Uuid::new_v4();
        registry.get_or_register(id, crate::service::draft_runtime::WorkingState::new(id));
        let entry = registry.get(&id).unwrap();
        let composed = orchestrator.composer.compose(&["introduction", "invariant_rule", "planning_mode", "exception_handling", "tool_library", "story_context"], "vi", "VI").unwrap();
        let call = |name: &str, args: Value, index: usize| ProviderChatResponse {
            content: Some("Thông báo tạm trước khi biết kết quả tool".into()),
            tool_calls: vec![ProviderToolCall { id: format!("call-{index}"), name: name.into(), arguments: args }], ..Default::default()
        };
        let provider = ReplayProvider { requests: Default::default(), responses: std::sync::Mutex::new([
            call("set_project", json!({"target":{"chapter_count":120}}), 1),
            call("get_tool_definition", json!({"tool_name":"set_project"}), 2),
            call("set_project", json!({"target":{"arc_count":1,"chapter_count":120,"word_per_chapter_count":2400},
                "declared_sources":{"target.arc_count":"user","target.chapter_count":"user","target.word_per_chapter_count":"user"}}), 3),
            call("get_field_null", json!({"limit":3}), 4),
            ProviderChatResponse { content: Some("Đã lưu các mục tiêu bạn đưa ra.".into()), ..Default::default() },
        ].into()) };
        let mut messages = vec![ProviderMessage::system(composed.prompt.clone()), ProviderMessage::user("1 Arc, 120 chương, mỗi chương 2400 từ")];
        let mut state = TurnState::default();
        let (tx, mut rx) = tokio::sync::mpsc::channel(32);
        orchestrator.run_turn_loop(&entry, id, Uuid::new_v4(), &composed, &mut messages, &mut state, "vi", &provider, Some(&tx), CancellationToken::new()).await.unwrap();
        let mut visible_text = Vec::new();
        while let Ok(event) = rx.try_recv() {
            if let ChatStreamEvent::AssistantDelta { delta, .. } = event { visible_text.push(delta); }
        }
        assert_eq!(visible_text, vec!["Đã lưu các mục tiêu bạn đưa ra."]);
        assert_eq!(state.all_tool_calls[0].result.as_ref().unwrap()["results"][0]["error"], "missing_or_invalid_declared_source");
        assert_eq!(state.all_tool_calls[1].result.as_ref().unwrap()["input_schema"]["properties"]["target"]["type"], "object");
        assert_eq!(entry.get_snapshot().business_flow_data.require.project.target.chapter_count.value, Some(120));
        assert!(state.has_applied_write);
        let questions = &state.all_tool_calls[3].result.as_ref().unwrap()["questions"];
        assert!(!questions.to_string().contains("target."));
        let requests = provider.requests.lock().unwrap();
        assert!(requests[3].messages[0].content.contains("2400"));
        assert!(!requests[3].messages[0].content.contains("{{story_context}}"));
        assert!(requests.iter().flat_map(|r| &r.messages).filter(|m| m.role == "tool").all(|m| !m.content.contains("authenticated_effect")));
        assert!(orchestrator.composer.compose(&["out_of_scope"], "vi", "VI").unwrap().tools.is_empty());
    }

    #[test]
    fn test_tool_help_returns_only_current_run_contract() {
        let tools = vec![ProviderToolDefinition { name: "set_project".into(), description: "Update project targets".into(),
            parameters: json!({"type":"object", "examples":[{"target":{"chapter_count":120},"declared_sources":{"target.chapter_count":"user"}}]}) }];
        let result = tool_definition_result(&json!({"tool_name":"set_project"}), &tools);
        assert_eq!(result["status"], "applied");
        assert_eq!(result["input_schema"], tools[0].parameters);
        assert_eq!(result["examples"][0]["target"]["chapter_count"], 120);
        assert_eq!(tool_definition_result(&json!({"tool_name":"commit_draft"}), &tools)["error"], "tool_not_available");
    }

    #[test]
    fn test_model_observation_keeps_field_errors_without_internal_state() {
        let result = model_tool_result(&json!({"status":"applied", "results":[{"field":"summary", "status":"rejected", "error":"missing_or_invalid_declared_source"}],
            "authenticated_effect":{"mutated_state":{"_counters":{"secret":1}},"pending_patches":[],"has_mutations":true}}));
        assert_eq!(result["results"][0]["error"], "missing_or_invalid_declared_source");
        assert!(result.get("authenticated_effect").is_none());
        assert_eq!(model_tool_result(&json!({"status":"rejected","results":[]}))["error"], "missing_error_details");
    }

    #[test]
    fn test_history_replays_tool_result_before_final_answer() {
        let message: ChatMessageDto = serde_json::from_value(json!({"id":"msg", "role":"assistant", "content":"Chưa lưu mục tiêu.", "timestamp":"now",
            "toolCalls":[{"id":"call-1","name":"set_project","parameters":{"target":{"chapter_count":120}},
                "result":{"status":"rejected","results":[]},"status":"rejected"}]})).unwrap();
        let mut history = Vec::new();
        append_chat_history(&mut history, message);
        assert_eq!(history.iter().map(|m| m.role.as_str()).collect::<Vec<_>>(), vec!["assistant","tool","assistant"]);
        assert_eq!(history[0].tool_calls.as_ref().unwrap()[0].id, "call-1");
        assert_eq!(history[1].tool_call_id.as_deref(), Some("call-1"));
        assert!(history[1].content.contains("rejected"));
        assert_eq!(history[2].content, "Chưa lưu mục tiêu.");
    }

    #[test]
    fn test_runtime_context_refreshes_targets_and_keeps_user_context() {
        let mut data = BusinessFlowData::new();
        let before = render_chat_prompt("## Context\n```json\n{{story_context}}\n```", &data, &[]);
        data.require.project.target.chapter_count = TrackedField::user(120);
        let after = render_chat_prompt("## Context\n```json\n{{story_context}}\n```", &data, &[]);
        assert!(!after.contains("{{story_context}}"));
        assert!(!after.contains("_counters"));
        assert!(after.contains("120"));
        assert!(after.contains("require"));
        assert_ne!(before, after);
        let custom = render_chat_prompt("Custom rules", &data, &[]);
        assert!(custom.starts_with("Custom rules"));
        assert!(custom.contains("120"));
    }
}
