//! Base draft repository providing typed transaction-scoped read operations
//! for draft sessions, draft versions, and pending patches (docs/business/be-core.md §3.6, §3.8, §4.4).
//!
//! Note: This module provides read-only snapshot queries scoped by `draft_session_id`.
//! Ownership of commit transactions (TX-01, TX-04, TX-08) is not located here.

use rusqlite::{params, Connection, Row};
use std::fmt;

/// Canonical error type for draft repository queries.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum DraftRepoError {
    #[error("Foreign draft access rejected: caller '{caller}' attempted access to draft '{target}'")]
    ForeignDraftAccess { caller: String, target: String },

    #[error("Invalid draft session ID (UUID format required): {0}")]
    InvalidSessionId(String),

    #[error("Missing schema or table: {0}")]
    MissingSchema(String),

    #[error("Draft record not found: {0}")]
    NotFound(String),

    #[error("Database error: {0}")]
    Sqlite(String),

    #[error("Invalid data in database: {0}")]
    InvalidData(String),
}

fn map_sqlite_err(e: rusqlite::Error) -> DraftRepoError {
    let msg = e.to_string();
    if msg.contains("no such table") {
        DraftRepoError::MissingSchema(msg)
    } else {
        DraftRepoError::Sqlite(msg)
    }
}

/// Validate that a draft_session_id string is a valid canonical 36-char UUID.
pub fn validate_draft_session_id(id: &str) -> Result<uuid::Uuid, DraftRepoError> {
    if id.len() != 36 {
        return Err(DraftRepoError::InvalidSessionId(format!(
            "invalid length {}, expected 36 characters",
            id.len()
        )));
    }
    uuid::Uuid::parse_str(id).map_err(|e| {
        DraftRepoError::InvalidSessionId(format!("failed to parse UUID '{}': {}", id, e))
    })
}

/// Lifecycle status of a draft session (docs/db.dbml:1255).
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DraftSessionStatus {
    Editing,
    Committing,
    Committed,
}

impl DraftSessionStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            DraftSessionStatus::Editing => "editing",
            DraftSessionStatus::Committing => "committing",
            DraftSessionStatus::Committed => "committed",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "editing" => Some(DraftSessionStatus::Editing),
            "committing" => Some(DraftSessionStatus::Committing),
            "committed" => Some(DraftSessionStatus::Committed),
            _ => None,
        }
    }
}

impl fmt::Display for DraftSessionStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Strongly-typed draft session record from `draft_sessions` table.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct DraftSessionRecord {
    pub id: String,
    pub session_id: String,
    pub project_id: Option<String>,
    pub head_version_seq: Option<i64>,
    pub status: DraftSessionStatus,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl DraftSessionRecord {
    fn from_row(row: &Row) -> Result<Self, rusqlite::Error> {
        let status_str: Option<String> = row.get(4)?;
        let status = status_str
            .as_deref()
            .and_then(DraftSessionStatus::from_str)
            .unwrap_or(DraftSessionStatus::Editing);

        Ok(Self {
            id: row.get(0)?,
            session_id: row.get(1)?,
            project_id: row.get(2)?,
            head_version_seq: row.get(3)?,
            status,
            created_at: row.get(5)?,
            updated_at: row.get(6)?,
        })
    }
}

/// Strongly-typed draft version record from `draft_versions` table.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct DraftVersionRecord {
    pub id: String,
    pub draft_session_id: String,
    pub seq: i64,
    pub seal_key: Option<String>,
    pub phase: Option<i64>,
    pub subphase: Option<i64>,
    pub data_json: String,
    pub data_hash: String,
    pub reason: String,
    pub parent_seq: Option<i64>,
    pub turn_id: Option<String>,
    pub chat_message_id: Option<String>,
    pub created_at: Option<String>,
}

impl DraftVersionRecord {
    fn from_row(row: &Row) -> Result<Self, rusqlite::Error> {
        Ok(Self {
            id: row.get(0)?,
            draft_session_id: row.get(1)?,
            seq: row.get(2)?,
            seal_key: row.get(3)?,
            phase: row.get(4)?,
            subphase: row.get(5)?,
            data_json: row.get(6)?,
            data_hash: row.get(7)?,
            reason: row.get(8)?,
            parent_seq: row.get(9)?,
            turn_id: row.get(10)?,
            chat_message_id: row.get(11)?,
            created_at: row.get(12)?,
        })
    }
}

/// Status of a pending patch (docs/db.dbml:1367).
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PendingPatchStatus {
    Pending,
    Confirmed,
    Rejected,
    Expired,
}

