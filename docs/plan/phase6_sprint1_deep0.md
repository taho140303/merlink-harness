---
sprint_id: phase6_sprint1_deep0
title: Add CI and reproducible packaging
phase: 6
sprint: 1
deep: 0
baseline_commit: 57a462efb953a665e3673b15e6cc33622a4f2d39
status: PLANNED
---

# phase6_sprint1_deep0 — Add CI and reproducible packaging

## 1. Sprint identity

Engineering phase 6 (Application assurance), within-phase depth 0; baseline is the Git commit above, not an uncommitted workspace. **PROPOSED, NOT IMPLEMENTED.**

## 2. Objective

Add CI and reproducible packaging. Observable handoff: CI workflow and reproducible Docker/package scripts.

## 3. Scope boundaries

**IN_SCOPE:** Use official pnpm/Tauri CLI compatible with Node/Rust versions; inspect generated changes.; Wire lint/typecheck/test/build and cargo test/check jobs once tools exist.

**OUT_OF_SCOPE / MUST NOT:** modify authoritative `docs/business/**`, `docs/db.dbml`, `docs/architecture/decisions/**`; use real application data; run production migration; add client request IDs to SF-06; invent business semantics; rewrite another sprint’s files.

## 4. Authoritative contract snapshot

- Repo has pnpm lock but Dockerfile assumes missing package-lock; repair package-manager choice only here.

- CI uses isolated temp DB/target, no secrets; do not run production migrations or edit shared Cargo.

**Targeted source references (read only these ranges unless a test proves broader context is necessary):**

- `package.json#L5-L45` — focus: pnpm scripts and RR stack; CLI: `sed -n "5,45p" package.json` (or `rg -n "<symbol>" package.json` for a changed anchor).

- `react-router.config.ts#L1-L6` — focus: SSR build target; CLI: `sed -n "1,6p" react-router.config.ts` (or `rg -n "<symbol>" react-router.config.ts` for a changed anchor).

- `docs/business/be-core.md#L131-L174` — focus: Tauri application lifecycle; CLI: `sed -n "131,174p" docs/business/be-core.md` (or `rg -n "<symbol>" docs/business/be-core.md` for a changed anchor).

## 5. Prerequisites

`depends_on: ["phase2_sprint1_deep0"]`.

- `phase2_sprint1_deep0` [BUILD_DEPENDENCY] produces Compilable Cargo workspace; module-root scaffolding only (no downstream implementation files) and exact dependency lockfile; inspect its accepted commit, exported artifact/test and `gate_phase2_sprint1_deep0`. If absent, do not start.

## 6. Inputs

Required: accepted outputs of all predecessors; frozen committed SSOT ranges above; actual source paths listed below. Optional context: existing UI mocks for fixtures only. No `report/` input; no historical research.

## 7. File ownership

```yaml

allowed_write_paths:

  - .github/workflows/verify.yml

  - Dockerfile

  - package.json

  - pnpm-lock.yaml

forbidden_write_paths:

  - docs/business/**

  - docs/db.dbml

  - docs/architecture/decisions/**

  - app-data/**

  - report/**

shared_read_only_paths:

  - react-router.config.ts

  - docs/business/be-core.md

generated_paths: []

```

Any allowed path may be absent at baseline (not only `src-tauri/`; new FE adapter/shim files are also future targets). Verify with `test -e`/`git ls-files` before editing; existing files are read first. Any extra path requires central integration approval, not a silent scope expansion.

## 8. Public interfaces

PROPOSED_INTERFACE: CI workflow and reproducible Docker/package scripts. Every consumed prerequisite interface is `PREREQUISITE_INTERFACE`, not an existing runtime guarantee. If a named signature conflicts with committed SSOT, stop `CONTRACT_BLOCKED`.

## 9. Implementation requirements

- Use official pnpm/Tauri CLI compatible with Node/Rust versions; inspect generated changes.

- Wire lint/typecheck/test/build and cargo test/check jobs once tools exist.

- Prefer official compatible CLI/tooling over manual generated files; verify installed version and diff. No external LLM/Jev request without authorization; optional Jev advice never overrides SSOT.

## 10. Domain invariants

- Repo has pnpm lock but Dockerfile assumes missing package-lock; repair package-manager choice only here.

- CI uses isolated temp DB/target, no secrets; do not run production migrations or edit shared Cargo.

- No partial state escapes; protected provenance, explicit authorization, no external-effect recovery replay where applicable.

## 11. Failure matrix

| Trigger | Expected behavior / classification | Durable effects and recovery | Required test |

|---|---|---|---|

