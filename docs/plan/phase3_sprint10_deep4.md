---
sprint_id: phase3_sprint10_deep4
title: Seal B+ round atomically in TX-04
phase: 3
sprint: 10
deep: 4
baseline_commit: 57a462efb953a665e3673b15e6cc33622a4f2d39
status: PLANNED
---

# phase3_sprint10_deep4 — Seal B+ round atomically in TX-04

## 1. Sprint identity

Engineering phase 3 (Draft domain and durable workflow), within-phase depth 4; baseline is the Git commit above, not an uncommitted workspace. **PROPOSED, NOT IMPLEMENTED.**

## 2. Objective

Seal B+ round atomically in TX-04. Observable handoff: seal_or_retry_round and workflow seal repository, TX04 test fixture.

## 3. Scope boundaries

**IN_SCOPE:** Implement precommit exact membership, authenticated effects, fingerprint chain, collision/retry checks and one BEGIN IMMEDIATE.; Use fake provider outcomes and disposable SQLite; do not replay external effects.

**OUT_OF_SCOPE / MUST NOT:** modify authoritative `docs/business/**`, `docs/db.dbml`, `docs/architecture/decisions/**`; use real application data; run production migration; add client request IDs to SF-06; invent business semantics; rewrite another sprint’s files.

## 4. Authoritative contract snapshot

- Historical identity/key lookup before head OCC; only genuinely new round requires sealed baseline equality.

- TX04 commits header+N evidence+optional one version/head; pending rows excluded; zero-net no version.

- **REQ-SF06-003** `docs/business/core.md#L966-L987`: TX-04 header/receipts/evidence/optional version/head; zero-net header but no fake version. (planning coverage `ASSIGNED`).

  Normative excerpt (verbatim baseline lines, truncated to 300 chars; use `sed` for the full range):

  > L966: * **Tính nguyên tử, Biên nhận bất biến (Insert-Once) và Kiểm tra trước khi tạo Version (`Q3.2`, `Q3.3` — APPROVED v2.4.5):**

  > L967: * *Bất biến bắt buộc:* **Một workflow task không bao giờ được coi là đã hoàn tất bền vững nếu các thay đổi Business State bắt buộc của nó không thể khôi phục được từ Sealed Draft Version.**

  > L968: * *Kiểm tra biên nhận trước khi tạo Draft Version (Retry Before Version Creation):* Khi thực thi hoặc retry một workflow task/round trong `BEGIN IMMEDIATE`, backend bắt buộc truy vấn biên nhận hiện có trong `workflow_task_receipts` theo `(draft_session_id, workflow_run_id, plan_generation, task_iden

  > L969: * *Biên nhận bất biến (Insert-Once Semantics):* Bảng `workflow_task_receipts` chỉ dùng cơ chế chèn một lần (`INSERT ... ON CONFLICT DO NOTHING` kèm truy vấn xác minh), **tuyệt đối cấm dùng `INSERT OR REPLACE`**.

- **REQ-TX-004** `docs/business/be-core.md#L502-L502`: MUST own TX-04 in one BEGIN IMMEDIATE boundary, with documented rollback and independent effects. (planning coverage `ASSIGNED`).

- **REQ-SPECTEST-T-WF-01** `docs/business/be-core.md#L2535-L2535`: MUST implement executable BE acceptance vector T-WF-01, preserving scenario and expected result; old v1-only recovery claims remain legacy-only under B+. (planning coverage `ASSIGNED`).

- **REQ-SPECTEST-WF-01** `docs/business/be-core.md#L2552-L2552`: MUST implement executable BE acceptance vector WF-01, preserving scenario and expected result; old v1-only recovery claims remain legacy-only under B+. (planning coverage `ASSIGNED`).

- **REQ-SPECTEST-WF-02** `docs/business/be-core.md#L2553-L2553`: MUST implement executable BE acceptance vector WF-02, preserving scenario and expected result; old v1-only recovery claims remain legacy-only under B+. (planning coverage `ASSIGNED`).

- **REQ-SPECTEST-WF-03** `docs/business/be-core.md#L2554-L2554`: MUST implement executable BE acceptance vector WF-03, preserving scenario and expected result; old v1-only recovery claims remain legacy-only under B+. (planning coverage `ASSIGNED`).

- **REQ-SPECTEST-WF-04** `docs/business/be-core.md#L2555-L2555`: MUST implement executable BE acceptance vector WF-04, preserving scenario and expected result; old v1-only recovery claims remain legacy-only under B+. (planning coverage `ASSIGNED`).

