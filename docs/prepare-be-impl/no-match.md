# NHỮNG KIẾN TRÚC & TIẾP CẬN KHÔNG PHÙ HỢP CHO MERLINK BACKEND (NO-MATCH CATALOG)

> **Tài liệu tham chiếu:** Tổng hợp và đối chiếu từ 5 báo cáo nghiên cứu chuyên sâu:
> - [`agent-a-report.md`](agent-a-report.md): Runtime Execution, State Machine & Driver Loop
> - [`agent-b-report.md`](agent-b-report.md): Tool Registry, Capability Security & HITL Approvals
> - [`agent-c-report.md`](agent-c-report.md): Context Assembly, Hierarchical Prompts & Memory
> - [`agent-d-report.md`](agent-d-report.md): Multi-Provider LLM Adapters, SSE Streaming & Outbound Guard
> - [`agent-e-report.md`](agent-e-report.md): Tauri v2 IPC Channels, Type Safety & SQLite Dual-Pool
>
> **Mục đích:** Ghi nhận và phân tích kỹ thuật chi tiết các kiến trúc, framework, thư viện và mẫu thiết kế đã được khảo sát trên thị trường nhưng **BỊ TỪ CHỐI (NO-MATCH)** áp dụng cho Backend của Merlink. Việc tách riêng file này giúp tài liệu kế hoạch hành động [`docs/be-tauri-plan.md`](../be-tauri-plan.md) giữ được tính cô đọng, thuần túy là hướng dẫn khẳng định tích cực (prescriptive & affirmative).

---

## 1. TỔNG QUAN NGUYÊN TẮC THIẾT KẾ MERLINK

Merlink là ứng dụng Desktop chuyên biệt cho tác giả sáng tác tiểu thuyết và xây dựng thế giới hư cấu (Novel Writing & Worldbuilding), vận hành theo mô hình:
- **Desktop Single-User Cục bộ:** 1 người dùng trên 1 máy tính cá nhân, không có môi trường multi-tenant.
- **Backend-Authoritative tuyệt đối:** Rust nắm toàn bộ Agent Runtime, kết nối mạng, SQLite và OS Keychain. React 19 chỉ là tầng trình diễn mỏng.
- **SQLite là Kho Lưu Trữ Sự Thật Duy Nhất:** Toàn bộ dữ liệu nghiệp vụ, metadata, trace kiểm toán đều lưu trữ trong SQLite cục bộ với chế độ WAL.
- **Tất Định & Kiểm Toán Được (Deterministic & Auditable):** Mỗi quyết định của Agent phải có khả năng tái hiện, giải thích và đảo ngược (reversible).

Mọi phương án kiến trúc đi ngược lại 4 nguyên tắc trên hoặc mang lại sự phức tạp không cần thiết (overengineering) đều bị từ chối.

---

## 2. CHI TIẾT CÁC KIẾN TRÚC & GIẢI PHÁP BỊ TỪ CHỐI

```
┌────────────────────────────────────────────────────────────────────────────────────────┐
│                              NO-MATCH ARCHITECTURE RADAR                               │
├──────────────────────────┬─────────────────────────────┬───────────────────────────────┤
│ Phân hệ                  │ Tiếp cận bị TỪ CHỐI         │ Phương án THAY THẾ đã chọn    │
├──────────────────────────┼─────────────────────────────┼───────────────────────────────┤
│ 1. Runtime & Execution   │ • Actor-based (Actix/Ractor)│ • Sans-I/O Loop (RunPhase FSM)│
│                          │ • Event-Choreography        │ • Centralized Driver Loop     │
│                          │ • Vòng lặp tự trị vô hạn    │ • Hard RunBudget + Timer      │
│                          │ • File kép JSONL + SQLite   │ • Single SQLite Store         │
│                          │ • Task Abort / Thread Kill  │ • Cooperative CancellationToken│
├──────────────────────────┼─────────────────────────────┼───────────────────────────────┤
│ 2. Tool & Security       │ • Docker Container Sandbox  │ • OCAP ToolContext Injection  │
│                          │ • WASM/WASI Tool Sandbox    │ • In-Process Strongly-Typed   │
│                          │ • Prompt-based Guardrails   │ • Pre-LLM Capability Filter   │
│                          │ • Chặn đồng bộ (thread::park)│• Async Suspension (oneshot)  │
│                          │ • Daemon / gRPC Server ngoài│ • Embedded In-Process Rust    │
│                          │ • Chung TX cho Trace & Data │ • Dual Connection Strategy    │
├──────────────────────────┼─────────────────────────────┼───────────────────────────────┤
│ 3. Context & Memory      │ • Vector DB ngoài (Qdrant)  │ • SQLite FTS5 (nhúng cục bộ)  │
│                          │ • Tóm tắt đệ quy tự động    │ • Hierarchical Compaction     │
│                          │ • Gộp Context & Character   │ • Tách context_memories riêng │
│                          │ • Tokenizers nặng của HF    │ • tiktoken-rs + EMA Calibration│
├──────────────────────────┼─────────────────────────────┼───────────────────────────────┤
│ 4. LLM & Networking      │ • Frontend gọi trực tiếp API│ • Rust Reqwest Client duy nhất│
│                          │ • Plaintext Key trong DB/.env│• OS Keychain qua keyring-rs  │
│                          │ • Prompt Tooling (Regex)    │ • Native Function Calling only│
│                          │ • Naive Retry không Jitter  │ • Exp Backoff + Full Jitter   │
│                          │ • Auto-reconnect SSE on POST│ • eventsource-stream (1-shot) │
│                          │ • Tùy tiện Custom Base URL  │ • Outbound Guard + SSRF check │
├──────────────────────────┼─────────────────────────────┼───────────────────────────────┤
│ 5. IPC & Storage         │ • Local WebSocket / gRPC    │ • Tauri in-memory IPC         │
│                          │ • Global app.emit Streaming │ • tauri::ipc::Channel<RunEvent>│
│                          │ • ORM nặng (Diesel/SeaORM)  │ • sqlx macro & query_as       │
│                          │ • SQLite WASM trên Webview  │ • SQLite Rust Native (WAL)    │
│                          │ • Pool chung Đọc/Ghi SQLite │ • Dual-Pool (1 Writer, N Read)│
│                          │ • Khóa ngoại 1-N characters │ • Bảng N-N project_characters │
└──────────────────────────┴─────────────────────────────┴───────────────────────────────┘
```

