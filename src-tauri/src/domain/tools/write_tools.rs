//! Write Tools Implementation & Two-Phase Per-Field Execution (`REQ-TOOLS-001`, `REQ-TOOLS-002`, `REQ-TOOLS-004`, `be-core.md` §5.3).
//!
//! Enforces:
//! 1. Registry authorization gating: Caller category checked before execution; AI caller blocked
//!    from invoking UI/CLI-only actions (`ErrorCode::UnauthorizedCaller`).
//! 2. Two-phase per-field evaluation for multi-field mutations (`SourceParamMode::MapDeclaredSources`):
//!    - Independent validation of each value parameter against `declared_sources`.
//!    - Missing/invalid declared source on a field rejects only that field (`missing_or_invalid_declared_source`),
//!      allowing independent valid fields to apply (`phase3_sprint5_deep2-F1`, `P3S5-P`).
//!    - Protected provenance preservation: AI callers attempting to overwrite fields with
//!      `source = user` or `user_edited` create a pending proposal patch (`pending_confirmation`)
//!      instead of mutating working state (`REQ-TOOLS-004`).
//! 3. Deletion routing to `cascade_delete::execute_delete` with full Q1 cascade preview.
//! 4. Pure execution strictly on clones of `WorkingState` (`BusinessFlowData`), preserving immutable baseline.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::domain::business_flow::{
    ArcEventBlockItem, ArcMotifSection, ArcThemeSection, BusinessFlowData,
    CharacterAppearanceItem, CharacterCompetenceItem, CharacterLifeExperienceItem,
    CharacterPsychologicalItem,
    CharacterMemoryItem, ChapterPhaseArcItem, ChapterPhaseChapterItem,
    ChapterPhaseSceneItem, LoreEntryItem, OverviewArcItem, ProvenanceSource,
    RelationshipCharacterPairSection, RelationshipMotifSection, RelationshipPhaseItem,
    RelationshipSceneStateItem, RequireCharacterItem, RequireGenreItem,
    SceneCharacterLink, SceneConfigArcItem, SceneConfigSceneItem,
    SceneLoreEntryLink, ThemeArgumentItem, TrackedField, WorldSettingItem,
};
use crate::domain::cascade_delete::{
    execute_delete, CascadeOutcome, CascadePreview, PendingProposal,
};
use crate::domain::tools::registry::{
    AllowedCaller, CallerCategory, SourceParamMode, ToolMetadata, ToolRegistry, ToolType,
};
use crate::error::ErrorCode;

/// Working state guard providing read-only access to baseline `BusinessFlowData` during tool execution.
#[derive(Debug, Clone)]
pub struct ExecutionGuard<'a> {
    pub working_state: &'a BusinessFlowData,
    pub draft_session_id: Option<String>,
}

impl<'a> ExecutionGuard<'a> {
    pub fn new(working_state: &'a BusinessFlowData) -> Self {
        Self {
            working_state,
            draft_session_id: None,
        }
    }

    pub fn with_session(
        working_state: &'a BusinessFlowData,
        draft_session_id: impl Into<String>,
    ) -> Self {
        Self {
            working_state,
            draft_session_id: Some(draft_session_id.into()),
        }
    }
}

/// Incoming tool call request across Tauri / CLI / Harness boundaries.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub name: String,
    pub caller: CallerCategory,
    pub parameters: serde_json::Value,
    #[serde(default)]
    pub declared_sources: Option<BTreeMap<String, ProvenanceSource>>,
    #[serde(default)]
    pub declared_source: Option<ProvenanceSource>,
}

impl ToolCall {
    pub fn new(
        name: impl Into<String>,
        caller: CallerCategory,
        parameters: serde_json::Value,
    ) -> Self {
        Self {
            name: name.into(),
            caller,
            parameters,
            declared_sources: None,
            declared_source: None,
        }
    }

    pub fn with_sources(
        name: impl Into<String>,
        caller: CallerCategory,
        parameters: serde_json::Value,
        declared_sources: BTreeMap<String, ProvenanceSource>,
    ) -> Self {
        Self {
            name: name.into(),
            caller,
            parameters,
            declared_sources: Some(declared_sources),
            declared_source: None,
        }
    }

    pub fn with_source(
        name: impl Into<String>,
        caller: CallerCategory,
        parameters: serde_json::Value,
        declared_source: ProvenanceSource,
    ) -> Self {
        Self {
            name: name.into(),
            caller,
            parameters,
            declared_sources: None,
            declared_source: Some(declared_source),
        }
    }
}

/// Evaluation result for an individual field in a multi-field tool call (`be-core.md` §5.3.1).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FieldResult {
    pub field: String,
    pub status: String, // "applied", "pending_confirmation", "rejected"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub candidates: Option<Vec<String>>,
}

/// Mutation and proposal evidence authenticated by the backend during execution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthenticatedEffect {
    pub mutated_state: Option<Box<BusinessFlowData>>,
    pub pending_patches: Vec<PendingProposal>,
    pub cascade_preview: Option<CascadePreview>,
    pub has_mutations: bool,
}

/// Final outcome returned by `ToolExecutor::execute` (`be-core.md` §5.3.1 Step 3).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolOutcome {
    pub status: String, // "applied", "pending_confirmation", "rejected"
    pub results: Vec<FieldResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authenticated_effect: Option<AuthenticatedEffect>,
}

impl ToolOutcome {
    pub fn is_applied(&self) -> bool {
        self.status == "applied"
    }

    pub fn is_pending(&self) -> bool {
        self.status == "pending_confirmation"
    }

    pub fn is_rejected(&self) -> bool {
        self.status == "rejected"
    }

    pub fn mutated_state(&self) -> Option<&BusinessFlowData> {
        self.authenticated_effect
            .as_ref()
            .and_then(|e| e.mutated_state.as_deref())
    }

    pub fn has_mutations(&self) -> bool {
        self.authenticated_effect
            .as_ref()
            .map(|e| e.has_mutations)
            .unwrap_or(false)
    }

    pub fn pending_patches(&self) -> &[PendingProposal] {
        self.authenticated_effect
            .as_ref()
            .map(|e| e.pending_patches.as_slice())
            .unwrap_or(&[])
    }

    pub fn cascade_preview(&self) -> Option<&CascadePreview> {
        self.authenticated_effect
            .as_ref()
            .and_then(|e| e.cascade_preview.as_ref())
    }

    pub fn rejected(error: impl Into<String>) -> Self {
        let err_str = error.into();
        Self {
            status: "rejected".to_string(),
            results: vec![FieldResult {
                field: "call".to_string(),
                status: "rejected".to_string(),
                patch_id: None,
                error: Some(err_str),
                candidates: None,
            }],
            authenticated_effect: None,
        }
    }
}

// ============================================================================
// Tool Executor Engine
// ============================================================================

/// Tool executor enforcing caller authorization, declared_sources validation,
/// protected field preservation, and cascade deletions.
pub struct ToolExecutor {
    registry: ToolRegistry,
}

impl Default for ToolExecutor {
    fn default() -> Self {
        Self::new(ToolRegistry::new())
    }
}

impl ToolExecutor {
    pub fn new(mut registry: ToolRegistry) -> Self {
        register_all_write_tools(&mut registry);
        Self { registry }
    }

    /// Associated function allowing execution without holding an explicit instance.
    pub fn execute(call: &ToolCall, guard: &ExecutionGuard) -> ToolOutcome {
        let executor = Self::default();
        executor.execute_call(call, guard)
    }

    /// Executes the tool call against the provided working state guard.
    pub fn execute_call(&self, call: &ToolCall, guard: &ExecutionGuard) -> ToolOutcome {
        // 1. Authorize caller against registry
        let meta = match self.registry.authorize(&call.name, call.caller) {
            Ok(m) => m,
            Err(ErrorCode::UnauthorizedCaller) => {
                return ToolOutcome::rejected("unauthorized_caller");
            }
            Err(ErrorCode::EntityNotFound) => {
                return ToolOutcome::rejected("tool_not_found");
            }
            Err(err) => {
                return ToolOutcome::rejected(format!("{:?}", err).to_lowercase());
            }
        };

        // 2. Route delete tools
        if meta.tool_type == ToolType::Delete || call.name.starts_with("delete_") {
            return self.execute_delete_tool(call, guard);
        }

        // 3. Route mutation tools (add_*, set_*)
        self.execute_mutation_tool(call, guard)
    }

    // ------------------------------------------------------------------------
    // Delete Tool Dispatch
    // ------------------------------------------------------------------------
    fn execute_delete_tool(&self, call: &ToolCall, guard: &ExecutionGuard) -> ToolOutcome {
        let (entity_type, target_code) = match extract_delete_target(&call.name, &call.parameters) {
            Some(t) => t,
            None => {
                return ToolOutcome::rejected("missing_target_code");
            }
        };

        let declared_source = extract_scalar_declared_source(call);
        if call.caller.is_ai() && declared_source.is_none() {
            return ToolOutcome {
                status: "rejected".to_string(),
                results: vec![FieldResult {
                    field: "declared_source".to_string(),
                    status: "rejected".to_string(),
                    patch_id: None,
                    error: Some("missing_or_invalid_declared_source".to_string()),
                    candidates: None,
                }],
                authenticated_effect: None,
            };
        }

        let cascade_outcome = execute_delete(
            guard.working_state,
            call.caller,
            &entity_type,
            &target_code,
            declared_source,
        );

        match cascade_outcome {
            CascadeOutcome::Applied {
                preview,
                mutated_state,
            } => ToolOutcome {
                status: "applied".to_string(),
                results: vec![FieldResult {
                    field: target_code.clone(),
                    status: "applied".to_string(),
                    patch_id: None,
                    error: None,
                    candidates: None,
                }],
                authenticated_effect: Some(AuthenticatedEffect {
                    mutated_state: Some(mutated_state),
                    pending_patches: Vec::new(),
                    cascade_preview: Some(preview),
                    has_mutations: true,
                }),
            },
            CascadeOutcome::PendingConfirmation { preview, proposal } => ToolOutcome {
                status: "pending_confirmation".to_string(),
                results: vec![FieldResult {
                    field: target_code.clone(),
                    status: "pending_confirmation".to_string(),
                    patch_id: Some(proposal.patch_id.clone()),
                    error: None,
                    candidates: None,
                }],
                authenticated_effect: Some(AuthenticatedEffect {
                    mutated_state: None,
                    pending_patches: vec![proposal],
                    cascade_preview: Some(preview),
                    has_mutations: false,
                }),
            },
            CascadeOutcome::Rejected {
                error,
                referencing_scenes,
                ..
            } => {
                let mut candidates = None;
                if !referencing_scenes.is_empty() {
                    candidates = Some(referencing_scenes);
                }
                ToolOutcome {
                    status: "rejected".to_string(),
                    results: vec![FieldResult {
                        field: target_code.clone(),
                        status: "rejected".to_string(),
                        patch_id: None,
                        error: Some(error),
                        candidates,
                    }],
                    authenticated_effect: None,
                }
            }
        }
    }

    // ------------------------------------------------------------------------
    // Mutation Tool Dispatch (add_*, set_*)
    // ------------------------------------------------------------------------
    fn execute_mutation_tool(&self, call: &ToolCall, guard: &ExecutionGuard) -> ToolOutcome {
        let mut candidate_state = guard.working_state.clone();
        let declared_sources = extract_declared_sources(call);

        let mut results = Vec::new();
        let mut pending_patches = Vec::new();

        let tool_name = call.name.as_str();

        match tool_name {
            "set_character" => {
                self.handle_set_character(
                    call,
                    &declared_sources,
                    guard.working_state,
                    &mut candidate_state,
                    &mut results,
                    &mut pending_patches,
                );
            }
            "add_character" => {
                self.handle_add_character(
                    call,
                    &declared_sources,
                    guard.working_state,
                    &mut candidate_state,
                    &mut results,
                    &mut pending_patches,
                );
            }
            "set_project" => {
                self.handle_set_project(
                    call,
                    &declared_sources,
                    guard.working_state,
                    &mut candidate_state,
                    &mut results,
                    &mut pending_patches,
                );
            }
            "add_project" => {
                self.handle_add_project(
                    call,
                    &declared_sources,
                    guard.working_state,
                    &mut candidate_state,
                    &mut results,
                    &mut pending_patches,
                );
            }
            "add_arc" => {
                self.handle_add_arc(
                    call,
                    &declared_sources,
                    guard.working_state,
                    &mut candidate_state,
                    &mut results,
                    &mut pending_patches,
                );
            }
            "set_arc" => {
                self.handle_set_arc(
                    call,
                    &declared_sources,
                    guard.working_state,
                    &mut candidate_state,
                    &mut results,
                    &mut pending_patches,
                );
            }
            "add_chapter" => {
                self.handle_add_chapter(
                    call,
                    &declared_sources,
                    guard.working_state,
                    &mut candidate_state,
                    &mut results,
                    &mut pending_patches,
                );
            }
            "set_chapter" => {
                self.handle_set_chapter(
                    call,
                    &declared_sources,
                    guard.working_state,
                    &mut candidate_state,
                    &mut results,
                    &mut pending_patches,
                );
            }
            "add_scene" => {
                self.handle_add_scene(
                    call,
                    &declared_sources,
                    guard.working_state,
                    &mut candidate_state,
                    &mut results,
                    &mut pending_patches,
                );
            }
            "set_scene" => {
                self.handle_set_scene(
                    call,
                    &declared_sources,
                    guard.working_state,
                    &mut candidate_state,
                    &mut results,
                    &mut pending_patches,
                );
            }
            "set_scene_config" => {
                self.handle_set_scene_config(
                    call,
                    &declared_sources,
                    guard.working_state,
                    &mut candidate_state,
                    &mut results,
                    &mut pending_patches,
                );
            }
            "add_world_setting" => {
                self.handle_add_world_setting(
                    call,
                    &declared_sources,
                    guard.working_state,
                    &mut candidate_state,
                    &mut results,
                    &mut pending_patches,
                );
            }
            "set_world_setting" => {
                self.handle_set_world_setting(
                    call,
                    &declared_sources,
                    guard.working_state,
                    &mut candidate_state,
                    &mut results,
                    &mut pending_patches,
                );
            }
            "add_lore_entry" => {
                self.handle_add_lore_entry(
                    call,
                    &declared_sources,
                    guard.working_state,
                    &mut candidate_state,
                    &mut results,
                    &mut pending_patches,
                );
            }
            "set_lore_entry" => {
                self.handle_set_lore_entry(
                    call,
                    &declared_sources,
                    guard.working_state,
                    &mut candidate_state,
                    &mut results,
                    &mut pending_patches,
                );
            }
            "add_relationship" => {
                self.handle_add_relationship(
                    call,
                    &declared_sources,
                    guard.working_state,
                    &mut candidate_state,
                    &mut results,
                    &mut pending_patches,
                );
            }
            "set_relationship_state" => {
                self.handle_set_relationship_state(
                    call,
                    &declared_sources,
                    guard.working_state,
                    &mut candidate_state,
                    &mut results,
                    &mut pending_patches,
                );
            }
            "add_scene_character" => {
                self.handle_add_scene_character(
                    call,
                    &declared_sources,
                    guard.working_state,
                    &mut candidate_state,
                    &mut results,
                    &mut pending_patches,
                );
            }
            "add_scene_lore_entry" => {
                self.handle_add_scene_lore_entry(
                    call,
                    &declared_sources,
                    guard.working_state,
                    &mut candidate_state,
                    &mut results,
                    &mut pending_patches,
                );
            }
            "set_arc_motif" => {
                self.handle_set_arc_motif(
                    call,
                    &declared_sources,
                    &mut candidate_state,
                    &mut results,
                );
            }
            "add_project_genre" => {
                self.handle_add_project_genre(
                    call,
                    &declared_sources,
                    guard.working_state,
                    &mut candidate_state,
                    &mut results,
                    &mut pending_patches,
                );
            }
            "add_arc_event_block" => {
                self.handle_add_arc_event_block(
                    call,
                    &declared_sources,
                    guard.working_state,
                    &mut candidate_state,
                    &mut results,
                    &mut pending_patches,
                );
            }
            "add_character_memory" => {
                self.handle_add_character_memory(
                    call,
                    &declared_sources,
                    guard.working_state,
                    &mut candidate_state,
                    &mut results,
                    &mut pending_patches,
                );
            }
            "add_character_appearance" | "set_character_appearance" => {
                self.handle_character_appearance(
                    call,
                    &declared_sources,
                    &mut candidate_state,
                    &mut results,
                );
            }
            "add_character_psychological" | "set_character_psychological" => {
                self.handle_character_psychological(
                    call,
                    &declared_sources,
                    &mut candidate_state,
                    &mut results,
                );
            }
            "add_character_competence" => {
                self.handle_add_character_competence(
                    call,
                    &declared_sources,
                    guard.working_state,
                    &mut candidate_state,
                    &mut results,
                    &mut pending_patches,
                );
            }
            "save_approved_outline" => {
                self.handle_save_approved_outline(
                    call,
                    &declared_sources,
                    &mut candidate_state,
                    &mut results,
                );
            }
            "add_theme" => {
                self.handle_add_theme(
                    call,
                    &declared_sources,
                    guard.working_state,
                    &mut candidate_state,
                    &mut results,
                    &mut pending_patches,
                );
            }
            "add_theme_argument" => {
                self.handle_add_theme_argument(
                    call,
                    &declared_sources,
                    guard.working_state,
                    &mut candidate_state,
                    &mut results,
                    &mut pending_patches,
                );
            }
            _ => {
                return ToolOutcome::rejected(format!("unimplemented_mutation_tool: {}", call.name));
            }
        }

        if results.is_empty() {
            return ToolOutcome::rejected("no_supported_fields");
        }

        // Step 3: Compute aggregate status
        let has_applied = results.iter().any(|r| r.status == "applied");
        let has_pending = results.iter().any(|r| r.status == "pending_confirmation");

        if has_applied {
            ToolOutcome {
                status: "applied".to_string(),
                results,
                authenticated_effect: Some(AuthenticatedEffect {
                    mutated_state: Some(Box::new(candidate_state)),
                    pending_patches,
                    cascade_preview: None,
                    has_mutations: true,
                }),
            }
        } else if has_pending {
            ToolOutcome {
                status: "pending_confirmation".to_string(),
                results,
                authenticated_effect: Some(AuthenticatedEffect {
                    mutated_state: None,
                    pending_patches,
                    cascade_preview: None,
                    has_mutations: false,
                }),
            }
        } else {
            ToolOutcome {
                status: "rejected".to_string(),
                results,
                authenticated_effect: None,
            }
        }
    }

