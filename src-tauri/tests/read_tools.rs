#![allow(non_snake_case)]

//! Acceptance & Integration tests for `phase3_sprint4_deep1`
//! ("Implement catalog queries and read tools").
//!
//! Per the backend module wiring rule in `docs/plan/phase3_sprint4_deep1.md`:
//! - This sprint does not edit any `mod.rs` or `lib.rs`.
//! - Instead, this test target loads the 2 owned implementation modules (`registry.rs`, `read_tools.rs`)
//!   and predecessor modules (`business_flow.rs`, `error.rs`) via `#[path = "..."]`.
//!
//! Acceptance vectors covered:
//! - `P3S4-P`: `get_field_null` returns <= 3 deterministic questions in system_language.
//! - `P3S4-N`: AI cannot request UI-only confirm/lock via registry (rejected with `ErrorCode::UnauthorizedCaller`).
//! - `P3S4-B`: Empty array `[]` and missing/null field differ cleanly.
//! - `phase3_sprint4_deep1-F1`: Unknown catalog code yields structured candidates, no mutation; P3S4-N.
//! - `phase3_sprint4_deep1-F2`: Missing translator locale triggers documented `en` fallback, no provider call.
//! - `phase3_sprint4_deep1-F0`: Predecessor gate absent or baseline drift returns `CONTRACT_BLOCKED`.
//! - `REQ-TOOLS-003`: Read tools never mutate state or create draft versions.

#[path = "../src/error.rs"]
pub mod error;

#[path = "../src/domain/business_flow.rs"]
pub mod business_flow;

#[path = "../src/domain/fingerprint.rs"]
pub mod fingerprint;

#[path = "../src/domain/tools/registry.rs"]
pub mod registry;

#[path = "../src/domain/tools/read_tools.rs"]
pub mod read_tools;

pub mod domain {
    pub use crate::business_flow;
    pub use crate::fingerprint;
    pub mod tools {
        pub use crate::read_tools;
        pub use crate::registry;
    }
}

use std::path::PathBuf;
use std::process::Command;

use crate::business_flow::{BusinessFlowData, TrackedField};
use crate::error::ErrorCode;
use crate::fingerprint::state_data_hash;
use crate::read_tools::{
    get_array_element_fields, get_field_null, get_state, resolve_catalog_genre, search_genres,
    search_lore, search_motifs, search_world_settings, CatalogRepo, CatalogRepoError,
    CharacterRecord, GenreRecord, LoreEntryRecord, MotifRecord, WorldSettingRecord,
};
use crate::registry::{AllowedCaller, CallerCategory, ToolRegistry, ToolType};

const BASELINE_COMMIT: &str = "57a462efb953a665e3673b15e6cc33622a4f2d39";
const PREDECESSOR_P3S1_COMMIT: &str = "702843c38e45b193be25b80222a48eadedfd3538";
const PREDECESSOR_P2S6_COMMIT: &str = "75d3886503643e6398c0be8c600b9b712811b3e0";

/// Pure Rust in-memory fixture implementing `CatalogRepo` (`P3S4-P`, `P3S4-N`, `P3S4-B`).
#[derive(Debug, Clone, Default)]
struct MockCatalogRepo {
    genres: Vec<GenreRecord>,
    motifs: Vec<MotifRecord>,
    world_settings: Vec<WorldSettingRecord>,
    lore_entries: Vec<LoreEntryRecord>,
    characters: Vec<CharacterRecord>,
}

