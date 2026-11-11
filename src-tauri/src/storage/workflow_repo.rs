//! Read-only workflow receipt repository (docs/business/be-core.md §3.9, §6.4, SF-06 B+).
//!
//! Provides typed read queries scoped strictly by `draft_session_id` for:
//! - `workflow_round_receipts`
//! - `workflow_task_receipts`
//!
//! Note: This module provides read-only queries. Ownership of round seal / receipt creation (TX-04)
//! is located in `service::workflow_service`, not in base storage repositories.

use rusqlite::{params, Connection, Row};

/// Canonical error type for workflow receipt queries.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum WorkflowRepoError {
    #[error(
        "Foreign draft access rejected: caller '{caller}' attempted access to draft '{target}'"
    )]
    ForeignDraftAccess { caller: String, target: String },

    #[error("Invalid draft session ID (UUID format required): {0}")]
    InvalidSessionId(String),

    #[error("Missing schema or table: {0}")]
    MissingSchema(String),

    #[error("Workflow receipt not found: {0}")]
    NotFound(String),

    #[error("Database error: {0}")]
    Sqlite(String),

    #[error("Invalid data in receipt: {0}")]
    InvalidData(String),
}

fn map_sqlite_err(e: rusqlite::Error) -> WorkflowRepoError {
    let msg = e.to_string();
    if msg.contains("no such table") {
        WorkflowRepoError::MissingSchema(msg)
    } else {
        WorkflowRepoError::Sqlite(msg)
    }
}

/// Validate that a draft_session_id string is a valid canonical 36-char UUID.
pub fn validate_draft_session_id(id: &str) -> Result<uuid::Uuid, WorkflowRepoError> {
    if id.len() != 36 {
        return Err(WorkflowRepoError::InvalidSessionId(format!(
            "invalid length {}, expected 36 characters",
            id.len()
        )));
    }
    uuid::Uuid::parse_str(id).map_err(|e| {
        WorkflowRepoError::InvalidSessionId(format!("failed to parse UUID '{}': {}", id, e))
    })
}

/// Workflow round receipt record (`workflow_round_receipts` table, docs/db.dbml:1300).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct WorkflowRoundReceiptRecord {
    pub id: String,
    pub draft_session_id: String,
    pub workflow_run_id: String,
    pub plan_generation: i64,
    pub round_index: i64,
    pub round_seal_key: String,
    pub task_count: i64,
    pub parallel_limit: i64,
    pub plan_digest: String,
    pub request_digest: String,
    pub base_version_seq: i64,
    pub base_data_hash: String,
    pub final_data_hash: String,
    pub resulting_version_seq: Option<i64>,
    pub created_at: String,
}

impl WorkflowRoundReceiptRecord {
    fn from_row(row: &Row) -> Result<Self, rusqlite::Error> {
        Ok(Self {
            id: row.get(0)?,
            draft_session_id: row.get(1)?,
            workflow_run_id: row.get(2)?,
            plan_generation: row.get(3)?,
            round_index: row.get(4)?,
            round_seal_key: row.get(5)?,
            task_count: row.get(6)?,
            parallel_limit: row.get(7)?,
            plan_digest: row.get(8)?,
            request_digest: row.get(9)?,
            base_version_seq: row.get(10)?,
            base_data_hash: row.get(11)?,
            final_data_hash: row.get(12)?,
            resulting_version_seq: row.get(13)?,
            created_at: row.get(14)?,
        })
    }
}

/// Workflow task receipt record (`workflow_task_receipts` table, docs/db.dbml:1326).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct WorkflowTaskReceiptRecord {
    pub id: String,
    pub draft_session_id: String,
    pub workflow_run_id: String,
    pub plan_generation: i64,
    pub task_identity: String,
    pub input_scope_fingerprint: String,
    pub output_scope_fingerprint: String,
    pub base_version_seq: i64,
    pub resulting_version_seq: Option<i64>,
    pub had_mutations: bool,
    pub round_receipt_id: Option<String>,
    pub task_index: Option<i64>,
    pub evidence_json: Option<String>,
    pub created_at: Option<String>,
}

impl WorkflowTaskReceiptRecord {
    fn from_row(row: &Row) -> Result<Self, rusqlite::Error> {
        let had_mutations_int: i64 = row.get(9)?;
        Ok(Self {
            id: row.get(0)?,
            draft_session_id: row.get(1)?,
            workflow_run_id: row.get(2)?,
            plan_generation: row.get(3)?,
            task_identity: row.get(4)?,
            input_scope_fingerprint: row.get(5)?,
            output_scope_fingerprint: row.get(6)?,
            base_version_seq: row.get(7)?,
            resulting_version_seq: row.get(8)?,
            had_mutations: had_mutations_int == 1,
            round_receipt_id: row.get(10)?,
            task_index: row.get(11)?,
            evidence_json: row.get(12)?,
            created_at: row.get(13)?,
        })
    }
}

