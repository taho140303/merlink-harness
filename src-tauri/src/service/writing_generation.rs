//! Writing generation service orchestration (`REQ-CMD-032`..`034`, `REQ-WRITE-002`, `be-core.md` §8.3, §10.2.9).
//!
//! Orchestrates:
//! - Streaming scene generation with ephemeral RAM buffering (`start_scene_generation`, IPC #32).
//! - Active generation cancellation (`cancel_scene_generation`, IPC #33).
//! - Dynamic `EditScope` expansion with human operator authorization (`respond_edit_scope_expansion`, IPC #34).
//! - Permanent lock invalidation (`LOCK-04`, `T-LOCK-01`) and accept gating (`SCOPE-04`).

use std::collections::{BTreeSet, HashMap};
use std::sync::{Arc, RwLock};
use chrono::Utc;
use rusqlite::params;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

use crate::error::{ErrorCode, IpcError};
use crate::ipc::dto::{
    CancelSceneGenerationReq, CancelSceneGenerationResponseDto, GenerationStreamStatus,
    RespondEditScopeExpansionReq, RespondEditScopeExpansionResponseDto,
    SceneGenerationResultDto, SceneGenerationStreamEvent, StartSceneGenerationReq,
};
use crate::providers::traits::{
    LlmProvider, ProviderError, ProviderStreamChunk, ProviderStreamRequest,
};
use crate::storage::db::DbPool;

// ============================================================================
// Ephemeral Domain Models (§8.3.1)
// ============================================================================

/// Request-scoped authorization boundary for AI mutations (`SCOPE-01`..`06`, `Q4.9`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestEditScope {
    pub request_id: Uuid,
    pub chapter_id: Uuid,
    pub allowed_scene_ids: BTreeSet<Uuid>,
    pub is_active: bool,
}

/// In-memory buffer holding generated content awaiting user review/accept (`core.md` §1.6).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneGenerationBuffer {
    pub request_id: Uuid,
    pub generation_id: Uuid,
    pub chapter_id: Uuid,
    pub scene_id: Uuid,
    pub baseline_version_number: i32,
    pub regeneration_note: Option<String>,
    pub accumulated_text: String,
    pub authorization_valid: bool,
    pub status: GenerationStreamStatus,
    pub created_at: String,
}

/// Internal representation of an actively executing generation.
#[derive(Clone)]
pub struct ActiveGeneration {
    pub request_id: Uuid,
    pub generation_id: Uuid,
    pub chapter_id: Uuid,
    pub scene_id: Uuid,
    pub baseline_version_number: i32,
    pub regeneration_note: Option<String>,
    pub accumulated_text: String,
    pub authorization_valid: bool,
    pub status: GenerationStreamStatus,
    pub cancel_token: CancellationToken,
}

/// Ephemeral registry tracking RAM buffers and scopes per chapter.
#[derive(Default)]
pub struct WritingGenerationState {
    /// Active edit scope per chapter (`chapter_id` -> `RequestEditScope`).
    pub scopes: HashMap<Uuid, RequestEditScope>,
    /// Currently running generation per chapter (at most 1 active per chapter).
    pub active_generations: HashMap<Uuid, ActiveGeneration>,
    /// Persisted in RAM buffers awaiting user review/accept (`chapter_id` -> (`generation_id` -> `SceneGenerationBuffer`)).
    pub buffers: HashMap<Uuid, HashMap<Uuid, SceneGenerationBuffer>>,
}

// ============================================================================
// Service Implementation
// ============================================================================

/// Orchestration service for scene content streaming and EditScope lifecycle.
pub struct WritingGenerationService {
    pool: DbPool,
    state: Arc<RwLock<WritingGenerationState>>,
    provider: Arc<RwLock<Option<Arc<dyn LlmProvider>>>>,
}

impl WritingGenerationService {
    /// Constructs a new `WritingGenerationService` with database pool and empty provider.
    pub fn new(pool: DbPool) -> Self {
        Self {
            pool,
            state: Arc::new(RwLock::new(WritingGenerationState::default())),
            provider: Arc::new(RwLock::new(None)),
        }
    }

    /// Constructs a new `WritingGenerationService` with an initial provider.
    pub fn with_provider(pool: DbPool, provider: Arc<dyn LlmProvider>) -> Self {
        Self {
            pool,
            state: Arc::new(RwLock::new(WritingGenerationState::default())),
            provider: Arc::new(RwLock::new(Some(provider))),
        }
    }

