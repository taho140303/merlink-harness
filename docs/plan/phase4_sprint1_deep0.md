---
sprint_id: phase4_sprint1_deep0
title: Run segmented workflow and cancellation
phase: 4
sprint: 1
deep: 0
baseline_commit: 57a462efb953a665e3673b15e6cc33622a4f2d39
status: PLANNED
---

# phase4_sprint1_deep0 — Run segmented workflow and cancellation

## 1. Sprint identity

Engineering phase 4 (Backend services and Writing Mode), within-phase depth 0; baseline is the Git commit above, not an uncommitted workspace. **PROPOSED, NOT IMPLEMENTED.**

## 2. Objective

Run segmented workflow and cancellation. Observable handoff: WorkflowService, WorkflowProgressEvent and IPC #16–18.

## 3. Scope boundaries

**IN_SCOPE:** Implement workflow orchestrator over accepted planner/codec/sealer/chat; IPC #16–18.; Test cancellation before/after TX04, failed task, pending TX08 survival, crash/replan, phase3 refs.

**OUT_OF_SCOPE / MUST NOT:** modify authoritative `docs/business/**`, `docs/db.dbml`, `docs/architecture/decisions/**`; use real application data; run production migration; add client request IDs to SF-06; invent business semantics; rewrite another sprint’s files.

## 4. Authoritative contract snapshot

- Phase1–2 runs sequentially; Phase3 probe→merge→parent-first write; Phase5 subphases do not mix.

- LLM compute parallel without SQLite TX; apply authenticated mutations by task_index under per-draft guard.

- Where plan generation depends on Phase-3 merged world/lore items, an uncommitted merger result cannot be recovered from report cache alone: persisted probe artifacts support deterministic plan reconstruction only after P1S3 proves the consumer; missing artifact means replan without fabricated completed round.

- **REQ-WF-002** `docs/business/core.md#L850-L867`: Cancellation waits for started mutations and seals valid completed round; no fake saved state. (planning coverage `ASSIGNED`).

  Normative excerpt (verbatim baseline lines, truncated to 300 chars; use `sed` for the full range):

  > L850: #### Khóa ghi và hủy workflow

  > L852: * Mỗi draft có một khóa ghi riêng theo `draft_session_id`. Trong một vòng workflow, chỉ một writer được phép thay đổi Working State; chat làm thay đổi draft, sửa tay, commit và restore không được thực thi đồng thời với vòng đang ghi. Các thao tác đọc không làm thay đổi State vẫn được phép nhưng phải

  > L853: * `DRAFT_LOCK_TIMEOUT = 30s` là thời gian tối đa chờ lấy khóa, không phải thời gian tối đa giữ khóa. Không tự thêm giới hạn 30 giây cho thời gian thực thi workflow.

  > L854: * Hủy workflow là yêu cầu dừng có kiểm soát:

- **REQ-WF-003** `docs/business/core.md#L943-L945`: Phase3 probe, pure merge/tree, parent-before-child write; temp reports not durable proof. (planning coverage `CONTRACT_VALIDATION_REQUIRED`).

  Normative excerpt (verbatim baseline lines, truncated to 300 chars; use `sed` for the full range):

  > L943: * **Ba giai đoạn Phase 3:** (1) thăm dò: mỗi subagent đọc phần chương của mình, ghi report địa hình/tổ chức/lore; (2) gộp: hàm thuần ở backend gộp report, loại trùng, dựng cây cha–con rồi gọi `plan_phase` chia danh sách đã gộp; (3) ghi: subagent ghi theo danh sách đã gộp và theo từng tầng của cây, *

  > L944: * **Phase 5:** Chạy tuần tự theo sub-phase 5.1, 5.2, 5.3; mỗi vòng thuộc đúng một sub-phase, không trộn nhiều sub-phase trong cùng vòng. Version `workflow_round` của vòng có thay đổi State ghi `phase = 5` và `subphase` tương ứng.

  > L945: * **Report trung gian:** Backend tạo thư mục `./merlink-harness/tmp/<draft_session_id>/<phase>/` sau khi phase trước hoàn thành và trước khi phase này bắt đầu. Dùng `draft_session_id` để tách report giữa các draft trước commit, không lấy địa chỉ cố định `require.project` làm `project_code`. Chỉ back

- **REQ-CMD-016** `docs/business/be-core.md#L2133-L2133`: MUST implement typed IPC #16 `start_or_resume_workflow` with the specified authorization, DTO, transaction and error behavior. (planning coverage `ASSIGNED`).

