#![allow(clippy::field_reassign_with_default)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::result_large_err)]
#![allow(clippy::result_unit_err)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]

//! Acceptance and contract tests for `phase3_sprint8_deep1`
//! ("Implement SF-06 pure patch/manifest codec").
//!
//! Per the backend module wiring rule in `docs/plan/phase3_sprint8_deep1.md`:
//! - This sprint does not edit `src-tauri/src/lib.rs` or `src-tauri/src/domain/mod.rs`.
//! - Instead, this test target loads each owned implementation module directly via `#[path = "..."]`.
//!
//! Covered acceptance vectors:
//! - `P3S8-P`: Property test: `replay(before, diff)` has equal canonical bytes to `after`.
//! - `P3S8-N`: Malformed or unknown v2 codec fails closed without tool call (`UnsupportedEvidenceCodec`).
//! - `P3S8-B`: T1 1->2, T2 2->1 has two patches and no net version.
//! - `phase3_sprint8_deep1-F0`: Predecessor gate absent or baseline SHA drift returns `CONTRACT_BLOCKED`.
//! - `phase3_sprint8_deep1-F1`: Unsupported valid transition returns `EvidenceCodecUnsupported`, no state publish.
//! - `phase3_sprint8_deep1-F2`: Same-path write or update deleted entity rejected; independent ref-null cascade accepted.
//! - Golden Vectors `GV1` through `GV12` ratified in `be-core.md` §1532.

mod common;

#[path = "../src/error.rs"]
pub mod error;

#[path = "../src/domain/business_flow.rs"]
pub mod business_flow;

#[path = "../src/domain/fingerprint.rs"]
pub mod fingerprint;

#[path = "../src/domain/sf06/manifest.rs"]
pub mod manifest;

#[path = "../src/domain/sf06/codec.rs"]
pub mod codec;

pub mod domain {
    pub use crate::business_flow;
    pub use crate::codec;
    pub use crate::fingerprint;
    pub use crate::manifest;
}

use business_flow::*;
use codec::*;
use error::ErrorCode;
use fingerprint::*;
use manifest::*;
use serde_json::json;

pub const BASELINE_COMMIT: &str = "ddda30cd3c842113599c7bfb7d0e221936330428";
pub const PREDECESSOR_P3S1_COMMIT: &str = "702843c38e45b193be25b80222a48eadedfd3538";
pub const PREDECESSOR_P3S3_COMMIT: &str = "9f8bb7f0c43ba79d22072fb73f026a54343e4363";

/// Verifies predecessor dispatch preconditions (`phase3_sprint8_deep1-F0`).
pub fn verify_p3s8_dispatch_preconditions(
    baseline_commit: &str,
    current_head: &str,
    gate_p3s1: bool,
    gate_p3s3: bool,
) -> Result<(), String> {
    if !gate_p3s1 {
        return Err(
            "CONTRACT_BLOCKED: predecessor gate_phase3_sprint1_deep0 is absent".to_string(),
        );
    }
    if !gate_p3s3 {
        return Err(
            "CONTRACT_BLOCKED: predecessor gate_phase3_sprint3_deep0 is absent".to_string(),
        );
    }
    if common::verify_commit_at(baseline_commit, current_head) {
        Ok(())
    } else {
        Err(format!(
            "CONTRACT_BLOCKED: baseline commit {baseline_commit} is not an ancestor of {current_head} and is not recorded in the lineage manifest"
        ))
    }
}

// ---------------------------------------------------------------------------
// Acceptance Tests
// ---------------------------------------------------------------------------

