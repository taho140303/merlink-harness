# PLAN: merlink-harness-ui → Tauri Agent Harness

> **v3:** v2 = chắt lọc từ 5 harness tham khảo (mục 2.1); v3 = xử lý review (mục 2.2, 5.4–5.5, 6.1, 7.2, roadmap có vertical slice).
>
> Mục tiêu: biến FE React hiện tại thành **desktop app** (Tauri) đóng vai trò **harness cho agent sống**: chat theo project, agent thao tác business data qua tool call, quản lý context memory, attach file, system prompt nhiều lớp, agent runtime có verify, **cắm API key + tự customize model**.

---

## 1. Phạm vi & nguyên tắc

| Nguyên tắc | Ý nghĩa |
|---|---|
| **Rust = backend nhúng** | Agent runtime, DB, file, secrets, tool execution nằm ở Rust. React chỉ là UI. |
| **Phân lớp một chiều** | `commands → agent → domain ← infra`; `llm → domain`. Domain không biết Tauri/SQLite/HTTP. |
| **Ép ranh giới bằng Cargo workspace** | Layer vi phạm thì compiler báo lỗi, không dựa vào quy ước. |
| **`commands/` mỏng** | Parse input → gọi use case → trả DTO. Không chứa logic. |
| **Streaming bằng `tauri::ipc::Channel`** | Token, tool call, verify, approval. Không polling. |
| **Type-safe IPC** | `tauri-specta` sinh `bindings.ts`; FE không viết tay `invoke("...")`. |
| **Provider-agnostic** | Runtime chỉ nói chuyện với trait `LlmClient`; model/provider là **dữ liệu cấu hình** (DB), không hard-code. |
| **Mọi ghi dữ liệu đều đảo ngược được** | Soft delete (`deleted_at`), `scene_versions`, recycle bin. |
| **Backend-authoritative tuyệt đối** | FE không gọi LLM, không giữ state nghiệp vụ; chỉ render `RunEvent` và gọi command. |
| **Capability thay vì prompt** | Quyền được cưỡng chế bằng *tập tool có mặt trong run* (theo mode), không bằng câu "đừng làm X" trong prompt. |
| **Giới hạn bằng runtime** | Budget (token / thời gian / số tool / số step) do runtime enforce, không nhờ prompt. |

---

## 2. Quyết định kiến trúc

| Vấn đề | Quyết định | Ghi chú |
|---|---|---|
| DB | SQLite local, migration chạy lúc khởi động | Schema bám `docs/db.dbml` |
| DB driver & Pool | **`sqlx` (sqlite)** + `sqlx::migrate!`<br>**Mô hình Dual-Pool (Writer / Reader)** | `writer_pool`: cố định `max_connections = 1` tuần tự hóa các lệnh ghi ở tầng ứng dụng, triệt tiêu hoàn toàn Lock Starvation trong async Rust; `reader_pool`: `max_connections = 10`, `read_only = true` phục vụ UI đọc song song mượt mà. PRAGMA: WAL, `foreign_keys=ON`, `busy_timeout=5000`, `synchronous=NORMAL`, `cache_size=-64000` (64MB RAM), `temp_store=MEMORY` |
| API key | `keyring` crate (OS keychain) | DB chỉ lưu `secret_ref`, không lưu key |
| ID | Business: UUIDv7 (đã có). Runtime: string có prefix `ses_`, `msg_`, `run_` (ULID/UUIDv7) | Khớp `sessions.id`/`messages.id` là `varchar` |
| Memory retrieval | Phase đầu: **SQLite FTS5 + pin + recency**. Lộ trình Phase 3: `sqlite-vec` + `fastembed-rs` nhúng in-process | Tránh phụ thuộc embedder sớm, zero tiến trình ngoài |
| Cancel | `CancellationToken` đi xuyên runtime → tool → llm | Đóng ngay HTTP socket stream, rollback transaction an toàn |
| FE routing | `react-router.config.ts`: `ssr: false` (SPA) | Dockerfile chỉ còn cho bản web |
| Server state FE | TanStack Query, invalidate khi nhận event từ Rust | Client state: Zustand (đã có `store.ts`) |
| Cấu trúc crate | **Cargo workspace 4 crate ngay từ Phase 0** (`domain`, `agent`, `llm`, `infra`), mỗi crate mỏng (vài file) | Hàng rào là `Cargo.toml`, compiler enforce; ma trận phụ thuộc ở mục 4.2 |
| Điều phối run | Async loop điều khiển bởi **state machine tường minh** (`RunPhase`) | Tách biệt Sans-I/O Functional Core (`state.rs`) và I/O Driver (`run_loop.rs`) (mục 7.1) |
| Domain event | `DomainEvent` + port `EventSink`; `RunEvent` (Channel), `entity_changed`, trace đều là subscriber | Một nguồn sự thật cho UI và trace |
| Trace | `run_steps` **append-only** trong SQLite; không thêm JSONL rollout riêng | Lưu trữ thống nhất trong SQLite; step `context` chứa Prompt Manifest |
| Đồng thời | 1 run active / session; semaphore toàn cục (mặc định 3 run). `chat_send` khi session đang có run → lỗi `RUN_ACTIVE`, **không queue** | Run chờ approval nhả slot semaphore để tránh chiếm dụng quota |
| Tool calling | Chỉ model `supports_tools = true` chạy agent; model còn lại = chat thuần | Tuân thủ ADR-005; sinh schema tự động qua `schemars` |
| Approval | Non-blocking Async Suspension (`tokio::sync::oneshot`), pause vô thời hạn cho human approval | Không block worker thread, Active Compute Timer tạm dừng đếm giờ tính toán |
| Token | `tiktoken-rs` xấp xỉ + **Adaptive EMA Calibration** (`token_calibration`) theo từng model | Mục 7.2, ADR-003; tự động thích ứng với đặc thù tiếng Việt |
| IPC | **`tauri-specta` v2 (RC-27) + `tauri::ipc::Channel<RunEvent>`** | Kênh stream trực tiếp strictly in-order cho token/progress; FE buffer bằng `requestAnimationFrame` |
| Log | `tracing` ra file xoay vòng, tách khỏi `run_steps` | Mục 14 |

### 2.1 Chắt lọc từ các harness tham khảo

Chỉ lấy **pattern khẳng định tích cực** (không phụ thuộc repo nào). Tiêu chí: tối ưu cho *một process Tauri, một người dùng, SQLite local, tool chỉ chạm DB nghiệp vụ*.

| Pattern (nguồn) | Áp dụng vào plan |
|---|---|
| Backend-authoritative (Orchestrix) | FE zero logic, kể cả gọi LLM; mọi request ra ngoài đi qua Rust |
| Tool boundary theo mode (Orchestrix plan/worker) | `RunMode`: `explore` (chỉ tool đọc) / `edit`; tool ghi **hoàn toàn không xuất hiện trong schema** khi ở `explore`; `request_edit_mode` để xin nâng quyền |
| Budget riêng mỗi run (Orchestrix) | `RunBudget`: step, tool call, retry, token, thời gian tính toán thực (Active Compute Timer) |
| Approval tách khỏi executor (LibrAgent `tool_approvals`) | `policy.rs` quyết định, `approval.rs` chờ user; đổi policy không đụng executor |
| Execution mode như policy (LibrAgent normal/yolo) | `ApprovalPolicy`: `ask` / `auto_reversible`. Bỏ mức `unsafe` vì không có tool shell/fs |
| Scope bị cưỡng chế (LibrAgent session-scoped proxy) | `ToolContext {project_id, session_id, run_id, mode}` do executor inject; tool không nhận id project từ args |
| Reconcile khi khởi động (Tinybot orphan recovery) | Run `running`/`awaiting_approval` còn sót khi app crash → `interrupted` kèm audit trace |
| Commit guard (Bảo vệ tính toàn vẹn giao dịch) | Tool ghi chỉ commit nếu run còn active trong cùng transaction, triệt tiêu race condition khi cancel |
| Instruction precedence + truy vết (Tinybot `InstructionComposer`) | Precedence cố định + **prompt manifest** lưu vào trace để audit và tối ưu prompt caching |
| Protocol adapter tách khỏi loop (Tinybot) | `llm` chuẩn hoá về `StreamItem`; loop không biết wire format |
| Compact recovery (LibrAgent) | Compactor lỗi → fallback cắt history tất định, không làm fail run |
| Domain event (Quecto) | `DomainEvent` + `EventSink` thay cho phát event rải rác |
| Ports & adapters (Quecto/HiddenSteps) | Đã có; thêm `EventSink`, `Clock`, `IdGen` để test tất định |
| Global concurrency gate (LibrAgent) | Semaphore toàn cục + 1 run/session; nhả permit khi treo chờ duyệt |
| GUI không chứa logic (Orchestrator) | Workspace cho phép thêm `crates/cli` headless để eval/regression (tuỳ chọn) |

> **Ghi chú về các phương án không phù hợp (No-Match):**  
> Toàn bộ các phân tích kỹ thuật, lập luận phủ định và lý do bác bỏ các kiến trúc / công nghệ khảo sát không phù hợp (như Actor-based event-choreography, Docker/WASM sandboxing, external daemons, vector DB rời, prompt-based tool calling, polling IPC, global event emission...) được tổng hợp và phân tích chuyên sâu tại tài liệu: [`docs/prepare-be-impl/no-match.md`](prepare-be-impl/no-match.md).

### 2.2 ADR phải ghi trước Phase 1

| ADR | Nội dung |
|---|---|
| 001 | Workspace 4 crate + ma trận phụ thuộc |
| 002 | `sqlx`, pragma, kiến trúc Dual-Pool (Writer/Reader), cách kiểm tra query |
| 003 | Chiến lược đếm token `tiktoken-rs` + Adaptive EMA calibration |
| 004 | Message envelope `payload_json` v1 (mục 5.5) |
| 005 | Không prompt-based tool calling; bắt buộc `supports_tools = true` |
| 006 | Non-blocking approval pause vô thời hạn, batch theo lượt, nhả permit semaphore |
| 007 | Quan hệ Nhiều-Nhiều (N-N) giữa projects và characters qua bảng liên kết `project_characters` (mục 5.4) |

### 2.3 Hệ thống Design Patterns chuẩn hóa (21 Patterns Chọn Lọc)

> **Nguyên tắc áp dụng:** Đây là danh mục pattern được chọn làm công cụ thiết kế cho các nhu cầu cụ thể, không phải yêu cầu tạo 21 framework/module trừu tượng riêng. Chỉ triển khai abstraction khi có hơn một implementation thực sự cần thay thế, hoặc khi abstraction giúp cưỡng chế an toàn, tính đúng đắn, hoặc kiểm thử được ranh giới.

> Các pattern dưới đây đã được chốt sau khi đối chiếu; mục 2.3.1–2.3.6 ghi **cách áp dụng cụ thể** (kiểu dữ liệu, thứ tự thực thi, bất biến) để triển khai không cần suy diễn thêm. Pattern bị loại bỏ và lý do: [`docs/prepare-be-impl/no-match.md`](prepare-be-impl/no-match.md), phân hệ 6.

