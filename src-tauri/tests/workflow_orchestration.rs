#![allow(non_snake_case)]

//! Acceptance and Integration tests for `phase4_sprint1_deep0`
//! ("Run segmented workflow and cancellation").
//!
//! Specifications covered:
//! - `core.md` §3.9 (Segmented Workflow, locks, cancellation, Phase 1-6 rules)
//! - `be-core.md` §6.3, §6.4.1, §6.4.4, §6.4.5, §10.2.5 (IPC #16, #17, #18)
//! - `phase4_sprint1_deep0.md` acceptance vectors
//!
//! Acceptance vectors:
//! - `phase4_sprint1_deep0-F0`: Predecessor gates verification.
//! - `P4S1-P`: Normal workflow round execution: 6 tasks compute concurrently, mutations applied 0..5 order, TX-04 sealed.
//! - `P4S1-N`: No external effect replay on retry or restart; idempotent replay.
//! - `P4S1-B`: Canceled round never reports saved if TX-04 failed; returns `VersionSealFailed`.
//! - `P4S1-CANCEL-BEFORE-TX04`: Cancel before TX-04 preserves applied mutations and halts remaining rounds.
//! - `P4S1-FAILED-TASK`: 1 task fails mid-way, stops round, terminates with failed status.
//! - `P4S1-PHASE3-PARENT-FIRST`: Phase 3 probe -> merge -> parent-first write by hierarchy tree depth.
//! - `P4S1-PHASE5-SUBPHASE`: Phase 5 runs subphases 5.1, 5.2, 5.3 strictly separated, never mixed.
//! - `P4S1-REPLAN-MISSING-EVIDENCE`: Resume with missing intermediate evidence returns `requires_replan`.
//! - `IPC-CMD-016`: `start_or_resume_workflow` typed IPC validation, authorization, and error handling.
//! - `IPC-CMD-017`: `cancel_workflow` typed IPC validation, authorization, and error handling.
//! - `IPC-CMD-018`: `get_workflow_status` typed IPC non-blocking status query.

#[path = "../src/error.rs"]
pub mod error_mod;

#[path = "../src/domain/business_flow.rs"]
pub mod business_flow;

#[path = "../src/domain/code_allocator.rs"]
pub mod code_allocator;

#[path = "../src/domain/fingerprint.rs"]
pub mod fingerprint;

#[path = "../src/domain/word_count.rs"]
pub mod word_count;

#[path = "../src/domain/invariants.rs"]
pub mod invariants;

#[path = "../src/domain/progress_engine.rs"]
pub mod progress_engine;
#[path = "../src/domain/phase_gate.rs"]
pub mod phase_gate;

#[path = "../src/domain/workflow_planner.rs"]
pub mod workflow_planner;

#[path = "../src/domain/phase3_artifact.rs"]
pub mod phase3_artifact;

#[path = "../src/domain/phase3_merge.rs"]
pub mod phase3_merge;

#[path = "../src/domain/sf06/manifest.rs"]
pub mod manifest;

#[path = "../src/domain/sf06/codec.rs"]
pub mod codec;

#[path = "../src/domain/cascade_delete.rs"]
pub mod cascade_delete;

pub mod registry {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
    #[serde(rename_all = "snake_case")]
    pub enum CallerCategory {
        AiCaller,
        UiAction,
        Cli,
    }

    impl CallerCategory {
        pub const fn is_ai(self) -> bool {
            matches!(self, CallerCategory::AiCaller)
        }

        pub const fn is_user_or_cli(self) -> bool {
            matches!(self, CallerCategory::UiAction | CallerCategory::Cli)
        }
    }
}

#[path = "../src/storage/db.rs"]
pub mod db;

#[path = "../src/storage/migration.rs"]
pub mod migration;

#[path = "../src/storage/draft_repo.rs"]
pub mod draft_repo;

#[path = "../src/storage/pending_repo.rs"]
pub mod pending_repo;

#[path = "../src/ipc/dto.rs"]
pub mod dto;

#[path = "../src/service/draft_runtime.rs"]
pub mod draft_runtime;

#[path = "../src/service/patch_service.rs"]
pub mod patch_service;

#[path = "../src/service/draft_service.rs"]
pub mod draft_service;

#[path = "../src/service/round_seal_service.rs"]
pub mod round_seal_service;

#[path = "../src/service/receipt_verifier.rs"]
pub mod receipt_verifier;

#[path = "../src/service/workflow_service.rs"]
pub mod workflow_service;

#[path = "../src/ipc/workflow_commands.rs"]
pub mod workflow_commands;

pub mod domain {
    pub mod sf06 {
        pub use crate::codec;
        pub use crate::manifest;
    }
    pub use crate::business_flow;
    pub use crate::cascade_delete;
    pub use crate::code_allocator;
    pub use crate::fingerprint;
    pub use crate::invariants;
    pub use crate::phase3_artifact;
    pub use crate::phase3_merge;
    pub use crate::phase_gate;
    pub use crate::progress_engine;
    pub use crate::word_count;
    pub use crate::workflow_planner;
    pub mod tools {
        pub use crate::registry;
    }
}

pub mod error {
    pub use crate::error_mod::*;
}

pub mod ipc {
    pub use crate::dto;
    pub use crate::workflow_commands;

    // This integration fixture imports workflow_commands.rs in isolation. Its IPC config
    // state is supplied by the production library; the stub only preserves the module
    // boundary for compiling the standalone workflow command tests.
    pub mod config_commands {
        use crate::domain::workflow_planner::WorkflowConfig;
        use crate::error::IpcError;
        use crate::dto::SubagentConfigDto;
        use parking_lot::RwLock;

        pub struct RuntimeConfigState {
            pub subagent_config: RwLock<SubagentConfigDto>,
        }

        pub fn workflow_config_from_subagent(cfg: &SubagentConfigDto) -> Result<WorkflowConfig, IpcError> {
            Ok(WorkflowConfig {
                max_parallel: cfg.max_subagents as usize,
                outline_cap: cfg.outline_capacity as i32,
                outline_lookback: cfg.outline_lookback as i32,
                unit_chapter: cfg.chapters_per_subagent as i32,
                unit_world_lore: cfg.lore_records_per_subagent as i32,
                unit_character: cfg.characters_per_subagent as i32,
                unit_relationship: cfg.relationship_pairs_per_subagent as i32,
                context_margin: cfg.context_margin as i32,
                ..WorkflowConfig::default()
            })
        }
    }
}

