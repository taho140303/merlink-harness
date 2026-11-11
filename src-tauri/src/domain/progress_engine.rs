//! 46 Master Attributes Progress Engine & Composite Provenance Evaluator
//! (`REQ-DOM-003`, `REQ-DOM-005`, `core.md` §3.1, `be-core.md` §4.6, `T-UNIT-01`, `P3S1-B`).
//!
//! Authoritative rules enforced (`APPROVED v2.4.4..v2.4.5`):
//! 1. **Fixed Denominator of 46 (`10 + 9 + 4 + 6 + 4 + 6 + 4 + 2 + 1 = 46`):**
//!    - Phase 0 (`require`): 10 attributes
//!    - Phase 1 (`overview_phase`): 9 attributes
//!    - Phase 2 (`chapter_phase`): 4 attributes
//!    - Phase 3 (`world_building_phase`): 6 attributes
//!    - Phase 4 (`scene_config_phase`): 4 attributes
//!    - Phase 5.1 (`character_appearance_phase`): 6 attributes
//!    - Phase 5.2 (`character_psychological_phase`): 4 attributes
//!    - Phase 5.3 (`character_life_experience_phase`): 2 attributes
//!    - Phase 6 (`relationship_phase`): 1 attribute
//! 2. **Explicit Exclusions (`50 - 4 = 46`):**
//!    - `optional.overview_phase.project.target.arc_tolerance`
//!    - `optional.overview_phase.project.target.chapter_tolerance`
//!    - `optional.overview_phase.project.target.word_per_chapter_tolerance`
//!    - `optional.relationship_phase.relationship.motif.event_blocks`
//!    - Furthermore, `scenes[].world_setting_code` is a mandatory pre-commit invariant (`domain::invariants`),
//!      NOT a 47th progress attribute (`core.md` §3.1.4, `phase3_sprint1_deep0-F2`).
//! 3. **Composite Provenance Precedence (`confirmed > ai_suggested > incomplete`):**
//!    - Every attribute contributes at most `1` to either `confirmed_count` or `ai_suggested_count`
//!      (never both, and `confirmed_count + ai_suggested_count + incomplete_count == 46`).
//!    - For an element in an array/composite attribute:
//!      - Missing/invalid required fields -> element is `Incomplete`.
//!      - All required fields of the element have `source` in `{User, UserEdited}` -> element is `Confirmed`.
//!      - All required fields are valid and at least one has `source == AiGenerated` -> element is `AiSuggested`.
//!    - Across an array $A = [e_1, \dots, e_n]$:
//!      - If any element is `Confirmed` -> attribute is `Confirmed`.
//!      - Else if any element is `AiSuggested` -> attribute is `AiSuggested`.
//!      - Else -> attribute is `Incomplete`.

use serde::{Deserialize, Serialize};

use super::business_flow::{
    ArcEventBlockItem, BusinessFlowData, CharacterCompetenceItem, CharacterMemoryItem,
    LoreEntryItem, ProvenanceSource, RelationshipSceneStateItem, RequireCharacterItem,
    RequireGenreItem, SceneCharacterLink, SceneLoreEntryLink, ThemeArgumentItem,
    ThemeResolutionSection, TrackedField,
};

/// Authoritative denominator of the Progress Engine (`46` Master Attributes).
pub const MASTER_ATTRIBUTE_DENOMINATOR: usize = 46;

/// The 4 fields explicitly excluded from the 50-item FE prototype list (`50 - 4 = 46`, `core.md` §3.1.2, `be-core.md` §4.6.1).
pub const EXCLUDED_PROTOTYPE_ATTRIBUTES: [&str; 4] = [
    "optional.overview_phase.project.target.arc_tolerance",
    "optional.overview_phase.project.target.chapter_tolerance",
    "optional.overview_phase.project.target.word_per_chapter_tolerance",
    "optional.relationship_phase.relationship.motif.event_blocks",
];

/// Mutually exclusive progress state of a single Master Attribute (`be-core.md` §4.6.2).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttributeProgressState {
    /// Contributes +1 to `confirmed_count` (`user` or `user_edited`).
    Confirmed,
    /// Contributes +1 to `ai_suggested_count` (`ai_generated`).
    AiSuggested,
    /// Contributes 0 (`incomplete`).
    Incomplete,
}

impl AttributeProgressState {
    pub const fn is_complete(self) -> bool {
        matches!(self, Self::Confirmed | Self::AiSuggested)
    }

    pub const fn is_confirmed(self) -> bool {
        matches!(self, Self::Confirmed)
    }

    pub const fn is_ai_suggested(self) -> bool {
        matches!(self, Self::AiSuggested)
    }
}

/// Phase group of a Master Attribute (`be-core.md` §4.6.1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum MasterAttributePhaseGroup {
    /// Phase 0 (`require`) — 10 attributes.
    Phase0Require,
    /// Phase 1 (`optional.overview_phase`) — 9 attributes.
    Phase1Overview,
    /// Phase 2 (`optional.chapter_phase`) — 4 attributes.
    Phase2Chapter,
    /// Phase 3 (`optional.world_building_phase`) — 6 attributes.
    Phase3WorldBuilding,
    /// Phase 4 (`optional.scene_config_phase`) — 4 attributes.
    Phase4SceneConfig,
    /// Sub-phase 5.1 (`optional.character_appearance_phase`) — 6 attributes.
    Phase5Sub1Appearance,
    /// Sub-phase 5.2 (`optional.character_psychological_phase`) — 4 attributes.
    Phase5Sub2Psychological,
    /// Sub-phase 5.3 (`optional.character_life_experience_phase`) — 2 attributes.
    Phase5Sub3LifeExperience,
    /// Phase 6 (`optional.relationship_phase`) — 1 attribute.
    Phase6Relationship,
}

impl MasterAttributePhaseGroup {
    pub const fn expected_count(self) -> usize {
        match self {
            Self::Phase0Require => 10,
            Self::Phase1Overview => 9,
            Self::Phase2Chapter => 4,
            Self::Phase3WorldBuilding => 6,
            Self::Phase4SceneConfig => 4,
            Self::Phase5Sub1Appearance => 6,
            Self::Phase5Sub2Psychological => 4,
            Self::Phase5Sub3LifeExperience => 2,
            Self::Phase6Relationship => 1,
        }
    }
}

