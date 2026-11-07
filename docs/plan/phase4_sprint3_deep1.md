---
sprint_id: phase4_sprint3_deep1
title: Expose project and character library services
phase: 4
sprint: 3
deep: 1
baseline_commit: 57a462efb953a665e3673b15e6cc33622a4f2d39
status: PLANNED
---

# phase4_sprint3_deep1 — Expose project and character library services

## 1. Sprint identity

Engineering phase 4 (Backend services and Writing Mode), within-phase depth 1; baseline is the Git commit above, not an uncommitted workspace. **PROPOSED, NOT IMPLEMENTED.**

## 2. Objective

Expose project and character library services. Observable handoff: Project/Character services, repos and IPC #22–28.

## 3. Scope boundaries

**IN_SCOPE:** Implement 7 project/character commands in dedicated files; no Writing component/UI edits.; Test cross-project reuse, typed read models, soft-delete and unauthorized linkage.

**OUT_OF_SCOPE / MUST NOT:** modify authoritative `docs/business/**`, `docs/db.dbml`, `docs/architecture/decisions/**`; use real application data; run production migration; add client request IDs to SF-06; invent business semantics; rewrite another sprint’s files.

## 4. Authoritative contract snapshot

- Project delete is scoped and does not destroy shared characters/world templates.

- Character search/link respects project ownership and stable code; no mock IDs persisted.

