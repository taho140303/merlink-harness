---
sprint_id: phase3_sprint1_deep0
title: Implement planner, progress and cross-entity validation
phase: 3
sprint: 1
deep: 0
baseline_commit: 57a462efb953a665e3673b15e6cc33622a4f2d39
status: PLANNED
---

# phase3_sprint1_deep0 — Implement planner, progress and cross-entity validation

## 1. Sprint identity

Engineering phase 3 (Draft domain and durable workflow), within-phase depth 0; baseline is the Git commit above, not an uncommitted workspace. **PROPOSED, NOT IMPLEMENTED.**

## 2. Objective

Implement planner, progress and cross-entity validation. Observable handoff: plan_outline_runs/split_balanced/plan_phase and progress/validation pure APIs.

## 3. Scope boundaries

**IN_SCOPE:** Implement pure planning/progress/validation APIs and property tests.; No provider/SQLite access; array order and stable codes come from frozen P2S2 types.

**OUT_OF_SCOPE / MUST NOT:** modify authoritative `docs/business/**`, `docs/db.dbml`, `docs/architecture/decisions/**`; use real application data; run production migration; add client request IDs to SF-06; invent business semantics; rewrite another sprint’s files.

## 4. Authoritative contract snapshot

- Plan Phase1–2 sequential windows and Phase3–6 groups; freeze P per plan; never hardcode six into SQLite.

- 46 attributes count source-sensitive and world_setting_code remains a separate commit gate.

- **REQ-DOM-003** `docs/business/core.md#L596-L620`: Exactly 46 master attributes and confirmed/AI composite provenance precedence. (planning coverage `ASSIGNED`).

  Normative excerpt (verbatim baseline lines, truncated to 300 chars; use `sed` for the full range):

  > L596: ### 3.1 Trình đo lường tiến độ (Progress Engine)

  > L598: * Hệ thống duyệt qua **46 thuộc tính Master list** (`MASTER_ATTRIBUTES_LIST` — chuẩn thẩm quyền của Business Core; không tự đổi thành 50 theo mã nguồn FE prototype).

  > L599: * Hiển thị hai phần trăm riêng trên thanh Header, mỗi phần chia cho tổng 46 thuộc tính và mỗi thuộc tính đóng góp 0 hoặc 1 theo evaluator (giữ nguyên cách tính mảng: một thuộc tính mảng đạt 1 khi có ít nhất một phần tử hợp lệ trong mảng tương ứng, và phân loại theo `source` của giá trị đã điền): **Đ

  > L600: * **Quy tắc đối chiếu 46 thuộc tính chuẩn, Composite Provenance và `world_setting_code` (`APPROVED v2.4.4..v2.4.5`):**

- **REQ-DOM-005** `docs/business/be-core.md#L681-L736`: Readiness, progress, validation; invalid world_setting_code blocks commit. (planning coverage `ASSIGNED`).

  Normative excerpt (verbatim baseline lines, truncated to 300 chars; use `sed` for the full range):

  > L681: ### 4.6 Tích hợp Trình Đo lường Tiến độ (46 Master Attributes Progress Engine)

  > L683: Theo `core.md` §3.1 và `fe-core.md` `Q-C1`, `Q-C2` (`APPROVED v2.4.5`):

  > L685: #### 4.6.1 Chuẩn thẩm quyền 46 Thuộc tính Master (`10 + 9 + 4 + 6 + 4 + 6 + 4 + 2 + 1 = 46`)

  > L687: `domain::progress_engine` `MUST` tính toán tiến độ trên đúng **46 thuộc tính chuẩn** (loại bỏ chính xác 4 mục không lưu cột DBML khỏi danh sách 50 của FE prototype: `arc_tolerance`, `chapter_tolerance`, `word_per_chapter_tolerance`, và `relationship.motif.event_blocks`):

