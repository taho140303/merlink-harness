---
sprint_id: phase4_sprint2_deep0
title: Commit draft atomically into relational schema
phase: 4
sprint: 2
deep: 0
baseline_commit: 57a462efb953a665e3673b15e6cc33622a4f2d39
status: PLANNED
---

# phase4_sprint2_deep0 — Commit draft atomically into relational schema

## 1. Sprint identity

Engineering phase 4 (Backend services and Writing Mode), within-phase depth 0; baseline is the Git commit above, not an uncommitted workspace. **PROPOSED, NOT IMPLEMENTED.**

## 2. Objective

Commit draft atomically into relational schema. Observable handoff: CommitService/CommitRepo, preview/commit/reopen IPC #19–21.

## 3. Scope boundaries

**IN_SCOPE:** Implement preview, atomic TX05 and reopen restriction; own IPC #19–21.; Use disposable relational DB to test collision rollback, shared character/lore ownership, reopen after written scenes.

**OUT_OF_SCOPE / MUST NOT:** modify authoritative `docs/business/**`, `docs/db.dbml`, `docs/architecture/decisions/**`; use real application data; run production migration; add client request IDs to SF-06; invent business semantics; rewrite another sprint’s files.

## 4. Authoritative contract snapshot

- No commit while pending or validation fails; code→UUID mapping and FK order must follow Core.

- Exact 21-table/22-step dispute resolved by P1S1 before implementation; no speculative schema edits.

- **REQ-COMMIT-001** `docs/business/core.md#L652-L660`: Start/commit blocks pending patches and validation failure, no automatic resolution. (planning coverage `ASSIGNED`).

  Normative excerpt (verbatim baseline lines, truncated to 300 chars; use `sed` for the full range):

  > L652: ### 3.5 Bắt đầu sáng tác (Start Action)

  > L654: * Nút **Start** chỉ xuất hiện duy nhất ở cuối Bước 6 (Relationship).

  > L655: * Khi người dùng nhấn nút này, hệ thống mở **Commit Confirmation Dialog**. Dialog đồng thời là nơi review plan: tóm tắt dữ liệu sẽ ghi, validation status và pending patches. Không có màn hình/phase review riêng.

  > L656: * Commit chỉ được phép khi không còn điều kiện chặn: plan thiếu dữ liệu bắt buộc, validation thất bại hoặc còn `pending_patches`. Dialog hiển thị lỗi và không cho xác nhận khi còn điều kiện chặn.

- **REQ-COMMIT-002** `docs/business/be-core.md#L1582-L1679`: TX-05 topo code→UUID insert, FK/collision rollback, reopen restriction. (planning coverage `CONTRACT_VALIDATION_REQUIRED`).

  Normative excerpt (verbatim baseline lines, truncated to 300 chars; use `sed` for the full range):

  > L1582: ## §7. Draft Commit Engine (Động cơ Commit Bản nháp sang Cơ sở dữ liệu Quan hệ)

  > L1584: ### 7.1 Kiểm tra Điều kiện Tiên quyết (`Pre-Commit Readiness & Invariant Verification`)

  > L1586: Theo `core.md` §2.8, §3.5, §3.7 và `fe-core.md` §7 (`H2 APPROVED`):

  > L1588: Khi người dùng mở **Commit Confirmation Dialog** (`get_commit_confirmation_preview`) hoặc bấm nút xác nhận **Start / Commit** (`commit_draft`):

- **REQ-CMD-019** `docs/business/be-core.md#L2136-L2136`: MUST implement typed IPC #19 `get_commit_confirmation_preview` with the specified authorization, DTO, transaction and error behavior. (planning coverage `ASSIGNED`).

- **REQ-CMD-020** `docs/business/be-core.md#L2137-L2137`: MUST implement typed IPC #20 `commit_draft` with the specified authorization, DTO, transaction and error behavior. (planning coverage `ASSIGNED`).

- **REQ-CMD-021** `docs/business/be-core.md#L2138-L2138`: MUST implement typed IPC #21 `reopen_committed_draft` with the specified authorization, DTO, transaction and error behavior. (planning coverage `ASSIGNED`).

- **REQ-TX-005** `docs/business/be-core.md#L503-L503`: MUST own TX-05 in one BEGIN IMMEDIATE boundary, with documented rollback and independent effects. (planning coverage `ASSIGNED`).

- **REQ-SPECTEST-T-ROLL-01** `docs/business/be-core.md#L2529-L2529`: MUST implement executable BE acceptance vector T-ROLL-01, preserving scenario and expected result; old v1-only recovery claims remain legacy-only under B+. (planning coverage `ASSIGNED`).