| # | Nhóm | Design Pattern | Thành phần áp dụng trong Merlink | Lợi ích tối ưu hóa cốt lõi |
|---|---|---|---|---|
| 1 | **Creational** | **Builder + Typestate** | `ContextBuilder<Stage>`, `LlmChatRequestBuilder`, `RunConfigBuilder` | Dùng zero-sized marker structs (`MissingProject` → `HasProject` → `ReadyToBuild`) ép buộc thứ tự nạp context và kiểm tra tham số bắt buộc khi compile-time; chỉ thêm Typestate khi các trạng thái thiếu/sai có thể biểu diễn thành kiểu rõ ràng |
| 2 | **Creational** | **Factory Method** | `LlmClientFactory`, `ToolRegistryFactory` | Khởi tạo `Arc<dyn LlmClient>` theo `ProviderProtocol`; tạo registry theo `RunMode` và chỉ đưa tool được phép vào schema |
| 3 | **Creational** | **Prototype / snapshot có chọn lọc** | `ToolContext::fork_for_call`, `SceneVersion` | Clone nhẹ các `Arc` và child `CancellationToken` cho từng tool call; snapshot nội dung scene trước thao tác có thể đảo ngược |
| 4 | **Creational / Architectural** | **Managed DI State** | `tauri::State<AppState>` tại Composition Root | Quản lý rõ vòng đời `SqliteDualPool`, semaphore, `SecretStore`, runtime và event sinks; đơn giản hóa test bằng dependency injection |
| 5 | **Structural** | **Adapter / Ports & Adapters** | `OpenAiCompatAdapter`, `AnthropicAdapter`, `OllamaAdapter`, `SecretStore` adapters | Chuẩn hóa giao thức SSE/NDJSON/thinking về `StreamItem`; domain không phụ thuộc wire protocol hoặc OS keychain |
| 6 | **Structural** | **Decorator / middleware composition** | Retry, tracing/redaction, context-overflow recovery, tool commit guard | Bọc hành vi xuyên suốt quanh `LlmClient` và `ToolExecutor` mà giữ phần nghiệp vụ chính tách biệt, thứ tự middleware được khai báo rõ |
| 7 | **Structural** | **Proxy + scope guard** | `OutboundGuard`, repository truy vấn theo `ToolContext` | Thẩm định endpoint outbound (scheme, DNS/IP, SSRF) và luôn giới hạn dữ liệu theo project qua `project_characters` trước khi trả entity cho tool |
| 8 | **Structural** | **Newtype** | `ProjectId`, `CharacterId`, `SessionId`, `RunId`, `SecretRef` | Phân biệt ID cùng biểu diễn cơ sở bằng kiểu riêng, phát hiện truyền nhầm khi compile-time với chi phí runtime bằng không |
| 9 | **Structural** | **Composite** | Cây `world_settings`, `themes`, `PromptLayer` | Biểu diễn rõ các cấu trúc phân cấp; thêm phát hiện cycle và giới hạn độ sâu khi duyệt cây lấy từ dữ liệu DB |
| 10 | **Structural** | **Facade** | Tauri commands mỏng | Chuẩn hóa ranh giới IPC: validate DTO → gọi use case → trả kết quả/Channel; command không giữ nghiệp vụ hay truy cập repository trực tiếp |
| 11 | **Behavioral** | **Sans-I/O State Machine (enum ADT)** | `RunPhase` + pure transition function | Tách quyết định chuyển trạng thái khỏi I/O; test exhaustive các transition mà không cần DB, mạng hoặc Tauri |
| 12 | **Behavioral** | **Command + reversible execution** | `ToolCommand`, approval batch, SQLite savepoints | Giữ nguyên args gốc và args đã được người dùng sửa; thực thi theo transaction/savepoint và dùng `scene_versions`/soft delete cho thao tác có thể khôi phục |
| 13 | **Behavioral** | **Verification pipeline** | `Schema` → `Scope` → `Domain Rules` → `Postconditions` | Các validator độc lập trả `Pass`, `Retry(feedback)` hoặc `Block(reason)` để lỗi có cấu trúc và dễ kiểm thử |
| 14 | **Behavioral** | **Iterator / async Stream** | `Stream<Item = Result<StreamItem, LlmError>>`, `ToolCallAccumulator` | Chuẩn hóa stream có cancellation/backpressure và chỉ thực thi tool call sau khi đã tích lũy, parse và validate đầy đủ |
| 15 | **Behavioral** | **Strategy** | Token estimation, compaction, memory retrieval | Thay thuật toán qua trait; mặc định FTS5 và tỉa tất định, chỉ bật vector retrieval ở phase sau khi có tiêu chí đo và migration rõ |
| 16 | **Behavioral** | **Observer / EventSink multiplexer** | IPC Channel, append-only trace, `entity_changed` | Một `DomainEvent` được chuyển tới observer tách biệt; lỗi gửi UI không làm mất audit hoặc làm hỏng transaction nghiệp vụ |
| 17 | **Behavioral** | **Memento / version snapshot** | `scene_versions`, prompt manifest, run checkpoints | Khôi phục nội dung scene và kiểm toán context của run; checkpoint phải versioned, có giới hạn dung lượng và không mặc định lưu secrets |
| 18 | **Behavioral** | **Template Method / pipeline** | Attachment ingestion pipeline | Chuẩn hóa validate → hash/dedupe → extract → token estimate/persist, trong khi extraction là hook theo MIME đã được allowlist |
| 19 | **Data Access** | **Repository + Unit of Work** | Reader repositories và writer transaction trên Dual-Pool | Đóng gói query theo aggregate; mọi lệnh ghi đi qua một Unit of Work/transaction có commit guard và quy tắc quyền project |
| 20 | **Domain** | **Specification** | Readiness / phase-gating rules | Tái sử dụng các rule kiểm tra required fields và trả danh sách vi phạm có mã; không nhân đôi logic tính progress giữa Rust và FE |
| 21 | **FE-BE Sync** | **CQRS nhẹ + rAF producer-consumer** | TanStack Query, Zustand, token buffer | TanStack Query giữ server cache; Zustand giữ UI state; buffer token theo frame để tránh render cho từng delta |

### 2.3.1 Creational — khởi tạo đối tượng

**Builder + Typestate — `crates/agent/src/context/builder.rs`**

Chuỗi trạng thái biên dịch: `MissingProject → HasProject → HasSession → ReadyToBuild`; chỉ `ReadyToBuild` có `.build()`.

```rust
impl ContextBuilder<MissingProject> {
    pub fn new() -> Self;
    pub fn with_project(self, id: ProjectId) -> ContextBuilder<HasProject>;
}
impl ContextBuilder<HasProject> {
    pub fn with_session(self, id: SessionId, mode: RunMode) -> ContextBuilder<HasSession>;
}
impl ContextBuilder<HasSession> {
    pub fn with_model_budget(self, cfg: ModelBudget) -> ContextBuilder<ReadyToBuild>;
}
impl ContextBuilder<ReadyToBuild> {
    pub fn add_layer(self, l: PromptLayer) -> Self;
    pub fn add_pinned_memory(self, m: MemoryItem) -> Self;
    pub fn add_history(self, m: ChatMessage) -> Self;
    pub fn set_tool_tokens(self, n: u32) -> Self;
    pub fn build(self) -> Result<BuiltContext, ContextBuildError>;
}
```

Bất biến trong `.build()`:
1. `reserve_output = min(max_output_tokens, 25%W)`; `buffer = 20%W` khi chưa hiệu chỉnh, giảm tới `5%W` khi calibration ổn định; `U = W − reserve_output − buffer`.
2. `system + tool_schemas > 50%U` → `ContextTooSmall` (báo UI chọn model lớn hơn; không nhồi).
3. Lượt hiện tại + attachment mới không vừa → lỗi tương ứng, không cắt tuỳ tiện.
4. Thứ tự nạp: system layers → pinned memory → lượt hiện tại → story directives → retrieved memory → history.

`LlmChatRequestBuilder` chỉ dùng 2 trạng thái thật sự cần (`MissingModel → Ready`); không thêm Typestate cho field optional.

**Factory — `crates/llm/src/factory.rs`, `crates/agent/src/tools/registry.rs`**

```rust
pub fn build_llm_client(
    provider: &AiProviderRecord,          // caller đọc DB, factory không query
    secrets: Arc<dyn SecretStore>,
    http: reqwest::Client,
) -> Result<Arc<dyn LlmClient>, LlmBuildError>;

pub fn build_tool_registry(mode: RunMode, ctx: &ToolContext) -> ToolRegistry;
```

- Mapping: `openai_compat → OpenAiCompatAdapter`, `anthropic → AnthropicAdapter`, `ollama → OllamaAdapter`; protocol lạ → `UnsupportedProtocol`, không fallback ngầm.
- `build_tool_registry`: read tools + `memory_*` + `request_edit_mode` luôn đăng ký; write tools **chỉ** khi `mode == Edit`. Test bắt buộc: schema ở `Explore` không chứa tên write tool nào.

**Prototype — `crates/agent/src/tools/context.rs`**

```rust
#[derive(Clone)]
pub struct ToolContext { project_id: ProjectId, session_id: SessionId, run_id: RunId, mode: RunMode, root: CancellationToken }

pub fn fork_for_call(&self, call_id: CallId) -> ToolCallContext {
    ToolCallContext { parent: self.clone(), call_id, call: self.root.child_token() }
}
```

- Hủy run hủy `root` → lan xuống mọi child. Timeout/cancel một call chỉ hủy `call`, không làm đứt run.
- Snapshot scene: trước `UPDATE scenes`, ghi `scene_versions` (`version_number = max+1`, `scene_source`, `content`, `summary`, `word_count`) trong cùng transaction với update.

**Managed DI State — `src-tauri/src/state.rs`**

```rust
pub struct AppState {
    pub pools: DatabasePools,          // writer + reader
    pub secrets: Arc<dyn SecretStore>,
    pub llm: Arc<dyn LlmClientFactory>,
    pub events: Arc<dyn EventSink>,
    pub run_gate: Arc<Semaphore>,
}
```

Một composition root duy nhất; command nhận `tauri::State<'_, AppState>`. Không `static mut`/`lazy_static`.

### 2.3.2 Structural — ráp nối và bảo vệ

**Adapter — `crates/domain/src/ports.rs`**

```rust
pub trait LlmClient: Send + Sync {
    async fn stream_chat(&self, req: LlmRequest) -> Result<ItemStream, LlmError>;
}
pub trait SecretStore: Send + Sync {
    async fn get(&self, r: &SecretRef) -> Result<Option<String>, SecretError>;
    async fn set(&self, r: &SecretRef, v: &str) -> Result<(), SecretError>;
    async fn delete(&self, r: &SecretRef) -> Result<(), SecretError>;
}
```

Adapter chỉ encode request + normalize event → `StreamItem`; không đọc DB, không retry, không ghi trace. `KeyringSecretStore` (prod) và `InMemorySecretStore` (test) cùng một port.

**Decorator — thứ tự bọc cố định**

