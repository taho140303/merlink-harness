//! Segmented Workflow Execution Service (`REQ-WF-001`, `REQ-WF-002`, `REQ-WF-003`, `be-core.md` §6.3–§6.4, §10.2.5).
//!
//! Responsibilities:
//! 1. **Phase 1–2:** Sequential chapter runs via `plan_outline_runs` / `plan_arc_outline_runs`.
//! 2. **Phase 3 (Three Stages):**
//!    - Stage 1 (Probe): Subagents read chapters in parallel, produce `WorldProbeReportV1` probe reports.
//!    - Stage 2 (Consolidation): Pure backend merge, deduplication, build parent-child hierarchy tree (`parent_code`),
//!      reconciling against `WorkingState` to protect `user` / `user_edited` data from being overwritten (`Q3.4`).
//!    - Stage 3 (Hierarchical Write): Subagents execute writes layer-by-layer (`plan_world_write_by_depth`),
//!      ensuring parent nodes are sealed in earlier rounds before child nodes are written in later rounds (parent-first write).
//! 3. **Phase 5 (Subphase Separation):**
//!    - Strictly sequential execution of subphases `5.1` (Appearance), `5.2` (Psychology), and `5.3` (Memory & Abilities).
//!    - Never mixes different subphases within the same round.
//! 4. **LLM Parallel Compute & Deterministic Mutation Ordering:**
//!    - Subagents compute / call LLM concurrently without holding SQLite locks or `operation_lock`.
//!    - State mutations are strictly applied in ascending `task_index` order under per-draft `operation_lock`.
//! 5. **Round Sealing (`TX-04`):**
//!    - At the end of each round, calls `round_seal_service::seal_workflow_round_tx` atomically.
//!    - Does not create a version if zero-net state change (`reason = "workflow_round"` only if durable changes).
//! 6. **Controlled Cancellation (`cancel_workflow`):**
//!    - Sets cancellation flag, halts scheduling of new tasks, waits for in-flight RAM mutations to complete,
//!      and seals already `applied` mutations via `TX-04` before releasing `operation_lock`.
//!    - Returns `ErrorCode::VersionSealFailed` if seal fails during cancellation.
//! 7. **Resume & Missing Evidence Replan (§6.4.4):**
//!    - Verifies receipts and intermediate evidence; returns `requires_replan` with
//!      `recoveryReason = "insufficient_task_evidence"` if intermediate evidence is missing, never skipping or blindly replaying.

use std::collections::{BTreeMap, HashMap};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use futures::future::join_all;
use parking_lot::RwLock;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{params, Connection, OptionalExtension};
use serde_json::{json, Value};
use tokio::sync::{broadcast, mpsc};
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

use crate::domain::fingerprint::state_data_hash;
use crate::domain::phase3_artifact::WorldProbeReportV1;
use crate::domain::phase3_merge::{merge_probe_reports, MergedCandidates};
use crate::domain::sf06::codec::{decode_evidence, replay, PatchOp, PatchV2};
use crate::domain::workflow_planner::{
    extract_arc_chapter_ranges, format_arc_chapter_task_identity, plan_arc_outline_runs,
    plan_chapter_phase_with_context, plan_character_subphase, plan_relationship_phase,
    WorkflowConfig, WorkflowTaskKind,
};
use crate::error::{ErrorCode, IpcError};
use crate::ipc::dto::{
    CancelWorkflowReq, CancelWorkflowResponseDto, DraftVersionSummaryDto, GetWorkflowStatusReq,
    StartOrResumeWorkflowReq, WorkflowExecutionSummaryDto, WorkflowProgressEvent,
    WorkflowStatusDto,
};
use crate::service::draft_runtime::{DraftRuntimeRegistry, WorkingState};
use crate::service::draft_service::DraftService;
use crate::service::patch_service::compute_progress_dto;
use crate::service::round_seal_service::{
    canonical_task_identity, seal_workflow_round_tx, AuthorizedRoundPlanContext,
    CompletedTaskOutcome, PlannedTaskSpec, WorkflowRoundSealRequest,
};

/// Error injection control for workflow fault testing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkflowErrorInjection {
    None,
    FailTx04,
    FailTask(usize),
    DelayTask { task_index: usize, delay_ms: u64 },
}

/// Execution context for a subagent task within a workflow round.
#[derive(Debug, Clone)]
pub struct TaskExecutionContext {
    pub draft_session_id: Uuid,
    pub workflow_run_id: Uuid,
    pub plan_generation: i64,
    pub phase: i32,
    pub subphase: Option<i32>,
    pub round_index: i64,
    pub task_index: usize,
    pub task_spec: PlannedTaskSpec,
    pub task_identity: String,
    pub snapshot: Arc<WorkingState>,
    pub system_language: String,
    pub cancel_token: CancellationToken,
    /// Composed system prompt for this phase (shared blocks + `planning_phase_n`).
    pub phase_prompt: String,
    /// Tool schemas granted by `prompt_block_tools` for this phase.
    pub tool_schemas: Vec<Value>,
}

/// Execution output produced by a subagent task.
#[derive(Debug, Clone)]
pub struct TaskExecutionOutput {
    pub task_index: usize,
    pub task_spec: PlannedTaskSpec,
    pub task_identity: String,
    pub mutations: Vec<PatchOp>,
    pub applied_write_count: u32,
    pub artifact: Option<Value>,
}

/// Pluggable task executor trait for parallel subagent computation.
#[async_trait::async_trait]
pub trait WorkflowTaskExecutor: Send + Sync {
    async fn execute_task(
        &self,
        ctx: &TaskExecutionContext,
    ) -> Result<TaskExecutionOutput, IpcError>;
}

/// Default task executor for production and standard workflow tasks.
#[derive(Debug, Default)]
pub struct DefaultWorkflowTaskExecutor;

#[async_trait::async_trait]
impl WorkflowTaskExecutor for DefaultWorkflowTaskExecutor {
    /// Placeholder executor for environments without a configured provider.
    ///
    /// It intentionally produces no artifact: a probe report must come from a real model and
    /// pass `validate_probe_candidates`, so a synthetic report here could be mistaken for a
    /// sealed Phase 3 result. Production installs a provider resolver instead.
    async fn execute_task(
        &self,
        ctx: &TaskExecutionContext,
    ) -> Result<TaskExecutionOutput, IpcError> {
        Ok(TaskExecutionOutput {
            task_index: ctx.task_index,
            task_spec: ctx.task_spec.clone(),
            task_identity: ctx.task_identity.clone(),
            mutations: Vec::new(),
            applied_write_count: 0,
            artifact: None,
        })
    }
}

/// In-memory state tracking an active running workflow on a draft session.
pub struct ActiveWorkflowState {
    pub workflow_run_id: Uuid,
    pub draft_session_id: Uuid,
    pub plan_generation: i32,
    pub phase: i32,
    pub subphase: Option<i32>,
    pub current_round: Arc<parking_lot::Mutex<u32>>,
    pub total_rounds: Arc<parking_lot::Mutex<u32>>,
    pub completed_tasks: Arc<parking_lot::Mutex<u32>>,
    pub total_tasks: Arc<parking_lot::Mutex<u32>>,
    pub cancel_requested: Arc<AtomicBool>,
    pub cancel_token: CancellationToken,
    pub completion_sender: broadcast::Sender<()>,
    pub last_sealed_version: Arc<RwLock<Option<DraftVersionSummaryDto>>>,
    pub error_injection: Arc<RwLock<WorkflowErrorInjection>>,
    pub applied_task_order: Arc<parking_lot::Mutex<Vec<usize>>>,
}

/// Removes an in-flight workflow run on every exit path, including panic.
struct ActiveRunGuard {
    active_runs: Arc<RwLock<HashMap<Uuid, Arc<ActiveWorkflowState>>>>,
    draft_session_id: Uuid,
    completion_sender: broadcast::Sender<()>,
}

impl Drop for ActiveRunGuard {
    fn drop(&mut self) {
        self.active_runs.write().remove(&self.draft_session_id);
        let _ = self.completion_sender.send(());
    }
}