    /// Configures or replaces the active LLM provider.
    pub fn set_provider(&self, provider: Arc<dyn LlmProvider>) {
        let mut guard = self.provider.write().unwrap();
        *guard = Some(provider);
    }

    /// Retrieves a reference clone of the current LLM provider, if configured.
    pub fn provider(&self) -> Option<Arc<dyn LlmProvider>> {
        self.provider.read().unwrap().clone()
    }

    /// Accessor for the underlying database pool.
    pub fn pool(&self) -> &DbPool {
        &self.pool
    }

    // ------------------------------------------------------------------------
    // Scope and Buffer Management
    // ------------------------------------------------------------------------

    /// Registers or updates an active edit scope for a chapter.
    pub fn register_edit_scope(&self, scope: RequestEditScope) {
        let mut state = self.state.write().unwrap();
        state.scopes.insert(scope.chapter_id, scope);
    }

    /// Retrieves the current edit scope for a chapter.
    pub fn get_edit_scope(&self, chapter_id: Uuid) -> Option<RequestEditScope> {
        let state = self.state.read().unwrap();
        state.scopes.get(&chapter_id).cloned()
    }

    /// Retrieves a specific generation buffer from RAM.
    pub fn get_buffer(&self, chapter_id: Uuid, generation_id: Uuid) -> Option<SceneGenerationBuffer> {
        let state = self.state.read().unwrap();
        state
            .buffers
            .get(&chapter_id)
            .and_then(|ch| ch.get(&generation_id).cloned())
    }

    /// Lists all generation buffers for a chapter.
    pub fn list_buffers(&self, chapter_id: Uuid) -> Vec<SceneGenerationBuffer> {
        let state = self.state.read().unwrap();
        state
            .buffers
            .get(&chapter_id)
            .map(|ch| ch.values().cloned().collect())
            .unwrap_or_default()
    }

    /// Checks if a chapter currently has an active generation streaming.
    pub fn has_active_generation(&self, chapter_id: Uuid) -> bool {
        let state = self.state.read().unwrap();
        state
            .active_generations
            .get(&chapter_id)
            .map(|g| g.status == GenerationStreamStatus::Streaming)
            .unwrap_or(false)
    }

    /// Returns the generation_id of the currently active generation for chapter_id, if any.
    pub fn get_active_generation_id(&self, chapter_id: Uuid) -> Option<Uuid> {
        let state = self.state.read().unwrap();
        state.active_generations.get(&chapter_id).map(|g| g.generation_id)
    }

    // ------------------------------------------------------------------------
    // Lock Invalidation Hook (LOCK-04, T-LOCK-01)
    // ------------------------------------------------------------------------

    /// Cancels and permanently invalidates an in-flight generation if it targets `scene_id`.
    ///
    /// Sets `status = InvalidatedByLock` and `authorization_valid = false` permanently.
    pub fn invalidate_by_lock(&self, scene_id: Uuid) -> Option<Uuid> {
        let mut state = self.state.write().unwrap();
        for (_, active) in state.active_generations.iter_mut() {
            if active.scene_id == scene_id {
                active.status = GenerationStreamStatus::InvalidatedByLock;
                active.authorization_valid = false;
                active.cancel_token.cancel();
                return Some(active.generation_id);
            }
        }
        None
    }

    /// Cancels and permanently invalidates an in-flight generation for the given `chapter_id`.
    pub fn invalidate_by_lock_for_chapter(&self, chapter_id: Uuid) -> Vec<Uuid> {
        let mut state = self.state.write().unwrap();
        let mut res = Vec::new();
        if let Some(active) = state.active_generations.get_mut(&chapter_id) {
            active.status = GenerationStreamStatus::InvalidatedByLock;
            active.authorization_valid = false;
            active.cancel_token.cancel();
            res.push(active.generation_id);
        }
        res
    }

    // ------------------------------------------------------------------------
    // Review, Accept & Discard Gating (SCOPE-04, LOCK-04)
    // ------------------------------------------------------------------------

