//! Transactional draft commit repository (`TX-05`, `be-core.md` §7.2, §7.3).
//!
//! Enforces:
//! 1. Atomic insertion across 21 relational tables in exact topological order within `BEGIN IMMEDIATE`.
//! 2. Global uniqueness and pre-commit code collision verification across the entire database (`CodeCollisionOnCommit`).
//! 3. Written-scenes verification (`has_written_scenes`) before allowing draft reopening (`DraftReopenForbidden`).
//! 4. Automatic full rollback on any failure, restoring `draft_sessions.status = 'editing'` without leaking partial rows.

use chrono::Utc;
use rusqlite::{params, Connection, Transaction, TransactionBehavior};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::domain::business_flow::BusinessFlowData;

/// Namespace classification for resolving `code -> UUID` during commit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EntityNamespace {
    Project,
    Character,
    Arc,
    Chapter,
    Scene,
    WorldSetting,
    LoreEntry,
    Theme,
    ArcEventBlock,
    Relationship,
    Competence,
}

/// Information about a conflicting entity code detected during commit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConflictingCode {
    pub entity_type: String,
    pub conflicting_code: String,
}

/// Errors originating from commit repository operations.
#[derive(Debug, thiserror::Error)]
pub enum CommitRepoError {
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error("Code collision on commit")]
    CodeCollision(Vec<ConflictingCode>),

    #[error("Draft reopen forbidden: project has written scenes")]
    DraftReopenForbidden,

    #[error("Draft session not found: {0}")]
    NotFound(String),

    #[error("Entity not found: {0}")]
    EntityNotFound(String),

    #[error("Invalid state: {0}")]
    InvalidState(String),
}

/// Repository for draft commit and reopen operations.
pub struct SqliteCommitRepo<'a> {
    conn: &'a Connection,
}

impl<'a> SqliteCommitRepo<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    /// Checks whether the project has any written scenes (content non-null/non-empty
    /// or exists in `scene_versions`) per `be-core.md` §7.3.
    pub fn has_written_scenes(&self, project_id: &str) -> Result<bool, CommitRepoError> {
        has_written_scenes(self.conn, project_id)
    }

    /// Verifies whether any entity code in `state` collides with existing records in SQLite.
    pub fn check_code_collisions(&self, state: &BusinessFlowData) -> Result<Vec<ConflictingCode>, CommitRepoError> {
        check_code_collisions(self.conn, state)
    }
}

/// Queries whether a project has written scenes.
pub fn has_written_scenes(conn: &Connection, project_id: &str) -> Result<bool, CommitRepoError> {
    let sql = "SELECT
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
        ) AS has_written_scenes;";

    let mut stmt = conn.prepare(sql)?;
    let has_written: bool = stmt.query_row(params![project_id], |row| row.get(0))?;
    Ok(has_written)
}

/// Verifies whether any entity code in `state` collides with existing records in SQLite (`core.md` §3.7, §3.8).
pub fn check_code_collisions(conn: &Connection, state: &BusinessFlowData) -> Result<Vec<ConflictingCode>, CommitRepoError> {
    let mut conflicts = Vec::new();

    // 1. Project code
    if let Some(ref code) = state.require.project.code {
        let code_trimmed = code.trim();
        if !code_trimmed.is_empty() {
            let mut stmt = conn.prepare("SELECT 1 FROM projects WHERE code = ?1 AND deleted_at IS NULL LIMIT 1")?;
            if stmt.exists(params![code_trimmed])? {
                conflicts.push(ConflictingCode {
                    entity_type: "projects".to_string(),
                    conflicting_code: code_trimmed.to_string(),
                });
            }
        }
    }

    // 2. Characters (new characters only, exclude reused library characters)
    for ch in &state.require.character {
        if !ch.reused_library_character {
            let code = ch.code.trim();
            if !code.is_empty() {
                let mut stmt = conn.prepare("SELECT 1 FROM characters WHERE code = ?1 AND deleted_at IS NULL LIMIT 1")?;
                if stmt.exists(params![code])? {
                    conflicts.push(ConflictingCode {
                        entity_type: "characters".to_string(),
                        conflicting_code: code.to_string(),
                    });
                }
            }
        }
    }

    // 3. Arcs
    for arc in &state.optional.overview_phase.arcs {
        let code = arc.code.trim();
        if !code.is_empty() {
            let mut stmt = conn.prepare("SELECT 1 FROM arcs WHERE code = ?1 AND deleted_at IS NULL LIMIT 1")?;
            if stmt.exists(params![code])? {
                conflicts.push(ConflictingCode {
                    entity_type: "arcs".to_string(),
                    conflicting_code: code.to_string(),
                });
            }
        }
    }

    // 4. Chapters
    for arc in &state.optional.chapter_phase.arcs {
        for ch in &arc.chapters {
            let code = ch.code.trim();
            if !code.is_empty() {
                let mut stmt = conn.prepare("SELECT 1 FROM chapters WHERE code = ?1 AND deleted_at IS NULL LIMIT 1")?;
                if stmt.exists(params![code])? {
                    conflicts.push(ConflictingCode {
                        entity_type: "chapters".to_string(),
                        conflicting_code: code.to_string(),
                    });
                }
            }
        }
    }

    // 5. Scenes
    for arc in &state.optional.chapter_phase.arcs {
        for ch in &arc.chapters {
            for sc in &ch.scenes {
                let code = sc.code.trim();
                if !code.is_empty() {
                    let mut stmt = conn.prepare("SELECT 1 FROM scenes WHERE code = ?1 AND deleted_at IS NULL LIMIT 1")?;
                    if stmt.exists(params![code])? {
                        conflicts.push(ConflictingCode {
                            entity_type: "scenes".to_string(),
                            conflicting_code: code.to_string(),
                        });
                    }
                }
            }
        }
    }

    // 6. World settings
    for ws in &state.optional.world_building_phase.world_settings {
        let code = ws.code.trim();
        if !code.is_empty() {
            let mut stmt = conn.prepare("SELECT 1 FROM world_settings WHERE code = ?1 AND deleted_at IS NULL LIMIT 1")?;
            if stmt.exists(params![code])? {
                conflicts.push(ConflictingCode {
                    entity_type: "world_settings".to_string(),
                    conflicting_code: code.to_string(),
                });
            }
        }
    }

    // 7. Lore entries
    for ws in &state.optional.world_building_phase.world_settings {
        for lore in &ws.lore_entries {
            let code = lore.code.trim();
            if !code.is_empty() {
                let mut stmt = conn.prepare("SELECT 1 FROM lore_entries WHERE code = ?1 AND deleted_at IS NULL LIMIT 1")?;
                if stmt.exists(params![code])? {
                    conflicts.push(ConflictingCode {
                        entity_type: "lore_entries".to_string(),
                        conflicting_code: code.to_string(),
                    });
                }
            }
        }
    }

    // 8. Themes
    for arc in &state.optional.overview_phase.arcs {
        if let Some(ref code) = arc.themes.code {
            let code_trimmed = code.trim();
            if !code_trimmed.is_empty() {
                let mut stmt = conn.prepare("SELECT 1 FROM themes WHERE code = ?1 AND deleted_at IS NULL LIMIT 1")?;
                if stmt.exists(params![code_trimmed])? {
                    conflicts.push(ConflictingCode {
                        entity_type: "themes".to_string(),
                        conflicting_code: code_trimmed.to_string(),
                    });
                }
            }
        }
    }

    // 9. Character competences
    for char_item in &state.optional.character_life_experience_phase.character {
        for comp in &char_item.competences {
            let code = comp.code.trim();
            if !code.is_empty() {
                let mut stmt = conn.prepare("SELECT 1 FROM character_competences WHERE code = ?1 LIMIT 1")?;
                if stmt.exists(params![code])? {
                    conflicts.push(ConflictingCode {
                        entity_type: "character_competences".to_string(),
                        conflicting_code: code.to_string(),
                    });
                }
            }
        }
    }

    Ok(conflicts)
}

