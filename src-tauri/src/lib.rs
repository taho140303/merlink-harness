mod adapters;
mod catalog;
#[macro_use]
mod commands;
mod db;
mod models;
mod outline;
mod paths;
mod runtime;
mod store;
mod super_json;
mod tools;
mod writing;
#[allow(unused_imports)]
use commands::{
    context_usage, delete_session, execute_tool, fetch_models, get_project, get_session,
    list_messages, list_projects, list_sessions, load_providers, outline_accept, outline_anchor,
    outline_cancel, outline_commit, outline_items, outline_regenerate, outline_reject, outline_run,
    outline_start, outline_status, rename_session, save_providers, send_chat, super_json_labels,
    super_json_progress, super_json_read, super_json_summary,
};
/// Shared handler list for `run()` and IPC contract tests.
macro_rules! merlink_handlers {
    () => {
        tauri::generate_handler![
            crate::commands::providers::load_providers,
            crate::commands::providers::save_providers,
            crate::commands::chat::send_chat,
            crate::commands::project::execute_tool,
            crate::commands::project::list_projects,
            crate::commands::project::get_project,
            crate::commands::super_json::super_json_read,
            crate::commands::super_json::super_json_progress,
            crate::commands::super_json::super_json_summary,
            crate::commands::super_json::super_json_labels,
            crate::commands::chat::context_usage,
            crate::commands::chat::list_messages,
            crate::commands::providers::fetch_models,
            crate::commands::sessions::list_sessions,
            crate::commands::sessions::get_session,
            crate::commands::sessions::rename_session,
            crate::commands::sessions::delete_session,
            crate::commands::outline::outline_anchor,
            crate::commands::outline::outline_status,
            crate::commands::outline::outline_start,
            crate::commands::outline::outline_run,
            crate::commands::outline::outline_items,
            crate::commands::outline::outline_regenerate,
            crate::commands::outline::outline_commit,
            crate::commands::outline::outline_accept,
            crate::commands::outline::outline_reject,
            crate::commands::outline::outline_cancel,
            crate::commands::writing::writing_status,
            crate::commands::writing::write_scene,
            crate::commands::writing::rewrite_scenes,
            crate::commands::writing::lock_scene
        ]
    };
}
#[cfg(test)]
mod test_support;
#[cfg(test)]
mod tests {
    #[test]
    fn test_phase_zero_prompt_includes_runtime_and_project() {
        let prompt =
            crate::runtime::assemble_prompt("DeepSeek", "deepseek-chat", "session-1", "project-1");
        assert!(prompt.contains("Runtime: DeepSeek / deepseek-chat"));
        assert!(prompt.contains("Project: project-1"));

        // A chat with no project yet says so (Sprint 10 · D1/D2/D5).
        let empty = crate::runtime::assemble_prompt("DeepSeek", "deepseek-chat", "session-1", "");
        assert!(empty.contains("Project: (none yet - call project_create"));
        assert!(!empty.contains("{project_state}"));
        assert!(!empty.contains("{project_id}"));
    }

    /// Sprint 11.13 · D2: no provider catalog ships; a fresh install starts empty.
    #[test]
    fn test_default_providers_are_empty() {
        assert!(
            crate::store::default_providers().is_empty(),
            "a hardcoded provider catalog must not ship"
        );
    }

    #[test]
    fn test_openai_endpoint_avoids_duplicate_path() {
        assert_eq!(
            crate::adapters::openai_endpoint("https://api.openai.com/v1"),
            "https://api.openai.com/v1/chat/completions"
        );
    }

    /// The embedded skeleton must match the documented file minus stripped keys.
    #[test]
    fn test_embedded_skeleton_matches_documented_file() {
        let documented = include_str!("../../docs/schema/runtime/super.json");
        let mut parsed: serde_json::Value = serde_json::from_str(documented).unwrap();
        parsed.as_object_mut().unwrap().remove("$comment");
        assert_eq!(crate::super_json::skeleton(), parsed);
    }

    /// The system prompt must spell out every project leaf and every table name;
    /// the per-table `.count`/`.rows` grammar is documented once.
    #[test]
    fn test_system_prompt_documents_every_valid_path() {
        let prompt = crate::runtime::assemble_prompt("OpenAI", "gpt-4o", "s", "p");
        for path in crate::super_json::valid_paths() {
            if let Some(table) = path
                .split_once(".schema.")
                .and_then(|(_, tail)| tail.split('.').next())
            {
                assert!(
                    prompt.contains(table),
                    "system prompt does not mention table {table}"
                );
            } else {
                assert!(
                    prompt.contains(path.as_str()),
                    "system prompt does not mention {path}"
                );
            }
        }
        assert!(prompt.contains("require.schema.<table>.count"));
        assert!(prompt.contains("optional.schema.<table>.rows"));
    }
}
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(merlink_handlers!())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/// Declared last: the handler macro resolves generated command macros textually.
#[cfg(test)]
mod ipc_contract_tests;
