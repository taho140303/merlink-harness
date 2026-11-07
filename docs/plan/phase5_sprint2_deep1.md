---
sprint_id: phase5_sprint2_deep1
title: Replace chat and progress mocks with live state
phase: 5
sprint: 2
deep: 1
baseline_commit: 57a462efb953a665e3673b15e6cc33622a4f2d39
status: PLANNED
---

# phase5_sprint2_deep1 — Replace chat and progress mocks with live state

## 1. Sprint identity

Engineering phase 5 (Frontend integration), within-phase depth 1; baseline is the Git commit above, not an uncommitted workspace. **PROPOSED, NOT IMPLEMENTED.**

## 2. Objective

Replace chat and progress mocks with live state. Observable handoff: Chat/progress integration and tests; props/event contract for DraftWorkspace owner.

## 3. Scope boundaries

**IN_SCOPE:** Replace fake timers/messages with IPC and Channel state; maintain controlled cancellation and interrupted-turn notice.; Add fake-transport tests for applied vs sealed, provider failure and refresh from backend.

**OUT_OF_SCOPE / MUST NOT:** modify authoritative `docs/business/**`, `docs/db.dbml`, `docs/architecture/decisions/**`; use real application data; run production migration; add client request IDs to SF-06; invent business semantics; rewrite another sprint’s files.

## 4. Authoritative contract snapshot

- Use backend `create_or_load_chat_session`/send/cancel and two progress values; no synthetic readiness or hidden direct State edit.

- Existing layout and feedbackContext remain; no DraftWorkspace or global i18n/router edits.

- `systemLanguage` request vs UI setting precedence is a P1S1 authority gate; never allow caller-provided language to silently override trusted setting.

- **REQ-FE-002** `docs/business/fe-core.md#L104-L176`: Replace mock chat/draft with real data, readiness, progress, feedback context. (planning coverage `ASSIGNED`).

  Normative excerpt (verbatim baseline lines, truncated to 300 chars; use `sed` for the full range):

  > L104: ## 3. Chat và Readiness Gate

  > L106: ### 3.1 Bảng hành vi

  > L108: / ID / Trạng thái / Quy tắc UI / Dữ liệu/hành vi / Nguồn Core / Nguồn FE / Ghi chú /

  > L109: / :--- / :--- / :--- / :--- / :--- / :--- / :--- /

**Targeted source references (read only these ranges unless a test proves broader context is necessary):**

- `docs/business/fe-core.md#L104-L130` — focus: Chat/readiness target; CLI: `sed -n "104,130p" docs/business/fe-core.md` (or `rg -n "<symbol>" docs/business/fe-core.md` for a changed anchor).

- `docs/business/fe-core.md#L32-L80` — focus: Current mock state; CLI: `sed -n "32,80p" docs/business/fe-core.md` (or `rg -n "<symbol>" docs/business/fe-core.md` for a changed anchor).

- `docs/business/fe-core.md#L495-L514` — focus: Preserve UI; CLI: `sed -n "495,514p" docs/business/fe-core.md` (or `rg -n "<symbol>" docs/business/fe-core.md` for a changed anchor).

- `docs/business/be-core.md#L2163-L2180` — focus: Chat/draft DTOs; CLI: `sed -n "2163,2180p" docs/business/be-core.md` (or `rg -n "<symbol>" docs/business/be-core.md` for a changed anchor).

## 5. Prerequisites

`depends_on: ["phase5_sprint1_deep0", "phase3_sprint7_deep4", "phase3_sprint2_deep0", "phase1_sprint1_deep0"]`.

- `phase5_sprint1_deep0` [TYPE_DEPENDENCY] produces Typed backend client/adapter and FE fixture tests; inspect its accepted commit, exported artifact/test and `gate_phase5_sprint1_deep0`. If absent, do not start.

- `phase3_sprint7_deep4` [INTEGRATION_DEPENDENCY] produces ChatService, prompt composer, ChatStreamEvent and IPC #1–4,#9; inspect its accepted commit, exported artifact/test and `gate_phase3_sprint7_deep4`. If absent, do not start.