/// Canonical identifier for each of the 46 Master Attributes (`10 + 9 + 4 + 6 + 4 + 6 + 4 + 2 + 1 = 46`, `be-core.md` §4.6.1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum MasterAttributeId {
    // --- Phase 0 (`require`): 10 ---
    RequireProjectDisplayName,
    RequireProjectSummary,
    RequireProjectLanguage,
    RequireProjectTargetArcCount,
    RequireProjectTargetChapterCount,
    RequireProjectTargetWordPerChapterCount,
    RequireProjectGenresCode,
    RequireCharacterDisplayNameOrNickname,
    RequireCharacterBiologicalSex,
    RequireCharacterMotifCode,

    // --- Phase 1 (`overview_phase`): 9 ---
    OverviewArcsDisplayName,
    OverviewArcsSummary,
    OverviewArcsPosition,
    OverviewArcsMotifCode,
    OverviewArcsMotifEventBlocks,
    OverviewArcsThemesQuestion,
    OverviewArcsThemesResolution,
    OverviewArcsThemesMessage,
    OverviewArcsThemesArguments,

    // --- Phase 2 (`chapter_phase`): 4 ---
    ChapterPhaseChaptersDisplayName,
    ChapterPhaseChaptersSummary,
    ChapterPhaseScenesDisplayName,
    ChapterPhaseScenesSummary,

    // --- Phase 3 (`world_building_phase`): 6 ---
    WorldSettingsDisplayName,
    WorldSettingsType,
    WorldSettingsDescription,
    WorldSettingsAccess,
    WorldSettingsLoreEntriesDisplayName,
    WorldSettingsLoreEntriesDescription,

    // --- Phase 4 (`scene_config_phase`): 4 ---
    SceneConfigStylePresetCode,
    SceneConfigNarratorConfigCode,
    SceneConfigLoreEntries,
    SceneConfigCharactersIsPov,

    // --- Phase 5.1 (`character_appearance_phase`): 6 ---
    CharacterAppearanceSpecies,
    CharacterAppearanceGenderIdentity,
    CharacterAppearanceHeightCm,
    CharacterAppearanceHairColor,
    CharacterAppearanceSkinTone,
    CharacterAppearanceOutfitStyle,

    // --- Phase 5.2 (`character_psychological_phase`): 4 ---
    CharacterPsychologicalTheWound,
    CharacterPsychologicalMoralCode,
    CharacterPsychologicalExternalWant,
    CharacterPsychologicalSocialMask,

    // --- Phase 5.3 (`character_life_experience_phase`): 2 ---
    CharacterLifeExperienceMemories,
    CharacterLifeExperienceCompetences,

    // --- Phase 6 (`relationship_phase`): 1 ---
    RelationshipCharacterScenes,
}

/// Ordered list of all 46 Master Attributes (`MASTER_ATTRIBUTES_LIST`, `core.md` §3.1, `be-core.md` §4.6.1).
pub const MASTER_ATTRIBUTES_LIST: [MasterAttributeId; MASTER_ATTRIBUTE_DENOMINATOR] = [
    // Phase 0 (10)
    MasterAttributeId::RequireProjectDisplayName,
    MasterAttributeId::RequireProjectSummary,
    MasterAttributeId::RequireProjectLanguage,
    MasterAttributeId::RequireProjectTargetArcCount,
    MasterAttributeId::RequireProjectTargetChapterCount,
    MasterAttributeId::RequireProjectTargetWordPerChapterCount,
    MasterAttributeId::RequireProjectGenresCode,
    MasterAttributeId::RequireCharacterDisplayNameOrNickname,
    MasterAttributeId::RequireCharacterBiologicalSex,
    MasterAttributeId::RequireCharacterMotifCode,
    // Phase 1 (9)
    MasterAttributeId::OverviewArcsDisplayName,
    MasterAttributeId::OverviewArcsSummary,
    MasterAttributeId::OverviewArcsPosition,
    MasterAttributeId::OverviewArcsMotifCode,
    MasterAttributeId::OverviewArcsMotifEventBlocks,
    MasterAttributeId::OverviewArcsThemesQuestion,
    MasterAttributeId::OverviewArcsThemesResolution,
    MasterAttributeId::OverviewArcsThemesMessage,
    MasterAttributeId::OverviewArcsThemesArguments,
    // Phase 2 (4)
    MasterAttributeId::ChapterPhaseChaptersDisplayName,
    MasterAttributeId::ChapterPhaseChaptersSummary,
    MasterAttributeId::ChapterPhaseScenesDisplayName,
    MasterAttributeId::ChapterPhaseScenesSummary,
    // Phase 3 (6)
    MasterAttributeId::WorldSettingsDisplayName,
    MasterAttributeId::WorldSettingsType,
    MasterAttributeId::WorldSettingsDescription,
    MasterAttributeId::WorldSettingsAccess,
    MasterAttributeId::WorldSettingsLoreEntriesDisplayName,
    MasterAttributeId::WorldSettingsLoreEntriesDescription,
    // Phase 4 (4)
    MasterAttributeId::SceneConfigStylePresetCode,
    MasterAttributeId::SceneConfigNarratorConfigCode,
    MasterAttributeId::SceneConfigLoreEntries,
    MasterAttributeId::SceneConfigCharactersIsPov,
    // Phase 5.1 (6)
    MasterAttributeId::CharacterAppearanceSpecies,
    MasterAttributeId::CharacterAppearanceGenderIdentity,
    MasterAttributeId::CharacterAppearanceHeightCm,
    MasterAttributeId::CharacterAppearanceHairColor,
    MasterAttributeId::CharacterAppearanceSkinTone,
    MasterAttributeId::CharacterAppearanceOutfitStyle,
    // Phase 5.2 (4)
    MasterAttributeId::CharacterPsychologicalTheWound,
    MasterAttributeId::CharacterPsychologicalMoralCode,
    MasterAttributeId::CharacterPsychologicalExternalWant,
    MasterAttributeId::CharacterPsychologicalSocialMask,
    // Phase 5.3 (2)
    MasterAttributeId::CharacterLifeExperienceMemories,
    MasterAttributeId::CharacterLifeExperienceCompetences,
    // Phase 6 (1)
    MasterAttributeId::RelationshipCharacterScenes,
];

