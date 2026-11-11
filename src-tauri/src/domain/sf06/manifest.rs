//! SF-06 Canonical Manifest & Digest Computation (`REQ-CODEC-003`, `be-core.md` §1512–§1528, `sf06-owner-ratification.md` §38–§42).
//!
//! Provides deterministic canonical hashing for segmented workflow execution:
//! 1. `plan_digest`:
//!    `"sf06:plan:v2:" + hex(SHA256(UTF8("merlink.sf06.plan.v2\0") || JCS({
//!      draft_session_id, workflow_run_id, plan_generation, round_index,
//!      task_count, parallel_limit,
//!      tasks: [{task_index, task_identity, taskSpec, plannerVersion, policyVersion, codecVersion}, ...]
//!    })))`
//! 2. `request_digest`:
//!    `"sf06:req:v2:" + hex(SHA256(UTF8("merlink.sf06.request.v2\0") || JCS({
//!      draft_session_id, workflow_run_id, plan_generation, round_index,
//!      task_count, parallel_limit, plan_digest, base_version_seq, base_data_hash
//!    })))`
//!
//! Key invariants:
//! - Domain separation prefix bytes end with a null terminator (`\0`).
//! - No transient inputs (e.g. `client_request_id`, user prompt, temporary LLM responses, timestamps)
//!   are ever included in digests (`sf06-owner-ratification.md` §38–§42).
//! - Tasks in `plan_digest` are strictly ordered by `task_index` ascending from `0` to `task_count - 1`.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use thiserror::Error;

use crate::domain::fingerprint::canonical_json_value;

/// Domain separation prefix for SF-06 plan digest v2 (`be-core.md` §1515).
pub const DOMAIN_PREFIX_PLAN_V2: &[u8] = b"merlink.sf06.plan.v2\0";

/// Canonical domain separation prefix for SF-06 request digest v2 (`be-core.md` §1520, `sf06-owner-ratification.md` §40).
pub const DOMAIN_PREFIX_REQUEST_V2: &[u8] = b"merlink.sf06.request.v2\0";

/// Short domain separation prefix alias (`merlink.sf06.req.v2\0`).
pub const DOMAIN_PREFIX_REQ_V2: &[u8] = b"merlink.sf06.req.v2\0";

/// Prefix string for SF-06 canonical plan digest v2.
pub const PLAN_DIGEST_PREFIX: &str = "sf06:plan:v2:";

/// Prefix string for SF-06 canonical request digest v2.
pub const REQUEST_DIGEST_PREFIX: &str = "sf06:req:v2:";

/// Errors encountered during manifest validation or digest computation.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ManifestError {
    #[error("Invalid generation: {0} (must be > 0)")]
    InvalidGeneration(i32),

    #[error("Invalid round index: {0} (must be >= 0)")]
    InvalidRoundIndex(i32),

    #[error("Invalid task count: {0} (must be >= 1)")]
    InvalidTaskCount(u32),

    #[error("Invalid parallel limit: {0} (must be >= 1)")]
    InvalidParallelLimit(u32),

    #[error(
        "Task count mismatch: header specifies {expected} tasks, but manifest contains {actual}"
    )]
    TaskCountMismatch { expected: u32, actual: u32 },

    #[error(
        "Task index mismatch at position {position}: expected index {expected}, found {actual}"
    )]
    TaskIndexMismatch {
        position: usize,
        expected: u32,
        actual: u32,
    },

    #[error("Transient input rejected: {field} must not be included in durable digest")]
    TransientInputForbidden { field: String },

    #[error("Digest mismatch for {digest_type}: expected '{expected}', computed '{computed}'")]
    DigestMismatch {
        digest_type: String,
        expected: String,
        computed: String,
    },

    #[error("Canonical serialization error: {0}")]
    Serialization(String),
}

/// A single task specification entry within `ManifestTask`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaskSpec {
    pub phase: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subphase: Option<i32>,
    pub kind: String,
    #[serde(rename = "assignedScope")]
    pub assigned_scope: Value,
    #[serde(default)]
    pub dependencies: Vec<String>,
    #[serde(rename = "taskIdentity", alias = "deterministicTaskIdentity")]
    pub task_identity: String,
}

