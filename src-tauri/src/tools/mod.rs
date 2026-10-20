//! Tool surface for the agent loop.
//!
//! [`TOOL_NAMES`] is the ordered public list. Submodules own the UI labels, the
//! provider JSON schemas, project-id creation, and dispatch; everything callers
//! used through `crate::tools` is re-exported here.

mod execute;
mod labels;
mod project;
mod specs;

pub use execute::execute_tool;
pub use labels::{field_labels, status_message, tool_path};
pub use specs::tool_specs;

/// Tools exposed to the model. `project_create` is listed first because the other
/// six are unusable until a chat has a project.
pub const TOOL_NAMES: [&str; 8] = [
    "project_create",
    "super_json_get",
    "super_json_set",
    "super_json_add_row",
    "super_json_add_field",
    "super_json_missing",
    "echo",
    "outline",
];
