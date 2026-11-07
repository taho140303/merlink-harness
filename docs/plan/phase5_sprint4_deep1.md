---
sprint_id: phase5_sprint4_deep1
title: Integrate project and character read models
phase: 5
sprint: 4
deep: 1
baseline_commit: 57a462efb953a665e3673b15e6cc33622a4f2d39
status: PLANNED
---

# phase5_sprint4_deep1 — Integrate project and character read models

## 1. Sprint identity

Engineering phase 5 (Frontend integration), within-phase depth 1; baseline is the Git commit above, not an uncommitted workspace. **PROPOSED, NOT IMPLEMENTED.**

## 2. Objective

Integrate project and character read models. Observable handoff: Project/character FE read-model integration and tests.

## 3. Scope boundaries

**IN_SCOPE:** Replace feature mocks through transport for project list/detail/character views; keep Writing ChapterDetailView untouched.; Test project delete, character link and no raw slug exposure.

**OUT_OF_SCOPE / MUST NOT:** modify authoritative `docs/business/**`, `docs/db.dbml`, `docs/architecture/decisions/**`; use real application data; run production migration; add client request IDs to SF-06; invent business semantics; rewrite another sprint’s files.

## 4. Authoritative contract snapshot

- Render real project/character data without changing route/layout; stable internal code not user label.

- Shared library character reuse via link command, no cross-project destructive delete.

- **REQ-FE-004** `docs/business/fe-core.md#L442-L481`: Project/character data, source, cascade, no visible code slugs. (planning coverage `ASSIGNED`).

  Normative excerpt (verbatim baseline lines, truncated to 300 chars; use `sed` for the full range):

  > L442: ## 10. Character Library, Memory và Competence

  > L444: ### 10.1 Bảng hành vi

  > L446: / ID / Trạng thái / Quy tắc UI / Dữ liệu/hành vi / Nguồn Core / Nguồn FE / Ghi chú /

  > L447: / :--- / :--- / :--- / :--- / :--- / :--- / :--- /

**Targeted source references (read only these ranges unless a test proves broader context is necessary):**

- `docs/business/fe-core.md#L442-L481` — focus: Character/source/cascade and hidden code; CLI: `sed -n "442,481p" docs/business/fe-core.md` (or `rg -n "<symbol>" docs/business/fe-core.md` for a changed anchor).

- `docs/business/fe-core.md#L82-L102` — focus: Existing routes; CLI: `sed -n "82,102p" docs/business/fe-core.md` (or `rg -n "<symbol>" docs/business/fe-core.md` for a changed anchor).

- `docs/business/be-core.md#L2214-L2229` — focus: Project/character DTOs; CLI: `sed -n "2214,2229p" docs/business/be-core.md` (or `rg -n "<symbol>" docs/business/be-core.md` for a changed anchor).

- `docs/design-system.md#L19-L48` — focus: Two-tier surfaces; CLI: `sed -n "19,48p" docs/design-system.md` (or `rg -n "<symbol>" docs/design-system.md` for a changed anchor).

## 5. Prerequisites

`depends_on: ["phase5_sprint1_deep0", "phase4_sprint3_deep1"]`.

- `phase5_sprint1_deep0` [TYPE_DEPENDENCY] produces Typed backend client/adapter and FE fixture tests; inspect its accepted commit, exported artifact/test and `gate_phase5_sprint1_deep0`. If absent, do not start.

- `phase4_sprint3_deep1` [INTEGRATION_DEPENDENCY] produces Project/Character services, repos and IPC #22–28; inspect its accepted commit, exported artifact/test and `gate_phase4_sprint3_deep1`. If absent, do not start.

## 6. Inputs

Required: accepted outputs of all predecessors; frozen committed SSOT ranges above; actual source paths listed below. Optional context: existing UI mocks for fixtures only. No `report/` input; no historical research.

## 7. File ownership

```yaml

allowed_write_paths:

  - src/features/projects/ProjectView.tsx

  - src/features/projects/ProjectDetailView.tsx

  - src/features/projects/ProjectCreatorPanel.tsx

  - src/features/characters/CharacterListView.tsx

  - src/features/characters/CharacterDetailLayout.tsx

  - src/features/projects/projectBackend.test.mjs

forbidden_write_paths:

  - docs/business/**

  - docs/db.dbml

  - docs/architecture/decisions/**

  - app-data/**

  - report/**

shared_read_only_paths:

  - docs/business/fe-core.md

  - docs/business/be-core.md

  - docs/design-system.md

generated_paths: []

```

Any allowed path may be absent at baseline (not only `src-tauri/`; new FE adapter/shim files are also future targets). Verify with `test -e`/`git ls-files` before editing; existing files are read first. Any extra path requires central integration approval, not a silent scope expansion.

## 8. Public interfaces

PROPOSED_INTERFACE: Project/character FE read-model integration and tests. Every consumed prerequisite interface is `PREREQUISITE_INTERFACE`, not an existing runtime guarantee. If a named signature conflicts with committed SSOT, stop `CONTRACT_BLOCKED`.

## 9. Implementation requirements

- Replace feature mocks through transport for project list/detail/character views; keep Writing ChapterDetailView untouched.

- Test project delete, character link and no raw slug exposure.

- Prefer official compatible CLI/tooling over manual generated files; verify installed version and diff. No external LLM/Jev request without authorization; optional Jev advice never overrides SSOT.

## 10. Domain invariants

