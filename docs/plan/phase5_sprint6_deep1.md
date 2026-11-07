---
sprint_id: phase5_sprint6_deep1
title: Integrate provider configuration UI safely
phase: 5
sprint: 6
deep: 1
baseline_commit: 57a462efb953a665e3673b15e6cc33622a4f2d39
status: PLANNED
---

# phase5_sprint6_deep1 — Integrate provider configuration UI safely

## 1. Sprint identity

Engineering phase 5 (Frontend integration), within-phase depth 1; baseline is the Git commit above, not an uncommitted workspace. **PROPOSED, NOT IMPLEMENTED.**

## 2. Objective

Integrate provider configuration UI safely. Observable handoff: Provider settings integration and secret-safe FE tests.

## 3. Scope boundaries

**IN_SCOPE:** Wire provider config commands through FE transport; clear key from transient form state after submission.; Add provider status/failure tests without external network or fake successful connectivity.

**OUT_OF_SCOPE / MUST NOT:** modify authoritative `docs/business/**`, `docs/db.dbml`, `docs/architecture/decisions/**`; use real application data; run production migration; add client request IDs to SF-06; invent business semantics; rewrite another sprint’s files.

## 4. Authoritative contract snapshot

- No API key persisted in FE localStorage, SQLite or log; keychain/session fallback status explicit.

- Provider configuration is live IPC state; no durable config claim unless committed SSOT specifies storage. Do not repurpose unrelated AI Factory localStorage BRD as authority or edit global SettingsDialog.

- **REQ-FE-006** `docs/business/fe-core.md#L32-L80`: Provider/config UI reflects backend; no mock data presented as durable. (planning coverage `ASSIGNED`).

  Normative excerpt (verbatim baseline lines, truncated to 300 chars; use `sed` for the full range):

  > L32: ## 1. Kiến trúc frontend hiện tại

  > L34: ### 1.1 Stack và giới hạn

  > L36: / Thành phần / Hiện trạng / Nguồn /

  > L37: / :--- / :--- / :--- /

**Targeted source references (read only these ranges unless a test proves broader context is necessary):**

- `docs/business/be-core.md#L2071-L2098` — focus: Keychain fallback and redaction; CLI: `sed -n "2071,2098p" docs/business/be-core.md` (or `rg -n "<symbol>" docs/business/be-core.md` for a changed anchor).

- `docs/business/be-core.md#L2245-L2251` — focus: Config IPC #38–40; CLI: `sed -n "2245,2251p" docs/business/be-core.md` (or `rg -n "<symbol>" docs/business/be-core.md` for a changed anchor).

- `docs/business/fe-core.md#L32-L80` — focus: Mock settings boundary; CLI: `sed -n "32,80p" docs/business/fe-core.md` (or `rg -n "<symbol>" docs/business/fe-core.md` for a changed anchor).

- `docs/design-system.md#L8-L16` — focus: UI chrome constraints; CLI: `sed -n "8,16p" docs/design-system.md` (or `rg -n "<symbol>" docs/design-system.md` for a changed anchor).

## 5. Prerequisites

`depends_on: ["phase5_sprint1_deep0", "phase2_sprint4_deep3"]`.

- `phase5_sprint1_deep0` [TYPE_DEPENDENCY] produces Typed backend client/adapter and FE fixture tests; inspect its accepted commit, exported artifact/test and `gate_phase5_sprint1_deep0`. If absent, do not start.

- `phase2_sprint4_deep3` [INTEGRATION_DEPENDENCY] produces LlmProvider trait, adapters, credential/redaction APIs and config IPC #38–40; inspect its accepted commit, exported artifact/test and `gate_phase2_sprint4_deep3`. If absent, do not start.

## 6. Inputs

Required: accepted outputs of all predecessors; frozen committed SSOT ranges above; actual source paths listed below. Optional context: existing UI mocks for fixtures only. No `report/` input; no historical research.

## 7. File ownership

```yaml

allowed_write_paths:

  - src/features/settings/AIProvidersTab.tsx

  - src/features/settings/ProviderForm.tsx

  - src/features/settings/ChooseModelsDialog.tsx

  - src/features/settings/providerBackend.test.mjs

forbidden_write_paths:

  - docs/business/**

  - docs/db.dbml

  - docs/architecture/decisions/**

  - app-data/**

  - report/**

shared_read_only_paths:

  - docs/business/be-core.md

  - docs/business/fe-core.md

  - docs/design-system.md

generated_paths: []

```

Any allowed path may be absent at baseline (not only `src-tauri/`; new FE adapter/shim files are also future targets). Verify with `test -e`/`git ls-files` before editing; existing files are read first. Any extra path requires central integration approval, not a silent scope expansion.

## 8. Public interfaces

PROPOSED_INTERFACE: Provider settings integration and secret-safe FE tests. Every consumed prerequisite interface is `PREREQUISITE_INTERFACE`, not an existing runtime guarantee. If a named signature conflicts with committed SSOT, stop `CONTRACT_BLOCKED`.

## 9. Implementation requirements

- Wire provider config commands through FE transport; clear key from transient form state after submission.

- Add provider status/failure tests without external network or fake successful connectivity.

- Prefer official compatible CLI/tooling over manual generated files; verify installed version and diff. No external LLM/Jev request without authorization; optional Jev advice never overrides SSOT.

## 10. Domain invariants

- No API key persisted in FE localStorage, SQLite or log; keychain/session fallback status explicit.

