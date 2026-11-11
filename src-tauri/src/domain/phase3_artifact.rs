//! Phase-3 Artifact Codec and Validation (`REQ-ART-001`, `core.md` §3.8–§3.9, `be-core.md` §6.4.1, §1200–§1207, §1549).
//!
//! This module implements strongly-typed, versioned artifacts for Phase-3 workflow execution:
//! 1. `WorldProbeReportV1`:
//!    - Produced during Phase-3 Stage 1 (`world_probe`).
//!    - Contains `{schemaVersion: 1, codecVersion: "v1", assignedScope, reportText}`.
//!    - Hashed with SHA-256 over RFC 8785 JCS UTF-8 bytes with domain prefix `merlink.sf06.artifact.v1\0`.
//! 2. `WorldWriteInputV1`:
//!    - Persisted in child evidence (`evidence_json` in TX-04) during Phase-3 Stage 3 (`world_write`).
//!    - Contains `{schemaVersion: 1, codecVersion: "v1", assignedScope, consumedInput, probeRefs}`.
//!    - `probeRefs` is a list of `{roundReceiptId, taskIdentity, artifactHash}` deterministically
//!      sorted by stable `taskIdentity`.
//!    - Validated at write seal time for same draft session, active branch history applicability,
//!      sealed status, and matching artifact hashes (`REQ-ART-001`, `P3S3-B`).
//! 3. Domain separation prefix & budget constraints:
//!    - `ARTIFACT_DOMAIN_PREFIX = b"merlink.sf06.artifact.v1\0"`.
//!    - `DEFAULT_MAX_ARTIFACT_BUDGET = 262_144` bytes (256 KiB).
//!    - Absence of runtime byte budget configuration fails closed with `ArtifactBudgetUnavailable`.
//!    - Payloads exceeding budget fail closed with `ArtifactBudgetExceeded`.
//! 4. Pure Rust implementation with zero network, SQLite, or Tauri dependency.

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use sha2::{Digest, Sha256};
use std::cmp::Ordering;
use thiserror::Error;

/// Domain separation prefix for Phase-3 artifact canonical hashing (`be-core.md` §1549).
/// Exactly 26 bytes: ASCII "merlink.sf06.artifact.v1" followed by a null byte `\0`.
pub const ARTIFACT_DOMAIN_PREFIX: &[u8] = b"merlink.sf06.artifact.v1\0";

/// Default maximum byte budget for RFC 8785 JCS UTF-8 artifact payloads: 262,144 bytes (256 KiB).
pub const DEFAULT_MAX_ARTIFACT_BUDGET: usize = 262_144;

/// Canonical schema version 1 for Phase-3 artifacts.
pub const SCHEMA_VERSION_V1: u32 = 1;

/// Canonical codec version "v1" for Phase-3 artifacts.
pub const CODEC_VERSION_V1: &str = "v1";

/// Error taxonomy for Phase-3 artifact encoding, decoding, budget, and applicability validation.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ArtifactError {
    #[error("Artifact budget unavailable: runtime byte budget configuration is missing")]
    ArtifactBudgetUnavailable,

    #[error("Artifact budget exceeded: payload size {actual_bytes} bytes exceeds budget of {budget_bytes} bytes")]
    ArtifactBudgetExceeded {
        actual_bytes: usize,
        budget_bytes: usize,
    },

    #[error("Unknown schema version: {version}")]
    UnknownSchemaVersion { version: u32 },

    #[error("Unknown codec version: {version}")]
    UnknownCodecVersion { version: String },

    #[error("Probe refs not sorted deterministically by taskIdentity: index {index} ('{prev}') > ('{current}')")]
    UnsortedProbeRefs {
        index: usize,
        prev: String,
        current: String,
    },

    #[error("Restore diverged: probe ref '{task_identity}' on branch '{probe_branch}' cannot authorize world_write on active branch '{active_branch}'")]
    RestoreDivergedBranch {
        task_identity: String,
        probe_branch: String,
        active_branch: String,
    },

    #[error("Draft session mismatch: probe ref '{task_identity}' belongs to draft '{probe_draft}' but active draft is '{active_draft}'")]
    DraftSessionMismatch {
        task_identity: String,
        probe_draft: String,
        active_draft: String,
    },

    #[error("Probe artifact hash mismatch for '{task_identity}': expected '{expected_hash}', found '{actual_hash}'")]
    ProbeHashMismatch {
        task_identity: String,
        expected_hash: String,
        actual_hash: String,
    },

    #[error("Probe artifact for '{task_identity}' is not sealed")]
    UnsealedProbeArtifact { task_identity: String },

    #[error("Missing probe ref for task identity: '{task_identity}'")]
    MissingProbeRef { task_identity: String },

    #[error("Requires replan: {0}")]
    RequiresReplan(String),

    #[error("Contract blocked: {0}")]
    ContractBlocked(String),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Non-finite number at path '{path}'")]
    NonFiniteNumber { path: String },
}

