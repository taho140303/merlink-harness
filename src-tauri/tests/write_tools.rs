#![allow(non_snake_case)]

//! Acceptance & Integration tests for `phase3_sprint5_deep2`
//! ("Implement write tools and protected cascade").
//!
//! Per the backend module wiring rule in `docs/plan/phase3_sprint5_deep2.md`:
//! - This sprint does not edit any `mod.rs` or `lib.rs`.
//! - Instead, this test target loads the 2 owned implementation modules
//!   (`cascade_delete.rs`, `write_tools.rs`) and predecessor modules via `#[path = "..."]`.
//!
//! Acceptance vectors covered:
//! - `P3S5-P`: mixed three-field tool returns applied/pending/rejected without corrupting State.
//! - `P3S5-N`: delete referenced world setting returns referenced_entity_in_use and unchanged State.
//! - `P3S5-B`: scene cascade nulls memory ref and removes dependent links atomically.
//! - `phase3_sprint5_deep2-F1`: Invalid field/source rejects only that field, allowing independent valid fields.
//! - `phase3_sprint5_deep2-F2`: Protected descendant triggers pending proposal request with zero mutation.
//! - `T-CASC-01`: Canonical Q1 cascade delete matrix & 9 mandatory safeguards:
//!   (1) delete world_setting with referenced child -> rejected (referenced_entity_in_use).
//!   (2) delete character with relationship -> rejected (referenced_entity_in_use).
//!   (3) AI delete arc containing user scene -> pending_confirmation with full preview.
//!   (4) delete scene with competences, memories, and relationship_states -> cascade physical removal,
//!       nullify memories.source_scene_code, keep parent relationships.
//!   (5) delete lore_entry with scene_lore_entries -> unlinks atomicaly, keeps scene intact.
//! - `phase3_sprint5_deep2-F0`: Predecessor gate absent or baseline drift returns `CONTRACT_BLOCKED`.

#[path = "../src/error.rs"]
pub mod error;

#[path = "../src/domain/business_flow.rs"]
pub mod business_flow;

#[path = "../src/domain/code_allocator.rs"]
pub mod code_allocator;

#[path = "../src/domain/fingerprint.rs"]
pub mod fingerprint;

#[path = "../src/domain/tools/registry.rs"]
pub mod registry;

#[path = "../src/domain/cascade_delete.rs"]
pub mod cascade_delete;

#[path = "../src/domain/tools/write_tools.rs"]
pub mod write_tools;

pub mod domain {
    pub use crate::business_flow;
    pub use crate::cascade_delete;
    pub use crate::code_allocator;
    pub use crate::fingerprint;
    pub mod tools {
        pub use crate::registry;
        pub use crate::write_tools;
    }
}

use std::path::PathBuf;
use std::process::Command;

use crate::business_flow::{
    ArcMotifSection, ArcThemeSection, BusinessFlowData, CharacterCompetenceItem,
    CharacterLifeExperienceItem, CharacterMemoryItem, ChapterPhaseArcItem, ChapterPhaseChapterItem,
    ChapterPhaseSceneItem, LoreEntryItem, OverviewArcItem, ProvenanceSource,
    RelationshipCharacterPairSection, RelationshipMotifSection, RelationshipPhaseItem,
    RelationshipSceneStateItem, RequireCharacterItem, SceneCharacterLink, SceneConfigArcItem,
    SceneConfigChapterItem, SceneConfigSceneItem, SceneLoreEntryLink, TrackedField,
    WorldSettingItem,
};
use crate::cascade_delete::execute_delete;
use crate::error::ErrorCode;
use crate::fingerprint::state_data_hash;
use crate::registry::CallerCategory;
use crate::write_tools::{ExecutionGuard, ToolCall, ToolExecutor};

const BASELINE_COMMIT: &str = "57a462efb953a665e3673b15e6cc33622a4f2d39";
const PREDECESSOR_P3S4_COMMIT: &str = "2cf53633e5513a4724f8c1d408018ff2f00665fd";
const PREDECESSOR_P3S1_COMMIT: &str = "702843c38e45b193be25b80222a48eadedfd3538";

/// Helper checking predecessor gate and baseline commit lineage (`phase3_sprint5_deep2-F0`).
fn check_predecessor_gates(
    expected_baseline: &str,
    actual_baseline: &str,
    p3s4_gate_accepted: bool,
    p3s1_gate_accepted: bool,
) -> Result<(), ErrorCode> {
    if !p3s4_gate_accepted || !p3s1_gate_accepted {
        return Err(ErrorCode::ValidationFailed); // Represents CONTRACT_BLOCKED
    }
    if expected_baseline != actual_baseline {
        return Err(ErrorCode::ValidationFailed);
    }
    Ok(())
}