- **REQ-SPECTEST-WF-05** `docs/business/be-core.md#L2556-L2556`: MUST implement executable BE acceptance vector WF-05, preserving scenario and expected result; old v1-only recovery claims remain legacy-only under B+. (planning coverage `ASSIGNED`).

- **REQ-SPECTEST-WF-06** `docs/business/be-core.md#L2557-L2557`: MUST implement executable BE acceptance vector WF-06, preserving scenario and expected result; old v1-only recovery claims remain legacy-only under B+. (planning coverage `ASSIGNED`).

- **REQ-SPECTEST-WF-07** `docs/business/be-core.md#L2558-L2558`: MUST implement executable BE acceptance vector WF-07, preserving scenario and expected result; old v1-only recovery claims remain legacy-only under B+. (planning coverage `ASSIGNED`).

- **REQ-SPECTEST-WF-08** `docs/business/be-core.md#L2559-L2559`: MUST implement executable BE acceptance vector WF-08, preserving scenario and expected result; old v1-only recovery claims remain legacy-only under B+. (planning coverage `ASSIGNED`).

- **REQ-SPECTEST-WF-09** `docs/business/be-core.md#L2560-L2560`: MUST implement executable BE acceptance vector WF-09, preserving scenario and expected result; old v1-only recovery claims remain legacy-only under B+. (planning coverage `ASSIGNED`).

- **REQ-SPECTEST-WF-10** `docs/business/be-core.md#L2561-L2561`: MUST implement executable BE acceptance vector WF-10, preserving scenario and expected result; old v1-only recovery claims remain legacy-only under B+. (planning coverage `ASSIGNED`).

- **REQ-SPECTEST-WF-C01** `docs/business/be-core.md#L2571-L2571`: MUST implement executable BE acceptance vector WF-C01, preserving scenario and expected result; old v1-only recovery claims remain legacy-only under B+. (planning coverage `ASSIGNED`).

- **REQ-SPECTEST-WF-C02** `docs/business/be-core.md#L2572-L2572`: MUST implement executable BE acceptance vector WF-C02, preserving scenario and expected result; old v1-only recovery claims remain legacy-only under B+. (planning coverage `ASSIGNED`).

- **REQ-SPECTEST-WF-C03** `docs/business/be-core.md#L2573-L2573`: MUST implement executable BE acceptance vector WF-C03, preserving scenario and expected result; old v1-only recovery claims remain legacy-only under B+. (planning coverage `ASSIGNED`).

- **REQ-SPECTEST-WF-C04** `docs/business/be-core.md#L2574-L2574`: MUST implement executable BE acceptance vector WF-C04, preserving scenario and expected result; old v1-only recovery claims remain legacy-only under B+. (planning coverage `ASSIGNED`).

- **REQ-SPECTEST-WF-C05** `docs/business/be-core.md#L2575-L2575`: MUST implement executable BE acceptance vector WF-C05, preserving scenario and expected result; old v1-only recovery claims remain legacy-only under B+. (planning coverage `ASSIGNED`).

- **REQ-SPECTEST-WF-C06** `docs/business/be-core.md#L2576-L2576`: MUST implement executable BE acceptance vector WF-C06, preserving scenario and expected result; old v1-only recovery claims remain legacy-only under B+. (planning coverage `ASSIGNED`).

- **REQ-SPECTEST-WF-C07** `docs/business/be-core.md#L2577-L2577`: MUST implement executable BE acceptance vector WF-C07, preserving scenario and expected result; old v1-only recovery claims remain legacy-only under B+. (planning coverage `ASSIGNED`).

- **REQ-SPECTEST-WF-C08** `docs/business/be-core.md#L2578-L2578`: MUST implement executable BE acceptance vector WF-C08, preserving scenario and expected result; old v1-only recovery claims remain legacy-only under B+. (planning coverage `ASSIGNED`).

- **REQ-SPECTEST-WF-C09** `docs/business/be-core.md#L2579-L2579`: MUST implement executable BE acceptance vector WF-C09, preserving scenario and expected result; old v1-only recovery claims remain legacy-only under B+. (planning coverage `ASSIGNED`).

- **REQ-SPECTEST-WF-C10** `docs/business/be-core.md#L2580-L2580`: MUST implement executable BE acceptance vector WF-C10, preserving scenario and expected result; old v1-only recovery claims remain legacy-only under B+. (planning coverage `ASSIGNED`).

