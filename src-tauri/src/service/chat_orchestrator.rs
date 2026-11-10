//! Chat & Prompt Orchestrator implementing ReAct single-turn loop,
//! prompt composition, and version sealing boundaries (§6.1, `docs/business/prompts/overview.md`, `core.md` §3.8).
//!
//! Enforces:
//! 1. Prompt composer (§6.1): Distinguishes `system_language` (UI communication language, "vi" | "en")
//!    from `project.language` (draft creative content language). Injects prompt blocks JIT.
//! 2. ReAct turn loop (`run_turn_loop`):
//!    - Strictly NO SQLite transaction held across network await (`CI-03`).
//!    - Max iterations limit (default 25, within the 20..=30 range of `be-core.md` §6.1) and consecutive rejections limit (3 consecutive rejections
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
use crate::service::chat_catalog;
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
    /// Number of most recent author messages replayed into the model context per turn.
    pub history_user_messages: usize,
    /// Number of most recent assistant messages replayed into the model context per turn.
    pub history_assistant_messages: usize,
    /// Character budget for replayed historical tool payloads. Older assistant messages keep
    /// their text but drop tool calls once the budget is spent, so long sessions stay in-context.
    pub history_tool_payload_budget: usize,
}

impl Default for OrchestratorConfig {
    fn default() -> Self {
        Self {
            max_iterations: 25,
            consecutive_rejected_limit: 3,
            draft_lock_timeout: DRAFT_LOCK_TIMEOUT,
            history_user_messages: 20,
            history_assistant_messages: 20,
            history_tool_payload_budget: 120_000,
        }
    }
}


