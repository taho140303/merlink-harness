//! Draft Runtime State, Per-Draft Concurrency Guards, and Working State Registry
//! (`REQ-CONC-001`, `be-core.md` §2.3, §2.6, `core.md` §3.6, `CI-03`).

use crate::domain::business_flow::BusinessFlowData;
use crate::error::{ErrorCode, IpcError};
use crate::ipc::dto::{DraftLockStatusDto, EventLogEntryDto, PendingPatchDto};
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex as TokioMutex;
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

/// Default timeout for acquiring the per-draft operation lock (`core.md` §3.6: 30s).
pub const DRAFT_LOCK_TIMEOUT: Duration = Duration::from_secs(30);

/// In-memory lifecycle status of an active draft in runtime (`CI-03`).
///
/// Note: `SealPendingRetry` is strictly an in-memory runtime status flag
/// in `DraftRuntimeEntry` and is never persisted into SQLite `draft_sessions.status` (`CI-03`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DraftRuntimeStatus {
    Editing,
    Committing,
    Committed,
    SealPendingRetry,
}

impl DraftRuntimeStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            DraftRuntimeStatus::Editing => "editing",
            DraftRuntimeStatus::Committing => "committing",
            DraftRuntimeStatus::Committed => "committed",
            DraftRuntimeStatus::SealPendingRetry => "seal_pending_retry",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "editing" => Some(DraftRuntimeStatus::Editing),
            "committing" => Some(DraftRuntimeStatus::Committing),
            "committed" => Some(DraftRuntimeStatus::Committed),
            "seal_pending_retry" => Some(DraftRuntimeStatus::SealPendingRetry),
            _ => None,
        }
    }
}

impl std::fmt::Display for DraftRuntimeStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Internal state tracked under the per-draft `operation_lock`.
#[derive(Debug, Clone, Default)]
pub struct DraftOperationGuardState {
    pub is_locked: bool,
    pub holder_info: Option<String>,
}

/// In-memory working draft state (`be-core.md` §2.3, `core.md` §3.6).
#[derive(Debug, Clone)]
pub struct WorkingState {
    pub draft_session_id: Uuid,
    pub status: DraftRuntimeStatus,
    pub phase: i32,
    pub subphase: Option<i32>,
    pub head_version_seq: Option<i32>,
    pub business_flow_data: BusinessFlowData,
    pub draft_overview_ui: serde_json::Value,
    pub pending_patches: Vec<PendingPatchDto>,
    pub event_log: Vec<EventLogEntryDto>,
}

impl WorkingState {
    pub fn new(draft_session_id: Uuid) -> Self {
        Self {
            draft_session_id,
            status: DraftRuntimeStatus::Editing,
            phase: 0,
            subphase: None,
            head_version_seq: None,
            business_flow_data: BusinessFlowData::new(),
            draft_overview_ui: serde_json::json!({}),
            pending_patches: Vec::new(),
            event_log: Vec::new(),
        }
    }
}

/// RAII lock guard for per-draft `operation_lock` that automatically
/// resets non-blocking `lock_holder` telemetry upon drop.
pub struct DraftOperationGuard<'a> {
    _guard: tokio::sync::MutexGuard<'a, DraftOperationGuardState>,
    entry: &'a DraftRuntimeEntry,
}

impl<'a> std::fmt::Debug for DraftOperationGuard<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DraftOperationGuard")
            .field("draft_session_id", &self.entry.draft_session_id)
            .finish()
    }
}

impl<'a> Drop for DraftOperationGuard<'a> {
    fn drop(&mut self) {
        *self.entry.lock_holder.write() = None;
    }
}

/// In-memory entry managing concurrency, snapshot reads, and lifecycle for one draft session.
pub struct DraftRuntimeEntry {
    pub draft_session_id: Uuid,
    /// Level 1 lock: serializes AI turn, workflow round, UI manual edit, restore, and phase confirm.
    pub operation_lock: TokioMutex<DraftOperationGuardState>,
    /// Level 3 lock: atomic clone-and-swap pointer for fast non-blocking snapshot reads.
    pub working_state: std::sync::RwLock<Arc<WorkingState>>,
    /// Cooperative cancellation token for the active operation.
    pub active_cancel_token: std::sync::Mutex<Option<CancellationToken>>,
    /// Telemetry for non-blocking lock state queries.
    pub lock_holder: RwLock<Option<String>>,
}