    /// Review and Accept buffer content into working state.
    ///
    /// Enforces the 3 mandatory gates:
    /// 1. `buffer.status == GenerationStreamStatus::Completed`
    /// 2. `buffer.authorization_valid == true` (never invalidated by lock)
    /// 3. Scene currently unlocked in SQLite (`scenes.is_locked == false`)
    ///
    /// Does NOT write to SQLite `scenes.content` or `scene_versions` (caller performs dirty save).
    pub fn accept_generation(&self, chapter_id: Uuid, generation_id: Uuid) -> Result<String, IpcError> {
        let buffer = {
            let state = self.state.read().unwrap();
            let chapter_buffers = state.buffers.get(&chapter_id).ok_or_else(|| {
                IpcError::new(
                    ErrorCode::EntityNotFound,
                    format!("No buffers found for chapter {}", chapter_id),
                )
            })?;
            chapter_buffers.get(&generation_id).cloned().ok_or_else(|| {
                IpcError::new(
                    ErrorCode::EntityNotFound,
                    format!("Generation {} not found in chapter {}", generation_id, chapter_id),
                )
            })?
        };

        // Gate 1: Check if invalidated by lock
        if buffer.status == GenerationStreamStatus::InvalidatedByLock {
            return Err(IpcError::new(
                ErrorCode::GenerationInvalidatedByLock,
                format!(
                    "Generation {} was invalidated by scene lock and cannot be accepted (LOCK-04, Q4.6)",
                    generation_id
                ),
            ));
        }

        // Gate 2: Check if completed
        if buffer.status != GenerationStreamStatus::Completed {
            return Err(IpcError::new(
                ErrorCode::ValidationFailed,
                format!(
                    "Generation {} is in status '{:?}', only Completed generations can be accepted",
                    generation_id, buffer.status
                ),
            ));
        }

        // Gate 3: Check authorization flag
        if !buffer.authorization_valid {
            return Err(IpcError::new(
                ErrorCode::GenerationInvalidatedByLock,
                format!(
                    "Generation {} was invalidated by scene lock and cannot be accepted (LOCK-04, Q4.6)",
                    generation_id
                ),
            ));
        }

        // Gate 4: Check current lock state in SQLite
        let is_locked = self.is_scene_locked(buffer.scene_id)?;
        if is_locked {
            return Err(IpcError::new(
                ErrorCode::SceneLocked,
                format!(
                    "Scene {} is currently locked; unlock before accepting generation (LOCK-05)",
                    buffer.scene_id
                ),
            ));
        }

        Ok(buffer.accumulated_text)
    }

    /// Discards a generation buffer from RAM without affecting persisted content.
    pub fn discard_generation(&self, chapter_id: Uuid, generation_id: Uuid) -> Result<(), IpcError> {
        let mut state = self.state.write().unwrap();
        if let Some(ch_buffers) = state.buffers.get_mut(&chapter_id) {
            ch_buffers.remove(&generation_id);
        }
        Ok(())
    }

    // ------------------------------------------------------------------------
    // SQLite Helper Methods
    // ------------------------------------------------------------------------

    fn is_scene_locked(&self, scene_id: Uuid) -> Result<bool, IpcError> {
        let conn = self
            .pool
            .get()
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
        let mut stmt = conn
            .prepare("SELECT is_locked FROM scenes WHERE id = ?1")
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
        let is_locked: Option<i32> = stmt
            .query_row(params![scene_id.to_string()], |row| row.get(0))
            .ok();
        Ok(is_locked.map(|v| v != 0).unwrap_or(false))
    }

    fn verify_scene_in_chapter(
        &self,
        scene_id: Uuid,
        chapter_id: Uuid,
    ) -> Result<bool, IpcError> {
        let conn = self
            .pool
            .get()
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
        let mut stmt = conn
            .prepare("SELECT is_locked FROM scenes WHERE id = ?1 AND chapter_id = ?2")
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
        let is_locked: i32 = stmt
            .query_row(
                params![scene_id.to_string(), chapter_id.to_string()],
                |row| row.get(0),
            )
            .map_err(|_| {
                IpcError::new(
                    ErrorCode::EntityNotFound,
                    format!("Scene {} not found in chapter {}", scene_id, chapter_id),
                )
            })?;
        Ok(is_locked != 0)
    }

    fn get_baseline_version_number(&self, scene_id: Uuid) -> Result<i32, IpcError> {
        let conn = self
            .pool
            .get()
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
        let mut stmt = conn
            .prepare(
                "SELECT COALESCE(MAX(version_number), 0) FROM scene_versions WHERE parent_id = ?1",
            )
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
        let ver: i32 = stmt
            .query_row(params![scene_id.to_string()], |row| row.get(0))
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
        Ok(ver)
    }

