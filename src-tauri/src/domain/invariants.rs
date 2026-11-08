//! Pre-Commit Readiness Gate & Cross-Entity Invariant Validation
//! (`REQ-DOM-005`, `core.md` §2.8, §3.1.4, §3.5, §3.7; `be-core.md` §4.6.1, §7.1; `T-UNIT-04`, `phase3_sprint1_deep0-F2`).
//!
//! This module is 100% pure Rust with zero dependency on `rusqlite`, `tokio::fs`, `reqwest`, or `tauri`.
//!
//! Key responsibilities:
//! 1. **`check_readiness_gate(&BusinessFlowData)`:**
//!    - Verifies that all 10 Phase 0 (`require`) Master Attributes are complete (`Confirmed` or `AiSuggested`),
//!      that `require.character[]` has at least 1 character where `display_name IS NOT NULL OR nickname IS NOT NULL`,
//!      and that `require.project.target.{arc_count, chapter_count, word_per_chapter_count}` are strictly positive.
//! 2. **`validate_scene_config(&BusinessFlowData)`:**
//!    - Validates Phase 4 scene configuration invariants including mandatory `world_setting_code` referencing
//!      an existing `world_settings[].code`, `characters[].character_code` referencing an existing character,
//!      `lore_entries[].lore_entry_code` referencing an existing lore entry, and POV count rules against
//!      `narrator_config_code` (`core.md` §2.8, §3.1.4, `be-core.md` §4.6.1, §7.1).
//! 3. **`validate_draft_for_commit(&BusinessFlowData, has_pending_patches: bool) -> ValidationReport`:**
//!    - Enforces all cross-entity commit invariants from `core.md` §2.8 and `be-core.md` §7.1:
//!      - `PendingPatchesExist`: blocks commit if `has_pending_patches == true`.
//!      - `RequirePhaseIncomplete`: blocks commit if any of the 10 Phase 0 attributes are `Incomplete`.
//!      - `CharacterMissingNameAndNickname`: every character must have `display_name` or `nickname`.
//!      - `ArcCountMismatch`: `overview_phase.arcs.len()` must equal `require.project.target.arc_count` (`> 0`).
//!      - `ChapterCountMismatch`: total chapters across `chapter_phase.arcs` must equal `require.project.target.chapter_count` (`> 0`).
//!      - `DuplicateChapterPosition` / `InvalidChapterPosition`: `chapters[].position` must be positive and globally unique across all Arcs.
//!      - `EventBlockOutOfArcRange`: `arc.motif.event_blocks[]` must reference chapters within the same Arc with `start <= end`.
//!      - `MissingWorldSettingCode` / `DanglingWorldSettingCode` (`P3S1-C`, `phase3_sprint1_deep0-F2`): every scene must have a non-empty
//!        `world_setting_code` resolving to an existing node in `world_building_phase.world_settings[]`
//!        (kept strictly outside the 46 progress attributes denominator!).
//!      - `WorldSettingParentCycle` / `DanglingWorldSettingParentCode`: `world_settings[].parent_code` must exist and form a DAG.
//!      - `InvalidScenePovCount`: POV character count (`is_pov == true`) must match `narrator_config_code`
//!        (limited/first/second person -> `1`; omniscient/objective -> `0`; multiple_pov -> `>= 2`).
//!      - `CompetenceMissingSceneCode` / `DanglingSceneCode`: `character_competences[].scene_code` is mandatory (`NOT NULL`)
//!        and must reference an existing scene; `character_memories[].source_scene_code` (if present) and
//!        `relationship[].character.scenes[].scene_code` must reference an existing scene.
//!      - `DanglingArcCode` / `DanglingChapterCode` / `DanglingCharacterCode` / `DanglingLoreEntryCode` / `DanglingThemeParentCode`.

use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

use super::business_flow::BusinessFlowData;
use super::progress_engine::{evaluate_progress, AttributeProgressState, MasterAttributePhaseGroup};

/// Structured error code for a pre-commit or phase validation violation (`be-core.md` §7.1, `T-UNIT-04`, `phase3_sprint1_deep0-F2`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationErrorCode {
    /// Unresolved `pending_patches` exist for the draft (`be-core.md` §7.1 rule 2).
    PendingPatchesExist,
    /// Structural error or duplicate entity `code` in `BusinessFlowData`.
    InvalidDraftStructure,
    /// One or more of the 10 Phase 0 (`require`) Master Attributes are incomplete (`be-core.md` §7.1 rule 3).
    RequirePhaseIncomplete,
    /// A character has both `display_name` and `nickname` unset/empty (`core.md` §2.8, `docs/db.dbml:679`).
    CharacterMissingNameAndNickname,
    /// Number of `arcs[]` does not match `require.project.target.arc_count` (`core.md` §2.8, `be-core.md` §7.1).
    ArcCountMismatch,
    /// Total number of `chapters[]` across all Arcs does not match `require.project.target.chapter_count` (`core.md` §2.8, `T-UNIT-04`).
    ChapterCountMismatch,
    /// A chapter has a missing or `<= 0` `position` (`be-core.md` §7.1).
    InvalidChapterPosition,
    /// Duplicate `chapters[].position` across Arcs in the project (`core.md` §2.8, `T-UNIT-04`).
    DuplicateChapterPosition,
    /// An `arc.motif.event_blocks[]` entry lies outside the Arc's chapter range or has `start > end` (`core.md` §2.8, `be-core.md` §7.1).
    EventBlockOutOfArcRange,
    /// A scene is missing `world_setting_code` (`scenes.world_setting_id NOT NULL`, `P3S1-C`, `phase3_sprint1_deep0-F2`, `T-UNIT-04`).
    MissingWorldSettingCode,
    /// A scene's `world_setting_code` references a non-existent `world_settings[].code` (`P3S1-C`, `phase3_sprint1_deep0-F2`, `T-UNIT-04`).
    DanglingWorldSettingCode,
    /// A `world_settings[].parent_code` references a non-existent `world_settings[].code` (`be-core.md` §7.1).
    DanglingWorldSettingParentCode,
    /// A cycle exists in `world_settings[].parent_code` (`be-core.md` §7.1).
    WorldSettingParentCycle,
    /// Scene POV count (`characters[].is_pov == true`) violates `narrator_config_code` rules (`core.md` §2.8, `T-UNIT-04`).
    InvalidScenePovCount,
    /// A `character_competences[]` item is missing mandatory `scene_code` or `domain_code` (`core.md` §2.6, §2.8, `be-core.md` §7.1).
    CompetenceMissingSceneCode,
    /// A `character_competences[]` item is missing `domain_code` (`be-core.md` §7.1).
    CompetenceMissingDomainCode,
    /// A reference to `arc_code` does not exist in `overview_phase.arcs[]`.
    DanglingArcCode,
    /// A reference to `chapter_code` does not exist in `chapter_phase.arcs[].chapters[]`.
    DanglingChapterCode,
    /// A reference to `scene_code` does not exist in `chapter_phase.arcs[].chapters[].scenes[]`.
    DanglingSceneCode,
    /// A reference to `character_code` does not exist in `require.character[]`.
    DanglingCharacterCode,
    /// A reference to `lore_entry_code` does not exist in `world_building_phase.world_settings[].lore_entries[]`.
    DanglingLoreEntryCode,
    /// A `themes.parent_theme_code` references a non-existent theme or forms a cycle.
    DanglingOrCyclicThemeParentCode,
}

