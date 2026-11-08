//! Cascade Delete Engine & 9 Mandatory Safeguards (`REQ-DOM-004`, `core.md` §1.3 `Q1`, `be-core.md` §5.4, `T-CASC-01`).
//!
//! Enforces:
//! 1. Complete transitive dependency set calculation before any mutation.
//! 2. Mandatory reference vs ownership classification:
//!    - WorldSetting referenced by any Scene -> HARD BLOCK (`referenced_entity_in_use`).
//!    - Character with existing Relationship -> HARD BLOCK (`referenced_entity_in_use`).
//! 3. Protected provenance checks:
//!    - AI callers attempting to delete an entity or cascade into descendants with `source = user`
//!      or `user_edited` return `pending_confirmation` with a full dynamic cascade preview.
//! 4. Scene cascade semantics:
//!    - Physically deletes `scene_characters`, `scene_lore_entries`, `character_competences`,
//!      and `relationship_states` belonging to that scene (preserving parent `relationships`).
//!    - Unlinks `character_memories` by setting `source_scene_code = null`.
//! 5. LoreEntry cascade semantics:
//!    - Deletes lore entry and atomically unlinks all `scene_lore_entries`, preserving `scene`.
//! 6. Pure in-memory execution on clones of `BusinessFlowData` (`writes stay in WorkingState clone`).

use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, HashSet};

use crate::domain::business_flow::{BusinessFlowData, ProvenanceSource};
use crate::domain::tools::registry::CallerCategory;
use crate::error::ErrorCode;

/// Reference to an entity marked for physical deletion in cascade preview.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CascadeEntityRef {
    pub entity_type: String,
    pub code: String,
    pub display_name: Option<String>,
    pub source: Option<ProvenanceSource>,
}

/// Reference to a link/junction marked for deletion in cascade preview.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CascadeLinkRef {
    pub link_type: String,
    pub parent_code: String,
    pub item_code: String,
    pub source: Option<ProvenanceSource>,
}

/// Reference to a soft reference marked for nullification in cascade preview.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CascadeNullifyRef {
    pub entity_type: String,
    pub owner_code: String,
    pub field_name: String,
    pub previous_value: Option<String>,
    pub source: Option<ProvenanceSource>,
}

/// Protected item that prevents direct AI deletion without user confirmation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CascadeProtectedItem {
    pub item_type: String,
    pub code: String,
    pub source: ProvenanceSource,
    pub description: String,
}

/// Reference that strictly blocks deletion (`referenced_entity_in_use`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CascadeBlockingRef {
    pub blocking_type: String,
    pub referencing_code: String,
    pub reason: String,
}

/// Full dynamic transitive impact preview computed prior to deletion (`REQ-DOM-004`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CascadePreview {
    pub target_entity: String,
    pub target_code: String,
    pub entities_to_delete: Vec<CascadeEntityRef>,
    pub links_to_delete: Vec<CascadeLinkRef>,
    pub references_to_nullify: Vec<CascadeNullifyRef>,
    pub protected_items: Vec<CascadeProtectedItem>,
    pub blocking_references: Vec<CascadeBlockingRef>,
    pub referencing_scenes: Vec<String>,
}

impl CascadePreview {
    pub fn new(target_entity: impl Into<String>, target_code: impl Into<String>) -> Self {
        Self {
            target_entity: target_entity.into(),
            target_code: target_code.into(),
            entities_to_delete: Vec::new(),
            links_to_delete: Vec::new(),
            references_to_nullify: Vec::new(),
            protected_items: Vec::new(),
            blocking_references: Vec::new(),
            referencing_scenes: Vec::new(),
        }
    }

    /// True if deletion is blocked by foreign key / mandatory references.
    pub fn is_blocked(&self) -> bool {
        !self.blocking_references.is_empty() || !self.referencing_scenes.is_empty()
    }

    /// True if any affected element or descendant has protected provenance (`User` or `UserEdited`).
    pub fn has_protected_items(&self) -> bool {
        !self.protected_items.is_empty()
    }
}

/// Proposal for a pending deletion requiring user confirmation (`be-core.md` §5.3.2, §5.4).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PendingProposal {
    pub patch_id: String,
    pub operation: String, // "delete"
    pub target_type: String,
    pub target_code: String,
    pub field_name: Option<String>,
    pub previous_value: Option<serde_json::Value>,
    pub new_value: Option<serde_json::Value>,
    pub status: String, // "pending"
}

/// Outcome of executing a cascade deletion.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CascadeOutcome {
    /// Deletion was successfully executed on working state clone.
    Applied {
        preview: CascadePreview,
        mutated_state: Box<BusinessFlowData>,
    },
    /// AI caller attempted to delete protected items; requires user confirmation.
    PendingConfirmation {
        preview: CascadePreview,
        proposal: PendingProposal,
    },
    /// Deletion was rejected (e.g. referenced entity in use, invalid target, invariant violation).
    Rejected {
        error: String,
        referencing_scenes: Vec<String>,
        details: Option<serde_json::Value>,
    },
}

impl CascadeOutcome {
    pub fn status(&self) -> &'static str {
        match self {
            Self::Applied { .. } => "applied",
            Self::PendingConfirmation { .. } => "pending_confirmation",
            Self::Rejected { .. } => "rejected",
        }
    }

    pub fn is_applied(&self) -> bool {
        matches!(self, Self::Applied { .. })
    }

    pub fn is_pending(&self) -> bool {
        matches!(self, Self::PendingConfirmation { .. })
    }

    pub fn is_rejected(&self) -> bool {
        matches!(self, Self::Rejected { .. })
    }

    pub fn preview(&self) -> Option<&CascadePreview> {
        match self {
            Self::Applied { preview, .. } => Some(preview),
            Self::PendingConfirmation { preview, .. } => Some(preview),
            Self::Rejected { .. } => None,
        }
    }

    pub fn mutated_state(&self) -> Option<&BusinessFlowData> {
        match self {
            Self::Applied { mutated_state, .. } => Some(mutated_state.as_ref()),
            _ => None,
        }
    }
}

/// Pure cascade planner computing full transitive impact preview before deletion (`REQ-DOM-004`).
pub struct CascadePlanner;