impl MasterAttributeId {
    /// Returns the canonical dot-separated attribute path from `be-core.md` §4.6.1.
    pub const fn path(self) -> &'static str {
        match self {
            // Phase 0 (10)
            Self::RequireProjectDisplayName => "require.project.display_name",
            Self::RequireProjectSummary => "require.project.summary",
            Self::RequireProjectLanguage => "require.project.language",
            Self::RequireProjectTargetArcCount => "require.project.target.arc_count",
            Self::RequireProjectTargetChapterCount => "require.project.target.chapter_count",
            Self::RequireProjectTargetWordPerChapterCount => {
                "require.project.target.word_per_chapter_count"
            }
            Self::RequireProjectGenresCode => "require.project.genres.code",
            Self::RequireCharacterDisplayNameOrNickname => "require.character.display_name",
            Self::RequireCharacterBiologicalSex => "require.character.biological_sex",
            Self::RequireCharacterMotifCode => "require.character.motif_code",

            // Phase 1 (9)
            Self::OverviewArcsDisplayName => "optional.overview_phase.arcs.display_name",
            Self::OverviewArcsSummary => "optional.overview_phase.arcs.summary",
            Self::OverviewArcsPosition => "optional.overview_phase.arcs.position",
            Self::OverviewArcsMotifCode => "optional.overview_phase.arcs.motif.code",
            Self::OverviewArcsMotifEventBlocks => {
                "optional.overview_phase.arcs.motif.event_blocks"
            }
            Self::OverviewArcsThemesQuestion => "optional.overview_phase.arcs.themes.question",
            Self::OverviewArcsThemesResolution => "optional.overview_phase.arcs.themes.resolution",
            Self::OverviewArcsThemesMessage => "optional.overview_phase.arcs.themes.message",
            Self::OverviewArcsThemesArguments => "optional.overview_phase.arcs.themes.arguments",

            // Phase 2 (4)
            Self::ChapterPhaseChaptersDisplayName => {
                "optional.chapter_phase.arcs.chapters.display_name"
            }
            Self::ChapterPhaseChaptersSummary => "optional.chapter_phase.arcs.chapters.summary",
            Self::ChapterPhaseScenesDisplayName => {
                "optional.chapter_phase.arcs.chapters.scenes.display_name"
            }
            Self::ChapterPhaseScenesSummary => {
                "optional.chapter_phase.arcs.chapters.scenes.summary"
            }

            // Phase 3 (6)
            Self::WorldSettingsDisplayName => {
                "optional.world_building_phase.world_settings.display_name"
            }
            Self::WorldSettingsType => "optional.world_building_phase.world_settings.type",
            Self::WorldSettingsDescription => {
                "optional.world_building_phase.world_settings.description"
            }
            Self::WorldSettingsAccess => "optional.world_building_phase.world_settings.access",
            Self::WorldSettingsLoreEntriesDisplayName => {
                "optional.world_building_phase.world_settings.lore_entries.display_name"
            }
            Self::WorldSettingsLoreEntriesDescription => {
                "optional.world_building_phase.world_settings.lore_entries.description"
            }

            // Phase 4 (4)
            Self::SceneConfigStylePresetCode => {
                "optional.scene_config_phase.arcs.chapters.scenes.style_preset_code"
            }
            Self::SceneConfigNarratorConfigCode => {
                "optional.scene_config_phase.arcs.chapters.scenes.narrator_config_code"
            }
            Self::SceneConfigLoreEntries => {
                "optional.scene_config_phase.arcs.chapters.scenes.lore_entries"
            }
            Self::SceneConfigCharactersIsPov => {
                "optional.scene_config_phase.arcs.chapters.scenes.characters.is_pov"
            }

            // Phase 5.1 (6)
            Self::CharacterAppearanceSpecies => {
                "optional.character_appearance_phase.character.species"
            }
            Self::CharacterAppearanceGenderIdentity => {
                "optional.character_appearance_phase.character.gender.identity"
            }
            Self::CharacterAppearanceHeightCm => {
                "optional.character_appearance_phase.character.height_cm"
            }
            Self::CharacterAppearanceHairColor => {
                "optional.character_appearance_phase.character.hair.color"
            }
            Self::CharacterAppearanceSkinTone => {
                "optional.character_appearance_phase.character.skin.tone"
            }
            Self::CharacterAppearanceOutfitStyle => {
                "optional.character_appearance_phase.character.outfit_style"
            }

            // Phase 5.2 (4)
            Self::CharacterPsychologicalTheWound => {
                "optional.character_psychological_phase.character.core_trauma_and_belief.the_wound"
            }
            Self::CharacterPsychologicalMoralCode => {
                "optional.character_psychological_phase.character.cognitive_moral_framework.moral_code"
            }
            Self::CharacterPsychologicalExternalWant => {
                "optional.character_psychological_phase.character.narrative_engine.external_want"
            }
            Self::CharacterPsychologicalSocialMask => {
                "optional.character_psychological_phase.character.behavioral_adaptation.social_mask"
            }

            // Phase 5.3 (2)
            Self::CharacterLifeExperienceMemories => {
                "optional.character_life_experience_phase.character.memories"
            }
            Self::CharacterLifeExperienceCompetences => {
                "optional.character_life_experience_phase.character.competences"
            }

            // Phase 6 (1)
            Self::RelationshipCharacterScenes => {
                "optional.relationship_phase.relationship.character.scenes"
            }
        }
    }

    /// Returns the Phase group for this Master Attribute.
    pub const fn phase_group(self) -> MasterAttributePhaseGroup {
        match self {
            Self::RequireProjectDisplayName
            | Self::RequireProjectSummary
            | Self::RequireProjectLanguage
            | Self::RequireProjectTargetArcCount
            | Self::RequireProjectTargetChapterCount
            | Self::RequireProjectTargetWordPerChapterCount
            | Self::RequireProjectGenresCode
            | Self::RequireCharacterDisplayNameOrNickname
            | Self::RequireCharacterBiologicalSex
            | Self::RequireCharacterMotifCode => MasterAttributePhaseGroup::Phase0Require,

            Self::OverviewArcsDisplayName
            | Self::OverviewArcsSummary
            | Self::OverviewArcsPosition
            | Self::OverviewArcsMotifCode
            | Self::OverviewArcsMotifEventBlocks
            | Self::OverviewArcsThemesQuestion
            | Self::OverviewArcsThemesResolution
            | Self::OverviewArcsThemesMessage
            | Self::OverviewArcsThemesArguments => MasterAttributePhaseGroup::Phase1Overview,

            Self::ChapterPhaseChaptersDisplayName
            | Self::ChapterPhaseChaptersSummary
            | Self::ChapterPhaseScenesDisplayName
            | Self::ChapterPhaseScenesSummary => MasterAttributePhaseGroup::Phase2Chapter,

            Self::WorldSettingsDisplayName
            | Self::WorldSettingsType
            | Self::WorldSettingsDescription
            | Self::WorldSettingsAccess
            | Self::WorldSettingsLoreEntriesDisplayName
            | Self::WorldSettingsLoreEntriesDescription => {
                MasterAttributePhaseGroup::Phase3WorldBuilding
            }

            Self::SceneConfigStylePresetCode
            | Self::SceneConfigNarratorConfigCode
            | Self::SceneConfigLoreEntries
            | Self::SceneConfigCharactersIsPov => MasterAttributePhaseGroup::Phase4SceneConfig,

            Self::CharacterAppearanceSpecies
            | Self::CharacterAppearanceGenderIdentity
            | Self::CharacterAppearanceHeightCm
            | Self::CharacterAppearanceHairColor
            | Self::CharacterAppearanceSkinTone
            | Self::CharacterAppearanceOutfitStyle => {
                MasterAttributePhaseGroup::Phase5Sub1Appearance
            }

            Self::CharacterPsychologicalTheWound
            | Self::CharacterPsychologicalMoralCode
            | Self::CharacterPsychologicalExternalWant
            | Self::CharacterPsychologicalSocialMask => {
                MasterAttributePhaseGroup::Phase5Sub2Psychological
            }

            Self::CharacterLifeExperienceMemories | Self::CharacterLifeExperienceCompetences => {
                MasterAttributePhaseGroup::Phase5Sub3LifeExperience
            }

            Self::RelationshipCharacterScenes => MasterAttributePhaseGroup::Phase6Relationship,
        }
    }
}

/// Evaluation result for a single Master Attribute (`be-core.md` §4.6.2).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AttributeProgressItem {
    pub id: MasterAttributeId,
    pub path: String,
    pub phase_group: MasterAttributePhaseGroup,
    pub state: AttributeProgressState,
}

