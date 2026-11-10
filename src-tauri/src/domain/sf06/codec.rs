//! SF-06 B+ Codec v2: Pure Atomic Clone Patch Replay & Canonical Diff (`REQ-CODEC-002`, `be-core.md` §1528–§1532).
//!
//! Provides:
//! 1. `PatchV2` and `EvidenceJsonV2` typed contracts (`schemaVersion = 2`, `codecVersion = "v2"`).
//! 2. `canonical_diff(s_before, s_after) -> Result<PatchV2, ErrorCode>`:
//!    Generates canonical diff with typed operations (`PutField`, `InsertEntity`, `DeleteEntity`,
//!    `SetCounter`, `ReplaceOrderedArray`) sorted canonically by path and op tag.
//! 3. `replay(s_before, patch) -> Result<BusinessFlowData, ErrorCode>`:
//!    Applies atomic clone patch to `s_before`, checking preconditions on `s_before`, detecting
//!    write path and ancestor/descendant conflicts, preserving semantic array ordering, accepting
//!    independent reference-null cascades, and validating the final state.
//! 4. `decode_evidence(evidence_val) -> Result<EvidenceJsonV2, ErrorCode>`:
//!    Fails closed with `ErrorCode::UnsupportedEvidenceCodec` for unknown codec or schema versions.

use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
use std::collections::{BTreeMap, BTreeSet, HashSet};

use crate::domain::business_flow::{BusinessFlowData, TrackedField};
use crate::domain::fingerprint::{
    state_data_hash, states_are_semantically_equal,
};
use crate::error::ErrorCode;

/// Authoritative schema version for SF-06 v2 codec evidence.
pub const SCHEMA_VERSION_V2: u32 = 2;

/// Authoritative codec version identifier for SF-06 v2 codec.
pub const CODEC_VERSION_V2: &str = "v2";

/// Canonical typed operations supported by SF-06 Pure Codec v2 (`be-core.md` §1530).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "op")]
pub enum PatchOp {
    /// Update or set a specific field at `path`.
    #[serde(rename = "PutField", alias = "put_field")]
    PutField {
        path: String,
        precondition: Value,
        value: Value,
    },
    /// Insert a new entity into a collection at `path`.
    #[serde(rename = "InsertEntity", alias = "insert_entity")]
    InsertEntity {
        path: String,
        entity_code: String,
        value: Value,
    },
    /// Delete an existing entity from a collection at `path`.
    #[serde(rename = "DeleteEntity", alias = "delete_entity")]
    DeleteEntity {
        path: String,
        entity_code: String,
    },
    /// Update a monotone parent counter in `_counters`.
    #[serde(rename = "SetCounter", alias = "set_counter")]
    SetCounter {
        counter_key: String,
        old_value: u64,
        new_value: u64,
    },
    /// Replace an ordered array preserving semantic order without sorting.
    #[serde(rename = "ReplaceOrderedArray", alias = "replace_ordered_array")]
    ReplaceOrderedArray {
        path: String,
        precondition: Value,
        value: Value,
    },
}

impl PatchOp {
    /// Returns the canonical path affected by this operation for sorting and conflict detection.
    pub fn canonical_path(&self) -> String {
        match self {
            Self::PutField { path, .. } => path.clone(),
            Self::InsertEntity { path, entity_code, .. } => format!("{}/{}", path.trim_end_matches('/'), entity_code),
            Self::DeleteEntity { path, entity_code } => format!("{}/{}", path.trim_end_matches('/'), entity_code),
            Self::SetCounter { counter_key, .. } => format!("/_counters/{}", counter_key),
            Self::ReplaceOrderedArray { path, .. } => path.clone(),
        }
    }

    /// Returns the stable operation tag string for canonical encoding sort (`be-core.md` §1530).
    pub const fn op_tag(&self) -> &'static str {
        match self {
            Self::PutField { .. } => "PutField",
            Self::InsertEntity { .. } => "InsertEntity",
            Self::DeleteEntity { .. } => "DeleteEntity",
            Self::SetCounter { .. } => "SetCounter",
            Self::ReplaceOrderedArray { .. } => "ReplaceOrderedArray",
        }
    }
}

/// A canonical v2 patch containing an ordered list of pure operations.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct PatchV2 {
    pub ops: Vec<PatchOp>,
}

impl PatchV2 {
    /// Creates an empty patch (no-op).
    pub fn empty() -> Self {
        Self { ops: Vec::new() }
    }

    /// Returns true if the patch contains zero operations.
    pub fn is_empty(&self) -> bool {
        self.ops.is_empty()
    }

    /// Sorts operations by canonical path and op tag for byte-exact deterministic encoding (`be-core.md` §1530).
    pub fn sort_canonical(&mut self) {
        self.ops.sort_by(|a, b| {
            let path_cmp = a.canonical_path().cmp(&b.canonical_path());
            if path_cmp == std::cmp::Ordering::Equal {
                a.op_tag().cmp(b.op_tag())
            } else {
                path_cmp
            }
        });
    }
}

