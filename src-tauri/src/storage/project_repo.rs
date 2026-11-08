//! Project storage repository for project listing, details, and scoped soft-delete
//! (`REQ-PROJECT-001`, `REQ-CMD-022`, `REQ-CMD-023`, `REQ-CMD-024`, `be-core.md` §10.2.7).

use chrono::Utc;
use rusqlite::{params, Connection, TransactionBehavior};
use uuid::Uuid;

use crate::error::{ErrorCode, IpcError};
use crate::ipc::dto::{
    ArcItemUiDto, ChapterBlockUiDto, ChapterItemUiDto, ProjectDetailDto, ProjectDetailsUiDto,
    ProjectItemUiDto,
};

/// Normalizes query string to lowercase and collapses whitespace.
pub fn normalize_query(q: &str) -> String {
    q.trim().to_lowercase()
}

/// Formats database status enum string into frontend UI status.
pub fn format_ui_status(status: Option<&str>) -> String {
    match status.unwrap_or("PLANNING") {
        "PLANNING" => "Planning".to_string(),
        "DRAFTING" => "Drafting".to_string(),
        "ONGOING" => "Ongoing".to_string(),
        "IN_REVIEW" => "In Review".to_string(),
        "COMPLETED" => "Completed".to_string(),
        other => other.to_string(),
    }
}

/// Lists projects matching optional query and sort criteria (`REQ-CMD-022`).
///
/// Only non-deleted projects (`deleted_at IS NULL`) are returned.
pub fn list_projects(
    conn: &Connection,
    query: Option<&str>,
    sort: Option<&str>,
) -> Result<Vec<ProjectItemUiDto>, IpcError> {
    let order_clause = match sort.unwrap_or("modified") {
        "a-z" => "ORDER BY LOWER(COALESCE(p.display_name, '')) ASC",
        "z-a" => "ORDER BY LOWER(COALESCE(p.display_name, '')) DESC",
        _ => "ORDER BY COALESCE(p.updated_at, p.created_at) DESC",
    };

    let sql = format!(
        "SELECT p.id, p.code, p.display_name, p.status, p.cover, p.summary, p.created_at, p.updated_at
         FROM projects p
         WHERE p.deleted_at IS NULL
         {order_clause}"
    );

    let mut stmt = conn
        .prepare(&sql)
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, format!("Prepare list_projects failed: {}", e)))?;

    let rows = stmt
        .query_map([], |row| {
            let id: String = row.get(0)?;
            let code: Option<String> = row.get(1)?;
            let display_name: Option<String> = row.get(2)?;
            let status: Option<String> = row.get(3)?;
            let cover: Option<String> = row.get(4)?;
            let summary: Option<String> = row.get(5)?;
            let created_at: Option<String> = row.get(6)?;
            let updated_at: Option<String> = row.get(7)?;

            Ok((id, code, display_name, status, cover, summary, created_at, updated_at))
        })
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, format!("Query list_projects failed: {}", e)))?;

    let norm_q = query.map(normalize_query).filter(|q| !q.is_empty());

    let mut items = Vec::new();
    for r in rows {
        let (id, code, display_name, status, cover, summary, created_at, updated_at) =
            r.map_err(|e| IpcError::new(ErrorCode::DatabaseError, format!("Row mapping failed: {}", e)))?;

        let title = display_name.unwrap_or_else(|| code.clone().unwrap_or_else(|| "Untitled Project".to_string()));
        let desc = summary.unwrap_or_default();

        if let Some(ref q) = norm_q {
            let title_match = title.to_lowercase().contains(q);
            let desc_match = desc.to_lowercase().contains(q);
            let code_match = code.as_deref().map(|c| c.to_lowercase().contains(q)).unwrap_or(false);
            if !title_match && !desc_match && !code_match {
                continue;
            }
        }

        let genres = fetch_project_genres(conn, &id)?;
        let details = fetch_project_details_counts(conn, &id, created_at.as_deref().unwrap_or(""))?;
        let modified_at = updated_at.or(created_at).unwrap_or_default();

        items.push(ProjectItemUiDto {
            id,
            title,
            status: format_ui_status(status.as_deref()),
            cover: cover.unwrap_or_default(),
            description: desc,
            genres,
            modified_at,
            details,
            arcs: None,
        });
    }

    Ok(items)
}

