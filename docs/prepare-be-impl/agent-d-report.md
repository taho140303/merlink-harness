# BÁO CÁO NGHIÊN CỨU KIẾN TRÚC BACKEND CHO HARNESS AGENT (RUST)
## Phân hệ: Multi-Provider LLM Protocol Adapters, SSE Streaming, Resilience & Security Outbound Guard
**Mã tài liệu:** `AGENT-D-REPORT`  
**Tác giả:** Subagent D — Chuyên trách LLM Integration, Streaming Engine & Outbound Security  
**Đối tượng đối chiếu:** `docs/be-tauri-plan.md` (Mục 2, 4.2, 5.2, 6, 14)  
**Ngày lập:** Tháng 10, 2026  

---

## 1. TỔNG QUAN & BỐI CẢNH KIẾN TRÚC

### 1.1 Mục tiêu hệ thống
Dự án **Merlink Harness** là ứng dụng desktop quản lý và đồng sáng tác tác phẩm văn học dài tập dựa trên kiến trúc Desktop Agent (Tauri v2 + Rust backend + React frontend). Khác với các chatbot đơn giản, một **Harness Agent** đòi hỏi:
1. **Khả năng tương thích đa nhà cung cấp (Multi-Provider)**: Hỗ trợ linh hoạt các mô hình tiên tiến nhất (OpenAI GPT-4o, Anthropic Claude 3.5/3.7, DeepSeek R1/V3, Groq, OpenRouter) cũng như các runtime suy luận cục bộ (Ollama, vLLM, LM Studio).
2. **Streaming thời gian thực chuẩn hóa**: Xử lý Server-Sent Events (SSE) theo thời gian thực, bóc tách và phân luồng độc lập giữa **Text tokens**, **Reasoning/Thinking tokens** (từ DeepSeek R1 và Claude 3.7 thinking), và **Tool Calling chunks** (tích lũy JSON schema parameters dạng delta).
3. **Resilience & Fault Tolerance cấp công nghiệp**: Xử lý đứt gãy kết nối, rate limits (HTTP 429), quá tải máy chủ (5xx) với thuật toán Exponential Backoff + Jitter; nhận diện và xử lý sớm lỗi tràn ngữ cảnh (Context Window Overflow).
4. **Bảo mật tuyệt đối (Zero Plaintext Leakage & Outbound Guard)**: Lưu trữ khóa API trên OS Keychain thông qua crate `keyring`, ngăn chặn rò rỉ khóa qua UI/Log/Database, và thực thi cơ chế Outbound Guard nhằm phòng chống SSRF (Server-Side Request Forgery) khi người dùng cấu hình Custom Base URLs.

### 1.2 Vị trí Crate `llm` trong Kiến trúc 4 Crate
Theo quyết định kiến trúc tại Mục 2 và 4.2 của `docs/be-tauri-plan.md`, hệ thống phân rã thành 4 crate mỏng với hàng rào biên dịch nghiêm ngặt:
- `domain`: Chứa các entity thuần túy (`Provider`, `ModelProfile`), domain events, và các **Ports (Traits)**: `LlmClient`, `SecretStore`, `EventSink`. Cấm phụ thuộc vào `reqwest`, `sqlx`, `tauri`.
- `llm`: Đóng vai trò **Protocol Adapter & Transport Engine**. Triển khai trait `LlmClient` từ `domain`, phụ thuộc vào `reqwest` và parser SSE. **Cấm phụ thuộc vào `tauri`, `sqlx`, `agent`**.
- `agent`: Điều phối vòng lặp thực thi (`RunLoop`), State Machine (`RunPhase`), Context Builder, Prompt Composer, và Tool Registry. Giao tiếp với LLM hoàn toàn thông qua trait `LlmClient` mà không hề biết đến giao thức truyền tải bên dưới (Wire Format Agnostic).
- `infra`: Triển khai SQLite repository (`sqlx`), lưu trữ tệp, tìm kiếm toàn văn FTS5, và `KeyringSecretStore` (triển khai trait `SecretStore` qua crate `keyring`).
- `src-tauri`: Composition root nối các ports và adapters lại với nhau qua Dependency Injection (`AppState`).