- **REQ-CMD-017** `docs/business/be-core.md#L2134-L2134`: MUST implement typed IPC #17 `cancel_workflow` with the specified authorization, DTO, transaction and error behavior. (planning coverage `ASSIGNED`).

- **REQ-CMD-018** `docs/business/be-core.md#L2135-L2135`: MUST implement typed IPC #18 `get_workflow_status` with the specified authorization, DTO, transaction and error behavior. (planning coverage `ASSIGNED`).

**Targeted source references (read only these ranges unless a test proves broader context is necessary):**

- `docs/business/core.md#L835-L952` — focus: Sequential/parallel phase plan and cancel; CLI: `sed -n "835,952p" docs/business/core.md` (or `rg -n "<symbol>" docs/business/core.md` for a changed anchor).

- `docs/business/be-core.md#L1000-L1090` — focus: Workflow service; CLI: `sed -n "1000,1090p" docs/business/be-core.md` (or `rg -n "<symbol>" docs/business/be-core.md` for a changed anchor).

- `docs/business/be-core.md#L1508-L1549` — focus: B+ evidence/artifacts; CLI: `sed -n "1508,1549p" docs/business/be-core.md` (or `rg -n "<symbol>" docs/business/be-core.md` for a changed anchor).

- `docs/business/be-core.md#L2198-L2204` — focus: Workflow IPC #16–18; CLI: `sed -n "2198,2204p" docs/business/be-core.md` (or `rg -n "<symbol>" docs/business/be-core.md` for a changed anchor).

**Owned IPC command contracts (verbatim narrow BE §10.2 rows; proposed, not currently implemented):**

- IPC-16 `start_or_resume_workflow` — `docs/business/be-core.md#L2202`: **`start_or_resume_workflow`**  ·  `{ draftSessionId: Uuid, phase: i32, subphase?: i32, workflowRunId?: Uuid, systemLanguage: "vi" \ ·  "en", onEvent: Channel<WorkflowProgressEvent> }`  ·  `WorkflowExecutionSummaryDto { workflowRunId: Uuid, planGeneration: i32, status: "completed" \ ·  "cancelled" \ ·  "failed" \ ·  "requires_replan", recoveryReason?: "insufficient_task_evidence", completedTasks: u32, totalTasks: u32, lastSealedVersion?: DraftVersionSummaryDto, pendingPatches: PendingPatchDto[] }`  ·  Kiểm tra `phase in 1..=6`; kiểm tra cấu hình `0 <= outlineLookback < outlineCapacity`. Khi resume, phân loại receipt theo §6.4.4; thiếu bằng chứng trung gian → DTO `status = "requires_replan", recoveryReason = "insufficient_task_evidence"`, không skip hoặc tự chạy lại receipt cũ.  ·  Giữ `operation_lock` theo từng vòng. Không giữ DB lock khi các subagent gọi LLM. Cuối mỗi vòng, thực thi **`TX-04` (`seal_workflow_round_tx`)** để ghi nguyên tử `draft_versions` (`reason = 'workflow_round'` chỉ nếu thay đổi ròng cuối vòng) và `workflow_task_receipts` (`SF-06`).  ·  `DraftBusy`, `DraftNotEditable`, `StaleWorkflowReceipt`, `VersionSealFailed`, `ProviderUnreachable`  ·  Hỗ trợ hủy có kiểm soát qua `cancel_workflow`.

- IPC-17 `cancel_workflow` — `docs/business/be-core.md#L2203`: **`cancel_workflow`**  ·  `{ draftSessionId: Uuid, workflowRunId: Uuid }`  ·  `CancelWorkflowResponseDto { status: "cancelled", lastSealedVersion?: DraftVersionSummaryDto, resumeAvailable: bool }`  ·  Xác minh `workflowRunId` đang chạy trên `draftSessionId`.  ·  Đặt cờ hủy; không khởi chạy task mới; chờ mutation đang chạy trong RAM hoàn tất; thực thi **`TX-04`** để niêm phong phần thay đổi `applied` của vòng hiện tại trước khi nhả `operation_lock` (`core.md` §3.9).  ·  `EntityNotFound`, `VersionSealFailed`  ·  Không rollback các thay đổi đã `applied`; nếu niêm phong thất bại trả `VersionSealFailed`.

