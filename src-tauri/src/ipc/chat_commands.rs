//! Chat & Session IPC Commands (#1, #2, #3, #4, #9)
//! (`REQ-CMD-001`..`004`, `REQ-CMD-009`, `be-core.md` §10.2.1, `core.md` §3.8).

use std::sync::Arc;
use chrono::Utc;
use rusqlite::{params, OptionalExtension};
use serde_json::Value;
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

use crate::domain::business_flow::BusinessFlowData;
use crate::error::{ErrorCode, IpcError};
use crate::ipc::dto::{
    CancelChatTurnReq, CancelChatTurnResponseDto, ChatSessionInitDto,
    ChatSessionListDto, ChatStreamEvent, CreateOrLoadChatSessionReq, DraftVersionSummaryDto,
    ListChatSessionsReq, RegenerateDraftPhaseReq, RegeneratePhaseResponseDto,
    SendChatMessageReq, SendChatTurnResponseDto, SessionSummaryDto,
};
use crate::providers::traits::LlmProvider;
use crate::service::chat_orchestrator::ChatOrchestrator;
use crate::service::draft_runtime::{DraftRuntimeStatus, WorkingState};
use crate::service::patch_service::compute_progress_dto;

// ============================================================================
// IPC #1: create_or_load_chat_session
// ============================================================================

/// Direct execution of IPC #1 `create_or_load_chat_session`.
pub fn exec_create_or_load_chat_session(
    orchestrator: &ChatOrchestrator,
    req: &CreateOrLoadChatSessionReq,
) -> Result<ChatSessionInitDto, IpcError> {
    let conn = orchestrator
        .pool()
        .get()
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

    let now = Utc::now().to_rfc3339();

    let (session_id, project_id, draft_session_id, head_version_seq, status) = match &req.session_id {
        Some(sid) => {
            // Find existing session
            let mut stmt = conn
                .prepare("SELECT id, project_id FROM sessions WHERE id = ?1")
                .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
            let mut rows = stmt
                .query(params![sid])
                .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

            let (s_id, p_id): (String, Option<String>) = match rows
                .next()
                .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?
            {
                Some(row) => (row.get(0).unwrap(), row.get(1).unwrap_or(None)),
                None => {
                    return Err(IpcError::new(
                        ErrorCode::EntityNotFound,
                        format!("Session '{}' not found", sid),
                    ));
                }
            };

            let proj_uuid = p_id.and_then(|s| Uuid::parse_str(&s).ok()).or(req.project_id);

            // Find associated draft session
            let mut d_stmt = conn
                .prepare("SELECT id, head_version_seq, status FROM draft_sessions WHERE session_id = ?1")
                .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
            let mut d_rows = d_stmt
                .query(params![s_id])
                .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

            let (d_id, head_seq, stat) = match d_rows
                .next()
                .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?
            {
                Some(r) => {
                    let id_str: String = r.get(0).unwrap();
                    let d_uuid = Uuid::parse_str(&id_str).unwrap_or_else(|_| Uuid::new_v4());
                    let h_seq: Option<i32> = r.get(1).unwrap_or(None);
                    let st: String = r.get(2).unwrap_or_else(|_| "editing".to_string());
                    (d_uuid, h_seq, st)
                }
                None => {
                    let d_uuid = Uuid::new_v4();
                    conn.execute(
                        "INSERT INTO draft_sessions (id, session_id, project_id, head_version_seq, status, created_at, updated_at)
                         VALUES (?1, ?2, ?3, NULL, 'editing', ?4, ?4)",
                        params![d_uuid.to_string(), s_id, proj_uuid.map(|u| u.to_string()), now],
                    )
                    .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
                    (d_uuid, None, "editing".to_string())
                }
            };

            (s_id, proj_uuid, d_id, head_seq, stat)
        }
        None => {
            let s_id = Uuid::new_v4().to_string();
            let d_uuid = Uuid::new_v4();

            conn.execute(
                "INSERT INTO sessions (id, project_id, display_name, created_at)
                 VALUES (?1, ?2, 'New Session', ?3)",
                params![s_id, req.project_id.map(|u| u.to_string()), now],
            )
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

            conn.execute(
                "INSERT INTO draft_sessions (id, session_id, project_id, head_version_seq, status, created_at, updated_at)
                 VALUES (?1, ?2, ?3, NULL, 'editing', ?4, ?4)",
                params![d_uuid.to_string(), s_id, req.project_id.map(|u| u.to_string()), now],
            )
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

            (s_id, req.project_id, d_uuid, None, "editing".to_string())
        }
    };

    // Ensure WorkingState is registered in memory
    let entry = match orchestrator.registry().get(&draft_session_id) {
        Some(e) => e,
        None => {
            let mut ws = WorkingState::new(draft_session_id);
            ws.status = DraftRuntimeStatus::from_str(&status).unwrap_or(DraftRuntimeStatus::Editing);
            ws.head_version_seq = head_version_seq;

            if let Some(seq) = head_version_seq {
                let mut v_stmt = conn
                    .prepare("SELECT data_json, phase, subphase FROM draft_versions WHERE draft_session_id = ?1 AND seq = ?2")
                    .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
                let mut v_rows = v_stmt
                    .query(params![draft_session_id.to_string(), seq])
                    .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
                if let Some(v_row) = v_rows
                    .next()
                    .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?
                {
                    let data_json: String = v_row.get(0).unwrap_or_default();
                    if let Ok(bfd) = BusinessFlowData::from_json_str_strict(&data_json) {
                        ws.business_flow_data = bfd;
                    }
                    ws.phase = v_row.get(1).unwrap_or(0);
                    ws.subphase = v_row.get(2).unwrap_or(None);
                }
            }

            orchestrator.registry().get_or_register(draft_session_id, ws)
        }
    };

    let ws_snap = entry.get_snapshot();

    // Fetch conversation message history
    let messages = orchestrator.fetch_recent_session_messages(&session_id, Uuid::nil(), 100)?;

    // Check for interrupted turn alert
    let mut interrupted_turn_alert = None;
    if ws_snap.status == DraftRuntimeStatus::SealPendingRetry {
        interrupted_turn_alert = Some("Draft version sealing is pending retry from previous turn.".to_string());
    } else if let Some(last_msg) = messages.last() {
        if last_msg.role == "user" {
            interrupted_turn_alert = Some("Previous conversation turn was interrupted before response completed.".to_string());
        }
    }

    let progress = compute_progress_dto(&ws_snap);
    let working_state_json = serde_json::to_value(&ws_snap.business_flow_data)
        .map_err(|e| IpcError::new(ErrorCode::ValidationFailed, e.to_string()))?;

    Ok(ChatSessionInitDto {
        session_id,
        draft_session_id,
        project_id,
        status,
        head_version_seq,
        messages,
        working_state: working_state_json,
        progress,
        interrupted_turn_alert,
    })
}