/// A reference entry pointing to an upstream sealed `world_probe` artifact (`be-core.md` §1549).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProbeRefEntry {
    /// Unique receipt ID of the completed probe round (`workflow_round_receipts.id`).
    pub round_receipt_id: String,
    /// Canonical task identity (e.g., `p3:s0:world_probe:arc=arc_01:ch=001..050`).
    pub task_identity: String,
    /// Domain-separated SHA-256 hex digest of the sealed probe artifact.
    pub artifact_hash: String,
}

impl ProbeRefEntry {
    pub fn new(
        round_receipt_id: impl Into<String>,
        task_identity: impl Into<String>,
        artifact_hash: impl Into<String>,
    ) -> Self {
        Self {
            round_receipt_id: round_receipt_id.into(),
            task_identity: task_identity.into(),
            artifact_hash: artifact_hash.into(),
        }
    }
}

impl Ord for ProbeRefEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.task_identity
            .cmp(&other.task_identity)
            .then_with(|| self.round_receipt_id.cmp(&other.round_receipt_id))
            .then_with(|| self.artifact_hash.cmp(&other.artifact_hash))
    }
}

impl PartialOrd for ProbeRefEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Sealed probe record stored in durable draft state or memory, used to check applicability at write seal time.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SealedProbeRecord {
    pub round_receipt_id: String,
    pub task_identity: String,
    pub draft_session_id: String,
    pub branch_id: String,
    pub artifact_hash: String,
    pub is_sealed: bool,
}

impl SealedProbeRecord {
    pub fn new(
        round_receipt_id: impl Into<String>,
        task_identity: impl Into<String>,
        draft_session_id: impl Into<String>,
        branch_id: impl Into<String>,
        artifact_hash: impl Into<String>,
        is_sealed: bool,
    ) -> Self {
        Self {
            round_receipt_id: round_receipt_id.into(),
            task_identity: task_identity.into(),
            draft_session_id: draft_session_id.into(),
            branch_id: branch_id.into(),
            artifact_hash: artifact_hash.into(),
            is_sealed,
        }
    }

    /// Creates a record by computing the domain-separated hash directly from a `WorldProbeReportV1`.
    pub fn from_probe_report(
        round_receipt_id: impl Into<String>,
        task_identity: impl Into<String>,
        draft_session_id: impl Into<String>,
        branch_id: impl Into<String>,
        report: &WorldProbeReportV1,
        is_sealed: bool,
    ) -> Result<Self, ArtifactError> {
        let hash = report.compute_artifact_hash()?;
        Ok(Self {
            round_receipt_id: round_receipt_id.into(),
            task_identity: task_identity.into(),
            draft_session_id: draft_session_id.into(),
            branch_id: branch_id.into(),
            artifact_hash: hash,
            is_sealed,
        })
    }
}

/// Context specifying the active draft session and active branch lineage for applicability checks (`P3S3-B`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DraftBranchContext {
    pub draft_session_id: String,
    pub active_branch_id: String,
    /// List of valid branch IDs in the current branch's direct history (including `active_branch_id`).
    pub valid_branch_history: Vec<String>,
}

impl DraftBranchContext {
    pub fn new(
        draft_session_id: impl Into<String>,
        active_branch_id: impl Into<String>,
        valid_branch_history: Vec<String>,
    ) -> Self {
        let active = active_branch_id.into();
        let mut history = valid_branch_history;
        if !history.contains(&active) {
            history.push(active.clone());
        }
        Self {
            draft_session_id: draft_session_id.into(),
            active_branch_id: active,
            valid_branch_history: history,
        }
    }

    /// Creates a simple context with a single active branch.
    pub fn single_branch(
        draft_session_id: impl Into<String>,
        branch_id: impl Into<String>,
    ) -> Self {
        let b = branch_id.into();
        Self {
            draft_session_id: draft_session_id.into(),
            active_branch_id: b.clone(),
            valid_branch_history: vec![b],
        }
    }
}

