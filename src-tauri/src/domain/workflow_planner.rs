//! Pure Segmented Workflow planner (`REQ-WF-001`, `core.md` §3.8–§3.9, `be-core.md` §6.3–§6.4.1, `T-UNIT-03`, `P3S1-P`, `P3S1-N`, `phase3_sprint1_deep0-F1`).
//!
//! This module is 100% pure Rust with zero dependency on `rusqlite`, `tokio::fs`, `reqwest`, or `tauri`.
//!
//! Responsibilities:
//! 1. **Phase 1–2 Sequential Outline Windows (`plan_outline_runs`, `plan_arc_outline_runs`):**
//!    - Splits an Arc's chapter range `lo..=hi` into sequential runs bounded by `cap` (`OUTLINE_CAP`, default 50)
//!      and `lookback` (`OUTLINE_LOOKBACK`, default 10).
//!    - First run in an Arc has `read_range = None` and writes up to `cap` chapters.
//!    - Subsequent runs in the same Arc read back up to `lookback` chapters (`max(lo, start - lookback)..=(start - 1)`)
//!      and write up to `cap - lookback` new chapters.
//!    - Rejects invalid parameters (`lo <= 0`, `lo > hi`, `cap <= 0`, `lookback < 0`, `lookback >= cap`)
//!      with `PlannerError::ValidationFailed` and produces zero tasks (`phase3_sprint1_deep0-F1`, `P3S1-N`).
//! 2. **Phase 3–6 & `lookup_outline` Balanced Splitting & Parallel Round Grouping (`split_balanced`, `plan_phase`, `context_range`):**
//!    - `split_balanced(lo, hi, unit)` divides `lo..=hi` into `k = ceil(total / unit)` contiguous balanced ranges
//!      whose lengths differ by at most 1 (`base = total / k`, first `extra = total % k` slices get `base + 1`).
//!    - `split_balanced_tasks(tasks, max_parallel)` / `chunk_into_rounds(tasks, max_parallel)` groups a list of tasks
//!      into sequential rounds of at most `max_parallel` (`P`) tasks per round (`15` tasks at `P = 6` -> `6 + 6 + 3`),
//!      never collapsing all tasks into one unbounded round (`P3S1-N`).
//!    - `plan_phase(items, unit, max_parallel)` freezes `max_parallel` (`P`) inside the returned `PhasePlan`
//!      (never hardcoding 6 into SQLite) and assigns a deterministic 0-based `task_index` within each round
//!      and `global_task_index` across the plan (`REQ-WF-001`, `core.md` §3.9 `Q3.5`).
//!    - `context_range(s, e, total_chapters, margin)` computes `[max(1, s - margin), min(total_chapters, e + margin)]`
//!      strictly for chapter-unit tasks (Phase 3 stage 1 probe & Phase 4 scene config).
//! 3. **Canonical `task_identity` Grammar (`be-core.md` §6.4.1):**
//!    - Deterministic `task_identity` formatting for `outline_window`, `world_probe`, `world_write`,
//!      `scene_config`, `char_profile`, and `rel_matrix`.

use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use thiserror::Error;

use super::business_flow::BusinessFlowData;

/// Default runtime parameter `OUTLINE_CAP = 50` (`core.md` §3.9).
pub const DEFAULT_OUTLINE_CAP: i32 = 50;

/// Default runtime parameter `OUTLINE_LOOKBACK = 10` (`core.md` §3.9).
pub const DEFAULT_OUTLINE_LOOKBACK: i32 = 10;

/// Default runtime parameter `OUTLINE_PAGE = 50` (`core.md` §3.8, §3.9).
pub const DEFAULT_OUTLINE_PAGE: i32 = 50;

/// Default runtime parameter `UNIT_CHAPTER = 30` (`core.md` §3.9).
pub const DEFAULT_UNIT_CHAPTER: i32 = 30;

/// Default runtime parameter `CONTEXT_MARGIN = 5` (`core.md` §3.9).
pub const DEFAULT_CONTEXT_MARGIN: i32 = 5;

/// Default runtime parameter `MAX_PARALLEL = 6` (`core.md` §3.9).
/// Note: Always passed as a configurable parameter and frozen into `PhasePlan.max_parallel`,
/// never hardcoded into SQLite (`REQ-WF-001`).
pub const DEFAULT_MAX_PARALLEL: usize = 6;

/// Default runtime parameter `DRAFT_LOCK_TIMEOUT = 30` seconds (`core.md` §3.9).
pub const DEFAULT_DRAFT_LOCK_TIMEOUT_SECS: u64 = 30;

/// Pure workflow planner error classification (`P3S1-N`, `phase3_sprint1_deep0-F1`).
#[derive(Debug, Clone, PartialEq, Eq, Error, Serialize, Deserialize)]
pub enum PlannerError {
    /// Configuration or range validation failed (`ValidationFailed`, `P3S1-N`, `phase3_sprint1_deep0-F1`).
    #[error("ValidationFailed (P3S1-N): {message}")]
    ValidationFailed { message: String },
}

