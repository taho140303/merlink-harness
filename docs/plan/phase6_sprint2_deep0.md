---
sprint_id: phase6_sprint2_deep0
title: Compose global Tauri and React integration
phase: 6
sprint: 2
deep: 0
baseline_commit: 57a462efb953a665e3673b15e6cc33622a4f2d39
status: PLANNED
---

# phase6_sprint2_deep0 — Compose global Tauri and React integration

## 1. Sprint identity

Engineering phase 6 (Application assurance), within-phase depth 0; baseline is the Git commit above, not an uncommitted workspace. **PROPOSED, NOT IMPLEMENTED.**

## 2. Objective

Compose global Tauri and React integration. Observable handoff: Integrated revision SHA, global registration, startup recovery and IPC inventory test.

## 3. Scope boundaries

**IN_SCOPE:** Integrate accepted branch commits deterministically by sprint ID; register commands and channel handlers.; Test 40-name registry, 3 payload types, startup crash fixtures and shutdown cancellation; preserve previous accepted commits.

**OUT_OF_SCOPE / MUST NOT:** modify authoritative `docs/business/**`, `docs/db.dbml`, `docs/architecture/decisions/**`; use real application data; run production migration; add client request IDs to SF-06; invent business semantics; rewrite another sprint’s files.

## 4. Authoritative contract snapshot

- Only this sprint composes global lib/mod/routes/dict; no component redesign.

- Ownership handoff: P2S1 scaffolded the crate and module roots; after P2S1 acceptance, P6S2 is the sole final composer and may edit those paths. No later repair to P2S1 may write them without a new integration revision.

- App startup migrates/verifies/reloads sealed state and recovers before commands; 40 commands/3 channels exactly, no omitted registration.

- **REQ-ARCH-002** `docs/business/be-core.md#L131-L255`: AppState startup/shutdown, canonical app-data/temp paths, typed errors and cancellation. (planning coverage `ASSIGNED`).

  Normative excerpt (verbatim baseline lines, truncated to 300 chars; use `sed` for the full range):

  > L131: ### 1.4 Vòng đời ứng dụng Tauri v2 & Quyền sở hữu `AppState`

  > L133: #### 1.4.1 Khởi tạo (`Builder::setup`)

  > L135: Khi ứng dụng khởi chạy, hàm `tauri::Builder::default().setup(...)` `MUST` thực hiện tuần tự các bước sau trước khi mở cửa sổ chính cho người dùng tương tác:

  > L137: 1. **Khởi tạo hệ thống ghi nhật ký có bộ lọc bảo mật (`redaction`):** Cài đặt subscriber cho `tracing` kèm lớp lọc tự động xóa mọi chuỗi khớp mẫu khóa API (`sk-*`, `Bearer *`, và các khóa đang nạp trong `CredentialStore`).