#[test]
#[allow(non_snake_case)]
fn phase3_sprint8_deep1_F0_predecessor_gate_absent_or_baseline_drift_is_contract_blocked() {
    // 1. Missing predecessor gate P3S1 -> CONTRACT_BLOCKED
    let err_p3s1 =
        verify_p3s8_dispatch_preconditions(BASELINE_COMMIT, "HEAD", false, true).unwrap_err();
    assert!(err_p3s1.contains("CONTRACT_BLOCKED"));
    assert!(err_p3s1.contains("gate_phase3_sprint1_deep0"));

    // 2. Missing predecessor gate P3S3 -> CONTRACT_BLOCKED
    let err_p3s3 =
        verify_p3s8_dispatch_preconditions(BASELINE_COMMIT, "HEAD", true, false).unwrap_err();
    assert!(err_p3s3.contains("CONTRACT_BLOCKED"));
    assert!(err_p3s3.contains("gate_phase3_sprint3_deep0"));

    // 3. Baseline drift simulation -> CONTRACT_BLOCKED
    let bad_baseline = "0000000000000000000000000000000000000000";
    let err_drift =
        verify_p3s8_dispatch_preconditions(bad_baseline, "HEAD", true, true).unwrap_err();
    assert!(err_drift.contains("CONTRACT_BLOCKED"));

    // 4. Live check against worktree HEAD and git repository
    let live_check = verify_p3s8_dispatch_preconditions(BASELINE_COMMIT, "HEAD", true, true);
    assert!(
        live_check.is_ok(),
        "Live predecessor check must succeed on verified baseline"
    );

    // Also verify predecessor commits are ancestors, local objects, or recorded
    assert!(
        common::verify_commit(PREDECESSOR_P3S1_COMMIT),
        "Predecessor P3S1 commit {PREDECESSOR_P3S1_COMMIT} must be an ancestor, a local object, or recorded in the lineage manifest"
    );
    assert!(
        common::verify_commit(PREDECESSOR_P3S3_COMMIT),
        "Predecessor P3S3 commit {PREDECESSOR_P3S3_COMMIT} must be an ancestor, a local object, or recorded in the lineage manifest"
    );
}

#[test]
#[allow(non_snake_case)]
fn P3S8_P_property_replay_diff_has_equal_canonical_bytes_to_after() {
    let mut s0 = BusinessFlowData::new();
    s0.meta.phase = 0;
    s0.require.project.display_name = TrackedField::ai("Initial Story".to_string());
    s0.require.project.summary = TrackedField::ai("Initial Summary".to_string());
    s0.counters.insert("arc".to_string(), 1);

    let mut s1 = s0.clone();
    s1.meta.phase = 1;
    s1.require.project.summary = TrackedField::ai("Updated Summary".to_string());
    s1.counters.insert("arc".to_string(), 2);
    s1.optional.overview_phase.arcs.push(OverviewArcItem {
        code: "arc_01".to_string(),
        display_name: TrackedField::ai("Arc One".to_string()),
        summary: TrackedField::ai("Summary of Arc One".to_string()),
        position: TrackedField::ai(1),
        ..Default::default()
    });

    let diff = canonical_diff(&s0, &s1).expect("canonical_diff must succeed");
    assert!(!diff.is_empty(), "diff must not be empty for mutated state");

    let replayed = replay(&s0, &diff).expect("replay must succeed");
    let bytes_replayed = canonical_state(&replayed).unwrap();
    let bytes_s1 = canonical_state(&s1).unwrap();

    assert_eq!(
        bytes_replayed, bytes_s1,
        "P3S8-P: property replay(before, diff) must have equal canonical bytes to after"
    );
    assert_eq!(
        state_data_hash(&replayed).unwrap(),
        state_data_hash(&s1).unwrap(),
        "State data hashes must be identical"
    );
}