```
TracingLlm( RetryLlm( <Adapter> ) )
TracingTool( CommitGuardTool( <Tool> ) )
```

- `RetryLlm`: chỉ retry transient (429/5xx/timeout/stream drop) **trước khi** tool chạy; `ContextOverflow` → compact rồi retry đúng 1 lần; 4xx khác fail-fast.
- `TracingLlm`: redact secret/header, đo `duration_ms`, ghi `run_steps`.
- `CommitGuardTool`: `SAVEPOINT sp_<call_id>` → execute → đọc `agent_runs.status` **trong cùng transaction** → chỉ commit khi `running|awaiting_approval`; ngược lại `ROLLBACK TO` và trả `ToolResult{rejected}`.
- Cấm retry tool write đã có side effect; retry chỉ cho LLM và read tool idempotent.

**Proxy — `crates/infra/src/proxy/`**

- `OutboundGuard`: `is_local = true` → chỉ `http/https` + `localhost`/`127.0.0.1`/`::1`/LAN khai báo; `is_local = false` → bắt buộc `https`, resolve DNS rồi kiểm **toàn bộ IP**, chặn loopback/link-local/private/`169.254.169.254`, giữ IP đã resolve cho kết nối (chống rebinding).
- `ScopedRepo`: scope lấy từ `ToolContext`, không từ args LLM. Nhân vật:

```sql
SELECT c.* FROM characters c
JOIN project_characters pc ON pc.character_id = c.id
WHERE pc.project_id = ? AND pc.deleted_at IS NULL AND c.deleted_at IS NULL
```

**Newtype — `crates/domain/src/ids.rs`**: `ProjectId`, `CharacterId`, `SessionId`, `RunId`, `SecretRef` (`#[serde(transparent)]`). Signature public của repo/tool không dùng `String`/`Uuid` trần.

**Composite — `crates/domain/src/story/world_setting.rs`, `theme.rs`**

- Duyệt cây có `depth_limit = 12` + `visited: HashSet<Id>`; gặp cycle → `HierarchyCycle`, dừng.
- Trả breadcrumb `Realm / World / Region / Facility / Room`; hợp nhất lore/rule gốc → ngọn.
- Reparent: từ chối nếu node đích là con/cháu của chính node đang đổi.

**Facade — `src-tauri/src/commands/`**: mỗi command theo khuôn `parse DTO → authorize scope → use case → map AppError{code,message,details} → DTO`. Command không mở transaction, không gọi repo trực tiếp; `chat_send` nhận `Channel<RunEvent>`.

### 2.3.3 State machine và Command (vòng chạy)

`crates/agent/src/runtime/phase.rs`:

```rust
pub enum RunPhase {
    Idle,
    BuildingContext { run_id: RunId },
    Streaming { run_id: RunId, pending: Vec<RawToolCall> },
    PolicyCheck { run_id: RunId, commands: Vec<ToolCommand> },
    AwaitingApproval { run_id: RunId, batch_id: BatchId, commands: Vec<ToolCommand> },
    VerifyingPre { run_id: RunId, commands: Vec<ToolCommand> },
    Executing { run_id: RunId, index: usize, commands: Vec<ToolCommand> },
    VerifyingPost { run_id: RunId, commands: Vec<ToolCommand> },
    Finalizing { run_id: RunId },
    Completed { run_id: RunId },
    Cancelled { run_id: RunId, reason: String },
    Blocked { run_id: RunId, reason: String },
    Failed { run_id: RunId, error: String },
    Interrupted { run_id: RunId },
}

pub fn transition(current: RunPhase, event: RunEvent)
    -> Result<(RunPhase, Vec<DriverCommand>), TransitionError>;
```

Quy tắc:
1. `transition` thuần: không DB, không mạng, không sleep; I/O nằm ở `DriverCommand` do `run_loop.rs` thực thi tuần tự.
2. Interrupt toàn cục (`UserCancelRequested`, `FatalErrorOccurred`) xử lý **trước** match, ở mọi phase chưa terminal.
3. Mỗi lần đổi phase ghi `run_step` append-only kèm budget usage.
4. Terminal không nhận event mới, trừ `run_continue` cho `Interrupted`.
5. `AwaitingApproval` nhả permit `run_gate`; có quyết định mới acquire lại. Approval không timeout; timeout chỉ cho execution.

`ToolCommand`:

```rust
pub struct ToolCommand {
    pub call_id: CallId,
    pub tool_name: String,
    pub original_args: serde_json::Value,
    pub edited_args: serde_json::Value,   // = original nếu user không sửa
    pub requires_approval: bool,
    pub savepoint: String,                // "sp_<call_id>"
}
```

Thực thi batch:
1. Args đã sửa → validate lại schema + scope + policy; sai → `ToolResult{error}`, không chạy.
2. `SAVEPOINT` → execute → `RELEASE`; lỗi → `ROLLBACK TO`, ghi `run_steps` verdict `block`, có thể dừng batch.
3. Sau batch: post-verify + commit guard → `COMMIT`; fail bất kỳ bước → rollback toàn transaction.
4. Undo không tự động: khôi phục qua `scene_restore_version` và soft delete; `*_delete` luôn cần approval.

### 2.3.4 Stream và Verification

```rust
pub enum StreamItem {
    Text(String),
    Reasoning(String),
    ToolCallDelta { index: usize, call_id: Option<String>, tool_name: Option<String>, args_delta: String },
    Usage(TokenUsage),
    Done(FinishReason),
}
```

- Accumulator gom theo `index`; `finalize()` sắp xếp index tăng dần, parse JSON từng buffer, lỗi → `MalformedJson` và **không dispatch** tool nào.
- Text/reasoning đẩy UI ngay; tool chỉ dispatch sau `Done` + `finalize()` thành công.
- Stream đứt trước `Done`: bỏ buffer, phát `StreamReset{reason}`, retry step nếu budget còn; không resume giữa chừng.

```rust
pub enum Verdict {
    Pass,
    Retry { stage: &'static str, feedback: String, suggested_fix: Option<serde_json::Value> },
    Block { stage: &'static str, reason: String },
}
```

Pipeline dừng ở verdict đầu tiên khác `Pass`, thứ tự: `Schema → Scope → DomainRule → PostCondition`. `Retry` tốn `max_retries_per_tool` và đưa feedback có cấu trúc vào prompt sau; `Block` rollback transaction.

### 2.3.5 Strategy, Observer, Memento, Template Method

```rust
pub trait TokenEstimator { fn estimate(&self, t: &str) -> u32; fn calibrate(&self, est: u32, actual: u32) -> f64; }
pub trait Compactor { async fn compact(&self, input: CompactionInput) -> Result<CompactionOutput, CompactError>; }
pub trait MemoryRetriever { async fn search(&self, scope: MemoryScope, q: &str, limit: usize) -> Result<Vec<MemoryHit>, RetrieveError>; }
```

- Token: `tiktoken-rs` × `token_calibration`; `calib = 0.2×clamp(actual/raw, 0.5, 3.0) + 0.8×calib_cũ`; persist sau mỗi call có `usage`.
- Compaction 3 tầng theo thứ tự: elide raw tool result cũ ngoài cửa sổ 6 message → bỏ retrieved memory điểm thấp (giữ pinned) → tóm tắt lượt cũ (`summary`, `covers: [from_seq, to_seq]`). Không cắt 6 message gần nhất/pinned/locked layer. Lỗi LLM → fallback cắt tất định, run không fail.
- Retrieval P1: FTS5 (`unicode61 remove_diacritics 0`) + recency + pin. P3: `sqlite-vec` + `fastembed-rs` hợp nhất RRF, giữ nguyên trait.

**Observer**: `EventSink::emit(DomainEvent)`; dispatch thứ tự cố định `AuditTrace → IpcChannel → EntityChanged`, mỗi observer cô lập lỗi. Audit dùng kết nối riêng autocommit append-only; `EntityChanged` chỉ cho mutation đã commit (`project_id, entity, ids`).

**Memento**: `scene_versions` unique `(scene_id, version_number)`, tăng đơn điệu trong cùng transaction; `scene_source ∈ {AI_GENERATED, HUMAN_EDITED, RESTORE}`. `run_steps(kind='context')` lưu manifest `{layers[{layer,template_id,version,sha256,tokens}], memory_ids, attachment_ids, token_breakdown}`; prune theo `app_settings` (mặc định 30 ngày / 500 run), không lưu secret hay nội dung file thô. `run_continue` chỉ cho `interrupted`, không replay tool đã thành công.

**Template Method** — `AttachmentPipeline.process(path, mime)`:
1. Validate size ≤ `max_attachment_mb` (25MB) và MIME ∈ {`text/plain`, `text/markdown`, docx, pdf}.
2. SHA-256 dedupe theo `attachments.sha256`; trùng → tái sử dụng row, không extract lại.
3. Extract hook theo định dạng; lỗi → `extract_status = failed` nhưng giữ row.
4. Token estimate + `extract_status = done`; `storage_path` tương đối trong app data dir.

### 2.3.6 Data access, domain rules và FE–BE sync

- `DatabasePools { writer, reader }`: writer `max_connections = 1`; reader `max_connections = 10` + `read_only(true)`.
- PRAGMA mọi connection: `journal_mode=WAL`, `synchronous=NORMAL`, `foreign_keys=ON`, `busy_timeout=5000`, `cache_size=-64000`, `temp_store=MEMORY`.
- Read repo trả DTO đã scope; write qua `UnitOfWork` (`begin` writer, `savepoint`, `commit(guard)`, `rollback`). `CommitGuard` RAII: drop chưa commit → rollback.
- Không mở transaction trong command; audit và business không dùng chung transaction.

```rust
pub struct ValidationReport { pub is_satisfied: bool, pub satisfied: usize, pub total: usize, pub violations: Vec<Violation> }
pub trait Specification<T> { fn validate(&self, v: &T) -> ValidationReport; }
```

- Chỉ ghép `And/Or` khi thật sự cần; rule một-consumer viết `validate_*` thuần.
- `RequireComplete` cho phase-gating: project meta (3), target (3), genre ≥ 1, character init ≥ 1 (mỗi char có name/sex/motif). FE hiển thị `violations[].field_key` và tiến độ từ cùng nguồn BE, không viết lại công thức.

```ts
export const queryKeys = {
  projects: ['projects'] as const,
  project: (id: string) => ['projects', id] as const,
  characters: (id: string) => ['projects', id, 'characters'] as const,
  character: (id: string, cid: string) => ['projects', id, 'characters', cid] as const,
  arcs: (id: string) => ['projects', id, 'arcs'] as const,
  chapters: (id: string, aid: string) => ['projects', id, 'arcs', aid, 'chapters'] as const,
  world: (id: string) => ['projects', id, 'world_settings'] as const,
  scenes: (id: string, cid: string) => ['projects', id, 'chapters', cid, 'scenes'] as const,
};
```

