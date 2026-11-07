---
sprint_id: phase5_sprint3_deep2
title: Integrate draft workflow patch restore and commit UI
phase: 5
sprint: 3
deep: 2
baseline_commit: 57a462efb953a665e3673b15e6cc33622a4f2d39
status: PLANNED
---

# phase5_sprint3_deep2 — Integrate draft workflow patch restore and commit UI

## 1. Sprint identity

Engineering phase 5 (Frontend integration), within-phase depth 2; baseline is the Git commit above, not an uncommitted workspace. **PROPOSED, NOT IMPLEMENTED.**

## 2. Objective

Integrate draft workflow patch restore and commit UI. Observable handoff: Draft/commit/workflow/pending UI and deterministic navigation/fixture tests.

## 3. Scope boundaries

**IN_SCOPE:** Wire version list/restore, pending confirm/batch, workflow status/cancel/replan and CommitPreview/CommitDraft.; Keep existing C01–C12/N01–N13 tests; add mock transport tests for failed commit and lost response.

**OUT_OF_SCOPE / MUST NOT:** modify authoritative `docs/business/**`, `docs/db.dbml`, `docs/architecture/decisions/**`; use real application data; run production migration; add client request IDs to SF-06; invent business semantics; rewrite another sprint’s files.

## 4. Authoritative contract snapshot

- Backend head is authoritative; Restore expires pending; historical Verified alone never marks current task applicable.

- Commit dialog blocks pending/validation, only real success navigates; no fake DEV fixture in production.

- **REQ-FE-003** `docs/business/fe-core.md#L178-L292`: Pending/restore/workflow/commit recovery statuses and zero-net semantics. (planning coverage `ASSIGNED`).

  Normative excerpt (verbatim baseline lines, truncated to 300 chars; use `sed` for the full range):

  > L178: ## 5. Source và Pending Patches

  > L180: ### 5.1 Contract đích

  > L182: / ID / Trạng thái / Quy tắc UI / Dữ liệu/hành vi / Nguồn Core / Nguồn FE / Ghi chú /

  > L183: / :--- / :--- / :--- / :--- / :--- / :--- / :--- /

**Targeted source references (read only these ranges unless a test proves broader context is necessary):**

- `docs/business/fe-core.md#L132-L292` — focus: Draft, pending, workflow, commit, recovery gaps; CLI: `sed -n "132,292p" docs/business/fe-core.md` (or `rg -n "<symbol>" docs/business/fe-core.md` for a changed anchor).

- `docs/business/be-core.md#L2193-L2212` — focus: Patch/workflow/commit IPC; CLI: `sed -n "2193,2212p" docs/business/be-core.md` (or `rg -n "<symbol>" docs/business/be-core.md` for a changed anchor).

- `docs/business/core.md#L652-L768` — focus: Start/commit/versions; CLI: `sed -n "652,768p" docs/business/core.md` (or `rg -n "<symbol>" docs/business/core.md` for a changed anchor).

- `docs/business/fe-core.md#L495-L514` — focus: UI preservation; CLI: `sed -n "495,514p" docs/business/fe-core.md` (or `rg -n "<symbol>" docs/business/fe-core.md` for a changed anchor).

## 5. Prerequisites

`depends_on: ["phase5_sprint2_deep1", "phase4_sprint1_deep0", "phase4_sprint2_deep0", "phase5_sprint1_deep0"]`.

- `phase5_sprint2_deep1` [RUNTIME_DEPENDENCY] produces Chat/progress integration and tests; props/event contract for DraftWorkspace owner; inspect its accepted commit, exported artifact/test and `gate_phase5_sprint2_deep1`. If absent, do not start.

- `phase4_sprint1_deep0` [INTEGRATION_DEPENDENCY] produces WorkflowService, WorkflowProgressEvent and IPC #16–18; inspect its accepted commit, exported artifact/test and `gate_phase4_sprint1_deep0`. If absent, do not start.