/// Fetches full project details including arc/chapter hierarchy (`REQ-CMD-023`).
pub fn get_project_detail(
    conn: &Connection,
    project_id: &Uuid,
) -> Result<ProjectDetailDto, IpcError> {
    let proj_id_str = project_id.to_string();

    let mut stmt = conn
        .prepare(
            "SELECT p.id, p.code, p.display_name, p.status, p.cover, p.summary, p.created_at, p.updated_at
             FROM projects p
             WHERE p.id = ?1 AND p.deleted_at IS NULL",
        )
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

    let mut rows = stmt
        .query(params![proj_id_str])
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

    let row = match rows
        .next()
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?
    {
        Some(r) => r,
        None => {
            return Err(IpcError::new(
                ErrorCode::EntityNotFound,
                format!("Project '{}' not found", project_id),
            ))
        }
    };

    let id: String = row.get(0).map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
    let code: Option<String> = row.get(1).map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
    let display_name: Option<String> = row.get(2).map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
    let status: Option<String> = row.get(3).map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
    let cover: Option<String> = row.get(4).map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
    let summary: Option<String> = row.get(5).map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
    let created_at: Option<String> = row.get(6).map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
    let updated_at: Option<String> = row.get(7).map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

    let title = display_name.unwrap_or_else(|| code.unwrap_or_else(|| "Untitled Project".to_string()));
    let desc = summary.unwrap_or_default();
    let genres = fetch_project_genres(conn, &id)?;
    let details = fetch_project_details_counts(conn, &id, created_at.as_deref().unwrap_or(""))?;
    let modified_at = updated_at.or(created_at).unwrap_or_default();

    let arcs = fetch_project_arcs_hierarchy(conn, &id)?;

    // Check has_committed_plan
    let has_committed_plan: bool = conn
        .query_row(
            "SELECT EXISTS (
                SELECT 1 FROM draft_sessions WHERE project_id = ?1 AND status = 'committed'
            ) OR EXISTS (
                SELECT 1 FROM arcs WHERE project_id = ?1 AND deleted_at IS NULL
            )",
            params![proj_id_str],
            |r| r.get(0),
        )
        .unwrap_or(false);

    // Latest draft session ID
    let draft_session_id: Option<Uuid> = conn
        .query_row(
            "SELECT id FROM draft_sessions WHERE project_id = ?1 ORDER BY created_at DESC LIMIT 1",
            params![proj_id_str],
            |r| {
                let s: String = r.get(0)?;
                Ok(Uuid::parse_str(&s).ok())
            },
        )
        .unwrap_or(None);

    // Check if any scenes have written content
    let has_written_scenes: bool = conn
        .query_row(
            "SELECT
                EXISTS (
                    SELECT 1 FROM scenes s
                    JOIN chapters c ON s.chapter_id = c.id
                    JOIN arcs a ON c.arc_id = a.id
                    WHERE a.project_id = ?1
                      AND s.deleted_at IS NULL
                      AND s.content IS NOT NULL
                      AND trim(s.content) <> ''
                )
                OR EXISTS (
                    SELECT 1 FROM scene_versions sv
                    JOIN scenes s ON sv.parent_id = s.id
                    JOIN chapters c ON s.chapter_id = c.id
                    JOIN arcs a ON c.arc_id = a.id
                    WHERE a.project_id = ?1
                )",
            params![proj_id_str],
            |r| r.get(0),
        )
        .unwrap_or(false);

    let can_reopen_draft = has_committed_plan && !has_written_scenes && draft_session_id.is_some();

    let project = ProjectItemUiDto {
        id,
        title,
        status: format_ui_status(status.as_deref()),
        cover: cover.unwrap_or_default(),
        description: desc,
        genres,
        modified_at,
        details,
        arcs: Some(arcs),
    };

    Ok(ProjectDetailDto {
        project,
        has_committed_plan,
        draft_session_id,
        can_reopen_draft,
    })
}

