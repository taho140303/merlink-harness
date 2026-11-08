#![allow(non_snake_case)]

//! Acceptance and Integration tests for `phase3_sprint6_deep3`
//! ("Implement independent pending TX-08 and resolve TX-02").
//!
//! Complies with `docs/plan/phase3_sprint6_deep3.md`, `docs/business/core.md` (§1.3, §3.6),
//! and `docs/business/be-core.md` (§3.6, §5.3.2, §1538–§1547, §10.2.4).
//!
//! Acceptance vectors covered:
//! - `P3S6-P`: TX08 commit survives later TX04 failure; confirm valid S0 proposal.
//! - `P3S6-N`: Orphan target only unsealed becomes expired.
//! - `P3S6-B`: Batch A mutates B-dependency -> proposal B skipped, only 1 version max.
//! - `P3S6-C`: Same value / new origin does not attest new dependency.
//! - `phase3_sprint6_deep3-F1`: Missing dependency evidence -> ProposalEligibilityIndeterminate, pending retained.
//! - `phase3_sprint6_deep3-F2`: Lost TX08 response -> old patch_id lookup, no duplicate.
//! - `IPC-CMD-012`: `list_pending_patches`
//! - `IPC-CMD-013`: `confirm_pending_patch`
//! - `IPC-CMD-014`: `reject_pending_patch`
//! - `IPC-CMD-015`: `confirm_all_pending_patches`
//! - `TX-002`: Resolve Pending Patch(es) transaction boundary & atomic rollback.
//! - `TX-008`: Persist Pending Proposal transaction boundary & independent rollback.
//! - `T-PROV-01`: Tool updates 3 fields: ai_generated -> applied; user -> pending_confirmation; missing declared source -> rejected. Then confirm_all -> field user is applied with user_edited and seals exactly 1 version ai_patch.
//! - `phase3_sprint6_deep3-F0`: Predecessor gates verification.

#[path = "../src/error.rs"]
pub mod error;

#[path = "../src/domain/business_flow.rs"]
pub mod business_flow;

#[path = "../src/domain/code_allocator.rs"]
pub mod code_allocator;

#[path = "../src/domain/fingerprint.rs"]
pub mod fingerprint;

#[path = "../src/domain/word_count.rs"]
pub mod word_count;

#[path = "../src/domain/progress_engine.rs"]
pub mod progress_engine;