/// Helper to normalize setting_type to fit SQLite check constraint
/// `CHECK (type IN ('REALM', 'WORLD', 'REGION', 'COUNTRY', 'CITY', 'TOWN', 'FACILITY', 'STREET', 'ROOM'))`.
fn normalize_world_setting_type(raw: Option<&str>) -> &'static str {
    match raw.map(|s| s.trim().to_uppercase()).as_deref() {
        Some("REALM") => "REALM",
        Some("WORLD") => "WORLD",
        Some("REGION") | Some("CONTINENT") => "REGION",
        Some("COUNTRY") | Some("KINGDOM") | Some("EMPIRE") => "COUNTRY",
        Some("CITY") => "CITY",
        Some("TOWN") | Some("VILLAGE") => "TOWN",
        Some("FACILITY") | Some("SECT") | Some("SHRINE") | Some("TEMPLE") => "FACILITY",
        Some("STREET") => "STREET",
        Some("ROOM") | Some("CAVE") => "ROOM",
        _ => "FACILITY",
    }
}

/// Helper to normalize access to fit SQLite check constraint
/// `CHECK (access IS NULL OR access IN ('PUBLIC', 'PRIVATE', 'SECRET', 'LOCKED', 'GUARDED', 'RESTRICTED'))`.
fn normalize_world_setting_access(raw: Option<&str>) -> &'static str {
    match raw.map(|s| s.trim().to_uppercase()).as_deref() {
        Some("PUBLIC") | Some("OPEN") => "PUBLIC",
        Some("PRIVATE") => "PRIVATE",
        Some("SECRET") => "SECRET",
        Some("LOCKED") => "LOCKED",
        Some("GUARDED") => "GUARDED",
        Some("RESTRICTED") => "RESTRICTED",
        _ => "PUBLIC",
    }
}

/// Helper to normalize lore type to fit SQLite check constraint
/// `CHECK (type IN ('CULTURE', 'HISTORY', 'FACTION', 'ITEM'))`.
fn normalize_lore_type(raw: Option<&str>) -> &'static str {
    match raw.map(|s| s.trim().to_uppercase()).as_deref() {
        Some("CULTURE") | Some("SCRIPTURE") | Some("MAGIC_SYSTEM") => "CULTURE",
        Some("HISTORY") | Some("MYTHOLOGY") | Some("TIMELINE") => "HISTORY",
        Some("FACTION") | Some("ORGANIZATION") | Some("CLAN") => "FACTION",
        Some("ITEM") | Some("ARTIFACT") | Some("WEAPON") => "ITEM",
        _ => "CULTURE",
    }
}

/// Helper to normalize biological sex
/// `CHECK (biological_sex IS NULL OR biological_sex IN ('NONE', 'MALE', 'FEMALE'))`.
fn normalize_sex(raw: Option<&str>) -> Option<&'static str> {
    match raw.map(|s| s.trim().to_uppercase()).as_deref() {
        Some("MALE") => Some("MALE"),
        Some("FEMALE") => Some("FEMALE"),
        Some("NONE") => Some("NONE"),
        _ => None,
    }
}

/// Helper to normalize resolution type
/// `CHECK (resolution_type IS NULL OR resolution_type IN ('RESOLVED', 'OPEN', 'AMBIGUOUS'))`.
fn normalize_resolution_type(raw: Option<&str>) -> &'static str {
    match raw.map(|s| s.trim().to_uppercase()).as_deref() {
        Some("RESOLVED") | Some("TRIUMPH") | Some("BITTERSWEET") | Some("TRAGEDY") => "RESOLVED",
        Some("AMBIGUOUS") => "AMBIGUOUS",
        _ => "OPEN",
    }
}

/// Helper to normalize theme argument role
/// `CHECK (role IN ('PRIMARY', 'ANTITHESIS', 'CAUTIONARY', 'WITNESS'))`.
fn normalize_theme_role(raw: Option<&str>) -> &'static str {
    match raw.map(|s| s.trim().to_uppercase()).as_deref() {
        Some("PRIMARY") | Some("PROPONENT") => "PRIMARY",
        Some("ANTITHESIS") | Some("CHALLENGER") | Some("OPPONENT") => "ANTITHESIS",
        Some("CAUTIONARY") => "CAUTIONARY",
        Some("WITNESS") => "WITNESS",
        _ => "PRIMARY",
    }
}

/// Helper to normalize theme answer mode
/// `CHECK (answer_mode IS NULL OR answer_mode IN ('VERBAL', 'ACTION', 'OUTCOME', 'IMPLICIT'))`.
fn normalize_answer_mode(raw: Option<&str>) -> Option<&'static str> {
    match raw.map(|s| s.trim().to_uppercase()).as_deref() {
        Some("VERBAL") => Some("VERBAL"),
        Some("ACTION") | Some("SACRIFICE") => Some("ACTION"),
        Some("OUTCOME") => Some("OUTCOME"),
        Some("IMPLICIT") => Some("IMPLICIT"),
        _ => Some("ACTION"),
    }
}

/// Helper to normalize scene lore usage
/// `CHECK (usage IS NULL OR usage IN ('SETTING', 'REFERENCE', 'FORESHADOW', 'PAYOFF', 'REVEAL', 'CONFLICT', 'CONSTRAINT', 'RED_HERRING'))`.
fn normalize_scene_lore_usage(raw: Option<&str>) -> &'static str {
    match raw.map(|s| s.trim().to_uppercase()).as_deref() {
        Some("SETTING") => "SETTING",
        Some("REFERENCE") => "REFERENCE",
        Some("FORESHADOW") => "FORESHADOW",
        Some("PAYOFF") => "PAYOFF",
        Some("REVEAL") => "REVEAL",
        Some("CONFLICT") => "CONFLICT",
        Some("CONSTRAINT") => "CONSTRAINT",
        Some("RED_HERRING") => "RED_HERRING",
        _ => "REFERENCE",
    }
}

/// Helper to normalize reveal levels
/// `CHECK (reveal IS NULL OR reveal IN ('NONE', 'HINT', 'PARTIAL', 'FULL'))`.
fn normalize_reveal(raw: Option<&str>, default: &'static str) -> &'static str {
    match raw.map(|s| s.trim().to_uppercase()).as_deref() {
        Some("NONE") => "NONE",
        Some("HINT") => "HINT",
        Some("PARTIAL") => "PARTIAL",
        Some("FULL") => "FULL",
        _ => default,
    }
}

/// Helper to normalize relationship hierarchy
/// `CHECK (hierarchy IS NULL OR hierarchy IN ('SUPERIOR', 'EQUAL', 'INFERIOR'))`.
fn normalize_hierarchy(raw: Option<&str>) -> &'static str {
    match raw.map(|s| s.trim().to_uppercase()).as_deref() {
        Some("SUPERIOR") => "SUPERIOR",
        Some("INFERIOR") => "INFERIOR",
        _ => "EQUAL",
    }
}