/// Executes scoped soft-delete within `BEGIN IMMEDIATE` (`REQ-CMD-024`).
///
/// Sets `deleted_at = now()` on project, arcs, chapters, scenes, project_characters,
/// and project-specific world_settings/lore_entries.
/// Preserves shared characters and global template world settings (`project_id IS NULL`).
pub fn delete_project(conn: &mut Connection, project_id: &Uuid) -> Result<bool, IpcError> {
    let proj_id_str = project_id.to_string();

    let tx = conn
        .transaction_with_behavior(TransactionBehavior::Immediate)
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, format!("Begin transaction failed: {}", e)))?;

    // Verify project exists and is not deleted
    let exists: bool = tx
        .query_row(
            "SELECT EXISTS (SELECT 1 FROM projects WHERE id = ?1 AND deleted_at IS NULL)",
            params![proj_id_str],
            |r| r.get(0),
        )
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

    if !exists {
        return Err(IpcError::new(
            ErrorCode::EntityNotFound,
            format!("Project '{}' not found or already deleted", project_id),
        ));
    }

    let now = Utc::now().to_rfc3339();

    // 1. Soft-delete scenes
    tx.execute(
        "UPDATE scenes SET deleted_at = ?1
         WHERE chapter_id IN (
             SELECT c.id FROM chapters c
             JOIN arcs a ON c.arc_id = a.id
             WHERE a.project_id = ?2
         ) AND deleted_at IS NULL",
        params![now, proj_id_str],
    )
    .map_err(|e| IpcError::new(ErrorCode::DatabaseError, format!("Soft delete scenes failed: {}", e)))?;

    // 2. Soft-delete chapters
    tx.execute(
        "UPDATE chapters SET deleted_at = ?1
         WHERE arc_id IN (
             SELECT a.id FROM arcs a
             WHERE a.project_id = ?2
         ) AND deleted_at IS NULL",
        params![now, proj_id_str],
    )
    .map_err(|e| IpcError::new(ErrorCode::DatabaseError, format!("Soft delete chapters failed: {}", e)))?;

    // 3. Soft-delete arcs
    tx.execute(
        "UPDATE arcs SET deleted_at = ?1
         WHERE project_id = ?2 AND deleted_at IS NULL",
        params![now, proj_id_str],
    )
    .map_err(|e| IpcError::new(ErrorCode::DatabaseError, format!("Soft delete arcs failed: {}", e)))?;

    // 4. Soft-delete themes belonging to project arcs
    tx.execute(
        "UPDATE themes SET deleted_at = ?1
         WHERE arc_id IN (SELECT a.id FROM arcs a WHERE a.project_id = ?2) AND deleted_at IS NULL",
        params![now, proj_id_str],
    )
    .map_err(|e| IpcError::new(ErrorCode::DatabaseError, format!("Soft delete themes failed: {}", e)))?;

    // 5. Soft-delete project_characters (links only, characters table preserved)
    tx.execute(
        "UPDATE project_characters SET deleted_at = ?1
         WHERE project_id = ?2 AND deleted_at IS NULL",
        params![now, proj_id_str],
    )
    .map_err(|e| IpcError::new(ErrorCode::DatabaseError, format!("Soft delete project_characters failed: {}", e)))?;

    // 6. Soft-delete lore_entries belonging to project-specific world_settings
    tx.execute(
        "UPDATE lore_entries SET deleted_at = ?1
         WHERE world_setting_id IN (
             SELECT id FROM world_settings WHERE project_id = ?2
         ) AND deleted_at IS NULL",
        params![now, proj_id_str],
    )
    .map_err(|e| IpcError::new(ErrorCode::DatabaseError, format!("Soft delete lore_entries failed: {}", e)))?;

    // 7. Soft-delete world_settings belonging to this project (NOT template ones where project_id IS NULL)
    tx.execute(
        "UPDATE world_settings SET deleted_at = ?1
         WHERE project_id = ?2 AND deleted_at IS NULL",
        params![now, proj_id_str],
    )
    .map_err(|e| IpcError::new(ErrorCode::DatabaseError, format!("Soft delete world_settings failed: {}", e)))?;

    // 8. Soft-delete project itself
    tx.execute(
        "UPDATE projects SET deleted_at = ?1
         WHERE id = ?2 AND deleted_at IS NULL",
        params![now, proj_id_str],
    )
    .map_err(|e| IpcError::new(ErrorCode::DatabaseError, format!("Soft delete projects failed: {}", e)))?;

    tx.commit()
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, format!("Commit soft delete failed: {}", e)))?;

    Ok(true)
}

// ----------------------------------------------------------------------------
// Internal Helpers
// ----------------------------------------------------------------------------

fn fetch_project_genres(conn: &Connection, project_id: &str) -> Result<Vec<String>, IpcError> {
    let mut stmt = conn
        .prepare(
            "SELECT COALESCE(g.display_name, g.code)
             FROM project_genres pg
             JOIN genres g ON pg.genre_id = g.id
             WHERE pg.project_id = ?1
             ORDER BY pg.is_primary DESC, g.display_name ASC",
        )
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

    let rows = stmt
        .query_map(params![project_id], |r| r.get::<_, String>(0))
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

    let mut genres = Vec::new();
    for g in rows {
        genres.push(g.map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?);
    }
    Ok(genres)
}

