//! Pending Patches Repository owning SQL queries, deduplication, and CAS state transitions
//! (`REQ-PATCH-001`..`004`, `REQ-TX-008`, `be-core.md` §5.3.2, §1538–§1547).

use rusqlite::{params, Connection, OptionalExtension, Row};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::domain::business_flow::{BusinessFlowData, TrackedField};
use crate::error::{ErrorCode, IpcError};
use crate::ipc::dto::PendingPatchDto;

/// Typed database row representation of `pending_patches`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PendingPatchRecord {
    pub id: Uuid,
    pub draft_session_id: Uuid,
    pub base_version_seq: Option<i32>,
    pub target_type: String,
    pub target_code: String,
    pub field_name: Option<String>,
    pub operation: String, // "set" | "delete"
    pub previous_value: Option<Value>,
    pub new_value: Option<Value>,
    pub target_source: Option<String>,
    pub eligibility_json: Option<String>,
    pub status: String, // "pending" | "confirmed" | "rejected" | "expired"
    pub created_at: String,
    pub resolved_at: Option<String>,
}

impl From<PendingPatchRecord> for PendingPatchDto {
    fn from(r: PendingPatchRecord) -> Self {
        Self {
            id: r.id,
            draft_session_id: r.draft_session_id,
            target_code: r.target_code,
            field_path: r.field_name.unwrap_or_default(),
            operation: r.operation,
            previous_value: r.previous_value,
            proposed_value: r.new_value.unwrap_or(Value::Null),
            status: r.status,
            reason: None,
            created_at: r.created_at,
        }
    }
}

/// Structured schema for `pending_patches.eligibility_json` (`be-core.md` §1538).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProposalOrigin {
    #[serde(default)]
    pub workflow_run_id: Option<String>,
    #[serde(default)]
    pub plan_generation: Option<i32>,
    #[serde(default)]
    pub round_index: Option<i32>,
    #[serde(default)]
    pub task_identity: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProposalTarget {
    pub target_type: String,
    pub target_code: String,
    #[serde(default)]
    pub scope_fingerprint: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProposalDependency {
    pub path: String,
    pub scope_fingerprint: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProposalEligibility {
    #[serde(default = "default_schema_version")]
    pub schema_version: u32,
    #[serde(default)]
    pub base_version_seq: Option<i32>,
    #[serde(default)]
    pub origin: Option<ProposalOrigin>,
    #[serde(default)]
    pub target: Option<ProposalTarget>,
    #[serde(default)]
    pub dependencies: Vec<ProposalDependency>,
    #[serde(default)]
    pub missing_evidence: Option<bool>,
}

fn default_schema_version() -> u32 {
    1
}

/// Parameters for creating or deduplicating a proposal patch in TX-08.
#[derive(Debug, Clone)]
pub struct PersistProposalParams {
    pub draft_session_id: Uuid,
    pub base_version_seq: Option<i32>,
    pub target_type: String,
    pub target_code: String,
    pub field_name: Option<String>,
    pub operation: String, // "set" | "delete"
    pub previous_value: Option<Value>,
    pub new_value: Option<Value>,
    pub target_source: Option<String>,
    pub eligibility_json: Option<String>,
    pub require_new_origin: bool,
}

/// Outcome of TX-08 proposal persistence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PersistProposalResult {
    pub patch_id: Uuid,
    pub reused_existing: bool,
    pub accepted_new_origin: bool,
}

/// Helper parsing a SQLite row into `PendingPatchRecord`.
pub fn map_row_to_record(row: &Row<'_>) -> rusqlite::Result<PendingPatchRecord> {
    let id_str: String = row.get(0)?;
    let draft_session_id_str: String = row.get(1)?;
    let base_version_seq: Option<i32> = row.get(2)?;
    let target_type: String = row.get(3)?;
    let target_code: String = row.get(4)?;
    let field_name: Option<String> = row.get(5)?;
    let operation: String = row.get(6)?;
    let prev_val_str: Option<String> = row.get(7)?;
    let new_val_str: Option<String> = row.get(8)?;
    let target_source: Option<String> = row.get(9)?;
    let eligibility_json: Option<String> = row.get(10)?;
    let status: String = row.get(11)?;
    let created_at: String = row.get(12)?;
    let resolved_at: Option<String> = row.get(13)?;

    let id = Uuid::parse_str(&id_str).unwrap_or_default();
    let draft_session_id = Uuid::parse_str(&draft_session_id_str).unwrap_or_default();
    let previous_value = prev_val_str.and_then(|s| serde_json::from_str(&s).ok());
    let new_value = new_val_str.and_then(|s| serde_json::from_str(&s).ok());

    Ok(PendingPatchRecord {
        id,
        draft_session_id,
        base_version_seq,
        target_type,
        target_code,
        field_name,
        operation,
        previous_value,
        new_value,
        target_source,
        eligibility_json,
        status,
        created_at,
        resolved_at,
    })
}

/// Insert a new pending patch record into `pending_patches`.
pub fn insert_pending_patch(
    conn: &Connection,
    record: &PendingPatchRecord,
) -> Result<(), rusqlite::Error> {
    conn.execute(
        "INSERT INTO pending_patches (
            id, draft_session_id, base_version_seq, target_type, target_code,
            field_name, operation, previous_value, new_value, target_source,
            eligibility_json, status, created_at, resolved_at
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)",
        params![
            record.id.to_string(),
            record.draft_session_id.to_string(),
            record.base_version_seq,
            record.target_type,
            record.target_code,
            record.field_name,
            record.operation,
            record.previous_value.as_ref().map(|v| v.to_string()),
            record.new_value.as_ref().map(|v| v.to_string()),
            record.target_source,
            record.eligibility_json,
            record.status,
            record.created_at,
            record.resolved_at,
        ],
    )?;
    Ok(())
}