---

### PHÂN HỆ 1: RUNTIME EXECUTION & STATE MACHINE

#### 1.1 Bác bỏ: Mô hình Actor-based Runtime Frameworks (Actix, Ractor, Xactor)
- **Mô tả cách tiếp cận:** Mô hình hóa các thực thể của Agent thành các Actor độc lập (như `SessionActor`, `ToolExecutorActor`, `ContextBuilderActor`) tương tác bằng hàng đợi tin nhắn bất đồng bộ (MPSC Mailboxes), tương tự kiến trúc của AutoAgents (`liquidos-ai/autoagents`).
- **Lý do từ chối kỹ thuật:**
  1. *Xung đột vòng đời Transaction của SQLite (`sqlx`):* `sqlx::Transaction<'a, Sqlite>` phụ thuộc chặt chẽ vào Rust Lifetimes và nguyên lý RAII. Khi một write tool thực thi, nó cần giữ transaction độc quyền. Việc chuyển giao transaction qua biên giới các Actor messages đòi hỏi bọc `Arc<tokio::sync::Mutex<Option<Transaction>>>`, triệt tiêu zero-cost abstraction và dễ dẫn đến deadlock khi mailbox bị nghẽn.
  2. *Mất tính tất định (Non-determinism):* Thứ tự xử lý message trong mailbox của các Actor phụ thuộc vào thời điểm lập lịch luồng của Tokio runtime. Điều này khiến việc ghi lại trace tuần tự và tái hiện lỗi (replay debugging) trở nên bất khả thi.
  3. *Overengineering cho Desktop đơn người dùng:* Merlink chỉ phục vụ 1 người dùng trên máy cục bộ với tối đa 3 concurrent runs (thường chỉ 1 run active trên một session). Một Explicit State Machine Loop dạng tuyến tính (`RunPhase`) đơn giản, dễ kiểm thử đơn vị và dễ bảo trì hơn gấp nhiều lần.
- **Phương án thay thế đã chọn:** **Explicit Sans-I/O State Machine kết hợp Async Driver Loop tuần tự (`run_loop.rs`)**.

#### 1.2 Bác bỏ: Kiến trúc Phân tán Sự kiện Phi tập trung (Event-Choreography)
- **Mô tả cách tiếp cận:** Không có một vòng lặp điều phối trung tâm. Khi một sự kiện xảy ra (ví dụ: `LLMChunkReceived`, `ToolProposed`), một Event Bus nội bộ sẽ phát tán sự kiện đó, và các handler tự động phản ứng để kích hoạt hành vi tiếp theo.
- **Lý do từ chối kỹ thuật:**
  1. *Phân mảnh trạng thái (Distributed State Nightmare):* Rất khó xác định hệ thống đang ở pha nào tại bất kỳ thời điểm nào. Trạng thái của run bị phân tán trên nhiều listener.
  2. *Rủi ro Race Condition:* Nếu hai sự kiện được bắn ra gần như đồng thời (ví dụ: người dùng gửi `Cancel` cùng lúc LLM phát `ToolProposed`), các event handler có thể chạy chồng chéo, dẫn đến việc tool vẫn chạy và commit vào database dù UI đã hiển thị hủy.
  3. *Không thể cưỡng chế RunBudget:* Rất khó đếm chính xác số bước lặp (steps) và thời gian thực thi (wall-clock compute duration) nếu không có một vòng lặp tuyến tính kiểm soát entry/exit của từng lượt.
- **Phương án thay thế đã chọn:** **Centralized Run Driver Loop (`run_loop.rs`) điều khiển bởi `RunPhase`**.