/// Abstract read-only repository trait for workflow receipts.
pub trait WorkflowReadRepo {
    fn find_round_receipt(
        &self,
        draft_session_id: &str,
        workflow_run_id: &str,
        plan_generation: i64,
        round_index: i64,
    ) -> Result<Option<WorkflowRoundReceiptRecord>, WorkflowRepoError>;

    fn find_round_receipt_by_id(
        &self,
        draft_session_id: &str,
        round_receipt_id: &str,
    ) -> Result<Option<WorkflowRoundReceiptRecord>, WorkflowRepoError>;

    fn find_round_receipt_by_seal_key(
        &self,
        draft_session_id: &str,
        round_seal_key: &str,
    ) -> Result<Option<WorkflowRoundReceiptRecord>, WorkflowRepoError>;

    fn list_round_receipts(
        &self,
        draft_session_id: &str,
        workflow_run_id: &str,
    ) -> Result<Vec<WorkflowRoundReceiptRecord>, WorkflowRepoError>;

    fn find_task_receipt(
        &self,
        draft_session_id: &str,
        workflow_run_id: &str,
        plan_generation: i64,
        task_identity: &str,
    ) -> Result<Option<WorkflowTaskReceiptRecord>, WorkflowRepoError>;

    fn find_task_receipt_by_id(
        &self,
        draft_session_id: &str,
        task_receipt_id: &str,
    ) -> Result<Option<WorkflowTaskReceiptRecord>, WorkflowRepoError>;

    fn list_task_receipts_for_round(
        &self,
        draft_session_id: &str,
        round_receipt_id: &str,
    ) -> Result<Vec<WorkflowTaskReceiptRecord>, WorkflowRepoError>;

    fn list_task_receipts_for_run(
        &self,
        draft_session_id: &str,
        workflow_run_id: &str,
        plan_generation: i64,
    ) -> Result<Vec<WorkflowTaskReceiptRecord>, WorkflowRepoError>;

    /// Scoped round receipt query with foreign-draft rejection.
    fn query_scoped_round_receipt(
        &self,
        caller_draft_session_id: &str,
        target_draft_session_id: &str,
        round_receipt_id: &str,
    ) -> Result<WorkflowRoundReceiptRecord, WorkflowRepoError>;
}

/// SQLite-backed implementation of `WorkflowReadRepo`.
pub struct SqliteWorkflowRepo<'a> {
    conn: &'a Connection,
}

impl<'a> SqliteWorkflowRepo<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }
}

impl<'a> WorkflowReadRepo for SqliteWorkflowRepo<'a> {
    fn find_round_receipt(
        &self,
        draft_session_id: &str,
        workflow_run_id: &str,
        plan_generation: i64,
        round_index: i64,
    ) -> Result<Option<WorkflowRoundReceiptRecord>, WorkflowRepoError> {
        validate_draft_session_id(draft_session_id)?;

        let mut stmt = self
            .conn
            .prepare(
                "SELECT id, draft_session_id, workflow_run_id, plan_generation, round_index,
                        round_seal_key, task_count, parallel_limit, plan_digest, request_digest,
                        base_version_seq, base_data_hash, final_data_hash, resulting_version_seq,
                        created_at
                 FROM workflow_round_receipts
                 WHERE draft_session_id = ?1 AND workflow_run_id = ?2 AND plan_generation = ?3 AND round_index = ?4",
            )
            .map_err(map_sqlite_err)?;

        let mut rows = stmt
            .query_map(
                params![
                    draft_session_id,
                    workflow_run_id,
                    plan_generation,
                    round_index
                ],
                WorkflowRoundReceiptRecord::from_row,
            )
            .map_err(map_sqlite_err)?;

        match rows.next() {
            Some(res) => res.map(Some).map_err(map_sqlite_err),
            None => Ok(None),
        }
    }