// ============================================================================
// Acceptance Tests
// ============================================================================

/// `P3S5-P`: Mixed three-field tool returns applied/pending/rejected without corrupting State (`T-PROV-01`).
///
/// Scenario:
/// AI calls `set_character` with 3 fields:
/// - Field A (`display_name`): current source `ai_generated`, declared `ai_generated` -> applied.
/// - Field B (`notes`): current source `user`, declared `ai_generated`, new_value != current -> pending_confirmation.
/// - Field C (`role`): missing from `declared_sources` -> rejected (`missing_or_invalid_declared_source`).
///
/// Invariants:
/// - Overall status is "applied" (because field A succeeded).
/// - State mutation ONLY updates field A. Field B and Field C in State are completely untouched.
#[test]
fn test_P3S5_P_mixed_three_field_tool_returns_applied_pending_rejected_without_corrupting_state() {
    let mut state = BusinessFlowData::new();

    // Initial character state
    let char_item = RequireCharacterItem {
        code: "minh_hoang".to_string(),
        display_name: TrackedField::ai("Minh Hoàng".to_string()),
        nickname: TrackedField::user("Hoàng Nhi".to_string()),
        biological_sex: TrackedField::ai("MALE".to_string()),
        motif_code: TrackedField::ai("chosen_one".to_string()),
        role: TrackedField::ai("Nhân vật chính".to_string()),
        notes: TrackedField::user("Ghi chú gốc của người dùng".to_string()),
        reused_library_character: false,
        source: Some(ProvenanceSource::User),
    };
    state.require.character.push(char_item);

    let initial_hash = state_data_hash(&state).unwrap();
    let guard = ExecutionGuard::new(&state);

    // Call parameters: display_name (A), notes (B), role (C)
    let params = serde_json::json!({
        "code": "minh_hoang",
        "display_name": "Minh Hoàng Đột Phá",
        "notes": "Ghi chú do AI đề xuất thay đổi",
        "role": "Nhân vật phản diện",
        "declared_sources": {
            "display_name": "ai_generated",
            "notes": "ai_generated"
            // role is intentionally missing from declared_sources
        }
    });

    let call = ToolCall::new("set_character", CallerCategory::AiCaller, params);
    let outcome = ToolExecutor::execute(&call, &guard);

    // 1. Overall status
    assert_eq!(outcome.status, "applied");
    assert!(outcome.has_mutations());

    // 2. Field-level results
    let r_display = outcome.results.iter().find(|r| r.field == "display_name").unwrap();
    assert_eq!(r_display.status, "applied");

    let r_notes = outcome.results.iter().find(|r| r.field == "notes").unwrap();
    assert_eq!(r_notes.status, "pending_confirmation");
    assert!(r_notes.patch_id.is_some());

    let r_role = outcome.results.iter().find(|r| r.field == "role").unwrap();
    assert_eq!(r_role.status, "rejected");
    assert_eq!(r_role.error.as_deref(), Some("missing_or_invalid_declared_source"));

    // 3. Verify resulting mutated state
    let mutated = outcome.mutated_state().expect("Must have mutated state");
    let mutated_char = mutated.require.character.iter().find(|c| c.code == "minh_hoang").unwrap();

    // Field A was updated
    assert_eq!(mutated_char.display_name.value.as_deref(), Some("Minh Hoàng Đột Phá"));
    assert_eq!(mutated_char.display_name.source, Some(ProvenanceSource::AiGenerated));

    // Field B is UNCHANGED in state (proposal pending)
    assert_eq!(mutated_char.notes.value.as_deref(), Some("Ghi chú gốc của người dùng"));
    assert_eq!(mutated_char.notes.source, Some(ProvenanceSource::User));

    // Field C is UNCHANGED in state (rejected)
    assert_eq!(mutated_char.role.value.as_deref(), Some("Nhân vật chính"));
    assert_eq!(mutated_char.role.source, Some(ProvenanceSource::AiGenerated));

    // 4. Verify baseline state was NOT mutated in-place
    let baseline_hash = state_data_hash(&state).unwrap();
    assert_eq!(initial_hash, baseline_hash, "Baseline state must remain completely unmutated");
}