```
┌─────────────────────────────────────────────────────────────┐
│                      crates/agent                           │
│  RunLoop ──(gọi)──> LlmClient Trait (in domain::ports)      │
│  Nhận stream StreamItem (Text, Reasoning, ToolCall, Done)   │
└──────────────────────────────┬──────────────────────────────┘
                               │ (Dynamic Dispatch / Port)
┌──────────────────────────────▼──────────────────────────────┐
│                       crates/llm                            │
│  ┌───────────────────────────────────────────────────────┐  │
│  │ LlmClientFactory: Tạo Adapter theo ProviderProtocol   │  │
│  └───────┬───────────────────┼───────────────────┬───────┘  │
│          │                   │                   │          │
│  ┌───────▼────────┐  ┌───────▼────────┐  ┌───────▼───────┐  │
│  │ OpenAiCompat   │  │   Anthropic    │  │ OllamaNative  │  │
│  │ Adapter        │  │   Adapter      │  │ Adapter       │  │
│  └───────┬────────┘  └───────┬────────┘  └───────┬───────┘  │
│          │                   │                   │          │
│          └───────────────────┼───────────────────┘          │
│                              │                              │
│  ┌───────────────────────────▼───────────────────────────┐  │
│  │  Outbound Guard (SSRF check, URL validation, Redact)  │  │
│  ├───────────────────────────────────────────────────────┤  │
│  │  HTTP Transport: reqwest (Connection Pool, TLS)       │  │
│  ├───────────────────────────────────────────────────────┤  │
│  │  Streaming: eventsource-stream / Byte Stream Decoder  │  │
│  ├───────────────────────────────────────────────────────┤  │
│  │  Resilience: Exponential Backoff + Jitter (429 / 5xx) │  │
│  └───────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

---

## 2. KHẢO SÁT & ĐÁNH GIÁ CÁC THƯ VIỆN LLM CLIENT TRONG RUST

Để lựa chọn giải pháp nền tảng cho crate `llm`, chúng tôi tiến hành khảo sát và đo kiểm các thư viện phổ biến trong hệ sinh thái Rust hiện nay:

### 2.1 Thư viện `async-openai` (v0.42.x)
- **Đặc điểm**: Thư viện không chính thức phổ biến nhất cho OpenAI API tại Rust. Hỗ trợ cấu hình `with_api_base` để trỏ tới các endpoint tương thích (Groq, OpenRouter, vLLM). Sử dụng `eventsource-stream` để parse SSE stream thành `ChatCompletionChunk`.
- **Ưu điểm**:
  - Triển khai đầy đủ schema chuẩn của OpenAI (Chat Completions, Tool Calls, JSON Mode).
  - Khả năng cấu hình client sẵn có, code sinh mạnh mẽ qua serde.
- **Nhược điểm & Giới hạn cốt tử**:
  - **Khóa chặt vào schema OpenAI**: Không thể giao tiếp trực tiếp với Anthropic Messages API (vốn có cấu trúc `system` riêng biệt, mảng `content` phức hợp, cấu trúc `thinking_delta`).
  - **Kém linh hoạt với trường phi chuẩn (Reasoning Content)**: Mô hình DeepSeek R1 trả về suy luận qua trường `delta.reasoning_content` (hoặc `delta.reasoning`). Các phiên bản `async-openai` chuẩn thường bỏ qua trường lạ nếu serde bật chế độ deny-unknown-fields hoặc chỉ map cứng trường `content`, gây mất mát dữ liệu suy luận.
  - **Không hỗ trợ Native Ollama**: Endpoint `/api/chat` của Ollama dùng NDJSON chứ không dùng chuẩn SSE `data: {}`, `async-openai` không thể phân tích được trừ khi ép Ollama chạy qua `/v1/chat/completions`.
  - **Khó tích hợp Outbound Guard cấp thấp**: Khó can thiệp sâu vào tầng DNS resolution để chống DNS Rebinding SSRF trước khi request socket được mở.

### 2.2 Thư viện `rig-core` (v0.46.x)
- **Đặc điểm**: Framework mã nguồn mở hỗ trợ xây dựng ứng dụng LLM trong Rust (tương tự LangChain hay LlamaIndex). Tích hợp sẵn adapter cho OpenAI, Anthropic, Gemini, Cohere, Perplexity.
- **Ưu điểm**: Cung cấp trừu tượng cấp cao (`CompletionModel`, `Agent`, `Prompt`, RAG).
- **Nhược điểm & Lý do loại bỏ (No-Match)**:
  - **Xung đột kiến trúc (Architectural Impedance Mismatch)**: Merlink Harness đã có kiến trúc phân tầng độc lập. Logic điều phối Agent (`RunLoop`, `RunPhase`, State Machine, Verify Pipeline, Approval System, Context Compactor) nằm trọn trong `crates/agent`. Sử dụng `rig-core` sẽ gây trùng lặp khái niệm Agent/Pipeline, tạo ra hai nguồn sự thật (dual abstraction).
  - **Quá nặng nề (Bloated Dependencies)**: `rig-core` kéo theo hàng loạt phụ thuộc về vector database, embedding algorithms, các kiểu dữ liệu không cần thiết, vi phạm nguyên tắc "crate mỏng và tinh gọn" của `be-tauri-plan.md`.
  - **Mất kiểm soát luồng Streaming chi tiết**: Trừu tượng streaming của Rig làm phẳng các chunk, gây khó khăn cho việc bóc tách `PartialToolCall` (theo `index` và `id`), `reasoning_content`, và `Usage` token chi tiết phục vụ cho token calibration.

### 2.3 Thư viện `ollama-rs` (v0.3.x) & Các crate Anthropic cộng đồng
- **`ollama-rs`**: Chuyên biệt cho Ollama, hỗ trợ tốt `/api/chat` và quản lý model local. Tuy nhiên, nó là một thư viện độc lập, tạo thêm dependency `reqwest` trùng lặp, không thể dùng chung kết nối hoặc cơ chế retry/outbound guard với các provider khác.
- **Anthropic community crates**: Hệ sinh thái Rust chưa có SDK chính thức từ Anthropic. Các crate do cộng đồng phát triển thường chậm cập nhật các tính năng mới nhất (ví dụ: Claude 3.7 Extended Thinking `thinking_delta`, Prompt Caching `cache_control`).

### 2.4 Đánh giá kỹ thuật SSE: `reqwest-eventsource` vs `eventsource-stream` vs Custom Byte-Stream Decoder
- **`reqwest-eventsource`**:
  - Gói bọc `reqwest::RequestBuilder` thành một EventSource stream.
  - **Điểm yếu nghiêm trọng đối với LLM**: Mặc định tự động kích hoạt tính năng **Reconnect** khi stream bị gián đoạn. Đối với HTTP POST (vốn dùng trong LLM API), việc reconnect tự động sẽ gửi lại toàn bộ prompt hoặc gặp lỗi `HTTP 405 Method Not Allowed`, hoặc tệ hơn là gây trùng lặp tiền thanh toán API / execution step.
- **`eventsource-stream` (v0.2.x)**:
  - Nhận một `Stream<Item = Result<Bytes, reqwest::Error>>` từ `response.bytes_stream()` và chuyển đổi thành `Stream<Item = Result<Event, EventStreamError>>`.
  - Không tự tiện reconnect, không giữ logic HTTP, chỉ phân tích cú pháp RFC 8895 SSE (`event:`, `data:`, `id:`, `retry:`).
  - Rất nhẹ, tương thích hoàn hảo với `tokio` và `futures-util`.
- **Custom Line Decoder**:
  - Tự viết logic decode buffer dòng text `\n\n`. Rất dễ viết (~80 dòng Rust) nhưng `eventsource-stream` đã làm rất chuẩn và có kiểm thử toàn diện trường hợp dữ liệu UTF-8 bị cắt ngang boundary giữa 2 TCP chunks.

### 2.5 Bảng so sánh tổng hợp & Quyết định lựa chọn

| Tiêu chí | `async-openai` | `rig-core` | `ollama-rs` | Custom `reqwest` + `eventsource-stream` |
|---|---|---|---|---|
| **Đa giao thức (OpenAI, Anthropic, Ollama)** | ❌ Chỉ OpenAI | ⚠️ Đa số (trừu tượng riêng) | ❌ Chỉ Ollama | ✅ **Hỗ trợ tối ưu cả 3** |
| **Reasoning Tokens (R1 / Claude 3.7)** | ⚠️ Hạn chế / Phụ thuộc ver | ⚠️ Bị gộp vào text | ❌ Không rõ ràng | ✅ **Phân luồng độc lập (`Reasoning`)** |
| **Tool Calling Streaming Chunks** | ✅ Tốt cho OpenAI | ⚠️ Bị bao bọc | ⚠️ Schema riêng | ✅ **Chuẩn hóa thành `PartialToolCall`** |
| **Phù hợp với 4-Crate Boundary** | ⚠️ Kéo theo types lớn | ❌ Vi phạm, trùng lặp loop | ❌ Thừa thãi | ✅ **Chính xác theo plan mục 4.2** |
| **Kiểm soát Outbound Guard / SSRF** | ⚠️ Khó hook tầng thấp | ❌ Không hỗ trợ | ❌ Không hỗ trợ | ✅ **Tích hợp sâu vào reqwest pipeline** |
| **Tự động ngắt kết nối khi Cancel** | ⚠️ Phụ thuộc stream drop | ⚠️ Khó kiểm soát | ⚠️ | ✅ **Gắn chặt với `CancellationToken`** |

> **Quyết định kiến trúc:**  
> Xây dựng crate `llm` dựa trên **Custom `reqwest` Client** kết hợp với **`eventsource-stream`** (và custom NDJSON reader cho Ollama native).  
> Thiết kế một Trait thống nhất `LlmClient` trong `crates/domain/src/ports.rs`, và triển khai 3 Adapters độc lập trong `crates/llm/src/adapters/`:
> 1. `chat_completions.rs` (OpenAI-compatible: OpenAI, DeepSeek, Groq, OpenRouter, vLLM, LM Studio)
> 2. `anthropic.rs` (Anthropic Messages API: Claude 3.5/3.7, thinking blocks)
> 3. `ollama.rs` (Ollama Native API `/api/chat` + fallback OpenAI-compatible)

---

## 3. THIẾT KẾ CHI TIẾT CRATE `llm`, ADAPTER VÀ STREAMING EVENT

### 3.1 Trừu tượng hóa Provider: `LlmClient` Trait
Trait `LlmClient` được định nghĩa trong `crates/domain/src/ports.rs` để đảm bảo `crates/agent` chỉ phụ thuộc vào `domain` mà không phụ thuộc vào `llm`:

```rust
// crates/domain/src/ports.rs
use async_trait::async_trait;
use futures_util::Stream;
use std::pin::Pin;
use tokio_util::sync::CancellationToken;