    // ------------------------------------------------------------------------
    // Handlers for specific mutation tools
    // ------------------------------------------------------------------------

    fn handle_set_character(
        &self,
        call: &ToolCall,
        declared_sources: &BTreeMap<String, ProvenanceSource>,
        baseline: &BusinessFlowData,
        candidate: &mut BusinessFlowData,
        results: &mut Vec<FieldResult>,
        pending_patches: &mut Vec<PendingProposal>,
    ) {
        let char_code = call
            .parameters
            .get("code")
            .or_else(|| call.parameters.get("character_code"))
            .and_then(|v| v.as_str());

        let target_code = match char_code {
            Some(c) => c,
            None => {
                results.push(FieldResult {
                    field: "code".to_string(),
                    status: "rejected".to_string(),
                    patch_id: None,
                    error: Some("missing_target_code".to_string()),
                    candidates: None,
                });
                return;
            }
        };

        let baseline_char = match baseline.require.character.iter().find(|c| c.code == target_code) {
            Some(c) => c,
            None => {
                results.push(FieldResult {
                    field: "code".to_string(),
                    status: "rejected".to_string(),
                    patch_id: None,
                    error: Some("target_code_not_found".to_string()),
                    candidates: None,
                });
                return;
            }
        };

        let candidate_char = candidate
            .require
            .character
            .iter_mut()
            .find(|c| c.code == target_code)
            .unwrap();

        // Evaluate supported fields
        let fields = [
            ("display_name", &baseline_char.display_name, &mut candidate_char.display_name),
            ("nickname", &baseline_char.nickname, &mut candidate_char.nickname),
            ("biological_sex", &baseline_char.biological_sex, &mut candidate_char.biological_sex),
            ("motif_code", &baseline_char.motif_code, &mut candidate_char.motif_code),
            ("role", &baseline_char.role, &mut candidate_char.role),
            ("notes", &baseline_char.notes, &mut candidate_char.notes),
        ];

        for (field_name, base_field, cand_field) in fields {
            if let Some(param_val) = call.parameters.get(field_name) {
                if param_val.is_null() {
                    continue;
                }
                let new_str = match param_val.as_str() {
                    Some(s) => s.to_string(),
                    None => {
                        results.push(FieldResult {
                            field: field_name.to_string(),
                            status: "rejected".to_string(),
                            patch_id: None,
                            error: Some("invalid_field_value".to_string()),
                            candidates: None,
                        });
                        continue;
                    }
                };

                let res = evaluate_single_field(
                    call.caller,
                    "character",
                    target_code,
                    field_name,
                    new_str,
                    base_field,
                    cand_field,
                    declared_sources,
                    pending_patches,
                );
                results.push(res);
            }
        }
    }

    fn handle_add_character(
        &self,
        call: &ToolCall,
        declared_sources: &BTreeMap<String, ProvenanceSource>,
        _baseline: &BusinessFlowData,
        candidate: &mut BusinessFlowData,
        results: &mut Vec<FieldResult>,
        _pending_patches: &mut Vec<PendingProposal>,
    ) {
        let code = call
            .parameters
            .get("code")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| {
                let disp = call
                    .parameters
                    .get("display_name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("char");
                format!("{}_{}", disp.to_lowercase().replace(' ', "_"), candidate.require.character.len() + 1)
            });

        if candidate.require.character.iter().any(|c| c.code == code) {
            results.push(FieldResult {
                field: "code".to_string(),
                status: "rejected".to_string(),
                patch_id: None,
                error: Some("code_already_exists".to_string()),
                candidates: None,
            });
            return;
        }

        let mut item = RequireCharacterItem {
            code: code.clone(),
            display_name: TrackedField::unset(),
            nickname: TrackedField::unset(),
            biological_sex: TrackedField::unset(),
            motif_code: TrackedField::unset(),
            role: TrackedField::unset(),
            notes: TrackedField::unset(),
            reused_library_character: false,
            source: Some(ProvenanceSource::User),
        };

        // Populate fields
        let fields = [
            ("display_name", &mut item.display_name),
            ("nickname", &mut item.nickname),
            ("biological_sex", &mut item.biological_sex),
            ("motif_code", &mut item.motif_code),
            ("role", &mut item.role),
            ("notes", &mut item.notes),
        ];

        let mut any_applied = false;

        for (f_name, target_field) in fields {
            if let Some(val) = call.parameters.get(f_name) {
                if let Some(s) = val.as_str() {
                    let declared = if call.caller.is_ai() {
                        match declared_sources.get(f_name) {
                            Some(src) if matches!(src, ProvenanceSource::User | ProvenanceSource::AiGenerated) => {
                                *src
                            }
                            _ => {
                                results.push(FieldResult {
                                    field: f_name.to_string(),
                                    status: "rejected".to_string(),
                                    patch_id: None,
                                    error: Some("missing_or_invalid_declared_source".to_string()),
                                    candidates: None,
                                });
                                continue;
                            }
                        }
                    } else {
                        *declared_sources.get(f_name).unwrap_or(&ProvenanceSource::User)
                    };

                    target_field.value = Some(s.to_string());
                    target_field.source = Some(declared);
                    results.push(FieldResult {
                        field: f_name.to_string(),
                        status: "applied".to_string(),
                        patch_id: None,
                        error: None,
                        candidates: None,
                    });
                    any_applied = true;
                }
            }
        }

        if any_applied {
            candidate.require.character.push(item);
        }
    }

    fn handle_set_project(
        &self,
        call: &ToolCall,
        declared_sources: &BTreeMap<String, ProvenanceSource>,
        baseline: &BusinessFlowData,
        candidate: &mut BusinessFlowData,
        results: &mut Vec<FieldResult>,
        pending_patches: &mut Vec<PendingProposal>,
    ) {
        let base_proj = &baseline.require.project;
        let cand_proj = &mut candidate.require.project;
        let Some(parameters) = call.parameters.as_object() else {
            results.push(field_rejection("call", "expected_object"));
            return;
        };
        for name in parameters.keys() {
            if !["display_name", "summary", "language", "target", "declared_sources"].contains(&name.as_str()) {
                results.push(field_rejection(name, "unknown_field"));
            }
        }
        let str_fields = [
            ("display_name", &base_proj.display_name, &mut cand_proj.display_name),
            ("summary", &base_proj.summary, &mut cand_proj.summary),
            ("language", &base_proj.language, &mut cand_proj.language),
        ];
        for (name, base, candidate_field) in str_fields {
            if let Some(value) = parameters.get(name) {
                let Some(value) = value.as_str().filter(|s| !s.trim().is_empty()) else {
                    results.push(field_rejection(name, "expected_nonempty_string"));
                    continue;
                };
                if name == "language" && !["VI", "EN", "JA", "ZH_HANT", "ZH_HANS"].contains(&value) {
                    results.push(field_rejection(name, "invalid_language"));
                    continue;
                }
                results.push(evaluate_single_field(
                    call.caller, "project", "require.project", name, value.to_string(),
                    base, candidate_field, declared_sources, pending_patches,
                ));
            }
        }
        if let Some(target) = parameters.get("target") {
            let Some(target) = target.as_object() else {
                results.push(field_rejection("target", "expected_object"));
                return;
            };
            for name in target.keys() {
                if !["arc_count", "chapter_count", "word_per_chapter_count",
                    "arc_tolerance", "chapter_tolerance", "word_per_chapter_tolerance"].contains(&name.as_str()) {
                    results.push(field_rejection(&format!("target.{name}"), "unknown_field"));
                }
            }
            let fields = [
                ("arc_count", &base_proj.target.arc_count, &mut cand_proj.target.arc_count),
                ("chapter_count", &base_proj.target.chapter_count, &mut cand_proj.target.chapter_count),
                ("word_per_chapter_count", &base_proj.target.word_per_chapter_count, &mut cand_proj.target.word_per_chapter_count),
            ];
            for (name, base, candidate_field) in fields {
                if let Some(value) = target.get(name) {
                    let field = format!("target.{name}");
                    let Some(value) = value.as_u64().and_then(|n| u32::try_from(n).ok()).filter(|n| *n > 0) else {
                        results.push(field_rejection(&field, "expected_positive_integer"));
                        continue;
                    };
                    results.push(evaluate_single_field(
                        call.caller, "project", "require.project", &field, value,
                        base, candidate_field, declared_sources, pending_patches,
                    ));
                }
            }
            // Advisory tolerances live outside `require`; zero is meaningful.
            let base_tolerance = &baseline.optional.overview_phase.project.target;
            let candidate_tolerance = &mut candidate.optional.overview_phase.project.target;
            let tolerance_fields = [
                ("arc_tolerance", &base_tolerance.arc_tolerance, &mut candidate_tolerance.arc_tolerance),
                ("chapter_tolerance", &base_tolerance.chapter_tolerance, &mut candidate_tolerance.chapter_tolerance),
                ("word_per_chapter_tolerance", &base_tolerance.word_per_chapter_tolerance, &mut candidate_tolerance.word_per_chapter_tolerance),
            ];
            for (name, base, candidate_field) in tolerance_fields {
                if let Some(value) = target.get(name) {
                    let field = format!("target.{name}");
                    let Some(value) = value.as_u64().and_then(|n| u32::try_from(n).ok()) else {
                        results.push(field_rejection(&field, "expected_nonnegative_integer"));
                        continue;
                    };
                    results.push(evaluate_single_field(
                        call.caller, "project", "optional.overview_phase.project", &field,
                        value, base, candidate_field, declared_sources, pending_patches,
                    ));
                }
            }
        }
    }

    fn handle_add_project(
        &self,
        call: &ToolCall,
        declared_sources: &BTreeMap<String, ProvenanceSource>,
        baseline: &BusinessFlowData,
        candidate: &mut BusinessFlowData,
        results: &mut Vec<FieldResult>,
        pending_patches: &mut Vec<PendingProposal>,
    ) {
        self.handle_set_project(call, declared_sources, baseline, candidate, results, pending_patches);
    }

    /// Assign the conflict motif of an existing Arc. The model used to have no schema for this
    /// tool and no handler at all, so Phase 1 could never satisfy the Arc-motif gate.
    fn handle_set_arc_motif(
        &self,
        call: &ToolCall,
        declared_sources: &BTreeMap<String, ProvenanceSource>,
        candidate: &mut BusinessFlowData,
        results: &mut Vec<FieldResult>,
    ) {
        let code = call
            .parameters
            .get("code")
            .or_else(|| call.parameters.get("arc_code"))
            .and_then(|value| value.as_str())
            .map(str::trim)
            .filter(|value| !value.is_empty());
        let Some(code) = code else {
            results.push(FieldResult {
                field: "code".to_string(),
                status: "rejected".to_string(),
                patch_id: None,
                error: Some("missing_target_code".to_string()),
                candidates: None,
            });
            return;
        };
        let motif_code = call
            .parameters
            .get("motif_code")
            .and_then(|value| value.as_str())
            .map(str::trim)
            .filter(|value| !value.is_empty());
        let Some(motif_code) = motif_code else {
            results.push(FieldResult {
                field: "motif_code".to_string(),
                status: "rejected".to_string(),
                patch_id: None,
                error: Some("missing_motif_code".to_string()),
                candidates: None,
            });
            return;
        };
        let Some(arc) = candidate
            .optional
            .overview_phase
            .arcs
            .iter_mut()
            .find(|arc| arc.code == code)
        else {
            results.push(FieldResult {
                field: "code".to_string(),
                status: "rejected".to_string(),
                patch_id: None,
                error: Some("target_code_not_found".to_string()),
                candidates: None,
            });
            return;
        };
        let source = *declared_sources
            .get("motif_code")
            .unwrap_or(&ProvenanceSource::AiGenerated);
        arc.motif.code = TrackedField::new(motif_code.to_string(), source);
        if let Some(note) = call
            .parameters
            .get("note")
            .and_then(|value| value.as_str())
            .map(str::trim)
            .filter(|value| !value.is_empty())
        {
            arc.motif.note = TrackedField::new(note.to_string(), source);
        }
        results.push(FieldResult {
            field: "motif_code".to_string(),
            status: "applied".to_string(),
            patch_id: None,
            error: None,
            candidates: Some(vec![motif_code.to_string()]),
        });
    }

