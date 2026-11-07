---
sprint_id: phase5_sprint5_deep3
title: Integrate chapter Writing UI with durable services
phase: 5
sprint: 5
deep: 3
baseline_commit: 57a462efb953a665e3673b15e6cc33622a4f2d39
status: PLANNED
---

# phase5_sprint5_deep3 — Integrate chapter Writing UI with durable services

## 1. Sprint identity

Engineering phase 5 (Frontend integration), within-phase depth 3; baseline is the Git commit above, not an uncommitted workspace. **PROPOSED, NOT IMPLEMENTED.**

## 2. Objective

Integrate chapter Writing UI with durable services. Observable handoff: ChapterDetailPage/ChapterDetailView live integration, scene adapter and visual/test evidence.

## 3. Scope boundaries

**IN_SCOPE:** Connect committed-plan gate/read-model, lock controls, generation events/Review, version list, Save receipt/OCC.; Preserve layout and compare before/after 1280/1279 viewports and supported sizes using available browser.

**OUT_OF_SCOPE / MUST NOT:** modify authoritative `docs/business/**`, `docs/db.dbml`, `docs/architecture/decisions/**`; use real application data; run production migration; add client request IDs to SF-06; invent business semantics; rewrite another sprint’s files.

## 4. Authoritative contract snapshot

- This sprint exclusively owns 979-line ChapterDetailView hotspot; no other parallel agent edits it.

- Real scenes.id replaces scn-*; lock not content dirty; Accept invalidated generation forbidden; save dirty content only and keep dirty on failure.

- **REQ-FE-005** `docs/business/fe-core.md#L294-L440`: Writing UI lock/stream/review/save/version with real IDs and dirty buffer preserved. (planning coverage `CONTRACT_VALIDATION_REQUIRED`).

  Normative excerpt (verbatim baseline lines, truncated to 300 chars; use `sed` for the full range):

  > L294: ## 9. Writing Mode (Wave 0 preview)

  > L296: ### 9.1 Trạng thái hợp đồng

  > L298: H1 đã `APPROVED` tại core 1.6. Wave 0/0.1 triển khai UI preview trên Chapter Detail; backend, persistence và committed-plan gate chưa có.

  > L300: / ID / Trạng thái / Quy tắc UI / Dữ liệu/hành vi / Nguồn Core / Nguồn FE / Ghi chú /

- **REQ-WRITE-004** `docs/business/fe-core.md#L294-L440`: Scene version restore UI requires an authoritative backend path; no 41st IPC may be invented. (planning coverage `CONTRACT_VALIDATION_REQUIRED`).

  Normative excerpt (verbatim baseline lines, truncated to 300 chars; use `sed` for the full range):

  > L294: ## 9. Writing Mode (Wave 0 preview)

  > L296: ### 9.1 Trạng thái hợp đồng

  > L298: H1 đã `APPROVED` tại core 1.6. Wave 0/0.1 triển khai UI preview trên Chapter Detail; backend, persistence và committed-plan gate chưa có.

  > L300: / ID / Trạng thái / Quy tắc UI / Dữ liệu/hành vi / Nguồn Core / Nguồn FE / Ghi chú /

**Targeted source references (read only these ranges unless a test proves broader context is necessary):**

- `docs/business/fe-core.md#L294-L440` — focus: Writing preview and target; CLI: `sed -n "294,440p" docs/business/fe-core.md` (or `rg -n "<symbol>" docs/business/fe-core.md` for a changed anchor).

- `docs/business/core.md#L162-L335` — focus: Lock/stream/save business policy; CLI: `sed -n "162,335p" docs/business/core.md` (or `rg -n "<symbol>" docs/business/core.md` for a changed anchor).

- `docs/business/be-core.md#L2231-L2243` — focus: Writing IPC #29–37; CLI: `sed -n "2231,2243p" docs/business/be-core.md` (or `rg -n "<symbol>" docs/business/be-core.md` for a changed anchor).

- `docs/business/fe-core.md#L495-L514` — focus: No layout redesign; CLI: `sed -n "495,514p" docs/business/fe-core.md` (or `rg -n "<symbol>" docs/business/fe-core.md` for a changed anchor).

## 5. Prerequisites

`depends_on: ["phase5_sprint3_deep2", "phase5_sprint4_deep1", "phase4_sprint4_deep1", "phase4_sprint5_deep2"]`.