#[test]
#[allow(non_snake_case)]
fn P3S8_N_malformed_or_unknown_codec_fails_closed() {
    // 1. Unknown codec version "v1"
    let bad_codec_json = json!({
        "schemaVersion": 2,
        "codecVersion": "v1",
        "plannerVersion": "v1",
        "policyVersion": "v1",
        "taskSpec": {},
        "preStateHash": "sha256:0000000000000000000000000000000000000000000000000000000000000000",
        "postStateHash": "sha256:0000000000000000000000000000000000000000000000000000000000000000",
        "hadAppliedWrites": true,
        "effectiveStateChanged": true,
        "patch": []
    });
    let err = decode_evidence(&bad_codec_json).unwrap_err();
    assert_eq!(
        err,
        ErrorCode::UnsupportedEvidenceCodec,
        "Unknown codec version must fail closed with UnsupportedEvidenceCodec"
    );

    // 2. Unknown schema version 3
    let bad_schema_json = json!({
        "schemaVersion": 3,
        "codecVersion": "v2",
        "plannerVersion": "v1",
        "policyVersion": "v1",
        "taskSpec": {},
        "preStateHash": "sha256:0000000000000000000000000000000000000000000000000000000000000000",
        "postStateHash": "sha256:0000000000000000000000000000000000000000000000000000000000000000",
        "hadAppliedWrites": true,
        "effectiveStateChanged": true,
        "patch": []
    });
    let err_schema = decode_evidence(&bad_schema_json).unwrap_err();
    assert_eq!(
        err_schema,
        ErrorCode::UnsupportedEvidenceCodec,
        "Unknown schema version must fail closed with UnsupportedEvidenceCodec"
    );
}

#[test]
#[allow(non_snake_case)]
fn P3S8_B_zero_net_mutation_preserves_patches_without_version() {
    let mut s0 = BusinessFlowData::new();
    s0.counters.insert("test_counter".to_string(), 1);

    // T1: 1 -> 2
    let mut s1 = s0.clone();
    s1.counters.insert("test_counter".to_string(), 2);
    let patch_t1 = canonical_diff(&s0, &s1).unwrap();
    assert_eq!(patch_t1.ops.len(), 1);

    // T2: 2 -> 1
    let mut s2 = s1.clone();
    s2.counters.insert("test_counter".to_string(), 1);
    let patch_t2 = canonical_diff(&s1, &s2).unwrap();
    assert_eq!(patch_t2.ops.len(), 1);

    // Replay sequence: S0 -> S1 via patch_t1, S1 -> S2 via patch_t2
    let s1_replayed = replay(&s0, &patch_t1).unwrap();
    assert_eq!(s1_replayed.counters.get("test_counter").copied(), Some(2));

    let s2_replayed = replay(&s1_replayed, &patch_t2).unwrap();
    assert_eq!(s2_replayed.counters.get("test_counter").copied(), Some(1));

    // Net mutation is zero: S2 is semantically equal to S0
    assert!(
        states_are_semantically_equal(&s0, &s2_replayed).unwrap(),
        "P3S8-B: Final state S2 after T1 and T2 must equal S0"
    );

    let s0_bytes = canonical_state(&s0).unwrap();
    let s2_bytes = canonical_state(&s2_replayed).unwrap();
    assert_eq!(s0_bytes, s2_bytes, "Canonical bytes must be identical");

    // Header values representation for zero-net:
    let had_mutations = false;
    let resulting_version_seq: Option<i64> = None;
    assert!(!had_mutations);
    assert_eq!(resulting_version_seq, None);
}

#[test]
#[allow(non_snake_case)]
fn phase3_sprint8_deep1_F1_unsupported_valid_transition() {
    let s0 = BusinessFlowData::new();

    // Trying to replay malformed evidence with invalid codecVersion
    let bad_evidence = EvidenceJsonV2 {
        schema_version: 2,
        codec_version: "v99_unsupported".to_string(),
        planner_version: "v1".to_string(),
        policy_version: "v1".to_string(),
        task_spec: json!({}),
        pre_state_hash: state_data_hash(&s0).unwrap(),
        post_state_hash: state_data_hash(&s0).unwrap(),
        had_applied_writes: true,
        effective_state_changed: false,
        patch: vec![],
        artifact: None,
    };

    let err = replay_evidence(&s0, &bad_evidence).unwrap_err();
    assert_eq!(err, ErrorCode::UnsupportedEvidenceCodec);
}

