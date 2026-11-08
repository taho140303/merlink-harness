//! Temporary file storage manager for workflow artifacts (docs/business/be-core.md §1.4, §3.9, §1549).
//!
//! Enforces:
//! - Configured temporary root (`<configured_temp_root>/<draft_session_id>/<phase>/task_<hash>.md`)
//! - Strict UUID validation for `draft_session_id`
//! - Phase bound validation (`1..=6`)
//! - Hard rejection of path traversal (`..`, absolute paths, separators)
//! - REQ-DB-004: Temporary artifacts are ephemeral caches and CANNOT serve as durable completion proof.

use std::fs;
use std::path::{Component, Path, PathBuf};

/// Canonical error type for temporary file store operations.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum TempStoreError {
    #[error("Path traversal rejected: {0}")]
    PathTraversal(String),

    #[error("Invalid draft session ID (canonical UUID required): {0}")]
    InvalidSessionId(String),

    #[error("Invalid phase number: {phase} (must be between 1 and 6)")]
    InvalidPhase { phase: u8 },

    #[error("Invalid task identifier/hash: {0}")]
    InvalidTaskHash(String),

    #[error("Temporary report cannot be used as durable completion proof (REQ-DB-004, be-core.md §1549)")]
    NonAuthoritativeProof,

    #[error("Temporary report not found: {0}")]
    NotFound(String),

    #[error("IO error: {0}")]
    Io(String),
}

impl From<std::io::Error> for TempStoreError {
    fn from(e: std::io::Error) -> Self {
        TempStoreError::Io(e.to_string())
    }
}

/// In-memory descriptor for a temporary report artifact.
///
/// In accordance with REQ-DB-004 and be-core.md §1549, a temporary report file
/// is strictly non-authoritative ephemeral cache and CANNOT provide durable proof of completion.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TempReportArtifact {
    pub draft_session_id: String,
    pub phase: u8,
    pub task_hash: String,
    pub file_path: PathBuf,
}

impl TempReportArtifact {
    /// Invariant REQ-DB-004: Always returns `false`.
    /// A temporary report can never be treated as durable completion.
    pub fn is_durable_completion_proof(&self) -> bool {
        false
    }
}

/// Manager for temporary workflow files under a configured root directory.
pub struct TempStore {
    root_dir: PathBuf,
}

impl TempStore {
    /// Create a new TempStore rooted at `root_dir`.
    pub fn new(root_dir: impl Into<PathBuf>) -> Self {
        Self {
            root_dir: root_dir.into(),
        }
    }

    /// Default canonical temporary storage root under app_data_dir (§1.4).
    pub fn canonical_temp_root(app_data_dir: &Path) -> PathBuf {
        app_data_dir.join("merlink-harness").join("tmp")
    }

    /// Read-only reference to the configured root directory.
    pub fn root_dir(&self) -> &Path {
        &self.root_dir
    }

    /// Validate that `draft_session_id` is a valid 36-char canonical UUID
    /// and contains no path separators or traversal sequences.
    pub fn validate_draft_session_id(draft_session_id: &str) -> Result<uuid::Uuid, TempStoreError> {
        if draft_session_id.contains("..")
            || draft_session_id.contains('/')
            || draft_session_id.contains('\\')
        {
            return Err(TempStoreError::PathTraversal(format!(
                "draft_session_id contains path traversal elements: '{}'",
                draft_session_id
            )));
        }

        if draft_session_id.len() != 36 {
            return Err(TempStoreError::InvalidSessionId(format!(
                "invalid length {}, expected 36 characters",
                draft_session_id.len()
            )));
        }

        uuid::Uuid::parse_str(draft_session_id).map_err(|e| {
            TempStoreError::InvalidSessionId(format!(
                "failed to parse UUID '{}': {}",
                draft_session_id, e
            ))
        })
    }

    /// Validate that `phase` is between 1 and 6 (§1.4.2).
    pub fn validate_phase(phase: u8) -> Result<(), TempStoreError> {
        if !(1..=6).contains(&phase) {
            return Err(TempStoreError::InvalidPhase { phase });
        }
        Ok(())
    }

    /// Validate `task_hash` ensuring it contains no traversal or separator characters.
    pub fn validate_task_hash(task_hash: &str) -> Result<(), TempStoreError> {
        if task_hash.is_empty() {
            return Err(TempStoreError::InvalidTaskHash("empty task hash".to_string()));
        }

        if task_hash.contains("..")
            || task_hash.contains('/')
            || task_hash.contains('\\')
            || task_hash.contains('\0')
        {
            return Err(TempStoreError::PathTraversal(format!(
                "task_hash contains traversal elements: '{}'",
                task_hash
            )));
        }

        // Must be safe characters: alphanumeric, dash, underscore
        if !task_hash
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
        {
            return Err(TempStoreError::InvalidTaskHash(format!(
                "task_hash contains invalid characters: '{}'",
                task_hash
            )));
        }

        Ok(())
    }