#### 1.3 Bác bỏ: Vòng lặp Agent Tự trị Vô hạn (Autonomous Infinite Agent Loops)
- **Mô tả cách tiếp cận:** Cho phép agent tự lặp và tự ra quyết định liên tục không có giới hạn cứng, dựa vào LLM tự nhận thức khi nào nên kết thúc.
- **Lý do từ chối kỹ thuật:**
  1. *Nguy cơ cạn kiệt tài nguyên & bùng nổ chi phí API:* Tác vụ sáng tác truyện có thể khiến mô hình bị lặp ý tưởng (hallucinatory loop) hoặc gọi đi gọi lại một tool sửa đổi.
  2. *Thiếu kiểm soát ngân sách:* Bắt buộc phải thực thi cơ chế **RunBudget** cứng: giới hạn tối đa số bước chạy (ví dụ 25 steps), giới hạn số tool calls (40 calls), giới hạn thời gian thực thi (timeout) và trần token. Khi chạm ngưỡng, runtime bắt buộc phải dừng và yêu cầu sự can thiệp của tác giả.
- **Phương án thay thế đã chọn:** **Hard RunBudget + Active Compute Timer** (chỉ tính giờ khi agent đang tính toán, tạm dừng đồng hồ khi đợi người dùng phê duyệt).

#### 1.4 Bác bỏ: Lưu trữ Vết kép (Dual-Store: JSONL Rollouts song song SQLite DB)
- **Mô tả cách tiếp cận:** Lưu metadata vào SQLite nhưng lại ghi lại toàn bộ trajectory, raw prompt và turn traces ra các file `.jsonl` nén (zstd) riêng biệt trên ổ đĩa (pattern của Tinybot).
- **Lý do từ chối kỹ thuật:**
  1. *Lệch pha trạng thái (State Drift):* Khi máy tính bị mất điện đột ngột hoặc tiến trình bị tắt ngang: SQLite với cơ chế WAL và Transaction ACID đảm bảo dữ liệu không bị hỏng, nhưng file JSONL có thể bị ghi dở (torn write), dẫn đến sự không nhất quán giữa dữ liệu session trong DB và file log ngoài đĩa.
  2. *Phức tạp hóa thao tác Dọn dẹp / Xóa Project:* Khi người dùng chọn "Xóa dự án" hoặc "Sao lưu dữ liệu", hệ thống phải xử lý đồng thời việc xóa DB và quét đĩa xóa file rời.
  3. *Dư thừa hiệu năng:* Bảng `run_steps` append-only trong SQLite với index `(run_id, seq)` đáp ứng hoàn hảo tốc độ ghi < 1ms và truy vấn phân trang tức thời.
- **Phương án thay thế đã chọn:** **SQLite là nguồn lưu trữ sự thật DUY NHẤT (Single Store of Truth)** cho cả dữ liệu nghiệp vụ lẫn toàn bộ vết thực thi (`run_steps`).

#### 1.5 Bác bỏ: Hủy tác vụ thô bạo (Task Abort / Thread Kill)
- **Mô tả cách tiếp cận:** Gọi `tokio::task::JoinHandle::abort()` để kết liễu task runtime khi người dùng nhấn nút Stop.
- **Lý do từ chối kỹ thuật:**
  1. *Làm rách dữ liệu giao dịch (Torn Transactions):* Nếu task bị ngắt ngang khi câu lệnh SQL vừa chạy xong nhưng chưa kịp hoàn tất xử lý logic, transaction SQLite có thể bị bỏ lửng, lock file chưa kịp giải phóng.
- **Phương án thay thế đã chọn:** **Cooperative CancellationToken Tree** (`tokio_util::sync::CancellationToken`) lan truyền từ UI -> Command -> RunLoop -> Tool -> HTTP Stream, cho phép từng tầng tự dọn dẹp và rollback an toàn.

---

### PHÂN HỆ 2: CÔNG CỤ & BẢO MẬT (TOOL REGISTRY & CAPABILITY SECURITY)

#### 2.1 Bác bỏ: Sandbox bằng Docker Container hoặc Máy ảo
- **Mô tả cách tiếp cận:** Khởi chạy Docker container hoặc micro-VM để thực thi các tool của Agent.
- **Lý do từ chối kỹ thuật:**
  1. *Sai lệch miền nghiệp vụ:* Merlink là ứng dụng sáng tác văn học và thiết kế thế giới. Mọi công cụ của Merlink chỉ thao tác trên cơ sở dữ liệu quan hệ (tạo nhân vật, cập nhật cảnh, liên kết quan hệ), hoàn toàn không có công cụ thực thi lệnh shell/bash hệ thống.
  2. *Trải nghiệm người dùng tồi tệ:* Ép buộc nhà văn hoặc người dùng desktop phổ thông phải cài đặt Docker Desktop ngốn hàng gigabyte RAM là rào cản phân phối sản phẩm không thể chấp nhận được.
- **Phương án thay thế đã chọn:** **Object-Capability (OCAP) Model với `ToolContext`** (tiêm tự động `project_id`, `session_id`, `run_id`, `mode`).

