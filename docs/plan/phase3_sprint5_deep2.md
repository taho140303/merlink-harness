---
sprint_id: phase3_sprint5_deep2
title: Implement write tools and protected cascade
phase: 3
sprint: 5
deep: 2
baseline_commit: 57a462efb953a665e3673b15e6cc33622a4f2d39
status: PLANNED
---

# phase3_sprint5_deep2 — Implement write tools and protected cascade

## 1. Sprint identity

Engineering phase 3 (Draft domain and durable workflow), within-phase depth 2; baseline is the Git commit above, not an uncommitted workspace. **PROPOSED, NOT IMPLEMENTED.**

## 2. Objective

Implement write tools and protected cascade. Observable handoff: Backend-owned typed tool outcomes, mutation evidence and full Q1 cascade implementation.

## 3. Scope boundaries

**IN_SCOPE:** Implement all mandated add/set/delete families from tool catalog; split internal modules only under exclusive ownership.; Table-driven fixtures cover each tool name, permissions, resulting State and complete cascade preview.

**OUT_OF_SCOPE / MUST NOT:** modify authoritative `docs/business/**`, `docs/db.dbml`, `docs/architecture/decisions/**`; use real application data; run production migration; add client request IDs to SF-06; invent business semantics; rewrite another sprint’s files.

## 4. Authoritative contract snapshot

- Backend validates declared_sources per field; caller identity not self-declared by AI; no direct protected overwrite.

- Cascade computes full transitive impact before atomic clone; no cross-project deletion, no implicit semantic merge.

- **REQ-DOM-004** `docs/business/core.md#L82-L109`: Q1 transitive cascade, full impact preview, protected descendants, atomic reject. (planning coverage `ASSIGNED`).

  Normative excerpt (verbatim baseline lines, truncated to 300 chars; use `sed` for the full range):

  > L82: / Thực thể bị xóa / Dữ liệu liên quan / Hành vi /

  > L83: / :--- / :--- / :--- /

  > L84: / Arc / Chapters thuộc Arc / Cascade (bắc cầu xuống Scenes và các phụ thuộc của Scene) /

  > L85: / Chapter / Scenes thuộc Chapter / Cascade (bắc cầu xuống các phụ thuộc của Scene) /

- **REQ-TOOLS-001** `docs/business/harness/tools.md#L9-L24`: Registry caller/source metadata; AI cannot call UI-only tools; field-level structured outcomes. (planning coverage `ASSIGNED`).

  Normative excerpt (verbatim baseline lines, truncated to 300 chars; use `sed` for the full range):

  > L9: ## 1. Cách đọc tài liệu

  > L11: * **Nhóm ưu tiên:** Nhóm 1 *Phải có* (chặn luồng Phase 0-2 và commit), Nhóm 2 *Cần, sẽ phát triển* (Phase 3-6 và workflow phân đoạn), Nhóm 3 *Nên có, làm sau cùng* (dự phòng, hoãn hoặc chưa có đặc tả).

  > L12: * **Client:** `AI` là tool được cấp cho model qua `prompt_block_tools`; `UI/CLI` là thao tác của người dùng, **không** cấp cho AI; `AI, UI` nghĩa là cùng một bề mặt tool, UI gọi với `source = user` (đúng triết lý CLI-first, core 1.4).

  > L13: * **Loại:** `Đọc` không tạo version; `Ghi` (`add_`, `set_`) và `Xóa` (`delete_`) chỉ góp vào version cuối lượt khi được `applied` (core 3.6).

- **REQ-TOOLS-002** `docs/business/harness/tools.md#L38-L110`: Required read/write tools and catalog retrieval across business phases. (planning coverage `ASSIGNED`).

  Normative excerpt (verbatim baseline lines, truncated to 300 chars; use `sed` for the full range):

  > L38: ## 3. Nhóm 1: Phải có

  > L40: ### 3.1 Tool đọc và tra cứu

  > L42: / Tool (tên ví dụ) / Client / Loại / Bảng DBML / Tham số (theo DBML) / Ghi chú /

  > L43: / :--- / :--- / :--- / :--- / :--- / :--- /