fn fetch_project_details_counts(
    conn: &Connection,
    project_id: &str,
    created_date: &str,
) -> Result<ProjectDetailsUiDto, IpcError> {
    let word_count: i64 = conn
        .query_row(
            "SELECT COALESCE(SUM(s.word_count), 0)
             FROM scenes s
             JOIN chapters c ON s.chapter_id = c.id
             JOIN arcs a ON c.arc_id = a.id
             WHERE a.project_id = ?1 AND s.deleted_at IS NULL",
            params![project_id],
            |r| r.get(0),
        )
        .unwrap_or(0);

    let chapters_count: i64 = conn
        .query_row(
            "SELECT COUNT(c.id)
             FROM chapters c
             JOIN arcs a ON c.arc_id = a.id
             WHERE a.project_id = ?1 AND c.deleted_at IS NULL",
            params![project_id],
            |r| r.get(0),
        )
        .unwrap_or(0);

    let characters_count: i64 = conn
        .query_row(
            "SELECT COUNT(pc.id)
             FROM project_characters pc
             WHERE pc.project_id = ?1 AND pc.deleted_at IS NULL",
            params![project_id],
            |r| r.get(0),
        )
        .unwrap_or(0);

    let world_count: i64 = conn
        .query_row(
            "SELECT COUNT(ws.id)
             FROM world_settings ws
             WHERE ws.project_id = ?1 AND ws.deleted_at IS NULL",
            params![project_id],
            |r| r.get(0),
        )
        .unwrap_or(0);

    let lore_count: i64 = conn
        .query_row(
            "SELECT COUNT(le.id)
             FROM lore_entries le
             JOIN world_settings ws ON le.world_setting_id = ws.id
             WHERE ws.project_id = ?1 AND le.deleted_at IS NULL",
            params![project_id],
            |r| r.get(0),
        )
        .unwrap_or(0);

    Ok(ProjectDetailsUiDto {
        word_count: word_count.to_string(),
        chapters: chapters_count.to_string(),
        characters: characters_count.to_string(),
        world_lore: (world_count + lore_count).to_string(),
        created_date: created_date.to_string(),
    })
}

fn fetch_project_arcs_hierarchy(conn: &Connection, project_id: &str) -> Result<Vec<ArcItemUiDto>, IpcError> {
    let mut arc_stmt = conn
        .prepare(
            "SELECT a.id, a.display_name, a.code
             FROM arcs a
             WHERE a.project_id = ?1 AND a.deleted_at IS NULL
             ORDER BY a.position ASC, a.id ASC",
        )
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

    let arc_rows = arc_stmt
        .query_map(params![project_id], |r| {
            let id: String = r.get(0)?;
            let display_name: Option<String> = r.get(1)?;
            let code: Option<String> = r.get(2)?;
            Ok((id, display_name.unwrap_or_else(|| code.unwrap_or_default())))
        })
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

    let mut arcs = Vec::new();
    for a in arc_rows {
        let (arc_id, arc_title) = a.map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

        let mut ch_stmt = conn
            .prepare(
                "SELECT c.id, c.display_name, c.code, c.created_at, c.updated_at
                 FROM chapters c
                 WHERE c.arc_id = ?1 AND c.deleted_at IS NULL
                 ORDER BY c.position ASC, c.id ASC",
            )
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

        let ch_rows = ch_stmt
            .query_map(params![arc_id], |r| {
                let id: String = r.get(0)?;
                let display_name: Option<String> = r.get(1)?;
                let code: Option<String> = r.get(2)?;
                let created_at: Option<String> = r.get(3)?;
                let updated_at: Option<String> = r.get(4)?;
                Ok((id, display_name.unwrap_or_else(|| code.unwrap_or_default()), created_at.or(updated_at).unwrap_or_default()))
            })
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

        let mut chapters = Vec::new();
        for ch in ch_rows {
            let (ch_id, ch_title, ch_date) = ch.map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

            let mut sc_stmt = conn
                .prepare(
                    "SELECT s.id, s.summary, s.content, s.is_locked
                     FROM scenes s
                     WHERE s.chapter_id = ?1 AND s.deleted_at IS NULL
                     ORDER BY s.position ASC, s.id ASC",
                )
                .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

            let sc_rows = sc_stmt
                .query_map(params![ch_id], |r| {
                    let id: String = r.get(0)?;
                    let summary: Option<String> = r.get(1)?;
                    let content: Option<String> = r.get(2)?;
                    let is_locked: Option<i64> = r.get(3)?;
                    let text = content.unwrap_or_else(|| summary.unwrap_or_default());
                    let locked = is_locked.map(|v| v == 1).unwrap_or(false);
                    Ok(ChapterBlockUiDto {
                        id: id.clone(),
                        scene_id: Some(id),
                        text,
                        locked,
                    })
                })
                .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

            let mut blocks = Vec::new();
            for sc in sc_rows {
                blocks.push(sc.map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?);
            }

            chapters.push(ChapterItemUiDto {
                id: ch_id,
                title: ch_title,
                date: ch_date,
                is_new: Some(false),
                blocks: Some(blocks),
            });
        }

        arcs.push(ArcItemUiDto {
            id: arc_id,
            title: arc_title,
            cover: None,
            chapters,
        });
    }

    Ok(arcs)
}
