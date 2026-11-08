//! Tool Registry & Caller Authorization Gating (`REQ-TOOLS-003`, `be-core.md` §5.1–§5.2, `harness/tools.md` §1).
//!
//! Enforces:
//! 1. Metadata registration for all backend tools (code/name, scope, tool_type, allowed_caller, source_param_mode, writes_draft_state).
//! 2. Caller authorization gate: AI callers are strictly prevented from invoking UI/CLI-only actions
//!    (`confirm_phase`, `set_scene_lock`, `lock_scene`, `unlock_scene`, `commit_draft`, etc.), returning
//!    `ErrorCode::UnauthorizedCaller`.
//! 3. Domain invariant: Read tools never create draft versions (`writes_draft_state = false`).

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::error::ErrorCode;

/// Category of caller invoking a tool (`be-core.md` §5.1, `harness/tools.md` §1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CallerCategory {
    /// AI agent / model calling via function call (`"ai_caller"`).
    AiCaller,
    /// User interaction on the UI frontend (`"ui_action"`).
    UiAction,
    /// CLI developer / testing tool invocation (`"cli"`).
    Cli,
}

impl CallerCategory {
    pub const fn is_ai(self) -> bool {
        matches!(self, CallerCategory::AiCaller)
    }

    pub const fn is_user_or_cli(self) -> bool {
        matches!(self, CallerCategory::UiAction | CallerCategory::Cli)
    }
}

/// Allowed caller permissions on a tool (`be-core.md` §5.1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AllowedCaller {
    /// Tool accessible only by AI callers (e.g. `get_field_null`, `get_array_element_fields`).
    AiOnly,
    /// Tool accessible by both AI and UI/CLI callers (e.g. read tools, entity mutation tools).
    AiAndUi,
    /// Tool strictly reserved for UI/CLI actions; AI invocation is forbidden (e.g. `confirm_phase`, `commit_draft`).
    UiCliOnly,
    /// Internal subagent execution only.
    SubagentOnly,
}

/// Category / type of tool operation (`harness/tools.md` §1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ToolType {
    /// Read-only query / lookup tool (never mutates state or creates draft version).
    Read,
    /// Mutation tool modifying draft state (`add_*`, `set_*`).
    Mutation,
    /// Deletion tool modifying draft state (`delete_*`).
    Delete,
    /// Workflow or UI orchestration action (`confirm_phase`, `commit_draft`, etc.).
    Action,
}

/// Operational scope where the tool is valid (`harness/tools.md` §1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ToolScope {
    /// Planning / draft preparation mode (Phase 0–6).
    Planning,
    /// Writing mode after plan commit (scene writing, locking).
    Writing,
    /// Global system / configuration management.
    System,
}

/// Declared provenance source mode required in tool arguments (`core.md` §1.3, `be-core.md` §5.1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceParamMode {
    /// Read tool or system action: no declared_source required.
    None,
    /// Multi-field mutation (`add_*`, `set_*`): requires `declared_sources` map.
    MapDeclaredSources,
    /// Single atomic link or delete (`delete_*`, link tools): requires scalar `declared_source`.
    ScalarDeclaredSource,
}

/// Complete metadata for a tool in the registry (`be-core.md` §5.1).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolMetadata {
    pub name: String,
    pub description: String,
    pub tool_type: ToolType,
    pub scope: ToolScope,
    pub allowed_caller: AllowedCaller,
    pub source_param_mode: SourceParamMode,
    pub writes_draft_state: bool,
}

impl ToolMetadata {
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        tool_type: ToolType,
        scope: ToolScope,
        allowed_caller: AllowedCaller,
        source_param_mode: SourceParamMode,
        writes_draft_state: bool,
    ) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            tool_type,
            scope,
            allowed_caller,
            source_param_mode,
            writes_draft_state,
        }
    }

    /// Returns `true` if the specified caller category is authorized to invoke this tool.
    pub fn is_allowed_caller(&self, caller: CallerCategory) -> bool {
        match (self.allowed_caller, caller) {
            (AllowedCaller::AiOnly, CallerCategory::AiCaller) => true,
            (AllowedCaller::AiOnly, CallerCategory::UiAction | CallerCategory::Cli) => true,
            (AllowedCaller::AiAndUi, _) => true,
            (AllowedCaller::UiCliOnly, CallerCategory::AiCaller) => false,
            (AllowedCaller::UiCliOnly, CallerCategory::UiAction | CallerCategory::Cli) => true,
            (AllowedCaller::SubagentOnly, _) => false,
        }
    }

    /// Read tools strictly never create draft versions (`REQ-TOOLS-003`).
    pub fn creates_draft_version(&self) -> bool {
        false
    }
}

