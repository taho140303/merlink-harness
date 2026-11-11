//! Authoritative ErrorCode, IpcError, and error propagation interfaces (`REQ-IPC-001`, `CI-04`).
//!
//! Backend separates errors into two distinct categories:
//! 1. Business rejection signals in tool execution (`ToolRejection`):
//!    AI tool call with invalid parameters, non-existent code, entity invariant violation,
//!    or deletion blocked by foreign key reference. Returned as `Ok(ToolExecutionOutput)` with
//!    status `"rejected"`.
//! 2. Infrastructure and Command-level IPC errors (`AppError` / `IpcError`):
//!    System-level errors (SQLite I/O, lock timeout, seal failure, provider unreachable, etc.)
//!    represented as `IpcError` with stable `ErrorCode` identifiers.

use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// Authoritative stable error code identifiers (`be-core.md` §1.5, `CI-04`).
///
/// Contains all 22 base variants plus SF-06 / B+ / migration / artifact / workflow variants
/// ratified in `contract-ledger.md` (CI-04).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
pub enum ErrorCode {
    // --- 22 Base variants (be-core.md §1.5) ---
    /// Quá hạn chờ khóa ghi draft (DRAFT_LOCK_TIMEOUT)
    DraftBusy,
    /// Draft đang ở trạng thái committing hoặc committed
    DraftNotEditable,
    /// Mở lại draft khi đã có nội dung cảnh được viết
    DraftReopenForbidden,
    /// Bấm Start/Commit khi còn pending_patches ở trạng thái pending
    PendingPatchesExist,
    /// Vi phạm bất biến khi confirm_phase hoặc commit_draft
    ValidationFailed,
    /// Trùng code thực thể với DB khi commit_draft
    CodeCollisionOnCommit,
    /// Niêm phong draft_versions thất bại sau khi đã thử lại 1 lần
    VersionSealFailed,
    /// Trùng seal_key nhưng khác data_hash hoặc metadata (SF-01)
    SealKeyPayloadMismatch,
    /// Trùng client_save_id nhưng khác payload_hash (SF-02)
    IdempotencyKeyPayloadMismatch,
    /// Biên nhận workflow không khớp fingerprint hiện tại (SF-06)
    StaleWorkflowReceipt,
    /// Thao tác viết/Accept/Save chạm cảnh có is_locked == true (LOCK-04/05)
    SceneLocked,
    /// AI cố sinh nội dung cho cảnh ngoài EditScope (SCOPE-01)
    OutOfEditScope,
    /// Cố Accept một generation đã bị hủy do Lock giữa chừng (Q4.6)
    GenerationInvalidatedByLock,
    /// expected_version_number != MAX(scene_versions.version_number) (Q4.2)
    OccVersionMismatch,
    /// AI cố gọi tool dành riêng cho UI/CLI
    UnauthorizedCaller,
    /// Không tìm thấy bản ghi theo ID hoặc code
    EntityNotFound,
    /// Không kết nối được Ollama / OpenCode / Model Provider
    ProviderUnreachable,
    /// Thiếu hoặc sai API Key
    ProviderAuthFailed,
    /// Phiên bản PRAGMA user_version của DB cao hơn phiên bản ứng dụng hỗ trợ (§3.4)
    UnsupportedSchemaVersion,
    /// Lỗi khi chạy migration hoặc đối soát cấu trúc schema lúc khởi động (§3.4, §3.5)
    MigrationFailed,
    /// Lỗi SQLite nội bộ
    DatabaseError,
    /// Lỗi bất ngờ khác
    InternalError,

    // --- SF-06 / B+ / Migration / Artifact / Workflow variants (CI-04 ratified) ---
    /// Draft baseline không sẵn sàng hoặc thiếu version gốc cho workflow (§6.4.7)
    WorkflowBaselineUnavailable,
    /// Transition trạng thái hợp lệ nhưng không thể mã hóa theo codec (§6.4.7)
    EvidenceCodecUnsupported,
    /// Codec của bằng chứng không được hỗ trợ (§6.4.7)
    UnsupportedEvidenceCodec,
    /// Đột biến trái thẩm quyền của role / subagent (§6.4.7)
    UnauthorizedTransition,
    /// Tiền điều kiện của patch không khớp với trạng thái trước (§6.4.7)
    PatchPreconditionFailed,
    /// Patch đã quá hạn hiệu lực hoặc đối tượng đích đã bị thay đổi / xóa (§6.4.7)
    PatchExpired,
    /// Xung đột điều kiện hợp lệ của đề xuất patch (§6.4.7)
    ProposalEligibilityConflict,
    /// Không thể xác định tính hợp lệ của đề xuất patch (§6.4.7, §10.2.4)
    ProposalEligibilityIndeterminate,
    /// Thiếu cấu hình định mức dung lượng artifact runtime (§6.4.7)
    ArtifactBudgetUnavailable,
    /// Dung lượng artifact vượt quá định mức cấu hình (§6.4.7)
    ArtifactBudgetExceeded,
    /// Vi phạm bất biến lập lịch hoặc tính toán của workflow (§12.3 WF-C05..07, §14 OTI-05)
    WorkflowInvariantViolation,
    /// Xác nhận patch không được phép với quyền hiện tại (§10.2.4)
    ConfirmationNotAuthorized,
    /// Codec không rõ hoặc bằng chứng thiếu, đòi hỏi lập kế hoạch lại (§6.4.7, §10.2.5)
    RequiresReplan,
}