- **REQ-SPECTEST-WF-C11** `docs/business/be-core.md#L2581-L2581`: MUST implement executable BE acceptance vector WF-C11, preserving scenario and expected result; old v1-only recovery claims remain legacy-only under B+. (planning coverage `ASSIGNED`).

- **REQ-SPECTEST-WF-C12** `docs/business/be-core.md#L2582-L2582`: MUST implement executable BE acceptance vector WF-C12, preserving scenario and expected result; old v1-only recovery claims remain legacy-only under B+. (planning coverage `ASSIGNED`).

- **REQ-SPECTEST-WF-C13** `docs/business/be-core.md#L2583-L2583`: MUST implement executable BE acceptance vector WF-C13, preserving scenario and expected result; old v1-only recovery claims remain legacy-only under B+. (planning coverage `ASSIGNED`).

- **REQ-SPECTEST-WF-C14** `docs/business/be-core.md#L2584-L2584`: MUST implement executable BE acceptance vector WF-C14, preserving scenario and expected result; old v1-only recovery claims remain legacy-only under B+. (planning coverage `ASSIGNED`).

- **REQ-SPECTEST-WF-C15** `docs/business/be-core.md#L2585-L2585`: MUST implement executable BE acceptance vector WF-C15, preserving scenario and expected result; old v1-only recovery claims remain legacy-only under B+. (planning coverage `ASSIGNED`).

**Targeted source references (read only these ranges unless a test proves broader context is necessary):**

- `docs/business/be-core.md#L493-L506` — focus: TX04 vs TX08; CLI: `sed -n "493,506p" docs/business/be-core.md` (or `rg -n "<symbol>" docs/business/be-core.md` for a changed anchor).

- `docs/business/be-core.md#L1512-L1538` — focus: Identity/digest/lookup before OCC; CLI: `sed -n "1512,1538p" docs/business/be-core.md` (or `rg -n "<symbol>" docs/business/be-core.md` for a changed anchor).

- `docs/db.dbml#L1299-L1355` — focus: Parent/child schema; CLI: `sed -n "1299,1355p" docs/db.dbml` (or `rg -n "<symbol>" docs/db.dbml` for a changed anchor).

- `docs/business/be-core.md#L2552-L2585` — focus: WF/WF-C acceptance cases; CLI: `sed -n "2552,2585p" docs/business/be-core.md` (or `rg -n "<symbol>" docs/business/be-core.md` for a changed anchor).

## 5. Prerequisites

`depends_on: ["phase3_sprint6_deep3", "phase3_sprint9_deep2", "phase3_sprint2_deep0", "phase3_sprint3_deep0"]`.

- `phase3_sprint6_deep3` [RUNTIME_DEPENDENCY] produces PatchService + pending repo, TX02/TX08 tests and IPC #12–15; inspect its accepted commit, exported artifact/test and `gate_phase3_sprint6_deep3`. If absent, do not start.

- `phase3_sprint9_deep2` [RUNTIME_DEPENDENCY] produces Read-only verify_historical_round API and classifier fixtures; inspect its accepted commit, exported artifact/test and `gate_phase3_sprint9_deep2`. If absent, do not start.

- `phase3_sprint2_deep0` [RUNTIME_DEPENDENCY] produces DraftService, DraftRuntimeRegistry guard, draft/version IPC #5–8,#10–11, TX01/03 fixtures; inspect its accepted commit, exported artifact/test and `gate_phase3_sprint2_deep0`. If absent, do not start.

- `phase3_sprint3_deep0` [DATA_CONTRACT_DEPENDENCY] produces Versioned WorldProbeReportV1/WorldWriteInputV1 and executable fixture proof; inspect its accepted commit, exported artifact/test and `gate_phase3_sprint3_deep0`. If absent, do not start.

## 6. Inputs

Required: accepted outputs of all predecessors; frozen committed SSOT ranges above; actual source paths listed below. Optional context: existing UI mocks for fixtures only. No `report/` input; no historical research.

## 7. File ownership

```yaml

allowed_write_paths:

  - src-tauri/src/storage/workflow_seal_repo.rs

  - src-tauri/src/service/round_seal_service.rs

  - src-tauri/tests/sf06_seal_tx.rs

forbidden_write_paths:

  - docs/business/**

  - docs/db.dbml

  - docs/architecture/decisions/**

  - app-data/**

  - report/**

shared_read_only_paths:

  - docs/business/be-core.md

  - docs/db.dbml

generated_paths: []

```