| Missing platform dependencies | explicit CI failure and documented prerequisite; P6S1-N. | No unauthorized partial commit; re-read actual durable state before retry | phase6_sprint1_deep0-F1 |

| Docker tries npm ci without package-lock | must fail fixture until corrected; P6S1-C. | No unauthorized partial commit; re-read actual durable state before retry | phase6_sprint1_deep0-F2 |

| Missing accepted predecessor / baseline mismatch | `CONTRACT_BLOCKED`; do not run implementation | No user-data effects; record issue and owner in sprint evidence | phase6_sprint1_deep0-F0 |

## 12. Acceptance tests

Tests below name **future targets**, not tests executed during planning. Each row must become an executable test before the sprint can be accepted.

| Test ID | Initial state | Input / action | Expected output and state | Forbidden side effects / invariant |

|---|---|---|---|---|

| P6S1-P | accepted integrated worktree or isolated CI checkout | In the owned test target, set up the fixture and run the scenario: CI commands use pnpm lockfile and disposable Rust test DB. | CI commands use pnpm lockfile and disposable Rust test DB. | No unrelated write; preserve exact expected state and listed invariant |

| P6S1-N | accepted integrated worktree or isolated CI checkout | In the owned test target, set up the fixture and run the scenario: workflow contains no production DB path or secret. | workflow contains no production DB path or secret. | No unrelated write; preserve exact expected state and listed invariant |

| P6S1-B | accepted integrated worktree or isolated CI checkout | In the owned test target, set up the fixture and run the scenario: package build passes in isolated runner after implementation. | package build passes in isolated runner after implementation. | No unrelated write; preserve exact expected state and listed invariant |

| phase6_sprint1_deep0-F1 | accepted prerequisite fixture / isolated worktree | Inject Missing platform dependencies into owned operation | explicit CI failure and documented prerequisite; P6S1-N. | No unauthorized commit or external replay |

| phase6_sprint1_deep0-F2 | accepted prerequisite fixture / isolated worktree | Inject Docker tries npm ci without package-lock into owned operation | must fail fixture until corrected; P6S1-C. | No unauthorized commit or external replay |

| phase6_sprint1_deep0-F0 | predecessor gate absent or baseline SHA drift | Attempt dispatch before predecessor accepted | CONTRACT_BLOCKED; no implementation started | No source/DB modification |

Future command after the target files exist: `pnpm test && pnpm typecheck && pnpm lint && pnpm build && cargo test --manifest-path src-tauri/Cargo.toml  # future target`.

## 13. Database and migration safety

No persistent database in this sprint; fixture DB (if any) is disposable and uniquely named per worktree.

## 14. Concurrency contract

- Peer `phase6_sprint2_deep0`: candidate parallel only after frozen contracts/gates; NONE: Disjoint exact write paths; shared types frozen by predecessors; worktree-local test resources.

Isolation: own worktree/branch from baseline or accepted integration SHA; unique temp root, test DB and port; no shared mutable process/global fixture. Same depth alone is NOT permission to dispatch.

## 15. Deliverables

- [ ] CI workflow and reproducible Docker/package scripts

- [ ] Tests above and evidence including command exit codes, scope diff and accepted commit SHA.

## 16. Completion criteria

`SPRINT_IMPLEMENTED` requires artifacts, `SPRINT_TESTED` requires executed acceptance tests, `SPRINT_INTEGRATED` requires merged checks, and **`SPRINT_ACCEPTED` requires `gate_phase6_sprint1_deep0` accepted**. A contract-evidence sprint cannot be ACCEPTED for an affected implementation successor while its owner decision is unresolved. No agent declaration alone unlocks successors. All required assertions, file scope and baseline must pass.

## 17. Handoff contract

- CI workflow and reproducible Docker/package scripts

- Exact artifacts: .github/workflows/verify.yml, Dockerfile, package.json, pnpm-lock.yaml; accepted integration SHA and `gate_phase6_sprint1_deep0` record. A successor must verify exports/ABI rather than infer from prose.

## 18. Resume and failure behavior

Not started: verify baseline and deps. Partial/test failure: inspect git status, changed paths and durable test DB, resume only safe incomplete steps. Implemented but unintegrated: do not dispatch successors. Interrupted agent/rejected patch: preserve branch, rebase only after review, rerun tests. Specification conflict: `CONTRACT_BLOCKED`, record evidence, request authority resolution; never edit SSOT in this sprint.


---

Subagent budget: 256K tokens maximum; start with the bounded references above, do not read entire SSOT by default. CLI-first with verified tool compatibility. Treat Jev/TypeSafe as optional advisory only when available and authorized; never send private repository content externally without consent.