- **REQ-WF-001** `docs/business/core.md#L835-L948`: Outline windows, balanced tasks, frozen MAX_PARALLEL, deterministic task_index application. (planning coverage `ASSIGNED`).

  Normative excerpt (verbatim baseline lines, truncated to 300 chars; use `sed` for the full range):

  > L835: ### 3.9 Workflow phân đoạn (Segmented Workflow)

  > L837: * **Mục tiêu:** Truyện dài hàng trăm chương không bị nhồi vào một lượt AI. Agent đóng gói Phase 1–6 thành workflow; **hàm thuần ở backend** tính kế hoạch chia task. Orchestrator không tự suy luận cách chia, chỉ chạy hàm kế hoạch rồi gọi tool/subagent theo kết quả.

  > L838: * **Tham số cấu hình runtime:** Giá trị mặc định tập trung trong cấu hình, không hard-code trong thuật toán.

  > L840: / Tham số / Mặc định / Ý nghĩa /

- **REQ-SPECTEST-T-UNIT-01** `docs/business/be-core.md#L2523-L2523`: MUST implement executable BE acceptance vector T-UNIT-01, preserving scenario and expected result; old v1-only recovery claims remain legacy-only under B+. (planning coverage `ASSIGNED`).

- **REQ-SPECTEST-T-UNIT-03** `docs/business/be-core.md#L2525-L2525`: MUST implement executable BE acceptance vector T-UNIT-03, preserving scenario and expected result; old v1-only recovery claims remain legacy-only under B+. (planning coverage `ASSIGNED`).

- **REQ-SPECTEST-T-UNIT-04** `docs/business/be-core.md#L2526-L2526`: MUST implement executable BE acceptance vector T-UNIT-04, preserving scenario and expected result; old v1-only recovery claims remain legacy-only under B+. (planning coverage `ASSIGNED`).

**Targeted source references (read only these ranges unless a test proves broader context is necessary):**

- `docs/business/core.md#L596-L620` — focus: 46 attributes and provenance precedence; CLI: `sed -n "596,620p" docs/business/core.md` (or `rg -n "<symbol>" docs/business/core.md` for a changed anchor).

- `docs/business/core.md#L835-L948` — focus: Outline cap/lookback, balanced rounds, phase order; CLI: `sed -n "835,948p" docs/business/core.md` (or `rg -n "<symbol>" docs/business/core.md` for a changed anchor).

- `docs/business/be-core.md#L681-L736` — focus: Progress/readiness; CLI: `sed -n "681,736p" docs/business/be-core.md` (or `rg -n "<symbol>" docs/business/be-core.md` for a changed anchor).

- `docs/business/be-core.md#L2523-L2526` — focus: Exact planner/validation vectors; CLI: `sed -n "2523,2526p" docs/business/be-core.md` (or `rg -n "<symbol>" docs/business/be-core.md` for a changed anchor).

## 5. Prerequisites

`depends_on: ["phase2_sprint2_deep1"]`.

- `phase2_sprint2_deep1` [TYPE_DEPENDENCY] produces Frozen BusinessFlowData/TrackedField/CanonicalStateBytes APIs and pure test target; inspect its accepted commit, exported artifact/test and `gate_phase2_sprint2_deep1`. If absent, do not start.

## 6. Inputs

Required: accepted outputs of all predecessors; frozen committed SSOT ranges above; actual source paths listed below. Optional context: existing UI mocks for fixtures only. No `report/` input; no historical research.

## 7. File ownership