**Targeted source references (read only these ranges unless a test proves broader context is necessary):**

- `docs/business/core.md#L652-L660` — focus: Start preview/pending hard gate; CLI: `sed -n "652,660p" docs/business/core.md` (or `rg -n "<symbol>" docs/business/core.md` for a changed anchor).

- `docs/business/core.md#L734-L768` — focus: Core topological and collision policy; CLI: `sed -n "734,768p" docs/business/core.md` (or `rg -n "<symbol>" docs/business/core.md` for a changed anchor).

- `docs/business/be-core.md#L1582-L1679` — focus: TX05 and reopening; CLI: `sed -n "1582,1679p" docs/business/be-core.md` (or `rg -n "<symbol>" docs/business/be-core.md` for a changed anchor).

- `docs/business/be-core.md#L2206-L2212` — focus: Commit IPC #19–21; CLI: `sed -n "2206,2212p" docs/business/be-core.md` (or `rg -n "<symbol>" docs/business/be-core.md` for a changed anchor).

**Owned IPC command contracts (verbatim narrow BE §10.2 rows; proposed, not currently implemented):**

- IPC-19 `get_commit_confirmation_preview` — `docs/business/be-core.md#L2210`: **`get_commit_confirmation_preview`**  ·  `{ draftSessionId: Uuid }`  ·  `CommitPreviewDto { draftSessionId, status: DraftSessionStatus, planSummary: PlanSummaryDto, validation: { status: "ready" \ ·  "blocked", issues: ValidationIssueDto[] }, pendingPatches: { count: usize, items: PendingPatchDto[] }, canConfirm: bool }`  ·  Đọc `WorkingState` tại `head_version_seq`, đếm `pending_patches` có `status == 'pending'`, chạy hàm thuần `validate_draft_for_commit`. `canConfirm = (status == 'editing' && validation.status == 'ready' && pendingPatches.count == 0)`.  ·  Read-only.  ·  `EntityNotFound`, `DatabaseError`  ·  Không áp dụng.

- IPC-20 `commit_draft` — `docs/business/be-core.md#L2211`: **`commit_draft`**  ·  `{ draftSessionId: Uuid }`  ·  `CommitDraftResponseDto { status: "committed", projectId: Uuid, redirectPath: String }`  ·  Chỉ `CallerContext::UiAction`. Nếu đã `committed`, trả ngay `projectId` hiện có (idempotent). Nếu `status == 'editing'`, bắt buộc `pending_patches == 0` và `validate_draft_for_commit` không có lỗi (`core.md` §3.5, §3.7).  ·  Lấy `operation_lock`; đặt `status = 'committing'`; mở giao dịch **`TX-05` (`BEGIN IMMEDIATE`)**: kiểm tra lại collision `code` trên toàn DB, chèn tuần tự 21 bảng theo thứ tự tô-pô (§7.2), cập nhật `draft_sessions(status = 'committed', project_id)` và `sessions(project_id)`. Nếu bất kỳ bước nào lỗi hoặc collision $\rightarrow$ `ROLLBACK` toàn bộ và đặt lại `status = 'editing'`.  ·  `DraftBusy`, `PendingPatchesExist`, `ValidationFailed`, `CodeCollisionOnCommit`, `DatabaseError`  ·  Trong lúc `committing`, UI khóa đóng dialog (`fe-core.md` ST-03); giao dịch SQLite không bị hủy giữa chừng để bảo đảm tính nguyên tử.

- IPC-21 `reopen_committed_draft` — `docs/business/be-core.md#L2212`: **`reopen_committed_draft`**  ·  `{ projectId: Uuid, activeSessionId: String }`  ·  `{ draftSessionId: Uuid, status: "editing" }`  ·  Kiểm tra nghiêm ngặt truy vấn `has_written_scenes` (§7.3): chỉ cho phép khi mọi `scenes.content` của project đều `NULL`/rỗng và `COUNT(scene_versions) == 0`.  ·  `BEGIN IMMEDIATE`: cập nhật `draft_sessions SET status = 'editing', session_id = ?2`.  ·  `DraftReopenForbidden`, `EntityNotFound`, `DatabaseError`  ·  Không áp dụng.

## 5. Prerequisites

`depends_on: ["phase3_sprint2_deep0", "phase3_sprint5_deep2", "phase3_sprint6_deep3", "phase2_sprint6_deep2", "phase1_sprint1_deep0"]`.