/// Phase-3 Stage 1 Producer Artifact (`be-core.md` §1549, `REQ-ART-001`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorldProbeReportV1 {
    pub schema_version: u32,
    pub codec_version: String,
    pub assigned_scope: String,
    pub report_text: String,
}

impl WorldProbeReportV1 {
    pub fn new(assigned_scope: impl Into<String>, report_text: impl Into<String>) -> Self {
        Self {
            schema_version: SCHEMA_VERSION_V1,
            codec_version: CODEC_VERSION_V1.to_string(),
            assigned_scope: assigned_scope.into(),
            report_text: report_text.into(),
        }
    }

    pub fn from_json_str(s: &str) -> Result<Self, ArtifactError> {
        serde_json::from_str(s).map_err(|e| ArtifactError::Serialization(e.to_string()))
    }

    pub fn from_json_slice(b: &[u8]) -> Result<Self, ArtifactError> {
        serde_json::from_slice(b).map_err(|e| ArtifactError::Serialization(e.to_string()))
    }

    /// Validates schema version, codec version, assigned scope, and budget; returns canonical JCS UTF-8 bytes.
    pub fn validate(&self, budget: Option<usize>) -> Result<Vec<u8>, ArtifactError> {
        if self.schema_version != SCHEMA_VERSION_V1 {
            return Err(ArtifactError::UnknownSchemaVersion {
                version: self.schema_version,
            });
        }
        if self.codec_version != CODEC_VERSION_V1 {
            return Err(ArtifactError::UnknownCodecVersion {
                version: self.codec_version.clone(),
            });
        }
        if self.assigned_scope.trim().is_empty() {
            return Err(ArtifactError::Serialization(
                "assignedScope must not be empty".to_string(),
            ));
        }

        let val =
            serde_json::to_value(self).map_err(|e| ArtifactError::Serialization(e.to_string()))?;
        let canonical_bytes = canonical_json_value(&val)?;

        validate_budget(canonical_bytes.len(), budget)?;
        Ok(canonical_bytes)
    }

    /// Serializes to RFC 8785 JCS canonical UTF-8 bytes without enforcing budget.
    pub fn canonical_jcs_bytes(&self) -> Result<Vec<u8>, ArtifactError> {
        let val =
            serde_json::to_value(self).map_err(|e| ArtifactError::Serialization(e.to_string()))?;
        canonical_json_value(&val)
    }

    /// Computes the domain-separated SHA-256 hex digest (`be-core.md` §1549).
    pub fn compute_artifact_hash(&self) -> Result<String, ArtifactError> {
        let canonical_bytes = self.canonical_jcs_bytes()?;
        let mut hasher = Sha256::new();
        hasher.update(ARTIFACT_DOMAIN_PREFIX);
        hasher.update(&canonical_bytes);
        Ok(hex::encode(hasher.finalize()))
    }

    /// Validates budget and returns both canonical JCS bytes and domain-separated SHA-256 hash.
    pub fn encode_and_hash(
        &self,
        budget: Option<usize>,
    ) -> Result<(Vec<u8>, String), ArtifactError> {
        let canonical_bytes = self.validate(budget)?;
        let mut hasher = Sha256::new();
        hasher.update(ARTIFACT_DOMAIN_PREFIX);
        hasher.update(&canonical_bytes);
        let hash = hex::encode(hasher.finalize());
        Ok((canonical_bytes, hash))
    }
}

/// Phase-3 Stage 3 Consumer Input Artifact (`be-core.md` §1200–§1207, §1549, `REQ-ART-001`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorldWriteInputV1 {
    pub schema_version: u32,
    pub codec_version: String,
    pub assigned_scope: String,
    pub consumed_input: serde_json::Value,
    pub probe_refs: Vec<ProbeRefEntry>,
}

impl WorldWriteInputV1 {
    pub fn new(
        assigned_scope: impl Into<String>,
        consumed_input: serde_json::Value,
        mut probe_refs: Vec<ProbeRefEntry>,
    ) -> Self {
        probe_refs.sort();
        Self {
            schema_version: SCHEMA_VERSION_V1,
            codec_version: CODEC_VERSION_V1.to_string(),
            assigned_scope: assigned_scope.into(),
            consumed_input,
            probe_refs,
        }
    }