    fn handle_add_arc(
        &self,
        call: &ToolCall,
        declared_sources: &BTreeMap<String, ProvenanceSource>,
        _baseline: &BusinessFlowData,
        candidate: &mut BusinessFlowData,
        results: &mut Vec<FieldResult>,
        _pending_patches: &mut Vec<PendingProposal>,
    ) {
        let source = *declared_sources
            .get("display_name")
            .unwrap_or(&ProvenanceSource::AiGenerated);
        let text = |key: &str| {
            call.parameters
                .get(key)
                .and_then(|value| value.as_str())
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .map(|value| TrackedField::new(value.to_string(), source))
                .unwrap_or_default()
        };
        let position = call
            .parameters
            .get("position")
            .and_then(|value| value.as_u64())
            .map(|value| TrackedField::new(value as u32, source));
        let code_param = call
            .parameters
            .get("code")
            .and_then(|value| value.as_str())
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(str::to_string);
        let motif_code = call
            .parameters
            .get("motif_code")
            .and_then(|value| value.as_str())
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(|value| TrackedField::new(value.to_string(), source));

        let protected = |field_source: Option<ProvenanceSource>| {
            matches!(
                field_source,
                Some(ProvenanceSource::User) | Some(ProvenanceSource::UserEdited)
            )
        };
        let merge_text = |current: &mut TrackedField<String>, incoming: TrackedField<String>| {
            if incoming.value.as_deref().map(str::trim).filter(|v| !v.is_empty()).is_some()
                && (!protected(current.source) || !protected(Some(source)) || source == ProvenanceSource::User)
            {
                *current = incoming;
            }
        };

        // Resolve the target Arc by explicit code first, then by position. Regenerating an Arc at
        // the same position must update it in place: generating a fresh `arc-N` code appended a
        // second copy and duplicated positions 2..5 in the live draft.
        let existing_index = code_param
            .as_ref()
            .and_then(|code| {
                candidate
                    .optional
                    .overview_phase
                    .arcs
                    .iter()
                    .position(|arc| &arc.code == code)
            })
            .or_else(|| {
                position.as_ref().and_then(|incoming| {
                    incoming.value.and_then(|wanted| {
                        candidate
                            .optional
                            .overview_phase
                            .arcs
                            .iter()
                            .position(|arc| arc.position.value == Some(wanted))
                    })
                })
            });

        if let Some(index) = existing_index {
            let arc = &mut candidate.optional.overview_phase.arcs[index];
            merge_text(&mut arc.display_name, text("display_name"));
            merge_text(&mut arc.summary, text("summary"));
            if let Some(incoming) = position.clone() {
                if incoming.value.is_some() && !protected(arc.position.source) {
                    arc.position = incoming;
                }
            }
            if let Some(incoming) = motif_code.clone() {
                if !protected(arc.motif.code.source) {
                    arc.motif.code = incoming;
                }
            }
            merge_text(&mut arc.motif.note, text("motif_note"));
            arc.source = Some(source);
            results.push(FieldResult {
                field: "code".to_string(),
                status: "applied".to_string(),
                patch_id: None,
                error: None,
                candidates: Some(vec![arc.code.clone()]),
            });
            return;
        }

        let target = candidate
            .require
            .project
            .target
            .arc_count
            .value
            .unwrap_or(0);
        if target > 0 && candidate.optional.overview_phase.arcs.len() >= target as usize {
            results.push(FieldResult {
                field: "position".to_string(),
                status: "rejected".to_string(),
                patch_id: None,
                error: Some("arc_target_reached".to_string()),
                candidates: None,
            });
            return;
        }

        let code = code_param.unwrap_or_else(|| {
            let existing: Vec<&str> = candidate
                .optional
                .overview_phase
                .arcs
                .iter()
                .map(|arc| arc.code.as_str())
                .collect();
            let mut next = existing.len() + 1;
            while existing.contains(&format!("arc-{next}").as_str()) {
                next += 1;
            }
            format!("arc-{next}")
        });
        let position = position.unwrap_or_else(|| {
            let next = candidate
                .optional
                .overview_phase
                .arcs
                .iter()
                .filter_map(|arc| arc.position.value)
                .max()
                .unwrap_or(0)
                + 1;
            TrackedField::new(next, source)
        });
        let mut motif = ArcMotifSection::default();
        if let Some(motif_code) = motif_code {
            motif.code = motif_code;
        }
        motif.note = text("motif_note");
        candidate.optional.overview_phase.arcs.push(OverviewArcItem {
            code: code.clone(),
            display_name: text("display_name"),
            summary: text("summary"),
            position,
            motif,
            themes: ArcThemeSection::default(),
            source: Some(source),
        });
        if !candidate.optional.chapter_phase.arcs.iter().any(|container| container.code == code) {
            candidate.optional.chapter_phase.arcs.push(ChapterPhaseArcItem {
                code: code.clone(),
                chapters: Vec::new(),
            });
        }
        if !candidate.optional.scene_config_phase.arcs.iter().any(|container| container.code == code) {
            candidate.optional.scene_config_phase.arcs.push(SceneConfigArcItem {
                code: code.clone(),
                chapters: Vec::new(),
            });
        }
        results.push(FieldResult {
            field: "code".to_string(),
            status: "applied".to_string(),
            patch_id: None,
            error: None,
            candidates: Some(vec![code]),
        });
    }

    fn handle_set_arc(
        &self,
        call: &ToolCall,
        declared_sources: &BTreeMap<String, ProvenanceSource>,
        baseline: &BusinessFlowData,
        candidate: &mut BusinessFlowData,
        results: &mut Vec<FieldResult>,
        pending_patches: &mut Vec<PendingProposal>,
    ) {
        let arc_code = match call.parameters.get("code").or_else(|| call.parameters.get("arc_code")).and_then(|v| v.as_str()) {
            Some(c) => c,
            None => {
                results.push(FieldResult {
                    field: "code".to_string(),
                    status: "rejected".to_string(),
                    patch_id: None,
                    error: Some("missing_target_code".to_string()),
                    candidates: None,
                });
                return;
            }
        };

        let base_arc = match baseline.optional.overview_phase.arcs.iter().find(|a| a.code == arc_code) {
            Some(a) => a,
            None => {
                results.push(FieldResult {
                    field: "code".to_string(),
                    status: "rejected".to_string(),
                    patch_id: None,
                    error: Some("target_code_not_found".to_string()),
                    candidates: None,
                });
                return;
            }
        };

        let cand_arc = candidate.optional.overview_phase.arcs.iter_mut().find(|a| a.code == arc_code).unwrap();

        if let Some(v) = call.parameters.get("display_name").and_then(|v| v.as_str()) {
            let res = evaluate_single_field(
                call.caller,
                "arc",
                arc_code,
                "display_name",
                v.to_string(),
                &base_arc.display_name,
                &mut cand_arc.display_name,
                declared_sources,
                pending_patches,
            );
            results.push(res);
        }
    }

    fn handle_add_chapter(
        &self,
        call: &ToolCall,
        declared_sources: &BTreeMap<String, ProvenanceSource>,
        _baseline: &BusinessFlowData,
        candidate: &mut BusinessFlowData,
        results: &mut Vec<FieldResult>,
        _pending_patches: &mut Vec<PendingProposal>,
    ) {
        let arc_code = match call.parameters.get("arc_code").and_then(|v| v.as_str()) {
            Some(a) => a,
            None => {
                results.push(FieldResult {
                    field: "arc_code".to_string(),
                    status: "rejected".to_string(),
                    patch_id: None,
                    error: Some("missing_target_code".to_string()),
                    candidates: None,
                });
                return;
            }
        };

        let arc_chap = match candidate.optional.chapter_phase.arcs.iter_mut().find(|a| a.code == arc_code) {
            Some(a) => a,
            None => {
                results.push(FieldResult {
                    field: "arc_code".to_string(),
                    status: "rejected".to_string(),
                    patch_id: None,
                    error: Some("target_code_not_found".to_string()),
                    candidates: None,
                });
                return;
            }
        };

        let source = *declared_sources
            .get("display_name")
            .unwrap_or(&ProvenanceSource::AiGenerated);
        let code = call
            .parameters
            .get("code")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| unique_child_code(
                arc_chap.chapters.iter().map(|chapter| chapter.code.as_str()),
                &format!("{arc_code}-c"),
                arc_chap.chapters.len() + 1,
            ));
        let text = |key: &str| {
            call.parameters
                .get(key)
                .and_then(|value| value.as_str())
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .map(|value| TrackedField::new(value.to_string(), source))
                .unwrap_or_default()
        };
        let position = call
            .parameters
            .get("position")
            .and_then(|value| value.as_u64())
            .map(|value| TrackedField::new(value as u32, source))
            .unwrap_or_default();

        let chap = ChapterPhaseChapterItem {
            code: code.clone(),
            display_name: text("display_name"),
            position,
            summary: text("summary"),
            scenes: Vec::new(),
            source: Some(source),
        };