impl DraftRuntimeEntry {
    pub fn new(draft_session_id: Uuid, initial_state: WorkingState) -> Self {
        Self {
            draft_session_id,
            operation_lock: TokioMutex::new(DraftOperationGuardState::default()),
            working_state: std::sync::RwLock::new(Arc::new(initial_state)),
            active_cancel_token: std::sync::Mutex::new(None),
            lock_holder: RwLock::new(None),
        }
    }

    /// Read non-blocking snapshot without acquiring `operation_lock` (`be-core.md` §2.3).
    pub fn get_snapshot(&self) -> Arc<WorkingState> {
        let ws = self.working_state.read().unwrap();
        Arc::clone(&*ws)
    }

    /// Atomically replaces the current working state pointer under `working_state.write()`.
    pub fn update_working_state(&self, new_state: WorkingState) {
        let mut ws = self.working_state.write().unwrap();
        *ws = Arc::new(new_state);
    }

    /// Mutates the working state atomically by cloning, modifying, and swapping the `Arc`.
    pub fn mutate_working_state<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut WorkingState) -> R,
    {
        let mut ws = self.working_state.write().unwrap();
        let mut cloned = (**ws).clone();
        let result = f(&mut cloned);
        *ws = Arc::new(cloned);
        result
    }

    /// Sets the in-memory runtime status (e.g. `DraftRuntimeStatus::SealPendingRetry`).
    pub fn set_runtime_status(&self, status: DraftRuntimeStatus) {
        let mut ws = self.working_state.write().unwrap();
        let mut cloned = (**ws).clone();
        cloned.status = status;
        *ws = Arc::new(cloned);
    }

    /// Non-blocking check of the current draft lock status.
    pub fn get_lock_status(&self) -> DraftLockStatusDto {
        let holder = self.lock_holder.read();
        match &*holder {
            Some(info) => DraftLockStatusDto {
                is_locked: true,
                holder_info: Some(info.clone()),
            },
            None => DraftLockStatusDto {
                is_locked: false,
                holder_info: None,
            },
        }
    }

    /// Acquires the per-draft operation lock with timeout (default 30s).
    /// Returns `ErrorCode::DraftBusy` on timeout without affecting running tasks.
    pub async fn acquire_operation_lock(
        &self,
        holder_info: &str,
        timeout_dur: Duration,
    ) -> Result<DraftOperationGuard<'_>, IpcError> {
        match tokio::time::timeout(timeout_dur, self.operation_lock.lock()).await {
            Ok(guard) => {
                *self.lock_holder.write() = Some(holder_info.to_string());
                Ok(DraftOperationGuard {
                    _guard: guard,
                    entry: self,
                })
            }
            Err(_) => Err(IpcError::new(
                ErrorCode::DraftBusy,
                format!(
                    "Draft {} is busy: timed out waiting for operation lock after {:?}",
                    self.draft_session_id, timeout_dur
                ),
            )),
        }
    }
}

/// Registry mapping `draft_session_id` to its active `DraftRuntimeEntry` (`be-core.md` §2.3).
pub struct DraftRuntimeRegistry {
    entries: RwLock<HashMap<Uuid, Arc<DraftRuntimeEntry>>>,
}

impl Default for DraftRuntimeRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl DraftRuntimeRegistry {
    pub fn new() -> Self {
        Self {
            entries: RwLock::new(HashMap::new()),
        }
    }

    pub fn get(&self, id: &Uuid) -> Option<Arc<DraftRuntimeEntry>> {
        self.entries.read().get(id).cloned()
    }

    pub fn get_or_register(&self, id: Uuid, initial_state: WorkingState) -> Arc<DraftRuntimeEntry> {
        let mut map = self.entries.write();
        map.entry(id)
            .or_insert_with(|| Arc::new(DraftRuntimeEntry::new(id, initial_state)))
            .clone()
    }

    pub fn register(&self, entry: Arc<DraftRuntimeEntry>) {
        let mut map = self.entries.write();
        map.insert(entry.draft_session_id, entry);
    }

    pub fn remove(&self, id: &Uuid) -> Option<Arc<DraftRuntimeEntry>> {
        self.entries.write().remove(id)
    }

    pub fn clear(&self) {
        self.entries.write().clear();
    }
}
