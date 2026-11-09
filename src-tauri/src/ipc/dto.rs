//! Authoritative Typed IPC Data Transfer Objects (DTOs) for Merlink Harness (`REQ-IPC-001`, `REQ-IPC-002`).
//!
//! Encompasses:
//! - Exactly 40 command Request/Response pairs across 10 functional groups.
//! - Exactly 3 Tauri v2 streaming channel payload schemas (`ChatStreamEvent`, `WorkflowProgressEvent`, `SceneGenerationStreamEvent`).
//! - Complete camelCase serialization/deserialization with `#[serde(rename_all = "camelCase")]`.
//! - Strict secret protection (`P2S5-N`): API keys are never serialized in output DTOs, and input request fields are skipped on serialization.
//! - Inclusion of full `content` in `SceneVersionItemDto` (`CI-08`) for client-side chapter dirty buffer restore.

use crate::error::IpcError;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use uuid::Uuid;

// ============================================================================
// Supporting / Domain-Bridge DTOs
// ============================================================================

/// Feedback context attached to chat turns or scene generations.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct FeedbackContextDto {
    pub code: String,
    pub tag: String,
    pub title: String,
    pub content: String,
}

/// Tool call audit item inside a chat message.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct ToolCallItemDto {
    pub id: String,
    pub name: String,
    pub parameters: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_time_ms: Option<u64>,
    pub status: String, // "success" | "error" | "rejected"
}

/// Chat message item for history rendering.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct ChatMessageDto {
    pub id: String,
    pub role: String, // "user" | "assistant"
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thinking_seconds: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thinking_details: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_thinking: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCallItemDto>>,
    pub timestamp: String,
}

/// Compact draft version summary for dropdown and history lists.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct DraftVersionSummaryDto {
    pub seq: i32,
    pub display_label: String, // e.g. "v.01"
    pub phase: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subphase: Option<i32>,
    pub reason: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_seq: Option<i32>,
    pub created_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_hash: Option<String>,
}

/// Progress summary across phases and Master Attributes.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
pub struct ProgressSummaryDto {
    pub current_phase: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_subphase: Option<i32>,
    pub completed_attributes: u32,
    pub total_attributes: u32,
    pub progress_percentage: f32,
    pub is_require_complete: bool,
    pub can_advance_phase: bool,
}

/// Pending patch item requiring user confirmation or rejection.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct PendingPatchDto {
    pub id: Uuid,
    pub draft_session_id: Uuid,
    pub target_code: String,
    pub field_path: String,
    pub operation: String, // "update" | "insert" | "delete"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_value: Option<serde_json::Value>,
    pub proposed_value: serde_json::Value,
    pub status: String, // "pending" | "confirmed" | "rejected" | "expired"
    pub reason: Option<String>,
    pub created_at: String,
}

/// Event log entry for the draft workspace audit log.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct EventLogEntryDto {
    pub id: String,
    pub event_type: String,
    pub description: String,
    pub timestamp: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

/// Lock status of a draft session.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct DraftLockStatusDto {
    pub is_locked: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub holder_info: Option<String>,
}

/// Attribute evaluation detail for ProcessDetailDialog (fe-core.md GAP-15).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct AttributeEvaluationDetailDto {
    pub code: String,
    pub display_name: String,
    pub phase: i32,
    pub is_required: bool,
    pub status: String, // "complete" | "incomplete" | "conflict"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_value_summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

/// Constraint conflict item.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct ConstraintConflictDto {
    pub constraint_id: String,
    pub description: String,
    pub conflicting_codes: Vec<String>,
}

/// Skipped patch item when confirming all pending patches.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct SkippedPatchDto {
    pub patch_id: Uuid,
    pub reason: String,
}

/// Plan summary for commit confirmation preview dialog.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct PlanSummaryDto {
    pub project_name: String,
    pub target_arcs: u32,
    pub target_chapters: u32,
    pub target_words_per_chapter: u32,
    pub character_count: u32,
    pub arc_count: u32,
    pub chapter_count: u32,
    pub world_setting_count: u32,
    pub lore_entry_count: u32,
}