```yaml

allowed_write_paths:

  - src-tauri/src/domain/workflow_planner.rs

  - src-tauri/src/domain/progress_engine.rs

  - src-tauri/src/domain/invariants.rs

  - src-tauri/tests/planner_progress.rs

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

PROPOSED_INTERFACE: plan_outline_runs, split_balanced, plan_phase, context_range, ProgressSummary(confirmed,ai_suggested,denominator=46), validate_draft_for_commit. Every consumed prerequisite interface is `PREREQUISITE_INTERFACE`, not an existing runtime guarantee. If a named signature conflicts with committed SSOT, stop `CONTRACT_BLOCKED`.

## 9. Implementation requirements

- Implement pure planning/progress/validation APIs and property tests.

- No provider/SQLite access; array order and stable codes come from frozen P2S2 types.

- Prefer official compatible CLI/tooling over manual generated files; verify installed version and diff. No external LLM/Jev request without authorization; optional Jev advice never overrides SSOT.

## 10. Domain invariants

- Plan Phase1–2 sequential windows and Phase3–6 groups; freeze P per plan; never hardcode six into SQLite.

- 46 attributes count source-sensitive and world_setting_code remains a separate commit gate.

- No partial state escapes; protected provenance, explicit authorization, no external-effect recovery replay where applicable.

## 11. Failure matrix

| Trigger | Expected behavior / classification | Durable effects and recovery | Required test |

|---|---|---|---|

| Invalid lookback>=cap | ValidationFailed, no tasks; P3S1-N. | No unauthorized partial commit; re-read actual durable state before retry | phase3_sprint1_deep0-F1 |

| Missing world_setting_code | commit validation issue, not 47th progress attribute; P3S1-C. | No unauthorized partial commit; re-read actual durable state before retry | phase3_sprint1_deep0-F2 |

| Missing accepted predecessor / baseline mismatch | `CONTRACT_BLOCKED`; do not run implementation | No user-data effects; record issue and owner in sprint evidence | phase3_sprint1_deep0-F0 |

## 12. Acceptance tests

Tests below name **future targets**, not tests executed during planning. Each row must become an executable test before the sprint can be accepted.

| Test ID | Initial state | Input / action | Expected output and state | Forbidden side effects / invariant |

|---|---|---|---|---|

| P3S1-P | pure Rust fixture (no DB/network/Tauri) + accepted predecessor types | In the owned test target, set up the fixture and run the scenario: Arc 146..260 yields writes 146..195,196..235,236..260. | Arc 146..260 yields writes 146..195,196..235,236..260. | No unrelated write; preserve exact expected state and listed invariant |

| P3S1-N | pure Rust fixture (no DB/network/Tauri) + accepted predecessor types | In the owned test target, set up the fixture and run the scenario: 15 tasks at P6 are rounds 6+6+3, never one round 15. | 15 tasks at P6 are rounds 6+6+3, never one round 15. | No unrelated write; preserve exact expected state and listed invariant |

| P3S1-B | pure Rust fixture (no DB/network/Tauri) + accepted predecessor types | In the owned test target, set up the fixture and run the scenario: 46-attribute mixed-source array counts once by confirmed>AI>incomplete. | 46-attribute mixed-source array counts once by confirmed>AI>incomplete. | No unrelated write; preserve exact expected state and listed invariant |

| phase3_sprint1_deep0-F1 | accepted prerequisite fixture / isolated worktree | Inject Invalid lookback>=cap into owned operation | ValidationFailed, no tasks; P3S1-N. | No unauthorized commit or external replay |

| phase3_sprint1_deep0-F2 | accepted prerequisite fixture / isolated worktree | Inject Missing world_setting_code into owned operation | commit validation issue, not 47th progress attribute; P3S1-C. | No unauthorized commit or external replay |

| T-UNIT-01 | disposable/synthetic fixture described by BE | Truyền `BusinessFlowData` có 4 trường `*_tolerance` / `relationship.motif.event_blocks` và các mảng trộn lẫn phần tử `user`, `user_edited`, `ai_generated`, `incomplete`. | Mẫu số luôn cố định bằng `46`. 4 trường loại trừ không làm tăng tử số/mẫu số. Mảng có $\ge 1$ phần tử hợp lệ đạt `confirmed` được tính vào `confirmed_count`; ưu tiên `confirmed > ai_suggested > incomplete`; không có thuộc tính nào được cộng vào cả hai tử số. | Only owner-authorized effects; old v1-only statements interpreted as legacy, no B+ regression |

| T-UNIT-03 | disposable/synthetic fixture described by BE | Chạy `plan_outline_runs(146, 260, 50, 10)` và `plan_phase` cho 5 Arc dài `30/55/65/50/65` với `unit = 30, max_parallel = 6`. | `plan_outline_runs` trả đúng 3 lần chạy: `(None, 146..195)`, `(186..195, 196..235)`, `(226..235, 236..260)`. `plan_phase` sinh đúng `11` task chia làm 2 vòng (`6` task và `5` task); `lookback >= cap` bị từ chối. | Only owner-authorized effects; old v1-only statements interpreted as legacy, no B+ regression |

| T-UNIT-04 | disposable/synthetic fixture described by BE | Kiểm tra bản nháp thiếu `scenes[].world_setting_code`, lệch tổng chương so với `target.chapter_count`, trùng `chapters.position` xuyên Arc, hoặc sai số lượng `is_pov` theo `narrator_config`. | Hàm `validate_draft_for_commit` phát hiện và trả về đúng mã lỗi cho từng vi phạm; `world_setting_code` bắt buộc phải có trước commit dù nằm ngoài 46 thuộc tính tiến độ. | Only owner-authorized effects; old v1-only statements interpreted as legacy, no B+ regression |

| phase3_sprint1_deep0-F0 | predecessor gate absent or baseline SHA drift | Attempt dispatch before predecessor accepted | CONTRACT_BLOCKED; no implementation started | No source/DB modification |

Backend module wiring rule: this sprint must not edit any `mod.rs`/`lib.rs`. Its integration test loads each owned implementation file via `#[path = "..."] mod <name>;` and fixture modules via the same mechanism. Production registration occurs only in P6S2 after this sprint is accepted. A compile success is therefore scoped to this test target, not a claim that the whole crate is wired.

