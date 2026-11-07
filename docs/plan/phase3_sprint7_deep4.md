---
sprint_id: phase3_sprint7_deep4
title: Implement ReAct chat and prompt orchestration
phase: 3
sprint: 7
deep: 4
baseline_commit: 57a462efb953a665e3673b15e6cc33622a4f2d39
status: PLANNED
---

# phase3_sprint7_deep4 — Implement ReAct chat and prompt orchestration

## 1. Sprint identity

Engineering phase 3 (Draft domain and durable workflow), within-phase depth 4; baseline is the Git commit above, not an uncommitted workspace. **PROPOSED, NOT IMPLEMENTED.**

## 2. Objective

Implement ReAct chat and prompt orchestration. Observable handoff: ChatService, prompt composer, ChatStreamEvent and IPC #1–4,#9.

## 3. Scope boundaries

**IN_SCOPE:** Implement prompt/script JIT tools and chat service; own IPC #1–4,#9 only.; Fake provider sequence tests max-iteration/rejected count/cancel/lost tool result; audit is not recovery proof.

**OUT_OF_SCOPE / MUST NOT:** modify authoritative `docs/business/**`, `docs/db.dbml`, `docs/architecture/decisions/**`; use real application data; run production migration; add client request IDs to SF-06; invent business semantics; rewrite another sprint’s files.

## 4. Authoritative contract snapshot

- Run network with no SQLite TX held; every controlled exit seals applied writes or returns failure, never false success.

- Only registry-approved tools execute; system_language UI setting and project.language content are separate.

- **REQ-CHAT-001** `docs/business/be-core.md#L901-L998`: ReAct/tool loop, rejected threshold, completed tool results, controlled-exit sealing. (planning coverage `ASSIGNED`).

  Normative excerpt (verbatim baseline lines, truncated to 300 chars; use `sed` for the full range):

  > L901: ## §6. AI Harness & Workflow Runtime (Vòng lặp AI Harness, Workflow Phân đoạn & Biên nhận `SF-06`)

  > L903: ### 6.1 Vòng lặp ReAct Đơn Lượt (`Single-Turn ReAct Loop`) & Xử lý Từ chối Liên tiếp

  > L905: Theo `core.md` §3.6, §3.8 và `prompts/overview.md`:

  > L907: #### 6.1.1 Cấu trúc Hàm Ngoài (`run_ai_turn`) và Hàm Trong (`run_turn_loop`)

- **REQ-CHAT-002** `docs/business/prompts/overview.md#L19-L151`: Prompt blocks/tools JIT and distinct system_language vs project.language. (planning coverage `ASSIGNED`).

  Normative excerpt (verbatim baseline lines, truncated to 300 chars; use `sed` for the full range):

  > L19: ## Ngôn ngữ giao tiếp và kho câu hỏi

  > L21: Mỗi lượt, pipeline lấy `system_language` từ cài đặt giao diện (`vi` hoặc `en`) và bơm tên ngôn ngữ đầy đủ vào system prompt qua `{{system_language}}`. Đây là ngôn ngữ giao tiếp, không phải `project.language` của nội dung sáng tác. Backend giữ kho câu hỏi cố định theo ngôn ngữ, không có kho thì dùng

  > L23: ```text

  > L24: // Mã giả

- **REQ-CMD-001** `docs/business/be-core.md#L2118-L2118`: MUST implement typed IPC #1 `create_or_load_chat_session` with the specified authorization, DTO, transaction and error behavior. (planning coverage `ASSIGNED`).

- **REQ-CMD-002** `docs/business/be-core.md#L2119-L2119`: MUST implement typed IPC #2 `list_chat_sessions` with the specified authorization, DTO, transaction and error behavior. (planning coverage `ASSIGNED`).

- **REQ-CMD-003** `docs/business/be-core.md#L2120-L2120`: MUST implement typed IPC #3 `send_chat_message` with the specified authorization, DTO, transaction and error behavior. (planning coverage `ASSIGNED`).

- **REQ-CMD-004** `docs/business/be-core.md#L2121-L2121`: MUST implement typed IPC #4 `cancel_chat_turn` with the specified authorization, DTO, transaction and error behavior. (planning coverage `ASSIGNED`).

