//! Character Service orchestrating character queries, detail projections,
//! library upserts, and draft linkage (`REQ-PROJECT-001`, IPC #25, #26, #27, #28, `be-core.md` §10.2.7).

use rusqlite::params;
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::business_flow::{ProvenanceSource, RequireCharacterItem, TrackedField};
use crate::domain::fingerprint::state_data_hash;
use crate::error::{ErrorCode, IpcError};
use crate::ipc::dto::{
    CharacterDetailDto, CharacterListDto, CreateOrUpdateLibraryCharacterReq,
    DraftVersionSummaryDto, GetCharacterDetailReq, LinkLibraryCharacterResponseDto,
    LinkLibraryCharacterToDraftReq, ListCharactersReq,
};
use crate::service::draft_runtime::{DraftRuntimeStatus, DRAFT_LOCK_TIMEOUT};
use crate::service::draft_service::{DraftService, SealVersionParams};
use crate::storage::character_repo;
use crate::storage::db::DbPool;

/// Service coordinating character search, detail retrieval, library mutations, and draft links.
pub struct CharacterService {
    pool: DbPool,
    draft_service: Arc<DraftService>,
}

impl CharacterService {
    pub fn new(pool: DbPool, draft_service: Arc<DraftService>) -> Self {
        Self {
            pool,
            draft_service,
        }
    }

    pub fn pool(&self) -> &DbPool {
        &self.pool
    }

    pub fn draft_service(&self) -> &Arc<DraftService> {
        &self.draft_service
    }

    /// IPC #25: `list_characters`
    pub fn list_characters(&self, req: &ListCharactersReq) -> Result<CharacterListDto, IpcError> {
        let conn = self.pool.get().map_err(|e| {
            IpcError::new(
                ErrorCode::DatabaseError,
                format!("Failed to acquire DB connection: {}", e),
            )
        })?;

        let items = character_repo::list_characters(
            &conn,
            req.project_id,
            req.biological_sex.as_deref(),
            req.query.as_deref(),
        )?;

        Ok(CharacterListDto { items })
    }

    /// IPC #26: `get_character_detail`
    pub fn get_character_detail(
        &self,
        req: &GetCharacterDetailReq,
    ) -> Result<CharacterDetailDto, IpcError> {
        let conn = self.pool.get().map_err(|e| {
            IpcError::new(
                ErrorCode::DatabaseError,
                format!("Failed to acquire DB connection: {}", e),
            )
        })?;

        character_repo::get_character_detail(&conn, &req.character_id, req.project_id.as_ref())
    }

    /// IPC #27: `create_or_update_library_character`
    pub fn create_or_update_library_character(
        &self,
        req: &CreateOrUpdateLibraryCharacterReq,
    ) -> Result<CharacterDetailDto, IpcError> {
        let mut conn = self.pool.get().map_err(|e| {
            IpcError::new(
                ErrorCode::DatabaseError,
                format!("Failed to acquire DB connection: {}", e),
            )
        })?;

        character_repo::create_or_update_library_character(
            &mut conn,
            req.character_id,
            req.project_id,
            &req.profile,
        )
    }

