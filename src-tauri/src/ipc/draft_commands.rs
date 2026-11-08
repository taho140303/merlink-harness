//! Draft, Working State, Progress & Versioning IPC Commands (#5, #6, #7, #8, #10, #11)
//! (`REQ-CMD-005`..`008`, `REQ-CMD-010`..`011`, `be-core.md` §10.2.2, §10.2.3, `CI-03`, `CI-08`).

use crate::domain::fingerprint::state_data_hash;
use crate::domain::progress_engine::{evaluate_progress, MasterAttributePhaseGroup};
use crate::error::{ErrorCode, IpcError};
use crate::ipc::dto::{
    ApplyManualDraftEditReq, AttributeEvaluationDetailDto, ConfirmPhaseReq,
    DraftVersionListDto, DraftWorkspaceStateDto, GetDraftWorkspaceStateReq,
    GetProgressDetailReq, ListDraftVersionsReq, ManualEditResponseDto,
    PhaseConfirmResponseDto, ProgressDetailDto, ProgressSummaryDto,
    RestoreDraftVersionReq, RestoreVersionResponseDto,
};
use crate::service::draft_runtime::{DraftRuntimeStatus, DRAFT_LOCK_TIMEOUT};
use crate::service::draft_service::{DraftService, RestoreVersionParams, SealVersionParams};
use std::sync::Arc;

// ============================================================================
// IPC #5: get_draft_workspace_state
// ============================================================================

/// Direct execution of IPC #5 `get_draft_workspace_state`.
///
/// Invariants (`be-core.md` §10.2.2):
/// - Read-only snapshot of working state without blocking callers holding `operation_lock`.
/// - Returns `DraftWorkspaceStateDto` with current lock telemetry.
/// - Errors: `ErrorCode::EntityNotFound`, `ErrorCode::DatabaseError`.
pub fn exec_get_draft_workspace_state(
    service: &DraftService,
    req: &GetDraftWorkspaceStateReq,
) -> Result<DraftWorkspaceStateDto, IpcError> {
    service.get_workspace_state(&req.draft_session_id)
}

#[tauri::command]
pub async fn get_draft_workspace_state(
    service: tauri::State<'_, Arc<DraftService>>,
    req: GetDraftWorkspaceStateReq,
) -> Result<DraftWorkspaceStateDto, IpcError> {
    exec_get_draft_workspace_state(&service, &req)
}

// ============================================================================
// IPC #6: get_progress_detail
// ============================================================================

/// Direct execution of IPC #6 `get_progress_detail`.
///
/// Invariants (`be-core.md` §10.2.2):
/// - Evaluates 46 Master Attributes across phases and renders complete Markdown summary.
/// - Errors: `ErrorCode::EntityNotFound`.
pub fn exec_get_progress_detail(
    service: &DraftService,
    req: &GetProgressDetailReq,
) -> Result<ProgressDetailDto, IpcError> {
    let entry = service.get_or_load_runtime_entry(req.draft_session_id)?;
    let snapshot = entry.get_snapshot();

    let eval = evaluate_progress(&snapshot.business_flow_data);

    let progress = ProgressSummaryDto {
        current_phase: snapshot.phase,
        current_subphase: snapshot.subphase,
        completed_attributes: eval.confirmed_count as u32,
        total_attributes: 46,
        progress_percentage: (eval.confirmed_count as f32 / 46.0) * 100.0,
        is_require_complete: eval.is_require_complete,
        can_advance_phase: if snapshot.phase == 0 {
            eval.is_require_complete
        } else {
            true
        },
    };

    let attributes = eval
        .attributes
        .iter()
        .map(|item| {
            let phase_num = match item.phase_group {
                MasterAttributePhaseGroup::Phase0Require => 0,
                MasterAttributePhaseGroup::Phase1Overview => 1,
                MasterAttributePhaseGroup::Phase2Chapter => 2,
                MasterAttributePhaseGroup::Phase3WorldBuilding => 3,
                MasterAttributePhaseGroup::Phase4SceneConfig => 4,
                MasterAttributePhaseGroup::Phase5Sub1Appearance
                | MasterAttributePhaseGroup::Phase5Sub2Psychological
                | MasterAttributePhaseGroup::Phase5Sub3LifeExperience => 5,
                MasterAttributePhaseGroup::Phase6Relationship => 6,
            };
            AttributeEvaluationDetailDto {
                code: item.path.clone(),
                display_name: item.path.clone(),
                phase: phase_num,
                is_required: item.phase_group == MasterAttributePhaseGroup::Phase0Require,
                status: match item.state {
                    crate::domain::progress_engine::AttributeProgressState::Confirmed
                    | crate::domain::progress_engine::AttributeProgressState::AiSuggested => {
                        "complete".to_string()
                    }
                    crate::domain::progress_engine::AttributeProgressState::Incomplete => {
                        "incomplete".to_string()
                    }
                },
                current_value_summary: None,
                source: None,
            }
        })
        .collect();

    let rendered_markdown = render_business_flow_markdown(
        &snapshot.business_flow_data,
        &req.system_language,
        snapshot.phase,
        snapshot.subphase,
    );

    let user_constraints = snapshot
        .business_flow_data
        .user_constraints
        .iter()
        .map(|c| c.content.clone())
        .collect();

    Ok(ProgressDetailDto {
        progress,
        attributes,
        rendered_markdown,
        user_constraints,
        conflicts: Vec::new(),
    })
}