impl std::fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Structured IPC Error response across Tauri command boundaries (`REQ-IPC-001`).
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, TS)]
#[serde(rename_all = "camelCase")]
pub struct IpcError {
    /// Machine-readable error code enum.
    pub code: ErrorCode,
    /// Human-readable explanation suitable for debug/logging or UI presentation.
    pub message: String,
    /// Structured contextual payload for the specific error (e.g. conflicting keys, field issues).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
    /// Whether the client may safely retry this operation without manual intervention.
    pub retryable: bool,
}

impl IpcError {
    /// Construct a non-retryable IPC error without extra details.
    pub fn new(code: ErrorCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            details: None,
            retryable: false,
        }
    }

    /// Construct a retryable IPC error.
    pub fn retryable(code: ErrorCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            details: None,
            retryable: true,
        }
    }

    /// Construct an IPC error with structured details.
    pub fn with_details(
        code: ErrorCode,
        message: impl Into<String>,
        details: serde_json::Value,
    ) -> Self {
        Self {
            code,
            message: message.into(),
            details: Some(details),
            retryable: false,
        }
    }

    /// Construct a retryable IPC error with structured details.
    pub fn retryable_with_details(
        code: ErrorCode,
        message: impl Into<String>,
        details: serde_json::Value,
    ) -> Self {
        Self {
            code,
            message: message.into(),
            details: Some(details),
            retryable: true,
        }
    }
}

impl std::fmt::Display for IpcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{:?}] {} (retryable={})",
            self.code, self.message, self.retryable
        )
    }
}

impl std::error::Error for IpcError {}

/// Business rejection signal in AI Tool Execution (`core.md` §1.4.7, §3.8, `be-core.md` §1.5).
///
/// Handlers return `Ok(ToolRejection)` when AI calls a tool with invalid enum, nonexistent code,
/// entity invariant violation, or referenced entity in use.
/// Must NOT return `Err(ToolInfraError)` for business rejections.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct ToolRejection {
    /// Fixed status tag indicating business rejection.
    pub status: String, // Always "rejected"
    /// Structured error identifier or explanation.
    pub error: String,
    /// The input value that triggered the rejection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<serde_json::Value>,
    /// Optional candidate suggestions for AI auto-correction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub candidates: Option<Vec<String>>,
}

impl ToolRejection {
    pub fn new(error: impl Into<String>) -> Self {
        Self {
            status: "rejected".to_string(),
            error: error.into(),
            input: None,
            candidates: None,
        }
    }

    pub fn with_input(error: impl Into<String>, input: serde_json::Value) -> Self {
        Self {
            status: "rejected".to_string(),
            error: error.into(),
            input: Some(input),
            candidates: None,
        }
    }

    pub fn with_candidates(
        error: impl Into<String>,
        input: serde_json::Value,
        candidates: Vec<String>,
    ) -> Self {
        Self {
            status: "rejected".to_string(),
            error: error.into(),
            input: Some(input),
            candidates: Some(candidates),
        }
    }
}

/// Infrastructure errors encountered during tool execution or backend processing.
///
/// Distinct from `ToolRejection`: tool handlers only return `Err(ToolInfraError)`
/// on fatal I/O, database unavailability, network loss, or unrecoverable lock timeout.
#[derive(Debug, thiserror::Error)]
pub enum ToolInfraError {
    #[error("Database infrastructure error: {0}")]
    Database(String),
    #[error("Operation timeout: {0}")]
    Timeout(String),
    #[error("Draft lock acquisition timeout: {0}")]
    Lock(String),
    #[error("Provider network infrastructure error: {0}")]
    Provider(String),
    #[error("Internal infrastructure failure: {0}")]
    Internal(String),
}

/// Top-level backend application error convertible to `IpcError`.
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("{0}")]
    Ipc(#[from] IpcError),

    #[error("Tool infrastructure failure: {0}")]
    ToolInfra(#[from] ToolInfraError),

    #[error("SQLite database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Contract blocked: {reason}")]
    ContractBlocked { reason: String },

    #[error("Internal application error: {0}")]
    Internal(String),
}

impl From<AppError> for IpcError {
    fn from(err: AppError) -> Self {
        match err {
            AppError::Ipc(ipc_err) => ipc_err,
            AppError::ToolInfra(ToolInfraError::Database(msg)) => {
                IpcError::new(ErrorCode::DatabaseError, msg)
            }
            AppError::ToolInfra(ToolInfraError::Timeout(msg)) => {
                IpcError::retryable(ErrorCode::DraftBusy, msg)
            }
            AppError::ToolInfra(ToolInfraError::Lock(msg)) => {
                IpcError::retryable(ErrorCode::DraftBusy, msg)
            }
            AppError::ToolInfra(ToolInfraError::Provider(msg)) => {
                IpcError::retryable(ErrorCode::ProviderUnreachable, msg)
            }
            AppError::ToolInfra(ToolInfraError::Internal(msg)) => {
                IpcError::new(ErrorCode::InternalError, msg)
            }
            AppError::Database(sql_err) => {
                IpcError::new(ErrorCode::DatabaseError, sql_err.to_string())
            }
            AppError::Json(json_err) => {
                IpcError::new(ErrorCode::ValidationFailed, json_err.to_string())
            }
            AppError::ContractBlocked { reason } => IpcError::new(
                ErrorCode::ValidationFailed,
                format!("CONTRACT_BLOCKED: {reason}"),
            ),
            AppError::Internal(msg) => IpcError::new(ErrorCode::InternalError, msg),
        }
    }
}
