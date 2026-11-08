//! Acceptance and integration tests for `phase4_sprint5_deep2`
//! ("Stream scene generation under EditScope").
//!
//! Complies with docs/business/core.md (§1.5, §1.6), docs/business/be-core.md (§8.3, §10.2.9, §10.3.3),
//! and docs/plan/evidence/contract-ledger.md (`REQ-WRITE-002`, `REQ-CMD-032`..`034`, `REQ-SPECTEST-T-LOCK-01`).

#![allow(non_snake_case)]

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

pub mod domain {
    pub use crate::business_flow;
    pub use crate::code_allocator;
    pub use crate::fingerprint;
    pub use crate::invariants;
    pub use crate::progress_engine;
    pub use crate::word_count;
}

#[path = "../src/storage/db.rs"]
pub mod db;
#[path = "../src/storage/migration.rs"]
pub mod migration;
#[path = "../src/storage/writing_repo.rs"]
pub mod writing_repo;

pub mod storage {
    pub use crate::db;
    pub use crate::migration;
    pub use crate::writing_repo;
}

#[path = "../src/providers/traits.rs"]
pub mod traits;
#[path = "../src/providers/redaction.rs"]
pub mod redaction;
#[path = "../src/providers/credentials.rs"]
pub mod credentials;
#[path = "../src/providers/ollama.rs"]
pub mod ollama;
#[path = "../src/providers/opencode.rs"]
pub mod opencode;

pub mod providers {
    pub use crate::redaction;
    pub use crate::traits;
    pub use crate::credentials;
    pub use crate::ollama;
    pub use crate::opencode;
}

#[path = "../src/service/writing_persistence.rs"]
pub mod writing_persistence;
#[path = "../src/service/writing_generation.rs"]
pub mod writing_generation;

pub mod service {
    pub use crate::writing_generation;
    pub use crate::writing_persistence;
}

#[path = "../src/ipc/dto.rs"]
pub mod dto;
#[path = "../src/ipc/writing_persistence_commands.rs"]
pub mod writing_persistence_commands;
#[path = "../src/ipc/writing_generation_commands.rs"]
pub mod writing_generation_commands;
#[path = "../src/ipc/config_commands.rs"]
pub mod config_commands;

pub mod ipc {
    pub use crate::config_commands;
    pub use crate::dto;
    pub use crate::writing_generation_commands;
    pub use crate::writing_persistence_commands;
}

use std::path::PathBuf;
use std::process::Command;
use std::sync::Arc;
use std::time::Duration;
use chrono::Utc;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{params, Connection};
use tempfile::TempDir;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

use error::ErrorCode;
use ipc::dto::*;
use ipc::writing_generation_commands::*;
use providers::traits::{
    LlmProvider, ProviderChatRequest, ProviderChatResponse, ProviderError, ProviderStreamChunk,
    ProviderStreamRequest, ProviderStreamSummary,
};
use service::writing_generation::{RequestEditScope, WritingGenerationService};
use service::writing_persistence::{CallerContext, WritingPersistenceService};
use storage::db::configure_connection;
use storage::migration::run_migrations;

// ============================================================================
// Test Fixture Setup Helpers
// ============================================================================

fn setup_test_db(temp_dir: &TempDir) -> (PathBuf, Pool<SqliteConnectionManager>) {
    let db_path = temp_dir.path().join("test_merlink_p4s5.sqlite3");
    {
        let mut conn = Connection::open(&db_path).expect("open test sqlite db");
        configure_connection(&conn).expect("configure sqlite pragmas");
        run_migrations(&mut conn).expect("run sqlite migrations");
    }

    let manager = SqliteConnectionManager::file(&db_path).with_init(|c| configure_connection(c));
    let pool = Pool::builder()
        .max_size(8)
        .build(manager)
        .expect("build connection pool");

    (db_path, pool)
}

/// Seeds a test project, arc, chapter, and scenes.
fn seed_test_writing_project(
    conn: &Connection,
    is_committed: bool,
) -> (Uuid, Uuid, Vec<Uuid>) {
    let project_id = Uuid::now_v7();
    let arc_id = Uuid::now_v7();
    let chapter_id = Uuid::now_v7();
    let ws_id = Uuid::now_v7();
    let session_id = format!("sess_{}", Uuid::now_v7().simple());
    let draft_id = Uuid::now_v7();

    let now = Utc::now().to_rfc3339();
    let status = if is_committed { "DRAFTING" } else { "PLANNING" };

    conn.execute(
        "INSERT INTO projects (id, code, display_name, summary, status, language, created_at, updated_at)
         VALUES (?1, ?2, 'Thần Đạo Đan Tôn', 'Hành trình luyện đan tu tiên', ?3, 'VI', ?4, ?4)",
        params![project_id.to_string(), format!("p_{}", project_id.simple()), status, now],
    )
    .expect("seed project");

    conn.execute(
        "INSERT INTO sessions (id, project_id, display_name) VALUES (?1, ?2, 'Active Session')",
        params![session_id, project_id.to_string()],
    )
    .expect("seed session");

    let draft_status = if is_committed { "committed" } else { "editing" };
    conn.execute(
        "INSERT INTO draft_sessions (id, session_id, project_id, status, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?5)",
        params![draft_id.to_string(), session_id, project_id.to_string(), draft_status, now],
    )
    .expect("seed draft_session");

    conn.execute(
        "INSERT INTO world_settings (id, code, project_id, display_name, type)
         VALUES (?1, ?2, ?3, 'Cổ Giới Đan Tông', 'REALM')",
        params![ws_id.to_string(), format!("ws_{}", ws_id.simple()), project_id.to_string()],
    )
    .expect("seed world_settings");

    conn.execute(
        "INSERT INTO arcs (id, code, display_name, project_id, position)
         VALUES (?1, ?2, 'Quyển 1: Xuất Thế', ?3, 1)",
        params![arc_id.to_string(), format!("arc_{}", arc_id.simple()), project_id.to_string()],
    )
    .expect("seed arc");

    conn.execute(
        "INSERT INTO chapters (id, code, display_name, arc_id, position, summary, created_at, updated_at)
         VALUES (?1, ?2, 'Chương 1: Trùng Sinh', ?3, 1, 'Lăng Hàn trọng sinh tại Thương Lãng Đại Lục', ?4, ?4)",
        params![chapter_id.to_string(), format!("ch_{}", chapter_id.simple()), arc_id.to_string(), now],
    )
    .expect("seed chapter");

    let mut scene_ids = Vec::new();
    for i in 1..=3 {
        let s_id = Uuid::now_v7();
        scene_ids.push(s_id);
        conn.execute(
            "INSERT INTO scenes (id, code, display_name, chapter_id, world_setting_id, summary, content, word_count, position, is_locked)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, NULL, 0, ?7, 0)",
            params![
                s_id.to_string(),
                format!("scn_{}", s_id.simple()),
                format!("Cảnh {}: Khởi đầu", i),
                chapter_id.to_string(),
                ws_id.to_string(),
                format!("Tóm tắt cảnh {}", i),
                i,
            ],
        )
        .expect("seed scene");
    }

    (project_id, chapter_id, scene_ids)
}

