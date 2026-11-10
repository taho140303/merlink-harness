//! Acceptance and integration tests for `phase4_sprint2_deep0`
//! ("Commit draft atomically into relational schema").
//!
//! Complies with docs/business/core.md (§3.7, §3.8), docs/business/be-core.md (§7.1, §7.2, §7.3, §10.2.6),
//! and docs/plan/evidence/contract-ledger.md.

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
#[path = "../src/storage/draft_repo.rs"]
pub mod draft_repo;
#[path = "../src/storage/commit_repo.rs"]
pub mod commit_repo;

pub mod storage {
    pub use crate::commit_repo;
    pub use crate::db;
    pub use crate::draft_repo;
    pub use crate::migration;
}

#[path = "../src/service/draft_runtime.rs"]
pub mod draft_runtime;
#[path = "../src/service/draft_service.rs"]
pub mod draft_service;
#[path = "../src/service/commit_service.rs"]
pub mod commit_service;

pub mod service {
    pub use crate::commit_service;
    pub use crate::draft_runtime;
    pub use crate::draft_service;
}

#[path = "../src/ipc/dto.rs"]
pub mod dto;
#[path = "../src/ipc/commit_commands.rs"]
pub mod commit_commands;

pub mod ipc {
    pub use crate::commit_commands;
    pub use crate::dto;
}

use chrono::Utc;
use error::ErrorCode;
use domain::business_flow::*;
use ipc::commit_commands::*;
use ipc::dto::*;
use service::commit_service::{CallerContext, CommitService};
use service::draft_runtime::DraftRuntimeRegistry;
use service::draft_service::{DraftService, SealVersionParams};
use storage::db::configure_connection;
use storage::migration::run_migrations;

use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::Connection;
use std::path::PathBuf;
use std::process::Command;
use std::sync::Arc;
use tempfile::TempDir;
use uuid::Uuid;

const BASELINE_COMMIT: &str = "d6a50b84d300546896c0a8aaed9e1cbff815b4a7";
const GATE_P3S2: &str = "ded1e9c5457ab8e72f456a515937a05af6ebba63";
const GATE_P3S5: &str = "fecaf1b6d2b73f0c54edc553b7ccf7e15799280a";
const GATE_P3S6: &str = "181eda6d87f7fac1d17e05f82d843222aa0a376e";
const GATE_P2S6: &str = "75d3886503643e6398c0be8c600b9b712811b3e0";
const GATE_P1S1: &str = "56dd590569a0d6123b4f09d6f1c6761a8b1d7ff2";