#[tauri::command]
pub async fn get_progress_detail(
    service: tauri::State<'_, Arc<DraftService>>,
    req: GetProgressDetailReq,
) -> Result<ProgressDetailDto, IpcError> {
    exec_get_progress_detail(&service, &req)
}

// ============================================================================
// IPC #7: apply_manual_draft_edit
// ============================================================================

/// Direct asynchronous execution of IPC #7 `apply_manual_draft_edit`.
///
/// Invariants (`be-core.md` §10.2.2, §5.3):
/// - Only allows mutation tools (`add_*`, `set_*`, `delete_*`).
/// - Takes `operation_lock` (timeout 30s) -> `ErrorCode::DraftBusy` on timeout.
/// - Validates draft status is `'editing'` -> `ErrorCode::DraftNotEditable`.
/// - Executes `TX-01` with `reason = 'manual_edit'`, `seal_key = "manual:{clientEditId}"`.
/// - Idempotent by `clientEditId`.
pub async fn exec_apply_manual_draft_edit(
    service: &DraftService,
    req: &ApplyManualDraftEditReq,
) -> Result<ManualEditResponseDto, IpcError> {
    let is_mutation_tool = req.tool_code.starts_with("add_")
        || req.tool_code.starts_with("set_")
        || req.tool_code.starts_with("delete_");

    if !is_mutation_tool {
        return Err(IpcError::new(
            ErrorCode::ValidationFailed,
            format!(
                "Tool '{}' is not an allowed entity mutation tool. Only add_*, set_*, delete_* are permitted for manual edits.",
                req.tool_code
            ),
        ));
    }

    let entry = service.get_or_load_runtime_entry(req.draft_session_id)?;
    let _guard = entry
        .acquire_operation_lock("apply_manual_draft_edit", DRAFT_LOCK_TIMEOUT)
        .await?;

    let snap = entry.get_snapshot();
    if snap.status == DraftRuntimeStatus::Committing || snap.status == DraftRuntimeStatus::Committed {
        return Err(IpcError::new(
            ErrorCode::DraftNotEditable,
            "Draft is committing or committed and cannot be edited",
        ));
    }

    let (current_phase, current_subphase, base_seq) = (
        snap.phase,
        snap.subphase,
        snap.head_version_seq.unwrap_or(0),
    );

    // Apply mutation into working state
    entry.mutate_working_state(|ws| {
        ws.business_flow_data.decision_log.push(
            crate::domain::business_flow::DecisionLogEntry {
                id: req.client_edit_id.clone(),
                summary: format!("{}: {}", req.tool_code, req.args),
                source: crate::domain::business_flow::ProvenanceSource::User,
            },
        );

        if req.tool_code == "set_genre" {
            if let Some(genre_name) = req.args.get("name").and_then(|v| v.as_str()) {
                let item = crate::domain::business_flow::RequireGenreItem {
                    code: crate::domain::business_flow::TrackedField {
                        value: Some(genre_name.to_lowercase()),
                        source: Some(crate::domain::business_flow::ProvenanceSource::User),
                    },
                    is_primary: crate::domain::business_flow::TrackedField {
                        value: Some(true),
                        source: Some(crate::domain::business_flow::ProvenanceSource::User),
                    },
                    display_name: crate::domain::business_flow::TrackedField {
                        value: Some(genre_name.to_string()),
                        source: Some(crate::domain::business_flow::ProvenanceSource::User),
                    },
                };
                ws.business_flow_data.require.project.genres = vec![item];
            }
        }
    });

    let updated_snap = entry.get_snapshot();
    let data_json = serde_json::to_string(&updated_snap.business_flow_data).map_err(|e| {
        IpcError::new(ErrorCode::InternalError, format!("Serialization error: {}", e))
    })?;
    let data_hash = state_data_hash(&updated_snap.business_flow_data).map_err(|e| {
        IpcError::new(ErrorCode::InternalError, format!("Hash computation error: {}", e))
    })?;

    let seal_params = SealVersionParams {
        draft_session_id: req.draft_session_id,
        seal_key: format!("manual:{}", req.client_edit_id),
        reason: "manual_edit".to_string(),
        phase: current_phase,
        subphase: current_subphase,
        data_json,
        data_hash,
        parent_seq: Some(base_seq),
        turn_id: None,
        chat_message_id: None,
    };

    let sealed_dto = service.seal_version(&seal_params)?;
    let progress_eval = evaluate_progress(&updated_snap.business_flow_data);

    let progress = ProgressSummaryDto {
        current_phase,
        current_subphase,
        completed_attributes: progress_eval.confirmed_count as u32,
        total_attributes: 46,
        progress_percentage: (progress_eval.confirmed_count as f32 / 46.0) * 100.0,
        is_require_complete: progress_eval.is_require_complete,
        can_advance_phase: if current_phase == 0 {
            progress_eval.is_require_complete
        } else {
            true
        },
    };

    Ok(ManualEditResponseDto {
        tool_output: serde_json::json!({
            "status": "applied",
            "tool_code": req.tool_code,
            "client_edit_id": req.client_edit_id,
        }),
        sealed_version: Some(sealed_dto),
        progress,
    })
}