- IPC-18 `get_workflow_status` — `docs/business/be-core.md#L2204`: **`get_workflow_status`**  ·  `{ draftSessionId: Uuid }`  ·  `WorkflowStatusDto { isRunning: bool, workflowRunId?: Uuid, planGeneration?: i32, currentPhase?: i32, currentSubphase?: i32, currentRound?: u32, totalRounds?: u32, completedTasks: u32, totalTasks: u32, resumeAvailable: bool }`  ·  Đọc runtime + SQLite; `resumeAvailable` không có nghĩa là mọi receipt có thể skip, phải phân loại chứng cứ §6.4.  ·  Read-only (không chờ `operation_lock`).  ·  `EntityNotFound`, `DatabaseError`  ·  Không áp dụng.

## 5. Prerequisites

`depends_on: ["phase3_sprint7_deep4", "phase3_sprint10_deep4", "phase3_sprint1_deep0"]`.

- `phase3_sprint7_deep4` [INTEGRATION_DEPENDENCY] produces ChatService, prompt composer, ChatStreamEvent and IPC #1–4,#9; inspect its accepted commit, exported artifact/test and `gate_phase3_sprint7_deep4`. If absent, do not start.

- `phase3_sprint10_deep4` [INTEGRATION_DEPENDENCY] produces seal_or_retry_round and workflow seal repository, TX04 test fixture; inspect its accepted commit, exported artifact/test and `gate_phase3_sprint10_deep4`. If absent, do not start.

- `phase3_sprint1_deep0` [INTEGRATION_DEPENDENCY] produces plan_outline_runs/split_balanced/plan_phase and progress/validation pure APIs; inspect its accepted commit, exported artifact/test and `gate_phase3_sprint1_deep0`. If absent, do not start.

## 6. Inputs

Required: accepted outputs of all predecessors; frozen committed SSOT ranges above; actual source paths listed below. Optional context: existing UI mocks for fixtures only. No `report/` input; no historical research.

## 7. File ownership

```yaml

allowed_write_paths:

  - src-tauri/src/service/workflow_service.rs

  - src-tauri/src/ipc/workflow_commands.rs

  - src-tauri/tests/workflow_orchestration.rs

forbidden_write_paths:

  - docs/business/**

  - docs/db.dbml

  - docs/architecture/decisions/**

  - app-data/**

  - report/**

shared_read_only_paths:

  - docs/business/core.md

  - docs/business/be-core.md

generated_paths: []

```

Any allowed path may be absent at baseline (not only `src-tauri/`; new FE adapter/shim files are also future targets). Verify with `test -e`/`git ls-files` before editing; existing files are read first. Any extra path requires central integration approval, not a silent scope expansion.

## 8. Public interfaces

PROPOSED_INTERFACE: WorkflowService::{start_or_resume,cancel,status}; workflow IPC #16–18 and WorkflowProgressEvent. Every consumed prerequisite interface is `PREREQUISITE_INTERFACE`, not an existing runtime guarantee. If a named signature conflicts with committed SSOT, stop `CONTRACT_BLOCKED`.

## 9. Implementation requirements

- Implement workflow orchestrator over accepted planner/codec/sealer/chat; IPC #16–18.

- Test cancellation before/after TX04, failed task, pending TX08 survival, crash/replan, phase3 refs.

- Prefer official compatible CLI/tooling over manual generated files; verify installed version and diff. No external LLM/Jev request without authorization; optional Jev advice never overrides SSOT.

## 10. Domain invariants

- Phase1–2 runs sequentially; Phase3 probe→merge→parent-first write; Phase5 subphases do not mix.

- LLM compute parallel without SQLite TX; apply authenticated mutations by task_index under per-draft guard.

- Where plan generation depends on Phase-3 merged world/lore items, an uncommitted merger result cannot be recovered from report cache alone: persisted probe artifacts support deterministic plan reconstruction only after P1S3 proves the consumer; missing artifact means replan without fabricated completed round.

- No partial state escapes; protected provenance, explicit authorization, no external-effect recovery replay where applicable.

## 11. Failure matrix

| Trigger | Expected behavior / classification | Durable effects and recovery | Required test |

|---|---|---|---|

| Missing artifact/invalid head | requires_replan, not LLM replay; P4S1-N. | No unauthorized partial commit; re-read actual durable state before retry | phase4_sprint1_deep0-F1 |

| Cancellation during mutation | wait for started tool, then seal eligible completed round or fail closed; P4S1-C. | No unauthorized partial commit; re-read actual durable state before retry | phase4_sprint1_deep0-F2 |

