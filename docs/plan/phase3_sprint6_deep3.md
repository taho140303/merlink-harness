---
sprint_id: phase3_sprint6_deep3
title: Implement independent pending TX-08 and resolve TX-02
phase: 3
sprint: 6
deep: 3
baseline_commit: 57a462efb953a665e3673b15e6cc33622a4f2d39
status: PLANNED
---

# phase3_sprint6_deep3 — Implement independent pending TX-08 and resolve TX-02

## 1. Sprint identity

Engineering phase 3 (Draft domain and durable workflow), within-phase depth 3; baseline is the Git commit above, not an uncommitted workspace. **PROPOSED, NOT IMPLEMENTED.**

## 2. Objective

Implement independent pending TX-08 and resolve TX-02. Observable handoff: PatchService + pending repo, TX02/TX08 tests and IPC #12–15.

## 3. Scope boundaries

**IN_SCOPE:** Implement proposal storage and confirm/reject/list IPC #12–15; no DTO/mod edits.; Test A–F, lost response, concurrent same-key requests, Restore, cumulative dependency invalidation.

**OUT_OF_SCOPE / MUST NOT:** modify authoritative `docs/business/**`, `docs/db.dbml`, `docs/architecture/decisions/**`; use real application data; run production migration; add client request IDs to SF-06; invent business semantics; rewrite another sprint’s files.

## 4. Authoritative contract snapshot

- TX08 commits before patch_id; no TX04 rollback of proposal. Borrow outer operation_lock, never reacquire.

- TX02 data validity and confirmation eligibility checked against cumulative candidate in batch; same-value reuse never adopts new origin.

- **REQ-PATCH-001** `docs/business/core.md#L39-L54`: Protected user/user_edited fields become pending proposals; no direct AI overwrite. (planning coverage `ASSIGNED`).

  Normative excerpt (verbatim baseline lines, truncated to 300 chars; use `sed` for the full range):

  > L39: #### Quy tắc ghi đè theo nguồn gốc dữ liệu

  > L41: / Nguồn hiện có của trường / phần tử / AI chat / regenerate (ghi đè hoặc xóa) được làm gì /

  > L42: / :--- / :--- /

  > L43: / `ai_generated` / AI được ghi đè tự do; chỉ xóa tự do khi không có hậu duệ `user`/`user_edited`. Nếu có hậu duệ được bảo vệ, trả `pending_confirmation` dù cha là `ai_generated`; các thay đổi được gộp vào version chốt cuối lượt (mục 3.6) với `reason = ai_patch` hoặc `regenerate` tương ứng. /

- **REQ-PATCH-002** `docs/business/be-core.md#L493-L506`: TX-08 proposal durability is independent of TX-04 and commits before patch_id response. (planning coverage `ASSIGNED`).

  Normative excerpt (verbatim baseline lines, truncated to 300 chars; use `sed` for the full range):

  > L493: ### 3.6 Ranh giới Giao dịch (Transaction Boundaries) & Hành vi Rollback

  > L495: Mọi thao tác thay đổi dữ liệu bền vững trong Merlink Harness `MUST` nằm gọn trong một trong **8** ranh giới giao dịch nguyên tử (`BEGIN IMMEDIATE ... COMMIT`) dưới đây; TX-08 độc lập với TX-04:

  > L497: / STT / Tên Giao dịch / Bảng SQLite được đọc / ghi trong Transaction / Hành vi khi lỗi / Rollback /

  > L498: / :---: / :--- / :--- / :--- /

- **REQ-PATCH-003** `docs/business/be-core.md#L812-L837`: TX-02 confirm/reject/batch; revalidate data and eligibility on cumulative candidate. (planning coverage `ASSIGNED`).

  Normative excerpt (verbatim baseline lines, truncated to 300 chars; use `sed` for the full range):

  > L812: #### 5.3.2 Quy tắc Chống Trùng và Vòng đời của `pending_patches`

  > L814: Theo `core.md` §1.3, §3.6 và `harness/tools.md` §1:

  > L815: 1. **Tối đa 1 patch `pending` cho mỗi `(draft_session_id, target_code, field_name)`:**

  > L816: - Khi tool handler chuẩn bị tạo một patch `pending` cho `(draft_session_id, target_code, field_name, new_value)` (với `operation = 'delete'`, `field_name` là `NULL`):