    fn find_round_receipt_by_id(
        &self,
        draft_session_id: &str,
        round_receipt_id: &str,
    ) -> Result<Option<WorkflowRoundReceiptRecord>, WorkflowRepoError> {
        validate_draft_session_id(draft_session_id)?;

        let mut stmt = self
            .conn
            .prepare(
                "SELECT id, draft_session_id, workflow_run_id, plan_generation, round_index,
                        round_seal_key, task_count, parallel_limit, plan_digest, request_digest,
                        base_version_seq, base_data_hash, final_data_hash, resulting_version_seq,
                        created_at
                 FROM workflow_round_receipts
                 WHERE draft_session_id = ?1 AND id = ?2",
            )
            .map_err(map_sqlite_err)?;

        let mut rows = stmt
            .query_map(
                params![draft_session_id, round_receipt_id],
                WorkflowRoundReceiptRecord::from_row,
            )
            .map_err(map_sqlite_err)?;

        match rows.next() {
            Some(res) => res.map(Some).map_err(map_sqlite_err),
            None => Ok(None),
        }
    }

    fn find_round_receipt_by_seal_key(
        &self,
        draft_session_id: &str,
        round_seal_key: &str,
    ) -> Result<Option<WorkflowRoundReceiptRecord>, WorkflowRepoError> {
        validate_draft_session_id(draft_session_id)?;

        let mut stmt = self
            .conn
            .prepare(
                "SELECT id, draft_session_id, workflow_run_id, plan_generation, round_index,
                        round_seal_key, task_count, parallel_limit, plan_digest, request_digest,
                        base_version_seq, base_data_hash, final_data_hash, resulting_version_seq,
                        created_at
                 FROM workflow_round_receipts
                 WHERE draft_session_id = ?1 AND round_seal_key = ?2",
            )
            .map_err(map_sqlite_err)?;

        let mut rows = stmt
            .query_map(
                params![draft_session_id, round_seal_key],
                WorkflowRoundReceiptRecord::from_row,
            )
            .map_err(map_sqlite_err)?;

        match rows.next() {
            Some(res) => res.map(Some).map_err(map_sqlite_err),
            None => Ok(None),
        }
    }

    fn list_round_receipts(
        &self,
        draft_session_id: &str,
        workflow_run_id: &str,
    ) -> Result<Vec<WorkflowRoundReceiptRecord>, WorkflowRepoError> {
        validate_draft_session_id(draft_session_id)?;

        let mut stmt = self
            .conn
            .prepare(
                "SELECT id, draft_session_id, workflow_run_id, plan_generation, round_index,
                        round_seal_key, task_count, parallel_limit, plan_digest, request_digest,
                        base_version_seq, base_data_hash, final_data_hash, resulting_version_seq,
                        created_at
                 FROM workflow_round_receipts
                 WHERE draft_session_id = ?1 AND workflow_run_id = ?2
                 ORDER BY plan_generation ASC, round_index ASC",
            )
            .map_err(map_sqlite_err)?;

        let rows = stmt
            .query_map(
                params![draft_session_id, workflow_run_id],
                WorkflowRoundReceiptRecord::from_row,
            )
            .map_err(map_sqlite_err)?;

        let mut records = Vec::new();
        for r in rows {
            records.push(r.map_err(map_sqlite_err)?);
        }
        Ok(records)
    }

    fn find_task_receipt(
        &self,
        draft_session_id: &str,
        workflow_run_id: &str,
        plan_generation: i64,
        task_identity: &str,
    ) -> Result<Option<WorkflowTaskReceiptRecord>, WorkflowRepoError> {
        validate_draft_session_id(draft_session_id)?;

        let mut stmt = self
            .conn
            .prepare(
                "SELECT id, draft_session_id, workflow_run_id, plan_generation, task_identity,
                        input_scope_fingerprint, output_scope_fingerprint, base_version_seq,
                        resulting_version_seq, had_mutations, round_receipt_id, task_index,
                        evidence_json, created_at
                 FROM workflow_task_receipts
                 WHERE draft_session_id = ?1 AND workflow_run_id = ?2 AND plan_generation = ?3 AND task_identity = ?4",
            )
            .map_err(map_sqlite_err)?;

        let mut rows = stmt
            .query_map(
                params![
                    draft_session_id,
                    workflow_run_id,
                    plan_generation,
                    task_identity
                ],
                WorkflowTaskReceiptRecord::from_row,
            )
            .map_err(map_sqlite_err)?;

        match rows.next() {
            Some(res) => res.map(Some).map_err(map_sqlite_err),
            None => Ok(None),
        }
    }