    // ------------------------------------------------------------------------
    // IPC #32: start_scene_generation
    // ------------------------------------------------------------------------

    /// Initiates streaming scene generation.
    pub async fn start_scene_generation(
        &self,
        req: &StartSceneGenerationReq,
        event_tx: Option<mpsc::Sender<SceneGenerationStreamEvent>>,
    ) -> Result<SceneGenerationResultDto, IpcError> {
        let provider = self.provider().ok_or_else(|| {
            IpcError::new(
                ErrorCode::ProviderUnreachable,
                "No LLM provider configured for scene generation",
            )
        })?;
        self.start_scene_generation_with_provider(req, provider, event_tx)
            .await
    }

    /// Initiates streaming scene generation using an explicitly supplied LLM provider.
    pub async fn start_scene_generation_with_provider(
        &self,
        req: &StartSceneGenerationReq,
        provider: Arc<dyn LlmProvider>,
        event_tx: Option<mpsc::Sender<SceneGenerationStreamEvent>>,
    ) -> Result<SceneGenerationResultDto, IpcError> {
        // 1. Validate mode
        let mode_norm = req.mode.trim().to_lowercase();
        if mode_norm != "generate" && mode_norm != "regenerate" {
            return Err(IpcError::new(
                ErrorCode::ValidationFailed,
                format!("Invalid mode '{}'; must be 'generate' or 'regenerate'", req.mode),
            ));
        }

        // 2. Normalize note: trim, empty -> None, max 2000 chars
        let normalized_note = req
            .note
            .as_ref()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty());

        if let Some(ref note) = normalized_note {
            if note.chars().count() > 2000 {
                return Err(IpcError::new(
                    ErrorCode::ValidationFailed,
                    "Regeneration note exceeds 2000 characters limit (core.md §1.6)",
                ));
            }
        }

        // 3. Check scene existence and lock state in SQLite BEFORE calling provider
        let is_locked = self.verify_scene_in_chapter(req.scene_id, req.chapter_id)?;
        if is_locked {
            return Err(IpcError::new(
                ErrorCode::SceneLocked,
                format!("Cannot generate content for locked scene {} (LOCK-04)", req.scene_id),
            ));
        }

        let baseline_version_number = self.get_baseline_version_number(req.scene_id)?;

        // 4. Check active generation constraint (at most 1 active per chapter)
        // and evaluate / establish EditScope
        let (request_id, generation_id, cancel_token) = {
            let mut state = self.state.write().unwrap();

            if let Some(active) = state.active_generations.get(&req.chapter_id) {
                if active.status == GenerationStreamStatus::Streaming {
                    return Err(IpcError::new(
                        ErrorCode::ValidationFailed,
                        format!(
                            "Chapter {} already has an active scene generation in progress (core.md §1.6)",
                            req.chapter_id
                        ),
                    ));
                }
            }

            // EditScope validation:
            // If active scope exists for chapter, scene_id must be authorized.
            // Otherwise, initialize EditScope = {scene_id}.
            let request_id = match state.scopes.get(&req.chapter_id) {
                Some(scope) if scope.is_active => {
                    if !scope.allowed_scene_ids.contains(&req.scene_id) {
                        return Err(IpcError::new(
                            ErrorCode::OutOfEditScope,
                            format!(
                                "Scene {} is outside the authorized EditScope for chapter {} (SCOPE-01)",
                                req.scene_id, req.chapter_id
                            ),
                        ));
                    }
                    scope.request_id
                }
                _ => {
                    let req_id = Uuid::now_v7();
                    let new_scope = RequestEditScope {
                        request_id: req_id,
                        chapter_id: req.chapter_id,
                        allowed_scene_ids: BTreeSet::from([req.scene_id]),
                        is_active: true,
                    };
                    state.scopes.insert(req.chapter_id, new_scope);
                    req_id
                }
            };

            let gen_id = Uuid::now_v7();
            let c_token = CancellationToken::new();

            let active_gen = ActiveGeneration {
                request_id,
                generation_id: gen_id,
                chapter_id: req.chapter_id,
                scene_id: req.scene_id,
                baseline_version_number,
                regeneration_note: normalized_note.clone(),
                accumulated_text: String::new(),
                authorization_valid: true,
                status: GenerationStreamStatus::Streaming,
                cancel_token: c_token.clone(),
            };

            state.active_generations.insert(req.chapter_id, active_gen);

            (request_id, gen_id, c_token)
        };