#[tauri::command]
pub async fn create_or_load_chat_session(
    orchestrator: tauri::State<'_, Arc<ChatOrchestrator>>,
    req: CreateOrLoadChatSessionReq,
) -> Result<ChatSessionInitDto, IpcError> {
    exec_create_or_load_chat_session(&orchestrator, &req)
}

// ============================================================================
// IPC #2: list_chat_sessions
// ============================================================================

/// Direct execution of IPC #2 `list_chat_sessions`.
pub fn exec_list_chat_sessions(
    orchestrator: &ChatOrchestrator,
    req: &ListChatSessionsReq,
) -> Result<ChatSessionListDto, IpcError> {
    let conn = orchestrator
        .pool()
        .get()
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

    let limit = req.limit.unwrap_or(20).clamp(1, 100) as usize;

    let (query, query_params): (String, Vec<rusqlite::types::Value>) = match (&req.project_id, &req.cursor) {
        (Some(pid), Some(cur)) => (
            "SELECT s.id, d.id, s.project_id, COALESCE(d.status, 'editing'), COALESCE(d.updated_at, s.created_at)
             FROM sessions s
             LEFT JOIN draft_sessions d ON s.id = d.session_id
             WHERE s.project_id = ?1 AND s.created_at < ?2
             ORDER BY s.created_at DESC
             LIMIT ?3"
                .to_string(),
            vec![
                pid.to_string().into(),
                cur.clone().into(),
                ((limit + 1) as i64).into(),
            ],
        ),
        (Some(pid), None) => (
            "SELECT s.id, d.id, s.project_id, COALESCE(d.status, 'editing'), COALESCE(d.updated_at, s.created_at)
             FROM sessions s
             LEFT JOIN draft_sessions d ON s.id = d.session_id
             WHERE s.project_id = ?1
             ORDER BY s.created_at DESC
             LIMIT ?2"
                .to_string(),
            vec![pid.to_string().into(), ((limit + 1) as i64).into()],
        ),
        (None, Some(cur)) => (
            "SELECT s.id, d.id, s.project_id, COALESCE(d.status, 'editing'), COALESCE(d.updated_at, s.created_at)
             FROM sessions s
             LEFT JOIN draft_sessions d ON s.id = d.session_id
             WHERE s.created_at < ?1
             ORDER BY s.created_at DESC
             LIMIT ?2"
                .to_string(),
            vec![cur.clone().into(), ((limit + 1) as i64).into()],
        ),
        (None, None) => (
            "SELECT s.id, d.id, s.project_id, COALESCE(d.status, 'editing'), COALESCE(d.updated_at, s.created_at)
             FROM sessions s
             LEFT JOIN draft_sessions d ON s.id = d.session_id
             ORDER BY s.created_at DESC
             LIMIT ?1"
                .to_string(),
            vec![((limit + 1) as i64).into()],
        ),
    };

    let mut stmt = conn
        .prepare(&query)
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

    let mut rows = stmt
        .query(rusqlite::params_from_iter(query_params))
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

    let mut items = Vec::new();
    while let Some(row) = rows
        .next()
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?
    {
        let sid: String = row.get(0).unwrap_or_default();
        let did_str: Option<String> = row.get(1).unwrap_or(None);
        let draft_session_id = parse_listed_draft_id(&sid, did_str)?;
        let pid_str: Option<String> = row.get(2).unwrap_or(None);
        let project_id = pid_str.and_then(|s| Uuid::parse_str(&s).ok());
        let status: String = row.get(3).unwrap_or_else(|_| "editing".to_string());
        let updated_at: String = row.get(4).unwrap_or_default();

        items.push(SessionSummaryDto {
            session_id: sid,
            draft_session_id,
            project_id,
            status,
            updated_at,
        });
    }

    let mut next_cursor = None;
    if items.len() > limit {
        items.truncate(limit);
        next_cursor = items.last().map(|s| s.updated_at.clone());
    }

    Ok(ChatSessionListDto { items, next_cursor })
}

