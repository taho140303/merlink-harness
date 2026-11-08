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
use serde_json::Value;
use std::collections::BTreeMap;

use crate::domain::business_flow::{
    ArcEventBlockItem, ArcMotifSection, ArcThemeSection, BusinessFlowData,
    CharacterCompetenceItem, CharacterLifeExperienceItem,
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

                let res = evaluate_single_string_field(
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

        let str_fields = [
            ("display_name", &base_proj.display_name, &mut cand_proj.display_name),
            ("summary", &base_proj.summary, &mut cand_proj.summary),
            ("language", &base_proj.language, &mut cand_proj.language),
        ];

        for (f_name, base_f, cand_f) in str_fields {
            if let Some(v) = call.parameters.get(f_name) {
                if let Some(s) = v.as_str() {
                    let res = evaluate_single_string_field(
                        call.caller,
                        "project",
                        "project",
                        f_name,
                        s.to_string(),
                        base_f,
                        cand_f,
                        declared_sources,
                        pending_patches,
                    );
                    results.push(res);
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

    fn handle_add_arc(
        &self,
        call: &ToolCall,
        _declared_sources: &BTreeMap<String, ProvenanceSource>,
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
            .unwrap_or_else(|| format!("arc-{}", candidate.optional.overview_phase.arcs.len() + 1));

        let mut arc = OverviewArcItem {
            code: code.clone(),
            display_name: TrackedField::unset(),
            summary: TrackedField::unset(),
            position: TrackedField::unset(),
            motif: ArcMotifSection::default(),
            themes: ArcThemeSection::default(),
            source: Some(ProvenanceSource::User),
        };

        if let Some(v) = call.parameters.get("display_name").and_then(|v| v.as_str()) {
            arc.display_name = TrackedField::user(v.to_string());
            results.push(FieldResult {
                field: "display_name".to_string(),
                status: "applied".to_string(),
                patch_id: None,
                error: None,
                candidates: None,
            });
        }

        candidate.optional.overview_phase.arcs.push(arc);
        candidate.optional.chapter_phase.arcs.push(ChapterPhaseArcItem {
            code: code.clone(),
            chapters: Vec::new(),
        });
        candidate.optional.scene_config_phase.arcs.push(SceneConfigArcItem {
            code,
            chapters: Vec::new(),
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
            let res = evaluate_single_string_field(
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
        _declared_sources: &BTreeMap<String, ProvenanceSource>,
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

        let code = call
            .parameters
            .get("code")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| format!("{}-c{}", arc_code, arc_chap.chapters.len() + 1));

        let chap = ChapterPhaseChapterItem {
            code: code.clone(),
            display_name: TrackedField::user(
                call.parameters.get("display_name").and_then(|v| v.as_str()).unwrap_or("Chapter").to_string(),
            ),
            position: TrackedField::unset(),
            summary: TrackedField::unset(),
            scenes: Vec::new(),
            source: Some(ProvenanceSource::User),
        };

        arc_chap.chapters.push(chap);
        results.push(FieldResult {
            field: "code".to_string(),
            status: "applied".to_string(),
            patch_id: None,
            error: None,
            candidates: None,
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
            let res = evaluate_single_string_field(
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
    }

    fn handle_add_scene(
        &self,
        call: &ToolCall,
        _declared_sources: &BTreeMap<String, ProvenanceSource>,
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

        let code = call
            .parameters
            .get("code")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| format!("{}-s{}", chap_code, chap.scenes.len() + 1));

        let ws_code = call.parameters.get("world_setting_code").and_then(|v| v.as_str()).map(|s| s.to_string());

        let sc = ChapterPhaseSceneItem {
            code: code.clone(),
            display_name: TrackedField::user(
                call.parameters.get("display_name").and_then(|v| v.as_str()).unwrap_or("Scene").to_string(),
            ),
            position: TrackedField::unset(),
            summary: TrackedField::unset(),
            world_setting_code: ws_code.map(TrackedField::user).unwrap_or_else(TrackedField::unset),
            source: Some(ProvenanceSource::User),
        };

        chap.scenes.push(sc);
        results.push(FieldResult {
            field: "code".to_string(),
            status: "applied".to_string(),
            patch_id: None,
            error: None,
            candidates: None,
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
            let res = evaluate_single_string_field(
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
            let res = evaluate_single_string_field(
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

            let res = evaluate_single_string_field(
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
        _declared_sources: &BTreeMap<String, ProvenanceSource>,
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

        let ws = WorldSettingItem {
            code: code.clone(),
            parent_code: parent_code.map(TrackedField::user).unwrap_or_else(TrackedField::unset),
            display_name: TrackedField::user(disp),
            setting_type: TrackedField::user(st_type),
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
            let res = evaluate_single_string_field(
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
        _declared_sources: &BTreeMap<String, ProvenanceSource>,
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

        let lore = LoreEntryItem {
            code: code.clone(),
            display_name: TrackedField::user(
                call.parameters.get("display_name").and_then(|v| v.as_str()).unwrap_or("Lore").to_string(),
            ),
            lore_type: TrackedField::user(
                call.parameters.get("type").and_then(|v| v.as_str()).unwrap_or("CONCEPT").to_string(),
            ),
            description: TrackedField::unset(),
            data: TrackedField::unset(),
            source: Some(ProvenanceSource::User),
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
            let res = evaluate_single_string_field(
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
        _declared_sources: &BTreeMap<String, ProvenanceSource>,
        _baseline: &BusinessFlowData,
        candidate: &mut BusinessFlowData,
        results: &mut Vec<FieldResult>,
        _pending_patches: &mut Vec<PendingProposal>,
    ) {
        let from = call.parameters.get("from_character_code").and_then(|v| v.as_str());
        let to = call.parameters.get("to_character_code").and_then(|v| v.as_str());

        if from.is_none() || to.is_none() {
            results.push(FieldResult {
                field: "characters".to_string(),
                status: "rejected".to_string(),
                patch_id: None,
                error: Some("missing_target_code".to_string()),
                candidates: None,
            });
            return;
        }
        let (from, to) = (from.unwrap(), to.unwrap());

        let code = call
            .parameters
            .get("code")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| format!("rel_{}_{}", from, to));

        let rel = RelationshipPhaseItem {
            code: code.clone(),
            motif: RelationshipMotifSection::default(),
            character: RelationshipCharacterPairSection {
                from_code: from.to_string(),
                to_code: to.to_string(),
                scenes: Vec::new(),
                roles: Vec::new(),
            },
            source: Some(ProvenanceSource::User),
        };

        candidate.optional.relationship_phase.relationship.push(rel);
        results.push(FieldResult {
            field: "code".to_string(),
            status: "applied".to_string(),
            patch_id: None,
            error: None,
            candidates: None,
        });
    }

    fn handle_set_relationship_state(
        &self,
        call: &ToolCall,
        _declared_sources: &BTreeMap<String, ProvenanceSource>,
        _baseline: &BusinessFlowData,
        candidate: &mut BusinessFlowData,
        results: &mut Vec<FieldResult>,
        _pending_patches: &mut Vec<PendingProposal>,
    ) {
        let rel_code = call.parameters.get("relationship_code").and_then(|v| v.as_str());
        let sc_code = call.parameters.get("scene_code").and_then(|v| v.as_str());

        if rel_code.is_none() || sc_code.is_none() {
            results.push(FieldResult {
                field: "relationship_code".to_string(),
                status: "rejected".to_string(),
                patch_id: None,
                error: Some("missing_target_code".to_string()),
                candidates: None,
            });
            return;
        }

        let (rel_code, sc_code) = (rel_code.unwrap(), sc_code.unwrap());
        let rel = match candidate.optional.relationship_phase.relationship.iter_mut().find(|r| r.code == rel_code) {
            Some(r) => r,
            None => {
                results.push(FieldResult {
                    field: "relationship_code".to_string(),
                    status: "rejected".to_string(),
                    patch_id: None,
                    error: Some("target_code_not_found".to_string()),
                    candidates: None,
                });
                return;
            }
        };

        let trust = call.parameters.get("trust").and_then(|v| v.as_str()).map(|s| s.to_string());
        let affection = call.parameters.get("affection").and_then(|v| v.as_str()).map(|s| s.to_string());

        let state_item = RelationshipSceneStateItem {
            scene_code: sc_code.to_string(),
            trust: trust.map(TrackedField::user).unwrap_or_else(TrackedField::unset),
            affection: affection.map(TrackedField::user).unwrap_or_else(TrackedField::unset),
            hostility: TrackedField::unset(),
            fear: TrackedField::unset(),
            respect: TrackedField::unset(),
            dependency: TrackedField::unset(),
            familiarity: TrackedField::unset(),
            source: Some(ProvenanceSource::User),
        };

        rel.character.scenes.retain(|s| s.scene_code != sc_code);
        rel.character.scenes.push(state_item);

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
        _declared_sources: &BTreeMap<String, ProvenanceSource>,
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

        candidate.require.project.genres.push(RequireGenreItem {
            code: TrackedField::user(genre_code.to_string()),
            is_primary: TrackedField::user(
                call.parameters.get("is_primary").and_then(|v| v.as_bool()).unwrap_or(false),
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
        _declared_sources: &BTreeMap<String, ProvenanceSource>,
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

        let code = call
            .parameters
            .get("event_block_code")
            .and_then(|v| v.as_str())
            .unwrap_or("eb-1");

        arc.motif.event_blocks.push(ArcEventBlockItem {
            code: code.to_string(),
            event_block_code: TrackedField::user(code.to_string()),
            start_chapter_code: TrackedField::unset(),
            end_chapter_code: TrackedField::unset(),
            start_chapter: TrackedField::unset(),
            end_chapter: TrackedField::unset(),
            summary: TrackedField::unset(),
            source: Some(ProvenanceSource::User),
        });

        results.push(FieldResult {
            field: "event_block_code".to_string(),
            status: "applied".to_string(),
            patch_id: None,
            error: None,
            candidates: None,
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

    fn handle_add_theme(
        &self,
        call: &ToolCall,
        _declared_sources: &BTreeMap<String, ProvenanceSource>,
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

        let question = call.parameters.get("question").and_then(|v| v.as_str()).unwrap_or("");
        arc.themes.question = TrackedField::user(question.to_string());
        arc.themes.source = Some(ProvenanceSource::User);

        results.push(FieldResult {
            field: "question".to_string(),
            status: "applied".to_string(),
            patch_id: None,
            error: None,
            candidates: None,
        });
    }

    fn handle_add_theme_argument(
        &self,
        call: &ToolCall,
        _declared_sources: &BTreeMap<String, ProvenanceSource>,
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
        let arc = candidate.optional.overview_phase.arcs.iter_mut().find(|a| a.themes.code.as_deref() == Some(theme_code) || a.code == theme_code);

        match arc {
            Some(a) => {
                a.themes.arguments.push(ThemeArgumentItem {
                    code: Some(format!("arg-{}", a.themes.arguments.len() + 1)),
                    character_code: TrackedField::user(char_code.to_string()),
                    character_display_name: TrackedField::unset(),
                    role: TrackedField::unset(),
                    answer_mode: TrackedField::unset(),
                    note: TrackedField::unset(),
                    required_knowledge: TrackedField::unset(),
                    required_competence: TrackedField::unset(),
                    source: Some(ProvenanceSource::User),
                });
                results.push(FieldResult {
                    field: "character_code".to_string(),
                    status: "applied".to_string(),
                    patch_id: None,
                    error: None,
                    candidates: None,
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

fn evaluate_single_string_field(
    caller: CallerCategory,
    target_type: &str,
    target_code: &str,
    field_name: &str,
    new_value: String,
    baseline_field: &TrackedField<String>,
    candidate_field: &mut TrackedField<String>,
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
                        previous_value: Some(Value::String(curr_val.clone())),
                        new_value: Some(Value::String(new_value)),
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
        ("add_theme", ToolType::Mutation, SourceParamMode::MapDeclaredSources),
        ("set_theme", ToolType::Mutation, SourceParamMode::MapDeclaredSources),
        ("delete_theme", ToolType::Delete, SourceParamMode::ScalarDeclaredSource),
        ("add_theme_argument", ToolType::Mutation, SourceParamMode::MapDeclaredSources),
        ("delete_theme_argument", ToolType::Delete, SourceParamMode::ScalarDeclaredSource),
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