impl CascadePlanner {
    /// Computes full transitive impact preview for deleting `target_code` of entity type `target_entity`.
    pub fn plan(
        state: &BusinessFlowData,
        target_entity: &str,
        target_code: &str,
    ) -> Result<CascadePreview, ErrorCode> {
        let mut preview = CascadePreview::new(target_entity, target_code);

        match target_entity {
            "world_setting" => Self::plan_world_setting(state, target_code, &mut preview)?,
            "character" => Self::plan_character(state, target_code, &mut preview)?,
            "arc" => Self::plan_arc(state, target_code, &mut preview)?,
            "chapter" => Self::plan_chapter(state, target_code, &mut preview)?,
            "scene" => Self::plan_scene(state, target_code, &mut preview)?,
            "lore_entry" => Self::plan_lore_entry(state, target_code, &mut preview)?,
            "theme" => Self::plan_theme(state, target_code, &mut preview)?,
            "relationship" => Self::plan_relationship(state, target_code, &mut preview)?,
            "scene_character" => Self::plan_scene_character(state, target_code, &mut preview)?,
            "scene_lore_entry" => Self::plan_scene_lore_entry(state, target_code, &mut preview)?,
            "project_genre" => Self::plan_project_genre(state, target_code, &mut preview)?,
            "arc_event_block" => Self::plan_arc_event_block(state, target_code, &mut preview)?,
            "theme_argument" => Self::plan_theme_argument(state, target_code, &mut preview)?,
            "character_distinguishing_mark" => {
                Self::plan_character_distinguishing_mark(state, target_code, &mut preview)?
            }
            "character_memory" => Self::plan_character_memory(state, target_code, &mut preview)?,
            "character_competence" => {
                Self::plan_character_competence(state, target_code, &mut preview)?
            }
            "competence_capability" => {
                Self::plan_competence_capability(state, target_code, &mut preview)?
            }
            "competence_limitation" => {
                Self::plan_competence_limitation(state, target_code, &mut preview)?
            }
            "relationship_role" => Self::plan_relationship_role(state, target_code, &mut preview)?,
            _ => return Err(ErrorCode::EntityNotFound),
        }

        Ok(preview)
    }