        arc_chap.chapters.push(chap);
        results.push(FieldResult {
            field: "code".to_string(),
            status: "applied".to_string(),
            patch_id: None,
            error: None,
            candidates: Some(vec![code]),
        });
    }

    fn handle_set_chapter(
        &self,
        call: &ToolCall,
        declared_sources: &BTreeMap<String, ProvenanceSource>,
        baseline: &BusinessFlowData,
        candidate: &mut BusinessFlowData,
        results: &mut Vec<FieldResult>,
        pending_patches: &mut Vec<PendingProposal>,
    ) {
        let chap_code = match call.parameters.get("code").or_else(|| call.parameters.get("chapter_code")).and_then(|v| v.as_str()) {
            Some(c) => c,
            None => {
                results.push(FieldResult {
                    field: "code".to_string(),
                    status: "rejected".to_string(),
                    patch_id: None,
                    error: Some("missing_target_code".to_string()),
                    candidates: None,
                });
                return;
            }
        };

        let base_chap = baseline.optional.chapter_phase.arcs.iter().flat_map(|a| &a.chapters).find(|c| c.code == chap_code);
        if base_chap.is_none() {
            results.push(FieldResult {
                field: "code".to_string(),
                status: "rejected".to_string(),
                patch_id: None,
                error: Some("target_code_not_found".to_string()),
                candidates: None,
            });
            return;
        }
        let base_chap = base_chap.unwrap();

        let cand_chap = candidate.optional.chapter_phase.arcs.iter_mut().flat_map(|a| &mut a.chapters).find(|c| c.code == chap_code).unwrap();

        if let Some(v) = call.parameters.get("display_name").and_then(|v| v.as_str()) {
            let res = evaluate_single_field(
                call.caller,
                "chapter",
                chap_code,
                "display_name",
                v.to_string(),
                &base_chap.display_name,
                &mut cand_chap.display_name,
                declared_sources,
                pending_patches,
            );
            results.push(res);
        }
        if let Some(v) = call.parameters.get("summary").and_then(|v| v.as_str()) {
            let res = evaluate_single_field(
                call.caller,
                "chapter",
                chap_code,
                "summary",
                v.to_string(),
                &base_chap.summary,
                &mut cand_chap.summary,
                declared_sources,
                pending_patches,
            );
            results.push(res);
        }
        if let Some(v) = call.parameters.get("position").and_then(|v| v.as_u64()) {
            let res = evaluate_single_field(
                call.caller,
                "chapter",
                chap_code,
                "position",
                v as u32,
                &base_chap.position,
                &mut cand_chap.position,
                declared_sources,
                pending_patches,
            );
            results.push(res);
        }
    }

    fn handle_add_scene(
        &self,
        call: &ToolCall,
        declared_sources: &BTreeMap<String, ProvenanceSource>,
        _baseline: &BusinessFlowData,
        candidate: &mut BusinessFlowData,
        results: &mut Vec<FieldResult>,
        _pending_patches: &mut Vec<PendingProposal>,
    ) {
        let chap_code = match call.parameters.get("chapter_code").and_then(|v| v.as_str()) {
            Some(c) => c,
            None => {
                results.push(FieldResult {
                    field: "chapter_code".to_string(),
                    status: "rejected".to_string(),
                    patch_id: None,
                    error: Some("missing_target_code".to_string()),
                    candidates: None,
                });
                return;
            }
        };

        let chap = match candidate.optional.chapter_phase.arcs.iter_mut().flat_map(|a| &mut a.chapters).find(|c| c.code == chap_code) {
            Some(c) => c,
            None => {
                results.push(FieldResult {
                    field: "chapter_code".to_string(),
                    status: "rejected".to_string(),
                    patch_id: None,
                    error: Some("target_code_not_found".to_string()),
                    candidates: None,
                });
                return;
            }
        };

        let source = *declared_sources
            .get("display_name")
            .unwrap_or(&ProvenanceSource::AiGenerated);
        let code = call
            .parameters
            .get("code")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| unique_child_code(
                chap.scenes.iter().map(|scene| scene.code.as_str()),
                &format!("{chap_code}-s"),
                chap.scenes.len() + 1,
            ));
        let text = |key: &str| {
            call.parameters
                .get(key)
                .and_then(|value| value.as_str())
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .map(|value| TrackedField::new(value.to_string(), source))
                .unwrap_or_default()
        };
        let position = call
            .parameters
            .get("position")
            .and_then(|value| value.as_u64())
            .map(|value| TrackedField::new(value as u32, source))
            .unwrap_or_default();

        let sc = ChapterPhaseSceneItem {
            code: code.clone(),
            display_name: text("display_name"),
            position,
            summary: text("summary"),
            world_setting_code: text("world_setting_code"),
            source: Some(source),
        };

        chap.scenes.push(sc);
        results.push(FieldResult {
            field: "code".to_string(),
            status: "applied".to_string(),
            patch_id: None,
            error: None,
            candidates: Some(vec![code]),
        });
    }

    fn handle_set_scene(
        &self,
        call: &ToolCall,
        declared_sources: &BTreeMap<String, ProvenanceSource>,
        baseline: &BusinessFlowData,
        candidate: &mut BusinessFlowData,
        results: &mut Vec<FieldResult>,
        pending_patches: &mut Vec<PendingProposal>,
    ) {
        let sc_code = match call.parameters.get("code").or_else(|| call.parameters.get("scene_code")).and_then(|v| v.as_str()) {
            Some(s) => s,
            None => {
                results.push(FieldResult {
                    field: "code".to_string(),
                    status: "rejected".to_string(),
                    patch_id: None,
                    error: Some("missing_target_code".to_string()),
                    candidates: None,
                });
                return;
            }
        };

        let base_sc = baseline.optional.chapter_phase.arcs.iter().flat_map(|a| &a.chapters).flat_map(|c| &c.scenes).find(|s| s.code == sc_code);
        if base_sc.is_none() {
            results.push(FieldResult {
                field: "code".to_string(),
                status: "rejected".to_string(),
                patch_id: None,
                error: Some("target_code_not_found".to_string()),
                candidates: None,
            });
            return;
        }
        let base_sc = base_sc.unwrap();

        let cand_sc = candidate.optional.chapter_phase.arcs.iter_mut().flat_map(|a| &mut a.chapters).flat_map(|c| &mut c.scenes).find(|s| s.code == sc_code).unwrap();

        if let Some(v) = call.parameters.get("display_name").and_then(|v| v.as_str()) {
            let res = evaluate_single_field(
                call.caller,
                "scene",
                sc_code,
                "display_name",
                v.to_string(),
                &base_sc.display_name,
                &mut cand_sc.display_name,
                declared_sources,
                pending_patches,
            );
            results.push(res);
        }

        if let Some(v) = call.parameters.get("world_setting_code").and_then(|v| v.as_str()) {
            let res = evaluate_single_field(
                call.caller,
                "scene",
                sc_code,
                "world_setting_code",
                v.to_string(),
                &base_sc.world_setting_code,
                &mut cand_sc.world_setting_code,
                declared_sources,
                pending_patches,
            );
            results.push(res);
        }
        if let Some(v) = call.parameters.get("summary").and_then(|v| v.as_str()) {
            let res = evaluate_single_field(
                call.caller,
                "scene",
                sc_code,
                "summary",
                v.to_string(),
                &base_sc.summary,
                &mut cand_sc.summary,
                declared_sources,
                pending_patches,
            );
            results.push(res);
        }
        if let Some(v) = call.parameters.get("position").and_then(|v| v.as_u64()) {
            let res = evaluate_single_field(
                call.caller,
                "scene",
                sc_code,
                "position",
                v as u32,
                &base_sc.position,
                &mut cand_sc.position,
                declared_sources,
                pending_patches,
            );
            results.push(res);
        }
    }

    fn handle_set_scene_config(
        &self,
        call: &ToolCall,
        declared_sources: &BTreeMap<String, ProvenanceSource>,
        baseline: &BusinessFlowData,
        candidate: &mut BusinessFlowData,
        results: &mut Vec<FieldResult>,
        pending_patches: &mut Vec<PendingProposal>,
    ) {
        let sc_code = match call.parameters.get("code").or_else(|| call.parameters.get("scene_code")).and_then(|v| v.as_str()) {
            Some(s) => s,
            None => {
                results.push(FieldResult {
                    field: "code".to_string(),
                    status: "rejected".to_string(),
                    patch_id: None,
                    error: Some("missing_target_code".to_string()),
                    candidates: None,
                });
                return;
            }
        };

        // Ensure scene exists in scene_config_phase
        let has_sc = candidate.optional.scene_config_phase.arcs.iter().flat_map(|a| &a.chapters).flat_map(|c| &c.scenes).any(|s| s.code == sc_code);
        if !has_sc {
            // Find in chapter_phase and create container in scene_config_phase if needed
            let mut found = false;
            for arc in &candidate.optional.chapter_phase.arcs {
                for chap in &arc.chapters {
                    if chap.scenes.iter().any(|s| s.code == sc_code) {
                        found = true;
                        let arc_cfg = candidate.optional.scene_config_phase.arcs.iter_mut().find(|a| a.code == arc.code);
                        if let Some(ac) = arc_cfg {
                            let chap_cfg = ac.chapters.iter_mut().find(|c| c.code == chap.code);
                            if let Some(cc) = chap_cfg {
                                cc.scenes.push(SceneConfigSceneItem {
                                    code: sc_code.to_string(),
                                    world_setting_code: TrackedField::unset(),
                                    style_preset_code: TrackedField::unset(),
                                    narrator_config_code: TrackedField::unset(),
                                    lore_entries: Vec::new(),
                                    characters: Vec::new(),
                                });
                            }
                        }
                    }
                }
            }
            if !found {
                results.push(FieldResult {
                    field: "code".to_string(),
                    status: "rejected".to_string(),
                    patch_id: None,
                    error: Some("target_code_not_found".to_string()),
                    candidates: None,
                });
                return;
            }
        }

        let base_sc = baseline.optional.scene_config_phase.arcs.iter().flat_map(|a| &a.chapters).flat_map(|c| &c.scenes).find(|s| s.code == sc_code);
        let cand_sc = candidate.optional.scene_config_phase.arcs.iter_mut().flat_map(|a| &mut a.chapters).flat_map(|c| &mut c.scenes).find(|s| s.code == sc_code).unwrap();

        if let Some(v) = call.parameters.get("world_setting_code").and_then(|v| v.as_str()) {
            let base_f = base_sc.map(|b| &b.world_setting_code);
            let dummy = TrackedField::unset();
            let base_f_ref = base_f.unwrap_or(&dummy);

            let res = evaluate_single_field(
                call.caller,
                "scene_config",
                sc_code,
                "world_setting_code",
                v.to_string(),
                base_f_ref,
                &mut cand_sc.world_setting_code,
                declared_sources,
                pending_patches,
            );
            results.push(res);
        }
    }

    fn handle_add_world_setting(
        &self,
        call: &ToolCall,
        declared_sources: &BTreeMap<String, ProvenanceSource>,
        _baseline: &BusinessFlowData,
        candidate: &mut BusinessFlowData,
        results: &mut Vec<FieldResult>,
        _pending_patches: &mut Vec<PendingProposal>,
    ) {
        let code = call
            .parameters
            .get("code")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| format!("ws-{}", candidate.optional.world_building_phase.world_settings.len() + 1));

        let parent_code = call.parameters.get("parent_code").and_then(|v| v.as_str()).map(|s| s.to_string());
        let disp = call.parameters.get("display_name").and_then(|v| v.as_str()).unwrap_or("Setting").to_string();
        let st_type = call.parameters.get("type").and_then(|v| v.as_str()).unwrap_or("LOCATION").to_string();

        let source = declared_sources
            .get("display_name")
            .copied()
            .unwrap_or(ProvenanceSource::AiGenerated);
        let text = |key: &str| {
            call.parameters
                .get(key)
                .and_then(|v| v.as_str())
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .map(|value| TrackedField::new(value.to_string(), source))
                .unwrap_or_default()
        };
        let ws = WorldSettingItem {
            code: code.clone(),
            parent_code: parent_code
                .map(|value| TrackedField::new(value, source))
                .unwrap_or_default(),
            display_name: TrackedField::new(disp, source),
            setting_type: TrackedField::new(st_type, source),
            description: text("description"),
            access: text("access"),
            spatial_features: TrackedField::unset(),
            sensory_visual: TrackedField::unset(),
            sensory_auditory: TrackedField::unset(),
            sensory_tactile: TrackedField::unset(),
            sensory_olfactory: TrackedField::unset(),
            sensory_gustatory: TrackedField::unset(),
            lore_entries: Vec::new(),
            source: Some(source),
        };

        candidate.optional.world_building_phase.world_settings.push(ws);
        results.push(FieldResult {
            field: "code".to_string(),
            status: "applied".to_string(),
            patch_id: None,
            error: None,
            candidates: None,
        });
    }

    fn handle_set_world_setting(
        &self,
        call: &ToolCall,
        declared_sources: &BTreeMap<String, ProvenanceSource>,
        baseline: &BusinessFlowData,
        candidate: &mut BusinessFlowData,
        results: &mut Vec<FieldResult>,
        pending_patches: &mut Vec<PendingProposal>,
    ) {
        let ws_code = match call.parameters.get("code").or_else(|| call.parameters.get("world_setting_code")).and_then(|v| v.as_str()) {
            Some(c) => c,
            None => {
                results.push(FieldResult {
                    field: "code".to_string(),
                    status: "rejected".to_string(),
                    patch_id: None,
                    error: Some("missing_target_code".to_string()),
                    candidates: None,
                });
                return;
            }
        };

        let base_ws = baseline.optional.world_building_phase.world_settings.iter().find(|w| w.code == ws_code);
        if base_ws.is_none() {
            results.push(FieldResult {
                field: "code".to_string(),
                status: "rejected".to_string(),
                patch_id: None,
                error: Some("target_code_not_found".to_string()),
                candidates: None,
            });
            return;
        }
        let base_ws = base_ws.unwrap();

        let cand_ws = candidate.optional.world_building_phase.world_settings.iter_mut().find(|w| w.code == ws_code).unwrap();

        if let Some(v) = call.parameters.get("display_name").and_then(|v| v.as_str()) {
            let res = evaluate_single_field(
                call.caller,
                "world_setting",
                ws_code,
                "display_name",
                v.to_string(),
                &base_ws.display_name,
                &mut cand_ws.display_name,
                declared_sources,
                pending_patches,
            );
            results.push(res);
        }
    }

    fn handle_add_lore_entry(
        &self,
        call: &ToolCall,
        declared_sources: &BTreeMap<String, ProvenanceSource>,
        _baseline: &BusinessFlowData,
        candidate: &mut BusinessFlowData,
        results: &mut Vec<FieldResult>,
        _pending_patches: &mut Vec<PendingProposal>,
    ) {
        let ws_code = call.parameters.get("world_setting_code").and_then(|v| v.as_str());
        let ws = if let Some(code) = ws_code {
            candidate.optional.world_building_phase.world_settings.iter_mut().find(|w| w.code == code)
        } else {
            candidate.optional.world_building_phase.world_settings.first_mut()
        };

        let ws = match ws {
            Some(w) => w,
            None => {
                results.push(FieldResult {
                    field: "world_setting_code".to_string(),
                    status: "rejected".to_string(),
                    patch_id: None,
                    error: Some("target_code_not_found".to_string()),
                    candidates: None,
                });
                return;
            }
        };

        let code = call
            .parameters
            .get("code")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| format!("lore-{}", ws.lore_entries.len() + 1));

        let source = declared_sources
            .get("display_name")
            .copied()
            .unwrap_or(ProvenanceSource::AiGenerated);
        let description = call
            .parameters
            .get("description")
            .and_then(|v| v.as_str())
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(|value| TrackedField::new(value.to_string(), source))
            .unwrap_or_default();
        let data = call
            .parameters
            .get("data")
            .filter(|value| value.is_object())
            .cloned()
            .map(|value| TrackedField::new(value, source))
            .unwrap_or_default();
        let lore = LoreEntryItem {
            code: code.clone(),
            display_name: TrackedField::new(
                call.parameters.get("display_name").and_then(|v| v.as_str()).unwrap_or("Lore").to_string(),
                source,
            ),
            lore_type: TrackedField::new(
                call.parameters.get("type").and_then(|v| v.as_str()).unwrap_or("CONCEPT").to_string(),
                source,
            ),
            description,
            data,
            source: Some(source),
        };

        ws.lore_entries.push(lore);
        results.push(FieldResult {
            field: "code".to_string(),
            status: "applied".to_string(),
            patch_id: None,
            error: None,
            candidates: None,
        });
    }

    fn handle_set_lore_entry(
        &self,
        call: &ToolCall,
        declared_sources: &BTreeMap<String, ProvenanceSource>,
        baseline: &BusinessFlowData,
        candidate: &mut BusinessFlowData,
        results: &mut Vec<FieldResult>,
        pending_patches: &mut Vec<PendingProposal>,
    ) {
        let lore_code = match call.parameters.get("code").or_else(|| call.parameters.get("lore_entry_code")).and_then(|v| v.as_str()) {
            Some(c) => c,
            None => {
                results.push(FieldResult {
                    field: "code".to_string(),
                    status: "rejected".to_string(),
                    patch_id: None,
                    error: Some("missing_target_code".to_string()),
                    candidates: None,
                });
                return;
            }
        };

        let mut base_lore = None;
        for ws in &baseline.optional.world_building_phase.world_settings {
            for l in &ws.lore_entries {
                if l.code == lore_code {
                    base_lore = Some(l);
                    break;
                }
            }
        }

        let base_lore = match base_lore {
            Some(l) => l,
            None => {
                results.push(FieldResult {
                    field: "code".to_string(),
                    status: "rejected".to_string(),
                    patch_id: None,
                    error: Some("target_code_not_found".to_string()),
                    candidates: None,
                });
                return;
            }
        };

        let mut cand_lore = None;
        for ws in &mut candidate.optional.world_building_phase.world_settings {
            for l in &mut ws.lore_entries {
                if l.code == lore_code {
                    cand_lore = Some(l);
                    break;
                }
            }
        }
        let cand_lore = cand_lore.unwrap();

        if let Some(v) = call.parameters.get("display_name").and_then(|v| v.as_str()) {
            let res = evaluate_single_field(
                call.caller,
                "lore_entry",
                lore_code,
                "display_name",
                v.to_string(),
                &base_lore.display_name,
                &mut cand_lore.display_name,
                declared_sources,
                pending_patches,
            );
            results.push(res);
        }
    }

    fn handle_add_relationship(
        &self,
        call: &ToolCall,
        declared_sources: &BTreeMap<String, ProvenanceSource>,
        _baseline: &BusinessFlowData,
        candidate: &mut BusinessFlowData,
        results: &mut Vec<FieldResult>,
        _pending_patches: &mut Vec<PendingProposal>,
    ) {
        let from = call
            .parameters
            .get("from_code")
            .or_else(|| call.parameters.get("from_character_code"))
            .and_then(|v| v.as_str());
        let to = call
            .parameters
            .get("to_code")
            .or_else(|| call.parameters.get("to_character_code"))
            .and_then(|v| v.as_str());

        let (Some(from), Some(to)) = (from, to) else {
            results.push(field_rejection("characters", "missing_target_code"));
            return;
        };
        for code in [from, to] {
            if !candidate.require.character.iter().any(|character| character.code == code) {
                results.push(field_rejection("characters", "target_code_not_found"));
                return;
            }
        }

        let source = *declared_sources
            .get("code")
            .or_else(|| declared_sources.get("display_name"))
            .unwrap_or(&ProvenanceSource::AiGenerated);
        let motif_code = call
            .parameters
            .get("motif_code")
            .and_then(|v| v.as_str())
            .filter(|value| !value.trim().is_empty());

        // The (from, to) pair is unique: reuse a populated relationship instead of pushing a
        // duplicate code that later fails validation.
        if let Some(existing) = candidate
            .optional
            .relationship_phase
            .relationship
            .iter_mut()
            .find(|item| item.character.from_code == from && item.character.to_code == to)
        {
            if let Some(motif) = motif_code {
                existing.motif.code = TrackedField::new(motif.to_string(), source);
            }
            results.push(FieldResult {
                field: "code".to_string(),
                status: "applied".to_string(),
                patch_id: None,
                error: None,
                candidates: Some(vec![existing.code.clone()]),
            });
            return;
        }

        let code = call
            .parameters
            .get("code")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| format!("rel_{from}_{to}"));

        let mut motif_section = RelationshipMotifSection::default();
        if let Some(motif_value) = motif_code {
            motif_section.code = TrackedField::new(motif_value.to_string(), source);
        }

        candidate.optional.relationship_phase.relationship.push(RelationshipPhaseItem {
            code: code.clone(),
            motif: motif_section,
            character: RelationshipCharacterPairSection {
                from_code: from.to_string(),
                to_code: to.to_string(),
                scenes: Vec::new(),
                roles: Vec::new(),
            },
            source: Some(source),
        });
        results.push(FieldResult {
            field: "code".to_string(),
            status: "applied".to_string(),
            patch_id: None,
            error: None,
            candidates: Some(vec![code]),
        });
    }

    fn handle_set_relationship_state(
        &self,
        call: &ToolCall,
        declared_sources: &BTreeMap<String, ProvenanceSource>,
        _baseline: &BusinessFlowData,
        candidate: &mut BusinessFlowData,
        results: &mut Vec<FieldResult>,
        _pending_patches: &mut Vec<PendingProposal>,
    ) {
        let rel_code = call.parameters.get("relationship_code").and_then(|v| v.as_str());
        let sc_code = call.parameters.get("scene_code").and_then(|v| v.as_str());

        let (Some(rel_code), Some(sc_code)) = (rel_code, sc_code) else {
            results.push(field_rejection("relationship_code", "missing_target_code"));
            return;
        };

        let scene_exists = candidate
            .optional
            .chapter_phase
            .arcs
            .iter()
            .flat_map(|arc| arc.chapters.iter())
            .flat_map(|chapter| chapter.scenes.iter())
            .any(|scene| scene.code == sc_code)
            || candidate
                .optional
                .scene_config_phase
                .arcs
                .iter()
                .flat_map(|arc| arc.chapters.iter())
                .flat_map(|chapter| chapter.scenes.iter())
                .any(|scene| scene.code == sc_code);
        if !scene_exists {
            results.push(field_rejection("scene_code", "target_code_not_found"));
            return;
        }

        let rel = match candidate
            .optional
            .relationship_phase
            .relationship
            .iter_mut()
            .find(|item| item.code == rel_code)
        {
            Some(rel) => rel,
            None => {
                results.push(field_rejection("relationship_code", "target_code_not_found"));
                return;
            }
        };

        const DIMENSIONS: [&str; 7] = [
            "trust", "affection", "hostility", "fear", "respect", "dependency", "familiarity",
        ];
        let mut values: std::collections::BTreeMap<&str, (String, ProvenanceSource)> =
            std::collections::BTreeMap::new();
        for dimension in DIMENSIONS {
            if let Some(value) = call.parameters.get(dimension).and_then(|v| v.as_str()) {
                let normalized = value.trim().to_ascii_uppercase();
                if !matches!(
                    normalized.as_str(),
                    "NONE" | "LOW" | "MODERATE" | "HIGH" | "ABSOLUTE"
                ) {
                    results.push(field_rejection(dimension, "invalid_enum"));
                    return;
                }
                let source = *declared_sources
                    .get(dimension)
                    .unwrap_or(&ProvenanceSource::AiGenerated);
                values.insert(dimension, (normalized, source));
            }
        }
        if values.is_empty() {
            results.push(field_rejection("state", "no_supported_fields"));
            return;
        }

        // Merge into the existing scene state instead of replacing it.
        let position = rel
            .character
            .scenes
            .iter()
            .position(|state| state.scene_code == sc_code);
        let state = match position {
            Some(index) => &mut rel.character.scenes[index],
            None => {
                rel.character.scenes.push(RelationshipSceneStateItem {
                    scene_code: sc_code.to_string(),
                    ..Default::default()
                });
                rel.character.scenes.last_mut().unwrap()
            }
        };
        for (dimension, (value, source)) in values {
            let assigned: Option<&mut TrackedField<String>> = match dimension {
                "trust" => Some(&mut state.trust),
                "affection" => Some(&mut state.affection),
                "hostility" => Some(&mut state.hostility),
                "fear" => Some(&mut state.fear),
                "respect" => Some(&mut state.respect),
                "dependency" => Some(&mut state.dependency),
                _ => Some(&mut state.familiarity),
            };
            if let Some(field) = assigned {
                *field = TrackedField::new(value, source);
            }
        }
        state.source = Some(ProvenanceSource::AiGenerated);
        results.push(FieldResult {
            field: "state".to_string(),
            status: "applied".to_string(),
            patch_id: None,
            error: None,
            candidates: None,
        });
    }

    fn handle_add_scene_character(
        &self,
        call: &ToolCall,
        _declared_sources: &BTreeMap<String, ProvenanceSource>,
        _baseline: &BusinessFlowData,
        candidate: &mut BusinessFlowData,
        results: &mut Vec<FieldResult>,
        _pending_patches: &mut Vec<PendingProposal>,
    ) {
        let sc_code = call.parameters.get("scene_code").and_then(|v| v.as_str());
        let char_code = call.parameters.get("character_code").and_then(|v| v.as_str());

        if sc_code.is_none() || char_code.is_none() {
            results.push(FieldResult {
                field: "target".to_string(),
                status: "rejected".to_string(),
                patch_id: None,
                error: Some("missing_target_code".to_string()),
                candidates: None,
            });
            return;
        }

        let (sc_code, char_code) = (sc_code.unwrap(), char_code.unwrap());
        let mut found = false;

        for arc in &mut candidate.optional.scene_config_phase.arcs {
            for chap in &mut arc.chapters {
                for sc in &mut chap.scenes {
                    if sc.code == sc_code {
                        sc.characters.push(SceneCharacterLink {
                            character_code: char_code.to_string(),
                            is_pov: TrackedField::unset(),
                            source: Some(ProvenanceSource::User),
                        });
                        found = true;
                    }
                }
            }
        }

        if found {
            results.push(FieldResult {
                field: "character_code".to_string(),
                status: "applied".to_string(),
                patch_id: None,
                error: None,
                candidates: None,
            });
        } else {
            results.push(FieldResult {
                field: "scene_code".to_string(),
                status: "rejected".to_string(),
                patch_id: None,
                error: Some("target_code_not_found".to_string()),
                candidates: None,
            });
        }
    }

    fn handle_add_scene_lore_entry(
        &self,
        call: &ToolCall,
        _declared_sources: &BTreeMap<String, ProvenanceSource>,
        _baseline: &BusinessFlowData,
        candidate: &mut BusinessFlowData,
        results: &mut Vec<FieldResult>,
        _pending_patches: &mut Vec<PendingProposal>,
    ) {
        let sc_code = call.parameters.get("scene_code").and_then(|v| v.as_str());
        let lore_code = call.parameters.get("lore_entry_code").and_then(|v| v.as_str());

        if sc_code.is_none() || lore_code.is_none() {
            results.push(FieldResult {
                field: "target".to_string(),
                status: "rejected".to_string(),
                patch_id: None,
                error: Some("missing_target_code".to_string()),
                candidates: None,
            });
            return;
        }

        let (sc_code, lore_code) = (sc_code.unwrap(), lore_code.unwrap());
        let mut found = false;

        for arc in &mut candidate.optional.scene_config_phase.arcs {
            for chap in &mut arc.chapters {
                for sc in &mut chap.scenes {
                    if sc.code == sc_code {
                        sc.lore_entries.push(SceneLoreEntryLink {
                            lore_entry_code: lore_code.to_string(),
                            usage: TrackedField::unset(),
                            reveal_to_reader: TrackedField::unset(),
                            reveal_to_pov: TrackedField::unset(),
                            source: Some(ProvenanceSource::User),
                        });
                        found = true;
                    }
                }
            }
        }

        if found {
            results.push(FieldResult {
                field: "lore_entry_code".to_string(),
                status: "applied".to_string(),
                patch_id: None,
                error: None,
                candidates: None,
            });
        } else {
            results.push(FieldResult {
                field: "scene_code".to_string(),
                status: "rejected".to_string(),
                patch_id: None,
                error: Some("target_code_not_found".to_string()),
                candidates: None,
            });
        }
    }

    fn handle_add_project_genre(
        &self,
        call: &ToolCall,
        declared_sources: &BTreeMap<String, ProvenanceSource>,
        _baseline: &BusinessFlowData,
        candidate: &mut BusinessFlowData,
        results: &mut Vec<FieldResult>,
        _pending_patches: &mut Vec<PendingProposal>,
    ) {
        let genre_code = match call.parameters.get("genre_code").and_then(|v| v.as_str()) {
            Some(g) => g,
            None => {
                results.push(FieldResult {
                    field: "genre_code".to_string(),
                    status: "rejected".to_string(),
                    patch_id: None,
                    error: Some("missing_field".to_string()),
                    candidates: None,
                });
                return;
            }
        };

        let source = if call.caller.is_ai() {
            match declared_sources.get("genre_code") {
                Some(ProvenanceSource::User) => ProvenanceSource::User,
                Some(ProvenanceSource::AiGenerated) => ProvenanceSource::AiGenerated,
                _ => {
                    // Tolerate the legacy scalar form some prompts/models still emit.
                    match call.parameters.get("declared_source").and_then(|value| value.as_str()) {
                        Some("user") => ProvenanceSource::User,
                        Some("ai_generated") => ProvenanceSource::AiGenerated,
                        _ => {
                            results.push(field_rejection("genre_code", "missing_or_invalid_declared_source"));
                            return;
                        }
                    }
                }
            }
        } else {
            ProvenanceSource::User
        };
        if candidate.require.project.genres.iter().any(|item| item.code.value.as_deref() == Some(genre_code)) {
            results.push(field_rejection("genre_code", "genre_already_attached"));
            return;
        }
        candidate.require.project.genres.push(RequireGenreItem {
            code: TrackedField::new(genre_code.to_string(), source),
            is_primary: TrackedField::new(
                call.parameters.get("is_primary").and_then(|v| v.as_bool()).unwrap_or(false), source,
            ),
            display_name: TrackedField::unset(),
        });

        results.push(FieldResult {
            field: "genre_code".to_string(),
            status: "applied".to_string(),
            patch_id: None,
            error: None,
            candidates: None,
        });
    }

    fn handle_add_arc_event_block(
        &self,
        call: &ToolCall,
        declared_sources: &BTreeMap<String, ProvenanceSource>,
        _baseline: &BusinessFlowData,
        candidate: &mut BusinessFlowData,
        results: &mut Vec<FieldResult>,
        _pending_patches: &mut Vec<PendingProposal>,
    ) {
        let arc_code = match call.parameters.get("arc_code").and_then(|v| v.as_str()) {
            Some(a) => a,
            None => {
                results.push(FieldResult {
                    field: "arc_code".to_string(),
                    status: "rejected".to_string(),
                    patch_id: None,
                    error: Some("missing_target_code".to_string()),
                    candidates: None,
                });
                return;
            }
        };

        let arc = match candidate.optional.overview_phase.arcs.iter_mut().find(|a| a.code == arc_code) {
            Some(a) => a,
            None => {
                results.push(FieldResult {
                    field: "arc_code".to_string(),
                    status: "rejected".to_string(),
                    patch_id: None,
                    error: Some("target_code_not_found".to_string()),
                    candidates: None,
                });
                return;
            }
        };

        let source = *declared_sources
            .get("summary")
            .unwrap_or(&ProvenanceSource::AiGenerated);
        let code = call
            .parameters
            .get("event_block_code")
            .or_else(|| call.parameters.get("code"))
            .and_then(|v| v.as_str())
            .map(str::to_string)
            .unwrap_or_else(|| format!("{arc_code}-eb-{}", arc.motif.event_blocks.len() + 1));
        let text = |key: &str| {
            call.parameters
                .get(key)
                .and_then(|value| value.as_str())
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .map(|value| TrackedField::new(value.to_string(), source))
                .unwrap_or_default()
        };
        let number = |key: &str| {
            call.parameters
                .get(key)
                .and_then(|value| value.as_u64())
                .map(|value| TrackedField::new(value as u32, source))
                .unwrap_or_default()
        };
        arc.motif.event_blocks.push(ArcEventBlockItem {
            code: code.clone(),
            event_block_code: TrackedField::new(code.clone(), source),
            start_chapter_code: text("start_chapter_code"),
            end_chapter_code: text("end_chapter_code"),
            start_chapter: number("start_chapter"),
            end_chapter: number("end_chapter"),
            summary: text("summary"),
            source: Some(source),
        });

        results.push(FieldResult {
            field: "code".to_string(),
            status: "applied".to_string(),
            patch_id: None,
            error: None,
            candidates: Some(vec![code]),
        });
    }

    fn handle_add_character_memory(
        &self,
        call: &ToolCall,
        _declared_sources: &BTreeMap<String, ProvenanceSource>,
        _baseline: &BusinessFlowData,
        candidate: &mut BusinessFlowData,
        results: &mut Vec<FieldResult>,
        _pending_patches: &mut Vec<PendingProposal>,
    ) {
        let char_code = match call.parameters.get("character_code").and_then(|v| v.as_str()) {
            Some(c) => c,
            None => {
                results.push(FieldResult {
                    field: "character_code".to_string(),
                    status: "rejected".to_string(),
                    patch_id: None,
                    error: Some("missing_target_code".to_string()),
                    candidates: None,
                });
                return;
            }
        };

        // Ensure character profile container exists in life experience phase
        if !candidate.optional.character_life_experience_phase.character.iter().any(|c| c.code == char_code) {
            candidate.optional.character_life_experience_phase.character.push(CharacterLifeExperienceItem {
                code: char_code.to_string(),
                memories: Vec::new(),
                competences: Vec::new(),
            });
        }

        let life = candidate.optional.character_life_experience_phase.character.iter_mut().find(|c| c.code == char_code).unwrap();
        let content = call.parameters.get("content").and_then(|v| v.as_str()).unwrap_or("");
        let sc_code = call.parameters.get("source_scene_code").and_then(|v| v.as_str());

        life.memories.push(CharacterMemoryItem {
            code: Some(format!("mem-{}", life.memories.len() + 1)),
            source_scene_code: sc_code.map(|s| TrackedField::user(s.to_string())).unwrap_or_else(TrackedField::unset),
            content: TrackedField::user(content.to_string()),
            source: Some(ProvenanceSource::User),
        });

        results.push(FieldResult {
            field: "content".to_string(),
            status: "applied".to_string(),
            patch_id: None,
            error: None,
            candidates: None,
        });
    }

    fn handle_add_character_competence(
        &self,
        call: &ToolCall,
        _declared_sources: &BTreeMap<String, ProvenanceSource>,
        _baseline: &BusinessFlowData,
        candidate: &mut BusinessFlowData,
        results: &mut Vec<FieldResult>,
        _pending_patches: &mut Vec<PendingProposal>,
    ) {
        let char_code = match call.parameters.get("character_code").and_then(|v| v.as_str()) {
            Some(c) => c,
            None => {
                results.push(FieldResult {
                    field: "character_code".to_string(),
                    status: "rejected".to_string(),
                    patch_id: None,
                    error: Some("missing_target_code".to_string()),
                    candidates: None,
                });
                return;
            }
        };

        if !candidate.optional.character_life_experience_phase.character.iter().any(|c| c.code == char_code) {
            candidate.optional.character_life_experience_phase.character.push(CharacterLifeExperienceItem {
                code: char_code.to_string(),
                memories: Vec::new(),
                competences: Vec::new(),
            });
        }

        let life = candidate.optional.character_life_experience_phase.character.iter_mut().find(|c| c.code == char_code).unwrap();
        let comp_code = call.parameters.get("code").and_then(|v| v.as_str()).map(|s| s.to_string()).unwrap_or_else(|| format!("{}-k{}", char_code, life.competences.len() + 1));
        let sc_code = call.parameters.get("scene_code").and_then(|v| v.as_str()).unwrap_or("");
        let disp = call.parameters.get("display_name").and_then(|v| v.as_str()).unwrap_or("Competence");

        life.competences.push(CharacterCompetenceItem {
            code: comp_code.clone(),
            domain_code: TrackedField::unset(),
            level_code: TrackedField::unset(),
            scene_code: TrackedField::user(sc_code.to_string()),
            display_name: TrackedField::user(disp.to_string()),
            description: TrackedField::unset(),
            reliability: TrackedField::unset(),
            experience: TrackedField::unset(),
            capabilities: Vec::new(),
            limitations: Vec::new(),
            source: Some(ProvenanceSource::User),
        });

        results.push(FieldResult {
            field: "code".to_string(),
            status: "applied".to_string(),
            patch_id: None,
            error: None,
            candidates: None,
        });
    }

    fn handle_character_appearance(
        &self,
        call: &ToolCall,
        declared_sources: &BTreeMap<String, ProvenanceSource>,
        candidate: &mut BusinessFlowData,
        results: &mut Vec<FieldResult>,
    ) {
        let Some(code) = call
            .parameters
            .get("character_code")
            .or_else(|| call.parameters.get("code"))
            .and_then(|value| value.as_str())
            .map(str::trim)
            .filter(|value| !value.is_empty())
        else {
            results.push(FieldResult {
                field: "character_code".to_string(),
                status: "rejected".to_string(),
                patch_id: None,
                error: Some("missing_target_code".to_string()),
                candidates: None,
            });
            return;
        };
        let source = *declared_sources
            .get("default_summary")
            .or_else(|| declared_sources.get("species"))
            .unwrap_or(&ProvenanceSource::AiGenerated);
        let text = |key: &str| {
            call.parameters
                .get(key)
                .and_then(|value| value.as_str())
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .map(|value| TrackedField::new(value.to_string(), source))
        };
        let section = &mut candidate.optional.character_appearance_phase.character;
        if !section.iter().any(|item| item.code == code) {
            section.push(CharacterAppearanceItem {
                code: code.to_string(),
                ..Default::default()
            });
        }
        let item = section.iter_mut().find(|item| item.code == code).unwrap();
        if let Some(value) = text("species") { item.species = value; }
        if let Some(value) = text("birth") { item.birth = value; }
        if let Some(value) = text("outfit_style") { item.outfit_style = value; }
        if let Some(value) = text("default_summary") { item.default_summary = value; }
        if let Some(value) = call.parameters.get("height_cm").and_then(|value| value.as_u64()) {
            item.height_cm = TrackedField::new(value.min(u32::MAX as u64) as u32, source);
        }
        if let Some(value) = call.parameters.get("weight_kg").and_then(|value| value.as_u64()) {
            item.weight_kg = TrackedField::new(value.min(u32::MAX as u64) as u32, source);
        }
        results.push(FieldResult {
            field: "character_code".to_string(),
            status: "applied".to_string(),
            patch_id: None,
            error: None,
            candidates: Some(vec![code.to_string()]),
        });
    }

    fn handle_character_psychological(
        &self,
        call: &ToolCall,
        declared_sources: &BTreeMap<String, ProvenanceSource>,
        candidate: &mut BusinessFlowData,
        results: &mut Vec<FieldResult>,
    ) {
        let Some(code) = call
            .parameters
            .get("character_code")
            .or_else(|| call.parameters.get("code"))
            .and_then(|value| value.as_str())
            .map(str::trim)
            .filter(|value| !value.is_empty())
        else {
            results.push(FieldResult {
                field: "character_code".to_string(),
                status: "rejected".to_string(),
                patch_id: None,
                error: Some("missing_target_code".to_string()),
                candidates: None,
            });
            return;
        };
        let source = *declared_sources
            .get("the_wound")
            .or_else(|| declared_sources.get("external_want"))
            .or_else(|| declared_sources.get("internal_need"))
            .unwrap_or(&ProvenanceSource::AiGenerated);
        let text = |key: &str| {
            call.parameters
                .get(key)
                .and_then(|value| value.as_str())
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .map(|value| TrackedField::new(value.to_string(), source))
        };
        let section = &mut candidate.optional.character_psychological_phase.character;
        if !section.iter().any(|item| item.code == code) {
            section.push(CharacterPsychologicalItem {
                code: code.to_string(),
                ..Default::default()
            });
        }
        let item = section.iter_mut().find(|item| item.code == code).unwrap();
        if let Some(value) = text("the_wound") { item.core_trauma_and_belief.the_wound = value; }
        if let Some(value) = text("the_lie") { item.core_trauma_and_belief.the_lie = value; }
        if let Some(value) = text("fundamental_fear") { item.core_trauma_and_belief.fundamental_fear = value; }
        if let Some(value) = text("external_want") { item.narrative_engine.external_want = value; }
        if let Some(value) = text("internal_need") { item.narrative_engine.internal_need = value; }
        if let Some(value) = text("internal_conflict") { item.narrative_engine.internal_conflict = value; }
        results.push(FieldResult {
            field: "character_code".to_string(),
            status: "applied".to_string(),
            patch_id: None,
            error: None,
            candidates: Some(vec![code.to_string()]),
        });
    }

    /// Persist an author-approved outline as a durable planning decision. Chat may present the
    /// outline as prose; this tool is the handoff that later phases (and the workflow, which has
    /// no chat history) can read from `decision_log`.
    fn handle_save_approved_outline(
        &self,
        call: &ToolCall,
        declared_sources: &BTreeMap<String, ProvenanceSource>,
        candidate: &mut BusinessFlowData,
        results: &mut Vec<FieldResult>,
    ) {
        let summary = call
            .parameters
            .get("summary")
            .and_then(|value| value.as_str())
            .map(str::trim)
            .filter(|value| !value.is_empty());
        let Some(summary) = summary else {
            results.push(FieldResult {
                field: "summary".to_string(),
                status: "rejected".to_string(),
                patch_id: None,
                error: Some("missing_summary".to_string()),
                candidates: None,
            });
            return;
        };
        let source = *declared_sources
            .get("summary")
            .unwrap_or(&ProvenanceSource::AiGenerated);
        let summary: String = summary.chars().take(20_000).collect();
        if let Some(existing) = candidate
            .decision_log
            .iter()
            .find(|entry| entry.summary == summary)
        {
            results.push(FieldResult {
                field: "summary".to_string(),
                status: "applied".to_string(),
                patch_id: None,
                error: None,
                candidates: Some(vec![existing.id.clone()]),
            });
            return;
        }
        let id = format!("decision-{}", candidate.decision_log.len() + 1);
        candidate.decision_log.push(crate::domain::business_flow::DecisionLogEntry {
            id: id.clone(),
            summary,
            source,
        });
        results.push(FieldResult {
            field: "summary".to_string(),
            status: "applied".to_string(),
            patch_id: None,
            error: None,
            candidates: Some(vec![id]),
        });
    }

    fn handle_add_theme(
        &self,
        call: &ToolCall,
        declared_sources: &BTreeMap<String, ProvenanceSource>,
        _baseline: &BusinessFlowData,
        candidate: &mut BusinessFlowData,
        results: &mut Vec<FieldResult>,
        _pending_patches: &mut Vec<PendingProposal>,
    ) {
        let arc_code = match call.parameters.get("arc_code").and_then(|v| v.as_str()) {
            Some(a) => a,
            None => {
                results.push(FieldResult {
                    field: "arc_code".to_string(),
                    status: "rejected".to_string(),
                    patch_id: None,
                    error: Some("missing_target_code".to_string()),
                    candidates: None,
                });
                return;
            }
        };

        let arc = match candidate.optional.overview_phase.arcs.iter_mut().find(|a| a.code == arc_code) {
            Some(a) => a,
            None => {
                results.push(FieldResult {
                    field: "arc_code".to_string(),
                    status: "rejected".to_string(),
                    patch_id: None,
                    error: Some("target_code_not_found".to_string()),
                    candidates: None,
                });
                return;
            }
        };

        let source = *declared_sources.get("question").unwrap_or(&ProvenanceSource::AiGenerated);
        let text = |key: &str| {
            call.parameters
                .get(key)
                .and_then(|value| value.as_str())
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .map(|value| TrackedField::new(value.to_string(), source))
                .unwrap_or_default()
        };
        let theme_code = call
            .parameters
            .get("theme_code")
            .and_then(|value| value.as_str())
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(str::to_string)
            .unwrap_or_else(|| format!("{arc_code}-theme"));
        arc.themes.code = Some(theme_code.clone());
        arc.themes.parent_theme_code = text("parent_theme_code");
        arc.themes.question = text("question");
        arc.themes.message = text("message");
        arc.themes.resolution.resolution_type = text("resolution_type");
        arc.themes.resolution.content = text("resolution");
        arc.themes.source = Some(source);

        results.push(FieldResult {
            field: "theme_code".to_string(),
            status: "applied".to_string(),
            patch_id: None,
            error: None,
            candidates: Some(vec![theme_code]),
        });
    }

    fn handle_add_theme_argument(
        &self,
        call: &ToolCall,
        declared_sources: &BTreeMap<String, ProvenanceSource>,
        _baseline: &BusinessFlowData,
        candidate: &mut BusinessFlowData,
        results: &mut Vec<FieldResult>,
        _pending_patches: &mut Vec<PendingProposal>,
    ) {
        let theme_code = call.parameters.get("theme_code").and_then(|v| v.as_str());
        let char_code = call.parameters.get("character_code").and_then(|v| v.as_str());

        if theme_code.is_none() || char_code.is_none() {
            results.push(FieldResult {
                field: "target".to_string(),
                status: "rejected".to_string(),
                patch_id: None,
                error: Some("missing_target_code".to_string()),
                candidates: None,
            });
            return;
        }

        let (theme_code, char_code) = (theme_code.unwrap(), char_code.unwrap());
        let source = *declared_sources
            .get("role")
            .unwrap_or(&ProvenanceSource::AiGenerated);
        let text = |key: &str| {
            call.parameters
                .get(key)
                .and_then(|value| value.as_str())
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .map(|value| TrackedField::new(value.to_string(), source))
                .unwrap_or_default()
        };
        let arc = candidate.optional.overview_phase.arcs.iter_mut().find(|a| a.themes.code.as_deref() == Some(theme_code) || a.code == theme_code);

        match arc {
            Some(a) => {
                let code = format!("{theme_code}-arg-{}", a.themes.arguments.len() + 1);
                a.themes.arguments.push(ThemeArgumentItem {
                    code: Some(code.clone()),
                    character_code: TrackedField::new(char_code.to_string(), source),
                    character_display_name: text("character_display_name"),
                    role: text("role"),
                    answer_mode: text("answer_mode"),
                    note: text("note"),
                    required_knowledge: text("required_knowledge"),
                    required_competence: text("required_competence"),
                    source: Some(source),
                });
                results.push(FieldResult {
                    field: "code".to_string(),
                    status: "applied".to_string(),
                    patch_id: None,
                    error: None,
                    candidates: Some(vec![code]),
                });
            }
            None => {
                results.push(FieldResult {
                    field: "theme_code".to_string(),
                    status: "rejected".to_string(),
                    patch_id: None,
                    error: Some("target_code_not_found".to_string()),
                    candidates: None,
                });
            }
        }
    }
}