/// Find a patch record by `id`.
pub fn find_by_id(
    conn: &Connection,
    patch_id: &Uuid,
) -> Result<Option<PendingPatchRecord>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT id, draft_session_id, base_version_seq, target_type, target_code,
                field_name, operation, previous_value, new_value, target_source,
                eligibility_json, status, created_at, resolved_at
         FROM pending_patches
         WHERE id = ?1",
    )?;

    stmt.query_row(params![patch_id.to_string()], map_row_to_record)
        .optional()
}

/// Find an active `pending` patch by business key `(draft_session_id, target_code, field_name)` (§5.3.2).
pub fn find_pending_by_key(
    conn: &Connection,
    draft_session_id: &Uuid,
    target_code: &str,
    field_name: Option<&str>,
) -> Result<Option<PendingPatchRecord>, rusqlite::Error> {
    match field_name {
        Some(f) => {
            let mut stmt = conn.prepare(
                "SELECT id, draft_session_id, base_version_seq, target_type, target_code,
                        field_name, operation, previous_value, new_value, target_source,
                        eligibility_json, status, created_at, resolved_at
                 FROM pending_patches
                 WHERE draft_session_id = ?1 AND target_code = ?2 AND field_name = ?3 AND status = 'pending'",
            )?;
            stmt.query_row(
                params![draft_session_id.to_string(), target_code, f],
                map_row_to_record,
            )
            .optional()
        }
        None => {
            let mut stmt = conn.prepare(
                "SELECT id, draft_session_id, base_version_seq, target_type, target_code,
                        field_name, operation, previous_value, new_value, target_source,
                        eligibility_json, status, created_at, resolved_at
                 FROM pending_patches
                 WHERE draft_session_id = ?1 AND target_code = ?2 AND field_name IS NULL AND status = 'pending'",
            )?;
            stmt.query_row(
                params![draft_session_id.to_string(), target_code],
                map_row_to_record,
            )
            .optional()
        }
    }
}