impl PlannerError {
    pub fn validation_failed(message: impl Into<String>) -> Self {
        Self::ValidationFailed {
            message: message.into(),
        }
    }

    pub const fn code(&self) -> &'static str {
        match self {
            Self::ValidationFailed { .. } => "ValidationFailed",
        }
    }
}

/// Runtime configuration parameters for Segmented Workflow (`core.md` §3.9).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkflowConfig {
    pub outline_cap: i32,
    pub outline_lookback: i32,
    pub outline_page: i32,
    pub unit_chapter: i32,
    pub context_margin: i32,
    pub max_parallel: usize,
    pub draft_lock_timeout_secs: u64,
}

impl Default for WorkflowConfig {
    fn default() -> Self {
        Self {
            outline_cap: DEFAULT_OUTLINE_CAP,
            outline_lookback: DEFAULT_OUTLINE_LOOKBACK,
            outline_page: DEFAULT_OUTLINE_PAGE,
            unit_chapter: DEFAULT_UNIT_CHAPTER,
            context_margin: DEFAULT_CONTEXT_MARGIN,
            max_parallel: DEFAULT_MAX_PARALLEL,
            draft_lock_timeout_secs: DEFAULT_DRAFT_LOCK_TIMEOUT_SECS,
        }
    }
}

impl WorkflowConfig {
    /// Validates runtime configuration invariants (`0 <= outline_lookback < outline_cap`, positive units and `max_parallel`).
    pub fn validate(&self) -> Result<(), PlannerError> {
        if self.outline_cap <= 0 || self.outline_lookback < 0 || self.outline_lookback >= self.outline_cap {
            return Err(PlannerError::validation_failed(format!(
                "invalid outline window config: require 0 <= lookback ({}) < cap ({})",
                self.outline_lookback, self.outline_cap
            )));
        }
        if self.outline_page <= 0 {
            return Err(PlannerError::validation_failed(format!(
                "outline_page must be >= 1, got {}",
                self.outline_page
            )));
        }
        if self.unit_chapter <= 0 {
            return Err(PlannerError::validation_failed(format!(
                "unit_chapter must be >= 1, got {}",
                self.unit_chapter
            )));
        }
        if self.context_margin < 0 {
            return Err(PlannerError::validation_failed(format!(
                "context_margin must be >= 0, got {}",
                self.context_margin
            )));
        }
        if self.max_parallel == 0 {
            return Err(PlannerError::validation_failed(
                "max_parallel (P) must be >= 1",
            ));
        }
        Ok(())
    }
}

/// A single sequential outline window run inside an Arc for Phase 1–2 (`core.md` §3.9, `be-core.md` §6.3.1).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OutlineWindow {
    /// Optional Arc `code` when planned for a named Arc (`plan_arc_outline_runs`).
    pub arc_code: Option<String>,
    /// 0-based index of this run within the Arc.
    pub run_index_in_arc: u32,
    /// Optional inclusive chapter position range `(from_ch, to_ch)` read back from the previous run in the same Arc.
    pub read: Option<(i32, i32)>,
    /// Inclusive chapter position range `(start_ch, end_ch)` written in this run.
    pub write: (i32, i32),
}

impl OutlineWindow {
    /// Alias accessor matching `be-core.md` §6.3.1 `read_range`.
    pub const fn read_range(&self) -> Option<(i32, i32)> {
        self.read
    }

    /// Alias accessor matching `be-core.md` §6.3.1 `write_range`.
    pub const fn write_range(&self) -> (i32, i32) {
        self.write
    }

    /// Returns `(read, write)` tuple representation `((Option<(i32, i32)>), (i32, i32))` (`T-UNIT-03`).
    pub const fn as_tuple(&self) -> (Option<(i32, i32)>, (i32, i32)) {
        (self.read, self.write)
    }

    /// Total chapters in context for this run (`read_count + write_count <= cap`).
    pub const fn total_context_chapters(&self) -> i32 {
        let write_len = self.write.1 - self.write.0 + 1;
        let read_len = match self.read {
            Some((r_lo, r_hi)) => r_hi - r_lo + 1,
            None => 0,
        };
        read_len + write_len
    }

    /// Canonical `task_identity` for Phase 1 or Phase 2 (`p{phase}:s0:outline_window:arc={arc_code}:ch={lo}..{hi}`).
    pub fn task_identity(&self, phase: i32, fallback_arc_code: &str) -> Result<String, PlannerError> {
        if phase != 1 && phase != 2 {
            return Err(PlannerError::validation_failed(format!(
                "outline_window task_identity is only valid for phase 1 or 2, got phase={phase}"
            )));
        }
        let arc = self
            .arc_code
            .as_deref()
            .unwrap_or(fallback_arc_code)
            .trim();
        if arc.is_empty() {
            return Err(PlannerError::validation_failed(
                "arc_code must not be empty for outline_window task_identity",
            ));
        }
        Ok(format!(
            "p{phase}:s0:outline_window:arc={arc}:ch={}..{}",
            self.write.0, self.write.1
        ))
    }
}

