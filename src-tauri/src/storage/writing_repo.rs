//! Writing persistence repository (`REQ-CMD-029`..`031`, `REQ-CMD-035`..`037`).
//!
//! Provides SQLite operations for:
//! - Writing Mode read-model queries and Committed-Plan gate verification (`WR-01`, `Q4.5`).
//! - Immediate scene locking and bulk locking (`TX-06`, `LOCK-01`..`LOCK-07`).
//! - Scene version history listing (`CI-08`, `SF-07`).
//! - Atomic chapter save with OCC and 5-property idempotency (`TX-07`, `SF-02`, `Q4.1`..`Q4.4`).
//! - Idempotency receipt retrieval (`Q4.7`).

use chrono::Utc;
use rusqlite::{params, Connection, OptionalExtension, TransactionBehavior};
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::domain::fingerprint::canonical_json_value;
use crate::domain::word_count::count_words;
use crate::error::{ErrorCode, IpcError};
use crate::ipc::dto::{
    BulkSceneLockStateDto, ChapterWritingReadModelDto, DirtySceneSaveItem, SaveChapterRequest,
    SaveChapterResponse, SaveReceiptLookupDto, SavedSceneResultDto, SceneLockStateDto,
    SceneReadModelDto, SceneVersionItemDto, SceneVersionListDto,
};

// ============================================================================
// Scene Source Mapping Helpers
// ============================================================================

/// Maps DTO scene source to database enum string (`'AI_GENERATED'`, `'HUMAN_EDITED'`, `'RESTORE'`).
pub fn to_db_scene_source(s: &str) -> &'static str {
    match s.trim() {
        "AI" | "AI_GENERATED" => "AI_GENERATED",
        "RESTORE" => "RESTORE",
        _ => "HUMAN_EDITED",
    }
}

/// Maps database enum string to DTO representation (`"AI"`, `"USER"`, `"RESTORE"`).
pub fn from_db_scene_source(s: &str) -> &'static str {
    match s.trim() {
        "AI_GENERATED" | "AI" => "AI",
        "RESTORE" => "RESTORE",
        _ => "USER",
    }
}

// ============================================================================
// Idempotency Payload Hash Computation
// ============================================================================

/// Computes SHA-256 hash over canonical JSON (RFC 8785) representation of sorted `dirty_scenes`.
///
/// Formulation (`be-core.md` §8.4.1, `SF-02`):
/// `payload_hash = SHA-256(JCS([{"content", "expected_version_number", "scene_id", "scene_source", "summary"}]))`
pub fn compute_save_chapter_payload_hash(
    dirty_scenes: &[DirtySceneSaveItem],
) -> Result<String, IpcError> {
    let mut sorted = dirty_scenes.to_vec();
    sorted.sort_by_key(|s| s.scene_id);
    let items: Vec<serde_json::Value> = sorted
        .iter()
        .map(|s| {
            let mut map = serde_json::Map::new();
            map.insert(
                "content".to_string(),
                serde_json::Value::String(s.content.clone()),
            );
            map.insert(
                "expected_version_number".to_string(),
                serde_json::Value::Number(s.expected_version_number.into()),
            );
            map.insert(
                "scene_id".to_string(),
                serde_json::Value::String(s.scene_id.to_string()),
            );
            map.insert(
                "scene_source".to_string(),
                serde_json::Value::String(s.scene_source.clone()),
            );
            if let Some(ref sum) = s.summary {
                map.insert(
                    "summary".to_string(),
                    serde_json::Value::String(sum.clone()),
                );
            }
            serde_json::Value::Object(map)
        })
        .collect();

    let jcs = canonical_json_value(&serde_json::Value::Array(items)).map_err(|e| {
        IpcError::new(
            ErrorCode::InternalError,
            format!("JCS encoding error: {e:?}"),
        )
    })?;
    let hash = hex::encode(Sha256::digest(&jcs));
    Ok(hash)
}

// ============================================================================
// Committed-Plan Gate Verification (WR-01)
// ============================================================================