/// Converts merged candidates into Phase 3 write rounds: parents first, then lore.
fn build_world_write_rounds(merged: &MergedCandidates) -> Vec<Vec<PlannedTaskSpec>> {
    let mut rounds = Vec::new();
    let mut depth = 0u32;
    loop {
        let layer = merged.world_layer(depth);
        if layer.is_empty() {
            break;
        }
        let candidates: Vec<Value> = layer
            .iter()
            .map(|candidate| {
                json!({
                    "tempKey": candidate.temp_key,
                    "parentTempKey": candidate.parent_temp_key,
                    "existingParentCode": candidate.existing_parent_code,
                    "displayName": candidate.display_name,
                    "type": candidate.setting_type,
                    "description": candidate.description,
                    "access": candidate.access,
                })
            })
            .collect();
        rounds.push(vec![PlannedTaskSpec {
            phase: 3,
            subphase: None,
            task_kind: "world_write".to_string(),
            assigned_scope: json!({
                "groupKey": format!("depth_{depth}"),
                "range": [1, candidates.len()],
                "candidates": { "worldSettings": candidates, "loreEntries": [] },
            }),
            dependencies: vec![],
            deterministic_identity: Some(format!("p3:s0:world_write:depth={depth}")),
        }]);
        depth += 1;
    }

    if !merged.lore_entries.is_empty() {
        let lore: Vec<Value> = merged
            .lore_entries
            .iter()
            .map(|entry| {
                json!({
                    "tempKey": entry.temp_key,
                    "worldTempKey": entry.world_temp_key,
                    "existingWorldCode": entry.existing_world_code,
                    "displayName": entry.display_name,
                    "type": entry.lore_type,
                    "description": entry.description,
                    "data": entry.data,
                })
            })
            .collect();
        rounds.push(vec![PlannedTaskSpec {
            phase: 3,
            subphase: None,
            task_kind: "world_write".to_string(),
            assigned_scope: json!({
                "groupKey": "lore",
                "range": [1, lore.len()],
                "candidates": { "worldSettings": [], "loreEntries": lore },
            }),
            dependencies: vec![],
            deterministic_identity: Some("p3:s0:world_write:lore".to_string()),
        }]);
    }
    rounds
}

/// Builds the task executor for one workflow run (production installs a live LLM executor).
pub type ExecutorFactory =
    Arc<dyn Fn() -> Result<Arc<dyn WorkflowTaskExecutor>, IpcError> + Send + Sync>;

/// Segmented Workflow Service.
pub struct WorkflowService {
    pool: Pool<SqliteConnectionManager>,
    registry: Arc<DraftRuntimeRegistry>,
    draft_service: Arc<DraftService>,
    config: RwLock<WorkflowConfig>,
    active_runs: Arc<RwLock<HashMap<Uuid, Arc<ActiveWorkflowState>>>>,
    task_executor: Arc<RwLock<Arc<dyn WorkflowTaskExecutor>>>,
    executor_factory: Arc<RwLock<Option<ExecutorFactory>>>,
    error_injection: Arc<RwLock<HashMap<Uuid, WorkflowErrorInjection>>>,
}