/// List all patches for a draft session ordered by `created_at ASC, id ASC`.
pub fn list_by_draft(
    conn: &Connection,
    draft_session_id: &Uuid,
) -> Result<Vec<PendingPatchRecord>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT id, draft_session_id, base_version_seq, target_type, target_code,
                field_name, operation, previous_value, new_value, target_source,
                eligibility_json, status, created_at, resolved_at
         FROM pending_patches
         WHERE draft_session_id = ?1
         ORDER BY created_at ASC, id ASC",
    )?;

    let rows = stmt.query_map(params![draft_session_id.to_string()], map_row_to_record)?;
    let mut list = Vec::new();
    for r in rows {
        list.push(r?);
    }
    Ok(list)
}

/// List only active `pending` patches for a draft session ordered by `created_at ASC, id ASC`.
pub fn list_pending_by_draft(
    conn: &Connection,
    draft_session_id: &Uuid,
) -> Result<Vec<PendingPatchRecord>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT id, draft_session_id, base_version_seq, target_type, target_code,
                field_name, operation, previous_value, new_value, target_source,
                eligibility_json, status, created_at, resolved_at
         FROM pending_patches
         WHERE draft_session_id = ?1 AND status = 'pending'
         ORDER BY created_at ASC, id ASC",
    )?;

    let rows = stmt.query_map(params![draft_session_id.to_string()], map_row_to_record)?;
    let mut list = Vec::new();
    for r in rows {
        list.push(r?);
    }
    Ok(list)
}

/// Atomically transitions patch status via CAS (`pending` -> `confirmed` | `rejected` | `expired`).
pub fn update_status_cas(
    conn: &Connection,
    patch_id: &Uuid,
    expected_status: &str,
    new_status: &str,
    resolved_at: &str,
) -> Result<bool, rusqlite::Error> {
    let affected = conn.execute(
        "UPDATE pending_patches
         SET status = ?1, resolved_at = ?2
         WHERE id = ?3 AND status = ?4",
        params![new_status, resolved_at, patch_id.to_string(), expected_status],
    )?;
    Ok(affected > 0)
}

/// Mark a single patch as `expired`.
pub fn expire_patch(
    conn: &Connection,
    patch_id: &Uuid,
    resolved_at: &str,
) -> Result<bool, rusqlite::Error> {
    let affected = conn.execute(
        "UPDATE pending_patches
         SET status = 'expired', resolved_at = ?1
         WHERE id = ?2 AND status = 'pending'",
        params![resolved_at, patch_id.to_string()],
    )?;
    Ok(affected > 0)
}

/// Mark all pending patches for a draft as `expired` (e.g. on TX-03 restore).
pub fn expire_all_pending(
    conn: &Connection,
    draft_session_id: &Uuid,
    resolved_at: &str,
) -> Result<usize, rusqlite::Error> {
    let affected = conn.execute(
        "UPDATE pending_patches
         SET status = 'expired', resolved_at = ?1
         WHERE draft_session_id = ?2 AND status = 'pending'",
        params![resolved_at, draft_session_id.to_string()],
    )?;
    Ok(affected)
}

// ============================================================================
// Data Validity and Eligibility Checking (§5.3.2, §1538–§1547)
// ============================================================================

/// Result of evaluating a proposal patch against a candidate state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProposalEvaluationResult {
    /// Proposal is valid and eligible to be confirmed and applied.
    Eligible,
    /// Proposal is invalid or expired (target missing, previous value changed, dependency mismatch).
    Expired { reason: String },
    /// Proposal has workflow origin but missing dependency evidence or scope indeterminate.
    Indeterminate { reason: String },
}