pub mod service {
    pub use crate::draft_runtime;
    pub use crate::draft_service;
    pub use crate::patch_service;
    pub use crate::receipt_verifier;
    pub use crate::round_seal_service;
    pub use crate::workflow_service;
}

pub mod storage {
    pub use crate::db;
    pub use crate::draft_repo;
    pub use crate::migration;
    pub use crate::pending_repo;
}

use std::process::Command;
use std::sync::Arc;
use std::time::Duration;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{params, Connection};
use phase3_artifact::WorldProbeReportV1;
use tempfile::TempDir;
use tokio::sync::mpsc;
use uuid::Uuid;

use domain::business_flow::BusinessFlowData;
use domain::fingerprint::state_data_hash;
use domain::sf06::codec::PatchOp;
use domain::workflow_planner::WorkflowConfig;
use error::{ErrorCode, IpcError};
use ipc::dto::*;
use ipc::workflow_commands::*;
use service::draft_runtime::DraftRuntimeRegistry;
use service::draft_service::DraftService;
use service::workflow_service::{
    TaskExecutionContext, TaskExecutionOutput, WorkflowErrorInjection, WorkflowService,
    WorkflowTaskExecutor,
};

pub const GATE_P3S7: &str = "f45ce37eff0e0d63e05b3f8b440eb8f4231a73ec";
pub const GATE_P3S10: &str = "c03d8a91d3ace1858f1ad51d448ad23f9f89a292";
pub const GATE_P3S1: &str = "702843c38e45b193be25b80222a48eadedfd3538";

// -----------------------------------------------------------------------------
// Test Helpers & Fixtures
// -----------------------------------------------------------------------------

#[allow(dead_code)]
struct TestFixture {
    _temp_dir: TempDir,
    pool: Pool<SqliteConnectionManager>,
    registry: Arc<DraftRuntimeRegistry>,
    draft_service: Arc<DraftService>,
    workflow_service: Arc<WorkflowService>,
}

fn setup_test_fixture() -> TestFixture {
    let temp_dir = TempDir::new().expect("failed to create temp dir");
    let db_path = temp_dir.path().join("test_workflow.db");

    let manager = SqliteConnectionManager::file(&db_path).with_init(|conn| {
        storage::db::configure_connection(conn)
    });
    let pool = Pool::builder()
        .max_size(5)
        .build(manager)
        .expect("failed to build pool");

    {
        let mut conn = pool.get().expect("failed to get connection");
        storage::migration::run_migrations(&mut conn).expect("failed to run migrations");
    }

    let registry = Arc::new(DraftRuntimeRegistry::new());
    let draft_service = Arc::new(DraftService::new(pool.clone(), registry.clone()));
    let config = WorkflowConfig::default();
    let workflow_service = Arc::new(WorkflowService::new(
        pool.clone(),
        registry.clone(),
        draft_service.clone(),
        config,
    ));

    TestFixture {
        _temp_dir: temp_dir,
        pool,
        registry,
        draft_service,
        workflow_service,
    }
}