/// Setup a fresh disposable SQLite database with 66-table migrations applied.
fn setup_test_db(temp_dir: &TempDir) -> (PathBuf, Pool<SqliteConnectionManager>) {
    let db_path = temp_dir.path().join("test_merlink_p4s2.sqlite3");
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

/// Seed initial project, session, and draft_session.
fn seed_draft_session(
    pool: &Pool<SqliteConnectionManager>,
    draft_session_id: Uuid,
    session_id: &str,
    status: &str,
) {
    let proj_uuid = Uuid::now_v7();
    let proj_str = proj_uuid.to_string();
    let draft_str = draft_session_id.to_string();
    let code_str = format!("p_{}", proj_uuid.simple());

    let conn = pool.get().expect("get conn for seeding");
    conn.execute(
        "INSERT INTO projects (id, code, display_name, status, language) VALUES (?1, ?2, 'Initial Planning Project', 'PLANNING', 'VI')",
        rusqlite::params![proj_str, code_str],
    ).expect("seed project");

    conn.execute(
        "INSERT INTO sessions (id, project_id, display_name) VALUES (?1, ?2, 'Active Session')",
        rusqlite::params![session_id, proj_str],
    ).expect("seed session");

    conn.execute(
        "INSERT INTO draft_sessions (id, session_id, project_id, head_version_seq, status, created_at, updated_at)
         VALUES (?1, ?2, ?3, NULL, ?4, '2026-03-01T00:00:00Z', '2026-03-01T00:00:00Z')",
        rusqlite::params![draft_str, session_id, proj_str, status],
    ).expect("seed draft_session");
}

/// Builds a fully valid full draft fixture matching `planner_progress.rs` that passes `validate_draft_for_commit`.
fn build_valid_full_draft_fixture(default_source: ProvenanceSource) -> BusinessFlowData {
    let tf_str = |s: &str| TrackedField::new(s.to_string(), default_source);
    let tf_u32 = |v: u32| TrackedField::new(v, default_source);
    let tf_bool = |b: bool| TrackedField::new(b, default_source);

    let mut data = BusinessFlowData::new();
    data.meta.phase = 6;
    data.meta.subphase = None;

    // Phase 0: require (10 attributes)
    data.require.project.code = Some("te_nu_nguu_nhan".to_string());
    data.require.project.display_name = tf_str("Tế Nữ Ngưu Nhân");
    data.require.project.summary = tf_str("Hành trình tu tiên phá vỡ số phận tế nữ.");
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
            code: "minh_hoang".to_string(),
            display_name: tf_str("Minh Hoàng"),
            nickname: tf_str("Hoàng Tử"),
            biological_sex: tf_str("MALE"),
            motif_code: tf_str("fallen_prince"),
            role: tf_str("PROTAGONIST"),
            notes: tf_str("Nhân vật chính"),
            reused_library_character: false,
            source: Some(default_source),
        },
        RequireCharacterItem {
            code: "linh_dan".to_string(),
            display_name: tf_str("Linh Đan"),
            nickname: tf_str("Tiểu Linh"),
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
            code: "te_nu_nguu_nhan-a1".to_string(),
            display_name: tf_str("Hồi 1: Huyết Tế Đàn"),
            summary: tf_str("Khởi đầu tại tế đàn cổ xưa."),
            position: tf_u32(1),
            motif: ArcMotifSection {
                code: tf_str("survival_escape"),
                note: tf_str("Thoát khỏi huyết tế"),
                event_blocks: vec![ArcEventBlockItem {
                    code: "te_nu_nguu_nhan-a1-eb1".to_string(),
                    event_block_code: tf_str("inciting_incident"),
                    start_chapter_code: tf_str("te_nu_nguu_nhan-a1-c1"),
                    end_chapter_code: tf_str("te_nu_nguu_nhan-a1-c2"),
                    start_chapter: tf_u32(1),
                    end_chapter: tf_u32(2),
                    summary: tf_str("Biến cố huyết đàn vỡ vụn"),
                    source: Some(default_source),
                }],
            },
            themes: ArcThemeSection {
                code: Some("te_nu_nguu_nhan-a1-t1".to_string()),
                parent_theme_code: TrackedField::unset(),
                question: tf_str("Số mệnh do trời hay do người?"),
                resolution: ThemeResolutionSection {
                    resolution_type: tf_str("TRIUMPH"),
                    content: tf_str("Tự tay chặt đứt xiềng xích."),
                },
                message: tf_str("Ý chí vượt lên định mệnh."),
                arguments: vec![ThemeArgumentItem {
                    code: Some("te_nu_nguu_nhan-a1-t1-arg1".to_string()),
                    character_code: tf_str("minh_hoang"),
                    character_display_name: tf_str("Minh Hoàng"),
                    role: tf_str("PROPONENT"),
                    answer_mode: tf_str("ACTION"),
                    note: tf_str("Phá đàn cứu người"),
                    required_knowledge: tf_str("Cổ ngữ tế đàn"),
                    required_competence: tf_str("Kiếm thuật"),
                    source: Some(default_source),
                }],
                source: Some(default_source),
            },
            source: Some(default_source),
        },
        OverviewArcItem {
            code: "te_nu_nguu_nhan-a2".to_string(),
            display_name: tf_str("Hồi 2: Thiên Sơn Phong Bão"),
            summary: tf_str("Bước chân vào tông môn Thiên Sơn."),
            position: tf_u32(2),
            motif: ArcMotifSection {
                code: tf_str("sect_ascension"),
                note: tf_str("Tu luyện tông môn"),
                event_blocks: vec![ArcEventBlockItem {
                    code: "te_nu_nguu_nhan-a2-eb1".to_string(),
                    event_block_code: tf_str("climax_duel"),
                    start_chapter_code: tf_str("te_nu_nguu_nhan-a2-c1"),
                    end_chapter_code: tf_str("te_nu_nguu_nhan-a2-c1"),
                    start_chapter: tf_u32(3),
                    end_chapter: tf_u32(3),
                    summary: tf_str("Đại hội tông môn"),
                    source: Some(default_source),
                }],
            },
            themes: ArcThemeSection {
                code: Some("te_nu_nguu_nhan-a2-t1".to_string()),
                parent_theme_code: tf_str("te_nu_nguu_nhan-a1-t1"),
                question: tf_str("Quyền lực đòi hỏi cái giá gì?"),
                resolution: ThemeResolutionSection {
                    resolution_type: tf_str("BITTERSWEET"),
                    content: tf_str("Giữ vững đạo tâm giữa tranh đoạt."),
                },
                message: tf_str("Sức mạnh chân chính đến từ sự bảo hộ."),
                arguments: vec![ThemeArgumentItem {
                    code: Some("te_nu_nguu_nhan-a2-t1-arg1".to_string()),
                    character_code: tf_str("linh_dan"),
                    character_display_name: tf_str("Linh Đan"),
                    role: tf_str("CHALLENGER"),
                    answer_mode: tf_str("SACRIFICE"),
                    note: tf_str("Bảo vệ đồng môn"),
                    required_knowledge: tf_str("Dược lý"),
                    required_competence: tf_str("Luyện đan"),
                    source: Some(default_source),
                }],
                source: Some(default_source),
            },
            source: Some(default_source),
        },
    ];

    // Phase 2: chapter_phase (3 chapters total matching target.chapter_count = 3)
    data.optional.chapter_phase.arcs = vec![
        ChapterPhaseArcItem {
            code: "te_nu_nguu_nhan-a1".to_string(),
            chapters: vec![
                ChapterPhaseChapterItem {
                    code: "te_nu_nguu_nhan-a1-c1".to_string(),
                    display_name: tf_str("Chương 1: Đêm Huyết Nguyệt"),
                    position: tf_u32(1),
                    summary: tf_str("Đêm trăng máu trên tế đàn."),
                    scenes: vec![ChapterPhaseSceneItem {
                        code: "te_nu_nguu_nhan-a1-c1-s1".to_string(),
                        display_name: tf_str("Cảnh 1: Trước giờ hành lễ"),
                        position: tf_u32(1),
                        summary: tf_str("Minh Hoàng tiến vào tế đàn."),
                        world_setting_code: tf_str("te_nu_nguu_nhan-w1"),
                        source: Some(default_source),
                    }],
                    source: Some(default_source),
                },
                ChapterPhaseChapterItem {
                    code: "te_nu_nguu_nhan-a1-c2".to_string(),
                    display_name: tf_str("Chương 2: Phá Giải Huyết Ấn"),
                    position: tf_u32(2),
                    summary: tf_str("Phá vỡ phong ấn cổ xưa."),
                    scenes: vec![ChapterPhaseSceneItem {
                        code: "te_nu_nguu_nhan-a1-c2-s1".to_string(),
                        display_name: tf_str("Cảnh 1: Giao chiến hộ pháp"),
                        position: tf_u32(1),
                        summary: tf_str("Hai người hợp sức đánh lui hộ pháp."),
                        world_setting_code: tf_str("te_nu_nguu_nhan-w2"),
                        source: Some(default_source),
                    }],
                    source: Some(default_source),
                },
            ],
        },
        ChapterPhaseArcItem {
            code: "te_nu_nguu_nhan-a2".to_string(),
            chapters: vec![ChapterPhaseChapterItem {
                code: "te_nu_nguu_nhan-a2-c1".to_string(),
                display_name: tf_str("Chương 3: Cổng Trời Thiên Sơn"),
                position: tf_u32(3),
                summary: tf_str("Đến chân núi Thiên Sơn."),
                scenes: vec![ChapterPhaseSceneItem {
                    code: "te_nu_nguu_nhan-a2-c1-s1".to_string(),
                    display_name: tf_str("Cảnh 1: Bậc thang khảo nghiệm"),
                    position: tf_u32(1),
                    summary: tf_str("Bước lên vạn bậc đá."),
                    world_setting_code: tf_str("te_nu_nguu_nhan-w1"),
                    source: Some(default_source),
                }],
                source: Some(default_source),
            }],
        },
    ];

    // Phase 3: world_building_phase
    data.optional.world_building_phase.world_settings = vec![
        WorldSettingItem {
            code: "te_nu_nguu_nhan-w1".to_string(),
            parent_code: TrackedField::unset(),
            display_name: tf_str("Thiên Sơn Đại Lục"),
            setting_type: tf_str("REGION"),
            description: tf_str("Đại lục rộng lớn nơi vạn tộc tu hành."),
            access: tf_str("PUBLIC"),
            lore_entries: vec![LoreEntryItem {
                code: "te_nu_nguu_nhan-l1".to_string(),
                display_name: tf_str("Huyết Nguyệt Cổ Điển"),
                lore_type: tf_str("CULTURE"),
                description: tf_str("Bí tịch ghi chép nghi thức huyết tế ngàn năm."),
                data: TrackedField::unset(),
                source: Some(default_source),
            }],
            source: Some(default_source),
            ..Default::default()
        },
        WorldSettingItem {
            code: "te_nu_nguu_nhan-w2".to_string(),
            parent_code: tf_str("te_nu_nguu_nhan-w1"),
            display_name: tf_str("Huyết Tế Cổ Đàn"),
            setting_type: tf_str("FACILITY"),
            description: tf_str("Tế đàn bằng đá đen nằm sâu trong thung lũng."),
            access: tf_str("RESTRICTED"),
            lore_entries: vec![],
            source: Some(default_source),
            ..Default::default()
        },
    ];

    // Phase 4: scene_config_phase
    data.optional.scene_config_phase.arcs = vec![
        SceneConfigArcItem {
            code: "te_nu_nguu_nhan-a1".to_string(),
            chapters: vec![
                SceneConfigChapterItem {
                    code: "te_nu_nguu_nhan-a1-c1".to_string(),
                    scenes: vec![SceneConfigSceneItem {
                        code: "te_nu_nguu_nhan-a1-c1-s1".to_string(),
                        world_setting_code: tf_str("te_nu_nguu_nhan-w1"),
                        style_preset_code: tf_str("cinematic_tense"),
                        narrator_config_code: tf_str("THIRD_PERSON_LIMITED"),
                        lore_entries: vec![SceneLoreEntryLink {
                            lore_entry_code: "te_nu_nguu_nhan-l1".to_string(),
                            usage: tf_str("FORESHADOW"),
                            reveal_to_reader: tf_str("PARTIAL"),
                            reveal_to_pov: tf_str("PARTIAL"),
                            source: Some(default_source),
                        }],
                        characters: vec![
                            SceneCharacterLink {
                                character_code: "minh_hoang".to_string(),
                                is_pov: tf_bool(true),
                                source: Some(default_source),
                            },
                            SceneCharacterLink {
                                character_code: "linh_dan".to_string(),
                                is_pov: tf_bool(false),
                                source: Some(default_source),
                            },
                        ],
                    }],
                },
                SceneConfigChapterItem {
                    code: "te_nu_nguu_nhan-a1-c2".to_string(),
                    scenes: vec![SceneConfigSceneItem {
                        code: "te_nu_nguu_nhan-a1-c2-s1".to_string(),
                        world_setting_code: tf_str("te_nu_nguu_nhan-w2"),
                        style_preset_code: tf_str("action_fast"),
                        narrator_config_code: tf_str("OMNISCIENT"),
                        lore_entries: vec![],
                        characters: vec![SceneCharacterLink {
                            character_code: "minh_hoang".to_string(),
                            is_pov: tf_bool(false),
                            source: Some(default_source),
                        }],
                    }],
                },
            ],
        },
        SceneConfigArcItem {
            code: "te_nu_nguu_nhan-a2".to_string(),
            chapters: vec![SceneConfigChapterItem {
                code: "te_nu_nguu_nhan-a2-c1".to_string(),
                scenes: vec![SceneConfigSceneItem {
                    code: "te_nu_nguu_nhan-a2-c1-s1".to_string(),
                    world_setting_code: tf_str("te_nu_nguu_nhan-w1"),
                    style_preset_code: tf_str("epic_grand"),
                    narrator_config_code: tf_str("FIRST_PERSON"),
                    lore_entries: vec![],
                    characters: vec![SceneCharacterLink {
                        character_code: "linh_dan".to_string(),
                        is_pov: tf_bool(true),
                        source: Some(default_source),
                    }],
                }],
            }],
        },
    ];

    // Phase 5.1: character_appearance_phase
    data.optional.character_appearance_phase.character = vec![CharacterAppearanceItem {
        code: "minh_hoang".to_string(),
        species: tf_str("HUMAN"),
        gender: CharacterGenderSection {
            identity: tf_str("MALE"),
            expression: tf_str("MALE"),
        },
        height_cm: tf_u32(182),
        hair: CharacterHairSection {
            color: tf_str("BLACK"),
            ..Default::default()
        },
        skin: CharacterSkinSection {
            tone: tf_str("FAIR"),
            ..Default::default()
        },
        outfit_style: tf_str("Huyền y kiếm khách"),
        distinguishing_marks: vec![DistinguishingMarkItem {
            code: Some("mark-1".to_string()),
            location: tf_str("Cổ tay trái"),
            content: tf_str("Vết sẹo hình kiếm"),
            source: Some(default_source),
        }],
        ..Default::default()
    }];

    // Phase 5.2: character_psychological_phase
    data.optional.character_psychological_phase.character = vec![CharacterPsychologicalItem {
        code: "minh_hoang".to_string(),
        core_trauma_and_belief: CoreTraumaAndBeliefSection {
            the_wound: tf_str("Chứng kiến gia tộc bị phản bội."),
            the_lie: tf_str("Chỉ có kẻ mạnh mới có quyền sống sót."),
            fundamental_fear: tf_str("Bị người thân tín bỏ rơi."),
        },
        cognitive_moral_framework: CognitiveMoralFrameworkSection {
            moral_alignment: tf_str("Chaotic Good"),
            moral_code: tf_str("Không bao giờ hy sinh kẻ yếu để cầu vinh."),
            cognitive_bias: tf_str("Xu hướng hoài nghi mọi người."),
            breaking_point: tf_str("Người vô tội bị đồ sát trước mắt."),
        },
        narrative_engine: NarrativeEngineSection {
            external_want: tf_str("Phá hủy huyết tế đàn và minh oan cho phụ thân."),
            internal_need: tf_str("Học cách tin tưởng đồng đội."),
            internal_conflict: tf_str("Hận thù gia tộc đối kháng với lòng trắc ẩn."),
        },
        behavioral_adaptation: BehavioralAdaptationSection {
            social_mask: SocialMaskSection {
                persona_type: tf_str("Lạnh lùng điềm tĩnh"),
                verbal_style: tf_str("Ngắn gọn dứt khoát"),
            },
            defense_mechanisms: DefenseMechanismsSection {
                primary_reaction: tf_str("Rút kiếm sẵn sàng chiến đấu"),
                stress_trigger: tf_str("Nhắc đến thảm kịch gia tộc"),
                coping_strategy: tf_str("Tập kiếm trong đêm thanh vắng"),
            },
        },
    }];

    // Phase 5.3: character_life_experience_phase
    data.optional.character_life_experience_phase.character = vec![CharacterLifeExperienceItem {
        code: "minh_hoang".to_string(),
        memories: vec![CharacterMemoryItem {
            code: Some("minh_hoang-m1".to_string()),
            source_scene_code: tf_str("te_nu_nguu_nhan-a1-c1-s1"),
            content: tf_str("Lời dặn cuối cùng của sư phụ trước đêm huyết nguyệt."),
            source: Some(default_source),
        }],
        competences: vec![CharacterCompetenceItem {
            code: "minh_hoang-k1".to_string(),
            domain_code: tf_str("SWORDPLAY"),
            level_code: tf_str("MASTER"),
            scene_code: tf_str("te_nu_nguu_nhan-a1-c2-s1"),
            display_name: tf_str("Thiên Sơn Kiếm Quyết"),
            description: tf_str("Kiếm pháp trấn phái tốc độ cực nhanh."),
            reliability: tf_str("HIGH"),
            experience: tf_str("10 năm khổ luyện"),
            capabilities: vec![tf_str("Phá giáp hộ thể")],
            limitations: vec![tf_str("Tiêu hao chân khí lớn")],
            source: Some(default_source),
        }],
    }];

    // Phase 6: relationship_phase
    data.optional.relationship_phase.relationship = vec![RelationshipPhaseItem {
        code: "te_nu_nguu_nhan-r1".to_string(),
        motif: RelationshipMotifSection {
            code: tf_str("comrades_in_arms"),
            event_blocks: vec![],
        },
        character: RelationshipCharacterPairSection {
            from_code: "minh_hoang".to_string(),
            to_code: "linh_dan".to_string(),
            scenes: vec![RelationshipSceneStateItem {
                scene_code: "te_nu_nguu_nhan-a1-c1-s1".to_string(),
                trust: tf_str("HIGH"),
                affection: tf_str("MODERATE"),
                hostility: tf_str("NONE"),
                fear: tf_str("NONE"),
                respect: tf_str("HIGH"),
                dependency: tf_str("MODERATE"),
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
// Acceptance Tests
// ============================================================================

/// phase4_sprint2_deep0-F0: Predecessor gate absent or baseline SHA drift check.
#[test]
fn phase4_sprint2_deep0_F0_predecessor_gate_and_baseline_check() {
    let mb_output = Command::new("git")
        .args(["merge-base", "--is-ancestor", BASELINE_COMMIT, "HEAD"])
        .output();
    let baseline_ok = match mb_output {
        Ok(out) if out.status.success() => true,
        _ => {
            let cat = Command::new("git")
                .args(["cat-file", "-e", &format!("{BASELINE_COMMIT}^{{commit}}")])
                .output();
            matches!(cat, Ok(out) if out.status.success())
        }
    };
    assert!(
        baseline_ok,
        "Baseline commit {} must be an ancestor of HEAD or exist in repository",
        BASELINE_COMMIT
    );

    let gates = [
        ("gate_phase3_sprint2_deep0", GATE_P3S2),
        ("gate_phase3_sprint5_deep2", GATE_P3S5),
        ("gate_phase3_sprint6_deep3", GATE_P3S6),
        ("gate_phase2_sprint6_deep2", GATE_P2S6),
        ("gate_phase1_sprint1_deep0", GATE_P1S1),
    ];

    for (name, sha) in gates {
        let cat_check = Command::new("git")
            .args(["cat-file", "-e", &format!("{}^{{commit}}", sha)])
            .output()
            .unwrap_or_else(|_| panic!("execute git cat-file for {}", name));
        assert!(
            cat_check.status.success(),
            "Predecessor gate commit {} ({}) must exist in repo",
            name,
            sha
        );
    }
}

/// TX-005 & T-ROLL-01: Collision code or foreign key failure rolls back TX-05 atomically,
/// restoring status to 'editing' without leaking rows.
#[tokio::test]
async fn TX_005_T_ROLL_01_atomic_rollback_on_collision_or_error() {
    let temp_dir = TempDir::new().unwrap();
    let (_db_path, pool) = setup_test_db(&temp_dir);
    let registry = Arc::new(DraftRuntimeRegistry::new());
    let draft_service = Arc::new(DraftService::new(pool.clone(), registry.clone()));
    let commit_service = Arc::new(CommitService::new(pool.clone(), registry.clone()));

    let draft_id = Uuid::now_v7();
    let session_id = "session_tx05_roll";
    seed_draft_session(&pool, draft_id, session_id, "editing");

    // Seal the valid full draft version
    let valid_data = build_valid_full_draft_fixture(ProvenanceSource::User);
    let json_str = serde_json::to_string(&valid_data).unwrap();
    let hash = fingerprint::state_data_hash(&valid_data).unwrap();

    draft_service
        .seal_version(&SealVersionParams {
            draft_session_id: draft_id,
            seal_key: "seal:full_v1".to_string(),
            reason: "manual_edit".to_string(),
            phase: 6,
            subphase: None,
            data_json: json_str,
            data_hash: hash,
            parent_seq: None,
            turn_id: None,
            chat_message_id: None,
        })
        .unwrap();

    // Inject conflicting code into arcs table: "te_nu_nguu_nhan-a1"
    {
        let conn = pool.get().unwrap();
        conn.execute(
            "INSERT INTO arcs (id, code, display_name, position) VALUES (?1, 'te_nu_nguu_nhan-a1', 'Conflicting Arc', 99)",
            rusqlite::params![Uuid::now_v7().to_string()],
        )
        .expect("insert conflicting arc");
    }

    // Call commit_draft -> MUST FAIL with CodeCollisionOnCommit
    let req = CommitDraftReq { draft_session_id: draft_id };
    let err = exec_commit_draft(&commit_service, &req, CallerContext::UiAction)
        .await
        .unwrap_err();

    assert_eq!(err.code, ErrorCode::CodeCollisionOnCommit);
    let details = err.details.expect("must contain details");
    assert!(details.to_string().contains("te_nu_nguu_nhan-a1"));

    // Verify rollback: draft_sessions status must be 'editing'
    let conn = pool.get().unwrap();
    let status: String = conn
        .query_row(
            "SELECT status FROM draft_sessions WHERE id = ?1",
            rusqlite::params![draft_id.to_string()],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(status, "editing", "Draft session status must be rolled back to editing");

    // Verify no partial records leaked into chapters or world_settings
    let ch_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM chapters", [], |row| row.get(0))
        .unwrap();
    assert_eq!(ch_count, 0, "No chapters leaked during rolled back TX-05");

    let ws_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM world_settings", [], |row| row.get(0))
        .unwrap();
    assert_eq!(ws_count, 0, "No world settings leaked during rolled back TX-05");
}

/// P4S2-PREVIEW: Preview calculates canConfirm = true when ready and 0 pending patches;
/// false when there are pending patches or missing required fields.
#[tokio::test]
async fn P4S2_PREVIEW_calculation_and_gating() {
    let temp_dir = TempDir::new().unwrap();
    let (_db_path, pool) = setup_test_db(&temp_dir);
    let registry = Arc::new(DraftRuntimeRegistry::new());
    let draft_service = Arc::new(DraftService::new(pool.clone(), registry.clone()));
    let commit_service = Arc::new(CommitService::new(pool.clone(), registry.clone()));

    let draft_id = Uuid::now_v7();
    let session_id = "session_preview";
    seed_draft_session(&pool, draft_id, session_id, "editing");

    // Case 1: Initially no sealed version -> not ready
    let preview_empty = exec_get_commit_confirmation_preview(
        &commit_service,
        &GetCommitPreviewReq { draft_session_id: draft_id },
    )
    .unwrap();
    assert!(!preview_empty.can_confirm);
    assert_eq!(preview_empty.validation.status, "blocked");

    // Seal valid full draft
    let valid_data = build_valid_full_draft_fixture(ProvenanceSource::User);
    let json_str = serde_json::to_string(&valid_data).unwrap();
    let hash = fingerprint::state_data_hash(&valid_data).unwrap();

    draft_service
        .seal_version(&SealVersionParams {
            draft_session_id: draft_id,
            seal_key: "seal:ready_v1".to_string(),
            reason: "manual_edit".to_string(),
            phase: 6,
            subphase: None,
            data_json: json_str,
            data_hash: hash,
            parent_seq: None,
            turn_id: None,
            chat_message_id: None,
        })
        .unwrap();

    // Case 2: Valid full draft and 0 pending patches -> can_confirm = true
    let preview_ready = exec_get_commit_confirmation_preview(
        &commit_service,
        &GetCommitPreviewReq { draft_session_id: draft_id },
    )
    .unwrap();
    assert!(preview_ready.can_confirm);
    assert_eq!(preview_ready.validation.status, "ready");
    assert_eq!(preview_ready.pending_patches.count, 0);
    assert_eq!(preview_ready.plan_summary.project_name, "Tế Nữ Ngưu Nhân");
    assert_eq!(preview_ready.plan_summary.character_count, 2);
    assert_eq!(preview_ready.plan_summary.arc_count, 2);
    assert_eq!(preview_ready.plan_summary.chapter_count, 3);
    assert_eq!(preview_ready.plan_summary.world_setting_count, 2);
    assert_eq!(preview_ready.plan_summary.lore_entry_count, 1);

    // Case 3: Insert 1 pending patch -> can_confirm becomes false
    {
        let conn = pool.get().unwrap();
        conn.execute(
            "INSERT INTO pending_patches (id, draft_session_id, base_version_seq, target_type, target_code, field_name, operation, new_value, status, created_at)
             VALUES (?1, ?2, 1, 'character', 'minh_hoang', 'notes', 'set', '\"Updated notes\"', 'pending', ?3)",
            rusqlite::params![Uuid::now_v7().to_string(), draft_id.to_string(), Utc::now().to_rfc3339()],
        ).unwrap();
    }

    let preview_with_pending = exec_get_commit_confirmation_preview(
        &commit_service,
        &GetCommitPreviewReq { draft_session_id: draft_id },
    )
    .unwrap();
    assert!(!preview_with_pending.can_confirm, "canConfirm must be false when pending patches exist");
    assert_eq!(preview_with_pending.pending_patches.count, 1);
    assert_eq!(preview_with_pending.validation.status, "blocked");
}

/// P4S2-COMMIT-SUCCESS: Successful commit creates all 21 relation tables topologically
/// with parent-child and foreign-key integrity.
#[tokio::test]
async fn P4S2_COMMIT_SUCCESS_creates_all_relations_topologically() {
    let temp_dir = TempDir::new().unwrap();
    let (_db_path, pool) = setup_test_db(&temp_dir);
    let registry = Arc::new(DraftRuntimeRegistry::new());
    let draft_service = Arc::new(DraftService::new(pool.clone(), registry.clone()));
    let commit_service = Arc::new(CommitService::new(pool.clone(), registry.clone()));

    let draft_id = Uuid::now_v7();
    let session_id = "session_commit_success";
    seed_draft_session(&pool, draft_id, session_id, "editing");

    // Seal valid full draft
    let valid_data = build_valid_full_draft_fixture(ProvenanceSource::User);
    let json_str = serde_json::to_string(&valid_data).unwrap();
    let hash = fingerprint::state_data_hash(&valid_data).unwrap();

    draft_service
        .seal_version(&SealVersionParams {
            draft_session_id: draft_id,
            seal_key: "seal:commit_ok".to_string(),
            reason: "manual_edit".to_string(),
            phase: 6,
            subphase: None,
            data_json: json_str,
            data_hash: hash,
            parent_seq: None,
            turn_id: None,
            chat_message_id: None,
        })
        .unwrap();

    // Commit draft
    let resp = exec_commit_draft(
        &commit_service,
        &CommitDraftReq { draft_session_id: draft_id },
        CallerContext::UiAction,
    )
    .await
    .unwrap();

    assert_eq!(resp.status, "committed");
    assert_eq!(resp.redirect_path, format!("/projects/{}", resp.project_id));

    let conn = pool.get().unwrap();
    let proj_id_str = resp.project_id.to_string();

    // 1. projects
    let p_status: String = conn
        .query_row("SELECT status FROM projects WHERE id = ?1", rusqlite::params![proj_id_str], |r| r.get(0))
        .unwrap();
    assert_eq!(p_status, "DRAFTING");

    // 2. project_genres
    let pg_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM project_genres WHERE project_id = ?1", rusqlite::params![proj_id_str], |r| r.get(0))
        .unwrap();
    assert_eq!(pg_count, 1);

    // 3. characters & 4. project_characters
    let pc_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM project_characters WHERE project_id = ?1", rusqlite::params![proj_id_str], |r| r.get(0))
        .unwrap();
    assert_eq!(pc_count, 2);

    // 5. character_motifs
    let cm_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM character_motifs", [], |r| r.get(0))
        .unwrap();
    assert!(cm_count >= 1);

    // 6. arcs
    let arc_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM arcs WHERE project_id = ?1", rusqlite::params![proj_id_str], |r| r.get(0))
        .unwrap();
    assert_eq!(arc_count, 2);

    // 7. arc_motifs
    let am_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM arc_motifs", [], |r| r.get(0))
        .unwrap();
    assert_eq!(am_count, 2);

    // 8. themes & 9. theme_arguments
    // The theme catalog is also seeded system-wide, so count only this project's themes.
    let th_count: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM themes t JOIN arcs a ON a.id = t.arc_id WHERE a.project_id = ?1",
            rusqlite::params![proj_id_str],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(th_count, 2);

    let th_arg_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM theme_arguments", [], |r| r.get(0))
        .unwrap();
    assert_eq!(th_arg_count, 2);

    // 10. chapters
    let ch_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM chapters", [], |r| r.get(0))
        .unwrap();
    assert_eq!(ch_count, 3);

    // 11. arc_event_blocks
    let aeb_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM arc_event_blocks", [], |r| r.get(0))
        .unwrap();
    assert_eq!(aeb_count, 2);

    // 12. world_settings & 13. lore_entries
    let ws_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM world_settings WHERE project_id = ?1", rusqlite::params![proj_id_str], |r| r.get(0))
        .unwrap();
    assert_eq!(ws_count, 2);

    let lore_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM lore_entries", [], |r| r.get(0))
        .unwrap();
    assert_eq!(lore_count, 1);

    // 14. scenes (content NULL, word_count 0, is_locked 0)
    let sc_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM scenes", [], |r| r.get(0))
        .unwrap();
    assert_eq!(sc_count, 3);

    let sc_dirty: i64 = conn
        .query_row("SELECT COUNT(*) FROM scenes WHERE content IS NOT NULL OR word_count <> 0 OR is_locked <> 0", [], |r| r.get(0))
        .unwrap();
    assert_eq!(sc_dirty, 0, "Scenes must have content = NULL, word_count = 0, is_locked = 0");

    // 15. scene_characters & 16. scene_lore_entries
    let sc_char_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM scene_characters", [], |r| r.get(0))
        .unwrap();
    assert!(sc_char_count >= 3);

    let sc_lore_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM scene_lore_entries", [], |r| r.get(0))
        .unwrap();
    assert_eq!(sc_lore_count, 1);

    // 17. character_appearances & marks
    let app_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM character_appearances", [], |r| r.get(0))
        .unwrap();
    assert_eq!(app_count, 1);

    let mark_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM character_distinguishing_marks", [], |r| r.get(0))
        .unwrap();
    assert_eq!(mark_count, 1);

    // 18. character_psychological
    let psych_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM character_psychological", [], |r| r.get(0))
        .unwrap();
    assert_eq!(psych_count, 1);

    // 19. character_memories
    let mem_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM character_memories", [], |r| r.get(0))
        .unwrap();
    assert_eq!(mem_count, 1);

    // 20. character_competences
    let comp_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM character_competences", [], |r| r.get(0))
        .unwrap();
    assert_eq!(comp_count, 1);

    // 21. relationships & states
    let rel_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM relationships", [], |r| r.get(0))
        .unwrap();
    assert_eq!(rel_count, 1);

    let state_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM relationship_states", [], |r| r.get(0))
        .unwrap();
    assert_eq!(state_count, 1);

    // 22. draft_sessions and sessions updated
    let ds_status: String = conn
        .query_row("SELECT status FROM draft_sessions WHERE id = ?1", rusqlite::params![draft_id.to_string()], |r| r.get(0))
        .unwrap();
    assert_eq!(ds_status, "committed");

    let sess_proj_id: String = conn
        .query_row("SELECT project_id FROM sessions WHERE id = ?1", rusqlite::params![session_id], |r| r.get(0))
        .unwrap();
    assert_eq!(sess_proj_id, proj_id_str);
}

/// P4S2-IDEMPOTENT: Calling commit on an already committed draft returns the same projectId.
#[tokio::test]
async fn P4S2_IDEMPOTENT_repeated_commit_returns_same_project_id() {
    let temp_dir = TempDir::new().unwrap();
    let (_db_path, pool) = setup_test_db(&temp_dir);
    let registry = Arc::new(DraftRuntimeRegistry::new());
    let draft_service = Arc::new(DraftService::new(pool.clone(), registry.clone()));
    let commit_service = Arc::new(CommitService::new(pool.clone(), registry.clone()));

    let draft_id = Uuid::now_v7();
    let session_id = "session_idem";
    seed_draft_session(&pool, draft_id, session_id, "editing");

    // Seal valid full draft
    let valid_data = build_valid_full_draft_fixture(ProvenanceSource::User);
    let json_str = serde_json::to_string(&valid_data).unwrap();
    let hash = fingerprint::state_data_hash(&valid_data).unwrap();

    draft_service
        .seal_version(&SealVersionParams {
            draft_session_id: draft_id,
            seal_key: "seal:commit_idem".to_string(),
            reason: "manual_edit".to_string(),
            phase: 6,
            subphase: None,
            data_json: json_str,
            data_hash: hash,
            parent_seq: None,
            turn_id: None,
            chat_message_id: None,
        })
        .unwrap();

    // Call 1
    let resp1 = exec_commit_draft(
        &commit_service,
        &CommitDraftReq { draft_session_id: draft_id },
        CallerContext::UiAction,
    )
    .await
    .unwrap();

    // Call 2 (Idempotency)
    let resp2 = exec_commit_draft(
        &commit_service,
        &CommitDraftReq { draft_session_id: draft_id },
        CallerContext::UiAction,
    )
    .await
    .unwrap();

    assert_eq!(resp1.project_id, resp2.project_id);
    assert_eq!(resp2.status, "committed");
    assert_eq!(resp2.redirect_path, format!("/projects/{}", resp1.project_id));
}

/// P4S2-REOPEN-ALLOWED: Reopen when no scenes have written content succeeds, transitioning draft to editing.
#[tokio::test]
async fn P4S2_REOPEN_ALLOWED_when_no_written_scenes() {
    let temp_dir = TempDir::new().unwrap();
    let (_db_path, pool) = setup_test_db(&temp_dir);
    let registry = Arc::new(DraftRuntimeRegistry::new());
    let draft_service = Arc::new(DraftService::new(pool.clone(), registry.clone()));
    let commit_service = Arc::new(CommitService::new(pool.clone(), registry.clone()));

    let draft_id = Uuid::now_v7();
    let session_id = "session_reopen_ok";
    seed_draft_session(&pool, draft_id, session_id, "editing");

    // Seal valid full draft and commit
    let valid_data = build_valid_full_draft_fixture(ProvenanceSource::User);
    let json_str = serde_json::to_string(&valid_data).unwrap();
    let hash = fingerprint::state_data_hash(&valid_data).unwrap();

    draft_service
        .seal_version(&SealVersionParams {
            draft_session_id: draft_id,
            seal_key: "seal:reopen_ok".to_string(),
            reason: "manual_edit".to_string(),
            phase: 6,
            subphase: None,
            data_json: json_str,
            data_hash: hash,
            parent_seq: None,
            turn_id: None,
            chat_message_id: None,
        })
        .unwrap();

    let commit_resp = exec_commit_draft(
        &commit_service,
        &CommitDraftReq { draft_session_id: draft_id },
        CallerContext::UiAction,
    )
    .await
    .unwrap();

    // Reopen draft
    let new_active_session = "active_session_reopened";
    let reopen_resp = exec_reopen_committed_draft(
        &commit_service,
        &ReopenCommittedDraftReq {
            project_id: commit_resp.project_id,
            active_session_id: new_active_session.to_string(),
        },
    )
    .await
    .unwrap();

    assert_eq!(reopen_resp.draft_session_id, draft_id);
    assert_eq!(reopen_resp.status, "editing");

    let conn = pool.get().unwrap();
    let (status, sess_id): (String, String) = conn
        .query_row(
            "SELECT status, session_id FROM draft_sessions WHERE id = ?1",
            rusqlite::params![draft_id.to_string()],
            |r| Ok((r.get(0)?, r.get(1)?)),
        )
        .unwrap();
    assert_eq!(status, "editing");
    assert_eq!(sess_id, new_active_session);
}

/// P4S2-REOPEN-FORBIDDEN: Reopen when scenes are written is rejected with DraftReopenForbidden.
#[tokio::test]
async fn P4S2_REOPEN_FORBIDDEN_when_scenes_written() {
    let temp_dir = TempDir::new().unwrap();
    let (_db_path, pool) = setup_test_db(&temp_dir);
    let registry = Arc::new(DraftRuntimeRegistry::new());
    let draft_service = Arc::new(DraftService::new(pool.clone(), registry.clone()));
    let commit_service = Arc::new(CommitService::new(pool.clone(), registry.clone()));

    let draft_id = Uuid::now_v7();
    let session_id = "session_reopen_forbidden";
    seed_draft_session(&pool, draft_id, session_id, "editing");

    // Seal valid full draft and commit
    let valid_data = build_valid_full_draft_fixture(ProvenanceSource::User);
    let json_str = serde_json::to_string(&valid_data).unwrap();
    let hash = fingerprint::state_data_hash(&valid_data).unwrap();

    draft_service
        .seal_version(&SealVersionParams {
            draft_session_id: draft_id,
            seal_key: "seal:reopen_forb".to_string(),
            reason: "manual_edit".to_string(),
            phase: 6,
            subphase: None,
            data_json: json_str,
            data_hash: hash,
            parent_seq: None,
            turn_id: None,
            chat_message_id: None,
        })
        .unwrap();

    let commit_resp = exec_commit_draft(
        &commit_service,
        &CommitDraftReq { draft_session_id: draft_id },
        CallerContext::UiAction,
    )
    .await
    .unwrap();

    // Case A: Write content directly to one scene
    {
        let conn = pool.get().unwrap();
        conn.execute(
            "UPDATE scenes SET content = 'Một đêm trăng sáng trên đỉnh Thiên Sơn...', word_count = 10 WHERE position = 1",
            [],
        )
        .unwrap();
    }

    let err_content = exec_reopen_committed_draft(
        &commit_service,
        &ReopenCommittedDraftReq {
            project_id: commit_resp.project_id,
            active_session_id: "sess_any".to_string(),
        },
    )
    .await
    .unwrap_err();
    assert_eq!(err_content.code, ErrorCode::DraftReopenForbidden);

    // Case B: Clear scene content, but add a scene_versions row
    {
        let conn = pool.get().unwrap();
        conn.execute("UPDATE scenes SET content = NULL, word_count = 0", []).unwrap();

        let sc_id: String = conn
            .query_row("SELECT id FROM scenes LIMIT 1", [], |r| r.get(0))
            .unwrap();
        conn.execute(
            "INSERT INTO scene_versions (id, parent_id, version_number, scene_source, content, created_at)
             VALUES (?1, ?2, 1, 'HUMAN_EDITED', 'Saved content in version', ?3)",
            rusqlite::params![Uuid::now_v7().to_string(), sc_id, Utc::now().to_rfc3339()],
        )
        .unwrap();
    }

    let err_version = exec_reopen_committed_draft(
        &commit_service,
        &ReopenCommittedDraftReq {
            project_id: commit_resp.project_id,
            active_session_id: "sess_any".to_string(),
        },
    )
    .await
    .unwrap_err();
    assert_eq!(err_version.code, ErrorCode::DraftReopenForbidden);
}

/// IPC-CMD-019..021: Verifies IPC commands execution and caller authorization.
#[tokio::test]
async fn IPC_CMD_019_through_021_authorization_and_invocation() {
    let temp_dir = TempDir::new().unwrap();
    let (_db_path, pool) = setup_test_db(&temp_dir);
    let registry = Arc::new(DraftRuntimeRegistry::new());
    let draft_service = Arc::new(DraftService::new(pool.clone(), registry.clone()));
    let commit_service = Arc::new(CommitService::new(pool.clone(), registry.clone()));

    let draft_id = Uuid::now_v7();
    let session_id = "session_ipc_auth";
    seed_draft_session(&pool, draft_id, session_id, "editing");

    // Seal valid full draft
    let valid_data = build_valid_full_draft_fixture(ProvenanceSource::User);
    let json_str = serde_json::to_string(&valid_data).unwrap();
    let hash = fingerprint::state_data_hash(&valid_data).unwrap();

    draft_service
        .seal_version(&SealVersionParams {
            draft_session_id: draft_id,
            seal_key: "seal:ipc".to_string(),
            reason: "manual_edit".to_string(),
            phase: 6,
            subphase: None,
            data_json: json_str,
            data_hash: hash,
            parent_seq: None,
            turn_id: None,
            chat_message_id: None,
        })
        .unwrap();

    // 1. IPC #19: preview
    let preview = exec_get_commit_confirmation_preview(
        &commit_service,
        &GetCommitPreviewReq { draft_session_id: draft_id },
    )
    .unwrap();
    assert!(preview.can_confirm);

    // 2. IPC #20: commit with AI caller must be rejected with UnauthorizedCaller
    let err_ai = exec_commit_draft(
        &commit_service,
        &CommitDraftReq { draft_session_id: draft_id },
        CallerContext::Ai,
    )
    .await
    .unwrap_err();
    assert_eq!(err_ai.code, ErrorCode::UnauthorizedCaller);

    // 3. IPC #20: commit with UiAction caller succeeds
    let commit_resp = exec_commit_draft(
        &commit_service,
        &CommitDraftReq { draft_session_id: draft_id },
        CallerContext::UiAction,
    )
    .await
    .unwrap();
    assert_eq!(commit_resp.status, "committed");

    // 4. IPC #21: reopen
    let reopen_resp = exec_reopen_committed_draft(
        &commit_service,
        &ReopenCommittedDraftReq {
            project_id: commit_resp.project_id,
            active_session_id: "new_session_123".to_string(),
        },
    )
    .await
    .unwrap();
    assert_eq!(reopen_resp.status, "editing");
}