Any allowed path may be absent at baseline (not only `src-tauri/`; new FE adapter/shim files are also future targets). Verify with `test -e`/`git ls-files` before editing; existing files are read first. Any extra path requires central integration approval, not a silent scope expansion.

## 8. Public interfaces

PROPOSED_INTERFACE: seal_or_retry_round(guard,tx,authorized_plan,effects)->Historical|Sealed|Conflict|StaleHead|RestoreDiverged. Every consumed prerequisite interface is `PREREQUISITE_INTERFACE`, not an existing runtime guarantee. If a named signature conflicts with committed SSOT, stop `CONTRACT_BLOCKED`.

## 9. Implementation requirements

- Implement precommit exact membership, authenticated effects, fingerprint chain, collision/retry checks and one BEGIN IMMEDIATE.

- Use fake provider outcomes and disposable SQLite; do not replay external effects.

- Prefer official compatible CLI/tooling over manual generated files; verify installed version and diff. No external LLM/Jev request without authorization; optional Jev advice never overrides SSOT.

## 10. Domain invariants

- Historical identity/key lookup before head OCC; only genuinely new round requires sealed baseline equality.

- TX04 commits header+N evidence+optional one version/head; pending rows excluded; zero-net no version.

- No partial state escapes; protected provenance, explicit authorization, no external-effect recovery replay where applicable.

## 11. Failure matrix

| Trigger | Expected behavior / classification | Durable effects and recovery | Required test |

|---|---|---|---|

| Lost reply after commit | history summary without second write; P3S10-C. | No unauthorized partial commit; re-read actual durable state before retry | phase3_sprint10_deep4-F1 |

| Failed child insertion | rollback entire TX04, keep prior TX08 proposal; P3S10-N. | No unauthorized partial commit; re-read actual durable state before retry | phase3_sprint10_deep4-F2 |

| Missing accepted predecessor / baseline mismatch | `CONTRACT_BLOCKED`; do not run implementation | No user-data effects; record issue and owner in sprint evidence | phase3_sprint10_deep4-F0 |

## 12. Acceptance tests

Tests below name **future targets**, not tests executed during planning. Each row must become an executable test before the sprint can be accepted.

| Test ID | Initial state | Input / action | Expected output and state | Forbidden side effects / invariant |

|---|---|---|---|---|

| P3S10-P | disposable uniquely-named SQLite S0 + accepted predecessor fixture | In the owned test target, set up the fixture and run the scenario: N children indices 0..N−1, one header, optional one version. | N children indices 0..N−1, one header, optional one version. | No unrelated write; preserve exact expected state and listed invariant |

| P3S10-N | disposable uniquely-named SQLite S0 + accepted predecessor fixture | In the owned test target, set up the fixture and run the scenario: corrupted/missing artifact rejects seal, no version/header. | corrupted/missing artifact rejects seal, no version/header. | No unrelated write; preserve exact expected state and listed invariant |

| P3S10-B | disposable uniquely-named SQLite S0 + accepted predecessor fixture | In the owned test target, set up the fixture and run the scenario: zero-net persists header/receipts only; retry discoverable. | zero-net persists header/receipts only; retry discoverable. | No unrelated write; preserve exact expected state and listed invariant |

| phase3_sprint10_deep4-F1 | accepted prerequisite fixture / isolated worktree | Inject Lost reply after commit into owned operation | history summary without second write; P3S10-C. | No unauthorized commit or external replay |

| phase3_sprint10_deep4-F2 | accepted prerequisite fixture / isolated worktree | Inject Failed child insertion into owned operation | rollback entire TX04, keep prior TX08 proposal; P3S10-N. | No unauthorized commit or external replay |

| TX-004 | disposable S0 + failure injection at every write step | Execute TX-04 then inject error/lose response | /  **TX-04**  /  **Workflow Round Seal & Receipt** (`seal_workflow_round_tx`)  /  Chèn `workflow_round_receipts` + đủ `workflow_task_receipts.evidence_json` và, nếu net-change, cùng một `draft_versions(reason='workflow_round')` + head. Không ghi/tái tạo `pending_patches`.  /  Rollback toàn bộ header/receipt/version/head nếu lỗi; TX-08 đã commit độc lập KHÔNG rollback. Legacy v1 paths ở §6.4 được thay thế cho các round B+ mới bởi §6.4.7.  / | Atomic rollback, no unrelated durable effect; retry checks receipt |