- Provider configuration is live IPC state; no durable config claim unless committed SSOT specifies storage. Do not repurpose unrelated AI Factory localStorage BRD as authority or edit global SettingsDialog.

- No partial state escapes; protected provenance, explicit authorization, no external-effect recovery replay where applicable.

## 11. Failure matrix

| Trigger | Expected behavior / classification | Durable effects and recovery | Required test |

|---|---|---|---|

| Keychain unavailable | session-only warning, no plaintext persistence; P5S6-N. | No unauthorized partial commit; re-read actual durable state before retry | phase5_sprint6_deep1-F1 |

| Provider 401 | auth failure, never implicit retry with key logged; P5S6-C. | No unauthorized partial commit; re-read actual durable state before retry | phase5_sprint6_deep1-F2 |

| Missing accepted predecessor / baseline mismatch | `CONTRACT_BLOCKED`; do not run implementation | No user-data effects; record issue and owner in sprint evidence | phase5_sprint6_deep1-F0 |

## 12. Acceptance tests

Tests below name **future targets**, not tests executed during planning. Each row must become an executable test before the sprint can be accepted.

| Test ID | Initial state | Input / action | Expected output and state | Forbidden side effects / invariant |

|---|---|---|---|---|

| P5S6-P | accepted backend gate + fake typed Tauri transport fixture | In the owned test target, set up the fixture and run the scenario: config loads from fake IPC and masks key. | config loads from fake IPC and masks key. | No unrelated write; preserve exact expected state and listed invariant |

| P5S6-N | accepted backend gate + fake typed Tauri transport fixture | In the owned test target, set up the fixture and run the scenario: serialized localStorage never contains synthetic key. | serialized localStorage never contains synthetic key. | No unrelated write; preserve exact expected state and listed invariant |

| P5S6-B | accepted backend gate + fake typed Tauri transport fixture | In the owned test target, set up the fixture and run the scenario: provider connection failure shown without success. | provider connection failure shown without success. | No unrelated write; preserve exact expected state and listed invariant |

| phase5_sprint6_deep1-F1 | accepted prerequisite fixture / isolated worktree | Inject Keychain unavailable into owned operation | session-only warning, no plaintext persistence; P5S6-N. | No unauthorized commit or external replay |

| phase5_sprint6_deep1-F2 | accepted prerequisite fixture / isolated worktree | Inject Provider 401 into owned operation | auth failure, never implicit retry with key logged; P5S6-C. | No unauthorized commit or external replay |

| phase5_sprint6_deep1-F0 | predecessor gate absent or baseline SHA drift | Attempt dispatch before predecessor accepted | CONTRACT_BLOCKED; no implementation started | No source/DB modification |

Future command after the target files exist: `pnpm test && pnpm typecheck && pnpm lint && pnpm build  # future target, after dependencies exist`. For FE changes, visual comparison at 1280/1279px and each declared viewport is mandatory; if tooling is unavailable the sprint is `BLOCKED_VISUAL_VERIFICATION` and cannot be accepted. `VISUAL_NOT_VERIFIED` is not a waiver.

## 13. Database and migration safety

No persistent database in this sprint; fixture DB (if any) is disposable and uniquely named per worktree.

## 14. Concurrency contract

- Peer `phase5_sprint2_deep1`: candidate parallel only after frozen contracts/gates; NONE: Disjoint exact write paths; shared types frozen by predecessors; worktree-local test resources.

- Peer `phase5_sprint4_deep1`: candidate parallel only after frozen contracts/gates; NONE: Disjoint exact write paths; shared types frozen by predecessors; worktree-local test resources.

Isolation: own worktree/branch from baseline or accepted integration SHA; unique temp root, test DB and port; no shared mutable process/global fixture. Same depth alone is NOT permission to dispatch.

## 15. Deliverables

- [ ] Provider settings integration and secret-safe FE tests

- [ ] Tests above and evidence including command exit codes, scope diff and accepted commit SHA.

## 16. Completion criteria

`SPRINT_IMPLEMENTED` requires artifacts, `SPRINT_TESTED` requires executed acceptance tests, `SPRINT_INTEGRATED` requires merged checks, and **`SPRINT_ACCEPTED` requires `gate_phase5_sprint6_deep1` accepted**. A contract-evidence sprint cannot be ACCEPTED for an affected implementation successor while its owner decision is unresolved. No agent declaration alone unlocks successors. All required assertions, file scope and baseline must pass.

## 17. Handoff contract

- Provider settings integration and secret-safe FE tests

- Exact artifacts: src/features/settings/AIProvidersTab.tsx, src/features/settings/ProviderForm.tsx, src/features/settings/ChooseModelsDialog.tsx, src/features/settings/providerBackend.test.mjs; accepted integration SHA and `gate_phase5_sprint6_deep1` record. A successor must verify exports/ABI rather than infer from prose.

## 18. Resume and failure behavior

Not started: verify baseline and deps. Partial/test failure: inspect git status, changed paths and durable test DB, resume only safe incomplete steps. Implemented but unintegrated: do not dispatch successors. Interrupted agent/rejected patch: preserve branch, rebase only after review, rerun tests. Specification conflict: `CONTRACT_BLOCKED`, record evidence, request authority resolution; never edit SSOT in this sprint.

**Explicit unblock requirements:** CI-10.


---

Subagent budget: 256K tokens maximum; start with the bounded references above, do not read entire SSOT by default. CLI-first with verified tool compatibility. Treat Jev/TypeSafe as optional advisory only when available and authorized; never send private repository content externally without consent.