/// Typed child evidence record stored in `workflow_task_receipts.evidence_json` (`be-core.md` §1528).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EvidenceJsonV2 {
    pub schema_version: u32,
    pub codec_version: String,
    pub planner_version: String,
    pub policy_version: String,
    pub task_spec: Value,
    pub pre_state_hash: String,
    pub post_state_hash: String,
    pub had_applied_writes: bool,
    pub effective_state_changed: bool,
    pub patch: Vec<PatchOp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub artifact: Option<Value>,
}

/// Decodes and strictly validates an `evidence_json` value.
///
/// Fails closed with `ErrorCode::UnsupportedEvidenceCodec` if `schemaVersion != 2` or `codecVersion != "v2"`
/// (`be-core.md` §1530, `P3S8-N`, `phase3_sprint8_deep1-F1`). Never normalizes v1 as v2.
pub fn decode_evidence(evidence_val: &Value) -> Result<EvidenceJsonV2, ErrorCode> {
    let schema_ver = evidence_val
        .get("schemaVersion")
        .and_then(Value::as_u64)
        .unwrap_or(0);
    let codec_ver = evidence_val
        .get("codecVersion")
        .and_then(Value::as_str)
        .unwrap_or("");

    if schema_ver != SCHEMA_VERSION_V2 as u64 || codec_ver != CODEC_VERSION_V2 {
        return Err(ErrorCode::UnsupportedEvidenceCodec);
    }

    serde_json::from_value(evidence_val.clone())
        .map_err(|_| ErrorCode::UnsupportedEvidenceCodec)
}

/// Replays evidence against `s_before` verifying pre-state and post-state hashes.
pub fn replay_evidence(
    s_before: &BusinessFlowData,
    evidence: &EvidenceJsonV2,
) -> Result<BusinessFlowData, ErrorCode> {
    if evidence.schema_version != SCHEMA_VERSION_V2 || evidence.codec_version != CODEC_VERSION_V2 {
        return Err(ErrorCode::UnsupportedEvidenceCodec);
    }

    let before_hash = state_data_hash(s_before)
        .map_err(|_| ErrorCode::ValidationFailed)?;
    if before_hash != evidence.pre_state_hash {
        return Err(ErrorCode::PatchPreconditionFailed);
    }

    let patch = PatchV2 {
        ops: evidence.patch.clone(),
    };
    let s_after = replay(s_before, &patch)?;

    let after_hash = state_data_hash(&s_after)
        .map_err(|_| ErrorCode::ValidationFailed)?;
    if after_hash != evidence.post_state_hash {
        return Err(ErrorCode::PatchPreconditionFailed);
    }

    Ok(s_after)
}

/// Resolves a JSON pointer path against a `Value` hierarchy, supporting both standard object keys,
/// array indices, and semantic entity identifiers (`code` or `id`).
pub fn resolve_pointer<'a>(root: &'a Value, path: &str) -> Option<&'a Value> {
    if path.is_empty() || path == "/" {
        return Some(root);
    }
    let trimmed = path.strip_prefix('/').unwrap_or(path);
    let mut current = root;
    for segment in trimmed.split('/') {
        if segment.is_empty() {
            continue;
        }
        match current {
            Value::Object(map) => {
                current = map.get(segment)?;
            }
            Value::Array(items) => {
                if let Ok(idx) = segment.parse::<usize>() {
                    if let Some(item) = items.get(idx) {
                        current = item;
                        continue;
                    }
                }
                let matched = items.iter().find(|item| {
                    item.get("code")
                        .and_then(Value::as_str)
                        .map(|c| c == segment)
                        .unwrap_or(false)
                        || item
                            .get("id")
                            .and_then(Value::as_str)
                            .map(|id| id == segment)
                            .unwrap_or(false)
                });
                current = matched?;
            }
            _ => return None,
        }
    }
    Some(current)
}

/// Resolves a mutable JSON pointer path against a `Value` hierarchy.
pub fn resolve_pointer_mut<'a>(root: &'a mut Value, path: &str) -> Option<&'a mut Value> {
    if path.is_empty() || path == "/" {
        return Some(root);
    }
    let trimmed = path.strip_prefix('/').unwrap_or(path);
    let mut current = root;
    for segment in trimmed.split('/') {
        if segment.is_empty() {
            continue;
        }
        match current {
            Value::Object(map) => {
                current = map.get_mut(segment)?;
            }
            Value::Array(items) => {
                let mut found_idx = None;
                if let Ok(idx) = segment.parse::<usize>() {
                    if idx < items.len() {
                        found_idx = Some(idx);
                    }
                }
                if found_idx.is_none() {
                    for (i, item) in items.iter().enumerate() {
                        if item.get("code").and_then(Value::as_str) == Some(segment)
                            || item.get("id").and_then(Value::as_str) == Some(segment)
                        {
                            found_idx = Some(i);
                            break;
                        }
                    }
                }
                let idx = found_idx?;
                current = items.get_mut(idx)?;
            }
            _ => return None,
        }
    }
    Some(current)
}