/// Helper to normalize relation dimension levels
/// `CHECK (val IS NULL OR val IN ('NONE', 'LOW', 'MODERATE', 'HIGH', 'ABSOLUTE'))`.
fn normalize_dimension(raw: Option<&str>) -> Option<&'static str> {
    match raw.map(|s| s.trim().to_uppercase()).as_deref() {
        Some("NONE") => Some("NONE"),
        Some("LOW") => Some("LOW"),
        Some("MODERATE") => Some("MODERATE"),
        Some("HIGH") => Some("HIGH"),
        Some("ABSOLUTE") => Some("ABSOLUTE"),
        _ => None,
    }
}

/// Helper to ensure a genre exists in the DB so foreign key checks pass.
fn ensure_genre_id(tx: &Transaction<'_>, genre_code: &str, display_name: Option<&str>) -> Result<String, CommitRepoError> {
    let mut stmt = tx.prepare("SELECT id FROM genres WHERE code = ?1 LIMIT 1")?;
    let mut rows = stmt.query(params![genre_code])?;
    if let Some(row) = rows.next()? {
        let id: String = row.get(0)?;
        return Ok(id);
    }
    let new_id = Uuid::now_v7().to_string();
    tx.execute(
        "INSERT INTO genres (id, code, display_name, description) VALUES (?1, ?2, ?3, '')",
        params![new_id, genre_code, display_name.unwrap_or(genre_code)],
    )?;
    Ok(new_id)
}

/// Helper to ensure a motif exists in the DB so foreign key checks pass.
fn ensure_motif_id(tx: &Transaction<'_>, motif_code: &str, category: &str) -> Result<String, CommitRepoError> {
    let mut stmt = tx.prepare("SELECT id FROM motifs WHERE code = ?1 LIMIT 1")?;
    let mut rows = stmt.query(params![motif_code])?;
    if let Some(row) = rows.next()? {
        let id: String = row.get(0)?;
        return Ok(id);
    }
    let new_id = Uuid::now_v7().to_string();
    let norm_category = match category.to_lowercase().as_str() {
        "character" | "character_arc" => "character_arc",
        "relationship" => "relationship",
        "symbolic" => "symbolic",
        _ => "plot",
    };
    tx.execute(
        "INSERT INTO motifs (id, code, display_name, category, version, is_system, min_chapter, max_chapter)
         VALUES (?1, ?2, ?3, ?4, '1.0', 0, 1, 100)",
        params![new_id, motif_code, motif_code, norm_category],
    )?;
    Ok(new_id)
}

/// Helper to ensure an event block exists.
fn ensure_event_block_id(tx: &Transaction<'_>, code: &str) -> Result<String, CommitRepoError> {
    let mut stmt = tx.prepare("SELECT id FROM event_blocks WHERE code = ?1 LIMIT 1")?;
    let mut rows = stmt.query(params![code])?;
    if let Some(row) = rows.next()? {
        let id: String = row.get(0)?;
        return Ok(id);
    }
    let new_id = Uuid::now_v7().to_string();
    tx.execute(
        "INSERT INTO event_blocks (id, code, component_id, state_id, motif_id) VALUES (?1, ?2, NULL, NULL, NULL)",
        params![new_id, code],
    )?;
    Ok(new_id)
}

/// Helper to ensure style preset exists.
fn ensure_style_preset_id(tx: &Transaction<'_>, code: &str) -> Result<Option<String>, CommitRepoError> {
    if code.trim().is_empty() {
        return Ok(None);
    }
    let mut stmt = tx.prepare("SELECT id FROM style_presets WHERE code = ?1 LIMIT 1")?;
    let mut rows = stmt.query(params![code])?;
    if let Some(row) = rows.next()? {
        let id: String = row.get(0)?;
        return Ok(Some(id));
    }
    let new_id = Uuid::now_v7().to_string();
    tx.execute(
        "INSERT INTO style_presets (id, code, category) VALUES (?1, ?2, 'NARRATIVE')",
        params![new_id, code],
    )?;
    Ok(Some(new_id))
}

/// Helper to ensure narrator config exists.
fn ensure_narrator_config_id(tx: &Transaction<'_>, code: &str) -> Result<Option<String>, CommitRepoError> {
    if code.trim().is_empty() {
        return Ok(None);
    }
    let mut stmt = tx.prepare("SELECT id FROM narrator_configs WHERE code = ?1 LIMIT 1")?;
    let mut rows = stmt.query(params![code])?;
    if let Some(row) = rows.next()? {
        let id: String = row.get(0)?;
        return Ok(Some(id));
    }
    let new_id = Uuid::now_v7().to_string();
    let (person, subtype) = match code.to_uppercase().as_str() {
        "FIRST_PERSON" => ("FIRST_PERSON", "PROTAGONIST"),
        "SECOND_PERSON" => ("SECOND_PERSON", "DIRECT_ADDRESS"),
        "OMNISCIENT" => ("THIRD_PERSON", "OMNISCIENT"),
        "THIRD_PERSON_LIMITED" | _ => ("THIRD_PERSON", "LIMITED"),
    };
    tx.execute(
        "INSERT INTO narrator_configs (id, code, display_name, person, person_subtype) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![new_id, code, code, person, subtype],
    )?;
    Ok(Some(new_id))
}

/// Helper to ensure competence domain exists.
fn ensure_competence_domain_id(tx: &Transaction<'_>, code: &str) -> Result<String, CommitRepoError> {
    let mut stmt = tx.prepare("SELECT id FROM competence_domains WHERE code = ?1 LIMIT 1")?;
    let mut rows = stmt.query(params![code])?;
    if let Some(row) = rows.next()? {
        let id: String = row.get(0)?;
        return Ok(id);
    }
    let new_id = Uuid::now_v7().to_string();
    tx.execute(
        "INSERT INTO competence_domains (id, code, display_name) VALUES (?1, ?2, ?3)",
        params![new_id, code, code],
    )?;
    Ok(new_id)
}

/// Helper to ensure relationship category exists.
fn ensure_relationship_category_id(tx: &Transaction<'_>, code: &str) -> Result<Option<String>, CommitRepoError> {
    if code.trim().is_empty() {
        return Ok(None);
    }
    let mut stmt = tx.prepare("SELECT id FROM relationship_categories WHERE code = ?1 LIMIT 1")?;
    let mut rows = stmt.query(params![code])?;
    if let Some(row) = rows.next()? {
        let id: String = row.get(0)?;
        return Ok(Some(id));
    }
    let new_id = Uuid::now_v7().to_string();
    tx.execute(
        "INSERT INTO relationship_categories (id, code, category, sub_type) VALUES (?1, ?2, 'SOCIAL', ?2)",
        params![new_id, code],
    )?;
    Ok(Some(new_id))
}

/// Helper to find appearance option by code.
fn lookup_appearance_option_id(tx: &Transaction<'_>, code: &str) -> Result<Option<String>, CommitRepoError> {
    if code.trim().is_empty() {
        return Ok(None);
    }
    let mut stmt = tx.prepare("SELECT id FROM appearance_options WHERE code = ?1 LIMIT 1")?;
    let mut rows = stmt.query(params![code])?;
    if let Some(row) = rows.next()? {
        let id: String = row.get(0)?;
        return Ok(Some(id));
    }
    Ok(None)
}