/// Central tool registry and authorization gate (`REQ-TOOLS-003`, `P3S4-N`).
#[derive(Debug, Clone)]
pub struct ToolRegistry {
    tools: BTreeMap<String, ToolMetadata>,
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl ToolRegistry {
    /// Creates a new `ToolRegistry` pre-populated with all standard planning, writing, and catalog tools.
    pub fn new() -> Self {
        let mut registry = Self {
            tools: BTreeMap::new(),
        };
        registry.register_standard_tools();
        registry
    }

    /// Registers a tool's metadata into the registry.
    pub fn register(&mut self, meta: ToolMetadata) {
        self.tools.insert(meta.name.clone(), meta);
    }

    /// Looks up metadata for a tool by name/code.
    pub fn lookup(&self, name: &str) -> Option<&ToolMetadata> {
        self.tools.get(name)
    }

    /// Returns all registered tool metadata.
    pub fn list(&self) -> Vec<&ToolMetadata> {
        self.tools.values().collect()
    }

    /// Validates caller authorization for a tool (`REQ-TOOLS-003`, `P3S4-N`).
    ///
    /// Rejects unauthorized AI calls to UI-only tools (`confirm_phase`, `set_scene_lock`, `commit_draft`)
    /// with `ErrorCode::UnauthorizedCaller`.
    pub fn authorize(&self, tool_name: &str, caller: CallerCategory) -> Result<&ToolMetadata, ErrorCode> {
        let meta = self.lookup(tool_name).ok_or(ErrorCode::EntityNotFound)?;

        if !meta.is_allowed_caller(caller) {
            return Err(ErrorCode::UnauthorizedCaller);
        }

        Ok(meta)
    }

    /// Registers all standard planning, writing, and catalog tools.
    fn register_standard_tools(&mut self) {
        // --- Read & Catalog tools (allowed for AI and UI, read-only, never creates draft version) ---
        self.register(ToolMetadata::new(
            "get_field_null",
            "Read up to 3 missing/null fields with deterministic questions according to system language",
            ToolType::Read,
            ToolScope::Planning,
            AllowedCaller::AiAndUi,
            SourceParamMode::None,
            false,
        ));

        self.register(ToolMetadata::new(
            "get_array_element_fields",
            "Read up to 3 priority or missing fields for an array path with system language questions",
            ToolType::Read,
            ToolScope::Planning,
            AllowedCaller::AiAndUi,
            SourceParamMode::None,
            false,
        ));

        self.register(ToolMetadata::new(
            "get_state",
            "Read working state or entity slice with provenance and pending patches",
            ToolType::Read,
            ToolScope::Planning,
            AllowedCaller::AiAndUi,
            SourceParamMode::None,
            false,
        ));

        self.register(ToolMetadata::new(
            "search_genres",
            "Search reference genre catalog with JIT retrieval",
            ToolType::Read,
            ToolScope::Planning,
            AllowedCaller::AiAndUi,
            SourceParamMode::None,
            false,
        ));
        self.register(ToolMetadata::new(
            "search_genre",
            "Search reference genre catalog (singular alias)",
            ToolType::Read,
            ToolScope::Planning,
            AllowedCaller::AiAndUi,
            SourceParamMode::None,
            false,
        ));

        self.register(ToolMetadata::new(
            "search_motifs",
            "Search reference motif catalog by category or query",
            ToolType::Read,
            ToolScope::Planning,
            AllowedCaller::AiAndUi,
            SourceParamMode::None,
            false,
        ));
        self.register(ToolMetadata::new(
            "search_motif",
            "Search reference motif catalog (singular alias)",
            ToolType::Read,
            ToolScope::Planning,
            AllowedCaller::AiAndUi,
            SourceParamMode::None,
            false,
        ));

        self.register(ToolMetadata::new(
            "search_lore",
            "Search lore entries in catalog by world setting or query",
            ToolType::Read,
            ToolScope::Planning,
            AllowedCaller::AiAndUi,
            SourceParamMode::None,
            false,
        ));
        self.register(ToolMetadata::new(
            "search_lore_entry",
            "Search lore entries in catalog (singular alias)",
            ToolType::Read,
            ToolScope::Planning,
            AllowedCaller::AiAndUi,
            SourceParamMode::None,
            false,
        ));

        self.register(ToolMetadata::new(
            "search_world_settings",
            "Search world settings in catalog by project or query",
            ToolType::Read,
            ToolScope::Planning,
            AllowedCaller::AiAndUi,
            SourceParamMode::None,
            false,
        ));
        self.register(ToolMetadata::new(
            "search_world_setting",
            "Search world settings in catalog (singular alias)",
            ToolType::Read,
            ToolScope::Planning,
            AllowedCaller::AiAndUi,
            SourceParamMode::None,
            false,
        ));

        self.register(ToolMetadata::new(
            "search_character",
            "Search characters in reference catalog and project library",
            ToolType::Read,
            ToolScope::Planning,
            AllowedCaller::AiAndUi,
            SourceParamMode::None,
            false,
        ));

        // --- UI/CLI-only actions (strictly blocked for AI: UnauthorizedCaller) ---
        self.register(ToolMetadata::new(
            "confirm_phase",
            "User UI action to confirm phase transition and advance draft",
            ToolType::Action,
            ToolScope::Planning,
            AllowedCaller::UiCliOnly,
            SourceParamMode::None,
            false,
        ));

        self.register(ToolMetadata::new(
            "set_scene_lock",
            "User UI action to toggle or enforce scene lock",
            ToolType::Action,
            ToolScope::Writing,
            AllowedCaller::UiCliOnly,
            SourceParamMode::None,
            false,
        ));

        self.register(ToolMetadata::new(
            "lock_scene",
            "User UI action to lock a scene against modifications",
            ToolType::Action,
            ToolScope::Writing,
            AllowedCaller::UiCliOnly,
            SourceParamMode::None,
            false,
        ));

        self.register(ToolMetadata::new(
            "unlock_scene",
            "User UI action to unlock a scene",
            ToolType::Action,
            ToolScope::Writing,
            AllowedCaller::UiCliOnly,
            SourceParamMode::None,
            false,
        ));

        self.register(ToolMetadata::new(
            "commit_draft",
            "User UI action to commit planning draft into durable database",
            ToolType::Action,
            ToolScope::Planning,
            AllowedCaller::UiCliOnly,
            SourceParamMode::None,
            false,
        ));

        self.register(ToolMetadata::new(
            "confirm_patch",
            "User UI action to confirm a pending proposal patch",
            ToolType::Action,
            ToolScope::Planning,
            AllowedCaller::UiCliOnly,
            SourceParamMode::None,
            false,
        ));

        self.register(ToolMetadata::new(
            "reject_patch",
            "User UI action to reject a pending proposal patch",
            ToolType::Action,
            ToolScope::Planning,
            AllowedCaller::UiCliOnly,
            SourceParamMode::None,
            false,
        ));

        self.register(ToolMetadata::new(
            "confirm_all_patches",
            "User UI action to confirm all pending proposal patches",
            ToolType::Action,
            ToolScope::Planning,
            AllowedCaller::UiCliOnly,
            SourceParamMode::None,
            false,
        ));

        self.register(ToolMetadata::new(
            "restore_version",
            "User UI action to restore a past draft version",
            ToolType::Action,
            ToolScope::Planning,
            AllowedCaller::UiCliOnly,
            SourceParamMode::None,
            false,
        ));

        self.register(ToolMetadata::new(
            "write_scene_content",
            "Direct scene content persistence in writing mode (UI controlled lifecycle)",
            ToolType::Action,
            ToolScope::Writing,
            AllowedCaller::UiCliOnly,
            SourceParamMode::None,
            false,
        ));

        // --- Mutation tools (AI and UI allowed, writes draft state) ---
        self.register(ToolMetadata::new(
            "add_project",
            "Add initial project requirements",
            ToolType::Mutation,
            ToolScope::Planning,
            AllowedCaller::AiAndUi,
            SourceParamMode::MapDeclaredSources,
            true,
        ));

        self.register(ToolMetadata::new(
            "set_project",
            "Update project requirement fields",
            ToolType::Mutation,
            ToolScope::Planning,
            AllowedCaller::AiAndUi,
            SourceParamMode::MapDeclaredSources,
            true,
        ));

        self.register(ToolMetadata::new(
            "add_character",
            "Add character to draft state",
            ToolType::Mutation,
            ToolScope::Planning,
            AllowedCaller::AiAndUi,
            SourceParamMode::MapDeclaredSources,
            true,
        ));

        self.register(ToolMetadata::new(
            "set_character",
            "Update character fields in draft state",
            ToolType::Mutation,
            ToolScope::Planning,
            AllowedCaller::AiAndUi,
            SourceParamMode::MapDeclaredSources,
            true,
        ));

        self.register(ToolMetadata::new(
            "delete_character",
            "Delete character from draft state with cascade verification",
            ToolType::Delete,
            ToolScope::Planning,
            AllowedCaller::AiAndUi,
            SourceParamMode::ScalarDeclaredSource,
            true,
        ));
    }
}