// ============================================================================
// Mock LLM Provider
// ============================================================================

#[derive(Clone)]
struct MockLlmProvider {
    chunks: Vec<String>,
    chunk_delay_ms: u64,
    error_to_inject: Option<String>,
}

impl MockLlmProvider {
    fn new(chunks: Vec<String>) -> Self {
        Self {
            chunks,
            chunk_delay_ms: 0,
            error_to_inject: None,
        }
    }

    fn with_delay(chunks: Vec<String>, delay_ms: u64) -> Self {
        Self {
            chunks,
            chunk_delay_ms: delay_ms,
            error_to_inject: None,
        }
    }

    fn with_error(error_type: &str) -> Self {
        Self {
            chunks: vec![],
            chunk_delay_ms: 0,
            error_to_inject: Some(error_type.to_string()),
        }
    }
}

#[async_trait::async_trait]
impl LlmProvider for MockLlmProvider {
    fn provider_id(&self) -> &'static str {
        "mock_writing_provider"
    }

    async fn chat_with_tools(
        &self,
        _req: &ProviderChatRequest,
        _cancel_token: CancellationToken,
    ) -> Result<ProviderChatResponse, ProviderError> {
        Ok(ProviderChatResponse::default())
    }

    async fn stream_completion(
        &self,
        _req: &ProviderStreamRequest,
        event_tx: mpsc::Sender<ProviderStreamChunk>,
        cancel_token: CancellationToken,
    ) -> Result<ProviderStreamSummary, ProviderError> {
        if let Some(ref err_type) = self.error_to_inject {
            match err_type.as_str() {
                "unreachable" => {
                    return Err(ProviderError::Unreachable {
                        status: Some(503),
                        message: "Connection to model provider lost (timeout/refused)".to_string(),
                    });
                }
                "auth" => {
                    return Err(ProviderError::AuthFailed {
                        status: 401,
                        message: "Invalid API credentials".to_string(),
                    });
                }
                _ => {
                    return Err(ProviderError::Internal("Simulated internal error".to_string()));
                }
            }
        }

        let mut total_text = String::new();
        let mut chunk_count = 0;

        for chunk in &self.chunks {
            if cancel_token.is_cancelled() {
                return Err(ProviderError::Cancelled);
            }
            if self.chunk_delay_ms > 0 {
                tokio::time::sleep(Duration::from_millis(self.chunk_delay_ms)).await;
            }
            if cancel_token.is_cancelled() {
                return Err(ProviderError::Cancelled);
            }
            if event_tx
                .send(ProviderStreamChunk {
                    delta: chunk.clone(),
                    finish_reason: None,
                })
                .await
                .is_err()
            {
                // Receiver dropped
                break;
            }
            total_text.push_str(chunk);
            chunk_count += 1;
        }

        Ok(ProviderStreamSummary {
            total_text,
            chunk_count,
            finish_reason: Some("stop".to_string()),
            usage: None,
        })
    }
}

// ============================================================================
// Acceptance Tests
// ============================================================================

/// `phase4_sprint5_deep2_F0`: Predecessor gate verification.
#[test]
fn test_phase4_sprint5_deep2_F0_predecessor_gates() {
    let baseline = "5af25ef9e3a69ddd3479cccd04e6903ff7881ade";
    let p1 = "e8c1e32b3c0badc47bffba73936b8998ed2b2fb1"; // gate_phase4_sprint4_deep1
    let p2 = "48a9a74a912a37e000b4ce2824385b963c794634"; // gate_phase4_sprint3_deep1
    let p3 = "6a271099f0ea8bf5847c60af81e6d9b5fb9ad918"; // gate_phase2_sprint4_deep3

    let verify_commit = |sha: &str| {
        let status = Command::new("git")
            .args(["merge-base", "--is-ancestor", sha, "HEAD"])
            .status();

        match status {
            Ok(s) if s.success() => true,
            _ => {
                let cat_status = Command::new("git")
                    .args(["cat-file", "-e", &format!("{}^{{commit}}", sha)])
                    .status();
                cat_status.map(|s| s.success()).unwrap_or(false)
            }
        }
    };

    assert!(verify_commit(baseline), "Baseline commit ({}) must exist", baseline);
    assert!(verify_commit(p1), "Predecessor gate_phase4_sprint4_deep1 ({}) must exist", p1);
    assert!(verify_commit(p2), "Predecessor gate_phase4_sprint3_deep1 ({}) must exist", p2);
    assert!(verify_commit(p3), "Predecessor gate_phase2_sprint4_deep3 ({}) must exist", p3);
}