- **REQ-PATCH-004** `docs/business/be-core.md#L1538-L1547`: Orphan cases A-F, dedupe origin, Restore expiry, concurrent CAS, legacy fail-closed. (planning coverage `ASSIGNED`).

  Normative excerpt (verbatim baseline lines, truncated to 300 chars; use `sed` for the full range):

  > L1538: **Proposal TX-08 và eligibility:** Tool executor tạo/reuse proposal dưới **guard `operation_lock(draft)` đã thuộc chat turn/workflow orchestrator**, không re-lock mutex không reentrant; guard tồn tại xuyên tool call, SQLite TX-08 chỉ mở quanh DB mutation và không chờ network. TX-08 commit trước resp

  > L1540: / Case / DB / lifecycle sau boundary / Data validity riêng / Confirmation eligibility riêng / Status / schema necessity /

  > L1541: / :--- / :--- / :--- / :--- / :--- /

  > L1542: / A: target sealed S0, round fail trước TX-04 / S0 + TX-08 `pending`, không header/receipt/version mới / Target identity/provenance/previous_value vẫn khớp ⇒ valid; nếu không ⇒ `expired` / Chỉ eligible nếu mọi read dependency fingerprint từ `eligibility_json` khớp head và user/draft/right; không suy

- **REQ-CMD-012** `docs/business/be-core.md#L2129-L2129`: MUST implement typed IPC #12 `list_pending_patches` with the specified authorization, DTO, transaction and error behavior. (planning coverage `ASSIGNED`).

- **REQ-CMD-013** `docs/business/be-core.md#L2130-L2130`: MUST implement typed IPC #13 `confirm_pending_patch` with the specified authorization, DTO, transaction and error behavior. (planning coverage `ASSIGNED`).

- **REQ-CMD-014** `docs/business/be-core.md#L2131-L2131`: MUST implement typed IPC #14 `reject_pending_patch` with the specified authorization, DTO, transaction and error behavior. (planning coverage `ASSIGNED`).

- **REQ-CMD-015** `docs/business/be-core.md#L2132-L2132`: MUST implement typed IPC #15 `confirm_all_pending_patches` with the specified authorization, DTO, transaction and error behavior. (planning coverage `ASSIGNED`).

- **REQ-TX-002** `docs/business/be-core.md#L500-L500`: MUST own TX-02 in one BEGIN IMMEDIATE boundary, with documented rollback and independent effects. (planning coverage `ASSIGNED`).

- **REQ-TX-008** `docs/business/be-core.md#L506-L506`: MUST own TX-08 in one BEGIN IMMEDIATE boundary, with documented rollback and independent effects. (planning coverage `ASSIGNED`).

- **REQ-SPECTEST-T-PROV-01** `docs/business/be-core.md#L2533-L2533`: MUST implement executable BE acceptance vector T-PROV-01, preserving scenario and expected result; old v1-only recovery claims remain legacy-only under B+. (planning coverage `ASSIGNED`).

**Targeted source references (read only these ranges unless a test proves broader context is necessary):**

- `docs/business/core.md#L48-L54` — focus: Immediate proposal and batch semantics; CLI: `sed -n "48,54p" docs/business/core.md` (or `rg -n "<symbol>" docs/business/core.md` for a changed anchor).

- `docs/business/be-core.md#L493-L506` — focus: TX02/TX08 ownership; CLI: `sed -n "493,506p" docs/business/be-core.md` (or `rg -n "<symbol>" docs/business/be-core.md` for a changed anchor).

- `docs/business/be-core.md#L812-L837` — focus: Dedupe and CAS; CLI: `sed -n "812,837p" docs/business/be-core.md` (or `rg -n "<symbol>" docs/business/be-core.md` for a changed anchor).