impl MockCatalogRepo {
    fn new_seeded() -> Self {
        Self {
            genres: vec![
                GenreRecord {
                    id: "g1".to_string(),
                    code: Some("fantasy".to_string()),
                    display_name: Some("Fantasy".to_string()),
                    description: Some("Magical and mythical worlds".to_string()),
                },
                GenreRecord {
                    id: "g2".to_string(),
                    code: Some("scifi".to_string()),
                    display_name: Some("Science Fiction".to_string()),
                    description: Some("Future technology and space exploration".to_string()),
                },
                GenreRecord {
                    id: "g3".to_string(),
                    code: Some("mystery".to_string()),
                    display_name: Some("Mystery".to_string()),
                    description: Some("Detective and puzzle solving".to_string()),
                },
            ],
            motifs: vec![
                MotifRecord {
                    id: "m1".to_string(),
                    code: Some("chosen_one".to_string()),
                    display_name: Some("The Chosen One".to_string()),
                    category: Some("Hero".to_string()),
                    version: Some("1.0".to_string()),
                    is_system: true,
                    description: Some("A protagonist destined to save the realm".to_string()),
                    min_chapter: Some(1),
                    max_chapter: None,
                },
                MotifRecord {
                    id: "m2".to_string(),
                    code: Some("mentor_death".to_string()),
                    display_name: Some("Death of the Mentor".to_string()),
                    category: Some("Plot".to_string()),
                    version: Some("1.0".to_string()),
                    is_system: true,
                    description: Some("Tragic passing of an experienced guide".to_string()),
                    min_chapter: Some(5),
                    max_chapter: None,
                },
            ],
            world_settings: vec![WorldSettingRecord {
                id: "ws1".to_string(),
                code: "elven_forest".to_string(),
                project_id: None,
                display_name: "Elven Forest".to_string(),
                setting_type: "region".to_string(),
                parent_id: None,
                description: Some("Ancient enchanted woods".to_string()),
                access: None,
                spatial_features: None,
                sensory_visual: None,
                sensory_auditory: None,
                sensory_tactile: None,
                sensory_olfactory: None,
                sensory_gustatory: None,
                created_at: None,
                updated_at: None,
                deleted_at: None,
            }],
            lore_entries: vec![LoreEntryRecord {
                id: "lore1".to_string(),
                code: "tree_of_life".to_string(),
                entry_type: "myth".to_string(),
                display_name: "Tree of Life".to_string(),
                description: Some("The primordial world tree".to_string()),
                world_setting_id: Some("ws1".to_string()),
                data: None,
                deleted_at: None,
            }],
            characters: vec![CharacterRecord {
                id: "c1".to_string(),
                code: Some("merlin".to_string()),
                display_name: Some("Merlin".to_string()),
                nickname: Some("Old Mage".to_string()),
                birth: None,
                species: Some("human".to_string()),
                biological_sex: Some("male".to_string()),
                gender_identity: None,
                gender_expression: None,
                deleted_at: None,
            }],
        }
    }
}

impl CatalogRepo for MockCatalogRepo {
    fn get_motif_by_code(&self, code: &str) -> Result<Option<MotifRecord>, CatalogRepoError> {
        Ok(self
            .motifs
            .iter()
            .find(|m| m.code.as_deref() == Some(code))
            .cloned())
    }

    fn list_motifs(&self, category: Option<&str>) -> Result<Vec<MotifRecord>, CatalogRepoError> {
        if let Some(cat) = category {
            Ok(self
                .motifs
                .iter()
                .filter(|m| m.category.as_deref() == Some(cat))
                .cloned()
                .collect())
        } else {
            Ok(self.motifs.clone())
        }
    }

    fn get_genre_by_code(&self, code: &str) -> Result<Option<GenreRecord>, CatalogRepoError> {
        Ok(self
            .genres
            .iter()
            .find(|g| g.code.as_deref() == Some(code))
            .cloned())
    }

    fn list_genres(&self) -> Result<Vec<GenreRecord>, CatalogRepoError> {
        Ok(self.genres.clone())
    }

    fn get_character_by_code(
        &self,
        code: &str,
    ) -> Result<Option<CharacterRecord>, CatalogRepoError> {
        Ok(self
            .characters
            .iter()
            .find(|c| c.code.as_deref() == Some(code))
            .cloned())
    }

    fn list_characters(
        &self,
        _include_deleted: bool,
    ) -> Result<Vec<CharacterRecord>, CatalogRepoError> {
        Ok(self.characters.clone())
    }

    fn get_world_setting_by_code(
        &self,
        code: &str,
    ) -> Result<Option<WorldSettingRecord>, CatalogRepoError> {
        Ok(self
            .world_settings
            .iter()
            .find(|w| w.code == code)
            .cloned())
    }

