# Agent Runtime — Phase 0 (Harness Shell)

> **Status:** Decided (from `overview.md` Q1-Q7).
> **Target:** Rust (Tauri) Backend + React Frontend.

---

## 1. Các quyết định kiến trúc (Decision Log)

- **D1 (Name):** Agent name là `Merlink`.
- **D2 (Identity):** Agent tự nhận thức model đang chạy qua system prompt (`Runtime: {provider} / {model}`).
- **D3 (Persistence):** Bỏ `localStorage` ở bản app thật. Dùng Tauri filesystem lưu tại `~/.merlink/config.json` và `~/.merlink/projects/`.
- **D4 (Tool Registry):** Chưa cần full tool. Chỉ build skeleton registry và 1 tool demo (`echo`).
- **D5 (Prompt Template):** Nhúng trực tiếp vào Rust code bằng `include_str!` (thư mục `src-tauri/prompts/`).
- **D6 (Provider Adapter):** Hỗ trợ 2 chuẩn: `OpenAI Compatible` (cho OpenAI, DeepSeek, Ollama) và `Anthropic` (native Messages API).
- **D7 (Multi-project):** Phase 0 chỉ inject `project_id` vào metadata, chưa query DB project.

---

## 2. Data Model (Rust Structs)

```rust
#[derive(Serialize, Deserialize, Clone)]
pub struct ModelConfig {
    pub id: String,
    pub name: String,
    pub context_window: String,
    pub max_tokens: String,
    pub input_types: InputTypes,
    pub reasoning_level: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ProviderConfig {
    pub id: String,
    pub name: String,
    pub kind: String, // "openai" | "anthropic" | "custom"
    pub api_key: String,
    pub base_url: String,
    pub models: Vec<ModelConfig>,
    pub default_model_id: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MessageEnvelope {
    pub id: String,
    pub role: String, // "user" | "assistant" | "system"
    pub content: String,
    pub meta: Option<MessageMeta>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MessageMeta {
    pub model: String,
    pub provider: String,
    pub session_id: String,
    pub project_id: String,
    pub prompt_template_id: String,
    pub prompt_hash: String,
    pub tokens: TokenUsage,
    pub latency_ms: u64,
    pub created_at: String,
}
```

---

## 3. Tauri Commands (Bridge API)

Frontend (React) sẽ giao tiếp với Backend (Rust) qua các hàm này:

- `load_providers() -> Vec<ProviderConfig>`: Đọc từ `~/.merlink/config.json`. Nếu chưa có, trả về default.
- `save_providers(providers: Vec<ProviderConfig>) -> Result<(), String>`: Ghi xuống file.
- `send_chat(provider_id: String, model_id: String, messages: Vec<MessageEnvelope>, project_id: String) -> Result<MessageEnvelope, String>`:
  - Xây dựng system prompt (inject identity + model info).
  - Gọi API tương ứng (OpenAI/Anthropic).
  - Đo lường token, latency, tính hash prompt.
  - Trả về `MessageEnvelope` đầy đủ metadata.
- `execute_tool(tool_name: String, args: serde_json::Value) -> Result<serde_json::Value, String>`: Thực thi tool (demo).

---

## 4. System Prompt Template

* [ ] Lưu tại `src-tauri/prompts/harness_system.md`:

```text
You are Merlink, a narrative harness orchestrator.
Runtime: {provider_name} / {model_name}
Session: {session_id}
Project: {project_id}

# Rules
- You orchestrate via tools, not direct prose unless explicitly asked.
- You do not impersonate other models. If asked "who are you?", answer "I am Merlink, running on {model_name} via {provider_name}."
- You do not invent world facts not present in the knowledge base.
```