/// `P4S5_P`: started -> delta -> completed carries generationId and valid metadata;
/// DB `scenes.content` and `scene_versions` not written directly.
#[tokio::test]
async fn test_P4S5_P_generation_lifecycle_stream_metadata_and_ram_isolation() {
    let temp_dir = TempDir::new().unwrap();
    let (_path, pool) = setup_test_db(&temp_dir);

    let (project_id, chapter_id, scene_ids) = {
        let conn = pool.get().unwrap();
        seed_test_writing_project(&conn, true)
    };
    let s1 = scene_ids[0];

    let chunks = vec![
        "Lăng Hàn mở mắt ra, ".to_string(),
        "nhìn thấy trần nhà gỗ quen thuộc. ".to_string(),
        "Hắn đã trở lại thời niên thiếu!".to_string(),
    ];
    let provider = Arc::new(MockLlmProvider::new(chunks.clone()));
    let service = WritingGenerationService::with_provider(pool.clone(), provider);

    let (event_tx, mut event_rx) = mpsc::channel(50);

    let req = StartSceneGenerationReq {
        project_id,
        chapter_id,
        scene_id: s1,
        mode: "generate".to_string(),
        note: Some("  Tả cảm giác hoang mang  ".to_string()),
        current_working_content: None,
    };

    let result = service
        .start_scene_generation(&req, Some(event_tx))
        .await
        .expect("start_scene_generation should succeed");

    assert_eq!(result.scene_id, s1);
    assert_eq!(result.baseline_version_number, 0);
    assert_eq!(result.status, GenerationStreamStatus::Completed);
    assert!(result.authorization_valid);
    assert_eq!(
        result.generated_text,
        "Lăng Hàn mở mắt ra, nhìn thấy trần nhà gỗ quen thuộc. Hắn đã trở lại thời niên thiếu!"
    );
    assert_eq!(
        result.normalized_note,
        Some("Tả cảm giác hoang mang".to_string())
    );

    // Collect and verify events
    let mut events = Vec::new();
    while let Ok(evt) = event_rx.try_recv() {
        events.push(evt);
    }

    assert_eq!(events.len(), 5); // Started, 3x Delta, Completed

    // Check Started event
    match &events[0] {
        SceneGenerationStreamEvent::Started {
            request_id,
            generation_id,
            scene_id,
            baseline_version_number,
            authorization_valid,
            status,
        } => {
            assert_eq!(*request_id, result.request_id);
            assert_eq!(*generation_id, result.generation_id);
            assert_eq!(*scene_id, s1);
            assert_eq!(*baseline_version_number, 0);
            assert!(*authorization_valid);
            assert_eq!(*status, GenerationStreamStatus::Streaming);
        }
        _ => panic!("Expected Started event at index 0"),
    }

    // Check Deltas
    for i in 1..=3 {
        match &events[i] {
            SceneGenerationStreamEvent::Delta {
                request_id,
                generation_id,
                scene_id,
                text_delta,
                authorization_valid,
            } => {
                assert_eq!(*request_id, result.request_id);
                assert_eq!(*generation_id, result.generation_id);
                assert_eq!(*scene_id, s1);
                assert_eq!(text_delta, &chunks[i - 1]);
                assert!(*authorization_valid);
            }
            _ => panic!("Expected Delta event at index {}", i),
        }
    }

    // Check Completed event
    match &events[4] {
        SceneGenerationStreamEvent::Completed {
            request_id,
            generation_id,
            scene_id,
            baseline_version_number,
            full_text,
            authorization_valid,
            status,
        } => {
            assert_eq!(*request_id, result.request_id);
            assert_eq!(*generation_id, result.generation_id);
            assert_eq!(*scene_id, s1);
            assert_eq!(*baseline_version_number, 0);
            assert_eq!(full_text, &result.generated_text);
            assert!(*authorization_valid);
            assert_eq!(*status, GenerationStreamStatus::Completed);
        }
        _ => panic!("Expected Completed event at index 4"),
    }

    // Verify In-Memory Buffer
    let buffer = service
        .get_buffer(chapter_id, result.generation_id)
        .expect("Buffer must exist in RAM");
    assert_eq!(buffer.status, GenerationStreamStatus::Completed);
    assert!(buffer.authorization_valid);
    assert_eq!(buffer.accumulated_text, result.generated_text);

    // CRITICAL: DB Isolation assertion - Zero writes to SQLite scenes.content or scene_versions!
    let conn = pool.get().unwrap();
    let content: Option<String> = conn
        .query_row(
            "SELECT content FROM scenes WHERE id = ?1",
            params![s1.to_string()],
            |r| r.get(0),
        )
        .unwrap();
    assert!(content.is_none(), "scenes.content must NOT be written during generation");

    let version_count: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM scene_versions WHERE parent_id = ?1",
            params![s1.to_string()],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(version_count, 0, "scene_versions must NOT be created during generation");
}