/// Progress summary over the 46 Master Attributes (`REQ-DOM-003`, `REQ-DOM-005`, `core.md` §3.1, `be-core.md` §4.6.2).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProgressSummary {
    /// Number of Master Attributes in `Confirmed` state (`0..=46`).
    pub confirmed_count: usize,
    /// Number of Master Attributes in `AiSuggested` state (`0..=46`).
    pub ai_suggested_count: usize,
    /// Number of Master Attributes in `Incomplete` state (`0..=46`).
    pub incomplete_count: usize,
    /// Authoritative fixed denominator (`46`).
    pub denominator: usize,
    /// Alias matching `be-core.md` §4.6.2 `total_count` (`46`).
    pub total_count: usize,
    /// Rounded percentage of confirmed attributes (`round((confirmed_count / 46.0) * 100.0)`).
    pub confirmed_pct: u32,
    /// Alias matching `be-core.md` §4.6.2 `confirmed_percent`.
    pub confirmed_percent: u32,
    /// Rounded percentage of AI-suggested attributes (`round((ai_suggested_count / 46.0) * 100.0)`).
    pub ai_suggested_pct: u32,
    /// Alias matching `be-core.md` §4.6.2 `ai_suggested_percent`.
    pub ai_suggested_percent: u32,
    /// True iff all 10 Phase 0 (`require`) attributes are non-`Incomplete` (`Confirmed` or `AiSuggested`).
    pub is_require_ready: bool,
    /// Alias matching `be-core.md` §4.6.2 `is_require_complete`.
    pub is_require_complete: bool,
    /// Per-attribute breakdown across all 46 Master Attributes in canonical order.
    pub attributes: Vec<AttributeProgressItem>,
}

/// Type alias matching `be-core.md` §4.6.2 `ProgressSummaryDto`.
pub type ProgressSummaryDto = ProgressSummary;

impl ProgressSummary {
    /// Looks up the evaluated state of a specific `MasterAttributeId`.
    pub fn state_of(&self, id: MasterAttributeId) -> AttributeProgressState {
        self.attributes
            .iter()
            .find(|item| item.id == id)
            .map(|item| item.state)
            .unwrap_or(AttributeProgressState::Incomplete)
    }

    /// Looks up the evaluated state by canonical path string.
    pub fn state_by_path(&self, path: &str) -> Option<AttributeProgressState> {
        self.attributes
            .iter()
            .find(|item| item.path == path)
            .map(|item| item.state)
    }
}

/// Evaluates all 46 Master Attributes on `data` and returns the deterministic `ProgressSummary`
/// (`REQ-DOM-003`, `REQ-DOM-005`, `T-UNIT-01`, `P3S1-B`).
pub fn evaluate_progress(data: &BusinessFlowData) -> ProgressSummary {
    let mut confirmed_count = 0usize;
    let mut ai_suggested_count = 0usize;
    let mut incomplete_count = 0usize;
    let mut attributes = Vec::with_capacity(MASTER_ATTRIBUTE_DENOMINATOR);

    for attr_id in MASTER_ATTRIBUTES_LIST {
        let state = evaluate_attribute(data, attr_id);
        match state {
            AttributeProgressState::Confirmed => confirmed_count += 1,
            AttributeProgressState::AiSuggested => ai_suggested_count += 1,
            AttributeProgressState::Incomplete => incomplete_count += 1,
        }
        attributes.push(AttributeProgressItem {
            id: attr_id,
            path: attr_id.path().to_string(),
            phase_group: attr_id.phase_group(),
            state,
        });
    }

    debug_assert_eq!(
        confirmed_count + ai_suggested_count + incomplete_count,
        MASTER_ATTRIBUTE_DENOMINATOR
    );

    let confirmed_pct =
        ((confirmed_count as f64 / MASTER_ATTRIBUTE_DENOMINATOR as f64) * 100.0).round() as u32;
    let ai_suggested_pct =
        ((ai_suggested_count as f64 / MASTER_ATTRIBUTE_DENOMINATOR as f64) * 100.0).round() as u32;

    let is_require_ready = attributes
        .iter()
        .filter(|a| a.phase_group == MasterAttributePhaseGroup::Phase0Require)
        .all(|a| a.state != AttributeProgressState::Incomplete);

    ProgressSummary {
        confirmed_count,
        ai_suggested_count,
        incomplete_count,
        denominator: MASTER_ATTRIBUTE_DENOMINATOR,
        total_count: MASTER_ATTRIBUTE_DENOMINATOR,
        confirmed_pct,
        confirmed_percent: confirmed_pct,
        ai_suggested_pct,
        ai_suggested_percent: ai_suggested_pct,
        is_require_ready,
        is_require_complete: is_require_ready,
        attributes,
    }
}

