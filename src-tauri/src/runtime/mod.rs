//! Agent runtime: prompt assembly, progress events, the model/tool loop, and
//! session-scoped queries.

mod agent;
#[allow(dead_code)] // runtime API for the outline fan-out module
mod complete;
mod events;
mod prompt;
mod session;

#[allow(unused_imports)] // part of the runtime API; not every item has a non-test caller
pub use agent::{MAX_TOOL_ROUNDS, STOP_MARKER, run_agent_turn, send_chat_internal};
#[allow(unused_imports)]
pub use complete::{Completion, complete, extract_json};
#[cfg(test)]
pub use events::RecordingSink;
#[allow(unused_imports)]
pub use events::{NoopSink, ProgressSink};
#[allow(unused_imports)]
pub use prompt::assemble_prompt;
pub use session::{
    context_usage_for_session, execute_tool_internal, session_messages, super_json_document,
    super_json_progress_for_session, super_json_summary_for_project,
};