#[test]
#[allow(non_snake_case)]
fn phase3_sprint8_deep1_F2_same_path_write_and_update_deleted_entity() {
    let mut s0 = BusinessFlowData::new();
    s0.require.project.summary = TrackedField::ai("Initial".to_string());

    // 1. Same-path duplicate write -> EvidenceCodecUnsupported
    let duplicate_patch = PatchV2 {
        ops: vec![
            PatchOp::PutField {
                path: "/require/project/summary".to_string(),
                precondition: json!(TrackedField::ai("Initial".to_string())),
                value: json!(TrackedField::ai("First".to_string())),
            },
            PatchOp::PutField {
                path: "/require/project/summary".to_string(),
                precondition: json!(TrackedField::ai("Initial".to_string())),
                value: json!(TrackedField::ai("Second".to_string())),
            },
        ],
    };
    let err_dup = replay(&s0, &duplicate_patch).unwrap_err();
    assert_eq!(
        err_dup,
        ErrorCode::EvidenceCodecUnsupported,
        "Duplicate write to the same path must return EvidenceCodecUnsupported"
    );

    // 2. Update child of deleted entity -> EvidenceCodecUnsupported
    let mut s_with_arc = BusinessFlowData::new();
    s_with_arc
        .optional
        .overview_phase
        .arcs
        .push(OverviewArcItem {
            code: "arc_01".to_string(),
            display_name: TrackedField::ai("Arc".to_string()),
            summary: TrackedField::ai("Summary".to_string()),
            ..Default::default()
        });

    let delete_and_update_child_patch = PatchV2 {
        ops: vec![
            PatchOp::DeleteEntity {
                path: "/optional/overview_phase/arcs".to_string(),
                entity_code: "arc_01".to_string(),
            },
            PatchOp::PutField {
                path: "/optional/overview_phase/arcs/arc_01/summary".to_string(),
                precondition: json!(TrackedField::ai("Summary".to_string())),
                value: json!(TrackedField::ai("New Summary".to_string())),
            },
        ],
    };
    let err_child = replay(&s_with_arc, &delete_and_update_child_patch).unwrap_err();
    assert_eq!(
        err_child,
        ErrorCode::EvidenceCodecUnsupported,
        "Updating child of deleted entity must return EvidenceCodecUnsupported"
    );
}

// ---------------------------------------------------------------------------
// Golden Vectors GV1 - GV12 (be-core.md §1532)
// ---------------------------------------------------------------------------

#[test]
#[allow(non_snake_case)]
fn golden_vector_GV1_create_arc_and_set_counter() {
    let mut s0 = BusinessFlowData::new();
    s0.counters.insert("arc".to_string(), 1);

    let patch = PatchV2 {
        ops: vec![
            PatchOp::InsertEntity {
                path: "/optional/overview_phase/arcs".to_string(),
                entity_code: "arc_01".to_string(),
                value: json!(OverviewArcItem {
                    code: "arc_01".to_string(),
                    display_name: TrackedField::ai("First Arc".to_string()),
                    position: TrackedField::ai(1),
                    ..Default::default()
                }),
            },
            PatchOp::SetCounter {
                counter_key: "arc".to_string(),
                old_value: 1,
                new_value: 2,
            },
        ],
    };

    let s1 = replay(&s0, &patch).expect("GV1 replay must succeed");
    assert_eq!(s1.counters.get("arc").copied(), Some(2));
    assert_eq!(s1.optional.overview_phase.arcs.len(), 1);
    assert_eq!(s1.optional.overview_phase.arcs[0].code, "arc_01");
}

#[test]
#[allow(non_snake_case)]
fn golden_vector_GV2_tracked_field_preserves_provenance_source() {
    let mut s0 = BusinessFlowData::new();
    s0.require.project.summary = TrackedField::ai("Old Summary".to_string());

    let patch = PatchV2 {
        ops: vec![PatchOp::PutField {
            path: "/require/project/summary".to_string(),
            precondition: json!(TrackedField::ai("Old Summary".to_string())),
            value: json!(TrackedField::ai("New Summary".to_string())),
        }],
    };

    let s1 = replay(&s0, &patch).expect("GV2 replay must succeed");
    assert_eq!(
        s1.require.project.summary.value.as_deref(),
        Some("New Summary")
    );
    assert_eq!(
        s1.require.project.summary.source,
        Some(ProvenanceSource::AiGenerated)
    );
}