- `docs/business/be-core.md#L1538-L1547` — focus: Eligibility evidence/orphan A–F; CLI: `sed -n "1538,1547p" docs/business/be-core.md` (or `rg -n "<symbol>" docs/business/be-core.md` for a changed anchor).

**Owned IPC command contracts (verbatim narrow BE §10.2 rows; proposed, not currently implemented):**

- IPC-12 `list_pending_patches` — `docs/business/be-core.md#L2193`: **`list_pending_patches`**  ·  `{ draftSessionId: Uuid }`  ·  `PendingPatchListDto { patches: PendingPatchWithPreviewDto[] }`  ·  Với mỗi patch `status == 'pending'`, thực hiện kiểm tra hết hiệu lực lười (lazy expiration check — §5.3.2) và tính động danh sách phần tử hậu duệ/liên kết bị ảnh hưởng (`affectedEntities[]`) cho patch `delete` (`core.md` §1.3).  ·  Nếu phát hiện patch đã hết hiệu lực, cập nhật `status = 'expired'` xuống SQLite; trả danh sách kèm preview động.  ·  `EntityNotFound`, `DatabaseError`  ·  Không áp dụng.

- IPC-13 `confirm_pending_patch` — `docs/business/be-core.md#L2194`: **`confirm_pending_patch`**  ·  `{ draftSessionId: Uuid, patchId: Uuid }`  ·  `PatchResolutionDto { status: "confirmed" \ ·  "already_confirmed" \ ·  "expired", sealedVersion?: DraftVersionSummaryDto, progress: ProgressSummaryDto }`  ·  Chỉ `CallerContext::UiAction` (cấm cấp cho AI — `harness/tools.md` §3.3). Compare-and-set trên `patch.status`.  ·  Lấy `operation_lock`; thực thi **`TX-02`**: nếu đã `confirmed` trả ngay `already_confirmed` kèm `seq` cũ; nếu còn `pending` và hợp lệ thì áp dụng vào `WorkingState` (`source = 'user_edited'`), đặt `status = 'confirmed'`, và niêm phong đúng 1 version `ai_patch` (`seal_key = "patch_confirm:{patchId}"`).  ·  `DraftBusy`, `DraftNotEditable`, `EntityNotFound`, `ValidationFailed`, `ProposalEligibilityIndeterminate`, `ConfirmationNotAuthorized`, `VersionSealFailed`  ·  Thực thi cục bộ nguyên tử; idempotent tuyệt đối khi gọi lặp.

- IPC-14 `reject_pending_patch` — `docs/business/be-core.md#L2195`: **`reject_pending_patch`**  ·  `{ draftSessionId: Uuid, patchId: Uuid }`  ·  `PatchResolutionDto { status: "rejected" \ ·  "already_rejected", sealedVersion: None, progress: ProgressSummaryDto }`  ·  Chỉ `CallerContext::UiAction`.  ·  Lấy `operation_lock`; thực thi **`TX-02`**: cập nhật `status = 'rejected'`, `resolved_at = now()`; **không** tạo `draft_versions`.  ·  `DraftBusy`, `DraftNotEditable`, `EntityNotFound`, `DatabaseError`  ·  Thực thi cục bộ nguyên tử; idempotent.

- IPC-15 `confirm_all_pending_patches` — `docs/business/be-core.md#L2196`: **`confirm_all_pending_patches`**  ·  `{ draftSessionId: Uuid, clientRequestId: String }`  ·  `ConfirmAllPatchesResponseDto { applied: Uuid[], skipped: { patchId: Uuid, reason: String }[], sealedVersion?: DraftVersionSummaryDto, progress: ProgressSummaryDto }`  ·  Chỉ `CallerContext::UiAction`. Duyệt các patch `pending` theo `created_at ASC, id ASC` (`core.md` §1.3, §3.6).  ·  Lấy `operation_lock`; thực thi **`TX-02`**: revalidate từng patch, bỏ qua patch lỗi/expired vào `skipped` (không rollback cả batch), áp dụng các patch hợp lệ vào `WorkingState` (`source = 'user_edited'`), và chỉ tạo đúng **1** version `ai_patch` (`seal_key = "patch_batch:{clientRequestId}"`) nếu `applied.len() > 0`.  ·  `DraftBusy`, `DraftNotEditable`, `ProposalEligibilityIndeterminate` (theo patch trong `skipped`), `ConfirmationNotAuthorized`, `VersionSealFailed`, `SealKeyPayloadMismatch`  ·  Thực thi cục bộ nguyên tử; idempotent theo `clientRequestId`.