- `phase3_sprint2_deep0` [INTEGRATION_DEPENDENCY] produces DraftService, DraftRuntimeRegistry guard, draft/version IPC #5–8,#10–11, TX01/03 fixtures; inspect its accepted commit, exported artifact/test and `gate_phase3_sprint2_deep0`. If absent, do not start.

- `phase3_sprint5_deep2` [INTEGRATION_DEPENDENCY] produces Backend-owned typed tool outcomes, mutation evidence and full Q1 cascade implementation; inspect its accepted commit, exported artifact/test and `gate_phase3_sprint5_deep2`. If absent, do not start.

- `phase3_sprint6_deep3` [INTEGRATION_DEPENDENCY] produces PatchService + pending repo, TX02/TX08 tests and IPC #12–15; inspect its accepted commit, exported artifact/test and `gate_phase3_sprint6_deep3`. If absent, do not start.

- `phase2_sprint6_deep2` [INTEGRATION_DEPENDENCY] produces Frozen DraftRepo/CatalogRepo/WorkflowReadRepo interfaces and temp-store API; inspect its accepted commit, exported artifact/test and `gate_phase2_sprint6_deep2`. If absent, do not start.

- `phase1_sprint1_deep0` [INTEGRATION_DEPENDENCY] produces docs/plan/evidence/contract-ledger.md with issue IDs, accepted SSOT SHA or CONTRACT_BLOCKED and affected sprint IDs; inspect its accepted commit, exported artifact/test and `gate_phase1_sprint1_deep0`. If absent, do not start.

## 6. Inputs

Required: accepted outputs of all predecessors; frozen committed SSOT ranges above; actual source paths listed below. Optional context: existing UI mocks for fixtures only. No `report/` input; no historical research.

## 7. File ownership