/// Verifies that the chapter belongs to a project with a committed plan (`WR-01`, `be-core.md` §8.1).
pub fn verify_chapter_belongs_to_committed_project(
    conn: &Connection,
    project_id: Uuid,
    chapter_id: Uuid,
) -> Result<(), IpcError> {
    // 1. Verify project exists
    let project_status: Option<String> = conn
        .query_row(
            "SELECT status FROM projects WHERE id = ?1 AND deleted_at IS NULL",
            params![project_id.to_string()],
            |row| row.get(0),
        )
        .optional()
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

    let project_status = match project_status {
        Some(status) => status,
        None => {
            return Err(IpcError::new(
                ErrorCode::EntityNotFound,
                format!("Project {} not found", project_id),
            ));
        }
    };

    // 2. Verify chapter exists in the project hierarchy
    let chapter_exists: bool = conn
        .query_row(
            "SELECT 1 FROM chapters c
             JOIN arcs a ON c.arc_id = a.id
             WHERE c.id = ?1 AND a.project_id = ?2
               AND c.deleted_at IS NULL AND a.deleted_at IS NULL",
            params![chapter_id.to_string(), project_id.to_string()],
            |_| Ok(true),
        )
        .optional()
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?
        .unwrap_or(false);

    if !chapter_exists {
        return Err(IpcError::new(
            ErrorCode::EntityNotFound,
            format!("Chapter {} not found in project {}", chapter_id, project_id),
        ));
    }

    // 3. Verify committed plan
    let is_committed_plan = if project_status != "PLANNING" {
        true
    } else {
        let has_committed_draft: bool = conn
            .query_row(
                "SELECT 1 FROM draft_sessions WHERE project_id = ?1 AND status = 'committed'",
                params![project_id.to_string()],
                |_| Ok(true),
            )
            .optional()
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?
            .unwrap_or(false);
        has_committed_draft
    };

    if !is_committed_plan {
        return Err(IpcError::new(
            ErrorCode::ValidationFailed,
            "Writing Mode requires a committed project plan (WR-01). Please commit the draft first.",
        ));
    }

    Ok(())
}

// ============================================================================
// Read Model Queries
// ============================================================================

/// Loads scenes for a chapter as `SceneReadModelDto` ordered by `position ASC` (`Q4.5`).
pub fn load_chapter_scenes_read_model(
    conn: &Connection,
    chapter_id: Uuid,
) -> Result<Vec<SceneReadModelDto>, IpcError> {
    let mut stmt = conn
        .prepare(
            "SELECT
                s.id,
                s.chapter_id,
                COALESCE(s.position, 0),
                COALESCE(s.display_name, ''),
                s.summary,
                s.content,
                COALESCE(s.word_count, 0),
                COALESCE(s.is_locked, 0),
                COALESCE(latest_sv.version_number, 0) AS head_version_number,
                latest_sv.scene_source AS current_scene_source
            FROM scenes s
            LEFT JOIN scene_versions latest_sv
                ON latest_sv.parent_id = s.id
                AND latest_sv.version_number = (
                    SELECT MAX(sv2.version_number)
                    FROM scene_versions sv2
                    WHERE sv2.parent_id = s.id
                )
            WHERE s.chapter_id = ?1
              AND s.deleted_at IS NULL
            ORDER BY s.position ASC",
        )
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

    let rows = stmt
        .query_map(params![chapter_id.to_string()], |row| {
            let id_str: String = row.get(0)?;
            let chapter_id_str: String = row.get(1)?;
            let position: i32 = row.get(2)?;
            let display_name: String = row.get(3)?;
            let summary: Option<String> = row.get(4)?;
            let content: Option<String> = row.get(5)?;
            let word_count: i64 = row.get(6)?;
            let is_locked_int: i32 = row.get(7)?;
            let head_ver: i32 = row.get(8)?;
            let raw_source: Option<String> = row.get(9)?;

            Ok((
                id_str,
                chapter_id_str,
                position,
                display_name,
                summary,
                content,
                word_count,
                is_locked_int,
                head_ver,
                raw_source,
            ))
        })
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

    let mut scenes = Vec::new();
    for row in rows {
        let (
            id_str,
            chapter_id_str,
            position,
            display_name,
            summary,
            content,
            word_count,
            is_locked_int,
            head_ver,
            raw_source,
        ) = row.map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

        let id = Uuid::parse_str(&id_str).map_err(|e| {
            IpcError::new(ErrorCode::InternalError, format!("Invalid scene UUID: {e}"))
        })?;
        let cid = Uuid::parse_str(&chapter_id_str).map_err(|e| {
            IpcError::new(
                ErrorCode::InternalError,
                format!("Invalid chapter UUID: {e}"),
            )
        })?;

        let current_scene_source = match raw_source {
            Some(ref s) if !s.trim().is_empty() => from_db_scene_source(s).to_string(),
            _ => String::new(),
        };

        scenes.push(SceneReadModelDto {
            id,
            chapter_id: cid,
            position,
            display_name,
            summary,
            content,
            word_count: word_count as u32,
            is_locked: is_locked_int != 0,
            head_version_number: head_ver,
            current_scene_source,
        });
    }

    Ok(scenes)
}