/// Type alias matching `be-core.md` §6.3.1 `OutlineWindowRun`.
pub type OutlineWindowRun = OutlineWindow;

/// Plans sequential outline runs for a single Arc spanning global chapter positions `lo..=hi`
/// (`core.md` §3.9, `be-core.md` §6.3.1, `T-UNIT-03`, `P3S1-P`, `phase3_sprint1_deep0-F1`).
///
/// Rules:
/// - Requires `1 <= lo <= hi`, `cap >= 1`, and `0 <= lookback < cap`.
/// - If any requirement is violated (including `lookback >= cap`), returns `Err(PlannerError::ValidationFailed)`
///   and generates zero runs (`phase3_sprint1_deep0-F1`, `P3S1-N`).
/// - Run 0 has `read = None` and writes `lo..=min(hi, lo + cap - 1)`.
/// - Run `k >= 1` has `read = if lookback > 0 { Some((max(lo, start - lookback), start - 1)) } else { None }`
///   and writes `start..=min(hi, start + (cap - lookback) - 1)`.
pub fn plan_outline_runs(
    lo: i32,
    hi: i32,
    cap: i32,
    lookback: i32,
) -> Result<Vec<OutlineWindow>, PlannerError> {
    if lo < 1 || lo > hi {
        return Err(PlannerError::validation_failed(format!(
            "invalid chapter range lo={lo}..=hi={hi}: require 1 <= lo <= hi"
        )));
    }
    if cap <= 0 || lookback < 0 || lookback >= cap {
        return Err(PlannerError::validation_failed(format!(
            "invalid outline window parameters (cap={cap}, lookback={lookback}): require 0 <= lookback < cap"
        )));
    }

    let mut runs = Vec::new();
    let mut start = lo;
    let mut idx = 0u32;

    while start <= hi {
        let is_first = runs.is_empty();
        let write_size = if is_first { cap } else { cap - lookback };
        let end = std::cmp::min(hi, start + write_size - 1);
        let read = if !is_first && lookback > 0 {
            Some((std::cmp::max(lo, start - lookback), start - 1))
        } else {
            None
        };

        runs.push(OutlineWindow {
            arc_code: None,
            run_index_in_arc: idx,
            read,
            write: (start, end),
        });

        start = end + 1;
        idx += 1;
    }

    Ok(runs)
}

/// Plans sequential outline runs for a named `arc_code` spanning `lo..=hi` (`be-core.md` §6.3.1).
pub fn plan_arc_outline_runs(
    arc_code: &str,
    lo: i32,
    hi: i32,
    cap: i32,
    lookback: i32,
) -> Result<Vec<OutlineWindow>, PlannerError> {
    let trimmed = arc_code.trim();
    if trimmed.is_empty() {
        return Err(PlannerError::validation_failed(
            "arc_code must not be empty in plan_arc_outline_runs",
        ));
    }
    let mut runs = plan_outline_runs(lo, hi, cap, lookback)?;
    for run in &mut runs {
        run.arc_code = Some(trimmed.to_string());
    }
    Ok(runs)
}

/// Splits an inclusive integer range `lo..=hi` into `k = ceil((hi - lo + 1) / unit)` balanced contiguous sub-ranges
/// (`core.md` §3.9, `be-core.md` §6.3.2, `T-UNIT-03`).
///
/// Example: `lo = 1, hi = 55, unit = 30` -> `total = 55, k = 2, base = 27, extra = 1` -> `[(1, 28), (29, 55)]` (lengths 28 and 27).
pub fn split_balanced(lo: i32, hi: i32, unit: i32) -> Result<Vec<(i32, i32)>, PlannerError> {
    if lo < 1 || lo > hi {
        return Err(PlannerError::validation_failed(format!(
            "invalid range lo={lo}..=hi={hi} for split_balanced: require 1 <= lo <= hi"
        )));
    }
    if unit <= 0 {
        return Err(PlannerError::validation_failed(format!(
            "invalid unit={unit} for split_balanced: require unit >= 1"
        )));
    }

    let total = hi - lo + 1;
    let k = (total + unit - 1) / unit;
    let base = total / k;
    let extra = total % k;

    let mut out = Vec::with_capacity(k as usize);
    let mut s = lo;
    for i in 0..k {
        let len = base + if i < extra { 1 } else { 0 };
        let e = s + len - 1;
        out.push((s, e));
        s = e + 1;
    }

    Ok(out)
}

