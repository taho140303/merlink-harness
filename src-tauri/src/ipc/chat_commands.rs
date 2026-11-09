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

    let project_exists_in_db = |pid: Uuid| -> bool {
        conn.query_row(
            "SELECT EXISTS(SELECT 1 FROM projects WHERE id = ?1 AND deleted_at IS NULL)",
            params![pid.to_string()],
            |r| r.get::<_, bool>(0),
        )
        .unwrap_or(false)
    };

    let find_project_draft_session = |pid: Uuid| -> Option<(Uuid, Option<i32>, String)> {
        conn.query_row(
            "SELECT id, head_version_seq, status FROM draft_sessions WHERE project_id = ?1 ORDER BY updated_at DESC, created_at DESC LIMIT 1",
            params![pid.to_string()],
            |r| {
                let id_str: String = r.get(0)?;
                let d_uuid = Uuid::parse_str(&id_str).unwrap_or_else(|_| Uuid::new_v4());
                let h_seq: Option<i32> = r.get(1).unwrap_or(None);
                let st: String = r.get(2).unwrap_or_else(|_| "editing".to_string());
                Ok((d_uuid, h_seq, st))
            },
        )
        .optional()
        .ok()
        .flatten()
    };

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
            let valid_proj_uuid = proj_uuid.filter(|&pid| project_exists_in_db(pid));

            // If session was given a valid project_id from req that wasn't stored on sessions yet, persist it
            if let Some(vpid) = valid_proj_uuid {
                let _ = conn.execute(
                    "UPDATE sessions SET project_id = ?1 WHERE id = ?2 AND project_id IS NULL",
                    params![vpid.to_string(), s_id],
                );
            }

            // If this chat session points to an existing created project in DB, share that project's draft session
            let shared_project_draft = valid_proj_uuid.and_then(find_project_draft_session);

            let (d_id, head_seq, stat) = if let Some(shared) = shared_project_draft {
                shared
            } else {
                // Find associated draft session strictly for this chat session
                let mut d_stmt = conn
                    .prepare("SELECT id, head_version_seq, status FROM draft_sessions WHERE session_id = ?1")
                    .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
                let mut d_rows = d_stmt
                    .query(params![s_id])
                    .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

                match d_rows
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
                }
            };

            (s_id, proj_uuid, d_id, head_seq, stat)
        }
        None => {
            let s_id = Uuid::new_v4().to_string();
            let valid_proj_uuid = req.project_id.filter(|&pid| project_exists_in_db(pid));

            conn.execute(
                "INSERT INTO sessions (id, project_id, display_name, created_at)
                 VALUES (?1, ?2, 'New Session', ?3)",
                params![s_id, req.project_id.map(|u| u.to_string()), now],
            )
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

            // If pointing to a created project in DB that already has a draft_session, share it;
            // otherwise create an isolated draft_session for this new chat session.
            let (d_uuid, head_seq, stat) = if let Some(shared) = valid_proj_uuid.and_then(find_project_draft_session) {
                shared
            } else {
                let d_uuid = Uuid::new_v4();
                conn.execute(
                    "INSERT INTO draft_sessions (id, session_id, project_id, head_version_seq, status, created_at, updated_at)
                     VALUES (?1, ?2, ?3, NULL, 'editing', ?4, ?4)",
                    params![d_uuid.to_string(), s_id, req.project_id.map(|u| u.to_string()), now],
                )
                .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
                (d_uuid, None, "editing".to_string())
            };

            (s_id, req.project_id, d_uuid, head_seq, stat)
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

    // Check for interrupted turn alert (be-core.md §11.2.2)
    let mut interrupted_turn_alert = None;
    if ws_snap.status == DraftRuntimeStatus::SealPendingRetry {
        interrupted_turn_alert = Some("Draft version sealing is pending retry from previous turn.".to_string());
    } else {
        // Only raise interruptedTurnAlert if an unsealed turn had applied tool writes (§11.2.2)
        let unsealed_stmt = conn.prepare(
            "SELECT 1 FROM messages m
             WHERE m.session_id = ?1
               AND m.turn_id IS NOT NULL
               AND m.role = 'tool'
               AND (
                   (json_extract(m.payload_json, '$.writes_draft_state') = 1 AND json_extract(m.payload_json, '$.has_applied_write') = 1)
                   OR json_extract(m.payload_json, '$.status') = 'applied'
               )
               AND NOT EXISTS (
                   SELECT 1 FROM draft_versions dv
                   WHERE dv.draft_session_id = ?2
                     AND dv.turn_id = m.turn_id
               )
             LIMIT 1",
        );
        if let Ok(mut stmt) = unsealed_stmt {
            if let Ok(exists) = stmt.exists(params![session_id, draft_session_id.to_string()]) {
                if exists {
                    interrupted_turn_alert = Some("Previous conversation turn was interrupted before response completed.".to_string());
                }
            }
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
    let filter_has_messages = req.has_messages.unwrap_or(false);
    let msg_filter = if filter_has_messages {
        " AND EXISTS (SELECT 1 FROM messages m WHERE m.session_id = s.id)"
    } else {
        ""
    };

    let draft_join = "LEFT JOIN draft_sessions d ON d.id = COALESCE(
        (SELECT ds.id FROM draft_sessions ds WHERE ds.session_id = s.id LIMIT 1),
        (SELECT dp.id FROM draft_sessions dp JOIN projects p ON p.id = s.project_id AND p.deleted_at IS NULL WHERE dp.project_id = s.project_id ORDER BY dp.updated_at DESC, dp.created_at DESC LIMIT 1)
    )";

    let (query, query_params): (String, Vec<rusqlite::types::Value>) = match (&req.project_id, &req.cursor) {
        (Some(pid), Some(cur)) => (
            format!(
                "SELECT s.id, d.id, s.project_id, COALESCE(d.status, 'editing'), COALESCE(d.updated_at, s.created_at), s.display_name, (SELECT COUNT(*) FROM messages m WHERE m.session_id = s.id)
                 FROM sessions s
                 {}
                 WHERE s.project_id = ?1 AND s.created_at < ?2{}
                 ORDER BY s.created_at DESC
                 LIMIT ?3",
                draft_join, msg_filter
            ),
            vec![
                pid.to_string().into(),
                cur.clone().into(),
                ((limit + 1) as i64).into(),
            ],
        ),
        (Some(pid), None) => (
            format!(
                "SELECT s.id, d.id, s.project_id, COALESCE(d.status, 'editing'), COALESCE(d.updated_at, s.created_at), s.display_name, (SELECT COUNT(*) FROM messages m WHERE m.session_id = s.id)
                 FROM sessions s
                 {}
                 WHERE s.project_id = ?1{}
                 ORDER BY s.created_at DESC
                 LIMIT ?2",
                draft_join, msg_filter
            ),
            vec![pid.to_string().into(), ((limit + 1) as i64).into()],
        ),
        (None, Some(cur)) => (
            format!(
                "SELECT s.id, d.id, s.project_id, COALESCE(d.status, 'editing'), COALESCE(d.updated_at, s.created_at), s.display_name, (SELECT COUNT(*) FROM messages m WHERE m.session_id = s.id)
                 FROM sessions s
                 {}
                 WHERE s.created_at < ?1{}
                 ORDER BY s.created_at DESC
                 LIMIT ?2",
                draft_join, msg_filter
            ),
            vec![cur.clone().into(), ((limit + 1) as i64).into()],
        ),
        (None, None) => (
            format!(
                "SELECT s.id, d.id, s.project_id, COALESCE(d.status, 'editing'), COALESCE(d.updated_at, s.created_at), s.display_name, (SELECT COUNT(*) FROM messages m WHERE m.session_id = s.id)
                 FROM sessions s
                 {}
                 WHERE 1=1{}
                 ORDER BY s.created_at DESC
                 LIMIT ?1",
                draft_join, msg_filter
            ),
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
        let display_name: Option<String> = row.get(5).unwrap_or(None);
        let message_count: Option<u32> = row.get(6).ok();

        items.push(SessionSummaryDto {
            session_id: sid,
            draft_session_id,
            project_id,
            status,
            updated_at,
            display_name,
            message_count,
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

    // Look up draft_session_id (either owned by this session or shared via a created project in DB)
    let draft_session_id_str: String = conn
        .query_row(
            "SELECT COALESCE(
                (SELECT ds.id FROM draft_sessions ds WHERE ds.session_id = ?1 LIMIT 1),
                (SELECT dp.id FROM sessions s JOIN projects p ON p.id = s.project_id AND p.deleted_at IS NULL JOIN draft_sessions dp ON dp.project_id = p.id WHERE s.id = ?1 ORDER BY dp.updated_at DESC, dp.created_at DESC LIMIT 1)
            )",
            params![req.session_id],
            |row| row.get::<_, Option<String>>(0),
        )
        .ok()
        .flatten()
        .ok_or_else(|| {
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
    let res = orchestrator
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
        .await;

    if res.is_err() {
        // If turn failed fatally before sealing any version or mutating state,
        // clean up the orphaned user message so the session does not have half-baked failed turns
        if let Ok(conn) = orchestrator.pool().get() {
            let _ = conn.execute(
                "DELETE FROM messages WHERE id = ?1",
                params![user_message_id.to_string()],
            );
        }
    }

    res
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
