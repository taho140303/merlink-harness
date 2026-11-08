//! LlmProvider trait and model request/streaming interfaces (`REQ-PROVIDER-001`, `§9.1`).

use crate::error::{ErrorCode, IpcError};
use crate::providers::redaction::{redact_sensitive_strings, Redactor};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio_util::sync::CancellationToken;

/// Tool definition offered to LLM for function calling / ReAct loop.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ProviderToolDefinition {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
}

/// Normalized tool call emitted by LLM.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ProviderToolCall {
    pub id: String,
    pub name: String,
    pub arguments: serde_json::Value,
}

/// Message exchange item for provider request.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProviderMessage {
    pub role: String,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "tool_calls")]
    pub tool_calls: Option<Vec<ProviderToolCall>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "tool_call_id")]
    pub tool_call_id: Option<String>,
}

impl ProviderMessage {
    pub fn system(content: impl Into<String>) -> Self {
        Self {
            role: "system".to_string(),
            content: content.into(),
            name: None,
            tool_calls: None,
            tool_call_id: None,
        }
    }

    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: "user".to_string(),
            content: content.into(),
            name: None,
            tool_calls: None,
            tool_call_id: None,
        }
    }

    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: "assistant".to_string(),
            content: content.into(),
            name: None,
            tool_calls: None,
            tool_call_id: None,
        }
    }

    pub fn tool_result(
        tool_call_id: impl Into<String>,
        name: impl Into<String>,
        content: impl Into<String>,
    ) -> Self {
        Self {
            role: "tool".to_string(),
            content: content.into(),
            name: Some(name.into()),
            tool_calls: None,
            tool_call_id: Some(tool_call_id.into()),
        }
    }
}

/// Token usage telemetry.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ProviderUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

/// Chat request payload for planning / ReAct loop.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ProviderChatRequest {
    pub model: String,
    pub messages: Vec<ProviderMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<ProviderToolDefinition>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
}

/// Chat response payload.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ProviderChatResponse {
    pub content: Option<String>,
    pub tool_calls: Vec<ProviderToolCall>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<ProviderUsage>,
}

/// Completion streaming request payload for Scene Generation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ProviderStreamRequest {
    pub model: String,
    pub prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<ProviderMessage>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
}

/// Individual delta chunk received during streaming.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ProviderStreamChunk {
    pub delta: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_reason: Option<String>,
}

/// Summary returned after completion stream terminates.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ProviderStreamSummary {
    pub total_text: String,
    pub chunk_count: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<ProviderUsage>,
}

/// Provider-level domain errors normalized per §9.1.2.
#[derive(Debug, Error)]
pub enum ProviderError {
    #[error("Provider authentication failed ({status}): {message}")]
    AuthFailed { status: u16, message: String },

    #[error("Provider unreachable (status={status:?}, network/timeout): {message}")]
    Unreachable { status: Option<u16>, message: String },

    #[error("Operation cancelled by user/token")]
    Cancelled,

    #[error("Validation failed: {0}")]
    Validation(String),

    #[error("Protocol error: {0}")]
    Protocol(String),

    #[error("Internal provider error: {0}")]
    Internal(String),
}

impl ProviderError {
    /// Convert this error to a sanitized `IpcError`, ensuring all sensitive keys are redacted.
    pub fn to_ipc_error(&self) -> IpcError {
        self.to_ipc_error_with_redactor(Redactor::global())
    }

    /// Convert using a specific redactor instance.
    pub fn to_ipc_error_with_redactor(&self, redactor: &Redactor) -> IpcError {
        match self {
            Self::AuthFailed { message, .. } => IpcError {
                code: ErrorCode::ProviderAuthFailed,
                message: redactor.redact(message),
                details: None,
                retryable: false,
            },
            Self::Unreachable { message, .. } => IpcError {
                code: ErrorCode::ProviderUnreachable,
                message: redactor.redact(message),
                details: None,
                retryable: true,
            },
            Self::Cancelled => IpcError {
                code: ErrorCode::InternalError,
                message: "Operation cancelled".to_string(),
                details: None,
                retryable: false,
            },
            Self::Validation(msg) => IpcError {
                code: ErrorCode::ValidationFailed,
                message: redactor.redact(msg),
                details: None,
                retryable: false,
            },
            Self::Protocol(msg) => IpcError {
                code: ErrorCode::InternalError,
                message: redactor.redact(msg),
                details: None,
                retryable: false,
            },
            Self::Internal(msg) => IpcError {
                code: ErrorCode::InternalError,
                message: redactor.redact(msg),
                details: None,
                retryable: false,
            },
        }
    }

    /// Map HTTP status code and message according to §9.1.2.
    pub fn from_http_status(status: u16, raw_message: &str) -> Self {
        let redacted = redact_sensitive_strings(raw_message);
        match status {
            401 | 403 => Self::AuthFailed {
                status,
                message: redacted,
            },
            429 | 502 | 503 | 504 => Self::Unreachable {
                status: Some(status),
                message: redacted,
            },
            _ => Self::Internal(format!("HTTP error {status}: {redacted}")),
        }
    }
}

/// Unified provider abstraction for both ReAct planning mode and Scene writing mode (`§9.1`).
#[async_trait::async_trait]
pub trait LlmProvider: Send + Sync {
    /// Distinct provider identity ("ollama", "opencode", etc.).
    fn provider_id(&self) -> &'static str;

    /// Planning mode: synchronous or ReAct loop step with native tool calling.
    async fn chat_with_tools(
        &self,
        req: &ProviderChatRequest,
        cancel_token: CancellationToken,
    ) -> Result<ProviderChatResponse, ProviderError>;

    /// Writing mode: streaming text completion.
    async fn stream_completion(
        &self,
        req: &ProviderStreamRequest,
        event_tx: tokio::sync::mpsc::Sender<ProviderStreamChunk>,
        cancel_token: CancellationToken,
    ) -> Result<ProviderStreamSummary, ProviderError>;
}
