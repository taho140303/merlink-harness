//! IPC contract tests (Sprint 11.6).
//!
//! Why this file exists: for three sprints in a row the app was only ever
//! verified in browser mode against the JS mock, never through Tauri's real IPC
//! layer. Sprint 9 shipped the price of that blind spot — the frontend called
//! `send_chat` with `session_id` while `tauri-macros` lower-camel-cases every
//! Rust parameter name, so the desktop chat could not send a single message and
//! nobody noticed.
//!
//! `tauri::test` (feature `test`, dev-dependency only) drives the *real* command
//! deserialisation path — the same `InvokeRequest` the webview sends — without a
//! webview. A wrong argument name therefore fails `cargo test` instead of
//! failing silently in the app.
//!
//! The registered command list comes from `merlink_handlers!` (D2): the app and
//! these tests expand the same macro, so the list cannot drift.

use serde_json::{Value, json};

use crate::test_support::TempHome;
use tauri::ipc::{CallbackFn, InvokeBody};
use tauri::test::{
    INVOKE_KEY, MockRuntime, get_ipc_response, mock_builder, mock_context, noop_assets,
};
use tauri::webview::InvokeRequest;

// ─────────────────────────────────────────────────────────────────────────────
// Harness
// ─────────────────────────────────────────────────────────────────────────────

/// A mock app + webview wired to the production command list, with storage
/// pointed at a temp home.
struct Harness {
    /// The real `~/.merlink` must never be touched by a test run.
    home: TempHome,
    /// Kept alive so the webview never outlives its app.
    _app: tauri::App<MockRuntime>,
    webview: tauri::WebviewWindow<MockRuntime>,
}

impl Harness {
    fn new(name: &str) -> Self {
        let home = TempHome::new(name);
        let app = mock_builder()
            // Textual macro scope: `merlink_handlers!` is defined above this
            // module in `lib.rs`, so it is in scope here by bare name.
            .invoke_handler(merlink_handlers!())
            .build(mock_context(noop_assets()))
            .expect("build mock app");
        let webview = tauri::WebviewWindowBuilder::new(&app, "main", Default::default())
            .build()
            .expect("build mock webview");
        Self {
            home,
            _app: app,
            webview,
        }
    }

    /// Sends one command exactly as the JS bridge does: a `cmd` plus a JSON
    /// body keyed by the *camelCase* argument names.
    fn call(&self, cmd: &str, body: Value) -> Result<Value, Value> {
        let response = get_ipc_response(
            &self.webview,
            InvokeRequest {
                cmd: cmd.into(),
                callback: CallbackFn(0),
                error: CallbackFn(1),
                url: if cfg!(any(windows, target_os = "android")) {
                    "http://tauri.localhost"
                } else {
                    "tauri://localhost"
                }
                .parse()
                .expect("invoke url"),
                body: InvokeBody::Json(body),
                headers: Default::default(),
                invoke_key: INVOKE_KEY.to_string(),
            },
        );
        response.map(|body| body.deserialize::<Value>().expect("IPC body is JSON"))
    }

    /// Same as [`Harness::call`] but panics with the backend message on error.
    fn ok(&self, cmd: &str, body: Value) -> Value {
        match self.call(cmd, body) {
            Ok(value) => value,
            Err(error) => panic!("{cmd} failed: {}", error_text(&error)),
        }
    }
}

/// `Err` values arrive either as a JSON string (our `Result<T, String>`
/// commands) or as a JSON value; normalise both to text.
fn error_text(error: &Value) -> String {
    match error {
        Value::String(text) => text.clone(),
        other => other.to_string(),
    }
}

/// The exact message `tauri::ipc::CommandItem` emits when a body key is missing,
/// which is what a snake_case key hits (`command send_chat missing required key
/// sessionId`).
const MISSING_KEY: &str = "missing required key";

/// Sprint 11.13 · D2: nothing ships any more, so tests that need a provider save
/// one into their own temp home first.
fn seed_contract_demo(harness: &Harness) {
    let mut provider = crate::store::catalog_fixture().remove(2); // deepseek shape
    provider.id = "contract-demo".into();
    provider.name = "Contract demo".into();
    provider.kind = "custom".into();
    provider.api_key.clear();
    provider.default_model_id = "deepseek-chat".into();
    harness.ok(
        "save_providers",
        json!({ "providers": serde_json::to_value([&provider]).expect("providers") }),
    );
}