- **REQ-CMD-009** `docs/business/be-core.md#L2126-L2126`: MUST implement typed IPC #9 `regenerate_draft_phase` with the specified authorization, DTO, transaction and error behavior. (planning coverage `ASSIGNED`).

**Targeted source references (read only these ranges unless a test proves broader context is necessary):**

- `docs/business/be-core.md#L901-L998` — focus: Outer seal and inner ReAct; CLI: `sed -n "901,998p" docs/business/be-core.md` (or `rg -n "<symbol>" docs/business/be-core.md` for a changed anchor).

- `docs/business/prompts/overview.md#L41-L151` — focus: Prompt composition; CLI: `sed -n "41,151p" docs/business/prompts/overview.md` (or `rg -n "<symbol>" docs/business/prompts/overview.md` for a changed anchor).

- `docs/business/prompts/overview.md#L153-L256` — focus: Tool loop and orphan result fill; CLI: `sed -n "153,256p" docs/business/prompts/overview.md` (or `rg -n "<symbol>" docs/business/prompts/overview.md` for a changed anchor).

- `docs/business/be-core.md#L2163-L2180` — focus: Chat IPC #1–4 and regenerate #9; CLI: `sed -n "2163,2180p" docs/business/be-core.md` (or `rg -n "<symbol>" docs/business/be-core.md` for a changed anchor).

**Owned IPC command contracts (verbatim narrow BE §10.2 rows; proposed, not currently implemented):**

- IPC-01 `create_or_load_chat_session` — `docs/business/be-core.md#L2167`: **`create_or_load_chat_session`**  ·  `{ sessionId?: String, projectId?: Uuid }`  ·  `ChatSessionInitDto { sessionId, draftSessionId, projectId, status, headVersionSeq, messages[], workingState, progress, interruptedTurnAlert? }`  ·  Kiểm tra `sessionId` hợp lệ; nếu truyền `projectId` đã có draft đang `editing`, gắn vào session sở hữu hoặc mở chế độ xem (`core.md` §3.6).  ·  `BEGIN IMMEDIATE` nếu tạo mới dòng `sessions` + `draft_sessions(status='editing')`; chỉ đọc nếu nạp lại session đã có.  ·  `DatabaseError`, `EntityNotFound`  ·  Không áp dụng (hoàn tất tức thời $<10\text{ ms}$).

- IPC-02 `list_chat_sessions` — `docs/business/be-core.md#L2168`: **`list_chat_sessions`**  ·  `{ projectId?: Uuid, limit?: u32, cursor?: String }`  ·  `ChatSessionListDto { items: SessionSummaryDto[], nextCursor? }`  ·  Kiểm tra `limit <= 100`.  ·  Read-only connection (`SELECT` trên `sessions` + `draft_sessions`).  ·  `DatabaseError`  ·  Không áp dụng.

- IPC-03 `send_chat_message` — `docs/business/be-core.md#L2169`: **`send_chat_message`**  ·  `{ sessionId: String, content: String, systemLanguage: "vi" \ ·  "en", feedbackContext?: { code: String, tag: String, title: String, content: String }, onEvent: Channel<ChatStreamEvent> }`  ·  `SendChatTurnResponseDto { turnId: Uuid, userMessageId: Uuid, assistantMessageId: Uuid, assistantText: String, sealedVersion?: DraftVersionSummaryDto, progress: ProgressSummaryDto, pendingPatches: PendingPatchDto[] }`  ·  `content.trim()` không rỗng; nếu có `feedbackContext`, `code` phải tồn tại trong `WorkingState` (hoặc là `"require.project"` / `"optional.overview_phase"` theo `core.md` §3.4). Draft phải đang `editing`.  ·  Lấy `operation_lock` (timeout 30s). Ghi `user` message trước vòng lặp; chạy ReAct loop (không giữ DB lock qua `await`); ghi `tool_result` audit; cuối lượt nếu `has_applied_write == true` thì chạy **`TX-01` (`seal_version`)** với `seal_key = "turn:{turn_id}"` (thử lại 1 lần nếu lỗi).  ·  `DraftBusy`, `DraftNotEditable`, `VersionSealFailed`, `ProviderUnreachable`, `ProviderAuthFailed`, `ValidationFailed`  ·  Hỗ trợ hủy qua `cancel_chat_turn`. Nếu hủy sau khi đã có tool ghi `applied`, vẫn chạy `TX-01` để niêm phong phần đã áp dụng.