/// `P4S5_N`: Lock midstream emits `invalidated_by_lock`, Unlock never revives old result.
#[tokio::test]
async fn test_P4S5_N_lock_midstream_invalidates_permanently() {
    let temp_dir = TempDir::new().unwrap();
    let (_path, pool) = setup_test_db(&temp_dir);

    let (project_id, chapter_id, scene_ids) = {
        let conn = pool.get().unwrap();
        seed_test_writing_project(&conn, true)
    };
    let s1 = scene_ids[0];

    // Delayed chunks to allow lock intervention midstream
    let chunks = vec![
        "Cảnh mở đầu chậm...".to_string(),
        "Nhịp tim đập dồn dập...".to_string(),
        "Nguy hiểm đang tới gần...".to_string(),
        "Không thể thoát khỏi định mệnh...".to_string(),
    ];
    let provider = Arc::new(MockLlmProvider::with_delay(chunks, 30));
    let service = Arc::new(WritingGenerationService::with_provider(pool.clone(), provider));
    let persistence = WritingPersistenceService::new(pool.clone());

    let (event_tx, mut event_rx) = mpsc::channel(50);

    let req = StartSceneGenerationReq {
        project_id,
        chapter_id,
        scene_id: s1,
        mode: "generate".to_string(),
        note: None,
        current_working_content: None,
    };

    let svc_clone = service.clone();
    let join_handle = tokio::spawn(async move {
        svc_clone.start_scene_generation(&req, Some(event_tx)).await
    });

    // Wait until generation is streaming
    tokio::time::sleep(Duration::from_millis(40)).await;

    // Trigger Lock midstream via persistence service (TX-06)
    let lock_res = persistence.set_scene_lock(
        &SetSceneLockReq {
            project_id,
            chapter_id,
            scene_id: s1,
            desired_locked: true,
        },
        CallerContext::UiAction,
    );
    assert!(lock_res.is_ok(), "Lock must succeed");

    // Also call programmatic hook if stream didn't catch interval yet
    service.invalidate_by_lock(s1);

    let gen_res = join_handle.await.unwrap().expect("Generation should complete with invalidated result");
    assert_eq!(gen_res.status, GenerationStreamStatus::InvalidatedByLock);
    assert!(!gen_res.authorization_valid, "authorization_valid must be false on lock");

    // Verify events on channel contain InvalidatedByLock
    let mut found_invalidated_event = false;
    while let Ok(evt) = event_rx.try_recv() {
        if let SceneGenerationStreamEvent::InvalidatedByLock {
            generation_id,
            authorization_valid,
            status,
            ..
        } = evt
        {
            assert_eq!(generation_id, gen_res.generation_id);
            assert!(!authorization_valid);
            assert_eq!(status, GenerationStreamStatus::InvalidatedByLock);
            found_invalidated_event = true;
        }
    }
    assert!(found_invalidated_event, "InvalidatedByLock event must be emitted");

    // Unlock scene 1
    let unlock_res = persistence.set_scene_lock(
        &SetSceneLockReq {
            project_id,
            chapter_id,
            scene_id: s1,
            desired_locked: false,
        },
        CallerContext::UiAction,
    );
    assert!(unlock_res.is_ok());

    // CRITICAL: Now scene 1 is unlocked, but attempting to Accept the invalidated generation MUST fail permanently!
    let accept_res = service.accept_generation(chapter_id, gen_res.generation_id);
    assert!(
        accept_res.is_err(),
        "Accept on generation invalidated by lock must be permanently rejected even after unlock"
    );
    assert_eq!(
        accept_res.unwrap_err().code,
        ErrorCode::GenerationInvalidatedByLock
    );

    // Persisted content still clean
    let conn = pool.get().unwrap();
    let content: Option<String> = conn
        .query_row(
            "SELECT content FROM scenes WHERE id = ?1",
            params![s1.to_string()],
            |r| r.get(0),
        )
        .unwrap();
    assert!(content.is_none());
}

/// `P4S5_B`: Cancel generation leaves persisted content unchanged.
#[tokio::test]
async fn test_P4S5_B_cancel_generation_leaves_persisted_content_unchanged() {
    let temp_dir = TempDir::new().unwrap();
    let (_path, pool) = setup_test_db(&temp_dir);

    let (project_id, chapter_id, scene_ids) = {
        let conn = pool.get().unwrap();
        seed_test_writing_project(&conn, true)
    };
    let s1 = scene_ids[0];

    let chunks = vec![
        "Đoạn văn 1...".to_string(),
        "Đoạn văn 2...".to_string(),
        "Đoạn văn 3...".to_string(),
    ];
    let provider = Arc::new(MockLlmProvider::with_delay(chunks, 30));
    let service = Arc::new(WritingGenerationService::with_provider(pool.clone(), provider));

    let (event_tx, mut event_rx) = mpsc::channel(50);

    let req = StartSceneGenerationReq {
        project_id,
        chapter_id,
        scene_id: s1,
        mode: "generate".to_string(),
        note: None,
        current_working_content: None,
    };

    let svc_clone = service.clone();
    let join_handle = tokio::spawn(async move {
        svc_clone.start_scene_generation(&req, Some(event_tx)).await
    });

    // Wait until started
    tokio::time::sleep(Duration::from_millis(30)).await;

    // Get active generation_id
    let gen_id = service
        .get_active_generation_id(chapter_id)
        .expect("active generation should exist");

    // Cancel generation
    let cancel_res = service
        .cancel_scene_generation(&CancelSceneGenerationReq {
            chapter_id,
            generation_id: gen_id,
        })
        .await
        .expect("Cancel must succeed");
    assert_eq!(cancel_res.status, "cancelled");

    let gen_result = join_handle.await.unwrap().expect("Generation returns cancelled result");
    assert_eq!(gen_result.status, GenerationStreamStatus::Cancelled);

    // Channel received Cancelled event
    let mut found_cancelled_event = false;
    while let Ok(evt) = event_rx.try_recv() {
        if let SceneGenerationStreamEvent::Cancelled {
            generation_id,
            status,
            ..
        } = evt
        {
            assert_eq!(generation_id, gen_id);
            assert_eq!(status, GenerationStreamStatus::Cancelled);
            found_cancelled_event = true;
        }
    }
    assert!(found_cancelled_event, "Cancelled event must be emitted");

    // Attempting to Accept cancelled generation fails
    let accept_res = service.accept_generation(chapter_id, gen_id);
    assert!(accept_res.is_err());
    assert_eq!(accept_res.unwrap_err().code, ErrorCode::ValidationFailed);

    // DB remains unchanged
    let conn = pool.get().unwrap();
    let content: Option<String> = conn
        .query_row(
            "SELECT content FROM scenes WHERE id = ?1",
            params![s1.to_string()],
            |r| r.get(0),
        )
        .unwrap();
    assert!(content.is_none());
}