Future command after the target files exist: `cargo test --manifest-path src-tauri/Cargo.toml --test planner_progress  # future target, after bootstrap`.

## 13. Database and migration safety

No persistent database in this sprint; fixture DB (if any) is disposable and uniquely named per worktree.

## 14. Concurrency contract

- Peer `phase3_sprint2_deep0`: candidate parallel only after frozen contracts/gates; NONE: Disjoint exact write paths; shared types frozen by predecessors; worktree-local test resources.

- Peer `phase3_sprint3_deep0`: candidate parallel only after frozen contracts/gates; NONE: Disjoint exact write paths; shared types frozen by predecessors; worktree-local test resources.

Isolation: own worktree/branch from baseline or accepted integration SHA; unique temp root, test DB and port; no shared mutable process/global fixture. Same depth alone is NOT permission to dispatch.

## 15. Deliverables

- [ ] plan_outline_runs/split_balanced/plan_phase and progress/validation pure APIs

- [ ] Tests above and evidence including command exit codes, scope diff and accepted commit SHA.

## 16. Completion criteria

`SPRINT_IMPLEMENTED` requires artifacts, `SPRINT_TESTED` requires executed acceptance tests, `SPRINT_INTEGRATED` requires merged checks, and **`SPRINT_ACCEPTED` requires `gate_phase3_sprint1_deep0` accepted**. A contract-evidence sprint cannot be ACCEPTED for an affected implementation successor while its owner decision is unresolved. No agent declaration alone unlocks successors. All required assertions, file scope and baseline must pass.

## 17. Handoff contract

- plan_outline_runs/split_balanced/plan_phase and progress/validation pure APIs

- Exact artifacts: src-tauri/src/domain/workflow_planner.rs, src-tauri/src/domain/progress_engine.rs, src-tauri/src/domain/invariants.rs, src-tauri/tests/planner_progress.rs; accepted integration SHA and `gate_phase3_sprint1_deep0` record. A successor must verify exports/ABI rather than infer from prose.

## 18. Resume and failure behavior

Not started: verify baseline and deps. Partial/test failure: inspect git status, changed paths and durable test DB, resume only safe incomplete steps. Implemented but unintegrated: do not dispatch successors. Interrupted agent/rejected patch: preserve branch, rebase only after review, rerun tests. Specification conflict: `CONTRACT_BLOCKED`, record evidence, request authority resolution; never edit SSOT in this sprint.


---

Subagent budget: 256K tokens maximum; start with the bounded references above, do not read entire SSOT by default. CLI-first with verified tool compatibility. Treat Jev/TypeSafe as optional advisory only when available and authorized; never send private repository content externally without consent.