- `entity_changed` → invalidate đúng nhánh; không invalidate toàn bộ `['projects']`.
- Zustand chỉ giữ: stream text/reasoning buffer, panel ratio, active tab, dialog/mode; không cache entity.
- `useTokenStreamRafBuffer`: 2 buffer trong `useRef`, một `requestAnimationFrame` flush cả hai; cleanup khi unmount/cancel; token sau `Done` bị bỏ.

### 2.3.7 Bất biến chung rút ra từ các pattern

1. **Ranh giới crate là hàng rào biên dịch**, không phải quy ước: `domain` không biết `tauri`/`sqlx`/`reqwest`; `agent` không biết wire format.
2. **Scope cưỡng chế trong query**, không kiểm tra sau khi đọc: mọi truy vấn tool nhận `ToolContext` và áp `project_id`, kể cả qua `project_characters`.
3. **Audit tách khỏi nghiệp vụ**: trace append-only dùng kết nối riêng để còn lại khi business transaction rollback.
4. **Không phát cache event trước commit**: `entity_changed` chỉ sau khi Unit of Work commit thành công.
5. **Cây dữ liệu có biên**: duyệt `world_settings`/`themes` phải có cycle detection, `depth_limit` và kiểm tra reparent.
6. **Streaming không dispatch sớm**: tool chỉ chạy sau khi arguments delta tích lũy, parse và validate; UI flush token theo frame.
7. **Không hứa hẹn tuyệt đối**: các pattern giảm thiểu rủi ro (lock contention, stale write, confused deputy) nhưng vẫn phải có test cho transition, retry, cancel và reconcile.

---

## 3. Kiến trúc tổng quan

```
┌──────────────────────── React (WebView) ────────────────────────┐
│ features/*  →  hooks (TanStack Query)  →  shared/lib/ipc        │
└───────────────────────────────┬─────────────────────────────────┘
                                │ invoke / Channel / events
┌───────────────────────────────▼─────────────────────────────────┐
│ src-tauri  commands/ (mỏng)                                      │
├──────────────────────────────────────────────────────────────────┤
│ crates/agent   runtime · context · prompt · tools · verify · mem │
├───────────────┬───────────────────────────┬──────────────────────┤
│ crates/domain │ crates/llm (adapter)      │ crates/infra         │
│ entity + port │ openai-compat/anthropic/  │ sqlite · fs · keyring│
│               │ ollama/custom             │ extractor            │
└───────────────┴───────────────────────────┴──────────────────────┘
```

---

## 4. Cấu trúc thư mục

### 4.1 Repo

```
merlink-harness-ui/
├── src/                     # FE (mục 12)
├── src-tauri/               # Rust (mục 4.2)
├── docs/
│   ├── bussiness-flow.md
│   ├── db.dbml
│   ├── design-system.md
│   ├── plan.md              # file này
│   └── adr/                 # quyết định kiến trúc (mới)
├── package.json
├── vite.config.ts
└── react-router.config.ts   # ssr: false
```

### 4.2 `src-tauri/`

```
src-tauri/
├── Cargo.toml                   # workspace root
├── tauri.conf.json
├── capabilities/default.json    # whitelist tối thiểu (dialog, fs scoped)
├── migrations/                  # 0001_core.sql (từ db.dbml) · 0002_scope.sql (project_id + backfill) · 0003_harness.sql · ...
├── seeds/                       # dữ liệu is_system: genres, motifs, style_presets, directives, narrator_configs...
├── resources/
│   └── prompts/                 # base prompt mặc định (bundle theo app)
│       ├── base.md
│       ├── tools-guide.md
│       └── verify-feedback.md
├── src/
│   ├── main.rs
│   ├── lib.rs                   # build app, đăng ký commands + state + specta
│   ├── state.rs                 # AppState (DI container)
│   ├── error.rs                 # AppError → serialize cho FE (code + message + details)
│   ├── telemetry.rs             # tracing + log file xoay vòng
│   ├── events.rs                # DTO cho Channel/event
│   └── commands/
│       ├── project.rs  session.rs  chat.rs  run.rs
│       ├── memory.rs   attachment.rs  prompt.rs
│       ├── provider.rs model.rs  tool.rs  settings.rs
│       ├── business/            # CRUD cho UI (characters, scenes, arcs, world...)
│       └── mod.rs
└── crates/
    ├── domain/                  # thuần Rust
    │   └── src/
    │       ├── story/           # Project, Arc, Chapter, Scene, Character, Lore, Theme...
    │       ├── session.rs       # Session, Message, Role, Envelope
    │       ├── run.rs           # AgentRun, RunStatus, Step, Verdict
    │       ├── tool.rs          # ToolSpec, ToolCall, ToolResult, ToolPolicy
    │       ├── memory.rs        # ContextMemory, MemoryScope
    │       ├── attachment.rs
    │       ├── prompt.rs        # PromptLayer, PromptTemplate
    │       ├── provider.rs      # Provider, ModelProfile, Capabilities
    │       ├── events.rs        # DomainEvent (RunStarted, ToolCallFinished, EntityChanged...)
    │       ├── ports.rs         # trait: LlmClient, *Repo, SecretStore, FileStore, Extractor, EventSink, Clock, IdGen
    │       ├── ids.rs           # Newtypes: ProjectId, CharacterId, SessionId, RunId, SecretRef
    │       └── specifications/  # composable rules where reused
    ├── agent/
    │   └── src/
    │       ├── runtime/
    │       │   ├── run_loop.rs  # Tokio I/O driver; executes DriverCommand and manages cancellation/budget
    │       │   ├── phase.rs     # Sans-I/O RunPhase + pure transition(phase, event)
    │       │   ├── budget.rs    # RunBudget (step/tool/retry/token/time)
    │       │   ├── scheduler.rs # 1 run/session + semaphore toàn cục
    │       │   └── reconcile.rs # startup: run mồ côi → interrupted
    │       ├── context/{builder,tokens,compactor}.rs # concrete strategies; abstract only when alternatives are needed
    │       ├── prompt/{composer,manifest,render,story_directives}.rs
    │       ├── tools/
    │       │   ├── registry.rs  executor.rs  schema.rs   # registry sau trait ToolProvider
    │       │   ├── context.rs   # ToolContext + fork_for_call; scoped capabilities
    │       │   ├── mode.rs      # RunMode → tập tool được phép (explore/edit)
    │       │   ├── scope.rs     # ToolContext {project_id, session_id, run_id, mode}
    │       │   ├── policy.rs    # quyết định: auto | approve | deny
    │       │   ├── approval.rs  # chờ user duyệt, timeout, edited_args
    │       │   └── builtin/{project,arc,chapter,scene,character,relationship,
    │       │               world,lore,theme,motif,memory,attachment}.rs
    │       ├── verify/{pipeline,schema,rules,postcond}.rs # schema -> scope -> domain -> postcondition
    │       │   └── rules/{scene,arc,world,relationship,theme}.rs
    │       ├── memory/{manager,retrieval}.rs
    │       └── trace.rs
    ├── llm/
    │   └── src/
    │       ├── client.rs        # trait LlmClient + LlmRequest/ItemStream
    │       ├── factory.rs       # build_llm_client(provider, secrets, http) theo provider.protocol
    │       ├── middleware/      # TracingLlm( RetryLlm( adapter ) )
    │       ├── secrets.rs       # KeyringSecretStore + InMemorySecretStore (impl SecretStore)
    │       ├── item.rs          # StreamItem chuẩn hoá: Text | Reasoning | ToolCall | Usage | Done
    │       ├── adapters/
    │       │   ├── chat_completions.rs  # OpenAI-compat: OpenAI, OpenRouter, Groq, vLLM, LM Studio, custom base_url
    │       │   ├── responses.rs         # OpenAI Responses API (tuỳ chọn)
    │       │   ├── anthropic.rs
    │       │   └── ollama.rs
    │       ├── sse.rs           # parse SSE chung
    │       ├── capabilities.rs  # tools? vision? json_mode? reasoning?
    │       └── retry.rs         # phân loại lỗi, backoff 429/5xx (mục 6.1)
    └── infra/
        └── src/
            ├── db/{pool,migrate}.rs # SqliteDualPool: serialized writer + read-only readers
            ├── db/repos/        # 1 repo/aggregate
            ├── fs/{attachment_store,extractor}.rs
            ├── proxy/{outbound_guard,scoped_repo}.rs # SSRF guard + project scope enforcement
            ├── secrets.rs       # keyring
            └── search.rs        # FTS5 (memory, scene, lore)
```

Tuỳ chọn: `crates/cli` — bin headless dùng lại `agent` để chạy kịch bản không GUI (test/eval); có sẵn nhờ GUI không chứa logic.

**Ma trận phụ thuộc (hàng rào là `Cargo.toml`):**

| Crate | Được phụ thuộc | Cấm |
|---|---|---|
| `domain` | `serde`, `specta`, `thiserror`, `async-trait`, `uuid` | `tauri`, `sqlx`, `reqwest`, `keyring` |
| `agent` | `domain`, `tokio`, `tokio-util` (CancellationToken), `tiktoken-rs`, `futures` | `tauri`, `sqlx`, `reqwest` |
| `llm` | `domain`, `reqwest`, SSE parser | `tauri`, `sqlx`, `agent` |
| `infra` | `domain`, `sqlx`, `keyring`, `tracing-appender`, extractor | `tauri`, `agent`, `llm` |
| `src-tauri` | tất cả (composition root: nối port ↔ adapter) | — |

CI chạy `cargo tree -p domain -e normal` và kiểm tra không có crate cấm.

---

## 5. Dữ liệu (bám `db.dbml`)

### 5.1 Đã có, dùng nguyên

| Nhóm | Bảng | Vai trò trong harness |
|---|---|---|
| Chat | `sessions`, `messages` (`payload_json`) | Session theo `project_id`; envelope (tool call/result, usage) nằm trong `payload_json` |
| Business | `projects, arcs, chapters, scenes, scene_versions, scene_characters, scene_lore_entries` | Đích của tool ghi |
| Meta | `characters, character_appearances, relationships, relationship_states, character_competences, character_memories` | Đích của tool đọc/ghi |
| World | `world_settings, lore_entries` | Tool world/lore |
| Theme | `themes, theme_arguments` | Tool theme |
| System prompt (dữ liệu) | `genres, motifs, event_blocks, event_transitions, constraints, motif_constraints, style_presets*, narrator_configs, narrative_directives` | **Nguồn để compile "story directives"** vào prompt (mục 7.3) |

### 5.2 Cần thêm cho harness (migration `0003_harness.sql`)