        // 5. Emit `Started` event
        if let Some(ref tx) = event_tx {
            let _ = tx
                .send(SceneGenerationStreamEvent::Started {
                    request_id,
                    generation_id,
                    scene_id: req.scene_id,
                    baseline_version_number,
                    authorization_valid: true,
                    status: GenerationStreamStatus::Streaming,
                })
                .await;
        }

        // 6. Spawn provider stream completion
        let prompt = match (&normalized_note, &req.current_working_content) {
            (Some(n), Some(c)) => format!("Mode: {}. Working content: {}. Note: {}", req.mode, c, n),
            (Some(n), None) => format!("Mode: {}. Note: {}", req.mode, n),
            (None, Some(c)) => format!("Mode: {}. Working content: {}", req.mode, c),
            (None, None) => format!("Mode: {}", req.mode),
        };

        let stream_req = ProviderStreamRequest {
            model: "default".to_string(),
            prompt,
            messages: None,
            temperature: Some(0.7),
            max_tokens: Some(4096),
        };

        let (chunk_tx, mut chunk_rx) = mpsc::channel::<ProviderStreamChunk>(100);
        let prov = provider.clone();
        let s_req = stream_req.clone();
        let c_token = cancel_token.clone();

        let provider_handle = tokio::spawn(async move {
            prov.stream_completion(&s_req, chunk_tx, c_token).await
        });

        // 7. Streaming pump with fast midstream lock detection
        let mut accumulated_text = String::new();
        let mut lock_poll_interval = tokio::time::interval(std::time::Duration::from_millis(20));
        let mut final_status = GenerationStreamStatus::Completed;

        loop {
            tokio::select! {
                _ = cancel_token.cancelled() => {
                    let state = self.state.read().unwrap();
                    if let Some(active) = state.active_generations.get(&req.chapter_id) {
                        if active.status == GenerationStreamStatus::InvalidatedByLock {
                            final_status = GenerationStreamStatus::InvalidatedByLock;
                        } else {
                            final_status = GenerationStreamStatus::Cancelled;
                        }
                    } else {
                        final_status = GenerationStreamStatus::Cancelled;
                    }
                    break;
                }
                _ = lock_poll_interval.tick() => {
                    // Check if scene locked in SQLite midstream
                    if let Ok(true) = self.is_scene_locked(req.scene_id) {
                        final_status = GenerationStreamStatus::InvalidatedByLock;
                        cancel_token.cancel();
                        break;
                    }
                }
                chunk_opt = chunk_rx.recv() => {
                    match chunk_opt {
                        Some(chunk) => {
                            // Check lock again on chunk boundary
                            if let Ok(true) = self.is_scene_locked(req.scene_id) {
                                final_status = GenerationStreamStatus::InvalidatedByLock;
                                cancel_token.cancel();
                                break;
                            }
                            accumulated_text.push_str(&chunk.delta);

                            // Update active text in RAM
                            {
                                let mut state = self.state.write().unwrap();
                                if let Some(active) = state.active_generations.get_mut(&req.chapter_id) {
                                    active.accumulated_text = accumulated_text.clone();
                                }
                            }

                            if let Some(ref tx) = event_tx {
                                let _ = tx
                                    .send(SceneGenerationStreamEvent::Delta {
                                        request_id,
                                        generation_id,
                                        scene_id: req.scene_id,
                                        text_delta: chunk.delta,
                                        authorization_valid: true,
                                    })
                                    .await;
                            }
                        }
                        None => {
                            // Upstream stream finished
                            break;
                        }
                    }
                }
            }
        }

        // 8. Reconcile provider outcome and finalize stream
        let prov_outcome = match provider_handle.await {
            Ok(res) => res,
            Err(e) => Err(ProviderError::Internal(e.to_string())),
        };

        if cancel_token.is_cancelled() || matches!(prov_outcome, Err(ProviderError::Cancelled)) {
            let state = self.state.read().unwrap();
            if let Some(active) = state.active_generations.get(&req.chapter_id) {
                if active.status == GenerationStreamStatus::InvalidatedByLock {
                    final_status = GenerationStreamStatus::InvalidatedByLock;
                } else {
                    final_status = GenerationStreamStatus::Cancelled;
                }
            } else {
                final_status = GenerationStreamStatus::Cancelled;
            }
        }

