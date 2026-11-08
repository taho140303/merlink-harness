//! Integration and acceptance tests for `phase3_sprint1_deep0`
//! ("Implement planner, progress and cross-entity validation").
//!
//! Per the backend module wiring rule in `docs/plan/phase3_sprint1_deep0.md`:
//! - This sprint does not edit `src-tauri/src/domain/mod.rs` or `src-tauri/src/lib.rs`.
//! - Instead, this test target loads the 3 owned implementation modules (`workflow_planner.rs`,
//!   `progress_engine.rs`, `invariants.rs`) and predecessor modules (`business_flow.rs`,
//!   `code_allocator.rs`, `fingerprint.rs`, `word_count.rs`) directly via `#[path = "..."]`.
//!
//! Covered acceptance vectors:
//! - `P3S1-P`: Arc `146..260` yields writes `146..195`, `196..235`, `236..260` (with reads `None`, `186..195`, `226..235`).
//! - `P3S1-N`: `15` tasks at `P = 6` are rounds `6 + 6 + 3`, never one round of `15`; frozen `max_parallel` per plan.
//! - `P3S1-B`: 46-attribute mixed-source array counts once by `confirmed > ai_suggested > incomplete` precedence.
//! - `T-UNIT-01`: `BusinessFlowData` with the 4 excluded fields (`arc_tolerance`, `chapter_tolerance`,
//!   `word_per_chapter_tolerance`, `relationship.motif.event_blocks`) and mixed `user`, `user_edited`,
//!   `ai_generated`, `incomplete` arrays keeps denominator fixed at `46`, never increments numerator/denominator
//!   for the 4 excluded fields, classifies arrays with >= 1 valid `confirmed` element into `confirmed_count`,
//!   respects `confirmed > ai_suggested > incomplete`, and never double-counts any attribute into both numerators.
//! - `T-UNIT-03`: `plan_outline_runs(146, 260, 50, 10)` returns `[(None, 146..195), (Some(186..195), 196..235), (Some(226..235), 236..260)]`;
//!   `plan_phase` for 5 Arcs of lengths `30/55/65/50/65` with `unit = 30, max_parallel = 6` produces `11` tasks split
//!   into 2 rounds (`6` tasks and `5` tasks); `lookback >= cap` is rejected with `ValidationFailed`.
//! - `T-UNIT-04`: `validate_draft_for_commit` detects and returns exact error codes for missing `scenes[].world_setting_code`,
//!   dangling `world_setting_code`, chapter count mismatch vs `target.chapter_count`, duplicate `chapters.position` across Arcs,
//!   and invalid `is_pov` count per `narrator_config`, while keeping `world_setting_code` mandatory before commit
//!   outside the 46 progress attributes.
//! - `phase3_sprint1_deep0-F1`: Invalid `lookback >= cap` returns `ValidationFailed` (`P3S1-N`) with zero tasks and no side effects.
//! - `phase3_sprint1_deep0-F2`: Missing `world_setting_code` is flagged as a commit validation issue (`P3S1-C`),
//!   not a 47th progress attribute (`denominator` stays `46`).
//! - `phase3_sprint1_deep0-F0`: Predecessor gate absent or baseline SHA drift returns `CONTRACT_BLOCKED` before implementation runs.

#[path = "../src/domain/business_flow.rs"]
pub mod business_flow;

#[path = "../src/domain/code_allocator.rs"]
pub mod code_allocator;

#[path = "../src/domain/fingerprint.rs"]
pub mod fingerprint;

#[path = "../src/domain/word_count.rs"]
pub mod word_count;

#[path = "../src/domain/workflow_planner.rs"]
pub mod workflow_planner;

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
    pub use crate::workflow_planner;
}

use domain::business_flow::{
    ArcEventBlockItem, ArcMotifSection, ArcThemeSection, BehavioralAdaptationSection,
    BusinessFlowData, ChapterPhaseArcItem, ChapterPhaseChapterItem, ChapterPhaseSceneItem,
    CharacterAppearanceItem, CharacterCompetenceItem, CharacterGenderSection, CharacterHairSection,
    CharacterLifeExperienceItem, CharacterMemoryItem, CharacterPsychologicalItem,
    CharacterSkinSection, CognitiveMoralFrameworkSection, CoreTraumaAndBeliefSection, DomainError,
    LoreEntryItem, NarrativeEngineSection, OverviewArcItem, ProvenanceSource,
    RelationshipCharacterPairSection, RelationshipMotifEventBlockItem, RelationshipMotifSection,
    RelationshipPhaseItem, RelationshipSceneStateItem, RequireCharacterItem, RequireGenreItem,
    SceneCharacterLink, SceneConfigArcItem, SceneConfigChapterItem, SceneConfigSceneItem,
    SceneLoreEntryLink, SocialMaskSection, ThemeArgumentItem, ThemeResolutionSection, TrackedField,
    WorldSettingItem,
};
use domain::fingerprint::canonical_state;
use domain::invariants::{
    check_readiness_gate, validate_draft_for_commit, validate_scene_config, ValidationErrorCode,
};
use domain::progress_engine::{
    evaluate_attribute, evaluate_progress, AttributeProgressState, MasterAttributeId,
    EXCLUDED_PROTOTYPE_ATTRIBUTES, MASTER_ATTRIBUTES_LIST, MASTER_ATTRIBUTE_DENOMINATOR,
};
use domain::workflow_planner::{
    chunk_into_rounds, context_range, format_arc_chapter_task_identity,
    format_char_profile_task_identity, format_rel_matrix_task_identity,
    format_world_write_task_identity, plan_all_character_subphases, plan_arc_outline_runs,
    plan_chapter_phase_with_context, plan_outline_runs, plan_phase, plan_relationship_phase,
    plan_world_write_by_depth, split_balanced, OutlineWindow, PlannerError, WorkflowConfig,
    WorkflowTaskKind,
};
use std::path::PathBuf;
use std::process::Command;