- **REQ-TOOLS-004** `docs/business/be-core.md#L781-L837`: Tool executor MUST enforce declared_sources/declared_source and protect user/user_edited fields per field atomically. (planning coverage `ASSIGNED`).

  Normative excerpt (verbatim baseline lines, truncated to 300 chars; use `sed` for the full range):

  > L781: ### 5.3 Kiểm tra Tham số, Cưỡng chế `declared_sources` / `declared_source` & Vòng đời Thực thi Tool

  > L783: Khi một tool ghi vào Draft State (`writes_draft_state == true`) được gọi, handler `MUST` thực thi theo quy trình 2 pha trên từng trường (Two-Phase Per-Field Execution — `core.md` §1.3, §1.4.7):

  > L785: #### 5.3.1 Quy tắc cho Tool Ghi Nhiều Trường (`SourceParamMode::MapDeclaredSources`)

  > L787: Áp dụng cho: `add_project`/`set_project`, `add_character`/`set_character`, `add_arc`/`set_arc`, `add_chapter`/`set_chapter`, `add_scene`/`set_scene`, `add_world_setting`/`set_world_setting`, `add_lore_entry`/`set_lore_entry`, `set_scene_config`, `add_theme`/`set_theme`, `add_character_appearance`/`s

- **REQ-TOOLS-004** `docs/business/be-core.md#L781-L837`: Tool executor MUST enforce declared_sources/declared_source and protect user/user_edited fields per field atomically. (planning coverage `ASSIGNED`).

  Normative excerpt (verbatim baseline lines, truncated to 300 chars; use `sed` for the full range):

  > L781: ### 5.3 Kiểm tra Tham số, Cưỡng chế `declared_sources` / `declared_source` & Vòng đời Thực thi Tool

  > L783: Khi một tool ghi vào Draft State (`writes_draft_state == true`) được gọi, handler `MUST` thực thi theo quy trình 2 pha trên từng trường (Two-Phase Per-Field Execution — `core.md` §1.3, §1.4.7):

  > L785: #### 5.3.1 Quy tắc cho Tool Ghi Nhiều Trường (`SourceParamMode::MapDeclaredSources`)

  > L787: Áp dụng cho: `add_project`/`set_project`, `add_character`/`set_character`, `add_arc`/`set_arc`, `add_chapter`/`set_chapter`, `add_scene`/`set_scene`, `add_world_setting`/`set_world_setting`, `add_lore_entry`/`set_lore_entry`, `set_scene_config`, `add_theme`/`set_theme`, `add_character_appearance`/`s

- **REQ-SPECTEST-T-CASC-01** `docs/business/be-core.md#L2534-L2534`: MUST implement executable BE acceptance vector T-CASC-01, preserving scenario and expected result; old v1-only recovery claims remain legacy-only under B+. (planning coverage `ASSIGNED`).

**Targeted source references (read only these ranges unless a test proves broader context is necessary):**

- `docs/business/core.md#L39-L109` — focus: Protected-field and Q1 cascade matrix; CLI: `sed -n "39,109p" docs/business/core.md` (or `rg -n "<symbol>" docs/business/core.md` for a changed anchor).

- `docs/business/harness/tools.md#L51-L60` — focus: Core write tools; CLI: `sed -n "51,60p" docs/business/harness/tools.md` (or `rg -n "<symbol>" docs/business/harness/tools.md` for a changed anchor).

- `docs/business/harness/tools.md#L87-L110` — focus: Phase3–6 write tools; CLI: `sed -n "87,110p" docs/business/harness/tools.md` (or `rg -n "<symbol>" docs/business/harness/tools.md` for a changed anchor).

- `docs/business/be-core.md#L781-L885` — focus: Per-field execution and cascade; CLI: `sed -n "781,885p" docs/business/be-core.md` (or `rg -n "<symbol>" docs/business/be-core.md` for a changed anchor).

## 5. Prerequisites

`depends_on: ["phase3_sprint4_deep1", "phase3_sprint1_deep0"]`.

- `phase3_sprint4_deep1` [RUNTIME_DEPENDENCY] produces Frozen ToolRegistry metadata and read tool schema/results; inspect its accepted commit, exported artifact/test and `gate_phase3_sprint4_deep1`. If absent, do not start.

- `phase3_sprint1_deep0` [RUNTIME_DEPENDENCY] produces plan_outline_runs/split_balanced/plan_phase and progress/validation pure APIs; inspect its accepted commit, exported artifact/test and `gate_phase3_sprint1_deep0`. If absent, do not start.

