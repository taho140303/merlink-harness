# BÁO CÁO NGHIÊN CỨU KIẾN TRÚC BACKEND RUST CHO HARNESS AGENT (SUBAGENT A)
**Chuyên đề:** Runtime Execution, State Machine, Loop điều phối, Budget Control & Error Recovery  
**Dự án:** Merlink Harness UI (Tauri v2 + React Desktop App)  
**Tác giả:** Subagent A (Backend Architecture Specialist)  
**Ngày hoàn thành:** 29/10/2026  

---

## 1. TỔNG QUAN & BỐI CẢNH

### 1.1 Bối cảnh dự án Merlink
Merlink là ứng dụng Desktop chuyên biệt hỗ trợ nhà văn, biên kịch và tác giả sáng tác tiểu thuyết (Creative Writing & Narrative Design). Khác với các ứng dụng chatbot thông thường vốn chỉ sinh văn bản thụ động, Merlink đóng vai trò là một **Agent Harness** hoàn chỉnh:
- Trực tiếp quản lý và thao tác trên cơ sở dữ liệu cốt truyện phức tạp (Arcs, Chapters, Scenes, Scene Versions, Characters, Relationships, Worldbuilding, Lore, Themes, Motifs).
- Duy trì bộ nhớ ngữ cảnh nhiều lớp (Context Memory, Story Directives, Prompt Manifest).
- Thực thi công cụ nghiệp vụ (Business Tools) có cơ chế thẩm định (Verification Pipeline) và phê duyệt từ người dùng (Human-in-the-loop Approval).
- Vận hành hoàn toàn cục bộ trên máy tính người dùng thông qua kiến trúc nhúng **Tauri v2 (Rust Backend) + React (Frontend SPA)** với cơ sở dữ liệu **SQLite**.

### 1.2 Vai trò và Nhiệm vụ của Subagent A
Là kiến trúc sư chịu trách nhiệm cho "trái tim" của hệ thống — **Agent Runtime Engine**, Subagent A tập trung nghiên cứu, thiết kế và tối ưu hóa các thành phần cốt lõi:
1. **Explicit State Machine & Driver Loop**: Cơ chế điều phối vòng lặp thực thi tuần tự, tất định, loại bỏ hoàn toàn các rủi ro của event-choreography.
2. **Run Budget Control**: Quản trị tài nguyên thực thi (bước lặp, số lần gọi công cụ, lượt thử lại, hạn ngạch token, thời gian tính toán thực).
3. **Deep Cancellation Propagation**: Lan truyền tín hiệu hủy bỏ tức thời (`tokio_util::sync::CancellationToken`) từ tầng UI Command xuống Runtime, Tool Execution và HTTP Client Stream.
4. **Orphan Run Reconciliation**: Khắc phục triệt để hiện tượng treo trạng thái (stuck in running) khi ứng dụng desktop bị tắt đột ngột hoặc crash.
5. **Transaction Commit Guard**: Bảo đảm tính toàn vẹn của dữ liệu nghiệp vụ bằng cách chặn đứng các tác vụ ghi muộn sau khi run đã bị hủy hoặc timeout.
6. **Architectural Trade-off**: Đánh giá và chứng minh tính ưu việt của Explicit State Machine Loop so với mô hình Actor-based trong ứng dụng desktop đơn tiến trình.

---

## 2. KHẢO SÁT CÁC DỰ ÁN & FRAMEWORK RUST AGENT THỰC TẾ

Để xây dựng một kiến trúc chuẩn mực và tránh "phát minh lại bánh xe", chúng tôi đã tiến hành khảo sát và mổ xẻ mã nguồn của các framework và dự án agent viết bằng Rust hoặc có mô hình harness tiêu biểu:

```
┌────────────────────────────────────────────────────────────────────────┐
│                      KHẢO SÁT HỆ SINH THÁI RUST AGENT                   │
├──────────────────┬──────────────────┬─────────────────┬────────────────┤
│ Rig Framework    │ LangChain-Rust   │ AutoGPT / Tauri │ Smolagents &   │
│ (0xPlaygrounds)  │ (Abraxas-365)    │ Desktop Pattern │ DeepSeek-Harn. │
├──────────────────┼──────────────────┼─────────────────┼────────────────┤
│ • Sans-I/O Run   │ • Sequential     │ • Single-User   │ • Multi-step   │
│   State Machine  │   Executor Loop  │   In-Process    │   Tool Loop    │
│ • Pure Step Enum │ • CancellationToken│• SQLite State │ • Verification │
│ • Tool Recovery  │ • Streaming MPSC │ • Tauri Channel │ • Defensive    │
│   (Repair/Skip)  │   Event Stream   │   Type-safe IPC │   Audit Trail  │
└──────────────────┴──────────────────┴─────────────────┴────────────────┘
```