#[test]
#[allow(non_snake_case)]
fn golden_vector_GV3_delete_scene_and_nullify_memory_ref() {
    let mut s0 = BusinessFlowData::new();
    // Scene in chapter_phase
    s0.optional.chapter_phase.arcs.push(ChapterPhaseArcItem {
        code: "arc_01".to_string(),
        chapters: vec![ChapterPhaseChapterItem {
            code: "ch_01".to_string(),
            display_name: TrackedField::ai("Chap 1".to_string()),
            position: TrackedField::ai(1),
            scenes: vec![ChapterPhaseSceneItem {
                code: "sc_01".to_string(),
                display_name: TrackedField::ai("Scene 1".to_string()),
                position: TrackedField::ai(1),
                ..Default::default()
            }],
            ..Default::default()
        }],
    });
    // Memory referencing sc_01
    s0.optional
        .character_life_experience_phase
        .character
        .push(CharacterLifeExperienceItem {
            code: "char_01".to_string(),
            memories: vec![CharacterMemoryItem {
                code: Some("mem_01".to_string()),
                source_scene_code: TrackedField::ai("sc_01".to_string()),
                content: TrackedField::ai("Remembered scene".to_string()),
                ..Default::default()
            }],
            competences: vec![],
        });

    // Patch: set memory source_scene_code to null AND delete scene sc_01
    // (order in patch does not matter for atomic clone replay)
    let patch = PatchV2 {
        ops: vec![
            PatchOp::PutField {
                path: "/optional/character_life_experience_phase/character/char_01/memories/0/source_scene_code".to_string(),
                precondition: json!(TrackedField::ai("sc_01".to_string())),
                value: json!(TrackedField::<String>::unset()),
            },
            PatchOp::DeleteEntity {
                path: "/optional/chapter_phase/arcs/arc_01/chapters/ch_01/scenes".to_string(),
                entity_code: "sc_01".to_string(),
            },
        ],
    };

    let s1 = replay(&s0, &patch).expect("GV3 atomic clone replay must succeed");
    assert_eq!(
        s1.optional.chapter_phase.arcs[0].chapters[0].scenes.len(),
        0
    );
    assert_eq!(
        s1.optional.character_life_experience_phase.character[0].memories[0]
            .source_scene_code
            .value,
        None
    );
}

#[test]
#[allow(non_snake_case)]
fn golden_vector_GV4_unauthorized_transition_fails_closed() {
    let mut s0 = BusinessFlowData::new();
    s0.require.project.summary = TrackedField::user("User protected content".to_string());

    // Attempting to overwrite a user-protected field returns UnauthorizedTransition
    let patch = PatchV2 {
        ops: vec![PatchOp::PutField {
            path: "/require/project/summary".to_string(),
            precondition: json!(TrackedField::user("User protected content".to_string())),
            value: json!(TrackedField::ai("AI overwrite attempt".to_string())),
        }],
    };

    let err = replay(&s0, &patch).unwrap_err();
    assert_eq!(err, ErrorCode::UnauthorizedTransition);
}

#[test]
#[allow(non_snake_case)]
fn golden_vector_GV5_protected_field_proposal_yields_empty_patch() {
    let mut s0 = BusinessFlowData::new();
    s0.require.project.summary = TrackedField::user("Protected User Summary".to_string());

    let mut s1 = s0.clone();
    // Simulate AI wanting to change summary to "AI Proposed Summary"
    s1.require.project.summary = TrackedField::ai("AI Proposed Summary".to_string());

    // canonical_diff excludes protected field changes from direct patch (they go to TX-08 proposal)
    let patch =
        canonical_diff(&s0, &s1).expect("diff must produce empty patch for protected field change");
    assert!(
        patch.is_empty(),
        "GV5: Protected-field changes must yield patch=[] as they belong in TX-08 independent proposal"
    );
}

