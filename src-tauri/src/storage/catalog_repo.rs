//! Static reference catalog repository (docs/business/be-core.md §1.4, §6.2).
//!
//! Provides typed read queries for static / reference entities:
//! - motifs
//! - genres
//! - characters
//! - world_settings
//! - lore_entries
//! - prompt_blocks
//! - style_presets

use rusqlite::{params, Connection, Row};

/// Canonical error type for catalog queries.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum CatalogRepoError {
    #[error("Missing schema or table: {0}")]
    MissingSchema(String),

    #[error("Catalog entity not found: {0}")]
    NotFound(String),

    #[error("Database error: {0}")]
    Sqlite(String),

    #[error("Invalid data in catalog: {0}")]
    InvalidData(String),
}

fn map_sqlite_err(e: rusqlite::Error) -> CatalogRepoError {
    let msg = e.to_string();
    if msg.contains("no such table") {
        CatalogRepoError::MissingSchema(msg)
    } else {
        CatalogRepoError::Sqlite(msg)
    }
}

/// Motif reference record (`motifs` table, docs/db.dbml:14).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct MotifRecord {
    pub id: String,
    pub code: Option<String>,
    pub display_name: Option<String>,
    pub category: Option<String>,
    pub version: Option<String>,
    pub is_system: bool,
    pub description: Option<String>,
    pub min_chapter: Option<i64>,
    pub max_chapter: Option<i64>,
}

impl MotifRecord {
    fn from_row(row: &Row) -> Result<Self, rusqlite::Error> {
        let is_system_int: Option<i64> = row.get(5)?;
        let is_system = is_system_int.map(|v| v == 1).unwrap_or(false);

        Ok(Self {
            id: row.get(0)?,
            code: row.get(1)?,
            display_name: row.get(2)?,
            category: row.get(3)?,
            version: row.get(4)?,
            is_system,
            description: row.get(6)?,
            min_chapter: row.get(7)?,
            max_chapter: row.get(8)?,
        })
    }
}

/// Genre reference record (`genres` table, docs/db.dbml:34).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct GenreRecord {
    pub id: String,
    pub code: Option<String>,
    pub display_name: Option<String>,
    pub description: Option<String>,
}

impl GenreRecord {
    fn from_row(row: &Row) -> Result<Self, rusqlite::Error> {
        Ok(Self {
            id: row.get(0)?,
            code: row.get(1)?,
            display_name: row.get(2)?,
            description: row.get(3)?,
        })
    }
}

/// Character reference record (`characters` table, docs/db.dbml:415).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct CharacterRecord {
    pub id: String,
    pub code: Option<String>,
    pub display_name: Option<String>,
    pub nickname: Option<String>,
    pub birth: Option<String>,
    pub species: Option<String>,
    pub biological_sex: Option<String>,
    pub gender_identity: Option<String>,
    pub gender_expression: Option<String>,
    pub deleted_at: Option<String>,
}

impl CharacterRecord {
    fn from_row(row: &Row) -> Result<Self, rusqlite::Error> {
        Ok(Self {
            id: row.get(0)?,
            code: row.get(1)?,
            display_name: row.get(2)?,
            nickname: row.get(3)?,
            birth: row.get(4)?,
            species: row.get(5)?,
            biological_sex: row.get(6)?,
            gender_identity: row.get(7)?,
            gender_expression: row.get(8)?,
            deleted_at: row.get(9)?,
        })
    }
}

/// World setting reference record (`world_settings` table, docs/db.dbml:336).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct WorldSettingRecord {
    pub id: String,
    pub code: String,
    pub project_id: Option<String>,
    pub display_name: String,
    pub setting_type: String,
    pub parent_id: Option<String>,
    pub description: Option<String>,
    pub access: Option<String>,
    pub spatial_features: Option<String>,
    pub sensory_visual: Option<String>,
    pub sensory_auditory: Option<String>,
    pub sensory_tactile: Option<String>,
    pub sensory_olfactory: Option<String>,
    pub sensory_gustatory: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub deleted_at: Option<String>,
}

