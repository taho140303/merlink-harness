//! Credential storage via OS Keychain with in-memory session-only fallback (`REQ-SEC-001`, `§9.2`).
//!
//! Absolute security invariants:
//! 1. NEVER write plaintext secrets to SQLite, JSON config files, or disk logs.
//! 2. Fall back to RAM (`SessionKeyStore`) when OS Keychain is unavailable.

use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

pub const KEYRING_SERVICE: &str = "com.merlink.harness";
pub const KEYRING_KEY_PREFIX: &str = "provider_api_key:";

/// Storage classification for provider credentials (`be-core.md` §9.2, §10.2.10).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CredentialStorageMode {
    OsKeychain,
    SessionOnlyFallback,
}

impl CredentialStorageMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::OsKeychain => "os_keychain",
            Self::SessionOnlyFallback => "session_only_fallback",
        }
    }
}

/// In-memory session key store used as fallback when OS Keychain is not available.
#[derive(Debug, Default, Clone)]
pub struct SessionKeyStore {
    keys: Arc<RwLock<HashMap<String, String>>>,
}

impl SessionKeyStore {
    pub fn new() -> Self {
        Self {
            keys: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn insert(&self, provider_id: impl Into<String>, api_key: impl Into<String>) {
        self.keys.write().insert(provider_id.into(), api_key.into());
    }

    pub fn get(&self, provider_id: &str) -> Option<String> {
        self.keys.read().get(provider_id).cloned()
    }

    pub fn remove(&self, provider_id: &str) -> Option<String> {
        self.keys.write().remove(provider_id)
    }

    pub fn contains(&self, provider_id: &str) -> bool {
        self.keys.read().contains_key(provider_id)
    }

    pub fn clear(&self) {
        self.keys.write().clear();
    }
}

/// Manager coordinating between OS Keychain (`keyring`) and `SessionKeyStore`.
#[derive(Debug, Clone)]
pub struct CredentialManager {
    service_name: String,
    session_store: SessionKeyStore,
    force_session_only: bool,
}

impl Default for CredentialManager {
    fn default() -> Self {
        Self::new()
    }
}

impl CredentialManager {
    pub fn new() -> Self {
        Self {
            service_name: KEYRING_SERVICE.to_string(),
            session_store: SessionKeyStore::new(),
            force_session_only: false,
        }
    }

    /// Construct a CredentialManager explicitly forcing session-only RAM storage (useful for testing `phase2_sprint4_deep3-F2`).
    pub fn new_with_session_only() -> Self {
        Self {
            service_name: KEYRING_SERVICE.to_string(),
            session_store: SessionKeyStore::new(),
            force_session_only: true,
        }
    }

    pub fn keyring_username(provider_id: &str) -> String {
        format!("{KEYRING_KEY_PREFIX}{provider_id}")
    }

    /// Test if OS Keychain service is accessible.
    pub fn is_keychain_available(&self) -> bool {
        if self.force_session_only {
            return false;
        }

        // Attempt a probe on a test key
        let entry = keyring::Entry::new(&self.service_name, "__merlink_keychain_probe__");
        match entry {
            Ok(entry) => {
                // A get_password returning EntryNotFound means keychain service itself is available
                match entry.get_password() {
                    Ok(_) => true,
                    Err(keyring::Error::NoEntry) => true,
                    Err(_) => false,
                }
            }
            Err(_) => false,
        }
    }

    /// Retrieve API key for a given provider.
    pub fn get_api_key(&self, provider_id: &str) -> Option<String> {
        // 1. Check in-memory session store first
        if let Some(key) = self.session_store.get(provider_id) {
            return Some(key);
        }

        // 2. If session-only is forced, do not query keychain
        if self.force_session_only {
            return None;
        }

        // 3. Query OS Keychain
        let username = Self::keyring_username(provider_id);
        if let Ok(entry) = keyring::Entry::new(&self.service_name, &username) {
            if let Ok(pwd) = entry.get_password() {
                return Some(pwd);
            }
        }

        None
    }

    /// Set API key for a provider.
    ///
    /// Returns the storage mode (`OsKeychain` or `SessionOnlyFallback`).
    /// If api_key is empty/whitespace, caller must validate before calling.
    pub fn set_api_key(
        &self,
        provider_id: &str,
        api_key: &str,
    ) -> Result<CredentialStorageMode, String> {
        let trimmed = api_key.trim();
        if trimmed.is_empty() {
            return Err("API key cannot be empty or whitespace".to_string());
        }

        // Always cache in RAM session store so credentials are immediately available in-process
        self.session_store.insert(provider_id, trimmed);

        if self.force_session_only {
            return Ok(CredentialStorageMode::SessionOnlyFallback);
        }

        // Attempt to store in OS Keychain
        let username = Self::keyring_username(provider_id);
        match keyring::Entry::new(&self.service_name, &username) {
            Ok(entry) => match entry.set_password(trimmed) {
                Ok(()) => Ok(CredentialStorageMode::OsKeychain),
                Err(_err) => Ok(CredentialStorageMode::SessionOnlyFallback),
            },
            Err(_err) => Ok(CredentialStorageMode::SessionOnlyFallback),
        }
    }

    /// Delete API key from both stores.
    pub fn delete_api_key(&self, provider_id: &str) {
        self.session_store.remove(provider_id);

        if !self.force_session_only {
            let username = Self::keyring_username(provider_id);
            if let Ok(entry) = keyring::Entry::new(&self.service_name, &username) {
                let _ = entry.delete_credential();
            }
        }
    }

    /// Check if API key exists.
    pub fn has_api_key(&self, provider_id: &str) -> bool {
        if self.session_store.contains(provider_id) {
            return true;
        }

        if self.force_session_only {
            return false;
        }

        let username = Self::keyring_username(provider_id);
        if let Ok(entry) = keyring::Entry::new(&self.service_name, &username) {
            return entry.get_password().is_ok();
        }

        false
    }

    /// Current storage mode for this provider (or system-wide default).
    pub fn current_storage_mode(&self, provider_id: &str) -> CredentialStorageMode {
        if self.session_store.contains(provider_id) {
            CredentialStorageMode::SessionOnlyFallback
        } else if self.is_keychain_available() {
            CredentialStorageMode::OsKeychain
        } else {
            CredentialStorageMode::SessionOnlyFallback
        }
    }
}

#[cfg(test)]
mod native_backend_tests {
    #[test]
    fn desktop_credentials_must_not_use_mock_storage() {
        // Creating an entry does not read or overwrite any real provider key.
        // An unavailable OS service is allowed: the manager falls back to RAM.
        if let Ok(entry) = keyring::Entry::new("com.merlink.harness.test", "backend-check") {
            assert!(
                !entry.get_credential().is::<keyring::mock::MockCredential>(),
                "Desktop builds must enable a native keyring backend; mock entries lose saved keys"
            );
        }
    }
}