impl WorkflowService {
    pub fn new(
        pool: Pool<SqliteConnectionManager>,
        registry: Arc<DraftRuntimeRegistry>,
        draft_service: Arc<DraftService>,
        config: WorkflowConfig,
    ) -> Self {
        Self {
            pool,
            registry,
            draft_service,
            config: RwLock::new(config),
            active_runs: Arc::new(RwLock::new(HashMap::new())),
            task_executor: Arc::new(RwLock::new(Arc::new(DefaultWorkflowTaskExecutor))),
            executor_factory: Arc::new(RwLock::new(None)),
            error_injection: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn with_executor(
        pool: Pool<SqliteConnectionManager>,
        registry: Arc<DraftRuntimeRegistry>,
        draft_service: Arc<DraftService>,
        config: WorkflowConfig,
        executor: Arc<dyn WorkflowTaskExecutor>,
    ) -> Self {
        Self {
            pool,
            registry,
            draft_service,
            config: RwLock::new(config),
            active_runs: Arc::new(RwLock::new(HashMap::new())),
            task_executor: Arc::new(RwLock::new(executor)),
            executor_factory: Arc::new(RwLock::new(None)),
            error_injection: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn pool(&self) -> &Pool<SqliteConnectionManager> {
        &self.pool
    }

    pub fn registry(&self) -> &Arc<DraftRuntimeRegistry> {
        &self.registry
    }

    pub fn draft_service(&self) -> &Arc<DraftService> {
        &self.draft_service
    }

    pub fn set_task_executor(&self, executor: Arc<dyn WorkflowTaskExecutor>) {
        *self.task_executor.write() = executor;
    }

    /// Install the production executor factory. When present, every run builds a fresh executor
    /// (with the saved provider/model) at start instead of reusing the placeholder executor.
    pub fn set_executor_factory(&self, factory: ExecutorFactory) {
        *self.executor_factory.write() = Some(factory);
    }

    pub fn set_config(&self, config: WorkflowConfig) {
        *self.config.write() = config;
    }

    pub fn get_config(&self) -> WorkflowConfig {
        *self.config.read()
    }

    pub fn set_error_injection(&self, draft_session_id: &Uuid, injection: WorkflowErrorInjection) {
        self.error_injection
            .write()
            .insert(*draft_session_id, injection);
    }

    pub fn get_applied_task_order(&self, draft_session_id: &Uuid) -> Vec<usize> {
        if let Some(run) = self.active_runs.read().get(draft_session_id) {
            run.applied_task_order.lock().clone()
        } else {
            Vec::new()
        }
    }

    /// Primary entry point for IPC #16 `start_or_resume_workflow`.
    pub async fn start_or_resume_workflow(
        &self,
        req: &StartOrResumeWorkflowReq,
        event_tx: Option<mpsc::Sender<WorkflowProgressEvent>>,
    ) -> Result<WorkflowExecutionSummaryDto, IpcError> {
        self.start_or_resume_workflow_with_config(req, event_tx, self.get_config())
            .await
    }

    /// Snapshot settings once per run; saving settings cannot change an in-flight plan.
    pub async fn start_or_resume_workflow_with_config(
        &self,
        req: &StartOrResumeWorkflowReq,
        event_tx: Option<mpsc::Sender<WorkflowProgressEvent>>,
        cfg: WorkflowConfig,
    ) -> Result<WorkflowExecutionSummaryDto, IpcError> {
        // 1. Parameter Validation
        if req.phase < 1 || req.phase > 6 {
            return Err(IpcError::new(
                ErrorCode::ValidationFailed,
                format!("Invalid workflow phase {}. Must be in 1..=6", req.phase),
            ));
        }

        if cfg.outline_lookback < 0 || cfg.outline_lookback >= cfg.outline_cap {
            return Err(IpcError::new(
                ErrorCode::ValidationFailed,
                format!(
                    "Invalid outline window configuration: requires 0 <= outline_lookback ({}) < outline_cap ({})",
                    cfg.outline_lookback, cfg.outline_cap
                ),
            ));
        }

        if req.phase == 5 {
            if !matches!(req.subphase, Some(1..=3)) {
                return Err(IpcError::new(
                    ErrorCode::ValidationFailed,
                    "Phase 5 requires subphase to be Some(1), Some(2), or Some(3)",
                ));
            }
        } else if req.subphase.is_some() {
            return Err(IpcError::new(
                ErrorCode::ValidationFailed,
                format!(
                    "Subphase is only valid for Phase 5, received for phase {}",
                    req.phase
                ),
            ));
        }

        // 2. Draft Session and State Validation
        let conn = self
            .pool
            .get()
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

        let (draft_status, head_seq): (String, i64) = conn
            .query_row(
                "SELECT status, head_version_seq FROM draft_sessions WHERE id = ?1",
                params![req.draft_session_id.to_string()],
                |r| Ok((r.get(0)?, r.get(1)?)),
            )
            .optional()
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?
            .ok_or_else(|| {
                IpcError::new(
                    ErrorCode::EntityNotFound,
                    format!("Draft session '{}' not found", req.draft_session_id),
                )
            })?;

        if draft_status != "editing" {
            return Err(IpcError::new(
                ErrorCode::DraftNotEditable,
                format!(
                    "Draft session '{}' is in '{}' state and not editable",
                    req.draft_session_id, draft_status
                ),
            ));
        }

        // Ensure runtime entry exists
        let runtime_entry = self
            .draft_service
            .get_or_load_runtime_entry(req.draft_session_id)?;

        // Check if workflow is already actively running on this draft
        {
            let runs = self.active_runs.read();
            if runs.contains_key(&req.draft_session_id) {
                return Err(IpcError::new(
                    ErrorCode::DraftBusy,
                    format!(
                        "Workflow is already running on draft session '{}'",
                        req.draft_session_id
                    ),
                ));
            }
        }

        // 3. Plan Generation and Rounds Construction
        let workflow_run_id = req.workflow_run_id.unwrap_or_else(Uuid::new_v4);
        let plan_generation = 1i64;

        // Any structural repair before the first round MUST be sealed as its own version.
        // TX-04 compares the round base hash with the stored head version hash; mutating the
        // runtime (dedupe or approval capture) without sealing produced
        // "Base data hash mismatch" on the first round.
        let mut repaired_before_round = false;
        let mut removed_arc_duplicates = 0usize;
        runtime_entry.mutate_working_state(|ws| {
            removed_arc_duplicates =
                crate::domain::phase_gate::dedupe_arcs_by_position(&mut ws.business_flow_data);
        });
        if removed_arc_duplicates > 0 {
            repaired_before_round = true;
        }
        // The model does not reliably call `save_approved_outline`, so a workflow start with an
        // empty `decision_log` would lose the outline the author just approved in chat. Capture
        // the latest assistant proposal deterministically as a planning decision before the
        // prompt is composed; the entry is included in the same repair version.
        {
            let current = runtime_entry.get_snapshot();
            if current.business_flow_data.decision_log.is_empty() {
                if let Some(entry) = self.capture_recent_approval(&req.draft_session_id) {
                    runtime_entry.mutate_working_state(|ws| {
                        ws.business_flow_data.decision_log.push(entry);
                    });
                    repaired_before_round = true;
                }
            }
        }
        if repaired_before_round {
            self.seal_repaired_base(&req.draft_session_id)?;
        }
        let working_state = runtime_entry.get_snapshot();
        // A workflow phase may only start when the previous phase completed its optional payload;
        // Phase 1 also needs the Phase 0 requirement. This mirrors `confirm_phase` gating.
        let previous_missing = if req.phase == 1 {
            let eval = crate::domain::progress_engine::evaluate_progress(
                &working_state.business_flow_data,
            );
            if eval.is_require_complete {
                Vec::new()
            } else {
                vec!["Phase 0: còn trường require chưa hoàn tất".to_string()]
            }
        } else if req.phase > 1 {
            crate::domain::phase_gate::missing_for_phase(
                &working_state.business_flow_data,
                req.phase - 1,
            )
        } else {
            Vec::new()
        };
        // Phase 5 sub-phases are ordered: 5.2 needs 5.1 complete, 5.3 needs 5.2 complete.
        let mut previous_missing = previous_missing;
        if req.phase == 5 {
            let sub = req.subphase.unwrap_or(1);
            if sub > 1 {
                previous_missing.extend(crate::domain::phase_gate::missing_for_subphase(
                    &working_state.business_flow_data,
                    sub - 1,
                ));
            }
        }
        if !previous_missing.is_empty() {
            return Err(IpcError::new(
                ErrorCode::ValidationFailed,
                format!(
                    "Không thể chạy phase {}{} khi phase trước chưa hoàn tất: {}",
                    req.phase,
                    req.subphase
                        .map(|sub| format!(".{sub}"))
                        .unwrap_or_default(),
                    previous_missing.join("; ")
                ),
            ));
        }
        let mut plan_rounds =
            self.build_rounds_for_phase(req.phase, req.subphase, &working_state, &cfg)?;

        let (mut phase_prompt, tool_schemas) = self.compose_phase_prompt(req.phase)?;
        if let Some(handoff) = self.recent_author_handoff(&req.draft_session_id) {
            phase_prompt.push_str(&handoff);
        }

        let total_rounds = plan_rounds.len() as u32;
        let total_tasks: u32 = plan_rounds.iter().map(|r| r.len() as u32).sum();

        // 4. Resume Classification (§6.4.4): Check existing receipts
        let existing_receipts =
            self.query_existing_receipts(&conn, &req.draft_session_id, &workflow_run_id)?;

        if !existing_receipts.is_empty() {
            // Check for missing intermediate evidence
            for receipt in &existing_receipts {
                let evidence_str = match &receipt.evidence_json {
                    Some(s) if !s.trim().is_empty() => s,
                    _ => {
                        // Missing intermediate evidence -> RequiresReplan
                        return Ok(WorkflowExecutionSummaryDto {
                            workflow_run_id,
                            plan_generation: plan_generation as i32,
                            status: "requires_replan".to_string(),
                            recovery_reason: Some("insufficient_task_evidence".to_string()),
                            completed_tasks: 0,
                            total_tasks,
                            last_sealed_version: self.get_head_version_summary(
                                &conn,
                                &req.draft_session_id,
                                head_seq,
                            )?,
                            pending_patches: working_state.pending_patches.clone(),
                        });
                    }
                };

                let parsed_evidence: Value = match serde_json::from_str(evidence_str) {
                    Ok(v) => v,
                    Err(_) => {
                        return Ok(WorkflowExecutionSummaryDto {
                            workflow_run_id,
                            plan_generation: plan_generation as i32,
                            status: "requires_replan".to_string(),
                            recovery_reason: Some("insufficient_task_evidence".to_string()),
                            completed_tasks: 0,
                            total_tasks,
                            last_sealed_version: self.get_head_version_summary(
                                &conn,
                                &req.draft_session_id,
                                head_seq,
                            )?,
                            pending_patches: working_state.pending_patches.clone(),
                        });
                    }
                };

                if decode_evidence(&parsed_evidence).is_err() {
                    return Ok(WorkflowExecutionSummaryDto {
                        workflow_run_id,
                        plan_generation: plan_generation as i32,
                        status: "requires_replan".to_string(),
                        recovery_reason: Some("insufficient_task_evidence".to_string()),
                        completed_tasks: 0,
                        total_tasks,
                        last_sealed_version: self.get_head_version_summary(
                            &conn,
                            &req.draft_session_id,
                            head_seq,
                        )?,
                        pending_patches: working_state.pending_patches.clone(),
                    });
                }
            }
        }

        // 5. Initialize Active Run Tracking
        let (completion_tx, _) = broadcast::channel(4);
        let cancel_token = CancellationToken::new();
        let cancel_requested = Arc::new(AtomicBool::new(false));
        let error_injection = Arc::new(RwLock::new(
            self.error_injection
                .read()
                .get(&req.draft_session_id)
                .copied()
                .unwrap_or(WorkflowErrorInjection::None),
        ));

        let active_state = Arc::new(ActiveWorkflowState {
            workflow_run_id,
            draft_session_id: req.draft_session_id,
            plan_generation: plan_generation as i32,
            phase: req.phase,
            subphase: req.subphase,
            current_round: Arc::new(parking_lot::Mutex::new(0)),
            total_rounds: Arc::new(parking_lot::Mutex::new(total_rounds)),
            completed_tasks: Arc::new(parking_lot::Mutex::new(0)),
            total_tasks: Arc::new(parking_lot::Mutex::new(total_tasks)),
            cancel_requested: cancel_requested.clone(),
            cancel_token: cancel_token.clone(),
            completion_sender: completion_tx,
            last_sealed_version: Arc::new(RwLock::new(None)),
            error_injection: error_injection.clone(),
            applied_task_order: Arc::new(parking_lot::Mutex::new(Vec::new())),
        });

        self.active_runs
            .write()
            .insert(req.draft_session_id, active_state.clone());

        // Panic-safe cleanup: if a task/tool panics, the future is dropped and this guard still
        // removes the in-flight entry, so the draft does not stay stuck on `DraftBusy`.
        let _active_run_guard = ActiveRunGuard {
            active_runs: self.active_runs.clone(),
            draft_session_id: req.draft_session_id,
            completion_sender: active_state.completion_sender.clone(),
        };

        // Emit WorkflowStarted event
        self.emit_event(
            &event_tx,
            WorkflowProgressEvent::WorkflowStarted {
                workflow_run_id,
                plan_generation: plan_generation as i32,
                phase: req.phase,
                subphase: req.subphase,
                total_rounds,
                total_tasks,
            },
        );

        // 6. Execute Rounds Loop
        let mut overall_status = "completed".to_string();
        let mut completed_tasks_count = 0u32;
        let executor: Arc<dyn WorkflowTaskExecutor> = match self.executor_factory.read().as_ref() {
            Some(factory) => factory()?,
            None => self.task_executor.read().clone(),
        };

        let mut probe_reports: Vec<(String, String)> = Vec::new();
        let mut replan_reason: Option<String> = None;
        let mut round_cursor = 0usize;
        while round_cursor < plan_rounds.len() {
            let round_idx = round_cursor;
            let round_tasks = plan_rounds[round_cursor].clone();
            round_cursor += 1;
            if cancel_requested.load(Ordering::SeqCst) || cancel_token.is_cancelled() {
                overall_status = "cancelled".to_string();
                break;
            }

            *active_state.current_round.lock() = (round_idx + 1) as u32;

            let active_task_identities: Vec<String> = round_tasks
                .iter()
                .map(canonical_task_identity)
                .collect();

            self.emit_event(
                &event_tx,
                WorkflowProgressEvent::RoundStarted {
                    workflow_run_id,
                    round_index: round_idx as u32,
                    active_task_identities: active_task_identities.clone(),
                },
            );

            // Parallel LLM compute: Subagents compute concurrently WITHOUT holding SQLite lock or operation_lock
            let snapshot = runtime_entry.get_snapshot();
            let mut compute_futs = Vec::with_capacity(round_tasks.len());

            for (task_idx, task_spec) in round_tasks.iter().enumerate() {
                let exec = executor.clone();
                let task_id = canonical_task_identity(task_spec);
                let ctx = TaskExecutionContext {
                    draft_session_id: req.draft_session_id,
                    workflow_run_id,
                    plan_generation,
                    phase: req.phase,
                    subphase: req.subphase,
                    round_index: round_idx as i64,
                    task_index: task_idx,
                    task_spec: task_spec.clone(),
                    task_identity: task_id,
                    snapshot: snapshot.clone(),
                    system_language: req.system_language.clone(),
                    cancel_token: cancel_token.clone(),
                    phase_prompt: phase_prompt.clone(),
                    tool_schemas: tool_schemas.clone(),
                };

                let err_inj = *error_injection.read();

                compute_futs.push(async move {
                    if let WorkflowErrorInjection::DelayTask {
                        task_index: d_idx,
                        delay_ms,
                    } = err_inj
                    {
                        if d_idx == task_idx {
                            tokio::time::sleep(Duration::from_millis(delay_ms)).await;
                        }
                    }

                    if let WorkflowErrorInjection::FailTask(f_idx) = err_inj {
                        if f_idx == task_idx {
                            return Err(IpcError::new(
                                ErrorCode::InternalError,
                                format!("Injected task failure on task_index {}", task_idx),
                            ));
                        }
                    }

                    exec.execute_task(&ctx).await
                });
            }

            let task_results = join_all(compute_futs).await;

            // Check if any task failed
            let mut round_task_outputs = Vec::with_capacity(task_results.len());
            let mut task_failed = false;
            for res in task_results {
                match res {
                    Ok(output) => round_task_outputs.push(output),
                    Err(_e) => {
                        task_failed = true;
                        overall_status = "failed".to_string();
                        break;
                    }
                }
            }

            if task_failed {
                break;
            }

            // Acquire per-draft operation_lock before state mutations and round sealing
            let timeout_dur = Duration::from_secs(cfg.draft_lock_timeout_secs);
            let _guard = match runtime_entry
                .acquire_operation_lock("workflow_round", timeout_dur)
                .await
            {
                Ok(g) => g,
                Err(e) => {
                    self.active_runs.write().remove(&req.draft_session_id);
                    let _ = active_state.completion_sender.send(());
                    return Err(e);
                }
            };

            // Deterministic mutation ordering: Sort outputs strictly by task_index ASC
            round_task_outputs.sort_by_key(|o| o.task_index);

            // Record applied task index ordering for verification
            {
                let mut order = active_state.applied_task_order.lock();
                for out in &round_task_outputs {
                    order.push(out.task_index);
                }
            }

            // Apply mutations to WorkingState sequentially
            let mut completed_task_outcomes = Vec::with_capacity(round_task_outputs.len());
            let mut any_applied_write = false;

            let current_working_state = runtime_entry.get_snapshot();
            let base_version_seq = current_working_state
                .head_version_seq
                .unwrap_or(head_seq as i32) as i64;
            let base_data_hash = state_data_hash(&current_working_state.business_flow_data)
                .map_err(|e| IpcError::new(ErrorCode::InternalError, e.to_string()))?;

            let mut prev_flow_data = current_working_state.business_flow_data.clone();

            for out in round_task_outputs {
                let in_hash = state_data_hash(&prev_flow_data)
                    .map_err(|e| IpcError::new(ErrorCode::InternalError, e.to_string()))?;
                let in_fp = format!("sf06:v2:{}", in_hash);

                // Apply mutations via codec replay
                let next_flow_data = if !out.mutations.is_empty() {
                    let patch = PatchV2 {
                        ops: out.mutations.clone(),
                    };
                    replay(&prev_flow_data, &patch).map_err(|e| {
                        IpcError::new(e, "Failed to replay task patch on working state")
                    })?
                } else {
                    prev_flow_data.clone()
                };

                let out_hash = state_data_hash(&next_flow_data)
                    .map_err(|e| IpcError::new(ErrorCode::InternalError, e.to_string()))?;
                let out_fp = format!("sf06:v2:{}", out_hash);

                let has_write = !out.mutations.is_empty() && out.applied_write_count > 0;
                if has_write {
                    any_applied_write = true;
                }

                // Mutate working state in runtime
                runtime_entry.mutate_working_state(|ws| {
                    ws.business_flow_data = next_flow_data.clone();
                });

                prev_flow_data = next_flow_data;

                if let Some(artifact) = &out.artifact {
                    if let Ok(report) =
                        serde_json::from_value::<WorldProbeReportV1>(artifact.clone())
                    {
                        probe_reports.push((out.task_identity.clone(), report.report_text));
                    }
                }

                completed_task_outcomes.push(CompletedTaskOutcome {
                    task_index: out.task_index,
                    task_spec: out.task_spec,
                    task_identity: out.task_identity.clone(),
                    input_scope_fingerprint: in_fp,
                    output_scope_fingerprint: out_fp,
                    has_applied_write: has_write,
                    applied_write_count: out.applied_write_count,
                    applied_mutations: out.mutations,
                    artifact: out.artifact,
                });

                completed_tasks_count += 1;
                *active_state.completed_tasks.lock() = completed_tasks_count;

                self.emit_event(
                    &event_tx,
                    WorkflowProgressEvent::TaskCompleted {
                        workflow_run_id,
                        task_identity: out.task_identity,
                        completed_tasks: completed_tasks_count,
                        total_tasks,
                        was_skipped_by_receipt: false,
                    },
                );
            }

            // Check error injection for FailTx04
            if *error_injection.read() == WorkflowErrorInjection::FailTx04 {
                self.active_runs.write().remove(&req.draft_session_id);
                let _ = active_state.completion_sender.send(());
                return Err(IpcError::new(
                    ErrorCode::VersionSealFailed,
                    "Injected TX-04 version seal failure",
                ));
            }

            // Seal round atomically via TX-04
            let round_seal_key = format!(
                "wf_round:{}:{}:{}",
                workflow_run_id, plan_generation, round_idx
            );
            let seal_req = WorkflowRoundSealRequest {
                draft_session_id: req.draft_session_id.to_string(),
                workflow_run_id: workflow_run_id.to_string(),
                plan_generation,
                round_index: round_idx as i64,
                round_seal_key,
                phase: req.phase,
                subphase: req.subphase,
                parallel_limit: cfg.max_parallel as i64,
                base_version_seq,
                base_data_hash,
                completed_tasks: completed_task_outcomes,
                has_applied_write: any_applied_write,
            };

            let mut authorized_tasks_map = BTreeMap::new();
            for (idx, spec) in round_tasks.iter().enumerate() {
                authorized_tasks_map.insert(idx, spec.clone());
            }

            let plan_ctx = AuthorizedRoundPlanContext {
                workflow_run_id: workflow_run_id.to_string(),
                plan_generation,
                phase: req.phase,
                subphase: req.subphase,
                round_index: round_idx as i64,
                parallel_limit: cfg.max_parallel as i64,
                authorized_tasks: authorized_tasks_map,
            };

            let mut conn_tx = self
                .pool
                .get()
                .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

            let seal_res =
                seal_workflow_round_tx(&mut conn_tx, &seal_req, &plan_ctx).map_err(|e| {
                    IpcError::new(
                        ErrorCode::VersionSealFailed,
                        format!("TX-04 round seal failed: {}", e.message),
                    )
                })?;

            let sealed_dto = if let Some(seq) = seal_res.sealed_version_seq {
                runtime_entry.mutate_working_state(|ws| {
                    ws.head_version_seq = Some(seq as i32);
                });

                let summary =
                    self.get_head_version_summary(&conn_tx, &req.draft_session_id, seq)?;
                *active_state.last_sealed_version.write() = summary.clone();
                summary
            } else {
                None
            };

            let updated_ws = runtime_entry.get_snapshot();
            let progress_dto = compute_progress_dto(&updated_ws);

            self.emit_event(
                &event_tx,
                WorkflowProgressEvent::RoundSealed {
                    workflow_run_id,
                    round_index: round_idx as u32,
                    sealed_version: sealed_dto,
                    progress: progress_dto,
                },
            );

            // If cancellation was requested during round execution, break after round seal
            if cancel_requested.load(Ordering::SeqCst) || cancel_token.is_cancelled() {
                overall_status = "cancelled".to_string();
                break;
            }

            // After the last probe round, merge the sealed reports and append the write rounds.
            if req.phase == 3
                && round_tasks
                    .iter()
                    .all(|task| task.task_kind == "world_probe")
                && round_cursor == plan_rounds.len()
            {
                let existing_codes: Vec<String> = runtime_entry
                    .get_snapshot()
                    .business_flow_data
                    .optional
                    .world_building_phase
                    .world_settings
                    .iter()
                    .map(|node| node.code.clone())
                    .collect();
                match merge_probe_reports(&probe_reports, &existing_codes) {
                    Ok(merged) => plan_rounds.extend(build_world_write_rounds(&merged)),
                    Err(error) => {
                        overall_status = "requires_replan".to_string();
                        replan_reason = Some(format!("phase3_merge_failed: {error}"));
                        break;
                    }
                }
            }
        }

        // 7. Cleanup and Finish Event
        let last_version = active_state.last_sealed_version.read().clone();
        self.active_runs.write().remove(&req.draft_session_id);
        let _ = active_state.completion_sender.send(());

        self.emit_event(
            &event_tx,
            WorkflowProgressEvent::WorkflowFinished {
                workflow_run_id,
                status: overall_status.clone(),
                last_sealed_version: last_version.clone(),
            },
        );

        let final_ws = runtime_entry.get_snapshot();

        Ok(WorkflowExecutionSummaryDto {
            workflow_run_id,
            plan_generation: plan_generation as i32,
            status: overall_status,
            recovery_reason: replan_reason,
            completed_tasks: completed_tasks_count,
            total_tasks,
            last_sealed_version: last_version,
            pending_patches: final_ws.pending_patches.clone(),
        })
    }

    /// Primary entry point for IPC #17 `cancel_workflow`.
    pub async fn cancel_workflow(
        &self,
        req: &CancelWorkflowReq,
    ) -> Result<CancelWorkflowResponseDto, IpcError> {
        let (cancel_token, cancel_requested, mut completion_rx, last_sealed_lock, error_inj) = {
            let runs = self.active_runs.read();
            let run = runs.get(&req.draft_session_id).ok_or_else(|| {
                IpcError::new(
                    ErrorCode::EntityNotFound,
                    format!(
                        "No active workflow found on draft session '{}'",
                        req.draft_session_id
                    ),
                )
            })?;

            if run.workflow_run_id != req.workflow_run_id {
                return Err(IpcError::new(
                    ErrorCode::EntityNotFound,
                    format!(
                        "Active workflow run '{}' does not match requested run '{}'",
                        run.workflow_run_id, req.workflow_run_id
                    ),
                ));
            }

            (
                run.cancel_token.clone(),
                run.cancel_requested.clone(),
                run.completion_sender.subscribe(),
                run.last_sealed_version.clone(),
                run.error_injection.clone(),
            )
        };

        // Set cancellation flag
        cancel_requested.store(true, Ordering::SeqCst);
        cancel_token.cancel();

        // Wait for running tasks and round seal to finish
        let _ = completion_rx.recv().await;

        if *error_inj.read() == WorkflowErrorInjection::FailTx04 {
            return Err(IpcError::new(
                ErrorCode::VersionSealFailed,
                "Version seal failed during workflow cancellation",
            ));
        }

        let last_sealed = last_sealed_lock.read().clone();

        Ok(CancelWorkflowResponseDto {
            status: "cancelled".to_string(),
            last_sealed_version: last_sealed,
            resume_available: true,
        })
    }

    /// Primary entry point for IPC #18 `get_workflow_status`.
    pub fn get_workflow_status(
        &self,
        req: &GetWorkflowStatusReq,
    ) -> Result<WorkflowStatusDto, IpcError> {
        let conn = self
            .pool
            .get()
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

        // Verify draft session exists
        let draft_status: String = conn
            .query_row(
                "SELECT status FROM draft_sessions WHERE id = ?1",
                params![req.draft_session_id.to_string()],
                |r| r.get(0),
            )
            .optional()
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?
            .ok_or_else(|| {
                IpcError::new(
                    ErrorCode::EntityNotFound,
                    format!("Draft session '{}' not found", req.draft_session_id),
                )
            })?;

        // Check active in-memory run
        {
            let runs = self.active_runs.read();
            if let Some(run) = runs.get(&req.draft_session_id) {
                return Ok(WorkflowStatusDto {
                    is_running: true,
                    workflow_run_id: Some(run.workflow_run_id),
                    plan_generation: Some(run.plan_generation),
                    current_phase: Some(run.phase),
                    current_subphase: run.subphase,
                    current_round: Some(*run.current_round.lock()),
                    total_rounds: Some(*run.total_rounds.lock()),
                    completed_tasks: *run.completed_tasks.lock(),
                    total_tasks: *run.total_tasks.lock(),
                    resume_available: true,
                });
            }
        }

        // Query historical SQLite receipts
        let latest_round: Option<(String, i64, i64)> = conn
            .query_row(
                "SELECT workflow_run_id, plan_generation, round_index
                 FROM workflow_round_receipts
                 WHERE draft_session_id = ?1
                 ORDER BY created_at DESC LIMIT 1",
                params![req.draft_session_id.to_string()],
                |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
            )
            .optional()
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

        if let Some((run_id_str, gen, r_idx)) = latest_round {
            let run_id = Uuid::parse_str(&run_id_str).unwrap_or_else(|_| Uuid::nil());
            let completed_tasks: i64 = conn
                .query_row(
                    "SELECT COUNT(*) FROM workflow_task_receipts WHERE draft_session_id = ?1 AND workflow_run_id = ?2",
                    params![req.draft_session_id.to_string(), run_id_str],
                    |r| r.get(0),
                )
                .unwrap_or(0);

            Ok(WorkflowStatusDto {
                is_running: false,
                workflow_run_id: Some(run_id),
                plan_generation: Some(gen as i32),
                current_phase: None,
                current_subphase: None,
                current_round: Some(r_idx as u32),
                total_rounds: None,
                completed_tasks: completed_tasks as u32,
                total_tasks: completed_tasks as u32,
                resume_available: draft_status == "editing",
            })
        } else {
            Ok(WorkflowStatusDto {
                is_running: false,
                workflow_run_id: None,
                plan_generation: None,
                current_phase: None,
                current_subphase: None,
                current_round: None,
                total_rounds: None,
                completed_tasks: 0,
                total_tasks: 0,
                resume_available: false,
            })
        }
    }

    /// Field-level guidance for the tools of each phase. The JSON schema is permissive, so the
    /// model needs the documented parameter names to call the write handlers correctly.
    fn phase_tool_guidance(phase: i32) -> &'static str {
        match phase {
            1 => "Tool guide: add_arc(code?, display_name, summary, position, declared_sources); set_arc(code, display_name?, summary?, position?, declared_sources); set_arc_motif(code, motif_code?, note?, declared_sources); add_arc_event_block(arc_code, code?, event_block_code?, start_chapter?, end_chapter?, summary?, declared_sources); add_theme(arc_code, code?, parent_theme_code?, question?, resolution_type?, resolution?, message?, declared_sources); add_theme_argument(theme_code, character_code?, role=PRIMARY|ANTITHESIS|CAUTIONARY|WITNESS, answer_mode, note?, declared_sources). Target Arc count: use require.project.target.arc_count exactly. Phase gate: every Arc must have motif_code, at least one event block, and theme question + message, otherwise the transition to Phase 2 is blocked.",
            2 => "Tool guide: add_chapter(arc_code, code?, display_name, position, summary, declared_sources); set_chapter(code, display_name?, position?, summary?, declared_sources); add_scene(chapter_code, code?, display_name, position, summary, declared_sources). chapter.position is unique across the whole project. Total chapters must equal require.project.target.chapter_count. Phase gate: every chapter needs display_name, summary and at least one scene with display_name/summary.",
            3 => "Tool guide: add_world_setting(code?, parent_code?, display_name, type=REALM|WORLD|REGION|COUNTRY|CITY|TOWN|FACILITY|STREET|ROOM, description?, access?, declared_sources); add_lore_entry(code?, world_setting_code, display_name, type=CULTURE|HISTORY|FACTION|ITEM, description?, data?, declared_sources). Write parents before children. Phase gate: at least one world setting with display_name and type; parent_code must reference an existing setting.",
            4 => "Tool guide: set_scene_config(scene_code, style_preset_code?, narrator_config_code?, world_setting_code?, declared_sources); add_scene_character(scene_code, character_code, is_pov?, declared_sources); add_scene_lore_entry(scene_code, lore_entry_code, usage?, reveal_to_reader?, reveal_to_pov?, declared_sources). Limited narrators need exactly one is_pov; omniscient needs none. Phase gate: every scene needs world_setting_code, narrator_config_code and style_preset_code that reference existing values.",
            5 => "Tool guide: add_character_appearance(character_code, species?, gender_identity?, height?, weight?, hair_color_code?, skin_tone_code?, outfit_style?, default_summary?, declared_sources); add_character_psychological(character_code, the_wound?, the_lie?, fundamental_fear?, moral_alignment?, moral_code?, cognitive_bias?, breaking_point?, external_want?, internal_need?, internal_conflict?, social_mask_persona_type?, social_mask_verbal_style?, defense_primary_reaction?, defense_stress_trigger?, defense_coping_strategy?, declared_sources); add_character_memory(character_code, code?, display_name?, summary?, source_scene_code?, declared_sources); add_character_competence(character_code, code?, domain_code?, level_code?, scene_code, capabilities?, limitations?, declared_sources). Competence scene_code is mandatory and must reference an existing scene. Phase gate: every character needs appearance (default_summary or species), psychological (the_wound/external_want/internal_need) and at least one memory or competence bound to an existing scene.",
            6 => "Tool guide: add_relationship(code?, from_code, to_code, motif_code?, declared_sources); add_relationship_role(relationship_code, category_code?, hierarchy?, is_primary?, declared_sources); set_relationship_state(relationship_code, scene_code, trust?, affection?, hostility?, fear?, respect?, familiarity?, dependency? with NONE|LOW|MODERATE|HIGH|ABSOLUTE, declared_sources). Phase gate: with two or more characters, at least one relationship is required and from_code/to_code must reference existing characters.",
            _ => "",
        }
    }

    /// Arc windows derived from the real `chapter_phase` Arc codes. Event-block chapter ranges are
    /// honoured when present; otherwise the confirmed chapter target is split across the arcs in
    /// order. Never invents codes that do not exist in the draft.
    pub fn draft_arc_ranges(
        flow: &crate::domain::business_flow::BusinessFlowData,
    ) -> Vec<(String, i32, i32)> {
        let codes: Vec<String> = flow
            .optional
            .chapter_phase
            .arcs
            .iter()
            .map(|arc| arc.code.clone())
            .collect();
        if codes.is_empty() {
            return Vec::new();
        }
        let chapter_count = flow.require.project.target.chapter_count.value.unwrap_or(0) as i32;
        let per_arc = if chapter_count > 0 {
            (chapter_count + codes.len() as i32 - 1) / codes.len() as i32
        } else {
            0
        };
        let mut ranges = Vec::new();
        let mut start = 1;
        for (index, code) in codes.iter().enumerate() {
            let from_event_blocks = flow
                .optional
                .overview_phase
                .arcs
                .iter()
                .find(|arc| arc.code == *code)
                .map(|arc| {
                    let mut lo = i32::MAX;
                    let mut hi = 0;
                    for block in &arc.motif.event_blocks {
                        if let Some(value) = block.start_chapter.value {
                            lo = lo.min(value as i32);
                        }
                        if let Some(value) = block.end_chapter.value {
                            hi = hi.max(value as i32);
                        }
                    }
                    (lo, hi)
                })
                .filter(|(lo, hi)| *lo != i32::MAX && *hi >= *lo);
            if let Some((lo, hi)) = from_event_blocks {
                ranges.push((code.clone(), lo, hi));
                start = start.max(hi + 1);
            } else if chapter_count > 0 && start <= chapter_count {
                let end = if index == codes.len() - 1 {
                    chapter_count
                } else {
                    (start + per_arc.max(1) - 1).min(chapter_count)
                };
                ranges.push((code.clone(), start, end));
                start = end + 1;
            }
        }
        ranges
    }

    /// Deterministic Arc window fallback from the confirmed target counts (no entity names
    /// are fabricated: the codes only identify planning scopes, never draft entities).
    fn planned_arc_ranges(
        flow: &crate::domain::business_flow::BusinessFlowData,
    ) -> Vec<(String, i32, i32)> {
        let arc_count = flow.require.project.target.arc_count.value.unwrap_or(0) as i32;
        let chapter_count = flow.require.project.target.chapter_count.value.unwrap_or(0) as i32;
        if arc_count <= 0 || chapter_count <= 0 {
            return Vec::new();
        }
        let per_arc = (chapter_count + arc_count - 1) / arc_count;
        let mut ranges = Vec::new();
        let mut start = 1;
        for index in 0..arc_count {
            if start > chapter_count {
                break;
            }
            let end = if index == arc_count - 1 {
                chapter_count
            } else {
                (start + per_arc.max(1) - 1).min(chapter_count)
            };
            ranges.push((format!("planned_arc_{}", index + 1), start, end));
            start = end + 1;
        }
        ranges
    }

    /// Compose the phase system prompt and the tool schemas granted to it
    /// (`prompt_blocks` + `prompt_block_tools`, `core.md` §3.8).
    /// Latest assistant message for the draft's chat session, wrapped as a planning decision.
    /// Used as a deterministic fallback when the model did not persist the author's approval.
    fn capture_recent_approval(
        &self,
        draft_session_id: &Uuid,
    ) -> Option<crate::domain::business_flow::DecisionLogEntry> {
        let conn = self.pool.get().ok()?;
        let session_id: String = conn
            .query_row(
                "SELECT session_id FROM draft_sessions WHERE id = ?1",
                params![draft_session_id.to_string()],
                |row| row.get(0),
            )
            .optional()
            .ok()??;
        let content: Option<String> = conn
            .query_row(
                "SELECT content FROM messages
                 WHERE session_id = ?1
                   AND role = 'assistant'
                   AND trim(coalesce(content, '')) <> ''
                 ORDER BY created_at DESC, rowid DESC
                 LIMIT 1",
                params![session_id],
                |row| row.get(0),
            )
            .optional()
            .ok()?;
        approved_outline_from(content.as_deref(), 0)
    }

    /// Seal a new base version after a structural repair (duplicate Arc positions), so the next
    /// TX-04 round compares against a stored hash that matches the runtime state.
    fn seal_repaired_base(&self, draft_session_id: &Uuid) -> Result<(), IpcError> {
        let entry = self
            .draft_service
            .get_or_load_runtime_entry(*draft_session_id)?;
        let snap = entry.get_snapshot();
        let data_json = serde_json::to_string(&snap.business_flow_data)
            .map_err(|e| IpcError::new(ErrorCode::InternalError, e.to_string()))?;
        let data_hash = state_data_hash(&snap.business_flow_data)
            .map_err(|e| IpcError::new(ErrorCode::ValidationFailed, e.to_string()))?;
        let base_seq = snap.head_version_seq.unwrap_or(0);
        let seal_key = format!("repair_arcs:{}:{}", base_seq, Uuid::new_v4());
        self.draft_service
            .seal_version(&crate::service::draft_service::SealVersionParams {
                draft_session_id: *draft_session_id,
                seal_key,
                reason: "repair_arcs".to_string(),
                phase: snap.phase,
                subphase: snap.subphase,
                data_json,
                data_hash,
                parent_seq: Some(base_seq),
                turn_id: None,
                chat_message_id: None,
            })?;
        Ok(())
    }

    /// Recent author/assistant exchange for this draft's chat session. The workflow composes its
    /// prompts from the draft snapshot only, so without this block Phase 1 regenerates an outline
    /// that the author already discussed and approved in chat.
    fn recent_author_handoff(&self, draft_session_id: &Uuid) -> Option<String> {
        let conn = self.pool.get().ok()?;
        let session_id: String = conn
            .query_row(
                "SELECT session_id FROM draft_sessions WHERE id = ?1",
                params![draft_session_id.to_string()],
                |row| row.get(0),
            )
            .optional()
            .ok()??;
        let mut stmt = conn
            .prepare(
                "SELECT role, content FROM messages
                 WHERE session_id = ?1
                   AND role IN ('user','assistant')
                   AND trim(coalesce(content, '')) <> ''
                 ORDER BY created_at DESC, rowid DESC
                 LIMIT 8",
            )
            .ok()?;
        let rows = stmt
            .query_map(params![session_id], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
            })
            .ok()?;
        let mut items: Vec<(String, String)> = rows.filter_map(Result::ok).collect();
        if items.is_empty() {
            return None;
        }
        items.reverse();
        let mut out = String::from("\n\n## Bàn giao từ cuộc trao đổi gần nhất trong chat\n");
        out.push_str(
            "Workflow không có lịch sử chat của lượt trước. Dưới đây là các tin nhắn gần nhất; nếu tác giả đã duyệt một dàn ý trong chat, kết quả của phase phải nhất quán với dàn ý đó, không tạo lại từ đầu.\n",
        );
        for (role, content) in items {
            let text: String = content.chars().take(1500).collect();
            let label = if role == "user" {
                "Tác giả"
            } else {
                "Trợ lý"
            };
            out.push_str(&format!("- {label}: {}\n", text.replace('\n', " ")));
        }
        Some(out)
    }

    pub fn compose_phase_prompt(&self, phase: i32) -> Result<(String, Vec<Value>), IpcError> {
        let conn = self
            .pool
            .get()
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
        let phase_code = format!("planning_phase_{phase}");
        let codes = [
            "introduction".to_string(),
            "invariant_rule".to_string(),
            "planning_mode".to_string(),
            phase_code.clone(),
            "exception_handling".to_string(),
            "out_of_scope".to_string(),
            "tool_library".to_string(),
            "story_context".to_string(),
        ];

        let mut sections = Vec::with_capacity(codes.len());
        for code in &codes {
            let content: Option<String> = conn
                .query_row(
                    "SELECT content FROM prompt_blocks WHERE prompt_code = ?1",
                    params![code],
                    |row| row.get(0),
                )
                .optional()
                .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
            if let Some(content) = content.filter(|text| !text.trim().is_empty()) {
                sections.push(format!("# {code}\n{content}"));
            }
        }
        if !sections
            .iter()
            .any(|section| section.starts_with(&format!("# {phase_code}")))
        {
            return Err(IpcError::new(
                ErrorCode::ValidationFailed,
                format!("Prompt block '{phase_code}' is not seeded; run pending migrations"),
            ));
        }

        let mut stmt = conn
            .prepare(
                "SELECT t.code, t.description, t.input_schema
                 FROM prompt_block_tools pbt
                 JOIN prompt_blocks pb ON pb.id = pbt.prompt_block_id
                 JOIN tools t ON t.id = pbt.tool_id
                 WHERE pb.prompt_code = ?1 AND t.input_schema IS NOT NULL
                 ORDER BY t.code",
            )
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;
        let mut schemas: Vec<Value> = stmt
            .query_map(params![phase_code], |row| {
                let code: String = row.get(0)?;
                let description: Option<String> = row.get(1)?;
                let schema: String = row.get(2)?;
                Ok((code, description, schema))
            })
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?
            .filter_map(Result::ok)
            .filter_map(|(code, description, schema)| {
                serde_json::from_str::<Value>(&schema)
                    .ok()
                    .map(|parameters| {
                        json!({
                            "name": code,
                            "description": description.unwrap_or_default(),
                            "parameters": parameters,
                        })
                    })
            })
            .collect();

        // Only advertise tools this phase can actually use. The DB grants every planning tool to
        // every phase block, which made each workflow request carry ~70 large schemas; a Phase 2
        // request then spent most of its context budget on Phase 5/6 tool definitions.
        schemas.retain(|schema| {
            schema
                .get("name")
                .and_then(|value| value.as_str())
                .map(|name| tool_allowed_in_phase(phase, name))
                .unwrap_or(false)
        });

        let guidance = Self::phase_tool_guidance(phase);
        if !guidance.is_empty() {
            sections.push(format!(
                "# Tool guidance\n{CHARACTER_RULE}{guidance}",
                CHARACTER_RULE = "Character rule: every character named by the author must exist in require.character. Before referencing a character in a scene, theme argument or relationship, check it exists and call add_character(code?, display_name, nickname?, biological_sex=MALE|FEMALE|NONE, motif_code?, role?, notes?, declared_sources) when missing. Never drop secondary characters and never invent a name the author did not confirm. "
            ));
        }

        Ok((sections.join("\n\n"), schemas))
    }

    // Helper: Plan rounds for a phase according to SSOT domain planner rules
    fn build_rounds_for_phase(
        &self,
        phase: i32,
        subphase: Option<i32>,
        ws: &WorkingState,
        cfg: &WorkflowConfig,
    ) -> Result<Vec<Vec<PlannedTaskSpec>>, IpcError> {
        let flow = &ws.business_flow_data;
        let invalid = |message: String| IpcError::new(ErrorCode::ValidationFailed, message);
        let mut rounds = Vec::new();

        match phase {
            1 | 2 => {
                // Phase 1–2: sequential outline windows over the Arc ranges in the draft.
                // Before Phase 1 writes anything, fall back to the user-confirmed target
                // allocation so the phase can actually run on a fresh draft.
                let mut arc_ranges =
                    extract_arc_chapter_ranges(flow).map_err(|e| invalid(e.to_string()))?;
                if arc_ranges.is_empty() {
                    arc_ranges = Self::draft_arc_ranges(flow);
                }
                if arc_ranges.is_empty() {
                    arc_ranges = Self::planned_arc_ranges(flow);
                }
                if arc_ranges.is_empty() {
                    return Err(invalid(
                        "Phase 1–2 requires either an Arc outline or a confirmed target arc/chapter count"
                            .into(),
                    ));
                }

                for (arc_code, lo, hi) in arc_ranges {
                    let outline_runs = plan_arc_outline_runs(
                        &arc_code,
                        lo,
                        hi,
                        cfg.outline_cap,
                        cfg.outline_lookback,
                    )
                    .map_err(|e| invalid(e.to_string()))?;

                    for run in outline_runs {
                        let arc = run.arc_code.as_deref().unwrap_or(&arc_code);
                        let kind = if phase == 2 {
                            "chapter_outline"
                        } else {
                            "outline_window"
                        };
                        let task_id = if phase == 2 {
                            format!(
                                "p2:s0:chapter_outline:arc={arc}:ch={}..={}",
                                run.write.0, run.write.1
                            )
                        } else {
                            format_arc_chapter_task_identity(
                                phase,
                                WorkflowTaskKind::OutlineWindow,
                                arc,
                                run.write.0,
                                run.write.1,
                            )
                            .unwrap_or_else(|_| {
                                format!(
                                    "task:p{}:outline_window:{}:{}..={}",
                                    phase, arc, run.write.0, run.write.1
                                )
                            })
                        };

                        let spec = PlannedTaskSpec {
                            phase,
                            subphase: None,
                            task_kind: kind.to_string(),
                            assigned_scope: json!({
                                "arcCode": arc,
                                "readRange": run.read,
                                "writeRange": [run.write.0, run.write.1],
                                "arcsTarget": flow.require.project.target.arc_count.value.unwrap_or(0),
                                "chaptersTarget": flow.require.project.target.chapter_count.value.unwrap_or(0),
                            }),
                            dependencies: vec![],
                            deterministic_identity: Some(task_id),
                        };

                        rounds.push(vec![spec]);
                    }
                }
            }
            3 => {
                // Phase 3 Stage 1: chapter probes over the Arc ranges actually present in the draft.
                let mut arc_ranges =
                    extract_arc_chapter_ranges(flow).map_err(|e| invalid(e.to_string()))?;
                if arc_ranges.is_empty() {
                    arc_ranges = Self::draft_arc_ranges(flow);
                }
                if arc_ranges.is_empty() {
                    return Err(invalid(
                        "Phase 3 requires at least one Arc with chapters in the draft".into(),
                    ));
                }
                let total_chapters = arc_ranges.iter().map(|(_, _, hi)| *hi).max().unwrap_or(0);
                let probe_plan = plan_chapter_phase_with_context(
                    3,
                    &arc_ranges,
                    total_chapters,
                    cfg.unit_chapter,
                    cfg.max_parallel,
                    cfg.context_margin,
                )
                .map_err(|e| invalid(e.to_string()))?;

                for r in probe_plan.rounds {
                    let mut round_specs = Vec::new();
                    for t in r.tasks {
                        let task_id = t.task_identity.clone().ok_or_else(|| {
                            invalid("Phase 3 probe task is missing its canonical identity".into())
                        })?;
                        round_specs.push(PlannedTaskSpec {
                            phase: 3,
                            subphase: None,
                            task_kind: "world_probe".to_string(),
                            assigned_scope: json!({
                                "groupKey": t.group_key,
                                "range": [t.lo, t.hi],
                                "contextRange": t.chapter_context_range,
                            }),
                            dependencies: vec![],
                            deterministic_identity: Some(task_id),
                        });
                    }
                    rounds.push(round_specs);
                }
            }
            4 => {
                // Phase 4: scene configuration over the Arc ranges actually present in the draft.
                let mut arc_ranges =
                    extract_arc_chapter_ranges(flow).map_err(|e| invalid(e.to_string()))?;
                if arc_ranges.is_empty() {
                    arc_ranges = Self::draft_arc_ranges(flow);
                }
                if arc_ranges.is_empty() {
                    return Err(invalid(
                        "Phase 4 requires at least one Arc with chapters in the draft".into(),
                    ));
                }
                let total_chapters = arc_ranges.iter().map(|(_, _, hi)| *hi).max().unwrap_or(0);
                let phase_plan = plan_chapter_phase_with_context(
                    4,
                    &arc_ranges,
                    total_chapters,
                    cfg.unit_chapter,
                    cfg.max_parallel,
                    cfg.context_margin,
                )
                .map_err(|e| invalid(e.to_string()))?;

                for r in phase_plan.rounds {
                    let mut round_specs = Vec::new();
                    for t in r.tasks {
                        let task_id = t.task_identity.clone().unwrap_or_else(|| {
                            format!("task:p4:scene_config:{}:{}..={}", t.group_key, t.lo, t.hi)
                        });

                        round_specs.push(PlannedTaskSpec {
                            phase: 4,
                            subphase: None,
                            task_kind: "scene_config".to_string(),
                            assigned_scope: json!({
                                "groupKey": t.group_key,
                                "range": [t.lo, t.hi],
                                "contextRange": t.chapter_context_range,
                            }),
                            dependencies: vec![],
                            deterministic_identity: Some(task_id),
                        });
                    }
                    rounds.push(round_specs);
                }
            }
            5 => {
                // Phase 5: character profiles for one subphase, over the draft's characters.
                let sub = subphase.unwrap_or(1);
                let char_codes: Vec<String> = flow
                    .require
                    .character
                    .iter()
                    .map(|character| character.code.clone())
                    .collect();
                if char_codes.is_empty() {
                    return Err(invalid(
                        "Phase 5 requires at least one character in the draft".into(),
                    ));
                }

                let plan =
                    plan_character_subphase(sub, &char_codes, cfg.unit_character, cfg.max_parallel)
                        .map_err(|e| invalid(e.to_string()))?;

                for r in plan.rounds {
                    let mut round_specs = Vec::new();
                    for t in r.tasks {
                        let task_id = t.task_identity.clone().ok_or_else(|| {
                            invalid("Phase 5 task is missing its canonical identity".into())
                        })?;

                        round_specs.push(PlannedTaskSpec {
                            phase: 5,
                            subphase: Some(sub),
                            task_kind: "char_profile".to_string(),
                            assigned_scope: json!({ "groupKey": t.group_key, "range": [t.lo, t.hi] }),
                            dependencies: vec![],
                            deterministic_identity: Some(task_id),
                        });
                    }
                    rounds.push(round_specs);
                }
            }
            6 => {
                // Phase 6: relationship matrix over the directed pairs in the draft.
                let mut pairs: Vec<(String, String)> = flow
                    .optional
                    .relationship_phase
                    .relationship
                    .iter()
                    .map(|item| {
                        (
                            item.character.from_code.clone(),
                            item.character.to_code.clone(),
                        )
                    })
                    .collect();
                if pairs.is_empty() {
                    // Phase 6 creates the relationship matrix, so plan over character pairs.
                    let codes: Vec<String> = flow
                        .require
                        .character
                        .iter()
                        .map(|character| character.code.clone())
                        .collect();
                    // Undirected pairs only: directed duplication doubles the LLM rounds for no
                    // added information (a relationship row already stores from/to).
                    for i in 0..codes.len() {
                        for j in (i + 1)..codes.len() {
                            pairs.push((codes[i].clone(), codes[j].clone()));
                        }
                    }
                }
                if pairs.is_empty() {
                    return Err(invalid(
                        "Phase 6 requires at least one relationship pair in the draft".into(),
                    ));
                }

                let plan = plan_relationship_phase(&pairs, cfg.unit_relationship, cfg.max_parallel)
                    .map_err(|e| invalid(e.to_string()))?;

                for r in plan.rounds {
                    let mut round_specs = Vec::new();
                    for t in r.tasks {
                        let task_id = t.task_identity.clone().ok_or_else(|| {
                            invalid("Phase 6 task is missing its canonical identity".into())
                        })?;

                        round_specs.push(PlannedTaskSpec {
                            phase: 6,
                            subphase: None,
                            task_kind: "rel_matrix".to_string(),
                            assigned_scope: json!({ "groupKey": t.group_key, "range": [t.lo, t.hi] }),
                            dependencies: vec![],
                            deterministic_identity: Some(task_id),
                        });
                    }
                    rounds.push(round_specs);
                }
            }
            _ => unreachable!(),
        }

        // Mutating phases allocate entity codes/counters, which are not commutative across
        // parallel tasks; run one task per round so canonical patches apply deterministically.
        if phase != 3 {
            rounds = rounds
                .into_iter()
                .flatten()
                .map(|task| vec![task])
                .collect();
        }

        Ok(rounds)
    }

    // Helper: Query existing task receipts from SQLite
    fn query_existing_receipts(
        &self,
        conn: &Connection,
        draft_session_id: &Uuid,
        workflow_run_id: &Uuid,
    ) -> Result<Vec<ExistingReceiptRow>, IpcError> {
        let mut stmt = conn
            .prepare(
                "SELECT id, task_identity, evidence_json, base_version_seq, resulting_version_seq
                 FROM workflow_task_receipts
                 WHERE draft_session_id = ?1 AND workflow_run_id = ?2",
            )
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

        let rows = stmt
            .query_map(
                params![draft_session_id.to_string(), workflow_run_id.to_string()],
                |r| {
                    Ok(ExistingReceiptRow {
                        id: r.get(0)?,
                        task_identity: r.get(1)?,
                        evidence_json: r.get(2)?,
                        base_version_seq: r.get(3)?,
                        resulting_version_seq: r.get(4)?,
                    })
                },
            )
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

        Ok(rows)
    }

    // Helper: Query draft version summary by sequence
    fn get_head_version_summary(
        &self,
        conn: &Connection,
        draft_session_id: &Uuid,
        seq: i64,
    ) -> Result<Option<DraftVersionSummaryDto>, IpcError> {
        let summary: Option<DraftVersionSummaryDto> = conn
            .query_row(
                "SELECT seq, phase, subphase, reason, parent_seq, created_at, data_hash
                 FROM draft_versions
                 WHERE draft_session_id = ?1 AND seq = ?2",
                params![draft_session_id.to_string(), seq],
                |r| {
                    let s: i32 = r.get(0)?;
                    Ok(DraftVersionSummaryDto {
                        seq: s,
                        display_label: format!("v.{:02}", s),
                        phase: r.get(1)?,
                        subphase: r.get(2)?,
                        reason: r.get(3)?,
                        parent_seq: r.get(4)?,
                        created_at: r.get(5)?,
                        data_hash: r.get(6)?,
                    })
                },
            )
            .optional()
            .map_err(|e| IpcError::new(ErrorCode::DatabaseError, e.to_string()))?;

        Ok(summary)
    }

    fn emit_event(
        &self,
        tx: &Option<mpsc::Sender<WorkflowProgressEvent>>,
        event: WorkflowProgressEvent,
    ) {
        if let Some(ref sender) = tx {
            let _ = sender.try_send(event);
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct ExistingReceiptRow {
    id: String,
    task_identity: String,
    evidence_json: Option<String>,
    base_version_seq: i64,
    resulting_version_seq: Option<i64>,
}

/// Build a `decision_log` entry from approved prose. Pure helper so the capture fallback can be
/// tested without a database or Tauri runtime.
fn approved_outline_from(
    content: Option<&str>,
    existing_decisions: usize,
) -> Option<crate::domain::business_flow::DecisionLogEntry> {
    let text = content?.trim();
    if text.is_empty() {
        return None;
    }
    Some(crate::domain::business_flow::DecisionLogEntry {
        id: format!("decision-{}", existing_decisions + 1),
        summary: text.chars().take(20_000).collect(),
        source: crate::domain::business_flow::ProvenanceSource::User,
    })
}

#[cfg(test)]
mod approved_outline_tests {
    use super::approved_outline_from;

    #[test]
    fn captures_the_last_assistant_proposal_as_a_decision() {
        let entry = approved_outline_from(Some("  5 Arc: Gặp gỡ / Thử thách / Kết thúc  "), 0)
            .expect("proposal must be captured");
        assert_eq!(entry.id, "decision-1");
        assert!(entry.summary.starts_with("5 Arc: Gặp gỡ"));
        assert_eq!(
            entry.source,
            crate::domain::business_flow::ProvenanceSource::User
        );
        assert!(approved_outline_from(Some("   "), 2).is_none());
        assert!(approved_outline_from(None, 0).is_none());
    }
}

/// Tool surface for one workflow phase. Read tools are stripped later by the executor and do not
/// need to appear here. Keep this list aligned with the phase guidance text.
pub(crate) fn phase_tool_codes(phase: i32) -> &'static [&'static str] {
    match phase {
        1 => &[
            "add_arc",
            "set_arc",
            "set_arc_motif",
            "add_arc_event_block",
            "set_arc_event_block",
            "delete_arc",
            "delete_arc_event_block",
            "add_theme",
            "set_theme",
            "add_theme_argument",
            "set_theme_argument",
            "delete_theme",
            "delete_theme_argument",
            "add_character",
            "set_character",
            "add_project_genre",
            "save_approved_outline",
        ],
        2 => &[
            "add_chapter",
            "set_chapter",
            "delete_chapter",
            "add_scene",
            "set_scene",
            "delete_scene",
            "save_approved_outline",
        ],
        3 => &[
            "add_world_setting",
            "set_world_setting",
            "delete_world_setting",
            "add_lore_entry",
            "set_lore_entry",
            "delete_lore_entry",
            "save_approved_outline",
        ],
        4 => &[
            "set_scene_config",
            "add_scene_character",
            "delete_scene_character",
            "add_scene_lore_entry",
            "delete_scene_lore_entry",
        ],
        5 => &[
            "add_character_appearance",
            "set_character_appearance",
            "add_character_psychological",
            "set_character_psychological",
            "add_character_memory",
            "set_character_memory",
            "delete_character_memory",
            "add_character_competence",
            "set_character_competence",
            "delete_character_competence",
        ],
        6 => &[
            "add_relationship",
            "set_relationship",
            "delete_relationship",
            "add_relationship_role",
            "set_relationship_state",
        ],
        _ => &[
            "set_project",
            "add_character",
            "set_character",
            "add_project_genre",
            "delete_project_genre",
            "save_approved_outline",
        ],
    }
}

pub(crate) fn tool_allowed_in_phase(phase: i32, name: &str) -> bool {
    phase_tool_codes(phase).contains(&name)
}

#[cfg(test)]
mod phase_tool_surface_tests {
    use super::*;

    #[test]
    fn each_phase_only_advertises_its_own_tools() {
        assert!(tool_allowed_in_phase(2, "add_chapter"));
        assert!(!tool_allowed_in_phase(2, "add_arc"));
        assert!(!tool_allowed_in_phase(2, "add_character_appearance"));
        assert!(tool_allowed_in_phase(5, "add_character_psychological"));
        assert!(tool_allowed_in_phase(6, "add_relationship"));
        assert!(!tool_allowed_in_phase(6, "add_scene"));
    }
}