/// Checks whether two JSON values are equivalent for TrackedField comparison (treating `{value: null, source: null}`
/// and `{value: null, source: "incomplete"}` or absent source as equal).
fn tracked_field_values_equal(a: &Value, b: &Value) -> bool {
    if a == b {
        return true;
    }
    if let (Value::Object(map_a), Value::Object(map_b)) = (a, b) {
        if map_a.contains_key("value") && map_b.contains_key("value") {
            let val_a = map_a.get("value").unwrap_or(&Value::Null);
            let val_b = map_b.get("value").unwrap_or(&Value::Null);
            if val_a != val_b {
                return false;
            }
            let src_a = map_a
                .get("source")
                .and_then(Value::as_str)
                .unwrap_or("incomplete");
            let src_b = map_b
                .get("source")
                .and_then(Value::as_str)
                .unwrap_or("incomplete");
            return src_a == src_b;
        }
    }
    false
}

/// Applies an atomic clone patch on `s_before`, returning the replayed state (`be-core.md` §1530).
///
/// Rules:
/// - Rejects duplicate write paths (`GV10`, `phase3_sprint8_deep1-F2`).
/// - Rejects updates to children of deleted entities (`GV11`, `phase3_sprint8_deep1-F2`).
/// - Accepts independent reference nullification alongside entity deletion (`GV3`, `GV11`).
/// - Checks all preconditions against `s_before` (`PatchPreconditionFailed`).
/// - Rejects unauthorized mutation of protected fields (`UnauthorizedTransition`, `GV4`, `GV5`).
/// - Normalizes missing optional `TrackedField`s into `{value: null, source: "incomplete"}` (`GV12`).
/// - Validates final state structural invariants (`ValidationFailed`).
pub fn replay(
    s_before: &BusinessFlowData,
    patch: &PatchV2,
) -> Result<BusinessFlowData, ErrorCode> {
    // 1. Conflict detection on patch operations
    let mut write_targets = HashSet::new();
    let mut deleted_entity_prefixes = Vec::new();
    let mut deleted_entity_codes = HashSet::new();

    for op in &patch.ops {
        let canonical_target = op.canonical_path();
        if !write_targets.insert(canonical_target.clone()) {
            // Duplicate write to the same path -> conflict
            return Err(ErrorCode::EvidenceCodecUnsupported);
        }

        if let PatchOp::DeleteEntity { path, entity_code } = op {
            let prefix = format!("{}/{}", path.trim_end_matches('/'), entity_code);
            deleted_entity_prefixes.push(prefix);
            deleted_entity_codes.insert(entity_code.clone());
        }
    }

    // Check ancestor/descendant conflict: modifying a child or property of an entity that is being deleted
    for op in &patch.ops {
        match op {
            PatchOp::PutField { path, .. }
            | PatchOp::InsertEntity { path, .. }
            | PatchOp::ReplaceOrderedArray { path, .. } => {
                for del_prefix in &deleted_entity_prefixes {
                    if path.starts_with(del_prefix) && path.len() > del_prefix.len() && path[del_prefix.len()..].starts_with('/') {
                        return Err(ErrorCode::EvidenceCodecUnsupported);
                    }
                }
            }
            _ => {}
        }
    }

    // 2. Clone `s_before` as a mutable JSON value
    let mut root_val = serde_json::to_value(s_before)
        .map_err(|_| ErrorCode::InternalError)?;

    // 3. Precondition verification and atomic mutation
    for op in &patch.ops {
        match op {
            PatchOp::SetCounter {
                counter_key,
                old_value,
                new_value,
            } => {
                let current_val = s_before.counters.get(counter_key).copied().unwrap_or(0);
                if current_val != *old_value {
                    return Err(ErrorCode::PatchPreconditionFailed);
                }
                let root_obj = root_val
                    .as_object_mut()
                    .ok_or(ErrorCode::ValidationFailed)?;
                let counters_map = root_obj
                    .entry("_counters")
                    .or_insert_with(|| Value::Object(Map::new()))
                    .as_object_mut()
                    .ok_or(ErrorCode::ValidationFailed)?;
                counters_map.insert(counter_key.clone(), json!(new_value));
            }

            PatchOp::PutField {
                path,
                precondition,
                value,
            } => {
                // Verify precondition on before state
                let current_in_before = resolve_pointer(&root_val, path)
                    .ok_or(ErrorCode::PatchPreconditionFailed)?;

                if !tracked_field_values_equal(current_in_before, precondition) {
                    return Err(ErrorCode::PatchPreconditionFailed);
                }

                // Check provenance protection on current field
                if let Some(src_str) = current_in_before.get("source").and_then(Value::as_str) {
                    if (src_str == "user" || src_str == "user_edited") && current_in_before != value {
                        return Err(ErrorCode::UnauthorizedTransition);
                    }
                }

                // Mutate clone
                let target = resolve_pointer_mut(&mut root_val, path)
                    .ok_or(ErrorCode::PatchPreconditionFailed)?;
                *target = value.clone();
            }

            PatchOp::InsertEntity {
                path,
                entity_code,
                value,
            } => {
                let array_val = resolve_pointer_mut(&mut root_val, path)
                    .and_then(Value::as_array_mut)
                    .ok_or(ErrorCode::PatchPreconditionFailed)?;

                // Check code collision in before state
                if array_val.iter().any(|item| {
                    item.get("code").and_then(Value::as_str) == Some(entity_code)
                }) {
                    return Err(ErrorCode::CodeCollisionOnCommit);
                }

                array_val.push(value.clone());
            }

            PatchOp::DeleteEntity { path, entity_code } => {
                let array_val = resolve_pointer_mut(&mut root_val, path)
                    .and_then(Value::as_array_mut)
                    .ok_or(ErrorCode::PatchPreconditionFailed)?;

                let pos = array_val.iter().position(|item| {
                    item.get("code").and_then(Value::as_str) == Some(entity_code)
                });

                match pos {
                    Some(idx) => {
                        array_val.remove(idx);
                    }
                    None => {
                        return Err(ErrorCode::PatchPreconditionFailed);
                    }
                }
            }

            PatchOp::ReplaceOrderedArray {
                path,
                precondition,
                value,
            } => {
                let current_in_before = resolve_pointer(&root_val, path)
                    .ok_or(ErrorCode::PatchPreconditionFailed)?;

                if current_in_before != precondition {
                    return Err(ErrorCode::PatchPreconditionFailed);
                }

                let target = resolve_pointer_mut(&mut root_val, path)
                    .ok_or(ErrorCode::PatchPreconditionFailed)?;
                *target = value.clone();
            }
        }
    }

    // 4. Strict validation of final state structure
    let final_state = BusinessFlowData::from_json_value_strict(&root_val)
        .map_err(|_| ErrorCode::ValidationFailed)?;

    final_state.validate_structure()
        .map_err(|_| ErrorCode::ValidationFailed)?;

    Ok(final_state)
}