/// `phase4_sprint5_deep2_F1`: Provider error/disconnect emits failed event,
/// partial buffer cannot be accepted.
#[tokio::test]
async fn test_phase4_sprint5_deep2_F1_provider_error_emits_failed_event() {
    let temp_dir = TempDir::new().unwrap();
    let (_path, pool) = setup_test_db(&temp_dir);

    let (project_id, chapter_id, scene_ids) = {
        let conn = pool.get().unwrap();
        seed_test_writing_project(&conn, true)
    };
    let s1 = scene_ids[0];

    let provider = Arc::new(MockLlmProvider::with_error("unreachable"));
    let service = WritingGenerationService::with_provider(pool.clone(), provider);

    let (event_tx, mut event_rx) = mpsc::channel(50);

    let req = StartSceneGenerationReq {
        project_id,
        chapter_id,
        scene_id: s1,
        mode: "generate".to_string(),
        note: None,
        current_working_content: None,
    };

    let result = service.start_scene_generation(&req, Some(event_tx)).await;
    assert!(result.is_err(), "Generation must fail with provider error");
    let err = result.unwrap_err();
    assert_eq!(err.code, ErrorCode::ProviderUnreachable);

    // Check failed event on channel
    let mut found_failed_event = false;
    let mut failed_gen_id = None;
    while let Ok(evt) = event_rx.try_recv() {
        if let SceneGenerationStreamEvent::Failed {
            generation_id,
            error,
            status,
            authorization_valid,
            ..
        } = evt
        {
            assert_eq!(error.code, ErrorCode::ProviderUnreachable);
            assert_eq!(status, GenerationStreamStatus::Failed);
            assert!(!authorization_valid);
            found_failed_event = true;
            failed_gen_id = Some(generation_id);
        }
    }
    assert!(found_failed_event, "Failed event must be emitted");

    // Acceptance of failed generation is rejected
    if let Some(gid) = failed_gen_id {
        let accept_res = service.accept_generation(chapter_id, gid);
        assert!(accept_res.is_err());
        assert_eq!(accept_res.unwrap_err().code, ErrorCode::ValidationFailed);
    }
}

/// `phase4_sprint5_deep2_F2`: Out-of-scope or locked scene rejected before calling provider.
#[tokio::test]
async fn test_phase4_sprint5_deep2_F2_outofscope_or_locked_rejected_before_provider() {
    let temp_dir = TempDir::new().unwrap();
    let (_path, pool) = setup_test_db(&temp_dir);

    let (project_id, chapter_id, scene_ids) = {
        let conn = pool.get().unwrap();
        seed_test_writing_project(&conn, true)
    };
    let s1 = scene_ids[0];
    let s2 = scene_ids[1];

    let provider = Arc::new(MockLlmProvider::new(vec!["text".to_string()]));
    let service = WritingGenerationService::with_provider(pool.clone(), provider);

    // 1. Scene 1 locked in SQLite
    {
        let conn = pool.get().unwrap();
        conn.execute(
            "UPDATE scenes SET is_locked = 1 WHERE id = ?1",
            params![s1.to_string()],
        )
        .unwrap();
    }

    let (event_tx, mut event_rx) = mpsc::channel(50);
    let req_locked = StartSceneGenerationReq {
        project_id,
        chapter_id,
        scene_id: s1,
        mode: "generate".to_string(),
        note: None,
        current_working_content: None,
    };

    let res_locked = service
        .start_scene_generation(&req_locked, Some(event_tx))
        .await;
    assert!(res_locked.is_err());
    assert_eq!(res_locked.unwrap_err().code, ErrorCode::SceneLocked);
    assert!(
        event_rx.try_recv().is_err(),
        "No events should be emitted if rejected before provider"
    );

    // 2. Establish EditScope that only allows scene 1
    service.register_edit_scope(RequestEditScope {
        request_id: Uuid::now_v7(),
        chapter_id,
        allowed_scene_ids: std::collections::BTreeSet::from([s1]),
        is_active: true,
    });

    let (event_tx2, mut event_rx2) = mpsc::channel(50);
    let req_out_of_scope = StartSceneGenerationReq {
        project_id,
        chapter_id,
        scene_id: s2,
        mode: "generate".to_string(),
        note: None,
        current_working_content: None,
    };

    let res_oos = service
        .start_scene_generation(&req_out_of_scope, Some(event_tx2))
        .await;
    assert!(res_oos.is_err());
    assert_eq!(res_oos.unwrap_err().code, ErrorCode::OutOfEditScope);
    assert!(
        event_rx2.try_recv().is_err(),
        "No events emitted when out of edit scope"
    );
}

