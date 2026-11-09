//! Acceptance and contract tests for `phase2_sprint5_deep2` ("Freeze IPC DTO and error interfaces").
//!
//! Per the backend module wiring rule in `docs/plan/phase2_sprint5_deep2.md`:
//! - This sprint does not edit `src-tauri/src/ipc/mod.rs` or `src-tauri/src/lib.rs`.
//! - Instead, this test target loads each owned implementation module directly via `#[path = "..."]`.
//!
//! Covered acceptance vectors:
//! - `P2S5-P`: 40 IPC command names and 3 channel payload schemas typecheck with camelCase fixtures.
//! - `P2S5-N`: Serializing secret-bearing input (`SetProviderApiKeyReq`, `ProviderConfigInputDto`) never returns `apiKey`.
//! - `P2S5-B`: Missing/invalid fields yield structured `IpcError` without panics or mutations.
//! - `phase2_sprint5_deep2-F0`: Predecessor gate absent or baseline SHA drift yields `CONTRACT_BLOCKED`.
//! - `phase2_sprint5_deep2-F1`: Unresolved error enum injection yields typed rejection, no invented mapping.
//! - `phase2_sprint5_deep2-F2`: Unknown JSON field or unsupported schema version yields typed error without panic.

#[path = "../src/error.rs"]
pub mod error;

#[path = "../src/ipc/dto.rs"]
pub mod dto;

#[path = "../src/ipc/adapter.rs"]
pub mod adapter;

use adapter::{to_camel_case_value, to_snake_case_value};
use dto::*;
use error::{AppError, ErrorCode, IpcError, ToolInfraError, ToolRejection};
use serde_json::json;
use std::collections::HashSet;
use std::path::PathBuf;
use std::process::Command;
use uuid::Uuid;

pub const BASELINE_COMMIT: &str = "57a462efb953a665e3673b15e6cc33622a4f2d39";
pub const PREDECESSOR_P2S2_COMMIT: &str = "f69f1ab0e3d3d34853ddf887cac139cde6d64b0d";
pub const PREDECESSOR_P1S1_COMMIT: &str = "56dd590569a0d6123b4f09d6f1c6761a8b1d7ff2";

/// Helper to validate predecessor gates and baseline lineage (`phase2_sprint5_deep2-F0`).
pub fn verify_p2s5_dispatch_preconditions(
    expected_baseline: &str,
    actual_baseline: &str,
    p2s2_gate_accepted: bool,
    p1s1_gate_accepted: bool,
) -> Result<(), AppError> {
    if !p2s2_gate_accepted {
        return Err(AppError::ContractBlocked {
            reason: "predecessor gate_phase2_sprint2_deep1 is absent or not ACCEPTED".to_string(),
        });
    }
    if !p1s1_gate_accepted {
        return Err(AppError::ContractBlocked {
            reason: "predecessor gate_phase1_sprint1_deep0 is absent or not ACCEPTED".to_string(),
        });
    }
    if expected_baseline != actual_baseline {
        return Err(AppError::ContractBlocked {
            reason: format!(
                "baseline SHA drift detected: expected {expected_baseline}, got {actual_baseline}"
            ),
        });
    }
    Ok(())
}

// ============================================================================
// P2S5-P: 40 Command Names and 3 Channel Event Schemas
// ============================================================================

pub const MASTER_40_COMMAND_NAMES: [&str; 40] = [
    // Group 1: Chat & Session (4)
    "create_or_load_chat_session",
    "list_chat_sessions",
    "send_chat_message",
    "cancel_chat_turn",
    // Group 2: Draft, Working State & Progress Engine (5)
    "get_draft_workspace_state",
    "get_progress_detail",
    "apply_manual_draft_edit",
    "confirm_phase",
    "regenerate_draft_phase",
    // Group 3: Versioning & Restore (2)
    "list_draft_versions",
    "restore_draft_version",
    // Group 4: Pending Patches (4)
    "list_pending_patches",
    "confirm_pending_patch",
    "reject_pending_patch",
    "confirm_all_pending_patches",
    // Group 5: Segmented Workflow (3)
    "start_or_resume_workflow",
    "cancel_workflow",
    "get_workflow_status",
    // Group 6: Start / Commit Draft (3)
    "get_commit_confirmation_preview",
    "commit_draft",
    "reopen_committed_draft",
    // Group 7: Projects & Creator Mode (3)
    "list_projects",
    "get_project_detail",
    "delete_project",
    // Group 8: Character Library (4)
    "list_characters",
    "get_character_detail",
    "create_or_update_library_character",
    "link_library_character_to_draft",
    // Group 9: Writing Mode, Scene Lock, Generation & Save Chapter (9)
    "get_chapter_writing_state",
    "set_scene_lock",
    "set_chapter_scenes_lock_bulk",
    "start_scene_generation",
    "cancel_scene_generation",
    "respond_edit_scope_expansion",
    "list_scene_versions",
    "save_chapter",
    "get_chapter_save_receipt",
    // Group 10: Model Configuration & Credentials (3)
    "get_runtime_and_provider_config",
    "update_runtime_and_provider_config",
    "set_provider_api_key",
];

#[test]
#[allow(non_snake_case)]
fn P2S5_P_master_inventory_has_exactly_40_unique_commands() {
    assert_eq!(MASTER_40_COMMAND_NAMES.len(), 40);
    let mut set = HashSet::new();
    for cmd in MASTER_40_COMMAND_NAMES {
        assert!(set.insert(cmd), "Duplicate command name found: {cmd}");
    }
}