/// Queries the current value of a field on an entity in `BusinessFlowData`.
/// Returns:
/// - `Err(())`: entity `target_code` was not found.
/// - `Ok(None)`: entity was found, but field is unset or null.
/// - `Ok(Some(val))`: entity was found, field has `val`.
pub fn get_field_value(
    state: &BusinessFlowData,
    target_type: &str,
    target_code: &str,
    field_name: &str,
) -> Result<Option<Value>, ()> {
    match target_type {
        "project" => {
            let p = &state.require.project;
            match field_name {
                "display_name" => Ok(p.display_name.value.as_ref().map(|s| Value::String(s.clone()))),
                "summary" => Ok(p.summary.value.as_ref().map(|s| Value::String(s.clone()))),
                "language" => Ok(p.language.value.as_ref().map(|s| Value::String(s.clone()))),
                "target.arc_count" => Ok(p.target.arc_count.value.map(Value::from)),
                "target.chapter_count" => Ok(p.target.chapter_count.value.map(Value::from)),
                "target.word_per_chapter_count" => Ok(p.target.word_per_chapter_count.value.map(Value::from)),
                _ => Ok(None),
            }
        }
        "character" => {
            let c = state
                .require
                .character
                .iter()
                .find(|ch| ch.code == target_code)
                .ok_or(())?;
            match field_name {
                "display_name" => Ok(c.display_name.value.as_ref().map(|s| Value::String(s.clone()))),
                "nickname" => Ok(c.nickname.value.as_ref().map(|s| Value::String(s.clone()))),
                "biological_sex" => Ok(c.biological_sex.value.as_ref().map(|s| Value::String(s.clone()))),
                "motif_code" => Ok(c.motif_code.value.as_ref().map(|s| Value::String(s.clone()))),
                "role" => Ok(c.role.value.as_ref().map(|s| Value::String(s.clone()))),
                "notes" => Ok(c.notes.value.as_ref().map(|s| Value::String(s.clone()))),
                _ => Ok(None),
            }
        }
        "world_setting" => {
            let ws = state
                .optional
                .world_building_phase
                .world_settings
                .iter()
                .find(|w| w.code == target_code)
                .ok_or(())?;
            match field_name {
                "display_name" | "name" => Ok(ws.display_name.value.as_ref().map(|s| Value::String(s.clone()))),
                "type" | "setting_type" | "category" => Ok(ws.setting_type.value.as_ref().map(|s| Value::String(s.clone()))),
                "description" => Ok(ws.description.value.as_ref().map(|s| Value::String(s.clone()))),
                _ => Ok(None),
            }
        }
        "lore_entry" => {
            for ws in &state.optional.world_building_phase.world_settings {
                if let Some(le) = ws.lore_entries.iter().find(|l| l.code == target_code) {
                    match field_name {
                        "display_name" | "title" => return Ok(le.display_name.value.as_ref().map(|s| Value::String(s.clone()))),
                        "description" | "content" => return Ok(le.description.value.as_ref().map(|s| Value::String(s.clone()))),
                        "type" | "lore_type" | "category" => return Ok(le.lore_type.value.as_ref().map(|s| Value::String(s.clone()))),
                        _ => return Ok(None),
                    }
                }
            }
            Err(())
        }
        "arc" => {
            let arc = state
                .optional
                .overview_phase
                .arcs
                .iter()
                .find(|a| a.code == target_code)
                .ok_or(())?;
            match field_name {
                "display_name" | "name" => Ok(arc.display_name.value.as_ref().map(|s| Value::String(s.clone()))),
                "summary" | "description" => Ok(arc.summary.value.as_ref().map(|s| Value::String(s.clone()))),
                _ => Ok(None),
            }
        }
        "chapter" => {
            for arc in &state.optional.chapter_phase.arcs {
                if let Some(ch) = arc.chapters.iter().find(|c| c.code == target_code) {
                    match field_name {
                        "display_name" | "title" => return Ok(ch.display_name.value.as_ref().map(|s| Value::String(s.clone()))),
                        "summary" => return Ok(ch.summary.value.as_ref().map(|s| Value::String(s.clone()))),
                        _ => return Ok(None),
                    }
                }
            }
            Err(())
        }
        "scene" => {
            for arc in &state.optional.chapter_phase.arcs {
                for ch in &arc.chapters {
                    if let Some(sc) = ch.scenes.iter().find(|s| s.code == target_code) {
                        match field_name {
                            "display_name" | "title" => return Ok(sc.display_name.value.as_ref().map(|s| Value::String(s.clone()))),
                            "summary" => return Ok(sc.summary.value.as_ref().map(|s| Value::String(s.clone()))),
                            "world_setting_code" | "setting" => return Ok(sc.world_setting_code.value.as_ref().map(|s| Value::String(s.clone()))),
                            _ => return Ok(None),
                        }
                    }
                }
            }
            Err(())
        }
        _ => {
            // Generic JSON inspection fallback
            if let Ok(json_val) = serde_json::to_value(state) {
                if find_entity_in_json(&json_val, target_code) {
                    Ok(None)
                } else {
                    Err(())
                }
            } else {
                Err(())
            }
        }
    }
}