// ============================================================================
// Two-Phase Per-Field Evaluation Helper
// ============================================================================

fn field_rejection(field: &str, error: &str) -> FieldResult {
    FieldResult {
        field: field.to_string(), status: "rejected".to_string(),
        patch_id: None, error: Some(error.to_string()), candidates: None,
    }
}

fn unique_child_code<'a>(
    existing: impl Iterator<Item = &'a str>,
    prefix: &str,
    mut next: usize,
) -> String {
    let taken: std::collections::BTreeSet<&str> = existing.collect();
    loop {
        let candidate = format!("{prefix}{next}");
        if !taken.contains(candidate.as_str()) {
            return candidate;
        }
        next += 1;
    }
}

fn evaluate_single_field<T: Clone + PartialEq + Serialize>(
    caller: CallerCategory,
    target_type: &str,
    target_code: &str,
    field_name: &str,
    new_value: T,
    baseline_field: &TrackedField<T>,
    candidate_field: &mut TrackedField<T>,
    declared_sources: &BTreeMap<String, ProvenanceSource>,
    pending_patches: &mut Vec<PendingProposal>,
) -> FieldResult {
    // 1. Provenance declaration check
    let declared_source = if caller.is_ai() {
        match declared_sources.get(field_name) {
            Some(src) if matches!(src, ProvenanceSource::User | ProvenanceSource::AiGenerated) => {
                *src
            }
            _ => {
                return FieldResult {
                    field: field_name.to_string(),
                    status: "rejected".to_string(),
                    patch_id: None,
                    error: Some("missing_or_invalid_declared_source".to_string()),
                    candidates: None,
                };
            }
        }
    } else {
        *declared_sources.get(field_name).unwrap_or(&ProvenanceSource::User)
    };

    // 2. Provenance overwrite rules (be-core.md §5.3.1)
    let current_source = baseline_field.source;
    let current_value = baseline_field.value.as_ref();

    match (current_value, current_source) {
        // Case A: Field currently unset / null -> directly apply
        (None, _) => {
            candidate_field.value = Some(new_value);
            candidate_field.source = Some(declared_source);
            FieldResult {
                field: field_name.to_string(),
                status: "applied".to_string(),
                patch_id: None,
                error: None,
                candidates: None,
            }
        }

        // Case B: Field current source is ai_generated
        (Some(_), Some(ProvenanceSource::AiGenerated)) => {
            if caller.is_user_or_cli() || declared_source == ProvenanceSource::User {
                // User edited AI content -> upgrade to user_edited
                candidate_field.value = Some(new_value);
                candidate_field.source = Some(ProvenanceSource::UserEdited);
            } else {
                // AI freely overwriting AI content
                candidate_field.value = Some(new_value);
                candidate_field.source = Some(ProvenanceSource::AiGenerated);
            }
            FieldResult {
                field: field_name.to_string(),
                status: "applied".to_string(),
                patch_id: None,
                error: None,
                candidates: None,
            }
        }

        // Case C: Field current source is user or user_edited (protected)
        (Some(curr_val), Some(ProvenanceSource::User | ProvenanceSource::UserEdited)) => {
            if caller.is_user_or_cli() {
                // UI / CLI user can update directly
                candidate_field.value = Some(new_value);
                candidate_field.source = Some(ProvenanceSource::UserEdited);
                FieldResult {
                    field: field_name.to_string(),
                    status: "applied".to_string(),
                    patch_id: None,
                    error: None,
                    candidates: None,
                }
            } else {
                // AI caller: cannot overwrite directly
                if curr_val == &new_value {
                    // No-op
                    FieldResult {
                        field: field_name.to_string(),
                        status: "applied".to_string(),
                        patch_id: None,
                        error: None,
                        candidates: None,
                    }
                } else {
                    // Create pending proposal patch
                    let patch_id = format!("patch_{}_{}", target_code, field_name);
                    let proposal = PendingProposal {
                        patch_id: patch_id.clone(),
                        operation: "set".to_string(),
                        target_type: target_type.to_string(),
                        target_code: target_code.to_string(),
                        field_name: Some(field_name.to_string()),
                        previous_value: Some(serde_json::json!(curr_val)),
                        new_value: Some(serde_json::json!(new_value)),
                        status: "pending".to_string(),
                    };
                    pending_patches.push(proposal);

                    FieldResult {
                        field: field_name.to_string(),
                        status: "pending_confirmation".to_string(),
                        patch_id: Some(patch_id),
                        error: None,
                        candidates: None,
                    }
                }
            }
        }

        // Other / fallback
        (Some(_), _) => {
            candidate_field.value = Some(new_value);
            candidate_field.source = Some(declared_source);
            FieldResult {
                field: field_name.to_string(),
                status: "applied".to_string(),
                patch_id: None,
                error: None,
                candidates: None,
            }
        }
    }
}