const BASELINE_COMMIT: &str = "57a462efb953a665e3673b15e6cc33622a4f2d39";
const PREDECESSOR_P2S2_COMMIT: &str = "f69f1ab0e3d3d34853ddf887cac139cde6d64b0d";

/// Helper to validate predecessor gate (`gate_phase2_sprint2_deep1`) and baseline lineage (`phase3_sprint1_deep0-F0`).
pub fn verify_p3s1_dispatch_preconditions(
    expected_baseline: &str,
    actual_baseline: &str,
    p2s2_gate_accepted: bool,
) -> Result<(), DomainError> {
    if !p2s2_gate_accepted {
        return Err(DomainError::ContractBlocked {
            reason: "predecessor gate_phase2_sprint2_deep1 is absent or not ACCEPTED".to_string(),
        });
    }
    if expected_baseline != actual_baseline {
        return Err(DomainError::ContractBlocked {
            reason: format!(
                "baseline SHA drift detected: expected {expected_baseline}, got {actual_baseline}"
            ),
        });
    }
    Ok(())
}

/// Builds a synthetic, fully valid `BusinessFlowData` fixture where all 46 Master Attributes are populated
/// and all cross-entity commit invariants hold.
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

    // Phase 1: overview_phase (9 attributes)
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

    // Phase 2: chapter_phase (4 attributes; 3 chapters total across 2 Arcs matching target.chapter_count = 3)
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

    // Phase 3: world_building_phase (6 attributes)
    data.optional.world_building_phase.world_settings = vec![
        WorldSettingItem {
            code: "te_nu_nguu_nhan-w1".to_string(),
            parent_code: TrackedField::unset(),
            display_name: tf_str("Thiên Sơn Đại Lục"),
            setting_type: tf_str("CONTINENT"),
            description: tf_str("Đại lục rộng lớn nơi vạn tộc tu hành."),
            access: tf_str("OPEN"),
            lore_entries: vec![LoreEntryItem {
                code: "te_nu_nguu_nhan-l1".to_string(),
                display_name: tf_str("Huyết Nguyệt Cổ Điển"),
                lore_type: tf_str("SCRIPTURE"),
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
            setting_type: tf_str("SHRINE"),
            description: tf_str("Tế đàn bằng đá đen nằm sâu trong thung lũng."),
            access: tf_str("RESTRICTED"),
            lore_entries: vec![],
            source: Some(default_source),
            ..Default::default()
        },
    ];

    // Phase 4: scene_config_phase (4 attributes)
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

    // Phase 5.1: character_appearance_phase (6 attributes)
    data.optional.character_appearance_phase.character = vec![CharacterAppearanceItem {
        code: "minh_hoang".to_string(),
        species: tf_str("HUMAN"),
        gender: CharacterGenderSection {
            identity: tf_str("MALE"),
            expression: tf_str("MASCULINE"),
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
        ..Default::default()
    }];

    // Phase 5.2: character_psychological_phase (4 attributes)
    data.optional.character_psychological_phase.character = vec![CharacterPsychologicalItem {
        code: "minh_hoang".to_string(),
        core_trauma_and_belief: CoreTraumaAndBeliefSection {
            the_wound: tf_str("Chứng kiến gia tộc bị phản bội."),
            ..Default::default()
        },
        cognitive_moral_framework: CognitiveMoralFrameworkSection {
            moral_code: tf_str("Không bao giờ hy sinh kẻ yếu để cầu vinh."),
            ..Default::default()
        },
        narrative_engine: NarrativeEngineSection {
            external_want: tf_str("Phá hủy huyết tế đàn và minh oan cho phụ thân."),
            ..Default::default()
        },
        behavioral_adaptation: BehavioralAdaptationSection {
            social_mask: SocialMaskSection {
                persona_type: tf_str("Lạnh lùng điềm tĩnh"),
                verbal_style: tf_str("Ngắn gọn dứt khoát"),
            },
            ..Default::default()
        },
    }];

    // Phase 5.3: character_life_experience_phase (2 attributes)
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

    // Phase 6: relationship_phase (1 attribute)
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
            roles: vec![],
        },
        source: Some(default_source),
    }];

    data
}