| T-WF-01 | disposable/synthetic fixture described by BE | (a) Chạy Vòng 1 có thay đổi State (`v.04`), gọi retry lại Vòng 1; (b) Chạy Vòng 2 có 0 thay đổi State; (c) Thực hiện `restore_draft_version(2)` tạo ra `v.05` (`5 > 4`), rồi bấm Resume Workflow. | (a) Retry Vòng 1 lookup receipt trước khi ghi; chỉ skip nếu có bằng chứng task-level đầy đủ, nếu không trả `RequiresReplan` và tạo `0` version mới. (b) Vòng 2 tạo biên nhận với `had_mutations = false, resulting_version_seq = NULL` và không tạo version giả. (c) Trên `v.05`, lineage của receipt `v.04` đã bị cắt; không suy chỉ từ `5 > 4` hoặc hash head; trả trạng thái recovery/replan, không tự chạy lại receipt cũ dưới cùng generation. | Only owner-authorized effects; old v1-only statements interpreted as legacy, no B+ regression |

| WF-01 | disposable/synthetic fixture described by BE | Gọi `seal_workflow_round_tx` với `req.completed_tasks = []` và `req.has_applied_write = false` (và trường hợp thứ hai với `req.has_applied_write = true`). | Với `has_applied_write = false`: trả về `Ok(WorkflowRoundSealResponse { sealed_version: None, receipts: [], was_idempotent_replay: false })` mà **không panic**, không tạo `draft_versions`, không tạo biên nhận giả. Với `has_applied_write = true`: trả về `Err(ErrorCode::ValidationFailed)`. | Only owner-authorized effects; old v1-only statements interpreted as legacy, no B+ regression |

| WF-02 | disposable/synthetic fixture described by BE | Gọi `seal_workflow_round_tx` với 1 task mới `T1` (`had_mutations = true`, làm đổi `data_hash` từ `v.03`). | Tạo đúng `1` dòng `draft_versions` mới (`seq = 4`, `reason = 'workflow_round'`, `seal_key = req.round_seal_key`), cập nhật `head_version_seq = 4`, và chèn `1` dòng `workflow_task_receipts` với `had_mutations = 1, resulting_version_seq = 4`. | Only owner-authorized effects; old v1-only statements interpreted as legacy, no B+ regression |

| WF-03 | disposable/synthetic fixture described by BE | Gọi `seal_workflow_round_tx` với 1 task mới `T1` (`had_mutations = false`, `data_hash` không đổi). | Tạo `0` dòng `draft_versions` mới (`head_version_seq` giữ nguyên); chèn đúng `1` dòng `workflow_task_receipts` với `had_mutations = 0, resulting_version_seq = NULL` (thỏa mãn ràng buộc `CHECK` của `SF-06`). | Only owner-authorized effects; old v1-only statements interpreted as legacy, no B+ regression |

| WF-04 | disposable/synthetic fixture described by BE | Gọi `seal_workflow_round_tx` với 3 task mới `[T1, T2, T3]` (trong đó `T1`, `T3` có thay đổi và `T2` không có thay đổi). | Áp dụng thay đổi theo đúng thứ tự tất định `task_index` (`T1 -> T2 -> T3`); chỉ tạo **đúng 1** dòng `draft_versions` (`seq = 4`) cho cả vòng; chèn 3 dòng `workflow_task_receipts` (`T1, T3` có `had_mutations = 1, resulting_version_seq = 4`; `T2` có `had_mutations = 0, resulting_version_seq = NULL`) trong cùng 1 giao dịch nguyên tử. | Only owner-authorized effects; old v1-only statements interpreted as legacy, no B+ regression |

| WF-05 | disposable/synthetic fixture described by BE | B+ kiểm header/child evidence durable theo §6.4.7; legacy v1 chỉ RAM authenticated mới có thể chứng minh đủ S_i. | B+ sau restart vẫn verify lịch sử từ SQLite, skip chỉ khi applicability proven; legacy thiếu S_i ⇒ `RequiresReplan`; không replay từ chuỗi request. | Only owner-authorized effects; old v1-only statements interpreted as legacy, no B+ regression |

| WF-06 | disposable/synthetic fixture described by BE | Prefix có hoặc thiếu evidence; suffix do backend thực thi xác thực. | Chỉ tiếp tục khi từng prefix và output prefix tại head được chứng minh; thiếu → `RequiresReplan`, interleaved → reject. | Only owner-authorized effects; old v1-only statements interpreted as legacy, no B+ regression |