- IPC-04 `cancel_chat_turn` — `docs/business/be-core.md#L2170`: **`cancel_chat_turn`**  ·  `{ sessionId: String, turnId: Uuid }`  ·  `{ cancelled: bool, sealedVersion?: DraftVersionSummaryDto }`  ·  Xác minh `turnId` đang chạy trong `DraftRuntimeEntry`.  ·  Kích hoạt `chat_turn_token.cancel()`; chờ `send_chat_message` hoàn tất bước niêm phong `TX-01` (nếu có) rồi trả về.  ·  `EntityNotFound`, `VersionSealFailed`  ·  Lệnh điều khiển hủy; idempotent khi gọi lặp.

- IPC-09 `regenerate_draft_phase` — `docs/business/be-core.md#L2180`: **`regenerate_draft_phase`**  ·  `{ draftSessionId: Uuid, phase: i32, subphase?: i32, systemLanguage: "vi" \ ·  "en", onEvent: Channel<ChatStreamEvent> }`  ·  `RegeneratePhaseResponseDto { turnId: Uuid, sealedVersion?: DraftVersionSummaryDto, pendingPatches: PendingPatchDto[], progress: ProgressSummaryDto }`  ·  Chỉ cho phép thay thế các trường có `source == 'ai_generated'`; giữ nguyên tuyệt đối mọi trường `user` và `user_edited` (`core.md` §1.3, §3.6).  ·  Lấy `operation_lock`; gọi model/workflow sinh lại các trường `ai_generated`; cuối lượt thực thi **`TX-01`** với `reason = 'regenerate'`, `seal_key = "regenerate:{turn_id}"`.  ·  `DraftBusy`, `DraftNotEditable`, `VersionSealFailed`, `ProviderUnreachable`  ·  Hỗ trợ hủy qua `cancel_chat_turn` / `cancel_workflow`.

## 5. Prerequisites

`depends_on: ["phase3_sprint6_deep3", "phase2_sprint4_deep3", "phase3_sprint2_deep0"]`.

- `phase3_sprint6_deep3` [RUNTIME_DEPENDENCY] produces PatchService + pending repo, TX02/TX08 tests and IPC #12–15; inspect its accepted commit, exported artifact/test and `gate_phase3_sprint6_deep3`. If absent, do not start.

- `phase2_sprint4_deep3` [INTEGRATION_DEPENDENCY] produces LlmProvider trait, adapters, credential/redaction APIs and config IPC #38–40; inspect its accepted commit, exported artifact/test and `gate_phase2_sprint4_deep3`. If absent, do not start.

- `phase3_sprint2_deep0` [RUNTIME_DEPENDENCY] produces DraftService, DraftRuntimeRegistry guard, draft/version IPC #5–8,#10–11, TX01/03 fixtures; inspect its accepted commit, exported artifact/test and `gate_phase3_sprint2_deep0`. If absent, do not start.

## 6. Inputs

Required: accepted outputs of all predecessors; frozen committed SSOT ranges above; actual source paths listed below. Optional context: existing UI mocks for fixtures only. No `report/` input; no historical research.

## 7. File ownership

```yaml

allowed_write_paths:

  - src-tauri/src/service/chat_service.rs

  - src-tauri/src/service/prompt_service.rs

  - src-tauri/src/ipc/chat_commands.rs

  - src-tauri/tests/chat_loop.rs

forbidden_write_paths:

  - docs/business/**

  - docs/db.dbml

  - docs/architecture/decisions/**

  - app-data/**

  - report/**

shared_read_only_paths:

  - docs/business/be-core.md

  - docs/business/prompts/overview.md

generated_paths: []

```