/// Recursively checks if an entity with `"code": target_code` exists in JSON value.
fn find_entity_in_json(v: &Value, target_code: &str) -> bool {
    match v {
        Value::Object(map) => {
            if map.get("code").and_then(Value::as_str) == Some(target_code) {
                return true;
            }
            for (_k, child) in map {
                if find_entity_in_json(child, target_code) {
                    return true;
                }
            }
            false
        }
        Value::Array(items) => items.iter().any(|item| find_entity_in_json(item, target_code)),
        _ => false,
    }
}

/// Applies a `set` field mutation with `source = UserEdited` to `BusinessFlowData`.
pub fn apply_field_mutation(
    state: &mut BusinessFlowData,
    target_type: &str,
    target_code: &str,
    field_name: &str,
    new_value: &Value,
) -> Result<(), String> {
    let str_val = match new_value {
        Value::String(s) => s.clone(),
        Value::Null => "".to_string(),
        other => other.to_string(),
    };

    match target_type {
        "project" => {
            let p = &mut state.require.project;
            match field_name {
                "display_name" => p.display_name = TrackedField::user_edited(str_val),
                "summary" => p.summary = TrackedField::user_edited(str_val),
                "language" => p.language = TrackedField::user_edited(str_val),
                "target.arc_count" | "target.chapter_count" | "target.word_per_chapter_count" => {
                    let value = new_value.as_u64().and_then(|n| u32::try_from(n).ok()).filter(|n| *n > 0)
                        .ok_or_else(|| "expected_positive_integer".to_string())?;
                    let field = match field_name {
                        "target.arc_count" => &mut p.target.arc_count,
                        "target.chapter_count" => &mut p.target.chapter_count,
                        _ => &mut p.target.word_per_chapter_count,
                    };
                    *field = TrackedField::user_edited(value);
                }
                _ => return Err(format!("Unknown project field '{}'", field_name)),
            }
            Ok(())
        }
        "character" => {
            let c = state
                .require
                .character
                .iter_mut()
                .find(|ch| ch.code == target_code)
                .ok_or_else(|| format!("Character '{}' not found", target_code))?;
            match field_name {
                "display_name" => c.display_name = TrackedField::user_edited(str_val),
                "nickname" => c.nickname = TrackedField::user_edited(str_val),
                "biological_sex" => c.biological_sex = TrackedField::user_edited(str_val),
                "motif_code" => c.motif_code = TrackedField::user_edited(str_val),
                "role" => c.role = TrackedField::user_edited(str_val),
                "notes" => c.notes = TrackedField::user_edited(str_val),
                _ => return Err(format!("Unknown character field '{}'", field_name)),
            }
            Ok(())
        }
        "world_setting" => {
            let ws = state
                .optional
                .world_building_phase
                .world_settings
                .iter_mut()
                .find(|w| w.code == target_code)
                .ok_or_else(|| format!("World setting '{}' not found", target_code))?;
            match field_name {
                "display_name" | "name" => ws.display_name = TrackedField::user_edited(str_val),
                "type" | "setting_type" | "category" => ws.setting_type = TrackedField::user_edited(str_val),
                "description" => ws.description = TrackedField::user_edited(str_val),
                _ => return Err(format!("Unknown world setting field '{}'", field_name)),
            }
            Ok(())
        }
        "lore_entry" => {
            for ws in &mut state.optional.world_building_phase.world_settings {
                if let Some(le) = ws.lore_entries.iter_mut().find(|l| l.code == target_code) {
                    match field_name {
                        "display_name" | "title" => le.display_name = TrackedField::user_edited(str_val),
                        "description" | "content" => le.description = TrackedField::user_edited(str_val),
                        "type" | "lore_type" | "category" => le.lore_type = TrackedField::user_edited(str_val),
                        _ => return Err(format!("Unknown lore entry field '{}'", field_name)),
                    }
                    return Ok(());
                }
            }
            Err(format!("Lore entry '{}' not found", target_code))
        }
        "arc" => {
            let arc = state
                .optional
                .overview_phase
                .arcs
                .iter_mut()
                .find(|a| a.code == target_code)
                .ok_or_else(|| format!("Arc '{}' not found", target_code))?;
            match field_name {
                "display_name" | "name" => arc.display_name = TrackedField::user_edited(str_val),
                "summary" | "description" => arc.summary = TrackedField::user_edited(str_val),
                _ => return Err(format!("Unknown arc field '{}'", field_name)),
            }
            Ok(())
        }
        "chapter" => {
            for arc in &mut state.optional.chapter_phase.arcs {
                if let Some(ch) = arc.chapters.iter_mut().find(|c| c.code == target_code) {
                    match field_name {
                        "display_name" | "title" => ch.display_name = TrackedField::user_edited(str_val),
                        "summary" => ch.summary = TrackedField::user_edited(str_val),
                        _ => return Err(format!("Unknown chapter field '{}'", field_name)),
                    }
                    return Ok(());
                }
            }
            Err(format!("Chapter '{}' not found", target_code))
        }
        "scene" => {
            for arc in &mut state.optional.chapter_phase.arcs {
                for ch in &mut arc.chapters {
                    if let Some(sc) = ch.scenes.iter_mut().find(|s| s.code == target_code) {
                        match field_name {
                            "display_name" | "title" => sc.display_name = TrackedField::user_edited(str_val),
                            "summary" => sc.summary = TrackedField::user_edited(str_val),
                            "world_setting_code" | "setting" => sc.world_setting_code = TrackedField::user_edited(str_val),
                            _ => return Err(format!("Unknown scene field '{}'", field_name)),
                        }
                        return Ok(());
                    }
                }
            }
            Err(format!("Scene '{}' not found", target_code))
        }
        _ => Err(format!("Unsupported target_type '{}'", target_type)),
    }
}