fn parse_listed_draft_id(session_id: &str, draft_id: Option<String>) -> Result<Uuid, IpcError> {
    let draft_id = draft_id.ok_or_else(|| IpcError::new(
        ErrorCode::DatabaseError,
        format!("Session '{}' has no linked draft session", session_id),
    ))?;
    Uuid::parse_str(&draft_id).map_err(|_| IpcError::new(
        ErrorCode::DatabaseError,
        format!("Session '{}' has an invalid linked draft session ID", session_id),
    ))
}

#[cfg(test)]
mod list_sessions_tests {
    use super::*;

    #[test]
    fn test_listed_draft_id_missing_link_fails_closed() {
        assert!(parse_listed_draft_id("session", None).is_err());
        assert!(parse_listed_draft_id("session", Some("corrupt".into())).is_err());
        let id = Uuid::new_v4();
        assert_eq!(parse_listed_draft_id("session", Some(id.to_string())).unwrap(), id);
    }
}

#[tauri::command]
pub async fn list_chat_sessions(
    orchestrator: tauri::State<'_, Arc<ChatOrchestrator>>,
    req: ListChatSessionsReq,
) -> Result<ChatSessionListDto, IpcError> {
    exec_list_chat_sessions(&orchestrator, &req)
}

// ============================================================================
// IPC #3: send_chat_message
// ============================================================================

/// Direct execution of IPC #3 `send_chat_message`.
pub async fn exec_send_chat_message(
    orchestrator: &ChatOrchestrator,
    provider: Arc<dyn LlmProvider>,
    req: &SendChatMessageReq,
    stream_tx: Option<tokio::sync::mpsc::Sender<ChatStreamEvent>>,
) -> Result<SendChatTurnResponseDto, IpcError> {
    let conn = orchestrator
        .pool()
        .get()
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

    // Look up draft_session_id
    let mut stmt = conn
        .prepare("SELECT id FROM draft_sessions WHERE session_id = ?1")
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

    let draft_session_id_str: String = stmt
        .query_row(params![req.session_id], |row| row.get(0))
        .map_err(|_| {
            IpcError::new(
                ErrorCode::EntityNotFound,
                format!("Draft session for session_id '{}' not found", req.session_id),
            )
        })?;

    let draft_session_id = Uuid::parse_str(&draft_session_id_str)
        .map_err(|e| IpcError::new(ErrorCode::InternalError, e.to_string()))?;

    let turn_id = Uuid::new_v4();
    let user_message_id = Uuid::new_v4();

    // Save user message to SQLite
    let payload = req
        .feedback_context
        .as_ref()
        .map(|f| serde_json::to_value(f).unwrap_or(Value::Null));

    orchestrator.save_message(
        user_message_id,
        &req.session_id,
        turn_id,
        "user",
        &req.content,
        payload.as_ref(),
    )?;

    // Run AI ReAct turn
    orchestrator
        .run_ai_turn(
            draft_session_id,
            &req.session_id,
            user_message_id,
            turn_id,
            &req.content,
            &req.system_language,
            provider,
            stream_tx,
            CancellationToken::new(),
        )
        .await
}