### 2.1 Rig Rust Framework (`0xPlaygrounds/rig`)
- **Kho mã nguồn:** [GitHub - 0xPlaygrounds/rig](https://github.com/0xPlaygrounds/rig)
- **Cấu trúc liên quan:** `crates/rig-agent` (đặc biệt là thư mục `src/run/` gồm `mod.rs`, `spec.rs`, `calls.rs`, `streamed.rs`, `policy.rs`).
- **Phân tích kỹ thuật chuyên sâu:**
  - **Mô hình Sans-I/O State Machine (Functional Core, Imperative Shell):** Rig tách biệt hoàn toàn trạng thái nội tại của agent run khỏi các tác vụ I/O. `AgentRun` là một struct thuần dữ liệu, có khả năng serialize/deserialize, sở hữu hàm `next_step() -> Result<AgentRunStep, PromptError>`.
  - Enum `AgentRunStep` chỉ thị cho Driver bên ngoài hành động tiếp theo:
    ```rust
    pub enum AgentRunStep {
        CallModel { prompt: Arc<Message>, history: Vec<Arc<Message>>, turn: usize },
        CallTools { calls: Vec<PendingToolCall> },
        Done(PromptResponse),
    }
    ```
  - **Tool Recovery Mechanism:** Khi LLM sinh ra lệnh gọi tool không hợp lệ hoặc sai schema, Rig không lập tức làm sập run mà cung cấp chính sách phục hồi linh hoạt qua `TurnPolicy` và `InvalidToolCallAction`:
    - `Retry`: Nạp lỗi phản hồi có cấu trúc vào context và yêu cầu model sinh lại.
    - `Repair`: Tự động sửa chữa tên hàm nếu match gần đúng.
    - `Skip`: Bỏ qua lệnh gọi lỗi và tiếp tục các lệnh gọi khác trong batch.
- **Bài học rút ra cho Merlink:** Mô hình Sans-I/O là hình mẫu hoàn hảo để đảm bảo tính tất định và khả năng viết unit test/integration test 100% không phụ thuộc vào mock server phức tạp. Tuy nhiên, Rig chưa tích hợp cơ chế bảo vệ transaction cơ sở dữ liệu (`Commit Guard`) và cơ chế quản lý vòng đời desktop SQLite (`Startup Reconcile`), những thứ Merlink cần tự hiện thực hóa ở tầng `crates/agent`.

### 2.2 LangChain-Rust (`Abraxas-365/langchain-rust`)
- **Kho mã nguồn:** [GitHub - Abraxas-365/langchain-rust](https://github.com/Abraxas-365/langchain-rust)
- **Cấu trúc liên quan:** `src/agent/executor.rs`.
- **Phân tích kỹ thuật chuyên sâu:**
  - **Vòng lặp điều phối tập trung (Sequential Executor Loop):** Khác với sự phức tạp của LangChain Python, bản Rust cài đặt `AgentExecutor` với vòng lặp `while` tuần tự, đón nhận `AgentEvent::Action` hoặc `AgentEvent::Finish`.
  - **CancellationToken tích hợp sâu:** LangChain-Rust sử dụng `tokio_util::sync::CancellationToken` để bọc toàn bộ khối thực thi:
    ```rust
    let run = self.run_inner(input_variables, emitter);
    match &self.cancellation {
        Some(token) => token
            .run_until_cancelled(run)
            .await
            .unwrap_or_else(|| Err(ChainError::AgentError("agent run cancelled".into()))),
        None => run.await,
    }
    ```
  - **Streaming Event Pipeline:** Dùng `async_stream` kết hợp `tokio::sync::mpsc::channel(1)` để kiểm soát backpressure: Consumer đọc chậm sẽ làm chậm run, và nếu Consumer drop stream, tác vụ sẽ tự động dừng lại.
- **Bài học rút ra cho Merlink:** Việc truyền `CancellationToken` và dùng Channel với backpressure là bắt buộc đối với desktop app để UI React không bị nghẽn (render lag do bão token). Tuy nhiên, `AgentExecutor` của LangChain-Rust lưu trữ trạng thái phân tán (biến cục bộ trong hàm `run`), không chuyển hóa thành explicit states, gây khó khăn cho việc lưu checkpoint và phục hồi sau sự cố.

### 2.3 AutoGPT Rust / Tauri Desktop Agents Patterns
- **Đặc thù Desktop Harness:** Trong môi trường Desktop, ứng dụng không phải là một microservice chạy 24/7 trên cloud mà chịu sự can thiệp trực tiếp từ người dùng cuối: tắt app, ngắt nguồn, đưa máy vào chế độ Sleep/Hibernate, thu nhỏ cửa sổ.
- **Thực tế IPC giữa Rust và Webview:**
  - Không dùng cơ chế phát sự kiện broadcast toàn cục không định danh (`app.emit()`), vì dễ gây mất trật tự thông điệp (out-of-order) và thiếu ràng buộc kiểu.
  - Sử dụng kênh điểm-điểm chuyên dụng `tauri::ipc::Channel<RunEvent>` gắn liền với từng lệnh `chat_send`, kết hợp thư viện sinh kiểu `tauri-specta` tạo ra `bindings.ts` an toàn tại compile-time.

### 2.4 DeepSeek Harness Architecture (`deepseek-harness`)
- **Tham khảo từ repo DeepSeek Harness:**
  - Quy tắc phân ranh giới năng lực (Capability Boundaries): Quyền hạn của Agent được kiểm soát bởi danh sách Tool được cung cấp trong từng Mode (`explore` vs `edit`), thay vì tin tưởng vào câu lệnh cấm đoán trong System Prompt.
  - Lưu trữ vết thực thi bất biến (Immutable Audit Ledger): Mỗi sự kiện, suy luận, kết quả tool đều được append-only vào bảng dữ liệu vết (`run_steps`), tuyệt đối không dùng thao tác `UPDATE` làm mất dữ liệu lịch sử.

### 2.5 Smolagents (HuggingFace) & Python vs Rust Harness Patterns
- **So sánh mô hình ngôn ngữ:**
  - Python (Smolagents/LangChain): Dựa vào Dynamic Dispatch, Global Interpreter Lock (GIL), và cơ chế ngắt thread thiếu an toàn (`Thread.interrupt()` hoặc raise exception bất đối xứng), dẫn đến rủi ro rò rỉ tài nguyên hệ thống (file descriptors, unclosed socket).
  - Rust: Ràng buộc quyền sở hữu (Ownership/Borrow Checker), huỷ tài nguyên theo nguyên lý RAII (`Drop` trait), mô hình bất đồng bộ tường minh (Cooperative Multitasking với Tokio). Khi một Future bị drop, mọi tài nguyên liên đới được giải phóng an toàn 100%.

### 2.6 Prime Agent Rust Rewrite (Prime Intellect, Tháng 10/2026)
- **Tài liệu tham khảo:** [Prime Intellect Engineering Blog - Rewriting Prime Agent in Rust](https://www.primeintellect.ai/blog/prime-agent-rust) (và công trình nghiên cứu Karten et al., 2026, arXiv:2608.23552).
- **Phân tích kỹ thuật chuyên sâu:**
  - Prime Agent (đã xử lý hơn 8 nghìn tỷ token) đã thực hiện việc chuyển đổi toàn bộ kiến trúc từ TypeScript sang Rust.
  - **Mô hình State Machine phân tầng (Orchestration of Finite State Machines):** Dự án sử dụng FSM để điều phối vòng lặp tính toán và kiểm tra tính đúng đắn (`correctness checks`), phân định rạch ròi giữa Planner, Implementer, Reviewer và Verifier.
  - **Hệ thống phân rã Crate 1 chiều (One-way Dependency Graph):** Mã nguồn được chia thành 9 crate có ranh giới phụ thuộc nghiêm ngặt được Cargo kiểm tra tự động.
  - **Cô lập Session và Chống Crash:** Mỗi phiên làm việc chạy trong một worker cô lập, trạng thái session được persist trên đĩa để client có thể re-attach ngay lập tức sau sự cố sập app.
  - **Số liệu Benchmark thực tế (TypeScript vs Rust):**
    - Thời gian phản hồi gõ phím từ lúc mở app (Cold start time-to-type): **nhanh hơn 13.22 lần** (55.8ms ở Rust so với 737.8ms ở TypeScript).
    - Mức tiêu thụ bộ nhớ RAM (RSS Memory sau khởi động): **nhỏ hơn 5.73 lần** (106MB ở Rust so với 607.4MB ở TypeScript).
    - Dung lượng cài đặt (Installed size): **gọn hơn 2.89 lần** (59.6MB so với 172.1MB).

### 2.7 Báo cáo Zylos AI Research (Tháng 3/2026) về Hạ tầng Runtime Rust Agent
- **Tài liệu tham khảo:** [Zylos Research - Building AI Agent Frameworks in Rust: Performance, Safety, and the Production Runtime Layer](https://zylos.ai/research/2026-03-31-rust-ai-agent-frameworks-infrastructure/)
- **Các phát hiện kỹ thuật cốt lõi:**
  1. **Loại bỏ hoàn toàn hiện tượng Spike Latency nhờ Không có Garbage Collector (Zero GC Pauses):** Trong quá trình stream token SSE, việc GC chạy giữa chừng trên Python/NodeJS tạo ra độ trễ giật cục (P99 lên tới 45ms). Ở Rust, bộ nhớ được thu hồi tất định theo scope (P99 giữ sát P50 ở mức 4.8ms, giảm 9.4 lần).
  2. **Thực thi Công cụ Song song Thực thụ (True Parallel Tool Execution):** Không bị trói buộc bởi Python GIL. Tokio Runtime cho phép hàng loạt tác vụ I/O (FTS5 search, file extraction, DB queries) chạy đa luồng tối ưu hóa 100% tài nguyên CPU đa lõi.
  3. **Serde Compile-time Tool Schemas:** Thay vì validate dictionary runtime lỏng lẻo như Python, Rust định nghĩa schema bằng trait và struct typed. Lỗi cấu trúc bị bắt ngay ở compile-time; khi LLM trả JSON, Serde deserialize thẳng vào typed struct.
  4. **Cây Cancellation Token (`CancellationToken Trees`):** Khẳng định `tokio_util::sync::CancellationToken` là mẫu hình chuẩn mực công nghiệp: 1 Root Token cho session, phân nhánh thành các Child Tokens cho từng tác vụ con (streaming, tool execution), kết hợp `tokio::select!` để race giữa hoàn thành và hủy bỏ một cách nguyên tử.
  5. **Native Backpressure với Bounded Channels:** Sử dụng `tokio::sync::mpsc::channel(BUFFER_SIZE)` có giới hạn dung lượng để consumer chậm tự động điều tiết tốc độ của producer, ngăn ngừa cạn kiệt RAM khi streaming token dài.

### 2.8 AutoAgents (`liquidos-ai/autoagents`) & Đánh giá Mô hình Ractor
- **Tài liệu tham khảo:** [AutoAgents Documentation & Framework Architecture](https://liquidos-ai.github.io/AutoAgents/)
- **Phân tích kỹ thuật:**
  - AutoAgents sử dụng framework Actor `Ractor` trên Rust để xây dựng các agent tự hành cộng tác theo mô hình publish/subscribe topics.
  - **Bài học rút ra cho Merlink:** Mô hình Actor của AutoAgents rất phù hợp cho mạng lưới multi-agent phân tán trên cloud/edge. Tuy nhiên, với một Desktop Harness in-process tương tác với SQLite cục bộ như Merlink, việc dùng Actor làm phát sinh rào cản lớn khi quản lý lifetime của `sqlx::Transaction` và gây trễ không cần thiết do xếp hàng mailbox. Điều này củng cố tính đúng đắn của quyết định chọn **Explicit Sans-I/O Loop** cho Merlink.

---

## 3. THIẾT KẾ KIẾN TRÚC RUNTIME & STATE MACHINE ĐỀ XUẤT CHO MERLINK

Dựa trên việc kế thừa mô hình Sans-I/O từ Rig, cơ chế cancellation an toàn từ LangChain-Rust, cùng yêu cầu đặc thù của Merlink (SQLite Local, 1 người dùng, can thiệp viết văn chuyên sâu), chúng tôi đề xuất kiến trúc Runtime chi tiết như sau:

```
┌────────────────────────────────────────────────────────────────────────┐
│                            MERLINK AGENT RUNTIME                       │
│                                                                        │
│   ┌────────────────────────────────────────────────────────────────┐   │
│   │               Async Run Loop Driver (run_loop.rs)              │   │
│   │   - Tokio select! [ cancellation, active_future ]              │   │
│   │   - Budget Tracker & Wall-clock Execution Timer                │   │
│   │   - Stream Item Normalization -> EventSink -> Tauri Channel    │   │
│   └───────────────┬────────────────────────────────▲───────────────┘   │
│                   │ advances with I/O results      │ requests next step│
│   ┌───────────────▼────────────────────────────────┴───────────────┐   │
│   │            Sans-I/O State Machine (phase.rs, state.rs)         │   │
│   │                                                                │   │
│   │  [Idle] ──> [BuildingContext] ──> [Streaming] ──> [PolicyCheck]│   │
│   │                                                         │      │   │
│   │        ┌─────────────────── AwaitingApproval ───────────┘      │   │
│   │        ▼                                                       │   │
│   │  [VerifyingPre] ──> [Executing] ──> [VerifyingPost]            │   │
│   │                                            │                   │   │
│   │      (Pass/Retry)                          │                   │   │
│   │         ┌──────────────────────────────────┘                   │   │
│   │         ▼                                                      │   │
│   │  [BuildingContext] (Next turn)   OR   [Finalizing] -> Completed│   │
│   │                                                                │   │
│   │  * Terminal: Completed | Cancelled | Failed | Blocked          │   │
│   └────────────────────────────────────────────────────────────────┘   │
└────────────────────────────────────────────────────────────────────────┘
```

### 3.1 Mô hình State Machine tường minh (`RunPhase`)
Thay vì để các sự kiện bất đồng bộ kích hoạt lẫn nhau một cách hỗn loạn, toàn bộ vòng đời của một Run được định nghĩa bằng một Finite State Machine (FSM) chặt chẽ.

#### Định nghĩa Enum trạng thái `RunPhase`:
```rust
// crates/agent/src/runtime/phase.rs

use serde::{Deserialize, Serialize};
use domain::run::{ApprovalId, StepId, ToolCallId};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "phase", content = "data")]
pub enum RunPhase {
    /// Trạng thái khởi tạo, chưa thực thi
    Idle,
    
    /// Đang tổng hợp System prompt, Pinned memories, Context history, Tool schemas
    BuildingContext { step_seq: u32 },
    
    /// Đang nhận stream từ LLM (tokens, reasoning chunk, tool call fragments)
    Streaming { step_seq: u32 },
    
    /// Model phát tín hiệu gọi Tool, kiểm tra quyền theo RunMode & ToolPolicy
    PolicyCheck { step_seq: u32, tool_calls_count: usize },
    
    /// Tạm dừng run loop vô thời hạn, nhả Semaphore slot, chờ phản hồi người dùng
    AwaitingApproval {
        approval_id: ApprovalId,
        calls: Vec<domain::tool::ToolCallSummary>,
    },
    
    /// Kiểm tra tiền điều kiện trước khi chạy Tool (Schema, Scope isolation, Business invariants)
    VerifyingPre { call_id: ToolCallId },
    
    /// Đang thực thi Tool trong SQLite Transaction (chỉ ghi dữ liệu với Commit Guard)
    Executing { call_id: ToolCallId },
    
    /// Thẩm định hậu điều kiện sau khi Tool chạy (Foreign keys, version increments, business rules)
    VerifyingPost { call_id: ToolCallId },
    
    /// Đã hoàn tất phản hồi cuối cùng, đóng gói message, cập nhật memory đề xuất
    Finalizing,
    
    // --- CÁC TRẠNG THÁI KẾT THÚC (TERMINAL STATES) ---
    /// Hoàn thành thành công toàn bộ chu trình
    Completed,
    
    /// Bị hủy bởi người dùng thông qua command chat_cancel
    Cancelled,
    
    /// Bị chặn do vượt quá RunBudget hoặc vi phạm Invariant không thể phục hồi
    Blocked { reason: BlockedReason },
    
    /// Thất bại do lỗi không thể khắc phục (LLM 4xx, hạ tầng hỏng)
    Failed { code: String, message: String },
    
    /// Bị gián đoạn do app tắt/crash (được gán trong quá trình Startup Reconcile)
    Interrupted { last_completed_step: u32 },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BlockedReason {
    BudgetExceeded { kind: BudgetKind, limit: u64, used: u64 },
    SceneLocked { scene_id: String },
    ContextOverflow { tokens: usize, window: usize },
    SafetyViolation(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BudgetKind {
    StepLimit,
    ToolCallLimit,
    ToolRetryLimit,
    LlmRetryLimit,
    TokenLimit,
    DurationLimit,
}
```

#### Ma trận chuyển dịch trạng thái hợp lệ (State Transition Matrix):
Mọi hành vi chuyển trạng thái đều phải thông qua hàm `transition_to(target_phase)`. Nếu hành vi chuyển dịch không nằm trong ma trận được phép, hàm lập tức trả về lỗi `IllegalTransitionError`, ngăn chặn trạng thái corrupt:

| Trạng thái hiện tại | Trạng thái tiếp theo hợp lệ | Điều kiện kích hoạt |
|---|---|---|
| `Idle` | `BuildingContext` | Nhận lệnh `chat_send` từ UI |
| `BuildingContext` | `Streaming` | Context đã được nạp và tokenize thành công |
| `BuildingContext` | `Blocked` / `Failed` | Context vượt quá kích thước cửa sổ (`CONTEXT_TOO_SMALL`) |
| `Streaming` | `PolicyCheck` | LLM hoàn tất sinh các `tool_call` |
| `Streaming` | `Finalizing` | LLM trả về câu trả lời cuối cùng (`final_answer`) |
| `Streaming` | `Streaming` (reset) | Lỗi stream giữa chừng, emit `StreamReset` thử lại |
| `PolicyCheck` | `AwaitingApproval` | Có ít nhất 1 tool yêu cầu duyệt (`policy = ask`) |
| `PolicyCheck` | `VerifyingPre` | Toàn bộ tool được phép tự chạy (`policy = auto`) |
| `AwaitingApproval` | `VerifyingPre` | Người dùng gửi lệnh `approval_decide` chấp thuận |
| `AwaitingApproval` | `BuildingContext` | Người dùng từ chối toàn bộ tool (trả `rejected` cho LLM) |
| `VerifyingPre` | `Executing` | Thẩm định tiền điều kiện đạt chuẩn (`Verdict::Pass`) |
| `VerifyingPre` | `BuildingContext` | Thẩm định thất bại, nạp feedback và yêu cầu LLM thử lại |
| `VerifyingPre` | `Blocked` | Vi phạm quy tắc bất biến nghiêm ngặt (vd: Scene bị khóa) |
| `Executing` | `VerifyingPost` | Tool thực thi xong câu lệnh trong transaction |
| `VerifyingPost` | `BuildingContext` | Thẩm định hậu điều kiện đạt chuẩn, bước sang turn mới |
| `VerifyingPost` | `Blocked` | Hậu điều kiện thất bại không thể phục hồi, rollback |
| `Finalizing` | `Completed` | Lưu message envelope và trace thành công |
| **Bất kỳ trạng thái** | `Cancelled` | Nhận tín hiệu `CancellationToken::cancel()` |
| **Bất kỳ trạng thái** | `Blocked` | Vượt bất kỳ hạn mức nào của `RunBudget` |
| **Bất kỳ trạng thái** | `Failed` | Lỗi fatal không thể phục hồi |

### 3.2 Cơ chế Async Run Loop (`run_loop.rs`)
Vòng lặp điều phối đóng vai trò là "người lái" (Driver) bất đồng bộ. Vòng lặp liên tục truy vấn State Machine, thực hiện các tác vụ I/O tương ứng, rồi đẩy kết quả trở lại để đưa State Machine sang pha kế tiếp.

#### Mã nguồn kiến trúc mẫu của Vòng lặp điều phối:
```rust
// crates/agent/src/runtime/run_loop.rs

use std::sync::Arc;
use tokio_util::sync::CancellationToken;
use domain::ports::{EventSink, LlmClient, StoragePool};
use crate::runtime::{phase::RunPhase, budget::BudgetTracker};

pub struct RunDriver<L: LlmClient, S: StoragePool, E: EventSink> {
    run_id: String,
    session_id: String,
    llm: Arc<L>,
    storage: Arc<S>,
    event_sink: Arc<E>,
    cancellation_token: CancellationToken,
    budget_tracker: BudgetTracker,
}

impl<L: LlmClient, S: StoragePool, E: EventSink> RunDriver<L, S, E> {
    pub async fn execute_loop(mut self, mut phase: RunPhase) -> Result<(), AgentError> {
        while !self.is_terminal(&phase) {
            // 1. Kiểm tra tín hiệu hủy bỏ trước mỗi bước
            if self.cancellation_token.is_cancelled() {
                phase = self.handle_cancellation().await?;
                break;
            }

            // 2. Kiểm tra Budget trước khi tiêu tốn thêm tài nguyên
            if let Some(exhausted) = self.budget_tracker.check_budget() {
                phase = self.handle_budget_exceeded(exhausted).await?;
                break;
            }

            // 3. Thực thi hành vi I/O tương ứng với pha hiện tại
            let next_phase = match phase {
                RunPhase::BuildingContext { step_seq } => {
                    self.execute_build_context(step_seq).await?
                }
                RunPhase::Streaming { step_seq } => {
                    tokio::select! {
                        biased;
                        _ = self.cancellation_token.cancelled() => {
                            self.handle_cancellation().await?
                        }
                        stream_res = self.execute_streaming(step_seq) => {
                            stream_res?
                        }
                    }
                }
                RunPhase::PolicyCheck { step_seq, tool_calls_count } => {
                    self.execute_policy_check(step_seq, tool_calls_count).await?
                }
                RunPhase::AwaitingApproval { approval_id, calls } => {
                    // Chờ phản hồi từ command IPC approval_decide
                    // Lưu ý: Run nhả Semaphore slot khi ở pha này!
                    self.wait_for_user_approval(approval_id, calls).await?
                }
                RunPhase::VerifyingPre { call_id } => {
                    self.execute_pre_verification(call_id).await?
                }
                RunPhase::Executing { call_id } => {
                    // Chạy trong SQLite transaction có Commit Guard
                    self.execute_tool_with_commit_guard(call_id).await?
                }
                RunPhase::VerifyingPost { call_id } => {
                    self.execute_post_verification(call_id).await?
                }
                RunPhase::Finalizing => {
                    self.execute_finalizing().await?
                }
                _ => break,
            };

            // 4. Ghi nhận vết chuyển trạng thái vào DB (Append-only run_steps)
            self.record_step_trace(&phase, &next_phase).await?;
            phase = next_phase;
        }

        // 5. Kết thúc vòng lặp, cập nhật trạng thái bảng agent_runs
        self.finalize_run_record(&phase).await?;
        Ok(())
    }
}
```

### 3.3 Quản lý Budget thực thi (`RunBudget`)
Để ngăn chặn các trường hợp agent lặp vô tận (infinite reasoning loop), gọi tràn lan công cụ hoặc làm cạn kiệt số dư tài khoản LLM của người dùng, Merlink áp dụng cấu trúc `RunBudget` được cưỡng chế ở cấp độ Runtime thay vì dựa vào prompt.

```rust
// crates/agent/src/runtime/budget.rs

use std::time::{Duration, Instant};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunBudget {
    /// Số bước lặp tối đa của một run (mặc định: 25)
    pub max_steps: usize,
    /// Tổng số lần gọi tool tối đa trong một run (mặc định: 40)
    pub max_tool_calls: usize,
    /// Số lần thử lại tối đa cho 1 tool bị lỗi schema/verification (mặc định: 2)
    pub max_retries_per_tool: usize,
    /// Số lần thử lại tối đa đối với lỗi mạng/429/5xx của LLM (mặc định: 3)
    pub max_llm_retries: usize,
    /// Tổng số token tối đa cho phép tiêu thụ (Prompt + Completion)
    pub max_tokens_total: Option<usize>,
    /// Thời gian chạy tối đa (Active Compute Duration, loại trừ thời gian chờ Approval)
    pub max_duration: Duration,
}

pub struct BudgetTracker {
    budget: RunBudget,
    current_steps: usize,
    current_tool_calls: usize,
    current_llm_retries: usize,
    current_tokens: usize,
    active_elapsed: Duration,
    timer_checkpoint: Option<Instant>,
    warned_80_percent: [bool; 6], // Đánh dấu đã cảnh báo ở ngưỡng 80%
}

impl BudgetTracker {
    /// Bắt đầu đếm thời gian tính toán thực
    pub fn resume_timer(&mut self) {
        self.timer_checkpoint = Some(Instant::now());
    }

    /// Tạm dừng đếm giờ khi rơi vào trạng thái chờ người dùng duyệt (AwaitingApproval)
    pub fn pause_timer(&mut self) {
        if let Some(start) = self.timer_checkpoint.take() {
            self.active_elapsed += start.elapsed();
        }
    }

    /// Kiểm tra các chỉ số ngân sách và phát tín hiệu cảnh báo tại 80%
    pub fn check_and_notify(&mut self, sink: &dyn EventSink) -> Option<BudgetExhausted> {
        let current_elapsed = self.active_elapsed 
            + self.timer_checkpoint.map(|t| t.elapsed()).unwrap_or_default();

        // Kiểm tra ngưỡng 80% để gửi cảnh báo RunEvent::BudgetWarning
        if self.current_steps as f64 >= self.budget.max_steps as f64 * 0.8 && !self.warned_80_percent[0] {
            self.warned_80_percent[0] = true;
            sink.emit(RunEvent::BudgetWarning {
                kind: "steps",
                used: self.current_steps as u64,
                limit: self.budget.max_steps as u64,
            });
        }

        // Kiểm tra vi phạm trần (Hard Cap)
        if self.current_steps >= self.budget.max_steps {
            return Some(BudgetExhausted::StepLimit(self.budget.max_steps));
        }
        if current_elapsed >= self.budget.max_duration {
            return Some(BudgetExhausted::DurationLimit(self.budget.max_duration));
        }
        None
    }
}
```

### 3.4 Lan truyền Cancellation Token xuyên luồng đa tầng
Việc hủy bỏ một tác vụ đang chạy trên desktop app phải diễn ra ngay lập tức, không để lại tiến trình ngầm tiêu hao CPU hay token của người dùng.

```
┌────────────────────────────────────────────────────────────────────────┐
│                   CANCELLATION TOKEN PROPAGATION TREE                  │
├────────────────────────────────────────────────────────────────────────┤
│  Frontend UI: Nút "Dừng" (Stop)                                        │
│        │                                                               │
│        ▼ invoke command "chat_cancel(run_id)"                          │
│  src-tauri Commands: session_registry.get_token(run_id).cancel()       │
│        │                                                               │
│        ├───────────────────────────────────────────────────────┐       │
│        ▼                                                       ▼       │
│  Runtime Loop Driver                             Child Cancellation    │
│  (tokio::select! ngắt vòng lặp)                  Tokens                │
│        │                                                       │       │
│        ├─────────────────────────────┬─────────────────────────┤       │
│        ▼                             ▼                         ▼       │
│  LLM Client Streaming        Tool Execution Task        Extractor Task │
│  (Hủy kết nối HTTP socket    (Rollback transaction      (Dừng parser   │
│   reqwest ngay lập tức)       không commit DB)           docx/pdf)     │
└────────────────────────────────────────────────────────────────────────┘
```

#### Chi tiết kỹ thuật hủy bỏ tầng LLM Streaming:
```rust
// crates/llm/src/client.rs

pub async fn stream_chat_completion(
    &self,
    request: CompletionRequest,
    cancel_token: CancellationToken,
) -> Result<Pin<Box<dyn Stream<Item = Result<StreamItem, LlmError>> + Send>>, LlmError> {
    let response = self.http_client
        .post(&self.endpoint)
        .json(&request)
        .send()
        .await?;

    let byte_stream = response.bytes_stream();
    
    // Tích hợp cancellation token trực tiếp vào Stream adapter
    let cancellable_stream = async_stream::try_stream! {
        tokio::pin!(byte_stream);
        loop {
            tokio::select! {
                biased;
                _ = cancel_token.cancelled() => {
                    // Ngắt stream ngay lập tức, HTTP socket bị đóng tự động
                    Err(LlmError::Cancelled)?;
                    break;
                }
                item = byte_stream.next() => {
                    match item {
                        Some(Ok(bytes)) => {
                            for stream_item in parse_sse_chunk(&bytes)? {
                                yield stream_item;
                            }
                        }
                        Some(Err(err)) => Err(LlmError::Transport(err.to_string()))?,
                        None => break,
                    }
                }
            }
        }
    };

    Ok(Box::pin(cancellable_stream))
}
```

### 3.5 Cơ chế phục hồi khi khởi động (Orphan Run Reconciliation)
Nếu ứng dụng máy tính bị buộc dừng đột ngột (SIGKILL, Task Manager End Task, mất điện, Kernel panic) khi một run đang ở trạng thái `running` hoặc `awaiting_approval`, khi khởi động lại, DB sẽ chứa các dòng bản ghi "mồ côi" không bao giờ kết thúc.

```
┌────────────────────────────────────────────────────────────────────────┐
│                   STARTUP RECONCILIATION FLOW                          │
├────────────────────────────────────────────────────────────────────────┤
│  Ứng dụng Tauri khởi động (src-tauri/src/main.rs)                      │
│        │                                                               │
│        ▼ Chạy trước khi cửa sổ Webview hiển thị                        │
│  OrphanReconciler::reconcile(&db_pool)                                 │
│        │                                                               │
│        ├─► Quét: SELECT id FROM agent_runs WHERE status IN             │
│        │         ('running', 'awaiting_approval')                      │
│        │                                                               │
│        ├─► UPDATE agent_runs SET status = 'interrupted',               │
│        │   finished_at = CURRENT_TIMESTAMP WHERE id = ...             │
│        │                                                               │
│        ├─► INSERT INTO run_steps (kind='system', name='reconcile',    │
│        │   output='{"reason": "App terminated unexpectedly"}')         │
│        │                                                               │
│        ▼ Hoàn tất dọn dẹp -> Mở khóa Session Scheduler                 │
│  Frontend UI tải Session: Hiển thị badge "Bị gián đoạn"                │
│  kèm nút "Tiếp tục" (run_continue)                                     │
└────────────────────────────────────────────────────────────────────────┘
```

#### Triển khai code trong `crates/agent/src/runtime/reconcile.rs`:
```rust
pub async fn reconcile_orphan_runs(pool: &sqlx::SqlitePool) -> Result<usize, sqlx::Error> {
    let mut tx = pool.begin().await?;

    // 1. Tìm các run bị treo
    let orphan_ids: Vec<String> = sqlx::query_scalar!(
        r#"SELECT id FROM agent_runs WHERE status IN ('running', 'awaiting_approval')"#
    )
    .fetch_all(&mut *tx)
    .await?;

    let count = orphan_ids.len();
    if count == 0 {
        return Ok(0);
    }

    // 2. Cập nhật trạng thái thành 'interrupted'
    sqlx::query!(
        r#"UPDATE agent_runs 
           SET status = 'interrupted', finished_at = CURRENT_TIMESTAMP 
           WHERE status IN ('running', 'awaiting_approval')"#
    )
    .execute(&mut *tx)
    .await?;

    // 3. Ghi vết audit vào bảng run_steps để người dùng xem lại trong trace
    for run_id in &orphan_ids {
        let step_id = uuid::Uuid::now_v7();
        sqlx::query!(
            r#"INSERT INTO run_steps (id, run_id, seq, kind, name, output_json, verdict, created_at)
               VALUES ($1, $2, 999999, 'system', 'orphan_reconcile', 
                       '{"event":"app_crash_detected","action":"marked_interrupted"}', 'block', CURRENT_TIMESTAMP)"#,
            step_id,
            run_id
        )
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;
    tracing::info!("Reconciled {} orphan agent runs on startup", count);
    Ok(count)
}
```

### 3.6 Transaction Commit Guard (Bảo vệ tính toàn vẹn dữ liệu)
Một kịch bản lỗi chí mạng (Race Condition) thường gặp:
1. Tool `scene_write_content` thực hiện viết 3000 từ vào DB trong một background worker task.
2. Người dùng phát hiện LLM viết sai ý, lập tức nhấn nút **Hủy (Stop)**.
3. Command `chat_cancel` chạy và cập nhật `agent_runs.status = 'cancelled'`.
4. Tuy nhiên, nếu tool ghi không có Commit Guard, khi hoàn thành câu lệnh SQL, nó sẽ gọi `tx.commit().await`. Kết quả: Dữ liệu bị ghi đè vào DB mặc dù người dùng đã hủy trước đó!

#### Cơ chế Commit Guard trong cùng SQLite Transaction:
```rust
// crates/agent/src/tools/executor.rs

pub async fn execute_write_tool_with_guard<F, Fut>(
    pool: &sqlx::SqlitePool,
    run_id: &str,
    tool_action: F,
) -> Result<ToolOutput, ToolExecutionError>
where
    F: FnOnce(&mut sqlx::Transaction<'_, sqlx::Sqlite>) -> Fut,
    Fut: std::future::Future<Output = Result<ToolOutput, ToolExecutionError>>,
{
    // Bắt đầu một transaction SQLite cô lập
    let mut tx = pool.begin().await?;

    // 1. Thực hiện các thao tác ghi dữ liệu nghiệp vụ
    let output = tool_action(&mut tx).await?;

    // 2. COMMIT GUARD: Truy vấn lại trạng thái của chính Run này trong cùng transaction
    let current_status: String = sqlx::query_scalar!(
        r#"SELECT status FROM agent_runs WHERE id = $1"#,
        run_id
    )
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| ToolExecutionError::Storage(e.to_string()))?;

    // 3. Rào chắn quyết định: Chỉ COMMIT nếu Run vẫn đang trong trạng thái Running
    if current_status == "running" {
        tx.commit().await.map_err(|e| ToolExecutionError::Storage(e.to_string()))?;
        Ok(output)
    } else {
        // Rollback ngay lập tức!
        tx.rollback().await.ok();
        tracing::warn!(
            run_id = %run_id,
            status = %current_status,
            "Transaction commit guard aborted commit because run is no longer active"
        );
        Err(ToolExecutionError::RunAborted(format!(
            "Run is in state '{}', write transaction was safely rolled back",
            current_status
        )))
    }
}
```

---

## 4. SO SÁNH ĐÁNH GIÁ: EXPLICIT LOOP VS ACTOR-BASED CHO DESKTOP HARNESS

Một câu hỏi kiến trúc kinh điển khi làm việc với Rust async là: **Khi nào nên dùng Actor-based (Actix, Tokio Actors) và khi nào nên dùng Explicit Loop kết hợp State Machine?**

| Tiêu chí kỹ thuật | Explicit State Machine Loop (Đề xuất) | Actor-based Architecture (Actix / Tokio Actors) |
|---|---|---|
| **Bản chất điều phối** | Tuần tự, trực diện: `while run.is_active()` | Phân tán thông qua hàng đợi tin nhắn (Mailbox MPSC) |
| **Quản lý DB Transaction** | Cực kỳ tự nhiên: `Transaction` nằm gọn trong scope của hàm | Rất khó: Khó borrow hoặc chuyển `Transaction<'a>` qua message boundary |
| **Tính tất định (Determinism)** | 100% tất định, dễ dàng tái hiện lỗi bằng replay | Khó tất định do phụ thuộc thứ tự đến của tin nhắn trong mailbox |
| **Kiểm thử (Testability)** | Dễ dàng viết test độc lập không cần mock actor framework | Phải khởi động actor system, mock mailbox, dễ dính timeout test |
| **Quản lý Cancellation** | Đơn giản: 1 `CancellationToken` duy nhất điều khiển toàn bộ | Phức tạp: Phải gửi `PoisonPill` hoặc tin nhắn Cancel đến từng actor |
| **Khả năng debug & trace** | Mỗi bước tương ứng 1 dòng tuần tự trong `run_steps` | Khó trace: Log phân tán qua nhiều actor, khó ghép ngữ cảnh |
| **Chi phí bộ nhớ & luồng** | Tối thiểu: Chỉ tạo future khi run hoạt động | Cao hơn: Mỗi actor chiếm giữ mailbox channel và task runtime |
| **Phù hợp với Desktop App** | **Tối ưu tuyệt đối** (Single-user, 1 run/session, in-process) | Kém phù hợp, mang lại độ phức tạp không tương xứng (Overengineering) |

### Kết luận đánh giá:
Mô hình Actor-based chỉ phát huy sức mạnh trong các hệ thống phân tán đa người dùng (Multi-tenant Distributed Systems) với hàng nghìn agent độc lập tương tác tự do (như game server hoặc simulation swarm). Trong bối cảnh của **Merlink (ứng dụng Desktop cục bộ, 1 người dùng, 1 tác vụ chạy tuần tự trên mỗi session, tương tác sâu với SQLite)**, **Explicit State Machine Loop là sự lựa chọn vượt trội tuyệt đối** về độ an toàn, độ trong sáng của code và khả năng kiểm soát vòng đời dữ liệu.

---

## 5. ĐỐI CHIẾU VỚI `docs/be-tauri-plan.md` (MỤC 1, 2, 7)

Đối chiếu bản thiết kế nghiên cứu của Subagent A với kế hoạch hiện tại trong `be-tauri-plan.md`:

```
┌────────────────────────────────────────────────────────────────────────┐
│                 BẢNG ĐỐI CHIẾU VỚI BE-TAURI-PLAN.MD                    │
├─────────────────────────┬────────────────────────┬─────────────────────┤
│ Nội dung trong Plan     │ Đánh giá của Subagent A│ Đề xuất nâng cấp    │
├─────────────────────────┼────────────────────────┼─────────────────────┤
│ Mục 1: Nguyên tắc       │ Rất phù hợp, chuẩn mực │ Bổ sung nguyên tắc  │
│ - Backend-authoritative │ kiến trúc clean code   │ "Sans-I/O Core" vào │
│ - Phân tầng 1 chiều     │ và an toàn bộ nhớ.     │ Cargo workspace.    │
│ - Capability by mode    │                        │                     │
├─────────────────────────┼────────────────────────┼─────────────────────┤
│ Mục 2: Quyết định       │ Rất chuẩn xác khi chọn │ Cần đặc tả chi tiết │
│ - SQLite + sqlx (WAL)   │ SQLite WAL, specta,    │ cấu trúc Step Trace │
│ - specta IPC Channel    │ CancellationToken.     │ cho từng sub-action.│
│ - Loop state machine    │                        │                     │
├─────────────────────────┼────────────────────────┼─────────────────────┤
│ Mục 7.1: State Machine  │ Mô hình các pha hợp lý,│ Cần thêm Timer phân │
│ - RunBudget (5 chỉ số)  │ nhưng chưa tách biệt   │ biệt Active Compute │
│ - Approval không timeout│ giữa Sans-I/O state và │ và Pause Wait để    │
│ - Commit Guard          │ Async Driver.          │ đo Budget chính xác.│
└─────────────────────────┴────────────────────────┴─────────────────────┤
```

### 5.1 Những điểm hoàn toàn phù hợp (Khuyến nghị giữ nguyên)
1. **Phân chia 4 Crate trong Cargo Workspace ngay từ đầu**:
   - `domain`: Thực thể nghiệp vụ, traits/ports, zero dependencies hạ tầng.
   - `agent`: Runtime, State Machine, Tool Registry, Verifier (không dính SQLite hay Tauri).
   - `llm`: Adapter giao thức OpenAI/Anthropic/Ollama, chuẩn hóa `StreamItem`.
   - `infra`: SQLite pool, repos, keyring, fs attachments.
   - Hàng rào `Cargo.toml` ngăn chặn lỗi rò rỉ phụ thuộc ở cấp compiler.
2. **Cơ chế 1 Run Active / Session**:
   - Chặn đứng hoàn toàn việc người dùng gửi 2 câu hỏi đồng thời trên cùng một phiên làm việc, loại bỏ tận gốc bài toán xử lý xung đột concurrent chat trong cùng 1 session.
3. **Approval nhả slot Semaphore**:
   - Khi một run rơi vào trạng thái chờ người dùng duyệt (`awaiting_approval`), nó lập tức trả lại vé Semaphore toàn cục (Global Concurrency Gate) để người dùng có thể thực hiện run ở các session khác.
4. **Append-only Run Steps**:
   - Bảng `run_steps` hoàn toàn là append-only, trở thành nguồn dữ liệu sự thật duy nhất (Single Source of Truth) cho cả UI Timeline và kiểm toán lỗi.

### 5.2 Những điểm cần bổ sung và tối ưu hóa chuyên sâu
1. **Bổ sung Tách biệt Sans-I/O State Machine và Async Driver**:
   - Hiện tại `plan.md` mới nêu chung `run_loop.rs` và `phase.rs`. Khuyến nghị tổ chức:
     - `crates/agent/src/runtime/state.rs`: Struct `RunStateMachine` thuần túy, nhận input và trả về `NextAction` (Sans-I/O).
     - `crates/agent/src/runtime/run_loop.rs`: Task bất đồng bộ xử lý I/O thực sự.
   - Việc này giúp kiểm thử toàn bộ logic state machine với 0ms network latency và 0 dependency mock.
2. **Tối ưu hóa Bộ đếm Budget (`Active Compute Timer`)**:
   - Trong `plan.md`, `max_duration` chưa được làm rõ cách đo khi có phê duyệt. Nếu người dùng mở hộp thoại duyệt tool rồi đi ăn trưa (30 phút), run không được phép bị tính là timeout.
   - **Đề xuất:** Bộ đếm thời gian bắt buộc phải có cơ chế `pause_timer()` khi chuyển vào `AwaitingApproval` và `resume_timer()` khi nhận được quyết định duyệt.
3. **Chuẩn hóa Commit Guard thành Decorator/Middleware**:
   - Không nên để từng Tool tự viết lại câu lệnh kiểm tra `SELECT status FROM agent_runs`.
   - **Đề xuất:** Cung cấp hàm bọc cấp cao `ToolExecutionContext::with_transaction_guard(...)` trong `crates/agent/src/tools/scope.rs`. Mọi tool ghi khi chạy đều tự động thừa hưởng cơ chế commit guard này.
4. **Đặc tả cơ chế Resume sau Gián đoạn (`run_continue`)**:
   - Khi run bị chuyển thành `interrupted` do app tắt đột ngột, người dùng bấm "Tiếp tục".
   - **Đề xuất:** Tạo một run mới liên kết với `parent_run_id`, nạp lại toàn bộ các `run_steps` đã thành công trước đó dưới dạng một bản tóm tắt ngắn gọn (Context Summary) gửi vào prompt của run mới, tránh việc LLM gọi lại các tool đã làm xong.

---

## 6. NHỮNG KIẾN TRÚC & CÁCH TIẾP CẬN KHÔNG PHÙ HỢP (NO-MATCH)

Dưới đây là tóm lược danh sách các kiến trúc, pattern hoặc giải pháp công nghệ **tuyệt đối KHÔNG sử dụng cho Merlink Backend**, kèm lý do kỹ thuật rõ ràng. Chi tiết phân tích đầy đủ đã được xuất bản độc lập tại tài liệu [docs/prepare-be-impl/no-match.md](./no-match.md):

### 1. Actor Framework toàn diện (Actix / Riker / Xactor)
- **Mô tả:** Mô hình hóa mỗi Session hoặc mỗi Tool thành một Actor độc lập chạy trên mailboxes.
- **Lý do từ chối:**
  - Vòng đời giao dịch cơ sở dữ liệu của `sqlx` dựa trên RAII `Transaction<'a>`. Việc chuyển giao reference hoặc lifetime của Transaction qua các Actor message đòi hỏi boxing phức tạp (`Arc<Mutex<...>>`), phá hủy cơ chế zero-cost abstraction của Rust.
  - Tăng độ phức tạp khi gỡ lỗi: Khó theo dõi stack trace tuần tự khi có sự cố.
  - Quá tải không cần thiết cho một ứng dụng desktop in-process một người dùng.

### 2. Event-Choreography (Kiến trúc phân tán sự kiện phi tập trung)
- **Mô tả:** Không có vòng lặp trung tâm; các thành phần (LLM, Tool, DB, UI) tự phát sự kiện và lắng nghe sự kiện của nhau trên một Event Bus nội bộ để tự quyết định bước tiếp theo.
- **Lý do từ chối:**
  - "Phân mảnh trạng thái" (Distributed State Nightmare): Rất khó xác định hệ thống đang ở pha nào tại một thời điểm cụ thể.
  - Dễ phát sinh Race Condition và Deadlock: Hai sự kiện phát đồng thời có thể khiến 2 tool cùng ghi đè một scene.
  - Không thể đảm bảo tính tất định để kiểm thử và phục hồi sau sự cố.

### 3. Prompt-based Tool Calling (Sinh JSON tool call qua System Prompt cho model yếu)
- **Mô tả:** Ép model không hỗ trợ native tool calling sinh ra text dạng `Action: tool_name` rồi dùng Regex để bóc tách.
- **Lý do từ chối:**
  - Độ tin cậy cực kỳ thấp: Dễ bị ảo giác (hallucination), sai cú pháp escape JSON trong tiểu thuyết (vốn chứa nhiều dấu ngoặc kép, ký tự lạ).
  - Nguy cơ Prompt Injection: Nội dung file đính kèm hoặc text do user nhập có thể lừa model giả mạo lệnh gọi tool.
  - Chi phí thử lại cao, tốn token vô ích. Đã chốt theo ADR-005: Model không hỗ trợ function calling chỉ dùng chat thuần.

### 4. Polling-based IPC từ Webview (Frontend định kỳ poll trạng thái Run)
- **Mô tả:** React FE gọi command `get_run_status` mỗi 500ms để kiểm tra xem agent đã chạy xong chưa.
- **Lý do từ chối:**
  - Tiêu tốn CPU và pin của máy tính người dùng liên tục ngay cả khi agent đang chờ LLM phản hồi.
  - Độ trễ cao, trải nghiệm người dùng giật cục, không thể hiện được hiệu ứng streaming mượt mà từng từ một.
  - Thay thế hoàn toàn bằng `tauri::ipc::Channel<RunEvent>` điểm-điểm hướng sự kiện đẩy (Push-based).

### 5. Multi-Agent Swarm / Hierarchical Multi-Process Daemon
- **Mô tả:** Dựng nhiều tiến trình daemon chạy ngầm bên ngoài Tauri, kết nối qua gRPC / Unix Domain Socket để phối hợp đàn agent (Swarm).
- **Lý do từ chối:**
  - Gia tăng đột biến độ phức tạp cài đặt và đóng gói ứng dụng (Packaging/Installer) trên Windows và macOS.
  - Rủi ro bị phần mềm diệt virus (Antivirus) chặn tiến trình con chạy nền.
  - Vấn đề đồng bộ và xung đột quyền ghi SQLite giữa nhiều tiến trình (file locking contention).

### 6. Hai nguồn sự thật cho Lịch sử Vết (Rollout file JSONL song song với SQLite)
- **Mô tả:** Ghi lịch sử hội thoại vào SQLite nhưng lại ghi vết suy luận chi tiết vào các file JSONL rời trên ổ cứng.
- **Lý do từ chối:**
  - Nguy cơ lệch pha dữ liệu (State Drift) khi ứng dụng bị tắt đột ngột: file SQLite đã commit nhưng file JSONL chưa flush đĩa hoặc ngược lại.
  - Khó khăn trong việc xóa sạch dữ liệu người dùng khi thực hiện thao tác Clear Data / Reset Project.
  - Bảng `run_steps` với index `(run_id, seq)` trong SQLite là hoàn toàn đủ nhanh và gọn cho việc lưu vết kiểm toán.

---

## 7. KẾT LUẬN & ĐỀ XUẤT CHO CÁC BƯỚC TIẾP THEO

Báo cáo nghiên cứu của Subagent A khẳng định rằng:
1. Kiến trúc **Async Run Loop điều khiển bởi Explicit Sans-I/O State Machine** là giải pháp kỹ thuật tối ưu nhất cho Merlink. Nó mang lại sự cân bằng hoàn hảo giữa tính tin cậy, an toàn dữ liệu, tính tất định trong kiểm thử và sự gọn gàng về tài nguyên máy tính.
2. Các cơ chế phòng thủ chuyên sâu gồm **Run Budget, Cancellation Token đa tầng, Orphan Run Reconciliation, và Transaction Commit Guard** sẽ biến Merlink thành một Harness Agent chuẩn công nghiệp, loại bỏ hoàn toàn các lỗi phổ biến như treo ứng dụng, ghi đè dữ liệu bẩn và tiêu tốn token ngoài tầm kiểm soát.

### Đề xuất hành động tiếp theo:
- Tham chiếu và cập nhật liên tục các kiến trúc bị từ chối ở Mục 6 tại tài liệu độc lập `docs/prepare-be-impl/no-match.md`.
- Chuyển giao các interface và state enum sang cho các subagent phụ trách tầng Tool, Verification và Context Builder để đồng bộ hóa mã nguồn.
- Tiến hành hiện thực hóa Phase 0 và Vertical Slice của Phase 1 theo đúng kế hoạch trong `be-tauri-plan.md`.