/// Retrieves complete writing state for a chapter (`REQ-CMD-029`).
pub fn get_chapter_writing_state(
    conn: &Connection,
    project_id: Uuid,
    chapter_id: Uuid,
) -> Result<ChapterWritingReadModelDto, IpcError> {
    // 1. Verify gate
    verify_chapter_belongs_to_committed_project(conn, project_id, chapter_id)?;

    // 2. Load chapter metadata
    let chapter_info: Option<(Option<String>, String, i32, Option<String>)> = conn
        .query_row(
            "SELECT c.arc_id, COALESCE(c.display_name, ''), COALESCE(c.position, 0), c.updated_at
             FROM chapters c
             JOIN arcs a ON c.arc_id = a.id
             WHERE c.id = ?1 AND a.project_id = ?2
               AND c.deleted_at IS NULL AND a.deleted_at IS NULL",
            params![chapter_id.to_string(), project_id.to_string()],
            |row| {
                let arc_id: Option<String> = row.get(0)?;
                let title: String = row.get(1)?;
                let pos: i32 = row.get(2)?;
                let updated_at: Option<String> = row.get(3)?;
                Ok((arc_id, title, pos, updated_at))
            },
        )
        .optional()
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

    let (arc_id_str, chapter_title, position, updated_at) = match chapter_info {
        Some(info) => info,
        None => {
            return Err(IpcError::new(
                ErrorCode::EntityNotFound,
                format!("Chapter {} not found", chapter_id),
            ));
        }
    };

    let arc_id = arc_id_str.and_then(|s| Uuid::parse_str(&s).ok());

    // 3. Load scenes
    let scenes = load_chapter_scenes_read_model(conn, chapter_id)?;

    Ok(ChapterWritingReadModelDto {
        project_id,
        chapter_id,
        arc_id,
        chapter_title,
        position,
        updated_at,
        scenes,
    })
}

// ============================================================================
// Scene Lock Operations (TX-06)
// ============================================================================

/// Immediately toggles lock on a single scene within `BEGIN IMMEDIATE` (`TX-06`, `LOCK-01`..`LOCK-07`).
pub fn set_scene_lock_tx(
    conn: &mut Connection,
    project_id: Uuid,
    chapter_id: Uuid,
    scene_id: Uuid,
    desired_locked: bool,
) -> Result<SceneLockStateDto, IpcError> {
    let tx = conn
        .transaction_with_behavior(TransactionBehavior::Immediate)
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

    // Verify scene exists in chapter/project
    let scene_exists: bool = tx
        .query_row(
            "SELECT 1 FROM scenes s
             JOIN chapters c ON s.chapter_id = c.id
             JOIN arcs a ON c.arc_id = a.id
             WHERE s.id = ?1 AND s.chapter_id = ?2 AND a.project_id = ?3
               AND s.deleted_at IS NULL AND c.deleted_at IS NULL AND a.deleted_at IS NULL",
            params![
                scene_id.to_string(),
                chapter_id.to_string(),
                project_id.to_string()
            ],
            |_| Ok(true),
        )
        .optional()
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?
        .unwrap_or(false);

    if !scene_exists {
        return Err(IpcError::new(
            ErrorCode::EntityNotFound,
            format!(
                "Scene {} not found in chapter {} and project {}",
                scene_id, chapter_id, project_id
            ),
        ));
    }

    let is_locked_val = if desired_locked { 1 } else { 0 };
    tx.execute(
        "UPDATE scenes SET is_locked = ?1 WHERE id = ?2 AND chapter_id = ?3 AND deleted_at IS NULL",
        params![is_locked_val, scene_id.to_string(), chapter_id.to_string()],
    )
    .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

    tx.commit()
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

    Ok(SceneLockStateDto {
        scene_id,
        is_locked: desired_locked,
        invalidated_generations: vec![],
    })
}