| WF-07 | disposable/synthetic fixture described by BE | Gọi `seal_workflow_round_tx` với task `T1` có cùng `(draft_session_id, workflow_run_id, plan_generation, task_identity)` với một biên nhận đã có trong DB nhưng khác `input_scope_fingerprint` hoặc `output_scope_fingerprint` tái tính toán. | Giao dịch bị bác bỏ ngay lập tức với `Err(ErrorCode::StaleWorkflowReceipt)` và `ROLLBACK` toàn bộ; không tạo version mới và không ghi đè biên nhận lịch sử. | Only owner-authorized effects; old v1-only statements interpreted as legacy, no B+ regression |

| WF-08 | disposable/synthetic fixture described by BE | Restore về snapshot cũ tạo version mới với seq lớn. | Cắt lineage → không reuse receipt cũ; giữ receipt, yêu cầu replan an toàn, không tự chạy lại dưới cùng generation. | Only owner-authorized effects; old v1-only statements interpreted as legacy, no B+ regression |

| WF-09 | disposable/synthetic fixture described by BE | Sửa vùng input của T1, T2 độc lập về mặt kế hoạch. | Không chứng nhận T2 chỉ dựa vào danh nghĩa độc lập; chứng minh lịch sử và applicability từng task, nếu thiếu → `Indeterminate`/`RequiresReplan`. | Only owner-authorized effects; old v1-only statements interpreted as legacy, no B+ regression |

| WF-10 | disposable/synthetic fixture described by BE | Hủy trước TX-04 hoặc ngay sau TX-04 commit. | Trước: không có cặp hàng mới; sau: bảo toàn cặp hàng đã commit; Resume chỉ skip các task có bằng chứng §6.4 đủ mạnh. | Only owner-authorized effects; old v1-only statements interpreted as legacy, no B+ regression |

| WF-C01 | disposable/synthetic fixture described by BE | Trạng thái đầu vòng $S_0$ có `A = 1`. Trong cùng một vòng: `T1` (`task_index = 0`) gọi tool đặt `A = 2` (`applied_write_count = 1`, $S_0 \neq S_1$); `T2` (`task_index = 1`) gọi tool đặt lại `A = 1` (`applied_write_count = 1`, $S_1 \neq S_2$, nhưng $\text{hash}(S_2) == \text{hash}(S_0)$). Gọi `seal_workflow_round_tx`. | Theo **Chiến lược B/C** (§6.4.1): Telemetry runtime báo cáo `T1` và `T2` đều có `has_applied_write = true` và `effective_task_state_changed = true`, nhưng `net_round_state_changed = false` và `durable_version_created = false`. **Không** chèn dòng mới vào `draft_versions` (`resulting_version_seq = None`, tuân thủ `core.md:724, 946, 970-971`). Chèn 2 dòng vào `workflow_task_receipts` với `had_mutations = 0, resulting_version_seq = NULL` (thỏa mãn `CHECK` constraint tại `docs/db.dbml:1335-1349`). | Only owner-authorized effects; old v1-only statements interpreted as legacy, no B+ regression |

| WF-C02 | disposable/synthetic fixture described by BE | Gọi `seal_workflow_round_tx` cho một vòng Phase 3 `review` gồm 3 subagent chỉ đọc (`applied_write_count = 0`, `raw_mutations = []`). | `has_applied_write = false`, `effective_task_state_changed = false`, `net_round_state_changed = false`, `durable_version_created = false`. Chèn 3 biên nhận với `had_mutations = 0, resulting_version_seq = NULL`; **0** dòng `draft_versions` mới được tạo; trả về `resulting_version_seq = None`. | Only owner-authorized effects; old v1-only statements interpreted as legacy, no B+ regression |

| WF-C03 | disposable/synthetic fixture described by BE | Gọi `seal_workflow_round_tx` với 1 task `T1` có `applied_write_count = 2` và làm thay đổi trạng thái ($S_0 \neq S_1$). | `has_applied_write = true`, `effective_task_state_changed = true`, `net_round_state_changed = true`, `durable_version_created = true`. Chèn đúng 1 dòng `draft_versions` mới (`version_seq = k`, `reason = 'workflow_round'`) và 1 biên nhận của `T1` với `had_mutations = 1, resulting_version_seq = k`. | Only owner-authorized effects; old v1-only statements interpreted as legacy, no B+ regression |