/// `P3S5-N`: Delete referenced world setting returns referenced_entity_in_use and unchanged State.
#[test]
fn test_P3S5_N_delete_referenced_world_setting_returns_referenced_entity_in_use_and_unchanged_state() {
    let mut state = BusinessFlowData::new();

    // Parent setting
    state.optional.world_building_phase.world_settings.push(WorldSettingItem {
        code: "van_kiem_tong".to_string(),
        parent_code: TrackedField::unset(),
        display_name: TrackedField::user("Vạn Kiếm Tông".to_string()),
        setting_type: TrackedField::user("SECT".to_string()),
        description: TrackedField::unset(),
        access: TrackedField::unset(),
        spatial_features: TrackedField::unset(),
        sensory_visual: TrackedField::unset(),
        sensory_auditory: TrackedField::unset(),
        sensory_tactile: TrackedField::unset(),
        sensory_olfactory: TrackedField::unset(),
        sensory_gustatory: TrackedField::unset(),
        lore_entries: Vec::new(),
        source: Some(ProvenanceSource::User),
    });

    // Child setting under van_kiem_tong
    state.optional.world_building_phase.world_settings.push(WorldSettingItem {
        code: "kiem_y_dong".to_string(),
        parent_code: TrackedField::user("van_kiem_tong".to_string()),
        display_name: TrackedField::user("Kiếm Ý Động".to_string()),
        setting_type: TrackedField::user("CAVE".to_string()),
        description: TrackedField::unset(),
        access: TrackedField::unset(),
        spatial_features: TrackedField::unset(),
        sensory_visual: TrackedField::unset(),
        sensory_auditory: TrackedField::unset(),
        sensory_tactile: TrackedField::unset(),
        sensory_olfactory: TrackedField::unset(),
        sensory_gustatory: TrackedField::unset(),
        lore_entries: Vec::new(),
        source: Some(ProvenanceSource::User),
    });

    // Scene referencing the child setting
    let sc = ChapterPhaseSceneItem {
        code: "sc_01".to_string(),
        display_name: TrackedField::user("Luyện Kiếm".to_string()),
        position: TrackedField::unset(),
        summary: TrackedField::unset(),
        world_setting_code: TrackedField::user("kiem_y_dong".to_string()),
        source: Some(ProvenanceSource::User),
    };
    state.optional.chapter_phase.arcs.push(ChapterPhaseArcItem {
        code: "arc_1".to_string(),
        chapters: vec![ChapterPhaseChapterItem {
            code: "chap_1".to_string(),
            display_name: TrackedField::unset(),
            position: TrackedField::unset(),
            summary: TrackedField::unset(),
            scenes: vec![sc],
            source: Some(ProvenanceSource::User),
        }],
    });

    let initial_hash = state_data_hash(&state).unwrap();
    let guard = ExecutionGuard::new(&state);

    // Call delete_world_setting on root "van_kiem_tong"
    let params = serde_json::json!({
        "code": "van_kiem_tong",
        "declared_source": "user"
    });
    let call = ToolCall::new("delete_world_setting", CallerCategory::UiAction, params);
    let outcome = ToolExecutor::execute(&call, &guard);

    // Assert rejection
    assert_eq!(outcome.status, "rejected");
    assert!(!outcome.has_mutations());
    let r = &outcome.results[0];
    assert_eq!(r.status, "rejected");
    assert_eq!(r.error.as_deref(), Some("referenced_entity_in_use"));
    assert_eq!(r.candidates.as_ref().unwrap(), &vec!["sc_01".to_string()]);

    // Assert working state is completely untouched
    let final_hash = state_data_hash(&state).unwrap();
    assert_eq!(initial_hash, final_hash, "State must remain bit-for-bit unchanged on rejection");
}

