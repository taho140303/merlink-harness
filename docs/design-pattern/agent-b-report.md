# BÁO CÁO KIẾN TRÚC CHUYÊN SÂU: STRUCTURAL DESIGN PATTERNS TRONG RUST
## Tối Ưu Hóa Cho Tauri Agent Harness (`merlink-harness-ui`)

- **Tác giả:** Subagent B (Gemini 3.8 High)
- **Tiếp nối:** Nghiên cứu tổng hợp từ Argon (`tinyfish`, `docs/be-tauri-plan.md`, `docs/db.dbml`)
- **Mục tiêu:** Định hình kiến trúc phần mềm tầng cấu trúc (Structural Patterns), thiết lập ranh giới an toàn, chuẩn hóa giao thức và bảo vệ tính toàn vẹn dữ liệu cho Desktop Agent Desktop Harness.
- **Trạng thái:** Kiến trúc hoàn chỉnh (Architectural Specification & Production Code Examples)

---

## MỤC LỤC
1. [Triết Lý Kiến Trúc: Structural Patterns trong Rust Idiomatic](#1-triết-lý-kiến-trúc-structural-patterns-trong-rust-idiomatic)
2. [Adapter Pattern: Ports & Adapters (Hexagonal Architecture)](#2-adapter-pattern-ports--adapters-hexagonal-architecture)
   - 2.1 Chuẩn hóa Đa Giao thức LLM (`StreamItem`, OpenAI SSE, Anthropic, Ollama)
   - 2.2 Trừu tượng hóa Kho Khóa Bí mật (`SecretStore`)
   - 2.3 Sơ đồ Kiến trúc & Code Mẫu Rust
3. [Decorator Pattern: Composable Middleware Wrapping](#3-decorator-pattern-composable-middleware-wrapping)
   - 3.1 Bọc `LlmClient`: Retry, Jitter, Context Compaction & Tracing
   - 3.2 Bọc `ToolExecutor`: CommitGuard (Transactional Savepoint & Run State Check)
   - 3.3 Sơ đồ Pipeline & Code Mẫu Rust
4. [Proxy Pattern & Newtype Pattern: Capability & Security Enforcement](#4-proxy-pattern--newtype-pattern-capability--security-enforcement)
   - 4.1 `OutboundGuardProxy`: Chống SSRF, DNS Rebinding & Bảo vệ Hạ tầng Mạng
   - 4.2 `ScopedRepoProxy`: Cưỡng chế Phạm vi Dự án (Anti-Confused Deputy Attack)
   - 4.3 `Newtype Pattern`: Type-Safety ở cấp độ Compiler
   - 4.4 Sơ đồ Bảo mật & Code Mẫu Rust
5. [Composite Pattern: Cây Phân Cấp Thực Thể Tự Tham Chiếu](#5-composite-pattern-cây-phân-cấp-thực-thể-tự-tham-chiếu)
   - 5.1 Quản lý Không gian Thế giới (`world_settings`) & Hệ Thống Chủ đề (`themes`)
   - 5.2 Kế thừa Quy tắc, Path Traversal & Phát hiện Vòng lặp (Cycle Detection)
   - 5.3 Sơ đồ Cây Thực Thể & Code Mẫu Rust
6. [Facade Pattern: Lớp IPC Command Mỏng Cho Desktop Frontend](#6-facade-pattern-lớp-ipc-command-mỏng-cho-desktop-frontend)
   - 6.1 Giao diện Thống nhất giữa React UI và Subsystem Rust
   - 6.2 Điều phối Luồng Nghiệp vụ & Streaming Event qua Channel
   - 6.3 Sơ đồ Luồng Gọi & Code Mẫu Rust
7. [Phân Tích Loại Bỏ: Flyweight & Bridge Cổ Điển (Negative Decisions)](#7-phân-tích-loại-bỏ-flyweight--bridge-cổ-điển-negative-decisions)
   - 7.1 Tại sao loại bỏ Flyweight truyền thống? (Thế chỗ bởi `Arc<str>` và String Interning)
   - 7.2 Tại sao loại bỏ Bridge truyền thống? (Thế chỗ bởi Traits và Monomorphization)
8. [Bảng Ma Trận Tổng Kết & Khuyến Nghị Triển Khai Cho Phase 1](#8-bảng-ma-trận-tổng-kết--khuyến-nghị-triển-khai-cho-phase-1)

---

## 1. Triết Lý Kiến Trúc: Structural Patterns trong Rust Idiomatic

Trong các ngôn ngữ hướng đối tượng cổ điển (C++, Java, C#), Structural Patterns thường dựa trên cây phân cấp lớp (class hierarchies), con trỏ ảo (virtual method tables) và kế thừa (inheritance). Tuy nhiên, Rust không có kế thừa class, không có con trỏ `null`, và áp dụng nghiêm ngặt các quy tắc về quyền sở hữu (Ownership), thời gian sống (Lifetimes) cũng như an toàn đa luồng (`Send + Sync`).

Khi áp dụng Structural Patterns vào hệ thống desktop agent runtime (`merlink-harness-ui`):
1. **Composition over Inheritance:** Thay vì dẫn xuất lớp, chúng ta bọc (wrap) các struct thông qua con trỏ thông minh (`Arc<dyn Trait>` hoặc generic `T: Trait`).
2. **Zero-Cost Abstractions:** Ưu tiên Static Dispatch (Monomorphization) tại các điểm nóng hiệu năng (hot-paths) và chỉ dùng Dynamic Dispatch (`dyn Trait`) tại các ranh giới kiến trúc (Architectural Boundaries) nơi cần tính đa hình cấu hình động (Runtime Pluggability).
3. **Compile-Time Enforcement:** Sử dụng hệ thống kiểu dữ liệu mạnh (Type System) và Newtype Pattern để biến các lỗi logic (nhầm ID dự án, rò rỉ khóa bí mật, gọi tool sai ngữ cảnh) thành lỗi không thể biên dịch.
4. **Hexagonal Boundaries:** Phân tách ranh giới crate rõ rệt (`domain` -> `agent` -> `llm` / `infra` -> `src-tauri`), biến domain thành hạt nhân độc lập với DB SQLite hay HTTP Client.

---

## 2. Adapter Pattern: Ports & Adapters (Hexagonal Architecture)

### 2.1 Chuẩn hóa Đa Giao thức LLM
Hệ thống Merlink yêu cầu kết nối linh hoạt tới 3 họ giao thức LLM hoàn toàn khác nhau:
- **OpenAI SSE (`chat_completions.rs`):** Chuẩn Server-Sent Events với payload `data: {"choices": [{"delta": {"content": "...", "tool_calls": [...]}}]}`.
- **Anthropic Messages (`anthropic.rs`):** Cấu trúc event phân mảnh (`message_start`, `content_block_start`, `content_block_delta` với `thinking_delta` cho reasoning, `content_block_stop`, `message_delta`).
- **Ollama NDJSON (`ollama.rs`):** Dòng JSON riêng lẻ (Newline Delimited JSON) với `{"message": {"content": "..."}, "done": false}`.

**Mục tiêu Adapter:** Tầng runtime (`agent`) chỉ nhìn thấy một cổng duy nhất (`port`): trait `LlmClient` trả về luồng `StreamItem` chuẩn hóa (`Text`, `Reasoning`, `ToolCallChunk`, `Usage`, `Done`). Toàn bộ việc parse socket, giải mã SSE/NDJSON, mapping delta nằm trọn trong `crates/llm/src/adapters/`.

### 2.2 Trừu tượng hóa Kho Khóa Bí mật (`SecretStore`)
Domain định nghĩa port `SecretStore`. Ứng dụng cung cấp 2 adapter:
- `KeyringSecretStore` (Production): Giao tiếp với OS Keychain (Windows Credential Manager, macOS Keychain, Linux Secret Service via DBus).
- `InMemorySecretStore` (CI / Headless Tests): Lưu trữ khóa trong bộ nhớ tạm thời (`HashMap<SecretRef, Zeroizing<String>>`), cho phép chạy test tự động 100% không cần màn hình desktop hay OS keyring daemon.

### 2.3 Sơ đồ Kiến trúc & Code Mẫu Rust

```
                    ┌────────────────────────────────────────────────────────┐
                    │               crates/agent (Run Loop)                  │
                    └───────────────────────────┬────────────────────────────┘
                                                │ (Gọi qua Port trừu tượng)
                                                ▼
                    ┌────────────────────────────────────────────────────────┐
                    │            PORT: crates/domain/src/ports.rs            │
                    │                  trait LlmClient                       │
                    └───────────────────────────┬────────────────────────────┘
                                                │
         ┌──────────────────────────────────────┼──────────────────────────────────────┐
         ▼                                      ▼                                      ▼
┌─────────────────────────┐            ┌─────────────────────────┐            ┌─────────────────────────┐
│ crates/llm/adapters/    │            │ crates/llm/adapters/    │            │ crates/llm/adapters/    │
│ chat_completions.rs     │            │ anthropic.rs            │            │ ollama.rs               │
├─────────────────────────┤            ├─────────────────────────┤            ├─────────────────────────┤
│ Parser: OpenAI SSE      │            │ Parser: Anthropic Event │            │ Parser: NDJSON lines    │
│ Transform: Delta Content│            │ Transform: thinking_    │            │ Transform: message.     │
│ & function call chunk   │            │ delta & tool_use blocks │            │ content & done flag     │
└────────────┬────────────┘            └────────────┬────────────┘            └────────────┬────────────┘
             │                                      │                                      │
             ▼                                      ▼                                      ▼
   OpenAI/Groq/vLLM HTTP                  Anthropic API HTTP                     Localhost:11434 HTTP
```

#### Code Rust Chuẩn Mực: Trait Port & Adapters

```rust
// ============================================================================
// File: crates/domain/src/ports.rs (Domain Port Definition)
// ============================================================================

use async_trait::async_trait;
use futures::Stream;
use std::pin::Pin;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SecretRef(pub String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StreamItem {
    Text(String),
    Reasoning(String), // Hỗ trợ thinking_delta từ Anthropic hoặc reasoning token từ DeepSeek-R1
    ToolCallChunk {
        id: String,
        name: Option<String>,
        arguments_delta: String,
    },
    Usage {
        input_tokens: u32,
        output_tokens: u32,
    },
    Done,
}

#[derive(Debug, thiserror::Error)]
pub enum LlmError {
    #[error("HTTP transport error: {0}")]
    Transport(String),
    #[error("Protocol parsing error: {0}")]
    Protocol(String),
    #[error("Rate limit exceeded (429): retry after {retry_after_secs:?} seconds")]
    RateLimit { retry_after_secs: Option<u64> },
    #[error("Context window overflow: prompt tokens exceed limit")]
    ContextOverflow,
    #[error("Upstream service unavailable (5xx): status {0}")]
    ServerUnavailable(u16),
    #[error("Authentication failed: invalid secret")]
    AuthenticationFailed,
}

pub type ItemStream = Pin<Box<dyn Stream<Item = Result<StreamItem, LlmError>> + Send>>;

#[derive(Debug, Clone)]
pub struct LlmRequest {
    pub model: String,
    pub messages: Vec<crate::session::MessageEnvelope>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
}

#[async_trait]
pub trait LlmClient: Send + Sync {
    async fn stream_chat(&self, req: LlmRequest) -> Result<ItemStream, LlmError>;
}

#[async_trait]
pub trait SecretStore: Send + Sync {
    async fn get(&self, secret_ref: &SecretRef) -> Result<Option<String>, std::io::Error>;
    async fn set(&self, secret_ref: &SecretRef, secret_value: &str) -> Result<(), std::io::Error>;
    async fn delete(&self, secret_ref: &SecretRef) -> Result<(), std::io::Error>;
}

// ============================================================================
// File: crates/llm/src/adapters/chat_completions.rs (OpenAI-Compat Adapter)
// ============================================================================

use async_trait::async_trait;
use futures::{StreamExt, TryStreamExt};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use crates_domain::ports::{LlmClient, LlmRequest, LlmError, ItemStream, StreamItem};

pub struct OpenAiCompatAdapter {
    http_client: reqwest::Client,
    base_url: String,
    api_key: String,
}

impl OpenAiCompatAdapter {
    pub fn new(base_url: String, api_key: String) -> Self {
        Self {
            http_client: reqwest::Client::new(),
            base_url,
            api_key,
        }
    }
}

#[async_trait]
impl LlmClient for OpenAiCompatAdapter {
    async fn stream_chat(&self, req: LlmRequest) -> Result<ItemStream, LlmError> {
        let endpoint = format!("{}/chat/completions", self.base_url.trim_end_matches('/'));
        
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", self.api_key))
                .map_err(|e| LlmError::Protocol(e.to_string()))?,
        );

        let body = serde_json::json!({
            "model": req.model,
            "stream": true,
            "messages": req.messages,
            "temperature": req.temperature,
            "max_tokens": req.max_tokens,
        });

        let response = self.http_client
            .post(&endpoint)
            .headers(headers)
            .json(&body)
            .send()
            .await
            .map_err(|e| LlmError::Transport(e.to_string()))?;

        let status = response.status();
        if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
            return Err(LlmError::RateLimit { retry_after_secs: None });
        } else if status == reqwest::StatusCode::BAD_REQUEST {
            return Err(LlmError::ContextOverflow);
        } else if status.is_server_error() {
            return Err(LlmError::ServerUnavailable(status.as_u16()));
        }

        // Chuyển đổi SSE byte-stream sang StreamItem chuẩn hóa
        let byte_stream = response.bytes_stream();
        let item_stream = async_stream::try_stream! {
            let mut lines = byte_stream;
            while let Some(chunk) = lines.next().await {
                let bytes = chunk.map_err(|e| LlmError::Transport(e.to_string()))?;
                let text = String::from_utf8_lossy(&bytes);
                
                for line in text.lines() {
                    let line = line.trim();
                    if line.starts_with("data: [DONE]") {
                        yield StreamItem::Done;
                        return;
                    }
                    if let Some(json_str) = line.strip_prefix("data: ") {
                        if let Ok(val) = serde_json::from_str::<serde_json::Value>(json_str) {
                            if let Some(delta) = val["choices"][0]["delta"]["content"].as_str() {
                                yield StreamItem::Text(delta.to_string());
                            }
                            if let Some(tool_calls) = val["choices"][0]["delta"]["tool_calls"].as_array() {
                                for tc in tool_calls {
                                    let id = tc["id"].as_str().unwrap_or_default().to_string();
                                    let name = tc["function"]["name"].as_str().map(|s| s.to_string());
                                    let args = tc["function"]["arguments"].as_str().unwrap_or_default().to_string();
                                    yield StreamItem::ToolCallChunk {
                                        id,
                                        name,
                                        arguments_delta: args,
                                    };
                                }
                            }
                        }
                    }
                }
            }
        };

        Ok(Box::pin(item_stream))
    }
}
```

---

## 3. Decorator Pattern: Composable Middleware Wrapping

### 3.1 Bọc `LlmClient`: Retry, Jitter, Context Compaction & Tracing
Thay vì đưa logic retry hoặc logging vào trong từng Adapter hoặc lặp lại trong Run Loop, Decorator Pattern cho phép bọc các lớp hành vi bên ngoài `Arc<dyn LlmClient>` như các lớp vỏ củ hành (onion middleware):

1. **`TracingLlmDecorator`**:
   - Chặn request, che mờ (redact) thông tin nhạy cảm (API keys, Authorization header).
   - Đo lường độ trễ (latency), số lượng token stream và emit log/audit step vào bảng SQLite `run_steps`.
2. **`RetryLlmDecorator`**:
   - Bắt các lỗi tạm thời (`LlmError::RateLimit`, `LlmError::ServerUnavailable`, sự cố rớt socket SSE).
   - Áp dụng chiến lược **Exponential Backoff + Full Jitter** (`sleep = rand(0..min(max_backoff, base * 2^attempt))`).
   - Bắt lỗi **`LlmError::ContextOverflow`**: Tự động gọi thành phần `ContextCompactor` (tóm tắt lịch sử cũ, cắt tỉa message không quan trọng) và tiến hành retry yêu cầu đúng 1 lần duy nhất trước khi chấp nhận thất bại.

### 3.2 Bọc `ToolExecutor`: CommitGuard (Transactional Savepoint & Run State Check)
Hệ sinh thái Agent tiềm ẩn nguy cơ: khi user nhấn Cancel, lệnh cancel truyền xuống nhưng một tool ghi dữ liệu đang chuẩn bị commit vào SQLite. Nếu không có cơ chế bảo vệ, dữ liệu bị ghi lậu (stale mutation).
- **`CommitGuardToolDecorator`**: Bọc quanh `ToolExecutor`:
  1. Mở một transaction duy nhất trên `writer_pool` (với SQLite single-writer).
  2. Tạo một `SAVEPOINT tool_execution_sp` trước khi cho tool chạy.
  3. Sau khi tool chạy xong thành công, trước khi gọi `COMMIT`: Truy vấn trạng thái run hiện tại `SELECT status FROM agent_runs WHERE id = $1`.
  4. Nếu status đã chuyển sang `cancelled` hoặc `interrupted`: Ngay lập tức gọi `ROLLBACK TO SAVEPOINT` và hủy bỏ toàn bộ tác động của tool call.

### 3.3 Sơ đồ Pipeline & Code Mẫu Rust

```
                                 [ Agent Run Loop ]
                                         │
                                         ▼
                 ┌───────────────────────────────────────────────┐
                 │             TracingLlmDecorator               │
                 │   - Mask Secret / Authorization Headers       │
                 │   - Đo Latency, Ghi step vào `run_steps`      │
                 └───────────────────────┬───────────────────────┘
                                         │
                                         ▼
                 ┌───────────────────────────────────────────────┐
                 │              RetryLlmDecorator                │
                 │   - Exponential Backoff + Full Jitter (429)   │
                 │   - Bắt ContextOverflow -> Gọi Compactor ->   │
                 │     Tỉa bớt message -> Retry duy nhất 1 lần   │
                 └───────────────────────┬───────────────────────┘
                                         │
                                         ▼
                 ┌───────────────────────────────────────────────┐
                 │       Target Adapter: Arc<dyn LlmClient>      │
                 │       (OpenAI / Anthropic / Ollama)           │
                 └───────────────────────────────────────────────┘
```

#### Code Rust Chuẩn Mực: Decorator Implementations

```rust
// ============================================================================
// File: crates/llm/src/retry.rs (Retry & Context Compaction Decorator)
// ============================================================================

use std::sync::Arc;
use std::time::Duration;
use async_trait::async_trait;
use rand::Rng;
use crates_domain::ports::{LlmClient, LlmRequest, LlmError, ItemStream};

#[async_trait]
pub trait ContextCompactor: Send + Sync {
    async fn compact(&self, req: &mut LlmRequest) -> Result<(), LlmError>;
}

pub struct RetryLlmDecorator {
    inner: Arc<dyn LlmClient>,
    compactor: Arc<dyn ContextCompactor>,
    max_retries: usize,
    base_backoff: Duration,
    max_backoff: Duration,
}

impl RetryLlmDecorator {
    pub fn new(
        inner: Arc<dyn LlmClient>,
        compactor: Arc<dyn ContextCompactor>,
        max_retries: usize,
    ) -> Self {
        Self {
            inner,
            compactor,
            max_retries,
            base_backoff: Duration::from_millis(500),
            max_backoff: Duration::from_secs(10),
        }
    }

    fn calculate_jitter(&self, attempt: usize) -> Duration {
        let exp = 2u64.saturating_pow(attempt as u32);
        let backoff_limit = (self.base_backoff.as_millis() as u64)
            .saturating_mul(exp)
            .min(self.max_backoff.as_millis() as u64);
        let mut rng = rand::thread_rng();
        let jittered_millis = rng.gen_range(0..=backoff_limit);
        Duration::from_millis(jittered_millis)
    }
}

#[async_trait]
impl LlmClient for RetryLlmDecorator {
    async fn stream_chat(&self, mut req: LlmRequest) -> Result<ItemStream, LlmError> {
        let mut attempts = 0;
        let mut has_compacted = false;

        loop {
            match self.inner.stream_chat(req.clone()).await {
                Ok(stream) => return Ok(stream),
                Err(LlmError::ContextOverflow) if !has_compacted => {
                    tracing::warn!("Context overflow detected. Invoking compactor and retrying once...");
                    self.compactor.compact(&mut req).await?;
                    has_compacted = true;
                    // Retry ngay lập tức sau khi compactor hoàn tất
                    continue;
                }
                Err(err @ LlmError::RateLimit { .. }) | Err(err @ LlmError::ServerUnavailable(_)) => {
                    attempts += 1;
                    if attempts > self.max_retries {
                        return Err(err);
                    }
                    let delay = self.calculate_jitter(attempts);
                    tracing::warn!(
                        "Transient error encountered ({:?}). Retrying attempt {}/{} in {:?}",
                        err, attempts, self.max_retries, delay
                    );
                    tokio::time::sleep(delay).await;
                }
                Err(fatal) => return Err(fatal),
            }
        }
    }
}

// ============================================================================
// File: crates/agent/src/tools/commit_guard.rs (Commit Guard Tool Decorator)
// ============================================================================

use async_trait::async_trait;
use sqlx::{Sqlite, Pool, Transaction};
use crates_domain::ports::RunId;

#[async_trait]
pub trait ToolExecutor: Send + Sync {
    async fn execute(
        &self,
        tool_name: &str,
        args_json: &serde_json::Value,
        tx: &mut Transaction<'_, Sqlite>,
    ) -> Result<serde_json::Value, String>;
}

pub struct CommitGuardToolDecorator {
    inner: Arc<dyn ToolExecutor>,
    db_pool: Pool<Sqlite>,
}

impl CommitGuardToolDecorator {
    pub fn new(inner: Arc<dyn ToolExecutor>, db_pool: Pool<Sqlite>) -> Self {
        Self { inner, db_pool }
    }

    pub async fn execute_guarded(
        &self,
        run_id: &RunId,
        tool_name: &str,
        args_json: &serde_json::Value,
    ) -> Result<serde_json::Value, String> {
        // 1. Mở transaction trên writer pool
        let mut tx = self.db_pool.begin().await.map_err(|e| e.to_string())?;

        // 2. Tạo SAVEPOINT để cô lập tool execution
        sqlx::query("SAVEPOINT tool_exec_sp")
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

        // 3. Thực thi tool bên trong transaction
        let exec_result = self.inner.execute(tool_name, args_json, &mut tx).await;

        if let Err(err) = exec_result {
            // Rollback savepoint nếu tool thực thi thất bại
            let _ = sqlx::query("ROLLBACK TO SAVEPOINT tool_exec_sp")
                .execute(&mut *tx)
                .await;
            return Err(err);
        }

        // 4. KIỂM TRA TRẠNG THÁI RUN TRƯỚC KHI COMMIT
        let status: Option<String> = sqlx::query_scalar(
            "SELECT status FROM agent_runs WHERE id = $1"
        )
        .bind(&run_id.0)
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        match status.as_deref() {
            Some("running") => {
                // Thỏa mãn điều kiện: Commit transaction an toàn
                sqlx::query("RELEASE SAVEPOINT tool_exec_sp")
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| e.to_string())?;
                tx.commit().await.map_err(|e| e.to_string())?;
                exec_result
            }
            Some(other) => {
                // Run đã bị cancel, pause hoặc interrupt trong lúc tool đang chạy
                let _ = sqlx::query("ROLLBACK TO SAVEPOINT tool_exec_sp")
                    .execute(&mut *tx)
                    .await;
                let _ = tx.rollback().await;
                Err(format!("CommitGuard aborted: run is in '{}' state, changes reverted.", other))
            }
            None => {
                let _ = tx.rollback().await;
                Err("CommitGuard aborted: run record not found.".to_string())
            }
        }
    }
}
```

---

## 4. Proxy Pattern & Newtype Pattern: Capability & Security Enforcement

### 4.1 `OutboundGuardProxy`: Chống SSRF, DNS Rebinding & Bảo vệ Hạ tầng Mạng
Trong một ứng dụng desktop hỗ trợ Custom Provider / Local Model, người dùng hoặc LLM có thể cung cấp `base_url`. Nếu không kiểm soát, hệ thống dễ tổn thương trước các cuộc tấn công **Server-Side Request Forgery (SSRF)**:
- Kẻ tấn công cấu hình URL trỏ vào metadata service đám mây (`http://169.254.169.254/latest/meta-data/`) để đánh cắp IAM token nếu app chạy trên cloud worktree.
- DNS Rebinding: Domain trỏ về IP công cộng lúc check nhưng resolve ra IP nội bộ (`127.0.0.1`, `10.0.0.0/8`, `192.168.0.0/16`) lúc fetch dữ liệu.

`OutboundGuardProxy` đóng vai trò là một Smart Proxy chặn trước mọi HTTP dispatch:
1. Phân biệt cờ `is_local`:
   - Nếu `is_local == true` (Ollama, LM Studio, vLLM local): Chỉ cho phép scheme `http://` hoặc `https://`, và host bắt buộc phải là `localhost`, `127.0.0.1`, `[::1]`, hoặc subnet mạng LAN được cấp phép rõ ràng.
   - Nếu `is_local == false` (Cloud Providers như OpenAI, Anthropic, OpenRouter): Bắt buộc scheme phải là `https://`. Nghiêm cấm phân giải ra bất kỳ địa chỉ IPv4/IPv6 loopback, link-local, private mạng nội bộ hoặc `169.254.169.254`.

### 4.2 `ScopedRepoProxy`: Cưỡng chế Phạm vi Dự án (Anti-Confused Deputy Attack)
Trong LLM tool calling, agent có thể bị prompt injection hoặc hallucination dẫn đến việc truyền `project_id` của một tiểu thuyết khác vào câu lệnh chỉnh sửa nhân vật.
- **Giải pháp:** `ToolContext` do runtime tạo ra giữ vững `project_id` chân thực của phiên làm việc. Tool implementation KHÔNG nhận `project_id` từ tham số do LLM sinh ra.
- `ScopedRepoProxy` bọc quanh Database Repository: Tự động tiêm `WHERE project_id = $1` vào mọi câu lệnh `SELECT`, `UPDATE`, `DELETE`, và thực hiện `JOIN project_characters` đối với bảng N-N giữa dự án và nhân vật. LLM hoàn toàn không thể chạm tới dữ liệu của dự án khác kể cả khi thao túng mã định danh.

### 4.3 `Newtype Pattern`: Type-Safety ở cấp độ Compiler
Tránh hoàn toàn lỗi hoán đổi nhầm các chuỗi ID (`session_id` truyền vào biến `run_id`, hoặc `project_id` truyền vào `character_id`).
- Định nghĩa:
  ```rust
  #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
  pub struct ProjectId(pub uuid::Uuid);

  #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
  pub struct SessionId(pub String); // Format: ses_01H...

  #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
  pub struct RunId(pub String);     // Format: run_01H...

  #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
  pub struct SecretRef(pub String); // Format: sec_keyring_...
  ```

### 4.4 Sơ đồ Bảo mật & Code Mẫu Rust

```
                    ┌────────────────────────────────────────────────────────┐
                    │               LLM / External Base URL                  │
                    └───────────────────────────┬────────────────────────────┘
                                                │
                                                ▼
                    ┌────────────────────────────────────────────────────────┐
                    │                 OutboundGuardProxy                     │
                    ├────────────────────────────────────────────────────────┤
                    │ [Check Scheme] : Cloud bắt buộc HTTPS?                 │
                    │ [DNS Resolve]  : Phân giải IP trước khi gửi            │
                    │ [IP Filtering] : Chặn 169.254.169.254, Private IP       │
                    │                  Chỉ cho phép localhost nếu is_local   │
                    └───────────────────────────┬────────────────────────────┘
                                                │ (An toàn)
                                                ▼
                                         reqwest::Client
```

#### Code Rust Chuẩn Mực: Outbound Guard & Scoped Repo Proxy

```rust
// ============================================================================
// File: crates/infra/src/proxy/outbound_guard.rs (SSRF Protection Proxy)
// ============================================================================

use std::net::{IpAddr, SocketAddr};
use url::Url;

#[derive(Debug, thiserror::Error)]
pub enum SecurityError {
    #[error("Invalid URL scheme: {0}. Cloud providers require HTTPS.")]
    InsecureScheme(String),
    #[error("SSRF Protection: Access to private/loopback IP {0} is forbidden for cloud providers.")]
    BlockedIp(IpAddr),
    #[error("SSRF Protection: Access to link-local/cloud metadata service is strictly blocked.")]
    CloudMetadataBlocked,
    #[error("DNS resolution failed for host: {0}")]
    DnsResolutionFailed(String),
}

pub struct OutboundGuardProxy;

impl OutboundGuardProxy {
    pub async fn validate_target(target_url: &str, is_local: bool) -> Result<Url, SecurityError> {
        let parsed = Url::parse(target_url)
            .map_err(|e| SecurityError::InsecureScheme(e.to_string()))?;

        // 1. Kiểm tra scheme
        if !is_local && parsed.scheme() != "https" {
            return Err(SecurityError::InsecureScheme(parsed.scheme().to_string()));
        }

        let host_str = parsed.host_str().ok_or_else(|| {
            SecurityError::InsecureScheme("Missing hostname in target URL".to_string())
        })?;

        // 2. Chặn trực tiếp địa chỉ AWS/GCP/Azure Metadata Service
        if host_str == "169.254.169.254" || host_str == "metadata.google.internal" {
            return Err(SecurityError::CloudMetadataBlocked);
        }

        // 3. Phân giải DNS và kiểm duyệt dải IP
        let port = parsed.port_or_known_default().unwrap_or(443);
        let addresses: Vec<SocketAddr> = tokio::net::lookup_host(format!("{}:{}", host_str, port))
            .await
            .map_err(|_| SecurityError::DnsResolutionFailed(host_str.to_string()))?
            .collect();

        for addr in addresses {
            let ip = addr.ip();
            let is_private_or_loopback = match ip {
                IpAddr::V4(v4) => v4.is_loopback() || v4.is_private() || v4.is_link_local(),
                IpAddr::V6(v6) => v6.is_loopback(),
            };

            if is_local {
                // Local provider chỉ được phép chạy trên loopback hoặc mạng nội bộ
                if !is_private_or_loopback {
                    return Err(SecurityError::BlockedIp(ip));
                }
            } else {
                // Cloud provider tuyệt đối không được trỏ vào loopback / private IP
                if is_private_or_loopback {
                    return Err(SecurityError::BlockedIp(ip));
                }
            }
        }

        Ok(parsed)
    }
}

// ============================================================================
// File: crates/infra/src/proxy/scoped_repo.rs (Confused Deputy Guard)
// ============================================================================

use sqlx::{Sqlite, Pool};
use uuid::Uuid;
use crates_domain::story::Character;

#[derive(Debug, Clone, Copy)]
pub struct ProjectId(pub Uuid);

pub struct ScopedCharacterRepoProxy {
    pool: Pool<Sqlite>,
    project_id: ProjectId,
}

impl ScopedCharacterRepoProxy {
    pub fn new(pool: Pool<Sqlite>, project_id: ProjectId) -> Self {
        Self { pool, project_id }
    }

    /// Cưỡng chế JOIN với bảng quan hệ nhiều-nhiều project_characters,
    /// bảo đảm character_id dù có tồn tại trên DB nhưng không thuộc project_id này
    /// thì truy vấn vẫn trả về None.
    pub async fn get_character_scoped(&self, character_id: Uuid) -> Result<Option<Character>, sqlx::Error> {
        let rec = sqlx::query_as!(
            Character,
            r#"
            SELECT c.id, c.name, c.summary
            FROM characters c
            INNER JOIN project_characters pc ON pc.character_id = c.id
            WHERE pc.project_id = $1 AND c.id = $2 AND c.deleted_at IS NULL
            "#,
            self.project_id.0,
            character_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(rec)
    }
}
```

---

## 5. Composite Pattern: Cây Phân Cấp Thực Thể Tự Tham Chiếu

### 5.1 Quản lý Không gian Thế giới (`world_settings`) & Hệ Thống Chủ đề (`themes`)
Hệ thống Merlink quản lý các tiểu thuyết có thế giới rộng lớn và nhiều phân đoạn cốt truyện (Arcs):
- Bảng `world_settings`: Cấu trúc cây không gian địa lý phân cấp qua trường `parent_id` tự tham chiếu:
  `Realm` (Tam giới) $\rightarrow$ `World` (Nhân giới) $\rightarrow$ `Continent` (Đông Thần Châu) $\rightarrow$ `Region` (Đại Chu Vương Triều) $\rightarrow$ `Facility` (Vương Cung) $\rightarrow$ `Room` (Tàng Thư Các).
- Bảng `themes`: Cấu trúc cây chủ đề phân cấp qua `parent_theme_id` (Theme gốc kế thừa cho từng Arc).

### 5.2 Kế thừa Quy tắc, Path Traversal & Phát hiện Vòng lặp (Cycle Detection)
Các thực thể tầng con tự động kế thừa quy tắc môi trường, cấm kỵ hoặc định luật vật lý từ tầng cha.
Khi client hoặc agent yêu cầu lấy thông tin địa điểm của một `Room`, Composite Pattern giải quyết:
1. **Full Path Traversal:** Trả về chuỗi địa danh hoàn chỉnh: `Realm / World / Continent / Region / Facility / Room`.
2. **Rule Aggregation:** Hợp nhất toàn bộ `lore_entries` và `rules` từ gốc tới ngọn.
3. **Cycle Detection:** Khi người dùng thao tác UI hoặc tool chỉnh sửa `parent_id`, thuật toán phát hiện chu trình (sử dụng `HashSet<Uuid>`) ngăn chặn đồ thị vòng (circular reference), triệt tiêu lỗi đệ quy vô hạn gây tràn stack.

### 5.3 Sơ đồ Cây Thực Thể & Code Mẫu Rust

```
                           [ Realm: Thần Thoại Cổ Đại ]
                                        │
                                        ▼ (parent_id)
                           [ World: Huyền Huyễn Đại Lục ]
                                        │
                                        ▼ (parent_id)
                           [ Continent: Đông Thần Châu ]
                                        │
                         ┌──────────────┴──────────────┐
                         ▼                             ▼
              [ Region: Vô Tận Hải ]       [ Region: Đại Chu Vương Triều ]
                                                       │
                                                       ▼
                                            [ Facility: Hoàng Cung ]
                                                       │
                                                       ▼
                                            [ Room: Tàng Thư Các ]
```

#### Code Rust Chuẩn Mực: Composite Implementation & Traversal

```rust
// ============================================================================
// File: crates/domain/src/story/world_setting.rs (Composite Pattern)
// ============================================================================

use std::collections::{HashMap, HashSet};
use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldSettingNode {
    pub id: Uuid,
    pub parent_id: Option<Uuid>,
    pub name: String,
    pub setting_type: String, // realm, world, continent, region, facility, room
    pub environment_rules: Vec<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum HierarchyError {
    #[error("Circular reference detected at node {0}")]
    CycleDetected(Uuid),
    #[error("Node not found: {0}")]
    NodeNotFound(Uuid),
}

pub struct WorldSettingTree {
    nodes: HashMap<Uuid, WorldSettingNode>,
}

impl WorldSettingTree {
    pub fn new(nodes_list: Vec<WorldSettingNode>) -> Self {
        let mut nodes = HashMap::new();
        for node in nodes_list {
            nodes.insert(node.id, node);
        }
        Self { nodes }
    }

    /// Duyệt ngược từ node hiện tại lên root để sinh đường dẫn phân cấp (Breadcrumb)
    /// và phát hiện vòng lặp vô hạn.
    pub fn resolve_path(&self, start_id: Uuid) -> Result<String, HierarchyError> {
        let mut path_segments = Vec::new();
        let mut visited = HashSet::new();
        let mut curr_id = Some(start_id);

        while let Some(id) = curr_id {
            if !visited.insert(id) {
                return Err(HierarchyError::CycleDetected(id));
            }

            let node = self.nodes.get(&id)
                .ok_or(HierarchyError::NodeNotFound(id))?;

            path_segments.push(node.name.clone());
            curr_id = node.parent_id;
        }

        path_segments.reverse();
        Ok(path_segments.join(" / "))
    }

    /// Kế thừa toàn bộ quy tắc môi trường (Environment Rules) từ Node gốc đến Node lá
    pub fn aggregate_inherited_rules(&self, start_id: Uuid) -> Result<Vec<String>, HierarchyError> {
        let mut rules = Vec::new();
        let mut visited = HashSet::new();
        let mut curr_id = Some(start_id);

        let mut ancestor_chain = Vec::new();

        while let Some(id) = curr_id {
            if !visited.insert(id) {
                return Err(HierarchyError::CycleDetected(id));
            }
            let node = self.nodes.get(&id)
                .ok_or(HierarchyError::NodeNotFound(id))?;
            ancestor_chain.push(node);
            curr_id = node.parent_id;
        }

        // Quy tắc từ root áp dụng trước, quy tắc lá override hoặc bổ sung sau
        for node in ancestor_chain.into_iter().rev() {
            rules.extend(node.environment_rules.clone());
        }

        Ok(rules)
    }
}
```

---

## 6. Facade Pattern: Lớp IPC Command Mỏng Cho Desktop Frontend

### 6.1 Giao diện Thống nhất giữa React UI và Subsystem Rust
Kiến trúc Merlink quy định: Tầng `src-tauri/src/commands/` là một **Facade mỏng**, không chứa nghiệp vụ logic (Business-agnostic orchestration).
React Frontend gọi các lệnh IPC thông qua Type-safe bindings do `tauri-specta` tạo ra (`invoke(...)`).

Tầng Facade thực hiện 5 bước điều phối chuẩn tắc:
1. Nhận tham số DTO từ Frontend.
2. Kiểm tra tính hợp lệ sơ bộ của phiên (Validation & Preconditions).
3. Đọc thông tin cấu hình (`ModelProfile`, `ProviderSettings`, `SecretStore`).
4. Khởi tạo hoặc ủy quyền cho `AgentRuntime` điều phối `RunLoop`.
5. Tạo `tauri::ipc::Channel<RunEvent>` để stream trực tiếp các gói token, tool call, verification step và approval status về React UI mà không qua cơ chế polling.

### 6.2 Điều phối Luồng Nghiệp vụ & Streaming Event qua Channel

```
React UI (WebView)
  │
  │  tauri-specta generated: chat_send(session_id, content, on_event)
  ▼
┌────────────────────────────────────────────────────────────────────────┐
│           FACADE: src-tauri/src/commands/chat.rs                       │
├────────────────────────────────────────────────────────────────────────┤
│ 1. Kiểm tra trạng thái Session (Active? Lock?)                         │
│ 2. Lấy Provider/Model profile & Giải mã Secret qua SecretStore         │
│ 3. Khởi tạo Channel<RunEvent> kết nối trực tiếp FE                     │
│ 4. Spawn tokio task gọi AgentRuntime::start_run(...)                   │
│ 5. Trả về RunSummaryDTO ngay lập tức                                   │
└───────────────────┬────────────────────────────────────────────────────┘
                    │
                    ▼
         crates/agent (Run Loop Core)
```

### 6.3 Code Mẫu Rust Chuẩn Mực

```rust
// ============================================================================
// File: src-tauri/src/commands/chat.rs (Tauri Specta Command Facade)
// ============================================================================

use tauri::ipc::Channel;
use serde::{Deserialize, Serialize};
use specta::Type;
use crates_domain::ports::{SessionId, RunId, StreamItem};

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub enum RunUiEvent {
    TokenDelta(String),
    ReasoningDelta(String),
    ToolExecuting { name: String, args: String },
    AwaitingApproval { run_id: String, tool_name: String, args: String },
    RunCompleted { total_tokens: u32 },
    RunFailed { error: String },
}

#[derive(Debug, Deserialize, Type)]
pub struct SendMessageDto {
    pub session_id: String,
    pub content: String,
}

#[derive(Debug, Serialize, Type)]
pub struct SendMessageResult {
    pub run_id: String,
}

/// Facade Command: Điểm tiếp nhận duy nhất từ UI, che giấu sự phức tạp của
/// DB pools, LLM client assembly, Secret retrieval, và Concurrency Semaphores.
#[tauri::command]
#[specta::specta]
pub async fn chat_send(
    dto: SendMessageDto,
    on_event: Channel<RunUiEvent>,
    state: tauri::State<'_, crate::AppState>,
) -> Result<SendMessageResult, String> {
    let session_id = SessionId(dto.session_id);
    let run_id = RunId(format!("run_{}", uuid::Uuid::now_v7()));

    // 1. Kiểm tra Semaphore & đảm bảo chỉ 1 run active / session
    let run_permit = state.agent_scheduler
        .acquire_session_permit(&session_id)
        .await
        .map_err(|e| e.to_string())?;

    // 2. Facade phân tách stream runtime thành UI channel event
    let event_sink = on_event.clone();
    let runtime = state.agent_runtime.clone();
    let run_id_cloned = run_id.clone();

    tokio::spawn(async move {
        // Giữ permit trong suốt vòng đời của run
        let _permit = run_permit;

        let result = runtime.execute_run(&session_id, &run_id_cloned, &dto.content, |item| {
            let ui_event = match item {
                StreamItem::Text(t) => RunUiEvent::TokenDelta(t),
                StreamItem::Reasoning(r) => RunUiEvent::ReasoningDelta(r),
                StreamItem::ToolCallChunk { name, arguments_delta, .. } => {
                    RunUiEvent::ToolExecuting {
                        name: name.unwrap_or_default(),
                        args: arguments_delta,
                    }
                }
                StreamItem::Done => RunUiEvent::RunCompleted { total_tokens: 0 },
                _ => return,
            };
            let _ = event_sink.send(ui_event);
        }).await;

        if let Err(err) = result {
            let _ = event_sink.send(RunUiEvent::RunFailed { error: err.to_string() });
        }
    });

    Ok(SendMessageResult {
        run_id: run_id.0,
    })
}
```

---

## 7. Phân Tích Loại Bỏ: Flyweight & Bridge Cổ Điển (Negative Decisions)

### 7.1 Tại sao loại bỏ Flyweight truyền thống?
- **Mô hình cổ điển (OOP Flyweight):** Tạo ra một `FlyweightFactory` quản lý một bảng băm các đối tượng chia sẻ trạng thái nội tại (intrinsic state) và tách riêng trạng thái ngoại lai (extrinsic state) được truyền vào hàm `operation(extrinsicState)`.
- **Lý do loại bỏ trong Rust:**
  1. Trong Rust, việc triển khai `FlyweightFactory` mutable sở hữu các đối tượng chia sẻ thường dẫn đến xung đột với bộ kiểm tra mượn (Borrow Checker), buộc phải dùng `Arc<RwLock<T>>` hoặc `Rc<RefCell<T>>` gây overhead khóa (lock contention) không cần thiết.
  2. Rust đã có sẵn **Zero-Cost Flyweight idioms**:
     - `Arc<str>` hoặc `Arc<[u8]>`: Chia sẻ chuỗi bất biến (tên character, prompt templates, system instructions) giữa nhiều luồng với chi phí bộ nhớ chỉ 16 bytes pointer + reference counter.
     - String Interning qua crate chuyên dụng như `lasso` hoặc `string-cache` nếu cần tối ưu bộ nhớ ở mức nén token cực đại.
     - Cấu trúc dữ liệu dạng ECS / Relational Tables: Lưu trữ ID thay vì nhân bản object.

### 7.2 Tại sao loại bỏ Bridge truyền thống?
- **Mô hình cổ điển (OOP Bridge):** Tách một trừu tượng lớn (Abstraction) thành hai cây phân cấp lớp riêng biệt: một cây cho Abstraction (ví dụ `RemoteControl` $\rightarrow$ `AdvancedRemoteControl`) và một cây cho Implementation (`Device` $\rightarrow$ `Tv`, `Radio`), kết nối với nhau qua một con trỏ lớp cơ sở.
- **Lý do loại bỏ trong Rust:**
  1. Đây là giải pháp giải quyết điểm yếu của kế thừa đa mức (multiple inheritance) trong C++ và Java. Rust hoàn toàn không có kế thừa class, nên nhu cầu chia tách hai cây phân cấp lớp không tồn tại.
  2. Rust giải quyết bài toán tách biệt Abstraction và Implementation tự nhiên và tinh gọn hơn rất nhiều thông qua:
     - **Trait Static Monomorphization:** `struct Service<T: Transport> { transport: T }` - Trình biên dịch sinh mã tối ưu hóa inlining, không tốn bất kỳ chi phí vtable con trỏ hàm nào.
     - Khi cần runtime polymorphism, Rust sử dụng `Box<dyn Trait>` hoặc `Arc<dyn Trait>` ngay tại điểm cắm (Ports & Adapters), biến cấu trúc Bridge cồng kềnh thành một trait composition đơn giản.

---

## 8. Bảng Ma Trận Tổng Kết & Khuyến Nghị Triển Khai Cho Phase 1

| Pattern | Vị trí Module (Crate) | Mục Tiêu Kỹ Thuật | Rủi Ro Triệt Tiêu |
|---|---|---|---|
| **Adapter** | `crates/llm/src/adapters/`<br>`crates/infra/src/secrets.rs` | Chuẩn hóa OpenAI SSE, Anthropic thinking, Ollama về `StreamItem`; cô lập OS Keychain / In-memory test | Xung đột giao thức, phụ thuộc chặt chẽ vào vendor API, không chạy được CI |
| **Decorator** | `crates/llm/src/retry.rs`<br>`crates/agent/src/tools/commit_guard.rs` | Exponential backoff + Jitter, Context auto-compaction, Transactional Savepoint cho tool | Tràn context token, nghẽn mạng 429, Stale Mutation khi Cancel run |
| **Proxy** | `crates/infra/src/proxy/outbound_guard.rs`<br>`crates/infra/src/proxy/scoped_repo.rs` | Thẩm định IP/Scheme chống SSRF; Cưỡng chế tiêm `project_id` vào mọi câu lệnh SQL | Đánh cắp metadata cloud, tấn công Confused Deputy giữa các dự án |
| **Newtype** | `crates/domain/src/story/`<br>`crates/domain/src/run.rs` | Bao bọc `ProjectId(Uuid)`, `SessionId(String)`, `RunId(String)` thành kiểu dữ liệu riêng | Truyền nhầm ID giữa các hàm ở cấp độ compile-time |
| **Composite** | `crates/domain/src/story/world_setting.rs`<br>`crates/domain/src/story/theme.rs` | Quản lý cây tự tham chiếu địa danh và theme; Hợp nhất quy tắc kế thừa và Cycle Detection | Đệ quy vô hạn làm sập tiến trình (Stack Overflow), sai lệch bối cảnh không gian |
| **Facade** | `src-tauri/src/commands/` | Che giấu toàn bộ độ phức tạp runtime sau các command Tauri mỏng; truyền `Channel<RunUiEvent>` | Rò rỉ logic nghiệp vụ lên React UI; Polling gây quá tải IPC |

### Khuyến Nghị Triển Khai (Actionable Next Steps):
1. **Hoàn thiện Crate Boundary:** Không import `reqwest`, `sqlx` vào `crates/domain` hoặc `crates/agent`. Ranh giới phải được bảo đảm bởi `Cargo.toml`.
2. **Triển khai OutboundGuardProxy sớm:** Bắt buộc tích hợp vào cấu hình Provider trước khi cho phép người dùng nhập Custom Endpoint URL.
3. **CommitGuard là cốt lõi an toàn giao dịch:** Tất cả các built-in tools can thiệp dữ liệu (`character`, `scene`, `lore`) bắt buộc phải bọc qua `CommitGuardToolDecorator` trước khi xuất hiện trong Tool Registry.