- `phase4_sprint2_deep0` [INTEGRATION_DEPENDENCY] produces CommitService/CommitRepo, preview/commit/reopen IPC #19–21; inspect its accepted commit, exported artifact/test and `gate_phase4_sprint2_deep0`. If absent, do not start.

- `phase5_sprint1_deep0` [TYPE_DEPENDENCY] produces Typed backend client/adapter and FE fixture tests; inspect its accepted commit, exported artifact/test and `gate_phase5_sprint1_deep0`. If absent, do not start.

## 6. Inputs

Required: accepted outputs of all predecessors; frozen committed SSOT ranges above; actual source paths listed below. Optional context: existing UI mocks for fixtures only. No `report/` input; no historical research.

## 7. File ownership

```yaml

allowed_write_paths:

  - src/features/draft/DraftWorkspace.tsx

  - src/features/draft/DraftToolbar.tsx

  - src/features/draft/CommitConfirmationDialog.tsx

  - src/features/draft/commitPreview.ts

  - src/features/draft/commitNavigation.ts

  - src/features/draft/draftBackend.test.mjs

forbidden_write_paths:

  - docs/business/**

  - docs/db.dbml

  - docs/architecture/decisions/**

  - app-data/**

  - report/**

shared_read_only_paths:

  - docs/business/fe-core.md

  - docs/business/be-core.md

  - docs/business/core.md

generated_paths: []

```

Any allowed path may be absent at baseline (not only `src-tauri/`; new FE adapter/shim files are also future targets). Verify with `test -e`/`git ls-files` before editing; existing files are read first. Any extra path requires central integration approval, not a silent scope expansion.

## 8. Public interfaces

PROPOSED_INTERFACE: Draft/commit/workflow/pending UI and deterministic navigation/fixture tests. Every consumed prerequisite interface is `PREREQUISITE_INTERFACE`, not an existing runtime guarantee. If a named signature conflicts with committed SSOT, stop `CONTRACT_BLOCKED`.

## 9. Implementation requirements

- Wire version list/restore, pending confirm/batch, workflow status/cancel/replan and CommitPreview/CommitDraft.

- Keep existing C01–C12/N01–N13 tests; add mock transport tests for failed commit and lost response.

- Prefer official compatible CLI/tooling over manual generated files; verify installed version and diff. No external LLM/Jev request without authorization; optional Jev advice never overrides SSOT.

## 10. Domain invariants

- Backend head is authoritative; Restore expires pending; historical Verified alone never marks current task applicable.

- Commit dialog blocks pending/validation, only real success navigates; no fake DEV fixture in production.

- No partial state escapes; protected provenance, explicit authorization, no external-effect recovery replay where applicable.

## 11. Failure matrix

| Trigger | Expected behavior / classification | Durable effects and recovery | Required test |

|---|---|---|---|

| WorkflowBaselineUnavailable | manual-repair message, no synthetic version; P5S3-N. | No unauthorized partial commit; re-read actual durable state before retry | phase5_sprint3_deep2-F1 |

| Indeterminate receipt | requires_replan UI, not completed badge; P5S3-C. | No unauthorized partial commit; re-read actual durable state before retry | phase5_sprint3_deep2-F2 |

| Missing accepted predecessor / baseline mismatch | `CONTRACT_BLOCKED`; do not run implementation | No user-data effects; record issue and owner in sprint evidence | phase5_sprint3_deep2-F0 |

## 12. Acceptance tests

Tests below name **future targets**, not tests executed during planning. Each row must become an executable test before the sprint can be accepted.

| Test ID | Initial state | Input / action | Expected output and state | Forbidden side effects / invariant |

|---|---|---|---|---|

| P5S3-P | accepted backend gate + fake typed Tauri transport fixture | In the owned test target, set up the fixture and run the scenario: commit success navigates once to real project in View mode. | commit success navigates once to real project in View mode. | No unrelated write; preserve exact expected state and listed invariant |