    pub fn from_json_str(s: &str) -> Result<Self, ArtifactError> {
        serde_json::from_str(s).map_err(|e| ArtifactError::Serialization(e.to_string()))
    }

    pub fn from_json_slice(b: &[u8]) -> Result<Self, ArtifactError> {
        serde_json::from_slice(b).map_err(|e| ArtifactError::Serialization(e.to_string()))
    }

    /// Sorts `probe_refs` deterministically in-place by `taskIdentity` (then `roundReceiptId`, `artifactHash`).
    pub fn sort_probe_refs(&mut self) {
        self.probe_refs.sort();
    }

    /// Verifies if `probe_refs` are in deterministic non-decreasing order by `task_identity`.
    pub fn is_probe_refs_sorted(&self) -> bool {
        self.probe_refs
            .windows(2)
            .all(|w| w[0].task_identity <= w[1].task_identity)
    }

    /// Validates schema version, codec version, assigned scope, deterministic probeRef sorting,
    /// and budget constraint.
    pub fn validate(&self, budget: Option<usize>) -> Result<Vec<u8>, ArtifactError> {
        if self.schema_version != SCHEMA_VERSION_V1 {
            return Err(ArtifactError::UnknownSchemaVersion {
                version: self.schema_version,
            });
        }
        if self.codec_version != CODEC_VERSION_V1 {
            return Err(ArtifactError::UnknownCodecVersion {
                version: self.codec_version.clone(),
            });
        }
        if self.assigned_scope.trim().is_empty() {
            return Err(ArtifactError::Serialization(
                "assignedScope must not be empty".to_string(),
            ));
        }

        // Check deterministic sorting of probe_refs
        for (i, window) in self.probe_refs.windows(2).enumerate() {
            if window[0].task_identity > window[1].task_identity {
                return Err(ArtifactError::UnsortedProbeRefs {
                    index: i + 1,
                    prev: window[0].task_identity.clone(),
                    current: window[1].task_identity.clone(),
                });
            }
        }

        let val =
            serde_json::to_value(self).map_err(|e| ArtifactError::Serialization(e.to_string()))?;
        let canonical_bytes = canonical_json_value(&val)?;

        validate_budget(canonical_bytes.len(), budget)?;
        Ok(canonical_bytes)
    }

    /// Serializes to RFC 8785 JCS canonical UTF-8 bytes without enforcing budget.
    pub fn canonical_jcs_bytes(&self) -> Result<Vec<u8>, ArtifactError> {
        let val =
            serde_json::to_value(self).map_err(|e| ArtifactError::Serialization(e.to_string()))?;
        canonical_json_value(&val)
    }

    /// Computes the domain-separated SHA-256 hex digest (`be-core.md` §1549).
    pub fn compute_artifact_hash(&self) -> Result<String, ArtifactError> {
        let canonical_bytes = self.canonical_jcs_bytes()?;
        let mut hasher = Sha256::new();
        hasher.update(ARTIFACT_DOMAIN_PREFIX);
        hasher.update(&canonical_bytes);
        Ok(hex::encode(hasher.finalize()))
    }

    /// Validates budget and returns both canonical JCS bytes and domain-separated SHA-256 hash.
    pub fn encode_and_hash(
        &self,
        budget: Option<usize>,
    ) -> Result<(Vec<u8>, String), ArtifactError> {
        let canonical_bytes = self.validate(budget)?;
        let mut hasher = Sha256::new();
        hasher.update(ARTIFACT_DOMAIN_PREFIX);
        hasher.update(&canonical_bytes);
        let hash = hex::encode(hasher.finalize());
        Ok((canonical_bytes, hash))
    }