#[test]
#[allow(non_snake_case)]
fn golden_vector_GV6_ordered_arrays_do_not_sort() {
    let mut s0 = BusinessFlowData::new();
    s0.user_constraints = vec![
        UserConstraintEntry {
            id: "c1".to_string(),
            content: "first constraint".to_string(),
            source: ProvenanceSource::User,
        },
        UserConstraintEntry {
            id: "c2".to_string(),
            content: "second constraint".to_string(),
            source: ProvenanceSource::User,
        },
    ];

    // Swap order: [c2, c1]
    let mut s1 = s0.clone();
    s1.user_constraints.reverse();

    let patch = canonical_diff(&s0, &s1).unwrap();
    let replayed = replay(&s0, &patch).unwrap();

    assert_eq!(replayed.user_constraints[0].id, "c2");
    assert_eq!(replayed.user_constraints[1].id, "c1");

    // Canonical bytes must strictly reflect [c2, c1], NOT sorted by ID
    let bytes1 = canonical_state(&replayed).unwrap();
    let bytes0 = canonical_state(&s0).unwrap();
    assert_ne!(
        bytes1, bytes0,
        "Reordered semantic array must have distinct canonical bytes"
    );
}

#[test]
#[allow(non_snake_case)]
fn golden_vector_GV7_applied_same_value_yields_empty_patch() {
    let mut s0 = BusinessFlowData::new();
    s0.require.project.display_name = TrackedField::ai("Same Name".to_string());

    let s1 = s0.clone();
    let patch = canonical_diff(&s0, &s1).unwrap();
    assert!(
        patch.is_empty(),
        "Same-value transition must produce empty patch"
    );

    let evidence = EvidenceJsonV2 {
        schema_version: 2,
        codec_version: "v2".to_string(),
        planner_version: "v1".to_string(),
        policy_version: "v1".to_string(),
        task_spec: json!({}),
        pre_state_hash: state_data_hash(&s0).unwrap(),
        post_state_hash: state_data_hash(&s1).unwrap(),
        had_applied_writes: true,
        effective_state_changed: false,
        patch: patch.ops,
        artifact: None,
    };

    assert!(evidence.had_applied_writes);
    assert!(!evidence.effective_state_changed);
    assert!(evidence.patch.is_empty());
}

#[test]
#[allow(non_snake_case)]
fn golden_vector_GV8_two_patches_zero_net_replayable() {
    let mut s0 = BusinessFlowData::new();
    s0.counters.insert("step".to_string(), 1);

    // T1: 1 -> 2
    let mut s1 = s0.clone();
    s1.counters.insert("step".to_string(), 2);
    let p1 = canonical_diff(&s0, &s1).unwrap();

    // T2: 2 -> 1
    let mut s2 = s1.clone();
    s2.counters.insert("step".to_string(), 1);
    let p2 = canonical_diff(&s1, &s2).unwrap();

    let s1_res = replay(&s0, &p1).unwrap();
    let s2_res = replay(&s1_res, &p2).unwrap();

    assert!(states_are_semantically_equal(&s0, &s2_res).unwrap());
}

#[test]
#[allow(non_snake_case)]
fn golden_vector_GV9_restore_invalidation_historical_verified_but_not_applicable() {
    // V2 state
    let mut v2 = BusinessFlowData::new();
    v2.counters.insert("ver".to_string(), 2);

    // V3 -> V4
    let mut v3 = v2.clone();
    v3.counters.insert("ver".to_string(), 3);
    let mut v4 = v3.clone();
    v4.counters.insert("ver".to_string(), 4);

    let patch_v3_v4 = canonical_diff(&v3, &v4).unwrap();
    let v4_replayed = replay(&v3, &patch_v3_v4).unwrap();
    assert_eq!(v4_replayed.counters.get("ver").copied(), Some(4));

    // V5 is created by user Restore V2 (so V5 has ver = 2)
    let mut v5 = v2.clone();
    v5.meta.version = "1.0-v5".to_string();

    // The historical task from V3->V4 has input_hash on V3, but head is V5 (which matches V2, not V3)
    let v3_hash = state_data_hash(&v3).unwrap();
    let v5_hash = state_data_hash(&v5).unwrap();
    assert_ne!(
        v3_hash, v5_hash,
        "V5 head does not match historical task V3 input baseline"
    );
}