## 5. Prerequisites

`depends_on: ["phase3_sprint2_deep0", "phase3_sprint5_deep2"]`.

- `phase3_sprint2_deep0` [RUNTIME_DEPENDENCY] produces DraftService, DraftRuntimeRegistry guard, draft/version IPC #5–8,#10–11, TX01/03 fixtures; inspect its accepted commit, exported artifact/test and `gate_phase3_sprint2_deep0`. If absent, do not start.

- `phase3_sprint5_deep2` [RUNTIME_DEPENDENCY] produces Backend-owned typed tool outcomes, mutation evidence and full Q1 cascade implementation; inspect its accepted commit, exported artifact/test and `gate_phase3_sprint5_deep2`. If absent, do not start.

## 6. Inputs

Required: accepted outputs of all predecessors; frozen committed SSOT ranges above; actual source paths listed below. Optional context: existing UI mocks for fixtures only. No `report/` input; no historical research.

## 7. File ownership

```yaml

allowed_write_paths:

  - src-tauri/src/storage/pending_repo.rs

  - src-tauri/src/service/patch_service.rs

  - src-tauri/src/ipc/patch_commands.rs

  - src-tauri/tests/pending_transactions.rs

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

PROPOSED_INTERFACE: PatchService::{persist_proposal,confirm,reject,confirm_all}; TX08 and TX02; patch IPC #12–15. Every consumed prerequisite interface is `PREREQUISITE_INTERFACE`, not an existing runtime guarantee. If a named signature conflicts with committed SSOT, stop `CONTRACT_BLOCKED`.

## 9. Implementation requirements

- Implement proposal storage and confirm/reject/list IPC #12–15; no DTO/mod edits.

- Test A–F, lost response, concurrent same-key requests, Restore, cumulative dependency invalidation.

- Prefer official compatible CLI/tooling over manual generated files; verify installed version and diff. No external LLM/Jev request without authorization; optional Jev advice never overrides SSOT.

## 10. Domain invariants

- TX08 commits before patch_id; no TX04 rollback of proposal. Borrow outer operation_lock, never reacquire.

- TX02 data validity and confirmation eligibility checked against cumulative candidate in batch; same-value reuse never adopts new origin.

- No partial state escapes; protected provenance, explicit authorization, no external-effect recovery replay where applicable.

## 11. Failure matrix

| Trigger | Expected behavior / classification | Durable effects and recovery | Required test |

|---|---|---|---|

| Missing dependency evidence | ProposalEligibilityIndeterminate, pending retained; P3S6-N. | No unauthorized partial commit; re-read actual durable state before retry | phase3_sprint6_deep3-F1 |

| Lost TX08 response | old patch_id lookup; no duplicate; P3S6-C. | No unauthorized partial commit; re-read actual durable state before retry | phase3_sprint6_deep3-F2 |

| Missing accepted predecessor / baseline mismatch | `CONTRACT_BLOCKED`; do not run implementation | No user-data effects; record issue and owner in sprint evidence | phase3_sprint6_deep3-F0 |

## 12. Acceptance tests

Tests below name **future targets**, not tests executed during planning. Each row must become an executable test before the sprint can be accepted.

| Test ID | Initial state | Input / action | Expected output and state | Forbidden side effects / invariant |

|---|---|---|---|---|

| P3S6-P | disposable uniquely-named SQLite S0 + accepted predecessor fixture | In the owned test target, set up the fixture and run the scenario: TX08 commit survives later TX04 failure; confirm valid S0 proposal. | TX08 commit survives later TX04 failure; confirm valid S0 proposal. | No unrelated write; preserve exact expected state and listed invariant |

| P3S6-N | disposable uniquely-named SQLite S0 + accepted predecessor fixture | In the owned test target, set up the fixture and run the scenario: orphan target only unsealed becomes expired. | orphan target only unsealed becomes expired. | No unrelated write; preserve exact expected state and listed invariant |

| P3S6-B | disposable uniquely-named SQLite S0 + accepted predecessor fixture | In the owned test target, set up the fixture and run the scenario: batch A mutates B-dependency and later proposal skipped, one version max. | batch A mutates B-dependency and later proposal skipped, one version max. | No unrelated write; preserve exact expected state and listed invariant |

| P3S6-C | disposable uniquely-named SQLite S0 + accepted predecessor fixture | In the owned test target, set up the fixture and run the scenario: same value/new origin does not attest new dependency. | same value/new origin does not attest new dependency. | No unrelated write; preserve exact expected state and listed invariant |

| phase3_sprint6_deep3-F1 | accepted prerequisite fixture / isolated worktree | Inject Missing dependency evidence into owned operation | ProposalEligibilityIndeterminate, pending retained; P3S6-N. | No unauthorized commit or external replay |

| phase3_sprint6_deep3-F2 | accepted prerequisite fixture / isolated worktree | Inject Lost TX08 response into owned operation | old patch_id lookup; no duplicate; P3S6-C. | No unauthorized commit or external replay |

| IPC-CMD-012 | isolated authorized and unauthorized caller fixtures | Invoke `list_pending_patches` with valid request and invalid/stale variant; use frozen DTO | /  **`list_pending_patches`**  /  `{ draftSessionId: Uuid }`  /  `PendingPatchListDto { patches: PendingPatchWithPreviewDto[] }`  /  Với mỗi patch `status == 'pending'`, thực hiện kiểm tra hết hiệu lực lười (lazy expiration check — §5.3.2) và tính động danh sách phần tử hậu duệ/liên kết bị ảnh hưởng (`affectedEntities[]`) cho patch `delete` (`core.md` §1.3).  /  Nếu phát hiện patch đã hết hiệu lực, cập nhật `status = 'expired'` xuống SQLite; trả danh sách kèm preview động.  /  `EntityNotFound`, `DatabaseError`  /  Không áp dụng.  / | No unlisted command, no unauthorized write; assert event/error/transaction result |

| IPC-CMD-013 | isolated authorized and unauthorized caller fixtures | Invoke `confirm_pending_patch` with valid request and invalid/stale variant; use frozen DTO | /  **`confirm_pending_patch`**  /  `{ draftSessionId: Uuid, patchId: Uuid }`  /  `PatchResolutionDto { status: "confirmed" \ /  "already_confirmed" \ /  "expired", sealedVersion?: DraftVersionSummaryDto, progress: ProgressSummaryDto }`  /  Chỉ `CallerContext::UiAction` (cấm cấp cho AI — `harness/tools.md` §3.3). Compare-and-set trên `patch.status`.  /  Lấy `operation_lock`; thực thi **`TX-02`**: nếu đã `confirmed` trả ngay `already_confirmed` kèm `seq` cũ; nếu còn `pending` và hợp lệ thì áp dụng vào `WorkingState` (`source = 'user_edited'`), đặt `status = 'confirmed'`, và niêm phong đúng 1 version `ai_patch` (`seal_key = "patch_confirm:{patchId}"`).  /  `DraftBusy`, `DraftNotEditable`, `EntityNotFound`, `ValidationFailed`, `ProposalEligibilityIndeterminate`, `ConfirmationNotAuthorized`, `VersionSealFailed`  /  Thực thi cục bộ nguyên tử; idempotent tuyệt đối khi gọi lặp.  / | No unlisted command, no unauthorized write; assert event/error/transaction result |

| IPC-CMD-014 | isolated authorized and unauthorized caller fixtures | Invoke `reject_pending_patch` with valid request and invalid/stale variant; use frozen DTO | /  **`reject_pending_patch`**  /  `{ draftSessionId: Uuid, patchId: Uuid }`  /  `PatchResolutionDto { status: "rejected" \ /  "already_rejected", sealedVersion: None, progress: ProgressSummaryDto }`  /  Chỉ `CallerContext::UiAction`.  /  Lấy `operation_lock`; thực thi **`TX-02`**: cập nhật `status = 'rejected'`, `resolved_at = now()`; **không** tạo `draft_versions`.  /  `DraftBusy`, `DraftNotEditable`, `EntityNotFound`, `DatabaseError`  /  Thực thi cục bộ nguyên tử; idempotent.  / | No unlisted command, no unauthorized write; assert event/error/transaction result |

| IPC-CMD-015 | isolated authorized and unauthorized caller fixtures | Invoke `confirm_all_pending_patches` with valid request and invalid/stale variant; use frozen DTO | /  **`confirm_all_pending_patches`**  /  `{ draftSessionId: Uuid, clientRequestId: String }`  /  `ConfirmAllPatchesResponseDto { applied: Uuid[], skipped: { patchId: Uuid, reason: String }[], sealedVersion?: DraftVersionSummaryDto, progress: ProgressSummaryDto }`  /  Chỉ `CallerContext::UiAction`. Duyệt các patch `pending` theo `created_at ASC, id ASC` (`core.md` §1.3, §3.6).  /  Lấy `operation_lock`; thực thi **`TX-02`**: revalidate từng patch, bỏ qua patch lỗi/expired vào `skipped` (không rollback cả batch), áp dụng các patch hợp lệ vào `WorkingState` (`source = 'user_edited'`), và chỉ tạo đúng **1** version `ai_patch` (`seal_key = "patch_batch:{clientRequestId}"`) nếu `applied.len() > 0`.  /  `DraftBusy`, `DraftNotEditable`, `ProposalEligibilityIndeterminate` (theo patch trong `skipped`), `ConfirmationNotAuthorized`, `VersionSealFailed`, `SealKeyPayloadMismatch`  /  Thực thi cục bộ nguyên tử; idempotent theo `clientRequestId`.  / | No unlisted command, no unauthorized write; assert event/error/transaction result |

| TX-002 | disposable S0 + failure injection at every write step | Execute TX-02 then inject error/lose response | /  **TX-02**  /  **Resolve Pending Patch(es)** (`confirm_pending_patch`, `reject_pending_patch`, `confirm_all_pending_patches`)  /  Đọc/Ghi `pending_patches`, `draft_versions` (tạo tối đa 1 version `ai_patch` khi có ít nhất 1 patch được áp dụng), `draft_sessions`.  /  Với `confirm_all_pending_patches`: từng patch lỗi logic được bỏ qua (`skipped`) ở bước kiểm tra trong bộ nhớ; việc ghi tập patch hợp lệ và tạo 1 version `ai_patch` diễn ra nguyên tử trong 1 transaction.  / | Atomic rollback, no unrelated durable effect; retry checks receipt |

| TX-008 | disposable S0 + failure injection at every write step | Execute TX-08 then inject error/lose response | /  **TX-08**  /  **Persist Pending Proposal** (tool executor)  /  Đọc/ghi `pending_patches`: insert, cùng-key dedupe, expire+replace, lazy expiry liên quan. Dưới **một** `operation_lock(draft)` do chat turn/workflow orchestrator đã giữ (tool executor nhận guard/token, **không re-lock**) và `BEGIN IMMEDIATE`, commit trước tool response `patch_id`; không giữ SQLite TX qua network await.  /  Rollback riêng TX-08 khi lỗi; proposal đã commit vẫn tồn tại nếu TX-04 sau đó fail. Không đưa row proposal vào pure Working State replay.  / | Atomic rollback, no unrelated durable effect; retry checks receipt |

| T-PROV-01 | disposable/synthetic fixture described by BE | AI gọi `set_character` cập nhật 3 trường trong 1 call: trường A (`ai_generated`), trường B (`user`), trường C (thiếu khóa trong `declared_sources`). Sau đó gọi `confirm_all_pending_patches`. | Trường A được áp dụng ngay (`applied`); trường B không đổi trong `WorkingState` và tạo đúng 1 `pending_patch` (`pending_confirmation`); trường C bị `rejected`. Khi gọi `confirm_all_pending_patches`, trường B được áp dụng với `source = 'user_edited'` và tạo đúng **1** version `ai_patch`. | Only owner-authorized effects; old v1-only statements interpreted as legacy, no B+ regression |

| phase3_sprint6_deep3-F0 | predecessor gate absent or baseline SHA drift | Attempt dispatch before predecessor accepted | CONTRACT_BLOCKED; no implementation started | No source/DB modification |

Backend module wiring rule: this sprint must not edit any `mod.rs`/`lib.rs`. Its integration test loads each owned implementation file via `#[path = "..."] mod <name>;` and fixture modules via the same mechanism. Production registration occurs only in P6S2 after this sprint is accepted. A compile success is therefore scoped to this test target, not a claim that the whole crate is wired.