    /// Validates applicability and provenance of all `probe_refs` against sealed draft state (`P3S3-B`).
    ///
    /// Verifies:
    /// 1. `probe_refs` are deterministically ordered.
    /// 2. Each referenced probe artifact exists in `sealed_probes`.
    /// 3. The probe artifact is sealed (`is_sealed == true`).
    /// 4. The probe artifact belongs to the same draft session (`draft_session_id`).
    /// 5. The probe artifact belongs to the active branch history (rejecting restore-diverged branches).
    /// 6. The probe artifact's hash matches the referenced `artifact_hash`.
    pub fn validate_probe_applicability(
        &self,
        context: &DraftBranchContext,
        sealed_probes: &[SealedProbeRecord],
    ) -> Result<(), ArtifactError> {
        // 1. Order check
        for (i, window) in self.probe_refs.windows(2).enumerate() {
            if window[0].task_identity > window[1].task_identity {
                return Err(ArtifactError::UnsortedProbeRefs {
                    index: i + 1,
                    prev: window[0].task_identity.clone(),
                    current: window[1].task_identity.clone(),
                });
            }
        }

        // 2-6. Applicability checks for each reference
        for ref_entry in &self.probe_refs {
            let probe = sealed_probes
                .iter()
                .find(|p| {
                    p.task_identity == ref_entry.task_identity
                        && p.round_receipt_id == ref_entry.round_receipt_id
                })
                .ok_or_else(|| ArtifactError::MissingProbeRef {
                    task_identity: ref_entry.task_identity.clone(),
                })?;

            if !probe.is_sealed {
                return Err(ArtifactError::UnsealedProbeArtifact {
                    task_identity: ref_entry.task_identity.clone(),
                });
            }

            if probe.draft_session_id != context.draft_session_id {
                return Err(ArtifactError::DraftSessionMismatch {
                    task_identity: ref_entry.task_identity.clone(),
                    probe_draft: probe.draft_session_id.clone(),
                    active_draft: context.draft_session_id.clone(),
                });
            }

            if !context.valid_branch_history.contains(&probe.branch_id) {
                return Err(ArtifactError::RestoreDivergedBranch {
                    task_identity: ref_entry.task_identity.clone(),
                    probe_branch: probe.branch_id.clone(),
                    active_branch: context.active_branch_id.clone(),
                });
            }

            if probe.artifact_hash != ref_entry.artifact_hash {
                return Err(ArtifactError::ProbeHashMismatch {
                    task_identity: ref_entry.task_identity.clone(),
                    expected_hash: ref_entry.artifact_hash.clone(),
                    actual_hash: probe.artifact_hash.clone(),
                });
            }
        }

        Ok(())
    }
}

/// Helper to validate runtime artifact budget (`be-core.md` §1549, `CI-02B`).
pub fn validate_budget(bytes_len: usize, budget: Option<usize>) -> Result<(), ArtifactError> {
    let budget_ceiling = budget.ok_or(ArtifactError::ArtifactBudgetUnavailable)?;
    if bytes_len > budget_ceiling {
        return Err(ArtifactError::ArtifactBudgetExceeded {
            actual_bytes: bytes_len,
            budget_bytes: budget_ceiling,
        });
    }
    Ok(())
}

/// Computes the v2 input scope fingerprint (`be-core.md` §6.4.1, §1200–§1207, §1549).
///
/// Formula: `"sf06:v2:" || hex(sha256(rfc8785_jcs({ "inputArtifactHash": hash, "stateSlice": slice })))`
pub fn compute_input_scope_fingerprint_v2(
    state_slice: &serde_json::Value,
    input_artifact_hash: &str,
) -> Result<String, ArtifactError> {
    let mut map = Map::new();
    map.insert(
        "inputArtifactHash".to_string(),
        Value::String(input_artifact_hash.to_string()),
    );
    map.insert("stateSlice".to_string(), state_slice.clone());
    let jcs_bytes = canonical_json_value(&Value::Object(map))?;
    let digest = Sha256::digest(&jcs_bytes);
    Ok(format!("sf06:v2:{}", hex::encode(digest)))
}

/// Validates consumer contract ratification (`CI-02`, `phase3_sprint3_deep0-F1`).
///
/// Fail closed if owner-ratified fixture / contract is missing, refusing to invent schema fields.
pub fn verify_consumer_contract_ratified(
    is_ratified: bool,
    fixture_id: Option<&str>,
) -> Result<(), ArtifactError> {
    if !is_ratified || fixture_id.is_none() {
        return Err(ArtifactError::ContractBlocked(
            "CI-02 / PHASE3_CONSUMER_SHAPE: absent owner-approved consumer contract; fail closed without guessing schema".to_string(),
        ));
    }
    Ok(())
}