#[tauri::command]
pub async fn apply_manual_draft_edit(
    service: tauri::State<'_, Arc<DraftService>>,
    req: ApplyManualDraftEditReq,
) -> Result<ManualEditResponseDto, IpcError> {
    exec_apply_manual_draft_edit(&service, &req).await
}

// ============================================================================
// IPC #8: confirm_phase
// ============================================================================

/// Direct asynchronous execution of IPC #8 `confirm_phase`.
///
/// Invariants (`be-core.md` §10.2.2):
/// - Checks gating: `is_require_complete == true` required before entering Phase 1.
/// - Validates subphase: subphase in 1..=3 only when phase == 5, None otherwise.
/// - Takes `operation_lock`.
/// - Executes `TX-01` with `reason = 'phase_confirm'`, `seal_key = "phase_confirm:{base_seq}:{targetPhase}:{sub}"`.
pub async fn exec_confirm_phase(
    service: &DraftService,
    req: &ConfirmPhaseReq,
) -> Result<PhaseConfirmResponseDto, IpcError> {
    let entry = service.get_or_load_runtime_entry(req.draft_session_id)?;
    let _guard = entry
        .acquire_operation_lock("confirm_phase", DRAFT_LOCK_TIMEOUT)
        .await?;

    let snap = entry.get_snapshot();
    if snap.status == DraftRuntimeStatus::Committing || snap.status == DraftRuntimeStatus::Committed {
        return Err(IpcError::new(
            ErrorCode::DraftNotEditable,
            "Draft is committing or committed and cannot confirm phase",
        ));
    }

    if req.target_phase < 0 || req.target_phase > 6 {
        return Err(IpcError::new(
            ErrorCode::ValidationFailed,
            format!("Invalid target phase: {}", req.target_phase),
        ));
    }

    if req.target_phase == 5 {
        if let Some(sub) = req.target_subphase {
            if sub < 1 || sub > 3 {
                return Err(IpcError::new(
                    ErrorCode::ValidationFailed,
                    "Phase 5 targetSubphase must be in 1..=3",
                ));
            }
        } else {
            return Err(IpcError::new(
                ErrorCode::ValidationFailed,
                "Phase 5 requires a targetSubphase in 1..=3",
            ));
        }
    } else if req.target_subphase.is_some() {
        return Err(IpcError::new(
            ErrorCode::ValidationFailed,
            "Target subphase is only allowed when target phase is 5",
        ));
    }

    let progress_eval = evaluate_progress(&snap.business_flow_data);
    if req.target_phase == 1 && !progress_eval.is_require_complete {
        return Err(IpcError::new(
            ErrorCode::ValidationFailed,
            "Cannot confirm phase 1: Phase 0 requirement is not complete (is_require_complete is false)",
        ));
    }

    let base_seq = snap.head_version_seq.unwrap_or(0);
    let sub_suffix = req.target_subphase.unwrap_or(0);
    let seal_key = format!("phase_confirm:{}:{}:{}", base_seq, req.target_phase, sub_suffix);

    entry.mutate_working_state(|ws| {
        ws.phase = req.target_phase;
        ws.subphase = req.target_subphase;
        ws.business_flow_data.meta.phase = req.target_phase;
        ws.business_flow_data.meta.subphase = req.target_subphase;
    });

    let updated_snap = entry.get_snapshot();
    let data_json = serde_json::to_string(&updated_snap.business_flow_data).map_err(|e| {
        IpcError::new(ErrorCode::InternalError, format!("Serialization error: {}", e))
    })?;
    let data_hash = state_data_hash(&updated_snap.business_flow_data).map_err(|e| {
        IpcError::new(ErrorCode::InternalError, format!("Hash computation error: {}", e))
    })?;

    let seal_params = SealVersionParams {
        draft_session_id: req.draft_session_id,
        seal_key,
        reason: "phase_confirm".to_string(),
        phase: req.target_phase,
        subphase: req.target_subphase,
        data_json,
        data_hash,
        parent_seq: Some(base_seq),
        turn_id: None,
        chat_message_id: None,
    };

    let sealed_dto = service.seal_version(&seal_params)?;

    Ok(PhaseConfirmResponseDto {
        phase: req.target_phase,
        subphase: req.target_subphase,
        sealed_version: sealed_dto,
    })
}