/// Validation issue item.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct ValidationIssueDto {
    pub field: String,
    pub message: String,
    pub severity: String, // "error" | "warning"
}

/// Commit validation result for preview.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct CommitValidationResultDto {
    pub status: String, // "ready" | "blocked"
    pub issues: Vec<ValidationIssueDto>,
}

/// Pending patches summary for commit preview.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct PendingPatchesSummaryDto {
    pub count: usize,
    pub items: Vec<PendingPatchDto>,
}

/// UI projection of project item matching `src/features/projects/types.ts`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct ProjectDetailsUiDto {
    pub word_count: String,
    pub chapters: String,
    pub characters: String,
    pub world_lore: String,
    pub created_date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct ChapterBlockUiDto {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene_id: Option<String>,
    pub text: String,
    pub locked: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct ChapterItemUiDto {
    pub id: String,
    pub title: String,
    pub date: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_new: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocks: Option<Vec<ChapterBlockUiDto>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct ArcItemUiDto {
    pub id: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover: Option<String>,
    pub chapters: Vec<ChapterItemUiDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct ProjectItemUiDto {
    pub id: String,
    pub title: String,
    pub status: String, // "Drafting" | "Completed" | "In Review" | "Ongoing" | "Planning"
    pub cover: String,
    pub description: String,
    pub genres: Vec<String>,
    pub modified_at: String,
    pub details: ProjectDetailsUiDto,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arcs: Option<Vec<ArcItemUiDto>>,
}

/// Character Library item matching `src/features/characters/types.ts`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct CharacterLibraryItemDto {
    pub id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<Uuid>,
    pub code: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    pub species: String,
    pub biological_sex: String, // "NONE" | "MALE" | "FEMALE"
    pub gender_identity: String,
    pub gender_expression: String,
    pub in_project: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_summary: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
pub struct CharacterAppearanceDto {
    pub height: f32,
    pub weight: f32,
    pub hair: String,
    pub eyes: String,
    pub skin: String,
    pub hair_note: String,
    pub eyes_note: String,
    pub skin_note: String,
    pub outfit_style: String,
    pub default_summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct CharacterPsychologicalDto {
    pub core_values: Vec<String>,
    pub trauma_triggers: Vec<String>,
    pub behavioral_quirks: Vec<String>,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct CharacterMemoryDto {
    pub id: Uuid,
    pub memory_type: String,
    pub title: String,
    pub description: String,
    pub emotional_charge: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct CharacterCompetenceDto {
    pub id: Uuid,
    pub domain: String,
    pub competence_name: String,
    pub level: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct CharacterRelationshipDto {
    pub target_character_id: Uuid,
    pub relationship_type: String,
    pub intensity: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
pub struct CharacterUpsertPayloadDto {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    pub species: String,
    pub biological_sex: String,
    pub gender_identity: String,
    pub gender_expression: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appearance: Option<CharacterAppearanceDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub psychological: Option<CharacterPsychologicalDto>,
    #[serde(default)]
    pub memories: Vec<CharacterMemoryDto>,
    #[serde(default)]
    pub competences: Vec<CharacterCompetenceDto>,
}

/// Scene read-model DTO for creator and writing views.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct SceneReadModelDto {
    pub id: Uuid,
    pub chapter_id: Uuid,
    pub position: i32,
    pub display_name: String,
    pub summary: Option<String>,
    pub content: Option<String>,
    pub word_count: u32,
    pub is_locked: bool,
    pub head_version_number: i32,
    pub current_scene_source: String, // "AI" | "USER" | "RESTORE"
}

/// Scene version item returned by `list_scene_versions` (#35).
///
/// MUST contain full `content` (`CI-08`) so frontend can restore historical scene text
/// into the chapter dirty buffer without requiring a dedicated 41st IPC restore command.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct SceneVersionItemDto {
    pub version_number: i32,
    pub content: String,
    pub word_count: u32,
    pub scene_source: String, // "AI" | "USER" | "RESTORE"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    pub created_at: String,
}

/// Dirty scene item in `save_chapter` request.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct DirtySceneSaveItem {
    pub scene_id: Uuid,
    pub expected_version_number: i32,
    pub content: String,
    pub scene_source: String, // "AI" | "USER" | "RESTORE"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
}

/// Result of a saved scene within `save_chapter`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct SavedSceneResultDto {
    pub scene_id: Uuid,
    pub new_version_number: i32,
    pub word_count: u32,
}

/// Generation stream status enum for Writing Mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub enum GenerationStreamStatus {
    Streaming,
    Completed,
    Cancelled,
    InvalidatedByLock,
    Failed,
}

/// Subagent configuration matching `src/features/settings/subagentConfig.ts`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct SubagentConfigDto {
    pub max_subagents: u32,
    pub rejected_threshold: u32,
    pub outline_capacity: u32,
    pub outline_lookback: u32,
    pub context_margin: u32,
    pub chapters_per_subagent: u32,
    pub lore_records_per_subagent: u32,
    pub characters_per_subagent: u32,
    pub relationship_pairs_per_subagent: u32,
    pub model: String,
}

impl Default for SubagentConfigDto {
    fn default() -> Self {
        Self {
            max_subagents: 6,
            rejected_threshold: 3,
            outline_capacity: 50,
            outline_lookback: 10,
            context_margin: 5,
            chapters_per_subagent: 30,
            lore_records_per_subagent: 10,
            characters_per_subagent: 5,
            relationship_pairs_per_subagent: 10,
            model: "chat".to_string(),
        }
    }
}

/// Provider status report DTO.
///
/// NOTE: STRICTLY NEVER contains `apiKey` (`P2S5-N`).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct ProviderStatusDto {
    pub provider_id: String,
    pub name: String,
    pub is_active: bool,
    pub has_api_key: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_name: Option<String>,
}

/// Summary item in session listing.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct SessionSummaryDto {
    pub session_id: String,
    pub draft_session_id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<Uuid>,
    pub status: String,
    pub updated_at: String,
    /// Session title derived from the first user message (max 10 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub message_count: Option<u32>,
}