impl ValidationErrorCode {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PendingPatchesExist => "pending_patches_exist",
            Self::InvalidDraftStructure => "invalid_draft_structure",
            Self::RequirePhaseIncomplete => "require_phase_incomplete",
            Self::CharacterMissingNameAndNickname => "character_missing_name_and_nickname",
            Self::ArcCountMismatch => "arc_count_mismatch",
            Self::ChapterCountMismatch => "chapter_count_mismatch",
            Self::InvalidChapterPosition => "invalid_chapter_position",
            Self::DuplicateChapterPosition => "duplicate_chapter_position",
            Self::EventBlockOutOfArcRange => "event_block_out_of_arc_range",
            Self::MissingWorldSettingCode => "missing_world_setting_code",
            Self::DanglingWorldSettingCode => "dangling_world_setting_code",
            Self::DanglingWorldSettingParentCode => "dangling_world_setting_parent_code",
            Self::WorldSettingParentCycle => "world_setting_parent_cycle",
            Self::InvalidScenePovCount => "invalid_scene_pov_count",
            Self::CompetenceMissingSceneCode => "competence_missing_scene_code",
            Self::CompetenceMissingDomainCode => "competence_missing_domain_code",
            Self::DanglingArcCode => "dangling_arc_code",
            Self::DanglingChapterCode => "dangling_chapter_code",
            Self::DanglingSceneCode => "dangling_scene_code",
            Self::DanglingCharacterCode => "dangling_character_code",
            Self::DanglingLoreEntryCode => "dangling_lore_entry_code",
            Self::DanglingOrCyclicThemeParentCode => "dangling_or_cyclic_theme_parent_code",
        }
    }
}

/// A single cross-entity validation issue reported by `validate_draft_for_commit` or `check_readiness_gate`
/// (`be-core.md` §7.1, `T-UNIT-04`, `phase3_sprint1_deep0-F2`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationIssue {
    pub code: ValidationErrorCode,
    /// Classification tag when applicable (e.g. `"P3S1-C"` for commit gate violations).
    pub classification: String,
    /// Dot/bracket path or entity reference where the violation was detected.
    pub path: String,
    /// Optional offending entity `code`.
    pub entity_code: Option<String>,
    /// Human-readable diagnostic explanation.
    pub message: String,
}

/// Structured validation report returned by `validate_draft_for_commit` (`REQ-DOM-005`, `be-core.md` §7.1, `T-UNIT-04`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationReport {
    /// True iff `issues.is_empty()` and `has_pending_patches == false`.
    pub is_valid_for_commit: bool,
    /// True iff Phase 0 (`require`) passes the Readiness Gate (`is_require_complete`).
    pub is_require_ready: bool,
    /// Whether unresolved `pending_patches` exist for the draft.
    pub has_pending_patches: bool,
    /// Ordered list of all validation issues found.
    pub issues: Vec<ValidationIssue>,
}

impl ValidationReport {
    /// Returns `true` if the report contains at least one issue with `code`.
    pub fn has_error_code(&self, code: ValidationErrorCode) -> bool {
        self.issues.iter().any(|i| i.code == code)
    }

    /// Returns all issues matching `code`.
    pub fn issues_with_code(&self, code: ValidationErrorCode) -> Vec<&ValidationIssue> {
        self.issues.iter().filter(|i| i.code == code).collect()
    }

    /// Returns the list of distinct `ValidationErrorCode`s in the report.
    pub fn error_codes(&self) -> Vec<ValidationErrorCode> {
        self.issues.iter().map(|i| i.code).collect()
    }
}

/// Evaluates the Phase 0 (`require`) Readiness Gate (`core.md` §2.1, `be-core.md` §4.6.2, §7.1).
pub fn check_readiness_gate(state: &BusinessFlowData) -> Vec<ValidationIssue> {
    let mut issues = Vec::new();
    let progress = evaluate_progress(state);

    for attr in progress
        .attributes
        .iter()
        .filter(|a| a.phase_group == MasterAttributePhaseGroup::Phase0Require)
    {
        if attr.state == AttributeProgressState::Incomplete {
            issues.push(ValidationIssue {
                code: ValidationErrorCode::RequirePhaseIncomplete,
                classification: "P3S1-C".to_string(),
                path: attr.path.to_string(),
                entity_code: None,
                message: format!(
                    "Phase 0 required attribute '{}' is incomplete",
                    attr.path
                ),
            });
        }
    }

    for (idx, ch) in state.require.character.iter().enumerate() {
        let has_display = ch
            .display_name
            .value
            .as_ref()
            .map_or(false, |s| !s.trim().is_empty());
        let has_nick = ch
            .nickname
            .value
            .as_ref()
            .map_or(false, |s| !s.trim().is_empty());
        if !has_display && !has_nick {
            issues.push(ValidationIssue {
                code: ValidationErrorCode::CharacterMissingNameAndNickname,
                classification: "P3S1-C".to_string(),
                path: format!("require.character[{idx}]"),
                entity_code: Some(ch.code.clone()),
                message: format!(
                    "Character '{}' must have at least one of display_name or nickname",
                    ch.code
                ),
            });
        }
    }

    issues
}