#[tauri::command]
pub async fn confirm_phase(
    service: tauri::State<'_, Arc<DraftService>>,
    req: ConfirmPhaseReq,
) -> Result<PhaseConfirmResponseDto, IpcError> {
    exec_confirm_phase(&service, &req).await
}

// ============================================================================
// IPC #10: list_draft_versions
// ============================================================================

/// Direct execution of IPC #10 `list_draft_versions`.
///
/// Invariants (`be-core.md` §10.2.3):
/// - Read-only query on `draft_versions` ordered by `seq DESC`.
/// - Errors: `ErrorCode::EntityNotFound`, `ErrorCode::DatabaseError`.
pub fn exec_list_draft_versions(
    service: &DraftService,
    req: &ListDraftVersionsReq,
) -> Result<DraftVersionListDto, IpcError> {
    service.list_versions(&req.draft_session_id)
}

#[tauri::command]
pub async fn list_draft_versions(
    service: tauri::State<'_, Arc<DraftService>>,
    req: ListDraftVersionsReq,
) -> Result<DraftVersionListDto, IpcError> {
    exec_list_draft_versions(&service, &req)
}

// ============================================================================
// IPC #11: restore_draft_version
// ============================================================================

/// Direct asynchronous execution of IPC #11 `restore_draft_version`.
///
/// Invariants (`be-core.md` §10.2.3, §4.5, `CI-03`):
/// - Requires `draft_sessions.status == 'editing'`.
/// - Takes `operation_lock` (timeout 30s).
/// - Executes `TX-03`: creates new version `seq = head + 1, reason = 'restore'`,
///   restores state, expires pending patches.
/// - Idempotent by `clientRequestId`.
pub async fn exec_restore_draft_version(
    service: &DraftService,
    req: &RestoreDraftVersionReq,
) -> Result<RestoreVersionResponseDto, IpcError> {
    let params = RestoreVersionParams {
        draft_session_id: req.draft_session_id,
        target_seq: req.target_seq,
        client_request_id: req.client_request_id.clone(),
    };
    service.restore_version(&params).await
}

#[tauri::command]
pub async fn restore_draft_version(
    service: tauri::State<'_, Arc<DraftService>>,
    req: RestoreDraftVersionReq,
) -> Result<RestoreVersionResponseDto, IpcError> {
    exec_restore_draft_version(&service, &req).await
}