Future command after the target files exist: `cargo test --manifest-path src-tauri/Cargo.toml --test pending_transactions  # future target, after bootstrap`.

## 13. Database and migration safety

Use `BEGIN IMMEDIATE` only for the assigned TX; WAL/FULL/FK ON/busy_timeout as specified. Use unique disposable DB/temp root per worktree. Never open a user application DB; no destructive migration without separate authorization. Backup/rollback and PRAGMA user_version are owned by P2S3. TX04 never rolls back committed TX08.

## 14. Concurrency contract

- No same-phase/same-depth peer. Cross-phase scheduling still requires exact predecessor gates and conflict scan.

Isolation: own worktree/branch from baseline or accepted integration SHA; unique temp root, test DB and port; no shared mutable process/global fixture. Same depth alone is NOT permission to dispatch.

## 15. Deliverables

- [ ] PatchService + pending repo, TX02/TX08 tests and IPC #12–15

- [ ] Tests above and evidence including command exit codes, scope diff and accepted commit SHA.

## 16. Completion criteria

`SPRINT_IMPLEMENTED` requires artifacts, `SPRINT_TESTED` requires executed acceptance tests, `SPRINT_INTEGRATED` requires merged checks, and **`SPRINT_ACCEPTED` requires `gate_phase3_sprint6_deep3` accepted**. A contract-evidence sprint cannot be ACCEPTED for an affected implementation successor while its owner decision is unresolved. No agent declaration alone unlocks successors. All required assertions, file scope and baseline must pass.

## 17. Handoff contract

- PatchService + pending repo, TX02/TX08 tests and IPC #12–15

- Exact artifacts: src-tauri/src/storage/pending_repo.rs, src-tauri/src/service/patch_service.rs, src-tauri/src/ipc/patch_commands.rs, src-tauri/tests/pending_transactions.rs; accepted integration SHA and `gate_phase3_sprint6_deep3` record. A successor must verify exports/ABI rather than infer from prose.

## 18. Resume and failure behavior

Not started: verify baseline and deps. Partial/test failure: inspect git status, changed paths and durable test DB, resume only safe incomplete steps. Implemented but unintegrated: do not dispatch successors. Interrupted agent/rejected patch: preserve branch, rebase only after review, rerun tests. Specification conflict: `CONTRACT_BLOCKED`, record evidence, request authority resolution; never edit SSOT in this sprint.


---

Subagent budget: 256K tokens maximum; start with the bounded references above, do not read entire SSOT by default. CLI-first with verified tool compatibility. Treat Jev/TypeSafe as optional advisory only when available and authorized; never send private repository content externally without consent.