/// `T_LOCK_01`: Scene Lock & EditScope comprehensive matrix.
/// - Case A: Stream -> Lock midstream -> Unlock -> Accept blocked forever (no scene_versions created by lock).
/// - Case C/D: Stream completed on Scene 2 -> EditScope expires -> Accept & SaveChapter succeeds.
/// - Case G/H: Generation on out-of-scope or locked scene blocked by OutOfEditScope / SceneLocked.
#[tokio::test]
async fn test_T_LOCK_01_comprehensive_matrix() {
    let temp_dir = TempDir::new().unwrap();
    let (_path, pool) = setup_test_db(&temp_dir);

    let (project_id, chapter_id, scene_ids) = {
        let conn = pool.get().unwrap();
        seed_test_writing_project(&conn, true)
    };
    let s1 = scene_ids[0];
    let s2 = scene_ids[1];
    let s3 = scene_ids[2];

    let provider = Arc::new(MockLlmProvider::with_delay(
        vec![
            "Chunk 1: Khởi nguồn... ".to_string(),
            "Chunk 2: Phát triển... ".to_string(),
            "Chunk 3: Cao trào... ".to_string(),
        ],
        25,
    ));
    let gen_service = Arc::new(WritingGenerationService::with_provider(pool.clone(), provider));
    let persist_service = WritingPersistenceService::new(pool.clone());

    // ------------------------------------------------------------------------
    // Case A: Stream on Scene 1 -> Lock(true) midstream -> Unlock(false) -> Accept fails
    // ------------------------------------------------------------------------
    let (tx1, _rx1) = mpsc::channel(50);
    let req1 = StartSceneGenerationReq {
        project_id,
        chapter_id,
        scene_id: s1,
        mode: "generate".to_string(),
        note: None,
        current_working_content: None,
    };

    let svc1 = gen_service.clone();
    let handle1 = tokio::spawn(async move {
        svc1.start_scene_generation(&req1, Some(tx1)).await
    });

    tokio::time::sleep(Duration::from_millis(35)).await;

    // User locks Scene 1 via TX-06
    let lock_s1 = persist_service.set_scene_lock(
        &SetSceneLockReq {
            project_id,
            chapter_id,
            scene_id: s1,
            desired_locked: true,
        },
        CallerContext::UiAction,
    );
    assert!(lock_s1.is_ok());
    gen_service.invalidate_by_lock(s1);

    let res1 = handle1.await.unwrap().expect("should return invalidated result");
    assert_eq!(res1.status, GenerationStreamStatus::InvalidatedByLock);
    assert!(!res1.authorization_valid);

    // Verify scenes.is_locked persisted in SQLite WITHOUT creating scene_versions
    {
        let conn = pool.get().unwrap();
        let is_locked: i32 = conn
            .query_row("SELECT is_locked FROM scenes WHERE id = ?1", params![s1.to_string()], |r| r.get(0))
            .unwrap();
        assert_eq!(is_locked, 1);
        let versions: i64 = conn
            .query_row("SELECT COUNT(*) FROM scene_versions WHERE parent_id = ?1", params![s1.to_string()], |r| r.get(0))
            .unwrap();
        assert_eq!(versions, 0, "Lock toggle must not create scene_versions");
    }

    // User unlocks Scene 1
    persist_service
        .set_scene_lock(
            &SetSceneLockReq {
                project_id,
                chapter_id,
                scene_id: s1,
                desired_locked: false,
            },
            CallerContext::UiAction,
        )
        .expect("Unlock must succeed");

    // Attempting to Accept the generation that was invalidated by lock MUST be rejected permanently!
    let accept_res1 = gen_service.accept_generation(chapter_id, res1.generation_id);
    assert!(accept_res1.is_err());
    assert_eq!(accept_res1.unwrap_err().code, ErrorCode::GenerationInvalidatedByLock);

    // ------------------------------------------------------------------------
    // Case C/D: Stream completes on Scene 2 -> Scope expires -> Accept & SaveChapter
    // ------------------------------------------------------------------------
    let fast_provider = Arc::new(MockLlmProvider::new(vec![
        "Nội dung hoàn chỉnh Cảnh 2 từ AI.".to_string(),
    ]));
    gen_service.set_provider(fast_provider);

    let req2 = StartSceneGenerationReq {
        project_id,
        chapter_id,
        scene_id: s2,
        mode: "generate".to_string(),
        note: None,
        current_working_content: None,
    };

    let res2 = gen_service
        .start_scene_generation(&req2, None)
        .await
        .expect("Generation on scene 2 should complete");
    assert_eq!(res2.status, GenerationStreamStatus::Completed);
    assert!(res2.authorization_valid);

    // Verify EditScope is no longer active for new AI requests
    let scope2 = gen_service.get_edit_scope(chapter_id).unwrap();
    assert!(!scope2.is_active, "Scope expires after AI request terminates (§8.3.2)");

    // Accept succeeds despite expired EditScope (SCOPE-05)
    let accepted_text = gen_service
        .accept_generation(chapter_id, res2.generation_id)
        .expect("Accept must succeed for valid completed generation even after scope expiry");
    assert_eq!(accepted_text, "Nội dung hoàn chỉnh Cảnh 2 từ AI.");

    // User executes save_chapter (TX-07)
    let save_req = SaveChapterRequest {
        project_id,
        chapter_id,
        client_save_id: format!("save_{}", Uuid::now_v7().simple()),
        dirty_scenes: vec![DirtySceneSaveItem {
            scene_id: s2,
            expected_version_number: 0,
            content: accepted_text.clone(),
            scene_source: "AI".to_string(),
            summary: Some("Tóm tắt mới sau khi AI sinh".to_string()),
        }],
    };

    let save_resp = persist_service
        .save_chapter(&save_req)
        .expect("save_chapter must succeed");
    assert_eq!(save_resp.saved_scenes.len(), 1);
    assert_eq!(save_resp.saved_scenes[0].scene_id, s2);
    assert_eq!(save_resp.saved_scenes[0].new_version_number, 1);

    // Verify in SQLite
    {
        let conn = pool.get().unwrap();
        let content: String = conn
            .query_row("SELECT content FROM scenes WHERE id = ?1", params![s2.to_string()], |r| r.get(0))
            .unwrap();
        assert_eq!(content, accepted_text);
    }

    // ------------------------------------------------------------------------
    // Case G: Generation on scene outside active EditScope is blocked
    // ------------------------------------------------------------------------
    gen_service.register_edit_scope(RequestEditScope {
        request_id: Uuid::now_v7(),
        chapter_id,
        allowed_scene_ids: std::collections::BTreeSet::from([s1]),
        is_active: true,
    });

    let req_case_g = StartSceneGenerationReq {
        project_id,
        chapter_id,
        scene_id: s3, // Outside EditScope
        mode: "generate".to_string(),
        note: None,
        current_working_content: None,
    };

    let res_g = gen_service.start_scene_generation(&req_case_g, None).await;
    assert!(res_g.is_err());
    assert_eq!(res_g.unwrap_err().code, ErrorCode::OutOfEditScope);

    // ------------------------------------------------------------------------
    // Case H: Generation on locked scene is blocked
    // ------------------------------------------------------------------------
    persist_service
        .set_scene_lock(
            &SetSceneLockReq {
                project_id,
                chapter_id,
                scene_id: s3,
                desired_locked: true,
            },
            CallerContext::UiAction,
        )
        .expect("Lock scene 3 must succeed");

    let req_case_h = StartSceneGenerationReq {
        project_id,
        chapter_id,
        scene_id: s3, // Locked scene
        mode: "generate".to_string(),
        note: None,
        current_working_content: None,
    };

    let res_h = gen_service.start_scene_generation(&req_case_h, None).await;
    assert!(res_h.is_err());
    assert_eq!(res_h.unwrap_err().code, ErrorCode::SceneLocked);
}