- `phase5_sprint3_deep2` [RUNTIME_DEPENDENCY] produces Draft/commit/workflow/pending UI and deterministic navigation/fixture tests; inspect its accepted commit, exported artifact/test and `gate_phase5_sprint3_deep2`. If absent, do not start.

- `phase5_sprint4_deep1` [RUNTIME_DEPENDENCY] produces Project/character FE read-model integration and tests; inspect its accepted commit, exported artifact/test and `gate_phase5_sprint4_deep1`. If absent, do not start.

- `phase4_sprint4_deep1` [INTEGRATION_DEPENDENCY] produces WritingPersistence/read model and IPC #29–31,#35–37; inspect its accepted commit, exported artifact/test and `gate_phase4_sprint4_deep1`. If absent, do not start.

- `phase4_sprint5_deep2` [INTEGRATION_DEPENDENCY] produces WritingGeneration service, SceneGenerationStreamEvent and IPC #32–34; inspect its accepted commit, exported artifact/test and `gate_phase4_sprint5_deep2`. If absent, do not start.

## 6. Inputs

Required: accepted outputs of all predecessors; frozen committed SSOT ranges above; actual source paths listed below. Optional context: existing UI mocks for fixtures only. No `report/` input; no historical research.

## 7. File ownership

```yaml

allowed_write_paths:

  - src/pages/ChapterDetailPage.tsx

  - src/features/projects/ChapterDetailView.tsx

  - src/features/projects/scenePreview.ts

  - src/features/projects/scenePreview.test.mjs

  - src/features/projects/chapterBackend.test.mjs

forbidden_write_paths:

  - docs/business/**

  - docs/db.dbml

  - docs/architecture/decisions/**

  - app-data/**

  - report/**

shared_read_only_paths:

  - docs/business/fe-core.md

  - docs/business/core.md

  - docs/business/be-core.md

generated_paths: []

```

Any allowed path may be absent at baseline (not only `src-tauri/`; new FE adapter/shim files are also future targets). Verify with `test -e`/`git ls-files` before editing; existing files are read first. Any extra path requires central integration approval, not a silent scope expansion.

## 8. Public interfaces

PROPOSED_INTERFACE: ChapterDetailPage/ChapterDetailView live integration, scene adapter and visual/test evidence. Every consumed prerequisite interface is `PREREQUISITE_INTERFACE`, not an existing runtime guarantee. If a named signature conflicts with committed SSOT, stop `CONTRACT_BLOCKED`.

## 9. Implementation requirements

- Connect committed-plan gate/read-model, lock controls, generation events/Review, version list, Save receipt/OCC.

- Preserve layout and compare before/after 1280/1279 viewports and supported sizes using available browser.

- Prefer official compatible CLI/tooling over manual generated files; verify installed version and diff. No external LLM/Jev request without authorization; optional Jev advice never overrides SSOT.

## 10. Domain invariants

- This sprint exclusively owns 979-line ChapterDetailView hotspot; no other parallel agent edits it.

- Real scenes.id replaces scn-*; lock not content dirty; Accept invalidated generation forbidden; save dirty content only and keep dirty on failure.

- No partial state escapes; protected provenance, explicit authorization, no external-effect recovery replay where applicable.

## 11. Failure matrix

| Trigger | Expected behavior / classification | Durable effects and recovery | Required test |

|---|---|---|---|

| Lock failure | UI remains unlocked, dirty buffer intact; P5S5-N. | No unauthorized partial commit; re-read actual durable state before retry | phase5_sprint5_deep3-F1 |

| Lost Save response | reuse clientSaveId and reconcile receipt, no blind overwrite; P5S5-C. | No unauthorized partial commit; re-read actual durable state before retry | phase5_sprint5_deep3-F2 |

| Missing accepted predecessor / baseline mismatch | `CONTRACT_BLOCKED`; do not run implementation | No user-data effects; record issue and owner in sprint evidence | phase5_sprint5_deep3-F0 |

## 12. Acceptance tests

Tests below name **future targets**, not tests executed during planning. Each row must become an executable test before the sprint can be accepted.

| Test ID | Initial state | Input / action | Expected output and state | Forbidden side effects / invariant |

|---|---|---|---|---|