/// A durable task item inside the round plan manifest (`be-core.md` §1518).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestTask {
    pub task_index: u32,
    pub task_identity: String,
    #[serde(rename = "taskSpec")]
    pub task_spec: Value,
    #[serde(rename = "plannerVersion")]
    pub planner_version: String,
    #[serde(rename = "policyVersion")]
    pub policy_version: String,
    #[serde(rename = "codecVersion")]
    pub codec_version: String,
}

impl ManifestTask {
    /// Constructs a new `ManifestTask`.
    pub fn new(
        task_index: u32,
        task_identity: impl Into<String>,
        task_spec: Value,
        planner_version: impl Into<String>,
        policy_version: impl Into<String>,
        codec_version: impl Into<String>,
    ) -> Self {
        Self {
            task_index,
            task_identity: task_identity.into(),
            task_spec,
            planner_version: planner_version.into(),
            policy_version: policy_version.into(),
            codec_version: codec_version.into(),
        }
    }
}

/// Canonical input required to compute `plan_digest` (`be-core.md` §1515–§1519).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlanDigestInput {
    pub draft_session_id: String,
    pub workflow_run_id: String,
    pub plan_generation: i32,
    pub round_index: i32,
    pub task_count: u32,
    pub parallel_limit: u32,
    pub tasks: Vec<ManifestTask>,
}

impl PlanDigestInput {
    /// Validates internal consistency before hashing.
    pub fn validate(&self) -> Result<(), ManifestError> {
        if self.plan_generation <= 0 {
            return Err(ManifestError::InvalidGeneration(self.plan_generation));
        }
        if self.round_index < 0 {
            return Err(ManifestError::InvalidRoundIndex(self.round_index));
        }
        if self.task_count == 0 {
            return Err(ManifestError::InvalidTaskCount(self.task_count));
        }
        if self.parallel_limit == 0 {
            return Err(ManifestError::InvalidParallelLimit(self.parallel_limit));
        }
        if self.tasks.len() as u32 != self.task_count {
            return Err(ManifestError::TaskCountMismatch {
                expected: self.task_count,
                actual: self.tasks.len() as u32,
            });
        }
        for (i, task) in self.tasks.iter().enumerate() {
            if task.task_index != i as u32 {
                return Err(ManifestError::TaskIndexMismatch {
                    position: i,
                    expected: i as u32,
                    actual: task.task_index,
                });
            }
        }
        Ok(())
    }
}

/// Canonical input required to compute `request_digest` (`be-core.md` §1520–§1524, `sf06-owner-ratification.md` §40).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RequestDigestInput {
    pub draft_session_id: String,
    pub workflow_run_id: String,
    pub plan_generation: i32,
    pub round_index: i32,
    pub task_count: u32,
    pub parallel_limit: u32,
    pub plan_digest: String,
    pub base_version_seq: i64,
    pub base_data_hash: String,
}

impl RequestDigestInput {
    /// Validates internal consistency before hashing.
    pub fn validate(&self) -> Result<(), ManifestError> {
        if self.plan_generation <= 0 {
            return Err(ManifestError::InvalidGeneration(self.plan_generation));
        }
        if self.round_index < 0 {
            return Err(ManifestError::InvalidRoundIndex(self.round_index));
        }
        if self.task_count == 0 {
            return Err(ManifestError::InvalidTaskCount(self.task_count));
        }
        if self.parallel_limit == 0 {
            return Err(ManifestError::InvalidParallelLimit(self.parallel_limit));
        }
        if !self.plan_digest.starts_with(PLAN_DIGEST_PREFIX) {
            return Err(ManifestError::DigestMismatch {
                digest_type: "plan_digest".to_string(),
                expected: format!("{PLAN_DIGEST_PREFIX}<64hex>"),
                computed: self.plan_digest.clone(),
            });
        }
        Ok(())
    }
}

