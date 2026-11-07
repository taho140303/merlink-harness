---
sprint_id: phase3_sprint2_deep0
title: Implement draft runtime and TX-01/TX-03
phase: 3
sprint: 2
deep: 0
baseline_commit: 57a462efb953a665e3673b15e6cc33622a4f2d39
status: PLANNED
---

# phase3_sprint2_deep0 — Implement draft runtime and TX-01/TX-03

## 1. Sprint identity

Engineering phase 3 (Draft domain and durable workflow), within-phase depth 0; baseline is the Git commit above, not an uncommitted workspace. **PROPOSED, NOT IMPLEMENTED.**

## 2. Objective

Implement draft runtime and TX-01/TX-03. Observable handoff: DraftService, DraftRuntimeRegistry guard, draft/version IPC #5–8,#10–11, TX01/03 fixtures.

## 3. Scope boundaries

**IN_SCOPE:** Implement draft runtime/repo use and commands #5–8,#10–11; no shared DTO/mod edits.; Test lost-response, identical/different seal-key payload, restore branch/phase/counters, transient seal failure.

**OUT_OF_SCOPE / MUST NOT:** modify authoritative `docs/business/**`, `docs/db.dbml`, `docs/architecture/decisions/**`; use real application data; run production migration; add client request IDs to SF-06; invent business semantics; rewrite another sprint’s files.

## 4. Authoritative contract snapshot

- TX01 seal and TX03 append-only Restore require P1S1 decisions on seal_pending_retry, pending expiry, restore identical payload, nullable legacy phase.

- Single operation_lock held by orchestrator; no nested lock, write TX after awaits only; head CAS under BEGIN IMMEDIATE.