/// Partitions a flat list of tasks into sequential rounds of at most `max_parallel` (`P`) tasks each
/// (`P3S1-N`: 15 tasks at `P = 6` -> rounds of `6 + 6 + 3`, never one round of 15).
pub fn chunk_into_rounds<T: Clone>(
    tasks: &[T],
    max_parallel: usize,
) -> Result<Vec<Vec<T>>, PlannerError> {
    if max_parallel == 0 {
        return Err(PlannerError::validation_failed(
            "max_parallel (P) must be >= 1 when partitioning tasks into rounds",
        ));
    }
    if tasks.is_empty() {
        return Ok(Vec::new());
    }
    Ok(tasks
        .chunks(max_parallel)
        .map(|chunk| chunk.to_vec())
        .collect())
}

/// Computes the expanded chapter context range `[max(1, s - margin), min(total_chapters, e + margin)]`
/// for chapter-unit tasks (`core.md` §3.9, `be-core.md` §6.3.2).
///
/// Only valid for tasks whose unit is chapters (Phase 3 stage 1 probe and Phase 4 scene config).
pub fn context_range(
    s: i32,
    e: i32,
    total_chapters: i32,
    margin: i32,
) -> Result<(i32, i32), PlannerError> {
    if s < 1 || s > e || total_chapters < e || margin < 0 {
        return Err(PlannerError::validation_failed(format!(
            "invalid context_range arguments (s={s}, e={e}, total_chapters={total_chapters}, margin={margin})"
        )));
    }
    Ok((
        std::cmp::max(1, s - margin),
        std::cmp::min(total_chapters, e + margin),
    ))
}

/// A planned task slice within a Phase 3–6 or `lookup_outline` round (`core.md` §3.9, `be-core.md` §6.3.2).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlannedTask {
    /// Group key (e.g., `arc_code` for chapter tasks, depth key for world_write, subphase key for char_profile).
    pub group_key: String,
    /// Inclusive start index/position `s` of the balanced range.
    pub lo: i32,
    /// Inclusive end index/position `e` of the balanced range.
    pub hi: i32,
    /// 0-based index of the round this task belongs to.
    pub round_index: usize,
    /// Deterministic 0-based application order index within its round (`0..round.len()`, `core.md` §3.9 `Q3.5`).
    pub task_index: usize,
    /// Deterministic 0-based index across the entire `PhasePlan`.
    pub global_task_index: usize,
    /// Optional canonical `task_identity` string when planned with phase/task-kind metadata (`be-core.md` §6.4.1).
    pub task_identity: Option<String>,
    /// Optional expanded chapter read context `(ctx_lo, ctx_hi)` for chapter-unit tasks (`Phase 3 probe`, `Phase 4`).
    pub chapter_context_range: Option<(i32, i32)>,
}

impl PlannedTask {
    /// Returns `(group_key, lo, hi)` tuple matching the reference specification in `core.md` §3.9.
    pub fn as_tuple(&self) -> (&str, i32, i32) {
        (&self.group_key, self.lo, self.hi)
    }

    /// Number of items/chapters `(hi - lo + 1)` covered by this task.
    pub const fn len(&self) -> i32 {
        self.hi - self.lo + 1
    }

    pub const fn is_empty(&self) -> bool {
        self.hi < self.lo
    }
}

/// A single execution round in a `PhasePlan`, containing at most `max_parallel` (`P`) independent subagent tasks.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlannedRound {
    pub round_index: usize,
    pub tasks: Vec<PlannedTask>,
}

impl PlannedRound {
    pub fn len(&self) -> usize {
        self.tasks.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }
}

/// Frozen multi-round execution plan for a phase or sub-phase (`REQ-WF-001`, `core.md` §3.9, `T-UNIT-03`).
///
/// Freezes `unit` and `max_parallel` (`P`) per plan instance so the orchestrator never hardcodes `6` into SQLite.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PhasePlan {
    /// Frozen unit size used to split each input group item.
    pub unit: i32,
    /// Frozen maximum parallel tasks per round (`P`).
    pub max_parallel: usize,
    /// Sequential rounds, each containing `1..=max_parallel` tasks.
    pub rounds: Vec<PlannedRound>,
}

impl PhasePlan {
    /// Total number of tasks across all rounds.
    pub fn total_tasks(&self) -> usize {
        self.rounds.iter().map(|r| r.tasks.len()).sum()
    }

    /// Total number of sequential rounds.
    pub fn total_rounds(&self) -> usize {
        self.rounds.len()
    }

    /// Returns the number of tasks in each round (e.g. `vec![6, 5]` or `vec![6, 6, 3]`).
    pub fn round_sizes(&self) -> Vec<usize> {
        self.rounds.iter().map(|r| r.tasks.len()).collect()
    }

    /// Flattens all tasks across rounds in deterministic execution order.
    pub fn all_tasks(&self) -> Vec<&PlannedTask> {
        self.rounds.iter().flat_map(|r| r.tasks.iter()).collect()
    }
}