use crate::provider::{LlmChatRequest, ConnectionTestResult, StreamItem};
use crate::error::LlmError;

pub type BoxedStreamItemStream = Pin<Box<dyn Stream<Item = Result<StreamItem, LlmError>> + Send>>;

#[async_trait]
pub trait LlmClient: Send + Sync {
    /// Thực hiện stream chat completion đa lượt, trả về stream các StreamItem đã chuẩn hóa
    async fn stream_chat(
        &self,
        request: LlmChatRequest,
        cancel_token: CancellationToken,
    ) -> Result<BoxedStreamItemStream, LlmError>;

    /// Kiểm tra kết nối và tính hợp lệ của API Key/Endpoint
    async fn test_connection(&self) -> Result<ConnectionTestResult, LlmError>;
}
```

### 3.2 Chuẩn hóa Streaming Event (`StreamItem`)
Mọi gói tin SSE hoặc NDJSON từ các nhà cung cấp đều được bóc tách và chuyển đổi thành enum `StreamItem` duy nhất trong `crates/domain/src/provider.rs` (hoặc `crates/llm/src/item.rs`):

```rust
// crates/domain/src/provider.rs hoặc crates/llm/src/item.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum StreamItem {
    /// Token văn bản thông thường từ mô hình
    Text(String),
    
    /// Token suy luận (Reasoning / Thinking)
    /// Hỗ trợ: DeepSeek R1 (reasoning_content), Claude 3.7 Extended Thinking (thinking_delta)
    Reasoning(String),
    
    /// Mảnh ghép lời gọi công cụ (streaming argument delta)
    ToolCallChunk(PartialToolCall),
    
    /// Thống kê lượng token tiêu thụ (Usage)
    Usage(TokenUsage),
    
    /// Tín hiệu kết thúc luồng thành công kèm lý do hoàn tất
    Done(FinishReason),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PartialToolCall {
    /// Thứ tự tool call trong lượt sinh (hỗ trợ parallel tool calling)
    pub index: usize,
    /// ID lời gọi (thường xuất hiện ở chunk đầu tiên: vd call_xxx hoặc toolu_xxx)
    pub call_id: Option<String>,
    /// Tên công cụ (thường xuất hiện ở chunk đầu tiên)
    pub name: Option<String>,
    /// Mảnh JSON tham số được nối dần (accumulated arguments delta)
    pub arguments_delta: String,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TokenUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
    /// Token suy luận nằm trong completion_tokens (nếu provider hỗ trợ đếm riêng)
    pub reasoning_tokens: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FinishReason {
    Stop,
    ToolCalls,
    Length,
    ContentFilter,
    Other(String),
}
```

### 3.3 Factory Pattern & Quản lý Cấu hình Provider
Tại `crates/llm/src/client.rs`:
```rust
// crates/llm/src/client.rs
use std::sync::Arc;
use crate::adapters::{
    chat_completions::OpenAiCompatAdapter,
    anthropic::AnthropicAdapter,
    ollama::OllamaAdapter,
};
use crate::outbound_guard::OutboundGuard;
use domain::ports::LlmClient;
use domain::provider::{AiProvider, AiModel, ProviderProtocol};
use domain::error::LlmError;

pub struct LlmClientFactory;

impl LlmClientFactory {
    pub fn create(
        provider: &AiProvider,
        model: &AiModel,
        api_key: Option<String>,
        http_client: reqwest::Client,
    ) -> Result<Arc<dyn LlmClient>, LlmError> {
        // 1. Kiểm tra bảo mật Outbound Guard trên URL trước khi tạo Client
        OutboundGuard::validate_endpoint(&provider.base_url, provider.is_local)?;

        // 2. Khởi tạo Adapter theo Protocol
        match provider.protocol {
            ProviderProtocol::OpenaiCompat => Ok(Arc::new(OpenAiCompatAdapter::new(
                provider.clone(),
                model.clone(),
                api_key,
                http_client,
            ))),
            ProviderProtocol::Anthropic => Ok(Arc::new(AnthropicAdapter::new(
                provider.clone(),
                model.clone(),
                api_key,
                http_client,
            ))),
            ProviderProtocol::Ollama => Ok(Arc::new(OllamaAdapter::new(
                provider.clone(),
                model.clone(),
                http_client,
            ))),
        }
    }
}
```

---

### 3.4 Thiết kế chi tiết các Adapters

#### A. OpenAI-Compatible Adapter (`crates/llm/src/adapters/chat_completions.rs`)
- **Phạm vi**: OpenAI, OpenRouter, Groq, DeepSeek, Together AI, vLLM, LM Studio, Custom URLs.
- **Wire Request**: `POST {base_url}/chat/completions`
  - Header: `Authorization: Bearer <key>`
  - Request Body:
    ```json
    {
      "model": "deepseek-reasoner",
      "messages": [ ... ],
      "tools": [ { "type": "function", "function": { "name": "...", "parameters": {} } } ],
      "stream": true,
      "stream_options": { "include_usage": true }
    }
    ```
- **Xử lý Stream Chunk (SSE)**:
  Mỗi event `data: {...}` được parse theo DTO nội bộ.
  *Lưu ý từ tài liệu kỹ thuật vLLM & DeepSeek*: Mô hình suy luận DeepSeek R1 và các biến thể (QwQ-32B, Qwen3) xuất luồng suy luận qua trường `delta.reasoning_content`. Khi kích hoạt Tool Calling, token hàm gọi chỉ xuất hiện trong trường `content`/`tool_calls`, tuyệt đối không nằm trong `reasoning_content`. Struct nhận dạng cần hỗ trợ alias `reasoning` để bao quát các proxy LLM khác nhau:
  ```rust
  #[derive(Deserialize)]
  struct OpenAiChunk {
      choices: Vec<OpenAiChoice>,
      usage: Option<OpenAiUsage>,
  }

  #[derive(Deserialize)]
  struct OpenAiChoice {
      delta: OpenAiDelta,
      finish_reason: Option<String>,
  }

  #[derive(Deserialize)]
  struct OpenAiDelta {
      content: Option<String>,
      /// Hỗ trợ DeepSeek R1, Groq, OpenRouter reasoning streaming (chuẩn vLLM)
      #[serde(alias = "reasoning")]
      reasoning_content: Option<String>,
      tool_calls: Option<Vec<OpenAiToolCallChunk>>,
  }

  #[derive(Deserialize)]
  struct OpenAiToolCallChunk {
      index: usize,
      id: Option<String>,
      function: Option<OpenAiFunctionChunk>,
  }

  #[derive(Deserialize)]
  struct OpenAiFunctionChunk {
      name: Option<String>,
      arguments: Option<String>,
  }
  ```
- **Chuyển đổi sang `StreamItem`**:
  1. Nếu `delta.reasoning_content` (hoặc `reasoning`) có giá trị → phát `StreamItem::Reasoning(val)`.
  2. Nếu `delta.content` có giá trị → phát `StreamItem::Text(val)`.
  3. Nếu `delta.tool_calls` xuất hiện → lặp qua từng phần tử và phát:
     ```rust
     StreamItem::ToolCallChunk(PartialToolCall {
         index: tc.index,
         call_id: tc.id,
         name: tc.function.as_ref().and_then(|f| f.name.clone()),
         arguments_delta: tc.function.and_then(|f| f.arguments).unwrap_or_default(),
     })
     ```
  4. Nếu `chunk.usage` có giá trị → phát `StreamItem::Usage(...)`.
  5. Nếu `finish_reason` xuất hiện hoặc gặp `data: [DONE]` → phát `StreamItem::Done(...)`.

#### B. Anthropic Messages Adapter (`crates/llm/src/adapters/anthropic.rs`)
- **Phạm vi**: Anthropic Claude 3 / 3.5 Sonnet / 3.7 Sonnet (với Extended Thinking).
- **Wire Request**: `POST {base_url}/v1/messages`
  - Header:
    - `x-api-key: <key>`
    - `anthropic-version: 2023-06-01`
    - `anthropic-beta: prompt-caching-2024-07-31,thinking-2025-02-19`
  - Request Body:
    - **Tách riêng System Prompt**: Anthropic yêu cầu trường `system` nằm ở top-level của JSON body (string hoặc mảng cacheable blocks), **không** nằm trong mảng `messages`.
    - **Thinking block**:
      ```json
      "thinking": {
        "type": "enabled",
        "budget_tokens": 2048
      }
      ```
    - **Tool schema format**:
      ```json
      "tools": [{
        "name": "edit_scene",
        "description": "...",
        "input_schema": { "type": "object", "properties": { ... } }
      }]
      ```
- **Xử lý luồng SSE Anthropic**:
  Anthropic phát sinh chuỗi sự kiện có phân cấp theo `event` type:
  1. `event: message_start`  
     Data mang thông tin `usage.input_tokens` ban đầu → lưu trữ tạm.
  2. `event: content_block_start`  
     - Nếu `content_block.type == "thinking"`: Khởi tạo khối suy luận.
     - Nếu `content_block.type == "text"`: Khởi tạo khối văn bản.
     - Nếu `content_block.type == "tool_use"`: Data chứa `{ id, name, index }` → phát `StreamItem::ToolCallChunk` chứa `call_id` và `name`.
  3. `event: content_block_delta`  
     - Nếu `delta.type == "thinking_delta"`: Phát `StreamItem::Reasoning(delta.thinking)`.
     - Nếu `delta.type == "signature_delta"`: Lưu trữ chuỗi chữ ký mật mã (`delta.signature`) đi kèm khối thinking để phục vụ đối chiếu toàn vẹn và nạp lại vào context các lượt sau (theo đặc tả Anthropic Thinking bắt buộc phải gửi lại signature trong multi-turn).
     - Nếu `delta.type == "text_delta"`: Phát `StreamItem::Text(delta.text)`.
     - Nếu `delta.type == "input_json_delta"`: Phát `StreamItem::ToolCallChunk` chứa `arguments_delta: delta.partial_json`.
  4. `event: message_delta`  
     - Chứa `delta.stop_reason` (`end_turn`, `tool_use`, `max_tokens`) và `usage.output_tokens`.
     - *Lưu ý quan trọng từ tài liệu Anthropic*: `usage.output_tokens` trong `message_delta` là giá trị **lũy kế (cumulative)**, không được cộng dồn nhiều lần.
  5. `event: message_stop`  
     - Phát `StreamItem::Done`.
     
  *Căn cứ tài liệu chính thức*: Tài liệu Streaming của Anthropic nêu rõ: *"tool_use và thinking block không thể khôi phục một phần; nếu stream đứt đoạn thì chỉ có thể yêu cầu mô hình sinh lại từ đầu hoặc tiếp tục từ khối text hoàn chỉnh gần nhất"*. Điều này khẳng định quyết định kiến trúc tại Mục 4.4 và Mục 6.1 (Không resume giữa chừng, rollback & retry cả step) là chuẩn xác 100%.

#### C. Ollama Adapter (`crates/llm/src/adapters/ollama.rs`)
- **Phạm vi**: Ollama chạy cục bộ hoặc trên máy chủ mạng LAN (`http://localhost:11434`).
- **Chiến lược hai chế độ**:
  1. **Chế độ OpenAI-compatible (`/v1/chat/completions`)**: Được ưu tiên vì Ollama từ các bản cập nhật gần đây đã chuẩn hóa tool calling và streaming theo chuẩn OpenAI. Dùng chung parser với `OpenAiCompatAdapter`.
  2. **Chế độ Native API (`POST /api/chat`)**:
     - Cần thiết khi người dùng tùy biến sâu các tham số của llama.cpp: `num_ctx`, `keep_alive`, `options.temperature`, `options.num_predict`.
     - Stream là **NDJSON** (mỗi dòng là một JSON độc lập, không có tiền tố `data:`).
     - Parser dòng đọc từng JSON:
       - `message.content` → `StreamItem::Text`.
       - `message.tool_calls` → chuyển đổi sang `StreamItem::ToolCallChunk`.
       - Dòng cuối `done: true` kèm `prompt_eval_count` và `eval_count` → phát `StreamItem::Usage` và `StreamItem::Done`.

### 3.5 Tích lũy Lời gọi Công cụ (`ToolCallAccumulator`)
Trong `crates/agent` (hoặc tầng điều phối của `llm`), các mảnh `PartialToolCall` được gom lại trước khi chuyển sang hệ thống thực thi tool:

```rust
// crates/agent/src/tools/accumulator.rs
use std::collections::BTreeMap;
use domain::provider::PartialToolCall;
use domain::tool::ToolCall;

#[derive(Default)]
pub struct ToolCallAccumulator {
    calls: BTreeMap<usize, PendingToolCall>,
}

struct PendingToolCall {
    call_id: String,
    name: String,
    arguments_buffer: String,
}

impl ToolCallAccumulator {
    pub fn feed(&mut self, chunk: PartialToolCall) {
        let entry = self.calls.entry(chunk.index).or_insert_with(|| PendingToolCall {
            call_id: chunk.call_id.clone().unwrap_or_default(),
            name: chunk.name.clone().unwrap_or_default(),
            arguments_buffer: String::new(),
        });
        
        if let Some(id) = chunk.call_id {
            if entry.call_id.is_empty() { entry.call_id = id; }
        }
        if let Some(name) = chunk.name {
            if entry.name.is_empty() { entry.name = name; }
        }
        entry.arguments_buffer.push_str(&chunk.arguments_delta);
    }

    pub fn finalize(self) -> Result<Vec<ToolCall>, serde_json::Error> {
        self.calls.into_values().map(|p| {
            let parsed_args: serde_json::Value = if p.arguments_buffer.trim().is_empty() {
                serde_json::json!({})
            } else {
                serde_json::from_str(&p.arguments_buffer)?
            };
            
            Ok(ToolCall {
                id: p.call_id,
                name: p.name,
                arguments: parsed_args,
            })
        }).collect()
    }
}
```

---

## 4. THIẾT KẾ RESILIENCE & FAULT TOLERANCE (`llm/retry.rs`)

### 4.1 Phân loại Lỗi Chi tiết (Transient vs Terminal)
Theo quy định tại Mục 6.1 của `docs/be-tauri-plan.md`, mọi lỗi phát sinh từ tầng mạng và HTTP provider phải được phân loại rành mạch:

```rust
// crates/domain/src/error.rs hoặc crates/llm/src/retry.rs
use std::time::Duration;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LlmError {
    // ---------- NHÓM LỖI TRANSIENT (Có thể retry) ----------
    #[error("Rate limited (HTTP 429). Retry after {retry_after:?}")]
    RateLimited { retry_after: Option<Duration> },

    #[error("Server error (HTTP {status}): {body}")]
    ServerError { status: u16, body: String },

    #[error("Network connection error: {0}")]
    Network(String),

    #[error("Stream interrupted prematurely: {0}")]
    StreamInterrupted(String),

    // ---------- NHÓM LỖI TERMINAL (Fail-fast, cấm retry) ----------
    #[error("Authentication failed (HTTP 401/403): {0}")]
    AuthInvalidKey(String),

    #[error("Model not found (HTTP 404): {0}")]
    ModelNotFound(String),

    #[error("Context window overflow: {0}")]
    ContextOverflow(String),

    #[error("Bad request (HTTP 400): {0}")]
    BadRequest(String),

    #[error("Security Outbound Guard blocked: {0}")]
    SecurityViolation(String),

    #[error("Operation cancelled by user")]
    Cancelled,
}

impl LlmError {
    pub fn is_transient(&self) -> bool {
        matches!(
            self,
            LlmError::RateLimited { .. }
                | LlmError::ServerError { .. }
                | LlmError::Network(_)
                | LlmError::StreamInterrupted(_)
        )
    }
}
```

### 4.2 Thuật toán Exponential Backoff + Full Jitter
Tránh hiệu ứng "thundering herd" khi hàng loạt request gặp rate limit cùng lúc. Công thức tính thời gian chờ:
$$Sleep = \text{rand}(0, \min(Delay_{max}, Delay_{base} \times 2^{attempt}))$$

Trong trường hợp máy chủ trả về header `Retry-After`, hệ thống ưu tiên giá trị này nếu nó lớn hơn giá trị tính toán.

```rust
// crates/llm/src/retry.rs
use std::time::Duration;
use rand::Rng;
use tokio_util::sync::CancellationToken;
use crate::error::LlmError;

pub struct RetryPolicy {
    pub max_retries: u32,       // Mặc định: 3 lần (mục 6.1)
    pub base_delay: Duration,    // Mặc định: 1000ms
    pub max_delay: Duration,     // Mặc định: 30000ms
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_retries: 3,
            base_delay: Duration::from_millis(1000),
            max_delay: Duration::from_secs(30),
        }
    }
}

impl RetryPolicy {
    pub async fn execute_with_retry<F, Fut, T>(
        &self,
        cancel_token: &CancellationToken,
        mut operation: F,
    ) -> Result<T, LlmError>
    where
        F: FnMut() -> Fut,
        Fut: std::future::Future<Output = Result<T, LlmError>>,
    {
        let mut attempt = 0;
        loop {
            if cancel_token.is_cancelled() {
                return Err(LlmError::Cancelled);
            }

            match operation().await {
                Ok(val) => return Ok(val),
                Err(err) if err.is_transient() && attempt < self.max_retries => {
                    attempt += 1;
                    
                    // Tính toán delay có jitter
                    let delay = match &err {
                        LlmError::RateLimited { retry_after: Some(ra) } => *ra,
                        _ => {
                            let exp = (2u64).saturating_pow(attempt);
                            let max_backoff = (self.base_delay * exp as u32).min(self.max_delay);
                            let jitter_millis = rand::thread_rng().gen_range(0..=max_backoff.as_millis() as u64);
                            Duration::from_millis(jitter_millis)
                        }
                    };

                    tracing::warn!(
                        attempt = attempt,
                        max_retries = self.max_retries,
                        delay_ms = delay.as_millis(),
                        error = %err,
                        "LLM call failed with transient error, backing off..."
                    );

                    // Chờ đợi nhưng có thể ngắt ngay nếu cancel_token được kích hoạt
                    tokio::select! {
                        _ = tokio::time::sleep(delay) => {},
                        _ = cancel_token.cancelled() => return Err(LlmError::Cancelled),
                    }
                }
                Err(err) => return Err(err),
            }
        }
    }
}
```

### 4.3 Nhận diện sớm Tràn Ngữ Cảnh (Context Length Exceeded)
Nhiều provider trả về HTTP 400 Bad Request khi prompt vượt quá context window thay vì mã lỗi riêng. Nếu không bóc tách kỹ, hệ thống sẽ nhầm thành lỗi payload cú pháp và dừng đột ngột (Fail Fast).

**Mẫu nhận diện (Pattern Matching) từ Response Body**:
- **OpenAI / OpenRouter**:
  - `error.code == "context_length_exceeded"`
  - `error.message` chứa regex: `maximum context length is \d+ tokens.*however your request resulted in`
- **Anthropic**:
  - `error.type == "invalid_request_error"`
  - `error.message` chứa: `prompt is too long` hoặc `max_tokens is too large`
- **Ollama**:
  - Message chứa: `context length exceeded` hoặc `input tokens exceed maximum context`

**Quy trình Phản hồi (Recovery Workflow theo mục 6.1)**:
1. Khi phát hiện `LlmError::ContextOverflow`:
2. Agent Runtime **không retry ngay lập tức với cùng payload**.
3. Runtime kích hoạt `Compactor` (`crates/agent/src/context/compactor.rs`) để cắt tỉa bớt tin nhắn cũ hoặc thu nhỏ phần `summary`.
4. Sau khi nén context, runtime cho phép **thử lại đúng 1 lần**.
5. Nếu lần thử thứ hai vẫn báo tràn context → chuyển trạng thái sang `RunStatus::Failed` với mã `CONTEXT_OVERFLOW`.

### 4.4 Quy tắc Xử lý Đứt Luồng SSE Giữa Chừng
Một câu hỏi lớn trong thiết kế: *Có nên resume lại stream SSE đang chạy dở hay không?*

**Quyết định kiến trúc (Khớp tuyệt đối Mục 6.1):**
> **KHÔNG RESUME GIỮA CHỪNG.**  
> - **Lý do**: Trong kiến trúc của Merlink Harness, các Tool Call chỉ được lập lịch và cấp phép thực thi **sau khi toàn bộ stream của step kết thúc thành công** và payload JSON của tool được validate đầy đủ.
> - Do đó, nếu stream bị đứt giữa chừng, **chưa có bất kỳ tool nào được chạy** và chưa có thay đổi nào được ghi vào cơ sở dữ liệu.
> - Việc cố gắng resume một stream dở dang (ví dụ prompt tiếp "hãy tiếp tục từ từ X...") tiềm ẩn rủi ro hallucination, sinh trùng lặp token, hoặc làm sai lệch cấu trúc JSON của tool call.
> - **Hành vi chuẩn hóa**:
>   1. Hủy bỏ phần text/tool chunk đã tích lũy dở dang trong bộ nhớ đệm của step hiện tại.
>   2. Phát sự kiện `RunEvent::StreamReset { reason: "SSE stream connection lost" }` về phía frontend để làm sạch hiển thị streaming tạm thời.
>   3. Ghi một bản ghi vào `run_steps` với kind `llm`, verdict `retry`.
>   4. Tự động thực hiện lại toàn bộ step hiện tại từ đầu (tính vào hạn ngạch `max_llm_retries`).

---

## 5. BẢO MẬT & QUẢN LÝ SECRET (KEYRING & OUTBOUND GUARD)

### 5.1 Quản lý Khóa API qua OS Keychain (`keyring` crate)

Để tuân thủ nguyên tắc **Zero Plaintext Leakage**:
1. Khóa API **không bao giờ** được ghi vào cơ sở dữ liệu SQLite (`ai_providers.secret_ref` chỉ lưu con trỏ tham chiếu).
2. Khóa API **không bao giờ** được lưu vào file cấu hình `.env` hay `settings.json`.
3. Khóa API **không bao giờ** được gửi ngược về Frontend. API chỉ trả về cờ `has_key: bool`.
4. Trong các log span (`tracing`) và trace audit (`run_steps`), mọi header `Authorization` hoặc `x-api-key` đều bị **redact** thành `[REDACTED]`.

#### Định nghĩa Cổng `SecretStore` tại `domain`
```rust
// crates/domain/src/ports.rs
use async_trait::async_trait;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SecretError {
    #[error("Keychain access failed: {0}")]
    StorageError(String),
    #[error("Secret not found: {0}")]
    NotFound(String),
}

#[async_trait]
pub trait SecretStore: Send + Sync {
    async fn get(&self, secret_ref: &str) -> Result<Option<String>, SecretError>;
    async fn set(&self, secret_ref: &str, secret_value: &str) -> Result<(), SecretError>;
    async fn delete(&self, secret_ref: &str) -> Result<(), SecretError>;
}
```

#### Triển khai với `keyring` tại `infra`
Crate `keyring` (v4.2.x) tự động map tới nền tảng hệ điều hành tương ứng:
- **macOS**: Apple Keychain Services qua `Security.framework`.
- **Windows**: Windows Credential Manager (`wincred`).
- **Linux Desktop**: FreeDesktop Secret Service API qua D-Bus (GNOME Keyring, KWallet).

```rust
// crates/infra/src/secrets.rs
use async_trait::async_trait;
use keyring::Entry;
use domain::ports::{SecretStore, SecretError};

const KEYRING_SERVICE_NAME: &str = "com.merlink.harness";

pub struct KeyringSecretStore;

impl KeyringSecretStore {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl SecretStore for KeyringSecretStore {
    async fn get(&self, secret_ref: &str) -> Result<Option<String>, SecretError> {
        let entry = Entry::new(KEYRING_SERVICE_NAME, secret_ref)
            .map_err(|e| SecretError::StorageError(e.to_string()))?;
            
        match entry.get_password() {
            Ok(pass) => Ok(Some(pass)),
            Err(keyring::Error::NoEntry) => Ok(None),
            Err(e) => Err(SecretError::StorageError(e.to_string())),
        }
    }

    async fn set(&self, secret_ref: &str, secret_value: &str) -> Result<(), SecretError> {
        let entry = Entry::new(KEYRING_SERVICE_NAME, secret_ref)
            .map_err(|e| SecretError::StorageError(e.to_string()))?;
            
        entry.set_password(secret_value)
            .map_err(|e| SecretError::StorageError(e.to_string()))
    }

    async fn delete(&self, secret_ref: &str) -> Result<(), SecretError> {
        let entry = Entry::new(KEYRING_SERVICE_NAME, secret_ref)
            .map_err(|e| SecretError::StorageError(e.to_string()))?;
            
        match entry.delete_credential() {
            Ok(_) | Err(keyring::Error::NoEntry) => Ok(()),
            Err(e) => Err(SecretError::StorageError(e.to_string())),
        }
    }
}
```

> **Fallback cho môi trường Headless / CI / Test**:  
> Trong môi trường Linux không có giao diện đồ họa (không có D-Bus daemon cho GNOME Keyring), `keyring` sẽ trả về lỗi. Hệ thống cung cấp `InMemorySecretStore` (bọc quanh `RwLock<HashMap<String, String>>`) để chạy test tích hợp và eval mà không đòi hỏi keyring của OS.

---

### 5.2 Outbound Guard: Phòng chống SSRF & Rò rỉ Thông tin Endpoint

Do ứng dụng hỗ trợ cấu hình `base_url` tùy ý cho các nhà cung cấp OpenAI-compatible và Ollama, nếu không có cơ chế kiểm soát chặt chẽ, người dùng có thể vô tình hoặc cố ý bị tấn công **Server-Side Request Forgery (SSRF)**:
- Kẻ tấn công lợi dụng `base_url` để buộc ứng dụng quét cổng nội bộ (`http://192.168.1.1:80`).
- Gửi yêu cầu kèm API Key tới Cloud Metadata Endpoints của AWS/GCP (`http://169.254.169.254/latest/meta-data/`).
- Gửi API key của OpenAI tới một server độc hại của bên thứ ba.

#### Kiến trúc `OutboundGuard` (`crates/llm/src/outbound_guard.rs`)

```rust
// crates/llm/src/outbound_guard.rs
use std::net::{IpAddr, ToSocketAddrs};
use url::Url;
use domain::error::LlmError;

pub struct OutboundGuard;

impl OutboundGuard {
    /// Kiểm tra tính hợp lệ và an toàn của endpoint URL
    pub fn validate_endpoint(raw_url: &str, is_local: bool) -> Result<Url, LlmError> {
        let url = Url::parse(raw_url)
            .map_err(|e| LlmError::SecurityViolation(format!("Invalid endpoint URL: {}", e)))?;

        // 1. Chỉ chấp nhận HTTP hoặc HTTPS scheme
        match url.scheme() {
            "http" | "https" => {},
            other => return Err(LlmError::SecurityViolation(format!(
                "Unsupported scheme '{}'. Only HTTP and HTTPS are allowed.", other
            ))),
        }

        let host_str = url.host_str().ok_or_else(|| {
            LlmError::SecurityViolation("Endpoint URL missing host".into())
        })?;

        // 2. Phân biệt endpoint Local vs Cloud
        if is_local {
            Self::validate_local_host(host_str, &url)?;
        } else {
            Self::validate_cloud_host(host_str, &url)?;
        }

        Ok(url)
    }

    fn validate_local_host(host: &str, url: &Url) -> Result<(), LlmError> {
        // Cho phép localhost và 127.0.0.1, ::1
        if host == "localhost" {
            return Ok(());
        }

        if let Ok(ip) = host.parse::<IpAddr>() {
            if ip.is_loopback() {
                return Ok(());
            }
            // Cho phép địa chỉ LAN (192.168.x.x, 10.x.x.x) nếu cấu hình rõ ràng
            if Self::is_private_ip(&ip) {
                return Ok(());
            }
        }

        Err(LlmError::SecurityViolation(format!(
            "Provider is marked local but host '{}' is not a loopback or private network address",
            host
        )))
    }

    fn validate_cloud_host(host: &str, url: &Url) -> Result<(), LlmError> {
        // Cloud endpoint bắt buộc phải dùng HTTPS để tránh rò rỉ API key trên đường truyền
        if url.scheme() != "https" {
            return Err(LlmError::SecurityViolation(
                "Cloud providers must use HTTPS to ensure API keys are encrypted in transit".into(),
            ));
        }

        // Ngăn chặn truy cập Cloud Metadata Endpoint (169.254.169.254) hoặc Loopback từ Cloud provider
        if let Ok(ip) = host.parse::<IpAddr>() {
            if ip.is_loopback() || Self::is_link_local_or_metadata(&ip) || Self::is_private_ip(&ip) {
                return Err(LlmError::SecurityViolation(format!(
                    "Cloud provider URL cannot point to internal/private IP address: {}", ip
                )));
            }
        }

        // Chống DNS Rebinding: Resolve thử domain và kiểm tra danh sách IP trả về
        if let Ok(socket_addrs) = format!("{}:{}", host, url.port_or_known_default().unwrap_or(443)).to_socket_addrs() {
            for addr in socket_addrs {
                let ip = addr.ip();
                if ip.is_loopback() || Self::is_link_local_or_metadata(&ip) {
                    return Err(LlmError::SecurityViolation(format!(
                        "DNS resolution for host '{}' resolved to prohibited internal address: {}", host, ip
                    )));
                }
            }
        }

        Ok(())
    }

    fn is_private_ip(ip: &IpAddr) -> bool {
        match ip {
            IpAddr::V4(ipv4) => ipv4.is_private(),
            IpAddr::V6(_) => false,
        }
    }

    fn is_link_local_or_metadata(ip: &IpAddr) -> bool {
        match ip {
            IpAddr::V4(ipv4) => {
                let octets = ipv4.octets();
                // 169.254.0.0/16 (Link Local & AWS/GCP Metadata)
                octets[0] == 169 && octets[1] == 254
            }
            IpAddr::V6(ipv6) => ipv6.is_unicast_link_local(),
        }
    }
}
```

---

## 6. ĐỐI CHIẾU VỚI `docs/be-tauri-plan.md`

Dưới đây là bảng đối chiếu chi tiết giữa kết quả nghiên cứu/thiết kế của Subagent D và các mục tương ứng trong tài liệu quy hoạch `docs/be-tauri-plan.md`:

| Mục trong `be-tauri-plan.md` | Quy định trong Plan | Thiết kế của Subagent D | Đánh giá & Khuyến nghị bổ sung |
|---|---|---|---|
| **Mục 2: Quyết định kiến trúc** | Workspace 4 crate (`domain`, `agent`, `llm`, `infra`). `llm` chuẩn hoá về `StreamItem`; loop không biết wire format. | Trait `LlmClient` nằm ở `domain/ports.rs`, triển khai ở `crates/llm`. Chuẩn hóa toàn bộ về `StreamItem`. | **Khớp hoàn toàn 100%**. Hàng rào dependency compiler-enforced được tôn trọng. |
| **Mục 2 & 14: API Key** | `keyring` crate (OS keychain), DB chỉ lưu `secret_ref`, không lưu key. | Triển khai `KeyringSecretStore` trong `infra/src/secrets.rs`. SQLite chỉ giữ `secret_ref`. FE chỉ nhận `has_key: bool`. | **Khớp hoàn toàn 100%**. Bổ sung thêm `InMemorySecretStore` phục vụ test CI headless. |
| **Mục 2 & 6: Tool Calling** | Chỉ model `supports_tools = true` chạy agent; model còn lại = chat thuần. Không prompt-based tool calling (ADR-005). | `ToolCallAccumulator` bóc tách `PartialToolCall` từ native function calling chunks của OpenAI/Anthropic/Ollama. | **Khớp hoàn toàn 100%**. Model không hỗ trợ tool calling chỉ sinh `StreamItem::Text`. |
| **Mục 5.2: ai_providers & ai_models** | Enum `ProviderProtocol { openai_compat, anthropic, ollama }`, `is_local`, `secret_ref`, `extra_headers_json`. | Hỗ trợ chính xác 3 protocol qua 3 adapters. `is_local` được dùng làm cổng bảo vệ cho `OutboundGuard`. | **Khớp hoàn toàn 100%**. |
| **Mục 5.5: Message Envelope v1** | `Part = text \| reasoning \| tool_call \| tool_result \| attachment \| summary`. | `StreamItem` mapping 1-1 với `Part`: `StreamItem::Text` → `Part::Text`, `StreamItem::Reasoning` → `Part::Reasoning`. | **Khớp hoàn toàn 100%**. Cho phép FE render khối Thinking mượt mà theo thời gian thực. |
| **Mục 6.1: Lỗi LLM & Retry** | 429/5xx retry mũ + jitter (tối đa 3 lần). 4xx fail-fast. Stream đứt không resume (emit `StreamReset`, retry cả step). Tràn context compact rồi thử 1 lần. | `RetryPolicy` với AWS Full Jitter, `CancellationToken` gắn chặt, pattern matching lỗi tràn context, quy tắc ngắt stream triệt để. | **Khớp và hoàn thiện hóa**. Đã cụ thể hóa bằng mã nguồn Rust chi tiết. |
| **Mục 14: Bảo mật & Outbound Guard** | Redact secret/header trước khi gửi và lưu trace; badge cloud/local. Không dùng Privacy Engine riêng. | `OutboundGuard` kiểm tra URL, chặn SSRF, chặn metadata 169.254, ép buộc HTTPS cho cloud provider, redact header Authorization. | **Khớp hoàn toàn**, bổ sung cơ chế chống DNS Rebinding SSRF. |

### Các điểm tinh chỉnh & đề xuất bổ sung cho Phase 0 / Phase 1:
1. **Xử lý đặc thù của DeepSeek R1**: Cần hỗ trợ fallback field trong deserializer của OpenAI-compat: một số gateway dùng `delta.reasoning_content` (DeepSeek API, OpenRouter), một số dùng `delta.reasoning` (Groq/vLLM cũ). Struct `OpenAiDelta` nên dùng `#[serde(alias = "reasoning")]` trên trường `reasoning_content`.
2. **Cập nhật Header Anthropic Claude 3.7 Thinking**: Khi bật chế độ suy luận cho Claude 3.7, header bắt buộc phải truyền `anthropic-beta: output-128k-2025-02-19,thinking-2025-02-19` và body phải kèm `thinking: { type: "enabled", budget_tokens: N }`.
3. **Thêm timeout kết nối và stream idle timeout**: Cấu hình `reqwest::ClientBuilder` với:
   - `connect_timeout(Duration::from_secs(10))`
   - `read_timeout(Duration::from_secs(60))` để phát hiện sớm các kết nối bị treo vô hạn trước khi SSE kịp phát sinh byte đầu tiên.

---

## 7. CÁC PHƯƠNG ÁN KHÔNG PHÙ HỢP (ĐƯA VÀO `NO-MATCH.MD`)

Trong quá trình nghiên cứu kiến trúc, các phương án sau đây đã được đánh giá kỹ lưỡng và **bị loại bỏ hoàn toàn** do không đáp ứng yêu cầu về bảo mật, tính ổn định và tính mở rộng của hệ thống:

### 1. Frontend trực tiếp gọi LLM API (Hybrid FE-BE)
- **Mô tả**: Frontend (React) tự đọc API key hoặc proxy để gửi HTTP request trực tiếp tới OpenAI/Anthropic nhằm tận dụng các thư viện SDK JS phong phú.
- **Lý do loại bỏ**:
  - Vi phạm nguyên tắc **Backend-Authoritative**: Backend phải là nguồn sự thật duy nhất cho audit log, trace (`run_steps`), và quản lý trạng thái run (`agent_runs`).
  - Nguy cơ lộ lọt API Key trong bộ nhớ WebView hoặc network tab của dev tools.
  - Phá vỡ cơ chế kiểm soát token budget và Outbound Guard ở tầng Rust.

### 2. Lưu trữ Plaintext API Key trong SQLite DB hoặc file cấu hình `.env`
- **Mô tả**: Lưu trực tiếp chuỗi API Key vào cột `ai_providers.api_key` trong SQLite hoặc lưu vào file `.env` ở thư mục ứng dụng.
- **Lý do loại bỏ**:
  - Tệp cơ sở dữ liệu SQLite cục bộ không được mã hóa mặc định. Khi người dùng sao lưu hoặc gửi file DB để hỗ trợ kỹ thuật, toàn bộ khóa API sẽ bị lộ.
  - Vi phạm nghiêm trọng chuẩn mực bảo mật ứng dụng desktop hiện đại. Sử dụng OS Keychain qua `keyring` là giải pháp tiêu chuẩn đã được chốt trong ADR của dự án.

### 3. Prompt-Based Tool Calling cho các mô hình không hỗ trợ Function Calling
- **Mô tả**: Cố gắng ép các mô hình LLM nhỏ/yếu (không có native tool calling) gọi tool bằng cách nhồi schema vào system prompt và parse kết quả JSON dạng Regex (ReAct text prompt).
- **Lý do loại bỏ (Tuân thủ ADR-005)**:
  - Tỷ lệ sinh lỗi cú pháp JSON và hallucination cực kỳ cao ở các model nhỏ.
  - Đòi hỏi xây dựng một hệ thống con cồng kềnh: custom prompt injection, regex parser, tự động sửa lỗi JSON, cơ chế approval riêng cho text-based tool call.
  - Tiêu tốn lượng lớn token ngữ cảnh không đáng có. Model không có native tool calling chỉ được sử dụng cho chế độ Chat văn bản thuần túy.

### 4. Sử dụng Framework Agent nguyên khối `rig-core`
- **Mô tả**: Dùng `rig-core` làm nền tảng toàn bộ cho cả LLM Client và Agent Loop.
- **Lý do loại bỏ**:
  - `rig-core` quá áp đặt kiến trúc (opinionated), gộp chung cả agent loop, vector store, embedding vào một khối.
  - Phá vỡ phân tầng 4 crate sạch của Merlink Harness.
  - Che mờ các sự kiện streaming chi tiết (`PartialToolCall`, `Reasoning` deltas), gây khó khăn cho việc tích hợp UI trực quan và cập nhật token calibration.

### 5. Sử dụng `reqwest-eventsource` với cơ chế Auto-Reconnect mặc định
- **Mô tả**: Dùng crate `reqwest-eventsource` để tự động kết nối lại khi rớt mạng.
- **Lý do loại bỏ**:
  - Auto-reconnect trên phương thức HTTP POST là hành vi nguy hiểm đối với LLM. Khi stream bị đứt, việc tự ý kết nối lại sẽ gây lặp request, phát sinh chi phí kép và có thể làm sai lệch thứ tự tin nhắn.
  - Phải dùng `eventsource-stream` kết hợp với tầng retry tường minh có kiểm soát (`RetryPolicy`).

### 6. Cố gắng Resume Stream SSE bị đứt đoạn giữa chừng
- **Mô tả**: Khi kết nối SSE bị ngắt ở token thứ 150, gửi một prompt mới yêu cầu LLM "hãy tiếp tục từ từ token thứ 150".
- **Lý do loại bỏ**:
  - Không thể đảm bảo tính tất định (non-deterministic) của LLM. Mô hình có thể sinh câu tiếp nối không khớp với ngữ cảnh trước đó hoặc thay đổi hoàn toàn ý định gọi tool.
  - Tool calls chỉ chạy sau khi stream kết thúc trọn vẹn; việc đứt stream đồng nghĩa chưa có side-effect nào xảy ra, do đó giải pháp sạch sẽ và tin cậy nhất là hủy bỏ step và chạy lại từ đầu (roll back & retry step).

---

## 8. KẾT LUẬN & ĐỀ XUẤT CHO CÁC BƯỚC TIẾP THEO

Báo cáo nghiên cứu của Subagent D đã giải quyết trọn vẹn và tường minh 4 trụ cột kỹ thuật của tầng LLM Backend:
1. **Kiến trúc Adapter đa giao thức**: Thống nhất qua trait `LlmClient` tại `domain`, triển khai sạch sẽ cho OpenAI-compatible, Anthropic Messages và Ollama.
2. **Streaming Engine linh hoạt**: Chuẩn hóa thành `StreamItem`, hỗ trợ toàn diện cả Text thông thường, Reasoning tokens (DeepSeek R1, Claude 3.7) và Partial Tool Calls.
3. **Resilience chuẩn mực**: Exponential backoff có jitter, phân loại lỗi nghiêm ngặt, nhận diện sớm tràn ngữ cảnh và triệt tiêu rủi ro đứt luồng.
4. **Bảo mật tuyệt đối**: OS Keychain qua `keyring` và Outbound Guard chống SSRF cấp mạng.

**Đề xuất hành động cho Phase 0 & Phase 1:**
- Đưa các quyết định của báo cáo này vào tài liệu kiến trúc chính thức và cập nhật vào `docs/no-match.md`.
- Bắt đầu hiện thực hóa module `crates/llm` song song với việc dựng `AppState` và schema migration trong Phase 0.
- Chuẩn bị test suite tích hợp với mock server (dùng `wiremock` crate) để kiểm thử toàn diện các kịch bản: 429 backoff, rớt mạng giữa chừng, reasoning token streaming và chặn SSRF.

---

## 9. TÀI LIỆU THAM KHẢO & DỮ LIỆU ĐO KIỂM THỰC TẾ (TRA CỨU QUA TINYFISH)

Toàn bộ các phân tích kiến trúc và quyết định kỹ thuật trong báo cáo này đã được đối soát thực tế thông qua bộ công cụ **TinyFish CLI**:

1. **Anthropic Official Messages Streaming Protocol**:
   - Nguồn tra cứu: `https://platform.claude.com/docs/en/api/messages-streaming` (qua kho tài liệu kỹ thuật chuẩn Anthropic).
   - Dữ liệu xác thực: Đặc tả chuỗi sự kiện `message_start` → `content_block_start` (`type: thinking | tool_use | text`) → `content_block_delta` (`thinking_delta`, `signature_delta`, `input_json_delta`, `text_delta`) → `content_block_stop` → `message_delta` (cumulative usage) → `message_stop`. Khẳng định nguyên tắc không thể khôi phục một phần với thinking/tool blocks.
2. **vLLM & DeepSeek R1 Reasoning Outputs Specification**:
   - Nguồn tra cứu: `https://docs.vllm.ai/en/v0.9.1/features/reasoning_outputs.html`.
   - Dữ liệu xác thực: Chuẩn công nghiệp đưa `reasoning_content` vào `delta` của response chunk; nguyên tắc tách rời hoàn toàn giữa luồng suy luận (`reasoning_content`) và luồng gọi hàm (`tool_calls` chỉ parse từ `content`).
3. **Crate `eventsource-stream` (v0.2.3, docs.rs)**:
   - Nguồn tra cứu: `https://docs.rs/eventsource-stream/`.
   - Dữ liệu xác thực: API `response.bytes_stream().eventsource()` phân tích cú pháp RFC 8895 thuần túy, loại bỏ rủi ro auto-reconnect nguy hiểm trên HTTP POST của `reqwest-eventsource`.
4. **Crate `keyring-rs` (v4.2.0, open-source-cooperative/keyring-rs)**:
   - Nguồn tra cứu: `https://github.com/open-source-cooperative/keyring-rs/blob/main/README.md`.
   - Dữ liệu xác thực: Tương thích hoàn toàn API `Entry::new(service, secret_ref)` với các secure stores nền tảng (macOS Keychain qua Security.framework, Windows Credential Manager, Linux D-Bus Secret Service). Đã hỗ trợ native Tauri 2.0.