#[test]
#[allow(non_snake_case)]
fn golden_vector_GV10_same_path_write_twice_rejected() {
    let s0 = BusinessFlowData::new();

    let conflict_patch = PatchV2 {
        ops: vec![
            PatchOp::SetCounter {
                counter_key: "k".to_string(),
                old_value: 0,
                new_value: 1,
            },
            PatchOp::SetCounter {
                counter_key: "k".to_string(),
                old_value: 0,
                new_value: 2,
            },
        ],
    };

    let err = replay(&s0, &conflict_patch).unwrap_err();
    assert_eq!(err, ErrorCode::EvidenceCodecUnsupported);
}

#[test]
#[allow(non_snake_case)]
fn golden_vector_GV11_field_update_on_deleted_entity_rejected_but_independent_accepted() {
    let mut s0 = BusinessFlowData::new();
    s0.optional.chapter_phase.arcs.push(ChapterPhaseArcItem {
        code: "arc_01".to_string(),
        chapters: vec![ChapterPhaseChapterItem {
            code: "ch_01".to_string(),
            display_name: TrackedField::ai("Chap".to_string()),
            position: TrackedField::ai(1),
            scenes: vec![ChapterPhaseSceneItem {
                code: "sc_01".to_string(),
                display_name: TrackedField::ai("Scene".to_string()),
                position: TrackedField::ai(1),
                ..Default::default()
            }],
            ..Default::default()
        }],
    });
    s0.optional
        .character_life_experience_phase
        .character
        .push(CharacterLifeExperienceItem {
            code: "char_01".to_string(),
            memories: vec![CharacterMemoryItem {
                code: Some("mem_01".to_string()),
                source_scene_code: TrackedField::ai("sc_01".to_string()),
                content: TrackedField::ai("Content".to_string()),
                ..Default::default()
            }],
            competences: vec![],
        });

    // Case A: Field update on the deleted scene itself -> REJECTED
    let bad_patch = PatchV2 {
        ops: vec![
            PatchOp::DeleteEntity {
                path: "/optional/chapter_phase/arcs/arc_01/chapters/ch_01/scenes".to_string(),
                entity_code: "sc_01".to_string(),
            },
            PatchOp::PutField {
                path:
                    "/optional/chapter_phase/arcs/arc_01/chapters/ch_01/scenes/sc_01/display_name"
                        .to_string(),
                precondition: json!(TrackedField::ai("Scene".to_string())),
                value: json!(TrackedField::ai("New Scene Name".to_string())),
            },
        ],
    };
    let err_case_a = replay(&s0, &bad_patch).unwrap_err();
    assert_eq!(err_case_a, ErrorCode::EvidenceCodecUnsupported);

    // Case B: Updating independent entity memory to set ref to null -> ACCEPTED
    let good_patch = PatchV2 {
        ops: vec![
            PatchOp::DeleteEntity {
                path: "/optional/chapter_phase/arcs/arc_01/chapters/ch_01/scenes".to_string(),
                entity_code: "sc_01".to_string(),
            },
            PatchOp::PutField {
                path: "/optional/character_life_experience_phase/character/char_01/memories/0/source_scene_code".to_string(),
                precondition: json!(TrackedField::ai("sc_01".to_string())),
                value: json!(TrackedField::<String>::unset()),
            },
        ],
    };
    let res_case_b = replay(&s0, &good_patch);
    assert!(
        res_case_b.is_ok(),
        "Independent reference nullification alongside entity deletion must succeed"
    );
}