/// Validates Phase 4 scene configurations and `world_setting_code` requirements across both
/// `chapter_phase` and `scene_config_phase` (`core.md` §2.8, §3.1.4; `be-core.md` §4.6.1, §7.1; `phase3_sprint1_deep0-F2`).
pub fn validate_scene_config(state: &BusinessFlowData) -> Vec<ValidationIssue> {
    let mut issues = Vec::new();

    let known_world_settings: BTreeSet<&str> = state
        .optional
        .world_building_phase
        .world_settings
        .iter()
        .map(|ws| ws.code.trim())
        .filter(|c| !c.is_empty())
        .collect();

    let known_lore_entries: BTreeSet<&str> = state
        .optional
        .world_building_phase
        .world_settings
        .iter()
        .flat_map(|ws| ws.lore_entries.iter())
        .map(|le| le.code.trim())
        .filter(|c| !c.is_empty())
        .collect();

    let known_characters: BTreeSet<&str> = state
        .require
        .character
        .iter()
        .map(|c| c.code.trim())
        .filter(|c| !c.is_empty())
        .collect();

    // Index Phase 4 scene configs by scene code so we can merge/resolve world_setting_code from either Phase 2 or Phase 4
    let mut phase4_scenes = BTreeMap::new();
    for sc_arc in &state.optional.scene_config_phase.arcs {
        for sc_ch in &sc_arc.chapters {
            for sc in &sc_ch.scenes {
                phase4_scenes.insert(sc.code.trim(), sc);
            }
        }
    }

    let mut checked_scene_codes = BTreeSet::new();

    // 1. Validate every scene in chapter_phase (merged with scene_config_phase if present)
    for (arc_idx, ch_arc) in state.optional.chapter_phase.arcs.iter().enumerate() {
        for (ch_idx, ch) in ch_arc.chapters.iter().enumerate() {
            for (sc_idx, sc) in ch.scenes.iter().enumerate() {
                let sc_code = sc.code.trim();
                checked_scene_codes.insert(sc_code);
                let p4_sc = phase4_scenes.get(sc_code).copied();

                let ws_code_opt = p4_sc
                    .and_then(|p4| p4.world_setting_code.value.as_deref())
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
                    .or_else(|| {
                        sc.world_setting_code
                            .value
                            .as_deref()
                            .map(str::trim)
                            .filter(|s| !s.is_empty())
                    });

                let path = format!(
                    "optional.chapter_phase.arcs[{arc_idx}].chapters[{ch_idx}].scenes[{sc_idx}]"
                );
                validate_single_scene_world_setting(
                    sc_code,
                    ws_code_opt,
                    &path,
                    &known_world_settings,
                    &mut issues,
                );
            }
        }
    }

    // 2. Validate Phase 4 scene_config_phase scenes (including world_setting_code if not already checked, POV rules, lore & character FKs)
    for (arc_idx, sc_arc) in state.optional.scene_config_phase.arcs.iter().enumerate() {
        for (ch_idx, sc_ch) in sc_arc.chapters.iter().enumerate() {
            for (sc_idx, sc) in sc_ch.scenes.iter().enumerate() {
                let sc_code = sc.code.trim();
                let base_path = format!(
                    "optional.scene_config_phase.arcs[{arc_idx}].chapters[{ch_idx}].scenes[{sc_idx}]"
                );

                if !checked_scene_codes.contains(sc_code) {
                    let ws_code_opt = sc
                        .world_setting_code
                        .value
                        .as_deref()
                        .map(str::trim)
                        .filter(|s| !s.is_empty());
                    validate_single_scene_world_setting(
                        sc_code,
                        ws_code_opt,
                        &base_path,
                        &known_world_settings,
                        &mut issues,
                    );
                }

                // Check lore_entry_code references
                for (le_idx, link) in sc.lore_entries.iter().enumerate() {
                    let le_code = link.lore_entry_code.trim();
                    if le_code.is_empty() || !known_lore_entries.contains(le_code) {
                        issues.push(ValidationIssue {
                            code: ValidationErrorCode::DanglingLoreEntryCode,
                            classification: "P3S1-C".to_string(),
                            path: format!("{base_path}.lore_entries[{le_idx}].lore_entry_code"),
                            entity_code: Some(sc.code.clone()),
                            message: format!(
                                "Scene '{}' references unknown lore_entry_code '{}'",
                                sc.code, link.lore_entry_code
                            ),
                        });
                    }
                }

                // Check character_code references and count POV characters
                let mut pov_true_count = 0usize;
                for (char_idx, ch_link) in sc.characters.iter().enumerate() {
                    let c_code = ch_link.character_code.trim();
                    if c_code.is_empty() || !known_characters.contains(c_code) {
                        issues.push(ValidationIssue {
                            code: ValidationErrorCode::DanglingCharacterCode,
                            classification: "P3S1-C".to_string(),
                            path: format!("{base_path}.characters[{char_idx}].character_code"),
                            entity_code: Some(sc.code.clone()),
                            message: format!(
                                "Scene '{}' references unknown character_code '{}'",
                                sc.code, ch_link.character_code
                            ),
                        });
                    }
                    if ch_link.is_pov.value == Some(true) {
                        pov_true_count += 1;
                    }
                }

                // Validate POV count against narrator_config_code (`core.md` §2.8, `be-core.md` §7.1, `T-UNIT-04`)
                if let Some(raw_nc) = sc.narrator_config_code.value.as_deref() {
                    let nc = raw_nc.trim().to_ascii_uppercase();
                    if !nc.is_empty() {
                        let is_omniscient_or_objective =
                            nc.contains("OMNISCIENT") || nc.contains("OBJECTIVE");
                        let is_multiple_pov = nc.contains("MULTIPLE") || nc.contains("MULTI_POV");
                        let is_limited = nc.contains("FIRST_PERSON")
                            || nc.contains("SECOND_PERSON")
                            || nc.contains("LIMITED")
                            || nc.contains("THIRD_PERSON_LIMITED");

                        if is_omniscient_or_objective && pov_true_count != 0 {
                            issues.push(ValidationIssue {
                                code: ValidationErrorCode::InvalidScenePovCount,
                                classification: "P3S1-C".to_string(),
                                path: format!("{base_path}.characters"),
                                entity_code: Some(sc.code.clone()),
                                message: format!(
                                    "Scene '{}' uses omniscient/objective narrator '{}' which requires 0 POV characters, found {}",
                                    sc.code, raw_nc, pov_true_count
                                ),
                            });
                        } else if is_multiple_pov && pov_true_count < 2 {
                            issues.push(ValidationIssue {
                                code: ValidationErrorCode::InvalidScenePovCount,
                                classification: "P3S1-C".to_string(),
                                path: format!("{base_path}.characters"),
                                entity_code: Some(sc.code.clone()),
                                message: format!(
                                    "Scene '{}' uses multiple-POV narrator '{}' which requires >= 2 POV characters, found {}",
                                    sc.code, raw_nc, pov_true_count
                                ),
                            });
                        } else if is_limited && !is_multiple_pov && pov_true_count != 1 {
                            issues.push(ValidationIssue {
                                code: ValidationErrorCode::InvalidScenePovCount,
                                classification: "P3S1-C".to_string(),
                                path: format!("{base_path}.characters"),
                                entity_code: Some(sc.code.clone()),
                                message: format!(
                                    "Scene '{}' uses limited narrator '{}' which requires exactly 1 POV character, found {}",
                                    sc.code, raw_nc, pov_true_count
                                ),
                            });
                        }
                    }
                }
            }
        }
    }

    issues
}