#[test]
#[allow(non_snake_case)]
fn P3S1_P_arc_146_to_260_yields_exact_outline_windows() {
    let runs = plan_outline_runs(146, 260, 50, 10).expect("plan_outline_runs(146, 260, 50, 10) must succeed");
    assert_eq!(runs.len(), 3, "Arc 146..=260 must split into 3 sequential outline runs");

    let writes: Vec<(i32, i32)> = runs.iter().map(|r| r.write).collect();
    assert_eq!(writes, vec![(146, 195), (196, 235), (236, 260)]);

    let tuples: Vec<(Option<(i32, i32)>, (i32, i32))> =
        runs.iter().map(OutlineWindow::as_tuple).collect();
    assert_eq!(
        tuples,
        vec![
            (None, (146, 195)),
            (Some((186, 195)), (196, 235)),
            (Some((226, 235)), (236, 260)),
        ]
    );

    // Verify contiguity, non-overlap, and cap bound on every window
    let mut expected_next = 146;
    for (idx, run) in runs.iter().enumerate() {
        assert_eq!(run.run_index_in_arc, idx as u32);
        assert_eq!(run.write.0, expected_next);
        assert!(run.write.0 <= run.write.1);
        assert!(run.total_context_chapters() <= 50);
        if idx == 0 {
            assert_eq!(run.read, None, "First run in an Arc must never look back");
        } else {
            let (r_lo, r_hi) = run.read.expect("Subsequent runs with lookback > 0 must have read range");
            assert!(r_lo >= 146, "Read range must never cross before start of Arc");
            assert_eq!(r_hi, run.write.0 - 1);
            assert!(r_hi - r_lo + 1 <= 10);
        }
        expected_next = run.write.1 + 1;
    }
    assert_eq!(expected_next - 1, 260);
}

#[test]
#[allow(non_snake_case)]
fn P3S1_N_fifteen_tasks_at_P6_are_rounds_6_6_3_never_one_round_15() {
    // 15 tasks across 15 single-unit items (or one 15-unit item with unit=1) at P=6
    let items: Vec<(String, i32, i32)> = (1..=15)
        .map(|i| (format!("arc_{i}"), i, i))
        .collect();

    let plan = plan_phase(&items, 1, 6).expect("plan_phase for 15 tasks at P=6 must succeed");
    assert_eq!(plan.max_parallel, 6, "max_parallel (P) must be frozen on the plan");
    assert_eq!(plan.total_tasks(), 15);
    assert_eq!(plan.total_rounds(), 3);
    assert_eq!(
        plan.round_sizes(),
        vec![6, 6, 3],
        "15 tasks at P=6 must be partitioned into rounds of 6 + 6 + 3, never one round of 15"
    );

    // Also verify chunk_into_rounds directly and verify custom P (e.g. P=4 -> 4+4+4+3) to prove P is not hardcoded to 6
    let raw_tasks: Vec<u32> = (1..=15).collect();
    let chunks_p6 = chunk_into_rounds(&raw_tasks, 6).unwrap();
    assert_eq!(
        chunks_p6.iter().map(Vec::len).collect::<Vec<_>>(),
        vec![6, 6, 3]
    );

    let plan_p4 = plan_phase(&items, 1, 4).expect("plan_phase at P=4 must succeed");
    assert_eq!(plan_p4.max_parallel, 4);
    assert_eq!(plan_p4.round_sizes(), vec![4, 4, 4, 3]);

    // Verify deterministic task_index within each round (0..round.len())
    for round in &plan.rounds {
        for (expected_idx, task) in round.tasks.iter().enumerate() {
            assert_eq!(task.task_index, expected_idx);
            assert_eq!(task.round_index, round.round_index);
        }
    }
}

#[test]
#[allow(non_snake_case)]
fn P3S1_B_46_attribute_mixed_source_array_counts_once_by_confirmed_gt_ai_gt_incomplete() {
    let mut data = BusinessFlowData::new();

    // Populate `require.character` with a mix of Incomplete, AiGenerated, and User/UserEdited items
    data.require.character = vec![
        // Element 0: Incomplete (missing display_name/nickname, sex, motif)
        RequireCharacterItem {
            code: "char_inc".to_string(),
            ..Default::default()
        },
        // Element 1: AI-generated valid element
        RequireCharacterItem {
            code: "char_ai".to_string(),
            display_name: TrackedField::ai("Nhân Vật AI".to_string()),
            biological_sex: TrackedField::ai("FEMALE".to_string()),
            motif_code: TrackedField::ai("mage_mentor".to_string()),
            ..Default::default()
        },
        // Element 2: User-edited valid element on display_name & biological_sex, but AI on motif_code
        RequireCharacterItem {
            code: "char_mixed".to_string(),
            display_name: TrackedField::user_edited("Nhân Vật User".to_string()),
            biological_sex: TrackedField::user("MALE".to_string()),
            motif_code: TrackedField::ai("warrior_king".to_string()),
            ..Default::default()
        },
    ];

    let summary = evaluate_progress(&data);
    assert_eq!(summary.denominator, 46);
    assert_eq!(
        summary.confirmed_count + summary.ai_suggested_count + summary.incomplete_count,
        46
    );

    // `require.character.display_name` has 1 Confirmed element (char_mixed) and 1 AiSuggested (char_ai) and 1 Incomplete (char_inc).
    // Precedence `Confirmed > AiSuggested > Incomplete` means it counts ONCE as Confirmed (+1 to confirmed_count, 0 to ai_suggested_count).
    assert_eq!(
        summary.state_of(MasterAttributeId::RequireCharacterDisplayNameOrNickname),
        AttributeProgressState::Confirmed
    );
    // `require.character.biological_sex` also has 1 Confirmed element and 1 AiSuggested -> Confirmed
    assert_eq!(
        summary.state_of(MasterAttributeId::RequireCharacterBiologicalSex),
        AttributeProgressState::Confirmed
    );
    // `require.character.motif_code` only has AiGenerated elements (and 1 Incomplete) -> AiSuggested
    assert_eq!(
        summary.state_of(MasterAttributeId::RequireCharacterMotifCode),
        AttributeProgressState::AiSuggested
    );

    // Total counts: 2 Confirmed + 1 AiSuggested + 43 Incomplete = 46
    assert_eq!(summary.confirmed_count, 2);
    assert_eq!(summary.ai_suggested_count, 1);
    assert_eq!(summary.incomplete_count, 43);
}