- **REQ-DRAFT-001** `docs/business/core.md#L662-L732`: TX-01 one sealed version per turn; idempotent seal, append-only Restore TX-03, retry/lost response. (planning coverage `CONTRACT_VALIDATION_REQUIRED`).

  Normative excerpt (verbatim baseline lines, truncated to 300 chars; use `sed` for the full range):

  > L662: ### 3.6 Vòng đời Draft & Version

  > L664: * Mục này chỉ nói về `draft_versions` của Planning. `scene_versions` của Writing Mode nằm tại mục 1.6; không trộn hai mô hình version.

  > L665: * `businessFlowData` (Super JSON gồm 6 phase) được lưu bền trong SQLite (bảng draft), không chỉ nằm trong RAM.

  > L666: * **Sở hữu State và khóa ghi:** Draft là đơn vị sở hữu State. Mỗi draft có đúng một working State trong bộ nhớ backend, khóa theo `draft_session_id`, và một khóa ghi (mutex theo draft). Cùng lúc chỉ một trong các bên sau được ghi vào draft: một lượt AI, một thao tác ghi của UI/CLI (sửa tay, confirm/

- **REQ-CONC-001** `docs/business/be-core.md#L257-L359`: Per-draft single writer; no SQLite transaction/DB lock across network await; nonblocking consistent reads. (planning coverage `ASSIGNED`).

  Normative excerpt (verbatim baseline lines, truncated to 300 chars; use `sed` for the full range):

  > L257: ## §2. Concurrency Model (Mô hình Đồng thời & Quản lý Khóa)

  > L259: ### 2.1 Nguyên tắc Vàng: Không bao giờ giữ Transaction hoặc Khóa DB qua lệnh chờ mạng (`await`)

  > L261: > **[!IMPORTANT]**

  > L262: > **Bất biến bắt buộc (`MUST`):** Không một luồng thực thi nào trong Merlink Harness được phép mở giao dịch SQLite (`BEGIN`, `BEGIN IMMEDIATE`, `BEGIN EXCLUSIVE`) hoặc giữ khóa kết nối ghi SQLite (`db_write_lock`) trong khi đang thực hiện lệnh gọi mạng bất đồng bộ (`llm_provider.chat(...).await` hoặ

- **REQ-CMD-005** `docs/business/be-core.md#L2122-L2122`: MUST implement typed IPC #5 `get_draft_workspace_state` with the specified authorization, DTO, transaction and error behavior. (planning coverage `ASSIGNED`).

- **REQ-CMD-006** `docs/business/be-core.md#L2123-L2123`: MUST implement typed IPC #6 `get_progress_detail` with the specified authorization, DTO, transaction and error behavior. (planning coverage `ASSIGNED`).

- **REQ-CMD-007** `docs/business/be-core.md#L2124-L2124`: MUST implement typed IPC #7 `apply_manual_draft_edit` with the specified authorization, DTO, transaction and error behavior. (planning coverage `ASSIGNED`).

- **REQ-CMD-008** `docs/business/be-core.md#L2125-L2125`: MUST implement typed IPC #8 `confirm_phase` with the specified authorization, DTO, transaction and error behavior. (planning coverage `ASSIGNED`).

- **REQ-CMD-010** `docs/business/be-core.md#L2127-L2127`: MUST implement typed IPC #10 `list_draft_versions` with the specified authorization, DTO, transaction and error behavior. (planning coverage `ASSIGNED`).

- **REQ-CMD-011** `docs/business/be-core.md#L2128-L2128`: MUST implement typed IPC #11 `restore_draft_version` with the specified authorization, DTO, transaction and error behavior. (planning coverage `ASSIGNED`).

- **REQ-TX-001** `docs/business/be-core.md#L499-L499`: MUST own TX-01 in one BEGIN IMMEDIATE boundary, with documented rollback and independent effects. (planning coverage `ASSIGNED`).

- **REQ-TX-003** `docs/business/be-core.md#L501-L501`: MUST own TX-03 in one BEGIN IMMEDIATE boundary, with documented rollback and independent effects. (planning coverage `ASSIGNED`).

- **REQ-SPECTEST-T-CONC-01** `docs/business/be-core.md#L2530-L2530`: MUST implement executable BE acceptance vector T-CONC-01, preserving scenario and expected result; old v1-only recovery claims remain legacy-only under B+. (planning coverage `ASSIGNED`).

- **REQ-SPECTEST-T-IDEM-01** `docs/business/be-core.md#L2531-L2531`: MUST implement executable BE acceptance vector T-IDEM-01, preserving scenario and expected result; old v1-only recovery claims remain legacy-only under B+. (planning coverage `ASSIGNED`).

- **REQ-SPECTEST-T-REST-01** `docs/business/be-core.md#L2532-L2532`: MUST implement executable BE acceptance vector T-REST-01, preserving scenario and expected result; old v1-only recovery claims remain legacy-only under B+. (planning coverage `ASSIGNED`).

**Targeted source references (read only these ranges unless a test proves broader context is necessary):**

- `docs/business/core.md#L662-L732` — focus: One-version turns, restore and patch expiry; CLI: `sed -n "662,732p" docs/business/core.md` (or `rg -n "<symbol>" docs/business/core.md` for a changed anchor).

- `docs/business/be-core.md#L562-L679` — focus: Seal key and restore TX; CLI: `sed -n "562,679p" docs/business/be-core.md` (or `rg -n "<symbol>" docs/business/be-core.md` for a changed anchor).

- `docs/business/be-core.md#L272-L351` — focus: Per-draft guard/lock hierarchy; CLI: `sed -n "272,351p" docs/business/be-core.md` (or `rg -n "<symbol>" docs/business/be-core.md` for a changed anchor).

- `docs/architecture/decisions/sf06-owner-ratification.md#L81-L83` — focus: Restore and seal failure blocker; CLI: `sed -n "81,83p" docs/architecture/decisions/sf06-owner-ratification.md` (or `rg -n "<symbol>" docs/architecture/decisions/sf06-owner-ratification.md` for a changed anchor).

**Owned IPC command contracts (verbatim narrow BE §10.2 rows; proposed, not currently implemented):**

- IPC-05 `get_draft_workspace_state` — `docs/business/be-core.md#L2176`: **`get_draft_workspace_state`**  ·  `{ draftSessionId: Uuid }`  ·  `DraftWorkspaceStateDto { draftSessionId, status, phase, subphase, headVersionSeq, businessFlowData, draftOverviewUi: DraftOverviewCamelDto, progress: ProgressSummaryDto, pendingPatches: PendingPatchDto[], eventLog: EventLogEntryDto[], lockState: DraftLockStatusDto }`  ·  Xác minh `draftSessionId` tồn tại. Cho phép gọi ngay cả khi workflow/AI đang giữ `operation_lock` (đọc từ `Arc<WorkingState>` snapshot — §2.3).  ·  Read-only (đọc RAM snapshot + `pending_patches` / `draft_versions` từ ReadPool).  ·  `EntityNotFound`, `DatabaseError`  ·  Không chặn, không cần hủy.

- IPC-06 `get_progress_detail` — `docs/business/be-core.md#L2177`: **`get_progress_detail`**  ·  `{ draftSessionId: Uuid, systemLanguage: "vi" \ ·  "en" }`  ·  `ProgressDetailDto { progress: ProgressSummaryDto, attributes: AttributeEvaluationDetailDto[], renderedMarkdown: String, userConstraints: String[], conflicts: ConstraintConflictDto[] }`  ·  Tính toán trực tiếp trên 46 Master Attributes (`core.md` §3.1, `Q-C1`, `Q-C2`) và kết xuất Markdown toàn bộ `BusinessFlowData` cho `ProcessDetailDialog` (`fe-core.md` GAP-15).  ·  Read-only trên `Arc<WorkingState>`.  ·  `EntityNotFound`  ·  Không áp dụng.

- IPC-07 `apply_manual_draft_edit` — `docs/business/be-core.md#L2178`: **`apply_manual_draft_edit`**  ·  `{ draftSessionId: Uuid, clientEditId: String, toolCode: String, args: serde_json::Value }`  ·  `ManualEditResponseDto { toolOutput: serde_json::Value, sealedVersion?: DraftVersionSummaryDto, progress: ProgressSummaryDto }`  ·  Chỉ cho phép gọi các tool ghi thực thể (`add_*`, `set_*`, `delete_*`) với `CallerContext::UiAction` (tự động gán `source = user` hoặc `user_edited` theo §5.3).  ·  Lấy `operation_lock` (timeout 30s); áp dụng vào `WorkingState`; thực thi **`TX-01`** với `reason = 'manual_edit'`, `seal_key = "manual:{clientEditId}"` (`SF-01`).  ·  `DraftBusy`, `DraftNotEditable`, `ValidationFailed`, `VersionSealFailed`, `SealKeyPayloadMismatch`  ·  Thực thi cục bộ nguyên tử, không có bước chờ mạng.

- IPC-08 `confirm_phase` — `docs/business/be-core.md#L2179`: **`confirm_phase`**  ·  `{ draftSessionId: Uuid, targetPhase: i32, targetSubphase?: i32, preDraftSuggestion?: String }`  ·  `PhaseConfirmResponseDto { phase: i32, subphase?: i32, sealedVersion: DraftVersionSummaryDto }`  ·  Kiểm tra điều kiện gating (`isRequireComplete == true` trước khi mở Phase 1); kiểm tra `(targetPhase == 5 && targetSubphase in 1..=3) \ · \ ·  (targetPhase != 5 && targetSubphase.is_none())`; chạy kiểm tra bất biến của phase hiện tại (`core.md` §1.3, §3.3, §3.6).  ·  Lấy `operation_lock`; thực thi **`TX-01`** với `reason = 'phase_confirm'`, `seal_key = "phase_confirm:{base_seq}:{targetPhase}:{sub}"`.  ·  `DraftBusy`, `DraftNotEditable`, `ValidationFailed`, `VersionSealFailed`  ·  Thực thi cục bộ nguyên tử.

- IPC-10 `list_draft_versions` — `docs/business/be-core.md#L2186`: **`list_draft_versions`**  ·  `{ draftSessionId: Uuid }`  ·  `DraftVersionListDto { headVersionSeq?: i32, versions: DraftVersionSummaryDto[] }`  ·  Xác minh `draftSessionId` tồn tại. Trả danh sách gồm `seq`, `displayLabel` (`"v.01"`..), `phase`, `subphase`, `reason`, `parentSeq`, `createdAt`.  ·  Read-only (`SELECT` trên `draft_versions ORDER BY seq DESC`).  ·  `EntityNotFound`, `DatabaseError`  ·  Không áp dụng.

- IPC-11 `restore_draft_version` — `docs/business/be-core.md#L2187`: **`restore_draft_version`**  ·  `{ draftSessionId: Uuid, targetSeq: i32, clientRequestId: String }`  ·  `RestoreVersionResponseDto { newVersion: DraftVersionSummaryDto, restoredPhase: i32, restoredSubphase?: i32, expiredPatchIds: Uuid[], workingState: DraftWorkspaceStateDto }`  ·  `draft_sessions.status == 'editing'`; `targetSeq` phải tồn tại trong `draft_versions` của `draftSessionId`.  ·  Lấy `operation_lock` (timeout 30s); thực thi **`TX-03`**: tạo dòng `draft_versions` mới (`seq = head + 1`, `reason = 'restore'`, `parent_seq = targetSeq`, `seal_key = "restore:{clientRequestId}:{targetSeq}"`), khôi phục nguyên tử cả `BusinessFlowData`, `phase`, `subphase`, `_counters`, và chuyển mọi `pending_patches` đang `pending` sang `expired`.  ·  `DraftBusy`, `DraftNotEditable`, `EntityNotFound`, `VersionSealFailed`, `SealKeyPayloadMismatch`  ·  Thực thi cục bộ nguyên tử; idempotent theo `clientRequestId`.

## 5. Prerequisites

`depends_on: ["phase2_sprint5_deep2", "phase2_sprint6_deep2", "phase1_sprint1_deep0"]`.

- `phase2_sprint5_deep2` [TYPE_DEPENDENCY] produces Frozen DTO/error/adapter Rust API plus generated TS binding path and schema fixture; inspect its accepted commit, exported artifact/test and `gate_phase2_sprint5_deep2`. If absent, do not start.

- `phase2_sprint6_deep2` [INTEGRATION_DEPENDENCY] produces Frozen DraftRepo/CatalogRepo/WorkflowReadRepo interfaces and temp-store API; inspect its accepted commit, exported artifact/test and `gate_phase2_sprint6_deep2`. If absent, do not start.

- `phase1_sprint1_deep0` [INTEGRATION_DEPENDENCY] produces docs/plan/evidence/contract-ledger.md with issue IDs, accepted SSOT SHA or CONTRACT_BLOCKED and affected sprint IDs; inspect its accepted commit, exported artifact/test and `gate_phase1_sprint1_deep0`. If absent, do not start.

## 6. Inputs

Required: accepted outputs of all predecessors; frozen committed SSOT ranges above; actual source paths listed below. Optional context: existing UI mocks for fixtures only. No `report/` input; no historical research.

## 7. File ownership

```yaml

allowed_write_paths:

  - src-tauri/src/service/draft_service.rs

  - src-tauri/src/service/draft_runtime.rs

  - src-tauri/src/ipc/draft_commands.rs

  - src-tauri/tests/draft_version_tx.rs

forbidden_write_paths:

  - docs/business/**

  - docs/db.dbml

  - docs/architecture/decisions/**

  - app-data/**

  - report/**

shared_read_only_paths:

  - docs/business/core.md

  - docs/business/be-core.md

  - docs/architecture/decisions/sf06-owner-ratification.md

generated_paths: []

```

Any allowed path may be absent at baseline (not only `src-tauri/`; new FE adapter/shim files are also future targets). Verify with `test -e`/`git ls-files` before editing; existing files are read first. Any extra path requires central integration approval, not a silent scope expansion.

## 8. Public interfaces

PROPOSED_INTERFACE: DraftService::{seal_version,restore_version,get_state}; guarded DraftRuntimeEntry; draft/version IPC #5–8,#10–11. Every consumed prerequisite interface is `PREREQUISITE_INTERFACE`, not an existing runtime guarantee. If a named signature conflicts with committed SSOT, stop `CONTRACT_BLOCKED`.

## 9. Implementation requirements

- Implement draft runtime/repo use and commands #5–8,#10–11; no shared DTO/mod edits.

- Test lost-response, identical/different seal-key payload, restore branch/phase/counters, transient seal failure.

- Prefer official compatible CLI/tooling over manual generated files; verify installed version and diff. No external LLM/Jev request without authorization; optional Jev advice never overrides SSOT.

## 10. Domain invariants

- TX01 seal and TX03 append-only Restore require P1S1 decisions on seal_pending_retry, pending expiry, restore identical payload, nullable legacy phase.

- Single operation_lock held by orchestrator; no nested lock, write TX after awaits only; head CAS under BEGIN IMMEDIATE.

- No partial state escapes; protected provenance, explicit authorization, no external-effect recovery replay where applicable.

## 11. Failure matrix

| Trigger | Expected behavior / classification | Durable effects and recovery | Required test |

|---|---|---|---|

| Missing P1S1 resolution | CONTRACT_BLOCKED, do not add enum state; P3S2-N. | No unauthorized partial commit; re-read actual durable state before retry | phase3_sprint2_deep0-F1 |

| Retry mismatch | SealKeyPayloadMismatch and 0 new versions; P3S2-C. | No unauthorized partial commit; re-read actual durable state before retry | phase3_sprint2_deep0-F2 |

| Missing accepted predecessor / baseline mismatch | `CONTRACT_BLOCKED`; do not run implementation | No user-data effects; record issue and owner in sprint evidence | phase3_sprint2_deep0-F0 |

## 12. Acceptance tests

Tests below name **future targets**, not tests executed during planning. Each row must become an executable test before the sprint can be accepted.

| Test ID | Initial state | Input / action | Expected output and state | Forbidden side effects / invariant |

|---|---|---|---|---|

| P3S2-P | disposable uniquely-named SQLite S0 + accepted predecessor fixture | In the owned test target, set up the fixture and run the scenario: one applied chat turn yields one version; identical retry returns original. | one applied chat turn yields one version; identical retry returns original. | No unrelated write; preserve exact expected state and listed invariant |

| P3S2-N | disposable uniquely-named SQLite S0 + accepted predecessor fixture | In the owned test target, set up the fixture and run the scenario: two failed seal attempts | two failed seal attempts => VersionSealFailed without success claim. | No unrelated write; preserve exact expected state and listed invariant |

| P3S2-B | disposable uniquely-named SQLite S0 + accepted predecessor fixture | In the owned test target, set up the fixture and run the scenario: Restore V2 from V4 appends V5 and expires all pending; no fake seq0. | Restore V2 from V4 appends V5 and expires all pending; no fake seq0. | No unrelated write; preserve exact expected state and listed invariant |

| phase3_sprint2_deep0-F1 | accepted prerequisite fixture / isolated worktree | Inject Missing P1S1 resolution into owned operation | CONTRACT_BLOCKED, do not add enum state; P3S2-N. | No unauthorized commit or external replay |

| phase3_sprint2_deep0-F2 | accepted prerequisite fixture / isolated worktree | Inject Retry mismatch into owned operation | SealKeyPayloadMismatch and 0 new versions; P3S2-C. | No unauthorized commit or external replay |

| IPC-CMD-005 | isolated authorized and unauthorized caller fixtures | Invoke `get_draft_workspace_state` with valid request and invalid/stale variant; use frozen DTO | /  **`get_draft_workspace_state`**  /  `{ draftSessionId: Uuid }`  /  `DraftWorkspaceStateDto { draftSessionId, status, phase, subphase, headVersionSeq, businessFlowData, draftOverviewUi: DraftOverviewCamelDto, progress: ProgressSummaryDto, pendingPatches: PendingPatchDto[], eventLog: EventLogEntryDto[], lockState: DraftLockStatusDto }`  /  Xác minh `draftSessionId` tồn tại. Cho phép gọi ngay cả khi workflow/AI đang giữ `operation_lock` (đọc từ `Arc<WorkingState>` snapshot — §2.3).  /  Read-only (đọc RAM snapshot + `pending_patches` / `draft_versions` từ ReadPool).  /  `EntityNotFound`, `DatabaseError`  /  Không chặn, không cần hủy.  / | No unlisted command, no unauthorized write; assert event/error/transaction result |

| IPC-CMD-006 | isolated authorized and unauthorized caller fixtures | Invoke `get_progress_detail` with valid request and invalid/stale variant; use frozen DTO | /  **`get_progress_detail`**  /  `{ draftSessionId: Uuid, systemLanguage: "vi" \ /  "en" }`  /  `ProgressDetailDto { progress: ProgressSummaryDto, attributes: AttributeEvaluationDetailDto[], renderedMarkdown: String, userConstraints: String[], conflicts: ConstraintConflictDto[] }`  /  Tính toán trực tiếp trên 46 Master Attributes (`core.md` §3.1, `Q-C1`, `Q-C2`) và kết xuất Markdown toàn bộ `BusinessFlowData` cho `ProcessDetailDialog` (`fe-core.md` GAP-15).  /  Read-only trên `Arc<WorkingState>`.  /  `EntityNotFound`  /  Không áp dụng.  / | No unlisted command, no unauthorized write; assert event/error/transaction result |

| IPC-CMD-007 | isolated authorized and unauthorized caller fixtures | Invoke `apply_manual_draft_edit` with valid request and invalid/stale variant; use frozen DTO | /  **`apply_manual_draft_edit`**  /  `{ draftSessionId: Uuid, clientEditId: String, toolCode: String, args: serde_json::Value }`  /  `ManualEditResponseDto { toolOutput: serde_json::Value, sealedVersion?: DraftVersionSummaryDto, progress: ProgressSummaryDto }`  /  Chỉ cho phép gọi các tool ghi thực thể (`add_*`, `set_*`, `delete_*`) với `CallerContext::UiAction` (tự động gán `source = user` hoặc `user_edited` theo §5.3).  /  Lấy `operation_lock` (timeout 30s); áp dụng vào `WorkingState`; thực thi **`TX-01`** với `reason = 'manual_edit'`, `seal_key = "manual:{clientEditId}"` (`SF-01`).  /  `DraftBusy`, `DraftNotEditable`, `ValidationFailed`, `VersionSealFailed`, `SealKeyPayloadMismatch`  /  Thực thi cục bộ nguyên tử, không có bước chờ mạng.  / | No unlisted command, no unauthorized write; assert event/error/transaction result |

| IPC-CMD-008 | isolated authorized and unauthorized caller fixtures | Invoke `confirm_phase` with valid request and invalid/stale variant; use frozen DTO | /  **`confirm_phase`**  /  `{ draftSessionId: Uuid, targetPhase: i32, targetSubphase?: i32, preDraftSuggestion?: String }`  /  `PhaseConfirmResponseDto { phase: i32, subphase?: i32, sealedVersion: DraftVersionSummaryDto }`  /  Kiểm tra điều kiện gating (`isRequireComplete == true` trước khi mở Phase 1); kiểm tra `(targetPhase == 5 && targetSubphase in 1..=3) \ / \ /  (targetPhase != 5 && targetSubphase.is_none())`; chạy kiểm tra bất biến của phase hiện tại (`core.md` §1.3, §3.3, §3.6).  /  Lấy `operation_lock`; thực thi **`TX-01`** với `reason = 'phase_confirm'`, `seal_key = "phase_confirm:{base_seq}:{targetPhase}:{sub}"`.  /  `DraftBusy`, `DraftNotEditable`, `ValidationFailed`, `VersionSealFailed`  /  Thực thi cục bộ nguyên tử.  / | No unlisted command, no unauthorized write; assert event/error/transaction result |

| IPC-CMD-010 | isolated authorized and unauthorized caller fixtures | Invoke `list_draft_versions` with valid request and invalid/stale variant; use frozen DTO | /  **`list_draft_versions`**  /  `{ draftSessionId: Uuid }`  /  `DraftVersionListDto { headVersionSeq?: i32, versions: DraftVersionSummaryDto[] }`  /  Xác minh `draftSessionId` tồn tại. Trả danh sách gồm `seq`, `displayLabel` (`"v.01"`..), `phase`, `subphase`, `reason`, `parentSeq`, `createdAt`.  /  Read-only (`SELECT` trên `draft_versions ORDER BY seq DESC`).  /  `EntityNotFound`, `DatabaseError`  /  Không áp dụng.  / | No unlisted command, no unauthorized write; assert event/error/transaction result |

| IPC-CMD-011 | isolated authorized and unauthorized caller fixtures | Invoke `restore_draft_version` with valid request and invalid/stale variant; use frozen DTO | /  **`restore_draft_version`**  /  `{ draftSessionId: Uuid, targetSeq: i32, clientRequestId: String }`  /  `RestoreVersionResponseDto { newVersion: DraftVersionSummaryDto, restoredPhase: i32, restoredSubphase?: i32, expiredPatchIds: Uuid[], workingState: DraftWorkspaceStateDto }`  /  `draft_sessions.status == 'editing'`; `targetSeq` phải tồn tại trong `draft_versions` của `draftSessionId`.  /  Lấy `operation_lock` (timeout 30s); thực thi **`TX-03`**: tạo dòng `draft_versions` mới (`seq = head + 1`, `reason = 'restore'`, `parent_seq = targetSeq`, `seal_key = "restore:{clientRequestId}:{targetSeq}"`), khôi phục nguyên tử cả `BusinessFlowData`, `phase`, `subphase`, `_counters`, và chuyển mọi `pending_patches` đang `pending` sang `expired`.  /  `DraftBusy`, `DraftNotEditable`, `EntityNotFound`, `VersionSealFailed`, `SealKeyPayloadMismatch`  /  Thực thi cục bộ nguyên tử; idempotent theo `clientRequestId`.  / | No unlisted command, no unauthorized write; assert event/error/transaction result |

| TX-001 | disposable S0 + failure injection at every write step | Execute TX-01 then inject error/lose response | /  **TX-01**  /  **Seal Draft Version** (`seal_version`)  /  Đọc/Ghi `draft_sessions`, `draft_versions`, `pending_patches` (cập nhật `expired` nếu có).  /  Rollback toàn bộ; thử lại tối đa 1 lần; nếu vẫn lỗi trả `VersionSealFailed` (`core.md` §3.6).  / | Atomic rollback, no unrelated durable effect; retry checks receipt |

| TX-003 | disposable S0 + failure injection at every write step | Execute TX-03 then inject error/lose response | /  **TX-03**  /  **Restore Draft Version** (`restore_draft_version`)  /  Đọc snapshot `draft_versions(seq)`; Ghi dòng mới `draft_versions(head_seq + 1, reason = 'restore')`; Cập nhật `draft_sessions.head_version_seq`; Chuyển mọi `pending_patches` đang `pending` sang `expired`.  /  Rollback toàn bộ; giữ nguyên `head_version_seq` và các patch hiện có.  / | Atomic rollback, no unrelated durable effect; retry checks receipt |

| T-CONC-01 | disposable/synthetic fixture described by BE | Trong khi một vòng workflow của $D_1$ đang chạy, gửi đồng thời: (a) 10 lệnh đọc `get_draft_workspace_state(D_1)`, (b) 1 lệnh ghi trên bản nháp khác $D_2$, và (c) 1 lệnh sửa tay trên $D_1$ với timeout ngắn. | (a) Cả 10 lệnh đọc trên $D_1$ trả về tức thời ($<20\text{ ms}$) từ snapshot mà không bị chặn; (b) lệnh ghi trên $D_2$ hoàn tất song song không bị $D_1$ chặn; (c) lệnh ghi trùng trên $D_1$ xếp hàng chờ hoặc trả `DraftBusy` khi hết timeout, tuyệt đối không xảy ra race condition hay lỗi `SQLITE_BUSY`. | Only owner-authorized effects; old v1-only statements interpreted as legacy, no B+ regression |

| T-IDEM-01 | disposable/synthetic fixture described by BE | Gọi `seal_draft_version_tx` 2 lần với cùng `(draft_session_id, seal_key)` và cùng `data_hash`; sau đó gọi lần 3 với cùng `seal_key` nhưng khác `data_hash`. | Lần 1 tạo `v.01`. Lần 2 trả lại đúng `v.01` và `COUNT(draft_versions)` vẫn bằng `1`. Lần 3 bị từ chối với lỗi `SealKeyPayloadMismatch`. | Only owner-authorized effects; old v1-only statements interpreted as legacy, no B+ regression |

| T-REST-01 | disposable/synthetic fixture described by BE | Tạo `v.01..v.04`, tạo 2 `pending_patches` ở `v.04`. Gọi `restore_draft_version(targetSeq = 2)`. Sau đó tạo thêm một Arc mới. | Tạo ra `v.05` (`reason = 'restore'`, `parent_seq = 2`, giữ nguyên `v.01..v.04`). `phase`, `subphase` và `_counters` quay về đúng giá trị của `v.02` (không đụng độ `code` khi tạo Arc mới). Cả 2 patch `pending` đều chuyển sang `status = 'expired'`. | Only owner-authorized effects; old v1-only statements interpreted as legacy, no B+ regression |

| phase3_sprint2_deep0-F0 | predecessor gate absent or baseline SHA drift | Attempt dispatch before predecessor accepted | CONTRACT_BLOCKED; no implementation started | No source/DB modification |

Backend module wiring rule: this sprint must not edit any `mod.rs`/`lib.rs`. Its integration test loads each owned implementation file via `#[path = "..."] mod <name>;` and fixture modules via the same mechanism. Production registration occurs only in P6S2 after this sprint is accepted. A compile success is therefore scoped to this test target, not a claim that the whole crate is wired.

Future command after the target files exist: `cargo test --manifest-path src-tauri/Cargo.toml --test draft_version_tx  # future target, after bootstrap`.

## 13. Database and migration safety

Use `BEGIN IMMEDIATE` only for the assigned TX; WAL/FULL/FK ON/busy_timeout as specified. Use unique disposable DB/temp root per worktree. Never open a user application DB; no destructive migration without separate authorization. Backup/rollback and PRAGMA user_version are owned by P2S3. TX04 never rolls back committed TX08.

## 14. Concurrency contract

- Peer `phase3_sprint1_deep0`: candidate parallel only after frozen contracts/gates; NONE: Disjoint exact write paths; shared types frozen by predecessors; worktree-local test resources.

- Peer `phase3_sprint3_deep0`: candidate parallel only after frozen contracts/gates; NONE: Disjoint exact write paths; shared types frozen by predecessors; worktree-local test resources.

Isolation: own worktree/branch from baseline or accepted integration SHA; unique temp root, test DB and port; no shared mutable process/global fixture. Same depth alone is NOT permission to dispatch.

## 15. Deliverables

- [ ] DraftService, DraftRuntimeRegistry guard, draft/version IPC #5–8,#10–11, TX01/03 fixtures

- [ ] Tests above and evidence including command exit codes, scope diff and accepted commit SHA.

## 16. Completion criteria

`SPRINT_IMPLEMENTED` requires artifacts, `SPRINT_TESTED` requires executed acceptance tests, `SPRINT_INTEGRATED` requires merged checks, and **`SPRINT_ACCEPTED` requires `gate_phase3_sprint2_deep0` accepted**. A contract-evidence sprint cannot be ACCEPTED for an affected implementation successor while its owner decision is unresolved. No agent declaration alone unlocks successors. All required assertions, file scope and baseline must pass.

## 17. Handoff contract

- DraftService, DraftRuntimeRegistry guard, draft/version IPC #5–8,#10–11, TX01/03 fixtures

- Exact artifacts: src-tauri/src/service/draft_service.rs, src-tauri/src/service/draft_runtime.rs, src-tauri/src/ipc/draft_commands.rs, src-tauri/tests/draft_version_tx.rs; accepted integration SHA and `gate_phase3_sprint2_deep0` record. A successor must verify exports/ABI rather than infer from prose.

## 18. Resume and failure behavior

Not started: verify baseline and deps. Partial/test failure: inspect git status, changed paths and durable test DB, resume only safe incomplete steps. Implemented but unintegrated: do not dispatch successors. Interrupted agent/rejected patch: preserve branch, rebase only after review, rerun tests. Specification conflict: `CONTRACT_BLOCKED`, record evidence, request authority resolution; never edit SSOT in this sprint.

**Explicit unblock requirements:** SEAL_RESTORE_ERROR_CONTRACT, CI-03, CONTRACT_REVIEW_REQUIRED:REQ-DRAFT-001.


---

Subagent budget: 256K tokens maximum; start with the bounded references above, do not read entire SSOT by default. CLI-first with verified tool compatibility. Treat Jev/TypeSafe as optional advisory only when available and authorized; never send private repository content externally without consent.