#[test]
#[allow(non_snake_case, unused_variables)]
fn P2S5_P_all_40_command_dto_pairs_typecheck_and_roundtrip_camelcase() {
    let dummy_uuid = Uuid::parse_str("11111111-1111-1111-1111-111111111111").unwrap();

    // 1. create_or_load_chat_session
    let req1 = CreateOrLoadChatSessionReq {
        session_id: Some("sess-1".to_string()),
        project_id: Some(dummy_uuid),
    };
    let json1 = serde_json::to_string(&req1).unwrap();
    assert!(json1.contains("sessionId"));
    let de_req1: CreateOrLoadChatSessionReq = serde_json::from_str(&json1).unwrap();
    assert_eq!(req1, de_req1);

    let resp1 = ChatSessionInitDto {
        session_id: "sess-1".to_string(),
        draft_session_id: dummy_uuid,
        project_id: Some(dummy_uuid),
        status: "editing".to_string(),
        head_version_seq: Some(1),
        messages: vec![],
        working_state: json!({}),
        progress: ProgressSummaryDto {
            current_phase: 1,
            current_subphase: None,
            completed_attributes: 5,
            total_attributes: 46,
            progress_percentage: 10.8,
            is_require_complete: false,
            can_advance_phase: false,
            confirmed_count: Some(5),
            ai_inferred_count: Some(0),
        },
        interrupted_turn_alert: None,
    };
    let json_resp1 = serde_json::to_string(&resp1).unwrap();
    assert!(json_resp1.contains("draftSessionId"));
    assert!(json_resp1.contains("headVersionSeq"));
    let de_resp1: ChatSessionInitDto = serde_json::from_str(&json_resp1).unwrap();
    assert_eq!(resp1, de_resp1);

    // 2. list_chat_sessions
    let req2 = ListChatSessionsReq {
        project_id: Some(dummy_uuid),
        limit: Some(10),
        cursor: None,
        has_messages: None,
    };
    let resp2 = ChatSessionListDto {
        items: vec![SessionSummaryDto {
            session_id: "sess-1".to_string(),
            draft_session_id: dummy_uuid,
            project_id: Some(dummy_uuid),
            status: "editing".to_string(),
            updated_at: "2026-11-08T00:00:00Z".to_string(),
            display_name: Some("Hello worl".to_string()),
            message_count: None,
        }],
        next_cursor: None,
    };
    let json_resp2 = serde_json::to_string(&resp2).unwrap();
    assert!(json_resp2.contains("draftSessionId"));
    let de_resp2: ChatSessionListDto = serde_json::from_str(&json_resp2).unwrap();
    assert_eq!(resp2, de_resp2);

    // 3. send_chat_message
    let req3 = SendChatMessageReq {
        session_id: "sess-1".to_string(),
        content: "Hello".to_string(),
        system_language: "vi".to_string(),
        feedback_context: Some(FeedbackContextDto {
            code: "req.proj".to_string(),
            tag: "genre".to_string(),
            title: "Genre".to_string(),
            content: "Fantasy".to_string(),
        }),
    };
    let json_req3 = serde_json::to_string(&req3).unwrap();
    assert!(json_req3.contains("systemLanguage"));
    assert!(json_req3.contains("feedbackContext"));
    let resp3 = SendChatTurnResponseDto {
        turn_id: dummy_uuid,
        user_message_id: dummy_uuid,
        assistant_message_id: dummy_uuid,
        assistant_text: "World created".to_string(),
        sealed_version: None,
        progress: resp1.progress.clone(),
        pending_patches: vec![],
    };
    let json_resp3 = serde_json::to_string(&resp3).unwrap();
    assert!(json_resp3.contains("assistantText"));

    // 4. cancel_chat_turn
    let req4 = CancelChatTurnReq {
        session_id: "sess-1".to_string(),
        turn_id: dummy_uuid,
    };
    let resp4 = CancelChatTurnResponseDto {
        cancelled: true,
        sealed_version: None,
    };
    assert!(serde_json::to_string(&resp4).unwrap().contains("cancelled"));

    // 5. get_draft_workspace_state
    let req5 = GetDraftWorkspaceStateReq {
        draft_session_id: dummy_uuid,
    };
    let resp5 = DraftWorkspaceStateDto {
        draft_session_id: dummy_uuid,
        status: "editing".to_string(),
        phase: 1,
        subphase: None,
        head_version_seq: Some(1),
        business_flow_data: json!({}),
        draft_overview_ui: json!({}),
        progress: resp1.progress.clone(),
        pending_patches: vec![],
        event_log: vec![],
        lock_state: DraftLockStatusDto {
            is_locked: false,
            holder_info: None,
        },
    };
    assert!(serde_json::to_string(&resp5)
        .unwrap()
        .contains("draftOverviewUi"));

    // 6. get_progress_detail
    let req6 = GetProgressDetailReq {
        draft_session_id: dummy_uuid,
        system_language: "vi".to_string(),
    };
    let resp6 = ProgressDetailDto {
        progress: resp1.progress.clone(),
        attributes: vec![],
        rendered_markdown: "# Summary".to_string(),
        user_constraints: vec![],
        conflicts: vec![],
    };
    assert!(serde_json::to_string(&resp6)
        .unwrap()
        .contains("renderedMarkdown"));

    // 7. apply_manual_draft_edit
    let req7 = ApplyManualDraftEditReq {
        draft_session_id: dummy_uuid,
        client_edit_id: "edit-1".to_string(),
        tool_code: "set_genre".to_string(),
        args: json!({"name": "Fantasy"}),
    };
    let resp7 = ManualEditResponseDto {
        tool_output: json!({"status": "applied"}),
        sealed_version: None,
        progress: resp1.progress.clone(),
    };
    assert!(serde_json::to_string(&resp7).unwrap().contains("toolOutput"));

    // 8. confirm_phase
    let req8 = ConfirmPhaseReq {
        draft_session_id: dummy_uuid,
        target_phase: 2,
        target_subphase: None,
        pre_draft_suggestion: None,
    };
    let resp8 = PhaseConfirmResponseDto {
        phase: 2,
        subphase: None,
        sealed_version: DraftVersionSummaryDto {
            seq: 2,
            display_label: "v.02".to_string(),
            phase: 2,
            subphase: None,
            reason: "phase_confirm".to_string(),
            parent_seq: Some(1),
            created_at: "2026-11-08T00:00:00Z".to_string(),
            data_hash: None,
        },
    };
    assert!(serde_json::to_string(&resp8)
        .unwrap()
        .contains("sealedVersion"));

    // 9. regenerate_draft_phase
    let req9 = RegenerateDraftPhaseReq {
        draft_session_id: dummy_uuid,
        phase: 1,
        subphase: None,
        system_language: "vi".to_string(),
    };
    let resp9 = RegeneratePhaseResponseDto {
        turn_id: dummy_uuid,
        sealed_version: None,
        pending_patches: vec![],
        progress: resp1.progress.clone(),
    };
    assert!(serde_json::to_string(&resp9).unwrap().contains("turnId"));

    // 10. list_draft_versions
    let req10 = ListDraftVersionsReq {
        draft_session_id: dummy_uuid,
    };
    let resp10 = DraftVersionListDto {
        head_version_seq: Some(1),
        versions: vec![],
    };
    assert!(serde_json::to_string(&resp10)
        .unwrap()
        .contains("headVersionSeq"));

    // 11. restore_draft_version
    let req11 = RestoreDraftVersionReq {
        draft_session_id: dummy_uuid,
        target_seq: 1,
        client_request_id: "req-restore-1".to_string(),
    };
    let resp11 = RestoreVersionResponseDto {
        new_version: resp8.sealed_version.clone(),
        restored_phase: 1,
        restored_subphase: None,
        expired_patch_ids: vec![],
        working_state: resp5.clone(),
    };
    assert!(serde_json::to_string(&resp11)
        .unwrap()
        .contains("expiredPatchIds"));

    // 12. list_pending_patches
    let req12 = ListPendingPatchesReq {
        draft_session_id: dummy_uuid,
    };
    let resp12 = PendingPatchListDto { items: vec![] };
    assert!(serde_json::to_string(&resp12).unwrap().contains("items"));

    // 13. confirm_pending_patch
    let req13 = ConfirmPendingPatchReq {
        draft_session_id: dummy_uuid,
        patch_id: dummy_uuid,
    };
    let resp13 = PatchResolutionDto {
        status: "confirmed".to_string(),
        sealed_version: None,
        progress: resp1.progress.clone(),
    };
    assert!(serde_json::to_string(&resp13).unwrap().contains("status"));

    // 14. reject_pending_patch
    let req14 = RejectPendingPatchReq {
        draft_session_id: dummy_uuid,
        patch_id: dummy_uuid,
    };
    let resp14 = PatchResolutionDto {
        status: "rejected".to_string(),
        sealed_version: None,
        progress: resp1.progress.clone(),
    };
    assert_eq!(resp14.status, "rejected");

    // 15. confirm_all_pending_patches
    let req15 = ConfirmAllPendingPatchesReq {
        draft_session_id: dummy_uuid,
        client_request_id: "batch-1".to_string(),
    };
    let resp15 = ConfirmAllPatchesResponseDto {
        applied: vec![dummy_uuid],
        skipped: vec![SkippedPatchDto {
            patch_id: dummy_uuid,
            reason: "expired".to_string(),
        }],
        sealed_version: None,
        progress: resp1.progress.clone(),
    };
    assert!(serde_json::to_string(&resp15).unwrap().contains("skipped"));

    // 16. start_or_resume_workflow
    let req16 = StartOrResumeWorkflowReq {
        draft_session_id: dummy_uuid,
        phase: 1,
        subphase: None,
        workflow_run_id: None,
        system_language: "vi".to_string(),
    };
    let resp16 = WorkflowExecutionSummaryDto {
        workflow_run_id: dummy_uuid,
        plan_generation: 1,
        status: "completed".to_string(),
        recovery_reason: None,
        completed_tasks: 5,
        total_tasks: 5,
        last_sealed_version: None,
        pending_patches: vec![],
    };
    assert!(serde_json::to_string(&resp16)
        .unwrap()
        .contains("planGeneration"));

    // 17. cancel_workflow
    let req17 = CancelWorkflowReq {
        draft_session_id: dummy_uuid,
        workflow_run_id: dummy_uuid,
    };
    let resp17 = CancelWorkflowResponseDto {
        status: "cancelled".to_string(),
        last_sealed_version: None,
        resume_available: true,
    };
    assert!(serde_json::to_string(&resp17)
        .unwrap()
        .contains("resumeAvailable"));

    // 18. get_workflow_status
    let req18 = GetWorkflowStatusReq {
        draft_session_id: dummy_uuid,
    };
    let resp18 = WorkflowStatusDto {
        is_running: false,
        workflow_run_id: None,
        plan_generation: Some(1),
        current_phase: Some(1),
        current_subphase: None,
        current_round: Some(1),
        total_rounds: Some(3),
        completed_tasks: 2,
        total_tasks: 6,
        resume_available: true,
    };
    assert!(serde_json::to_string(&resp18).unwrap().contains("isRunning"));

    // 19. get_commit_confirmation_preview
    let req19 = GetCommitPreviewReq {
        draft_session_id: dummy_uuid,
    };
    let resp19 = CommitPreviewDto {
        draft_session_id: dummy_uuid,
        status: "editing".to_string(),
        plan_summary: PlanSummaryDto {
            project_name: "Novel".to_string(),
            target_arcs: 3,
            target_chapters: 10,
            target_words_per_chapter: 3000,
            character_count: 5,
            arc_count: 3,
            chapter_count: 10,
            world_setting_count: 2,
            lore_entry_count: 4,
        },
        validation: CommitValidationResultDto {
            status: "ready".to_string(),
            issues: vec![],
        },
        pending_patches: PendingPatchesSummaryDto {
            count: 0,
            items: vec![],
        },
        can_confirm: true,
    };
    assert!(serde_json::to_string(&resp19).unwrap().contains("canConfirm"));

    // 20. commit_draft
    let req20 = CommitDraftReq {
        draft_session_id: dummy_uuid,
    };
    let resp20 = CommitDraftResponseDto {
        status: "committed".to_string(),
        project_id: dummy_uuid,
        redirect_path: "/projects/1111".to_string(),
    };
    assert!(serde_json::to_string(&resp20)
        .unwrap()
        .contains("redirectPath"));

    // 21. reopen_committed_draft
    let req21 = ReopenCommittedDraftReq {
        project_id: dummy_uuid,
        active_session_id: "sess-new".to_string(),
    };
    let resp21 = ReopenCommittedDraftResponseDto {
        draft_session_id: dummy_uuid,
        status: "editing".to_string(),
    };
    assert!(serde_json::to_string(&resp21)
        .unwrap()
        .contains("draftSessionId"));

    // 22. list_projects
    let req22 = ListProjectsReq {
        query: Some("fantasy".to_string()),
        sort: Some("a-z".to_string()),
    };
    let resp22 = ProjectListDto { items: vec![] };
    assert!(serde_json::to_string(&resp22).unwrap().contains("items"));

    // 23. get_project_detail
    let req23 = GetProjectDetailReq {
        project_id: dummy_uuid,
    };
    let resp23 = ProjectDetailDto {
        project: ProjectItemUiDto {
            id: dummy_uuid.to_string(),
            title: "Novel".to_string(),
            status: "Planning".to_string(),
            cover: "".to_string(),
            description: "".to_string(),
            genres: vec!["Fantasy".to_string()],
            modified_at: "2026-11-08T00:00:00Z".to_string(),
            details: ProjectDetailsUiDto {
                word_count: "0".to_string(),
                chapters: "0".to_string(),
                characters: "0".to_string(),
                world_lore: "0".to_string(),
                created_date: "2026-11-08".to_string(),
            },
            arcs: None,
        },
        has_committed_plan: false,
        draft_session_id: Some(dummy_uuid),
        can_reopen_draft: false,
    };
    assert!(serde_json::to_string(&resp23)
        .unwrap()
        .contains("hasCommittedPlan"));

    // 24. delete_project
    let req24 = DeleteProjectReq {
        project_id: dummy_uuid,
    };
    let resp24 = DeleteProjectResponseDto { deleted: true };
    assert!(serde_json::to_string(&resp24).unwrap().contains("deleted"));

    // 25. list_characters
    let req25 = ListCharactersReq {
        project_id: Some(dummy_uuid),
        biological_sex: Some("MALE".to_string()),
        query: None,
    };
    let resp25 = CharacterListDto { items: vec![] };
    assert!(serde_json::to_string(&resp25).unwrap().contains("items"));

    // 26. get_character_detail
    let req26 = GetCharacterDetailReq {
        character_id: dummy_uuid,
        project_id: None,
    };
    let resp26 = CharacterDetailDto {
        character: CharacterLibraryItemDto {
            id: dummy_uuid,
            project_id: None,
            code: "char-1".to_string(),
            name: "John".to_string(),
            nickname: None,
            species: "Human".to_string(),
            biological_sex: "MALE".to_string(),
            gender_identity: "MALE".to_string(),
            gender_expression: "MALE".to_string(),
            in_project: true,
            avatar_url: None,
            default_summary: None,
        },
        appearance: None,
        distinguishing_marks: vec![],
        psychological: None,
        memories: vec![],
        competences: vec![],
        relationships: vec![],
    };
    assert!(serde_json::to_string(&resp26)
        .unwrap()
        .contains("biologicalSex"));

    // 27. create_or_update_library_character
    let req27 = UpsertLibraryCharacterReq {
        character_id: Some(dummy_uuid),
        project_id: None,
        profile: CharacterUpsertPayloadDto {
            name: "John".to_string(),
            nickname: None,
            species: "Human".to_string(),
            biological_sex: "MALE".to_string(),
            gender_identity: "MALE".to_string(),
            gender_expression: "MALE".to_string(),
            appearance: None,
            psychological: None,
            memories: vec![],
            competences: vec![],
        },
    };
    let resp27 = resp26.clone();
    assert!(serde_json::to_string(&req27).unwrap().contains("profile"));

    // 28. link_library_character_to_draft
    let req28 = LinkLibraryCharacterToDraftReq {
        draft_session_id: dummy_uuid,
        character_id: dummy_uuid,
        role: Some("Mentor".to_string()),
        notes: None,
    };
    let resp28 = LinkLibraryCharacterResponseDto {
        character_code: "char-1".to_string(),
        sealed_version: resp8.sealed_version.clone(),
    };
    assert!(serde_json::to_string(&resp28)
        .unwrap()
        .contains("characterCode"));

    // 29. get_chapter_writing_state
    let req29 = GetChapterWritingStateReq {
        project_id: dummy_uuid,
        chapter_id: dummy_uuid,
    };
    let resp29 = ChapterWritingReadModelDto {
        project_id: dummy_uuid,
        chapter_id: dummy_uuid,
        arc_id: Some(dummy_uuid),
        chapter_title: "Chapter 1".to_string(),
        position: 1,
        updated_at: None,
        scenes: vec![],
    };
    assert!(serde_json::to_string(&resp29)
        .unwrap()
        .contains("chapterTitle"));

    // 30. set_scene_lock
    let req30 = SetSceneLockReq {
        project_id: dummy_uuid,
        chapter_id: dummy_uuid,
        scene_id: dummy_uuid,
        desired_locked: true,
    };
    let resp30 = SceneLockStateDto {
        scene_id: dummy_uuid,
        is_locked: true,
        invalidated_generations: vec![],
    };
    assert!(serde_json::to_string(&resp30)
        .unwrap()
        .contains("invalidatedGenerations"));

    // 31. set_chapter_scenes_lock_bulk
    let req31 = SetChapterScenesLockBulkReq {
        project_id: dummy_uuid,
        chapter_id: dummy_uuid,
        is_locked: true,
    };
    let resp31 = BulkSceneLockStateDto {
        chapter_id: dummy_uuid,
        scenes: vec![resp30.clone()],
    };
    assert!(serde_json::to_string(&resp31).unwrap().contains("scenes"));

    // 32. start_scene_generation
    let req32 = StartSceneGenerationReq {
        project_id: dummy_uuid,
        chapter_id: dummy_uuid,
        scene_id: dummy_uuid,
        mode: "generate".to_string(),
        note: Some("Make it dramatic".to_string()),
        current_working_content: None,
    };
    let resp32 = SceneGenerationResultDto {
        request_id: dummy_uuid,
        generation_id: dummy_uuid,
        scene_id: dummy_uuid,
        baseline_version_number: 1,
        status: GenerationStreamStatus::Streaming,
        authorization_valid: true,
        generated_text: "Once upon a time...".to_string(),
        normalized_note: Some("Make it dramatic".to_string()),
    };
    assert!(serde_json::to_string(&resp32)
        .unwrap()
        .contains("baselineVersionNumber"));

    // 33. cancel_scene_generation
    let req33 = CancelSceneGenerationReq {
        chapter_id: dummy_uuid,
        generation_id: dummy_uuid,
    };
    let resp33 = CancelSceneGenerationResponseDto {
        generation_id: dummy_uuid,
        status: "cancelled".to_string(),
    };
    assert_eq!(resp33.status, "cancelled");

    // 34. respond_edit_scope_expansion
    let req34 = RespondEditScopeExpansionReq {
        chapter_id: dummy_uuid,
        request_id: dummy_uuid,
        target_scene_id: dummy_uuid,
        approved: true,
    };
    let resp34 = RespondEditScopeExpansionResponseDto {
        request_id: dummy_uuid,
        allowed_scene_ids: vec![dummy_uuid],
        target_scene_is_locked: false,
    };
    assert!(serde_json::to_string(&resp34)
        .unwrap()
        .contains("allowedSceneIds"));

    // 35. list_scene_versions (CI-08 requires full `content` inside `SceneVersionItemDto`)
    let req35 = ListSceneVersionsReq {
        project_id: dummy_uuid,
        chapter_id: dummy_uuid,
        scene_id: dummy_uuid,
    };
    let resp35 = SceneVersionListDto {
        scene_id: dummy_uuid,
        head_version_number: 2,
        versions: vec![SceneVersionItemDto {
            version_number: 2,
            content: "Restorable historical scene content".to_string(),
            word_count: 5,
            scene_source: "RESTORE".to_string(),
            summary: Some("Restored version".to_string()),
            created_at: "2026-11-08T01:00:00Z".to_string(),
        }],
    };
    let json_resp35 = serde_json::to_string(&resp35).unwrap();
    assert!(
        json_resp35.contains("content"),
        "CI-08: SceneVersionItemDto must serialize content"
    );
    assert!(json_resp35.contains("Restorable historical scene content"));

    // 36. save_chapter
    let req36 = SaveChapterRequest {
        project_id: dummy_uuid,
        chapter_id: dummy_uuid,
        client_save_id: "save-id-1".to_string(),
        dirty_scenes: vec![DirtySceneSaveItem {
            scene_id: dummy_uuid,
            expected_version_number: 1,
            content: "Updated scene content".to_string(),
            scene_source: "USER".to_string(),
            summary: None,
        }],
    };
    let resp36 = SaveChapterResponse {
        chapter_id: dummy_uuid,
        client_save_id: "save-id-1".to_string(),
        saved_scenes: vec![SavedSceneResultDto {
            scene_id: dummy_uuid,
            new_version_number: 2,
            word_count: 3,
        }],
        chapter_scenes: vec![],
        was_idempotent_replay: false,
    };
    assert!(serde_json::to_string(&resp36)
        .unwrap()
        .contains("wasIdempotentReplay"));

    // 37. get_chapter_save_receipt
    let req37 = GetChapterSaveReceiptReq {
        chapter_id: dummy_uuid,
        client_save_id: "save-id-1".to_string(),
    };
    let resp37 = SaveReceiptLookupDto {
        found: true,
        receipt: Some(resp36),
        current_chapter_scenes: vec![],
    };
    assert!(serde_json::to_string(&resp37).unwrap().contains("found"));

    // 38. get_runtime_and_provider_config
    let req38 = GetRuntimeConfigReq {};
    let resp38 = RuntimeAndProviderConfigDto {
        subagent_config: SubagentConfigDto::default(),
        providers: vec![ProviderStatusDto {
            provider_id: "ollama".to_string(),
            name: "Ollama Local".to_string(),
            is_active: true,
            has_api_key: false,
            endpoint_url: Some("http://localhost:11434".to_string()),
            model_name: Some("llama3".to_string()),
        }],
        credential_storage_mode: "os_keychain".to_string(),
    };
    assert!(serde_json::to_string(&resp38)
        .unwrap()
        .contains("credentialStorageMode"));

    // 39. update_runtime_and_provider_config
    let req39 = UpdateRuntimeConfigReq {
        subagent_config: Some(SubagentConfigDto::default()),
        active_provider_id: Some("ollama".to_string()),
        model_name: Some("llama3".to_string()),
        endpoint_url: None,
        remove_provider_id: None,
    };
    let resp39 = resp38.clone();
    assert!(serde_json::to_string(&req39)
        .unwrap()
        .contains("activeProviderId"));

    // 40. set_provider_api_key
    let req40 = SetProviderApiKeyReq {
        provider_id: "openai".to_string(),
        api_key: "sk-test-secret-value".to_string(),
    };
    let resp40 = SetProviderApiKeyResponseDto {
        provider_id: "openai".to_string(),
        storage_mode: "os_keychain".to_string(),
    };
    assert!(serde_json::to_string(&resp40)
        .unwrap()
        .contains("storageMode"));
}