/// Generates a canonical `PatchV2` transforming `s_before` into `s_after` (`be-core.md` §1530).
///
/// Ensures:
/// - Exact semantic equality check: if `s_before == s_after`, returns an empty patch (`GV7`).
/// - User/user-edited fields are excluded from direct patch (`GV5`).
/// - Operations are canonically sorted by path and op tag.
/// - Replay on `s_before` yields exact canonical bytes to `s_after` (`P3S8-P`).
pub fn canonical_diff(
    s_before: &BusinessFlowData,
    s_after: &BusinessFlowData,
) -> Result<PatchV2, ErrorCode> {
    if states_are_semantically_equal(s_before, s_after).map_err(|_| ErrorCode::ValidationFailed)? {
        return Ok(PatchV2::empty());
    }

    let mut ops = Vec::new();

    // 1. Counters
    let mut all_counter_keys = BTreeSet::new();
    for k in s_before.counters.keys() {
        all_counter_keys.insert(k.clone());
    }
    for k in s_after.counters.keys() {
        all_counter_keys.insert(k.clone());
    }
    for key in all_counter_keys {
        let old_v = s_before.counters.get(&key).copied().unwrap_or(0);
        let new_v = s_after.counters.get(&key).copied().unwrap_or(0);
        if old_v != new_v {
            ops.push(PatchOp::SetCounter {
                counter_key: key,
                old_value: old_v,
                new_value: new_v,
            });
        }
    }

    // 2. Meta
    if s_before.meta.phase != s_after.meta.phase {
        ops.push(PatchOp::PutField {
            path: "/meta/phase".to_string(),
            precondition: json!(s_before.meta.phase),
            value: json!(s_after.meta.phase),
        });
    }
    if s_before.meta.subphase != s_after.meta.subphase {
        ops.push(PatchOp::PutField {
            path: "/meta/subphase".to_string(),
            precondition: json!(s_before.meta.subphase),
            value: json!(s_after.meta.subphase),
        });
    }

    // 3. User Constraints (ordered array)
    if s_before.user_constraints != s_after.user_constraints {
        ops.push(PatchOp::ReplaceOrderedArray {
            path: "/user_constraints".to_string(),
            precondition: json!(s_before.user_constraints),
            value: json!(s_after.user_constraints),
        });
    }

    // 4. Decision Log (ordered array)
    if s_before.decision_log != s_after.decision_log {
        ops.push(PatchOp::ReplaceOrderedArray {
            path: "/decision_log".to_string(),
            precondition: json!(s_before.decision_log),
            value: json!(s_after.decision_log),
        });
    }

    // 5. Require Section
    // Project fields (respecting provenance)
    diff_tracked_field(
        "/require/project/display_name",
        &s_before.require.project.display_name,
        &s_after.require.project.display_name,
        &mut ops,
    );
    diff_tracked_field(
        "/require/project/summary",
        &s_before.require.project.summary,
        &s_after.require.project.summary,
        &mut ops,
    );
    diff_tracked_field(
        "/require/project/language",
        &s_before.require.project.language,
        &s_after.require.project.language,
        &mut ops,
    );
    diff_tracked_field(
        "/require/project/target/arc_count",
        &s_before.require.project.target.arc_count,
        &s_after.require.project.target.arc_count,
        &mut ops,
    );
    diff_tracked_field(
        "/require/project/target/chapter_count",
        &s_before.require.project.target.chapter_count,
        &s_after.require.project.target.chapter_count,
        &mut ops,
    );
    diff_tracked_field(
        "/require/project/target/word_per_chapter_count",
        &s_before.require.project.target.word_per_chapter_count,
        &s_after.require.project.target.word_per_chapter_count,
        &mut ops,
    );

    // Require Characters
    diff_character_list(
        "/require/character",
        &s_before.require.character,
        &s_after.require.character,
        &mut ops,
    );

    // 6. Optional Section
    // Overview Phase Arcs
    diff_overview_arcs(
        "/optional/overview_phase/arcs",
        &s_before.optional.overview_phase.arcs,
        &s_after.optional.overview_phase.arcs,
        &mut ops,
    );

    // Chapter Phase Arcs & Chapters & Scenes
    // Chapter Phase: one array replacement keeps the round-trip verifier exact while the
    // fine-grained per-entity encoding is still being extended.
    if s_before.optional.chapter_phase.arcs != s_after.optional.chapter_phase.arcs {
        ops.push(PatchOp::ReplaceOrderedArray {
            path: "/optional/chapter_phase/arcs".to_string(),
            precondition: json!(s_before.optional.chapter_phase.arcs),
            value: json!(s_after.optional.chapter_phase.arcs),
        });
    }

    // World Building Phase (world settings + nested lore)
    if s_before.optional.world_building_phase.world_settings
        != s_after.optional.world_building_phase.world_settings
    {
        ops.push(PatchOp::ReplaceOrderedArray {
            path: "/optional/world_building_phase/world_settings".to_string(),
            precondition: json!(s_before.optional.world_building_phase.world_settings),
            value: json!(s_after.optional.world_building_phase.world_settings),
        });
    }

    // Scene Config Phase
    diff_scene_config_phase(
        "/optional/scene_config_phase/arcs",
        &s_before.optional.scene_config_phase.arcs,
        &s_after.optional.scene_config_phase.arcs,
        &mut ops,
    );

    // Character appearance / psychology / life experience phases.
    if s_before.optional.character_appearance_phase.character
        != s_after.optional.character_appearance_phase.character
    {
        ops.push(PatchOp::ReplaceOrderedArray {
            path: "/optional/character_appearance_phase/character".to_string(),
            precondition: json!(s_before.optional.character_appearance_phase.character),
            value: json!(s_after.optional.character_appearance_phase.character),
        });
    }
    if s_before.optional.character_psychological_phase.character
        != s_after.optional.character_psychological_phase.character
    {
        ops.push(PatchOp::ReplaceOrderedArray {
            path: "/optional/character_psychological_phase/character".to_string(),
            precondition: json!(s_before.optional.character_psychological_phase.character),
            value: json!(s_after.optional.character_psychological_phase.character),
        });
    }
    if s_before.optional.character_life_experience_phase.character
        != s_after.optional.character_life_experience_phase.character
    {
        ops.push(PatchOp::ReplaceOrderedArray {
            path: "/optional/character_life_experience_phase/character".to_string(),
            precondition: json!(s_before.optional.character_life_experience_phase.character),
            value: json!(s_after.optional.character_life_experience_phase.character),
        });
    }

    // Relationship Phase
    diff_relationship_phase(
        "/optional/relationship_phase/relationship",
        &s_before.optional.relationship_phase.relationship,
        &s_after.optional.relationship_phase.relationship,
        &mut ops,
    );

    let mut patch = PatchV2 { ops };
    patch.sort_canonical();

    // Verify round-trip replay (excluding protected fields which are deferred to TX-08 proposals)
    let replayed = replay(s_before, &patch)?;
    let mut expected_after = s_after.clone();
    revert_protected_fields(&mut expected_after, s_before);
    if !states_are_semantically_equal(&replayed, &expected_after).map_err(|_| ErrorCode::ValidationFailed)? {
        return Err(ErrorCode::EvidenceCodecUnsupported);
    }

    Ok(patch)
}