Any allowed path may be absent at baseline (not only `src-tauri/`; new FE adapter/shim files are also future targets). Verify with `test -e`/`git ls-files` before editing; existing files are read first. Any extra path requires central integration approval, not a silent scope expansion.

## 8. Public interfaces

PROPOSED_INTERFACE: ChatService::{send,cancel}; PromptComposer::compose; chat IPC #1–4,#9 and ChatStreamEvent. Every consumed prerequisite interface is `PREREQUISITE_INTERFACE`, not an existing runtime guarantee. If a named signature conflicts with committed SSOT, stop `CONTRACT_BLOCKED`.

## 9. Implementation requirements

- Implement prompt/script JIT tools and chat service; own IPC #1–4,#9 only.

- Fake provider sequence tests max-iteration/rejected count/cancel/lost tool result; audit is not recovery proof.

- Prefer official compatible CLI/tooling over manual generated files; verify installed version and diff. No external LLM/Jev request without authorization; optional Jev advice never overrides SSOT.

## 10. Domain invariants

- Run network with no SQLite TX held; every controlled exit seals applied writes or returns failure, never false success.

- Only registry-approved tools execute; system_language UI setting and project.language content are separate.

- No partial state escapes; protected provenance, explicit authorization, no external-effect recovery replay where applicable.

## 11. Failure matrix

| Trigger | Expected behavior / classification | Durable effects and recovery | Required test |

|---|---|---|---|

| Provider timeout | typed failure and seal applied work if possible; P3S7-N. | No unauthorized partial commit; re-read actual durable state before retry | phase3_sprint7_deep4-F1 |

| Model requests UI-only tool | UnauthorizedCaller, no effects; P3S7-C. | No unauthorized partial commit; re-read actual durable state before retry | phase3_sprint7_deep4-F2 |

| Missing accepted predecessor / baseline mismatch | `CONTRACT_BLOCKED`; do not run implementation | No user-data effects; record issue and owner in sprint evidence | phase3_sprint7_deep4-F0 |

## 12. Acceptance tests

Tests below name **future targets**, not tests executed during planning. Each row must become an executable test before the sprint can be accepted.

| Test ID | Initial state | Input / action | Expected output and state | Forbidden side effects / invariant |

|---|---|---|---|---|

| P3S7-P | disposable uniquely-named SQLite S0 + accepted predecessor fixture | In the owned test target, set up the fixture and run the scenario: two valid tool writes produce one sealed Draft Version. | two valid tool writes produce one sealed Draft Version. | No unrelated write; preserve exact expected state and listed invariant |

| P3S7-N | disposable uniquely-named SQLite S0 + accepted predecessor fixture | In the owned test target, set up the fixture and run the scenario: rejected threshold fills all missing tool_result items before final reply. | rejected threshold fills all missing tool_result items before final reply. | No unrelated write; preserve exact expected state and listed invariant |

| P3S7-B | disposable uniquely-named SQLite S0 + accepted predecessor fixture | In the owned test target, set up the fixture and run the scenario: cancellation after applied write performs TX01 or VersionSealFailed, no silent loss. | cancellation after applied write performs TX01 or VersionSealFailed, no silent loss. | No unrelated write; preserve exact expected state and listed invariant |

| phase3_sprint7_deep4-F1 | accepted prerequisite fixture / isolated worktree | Inject Provider timeout into owned operation | typed failure and seal applied work if possible; P3S7-N. | No unauthorized commit or external replay |

| phase3_sprint7_deep4-F2 | accepted prerequisite fixture / isolated worktree | Inject Model requests UI-only tool into owned operation | UnauthorizedCaller, no effects; P3S7-C. | No unauthorized commit or external replay |

| IPC-CMD-001 | isolated authorized and unauthorized caller fixtures | Invoke `create_or_load_chat_session` with valid request and invalid/stale variant; use frozen DTO | /  **`create_or_load_chat_session`**  /  `{ sessionId?: String, projectId?: Uuid }`  /  `ChatSessionInitDto { sessionId, draftSessionId, projectId, status, headVersionSeq, messages[], workingState, progress, interruptedTurnAlert? }`  /  Kiểm tra `sessionId` hợp lệ; nếu truyền `projectId` đã có draft đang `editing`, gắn vào session sở hữu hoặc mở chế độ xem (`core.md` §3.6).  /  `BEGIN IMMEDIATE` nếu tạo mới dòng `sessions` + `draft_sessions(status='editing')`; chỉ đọc nếu nạp lại session đã có.  /  `DatabaseError`, `EntityNotFound`  /  Không áp dụng (hoàn tất tức thời $<10\text{ ms}$).  / | No unlisted command, no unauthorized write; assert event/error/transaction result |