#### 2.2 Bác bỏ: WASM / WASI Sandboxing cho Toàn bộ Tool Nội bộ
- **Mô tả cách tiếp cận:** Biên dịch từng tool nghiệp vụ thành module WebAssembly độc lập chạy trên Wasmtime.
- **Lý do từ chối kỹ thuật:**
  1. *Mất khả năng chia sẻ giao dịch SQLite trực tiếp:* Các tool của Merlink cần chạy trực tiếp trên cùng một `sqlx::Transaction` để bảo đảm tính nguyên tử (atomic commit/rollback). Module WASM bị cô lập bộ nhớ sẽ buộc phải serialize/deserialize qua FFI bridge, làm mất đi khả năng giữ con trỏ transaction của Rust.
  2. *Overengineering:* Mọi tool trong Merlink đều là mã nguồn do chính dự án phát triển và biên dịch tĩnh trong binary nhị phân của app, không phải plugin bên thứ ba tải từ internet.
- **Phương án thay thế đã chọn:** **In-process Strongly-Typed Tool Trait** (`schemars` schema generation + `thiserror` validation).

#### 2.3 Bác bỏ: Bảo mật dựa trên Prompt Guardrails (Prompt-based Security)
- **Mô tả cách tiếp cận:** Nhắc nhở LLM trong System Prompt: *"Bạn chỉ được đọc dữ liệu, không được phép sửa đổi trừ khi người dùng cho phép"*.
- **Lý do từ chối kỹ thuật:**
  1. *Không đáng tin cậy trước Prompt Injection:* Khi agent đọc các tệp đính kèm văn bản từ bên ngoài, chỉ thị độc hại có thể ghi đè lời dặn trong system prompt.
  2. *Lãng phí Token:* Gửi toàn bộ schema của hàng chục tool ghi khi đang ở chế độ xem làm lãng phí 15–20% context window.
- **Phương án thay thế đã chọn:** **Pre-LLM Capability Filtering theo `RunMode`**. Khi ở `explore`, tool ghi hoàn toàn không xuất hiện trong JSON Schema gửi lên LLM.

#### 2.4 Bác bỏ: Chặn Đồng bộ Luồng (`thread::park` / Sync Channels) khi chờ Duyệt (HITL)
- **Mô tả cách tiếp cận:** Block luồng hệ điều hành bằng `thread::park()` hoặc sync mutex trong lúc chờ người dùng bấm Approve trên UI.
- **Lý do từ chối kỹ thuật:**
  1. *Cạn kiệt luồng Worker của Tokio (Thread Starvation):* Trong runtime bất đồng bộ, việc block một luồng bằng mã đồng bộ sẽ chiếm dụng vĩnh viễn worker thread của thread pool. Khi người dùng mở hộp thoại duyệt và bỏ đi hàng giờ, ứng dụng sẽ bị treo cứng (freeze).
- **Phương án thay thế đã chọn:** **Non-blocking Async Suspension** với `tokio::sync::oneshot` kết hợp nhả slot semaphore toàn cục, đưa task vào trạng thái parked tự nhiên của Tokio.

#### 2.5 Bác bỏ: Daemon Tiến trình Rời & Multi-Process Swarm qua gRPC
- **Mô tả cách tiếp cận:** Tách Backend Rust thành một daemon tiến trình chạy ngầm riêng biệt (`merlink-daemon`), giao tiếp với giao diện Tauri qua gRPC hoặc IPC socket.
- **Lý do từ chối kỹ thuật:**
  1. *Nguy cơ tiến trình mồ côi (Zombie Processes):* Nếu app chính gặp sự cố, daemon ngoài rất dễ bị sót lại chạy ngầm, gây rò rỉ pin và giữ lock file cơ sở dữ liệu SQLite.
  2. *Xung đột file lock:* Khi cả GUI process và Daemon process cùng mở kết nối ghi vào file SQLite cục bộ, nguy cơ gặp lỗi `SQLITE_BUSY` tăng vọt.
- **Phương án thay thế đã chọn:** **In-process Native Embedded Backend**: Toàn bộ mã nguồn Rust chạy ngay trong tiến trình chính của ứng dụng Tauri v2.

#### 2.6 Bác bỏ: Dùng chung một Transaction cho cả Trace Audit và Business Data
- **Mô tả cách tiếp cận:** Ghi `run_steps` và dữ liệu truyện (scenes, characters) trong cùng một SQLite Transaction để đơn giản hóa quản lý kết nối.
- **Lý do từ chối kỹ thuật:**
  1. *Mất sạch vết kiểm toán khi nghiệp vụ lỗi hoặc rollback:* Khi một tool bị lỗi hoặc bị người dùng reject và transaction bị rollback, toàn bộ các bản ghi `run_steps` trước đó cũng bị xóa sổ, khiến hệ thống không thể biết chuyện gì đã xảy ra để hiển thị trên UI hay phục vụ debugging.
- **Phương án thay thế đã chọn:** **Dual Connection Strategy** (Kênh Audit ghi append-only autocommit độc lập; Kênh Business chạy trong `sqlx::Transaction` riêng biệt có Savepoints).

---