/// Evaluates a single `MasterAttributeId` on `data` according to `core.md` §3.1.3 and `be-core.md` §4.6.2.
pub fn evaluate_attribute(
    data: &BusinessFlowData,
    attr_id: MasterAttributeId,
) -> AttributeProgressState {
    match attr_id {
        // =========================================================================
        // Phase 0 (`require`): 10 attributes
        // =========================================================================
        MasterAttributeId::RequireProjectDisplayName => {
            eval_tracked_string(&data.require.project.display_name)
        }
        MasterAttributeId::RequireProjectSummary => {
            eval_tracked_string(&data.require.project.summary)
        }
        MasterAttributeId::RequireProjectLanguage => {
            eval_tracked_string(&data.require.project.language)
        }
        MasterAttributeId::RequireProjectTargetArcCount => {
            eval_tracked_positive_u32(&data.require.project.target.arc_count)
        }
        MasterAttributeId::RequireProjectTargetChapterCount => {
            eval_tracked_positive_u32(&data.require.project.target.chapter_count)
        }
        MasterAttributeId::RequireProjectTargetWordPerChapterCount => {
            eval_tracked_positive_u32(&data.require.project.target.word_per_chapter_count)
        }
        MasterAttributeId::RequireProjectGenresCode => eval_array_elements(
            data.require
                .project
                .genres
                .iter()
                .map(eval_require_genre_item),
        ),
        MasterAttributeId::RequireCharacterDisplayNameOrNickname => eval_array_elements(
            data.require
                .character
                .iter()
                .map(eval_character_name_or_nickname),
        ),
        MasterAttributeId::RequireCharacterBiologicalSex => eval_array_elements(
            data.require
                .character
                .iter()
                .map(|c| eval_tracked_string_with_parent(&c.biological_sex, c.source)),
        ),
        MasterAttributeId::RequireCharacterMotifCode => eval_array_elements(
            data.require
                .character
                .iter()
                .map(|c| eval_tracked_string_with_parent(&c.motif_code, c.source)),
        ),

        // =========================================================================
        // Phase 1 (`optional.overview_phase`): 9 attributes
        // =========================================================================
        MasterAttributeId::OverviewArcsDisplayName => eval_array_elements(
            data.optional
                .overview_phase
                .arcs
                .iter()
                .map(|a| eval_tracked_string_with_parent(&a.display_name, a.source)),
        ),
        MasterAttributeId::OverviewArcsSummary => eval_array_elements(
            data.optional
                .overview_phase
                .arcs
                .iter()
                .map(|a| eval_tracked_string_with_parent(&a.summary, a.source)),
        ),
        MasterAttributeId::OverviewArcsPosition => eval_array_elements(
            data.optional
                .overview_phase
                .arcs
                .iter()
                .map(|a| eval_tracked_positive_u32_with_parent(&a.position, a.source)),
        ),
        MasterAttributeId::OverviewArcsMotifCode => eval_array_elements(
            data.optional
                .overview_phase
                .arcs
                .iter()
                .map(|a| eval_tracked_string_with_parent(&a.motif.code, a.source)),
        ),
        MasterAttributeId::OverviewArcsMotifEventBlocks => {
            eval_array_elements(data.optional.overview_phase.arcs.iter().flat_map(|a| {
                a.motif
                    .event_blocks
                    .iter()
                    .map(move |eb| eval_arc_event_block(eb, a.source))
            }))
        }
        MasterAttributeId::OverviewArcsThemesQuestion => {
            eval_array_elements(data.optional.overview_phase.arcs.iter().map(|a| {
                eval_tracked_string_with_parent(&a.themes.question, a.themes.source.or(a.source))
            }))
        }
        MasterAttributeId::OverviewArcsThemesResolution => {
            eval_array_elements(
                data.optional.overview_phase.arcs.iter().map(|a| {
                    eval_theme_resolution(&a.themes.resolution, a.themes.source.or(a.source))
                }),
            )
        }
        MasterAttributeId::OverviewArcsThemesMessage => {
            eval_array_elements(data.optional.overview_phase.arcs.iter().map(|a| {
                eval_tracked_string_with_parent(&a.themes.message, a.themes.source.or(a.source))
            }))
        }
        MasterAttributeId::OverviewArcsThemesArguments => {
            eval_array_elements(data.optional.overview_phase.arcs.iter().flat_map(|a| {
                let fallback = a.themes.source.or(a.source);
                a.themes
                    .arguments
                    .iter()
                    .map(move |arg| eval_theme_argument(arg, fallback))
            }))
        }

        // =========================================================================
        // Phase 2 (`optional.chapter_phase`): 4 attributes
        // =========================================================================
        MasterAttributeId::ChapterPhaseChaptersDisplayName => eval_array_elements(
            data.optional
                .chapter_phase
                .arcs
                .iter()
                .flat_map(|a| a.chapters.iter())
                .map(|ch| eval_tracked_string_with_parent(&ch.display_name, ch.source)),
        ),
        MasterAttributeId::ChapterPhaseChaptersSummary => eval_array_elements(
            data.optional
                .chapter_phase
                .arcs
                .iter()
                .flat_map(|a| a.chapters.iter())
                .map(|ch| eval_tracked_string_with_parent(&ch.summary, ch.source)),
        ),
        MasterAttributeId::ChapterPhaseScenesDisplayName => eval_array_elements(
            data.optional
                .chapter_phase
                .arcs
                .iter()
                .flat_map(|a| a.chapters.iter())
                .flat_map(|ch| ch.scenes.iter())
                .map(|sc| eval_tracked_string_with_parent(&sc.display_name, sc.source)),
        ),
        MasterAttributeId::ChapterPhaseScenesSummary => eval_array_elements(
            data.optional
                .chapter_phase
                .arcs
                .iter()
                .flat_map(|a| a.chapters.iter())
                .flat_map(|ch| ch.scenes.iter())
                .map(|sc| eval_tracked_string_with_parent(&sc.summary, sc.source)),
        ),

        // =========================================================================
        // Phase 3 (`optional.world_building_phase`): 6 attributes
        // =========================================================================
        MasterAttributeId::WorldSettingsDisplayName => eval_array_elements(
            data.optional
                .world_building_phase
                .world_settings
                .iter()
                .map(|ws| eval_tracked_string_with_parent(&ws.display_name, ws.source)),
        ),
        MasterAttributeId::WorldSettingsType => eval_array_elements(
            data.optional
                .world_building_phase
                .world_settings
                .iter()
                .map(|ws| eval_tracked_string_with_parent(&ws.setting_type, ws.source)),
        ),
        MasterAttributeId::WorldSettingsDescription => eval_array_elements(
            data.optional
                .world_building_phase
                .world_settings
                .iter()
                .map(|ws| eval_tracked_string_with_parent(&ws.description, ws.source)),
        ),
        MasterAttributeId::WorldSettingsAccess => eval_array_elements(
            data.optional
                .world_building_phase
                .world_settings
                .iter()
                .map(|ws| eval_tracked_string_with_parent(&ws.access, ws.source)),
        ),
        MasterAttributeId::WorldSettingsLoreEntriesDisplayName => eval_array_elements(
            data.optional
                .world_building_phase
                .world_settings
                .iter()
                .flat_map(|ws| {
                    ws.lore_entries.iter().map(move |le| {
                        eval_tracked_string_with_parent(&le.display_name, le.source.or(ws.source))
                    })
                }),
        ),
        MasterAttributeId::WorldSettingsLoreEntriesDescription => eval_array_elements(
            data.optional
                .world_building_phase
                .world_settings
                .iter()
                .flat_map(|ws| {
                    ws.lore_entries
                        .iter()
                        .map(move |le| eval_lore_entry_description(le, ws.source))
                }),
        ),

        // =========================================================================
        // Phase 4 (`optional.scene_config_phase`): 4 attributes
        // =========================================================================
        MasterAttributeId::SceneConfigStylePresetCode => eval_array_elements(
            data.optional
                .scene_config_phase
                .arcs
                .iter()
                .flat_map(|a| a.chapters.iter())
                .flat_map(|ch| ch.scenes.iter())
                .map(|sc| eval_tracked_string(&sc.style_preset_code)),
        ),
        MasterAttributeId::SceneConfigNarratorConfigCode => eval_array_elements(
            data.optional
                .scene_config_phase
                .arcs
                .iter()
                .flat_map(|a| a.chapters.iter())
                .flat_map(|ch| ch.scenes.iter())
                .map(|sc| eval_tracked_string(&sc.narrator_config_code)),
        ),
        MasterAttributeId::SceneConfigLoreEntries => eval_array_elements(
            data.optional
                .scene_config_phase
                .arcs
                .iter()
                .flat_map(|a| a.chapters.iter())
                .flat_map(|ch| ch.scenes.iter())
                .flat_map(|sc| sc.lore_entries.iter())
                .map(eval_scene_lore_entry_link),
        ),
        MasterAttributeId::SceneConfigCharactersIsPov => eval_array_elements(
            data.optional
                .scene_config_phase
                .arcs
                .iter()
                .flat_map(|a| a.chapters.iter())
                .flat_map(|ch| ch.scenes.iter())
                .map(|sc| eval_scene_characters_pov(&sc.narrator_config_code, &sc.characters)),
        ),

        // =========================================================================
        // Phase 5.1 (`optional.character_appearance_phase`): 6 attributes
        // =========================================================================
        MasterAttributeId::CharacterAppearanceSpecies => eval_array_elements(
            data.optional
                .character_appearance_phase
                .character
                .iter()
                .map(|c| eval_tracked_string(&c.species)),
        ),
        MasterAttributeId::CharacterAppearanceGenderIdentity => eval_array_elements(
            data.optional
                .character_appearance_phase
                .character
                .iter()
                .map(|c| eval_tracked_string(&c.gender.identity)),
        ),
        MasterAttributeId::CharacterAppearanceHeightCm => eval_array_elements(
            data.optional
                .character_appearance_phase
                .character
                .iter()
                .map(|c| eval_tracked_positive_u32(&c.height_cm)),
        ),
        MasterAttributeId::CharacterAppearanceHairColor => eval_array_elements(
            data.optional
                .character_appearance_phase
                .character
                .iter()
                .map(|c| eval_tracked_string(&c.hair.color)),
        ),
        MasterAttributeId::CharacterAppearanceSkinTone => eval_array_elements(
            data.optional
                .character_appearance_phase
                .character
                .iter()
                .map(|c| eval_tracked_string(&c.skin.tone)),
        ),
        MasterAttributeId::CharacterAppearanceOutfitStyle => eval_array_elements(
            data.optional
                .character_appearance_phase
                .character
                .iter()
                .map(|c| eval_tracked_string(&c.outfit_style)),
        ),

        // =========================================================================
        // Phase 5.2 (`optional.character_psychological_phase`): 4 attributes
        // =========================================================================
        MasterAttributeId::CharacterPsychologicalTheWound => eval_array_elements(
            data.optional
                .character_psychological_phase
                .character
                .iter()
                .map(|c| eval_tracked_string(&c.core_trauma_and_belief.the_wound)),
        ),
        MasterAttributeId::CharacterPsychologicalMoralCode => eval_array_elements(
            data.optional
                .character_psychological_phase
                .character
                .iter()
                .map(|c| eval_tracked_string(&c.cognitive_moral_framework.moral_code)),
        ),
        MasterAttributeId::CharacterPsychologicalExternalWant => eval_array_elements(
            data.optional
                .character_psychological_phase
                .character
                .iter()
                .map(|c| eval_tracked_string(&c.narrative_engine.external_want)),
        ),
        MasterAttributeId::CharacterPsychologicalSocialMask => eval_array_elements(
            data.optional
                .character_psychological_phase
                .character
                .iter()
                .map(|c| {
                    eval_social_mask(
                        &c.behavioral_adaptation.social_mask.persona_type,
                        &c.behavioral_adaptation.social_mask.verbal_style,
                    )
                }),
        ),

        // =========================================================================
        // Phase 5.3 (`optional.character_life_experience_phase`): 2 attributes
        // =========================================================================
        MasterAttributeId::CharacterLifeExperienceMemories => eval_array_elements(
            data.optional
                .character_life_experience_phase
                .character
                .iter()
                .flat_map(|c| c.memories.iter())
                .map(eval_character_memory),
        ),
        MasterAttributeId::CharacterLifeExperienceCompetences => eval_array_elements(
            data.optional
                .character_life_experience_phase
                .character
                .iter()
                .flat_map(|c| c.competences.iter())
                .map(eval_character_competence),
        ),

        // =========================================================================
        // Phase 6 (`optional.relationship_phase`): 1 attribute
        // =========================================================================
        MasterAttributeId::RelationshipCharacterScenes => eval_array_elements(
            data.optional
                .relationship_phase
                .relationship
                .iter()
                .flat_map(|rel| {
                    rel.character
                        .scenes
                        .iter()
                        .map(move |sc| eval_relationship_scene_state(sc, rel.source))
                }),
        ),
    }
}