/// Plans balanced subagent tasks across input items `(group_key, lo, hi)` and packs them into
/// sequential rounds of at most `max_parallel` (`P`) tasks per round (`core.md` §3.9, `be-core.md` §6.3.2, `T-UNIT-03`, `P3S1-N`).
///
/// - Each item `(group_key, lo, hi)` is split via `split_balanced(lo, hi, unit)`.
/// - Tasks from consecutive items may share a round up to `max_parallel`, while each individual task
///   strictly retains its own `group_key` (two Arcs are never merged into a single task).
/// - Validates `unit >= 1`, `max_parallel >= 1`, and `1 <= lo <= hi` for every item before producing any task.
pub fn plan_phase<S: AsRef<str>>(
    items: &[(S, i32, i32)],
    unit: i32,
    max_parallel: usize,
) -> Result<PhasePlan, PlannerError> {
    if unit <= 0 {
        return Err(PlannerError::validation_failed(format!(
            "invalid unit={unit} in plan_phase: require unit >= 1"
        )));
    }
    if max_parallel == 0 {
        return Err(PlannerError::validation_failed(
            "invalid max_parallel=0 in plan_phase: require max_parallel >= 1",
        ));
    }

    let mut raw_slices: Vec<(String, i32, i32)> = Vec::new();
    for (group_key, lo, hi) in items {
        let key = group_key.as_ref().trim();
        if key.is_empty() {
            return Err(PlannerError::validation_failed(
                "group_key must not be empty in plan_phase",
            ));
        }
        let slices = split_balanced(*lo, *hi, unit)?;
        for (s, e) in slices {
            raw_slices.push((key.to_string(), s, e));
        }
    }

    let mut rounds = Vec::new();
    for (round_idx, chunk) in raw_slices.chunks(max_parallel).enumerate() {
        let mut round_tasks = Vec::with_capacity(chunk.len());
        for (task_idx, (group_key, s, e)) in chunk.iter().enumerate() {
            let global_idx = round_idx * max_parallel + task_idx;
            round_tasks.push(PlannedTask {
                group_key: group_key.clone(),
                lo: *s,
                hi: *e,
                round_index: round_idx,
                task_index: task_idx,
                global_task_index: global_idx,
                task_identity: None,
                chapter_context_range: None,
            });
        }
        rounds.push(PlannedRound {
            round_index: round_idx,
            tasks: round_tasks,
        });
    }

    Ok(PhasePlan {
        unit,
        max_parallel,
        rounds,
    })
}

/// Canonical task kind for Segmented Workflow (`be-core.md` §6.4.1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkflowTaskKind {
    /// Phase 1–2 sequential outline window (`p{1|2}:s0:outline_window:arc={arc_code}:ch={lo}..{hi}`).
    OutlineWindow,
    /// Phase 3 Stage 1 chapter probe (`p3:s0:world_probe:arc={arc_code}:ch={lo}..{hi}`).
    WorldProbe,
    /// Phase 3 Stage 3 hierarchical world/lore write (`p3:s0:world_write:depth={d}:codes={c1},{c2},...`).
    WorldWrite,
    /// Phase 4 scene configuration (`p4:s0:scene_config:arc={arc_code}:ch={lo}..{hi}`).
    SceneConfig,
    /// Phase 5.1 / 5.2 / 5.3 character profile (`p5:s{1|2|3}:char_profile:codes={c1},{c2},...`).
    CharProfile,
    /// Phase 6 relationship matrix (`p6:s0:rel_matrix:pairs={from1}>{to1},{from2}>{to2},...`).
    RelMatrix,
}

impl WorkflowTaskKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OutlineWindow => "outline_window",
            Self::WorldProbe => "world_probe",
            Self::WorldWrite => "world_write",
            Self::SceneConfig => "scene_config",
            Self::CharProfile => "char_profile",
            Self::RelMatrix => "rel_matrix",
        }
    }
}

/// Formats the canonical `task_identity` for chapter-scoped tasks (`outline_window`, `world_probe`, `scene_config`)
/// per `be-core.md` §6.4.1.
pub fn format_arc_chapter_task_identity(
    phase: i32,
    kind: WorkflowTaskKind,
    arc_code: &str,
    lo: i32,
    hi: i32,
) -> Result<String, PlannerError> {
    let arc = arc_code.trim();
    if arc.is_empty() {
        return Err(PlannerError::validation_failed(
            "arc_code must not be empty for chapter task_identity",
        ));
    }
    if lo < 1 || lo > hi {
        return Err(PlannerError::validation_failed(format!(
            "invalid chapter range {lo}..{hi} for task_identity"
        )));
    }
    match (phase, kind) {
        (1 | 2, WorkflowTaskKind::OutlineWindow)
        | (3, WorkflowTaskKind::WorldProbe)
        | (4, WorkflowTaskKind::SceneConfig) => Ok(format!(
            "p{phase}:s0:{}:arc={arc}:ch={lo}..{hi}",
            kind.as_str()
        )),
        _ => Err(PlannerError::validation_failed(format!(
            "invalid (phase={phase}, kind={:?}) combination for arc-chapter task_identity",
            kind
        ))),
    }
}