impl PendingPatchStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            PendingPatchStatus::Pending => "pending",
            PendingPatchStatus::Confirmed => "confirmed",
            PendingPatchStatus::Rejected => "rejected",
            PendingPatchStatus::Expired => "expired",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "pending" => Some(PendingPatchStatus::Pending),
            "confirmed" => Some(PendingPatchStatus::Confirmed),
            "rejected" => Some(PendingPatchStatus::Rejected),
            "expired" => Some(PendingPatchStatus::Expired),
            _ => None,
        }
    }
}

impl fmt::Display for PendingPatchStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Strongly-typed pending patch record from `pending_patches` table.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct PendingPatchRecord {
    pub id: String,
    pub draft_session_id: String,
    pub base_version_seq: Option<i64>,
    pub target_type: String,
    pub target_code: String,
    pub field_name: Option<String>,
    pub operation: String,
    pub previous_value: Option<String>,
    pub new_value: Option<String>,
    pub target_source: Option<String>,
    pub eligibility_json: Option<String>,
    pub status: PendingPatchStatus,
    pub created_at: Option<String>,
    pub resolved_at: Option<String>,
}

impl PendingPatchRecord {
    fn from_row(row: &Row) -> Result<Self, rusqlite::Error> {
        let status_str: Option<String> = row.get(11)?;
        let status = status_str
            .as_deref()
            .and_then(PendingPatchStatus::from_str)
            .unwrap_or(PendingPatchStatus::Pending);

        Ok(Self {
            id: row.get(0)?,
            draft_session_id: row.get(1)?,
            base_version_seq: row.get(2)?,
            target_type: row.get(3)?,
            target_code: row.get(4)?,
            field_name: row.get(5)?,
            operation: row.get(6)?,
            previous_value: row.get(7)?,
            new_value: row.get(8)?,
            target_source: row.get(9)?,
            eligibility_json: row.get(10)?,
            status,
            created_at: row.get(12)?,
            resolved_at: row.get(13)?,
        })
    }
}

/// Abstract read-only repository trait for draft operations.
pub trait DraftRepo {
    fn find_session_by_id(&self, draft_session_id: &str) -> Result<Option<DraftSessionRecord>, DraftRepoError>;
    fn find_session_by_session_id(&self, session_id: &str) -> Result<Option<DraftSessionRecord>, DraftRepoError>;
    fn get_session_status(&self, draft_session_id: &str) -> Result<Option<DraftSessionStatus>, DraftRepoError>;
    fn get_head_version(&self, draft_session_id: &str) -> Result<Option<DraftVersionRecord>, DraftRepoError>;
    fn get_version_by_seq(&self, draft_session_id: &str, seq: i64) -> Result<Option<DraftVersionRecord>, DraftRepoError>;
    fn find_version_by_seal_key(&self, draft_session_id: &str, seal_key: &str) -> Result<Option<DraftVersionRecord>, DraftRepoError>;
    fn list_versions(&self, draft_session_id: &str, limit: Option<u32>) -> Result<Vec<DraftVersionRecord>, DraftRepoError>;
    fn list_pending_patches(&self, draft_session_id: &str, status: Option<PendingPatchStatus>) -> Result<Vec<PendingPatchRecord>, DraftRepoError>;
    fn find_pending_patch_by_id(&self, draft_session_id: &str, patch_id: &str) -> Result<Option<PendingPatchRecord>, DraftRepoError>;

    /// Scoped version lookup rejecting foreign-draft access.
    fn query_scoped_version_record(
        &self,
        caller_draft_session_id: &str,
        target_draft_session_id: &str,
        seq: i64,
    ) -> Result<DraftVersionRecord, DraftRepoError>;

    /// Scoped patch lookup rejecting foreign-draft access.
    fn query_scoped_patch(
        &self,
        caller_draft_session_id: &str,
        patch_id: &str,
    ) -> Result<PendingPatchRecord, DraftRepoError>;
}

/// SQLite-backed implementation of `DraftRepo` operating on a connection or transaction.
pub struct SqliteDraftRepo<'a> {
    conn: &'a Connection,
}

impl<'a> SqliteDraftRepo<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }
}

impl<'a> DraftRepo for SqliteDraftRepo<'a> {
    fn find_session_by_id(&self, draft_session_id: &str) -> Result<Option<DraftSessionRecord>, DraftRepoError> {
        validate_draft_session_id(draft_session_id)?;

        let mut stmt = self
            .conn
            .prepare(
                "SELECT id, session_id, project_id, head_version_seq, status, created_at, updated_at
                 FROM draft_sessions
                 WHERE id = ?1",
            )
            .map_err(map_sqlite_err)?;

        let mut rows = stmt
            .query_map(params![draft_session_id], DraftSessionRecord::from_row)
            .map_err(map_sqlite_err)?;

        match rows.next() {
            Some(res) => res.map(Some).map_err(map_sqlite_err),
            None => Ok(None),
        }
    }