    // ------------------------------------------------------------------------
    // World Setting Planner (Q1.1)
    // ------------------------------------------------------------------------
    fn plan_world_setting(
        state: &BusinessFlowData,
        target_code: &str,
        preview: &mut CascadePreview,
    ) -> Result<(), ErrorCode> {
        let all_settings = &state.optional.world_building_phase.world_settings;
        let _root = all_settings
            .iter()
            .find(|ws| ws.code == target_code)
            .ok_or(ErrorCode::EntityNotFound)?;

        // 1. Build complete subtree T of target_code and recursive descendants
        let mut subtree_codes = BTreeSet::new();
        subtree_codes.insert(target_code.to_string());

        let mut changed = true;
        while changed {
            changed = false;
            for ws in all_settings {
                if let Some(parent) = &ws.parent_code.value {
                    if subtree_codes.contains(parent) && !subtree_codes.contains(&ws.code) {
                        subtree_codes.insert(ws.code.clone());
                        changed = true;
                    }
                }
            }
        }

        // 2. Check referencing scenes across chapter_phase and scene_config_phase
        let mut referencing_scenes = BTreeSet::new();

        for arc in &state.optional.chapter_phase.arcs {
            for chap in &arc.chapters {
                for sc in &chap.scenes {
                    if let Some(ws_code) = &sc.world_setting_code.value {
                        if subtree_codes.contains(ws_code) {
                            referencing_scenes.insert(sc.code.clone());
                        }
                    }
                }
            }
        }

        for arc in &state.optional.scene_config_phase.arcs {
            for chap in &arc.chapters {
                for sc in &chap.scenes {
                    if let Some(ws_code) = &sc.world_setting_code.value {
                        if subtree_codes.contains(ws_code) {
                            referencing_scenes.insert(sc.code.clone());
                        }
                    }
                }
            }
        }

        if !referencing_scenes.is_empty() {
            preview.referencing_scenes = referencing_scenes.into_iter().collect();
            for sc_code in &preview.referencing_scenes {
                preview.blocking_references.push(CascadeBlockingRef {
                    blocking_type: "scene_reference".to_string(),
                    referencing_code: sc_code.clone(),
                    reason: format!(
                        "Scene '{sc_code}' is currently referencing a world setting node in target subtree"
                    ),
                });
            }
            return Ok(());
        }

        // 3. Populate entities and lore cascade for subtree T
        for ws in all_settings {
            if subtree_codes.contains(&ws.code) {
                preview.entities_to_delete.push(CascadeEntityRef {
                    entity_type: "world_setting".to_string(),
                    code: ws.code.clone(),
                    display_name: ws.display_name.value.clone(),
                    source: ws.source,
                });
                if let Some(src) = ws.source {
                    if src.is_protected_from_ai_overwrite() {
                        preview.protected_items.push(CascadeProtectedItem {
                            item_type: "world_setting".to_string(),
                            code: ws.code.clone(),
                            source: src,
                            description: format!("Protected world setting '{}'", ws.code),
                        });
                    }
                }

                // Lore entries attached to this world setting node
                for lore in &ws.lore_entries {
                    preview.entities_to_delete.push(CascadeEntityRef {
                        entity_type: "lore_entry".to_string(),
                        code: lore.code.clone(),
                        display_name: lore.display_name.value.clone(),
                        source: lore.source,
                    });
                    if let Some(src) = lore.source {
                        if src.is_protected_from_ai_overwrite() {
                            preview.protected_items.push(CascadeProtectedItem {
                                item_type: "lore_entry".to_string(),
                                code: lore.code.clone(),
                                source: src,
                                description: format!("Protected lore entry '{}'", lore.code),
                            });
                        }
                    }

                    // Links from scenes to this lore entry
                    for arc in &state.optional.scene_config_phase.arcs {
                        for chap in &arc.chapters {
                            for sc in &chap.scenes {
                                for l_link in &sc.lore_entries {
                                    if l_link.lore_entry_code == lore.code {
                                        preview.links_to_delete.push(CascadeLinkRef {
                                            link_type: "scene_lore_entry".to_string(),
                                            parent_code: sc.code.clone(),
                                            item_code: lore.code.clone(),
                                            source: l_link.source,
                                        });
                                        if let Some(src) = l_link.source {
                                            if src.is_protected_from_ai_overwrite() {
                                                preview.protected_items.push(
                                                    CascadeProtectedItem {
                                                        item_type: "scene_lore_entry".to_string(),
                                                        code: format!("{}:{}", sc.code, lore.code),
                                                        source: src,
                                                        description: format!(
                                                            "Protected scene lore link on scene '{}'",
                                                            sc.code
                                                        ),
                                                    },
                                                );
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    // ------------------------------------------------------------------------
    // Character Planner (Q1.2)
    // ------------------------------------------------------------------------
    fn plan_character(
        state: &BusinessFlowData,
        target_code: &str,
        preview: &mut CascadePreview,
    ) -> Result<(), ErrorCode> {
        let char_item = state
            .require
            .character
            .iter()
            .find(|c| c.code == target_code)
            .ok_or(ErrorCode::EntityNotFound)?;

        // 1. Hard check: Character part of any relationship
        for rel in &state.optional.relationship_phase.relationship {
            if rel.character.from_code == target_code || rel.character.to_code == target_code {
                preview.blocking_references.push(CascadeBlockingRef {
                    blocking_type: "relationship_reference".to_string(),
                    referencing_code: rel.code.clone(),
                    reason: format!(
                        "Character '{target_code}' is part of relationship '{}' ({} -> {})",
                        rel.code, rel.character.from_code, rel.character.to_code
                    ),
                });
            }
        }

        if !preview.blocking_references.is_empty() {
            return Ok(());
        }

        // 2. Add character root entity
        preview.entities_to_delete.push(CascadeEntityRef {
            entity_type: "character".to_string(),
            code: target_code.to_string(),
            display_name: char_item.display_name.value.clone(),
            source: char_item.source,
        });
        if let Some(src) = char_item.source {
            if src.is_protected_from_ai_overwrite() {
                preview.protected_items.push(CascadeProtectedItem {
                    item_type: "character".to_string(),
                    code: target_code.to_string(),
                    source: src,
                    description: format!("Protected character '{target_code}'"),
                });
            }
        }

        // 3. Check character appearance, psychological, and life experience profiles
        for app in &state.optional.character_appearance_phase.character {
            if app.code == target_code {
                preview.entities_to_delete.push(CascadeEntityRef {
                    entity_type: "character_appearance".to_string(),
                    code: target_code.to_string(),
                    display_name: None,
                    source: None,
                });
            }
        }

        for psych in &state.optional.character_psychological_phase.character {
            if psych.code == target_code {
                preview.entities_to_delete.push(CascadeEntityRef {
                    entity_type: "character_psychological".to_string(),
                    code: target_code.to_string(),
                    display_name: None,
                    source: None,
                });
            }
        }

        for life in &state.optional.character_life_experience_phase.character {
            if life.code == target_code {
                for mem in &life.memories {
                    let mem_code = mem.code.clone().unwrap_or_else(|| "unnamed".to_string());
                    preview.entities_to_delete.push(CascadeEntityRef {
                        entity_type: "character_memory".to_string(),
                        code: mem_code.clone(),
                        display_name: None,
                        source: mem.source,
                    });
                    if let Some(src) = mem.source {
                        if src.is_protected_from_ai_overwrite() {
                            preview.protected_items.push(CascadeProtectedItem {
                                item_type: "character_memory".to_string(),
                                code: mem_code,
                                source: src,
                                description: format!(
                                    "Protected character memory on character '{target_code}'"
                                ),
                            });
                        }
                    }
                }

                for comp in &life.competences {
                    preview.entities_to_delete.push(CascadeEntityRef {
                        entity_type: "character_competence".to_string(),
                        code: comp.code.clone(),
                        display_name: comp.display_name.value.clone(),
                        source: comp.source,
                    });
                    if let Some(src) = comp.source {
                        if src.is_protected_from_ai_overwrite() {
                            preview.protected_items.push(CascadeProtectedItem {
                                item_type: "character_competence".to_string(),
                                code: comp.code.clone(),
                                source: src,
                                description: format!("Protected character competence '{}'", comp.code),
                            });
                        }
                    }
                }
            }
        }

        // 4. Scene character links
        for arc in &state.optional.scene_config_phase.arcs {
            for chap in &arc.chapters {
                for sc in &chap.scenes {
                    for c_link in &sc.characters {
                        if c_link.character_code == target_code {
                            preview.links_to_delete.push(CascadeLinkRef {
                                link_type: "scene_character".to_string(),
                                parent_code: sc.code.clone(),
                                item_code: target_code.to_string(),
                                source: c_link.source,
                            });
                            if let Some(src) = c_link.source {
                                if src.is_protected_from_ai_overwrite() {
                                    preview.protected_items.push(CascadeProtectedItem {
                                        item_type: "scene_character".to_string(),
                                        code: format!("{}:{}", sc.code, target_code),
                                        source: src,
                                        description: format!(
                                            "Protected scene character link in scene '{}'",
                                            sc.code
                                        ),
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }

        // 5. Theme arguments referencing character
        for arc in &state.optional.overview_phase.arcs {
            for arg in &arc.themes.arguments {
                if arg.character_code.value.as_deref() == Some(target_code) {
                    let arg_code = arg.code.clone().unwrap_or_else(|| "arg".to_string());
                    preview.links_to_delete.push(CascadeLinkRef {
                        link_type: "theme_argument".to_string(),
                        parent_code: arc.code.clone(),
                        item_code: arg_code.clone(),
                        source: arg.source,
                    });
                    if let Some(src) = arg.source {
                        if src.is_protected_from_ai_overwrite() {
                            preview.protected_items.push(CascadeProtectedItem {
                                item_type: "theme_argument".to_string(),
                                code: arg_code,
                                source: src,
                                description: format!(
                                    "Protected theme argument for character '{target_code}'"
                                ),
                            });
                        }
                    }
                }
            }
        }

        Ok(())
    }

    // ------------------------------------------------------------------------
    // Arc Planner (Q1.3)
    // ------------------------------------------------------------------------
    fn plan_arc(
        state: &BusinessFlowData,
        target_code: &str,
        preview: &mut CascadePreview,
    ) -> Result<(), ErrorCode> {
        let arc_item = state
            .optional
            .overview_phase
            .arcs
            .iter()
            .find(|a| a.code == target_code);

        let arc_chap = state
            .optional
            .chapter_phase
            .arcs
            .iter()
            .find(|a| a.code == target_code);

        if arc_item.is_none() && arc_chap.is_none() {
            return Err(ErrorCode::EntityNotFound);
        }

        let arc_source = arc_item.and_then(|a| a.source);
        preview.entities_to_delete.push(CascadeEntityRef {
            entity_type: "arc".to_string(),
            code: target_code.to_string(),
            display_name: arc_item.and_then(|a| a.display_name.value.clone()),
            source: arc_source,
        });

        if let Some(src) = arc_source {
            if src.is_protected_from_ai_overwrite() {
                preview.protected_items.push(CascadeProtectedItem {
                    item_type: "arc".to_string(),
                    code: target_code.to_string(),
                    source: src,
                    description: format!("Protected arc '{target_code}'"),
                });
            }
        }

        // Cascade down to chapters in this arc
        if let Some(ac) = arc_chap {
            for chap in &ac.chapters {
                Self::plan_chapter_internal(state, chap.code.as_str(), preview);
            }
        }

        Ok(())
    }

    // ------------------------------------------------------------------------
    // Chapter Planner
    // ------------------------------------------------------------------------
    fn plan_chapter(
        state: &BusinessFlowData,
        target_code: &str,
        preview: &mut CascadePreview,
    ) -> Result<(), ErrorCode> {
        let found = state
            .optional
            .chapter_phase
            .arcs
            .iter()
            .flat_map(|a| &a.chapters)
            .any(|c| c.code == target_code);

        if !found {
            return Err(ErrorCode::EntityNotFound);
        }

        Self::plan_chapter_internal(state, target_code, preview);
        Ok(())
    }

    fn plan_chapter_internal(
        state: &BusinessFlowData,
        chapter_code: &str,
        preview: &mut CascadePreview,
    ) {
        for arc in &state.optional.chapter_phase.arcs {
            for chap in &arc.chapters {
                if chap.code == chapter_code {
                    preview.entities_to_delete.push(CascadeEntityRef {
                        entity_type: "chapter".to_string(),
                        code: chapter_code.to_string(),
                        display_name: chap.display_name.value.clone(),
                        source: chap.source,
                    });
                    if let Some(src) = chap.source {
                        if src.is_protected_from_ai_overwrite() {
                            preview.protected_items.push(CascadeProtectedItem {
                                item_type: "chapter".to_string(),
                                code: chapter_code.to_string(),
                                source: src,
                                description: format!("Protected chapter '{chapter_code}'"),
                            });
                        }
                    }

                    // Cascade down to scenes in this chapter
                    for sc in &chap.scenes {
                        Self::plan_scene_internal(state, sc.code.as_str(), preview);
                    }
                }
            }
        }
    }

    // ------------------------------------------------------------------------
    // Scene Planner (Q1.4)
    // ------------------------------------------------------------------------
    fn plan_scene(
        state: &BusinessFlowData,
        target_code: &str,
        preview: &mut CascadePreview,
    ) -> Result<(), ErrorCode> {
        let found_chap = state
            .optional
            .chapter_phase
            .arcs
            .iter()
            .flat_map(|a| &a.chapters)
            .flat_map(|c| &c.scenes)
            .any(|s| s.code == target_code);

        let found_cfg = state
            .optional
            .scene_config_phase
            .arcs
            .iter()
            .flat_map(|a| &a.chapters)
            .flat_map(|c| &c.scenes)
            .any(|s| s.code == target_code);

        if !found_chap && !found_cfg {
            return Err(ErrorCode::EntityNotFound);
        }

        Self::plan_scene_internal(state, target_code, preview);
        Ok(())
    }

    fn plan_scene_internal(
        state: &BusinessFlowData,
        scene_code: &str,
        preview: &mut CascadePreview,
    ) {
        // 1. Scene entity itself
        let mut scene_source = None;
        let mut display_name = None;

        for arc in &state.optional.chapter_phase.arcs {
            for chap in &arc.chapters {
                for sc in &chap.scenes {
                    if sc.code == scene_code {
                        scene_source = sc.source;
                        display_name = sc.display_name.value.clone();
                    }
                }
            }
        }

        preview.entities_to_delete.push(CascadeEntityRef {
            entity_type: "scene".to_string(),
            code: scene_code.to_string(),
            display_name,
            source: scene_source,
        });

        if let Some(src) = scene_source {
            if src.is_protected_from_ai_overwrite() {
                preview.protected_items.push(CascadeProtectedItem {
                    item_type: "scene".to_string(),
                    code: scene_code.to_string(),
                    source: src,
                    description: format!("Protected scene '{scene_code}'"),
                });
            }
        }

        // 2. Scene characters and Scene lore links in scene_config_phase
        for arc in &state.optional.scene_config_phase.arcs {
            for chap in &arc.chapters {
                for sc in &chap.scenes {
                    if sc.code == scene_code {
                        for c_link in &sc.characters {
                            preview.links_to_delete.push(CascadeLinkRef {
                                link_type: "scene_character".to_string(),
                                parent_code: scene_code.to_string(),
                                item_code: c_link.character_code.clone(),
                                source: c_link.source,
                            });
                            if let Some(src) = c_link.source {
                                if src.is_protected_from_ai_overwrite() {
                                    preview.protected_items.push(CascadeProtectedItem {
                                        item_type: "scene_character".to_string(),
                                        code: format!("{}:{}", scene_code, c_link.character_code),
                                        source: src,
                                        description: format!(
                                            "Protected character link in scene '{scene_code}'"
                                        ),
                                    });
                                }
                            }
                        }

                        for l_link in &sc.lore_entries {
                            preview.links_to_delete.push(CascadeLinkRef {
                                link_type: "scene_lore_entry".to_string(),
                                parent_code: scene_code.to_string(),
                                item_code: l_link.lore_entry_code.clone(),
                                source: l_link.source,
                            });
                            if let Some(src) = l_link.source {
                                if src.is_protected_from_ai_overwrite() {
                                    preview.protected_items.push(CascadeProtectedItem {
                                        item_type: "scene_lore_entry".to_string(),
                                        code: format!("{}:{}", scene_code, l_link.lore_entry_code),
                                        source: src,
                                        description: format!(
                                            "Protected lore link in scene '{scene_code}'"
                                        ),
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }

        // 3. Dependent character_competences (physically removed)
        for life in &state.optional.character_life_experience_phase.character {
            for comp in &life.competences {
                if comp.scene_code.value.as_deref() == Some(scene_code) {
                    preview.entities_to_delete.push(CascadeEntityRef {
                        entity_type: "character_competence".to_string(),
                        code: comp.code.clone(),
                        display_name: comp.display_name.value.clone(),
                        source: comp.source,
                    });
                    if let Some(src) = comp.source {
                        if src.is_protected_from_ai_overwrite() {
                            preview.protected_items.push(CascadeProtectedItem {
                                item_type: "character_competence".to_string(),
                                code: comp.code.clone(),
                                source: src,
                                description: format!(
                                    "Protected character competence '{}' attached to scene '{scene_code}'",
                                    comp.code
                                ),
                            });
                        }
                    }
                }
            }
        }

        // 4. Dependent relationship_states (physically removed; parent relationship kept)
        for rel in &state.optional.relationship_phase.relationship {
            for st in &rel.character.scenes {
                if st.scene_code == scene_code {
                    preview.links_to_delete.push(CascadeLinkRef {
                        link_type: "relationship_state".to_string(),
                        parent_code: rel.code.clone(),
                        item_code: scene_code.to_string(),
                        source: st.source,
                    });
                    if let Some(src) = st.source {
                        if src.is_protected_from_ai_overwrite() {
                            preview.protected_items.push(CascadeProtectedItem {
                                item_type: "relationship_state".to_string(),
                                code: format!("{}:{}", rel.code, scene_code),
                                source: src,
                                description: format!(
                                    "Protected relationship state in scene '{scene_code}'"
                                ),
                            });
                        }
                    }
                }
            }
        }

        // 5. Dependent character_memories (nullify source_scene_code)
        for life in &state.optional.character_life_experience_phase.character {
            for mem in &life.memories {
                if mem.source_scene_code.value.as_deref() == Some(scene_code) {
                    preview.references_to_nullify.push(CascadeNullifyRef {
                        entity_type: "character_memory".to_string(),
                        owner_code: life.code.clone(),
                        field_name: "source_scene_code".to_string(),
                        previous_value: Some(scene_code.to_string()),
                        source: mem.source,
                    });
                    if let Some(src) = mem.source {
                        if src.is_protected_from_ai_overwrite() {
                            preview.protected_items.push(CascadeProtectedItem {
                                item_type: "character_memory".to_string(),
                                code: format!("{}:memory", life.code),
                                source: src,
                                description: format!(
                                    "Protected character memory on character '{}' unlinked from scene '{scene_code}'",
                                    life.code
                                ),
                            });
                        }
                    }
                }
            }
        }
    }

    // ------------------------------------------------------------------------
    // Lore Entry Planner (Q1.5)
    // ------------------------------------------------------------------------
    fn plan_lore_entry(
        state: &BusinessFlowData,
        target_code: &str,
        preview: &mut CascadePreview,
    ) -> Result<(), ErrorCode> {
        let mut found_lore = None;
        for ws in &state.optional.world_building_phase.world_settings {
            for lore in &ws.lore_entries {
                if lore.code == target_code {
                    found_lore = Some(lore);
                    break;
                }
            }
            if found_lore.is_some() {
                break;
            }
        }

        let lore = found_lore.ok_or(ErrorCode::EntityNotFound)?;

        preview.entities_to_delete.push(CascadeEntityRef {
            entity_type: "lore_entry".to_string(),
            code: target_code.to_string(),
            display_name: lore.display_name.value.clone(),
            source: lore.source,
        });

        if let Some(src) = lore.source {
            if src.is_protected_from_ai_overwrite() {
                preview.protected_items.push(CascadeProtectedItem {
                    item_type: "lore_entry".to_string(),
                    code: target_code.to_string(),
                    source: src,
                    description: format!("Protected lore entry '{target_code}'"),
                });
            }
        }

        // Unlink scene_lore_entries across all scenes
        for arc in &state.optional.scene_config_phase.arcs {
            for chap in &arc.chapters {
                for sc in &chap.scenes {
                    for l_link in &sc.lore_entries {
                        if l_link.lore_entry_code == target_code {
                            preview.links_to_delete.push(CascadeLinkRef {
                                link_type: "scene_lore_entry".to_string(),
                                parent_code: sc.code.clone(),
                                item_code: target_code.to_string(),
                                source: l_link.source,
                            });
                            if let Some(src) = l_link.source {
                                if src.is_protected_from_ai_overwrite() {
                                    preview.protected_items.push(CascadeProtectedItem {
                                        item_type: "scene_lore_entry".to_string(),
                                        code: format!("{}:{}", sc.code, target_code),
                                        source: src,
                                        description: format!(
                                            "Protected scene lore link in scene '{}'",
                                            sc.code
                                        ),
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    // ------------------------------------------------------------------------
    // Theme Planner
    // ------------------------------------------------------------------------
    fn plan_theme(
        state: &BusinessFlowData,
        target_code: &str,
        preview: &mut CascadePreview,
    ) -> Result<(), ErrorCode> {
        let mut found = false;
        for arc in &state.optional.overview_phase.arcs {
            if arc.themes.code.as_deref() == Some(target_code) || arc.code == target_code {
                found = true;
                preview.entities_to_delete.push(CascadeEntityRef {
                    entity_type: "theme".to_string(),
                    code: target_code.to_string(),
                    display_name: arc.themes.question.value.clone(),
                    source: arc.themes.source,
                });
                if let Some(src) = arc.themes.source {
                    if src.is_protected_from_ai_overwrite() {
                        preview.protected_items.push(CascadeProtectedItem {
                            item_type: "theme".to_string(),
                            code: target_code.to_string(),
                            source: src,
                            description: format!("Protected theme '{target_code}'"),
                        });
                    }
                }
            }
        }

        if !found {
            return Err(ErrorCode::EntityNotFound);
        }

        Ok(())
    }

    // ------------------------------------------------------------------------
    // Relationship Planner
    // ------------------------------------------------------------------------
    fn plan_relationship(
        state: &BusinessFlowData,
        target_code: &str,
        preview: &mut CascadePreview,
    ) -> Result<(), ErrorCode> {
        let rel = state
            .optional
            .relationship_phase
            .relationship
            .iter()
            .find(|r| r.code == target_code)
            .ok_or(ErrorCode::EntityNotFound)?;

        preview.entities_to_delete.push(CascadeEntityRef {
            entity_type: "relationship".to_string(),
            code: target_code.to_string(),
            display_name: None,
            source: rel.source,
        });

        if let Some(src) = rel.source {
            if src.is_protected_from_ai_overwrite() {
                preview.protected_items.push(CascadeProtectedItem {
                    item_type: "relationship".to_string(),
                    code: target_code.to_string(),
                    source: src,
                    description: format!("Protected relationship '{target_code}'"),
                });
            }
        }

        Ok(())
    }

    // ------------------------------------------------------------------------
    // Granular link deletion planners
    // ------------------------------------------------------------------------
    fn plan_scene_character(
        state: &BusinessFlowData,
        target_code: &str,
        preview: &mut CascadePreview,
    ) -> Result<(), ErrorCode> {
        let parts: Vec<&str> = target_code.split(':').collect();
        let (scene_code, char_code) = if parts.len() == 2 {
            (parts[0], parts[1])
        } else {
            ("", target_code)
        };

        for arc in &state.optional.scene_config_phase.arcs {
            for chap in &arc.chapters {
                for sc in &chap.scenes {
                    if scene_code.is_empty() || sc.code == scene_code {
                        for c in &sc.characters {
                            if c.character_code == char_code {
                                preview.links_to_delete.push(CascadeLinkRef {
                                    link_type: "scene_character".to_string(),
                                    parent_code: sc.code.clone(),
                                    item_code: char_code.to_string(),
                                    source: c.source,
                                });
                                return Ok(());
                            }
                        }
                    }
                }
            }
        }
        Err(ErrorCode::EntityNotFound)
    }

    fn plan_scene_lore_entry(
        state: &BusinessFlowData,
        target_code: &str,
        preview: &mut CascadePreview,
    ) -> Result<(), ErrorCode> {
        let parts: Vec<&str> = target_code.split(':').collect();
        let (scene_code, lore_code) = if parts.len() == 2 {
            (parts[0], parts[1])
        } else {
            ("", target_code)
        };

        for arc in &state.optional.scene_config_phase.arcs {
            for chap in &arc.chapters {
                for sc in &chap.scenes {
                    if scene_code.is_empty() || sc.code == scene_code {
                        for l in &sc.lore_entries {
                            if l.lore_entry_code == lore_code {
                                preview.links_to_delete.push(CascadeLinkRef {
                                    link_type: "scene_lore_entry".to_string(),
                                    parent_code: sc.code.clone(),
                                    item_code: lore_code.to_string(),
                                    source: l.source,
                                });
                                return Ok(());
                            }
                        }
                    }
                }
            }
        }
        Err(ErrorCode::EntityNotFound)
    }

    fn plan_project_genre(
        state: &BusinessFlowData,
        target_code: &str,
        preview: &mut CascadePreview,
    ) -> Result<(), ErrorCode> {
        for g in &state.require.project.genres {
            if g.code.value.as_deref() == Some(target_code) {
                preview.links_to_delete.push(CascadeLinkRef {
                    link_type: "project_genre".to_string(),
                    parent_code: "project".to_string(),
                    item_code: target_code.to_string(),
                    source: g.code.source,
                });
                return Ok(());
            }
        }
        Err(ErrorCode::EntityNotFound)
    }

    fn plan_arc_event_block(
        state: &BusinessFlowData,
        target_code: &str,
        preview: &mut CascadePreview,
    ) -> Result<(), ErrorCode> {
        for arc in &state.optional.overview_phase.arcs {
            for eb in &arc.motif.event_blocks {
                if eb.code == target_code || eb.event_block_code.value.as_deref() == Some(target_code) {
                    preview.links_to_delete.push(CascadeLinkRef {
                        link_type: "arc_event_block".to_string(),
                        parent_code: arc.code.clone(),
                        item_code: target_code.to_string(),
                        source: eb.source,
                    });
                    return Ok(());
                }
            }
        }
        Err(ErrorCode::EntityNotFound)
    }

    fn plan_theme_argument(
        state: &BusinessFlowData,
        target_code: &str,
        preview: &mut CascadePreview,
    ) -> Result<(), ErrorCode> {
        for arc in &state.optional.overview_phase.arcs {
            for arg in &arc.themes.arguments {
                if arg.code.as_deref() == Some(target_code)
                    || arg.character_code.value.as_deref() == Some(target_code)
                {
                    preview.links_to_delete.push(CascadeLinkRef {
                        link_type: "theme_argument".to_string(),
                        parent_code: arc.code.clone(),
                        item_code: target_code.to_string(),
                        source: arg.source,
                    });
                    return Ok(());
                }
            }
        }
        Err(ErrorCode::EntityNotFound)
    }

    fn plan_character_distinguishing_mark(
        state: &BusinessFlowData,
        target_code: &str,
        preview: &mut CascadePreview,
    ) -> Result<(), ErrorCode> {
        for app in &state.optional.character_appearance_phase.character {
            for mark in &app.distinguishing_marks {
                if mark.code.as_deref() == Some(target_code) {
                    preview.links_to_delete.push(CascadeLinkRef {
                        link_type: "character_distinguishing_mark".to_string(),
                        parent_code: app.code.clone(),
                        item_code: target_code.to_string(),
                        source: mark.source,
                    });
                    return Ok(());
                }
            }
        }
        Err(ErrorCode::EntityNotFound)
    }

    fn plan_character_memory(
        state: &BusinessFlowData,
        target_code: &str,
        preview: &mut CascadePreview,
    ) -> Result<(), ErrorCode> {
        for life in &state.optional.character_life_experience_phase.character {
            for mem in &life.memories {
                if mem.code.as_deref() == Some(target_code) {
                    preview.links_to_delete.push(CascadeLinkRef {
                        link_type: "character_memory".to_string(),
                        parent_code: life.code.clone(),
                        item_code: target_code.to_string(),
                        source: mem.source,
                    });
                    return Ok(());
                }
            }
        }
        Err(ErrorCode::EntityNotFound)
    }

    fn plan_character_competence(
        state: &BusinessFlowData,
        target_code: &str,
        preview: &mut CascadePreview,
    ) -> Result<(), ErrorCode> {
        for life in &state.optional.character_life_experience_phase.character {
            for comp in &life.competences {
                if comp.code == target_code {
                    preview.entities_to_delete.push(CascadeEntityRef {
                        entity_type: "character_competence".to_string(),
                        code: target_code.to_string(),
                        display_name: comp.display_name.value.clone(),
                        source: comp.source,
                    });
                    return Ok(());
                }
            }
        }
        Err(ErrorCode::EntityNotFound)
    }

    fn plan_competence_capability(
        _state: &BusinessFlowData,
        target_code: &str,
        preview: &mut CascadePreview,
    ) -> Result<(), ErrorCode> {
        preview.links_to_delete.push(CascadeLinkRef {
            link_type: "competence_capability".to_string(),
            parent_code: "competence".to_string(),
            item_code: target_code.to_string(),
            source: None,
        });
        Ok(())
    }

    fn plan_competence_limitation(
        _state: &BusinessFlowData,
        target_code: &str,
        preview: &mut CascadePreview,
    ) -> Result<(), ErrorCode> {
        preview.links_to_delete.push(CascadeLinkRef {
            link_type: "competence_limitation".to_string(),
            parent_code: "competence".to_string(),
            item_code: target_code.to_string(),
            source: None,
        });
        Ok(())
    }

    fn plan_relationship_role(
        state: &BusinessFlowData,
        target_code: &str,
        preview: &mut CascadePreview,
    ) -> Result<(), ErrorCode> {
        for rel in &state.optional.relationship_phase.relationship {
            for role in &rel.character.roles {
                if role.category_code.value.as_deref() == Some(target_code) {
                    preview.links_to_delete.push(CascadeLinkRef {
                        link_type: "relationship_role".to_string(),
                        parent_code: rel.code.clone(),
                        item_code: target_code.to_string(),
                        source: role.source,
                    });
                    return Ok(());
                }
            }
        }
        Err(ErrorCode::EntityNotFound)
    }
}

// ============================================================================
// Cascade Execution Function
// ============================================================================

/// Executes a cascade delete following the 9 mandatory safeguards (`be-core.md` §5.4).
///
/// Returns:
/// - `CascadeOutcome::Rejected` if references block deletion or entity not found.
/// - `CascadeOutcome::PendingConfirmation` if AI caller encounters protected elements.
/// - `CascadeOutcome::Applied` with mutated working state clone if fully validated.
pub fn execute_delete(
    state: &BusinessFlowData,
    caller: CallerCategory,
    target_entity: &str,
    target_code: &str,
    _declared_source: Option<ProvenanceSource>,
) -> CascadeOutcome {
    // Step 1: Compute full transitive impact preview
    let preview = match CascadePlanner::plan(state, target_entity, target_code) {
        Ok(p) => p,
        Err(ErrorCode::EntityNotFound) => {
            return CascadeOutcome::Rejected {
                error: "target_code_not_found".to_string(),
                referencing_scenes: Vec::new(),
                details: None,
            };
        }
        Err(err) => {
            return CascadeOutcome::Rejected {
                error: format!("{:?}", err).to_lowercase(),
                referencing_scenes: Vec::new(),
                details: None,
            };
        }
    };

    // Step 2: Reference check: If mandatory references block deletion, reject immediately
    if preview.is_blocked() {
        return CascadeOutcome::Rejected {
            error: "referenced_entity_in_use".to_string(),
            referencing_scenes: preview.referencing_scenes.clone(),
            details: Some(serde_json::json!({
                "blocking_references": preview.blocking_references,
            })),
        };
    }

    // Step 3 & 6: Protected Provenance Check
    // If caller is AI and there are protected items in target or descendants: create pending proposal
    if caller.is_ai() && preview.has_protected_items() {
        let proposal = PendingProposal {
            patch_id: format!("patch_del_{}_{}", target_entity, target_code),
            operation: "delete".to_string(),
            target_type: target_entity.to_string(),
            target_code: target_code.to_string(),
            field_name: None,
            previous_value: None,
            new_value: None,
            status: "pending".to_string(),
        };

        return CascadeOutcome::PendingConfirmation { preview, proposal };
    }

    // Step 7 & 8: Atomic physical deletion on clone of BusinessFlowData
    let mut clone = state.clone();
    apply_preview_mutations(&mut clone, &preview);

    CascadeOutcome::Applied {
        preview,
        mutated_state: Box::new(clone),
    }
}

/// Helper executing deletions in-place on a mutable `BusinessFlowData`.
pub fn execute_delete_mut(
    state: &mut BusinessFlowData,
    caller: CallerCategory,
    target_entity: &str,
    target_code: &str,
    declared_source: Option<ProvenanceSource>,
) -> CascadeOutcome {
    let outcome = execute_delete(state, caller, target_entity, target_code, declared_source);
    if let CascadeOutcome::Applied { ref mutated_state, .. } = outcome {
        *state = (**mutated_state).clone();
    }
    outcome
}

/// Applies all deletions, link removals, and reference nullifications from preview onto state.
fn apply_preview_mutations(state: &mut BusinessFlowData, preview: &CascadePreview) {
    let entity_codes_to_delete: HashSet<(String, String)> = preview
        .entities_to_delete
        .iter()
        .map(|e| (e.entity_type.clone(), e.code.clone()))
        .collect();

    let links_to_delete: HashSet<(String, String, String)> = preview
        .links_to_delete
        .iter()
        .map(|l| (l.link_type.clone(), l.parent_code.clone(), l.item_code.clone()))
        .collect();

    // 1. Arc deletions
    state
        .optional
        .overview_phase
        .arcs
        .retain(|a| !entity_codes_to_delete.contains(&("arc".to_string(), a.code.clone())));
    state
        .optional
        .chapter_phase
        .arcs
        .retain(|a| !entity_codes_to_delete.contains(&("arc".to_string(), a.code.clone())));
    state
        .optional
        .scene_config_phase
        .arcs
        .retain(|a| !entity_codes_to_delete.contains(&("arc".to_string(), a.code.clone())));

    // 2. Chapter deletions
    for arc in &mut state.optional.chapter_phase.arcs {
        arc.chapters.retain(|c| {
            !entity_codes_to_delete.contains(&("chapter".to_string(), c.code.clone()))
        });
    }
    for arc in &mut state.optional.scene_config_phase.arcs {
        arc.chapters.retain(|c| {
            !entity_codes_to_delete.contains(&("chapter".to_string(), c.code.clone()))
        });
    }

    // 3. Scene deletions
    for arc in &mut state.optional.chapter_phase.arcs {
        for chap in &mut arc.chapters {
            chap.scenes.retain(|s| {
                !entity_codes_to_delete.contains(&("scene".to_string(), s.code.clone()))
            });
        }
    }
    for arc in &mut state.optional.scene_config_phase.arcs {
        for chap in &mut arc.chapters {
            chap.scenes.retain(|s| {
                !entity_codes_to_delete.contains(&("scene".to_string(), s.code.clone()))
            });
        }
    }

    // 4. World Setting deletions & attached lore entries
    state.optional.world_building_phase.world_settings.retain(|ws| {
        !entity_codes_to_delete.contains(&("world_setting".to_string(), ws.code.clone()))
    });

    for ws in &mut state.optional.world_building_phase.world_settings {
        ws.lore_entries.retain(|lore| {
            !entity_codes_to_delete.contains(&("lore_entry".to_string(), lore.code.clone()))
        });
    }

    // 5. Character deletions & profiles
    state.require.character.retain(|c| {
        !entity_codes_to_delete.contains(&("character".to_string(), c.code.clone()))
    });
    state
        .optional
        .character_appearance_phase
        .character
        .retain(|c| !entity_codes_to_delete.contains(&("character".to_string(), c.code.clone())));
    state
        .optional
        .character_psychological_phase
        .character
        .retain(|c| !entity_codes_to_delete.contains(&("character".to_string(), c.code.clone())));
    state
        .optional
        .character_life_experience_phase
        .character
        .retain(|c| !entity_codes_to_delete.contains(&("character".to_string(), c.code.clone())));

    // Competence deletions within character profiles
    for life in &mut state.optional.character_life_experience_phase.character {
        life.competences.retain(|comp| {
            !entity_codes_to_delete.contains(&("character_competence".to_string(), comp.code.clone()))
        });
        life.memories.retain(|mem| {
            if let Some(code) = &mem.code {
                !entity_codes_to_delete.contains(&("character_memory".to_string(), code.clone()))
            } else {
                true
            }
        });
    }

    // 6. Relationship deletions & states
    state.optional.relationship_phase.relationship.retain(|r| {
        !entity_codes_to_delete.contains(&("relationship".to_string(), r.code.clone()))
    });

    for rel in &mut state.optional.relationship_phase.relationship {
        rel.character.scenes.retain(|st| {
            !links_to_delete.contains(&(
                "relationship_state".to_string(),
                rel.code.clone(),
                st.scene_code.clone(),
            ))
        });
        rel.character.roles.retain(|role| {
            if let Some(cat) = &role.category_code.value {
                !links_to_delete.contains(&(
                    "relationship_role".to_string(),
                    rel.code.clone(),
                    cat.clone(),
                ))
            } else {
                true
            }
        });
    }

    // 7. Scene links (characters and lore entries)
    for arc in &mut state.optional.scene_config_phase.arcs {
        for chap in &mut arc.chapters {
            for sc in &mut chap.scenes {
                sc.characters.retain(|c| {
                    !links_to_delete.contains(&(
                        "scene_character".to_string(),
                        sc.code.clone(),
                        c.character_code.clone(),
                    ))
                });
                sc.lore_entries.retain(|l| {
                    !links_to_delete.contains(&(
                        "scene_lore_entry".to_string(),
                        sc.code.clone(),
                        l.lore_entry_code.clone(),
                    ))
                });
            }
        }
    }

    // 8. Theme arguments and themes
    for arc in &mut state.optional.overview_phase.arcs {
        arc.themes.arguments.retain(|arg| {
            if let Some(code) = &arg.code {
                !links_to_delete.contains(&("theme_argument".to_string(), arc.code.clone(), code.clone()))
            } else if let Some(char_code) = &arg.character_code.value {
                !links_to_delete.contains(&(
                    "theme_argument".to_string(),
                    arc.code.clone(),
                    char_code.clone(),
                ))
            } else {
                true
            }
        });

        for eb in &mut arc.motif.event_blocks {
            if links_to_delete.contains(&(
                "arc_event_block".to_string(),
                arc.code.clone(),
                eb.code.clone(),
            )) {
                // will be handled below
            }
        }
        arc.motif.event_blocks.retain(|eb| {
            !links_to_delete.contains(&(
                "arc_event_block".to_string(),
                arc.code.clone(),
                eb.code.clone(),
            ))
        });
    }

    // 9. Project genres
    state.require.project.genres.retain(|g| {
        if let Some(code) = &g.code.value {
            !links_to_delete.contains(&(
                "project_genre".to_string(),
                "project".to_string(),
                code.clone(),
            ))
        } else {
            true
        }
    });

    // 10. Reference nullifications (e.g. character_memories.source_scene_code = null)
    for null_ref in &preview.references_to_nullify {
        if null_ref.entity_type == "character_memory" && null_ref.field_name == "source_scene_code" {
            for life in &mut state.optional.character_life_experience_phase.character {
                if life.code == null_ref.owner_code {
                    for mem in &mut life.memories {
                        if mem.source_scene_code.value.as_ref() == null_ref.previous_value.as_ref() {
                            mem.source_scene_code.value = None;
                            mem.source_scene_code.source = None;
                        }
                    }
                }
            }
        }
    }
}