/// Validates recovery artifacts when reconstructing input fingerprint (`be-core.md` §1549, `phase3_sprint3_deep0-F2`).
///
/// Missing required artifacts or missing `probeRefs` reject seal and return `RequiresReplan`.
pub fn verify_recovery_artifacts(
    report_artifact: Option<&WorldProbeReportV1>,
    write_input_artifact: Option<&WorldWriteInputV1>,
    budget: Option<usize>,
) -> Result<(), ArtifactError> {
    let budget = budget.ok_or(ArtifactError::ArtifactBudgetUnavailable)?;

    if let Some(report) = report_artifact {
        let bytes = report.validate(Some(budget))?;
        if bytes.len() > budget {
            return Err(ArtifactError::ArtifactBudgetExceeded {
                actual_bytes: bytes.len(),
                budget_bytes: budget,
            });
        }
    }

    if let Some(write_input) = write_input_artifact {
        if write_input.probe_refs.is_empty() {
            return Err(ArtifactError::RequiresReplan(
                "missing required probeRefs on recovery; cannot fabricate completion".to_string(),
            ));
        }
        let bytes = write_input.validate(Some(budget))?;
        if bytes.len() > budget {
            return Err(ArtifactError::ArtifactBudgetExceeded {
                actual_bytes: bytes.len(),
                budget_bytes: budget,
            });
        }
    } else {
        return Err(ArtifactError::RequiresReplan(
            "missing required write input artifact on recovery; cannot fabricate completion"
                .to_string(),
        ));
    }

    Ok(())
}

/// Validates predecessor gate acceptance and baseline lineage (`phase3_sprint3_deep0-F0`).
pub fn verify_p3s3_dispatch_preconditions(
    expected_baseline: &str,
    actual_baseline: &str,
    p1s3_gate_accepted: bool,
    p2s2_gate_accepted: bool,
) -> Result<(), ArtifactError> {
    if !p1s3_gate_accepted {
        return Err(ArtifactError::ContractBlocked(
            "predecessor gate_phase1_sprint3_deep0 is absent or not ACCEPTED".to_string(),
        ));
    }
    if !p2s2_gate_accepted {
        return Err(ArtifactError::ContractBlocked(
            "predecessor gate_phase2_sprint2_deep1 is absent or not ACCEPTED".to_string(),
        ));
    }
    if expected_baseline != actual_baseline {
        return Err(ArtifactError::ContractBlocked(format!(
            "baseline SHA drift detected: expected {expected_baseline}, got {actual_baseline}"
        )));
    }
    Ok(())
}