/// Executes TX-05 atomic insertion into relational schema within `BEGIN IMMEDIATE`.
///
/// Inserts in topological foreign-key order:
/// 1. `projects`
/// 2. `project_genres`
/// 3. `characters`
/// 4. `project_characters`
/// 5. `character_motifs`
/// 6. `arcs`
/// 7. `arc_motifs`
/// 8. `themes`
/// 9. `theme_arguments`
/// 10. `chapters`
/// 11. `arc_event_blocks`
/// 12. `world_settings`
/// 13. `lore_entries`
/// 14. `scenes`
/// 15. `scene_characters`
/// 16. `scene_lore_entries`
/// 17. `character_appearances` & `character_distinguishing_marks`
/// 18. `character_psychological`
/// 19. `character_memories`
/// 20. `character_competences`, `competence_capabilities`, `competence_limitations`
/// 21. `relationships`, `relationship_roles`, `relationship_states`
/// 22. updates `draft_sessions` (`status = 'committed', project_id`) and `sessions` (`project_id`).
pub fn commit_draft_tx(
    conn: &mut Connection,
    draft_session_id: &Uuid,
    session_id: &str,
    state: &BusinessFlowData,
) -> Result<Uuid, CommitRepoError> {
    let tx = conn.transaction_with_behavior(TransactionBehavior::Immediate)?;

    // Step A: Recheck code collisions in transaction
    let collisions = check_code_collisions(&tx, state)?;
    if !collisions.is_empty() {
        return Err(CommitRepoError::CodeCollision(collisions));
    }

    let project_id = Uuid::now_v7();
    let project_id_str = project_id.to_string();
    let now = Utc::now().to_rfc3339();

    let mut code_to_id: HashMap<(EntityNamespace, String), String> = HashMap::new();

    // ------------------------------------------------------------------------
    // 1. projects
    // ------------------------------------------------------------------------
    let project_code = state
        .require
        .project
        .code
        .clone()
        .unwrap_or_else(|| format!("prj_{}", &project_id_str[..8]));
    let display_name = state
        .require
        .project
        .display_name
        .value
        .as_deref()
        .unwrap_or("Untitled Project");
    let summary = state.require.project.summary.value.as_deref().unwrap_or("");
    let language = state.require.project.language.value.as_deref().unwrap_or("VI");

    tx.execute(
        "INSERT INTO projects (id, code, display_name, summary, cover, status, language, created_at, updated_at, deleted_at)
         VALUES (?1, ?2, ?3, ?4, NULL, 'DRAFTING', ?5, ?6, ?7, NULL)",
        params![project_id_str, project_code, display_name, summary, language, now, now],
    )?;
    code_to_id.insert((EntityNamespace::Project, project_code.clone()), project_id_str.clone());

    // ------------------------------------------------------------------------
    // 2. project_genres
    // ------------------------------------------------------------------------
    for genre in &state.require.project.genres {
        if let Some(ref g_code) = genre.code.value {
            if !g_code.trim().is_empty() {
                let genre_id = ensure_genre_id(&tx, g_code, genre.display_name.value.as_deref())?;
                let link_id = Uuid::now_v7().to_string();
                let is_prim = if genre.is_primary.value == Some(true) { 1 } else { 0 };
                tx.execute(
                    "INSERT INTO project_genres (id, project_id, genre_id, is_primary) VALUES (?1, ?2, ?3, ?4)
                     ON CONFLICT (project_id, genre_id) DO NOTHING",
                    params![link_id, project_id_str, genre_id, is_prim],
                )?;
            }
        }
    }

    // ------------------------------------------------------------------------
    // 3. characters
    // ------------------------------------------------------------------------
    for ch in &state.require.character {
        let ch_code = ch.code.trim();
        if ch.reused_library_character {
            let mut stmt = tx.prepare("SELECT id FROM characters WHERE code = ?1 AND deleted_at IS NULL LIMIT 1")?;
            let mut rows = stmt.query(params![ch_code])?;
            if let Some(row) = rows.next()? {
                let existing_id: String = row.get(0)?;
                code_to_id.insert((EntityNamespace::Character, ch_code.to_string()), existing_id);
            } else {
                return Err(CommitRepoError::EntityNotFound(format!(
                    "Reused library character code '{}' not found in database",
                    ch_code
                )));
            }
        } else {
            let char_id = Uuid::now_v7().to_string();
            // Lookup appearance data for gender & species
            let appearance_match = state
                .optional
                .character_appearance_phase
                .character
                .iter()
                .find(|app| app.code == ch_code);

            let species = appearance_match.and_then(|app| app.species.value.clone());
            let birth = appearance_match.and_then(|app| app.birth.value.clone());
            let gender_identity = appearance_match.and_then(|app| normalize_sex(app.gender.identity.value.as_deref()));
            let gender_expression = appearance_match.and_then(|app| normalize_sex(app.gender.expression.value.as_deref()));
            let biological_sex = normalize_sex(ch.biological_sex.value.as_deref());

            let display_name = ch.display_name.value.as_deref();
            let nickname = ch.nickname.value.as_deref();
            // Invariant: display_name IS NOT NULL OR nickname IS NOT NULL
            let final_display_name = if display_name.is_none() && nickname.is_none() {
                Some(ch_code)
            } else {
                display_name
            };

            tx.execute(
                "INSERT INTO characters (id, code, display_name, nickname, birth, species, biological_sex, gender_identity, gender_expression, deleted_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, NULL)",
                params![char_id, ch_code, final_display_name, nickname, birth, species, biological_sex, gender_identity, gender_expression],
            )?;
            code_to_id.insert((EntityNamespace::Character, ch_code.to_string()), char_id);
        }
    }

    // ------------------------------------------------------------------------
    // 4. project_characters
    // ------------------------------------------------------------------------
    for ch in &state.require.character {
        let ch_code = ch.code.trim();
        if let Some(char_id) = code_to_id.get(&(EntityNamespace::Character, ch_code.to_string())) {
            let link_id = Uuid::now_v7().to_string();
            let role = ch.role.value.as_deref();
            let notes = ch.notes.value.as_deref();
            tx.execute(
                "INSERT INTO project_characters (id, project_id, character_id, role, notes, created_at, deleted_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, NULL)
                 ON CONFLICT (project_id, character_id) DO NOTHING",
                params![link_id, project_id_str, char_id, role, notes, now],
            )?;
        }
    }

    // ------------------------------------------------------------------------
    // 5. character_motifs
    // ------------------------------------------------------------------------
    for ch in &state.require.character {
        let ch_code = ch.code.trim();
        if let Some(char_id) = code_to_id.get(&(EntityNamespace::Character, ch_code.to_string())) {
            if let Some(ref m_code) = ch.motif_code.value {
                if !m_code.trim().is_empty() {
                    let motif_id = ensure_motif_id(&tx, m_code, "CHARACTER")?;
                    let link_id = Uuid::now_v7().to_string();
                    tx.execute(
                        "INSERT INTO character_motifs (id, motif_id, character_id, description)
                         VALUES (?1, ?2, ?3, NULL)
                         ON CONFLICT (character_id, motif_id) DO NOTHING",
                        params![link_id, motif_id, char_id],
                    )?;
                }
            }
        }
    }

    // ------------------------------------------------------------------------
    // 6. arcs
    // ------------------------------------------------------------------------
    let mut sorted_arcs = state.optional.overview_phase.arcs.clone();
    sorted_arcs.sort_by_key(|a| a.position.value.unwrap_or(0));

    for (idx, arc) in sorted_arcs.iter().enumerate() {
        let arc_code = arc.code.trim();
        let arc_id = Uuid::now_v7().to_string();
        let pos = arc.position.value.unwrap_or((idx + 1) as u32);
        let disp = arc.display_name.value.as_deref();
        let sum = arc.summary.value.as_deref();

        tx.execute(
            "INSERT INTO arcs (id, code, display_name, project_id, position, summary, deleted_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, NULL)",
            params![arc_id, arc_code, disp, project_id_str, pos, sum],
        )?;
        code_to_id.insert((EntityNamespace::Arc, arc_code.to_string()), arc_id);
    }

    // ------------------------------------------------------------------------
    // 7. arc_motifs
    // ------------------------------------------------------------------------
    let mut arc_motif_ids: HashMap<String, String> = HashMap::new();
    for arc in &sorted_arcs {
        let arc_code = arc.code.trim();
        if let Some(arc_id) = code_to_id.get(&(EntityNamespace::Arc, arc_code.to_string())) {
            if let Some(ref m_code) = arc.motif.code.value {
                if !m_code.trim().is_empty() {
                    let motif_id = ensure_motif_id(&tx, m_code, "ARC")?;
                    let arc_motif_id = Uuid::now_v7().to_string();
                    let note = arc.motif.note.value.as_deref();
                    tx.execute(
                        "INSERT INTO arc_motifs (id, arc_id, motif_id, note)
                         VALUES (?1, ?2, ?3, ?4)
                         ON CONFLICT (arc_id) DO NOTHING",
                        params![arc_motif_id, arc_id, motif_id, note],
                    )?;
                    arc_motif_ids.insert(arc_code.to_string(), arc_motif_id);
                }
            }
        }
    }

    // ------------------------------------------------------------------------
    // 8. themes
    // ------------------------------------------------------------------------
    for arc in &sorted_arcs {
        let arc_code = arc.code.trim();
        let arc_id = code_to_id.get(&(EntityNamespace::Arc, arc_code.to_string())).cloned();

        if let Some(ref t_code) = arc.themes.code {
            let theme_code = t_code.trim();
            if !theme_code.is_empty() {
                let theme_id = Uuid::now_v7().to_string();
                let parent_code = arc.themes.parent_theme_code.value.as_deref().unwrap_or("").trim();
                let parent_theme_id = if !parent_code.is_empty() {
                    code_to_id.get(&(EntityNamespace::Theme, parent_code.to_string())).cloned()
                } else {
                    None
                };

                let question = arc.themes.question.value.as_deref().unwrap_or("What is the core conflict?");
                let res_type = normalize_resolution_type(arc.themes.resolution.resolution_type.value.as_deref());
                let res_content = arc.themes.resolution.content.value.as_deref();
                let message = arc.themes.message.value.as_deref();

                tx.execute(
                    "INSERT INTO themes (id, code, arc_id, parent_theme_id, question, resolution_type, resolution, message, created_at, updated_at, deleted_at)
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, NULL)",
                    params![theme_id, theme_code, arc_id, parent_theme_id, question, res_type, res_content, message, now, now],
                )?;
                code_to_id.insert((EntityNamespace::Theme, theme_code.to_string()), theme_id.clone());

                // --------------------------------------------------------------------
                // 9. theme_arguments
                // --------------------------------------------------------------------
                for arg in &arc.themes.arguments {
                    if let Some(ref char_code) = arg.character_code.value {
                        if let Some(char_id) = code_to_id.get(&(EntityNamespace::Character, char_code.trim().to_string())) {
                            let arg_id = Uuid::now_v7().to_string();
                            let role = normalize_theme_role(arg.role.value.as_deref());
                            let answer_mode = normalize_answer_mode(arg.answer_mode.value.as_deref());
                            let note = arg.note.value.as_deref();
                            let req_know = arg.required_knowledge.value.as_deref();
                            let req_comp = arg.required_competence.value.as_deref();

                            tx.execute(
                                "INSERT INTO theme_arguments (id, theme_id, character_id, role, answer_mode, note, required_knowledge, required_competence)
                                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
                                 ON CONFLICT (theme_id, character_id) DO NOTHING",
                                params![arg_id, theme_id, char_id, role, answer_mode, note, req_know, req_comp],
                            )?;
                        }
                    }
                }
            }
        }
    }

    // ------------------------------------------------------------------------
    // 10. chapters
    // ------------------------------------------------------------------------
    for arc in &state.optional.chapter_phase.arcs {
        let arc_code = arc.code.trim();
        let arc_id = code_to_id.get(&(EntityNamespace::Arc, arc_code.to_string())).cloned();

        let mut sorted_chapters = arc.chapters.clone();
        sorted_chapters.sort_by_key(|c| c.position.value.unwrap_or(0));

        for (idx, ch) in sorted_chapters.iter().enumerate() {
            let ch_code = ch.code.trim();
            let chapter_id = Uuid::now_v7().to_string();
            let pos = ch.position.value.unwrap_or((idx + 1) as u32);
            let disp = ch.display_name.value.as_deref();
            let sum = ch.summary.value.as_deref();

            tx.execute(
                "INSERT INTO chapters (id, code, display_name, arc_id, position, summary, created_at, updated_at, deleted_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, NULL)",
                params![chapter_id, ch_code, disp, arc_id, pos, sum, now, now],
            )?;
            code_to_id.insert((EntityNamespace::Chapter, ch_code.to_string()), chapter_id);
        }
    }

    // ------------------------------------------------------------------------
    // 11. arc_event_blocks
    // ------------------------------------------------------------------------
    for arc in &sorted_arcs {
        let arc_code = arc.code.trim();
        if let Some(arc_motif_id) = arc_motif_ids.get(arc_code) {
            for eb in &arc.motif.event_blocks {
                let eb_code = eb.event_block_code.value.as_deref().unwrap_or("").trim();
                let start_ch_code = eb.start_chapter_code.value.as_deref().unwrap_or("").trim();
                let end_ch_code = eb.end_chapter_code.value.as_deref().unwrap_or("").trim();

                let start_ch_id = code_to_id.get(&(EntityNamespace::Chapter, start_ch_code.to_string())).cloned();
                let end_ch_id = code_to_id.get(&(EntityNamespace::Chapter, end_ch_code.to_string())).cloned();

                if let (Some(start_id), Some(end_id)) = (start_ch_id, end_ch_id) {
                    if !eb_code.is_empty() {
                        let eb_id = ensure_event_block_id(&tx, eb_code)?;
                        let link_id = Uuid::now_v7().to_string();
                        let sum = eb.summary.value.as_deref();
                        tx.execute(
                            "INSERT INTO arc_event_blocks (id, arc_motif_id, event_block_id, start_chapter_id, end_chapter_id, summary)
                             VALUES (?1, ?2, ?3, ?4, ?5, ?6)
                             ON CONFLICT (arc_motif_id, event_block_id) DO NOTHING",
                            params![link_id, arc_motif_id, eb_id, start_id, end_id, sum],
                        )?;
                    }
                }
            }
        }
    }

    // ------------------------------------------------------------------------
    // 12. world_settings (Topological BFS order: parents before children)
    // ------------------------------------------------------------------------
    let mut remaining_ws = state.optional.world_building_phase.world_settings.clone();
    let mut inserted_ws = std::collections::HashSet::new();

    while !remaining_ws.is_empty() {
        let mut progress = false;
        let mut next_remaining = Vec::new();

        for ws in remaining_ws {
            let parent_code = ws.parent_code.value.as_deref().unwrap_or("").trim();
            if parent_code.is_empty() || inserted_ws.contains(parent_code) {
                let ws_code = ws.code.trim();
                let ws_id = Uuid::now_v7().to_string();
                let parent_id = if !parent_code.is_empty() {
                    code_to_id.get(&(EntityNamespace::WorldSetting, parent_code.to_string())).cloned()
                } else {
                    None
                };

                let disp = ws.display_name.value.as_deref().unwrap_or(ws_code);
                let st_type = normalize_world_setting_type(ws.setting_type.value.as_deref());
                let access = normalize_world_setting_access(ws.access.value.as_deref());
                let desc = ws.description.value.as_deref();
                let spatial = ws.spatial_features.value.as_deref();
                let vis = ws.sensory_visual.value.as_deref();
                let aud = ws.sensory_auditory.value.as_deref();
                let tac = ws.sensory_tactile.value.as_deref();
                let olf = ws.sensory_olfactory.value.as_deref();
                let gus = ws.sensory_gustatory.value.as_deref();

                tx.execute(
                    "INSERT INTO world_settings (id, code, project_id, display_name, type, parent_id, description, access,
                                                 spatial_features, sensory_visual, sensory_auditory, sensory_tactile, sensory_olfactory, sensory_gustatory,
                                                 created_at, updated_at, deleted_at)
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, NULL)",
                    params![ws_id, ws_code, project_id_str, disp, st_type, parent_id, desc, access, spatial, vis, aud, tac, olf, gus, now, now],
                )?;
                code_to_id.insert((EntityNamespace::WorldSetting, ws_code.to_string()), ws_id);
                inserted_ws.insert(ws_code.to_string());
                progress = true;
            } else {
                next_remaining.push(ws);
            }
        }

        if !progress {
            // Cycle fallback: insert remaining without parent to break circularity
            for ws in next_remaining {
                let ws_code = ws.code.trim();
                let ws_id = Uuid::now_v7().to_string();
                let disp = ws.display_name.value.as_deref().unwrap_or(ws_code);
                let st_type = normalize_world_setting_type(ws.setting_type.value.as_deref());
                let access = normalize_world_setting_access(ws.access.value.as_deref());
                let desc = ws.description.value.as_deref();

                tx.execute(
                    "INSERT INTO world_settings (id, code, project_id, display_name, type, parent_id, description, access, created_at, updated_at, deleted_at)
                     VALUES (?1, ?2, ?3, ?4, ?5, NULL, ?6, ?7, ?8, ?9, NULL)",
                    params![ws_id, ws_code, project_id_str, disp, st_type, desc, access, now, now],
                )?;
                code_to_id.insert((EntityNamespace::WorldSetting, ws_code.to_string()), ws_id);
            }
            break;
        }
        remaining_ws = next_remaining;
    }

    // ------------------------------------------------------------------------
    // 13. lore_entries
    // ------------------------------------------------------------------------
    for ws in &state.optional.world_building_phase.world_settings {
        let ws_code = ws.code.trim();
        let ws_id_opt = code_to_id.get(&(EntityNamespace::WorldSetting, ws_code.to_string())).cloned();
        if let Some(ws_id) = ws_id_opt {
            for lore in &ws.lore_entries {
                let lore_code = lore.code.trim();
                let lore_id = Uuid::now_v7().to_string();
                let l_type = normalize_lore_type(lore.lore_type.value.as_deref());
                let disp = lore.display_name.value.as_deref().unwrap_or(lore_code);
                let desc = lore.description.value.as_deref();
                let data_str = lore
                    .data
                    .value
                    .as_ref()
                    .and_then(|v| serde_json::to_string(v).ok())
                    .unwrap_or_else(|| "{}".to_string());

                tx.execute(
                    "INSERT INTO lore_entries (id, code, type, display_name, description, world_setting_id, data, deleted_at)
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, NULL)",
                    params![lore_id, lore_code, l_type, disp, desc, ws_id, data_str],
                )?;
                code_to_id.insert((EntityNamespace::LoreEntry, lore_code.to_string()), lore_id);
            }
        }
    }

    // Collect scene configs for fast lookup
    let mut scene_configs = HashMap::new();
    for arc in &state.optional.scene_config_phase.arcs {
        for ch in &arc.chapters {
            for sc in &ch.scenes {
                scene_configs.insert(sc.code.trim().to_string(), sc.clone());
            }
        }
    }

    // ------------------------------------------------------------------------
    // 14. scenes (content = NULL, word_count = 0, is_locked = 0)
    // ------------------------------------------------------------------------
    for arc in &state.optional.chapter_phase.arcs {
        for ch in &arc.chapters {
            let ch_code = ch.code.trim();
            let chapter_id = code_to_id.get(&(EntityNamespace::Chapter, ch_code.to_string())).cloned();

            let mut sorted_scenes = ch.scenes.clone();
            sorted_scenes.sort_by_key(|s| s.position.value.unwrap_or(0));

            for (idx, sc) in sorted_scenes.iter().enumerate() {
                let sc_code = sc.code.trim();
                let scene_id = Uuid::now_v7().to_string();
                let pos = sc.position.value.unwrap_or((idx + 1) as u32);
                let disp = sc.display_name.value.as_deref();
                let sum = sc.summary.value.as_deref();

                // Look up world_setting_id (required foreign key)
                let ws_code = scene_configs
                    .get(sc_code)
                    .and_then(|cfg| cfg.world_setting_code.value.as_deref())
                    .or(sc.world_setting_code.value.as_deref())
                    .unwrap_or("");
                let ws_id = code_to_id
                    .get(&(EntityNamespace::WorldSetting, ws_code.trim().to_string()))
                    .cloned()
                    .ok_or_else(|| {
                        CommitRepoError::EntityNotFound(format!(
                            "Scene '{}' references missing world setting '{}'",
                            sc_code, ws_code
                        ))
                    })?;

                let style_preset_code = scene_configs
                    .get(sc_code)
                    .and_then(|cfg| cfg.style_preset_code.value.as_deref())
                    .unwrap_or("");
                let style_preset_id = ensure_style_preset_id(&tx, style_preset_code)?;

                let narrator_code = scene_configs
                    .get(sc_code)
                    .and_then(|cfg| cfg.narrator_config_code.value.as_deref())
                    .unwrap_or("");
                let narrator_config_id = ensure_narrator_config_id(&tx, narrator_code)?;

                tx.execute(
                    "INSERT INTO scenes (id, code, display_name, chapter_id, world_setting_id, style_preset_id, narrator_config_id,
                                         summary, content, word_count, position, is_locked, deleted_at)
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, NULL, 0, ?9, 0, NULL)",
                    params![scene_id, sc_code, disp, chapter_id, ws_id, style_preset_id, narrator_config_id, sum, pos],
                )?;
                code_to_id.insert((EntityNamespace::Scene, sc_code.to_string()), scene_id.clone());

                // ------------------------------------------------------------
                // 15. scene_characters & 16. scene_lore_entries
                // ------------------------------------------------------------
                if let Some(cfg) = scene_configs.get(sc_code) {
                    for char_link in &cfg.characters {
                        let c_code = char_link.character_code.trim();
                        if let Some(char_id) = code_to_id.get(&(EntityNamespace::Character, c_code.to_string())) {
                            let link_id = Uuid::now_v7().to_string();
                            let is_pov = if char_link.is_pov.value == Some(true) { 1 } else { 0 };
                            tx.execute(
                                "INSERT INTO scene_characters (id, character_id, scene_id, is_pov)
                                 VALUES (?1, ?2, ?3, ?4)
                                 ON CONFLICT (scene_id, character_id) DO NOTHING",
                                params![link_id, char_id, scene_id, is_pov],
                            )?;
                        }
                    }

                    for lore_link in &cfg.lore_entries {
                        let l_code = lore_link.lore_entry_code.trim();
                        if let Some(lore_id) = code_to_id.get(&(EntityNamespace::LoreEntry, l_code.to_string())) {
                            let link_id = Uuid::now_v7().to_string();
                            let usage = normalize_scene_lore_usage(lore_link.usage.value.as_deref());
                            let reveal_r = normalize_reveal(lore_link.reveal_to_reader.value.as_deref(), "PARTIAL");
                            let reveal_p = normalize_reveal(lore_link.reveal_to_pov.value.as_deref(), "NONE");
                            tx.execute(
                                "INSERT INTO scene_lore_entries (id, scene_id, lore_entry_id, usage, reveal_to_reader, reveal_to_pov)
                                 VALUES (?1, ?2, ?3, ?4, ?5, ?6)
                                 ON CONFLICT (scene_id, lore_entry_id) DO NOTHING",
                                params![link_id, scene_id, lore_id, usage, reveal_r, reveal_p],
                            )?;
                        }
                    }
                }
            }
        }
    }

    // ------------------------------------------------------------------------
    // 17. character_appearances & character_distinguishing_marks
    // ------------------------------------------------------------------------
    for app in &state.optional.character_appearance_phase.character {
        let ch_code = app.code.trim();
        if let Some(char_id) = code_to_id.get(&(EntityNamespace::Character, ch_code.to_string())) {
            let app_id = Uuid::now_v7().to_string();
            let height = app.height_cm.value.map(|h| h as i64);
            let height_unit = app.height_unit.value.as_deref().unwrap_or("CM");
            let weight = app.weight_kg.value.map(|w| w as i64);
            let weight_unit = app.weight_unit.value.as_deref().unwrap_or("KG");

            let hair_c = app.hair.color.value.as_deref().and_then(|c| lookup_appearance_option_id(&tx, c).ok().flatten());
            let hair_l = app.hair.length.value.as_deref().and_then(|c| lookup_appearance_option_id(&tx, c).ok().flatten());
            let hair_s = app.hair.style.value.as_deref().and_then(|c| lookup_appearance_option_id(&tx, c).ok().flatten());
            let hair_t = app.hair.texture.value.as_deref().and_then(|c| lookup_appearance_option_id(&tx, c).ok().flatten());

            let eye_p = app.eyes.pupil_shape.value.as_deref().and_then(|c| lookup_appearance_option_id(&tx, c).ok().flatten());
            let eye_i = app.eyes.iris_color.value.as_deref().and_then(|c| lookup_appearance_option_id(&tx, c).ok().flatten());
            let eye_s = app.eyes.sclera_color.value.as_deref().and_then(|c| lookup_appearance_option_id(&tx, c).ok().flatten());

            let skin_t = app.skin.tone.value.as_deref().and_then(|c| lookup_appearance_option_id(&tx, c).ok().flatten());
            let skin_tx = app.skin.texture.value.as_deref().and_then(|c| lookup_appearance_option_id(&tx, c).ok().flatten());

            let hair_note = app.hair.note.value.as_deref();
            let eyes_note = app.eyes.note.value.as_deref();
            let skin_note = app.skin.note.value.as_deref();
            let outfit = app.outfit_style.value.as_deref();
            let def_sum = app.default_summary.value.as_deref();

            tx.execute(
                "INSERT INTO character_appearances (id, character_id, height, height_unit, weight, weight_unit,
                                                   hair_color_id, hair_length_id, hair_style_id, hair_texture_id,
                                                   pupil_shape_id, iris_color_id, sclera_color_id, skin_tone_id, skin_texture_id,
                                                   hair_note, eyes_note, skin_note, outfit_style, default_summary)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20)
                 ON CONFLICT (character_id) DO NOTHING",
                params![app_id, char_id, height, height_unit, weight, weight_unit,
                        hair_c, hair_l, hair_s, hair_t, eye_p, eye_i, eye_s, skin_t, skin_tx,
                        hair_note, eyes_note, skin_note, outfit, def_sum],
            )?;

            for mark in &app.distinguishing_marks {
                let mark_id = Uuid::now_v7().to_string();
                let loc = mark.location.value.as_deref();
                let content = mark.content.value.as_deref().unwrap_or("");
                tx.execute(
                    "INSERT INTO character_distinguishing_marks (id, character_id, location, content)
                     VALUES (?1, ?2, ?3, ?4)",
                    params![mark_id, char_id, loc, content],
                )?;
            }
        }
    }

    // ------------------------------------------------------------------------
    // 18. character_psychological
    // ------------------------------------------------------------------------
    for psych in &state.optional.character_psychological_phase.character {
        let ch_code = psych.code.trim();
        if let Some(char_id) = code_to_id.get(&(EntityNamespace::Character, ch_code.to_string())) {
            let the_wound = psych.core_trauma_and_belief.the_wound.value.as_deref();
            let the_lie = psych.core_trauma_and_belief.the_lie.value.as_deref();
            let fear = psych.core_trauma_and_belief.fundamental_fear.value.as_deref();
            let alignment = psych.cognitive_moral_framework.moral_alignment.value.as_deref();
            let code_m = psych.cognitive_moral_framework.moral_code.value.as_deref();
            let bias = psych.cognitive_moral_framework.cognitive_bias.value.as_deref();
            let breaking = psych.cognitive_moral_framework.breaking_point.value.as_deref();
            let want = psych.narrative_engine.external_want.value.as_deref();
            let need = psych.narrative_engine.internal_need.value.as_deref();
            let conflict = psych.narrative_engine.internal_conflict.value.as_deref();
            let persona = psych.behavioral_adaptation.social_mask.persona_type.value.as_deref();
            let style = psych.behavioral_adaptation.social_mask.verbal_style.value.as_deref();
            let reaction = psych.behavioral_adaptation.defense_mechanisms.primary_reaction.value.as_deref();
            let trigger = psych.behavioral_adaptation.defense_mechanisms.stress_trigger.value.as_deref();
            let strategy = psych.behavioral_adaptation.defense_mechanisms.coping_strategy.value.as_deref();

            tx.execute(
                "INSERT INTO character_psychological (character_id, the_wound, the_lie, fundamental_fear, moral_alignment,
                                                      moral_code, cognitive_bias, breaking_point, external_want, internal_need,
                                                      internal_conflict, persona_type, verbal_style, primary_reaction, stress_trigger, coping_strategy)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16)
                 ON CONFLICT (character_id) DO NOTHING",
                params![char_id, the_wound, the_lie, fear, alignment, code_m, bias, breaking, want, need, conflict, persona, style, reaction, trigger, strategy],
            )?;
        }
    }

    // ------------------------------------------------------------------------
    // 19. character_memories
    // ------------------------------------------------------------------------
    for exp in &state.optional.character_life_experience_phase.character {
        let ch_code = exp.code.trim();
        if let Some(char_id) = code_to_id.get(&(EntityNamespace::Character, ch_code.to_string())) {
            for mem in &exp.memories {
                let mem_id = Uuid::now_v7().to_string();
                let source_scene_code = mem.source_scene_code.value.as_deref().unwrap_or("").trim();
                let source_scene_id = if !source_scene_code.is_empty() {
                    code_to_id.get(&(EntityNamespace::Scene, source_scene_code.to_string())).cloned()
                } else {
                    None
                };
                let content = mem.content.value.as_deref().unwrap_or("");

                tx.execute(
                    "INSERT INTO character_memories (id, character_id, source_scene_id, content, created_at)
                     VALUES (?1, ?2, ?3, ?4, ?5)",
                    params![mem_id, char_id, source_scene_id, content, now],
                )?;
            }
        }
    }

    // ------------------------------------------------------------------------
    // 20. character_competences, competence_capabilities, competence_limitations
    // ------------------------------------------------------------------------
    for exp in &state.optional.character_life_experience_phase.character {
        let ch_code = exp.code.trim();
        let char_id_opt = code_to_id.get(&(EntityNamespace::Character, ch_code.to_string())).cloned();
        if let Some(char_id) = char_id_opt {
            for comp in &exp.competences {
                let comp_code = comp.code.trim();
                let comp_id = Uuid::now_v7().to_string();

                let d_code = comp.domain_code.value.as_deref().unwrap_or("GENERAL").trim();
                let domain_id = ensure_competence_domain_id(&tx, d_code)?;

                let sc_code = comp.scene_code.value.as_deref().unwrap_or("").trim();
                let scene_id = code_to_id
                    .get(&(EntityNamespace::Scene, sc_code.to_string()))
                    .cloned()
                    .ok_or_else(|| {
                        CommitRepoError::EntityNotFound(format!(
                            "Competence '{}' references missing scene '{}'",
                            comp_code, sc_code
                        ))
                    })?;

                let disp = comp.display_name.value.as_deref().unwrap_or(comp_code);
                let desc = comp.description.value.as_deref();
                let rel = comp.reliability.value.as_deref();
                let exp_s = comp.experience.value.as_deref();

                tx.execute(
                    "INSERT INTO character_competences (id, code, character_id, domain_id, level_id, scene_id, display_name, description, reliability, experience)
                     VALUES (?1, ?2, ?3, ?4, NULL, ?5, ?6, ?7, ?8, ?9)",
                    params![comp_id, comp_code, char_id, domain_id, scene_id, disp, desc, rel, exp_s],
                )?;
                code_to_id.insert((EntityNamespace::Competence, comp_code.to_string()), comp_id.clone());

                for cap in &comp.capabilities {
                    if let Some(ref c) = cap.value {
                        let cap_id = Uuid::now_v7().to_string();
                        tx.execute(
                            "INSERT INTO competence_capabilities (id, competence_id, content) VALUES (?1, ?2, ?3)",
                            params![cap_id, comp_id, c],
                        )?;
                    }
                }

                for lim in &comp.limitations {
                    if let Some(ref l) = lim.value {
                        let lim_id = Uuid::now_v7().to_string();
                        tx.execute(
                            "INSERT INTO competence_limitations (id, competence_id, content) VALUES (?1, ?2, ?3)",
                            params![lim_id, comp_id, l],
                        )?;
                    }
                }
            }
        }
    }

    // ------------------------------------------------------------------------
    // 21. relationships, relationship_roles, relationship_states
    // ------------------------------------------------------------------------
    for rel in &state.optional.relationship_phase.relationship {
        let from_code = rel.character.from_code.trim();
        let to_code = rel.character.to_code.trim();

        let from_id = code_to_id.get(&(EntityNamespace::Character, from_code.to_string())).cloned();
        let to_id = code_to_id.get(&(EntityNamespace::Character, to_code.to_string())).cloned();

        if let (Some(f_id), Some(t_id)) = (from_id, to_id) {
            let rel_id = Uuid::now_v7().to_string();
            let motif_id = if let Some(ref m_code) = rel.motif.code.value {
                if !m_code.trim().is_empty() {
                    Some(ensure_motif_id(&tx, m_code, "RELATIONSHIP")?)
                } else {
                    None
                }
            } else {
                None
            };

            tx.execute(
                "INSERT INTO relationships (id, motif_id, from_character_id, to_character_id)
                 VALUES (?1, ?2, ?3, ?4)
                 ON CONFLICT (from_character_id, to_character_id) DO NOTHING",
                params![rel_id, motif_id, f_id, t_id],
            )?;

            for role in &rel.character.roles {
                let role_id = Uuid::now_v7().to_string();
                let hier = normalize_hierarchy(role.hierarchy.value.as_deref());
                let cat_code = role.category_code.value.as_deref().unwrap_or("").trim();
                let cat_id = ensure_relationship_category_id(&tx, cat_code)?;
                let is_prim = if role.is_primary.value == Some(true) { 1 } else { 0 };

                tx.execute(
                    "INSERT INTO relationship_roles (id, hierarchy, category_id, relationship_id, is_primary)
                     VALUES (?1, ?2, ?3, ?4, ?5)",
                    params![role_id, hier, cat_id, rel_id, is_prim],
                )?;
            }

            for r_state in &rel.character.scenes {
                let sc_code = r_state.scene_code.trim();
                if let Some(scene_id) = code_to_id.get(&(EntityNamespace::Scene, sc_code.to_string())) {
                    let st_id = Uuid::now_v7().to_string();
                    let trust = normalize_dimension(r_state.trust.value.as_deref());
                    let affection = normalize_dimension(r_state.affection.value.as_deref());
                    let hostility = normalize_dimension(r_state.hostility.value.as_deref());
                    let fear = normalize_dimension(r_state.fear.value.as_deref());
                    let respect = normalize_dimension(r_state.respect.value.as_deref());
                    let familiarity = normalize_dimension(r_state.familiarity.value.as_deref());
                    let dependency = normalize_dimension(r_state.dependency.value.as_deref());

                    tx.execute(
                        "INSERT INTO relationship_states (id, relationship_id, scene_id, trust, affection, hostility, fear, respect, familiarity, dependency, created_at)
                         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
                        params![st_id, rel_id, scene_id, trust, affection, hostility, fear, respect, familiarity, dependency, now],
                    )?;
                }
            }
        }
    }

    // ------------------------------------------------------------------------
    // 22. draft_sessions & sessions update
    // ------------------------------------------------------------------------
    tx.execute(
        "UPDATE draft_sessions SET project_id = ?1, status = 'committed', updated_at = ?2 WHERE id = ?3",
        params![project_id_str, now, draft_session_id.to_string()],
    )?;
    tx.execute(
        "UPDATE sessions SET project_id = ?1 WHERE id = ?2",
        params![project_id_str, session_id],
    )?;

    tx.commit()?;
    Ok(project_id)
}