// ============================================================================
// Group 1: Chat & Session (Commands 1..4)
// ============================================================================

/// Command 1: `create_or_load_chat_session` Request
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct CreateOrLoadChatSessionReq {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<Uuid>,
}

/// Command 1: `create_or_load_chat_session` Response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
pub struct ChatSessionInitDto {
    pub session_id: String,
    pub draft_session_id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<Uuid>,
    pub status: String, // "editing" | "committing" | "committed"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head_version_seq: Option<i32>,
    pub messages: Vec<ChatMessageDto>,
    pub working_state: serde_json::Value,
    pub progress: ProgressSummaryDto,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interrupted_turn_alert: Option<String>,
}

/// Command 2: `list_chat_sessions` Request
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct ListChatSessionsReq {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_messages: Option<bool>,
}

/// Command 2: `list_chat_sessions` Response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct ChatSessionListDto {
    pub items: Vec<SessionSummaryDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}

/// Command 3: `send_chat_message` Request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct SendChatMessageReq {
    pub session_id: String,
    pub content: String,
    pub system_language: String, // "vi" | "en"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedback_context: Option<FeedbackContextDto>,
}

/// Command 3: `send_chat_message` Response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
pub struct SendChatTurnResponseDto {
    pub turn_id: Uuid,
    pub user_message_id: Uuid,
    pub assistant_message_id: Uuid,
    pub assistant_text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sealed_version: Option<DraftVersionSummaryDto>,
    pub progress: ProgressSummaryDto,
    pub pending_patches: Vec<PendingPatchDto>,
}

/// Command 4: `cancel_chat_turn` Request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct CancelChatTurnReq {
    pub session_id: String,
    pub turn_id: Uuid,
}

/// Command 4: `cancel_chat_turn` Response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct CancelChatTurnResponseDto {
    pub cancelled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sealed_version: Option<DraftVersionSummaryDto>,
}

// ============================================================================
// Group 2: Draft, Working State & Progress Engine (Commands 5..9)
// ============================================================================

/// Command 5: `get_draft_workspace_state` Request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct GetDraftWorkspaceStateReq {
    pub draft_session_id: Uuid,
}

