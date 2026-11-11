#![allow(clippy::field_reassign_with_default)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::result_large_err)]
#![allow(clippy::result_unit_err)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]

//! Integration and acceptance tests for `phase2_sprint2_deep1` ("Freeze pure domain types and canonical state").
//!
//! Per the backend module wiring rule in `docs/plan/phase2_sprint2_deep1.md`:
//! - This sprint does not edit `src-tauri/src/domain/mod.rs` or `src-tauri/src/lib.rs`.
//! - Instead, this test target loads the 4 owned implementation modules directly via `#[path = "..."]`.
//!
//! Covered acceptance vectors:
//! - `P2S2-P`: Canonical RFC 8785 JCS bytes are identical for reordered object keys and missing vs
//!   explicit-null optional `TrackedField`s, but differ when semantic arrays (`event_blocks`, `arguments`) are reordered.
//! - `P2S2-N`: Protected provenance source (`user`, `user_edited`) and `meta.updated_at` exclusion verified;
//!   no database or filesystem side effects.
//! - `P2S2-B`: Deleting an entity then allocating a new one yields the next suffix and never reuses a deleted `code`.
//! - `T-UNIT-02`: Create Project, 2 Arcs, 3 Chapters, delete Chapter 2 then create a new Chapter, and rename Arc 1's
//!   `display_name`; Arc 1's `code` stays unchanged, the new Chapter receives `-c4` (never reusing `-c2`), and `_counters`
//!   increases monotonically.
//! - `T-UNIT-05`: Multilingual word count for Vietnamese, English, Japanese, and Chinese matches UAX #29 + CJK rules.
//! - `phase2_sprint2_deep1-F1`: Injecting an unknown/malformed State shape returns a typed validation error (`P2S2-N`)
//!   with zero mutation of the caller's state.
//! - `phase2_sprint2_deep1-F2`: Injecting a `code` collision or `_counters` rewind is rejected (`P2S2-C`) without
//!   publishing the candidate state.
//! - `phase2_sprint2_deep1-F0`: Predecessor gate absent or baseline SHA drift returns `CONTRACT_BLOCKED` before
//!   implementation starts.

mod common;

#[path = "../src/domain/business_flow.rs"]
pub mod business_flow;

#[path = "../src/domain/code_allocator.rs"]
pub mod code_allocator;

#[path = "../src/domain/fingerprint.rs"]
pub mod fingerprint;

#[path = "../src/domain/word_count.rs"]
pub mod word_count;

pub mod domain {
    pub use crate::business_flow;
    pub use crate::code_allocator;
    pub use crate::fingerprint;
    pub use crate::word_count;
}

use domain::business_flow::{
    BusinessFlowData, ChapterPhaseArcItem, ChapterPhaseChapterItem, ChapterPhaseSceneItem,
    DecisionLogEntry, DomainError, DraftMeta, EntityCounters, Language, OverviewArcItem,
    ProvenanceSource, RequireCharacterItem, RequireGenreItem, TrackedField, UserConstraintEntry,
};
use domain::code_allocator::{CodeAllocator, EntityCodeSpec};
use domain::fingerprint::{
    canonical_json_value, canonical_state, canonical_state_from_json_value, state_data_hash,
    states_are_semantically_equal, CanonicalStateBytes,
};
use domain::word_count::{count_words, count_words_for_language};
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::path::PathBuf;

const BASELINE_COMMIT: &str = "ddda30cd3c842113599c7bfb7d0e221936330428";
const PREDECESSOR_P2S1_COMMIT: &str = "cc6fe79511e4349b5fa22815d4d7a06ccc4cfad2";