#[test]
#[allow(non_snake_case)]
fn T_UNIT_01_master_46_attributes_and_composite_provenance_excludes_4_prototype_fields() {
    // 1. Start with a completely empty draft -> 0 Confirmed, 0 AiSuggested, 46 Incomplete
    let mut data = BusinessFlowData::new();
    let empty_summary = evaluate_progress(&data);
    assert_eq!(empty_summary.denominator, MASTER_ATTRIBUTE_DENOMINATOR);
    assert_eq!(empty_summary.total_count, 46);
    assert_eq!(empty_summary.confirmed_count, 0);
    assert_eq!(empty_summary.ai_suggested_count, 0);
    assert_eq!(empty_summary.incomplete_count, 46);
    assert!(!empty_summary.is_require_ready);

    // 2. Populate ONLY the 4 excluded fields (`arc_tolerance`, `chapter_tolerance`,
    //    `word_per_chapter_tolerance`, `relationship.motif.event_blocks`) with User/Ai values.
    //    They MUST NOT increase numerator or denominator (`50 - 4 = 46`).
    data.optional.overview_phase.project.target.arc_tolerance = TrackedField::user(2);
    data.optional
        .overview_phase
        .project
        .target
        .chapter_tolerance = TrackedField::user(10);
    data.optional
        .overview_phase
        .project
        .target
        .word_per_chapter_tolerance = TrackedField::ai(500);
    data.optional.relationship_phase.relationship.push(
        RelationshipPhaseItem {
            code: "rel_1".to_string(),
            motif: RelationshipMotifSection {
                code: TrackedField::user("rivals".to_string()),
                event_blocks: vec![RelationshipMotifEventBlockItem {
                    code: TrackedField::user("reb_1".to_string()),
                    summary: TrackedField::user("Biến cố quan hệ bị loại khỏi 46 thuộc tính".to_string()),
                }],
            },
            character: RelationshipCharacterPairSection {
                from_code: "c1".to_string(),
                to_code: "c2".to_string(),
                scenes: vec![],
                roles: vec![],
            },
            source: Some(ProvenanceSource::User),
        },
    );

    let after_excluded = evaluate_progress(&data);
    assert_eq!(after_excluded.denominator, 46);
    assert_eq!(
        after_excluded.confirmed_count, 0,
        "The 4 excluded prototype fields must never increment confirmed_count"
    );
    assert_eq!(
        after_excluded.ai_suggested_count, 0,
        "The 4 excluded prototype fields must never increment ai_suggested_count"
    );
    assert_eq!(after_excluded.incomplete_count, 46);
    for excluded_path in EXCLUDED_PROTOTYPE_ATTRIBUTES {
        assert!(
            after_excluded.state_by_path(excluded_path).is_none(),
            "Excluded attribute '{excluded_path}' must not appear in the 46 Master Attributes list"
        );
    }

    // 3. Build a full 46-attribute fixture all from AI -> 0 Confirmed, 46 AiSuggested, is_require_ready = true
    let mut full_ai = build_valid_full_draft_fixture(ProvenanceSource::AiGenerated);
    let ai_summary = evaluate_progress(&full_ai);
    assert_eq!(ai_summary.denominator, 46);
    assert_eq!(ai_summary.confirmed_count, 0);
    assert_eq!(ai_summary.ai_suggested_count, 46);
    assert_eq!(ai_summary.incomplete_count, 0);
    assert_eq!(ai_summary.confirmed_pct, 0);
    assert_eq!(ai_summary.ai_suggested_pct, 100);
    assert!(
        ai_summary.is_require_ready,
        "Phase 0 require is ready when all 10 attributes are non-Incomplete (including AiSuggested)"
    );

    // 4. Test Composite Provenance on multi-field composite elements (`event_blocks`, `themes.resolution`, `social_mask`):
    //    - In `full_ai.optional.overview_phase.arcs[0].themes.resolution`, set `resolution_type` to `User`
    //      while keeping `content` as `AiGenerated`, and clear `arcs[1].themes.resolution`.
    //      Because one required field of the composite resolution is still `AiGenerated`, the element is `AiSuggested`!
    full_ai.optional.overview_phase.arcs[1].themes.resolution = ThemeResolutionSection::default();
    full_ai.optional.overview_phase.arcs[0]
        .themes
        .resolution
        .resolution_type = TrackedField::user("TRIUMPH".to_string());
    full_ai.optional.overview_phase.arcs[0]
        .themes
        .resolution
        .content = TrackedField::ai("Chiến thắng nhưng còn AI source".to_string());

    assert_eq!(
        evaluate_attribute(&full_ai, MasterAttributeId::OverviewArcsThemesResolution),
        AttributeProgressState::AiSuggested,
        "Composite element with 1 User field and 1 AiGenerated required field must be classified as AiSuggested"
    );

    //    - Now upgrade `content` to `UserEdited` as well -> all required fields of `arcs[0].themes.resolution`
    //      are in `{User, UserEdited}`, so the element becomes `Confirmed`, and the array attribute becomes `Confirmed`!
    full_ai.optional.overview_phase.arcs[0]
        .themes
        .resolution
        .content = TrackedField::user_edited("Chiến thắng đã được người dùng duyệt".to_string());
    assert_eq!(
        evaluate_attribute(&full_ai, MasterAttributeId::OverviewArcsThemesResolution),
        AttributeProgressState::Confirmed,
        "Composite element where all required fields are User/UserEdited must be classified as Confirmed"
    );

    //    - Even if `arcs[1].themes.resolution` is populated with `AiGenerated`, the array attribute
    //      still evaluates to `Confirmed` because `Confirmed > AiSuggested > Incomplete`, and contributes
    //      +1 ONLY to `confirmed_count` (never to both numerators).
    full_ai.optional.overview_phase.arcs[1].themes.resolution = ThemeResolutionSection {
        resolution_type: TrackedField::ai("TRAGEDY".to_string()),
        content: TrackedField::ai("Bi kịch do AI gợi ý".to_string()),
    };
    let mixed_summary = evaluate_progress(&full_ai);
    assert_eq!(mixed_summary.denominator, 46);
    assert_eq!(mixed_summary.confirmed_count, 1);
    assert_eq!(mixed_summary.ai_suggested_count, 45);
    assert_eq!(mixed_summary.incomplete_count, 0);
    assert_eq!(
        mixed_summary.confirmed_count + mixed_summary.ai_suggested_count,
        46,
        "No attribute may ever be counted into both confirmed_count and ai_suggested_count"
    );
}