```dbml
// ---------- PROVIDER & MODEL ----------
enum ProviderProtocol { openai_compat anthropic ollama }

Table ai_providers {
  id uuid [pk, note: 'UUIDv7']
  code varchar [unique]
  display_name text [not null]
  protocol ProviderProtocol [not null]
  base_url varchar [note: 'Cho phép custom endpoint']
  is_local boolean [default: false, note: 'true = Ollama/LM Studio local; dùng cho outbound guard']
  secret_ref varchar [note: 'Khóa trong OS keychain — KHÔNG lưu key ở DB']
  extra_headers_json text
  is_enabled boolean [default: true]
  created_at timestamp
  updated_at timestamp
  deleted_at timestamp
}

Table ai_models {
  id uuid [pk, note: 'UUIDv7']
  provider_id uuid [ref: > ai_providers.id, not null]
  model_id varchar [not null, note: 'Tên gửi lên API, vd: gpt-4.1, llama3.1:8b']
  display_name text [not null]
  context_window int
  max_output_tokens int
  tokenizer varchar [default: 'o200k', note: 'o200k | cl100k | heuristic']
  token_calibration numeric [default: 1.0, note: 'EMA(prompt_tokens thực / ước lượng); cập nhật sau mỗi call']
  supports_tools boolean [default: true]
  supports_vision boolean [default: false]
  supports_json_mode boolean [default: false]
  supports_reasoning boolean [default: false]
  default_params_json text [note: 'temperature, top_p, reasoning_effort...']
  price_in_per_m numeric
  price_out_per_m numeric
  is_custom boolean [default: false]
  is_enabled boolean [default: true]

  indexes { (provider_id, model_id) [unique] }
}

// ---------- PROMPT ----------
enum PromptLayerKind { base project session tools verify }

Table prompt_templates {
  id uuid [pk, note: 'UUIDv7']
  code varchar [unique]
  kind PromptLayerKind [not null]
  display_name text
  content text [not null, note: 'Markdown + biến {{project.language}}, {{tools}}...']
  is_locked boolean [default: false, note: 'Invariant: lớp khác không override được']
  is_system boolean [default: false]
  version varchar
  updated_at timestamp
}

Table prompt_bindings {
  id uuid [pk, note: 'UUIDv7']
  scope_type varchar [note: 'global | project | session']
  scope_id varchar
  template_id uuid [ref: > prompt_templates.id, not null]
  position int [note: 'Thứ tự ghép']
  is_enabled boolean [default: true]
  indexes { (scope_type, scope_id, position) }
}

// ---------- CONTEXT MEMORY (của agent, KHÁC character_memories) ----------
enum MemoryScope { global project session }

Table context_memories {
  id uuid [pk, note: 'UUIDv7']
  scope MemoryScope [not null]
  project_id uuid [ref: > projects.id]
  session_id varchar [ref: > sessions.id]
  content text [not null]
  content_hash varchar [not null, note: 'sha256 của nội dung đã normalize; dùng dedupe exact']
  tags text
  is_pinned boolean [default: false, note: 'Luôn đưa vào context']
  source_message_id varchar [ref: > messages.id]
  origin varchar [note: 'user | agent']
  expires_at timestamp
  created_at timestamp
  updated_at timestamp
  deleted_at timestamp

  indexes { (scope, project_id) (session_id) is_pinned deleted_at (scope, project_id, content_hash) [unique] }
}
// + virtual table FTS5: context_memories_fts(content, tags)

// ---------- ATTACHMENT ----------
Table attachments {
  id uuid [pk, note: 'UUIDv7']
  project_id uuid [ref: > projects.id]
  file_name text [not null]
  mime varchar
  size_bytes bigint
  sha256 varchar [unique, note: 'Dedupe']
  storage_path varchar [not null, note: 'Tương đối trong app data dir']
  extract_status varchar [note: 'pending | done | failed | unsupported']
  extracted_text text
  token_estimate int
  created_at timestamp
  deleted_at timestamp
}

Table message_attachments {
  id uuid [pk]
  message_id varchar [ref: > messages.id, not null]
  attachment_id uuid [ref: > attachments.id, not null]
  indexes { (message_id, attachment_id) [unique] }
}

// ---------- RUN / TRACE ----------
enum RunStatus { running awaiting_approval completed failed cancelled blocked interrupted }

Table agent_runs {
  id varchar [pk, note: 'run_<id>']
  session_id varchar [ref: > sessions.id, not null]
  trigger_message_id varchar [ref: > messages.id]
  model_id uuid [ref: > ai_models.id]
  status RunStatus
  mode varchar [note: 'explore | edit']
  approval_policy varchar [note: 'ask | auto_reversible']
  budget_json text [note: 'RunBudget lúc bắt đầu']
  error text
  tokens_in int
  tokens_out int
  started_at timestamp
  finished_at timestamp
  indexes { session_id started_at }
}

Table run_steps {
  id uuid [pk]
  run_id varchar [ref: > agent_runs.id, not null]
  seq int [not null]
  group_id varchar [note: 'Gom các call cùng một ApprovalRequest / wave']
  kind varchar [note: 'context | llm | tool_call | verify | approval | memory']
  name varchar
  input_json text
  output_json text
  verdict varchar [note: 'pass | retry | block']
  duration_ms int
  created_at timestamp
  indexes { (run_id, seq) [unique] }
  Note: 'Append-only: không UPDATE/DELETE. Step `context` chứa prompt manifest.'
}

Table tool_policies {
  tool_name varchar [pk]
  mode varchar [note: 'auto | approve | deny']
  scope_type varchar [default: 'global']
  scope_id varchar
}

Table app_settings {
  key varchar [pk]
  value_json text
}
```

### 5.3 Chỉnh nhỏ trên bảng hiện có

| Bảng | Chỉnh | Lý do |
|---|---|---|
| `sessions` | `+ updated_at, + default_model_id, + archived_at` | Sắp xếp history, nhớ model theo session |
| `messages` | `+ run_id, + seq` | Sắp xếp ổn định, gắn message ↔ run |
| `characters` | Giữ nguyên độc lập (Identity). Tạo bảng liên kết N-N **`project_characters`** (kèm `role`, `notes`, `created_at`, `deleted_at`) | Bám chuẩn `docs/db.dbml`. Cho phép nhân vật xuất hiện linh hoạt trong nhiều truyện của cùng vũ trụ tác giả |
| `lore_entries` | **`+ project_id`** (nullable: null = template dùng chung, agent chỉ đọc) | Khớp với `world_settings.project_id` |

### 5.4 Scope theo project & backfill (gate của Phase 0)

Mọi entity tool chạm tới phải resolve được `project_id`:

| Entity | Resolve |
|---|---|
| arcs | `arcs.project_id` |
| chapters | `arc_id → arcs.project_id` |
| scenes | `chapter_id → chapters → arcs` |
| themes | `arc_id → arcs` |
| characters | qua bảng liên kết **`project_characters`** (`project_id = ctx.project_id` và `deleted_at IS NULL`) |
| lore_entries, world_settings | `project_id` trực tiếp (null = template, read-only với agent) |
| relationships | qua `from_character_id` và `to_character_id` đều thuộc `project_characters` của project này |
| character_memories / competences / appearances | qua `character_id` |

`ScopeResolver::owns(kind, id)` (trong `tools/scope.rs`) phục vụ verify; ngoài ra **repo method dành cho tool nhận `ToolContext` và đưa điều kiện project vào `WHERE`** (ví dụ với nhân vật: `JOIN project_characters pc ON pc.character_id = c.id WHERE pc.project_id = $ctx.project_id AND pc.deleted_at IS NULL`) — không chỉ kiểm tra sau khi đã đọc.

**Backfill (`0002_scope.sql`):**
1. Tạo bảng liên kết `project_characters (id, project_id, character_id, role, notes, created_at, deleted_at)`.
2. Nhân vật có scene → tự động gán liên kết vào project của scene đầu tiên (`scene_characters → scenes → chapters → arcs`).
3. Nhân vật không gắn scene nào → gán liên kết vào project mặc định đầu tiên hoặc project `legacy`.
4. `lore_entries.project_id` lấy từ `world_settings.project_id`, không có thì null.
5. Hỗ trợ đầy đủ việc tái sử dụng nhân vật giữa các tập truyện trong cùng vũ trụ sáng tác mà không cần nhân bản trùng lặp thực thể.

### 5.5 Message envelope (`messages.payload_json`, v1)

Rust enum có `#[derive(specta::Type)]`, sinh type cho FE; BE và FE dùng chung một định nghĩa.

```
MessageEnvelope { v: 1, parts: Part[], usage?: Usage, model_id?: string, run_id?: string }

Part =
  | { type: "text",       text }
  | { type: "reasoning",  text }
  | { type: "tool_call",  call_id, name, args }
  | { type: "tool_result",call_id, ok, content, truncated? }
  | { type: "attachment", attachment_id }
  | { type: "summary",    covers: [seq_from, seq_to] }      // do compactor sinh

role ∈ user | assistant | tool | summary
```

- `messages.content` = phần chiếu text (nối các `text` part) để hiển thị và FTS; chi tiết nằm ở `payload_json`.
- Part lạ (version mới hơn) → FE render fallback, không crash. Đổi cấu trúc = tăng `v` + migration đọc.
- Compactor **không xoá** message cũ; ContextBuilder thay đoạn được `summary.covers` bao phủ bằng chính summary.

---

## 6. Provider & Model (cắm API key, customize)

**Luồng cấu hình (UI `settings/AIProvidersTab` + `ProviderForm`):**
1. Chọn **protocol**: `openai_compat` / `anthropic` / `ollama`.
2. Nhập `base_url` (tuỳ chọn, có preset) và **API key** → lưu vào keychain, DB chỉ giữ `secret_ref`.
3. **Test connection** (command `provider_test`) + **fetch danh sách model** (nếu provider hỗ trợ `/models`).
4. **Custom model**: thêm tay `model_id`, `context_window`, `max_output_tokens`, capability flags, default params, giá.
5. `ModelSelectorPopover` chọn model theo session; fallback về model mặc định.

**Capability-aware runtime:**
- `supports_tools = false` → model chỉ dùng được **chat thuần** (không tool, không `edit`). `ModelSelectorPopover` gắn badge và disable model này cho run cần tool. **Không làm prompt-based tool calling** (ADR-005): hệ thống con này cần format, parse, retry và approval riêng, không đáng với phạm vi hiện tại.
- `supports_vision = false` → attachment ảnh bị từ chối/ẩn.
- `context_window` → đầu vào cho token allocation (mục 7.2). `context_window < 16k` → disable `edit` mode và cảnh báo trên UI.

**Adapter:** mỗi protocol chỉ encode request và normalize stream thành `StreamItem`; runtime không biết wire format. `SecretStore` cũng là port với adapter production (`KeyringSecretStore`) và adapter test (`InMemorySecretStore`).

**Bảo mật:** key không bao giờ trả về FE; FE chỉ nhận `has_key: bool`. Log/trace phải redact header `Authorization`. Provider có `is_local`; khi gửi dữ liệu project/attachment tới provider không local, UI hiện badge "cloud", và `prompt_preview` cho thấy đúng nội dung sẽ gửi.

### 6.1 Lỗi LLM & retry (`llm/retry.rs`)

| Tình huống | Hành vi |
|---|---|
| 429 | Backoff mũ + jitter, tôn trọng `Retry-After`; tối đa `max_llm_retries` (3) |
| 5xx, timeout, mất kết nối | Như trên |
| 4xx khác (401/403/400/404) | Fail fast → `RunFailed{code}` (key sai, model sai...), không retry |
| Stream đứt giữa chừng | **Không resume.** Tool chỉ chạy *sau khi* stream của step kết thúc, nên đứt stream = chưa có tool nào chạy → bỏ phần đã stream, emit `StreamReset{reason}`, chạy lại step |
| Provider báo context overflow | Cắt/compact thêm rồi thử lại **1 lần**; vẫn lỗi → `RunFailed{CONTEXT_OVERFLOW}` |
| Cancel | Dừng retry ngay (CancellationToken) |