### PHÂN HỆ 3: QUẢN LÝ NGỮ CẢNH & BỘ NHỚ (CONTEXT, PROMPTS & MEMORY)

#### 3.1 Bác bỏ: Máy chủ Vector Database Độc lập (Standalone Vector DB như Qdrant / Milvus / Chroma)
- **Mô tả cách tiếp cận:** Dựng một vector database server độc lập chạy nền để phục vụ semantic search cho agent.
- **Lý do từ chối kỹ thuật:**
  1. *Gánh nặng tài nguyên:* Các vector database riêng biệt ngốn hàng trăm megabyte RAM, làm nặng nề bộ cài đặt ứng dụng desktop cá nhân.
  2. *Giai đoạn đầu (Phase 0–3) chưa cần thiết:* Với một dự án truyện dài, việc tìm kiếm cảnh, nhân vật và thiết lập thế giới hoàn toàn có thể giải quyết xuất sắc bằng **SQLite FTS5 (Full-Text Search)** kết hợp bộ lọc Metadata phân cấp (Scope, Tags, Recency Scoring).
- **Phương án thay thế đã chọn:** **SQLite FTS5 nhúng bản địa**, lộ trình Phase 3 nâng cấp in-process semantic search qua extension `sqlite-vec` và crate `fastembed-rs`.

#### 3.2 Bác bỏ: Tóm tắt Đệ quy Tự động Mất Kiểm soát (Uncontrolled Recursive Summarization)
- **Mô tả cách tiếp cận:** Tự động gọi LLM tóm tắt toàn bộ lịch sử trò chuyện mỗi khi số lượng token vượt qua ngưỡng cố định.
- **Lý do từ chối kỹ thuật:**
  1. *Mất mát chi tiết văn học cốt lõi (Loss of Literary Nuance):* Trong sáng tác tiểu thuyết, các chi tiết cài cắm (foreshadowing), phục trang, ánh mắt nhân vật là linh hồn của tác phẩm. Việc tóm tắt đệ quy liên tục qua nhiều vòng sẽ biến các tình tiết sống động thành các câu tóm tắt cụt lủn, vô hồn.
  2. *Bùng nổ chi phí token ẩn:* Việc liên tục gọi mô hình phụ để tóm tắt gây tốn kém chi phí không báo trước cho người dùng.
- **Phương án thay thế đã chọn:** **Hierarchical Compaction**: Cắt tỉa raw tool results cũ trước, giảm memories phụ, bảo vệ bất biến 6 tin nhắn gần nhất + Pinned Memories + Locked System Layers; fallback cắt tỉa tất định không bao giờ làm crash run loop.

#### 3.3 Bác bỏ: Trộn lẫn Trí nhớ Trợ lý (Context Memory) với Ký ức Nhân vật (Character Memory)
- **Mô tả cách tiếp cận:** Đưa toàn bộ memory vào chung một bảng và nạp chung vào prompt.
- **Lý do từ chối kỹ thuật:**
  1. *Hiện tượng loạn vai (Character Identity Bleeding / Persona Bleeding):* Trí nhớ của Agent (`context_memories`) là kiến thức vận hành giữa Tác giả và Trợ lý AI (ví dụ: *"Tác giả muốn chương 3 tập trung vào miêu tả tâm lý"*). Ký ức Nhân vật (`character_memories`) là sự kiện xảy ra trong thế giới hư cấu (ví dụ: *"Phù Thanh Cơ nhớ cái chết của trưởng lão"*). Nếu trộn lẫn, AI sẽ đưa các nhận xét kỹ thuật của tác giả vào trong lời thoại của nhân vật, phá hủy tính chân thực của tác phẩm.
- **Phương án thay thế đã chọn:** **Tách biệt tuyệt đối**: `context_memories` lưu trí nhớ trợ lý theo scope (global, project, session); `character_memories` lưu ký ức nhân vật theo `character_id` và `scene_id`.

#### 3.4 Bác bỏ: Phụ thuộc vào Tokenizer Nặng của Hugging Face (`tokenizers` crate) ở Phase Đầu
- **Mô tả cách tiếp cận:** Cài đặt crate `tokenizers` của Hugging Face và tải các file `tokenizer.json` dung lượng lớn của từng họ mô hình.
- **Lý do từ chối kỹ thuật:**
  1. *Phình to dung lượng bộ cài:* Mỗi file `tokenizer.json` nặng từ 2MB đến 10MB, làm tăng kích thước cài đặt không cần thiết.
- **Phương án thay thế đã chọn:** Sử dụng `tiktoken-rs` làm baseline đếm token thô, sau đó kết hợp **Thuật toán Hiệu chuẩn Thích ứng EMA (`token_calibration`)** tự động cập nhật hệ số hiệu chuẩn theo `usage` thực tế trả về từ Provider.

---

### PHÂN HỆ 4: KẾT NỐI MẠNG & LLM PROTOCOL (LLM INTEGRATION & SECURITY)