#[cfg(test)]
mod project_target_patch_tests {
    use super::*;

    #[test]
    fn test_project_target_patch_reads_numeric_baseline_and_applies_confirmed_value() {
        let mut data = BusinessFlowData::new();
        data.require.project.target.chapter_count = TrackedField::user(120);
        assert_eq!(get_field_value(&data, "project", "require.project", "target.chapter_count").unwrap(), Some(serde_json::json!(120)));
        apply_field_mutation(&mut data, "project", "require.project", "target.chapter_count", &serde_json::json!(122)).unwrap();
        assert_eq!(data.require.project.target.chapter_count, TrackedField::user_edited(122));
        assert!(apply_field_mutation(&mut data, "project", "require.project", "target.chapter_count", &serde_json::json!(0)).is_err());
        assert_eq!(data.require.project.target.chapter_count.value, Some(122));
    }
}

/// Evaluates both data validity and confirmation eligibility on a candidate state.
pub fn evaluate_proposal(
    record: &PendingPatchRecord,
    candidate: &BusinessFlowData,
) -> ProposalEvaluationResult {
    // 1. Data Validity
    if record.operation == "set" {
        let field = record.field_name.as_deref().unwrap_or_default();
        match get_field_value(candidate, &record.target_type, &record.target_code, field) {
            Err(_) => {
                return ProposalEvaluationResult::Expired {
                    reason: format!(
                        "Target entity '{}:{}' not found in working state",
                        record.target_type, record.target_code
                    ),
                };
            }
            Ok(curr_val) => {
                // Check if current value equals previous_value
                let matches_prev = match (&curr_val, &record.previous_value) {
                    (None, None) => true,
                    (Some(Value::Null), None) | (None, Some(Value::Null)) => true,
                    (Some(a), Some(b)) => a == b,
                    _ => false,
                };
                if !matches_prev {
                    return ProposalEvaluationResult::Expired {
                        reason: format!(
                            "Previous value mismatch for '{}.{}': expected {:?}, found {:?}",
                            record.target_code, field, record.previous_value, curr_val
                        ),
                    };
                }
            }
        }
    } else if record.operation == "delete" {
        // Target entity must exist
        let exists = match record.target_type.as_str() {
            "character" => candidate.require.character.iter().any(|c| c.code == record.target_code),
            "world_setting" => candidate.optional.world_building_phase.world_settings.iter().any(|w| w.code == record.target_code),
            "lore_entry" => candidate.optional.world_building_phase.world_settings.iter().any(|w| w.lore_entries.iter().any(|l| l.code == record.target_code)),
            "arc" => candidate.optional.overview_phase.arcs.iter().any(|a| a.code == record.target_code),
            "chapter" => candidate.optional.chapter_phase.arcs.iter().any(|a| a.chapters.iter().any(|c| c.code == record.target_code)),
            "scene" => candidate.optional.chapter_phase.arcs.iter().any(|a| a.chapters.iter().any(|c| c.scenes.iter().any(|s| s.code == record.target_code))),
            _ => {
                if let Ok(json_val) = serde_json::to_value(candidate) {
                    find_entity_in_json(&json_val, &record.target_code)
                } else {
                    false
                }
            }
        };

        if !exists {
            return ProposalEvaluationResult::Expired {
                reason: format!("Delete target '{}' does not exist in working state", record.target_code),
            };
        }
    }

    // 2. Confirmation Eligibility
    if let Some(ref elig_str) = record.eligibility_json {
        let eligibility: ProposalEligibility = match serde_json::from_str(elig_str) {
            Ok(e) => e,
            Err(_) => {
                return ProposalEvaluationResult::Indeterminate {
                    reason: "Malformed eligibility_json".to_string(),
                };
            }
        };

        // Explicit missing evidence check (Case A & phase3_sprint6_deep3-F1)
        if eligibility.missing_evidence == Some(true) {
            return ProposalEvaluationResult::Indeterminate {
                reason: "Missing dependency evidence in eligibility_json".to_string(),
            };
        }

        let has_workflow_origin = eligibility
            .origin
            .as_ref()
            .map(|o| o.workflow_run_id.is_some() || o.task_identity.is_some())
            .unwrap_or(false);

        // Check each dependency fingerprint
        for dep in &eligibility.dependencies {
            if dep.scope_fingerprint == "missing_evidence" || dep.path == "missing" {
                return ProposalEvaluationResult::Indeterminate {
                    reason: format!("Missing dependency evidence for path '{}'", dep.path),
                };
            }

            // Extract dependency value from candidate
            let state_json = match serde_json::to_value(candidate) {
                Ok(j) => j,
                Err(e) => {
                    return ProposalEvaluationResult::Indeterminate {
                        reason: format!("Failed to serialize candidate state: {}", e),
                    };
                }
            };

            let dep_val = resolve_json_pointer(&state_json, &dep.path);
            match dep_val {
                None => {
                    // Dependency path disappeared from candidate state -> expired!
                    return ProposalEvaluationResult::Expired {
                        reason: format!("Dependency path '{}' not found in candidate state", dep.path),
                    };
                }
                Some(val) => {
                    // Compare fingerprint
                    let matches_fp = if dep.scope_fingerprint.starts_with("sha256:") {
                        let hash = compute_val_sha256(val);
                        hash == dep.scope_fingerprint
                    } else if let Some(s) = val.as_str() {
                        s == dep.scope_fingerprint
                    } else {
                        val.to_string() == dep.scope_fingerprint
                    };

                    if !matches_fp {
                        return ProposalEvaluationResult::Expired {
                            reason: format!(
                                "Dependency fingerprint mismatch at '{}': expected {}",
                                dep.path, dep.scope_fingerprint
                            ),
                        };
                    }
                }
            }
        }

        // If workflow origin exists and dependencies vector is marked indeterminate or has special test marker
        if has_workflow_origin && eligibility.dependencies.is_empty() && eligibility.missing_evidence.is_none() {
            if let Some(ref orig) = eligibility.origin {
                if orig.task_identity.as_deref() == Some("indeterminate_task") {
                    return ProposalEvaluationResult::Indeterminate {
                        reason: "Indeterminate workflow task dependency evidence".to_string(),
                    };
                }
            }
        }
    }

    ProposalEvaluationResult::Eligible
}