Mỗi lần retry ghi một `run_step` (kind `llm`, verdict `retry`). Retry LLM tính vào `max_llm_retries`, tách khỏi `max_retries_per_tool`. `RunFailed` mang `code` + thông điệp đọc được.

---

## 7. Agent runtime

### 7.1 Run: mode, budget, state machine

**Run mode** (capability, không phải prompt):

| Mode | Tool có mặt trong run | Dùng khi |
|---|---|---|
| `explore` (mặc định) | Tool đọc + `memory_*` + `attachment_read` + `request_edit_mode(reason)` | Hỏi đáp, phân tích, lập kế hoạch |
| `edit` | `explore` + tool ghi (mục 8) | User bật, hoặc duyệt `request_edit_mode` |

Prompt injection trong file đính kèm khi ở `explore` không thể ghi dữ liệu vì tool ghi không tồn tại trong run.

**Approval policy** (độc lập với mode):
- `ask` (mặc định): mọi tool ghi cần duyệt.
- `auto_reversible`: tự chạy tool ghi đảo ngược được (tạo mới, sửa có `scene_versions`). `*_delete` và ghi đè lên bản `HUMAN_EDITED` luôn hỏi.

**Approval request (batch theo lượt):** trong một lượt assistant, mọi tool ghi được gom thành **một** `ApprovalRequest { approval_id, calls[] }` (kèm diff từng call). User duyệt/loại từng call hoặc "duyệt tất cả"; call bị loại trả `ToolResult{rejected}` cho LLM. Giữa các lượt có lựa chọn **"Duyệt phần còn lại của run"**, chỉ áp cho tool đảo ngược được (không áp `*_delete`, không đè bản `HUMAN_EDITED`). Dữ liệu đã hỗ trợ batch từ đầu (`run_steps.group_id`); UI ban đầu có thể hiển thị tuần tự.

**Approval không có timeout:** run giữ `awaiting_approval` vô thời hạn và **nhả slot semaphore**; timeout (mặc định 30s) chỉ áp cho *thực thi tool*.

**RunBudget** (runtime enforce; cấu hình theo session/model): `max_steps` (25), `max_tool_calls` (40), `max_retries_per_tool` (2), `max_llm_retries` (3), `max_tokens_total`, `max_duration`. Chạm 80% → `BudgetWarning`; vượt → run `blocked` (reason = budget).

**State machine (Sans-I/O Functional Core + Async Driver):** logic `transition(phase, event)` là hàm thuần trả `Result<(RunPhase, Vec<DriverCommand>), TransitionError>`; `run_loop.rs` thực thi command I/O bằng Tokio, `CancellationToken` và `EventSink`. Nhờ vậy có thể kiểm thử exhaustive mọi transition không cần DB/mạng. Mỗi lần đổi phase ghi append-only `run_step`. Định nghĩa `RunPhase`/`RunEvent`/`DriverCommand` và quy tắc chuyển: mục 2.3.3.

```
Idle → BuildingContext → Streaming ─┬─ final answer → Finalizing → Completed
                                    └─ tool_call → PolicyCheck
PolicyCheck → (approve) AwaitingApproval ⇄ approve | reject | edit args
            → (auto)    VerifyingPre
VerifyingPre → Executing → VerifyingPost
VerifyingPost ─ pass  → BuildingContext (vòng tiếp)
              ─ retry (≤ budget) → BuildingContext (kèm feedback có cấu trúc)
              ─ block → RolledBack → Blocked
Bất kỳ phase: cancel → Cancelled · vượt budget → Blocked · lỗi → Failed
```

**Độ bền:**
- **Scheduler:** 1 run active / session (`chat_send` khi đang chạy → `RUN_ACTIVE`); semaphore toàn cục; run chờ approval không giữ slot.
- **Active Compute Timer:** `max_duration` chỉ tính thời gian agent thực sự chạy; tạm dừng khi `AwaitingApproval`, tiếp tục khi nhận quyết định và phát `BudgetWarning` tại ngưỡng cấu hình.
- **Sans-I/O core:** `RunPhase` không sở hữu I/O handle hoặc DB transaction; các `DriverCommand` mô tả yêu cầu I/O để driver thực thi.
- **Commit guard:** tool ghi chỉ commit nếu `agent_runs.status` còn `running`/`awaiting_approval` *trong cùng transaction* → run đã cancel/interrupted không ghi muộn.
- **Startup reconcile:** run còn `running`/`awaiting_approval` khi mở app → `interrupted`; UI cho "Tiếp tục" (`run_continue`: run mới từ history, kèm tóm tắt step đã xong).
- **Compact recovery:** compactor lỗi → fallback cắt history theo budget, không làm fail run.
- **Sau final:** MemoryManager đề xuất memory; đóng trace.

### 7.2 Context builder & budget

`ContextBuilder` dùng Builder API để làm rõ các input cần thiết; chỉ dùng Typestate cho các invariant bắt buộc có thể chứng minh hữu ích ở compile-time. Prompt layers được lắp ráp theo thứ tự precedence, sau đó cấp phát token budget và ghi `PromptManifest` có version/hash vào `run_steps`.

Thứ tự ưu tiên khi cắt (giữ cao → bỏ trước thấp):

1. System layers (base → project → session → tools/verify)
2. Pinned memories
3. Turn hiện tại + attachment user vừa gắn
4. Story directives đã compile cho tác vụ (mục 7.3)
5. Memory truy xuất theo độ liên quan (FTS5 + recency)
6. History gần nhất (nguyên văn)
7. History cũ → **tóm tắt** (compactor), lưu thành summary message

`context/tokens.rs` trả về **breakdown theo segment** (system / memory / history / attachments / tools) để `ContextUsagePopover` hiển thị.

Công thức EMA, ngưỡng và thứ tự nén chi tiết: mục 2.3.1 và 2.3.5.

**Token allocation policy** (W = `context_window`):

- `reserve_output = min(max_output_tokens, 25% W)`; `buffer` = 20% lúc chưa hiệu chỉnh, hạ dần (tối thiểu 5%) khi `token_calibration` ổn định.
- `U = W − reserve_output − buffer` (token dùng được cho input). **Tool schema tính riêng** vì có thể lớn; lọc theo `RunMode` trước khi tính.

| Segment | Trần (% của U) | Khi vượt |
|---|---|---|
| System layers (locked trước) | 30% | Bỏ lớp tuỳ chọn: ví dụ trong tools-guide → story directives rút gọn |
| Tool schemas | 15% | Chỉ nạp tool của mode hiện tại; nhóm tool nhỏ hơn |
| Pinned memory | 10% | Giữ cái mới/cập nhật gần nhất, báo "N memory bị bỏ" trên UI |
| Memory truy xuất | 10% | Cắt theo điểm liên quan |
| Attachments | 25% | Chuyển sang tóm tắt + `attachment_read` theo đoạn |
| Summary history cũ | 10% | Compact lại |
| History gần nhất + lượt hiện tại | Phần còn lại | Xem thuật toán |

**Thuật toán (greedy theo độ ưu tiên, dừng khi hết budget):**
1. Tính phần cố định: system + tool schemas + lượt user hiện tại (+ attachment vừa gắn).
2. Nếu `system + tool schemas > 50% U` hoặc lượt hiện tại không vừa → `CONTEXT_TOO_SMALL`, báo user và gợi ý model có context lớn hơn. Không cố nhồi.
3. Thêm pinned memory → memory truy xuất → summary → history.
4. History: **luôn giữ nguyên văn 6 message gần nhất**. Nếu vẫn vượt, lược thân `tool_result` cũ trước (`[đã lược, N token]`), rồi mới bỏ message xa hơn (đã có summary thì thay bằng summary).

**Đếm token (ADR-003):** `tiktoken-rs` (`o200k` mặc định, chọn theo `ai_models.tokenizer`) làm xấp xỉ; sau mỗi call lấy `usage.prompt_tokens` thật cập nhật `token_calibration` (EMA). Nội dung tiếng Việt thường tốn token hơn tiếng Anh và sai số lớn hơn với tokenizer lạ → dùng buffer 20% ban đầu thay vì 10–15%. Provider không trả usage (một số model local) → heuristic theo ký tự + buffer lớn hơn. Provider báo overflow → xử lý theo mục 6.1.

### 7.3 Prompt composer (nhiều lớp)

`PromptComposer` triển khai theo Strategy/Template-style composition: mỗi prompt layer có renderer riêng, còn composer áp dụng precedence, locked-layer invariant và manifest. Giữ implementation đơn giản; không tạo framework template tổng quát nếu các lớp cấu hình hiện tại chưa cần thay đổi runtime.

| Lớp | Nguồn | Ví dụ |
|---|---|---|
| `base` | `resources/prompts/base.md` hoặc template user override | Vai trò agent, quy tắc chung |
| `project` | `projects` (language, status, genres, summary) | "Dự án VI, thể loại fantasy, đang DRAFTING" |
| `session` | `prompt_bindings(scope=session)` | Hướng dẫn riêng cho phiên |
| `story directives` | Compile từ `style_presets` + `narrator_configs` + `narrative_directives` + `motifs/constraints` của scene/arc đang làm | Khi viết/sinh scene |
| `tools` | Tự sinh từ `ToolSpec` + `tools-guide.md` | Hướng dẫn dùng tool |
| `verify` | `verify-feedback.md` | Format báo lỗi khi retry |

Quy ước bắt buộc đưa vào base prompt: `code` là slug nội bộ, **không hiển thị cho người dùng**; dùng `display_name`.

**Precedence (cố định):** `base` < `project` < `session` < `story directives` — lớp cụ thể hơn thắng khi cấu hình cùng một thuộc tính. `is_locked` là invariant/guard riêng, không phải tầng precedence: các quy tắc bất biến (quy ước `code`/`display_name`, ngôn ngữ dự án, tool contract, safety rules) phải còn nguyên trong prompt cuối cùng; nội dung project/session không được ghi đè.

**Prompt manifest:** mỗi lần build context, lưu vào step `context`: `{layer, template_id, version, sha256, tokens}` + id memory/attachment được nạp. Khi agent làm sai có thể truy vết lớp/nguồn nào đã vào prompt; `prompt_preview` trả đúng nội dung sẽ gửi.

---

## 8. Tool catalog (business data)

**Quy ước:** tool **không nhận `project_id`/`session_id` từ args**; executor inject `ToolContext` (`scope.rs`) lấy từ run → agent không thể chạm project khác. Tên tool dạng `<entity>_<verb>`. Tool được lọc theo `RunMode` *trước khi* đưa cho LLM (mục 7.1). Registry nằm sau trait `ToolProvider` để thêm nguồn tool khác (vd MCP) mà không đổi runtime.

