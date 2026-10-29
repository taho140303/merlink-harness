# BÁO CÁO TỔNG HỢP: DESIGN PATTERNS CHO MERLINK BACKEND

> **Mục đích:** Chỉ mục và bản tổng hợp các quyết định design pattern đã được nghiên cứu, đối chiếu và chấp nhận cho kiến trúc backend Merlink (Tauri v2 + Rust + SQLite + React 19).
> **Nguồn:** 5 báo cáo chuyên sâu trong cùng thư mục; catalog quyết định (chấp nhận / loại bỏ) được đồng bộ vào [`docs/be-tauri-plan.md`](../be-tauri-plan.md) mục 2.3.
> **Nguyên tắc xuyên suốt:** Pattern là công cụ cho một nhu cầu cụ thể, không phải mục tiêu tự thân. Chỉ tạo abstraction khi có từ hai implementation thực sự cần thay thế, khi abstraction cưỡng chế được an toàn/tính đúng đắn, hoặc khi nó cho phép kiểm thử ranh giới quan trọng.

---

## 1. Chỉ mục báo cáo

| Báo cáo | Nhóm pattern | Nội dung chính |
|---|---|---|
| [`agent-a-report.md`](agent-a-report.md) | Creational | Builder + Typestate, Factory, Prototype/snapshot, Managed DI State thay Singleton |
| [`agent-b-report.md`](agent-b-report.md) | Structural | Adapter (Ports & Adapters), Decorator/middleware, Proxy + scope guard, Newtype, Composite, Facade |
| [`agent-c-report.md`](agent-c-report.md) | Behavioral 1 | State machine Sans-I/O, Command + savepoint, Verification pipeline, Iterator/async stream |
| [`agent-d-report.md`](agent-d-report.md) | Behavioral 2 | Strategy (token/compaction/retrieval), Observer/EventSink, Memento, Template Method |
| [`agent-e-report.md`](agent-e-report.md) | Data & FE/BE sync | Repository + Unit of Work (Dual-Pool), Specification, CQRS nhẹ, rAF producer-consumer |

Danh mục pattern bị loại bỏ (kèm lý do kỹ thuật): [`docs/prepare-be-impl/no-match.md`](../prepare-be-impl/no-match.md), phân hệ 6.

---

## 2. Catalog quyết định

### 2.1 Được chấp nhận (21 pattern)

| # | Pattern | Vị trí áp dụng | Điều kiện để thực sự triển khai |
|---|---|---|---|
| 1 | Builder + Typestate | `ContextBuilder`, `LlmChatRequestBuilder` | Chỉ dùng Typestate cho invariant thứ tự/bắt buộc thực sự; còn lại Builder thường |
| 2 | Factory Method | `LlmClientFactory`, `ToolRegistryFactory` | Có ≥ 2 protocol/mode cần chọn động |
| 3 | Prototype / snapshot | `ToolContext::fork_for_call`, `SceneVersion` | Clone `Arc` + child token; snapshot trước ghi đè |
| 4 | Managed DI State | `AppState` qua `tauri::State` | Một composition root duy nhất |
| 5 | Adapter | 3 LLM adapter, `SecretStore` | Chuẩn hóa `StreamItem`; production + test adapter |
| 6 | Decorator | Retry, tracing/redaction, commit guard, context recovery | Thứ tự middleware khai báo tường minh |
| 7 | Proxy + scope guard | Outbound guard, repo scoped theo `ToolContext` | Cưỡng chế ở tầng repo, không kiểm tra sau khi đọc |
| 8 | Newtype | `ProjectId`, `CharacterId`, `SessionId`, `RunId`, `SecretRef` | Zero-cost; đặt trong `domain` |
| 9 | Composite | Cây `world_settings`, `themes`, `PromptLayer` | Bắt buộc có cycle detection + giới hạn độ sâu |
| 10 | Facade | Tauri commands | Command mỏng: validate → use case → DTO/Channel |
| 11 | Sans-I/O State Machine | `RunPhase` + pure `transition` | Functional core tách khỏi I/O driver |
| 12 | Command + reversible execution | Tool command, approval batch, savepoint | Lưu original/edited args; undo qua version + soft delete |
| 13 | Verification pipeline | Schema → Scope → Domain rules → Postconditions | Mỗi bước trả `Pass \| Retry \| Block` |
| 14 | Iterator / async Stream | `StreamItem`, `ToolCallAccumulator` | Không dispatch tool khi args chưa đủ và chưa validate |
| 15 | Strategy | Token estimation, compaction, memory retrieval | Chỉ tách trait khi có ≥ 2 chiến lược dùng thật |
| 16 | Observer / EventSink | IPC channel, audit trace, `entity_changed` | Observer cô lập lỗi; event dữ liệu phát sau commit |
| 17 | Memento | `scene_versions`, prompt manifest, run checkpoint | Có version, giới hạn dung lượng, không lưu secret |
| 18 | Template Method | Attachment ingestion | Whitelist MIME/size, hash dedupe, extract hook theo định dạng |
| 19 | Repository + Unit of Work | Reader repos, writer transaction trên Dual-Pool | Mọi ghi qua UoW + commit guard |
| 20 | Specification | Rule tái sử dụng cho readiness/phase gating | Chỉ khi rule được tái sử dụng hoặc cần AND/OR composition |
| 21 | CQRS nhẹ + rAF producer-consumer | TanStack Query, Zustand, token buffer | Server state ≠ UI state; flush theo frame |