#[test]
#[allow(non_snake_case)]
fn T_UNIT_03_plan_outline_runs_and_plan_phase_for_5_arcs_match_spec() {
    // Part 1: `plan_outline_runs(146, 260, 50, 10)` -> 3 runs: `(None, 146..195)`, `(186..195, 196..235)`, `(226..235, 236..260)`
    let outline_runs = plan_outline_runs(146, 260, 50, 10).unwrap();
    let run_tuples: Vec<_> = outline_runs.iter().map(OutlineWindow::as_tuple).collect();
    assert_eq!(
        run_tuples,
        vec![
            (None, (146, 195)),
            (Some((186, 195)), (196, 235)),
            (Some((226, 235)), (236, 260)),
        ]
    );

    let named_runs = plan_arc_outline_runs("te_nu-a2", 146, 260, 50, 10).unwrap();
    assert_eq!(
        named_runs[0].task_identity(2, "").unwrap(),
        "p2:s0:outline_window:arc=te_nu-a2:ch=146..195"
    );
    assert_eq!(
        named_runs[1].task_identity(2, "").unwrap(),
        "p2:s0:outline_window:arc=te_nu-a2:ch=196..235"
    );
    assert_eq!(
        named_runs[2].task_identity(2, "").unwrap(),
        "p2:s0:outline_window:arc=te_nu-a2:ch=236..260"
    );

    // Part 2: 5 Arcs of lengths 30 / 55 / 65 / 50 / 65 chapters (total = 265 chapters) with `unit = 30, max_parallel = 6`
    // Arc 1 (len 30): 1..=30 -> 1 task: (1, 30)
    // Arc 2 (len 55): 31..=85 -> 2 tasks: (31, 58) [len 28], (59, 85) [len 27]
    // Arc 3 (len 65): 86..=150 -> 3 tasks: (86, 107) [len 22], (108, 129) [len 22], (130, 150) [len 21]
    // Arc 4 (len 50): 151..=200 -> 2 tasks: (151, 175) [len 25], (176, 200) [len 25]
    // Arc 5 (len 65): 201..=265 -> 3 tasks: (201, 222) [len 22], (223, 244) [len 22], (245, 265) [len 21]
    let five_arcs = vec![
        ("arc_1", 1, 30),
        ("arc_2", 31, 85),
        ("arc_3", 86, 150),
        ("arc_4", 151, 200),
        ("arc_5", 201, 265),
    ];

    let phase_plan = plan_phase(&five_arcs, 30, 6).expect("plan_phase for 5 Arcs must succeed");
    assert_eq!(phase_plan.unit, 30);
    assert_eq!(phase_plan.max_parallel, 6);
    assert_eq!(phase_plan.total_tasks(), 11, "1 + 2 + 3 + 2 + 3 = 11 tasks");
    assert_eq!(phase_plan.total_rounds(), 2);
    assert_eq!(
        phase_plan.round_sizes(),
        vec![6, 5],
        "11 tasks at max_parallel=6 must split into 2 rounds of 6 tasks and 5 tasks"
    );

    let round0_tuples: Vec<(&str, i32, i32)> = phase_plan.rounds[0]
        .tasks
        .iter()
        .map(|t| t.as_tuple())
        .collect();
    assert_eq!(
        round0_tuples,
        vec![
            ("arc_1", 1, 30),
            ("arc_2", 31, 58),
            ("arc_2", 59, 85),
            ("arc_3", 86, 107),
            ("arc_3", 108, 129),
            ("arc_3", 130, 150),
        ]
    );

    let round1_tuples: Vec<(&str, i32, i32)> = phase_plan.rounds[1]
        .tasks
        .iter()
        .map(|t| t.as_tuple())
        .collect();
    assert_eq!(
        round1_tuples,
        vec![
            ("arc_4", 151, 175),
            ("arc_4", 176, 200),
            ("arc_5", 201, 222),
            ("arc_5", 223, 244),
            ("arc_5", 245, 265),
        ]
    );

    // Verify context_range on chapter tasks (`[max(1, s - 5), min(265, e + 5)]`)
    let p4_plan = plan_chapter_phase_with_context(4, &five_arcs, 265, 30, 6, 5).unwrap();
    assert_eq!(
        p4_plan.rounds[0].tasks[0].chapter_context_range,
        Some((1, 35))
    );
    assert_eq!(
        p4_plan.rounds[0].tasks[1].chapter_context_range,
        Some((26, 63))
    );
    assert_eq!(
        p4_plan.rounds[1].tasks[4].chapter_context_range,
        Some((240, 265))
    );
    assert_eq!(
        p4_plan.rounds[0].tasks[0].task_identity.as_deref(),
        Some("p4:s0:scene_config:arc=arc_1:ch=1..30")
    );

    // Part 3: `lookback >= cap` is rejected with `ValidationFailed`
    let err_eq = plan_outline_runs(146, 260, 50, 50).unwrap_err();
    assert_eq!(err_eq.code(), "ValidationFailed");
    let err_gt = plan_outline_runs(146, 260, 50, 75).unwrap_err();
    assert_eq!(err_gt.code(), "ValidationFailed");
}