- **REQ-IPC-002** `docs/business/be-core.md#L2110-L2251`: All 40 commands in 10 groups with caller authorization and 3 event channels. (planning coverage `ASSIGNED`).

  Normative excerpt (verbatim baseline lines, truncated to 300 chars; use `sed` for the full range):

  > L2110: ### 10.2 Danh mục Đầy đủ các Lệnh Typed IPC (Command Contract Specification — 40 Commands)

  > L2112: Hệ thống Backend cung cấp tổng cộng **40 lệnh Typed IPC duy nhất** chia theo **10 nhóm chức năng** (`4 + 5 + 2 + 4 + 3 + 3 + 3 + 4 + 9 + 3 = 40`), đi kèm **3 kênh sự kiện streaming (`tauri::ipc::Channel<T>`)** được định nghĩa tách biệt tại §10.3 (`ChatStreamEvent`, `WorkflowProgressEvent`, `SceneGen

  > L2114: #### Bảng Tổng hợp Kiểm kê 40 Lệnh IPC (Master IPC Command Inventory)

  > L2116: / # / Command Name / Category (Nhóm chức năng) / Request DTO / Response DTO / Read-Only vs Mutating / Uses Streaming (`Channel<T>`) / Requires SQLite Transaction /

- **REQ-IPC-003** `docs/business/be-core.md#L2253-L2412`: Streaming event lifecycle, cancellation, FE reconciliation and loss handling. (planning coverage `ASSIGNED`).

  Normative excerpt (verbatim baseline lines, truncated to 300 chars; use `sed` for the full range):

  > L2253: ### 10.3 Định nghĩa Payload của các Kênh Streaming (Tauri v2 `Channel<T>` Events)

  > L2255: Tauri v2 sử dụng `tauri::ipc::Channel<T>` có kiểu tĩnh để truyền luồng sự kiện từ Rust sang Frontend theo thời gian thực:

  > L2257: #### 10.3.1 `ChatStreamEvent` (Luồng Hội thoại & Tool Call trong Planning Mode)

  > L2259: ```rust

- **REQ-OPS-001** `docs/business/be-core.md#L131-L174`: Tauri startup, registration, shutdown and recovery before IPC available. (planning coverage `ASSIGNED`).

  Normative excerpt (verbatim baseline lines, truncated to 300 chars; use `sed` for the full range):

  > L131: ### 1.4 Vòng đời ứng dụng Tauri v2 & Quyền sở hữu `AppState`

  > L133: #### 1.4.1 Khởi tạo (`Builder::setup`)

  > L135: Khi ứng dụng khởi chạy, hàm `tauri::Builder::default().setup(...)` `MUST` thực hiện tuần tự các bước sau trước khi mở cửa sổ chính cho người dùng tương tác:

  > L137: 1. **Khởi tạo hệ thống ghi nhật ký có bộ lọc bảo mật (`redaction`):** Cài đặt subscriber cho `tracing` kèm lớp lọc tự động xóa mọi chuỗi khớp mẫu khóa API (`sk-*`, `Bearer *`, và các khóa đang nạp trong `CredentialStore`).

- **REQ-SPECTEST-T-REC-01** `docs/business/be-core.md#L2538-L2538`: MUST implement executable BE acceptance vector T-REC-01, preserving scenario and expected result; old v1-only recovery claims remain legacy-only under B+. (planning coverage `ASSIGNED`).

- **REQ-SPECTEST-IPC-01** `docs/business/be-core.md#L2562-L2562`: MUST implement executable BE acceptance vector IPC-01, preserving scenario and expected result; old v1-only recovery claims remain legacy-only under B+. (planning coverage `ASSIGNED`).

- **REQ-SPECTEST-IPC-02** `docs/business/be-core.md#L2563-L2563`: MUST implement executable BE acceptance vector IPC-02, preserving scenario and expected result; old v1-only recovery claims remain legacy-only under B+. (planning coverage `ASSIGNED`).

**Targeted source references (read only these ranges unless a test proves broader context is necessary):**

- `docs/business/be-core.md#L131-L174` — focus: Tauri startup/shutdown order; CLI: `sed -n "131,174p" docs/business/be-core.md` (or `rg -n "<symbol>" docs/business/be-core.md` for a changed anchor).

- `docs/business/be-core.md#L2110-L2257` — focus: All 40 IPC and channels; CLI: `sed -n "2110,2257p" docs/business/be-core.md` (or `rg -n "<symbol>" docs/business/be-core.md` for a changed anchor).

- `docs/business/be-core.md#L2415-L2511` — focus: Eight recovery scenarios; CLI: `sed -n "2415,2511p" docs/business/be-core.md` (or `rg -n "<symbol>" docs/business/be-core.md` for a changed anchor).

- `docs/business/fe-core.md#L495-L514` — focus: Layout/i18n preservation; CLI: `sed -n "495,514p" docs/business/fe-core.md` (or `rg -n "<symbol>" docs/business/fe-core.md` for a changed anchor).

## 5. Prerequisites

`depends_on: ["phase4_sprint1_deep0", "phase4_sprint2_deep0", "phase4_sprint3_deep1", "phase4_sprint4_deep1", "phase4_sprint5_deep2", "phase5_sprint2_deep1", "phase5_sprint3_deep2", "phase5_sprint4_deep1", "phase5_sprint5_deep3", "phase5_sprint6_deep1"]`.

- `phase4_sprint1_deep0` [INTEGRATION_DEPENDENCY] produces WorkflowService, WorkflowProgressEvent and IPC #16–18; inspect its accepted commit, exported artifact/test and `gate_phase4_sprint1_deep0`. If absent, do not start.

- `phase4_sprint2_deep0` [INTEGRATION_DEPENDENCY] produces CommitService/CommitRepo, preview/commit/reopen IPC #19–21; inspect its accepted commit, exported artifact/test and `gate_phase4_sprint2_deep0`. If absent, do not start.

- `phase4_sprint3_deep1` [INTEGRATION_DEPENDENCY] produces Project/Character services, repos and IPC #22–28; inspect its accepted commit, exported artifact/test and `gate_phase4_sprint3_deep1`. If absent, do not start.

- `phase4_sprint4_deep1` [INTEGRATION_DEPENDENCY] produces WritingPersistence/read model and IPC #29–31,#35–37; inspect its accepted commit, exported artifact/test and `gate_phase4_sprint4_deep1`. If absent, do not start.

- `phase4_sprint5_deep2` [INTEGRATION_DEPENDENCY] produces WritingGeneration service, SceneGenerationStreamEvent and IPC #32–34; inspect its accepted commit, exported artifact/test and `gate_phase4_sprint5_deep2`. If absent, do not start.

- `phase5_sprint2_deep1` [INTEGRATION_DEPENDENCY] produces Chat/progress integration and tests; props/event contract for DraftWorkspace owner; inspect its accepted commit, exported artifact/test and `gate_phase5_sprint2_deep1`. If absent, do not start.

- `phase5_sprint3_deep2` [INTEGRATION_DEPENDENCY] produces Draft/commit/workflow/pending UI and deterministic navigation/fixture tests; inspect its accepted commit, exported artifact/test and `gate_phase5_sprint3_deep2`. If absent, do not start.

- `phase5_sprint4_deep1` [INTEGRATION_DEPENDENCY] produces Project/character FE read-model integration and tests; inspect its accepted commit, exported artifact/test and `gate_phase5_sprint4_deep1`. If absent, do not start.

- `phase5_sprint5_deep3` [INTEGRATION_DEPENDENCY] produces ChapterDetailPage/ChapterDetailView live integration, scene adapter and visual/test evidence; inspect its accepted commit, exported artifact/test and `gate_phase5_sprint5_deep3`. If absent, do not start.

- `phase5_sprint6_deep1` [INTEGRATION_DEPENDENCY] produces Provider settings integration and secret-safe FE tests; inspect its accepted commit, exported artifact/test and `gate_phase5_sprint6_deep1`. If absent, do not start.

## 6. Inputs

Required: accepted outputs of all predecessors; frozen committed SSOT ranges above; actual source paths listed below. Optional context: existing UI mocks for fixtures only. No `report/` input; no historical research.

## 7. File ownership

```yaml

allowed_write_paths:

  - src-tauri/src/lib.rs

  - src-tauri/src/ipc/mod.rs

  - src-tauri/src/domain/mod.rs

  - src-tauri/src/storage/mod.rs

  - src-tauri/src/service/mod.rs

  - src-tauri/src/providers/mod.rs

  - src-tauri/src/domain/sf06/mod.rs

  - src-tauri/src/domain/tools/mod.rs

  - src-tauri/src/service/recovery_service.rs

  - src/app/providers/AppProviders.tsx

  - src/app/routes.ts

  - src/shared/i18n/dict.ts

  - src-tauri/tests/startup_recovery.rs

forbidden_write_paths:

  - docs/business/**

  - docs/db.dbml

  - docs/architecture/decisions/**

  - app-data/**

  - report/**

shared_read_only_paths:

  - docs/business/be-core.md

  - docs/business/fe-core.md

generated_paths: []

```

Any allowed path may be absent at baseline (not only `src-tauri/`; new FE adapter/shim files are also future targets). Verify with `test -e`/`git ls-files` before editing; existing files are read first. Any extra path requires central integration approval, not a silent scope expansion.

## 8. Public interfaces

PROPOSED_INTERFACE: Tauri command registration for exactly 40 names and 3 event channels; startup recovery before serving IPC. Every consumed prerequisite interface is `PREREQUISITE_INTERFACE`, not an existing runtime guarantee. If a named signature conflicts with committed SSOT, stop `CONTRACT_BLOCKED`.

## 9. Implementation requirements

- Integrate accepted branch commits deterministically by sprint ID; register commands and channel handlers.

- Test 40-name registry, 3 payload types, startup crash fixtures and shutdown cancellation; preserve previous accepted commits.

- Prefer official compatible CLI/tooling over manual generated files; verify installed version and diff. No external LLM/Jev request without authorization; optional Jev advice never overrides SSOT.

## 10. Domain invariants

- Only this sprint composes global lib/mod/routes/dict; no component redesign.

- Ownership handoff: P2S1 scaffolded the crate and module roots; after P2S1 acceptance, P6S2 is the sole final composer and may edit those paths. No later repair to P2S1 may write them without a new integration revision.

- App startup migrates/verifies/reloads sealed state and recovers before commands; 40 commands/3 channels exactly, no omitted registration.

- No partial state escapes; protected provenance, explicit authorization, no external-effect recovery replay where applicable.

## 11. Failure matrix

| Trigger | Expected behavior / classification | Durable effects and recovery | Required test |

|---|---|---|---|

| Missing command/ABI mismatch | integration gate rejects, no downstream dispatch; P6S2-N. | No unauthorized partial commit; re-read actual durable state before retry | phase6_sprint2_deep0-F1 |

| Recovery corrupt evidence | RequiresReplan not ValidCompleted; P6S2-C. | No unauthorized partial commit; re-read actual durable state before retry | phase6_sprint2_deep0-F2 |

| Missing accepted predecessor / baseline mismatch | `CONTRACT_BLOCKED`; do not run implementation | No user-data effects; record issue and owner in sprint evidence | phase6_sprint2_deep0-F0 |

## 12. Acceptance tests

Tests below name **future targets**, not tests executed during planning. Each row must become an executable test before the sprint can be accepted.

| Test ID | Initial state | Input / action | Expected output and state | Forbidden side effects / invariant |

|---|---|---|---|---|

| P6S2-P | accepted integrated worktree or isolated CI checkout | In the owned test target, set up the fixture and run the scenario: exactly 40 registered commands and 3 typed channel variants/groups. | exactly 40 registered commands and 3 typed channel variants/groups. | No unrelated write; preserve exact expected state and listed invariant |

| P6S2-N | accepted integrated worktree or isolated CI checkout | In the owned test target, set up the fixture and run the scenario: stale/legacy evidence never auto-skipped. | stale/legacy evidence never auto-skipped. | No unrelated write; preserve exact expected state and listed invariant |

| P6S2-B | accepted integrated worktree or isolated CI checkout | In the owned test target, set up the fixture and run the scenario: startup after interrupted commit/turn/Save returns authoritative state. | startup after interrupted commit/turn/Save returns authoritative state. | No unrelated write; preserve exact expected state and listed invariant |

| phase6_sprint2_deep0-F1 | accepted prerequisite fixture / isolated worktree | Inject Missing command/ABI mismatch into owned operation | integration gate rejects, no downstream dispatch; P6S2-N. | No unauthorized commit or external replay |

| phase6_sprint2_deep0-F2 | accepted prerequisite fixture / isolated worktree | Inject Recovery corrupt evidence into owned operation | RequiresReplan not ValidCompleted; P6S2-C. | No unauthorized commit or external replay |

| T-REC-01 | disposable/synthetic fixture described by BE | Giả lập DB có `draft_sessions.status = 'committing'`, `messages.payload_json` có tool ghi `applied` chưa có `draft_versions`, và file report tạm lệch fingerprint trong `./merlink-harness/tmp/`; chạy `run_startup_recovery`. | `draft_sessions.status` được đưa về `'editing'`; `WorkingState` nạp từ sealed version cuối; phát hiện đúng `interruptedTurnAlert`; xóa sạch report tạm lệch fingerprint. | Only owner-authorized effects; old v1-only statements interpreted as legacy, no B+ regression |

| IPC-01 | disposable/synthetic fixture described by BE | Đếm tổng số lệnh IPC duy nhất trong Bảng Tổng hợp §10.2 và đối chiếu với tổng số lệnh trong 10 bảng chi tiết §10.2.1–§10.2.10 cùng 3 kênh streaming tại §10.3. | Khớp chính xác **40 lệnh IPC duy nhất** (`4 + 5 + 2 + 4 + 3 + 3 + 3 + 4 + 9 + 3 = 40`) và **3 kênh sự kiện streaming (`Channel<T>`)** (`ChatStreamEvent`, `WorkflowProgressEvent`, `SceneGenerationStreamEvent`); không có lệnh trùng lặp hoặc tính thiếu. | Only owner-authorized effects; old v1-only statements interpreted as legacy, no B+ regression |

| IPC-02 | disposable/synthetic fixture described by BE | Đối chiếu mọi tên lệnh IPC được nhắc tới trong §1, §3.6, §4, §5, §6, §7, §8, §9, §11, §12, §13 với Bảng Tổng hợp 40 lệnh tại §10.2. | 100% tên lệnh IPC trong toàn bộ văn bản (`get_commit_confirmation_preview`, `set_scene_lock`, `set_chapter_scenes_lock_bulk`, `respond_edit_scope_expansion`, `confirm_pending_patch`, `confirm_all_pending_patches`, `restore_draft_version`...) khớp chính xác từng ký tự với định nghĩa chuẩn tại §10.2. | Only owner-authorized effects; old v1-only statements interpreted as legacy, no B+ regression |

| phase6_sprint2_deep0-F0 | predecessor gate absent or baseline SHA drift | Attempt dispatch before predecessor accepted | CONTRACT_BLOCKED; no implementation started | No source/DB modification |

Future command after the target files exist: `pnpm test && pnpm typecheck && pnpm lint && pnpm build && cargo test --manifest-path src-tauri/Cargo.toml  # future target`. For FE changes, visual comparison at 1280/1279px and each declared viewport is mandatory; if tooling is unavailable the sprint is `BLOCKED_VISUAL_VERIFICATION` and cannot be accepted. `VISUAL_NOT_VERIFIED` is not a waiver.

## 13. Database and migration safety

Use `BEGIN IMMEDIATE` only for the assigned TX; WAL/FULL/FK ON/busy_timeout as specified. Use unique disposable DB/temp root per worktree. Never open a user application DB; no destructive migration without separate authorization. Backup/rollback and PRAGMA user_version are owned by P2S3. TX04 never rolls back committed TX08.

## 14. Concurrency contract

- Peer `phase6_sprint1_deep0`: candidate parallel only after frozen contracts/gates; NONE: Disjoint exact write paths; shared types frozen by predecessors; worktree-local test resources.

Isolation: own worktree/branch from baseline or accepted integration SHA; unique temp root, test DB and port; no shared mutable process/global fixture. Same depth alone is NOT permission to dispatch.

## 15. Deliverables

- [ ] Integrated revision SHA, global registration, startup recovery and IPC inventory test

- [ ] Tests above and evidence including command exit codes, scope diff and accepted commit SHA.

## 16. Completion criteria

`SPRINT_IMPLEMENTED` requires artifacts, `SPRINT_TESTED` requires executed acceptance tests, `SPRINT_INTEGRATED` requires merged checks, and **`SPRINT_ACCEPTED` requires `gate_phase6_sprint2_deep0` accepted**. A contract-evidence sprint cannot be ACCEPTED for an affected implementation successor while its owner decision is unresolved. No agent declaration alone unlocks successors. All required assertions, file scope and baseline must pass.

## 17. Handoff contract

- Integrated revision SHA, global registration, startup recovery and IPC inventory test

- Exact artifacts: src-tauri/src/lib.rs, src-tauri/src/ipc/mod.rs, src-tauri/src/domain/mod.rs, src-tauri/src/storage/mod.rs, src-tauri/src/service/mod.rs, src-tauri/src/providers/mod.rs, src-tauri/src/domain/sf06/mod.rs, src-tauri/src/domain/tools/mod.rs, src-tauri/src/service/recovery_service.rs, src/app/providers/AppProviders.tsx, src/app/routes.ts, src/shared/i18n/dict.ts, src-tauri/tests/startup_recovery.rs; accepted integration SHA and `gate_phase6_sprint2_deep0` record. A successor must verify exports/ABI rather than infer from prose.

## 18. Resume and failure behavior

Not started: verify baseline and deps. Partial/test failure: inspect git status, changed paths and durable test DB, resume only safe incomplete steps. Implemented but unintegrated: do not dispatch successors. Interrupted agent/rejected patch: preserve branch, rebase only after review, rerun tests. Specification conflict: `CONTRACT_BLOCKED`, record evidence, request authority resolution; never edit SSOT in this sprint.


---

Subagent budget: 256K tokens maximum; start with the bounded references above, do not read entire SSOT by default. CLI-first with verified tool compatibility. Treat Jev/TypeSafe as optional advisory only when available and authorized; never send private repository content externally without consent.