/// `IPC_CMD_032`: Contract test for IPC #32 `start_scene_generation`.
#[tokio::test]
async fn test_IPC_CMD_032_start_scene_generation() {
    let temp_dir = TempDir::new().unwrap();
    let (_path, pool) = setup_test_db(&temp_dir);

    let (project_id, chapter_id, scene_ids) = {
        let conn = pool.get().unwrap();
        seed_test_writing_project(&conn, true)
    };
    let s1 = scene_ids[0];

    let provider = Arc::new(MockLlmProvider::new(vec!["Nội dung IPC-32".to_string()]));
    let service = WritingGenerationService::with_provider(pool.clone(), provider);

    // 1. Valid execution via exec_start_scene_generation
    let (tx, mut rx) = mpsc::channel(20);
    let req = StartSceneGenerationReq {
        project_id,
        chapter_id,
        scene_id: s1,
        mode: "generate".to_string(),
        note: Some("  Viết mạch lạc, tự nhiên  ".to_string()),
        current_working_content: Some("Bối cảnh cũ".to_string()),
    };

    let res = exec_start_scene_generation(&service, &req, Some(tx))
        .await
        .expect("exec_start_scene_generation must succeed");
    assert_eq!(res.generated_text, "Nội dung IPC-32");
    assert_eq!(res.status, GenerationStreamStatus::Completed);
    assert_eq!(
        res.normalized_note,
        Some("Viết mạch lạc, tự nhiên".to_string())
    );

    // Verify events were delivered
    let mut event_count = 0;
    while let Ok(_) = rx.try_recv() {
        event_count += 1;
    }
    assert!(event_count >= 2); // Started + Completed

    // 2. Note trimming & empty note normalization
    let req_empty_note = StartSceneGenerationReq {
        project_id,
        chapter_id,
        scene_id: s1,
        mode: "regenerate".to_string(),
        note: Some("     \t \n   ".to_string()),
        current_working_content: None,
    };
    let res_empty_note = exec_start_scene_generation(&service, &req_empty_note, None)
        .await
        .unwrap();
    assert_eq!(res_empty_note.normalized_note, None);

    // 3. Note exceeding 2000 characters rejected with ValidationFailed
    let long_note = "a".repeat(2001);
    let req_long_note = StartSceneGenerationReq {
        project_id,
        chapter_id,
        scene_id: s1,
        mode: "generate".to_string(),
        note: Some(long_note),
        current_working_content: None,
    };
    let err_long = exec_start_scene_generation(&service, &req_long_note, None).await;
    assert!(err_long.is_err());
    assert_eq!(err_long.unwrap_err().code, ErrorCode::ValidationFailed);

    // 4. Invalid mode rejected with ValidationFailed
    let req_invalid_mode = StartSceneGenerationReq {
        project_id,
        chapter_id,
        scene_id: s1,
        mode: "invalid_mode".to_string(),
        note: None,
        current_working_content: None,
    };
    let err_mode = exec_start_scene_generation(&service, &req_invalid_mode, None).await;
    assert!(err_mode.is_err());
    assert_eq!(err_mode.unwrap_err().code, ErrorCode::ValidationFailed);

    // 5. Max 1 active generation per chapter
    let delayed_provider = Arc::new(MockLlmProvider::with_delay(
        vec!["Chậm...".to_string()],
        100,
    ));
    service.set_provider(delayed_provider);

    let svc_ref = Arc::new(service);
    let svc_clone = svc_ref.clone();
    let req_dup = StartSceneGenerationReq {
        project_id,
        chapter_id,
        scene_id: s1,
        mode: "generate".to_string(),
        note: None,
        current_working_content: None,
    };

    let req_dup_clone = req_dup.clone();
    let bg_gen = tokio::spawn(async move {
        exec_start_scene_generation(&svc_clone, &req_dup_clone, None).await
    });

    tokio::time::sleep(Duration::from_millis(30)).await;

    // Second call on same chapter while first is streaming MUST fail
    let second_call = exec_start_scene_generation(&svc_ref, &req_dup, None).await;
    assert!(second_call.is_err());
    assert_eq!(second_call.unwrap_err().code, ErrorCode::ValidationFailed);

    let _ = bg_gen.await;
}