/// Command 5: `get_draft_workspace_state` Response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
pub struct DraftWorkspaceStateDto {
    pub draft_session_id: Uuid,
    pub status: String,
    pub phase: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subphase: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head_version_seq: Option<i32>,
    pub business_flow_data: serde_json::Value,
    pub draft_overview_ui: serde_json::Value,
    pub progress: ProgressSummaryDto,
    pub pending_patches: Vec<PendingPatchDto>,
    pub event_log: Vec<EventLogEntryDto>,
    pub lock_state: DraftLockStatusDto,
}

/// Command 6: `get_progress_detail` Request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct GetProgressDetailReq {
    pub draft_session_id: Uuid,
    pub system_language: String,
}

/// Command 6: `get_progress_detail` Response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
pub struct ProgressDetailDto {
    pub progress: ProgressSummaryDto,
    pub attributes: Vec<AttributeEvaluationDetailDto>,
    pub rendered_markdown: String,
    pub user_constraints: Vec<String>,
    pub conflicts: Vec<ConstraintConflictDto>,
}

/// Command 7: `apply_manual_draft_edit` Request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct ApplyManualDraftEditReq {
    pub draft_session_id: Uuid,
    pub client_edit_id: String,
    pub tool_code: String,
    pub args: serde_json::Value,
}

/// Command 7: `apply_manual_draft_edit` Response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
pub struct ManualEditResponseDto {
    pub tool_output: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sealed_version: Option<DraftVersionSummaryDto>,
    pub progress: ProgressSummaryDto,
}

/// Command 8: `confirm_phase` Request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct ConfirmPhaseReq {
    pub draft_session_id: Uuid,
    pub target_phase: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_subphase: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_draft_suggestion: Option<String>,
}

/// Command 8: `confirm_phase` Response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct PhaseConfirmResponseDto {
    pub phase: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subphase: Option<i32>,
    pub sealed_version: DraftVersionSummaryDto,
}

/// Command 9: `regenerate_draft_phase` Request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct RegenerateDraftPhaseReq {
    pub draft_session_id: Uuid,
    pub phase: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subphase: Option<i32>,
    pub system_language: String,
}

/// Command 9: `regenerate_draft_phase` Response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
pub struct RegeneratePhaseResponseDto {
    pub turn_id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sealed_version: Option<DraftVersionSummaryDto>,
    pub pending_patches: Vec<PendingPatchDto>,
    pub progress: ProgressSummaryDto,
}

// ============================================================================
// Group 3: Versioning & Restore (Commands 10..11)
// ============================================================================

/// Command 10: `list_draft_versions` Request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct ListDraftVersionsReq {
    pub draft_session_id: Uuid,
}

/// Command 10: `list_draft_versions` Response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct DraftVersionListDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head_version_seq: Option<i32>,
    pub versions: Vec<DraftVersionSummaryDto>,
}

/// Command 11: `restore_draft_version` Request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct RestoreDraftVersionReq {
    pub draft_session_id: Uuid,
    pub target_seq: i32,
    pub client_request_id: String,
}

/// Command 11: `restore_draft_version` Response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
pub struct RestoreVersionResponseDto {
    pub new_version: DraftVersionSummaryDto,
    pub restored_phase: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restored_subphase: Option<i32>,
    pub expired_patch_ids: Vec<Uuid>,
    pub working_state: DraftWorkspaceStateDto,
}

// ============================================================================
// Group 4: Pending Patches (Commands 12..15)
// ============================================================================

/// Command 12: `list_pending_patches` Request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct ListPendingPatchesReq {
    pub draft_session_id: Uuid,
}

/// Command 12: `list_pending_patches` Response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct PendingPatchListDto {
    pub items: Vec<PendingPatchDto>,
}

/// Command 13: `confirm_pending_patch` Request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct ConfirmPendingPatchReq {
    pub draft_session_id: Uuid,
    pub patch_id: Uuid,
}

/// Command 13 & 14: `confirm_pending_patch` & `reject_pending_patch` Response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
pub struct PatchResolutionDto {
    pub status: String, // "confirmed" | "already_confirmed" | "rejected" | "already_rejected"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sealed_version: Option<DraftVersionSummaryDto>,
    pub progress: ProgressSummaryDto,
}