#[test]
#[allow(non_snake_case)]
fn T_UNIT_04_validate_draft_for_commit_detects_missing_world_setting_chapter_count_duplicate_pos_and_pov_errors(
) {
    // 0. Baseline valid draft passes commit validation with 0 issues
    let valid_draft = build_valid_full_draft_fixture(ProvenanceSource::User);
    let baseline_report = validate_draft_for_commit(&valid_draft, false);
    assert!(
        baseline_report.is_valid_for_commit,
        "Valid full draft fixture must pass commit validation, got issues: {:?}",
        baseline_report.issues
    );
    assert!(baseline_report.is_require_ready);
    assert!(baseline_report.issues.is_empty());

    // 1. Missing `scenes[].world_setting_code` (`MissingWorldSettingCode`) and dangling `world_setting_code` (`DanglingWorldSettingCode`)
    let mut draft_missing_ws = valid_draft.clone();
    draft_missing_ws.optional.chapter_phase.arcs[0].chapters[0].scenes[0].world_setting_code =
        TrackedField::unset();
    draft_missing_ws.optional.scene_config_phase.arcs[0].chapters[0].scenes[0].world_setting_code =
        TrackedField::unset();
    draft_missing_ws.optional.chapter_phase.arcs[0].chapters[1].scenes[0].world_setting_code =
        TrackedField::user("non_existent_ws_999".to_string());
    draft_missing_ws.optional.scene_config_phase.arcs[0].chapters[1].scenes[0].world_setting_code =
        TrackedField::user("non_existent_ws_999".to_string());

    let ws_report = validate_draft_for_commit(&draft_missing_ws, false);
    assert!(!ws_report.is_valid_for_commit);
    assert!(ws_report.has_error_code(ValidationErrorCode::MissingWorldSettingCode));
    assert!(ws_report.has_error_code(ValidationErrorCode::DanglingWorldSettingCode));

    // And yet Progress Engine still reports 46/46 Confirmed because `world_setting_code` is outside the 46 attributes!
    let ws_progress = evaluate_progress(&draft_missing_ws);
    assert_eq!(ws_progress.denominator, 46);
    assert_eq!(ws_progress.confirmed_count, 46);

    // 2. Chapter count mismatch against `target.chapter_count` (`ChapterCountMismatch`)
    let mut draft_ch_mismatch = valid_draft.clone();
    draft_ch_mismatch.require.project.target.chapter_count = TrackedField::user(10); // actual is 3
    let ch_report = validate_draft_for_commit(&draft_ch_mismatch, false);
    assert!(!ch_report.is_valid_for_commit);
    assert!(ch_report.has_error_code(ValidationErrorCode::ChapterCountMismatch));

    // 3. Duplicate `chapters.position` across Arcs (`DuplicateChapterPosition`)
    let mut draft_dup_pos = valid_draft.clone();
    // Set Arc 2 Chapter 1's global position to 2 (colliding with Arc 1 Chapter 2's position = 2)
    draft_dup_pos.optional.chapter_phase.arcs[1].chapters[0].position = TrackedField::user(2);
    let dup_pos_report = validate_draft_for_commit(&draft_dup_pos, false);
    assert!(!dup_pos_report.is_valid_for_commit);
    assert!(dup_pos_report.has_error_code(ValidationErrorCode::DuplicateChapterPosition));

    // 4. Invalid `is_pov` count per `narrator_config`:
    //    - Scene 1 has `THIRD_PERSON_LIMITED` (requires 1 POV) -> set both characters to `is_pov = true` (2 POVs)
    //    - Scene 2 has `OMNISCIENT` (requires 0 POV) -> set character to `is_pov = true` (1 POV)
    //    - Scene 3 has `MULTIPLE_POV` (requires >= 2 POV) -> set only 1 character with `is_pov = true`
    let mut draft_bad_pov = valid_draft.clone();
    draft_bad_pov.optional.scene_config_phase.arcs[0].chapters[0].scenes[0].characters[1].is_pov =
        TrackedField::user(true);
    draft_bad_pov.optional.scene_config_phase.arcs[0].chapters[1].scenes[0].characters[0].is_pov =
        TrackedField::user(true);
    draft_bad_pov.optional.scene_config_phase.arcs[1].chapters[0].scenes[0].narrator_config_code =
        TrackedField::user("MULTIPLE_POV".to_string());

    let pov_report = validate_draft_for_commit(&draft_bad_pov, false);
    assert!(!pov_report.is_valid_for_commit);
    let pov_issues = pov_report.issues_with_code(ValidationErrorCode::InvalidScenePovCount);
    assert_eq!(
        pov_issues.len(),
        3,
        "All 3 narrator_config POV violations (limited=2, omniscient=1, multiple_pov=1) must be caught"
    );

    // 5. Pending patches block commit (`PendingPatchesExist`) and missing competence `scene_code` (`CompetenceMissingSceneCode`)
    let mut draft_comp = valid_draft.clone();
    draft_comp.optional.character_life_experience_phase.character[0].competences[0].scene_code =
        TrackedField::unset();
    let comp_report = validate_draft_for_commit(&draft_comp, true);
    assert!(!comp_report.is_valid_for_commit);
    assert!(comp_report.has_error_code(ValidationErrorCode::PendingPatchesExist));
    assert!(comp_report.has_error_code(ValidationErrorCode::CompetenceMissingSceneCode));
}

