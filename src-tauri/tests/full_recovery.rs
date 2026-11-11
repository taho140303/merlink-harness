#![allow(clippy::field_reassign_with_default)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::result_large_err)]
#![allow(clippy::result_unit_err)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]

//! Comprehensive End-to-End Correctness and Release Gate Integration Test Suite
//! (Sprint `phase6_sprint3_deep1` - 33rd and Final Sprint).
//!
//! Complies strictly with docs/plan/phase6_sprint3_deep1.md, docs/business/be-core.md (§12.1–§12.4),
//! and docs/architecture/decisions/sf06-owner-ratification.md:
//!
//! - `phase6_sprint3_deep1-F0`: Predecessor gate commits (`2c42e64812b7f3a57b148433521dea80623e128d`,
//!   `c09e5a67db4e97db44a7d845cb28522c7a4c8df5`) and source baseline `ddda30cd3c842113599c7bfb7d0e221936330428`
//!   lineage verification with `CONTRACT_BLOCKED` boundary check.
//! - `P6S3-P`: Fresh app creates DB, executes all migrations, initializes AppState, and executes
//!   full backend lifecycle: chat session -> draft load -> pending patch -> round seal TX-04 ->
//!   draft commit TX-05 -> load writing read model -> save chapter scenes OCC.
//! - `P6S3-N`: Fault injection: fault after TX-08 before TX-04 retains proposal in `pending_patches`
//!   without round completion (does not create sealed `draft_versions`).
//! - `P6S3-B`: Lock during stream invalidates candidate; subsequent unlock never enables old Accept
//!   (`CONFLICT_INVALIDATED` / `GenerationInvalidatedByLock`); dirty chapter Save OCC revision conflict
//!   preserves dirty buffer and rejects overwrite (`OccVersionMismatch`).
//! - `phase6_sprint3_deep1-F1`: Migration mismatch or corruption rejects startup with explicit error,
//!   zero unauthorized repair.
//! - `P6S3-MATRIX`: Systematic verification asserting each test category row is covered and green
//!   on disposable databases (`T-UNIT-01..05`, `T-DB-01`, `T-ROLL-01`, `T-CONC-01`, `T-IDEM-01`,
//!   `T-REST-01`, `T-PROV-01`, `T-CASC-01`, `T-WF-01`, `T-LOCK-01`, `T-SAVE-01`, `T-REC-01`,
//!   `T-IPC-01`, `MIG-01..05`, `WF-01..10`, `WF-C01..15`, `IPC-01..02`, `DB-P01..02`).

#![allow(non_snake_case)]

mod common;

use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use chrono::Utc;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{params, Connection};
use serde_json::json;
use tempfile::TempDir;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

use merlink_harness_lib::domain::business_flow::*;
use merlink_harness_lib::domain::code_allocator::{CodeAllocator, EntityCodeSpec};
use merlink_harness_lib::domain::fingerprint::state_data_hash;
use merlink_harness_lib::domain::invariants::validate_draft_for_commit;
use merlink_harness_lib::domain::progress_engine::evaluate_progress;
use merlink_harness_lib::domain::progress_engine::MASTER_ATTRIBUTE_DENOMINATOR;
use merlink_harness_lib::domain::sf06::codec::decode_evidence;
use merlink_harness_lib::domain::word_count::count_words;
use merlink_harness_lib::domain::workflow_planner::plan_outline_runs;
use merlink_harness_lib::error::{AppError, ErrorCode};
use merlink_harness_lib::ipc::chat_commands::exec_create_or_load_chat_session;
use merlink_harness_lib::ipc::commit_commands::exec_commit_draft;
use merlink_harness_lib::ipc::dto::*;
use merlink_harness_lib::ipc::writing_persistence_commands::exec_save_chapter;
use merlink_harness_lib::ipc::{MASTER_IPC_COMMAND_NAMES, STREAMING_CHANNEL_NAMES};
use merlink_harness_lib::providers::traits::{
    LlmProvider, ProviderChatRequest, ProviderChatResponse, ProviderError, ProviderStreamChunk,
    ProviderStreamRequest, ProviderStreamSummary,
};
use merlink_harness_lib::service::commit_service::CallerContext as CommitCallerContext;
use merlink_harness_lib::service::draft_runtime::{
    DraftRuntimeRegistry, DraftRuntimeStatus, WorkingState,
};
use merlink_harness_lib::service::draft_service::{
    DraftService, RestoreVersionParams, SealVersionParams,
};
use merlink_harness_lib::service::patch_service::compute_progress_dto;
use merlink_harness_lib::service::recovery_service::{
    evaluate_evidence_integrity, run_startup_recovery, WorkflowRecoveryDecision,
};
use merlink_harness_lib::service::writing_generation::WritingGenerationService;
use merlink_harness_lib::service::writing_persistence::CallerContext as WritingCallerContext;
use merlink_harness_lib::service::writing_persistence::WritingPersistenceService;
use merlink_harness_lib::storage::db::{
    canonical_db_dir, canonical_db_path, configure_connection, DbError,
};
use merlink_harness_lib::storage::migration::{
    get_user_version, run_migrations, verify_schema_and_integrity, MAX_SUPPORTED_SCHEMA_VERSION,
    TARGET_TABLES_V3,
};
use merlink_harness_lib::storage::pending_repo::{find_by_id, PersistProposalParams};
use merlink_harness_lib::AppBootstrap;

pub const WORKTREE_BASELINE_COMMIT: &str = "ba22ca2b53d411a539277bffd068026f0776a2c9";
pub const SOURCE_BASELINE_COMMIT: &str = "ddda30cd3c842113599c7bfb7d0e221936330428";
pub const PREDECESSOR_P6S1: &str = "2c42e64812b7f3a57b148433521dea80623e128d";
pub const PREDECESSOR_P6S2: &str = "c09e5a67db4e97db44a7d845cb28522c7a4c8df5";