fn revert_protected_fields(target: &mut BusinessFlowData, source: &BusinessFlowData) {
    if source.require.project.display_name.is_protected_from_ai_overwrite() {
        target.require.project.display_name = source.require.project.display_name.clone();
    }
    if source.require.project.summary.is_protected_from_ai_overwrite() {
        target.require.project.summary = source.require.project.summary.clone();
    }
    if source.require.project.language.is_protected_from_ai_overwrite() {
        target.require.project.language = source.require.project.language.clone();
    }
    if source.require.project.target.arc_count.is_protected_from_ai_overwrite() {
        target.require.project.target.arc_count = source.require.project.target.arc_count.clone();
    }
    if source.require.project.target.chapter_count.is_protected_from_ai_overwrite() {
        target.require.project.target.chapter_count = source.require.project.target.chapter_count.clone();
    }
    if source.require.project.target.word_per_chapter_count.is_protected_from_ai_overwrite() {
        target.require.project.target.word_per_chapter_count = source.require.project.target.word_per_chapter_count.clone();
    }
}

fn diff_tracked_field<T: Serialize + PartialEq>(
    path: &str,
    before: &TrackedField<T>,
    after: &TrackedField<T>,
    ops: &mut Vec<PatchOp>,
) {
    if before == after {
        return;
    }
    // If the before field was protected (User/UserEdited), AI cannot directly patch it (GV5)
    if before.is_protected_from_ai_overwrite() {
        return;
    }
    ops.push(PatchOp::PutField {
        path: path.to_string(),
        precondition: json!(before),
        value: json!(after),
    });
}