/// Evaluates whether a single `ProvenanceSource` maps to `Confirmed`, `AiSuggested`, or `Incomplete`.
pub const fn classify_source(source: Option<ProvenanceSource>) -> AttributeProgressState {
    match source {
        Some(ProvenanceSource::User | ProvenanceSource::UserEdited) => {
            AttributeProgressState::Confirmed
        }
        Some(ProvenanceSource::AiGenerated) => AttributeProgressState::AiSuggested,
        Some(ProvenanceSource::System | ProvenanceSource::Incomplete) | None => {
            AttributeProgressState::Incomplete
        }
    }
}

/// Combines all required field states of a single composite element (`core.md` §3.1.3, `be-core.md` §4.6.2):
/// - If any required field is `Incomplete` (or the slice is empty) -> the element is `Incomplete`.
/// - If all required fields are `Confirmed` (`User` or `UserEdited`) -> the element is `Confirmed`.
/// - If all required fields are complete and at least one is `AiSuggested` -> the element is `AiSuggested`.
pub fn combine_required_element_fields(
    field_states: &[AttributeProgressState],
) -> AttributeProgressState {
    if field_states.is_empty() {
        return AttributeProgressState::Incomplete;
    }
    let mut has_ai = false;
    for state in field_states {
        match state {
            AttributeProgressState::Incomplete => return AttributeProgressState::Incomplete,
            AttributeProgressState::AiSuggested => has_ai = true,
            AttributeProgressState::Confirmed => {}
        }
    }
    if has_ai {
        AttributeProgressState::AiSuggested
    } else {
        AttributeProgressState::Confirmed
    }
}

/// Aggregates element states across an array according to deterministic precedence
/// `Confirmed > AiSuggested > Incomplete` (`core.md` §3.1.3, `be-core.md` §4.6.2, `P3S1-B`, `T-UNIT-01`).
pub fn eval_array_elements<I>(elements: I) -> AttributeProgressState
where
    I: IntoIterator<Item = AttributeProgressState>,
{
    let mut saw_ai = false;
    for state in elements {
        match state {
            AttributeProgressState::Confirmed => return AttributeProgressState::Confirmed,
            AttributeProgressState::AiSuggested => saw_ai = true,
            AttributeProgressState::Incomplete => {}
        }
    }
    if saw_ai {
        AttributeProgressState::AiSuggested
    } else {
        AttributeProgressState::Incomplete
    }
}

fn eval_tracked_string(field: &TrackedField<String>) -> AttributeProgressState {
    eval_tracked_string_with_parent(field, None)
}

fn eval_tracked_string_with_parent(
    field: &TrackedField<String>,
    parent_source: Option<ProvenanceSource>,
) -> AttributeProgressState {
    match &field.value {
        Some(s) if !s.trim().is_empty() => classify_source(field.source.or(parent_source)),
        _ => AttributeProgressState::Incomplete,
    }
}

fn eval_tracked_positive_u32(field: &TrackedField<u32>) -> AttributeProgressState {
    eval_tracked_positive_u32_with_parent(field, None)
}

fn eval_tracked_positive_u32_with_parent(
    field: &TrackedField<u32>,
    parent_source: Option<ProvenanceSource>,
) -> AttributeProgressState {
    match field.value {
        Some(v) if v > 0 => classify_source(field.source.or(parent_source)),
        _ => AttributeProgressState::Incomplete,
    }
}