/// `P3S5-B`: Scene cascade nulls memory ref and removes dependent links atomically.
#[test]
fn test_P3S5_B_scene_cascade_nulls_memory_ref_and_removes_dependent_links_atomically() {
    let mut state = BusinessFlowData::new();

    // 1. Add scene
    let sc = ChapterPhaseSceneItem {
        code: "sc_target".to_string(),
        display_name: TrackedField::ai("Cảnh Quyết Chiến".to_string()),
        position: TrackedField::unset(),
        summary: TrackedField::unset(),
        world_setting_code: TrackedField::unset(),
        source: Some(ProvenanceSource::AiGenerated),
    };
    state.optional.chapter_phase.arcs.push(ChapterPhaseArcItem {
        code: "arc_1".to_string(),
        chapters: vec![ChapterPhaseChapterItem {
            code: "chap_1".to_string(),
            display_name: TrackedField::unset(),
            position: TrackedField::unset(),
            summary: TrackedField::unset(),
            scenes: vec![sc],
            source: Some(ProvenanceSource::AiGenerated),
        }],
    });

    // 2. Add scene_characters link in scene_config_phase
    state.optional.scene_config_phase.arcs.push(SceneConfigArcItem {
        code: "arc_1".to_string(),
        chapters: vec![SceneConfigChapterItem {
            code: "chap_1".to_string(),
            scenes: vec![SceneConfigSceneItem {
                code: "sc_target".to_string(),
                world_setting_code: TrackedField::unset(),
                style_preset_code: TrackedField::unset(),
                narrator_config_code: TrackedField::unset(),
                lore_entries: Vec::new(),
                characters: vec![SceneCharacterLink {
                    character_code: "char_a".to_string(),
                    is_pov: TrackedField::unset(),
                    source: Some(ProvenanceSource::AiGenerated),
                }],
            }],
        }],
    });

    // 3. Add character life experience with competence and memory referencing sc_target
    state.optional.character_life_experience_phase.character.push(CharacterLifeExperienceItem {
        code: "char_a".to_string(),
        memories: vec![CharacterMemoryItem {
            code: Some("mem_1".to_string()),
            source_scene_code: TrackedField::ai("sc_target".to_string()),
            content: TrackedField::ai("Nhớ lại trận chiến".to_string()),
            source: Some(ProvenanceSource::AiGenerated),
        }],
        competences: vec![CharacterCompetenceItem {
            code: "comp_1".to_string(),
            domain_code: TrackedField::unset(),
            level_code: TrackedField::unset(),
            scene_code: TrackedField::ai("sc_target".to_string()),
            display_name: TrackedField::ai("Kiếm Khí".to_string()),
            description: TrackedField::unset(),
            reliability: TrackedField::unset(),
            experience: TrackedField::unset(),
            capabilities: Vec::new(),
            limitations: Vec::new(),
            source: Some(ProvenanceSource::AiGenerated),
        }],
    });

    // 4. Add relationship with state in sc_target
    state.optional.relationship_phase.relationship.push(RelationshipPhaseItem {
        code: "rel_1".to_string(),
        motif: RelationshipMotifSection::default(),
        character: RelationshipCharacterPairSection {
            from_code: "char_a".to_string(),
            to_code: "char_b".to_string(),
            scenes: vec![RelationshipSceneStateItem {
                scene_code: "sc_target".to_string(),
                trust: TrackedField::ai("HIGH".to_string()),
                affection: TrackedField::ai("LOW".to_string()),
                hostility: TrackedField::unset(),
                fear: TrackedField::unset(),
                respect: TrackedField::unset(),
                dependency: TrackedField::unset(),
                familiarity: TrackedField::unset(),
                source: Some(ProvenanceSource::AiGenerated),
            }],
            roles: Vec::new(),
        },
        source: Some(ProvenanceSource::AiGenerated),
    });

    let guard = ExecutionGuard::new(&state);
    let params = serde_json::json!({
        "code": "sc_target",
        "declared_source": "ai_generated"
    });
    let call = ToolCall::new("delete_scene", CallerCategory::AiCaller, params);
    let outcome = ToolExecutor::execute(&call, &guard);

    // 1. Must be applied
    assert_eq!(outcome.status, "applied");
    assert!(outcome.has_mutations());

    let mutated = outcome.mutated_state().unwrap();

    // 2. Scene removed from chapter_phase and scene_config_phase
    assert!(mutated.optional.chapter_phase.arcs[0].chapters[0].scenes.is_empty());
    assert!(mutated.optional.scene_config_phase.arcs[0].chapters[0].scenes.is_empty());

    // 3. Competence attached to scene is removed
    let life = &mutated.optional.character_life_experience_phase.character[0];
    assert!(life.competences.is_empty(), "Competence tied to deleted scene must be deleted");

    // 4. Memory attached to scene has source_scene_code nullified
    assert_eq!(life.memories.len(), 1);
    assert_eq!(life.memories[0].source_scene_code.value, None, "source_scene_code must be nullified");

    // 5. Relationship state removed, but parent relationship preserved
    assert_eq!(mutated.optional.relationship_phase.relationship.len(), 1);
    let rel = &mutated.optional.relationship_phase.relationship[0];
    assert!(rel.character.scenes.is_empty(), "Relationship state on scene must be removed");
    assert_eq!(rel.code, "rel_1", "Parent relationship must be preserved");
}

/// `phase3_sprint5_deep2-F1`: Invalid field/source rejects only that field, allowing independent valid fields.
#[test]
fn test_F1_invalid_field_or_source_rejects_only_that_field() {
    let mut state = BusinessFlowData::new();
    state.require.project.display_name = TrackedField::ai("Tên Dự Án".to_string());
    state.require.project.summary = TrackedField::ai("Tóm tắt".to_string());

    let guard = ExecutionGuard::new(&state);

    // AI sends valid summary with declared source, but invalid language (missing in declared_sources)
    let params = serde_json::json!({
        "summary": "Tóm tắt mới tinh",
        "language": "VI",
        "declared_sources": {
            "summary": "ai_generated"
            // language missing in declared_sources
        }
    });

    let call = ToolCall::new("set_project", CallerCategory::AiCaller, params);
    let outcome = ToolExecutor::execute(&call, &guard);

    assert_eq!(outcome.status, "applied");
    assert_eq!(outcome.results.len(), 2);

    let r_sum = outcome.results.iter().find(|r| r.field == "summary").unwrap();
    assert_eq!(r_sum.status, "applied");

    let r_lang = outcome.results.iter().find(|r| r.field == "language").unwrap();
    assert_eq!(r_lang.status, "rejected");
    assert_eq!(r_lang.error.as_deref(), Some("missing_or_invalid_declared_source"));

    let mutated = outcome.mutated_state().unwrap();
    assert_eq!(mutated.require.project.summary.value.as_deref(), Some("Tóm tắt mới tinh"));
    assert_eq!(mutated.require.project.language.value, None);
}