impl WorldSettingRecord {
    fn from_row(row: &Row) -> Result<Self, rusqlite::Error> {
        Ok(Self {
            id: row.get(0)?,
            code: row.get(1)?,
            project_id: row.get(2)?,
            display_name: row.get(3)?,
            setting_type: row.get(4)?,
            parent_id: row.get(5)?,
            description: row.get(6)?,
            access: row.get(7)?,
            spatial_features: row.get(8)?,
            sensory_visual: row.get(9)?,
            sensory_auditory: row.get(10)?,
            sensory_tactile: row.get(11)?,
            sensory_olfactory: row.get(12)?,
            sensory_gustatory: row.get(13)?,
            created_at: row.get(14)?,
            updated_at: row.get(15)?,
            deleted_at: row.get(16)?,
        })
    }
}

/// Lore entry reference record (`lore_entries` table, docs/db.dbml:363).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct LoreEntryRecord {
    pub id: String,
    pub code: String,
    pub entry_type: String,
    pub display_name: String,
    pub description: Option<String>,
    pub world_setting_id: Option<String>,
    pub data: Option<String>,
    pub deleted_at: Option<String>,
}

impl LoreEntryRecord {
    fn from_row(row: &Row) -> Result<Self, rusqlite::Error> {
        Ok(Self {
            id: row.get(0)?,
            code: row.get(1)?,
            entry_type: row.get(2)?,
            display_name: row.get(3)?,
            description: row.get(4)?,
            world_setting_id: row.get(5)?,
            data: row.get(6)?,
            deleted_at: row.get(7)?,
        })
    }
}

/// Prompt block reference record (`prompt_blocks` table, docs/db.dbml:735).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct PromptBlockRecord {
    pub id: String,
    pub prompt_code: String,
    pub display_name: Option<String>,
    pub heading_level: i64,
    pub content_format: String,
    pub content: String,
}

impl PromptBlockRecord {
    fn from_row(row: &Row) -> Result<Self, rusqlite::Error> {
        Ok(Self {
            id: row.get(0)?,
            prompt_code: row.get(1)?,
            display_name: row.get(2)?,
            heading_level: row.get(3)?,
            content_format: row.get(4)?,
            content: row.get(5)?,
        })
    }
}

/// Style preset reference record (`style_presets` table, docs/db.dbml:139).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct StylePresetRecord {
    pub id: String,
    pub code: Option<String>,
    pub category: Option<String>,
    pub tone_pace: Option<String>,
    pub tone_emotion: Option<String>,
    pub is_system: bool,
}

impl StylePresetRecord {
    fn from_row(row: &Row) -> Result<Self, rusqlite::Error> {
        let is_sys: Option<i64> = row.get(5)?;
        Ok(Self {
            id: row.get(0)?,
            code: row.get(1)?,
            category: row.get(2)?,
            tone_pace: row.get(3)?,
            tone_emotion: row.get(4)?,
            is_system: is_sys.map(|v| v == 1).unwrap_or(false),
        })
    }
}

/// Abstract read-only trait for static catalog queries.
pub trait CatalogRepo {
    fn get_motif_by_code(&self, code: &str) -> Result<Option<MotifRecord>, CatalogRepoError>;
    fn list_motifs(&self, category: Option<&str>) -> Result<Vec<MotifRecord>, CatalogRepoError>;

    fn get_genre_by_code(&self, code: &str) -> Result<Option<GenreRecord>, CatalogRepoError>;
    fn list_genres(&self) -> Result<Vec<GenreRecord>, CatalogRepoError>;

    fn get_character_by_code(
        &self,
        code: &str,
    ) -> Result<Option<CharacterRecord>, CatalogRepoError>;
    fn list_characters(
        &self,
        include_deleted: bool,
    ) -> Result<Vec<CharacterRecord>, CatalogRepoError>;

    fn get_world_setting_by_code(
        &self,
        code: &str,
    ) -> Result<Option<WorldSettingRecord>, CatalogRepoError>;
    fn list_world_settings(
        &self,
        project_id: Option<&str>,
        include_deleted: bool,
    ) -> Result<Vec<WorldSettingRecord>, CatalogRepoError>;

    fn get_lore_entry_by_code(
        &self,
        code: &str,
    ) -> Result<Option<LoreEntryRecord>, CatalogRepoError>;
    fn list_lore_entries(
        &self,
        world_setting_id: Option<&str>,
        include_deleted: bool,
    ) -> Result<Vec<LoreEntryRecord>, CatalogRepoError>;