| IPC-CMD-002 | isolated authorized and unauthorized caller fixtures | Invoke `list_chat_sessions` with valid request and invalid/stale variant; use frozen DTO | /  **`list_chat_sessions`**  /  `{ projectId?: Uuid, limit?: u32, cursor?: String }`  /  `ChatSessionListDto { items: SessionSummaryDto[], nextCursor? }`  /  Kiểm tra `limit <= 100`.  /  Read-only connection (`SELECT` trên `sessions` + `draft_sessions`).  /  `DatabaseError`  /  Không áp dụng.  / | No unlisted command, no unauthorized write; assert event/error/transaction result |

| IPC-CMD-003 | isolated authorized and unauthorized caller fixtures | Invoke `send_chat_message` with valid request and invalid/stale variant; use frozen DTO | /  **`send_chat_message`**  /  `{ sessionId: String, content: String, systemLanguage: "vi" \ /  "en", feedbackContext?: { code: String, tag: String, title: String, content: String }, onEvent: Channel<ChatStreamEvent> }`  /  `SendChatTurnResponseDto { turnId: Uuid, userMessageId: Uuid, assistantMessageId: Uuid, assistantText: String, sealedVersion?: DraftVersionSummaryDto, progress: ProgressSummaryDto, pendingPatches: PendingPatchDto[] }`  /  `content.trim()` không rỗng; nếu có `feedbackContext`, `code` phải tồn tại trong `WorkingState` (hoặc là `"require.project"` / `"optional.overview_phase"` theo `core.md` §3.4). Draft phải đang `editing`.  /  Lấy `operation_lock` (timeout 30s). Ghi `user` message trước vòng lặp; chạy ReAct loop (không giữ DB lock qua `await`); ghi `tool_result` audit; cuối lượt nếu `has_applied_write == true` thì chạy **`TX-01` (`seal_version`)** với `seal_key = "turn:{turn_id}"` (thử lại 1 lần nếu lỗi).  /  `DraftBusy`, `DraftNotEditable`, `VersionSealFailed`, `ProviderUnreachable`, `ProviderAuthFailed`, `ValidationFailed`  /  Hỗ trợ hủy qua `cancel_chat_turn`. Nếu hủy sau khi đã có tool ghi `applied`, vẫn chạy `TX-01` để niêm phong phần đã áp dụng.  / | No unlisted command, no unauthorized write; assert event/error/transaction result |

| IPC-CMD-004 | isolated authorized and unauthorized caller fixtures | Invoke `cancel_chat_turn` with valid request and invalid/stale variant; use frozen DTO | /  **`cancel_chat_turn`**  /  `{ sessionId: String, turnId: Uuid }`  /  `{ cancelled: bool, sealedVersion?: DraftVersionSummaryDto }`  /  Xác minh `turnId` đang chạy trong `DraftRuntimeEntry`.  /  Kích hoạt `chat_turn_token.cancel()`; chờ `send_chat_message` hoàn tất bước niêm phong `TX-01` (nếu có) rồi trả về.  /  `EntityNotFound`, `VersionSealFailed`  /  Lệnh điều khiển hủy; idempotent khi gọi lặp.  / | No unlisted command, no unauthorized write; assert event/error/transaction result |