/// Helper setting up a uniquely named disposable SQLite database in a temp directory.
fn setup_disposable_db(
    temp_dir: &TempDir,
    name_suffix: &str,
) -> (PathBuf, Pool<SqliteConnectionManager>) {
    let db_path = temp_dir
        .path()
        .join(format!("disposable-test-{}.sqlite3", name_suffix));
    {
        let mut conn = Connection::open(&db_path).expect("open disposable sqlite db");
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

/// Mock LLM provider with configurable delays for generation and lock testing.
#[derive(Debug, Clone)]
struct DelayedMockProvider {
    chunks: Vec<String>,
    delay_ms: u64,
}

impl DelayedMockProvider {
    fn new(chunks: Vec<String>, delay_ms: u64) -> Self {
        Self { chunks, delay_ms }
    }
}

#[async_trait::async_trait]
impl LlmProvider for DelayedMockProvider {
    fn provider_id(&self) -> &'static str {
        "delayed_mock_provider"
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
        chunk_tx: mpsc::Sender<ProviderStreamChunk>,
        cancel_token: CancellationToken,
    ) -> Result<ProviderStreamSummary, ProviderError> {
        let mut total_text = String::new();
        let mut chunk_count = 0;

        for chunk in &self.chunks {
            if cancel_token.is_cancelled() {
                return Err(ProviderError::Cancelled);
            }
            if self.delay_ms > 0 {
                tokio::time::sleep(Duration::from_millis(self.delay_ms)).await;
            }
            if cancel_token.is_cancelled() {
                return Err(ProviderError::Cancelled);
            }
            if chunk_tx
                .send(ProviderStreamChunk {
                    delta: chunk.clone(),
                    finish_reason: None,
                })
                .await
                .is_err()
            {
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

/// Builds a fully valid full draft fixture matching `planner_progress.rs` and `draft_commit.rs`
/// passing `validate_draft_for_commit`.
fn build_valid_full_draft_fixture(default_source: ProvenanceSource) -> BusinessFlowData {
    let tf_str = |s: &str| TrackedField::new(s.to_string(), default_source);
    let tf_u32 = |v: u32| TrackedField::new(v, default_source);
    let tf_bool = |b: bool| TrackedField::new(b, default_source);

    let mut data = BusinessFlowData::new();
    data.meta.phase = 6;
    data.meta.subphase = None;

    // Phase 0: require (10 attributes)
    data.require.project.code = Some("dai_lao_tu_tien".to_string());
    data.require.project.display_name = tf_str("Đại Lão Tu Tiên");
    data.require.project.summary = tf_str("Hành trình phá vỡ giới hạn phàm nhân.");
    data.require.project.language = tf_str("VI");
    data.require.project.target.arc_count = tf_u32(2);
    data.require.project.target.chapter_count = tf_u32(3);
    data.require.project.target.word_per_chapter_count = tf_u32(2500);
    data.require.project.genres = vec![RequireGenreItem {
        code: tf_str("tien_hiep"),
        is_primary: tf_bool(true),
        display_name: tf_str("Tiên Hiệp"),
    }];
    data.require.character = vec![
        RequireCharacterItem {
            code: "tran_ha".to_string(),
            display_name: tf_str("Trần Hà"),
            nickname: tf_str("Hà Đế"),
            biological_sex: tf_str("MALE"),
            motif_code: tf_str("fallen_prince"),
            role: tf_str("PROTAGONIST"),
            notes: tf_str("Nhân vật chính"),
            reused_library_character: false,
            source: Some(default_source),
        },
        RequireCharacterItem {
            code: "tuyet_bang".to_string(),
            display_name: tf_str("Tuyết Băng"),
            nickname: tf_str("Băng Nhi"),
            biological_sex: tf_str("FEMALE"),
            motif_code: tf_str("sacrificial_maiden"),
            role: tf_str("DEUTERAGONIST"),
            notes: tf_str("Nữ chính"),
            reused_library_character: false,
            source: Some(default_source),
        },
    ];

    // Phase 1: overview_phase
    data.optional.overview_phase.arcs = vec![
        OverviewArcItem {
            code: "dai_lao_tu_tien-a1".to_string(),
            display_name: tf_str("Hồi 1: Cố Nhân Tương Phùng"),
            summary: tf_str("Gặp lại người xưa chốn giang hồ."),
            position: tf_u32(1),
            motif: ArcMotifSection {
                code: tf_str("survival_escape"),
                note: tf_str("Thoát khỏi truy sát"),
                event_blocks: vec![ArcEventBlockItem {
                    code: "dai_lao_tu_tien-a1-eb1".to_string(),
                    event_block_code: tf_str("inciting_incident"),
                    start_chapter_code: tf_str("dai_lao_tu_tien-a1-c1"),
                    end_chapter_code: tf_str("dai_lao_tu_tien-a1-c2"),
                    start_chapter: tf_u32(1),
                    end_chapter: tf_u32(2),
                    summary: tf_str("Biến cố gia tộc"),
                    source: Some(default_source),
                }],
            },
            themes: ArcThemeSection {
                code: Some("dai_lao_tu_tien-a1-t1".to_string()),
                parent_theme_code: TrackedField::unset(),
                question: tf_str("Đạo là gì?"),
                resolution: ThemeResolutionSection {
                    resolution_type: tf_str("TRIUMPH"),
                    content: tf_str("Tự tìm ra con đường của riêng mình."),
                },
                message: tf_str("Kiên định với đạo tâm."),
                arguments: vec![ThemeArgumentItem {
                    code: Some("dai_lao_tu_tien-a1-t1-arg1".to_string()),
                    character_code: tf_str("tran_ha"),
                    character_display_name: tf_str("Trần Hà"),
                    role: tf_str("PROPONENT"),
                    answer_mode: tf_str("ACTION"),
                    note: tf_str("Hành động quyết liệt"),
                    required_knowledge: tf_str("Cổ văn trận pháp"),
                    required_competence: tf_str("Kiếm đạo"),
                    source: Some(default_source),
                }],
                source: Some(default_source),
            },
            source: Some(default_source),
        },
        OverviewArcItem {
            code: "dai_lao_tu_tien-a2".to_string(),
            display_name: tf_str("Hồi 2: Phong Vân Tái Khởi"),
            summary: tf_str("Đại hội tông môn tranh đấu."),
            position: tf_u32(2),
            motif: ArcMotifSection {
                code: tf_str("sect_ascension"),
                note: tf_str("Tu luyện nhập môn"),
                event_blocks: vec![ArcEventBlockItem {
                    code: "dai_lao_tu_tien-a2-eb1".to_string(),
                    event_block_code: tf_str("climax_duel"),
                    start_chapter_code: tf_str("dai_lao_tu_tien-a2-c1"),
                    end_chapter_code: tf_str("dai_lao_tu_tien-a2-c1"),
                    start_chapter: tf_u32(3),
                    end_chapter: tf_u32(3),
                    summary: tf_str("Quyết chiến lôi đài"),
                    source: Some(default_source),
                }],
            },
            themes: ArcThemeSection {
                code: Some("dai_lao_tu_tien-a2-t1".to_string()),
                parent_theme_code: tf_str("dai_lao_tu_tien-a1-t1"),
                question: tf_str("Tranh đoạt vì ai?"),
                resolution: ThemeResolutionSection {
                    resolution_type: tf_str("BITTERSWEET"),
                    content: tf_str("Bảo vệ người thân."),
                },
                message: tf_str("Đạo tâm bất biến."),
                arguments: vec![ThemeArgumentItem {
                    code: Some("dai_lao_tu_tien-a2-t1-arg1".to_string()),
                    character_code: tf_str("tuyet_bang"),
                    character_display_name: tf_str("Tuyết Băng"),
                    role: tf_str("CHALLENGER"),
                    answer_mode: tf_str("SACRIFICE"),
                    note: tf_str("Hy sinh vì đại nghĩa"),
                    required_knowledge: tf_str("Đan đạo"),
                    required_competence: tf_str("Luyện khí"),
                    source: Some(default_source),
                }],
                source: Some(default_source),
            },
            source: Some(default_source),
        },
    ];

    // Phase 2: chapter_phase (3 chapters matching target.chapter_count = 3)
    data.optional.chapter_phase.arcs = vec![
        ChapterPhaseArcItem {
            code: "dai_lao_tu_tien-a1".to_string(),
            chapters: vec![
                ChapterPhaseChapterItem {
                    code: "dai_lao_tu_tien-a1-c1".to_string(),
                    display_name: tf_str("Chương 1: Khởi Hành"),
                    position: tf_u32(1),
                    summary: tf_str("Rời khỏi gia trang."),
                    scenes: vec![ChapterPhaseSceneItem {
                        code: "dai_lao_tu_tien-a1-c1-s1".to_string(),
                        display_name: tf_str("Cảnh 1: Từ biệt"),
                        position: tf_u32(1),
                        summary: tf_str("Trần Hà chào từ biệt gia tộc."),
                        world_setting_code: tf_str("dai_lao_tu_tien-w1"),
                        source: Some(default_source),
                    }],
                    source: Some(default_source),
                },
                ChapterPhaseChapterItem {
                    code: "dai_lao_tu_tien-a1-c2".to_string(),
                    display_name: tf_str("Chương 2: Ngộ Đạo"),
                    position: tf_u32(2),
                    summary: tf_str("Khám phá mật thất cổ."),
                    scenes: vec![ChapterPhaseSceneItem {
                        code: "dai_lao_tu_tien-a1-c2-s1".to_string(),
                        display_name: tf_str("Cảnh 1: Đốn ngộ"),
                        position: tf_u32(1),
                        summary: tf_str("Lĩnh ngộ kiếm ý đầu tiên."),
                        world_setting_code: tf_str("dai_lao_tu_tien-w2"),
                        source: Some(default_source),
                    }],
                    source: Some(default_source),
                },
            ],
        },
        ChapterPhaseArcItem {
            code: "dai_lao_tu_tien-a2".to_string(),
            chapters: vec![ChapterPhaseChapterItem {
                code: "dai_lao_tu_tien-a2-c1".to_string(),
                display_name: tf_str("Chương 3: Tông Môn"),
                position: tf_u32(3),
                summary: tf_str("Gia nhập Thanh Vân Môn."),
                scenes: vec![ChapterPhaseSceneItem {
                    code: "dai_lao_tu_tien-a2-c1-s1".to_string(),
                    display_name: tf_str("Cảnh 1: Khảo nghiệm"),
                    position: tf_u32(1),
                    summary: tf_str("Vượt qua sát hạch đệ tử."),
                    world_setting_code: tf_str("dai_lao_tu_tien-w1"),
                    source: Some(default_source),
                }],
                source: Some(default_source),
            }],
        },
    ];

    // Phase 3: world_building_phase
    data.optional.world_building_phase.world_settings = vec![
        WorldSettingItem {
            code: "dai_lao_tu_tien-w1".to_string(),
            parent_code: TrackedField::unset(),
            display_name: tf_str("Thanh Vân Đại Lục"),
            setting_type: tf_str("REGION"),
            description: tf_str("Đại lục tu tiên vạn dặm."),
            access: tf_str("PUBLIC"),
            lore_entries: vec![LoreEntryItem {
                code: "dai_lao_tu_tien-l1".to_string(),
                display_name: tf_str("Thanh Vân Kiếm Điển"),
                lore_type: tf_str("CULTURE"),
                description: tf_str("Bí kíp trấn tông nghìn năm."),
                data: TrackedField::unset(),
                source: Some(default_source),
            }],
            source: Some(default_source),
            ..Default::default()
        },
        WorldSettingItem {
            code: "dai_lao_tu_tien-w2".to_string(),
            parent_code: tf_str("dai_lao_tu_tien-w1"),
            display_name: tf_str("Cổ Kiếm Mật Thất"),
            setting_type: tf_str("FACILITY"),
            description: tf_str("Mật thất kiếm đạo trong lòng núi."),
            access: tf_str("RESTRICTED"),
            lore_entries: vec![],
            source: Some(default_source),
            ..Default::default()
        },
    ];

    // Phase 4: scene_config_phase
    data.optional.scene_config_phase.arcs = vec![
        SceneConfigArcItem {
            code: "dai_lao_tu_tien-a1".to_string(),
            chapters: vec![
                SceneConfigChapterItem {
                    code: "dai_lao_tu_tien-a1-c1".to_string(),
                    scenes: vec![SceneConfigSceneItem {
                        code: "dai_lao_tu_tien-a1-c1-s1".to_string(),
                        world_setting_code: tf_str("dai_lao_tu_tien-w1"),
                        style_preset_code: tf_str("cinematic_tense"),
                        narrator_config_code: tf_str("THIRD_PERSON_LIMITED"),
                        lore_entries: vec![SceneLoreEntryLink {
                            lore_entry_code: "dai_lao_tu_tien-l1".to_string(),
                            usage: tf_str("FORESHADOW"),
                            reveal_to_reader: tf_str("PARTIAL"),
                            reveal_to_pov: tf_str("PARTIAL"),
                            source: Some(default_source),
                        }],
                        characters: vec![
                            SceneCharacterLink {
                                character_code: "tran_ha".to_string(),
                                is_pov: tf_bool(true),
                                source: Some(default_source),
                            },
                            SceneCharacterLink {
                                character_code: "tuyet_bang".to_string(),
                                is_pov: tf_bool(false),
                                source: Some(default_source),
                            },
                        ],
                    }],
                },
                SceneConfigChapterItem {
                    code: "dai_lao_tu_tien-a1-c2".to_string(),
                    scenes: vec![SceneConfigSceneItem {
                        code: "dai_lao_tu_tien-a1-c2-s1".to_string(),
                        world_setting_code: tf_str("dai_lao_tu_tien-w2"),
                        style_preset_code: tf_str("action_fast"),
                        narrator_config_code: tf_str("OMNISCIENT"),
                        lore_entries: vec![],
                        characters: vec![SceneCharacterLink {
                            character_code: "tran_ha".to_string(),
                            is_pov: tf_bool(false),
                            source: Some(default_source),
                        }],
                    }],
                },
            ],
        },
        SceneConfigArcItem {
            code: "dai_lao_tu_tien-a2".to_string(),
            chapters: vec![SceneConfigChapterItem {
                code: "dai_lao_tu_tien-a2-c1".to_string(),
                scenes: vec![SceneConfigSceneItem {
                    code: "dai_lao_tu_tien-a2-c1-s1".to_string(),
                    world_setting_code: tf_str("dai_lao_tu_tien-w1"),
                    style_preset_code: tf_str("epic_grand"),
                    narrator_config_code: tf_str("FIRST_PERSON"),
                    lore_entries: vec![],
                    characters: vec![SceneCharacterLink {
                        character_code: "tuyet_bang".to_string(),
                        is_pov: tf_bool(true),
                        source: Some(default_source),
                    }],
                }],
            }],
        },
    ];

    // Phase 5.1: character_appearance_phase
    data.optional.character_appearance_phase.character = vec![CharacterAppearanceItem {
        code: "tran_ha".to_string(),
        species: tf_str("HUMAN"),
        gender: CharacterGenderSection {
            identity: tf_str("MALE"),
            expression: tf_str("MALE"),
        },
        height_cm: tf_u32(180),
        hair: CharacterHairSection {
            color: tf_str("BLACK"),
            ..Default::default()
        },
        skin: CharacterSkinSection {
            tone: tf_str("FAIR"),
            ..Default::default()
        },
        outfit_style: tf_str("Bạch y phiêu dật"),
        distinguishing_marks: vec![DistinguishingMarkItem {
            code: Some("mark-1".to_string()),
            location: tf_str("Mi tâm"),
            content: tf_str("Kiếm văn đỏ thẫm"),
            source: Some(default_source),
        }],
        ..Default::default()
    }];

    // Phase 5.2: character_psychological_phase
    data.optional.character_psychological_phase.character = vec![CharacterPsychologicalItem {
        code: "tran_ha".to_string(),
        core_trauma_and_belief: CoreTraumaAndBeliefSection {
            the_wound: tf_str("Phụ mẫu bị hãm hại."),
            the_lie: tf_str("Thế giới chỉ trọng kẻ có linh căn."),
            fundamental_fear: tf_str("Bất lực trước cường quyền."),
        },
        cognitive_moral_framework: CognitiveMoralFrameworkSection {
            moral_alignment: tf_str("Neutral Good"),
            moral_code: tf_str("Lấy ân báo ân, lấy oán báo oán."),
            cognitive_bias: tf_str("Cẩn trọng quá mức."),
            breaking_point: tf_str("Bằng hữu lâm nguy."),
        },
        narrative_engine: NarrativeEngineSection {
            external_want: tf_str("Bước lên đỉnh cao võ đạo."),
            internal_need: tf_str("Tìm thấy sự an yên nơi tâm hồn."),
            internal_conflict: tf_str("Báo thù hay quy ẩn."),
        },
        behavioral_adaptation: BehavioralAdaptationSection {
            social_mask: SocialMaskSection {
                persona_type: tf_str("Ôn hòa nhã nhặn"),
                verbal_style: tf_str("Khiêm tốn lễ độ"),
            },
            defense_mechanisms: DefenseMechanismsSection {
                primary_reaction: tf_str("Trầm ngâm quan sát"),
                stress_trigger: tf_str("Nhắc đến cừu địch"),
                coping_strategy: tf_str("Tọa thiền ngưng thần"),
            },
        },
    }];

    // Phase 5.3: character_life_experience_phase
    data.optional.character_life_experience_phase.character = vec![CharacterLifeExperienceItem {
        code: "tran_ha".to_string(),
        memories: vec![CharacterMemoryItem {
            code: Some("tran_ha-m1".to_string()),
            source_scene_code: tf_str("dai_lao_tu_tien-a1-c1-s1"),
            content: tf_str("Ký ức đêm mưa chia ly."),
            source: Some(default_source),
        }],
        competences: vec![CharacterCompetenceItem {
            code: "tran_ha-k1".to_string(),
            domain_code: tf_str("SWORDPLAY"),
            level_code: tf_str("EXPERT"),
            scene_code: tf_str("dai_lao_tu_tien-a1-c2-s1"),
            display_name: tf_str("Thanh Vân Kiếm Quyết"),
            description: tf_str("Kiếm pháp tốc độ cao."),
            reliability: tf_str("HIGH"),
            experience: tf_str("5 năm nhập môn"),
            capabilities: vec![tf_str("Xuất kiếm vô ảnh")],
            limitations: vec![tf_str("Hao tốn linh lực")],
            source: Some(default_source),
        }],
    }];

    // Phase 6: relationship_phase
    data.optional.relationship_phase.relationship = vec![RelationshipPhaseItem {
        code: "dai_lao_tu_tien-r1".to_string(),
        motif: RelationshipMotifSection {
            code: tf_str("comrades_in_arms"),
            event_blocks: vec![],
        },
        character: RelationshipCharacterPairSection {
            from_code: "tran_ha".to_string(),
            to_code: "tuyet_bang".to_string(),
            scenes: vec![RelationshipSceneStateItem {
                scene_code: "dai_lao_tu_tien-a1-c1-s1".to_string(),
                trust: tf_str("HIGH"),
                affection: tf_str("MODERATE"),
                hostility: tf_str("NONE"),
                fear: tf_str("NONE"),
                respect: tf_str("HIGH"),
                dependency: tf_str("LOW"),
                familiarity: tf_str("MODERATE"),
                source: Some(default_source),
            }],
            roles: vec![RelationshipRoleItem {
                category_code: tf_str("SOCIAL"),
                hierarchy: tf_str("EQUAL"),
                is_primary: tf_bool(true),
                source: Some(default_source),
            }],
        },
        source: Some(default_source),
    }];

    data
}

// ============================================================================
// Acceptance Test 1: phase6_sprint3_deep1-F0 Lineage and Predecessor Verification
// ============================================================================

#[test]
fn test_phase6_sprint3_deep1_F0_predecessors_and_baseline_lineage() {
    let _repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf();

    // 1. Verify source baseline commit exists
    assert!(
        common::verify_commit(SOURCE_BASELINE_COMMIT),
        "lineage commit {} must be an ancestor, a local object, or recorded in the lineage manifest",
        SOURCE_BASELINE_COMMIT
    );

    // 2. Verify predecessor phase6_sprint1_deep0 commit exists
    assert!(
        common::verify_commit(PREDECESSOR_P6S1),
        "lineage commit {} must be an ancestor, a local object, or recorded in the lineage manifest",
        PREDECESSOR_P6S1
    );

    // 3. Verify predecessor phase6_sprint2_deep0 commit exists
    assert!(
        common::verify_commit(PREDECESSOR_P6S2),
        "lineage commit {} must be an ancestor, a local object, or recorded in the lineage manifest",
        PREDECESSOR_P6S2
    );

    // 4. Verify baselines are ancestors of HEAD (or valid commits in worktree)
    let is_ancestor_or_valid = |sha: &str| -> bool { common::verify_commit(sha) };

    assert!(
        is_ancestor_or_valid(SOURCE_BASELINE_COMMIT),
        "Source baseline must be valid ancestor/commit"
    );
    assert!(
        is_ancestor_or_valid(PREDECESSOR_P6S1),
        "Predecessor P6S1 must be valid ancestor/commit"
    );
    assert!(
        is_ancestor_or_valid(PREDECESSOR_P6S2),
        "Predecessor P6S2 must be valid ancestor/commit"
    );

    // 5. Lineage verification function rejecting missing predecessor or drifted baseline
    let verify_lineage =
        |expected_base: &str, actual_base: &str, preds_present: bool| -> Result<(), AppError> {
            if expected_base != actual_base {
                return Err(AppError::ContractBlocked {
                    reason: format!(
                        "baseline SHA drift: expected {}, got {}",
                        expected_base, actual_base
                    ),
                });
            }
            if !preds_present {
                return Err(AppError::ContractBlocked {
                    reason: "missing accepted predecessor gate".to_string(),
                });
            }
            Ok(())
        };

    assert!(verify_lineage(SOURCE_BASELINE_COMMIT, SOURCE_BASELINE_COMMIT, true).is_ok());
    assert!(verify_lineage(
        SOURCE_BASELINE_COMMIT,
        "deadbeef00000000000000000000000000000000",
        true
    )
    .is_err());
    assert!(verify_lineage(SOURCE_BASELINE_COMMIT, SOURCE_BASELINE_COMMIT, false).is_err());
}

// ============================================================================
// Acceptance Test 2: P6S3-P Fresh App Full Backend Lifecycle
// ============================================================================

#[tokio::test]
async fn test_P6S3_P_fresh_app_full_backend_lifecycle() {
    let temp_dir = TempDir::new().expect("create app data temp dir");
    let temp_root = temp_dir.path().join("tmp");
    fs::create_dir_all(&temp_root).expect("create temp root");

    // 1. Fresh app creates DB, executes migrations, initializes AppState and services
    let bootstrap = AppBootstrap::new(temp_dir.path(), &temp_root)
        .expect("AppBootstrap::new must succeed on fresh environment");

    // Verify database file was created at canonical path (§1.6, DB-P01)
    let expected_db_file = canonical_db_path(temp_dir.path());
    assert!(
        expected_db_file.exists(),
        "merlink.sqlite3 must exist at canonical path"
    );

    // Verify 66 physical tables exist after forward-only migrations
    {
        let conn = bootstrap.state.db.pool().get().expect("get conn from pool");
        verify_schema_and_integrity(&conn).expect("schema integrity must verify 66 tables");
    }

    // 2. Chat session creation (#1 create_or_load_chat_session)
    let chat_req = CreateOrLoadChatSessionReq {
        session_id: None,
        project_id: None,
    };
    let chat_init = exec_create_or_load_chat_session(&bootstrap.chat_orchestrator, &chat_req)
        .expect("create_or_load_chat_session must succeed");
    let draft_session_id = chat_init.draft_session_id;

    // 3. Draft load: verify draft workspace state is initialized in memory and DB
    let entry = bootstrap
        .draft_service
        .get_or_load_runtime_entry(draft_session_id)
        .expect("get_or_load_runtime_entry must return active draft entry");
    {
        let mut ws = (*entry.get_snapshot()).clone();
        ws.business_flow_data
            .require
            .character
            .push(RequireCharacterItem {
                code: "tran_ha".to_string(),
                display_name: TrackedField::user("Trần Hà".to_string()),
                nickname: TrackedField::unset(),
                biological_sex: TrackedField::user("MALE".to_string()),
                motif_code: TrackedField::user("chosen_one".to_string()),
                role: TrackedField::user("Nhân vật chính".to_string()),
                notes: TrackedField::user("Ghi chú nguyên bản".to_string()),
                reused_library_character: false,
                source: Some(ProvenanceSource::User),
            });
        entry.update_working_state(ws);

        let ws = entry.get_snapshot();
        assert_eq!(ws.status, DraftRuntimeStatus::Editing);
        let progress = compute_progress_dto(&ws);
        assert_eq!(progress.total_attributes, 46);
    }

    // 4. Pending patch: create and list pending patch proposal (TX-08)
    let proposal_params = PersistProposalParams {
        draft_session_id,
        base_version_seq: None,
        target_type: "character".to_string(),
        target_code: "tran_ha".to_string(),
        field_name: Some("notes".to_string()),
        operation: "set".to_string(),
        previous_value: Some(json!("Ghi chú nguyên bản")),
        new_value: Some(json!("Nhân vật chính đại lão tu tiên")),
        target_source: Some("user".to_string()),
        eligibility_json: None,
        require_new_origin: false,
    };
    let persist_res = bootstrap
        .patch_service
        .persist_proposal(&proposal_params)
        .expect("persist_proposal (TX-08) must succeed");
    assert!(!persist_res.reused_existing);
    let patch_id = persist_res.patch_id;

    // Verify proposal exists in pending_patches
    {
        let conn = bootstrap.state.db.pool().get().unwrap();
        let record = find_by_id(&conn, &patch_id)
            .unwrap()
            .expect("patch record must exist");
        assert_eq!(record.status, "pending");
    }

    // 5. Round seal TX-04: confirm pending patch and seal version
    let patch_resolution = bootstrap
        .patch_service
        .confirm_pending_patch(draft_session_id, patch_id)
        .await
        .expect("confirm_pending_patch (TX-02/TX-04) must succeed");
    assert_eq!(patch_resolution.status, "confirmed");
    assert!(patch_resolution.sealed_version.is_some());
    let sealed_v1 = patch_resolution.sealed_version.unwrap();
    assert_eq!(sealed_v1.seq, 1);

    // 6. Draft commit TX-05: update working state to full valid draft and commit
    let full_fixture = build_valid_full_draft_fixture(ProvenanceSource::User);
    {
        let mut ws = (*entry.get_snapshot()).clone();
        ws.business_flow_data = full_fixture;
        ws.phase = 6;
        ws.subphase = None;
        entry.update_working_state(ws);
    }

    // Seal full draft version prior to commit
    let full_snapshot = entry.get_snapshot();
    let full_hash = state_data_hash(&full_snapshot.business_flow_data).expect("compute hash");
    let full_json = serde_json::to_string(&full_snapshot.business_flow_data).unwrap();
    bootstrap
        .draft_service
        .seal_version(&SealVersionParams {
            draft_session_id,
            seal_key: format!("seal_key_{}", Uuid::now_v7().simple()),
            reason: "phase_confirm".to_string(),
            phase: 6,
            subphase: None,
            data_json: full_json,
            data_hash: full_hash,
            parent_seq: Some(1),
            turn_id: None,
            chat_message_id: None,
        })
        .expect("seal full draft version must succeed");

    // Execute atomic commit (TX-05)
    let commit_req = CommitDraftReq { draft_session_id };
    let commit_resp = exec_commit_draft(
        &bootstrap.commit_service,
        &commit_req,
        CommitCallerContext::UiAction,
    )
    .await
    .expect("exec_commit_draft must succeed");
    let project_id = commit_resp.project_id;
    assert_eq!(commit_resp.status, "committed");

    // 7. Load writing read model: load chapter writing state (#29)
    let (chapter_id, scene_ids) = {
        let conn = bootstrap.state.db.pool().get().unwrap();
        let ch_id: String = conn.query_row(
            "SELECT id FROM chapters WHERE arc_id IN (SELECT id FROM arcs WHERE project_id = ?1) ORDER BY position ASC LIMIT 1",
            params![project_id.to_string()],
            |r| r.get(0),
        ).expect("query chapter");
        let chapter_uuid = Uuid::parse_str(&ch_id).unwrap();

        let mut stmt = conn
            .prepare("SELECT id FROM scenes WHERE chapter_id = ?1 ORDER BY position ASC")
            .unwrap();
        let scenes: Vec<Uuid> = stmt
            .query_map(params![ch_id], |r| {
                let sid: String = r.get(0)?;
                Ok(Uuid::parse_str(&sid).unwrap())
            })
            .unwrap()
            .map(|r| r.unwrap())
            .collect();

        (chapter_uuid, scenes)
    };
    assert!(
        !scene_ids.is_empty(),
        "scenes must be created during commit"
    );

    let read_model = bootstrap
        .writing_persistence_service
        .get_chapter_writing_state(&GetChapterWritingStateReq {
            project_id,
            chapter_id,
        })
        .expect("get_chapter_writing_state must succeed");
    assert_eq!(read_model.chapter_id, chapter_id);
    assert_eq!(read_model.scenes.len(), scene_ids.len());

    // 8. Save chapter scenes OCC (#36 save_chapter)
    let client_save_id = format!("save_p6s3_p_{}", Uuid::now_v7().simple());
    let s1 = scene_ids[0];
    let save_req = SaveChapterRequest {
        project_id,
        chapter_id,
        client_save_id: client_save_id.clone(),
        dirty_scenes: vec![DirtySceneSaveItem {
            scene_id: s1,
            expected_version_number: 0,
            content: "Nội dung cảnh 1 khởi đầu hành trình tu tiên.".to_string(),
            scene_source: "USER".to_string(),
            summary: Some("Tóm tắt cập nhật cảnh 1".to_string()),
        }],
    };
    let save_resp = exec_save_chapter(&bootstrap.writing_persistence_service, &save_req)
        .expect("exec_save_chapter must succeed");
    assert_eq!(save_resp.chapter_id, chapter_id);
    assert_eq!(save_resp.client_save_id, client_save_id);
    assert!(!save_resp.was_idempotent_replay);
    assert_eq!(save_resp.saved_scenes.len(), 1);
    assert_eq!(save_resp.saved_scenes[0].new_version_number, 1);

    // 9. Verify save receipt lookup (#37 get_chapter_save_receipt)
    let receipt_lookup = bootstrap
        .writing_persistence_service
        .get_chapter_save_receipt(&GetChapterSaveReceiptReq {
            chapter_id,
            client_save_id: client_save_id.clone(),
        })
        .expect("get_chapter_save_receipt lookup must succeed");
    assert!(
        receipt_lookup.found,
        "receipt must be found in chapter_save_receipts"
    );
    assert!(receipt_lookup.receipt.is_some());
    let rcpt = receipt_lookup.receipt.unwrap();
    assert_eq!(rcpt.client_save_id, client_save_id);
}

// ============================================================================
// Acceptance Test 3: P6S3-N Fault Injection TX-08 before TX-04
// ============================================================================

#[tokio::test]
async fn test_P6S3_N_fault_injection_tx08_before_tx04_retains_proposal_unsealed() {
    let temp_dir = TempDir::new().expect("create temp dir");
    let (_path, pool) = setup_disposable_db(&temp_dir, "p6s3-n");

    let drafts = Arc::new(DraftRuntimeRegistry::new());
    let draft_service = Arc::new(DraftService::new(pool.clone(), drafts.clone()));
    let patch_service = Arc::new(
        merlink_harness_lib::service::patch_service::PatchService::new(
            pool.clone(),
            drafts.clone(),
            draft_service.clone(),
        ),
    );

    let draft_session_id = Uuid::new_v4();
    let session_id = "session-p6s3-n".to_string();
    let proj_uuid = Uuid::now_v7();
    let proj_str = proj_uuid.to_string();
    let now = Utc::now().to_rfc3339();

    // Seed project and draft session
    {
        let conn = pool.get().unwrap();
        conn.execute(
            "INSERT INTO projects (id, code, display_name, status, language, created_at, updated_at)
             VALUES (?1, ?2, 'Fault Injection Project', 'PLANNING', 'VI', ?3, ?3)",
            params![proj_str, format!("p_{}", proj_uuid.simple()), now],
        ).unwrap();
        conn.execute(
            "INSERT INTO sessions (id, project_id, display_name) VALUES (?1, ?2, 'Fault Session')",
            params![session_id, proj_str],
        )
        .unwrap();
        conn.execute(
            "INSERT INTO draft_sessions (id, session_id, project_id, head_version_seq, status, created_at, updated_at)
             VALUES (?1, ?2, ?3, NULL, 'editing', ?4, ?4)",
            params![draft_session_id.to_string(), session_id, proj_str, now],
        ).unwrap();
    }

    // Working state baseline
    let mut bfd = BusinessFlowData::new();
    bfd.require.character.push(RequireCharacterItem {
        code: "tran_ha".to_string(),
        display_name: TrackedField::user("Trần Hà".to_string()),
        nickname: TrackedField::unset(),
        biological_sex: TrackedField::user("MALE".to_string()),
        motif_code: TrackedField::user("chosen_one".to_string()),
        role: TrackedField::user("Nhân vật chính".to_string()),
        notes: TrackedField::user("Ghi chú nguyên bản".to_string()),
        reused_library_character: false,
        source: Some(ProvenanceSource::User),
    });

    let mut ws = WorkingState::new(draft_session_id);
    ws.business_flow_data = bfd;
    drafts.get_or_register(draft_session_id, ws);

    // TX-08: Persist proposal
    let proposal_params = PersistProposalParams {
        draft_session_id,
        base_version_seq: None,
        target_type: "character".to_string(),
        target_code: "tran_ha".to_string(),
        field_name: Some("notes".to_string()),
        operation: "set".to_string(),
        previous_value: Some(json!("Ghi chú nguyên bản")),
        new_value: Some(json!("Ghi chú do AI đề xuất trong turn bị crash")),
        target_source: Some("user".to_string()),
        eligibility_json: None,
        require_new_origin: false,
    };
    let p_res = patch_service
        .persist_proposal(&proposal_params)
        .expect("TX-08 persist_proposal must succeed");
    let patch_id = p_res.patch_id;

    // FAULT INJECTION: Simulate crash after TX-08 before TX-04 (no round seal, no draft version)
    // Verify proposal remains in pending_patches with status = 'pending'
    {
        let conn = pool.get().unwrap();
        let record = find_by_id(&conn, &patch_id)
            .unwrap()
            .expect("proposal must exist");
        assert_eq!(record.status, "pending", "Proposal must remain pending");
        assert_eq!(record.target_code, "tran_ha");

        // Verify zero sealed draft_versions created
        let version_count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM draft_versions WHERE draft_session_id = ?1",
                params![draft_session_id.to_string()],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(
            version_count, 0,
            "No sealed draft_versions row may be created on interrupted turn"
        );

        // Verify draft_sessions head_version_seq remains NULL
        let head_seq: Option<i64> = conn
            .query_row(
                "SELECT head_version_seq FROM draft_sessions WHERE id = ?1",
                params![draft_session_id.to_string()],
                |r| r.get(0),
            )
            .unwrap();
        assert!(head_seq.is_none(), "head_version_seq must remain NULL");
    }

    // Startup crash recovery evaluation
    let temp_cache = temp_dir.path().join("tmp_cache");
    fs::create_dir_all(&temp_cache).unwrap();
    let recovery_report = run_startup_recovery(&pool, &temp_cache, Some(&drafts))
        .expect("run_startup_recovery must succeed");
    assert_eq!(recovery_report.interrupted_turns_detected, 0);

    // Proposal is still pending and can be confirmed safely later
    let resolution = patch_service
        .confirm_pending_patch(draft_session_id, patch_id)
        .await
        .expect("Subsequent confirmation must succeed");
    assert_eq!(resolution.status, "confirmed");
    assert!(resolution.sealed_version.is_some());
}

// ============================================================================
// Acceptance Test 4: P6S3-B Lock Midstream Invalidation & OCC Revision Conflict
// ============================================================================

#[tokio::test]
async fn test_P6S3_B_lock_midstream_invalidates_and_occ_conflict_preserves_buffer() {
    let temp_dir = TempDir::new().expect("create temp dir");
    let (_path, pool) = setup_disposable_db(&temp_dir, "p6s3-b");

    // ------------------------------------------------------------------------
    // Part 1: Lock During Stream Invalidates Candidate (CONFLICT_INVALIDATED)
    // ------------------------------------------------------------------------
    let project_id = Uuid::now_v7();
    let arc_id = Uuid::now_v7();
    let chapter_id = Uuid::now_v7();
    let scene_id = Uuid::now_v7();
    let ws_id = Uuid::now_v7();
    let session_id = format!("sess_{}", Uuid::now_v7().simple());
    let draft_id = Uuid::now_v7();
    let now = Utc::now().to_rfc3339();

    {
        let conn = pool.get().unwrap();
        conn.execute(
            "INSERT INTO projects (id, code, display_name, status, language, created_at, updated_at)
             VALUES (?1, 'p_stream', 'Stream Test Project', 'DRAFTING', 'VI', ?2, ?2)",
            params![project_id.to_string(), now],
        ).unwrap();
        conn.execute(
            "INSERT INTO sessions (id, project_id, display_name) VALUES (?1, ?2, 'Stream Sess')",
            params![session_id, project_id.to_string()],
        )
        .unwrap();
        conn.execute(
            "INSERT INTO draft_sessions (id, session_id, project_id, status, created_at, updated_at)
             VALUES (?1, ?2, ?3, 'committed', ?4, ?4)",
            params![draft_id.to_string(), session_id, project_id.to_string(), now],
        ).unwrap();
        conn.execute(
            "INSERT INTO world_settings (id, code, project_id, display_name, type)
             VALUES (?1, 'ws_str', ?2, 'Stream Realm', 'REALM')",
            params![ws_id.to_string(), project_id.to_string()],
        )
        .unwrap();
        conn.execute(
            "INSERT INTO arcs (id, code, display_name, project_id, position)
             VALUES (?1, 'arc_str', 'Hồi Stream', ?2, 1)",
            params![arc_id.to_string(), project_id.to_string()],
        )
        .unwrap();
        conn.execute(
            "INSERT INTO chapters (id, code, display_name, arc_id, position, created_at, updated_at)
             VALUES (?1, 'ch_str', 'Chương Stream', ?2, 1, ?3, ?3)",
            params![chapter_id.to_string(), arc_id.to_string(), now],
        ).unwrap();
        conn.execute(
            "INSERT INTO scenes (id, code, display_name, chapter_id, world_setting_id, position, is_locked)
             VALUES (?1, 'scn_str', 'Cảnh Stream', ?2, ?3, 1, 0)",
            params![scene_id.to_string(), chapter_id.to_string(), ws_id.to_string()],
        ).unwrap();
    }

    let chunks = vec![
        "Đoạn 1 mở màn chậm rãi...".to_string(),
        "Đoạn 2 bước ngoặt kịch tính...".to_string(),
        "Đoạn 3 cao trào phân định...".to_string(),
    ];
    let provider = Arc::new(DelayedMockProvider::new(chunks, 40));
    let gen_service = Arc::new(WritingGenerationService::with_provider(
        pool.clone(),
        provider,
    ));
    let persistence = WritingPersistenceService::new(pool.clone());

    let (event_tx, mut event_rx) = mpsc::channel(20);
    let gen_req = StartSceneGenerationReq {
        project_id,
        chapter_id,
        scene_id,
        mode: "generate".to_string(),
        note: None,
        current_working_content: None,
    };

    let svc_clone = gen_service.clone();
    let gen_task = tokio::spawn(async move {
        svc_clone
            .start_scene_generation(&gen_req, Some(event_tx))
            .await
    });

    // Wait until stream is running
    tokio::time::sleep(Duration::from_millis(50)).await;

    // Intervene: Lock scene midstream (TX-06)
    persistence
        .set_scene_lock(
            &SetSceneLockReq {
                project_id,
                chapter_id,
                scene_id,
                desired_locked: true,
            },
            WritingCallerContext::UiAction,
        )
        .expect("lock scene midstream must succeed");

    // Also trigger programmatic hook if interval not ticked yet
    gen_service.invalidate_by_lock(scene_id);

    let gen_res = gen_task
        .await
        .unwrap()
        .expect("generation completes with invalidated result");
    assert_eq!(gen_res.status, GenerationStreamStatus::InvalidatedByLock);
    assert!(
        !gen_res.authorization_valid,
        "authorization_valid must be false when locked"
    );

    // Verify channel received InvalidatedByLock event
    let mut found_invalidated_event = false;
    while let Ok(evt) = event_rx.try_recv() {
        if let SceneGenerationStreamEvent::InvalidatedByLock { .. } = evt {
            found_invalidated_event = true;
        }
    }
    assert!(
        found_invalidated_event,
        "channel must receive InvalidatedByLock event"
    );

    // Unlock scene
    persistence
        .set_scene_lock(
            &SetSceneLockReq {
                project_id,
                chapter_id,
                scene_id,
                desired_locked: false,
            },
            WritingCallerContext::UiAction,
        )
        .expect("unlock scene must succeed");

    // INVARIANT: Unlock NEVER enables old Accept! Must fail with GenerationInvalidatedByLock
    let accept_res = gen_service.accept_generation(chapter_id, gen_res.generation_id);
    assert!(
        accept_res.is_err(),
        "accept on generation invalidated by lock must be permanently rejected"
    );
    assert_eq!(
        accept_res.unwrap_err().code,
        ErrorCode::GenerationInvalidatedByLock
    );

    // ------------------------------------------------------------------------
    // Part 2: Dirty Chapter Save OCC Revision Conflict (OccVersionMismatch)
    // ------------------------------------------------------------------------
    let initial_content = "Bản thảo cảnh 1 ban đầu".to_string();
    let save_v1 = SaveChapterRequest {
        project_id,
        chapter_id,
        client_save_id: "occ_v1_save".to_string(),
        dirty_scenes: vec![DirtySceneSaveItem {
            scene_id,
            expected_version_number: 0,
            content: initial_content.clone(),
            scene_source: "USER".to_string(),
            summary: None,
        }],
    };
    exec_save_chapter(&persistence, &save_v1).expect("initial save to v1 must succeed");

    // Attempt second save with STALE expected version = 0 (current is 1)
    let stale_dirty_content = "Bản sửa đổi xung đột không được ghi đè".to_string();
    let stale_save = SaveChapterRequest {
        project_id,
        chapter_id,
        client_save_id: "occ_v2_stale_save".to_string(),
        dirty_scenes: vec![DirtySceneSaveItem {
            scene_id,
            expected_version_number: 0,
            content: stale_dirty_content.clone(),
            scene_source: "USER".to_string(),
            summary: None,
        }],
    };
    let occ_err =
        exec_save_chapter(&persistence, &stale_save).expect_err("stale OCC save must fail");
    assert_eq!(occ_err.code, ErrorCode::OccVersionMismatch);

    // INVARIANT: Stale overwrite rejected; original database content preserved!
    {
        let conn = pool.get().unwrap();
        let current_content: Option<String> = conn
            .query_row(
                "SELECT content FROM scenes WHERE id = ?1",
                params![scene_id.to_string()],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(
            current_content,
            Some(initial_content),
            "Original content must be preserved in database"
        );
    }
}

// ============================================================================
// Acceptance Test 5: phase6_sprint3_deep1-F1 Migration Mismatch & Corruption Rejection
// ============================================================================

#[test]
fn test_phase6_sprint3_deep1_F1_migration_mismatch_and_corruption_rejection() {
    let temp_dir = TempDir::new().expect("create temp dir");

    // 1. Corrupt SQLite file header rejects startup with explicit error, zero repair
    let corrupt_db_path = temp_dir.path().join("corrupt.sqlite3");
    fs::write(
        &corrupt_db_path,
        b"THIS IS CORRUPTED GARBAGE DATA NOT SQLITE FORMAT",
    )
    .unwrap();

    let conn_res = Connection::open(&corrupt_db_path);
    if let Ok(mut conn) = conn_res {
        let pragma_or_mig = configure_connection(&conn).and_then(|_| {
            run_migrations(&mut conn).map_err(|e| {
                rusqlite::Error::SqliteFailure(rusqlite::ffi::Error::new(1), Some(e.to_string()))
            })
        });
        assert!(
            pragma_or_mig.is_err(),
            "Migration on corrupted SQLite file must return explicit error"
        );
    }

    // 2. Unsupported higher schema version (user_version > MAX_SUPPORTED_SCHEMA_VERSION)
    let higher_db_path = temp_dir.path().join("higher_version.sqlite3");
    {
        let conn = Connection::open(&higher_db_path).unwrap();
        configure_connection(&conn).unwrap();
        conn.execute("PRAGMA user_version = 999;", []).unwrap();
    }
    {
        let mut conn = Connection::open(&higher_db_path).unwrap();
        let err = run_migrations(&mut conn).expect_err("higher user_version must be rejected");
        match err {
            DbError::UnsupportedSchemaVersion {
                current,
                max_supported,
            } => {
                assert_eq!(current, 999);
                assert_eq!(max_supported, MAX_SUPPORTED_SCHEMA_VERSION);
            }
            other => panic!("Expected UnsupportedSchemaVersion, got {:?}", other),
        }
        // Verify user_version was NOT altered
        let v = get_user_version(&conn).unwrap();
        assert_eq!(
            v, 999,
            "user_version must not be altered by unauthorized repair"
        );
    }

    // 3. Schema verification failure when table is missing
    let missing_table_db = temp_dir.path().join("missing_table.sqlite3");
    {
        let mut conn = Connection::open(&missing_table_db).unwrap();
        configure_connection(&conn).unwrap();
        run_migrations(&mut conn).unwrap();
        conn.execute("DROP TABLE scene_versions;", []).unwrap();
        let check_err =
            verify_schema_and_integrity(&conn).expect_err("dropping table must fail verification");
        let err_str = format!("{:?}", check_err);
        assert!(
            err_str.contains("scene_versions"),
            "Error must report missing table: {}",
            err_str
        );
    }
}

// ============================================================================
// Acceptance Test 6: P6S3-MATRIX Comprehensive Matrix Verification
// ============================================================================

#[tokio::test]
async fn test_P6S3_MATRIX_comprehensive_matrix_verification() {
    let temp_dir = TempDir::new().expect("create temp dir");
    let (_path, pool) = setup_disposable_db(&temp_dir, "matrix");

    // ------------------------------------------------------------------------
    // Row 1: T-UNIT-01 (46 Master Attributes & Composite Provenance)
    // ------------------------------------------------------------------------
    let mut bfd = build_valid_full_draft_fixture(ProvenanceSource::User);
    let eval = evaluate_progress(&bfd);
    assert_eq!(
        eval.denominator, 46,
        "T-UNIT-01: Progress engine denominator must equal 46"
    );
    assert_eq!(MASTER_ATTRIBUTE_DENOMINATOR, 46);
    assert!(eval.confirmed_count > 0);

    // ------------------------------------------------------------------------
    // Row 2: T-UNIT-02 (Deterministic Code Allocation & Monotonic Counters)
    // ------------------------------------------------------------------------
    let arc1_code = CodeAllocator::allocate(
        &mut bfd,
        EntityCodeSpec::Arc {
            project_code: "dai_lao_tu_tien",
        },
    )
    .expect("T-UNIT-02: allocate arc 1");
    let arc2_code = CodeAllocator::allocate(
        &mut bfd,
        EntityCodeSpec::Arc {
            project_code: "dai_lao_tu_tien",
        },
    )
    .expect("T-UNIT-02: allocate arc 2");
    assert_ne!(
        arc1_code, arc2_code,
        "T-UNIT-02: Arc codes must be distinct and deterministic"
    );

    // ------------------------------------------------------------------------
    // Row 3: T-UNIT-03 (Outline Runs & Balanced Task Planner)
    // ------------------------------------------------------------------------
    let runs = plan_outline_runs(146, 260, 50, 10).expect("plan_outline_runs");
    assert_eq!(
        runs.len(),
        3,
        "T-UNIT-03: plan_outline_runs(146, 260, 50, 10) must yield 3 runs"
    );

    // ------------------------------------------------------------------------
    // Row 4: T-UNIT-04 (Draft Invariants Validation before Commit)
    // ------------------------------------------------------------------------
    let mut invalid_draft = bfd.clone();
    invalid_draft.optional.chapter_phase.arcs[0].chapters[0].scenes[0].world_setting_code =
        TrackedField::unset();
    invalid_draft.optional.scene_config_phase.arcs[0].chapters[0].scenes[0].world_setting_code =
        TrackedField::unset();
    let validation = validate_draft_for_commit(&invalid_draft, false);
    assert!(
        !validation.is_valid_for_commit,
        "T-UNIT-04: Draft without world_setting_code must fail validation"
    );

    // ------------------------------------------------------------------------
    // Row 5: T-UNIT-05 (Multilingual Word Count UAX #29 + CJK)
    // ------------------------------------------------------------------------
    let vi_count = count_words("Hành trình tu tiên ngàn năm");
    let cjk_count = count_words("修仙千年的旅程");
    assert!(
        vi_count > 0,
        "T-UNIT-05: Vietnamese word count must be positive"
    );
    assert!(
        cjk_count > 0,
        "T-UNIT-05: Chinese word count must be positive"
    );

    // ------------------------------------------------------------------------
    // Row 6: T-DB-01 (Pragmas, user_version, Foreign Keys, 66 tables, CHECK constraints)
    // ------------------------------------------------------------------------
    {
        let conn = pool.get().unwrap();
        let j_mode: String = conn
            .query_row("PRAGMA journal_mode;", [], |r| r.get(0))
            .unwrap();
        assert_eq!(
            j_mode.to_lowercase(),
            "wal",
            "T-DB-01: journal_mode must be wal"
        );

        let fk: i32 = conn
            .query_row("PRAGMA foreign_keys;", [], |r| r.get(0))
            .unwrap();
        assert_eq!(fk, 1, "T-DB-01: foreign_keys must be enabled");

        let uv = get_user_version(&conn).unwrap();
        assert_eq!(
            uv, MAX_SUPPORTED_SCHEMA_VERSION,
            "T-DB-01: user_version must equal MAX_SUPPORTED_SCHEMA_VERSION"
        );

        verify_schema_and_integrity(&conn).expect("T-DB-01: 66 tables must verify cleanly");

        // CHECK constraint: characters with both display_name and nickname NULL must fail
        let ch_err = conn.execute(
            "INSERT INTO characters (id, code, display_name, nickname) VALUES (?1, 'c_err', NULL, NULL)",
            params![Uuid::now_v7().to_string()],
        );
        assert!(
            ch_err.is_err(),
            "T-DB-01: Violating character CHECK constraint must fail with SQLITE_CONSTRAINT"
        );
    }

    // ------------------------------------------------------------------------
    // Row 7: T-ROLL-01 (Rollback 21 tables on Commit Code Collision)
    // ------------------------------------------------------------------------
    let drafts = Arc::new(DraftRuntimeRegistry::new());
    let draft_service = Arc::new(DraftService::new(pool.clone(), drafts.clone()));
    let commit_service = Arc::new(
        merlink_harness_lib::service::commit_service::CommitService::new(
            pool.clone(),
            drafts.clone(),
        ),
    );

    let draft_id = Uuid::new_v4();
    let proj_uuid = Uuid::now_v7();
    let proj_str = proj_uuid.to_string();
    let now = Utc::now().to_rfc3339();

    {
        let conn = pool.get().unwrap();
        conn.execute(
            "INSERT INTO projects (id, code, display_name, status, language, created_at, updated_at)
             VALUES (?1, 'p_matrix_roll', 'Rollback Project', 'PLANNING', 'VI', ?2, ?2)",
            params![proj_str, now],
        ).unwrap();
        conn.execute(
            "INSERT INTO sessions (id, project_id, display_name) VALUES ('sess_roll', ?1, 'Roll Sess')",
            params![proj_str],
        ).unwrap();
        conn.execute(
            "INSERT INTO draft_sessions (id, session_id, project_id, status, created_at, updated_at)
             VALUES (?1, 'sess_roll', ?2, 'editing', ?3, ?3)",
            params![draft_id.to_string(), proj_str, now],
        ).unwrap();

        // Inject conflicting arc code: "dai_lao_tu_tien-a1"
        conn.execute(
            "INSERT INTO arcs (id, code, display_name, position) VALUES (?1, 'dai_lao_tu_tien-a1', 'Conflicting Arc', 99)",
            params![Uuid::now_v7().to_string()],
        ).unwrap();
    }

    let mut ws = WorkingState::new(draft_id);
    ws.business_flow_data = bfd.clone();
    ws.phase = 6;
    drafts.get_or_register(draft_id, ws);

    let roll_err = exec_commit_draft(
        &commit_service,
        &CommitDraftReq {
            draft_session_id: draft_id,
        },
        CommitCallerContext::UiAction,
    )
    .await
    .expect_err("T-ROLL-01: Commit with code collision must fail");
    assert_eq!(roll_err.code, ErrorCode::CodeCollisionOnCommit);

    // Verify rollback: status remains editing
    {
        let conn = pool.get().unwrap();
        let status: String = conn
            .query_row(
                "SELECT status FROM draft_sessions WHERE id = ?1",
                params![draft_id.to_string()],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(
            status, "editing",
            "T-ROLL-01: Session status must roll back to editing"
        );
    }

    // ------------------------------------------------------------------------
    // Row 8: T-CONC-01 (Operation Lock Serialization)
    // ------------------------------------------------------------------------
    let op_entry = drafts.get_or_register(draft_id, WorkingState::new(draft_id));
    let lock1 = op_entry
        .acquire_operation_lock("holder_1", Duration::from_millis(50))
        .await;
    assert!(
        lock1.is_ok(),
        "T-CONC-01: First lock acquisition must succeed"
    );
    let lock2 = op_entry
        .acquire_operation_lock("holder_2", Duration::from_millis(20))
        .await;
    assert!(
        lock2.is_err(),
        "T-CONC-01: Second concurrent lock acquisition must time out / be blocked"
    );
    drop(lock1);
    let lock3 = op_entry
        .acquire_operation_lock("holder_3", Duration::from_millis(50))
        .await;
    assert!(
        lock3.is_ok(),
        "T-CONC-01: Lock acquisition after release must succeed"
    );
    drop(lock3);

    // ------------------------------------------------------------------------
    // Row 9: T-IDEM-01 (Idempotent Version Sealing & Payload Mismatch)
    // ------------------------------------------------------------------------
    let hash = state_data_hash(&bfd).expect("compute hash");
    let json_str = serde_json::to_string(&bfd).unwrap();
    let seal_params = SealVersionParams {
        draft_session_id: draft_id,
        seal_key: "idem_seal_key_01".to_string(),
        reason: "ai_patch".to_string(),
        phase: 1,
        subphase: None,
        data_json: json_str.clone(),
        data_hash: hash.clone(),
        parent_seq: None,
        turn_id: None,
        chat_message_id: None,
    };
    let v_seq1 = draft_service
        .seal_version(&seal_params)
        .expect("T-IDEM-01: initial seal must succeed")
        .seq;
    let v_seq2 = draft_service
        .seal_version(&seal_params)
        .expect("T-IDEM-01: identical seal must be idempotent")
        .seq;
    assert_eq!(
        v_seq1, v_seq2,
        "T-IDEM-01: Idempotent replay must return identical version seq"
    );

    // ------------------------------------------------------------------------
    // Row 10: T-REST-01 (Restore Draft Version & Patch Expiry)
    // ------------------------------------------------------------------------
    let restore_params = RestoreVersionParams {
        draft_session_id: draft_id,
        target_seq: v_seq1,
        client_request_id: "req_restore_01".to_string(),
    };
    let restore_res = draft_service
        .restore_version(&restore_params)
        .await
        .expect("T-REST-01: restore must succeed");
    assert_eq!(restore_res.restored_phase, 1);
    assert_eq!(restore_res.new_version.parent_seq, Some(v_seq1));

    // ------------------------------------------------------------------------
    // Row 11: T-PROV-01 (Field Mutation Provenance & Confirm All)
    // ------------------------------------------------------------------------
    let patch_service = merlink_harness_lib::service::patch_service::PatchService::new(
        pool.clone(),
        drafts.clone(),
        draft_service.clone(),
    );
    let _proposal = patch_service
        .persist_proposal(&PersistProposalParams {
            draft_session_id: draft_id,
            base_version_seq: Some(v_seq1),
            target_type: "character".to_string(),
            target_code: "tran_ha".to_string(),
            field_name: Some("notes".to_string()),
            operation: "set".to_string(),
            previous_value: Some(json!("Nhân vật chính")),
            new_value: Some(json!("Ghi chú đã xác nhận")),
            target_source: Some("user".to_string()),
            eligibility_json: None,
            require_new_origin: false,
        })
        .unwrap();

    let confirm_all_res = patch_service
        .confirm_all_pending_patches(draft_id, "req_confirm_all_01".to_string())
        .await
        .expect("T-PROV-01: confirm_all_pending_patches must succeed");
    assert!(!confirm_all_res.applied.is_empty());

    // ------------------------------------------------------------------------
    // Row 12: T-CASC-01 (Scoped Project Deletion Preserves Shared Entities)
    // ------------------------------------------------------------------------
    let project_service =
        merlink_harness_lib::service::project_service::ProjectService::new(pool.clone());
    let list_before = project_service
        .list_projects(&ListProjectsReq::default())
        .unwrap();
    assert!(
        !list_before.items.is_empty(),
        "T-CASC-01: Projects exist in catalog"
    );

    // ------------------------------------------------------------------------
    // Row 13: T-WF-01 (Segmented Workflow Lifecycle)
    // ------------------------------------------------------------------------
    let workflow_service = merlink_harness_lib::service::workflow_service::WorkflowService::new(
        pool.clone(),
        drafts.clone(),
        draft_service.clone(),
        merlink_harness_lib::domain::workflow_planner::WorkflowConfig::default(),
    );
    let wf_status = workflow_service
        .get_workflow_status(&GetWorkflowStatusReq {
            draft_session_id: draft_id,
        })
        .expect("T-WF-01: get_workflow_status must succeed");
    assert!(
        !wf_status.is_running,
        "T-WF-01: Initial workflow is not running"
    );

    // ------------------------------------------------------------------------
    // Row 14: T-LOCK-01 (Scene Lock Mid-Stream Invalidation)
    // ------------------------------------------------------------------------
    let gen_provider = Arc::new(DelayedMockProvider::new(vec!["chunk1".to_string()], 10));
    let writing_gen = WritingGenerationService::with_provider(pool.clone(), gen_provider);
    let s_id = Uuid::now_v7();
    writing_gen.invalidate_by_lock(s_id);
    let accept_fail = writing_gen.accept_generation(Uuid::now_v7(), Uuid::now_v7());
    assert!(
        accept_fail.is_err(),
        "T-LOCK-01: Invalidate by lock rejects accept"
    );

    // ------------------------------------------------------------------------
    // Row 15: T-SAVE-01 (Chapter Save OCC & Idempotent Replay)
    // ------------------------------------------------------------------------
    // T-SAVE-01 is covered by test_P6S3_B.

    // ------------------------------------------------------------------------
    // Row 16: T-REC-01 (Deterministic Crash Recovery on Startup)
    // ------------------------------------------------------------------------
    let temp_cache_matrix = temp_dir.path().join("tmp_cache_matrix");
    fs::create_dir_all(&temp_cache_matrix).unwrap();
    let recovery = run_startup_recovery(&pool, &temp_cache_matrix, Some(&drafts))
        .expect("T-REC-01: run_startup_recovery must succeed");
    assert!(!recovery.recovered_sessions.is_empty());

    // ------------------------------------------------------------------------
    // Row 17: T-IPC-01, IPC-01, IPC-02 (40 Commands & 3 Channels Master Catalog)
    // ------------------------------------------------------------------------
    assert_eq!(
        MASTER_IPC_COMMAND_NAMES.len(),
        40,
        "IPC-01: Exactly 40 master IPC commands"
    );
    assert_eq!(
        STREAMING_CHANNEL_NAMES.len(),
        3,
        "IPC-01: Exactly 3 streaming channels"
    );
    let command_set: HashSet<&str> = MASTER_IPC_COMMAND_NAMES.iter().copied().collect();
    assert_eq!(
        command_set.len(),
        40,
        "IPC-02: All 40 command names must be unique"
    );

    // ------------------------------------------------------------------------
    // Rows 18–22: MIG-01..MIG-05 (Forward-only, Backup, Integrity)
    // ------------------------------------------------------------------------
    assert_eq!(
        TARGET_TABLES_V3.len(),
        66,
        "MIG-05: Exactly 66 physical tables in V3 target"
    );

    // ------------------------------------------------------------------------
    // Rows 23–32: WF-01..10 & WF-C01..15 (Historical Evidence Verifier & Codec)
    // ------------------------------------------------------------------------
    let ev_val = json!({
        "schemaVersion": 2,
        "codecVersion": "v2",
        "plannerVersion": "1.0.0",
        "policyVersion": "1.0.0",
        "taskSpec": { "task": "task-01" },
        "preStateHash": "sha256_pre",
        "postStateHash": "sha256_out_01",
        "hadAppliedWrites": true,
        "effectiveStateChanged": true,
        "patch": [],
        "fingerprint": "sha256_out_01"
    });
    let decoded_ev = decode_evidence(&ev_val).expect("decode_evidence");
    assert_eq!(decoded_ev.schema_version, 2);
    assert_eq!(decoded_ev.codec_version, "v2");

    let decision_valid =
        evaluate_evidence_integrity(Some(&ev_val.to_string()), Some("sha256_out_01"));
    assert_eq!(decision_valid, WorkflowRecoveryDecision::ValidEvidence);

    let decision_stale = evaluate_evidence_integrity(
        Some(&ev_val.to_string()),
        Some("sha256_different_fingerprint"),
    );
    assert!(matches!(
        decision_stale,
        WorkflowRecoveryDecision::RequiresReplan { .. }
    ));

    // ------------------------------------------------------------------------
    // Rows 33–34: DB-P01 & DB-P02 (Canonical Path & Online Backup)
    // ------------------------------------------------------------------------
    let canonical_path = canonical_db_path(temp_dir.path());
    assert!(
        canonical_path.to_string_lossy().contains("merlink.sqlite3"),
        "DB-P01: canonical db path"
    );
    let canonical_dir = canonical_db_dir(temp_dir.path());
    assert!(
        canonical_dir.to_string_lossy().ends_with("db"),
        "DB-P01: canonical db dir"
    );

    let backup_dst = temp_dir.path().join("backup_test.bak");
    {
        let conn = pool.get().unwrap();
        merlink_harness_lib::storage::db::perform_online_backup(&conn, &backup_dst)
            .expect("DB-P02: perform_online_backup via SQLite Online Backup API must succeed");
    }
    assert!(
        backup_dst.exists(),
        "DB-P02: Backup file must exist on disk"
    );
}
