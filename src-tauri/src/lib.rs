mod adapters;
mod db;
mod models;
mod paths;
mod runtime;
mod store;
mod super_json;
mod tools;

/// The IPC surface of the app, in one place (Sprint 11.6 · D2).
///
/// `run()` and the IPC contract tests both expand this macro, so the list of
/// registered commands cannot drift between the app and the tests. Copying the
/// list into two places is exactly the drift this sprint exists to stop.
macro_rules! merlink_handlers {
    () => {
        tauri::generate_handler![
            load_providers,
            save_providers,
            send_chat,
            execute_tool,
            super_json_read,
            super_json_progress,
            super_json_summary,
            super_json_labels,
            context_usage,
            list_messages,
            fetch_models
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

        // A chat with no project yet says so, so the model knows to create one
        // (Sprint 10 · D1/D2/D5).
        let empty = crate::runtime::assemble_prompt("DeepSeek", "deepseek-chat", "session-1", "");
        assert!(empty.contains("Project: (none yet - call project_create"));
        assert!(!empty.contains("{project_state}"));
        assert!(!empty.contains("{project_id}"));
    }

    /// Sprint 11.13 · D2: the shipped catalog is gone. Providers are only what
    /// the author adds, so a fresh install starts with an empty list.
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

    /// The skeleton embedded in the binary must match the documented file, minus
    /// the documentation keys that are stripped before writing to a project.
    #[test]
    fn test_embedded_skeleton_matches_documented_file() {
        let documented = include_str!("../../docs/schema/runtime/super.json");
        let mut parsed: serde_json::Value = serde_json::from_str(documented).unwrap();
        parsed.as_object_mut().unwrap().remove("$comment");
        assert_eq!(crate::super_json::skeleton(), parsed);
    }

    /// The system prompt is the model's only map of the super json, so every
    /// writable path must be spelled out there.
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

#[tauri::command]
fn load_providers() -> Result<Vec<models::ProviderConfig>, String> {
    store::load_providers()
}

#[tauri::command]
fn save_providers(providers: Vec<models::ProviderConfig>) -> Result<(), String> {
    store::save_providers(&providers)
}

#[tauri::command]
async fn send_chat(
    provider_id: String,
    model_id: String,
    messages: Vec<models::MessageEnvelope>,
    project_id: String,
    session_id: String,
    channel: tauri::ipc::Channel<models::AgentEvent>,
) -> Result<models::ChatTurn, String> {
    runtime::send_chat_internal(
        provider_id,
        model_id,
        messages,
        project_id,
        session_id,
        channel,
    )
    .await
}

#[tauri::command]
fn execute_tool(
    tool_name: String,
    args: serde_json::Value,
    project_id: String,
    session_id: String,
) -> Result<serde_json::Value, String> {
    runtime::execute_tool_internal(&tool_name, args, &project_id, &session_id)
}

#[tauri::command]
fn super_json_read(project_id: String) -> Result<serde_json::Value, String> {
    runtime::super_json_document(&project_id)
}

/// Progress for the chat header (D7). Computed in Rust: the frontend renders the
/// numbers, it never derives them.
#[tauri::command]
fn super_json_progress(session_id: String) -> Result<models::SuperJsonProgress, String> {
    runtime::super_json_progress_for_session(&session_id)
}

/// The `Chi tiết` dialog's six-paragraph summary (Sprint 11.11 · D2/D5).
///
/// Called from JS with **camelCase** keys (`{ projectId }`); `tauri-macros`
/// lower-camel-cases the Rust parameter name, so `project_id` is rejected with
/// "missing required key".
#[tauri::command]
fn super_json_summary(project_id: String) -> Result<String, String> {
    runtime::super_json_summary_for_project(&project_id)
}

/// `path -> Vietnamese label` for the tool-log dialog (Sprint 11.12 · D8).
///
/// The only source is `tools::PATH_LABELS`; the frontend never hardcodes a second
/// map. Takes no arguments, so JS calls `safeInvoke("super_json_labels")`.
#[tauri::command]
fn super_json_labels() -> std::collections::BTreeMap<String, String> {
    tools::field_labels()
        .into_iter()
        .map(|(path, label)| (path.to_owned(), label.to_owned()))
        .collect()
}

#[tauri::command]
fn context_usage(session_id: String, provider_id: String, model_id: String) -> Result<models::ContextUsage, String> {
    runtime::context_usage_for_session(&session_id, &provider_id, &model_id)
}

#[tauri::command]
fn list_messages(session_id: String) -> Result<Vec<models::MessageEnvelope>, String> {
    runtime::session_messages(&session_id)
}

/// Lists the models a provider serves (Sprint 11.5 · D1).
///
/// Called from JS with **camelCase** keys (`providerKind`, `baseUrl`, `apiKey`) —
/// `tauri-macros` lower-camel-cases every Rust parameter name, and snake_case keys
/// fail with "missing required key" (D11).
#[tauri::command]
async fn fetch_models(
    provider_kind: String,
    base_url: String,
    api_key: String,
) -> Result<Vec<models::FetchedModel>, String> {
    runtime::fetch_models_internal(&provider_kind, &base_url, &api_key).await
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(merlink_handlers!())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/// Declared last on purpose: `tauri::generate_handler!` resolves the
/// `__tauri_command_name_*` / `__cmd__*` macros that `#[tauri::command]`
/// generates textually below their definitions, so this module must come after
/// every command item.
#[cfg(test)]
mod ipc_contract_tests;