| IPC-CMD-009 | isolated authorized and unauthorized caller fixtures | Invoke `regenerate_draft_phase` with valid request and invalid/stale variant; use frozen DTO | /  **`regenerate_draft_phase`**  /  `{ draftSessionId: Uuid, phase: i32, subphase?: i32, systemLanguage: "vi" \ /  "en", onEvent: Channel<ChatStreamEvent> }`  /  `RegeneratePhaseResponseDto { turnId: Uuid, sealedVersion?: DraftVersionSummaryDto, pendingPatches: PendingPatchDto[], progress: ProgressSummaryDto }`  /  Chỉ cho phép thay thế các trường có `source == 'ai_generated'`; giữ nguyên tuyệt đối mọi trường `user` và `user_edited` (`core.md` §1.3, §3.6).  /  Lấy `operation_lock`; gọi model/workflow sinh lại các trường `ai_generated`; cuối lượt thực thi **`TX-01`** với `reason = 'regenerate'`, `seal_key = "regenerate:{turn_id}"`.  /  `DraftBusy`, `DraftNotEditable`, `VersionSealFailed`, `ProviderUnreachable`  /  Hỗ trợ hủy qua `cancel_chat_turn` / `cancel_workflow`.  / | No unlisted command, no unauthorized write; assert event/error/transaction result |

| phase3_sprint7_deep4-F0 | predecessor gate absent or baseline SHA drift | Attempt dispatch before predecessor accepted | CONTRACT_BLOCKED; no implementation started | No source/DB modification |

Backend module wiring rule: this sprint must not edit any `mod.rs`/`lib.rs`. Its integration test loads each owned implementation file via `#[path = "..."] mod <name>;` and fixture modules via the same mechanism. Production registration occurs only in P6S2 after this sprint is accepted. A compile success is therefore scoped to this test target, not a claim that the whole crate is wired.

Future command after the target files exist: `cargo test --manifest-path src-tauri/Cargo.toml --test chat_loop  # future target, after bootstrap`.

## 13. Database and migration safety

Use `BEGIN IMMEDIATE` only for the assigned TX; WAL/FULL/FK ON/busy_timeout as specified. Use unique disposable DB/temp root per worktree. Never open a user application DB; no destructive migration without separate authorization. Backup/rollback and PRAGMA user_version are owned by P2S3. TX04 never rolls back committed TX08.

## 14. Concurrency contract

- Peer `phase3_sprint10_deep4`: candidate parallel only after frozen contracts/gates; NONE: Disjoint exact write paths; shared types frozen by predecessors; worktree-local test resources.

Isolation: own worktree/branch from baseline or accepted integration SHA; unique temp root, test DB and port; no shared mutable process/global fixture. Same depth alone is NOT permission to dispatch.

## 15. Deliverables

- [ ] ChatService, prompt composer, ChatStreamEvent and IPC #1–4,#9

- [ ] Tests above and evidence including command exit codes, scope diff and accepted commit SHA.

## 16. Completion criteria

`SPRINT_IMPLEMENTED` requires artifacts, `SPRINT_TESTED` requires executed acceptance tests, `SPRINT_INTEGRATED` requires merged checks, and **`SPRINT_ACCEPTED` requires `gate_phase3_sprint7_deep4` accepted**. A contract-evidence sprint cannot be ACCEPTED for an affected implementation successor while its owner decision is unresolved. No agent declaration alone unlocks successors. All required assertions, file scope and baseline must pass.

## 17. Handoff contract

- ChatService, prompt composer, ChatStreamEvent and IPC #1–4,#9

- Exact artifacts: src-tauri/src/service/chat_service.rs, src-tauri/src/service/prompt_service.rs, src-tauri/src/ipc/chat_commands.rs, src-tauri/tests/chat_loop.rs; accepted integration SHA and `gate_phase3_sprint7_deep4` record. A successor must verify exports/ABI rather than infer from prose.

## 18. Resume and failure behavior

Not started: verify baseline and deps. Partial/test failure: inspect git status, changed paths and durable test DB, resume only safe incomplete steps. Implemented but unintegrated: do not dispatch successors. Interrupted agent/rejected patch: preserve branch, rebase only after review, rerun tests. Specification conflict: `CONTRACT_BLOCKED`, record evidence, request authority resolution; never edit SSOT in this sprint.


---

Subagent budget: 256K tokens maximum; start with the bounded references above, do not read entire SSOT by default. CLI-first with verified tool compatibility. Treat Jev/TypeSafe as optional advisory only when available and authorized; never send private repository content externally without consent.