#### 4.1 Bác bỏ: Frontend Gọi Trực tiếp API LLM (Client-Side Hybrid Approach)
- **Mô tả cách tiếp cận:** Mã JavaScript trong React trực tiếp gọi `fetch("https://api.openai.com/...")` để stream token.
- **Lý do từ chối kỹ thuật:**
  1. *Lộ API Key:* Đưa API key lên Webview làm tăng nguy cơ bị rò rỉ qua các tiện ích mở rộng trình duyệt hoặc lỗ hổng XSS.
  2. *Phá vỡ nguyên tắc Backend-Authoritative:* Backend Rust không thể kiểm soát token budget, không thể can thiệp verification pipeline, không thể thực thi transaction guard, và không thể ghi nhận đầy đủ audit log `run_steps`.
- **Phương án thay thế đã chọn:** **Mọi request ra ngoài bắt buộc đi qua Crate `llm` trong Rust**.

#### 4.2 Bác bỏ: Lưu trữ API Keys dạng Plaintext trong Cơ sở dữ liệu hoặc File Cấu hình
- **Mô tả cách tiếp cận:** Lưu trực tiếp chuỗi API key vào bảng SQLite `ai_providers` hoặc tệp `settings.json`.
- **Lý do từ chối kỹ thuật:**
  1. *Rủi ro an ninh mạng nghiêm trọng:* Các tệp SQLite hoặc JSON trên máy người dùng rất dễ bị các phần mềm độc hại (infostealers) quét và chiếm đoạt.
- **Phương án thay thế đã chọn:** **Lưu trữ khóa API vào OS Credential Store** qua crate `keyring` (Windows Credential Manager, macOS Keychain, Linux Secret Service). SQLite chỉ lưu `secret_ref`.

#### 4.3 Bác bỏ: Prompt-based Tool Calling cho các Model Không Hỗ trợ Function Calling
- **Mô tả cách tiếp cận:** Hướng dẫn prompt cho các model văn bản thuần cũ sinh text dạng `Action: <tool_name>\nAction Input: <json>` rồi dùng Regex bóc tách.
- **Lý do từ chối kỹ thuật (ADR-005):**
  1. *Tỷ lệ lỗi cú pháp và ảo giác cực cao:* Nội dung tiểu thuyết có nhiều ký tự đặc biệt, dấu ngoặc kép, xuống dòng, khiến Regex parser rất dễ bị vỡ hoặc parse thiếu trường.
  2. *Nguy cơ Prompt Injection:* Dữ liệu ngoại lai có thể đánh lừa bộ parser regex thực thi công cụ ngoài ý muốn.
- **Phương án thay thế đã chọn:** Cờ `ai_models.supports_tools = true` là điều kiện tiên quyết để tham gia vào Agent Run. Các model không hỗ trợ function calling chỉ được dùng ở chế độ Chat văn bản thuần túy.

#### 4.4 Bác bỏ: Thử lại Ngây thơ (Naive Retry không có Exponential Backoff & Jitter)
- **Mô tả cách tiếp cận:** Khi gặp lỗi HTTP 429 hoặc 500, ngay lập tức gửi lại request cùng thời điểm hoặc sleep một khoảng thời gian cố định.
- **Lý do từ chối kỹ thuật:**
  1. *Hiện tượng Thundering Herd & Khóa Tài khoản:* Gửi lại dồn dập khiến server của nhà cung cấp tiếp tục từ chối và kéo dài thời gian áp dụng rate limit, thậm chí có thể dẫn đến việc tài khoản bị tạm khóa.
- **Phương án thay thế đã chọn:** **Exponential Backoff có cộng thêm Full Jitter ngẫu nhiên**, tôn trọng nghiêm ngặt header `Retry-After`, phân loại rõ ràng transient errors (thử lại tối đa 3 lần) vs terminal errors (fail fast).

#### 4.5 Bác bỏ: Thư viện `reqwest-eventsource` với Cơ chế Tự động Reconnect trên HTTP POST
- **Mô tả cách tiếp cận:** Sử dụng crate `reqwest-eventsource` để parse SSE stream.
- **Lý do từ chối kỹ thuật:**
  1. *Sai lệch bản chất giao thức LLM:* `reqwest-eventsource` tuân theo chuẩn SSE trình duyệt (vốn thiết kế cho HTTP GET), tự động gửi lại request khi stream bị đứt. Trong LLM API, request là **HTTP POST** chứa toàn bộ hội thoại và có thể đã sinh ra một nửa tool call. Tự động reconnect sẽ gây trùng lặp request và tính tiền 2 lần.
- **Phương án thay thế đã chọn:** **`eventsource-stream` (RFC 8895 parser nhẹ, zero-reconnect)**. Khi đứt luồng, hệ thống chủ động hủy buffer, emit `StreamReset` và retry có kiểm soát ở tầng run loop.

#### 4.6 Bác bỏ: Bỏ qua Outbound Guard đối với Custom Base URLs
- **Mô tả cách tiếp cận:** Cho phép người dùng nhập bất kỳ `base_url` nào và gửi thẳng API key đến URL đó mà không kiểm tra.
- **Lý do từ chối kỹ thuật:**
  1. *Nguy cơ rò rỉ API Key & Tấn công SSRF:* Người dùng có thể cấu hình endpoint trỏ về địa chỉ nội bộ độc hại (ví dụ AWS metadata `169.254.169.254` hoặc port nội bộ).