fn validate_single_scene_world_setting(
    scene_code: &str,
    ws_code_opt: Option<&str>,
    path: &str,
    known_world_settings: &BTreeSet<&str>,
    issues: &mut Vec<ValidationIssue>,
) {
    match ws_code_opt {
        None => {
            issues.push(ValidationIssue {
                code: ValidationErrorCode::MissingWorldSettingCode,
                classification: "P3S1-C".to_string(),
                path: format!("{path}.world_setting_code"),
                entity_code: Some(scene_code.to_string()),
                message: format!(
                    "Scene '{scene_code}' is missing mandatory world_setting_code before commit (P3S1-C)"
                ),
            });
        }
        Some(ws_code) if !known_world_settings.contains(ws_code) => {
            issues.push(ValidationIssue {
                code: ValidationErrorCode::DanglingWorldSettingCode,
                classification: "P3S1-C".to_string(),
                path: format!("{path}.world_setting_code"),
                entity_code: Some(scene_code.to_string()),
                message: format!(
                    "Scene '{scene_code}' references non-existent world_setting_code '{ws_code}' (P3S1-C)"
                ),
            });
        }
        Some(_) => {}
    }
}

/// Validates all cross-entity commit invariants on `state` (`REQ-DOM-005`, `core.md` §2.8, §3.1.4, §3.5, §3.7;
/// `be-core.md` §7.1; `T-UNIT-04`, `phase3_sprint1_deep0-F2`).
///
/// Note: Does not mutate `state` and keeps `world_setting_code` strictly as a commit validation rule
/// outside the 46 Master Attributes denominator (`phase3_sprint1_deep0-F2`).
pub fn validate_draft_for_commit(
    state: &BusinessFlowData,
    has_pending_patches: bool,
) -> ValidationReport {
    let mut issues = Vec::new();

    // 1. Pending patches block commit (`core.md` §3.5, §3.7, `be-core.md` §7.1 rule 2)
    if has_pending_patches {
        issues.push(ValidationIssue {
            code: ValidationErrorCode::PendingPatchesExist,
            classification: "P3S1-C".to_string(),
            path: "pending_patches".to_string(),
            entity_code: None,
            message: "Cannot commit draft while unresolved pending_patches exist".to_string(),
        });
    }

    // 2. Structural & unique entity code check
    if let Err(err) = state.validate_structure() {
        issues.push(ValidationIssue {
            code: ValidationErrorCode::InvalidDraftStructure,
            classification: "P3S1-C".to_string(),
            path: "meta".to_string(),
            entity_code: None,
            message: err.to_string(),
        });
    }

    // 3. Phase 0 Readiness Gate (`is_require_complete` & character display_name/nickname rule)
    let readiness_issues = check_readiness_gate(state);
    let is_require_ready = readiness_issues.is_empty();
    issues.extend(readiness_issues);

    // 4. Arc count vs `require.project.target.arc_count` (`core.md` §2.8, `be-core.md` §7.1)
    let expected_arc_count = state.require.project.target.arc_count.value.unwrap_or(0) as usize;
    let actual_overview_arcs = state.optional.overview_phase.arcs.len();
    if expected_arc_count == 0 || actual_overview_arcs != expected_arc_count {
        issues.push(ValidationIssue {
            code: ValidationErrorCode::ArcCountMismatch,
            classification: "P3S1-C".to_string(),
            path: "optional.overview_phase.arcs".to_string(),
            entity_code: None,
            message: format!(
                "Arc count mismatch: overview_phase.arcs has {actual_overview_arcs}, expected target.arc_count = {expected_arc_count}"
            ),
        });
    }

    // Build known entity maps
    let known_overview_arcs: BTreeSet<&str> = state
        .optional
        .overview_phase
        .arcs
        .iter()
        .map(|a| a.code.trim())
        .filter(|c| !c.is_empty())
        .collect();

    let known_characters: BTreeSet<&str> = state
        .require
        .character
        .iter()
        .map(|c| c.code.trim())
        .filter(|c| !c.is_empty())
        .collect();

    // 5. Chapter count vs `require.project.target.chapter_count` and global uniqueness of `chapters[].position` across Arcs
    let expected_chapter_count =
        state.require.project.target.chapter_count.value.unwrap_or(0) as usize;
    let mut total_chapters = 0usize;
    let mut seen_chapter_positions: BTreeMap<u32, String> = BTreeMap::new();
    let mut chapter_to_arc_and_pos: BTreeMap<&str, (&str, Option<u32>)> = BTreeMap::new();
    let mut arc_chapter_pos_bounds: BTreeMap<&str, (u32, u32)> = BTreeMap::new();
    let mut known_scenes: BTreeSet<&str> = BTreeSet::new();

    for (arc_idx, ch_arc) in state.optional.chapter_phase.arcs.iter().enumerate() {
        let arc_code = ch_arc.code.trim();
        if !known_overview_arcs.contains(arc_code) {
            issues.push(ValidationIssue {
                code: ValidationErrorCode::DanglingArcCode,
                classification: "P3S1-C".to_string(),
                path: format!("optional.chapter_phase.arcs[{arc_idx}].code"),
                entity_code: Some(ch_arc.code.clone()),
                message: format!(
                    "chapter_phase arc '{}' does not exist in overview_phase.arcs",
                    ch_arc.code
                ),
            });
        }

        total_chapters += ch_arc.chapters.len();

        for (ch_idx, ch) in ch_arc.chapters.iter().enumerate() {
            let ch_code = ch.code.trim();
            let pos_opt = ch.position.value;
            chapter_to_arc_and_pos.insert(ch_code, (arc_code, pos_opt));

            match pos_opt {
                Some(pos) if pos > 0 => {
                    if let Some(prev_ch_code) =
                        seen_chapter_positions.insert(pos, ch.code.clone())
                    {
                        issues.push(ValidationIssue {
                            code: ValidationErrorCode::DuplicateChapterPosition,
                            classification: "P3S1-C".to_string(),
                            path: format!(
                                "optional.chapter_phase.arcs[{arc_idx}].chapters[{ch_idx}].position"
                            ),
                            entity_code: Some(ch.code.clone()),
                            message: format!(
                                "Duplicate chapter position {pos} across Arcs: used by both '{prev_ch_code}' and '{}'",
                                ch.code
                            ),
                        });
                    }
                    arc_chapter_pos_bounds
                        .entry(arc_code)
                        .and_modify(|(min_p, max_p)| {
                            *min_p = (*min_p).min(pos);
                            *max_p = (*max_p).max(pos);
                        })
                        .or_insert((pos, pos));
                }
                _ => {
                    issues.push(ValidationIssue {
                        code: ValidationErrorCode::InvalidChapterPosition,
                        classification: "P3S1-C".to_string(),
                        path: format!(
                            "optional.chapter_phase.arcs[{arc_idx}].chapters[{ch_idx}].position"
                        ),
                        entity_code: Some(ch.code.clone()),
                        message: format!(
                            "Chapter '{}' must have a positive position (> 0)",
                            ch.code
                        ),
                    });
                }
            }

            for sc in &ch.scenes {
                let sc_code = sc.code.trim();
                if !sc_code.is_empty() {
                    known_scenes.insert(sc_code);
                }
            }
        }
    }

    if expected_chapter_count == 0 || total_chapters != expected_chapter_count {
        issues.push(ValidationIssue {
            code: ValidationErrorCode::ChapterCountMismatch,
            classification: "P3S1-C".to_string(),
            path: "optional.chapter_phase.arcs".to_string(),
            entity_code: None,
            message: format!(
                "Total chapter count mismatch: chapter_phase has {total_chapters} chapters, expected target.chapter_count = {expected_chapter_count}"
            ),
        });
    }

    // 6. Validate `arc.motif.event_blocks[]` chapter bounds & theme arguments/parent codes (`core.md` §2.8, `be-core.md` §7.1)
    let known_themes: BTreeMap<&str, Option<&str>> = state
        .optional
        .overview_phase
        .arcs
        .iter()
        .filter_map(|a| {
            let tc = a.themes.code.as_deref()?.trim();
            if tc.is_empty() {
                None
            } else {
                let ptc = a
                    .themes
                    .parent_theme_code
                    .value
                    .as_deref()
                    .map(str::trim)
                    .filter(|s| !s.is_empty());
                Some((tc, ptc))
            }
        })
        .collect();

    for (arc_idx, arc) in state.optional.overview_phase.arcs.iter().enumerate() {
        let arc_code = arc.code.trim();
        let bounds = arc_chapter_pos_bounds.get(arc_code).copied();

        for (eb_idx, eb) in arc.motif.event_blocks.iter().enumerate() {
            let eb_path = format!(
                "optional.overview_phase.arcs[{arc_idx}].motif.event_blocks[{eb_idx}]"
            );

            // Check by start_chapter_code / end_chapter_code if provided
            let start_code = eb
                .start_chapter_code
                .value
                .as_deref()
                .map(str::trim)
                .filter(|s| !s.is_empty());
            let end_code = eb
                .end_chapter_code
                .value
                .as_deref()
                .map(str::trim)
                .filter(|s| !s.is_empty());

            if start_code.is_some() || end_code.is_some() {
                match (start_code, end_code) {
                    (Some(sc), Some(ec)) => {
                        let s_info = chapter_to_arc_and_pos.get(sc).copied();
                        let e_info = chapter_to_arc_and_pos.get(ec).copied();
                        match (s_info, e_info) {
                            (Some((s_arc, Some(s_pos))), Some((e_arc, Some(e_pos)))) => {
                                if s_arc != arc_code || e_arc != arc_code || s_pos > e_pos {
                                    issues.push(ValidationIssue {
                                        code: ValidationErrorCode::EventBlockOutOfArcRange,
                                        classification: "P3S1-C".to_string(),
                                        path: eb_path.clone(),
                                        entity_code: Some(eb.code.clone()),
                                        message: format!(
                                            "Event block '{}' chapters ({sc}..{ec}) do not belong to arc '{arc_code}' or have start > end",
                                            eb.code
                                        ),
                                    });
                                }
                            }
                            _ => {
                                issues.push(ValidationIssue {
                                    code: ValidationErrorCode::EventBlockOutOfArcRange,
                                    classification: "P3S1-C".to_string(),
                                    path: eb_path.clone(),
                                    entity_code: Some(eb.code.clone()),
                                    message: format!(
                                        "Event block '{}' references unknown start/end chapter codes ({sc:?}, {ec:?})",
                                        eb.code
                                    ),
                                });
                            }
                        }
                    }
                    _ => {
                        issues.push(ValidationIssue {
                            code: ValidationErrorCode::EventBlockOutOfArcRange,
                            classification: "P3S1-C".to_string(),
                            path: eb_path.clone(),
                            entity_code: Some(eb.code.clone()),
                            message: format!(
                                "Event block '{}' must specify both start_chapter_code and end_chapter_code",
                                eb.code
                            ),
                        });
                    }
                }
            }

            // Also check numeric start_chapter / end_chapter if provided
            if let (Some(s_num), Some(e_num)) = (eb.start_chapter.value, eb.end_chapter.value) {
                let out_of_bounds = s_num > e_num
                    || bounds.map_or(false, |(min_p, max_p)| s_num < min_p || e_num > max_p);
                if out_of_bounds {
                    issues.push(ValidationIssue {
                        code: ValidationErrorCode::EventBlockOutOfArcRange,
                        classification: "P3S1-C".to_string(),
                        path: eb_path,
                        entity_code: Some(eb.code.clone()),
                        message: format!(
                            "Event block '{}' range {s_num}..={e_num} is outside arc '{arc_code}' bounds {bounds:?}",
                            eb.code
                        ),
                    });
                }
            }
        }

        // Theme parent check
        if let Some(parent_tc) = arc
            .themes
            .parent_theme_code
            .value
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty())
        {
            if !known_themes.contains_key(parent_tc) {
                issues.push(ValidationIssue {
                    code: ValidationErrorCode::DanglingOrCyclicThemeParentCode,
                    classification: "P3S1-C".to_string(),
                    path: format!(
                        "optional.overview_phase.arcs[{arc_idx}].themes.parent_theme_code"
                    ),
                    entity_code: arc.themes.code.clone(),
                    message: format!("Theme references non-existent parent_theme_code '{parent_tc}'"),
                });
            } else if let Some(self_tc) = arc.themes.code.as_deref().map(str::trim) {
                let mut curr = Some(parent_tc);
                let mut visited = BTreeSet::new();
                visited.insert(self_tc);
                while let Some(c) = curr {
                    if !visited.insert(c) {
                        issues.push(ValidationIssue {
                            code: ValidationErrorCode::DanglingOrCyclicThemeParentCode,
                            classification: "P3S1-C".to_string(),
                            path: format!(
                                "optional.overview_phase.arcs[{arc_idx}].themes.parent_theme_code"
                            ),
                            entity_code: arc.themes.code.clone(),
                            message: format!("Cycle detected in themes.parent_theme_code at '{c}'"),
                        });
                        break;
                    }
                    curr = known_themes.get(c).copied().flatten();
                }
            }
        }

        // Theme argument character_code check
        for (arg_idx, arg) in arc.themes.arguments.iter().enumerate() {
            if let Some(c_code) = arg
                .character_code
                .value
                .as_deref()
                .map(str::trim)
                .filter(|s| !s.is_empty())
            {
                if !known_characters.contains(c_code) {
                    issues.push(ValidationIssue {
                        code: ValidationErrorCode::DanglingCharacterCode,
                        classification: "P3S1-C".to_string(),
                        path: format!(
                            "optional.overview_phase.arcs[{arc_idx}].themes.arguments[{arg_idx}].character_code"
                        ),
                        entity_code: arg.code.clone(),
                        message: format!(
                            "Theme argument references unknown character_code '{c_code}'"
                        ),
                    });
                }
            }
        }
    }

    // 7. World settings tree parent existence & cycle detection (`be-core.md` §7.1)
    let ws_parent_map: BTreeMap<&str, Option<&str>> = state
        .optional
        .world_building_phase
        .world_settings
        .iter()
        .map(|ws| {
            let p = ws
                .parent_code
                .value
                .as_deref()
                .map(str::trim)
                .filter(|s| !s.is_empty());
            (ws.code.trim(), p)
        })
        .collect();

    for (ws_idx, ws) in state
        .optional
        .world_building_phase
        .world_settings
        .iter()
        .enumerate()
    {
        let ws_code = ws.code.trim();
        let mut curr = ws_parent_map.get(ws_code).copied().flatten();
        let mut visited = BTreeSet::new();
        visited.insert(ws_code);

        while let Some(parent_code) = curr {
            if !ws_parent_map.contains_key(parent_code) {
                issues.push(ValidationIssue {
                    code: ValidationErrorCode::DanglingWorldSettingParentCode,
                    classification: "P3S1-C".to_string(),
                    path: format!(
                        "optional.world_building_phase.world_settings[{ws_idx}].parent_code"
                    ),
                    entity_code: Some(ws.code.clone()),
                    message: format!(
                        "World setting '{}' references non-existent parent_code '{parent_code}'",
                        ws.code
                    ),
                });
                break;
            }
            if !visited.insert(parent_code) {
                issues.push(ValidationIssue {
                    code: ValidationErrorCode::WorldSettingParentCycle,
                    classification: "P3S1-C".to_string(),
                    path: format!(
                        "optional.world_building_phase.world_settings[{ws_idx}].parent_code"
                    ),
                    entity_code: Some(ws.code.clone()),
                    message: format!(
                        "Cycle detected in world_settings hierarchy involving '{}' and '{parent_code}'",
                        ws.code
                    ),
                });
                break;
            }
            curr = ws_parent_map.get(parent_code).copied().flatten();
        }
    }

    // 8. Validate Phase 4 scene_config arc/chapter/scene cross-references and scene config invariants
    for (arc_idx, sc_arc) in state.optional.scene_config_phase.arcs.iter().enumerate() {
        let arc_code = sc_arc.code.trim();
        if !known_overview_arcs.contains(arc_code) {
            issues.push(ValidationIssue {
                code: ValidationErrorCode::DanglingArcCode,
                classification: "P3S1-C".to_string(),
                path: format!("optional.scene_config_phase.arcs[{arc_idx}].code"),
                entity_code: Some(sc_arc.code.clone()),
                message: format!(
                    "scene_config_phase arc '{}' does not exist in overview_phase.arcs",
                    sc_arc.code
                ),
            });
        }
        for (ch_idx, sc_ch) in sc_arc.chapters.iter().enumerate() {
            let ch_code = sc_ch.code.trim();
            if !chapter_to_arc_and_pos.contains_key(ch_code) {
                issues.push(ValidationIssue {
                    code: ValidationErrorCode::DanglingChapterCode,
                    classification: "P3S1-C".to_string(),
                    path: format!(
                        "optional.scene_config_phase.arcs[{arc_idx}].chapters[{ch_idx}].code"
                    ),
                    entity_code: Some(sc_ch.code.clone()),
                    message: format!(
                        "scene_config_phase chapter '{}' does not exist in chapter_phase",
                        sc_ch.code
                    ),
                });
            }
            for (sc_idx, sc) in sc_ch.scenes.iter().enumerate() {
                let sc_code = sc.code.trim();
                if !known_scenes.is_empty() && !known_scenes.contains(sc_code) {
                    issues.push(ValidationIssue {
                        code: ValidationErrorCode::DanglingSceneCode,
                        classification: "P3S1-C".to_string(),
                        path: format!(
                            "optional.scene_config_phase.arcs[{arc_idx}].chapters[{ch_idx}].scenes[{sc_idx}].code"
                        ),
                        entity_code: Some(sc.code.clone()),
                        message: format!(
                            "scene_config_phase scene '{}' does not exist in chapter_phase",
                            sc.code
                        ),
                    });
                }
            }
        }
    }

    issues.extend(validate_scene_config(state));

    // 9. Validate Phase 5.1, 5.2, 5.3 character references, competence mandatory scene_code/domain_code, and memory nullable source_scene_code
    for (idx, item) in state
        .optional
        .character_appearance_phase
        .character
        .iter()
        .enumerate()
    {
        if !known_characters.contains(item.code.trim()) {
            issues.push(ValidationIssue {
                code: ValidationErrorCode::DanglingCharacterCode,
                classification: "P3S1-C".to_string(),
                path: format!("optional.character_appearance_phase.character[{idx}].code"),
                entity_code: Some(item.code.clone()),
                message: format!(
                    "character_appearance_phase references unknown character '{}'",
                    item.code
                ),
            });
        }
    }

    for (idx, item) in state
        .optional
        .character_psychological_phase
        .character
        .iter()
        .enumerate()
    {
        if !known_characters.contains(item.code.trim()) {
            issues.push(ValidationIssue {
                code: ValidationErrorCode::DanglingCharacterCode,
                classification: "P3S1-C".to_string(),
                path: format!("optional.character_psychological_phase.character[{idx}].code"),
                entity_code: Some(item.code.clone()),
                message: format!(
                    "character_psychological_phase references unknown character '{}'",
                    item.code
                ),
            });
        }
    }

    for (c_idx, item) in state
        .optional
        .character_life_experience_phase
        .character
        .iter()
        .enumerate()
    {
        if !known_characters.contains(item.code.trim()) {
            issues.push(ValidationIssue {
                code: ValidationErrorCode::DanglingCharacterCode,
                classification: "P3S1-C".to_string(),
                path: format!(
                    "optional.character_life_experience_phase.character[{c_idx}].code"
                ),
                entity_code: Some(item.code.clone()),
                message: format!(
                    "character_life_experience_phase references unknown character '{}'",
                    item.code
                ),
            });
        }

        // Memories: `source_scene_code` is nullable, but if present must reference a valid scene (`be-core.md` §7.1)
        for (m_idx, mem) in item.memories.iter().enumerate() {
            if let Some(sc_code) = mem
                .source_scene_code
                .value
                .as_deref()
                .map(str::trim)
                .filter(|s| !s.is_empty())
            {
                if !known_scenes.contains(sc_code) {
                    issues.push(ValidationIssue {
                        code: ValidationErrorCode::DanglingSceneCode,
                        classification: "P3S1-C".to_string(),
                        path: format!(
                            "optional.character_life_experience_phase.character[{c_idx}].memories[{m_idx}].source_scene_code"
                        ),
                        entity_code: mem.code.clone(),
                        message: format!(
                            "Character memory references non-existent source_scene_code '{sc_code}'"
                        ),
                    });
                }
            }
        }

        // Competences: `scene_code` is mandatory (`NOT NULL`) and must reference a valid scene; `domain_code` must be non-empty (`core.md` §2.6, §2.8, `be-core.md` §7.1)
        for (k_idx, comp) in item.competences.iter().enumerate() {
            let base_path = format!(
                "optional.character_life_experience_phase.character[{c_idx}].competences[{k_idx}]"
            );
            let dom_ok = comp
                .domain_code
                .value
                .as_deref()
                .map(str::trim)
                .map_or(false, |s| !s.is_empty());
            if !dom_ok {
                issues.push(ValidationIssue {
                    code: ValidationErrorCode::CompetenceMissingDomainCode,
                    classification: "P3S1-C".to_string(),
                    path: format!("{base_path}.domain_code"),
                    entity_code: Some(comp.code.clone()),
                    message: format!(
                        "Character competence '{}' is missing mandatory domain_code",
                        comp.code
                    ),
                });
            }

            match comp
                .scene_code
                .value
                .as_deref()
                .map(str::trim)
                .filter(|s| !s.is_empty())
            {
                None => {
                    issues.push(ValidationIssue {
                        code: ValidationErrorCode::CompetenceMissingSceneCode,
                        classification: "P3S1-C".to_string(),
                        path: format!("{base_path}.scene_code"),
                        entity_code: Some(comp.code.clone()),
                        message: format!(
                            "Character competence '{}' is missing mandatory scene_code (character_competences.scene_id NOT NULL)",
                            comp.code
                        ),
                    });
                }
                Some(sc_code) if !known_scenes.contains(sc_code) => {
                    issues.push(ValidationIssue {
                        code: ValidationErrorCode::DanglingSceneCode,
                        classification: "P3S1-C".to_string(),
                        path: format!("{base_path}.scene_code"),
                        entity_code: Some(comp.code.clone()),
                        message: format!(
                            "Character competence '{}' references non-existent scene_code '{sc_code}'",
                            comp.code
                        ),
                    });
                }
                Some(_) => {}
            }
        }
    }

    // 10. Validate Phase 6 relationships (`from_code`, `to_code`, and `scenes[].scene_code`)
    for (r_idx, rel) in state
        .optional
        .relationship_phase
        .relationship
        .iter()
        .enumerate()
    {
        let base_path = format!("optional.relationship_phase.relationship[{r_idx}]");
        let from_code = rel.character.from_code.trim();
        let to_code = rel.character.to_code.trim();

        if from_code.is_empty() || !known_characters.contains(from_code) {
            issues.push(ValidationIssue {
                code: ValidationErrorCode::DanglingCharacterCode,
                classification: "P3S1-C".to_string(),
                path: format!("{base_path}.character.from_code"),
                entity_code: Some(rel.code.clone()),
                message: format!(
                    "Relationship '{}' references unknown from_code '{}'",
                    rel.code, rel.character.from_code
                ),
            });
        }
        if to_code.is_empty() || !known_characters.contains(to_code) {
            issues.push(ValidationIssue {
                code: ValidationErrorCode::DanglingCharacterCode,
                classification: "P3S1-C".to_string(),
                path: format!("{base_path}.character.to_code"),
                entity_code: Some(rel.code.clone()),
                message: format!(
                    "Relationship '{}' references unknown to_code '{}'",
                    rel.code, rel.character.to_code
                ),
            });
        }

        for (sc_idx, sc_state) in rel.character.scenes.iter().enumerate() {
            let sc_code = sc_state.scene_code.trim();
            if sc_code.is_empty() || !known_scenes.contains(sc_code) {
                issues.push(ValidationIssue {
                    code: ValidationErrorCode::DanglingSceneCode,
                    classification: "P3S1-C".to_string(),
                    path: format!("{base_path}.character.scenes[{sc_idx}].scene_code"),
                    entity_code: Some(rel.code.clone()),
                    message: format!(
                        "Relationship '{}' state references unknown scene_code '{}'",
                        rel.code, sc_state.scene_code
                    ),
                });
            }
        }
    }

    ValidationReport {
        is_valid_for_commit: issues.is_empty(),
        is_require_ready,
        has_pending_patches,
        issues,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::business_flow::{
        ChapterPhaseArcItem, ChapterPhaseChapterItem, ChapterPhaseSceneItem, OverviewArcItem,
        RequireCharacterItem, RequireGenreItem, TrackedField, WorldSettingItem,
    };

    #[test]
    fn missing_world_setting_code_is_flagged_by_validate_draft_for_commit() {
        let mut data = BusinessFlowData::new();
        data.require.project.code = Some("p1".to_string());
        data.require.project.display_name = TrackedField::user("Dự Án 1".to_string());
        data.require.project.summary = TrackedField::user("Tóm tắt".to_string());
        data.require.project.language = TrackedField::user("VI".to_string());
        data.require.project.target.arc_count = TrackedField::user(1);
        data.require.project.target.chapter_count = TrackedField::user(1);
        data.require.project.target.word_per_chapter_count = TrackedField::user(2000);
        data.require.project.genres.push(RequireGenreItem {
            code: TrackedField::user("tien_hiep".to_string()),
            is_primary: TrackedField::user(true),
            display_name: TrackedField::user("Tiên Hiệp".to_string()),
        });
        data.require.character.push(RequireCharacterItem {
            code: "char_1".to_string(),
            display_name: TrackedField::user("Lâm Phong".to_string()),
            biological_sex: TrackedField::user("MALE".to_string()),
            motif_code: TrackedField::user("hero_journey".to_string()),
            ..Default::default()
        });
        data.optional.overview_phase.arcs.push(OverviewArcItem {
            code: "p1-a1".to_string(),
            display_name: TrackedField::user("Hồi 1".to_string()),
            position: TrackedField::user(1),
            ..Default::default()
        });
        data.optional.chapter_phase.arcs.push(ChapterPhaseArcItem {
            code: "p1-a1".to_string(),
            chapters: vec![ChapterPhaseChapterItem {
                code: "p1-a1-c1".to_string(),
                display_name: TrackedField::user("Chương 1".to_string()),
                position: TrackedField::user(1),
                scenes: vec![ChapterPhaseSceneItem {
                    code: "p1-a1-c1-s1".to_string(),
                    display_name: TrackedField::user("Cảnh 1".to_string()),
                    position: TrackedField::user(1),
                    summary: TrackedField::user("Mở màn".to_string()),
                    world_setting_code: TrackedField::unset(),
                    source: None,
                }],
                ..Default::default()
            }],
        });
        data.optional
            .world_building_phase
            .world_settings
            .push(WorldSettingItem {
                code: "p1-w1".to_string(),
                display_name: TrackedField::user("Thanh Vân Tông".to_string()),
                ..Default::default()
            });

        let report = validate_draft_for_commit(&data, false);
        assert!(!report.is_valid_for_commit);
        assert!(report.has_error_code(ValidationErrorCode::MissingWorldSettingCode));
    }
}