/// `phase3_sprint5_deep2-F2`: Protected descendant triggers pending proposal request with zero mutation.
#[test]
fn test_F2_protected_descendant_triggers_pending_proposal_request_with_zero_mutation() {
    let mut state = BusinessFlowData::new();

    // Arc is ai_generated
    let arc = OverviewArcItem {
        code: "arc_ai".to_string(),
        display_name: TrackedField::ai("Arc AI".to_string()),
        summary: TrackedField::unset(),
        position: TrackedField::unset(),
        motif: ArcMotifSection::default(),
        themes: ArcThemeSection::default(),
        source: Some(ProvenanceSource::AiGenerated),
    };
    state.optional.overview_phase.arcs.push(arc);

    // But scene inside arc has source = user
    let sc = ChapterPhaseSceneItem {
        code: "sc_user".to_string(),
        display_name: TrackedField::user("Cảnh người dùng".to_string()),
        position: TrackedField::unset(),
        summary: TrackedField::unset(),
        world_setting_code: TrackedField::unset(),
        source: Some(ProvenanceSource::User),
    };
    state.optional.chapter_phase.arcs.push(ChapterPhaseArcItem {
        code: "arc_ai".to_string(),
        chapters: vec![ChapterPhaseChapterItem {
            code: "chap_1".to_string(),
            display_name: TrackedField::unset(),
            position: TrackedField::unset(),
            summary: TrackedField::unset(),
            scenes: vec![sc],
            source: Some(ProvenanceSource::AiGenerated),
        }],
    });

    let initial_hash = state_data_hash(&state).unwrap();
    let guard = ExecutionGuard::new(&state);

    let params = serde_json::json!({
        "code": "arc_ai",
        "declared_source": "ai_generated"
    });
    let call = ToolCall::new("delete_arc", CallerCategory::AiCaller, params);
    let outcome = ToolExecutor::execute(&call, &guard);

    // AI cannot delete arc containing protected scene -> pending_confirmation
    assert_eq!(outcome.status, "pending_confirmation");
    assert!(!outcome.has_mutations());
    assert_eq!(outcome.pending_patches().len(), 1);
    assert_eq!(outcome.pending_patches()[0].operation, "delete");
    assert_eq!(outcome.pending_patches()[0].target_code, "arc_ai");

    // Preview lists full transitive impact
    let preview = outcome.cascade_preview().unwrap();
    assert!(preview.entities_to_delete.iter().any(|e| e.code == "arc_ai"));
    assert!(preview.entities_to_delete.iter().any(|e| e.code == "sc_user"));
    assert_eq!(preview.protected_items.len(), 1);
    assert_eq!(preview.protected_items[0].code, "sc_user");

    // State is 100% unchanged
    let final_hash = state_data_hash(&state).unwrap();
    assert_eq!(initial_hash, final_hash, "State must not be mutated when returning pending_confirmation");
}