- **Phương án thay thế đã chọn:** **Outbound Guard**: Phân biệt rạch ròi giữa `is_local = true` (Ollama/LM Studio chỉ cho phép localhost/LAN) và Cloud Provider (bắt buộc HTTPS, cấm trỏ về private IP).

---

### PHÂN HỆ 5: IPC & CƠ SỞ DỮ LIỆU CỤC BỘ (IPC & STORAGE ARCHITECTURE)

#### 5.1 Bác bỏ: Phát Sự kiện Toàn cục (`app.emit` / `window.emit`) cho Luồng Streaming Token
- **Mô tả cách tiếp cận:** Khi LLM sinh token, backend Rust gọi `app.emit("llm-token", token)` để gửi lên Frontend.
- **Lý do từ chối kỹ thuật:**
  1. *Xáo trộn thứ tự sự kiện (Out-of-Order Delivery):* Event bus của Tauri dựa trên pub-sub toàn cục. Với các async listener trên Webview, khi token stream tốc độ cao (50–100 token/giây), các sự kiện có thể bị xử lý sai thứ tự, làm méo mó văn bản hiển thị.
  2. *Rò rỉ dữ liệu giữa các Cửa sổ / Session:* `app.emit` phát tới mọi subscriber trên toàn bộ ứng dụng; nếu người dùng mở hai tab/cửa sổ khác nhau, token của session A sẽ bị bắn sang session B.
  3. *Cạm bẫy Rò rỉ Bộ nhớ với `unlisten`:* Hàm `listen()` trong Tauri trả về một `Promise<UnlistenFn>`. Nếu component React bị unmount trước khi Promise hoàn tất, hàm hủy listener không bao giờ được gọi.
- **Phương án thay thế đã chọn:** **`tauri::ipc::Channel<RunEvent>`** (Direct Streaming Channel) gắn trực tiếp với command `chat_send`, bảo đảm strictly in-order delivery và tự động giải phóng sạch sẽ khi stream kết thúc.

#### 5.2 Bác bỏ: Sử dụng Local WebSocket hoặc gRPC Server cho Ứng dụng Desktop Đơn
- **Mô tả cách tiếp cận:** Dựng một local HTTP/WebSocket/gRPC server chạy tại `localhost:port` bên trong Rust để React kết nối tới.
- **Lý do từ chối kỹ thuật:**
  1. *Lỗ hổng bảo mật nghiêm trọng:* Mọi website độc hại người dùng mở trên trình duyệt web thông thường đều có thể gửi request tới `localhost:port` nếu server không có cơ chế xác thực token phức tạp.
  2. *Xung đột cổng (Port Collision):* Nếu cổng bị chiếm bởi phần mềm khác, ứng dụng sẽ không thể khởi động.
- **Phương án thay thế đã chọn:** **Tauri in-memory IPC**, giao tiếp qua bộ đệm IPC của Webview, không mở cổng mạng, an toàn tuyệt đối.

#### 5.3 Bác bỏ: Dùng chung Một Connection Pool SQLite cho cả Tác vụ Đọc và Ghi
- **Mô tả cách tiếp cận:** Khởi tạo một `SqlitePool` duy nhất với `max_connections = 5` và dùng chung cho cả UI load dữ liệu lẫn Agent chạy tool ghi.
- **Lý do từ chối kỹ thuật (Phát hiện thực nghiệm 2026):**
  1. *Cạm bẫy Lock Starvation trong Async Rust:* SQLite có cơ chế single-writer. Khi một async transaction ghi được mở qua nhiều điểm `.await` của Tokio, nếu các connection đọc khác cũng chen vào tranh chấp lock ở tầng OS, SQLite sẽ liên tục phát sinh độ trễ và đẩy hiệu năng ghi sụt giảm tới 20 lần do xung đột file lock.
- **Phương án thay thế đã chọn: Mô hình Dual-Pool:**
  - **`writer_pool`**: Cố định **`max_connections = 1`** để tuần tự hóa toàn bộ các tác vụ ghi ở hàng đợi bộ nhớ ứng dụng.
  - **`reader_pool`**: Thiết lập `max_connections = 10` với cờ `read_only = true` phục vụ giao diện người dùng đọc song song mượt mà mà không bao giờ bị chặn hay gây lock contention cho writer.

#### 5.4 Bác bỏ: Ràng buộc Khóa ngoại Cứng 1-N (`characters.project_id NOT NULL`)
- **Mô tả cách tiếp cận:** Gắn trực tiếp cột `project_id NOT NULL` vào bảng `characters`.
- **Lý do từ chối kỹ thuật:**
  1. *Triệt tiêu tính sáng tạo của Tác giả:* Trong thực tế văn học, một tác giả thường xây dựng các vũ trụ truyện (Shared Universe / Franchise) nơi các nhân vật xuất hiện xuyên suốt qua nhiều bộ truyện khác nhau (ví dụ: nhân vật chính ở phần 1 trở thành khách mời / cameo ở phần 2, hoặc xuất hiện trong ngoại truyện).