        let now_str = Utc::now().to_rfc3339();

        if final_status == GenerationStreamStatus::InvalidatedByLock {
            let buffer = SceneGenerationBuffer {
                request_id,
                generation_id,
                chapter_id: req.chapter_id,
                scene_id: req.scene_id,
                baseline_version_number,
                regeneration_note: normalized_note.clone(),
                accumulated_text: accumulated_text.clone(),
                authorization_valid: false,
                status: GenerationStreamStatus::InvalidatedByLock,
                created_at: now_str,
            };

            {
                let mut state = self.state.write().unwrap();
                state.active_generations.remove(&req.chapter_id);
                if let Some(scope) = state.scopes.get_mut(&req.chapter_id) {
                    scope.is_active = false;
                }
                state
                    .buffers
                    .entry(req.chapter_id)
                    .or_default()
                    .insert(generation_id, buffer);
            }

            if let Some(ref tx) = event_tx {
                let _ = tx
                    .send(SceneGenerationStreamEvent::InvalidatedByLock {
                        request_id,
                        generation_id,
                        scene_id: req.scene_id,
                        partial_text: accumulated_text.clone(),
                        authorization_valid: false,
                        status: GenerationStreamStatus::InvalidatedByLock,
                    })
                    .await;
            }

            return Ok(SceneGenerationResultDto {
                request_id,
                generation_id,
                scene_id: req.scene_id,
                baseline_version_number,
                status: GenerationStreamStatus::InvalidatedByLock,
                authorization_valid: false,
                generated_text: accumulated_text,
                normalized_note,
            });
        }

        if final_status == GenerationStreamStatus::Cancelled {
            let buffer = SceneGenerationBuffer {
                request_id,
                generation_id,
                chapter_id: req.chapter_id,
                scene_id: req.scene_id,
                baseline_version_number,
                regeneration_note: normalized_note.clone(),
                accumulated_text: accumulated_text.clone(),
                authorization_valid: true,
                status: GenerationStreamStatus::Cancelled,
                created_at: now_str,
            };

            {
                let mut state = self.state.write().unwrap();
                state.active_generations.remove(&req.chapter_id);
                if let Some(scope) = state.scopes.get_mut(&req.chapter_id) {
                    scope.is_active = false;
                }
                state
                    .buffers
                    .entry(req.chapter_id)
                    .or_default()
                    .insert(generation_id, buffer);
            }

            if let Some(ref tx) = event_tx {
                let _ = tx
                    .send(SceneGenerationStreamEvent::Cancelled {
                        request_id,
                        generation_id,
                        scene_id: req.scene_id,
                        partial_text: accumulated_text.clone(),
                        authorization_valid: true,
                        status: GenerationStreamStatus::Cancelled,
                    })
                    .await;
            }

            return Ok(SceneGenerationResultDto {
                request_id,
                generation_id,
                scene_id: req.scene_id,
                baseline_version_number,
                status: GenerationStreamStatus::Cancelled,
                authorization_valid: true,
                generated_text: accumulated_text,
                normalized_note,
            });
        }

        // If provider returned an error:
        if let Err(prov_err) = prov_outcome {
            let ipc_err = prov_err.to_ipc_error();

            let buffer = SceneGenerationBuffer {
                request_id,
                generation_id,
                chapter_id: req.chapter_id,
                scene_id: req.scene_id,
                baseline_version_number,
                regeneration_note: normalized_note.clone(),
                accumulated_text: accumulated_text.clone(),
                authorization_valid: false,
                status: GenerationStreamStatus::Failed,
                created_at: now_str,
            };

            {
                let mut state = self.state.write().unwrap();
                state.active_generations.remove(&req.chapter_id);
                if let Some(scope) = state.scopes.get_mut(&req.chapter_id) {
                    scope.is_active = false;
                }
                state
                    .buffers
                    .entry(req.chapter_id)
                    .or_default()
                    .insert(generation_id, buffer);
            }

            if let Some(ref tx) = event_tx {
                let _ = tx
                    .send(SceneGenerationStreamEvent::Failed {
                        request_id,
                        generation_id,
                        scene_id: req.scene_id,
                        error: ipc_err.clone(),
                        authorization_valid: false,
                        status: GenerationStreamStatus::Failed,
                    })
                    .await;
            }

            return Err(ipc_err);
        }