fn seed_openai_with_key(harness: &Harness) {
    let mut provider = crate::store::catalog_fixture().remove(0);
    provider.api_key = "contract-key".into();
    harness.ok(
        "save_providers",
        json!({ "providers": serde_json::to_value([&provider]).expect("providers") }),
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Contract table
// ─────────────────────────────────────────────────────────────────────────────

/// One row of the IPC contract: a command and the *exact* body the frontend
/// sends for it — camelCase keys, real JSON values.
struct ContractEntry {
    cmd: &'static str,
    body: fn() -> Value,
}

impl ContractEntry {
    fn camel_keys(&self) -> Vec<String> {
        keys_of(&(self.body)())
    }

    fn snake_keys(&self) -> Vec<String> {
        self.camel_keys()
            .iter()
            .map(|key| camel_to_snake(key))
            .collect()
    }

    /// The same body the frontend sends, with every top-level key renamed to
    /// snake_case — i.e. the Sprint 9 mistake, reproduced on purpose.
    fn body_with_snake_keys(&self) -> Value {
        let renames: Vec<(String, String)> = self
            .camel_keys()
            .into_iter()
            .map(|key| {
                let snake = camel_to_snake(&key);
                (key, snake)
            })
            .collect();
        self.rename_keys(&renames)
    }

    /// Same, but for exactly one key: every other key stays camelCase, so the
    /// error can only be about the key under test.
    fn body_with_snake_key(&self, key: &str) -> Value {
        self.rename_keys(&[(key.to_string(), camel_to_snake(key))])
    }

    fn rename_keys(&self, renames: &[(String, String)]) -> Value {
        let mut body = (self.body)();
        if let Some(object) = body.as_object_mut() {
            for (from, to) in renames {
                let value = object
                    .remove(from)
                    .unwrap_or_else(|| panic!("contract body has no key `{from}`"));
                object.insert(to.clone(), value);
            }
        }
        body
    }

    /// True when at least one key actually changes spelling. `save_providers`
    /// (`providers`) and `load_providers` (no arguments) have no snake_case
    /// variant, so they cannot be part of the rejection proof.
    fn has_multiword_key(&self) -> bool {
        self.camel_keys()
            .iter()
            .any(|key| camel_to_snake(key) != *key)
    }
}

fn keys_of(value: &Value) -> Vec<String> {
    value
        .as_object()
        .expect("contract bodies are JSON objects")
        .keys()
        .cloned()
        .collect()
}

/// `providerId` -> `provider_id`, `apiKey` -> `api_key`.
fn camel_to_snake(key: &str) -> String {
    let mut out = String::with_capacity(key.len() + 2);
    for ch in key.chars() {
        if ch.is_ascii_uppercase() {
            out.push('_');
            out.push(ch.to_ascii_lowercase());
        } else {
            out.push(ch);
        }
    }
    out
}

fn body_load_providers() -> Value {
    // `Chat.tsx` and `SettingsDialog.tsx` both call `load_providers` with no
    // arguments at all.
    json!({})
}

fn body_save_providers() -> Value {
    // `SettingsDialog.tsx:464` — `safeInvoke("save_providers", { providers })`.
    // Sprint 11.13 · D2: the shipped catalog is empty, so this row uses the
    // test-only fixture purely as a realistic wire shape.
    json!({ "providers": serde_json::to_value(crate::store::catalog_fixture()).expect("providers") })
}

fn body_send_chat() -> Value {
    // `Chat.tsx:451`. `contract-demo` is a custom provider the test saves into
    // its temp home with an empty api key, so the turn takes the offline "demo
    // mode" path and needs no network (Sprint 11.13 · D2 removed the seeded
    // providers this body used to rely on). The channel travels in the body as
    // the JS bridge serialises a `Channel` (`__CHANNEL__:<id>`), which
    // `Channel`'s `CommandArg` reads back.
    json!({
        "providerId": "contract-demo",
        "modelId": "deepseek-chat",
        "messages": [{ "id": "contract-user", "role": "user", "content": "Xin chào" }],
        "projectId": "",
        "sessionId": "contract-session-send-chat",
        "channel": "__CHANNEL__:1"
    })
}

fn body_execute_tool() -> Value {
    // No frontend caller yet; keys come from the Rust signature in `lib.rs`
    // (`tool_name`, `args`, `project_id`, `session_id`) lower-camel-cased.
    json!({
        "toolName": "echo",
        "args": { "ping": "pong" },
        "projectId": "",
        "sessionId": "contract-session-execute-tool"
    })
}

fn body_super_json_read() -> Value {
    // `Chat.tsx:227` — `safeInvoke("super_json_read", { projectId: id })`.
    json!({ "projectId": "contract-project" })
}

fn body_super_json_progress() -> Value {
    // `Chat.tsx:239` — `safeInvoke("super_json_progress", { sessionId })`.
    json!({ "sessionId": "contract-session-progress" })
}

fn body_super_json_summary() -> Value {
    // `Chat.tsx` — `safeInvoke("super_json_summary", { projectId: id })` (D5).
    json!({ "projectId": "contract-project" })
}

fn body_super_json_labels() -> Value {
    // `Chat.tsx` — `safeInvoke("super_json_labels")`: the command takes no
    // arguments (Sprint 11.12 · D8), so the body is empty.
    json!({})
}

fn body_context_usage() -> Value {
    json!({"sessionId":"contract-session-context", "providerId":"openai", "modelId":"gpt-4o"})
}

fn body_list_messages() -> Value {
    // No frontend caller yet; keys come from the Rust signature in `lib.rs`.
    json!({ "sessionId": "contract-session-list-messages" })
}

fn body_fetch_models() -> Value {
    // `SettingsDialog.tsx:381`. The URL is a closed port so the call fails fast
    // and locally: this row exists for its *key shape*, not its network result.
    json!({ "providerKind": "openai", "baseUrl": "http://127.0.0.1:1", "apiKey": "" })
}

/// Every command `merlink_handlers!` registers must have exactly one row here
/// (D5). `test_contract_table_covers_every_registered_command` enforces both
/// directions.
static CONTRACT: &[ContractEntry] = &[
    ContractEntry {
        cmd: "load_providers",
        body: body_load_providers,
    },
    ContractEntry {
        cmd: "save_providers",
        body: body_save_providers,
    },
    ContractEntry {
        cmd: "send_chat",
        body: body_send_chat,
    },
    ContractEntry {
        cmd: "execute_tool",
        body: body_execute_tool,
    },
    ContractEntry {
        cmd: "super_json_read",
        body: body_super_json_read,
    },
    ContractEntry {
        cmd: "super_json_progress",
        body: body_super_json_progress,
    },
    ContractEntry {
        cmd: "super_json_summary",
        body: body_super_json_summary,
    },
    ContractEntry {
        cmd: "super_json_labels",
        body: body_super_json_labels,
    },
    ContractEntry {
        cmd: "context_usage",
        body: body_context_usage,
    },
    ContractEntry {
        cmd: "list_messages",
        body: body_list_messages,
    },
    ContractEntry {
        cmd: "fetch_models",
        body: body_fetch_models,
    },
];

fn contract_entry(cmd: &str) -> &'static ContractEntry {
    CONTRACT
        .iter()
        .find(|entry| entry.cmd == cmd)
        .unwrap_or_else(|| panic!("no contract row for {cmd}"))
}

// ─────────────────────────────────────────────────────────────────────────────
// D4 — real commands through the real IPC layer, returning real values
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_ipc_load_providers_end_to_end() {
    let harness = Harness::new("ipc-load-providers");

    // No `config.json` yet: there is no shipped catalog any more, so the list is
    // genuinely empty (Sprint 11.13 · D2).
    let providers = harness.ok("load_providers", body_load_providers());
    let list = providers.as_array().expect("providers is an array");
    assert!(
        list.is_empty(),
        "a fresh install must have no providers, got {}",
        list.len()
    );

    // Round trip: save the author's own provider through IPC, read it back.
    let mut edited = crate::store::catalog_fixture().remove(0);
    edited.id = "custom-contract".into();
    edited.name = "Contract".into();
    edited.kind = "custom".into();
    edited.api_key = "contract-key".into();
    let saved = harness.ok(
        "save_providers",
        json!({ "providers": serde_json::to_value([&edited]).expect("providers") }),
    );
    assert!(saved.is_null(), "save_providers returns unit, got {saved}");

    let reloaded = harness.ok("load_providers", body_load_providers());
    let reloaded = reloaded.as_array().expect("providers is an array");
    assert_eq!(reloaded.len(), 1, "exactly the saved provider comes back");
    assert_eq!(reloaded[0]["id"], "custom-contract");
    assert_eq!(reloaded[0]["api_key"], "contract-key");
    assert!(
        harness.home.path().join("config.json").is_file(),
        "save_providers over IPC did not write config.json into the temp home"
    );
}

#[test]
fn test_ipc_context_usage_end_to_end() {
    let harness = Harness::new("ipc-context-usage");
    // Sprint 11.13 · D2: context usage needs a real configured model.
    seed_openai_with_key(&harness);
    let usage = harness.ok("context_usage", body_context_usage());
    assert_eq!(usage["has_model"], true);
    assert_eq!(usage["total_tokens"], 128000);
    assert_eq!(
        usage["used_tokens"],
        usage["system_tokens"].as_u64().unwrap()
            + usage["tool_tokens"].as_u64().unwrap()
            + usage["message_tokens"].as_u64().unwrap()
    );
}

#[test]
fn test_ipc_super_json_progress_end_to_end() {
    let harness = Harness::new("ipc-super-json-progress");

    // 1. A session with no project reports the "no project" numbers, computed in
    //    Rust and shipped over IPC.
    let empty = harness.ok("super_json_progress", body_super_json_progress());
    assert_eq!(empty["has_project"], json!(false));
    assert_eq!(empty["project_id"], json!(""));
    assert_eq!(empty["required_total"], json!(12));
    assert_eq!(empty["optional_total"], json!(35));
    assert_eq!(empty["filled_required"], json!(0));
    assert_eq!(empty["percent"], json!(0));

    // 2. Create the project through the IPC layer too, so the number below is
    //    produced by real work this IPC path performed.
    let created = harness.ok(
        "execute_tool",
        json!({
            "toolName": "project_create",
            "args": { "name": "Nua Dem" },
            "projectId": "",
            "sessionId": "contract-session-progress"
        }),
    );
    let project_id = created["project_id"]
        .as_str()
        .expect("project id")
        .to_string();
    assert!(!project_id.is_empty());

    // 3. The same session now reports real progress: `project_create` writes
    //    exactly one required unit (`require.project.name`), which is 70%/12.
    let progress = harness.ok("super_json_progress", body_super_json_progress());
    assert_eq!(progress["has_project"], json!(true));
    assert_eq!(progress["project_id"], json!(project_id));
    assert_eq!(progress["required_total"], json!(12));
    assert_eq!(progress["optional_total"], json!(35));
    assert_eq!(progress["filled_required"], json!(1));
    assert_eq!(progress["filled_optional"], json!(0));
    assert_eq!(progress["percent"], json!(6));
    assert!(
        progress["label"]
            .as_str()
            .is_some_and(|label| !label.is_empty()),
        "the header bar needs a label, got {progress}"
    );

    // 4. The value matches the domain function the command wraps.
    let expected = crate::super_json::progress(&project_id).expect("progress");
    assert_eq!(progress["percent"], json!(expected.percent));
    assert_eq!(progress["filled_required"], json!(expected.filled_required));
}

#[test]
fn test_ipc_unknown_project_reads_do_not_create_ghost_document() {
    let harness = Harness::new("ipc-super-json-ghost");
    let ghost = "contract-ghost";
    let directory = harness.home.path().join("projects").join(ghost);
    for command in ["super_json_read", "super_json_summary"] {
        let error = harness
            .call(command, json!({ "projectId": ghost }))
            .expect_err("unknown project must not be read");
        assert_eq!(error_text(&error), format!("Project not found: {ghost}"));
        assert!(
            !directory.join("super.json").exists(),
            "{command} created a ghost document"
        );
        assert!(
            !directory.exists(),
            "{command} created a ghost project directory"
        );
    }
    for command in ["super_json_read", "super_json_summary"] {
        let error = harness
            .call(command, json!({ "projectId": "  " }))
            .expect_err("blank project id must be rejected");
        assert!(!error_text(&error).is_empty());
    }
}

#[test]
fn test_ipc_super_json_summary_end_to_end() {
    let harness = Harness::new("ipc-super-json-summary");

    // 1. Create an unnamed project first: reads must not create SQLite projects.
    //    Its skeleton still prints the placeholder everywhere as prose.
    let unnamed = harness.ok(
        "execute_tool",
        json!({
            "toolName": "project_create", "args": {},
            "projectId": "", "sessionId": "contract-session-empty-summary"
        }),
    );
    let empty_project_id = unnamed["project_id"].as_str().expect("project id");
    let empty_document = harness.ok("super_json_read", json!({ "projectId": empty_project_id }));
    assert!(empty_document["require"]["project"]["name"].is_null());
    let empty = harness.ok(
        "super_json_summary",
        json!({ "projectId": empty_project_id }),
    );
    let empty = empty.as_str().expect("super_json_summary returns a string");
    assert_eq!(
        empty.split("\n\n").count(),
        6,
        "expected six paragraphs:\n{empty}"
    );
    assert!(
        empty.contains("[chưa có thông tin]"),
        "an empty super json must print placeholders:\n{empty}"
    );
    assert!(
        !empty.trim_start().starts_with('{'),
        "raw JSON came back:\n{empty}"
    );
    assert!(!empty.contains("null"), "raw JSON came back:\n{empty}");
    println!("empty super json summary ->\n{empty}");

    // 2. Fill fields through the *tool* IPC path, then read the prose back
    //    through the summary command. The project id is produced by real work.
    let created = harness.ok(
        "execute_tool",
        json!({
            "toolName": "project_create",
            "args": { "name": "Nua Dem" },
            "projectId": "",
            "sessionId": "contract-session-summary"
        }),
    );
    let project_id = created["project_id"]
        .as_str()
        .expect("project id")
        .to_string();

    for (path, value) in [
        ("require.project.language", json!("VI")),
        ("optional.project.plan.arc.tolerance", json!(1)),
        ("require.schema.themes.count", json!(1)),
        (
            "require.schema.themes.rows",
            json!([{ "scope": "PROJECT", "question": "Ký ức có làm nên con người?", "resolution_type": "OPEN" }]),
        ),
        ("optional.schema.world_settings.count", json!(1)),
        (
            "optional.schema.world_settings.rows",
            json!([{ "name": "Thành phố", "type": "CITY" }]),
        ),
    ] {
        harness.ok(
            "execute_tool",
            json!({
                "toolName": "super_json_set",
                "args": { "path": path, "value": value },
                "projectId": project_id,
                "sessionId": "contract-session-summary"
            }),
        );
    }

    let filled = harness.ok("super_json_summary", json!({ "projectId": project_id }));
    let filled = filled
        .as_str()
        .expect("super_json_summary returns a string");
    assert_eq!(
        filled.split("\n\n").count(),
        6,
        "expected six paragraphs:\n{filled}"
    );
    // Values written over IPC are in the prose ...
    assert!(filled.contains("Tên dự án: Nua Dem"), "{filled}");
    assert!(filled.contains("tiếng Việt"), "enum not mapped:\n{filled}");
    assert!(filled.contains("để ngỏ"), "enum not mapped:\n{filled}");
    assert!(filled.contains("thành phố"), "enum not mapped:\n{filled}");
    assert!(
        filled.contains("Kế hoạch: [chưa có thông tin] arc"),
        "plan placeholder missing:\n{filled}"
    );
    assert!(filled.contains("Sai số chấp nhận: 1 arc"), "{filled}");
    // ... and the fields still empty still say so.
    assert!(filled.contains("Tác giả: [chưa có thông tin]."), "{filled}");
    assert!(
        !filled.contains("null") && !filled.contains("\"name\""),
        "{filled}"
    );
    println!("filled super json summary ->\n{filled}");

    // 3. The camelCase key is the only accepted spelling (the Sprint 9 trap).
    let snake = harness
        .call("super_json_summary", json!({ "project_id": project_id }))
        .expect_err("snake_case project_id must be rejected");
    assert!(
        error_text(&snake).contains(MISSING_KEY),
        "snake_case was not rejected with a missing-key error: {}",
        error_text(&snake)
    );
}

#[test]
fn test_ipc_super_json_labels_end_to_end() {
    let harness = Harness::new("ipc-super-json-labels");

    // Sprint 11.12 · D8: no arguments, so the command is called with `{}` — the
    // exact body the frontend sends for `safeInvoke("super_json_labels")`.
    let labels = harness.ok("super_json_labels", body_super_json_labels());
    let map = labels.as_object().expect("labels is a JSON object");
    assert_eq!(
        map.len(),
        81,
        "the command must return all 81 labels: {labels}"
    );
    assert_eq!(
        map.len(),
        crate::super_json::valid_paths().len(),
        "the label map and the 81-path whitelist drifted"
    );

    // Every path is present, every value is a non-empty Vietnamese label.
    for path in crate::super_json::valid_paths() {
        let label = map
            .get(path)
            .unwrap_or_else(|| panic!("no label for {path}"))
            .as_str()
            .unwrap_or_else(|| panic!("label for {path} is not a string"));
        assert!(!label.trim().is_empty(), "{path} has an empty label");
    }
    assert_eq!(map["require.project.name"], json!("tên truyện"));
    assert_eq!(map["require.project.plan.word.count"], json!("số từ"));
    assert_eq!(
        map["optional.schema.lore_entries.rows"],
        json!("danh sách lore")
    );

    // The command is a pure read: it must not create a project either.
    assert!(!harness.home.path().join("projects").exists());
    println!("super_json_labels -> {labels}");
}

#[test]
fn test_ipc_list_messages_end_to_end() {
    let harness = Harness::new("ipc-list-messages");
    // Sprint 11.13 · D2: `body_send_chat` names a provider that must be saved.
    seed_contract_demo(&harness);

    // A fresh session has no transcript.
    let empty = harness.ok("list_messages", body_list_messages());
    assert_eq!(empty, json!([]));

    // Produce a real turn through IPC (demo path: no api key, no network), which
    // persists both the user message and the assistant reply.
    let turn = harness.ok("send_chat", body_send_chat());
    let produced = turn["messages"].as_array().expect("turn messages");
    assert_eq!(produced.len(), 1);
    assert!(
        produced[0]["content"]
            .as_str()
            .is_some_and(|content| content.contains("[Demo mode]")),
        "unexpected demo reply: {produced:?}"
    );

    // Read them back through IPC, from SQLite.
    let listed = harness.ok(
        "list_messages",
        json!({ "sessionId": "contract-session-send-chat" }),
    );
    let listed = listed.as_array().expect("messages array");
    assert_eq!(
        listed.len(),
        2,
        "expected the user message and the reply: {listed:?}"
    );
    assert_eq!(listed[0]["role"], json!("user"));
    assert_eq!(listed[0]["id"], json!("contract-user"));
    assert_eq!(listed[1]["role"], json!("assistant"));
    assert_eq!(listed[1]["id"], produced[0]["id"]);
    assert!(
        listed[1]["content"]
            .as_str()
            .is_some_and(|content| content.contains("[Demo mode]")),
        "the persisted reply lost its content: {listed:?}"
    );

    // Sessions stay isolated across the IPC boundary.
    let other = harness.ok(
        "list_messages",
        json!({ "sessionId": "contract-session-other" }),
    );
    assert_eq!(other, json!([]));
}

#[test]
fn test_ipc_execute_tool_end_to_end() {
    let harness = Harness::new("ipc-execute-tool");

    // A tool that needs no storage: the response is the argument echoed back.
    let echoed = harness.ok("execute_tool", body_execute_tool());
    assert_eq!(echoed, json!({ "ping": "pong" }));

    // A tool that does: `project_create` writes SQLite + the super json, so this
    // proves the IPC call really ran the tool rather than returning a stub.
    let created = harness.ok(
        "execute_tool",
        json!({
            "toolName": "project_create",
            "args": { "name": "Nua Dem" },
            "projectId": "",
            "sessionId": "contract-session-execute-tool"
        }),
    );
    assert_eq!(created["ok"], json!(true));
    assert_eq!(created["name"], json!("Nua Dem"));
    let project_id = created["project_id"]
        .as_str()
        .expect("project id")
        .to_string();
    assert!(!project_id.is_empty());
    assert_eq!(
        created["missing_required"]
            .as_array()
            .expect("missing_required")
            .len(),
        11,
        "12 required units minus the name project_create just wrote"
    );

    // The artifacts really landed in the temp home, not in a real ~/.merlink.
    let super_json = harness
        .home
        .path()
        .join("projects")
        .join(&project_id)
        .join("super.json");
    let document: Value =
        serde_json::from_str(&std::fs::read_to_string(&super_json).expect("super.json"))
            .expect("super.json is JSON");
    assert_eq!(document["require"]["project"]["name"], json!("Nua Dem"));
    assert!(harness.home.path().join("merlink.db").is_file());

    // Unknown tools surface their real message over IPC instead of a panic.
    let unknown = harness
        .call(
            "execute_tool",
            json!({
                "toolName": "nope",
                "args": {},
                "projectId": "",
                "sessionId": "contract-session-execute-tool"
            }),
        )
        .expect_err("an unknown tool must be rejected");
    assert!(
        error_text(&unknown).contains("Unknown tool: nope"),
        "unexpected error: {}",
        error_text(&unknown)
    );
}

#[test]
fn test_ipc_execute_tool_rejects_project_from_another_session() {
    let harness = Harness::new("ipc-execute-tool-project-guard");
    let created_a = harness.ok(
        "execute_tool",
        json!({
            "toolName": "project_create", "args": { "name": "Alpha" },
            "projectId": "", "sessionId": "session-a"
        }),
    );
    let created_b = harness.ok(
        "execute_tool",
        json!({
            "toolName": "project_create", "args": { "name": "Beta" },
            "projectId": "", "sessionId": "session-b"
        }),
    );
    let project_a = created_a["project_id"].as_str().unwrap();
    let project_b = created_b["project_id"].as_str().unwrap();
    assert_ne!(project_a, project_b);

    let error = harness
        .call(
            "execute_tool",
            json!({
                "toolName": "super_json_set",
                "args": { "path": "require.schema.project_genres.count", "value": 1 },
                "projectId": project_b, "sessionId": "session-a"
            }),
        )
        .expect_err("cross-session project mutation must fail");
    assert_eq!(
        error_text(&error),
        "Project does not belong to this session"
    );
    assert!(
        crate::super_json::get(project_b, "require.schema.project_genres.count")
            .unwrap()
            .is_null()
    );

    let bound = harness.ok(
        "execute_tool",
        json!({
            "toolName": "super_json_get", "args": {},
            "projectId": "", "sessionId": "session-a"
        }),
    );
    assert_eq!(bound["require"]["project"]["name"], json!("Alpha"));

    let unbound = harness
        .call(
            "execute_tool",
            json!({
                "toolName": "super_json_get", "args": {},
                "projectId": project_a, "sessionId": "session-unbound"
            }),
        )
        .expect_err("an unbound session must not read another project");
    assert!(error_text(&unbound).starts_with("This chat is not bound to a project"));
}

// ─────────────────────────────────────────────────────────────────────────────
// The Sprint 9 trap: camelCase is the only accepted spelling
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_ipc_send_chat_requires_camel_case_session_id() {
    let harness = Harness::new("ipc-send-chat-camel");

    // Sprint 11.13 · D2: no providers ship, so the test saves the `contract-demo`
    // provider `body_send_chat` names. It has no api key, so the turn runs the
    // offline demo path and completes for real.
    seed_contract_demo(&harness);

    // Correct spelling (`sessionId`) must NOT be rejected as a missing key.
    let camel = harness.call("send_chat", body_send_chat());
    println!("send_chat with sessionId -> {camel:?}");
    let camel = camel.expect("send_chat must accept camelCase sessionId");
    assert!(
        !error_text(&camel).contains(MISSING_KEY),
        "camelCase was rejected: {camel}"
    );
    assert_eq!(camel["project_id"], json!(""));
    assert_eq!(
        camel["messages"].as_array().map(Vec::len),
        Some(1),
        "the demo turn must return one assistant message: {camel}"
    );

    // The Sprint 9 spelling (`session_id`) must be rejected, loudly. Every other
    // key stays camelCase, so `sessionId` is the only thing that can fail — this
    // is the exact call that used to break the desktop chat.
    let send_chat = contract_entry("send_chat");
    let snake_body = send_chat.body_with_snake_key("sessionId");
    assert_eq!(
        snake_body["session_id"],
        json!("contract-session-send-chat")
    );
    assert_eq!(snake_body["providerId"], json!("contract-demo"));
    let snake = harness
        .call("send_chat", snake_body)
        .expect_err("snake_case session_id must be rejected");
    println!("send_chat with session_id -> {}", error_text(&snake));
    assert!(
        error_text(&snake).contains(MISSING_KEY),
        "snake_case was not rejected with a missing-key error: {}",
        error_text(&snake)
    );
    assert!(
        error_text(&snake).contains("sessionId"),
        "the error must name the camelCase key the app expects: {}",
        error_text(&snake)
    );
    assert!(
        !error_text(&snake).contains("providerId"),
        "only sessionId should be missing, but the error also names providerId: {}",
        error_text(&snake)
    );
}

#[test]
fn test_ipc_snake_case_keys_are_rejected() {
    let harness = Harness::new("ipc-snake-case");

    let mut control_failures = Vec::new();
    let mut rejection_failures = Vec::new();
    let mut rejected = Vec::new();
    let mut no_snake_variant = Vec::new();

    for entry in CONTRACT {
        // Control group: the real frontend spelling never fails on a key.
        match harness.call(entry.cmd, (entry.body)()) {
            Ok(_) => {}
            Err(error) if error_text(&error).contains(MISSING_KEY) => {
                control_failures.push(format!(
                    "{}: camelCase body was rejected: {}",
                    entry.cmd,
                    error_text(&error)
                ))
            }
            // Any other error (network, unknown state) is fine here: this test
            // is about key spelling, not about the command succeeding.
            Err(_) => {}
        }

        if !entry.has_multiword_key() {
            no_snake_variant.push(entry.cmd);
            continue;
        }

        match harness.call(entry.cmd, entry.body_with_snake_keys()) {
            Err(error) if error_text(&error).contains(MISSING_KEY) => {
                rejected.push(format!("{}: {}", entry.cmd, error_text(&error)));
            }
            Err(error) => rejection_failures.push(format!(
                "{}: failed with the wrong error: {}",
                entry.cmd,
                error_text(&error)
            )),
            Ok(value) => rejection_failures.push(format!(
                "{}: snake_case keys were ACCEPTED (returned {value})",
                entry.cmd
            )),
        }
    }

    println!("camelCase control group: no missing-key errors");
    for line in &rejected {
        println!("snake_case rejected -> {line}");
    }

    assert!(
        control_failures.is_empty(),
        "{}",
        control_failures.join("\n")
    );
    assert!(
        rejection_failures.is_empty(),
        "{}",
        rejection_failures.join("\n")
    );
    assert_eq!(
        no_snake_variant,
        vec!["load_providers", "save_providers", "super_json_labels"],
        "only commands whose keys are unchanged by snake_case may be exempt"
    );
    assert_eq!(
        rejected.len(),
        CONTRACT.len() - no_snake_variant.len(),
        "every command with a multi-word key must be covered by a real rejection"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// D5 — the table cannot drift away from the registered handlers
// ─────────────────────────────────────────────────────────────────────────────

/// `lib.rs` is read as text on purpose: the point is to catch a command that was
/// added to `merlink_handlers!` and never declared in [`CONTRACT`].
const LIB_SOURCE: &str = include_str!("lib.rs");

/// Extracts the command identifiers from `merlink_handlers!` in `lib.rs`.
///
/// Deliberately textual: a parsing test that silently finds nothing would pass
/// forever, so the caller asserts the parse produced the expected names.
fn registered_commands() -> Vec<String> {
    let macro_start = LIB_SOURCE
        .find("macro_rules! merlink_handlers")
        .expect("lib.rs must define merlink_handlers!");
    let after_macro = &LIB_SOURCE[macro_start..];
    const OPENER: &str = "generate_handler![";
    let list_start = after_macro
        .find(OPENER)
        .expect("merlink_handlers! must expand tauri::generate_handler!")
        + OPENER.len();
    let list = &after_macro[list_start..];
    let list_end = list
        .find(']')
        .expect("generate_handler! list must be closed");
    list[..list_end]
        .split(',')
        .map(str::trim)
        .filter(|name| !name.is_empty())
        // Handlers are spelled as fully-qualified paths (`crate::commands::...`),
        // so the contract is checked against the bare command name.
        .map(|path| path.rsplit("::").next().unwrap_or(path).to_string())
        .collect()
}

#[test]
fn test_contract_table_covers_every_registered_command() {
    let registered = registered_commands();

    // Guard the parser itself: an empty or partial parse must not be mistaken
    // for "everything is covered".
    assert!(
        registered.contains(&"send_chat".to_string())
            && registered.contains(&"fetch_models".to_string()),
        "the extraction of merlink_handlers! is broken, got {registered:?}"
    );

    let declared: Vec<String> = CONTRACT.iter().map(|entry| entry.cmd.to_string()).collect();

    let missing: Vec<&String> = registered
        .iter()
        .filter(|cmd| !declared.contains(*cmd))
        .collect();
    let extra: Vec<&String> = declared
        .iter()
        .filter(|cmd| !registered.contains(*cmd))
        .collect();

    assert!(
        missing.is_empty(),
        "these commands are registered in merlink_handlers! but have no contract row: {missing:?}"
    );
    assert!(
        extra.is_empty(),
        "these contract rows are not registered in merlink_handlers!: {extra:?}"
    );
    assert_eq!(registered.len(), declared.len());
}

/// The bodies above were copied out of the frontend, so the keys must literally
/// appear there. Commands the frontend does not call yet are listed explicitly
/// rather than silently skipped.
const RUST_ONLY_COMMANDS: &[&str] = &["execute_tool", "list_messages"];

#[test]
fn test_every_contract_entry_uses_camel_case_keys() {
    const CHAT_TSX: &str = include_str!("../../src/pages/Chat.tsx");
    const SETTINGS_DIALOG_TSX: &str = include_str!("../../src/components/SettingsDialog.tsx");

    for entry in CONTRACT {
        let keys = entry.camel_keys();
        for key in &keys {
            assert!(
                !key.contains('_'),
                "{}: `{key}` is snake_case; tauri-macros lower-camel-cases every parameter",
                entry.cmd
            );
            assert!(
                key.chars().next().is_some_and(|ch| ch.is_ascii_lowercase()),
                "{}: `{key}` is not lowerCamelCase",
                entry.cmd
            );
        }

        if RUST_ONLY_COMMANDS.contains(&entry.cmd) {
            continue;
        }

        for key in &keys {
            assert!(
                CHAT_TSX.contains(key.as_str()) || SETTINGS_DIALOG_TSX.contains(key.as_str()),
                "{}: key `{key}` is not sent by Chat.tsx or SettingsDialog.tsx — \
                 the contract table is guessing",
                entry.cmd
            );
        }
    }

    // The trap itself must stay in the table, spelled as the frontend spells it.
    let send_chat = contract_entry("send_chat");
    assert!(
        send_chat.camel_keys().iter().any(|key| key == "sessionId"),
        "send_chat must declare sessionId (Sprint 9 regression guard)"
    );
    assert!(
        send_chat.snake_keys().iter().any(|key| key == "session_id"),
        "the snake_case variant of sessionId must be derivable"
    );

    // Every registered command is either frontend-called or explicitly exempt.
    let registered = registered_commands();
    for cmd in &registered {
        assert!(
            CONTRACT.iter().any(|entry| entry.cmd == cmd.as_str()),
            "{cmd} has no contract row"
        );
    }
    assert_eq!(
        registered
            .iter()
            .filter(|cmd| RUST_ONLY_COMMANDS.contains(&cmd.as_str()))
            .count(),
        RUST_ONLY_COMMANDS.len()
    );
}