    fn get_prompt_block(
        &self,
        prompt_code: &str,
    ) -> Result<Option<PromptBlockRecord>, CatalogRepoError>;
    fn list_prompt_blocks(&self) -> Result<Vec<PromptBlockRecord>, CatalogRepoError>;

    fn get_style_preset_by_code(
        &self,
        code: &str,
    ) -> Result<Option<StylePresetRecord>, CatalogRepoError>;
    fn list_style_presets(&self) -> Result<Vec<StylePresetRecord>, CatalogRepoError>;
}

/// SQLite-backed implementation of `CatalogRepo`.
pub struct SqliteCatalogRepo<'a> {
    conn: &'a Connection,
}

impl<'a> SqliteCatalogRepo<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }
}

impl<'a> CatalogRepo for SqliteCatalogRepo<'a> {
    fn get_motif_by_code(&self, code: &str) -> Result<Option<MotifRecord>, CatalogRepoError> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT id, code, display_name, category, version, is_system, description, min_chapter, max_chapter
                 FROM motifs
                 WHERE code = ?1",
            )
            .map_err(map_sqlite_err)?;

        let mut rows = stmt
            .query_map(params![code], MotifRecord::from_row)
            .map_err(map_sqlite_err)?;

        match rows.next() {
            Some(res) => res.map(Some).map_err(map_sqlite_err),
            None => Ok(None),
        }
    }

    fn list_motifs(&self, category: Option<&str>) -> Result<Vec<MotifRecord>, CatalogRepoError> {
        let (sql, cat) = match category {
            Some(c) => (
                "SELECT id, code, display_name, category, version, is_system, description, min_chapter, max_chapter
                 FROM motifs
                 WHERE category = ?1
                 ORDER BY code ASC",
                Some(c),
            ),
            None => (
                "SELECT id, code, display_name, category, version, is_system, description, min_chapter, max_chapter
                 FROM motifs
                 ORDER BY code ASC",
                None,
            ),
        };

        let mut stmt = self.conn.prepare(sql).map_err(map_sqlite_err)?;
        let rows = if let Some(c) = cat {
            stmt.query_map(params![c], MotifRecord::from_row)
        } else {
            stmt.query_map([], MotifRecord::from_row)
        }
        .map_err(map_sqlite_err)?;

        let mut records = Vec::new();
        for r in rows {
            records.push(r.map_err(map_sqlite_err)?);
        }
        Ok(records)
    }

    fn get_genre_by_code(&self, code: &str) -> Result<Option<GenreRecord>, CatalogRepoError> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT id, code, display_name, description
                 FROM genres
                 WHERE code = ?1",
            )
            .map_err(map_sqlite_err)?;

        let mut rows = stmt
            .query_map(params![code], GenreRecord::from_row)
            .map_err(map_sqlite_err)?;

        match rows.next() {
            Some(res) => res.map(Some).map_err(map_sqlite_err),
            None => Ok(None),
        }
    }

    fn list_genres(&self) -> Result<Vec<GenreRecord>, CatalogRepoError> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT id, code, display_name, description
                 FROM genres
                 ORDER BY code ASC",
            )
            .map_err(map_sqlite_err)?;

        let rows = stmt
            .query_map([], GenreRecord::from_row)
            .map_err(map_sqlite_err)?;

        let mut records = Vec::new();
        for r in rows {
            records.push(r.map_err(map_sqlite_err)?);
        }
        Ok(records)
    }

    fn get_character_by_code(
        &self,
        code: &str,
    ) -> Result<Option<CharacterRecord>, CatalogRepoError> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT id, code, display_name, nickname, birth, species, biological_sex, gender_identity, gender_expression, deleted_at
                 FROM characters
                 WHERE code = ?1",
            )
            .map_err(map_sqlite_err)?;

        let mut rows = stmt
            .query_map(params![code], CharacterRecord::from_row)
            .map_err(map_sqlite_err)?;

        match rows.next() {
            Some(res) => res.map(Some).map_err(map_sqlite_err),
            None => Ok(None),
        }
    }

    fn list_characters(
        &self,
        include_deleted: bool,
    ) -> Result<Vec<CharacterRecord>, CatalogRepoError> {
        let sql = if include_deleted {
            "SELECT id, code, display_name, nickname, birth, species, biological_sex, gender_identity, gender_expression, deleted_at
             FROM characters
             ORDER BY code ASC"
        } else {
            "SELECT id, code, display_name, nickname, birth, species, biological_sex, gender_identity, gender_expression, deleted_at
             FROM characters
             WHERE deleted_at IS NULL
             ORDER BY code ASC"
        };

        let mut stmt = self.conn.prepare(sql).map_err(map_sqlite_err)?;
        let rows = stmt
            .query_map([], CharacterRecord::from_row)
            .map_err(map_sqlite_err)?;

        let mut records = Vec::new();
        for r in rows {
            records.push(r.map_err(map_sqlite_err)?);
        }
        Ok(records)
    }

    fn get_world_setting_by_code(
        &self,
        code: &str,
    ) -> Result<Option<WorldSettingRecord>, CatalogRepoError> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT id, code, project_id, display_name, type, parent_id, description, access,
                        spatial_features, sensory_visual, sensory_auditory, sensory_tactile, sensory_olfactory, sensory_gustatory,
                        created_at, updated_at, deleted_at
                 FROM world_settings
                 WHERE code = ?1",
            )
            .map_err(map_sqlite_err)?;

        let mut rows = stmt
            .query_map(params![code], WorldSettingRecord::from_row)
            .map_err(map_sqlite_err)?;

        match rows.next() {
            Some(res) => res.map(Some).map_err(map_sqlite_err),
            None => Ok(None),
        }
    }

    fn list_world_settings(
        &self,
        project_id: Option<&str>,
        include_deleted: bool,
    ) -> Result<Vec<WorldSettingRecord>, CatalogRepoError> {
        let (sql, p_id) = match (project_id, include_deleted) {
            (Some(pid), true) => (
                "SELECT id, code, project_id, display_name, type, parent_id, description, access,
                        spatial_features, sensory_visual, sensory_auditory, sensory_tactile, sensory_olfactory, sensory_gustatory,
                        created_at, updated_at, deleted_at
                 FROM world_settings
                 WHERE project_id = ?1
                 ORDER BY code ASC",
                Some(pid),
            ),
            (Some(pid), false) => (
                "SELECT id, code, project_id, display_name, type, parent_id, description, access,
                        spatial_features, sensory_visual, sensory_auditory, sensory_tactile, sensory_olfactory, sensory_gustatory,
                        created_at, updated_at, deleted_at
                 FROM world_settings
                 WHERE project_id = ?1 AND deleted_at IS NULL
                 ORDER BY code ASC",
                Some(pid),
            ),
            (None, true) => (
                "SELECT id, code, project_id, display_name, type, parent_id, description, access,
                        spatial_features, sensory_visual, sensory_auditory, sensory_tactile, sensory_olfactory, sensory_gustatory,
                        created_at, updated_at, deleted_at
                 FROM world_settings
                 ORDER BY code ASC",
                None,
            ),
            (None, false) => (
                "SELECT id, code, project_id, display_name, type, parent_id, description, access,
                        spatial_features, sensory_visual, sensory_auditory, sensory_tactile, sensory_olfactory, sensory_gustatory,
                        created_at, updated_at, deleted_at
                 FROM world_settings
                 WHERE deleted_at IS NULL
                 ORDER BY code ASC",
                None,
            ),
        };

        let mut stmt = self.conn.prepare(sql).map_err(map_sqlite_err)?;
        let rows = if let Some(pid) = p_id {
            stmt.query_map(params![pid], WorldSettingRecord::from_row)
        } else {
            stmt.query_map([], WorldSettingRecord::from_row)
        }
        .map_err(map_sqlite_err)?;

        let mut records = Vec::new();
        for r in rows {
            records.push(r.map_err(map_sqlite_err)?);
        }
        Ok(records)
    }

    fn get_lore_entry_by_code(
        &self,
        code: &str,
    ) -> Result<Option<LoreEntryRecord>, CatalogRepoError> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT id, code, type, display_name, description, world_setting_id, data, deleted_at
                 FROM lore_entries
                 WHERE code = ?1",
            )
            .map_err(map_sqlite_err)?;

        let mut rows = stmt
            .query_map(params![code], LoreEntryRecord::from_row)
            .map_err(map_sqlite_err)?;

        match rows.next() {
            Some(res) => res.map(Some).map_err(map_sqlite_err),
            None => Ok(None),
        }
    }

    fn list_lore_entries(
        &self,
        world_setting_id: Option<&str>,
        include_deleted: bool,
    ) -> Result<Vec<LoreEntryRecord>, CatalogRepoError> {
        let (sql, ws_id) = match (world_setting_id, include_deleted) {
            (Some(ws), true) => (
                "SELECT id, code, type, display_name, description, world_setting_id, data, deleted_at
                 FROM lore_entries
                 WHERE world_setting_id = ?1
                 ORDER BY code ASC",
                Some(ws),
            ),
            (Some(ws), false) => (
                "SELECT id, code, type, display_name, description, world_setting_id, data, deleted_at
                 FROM lore_entries
                 WHERE world_setting_id = ?1 AND deleted_at IS NULL
                 ORDER BY code ASC",
                Some(ws),
            ),
            (None, true) => (
                "SELECT id, code, type, display_name, description, world_setting_id, data, deleted_at
                 FROM lore_entries
                 ORDER BY code ASC",
                None,
            ),
            (None, false) => (
                "SELECT id, code, type, display_name, description, world_setting_id, data, deleted_at
                 FROM lore_entries
                 WHERE deleted_at IS NULL
                 ORDER BY code ASC",
                None,
            ),
        };

        let mut stmt = self.conn.prepare(sql).map_err(map_sqlite_err)?;
        let rows = if let Some(ws) = ws_id {
            stmt.query_map(params![ws], LoreEntryRecord::from_row)
        } else {
            stmt.query_map([], LoreEntryRecord::from_row)
        }
        .map_err(map_sqlite_err)?;

        let mut records = Vec::new();
        for r in rows {
            records.push(r.map_err(map_sqlite_err)?);
        }
        Ok(records)
    }

    fn get_prompt_block(
        &self,
        prompt_code: &str,
    ) -> Result<Option<PromptBlockRecord>, CatalogRepoError> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT id, prompt_code, display_name, heading_level, content_format, content
                 FROM prompt_blocks
                 WHERE prompt_code = ?1",
            )
            .map_err(map_sqlite_err)?;

        let mut rows = stmt
            .query_map(params![prompt_code], PromptBlockRecord::from_row)
            .map_err(map_sqlite_err)?;

        match rows.next() {
            Some(res) => res.map(Some).map_err(map_sqlite_err),
            None => Ok(None),
        }
    }

    fn list_prompt_blocks(&self) -> Result<Vec<PromptBlockRecord>, CatalogRepoError> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT id, prompt_code, display_name, heading_level, content_format, content
                 FROM prompt_blocks
                 ORDER BY prompt_code ASC",
            )
            .map_err(map_sqlite_err)?;

        let rows = stmt
            .query_map([], PromptBlockRecord::from_row)
            .map_err(map_sqlite_err)?;

        let mut records = Vec::new();
        for r in rows {
            records.push(r.map_err(map_sqlite_err)?);
        }
        Ok(records)
    }

    fn get_style_preset_by_code(
        &self,
        code: &str,
    ) -> Result<Option<StylePresetRecord>, CatalogRepoError> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT id, code, category, tone_pace, tone_emotion, is_system
                 FROM style_presets
                 WHERE code = ?1",
            )
            .map_err(map_sqlite_err)?;

        let mut rows = stmt
            .query_map(params![code], StylePresetRecord::from_row)
            .map_err(map_sqlite_err)?;

        match rows.next() {
            Some(res) => res.map(Some).map_err(map_sqlite_err),
            None => Ok(None),
        }
    }

    fn list_style_presets(&self) -> Result<Vec<StylePresetRecord>, CatalogRepoError> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT id, code, category, tone_pace, tone_emotion, is_system
                 FROM style_presets
                 ORDER BY code ASC",
            )
            .map_err(map_sqlite_err)?;

        let rows = stmt
            .query_map([], StylePresetRecord::from_row)
            .map_err(map_sqlite_err)?;

        let mut records = Vec::new();
        for r in rows {
            records.push(r.map_err(map_sqlite_err)?);
        }
        Ok(records)
    }
}