fn eval_tracked_bool_with_parent(
    field: &TrackedField<bool>,
    parent_source: Option<ProvenanceSource>,
) -> AttributeProgressState {
    match field.value {
        Some(_) => classify_source(field.source.or(parent_source)),
        None => AttributeProgressState::Incomplete,
    }
}

fn eval_require_genre_item(genre: &RequireGenreItem) -> AttributeProgressState {
    let code_state = eval_tracked_string(&genre.code);
    if code_state == AttributeProgressState::Incomplete {
        return AttributeProgressState::Incomplete;
    }
    // If is_primary or display_name are also populated, include them in composite provenance
    let mut required = vec![code_state];
    if genre.is_primary.value.is_some() {
        required.push(eval_tracked_bool_with_parent(
            &genre.is_primary,
            genre.code.source,
        ));
    }
    if genre
        .display_name
        .value
        .as_ref()
        .is_some_and(|s| !s.trim().is_empty())
    {
        required.push(eval_tracked_string_with_parent(
            &genre.display_name,
            genre.code.source,
        ));
    }
    combine_required_element_fields(&required)
}

/// Evaluates `require.character.display_name` *(or `nickname`)* per `core.md` §2.1, §2.8 and `be-core.md` §4.6.1.
fn eval_character_name_or_nickname(ch: &RequireCharacterItem) -> AttributeProgressState {
    let dn = eval_tracked_string_with_parent(&ch.display_name, ch.source);
    let nn = eval_tracked_string_with_parent(&ch.nickname, ch.source);
    match (dn, nn) {
        (AttributeProgressState::Incomplete, AttributeProgressState::Incomplete) => {
            AttributeProgressState::Incomplete
        }
        (a, AttributeProgressState::Incomplete) => a,
        (AttributeProgressState::Incomplete, b) => b,
        (a, b) => combine_required_element_fields(&[a, b]),
    }
}

fn eval_arc_event_block(
    eb: &ArcEventBlockItem,
    arc_source: Option<ProvenanceSource>,
) -> AttributeProgressState {
    let fallback = eb.source.or(arc_source);
    let summary_state = eval_tracked_string_with_parent(&eb.summary, fallback);
    if summary_state == AttributeProgressState::Incomplete {
        return AttributeProgressState::Incomplete;
    }

    // Event block requires summary plus either chapter codes (start_chapter_code & end_chapter_code)
    // or chapter numbers (start_chapter & end_chapter) or event_block_code.
    let mut required = vec![summary_state];

    let start_code_state = eval_tracked_string_with_parent(&eb.start_chapter_code, fallback);
    let end_code_state = eval_tracked_string_with_parent(&eb.end_chapter_code, fallback);
    let start_num_state = eval_tracked_positive_u32_with_parent(&eb.start_chapter, fallback);
    let end_num_state = eval_tracked_positive_u32_with_parent(&eb.end_chapter, fallback);

    if start_code_state != AttributeProgressState::Incomplete
        && end_code_state != AttributeProgressState::Incomplete
    {
        required.push(start_code_state);
        required.push(end_code_state);
    } else if start_num_state != AttributeProgressState::Incomplete
        && end_num_state != AttributeProgressState::Incomplete
    {
        if eb.start_chapter.value.unwrap_or(1) > eb.end_chapter.value.unwrap_or(0) {
            return AttributeProgressState::Incomplete;
        }
        required.push(start_num_state);
        required.push(end_num_state);
    }

    if eb
        .event_block_code
        .value
        .as_ref()
        .is_some_and(|s| !s.trim().is_empty())
    {
        required.push(eval_tracked_string_with_parent(
            &eb.event_block_code,
            fallback,
        ));
    }

    combine_required_element_fields(&required)
}

fn eval_theme_resolution(
    res: &ThemeResolutionSection,
    fallback: Option<ProvenanceSource>,
) -> AttributeProgressState {
    let type_state = eval_tracked_string_with_parent(&res.resolution_type, fallback);
    let content_state = eval_tracked_string_with_parent(&res.content, fallback);

    match (type_state, content_state) {
        (AttributeProgressState::Incomplete, AttributeProgressState::Incomplete) => {
            AttributeProgressState::Incomplete
        }
        // If both resolution.type and resolution.content are present, both must be satisfied for composite provenance
        (a, b)
            if a != AttributeProgressState::Incomplete
                && b != AttributeProgressState::Incomplete =>
        {
            combine_required_element_fields(&[a, b])
        }
        // If only content or only type is populated while the other is unset, check if the populated one is valid
        (a, AttributeProgressState::Incomplete) if res.content.value.is_none() => a,
        (AttributeProgressState::Incomplete, b) if res.resolution_type.value.is_none() => b,
        _ => AttributeProgressState::Incomplete,
    }
}

fn eval_theme_argument(
    arg: &ThemeArgumentItem,
    arc_theme_source: Option<ProvenanceSource>,
) -> AttributeProgressState {
    let fallback = arg.source.or(arc_theme_source);
    let char_code = eval_tracked_string_with_parent(&arg.character_code, fallback);
    let char_name = eval_tracked_string_with_parent(&arg.character_display_name, fallback);
    let has_char = char_code != AttributeProgressState::Incomplete
        || char_name != AttributeProgressState::Incomplete;
    if !has_char {
        return AttributeProgressState::Incomplete;
    }

    let mut required = Vec::new();
    if char_code != AttributeProgressState::Incomplete {
        required.push(char_code);
    } else {
        required.push(char_name);
    }

    let role = eval_tracked_string_with_parent(&arg.role, fallback);
    let mode = eval_tracked_string_with_parent(&arg.answer_mode, fallback);
    let note = eval_tracked_string_with_parent(&arg.note, fallback);

    if role == AttributeProgressState::Incomplete
        && mode == AttributeProgressState::Incomplete
        && note == AttributeProgressState::Incomplete
    {
        return AttributeProgressState::Incomplete;
    }
    if role != AttributeProgressState::Incomplete {
        required.push(role);
    }
    if mode != AttributeProgressState::Incomplete {
        required.push(mode);
    }
    if note != AttributeProgressState::Incomplete {
        required.push(note);
    }

    combine_required_element_fields(&required)
}

fn eval_lore_entry_description(
    le: &LoreEntryItem,
    ws_source: Option<ProvenanceSource>,
) -> AttributeProgressState {
    eval_tracked_string_with_parent(&le.description, le.source.or(ws_source))
}

fn eval_scene_lore_entry_link(link: &SceneLoreEntryLink) -> AttributeProgressState {
    if link.lore_entry_code.trim().is_empty() {
        return AttributeProgressState::Incomplete;
    }
    let fallback = link.source;
    let mut fields = Vec::new();
    if link.usage.value.is_some() {
        fields.push(eval_tracked_string_with_parent(&link.usage, fallback));
    }
    if link.reveal_to_reader.value.is_some() {
        fields.push(eval_tracked_string_with_parent(
            &link.reveal_to_reader,
            fallback,
        ));
    }
    if link.reveal_to_pov.value.is_some() {
        fields.push(eval_tracked_string_with_parent(
            &link.reveal_to_pov,
            fallback,
        ));
    }
    if fields.is_empty() {
        classify_source(fallback)
    } else {
        combine_required_element_fields(&fields)
    }
}