| Nhóm | Tool (ví dụ) | Bảng chính | Mặc định |
|---|---|---|---|
| Project | `project_get`, `project_update_summary` | projects, project_genres | read: auto · write: approve |
| Arc | `arc_list/get/create/update/reorder`, `arc_set_motif`, `arc_set_event_blocks` | arcs, arc_motifs, arc_event_blocks | write: approve |
| Chapter | `chapter_list/get/create/update/reorder` | chapters | write: approve |
| Scene | `scene_list/get`, `scene_create`, `scene_update_summary`, `scene_write_content`, `scene_restore_version` | scenes, scene_versions, scene_characters, scene_lore_entries | `write_content`: approve |
| Character | `character_search/get/create/update`, `character_add_memory`, `character_set_competence`, `project_character_link/unlink` | characters, project_characters, character_appearances, character_memories, character_competences | write: approve |
| Relationship | `relationship_get/upsert`, `relationship_record_state` | relationships, relationship_roles, relationship_states | write: approve |
| World | `world_tree`, `world_get/create/update` | world_settings | write: approve |
| Lore | `lore_search/get/create/update` | lore_entries | write: approve |
| Theme | `theme_get/create/update`, `theme_set_argument` | themes, theme_arguments | write: approve |
| Reference (read-only) | `motif_search`, `genre_list`, `style_preset_get`, `narrator_config_get`, `directive_list` | bảng system | auto |
| Memory | `memory_search`, `memory_save`, `memory_forget` | context_memories | `save`: auto (có thể bật approve) |
| Attachment | `attachment_read(id, range)` | attachments | auto |
| Mode | `request_edit_mode(reason)` | — | chỉ có ở `explore`; luôn hỏi user |
| Delete | `*_delete` | — | **chỉ soft delete** (`deleted_at`), luôn approve |

**Nguyên tắc tool:**
- Registry tạo tập tool từ `RunMode`; executor kiểm tra capability lần hai. `ToolContext` được tiêm từ run, không lấy `project_id`/`session_id` từ arguments của model.
- Tool calls trong cùng lượt được gom thành approval batch; mỗi call giữ `original_args`, `edited_args` và diff preview. Edited args phải validate lại schema, scope và policy trước khi chạy.
- Chỉ phát `entity_changed` sau khi Unit of Work commit thành công; rollback/reject không invalidate cache như thể dữ liệu đã ghi.
- Read tool trả kết quả **gọn** (id + `display_name` + trường cần thiết); có phân trang/`limit`.
- Write tool trả **diff** (before/after) để UI hiển thị trong approval.
- Mọi tool ghi chạy trong **một transaction**; verify.post fail → rollback.
- `scene_write_content` luôn tạo `scene_versions` mới (`scene_source = AI_GENERATED`, `version_number + 1`) và **bị chặn nếu `scenes.is_locked = true`**.

---

## 9. Verify pipeline

`Verifier` trả `Verdict { Pass | Retry(feedback) | Block(reason) }`, chạy theo chuỗi (định nghĩa kiểu và quy tắc dừng: mục 2.3.4):

1. **Schema**: args khớp JSON schema của `ToolSpec`; enum hợp lệ theo DBML (`LocationType`, `RevealDepth`, `Language`...).
2. **Scope**: mọi id tham chiếu thuộc `session.project_id`.
3. **Domain rules** (bám schema hiện có; validator độc lập chạy theo thứ tự trong Verification Pipeline):

| Rule | Mức |
|---|---|
| `code` là slug (snake_case), unique; không dùng `code` làm nội dung hiển thị | Retry |
| Scene `is_locked` → cấm ghi đè | **Block** |
| `world_settings`: cha phải đúng thứ bậc `realm > world > region > country > city > town > facility > street > room`; không tạo vòng | Retry |
| `arc_motifs`: mỗi arc tối đa 1 motif plot | Retry |
| `arc_event_blocks`: `start_chapter.position ≤ end_chapter.position`, cả hai chapter thuộc arc/project; event block thuộc đúng motif | Retry |
| `event_transitions`: `from`/`to` cùng motif | Retry |
| `scene_characters`: nếu `narrator_config.pov_scope = SINGLE_POV` thì tối đa 1 `is_pov` | Retry |
| `scene_lore_entries`: `PAYOFF` cần `FORESHADOW` ở scene trước; `reveal_to_pov > reveal_to_reader` → cảnh báo | Warn/Retry |
| `character_competences`: bắt buộc `scene_id`; `level` thuộc đúng `domain` | Retry |
| `relationship_states`: `from ≠ to`; giá trị thuộc `RelationDimensionLevel` | Retry |
| `theme_arguments`: (VFILL, phase sau) nhân vật có `required_knowledge/competence` | Warn |

4. **Post-condition**: đọc lại dữ liệu sau ghi trong cùng Unit of Work, so với intent (số dòng, FK, version tăng); chỉ commit khi mọi handler pass và CommitGuard xác nhận run còn active.
5. **Output verify (tuỳ chọn)**: nội dung scene sinh ra khớp `language`, `word_count` hợp lý, tuân thủ `narrative_directives`/`style_preset` (rule-based trước, LLM-judge sau).

Giới hạn retry/step nằm trong `RunBudget` (mục 7.1). `Block` → rollback toàn bộ transaction của tool.

---

## 10. Memory

| Loại | Nơi lưu | Ai dùng |
|---|---|---|
| **Context memory** (agent) | `context_memories` (scope global/project/session) | ContextBuilder |
| **Character memory** (truyện) | `character_memories` | Tool `character_add_memory`, dữ liệu truyện — **không trộn** với context memory |

**Quản lý (FE `features/memory`):** list/search/lọc theo scope, thêm/sửa/xoá, **pin**, đặt TTL, xem nguồn (message gốc), bật/tắt "agent được tự ghi".

**Memory manager:**
- Ghi: do user, hoặc agent qua `memory_save` (origin = `agent`, hiển thị để user duyệt/xoá).
- Dedupe: **chỉ exact/near-exact** — normalize (NFC, lowercase, bỏ dấu câu/khoảng trắng thừa) → `content_hash` unique trong scope. Trùng ngữ nghĩa hoặc mâu thuẫn **không** được phát hiện ở giai đoạn này (cần embedding). Bù lại: `memory_save` trả kèm top-3 memory gần nhất (FTS5) để agent chọn cập nhật thay vì tạo mới; memory do agent ghi hiện ra cho user duyệt/xoá.
- Truy xuất: pinned → FTS5 theo query turn hiện tại → recency, cắt theo budget.
- Quên: soft delete; tôn trọng `expires_at`.

---

## 11. Attachment

1. FE chọn file qua dialog của Tauri → gửi **path** cho Rust (không đẩy bytes qua IPC).
2. Rust copy vào `app_data/attachments/<sha256>`, dedupe theo `sha256`.
3. `extractor`: txt/md trực tiếp (giai đoạn đầu); **docx** tiếp theo (người viết truyện hay dùng Word); **pdf** sau khi đánh giá kỹ crate; ảnh chỉ khi `supports_vision`. Extractor chạy ngoài loop (blocking thread + timeout); file chưa `extract_status = done` thì không đưa vào context.
4. Lưu `extracted_text`, `token_estimate`; trạng thái `extract_status` emit về UI.
5. ContextBuilder: file nhỏ → nhúng thẳng; file lớn → chỉ đưa tóm tắt/mục lục, agent đọc theo đoạn bằng `attachment_read`.
6. Giới hạn: dung lượng tối đa, whitelist mime, quét path traversal.

---

## 12. IPC contract

**Commands (rút gọn):**

| Nhóm | Command |
|---|---|
| Project | `project_list/get/create/update/delete` |
| Session | `session_list(project_id)`, `session_create`, `session_rename`, `session_archive`, `message_list(session_id, cursor)` |
| Chat | `chat_send(session_id, input, mode?, on_event: Channel<RunEvent>)`, `chat_cancel(run_id)` |
| Approval | `approval_decide(run_id, approval_id, decisions[{call_id, decision, edited_args?}] \| approve_all \| approve_rest_of_run)` |
| Memory | `memory_list/create/update/delete/pin` |
| Attachment | `attachment_add(paths)`, `attachment_list`, `attachment_delete` |
| Prompt | `prompt_list/get/save/bind/preview` (preview = prompt đã compose) |
| Provider/Model | `provider_*`, `provider_test`, `model_list/upsert/delete`, `model_fetch_remote` |
| Business | `character_*`, `scene_*`, `arc_*`, `world_*`... (UI sửa trực tiếp, không qua agent) |
| Trace | `run_get(run_id)`, `run_steps(run_id)`, `run_continue(run_id)` (cho run `interrupted`) |

**`RunEvent` (stream):**

```
RunStarted { run_id, mode, budget }
ContextBuilt { usage: { system, memory, history, attachments, tools, total, limit } }
Token { text }
ToolCallProposed { call_id, name, args }
ApprovalRequired { approval_id, calls: [{call_id, name, diff}] }
ToolCallStarted / ToolCallFinished { call_id, result | error }
VerifyResult { call_id, verdict, feedback? }
MemorySaved { id }
ModeChangeRequested { reason }      # agent xin nâng explore → edit, chờ user
ModeChanged { mode }
StreamReset { reason }              # lỗi mạng/5xx: bỏ phần đã stream của step, thử lại
BudgetWarning { kind, used, limit } # ở 80% mỗi giới hạn
RunFinished { status, usage }
RunFailed { error }
```

**Event toàn cục** (để invalidate cache): `entity_changed { project_id, entity, ids }` — phát sau khi tool/UI ghi commit thành công; FE invalidate query keys có scope project. `RunEvent`, `entity_changed` và trace là observer của `EventSink` (từ `DomainEvent`), không phát tay rải rác. Observer được cô lập lỗi; audit trace ghi riêng để còn lại khi business transaction rollback. Thứ tự dispatch và chi tiết observer: mục 2.3.5.

---

## 13. Frontend

### 13.1 Cấu trúc đích

```
src/
├── app/                              # giữ (providers, router, layouts)
├── features/
│   ├── chat/
│   │   ├── components/               # ChatView, ChatInputBar, ChatMessageItem, ToolCallsPopover, ...
│   │   ├── api/                      # useSendMessage, useRunStream, useMessages
│   │   ├── store.ts                  # ephemeral UI state: run, stream buffers, panel/tabs
│   │   └── index.ts
│   ├── sessions/                     # MỚI: list/tạo/đổi tên/archive theo project
│   ├── memory/                       # MỚI: quản lý context memory
│   ├── attachments/                  # MỚI: picker, chip, trạng thái extract
│   ├── agent-runs/                   # MỚI: timeline, approval dialog, verify result
│   ├── system-prompt/                # giữ (data prompt) + thêm tab Templates/Bindings/Preview
│   ├── settings/                     # AIProvidersTab, ModelsTab (MỚI), ProviderForm
│   └── projects/ characters/ draft/ worldbuilding/ navigation/   # giữ
├── shared/
│   ├── lib/
│   │   ├── ipc/{bindings.ts,client.ts,channel.ts}   # DUY NHẤT được import @tauri-apps/api
│   │   └── query-client.ts           # TanStack Query quản lý server state/cache
│   ├── components/ui/ hooks/ i18n/ utils/           # giữ
│   └── services/                                    # bỏ (thay bằng lib/ipc)
└── pages/                                           # giữ mỏng
```