/// Command 14: `reject_pending_patch` Request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct RejectPendingPatchReq {
    pub draft_session_id: Uuid,
    pub patch_id: Uuid,
}

/// Command 15: `confirm_all_pending_patches` Request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct ConfirmAllPendingPatchesReq {
    pub draft_session_id: Uuid,
    pub client_request_id: String,
}

/// Command 15: `confirm_all_pending_patches` Response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
pub struct ConfirmAllPatchesResponseDto {
    pub applied: Vec<Uuid>,
    pub skipped: Vec<SkippedPatchDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sealed_version: Option<DraftVersionSummaryDto>,
    pub progress: ProgressSummaryDto,
}

// ============================================================================
// Group 5: Segmented Workflow (Commands 16..18)
// ============================================================================

/// Command 16: `start_or_resume_workflow` Request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct StartOrResumeWorkflowReq {
    pub draft_session_id: Uuid,
    pub phase: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subphase: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_run_id: Option<Uuid>,
    pub system_language: String,
}

/// Command 16: `start_or_resume_workflow` Response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowExecutionSummaryDto {
    pub workflow_run_id: Uuid,
    pub plan_generation: i32,
    pub status: String, // "completed" | "cancelled" | "failed" | "requires_replan"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_reason: Option<String>,
    pub completed_tasks: u32,
    pub total_tasks: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_sealed_version: Option<DraftVersionSummaryDto>,
    pub pending_patches: Vec<PendingPatchDto>,
}

/// Command 17: `cancel_workflow` Request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct CancelWorkflowReq {
    pub draft_session_id: Uuid,
    pub workflow_run_id: Uuid,
}

/// Command 17: `cancel_workflow` Response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct CancelWorkflowResponseDto {
    pub status: String, // "cancelled"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_sealed_version: Option<DraftVersionSummaryDto>,
    pub resume_available: bool,
}

/// Command 18: `get_workflow_status` Request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct GetWorkflowStatusReq {
    pub draft_session_id: Uuid,
}

/// Command 18: `get_workflow_status` Response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowStatusDto {
    pub is_running: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_run_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_generation: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_phase: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_subphase: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_round: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_rounds: Option<u32>,
    pub completed_tasks: u32,
    pub total_tasks: u32,
    pub resume_available: bool,
}

// ============================================================================
// Group 6: Start / Commit Draft (Commands 19..21)
// ============================================================================

/// Command 19: `get_commit_confirmation_preview` Request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct GetCommitPreviewReq {
    pub draft_session_id: Uuid,
}

/// Type alias for Command 19 request
pub type GetCommitConfirmationPreviewReq = GetCommitPreviewReq;

/// Command 19: `get_commit_confirmation_preview` Response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct CommitPreviewDto {
    pub draft_session_id: Uuid,
    pub status: String,
    pub plan_summary: PlanSummaryDto,
    pub validation: CommitValidationResultDto,
    pub pending_patches: PendingPatchesSummaryDto,
    pub can_confirm: bool,
}

/// Command 20: `commit_draft` Request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct CommitDraftReq {
    pub draft_session_id: Uuid,
}

/// Command 20: `commit_draft` Response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct CommitDraftResponseDto {
    pub status: String, // "committed"
    pub project_id: Uuid,
    pub redirect_path: String,
}

/// Command 21: `reopen_committed_draft` Request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct ReopenCommittedDraftReq {
    pub project_id: Uuid,
    pub active_session_id: String,
}

/// Command 21: `reopen_committed_draft` Response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct ReopenCommittedDraftResponseDto {
    pub draft_session_id: Uuid,
    pub status: String, // "editing"
}

/// Type alias for Command 21 response
pub type ReopenDraftResponseDto = ReopenCommittedDraftResponseDto;

// ============================================================================
// Group 7: Projects & Creator Mode (Commands 22..24)
// ============================================================================

/// Command 22: `list_projects` Request
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct ListProjectsReq {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>, // "a-z" | "z-a" | "modified"
}

/// Command 22: `list_projects` Response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct ProjectListDto {
    pub items: Vec<ProjectItemUiDto>,
}