/// Evaluates `optional.scene_config_phase.arcs.chapters.scenes.characters.is_pov`
/// checking validity against `narrator_config_code` (`be-core.md` §4.6.1, `core.md` §2.8, §3.1.5).
fn eval_scene_characters_pov(
    narrator_config_code: &TrackedField<String>,
    characters: &[SceneCharacterLink],
) -> AttributeProgressState {
    if characters.is_empty() {
        return AttributeProgressState::Incomplete;
    }

    let mut field_states = Vec::with_capacity(characters.len());
    let mut pov_true_count = 0usize;

    for ch in characters {
        if ch.character_code.trim().is_empty() {
            return AttributeProgressState::Incomplete;
        }
        let st = eval_tracked_bool_with_parent(&ch.is_pov, ch.source);
        if st == AttributeProgressState::Incomplete {
            return AttributeProgressState::Incomplete;
        }
        if ch.is_pov.value == Some(true) {
            pov_true_count += 1;
        }
        field_states.push(st);
    }

    // Check POV count validity against narrator_config_code when present (`core.md` §2.8, `be-core.md` §4.6.1)
    if let Some(raw_nc) = &narrator_config_code.value {
        let nc = raw_nc.trim().to_ascii_uppercase();
        if !nc.is_empty() {
            let is_omniscient_or_objective = nc.contains("OMNISCIENT") || nc.contains("OBJECTIVE");
            let is_multiple_pov = nc.contains("MULTIPLE") || nc.contains("MULTI_POV");
            let is_limited = nc.contains("FIRST_PERSON")
                || nc.contains("SECOND_PERSON")
                || nc.contains("LIMITED")
                || nc.contains("THIRD_PERSON_LIMITED");

            if is_omniscient_or_objective && pov_true_count != 0 {
                return AttributeProgressState::Incomplete;
            }
            if is_multiple_pov && pov_true_count < 2 {
                return AttributeProgressState::Incomplete;
            }
            if is_limited && !is_multiple_pov && pov_true_count != 1 {
                return AttributeProgressState::Incomplete;
            }
        }
    } else if pov_true_count == 0 {
        // Without an omniscient narrator_config_code, all-false is_pov is not yet configured with a POV
        return AttributeProgressState::Incomplete;
    }

    combine_required_element_fields(&field_states)
}

fn eval_social_mask(
    persona_type: &TrackedField<String>,
    verbal_style: &TrackedField<String>,
) -> AttributeProgressState {
    let p_state = eval_tracked_string(persona_type);
    let v_state = eval_tracked_string(verbal_style);
    match (p_state, v_state) {
        (AttributeProgressState::Incomplete, AttributeProgressState::Incomplete) => {
            AttributeProgressState::Incomplete
        }
        (a, b)
            if a != AttributeProgressState::Incomplete
                && b != AttributeProgressState::Incomplete =>
        {
            combine_required_element_fields(&[a, b])
        }
        (a, AttributeProgressState::Incomplete) if verbal_style.value.is_none() => a,
        (AttributeProgressState::Incomplete, b) if persona_type.value.is_none() => b,
        _ => AttributeProgressState::Incomplete,
    }
}

fn eval_character_memory(mem: &CharacterMemoryItem) -> AttributeProgressState {
    eval_tracked_string_with_parent(&mem.content, mem.source)
}

fn eval_character_competence(comp: &CharacterCompetenceItem) -> AttributeProgressState {
    if comp.code.trim().is_empty() {
        return AttributeProgressState::Incomplete;
    }
    let fallback = comp.source;
    let name_or_domain = {
        let dn = eval_tracked_string_with_parent(&comp.display_name, fallback);
        let dom = eval_tracked_string_with_parent(&comp.domain_code, fallback);
        if dn != AttributeProgressState::Incomplete && dom != AttributeProgressState::Incomplete {
            combine_required_element_fields(&[dn, dom])
        } else if dn != AttributeProgressState::Incomplete {
            dn
        } else {
            dom
        }
    };
    if name_or_domain == AttributeProgressState::Incomplete {
        return AttributeProgressState::Incomplete;
    }

    let mut required = vec![name_or_domain];
    if comp
        .scene_code
        .value
        .as_ref()
        .is_some_and(|s| !s.trim().is_empty())
    {
        required.push(eval_tracked_string_with_parent(&comp.scene_code, fallback));
    }
    if comp
        .level_code
        .value
        .as_ref()
        .is_some_and(|s| !s.trim().is_empty())
    {
        required.push(eval_tracked_string_with_parent(&comp.level_code, fallback));
    }
    if comp
        .description
        .value
        .as_ref()
        .is_some_and(|s| !s.trim().is_empty())
    {
        required.push(eval_tracked_string_with_parent(&comp.description, fallback));
    }

    combine_required_element_fields(&required)
}

fn eval_relationship_scene_state(
    sc: &RelationshipSceneStateItem,
    rel_source: Option<ProvenanceSource>,
) -> AttributeProgressState {
    if sc.scene_code.trim().is_empty() {
        return AttributeProgressState::Incomplete;
    }
    let fallback = sc.source.or(rel_source);
    let dims = [
        &sc.trust,
        &sc.affection,
        &sc.hostility,
        &sc.fear,
        &sc.respect,
        &sc.dependency,
        &sc.familiarity,
    ];
    let mut populated = Vec::new();
    for dim in dims {
        if dim.value.as_ref().is_some_and(|s| !s.trim().is_empty()) {
            populated.push(eval_tracked_string_with_parent(dim, fallback));
        }
    }
    if populated.is_empty() {
        AttributeProgressState::Incomplete
    } else {
        combine_required_element_fields(&populated)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn master_attribute_list_has_exact_46_distribution() {
        assert_eq!(MASTER_ATTRIBUTES_LIST.len(), 46);
        let count_group = |g: MasterAttributePhaseGroup| {
            MASTER_ATTRIBUTES_LIST
                .iter()
                .filter(|id| id.phase_group() == g)
                .count()
        };
        assert_eq!(count_group(MasterAttributePhaseGroup::Phase0Require), 10);
        assert_eq!(count_group(MasterAttributePhaseGroup::Phase1Overview), 9);
        assert_eq!(count_group(MasterAttributePhaseGroup::Phase2Chapter), 4);
        assert_eq!(
            count_group(MasterAttributePhaseGroup::Phase3WorldBuilding),
            6
        );
        assert_eq!(count_group(MasterAttributePhaseGroup::Phase4SceneConfig), 4);
        assert_eq!(
            count_group(MasterAttributePhaseGroup::Phase5Sub1Appearance),
            6
        );
        assert_eq!(
            count_group(MasterAttributePhaseGroup::Phase5Sub2Psychological),
            4
        );
        assert_eq!(
            count_group(MasterAttributePhaseGroup::Phase5Sub3LifeExperience),
            2
        );
        assert_eq!(
            count_group(MasterAttributePhaseGroup::Phase6Relationship),
            1
        );
    }
}