    /// Resolve the directory path for a draft session: `<root>/<draft_session_id>/`.
    pub fn resolve_session_dir(&self, draft_session_id: &str) -> Result<PathBuf, TempStoreError> {
        Self::validate_draft_session_id(draft_session_id)?;
        let path = self.root_dir.join(draft_session_id);
        self.verify_no_traversal(&path)?;
        Ok(path)
    }

    /// Resolve the directory path for a phase: `<root>/<draft_session_id>/<phase>/`.
    pub fn resolve_phase_dir(&self, draft_session_id: &str, phase: u8) -> Result<PathBuf, TempStoreError> {
        Self::validate_draft_session_id(draft_session_id)?;
        Self::validate_phase(phase)?;
        let path = self
            .root_dir
            .join(draft_session_id)
            .join(format!("{}", phase));
        self.verify_no_traversal(&path)?;
        Ok(path)
    }

    /// Resolve full canonical report file path:
    /// `<root>/<draft_session_id>/<phase>/task_<task_hash>.md`
    pub fn resolve_report_path(
        &self,
        draft_session_id: &str,
        phase: u8,
        task_hash: &str,
    ) -> Result<PathBuf, TempStoreError> {
        Self::validate_draft_session_id(draft_session_id)?;
        Self::validate_phase(phase)?;
        Self::validate_task_hash(task_hash)?;

        let filename = format!("task_{}.md", task_hash);
        let path = self
            .root_dir
            .join(draft_session_id)
            .join(format!("{}", phase))
            .join(filename);

        self.verify_no_traversal(&path)?;
        Ok(path)
    }

    /// Ensure the path does not escape `root_dir`.
    fn verify_no_traversal(&self, path: &Path) -> Result<(), TempStoreError> {
        for component in path.components() {
            if let Component::ParentDir = component {
                return Err(TempStoreError::PathTraversal(format!(
                    "path contains parent directory component: {:?}",
                    path
                )));
            }
        }

        // Relative traversal check
        if let Ok(rel) = path.strip_prefix(&self.root_dir) {
            for c in rel.components() {
                if let Component::ParentDir = c {
                    return Err(TempStoreError::PathTraversal(format!(
                        "path escapes root via: {:?}",
                        path
                    )));
                }
            }
        }

        Ok(())
    }

    /// Write temporary phase report content to `<root>/<draft_session_id>/<phase>/task_<task_hash>.md`.
    pub fn write_phase_report(
        &self,
        draft_session_id: &str,
        phase: u8,
        task_hash: &str,
        content: &str,
    ) -> Result<TempReportArtifact, TempStoreError> {
        let file_path = self.resolve_report_path(draft_session_id, phase, task_hash)?;

        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(&file_path, content)?;

        Ok(TempReportArtifact {
            draft_session_id: draft_session_id.to_string(),
            phase,
            task_hash: task_hash.to_string(),
            file_path,
        })
    }

    /// Read temporary phase report content.
    pub fn read_phase_report(
        &self,
        draft_session_id: &str,
        phase: u8,
        task_hash: &str,
    ) -> Result<String, TempStoreError> {
        let file_path = self.resolve_report_path(draft_session_id, phase, task_hash)?;

        if !file_path.exists() {
            return Err(TempStoreError::NotFound(file_path.to_string_lossy().to_string()));
        }

        let content = fs::read_to_string(&file_path)?;
        Ok(content)
    }

    /// Read all phase reports in `<root>/<draft_session_id>/<phase>/`.
    /// Returns vector of `(task_hash, content)`.
    pub fn read_phase_reports(
        &self,
        draft_session_id: &str,
        phase: u8,
    ) -> Result<Vec<(String, String)>, TempStoreError> {
        let phase_dir = self.resolve_phase_dir(draft_session_id, phase)?;
        if !phase_dir.exists() {
            return Ok(Vec::new());
        }

        let mut reports = Vec::new();
        for entry in fs::read_dir(&phase_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                if let Some(filename) = path.file_name().and_then(|f| f.to_str()) {
                    if filename.starts_with("task_") && filename.ends_with(".md") {
                        let hash = filename
                            .trim_start_matches("task_")
                            .trim_end_matches(".md");
                        let content = fs::read_to_string(&path)?;
                        reports.push((hash.to_string(), content));
                    }
                }
            }
        }

        reports.sort_by(|a, b| a.0.cmp(&b.0));
        Ok(reports)
    }

    /// Clean temporary files for a draft session: `<root>/<draft_session_id>/`.
    pub fn clean_session(&self, draft_session_id: &str) -> Result<(), TempStoreError> {
        let session_dir = self.resolve_session_dir(draft_session_id)?;
        if session_dir.exists() {
            fs::remove_dir_all(&session_dir)?;
        }
        Ok(())
    }

    /// Enforce REQ-DB-004: Temp report cannot mark task complete or serve as durable proof.
    /// Always returns `Err(TempStoreError::NonAuthoritativeProof)`.
    pub fn verify_durable_completion(&self, _artifact: &TempReportArtifact) -> Result<(), TempStoreError> {
        Err(TempStoreError::NonAuthoritativeProof)
    }
}