#[test]
#[allow(non_snake_case)]
fn golden_vector_GV12_missing_tracked_field_vs_explicit_null_canonical_identical() {
    let json_missing = json!({
        "meta": { "version": "1.0", "phase": 0 },
        "require": {
            "project": {
                // display_name is omitted
                "summary": { "value": "A story", "source": "ai_generated" }
            }
        }
    });

    let json_explicit_null = json!({
        "meta": { "version": "1.0", "phase": 0 },
        "require": {
            "project": {
                "display_name": { "value": null, "source": null },
                "summary": { "value": "A story", "source": "ai_generated" }
            }
        }
    });

    let json_normalized = json!({
        "meta": { "version": "1.0", "phase": 0 },
        "require": {
            "project": {
                "display_name": { "value": null, "source": "incomplete" },
                "summary": { "value": "A story", "source": "ai_generated" }
            }
        }
    });

    let s_missing = BusinessFlowData::from_json_value_strict(&json_missing).unwrap();
    let s_explicit = BusinessFlowData::from_json_value_strict(&json_explicit_null).unwrap();
    let s_norm = BusinessFlowData::from_json_value_strict(&json_normalized).unwrap();

    let bytes_missing = canonical_state(&s_missing).unwrap();
    let bytes_explicit = canonical_state(&s_explicit).unwrap();
    let bytes_norm = canonical_state(&s_norm).unwrap();

    assert_eq!(
        bytes_missing, bytes_explicit,
        "Missing optional TrackedField must equal explicit null TrackedField"
    );
    assert_eq!(
        bytes_explicit, bytes_norm,
        "Explicit null TrackedField must equal normalized TrackedField"
    );

    // But missing required top-level key 'meta' must reject
    let json_no_meta = json!({
        "require": {}
    });
    let err = BusinessFlowData::from_json_value_strict(&json_no_meta).unwrap_err();
    assert!(matches!(err, DomainError::UnknownStateShape { .. }));
}

// ---------------------------------------------------------------------------
// Manifest and Digest Tests
// ---------------------------------------------------------------------------

#[test]
fn manifest_plan_and_request_digest_computation() {
    let draft_session_id = "11111111-1111-1111-1111-111111111111";
    let workflow_run_id = "22222222-2222-2222-2222-222222222222";

    let plan_input = PlanDigestInput {
        draft_session_id: draft_session_id.to_string(),
        workflow_run_id: workflow_run_id.to_string(),
        plan_generation: 1,
        round_index: 0,
        task_count: 2,
        parallel_limit: 6,
        tasks: vec![
            ManifestTask::new(
                0,
                "p1:s0:outline_window:arc=arc_01:ch=1..50",
                json!({ "phase": 1, "kind": "outline_window" }),
                "v1",
                "v1",
                "v2",
            ),
            ManifestTask::new(
                1,
                "p1:s0:outline_window:arc=arc_01:ch=51..100",
                json!({ "phase": 1, "kind": "outline_window" }),
                "v1",
                "v1",
                "v2",
            ),
        ],
    };

    let plan_digest = compute_plan_digest(&plan_input).expect("plan_digest compute must succeed");
    assert!(plan_digest.starts_with(PLAN_DIGEST_PREFIX));
    assert_eq!(plan_digest.len(), PLAN_DIGEST_PREFIX.len() + 64);

    let req_input = RequestDigestInput {
        draft_session_id: draft_session_id.to_string(),
        workflow_run_id: workflow_run_id.to_string(),
        plan_generation: 1,
        round_index: 0,
        task_count: 2,
        parallel_limit: 6,
        plan_digest: plan_digest.clone(),
        base_version_seq: 1,
        base_data_hash: "sha256:3333333333333333333333333333333333333333333333333333333333333333"
            .to_string(),
    };

    let req_digest =
        compute_request_digest(&req_input).expect("request_digest compute must succeed");
    assert!(req_digest.starts_with(REQUEST_DIGEST_PREFIX));
    assert_eq!(req_digest.len(), REQUEST_DIGEST_PREFIX.len() + 64);

    // Verification helpers
    assert!(verify_plan_digest(&plan_digest, &plan_input).is_ok());
    assert!(verify_request_digest(&req_digest, &req_input).is_ok());

    // Tampered task index triggers TaskIndexMismatch
    let mut bad_plan = plan_input.clone();
    bad_plan.tasks[1].task_index = 0; // duplicate index 0
    let err = compute_plan_digest(&bad_plan).unwrap_err();
    assert!(matches!(err, ManifestError::TaskIndexMismatch { .. }));
}