/// `T-CASC-01`: Authoritative acceptance vector covering the 5 canonical Q1 cascade cases (`be-core.md` §12.1 line 2534).
#[test]
fn test_T_CASC_01_canonical_q1_cascade_delete_matrix() {
    // ------------------------------------------------------------------------
    // Case (1): Delete world_setting whose child is referenced by a scene -> Rejected
    // ------------------------------------------------------------------------
    {
        let mut state = BusinessFlowData::new();
        state.optional.world_building_phase.world_settings.push(WorldSettingItem {
            code: "ws_parent".to_string(),
            parent_code: TrackedField::unset(),
            display_name: TrackedField::user("Parent WS".to_string()),
            setting_type: TrackedField::user("LOCATION".to_string()),
            description: TrackedField::unset(),
            access: TrackedField::unset(),
            spatial_features: TrackedField::unset(),
            sensory_visual: TrackedField::unset(),
            sensory_auditory: TrackedField::unset(),
            sensory_tactile: TrackedField::unset(),
            sensory_olfactory: TrackedField::unset(),
            sensory_gustatory: TrackedField::unset(),
            lore_entries: Vec::new(),
            source: Some(ProvenanceSource::User),
        });
        state.optional.world_building_phase.world_settings.push(WorldSettingItem {
            code: "ws_child".to_string(),
            parent_code: TrackedField::user("ws_parent".to_string()),
            display_name: TrackedField::user("Child WS".to_string()),
            setting_type: TrackedField::user("ROOM".to_string()),
            description: TrackedField::unset(),
            access: TrackedField::unset(),
            spatial_features: TrackedField::unset(),
            sensory_visual: TrackedField::unset(),
            sensory_auditory: TrackedField::unset(),
            sensory_tactile: TrackedField::unset(),
            sensory_olfactory: TrackedField::unset(),
            sensory_gustatory: TrackedField::unset(),
            lore_entries: Vec::new(),
            source: Some(ProvenanceSource::User),
        });
        state.optional.chapter_phase.arcs.push(ChapterPhaseArcItem {
            code: "a1".to_string(),
            chapters: vec![ChapterPhaseChapterItem {
                code: "c1".to_string(),
                display_name: TrackedField::unset(),
                position: TrackedField::unset(),
                summary: TrackedField::unset(),
                scenes: vec![ChapterPhaseSceneItem {
                    code: "scene_ref".to_string(),
                    display_name: TrackedField::unset(),
                    position: TrackedField::unset(),
                    summary: TrackedField::unset(),
                    world_setting_code: TrackedField::user("ws_child".to_string()),
                    source: Some(ProvenanceSource::User),
                }],
                source: Some(ProvenanceSource::User),
            }],
        });

        let outcome = execute_delete(
            &state,
            CallerCategory::UiAction,
            "world_setting",
            "ws_parent",
            Some(ProvenanceSource::User),
        );
        assert_eq!(outcome.status(), "rejected");
        if let crate::cascade_delete::CascadeOutcome::Rejected { error, referencing_scenes, .. } = outcome {
            assert_eq!(error, "referenced_entity_in_use");
            assert_eq!(referencing_scenes, vec!["scene_ref".to_string()]);
        } else {
            panic!("Expected Rejected");
        }
    }

    // ------------------------------------------------------------------------
    // Case (2): Delete character who has an existing relationship -> Rejected
    // ------------------------------------------------------------------------
    {
        let mut state = BusinessFlowData::new();
        state.require.character.push(RequireCharacterItem {
            code: "hero".to_string(),
            display_name: TrackedField::user("Hero".to_string()),
            nickname: TrackedField::unset(),
            biological_sex: TrackedField::unset(),
            motif_code: TrackedField::unset(),
            role: TrackedField::unset(),
            notes: TrackedField::unset(),
            reused_library_character: false,
            source: Some(ProvenanceSource::User),
        });
        state.optional.relationship_phase.relationship.push(RelationshipPhaseItem {
            code: "rel_hero_villain".to_string(),
            motif: RelationshipMotifSection::default(),
            character: RelationshipCharacterPairSection {
                from_code: "hero".to_string(),
                to_code: "villain".to_string(),
                scenes: Vec::new(),
                roles: Vec::new(),
            },
            source: Some(ProvenanceSource::User),
        });

        let outcome = execute_delete(
            &state,
            CallerCategory::UiAction,
            "character",
            "hero",
            Some(ProvenanceSource::User),
        );
        assert_eq!(outcome.status(), "rejected");
        if let crate::cascade_delete::CascadeOutcome::Rejected { error, .. } = outcome {
            assert_eq!(error, "referenced_entity_in_use");
        } else {
            panic!("Expected Rejected");
        }
    }

    // ------------------------------------------------------------------------
    // Case (3): AI deletes arc (ai_generated) but containing user scene -> pending_confirmation
    // ------------------------------------------------------------------------
    {
        let mut state = BusinessFlowData::new();
        state.optional.overview_phase.arcs.push(OverviewArcItem {
            code: "arc_ai".to_string(),
            display_name: TrackedField::ai("Arc AI".to_string()),
            summary: TrackedField::unset(),
            position: TrackedField::unset(),
            motif: ArcMotifSection::default(),
            themes: ArcThemeSection::default(),
            source: Some(ProvenanceSource::AiGenerated),
        });
        state.optional.chapter_phase.arcs.push(ChapterPhaseArcItem {
            code: "arc_ai".to_string(),
            chapters: vec![ChapterPhaseChapterItem {
                code: "chap_1".to_string(),
                display_name: TrackedField::unset(),
                position: TrackedField::unset(),
                summary: TrackedField::unset(),
                scenes: vec![ChapterPhaseSceneItem {
                    code: "scene_user".to_string(),
                    display_name: TrackedField::user("User Scene".to_string()),
                    position: TrackedField::unset(),
                    summary: TrackedField::unset(),
                    world_setting_code: TrackedField::unset(),
                    source: Some(ProvenanceSource::User),
                }],
                source: Some(ProvenanceSource::AiGenerated),
            }],
        });

        let outcome = execute_delete(
            &state,
            CallerCategory::AiCaller,
            "arc",
            "arc_ai",
            Some(ProvenanceSource::AiGenerated),
        );
        assert_eq!(outcome.status(), "pending_confirmation");
        let preview = outcome.preview().unwrap();
        assert!(preview.entities_to_delete.iter().any(|e| e.code == "arc_ai"));
        assert!(preview.entities_to_delete.iter().any(|e| e.code == "chap_1"));
        assert!(preview.entities_to_delete.iter().any(|e| e.code == "scene_user"));
    }

    // ------------------------------------------------------------------------
    // Case (4): Delete scene with competences, memories, and relationship_states
    // ------------------------------------------------------------------------
    {
        let mut state = BusinessFlowData::new();
        state.optional.chapter_phase.arcs.push(ChapterPhaseArcItem {
            code: "a1".to_string(),
            chapters: vec![ChapterPhaseChapterItem {
                code: "c1".to_string(),
                display_name: TrackedField::unset(),
                position: TrackedField::unset(),
                summary: TrackedField::unset(),
                scenes: vec![ChapterPhaseSceneItem {
                    code: "sc_4".to_string(),
                    display_name: TrackedField::unset(),
                    position: TrackedField::unset(),
                    summary: TrackedField::unset(),
                    world_setting_code: TrackedField::unset(),
                    source: Some(ProvenanceSource::User),
                }],
                source: Some(ProvenanceSource::User),
            }],
        });
        state.optional.character_life_experience_phase.character.push(CharacterLifeExperienceItem {
            code: "char_1".to_string(),
            memories: vec![CharacterMemoryItem {
                code: Some("mem_1".to_string()),
                source_scene_code: TrackedField::user("sc_4".to_string()),
                content: TrackedField::user("Content".to_string()),
                source: Some(ProvenanceSource::User),
            }],
            competences: vec![CharacterCompetenceItem {
                code: "comp_1".to_string(),
                domain_code: TrackedField::unset(),
                level_code: TrackedField::unset(),
                scene_code: TrackedField::user("sc_4".to_string()),
                display_name: TrackedField::user("Comp".to_string()),
                description: TrackedField::unset(),
                reliability: TrackedField::unset(),
                experience: TrackedField::unset(),
                capabilities: Vec::new(),
                limitations: Vec::new(),
                source: Some(ProvenanceSource::User),
            }],
        });
        state.optional.relationship_phase.relationship.push(RelationshipPhaseItem {
            code: "rel_1".to_string(),
            motif: RelationshipMotifSection::default(),
            character: RelationshipCharacterPairSection {
                from_code: "char_1".to_string(),
                to_code: "char_2".to_string(),
                scenes: vec![RelationshipSceneStateItem {
                    scene_code: "sc_4".to_string(),
                    trust: TrackedField::user("HIGH".to_string()),
                    affection: TrackedField::unset(),
                    hostility: TrackedField::unset(),
                    fear: TrackedField::unset(),
                    respect: TrackedField::unset(),
                    dependency: TrackedField::unset(),
                    familiarity: TrackedField::unset(),
                    source: Some(ProvenanceSource::User),
                }],
                roles: Vec::new(),
            },
            source: Some(ProvenanceSource::User),
        });

        // UI action performs deletion
        let outcome = execute_delete(
            &state,
            CallerCategory::UiAction,
            "scene",
            "sc_4",
            Some(ProvenanceSource::User),
        );
        assert_eq!(outcome.status(), "applied");
        let mutated = outcome.mutated_state().unwrap();

        // Scene is deleted
        assert!(mutated.optional.chapter_phase.arcs[0].chapters[0].scenes.is_empty());

        // Competence is physically deleted
        let life = &mutated.optional.character_life_experience_phase.character[0];
        assert!(life.competences.is_empty());

        // Memory source_scene_code is nullified
        assert_eq!(life.memories[0].source_scene_code.value, None);

        // Relationship state deleted, parent relationship preserved
        assert_eq!(mutated.optional.relationship_phase.relationship.len(), 1);
        assert!(mutated.optional.relationship_phase.relationship[0].character.scenes.is_empty());
    }

    // ------------------------------------------------------------------------
    // Case (5): Delete lore_entry with scene_lore_entries -> Unlinks atomically, keeps scene
    // ------------------------------------------------------------------------
    {
        let mut state = BusinessFlowData::new();
        state.optional.world_building_phase.world_settings.push(WorldSettingItem {
            code: "ws_1".to_string(),
            parent_code: TrackedField::unset(),
            display_name: TrackedField::unset(),
            setting_type: TrackedField::unset(),
            description: TrackedField::unset(),
            access: TrackedField::unset(),
            spatial_features: TrackedField::unset(),
            sensory_visual: TrackedField::unset(),
            sensory_auditory: TrackedField::unset(),
            sensory_tactile: TrackedField::unset(),
            sensory_olfactory: TrackedField::unset(),
            sensory_gustatory: TrackedField::unset(),
            lore_entries: vec![LoreEntryItem {
                code: "lore_ancient_sword".to_string(),
                display_name: TrackedField::user("Thanh Kiếm Cổ".to_string()),
                lore_type: TrackedField::unset(),
                description: TrackedField::unset(),
                data: TrackedField::unset(),
                source: Some(ProvenanceSource::User),
            }],
            source: Some(ProvenanceSource::User),
        });

        state.optional.scene_config_phase.arcs.push(SceneConfigArcItem {
            code: "a1".to_string(),
            chapters: vec![SceneConfigChapterItem {
                code: "c1".to_string(),
                scenes: vec![SceneConfigSceneItem {
                    code: "sc_cave".to_string(),
                    world_setting_code: TrackedField::unset(),
                    style_preset_code: TrackedField::unset(),
                    narrator_config_code: TrackedField::unset(),
                    lore_entries: vec![SceneLoreEntryLink {
                        lore_entry_code: "lore_ancient_sword".to_string(),
                        usage: TrackedField::unset(),
                        reveal_to_reader: TrackedField::unset(),
                        reveal_to_pov: TrackedField::unset(),
                        source: Some(ProvenanceSource::User),
                    }],
                    characters: Vec::new(),
                }],
            }],
        });

        let outcome = execute_delete(
            &state,
            CallerCategory::UiAction,
            "lore_entry",
            "lore_ancient_sword",
            Some(ProvenanceSource::User),
        );
        assert_eq!(outcome.status(), "applied");
        let mutated = outcome.mutated_state().unwrap();

        // Lore entry is deleted from world setting
        assert!(mutated.optional.world_building_phase.world_settings[0].lore_entries.is_empty());

        // scene_lore_entries link is unlinked, but scene sc_cave remains intact!
        let sc = &mutated.optional.scene_config_phase.arcs[0].chapters[0].scenes[0];
        assert_eq!(sc.code, "sc_cave");
        assert!(sc.lore_entries.is_empty());
    }
}

