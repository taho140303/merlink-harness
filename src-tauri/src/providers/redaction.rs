//! Sensitive-data redaction filter and protected credential wrappers (`REQ-SEC-001`, `§9.3`).
//!
//! Enforces that no synthetic or live API keys/secrets leak into logs, errors, or IPC payloads.

use once_cell::sync::Lazy;
use parking_lot::RwLock;
use regex::Regex;
use std::collections::HashSet;
use std::fmt;

pub const REDACTED_PLACEHOLDER: &str = "[REDACTED]";

/// Standard secret pattern regex matching:
/// - `sk-[a-zA-Z0-9_-]{16,}`
/// - `Bearer\s+[a-zA-Z0-9._-]+`
static SENSITIVE_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(sk-[a-zA-Z0-9_\-]{16,}|Bearer\s+[a-zA-Z0-9._\-]+)")
        .expect("valid regex for sensitive pattern")
});

/// Global default Redactor singleton.
static GLOBAL_REDACTOR: Lazy<Redactor> = Lazy::new(Redactor::new);

/// Wrapper for API keys and secrets that redacts them in `Debug` and `Display` (`§9.3`).
#[derive(Clone, PartialEq, Eq)]
pub struct SecretKey(String);

impl SecretKey {
    pub fn new(key: impl Into<String>) -> Self {
        Self(key.into())
    }

    /// Explicitly expose the underlying secret for HTTP authorization headers only.
    pub fn expose_secret(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for SecretKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{REDACTED_PLACEHOLDER}")
    }
}

impl fmt::Display for SecretKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{REDACTED_PLACEHOLDER}")
    }
}

/// Redactor engine maintaining registered dynamic secrets and regex patterns.
#[derive(Debug)]
pub struct Redactor {
    registered_secrets: RwLock<HashSet<String>>,
}

impl Default for Redactor {
    fn default() -> Self {
        Self::new()
    }
}

impl Redactor {
    pub fn new() -> Self {
        Self {
            registered_secrets: RwLock::new(HashSet::new()),
        }
    }

    pub fn global() -> &'static Redactor {
        &GLOBAL_REDACTOR
    }

    /// Register a sensitive string so that any subsequent log or error formatting redacts it.
    pub fn register_secret(&self, secret: &str) {
        let trimmed = secret.trim();
        if !trimmed.is_empty() {
            self.registered_secrets.write().insert(trimmed.to_string());
        }
    }

    /// Scans and replaces all registered secrets and regex pattern matches with `"[REDACTED]"`.
    pub fn redact(&self, input: &str) -> String {
        if input.is_empty() {
            return String::new();
        }

        // 1. Redact via standard regex pattern: (sk-... or Bearer ...)
        let mut result = SENSITIVE_PATTERN
            .replace_all(input, REDACTED_PLACEHOLDER)
            .to_string();

        // 2. Redact all explicitly registered secrets
        let secrets = self.registered_secrets.read();
        for secret in secrets.iter() {
            if !secret.is_empty() && result.contains(secret) {
                result = result.replace(secret, REDACTED_PLACEHOLDER);
            }
        }

        result
    }
}

/// Convenience free function utilizing the global redactor (`be-core.md#L2090`).
pub fn redact_sensitive_strings(text: &str) -> String {
    GLOBAL_REDACTOR.redact(text)
}