/// Immediately toggles lock for all scenes in a chapter within `BEGIN IMMEDIATE` (`TX-06`).
pub fn set_chapter_scenes_lock_bulk_tx(
    conn: &mut Connection,
    project_id: Uuid,
    chapter_id: Uuid,
    is_locked: bool,
) -> Result<BulkSceneLockStateDto, IpcError> {
    let tx = conn
        .transaction_with_behavior(TransactionBehavior::Immediate)
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

    // Verify chapter exists in project
    let chapter_exists: bool = tx
        .query_row(
            "SELECT 1 FROM chapters c
             JOIN arcs a ON c.arc_id = a.id
             WHERE c.id = ?1 AND a.project_id = ?2
               AND c.deleted_at IS NULL AND a.deleted_at IS NULL",
            params![chapter_id.to_string(), project_id.to_string()],
            |_| Ok(true),
        )
        .optional()
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?
        .unwrap_or(false);

    if !chapter_exists {
        return Err(IpcError::new(
            ErrorCode::EntityNotFound,
            format!("Chapter {} not found in project {}", chapter_id, project_id),
        ));
    }

    let is_locked_val = if is_locked { 1 } else { 0 };
    tx.execute(
        "UPDATE scenes SET is_locked = ?1 WHERE chapter_id = ?2 AND deleted_at IS NULL",
        params![is_locked_val, chapter_id.to_string()],
    )
    .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

    let mut stmt = tx
        .prepare(
            "SELECT id FROM scenes WHERE chapter_id = ?1 AND deleted_at IS NULL ORDER BY position ASC",
        )
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

    let scene_ids_res = stmt
        .query_map(params![chapter_id.to_string()], |row| {
            let id_str: String = row.get(0)?;
            Ok(id_str)
        })
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

    let mut scenes = Vec::new();
    for id_res in scene_ids_res {
        let id_str = id_res.map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
        if let Ok(sid) = Uuid::parse_str(&id_str) {
            scenes.push(SceneLockStateDto {
                scene_id: sid,
                is_locked,
                invalidated_generations: vec![],
            });
        }
    }
    drop(stmt);

    tx.commit()
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

    Ok(BulkSceneLockStateDto { chapter_id, scenes })
}

// ============================================================================
// Scene Versions Listing (REQ-CMD-035)
// ============================================================================

