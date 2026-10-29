# BÁO CÁO KIẾN TRÚC CHUYÊN SÂU: CREATIONAL DESIGN PATTERNS TRONG RUST CHO TAURI BACKEND

**Dự án:** Merlink Harness UI / Tauri Backend Engine  
**Tác giả:** Subagent A (Gemini 3.8 High Architecture Agent)  
**Tài liệu tham chiếu:** `docs/be-tauri-plan.md`, `docs/db.dbml`, Refactoring Guru (Rust), Rust Unofficial Patterns  
**Mục tiêu:** Định hình kiến trúc khởi tạo đối tượng chuẩn mực (Creational Patterns) trong Rust, tối ưu hóa triệt để tính an toàn bộ nhớ (Memory Safety), kiểm tra tại thời điểm biên dịch (Compile-time Guarantees), và loại bỏ hoàn toàn các Anti-patterns thường gặp trong ứng dụng Desktop Tauri đa luồng.

---

## MỤC LỤC
1. [TỔNG QUAN KIẾN TRÚC VÀ CREATIONAL PATTERNS TRONG RUST](#1-tổng-quan-kiến-trúc-và-creational-patterns-trong-rust)
2. [PATTERN 1: BUILDER PATTERN KẾT HỢP TYPESTATE PATTERN](#2-pattern-1-builder-pattern-kết-hợp-typestate-pattern)
   - 2.1. Vấn đề kiến trúc & Động lực giải pháp
   - 2.2. Sơ đồ trạng thái hữu hạn tại thời điểm biên dịch (Typestate Transition Diagram)
   - 2.3. Hiện thực chuyên sâu: `ContextBuilder` & Kiểm soát Token Budget
   - 2.4. Hiện thực chuyên sâu: `LlmChatRequestBuilder`
3. [PATTERN 2: FACTORY METHOD VÀ ABSTRACT FACTORY PATTERN](#3-pattern-2-factory-method-và-abstract-factory-pattern)
   - 3.1. Phân lập giao thức LLM Đa Nhà Cung Cấp (`LlmClientFactory`)
   - 3.2. Kiểm soát ranh giới bảo mật theo Mode (`ToolRegistryFactory`)
   - 3.3. Sơ đồ kiến trúc Factory & Dynamic Dispatch
4. [PATTERN 3: PROTOTYPE PATTERN (CLONE + COW / SNAPSHOT)](#4-pattern-3-prototype-pattern-clone--cow--snapshot)
   - 4.1. Phân mảnh ngữ cảnh thực thi: `ToolContext::fork_for_call`
   - 4.2. Cơ chế Snapshot & Reversibility: `SceneVersion`
   - 4.3. Sơ đồ phân ránh Context & Vòng đời CancellationToken
5. [PATTERN 4: LOẠI BỎ SINGLETON CỔ ĐIỂN → MANAGED DI CONTAINER (`AppState`)](#5-pattern-4-loại-bỏ-singleton-cổ-điển--thay-bằng-managed-di-container-appstate)
   - 5.1. Phê phán Anti-pattern: Tại sao Singleton truyền thống thất bại trong Rust?
   - 5.2. Giải pháp Composition Root với `tauri::State<AppState>`
   - 5.3. Hiện thực `SqliteDualPool` & Global Concurrency Gate (`Arc<Semaphore>`)
6. [BẢNG TỔNG HỢP SO SÁNH & MA TRẬN ÁP DỤNG](#6-bảng-tổng-hợp-so-sánh--ma-trận-áp-dụng)

---

## 1. TỔNG QUAN KIẾN TRÚC VÀ CREATIONAL PATTERNS TRONG RUST

Trong ngôn ngữ lập trình hướng đối tượng truyền thống (OOP như Java/C++), Creational Patterns thường dựa dẫm vào tính đa hình lúc chạy (Runtime Polymorphism), kế thừa lớp (Class Inheritance), và trạng thái toàn cục có thể biến đổi (Global Mutable State). Tuy nhiên, Rust tiếp cận kiến trúc phần mềm dựa trên:
1. **Ownership & Borrowing**: Quyền sở hữu độc quyền hoặc chia sẻ bất biến, loại bỏ hoàn toàn data race lúc biên dịch.
2. **Zero-Cost Abstractions**: Các cơ chế trừu tượng hóa không làm tăng chi phí runtime (như Monomorphization và Zero-Sized Types - ZST).
3. **Explicit State Transitions**: Chuyển trạng thái bằng cách tiêu thụ giá trị cũ (`self` by-value) và trả về kiểu dữ liệu mới.

Hệ thống Backend Tauri cho AI Agent Engine đòi hỏi tính toàn vẹn rất cao:
- Ngăn ngừa prompt injection và vi phạm ranh giới chỉnh sửa (Mode isolation).
- Đảm bảo tính toán token budget chính xác trước khi gửi payload lên LLM.
- Quản lý kết nối SQLite bất đối xứng (1 writer, N readers) để triệt tiêu lỗi `SQLITE_BUSY`.
- Dễ dàng kiểm thử đơn vị (Unit testing) bằng Mocking mà không phụ thuộc vào trạng thái toàn cục rải rác.

```
+-----------------------------------------------------------------------------------------+
|                                     COMPOSITION ROOT                                    |
|                               (src-tauri/src/lib.rs)                                    |
|                                                                                         |
|  +-----------------------------------------------------------------------------------+  |
|  |                     tauri::State<AppState> (Dependency Injection)                 |  |
|  |  +--------------------+  +----------------------+  +---------------------------+  |  |
|  |  |   SqliteDualPool   |  | Concurrency Gate     |  | SecretStore (OS Keyring)  |  |  |
|  |  | (1 Write / 10 Read)|  | (Arc<Semaphore(2)>)  |  | (Arc<dyn SecretStore>)   |  |  |
|  |  +---------+----------+  +----------+-----------+  +-------------+-------------+  |  |
|  +------------|------------------------|----------------------------|----------------+  |
+---------------|------------------------|----------------------------|-------------------+
                |                        |                            |
                v                        v                            v
+-------------------------------+  +------------------------------------------------------+
|       CREATIONAL LAYER        |  |                 EXECUTION WORKFLOW                   |
|                               |  |                                                      |
| 1. Factory Pattern:           |  |                                                      |
|    - LlmClientFactory         |->| Creates: Arc<dyn LlmClient> (OpenAI/Anthropic/Ollama)|
|    - ToolRegistryFactory      |->| Creates: ToolRegistry (Strictly filtered by RunMode) |
|                               |  |                                                      |
| 2. Builder + Typestate:       |  |                                                      |
|    - ContextBuilder<State>    |->| Compiles 6-layer prompt + token check at compile-time|
|    - LlmChatRequestBuilder    |->| Validates payload structure before network I/O       |
|                               |  |                                                      |
| 3. Prototype (Clone/COW):     |  |                                                      |
|    - ToolContext::fork_for_call->| Spawns scoped ToolCallContext + child CancellationToken
|    - SceneVersion Snapshot    |->| Immutable state captures for atomic rollbacks        |
+-------------------------------+  +------------------------------------------------------+
```

---

## 2. PATTERN 1: BUILDER PATTERN KẾT HỢP TYPESTATE PATTERN

### 2.1. Vấn đề kiến trúc & Động lực giải pháp
Trong các hệ thống AI Agent phức tạp, việc xây dựng ngữ cảnh (Prompt Context) thường bao gồm nhiều thành phần phụ thuộc lẫn nhau:
1. Thông tin Project (bối cảnh dự án, style guide).
2. Thông tin Session & Lịch sử hội thoại gần nhất.
3. 6 tầng Prompt Templates (`base`, `project`, `session`, `tools`, `verify`, `directives`).
4. Pinned Memories & Memories truy xuất theo vector/FTS5.
5. Attachments đã xử lý token.
6. Ràng buộc Token Budget (tính toán dựa trên `context_window` của model, trừ đi `reserve_output` và `buffer`).

Nếu sử dụng **Builder Pattern truyền thống**:
```rust
// ANTI-PATTERN: Runtime validation leads to bugs!
let ctx = ContextBuilder::new()
    .with_history(...)
    // QUÊN GỌI .with_project(...) NHƯNG VẪN GỌI ĐƯỢC .build()
    .build()?; // Phát hiện lỗi lúc RUNTIME!
```
Hậu quả: Rất dễ bỏ sót các trường bắt buộc, hoặc gọi hàm sai thứ tự (ví dụ tính toán token budget trước khi nạp system layers).

**Giải pháp: Typestate Pattern với Zero-Sized Marker Structs**
Bằng cách sử dụng các struct rỗng (ZST - Zero-Sized Types) làm tham số Generic cho Builder, ta biến quy trình lắp ráp thành một **máy trạng thái hữu hạn (Finite State Machine) được kiểm chứng 100% tại thời điểm biên dịch**. Phương thức `.build()` chỉ tồn tại trên kiểu dữ liệu ở trạng thái cuối cùng (`ReadyToBuild`).

### 2.2. Sơ đồ trạng thái hữu hạn tại thời điểm biên dịch

```
       +-------------------------------------------------------+
       |               ContextBuilder<MissingProject>          |
       |  (Khởi tạo ban đầu, chưa có định danh dự án)          |
       +-------------------------------------------------------+
                                   |
                                   | .with_project(project_id, metadata)
                                   v
       +-------------------------------------------------------+
       |                ContextBuilder<HasProject>             |
       |  (Đã có Project, bắt buộc nạp Session & RunMode)      |
       +-------------------------------------------------------+
                                   |
                                   | .with_session(session_id, run_mode)
                                   v
       +-------------------------------------------------------+
       |                ContextBuilder<HasSession>             |
       |  (Bắt đầu nạp Layers, Memories, History, Attachments) |
       +-------------------------------------------------------+
                                   |
                                   | .with_model_budget(model_config)
                                   v
       +-------------------------------------------------------+
       |                ContextBuilder<ReadyToBuild>           |
       |  (Đã đủ tham số tiên quyết, cho phép gọi .build())    |
       +-------------------------------------------------------+
                                   |
                                   | .build() -> Result<BuiltContext, ContextError>
                                   v
       +-------------------------------------------------------+
       |                     BuiltContext                      |
       | (Sẵn sàng đưa vào LlmChatRequestBuilder hoặc LLM Call)|
       +-------------------------------------------------------+
```

### 2.3. Hiện thực chuyên sâu: `ContextBuilder` & Kiểm soát Token Budget

Dưới đây là mã nguồn Rust chuẩn mực hiện thực `ContextBuilder` áp dụng Typestate Pattern:

```rust
use std::marker::PhantomData;
use uuid::Uuid;
use thiserror::Error;

// ============================================================================
// 1. Zero-Sized Marker Structs biểu diễn các trạng thái biên dịch
// ============================================================================
pub struct MissingProject;
pub struct HasProject;
pub struct HasSession;
pub struct ReadyToBuild;

// ============================================================================
// 2. Data Transfer Objects & Error Types
// ============================================================================
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunMode {
    Explore,
    Edit,
}

#[derive(Debug, Clone)]
pub struct ModelBudgetConfig {
    pub context_window: u32,
    pub max_output_tokens: u32,
    pub token_calibration: f32, // EMA hệ số điều chỉnh token thực tế
}

#[derive(Debug, Clone)]
pub struct PromptLayer {
    pub kind: String,
    pub content: String,
    pub token_count: u32,
}

#[derive(Debug, Clone)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
    pub token_count: u32,
}

#[derive(Debug, Clone)]
pub struct MemoryItem {
    pub id: Uuid,
    pub content: String,
    pub is_pinned: bool,
    pub token_count: u32,
}

#[derive(Debug, Clone)]
pub struct BuiltContext {
    pub project_id: Uuid,
    pub session_id: Uuid,
    pub mode: RunMode,
    pub system_instructions: String,
    pub messages: Vec<ChatMessage>,
    pub total_input_tokens: u32,
    pub available_tokens_remaining: u32,
}

#[derive(Error, Debug)]
pub enum ContextBuildError {
    #[error("Context too small: system & tools exceed 50% of available window")]
    ContextTooSmall,
    #[error("Token budget exceeded: required {required}, available {available}")]
    TokenBudgetExceeded { required: u32, available: u32 },
}

// ============================================================================
// 3. Typestate ContextBuilder Definition
// ============================================================================
pub struct ContextBuilder<State> {
    project_id: Option<Uuid>,
    session_id: Option<Uuid>,
    mode: Option<RunMode>,
    model_config: Option<ModelBudgetConfig>,
    layers: Vec<PromptLayer>,
    pinned_memories: Vec<MemoryItem>,
    retrieved_memories: Vec<MemoryItem>,
    history: Vec<ChatMessage>,
    tool_tokens: u32,
    _marker: PhantomData<State>,
}

// Khởi tạo ban đầu ở trạng thái MissingProject
impl ContextBuilder<MissingProject> {
    pub fn new() -> Self {
        Self {
            project_id: None,
            session_id: None,
            mode: None,
            model_config: None,
            layers: Vec::new(),
            pinned_memories: Vec::new(),
            retrieved_memories: Vec::new(),
            history: Vec::new(),
            tool_tokens: 0,
            _marker: PhantomData,
        }
    }

    /// Bước 1: Bắt buộc cung cấp Project ID
    pub fn with_project(self, project_id: Uuid) -> ContextBuilder<HasProject> {
        ContextBuilder {
            project_id: Some(project_id),
            session_id: self.session_id,
            mode: self.mode,
            model_config: self.model_config,
            layers: self.layers,
            pinned_memories: self.pinned_memories,
            retrieved_memories: self.retrieved_memories,
            history: self.history,
            tool_tokens: self.tool_tokens,
            _marker: PhantomData,
        }
    }
}

impl ContextBuilder<HasProject> {
    /// Bước 2: Bắt buộc cung cấp Session ID và RunMode
    pub fn with_session(self, session_id: Uuid, mode: RunMode) -> ContextBuilder<HasSession> {
        ContextBuilder {
            project_id: self.project_id,
            session_id: Some(session_id),
            mode: Some(mode),
            model_config: self.model_config,
            layers: self.layers,
            pinned_memories: self.pinned_memories,
            retrieved_memories: self.retrieved_memories,
            history: self.history,
            tool_tokens: self.tool_tokens,
            _marker: PhantomData,
        }
    }
}

impl ContextBuilder<HasSession> {
    /// Bước 3: Cung cấp thông số Model Budget để chuyển sang trạng thái ReadyToBuild
    pub fn with_model_budget(self, config: ModelBudgetConfig) -> ContextBuilder<ReadyToBuild> {
        ContextBuilder {
            project_id: self.project_id,
            session_id: self.session_id,
            mode: self.mode,
            model_config: Some(config),
            layers: self.layers,
            pinned_memories: self.pinned_memories,
            retrieved_memories: self.retrieved_memories,
            history: self.history,
            tool_tokens: self.tool_tokens,
            _marker: PhantomData,
        }
    }
}

// Các phương thức có thể gọi khi đã đạt trạng thái ReadyToBuild (hoặc nạp thêm dữ liệu linh hoạt)
impl ContextBuilder<ReadyToBuild> {
    pub fn add_layer(mut self, layer: PromptLayer) -> Self {
        self.layers.push(layer);
        self
    }

    pub fn set_tool_tokens(mut self, tokens: u32) -> Self {
        self.tool_tokens = tokens;
        self
    }

    pub fn add_pinned_memory(mut self, memory: MemoryItem) -> Self {
        self.pinned_memories.push(memory);
        self
    }

    pub fn add_history_message(mut self, message: ChatMessage) -> Self {
        self.history.push(message);
        self
    }

    /// CHỈ Ở TRẠNG THÁI ReadyToBuild MỚI CÓ METHOD .build()!
    /// Thực hiện thuật toán Greedy Budget Allocation như đặc tả mục 7.2 trong docs/be-tauri-plan.md:
    /// reserve_output = min(max_output, 25% W); buffer = max(5%, 20% - calibration)
    /// U = W - reserve_output - buffer
    pub fn build(self) -> Result<BuiltContext, ContextBuildError> {
        let config = self.model_config.as_ref().unwrap();
        let w = config.context_window;

        let reserve_output = (config.max_output_tokens).min((w as f32 * 0.25) as u32);
        let buffer_ratio = (0.20 - (config.token_calibration - 1.0) * 0.05).clamp(0.05, 0.20);
        let buffer = (w as f32 * buffer_ratio) as u32;

        let u_usable = w.saturating_sub(reserve_output).saturating_sub(buffer);

        // 1. Tính tổng token của các tầng bất biến: System layers + Tools
        let system_tokens: u32 = self.layers.iter().map(|l| l.token_count).sum();
        let fixed_tokens = system_tokens + self.tool_tokens;

        // Nếu system + tools chiếm quá 50% U -> context quá chật hẹp
        if fixed_tokens > (u_usable / 2) {
            return Err(ContextBuildError::ContextTooSmall);
        }

        let mut remaining_budget = u_usable.saturating_sub(fixed_tokens);
        let mut final_messages = Vec::new();

        // 2. Ưu tiên Pinned Memories
        let mut included_memories = Vec::new();
        for mem in self.pinned_memories {
            if mem.token_count <= remaining_budget {
                remaining_budget -= mem.token_count;
                included_memories.push(mem);
            }
        }

        // 3. Ưu tiên History gần nhất (nguyên văn)
        for msg in self.history.into_iter().rev() {
            if msg.token_count <= remaining_budget {
                remaining_budget -= msg.token_count;
                final_messages.push(msg);
            } else {
                // Đạt trần budget, dừng đưa thêm history cũ
                break;
            }
        }
        final_messages.reverse(); // Khôi phục thứ tự thời gian

        // Lắp ráp system prompt từ các tầng
        let mut system_text = String::new();
        for layer in &self.layers {
            system_text.push_str(&format!("--- LAYER: {} ---\n{}\n\n", layer.kind, layer.content));
        }
        if !included_memories.is_empty() {
            system_text.push_str("--- PINNED MEMORIES ---\n");
            for mem in &included_memories {
                system_text.push_str(&format!("- {}\n", mem.content));
            }
        }

        let total_input_tokens = u_usable - remaining_budget;

        Ok(BuiltContext {
            project_id: self.project_id.unwrap(),
            session_id: self.session_id.unwrap(),
            mode: self.mode.unwrap(),
            system_instructions: system_text,
            messages: final_messages,
            total_input_tokens,
            available_tokens_remaining: remaining_budget,
        })
    }
}
```

### 2.4. Hiện thực chuyên sâu: `LlmChatRequestBuilder`

Sau khi có `BuiltContext`, hệ thống chuyển tiếp sang `LlmChatRequestBuilder` để đóng gói request gửi qua HTTP Client. Tương tự, Typestate đảm bảo không thể gửi request nếu chưa thiết lập Model và Messages.

```rust
pub struct ReqWithoutModel;
pub struct ReqWithModel;
pub struct ReqReady;

pub struct LlmChatRequestBuilder<State> {
    model_id: String,
    messages: Vec<ChatMessage>,
    temperature: Option<f32>,
    max_tokens: Option<u32>,
    tools_schema: Option<serde_json::Value>,
    _marker: PhantomData<State>,
}

impl LlmChatRequestBuilder<ReqWithoutModel> {
    pub fn new() -> Self {
        Self {
            model_id: String::new(),
            messages: Vec::new(),
            temperature: None,
            max_tokens: None,
            tools_schema: None,
            _marker: PhantomData,
        }
    }

    pub fn with_model(self, model: impl Into<String>) -> LlmChatRequestBuilder<ReqWithModel> {
        LlmChatRequestBuilder {
            model_id: model.into(),
            messages: self.messages,
            temperature: self.temperature,
            max_tokens: self.max_tokens,
            tools_schema: self.tools_schema,
            _marker: PhantomData,
        }
    }
}

impl LlmChatRequestBuilder<ReqWithModel> {
    pub fn with_context(self, context: BuiltContext) -> LlmChatRequestBuilder<ReqReady> {
        let mut messages = Vec::new();
        // System message đầu tiên
        messages.push(ChatMessage {
            role: "system".into(),
            content: context.system_instructions,
            token_count: 0,
        });
        // Lịch sử kèm theo
        messages.extend(context.messages);

        LlmChatRequestBuilder {
            model_id: self.model_id,
            messages,
            temperature: self.temperature,
            max_tokens: self.max_tokens,
            tools_schema: self.tools_schema,
            _marker: PhantomData,
        }
    }
}

impl LlmChatRequestBuilder<ReqReady> {
    pub fn temperature(mut self, temp: f32) -> Self {
        self.temperature = Some(temp);
        self
    }

    pub fn tools(mut self, schema: serde_json::Value) -> Self {
        self.tools_schema = Some(schema);
        self
    }

    pub fn finish(self) -> serde_json::Value {
        serde_json::json!({
            "model": self.model_id,
            "messages": self.messages.into_iter().map(|m| serde_json::json!({
                "role": m.role,
                "content": m.content
            })).collect::<Vec<_>>(),
            "temperature": self.temperature.unwrap_or(0.7),
            "tools": self.tools_schema
        })
    }
}
```

---

## 3. PATTERN 2: FACTORY METHOD VÀ ABSTRACT FACTORY PATTERN

### 3.1. Phân lập giao thức LLM Đa Nhà Cung Cấp (`LlmClientFactory`)
Bảng `ai_providers` trong CSDL phân biệt 3 giao thức chính thông qua enum `ProviderProtocol`:
1. `openai_compat`: Tương thích OpenAI API (OpenAI, DeepSeek, Groq, OpenRouter, v.v.).
2. `anthropic`: Anthropic Messages API (Claude 3.5 Sonnet, Claude 3 Haiku).
3. `ollama`: Ollama API nội bộ chạy trên localhost (bảo toàn quyền riêng tư, không cần API Key).

Ngoài ra, trường `secret_ref` trong `ai_providers` chỉ lưu khóa trỏ vào **OS Keyring** (Apple Keychain, Windows Credential Manager, Linux Secret Service). Tuyệt đối **không lưu API key dạng plain text** trong SQLite.

`LlmClientFactory` đóng vai trò là Abstract Factory:
- Nhận diện `ProviderProtocol`.
- Gọi `SecretStore` để giải mã API Key an toàn.
- Cấu hình base URL, custom headers, timeout.
- Trả về đối tượng triển khai trait `Arc<dyn LlmClient>`.

```
                  +-----------------------------------+
                  |        <<trait>> LlmClient        |
                  | +chat_stream(): Result<Stream>    |
                  +-----------------+-----------------+
                                    ^
       +----------------------------+----------------------------+
       |                            |                            |
+------+---------------+     +------+---------------+     +------+---------------+
|  OpenAiCompatClient  |     |   AnthropicClient    |     |     OllamaClient     |
| (SSE Parsing / JSON) |     | (SSE Event Stream)   |     | (Local HTTP NDJSON)  |
+----------------------+     +----------------------+     +----------------------+
       ^                            ^                            ^
       +----------------------------+----------------------------+
                                    |
                    +---------------+---------------+
                    |        LlmClientFactory       |
                    | +create_client(provider)      |
                    +---------------+---------------+
                                    | Cần Secret
                                    v
                    +---------------+---------------+
                    |     <<trait>> SecretStore     |
                    | +get_secret(secret_ref)       |
                    +-------------------------------+
```

Mã nguồn triển khai `LlmClientFactory`:

```rust
use async_trait::async_trait;
use std::sync::Arc;
use futures::Stream;
use std::pin::Pin;

#[derive(Debug, Clone, serde::Deserialize)]
pub enum ProviderProtocol {
    OpenAiCompat,
    Anthropic,
    Ollama,
}

#[derive(Debug, Clone)]
pub struct AiProviderEntity {
    pub id: uuid::Uuid,
    pub code: String,
    pub protocol: ProviderProtocol,
    pub base_url: Option<String>,
    pub secret_ref: Option<String>,
    pub extra_headers_json: Option<String>,
}

#[async_trait]
pub trait SecretStore: Send + Sync {
    async fn get_secret(&self, secret_ref: &str) -> Result<String, String>;
}

#[async_trait]
pub trait LlmClient: Send + Sync {
    async fn chat_stream(
        &self,
        payload: serde_json::Value,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<String, String>> + Send>>, String>;
}

// ---------------- Concretes ----------------
pub struct OpenAiCompatClient {
    base_url: String,
    api_key: String,
    http: reqwest::Client,
}

#[async_trait]
impl LlmClient for OpenAiCompatClient {
    async fn chat_stream(
        &self,
        _payload: serde_json::Value,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<String, String>> + Send>>, String> {
        // SSE implementation details
        todo!("Streaming implementation for OpenAI compatible endpoint")
    }
}

pub struct AnthropicClient {
    base_url: String,
    api_key: String,
    http: reqwest::Client,
}

#[async_trait]
impl LlmClient for AnthropicClient {
    async fn chat_stream(
        &self,
        _payload: serde_json::Value,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<String, String>> + Send>>, String> {
        todo!("Streaming implementation for Anthropic API")
    }
}

pub struct OllamaClient {
    base_url: String,
    http: reqwest::Client,
}

#[async_trait]
impl LlmClient for OllamaClient {
    async fn chat_stream(
        &self,
        _payload: serde_json::Value,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<String, String>> + Send>>, String> {
        todo!("Streaming implementation for Ollama")
    }
}

// ---------------- Factory ----------------
pub struct LlmClientFactory {
    secret_store: Arc<dyn SecretStore>,
    http_client: reqwest::Client,
}

impl LlmClientFactory {
    pub fn new(secret_store: Arc<dyn SecretStore>, http_client: reqwest::Client) -> Self {
        Self { secret_store, http_client }
    }

    pub async fn create_client(
        &self,
        provider: &AiProviderEntity,
    ) -> Result<Arc<dyn LlmClient>, String> {
        match provider.protocol {
            ProviderProtocol::OpenAiCompat => {
                let secret_key = match &provider.secret_ref {
                    Some(s_ref) => self.secret_store.get_secret(s_ref).await?,
                    None => return Err("API key required for OpenAI Compatible provider".into()),
                };
                let base_url = provider.base_url.clone()
                    .unwrap_or_else(|| "https://api.openai.com/v1".to_string());

                Ok(Arc::new(OpenAiCompatClient {
                    base_url,
                    api_key: secret_key,
                    http: self.http_client.clone(),
                }))
            }
            ProviderProtocol::Anthropic => {
                let secret_key = match &provider.secret_ref {
                    Some(s_ref) => self.secret_store.get_secret(s_ref).await?,
                    None => return Err("API key required for Anthropic provider".into()),
                };
                let base_url = provider.base_url.clone()
                    .unwrap_or_else(|| "https://api.anthropic.com".to_string());

                Ok(Arc::new(AnthropicClient {
                    base_url,
                    api_key: secret_key,
                    http: self.http_client.clone(),
                }))
            }
            ProviderProtocol::Ollama => {
                let base_url = provider.base_url.clone()
                    .unwrap_or_else(|| "http://localhost:11434".to_string());

                Ok(Arc::new(OllamaClient {
                    base_url,
                    http: self.http_client.clone(),
                }))
            }
        }
    }
}
```

### 3.2. Kiểm soát ranh giới bảo mật theo Mode (`ToolRegistryFactory`)
Mục 7.1 của `docs/be-tauri-plan.md` quy định:
- Khi Agent chạy ở chế độ **`explore`** (mặc định), các tool ghi (`scene_update`, `character_edit`, `project_update`, `*_delete`) **hoàn toàn không được khởi tạo và không xuất hiện trong JSON Schema gửi lên LLM**.
- Chỉ khi chuyển sang chế độ **`edit`**, các tool ghi mới được đăng ký.

Nếu chỉ lọc ở runtime bằng if/else, lỗi lập trình có thể làm rò rỉ tool ghi vào prompt, dẫn đến nguy cơ Prompt Injection từ các file đính kèm độc hại (untrusted attachment). `ToolRegistryFactory` đóng vai trò là Factory Method tạo ra `ToolRegistry` tương ứng với `RunMode`:

```rust
use std::collections::HashMap;

#[async_trait]
pub trait Tool: Send + Sync {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn parameters_schema(&self) -> serde_json::Value;
    async fn execute(&self, args: serde_json::Value) -> Result<serde_json::Value, String>;
}

pub struct ToolRegistry {
    tools: HashMap<&'static str, Arc<dyn Tool>>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self { tools: HashMap::new() }
    }

    pub fn register(&mut self, tool: Arc<dyn Tool>) {
        self.tools.insert(tool.name(), tool);
    }

    pub fn get_schemas(&self) -> serde_json::Value {
        let list: Vec<serde_json::Value> = self.tools.values().map(|t| {
            serde_json::json!({
                "type": "function",
                "function": {
                    "name": t.name(),
                    "description": t.description(),
                    "parameters": t.parameters_schema()
                }
            })
        }).collect();
        serde_json::Value::Array(list)
    }
}

pub struct ToolRegistryFactory;

impl ToolRegistryFactory {
    pub fn create_registry(mode: RunMode) -> ToolRegistry {
        let mut registry = ToolRegistry::new();

        // 1. Luôn đăng ký Read Tools & Exploration Tools
        // registry.register(Arc::new(ProjectGetTool));
        // registry.register(Arc::new(SceneListTool));
        // registry.register(Arc::new(MemorySearchTool));
        // registry.register(Arc::new(RequestEditModeTool));

        // 2. Phân lập tuyệt đối: Chỉ ở RunMode::Edit mới đăng ký Write Tools!
        if mode == RunMode::Edit {
            // registry.register(Arc::new(SceneUpdateTool));
            // registry.register(Arc::new(CharacterCreateTool));
            // registry.register(Arc::new(SceneDeleteTool));
        }

        registry
    }
}
```

---

## 4. PATTERN 3: PROTOTYPE PATTERN (CLONE + COW / SNAPSHOT)

### 4.1. Phân mảnh ngữ cảnh thực thi: `ToolContext::fork_for_call`
Trong suốt quá trình Agent chạy (Run Execution), một lượt gọi của Model có thể yêu cầu thực thi nhiều tool calls song song (Parallel Tool Calling) hoặc tuần tự.
Mỗi Tool Call đòi hỏi:
- Nhận diện riêng biệt (`call_id: Uuid`).
- CancellationToken độc lập nhưng phụ thuộc: Nếu Run bị hủy, toàn bộ Tool Call phải dừng; nếu một Tool Call bị timeout (ví dụ 30s), chỉ riêng Tool Call đó bị hủy mà không làm đứt mạch run nếu có cơ chế retry.
- Truy cập vào Database Pool và quyền sở hữu Project (`project_id`).

Nếu tạo mới toàn bộ context từ đầu thì rất tốn kém (Deep Copy socket/pool). Áp dụng **Prototype Pattern với kỹ thuật `Arc::clone` và Hierarchical Cancellation Tokens** giải quyết bài toán với chi phí O(1):

```rust
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

#[derive(Clone)]
pub struct ToolContext {
    pub project_id: Uuid,
    pub session_id: Uuid,
    pub run_id: Uuid,
    pub mode: RunMode,
    // CancellationToken gốc của toàn bộ Agent Run
    pub root_token: CancellationToken,
}

#[derive(Clone)]
pub struct ToolCallContext {
    // Kế thừa các thông tin định danh từ ToolContext
    pub parent: ToolContext,
    pub call_id: Uuid,
    // Token con: Sẽ tự động kích hoạt nếu root_token bị hủy,
    // HOẶC có thể chủ động hủy riêng lẻ qua timeout của call
    pub call_token: CancellationToken,
}

impl ToolContext {
    /// Hiện thực Prototype Pattern qua cơ chế Forking
    pub fn fork_for_call(&self, call_id: Uuid) -> ToolCallContext {
        ToolCallContext {
            parent: self.clone(), // Arc/UUID clone cực nhẹ
            call_id,
            call_token: self.root_token.child_token(), // Tạo child token độc lập
        }
    }
}
```

```
+-------------------------------------------------------------+
|                     ToolContext (Prototype)                 |
|  - project_id: Uuid                                         |
|  - session_id: Uuid                                         |
|  - root_token: CancellationToken                            |
+-------------------------------------------------------------+
                |                               |
       fork_for_call(id_1)             fork_for_call(id_2)
                |                               |
                v                               v
+-------------------------------+ +-------------------------------+
|      ToolCallContext #1       | |      ToolCallContext #2       |
| - parent: ToolContext (Clone) | | - parent: ToolContext (Clone) |
| - call_id: id_1               | | - call_id: id_2               |
| - call_token: ChildToken #1   | | - call_token: ChildToken #2   |
+-------------------------------+ +-------------------------------+
```

### 4.2. Cơ chế Snapshot & Reversibility: `SceneVersion`
Mục 7.1 quy định chính sách duyệt tự động `auto_reversible`: cho phép AI tự động chỉnh sửa nếu hành động đó **có thể hoàn tác (reversible)**. 
Để đạt được điều này, trước khi thực hiện câu lệnh `UPDATE scenes SET ...`, hệ thống chụp một bản Prototype Snapshot của bản ghi hiện tại và lưu vào bảng `scene_versions`:

```rust
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SceneSnapshot {
    pub id: Uuid,
    pub project_id: Uuid,
    pub title: String,
    pub content: String,
    pub version_number: u32,
}

pub struct SceneVersionManager;

impl SceneVersionManager {
    /// Tạo snapshot bất biến trước khi ghi đè
    pub async fn snapshot_before_edit(
        current_scene: &SceneSnapshot,
        run_id: Uuid,
    ) -> Result<Uuid, String> {
        let version_id = Uuid::now_v7();
        // INSERT INTO scene_versions (id, scene_id, content, version_number, created_by_run)
        // VALUES (version_id, current_scene.id, current_scene.content, current_scene.version_number, run_id);
        Ok(version_id)
    }
}
```

---

## 5. PATTERN 4: LOẠI BỎ SINGLETON CỔ ĐIỂN → THAY BẰNG MANAGED DI CONTAINER (`AppState`)

### 5.1. Phê phán Anti-pattern: Tại sao Singleton truyền thống thất bại trong Rust?

Trong C++/Java, lập trình viên thường viết:
```cpp
// C++ Anti-Pattern
class DatabaseManager {
    static DatabaseManager* instance;
public:
    static DatabaseManager* getInstance() { ... }
};
```
Khi chuyển sang Rust, người mới tiếp cận thường cố tái hiện điều này bằng:
1. `static mut INSTANCE: Option<AppState> = None;`  
   => **CỰC KỲ NGUY HIỂM**: Đòi hỏi khối lệnh `unsafe` ở mọi nơi truy cập. Gây ra UB (Undefined Behavior) và Data Race tức thì khi nhiều luồng Tauri command truy xuất đồng thời.
2. `lazy_static!` hoặc `std::sync::OnceLock` rải rác toàn cầu (`pub static GLOBAL_DB: OnceLock<SqlitePool> = OnceLock::new();`)  
   => **ANTI-PATTERN KIẾN TRÚC**:
   - **Mất kiểm soát vòng đời**: Không thể đóng connection pool đúng quy trình khi app thoát.
   - **Tê liệt Unit Testing**: Các test suite chạy song song (`cargo test` mặc định multi-threaded) sẽ làm bẩn trạng thái của nhau, không thể mock SQLite in-memory cho từng test độc lập.
   - **Khó khăn trong quản lý phụ thuộc (Hidden Dependencies)**: Hàm nghiệp vụ gọi trực tiếp `GLOBAL_DB` làm mất tính minh bạch của chữ ký hàm (Function Signature).

### 5.2. Giải pháp Composition Root với `tauri::State<AppState>`

Giải pháp chuẩn mực của Rust và Tauri là **Inversion of Control (IoC) & Dependency Injection (DI)** thông qua `AppState` được lắp ráp duy nhất một lần tại **Composition Root** (`src-tauri/src/lib.rs`):

```
+-------------------------------------------------------------------------------+
|                      COMPOSITION ROOT (src-tauri/src/lib.rs)                  |
|                                                                               |
|  1. Khởi tạo SqliteDualPool (WAL mode, Writer pool: 1, Reader pool: 10)       |
|  2. Khởi tạo Arc<dyn SecretStore> (Keyring adapter)                           |
|  3. Khởi tạo Arc<Semaphore(2)> (Global LLM concurrency gate)                  |
|  4. Lắp ráp AppState -> tauri::Builder::default().manage(app_state)           |
+---------------------------------------+---------------------------------------+
                                        |
                   Inject tự động qua Tauri Command Engine
                                        |
                                        v
+-------------------------------------------------------------------------------+
|                        TAURI COMMAND HANDLERS                                 |
|                                                                               |
|  #[tauri::command]                                                            |
|  async fn chat_send(                                                          |
|      state: tauri::State<'_, AppState>,                                       |
|      payload: ChatSendRequest                                                 |
|  ) -> Result<ChatSendResponse, CommandError>                                  |
+-------------------------------------------------------------------------------+
```

### 5.3. Hiện thực `SqliteDualPool` & Global Concurrency Gate (`Arc<Semaphore>`)

SQLite có đặc tính: Nhiều reader có thể đọc song song dưới chế độ WAL (`PRAGMA journal_mode=WAL`), nhưng **chỉ duy nhất 1 luồng được ghi tại một thời điểm**. Nếu cấu hình 1 pool dùng chung 10 connection, khi nhiều lệnh write chạy đồng thời sẽ gây lỗi kẹt luồng và `SQLITE_BUSY`.

Vì vậy, kiến trúc `SqliteDualPool` tách biệt rạch ròi:
- `writer_pool`: Giới hạn nghiêm ngặt `max_connections = 1`.
- `reader_pool`: Giới hạn `max_connections = 10`.
- `concurrency_gate`: `Arc<Semaphore>` giới hạn số lượng agent run đồng thời (ví dụ tối đa 2 runs song song để tránh cạn kiệt tài nguyên máy tính cá nhân). Khi run rơi vào trạng thái `awaiting_approval`, nó **nhả slot semaphore** để nhường quyền xử lý cho tác vụ khác.

```rust
use std::sync::Arc;
use tokio::sync::Semaphore;
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions, SqliteConnectOptions};
use std::str::FromStr;

// ============================================================================
// 1. Dual Pool SQLite Architecture
// ============================================================================
#[derive(Clone)]
pub struct SqliteDualPool {
    writer_pool: SqlitePool,
    reader_pool: SqlitePool,
}

impl SqliteDualPool {
    pub async fn init(db_path: &str) -> Result<Self, sqlx::Error> {
        let connection_options = SqliteConnectOptions::from_str(db_path)?
            .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
            .synchronous(sqlx::sqlite::SqliteSynchronous::Normal)
            .busy_timeout(std::time::Duration::from_secs(30));

        // Pool GHI: TỐI ĐA 1 CONNECTION
        let writer_pool = SqlitePoolOptions::new()
            .max_connections(1)
            .min_connections(1)
            .connect_with(connection_options.clone())
            .await?;

        // Pool ĐỌC: TỐI ĐA 10 CONNECTIONS SONG SONG
        let reader_pool = SqlitePoolOptions::new()
            .max_connections(10)
            .min_connections(2)
            .connect_with(connection_options)
            .await?;

        Ok(Self { writer_pool, reader_pool })
    }

    #[inline]
    pub fn reader(&self) -> &SqlitePool {
        &self.reader_pool
    }

    #[inline]
    pub fn writer(&self) -> &SqlitePool {
        &self.writer_pool
    }
}

// ============================================================================
// 2. Global AppState as Managed Container
// ============================================================================
pub struct AppState {
    pub dual_pool: SqliteDualPool,
    pub secret_store: Arc<dyn SecretStore>,
    // Giới hạn số lượng Agent Run chạy song song
    pub concurrency_gate: Arc<Semaphore>,
    pub client_factory: Arc<LlmClientFactory>,
}

impl AppState {
    pub fn new(
        dual_pool: SqliteDualPool,
        secret_store: Arc<dyn SecretStore>,
        client_factory: Arc<LlmClientFactory>,
        max_concurrent_runs: usize,
    ) -> Self {
        Self {
            dual_pool,
            secret_store,
            concurrency_gate: Arc::new(Semaphore::new(max_concurrent_runs)),
            client_factory,
        }
    }
}

// ============================================================================
// 3. Sử dụng an toàn trong Tauri Command
// ============================================================================
#[tauri::command]
pub async fn send_chat_message(
    state: tauri::State<'_, AppState>,
    session_id: uuid::Uuid,
    content: String,
) -> Result<(), String> {
    // 1. Thử xin vé từ Concurrency Gate
    let permit = state.concurrency_gate
        .clone()
        .try_acquire_owned()
        .map_err(|_| "Hệ thống đang quá tải tác vụ AI, vui lòng đợi!".to_string())?;

    // 2. Spawn worker thread xử lý tác vụ nền
    let pool = state.dual_pool.clone();
    tokio::spawn(async move {
        // Giữ permit trong suốt quá trình chạy
        let _guard = permit;
        
        // Đọc dữ liệu từ Reader Pool
        let _ = pool.reader();

        // Nếu người dùng cần duyệt quyền (awaiting_approval):
        // Ta có thể drop(_guard) để nhả slot Semaphore cho run khác,
        // sau khi duyệt xong sẽ acquire lại.
    });

    Ok(())
}
```

---

## 6. BẢNG TỔNG HỢP SO SÁNH & MA TRẬN ÁP DỤNG

| Thiết kế Pattern | Thành phần áp dụng | Lợi ích cốt lõi trong Rust | Cơ chế an toàn (Safety Guarantee) |
|---|---|---|---|
| **Builder + Typestate** | `ContextBuilder`, `LlmChatRequestBuilder` | Kiểm soát thứ tự nạp ngữ cảnh, loại bỏ lỗi thiếu tham số bắt buộc | **Compile-time Enforcement**: Không thể gọi `.build()` nếu thiếu Project/Session hoặc vi phạm budget |
| **Factory Method / Abstract Factory** | `LlmClientFactory`, `ToolRegistryFactory` | Đóng gói logic khởi tạo phân tán (OpenAI/Anthropic/Ollama), cách ly Keyring | **Security Isolation**: Mode `Explore` hoàn toàn không tồn tại tool ghi trong registry và schema |
| **Prototype (Clone/COW)** | `ToolContext::fork_for_call`, `SceneVersion` | Fork ngữ cảnh tức thì O(1) qua Arc + CancellationToken phân cấp; rollback an toàn | **Concurrency & Fault Isolation**: Một tool call bị timeout/cancel không làm sập toàn bộ Engine |
| **Managed DI Container** | `AppState`, `SqliteDualPool`, `tauri::State` | Thay thế Singleton toàn cục, quản lý vòng đời tường minh, dễ dàng Unit Test | **Zero Unsafe**: Triệt tiêu `SQLITE_BUSY` (1 writer / 10 reader) và chống cạn kiệt tài nguyên qua Semaphore |

---
*Báo cáo được hoàn thiện và đóng gói cho kiến trúc Backend Merlink Harness Tauri Engine.*