/// Formats the canonical `task_identity` for Phase 3 Stage 3 hierarchical world writes
/// (`p3:s0:world_write:depth={d}:codes={c1},{c2},...` with codes sorted lexicographically per `be-core.md` §6.4.1).
pub fn format_world_write_task_identity<S: AsRef<str>>(
    depth: u32,
    codes: &[S],
) -> Result<String, PlannerError> {
    if codes.is_empty() {
        return Err(PlannerError::validation_failed(
            "world_write task_identity requires at least one entity code",
        ));
    }
    let mut sorted: Vec<String> = Vec::with_capacity(codes.len());
    for c in codes {
        let trimmed = c.as_ref().trim();
        if trimmed.is_empty() {
            return Err(PlannerError::validation_failed(
                "world_write entity code must not be empty",
            ));
        }
        sorted.push(trimmed.to_string());
    }
    sorted.sort();
    sorted.dedup();
    Ok(format!(
        "p3:s0:world_write:depth={depth}:codes={}",
        sorted.join(",")
    ))
}

/// Formats the canonical `task_identity` for Phase 5 sub-phases `5.1`, `5.2`, `5.3`
/// (`p5:s{subphase}:char_profile:codes={char_code_1},{char_code_2},...` per `be-core.md` §6.4.1).
pub fn format_char_profile_task_identity<S: AsRef<str>>(
    subphase: i32,
    char_codes: &[S],
) -> Result<String, PlannerError> {
    if !(1..=3).contains(&subphase) {
        return Err(PlannerError::validation_failed(format!(
            "Phase 5 char_profile requires subphase in 1..=3, got {subphase}"
        )));
    }
    if char_codes.is_empty() {
        return Err(PlannerError::validation_failed(
            "char_profile task_identity requires at least one character code",
        ));
    }
    let mut cleaned = Vec::with_capacity(char_codes.len());
    for c in char_codes {
        let trimmed = c.as_ref().trim();
        if trimmed.is_empty() {
            return Err(PlannerError::validation_failed(
                "character code must not be empty in char_profile task_identity",
            ));
        }
        cleaned.push(trimmed.to_string());
    }
    Ok(format!(
        "p5:s{subphase}:char_profile:codes={}",
        cleaned.join(",")
    ))
}

/// Formats the canonical `task_identity` for Phase 6 relationship pairs
/// (`p6:s0:rel_matrix:pairs={from1}>{to1},{from2}>{to2},...` per `be-core.md` §6.4.1).
pub fn format_rel_matrix_task_identity<S1: AsRef<str>, S2: AsRef<str>>(
    pairs: &[(S1, S2)],
) -> Result<String, PlannerError> {
    if pairs.is_empty() {
        return Err(PlannerError::validation_failed(
            "rel_matrix task_identity requires at least one (from_code, to_code) pair",
        ));
    }
    let mut formatted = Vec::with_capacity(pairs.len());
    for (from, to) in pairs {
        let f = from.as_ref().trim();
        let t = to.as_ref().trim();
        if f.is_empty() || t.is_empty() {
            return Err(PlannerError::validation_failed(
                "relationship pair codes must not be empty",
            ));
        }
        formatted.push(format!("{f}>{t}"));
    }
    Ok(format!("p6:s0:rel_matrix:pairs={}", formatted.join(",")))
}

/// Plans chapter-based Phase 3 Stage 1 (`world_probe`) or Phase 4 (`scene_config`) rounds with
/// canonical `task_identity` and `±context_margin` chapter context ranges (`core.md` §3.9, `be-core.md` §6.3.2).
pub fn plan_chapter_phase_with_context<S: AsRef<str>>(
    phase: i32,
    arc_ranges: &[(S, i32, i32)],
    total_chapters: i32,
    unit: i32,
    max_parallel: usize,
    margin: i32,
) -> Result<PhasePlan, PlannerError> {
    let kind = match phase {
        3 => WorkflowTaskKind::WorldProbe,
        4 => WorkflowTaskKind::SceneConfig,
        _ => {
            return Err(PlannerError::validation_failed(format!(
                "plan_chapter_phase_with_context only supports phase 3 (world_probe) or phase 4 (scene_config), got phase={phase}"
            )))
        }
    };

    let mut plan = plan_phase(arc_ranges, unit, max_parallel)?;
    for round in &mut plan.rounds {
        for task in &mut round.tasks {
            let ctx = context_range(task.lo, task.hi, total_chapters, margin)?;
            let identity =
                format_arc_chapter_task_identity(phase, kind, &task.group_key, task.lo, task.hi)?;
            task.chapter_context_range = Some(ctx);
            task.task_identity = Some(identity);
        }
    }
    Ok(plan)
}