/// Helper to validate predecessor gate and baseline lineage (`phase2_sprint2_deep1-F0`).
pub fn verify_p2s2_dispatch_preconditions(
    expected_baseline: &str,
    actual_baseline: &str,
    p2s1_gate_accepted: bool,
) -> Result<(), DomainError> {
    if !p2s1_gate_accepted {
        return Err(DomainError::ContractBlocked {
            reason: "predecessor gate_phase2_sprint1_deep0 is absent or not ACCEPTED".to_string(),
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

#[test]
#[allow(non_snake_case)]
fn P2S2_P_canonical_bytes_identical_for_reordered_keys_and_missing_null_but_not_reordered_semantic_arrays(
) {
    // Two JSON documents with completely different object key ordering and omitted optional TrackedFields
    // vs explicit {"value": null, "source": null}, plus different meta.updated_at values.
    let doc_a: Value = serde_json::from_str(
        r#"{
            "meta": {
                "version": "1.0",
                "phase": 1,
                "subphase": null,
                "updated_at": "2026-11-07T08:00:00Z"
            },
            "require": {
                "project": {
                    "code": "te_nu_nguu_nhan",
                    "display_name": { "value": "Tế Nữ Ngưu Nhân", "source": "user" },
                    "summary": { "value": null, "source": null }
                },
                "character": []
            },
            "optional": {
                "overview_phase": {
                    "arcs": [
                        {
                            "code": "te_nu_nguu_nhan-a1",
                            "display_name": { "value": "Hồi 1: Khởi Nguyên", "source": "ai_generated" },
                            "motif": {
                                "code": { "value": "revenge_climb", "source": "user_edited" },
                                "event_blocks": [
                                    {
                                        "code": "te_nu_nguu_nhan-a1-eb1",
                                        "start_chapter": { "value": 1, "source": "user" },
                                        "end_chapter": { "value": 5, "source": "user" },
                                        "summary": { "value": "Biến cố mở màn", "source": "user" }
                                    },
                                    {
                                        "code": "te_nu_nguu_nhan-a1-eb2",
                                        "start_chapter": { "value": 6, "source": "ai_generated" },
                                        "end_chapter": { "value": 10, "source": "ai_generated" },
                                        "summary": { "value": "Cao trào hồi 1", "source": "ai_generated" }
                                    }
                                ]
                            },
                            "themes": {
                                "question": { "value": "Công lý thuộc về ai?", "source": "user" },
                                "arguments": [
                                    {
                                        "code": "arg-1",
                                        "character_code": { "value": "minh_hoang", "source": "user" },
                                        "role": { "value": "PRIMARY", "source": "user" }
                                    },
                                    {
                                        "code": "arg-2",
                                        "character_code": { "value": "hac_de", "source": "ai_generated" },
                                        "role": { "value": "ANTITHESIS", "source": "ai_generated" }
                                    }
                                ]
                            }
                        }
                    ]
                }
            },
            "_counters": {
                "te_nu_nguu_nhan": 1,
                "te_nu_nguu_nhan-a1:eb": 2
            }
        }"#,
    )
    .expect("doc_a must be valid JSON");

    // Reorder keys at every object level, omit `summary` entirely (vs explicit null), and change `updated_at`.
    let doc_b: Value = serde_json::from_str(
        r#"{
            "_counters": {
                "te_nu_nguu_nhan-a1:eb": 2,
                "te_nu_nguu_nhan": 1
            },
            "optional": {
                "overview_phase": {
                    "arcs": [
                        {
                            "themes": {
                                "arguments": [
                                    {
                                        "role": { "source": "user", "value": "PRIMARY" },
                                        "character_code": { "source": "user", "value": "minh_hoang" },
                                        "code": "arg-1"
                                    },
                                    {
                                        "character_code": { "value": "hac_de", "source": "ai_generated" },
                                        "code": "arg-2",
                                        "role": { "value": "ANTITHESIS", "source": "ai_generated" }
                                    }
                                ],
                                "question": { "source": "user", "value": "Công lý thuộc về ai?" }
                            },
                            "motif": {
                                "event_blocks": [
                                    {
                                        "summary": { "source": "user", "value": "Biến cố mở màn" },
                                        "end_chapter": { "source": "user", "value": 5 },
                                        "start_chapter": { "source": "user", "value": 1 },
                                        "code": "te_nu_nguu_nhan-a1-eb1"
                                    },
                                    {
                                        "end_chapter": { "value": 10, "source": "ai_generated" },
                                        "code": "te_nu_nguu_nhan-a1-eb2",
                                        "summary": { "value": "Cao trào hồi 1", "source": "ai_generated" },
                                        "start_chapter": { "value": 6, "source": "ai_generated" }
                                    }
                                ],
                                "code": { "source": "user_edited", "value": "revenge_climb" }
                            },
                            "display_name": { "source": "ai_generated", "value": "Hồi 1: Khởi Nguyên" },
                            "code": "te_nu_nguu_nhan-a1"
                        }
                    ]
                }
            },
            "require": {
                "character": [],
                "project": {
                    "display_name": { "source": "user", "value": "Tế Nữ Ngưu Nhân" },
                    "code": "te_nu_nguu_nhan"
                }
            },
            "meta": {
                "updated_at": "2099-12-31T23:59:59Z",
                "phase": 1,
                "version": "1.0"
            }
        }"#,
    )
    .expect("doc_b must be valid JSON");

    let bytes_a = canonical_state_from_json_value(&doc_a).expect("doc_a canonicalize");
    let bytes_b = canonical_state_from_json_value(&doc_b).expect("doc_b canonicalize");
    assert_eq!(
        bytes_a, bytes_b,
        "Reordered object keys, omitted vs explicit-null TrackedField, and changed meta.updated_at must yield identical JCS bytes"
    );

    let state_a = BusinessFlowData::from_json_value_strict(&doc_a).unwrap();
    let state_b = BusinessFlowData::from_json_value_strict(&doc_b).unwrap();
    assert!(states_are_semantically_equal(&state_a, &state_b).unwrap());
    assert_eq!(
        state_data_hash(&state_a).unwrap(),
        state_data_hash(&state_b).unwrap()
    );

    // Now reorder a semantic array (`event_blocks` and `themes.arguments`) and verify canonical bytes DIFFER.
    let mut state_reordered_event_blocks = state_a.clone();
    state_reordered_event_blocks.optional.overview_phase.arcs[0]
        .motif
        .event_blocks
        .reverse();
    let bytes_reordered_eb = canonical_state(&state_reordered_event_blocks).unwrap();
    assert_ne!(
        bytes_a, bytes_reordered_eb,
        "Reordering semantic array event_blocks[] must change canonical JCS bytes"
    );
    assert!(!states_are_semantically_equal(&state_a, &state_reordered_event_blocks).unwrap());

    let mut state_reordered_args = state_a.clone();
    state_reordered_args.optional.overview_phase.arcs[0]
        .themes
        .arguments
        .reverse();
    let bytes_reordered_args = canonical_state(&state_reordered_args).unwrap();
    assert_ne!(
        bytes_a, bytes_reordered_args,
        "Reordering semantic array themes.arguments[] must change canonical JCS bytes"
    );
}

#[test]
#[allow(non_snake_case)]
fn P2S2_N_protected_source_and_meta_updated_at_exclusion_verified_with_no_db_writes() {
    let mut state = BusinessFlowData {
        meta: DraftMeta {
            version: "1.0".to_string(),
            phase: 0,
            subphase: None,
            updated_at: Some("2026-11-07T00:00:00Z".to_string()),
        },
        ..Default::default()
    };
    state.require.project.code = Some("p1".to_string());
    state.require.project.display_name = TrackedField::user("Thiên Đạo Đồ Thư Quán".to_string());
    state.require.project.summary = TrackedField::ai("Bản tóm tắt do AI đề xuất".to_string());
    state.require.project.language = TrackedField::user_edited("VI".to_string());
    state.require.project.genres.push(RequireGenreItem {
        code: TrackedField::user("tien_hiep".to_string()),
        is_primary: TrackedField::user(true),
        display_name: TrackedField::unset(),
    });
    state.user_constraints.push(UserConstraintEntry {
        id: "uc-1".to_string(),
        content: "Không có hậu cung".to_string(),
        source: ProvenanceSource::User,
    });
    state.decision_log.push(DecisionLogEntry {
        id: "dl-1".to_string(),
        summary: "Chốt 5 Arc".to_string(),
        source: ProvenanceSource::UserEdited,
    });

    // Verify provenance protection rules (`core.md` §1.3).
    assert!(
        state
            .require
            .project
            .display_name
            .is_protected_from_ai_overwrite(),
        "user source must be protected from direct AI overwrite"
    );
    assert!(
        state
            .require
            .project
            .language
            .is_protected_from_ai_overwrite(),
        "user_edited source must be protected from direct AI overwrite"
    );
    assert!(
        !state
            .require
            .project
            .summary
            .is_protected_from_ai_overwrite(),
        "ai_generated source must allow AI overwrite"
    );
    assert!(
        !TrackedField::<String>::unset().is_protected_from_ai_overwrite(),
        "unset/null field must not be marked protected"
    );

    // Changing only `meta.updated_at` must not change canonical bytes or SHA-256 hash.
    let canonical_before = CanonicalStateBytes::from_state(&state).unwrap();
    let hash_before = state_data_hash(&state).unwrap();
    assert!(hash_before.starts_with("sha256:"));
    assert_eq!(hash_before.len(), "sha256:".len() + 64);

    let canonical_str = std::str::from_utf8(canonical_before.as_bytes()).unwrap();
    assert!(
        !canonical_str.contains("updated_at"),
        "canonical JCS bytes must exclude meta.updated_at"
    );

    let mut state_later = state.clone();
    state_later.meta.updated_at = Some("2030-01-01T12:34:56Z".to_string());
    assert_eq!(
        CanonicalStateBytes::from_state(&state_later).unwrap(),
        canonical_before
    );
    assert_eq!(state_data_hash(&state_later).unwrap(), hash_before);

    // Changing a field's `source` from `ai_generated` to `user_edited` (even with the exact same `value`)
    // MUST change the canonical bytes and `data_hash` (`be-core.md` §4.3, §6.4.2).
    let mut state_source_upgraded = state.clone();
    state_source_upgraded.require.project.summary.source = Some(ProvenanceSource::UserEdited);
    assert!(!states_are_semantically_equal(&state, &state_source_upgraded).unwrap());
    assert_ne!(
        state_data_hash(&state).unwrap(),
        state_data_hash(&state_source_upgraded).unwrap()
    );

    // Changing `_counters` MUST change the round-level `data_hash` (`be-core.md` §4.3, §6.4.2).
    let mut state_counter_changed = state.clone();
    state_counter_changed.counters.insert("p1".to_string(), 1);
    assert!(!states_are_semantically_equal(&state, &state_counter_changed).unwrap());
    assert_ne!(
        state_data_hash(&state).unwrap(),
        state_data_hash(&state_counter_changed).unwrap()
    );

    // Verify no SQLite DB or app-data directory was created in the repository root.
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf();
    assert!(
        !repo_root.join("app-data").exists(),
        "Pure domain tests must never create app-data or SQLite files"
    );
}

#[test]
#[allow(non_snake_case)]
fn P2S2_B_delete_then_allocate_yields_next_suffix_and_never_reuses_deleted_code() {
    let mut state = BusinessFlowData::new();
    let project_code =
        CodeAllocator::allocate(&mut state, EntityCodeSpec::Project { display_name: "p1" })
            .unwrap();
    assert_eq!(project_code, "p1");
    state.require.project.code = Some(project_code.clone());

    // Allocate 2 scenes under chapter p1-a1-c1, delete scene 1, then allocate another scene.
    let s1 = CodeAllocator::allocate(
        &mut state,
        EntityCodeSpec::Scene {
            chapter_code: "p1-a1-c1",
        },
    )
    .unwrap();
    let s2 = CodeAllocator::allocate(
        &mut state,
        EntityCodeSpec::Scene {
            chapter_code: "p1-a1-c1",
        },
    )
    .unwrap();
    assert_eq!(s1, "p1-a1-c1-s1");
    assert_eq!(s2, "p1-a1-c1-s2");

    state.optional.chapter_phase.arcs.push(ChapterPhaseArcItem {
        code: "p1-a1".to_string(),
        chapters: vec![ChapterPhaseChapterItem {
            code: "p1-a1-c1".to_string(),
            scenes: vec![
                ChapterPhaseSceneItem {
                    code: s1.clone(),
                    ..Default::default()
                },
                ChapterPhaseSceneItem {
                    code: s2.clone(),
                    ..Default::default()
                },
            ],
            ..Default::default()
        }],
    });

    // Delete s1 and s2 physically from state
    state.optional.chapter_phase.arcs[0].chapters[0]
        .scenes
        .clear();

    // Next allocated scene must be p1-a1-c1-s3, never reusing s1 or s2
    let s3 = CodeAllocator::allocate(
        &mut state,
        EntityCodeSpec::Scene {
            chapter_code: "p1-a1-c1",
        },
    )
    .unwrap();
    assert_eq!(s3, "p1-a1-c1-s3");
    assert_eq!(state.counters.get("p1-a1-c1").copied(), Some(3));
}

#[test]
#[allow(non_snake_case)]
fn T_UNIT_02_code_allocator_and_monotone_counters_across_delete_and_rename() {
    // Scenario from `docs/business/be-core.md#L2524`:
    // "Tạo Project, 2 Arc, 3 Chapter, xóa Chapter 2 rồi tạo Chapter mới; đổi `display_name` của Arc 1."
    // Expected:
    // "`code` của Arc 1 giữ nguyên khi đổi `display_name`. Chapter mới sau khi xóa Chapter 2 nhận hậu tố `-c4`
    // (không tái sử dụng `-c2`). `_counters` tăng đơn điệu."
    let mut state = BusinessFlowData::new();

    // 1. Create Project
    let project_code = CodeAllocator::allocate(
        &mut state,
        EntityCodeSpec::Project {
            display_name: "Tế Nữ Ngưu Nhân",
        },
    )
    .unwrap();
    assert_eq!(project_code, "te_nu_nguu_nhan");
    state.require.project.code = Some(project_code.clone());
    state.require.project.display_name = TrackedField::user("Tế Nữ Ngưu Nhân".to_string());

    // 2. Create 2 Arcs
    let arc1_code = CodeAllocator::allocate(
        &mut state,
        EntityCodeSpec::Arc {
            project_code: &project_code,
        },
    )
    .unwrap();
    let arc2_code = CodeAllocator::allocate(
        &mut state,
        EntityCodeSpec::Arc {
            project_code: &project_code,
        },
    )
    .unwrap();
    assert_eq!(arc1_code, "te_nu_nguu_nhan-a1");
    assert_eq!(arc2_code, "te_nu_nguu_nhan-a2");
    assert_eq!(state.counters.get(&project_code).copied(), Some(2));

    state.optional.overview_phase.arcs.push(OverviewArcItem {
        code: arc1_code.clone(),
        display_name: TrackedField::user("Hồi 1: Tuyệt Cảnh".to_string()),
        position: TrackedField::user(1),
        ..Default::default()
    });
    state.optional.overview_phase.arcs.push(OverviewArcItem {
        code: arc2_code.clone(),
        display_name: TrackedField::user("Hồi 2: Phản Kích".to_string()),
        position: TrackedField::user(2),
        ..Default::default()
    });
    state.optional.chapter_phase.arcs.push(ChapterPhaseArcItem {
        code: arc1_code.clone(),
        chapters: vec![],
    });
    state.optional.chapter_phase.arcs.push(ChapterPhaseArcItem {
        code: arc2_code.clone(),
        chapters: vec![],
    });

    // 3. Create 3 Chapters under Arc 1
    let ch1_code = CodeAllocator::allocate(
        &mut state,
        EntityCodeSpec::Chapter {
            arc_code: &arc1_code,
        },
    )
    .unwrap();
    let ch2_code = CodeAllocator::allocate(
        &mut state,
        EntityCodeSpec::Chapter {
            arc_code: &arc1_code,
        },
    )
    .unwrap();
    let ch3_code = CodeAllocator::allocate(
        &mut state,
        EntityCodeSpec::Chapter {
            arc_code: &arc1_code,
        },
    )
    .unwrap();

    assert_eq!(ch1_code, "te_nu_nguu_nhan-a1-c1");
    assert_eq!(ch2_code, "te_nu_nguu_nhan-a1-c2");
    assert_eq!(ch3_code, "te_nu_nguu_nhan-a1-c3");
    assert_eq!(state.counters.get(&arc1_code).copied(), Some(3));

    state.optional.chapter_phase.arcs[0].chapters = vec![
        ChapterPhaseChapterItem {
            code: ch1_code.clone(),
            display_name: TrackedField::user("Chương 1".to_string()),
            position: TrackedField::user(1),
            ..Default::default()
        },
        ChapterPhaseChapterItem {
            code: ch2_code.clone(),
            display_name: TrackedField::user("Chương 2".to_string()),
            position: TrackedField::user(2),
            ..Default::default()
        },
        ChapterPhaseChapterItem {
            code: ch3_code.clone(),
            display_name: TrackedField::user("Chương 3".to_string()),
            position: TrackedField::user(3),
            ..Default::default()
        },
    ];

    // 4. Delete Chapter 2 physically from WorkingState
    state.optional.chapter_phase.arcs[0]
        .chapters
        .retain(|c| c.code != ch2_code);
    assert_eq!(state.optional.chapter_phase.arcs[0].chapters.len(), 2);
    assert_eq!(
        state.counters.get(&arc1_code).copied(),
        Some(3),
        "Deleting Chapter 2 must never decrement _counters"
    );

    // 5. Create a new Chapter under Arc 1 -> must receive suffix -c4, never reusing -c2
    let ch4_code = CodeAllocator::allocate(
        &mut state,
        EntityCodeSpec::Chapter {
            arc_code: &arc1_code,
        },
    )
    .unwrap();
    assert_eq!(ch4_code, "te_nu_nguu_nhan-a1-c4");
    assert_eq!(state.counters.get(&arc1_code).copied(), Some(4));
    state.optional.chapter_phase.arcs[0]
        .chapters
        .push(ChapterPhaseChapterItem {
            code: ch4_code.clone(),
            display_name: TrackedField::user("Chương 4 Mới".to_string()),
            position: TrackedField::user(3),
            ..Default::default()
        });

    // 6. Rename `display_name` of Arc 1 -> `code` of Arc 1 and its children must remain unchanged
    state.optional.overview_phase.arcs[0].display_name =
        TrackedField::user_edited("Hồi 1: Đổi Tên Mới Hoàn Toàn".to_string());
    assert_eq!(
        state.optional.overview_phase.arcs[0].code, "te_nu_nguu_nhan-a1",
        "Arc 1 code must remain immutable when display_name changes"
    );
    assert_eq!(
        state.optional.chapter_phase.arcs[0].chapters[2].code,
        "te_nu_nguu_nhan-a1-c4"
    );
}

#[test]
#[allow(non_snake_case)]
fn T_UNIT_05_multilingual_word_count_matches_uax29_and_cjk_rules() {
    // Scenario from `docs/business/be-core.md#L2527`:
    // "Đếm văn bản Tiếng Việt, Tiếng Anh, Tiếng Nhật và Tiếng Trung."
    // Expected: "Trả về số lượng từ tất định khớp quy tắc UAX #29 + CJK."

    // 1. Vietnamese (each space-separated syllable is 1 word under UAX #29)
    let vi_text = "Đêm xuống trên đỉnh Hoa Sơn, gió lạnh thổi qua rừng trúc.";
    // Syllables: Đêm(1) xuống(2) trên(3) đỉnh(4) Hoa(5) Sơn(6) gió(7) lạnh(8) thổi(9) qua(10) rừng(11) trúc(12) = 12
    assert_eq!(count_words(vi_text), 12);
    assert_eq!(count_words_for_language(vi_text, Language::Vi), 12);

    // 2. English (UAX #29 words, contractions like "don't" count as 1 word, punctuation ignored)
    let en_text = "A hero's journey doesn't end when the battle is won — it begins anew.";
    // A(1) hero's(2) journey(3) doesn't(4) end(5) when(6) the(7) battle(8) is(9) won(10) it(11) begins(12) anew(13) = 13
    assert_eq!(count_words(en_text), 13);
    assert_eq!(count_words_for_language(en_text, Language::En), 13);

    // 3. Japanese (Han + Hiragana + Katakana characters each count as 1 unit, punctuation ignored)
    // "桜" (1) "の" (2) "花" (3) "び" (4) "ら" (5) "が" (6) "風" (7) "に" (8) "舞" (9) "う" (10) "。" (ignored) = 10
    let ja_text = "桜の花びらが風に舞う。";
    assert_eq!(count_words(ja_text), 10);
    assert_eq!(count_words_for_language(ja_text, Language::Ja), 10);

    // Japanese with Katakana + Latin number:
    // "ア"(1) "イ"(2) "ス"(3) "100"(4) "個"(5) = 5
    assert_eq!(count_words("アイス100個"), 5);

    // 4. Chinese Traditional & Simplified (each Han ideograph is 1 word unit, punctuation ignored)
    // "天"(1) "下"(2) "風"(3) "雲"(4) "出"(5) "我"(6) "輩"(7) "，"(0) "一"(8) "入"(9) "江"(10) "湖"(11) "歲"(12) "月"(13) "催"(14) "。"(0) = 14
    let zh_hant_text = "天下風雲出我輩，一入江湖歲月催。";
    assert_eq!(count_words(zh_hant_text), 14);
    assert_eq!(count_words_for_language(zh_hant_text, Language::ZhHant), 14);

    let zh_hans_text = "天下风云出我辈，一入江湖岁月催。";
    assert_eq!(count_words(zh_hans_text), 14);
    assert_eq!(count_words_for_language(zh_hans_text, Language::ZhHans), 14);

    // Empty and punctuation-only inputs return 0
    assert_eq!(count_words(""), 0);
    assert_eq!(count_words("   ... — ！ ？ 「 」 \n\t "), 0);
}

#[test]
#[allow(non_snake_case)]
fn phase2_sprint2_deep1_F1_unknown_state_shape_returns_typed_validation_error_with_no_mutation() {
    let mut valid_state = BusinessFlowData::new();
    valid_state.require.project.code = Some("p1".to_string());
    valid_state.require.project.display_name = TrackedField::user("Truyện Gốc".to_string());
    let snapshot_before = canonical_state(&valid_state).unwrap();

    // Case 1: Unknown top-level field (e.g. trying to embed `pending_patches` inside sealed BusinessFlowData)
    let unknown_top_level = json!({
        "meta": { "version": "1.0", "phase": 0 },
        "require": {},
        "pending_patches": [{ "id": "patch-1" }]
    });
    let err1 = BusinessFlowData::from_json_value_strict(&unknown_top_level).unwrap_err();
    assert!(
        matches!(err1, DomainError::UnknownStateShape { .. }),
        "Expected UnknownStateShape (P2S2-N), got: {err1:?}"
    );
    assert!(err1.to_string().contains("P2S2-N"));

    // Case 2: Prototype misspelled phase key `charactar_psychological_phase`
    let misspelled_phase = json!({
        "meta": { "version": "1.0", "phase": 5, "subphase": 2 },
        "optional": {
            "charactar_psychological_phase": { "character": [] }
        }
    });
    let err2 = BusinessFlowData::from_json_value_strict(&misspelled_phase).unwrap_err();
    assert!(matches!(err2, DomainError::UnknownStateShape { .. }));

    // Case 3: Invalid subphase on non-Phase-5
    let bad_subphase = json!({
        "meta": { "version": "1.0", "phase": 3, "subphase": 2 }
    });
    let err3 = BusinessFlowData::from_json_value_strict(&bad_subphase).unwrap_err();
    assert_eq!(
        err3,
        DomainError::InvalidPhaseSubphase {
            phase: 3,
            subphase: Some(2),
        }
    );
    assert!(err3.to_string().contains("P2S2-N"));

    // Verify existing valid_state was never mutated by any failed parse/validation attempt.
    assert_eq!(canonical_state(&valid_state).unwrap(), snapshot_before);
}

#[test]
#[allow(non_snake_case)]
fn phase2_sprint2_deep1_F2_collision_or_counter_rewind_rejected_without_publishing_candidate() {
    let mut state = BusinessFlowData::new();
    let proj_code = CodeAllocator::allocate(
        &mut state,
        EntityCodeSpec::Project {
            display_name: "Thiên Cơ",
        },
    )
    .unwrap();
    state.require.project.code = Some(proj_code.clone());

    let arc1 = CodeAllocator::allocate(
        &mut state,
        EntityCodeSpec::Arc {
            project_code: &proj_code,
        },
    )
    .unwrap();
    let arc2 = CodeAllocator::allocate(
        &mut state,
        EntityCodeSpec::Arc {
            project_code: &proj_code,
        },
    )
    .unwrap();
    state.optional.overview_phase.arcs.push(OverviewArcItem {
        code: arc1.clone(),
        ..Default::default()
    });
    state.optional.overview_phase.arcs.push(OverviewArcItem {
        code: arc2.clone(),
        ..Default::default()
    });
    assert_eq!(state.counters.get(&proj_code).copied(), Some(2));

    let baseline_bytes = canonical_state(&state).unwrap();

    // 1. Attempt to rewind `_counters["thien_co"]` from 2 back to 1 -> must fail with P2S2-C CounterRewind
    let mut rewound_counters: EntityCounters = BTreeMap::new();
    rewound_counters.insert(proj_code.clone(), 1);
    let rewind_err =
        CodeAllocator::validate_and_apply_counter_transition(&mut state, rewound_counters, None)
            .unwrap_err();
    assert_eq!(
        rewind_err,
        DomainError::CounterRewind {
            counter_key: proj_code.clone(),
            current: 2,
            proposed: 1,
        }
    );
    assert!(rewind_err.to_string().contains("P2S2-C"));
    assert_eq!(
        canonical_state(&state).unwrap(),
        baseline_bytes,
        "Rejected counter rewind must leave state completely unchanged"
    );

    // 2. Attempt to register a colliding entity code (`thien_co-a1`) -> must fail with P2S2-C CodeCollision
    let proposed_valid_counters = state.counters.clone();
    let collision_err = CodeAllocator::validate_and_apply_counter_transition(
        &mut state,
        proposed_valid_counters,
        Some(&arc1),
    )
    .unwrap_err();
    assert_eq!(
        collision_err,
        DomainError::CodeCollision { code: arc1.clone() }
    );
    assert!(collision_err.to_string().contains("P2S2-C"));
    assert_eq!(
        canonical_state(&state).unwrap(),
        baseline_bytes,
        "Rejected code collision must leave state completely unchanged"
    );

    // 3. Also verify that if a state somehow contains duplicate codes across collections,
    // `canonical_state` / `CodeAllocator::allocate` rejects it before mutating or publishing.
    let mut corrupted = state.clone();
    corrupted.require.character.push(RequireCharacterItem {
        code: arc1.clone(), // collides with arc1!
        ..Default::default()
    });
    let alloc_err = CodeAllocator::allocate(
        &mut corrupted,
        EntityCodeSpec::Arc {
            project_code: &proj_code,
        },
    )
    .unwrap_err();
    assert_eq!(alloc_err, DomainError::CodeCollision { code: arc1 });
}

#[test]
#[allow(non_snake_case)]
fn phase2_sprint2_deep1_F0_missing_predecessor_or_baseline_drift_is_contract_blocked() {
    // 1. Missing predecessor gate -> CONTRACT_BLOCKED
    let err_missing_gate =
        verify_p2s2_dispatch_preconditions(BASELINE_COMMIT, BASELINE_COMMIT, false).unwrap_err();
    assert!(matches!(
        err_missing_gate,
        DomainError::ContractBlocked { .. }
    ));
    assert!(err_missing_gate.to_string().contains("CONTRACT_BLOCKED"));

    // 2. Baseline SHA drift -> CONTRACT_BLOCKED
    let drifted = "deadbeefdeadbeefdeadbeefdeadbeefdeadbeef";
    let err_drift = verify_p2s2_dispatch_preconditions(BASELINE_COMMIT, drifted, true).unwrap_err();
    assert!(matches!(err_drift, DomainError::ContractBlocked { .. }));
    assert!(err_drift.to_string().contains("CONTRACT_BLOCKED"));

    // 3. Positive verification in the live worktree: BASELINE_COMMIT is an ancestor of HEAD, and PREDECESSOR_P2S1_COMMIT exists in git history
    let _repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf();
    assert!(
        common::verify_commit(BASELINE_COMMIT),
        "lineage commit {} must be an ancestor, a local object, or recorded in the lineage manifest",
        BASELINE_COMMIT
    );
    assert!(
        common::verify_commit(PREDECESSOR_P2S1_COMMIT),
        "lineage commit {} must be an ancestor, a local object, or recorded in the lineage manifest",
        PREDECESSOR_P2S1_COMMIT
    );

    assert!(verify_p2s2_dispatch_preconditions(BASELINE_COMMIT, BASELINE_COMMIT, true).is_ok());

    // Ensure RFC 8785 JCS helper is also directly exercised via public re-export
    let sample = json!({"b": 1, "a": 2});
    assert_eq!(
        canonical_json_value(&sample).unwrap(),
        b"{\"a\":2,\"b\":1}".to_vec()
    );
}