/// Command 23: `get_project_detail` Request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct GetProjectDetailReq {
    pub project_id: Uuid,
}

/// Command 23: `get_project_detail` Response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct ProjectDetailDto {
    pub project: ProjectItemUiDto,
    pub has_committed_plan: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draft_session_id: Option<Uuid>,
    pub can_reopen_draft: bool,
}

/// Command 24: `delete_project` Request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct DeleteProjectReq {
    pub project_id: Uuid,
}

/// Command 24: `delete_project` Response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct DeleteProjectResponseDto {
    pub deleted: bool,
}

// ============================================================================
// Group 8: Character Library (Commands 25..28)
// ============================================================================

/// Command 25: `list_characters` Request
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct ListCharactersReq {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub biological_sex: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
}

/// Command 25: `list_characters` Response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct CharacterListDto {
    pub items: Vec<CharacterLibraryItemDto>,
}

/// Command 26: `get_character_detail` Request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct GetCharacterDetailReq {
    pub character_id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<Uuid>,
}

/// Command 26: `get_character_detail` Response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
pub struct CharacterDetailDto {
    pub character: CharacterLibraryItemDto,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appearance: Option<CharacterAppearanceDto>,
    #[serde(default)]
    pub distinguishing_marks: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub psychological: Option<CharacterPsychologicalDto>,
    #[serde(default)]
    pub memories: Vec<CharacterMemoryDto>,
    #[serde(default)]
    pub competences: Vec<CharacterCompetenceDto>,
    #[serde(default)]
    pub relationships: Vec<CharacterRelationshipDto>,
}

/// Command 27: `create_or_update_library_character` Request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
pub struct UpsertLibraryCharacterReq {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub character_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<Uuid>,
    pub profile: CharacterUpsertPayloadDto,
}

/// Type alias for Command 27 request
pub type CreateOrUpdateLibraryCharacterReq = UpsertLibraryCharacterReq;

/// Command 28: `link_library_character_to_draft` Request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct LinkLibraryCharacterToDraftReq {
    pub draft_session_id: Uuid,
    pub character_id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

/// Command 28: `link_library_character_to_draft` Response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct LinkLibraryCharacterResponseDto {
    pub character_code: String,
    pub sealed_version: DraftVersionSummaryDto,
}

// ============================================================================
// Group 9: Writing Mode, Scene Lock, Generation & Save Chapter (Commands 29..37)
// ============================================================================

/// Command 29: `get_chapter_writing_state` Request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct GetChapterWritingStateReq {
    pub project_id: Uuid,
    pub chapter_id: Uuid,
}

/// Type alias for Command 29 request
pub type GetChapterWorkspaceReq = GetChapterWritingStateReq;

/// Command 29: `get_chapter_writing_state` Response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct ChapterWritingReadModelDto {
    pub project_id: Uuid,
    pub chapter_id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arc_id: Option<Uuid>,
    pub chapter_title: String,
    pub position: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    pub scenes: Vec<SceneReadModelDto>,
}

/// Command 30: `set_scene_lock` Request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct SetSceneLockReq {
    pub project_id: Uuid,
    pub chapter_id: Uuid,
    pub scene_id: Uuid,
    pub desired_locked: bool,
}

/// Command 30: `set_scene_lock` Response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct SceneLockStateDto {
    pub scene_id: Uuid,
    pub is_locked: bool,
    pub invalidated_generations: Vec<Uuid>,
}

/// Type alias for Command 30 response
pub type SceneLockResponseDto = SceneLockStateDto;

/// Command 31: `set_chapter_scenes_lock_bulk` Request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct SetChapterScenesLockBulkReq {
    pub project_id: Uuid,
    pub chapter_id: Uuid,
    pub is_locked: bool,
}

/// Command 31: `set_chapter_scenes_lock_bulk` Response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct BulkSceneLockStateDto {
    pub chapter_id: Uuid,
    pub scenes: Vec<SceneLockStateDto>,
}

/// Command 32: `start_scene_generation` Request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct StartSceneGenerationReq {
    pub project_id: Uuid,
    pub chapter_id: Uuid,
    pub scene_id: Uuid,
    pub mode: String, // "generate" | "regenerate"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_working_content: Option<String>,
}

