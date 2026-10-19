mod adapters;
#[macro_use]
mod commands;
mod db;
mod models;
mod paths;
mod runtime;
mod store;
mod super_json;
mod tools;
use commands::{
    context_usage, execute_tool, fetch_models, list_messages, load_providers, save_providers,
    send_chat, super_json_labels, super_json_progress, super_json_read, super_json_summary,
};
/// Shared handler list for `run()` and IPC contract tests.
macro_rules! merlink_handlers {
    () => {
        tauri::generate_handler![
            crate::commands::providers::load_providers,
            crate::commands::providers::save_providers,
            crate::commands::chat::send_chat,
            crate::commands::project::execute_tool,
            crate::commands::super_json::super_json_read,
            crate::commands::super_json::super_json_progress,
            crate::commands::super_json::super_json_summary,
            crate::commands::super_json::super_json_labels,
            crate::commands::chat::context_usage,
            crate::commands::chat::list_messages,
            crate::commands::providers::fetch_models
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

    /// The system prompt must spell out every writable super-json path.
    #[test]
    fn test_system_prompt_documents_every_valid_path() {
        let prompt = crate::runtime::assemble_prompt("OpenAI", "gpt-4o", "s", "p");
        for path in crate::super_json::valid_paths() {
            assert!(
                prompt.contains(path.as_str()),
                "system prompt does not mention {path}"
            );
        }
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