/// Validate the Owner-approved JSON report before accepting it as a durable probe artifact.
/// `tempKey` is scoped to one report and never becomes a draft entity code.
pub fn validate_probe_candidates(
    report_text: &str,
    existing_world_codes: &[String],
) -> Result<Value, ArtifactError> {
    use std::collections::{BTreeMap, BTreeSet};
    let report: Value = serde_json::from_str(report_text)
        .map_err(|e| ArtifactError::Serialization(e.to_string()))?;
    let object = report.as_object().ok_or_else(|| {
        ArtifactError::Serialization("ProbeCandidatesV1 must be an object".into())
    })?;
    if !object.contains_key("worldSettings")
        || !object.contains_key("loreEntries")
        || object.get("schemaVersion").and_then(Value::as_u64) != Some(1)
    {
        return Err(ArtifactError::Serialization(
            "Invalid ProbeCandidatesV1 version or keys".into(),
        ));
    }
    let worlds = object
        .get("worldSettings")
        .and_then(Value::as_array)
        .ok_or_else(|| ArtifactError::Serialization("worldSettings must be an array".into()))?;
    let lores = object
        .get("loreEntries")
        .and_then(Value::as_array)
        .ok_or_else(|| ArtifactError::Serialization("loreEntries must be an array".into()))?;
    let existing: BTreeSet<&str> = existing_world_codes.iter().map(String::as_str).collect();
    let mut parents = BTreeMap::<&str, Option<&str>>::new();
    for item in worlds {
        let entry = item.as_object().ok_or_else(|| {
            ArtifactError::Serialization("World candidate must be an object".into())
        })?;
        let get = |key: &str| {
            entry
                .get(key)
                .and_then(Value::as_str)
                .filter(|s| !s.trim().is_empty())
        };
        let key = get("tempKey")
            .ok_or_else(|| ArtifactError::Serialization("Missing world tempKey".into()))?;
        let access_value = entry
            .get("access")
            .and_then(Value::as_str)
            .map(str::trim)
            .filter(|value| !value.is_empty());
        if get("displayName").is_none()
            || !matches!(
                get("type"),
                Some(
                    "REALM"
                        | "WORLD"
                        | "REGION"
                        | "COUNTRY"
                        | "CITY"
                        | "TOWN"
                        | "FACILITY"
                        | "STREET"
                        | "ROOM"
                )
            )
            || access_value.is_some()
                && !matches!(
                    access_value,
                    Some("PUBLIC" | "PRIVATE" | "SECRET" | "LOCKED" | "GUARDED" | "RESTRICTED")
                )
        {
            return Err(ArtifactError::Serialization(
                "Invalid world candidate fields or enum".into(),
            ));
        }
        let parent = get("parentTempKey");
        let external = get("existingParentCode");
        if parent.is_some() && external.is_some()
            || external.is_some_and(|code| !existing.contains(code))
            || parents.insert(key, parent).is_some()
        {
            return Err(ArtifactError::Serialization(
                "Invalid or duplicate world parent reference".into(),
            ));
        }
    }
    for key in parents.keys() {
        let mut seen = BTreeSet::new();
        let mut next = Some(*key);
        while let Some(current) = next {
            if !seen.insert(current) {
                return Err(ArtifactError::Serialization("World parent cycle".into()));
            }
            next = match parents.get(current) {
                Some(parent) => *parent,
                None => {
                    return Err(ArtifactError::Serialization(
                        "Missing world parentTempKey".into(),
                    ))
                }
            };
        }
    }
    let mut lore_keys = BTreeSet::new();
    for item in lores {
        let entry = item.as_object().ok_or_else(|| {
            ArtifactError::Serialization("Lore candidate must be an object".into())
        })?;
        let get = |key: &str| {
            entry
                .get(key)
                .and_then(Value::as_str)
                .filter(|s| !s.trim().is_empty())
        };
        let key = get("tempKey")
            .ok_or_else(|| ArtifactError::Serialization("Missing lore tempKey".into()))?;
        let world = get("worldTempKey");
        let external = get("existingWorldCode");
        if !lore_keys.insert(key)
            || get("displayName").is_none()
            || !matches!(
                get("type"),
                Some("CULTURE" | "HISTORY" | "FACTION" | "ITEM")
            )
            || !entry.get("data").is_some_and(Value::is_object)
            || world.is_some() == external.is_some()
            || world.is_some_and(|code| !parents.contains_key(code))
            || external.is_some_and(|code| !existing.contains(code))
        {
            return Err(ArtifactError::Serialization(
                "Invalid lore candidate or world reference".into(),
            ));
        }
    }
    Ok(report)
}

// ============================================================================
// RFC 8785 JSON Canonicalization Scheme (JCS) Implementation
// ============================================================================

/// Serializes a `serde_json::Value` to UTF-8 bytes according to RFC 8785 (JCS).
pub fn canonical_json_value(value: &Value) -> Result<Vec<u8>, ArtifactError> {
    let mut buf = Vec::with_capacity(256);
    write_jcs_value(value, "$", &mut buf)?;
    Ok(buf)
}

fn write_jcs_value(value: &Value, path: &str, out: &mut Vec<u8>) -> Result<(), ArtifactError> {
    match value {
        Value::Null => {
            out.extend_from_slice(b"null");
            Ok(())
        }
        Value::Bool(true) => {
            out.extend_from_slice(b"true");
            Ok(())
        }
        Value::Bool(false) => {
            out.extend_from_slice(b"false");
            Ok(())
        }
        Value::Number(num) => write_jcs_number(num, path, out),
        Value::String(s) => {
            write_jcs_string(s, out);
            Ok(())
        }
        Value::Array(items) => {
            out.push(b'[');
            for (idx, item) in items.iter().enumerate() {
                if idx > 0 {
                    out.push(b',');
                }
                let child_path = format!("{path}[{idx}]");
                write_jcs_value(item, &child_path, out)?;
            }
            out.push(b']');
            Ok(())
        }
        Value::Object(map) => write_jcs_object(map, path, out),
    }
}

fn write_jcs_object(
    map: &Map<String, Value>,
    path: &str,
    out: &mut Vec<u8>,
) -> Result<(), ArtifactError> {
    let mut entries: Vec<(&String, &Value)> = map.iter().collect();
    entries.sort_by(|(k1, _), (k2, _)| cmp_utf16_code_units(k1, k2));

    out.push(b'{');
    for (idx, (key, val)) in entries.iter().enumerate() {
        if idx > 0 {
            out.push(b',');
        }
        write_jcs_string(key, out);
        out.push(b':');
        let child_path = format!("{path}.{key}");
        write_jcs_value(val, &child_path, out)?;
    }
    out.push(b'}');
    Ok(())
}