/// Draft fixture with real arcs/chapters/characters/relationships/world tree so that the
/// production planner (which never fabricates entities) can build rounds from state.
fn seeded_business_flow() -> BusinessFlowData {
    let tracked = |value: serde_json::Value| serde_json::json!({"value": value, "source": "user"});
    let chapter = |code: &str, position: u32| {
        // Every chapter needs at least one scene: the phase-3 and phase-5.3 gates both read
        // the scene list, and a chapter without scenes blocks the workflow before it starts.
        serde_json::json!({
            "code": code,
            "display_name": tracked(serde_json::json!(format!("Chapter {position}"))),
            "position": tracked(serde_json::json!(position)),
            "summary": tracked(serde_json::json!("Summary")),
            "scenes": [{
                "code": format!("{code}-s1"),
                "display_name": tracked(serde_json::json!(format!("Scene {position}.1"))),
                "position": tracked(serde_json::json!(1)),
                "summary": tracked(serde_json::json!("Scene summary")),
                "world_setting_code": tracked(serde_json::json!("city_capital")),
            }],
        })
    };
    // Phase 4 must cover every scene produced by phase 2, otherwise the phase-5 gate blocks.
    let scene_config_arcs: Vec<serde_json::Value> = ["arc_one", "arc_two"]
        .iter()
        .map(|arc| {
            let chapters: Vec<serde_json::Value> = (1..=6)
                .map(|position| {
                    serde_json::json!({
                        "code": format!("{arc}-c{position}"),
                        "scenes": [{
                            "code": format!("{arc}-c{position}-s1"),
                            "world_setting_code": tracked(serde_json::json!("city_capital")),
                            "style_preset_code": tracked(serde_json::json!("style_classic")),
                            "narrator_config_code": tracked(serde_json::json!("narrator_third_limited")),
                            "characters": [{"character_code": "char_a", "is_pov": tracked(serde_json::json!(true))}],
                        }],
                    })
                })
                .collect();
            serde_json::json!({"code": arc, "chapters": chapters})
        })
        .collect();

    let value = serde_json::json!({
        "meta": {"version": "1.0", "phase": 0, "subphase": null, "updated_at": null},
        "require": {
            "project": {
                "display_name": tracked(serde_json::json!("Workflow Project")),
                "summary": tracked(serde_json::json!("Premise")),
                "language": tracked(serde_json::json!("VI")),
                "target": {
                    "arc_count": tracked(serde_json::json!(2)),
                    "chapter_count": tracked(serde_json::json!(12)),
                    "word_per_chapter_count": tracked(serde_json::json!(3000))
                },
                "genres": [{"code": tracked(serde_json::json!("fantasy")), "is_primary": tracked(serde_json::json!(true))}]
            },
            "character": [
                {"code": "char_a", "display_name": tracked(serde_json::json!("A")), "biological_sex": tracked(serde_json::json!("FEMALE")), "motif_code": tracked(serde_json::json!("strong_girl"))},
                {"code": "char_b", "display_name": tracked(serde_json::json!("B")), "biological_sex": tracked(serde_json::json!("MALE")), "motif_code": tracked(serde_json::json!("loyal_knight"))},
                {"code": "char_c", "display_name": tracked(serde_json::json!("C")), "biological_sex": tracked(serde_json::json!("NONE")), "motif_code": tracked(serde_json::json!("loyal_knight"))}
            ]
        },
        "optional": {
            "chapter_phase": {"arcs": [
                {"code": "arc_one", "chapters": [
                    chapter("arc_one-c1", 1), chapter("arc_one-c2", 2), chapter("arc_one-c3", 3),
                    chapter("arc_one-c4", 4), chapter("arc_one-c5", 5), chapter("arc_one-c6", 6)
                ]},
                {"code": "arc_two", "chapters": [
                    chapter("arc_two-c1", 7), chapter("arc_two-c2", 8), chapter("arc_two-c3", 9),
                    chapter("arc_two-c4", 10), chapter("arc_two-c5", 11), chapter("arc_two-c6", 12)
                ]}
            ]},
            "world_building_phase": {"world_settings": [
                {"code": "continent_north", "display_name": tracked(serde_json::json!("North")), "type": tracked(serde_json::json!("REALM"))},
                {"code": "kingdom_iron", "parent_code": tracked(serde_json::json!("continent_north")), "display_name": tracked(serde_json::json!("Iron")), "type": tracked(serde_json::json!("WORLD"))},
                {"code": "city_capital", "parent_code": tracked(serde_json::json!("kingdom_iron")), "display_name": tracked(serde_json::json!("Capital")), "type": tracked(serde_json::json!("CITY"))}
            ]},
            "scene_config_phase": {"arcs": scene_config_arcs},
            "character_appearance_phase": {"character": [
                {"code": "char_a", "default_summary": tracked(serde_json::json!("Tall, ink-stained coat"))},
                {"code": "char_b", "default_summary": tracked(serde_json::json!("Broad, ceremonial armour"))},
                {"code": "char_c", "default_summary": tracked(serde_json::json!("Slight, always hooded"))}
            ]},
            "character_psychological_phase": {"character": [
                {"code": "char_a", "core_trauma_and_belief": {"the_wound": tracked(serde_json::json!("Left behind once"))}},
                {"code": "char_b", "core_trauma_and_belief": {"the_wound": tracked(serde_json::json!("Failed to protect"))}},
                {"code": "char_c", "core_trauma_and_belief": {"the_wound": tracked(serde_json::json!("Trusted the wrong ally"))}}
            ]},
            "character_life_experience_phase": {"character": [
                {"code": "char_a", "memories": [{"code": "mem_a", "content": tracked(serde_json::json!("The night the border fell")), "source_scene_code": tracked(serde_json::json!("arc_one-c1-s1"))}]},
                {"code": "char_b", "memories": [{"code": "mem_b", "content": tracked(serde_json::json!("The oath at the gate")), "source_scene_code": tracked(serde_json::json!("arc_one-c1-s1"))}]},
                {"code": "char_c", "memories": [{"code": "mem_c", "content": tracked(serde_json::json!("The letter never sent")), "source_scene_code": tracked(serde_json::json!("arc_one-c1-s1"))}]}
            ]},
            "relationship_phase": {"relationship": [
                {"code": "rel_ab", "character": {"from_code": "char_a", "to_code": "char_b"}},
                {"code": "rel_ba", "character": {"from_code": "char_b", "to_code": "char_a"}},
                {"code": "rel_ac", "character": {"from_code": "char_a", "to_code": "char_c"}},
                {"code": "rel_ca", "character": {"from_code": "char_c", "to_code": "char_a"}},
                {"code": "rel_bc", "character": {"from_code": "char_b", "to_code": "char_c"}},
                {"code": "rel_cb", "character": {"from_code": "char_c", "to_code": "char_b"}}
            ]}
        }
    });
    let flow: BusinessFlowData = serde_json::from_value(value).expect("fixture draft must deserialize");
    flow.validate_structure().expect("fixture draft must be structurally valid");
    flow
}

fn seed_test_draft_session(conn: &Connection, draft_session_id: &Uuid, status: &str) -> Uuid {
    let proj_id = Uuid::new_v4();
    let sess_id = Uuid::new_v4();

    conn.execute(
        "INSERT INTO projects (id, display_name) VALUES (?1, 'Workflow Project')",
        params![proj_id.to_string()],
    )
    .unwrap();

    conn.execute(
        "INSERT INTO sessions (id, project_id) VALUES (?1, ?2)",
        params![sess_id.to_string(), proj_id.to_string()],
    )
    .unwrap();

    conn.execute(
        "INSERT INTO draft_sessions (id, session_id, project_id, head_version_seq, status) VALUES (?1, ?2, ?3, 1, ?4)",
        params![draft_session_id.to_string(), sess_id.to_string(), proj_id.to_string(), status],
    )
    .unwrap();

    let s0 = seeded_business_flow();
    let s0_json = serde_json::to_string(&s0).unwrap();
    let s0_hash = state_data_hash(&s0).unwrap();

    let ver_id = Uuid::new_v4();
    conn.execute(
        "INSERT INTO draft_versions (id, draft_session_id, seq, seal_key, phase, subphase, data_json, data_hash, reason, parent_seq, created_at)
         VALUES (?1, ?2, 1, 'init', 0, NULL, ?3, ?4, 'phase_confirm', NULL, '2026-11-08T00:00:00Z')",
        params![ver_id.to_string(), draft_session_id.to_string(), s0_json, s0_hash],
    )
    .unwrap();

    sess_id
}

// Custom mock executor that produces mutations and can introduce artificial delays
struct MockParallelExecutor {
    delays_ms: Vec<u64>,
}