/// Lists version history for a scene ordered by `version_number DESC` (`REQ-CMD-035`, `CI-08`).
pub fn list_scene_versions(
    conn: &Connection,
    project_id: Uuid,
    chapter_id: Uuid,
    scene_id: Uuid,
) -> Result<SceneVersionListDto, IpcError> {
    // Verify scene exists in chapter/project
    let scene_exists: bool = conn
        .query_row(
            "SELECT 1 FROM scenes s
             JOIN chapters c ON s.chapter_id = c.id
             JOIN arcs a ON c.arc_id = a.id
             WHERE s.id = ?1 AND s.chapter_id = ?2 AND a.project_id = ?3
               AND s.deleted_at IS NULL AND c.deleted_at IS NULL AND a.deleted_at IS NULL",
            params![
                scene_id.to_string(),
                chapter_id.to_string(),
                project_id.to_string()
            ],
            |_| Ok(true),
        )
        .optional()
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?
        .unwrap_or(false);

    if !scene_exists {
        return Err(IpcError::new(
            ErrorCode::EntityNotFound,
            format!(
                "Scene {} not found in chapter {} and project {}",
                scene_id, chapter_id, project_id
            ),
        ));
    }

    let mut stmt = conn
        .prepare(
            "SELECT version_number, content, word_count, scene_source, summary, created_at
             FROM scene_versions
             WHERE parent_id = ?1
             ORDER BY version_number DESC",
        )
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

    let rows = stmt
        .query_map(params![scene_id.to_string()], |row| {
            let ver: i32 = row.get(0)?;
            let content: Option<String> = row.get(1)?;
            let word_count: Option<i64> = row.get(2)?;
            let source: Option<String> = row.get(3)?;
            let summary: Option<String> = row.get(4)?;
            let created_at: Option<String> = row.get(5)?;
            Ok((ver, content, word_count, source, summary, created_at))
        })
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

    let mut versions = Vec::new();
    for row in rows {
        let (ver, content, word_count, source, summary, created_at) =
            row.map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

        let scene_source = match source {
            Some(ref s) => from_db_scene_source(s).to_string(),
            None => "USER".to_string(),
        };

        versions.push(SceneVersionItemDto {
            version_number: ver,
            content: content.unwrap_or_default(),
            word_count: word_count.unwrap_or(0) as u32,
            scene_source,
            summary,
            created_at: created_at.unwrap_or_default(),
        });
    }

    let head_version_number = versions.first().map(|v| v.version_number).unwrap_or(0);

    Ok(SceneVersionListDto {
        scene_id,
        head_version_number,
        versions,
    })
}

// ============================================================================
// Atomic Chapter Save (TX-07)
// ============================================================================

