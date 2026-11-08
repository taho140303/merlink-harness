//! LLM provider clients, OS Keychain credentials, and log redaction module root (REQ-ARCH-001, P6S2).
//!
//! Depends on `domain`, must not call up into `service` or `ipc`.

pub mod credentials;
pub mod ollama;
pub mod opencode;
pub mod redaction;
pub mod traits;