    fn list_world_settings(
        &self,
        project_id: Option<&str>,
        _include_deleted: bool,
    ) -> Result<Vec<WorldSettingRecord>, CatalogRepoError> {
        if let Some(pid) = project_id {
            Ok(self
                .world_settings
                .iter()
                .filter(|w| w.project_id.as_deref() == Some(pid))
                .cloned()
                .collect())
        } else {
            Ok(self.world_settings.clone())
        }
    }

    fn get_lore_entry_by_code(
        &self,
        code: &str,
    ) -> Result<Option<LoreEntryRecord>, CatalogRepoError> {
        Ok(self.lore_entries.iter().find(|l| l.code == code).cloned())
    }

    fn list_lore_entries(
        &self,
        world_setting_id: Option<&str>,
        _include_deleted: bool,
    ) -> Result<Vec<LoreEntryRecord>, CatalogRepoError> {
        if let Some(wsid) = world_setting_id {
            Ok(self
                .lore_entries
                .iter()
                .filter(|l| l.world_setting_id.as_deref() == Some(wsid))
                .cloned()
                .collect())
        } else {
            Ok(self.lore_entries.clone())
        }
    }
}

/// Helper to simulate predecessor check and dispatch gating (`phase3_sprint4_deep1-F0`).
fn verify_p3s4_dispatch_preconditions(
    expected_baseline: &str,
    actual_baseline: &str,
    p3s1_gate_accepted: bool,
    p2s6_gate_accepted: bool,
) -> Result<(), ErrorCode> {
    if !p3s1_gate_accepted || !p2s6_gate_accepted {
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

/// `P3S4-P`: `get_field_null` returns <= 3 deterministic questions.
///
/// Invariants:
/// - Maximum of 3 questions even if limit is larger or omitted.
/// - `system_language` determines question language without AI supplying language parameter.
/// - Results are deterministic in order.
#[test]
fn test_P3S4_P_get_field_null_returns_max_three_deterministic_questions() {
    let data = BusinessFlowData::new();

    // 1. Unbounded / default limit returns exactly 3 questions
    let questions_vi = get_field_null(&data, None, "vi");
    assert_eq!(
        questions_vi.len(),
        3,
        "Must return exactly 3 questions when limit is None and >=3 fields are null"
    );
    assert_eq!(questions_vi[0].field, "require.project.display_name");
    assert_eq!(questions_vi[0].question, "Bạn muốn đặt tên truyện là gì?");
    assert_eq!(questions_vi[1].field, "require.project.summary");
    assert_eq!(questions_vi[2].field, "require.project.language");

    // 2. English system_language returns English questions
    let questions_en = get_field_null(&data, None, "en");
    assert_eq!(questions_en.len(), 3);
    assert_eq!(questions_en[0].field, "require.project.display_name");
    assert_eq!(questions_en[0].question, "What is the title of your story?");
    assert_eq!(questions_en[1].field, "require.project.summary");
    assert_eq!(questions_en[2].field, "require.project.language");

    // 3. Exceeding limit (e.g. limit = 10) is strictly capped at 3
    let capped = get_field_null(&data, Some(10), "vi");
    assert_eq!(capped.len(), 3, "Limit > 3 must be capped at 3");

    // 4. Smaller limit (e.g. limit = 1 or 2) is respected
    let single = get_field_null(&data, Some(1), "vi");
    assert_eq!(single.len(), 1);
    assert_eq!(single[0].field, "require.project.display_name");

    // 5. Partially populated data advances to remaining null fields deterministically
    let mut partially_populated = BusinessFlowData::new();
    partially_populated.require.project.display_name = TrackedField::user("Thiên Đạo".to_string());
    partially_populated.require.project.summary = TrackedField::user("Tóm tắt truyện".to_string());
    partially_populated.require.project.language = TrackedField::user("VI".to_string());

    let next_questions = get_field_null(&partially_populated, None, "vi");
    assert_eq!(next_questions.len(), 3);
    assert_eq!(
        next_questions[0].field,
        "require.project.target.word_per_chapter_count"
    );
    assert_eq!(
        next_questions[1].field,
        "require.project.target.chapter_count"
    );
    assert_eq!(next_questions[2].field, "require.project.target.arc_count");
}

/// `P3S4-N`: AI cannot request UI-only confirm/lock via registry.
///
/// Invariants:
/// - `confirm_phase`, `set_scene_lock`, `lock_scene`, `unlock_scene`, `commit_draft` are `UiCliOnly`.
/// - Calling them with `CallerCategory::AiCaller` returns `Err(ErrorCode::UnauthorizedCaller)`.
/// - Calling them with `CallerCategory::UiAction` or `CallerCategory::Cli` succeeds.
#[test]
fn test_P3S4_N_ai_cannot_request_ui_only_actions_via_registry() {
    let registry = ToolRegistry::new();

    let ui_only_tools = [
        "confirm_phase",
        "set_scene_lock",
        "lock_scene",
        "unlock_scene",
        "commit_draft",
        "confirm_patch",
        "reject_patch",
        "confirm_all_patches",
        "restore_version",
        "write_scene_content",
    ];

    for tool_name in ui_only_tools {
        // AI invocation must be denied
        let err = registry
            .authorize(tool_name, CallerCategory::AiCaller)
            .unwrap_err();
        assert_eq!(
            err,
            ErrorCode::UnauthorizedCaller,
            "AI caller must be blocked from UI-only tool: {tool_name}"
        );

        // UI action invocation must succeed
        let ok_ui = registry
            .authorize(tool_name, CallerCategory::UiAction)
            .unwrap();
        assert_eq!(ok_ui.name, tool_name);
        assert_eq!(ok_ui.allowed_caller, AllowedCaller::UiCliOnly);

        // CLI invocation must succeed
        let ok_cli = registry.authorize(tool_name, CallerCategory::Cli).unwrap();
        assert_eq!(ok_cli.name, tool_name);
    }

    // AI invocation on read tools must succeed
    let read_tools = [
        "get_field_null",
        "get_array_element_fields",
        "get_state",
        "search_genres",
        "search_motifs",
        "search_lore",
        "search_world_settings",
        "search_character",
    ];

    for tool_name in read_tools {
        let meta = registry
            .authorize(tool_name, CallerCategory::AiCaller)
            .unwrap();
        assert_eq!(meta.tool_type, ToolType::Read);
        assert!(!meta.writes_draft_state);
    }
}

/// `P3S4-B`: Empty array `[]` and missing field differ.
///
/// Invariants:
/// - An empty array `[]` is NOT reported as null by `get_field_null`.
/// - `get_array_element_fields` recognizes the array and returns priority creation fields.
/// - When `code` is Some, returns null fields of the specific array element.
#[test]
fn test_P3S4_B_empty_array_and_missing_field_differ() {
    let mut data = BusinessFlowData::new();

    // In initial BusinessFlowData:
    // - require.project.display_name is TrackedField::unset() (missing / null)
    // - require.character is vec![] (empty array, NOT null)
    assert!(data.require.project.display_name.value.is_none());
    assert!(data.require.character.is_empty());

    // 1. get_field_null returns null fields, NEVER includes empty arrays like require.character
    let null_fields = get_field_null(&data, Some(3), "vi");
    assert!(null_fields
        .iter()
        .any(|f| f.field == "require.project.display_name"));
    assert!(
        !null_fields
            .iter()
            .any(|f| f.field.contains("require.character")),
        "Empty array require.character must NOT be reported as null field by get_field_null"
    );

    // 2. get_array_element_fields with code=None on empty array returns priority creation fields
    let char_creation_fields =
        get_array_element_fields(&data, "require.character", None, Some(3), "vi").unwrap();
    assert_eq!(char_creation_fields.len(), 3);
    assert_eq!(char_creation_fields[0].field, "display_name");
    assert_eq!(char_creation_fields[1].field, "role");
    assert_eq!(char_creation_fields[2].field, "biological_sex");

    // 3. Populate a character with partial fields
    let mut char_item = crate::business_flow::RequireCharacterItem::default();
    char_item.code = "char_protagonist".to_string();
    char_item.display_name = TrackedField::user("Lâm Phong".to_string());
    // nickname, role, biological_sex, motif_code remain null
    data.require.character.push(char_item);

    // 4. get_array_element_fields with code=Some("char_protagonist") returns its missing fields
    let element_nulls = get_array_element_fields(
        &data,
        "require.character",
        Some("char_protagonist"),
        Some(3),
        "vi",
    )
    .unwrap();
    assert_eq!(element_nulls.len(), 3);
    assert_eq!(element_nulls[0].field, "nickname");
    assert_eq!(element_nulls[1].field, "role");
    assert_eq!(element_nulls[2].field, "biological_sex");

    // 5. Querying non-existent character code returns ToolRejection with candidates
    let rejection = get_array_element_fields(
        &data,
        "require.character",
        Some("char_nonexistent"),
        Some(3),
        "vi",
    )
    .unwrap_err();
    assert_eq!(rejection.status, "rejected");
    assert_eq!(rejection.error, "character_not_found");
    assert_eq!(
        rejection.candidates,
        Some(vec!["char_protagonist".to_string()])
    );

    // 6. Nonexistent array path returns rejection
    let invalid_path =
        get_array_element_fields(&data, "invalid.path", None, Some(3), "vi").unwrap_err();
    assert_eq!(invalid_path.status, "rejected");
    assert!(invalid_path
        .error
        .contains("unsupported_or_unknown_array_path"));
}

/// `phase3_sprint4_deep1-F1`: Unknown catalog code yields structured candidates, no mutation.
#[test]
fn test_phase3_sprint4_deep1_F1_unknown_catalog_code_returns_candidates_no_mutation() {
    let catalog = MockCatalogRepo::new_seeded();

    // 1. Querying an unknown genre code "isekai" returns rejected with candidates
    let res = resolve_catalog_genre(&catalog, "isekai").unwrap();
    let rejection = res.unwrap_err();

    assert_eq!(rejection.status, "rejected");
    assert_eq!(rejection.error, "unknown_genre");
    assert_eq!(
        rejection.input,
        Some(serde_json::Value::String("isekai".to_string()))
    );

    let candidates = rejection.candidates.expect("Must provide candidates");
    assert!(candidates.contains(&"fantasy".to_string()));
    assert!(candidates.contains(&"scifi".to_string()));
    assert!(candidates.contains(&"mystery".to_string()));

    // 2. Search genres with non-matching query also returns candidates without error
    let search_res = search_genres(&catalog, Some("nonexistent_genre"), Some(5)).unwrap();
    assert_eq!(search_res.status, "rejected");
    assert_eq!(search_res.error, Some("unknown_genre".to_string()));
    assert!(search_res.items.is_empty());
    assert!(search_res.candidates.is_some());

    // 3. Search motifs with non-matching query returns candidates
    let motif_res = search_motifs(&catalog, None, Some("nonexistent_motif"), Some(5)).unwrap();
    assert_eq!(motif_res.status, "rejected");
    assert_eq!(motif_res.error, Some("unknown_motif".to_string()));
    assert!(motif_res.candidates.is_some());

    // 4. Verify catalog data was not modified
    let all_genres = catalog.list_genres().unwrap();
    assert_eq!(all_genres.len(), 3);
}

/// `phase3_sprint4_deep1-F2`: Missing translator locale triggers documented `en` fallback, no provider call.
#[test]
fn test_phase3_sprint4_deep1_F2_missing_translator_locale_triggers_en_fallback() {
    let data = BusinessFlowData::new();

    let missing_locales = ["fr", "de", "es", "ja", "", "unknown_locale_xyz"];

    for locale in missing_locales {
        let questions = get_field_null(&data, Some(3), locale);
        assert_eq!(questions.len(), 3);
        assert_eq!(questions[0].field, "require.project.display_name");
        // English fallback question
        assert_eq!(
            questions[0].question, "What is the title of your story?",
            "Locale '{locale}' must fallback to documented English question"
        );
        assert_eq!(
            questions[1].question, "What is the summary of the story?",
            "Locale '{locale}' must fallback to documented English question"
        );
        assert_eq!(
            questions[2].question, "What language is the story written in?",
            "Locale '{locale}' must fallback to documented English question"
        );
    }
}

/// `phase3_sprint4_deep1-F0`: Predecessor gate absent or baseline drift returns `CONTRACT_BLOCKED`.
#[test]
fn test_phase3_sprint4_deep1_F0_predecessor_gate_or_baseline_drift_blocks_dispatch() {
    // 1. Missing predecessor gate phase3_sprint1_deep0 -> blocked
    let err_p3s1 =
        verify_p3s4_dispatch_preconditions(BASELINE_COMMIT, BASELINE_COMMIT, false, true);
    assert!(err_p3s1.is_err());

    // 2. Missing predecessor gate phase2_sprint6_deep2 -> blocked
    let err_p2s6 =
        verify_p3s4_dispatch_preconditions(BASELINE_COMMIT, BASELINE_COMMIT, true, false);
    assert!(err_p2s6.is_err());

    // 3. Baseline drift -> blocked
    let err_drift = verify_p3s4_dispatch_preconditions(
        BASELINE_COMMIT,
        "0000000000000000000000000000000000000000",
        true,
        true,
    );
    assert!(err_drift.is_err());

    // 4. Live repository validation:
    //    - BASELINE_COMMIT is ancestor of HEAD
    //    - PREDECESSOR_P3S1_COMMIT exists as commit object
    //    - PREDECESSOR_P2S6_COMMIT exists as commit object
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf();

    let baseline_check = Command::new("git")
        .args(["merge-base", "--is-ancestor", BASELINE_COMMIT, "HEAD"])
        .current_dir(&repo_root)
        .status()
        .expect("git merge-base must execute");
    assert!(
        baseline_check.success(),
        "HEAD must descend from BASELINE_COMMIT {BASELINE_COMMIT}"
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
        "PREDECESSOR_P3S1_COMMIT {PREDECESSOR_P3S1_COMMIT} must exist"
    );

    let p2s6_check = Command::new("git")
        .args([
            "cat-file",
            "-e",
            &format!("{PREDECESSOR_P2S6_COMMIT}^{{commit}}"),
        ])
        .current_dir(&repo_root)
        .status()
        .expect("git cat-file must execute");
    assert!(
        p2s6_check.success(),
        "PREDECESSOR_P2S6_COMMIT {PREDECESSOR_P2S6_COMMIT} must exist"
    );
}

/// `REQ-TOOLS-003`: Read tools never mutate state or create draft versions.
#[test]
fn test_REQ_TOOLS_003_read_tools_never_create_draft_versions_or_mutate_state() {
    let mut data = BusinessFlowData::new();
    data.require.project.display_name = TrackedField::user("Cổ Chân Nhân".to_string());
    let initial_hash = state_data_hash(&data).unwrap();

    let catalog = MockCatalogRepo::new_seeded();

    // 1. Invoke get_field_null
    let _ = get_field_null(&data, Some(3), "vi");

    // 2. Invoke get_array_element_fields
    let _ = get_array_element_fields(&data, "require.character", None, Some(3), "vi");

    // 3. Invoke get_state
    let state_out = get_state(&data, None, None);
    assert_eq!(state_out.status, "applied");

    // 4. Invoke catalog search tools
    let _ = search_genres(&catalog, Some("fantasy"), Some(5)).unwrap();
    let _ = search_motifs(&catalog, None, Some("hero"), Some(5)).unwrap();
    let _ = search_world_settings(&catalog, None, None, Some(5)).unwrap();
    let _ = search_lore(&catalog, None, None, Some(5)).unwrap();

    // 5. Verify data is 100% bit-for-bit identical (zero mutation)
    let final_hash = state_data_hash(&data).unwrap();
    assert_eq!(
        initial_hash, final_hash,
        "Read tools must never mutate BusinessFlowData"
    );
}