/// Plans Phase 3 Stage 3 hierarchical world-building write rounds (`core.md` §3.9, `be-core.md` §6.3.2).
///
/// Groups `world_settings` by tree depth (`depth = 0` for root nodes with `parent_code == None`,
/// `depth = parent_depth + 1` for children) so parent layers always complete in earlier rounds
/// before child layers are scheduled.
/// Rejects orphan `parent_code` or cycles with `PlannerError::ValidationFailed`.
pub fn plan_world_write_by_depth(
    nodes: &[(String, Option<String>)],
    unit: i32,
    max_parallel: usize,
) -> Result<Vec<PhasePlan>, PlannerError> {
    if unit <= 0 {
        return Err(PlannerError::validation_failed(
            "unit must be >= 1 for plan_world_write_by_depth",
        ));
    }
    if max_parallel == 0 {
        return Err(PlannerError::validation_failed(
            "max_parallel must be >= 1 for plan_world_write_by_depth",
        ));
    }
    if nodes.is_empty() {
        return Ok(Vec::new());
    }

    let mut parent_map: BTreeMap<String, Option<String>> = BTreeMap::new();
    for (code, parent) in nodes {
        let c = code.trim();
        if c.is_empty() {
            return Err(PlannerError::validation_failed(
                "world_setting code must not be empty",
            ));
        }
        let p = parent
            .as_ref()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty());
        if parent_map.insert(c.to_string(), p).is_some() {
            return Err(PlannerError::validation_failed(format!(
                "duplicate world_setting code '{c}' in plan_world_write_by_depth"
            )));
        }
    }

    // Compute depth for each node, detecting missing parents or cycles
    let mut depths: BTreeMap<u32, Vec<String>> = BTreeMap::new();
    for code in parent_map.keys() {
        let mut depth = 0u32;
        let mut curr = code.as_str();
        let mut visited = BTreeSet::new();
        visited.insert(curr.to_string());

        while let Some(Some(parent_code)) = parent_map.get(curr) {
            if !parent_map.contains_key(parent_code) {
                return Err(PlannerError::validation_failed(format!(
                    "world_setting '{code}' references non-existent parent_code '{parent_code}'"
                )));
            }
            if !visited.insert(parent_code.clone()) {
                return Err(PlannerError::validation_failed(format!(
                    "cycle detected in world_settings hierarchy involving '{parent_code}'"
                )));
            }
            depth += 1;
            curr = parent_code.as_str();
        }
        depths.entry(depth).or_default().push(code.clone());
    }

    let mut depth_plans = Vec::with_capacity(depths.len());
    for (depth, mut codes_at_depth) in depths {
        codes_at_depth.sort();
        let group_key = format!("depth_{depth}");
        let items = vec![(group_key, 1, codes_at_depth.len() as i32)];
        let mut plan = plan_phase(&items, unit, max_parallel)?;
        for round in &mut plan.rounds {
            for task in &mut round.tasks {
                let start_idx = (task.lo - 1) as usize;
                let end_idx = task.hi as usize;
                let slice = &codes_at_depth[start_idx..end_idx];
                task.task_identity = Some(format_world_write_task_identity(depth, slice)?);
                // Entity-indexed tasks NEVER apply chapter ±CONTEXT_MARGIN (`core.md` §3.9)
                task.chapter_context_range = None;
            }
        }
        depth_plans.push(plan);
    }

    Ok(depth_plans)
}

/// Plans Phase 5 character profile rounds for a single subphase (`1`, `2`, or `3`) (`core.md` §3.9, `be-core.md` §6.3.2).
///
/// Sub-phases `5.1`, `5.2`, `5.3` execute sequentially and are never mixed in the same round.
/// Entity slices never apply chapter `±CONTEXT_MARGIN`.
pub fn plan_character_subphase<S: AsRef<str>>(
    subphase: i32,
    character_codes: &[S],
    unit: i32,
    max_parallel: usize,
) -> Result<PhasePlan, PlannerError> {
    if !(1..=3).contains(&subphase) {
        return Err(PlannerError::validation_failed(format!(
            "Phase 5 requires subphase in 1..=3, got {subphase}"
        )));
    }
    if unit <= 0 {
        return Err(PlannerError::validation_failed(
            "unit must be >= 1 for plan_character_subphase",
        ));
    }
    if max_parallel == 0 {
        return Err(PlannerError::validation_failed(
            "max_parallel must be >= 1 for plan_character_subphase",
        ));
    }
    if character_codes.is_empty() {
        return Ok(PhasePlan {
            unit,
            max_parallel,
            rounds: Vec::new(),
        });
    }

    let mut sorted_codes: Vec<String> = Vec::with_capacity(character_codes.len());
    for c in character_codes {
        let trimmed = c.as_ref().trim();
        if trimmed.is_empty() {
            return Err(PlannerError::validation_failed(
                "character_code must not be empty in plan_character_subphase",
            ));
        }
        sorted_codes.push(trimmed.to_string());
    }
    sorted_codes.sort();
    sorted_codes.dedup();

    let group_key = format!("p5_s{subphase}");
    let items = vec![(group_key, 1, sorted_codes.len() as i32)];
    let mut plan = plan_phase(&items, unit, max_parallel)?;
    for round in &mut plan.rounds {
        for task in &mut round.tasks {
            let start_idx = (task.lo - 1) as usize;
            let end_idx = task.hi as usize;
            let slice = &sorted_codes[start_idx..end_idx];
            task.task_identity = Some(format_char_profile_task_identity(subphase, slice)?);
            task.chapter_context_range = None;
        }
    }
    Ok(plan)
}