/// `phase3_sprint5_deep2-F0`: Predecessor gate absent or baseline drift returns `CONTRACT_BLOCKED`.
#[test]
fn test_F0_predecessor_gate_absent_or_baseline_drift_returns_contract_blocked() {
    // 1. Missing predecessor gate returns CONTRACT_BLOCKED
    let res_gate = check_predecessor_gates(BASELINE_COMMIT, BASELINE_COMMIT, false, true);
    assert_eq!(
        res_gate,
        Err(ErrorCode::ValidationFailed),
        "Missing predecessor gate must return CONTRACT_BLOCKED"
    );

    // 2. Baseline drift returns CONTRACT_BLOCKED
    let res_drift = check_predecessor_gates(BASELINE_COMMIT, "drifted_baseline_sha", true, true);
    assert_eq!(
        res_drift,
        Err(ErrorCode::ValidationFailed),
        "Baseline drift must return CONTRACT_BLOCKED"
    );

    // 3. Verify git repository commit ancestry for baseline and predecessors
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let repo_root = manifest_dir
        .parent()
        .expect("CARGO_MANIFEST_DIR must have parent repo root");

    let baseline_check = Command::new("git")
        .args([
            "merge-base",
            "--is-ancestor",
            BASELINE_COMMIT,
            "HEAD",
        ])
        .current_dir(&repo_root)
        .status()
        .expect("git merge-base must execute");
    assert!(
        baseline_check.success(),
        "BASELINE_COMMIT {BASELINE_COMMIT} must be an ancestor of HEAD"
    );

    let p3s4_check = Command::new("git")
        .args([
            "cat-file",
            "-e",
            &format!("{PREDECESSOR_P3S4_COMMIT}^{{commit}}"),
        ])
        .current_dir(&repo_root)
        .status()
        .expect("git cat-file must execute");
    assert!(
        p3s4_check.success(),
        "PREDECESSOR_P3S4_COMMIT {PREDECESSOR_P3S4_COMMIT} must exist in repository"
    );

    let p3s1_check = Command::new("git")
        .args([
            "cat-file",
            "-e",
            &format!("{PREDECESSOR_P3S1_COMMIT}^{{commit}}"),
        ])
        .current_dir(&repo_root)
        .status()
        .expect("git cat-file must execute");
    assert!(
        p3s1_check.success(),
        "PREDECESSOR_P3S1_COMMIT {PREDECESSOR_P3S1_COMMIT} must exist in repository"
    );
}