// ============================================================================
// Helper extraction routines
// ============================================================================

#[cfg(test)]
mod project_target_regression_tests {
    use super::*;
    use serde_json::{json, Value};

    fn execute(data: &BusinessFlowData, args: Value) -> ToolOutcome {
        ToolExecutor::execute(&ToolCall::new("set_project", CallerCategory::AiCaller, args), &ExecutionGuard::new(data))
    }

    #[test]
    fn test_project_targets_from_failed_chat_are_saved() {
        let data = BusinessFlowData::new();
        let result = execute(&data, json!({"target": {"arc_count": 1, "chapter_count": 120, "word_per_chapter_count": 2400},
            "declared_sources": {"target.arc_count": "user", "target.chapter_count": "user", "target.word_per_chapter_count": "user"}}));
        assert!(result.is_applied(), "{result:?}");
        let target = &result.mutated_state().unwrap().require.project.target;
        assert_eq!(target.arc_count, TrackedField::user(1));
        assert_eq!(target.chapter_count, TrackedField::user(120));
        assert_eq!(target.word_per_chapter_count, TrackedField::user(2400));
    }

    #[test]
    fn test_project_targets_reject_invalid_values_per_field() {
        for invalid in [json!(0), json!(-1), json!(2.5), json!("120"), json!(4294967296_u64), Value::Null] {
            let result = execute(&BusinessFlowData::new(), json!({"target": {"arc_count": 1, "chapter_count": invalid},
                "declared_sources": {"target.arc_count": "user", "target.chapter_count": "user"}}));
            assert!(result.is_applied());
            assert_eq!(result.mutated_state().unwrap().require.project.target.chapter_count.value, None);
            assert!(result.results.iter().any(|r| r.field == "target.chapter_count" && r.error.as_deref() == Some("expected_positive_integer")));
        }
    }