```yaml

allowed_write_paths:

  - src-tauri/src/storage/commit_repo.rs

  - src-tauri/src/service/commit_service.rs

  - src-tauri/src/ipc/commit_commands.rs

  - src-tauri/tests/draft_commit.rs

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

PROPOSED_INTERFACE: CommitService::{preview,commit,reopen}; commit IPC #19–21; TX05 one atomic transition. Every consumed prerequisite interface is `PREREQUISITE_INTERFACE`, not an existing runtime guarantee. If a named signature conflicts with committed SSOT, stop `CONTRACT_BLOCKED`.

## 9. Implementation requirements

- Implement preview, atomic TX05 and reopen restriction; own IPC #19–21.

- Use disposable relational DB to test collision rollback, shared character/lore ownership, reopen after written scenes.

- Prefer official compatible CLI/tooling over manual generated files; verify installed version and diff. No external LLM/Jev request without authorization; optional Jev advice never overrides SSOT.

## 10. Domain invariants

- No commit while pending or validation fails; code→UUID mapping and FK order must follow Core.

- Exact 21-table/22-step dispute resolved by P1S1 before implementation; no speculative schema edits.

- No partial state escapes; protected provenance, explicit authorization, no external-effect recovery replay where applicable.

## 11. Failure matrix

| Trigger | Expected behavior / classification | Durable effects and recovery | Required test |

|---|---|---|---|

| Unresolved insert order/count | CONTRACT_BLOCKED, no TX05; P4S2-N. | No unauthorized partial commit; re-read actual durable state before retry | phase4_sprint2_deep0-F1 |

| Collision mid-insert | rollback all rows and keep editing draft; P4S2-C. | No unauthorized partial commit; re-read actual durable state before retry | phase4_sprint2_deep0-F2 |

| Missing accepted predecessor / baseline mismatch | `CONTRACT_BLOCKED`; do not run implementation | No user-data effects; record issue and owner in sprint evidence | phase4_sprint2_deep0-F0 |

## 12. Acceptance tests

Tests below name **future targets**, not tests executed during planning. Each row must become an executable test before the sprint can be accepted.

| Test ID | Initial state | Input / action | Expected output and state | Forbidden side effects / invariant |

|---|---|---|---|---|

| P4S2-P | disposable uniquely-named SQLite S0 + accepted predecessor fixture | In the owned test target, set up the fixture and run the scenario: valid sealed plan commits once with correct code UUID map and mode view. | valid sealed plan commits once with correct code UUID map and mode view. | No unrelated write; preserve exact expected state and listed invariant |

| P4S2-N | disposable uniquely-named SQLite S0 + accepted predecessor fixture | In the owned test target, set up the fixture and run the scenario: pending patch blocks before TX, no ignored proposal. | pending patch blocks before TX, no ignored proposal. | No unrelated write; preserve exact expected state and listed invariant |

| P4S2-B | disposable uniquely-named SQLite S0 + accepted predecessor fixture | In the owned test target, set up the fixture and run the scenario: second Start yields prior project id, no duplicate. | second Start yields prior project id, no duplicate. | No unrelated write; preserve exact expected state and listed invariant |

| phase4_sprint2_deep0-F1 | accepted prerequisite fixture / isolated worktree | Inject Unresolved insert order/count into owned operation | CONTRACT_BLOCKED, no TX05; P4S2-N. | No unauthorized commit or external replay |

| phase4_sprint2_deep0-F2 | accepted prerequisite fixture / isolated worktree | Inject Collision mid-insert into owned operation | rollback all rows and keep editing draft; P4S2-C. | No unauthorized commit or external replay |

| IPC-CMD-019 | isolated authorized and unauthorized caller fixtures | Invoke `get_commit_confirmation_preview` with valid request and invalid/stale variant; use frozen DTO | /  **`get_commit_confirmation_preview`**  /  `{ draftSessionId: Uuid }`  /  `CommitPreviewDto { draftSessionId, status: DraftSessionStatus, planSummary: PlanSummaryDto, validation: { status: "ready" \ /  "blocked", issues: ValidationIssueDto[] }, pendingPatches: { count: usize, items: PendingPatchDto[] }, canConfirm: bool }`  /  Đọc `WorkingState` tại `head_version_seq`, đếm `pending_patches` có `status == 'pending'`, chạy hàm thuần `validate_draft_for_commit`. `canConfirm = (status == 'editing' && validation.status == 'ready' && pendingPatches.count == 0)`.  /  Read-only.  /  `EntityNotFound`, `DatabaseError`  /  Không áp dụng.  / | No unlisted command, no unauthorized write; assert event/error/transaction result |

| IPC-CMD-020 | isolated authorized and unauthorized caller fixtures | Invoke `commit_draft` with valid request and invalid/stale variant; use frozen DTO | /  **`commit_draft`**  /  `{ draftSessionId: Uuid }`  /  `CommitDraftResponseDto { status: "committed", projectId: Uuid, redirectPath: String }`  /  Chỉ `CallerContext::UiAction`. Nếu đã `committed`, trả ngay `projectId` hiện có (idempotent). Nếu `status == 'editing'`, bắt buộc `pending_patches == 0` và `validate_draft_for_commit` không có lỗi (`core.md` §3.5, §3.7).  /  Lấy `operation_lock`; đặt `status = 'committing'`; mở giao dịch **`TX-05` (`BEGIN IMMEDIATE`)**: kiểm tra lại collision `code` trên toàn DB, chèn tuần tự 21 bảng theo thứ tự tô-pô (§7.2), cập nhật `draft_sessions(status = 'committed', project_id)` và `sessions(project_id)`. Nếu bất kỳ bước nào lỗi hoặc collision $\rightarrow$ `ROLLBACK` toàn bộ và đặt lại `status = 'editing'`.  /  `DraftBusy`, `PendingPatchesExist`, `ValidationFailed`, `CodeCollisionOnCommit`, `DatabaseError`  /  Trong lúc `committing`, UI khóa đóng dialog (`fe-core.md` ST-03); giao dịch SQLite không bị hủy giữa chừng để bảo đảm tính nguyên tử.  / | No unlisted command, no unauthorized write; assert event/error/transaction result |

| IPC-CMD-021 | isolated authorized and unauthorized caller fixtures | Invoke `reopen_committed_draft` with valid request and invalid/stale variant; use frozen DTO | /  **`reopen_committed_draft`**  /  `{ projectId: Uuid, activeSessionId: String }`  /  `{ draftSessionId: Uuid, status: "editing" }`  /  Kiểm tra nghiêm ngặt truy vấn `has_written_scenes` (§7.3): chỉ cho phép khi mọi `scenes.content` của project đều `NULL`/rỗng và `COUNT(scene_versions) == 0`.  /  `BEGIN IMMEDIATE`: cập nhật `draft_sessions SET status = 'editing', session_id = ?2`.  /  `DraftReopenForbidden`, `EntityNotFound`, `DatabaseError`  /  Không áp dụng.  / | No unlisted command, no unauthorized write; assert event/error/transaction result |

| TX-005 | disposable S0 + failure injection at every write step | Execute TX-05 then inject error/lose response | /  **TX-05**  /  **Commit Draft to Relational DB** (`commit_draft`)  /  Chèn theo thứ tự tô-pô vào 21 bảng thực thể nghiệp vụ; cập nhật `draft_sessions(status = 'committed', project_id)` và `sessions(project_id)`.  /  Rollback toàn bộ 21 bảng trong cùng transaction; đặt lại `draft_sessions.status = 'editing'`; giữ nguyên bản nháp (`core.md` §3.7).  / | Atomic rollback, no unrelated durable effect; retry checks receipt |

| T-ROLL-01 | disposable/synthetic fixture described by BE | Hai bản nháp $D_1$ và $D_2$ cùng có một nhân vật mới trùng `code = "minh_hoang"`. Commit $D_1$ thành công. Sau đó gọi `commit_draft` cho $D_2$. | Giao dịch `TX-05` của $D_2$ bị rollback toàn bộ: `0` dòng mới nào của $D_2$ lọt vào `projects`, `arcs`, `chapters`, `scenes`...; trạng thái của $D_2$ quay về `'editing'`; trả lỗi `CodeCollisionOnCommit`. | Only owner-authorized effects; old v1-only statements interpreted as legacy, no B+ regression |

| phase4_sprint2_deep0-F0 | predecessor gate absent or baseline SHA drift | Attempt dispatch before predecessor accepted | CONTRACT_BLOCKED; no implementation started | No source/DB modification |

Backend module wiring rule: this sprint must not edit any `mod.rs`/`lib.rs`. Its integration test loads each owned implementation file via `#[path = "..."] mod <name>;` and fixture modules via the same mechanism. Production registration occurs only in P6S2 after this sprint is accepted. A compile success is therefore scoped to this test target, not a claim that the whole crate is wired.