## 6. Inputs

Required: accepted outputs of all predecessors; frozen committed SSOT ranges above; actual source paths listed below. Optional context: existing UI mocks for fixtures only. No `report/` input; no historical research.

## 7. File ownership

```yaml

allowed_write_paths:

  - src-tauri/src/domain/tools/write_tools.rs

  - src-tauri/src/domain/cascade_delete.rs

  - src-tauri/tests/write_tools.rs

forbidden_write_paths:

  - docs/business/**

  - docs/db.dbml

  - docs/architecture/decisions/**

  - app-data/**

  - report/**

shared_read_only_paths:

  - docs/business/core.md

  - docs/business/harness/tools.md

  - docs/business/be-core.md

generated_paths: []

```

Any allowed path may be absent at baseline (not only `src-tauri/`; new FE adapter/shim files are also future targets). Verify with `test -e`/`git ls-files` before editing; existing files are read first. Any extra path requires central integration approval, not a silent scope expansion.

## 8. Public interfaces

PROPOSED_INTERFACE: ToolExecutor::execute(call,guard)->ToolOutcome {status,results,authenticated_effect}; CascadePreview; writes stay in WorkingState clone. Every consumed prerequisite interface is `PREREQUISITE_INTERFACE`, not an existing runtime guarantee. If a named signature conflicts with committed SSOT, stop `CONTRACT_BLOCKED`.

## 9. Implementation requirements

- Implement all mandated add/set/delete families from tool catalog; split internal modules only under exclusive ownership.

- Table-driven fixtures cover each tool name, permissions, resulting State and complete cascade preview.

- Prefer official compatible CLI/tooling over manual generated files; verify installed version and diff. No external LLM/Jev request without authorization; optional Jev advice never overrides SSOT.

## 10. Domain invariants

- Backend validates declared_sources per field; caller identity not self-declared by AI; no direct protected overwrite.

- Cascade computes full transitive impact before atomic clone; no cross-project deletion, no implicit semantic merge.

- No partial state escapes; protected provenance, explicit authorization, no external-effect recovery replay where applicable.

## 11. Failure matrix

| Trigger | Expected behavior / classification | Durable effects and recovery | Required test |

|---|---|---|---|

| Invalid field/source | reject that field, allow independent valid fields; P3S5-N. | No unauthorized partial commit; re-read actual durable state before retry | phase3_sprint5_deep2-F1 |

| Protected descendant | pending proposal request, no mutation; P3S5-C. | No unauthorized partial commit; re-read actual durable state before retry | phase3_sprint5_deep2-F2 |

| Missing accepted predecessor / baseline mismatch | `CONTRACT_BLOCKED`; do not run implementation | No user-data effects; record issue and owner in sprint evidence | phase3_sprint5_deep2-F0 |

## 12. Acceptance tests

Tests below name **future targets**, not tests executed during planning. Each row must become an executable test before the sprint can be accepted.

| Test ID | Initial state | Input / action | Expected output and state | Forbidden side effects / invariant |

|---|---|---|---|---|

| P3S5-P | pure Rust fixture (no DB/network/Tauri) + accepted predecessor types | In the owned test target, set up the fixture and run the scenario: mixed three-field tool returns applied/pending/rejected without corrupting State. | mixed three-field tool returns applied/pending/rejected without corrupting State. | No unrelated write; preserve exact expected state and listed invariant |

| P3S5-N | pure Rust fixture (no DB/network/Tauri) + accepted predecessor types | In the owned test target, set up the fixture and run the scenario: delete referenced world setting returns referenced_entity_in_use and unchanged State. | delete referenced world setting returns referenced_entity_in_use and unchanged State. | No unrelated write; preserve exact expected state and listed invariant |

| P3S5-B | pure Rust fixture (no DB/network/Tauri) + accepted predecessor types | In the owned test target, set up the fixture and run the scenario: scene cascade nulls memory ref and removes dependent links atomically. | scene cascade nulls memory ref and removes dependent links atomically. | No unrelated write; preserve exact expected state and listed invariant |

| phase3_sprint5_deep2-F1 | accepted prerequisite fixture / isolated worktree | Inject Invalid field/source into owned operation | reject that field, allow independent valid fields; P3S5-N. | No unauthorized commit or external replay |