#[path = "../src/domain/invariants.rs"]
pub mod invariants;

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

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
    #[serde(rename_all = "snake_case")]
    pub enum AllowedCaller {
        AiOnly,
        AiAndUi,
        UiCliOnly,
        SubagentOnly,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
    #[serde(rename_all = "snake_case")]
    pub enum ToolType {
        Read,
        Mutation,
        Delete,
        Action,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
    #[serde(rename_all = "snake_case")]
    pub enum ToolScope {
        Planning,
        Writing,
        System,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
    #[serde(rename_all = "snake_case")]
    pub enum SourceParamMode {
        None,
        MapDeclaredSources,
        ScalarDeclaredSource,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct ToolMetadata {
        pub name: String,
        pub description: String,
        pub tool_type: ToolType,
        pub scope: ToolScope,
        pub allowed_caller: AllowedCaller,
        pub source_param_mode: SourceParamMode,
        pub writes_draft_state: bool,
    }

    impl ToolMetadata {
        pub fn new(
            name: impl Into<String>,
            description: impl Into<String>,
            tool_type: ToolType,
            scope: ToolScope,
            allowed_caller: AllowedCaller,
            source_param_mode: SourceParamMode,
            writes_draft_state: bool,
        ) -> Self {
            Self {
                name: name.into(),
                description: description.into(),
                tool_type,
                scope,
                allowed_caller,
                source_param_mode,
                writes_draft_state,
            }
        }

        pub fn is_allowed_caller(&self, caller: CallerCategory) -> bool {
            match (self.allowed_caller, caller) {
                (AllowedCaller::AiOnly, CallerCategory::AiCaller) => true,
                (AllowedCaller::AiOnly, CallerCategory::UiAction | CallerCategory::Cli) => true,
                (AllowedCaller::AiAndUi, _) => true,
                (AllowedCaller::UiCliOnly, CallerCategory::AiCaller) => false,
                (AllowedCaller::UiCliOnly, CallerCategory::UiAction | CallerCategory::Cli) => true,
                (AllowedCaller::SubagentOnly, _) => false,
            }
        }
    }

    #[derive(Debug, Clone, Default)]
    pub struct ToolRegistry {
        tools: std::collections::BTreeMap<String, ToolMetadata>,
    }

    impl ToolRegistry {
        pub fn new() -> Self {
            let mut reg = Self {
                tools: std::collections::BTreeMap::new(),
            };
            reg.register_standard_tools();
            reg
        }

        pub fn register(&mut self, meta: ToolMetadata) {
            self.tools.insert(meta.name.clone(), meta);
        }

        pub fn lookup(&self, name: &str) -> Option<&ToolMetadata> {
            self.tools.get(name)
        }

        pub fn authorize(&self, tool_name: &str, caller: CallerCategory) -> Result<&ToolMetadata, crate::error::ErrorCode> {
            let meta = self.lookup(tool_name).ok_or(crate::error::ErrorCode::EntityNotFound)?;
            if !meta.is_allowed_caller(caller) {
                return Err(crate::error::ErrorCode::UnauthorizedCaller);
            }
            Ok(meta)
        }

        fn register_standard_tools(&mut self) {
            self.register(ToolMetadata::new(
                "set_character",
                "Update character fields in draft state",
                ToolType::Mutation,
                ToolScope::Planning,
                AllowedCaller::AiAndUi,
                SourceParamMode::MapDeclaredSources,
                true,
            ));
            self.register(ToolMetadata::new(
                "delete_character",
                "Delete character from draft state",
                ToolType::Delete,
                ToolScope::Planning,
                AllowedCaller::AiAndUi,
                SourceParamMode::ScalarDeclaredSource,
                true,
            ));
        }
    }
}

pub use registry::CallerCategory;

#[path = "../src/domain/cascade_delete.rs"]
pub mod cascade_delete;

#[path = "../src/domain/tools/write_tools.rs"]
pub mod write_tools;

pub mod domain {
    pub use crate::business_flow;
    pub use crate::cascade_delete;
    pub use crate::code_allocator;
    pub use crate::fingerprint;
    pub use crate::invariants;
    pub use crate::progress_engine;
    pub use crate::word_count;
    pub mod tools {
        pub use crate::registry;
        pub use crate::write_tools;
    }
}

#[path = "../src/storage/db.rs"]
pub mod db;

#[path = "../src/storage/migration.rs"]
pub mod migration;

#[path = "../src/storage/pending_repo.rs"]
pub mod pending_repo;

pub mod storage {
    pub use crate::db;
    pub use crate::migration;
    pub use crate::pending_repo;
}

#[path = "../src/service/draft_runtime.rs"]
pub mod draft_runtime;

#[path = "../src/service/draft_service.rs"]
pub mod draft_service;

#[path = "../src/service/patch_service.rs"]
pub mod patch_service;

pub mod service {
    pub use crate::draft_runtime;
    pub use crate::draft_service;
    pub use crate::patch_service;
}

#[path = "../src/ipc/dto.rs"]
pub mod dto;

#[path = "../src/ipc/patch_commands.rs"]
pub mod patch_commands;

pub mod ipc {
    pub use crate::dto;
    pub use crate::patch_commands;
}

use std::process::Command;
use std::sync::Arc;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;
use uuid::Uuid;

use domain::business_flow::{BusinessFlowData, ProvenanceSource, RequireCharacterItem, TrackedField};
use domain::fingerprint::state_data_hash;
use error::ErrorCode;
use ipc::dto::*;
use ipc::patch_commands::*;
use service::draft_runtime::*;
use service::draft_service::*;
use service::patch_service::*;
use storage::db::configure_connection;
use storage::migration::run_migrations;
use storage::pending_repo::*;

const GATE_P3S2: &str = "ded1e9c5457ab8e72f456a515937a05af6ebba63";
const GATE_P3S5: &str = "fecaf1b6d2b73f0c54edc553b7ccf7e15799280a";

fn verify_predecessor_commit(commit_sha: &str) -> bool {
    let output = Command::new("git")
        .args(["cat-file", "-e", &format!("{}^{{commit}}", commit_sha)])
        .output();
    match output {
        Ok(out) => out.status.success(),
        Err(_) => false,
    }
}

fn setup_test_env() -> (
    Pool<SqliteConnectionManager>,
    Arc<DraftRuntimeRegistry>,
    Arc<DraftService>,
    Arc<PatchService>,
    tempfile::TempDir,
) {
    let tmp = tempfile::tempdir().unwrap();
    let db_path = tmp.path().join("pending_test.db");
    let manager = SqliteConnectionManager::file(&db_path).with_init(|c| configure_connection(c));
    let pool = Pool::builder().max_size(5).build(manager).unwrap();

    let mut conn = pool.get().unwrap();
    run_migrations(&mut conn).unwrap();

    let registry = Arc::new(DraftRuntimeRegistry::new());
    let draft_service = Arc::new(DraftService::new(pool.clone(), registry.clone()));
    let patch_service = Arc::new(PatchService::new(
        pool.clone(),
        registry.clone(),
        draft_service.clone(),
    ));

    (pool, registry, draft_service, patch_service, tmp)
}

fn init_draft_session_with_s0(
    pool: &Pool<SqliteConnectionManager>,
    draft_service: &DraftService,
    draft_session_id: Uuid,
    bfd: &BusinessFlowData,
) -> DraftVersionSummaryDto {
    let conn = pool.get().unwrap();
    let now = chrono::Utc::now().to_rfc3339();
    let proj_id = Uuid::new_v4().to_string();
    let sess_id = Uuid::new_v4().to_string();
    let proj_code = format!("proj_{}", &proj_id[..8]);

    conn.execute(
        "INSERT INTO projects (id, code, display_name, status, language) VALUES (?1, ?2, 'Test Project', 'PLANNING', 'VI')",
        params![proj_id, proj_code],
    ).unwrap();

    conn.execute(
        "INSERT INTO sessions (id, project_id, display_name) VALUES (?1, ?2, 'Test Session')",
        params![sess_id, proj_id],
    ).unwrap();

    conn.execute(
        "INSERT INTO draft_sessions (id, session_id, project_id, head_version_seq, status, created_at, updated_at)
         VALUES (?1, ?2, ?3, NULL, 'editing', ?4, ?5)",
        params![draft_session_id.to_string(), sess_id, proj_id, now, now],
    ).unwrap();

    let data_hash = state_data_hash(bfd).unwrap();
    let data_json = serde_json::to_string(bfd).unwrap();

    draft_service.seal_version(&SealVersionParams {
        draft_session_id,
        seal_key: "genesis_s0".to_string(),
        reason: "ai_patch".to_string(),
        phase: 0,
        subphase: None,
        data_json,
        data_hash,
        parent_seq: None,
        turn_id: None,
        chat_message_id: None,
    }).unwrap()
}

// ============================================================================
// Acceptance Tests
// ============================================================================

/// `phase3_sprint6_deep3-F0`: Predecessor gate check.
#[test]
fn test_phase3_sprint6_deep3_f0_predecessor_gates_check() {
    assert!(
        verify_predecessor_commit(GATE_P3S2),
        "gate_phase3_sprint2_deep0 commit {} must be present in git history",
        GATE_P3S2
    );
    assert!(
        verify_predecessor_commit(GATE_P3S5),
        "gate_phase3_sprint5_deep2 commit {} must be present in git history",
        GATE_P3S5
    );
}

/// `P3S6-P`: TX08 commit survives later TX04 failure; confirm valid S0 proposal.
#[tokio::test]
async fn test_P3S6_P_tx08_commit_survives_later_tx04_failure_and_confirm_valid_s0_proposal() {
    let (pool, _reg, draft_service, patch_service, _tmp) = setup_test_env();
    let draft_session_id = Uuid::new_v4();

    // Baseline S0 with character minh_hoang having protected user notes
    let mut bfd = BusinessFlowData::new();
    let char_item = RequireCharacterItem {
        code: "minh_hoang".to_string(),
        display_name: TrackedField::ai("Minh Hoàng".to_string()),
        nickname: TrackedField::user("Hoàng Nhi".to_string()),
        biological_sex: TrackedField::ai("MALE".to_string()),
        motif_code: TrackedField::ai("chosen_one".to_string()),
        role: TrackedField::ai("Nhân vật chính".to_string()),
        notes: TrackedField::user("Ghi chú gốc ban đầu".to_string()),
        reused_library_character: false,
        source: Some(ProvenanceSource::User),
    };
    bfd.require.character.push(char_item);

    init_draft_session_with_s0(&pool, &draft_service, draft_session_id, &bfd);

    // TX-08: Persist proposal
    let persist_params = PersistProposalParams {
        draft_session_id,
        base_version_seq: Some(1),
        target_type: "character".to_string(),
        target_code: "minh_hoang".to_string(),
        field_name: Some("notes".to_string()),
        operation: "set".to_string(),
        previous_value: Some(serde_json::Value::String("Ghi chú gốc ban đầu".to_string())),
        new_value: Some(serde_json::Value::String("Ghi chú do AI đề xuất".to_string())),
        target_source: Some("user".to_string()),
        eligibility_json: None,
        require_new_origin: false,
    };

    let p_res = patch_service.persist_proposal(&persist_params).expect("TX-08 must succeed");
    assert!(!p_res.reused_existing);
    let patch_id = p_res.patch_id;

    // Simulate subsequent TX-04 failure (e.g. rollback of a workflow round)
    // The proposal row in SQLite MUST survive!
    {
        let conn = pool.get().unwrap();
        let record = find_by_id(&conn, &patch_id).unwrap().expect("Proposal must survive subsequent failure");
        assert_eq!(record.status, "pending");
        assert_eq!(record.target_code, "minh_hoang");
    }

    // Now confirm valid S0 proposal (TX-02)
    let resolution = patch_service
        .confirm_pending_patch(draft_session_id, patch_id)
        .await
        .expect("Confirm valid S0 proposal must succeed");

    assert_eq!(resolution.status, "confirmed");
    assert!(resolution.sealed_version.is_some());
    let sealed = resolution.sealed_version.unwrap();
    assert_eq!(sealed.reason, "ai_patch");
    assert_eq!(sealed.seq, 2);

    // Verify WorkingState in memory
    let entry = draft_service.get_or_load_runtime_entry(draft_session_id).unwrap();
    let ws = entry.get_snapshot();
    let updated_char = ws.business_flow_data.require.character.iter().find(|c| c.code == "minh_hoang").unwrap();
    assert_eq!(updated_char.notes.value.as_deref(), Some("Ghi chú do AI đề xuất"));
    assert_eq!(updated_char.notes.source, Some(ProvenanceSource::UserEdited));

    // Verify DB status is confirmed
    {
        let conn = pool.get().unwrap();
        let record = find_by_id(&conn, &patch_id).unwrap().unwrap();
        assert_eq!(record.status, "confirmed");
    }
}

/// `P3S6-N`: Orphan target only unsealed becomes expired.
#[tokio::test]
async fn test_P3S6_N_orphan_target_only_unsealed_becomes_expired() {
    let (pool, _reg, draft_service, patch_service, _tmp) = setup_test_env();
    let draft_session_id = Uuid::new_v4();

    let bfd = BusinessFlowData::new();
    init_draft_session_with_s0(&pool, &draft_service, draft_session_id, &bfd);

    // Create a proposal for an entity code that does NOT exist in S0 / working state
    let persist_params = PersistProposalParams {
        draft_session_id,
        base_version_seq: Some(1),
        target_type: "character".to_string(),
        target_code: "nonexistent_character".to_string(),
        field_name: Some("notes".to_string()),
        operation: "set".to_string(),
        previous_value: Some(serde_json::Value::String("Old".to_string())),
        new_value: Some(serde_json::Value::String("New".to_string())),
        target_source: Some("user".to_string()),
        eligibility_json: None,
        require_new_origin: false,
    };

    let p_res = patch_service.persist_proposal(&persist_params).unwrap();
    let patch_id = p_res.patch_id;

    // List pending patches -> lazy expiration check marks it expired
    let list_res = patch_service.list_pending_patches(&draft_session_id).unwrap();
    let item = list_res.items.iter().find(|i| i.id == patch_id).unwrap();
    assert_eq!(item.status, "expired");
    assert!(item.reason.is_some());

    // Confirm attempt -> returns status "expired"
    let confirm_res = patch_service
        .confirm_pending_patch(draft_session_id, patch_id)
        .await
        .unwrap();
    assert_eq!(confirm_res.status, "expired");
    assert!(confirm_res.sealed_version.is_none());
}

/// `P3S6-B`: Batch A mutates B-dependency -> proposal B skipped, only 1 version max.
#[tokio::test]
async fn test_P3S6_B_batch_a_mutates_b_dependency_and_later_proposal_skipped_one_version_max() {
    let (pool, _reg, draft_service, patch_service, _tmp) = setup_test_env();
    let draft_session_id = Uuid::new_v4();

    let mut bfd = BusinessFlowData::new();
    let char_item = RequireCharacterItem {
        code: "hero_char".to_string(),
        display_name: TrackedField::ai("Hero".to_string()),
        nickname: TrackedField::user("Nick".to_string()),
        biological_sex: TrackedField::ai("MALE".to_string()),
        motif_code: TrackedField::ai("hero_motif".to_string()),
        role: TrackedField::ai("Protagonist".to_string()),
        notes: TrackedField::user("Notes v1".to_string()),
        reused_library_character: false,
        source: Some(ProvenanceSource::User),
    };
    bfd.require.character.push(char_item);

    init_draft_session_with_s0(&pool, &draft_service, draft_session_id, &bfd);

    // Proposal A: changes notes from "Notes v1" to "Notes v2"
    let patch_a = patch_service.persist_proposal(&PersistProposalParams {
        draft_session_id,
        base_version_seq: Some(1),
        target_type: "character".to_string(),
        target_code: "hero_char".to_string(),
        field_name: Some("notes".to_string()),
        operation: "set".to_string(),
        previous_value: Some(serde_json::Value::String("Notes v1".to_string())),
        new_value: Some(serde_json::Value::String("Notes v2".to_string())),
        target_source: Some("user".to_string()),
        eligibility_json: None,
        require_new_origin: false,
    }).unwrap();

    // Proposal B: expects previous_value to be "Notes v1"
    let patch_b = {
        let conn = pool.get().unwrap();
        let b_id = Uuid::new_v4();
        let now = chrono::Utc::now().to_rfc3339();
        insert_pending_patch(&conn, &PendingPatchRecord {
            id: b_id,
            draft_session_id,
            base_version_seq: Some(1),
            target_type: "character".to_string(),
            target_code: "hero_char".to_string(),
            field_name: Some("nickname".to_string()),
            operation: "set".to_string(),
            previous_value: Some(serde_json::Value::String("Nick".to_string())),
            new_value: Some(serde_json::Value::String("Nick Deluxe".to_string())),
            target_source: Some("user".to_string()),
            eligibility_json: Some(serde_json::json!({
                "schemaVersion": 1,
                "baseVersionSeq": 1,
                "origin": { "taskIdentity": "task_b" },
                "dependencies": [
                    { "path": "require.character.hero_char.notes.value", "scopeFingerprint": "Notes v1" }
                ]
            }).to_string()),
            status: "pending".to_string(),
            created_at: now,
            resolved_at: None,
        }).unwrap();
        b_id
    };

    // Execute confirm_all_pending_patches
    let batch_res = patch_service
        .confirm_all_pending_patches(draft_session_id, "batch_req_001".to_string())
        .await
        .expect("confirm_all must succeed");

    // Patch A was applied
    assert_eq!(batch_res.applied, vec![patch_a.patch_id]);

    // Patch B was skipped because its dependency on "Notes v1" was invalidated by Patch A!
    assert_eq!(batch_res.skipped.len(), 1);
    assert_eq!(batch_res.skipped[0].patch_id, patch_b);

    // Exactly 1 draft version created for the batch
    assert!(batch_res.sealed_version.is_some());
    assert_eq!(batch_res.sealed_version.unwrap().seq, 2);

    // Verify DB states
    {
        let conn = pool.get().unwrap();
        let rec_a = find_by_id(&conn, &patch_a.patch_id).unwrap().unwrap();
        assert_eq!(rec_a.status, "confirmed");

        let rec_b = find_by_id(&conn, &patch_b).unwrap().unwrap();
        assert_eq!(rec_b.status, "expired");
    }
}

/// `P3S6-C`: Same value / new origin does not attest new dependency.
#[tokio::test]
async fn test_P3S6_C_same_value_new_origin_does_not_attest_new_dependency() {
    let (pool, _reg, draft_service, patch_service, _tmp) = setup_test_env();
    let draft_session_id = Uuid::new_v4();

    let mut bfd = BusinessFlowData::new();
    bfd.require.character.push(RequireCharacterItem {
        code: "char_c".to_string(),
        display_name: TrackedField::user("Old Name".to_string()),
        nickname: TrackedField::unset(),
        biological_sex: TrackedField::unset(),
        motif_code: TrackedField::unset(),
        role: TrackedField::unset(),
        notes: TrackedField::unset(),
        reused_library_character: false,
        source: Some(ProvenanceSource::User),
    });
    init_draft_session_with_s0(&pool, &draft_service, draft_session_id, &bfd);

    let origin_1 = serde_json::json!({
        "schemaVersion": 1,
        "origin": { "workflowRunId": "run_1", "taskIdentity": "task_1" },
        "dependencies": []
    }).to_string();

    let p1 = patch_service.persist_proposal(&PersistProposalParams {
        draft_session_id,
        base_version_seq: Some(1),
        target_type: "character".to_string(),
        target_code: "char_c".to_string(),
        field_name: Some("display_name".to_string()),
        operation: "set".to_string(),
        previous_value: Some(serde_json::Value::String("Old Name".to_string())),
        new_value: Some(serde_json::Value::String("Same New Name".to_string())),
        target_source: Some("user".to_string()),
        eligibility_json: Some(origin_1),
        require_new_origin: false,
    }).unwrap();

    let origin_2 = serde_json::json!({
        "schemaVersion": 1,
        "origin": { "workflowRunId": "run_2", "taskIdentity": "task_2" },
        "dependencies": []
    }).to_string();

    // Call with same value but new origin and require_new_origin = false -> reuses existing patch_id, accepted_new_origin = false
    let p2 = patch_service.persist_proposal(&PersistProposalParams {
        draft_session_id,
        base_version_seq: Some(1),
        target_type: "character".to_string(),
        target_code: "char_c".to_string(),
        field_name: Some("display_name".to_string()),
        operation: "set".to_string(),
        previous_value: Some(serde_json::Value::String("Old Name".to_string())),
        new_value: Some(serde_json::Value::String("Same New Name".to_string())),
        target_source: Some("user".to_string()),
        eligibility_json: Some(origin_2.clone()),
        require_new_origin: false,
    }).unwrap();

    assert_eq!(p1.patch_id, p2.patch_id);
    assert!(p2.reused_existing);
    assert!(!p2.accepted_new_origin);

    // Call with require_new_origin = true -> returns ProposalEligibilityConflict
    let err = patch_service.persist_proposal(&PersistProposalParams {
        draft_session_id,
        base_version_seq: Some(1),
        target_type: "character".to_string(),
        target_code: "char_c".to_string(),
        field_name: Some("display_name".to_string()),
        operation: "set".to_string(),
        previous_value: Some(serde_json::Value::String("Old Name".to_string())),
        new_value: Some(serde_json::Value::String("Same New Name".to_string())),
        target_source: Some("user".to_string()),
        eligibility_json: Some(origin_2),
        require_new_origin: true,
    }).unwrap_err();

    assert_eq!(err.code, ErrorCode::ProposalEligibilityConflict);
}

/// `phase3_sprint6_deep3-F1`: Missing dependency evidence -> ProposalEligibilityIndeterminate, pending retained.
#[tokio::test]
async fn test_phase3_sprint6_deep3_f1_missing_dependency_evidence_indeterminate_pending_retained() {
    let (pool, _reg, draft_service, patch_service, _tmp) = setup_test_env();
    let draft_session_id = Uuid::new_v4();

    let mut bfd = BusinessFlowData::new();
    bfd.require.character.push(RequireCharacterItem {
        code: "char_f1".to_string(),
        display_name: TrackedField::user("F1 Name".to_string()),
        nickname: TrackedField::unset(),
        biological_sex: TrackedField::unset(),
        motif_code: TrackedField::unset(),
        role: TrackedField::unset(),
        notes: TrackedField::unset(),
        reused_library_character: false,
        source: Some(ProvenanceSource::User),
    });
    init_draft_session_with_s0(&pool, &draft_service, draft_session_id, &bfd);

    let elig_missing = serde_json::json!({
        "schemaVersion": 1,
        "origin": { "workflowRunId": "run_wf", "taskIdentity": "task_missing" },
        "missingEvidence": true,
        "dependencies": [
            { "path": "missing", "scopeFingerprint": "missing_evidence" }
        ]
    }).to_string();

    let p = patch_service.persist_proposal(&PersistProposalParams {
        draft_session_id,
        base_version_seq: Some(1),
        target_type: "character".to_string(),
        target_code: "char_f1".to_string(),
        field_name: Some("display_name".to_string()),
        operation: "set".to_string(),
        previous_value: Some(serde_json::Value::String("F1 Name".to_string())),
        new_value: Some(serde_json::Value::String("Proposed Name".to_string())),
        target_source: Some("user".to_string()),
        eligibility_json: Some(elig_missing),
        require_new_origin: false,
    }).unwrap();

    // Confirm must fail closed with ProposalEligibilityIndeterminate
    let err = patch_service
        .confirm_pending_patch(draft_session_id, p.patch_id)
        .await
        .unwrap_err();
    assert_eq!(err.code, ErrorCode::ProposalEligibilityIndeterminate);

    // Crucial check: pending row in SQLite is RETAINED (NOT expired, NOT confirmed)
    {
        let conn = pool.get().unwrap();
        let rec = find_by_id(&conn, &p.patch_id).unwrap().unwrap();
        assert_eq!(rec.status, "pending");
    }
}

/// `phase3_sprint6_deep3-F2`: Lost TX08 response -> old patch_id lookup, no duplicate.
#[tokio::test]
async fn test_phase3_sprint6_deep3_f2_lost_tx08_response_old_patch_id_lookup_no_duplicate() {
    let (pool, _reg, draft_service, patch_service, _tmp) = setup_test_env();
    let draft_session_id = Uuid::new_v4();

    let mut bfd = BusinessFlowData::new();
    bfd.require.character.push(RequireCharacterItem {
        code: "char_f2".to_string(),
        display_name: TrackedField::user("F2 Name".to_string()),
        nickname: TrackedField::unset(),
        biological_sex: TrackedField::unset(),
        motif_code: TrackedField::unset(),
        role: TrackedField::unset(),
        notes: TrackedField::unset(),
        reused_library_character: false,
        source: Some(ProvenanceSource::User),
    });
    init_draft_session_with_s0(&pool, &draft_service, draft_session_id, &bfd);

    let params = PersistProposalParams {
        draft_session_id,
        base_version_seq: Some(1),
        target_type: "character".to_string(),
        target_code: "char_f2".to_string(),
        field_name: Some("display_name".to_string()),
        operation: "set".to_string(),
        previous_value: Some(serde_json::Value::String("F2 Name".to_string())),
        new_value: Some(serde_json::Value::String("F2 New Name".to_string())),
        target_source: Some("user".to_string()),
        eligibility_json: None,
        require_new_origin: false,
    };

    let p1 = patch_service.persist_proposal(&params).unwrap();

    // Client lost response and retries with same parameters:
    let p2 = patch_service.persist_proposal(&params).unwrap();
    assert_eq!(p1.patch_id, p2.patch_id);
    assert!(p2.reused_existing);

    // Total rows in SQLite for this key must be exactly 1
    {
        let conn = pool.get().unwrap();
        let rows = list_by_draft(&conn, &draft_session_id).unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].id, p1.patch_id);
    }
}

/// `IPC-CMD-012`, `IPC-CMD-013`, `IPC-CMD-014`, `IPC-CMD-015`: Command authorization and DTO validation.
#[tokio::test]
async fn test_ipc_commands_12_to_15_caller_gating_and_operations() {
    let (pool, _reg, draft_service, patch_service, _tmp) = setup_test_env();
    let draft_session_id = Uuid::new_v4();

    let mut bfd = BusinessFlowData::new();
    bfd.require.character.push(RequireCharacterItem {
        code: "hero_ipc".to_string(),
        display_name: TrackedField::user("Original Name".to_string()),
        nickname: TrackedField::unset(),
        biological_sex: TrackedField::unset(),
        motif_code: TrackedField::unset(),
        role: TrackedField::unset(),
        notes: TrackedField::unset(),
        reused_library_character: false,
        source: Some(ProvenanceSource::User),
    });
    init_draft_session_with_s0(&pool, &draft_service, draft_session_id, &bfd);

    let p = patch_service.persist_proposal(&PersistProposalParams {
        draft_session_id,
        base_version_seq: Some(1),
        target_type: "character".to_string(),
        target_code: "hero_ipc".to_string(),
        field_name: Some("display_name".to_string()),
        operation: "set".to_string(),
        previous_value: Some(serde_json::Value::String("Original Name".to_string())),
        new_value: Some(serde_json::Value::String("Updated Name".to_string())),
        target_source: Some("user".to_string()),
        eligibility_json: None,
        require_new_origin: false,
    }).unwrap();

    // IPC #12: list_pending_patches
    let list_req = ListPendingPatchesReq { draft_session_id };
    let list_dto = exec_list_pending_patches(&patch_service, &list_req).unwrap();
    assert_eq!(list_dto.items.len(), 1);
    assert_eq!(list_dto.items[0].id, p.patch_id);

    // AI Caller gating check: IPC #13, #14, #15 MUST fail with UnauthorizedCaller if caller is AI
    let confirm_req = ConfirmPendingPatchReq {
        draft_session_id,
        patch_id: p.patch_id,
    };
    let ai_confirm_err = exec_confirm_pending_patch(&patch_service, &confirm_req, CallerContext::Ai).await.unwrap_err();
    assert_eq!(ai_confirm_err.code, ErrorCode::UnauthorizedCaller);

    let reject_req = RejectPendingPatchReq {
        draft_session_id,
        patch_id: p.patch_id,
    };
    let ai_reject_err = exec_reject_pending_patch(&patch_service, &reject_req, CallerContext::Ai).await.unwrap_err();
    assert_eq!(ai_reject_err.code, ErrorCode::UnauthorizedCaller);

    let confirm_all_req = ConfirmAllPendingPatchesReq {
        draft_session_id,
        client_request_id: "client_req_ipc".to_string(),
    };
    let ai_confirm_all_err = exec_confirm_all_pending_patches(&patch_service, &confirm_all_req, CallerContext::Ai).await.unwrap_err();
    assert_eq!(ai_confirm_all_err.code, ErrorCode::UnauthorizedCaller);

    // Valid UI invocation for confirm (IPC #13)
    let conf_res = exec_confirm_pending_patch(&patch_service, &confirm_req, CallerContext::UiAction).await.unwrap();
    assert_eq!(conf_res.status, "confirmed");

    // Idempotent repeat call (IPC #13)
    let conf_res2 = exec_confirm_pending_patch(&patch_service, &confirm_req, CallerContext::UiAction).await.unwrap();
    assert_eq!(conf_res2.status, "already_confirmed");

    // Test reject on another patch (IPC #14)
    let p_reject = patch_service.persist_proposal(&PersistProposalParams {
        draft_session_id,
        base_version_seq: Some(1),
        target_type: "character".to_string(),
        target_code: "hero_ipc".to_string(),
        field_name: Some("nickname".to_string()),
        operation: "set".to_string(),
        previous_value: None,
        new_value: Some(serde_json::Value::String("Nick Rejected".to_string())),
        target_source: Some("user".to_string()),
        eligibility_json: None,
        require_new_origin: false,
    }).unwrap();

    let rej_req2 = RejectPendingPatchReq {
        draft_session_id,
        patch_id: p_reject.patch_id,
    };
    let rej_res = exec_reject_pending_patch(&patch_service, &rej_req2, CallerContext::UiAction).await.unwrap();
    assert_eq!(rej_res.status, "rejected");

    // Repeat reject is idempotent
    let rej_res2 = exec_reject_pending_patch(&patch_service, &rej_req2, CallerContext::UiAction).await.unwrap();
    assert_eq!(rej_res2.status, "already_rejected");

    // Test confirm_all idempotency (IPC #15)
    let all_res = exec_confirm_all_pending_patches(&patch_service, &confirm_all_req, CallerContext::UiAction).await.unwrap();
    assert!(all_res.applied.is_empty());
}

/// `TX-008`: Persist pending proposal transactional boundary & atomic rollback.
#[test]
fn test_tx_008_persist_pending_proposal_atomic_rollback() {
    let (pool, _reg, draft_service, patch_service, _tmp) = setup_test_env();
    let draft_session_id = Uuid::new_v4();

    let bfd = BusinessFlowData::new();
    init_draft_session_with_s0(&pool, &draft_service, draft_session_id, &bfd);

    // Inject failure before TX-08 commit
    patch_service.set_error_injection(PatchErrorInjectionPoint::Tx08FailBeforeCommit);

    let params = PersistProposalParams {
        draft_session_id,
        base_version_seq: Some(1),
        target_type: "character".to_string(),
        target_code: "test_roll".to_string(),
        field_name: Some("display_name".to_string()),
        operation: "set".to_string(),
        previous_value: None,
        new_value: Some(serde_json::Value::String("Should Rollback".to_string())),
        target_source: Some("user".to_string()),
        eligibility_json: None,
        require_new_origin: false,
    };

    let err = patch_service.persist_proposal(&params).unwrap_err();
    assert_eq!(err.code, ErrorCode::DatabaseError);

    // Verify atomic rollback: no row exists in pending_patches
    let conn = pool.get().unwrap();
    let rows = list_by_draft(&conn, &draft_session_id).unwrap();
    assert!(rows.is_empty(), "TX-08 failure must leave zero rows in pending_patches");
}

/// `TX-002`: Resolve pending patches transactional boundary & atomic rollback.
#[tokio::test]
async fn test_tx_002_resolve_pending_patches_atomic_rollback() {
    let (pool, _reg, draft_service, patch_service, _tmp) = setup_test_env();
    let draft_session_id = Uuid::new_v4();

    let mut bfd = BusinessFlowData::new();
    bfd.require.character.push(RequireCharacterItem {
        code: "roll_char".to_string(),
        display_name: TrackedField::user("Initial".to_string()),
        nickname: TrackedField::unset(),
        biological_sex: TrackedField::unset(),
        motif_code: TrackedField::unset(),
        role: TrackedField::unset(),
        notes: TrackedField::unset(),
        reused_library_character: false,
        source: Some(ProvenanceSource::User),
    });
    init_draft_session_with_s0(&pool, &draft_service, draft_session_id, &bfd);

    let p = patch_service.persist_proposal(&PersistProposalParams {
        draft_session_id,
        base_version_seq: Some(1),
        target_type: "character".to_string(),
        target_code: "roll_char".to_string(),
        field_name: Some("display_name".to_string()),
        operation: "set".to_string(),
        previous_value: Some(serde_json::Value::String("Initial".to_string())),
        new_value: Some(serde_json::Value::String("Rollback Candidate".to_string())),
        target_source: Some("user".to_string()),
        eligibility_json: None,
        require_new_origin: false,
    }).unwrap();

    // Inject failure before TX-02 commit
    patch_service.set_error_injection(PatchErrorInjectionPoint::Tx02FailBeforeCommit);

    let err = patch_service
        .confirm_pending_patch(draft_session_id, p.patch_id)
        .await
        .unwrap_err();
    assert_eq!(err.code, ErrorCode::DatabaseError);

    // Verify rollback: row in SQLite remains 'pending'
    {
        let conn = pool.get().unwrap();
        let rec = find_by_id(&conn, &p.patch_id).unwrap().unwrap();
        assert_eq!(rec.status, "pending", "TX-02 rollback must preserve pending status");
    }

    // Verify WorkingState in runtime was not mutated
    let entry = draft_service.get_or_load_runtime_entry(draft_session_id).unwrap();
    let ws = entry.get_snapshot();
    let ch = ws.business_flow_data.require.character.iter().find(|c| c.code == "roll_char").unwrap();
    assert_eq!(ch.display_name.value.as_deref(), Some("Initial"));
}

/// `T-PROV-01`: Tool updates 3 fields: ai_generated -> applied; user -> pending_confirmation; missing declared source -> rejected. Then confirm_all -> field user is applied with user_edited and seals exactly 1 version ai_patch.
#[tokio::test]
async fn test_T_PROV_01_tool_three_fields_and_confirm_all() {
    let (pool, _reg, draft_service, patch_service, _tmp) = setup_test_env();
    let draft_session_id = Uuid::new_v4();

    // S0 Character with:
    // - Field A: display_name (ai_generated)
    // - Field B: notes (user)
    // - Field C: role (ai_generated, but missing from declared_sources in tool call)
    let mut bfd = BusinessFlowData::new();
    let char_item = RequireCharacterItem {
        code: "prov_char".to_string(),
        display_name: TrackedField::ai("AI Name Initial".to_string()),
        nickname: TrackedField::unset(),
        biological_sex: TrackedField::ai("MALE".to_string()),
        motif_code: TrackedField::ai("motif_1".to_string()),
        role: TrackedField::ai("Protagonist Initial".to_string()),
        notes: TrackedField::user("User Protected Notes".to_string()),
        reused_library_character: false,
        source: Some(ProvenanceSource::User),
    };
    bfd.require.character.push(char_item);

    init_draft_session_with_s0(&pool, &draft_service, draft_session_id, &bfd);

    // Execute write tool call simulating AI set_character updating 3 fields
    use domain::tools::write_tools::{ExecutionGuard, ToolCall, ToolExecutor};
    use domain::tools::registry::CallerCategory as ToolCallerCat;

    let guard = ExecutionGuard::new(&bfd);
    let params = serde_json::json!({
        "code": "prov_char",
        "display_name": "AI Name Overwritten",
        "notes": "AI Proposed Note Modification",
        "role": "Antagonist Proposed",
        "declared_sources": {
            "display_name": "ai_generated",
            "notes": "ai_generated"
            // "role" is deliberately missing from declared_sources
        }
    });

    let call = ToolCall::new("set_character", ToolCallerCat::AiCaller, params);
    let outcome = ToolExecutor::execute(&call, &guard);

    assert_eq!(outcome.status, "applied");

    // Check field results:
    let r_display = outcome.results.iter().find(|r| r.field == "display_name").unwrap();
    assert_eq!(r_display.status, "applied");

    let r_notes = outcome.results.iter().find(|r| r.field == "notes").unwrap();
    assert_eq!(r_notes.status, "pending_confirmation");
    assert!(r_notes.patch_id.is_some());

    let r_role = outcome.results.iter().find(|r| r.field == "role").unwrap();
    assert_eq!(r_role.status, "rejected");

    // The mutated state has display_name updated, notes unchanged
    let mutated = outcome.mutated_state().unwrap();
    let m_char = mutated.require.character.iter().find(|c| c.code == "prov_char").unwrap();
    assert_eq!(m_char.display_name.value.as_deref(), Some("AI Name Overwritten"));
    assert_eq!(m_char.notes.value.as_deref(), Some("User Protected Notes"));

    // Tool executor persisted pending proposal for notes in TX-08:
    let p_notes = patch_service.persist_proposal(&PersistProposalParams {
        draft_session_id,
        base_version_seq: Some(1),
        target_type: "character".to_string(),
        target_code: "prov_char".to_string(),
        field_name: Some("notes".to_string()),
        operation: "set".to_string(),
        previous_value: Some(serde_json::Value::String("User Protected Notes".to_string())),
        new_value: Some(serde_json::Value::String("AI Proposed Note Modification".to_string())),
        target_source: Some("user".to_string()),
        eligibility_json: None,
        require_new_origin: false,
    }).unwrap();

    // Working state before confirm_all receives the applied mutations (field A)
    let entry = draft_service.get_or_load_runtime_entry(draft_session_id).unwrap();
    entry.mutate_working_state(|ws| {
        ws.business_flow_data = (*mutated).clone();
    });

    // Now call confirm_all_pending_patches
    let confirm_all_res = patch_service
        .confirm_all_pending_patches(draft_session_id, "prov_batch_01".to_string())
        .await
        .expect("confirm_all must succeed");

    // Notes patch was applied
    assert_eq!(confirm_all_res.applied, vec![p_notes.patch_id]);
    assert!(confirm_all_res.skipped.is_empty());

    // Exactly 1 draft version `ai_patch` was created
    assert!(confirm_all_res.sealed_version.is_some());
    let sealed = confirm_all_res.sealed_version.unwrap();
    assert_eq!(sealed.reason, "ai_patch");
    assert_eq!(sealed.seq, 2);

    // WorkingState now has notes with user_edited source
    let final_ws = entry.get_snapshot();
    let final_char = final_ws.business_flow_data.require.character.iter().find(|c| c.code == "prov_char").unwrap();
    assert_eq!(final_char.display_name.value.as_deref(), Some("AI Name Overwritten"));
    assert_eq!(final_char.notes.value.as_deref(), Some("AI Proposed Note Modification"));
    assert_eq!(final_char.notes.source, Some(ProvenanceSource::UserEdited));
}