    #[test]
    fn test_project_targets_require_source_and_protect_user_value() {
        let mut data = BusinessFlowData::new();
        let missing = execute(&data, json!({"target": {"chapter_count": 120}}));
        assert_eq!(missing.results[0].error.as_deref(), Some("missing_or_invalid_declared_source"));
        data.require.project.target.chapter_count = TrackedField::user(120);
        let pending = execute(&data, json!({"target": {"chapter_count": 122}, "declared_sources": {"target.chapter_count": "user"}}));
        assert!(pending.is_pending());
        let patch = &pending.pending_patches()[0];
        assert_eq!(patch.target_code, "require.project");
        assert_eq!(patch.previous_value, Some(json!(120)));
        assert_eq!(patch.new_value, Some(json!(122)));
        let repeat = execute(&data, json!({"target": {"chapter_count": 120}, "declared_sources": {"target.chapter_count": "user"}}));
        assert!(repeat.is_applied());
        assert!(repeat.pending_patches().is_empty());
    }

    #[test]
    fn test_project_rejects_unknown_or_empty_arguments_with_diagnostics() {
        for args in [json!({}), json!({"target": {"chapters": 120}}), json!({"summary": 123})] {
            let result = execute(&BusinessFlowData::new(), args);
            assert!(result.is_rejected());
            assert!(!result.results.is_empty());
            assert!(result.results.iter().all(|r| r.error.is_some()));
        }
    }
}

fn extract_declared_sources(call: &ToolCall) -> BTreeMap<String, ProvenanceSource> {
    if let Some(ds) = &call.declared_sources {
        return ds.clone();
    }

    if let Some(obj) = call.parameters.get("declared_sources").and_then(|v| v.as_object()) {
        let mut map = BTreeMap::new();
        for (k, v) in obj {
            if let Some(s) = v.as_str() {
                match s {
                    "user" => {
                        map.insert(k.clone(), ProvenanceSource::User);
                    }
                    "ai_generated" | "ai" => {
                        map.insert(k.clone(), ProvenanceSource::AiGenerated);
                    }
                    "user_edited" => {
                        map.insert(k.clone(), ProvenanceSource::UserEdited);
                    }
                    _ => {}
                }
            }
        }
        return map;
    }

    BTreeMap::new()
}

fn extract_scalar_declared_source(call: &ToolCall) -> Option<ProvenanceSource> {
    if let Some(s) = call.declared_source {
        return Some(s);
    }

    call.parameters
        .get("declared_source")
        .and_then(|v| v.as_str())
        .and_then(|s| match s {
            "user" => Some(ProvenanceSource::User),
            "ai_generated" | "ai" => Some(ProvenanceSource::AiGenerated),
            "user_edited" => Some(ProvenanceSource::UserEdited),
            _ => None,
        })
}

fn extract_delete_target(tool_name: &str, params: &serde_json::Value) -> Option<(String, String)> {
    match tool_name {
        "delete_character" => {
            let code = params.get("code").or_else(|| params.get("character_code"))?.as_str()?;
            Some(("character".to_string(), code.to_string()))
        }
        "delete_arc" => {
            let code = params.get("code").or_else(|| params.get("arc_code"))?.as_str()?;
            Some(("arc".to_string(), code.to_string()))
        }
        "delete_chapter" => {
            let code = params.get("code").or_else(|| params.get("chapter_code"))?.as_str()?;
            Some(("chapter".to_string(), code.to_string()))
        }
        "delete_scene" => {
            let code = params.get("code").or_else(|| params.get("scene_code"))?.as_str()?;
            Some(("scene".to_string(), code.to_string()))
        }
        "delete_world_setting" => {
            let code = params.get("code").or_else(|| params.get("world_setting_code"))?.as_str()?;
            Some(("world_setting".to_string(), code.to_string()))
        }
        "delete_lore_entry" => {
            let code = params.get("code").or_else(|| params.get("lore_entry_code"))?.as_str()?;
            Some(("lore_entry".to_string(), code.to_string()))
        }
        "delete_relationship" => {
            let code = params.get("code").or_else(|| params.get("relationship_code"))?.as_str()?;
            Some(("relationship".to_string(), code.to_string()))
        }
        "delete_theme" => {
            let code = params.get("code").or_else(|| params.get("theme_code"))?.as_str()?;
            Some(("theme".to_string(), code.to_string()))
        }
        "delete_scene_character" => {
            let sc = params.get("scene_code")?.as_str()?;
            let ch = params.get("character_code")?.as_str()?;
            Some(("scene_character".to_string(), format!("{sc}:{ch}")))
        }
        "delete_scene_lore_entry" => {
            let sc = params.get("scene_code")?.as_str()?;
            let l = params.get("lore_entry_code")?.as_str()?;
            Some(("scene_lore_entry".to_string(), format!("{sc}:{l}")))
        }
        "delete_project_genre" => {
            let code = params.get("genre_code")?.as_str()?;
            Some(("project_genre".to_string(), code.to_string()))
        }
        "delete_arc_event_block" => {
            let code = params.get("event_block_code").or_else(|| params.get("code"))?.as_str()?;
            Some(("arc_event_block".to_string(), code.to_string()))
        }
        "delete_theme_argument" => {
            let th = params.get("theme_code")?.as_str()?;
            let ch = params.get("character_code")?.as_str()?;
            Some(("theme_argument".to_string(), format!("{th}:{ch}")))
        }
        "delete_character_memory" => {
            let code = params.get("code")?.as_str()?;
            Some(("character_memory".to_string(), code.to_string()))
        }
        "delete_character_competence" => {
            let code = params.get("code").or_else(|| params.get("competence_code"))?.as_str()?;
            Some(("character_competence".to_string(), code.to_string()))
        }
        "delete_relationship_role" => {
            let code = params.get("category_code")?.as_str()?;
            Some(("relationship_role".to_string(), code.to_string()))
        }
        _ => None,
    }
}

/// Populates all write and delete tool metadata into registry (`tools.md` §3.2, §4.2).
pub fn register_all_write_tools(registry: &mut ToolRegistry) {
    let tools = [
        ("add_project", ToolType::Mutation, SourceParamMode::MapDeclaredSources),
        ("set_project", ToolType::Mutation, SourceParamMode::MapDeclaredSources),
        ("add_project_genre", ToolType::Mutation, SourceParamMode::MapDeclaredSources),
        ("delete_project_genre", ToolType::Delete, SourceParamMode::ScalarDeclaredSource),
        ("add_character", ToolType::Mutation, SourceParamMode::MapDeclaredSources),
        ("set_character", ToolType::Mutation, SourceParamMode::MapDeclaredSources),
        ("delete_character", ToolType::Delete, SourceParamMode::ScalarDeclaredSource),
        ("add_arc", ToolType::Mutation, SourceParamMode::MapDeclaredSources),
        ("set_arc", ToolType::Mutation, SourceParamMode::MapDeclaredSources),
        ("set_arc_motif", ToolType::Mutation, SourceParamMode::MapDeclaredSources),
        ("delete_arc", ToolType::Delete, SourceParamMode::ScalarDeclaredSource),
        ("add_chapter", ToolType::Mutation, SourceParamMode::MapDeclaredSources),
        ("set_chapter", ToolType::Mutation, SourceParamMode::MapDeclaredSources),
        ("delete_chapter", ToolType::Delete, SourceParamMode::ScalarDeclaredSource),
        ("add_scene", ToolType::Mutation, SourceParamMode::MapDeclaredSources),
        ("set_scene", ToolType::Mutation, SourceParamMode::MapDeclaredSources),
        ("delete_scene", ToolType::Delete, SourceParamMode::ScalarDeclaredSource),
        ("set_scene_config", ToolType::Mutation, SourceParamMode::MapDeclaredSources),
        ("add_scene_character", ToolType::Mutation, SourceParamMode::MapDeclaredSources),
        ("delete_scene_character", ToolType::Delete, SourceParamMode::ScalarDeclaredSource),
        ("add_scene_lore_entry", ToolType::Mutation, SourceParamMode::MapDeclaredSources),
        ("set_scene_lore_entry", ToolType::Mutation, SourceParamMode::MapDeclaredSources),
        ("delete_scene_lore_entry", ToolType::Delete, SourceParamMode::ScalarDeclaredSource),
        ("add_world_setting", ToolType::Mutation, SourceParamMode::MapDeclaredSources),
        ("set_world_setting", ToolType::Mutation, SourceParamMode::MapDeclaredSources),
        ("delete_world_setting", ToolType::Delete, SourceParamMode::ScalarDeclaredSource),
        ("add_lore_entry", ToolType::Mutation, SourceParamMode::MapDeclaredSources),
        ("set_lore_entry", ToolType::Mutation, SourceParamMode::MapDeclaredSources),
        ("delete_lore_entry", ToolType::Delete, SourceParamMode::ScalarDeclaredSource),
        ("add_arc_event_block", ToolType::Mutation, SourceParamMode::MapDeclaredSources),
        ("delete_arc_event_block", ToolType::Delete, SourceParamMode::ScalarDeclaredSource),
        ("save_approved_outline", ToolType::Mutation, SourceParamMode::MapDeclaredSources),
        ("add_theme", ToolType::Mutation, SourceParamMode::MapDeclaredSources),
        ("set_theme", ToolType::Mutation, SourceParamMode::MapDeclaredSources),
        ("delete_theme", ToolType::Delete, SourceParamMode::ScalarDeclaredSource),
        ("add_theme_argument", ToolType::Mutation, SourceParamMode::MapDeclaredSources),
        ("delete_theme_argument", ToolType::Delete, SourceParamMode::ScalarDeclaredSource),
        ("add_character_appearance", ToolType::Mutation, SourceParamMode::MapDeclaredSources),
        ("set_character_appearance", ToolType::Mutation, SourceParamMode::MapDeclaredSources),
        ("add_character_psychological", ToolType::Mutation, SourceParamMode::MapDeclaredSources),
        ("set_character_psychological", ToolType::Mutation, SourceParamMode::MapDeclaredSources),
        ("add_character_memory", ToolType::Mutation, SourceParamMode::MapDeclaredSources),
        ("set_character_memory", ToolType::Mutation, SourceParamMode::MapDeclaredSources),
        ("delete_character_memory", ToolType::Delete, SourceParamMode::ScalarDeclaredSource),
        ("add_character_competence", ToolType::Mutation, SourceParamMode::MapDeclaredSources),
        ("set_character_competence", ToolType::Mutation, SourceParamMode::MapDeclaredSources),
        ("delete_character_competence", ToolType::Delete, SourceParamMode::ScalarDeclaredSource),
        ("add_relationship", ToolType::Mutation, SourceParamMode::MapDeclaredSources),
        ("set_relationship", ToolType::Mutation, SourceParamMode::MapDeclaredSources),
        ("delete_relationship", ToolType::Delete, SourceParamMode::ScalarDeclaredSource),
        ("set_relationship_state", ToolType::Mutation, SourceParamMode::MapDeclaredSources),
        ("add_relationship_role", ToolType::Mutation, SourceParamMode::MapDeclaredSources),
        ("delete_relationship_role", ToolType::Delete, SourceParamMode::ScalarDeclaredSource),
    ];

    for (name, tool_type, param_mode) in tools {
        if registry.lookup(name).is_none() {
            registry.register(ToolMetadata::new(
                name,
                format!("Tool {name}"),
                tool_type,
                crate::domain::tools::registry::ToolScope::Planning,
                AllowedCaller::AiAndUi,
                param_mode,
                true,
            ));
        }
    }
}

#[cfg(test)]
mod phase6_7_handlers {
    use super::*;

    fn state_with_cast_and_scene() -> BusinessFlowData {
        let mut state = BusinessFlowData::default();
        state.require.character.push(RequireCharacterItem {
            code: "char_a".into(),
            display_name: TrackedField::user("A".to_string()),
            ..Default::default()
        });
        state.require.character.push(RequireCharacterItem {
            code: "char_b".into(),
            display_name: TrackedField::user("B".to_string()),
            ..Default::default()
        });
        state.optional.chapter_phase.arcs.push(ChapterPhaseArcItem {
            code: "arc_one".into(),
            chapters: vec![ChapterPhaseChapterItem {
                code: "arc_one-c1".into(),
                position: TrackedField::user(1),
                scenes: vec![ChapterPhaseSceneItem {
                    code: "arc_one-c1-s1".into(),
                    position: TrackedField::user(1),
                    ..Default::default()
                }],
                ..Default::default()
            }],
        });
        state
    }

    fn sources(pairs: &[(&str, &str)]) -> BTreeMap<String, ProvenanceSource> {
        pairs
            .iter()
            .map(|(field, source)| {
                (
                    (*field).to_string(),
                    if *source == "user" {
                        ProvenanceSource::User
                    } else {
                        ProvenanceSource::AiGenerated
                    },
                )
            })
            .collect()
    }

    fn call(name: &str, parameters: serde_json::Value, declared: &[(&str, &str)]) -> ToolCall {
        ToolCall {
            name: name.into(),
            caller: CallerCategory::AiCaller,
            parameters,
            declared_sources: Some(sources(declared)),
            declared_source: None,
        }
    }

    #[test]
    fn set_arc_motif_assigns_the_missing_phase_one_gate_field() {
        let executor = ToolExecutor::default();
        let mut state = BusinessFlowData::default();
        let add = call(
            "add_arc",
            serde_json::json!({"display_name": "Arc 1", "summary": "S", "position": 1}),
            &[("display_name", "ai_generated")],
        );
        state = executor
            .execute_call(&add, &ExecutionGuard::new(&state))
            .mutated_state()
            .unwrap()
            .clone();
        let code = state.optional.overview_phase.arcs[0].code.clone();

        let set = call(
            "set_arc_motif",
            serde_json::json!({"code": code, "motif_code": "positive_change_arc", "note": "growth"}),
            &[("motif_code", "ai_generated")],
        );
        let outcome = executor.execute_call(&set, &ExecutionGuard::new(&state));
        assert!(outcome.is_applied(), "{outcome:?}");
        let updated = outcome.mutated_state().unwrap().clone();
        assert_eq!(
            updated.optional.overview_phase.arcs[0].motif.code.value.as_deref(),
            Some("positive_change_arc")
        );
        assert_eq!(updated.optional.overview_phase.arcs[0].motif.note.value.as_deref(), Some("growth"));

        let unknown = call(
            "set_arc_motif",
            serde_json::json!({"code": "missing", "motif_code": "positive_change_arc"}),
            &[("motif_code", "ai_generated")],
        );
        assert_eq!(executor.execute_call(&unknown, &ExecutionGuard::new(&state)).status, "rejected");
    }