/// Atomic chapter save executing OCC, lock checks, version persistence, and receipt indexing (`TX-07`).
pub fn save_chapter_tx(
    conn: &mut Connection,
    req: &SaveChapterRequest,
) -> Result<SaveChapterResponse, IpcError> {
    let payload_hash = compute_save_chapter_payload_hash(&req.dirty_scenes)?;

    let tx = conn
        .transaction_with_behavior(TransactionBehavior::Immediate)
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

    // ------------------------------------------------------------------------
    // Step 1: Idempotency Receipt Check (SF-02, Q4.1)
    // ------------------------------------------------------------------------
    let existing_receipt: Option<(String, String)> = tx
        .query_row(
            "SELECT payload_hash, response_json FROM chapter_save_receipts
             WHERE chapter_id = ?1 AND client_save_id = ?2",
            params![req.chapter_id.to_string(), req.client_save_id],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .optional()
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

    if let Some((stored_hash, response_json)) = existing_receipt {
        if stored_hash == payload_hash {
            // Idempotent replay: return cached response with was_idempotent_replay = true
            let mut cached_resp: SaveChapterResponse = serde_json::from_str(&response_json)
                .map_err(|e| {
                    IpcError::new(
                        ErrorCode::InternalError,
                        format!("Invalid receipt JSON: {e}"),
                    )
                })?;
            cached_resp.was_idempotent_replay = true;
            tx.commit()
                .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
            return Ok(cached_resp);
        } else {
            // Replay conflict: same client_save_id with different payload
            return Err(IpcError::new(
                ErrorCode::IdempotencyKeyPayloadMismatch,
                "Idempotency conflict: client_save_id already used with a different payload",
            ));
        }
    }

    // ------------------------------------------------------------------------
    // Step 2: Verify Chapter Ownership & Committed-Plan Gate (WR-01)
    // ------------------------------------------------------------------------
    verify_chapter_belongs_to_committed_project(&tx, req.project_id, req.chapter_id)?;

    // If no dirty scenes, return current state without creating versions or receipt
    if req.dirty_scenes.is_empty() {
        let current_scenes = load_chapter_scenes_read_model(&tx, req.chapter_id)?;
        tx.commit()
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
        return Ok(SaveChapterResponse {
            chapter_id: req.chapter_id,
            client_save_id: req.client_save_id.clone(),
            saved_scenes: vec![],
            chapter_scenes: current_scenes,
            was_idempotent_replay: false,
        });
    }

    // ------------------------------------------------------------------------
    // Step 3: Validate All Dirty Scenes (Scene Existence, Lock Check, OCC Check)
    //
    // INVARIANT: Clean scenes (locked or unlocked) are NOT validated here and
    // do NOT block Save of other dirty scenes (P4S4-CLEAN-LOCKED-DOES-NOT-BLOCK).
    // If ANY dirty scene is locked or fails OCC, transaction aborts completely
    // (P4S4-DIRTY-LOCKED-ABORTS-ALL, P4S4-OCC-MISMATCH).
    // ------------------------------------------------------------------------
    struct ValidatedSceneUpdate<'a> {
        item: &'a DirtySceneSaveItem,
        next_ver: i32,
        word_count: u32,
    }

    let mut validated_updates = Vec::with_capacity(req.dirty_scenes.len());

    for item in &req.dirty_scenes {
        // Query scene in chapter
        let scene_row: Option<(i32, Option<String>)> = tx
            .query_row(
                "SELECT COALESCE(is_locked, 0), content FROM scenes
                 WHERE id = ?1 AND chapter_id = ?2 AND deleted_at IS NULL",
                params![item.scene_id.to_string(), req.chapter_id.to_string()],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .optional()
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

        let (is_locked_int, _current_content) = match scene_row {
            Some(row) => row,
            None => {
                return Err(IpcError::new(
                    ErrorCode::EntityNotFound,
                    format!(
                        "Scene {} not found in chapter {}",
                        item.scene_id, req.chapter_id
                    ),
                ));
            }
        };

        // 1. Lock check (LOCK-05)
        if is_locked_int != 0 {
            return Err(IpcError::with_details(
                ErrorCode::SceneLocked,
                format!(
                    "Cảnh {} đang bị khóa, không thể lưu thay đổi nội dung",
                    item.scene_id
                ),
                serde_json::json!({ "locked_scene_id": item.scene_id }),
            ));
        }

        // 2. OCC check: expected_version_number must match COALESCE(MAX(version_number), 0) (Q4.2)
        let current_max_ver: i32 = tx
            .query_row(
                "SELECT COALESCE(MAX(version_number), 0) FROM scene_versions WHERE parent_id = ?1",
                params![item.scene_id.to_string()],
                |row| row.get(0),
            )
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

        if item.expected_version_number != current_max_ver {
            return Err(IpcError::with_details(
                ErrorCode::OccVersionMismatch,
                format!(
                    "Xung đột phiên bản trên cảnh {}: kỳ vọng v.{}, nhưng DB đang ở v.{}",
                    item.scene_id, item.expected_version_number, current_max_ver
                ),
                serde_json::json!({
                    "scene_id": item.scene_id,
                    "expected_version_number": item.expected_version_number,
                    "actual_version_number": current_max_ver,
                }),
            ));
        }

        let word_count = count_words(&item.content);
        let next_ver = current_max_ver + 1;
        validated_updates.push(ValidatedSceneUpdate {
            item,
            next_ver,
            word_count,
        });
    }

    // ------------------------------------------------------------------------
    // Step 4: Persist Updates to `scenes` and `scene_versions` (SF-07)
    // ------------------------------------------------------------------------
    let now_str = Utc::now().to_rfc3339();
    let mut saved_scenes = Vec::with_capacity(validated_updates.len());

    for update in validated_updates {
        let item = update.item;
        let db_source = to_db_scene_source(&item.scene_source);

        // Update scenes table
        tx.execute(
            "UPDATE scenes SET content = ?1, word_count = ?2
             WHERE id = ?3 AND chapter_id = ?4 AND deleted_at IS NULL",
            params![
                item.content,
                update.word_count as i64,
                item.scene_id.to_string(),
                req.chapter_id.to_string()
            ],
        )
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

        // Insert new immutable row in scene_versions (SF-07: parent_id NOT NULL)
        let version_id = Uuid::now_v7().to_string();
        tx.execute(
            "INSERT INTO scene_versions (
                id, parent_id, version_number, scene_source, summary, content, word_count, created_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                version_id,
                item.scene_id.to_string(),
                update.next_ver,
                db_source,
                item.summary,
                item.content,
                update.word_count as i64,
                now_str
            ],
        )
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

        saved_scenes.push(SavedSceneResultDto {
            scene_id: item.scene_id,
            new_version_number: update.next_ver,
            word_count: update.word_count,
        });
    }

    // Update chapter updated_at
    tx.execute(
        "UPDATE chapters SET updated_at = ?1 WHERE id = ?2 AND deleted_at IS NULL",
        params![now_str, req.chapter_id.to_string()],
    )
    .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

    // Load fresh read model for the entire chapter
    let chapter_scenes = load_chapter_scenes_read_model(&tx, req.chapter_id)?;

    let response = SaveChapterResponse {
        chapter_id: req.chapter_id,
        client_save_id: req.client_save_id.clone(),
        saved_scenes,
        chapter_scenes,
        was_idempotent_replay: false,
    };

    // ------------------------------------------------------------------------
    // Step 5: Insert Receipt into `chapter_save_receipts` (SF-02)
    // ------------------------------------------------------------------------
    let response_json = serde_json::to_string(&response).map_err(|e| {
        IpcError::new(
            ErrorCode::InternalError,
            format!("Serialization error: {e}"),
        )
    })?;
    let receipt_id = Uuid::now_v7().to_string();

    let inserted = tx
        .execute(
            "INSERT INTO chapter_save_receipts (
                id, chapter_id, client_save_id, payload_hash, response_json, created_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6)
            ON CONFLICT (chapter_id, client_save_id) DO NOTHING",
            params![
                receipt_id,
                req.chapter_id.to_string(),
                req.client_save_id,
                payload_hash,
                response_json,
                now_str
            ],
        )
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

    if inserted == 0 {
        // Race check
        let existing: Option<String> = tx
            .query_row(
                "SELECT payload_hash FROM chapter_save_receipts WHERE chapter_id = ?1 AND client_save_id = ?2",
                params![req.chapter_id.to_string(), req.client_save_id],
                |row| row.get(0),
            )
            .optional()
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

        if existing.as_deref() != Some(&payload_hash) {
            return Err(IpcError::new(
                ErrorCode::IdempotencyKeyPayloadMismatch,
                "Concurrent Save Chapter idempotency payload mismatch",
            ));
        }
    }

    tx.commit()
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

    Ok(response)
}