### 2.2 Bị loại bỏ

| Pattern | Lý do chính | Thay thế |
|---|---|---|
| Global mutable Singleton | Xung đột ownership, khó test song song, nguy cơ deadlock | Managed DI State (`AppState`) |
| OOP State (`Box<dyn State>`) | Cấp phát heap mỗi lần chuyển, mất exhaustive match | `enum RunPhase` + `match` |
| Classic Bridge | Hai cây kế thừa chồng chéo, không cần thiết | Trait + generic/trait object |
| Classic Flyweight | Bảng factory quản lý con trỏ thủ công | `Arc<str>`, interning |
| Classic Visitor | Double dispatch khó viết với borrow checker | Enum + exhaustive `match` |
| Heavyweight Interpreter | Tự dựng AST trong khi đã có SQLite | SQLite FTS5 + rule cụ thể |
| Specification cho mọi validation | Over-abstraction khi chỉ có một consumer | Validation function thuần |
| Monolithic Mediator | God object, điểm nghẽn phụ thuộc | `DomainEvent` + EventSink |

---

## 3. Ánh xạ sang kế hoạch triển khai

| Khu vực kế hoạch | Pattern liên quan |
|---|---|
| Mục 4.2 cấu trúc crate | Newtype (`ids.rs`), ports, specifications, `ToolContext` |
| Mục 5 dữ liệu & scope | Repository/UoW, `project_characters` N-N, scope guard |
| Mục 6 provider | Adapter, Factory, Decorator (retry/redaction) |
| Mục 7.1 runtime | Sans-I/O state machine, Command/savepoint, Active Compute Timer |
| Mục 7.2–7.3 context & prompt | Builder, Strategy (token/compaction), Template Method cho layer render |
| Mục 8–9 tool & verify | Registry theo mode, Verification pipeline, diff + edited args |
| Mục 10 memory | Strategy retrieval, tách `context_memories` khỏi `character_memories` |
| Mục 12 IPC | Facade command, Observer EventSink, Channel streaming |
| Mục 13 frontend | CQRS nhẹ, rAF producer-consumer, query invalidation theo project |

---

## 4. Ràng buộc kỹ thuật rút ra từ các báo cáo

1. **Ranh giới crate là hàng rào biên dịch**, không phải quy ước: `domain` không biết `tauri`/`sqlx`/`reqwest`; `agent` không biết wire format.
2. **Scope được cưỡng chế trong query**, không kiểm tra sau khi đã đọc: mọi truy vấn tool nhận `ToolContext` và áp `project_id`, kể cả qua bảng nối `project_characters`.
3. **Audit tách khỏi nghiệp vụ**: trace append-only dùng kết nối riêng để còn lại khi business transaction rollback.
4. **Không phát cache event trước commit**: `entity_changed` chỉ sau khi Unit of Work commit thành công.
5. **An toàn dữ liệu cây**: duyệt `world_settings`/`themes` phải có cycle detection và giới hạn độ sâu.
6. **Streaming không dispatch sớm**: tool call chỉ chạy sau khi arguments delta tích lũy, parse và validate đầy đủ; UI flush token theo frame.
7. **Không hứa hẹn tuyệt đối**: các pattern giảm thiểu rủi ro (lock contention, stale write, confused deputy) nhưng vẫn cần test cho từng transition, retry, cancel và reconcile.