// ============================================================================
// Helper formatting
// ============================================================================

fn render_business_flow_markdown(
    data: &crate::domain::business_flow::BusinessFlowData,
    system_lang: &str,
    phase: i32,
    subphase: Option<i32>,
) -> String {
    let mut md = String::new();
    let is_vi = system_lang == "vi";

    if is_vi {
        md.push_str("# Tổng Quan Bản Nháp Kịch Bản\n\n");
        md.push_str(&format!(
            "**Dự án:** {}\n\n",
            data.require
                .project
                .display_name
                .value
                .as_deref()
                .unwrap_or("Chưa đặt tên")
        ));
        md.push_str(&format!("**Giai đoạn hiện tại:** Phase {} (Subphase: {:?})\n\n", phase, subphase));

        md.push_str("## 1. Thông Số Mục Tiêu (Phase 0)\n");
        md.push_str(&format!(
            "- **Số Arc mục tiêu:** {}\n",
            data.require.project.target.arc_count.value.unwrap_or(0)
        ));
        md.push_str(&format!(
            "- **Số Chương mục tiêu:** {}\n",
            data.require.project.target.chapter_count.value.unwrap_or(0)
        ));
        md.push_str(&format!(
            "- **Số từ mỗi chương:** {}\n\n",
            data.require.project.target.word_per_chapter_count.value.unwrap_or(0)
        ));

        md.push_str("## 2. Nhân Vật\n");
        if data.require.character.is_empty() {
            md.push_str("_Chưa có nhân vật nào._\n\n");
        } else {
            for c in &data.require.character {
                let name = c
                    .display_name
                    .value
                    .as_deref()
                    .or(c.nickname.value.as_deref())
                    .unwrap_or("Không tên");
                md.push_str(&format!("- **{}** (Mã: `{}`)\n", name, c.code));
            }
            md.push('\n');
        }

        md.push_str("## 3. Danh Sách Arc & Chương\n");
        if data.optional.overview_phase.arcs.is_empty() {
            md.push_str("_Chưa có Arc nào._\n\n");
        } else {
            for arc in &data.optional.overview_phase.arcs {
                let arc_title = arc.display_name.value.as_deref().unwrap_or("Arc");
                md.push_str(&format!("- **{}** (`{}`)\n", arc_title, arc.code));
            }
            md.push('\n');
        }
    } else {
        md.push_str("# Draft Workspace Summary\n\n");
        md.push_str(&format!(
            "**Project:** {}\n\n",
            data.require
                .project
                .display_name
                .value
                .as_deref()
                .unwrap_or("Untitled")
        ));
        md.push_str(&format!("**Current Phase:** Phase {} (Subphase: {:?})\n\n", phase, subphase));

        md.push_str("## 1. Phase 0 Targets\n");
        md.push_str(&format!(
            "- **Target Arc Count:** {}\n",
            data.require.project.target.arc_count.value.unwrap_or(0)
        ));
        md.push_str(&format!(
            "- **Target Chapter Count:** {}\n",
            data.require.project.target.chapter_count.value.unwrap_or(0)
        ));
        md.push_str(&format!(
            "- **Target Words/Chapter:** {}\n\n",
            data.require.project.target.word_per_chapter_count.value.unwrap_or(0)
        ));

        md.push_str("## 2. Characters\n");
        if data.require.character.is_empty() {
            md.push_str("_No characters defined._\n\n");
        } else {
            for c in &data.require.character {
                let name = c
                    .display_name
                    .value
                    .as_deref()
                    .or(c.nickname.value.as_deref())
                    .unwrap_or("Unnamed");
                md.push_str(&format!("- **{}** (Code: `{}`)\n", name, c.code));
            }
            md.push('\n');
        }

        md.push_str("## 3. Arcs & Chapters\n");
        if data.optional.overview_phase.arcs.is_empty() {
            md.push_str("_No arcs defined._\n\n");
        } else {
            for arc in &data.optional.overview_phase.arcs {
                let arc_title = arc.display_name.value.as_deref().unwrap_or("Arc");
                md.push_str(&format!("- **{}** (`{}`)\n", arc_title, arc.code));
            }
            md.push('\n');
        }
    }

    md
}