- **REQ-PROJECT-001** `docs/business/be-core.md#L2214-L2229`: Project/character library read/write, cross-project sharing and safe delete. (planning coverage `ASSIGNED`).

  Normative excerpt (verbatim baseline lines, truncated to 300 chars; use `sed` for the full range):

  > L2214: #### 10.2.7 Nhóm 7: Projects & Creator Mode (`ipc::project_commands` — `H3 APPROVED`)

  > L2216: / Command Name / Request Type / Response Type / Validation & Authorization / Transaction Behavior / Error Codes / Cancellation Behavior /

  > L2217: / :--- / :--- / :--- / :--- / :--- / :--- / :--- /

  > L2218: / **`list_projects`** / `{ query?: String, sort?: "a-z" \/ "z-a" \/ "modified" }` / `ProjectListDto { items: ProjectItemUiDto[] }` / Chuẩn hóa NFC cho `query`. Ánh xạ sang đúng cấu trúc `ProjectItem` của FE (`src/features/projects/types.ts`). / Read-only (`SELECT` trên `projects` `WHERE deleted_at I

- **REQ-CMD-022** `docs/business/be-core.md#L2139-L2139`: MUST implement typed IPC #22 `list_projects` with the specified authorization, DTO, transaction and error behavior. (planning coverage `ASSIGNED`).

- **REQ-CMD-023** `docs/business/be-core.md#L2140-L2140`: MUST implement typed IPC #23 `get_project_detail` with the specified authorization, DTO, transaction and error behavior. (planning coverage `ASSIGNED`).

- **REQ-CMD-024** `docs/business/be-core.md#L2141-L2141`: MUST implement typed IPC #24 `delete_project` with the specified authorization, DTO, transaction and error behavior. (planning coverage `ASSIGNED`).

- **REQ-CMD-025** `docs/business/be-core.md#L2142-L2142`: MUST implement typed IPC #25 `list_characters` with the specified authorization, DTO, transaction and error behavior. (planning coverage `ASSIGNED`).

- **REQ-CMD-026** `docs/business/be-core.md#L2143-L2143`: MUST implement typed IPC #26 `get_character_detail` with the specified authorization, DTO, transaction and error behavior. (planning coverage `ASSIGNED`).

- **REQ-CMD-027** `docs/business/be-core.md#L2144-L2144`: MUST implement typed IPC #27 `create_or_update_library_character` with the specified authorization, DTO, transaction and error behavior. (planning coverage `ASSIGNED`).

- **REQ-CMD-028** `docs/business/be-core.md#L2145-L2145`: MUST implement typed IPC #28 `link_library_character_to_draft` with the specified authorization, DTO, transaction and error behavior. (planning coverage `ASSIGNED`).

**Targeted source references (read only these ranges unless a test proves broader context is necessary):**

- `docs/business/be-core.md#L2214-L2229` — focus: IPC #22–28 and data access; CLI: `sed -n "2214,2229p" docs/business/be-core.md` (or `rg -n "<symbol>" docs/business/be-core.md` for a changed anchor).

- `docs/business/core.md#L791-L823` — focus: Character search and shared links; CLI: `sed -n "791,823p" docs/business/core.md` (or `rg -n "<symbol>" docs/business/core.md` for a changed anchor).

- `docs/business/fe-core.md#L442-L481` — focus: Character/project UI contract; CLI: `sed -n "442,481p" docs/business/fe-core.md` (or `rg -n "<symbol>" docs/business/fe-core.md` for a changed anchor).

**Owned IPC command contracts (verbatim narrow BE §10.2 rows; proposed, not currently implemented):**

- IPC-22 `list_projects` — `docs/business/be-core.md#L2218`: **`list_projects`**  ·  `{ query?: String, sort?: "a-z" \ ·  "z-a" \ ·  "modified" }`  ·  `ProjectListDto { items: ProjectItemUiDto[] }`  ·  Chuẩn hóa NFC cho `query`. Ánh xạ sang đúng cấu trúc `ProjectItem` của FE (`src/features/projects/types.ts`).  ·  Read-only (`SELECT` trên `projects` `WHERE deleted_at IS NULL` kèm đếm chương, từ, nhân vật, lore).  ·  `DatabaseError`  ·  Không áp dụng.

- IPC-23 `get_project_detail` — `docs/business/be-core.md#L2219`: **`get_project_detail`**  ·  `{ projectId: Uuid }`  ·  `ProjectDetailDto { project: ProjectItemUiDto, hasCommittedPlan: bool, draftSessionId?: Uuid, canReopenDraft: bool }`  ·  Xác minh `projectId` tồn tại và `deleted_at IS NULL`. Cung cấp danh sách `arcs[].chapters[]` kèm số cảnh của từng chương cho cả `view` mode và `creator` mode (`fe-core.md` WR-02).  ·  Read-only.  ·  `EntityNotFound`, `DatabaseError`  ·  Không áp dụng.

- IPC-24 `delete_project` — `docs/business/be-core.md#L2220`: **`delete_project`**  ·  `{ projectId: Uuid }`  ·  `{ deleted: bool }`  ·  Xác minh `projectId` tồn tại; bảo vệ không xóa thực thể thư viện dùng chung (`characters`, template `world_settings` có `project_id IS NULL`) đang được project khác tham chiếu (`core.md` §1.3 `Q1`).  ·  `BEGIN IMMEDIATE`: cập nhật `deleted_at = now()` trên `projects`, `arcs`, `chapters`, `scenes`, `project_characters`, và `world_settings` thuộc riêng project đó.  ·  `EntityNotFound`, `DatabaseError`  ·  Thực thi cục bộ nguyên tử.

- IPC-25 `list_characters` — `docs/business/be-core.md#L2226`: **`list_characters`**  ·  `{ projectId?: Uuid, biologicalSex?: BiologicalSex, query?: String }`  ·  `CharacterListDto { items: CharacterLibraryItemDto[] }`  ·  Hỗ trợ thuật toán tìm kiếm 4 tầng theo `core.md` §3.8: chuẩn hóa NFC không phân biệt hoa thường $\rightarrow$ khớp chính xác $\rightarrow$ tiền tố/chuỗi con $\rightarrow$ bỏ dấu tiếng Việt, kèm cờ `inProject`.  ·  Read-only (`SELECT` trên `characters` kết hợp `project_characters`).  ·  `DatabaseError`  ·  Không áp dụng.

- IPC-26 `get_character_detail` — `docs/business/be-core.md#L2227`: **`get_character_detail`**  ·  `{ characterId: Uuid, projectId?: Uuid }`  ·  `CharacterDetailDto { character, appearance, distinguishingMarks[], psychological, memories[], competences[], relationships[] }`  ·  Xác minh `characterId` tồn tại. Ánh xạ đầy đủ 3 tab `core`, `life`, `psych` cho `CharacterDetailLayout` (`fe-core.md` §10).  ·  Read-only.  ·  `EntityNotFound`, `DatabaseError`  ·  Không áp dụng.

- IPC-27 `create_or_update_library_character` — `docs/business/be-core.md#L2228`: **`create_or_update_library_character`**  ·  `{ characterId?: Uuid, projectId?: Uuid, profile: CharacterUpsertPayloadDto }`  ·  `CharacterDetailDto`  ·  Kiểm tra bất biến `display_name IS NOT NULL OR nickname IS NOT NULL`; kiểm tra `character_competences.scene_id NOT NULL` nếu thêm năng lực gắn cảnh.  ·  `BEGIN IMMEDIATE`: ghi vào `characters`, `project_characters`, `character_appearances`, `character_psychological`, `character_memories`, `character_competences`.  ·  `ValidationFailed`, `EntityNotFound`, `DatabaseError`  ·  Thực thi cục bộ nguyên tử.

- IPC-28 `link_library_character_to_draft` — `docs/business/be-core.md#L2229`: **`link_library_character_to_draft`**  ·  `{ draftSessionId: Uuid, characterId: Uuid, role?: String, notes?: String }`  ·  `{ characterCode: String, sealedVersion: DraftVersionSummaryDto }`  ·  Xác minh `characterId` tồn tại trong thư viện `characters`; đưa nhân vật vào `WorkingState` của bản nháp với cờ tái sử dụng mà không tạo bản sao (`core.md` §3.8, `fe-core.md` CHR-04).  ·  Lấy `operation_lock`; thực thi **`TX-01`**.  ·  `DraftBusy`, `EntityNotFound`, `VersionSealFailed`  ·  Thực thi cục bộ nguyên tử.

## 5. Prerequisites

`depends_on: ["phase4_sprint2_deep0", "phase3_sprint2_deep0"]`.

- `phase4_sprint2_deep0` [RUNTIME_DEPENDENCY] produces CommitService/CommitRepo, preview/commit/reopen IPC #19–21; inspect its accepted commit, exported artifact/test and `gate_phase4_sprint2_deep0`. If absent, do not start.

- `phase3_sprint2_deep0` [INTEGRATION_DEPENDENCY] produces DraftService, DraftRuntimeRegistry guard, draft/version IPC #5–8,#10–11, TX01/03 fixtures; inspect its accepted commit, exported artifact/test and `gate_phase3_sprint2_deep0`. If absent, do not start.

## 6. Inputs

Required: accepted outputs of all predecessors; frozen committed SSOT ranges above; actual source paths listed below. Optional context: existing UI mocks for fixtures only. No `report/` input; no historical research.

## 7. File ownership

```yaml

allowed_write_paths:

  - src-tauri/src/storage/project_repo.rs

  - src-tauri/src/storage/character_repo.rs

  - src-tauri/src/service/project_service.rs

  - src-tauri/src/service/character_service.rs

  - src-tauri/src/ipc/project_commands.rs

  - src-tauri/src/ipc/character_commands.rs

  - src-tauri/tests/catalog_services.rs

forbidden_write_paths:

  - docs/business/**

  - docs/db.dbml

  - docs/architecture/decisions/**

  - app-data/**

  - report/**

shared_read_only_paths:

  - docs/business/be-core.md

  - docs/business/core.md

  - docs/business/fe-core.md

generated_paths: []

```

Any allowed path may be absent at baseline (not only `src-tauri/`; new FE adapter/shim files are also future targets). Verify with `test -e`/`git ls-files` before editing; existing files are read first. Any extra path requires central integration approval, not a silent scope expansion.

## 8. Public interfaces

PROPOSED_INTERFACE: ProjectService and CharacterService, IPC #22–28, stable read models. Every consumed prerequisite interface is `PREREQUISITE_INTERFACE`, not an existing runtime guarantee. If a named signature conflicts with committed SSOT, stop `CONTRACT_BLOCKED`.

## 9. Implementation requirements

- Implement 7 project/character commands in dedicated files; no Writing component/UI edits.

- Test cross-project reuse, typed read models, soft-delete and unauthorized linkage.

- Prefer official compatible CLI/tooling over manual generated files; verify installed version and diff. No external LLM/Jev request without authorization; optional Jev advice never overrides SSOT.

## 10. Domain invariants

- Project delete is scoped and does not destroy shared characters/world templates.

- Character search/link respects project ownership and stable code; no mock IDs persisted.

- No partial state escapes; protected provenance, explicit authorization, no external-effect recovery replay where applicable.

## 11. Failure matrix

| Trigger | Expected behavior / classification | Durable effects and recovery | Required test |

|---|---|---|---|

| Cross-project shared reference | preserve shared row, not cascade; P4S3-N. | No unauthorized partial commit; re-read actual durable state before retry | phase4_sprint3_deep1-F1 |

| Unknown character/project | EntityNotFound and zero writes; P4S3-C. | No unauthorized partial commit; re-read actual durable state before retry | phase4_sprint3_deep1-F2 |

| Missing accepted predecessor / baseline mismatch | `CONTRACT_BLOCKED`; do not run implementation | No user-data effects; record issue and owner in sprint evidence | phase4_sprint3_deep1-F0 |

## 12. Acceptance tests

Tests below name **future targets**, not tests executed during planning. Each row must become an executable test before the sprint can be accepted.

| Test ID | Initial state | Input / action | Expected output and state | Forbidden side effects / invariant |

|---|---|---|---|---|

| P4S3-P | disposable uniquely-named SQLite S0 + accepted predecessor fixture | In the owned test target, set up the fixture and run the scenario: list/get/delete project + 4 character commands respond from disposable DB. | list/get/delete project + 4 character commands respond from disposable DB. | No unrelated write; preserve exact expected state and listed invariant |

| P4S3-N | disposable uniquely-named SQLite S0 + accepted predecessor fixture | In the owned test target, set up the fixture and run the scenario: deleting project leaves shared character/template intact. | deleting project leaves shared character/template intact. | No unrelated write; preserve exact expected state and listed invariant |

| P4S3-B | disposable uniquely-named SQLite S0 + accepted predecessor fixture | In the owned test target, set up the fixture and run the scenario: link existing character avoids duplicate library row. | link existing character avoids duplicate library row. | No unrelated write; preserve exact expected state and listed invariant |

| phase4_sprint3_deep1-F1 | accepted prerequisite fixture / isolated worktree | Inject Cross-project shared reference into owned operation | preserve shared row, not cascade; P4S3-N. | No unauthorized commit or external replay |

| phase4_sprint3_deep1-F2 | accepted prerequisite fixture / isolated worktree | Inject Unknown character/project into owned operation | EntityNotFound and zero writes; P4S3-C. | No unauthorized commit or external replay |

| IPC-CMD-022 | isolated authorized and unauthorized caller fixtures | Invoke `list_projects` with valid request and invalid/stale variant; use frozen DTO | /  **`list_projects`**  /  `{ query?: String, sort?: "a-z" \ /  "z-a" \ /  "modified" }`  /  `ProjectListDto { items: ProjectItemUiDto[] }`  /  Chuẩn hóa NFC cho `query`. Ánh xạ sang đúng cấu trúc `ProjectItem` của FE (`src/features/projects/types.ts`).  /  Read-only (`SELECT` trên `projects` `WHERE deleted_at IS NULL` kèm đếm chương, từ, nhân vật, lore).  /  `DatabaseError`  /  Không áp dụng.  / | No unlisted command, no unauthorized write; assert event/error/transaction result |

| IPC-CMD-023 | isolated authorized and unauthorized caller fixtures | Invoke `get_project_detail` with valid request and invalid/stale variant; use frozen DTO | /  **`get_project_detail`**  /  `{ projectId: Uuid }`  /  `ProjectDetailDto { project: ProjectItemUiDto, hasCommittedPlan: bool, draftSessionId?: Uuid, canReopenDraft: bool }`  /  Xác minh `projectId` tồn tại và `deleted_at IS NULL`. Cung cấp danh sách `arcs[].chapters[]` kèm số cảnh của từng chương cho cả `view` mode và `creator` mode (`fe-core.md` WR-02).  /  Read-only.  /  `EntityNotFound`, `DatabaseError`  /  Không áp dụng.  / | No unlisted command, no unauthorized write; assert event/error/transaction result |

| IPC-CMD-024 | isolated authorized and unauthorized caller fixtures | Invoke `delete_project` with valid request and invalid/stale variant; use frozen DTO | /  **`delete_project`**  /  `{ projectId: Uuid }`  /  `{ deleted: bool }`  /  Xác minh `projectId` tồn tại; bảo vệ không xóa thực thể thư viện dùng chung (`characters`, template `world_settings` có `project_id IS NULL`) đang được project khác tham chiếu (`core.md` §1.3 `Q1`).  /  `BEGIN IMMEDIATE`: cập nhật `deleted_at = now()` trên `projects`, `arcs`, `chapters`, `scenes`, `project_characters`, và `world_settings` thuộc riêng project đó.  /  `EntityNotFound`, `DatabaseError`  /  Thực thi cục bộ nguyên tử.  / | No unlisted command, no unauthorized write; assert event/error/transaction result |

| IPC-CMD-025 | isolated authorized and unauthorized caller fixtures | Invoke `list_characters` with valid request and invalid/stale variant; use frozen DTO | /  **`list_characters`**  /  `{ projectId?: Uuid, biologicalSex?: BiologicalSex, query?: String }`  /  `CharacterListDto { items: CharacterLibraryItemDto[] }`  /  Hỗ trợ thuật toán tìm kiếm 4 tầng theo `core.md` §3.8: chuẩn hóa NFC không phân biệt hoa thường $\rightarrow$ khớp chính xác $\rightarrow$ tiền tố/chuỗi con $\rightarrow$ bỏ dấu tiếng Việt, kèm cờ `inProject`.  /  Read-only (`SELECT` trên `characters` kết hợp `project_characters`).  /  `DatabaseError`  /  Không áp dụng.  / | No unlisted command, no unauthorized write; assert event/error/transaction result |

| IPC-CMD-026 | isolated authorized and unauthorized caller fixtures | Invoke `get_character_detail` with valid request and invalid/stale variant; use frozen DTO | /  **`get_character_detail`**  /  `{ characterId: Uuid, projectId?: Uuid }`  /  `CharacterDetailDto { character, appearance, distinguishingMarks[], psychological, memories[], competences[], relationships[] }`  /  Xác minh `characterId` tồn tại. Ánh xạ đầy đủ 3 tab `core`, `life`, `psych` cho `CharacterDetailLayout` (`fe-core.md` §10).  /  Read-only.  /  `EntityNotFound`, `DatabaseError`  /  Không áp dụng.  / | No unlisted command, no unauthorized write; assert event/error/transaction result |

| IPC-CMD-027 | isolated authorized and unauthorized caller fixtures | Invoke `create_or_update_library_character` with valid request and invalid/stale variant; use frozen DTO | /  **`create_or_update_library_character`**  /  `{ characterId?: Uuid, projectId?: Uuid, profile: CharacterUpsertPayloadDto }`  /  `CharacterDetailDto`  /  Kiểm tra bất biến `display_name IS NOT NULL OR nickname IS NOT NULL`; kiểm tra `character_competences.scene_id NOT NULL` nếu thêm năng lực gắn cảnh.  /  `BEGIN IMMEDIATE`: ghi vào `characters`, `project_characters`, `character_appearances`, `character_psychological`, `character_memories`, `character_competences`.  /  `ValidationFailed`, `EntityNotFound`, `DatabaseError`  /  Thực thi cục bộ nguyên tử.  / | No unlisted command, no unauthorized write; assert event/error/transaction result |

| IPC-CMD-028 | isolated authorized and unauthorized caller fixtures | Invoke `link_library_character_to_draft` with valid request and invalid/stale variant; use frozen DTO | /  **`link_library_character_to_draft`**  /  `{ draftSessionId: Uuid, characterId: Uuid, role?: String, notes?: String }`  /  `{ characterCode: String, sealedVersion: DraftVersionSummaryDto }`  /  Xác minh `characterId` tồn tại trong thư viện `characters`; đưa nhân vật vào `WorkingState` của bản nháp với cờ tái sử dụng mà không tạo bản sao (`core.md` §3.8, `fe-core.md` CHR-04).  /  Lấy `operation_lock`; thực thi **`TX-01`**.  /  `DraftBusy`, `EntityNotFound`, `VersionSealFailed`  /  Thực thi cục bộ nguyên tử.  / | No unlisted command, no unauthorized write; assert event/error/transaction result |

| phase4_sprint3_deep1-F0 | predecessor gate absent or baseline SHA drift | Attempt dispatch before predecessor accepted | CONTRACT_BLOCKED; no implementation started | No source/DB modification |

Backend module wiring rule: this sprint must not edit any `mod.rs`/`lib.rs`. Its integration test loads each owned implementation file via `#[path = "..."] mod <name>;` and fixture modules via the same mechanism. Production registration occurs only in P6S2 after this sprint is accepted. A compile success is therefore scoped to this test target, not a claim that the whole crate is wired.

Future command after the target files exist: `cargo test --manifest-path src-tauri/Cargo.toml --test catalog_services  # future target, after bootstrap`.

## 13. Database and migration safety

Use `BEGIN IMMEDIATE` only for the assigned TX; WAL/FULL/FK ON/busy_timeout as specified. Use unique disposable DB/temp root per worktree. Never open a user application DB; no destructive migration without separate authorization. Backup/rollback and PRAGMA user_version are owned by P2S3. TX04 never rolls back committed TX08.

## 14. Concurrency contract

- Peer `phase4_sprint4_deep1`: candidate parallel only after frozen contracts/gates; NONE: Disjoint exact write paths; shared types frozen by predecessors; worktree-local test resources.

Isolation: own worktree/branch from baseline or accepted integration SHA; unique temp root, test DB and port; no shared mutable process/global fixture. Same depth alone is NOT permission to dispatch.

## 15. Deliverables

- [ ] Project/Character services, repos and IPC #22–28

- [ ] Tests above and evidence including command exit codes, scope diff and accepted commit SHA.

## 16. Completion criteria

`SPRINT_IMPLEMENTED` requires artifacts, `SPRINT_TESTED` requires executed acceptance tests, `SPRINT_INTEGRATED` requires merged checks, and **`SPRINT_ACCEPTED` requires `gate_phase4_sprint3_deep1` accepted**. A contract-evidence sprint cannot be ACCEPTED for an affected implementation successor while its owner decision is unresolved. No agent declaration alone unlocks successors. All required assertions, file scope and baseline must pass.

## 17. Handoff contract

- Project/Character services, repos and IPC #22–28

- Exact artifacts: src-tauri/src/storage/project_repo.rs, src-tauri/src/storage/character_repo.rs, src-tauri/src/service/project_service.rs, src-tauri/src/service/character_service.rs, src-tauri/src/ipc/project_commands.rs, src-tauri/src/ipc/character_commands.rs, src-tauri/tests/catalog_services.rs; accepted integration SHA and `gate_phase4_sprint3_deep1` record. A successor must verify exports/ABI rather than infer from prose.

## 18. Resume and failure behavior

Not started: verify baseline and deps. Partial/test failure: inspect git status, changed paths and durable test DB, resume only safe incomplete steps. Implemented but unintegrated: do not dispatch successors. Interrupted agent/rejected patch: preserve branch, rebase only after review, rerun tests. Specification conflict: `CONTRACT_BLOCKED`, record evidence, request authority resolution; never edit SSOT in this sprint.


---

Subagent budget: 256K tokens maximum; start with the bounded references above, do not read entire SSOT by default. CLI-first with verified tool compatibility. Treat Jev/TypeSafe as optional advisory only when available and authorized; never send private repository content externally without consent.