/// Compares two Rust UTF-8 strings by their UTF-16 code unit sequences as required by RFC 8785 §3.2.3.
fn cmp_utf16_code_units(a: &str, b: &str) -> Ordering {
    let mut it_a = a.encode_utf16();
    let mut it_b = b.encode_utf16();
    loop {
        match (it_a.next(), it_b.next()) {
            (Some(ua), Some(ub)) => match ua.cmp(&ub) {
                Ordering::Equal => continue,
                non_eq => return non_eq,
            },
            (None, Some(_)) => return Ordering::Less,
            (Some(_), None) => return Ordering::Greater,
            (None, None) => return Ordering::Equal,
        }
    }
}

fn write_jcs_string(s: &str, out: &mut Vec<u8>) {
    out.push(b'"');
    for ch in s.chars() {
        match ch {
            '"' => out.extend_from_slice(b"\\\""),
            '\\' => out.extend_from_slice(b"\\\\"),
            '\u{0008}' => out.extend_from_slice(b"\\b"),
            '\u{0009}' => out.extend_from_slice(b"\\t"),
            '\u{000A}' => out.extend_from_slice(b"\\n"),
            '\u{000C}' => out.extend_from_slice(b"\\f"),
            '\u{000D}' => out.extend_from_slice(b"\\r"),
            c if (c as u32) <= 0x001F => {
                let code = c as u32;
                let escaped = format!("\\u{code:04x}");
                out.extend_from_slice(escaped.as_bytes());
            }
            c => {
                let mut utf8_buf = [0u8; 4];
                let encoded = c.encode_utf8(&mut utf8_buf);
                out.extend_from_slice(encoded.as_bytes());
            }
        }
    }
    out.push(b'"');
}

fn write_jcs_number(
    num: &serde_json::Number,
    path: &str,
    out: &mut Vec<u8>,
) -> Result<(), ArtifactError> {
    if let Some(i) = num.as_i64() {
        out.extend_from_slice(i.to_string().as_bytes());
        return Ok(());
    }
    if let Some(u) = num.as_u64() {
        out.extend_from_slice(u.to_string().as_bytes());
        return Ok(());
    }
    if let Some(f) = num.as_f64() {
        if !f.is_finite() {
            return Err(ArtifactError::NonFiniteNumber {
                path: path.to_string(),
            });
        }
        if f == 0.0 {
            out.push(b'0');
            return Ok(());
        }
        if f.fract() == 0.0 && f.abs() < 1e21 {
            let formatted = format!("{f:.0}");
            out.extend_from_slice(formatted.as_bytes());
            return Ok(());
        }
        out.extend_from_slice(f.to_string().as_bytes());
        return Ok(());
    }
    Err(ArtifactError::NonFiniteNumber {
        path: path.to_string(),
    })
}

#[cfg(test)]
mod probe_candidates_tests {
    use super::*;

    #[test]
    fn test_probe_candidates_validate_parent_and_lore_references() {
        let valid = r#"{"schemaVersion":1,"worldSettings":[{"tempKey":"w1","displayName":"City","type":"CITY"},{"tempKey":"w2","parentTempKey":"w1","displayName":"Library","type":"FACILITY"}],"loreEntries":[{"tempKey":"l1","worldTempKey":"w2","displayName":"Fire","type":"HISTORY","data":{}}]}"#;
        assert!(validate_probe_candidates(valid, &[]).is_ok());
        assert!(validate_probe_candidates(
            &valid.replace("\"parentTempKey\":\"w1\"", "\"parentTempKey\":\"missing\""),
            &[]
        )
        .is_err());
        assert!(validate_probe_candidates(
            &valid.replace("\"parentTempKey\":\"w1\"", "\"parentTempKey\":\"w2\""),
            &[]
        )
        .is_err());
        assert!(validate_probe_candidates(
            &valid.replace("\"worldTempKey\":\"w2\"", "\"worldTempKey\":\"missing\""),
            &[]
        )
        .is_err());
        assert!(validate_probe_candidates(
            &valid.replace("\"type\":\"CITY\"", "\"type\":\"UNKNOWN\""),
            &[]
        )
        .is_err());
    }
}