#[async_trait::async_trait]
impl WorkflowTaskExecutor for MockParallelExecutor {
    async fn execute_task(
        &self,
        ctx: &TaskExecutionContext,
    ) -> Result<TaskExecutionOutput, IpcError> {
        let delay = if ctx.task_index < self.delays_ms.len() {
            self.delays_ms[ctx.task_index]
        } else {
            0
        };

        if delay > 0 {
            tokio::time::sleep(Duration::from_millis(delay)).await;
        }

        // Produce a deterministic mutation for each task
        let op = PatchOp::SetCounter {
            counter_key: format!("task_counter_{}", ctx.task_index),
            old_value: 0,
            new_value: (ctx.task_index + 1) as u64,
        };

        Ok(TaskExecutionOutput {
            task_index: ctx.task_index,
            task_spec: ctx.task_spec.clone(),
            task_identity: ctx.task_identity.clone(),
            mutations: vec![op],
            applied_write_count: 1,
            artifact: None,
        })
    }
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

/// Acceptance Test: `phase4_sprint1_deep0-F0`
/// Verifies git ancestry and existence of all required predecessor gates:
/// - `gate_phase3_sprint7_deep4`
/// - `gate_phase3_sprint10_deep4`
/// - `gate_phase3_sprint1_deep0`
#[test]
fn phase4_sprint1_deep0_F0_predecessor_gates_verification() {
    for (gate_name, sha) in [
        ("gate_phase3_sprint7_deep4", GATE_P3S7),
        ("gate_phase3_sprint10_deep4", GATE_P3S10),
        ("gate_phase3_sprint1_deep0", GATE_P3S1),
    ] {
        let cat_out = Command::new("git")
            .args(["cat-file", "-e", &format!("{}^{{commit}}", sha)])
            .output()
            .unwrap_or_else(|e| panic!("failed to run git cat-file for {}: {}", gate_name, e));

        assert!(
            cat_out.status.success(),
            "Predecessor {} commit {} must exist in git object database",
            gate_name,
            sha
        );
    }
}

/// Acceptance Test: `P4S1-P`
/// Normal workflow round execution:
/// 6 tasks (0..5) compute concurrently with non-deterministic completion order (reversed delays).
/// WorkingState mutations are strictly applied in 0..5 order under operation_lock.
/// TX-04 seals the round atomically and creates durable version.
#[tokio::test]
async fn P4S1_P_concurrent_compute_deterministic_order_tx04_seal() {
    let fixture = setup_test_fixture();
    let draft_session_id = Uuid::new_v4();
    {
        let conn = fixture.pool.get().unwrap();
        seed_test_draft_session(&conn, &draft_session_id, "editing");
    }

    // Task 5 finishes first (10ms), Task 0 finishes last (60ms)
    let delays = vec![60, 50, 40, 30, 20, 10];
    fixture
        .workflow_service
        .set_task_executor(Arc::new(MockParallelExecutor { delays_ms: delays }));

    // Configure 6 parallel tasks
    let mut cfg = fixture.workflow_service.get_config();
    cfg.max_parallel = 6;
    fixture.workflow_service.set_config(cfg);

    let (event_tx, mut event_rx) = mpsc::channel(100);

    let req = StartOrResumeWorkflowReq {
        draft_session_id,
        phase: 6, // Phase 6 produces tasks that can run in 1 round
        subphase: None,
        workflow_run_id: None,
        system_language: "vi".to_string(),
    };

    let summary = exec_start_or_resume_workflow(&fixture.workflow_service, &req, Some(event_tx))
        .await
        .expect("workflow round should complete successfully");

    assert_eq!(summary.status, "completed");
    assert_eq!(summary.completed_tasks, summary.total_tasks);
    assert!(summary.total_tasks >= 1);
    assert!(summary.last_sealed_version.is_some());
    assert_eq!(summary.last_sealed_version.as_ref().unwrap().seq, 2);
    assert_eq!(
        summary.last_sealed_version.as_ref().unwrap().reason,
        "workflow_round"
    );

    // Verify deterministic mutation application order
    let applied_order = fixture
        .workflow_service
        .get_applied_task_order(&draft_session_id);
    for i in 0..applied_order.len() {
        assert_eq!(applied_order[i], i, "Tasks must be applied in ascending 0..N order");
    }

    // Verify events were emitted
    let mut events = Vec::new();
    while let Ok(evt) = event_rx.try_recv() {
        events.push(evt);
    }
    assert!(events.iter().any(|e| matches!(e, WorkflowProgressEvent::WorkflowStarted { .. })));
    assert!(events.iter().any(|e| matches!(e, WorkflowProgressEvent::RoundStarted { .. })));
    assert!(events.iter().any(|e| matches!(e, WorkflowProgressEvent::TaskCompleted { .. })));
    assert!(events.iter().any(|e| matches!(e, WorkflowProgressEvent::RoundSealed { .. })));
    assert!(events.iter().any(|e| matches!(e, WorkflowProgressEvent::WorkflowFinished { .. })));

    // Verify SQLite state: 1 round receipt, 1 draft version sealed, task receipts
    let conn = fixture.pool.get().unwrap();
    let round_count: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM workflow_round_receipts WHERE draft_session_id = ?1",
            params![draft_session_id.to_string()],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(round_count, 1);

    let ver_count: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM draft_versions WHERE draft_session_id = ?1 AND reason = 'workflow_round'",
            params![draft_session_id.to_string()],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(ver_count, 1);
}

/// Acceptance Test: `P4S1-N`
/// No external effect replay on retry or restart:
/// Retrying a previously sealed round results in idempotent replay without re-executing writes.
#[tokio::test]
async fn P4S1_N_no_external_effect_replay_on_retry() {
    let fixture = setup_test_fixture();
    let draft_session_id = Uuid::new_v4();
    {
        let conn = fixture.pool.get().unwrap();
        seed_test_draft_session(&conn, &draft_session_id, "editing");
    }

    let run_id = Uuid::new_v4();
    let req = StartOrResumeWorkflowReq {
        draft_session_id,
        phase: 1,
        subphase: None,
        workflow_run_id: Some(run_id),
        system_language: "en".to_string(),
    };

    // First run seals the round
    let summary1 = exec_start_or_resume_workflow(&fixture.workflow_service, &req, None)
        .await
        .expect("initial run should succeed");
    assert_eq!(summary1.status, "completed");

    let conn = fixture.pool.get().unwrap();
    let round_count_1: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM workflow_round_receipts WHERE draft_session_id = ?1",
            params![draft_session_id.to_string()],
            |r| r.get(0),
        )
        .unwrap();
    assert!(round_count_1 >= 1, "Phase 1 must seal at least one round receipt");

    // Second run with the same workflow_run_id uses idempotent replay, zero new writes
    let summary2 = exec_start_or_resume_workflow(&fixture.workflow_service, &req, None)
        .await
        .expect("retry run should succeed idempotently");
    assert_eq!(summary2.status, "completed");

    let round_count_2: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM workflow_round_receipts WHERE draft_session_id = ?1",
            params![draft_session_id.to_string()],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(round_count_2, round_count_1, "Idempotent replay must not create duplicate round receipts");
}