- `phase3_sprint2_deep0` [INTEGRATION_DEPENDENCY] produces DraftService, DraftRuntimeRegistry guard, draft/version IPC #5–8,#10–11, TX01/03 fixtures; inspect its accepted commit, exported artifact/test and `gate_phase3_sprint2_deep0`. If absent, do not start.

- `phase1_sprint1_deep0` [INTEGRATION_DEPENDENCY] produces docs/plan/evidence/contract-ledger.md with issue IDs, accepted SSOT SHA or CONTRACT_BLOCKED and affected sprint IDs; inspect its accepted commit, exported artifact/test and `gate_phase1_sprint1_deep0`. If absent, do not start.

## 6. Inputs

Required: accepted outputs of all predecessors; frozen committed SSOT ranges above; actual source paths listed below. Optional context: existing UI mocks for fixtures only. No `report/` input; no historical research.

## 7. File ownership

```yaml

allowed_write_paths:

  - src/features/chat/ChatView.tsx

  - src/features/chat/ChatHeader.tsx

  - src/features/business-flow/progressEngine.ts

  - src/features/chat/chatBackend.test.mjs

forbidden_write_paths:

  - docs/business/**

  - docs/db.dbml

  - docs/architecture/decisions/**

  - app-data/**

  - report/**

shared_read_only_paths:

  - docs/business/fe-core.md

  - docs/business/be-core.md

generated_paths: []

```

Any allowed path may be absent at baseline (not only `src-tauri/`; new FE adapter/shim files are also future targets). Verify with `test -e`/`git ls-files` before editing; existing files are read first. Any extra path requires central integration approval, not a silent scope expansion.

## 8. Public interfaces

PROPOSED_INTERFACE: Chat/progress integration and tests; props/event contract for DraftWorkspace owner. Every consumed prerequisite interface is `PREREQUISITE_INTERFACE`, not an existing runtime guarantee. If a named signature conflicts with committed SSOT, stop `CONTRACT_BLOCKED`.

## 9. Implementation requirements

- Replace fake timers/messages with IPC and Channel state; maintain controlled cancellation and interrupted-turn notice.

- Add fake-transport tests for applied vs sealed, provider failure and refresh from backend.

- Prefer official compatible CLI/tooling over manual generated files; verify installed version and diff. No external LLM/Jev request without authorization; optional Jev advice never overrides SSOT.

## 10. Domain invariants

- Use backend `create_or_load_chat_session`/send/cancel and two progress values; no synthetic readiness or hidden direct State edit.

- Existing layout and feedbackContext remain; no DraftWorkspace or global i18n/router edits.

- `systemLanguage` request vs UI setting precedence is a P1S1 authority gate; never allow caller-provided language to silently override trusted setting.

- No partial state escapes; protected provenance, explicit authorization, no external-effect recovery replay where applicable.

## 11. Failure matrix

| Trigger | Expected behavior / classification | Durable effects and recovery | Required test |

|---|---|---|---|

| Lost response | reconcile durable backend before claiming version; P5S2-N. | No unauthorized partial commit; re-read actual durable state before retry | phase5_sprint2_deep1-F1 |

| No backend | explicit unavailable UI, no mock marked real; P5S2-C. | No unauthorized partial commit; re-read actual durable state before retry | phase5_sprint2_deep1-F2 |

| Missing accepted predecessor / baseline mismatch | `CONTRACT_BLOCKED`; do not run implementation | No user-data effects; record issue and owner in sprint evidence | phase5_sprint2_deep1-F0 |

## 12. Acceptance tests

Tests below name **future targets**, not tests executed during planning. Each row must become an executable test before the sprint can be accepted.

| Test ID | Initial state | Input / action | Expected output and state | Forbidden side effects / invariant |

|---|---|---|---|---|

| P5S2-P | accepted backend gate + fake typed Tauri transport fixture | In the owned test target, set up the fixture and run the scenario: streamed tool result shows pending independently of sealed version. | streamed tool result shows pending independently of sealed version. | No unrelated write; preserve exact expected state and listed invariant |