| Missing accepted predecessor / baseline mismatch | `CONTRACT_BLOCKED`; do not run implementation | No user-data effects; record issue and owner in sprint evidence | phase4_sprint1_deep0-F0 |

## 12. Acceptance tests

Tests below name **future targets**, not tests executed during planning. Each row must become an executable test before the sprint can be accepted.

| Test ID | Initial state | Input / action | Expected output and state | Forbidden side effects / invariant |

|---|---|---|---|---|

| P4S1-P | disposable uniquely-named SQLite S0 + accepted predecessor fixture | In the owned test target, set up the fixture and run the scenario: P6 tasks compute concurrently and state applies 0..5 order regardless response order. | P6 tasks compute concurrently and state applies 0..5 order regardless response order. | No unrelated write; preserve exact expected state and listed invariant |

| P4S1-N | disposable uniquely-named SQLite S0 + accepted predecessor fixture | In the owned test target, set up the fixture and run the scenario: no external effect replay on retry or restart. | no external effect replay on retry or restart. | No unrelated write; preserve exact expected state and listed invariant |

| P4S1-B | disposable uniquely-named SQLite S0 + accepted predecessor fixture | In the owned test target, set up the fixture and run the scenario: canceled round never reports saved if TX04 failed. | canceled round never reports saved if TX04 failed. | No unrelated write; preserve exact expected state and listed invariant |

| phase4_sprint1_deep0-F1 | accepted prerequisite fixture / isolated worktree | Inject Missing artifact/invalid head into owned operation | requires_replan, not LLM replay; P4S1-N. | No unauthorized commit or external replay |

| phase4_sprint1_deep0-F2 | accepted prerequisite fixture / isolated worktree | Inject Cancellation during mutation into owned operation | wait for started tool, then seal eligible completed round or fail closed; P4S1-C. | No unauthorized commit or external replay |

| IPC-CMD-016 | isolated authorized and unauthorized caller fixtures | Invoke `start_or_resume_workflow` with valid request and invalid/stale variant; use frozen DTO | /  **`start_or_resume_workflow`**  /  `{ draftSessionId: Uuid, phase: i32, subphase?: i32, workflowRunId?: Uuid, systemLanguage: "vi" \ /  "en", onEvent: Channel<WorkflowProgressEvent> }`  /  `WorkflowExecutionSummaryDto { workflowRunId: Uuid, planGeneration: i32, status: "completed" \ /  "cancelled" \ /  "failed" \ /  "requires_replan", recoveryReason?: "insufficient_task_evidence", completedTasks: u32, totalTasks: u32, lastSealedVersion?: DraftVersionSummaryDto, pendingPatches: PendingPatchDto[] }`  /  Kiểm tra `phase in 1..=6`; kiểm tra cấu hình `0 <= outlineLookback < outlineCapacity`. Khi resume, phân loại receipt theo §6.4.4; thiếu bằng chứng trung gian → DTO `status = "requires_replan", recoveryReason = "insufficient_task_evidence"`, không skip hoặc tự chạy lại receipt cũ.  /  Giữ `operation_lock` theo từng vòng. Không giữ DB lock khi các subagent gọi LLM. Cuối mỗi vòng, thực thi **`TX-04` (`seal_workflow_round_tx`)** để ghi nguyên tử `draft_versions` (`reason = 'workflow_round'` chỉ nếu thay đổi ròng cuối vòng) và `workflow_task_receipts` (`SF-06`).  /  `DraftBusy`, `DraftNotEditable`, `StaleWorkflowReceipt`, `VersionSealFailed`, `ProviderUnreachable`  /  Hỗ trợ hủy có kiểm soát qua `cancel_workflow`.  / | No unlisted command, no unauthorized write; assert event/error/transaction result |

| IPC-CMD-017 | isolated authorized and unauthorized caller fixtures | Invoke `cancel_workflow` with valid request and invalid/stale variant; use frozen DTO | /  **`cancel_workflow`**  /  `{ draftSessionId: Uuid, workflowRunId: Uuid }`  /  `CancelWorkflowResponseDto { status: "cancelled", lastSealedVersion?: DraftVersionSummaryDto, resumeAvailable: bool }`  /  Xác minh `workflowRunId` đang chạy trên `draftSessionId`.  /  Đặt cờ hủy; không khởi chạy task mới; chờ mutation đang chạy trong RAM hoàn tất; thực thi **`TX-04`** để niêm phong phần thay đổi `applied` của vòng hiện tại trước khi nhả `operation_lock` (`core.md` §3.9).  /  `EntityNotFound`, `VersionSealFailed`  /  Không rollback các thay đổi đã `applied`; nếu niêm phong thất bại trả `VersionSealFailed`.  / | No unlisted command, no unauthorized write; assert event/error/transaction result |