### 13.2 Ánh xạ component hiện có → backend

| Component hiện có | Nối vào |
|---|---|
| `ChatView`, `ChatInputBar` | `chat_send`, `RunEvent`; toggle **Explore/Edit** + approval policy |
| `ContextUsagePopover` | `ContextBuilt.usage` |
| `ModelSelectorPopover` | `model_list`, `session.default_model_id` |
| `ToolCallsPopover`, `ProcessDetailDialog` | `run_steps`, `ToolCallProposed/Finished`, `VerifyResult` |
| `SidebarHistory` | `session_list(project_id)` |
| `ProjectChatPane`, `CharacterChatPane` | chat với `project_id` (+ `focus` = character/scene đang mở) |
| `AIProvidersTab`, `ProviderForm` | `provider_*`, keychain |
| `RecycleBinTab` | các entity có `deleted_at` + `restore` |
| `system-prompt/*` Tabs | business CRUD bảng system (`is_system` read-only) |
| `mockData.ts`, `mockProjects.ts` | thay bằng hook IPC; giữ mock cho test/Storybook |

### 13.3 Quy tắc
- Chỉ `shared/lib/ipc` import `@tauri-apps/api`. Feature gọi qua hook.
- Cây `queryKeys` phân cấp theo project/entity và cách invalidate: mục 2.3.6; không invalidate toàn bộ `['projects']` khi chỉ một entity đổi.
- `bindings.ts` sinh tự động, **không sửa tay**, commit vào repo.
- Streaming: buffer text/reasoning riêng trong `useRef`, flush bằng `requestAnimationFrame` tối đa một lần mỗi frame; cleanup RAF khi unmount/cancel. Không thực thi tool đến khi arguments delta đã tích lũy, parse và validate đầy đủ.
- Approval là UI chặn: chạy tool ghi chỉ tiếp tục sau `approval_decide`. Chờ vô thời hạn; run đang chờ phải hiện rõ trong sidebar. Khi session có run active, `ChatInputBar` chuyển sang nút Dừng (BE vẫn trả `RUN_ACTIVE` nếu bị gọi).
- FE không giữ state nghiệp vụ làm nguồn sự thật: TanStack Query giữ server state/cache; Zustand chỉ giữ ephemeral UI state (stream buffer, panel ratio, active tabs, dialogs).

---

## 14. Bảo mật & vận hành

- `capabilities/`: chỉ mở `dialog` + `fs` scoped (app data dir); không mở `shell`, không `fs` toàn cục.
- CSP chặt; không `eval`; WebView không gọi LLM trực tiếp (mọi request ra ngoài đi qua Rust).
- **Outbound guard:** redact secret/header trước khi gửi và trước khi lưu trace; badge cloud/local + `prompt_preview` (mục 6). Không có Privacy Engine riêng.
- Attachment ingestion chạy trong app data scope: whitelist MIME/size → SHA-256 dedupe → trích xuất theo định dạng cho phép → token estimate/persist; không dùng path do model cung cấp.
- Backup DB: checkpoint WAL trước khi copy file SQLite; nút Export/Import trong Settings.
- Auto-update (Tauri updater) — để sau.
- Telemetry: mặc định tắt.
- **Observability (log hệ thống, tách khỏi `run_steps`):** `tracing` + `tracing-subscriber` + `tracing-appender`; file ở `app_data/logs/`, xoay theo ngày, giữ 7 ngày; level cấu hình trong Settings (`app_settings`), mặc định `info`. Span mang `run_id`/`session_id` để nối log ↔ trace. Không log nội dung prompt/scene ở `info`; secret luôn redact. Có panic hook ghi log và nút "Mở thư mục log".

---

## 15. Rủi ro & câu hỏi mở

| # | Vấn đề | Đề xuất |
|---|---|---|
| 1 | Liên kết `characters` và `projects` theo N-N | Tạo bảng nối `project_characters` trong migration `0002_scope.sql` + backfill (mục 5.4); cô lập query qua `JOIN project_characters` |
| 2 | Model không hỗ trợ tool calling (local/custom) | Chat thuần, không có prompt-based tool calling (mục 6, ADR-005) |
| 3 | Scene dài làm phình context | Chỉ nạp `summary`; `scene_get(range)` khi cần nội dung |
| 4 | Agent ghi nhiều entity trong một run | `ApprovalRequest` batch theo lượt + "duyệt phần còn lại của run" cho tool đảo ngược được (mục 7.1). Change set kiểu draft → apply để Phase 6 |
| 5 | Lock Starvation khi ghi SQLite async | **Mô hình Dual-Pool**: `writer_pool` (max 1 conn) + `reader_pool` (max 10 conn) (mục 2, ADR-002) |
| 6 | sqlx vs rusqlite | **Đã chốt sqlx** (ADR-002) |
| 7 | Có cần embedding cho memory ngay không | Mặc định: chưa, dùng FTS5; Phase 3 nâng cấp `sqlite-vec` + `fastembed-rs` nhúng |
| 8 | Mock vs bản web (Dockerfile) | Bản web không có agent runtime; tách build target hoặc bỏ |
| 9 | App bị kill giữa run | Startup reconcile → `interrupted` + commit guard (mục 7.1) |
| 10 | Model yếu gọi tool sai liên tục | `RunBudget` + `max_retries_per_tool`; có thể hạ về `explore` |
| 11 | `tauri-specta` + `Channel<T>` version compatibility | Ghim cứng `specta = "=2.0.0-rc.27"` và `tauri-specta = "=2.0.0-rc.27"` (mục 12) |
| 12 | Đếm token sai làm tràn context | Calibration theo `usage` + buffer + retry 1 lần khi overflow (mục 7.2) |
| 13 | Build theo chiều ngang, lỗi thiết kế core loop lộ muộn | Vertical slice ở Phase 1 (mục 16) |

---

## 16. Roadmap (đi theo lát cắt dọc)

**Phase 0 — Nền tảng (gate)**
- [ ] Workspace 4 crate + CI kiểm `cargo tree` (mục 4.2)
- [ ] Migration `0001` (từ `db.dbml`) + **`0002_scope.sql`: bảng liên kết `project_characters`, `project_id` cho `lore_entries` + backfill** (mục 5.4) + seed system
- [ ] `sqlx` **Dual-Pool** (`writer_pool` max 1 conn, `reader_pool` max 10 conn, WAL, FK, busy_timeout=5000), `error.rs`, Sans-I/O `phase.rs` + I/O `run_loop.rs`, `AppState`, `telemetry.rs` (tracing + log file)
- [ ] **Spike IPC:** Ghim `tauri-specta = "=2.0.0-rc.27"` + `Channel<RunEvent>` (tagged enum) → sinh `bindings.ts`
- [ ] Chốt message envelope v1 (mục 5.5) và các ADR 001–007
- [ ] FE: `shared/lib/ipc`, TanStack Query, `ssr: false`

**Phase 1 — Vertical slice (đi hết đường với 1 tool)**
- [ ] `MockLlmClient` theo kịch bản (gọi `project_get` rồi trả lời)
- [ ] Sans-I/O `RunPhase` state machine + async `run_loop.rs` driver + multiplexed `EventSink` (IPC, audit trace, entity invalidation) + `run_steps` + scheduler (`RUN_ACTIVE`) + startup reconcile + Active Compute Timer
- [ ] `ToolContext` + registry + tool `project_get` (read-only)
- [ ] `chat_send` + Channel + cancel
- [ ] FE: `ChatView` render stream, tool call, trace tối thiểu
- [ ] Test tự động cho slice (mock LLM, không cần GUI)
- **Điều kiện qua phase:** slice chạy end-to-end, test xanh, `run_steps` đúng. Chưa đạt thì chưa mở rộng.

**Phase 2 — Provider & model thật**
- [ ] `ai_providers`/`ai_models`, keychain, test connection, custom model
- [ ] Adapter `chat_completions` → `StreamItem`, rồi `anthropic`, `ollama`; `retry.rs` + `StreamReset`
- [ ] Thay `MockLlmClient` bằng provider thật trong slice
- [ ] `sessions` theo project, `ModelSelectorPopover`, `SidebarHistory`

**Phase 3 — Dữ liệu thật thay mock** (có thể chạy song song Phase 2; làm từng feature, xong feature nào bỏ mock feature đó)
- [ ] Repo + command: project, character, arc/chapter/scene, world, lore, theme
- [ ] Soft delete + RecycleBin

**Phase 4 — Context, prompt, memory, attachment**
- [ ] Prompt templates + bindings + composer + manifest + preview
- [ ] `ContextBuilder` + token allocation Strategy + `tiktoken-rs`/EMA calibration + `ContextUsagePopover`
- [ ] `context_memories` + FTS5 retrieval strategy + `content_hash` + UI quản lý (không trộn với `character_memories`)
- [ ] Attachment: txt/md trước, rồi docx

**Phase 5 — Tool, approval, verify**
- [ ] Repository/Specification cho tool đọc từng entity (đi cùng Phase 3), luôn áp project scope qua `ToolContext`
- [ ] `RunMode` explore/edit, `request_edit_mode`
- [ ] Tool ghi qua `UnitOfWork` + transaction/Savepoint + diff + **CommitGuard** + `scene_versions` Memento
- [ ] `ApprovalRequest` batch + `approval_decide` + "duyệt phần còn lại của run"
- [ ] Verification Chain of Responsibility (schema → scope → domain rules → post-condition) với kết quả `Pass | Retry(feedback) | Block(reason)`
- [ ] `ProcessDetailDialog`/`ToolCallsPopover` nối `run_steps`, prompt manifest

**Phase 6 — Hoàn thiện**
- [ ] Hierarchical Compaction Strategy (elide tool results → prune memories/attachments → summarize turns, deterministic fallback), change set draft → apply
- [ ] Story directives compile, output verify
- [ ] PDF extraction (sau khi đánh giá crate), export/import, updater, hardening `capabilities/`
- [ ] (Tuỳ chọn, đo trước khi bật) hybrid retrieval `sqlite-vec` + `fastembed-rs`, LLM-judge verifier, `crates/cli` headless eval

---

## 17. Definition of Done (mỗi phase)

- `cargo clippy -D warnings`, `cargo test` (mock `LlmClient` + repo in-memory), `pnpm typecheck && pnpm lint` xanh.
- `bindings.ts` không có diff sau khi regenerate.
- Domain/agent không phụ thuộc `tauri`, `sqlx`, `reqwest` (kiểm bằng `cargo tree`).
- Runtime test bằng mock `LlmClient` + `Clock`/`IdGen` giả: có test cho từng chuyển `RunPhase`, vượt budget, cancel giữa approval, reconcile sau kill, retry LLM (429/5xx/stream đứt), context overflow, `RUN_ACTIVE`.
- Có ADR cho mỗi quyết định ở mục 2 và 15.