| P5S5-P | accepted backend gate + fake typed Tauri transport fixture | In the owned test target, set up the fixture and run the scenario: valid completed stream remains reviewable after request scope expires. | valid completed stream remains reviewable after request scope expires. | No unrelated write; preserve exact expected state and listed invariant |

| P5S5-N | accepted backend gate + fake typed Tauri transport fixture | In the owned test target, set up the fixture and run the scenario: lock-unlock cannot accept invalidated generation. | lock-unlock cannot accept invalidated generation. | No unrelated write; preserve exact expected state and listed invariant |

| P5S5-B | accepted backend gate + fake typed Tauri transport fixture | In the owned test target, set up the fixture and run the scenario: locked clean scene does not block saving another dirty unlocked scene. | locked clean scene does not block saving another dirty unlocked scene. | No unrelated write; preserve exact expected state and listed invariant |

| phase5_sprint5_deep3-F1 | accepted prerequisite fixture / isolated worktree | Inject Lock failure into owned operation | UI remains unlocked, dirty buffer intact; P5S5-N. | No unauthorized commit or external replay |

| phase5_sprint5_deep3-F2 | accepted prerequisite fixture / isolated worktree | Inject Lost Save response into owned operation | reuse clientSaveId and reconcile receipt, no blind overwrite; P5S5-C. | No unauthorized commit or external replay |

| phase5_sprint5_deep3-F0 | predecessor gate absent or baseline SHA drift | Attempt dispatch before predecessor accepted | CONTRACT_BLOCKED; no implementation started | No source/DB modification |

Future command after the target files exist: `pnpm test && pnpm typecheck && pnpm lint && pnpm build  # future target, after dependencies exist`. For FE changes, visual comparison at 1280/1279px and each declared viewport is mandatory; if tooling is unavailable the sprint is `BLOCKED_VISUAL_VERIFICATION` and cannot be accepted. `VISUAL_NOT_VERIFIED` is not a waiver.

## 13. Database and migration safety

No persistent database in this sprint; fixture DB (if any) is disposable and uniquely named per worktree.

## 14. Concurrency contract

- No same-phase/same-depth peer. Cross-phase scheduling still requires exact predecessor gates and conflict scan.

Isolation: own worktree/branch from baseline or accepted integration SHA; unique temp root, test DB and port; no shared mutable process/global fixture. Same depth alone is NOT permission to dispatch.

## 15. Deliverables

- [ ] ChapterDetailPage/ChapterDetailView live integration, scene adapter and visual/test evidence

- [ ] Tests above and evidence including command exit codes, scope diff and accepted commit SHA.

## 16. Completion criteria

`SPRINT_IMPLEMENTED` requires artifacts, `SPRINT_TESTED` requires executed acceptance tests, `SPRINT_INTEGRATED` requires merged checks, and **`SPRINT_ACCEPTED` requires `gate_phase5_sprint5_deep3` accepted**. A contract-evidence sprint cannot be ACCEPTED for an affected implementation successor while its owner decision is unresolved. No agent declaration alone unlocks successors. All required assertions, file scope and baseline must pass.

## 17. Handoff contract

- ChapterDetailPage/ChapterDetailView live integration, scene adapter and visual/test evidence

- Exact artifacts: src/pages/ChapterDetailPage.tsx, src/features/projects/ChapterDetailView.tsx, src/features/projects/scenePreview.ts, src/features/projects/scenePreview.test.mjs, src/features/projects/chapterBackend.test.mjs; accepted integration SHA and `gate_phase5_sprint5_deep3` record. A successor must verify exports/ABI rather than infer from prose.

## 18. Resume and failure behavior

Not started: verify baseline and deps. Partial/test failure: inspect git status, changed paths and durable test DB, resume only safe incomplete steps. Implemented but unintegrated: do not dispatch successors. Interrupted agent/rejected patch: preserve branch, rebase only after review, rerun tests. Specification conflict: `CONTRACT_BLOCKED`, record evidence, request authority resolution; never edit SSOT in this sprint.

**Explicit unblock requirements:** CI-08, CONTRACT_REVIEW_REQUIRED:REQ-FE-005, CONTRACT_REVIEW_REQUIRED:REQ-WRITE-004.


---

Subagent budget: 256K tokens maximum; start with the bounded references above, do not read entire SSOT by default. CLI-first with verified tool compatibility. Treat Jev/TypeSafe as optional advisory only when available and authorized; never send private repository content externally without consent.