// ============================================================================
// Save Receipt Lookup (REQ-CMD-037)
// ============================================================================

/// Retrieves a save receipt for a chapter and clientSaveId (`REQ-CMD-037`, `Q4.7`).
pub fn get_chapter_save_receipt(
    conn: &Connection,
    chapter_id: Uuid,
    client_save_id: &str,
) -> Result<SaveReceiptLookupDto, IpcError> {
    // Verify chapter exists
    let chapter_exists: bool = conn
        .query_row(
            "SELECT 1 FROM chapters WHERE id = ?1 AND deleted_at IS NULL",
            params![chapter_id.to_string()],
            |_| Ok(true),
        )
        .optional()
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?
        .unwrap_or(false);

    if !chapter_exists {
        return Err(IpcError::new(
            ErrorCode::EntityNotFound,
            format!("Chapter {} not found", chapter_id),
        ));
    }

    let current_chapter_scenes = load_chapter_scenes_read_model(conn, chapter_id)?;

    let receipt_json: Option<String> = conn
        .query_row(
            "SELECT response_json FROM chapter_save_receipts WHERE chapter_id = ?1 AND client_save_id = ?2",
            params![chapter_id.to_string(), client_save_id],
            |row| row.get(0),
        )
        .optional()
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

    match receipt_json {
        Some(json_str) => {
            let receipt: SaveChapterResponse = serde_json::from_str(&json_str).map_err(|e| {
                IpcError::new(
                    ErrorCode::InternalError,
                    format!("Invalid receipt JSON: {e}"),
                )
            })?;
            Ok(SaveReceiptLookupDto {
                found: true,
                receipt: Some(receipt),
                current_chapter_scenes,
            })
        }
        None => Ok(SaveReceiptLookupDto {
            found: false,
            receipt: None,
            current_chapter_scenes,
        }),
    }
}