| P5S3-N | accepted backend gate + fake typed Tauri transport fixture | In the owned test target, set up the fixture and run the scenario: pending patch blocks Confirm and cannot be dismissed away. | pending patch blocks Confirm and cannot be dismissed away. | No unrelated write; preserve exact expected state and listed invariant |

| P5S3-B | accepted backend gate + fake typed Tauri transport fixture | In the owned test target, set up the fixture and run the scenario: failed TX05 preserves draft and displays structured error. | failed TX05 preserves draft and displays structured error. | No unrelated write; preserve exact expected state and listed invariant |

| phase5_sprint3_deep2-F1 | accepted prerequisite fixture / isolated worktree | Inject WorkflowBaselineUnavailable into owned operation | manual-repair message, no synthetic version; P5S3-N. | No unauthorized commit or external replay |

| phase5_sprint3_deep2-F2 | accepted prerequisite fixture / isolated worktree | Inject Indeterminate receipt into owned operation | requires_replan UI, not completed badge; P5S3-C. | No unauthorized commit or external replay |

| phase5_sprint3_deep2-F0 | predecessor gate absent or baseline SHA drift | Attempt dispatch before predecessor accepted | CONTRACT_BLOCKED; no implementation started | No source/DB modification |

Future command after the target files exist: `pnpm test && pnpm typecheck && pnpm lint && pnpm build  # future target, after dependencies exist`. For FE changes, visual comparison at 1280/1279px and each declared viewport is mandatory; if tooling is unavailable the sprint is `BLOCKED_VISUAL_VERIFICATION` and cannot be accepted. `VISUAL_NOT_VERIFIED` is not a waiver.

## 13. Database and migration safety

No persistent database in this sprint; fixture DB (if any) is disposable and uniquely named per worktree.

## 14. Concurrency contract

- No same-phase/same-depth peer. Cross-phase scheduling still requires exact predecessor gates and conflict scan.

Isolation: own worktree/branch from baseline or accepted integration SHA; unique temp root, test DB and port; no shared mutable process/global fixture. Same depth alone is NOT permission to dispatch.

## 15. Deliverables

- [ ] Draft/commit/workflow/pending UI and deterministic navigation/fixture tests

- [ ] Tests above and evidence including command exit codes, scope diff and accepted commit SHA.

## 16. Completion criteria

`SPRINT_IMPLEMENTED` requires artifacts, `SPRINT_TESTED` requires executed acceptance tests, `SPRINT_INTEGRATED` requires merged checks, and **`SPRINT_ACCEPTED` requires `gate_phase5_sprint3_deep2` accepted**. A contract-evidence sprint cannot be ACCEPTED for an affected implementation successor while its owner decision is unresolved. No agent declaration alone unlocks successors. All required assertions, file scope and baseline must pass.

## 17. Handoff contract

- Draft/commit/workflow/pending UI and deterministic navigation/fixture tests

- Exact artifacts: src/features/draft/DraftWorkspace.tsx, src/features/draft/DraftToolbar.tsx, src/features/draft/CommitConfirmationDialog.tsx, src/features/draft/commitPreview.ts, src/features/draft/commitNavigation.ts, src/features/draft/draftBackend.test.mjs; accepted integration SHA and `gate_phase5_sprint3_deep2` record. A successor must verify exports/ABI rather than infer from prose.

## 18. Resume and failure behavior

Not started: verify baseline and deps. Partial/test failure: inspect git status, changed paths and durable test DB, resume only safe incomplete steps. Implemented but unintegrated: do not dispatch successors. Interrupted agent/rejected patch: preserve branch, rebase only after review, rerun tests. Specification conflict: `CONTRACT_BLOCKED`, record evidence, request authority resolution; never edit SSOT in this sprint.


---

Subagent budget: 256K tokens maximum; start with the bounded references above, do not read entire SSOT by default. CLI-first with verified tool compatibility. Treat Jev/TypeSafe as optional advisory only when available and authorized; never send private repository content externally without consent.