Future command after the target files exist: `cargo test --manifest-path src-tauri/Cargo.toml --test draft_commit  # future target, after bootstrap`.

## 13. Database and migration safety

Use `BEGIN IMMEDIATE` only for the assigned TX; WAL/FULL/FK ON/busy_timeout as specified. Use unique disposable DB/temp root per worktree. Never open a user application DB; no destructive migration without separate authorization. Backup/rollback and PRAGMA user_version are owned by P2S3. TX04 never rolls back committed TX08.

## 14. Concurrency contract

- Peer `phase4_sprint1_deep0`: candidate parallel only after frozen contracts/gates; NONE: Disjoint exact write paths; shared types frozen by predecessors; worktree-local test resources.

Isolation: own worktree/branch from baseline or accepted integration SHA; unique temp root, test DB and port; no shared mutable process/global fixture. Same depth alone is NOT permission to dispatch.

## 15. Deliverables

- [ ] CommitService/CommitRepo, preview/commit/reopen IPC #19–21

- [ ] Tests above and evidence including command exit codes, scope diff and accepted commit SHA.

## 16. Completion criteria

`SPRINT_IMPLEMENTED` requires artifacts, `SPRINT_TESTED` requires executed acceptance tests, `SPRINT_INTEGRATED` requires merged checks, and **`SPRINT_ACCEPTED` requires `gate_phase4_sprint2_deep0` accepted**. A contract-evidence sprint cannot be ACCEPTED for an affected implementation successor while its owner decision is unresolved. No agent declaration alone unlocks successors. All required assertions, file scope and baseline must pass.

## 17. Handoff contract

- CommitService/CommitRepo, preview/commit/reopen IPC #19–21

- Exact artifacts: src-tauri/src/storage/commit_repo.rs, src-tauri/src/service/commit_service.rs, src-tauri/src/ipc/commit_commands.rs, src-tauri/tests/draft_commit.rs; accepted integration SHA and `gate_phase4_sprint2_deep0` record. A successor must verify exports/ABI rather than infer from prose.

## 18. Resume and failure behavior

Not started: verify baseline and deps. Partial/test failure: inspect git status, changed paths and durable test DB, resume only safe incomplete steps. Implemented but unintegrated: do not dispatch successors. Interrupted agent/rejected patch: preserve branch, rebase only after review, rerun tests. Specification conflict: `CONTRACT_BLOCKED`, record evidence, request authority resolution; never edit SSOT in this sprint.

**Explicit unblock requirements:** TX05_MAPPING_AUTHORITY, CI-05, CONTRACT_REVIEW_REQUIRED:REQ-COMMIT-002.


---

Subagent budget: 256K tokens maximum; start with the bounded references above, do not read entire SSOT by default. CLI-first with verified tool compatibility. Treat Jev/TypeSafe as optional advisory only when available and authorized; never send private repository content externally without consent.