#[test]
#[allow(non_snake_case)]
fn phase3_sprint1_deep0_F1_invalid_lookback_ge_cap_returns_validation_failed_and_no_tasks() {
    let fixture = build_valid_full_draft_fixture(ProvenanceSource::User);
    let before_bytes = canonical_state(&fixture).expect("canonical_state must succeed");

    // 1. Inject lookback == cap (50 >= 50)
    let err_equal = plan_outline_runs(1, 145, 50, 50).unwrap_err();
    assert_eq!(err_equal.code(), "ValidationFailed");
    assert!(
        err_equal.to_string().contains("ValidationFailed")
            && err_equal.to_string().contains("P3S1-N"),
        "Error message must include ValidationFailed and P3S1-N: {err_equal}"
    );

    // 2. Inject lookback > cap (60 > 50)
    let err_greater = plan_arc_outline_runs("te_nu_nguu_nhan-a1", 1, 145, 50, 60).unwrap_err();
    assert!(matches!(err_greater, PlannerError::ValidationFailed { .. }));

    // 3. Inject invalid WorkflowConfig with lookback >= cap
    let bad_cfg = WorkflowConfig {
        outline_cap: 30,
        outline_lookback: 30,
        ..Default::default()
    };
    let cfg_err = bad_cfg.validate().unwrap_err();
    assert_eq!(cfg_err.code(), "ValidationFailed");

    // 4. Verify caller fixture state is completely untouched (no partial state escape)
    let after_bytes = canonical_state(&fixture).unwrap();
    assert_eq!(before_bytes, after_bytes);
}

#[test]
#[allow(non_snake_case)]
fn phase3_sprint1_deep0_F2_missing_world_setting_code_is_commit_issue_not_47th_progress_attribute() {
    let mut draft = build_valid_full_draft_fixture(ProvenanceSource::User);
    // Strip world_setting_code from all scenes in both Phase 2 and Phase 4
    for arc in &mut draft.optional.chapter_phase.arcs {
        for ch in &mut arc.chapters {
            for sc in &mut ch.scenes {
                sc.world_setting_code = TrackedField::unset();
            }
        }
    }
    for arc in &mut draft.optional.scene_config_phase.arcs {
        for ch in &mut arc.chapters {
            for sc in &mut ch.scenes {
                sc.world_setting_code = TrackedField::unset();
            }
        }
    }

    let before_bytes = canonical_state(&draft).unwrap();

    // 1. Progress Engine MUST still have denominator = 46 and 46/46 Confirmed (not a 47th progress attribute!)
    let progress = evaluate_progress(&draft);
    assert_eq!(progress.denominator, 46);
    assert_eq!(progress.total_count, 46);
    assert_eq!(progress.attributes.len(), 46);
    assert_eq!(progress.confirmed_count, 46);
    assert_eq!(progress.incomplete_count, 0);
    assert!(
        progress
            .attributes
            .iter()
            .all(|a| !a.path.ends_with("world_setting_code")),
        "world_setting_code must not be a 47th progress attribute"
    );

    // 2. Commit validation (`validate_draft_for_commit` & `validate_scene_config`) MUST block commit with `P3S1-C` / `MissingWorldSettingCode`
    let scene_issues = validate_scene_config(&draft);
    assert_eq!(scene_issues.len(), 3);
    for issue in &scene_issues {
        assert_eq!(issue.code, ValidationErrorCode::MissingWorldSettingCode);
        assert_eq!(issue.classification, "P3S1-C");
    }

    let commit_report = validate_draft_for_commit(&draft, false);
    assert!(!commit_report.is_valid_for_commit);
    assert!(commit_report.has_error_code(ValidationErrorCode::MissingWorldSettingCode));
    for issue in commit_report.issues_with_code(ValidationErrorCode::MissingWorldSettingCode) {
        assert_eq!(issue.classification, "P3S1-C");
    }

    // 3. Also test Phase 3 hierarchical world_write planner, Phase 5 subphases, and Phase 6 relationship planner
    let ws_nodes = vec![
        ("te_nu-w2".to_string(), Some("te_nu-w1".to_string())),
        ("te_nu-w1".to_string(), None),
        ("te_nu-w3".to_string(), Some("te_nu-w1".to_string())),
    ];
    let depth_plans = plan_world_write_by_depth(&ws_nodes, 8, 6).unwrap();
    assert_eq!(depth_plans.len(), 2, "2 tree depths: root depth 0 and child depth 1");
    assert_eq!(
        depth_plans[0].rounds[0].tasks[0].task_identity.as_deref(),
        Some("p3:s0:world_write:depth=0:codes=te_nu-w1")
    );
    assert_eq!(
        depth_plans[1].rounds[0].tasks[0].task_identity.as_deref(),
        Some("p3:s0:world_write:depth=1:codes=te_nu-w2,te_nu-w3")
    );

    let char_subphases =
        plan_all_character_subphases(&["minh_hoang", "linh_dan"], 4, 6).unwrap();
    assert_eq!(
        char_subphases[0].rounds[0].tasks[0].task_identity.as_deref(),
        Some("p5:s1:char_profile:codes=linh_dan,minh_hoang")
    );
    assert_eq!(
        char_subphases[1].rounds[0].tasks[0].task_identity.as_deref(),
        Some("p5:s2:char_profile:codes=linh_dan,minh_hoang")
    );
    assert_eq!(
        char_subphases[2].rounds[0].tasks[0].task_identity.as_deref(),
        Some("p5:s3:char_profile:codes=linh_dan,minh_hoang")
    );

    let rel_plan =
        plan_relationship_phase(&[("minh_hoang", "linh_dan")], 10, 6).unwrap();
    assert_eq!(
        rel_plan.rounds[0].tasks[0].task_identity.as_deref(),
        Some("p6:s0:rel_matrix:pairs=minh_hoang>linh_dan")
    );

    // State remains completely unmodified
    assert_eq!(canonical_state(&draft).unwrap(), before_bytes);
}