| IPC-CMD-018 | isolated authorized and unauthorized caller fixtures | Invoke `get_workflow_status` with valid request and invalid/stale variant; use frozen DTO | /  **`get_workflow_status`**  /  `{ draftSessionId: Uuid }`  /  `WorkflowStatusDto { isRunning: bool, workflowRunId?: Uuid, planGeneration?: i32, currentPhase?: i32, currentSubphase?: i32, currentRound?: u32, totalRounds?: u32, completedTasks: u32, totalTasks: u32, resumeAvailable: bool }`  /  Đọc runtime + SQLite; `resumeAvailable` không có nghĩa là mọi receipt có thể skip, phải phân loại chứng cứ §6.4.  /  Read-only (không chờ `operation_lock`).  /  `EntityNotFound`, `DatabaseError`  /  Không áp dụng.  / | No unlisted command, no unauthorized write; assert event/error/transaction result |

| phase4_sprint1_deep0-F0 | predecessor gate absent or baseline SHA drift | Attempt dispatch before predecessor accepted | CONTRACT_BLOCKED; no implementation started | No source/DB modification |

Backend module wiring rule: this sprint must not edit any `mod.rs`/`lib.rs`. Its integration test loads each owned implementation file via `#[path = "..."] mod <name>;` and fixture modules via the same mechanism. Production registration occurs only in P6S2 after this sprint is accepted. A compile success is therefore scoped to this test target, not a claim that the whole crate is wired.

Future command after the target files exist: `cargo test --manifest-path src-tauri/Cargo.toml --test workflow_orchestration  # future target, after bootstrap`.

## 13. Database and migration safety

Use `BEGIN IMMEDIATE` only for the assigned TX; WAL/FULL/FK ON/busy_timeout as specified. Use unique disposable DB/temp root per worktree. Never open a user application DB; no destructive migration without separate authorization. Backup/rollback and PRAGMA user_version are owned by P2S3. TX04 never rolls back committed TX08.

## 14. Concurrency contract

- Peer `phase4_sprint2_deep0`: candidate parallel only after frozen contracts/gates; NONE: Disjoint exact write paths; shared types frozen by predecessors; worktree-local test resources.

Isolation: own worktree/branch from baseline or accepted integration SHA; unique temp root, test DB and port; no shared mutable process/global fixture. Same depth alone is NOT permission to dispatch.

## 15. Deliverables

- [ ] WorkflowService, WorkflowProgressEvent and IPC #16–18

- [ ] Tests above and evidence including command exit codes, scope diff and accepted commit SHA.

## 16. Completion criteria

`SPRINT_IMPLEMENTED` requires artifacts, `SPRINT_TESTED` requires executed acceptance tests, `SPRINT_INTEGRATED` requires merged checks, and **`SPRINT_ACCEPTED` requires `gate_phase4_sprint1_deep0` accepted**. A contract-evidence sprint cannot be ACCEPTED for an affected implementation successor while its owner decision is unresolved. No agent declaration alone unlocks successors. All required assertions, file scope and baseline must pass.

## 17. Handoff contract

- WorkflowService, WorkflowProgressEvent and IPC #16–18

- Exact artifacts: src-tauri/src/service/workflow_service.rs, src-tauri/src/ipc/workflow_commands.rs, src-tauri/tests/workflow_orchestration.rs; accepted integration SHA and `gate_phase4_sprint1_deep0` record. A successor must verify exports/ABI rather than infer from prose.

## 18. Resume and failure behavior

Not started: verify baseline and deps. Partial/test failure: inspect git status, changed paths and durable test DB, resume only safe incomplete steps. Implemented but unintegrated: do not dispatch successors. Interrupted agent/rejected patch: preserve branch, rebase only after review, rerun tests. Specification conflict: `CONTRACT_BLOCKED`, record evidence, request authority resolution; never edit SSOT in this sprint.

**Explicit unblock requirements:** CI-02, CONTRACT_REVIEW_REQUIRED:REQ-WF-003.


---

Subagent budget: 256K tokens maximum; start with the bounded references above, do not read entire SSOT by default. CLI-first with verified tool compatibility. Treat Jev/TypeSafe as optional advisory only when available and authorized; never send private repository content externally without consent.