/// Plans all three sequential sub-phases (`5.1`, `5.2`, `5.3`) of Phase 5 (`core.md` §3.9).
pub fn plan_all_character_subphases<S: AsRef<str>>(
    character_codes: &[S],
    unit: i32,
    max_parallel: usize,
) -> Result<[PhasePlan; 3], PlannerError> {
    Ok([
        plan_character_subphase(1, character_codes, unit, max_parallel)?,
        plan_character_subphase(2, character_codes, unit, max_parallel)?,
        plan_character_subphase(3, character_codes, unit, max_parallel)?,
    ])
}

/// Plans Phase 6 relationship matrix rounds over directed character pairs `(from_code, to_code)`
/// (`core.md` §3.9, `be-core.md` §6.3.2).
pub fn plan_relationship_phase<S1: AsRef<str>, S2: AsRef<str>>(
    pairs: &[(S1, S2)],
    unit: i32,
    max_parallel: usize,
) -> Result<PhasePlan, PlannerError> {
    if unit <= 0 {
        return Err(PlannerError::validation_failed(
            "unit must be >= 1 for plan_relationship_phase",
        ));
    }
    if max_parallel == 0 {
        return Err(PlannerError::validation_failed(
            "max_parallel must be >= 1 for plan_relationship_phase",
        ));
    }
    if pairs.is_empty() {
        return Ok(PhasePlan {
            unit,
            max_parallel,
            rounds: Vec::new(),
        });
    }

    let mut sorted_pairs: Vec<(String, String)> = Vec::with_capacity(pairs.len());
    for (from, to) in pairs {
        let f = from.as_ref().trim();
        let t = to.as_ref().trim();
        if f.is_empty() || t.is_empty() {
            return Err(PlannerError::validation_failed(
                "relationship pair codes must not be empty in plan_relationship_phase",
            ));
        }
        sorted_pairs.push((f.to_string(), t.to_string()));
    }
    sorted_pairs.sort();
    sorted_pairs.dedup();

    let items = vec![("p6_rel".to_string(), 1, sorted_pairs.len() as i32)];
    let mut plan = plan_phase(&items, unit, max_parallel)?;
    for round in &mut plan.rounds {
        for task in &mut round.tasks {
            let start_idx = (task.lo - 1) as usize;
            let end_idx = task.hi as usize;
            let slice = &sorted_pairs[start_idx..end_idx];
            task.task_identity = Some(format_rel_matrix_task_identity(slice)?);
            task.chapter_context_range = None;
        }
    }
    Ok(plan)
}

/// Extracts ordered `(arc_code, min_chapter_pos, max_chapter_pos)` items from `BusinessFlowData`
/// for chapter-based workflow planning.
pub fn extract_arc_chapter_ranges(
    state: &BusinessFlowData,
) -> Result<Vec<(String, i32, i32)>, PlannerError> {
    let mut ranges = Vec::new();
    for ch_arc in &state.optional.chapter_phase.arcs {
        if ch_arc.chapters.is_empty() {
            continue;
        }
        let mut min_pos: Option<i32> = None;
        let mut max_pos: Option<i32> = None;
        for ch in &ch_arc.chapters {
            let pos = ch.position.value.ok_or_else(|| {
                PlannerError::validation_failed(format!(
                    "chapter '{}' in arc '{}' is missing position",
                    ch.code, ch_arc.code
                ))
            })? as i32;
            if pos < 1 {
                return Err(PlannerError::validation_failed(format!(
                    "chapter '{}' has non-positive position {pos}",
                    ch.code
                )));
            }
            min_pos = Some(min_pos.map_or(pos, |m| std::cmp::min(m, pos)));
            max_pos = Some(max_pos.map_or(pos, |m| std::cmp::max(m, pos)));
        }
        if let (Some(lo), Some(hi)) = (min_pos, max_pos) {
            ranges.push((ch_arc.code.clone(), lo, hi));
        }
    }
    Ok(ranges)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn outline_runs_146_to_260_matches_spec() {
        let runs = plan_outline_runs(146, 260, 50, 10).unwrap();
        let tuples: Vec<_> = runs.iter().map(OutlineWindow::as_tuple).collect();
        assert_eq!(
            tuples,
            vec![
                (None, (146, 195)),
                (Some((186, 195)), (196, 235)),
                (Some((226, 235)), (236, 260)),
            ]
        );
    }

    #[test]
    fn invalid_lookback_ge_cap_returns_validation_failed() {
        assert!(plan_outline_runs(1, 100, 50, 50).is_err());
        assert!(plan_outline_runs(1, 100, 50, 60).is_err());
        assert!(plan_outline_runs(1, 100, 0, 0).is_err());
    }
}