- **Phương án thay thế đã chọn:** Chuyển sang mô hình quan hệ **Nhiều-Nhiều (N-N)** thông qua bảng liên kết **`project_characters`** (gồm `project_id`, `character_id`, `role`, `notes`, `created_at`, `deleted_at`). Bảng `characters` giữ nguyên tính độc lập, vai trò cụ thể của nhân vật trong từng truyện được quản lý linh hoạt qua bảng nối.

---

### PHÂN HỆ 6: NHỮNG DESIGN PATTERN CỔ ĐIỂN (GOF) BỊ LOẠI BỎ TRONG RUST

#### 6.1 Bác bỏ: Global Mutable Singleton (`static mut` / `lazy_static` rải rác)
- **Lý do từ chối kỹ thuật:** Vi phạm quy tắc Ownership & Borrowing của Rust, gây khó khăn cho việc chạy unit test song song (vì các test chia sẻ chung một state toàn cục) và tiềm ẩn rủi ro deadlock.
- **Phương án thay thế đã chọn:** **Managed Dependency Injection (`AppState`)** khởi tạo tại Composition Root (`src-tauri/src/lib.rs`) và tiêm vào command qua `tauri::State<AppState>`.

#### 6.2 Bác bỏ: OOP State Pattern với Cấp phát Heap (`Box<dyn State>`)
- **Lý do từ chối kỹ thuật:** Tốn chi phí cấp phát bộ nhớ trên heap mỗi lần chuyển trạng thái, mất khả năng kiểm tra vét cạn (exhaustive pattern matching) của trình biên dịch Rust.
- **Phương án thay thế đã chọn:** **Algebraic Data Type (`enum RunPhase`) + Sans-I/O Functional Core** — zero-allocation, kiểm tra tính hợp lệ ngay lúc compile.

#### 6.3 Bác bỏ: Classic Bridge & Flyweight Pattern hướng đối tượng
- **Lý do từ chối kỹ thuật:** Bridge cổ điển tạo ra hai hệ thống phân cấp kế thừa chồng chéo rườm rà; Flyweight cổ điển đòi hỏi bảng factory quản lý con trỏ phức tạp.
- **Phương án thay thế đã chọn:** Sử dụng **Generic Traits / Trait Objects** (`Arc<dyn LlmClient>`, `Arc<dyn SecretStore>`) và kiểu dữ liệu chia sẻ bất biến bản địa của Rust (`Arc<str>`).

#### 6.4 Bác bỏ: Classic Visitor & Heavyweight Interpreter Pattern
- **Lý do từ chối kỹ thuật:** Visitor cổ điển đòi hỏi double-dispatch rườm rà (`accept` / `visit_*`) rất khó viết với borrow checker của Rust; Interpreter tự xây dựng cây AST để lọc dữ liệu là dư thừa khi đã có SQLite.
- **Phương án thay thế đã chọn:** Dùng **`match` exhaustive trên Rust Enum** thay cho Visitor; dùng **SQLite FTS5 & các rule/specification cụ thể** thay cho Interpreter. Specification chỉ được tạo thành abstraction dùng chung khi có nhiều consumer hoặc quy tắc composition thực sự; phase gating đơn lẻ có thể dùng validation function thuần.

---

### 2.8 Bác bỏ: Specification Framework cho mọi validation đơn lẻ
- **Lý do từ chối kỹ thuật:** Tạo cây interface/composite cho một rule chỉ có một consumer làm tăng số type và indirection nhưng không cải thiện tính đúng đắn đáng kể.
- **Phương án thay thế đã chọn:** Dùng validation function thuần cho một điều kiện đơn giản; chỉ dùng Specification khi rule được tái sử dụng hoặc thực sự cần AND/OR composition, diagnostics có cấu trúc và test riêng.

## 3. TỔNG KẾT

Tất cả các quyết định loại bỏ trên đây xuất phát từ phân tích kỹ thuật nghiêm ngặt nhằm bảo đảm hệ thống backend của Merlink:
1. **Gọn nhẹ tối đa:** Không kéo theo các dependency hoặc tiến trình cồng kềnh (Docker, WASM, standalone vector DB, local gRPC server).
2. **Bảo mật tuyệt đối:** Không lưu key plaintext, không tin tưởng prompt guardrail, cưỡng chế quyền bằng OCAP capabilities.
3. **Hiệu năng & Độ ổn định cao:** Tránh lock contention trên SQLite bằng Dual-Pool, tránh xáo trộn sự kiện bằng Tauri IPC Channel, tránh nghẽn thread Tokio bằng Async Suspension, và tận dụng tối đa hệ thống kiểu zero-cost của Rust.

Mọi tài liệu thiết kế và hướng dẫn thực thi tiếp theo tại [`docs/be-tauri-plan.md`](../be-tauri-plan.md) sẽ chỉ tập trung vào việc mô tả chi tiết, khẳng định các giải pháp tối ưu đã được lựa chọn.