        // Success: Completed
        let buffer = SceneGenerationBuffer {
            request_id,
            generation_id,
            chapter_id: req.chapter_id,
            scene_id: req.scene_id,
            baseline_version_number,
            regeneration_note: normalized_note.clone(),
            accumulated_text: accumulated_text.clone(),
            authorization_valid: true,
            status: GenerationStreamStatus::Completed,
            created_at: now_str,
        };

        {
            let mut state = self.state.write().unwrap();
            state.active_generations.remove(&req.chapter_id);
            if let Some(scope) = state.scopes.get_mut(&req.chapter_id) {
                scope.is_active = false; // Scope expires for new AI requests (§8.3.2)
            }
            state
                .buffers
                .entry(req.chapter_id)
                .or_default()
                .insert(generation_id, buffer);
        }

        if let Some(ref tx) = event_tx {
            let _ = tx
                .send(SceneGenerationStreamEvent::Completed {
                    request_id,
                    generation_id,
                    scene_id: req.scene_id,
                    baseline_version_number,
                    full_text: accumulated_text.clone(),
                    authorization_valid: true,
                    status: GenerationStreamStatus::Completed,
                })
                .await;
        }

        Ok(SceneGenerationResultDto {
            request_id,
            generation_id,
            scene_id: req.scene_id,
            baseline_version_number,
            status: GenerationStreamStatus::Completed,
            authorization_valid: true,
            generated_text: accumulated_text,
            normalized_note,
        })
    }

    // ------------------------------------------------------------------------
    // IPC #33: cancel_scene_generation
    // ------------------------------------------------------------------------

    /// Cancels an in-flight scene generation. Idempotent when called repeatedly.
    pub async fn cancel_scene_generation(
        &self,
        req: &CancelSceneGenerationReq,
    ) -> Result<CancelSceneGenerationResponseDto, IpcError> {
        let mut state = self.state.write().unwrap();

        // 1. Check if active in this chapter
        if let Some(active) = state.active_generations.get_mut(&req.chapter_id) {
            if active.generation_id == req.generation_id {
                active.status = GenerationStreamStatus::Cancelled;
                active.cancel_token.cancel();
                return Ok(CancelSceneGenerationResponseDto {
                    generation_id: req.generation_id,
                    status: "cancelled".to_string(),
                });
            }
        }

        // 2. Check if already in stored buffers for this chapter
        if let Some(buffers) = state.buffers.get(&req.chapter_id) {
            if buffers.contains_key(&req.generation_id) {
                return Ok(CancelSceneGenerationResponseDto {
                    generation_id: req.generation_id,
                    status: "cancelled".to_string(),
                });
            }
        }

        Err(IpcError::new(
            ErrorCode::EntityNotFound,
            format!(
                "Active or buffered generation {} not found in chapter {}",
                req.generation_id, req.chapter_id
            ),
        ))
    }

    // ------------------------------------------------------------------------
    // IPC #34: respond_edit_scope_expansion
    // ------------------------------------------------------------------------

    /// Evaluates user decision to expand EditScope to `target_scene_id`.
    ///
    /// Never automatically unlocks target scene if `scenes.is_locked == true` (`SL-10`).
    pub async fn respond_edit_scope_expansion(
        &self,
        req: &RespondEditScopeExpansionReq,
    ) -> Result<RespondEditScopeExpansionResponseDto, IpcError> {
        // 1. Verify target scene belongs to chapter in SQLite
        let is_locked = self.verify_scene_in_chapter(req.target_scene_id, req.chapter_id)?;

        // 2. Verify edit scope exists for request_id in chapter
        let mut state = self.state.write().unwrap();
        let scope = state
            .scopes
            .get_mut(&req.chapter_id)
            .filter(|s| s.request_id == req.request_id)
            .ok_or_else(|| {
                IpcError::new(
                    ErrorCode::EntityNotFound,
                    format!(
                        "Edit scope for request {} not found in chapter {}",
                        req.request_id, req.chapter_id
                    ),
                )
            })?;

        // 3. If approved, add target_scene_id to allowed set (lock status remains unchanged)
        if req.approved {
            scope.allowed_scene_ids.insert(req.target_scene_id);
        }

        let allowed_scene_ids = scope.allowed_scene_ids.iter().copied().collect();

        Ok(RespondEditScopeExpansionResponseDto {
            request_id: req.request_id,
            allowed_scene_ids,
            target_scene_is_locked: is_locked,
        })
    }
}