- Render real project/character data without changing route/layout; stable internal code not user label.

- Shared library character reuse via link command, no cross-project destructive delete.

- No partial state escapes; protected provenance, explicit authorization, no external-effect recovery replay where applicable.

## 11. Failure matrix

| Trigger | Expected behavior / classification | Durable effects and recovery | Required test |

|---|---|---|---|

| Absent project | not-found, not mock fallback; P5S4-N. | No unauthorized partial commit; re-read actual durable state before retry | phase5_sprint4_deep1-F1 |

| Link failure | preserve existing library rows/state; P5S4-C. | No unauthorized partial commit; re-read actual durable state before retry | phase5_sprint4_deep1-F2 |

| Missing accepted predecessor / baseline mismatch | `CONTRACT_BLOCKED`; do not run implementation | No user-data effects; record issue and owner in sprint evidence | phase5_sprint4_deep1-F0 |

## 12. Acceptance tests

Tests below name **future targets**, not tests executed during planning. Each row must become an executable test before the sprint can be accepted.

| Test ID | Initial state | Input / action | Expected output and state | Forbidden side effects / invariant |

|---|---|---|---|---|

| P5S4-P | accepted backend gate + fake typed Tauri transport fixture | In the owned test target, set up the fixture and run the scenario: project list/read model and character library render from fake IPC. | project list/read model and character library render from fake IPC. | No unrelated write; preserve exact expected state and listed invariant |

| P5S4-N | accepted backend gate + fake typed Tauri transport fixture | In the owned test target, set up the fixture and run the scenario: deleted project absent but shared character preserved. | deleted project absent but shared character preserved. | No unrelated write; preserve exact expected state and listed invariant |

| P5S4-B | accepted backend gate + fake typed Tauri transport fixture | In the owned test target, set up the fixture and run the scenario: no visible entity code in rendered snapshot. | no visible entity code in rendered snapshot. | No unrelated write; preserve exact expected state and listed invariant |

| phase5_sprint4_deep1-F1 | accepted prerequisite fixture / isolated worktree | Inject Absent project into owned operation | not-found, not mock fallback; P5S4-N. | No unauthorized commit or external replay |

| phase5_sprint4_deep1-F2 | accepted prerequisite fixture / isolated worktree | Inject Link failure into owned operation | preserve existing library rows/state; P5S4-C. | No unauthorized commit or external replay |

| phase5_sprint4_deep1-F0 | predecessor gate absent or baseline SHA drift | Attempt dispatch before predecessor accepted | CONTRACT_BLOCKED; no implementation started | No source/DB modification |

Future command after the target files exist: `pnpm test && pnpm typecheck && pnpm lint && pnpm build  # future target, after dependencies exist`. For FE changes, visual comparison at 1280/1279px and each declared viewport is mandatory; if tooling is unavailable the sprint is `BLOCKED_VISUAL_VERIFICATION` and cannot be accepted. `VISUAL_NOT_VERIFIED` is not a waiver.

## 13. Database and migration safety

No persistent database in this sprint; fixture DB (if any) is disposable and uniquely named per worktree.

## 14. Concurrency contract

- Peer `phase5_sprint2_deep1`: candidate parallel only after frozen contracts/gates; NONE: Disjoint exact write paths; shared types frozen by predecessors; worktree-local test resources.

- Peer `phase5_sprint6_deep1`: candidate parallel only after frozen contracts/gates; NONE: Disjoint exact write paths; shared types frozen by predecessors; worktree-local test resources.

Isolation: own worktree/branch from baseline or accepted integration SHA; unique temp root, test DB and port; no shared mutable process/global fixture. Same depth alone is NOT permission to dispatch.

## 15. Deliverables

- [ ] Project/character FE read-model integration and tests

- [ ] Tests above and evidence including command exit codes, scope diff and accepted commit SHA.

## 16. Completion criteria

`SPRINT_IMPLEMENTED` requires artifacts, `SPRINT_TESTED` requires executed acceptance tests, `SPRINT_INTEGRATED` requires merged checks, and **`SPRINT_ACCEPTED` requires `gate_phase5_sprint4_deep1` accepted**. A contract-evidence sprint cannot be ACCEPTED for an affected implementation successor while its owner decision is unresolved. No agent declaration alone unlocks successors. All required assertions, file scope and baseline must pass.

## 17. Handoff contract

- Project/character FE read-model integration and tests

- Exact artifacts: src/features/projects/ProjectView.tsx, src/features/projects/ProjectDetailView.tsx, src/features/projects/ProjectCreatorPanel.tsx, src/features/characters/CharacterListView.tsx, src/features/characters/CharacterDetailLayout.tsx, src/features/projects/projectBackend.test.mjs; accepted integration SHA and `gate_phase5_sprint4_deep1` record. A successor must verify exports/ABI rather than infer from prose.

## 18. Resume and failure behavior

Not started: verify baseline and deps. Partial/test failure: inspect git status, changed paths and durable test DB, resume only safe incomplete steps. Implemented but unintegrated: do not dispatch successors. Interrupted agent/rejected patch: preserve branch, rebase only after review, rerun tests. Specification conflict: `CONTRACT_BLOCKED`, record evidence, request authority resolution; never edit SSOT in this sprint.


---

Subagent budget: 256K tokens maximum; start with the bounded references above, do not read entire SSOT by default. CLI-first with verified tool compatibility. Treat Jev/TypeSafe as optional advisory only when available and authorized; never send private repository content externally without consent.