    fn find_task_receipt_by_id(
        &self,
        draft_session_id: &str,
        task_receipt_id: &str,
    ) -> Result<Option<WorkflowTaskReceiptRecord>, WorkflowRepoError> {
        validate_draft_session_id(draft_session_id)?;

        let mut stmt = self
            .conn
            .prepare(
                "SELECT id, draft_session_id, workflow_run_id, plan_generation, task_identity,
                        input_scope_fingerprint, output_scope_fingerprint, base_version_seq,
                        resulting_version_seq, had_mutations, round_receipt_id, task_index,
                        evidence_json, created_at
                 FROM workflow_task_receipts
                 WHERE draft_session_id = ?1 AND id = ?2",
            )
            .map_err(map_sqlite_err)?;

        let mut rows = stmt
            .query_map(
                params![draft_session_id, task_receipt_id],
                WorkflowTaskReceiptRecord::from_row,
            )
            .map_err(map_sqlite_err)?;

        match rows.next() {
            Some(res) => res.map(Some).map_err(map_sqlite_err),
            None => Ok(None),
        }
    }

    fn list_task_receipts_for_round(
        &self,
        draft_session_id: &str,
        round_receipt_id: &str,
    ) -> Result<Vec<WorkflowTaskReceiptRecord>, WorkflowRepoError> {
        validate_draft_session_id(draft_session_id)?;

        let mut stmt = self
            .conn
            .prepare(
                "SELECT id, draft_session_id, workflow_run_id, plan_generation, task_identity,
                        input_scope_fingerprint, output_scope_fingerprint, base_version_seq,
                        resulting_version_seq, had_mutations, round_receipt_id, task_index,
                        evidence_json, created_at
                 FROM workflow_task_receipts
                 WHERE draft_session_id = ?1 AND round_receipt_id = ?2
                 ORDER BY task_index ASC, id ASC",
            )
            .map_err(map_sqlite_err)?;

        let rows = stmt
            .query_map(
                params![draft_session_id, round_receipt_id],
                WorkflowTaskReceiptRecord::from_row,
            )
            .map_err(map_sqlite_err)?;

        let mut records = Vec::new();
        for r in rows {
            records.push(r.map_err(map_sqlite_err)?);
        }
        Ok(records)
    }

    fn list_task_receipts_for_run(
        &self,
        draft_session_id: &str,
        workflow_run_id: &str,
        plan_generation: i64,
    ) -> Result<Vec<WorkflowTaskReceiptRecord>, WorkflowRepoError> {
        validate_draft_session_id(draft_session_id)?;

        let mut stmt = self
            .conn
            .prepare(
                "SELECT id, draft_session_id, workflow_run_id, plan_generation, task_identity,
                        input_scope_fingerprint, output_scope_fingerprint, base_version_seq,
                        resulting_version_seq, had_mutations, round_receipt_id, task_index,
                        evidence_json, created_at
                 FROM workflow_task_receipts
                 WHERE draft_session_id = ?1 AND workflow_run_id = ?2 AND plan_generation = ?3
                 ORDER BY task_index ASC, id ASC",
            )
            .map_err(map_sqlite_err)?;

        let rows = stmt
            .query_map(
                params![draft_session_id, workflow_run_id, plan_generation],
                WorkflowTaskReceiptRecord::from_row,
            )
            .map_err(map_sqlite_err)?;

        let mut records = Vec::new();
        for r in rows {
            records.push(r.map_err(map_sqlite_err)?);
        }
        Ok(records)
    }

    fn query_scoped_round_receipt(
        &self,
        caller_draft_session_id: &str,
        target_draft_session_id: &str,
        round_receipt_id: &str,
    ) -> Result<WorkflowRoundReceiptRecord, WorkflowRepoError> {
        validate_draft_session_id(caller_draft_session_id)?;
        validate_draft_session_id(target_draft_session_id)?;

        if caller_draft_session_id != target_draft_session_id {
            return Err(WorkflowRepoError::ForeignDraftAccess {
                caller: caller_draft_session_id.to_string(),
                target: target_draft_session_id.to_string(),
            });
        }

        // Check ownership of the receipt in DB
        let mut owner_stmt = self
            .conn
            .prepare("SELECT draft_session_id FROM workflow_round_receipts WHERE id = ?1")
            .map_err(map_sqlite_err)?;

        let mut owner_rows = owner_stmt
            .query_map(params![round_receipt_id], |row| row.get::<_, String>(0))
            .map_err(map_sqlite_err)?;

        let owner = match owner_rows.next() {
            Some(res) => res.map_err(map_sqlite_err)?,
            None => return Err(WorkflowRepoError::NotFound(round_receipt_id.to_string())),
        };

        if owner != caller_draft_session_id {
            return Err(WorkflowRepoError::ForeignDraftAccess {
                caller: caller_draft_session_id.to_string(),
                target: owner,
            });
        }

        self.find_round_receipt_by_id(caller_draft_session_id, round_receipt_id)?
            .ok_or_else(|| WorkflowRepoError::NotFound(round_receipt_id.to_string()))
    }
}