/// Reopens a committed draft if and only if no scene content has been written (`be-core.md` §7.3).
pub fn reopen_committed_draft(
    conn: &mut Connection,
    project_id: &Uuid,
    active_session_id: &str,
) -> Result<Uuid, CommitRepoError> {
    let project_id_str = project_id.to_string();

    // 1. Verify written scenes
    if has_written_scenes(conn, &project_id_str)? {
        return Err(CommitRepoError::DraftReopenForbidden);
    }

    // 2. Find draft_session for this project
    let mut stmt = conn.prepare("SELECT id, status FROM draft_sessions WHERE project_id = ?1 LIMIT 1")?;
    let mut rows = stmt.query(params![project_id_str])?;
    let draft_session_id_str: String = if let Some(row) = rows.next()? {
        row.get(0)?
    } else {
        return Err(CommitRepoError::NotFound(format!(
            "No draft session found for project '{}'",
            project_id_str
        )));
    };

    let draft_session_id = Uuid::parse_str(&draft_session_id_str).map_err(|e| {
        CommitRepoError::InvalidState(format!("Invalid UUID in draft_sessions.id: {}", e))
    })?;

    let now = Utc::now().to_rfc3339();
    conn.execute(
        "INSERT OR IGNORE INTO sessions (id, project_id, display_name, created_at) VALUES (?1, ?2, 'Active Session', ?3)",
        params![active_session_id, project_id_str, now],
    )?;
    conn.execute(
        "UPDATE draft_sessions SET status = 'editing', session_id = ?1, updated_at = ?2 WHERE id = ?3",
        params![active_session_id, now, draft_session_id_str],
    )?;

    Ok(draft_session_id)
}