fn diff_character_list(
    path: &str,
    before: &[crate::domain::business_flow::RequireCharacterItem],
    after: &[crate::domain::business_flow::RequireCharacterItem],
    ops: &mut Vec<PatchOp>,
) {
    let before_map: BTreeMap<_, _> = before.iter().map(|c| (&c.code, c)).collect();
    let after_map: BTreeMap<_, _> = after.iter().map(|c| (&c.code, c)).collect();

    // Deleted
    for (code, _) in before_map.iter() {
        if !after_map.contains_key(*code) {
            ops.push(PatchOp::DeleteEntity {
                path: path.to_string(),
                entity_code: (*code).clone(),
            });
        }
    }

    // Inserted
    for (code, ch) in after_map.iter() {
        if !before_map.contains_key(*code) {
            ops.push(PatchOp::InsertEntity {
                path: path.to_string(),
                entity_code: (*code).clone(),
                value: json!(ch),
            });
        }
    }

    // Modified
    for (code, after_ch) in after_map.iter() {
        if let Some(before_ch) = before_map.get(*code) {
            let entity_path = format!("{}/{}", path, code);
            diff_tracked_field(
                &format!("{}/display_name", entity_path),
                &before_ch.display_name,
                &after_ch.display_name,
                ops,
            );
            diff_tracked_field(
                &format!("{}/nickname", entity_path),
                &before_ch.nickname,
                &after_ch.nickname,
                ops,
            );
            diff_tracked_field(
                &format!("{}/biological_sex", entity_path),
                &before_ch.biological_sex,
                &after_ch.biological_sex,
                ops,
            );
            diff_tracked_field(
                &format!("{}/motif_code", entity_path),
                &before_ch.motif_code,
                &after_ch.motif_code,
                ops,
            );
            diff_tracked_field(
                &format!("{}/role", entity_path),
                &before_ch.role,
                &after_ch.role,
                ops,
            );
            diff_tracked_field(
                &format!("{}/notes", entity_path),
                &before_ch.notes,
                &after_ch.notes,
                ops,
            );
        }
    }
}