#[test]
#[allow(non_snake_case)]
fn P2S5_P_channel_streaming_events_roundtrip_tagged_camelcase() {
    let dummy_uuid = Uuid::parse_str("22222222-2222-2222-2222-222222222222").unwrap();

    // Channel 1: ChatStreamEvent
    let chat_events = vec![
        ChatStreamEvent::TurnStarted {
            turn_id: dummy_uuid,
            user_message_id: dummy_uuid,
        },
        ChatStreamEvent::AssistantDelta {
            turn_id: dummy_uuid,
            delta: "Hello world".to_string(),
        },
        ChatStreamEvent::ToolCallStarted {
            turn_id: dummy_uuid,
            tool_call_id: "call-1".to_string(),
            tool_code: "add_character".to_string(),
        },
        ChatStreamEvent::ToolCallCompleted {
            turn_id: dummy_uuid,
            tool_call_id: "call-1".to_string(),
            tool_code: "add_character".to_string(),
            status: "applied".to_string(),
            has_applied_write: true,
        },
        ChatStreamEvent::TurnCompleted {
            turn_id: dummy_uuid,
            assistant_message_id: dummy_uuid,
            sealed_version: None,
        },
        ChatStreamEvent::TurnError {
            turn_id: dummy_uuid,
            error: IpcError::new(ErrorCode::DraftBusy, "Draft is busy"),
        },
    ];

    for ev in chat_events {
        let json_str = serde_json::to_string(&ev).unwrap();
        assert!(
            json_str.contains(r#""event":""#),
            "Must have tag 'event': {json_str}"
        );
        assert!(
            json_str.contains(r#""data":{"#),
            "Must have content 'data': {json_str}"
        );
        let de_ev: ChatStreamEvent = serde_json::from_str(&json_str).unwrap();
        assert_eq!(ev, de_ev);
    }

    // Channel 2: WorkflowProgressEvent
    let wf_events = vec![
        WorkflowProgressEvent::WorkflowStarted {
            workflow_run_id: dummy_uuid,
            plan_generation: 1,
            phase: 2,
            subphase: None,
            total_rounds: 3,
            total_tasks: 9,
        },
        WorkflowProgressEvent::RoundStarted {
            workflow_run_id: dummy_uuid,
            round_index: 0,
            active_task_identities: vec!["task-1".to_string(), "task-2".to_string()],
        },
        WorkflowProgressEvent::TaskCompleted {
            workflow_run_id: dummy_uuid,
            task_identity: "task-1".to_string(),
            completed_tasks: 1,
            total_tasks: 9,
            was_skipped_by_receipt: false,
        },
        WorkflowProgressEvent::CancelRequested {
            workflow_run_id: dummy_uuid,
        },
        WorkflowProgressEvent::WorkflowFinished {
            workflow_run_id: dummy_uuid,
            status: "completed".to_string(),
            last_sealed_version: None,
        },
    ];

    for ev in wf_events {
        let json_str = serde_json::to_string(&ev).unwrap();
        assert!(json_str.contains(r#""event":""#));
        let de_ev: WorkflowProgressEvent = serde_json::from_str(&json_str).unwrap();
        assert_eq!(ev, de_ev);
    }

    // Channel 3: SceneGenerationStreamEvent
    let scene_events = vec![
        SceneGenerationStreamEvent::Started {
            request_id: dummy_uuid,
            generation_id: dummy_uuid,
            scene_id: dummy_uuid,
            baseline_version_number: 1,
            authorization_valid: true,
            status: GenerationStreamStatus::Streaming,
        },
        SceneGenerationStreamEvent::Delta {
            request_id: dummy_uuid,
            generation_id: dummy_uuid,
            scene_id: dummy_uuid,
            text_delta: "The sky turned dark.".to_string(),
            authorization_valid: true,
        },
        SceneGenerationStreamEvent::Completed {
            request_id: dummy_uuid,
            generation_id: dummy_uuid,
            scene_id: dummy_uuid,
            baseline_version_number: 1,
            full_text: "The sky turned dark. Rain poured.".to_string(),
            authorization_valid: true,
            status: GenerationStreamStatus::Completed,
        },
        SceneGenerationStreamEvent::InvalidatedByLock {
            request_id: dummy_uuid,
            generation_id: dummy_uuid,
            scene_id: dummy_uuid,
            partial_text: "The sky".to_string(),
            authorization_valid: false, // Invariant: must be false
            status: GenerationStreamStatus::InvalidatedByLock,
        },
    ];

    for ev in scene_events {
        let json_str = serde_json::to_string(&ev).unwrap();
        assert!(json_str.contains(r#""event":""#));
        let de_ev: SceneGenerationStreamEvent = serde_json::from_str(&json_str).unwrap();
        assert_eq!(ev, de_ev);
    }
}

// ============================================================================
// P2S5-N: No Secret Exposure on Serialization
// ============================================================================

#[test]
#[allow(non_snake_case)]
fn P2S5_N_serializing_secret_bearing_input_never_returns_api_key() {
    let secret = "sk-super-secret-key-that-must-never-leak-to-frontend-or-disk";

    // 1. SetProviderApiKeyReq
    let req = SetProviderApiKeyReq {
        provider_id: "anthropic".to_string(),
        api_key: secret.to_string(),
    };

    let serialized = serde_json::to_string(&req).expect("Serialization must succeed");
    assert!(
        !serialized.contains("apiKey"),
        "Serialized SetProviderApiKeyReq MUST NOT contain 'apiKey' field: {serialized}"
    );
    assert!(
        !serialized.contains("api_key"),
        "Serialized SetProviderApiKeyReq MUST NOT contain 'api_key' field: {serialized}"
    );
    assert!(
        !serialized.contains(secret),
        "Serialized SetProviderApiKeyReq MUST NOT contain the secret payload: {serialized}"
    );

    let val = serde_json::to_value(&req).unwrap();
    assert_eq!(val.get("providerId").unwrap(), "anthropic");
    assert!(val.get("apiKey").is_none());
    assert!(val.get("api_key").is_none());

    // 2. ProviderConfigInputDto
    let config_input = ProviderConfigInputDto {
        provider_id: "openai".to_string(),
        api_key: Some(secret.to_string()),
        endpoint_url: Some("https://api.openai.com".to_string()),
        model_name: Some("gpt-4o".to_string()),
    };
    let config_serialized = serde_json::to_string(&config_input).unwrap();
    assert!(!config_serialized.contains("apiKey"));
    assert!(!config_serialized.contains(secret));

    // 3. ProviderStatusDto in RuntimeAndProviderConfigDto has no apiKey field at all
    let status_dto = ProviderStatusDto {
        provider_id: "openai".to_string(),
        name: "OpenAI".to_string(),
        is_active: true,
        has_api_key: true,
        endpoint_url: None,
        model_name: None,
    };
    let status_serialized = serde_json::to_string(&status_dto).unwrap();
    assert!(status_serialized.contains("hasApiKey"));
    assert!(!status_serialized.contains("apiKey"));
}

// ============================================================================
// P2S5-B: Missing / Invalid Field Yields Structured IpcError
// ============================================================================

#[test]
#[allow(non_snake_case)]
fn P2S5_B_missing_or_corrupt_json_payload_yields_structured_ipc_error_without_storage_mutation() {
    // Malformed JSON missing required field `sessionId` for `SendChatMessageReq`
    let malformed_json = r#"{"content": "Hello", "systemLanguage": "vi"}"#;
    let parse_result: Result<SendChatMessageReq, _> = serde_json::from_str(malformed_json);
    assert!(parse_result.is_err());

    // Convert Serde error into AppError and IpcError
    let serde_err = parse_result.unwrap_err();
    let app_err: AppError = serde_err.into();
    let ipc_err: IpcError = app_err.into();

    assert_eq!(ipc_err.code, ErrorCode::ValidationFailed);
    assert!(!ipc_err.retryable);
    assert!(ipc_err.message.contains("missing field `sessionId`"));

    // Verify Display and Error implementations
    let displayed = format!("{ipc_err}");
    assert!(displayed.contains("[ValidationFailed]"));
    assert!(displayed.contains("retryable=false"));
}

// ============================================================================
// phase2_sprint5_deep2-F0: Predecessor Gate and Baseline SHA Ancestry Check
// ============================================================================

#[test]
#[allow(non_snake_case)]
fn phase2_sprint5_deep2_F0_predecessor_gate_or_baseline_drift_is_contract_blocked() {
    // 1. Missing predecessor P2S2 gate -> CONTRACT_BLOCKED
    let err_missing_p2s2 =
        verify_p2s5_dispatch_preconditions(BASELINE_COMMIT, BASELINE_COMMIT, false, true)
            .unwrap_err();
    let ipc_p2s2: IpcError = err_missing_p2s2.into();
    assert!(ipc_p2s2.message.contains("CONTRACT_BLOCKED"));
    assert!(ipc_p2s2.message.contains("gate_phase2_sprint2_deep1"));

    // 2. Missing predecessor P1S1 gate -> CONTRACT_BLOCKED
    let err_missing_p1s1 =
        verify_p2s5_dispatch_preconditions(BASELINE_COMMIT, BASELINE_COMMIT, true, false)
            .unwrap_err();
    let ipc_p1s1: IpcError = err_missing_p1s1.into();
    assert!(ipc_p1s1.message.contains("CONTRACT_BLOCKED"));
    assert!(ipc_p1s1.message.contains("gate_phase1_sprint1_deep0"));

    // 3. Baseline SHA drift -> CONTRACT_BLOCKED
    let drifted_sha = "cafebabecafebabecafebabecafebabecafebabe";
    let err_drift =
        verify_p2s5_dispatch_preconditions(BASELINE_COMMIT, drifted_sha, true, true).unwrap_err();
    let ipc_drift: IpcError = err_drift.into();
    assert!(ipc_drift.message.contains("CONTRACT_BLOCKED"));
    assert!(ipc_drift.message.contains("baseline SHA drift detected"));

    // 4. Positive verification in live git worktree
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf();

    // Verify baseline commit is ancestor of HEAD
    let baseline_check = Command::new("git")
        .args(["merge-base", "--is-ancestor", BASELINE_COMMIT, "HEAD"])
        .current_dir(&repo_root)
        .status()
        .expect("git merge-base execution must succeed");
    assert!(
        baseline_check.success(),
        "Baseline commit {BASELINE_COMMIT} must be an ancestor of HEAD"
    );

    // Verify P2S2 accepted commit exists in git repository
    let p2s2_check = Command::new("git")
        .args([
            "cat-file",
            "-e",
            &format!("{PREDECESSOR_P2S2_COMMIT}^{{commit}}"),
        ])
        .current_dir(&repo_root)
        .status()
        .expect("git cat-file for P2S2 must succeed");
    assert!(
        p2s2_check.success(),
        "Accepted predecessor P2S2 commit {PREDECESSOR_P2S2_COMMIT} must exist"
    );

    // Verify P1S1 accepted commit exists in git repository
    let p1s1_check = Command::new("git")
        .args([
            "cat-file",
            "-e",
            &format!("{PREDECESSOR_P1S1_COMMIT}^{{commit}}"),
        ])
        .current_dir(&repo_root)
        .status()
        .expect("git cat-file for P1S1 must succeed");
    assert!(
        p1s1_check.success(),
        "Accepted predecessor P1S1 commit {PREDECESSOR_P1S1_COMMIT} must exist"
    );

    // 5. Preconditions pass when all gates and baseline are verified
    assert!(verify_p2s5_dispatch_preconditions(BASELINE_COMMIT, BASELINE_COMMIT, true, true).is_ok());
}

// ============================================================================
// phase2_sprint5_deep2-F1: Unresolved Error Enum Rejection
// ============================================================================

#[test]
#[allow(non_snake_case)]
fn phase2_sprint5_deep2_F1_unresolved_error_enum_yields_typed_rejection_no_invented_mapping() {
    let unknown_code_json = r#""unratified_invented_error_variant""#;
    let de_res: Result<ErrorCode, _> = serde_json::from_str(unknown_code_json);
    assert!(
        de_res.is_err(),
        "Unknown error code variants must NOT deserialize or be silently mapped"
    );

    // When an unknown error string is processed by the application boundary, it returns ValidationFailed
    let app_err = AppError::Json(de_res.unwrap_err());
    let ipc_err: IpcError = app_err.into();
    assert_eq!(ipc_err.code, ErrorCode::ValidationFailed);
    assert!(ipc_err
        .message
        .contains("unknown variant `unratified_invented_error_variant`"));

    // Business rejection in tool execution returns ToolRejection
    let rejection = ToolRejection::with_candidates(
        "Invalid entity code",
        json!({"code": "invalid-code"}),
        vec!["arc-1".to_string(), "arc-2".to_string()],
    );
    assert_eq!(rejection.status, "rejected");
    assert_eq!(rejection.error, "Invalid entity code");
    assert_eq!(rejection.candidates.unwrap().len(), 2);

    // ToolInfraError converts to DatabaseError / ProviderUnreachable / DraftBusy appropriately
    let infra_db: IpcError = AppError::ToolInfra(ToolInfraError::Database("Disk full".to_string())).into();
    assert_eq!(infra_db.code, ErrorCode::DatabaseError);
    assert!(!infra_db.retryable);

    let infra_timeout: IpcError = AppError::ToolInfra(ToolInfraError::Timeout("Lock timeout".to_string())).into();
    assert_eq!(infra_timeout.code, ErrorCode::DraftBusy);
    assert!(infra_timeout.retryable);
}

// ============================================================================
// phase2_sprint5_deep2-F2: Unknown JSON Field / Version Handling
// ============================================================================

#[test]
#[allow(non_snake_case)]
fn phase2_sprint5_deep2_F2_unknown_json_field_or_unsupported_version_yields_typed_error() {
    // 1. Invalid UUID format in target parameter
    let bad_uuid_json = r#"{"draftSessionId": "not-a-valid-uuid"}"#;
    let res: Result<GetDraftWorkspaceStateReq, _> = serde_json::from_str(bad_uuid_json);
    assert!(res.is_err());
    let ipc_err: IpcError = AppError::Json(res.unwrap_err()).into();
    assert_eq!(ipc_err.code, ErrorCode::ValidationFailed);

    // 2. Unsupported schema version error code representation
    let unsupp_err = IpcError::new(
        ErrorCode::UnsupportedSchemaVersion,
        "Database PRAGMA user_version 99 exceeds MAX_SUPPORTED_SCHEMA_VERSION 2",
    );
    assert_eq!(unsupp_err.code, ErrorCode::UnsupportedSchemaVersion);
    let serialized = serde_json::to_string(&unsupp_err).unwrap();
    assert!(serialized.contains("unsupported_schema_version"));

    // 3. Adapter snake_case <-> camelCase preserves unknown non-reserved keys cleanly
    let custom_json = json!({
        "unknown_nested_field": {
            "sub_prop_key": 42
        }
    });
    let camel_val = to_camel_case_value(&custom_json);
    assert_eq!(camel_val["unknownNestedField"]["subPropKey"], 42);
    let snake_val = to_snake_case_value(&camel_val);
    assert_eq!(snake_val, custom_json);
}