    fn find_session_by_session_id(&self, session_id: &str) -> Result<Option<DraftSessionRecord>, DraftRepoError> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT id, session_id, project_id, head_version_seq, status, created_at, updated_at
                 FROM draft_sessions
                 WHERE session_id = ?1",
            )
            .map_err(map_sqlite_err)?;

        let mut rows = stmt
            .query_map(params![session_id], DraftSessionRecord::from_row)
            .map_err(map_sqlite_err)?;

        match rows.next() {
            Some(res) => res.map(Some).map_err(map_sqlite_err),
            None => Ok(None),
        }
    }

    fn get_session_status(&self, draft_session_id: &str) -> Result<Option<DraftSessionStatus>, DraftRepoError> {
        validate_draft_session_id(draft_session_id)?;

        let mut stmt = self
            .conn
            .prepare("SELECT status FROM draft_sessions WHERE id = ?1")
            .map_err(map_sqlite_err)?;

        let mut rows = stmt
            .query_map(params![draft_session_id], |row| {
                let status_str: Option<String> = row.get(0)?;
                Ok(status_str.as_deref().and_then(DraftSessionStatus::from_str))
            })
            .map_err(map_sqlite_err)?;

        match rows.next() {
            Some(res) => res.map_err(map_sqlite_err),
            None => Ok(None),
        }
    }

    fn get_head_version(&self, draft_session_id: &str) -> Result<Option<DraftVersionRecord>, DraftRepoError> {
        validate_draft_session_id(draft_session_id)?;

        let mut stmt = self
            .conn
            .prepare(
                "SELECT v.id, v.draft_session_id, v.seq, v.seal_key, v.phase, v.subphase,
                        v.data_json, v.data_hash, v.reason, v.parent_seq, v.turn_id,
                        v.chat_message_id, v.created_at
                 FROM draft_versions v
                 INNER JOIN draft_sessions s ON s.id = v.draft_session_id AND s.head_version_seq = v.seq
                 WHERE v.draft_session_id = ?1",
            )
            .map_err(map_sqlite_err)?;

        let mut rows = stmt
            .query_map(params![draft_session_id], DraftVersionRecord::from_row)
            .map_err(map_sqlite_err)?;

        match rows.next() {
            Some(res) => res.map(Some).map_err(map_sqlite_err),
            None => Ok(None),
        }
    }

    fn get_version_by_seq(&self, draft_session_id: &str, seq: i64) -> Result<Option<DraftVersionRecord>, DraftRepoError> {
        validate_draft_session_id(draft_session_id)?;

        let mut stmt = self
            .conn
            .prepare(
                "SELECT id, draft_session_id, seq, seal_key, phase, subphase,
                        data_json, data_hash, reason, parent_seq, turn_id,
                        chat_message_id, created_at
                 FROM draft_versions
                 WHERE draft_session_id = ?1 AND seq = ?2",
            )
            .map_err(map_sqlite_err)?;

        let mut rows = stmt
            .query_map(params![draft_session_id, seq], DraftVersionRecord::from_row)
            .map_err(map_sqlite_err)?;

        match rows.next() {
            Some(res) => res.map(Some).map_err(map_sqlite_err),
            None => Ok(None),
        }
    }

    fn find_version_by_seal_key(&self, draft_session_id: &str, seal_key: &str) -> Result<Option<DraftVersionRecord>, DraftRepoError> {
        validate_draft_session_id(draft_session_id)?;

        let mut stmt = self
            .conn
            .prepare(
                "SELECT id, draft_session_id, seq, seal_key, phase, subphase,
                        data_json, data_hash, reason, parent_seq, turn_id,
                        chat_message_id, created_at
                 FROM draft_versions
                 WHERE draft_session_id = ?1 AND seal_key = ?2",
            )
            .map_err(map_sqlite_err)?;

        let mut rows = stmt
            .query_map(params![draft_session_id, seal_key], DraftVersionRecord::from_row)
            .map_err(map_sqlite_err)?;

        match rows.next() {
            Some(res) => res.map(Some).map_err(map_sqlite_err),
            None => Ok(None),
        }
    }

    fn list_versions(&self, draft_session_id: &str, limit: Option<u32>) -> Result<Vec<DraftVersionRecord>, DraftRepoError> {
        validate_draft_session_id(draft_session_id)?;

        let limit_val = limit.map(|l| l as i64).unwrap_or(1000);
        let mut stmt = self
            .conn
            .prepare(
                "SELECT id, draft_session_id, seq, seal_key, phase, subphase,
                        data_json, data_hash, reason, parent_seq, turn_id,
                        chat_message_id, created_at
                 FROM draft_versions
                 WHERE draft_session_id = ?1
                 ORDER BY seq ASC
                 LIMIT ?2",
            )
            .map_err(map_sqlite_err)?;

        let rows = stmt
            .query_map(params![draft_session_id, limit_val], DraftVersionRecord::from_row)
            .map_err(map_sqlite_err)?;

        let mut records = Vec::new();
        for r in rows {
            records.push(r.map_err(map_sqlite_err)?);
        }
        Ok(records)
    }

    fn list_pending_patches(&self, draft_session_id: &str, status: Option<PendingPatchStatus>) -> Result<Vec<PendingPatchRecord>, DraftRepoError> {
        validate_draft_session_id(draft_session_id)?;

        let (sql, filter_status) = match status {
            Some(st) => (
                "SELECT id, draft_session_id, base_version_seq, target_type, target_code,
                        field_name, operation, previous_value, new_value, target_source,
                        eligibility_json, status, created_at, resolved_at
                 FROM pending_patches
                 WHERE draft_session_id = ?1 AND status = ?2
                 ORDER BY created_at ASC, id ASC",
                Some(st.as_str()),
            ),
            None => (
                "SELECT id, draft_session_id, base_version_seq, target_type, target_code,
                        field_name, operation, previous_value, new_value, target_source,
                        eligibility_json, status, created_at, resolved_at
                 FROM pending_patches
                 WHERE draft_session_id = ?1
                 ORDER BY created_at ASC, id ASC",
                None,
            ),
        };

        let mut stmt = self.conn.prepare(sql).map_err(map_sqlite_err)?;

        let rows = if let Some(st) = filter_status {
            stmt.query_map(params![draft_session_id, st], PendingPatchRecord::from_row)
        } else {
            stmt.query_map(params![draft_session_id], PendingPatchRecord::from_row)
        }
        .map_err(map_sqlite_err)?;

        let mut records = Vec::new();
        for r in rows {
            records.push(r.map_err(map_sqlite_err)?);
        }
        Ok(records)
    }

    fn find_pending_patch_by_id(&self, draft_session_id: &str, patch_id: &str) -> Result<Option<PendingPatchRecord>, DraftRepoError> {
        validate_draft_session_id(draft_session_id)?;

        let mut stmt = self
            .conn
            .prepare(
                "SELECT id, draft_session_id, base_version_seq, target_type, target_code,
                        field_name, operation, previous_value, new_value, target_source,
                        eligibility_json, status, created_at, resolved_at
                 FROM pending_patches
                 WHERE draft_session_id = ?1 AND id = ?2",
            )
            .map_err(map_sqlite_err)?;

        let mut rows = stmt
            .query_map(params![draft_session_id, patch_id], PendingPatchRecord::from_row)
            .map_err(map_sqlite_err)?;

        match rows.next() {
            Some(res) => res.map(Some).map_err(map_sqlite_err),
            None => Ok(None),
        }
    }

    fn query_scoped_version_record(
        &self,
        caller_draft_session_id: &str,
        target_draft_session_id: &str,
        seq: i64,
    ) -> Result<DraftVersionRecord, DraftRepoError> {
        validate_draft_session_id(caller_draft_session_id)?;
        validate_draft_session_id(target_draft_session_id)?;

        if caller_draft_session_id != target_draft_session_id {
            return Err(DraftRepoError::ForeignDraftAccess {
                caller: caller_draft_session_id.to_string(),
                target: target_draft_session_id.to_string(),
            });
        }

        self.get_version_by_seq(caller_draft_session_id, seq)?
            .ok_or_else(|| DraftRepoError::NotFound(format!("seq {} in draft {}", seq, caller_draft_session_id)))
    }

    fn query_scoped_patch(
        &self,
        caller_draft_session_id: &str,
        patch_id: &str,
    ) -> Result<PendingPatchRecord, DraftRepoError> {
        validate_draft_session_id(caller_draft_session_id)?;

        // First, check if the patch exists anywhere in the database
        let mut owner_stmt = self
            .conn
            .prepare("SELECT draft_session_id FROM pending_patches WHERE id = ?1")
            .map_err(map_sqlite_err)?;

        let mut owner_rows = owner_stmt
            .query_map(params![patch_id], |row| row.get::<_, String>(0))
            .map_err(map_sqlite_err)?;

        let owner = match owner_rows.next() {
            Some(res) => res.map_err(map_sqlite_err)?,
            None => return Err(DraftRepoError::NotFound(patch_id.to_string())),
        };

        if owner != caller_draft_session_id {
            return Err(DraftRepoError::ForeignDraftAccess {
                caller: caller_draft_session_id.to_string(),
                target: owner,
            });
        }

        self.find_pending_patch_by_id(caller_draft_session_id, patch_id)?
            .ok_or_else(|| DraftRepoError::NotFound(patch_id.to_string()))
    }
}