    #[test]
    fn add_project_genre_accepts_the_map_source_contract() {
        let executor = ToolExecutor::default();
        let mut state = BusinessFlowData::default();
        let add = call(
            "add_project_genre",
            serde_json::json!({"genre_code": "romance", "is_primary": true}),
            &[("genre_code", "user")],
        );
        let outcome = executor.execute_call(&add, &ExecutionGuard::new(&state));
        assert!(outcome.is_applied(), "map declared_sources must apply: {outcome:?}");
        state = outcome.mutated_state().unwrap().clone();
        assert_eq!(state.require.project.genres.len(), 1);
        assert_eq!(
            state.require.project.genres[0].code.value.as_deref(),
            Some("romance")
        );
        assert_eq!(
            state.require.project.genres[0].code.source,
            Some(ProvenanceSource::User)
        );

        // Legacy scalar payloads are tolerated instead of rejecting the whole genre step.
        let legacy = call(
            "add_project_genre",
            serde_json::json!({"genre_code": "urban_dushi", "is_primary": false, "declared_source": "user"}),
            &[],
        );
        let outcome = executor.execute_call(&legacy, &ExecutionGuard::new(&state));
        assert!(outcome.is_applied(), "legacy scalar source must apply: {outcome:?}");
        assert_eq!(outcome.mutated_state().unwrap().require.project.genres.len(), 2);
    }

    #[test]
    fn appearance_and_psychological_handlers_satisfy_the_phase_five_gate() {
        let executor = ToolExecutor::default();
        let mut state = BusinessFlowData::default();
        let mut hero = crate::domain::business_flow::RequireCharacterItem::default();
        hero.code = "hero".into();
        hero.display_name = TrackedField::user("Anh hùng".into());
        state.require.character.push(hero);

        let appearance = call(
            "add_character_appearance",
            serde_json::json!({"character_code": "hero", "default_summary": "Cao, tóc đen", "species": "HUMAN"}),
            &[("default_summary", "ai_generated")],
        );
        let outcome = executor.execute_call(&appearance, &ExecutionGuard::new(&state));
        assert!(outcome.is_applied(), "{outcome:?}");
        state = outcome.mutated_state().unwrap().clone();

        let psychology = call(
            "add_character_psychological",
            serde_json::json!({
                "character_code": "hero",
                "the_wound": "Bị bỏ rơi",
                "external_want": "Được công nhận",
                "internal_need": "Tự chấp nhận"
            }),
            &[("the_wound", "ai_generated")],
        );
        let outcome = executor.execute_call(&psychology, &ExecutionGuard::new(&state));
        assert!(outcome.is_applied(), "{outcome:?}");
        state = outcome.mutated_state().unwrap().clone();

        assert!(
            crate::domain::phase_gate::missing_for_subphase(&state, 1).is_empty(),
            "{:?}",
            crate::domain::phase_gate::missing_for_subphase(&state, 1)
        );
        assert!(
            crate::domain::phase_gate::missing_for_subphase(&state, 2).is_empty(),
            "{:?}",
            crate::domain::phase_gate::missing_for_subphase(&state, 2)
        );
    }

    #[test]
    fn add_arc_merges_duplicate_positions_and_respects_the_target() {
        let executor = ToolExecutor::default();
        let mut state = BusinessFlowData::default();
        state.require.project.target.arc_count = TrackedField::user(2);
        let make = |name: &str, position: u64, motif: &str| {
            call(
                "add_arc",
                serde_json::json!({
                    "display_name": name,
                    "summary": format!("Tóm tắt {name}"),
                    "position": position,
                    "motif_code": motif
                }),
                &[("display_name", "ai_generated"), ("summary", "ai_generated")],
            )
        };

        let first = executor.execute_call(&make("Arc Một", 1, "m1"), &ExecutionGuard::new(&state));
        assert!(first.is_applied(), "{first:?}");
        state = first.mutated_state().unwrap().clone();
        let first_code = state.optional.overview_phase.arcs[0].code.clone();

        // Regenerating the same position merges into the existing Arc instead of appending a copy.
        let again = executor.execute_call(&make("Arc Một (mới)", 1, "m1"), &ExecutionGuard::new(&state));
        assert!(again.is_applied(), "{again:?}");
        state = again.mutated_state().unwrap().clone();
        assert_eq!(state.optional.overview_phase.arcs.len(), 1, "position 1 must stay single");
        assert_eq!(state.optional.overview_phase.arcs[0].code, first_code);
        assert_eq!(
            state.optional.overview_phase.arcs[0].display_name.value.as_deref(),
            Some("Arc Một (mới)")
        );

        let second = executor.execute_call(&make("Arc Hai", 2, "m2"), &ExecutionGuard::new(&state));
        state = second.mutated_state().unwrap().clone();
        assert_eq!(state.optional.overview_phase.arcs.len(), 2);

        // A third Arc exceeds target.arc_count = 2 and is rejected instead of duplicating positions.
        let extra = executor.execute_call(&make("Arc Ba", 3, "m3"), &ExecutionGuard::new(&state));
        assert_eq!(extra.status, "rejected");
        assert_eq!(extra.results[0].error.as_deref(), Some("arc_target_reached"));
    }

    #[test]
    fn approved_outline_is_persisted_to_decision_log_idempotently() {
        let executor = ToolExecutor::default();
        let state = BusinessFlowData::default();
        let outline = call(
            "save_approved_outline",
            serde_json::json!({
                "summary": "5 Arc: Gặp gỡ tình cờ / Thử thách / Khẳng định / Đối đầu / Kết thúc"
            }),
            &[("summary", "user")],
        );
        let outcome = executor.execute_call(&outline, &ExecutionGuard::new(&state));
        assert!(outcome.is_applied(), "approved outline must be saved: {outcome:?}");
        let updated = outcome.mutated_state().unwrap().clone();
        assert_eq!(updated.decision_log.len(), 1);
        assert_eq!(updated.decision_log[0].source, crate::domain::business_flow::ProvenanceSource::User);
        assert!(updated.decision_log[0].summary.contains("5 Arc"));

        // Repeating the same approval reuses the existing decision instead of duplicating it.
        let again = executor.execute_call(&outline, &ExecutionGuard::new(&updated));
        assert!(again.is_applied());
        assert_eq!(again.mutated_state().unwrap().decision_log.len(), 1);

        let blank = call(
            "save_approved_outline",
            serde_json::json!({"summary": "   "}),
            &[("summary", "user")],
        );
        assert_eq!(executor.execute_call(&blank, &ExecutionGuard::new(&state)).status, "rejected");
    }

    #[test]
    fn relationship_accepts_aliases_rejects_ghosts_and_records_all_dimensions() {
        let executor = ToolExecutor::default();
        let mut state = state_with_cast_and_scene();

        let add = call(
            "add_relationship",
            serde_json::json!({"from_code": "char_a", "to_code": "char_b", "motif_code": "allies"}),
            &[("code", "ai_generated")],
        );
        let outcome = executor.execute_call(&add, &ExecutionGuard::new(&state));
        assert!(outcome.is_applied(), "alias pair must be written: {outcome:?}");
        state = outcome.mutated_state().unwrap().clone();
        let code = state.optional.relationship_phase.relationship[0].code.clone();
        assert_eq!(state.optional.relationship_phase.relationship[0].motif.code.value.as_deref(), Some("allies"));

        // Duplicate pair reuses the existing relationship instead of pushing a second one.
        let outcome = executor.execute_call(&add, &ExecutionGuard::new(&state));
        assert!(outcome.is_applied());
        assert_eq!(outcome.mutated_state().unwrap().optional.relationship_phase.relationship.len(), 1);

        let ghost = call(
            "add_relationship",
            serde_json::json!({"from_code": "char_a", "to_code": "char_ghost"}),
            &[("code", "ai_generated")],
        );
        let rejected = executor.execute_call(&ghost, &ExecutionGuard::new(&state));
        assert_eq!(rejected.status, "rejected");
        assert!(rejected.results[0].error.as_deref() == Some("target_code_not_found"));

        let state_call = call(
            "set_relationship_state",
            serde_json::json!({
                "relationship_code": code,
                "scene_code": "arc_one-c1-s1",
                "trust": "HIGH", "affection": "MODERATE", "hostility": "LOW",
                "fear": "NONE", "respect": "HIGH", "dependency": "LOW", "familiarity": "ABSOLUTE"
            }),
            &[
                ("trust", "ai_generated"), ("affection", "ai_generated"), ("hostility", "ai_generated"),
                ("fear", "ai_generated"), ("respect", "ai_generated"), ("dependency", "ai_generated"),
                ("familiarity", "ai_generated"),
            ],
        );
        let outcome = executor.execute_call(&state_call, &ExecutionGuard::new(&state));
        assert!(outcome.is_applied(), "seven dimensions must apply: {outcome:?}");
        let updated = outcome.mutated_state().unwrap().clone();
        let scene_state = &updated.optional.relationship_phase.relationship[0].character.scenes[0];
        assert_eq!(scene_state.familiarity.value.as_deref(), Some("ABSOLUTE"));
        assert_eq!(scene_state.dependency.value.as_deref(), Some("LOW"));

        // Invalid enum and unknown scene are rejected, and the existing values are preserved.
        let invalid = call(
            "set_relationship_state",
            serde_json::json!({"relationship_code": code, "scene_code": "arc_one-c1-s1", "trust": "VERY_HIGH"}),
            &[("trust", "ai_generated")],
        );
        assert_eq!(executor.execute_call(&invalid, &ExecutionGuard::new(&state)).status, "rejected");
        let wrong_scene = call(
            "set_relationship_state",
            serde_json::json!({"relationship_code": code, "scene_code": "missing", "trust": "LOW"}),
            &[("trust", "ai_generated")],
        );
        assert_eq!(executor.execute_call(&wrong_scene, &ExecutionGuard::new(&state)).status, "rejected");
    }

    #[test]
    fn chapters_and_scenes_write_position_summary_and_return_codes() {
        let executor = ToolExecutor::default();
        let mut state = BusinessFlowData::default();
        state.optional.chapter_phase.arcs.push(ChapterPhaseArcItem {
            code: "arc_one".into(),
            chapters: Vec::new(),
        });

        let add_chapter = call(
            "add_chapter",
            serde_json::json!({
                "arc_code": "arc_one", "display_name": "Chương 1",
                "position": 1, "summary": "Mở đầu"
            }),
            &[("display_name", "ai_generated"), ("position", "ai_generated"), ("summary", "ai_generated")],
        );
        let outcome = executor.execute_call(&add_chapter, &ExecutionGuard::new(&state));
        assert!(outcome.is_applied(), "chapter must be written: {outcome:?}");
        assert_eq!(outcome.results[0].candidates.as_ref().unwrap()[0], "arc_one-c1");
        state = outcome.mutated_state().unwrap().clone();
        let chapter = &state.optional.chapter_phase.arcs[0].chapters[0];
        assert_eq!(chapter.position.value, Some(1));
        assert_eq!(chapter.summary.value.as_deref(), Some("Mở đầu"));
        assert_eq!(chapter.display_name.source, Some(ProvenanceSource::AiGenerated));

        let add_scene = call(
            "add_scene",
            serde_json::json!({
                "chapter_code": "arc_one-c1", "display_name": "Cảnh 1",
                "position": 1, "summary": "Gặp gỡ"
            }),
            &[("display_name", "ai_generated"), ("position", "ai_generated"), ("summary", "ai_generated")],
        );
        let outcome = executor.execute_call(&add_scene, &ExecutionGuard::new(&state));
        assert!(outcome.is_applied(), "scene must be written: {outcome:?}");
        let updated = outcome.mutated_state().unwrap().clone();
        let scene = &updated.optional.chapter_phase.arcs[0].chapters[0].scenes[0];
        assert_eq!(scene.position.value, Some(1));
        assert_eq!(scene.summary.value.as_deref(), Some("Gặp gỡ"));
    }
}

#[cfg(test)]
mod phase1_handlers {
    use super::*;

    fn call(name: &str, parameters: serde_json::Value) -> ToolCall {
        ToolCall {
            name: name.into(),
            caller: CallerCategory::AiCaller,
            parameters,
            declared_sources: Some(
                [("display_name".to_string(), ProvenanceSource::AiGenerated),
                 ("question".to_string(), ProvenanceSource::AiGenerated),
                 ("summary".to_string(), ProvenanceSource::AiGenerated),
                 ("role".to_string(), ProvenanceSource::AiGenerated)]
                    .into_iter()
                    .collect(),
            ),
            declared_source: None,
        }
    }

    #[test]
    fn arc_theme_arguments_and_event_blocks_are_written_with_all_fields() {
        let executor = ToolExecutor::default();
        let mut state = BusinessFlowData::default();
        state.require.character.push(RequireCharacterItem {
            code: "char_a".into(),
            display_name: TrackedField::user("A".to_string()),
            ..Default::default()
        });

        let outcome = executor.execute_call(
            &call("add_arc", serde_json::json!({
                "code": "arc-1", "display_name": "Khởi đầu", "summary": "Mở màn",
                "position": 1, "motif_code": "opening"
            })),
            &ExecutionGuard::new(&state),
        );
        assert!(outcome.is_applied(), "arc must apply: {outcome:?}");
        state = outcome.mutated_state().unwrap().clone();
        let arc = &state.optional.overview_phase.arcs[0];
        assert_eq!(arc.position.value, Some(1));
        assert_eq!(arc.summary.value.as_deref(), Some("Mở màn"));
        assert_eq!(arc.motif.code.value.as_deref(), Some("opening"));
        assert_eq!(state.optional.chapter_phase.arcs.len(), 1, "chapter container created");

        let outcome = executor.execute_call(
            &call("add_theme", serde_json::json!({
                "arc_code": "arc-1", "theme_code": "theme-1",
                "question": "Tham vọng có đáng đánh đổi?", "message": "Cái giá của tham vọng",
                "resolution": "Chọn buông bỏ", "resolution_type": "RESOLVED"
            })),
            &ExecutionGuard::new(&state),
        );
        assert!(outcome.is_applied(), "theme must apply: {outcome:?}");
        state = outcome.mutated_state().unwrap().clone();
        let theme = &state.optional.overview_phase.arcs[0].themes;
        assert_eq!(theme.code.as_deref(), Some("theme-1"));
        assert_eq!(theme.question.value.as_deref(), Some("Tham vọng có đáng đánh đổi?"));
        assert_eq!(theme.message.value.as_deref(), Some("Cái giá của tham vọng"));
        assert_eq!(theme.resolution.resolution_type.value.as_deref(), Some("RESOLVED"));
        assert_eq!(theme.resolution.content.value.as_deref(), Some("Chọn buông bỏ"));

        let outcome = executor.execute_call(
            &call("add_theme_argument", serde_json::json!({
                "theme_code": "theme-1", "character_code": "char_a",
                "role": "PRIMARY", "answer_mode": "ACTION", "note": "Hành động"
            })),
            &ExecutionGuard::new(&state),
        );
        assert!(outcome.is_applied(), "argument must apply: {outcome:?}");
        state = outcome.mutated_state().unwrap().clone();
        let argument = &state.optional.overview_phase.arcs[0].themes.arguments[0];
        assert_eq!(argument.role.value.as_deref(), Some("PRIMARY"));
        assert_eq!(argument.answer_mode.value.as_deref(), Some("ACTION"));

        let outcome = executor.execute_call(
            &call("add_arc_event_block", serde_json::json!({
                "arc_code": "arc-1", "start_chapter": 1, "end_chapter": 25,
                "summary": "Khối mở đầu"
            })),
            &ExecutionGuard::new(&state),
        );
        assert!(outcome.is_applied(), "event block must apply: {outcome:?}");
        state = outcome.mutated_state().unwrap().clone();
        let block = &state.optional.overview_phase.arcs[0].motif.event_blocks[0];
        assert_eq!(block.start_chapter.value, Some(1));
        assert_eq!(block.end_chapter.value, Some(25));
        assert_eq!(block.summary.value.as_deref(), Some("Khối mở đầu"));
        assert_eq!(block.code, "arc-1-eb-1");
    }
}