/// Computes canonical `plan_digest` from `PlanDigestInput` (`be-core.md` §1515).
///
/// Output format: `"sf06:plan:v2:<64_lowercase_hex>"`.
pub fn compute_plan_digest(input: &PlanDigestInput) -> Result<String, ManifestError> {
    input.validate()?;

    // Normalize IDs to canonical lowercase
    let normalized_input = PlanDigestInput {
        draft_session_id: input.draft_session_id.trim().to_ascii_lowercase(),
        workflow_run_id: input.workflow_run_id.trim().to_ascii_lowercase(),
        plan_generation: input.plan_generation,
        round_index: input.round_index,
        task_count: input.task_count,
        parallel_limit: input.parallel_limit,
        tasks: input.tasks.clone(),
    };

    let jcs_value = serde_json::to_value(&normalized_input).map_err(|e| {
        ManifestError::Serialization(format!("failed to serialize plan input: {e}"))
    })?;

    let jcs_bytes = canonical_json_value(&jcs_value).map_err(|e| {
        ManifestError::Serialization(format!("failed to JCS canonicalize plan input: {e}"))
    })?;

    let mut hasher = Sha256::new();
    hasher.update(DOMAIN_PREFIX_PLAN_V2);
    hasher.update(&jcs_bytes);
    let digest = hasher.finalize();

    Ok(format!("{PLAN_DIGEST_PREFIX}{}", hex::encode(digest)))
}

/// Computes canonical `request_digest` using SSOT domain separation `b"merlink.sf06.request.v2\0"` (`be-core.md` §1520).
///
/// Output format: `"sf06:req:v2:<64_lowercase_hex>"`.
pub fn compute_request_digest(input: &RequestDigestInput) -> Result<String, ManifestError> {
    compute_request_digest_with_prefix(input, DOMAIN_PREFIX_REQUEST_V2)
}

/// Computes `request_digest` with custom domain prefix (e.g. `DOMAIN_PREFIX_REQ_V2`).
pub fn compute_request_digest_with_prefix(
    input: &RequestDigestInput,
    domain_prefix: &[u8],
) -> Result<String, ManifestError> {
    input.validate()?;

    let normalized_input = RequestDigestInput {
        draft_session_id: input.draft_session_id.trim().to_ascii_lowercase(),
        workflow_run_id: input.workflow_run_id.trim().to_ascii_lowercase(),
        plan_generation: input.plan_generation,
        round_index: input.round_index,
        task_count: input.task_count,
        parallel_limit: input.parallel_limit,
        plan_digest: input.plan_digest.trim().to_string(),
        base_version_seq: input.base_version_seq,
        base_data_hash: input.base_data_hash.trim().to_string(),
    };

    let jcs_value = serde_json::to_value(&normalized_input).map_err(|e| {
        ManifestError::Serialization(format!("failed to serialize request input: {e}"))
    })?;

    let jcs_bytes = canonical_json_value(&jcs_value).map_err(|e| {
        ManifestError::Serialization(format!("failed to JCS canonicalize request input: {e}"))
    })?;

    let mut hasher = Sha256::new();
    hasher.update(domain_prefix);
    hasher.update(&jcs_bytes);
    let digest = hasher.finalize();

    Ok(format!("{REQUEST_DIGEST_PREFIX}{}", hex::encode(digest)))
}

/// Verifies that a given `plan_digest` string matches the recomputed digest from child tasks.
pub fn verify_plan_digest(
    expected_digest: &str,
    input: &PlanDigestInput,
) -> Result<(), ManifestError> {
    let computed = compute_plan_digest(input)?;
    if expected_digest != computed {
        return Err(ManifestError::DigestMismatch {
            digest_type: "plan_digest".to_string(),
            expected: expected_digest.to_string(),
            computed,
        });
    }
    Ok(())
}

/// Verifies that a given `request_digest` string matches the recomputed digest from header fields.
pub fn verify_request_digest(
    expected_digest: &str,
    input: &RequestDigestInput,
) -> Result<(), ManifestError> {
    let computed = compute_request_digest(input)?;
    if expected_digest != computed {
        // Try fallback prefix if needed
        let fallback_computed = compute_request_digest_with_prefix(input, DOMAIN_PREFIX_REQ_V2)?;
        if expected_digest == fallback_computed {
            return Ok(());
        }
        return Err(ManifestError::DigestMismatch {
            digest_type: "request_digest".to_string(),
            expected: expected_digest.to_string(),
            computed,
        });
    }
    Ok(())
}