| P5S2-N | accepted backend gate + fake typed Tauri transport fixture | In the owned test target, set up the fixture and run the scenario: provider failure never marks chat turn saved. | provider failure never marks chat turn saved. | No unrelated write; preserve exact expected state and listed invariant |

| P5S2-B | accepted backend gate + fake typed Tauri transport fixture | In the owned test target, set up the fixture and run the scenario: 46-attribute confirmed/AI progress matches DTO. | 46-attribute confirmed/AI progress matches DTO. | No unrelated write; preserve exact expected state and listed invariant |

| phase5_sprint2_deep1-F1 | accepted prerequisite fixture / isolated worktree | Inject Lost response into owned operation | reconcile durable backend before claiming version; P5S2-N. | No unauthorized commit or external replay |

| phase5_sprint2_deep1-F2 | accepted prerequisite fixture / isolated worktree | Inject No backend into owned operation | explicit unavailable UI, no mock marked real; P5S2-C. | No unauthorized commit or external replay |

| phase5_sprint2_deep1-F0 | predecessor gate absent or baseline SHA drift | Attempt dispatch before predecessor accepted | CONTRACT_BLOCKED; no implementation started | No source/DB modification |

Future command after the target files exist: `pnpm test && pnpm typecheck && pnpm lint && pnpm build  # future target, after dependencies exist`. For FE changes, visual comparison at 1280/1279px and each declared viewport is mandatory; if tooling is unavailable the sprint is `BLOCKED_VISUAL_VERIFICATION` and cannot be accepted. `VISUAL_NOT_VERIFIED` is not a waiver.

## 13. Database and migration safety

No persistent database in this sprint; fixture DB (if any) is disposable and uniquely named per worktree.

## 14. Concurrency contract

- Peer `phase5_sprint4_deep1`: candidate parallel only after frozen contracts/gates; NONE: Disjoint exact write paths; shared types frozen by predecessors; worktree-local test resources.

- Peer `phase5_sprint6_deep1`: candidate parallel only after frozen contracts/gates; NONE: Disjoint exact write paths; shared types frozen by predecessors; worktree-local test resources.

Isolation: own worktree/branch from baseline or accepted integration SHA; unique temp root, test DB and port; no shared mutable process/global fixture. Same depth alone is NOT permission to dispatch.

## 15. Deliverables

- [ ] Chat/progress integration and tests; props/event contract for DraftWorkspace owner

- [ ] Tests above and evidence including command exit codes, scope diff and accepted commit SHA.

## 16. Completion criteria

`SPRINT_IMPLEMENTED` requires artifacts, `SPRINT_TESTED` requires executed acceptance tests, `SPRINT_INTEGRATED` requires merged checks, and **`SPRINT_ACCEPTED` requires `gate_phase5_sprint2_deep1` accepted**. A contract-evidence sprint cannot be ACCEPTED for an affected implementation successor while its owner decision is unresolved. No agent declaration alone unlocks successors. All required assertions, file scope and baseline must pass.

## 17. Handoff contract

- Chat/progress integration and tests; props/event contract for DraftWorkspace owner

- Exact artifacts: src/features/chat/ChatView.tsx, src/features/chat/ChatHeader.tsx, src/features/business-flow/progressEngine.ts, src/features/chat/chatBackend.test.mjs; accepted integration SHA and `gate_phase5_sprint2_deep1` record. A successor must verify exports/ABI rather than infer from prose.

## 18. Resume and failure behavior

Not started: verify baseline and deps. Partial/test failure: inspect git status, changed paths and durable test DB, resume only safe incomplete steps. Implemented but unintegrated: do not dispatch successors. Interrupted agent/rejected patch: preserve branch, rebase only after review, rerun tests. Specification conflict: `CONTRACT_BLOCKED`, record evidence, request authority resolution; never edit SSOT in this sprint.

**Explicit unblock requirements:** CI-09.


---

Subagent budget: 256K tokens maximum; start with the bounded references above, do not read entire SSOT by default. CLI-first with verified tool compatibility. Treat Jev/TypeSafe as optional advisory only when available and authorized; never send private repository content externally without consent.