| WF-C04 | disposable/synthetic fixture described by BE | Gọi `seal_workflow_round_tx` với `[T1 (mutating, S_0 != S_1), T2 (read-only, S_1 == S_2)]`, tổng vòng có $\text{hash}(S_2) \neq \text{hash}(S_0)$. | Tạo đúng 1 dòng `draft_versions` mới (`version_seq = k`). Biên nhận của `T1` được lưu với `had_mutations = 1, resulting_version_seq = k`; biên nhận của `T2` được lưu với `had_mutations = 0, resulting_version_seq = NULL`. Cả hai được commit nguyên tử trong cùng giao dịch `TX-04`. | Only owner-authorized effects; old v1-only statements interpreted as legacy, no B+ regression |

| WF-C05 | disposable/synthetic fixture described by BE | Gọi `seal_workflow_round_tx` với `completed_tasks = [(0, "A"), (1, "A")]`. | Hàm `validate_round_tasks_pre_mutation` (§6.4.4) phát hiện `"A"` đã tồn tại trong `seen_identities: HashSet<&str>`, trả về `Err(ErrorCode::ValidationFailed)` và `ROLLBACK` trước mọi thay đổi trạng thái hay ghi DB (`0` dòng `draft_versions`, `0` dòng `workflow_task_receipts`). | Only owner-authorized effects; old v1-only statements interpreted as legacy, no B+ regression |

| WF-C06 | disposable/synthetic fixture described by BE | Gọi `seal_workflow_round_tx` với `completed_tasks = [(0, "A"), (1, "B"), (2, "A")]`. | Mặc dù khi sắp xếp theo `task_index ASC` hai phần tử `"A"` bị ngăn cách bởi `"B"`, `seen_identities: HashSet<&str>` phát hiện ngay `"A"` ở `task_index = 2` là trùng lặp toàn cục, trả về `Err(ErrorCode::ValidationFailed)` và `ROLLBACK` toàn bộ trước khi chạm vào `WorkingState` hay SQLite. | Only owner-authorized effects; old v1-only statements interpreted as legacy, no B+ regression |

| WF-C07 | disposable/synthetic fixture described by BE | Gọi `seal_workflow_round_tx` với `completed_tasks = [(0, "A"), (0, "B")]`. | `seen_indices: HashSet<u32>` trong `validate_round_tasks_pre_mutation` (§6.4.4) phát hiện `task_index = 0` bị trùng, trả về `Err(ErrorCode::ValidationFailed)` và `ROLLBACK` toàn bộ. | Only owner-authorized effects; old v1-only statements interpreted as legacy, no B+ regression |

| WF-C08 | disposable/synthetic fixture described by BE | So sánh input S(i-1) và output Si, không dùng head thay S(i-1). | Đủ lịch sử xác thực và applicability → `ValidCompleted`; thiếu S_i → `Indeterminate`. | Only owner-authorized effects; old v1-only statements interpreted as legacy, no B+ regression |

| WF-C09 | disposable/synthetic fixture described by BE | T2 ghi đè output T1 sau khi T1 thực thi hợp lệ. | Không kết luận T1 lịch sử sai chỉ từ head; có bằng chứng chuỗi hạ nguồn → `ValidCompletedSupersededByDownstream`, thiếu → `Indeterminate`. | Only owner-authorized effects; old v1-only statements interpreted as legacy, no B+ regression |

| WF-C10 | disposable/synthetic fixture described by BE | Head sau sửa tay khác input lịch sử. | Có bằng chứng phạm vi và thay đổi gây invalidity → `StaleInputChanged`; thiếu bằng chứng → `Indeterminate`, không skip. | Only owner-authorized effects; old v1-only statements interpreted as legacy, no B+ regression |

| WF-C11 | disposable/synthetic fixture described by BE | Restore về version trước receipt. | Cắt lineage; không reuse receipt, chỉ replan sau khi xác định task cần chạy lại an toàn. | Only owner-authorized effects; old v1-only statements interpreted as legacy, no B+ regression |

| WF-C12 | disposable/synthetic fixture described by BE | Prefix còn/chưa còn trusted task history. | Prefix chỉ được reuse khi đủ chứng cứ; interleaved luôn reject; không khẳng định SQLite tự tái dựng S_i. | Only owner-authorized effects; old v1-only statements interpreted as legacy, no B+ regression |

| WF-C13 | disposable/synthetic fixture described by BE | Trong DB đã có biên nhận `R(T1)` cho `(draft_session_id, workflow_run_id, plan_generation, task_identity)`. Gọi `seal_workflow_round_tx` với `T1` nhưng truyền `req.input_scope_fingerprint` hoặc `req.output_scope_fingerprint` khác với `R(T1)` (hoặc trạng thái tái tính toán không khớp). | Giao dịch bị bác bỏ ngay lập tức với `Err(ErrorCode::StaleWorkflowReceipt)` (§6.4.3–§6.4.5) và `ROLLBACK` toàn bộ; biên nhận lịch sử `R(T1)` được bảo toàn nguyên vẹn. | Only owner-authorized effects; old v1-only statements interpreted as legacy, no B+ regression |

