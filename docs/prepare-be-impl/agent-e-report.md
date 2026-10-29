# BÁO CÁO NGHIÊN CỨU KIẾN TRÚC BACKEND HARNESS AGENT
## Chuyên sâu: Tauri v2 IPC Architecture, Type-Safe Bindings, Streaming Channels & SQLite Database với sqlx

**Người thực hiện:** Subagent E — Backend Architecture & Core Systems  
**Dự án:** Merlink Harness UI (`merlink-harness-ui`)  
**Tài liệu đối chiếu:** `docs/be-tauri-plan.md` (mục 1, 2, 3, 4, 5, 12, 13) và `docs/db.dbml`  
**Nguồn tra cứu thực tế:** Tài liệu chính thức Tauri v2 (`v2.tauri.app/develop/calling-frontend`), `tauri-specta` v2 (`specta.dev/docs/tauri-specta`), và thực nghiệm tối ưu SQLite WAL Concurrency với `sqlx` (2026)  
**Ngày hoàn thành:** 29/10/2026  

---

## MỤC LỤC
1. [TỔNG QUAN & BỐI CẢNH KIẾN TRÚC](#1-tổng-quan--bối-cảnh-kiến-trúc)
2. [KHẢO SÁT CHUYÊN SÂU CÔNG NGHỆ TAURI V2 IPC & SQLITE TRONG RUST](#2-khảo-sát-chuyên-sâu-công-nghệ-tauri-v2-ipc--sqlite-trong-rust)
   - 2.1 So sánh toàn diện: `invoke` vs `app.emit` vs `tauri::ipc::Channel<T>`
   - 2.2 Type-Safe Bindings với `tauri-specta` v2 (RC-27)
   - 2.3 Phân tích chuyên sâu Concurrency & Locking của SQLite với `sqlx`
3. [THIẾT KẾ CHI TIẾT HỆ THỐNG](#3-thiết-kế-chi-tiết-hệ-thống)
   - 3.1 Mô hình IPC Streaming với `tauri::ipc::Channel<RunEvent>`
   - 3.2 Khung sinh mã Type-Safe với `tauri-specta` v2 & Quản trị Lỗi `AppError`
   - 3.3 Kiến trúc SQLite Cục bộ Tối ưu: Mô hình Dual-Pool (Writer/Reader) & PRAGMA Tuning
     - Phân tích cạm bẫy Lock Starvation trong Async Rust và Giải pháp Dual-Pool
     - Cấu hình PRAGMA chuẩn cho Desktop App
     - Chiến lược Migration nhúng binary với `sqlx::migrate!`
     - Transaction Isolation & Cơ chế Commit Guard
   - 3.4 Đồng bộ Mô hình Dữ liệu DBML và Frontend Cache Invalidation
     - Cập nhật quan hệ N-N `project_characters` thay thế 1-N cũ
     - Cơ chế Cache Invalidation với TanStack Query và `events.entityChanged`
4. [ĐỐI CHIẾU VỚI `docs/be-tauri-plan.md`](#4-đối-chiếu-với-docsbe-tauri-planmd)
   - 4.1 Các điểm hoàn toàn phù hợp và cần phát huy
   - 4.2 Các điểm BẮT BUỘC CẬP NHẬT trong Plan (Critical Updates)
5. [DANH MỤC PHƯƠNG ÁN KHÔNG PHÙ HỢP (NO-MATCH CATALOG)](#5-danh-mục-phương-án-không-phù-hợp-no-match-catalog)
6. [KẾT LUẬN & KIẾN NGHỊ HÀNH ĐỘNG TIẾP THEO](#6-kết-luận--kiến-nghị-hành-động-tiếp-theo)

---

## 1. TỔNG QUAN & BỐI CẢNH KIẾN TRÚC

Dự án `merlink-harness-ui` đang trong giai đoạn chuyển đổi căn bản: từ một giao diện Web SPA thông thường trở thành một **Desktop AI Agent Harness** đầy đủ tính năng được đóng gói bằng **Tauri v2**. Trong mô hình kiến trúc mới:
- **Rust (Backend nhúng):** Đóng vai trò Backend-Authoritative tuyệt đối. Toàn bộ Agent Runtime, vòng lặp điều phối tác vụ (Run Loop), quản lý phiên chat (`sessions`, `messages`), thực thi công cụ (Tool Execution & Approval), quản lý tệp đính kèm, lưu trữ API keys trong OS Keychain, và lưu trữ dữ liệu nghiệp vụ tiểu thuyết (Novel Writing Domain) được đảm nhiệm toàn vẹn bởi Rust.
- **React (Frontend UI):** Hoàn toàn là một tầng trình diễn mỏng (Thin Presentation Layer). Frontend không chứa logic nghiệp vụ, không trực tiếp gọi API LLM bên ngoài, không tự quản lý trạng thái giao dịch; Frontend chỉ hiển thị luồng sự kiện streaming từ Rust (`RunEvent`), nhận tương tác người dùng và gọi các Command có kiểu dữ liệu an toàn (Type-safe IPC).

Để hiện thực hóa kiến trúc này, 4 trụ cột kỹ thuật cốt lõi cần được thiết kế chính xác và đồng bộ:
1. **Giao tiếp IPC Streaming độ trễ cực thấp:** Cơ chế truyền tải hàng trăm token/giây từ LLM cùng các thông báo trung gian (tool calls, diff duyệt, kết quả verify) từ Rust sang React mà không gây tắc nghẽn giao diện.
2. **An toàn kiểu dữ liệu 100% (End-to-End Type Safety):** Tự động sinh mã TypeScript từ Rust struct/enum mà không cần viết tay thủ công, loại bỏ hoàn toàn nguy cơ sai lệch kiểu (type drift) hay lỗi cú pháp chuỗi invoke (`invoke("some_cmd")`).
3. **Cơ sở dữ liệu nhúng SQLite vững chắc với `sqlx`:** Đảm bảo khả năng đọc/ghi đồng thời không xung đột, cấu hình PRAGMA tối ưu cho ổ đĩa máy tính để bàn, nhúng migration vào mã nhị phân, và cơ chế bảo vệ giao dịch (Commit Guard) chống ghi đè dữ liệu rác khi Agent bị hủy.
4. **Đồng bộ mô hình quan hệ miền dữ liệu với UI:** Đồng bộ hóa cấu trúc bảng thực tế từ `docs/db.dbml` — đặc biệt là quan hệ Nhiều-Nhiều (N-N) mới giữa dự án và nhân vật thông qua bảng liên kết `project_characters`, đồng thời thiết lập cơ chế giải phóng/cập nhật cache (Cache Invalidation) tức thì trên Frontend thông qua TanStack Query.

---

## 2. KHẢO SÁT CHUYÊN SÂU CÔNG NGHỆ TAURI V2 IPC & SQLITE TRONG RUST

### 2.1 So sánh toàn diện: `invoke` vs `app.emit` vs `tauri::ipc::Channel<T>`

Theo tài liệu chính thức từ đội ngũ phát triển Tauri v2 (`v2.tauri.app/develop/calling-frontend`):

| Tiêu chí so sánh | `invoke` (Truyền thống) | `app.emit` / `window.emit` (Global Events) | `tauri::ipc::Channel<T>` (Tauri v2 Mới) |
|---|---|---|---|
| **Mô hình truyền thông** | Request - Single Response (1 request $\to$ 1 response) | Publish - Subscribe toàn cục (1 publisher $\to$ N subscribers) | Direct Stream (1 command $\to$ N streamed chunks $\to$ Finish) |
| **Phù hợp cho Streaming LLM?** | ❌ Không (chỉ trả về một lần sau khi xong toàn bộ text) | ⚠️ Có thể, nhưng tồn tại nhiều nhược điểm nghiêm trọng | ✅ **Tối ưu tuyệt đối** cho token streaming, tool progress |
| **Bảo đảm thứ tự (Ordering)** | N/A (1 giá trị duy nhất) | ❌ **Không bảo đảm** nếu listener là async function và emit dồn dập | ✅ **Nghiêm ngặt**: Channels được thiết kế để bảo đảm thứ tự gói tin |
| **Phạm vi cách ly (Isolation)** | Tốt (gắn với Promise cụ thể của lần gọi) | ❌ Rò rỉ sự kiện giữa các window/session nếu không lọc thủ công | ✅ **Hoàn toàn cách ly** theo từng lần gọi hàm cụ thể |
| **Quản lý tài nguyên (Lifecycle)** | Tự động giải phóng khi Promise resolve/reject | ❌ Cạm bẫy `unlisten`: trả về Promise, nếu gọi trước khi resolve sẽ không huỷ được; dễ leak bộ nhớ | ✅ Tự động hủy khi Channel đóng hoặc component unmount |
| **Overhead tuần tự hóa (Serialization)** | Trung bình (qua JSON bridge của Webview) | Cao (dưới nắp máy dùng `eval` JS toàn cục, serialize JSON string) | **Thấp nhất**: Direct callback dispatch theo ID nội bộ |

#### Cạm bẫy của Event System truyền thống trong Tauri:
- **Async listeners & mất thứ tự gói tin:** Tài liệu Tauri nêu rõ: nếu frontend listener là một hàm async và backend gọi `emit` liên tục (như khi stream token LLM), các sự kiện có thể hoàn thành xử lý sai thứ tự (`out of order`), dẫn đến hiện tượng xáo trộn chữ trên màn hình chat.
- **Rò rỉ bộ nhớ từ `unlisten` Promise:** Trên Frontend, hàm `listen()` trả về một `Promise<UnlistenFn>`. Nếu component unmount nhanh trước khi Promise resolve và gọi `unlisten()`, trình duyệt sẽ không gỡ bỏ được handler, gây rò rỉ listener tích luỹ qua các phiên chat.
- **`tauri::ipc::Channel` là lời giải hoàn hảo:** Channel truyền dữ liệu trực tiếp đến callback đã đăng ký lúc invoke, bảo đảm thứ tự tuần tự tuyệt đối, không broadcast ra ngoài cửa sổ khác và tự động dọn dẹp khi stream kết thúc.

---

### 2.2 Type-Safe Bindings với `tauri-specta` v2 (RC-27)

Tài liệu chính thức từ `specta.dev/docs/tauri-specta` xác nhận sự sẵn sàng của `tauri-specta` v2 cho Tauri v2:
- Cấu hình dependencies khuyến nghị trong `src-tauri/Cargo.toml`:
  ```toml
  specta = { version = "=2.0.0-rc.27", features = ["derive", "function"] }
  specta-typescript = "0.0.14"
  tauri-specta = { version = "=2.0.0-rc.27", features = ["derive", "typescript"] }
  serde = { version = "1.0", features = ["derive"] }
  ```
- **Hỗ trợ Native `tauri::ipc::Channel<T>`:** `tauri-specta` v2 hỗ trợ trực tiếp `tauri::ipc::Channel<T>` như một tham số trực tiếp của lệnh command (direct command argument).
- **Hỗ trợ Typed Events:** Bên cạnh command, `tauri-specta` v2 cung cấp trait `tauri_specta::Event`, cho phép derive trên Struct để phát và lắng nghe sự kiện toàn cục có kiểu dữ liệu an toàn (`events.entityChanged.listen(...)`).
- **Chế độ kiểm soát lỗi (`ErrorHandlingMode`):** Hỗ trợ cấu hình `.error_handling(tauri_specta::ErrorHandlingMode::Result)` để sinh ra kiểu trả về dạng `{ status: "ok", data: T } | { status: "error", error: E }` trên TypeScript, giúp Frontend xử lý lỗi dạng functional value thay vì rơi vào exception không được định kiểu.

---

### 2.3 Phân tích chuyên sâu Concurrency & Locking của SQLite với `sqlx`

Thực nghiệm và phân tích từ cộng đồng kỹ thuật Rust (đặc biệt là bài viết *PSA: Your SQLite Connection Pool Might Be Ruining Your Write Performance*, 2026) đã chỉ ra một cạm bẫy chí tử khi kết hợp Async Rust (`sqlx` + Tokio) với SQLite:

#### Bản chất Single-Writer của SQLite:
- SQLite dù chạy ở chế độ WAL (Write-Ahead Logging) cho phép nhiều Readers chạy song song với Writer, nhưng **tại bất kỳ thời điểm nào chỉ có DUY NHẤT 1 Writer được phép nắm giữ khoá EXCLUSIVE lock trên file DB**.
- Trong `sqlx`, khi mở một transaction ghi thông thường và thực thi câu lệnh SQL có chứa điểm `.await`, tác vụ hiện tại sẽ yield quyền điều khiển lại cho Tokio runtime.
- Nếu Tokio runtime lúc này lập lịch cho một tác vụ ghi khác cũng cố gắng acquire write lock, tác vụ đó sẽ bị block và phải chờ ở `busy_timeout` (5000ms).
- Nếu ứng dụng sử dụng một `SqlitePool` dùng chung với nhiều connections (vd: 20-50 connections) cho cả đọc lẫn ghi dồn dập, hiện tượng **Lock Starvation & Contention** sẽ xảy ra ở tầng OS file lock, khiến hiệu năng ghi giảm sút tới 20 lần (từ 60,000 ops/s xuống chỉ còn 2,500 ops/s, độ trễ P99 vọt lên hàng giây!).

#### Kiến trúc Giải pháp Tối ưu: Mô hình Dual-Pool (Writer Pool + Reader Pool)
Để giải quyết triệt để vấn đề này trên desktop app:
1. **`writer_pool` (Single-Writer Connection):** Thiết lập `max_connections(1)`. Mọi thao tác ghi dữ liệu (Tool mutations, Chat messages, Agent traces) đều đi qua pool này. Do pool chỉ có 1 connection, các transaction ghi sẽ tự động xếp hàng (queue) nhẹ nhàng ở tầng bộ nhớ của ứng dụng thay vì tranh chấp kịch liệt ở tầng file lock của SQLite.
2. **`reader_pool` (Concurrent Reader Pool):** Thiết lập `max_connections(8 - 16)`, kích hoạt `read_only(true)`. Phục vụ tất cả các truy vấn hiển thị dữ liệu của UI và TanStack Query. Nhờ WAL mode, các truy vấn này diễn ra tức thì, hoàn toàn không bị ảnh hưởng bởi tiến trình ghi của Agent.

---

## 3. THIẾT KẾ CHI TIẾT HỆ THỐNG

### 3.1 Mô hình IPC Streaming với `tauri::ipc::Channel<RunEvent>`

Trong Merlink Harness, một lượt chạy của Agent (Agent Run) là một quy trình đa bước phức tạp: nạp context $\to$ gọi LLM $\to$ stream token $\to$ phát hiện tool call $\to$ kiểm tra quyền / yêu cầu duyệt $\to$ thực thi tool $\to$ verify kết quả $\to$ tiếp tục vòng lặp cho đến khi hoàn thành hoặc chạm ngưỡng budget.

#### Định nghĩa Enums & Structs cho `RunEvent` (Rust):
```rust
// crates/domain/src/run.rs hoặc src-tauri/src/events.rs
use serde::{Deserialize, Serialize};
use specta::Type;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(tag = "kind", content = "data")]
pub enum RunEvent {
    RunStarted {
        run_id: String,
        session_id: String,
        mode: String, // "explore" | "edit"
        max_steps: u32,
    },
    ContextBuilt {
        run_id: String,
        usage: ContextUsageDto,
    },
    Token {
        run_id: String,
        text: String,
    },
    ReasoningToken {
        run_id: String,
        text: String,
    },
    ToolCallProposed {
        run_id: String,
        call_id: String,
        tool_name: String,
        args_json: String,
    },
    ApprovalRequired {
        run_id: String,
        approval_id: String,
        calls: Vec<ToolApprovalCallDto>,
    },
    ToolCallStarted {
        run_id: String,
        call_id: String,
        tool_name: String,
    },
    ToolCallFinished {
        run_id: String,
        call_id: String,
        is_ok: bool,
        output_json: String,
    },
    VerifyResult {
        run_id: String,
        call_id: String,
        verdict: String, // "pass" | "retry" | "block"
        feedback: Option<String>,
    },
    ModeChangeRequested {
        run_id: String,
        reason: String,
    },
    ModeChanged {
        run_id: String,
        new_mode: String,
    },
    BudgetWarning {
        run_id: String,
        kind: String, // "step" | "token" | "duration"
        used: u64,
        limit: u64,
    },
    StreamReset {
        run_id: String,
        reason: String,
    },
    RunFinished {
        run_id: String,
        status: String, // "completed" | "cancelled" | "blocked"
        total_tokens_in: u32,
        total_tokens_out: u32,
    },
    RunFailed {
        run_id: String,
        error_code: String,
        message: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct ContextUsageDto {
    pub system_tokens: u32,
    pub memory_tokens: u32,
    pub history_tokens: u32,
    pub attachment_tokens: u32,
    pub tool_schema_tokens: u32,
    pub total_input_tokens: u32,
    pub context_limit: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct ToolApprovalCallDto {
    pub call_id: String,
    pub tool_name: String,
    pub diff_preview: Option<String>,
    pub args_json: String,
}
```

#### Triển khai Command trong Rust (`src-tauri/src/commands/chat.rs`):
```rust
use tauri::ipc::Channel;
use crate::error::AppResult;
use crate::events::RunEvent;
use crate::state::AppState;

#[tauri::command]
#[specta::specta]
pub async fn chat_send(
    session_id: String,
    message: String,
    on_event: Channel<RunEvent>,
    state: tauri::State<'_, AppState>,
) -> AppResult<String> {
    // 1. Kiểm tra session hiện tại có đang chạy run nào không (Scheduler Semaphore & 1 run/session)
    let run_id = state.agent_scheduler.allocate_run(&session_id).await?;

    // 2. Clone state & channel đưa vào Tokio background task
    let runtime = state.agent_runtime.clone();
    let cancellation_token = state.agent_scheduler.get_cancellation_token(&run_id).await?;

    tokio::spawn(async move {
        let result = runtime.execute_run(
            &run_id,
            &session_id,
            &message,
            on_event.clone(),
            cancellation_token,
        ).await;

        if let Err(err) = result {
            let _ = on_event.send(RunEvent::RunFailed {
                run_id: run_id.clone(),
                error_code: err.code(),
                message: err.to_string(),
            });
        }
    });

    // Trả về run_id ngay lập tức để FE theo dõi
    Ok(run_id)
}
```

#### Xử lý Backpressure và Buffering trên Frontend React (`src/features/chat/api/useRunStream.ts`):
LLM có thể xuất token với tốc độ rất cao (50-100 tokens/s). Nếu mỗi token bắn từ Channel đều gọi `setState` ngay lập tức, React sẽ bị giật khung hình (frame drops) và tiêu tốn CPU.
Giải pháp tối ưu là sử dụng **`requestAnimationFrame` Token Buffering**:
```ts
// src/shared/lib/ipc/channel.ts hoặc features/chat/hooks/useRunStream.ts
import { Channel } from "@tauri-apps/api/core";
import { useRef, useCallback } from "react";
import type { RunEvent } from "@/shared/lib/ipc/bindings";

export function useRunEventChannel(onEventDispatched: (event: RunEvent) => void) {
  const tokenBufferRef = useRef<string>("");
  const rafIdRef = useRef<number | null>(null);

  const flushTokenBuffer = useCallback(() => {
    if (tokenBufferRef.current.length > 0) {
      onEventDispatched({
        kind: "Token",
        data: { run_id: "", text: tokenBufferRef.current },
      });
      tokenBufferRef.current = "";
    }
    rafIdRef.current = null;
  }, [onEventDispatched]);

  const createChannel = useCallback(() => {
    const channel = new Channel<RunEvent>();
    
    channel.onmessage = (event: RunEvent) => {
      if (event.kind === "Token") {
        tokenBufferRef.current += event.data.text;
        if (!rafIdRef.current) {
          rafIdRef.current = requestAnimationFrame(flushTokenBuffer);
        }
      } else {
        // Đối với các event cấu trúc quan trọng (ToolCall, ApprovalRequired,...), flush buffer ngay
        if (rafIdRef.current) {
          cancelAnimationFrame(rafIdRef.current);
          flushTokenBuffer();
        }
        onEventDispatched(event);
      }
    };

    return channel;
  }, [flushTokenBuffer, onEventDispatched]);

  return { createChannel };
}
```

---

### 3.2 Khung sinh mã Type-Safe với `tauri-specta` v2 & Quản trị Lỗi `AppError`

#### Kiến trúc Xuất Bindings tập trung (`src-tauri/src/lib.rs`):
```rust
use specta_typescript::Typescript;
use tauri_specta::{collect_commands, collect_events, Builder};
use crate::events::EntityChanged;

pub fn create_specta_builder() -> Builder<tauri::Wry> {
    Builder::<tauri::Wry>::new()
        .commands(collect_commands![
            // Session & Chat
            commands::chat::chat_send,
            commands::chat::chat_cancel,
            commands::session::session_list,
            commands::session::session_create,
            // Approval & Mode
            commands::run::approval_decide,
            commands::run::mode_switch,
            // Projects & Characters (Business)
            commands::project::project_list,
            commands::project::project_get,
            commands::character::character_list_by_project,
            commands::character::character_create_for_project,
            // Providers & Models
            commands::provider::provider_list,
            commands::provider::provider_test,
        ])
        .events(collect_events![EntityChanged])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn export_typescript_bindings() {
        let builder = create_specta_builder();
        builder
            .export(
                Typescript::default()
                    .bigint(specta_typescript::BigIntExportBehavior::Number)
                    .formatter(specta_typescript::formatter::prettier),
                "../src/shared/lib/ipc/bindings.ts",
            )
            .expect("Failed to export TypeScript bindings");
    }
}
```

#### Thiết kế `AppError` chuẩn cho IPC Serialization:
Toàn bộ lỗi nghiệp vụ và hệ thống từ Rust được chuyển đổi thành cấu trúc chuẩn hoá để React hiển thị thông báo lỗi thân thiện:
```rust
// src-tauri/src/error.rs
use serde::{Deserialize, Serialize};
use specta::Type;
use thiserror::Error;

#[derive(Debug, Error, Serialize, Deserialize, Type)]
#[serde(tag = "type", content = "payload")]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(String),

    #[error("Session {0} is currently active")]
    RunActive(String),

    #[error("Run was cancelled by user")]
    RunCancelled,

    #[error("Context window overflowed (limit: {limit}, required: {required})")]
    ContextOverflow { limit: u32, required: u32 },

    #[error("Tool validation failed for {tool}: {reason}")]
    ToolValidationFailed { tool: String, reason: String },

    #[error("Unauthorized action: {0}")]
    Unauthorized(String),

    #[error("Resource not found: {0}")]
    NotFound(String),
}

pub type AppResult<T> = Result<T, AppError>;
```

---

### 3.3 Kiến trúc SQLite Cục bộ Tối ưu: Mô hình Dual-Pool (Writer/Reader) & PRAGMA Tuning

#### Cấu hình PRAGMA chuẩn cho Desktop App:
SQLite trong môi trường Desktop nếu chỉ mở với thiết lập mặc định sẽ rất dễ gặp lỗi khoá `database is locked`, hiệu năng ghi kém và không ràng buộc khóa ngoại. Cần cấu hình nghiêm ngặt 5 chỉ thị sau ngay khi thiết lập pool:

1. **`PRAGMA journal_mode = WAL;` (Write-Ahead Logging):**
   - Thay vì ghi đè trực tiếp lên cơ sở dữ liệu và dùng file rollback journal, SQLite ghi tất cả thao tác sửa đổi vào file `-wal`.
   - **Tác dụng quyết định:** Các tiến trình Đọc (Readers) hoàn toàn **không chặn** tiến trình Ghi (Writer), và Writer **không chặn** Readers. Frontend có thể truy vấn danh sách nhân vật, chương hồi một cách trơn tru trong khi Agent đang ghi hàng chục bước thực thi `run_steps`.
2. **`PRAGMA foreign_keys = ON;`:**
   - Mặc định SQLite TẮT kiểm tra khoá ngoại vì lý do tương thích ngược lịch sử! Bắt buộc phải kích hoạt `ON` trên từng kết nối để đảm bảo tính toàn vẹn quan hệ (vd: xoá project cascade xoá sessions, kiểm tra id tồn tại trong `project_characters`).
3. **`PRAGMA busy_timeout = 5000;` (5000ms):**
   - Khi có 2 tiến trình ghi cạnh tranh, thay vì trả về lỗi `SQLITE_BUSY` ngay lập tức làm sập command, SQLite sẽ tự động chờ (sleep-retry) lên tới 5 giây để đợi kết nối kia hoàn thành commit.
4. **`PRAGMA synchronous = NORMAL;`:**
   - Trong chế độ WAL, thiết lập `synchronous = NORMAL` đảm bảo dữ liệu toàn vẹn ngay cả khi ứng dụng bị tắt đột ngột (crash/kill process), đồng thời giảm số lần gọi hàm `fsync` xuống đĩa cứng từ mỗi transaction xuống chỉ khi checkpoint WAL. Tốc độ ghi tăng gấp 10-20 lần so với chế độ `FULL`.
5. **`PRAGMA cache_size = -64000;` & `PRAGMA temp_store = MEMORY;`:**
   - Cấp phát 64MB bộ nhớ RAM làm trang bộ nhớ đệm (cache), lưu các bảng tạm và chỉ mục tạm thời trong RAM thay vì ghi ra disk.

#### Cấu hình Dual-Pool Database (`src-tauri/crates/infra/src/db/pool.rs`):
```rust
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous};
use sqlx::SqlitePool;
use std::path::Path;
use std::time::Duration;
use crate::error::AppResult;

pub struct DbContext {
    pub writer: SqlitePool,
    pub reader: SqlitePool,
}

pub async fn init_sqlite_db(db_path: &Path) -> AppResult<DbContext> {
    // 1. Cấu hình kết nối cơ sở
    let base_options = SqliteConnectOptions::new()
        .filename(db_path)
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal)
        .synchronous(SqliteSynchronous::Normal)
        .busy_timeout(Duration::from_millis(5000))
        .foreign_keys(true)
        .pragma("cache_size", "-64000")
        .pragma("temp_store", "MEMORY");

    // 2. Writer Pool: Cố định 1 kết nối duy nhất để tuần tự hoá các tác vụ ghi ở tầng ứng dụng,
    // loại bỏ triệt để lock contention trên SQLite file.
    let writer = SqlitePoolOptions::new()
        .max_connections(1)
        .acquire_timeout(Duration::from_secs(10))
        .connect_with(base_options.clone())
        .await?;

    // 3. Reader Pool: Nhiều kết nối đọc đồng thời cho UI, cấu hình read_only(true)
    let reader_options = base_options.clone().read_only(true);
    let reader = SqlitePoolOptions::new()
        .max_connections(10)
        .min_connections(2)
        .acquire_timeout(Duration::from_secs(5))
        .connect_with(reader_options)
        .await?;

    Ok(DbContext { writer, reader })
}
```

#### Chiến lược Migration nhúng binary với `sqlx::migrate!` (`crates/infra/src/db/migrate.rs`):
Thay vì phụ thuộc vào thư mục migrations bên ngoài, ta nhúng tĩnh vào file nhị phân bằng macro `sqlx::migrate!`:
```rust
use sqlx::SqlitePool;
use crate::error::AppResult;

pub async fn run_migrations(writer_pool: &SqlitePool) -> AppResult<()> {
    tracing::info!("Starting database migrations on writer connection...");
    
    // Nhúng toàn bộ thư mục migrations vào binary tại compile time
    sqlx::migrate!("../../migrations")
        .run(writer_pool)
        .await
        .map_err(|e| crate::error::AppError::Database(format!("Migration failed: {}", e)))?;

    tracing::info!("Database migrations applied successfully.");
    Ok(())
}
```

**Kế hoạch phân bổ tệp migration:**
- `migrations/0001_core.sql`: Chứa các bảng nghiệp vụ tiểu thuyết chuẩn theo `docs/db.dbml` (`projects`, `arcs`, `chapters`, `scenes`, `characters`, `relationships`, `world_settings`, `lore_entries`, `themes`,...).
- `migrations/0002_scope.sql`: Cập nhật cấu trúc bảng liên kết `project_characters` (N-N), các chỉ mục phục vụ phân vùng dữ liệu theo dự án (`indexes { (project_id, character_id) [unique] }`).
- `migrations/0003_harness.sql`: Bảng dữ liệu Agent Harness (`ai_providers`, `ai_models`, `prompt_templates`, `prompt_bindings`, `context_memories`, `attachments`, `agent_runs`, `run_steps`, `tool_policies`, `app_settings`) kèm bảng ảo FTS5 `context_memories_fts`.

#### Transaction Isolation & Cơ chế Commit Guard:
Trong quá trình Agent thực thi, một Tool có thể mất vài giây để xử lý hoặc chờ người dùng nhấn nút "Duyệt" (Approval). Trong khoảng thời gian đó, người dùng có thể nhấn nút "Hủy run" (Cancel) hoặc đóng ứng dụng. Nếu Tool vẫn tiếp tục ghi dữ liệu vào SQLite sau khi run đã bị hủy, cơ sở dữ liệu sẽ rơi vào trạng thái rác/hỗn loạn (Zombie Writes).

**Mô hình Commit Guard Pattern trong `crates/agent/src/tools/executor.rs`:**
```rust
use sqlx::{Sqlite, Transaction};
use crate::error::{AppError, AppResult};

pub async fn execute_tool_with_commit_guard<F, Fut, T>(
    writer_pool: &sqlx::SqlitePool,
    run_id: &str,
    tool_action: F,
) -> AppResult<T>
where
    F: FnOnce(&mut Transaction<'_, Sqlite>) -> Fut,
    Fut: std::future::Future<Output = AppResult<T>>,
{
    // 1. Mở transaction trên writer connection
    let mut tx = writer_pool.begin().await?;

    // 2. Thực thi thao tác nghiệp vụ của Tool (chưa commit)
    let result = tool_action(&mut tx).await?;

    // 3. COMMIT GUARD: Kiểm tra xem agent_runs hiện tại có còn hợp lệ (Running / AwaitingApproval) không
    let status_row: (String,) = sqlx::query_as(
        "SELECT status FROM agent_runs WHERE id = ?"
    )
    .bind(run_id)
    .fetch_one(&mut *tx)
    .await?;

    if status_row.0 != "running" && status_row.0 != "awaiting_approval" {
        // Run đã bị cancel, timeout, hoặc fail giữa chừng -> TỰ ĐỘNG ROLLBACK
        tx.rollback().await?;
        return Err(AppError::RunCancelled);
    }

    // 4. Nếu trạng thái hoàn toàn hợp lệ -> Thực hiện COMMIT an toàn
    tx.commit().await?;
    Ok(result)
}
```

---

### 3.4 Đồng bộ Mô hình Dữ liệu DBML và Frontend Cache Invalidation

#### Cập nhật quan hệ N-N `project_characters` thay thế 1-N cũ:
Trong tài liệu kế hoạch cũ (`be-tauri-plan.md` mục 5.3 & 5.4), nhóm thiết kế dự định ép buộc thêm cột `project_id NOT NULL` vào trực tiếp bảng `characters` (quan hệ 1-N cứng). Tuy nhiên, **tệp mô hình dữ liệu chính thức mới nhất `docs/db.dbml` (dòng 657-673)** đã loại bỏ cách tiếp cận này và định nghĩa bảng liên kết Nhiều-Nhiều:
```dbml
Table characters {
    id uuid [pk, note: 'UUIDv7']
    code varchar [unique]
    display_name text
    birth timestamp
    species varchar
    biological_sex gender
    gender_identity gender
    gender_expression gender
    deleted_at timestamp
}

Table project_characters {
    id uuid [pk, note: 'UUIDv7']
    project_id uuid [ref: > projects.id, not null, note: 'Dự án liên kết']
    character_id uuid [ref: > characters.id, not null, note: 'Nhân vật tham gia dự án']
    role varchar [note: 'Vai trò: PROTAGONIST, ANTAGONIST, SUPPORTING, CAMEO...']
    notes text [note: 'Ghi chú thiết lập riêng của nhân vật trong phạm vi dự án này']
    created_at timestamp
    deleted_at timestamp

    indexes {
        (project_id, character_id) [unique]
        project_id
        character_id
        deleted_at
    }
}
```

**Phân tích tác động kiến trúc của sự thay đổi này:**
1. **Tách biệt Bản sắc Nhân vật (Identity) và Vai trò trong Dự án (Contextual Role):**
   - Bảng `characters` đóng vai trò là "Thư viện nhân vật gốc" (Master Character Library) chứa các đặc tính cốt lõi không đổi: giới tính sinh học, diện mạo (`character_appearances`), năng lực nền tảng (`character_competences`).
   - Bảng `project_characters` chứa các thuộc tính ngữ cảnh gắn liền với một dự án truyện cụ thể: `role` (nhân vật chính, phản diện, khách mời), `notes` (thiết lập riêng cho tập này).
2. **Loại bỏ nhu cầu "Clone nhân vật" phức tạp:** Người dùng có thể tái sử dụng một nhân vật qua nhiều bộ truyện/phần tiếp theo mà không cần nhân bản dòng dữ liệu, tránh phân mảnh dữ liệu.
3. **Thay đổi trong `ToolContext` và `ScopeResolver` (Rust):**
   - Khi Agent thực hiện các thao tác tìm kiếm hoặc chỉnh sửa nhân vật trong một Project, câu lệnh SQL bắt buộc phải JOIN qua `project_characters`:
     ```sql
     SELECT c.*, pc.role, pc.notes 
     FROM characters c
     JOIN project_characters pc ON c.id = pc.character_id
     WHERE pc.project_id = ? 
       AND pc.deleted_at IS NULL 
       AND c.deleted_at IS NULL;
     ```
   - Khi Tool tạo một nhân vật mới trong ngữ cảnh của Project: Trong cùng 1 transaction, Tool tạo bản ghi trong `characters`, sau đó tự động tạo bản ghi liên kết tương ứng trong `project_characters`.

#### Cơ chế Cache Invalidation với TanStack Query và Typed Events (`events.entityChanged`):
Để giữ cho Frontend luôn cập nhật ngay khi Agent hoặc người dùng chỉnh sửa dữ liệu mà không cần tải lại trang (reload) hay liên tục polling:

```
[Tool Execution trong Rust] 
       │
       ▼ ghi DB thành công
[EventSink.emit(EntityChanged)]
       │
       ▼ broadcast qua tauri-specta typed event
[events.entityChanged.listen trên React Webview]
       │
       ▼ tự động kích hoạt
[TanStack Query: queryClient.invalidateQueries({ queryKey })]
       │
       ▼ fetch lại dữ liệu qua typed command
[React UI Render dữ liệu mới]
```

**Triển khai trên Rust (`src-tauri/src/events.rs`):**
```rust
use serde::{Deserialize, Serialize};
use specta::Type;
use tauri_specta::Event;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, Type, Event)]
pub struct EntityChanged {
    pub entity: String, // "project" | "character" | "scene" | "chapter" | "arc"
    pub project_id: Option<Uuid>,
    pub ids: Vec<Uuid>,
    pub op: String, // "created" | "updated" | "deleted"
}

// Khi Tool hoàn thành ghi dữ liệu:
EntityChanged {
    entity: "character".into(),
    project_id: Some(project_id),
    ids: vec![character_id],
    op: "updated".into(),
}.emit(app_handle)?;
```

**Triển khai trên Frontend React (`src/shared/lib/ipc/useEntitySync.ts`):**
```ts
import { useEffect } from "react";
import { useQueryClient } from "@tanstack/react-query";
import { events } from "@/shared/lib/ipc/bindings";

export function useEntitySync() {
  const queryClient = useQueryClient();

  useEffect(() => {
    let unlistenFn: (() => void) | null = null;

    events.entityChanged.listen((event) => {
      const { entity, project_id } = event.payload;

      // Invalidate các query cụ thể theo mô hình phân cấp queryKey
      if (project_id) {
        queryClient.invalidateQueries({ queryKey: [entity, project_id] });
        queryClient.invalidateQueries({ queryKey: ["projects", project_id, entity] });
      } else {
        queryClient.invalidateQueries({ queryKey: [entity] });
      }
    }).then((unlisten) => {
      unlistenFn = unlisten;
    });

    return () => {
      if (unlistenFn) {
        unlistenFn();
      }
    };
  }, [queryClient]);
}
```

---

## 4. ĐỐI CHIẾU VỚI `docs/be-tauri-plan.md`

Tiến hành rà soát chi tiết các mục 1, 2, 3, 4, 5, 12, 13 trong bản kế hoạch hiện tại `docs/be-tauri-plan.md`:

### 4.1 Các điểm hoàn toàn phù hợp và cần phát huy
1. **Phân lớp kiến trúc một chiều và Cấu trúc Workspace 4 Crate (Mục 1, 3, 4):**
   - Việc chia `src-tauri` thành 4 crate nội bộ (`domain`, `agent`, `llm`, `infra`) và thiết lập hàng rào phụ thuộc bằng `Cargo.toml` là quyết định xuất sắc, bảo đảm `domain` thuần khiết, không bị phụ thuộc ngược vào `tauri` hay `sqlx`.
2. **Lựa chọn `sqlx` + SQLite Cục bộ (Mục 2):**
   - Đã chốt chính xác `sqlx` (async) kết hợp `sqlx::migrate!`. Hoàn toàn đồng thuận với việc không dùng ORM cồng kềnh.
3. **Mô hình Streaming bằng `tauri::ipc::Channel` (Mục 1, 12):**
   - Quyết định sử dụng `Channel<RunEvent>` thay cho `app.emit` hoàn toàn phù hợp với thực tiễn tối ưu hiệu năng của Tauri v2.
4. **FE State Management (Mục 2, 13):**
   - Phân định ranh giới rành mạch: TanStack Query quản lý Server State từ DB, Zustand quản lý Client UI State cục bộ (trạng thái đóng mở modal, buffer streaming).

### 4.2 Các điểm BẮT BUỘC CẬP NHẬT trong Plan (Critical Updates)

| Vị trí trong `be-tauri-plan.md` | Nội dung hiện tại trong Plan | Điểm bất cập / Mâu thuẫn | Giải pháp cập nhật chuẩn xác |
|---|---|---|---|
| **Mục 5.3 (Bảng hiện có)** | Ghi: `characters: + project_id NOT NULL (sau backfill) - Blocker Phase 0` | Mâu thuẫn với `docs/db.dbml`. `db.dbml` đã chuyển sang N-N qua bảng `project_characters`. Cột `characters.project_id` không tồn tại trong thiết kế mới. | **Sửa bỏ yêu cầu thêm cột `project_id` vào `characters`.** Giữ bảng `characters` độc lập và định nghĩa bảng liên kết `project_characters`. |
| **Mục 5.4 (Scope theo project & backfill)** | Quy định: Nhân vật thuộc về một project duy nhất. Nếu xuất hiện ở nhiều project thì nhân bản row. | Vi phạm nguyên tắc chuẩn hóa dữ liệu, làm mất tính nhất quán khi một nhân vật xuất hiện trong chuỗi tác phẩm (series/franchise). | **Cập nhật quy tắc Scope:** Nhân vật thuộc về project thông qua `project_characters`. `ScopeResolver::owns_character(project_id, char_id)` kiểm tra sự tồn tại trong `project_characters`. |
| **Mục 15 (#1) & Mục 16 (Phase 0 Roadmap)** | Liệt kê việc thiếu `project_id` trong `characters` là "Blocker Phase 0" cần backfill `0002_scope.sql`. | Gây hiểu lầm cho đội ngũ phát triển Backend khi triển khai migration. | **Cập nhật task Phase 0:** Migration `0002_scope.sql` tập trung tạo bảng `project_characters` và các index liên kết `(project_id, character_id)`, không can thiệp thêm cột vào bảng `characters`. |
| **Mục 8 & 12 (Tool Signatures & Repo)** | Các tool thao tác nhân vật nhận giả định `character.project_id`. | Câu lệnh SQL sẽ bị lỗi runtime do không có cột `project_id` trong bảng `characters`. | **Cập nhật chữ ký Tool & Repo:** Tool `character_create` nhận `project_id` trong `ToolContext`, thực hiện insert vào cả `characters` và `project_characters` trong cùng một transaction có Commit Guard. |
| **Mục 2 & Mục 4 (Database Pool)** | Chưa đề cập cấu trúc Single-Writer vs Multi-Reader Pool. | Dễ rơi vào bẫy lock contention/starvation khi Agent vừa ghi trace vừa thực thi tool async trên pool dùng chung. | **Bổ sung mô hình Dual-Pool:** `writer` pool (max 1 connection) và `reader` pool (max 10 connections, `read_only=true`). |

---

## 5. DANH MỤC PHƯƠNG ÁN KHÔNG PHÙ HỢP (NO-MATCH CATALOG)

Dưới đây là bảng tổng hợp các phương án kỹ thuật đã được nghiên cứu kỹ lưỡng nhưng **bị loại bỏ (No-Match)**, cần được đưa vào tài liệu `no-match.md` của dự án để ngăn ngừa việc tái đề xuất hoặc đi sai hướng:

```
┌──────────────────────────────────────────────────────────────────────────────────────────────────┐
│                                 DANH MỤC NO-MATCH (BỊ LOẠI BỎ)                                   │
└──────────────────────────────────────────────────────────────────────────────────────────────────┘
```

### 1. Dùng WebSocket / gRPC Local thay vì Tauri IPC
- **Đề xuất:** Mở một máy chủ WebSocket hoặc gRPC cục bộ (localhost port) trên tiến trình Rust để Frontend kết nối truyền nhận dữ liệu.
- **Lý do loại bỏ:**
  - **Lỗ hổng bảo mật nghiêm trọng:** Mở cổng TCP cục bộ khiến ứng dụng dễ bị tấn công bởi các website độc hại chạy trên trình duyệt khác của người dùng (tấn công qua Cross-Site WebSocket Hijacking hoặc quét cổng localhost).
  - **Quản lý vòng đời phức tạp:** Xung đột cổng (port collision) nếu cổng mặc định bị chiếm dụng bởi phần mềm khác; tốn tài nguyên quản lý handshake, ping/pong, reconnect.
  - **Tauri IPC v2 vượt trội:** Tauri IPC chạy trực tiếp trên bộ nhớ (in-memory bridge) giữa Webview và Rust, bảo mật tuyệt đối theo ranh giới tiến trình của hệ điều hành, không có network overhead.

### 2. Dùng `app.emit` / `window.emit` toàn cục thay cho `tauri::ipc::Channel`
- **Đề xuất:** Sử dụng hệ thống phát sự kiện mặc định `app.emit("token", data)` để stream dữ liệu cho chat.
- **Lý do loại bỏ:**
  - **Mất thứ tự sự kiện (Out-of-order execution):** Khi các listener là hàm bất đồng bộ, việc phát sự kiện dồn dập sẽ làm xáo trộn thứ tự render của các token/khối văn bản.
  - **Rò rỉ sự kiện giữa các Session/Window:** Khi mở nhiều cửa sổ hoặc chuyển đổi phiên chat, các sự kiện token từ một session sẽ bị bắn tràn lan cho toàn bộ ứng dụng, đòi hỏi FE phải lọc phức tạp.
  - **Rò rỉ bộ nhớ (Memory Leaks):** Lập trình viên dễ quên hủy đăng ký hàm nghe (`unlisten` là một Promise) khi component bị unmount.
  - **Overhead tuần tự hóa:** Sự kiện toàn cục phải đi qua vòng lặp định tuyến của Tauri Event Loop thay vì gọi thẳng vào callback handler ID như `Channel`.

### 3. Dùng ORM nặng (Diesel hoặc SeaORM) thay cho `sqlx`
- **Đề xuất:** Áp dụng SeaORM hoặc Diesel để sinh code truy vấn hướng đối tượng.
- **Lý do loại bỏ:**
  - **Thời gian biên dịch (Compile Times) tăng vọt:** Các ORM này sử dụng hệ thống macro suy diễn kiểu cực kỳ phức tạp, làm chậm tốc độ build của Rust lên nhiều lần.
  - **Khó tích hợp SQLite PRAGMA và FTS5:** Khó khăn trong việc can thiệp sâu vào các câu lệnh tuning cấp thấp (`WAL`, `busy_timeout`) và bảng ảo tìm kiếm toàn văn FTS5 cho Context Memory của Agent.
  - `sqlx` cung cấp sự cân bằng hoàn hảo: Async native, kiểm soát SQL trực tiếp, tốc độ thực thi tối đa, và hỗ trợ migration nhúng chuẩn mực.

### 4. Chạy SQLite trên Webview bằng WASM (absurd-sql / sqlite-wasm)
- **Đề xuất:** Đưa cơ sở dữ liệu SQLite chạy trực tiếp trên trình duyệt Webview bằng WebAssembly, lưu qua IndexedDB.
- **Lý do loại bỏ:**
  - **Vi phạm nguyên tắc Backend-Authoritative:** Làm mất vai trò làm chủ dữ liệu của Rust.
  - **Hiệu năng kém:** Bị giới hạn bởi bộ nhớ của Webview và tốc độ ghi chậm của IndexedDB.
  - **Bất khả thi cho Agent:** Agent Runtime nằm ở Rust, nếu DB nằm ở Webview thì mỗi khi Agent cần đọc dữ liệu để xây dựng Context hoặc thực thi Tool, nó phải gọi IPC ngược lên Webview để query, gây tắc nghẽn IPC nghiêm trọng.

### 5. Dùng một Connection Pool duy nhất cho cả Đọc và Ghi trong SQLite
- **Đề xuất:** Dùng chung 1 pool 20-50 connections cho tất cả các câu lệnh SELECT và INSERT/UPDATE.
- **Lý do loại bỏ:**
  - Như phân tích thực nghiệm, SQLite chỉ có 1 writer duy nhất. Khi nhiều tác vụ async nắm giữ write transaction qua các điểm `.await`, chúng gây ra tình trạng Lock Starvation kịch liệt ở tầng file lock, khiến tốc độ ghi sụt giảm 20 lần và gây lỗi `database is locked`.
  - Phải tách thành Dual-Pool: 1 Writer duy nhất (max 1 connection) để tuần tự hóa ghi + Pool Readers độc lập.

### 6. Prompt-based Tool Calling trên các Model không hỗ trợ Function Calling
- **Đề xuất:** Cố gắng ép các mô hình ngôn ngữ không hỗ trợ tool calling xuất ra JSON trong prompt và dùng regex để trích xuất tool call.
- **Lý do loại bỏ:**
  - Tỷ lệ lỗi cú pháp (syntax hallucination) cực cao, khó kiểm soát định dạng, gây lãng phí token và độ trễ phản hồi lớn.
  - Đã chốt trong kiến trúc (ADR-005): Chỉ các model có cờ `supports_tools = true` mới được tham gia Agent Mode; các model khác chỉ phục vụ chat thông thường.

---

## 6. KẾT LUẬN & KIẾN NGHỊ HÀNH ĐỘNG TIẾP THEO

### Kết luận đánh giá:
Kiến trúc Backend dự kiến cho `merlink-harness-ui` với sự kết hợp của **Tauri v2 IPC (`tauri::ipc::Channel`)**, **Type-Safe Generation (`tauri-specta` v2 RC-27)**, và **SQLite nhúng tối ưu (`sqlx` + WAL + Dual-Pool + Commit Guard)** là một giải pháp kỹ thuật cực kỳ vững chắc, hiện đại, đảm bảo hiệu năng cao, độ trễ thấp và độ tin cậy tuyệt đối cho một desktop AI harness.

### Các hành động cần thực thi ngay trong Phase 0:
1. **Cập nhật tài liệu `docs/be-tauri-plan.md`:** Đồng bộ hoá nội dung mục 5.3, 5.4 và roadmap Phase 0 theo đúng mô hình bảng liên kết Nhiều-Nhiều `project_characters` từ `docs/db.dbml`.
2. **Khởi tạo Cargo Workspace:** Thiết lập cấu trúc 4 crate (`domain`, `agent`, `llm`, `infra`) kèm cấu hình cấm phụ thuộc chéo trong `Cargo.toml`. Pin chặt các phiên bản `specta = "=2.0.0-rc.27"` và `tauri-specta = "=2.0.0-rc.27"`.
3. **Triển khai Spike IPC & Dual-Pool Database:** Viết bản thử nghiệm mẫu (Spike) kết nối `chat_send` với `tauri::ipc::Channel<RunEvent>`, cấu hình Dual-Pool SQLite và xuất file `bindings.ts` bằng `tauri-specta` v2 để chốt kiến trúc trước khi code diện rộng.
4. **Xây dựng Migration `0001_core.sql` và `0002_scope.sql`:** Dựa trên `docs/db.dbml`, tích hợp macro `sqlx::migrate!` vào hàm khởi tạo ứng dụng.