/// `IPC_CMD_033`: Contract test for IPC #33 `cancel_scene_generation`.
#[tokio::test]
async fn test_IPC_CMD_033_cancel_scene_generation() {
    let temp_dir = TempDir::new().unwrap();
    let (_path, pool) = setup_test_db(&temp_dir);

    let (project_id, chapter_id, scene_ids) = {
        let conn = pool.get().unwrap();
        seed_test_writing_project(&conn, true)
    };
    let s1 = scene_ids[0];

    let provider = Arc::new(MockLlmProvider::with_delay(
        vec!["Chunk 1...".to_string(), "Chunk 2...".to_string()],
        60,
    ));
    let service = Arc::new(WritingGenerationService::with_provider(pool.clone(), provider));

    let req = StartSceneGenerationReq {
        project_id,
        chapter_id,
        scene_id: s1,
        mode: "generate".to_string(),
        note: None,
        current_working_content: None,
    };

    let svc_clone = service.clone();
    let bg_gen = tokio::spawn(async move {
        exec_start_scene_generation(&svc_clone, &req, None).await
    });

    tokio::time::sleep(Duration::from_millis(25)).await;

    let active_gen_id = service
        .get_active_generation_id(chapter_id)
        .expect("active generation should exist");

    // 1. Cancel in-flight generation
    let cancel_res = exec_cancel_scene_generation(
        &service,
        &CancelSceneGenerationReq {
            chapter_id,
            generation_id: active_gen_id,
        },
    )
    .await
    .expect("Cancel must succeed");
    assert_eq!(cancel_res.generation_id, active_gen_id);
    assert_eq!(cancel_res.status, "cancelled");

    // Wait for task to finish
    let res = bg_gen.await.unwrap().unwrap();
    assert_eq!(res.status, GenerationStreamStatus::Cancelled);

    // 2. Idempotent cancel replay on already cancelled generation
    let replay_cancel = exec_cancel_scene_generation(
        &service,
        &CancelSceneGenerationReq {
            chapter_id,
            generation_id: active_gen_id,
        },
    )
    .await
    .expect("Idempotent cancel replay must succeed");
    assert_eq!(replay_cancel.status, "cancelled");

    // 3. Unknown generation returns EntityNotFound
    let unknown_cancel = exec_cancel_scene_generation(
        &service,
        &CancelSceneGenerationReq {
            chapter_id,
            generation_id: Uuid::now_v7(),
        },
    )
    .await;
    assert!(unknown_cancel.is_err());
    assert_eq!(unknown_cancel.unwrap_err().code, ErrorCode::EntityNotFound);
}

/// `IPC_CMD_034`: Contract test for IPC #34 `respond_edit_scope_expansion`.
#[tokio::test]
async fn test_IPC_CMD_034_respond_edit_scope_expansion() {
    let temp_dir = TempDir::new().unwrap();
    let (_path, pool) = setup_test_db(&temp_dir);

    let (_project_id, chapter_id, scene_ids) = {
        let conn = pool.get().unwrap();
        seed_test_writing_project(&conn, true)
    };
    let s1 = scene_ids[0];
    let s2 = scene_ids[1];
    let s3 = scene_ids[2];

    let service = WritingGenerationService::new(pool.clone());
    let req_id = Uuid::now_v7();

    // Initialize EditScope with s1
    service.register_edit_scope(RequestEditScope {
        request_id: req_id,
        chapter_id,
        allowed_scene_ids: std::collections::BTreeSet::from([s1]),
        is_active: true,
    });

    // 1. User approves expanding to s2
    let expand_s2 = exec_respond_edit_scope_expansion(
        &service,
        &RespondEditScopeExpansionReq {
            chapter_id,
            request_id: req_id,
            target_scene_id: s2,
            approved: true,
        },
    )
    .await
    .expect("Scope expansion must succeed");
    assert_eq!(expand_s2.request_id, req_id);
    assert!(expand_s2.allowed_scene_ids.contains(&s1));
    assert!(expand_s2.allowed_scene_ids.contains(&s2));
    assert!(!expand_s2.target_scene_is_locked);

    // 2. User rejects expanding to s3
    let expand_s3_rejected = exec_respond_edit_scope_expansion(
        &service,
        &RespondEditScopeExpansionReq {
            chapter_id,
            request_id: req_id,
            target_scene_id: s3,
            approved: false,
        },
    )
    .await
    .expect("Scope response must succeed");
    assert!(!expand_s3_rejected.allowed_scene_ids.contains(&s3));

    // 3. Target scene is locked in SQLite: approved expansion includes it in scope,
    // but target_scene_is_locked is true and SQLite is_locked remains 1 (SL-10)
    {
        let conn = pool.get().unwrap();
        conn.execute(
            "UPDATE scenes SET is_locked = 1 WHERE id = ?1",
            params![s3.to_string()],
        )
        .unwrap();
    }

    let expand_s3_locked = exec_respond_edit_scope_expansion(
        &service,
        &RespondEditScopeExpansionReq {
            chapter_id,
            request_id: req_id,
            target_scene_id: s3,
            approved: true,
        },
    )
    .await
    .expect("Scope response must succeed");
    assert!(expand_s3_locked.allowed_scene_ids.contains(&s3));
    assert!(expand_s3_locked.target_scene_is_locked, "Target scene is locked");

    // In SQLite, s3 is STILL locked (SL-10: no automatic unlock)
    {
        let conn = pool.get().unwrap();
        let is_locked: i32 = conn
            .query_row("SELECT is_locked FROM scenes WHERE id = ?1", params![s3.to_string()], |r| r.get(0))
            .unwrap();
        assert_eq!(is_locked, 1, "Scope expansion must NOT automatically unlock scene (SL-10)");
    }

    // 4. Non-existent scene or request returns EntityNotFound
    let err_scene = exec_respond_edit_scope_expansion(
        &service,
        &RespondEditScopeExpansionReq {
            chapter_id,
            request_id: req_id,
            target_scene_id: Uuid::now_v7(),
            approved: true,
        },
    )
    .await;
    assert!(err_scene.is_err());
    assert_eq!(err_scene.unwrap_err().code, ErrorCode::EntityNotFound);

    let err_req = exec_respond_edit_scope_expansion(
        &service,
        &RespondEditScopeExpansionReq {
            chapter_id,
            request_id: Uuid::now_v7(),
            target_scene_id: s1,
            approved: true,
        },
    )
    .await;
    assert!(err_req.is_err());
    assert_eq!(err_req.unwrap_err().code, ErrorCode::EntityNotFound);
}