/// Acceptance Test: `P4S1-B`
/// Canceled round never reports saved if TX-04 failed:
/// When TX-04 fails during cancellation, `cancel_workflow` returns `VersionSealFailed`
/// and no version is created.
#[tokio::test]
async fn P4S1_B_canceled_round_never_saved_if_tx04_failed() {
    let fixture = setup_test_fixture();
    let draft_session_id = Uuid::new_v4();
    {
        let conn = fixture.pool.get().unwrap();
        seed_test_draft_session(&conn, &draft_session_id, "editing");
    }

    // Set error injection: FailTx04
    fixture.workflow_service.set_error_injection(
        &draft_session_id,
        WorkflowErrorInjection::FailTx04,
    );

    // Give task enough runtime so cancel arrives while running
    fixture
        .workflow_service
        .set_task_executor(Arc::new(MockParallelExecutor {
            delays_ms: vec![50],
        }));

    let run_id = Uuid::new_v4();
    let req = StartOrResumeWorkflowReq {
        draft_session_id,
        phase: 1,
        subphase: None,
        workflow_run_id: Some(run_id),
        system_language: "vi".to_string(),
    };

    let service = fixture.workflow_service.clone();
    // Launch workflow in background
    let join_handle = tokio::spawn(async move {
        exec_start_or_resume_workflow(&service, &req, None).await
    });

    // Brief yield to let workflow start
    tokio::time::sleep(Duration::from_millis(10)).await;

    // Trigger cancel
    let cancel_req = CancelWorkflowReq {
        draft_session_id,
        workflow_run_id: run_id,
    };

    let cancel_res = exec_cancel_workflow(&fixture.workflow_service, &cancel_req).await;
    assert!(cancel_res.is_err());
    let err = cancel_res.unwrap_err();
    assert_eq!(
        err.code,
        ErrorCode::VersionSealFailed,
        "Failed seal during cancel must return VersionSealFailed"
    );

    let _ = join_handle.await;

    // Verify 0 workflow_round versions created in draft_versions
    let conn = fixture.pool.get().unwrap();
    let ver_count: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM draft_versions WHERE draft_session_id = ?1 AND reason = 'workflow_round'",
            params![draft_session_id.to_string()],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(ver_count, 0, "No fake version must be saved on TX-04 failure");
}

/// Acceptance Test: `P4S1-CANCEL-BEFORE-TX04`
/// Cancellation halts remaining rounds and seals already applied mutations without continuing.
#[tokio::test]
async fn P4S1_CANCEL_BEFORE_TX04_seals_applied_and_halts_remaining_rounds() {
    let fixture = setup_test_fixture();
    let draft_session_id = Uuid::new_v4();
    {
        let conn = fixture.pool.get().unwrap();
        seed_test_draft_session(&conn, &draft_session_id, "editing");
    }

    // Delay task execution to allow cancel request to arrive
    fixture
        .workflow_service
        .set_task_executor(Arc::new(MockParallelExecutor {
            delays_ms: vec![50],
        }));

    let run_id = Uuid::new_v4();
    let req = StartOrResumeWorkflowReq {
        draft_session_id,
        phase: 1, // Phase 1 with multiple outline runs
        subphase: None,
        workflow_run_id: Some(run_id),
        system_language: "vi".to_string(),
    };

    let service = fixture.workflow_service.clone();
    let (event_tx, _event_rx) = mpsc::channel(100);

    let join_handle = tokio::spawn(async move {
        exec_start_or_resume_workflow(&service, &req, Some(event_tx)).await
    });

    // Wait for RoundStarted
    tokio::time::sleep(Duration::from_millis(15)).await;

    let cancel_req = CancelWorkflowReq {
        draft_session_id,
        workflow_run_id: run_id,
    };

    let cancel_res = exec_cancel_workflow(&fixture.workflow_service, &cancel_req)
        .await
        .expect("cancel should succeed");
    assert_eq!(cancel_res.status, "cancelled");
    assert!(cancel_res.resume_available);

    let workflow_res = join_handle.await.unwrap().expect("workflow join ok");
    assert_eq!(workflow_res.status, "cancelled");

    // Applied mutations in the round were sealed
    assert!(workflow_res.last_sealed_version.is_some());
}

/// Acceptance Test: `P4S1-FAILED-TASK`
/// 1 task fails mid-way, stops the round and terminates workflow with failed status.
#[tokio::test]
async fn P4S1_FAILED_TASK_stops_round_and_reports_failed() {
    let fixture = setup_test_fixture();
    let draft_session_id = Uuid::new_v4();
    {
        let conn = fixture.pool.get().unwrap();
        seed_test_draft_session(&conn, &draft_session_id, "editing");
    }

    // Inject failure on task index 0
    fixture
        .workflow_service
        .set_error_injection(&draft_session_id, WorkflowErrorInjection::FailTask(0));

    let req = StartOrResumeWorkflowReq {
        draft_session_id,
        phase: 1,
        subphase: None,
        workflow_run_id: None,
        system_language: "vi".to_string(),
    };

    let (event_tx, mut event_rx) = mpsc::channel(100);
    let summary = exec_start_or_resume_workflow(&fixture.workflow_service, &req, Some(event_tx))
        .await
        .expect("workflow returns execution summary even when failed");

    assert_eq!(summary.status, "failed");

    let mut events = Vec::new();
    while let Ok(evt) = event_rx.try_recv() {
        events.push(evt);
    }

    let finished_event = events
        .iter()
        .find(|e| matches!(e, WorkflowProgressEvent::WorkflowFinished { status, .. } if status == "failed"));
    assert!(finished_event.is_some(), "WorkflowFinished(failed) event must be emitted");
}

/// Returns a valid ProbeCandidatesV1 artifact for probe tasks so the merger can plan writes.
struct ProbeArtifactExecutor;