/// Command 32: `start_scene_generation` Response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct SceneGenerationResultDto {
    pub request_id: Uuid,
    pub generation_id: Uuid,
    pub scene_id: Uuid,
    pub baseline_version_number: i32,
    pub status: GenerationStreamStatus,
    pub authorization_valid: bool,
    pub generated_text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normalized_note: Option<String>,
}

/// Command 33: `cancel_scene_generation` Request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct CancelSceneGenerationReq {
    pub chapter_id: Uuid,
    pub generation_id: Uuid,
}

/// Command 33: `cancel_scene_generation` Response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct CancelSceneGenerationResponseDto {
    pub generation_id: Uuid,
    pub status: String, // "cancelled"
}

/// Command 34: `respond_edit_scope_expansion` Request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct RespondEditScopeExpansionReq {
    pub chapter_id: Uuid,
    pub request_id: Uuid,
    pub target_scene_id: Uuid,
    pub approved: bool,
}

/// Command 34: `respond_edit_scope_expansion` Response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct RespondEditScopeExpansionResponseDto {
    pub request_id: Uuid,
    pub allowed_scene_ids: Vec<Uuid>,
    pub target_scene_is_locked: bool,
}

/// Command 35: `list_scene_versions` Request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct ListSceneVersionsReq {
    pub project_id: Uuid,
    pub chapter_id: Uuid,
    pub scene_id: Uuid,
}

/// Command 35: `list_scene_versions` Response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct SceneVersionListDto {
    pub scene_id: Uuid,
    pub head_version_number: i32,
    pub versions: Vec<SceneVersionItemDto>,
}

/// Command 36: `save_chapter` Request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct SaveChapterRequest {
    pub project_id: Uuid,
    pub chapter_id: Uuid,
    pub client_save_id: String,
    pub dirty_scenes: Vec<DirtySceneSaveItem>,
}

/// Type alias for Command 36 request
pub type SaveChapterReq = SaveChapterRequest;

/// Command 36: `save_chapter` Response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct SaveChapterResponse {
    pub chapter_id: Uuid,
    pub client_save_id: String,
    pub saved_scenes: Vec<SavedSceneResultDto>,
    pub chapter_scenes: Vec<SceneReadModelDto>,
    pub was_idempotent_replay: bool,
}

/// Type alias for Command 36 response
pub type SaveChapterResp = SaveChapterResponse;

/// Command 37: `get_chapter_save_receipt` Request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct GetChapterSaveReceiptReq {
    pub chapter_id: Uuid,
    pub client_save_id: String,
}

/// Command 37: `get_chapter_save_receipt` Response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct SaveReceiptLookupDto {
    pub found: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt: Option<SaveChapterResponse>,
    pub current_chapter_scenes: Vec<SceneReadModelDto>,
}

// ============================================================================
// Group 10: Model Configuration & Credentials (Commands 38..40)
// ============================================================================

/// Command 38: `get_runtime_and_provider_config` Request
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct GetRuntimeConfigReq {}

/// Command 38 & 39: `get_runtime_and_provider_config` / `update_runtime_and_provider_config` Response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeAndProviderConfigDto {
    pub subagent_config: SubagentConfigDto,
    pub providers: Vec<ProviderStatusDto>,
    pub credential_storage_mode: String, // "os_keychain" | "session_only_fallback"
}

/// Command 39: `update_runtime_and_provider_config` Request
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRuntimeConfigReq {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subagent_config: Option<SubagentConfigDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_provider_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_url: Option<String>,
}

/// Command 40: `set_provider_api_key` Request
///
/// STRICT SECURITY INVARIANT (P2S5-N):
/// `apiKey` field has `#[serde(skip_serializing)]` so that even if the request DTO
/// is serialized, the sensitive secret `apiKey` is never emitted!
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct SetProviderApiKeyReq {
    pub provider_id: String,
    #[serde(skip_serializing)]
    pub api_key: String,
}

/// Generic provider credential input DTO for testing/adapters.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct ProviderConfigInputDto {
    pub provider_id: String,
    #[serde(skip_serializing)]
    pub api_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_name: Option<String>,
}