| phase3_sprint5_deep2-F2 | accepted prerequisite fixture / isolated worktree | Inject Protected descendant into owned operation | pending proposal request, no mutation; P3S5-C. | No unauthorized commit or external replay |

| T-CASC-01 | disposable/synthetic fixture described by BE | (1) Xóa `world_setting` có nút con đang được `scene` tham chiếu; (2) Xóa `character` đang có `relationship`; (3) AI xóa `arc` (`ai_generated`) nhưng chứa 1 `scene` có `source = user`; (4) Xóa `scene` có `character_competences`, `character_memories`, `relationship_states`; (5) Xóa `lore_entry` có `scene_lore_entries`. | (1) & (2) Bị chặn với `status = "rejected", error = "referenced_entity_in_use"`, không xóa một phần. (3) Trả `pending_confirmation` kèm preview liệt kê đầy đủ toàn bộ chương/cảnh con. (4) Xóa vật lý `scene`, `character_competences`, `relationship_states` của cảnh đó (giữ `relationships` cha), và đặt `character_memories.source_scene_code = null`. (5) Xóa `lore_entry` và gỡ liên kết `scene_lore_entries` nguyên tử, giữ nguyên `scene`. | Only owner-authorized effects; old v1-only statements interpreted as legacy, no B+ regression |

| phase3_sprint5_deep2-F0 | predecessor gate absent or baseline SHA drift | Attempt dispatch before predecessor accepted | CONTRACT_BLOCKED; no implementation started | No source/DB modification |

Backend module wiring rule: this sprint must not edit any `mod.rs`/`lib.rs`. Its integration test loads each owned implementation file via `#[path = "..."] mod <name>;` and fixture modules via the same mechanism. Production registration occurs only in P6S2 after this sprint is accepted. A compile success is therefore scoped to this test target, not a claim that the whole crate is wired.

Future command after the target files exist: `cargo test --manifest-path src-tauri/Cargo.toml --test write_tools  # future target, after bootstrap`.

## 13. Database and migration safety

No persistent database in this sprint; fixture DB (if any) is disposable and uniquely named per worktree.

## 14. Concurrency contract

- Peer `phase3_sprint9_deep2`: candidate parallel only after frozen contracts/gates; NONE: Disjoint exact write paths; shared types frozen by predecessors; worktree-local test resources.

Isolation: own worktree/branch from baseline or accepted integration SHA; unique temp root, test DB and port; no shared mutable process/global fixture. Same depth alone is NOT permission to dispatch.

## 15. Deliverables

- [ ] Backend-owned typed tool outcomes, mutation evidence and full Q1 cascade implementation

- [ ] Tests above and evidence including command exit codes, scope diff and accepted commit SHA.

## 16. Completion criteria

`SPRINT_IMPLEMENTED` requires artifacts, `SPRINT_TESTED` requires executed acceptance tests, `SPRINT_INTEGRATED` requires merged checks, and **`SPRINT_ACCEPTED` requires `gate_phase3_sprint5_deep2` accepted**. A contract-evidence sprint cannot be ACCEPTED for an affected implementation successor while its owner decision is unresolved. No agent declaration alone unlocks successors. All required assertions, file scope and baseline must pass.

## 17. Handoff contract

- Backend-owned typed tool outcomes, mutation evidence and full Q1 cascade implementation

- Exact artifacts: src-tauri/src/domain/tools/write_tools.rs, src-tauri/src/domain/cascade_delete.rs, src-tauri/tests/write_tools.rs; accepted integration SHA and `gate_phase3_sprint5_deep2` record. A successor must verify exports/ABI rather than infer from prose.

## 18. Resume and failure behavior

Not started: verify baseline and deps. Partial/test failure: inspect git status, changed paths and durable test DB, resume only safe incomplete steps. Implemented but unintegrated: do not dispatch successors. Interrupted agent/rejected patch: preserve branch, rebase only after review, rerun tests. Specification conflict: `CONTRACT_BLOCKED`, record evidence, request authority resolution; never edit SSOT in this sprint.


---

Subagent budget: 256K tokens maximum; start with the bounded references above, do not read entire SSOT by default. CLI-first with verified tool compatibility. Treat Jev/TypeSafe as optional advisory only when available and authorized; never send private repository content externally without consent.