    /// IPC #28: `link_library_character_to_draft`
    pub async fn link_library_character_to_draft(
        &self,
        req: &LinkLibraryCharacterToDraftReq,
    ) -> Result<LinkLibraryCharacterResponseDto, IpcError> {
        // Step 1: Verify character exists in library
        let (code, display_name, nickname, biological_sex) = {
            let conn = self.pool.get().map_err(|e| {
                IpcError::new(
                    ErrorCode::DatabaseError,
                    format!("Failed to acquire DB connection: {}", e),
                )
            })?;

            let mut stmt = conn
                .prepare(
                    "SELECT code, display_name, nickname, biological_sex
                     FROM characters
                     WHERE id = ?1 AND deleted_at IS NULL",
                )
                .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

            let mut rows = stmt
                .query(params![req.character_id.to_string()])
                .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

            match rows
                .next()
                .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?
            {
                Some(r) => {
                    let code: Option<String> = r.get(0).ok();
                    let display_name: Option<String> = r.get(1).ok();
                    let nickname: Option<String> = r.get(2).ok();
                    let biological_sex: Option<String> = r.get(3).ok();
                    (code, display_name, nickname, biological_sex)
                }
                None => {
                    return Err(IpcError::new(
                        ErrorCode::EntityNotFound,
                        format!("Character '{}' not found in library", req.character_id),
                    ));
                }
            }
        };

        let char_code = code.unwrap_or_else(|| Uuid::now_v7().to_string());
        let resolved_name = display_name
            .clone()
            .or_else(|| nickname.clone())
            .unwrap_or_else(|| char_code.clone());

        // Check if req.draft_session_id is a draft_session_id or a committed project_id
        let (target_draft_id, direct_project_id): (Option<Uuid>, Option<Uuid>) = {
            let conn = self.pool.get().map_err(|e| {
                IpcError::new(
                    ErrorCode::DatabaseError,
                    format!("Failed to acquire DB connection: {}", e),
                )
            })?;

            let draft_proj: Option<Option<String>> = conn
                .query_row(
                    "SELECT project_id FROM draft_sessions WHERE id = ?1",
                    params![req.draft_session_id.to_string()],
                    |r| r.get(0),
                )
                .ok();

            if let Some(proj_opt) = draft_proj {
                let parsed_proj = proj_opt.and_then(|s| Uuid::parse_str(&s).ok()).filter(|pid| {
                    conn.query_row(
                        "SELECT EXISTS(SELECT 1 FROM projects WHERE id = ?1 AND deleted_at IS NULL)",
                        params![pid.to_string()],
                        |r| r.get::<_, bool>(0),
                    )
                    .unwrap_or(false)
                });
                (Some(req.draft_session_id), parsed_proj)
            } else {
                // Check if req.draft_session_id is actually an existing project_id in `projects`
                let is_project: bool = conn
                    .query_row(
                        "SELECT EXISTS(SELECT 1 FROM projects WHERE id = ?1 AND deleted_at IS NULL)",
                        params![req.draft_session_id.to_string()],
                        |r| r.get(0),
                    )
                    .unwrap_or(false);

                if is_project {
                    let editable_draft: Option<Uuid> = conn
                        .query_row(
                            "SELECT id FROM draft_sessions WHERE project_id = ?1 AND status = 'editing' ORDER BY updated_at DESC, created_at DESC LIMIT 1",
                            params![req.draft_session_id.to_string()],
                            |r| r.get::<_, String>(0),
                        )
                        .ok()
                        .and_then(|s| Uuid::parse_str(&s).ok());
                    (editable_draft, Some(req.draft_session_id))
                } else {
                    (Some(req.draft_session_id), None)
                }
            }
        };

        // If associated with a committed project in DB, persist link in `project_characters`
        if let Some(pid) = direct_project_id {
            let conn = self.pool.get().map_err(|e| {
                IpcError::new(
                    ErrorCode::DatabaseError,
                    format!("Failed to acquire DB connection: {}", e),
                )
            })?;
            let now = chrono::Utc::now().to_rfc3339();
            let link_id = Uuid::now_v7().to_string();
            conn.execute(
                "INSERT INTO project_characters (id, project_id, character_id, role, notes, created_at, deleted_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, NULL)
                 ON CONFLICT(project_id, character_id) DO UPDATE SET
                    role = COALESCE(excluded.role, project_characters.role),
                    notes = COALESCE(excluded.notes, project_characters.notes),
                    deleted_at = NULL",
                params![
                    link_id,
                    pid.to_string(),
                    req.character_id.to_string(),
                    req.role,
                    req.notes,
                    now
                ],
            )
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
        }

        // If this was a direct project link and there is no editable draft session for that project, return synthetic sealed version
        let Some(draft_session_id) = target_draft_id else {
            return Ok(LinkLibraryCharacterResponseDto {
                character_code: char_code,
                sealed_version: DraftVersionSummaryDto {
                    seq: 1,
                    display_label: "v.01".to_string(),
                    phase: 0,
                    subphase: None,
                    reason: "manual_edit".to_string(),
                    parent_seq: None,
                    created_at: chrono::Utc::now().to_rfc3339(),
                    data_hash: None,
                },
            });
        };

        // Step 2: Load runtime entry and acquire operation lock
        let entry = self
            .draft_service
            .get_or_load_runtime_entry(draft_session_id)?;
        let _guard = entry
            .acquire_operation_lock("link_library_character_to_draft", DRAFT_LOCK_TIMEOUT)
            .await?;

        // Step 3: Check draft editable status
        let snap = entry.get_snapshot();
        if snap.status == DraftRuntimeStatus::Committing
            || snap.status == DraftRuntimeStatus::Committed
        {
            return Err(IpcError::new(
                ErrorCode::DraftNotEditable,
                "Draft is committing or committed and cannot link characters",
            ));
        }

        // Step 4: Update WorkingState with reused library character
        let mut updated_ws = (*snap).clone();
        let exists = updated_ws
            .business_flow_data
            .require
            .character
            .iter()
            .any(|c| c.code == char_code);

        if !exists {
            let new_char = RequireCharacterItem {
                code: char_code.clone(),
                display_name: TrackedField::new(resolved_name, ProvenanceSource::User),
                nickname: TrackedField::new(nickname.unwrap_or_default(), ProvenanceSource::User),
                biological_sex: TrackedField::new(
                    biological_sex.unwrap_or_else(|| "NONE".to_string()),
                    ProvenanceSource::User,
                ),
                motif_code: TrackedField::default(),
                role: TrackedField::new(
                    req.role.clone().unwrap_or_default(),
                    ProvenanceSource::User,
                ),
                notes: TrackedField::new(
                    req.notes.clone().unwrap_or_default(),
                    ProvenanceSource::User,
                ),
                reused_library_character: true,
                source: Some(ProvenanceSource::User),
            };
            updated_ws
                .business_flow_data
                .require
                .character
                .push(new_char);
        }

        entry.update_working_state(updated_ws.clone());

        // Step 5: Execute TX-01 to seal new version
        let data_json = serde_json::to_string(&updated_ws.business_flow_data).map_err(|e| {
            IpcError::new(
                ErrorCode::ValidationFailed,
                format!("Failed to serialize business flow data: {}", e),
            )
        })?;

        let data_hash = state_data_hash(&updated_ws.business_flow_data).map_err(|e| {
            IpcError::new(
                ErrorCode::InternalError,
                format!("Failed to compute data hash: {}", e),
            )
        })?;

        let base_seq = snap.head_version_seq.unwrap_or(0);
        let seal_key = format!(
            "link_char:{}:{}:{}",
            draft_session_id,
            req.character_id,
            base_seq + 1
        );

        let seal_params = SealVersionParams {
            draft_session_id,
            seal_key,
            reason: "manual_edit".to_string(),
            phase: snap.phase,
            subphase: snap.subphase,
            data_json,
            data_hash,
            parent_seq: snap.head_version_seq,
            turn_id: None,
            chat_message_id: None,
        };

        let sealed_version = self.draft_service.seal_version(&seal_params)?;

        Ok(LinkLibraryCharacterResponseDto {
            character_code: char_code,
            sealed_version,
        })
    }
}