#[test]
#[allow(non_snake_case)]
fn phase3_sprint1_deep0_F0_missing_predecessor_or_baseline_drift_is_contract_blocked() {
    // 1. Missing predecessor gate -> CONTRACT_BLOCKED
    let err_missing_gate =
        verify_p3s1_dispatch_preconditions(BASELINE_COMMIT, BASELINE_COMMIT, false).unwrap_err();
    assert!(matches!(err_missing_gate, DomainError::ContractBlocked { .. }));
    assert!(err_missing_gate.to_string().contains("CONTRACT_BLOCKED"));

    // 2. Baseline SHA drift -> CONTRACT_BLOCKED
    let drifted = "0000000000000000000000000000000000000000";
    let err_drift = verify_p3s1_dispatch_preconditions(BASELINE_COMMIT, drifted, true).unwrap_err();
    assert!(matches!(err_drift, DomainError::ContractBlocked { .. }));
    assert!(err_drift.to_string().contains("CONTRACT_BLOCKED"));

    // 3. Positive verification in the live git repository:
    //    - BASELINE_COMMIT (`57a462efb953a665e3673b15e6cc33622a4f2d39`) is an ancestor of HEAD
    //    - PREDECESSOR_P2S2_COMMIT (`f69f1ab0e3d3d34853ddf887cac139cde6d64b0d`) exists as a commit object
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf();

    let baseline_status = Command::new("git")
        .args(["merge-base", "--is-ancestor", BASELINE_COMMIT, "HEAD"])
        .current_dir(&repo_root)
        .status()
        .expect("git merge-base must execute");
    assert!(
        baseline_status.success(),
        "Worktree HEAD must descend from BASELINE_COMMIT {BASELINE_COMMIT}"
    );

    let pred_exists = Command::new("git")
        .args([
            "cat-file",
            "-e",
            &format!("{PREDECESSOR_P2S2_COMMIT}^{{commit}}"),
        ])
        .current_dir(&repo_root)
        .status()
        .expect("git cat-file must execute");
    assert!(
        pred_exists.success(),
        "Accepted predecessor commit {PREDECESSOR_P2S2_COMMIT} (phase2_sprint2_deep1) must exist in git repository"
    );

    assert!(verify_p3s1_dispatch_preconditions(BASELINE_COMMIT, BASELINE_COMMIT, true).is_ok());

    // Exercise remaining public re-exports to guarantee clean compilation without warnings
    assert_eq!(MASTER_ATTRIBUTES_LIST.len(), 46);
    assert_eq!(split_balanced(1, 55, 30).unwrap(), vec![(1, 28), (29, 55)]);
    assert_eq!(context_range(1, 30, 100, 5).unwrap(), (1, 35));
    assert_eq!(
        format_arc_chapter_task_identity(3, WorkflowTaskKind::WorldProbe, "a1", 1, 30).unwrap(),
        "p3:s0:world_probe:arc=a1:ch=1..30"
    );
    assert_eq!(
        format_world_write_task_identity(1, &["w2", "w1"]).unwrap(),
        "p3:s0:world_write:depth=1:codes=w1,w2"
    );
    assert_eq!(
        format_char_profile_task_identity(1, &["c1"]).unwrap(),
        "p5:s1:char_profile:codes=c1"
    );
    assert_eq!(
        format_rel_matrix_task_identity(&[("c1", "c2")]).unwrap(),
        "p6:s0:rel_matrix:pairs=c1>c2"
    );
    let empty_ready = check_readiness_gate(&BusinessFlowData::new());
    assert!(!empty_ready.is_empty());
}