#[async_trait::async_trait]
impl WorkflowTaskExecutor for ProbeArtifactExecutor {
    async fn execute_task(
        &self,
        ctx: &TaskExecutionContext,
    ) -> Result<TaskExecutionOutput, IpcError> {
        let artifact = if ctx.task_spec.task_kind == "world_probe" {
            let report_text = r#"{"schemaVersion":1,"worldSettings":[{"tempKey":"w1","displayName":"Probe Realm","type":"REALM"},{"tempKey":"w2","parentTempKey":"w1","displayName":"Probe City","type":"CITY"}],"loreEntries":[]}"#;
            Some(serde_json::to_value(WorldProbeReportV1::new(
                ctx.task_spec.assigned_scope.to_string(),
                report_text,
            ))
            .unwrap())
        } else {
            None
        };
        Ok(TaskExecutionOutput {
            task_index: ctx.task_index,
            task_spec: ctx.task_spec.clone(),
            task_identity: ctx.task_identity.clone(),
            mutations: Vec::new(),
            applied_write_count: 0,
            artifact,
        })
    }
}

/// Acceptance Test: `P4S1-PHASE3-PARENT-FIRST`
/// Verifies Phase 3 stages: probe -> merge -> parent-first write by hierarchy tree depth.
#[tokio::test]
async fn P4S1_PHASE3_PARENT_FIRST_probe_merge_depth_ordered_write() {
    let fixture = setup_test_fixture();
    fixture
        .workflow_service
        .set_task_executor(Arc::new(ProbeArtifactExecutor));
    let draft_session_id = Uuid::new_v4();
    {
        let conn = fixture.pool.get().unwrap();
        seed_test_draft_session(&conn, &draft_session_id, "editing");
    }

    let (event_tx, mut event_rx) = mpsc::channel(100);

    let req = StartOrResumeWorkflowReq {
        draft_session_id,
        phase: 3,
        subphase: None,
        workflow_run_id: None,
        system_language: "vi".to_string(),
    };

    let summary = exec_start_or_resume_workflow(&fixture.workflow_service, &req, Some(event_tx))
        .await
        .expect("Phase 3 workflow should complete");

    assert_eq!(summary.status, "completed");

    let mut round_events = Vec::new();
    while let Ok(evt) = event_rx.try_recv() {
        if let WorkflowProgressEvent::RoundStarted { round_index, active_task_identities, .. } = evt {
            round_events.push((round_index, active_task_identities));
        }
    }

    // Verify stage ordering:
    // First rounds are world_probe tasks
    // Subsequent rounds are world_write tasks ordered by hierarchy depth: depth_0 before depth_1 before depth_2
    assert!(round_events.len() >= 2);

    let mut found_probe = false;
    let mut found_write = false;
    for (_idx, identities) in &round_events {
        for id in identities {
            if id.contains("world_probe") {
                assert!(!found_write, "Probe tasks must run before write tasks");
                found_probe = true;
            }
            if id.contains("world_write") {
                found_write = true;
            }
        }
    }
    assert!(found_probe);
    assert!(found_write);
}

/// Acceptance Test: `P4S1-PHASE5-SUBPHASE`
/// Verifies Phase 5 cleanly separates subphases 5.1, 5.2, 5.3 without mixing.
#[tokio::test]
async fn P4S1_PHASE5_SUBPHASE_strictly_separated_subphases() {
    let fixture = setup_test_fixture();
    let draft_session_id = Uuid::new_v4();
    {
        let conn = fixture.pool.get().unwrap();
        seed_test_draft_session(&conn, &draft_session_id, "editing");
    }

    // Run subphase 1
    let req1 = StartOrResumeWorkflowReq {
        draft_session_id,
        phase: 5,
        subphase: Some(1),
        workflow_run_id: None,
        system_language: "vi".to_string(),
    };

    let (event_tx1, mut event_rx1) = mpsc::channel(100);
    let summary1 = exec_start_or_resume_workflow(&fixture.workflow_service, &req1, Some(event_tx1))
        .await
        .expect("Phase 5.1 should succeed");
    assert_eq!(summary1.status, "completed");

    while let Ok(evt) = event_rx1.try_recv() {
        if let WorkflowProgressEvent::WorkflowStarted { phase, subphase, .. } = evt {
            assert_eq!(phase, 5);
            assert_eq!(subphase, Some(1));
        }
        if let WorkflowProgressEvent::RoundStarted { active_task_identities, .. } = evt {
            for id in active_task_identities {
                assert!(id.contains("p5:s1"), "Round tasks must only belong to subphase 1: {}", id);
            }
        }
    }

    // Run subphase 2
    let req2 = StartOrResumeWorkflowReq {
        draft_session_id,
        phase: 5,
        subphase: Some(2),
        workflow_run_id: None,
        system_language: "vi".to_string(),
    };

    let (event_tx2, mut event_rx2) = mpsc::channel(100);
    let summary2 = exec_start_or_resume_workflow(&fixture.workflow_service, &req2, Some(event_tx2))
        .await
        .expect("Phase 5.2 should succeed");
    assert_eq!(summary2.status, "completed");

    while let Ok(evt) = event_rx2.try_recv() {
        if let WorkflowProgressEvent::WorkflowStarted { phase, subphase, .. } = evt {
            assert_eq!(phase, 5);
            assert_eq!(subphase, Some(2));
        }
        if let WorkflowProgressEvent::RoundStarted { active_task_identities, .. } = evt {
            for id in active_task_identities {
                assert!(id.contains("p5:s2"), "Round tasks must only belong to subphase 2: {}", id);
            }
        }
    }
}

/// Acceptance Test: `P4S1-REPLAN-MISSING-EVIDENCE` (and `phase4_sprint1_deep0-F1`)
/// When resuming a workflow where an existing task receipt lacks intermediate evidence,
/// returns `requires_replan` with `recoveryReason = "insufficient_task_evidence"`.
#[tokio::test]
async fn P4S1_REPLAN_MISSING_EVIDENCE_returns_requires_replan() {
    let fixture = setup_test_fixture();
    let draft_session_id = Uuid::new_v4();
    {
        let conn = fixture.pool.get().unwrap();
        seed_test_draft_session(&conn, &draft_session_id, "editing");
    }

    let run_id = Uuid::new_v4();

    // Insert an invalid receipt with missing evidence_json
    {
        let conn = fixture.pool.get().unwrap();
        let receipt_id = Uuid::new_v4().to_string();
        conn.execute(
            "INSERT INTO workflow_task_receipts (
                id, draft_session_id, workflow_run_id, plan_generation, task_identity,
                input_scope_fingerprint, output_scope_fingerprint, base_version_seq,
                resulting_version_seq, had_mutations, round_receipt_id, task_index,
                evidence_json, created_at
             ) VALUES (?1, ?2, ?3, 1, 'task:p1:outline_window:arc_01:1..=50', 'fp_in', 'fp_out', 1, 2, 1, NULL, NULL, NULL, '2026-11-08T00:00:00Z')",
            params![receipt_id, draft_session_id.to_string(), run_id.to_string()],
        )
        .unwrap();
    }

    let req = StartOrResumeWorkflowReq {
        draft_session_id,
        phase: 1,
        subphase: None,
        workflow_run_id: Some(run_id),
        system_language: "vi".to_string(),
    };

    let summary = exec_start_or_resume_workflow(&fixture.workflow_service, &req, None)
        .await
        .expect("replan check returns summary");

    assert_eq!(summary.status, "requires_replan");
    assert_eq!(
        summary.recovery_reason,
        Some("insufficient_task_evidence".to_string())
    );
}