/// Command 40: `set_provider_api_key` Response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct SetProviderApiKeyResponseDto {
    pub provider_id: String,
    pub storage_mode: String, // "os_keychain" | "session_only_fallback"
}

// ============================================================================
// Streaming Channel Event Payloads (Tauri v2 Channel<T>)
// ============================================================================

/// Channel 1: `ChatStreamEvent` (§10.3.1)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[serde(tag = "event", content = "data", rename_all = "camelCase")]
pub enum ChatStreamEvent {
    TurnStarted {
        turn_id: Uuid,
        user_message_id: Uuid,
    },
    AssistantDelta {
        turn_id: Uuid,
        delta: String,
    },
    ToolCallStarted {
        turn_id: Uuid,
        tool_call_id: String,
        tool_code: String,
    },
    ToolCallCompleted {
        turn_id: Uuid,
        tool_call_id: String,
        tool_code: String,
        status: String, // "applied" | "pending_confirmation" | "rejected"
        has_applied_write: bool,
    },
    WorkingStateUpdated {
        turn_id: Uuid,
        progress: ProgressSummaryDto,
        pending_patch_count: usize,
    },
    TurnCompleted {
        turn_id: Uuid,
        assistant_message_id: Uuid,
        sealed_version: Option<DraftVersionSummaryDto>,
    },
    TurnError {
        turn_id: Uuid,
        error: IpcError,
    },
}

/// Channel 2: `WorkflowProgressEvent` (§10.3.2)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[serde(tag = "event", content = "data", rename_all = "camelCase")]
pub enum WorkflowProgressEvent {
    WorkflowStarted {
        workflow_run_id: Uuid,
        plan_generation: i32,
        phase: i32,
        subphase: Option<i32>,
        total_rounds: u32,
        total_tasks: u32,
    },
    RoundStarted {
        workflow_run_id: Uuid,
        round_index: u32,
        active_task_identities: Vec<String>,
    },
    TaskCompleted {
        workflow_run_id: Uuid,
        task_identity: String,
        completed_tasks: u32,
        total_tasks: u32,
        was_skipped_by_receipt: bool,
    },
    RoundSealed {
        workflow_run_id: Uuid,
        round_index: u32,
        sealed_version: Option<DraftVersionSummaryDto>,
        progress: ProgressSummaryDto,
    },
    CancelRequested {
        workflow_run_id: Uuid,
    },
    WorkflowFinished {
        workflow_run_id: Uuid,
        status: String, // "completed" | "cancelled" | "failed"
        last_sealed_version: Option<DraftVersionSummaryDto>,
    },
}

/// Channel 3: `SceneGenerationStreamEvent` (§10.3.3)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, TS)]
#[serde(tag = "event", content = "data", rename_all = "camelCase")]
pub enum SceneGenerationStreamEvent {
    Started {
        request_id: Uuid,
        generation_id: Uuid,
        scene_id: Uuid,
        baseline_version_number: i32,
        authorization_valid: bool,
        status: GenerationStreamStatus,
    },
    Delta {
        request_id: Uuid,
        generation_id: Uuid,
        scene_id: Uuid,
        text_delta: String,
        authorization_valid: bool,
    },
    Completed {
        request_id: Uuid,
        generation_id: Uuid,
        scene_id: Uuid,
        baseline_version_number: i32,
        full_text: String,
        authorization_valid: bool,
        status: GenerationStreamStatus,
    },
    Cancelled {
        request_id: Uuid,
        generation_id: Uuid,
        scene_id: Uuid,
        partial_text: String,
        authorization_valid: bool,
        status: GenerationStreamStatus,
    },
    InvalidatedByLock {
        request_id: Uuid,
        generation_id: Uuid,
        scene_id: Uuid,
        partial_text: String,
        authorization_valid: bool, // Always false (LOCK-04, Q4.6)
        status: GenerationStreamStatus,
    },
    Failed {
        request_id: Uuid,
        generation_id: Uuid,
        scene_id: Uuid,
        error: IpcError,
        authorization_valid: bool,
        status: GenerationStreamStatus,
    },
}