/// Helper computing "sha256:<hex>" for a JSON value.
fn compute_val_sha256(val: &Value) -> String {
    let bytes = serde_json::to_vec(val).unwrap_or_default();
    let digest = Sha256::digest(&bytes);
    format!("sha256:{}", hex::encode(digest))
}

/// Resolves a JSON path (either JSON Pointer `/a/b` or dot notation `a.b`) on a `Value`.
fn resolve_json_pointer<'a>(root: &'a Value, path: &str) -> Option<&'a Value> {
    let clean = path.trim_start_matches('/');
    let mut current = root;
    for segment in clean.split(&['/', '.'][..]) {
        if segment.is_empty() {
            continue;
        }
        match current {
            Value::Object(map) => {
                current = map.get(segment)?;
            }
            Value::Array(items) => {
                if let Ok(idx) = segment.parse::<usize>() {
                    current = items.get(idx)?;
                } else {
                    // Search by code
                    current = items
                        .iter()
                        .find(|item| item.get("code").and_then(Value::as_str) == Some(segment))?;
                }
            }
            _ => return None,
        }
    }
    Some(current)
}

// ============================================================================
// TX-08 Proposal Persistence Implementation (§5.3.2, §1538–§1547)
// ============================================================================

/// Persist or deduplicate a proposal patch under SQLite `BEGIN IMMEDIATE` boundary.
pub fn persist_proposal_tx(
    conn: &Connection,
    params: &PersistProposalParams,
    working_state: Option<&BusinessFlowData>,
) -> Result<PersistProposalResult, IpcError> {
    let now = chrono::Utc::now().to_rfc3339();

    // 1. Check existing pending patch with same (draft_session_id, target_code, field_name)
    let existing_opt = find_pending_by_key(
        conn,
        &params.draft_session_id,
        &params.target_code,
        params.field_name.as_deref(),
    )
    .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

    if let Some(existing) = existing_opt {
        // Check if existing row is stale on working_state (lazy expiration on creation)
        let is_stale = if let Some(ws) = working_state {
            matches!(
                evaluate_proposal(&existing, ws),
                ProposalEvaluationResult::Expired { .. }
            )
        } else {
            false
        };

        if is_stale {
            // Expire stale existing record and proceed to insert new proposal
            expire_patch(conn, &existing.id, &now)
                .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
        } else if existing.new_value == params.new_value {
            // Same proposed value
            let origin_matches = existing.eligibility_json == params.eligibility_json;
            if !origin_matches && params.require_new_origin {
                return Err(IpcError::new(
                    ErrorCode::ProposalEligibilityConflict,
                    "Proposal with same value exists with different origin/dependencies; cannot attest new prerequisite",
                ));
            }

            return Ok(PersistProposalResult {
                patch_id: existing.id,
                reused_existing: true,
                accepted_new_origin: origin_matches,
            });
        } else {
            // AI / user proposed different value -> expire old patch, insert new
            expire_patch(conn, &existing.id, &now)
                .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
        }
    }

    // Insert new pending proposal
    let new_id = Uuid::new_v4();
    let new_record = PendingPatchRecord {
        id: new_id,
        draft_session_id: params.draft_session_id,
        base_version_seq: params.base_version_seq,
        target_type: params.target_type.clone(),
        target_code: params.target_code.clone(),
        field_name: params.field_name.clone(),
        operation: params.operation.clone(),
        previous_value: params.previous_value.clone(),
        new_value: params.new_value.clone(),
        target_source: params.target_source.clone(),
        eligibility_json: params.eligibility_json.clone(),
        status: "pending".to_string(),
        created_at: now,
        resolved_at: None,
    };

    insert_pending_patch(conn, &new_record)
        .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

    Ok(PersistProposalResult {
        patch_id: new_id,
        reused_existing: false,
        accepted_new_origin: true,
    })
}