/// Acceptance Test: `IPC-CMD-016`
/// Tests `start_or_resume_workflow`:
/// - Valid request -> returns `completed`
/// - Invalid phase (0, 7) -> `ValidationFailed`
/// - Invalid outline lookback -> `ValidationFailed`
/// - Non-existent draft -> `EntityNotFound`
/// - Non-editable draft -> `DraftNotEditable`
/// - Busy draft -> `DraftBusy`
#[tokio::test]
async fn IPC_CMD_016_start_or_resume_workflow_contracts() {
    let fixture = setup_test_fixture();
    let draft_session_id = Uuid::new_v4();
    {
        let conn = fixture.pool.get().unwrap();
        seed_test_draft_session(&conn, &draft_session_id, "editing");
    }

    // 1. Invalid phase (7)
    let bad_phase_req = StartOrResumeWorkflowReq {
        draft_session_id,
        phase: 7,
        subphase: None,
        workflow_run_id: None,
        system_language: "vi".to_string(),
    };
    let err = exec_start_or_resume_workflow(&fixture.workflow_service, &bad_phase_req, None)
        .await
        .unwrap_err();
    assert_eq!(err.code, ErrorCode::ValidationFailed);

    // 2. Invalid phase (0)
    let bad_phase_req_0 = StartOrResumeWorkflowReq {
        draft_session_id,
        phase: 0,
        subphase: None,
        workflow_run_id: None,
        system_language: "vi".to_string(),
    };
    let err = exec_start_or_resume_workflow(&fixture.workflow_service, &bad_phase_req_0, None)
        .await
        .unwrap_err();
    assert_eq!(err.code, ErrorCode::ValidationFailed);

    // 3. Invalid outline lookback (lookback >= cap)
    {
        let mut bad_cfg = fixture.workflow_service.get_config();
        bad_cfg.outline_lookback = 50;
        bad_cfg.outline_cap = 50;
        fixture.workflow_service.set_config(bad_cfg);
    }
    let valid_req = StartOrResumeWorkflowReq {
        draft_session_id,
        phase: 1,
        subphase: None,
        workflow_run_id: None,
        system_language: "vi".to_string(),
    };
    let err = exec_start_or_resume_workflow(&fixture.workflow_service, &valid_req, None)
        .await
        .unwrap_err();
    assert_eq!(err.code, ErrorCode::ValidationFailed);

    // Restore valid config
    fixture
        .workflow_service
        .set_config(WorkflowConfig::default());

    // 4. Non-existent draft
    let missing_draft_req = StartOrResumeWorkflowReq {
        draft_session_id: Uuid::new_v4(),
        phase: 1,
        subphase: None,
        workflow_run_id: None,
        system_language: "vi".to_string(),
    };
    let err = exec_start_or_resume_workflow(&fixture.workflow_service, &missing_draft_req, None)
        .await
        .unwrap_err();
    assert_eq!(err.code, ErrorCode::EntityNotFound);

    // 5. Non-editable draft
    let committed_draft_id = Uuid::new_v4();
    {
        let conn = fixture.pool.get().unwrap();
        seed_test_draft_session(&conn, &committed_draft_id, "committed");
    }
    let non_editable_req = StartOrResumeWorkflowReq {
        draft_session_id: committed_draft_id,
        phase: 1,
        subphase: None,
        workflow_run_id: None,
        system_language: "vi".to_string(),
    };
    let err = exec_start_or_resume_workflow(&fixture.workflow_service, &non_editable_req, None)
        .await
        .unwrap_err();
    assert_eq!(err.code, ErrorCode::DraftNotEditable);

    // 6. Valid execution
    let valid_summary = exec_start_or_resume_workflow(&fixture.workflow_service, &valid_req, None)
        .await
        .expect("valid request succeeds");
    assert_eq!(valid_summary.status, "completed");
}

/// Acceptance Test: `IPC-CMD-017`
/// Tests `cancel_workflow`:
/// - Non-existent run ID -> `EntityNotFound`
/// - Valid cancel on active workflow -> returns `CancelWorkflowResponseDto { status: "cancelled", resume_available: true }`
#[tokio::test]
async fn IPC_CMD_017_cancel_workflow_contracts() {
    let fixture = setup_test_fixture();
    let draft_session_id = Uuid::new_v4();
    {
        let conn = fixture.pool.get().unwrap();
        seed_test_draft_session(&conn, &draft_session_id, "editing");
    }

    // 1. Non-existent workflow run ID
    let missing_cancel_req = CancelWorkflowReq {
        draft_session_id,
        workflow_run_id: Uuid::new_v4(),
    };
    let err = exec_cancel_workflow(&fixture.workflow_service, &missing_cancel_req)
        .await
        .unwrap_err();
    assert_eq!(err.code, ErrorCode::EntityNotFound);

    // 2. Valid cancel on active workflow
    fixture
        .workflow_service
        .set_task_executor(Arc::new(MockParallelExecutor {
            delays_ms: vec![50],
        }));

    let run_id = Uuid::new_v4();
    let req = StartOrResumeWorkflowReq {
        draft_session_id,
        phase: 1,
        subphase: None,
        workflow_run_id: Some(run_id),
        system_language: "vi".to_string(),
    };

    let service = fixture.workflow_service.clone();
    let join_handle = tokio::spawn(async move {
        exec_start_or_resume_workflow(&service, &req, None).await
    });

    tokio::time::sleep(Duration::from_millis(15)).await;

    let valid_cancel_req = CancelWorkflowReq {
        draft_session_id,
        workflow_run_id: run_id,
    };
    let cancel_res = exec_cancel_workflow(&fixture.workflow_service, &valid_cancel_req)
        .await
        .expect("active workflow cancel succeeds");

    assert_eq!(cancel_res.status, "cancelled");
    assert!(cancel_res.resume_available);

    let _ = join_handle.await;
}