fn diff_overview_arcs(
    path: &str,
    before: &[crate::domain::business_flow::OverviewArcItem],
    after: &[crate::domain::business_flow::OverviewArcItem],
    ops: &mut Vec<PatchOp>,
) {
    let before_map: BTreeMap<_, _> = before.iter().map(|a| (&a.code, a)).collect();
    let after_map: BTreeMap<_, _> = after.iter().map(|a| (&a.code, a)).collect();

    for (code, _) in before_map.iter() {
        if !after_map.contains_key(*code) {
            ops.push(PatchOp::DeleteEntity {
                path: path.to_string(),
                entity_code: (*code).clone(),
            });
        }
    }

    for (code, arc) in after_map.iter() {
        if !before_map.contains_key(*code) {
            ops.push(PatchOp::InsertEntity {
                path: path.to_string(),
                entity_code: (*code).clone(),
                value: json!(arc),
            });
        }
    }

    for (code, after_arc) in after_map.iter() {
        if let Some(before_arc) = before_map.get(*code) {
            let arc_path = format!("{}/{}", path, code);
            diff_tracked_field(
                &format!("{}/display_name", arc_path),
                &before_arc.display_name,
                &after_arc.display_name,
                ops,
            );
            diff_tracked_field(
                &format!("{}/summary", arc_path),
                &before_arc.summary,
                &after_arc.summary,
                ops,
            );
            diff_tracked_field(
                &format!("{}/position", arc_path),
                &before_arc.position,
                &after_arc.position,
                ops,
            );

            // Event blocks
            let eb_path = format!("{}/motif/event_blocks", arc_path);
            let before_eb: BTreeMap<_, _> = before_arc.motif.event_blocks.iter().map(|e| (&e.code, e)).collect();
            let after_eb: BTreeMap<_, _> = after_arc.motif.event_blocks.iter().map(|e| (&e.code, e)).collect();

            for (eb_code, _) in before_eb.iter() {
                if !after_eb.contains_key(*eb_code) {
                    ops.push(PatchOp::DeleteEntity {
                        path: eb_path.clone(),
                        entity_code: (*eb_code).clone(),
                    });
                }
            }
            for (eb_code, eb) in after_eb.iter() {
                if !before_eb.contains_key(*eb_code) {
                    ops.push(PatchOp::InsertEntity {
                        path: eb_path.clone(),
                        entity_code: (*eb_code).clone(),
                        value: json!(eb),
                    });
                }
            }
        }
    }
}

#[allow(dead_code)]
fn diff_chapter_phase_arcs(
    path: &str,
    before: &[crate::domain::business_flow::ChapterPhaseArcItem],
    after: &[crate::domain::business_flow::ChapterPhaseArcItem],
    ops: &mut Vec<PatchOp>,
) {
    let before_map: BTreeMap<_, _> = before.iter().map(|a| (&a.code, a)).collect();
    let after_map: BTreeMap<_, _> = after.iter().map(|a| (&a.code, a)).collect();

    for (arc_code, _) in before_map.iter() {
        if !after_map.contains_key(*arc_code) {
            ops.push(PatchOp::DeleteEntity {
                path: path.to_string(),
                entity_code: (*arc_code).clone(),
            });
        }
    }

    for (arc_code, arc) in after_map.iter() {
        if !before_map.contains_key(*arc_code) {
            ops.push(PatchOp::InsertEntity {
                path: path.to_string(),
                entity_code: (*arc_code).clone(),
                value: json!(arc),
            });
        }
    }

    for (arc_code, after_arc) in after_map.iter() {
        if let Some(before_arc) = before_map.get(*arc_code) {
            let chapters_path = format!("{}/{}/chapters", path, arc_code);
            let before_ch: BTreeMap<_, _> = before_arc.chapters.iter().map(|c| (&c.code, c)).collect();
            let after_ch: BTreeMap<_, _> = after_arc.chapters.iter().map(|c| (&c.code, c)).collect();

            for (ch_code, _) in before_ch.iter() {
                if !after_ch.contains_key(*ch_code) {
                    ops.push(PatchOp::DeleteEntity {
                        path: chapters_path.clone(),
                        entity_code: (*ch_code).clone(),
                    });
                }
            }
            for (ch_code, ch) in after_ch.iter() {
                if !before_ch.contains_key(*ch_code) {
                    ops.push(PatchOp::InsertEntity {
                        path: chapters_path.clone(),
                        entity_code: (*ch_code).clone(),
                        value: json!(ch),
                    });
                }
            }

            for (ch_code, after_chap) in after_ch.iter() {
                if let Some(before_chap) = before_ch.get(*ch_code) {
                    let scenes_path = format!("{}/{}/scenes", chapters_path, ch_code);
                    let before_sc: BTreeMap<_, _> = before_chap.scenes.iter().map(|s| (&s.code, s)).collect();
                    let after_sc: BTreeMap<_, _> = after_chap.scenes.iter().map(|s| (&s.code, s)).collect();

                    for (sc_code, _) in before_sc.iter() {
                        if !after_sc.contains_key(*sc_code) {
                            ops.push(PatchOp::DeleteEntity {
                                path: scenes_path.clone(),
                                entity_code: (*sc_code).clone(),
                            });
                        }
                    }
                    for (sc_code, sc) in after_sc.iter() {
                        if !before_sc.contains_key(*sc_code) {
                            ops.push(PatchOp::InsertEntity {
                                path: scenes_path.clone(),
                                entity_code: (*sc_code).clone(),
                                value: json!(sc),
                            });
                        }
                    }
                    for (sc_code, after_scene) in after_sc.iter() {
                        if let Some(before_scene) = before_sc.get(*sc_code) {
                            let scene_path = format!("{}/{}", scenes_path, sc_code);
                            diff_tracked_field(
                                &format!("{}/display_name", scene_path),
                                &before_scene.display_name,
                                &after_scene.display_name,
                                ops,
                            );
                            diff_tracked_field(
                                &format!("{}/summary", scene_path),
                                &before_scene.summary,
                                &after_scene.summary,
                                ops,
                            );
                            diff_tracked_field(
                                &format!("{}/world_setting_code", scene_path),
                                &before_scene.world_setting_code,
                                &after_scene.world_setting_code,
                                ops,
                            );
                        }
                    }
                }
            }
        }
    }
}