| WF-C14 | disposable/synthetic fixture described by BE | Caller lặp lại request có cùng seal key và fingerprint. | B+ lookup header trước OCC rồi verify durable evidence/applicability; legacy v1 chỉ cùng tiến trình có outcomes authenticated mới có thể kiểm đầy đủ, SQLite-only thiếu patch ⇒ `RequiresReplan`; không duplicate version. | Only owner-authorized effects; old v1-only statements interpreted as legacy, no B+ regression |

| WF-C15 | disposable/synthetic fixture described by BE | Crash sau TX-04 trước IPC response. | B+ SQLite có header/receipts/evidence/optional version để verify S_i; thiếu/corrupt evidence ⇒ `Indeterminate`/`RequiresReplan`, không tái thực thi receipt hay tạo version trùng. Legacy v1 không chứng minh S_i. | Only owner-authorized effects; old v1-only statements interpreted as legacy, no B+ regression |

| phase3_sprint10_deep4-F0 | predecessor gate absent or baseline SHA drift | Attempt dispatch before predecessor accepted | CONTRACT_BLOCKED; no implementation started | No source/DB modification |

Backend module wiring rule: this sprint must not edit any `mod.rs`/`lib.rs`. Its integration test loads each owned implementation file via `#[path = "..."] mod <name>;` and fixture modules via the same mechanism. Production registration occurs only in P6S2 after this sprint is accepted. A compile success is therefore scoped to this test target, not a claim that the whole crate is wired.

Future command after the target files exist: `cargo test --manifest-path src-tauri/Cargo.toml --test sf06_seal_tx  # future target, after bootstrap`.

## 13. Database and migration safety

Use `BEGIN IMMEDIATE` only for the assigned TX; WAL/FULL/FK ON/busy_timeout as specified. Use unique disposable DB/temp root per worktree. Never open a user application DB; no destructive migration without separate authorization. Backup/rollback and PRAGMA user_version are owned by P2S3. TX04 never rolls back committed TX08.

## 14. Concurrency contract

- Peer `phase3_sprint7_deep4`: candidate parallel only after frozen contracts/gates; NONE: Disjoint exact write paths; shared types frozen by predecessors; worktree-local test resources.

Isolation: own worktree/branch from baseline or accepted integration SHA; unique temp root, test DB and port; no shared mutable process/global fixture. Same depth alone is NOT permission to dispatch.

## 15. Deliverables

- [ ] seal_or_retry_round and workflow seal repository, TX04 test fixture

- [ ] Tests above and evidence including command exit codes, scope diff and accepted commit SHA.

## 16. Completion criteria

`SPRINT_IMPLEMENTED` requires artifacts, `SPRINT_TESTED` requires executed acceptance tests, `SPRINT_INTEGRATED` requires merged checks, and **`SPRINT_ACCEPTED` requires `gate_phase3_sprint10_deep4` accepted**. A contract-evidence sprint cannot be ACCEPTED for an affected implementation successor while its owner decision is unresolved. No agent declaration alone unlocks successors. All required assertions, file scope and baseline must pass.

## 17. Handoff contract

- seal_or_retry_round and workflow seal repository, TX04 test fixture

- Exact artifacts: src-tauri/src/storage/workflow_seal_repo.rs, src-tauri/src/service/round_seal_service.rs, src-tauri/tests/sf06_seal_tx.rs; accepted integration SHA and `gate_phase3_sprint10_deep4` record. A successor must verify exports/ABI rather than infer from prose.

## 18. Resume and failure behavior

Not started: verify baseline and deps. Partial/test failure: inspect git status, changed paths and durable test DB, resume only safe incomplete steps. Implemented but unintegrated: do not dispatch successors. Interrupted agent/rejected patch: preserve branch, rebase only after review, rerun tests. Specification conflict: `CONTRACT_BLOCKED`, record evidence, request authority resolution; never edit SSOT in this sprint.

**Explicit unblock requirements:** CI-02.


---

Subagent budget: 256K tokens maximum; start with the bounded references above, do not read entire SSOT by default. CLI-first with verified tool compatibility. Treat Jev/TypeSafe as optional advisory only when available and authorized; never send private repository content externally without consent.