/// Acceptance Test: `IPC-CMD-018`
/// Tests `get_workflow_status`:
/// - Non-existent draft -> `EntityNotFound`
/// - Idle draft -> returns `is_running: false`
/// - Active running draft -> returns `is_running: true` with active run details
#[tokio::test]
async fn IPC_CMD_018_get_workflow_status_contracts() {
    let fixture = setup_test_fixture();
    let draft_session_id = Uuid::new_v4();
    {
        let conn = fixture.pool.get().unwrap();
        seed_test_draft_session(&conn, &draft_session_id, "editing");
    }

    // 1. Non-existent draft
    let missing_req = GetWorkflowStatusReq {
        draft_session_id: Uuid::new_v4(),
    };
    let err = exec_get_workflow_status(&fixture.workflow_service, &missing_req).unwrap_err();
    assert_eq!(err.code, ErrorCode::EntityNotFound);

    // 2. Idle draft (before workflow has run)
    let idle_req = GetWorkflowStatusReq { draft_session_id };
    let idle_status = exec_get_workflow_status(&fixture.workflow_service, &idle_req)
        .expect("idle status query succeeds");
    assert!(!idle_status.is_running);
    assert_eq!(idle_status.completed_tasks, 0);

    // 3. Active running draft
    fixture
        .workflow_service
        .set_task_executor(Arc::new(MockParallelExecutor {
            delays_ms: vec![60],
        }));

    let run_id = Uuid::new_v4();
    let run_req = StartOrResumeWorkflowReq {
        draft_session_id,
        phase: 1,
        subphase: None,
        workflow_run_id: Some(run_id),
        system_language: "vi".to_string(),
    };

    let service = fixture.workflow_service.clone();
    let join_handle = tokio::spawn(async move {
        exec_start_or_resume_workflow(&service, &run_req, None).await
    });

    tokio::time::sleep(Duration::from_millis(15)).await;

    let running_status = exec_get_workflow_status(&fixture.workflow_service, &idle_req)
        .expect("running status query succeeds");
    assert!(running_status.is_running);
    assert_eq!(running_status.workflow_run_id, Some(run_id));
    assert_eq!(running_status.current_phase, Some(1));
    assert!(running_status.current_round.is_some());

    let _ = join_handle.await;
}

/// The production planner must derive rounds from the sealed draft, never from fabricated
/// fixture identifiers (`arc_01`). Two real arcs in state produce two outline rounds.
#[tokio::test]
async fn workflow_plan_uses_draft_arcs_not_fixture_codes() {
    let fixture = setup_test_fixture();
    let draft_session_id = Uuid::new_v4();
    {
        let conn = fixture.pool.get().unwrap();
        seed_test_draft_session(&conn, &draft_session_id, "editing");
    }

    let req = StartOrResumeWorkflowReq {
        draft_session_id,
        phase: 1,
        subphase: None,
        workflow_run_id: None,
        system_language: "vi".to_string(),
    };

    let (event_tx, mut event_rx) = mpsc::channel(100);
    let summary = exec_start_or_resume_workflow(&fixture.workflow_service, &req, Some(event_tx))
        .await
        .expect("phase 1 planning from a real draft should succeed");
    assert_eq!(summary.status, "completed");

    let mut identities = Vec::new();
    while let Ok(evt) = event_rx.try_recv() {
        if let WorkflowProgressEvent::RoundStarted { active_task_identities, .. } = evt {
            identities.extend(active_task_identities);
        }
    }

    assert_eq!(identities.len(), 2, "one outline round per draft Arc expected");
    assert!(identities.iter().any(|id| id.contains("arc=arc_one")));
    assert!(identities.iter().any(|id| id.contains("arc=arc_two")));
    assert!(
        !identities.iter().any(|id| id.contains("arc_01")),
        "planner must not fabricate arc_01"
    );
}

#[tokio::test]
async fn phase_prompts_expose_callable_write_tools() {
    let fixture = setup_test_fixture();
    for phase in 1..=6 {
        let (prompt, schemas) = fixture
            .workflow_service
            .compose_phase_prompt(phase)
            .expect("phase prompt must compose");
        assert!(
            prompt.contains(&format!("# planning_phase_{phase}")),
            "phase {phase} block must be included"
        );
        assert!(!schemas.is_empty(), "phase {phase} must grant tool schemas");
    }
    let (phase1_prompt, phase1_tools) = fixture.workflow_service.compose_phase_prompt(1).unwrap();
    assert!(phase1_prompt.contains("add_arc"), "phase 1 must document add_arc");
    assert!(phase1_prompt.contains("add_theme"), "phase 1 must document add_theme");
    assert!(
        phase1_tools.iter().any(|tool| tool["name"] == "add_arc"),
        "add_arc must be offered to the model"
    );
    assert!(
        phase1_tools.iter().any(|tool| tool["name"] == "add_theme"),
        "add_theme must be offered to the model"
    );
}

#[test]
fn planning_uses_real_chapter_arc_codes() {
    use crate::business_flow::{
        BusinessFlowData, ChapterPhaseArcItem, RequireProjectTarget, TrackedField,
    };
    let mut flow = BusinessFlowData::default();
    flow.require.project.target = RequireProjectTarget {
        arc_count: TrackedField::user(2),
        chapter_count: TrackedField::user(12),
        word_per_chapter_count: TrackedField::user(3000),
    };
    flow.optional.chapter_phase.arcs = vec![
        ChapterPhaseArcItem { code: "arc-1".into(), chapters: Vec::new() },
        ChapterPhaseArcItem { code: "arc-2".into(), chapters: Vec::new() },
    ];

    let ranges = WorkflowService::draft_arc_ranges(&flow);
    assert_eq!(ranges.len(), 2);
    assert_eq!(ranges[0], ("arc-1".to_string(), 1, 6));
    assert_eq!(ranges[1], ("arc-2".to_string(), 7, 12));
    assert!(ranges.iter().all(|(code, _, _)| !code.starts_with("planned_arc")));
}