#[allow(dead_code)]
fn diff_world_settings(
    path: &str,
    before: &[crate::domain::business_flow::WorldSettingItem],
    after: &[crate::domain::business_flow::WorldSettingItem],
    ops: &mut Vec<PatchOp>,
) {
    let before_map: BTreeMap<_, _> = before.iter().map(|w| (&w.code, w)).collect();
    let after_map: BTreeMap<_, _> = after.iter().map(|w| (&w.code, w)).collect();

    for (code, _) in before_map.iter() {
        if !after_map.contains_key(*code) {
            ops.push(PatchOp::DeleteEntity {
                path: path.to_string(),
                entity_code: (*code).clone(),
            });
        }
    }

    for (code, ws) in after_map.iter() {
        if !before_map.contains_key(*code) {
            ops.push(PatchOp::InsertEntity {
                path: path.to_string(),
                entity_code: (*code).clone(),
                value: json!(ws),
            });
        }
    }

    for (code, after_ws) in after_map.iter() {
        if let Some(before_ws) = before_map.get(*code) {
            let ws_path = format!("{}/{}", path, code);
            diff_tracked_field(
                &format!("{}/display_name", ws_path),
                &before_ws.display_name,
                &after_ws.display_name,
                ops,
            );
            diff_tracked_field(
                &format!("{}/description", ws_path),
                &before_ws.description,
                &after_ws.description,
                ops,
            );
            if before_ws.lore_entries != after_ws.lore_entries {
                ops.push(PatchOp::ReplaceOrderedArray {
                    path: format!("{ws_path}/lore_entries"),
                    precondition: json!(before_ws.lore_entries),
                    value: json!(after_ws.lore_entries),
                });
            }
        }
    }
}

fn diff_scene_config_phase(
    path: &str,
    before: &[crate::domain::business_flow::SceneConfigArcItem],
    after: &[crate::domain::business_flow::SceneConfigArcItem],
    ops: &mut Vec<PatchOp>,
) {
    if before == after {
        return;
    }
    ops.push(PatchOp::ReplaceOrderedArray {
        path: path.to_string(),
        precondition: json!(before),
        value: json!(after),
    });
}

#[allow(dead_code)]
fn diff_character_life_experience(
    path: &str,
    before: &[crate::domain::business_flow::CharacterLifeExperienceItem],
    after: &[crate::domain::business_flow::CharacterLifeExperienceItem],
    ops: &mut Vec<PatchOp>,
) {
    let before_map: BTreeMap<_, _> = before.iter().map(|c| (&c.code, c)).collect();
    let after_map: BTreeMap<_, _> = after.iter().map(|c| (&c.code, c)).collect();

    for (ch_code, after_life) in after_map.iter() {
        if let Some(before_life) = before_map.get(*ch_code) {
            let char_path = format!("{}/{}", path, ch_code);
            // Compare memories
            if before_life.memories != after_life.memories {
                for (mem_idx, (b_mem, a_mem)) in before_life.memories.iter().zip(after_life.memories.iter()).enumerate() {
                    if b_mem.source_scene_code != a_mem.source_scene_code {
                        diff_tracked_field(
                            &format!("{}/memories/{}/source_scene_code", char_path, mem_idx),
                            &b_mem.source_scene_code,
                            &a_mem.source_scene_code,
                            ops,
                        );
                    }
                }
                if before_life.memories.len() != after_life.memories.len() {
                    ops.push(PatchOp::ReplaceOrderedArray {
                        path: format!("{}/memories", char_path),
                        precondition: json!(before_life.memories),
                        value: json!(after_life.memories),
                    });
                }
            }
        }
    }
}

fn diff_relationship_phase(
    path: &str,
    before: &[crate::domain::business_flow::RelationshipPhaseItem],
    after: &[crate::domain::business_flow::RelationshipPhaseItem],
    ops: &mut Vec<PatchOp>,
) {
    if before == after {
        return;
    }
    ops.push(PatchOp::ReplaceOrderedArray {
        path: path.to_string(),
        precondition: json!(before),
        value: json!(after),
    });
}