/// Field-level parameter guide per phase. Mirrors `WorkflowService::phase_tool_guidance`.
/// The chat path needs the guide because the JSON schema stays permissive; tests compile this
/// module without `workflow_service`, so the strings are kept local on purpose.
fn phase_tool_guidance(phase: i32) -> &'static str {
    match phase {
        1 => "Tool guide: add_arc(code?, display_name, summary, position, declared_sources); set_arc(code, display_name?, summary?, position?, declared_sources); set_arc_motif(code, motif_code?, note?, declared_sources); add_arc_event_block(arc_code, code?, event_block_code?, start_chapter?, end_chapter?, summary?, declared_sources); add_theme(arc_code, code?, parent_theme_code?, question?, resolution_type?, resolution?, message?, declared_sources); add_theme_argument(theme_code, character_code?, role=PRIMARY|ANTITHESIS|CAUTIONARY|WITNESS, answer_mode, note?, declared_sources). Target Arc count: use require.project.target.arc_count exactly. Phase gate: every Arc must have motif_code, at least one event block, and theme question + message, otherwise the transition to Phase 2 is blocked.",
        2 => "Tool guide: add_chapter(arc_code, code?, display_name, position, summary, declared_sources); set_chapter(code, display_name?, position?, summary?, declared_sources); add_scene(chapter_code, code?, display_name, position, summary, declared_sources). chapter.position is unique across the whole project. Total chapters must equal require.project.target.chapter_count. Phase gate: every chapter needs display_name, summary and at least one scene with display_name/summary.",
        3 => "Tool guide: add_world_setting(code?, parent_code?, display_name, type=REALM|WORLD|REGION|COUNTRY|CITY|TOWN|FACILITY|STREET|ROOM, description?, access?, declared_sources); add_lore_entry(code?, world_setting_code, display_name, type=CULTURE|HISTORY|FACTION|ITEM, description?, data?, declared_sources). Write parents before children. Phase gate: at least one world setting with display_name and type; parent_code must reference an existing setting.",
        4 => "Tool guide: set_scene_config(scene_code, style_preset_code?, narrator_config_code?, world_setting_code?, declared_sources); add_scene_character(scene_code, character_code, is_pov?, declared_sources); add_scene_lore_entry(scene_code, lore_entry_code, usage?, reveal_to_reader?, reveal_to_pov?, declared_sources). Limited narrators need exactly one is_pov; omniscient needs none. Phase gate: every scene needs world_setting_code, narrator_config_code and style_preset_code that reference existing values.",
        5 => "Tool guide: add_character_appearance(character_code, species?, gender_identity?, height?, weight?, hair_color_code?, skin_tone_code?, outfit_style?, default_summary?, declared_sources); add_character_psychological(character_code, the_wound?, the_lie?, fundamental_fear?, moral_alignment?, moral_code?, cognitive_bias?, breaking_point?, external_want?, internal_need?, internal_conflict?, social_mask_persona_type?, social_mask_verbal_style?, defense_primary_reaction?, defense_stress_trigger?, defense_coping_strategy?, declared_sources); add_character_memory(character_code, code?, display_name?, summary?, source_scene_code?, declared_sources); add_character_competence(character_code, code?, domain_code?, level_code?, scene_code, capabilities?, limitations?, declared_sources). Competence scene_code is mandatory and must reference an existing scene. Phase gate: every character needs appearance (default_summary or species), psychological (the_wound/external_want/internal_need) and at least one memory or competence bound to an existing scene.",
        6 => "Tool guide: add_relationship(code?, from_code, to_code, motif_code?, declared_sources); add_relationship_role(relationship_code, category_code?, hierarchy?, is_primary?, declared_sources); set_relationship_state(relationship_code, scene_code, trust?, affection?, hostility?, fear?, respect?, familiarity?, dependency? with NONE|LOW|MODERATE|HIGH|ABSOLUTE, declared_sources). Phase gate: with two or more characters, at least one relationship is required and from_code/to_code must reference existing characters.",
        _ => "",
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
        // The UI communication language is not an author-confirmed story language.
        // Keep project.language unset until the author specifies it explicitly.
        let ws_snapshot = entry.get_snapshot();
        let project_lang = ws_snapshot
            .business_flow_data
            .require
            .project
            .language
            .value
            .as_deref()
            .unwrap_or("vi");

        // Phase-aware composition (`core.md` §2, §3.8): the current workspace phase decides which
        // planning prompt block is composed. Before this, the script hardcoded `planning_phase_0`
        // for every turn, so phases 1-6 kept Phase 0 instructions ("không sinh nội dung phái sinh
        // của phase chưa mở") and the model never touched the optional-phase tools.
        // `WorkingState.phase` is the runtime phase sealed by workflow/confirm; the JSON
        // `meta.phase` inside the snapshot is not advanced by workflow rounds and would pin chat
        // at Phase 0 even when Phase 1 data already exists.
        let ws_phase = ws_snapshot.phase.clamp(0, 6);
        let phase_block = match ws_phase {
            1 => "planning_phase_1",
            2 => "planning_phase_2",
            3 => "planning_phase_3",
            4 => "planning_phase_4",
            5 => "planning_phase_5",
            6 => "planning_phase_6",
            _ => "planning_phase_0",
        };
        let script = [
            "introduction",
            "invariant_rule",
            "planning_mode",
            phase_block,
            "exception_handling",
            "out_of_scope",
            "tool_library",
            "story_context",
        ];
        let mut composed = self
            .composer
            .compose(&script, system_language, project_lang)?;
        if ws_phase == 0 {
            composed.prompt.push_str("\n\n## Phase 0 collection\nThe JSON business state in context is the single source of truth (SSOT). Never assume a write succeeded because a tool was called: read the tool result (status, error) and re-check the latest state. If get_field_null reports empty_arrays/array_questions, or the JSON still has an empty required array or null required field, ask the author again in this same turn; do not claim the value was saved and do not move on.\nAfter saving facts from the current user turn, check the latest get_field_null and get_array_element_fields observations. If Phase 0 remains incomplete, ask relevant fixed questions from those observations (up to three total), rather than claiming it is complete. A system-sourced project language is only a UI default: ask the author for the story language. Empty project.genres and missing character motif_code require separate array questions. Resolve genre and character_arc motif names to catalog codes using search_genre/search_motif before writing; never invent codes or assert a user-selected genre/motif without their confirmation. Optional target tolerances are not required for Phase 0; save them when provided, including zero.\n");
        } else {
            // The JSON schema stays permissive on purpose; the field-level guide supplies the
            // documented parameter names and enums for the phase (same guide the workflow uses).
            let guidance = phase_tool_guidance(ws_phase);
            if !guidance.is_empty() {
                composed.prompt.push_str("\n\n## Phase tool guide\n");
                composed.prompt.push_str(guidance);
                composed.prompt.push('\n');
            }
        }
        if !composed.tools.iter().any(|tool| tool.name == "get_field_null") {
            return Err(IpcError::new(ErrorCode::ValidationFailed,
                "Chat prompt is missing the required get_field_null tool"));
        }

        // 3. Assemble conversation history for provider request
        let mut messages = vec![ProviderMessage::system(composed.prompt.clone())];
        // Replay complete tool-call/result pairs so failed writes are not mistaken for saved facts.
        // Per-role history window: keep the last 20 author messages and the last 20 assistant
        // messages so early instructions survive tool-heavy turns (a flat `LIMIT 10` silently
        // dropped the beginning of the session). Historical tool payloads are bounded by
        // `history_tool_payload_budget`; older assistant text is always retained.
        let history = self.fetch_recent_role_window(
            session_id,
            user_message_id,
            self.config.history_user_messages,
            self.config.history_assistant_messages,
        )?;
        let replay_flags = replay_tool_flags(&history, self.config.history_tool_payload_budget);
        for (hist_msg, replay_tools) in history.into_iter().zip(replay_flags) {
            append_chat_history(&mut messages, hist_msg, replay_tools);
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
        // Fixed read observations: exactly one `get_field_null` call and one
        // `get_array_element_fields` call per turn, appended after the user message (before the
        // first model request). Their parameters are stable, so the observations are refreshed in
        // place whenever the working state changes instead of appending new call pairs: a
        // per-change injection repeated up to MAX_ITER times and inflated the context (28 calls,
        // 19 of them backend-injected, in one recorded turn).
        let mut injected_reads: Option<InjectedReads> = None;
        let initial = entry.get_snapshot();
        if composed.tools.iter().any(|tool| tool.name == "get_field_null") {
            injected_reads = push_injected_reads(
                messages,
                &mut state.all_tool_calls,
                &initial.business_flow_data,
                system_language,
                composed.tools.iter().any(|tool| tool.name == "get_array_element_fields"),
                turn_id,
                stream_tx,
            );
        }
        let mut last_null_field_state: Option<BusinessFlowData> = None;

        for _ in 0..max_iter {
            if cancel_token.is_cancelled() {
                return Ok("Turn cancelled by user".to_string());
            }

            let current = entry.get_snapshot();
            if let Some(ref injected) = injected_reads {
                if last_null_field_state.as_ref() != Some(&current.business_flow_data) {
                    // Keep the two fixed observations current without growing the message list.
                    injected.refresh(
                        messages,
                        &mut state.all_tool_calls,
                        &current.business_flow_data,
                        system_language,
                    );
                    last_null_field_state = Some(current.business_flow_data.clone());
                }
            }
            messages[0].content = render_chat_prompt_with_phase(
                &composed.prompt,
                &current.business_flow_data,
                &current.pending_patches,
                current.phase,
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
                            } else if tool_name == "search_genre" || tool_name == "search_motif" {
                                match self.pool.get() {
                                    Ok(conn) => {
                                        let lookup = if tool_name == "search_genre" {
                                            chat_catalog::search_genre(&conn, &call.arguments)
                                        } else {
                                            chat_catalog::search_motif(&conn, &call.arguments)
                                        };
                                        lookup.unwrap_or_else(|_| serde_json::json!({"status":"rejected","error":"catalog_lookup_failed"}))
                                    }
                                    Err(_) => serde_json::json!({"status":"rejected","error":"catalog_unavailable"}),
                                }
                            } else if tool_name.starts_with("search_") {
                                match self.pool.get() {
                                    Ok(conn) => chat_catalog::search_catalog(&conn, &tool_name, &call.arguments)
                                        .unwrap_or_else(|_| serde_json::json!({"status":"rejected","error":"catalog_lookup_failed"})),
                                    Err(_) => serde_json::json!({"status":"rejected","error":"catalog_unavailable"}),
                                }
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
                        // Reference codes must exist in the seeded catalog before a write.
                        let catalog_field = match tool_name.as_str() {
                            "add_project_genre" => Some(("genre_code", false)),
                            "add_character" | "set_character" if call.arguments.get("motif_code").is_some() => Some(("motif_code", true)),
                            _ => None,
                        };
                        if let Some((field, is_motif)) = catalog_field {
                            if let Some(code) = call.arguments.get(field).and_then(Value::as_str) {
                                let valid = self.pool.get().ok().and_then(|conn| {
                                    let result = if is_motif {
                                        chat_catalog::validate_motif_code(&conn, code)
                                    } else {
                                        chat_catalog::validate_genre_code(&conn, code)
                                    };
                                    result.ok()
                                }).unwrap_or(false);
                                if !valid {
                                    let result = serde_json::json!({"status":"rejected", "error":"unknown_catalog_code", "field":field,
                                        "recovery": if is_motif {"Search character_arc motifs with search_motif before retrying."} else {"Search genres with search_genre before retrying."}});
                                    (ToolOutcome::rejected("unknown_catalog_code"), start_time.elapsed().as_millis() as u64, Some(result))
                                } else {
                                    self.execute_chat_mutation(&entry, draft_session_id, &call, &tool_name, meta.writes_draft_state, state, start_time)
                                }
                            } else {
                                self.execute_chat_mutation(&entry, draft_session_id, &call, &tool_name, meta.writes_draft_state, state, start_time)
                            }
                        } else {
                            self.execute_chat_mutation(&entry, draft_session_id, &call, &tool_name, meta.writes_draft_state, state, start_time)
                        }
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
                    messages[0].content = render_chat_prompt_with_phase(&composed.prompt, &current.business_flow_data, &current.pending_patches, current.phase);
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
                format!(
                    "Mình đã dừng lượt này sau {max_iter} vòng lặp để tránh lặp vô hạn. Các thay đổi hợp lệ đã được lưu trữ; bạn nhắn tiếp để mình làm tiếp phần còn thiếu."
                )
            } else {
                format!(
                    "This turn stopped after {max_iter} iterations to avoid looping. Valid changes have been saved; send another message to continue with the remaining work."
                )
            };
        }

        Ok(state.assistant_text.clone())
    }

    fn execute_chat_mutation(
        &self,
        entry: &Arc<DraftRuntimeEntry>,
        draft_session_id: Uuid,
        call: &ProviderToolCall,
        tool_name: &str,
        writes_draft_state: bool,
        state: &mut TurnState,
        started: std::time::Instant,
    ) -> (ToolOutcome, u64, Option<Value>) {
        let snapshot = entry.get_snapshot();
        let guard = ExecutionGuard::with_session(&snapshot.business_flow_data, draft_session_id.to_string());
        let mut request = ToolCall::new(tool_name.to_string(), CallerCategory::AiCaller, call.arguments.clone());
        if let Some(map) = call.arguments.get("declared_sources").and_then(Value::as_object) {
            let sources = map.iter().filter_map(|(name, value)|
                serde_json::from_value::<ProvenanceSource>(value.clone()).ok().map(|source| (name.clone(), source))
            ).collect();
            request.declared_sources = Some(sources);
        } else if let Some(value) = call.arguments.get("declared_source") {
            request.declared_source = serde_json::from_value(value.clone()).ok();
        }
        let outcome = ToolExecutor::execute(&request, &guard);
        if writes_draft_state && outcome.is_applied() {
            if let Some(mutated) = outcome.mutated_state() {
                entry.mutate_working_state(|ws| ws.business_flow_data = mutated.clone());
                state.has_applied_write = true;
            }
        }
        if !outcome.pending_patches().is_empty() {
            if let Ok(mut conn) = self.pool.get() {
                let current = entry.get_snapshot();
                for proposal in outcome.pending_patches() {
                    let params = PersistProposalParams {
                        draft_session_id, base_version_seq: current.head_version_seq,
                        target_type: proposal.target_type.clone(), target_code: proposal.target_code.clone(),
                        field_name: proposal.field_name.clone(), operation: proposal.operation.clone(),
                        previous_value: proposal.previous_value.clone(), new_value: proposal.new_value.clone(),
                        target_source: Some("user".to_string()), eligibility_json: None, require_new_origin: false,
                    };
                    if let Ok(saved) = persist_proposal_tx(&mut conn, &params, Some(&current.business_flow_data)) {
                        entry.mutate_working_state(|ws| ws.pending_patches.push(PendingPatchDto {
                            id: saved.patch_id, draft_session_id, target_code: proposal.target_code.clone(),
                            field_path: proposal.field_name.clone().unwrap_or_default(), operation: proposal.operation.clone(),
                            previous_value: proposal.previous_value.clone(),
                            proposed_value: proposal.new_value.clone().unwrap_or(Value::Null),
                            status: "pending".to_string(), reason: None, created_at: Utc::now().to_rfc3339(),
                        }));
                    }
                }
            }
        }
        (outcome, started.elapsed().as_millis() as u64, None)
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

    /// Fetch the most recent `user_limit` author messages and `assistant_limit` assistant
    /// messages (excluding `exclude_id`) and return them in chronological order.
    ///
    /// A flat `LIMIT n` over all roles loses the beginning of long sessions: each assistant
    /// turn can be stored as one row while tool-heavy turns inflate row counts. Per-role windows
    /// keep early author instructions and earlier assistant answers in context
    /// (`docs/business/prompts/overview.md` §context composition).
    pub fn fetch_recent_role_window(
        &self,
        session_id: &str,
        exclude_id: Uuid,
        user_limit: usize,
        assistant_limit: usize,
    ) -> Result<Vec<ChatMessageDto>, IpcError> {
        let conn = self
            .pool
            .get()
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

        // (created_at, rowid, message) so equal timestamps keep insertion order.
        let mut picked: Vec<(String, i64, ChatMessageDto)> = Vec::new();
        for (role, limit) in [("user", user_limit), ("assistant", assistant_limit)] {
            if limit == 0 {
                continue;
            }
            let mut stmt = conn
                .prepare(
                    "SELECT rowid, id, role, content, payload_json, created_at
                     FROM messages
                     WHERE session_id = ?1 AND id != ?2 AND role = ?3
                     ORDER BY created_at DESC, rowid DESC
                     LIMIT ?4",
                )
                .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
            let mut rows = stmt
                .query(params![session_id, exclude_id.to_string(), role, limit as i64])
                .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
            while let Some(row) = rows
                .next()
                .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?
            {
                let order: i64 = row.get(0).unwrap_or_default();
                let id: String = row.get(1).unwrap_or_default();
                let role: String = row.get(2).unwrap_or_default();
                let content: String = row.get(3).unwrap_or_default();
                let payload_str: Option<String> = row.get(4).unwrap_or(None);
                let created_at: String = row.get(5).unwrap_or_default();
                let tool_calls = payload_str
                    .as_deref()
                    .and_then(|s| serde_json::from_str::<Value>(s).ok())
                    .and_then(|v| v.get("tool_calls").cloned())
                    .and_then(|v| serde_json::from_value::<Vec<ToolCallItemDto>>(v).ok());
                picked.push((
                    created_at.clone(),
                    order,
                    ChatMessageDto {
                        id,
                        role,
                        content,
                        thinking_seconds: None,
                        thinking_details: None,
                        is_thinking: None,
                        tool_calls,
                        timestamp: created_at,
                    },
                ));
            }
        }

        picked.sort_by(|left, right| {
            left.0.cmp(&right.0).then(left.1.cmp(&right.1))
        });
        Ok(picked.into_iter().map(|(_, _, message)| message).collect())
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

/// Decide which historical assistant messages still replay their tool payload, newest first,
/// until the character budget is spent. The text content of every message is always kept.
fn replay_tool_flags(history: &[ChatMessageDto], budget: usize) -> Vec<bool> {
    let mut flags = vec![false; history.len()];
    let mut spent = 0usize;
    for (index, message) in history.iter().enumerate().rev() {
        if message.role != "assistant" {
            continue;
        }
        let Some(calls) = message.tool_calls.as_ref().filter(|calls| !calls.is_empty()) else {
            continue;
        };
        let size: usize = calls
            .iter()
            .map(|call| {
                serde_json::to_string(call).map(|raw| raw.len()).unwrap_or(0)
            })
            .sum();
        if spent.saturating_add(size) <= budget {
            spent += size;
            flags[index] = true;
        }
    }
    flags
}

/// Bookkeeping for the fixed per-turn read observations. The message list keeps exactly one
/// assistant tool-call message plus one tool-result message per read probe; only their contents
/// (and the array probe target) are refreshed while the loop runs, so context stays constant.
#[derive(Clone, Copy, PartialEq, Eq)]
enum ProbeKind {
    Scalar,
    Genres,
    Characters,
}

struct InjectedReads {
    /// (kind, index of the tool-result message, tool_call id)
    slots: Vec<(ProbeKind, usize, String)>,
    assistant_index: usize,
}

fn scalar_observation(data: &BusinessFlowData, system_language: &str) -> Value {
    read_tool_result(
        "get_field_null",
        &serde_json::json!({ "limit": 3 }),
        data,
        system_language,
    )
}

/// Build the observation for one probe. `get_field_null` is parameterless; the array probe needs
/// an `array_path`, so one probe covers genres and another covers the first incomplete character.
fn probe(data: &BusinessFlowData, system_language: &str, kind: ProbeKind, limit: usize) -> (Value, Value) {
    match kind {
        ProbeKind::Scalar => (
            serde_json::json!({ "limit": 3 }),
            scalar_observation(data, system_language),
        ),
        ProbeKind::Genres => {
            let args = serde_json::json!({
                "array_path": "require.project.genres",
                "limit": limit.max(1),
            });
            let observation = read_tool_result("get_array_element_fields", &args, data, system_language);
            (args, observation)
        }
        ProbeKind::Characters => {
            let code = data
                .require
                .character
                .iter()
                .find(|character| {
                    character.biological_sex.value.is_none() || character.motif_code.value.is_none()
                })
                .map(|character| character.code.clone());
            let args = serde_json::json!({
                "array_path": "require.character",
                "code": code,
                "limit": limit.max(1),
            });
            let observation = read_tool_result("get_array_element_fields", &args, data, system_language);
            (args, observation)
        }
    }
}

fn probe_name(kind: ProbeKind) -> &'static str {
    match kind {
        ProbeKind::Scalar => "get_field_null",
        ProbeKind::Genres | ProbeKind::Characters => "get_array_element_fields",
    }
}

/// Append the fixed read probes once per turn and remember their rows so later refreshes rewrite
/// the same messages instead of pushing new call/result pairs.
fn push_injected_reads(
    messages: &mut Vec<ProviderMessage>,
    all_tool_calls: &mut Vec<ToolCallItemDto>,
    data: &BusinessFlowData,
    system_language: &str,
    with_array_probe: bool,
    turn_id: Uuid,
    stream_tx: Option<&tokio::sync::mpsc::Sender<ChatStreamEvent>>,
) -> Option<InjectedReads> {
    let mut kinds = vec![ProbeKind::Scalar];
    if with_array_probe {
        kinds.push(ProbeKind::Genres);
        kinds.push(ProbeKind::Characters);
    }

    let mut calls = Vec::new();
    let mut observations = Vec::new();
    for kind in &kinds {
        let (args, observation) = probe(data, system_language, *kind, 3);
        calls.push(ProviderToolCall {
            id: Uuid::new_v4().to_string(),
            name: probe_name(*kind).to_string(),
            arguments: args.clone(),
        });
        observations.push((args, observation));
    }

    // No gap, no probe: unconditional injection showed "3 công cụ" on every turn (even a plain
    // approval). A bare `get_array_element_fields` probe without a target element always returns
    // "fields to create a new element", so the gap test uses the State, not the observation shape.
    let scalar_questions = observations
        .first()
        .and_then(|(_, observation)| observation["questions"].as_array())
        .map(|questions| questions.len())
        .unwrap_or(0);
    let character_gap = data.require.character.iter().any(|character| {
        character.biological_sex.value.is_none()
            || character.motif_code.value.is_none()
            || (character.display_name.value.is_none() && character.nickname.value.is_none())
    });
    let has_gap = scalar_questions > 0
        || data.require.project.genres.is_empty()
        || data.require.character.is_empty()
        || character_gap;
    if !has_gap {
        return None;
    }

    let assistant_index = messages.len();
    messages.push(ProviderMessage {
        role: "assistant".to_string(),
        content: String::new(),
        name: None,
        tool_calls: Some(calls.clone()),
        tool_call_id: None,
    });

    let mut slots = Vec::new();
    for (call, (args, observation)) in calls.into_iter().zip(observations.into_iter()) {
        let result_index = messages.len();
        messages.push(ProviderMessage::tool_result(
            call.id.clone(),
            call.name.clone(),
            observation.to_string(),
        ));
        all_tool_calls.push(ToolCallItemDto {
            id: call.id.clone(),
            name: call.name.clone(),
            parameters: args,
            result: Some(observation),
            execution_time_ms: Some(0),
            status: "success".to_string(),
        });
        if let Some(tx) = stream_tx {
            let _ = tx.try_send(ChatStreamEvent::ToolCallStarted {
                turn_id,
                tool_call_id: call.id.clone(),
                tool_code: call.name.clone(),
            });
            let _ = tx.try_send(ChatStreamEvent::ToolCallCompleted {
                turn_id,
                tool_call_id: call.id.clone(),
                tool_code: call.name.clone(),
                status: "applied".to_string(),
                has_applied_write: false,
            });
        }
        let kind = kinds[slots.len()];
        slots.push((kind, result_index, call.id));
    }

    Some(InjectedReads {
        slots,
        assistant_index,
    })
}

impl InjectedReads {
    /// Recompute the observations against the latest working state and rewrite the existing
    /// messages in place. The number of tool messages never changes after the first request.
    fn refresh(
        &self,
        messages: &mut [ProviderMessage],
        all_tool_calls: &mut [ToolCallItemDto],
        data: &BusinessFlowData,
        system_language: &str,
    ) {
        let scalar_count = self
            .slots
            .iter()
            .find(|(kind, _, _)| *kind == ProbeKind::Scalar)
            .and_then(|(_, index, _)| messages.get(*index))
            .and_then(|message| serde_json::from_str::<Value>(&message.content).ok())
            .and_then(|value| value["questions"].as_array().map(|questions| questions.len()))
            .unwrap_or(0);
        let mut remaining = 3usize.saturating_sub(scalar_count).max(1);

        for (kind, result_index, call_id) in &self.slots {
            let (args, mut observation) = probe(data, system_language, *kind, remaining);
            let question_count = observation["questions"]
                .as_array()
                .map(|questions| questions.len())
                .unwrap_or(0);
            if *kind == ProbeKind::Characters && remaining == 0 {
                // Keep the three-question budget: an exhausted budget leaves the probe empty
                // rather than surfacing a fourth question to the model.
                if let Some(object) = observation.as_object_mut() {
                    object.insert("questions".to_string(), Value::Array(Vec::new()));
                }
            } else if *kind != ProbeKind::Scalar {
                remaining = remaining.saturating_sub(question_count);
            }

            if let Some(message) = messages.get_mut(*result_index) {
                message.content = observation.to_string();
            }
            if let Some(message) = messages.get_mut(self.assistant_index) {
                if let Some(calls) = message.tool_calls.as_mut() {
                    if let Some(call) = calls.iter_mut().find(|call| call.id == *call_id) {
                        call.arguments = args.clone();
                    }
                }
            }
            if let Some(entry) = all_tool_calls.iter_mut().find(|call| call.id == *call_id) {
                entry.parameters = args;
                entry.result = Some(observation);
            }
        }
    }
}

fn append_chat_history(
    messages: &mut Vec<ProviderMessage>,
    history: ChatMessageDto,
    replay_tools: bool,
) {
    if history.role == "user" {
        messages.push(ProviderMessage::user(history.content));
    } else if history.role == "assistant" {
        if let Some(calls) = history.tool_calls.filter(|calls| !calls.is_empty() && replay_tools) {
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

#[allow(dead_code)]
fn render_chat_prompt(template: &str, data: &BusinessFlowData, patches: &[PendingPatchDto]) -> String {
    render_chat_prompt_with_phase(template, data, patches, data.meta.phase)
}

fn render_chat_prompt_with_phase(
    template: &str,
    data: &BusinessFlowData,
    patches: &[PendingPatchDto],
    phase: i32,
) -> String {
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
        "mode": "planning", "phase": phase, "subphase": data.meta.subphase,
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
        "get_field_null" => {
            // Scalar completeness alone can look "done" while a required array is still empty
            // (project.genres was empty after a rejected write and the readiness gate stayed
            // closed). Surface empty required arrays next to the scalar questions.
            let mut empty_arrays: Vec<&str> = Vec::new();
            if data.require.project.genres.is_empty() {
                empty_arrays.push("require.project.genres");
            }
            if data.require.character.is_empty() {
                empty_arrays.push("require.character");
            }
            let array_questions: Vec<Value> = empty_arrays
                .iter()
                .filter_map(|path| {
                    get_array_element_fields(data, path, None, Some(3), system_language)
                        .ok()
                        .map(|questions| serde_json::json!({"array_path": path, "questions": questions}))
                })
                .collect();
            serde_json::json!({
                "status": "applied",
                "questions": get_field_null(data, limit, system_language),
                "empty_arrays": empty_arrays,
                "array_questions": array_questions,
            })
        }
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
        assert_eq!(result["empty_arrays"][0], "require.project.genres");
        assert_eq!(result["empty_arrays"][1], "require.character");
        assert_eq!(result["array_questions"][0]["array_path"], "require.project.genres");
        assert!(!result["array_questions"][0]["questions"].as_array().unwrap().is_empty());
    }
}

#[cfg(test)]
mod chat_context_regression_tests {
    use super::*;
    use crate::domain::business_flow::TrackedField;
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
    async fn phase_zero_complete_state_does_not_inject_read_probes() {
        use crate::domain::business_flow::RequireGenreItem;
        use crate::providers::traits::ProviderChatResponse;
        let tmp = tempfile::tempdir().unwrap();
        let pool = Pool::builder().max_size(2).build(SqliteConnectionManager::file(tmp.path().join("chat.db"))).unwrap();
        crate::storage::migration::run_migrations(&mut pool.get().unwrap()).unwrap();
        let registry = Arc::new(DraftRuntimeRegistry::new());
        let draft_service = Arc::new(DraftService::new(pool.clone(), registry.clone()));
        let patch_service = Arc::new(PatchService::new(pool.clone(), registry.clone(), draft_service.clone()));
        let orchestrator = ChatOrchestrator::new(pool, registry.clone(), draft_service, patch_service);
        let id = Uuid::new_v4();
        registry.get_or_register(id, crate::service::draft_runtime::WorkingState::new(id));
        let entry = registry.get(&id).unwrap();
        entry.mutate_working_state(|ws| {
            let project = &mut ws.business_flow_data.require.project;
            project.display_name = TrackedField::user("Truyện".into());
            project.summary = TrackedField::user("Tóm tắt".into());
            project.language = TrackedField::user("VI".into());
            project.target.arc_count = TrackedField::user(5);
            project.target.chapter_count = TrackedField::user(150);
            project.target.word_per_chapter_count = TrackedField::user(2000);
            project.genres.push(RequireGenreItem {
                code: TrackedField::user("romance".into()),
                is_primary: TrackedField::user(true),
                display_name: TrackedField::user("Lãng mạn".into()),
            });
            let mut character = crate::domain::business_flow::RequireCharacterItem::default();
            character.code = "hero".into();
            character.display_name = TrackedField::user("Anh hùng".into());
            character.biological_sex = TrackedField::user("MALE".into());
            character.motif_code = TrackedField::user("positive_change_arc".into());
            ws.business_flow_data.require.character.push(character);
        });
        let composed = orchestrator.composer.compose(&["tool_library"], "vi", "VI").unwrap();
        let provider = ReplayProvider {
            requests: Default::default(),
            responses: std::sync::Mutex::new([
                ProviderChatResponse { content: Some("Okey, mình ghi nhận.".into()), ..Default::default() },
            ].into()),
        };
        let mut messages = vec![ProviderMessage::system(composed.prompt.clone()), ProviderMessage::user("okey đấy")];
        let mut state = TurnState::default();
        orchestrator.run_turn_loop(&entry, id, Uuid::new_v4(), &composed, &mut messages,
            &mut state, "vi", &provider, None, CancellationToken::new()).await.unwrap();

        // No missing Phase 0 field: internal read probes must stay out of the turn entirely, so the
        // UI does not show "3 công cụ" for a plain approval message.
        assert!(state.all_tool_calls.is_empty(), "no probe expected when nothing is missing");
        let requests = provider.requests.lock().unwrap();
        assert!(requests[0].messages.iter().all(|message| message.role != "tool"));
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
        let calls = &state.all_tool_calls;
        assert_eq!(calls.iter().find(|call| call.id == "call-1").unwrap().result.as_ref().unwrap()["results"][0]["error"], "missing_or_invalid_declared_source");
        assert_eq!(calls.iter().find(|call| call.id == "call-2").unwrap().result.as_ref().unwrap()["input_schema"]["properties"]["target"]["type"], "object");
        assert_eq!(entry.get_snapshot().business_flow_data.require.project.target.chapter_count.value, Some(120));
        assert!(state.has_applied_write);
        let questions = &calls.iter().find(|call| call.id == "call-4").unwrap().result.as_ref().unwrap()["questions"];
        assert!(!questions.to_string().contains("target."));
        // One fixed read per turn, refreshed in place after the write instead of appended again.
        let automatic_reads: Vec<_> = calls.iter().filter(|call| call.name == "get_field_null" && call.id != "call-4").collect();
        assert_eq!(automatic_reads.len(), 1, "the fixed read is injected once and refreshed in place");
        assert!(!automatic_reads[0].result.as_ref().unwrap()["questions"].to_string().contains("target."));
        let automatic_array_reads: Vec<_> = calls
            .iter()
            .filter(|call| call.name == "get_array_element_fields")
            .collect();
        assert_eq!(automatic_array_reads.len(), 2, "genres and characters probes stay fixed");
        let requests = provider.requests.lock().unwrap();
        // The fixed probe set never grows: every request keeps exactly the same three probe
        // messages (one scalar + two arrays) regardless of how many writes the loop performed.
        let injected_ids: std::collections::HashSet<&str> = automatic_reads
            .iter()
            .map(|call| call.id.as_str())
            .chain(automatic_array_reads.iter().map(|call| call.id.as_str()))
            .collect();
        assert!(requests.iter().all(|request| {
            request
                .messages
                .iter()
                .filter(|message| {
                    message.role == "tool"
                        && message
                            .tool_call_id
                            .as_deref()
                            .map(|id| injected_ids.contains(id))
                            .unwrap_or(false)
                })
                .count()
                == 3
        }), "read probes must not be appended per state change");
        assert!(requests[3].messages[0].content.contains("2400"));
        assert!(!requests[3].messages[0].content.contains("{{story_context}}"));
        assert!(requests.iter().flat_map(|r| &r.messages).filter(|m| m.role == "tool").all(|m| !m.content.contains("authenticated_effect")));
        assert!(orchestrator.composer.compose(&["out_of_scope"], "vi", "VI").unwrap().tools.is_empty());
    }

    #[tokio::test]
    async fn test_every_chat_request_reads_null_fields_even_without_model_tool_calls() {
        use crate::providers::traits::ProviderChatResponse;
        let tmp = tempfile::tempdir().unwrap();
        let pool = Pool::builder().max_size(2).build(SqliteConnectionManager::file(tmp.path().join("chat.db"))).unwrap();
        crate::storage::migration::run_migrations(&mut pool.get().unwrap()).unwrap();
        let registry = Arc::new(DraftRuntimeRegistry::new());
        let draft_service = Arc::new(DraftService::new(pool.clone(), registry.clone()));
        let patch_service = Arc::new(PatchService::new(pool.clone(), registry.clone(), draft_service.clone()));
        let orchestrator = ChatOrchestrator::new(pool, registry.clone(), draft_service, patch_service);
        let id = Uuid::new_v4();
        registry.get_or_register(id, crate::service::draft_runtime::WorkingState::new(id));
        let entry = registry.get(&id).unwrap();
        let composed = orchestrator.composer.compose(&["tool_library", "story_context"], "vi", "VI").unwrap();
        assert!(composed.tools.iter().any(|tool| tool.name == "get_field_null"));
        for name in ["get_array_element_fields", "search_genre", "search_motif", "add_project_genre"] {
            assert!(composed.tools.iter().any(|tool| tool.name == name), "missing {name}");
        }
        assert_eq!(composed.tools.iter().find(|tool| tool.name == "set_project").unwrap()
            .parameters["properties"]["target"]["properties"]["chapter_tolerance"]["minimum"], 0);
        let provider = ReplayProvider { requests: Default::default(), responses: std::sync::Mutex::new([
            ProviderChatResponse { content: Some("Xin chào".into()), ..Default::default() },
            ProviderChatResponse { content: Some("Đã ghi nhận".into()), ..Default::default() },
        ].into()) };
        for (index, text) in ["Chào", "Tên arc không cần đặt"].iter().enumerate() {
            let mut messages = vec![ProviderMessage::system(composed.prompt.clone()), ProviderMessage::user(*text)];
            let mut state = TurnState::default();
            orchestrator.run_turn_loop(&entry, id, Uuid::new_v4(), &composed, &mut messages,
                &mut state, "vi", &provider, None, CancellationToken::new()).await.unwrap();
            // Every turn audits both the scalar-null read and the array-element read.
            assert!(state.all_tool_calls.len() >= 2, "turn {index} must audit both reads");
            assert_eq!(state.all_tool_calls[0].name, "get_field_null");
            assert_eq!(state.all_tool_calls[0].result.as_ref().unwrap()["status"], "applied");
            assert!(
                state.all_tool_calls.iter().any(|call| call.name == "get_array_element_fields"),
                "turn {index} must also probe array element fields"
            );
            let requests = provider.requests.lock().unwrap();
            let request = &requests[index];
            assert_eq!(request.messages[2].tool_calls.as_ref().unwrap()[0].id,
                state.all_tool_calls[0].id);
            assert_eq!(request.messages[3].tool_call_id.as_deref(), Some(state.all_tool_calls[0].id.as_str()));
            assert_eq!(request.messages[3].role, "tool");
        }
    }

    #[tokio::test]
    async fn phase_zero_array_gaps_are_visible_without_model_calls() {
        use crate::domain::business_flow::RequireCharacterItem;
        use crate::providers::traits::ProviderChatResponse;
        let tmp = tempfile::tempdir().unwrap();
        let pool = Pool::builder().max_size(2).build(SqliteConnectionManager::file(tmp.path().join("chat.db"))).unwrap();
        crate::storage::migration::run_migrations(&mut pool.get().unwrap()).unwrap();
        let registry = Arc::new(DraftRuntimeRegistry::new());
        let draft_service = Arc::new(DraftService::new(pool.clone(), registry.clone()));
        let patch_service = Arc::new(PatchService::new(pool.clone(), registry.clone(), draft_service.clone()));
        let orchestrator = ChatOrchestrator::new(pool, registry.clone(), draft_service, patch_service);
        let id = Uuid::new_v4();
        registry.get_or_register(id, crate::service::draft_runtime::WorkingState::new(id));
        let entry = registry.get(&id).unwrap();
        entry.mutate_working_state(|ws| {
            let project = &mut ws.business_flow_data.require.project;
            project.display_name = TrackedField::user("Truyện".into());
            project.summary = TrackedField::user("Tóm tắt".into());
            project.language = TrackedField::user("VI".into());
            project.target.arc_count = TrackedField::user(1);
            project.target.chapter_count = TrackedField::user(100);
            project.target.word_per_chapter_count = TrackedField::user(2000);
            let mut character = RequireCharacterItem::default();
            character.code = "hero".into();
            character.display_name = TrackedField::user("Anh hùng".into());
            character.biological_sex = TrackedField::user("MALE".into());
            ws.business_flow_data.require.character.push(character);
        });
        let composed = orchestrator.composer.compose(&["tool_library"], "vi", "VI").unwrap();
        let provider = ReplayProvider { requests: Default::default(), responses: std::sync::Mutex::new([
            ProviderChatResponse { content: Some("Đã hiểu".into()), ..Default::default() },
        ].into()) };
        let mut messages = vec![ProviderMessage::system(composed.prompt.clone()), ProviderMessage::user("Xin chào")];
        let mut state = TurnState::default();
        orchestrator.run_turn_loop(&entry, id, Uuid::new_v4(), &composed, &mut messages,
            &mut state, "vi", &provider, None, CancellationToken::new()).await.unwrap();
        assert_eq!(state.all_tool_calls.iter().map(|call| call.name.as_str()).collect::<Vec<_>>(),
            vec!["get_field_null", "get_array_element_fields", "get_array_element_fields"]);
        assert_eq!(state.all_tool_calls[1].parameters["array_path"], "require.project.genres");
        assert_eq!(state.all_tool_calls[2].result.as_ref().unwrap()["questions"][0]["field"], "motif_code");
        let requests = provider.requests.lock().unwrap();
        assert_eq!(requests[0].messages.iter().filter(|m| m.role == "tool").count(), 3);
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
        append_chat_history(&mut history, message, true);
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