// ============================================================================
// IPC #4: cancel_chat_turn
// ============================================================================

/// Direct execution of IPC #4 `cancel_chat_turn`.
pub async fn exec_cancel_chat_turn(
    orchestrator: &ChatOrchestrator,
    req: &CancelChatTurnReq,
) -> Result<CancelChatTurnResponseDto, IpcError> {
    let conn = orchestrator
        .pool()
        .get()
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

    let mut stmt = conn
        .prepare("SELECT id FROM draft_sessions WHERE session_id = ?1")
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

    let draft_session_id_str: Option<String> = stmt
        .query_row(params![req.session_id], |r| r.get(0))
        .optional()
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

    let draft_session_id = match draft_session_id_str {
        Some(s) => Uuid::parse_str(&s).unwrap_or_else(|_| Uuid::nil()),
        None => {
            return Ok(CancelChatTurnResponseDto {
                cancelled: true,
                sealed_version: None,
            });
        }
    };

    if let Some(entry) = orchestrator.registry().get(&draft_session_id) {
        if let Some(token) = entry.active_cancel_token.lock().unwrap().take() {
            token.cancel();
        }
    }

    // Check if turn was sealed
    let mut v_stmt = conn
        .prepare(
            "SELECT seq, phase, subphase, reason, parent_seq, created_at, data_hash
             FROM draft_versions
             WHERE draft_session_id = ?1 AND turn_id = ?2",
        )
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

    let sealed_version = v_stmt
        .query_row(
            params![draft_session_id.to_string(), req.turn_id.to_string()],
            |row| {
                let seq: i32 = row.get(0)?;
                Ok(DraftVersionSummaryDto {
                    seq,
                    display_label: format!("v.{:02}", seq),
                    phase: row.get(1)?,
                    subphase: row.get(2)?,
                    reason: row.get(3)?,
                    parent_seq: row.get(4)?,
                    created_at: row.get(5)?,
                    data_hash: row.get(6)?,
                })
            },
        )
        .optional()
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

    Ok(CancelChatTurnResponseDto {
        cancelled: true,
        sealed_version,
    })
}

#[tauri::command]
pub async fn cancel_chat_turn(
    orchestrator: tauri::State<'_, Arc<ChatOrchestrator>>,
    req: CancelChatTurnReq,
) -> Result<CancelChatTurnResponseDto, IpcError> {
    exec_cancel_chat_turn(&orchestrator, &req).await
}

// ============================================================================
// IPC #9: regenerate_draft_phase
// ============================================================================

/// Direct execution of IPC #9 `regenerate_draft_phase`.
pub async fn exec_regenerate_draft_phase(
    orchestrator: &ChatOrchestrator,
    req: &RegenerateDraftPhaseReq,
    candidate_override: Option<BusinessFlowData>,
) -> Result<RegeneratePhaseResponseDto, IpcError> {
    let turn_id = Uuid::new_v4();

    let sealed = orchestrator
        .regenerate_draft_phase(
            req.draft_session_id,
            turn_id,
            req.phase,
            req.subphase,
            candidate_override,
        )
        .await?;

    let entry = orchestrator
        .registry()
        .get(&req.draft_session_id)
        .ok_or_else(|| IpcError::new(ErrorCode::EntityNotFound, "Draft session not in runtime"))?;

    let ws = entry.get_snapshot();
    let progress = compute_progress_dto(&ws);

    Ok(RegeneratePhaseResponseDto {
        turn_id,
        sealed_version: Some(sealed),
        pending_patches: ws.pending_patches.clone(),
        progress,
    })
}

#[tauri::command]
pub async fn regenerate_draft_phase(
    orchestrator: tauri::State<'_, Arc<ChatOrchestrator>>,
    req: RegenerateDraftPhaseReq,
) -> Result<RegeneratePhaseResponseDto, IpcError> {
    exec_regenerate_draft_phase(&orchestrator, &req, None).await
}
