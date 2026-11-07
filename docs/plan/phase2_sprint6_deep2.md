---
sprint_id: phase2_sprint6_deep2
title: Implement isolated base repositories
phase: 2
sprint: 6
deep: 2
baseline_commit: 57a462efb953a665e3673b15e6cc33622a4f2d39
status: PLANNED
---

# phase2_sprint6_deep2 — Implement isolated base repositories

## 1. Sprint identity

Engineering phase 2 (Foundation and frozen interfaces), within-phase depth 2; baseline is the Git commit above, not an uncommitted workspace. **PROPOSED, NOT IMPLEMENTED.**

## 2. Objective

Implement isolated base repositories. Observable handoff: Frozen DraftRepo/CatalogRepo/WorkflowReadRepo interfaces and temp-store API.

## 3. Scope boundaries

**IN_SCOPE:** Implement query interfaces consumed by draft/tools/verifier; no schema changes, no shared migration files.; Test read-only snapshot and path traversal with disposable DB/root.

**OUT_OF_SCOPE / MUST NOT:** modify authoritative `docs/business/**`, `docs/db.dbml`, `docs/architecture/decisions/**`; use real application data; run production migration; add client request IDs to SF-06; invent business semantics; rewrite another sprint’s files.

## 4. Authoritative contract snapshot

- Base repositories expose typed transaction-scoped draft/version/catalog/round read APIs; no TX01/04/08 ownership here.

- Temp store must be under configured root with UUID path validation and cannot provide durable proof.

**Targeted source references (read only these ranges unless a test proves broader context is necessary):**

- `docs/business/be-core.md#L93-L102` — focus: Storage module ownership; CLI: `sed -n "93,102p" docs/business/be-core.md` (or `rg -n "<symbol>" docs/business/be-core.md` for a changed anchor).

- `docs/business/be-core.md#L361-L382` — focus: SQLite connections and lock order; CLI: `sed -n "361,382p" docs/business/be-core.md` (or `rg -n "<symbol>" docs/business/be-core.md` for a changed anchor).

- `docs/business/be-core.md#L887-L899` — focus: Audit/event log; CLI: `sed -n "887,899p" docs/business/be-core.md` (or `rg -n "<symbol>" docs/business/be-core.md` for a changed anchor).

- `docs/business/be-core.md#L1549-L1549` — focus: Temp reports nonauthoritative; CLI: `sed -n "1549,1549p" docs/business/be-core.md` (or `rg -n "<symbol>" docs/business/be-core.md` for a changed anchor).

## 5. Prerequisites

`depends_on: ["phase2_sprint2_deep1", "phase2_sprint3_deep1"]`.

- `phase2_sprint2_deep1` [TYPE_DEPENDENCY] produces Frozen BusinessFlowData/TrackedField/CanonicalStateBytes APIs and pure test target; inspect its accepted commit, exported artifact/test and `gate_phase2_sprint2_deep1`. If absent, do not start.

- `phase2_sprint3_deep1` [MIGRATION_DEPENDENCY] produces Versioned SQL filenames, DbManager, schema manifest and disposable migration tests; inspect its accepted commit, exported artifact/test and `gate_phase2_sprint3_deep1`. If absent, do not start.

## 6. Inputs

Required: accepted outputs of all predecessors; frozen committed SSOT ranges above; actual source paths listed below. Optional context: existing UI mocks for fixtures only. No `report/` input; no historical research.

## 7. File ownership

```yaml

allowed_write_paths:

  - src-tauri/src/storage/draft_repo.rs

  - src-tauri/src/storage/catalog_repo.rs

  - src-tauri/src/storage/workflow_repo.rs

  - src-tauri/src/storage/temp_store.rs

  - src-tauri/tests/base_repo.rs

forbidden_write_paths:

  - docs/business/**

  - docs/db.dbml

  - docs/architecture/decisions/**

  - app-data/**

  - report/**

shared_read_only_paths:

  - docs/business/be-core.md

generated_paths: []

```

Any allowed path may be absent at baseline (not only `src-tauri/`; new FE adapter/shim files are also future targets). Verify with `test -e`/`git ls-files` before editing; existing files are read first. Any extra path requires central integration approval, not a silent scope expansion.

## 8. Public interfaces

PROPOSED_INTERFACE: DraftRepo, CatalogRepo, WorkflowReadRepo transaction-scoped query traits; TempStore does not prove completion. Every consumed prerequisite interface is `PREREQUISITE_INTERFACE`, not an existing runtime guarantee. If a named signature conflicts with committed SSOT, stop `CONTRACT_BLOCKED`.

## 9. Implementation requirements

- Implement query interfaces consumed by draft/tools/verifier; no schema changes, no shared migration files.

- Test read-only snapshot and path traversal with disposable DB/root.

- Prefer official compatible CLI/tooling over manual generated files; verify installed version and diff. No external LLM/Jev request without authorization; optional Jev advice never overrides SSOT.

## 10. Domain invariants

- Base repositories expose typed transaction-scoped draft/version/catalog/round read APIs; no TX01/04/08 ownership here.

- Temp store must be under configured root with UUID path validation and cannot provide durable proof.

- No partial state escapes; protected provenance, explicit authorization, no external-effect recovery replay where applicable.

## 11. Failure matrix

| Trigger | Expected behavior / classification | Durable effects and recovery | Required test |

|---|---|---|---|

| Missing schema | block until P2S3 gate; no fallback JSON database, P2S6-N. | No unauthorized partial commit; re-read actual durable state before retry | phase2_sprint6_deep2-F1 |

| Traversal/foreign-draft query | typed reject, zero writes, P2S6-C. | No unauthorized partial commit; re-read actual durable state before retry | phase2_sprint6_deep2-F2 |

| Missing accepted predecessor / baseline mismatch | `CONTRACT_BLOCKED`; do not run implementation | No user-data effects; record issue and owner in sprint evidence | phase2_sprint6_deep2-F0 |

## 12. Acceptance tests

Tests below name **future targets**, not tests executed during planning. Each row must become an executable test before the sprint can be accepted.

| Test ID | Initial state | Input / action | Expected output and state | Forbidden side effects / invariant |

|---|---|---|---|---|

| P2S6-P | disposable uniquely-named SQLite S0 + accepted predecessor fixture | In the owned test target, set up the fixture and run the scenario: repositories return exact scoped records from disposable seeded DB. | repositories return exact scoped records from disposable seeded DB. | No unrelated write; preserve exact expected state and listed invariant |

| P2S6-N | disposable uniquely-named SQLite S0 + accepted predecessor fixture | In the owned test target, set up the fixture and run the scenario: ../ path rejected; temp report cannot mark task complete. | ../ path rejected; temp report cannot mark task complete. | No unrelated write; preserve exact expected state and listed invariant |

| P2S6-B | disposable uniquely-named SQLite S0 + accepted predecessor fixture | In the owned test target, set up the fixture and run the scenario: injected SQL error does not expose partial result. | injected SQL error does not expose partial result. | No unrelated write; preserve exact expected state and listed invariant |

| phase2_sprint6_deep2-F1 | accepted prerequisite fixture / isolated worktree | Inject Missing schema into owned operation | block until P2S3 gate; no fallback JSON database, P2S6-N. | No unauthorized commit or external replay |

| phase2_sprint6_deep2-F2 | accepted prerequisite fixture / isolated worktree | Inject Traversal/foreign-draft query into owned operation | typed reject, zero writes, P2S6-C. | No unauthorized commit or external replay |

| phase2_sprint6_deep2-F0 | predecessor gate absent or baseline SHA drift | Attempt dispatch before predecessor accepted | CONTRACT_BLOCKED; no implementation started | No source/DB modification |

Backend module wiring rule: this sprint must not edit any `mod.rs`/`lib.rs`. Its integration test loads each owned implementation file via `#[path = "..."] mod <name>;` and fixture modules via the same mechanism. Production registration occurs only in P6S2 after this sprint is accepted. A compile success is therefore scoped to this test target, not a claim that the whole crate is wired.

Future command after the target files exist: `cargo test --manifest-path src-tauri/Cargo.toml --test base_repo  # future target, after bootstrap`.

## 13. Database and migration safety

Use `BEGIN IMMEDIATE` only for the assigned TX; WAL/FULL/FK ON/busy_timeout as specified. Use unique disposable DB/temp root per worktree. Never open a user application DB; no destructive migration without separate authorization. Backup/rollback and PRAGMA user_version are owned by P2S3. TX04 never rolls back committed TX08.

## 14. Concurrency contract

- Peer `phase2_sprint5_deep2`: candidate parallel only after frozen contracts/gates; NONE: Disjoint exact write paths; shared types frozen by predecessors; worktree-local test resources.

Isolation: own worktree/branch from baseline or accepted integration SHA; unique temp root, test DB and port; no shared mutable process/global fixture. Same depth alone is NOT permission to dispatch.

## 15. Deliverables

- [ ] Frozen DraftRepo/CatalogRepo/WorkflowReadRepo interfaces and temp-store API

- [ ] Tests above and evidence including command exit codes, scope diff and accepted commit SHA.

## 16. Completion criteria

`SPRINT_IMPLEMENTED` requires artifacts, `SPRINT_TESTED` requires executed acceptance tests, `SPRINT_INTEGRATED` requires merged checks, and **`SPRINT_ACCEPTED` requires `gate_phase2_sprint6_deep2` accepted**. A contract-evidence sprint cannot be ACCEPTED for an affected implementation successor while its owner decision is unresolved. No agent declaration alone unlocks successors. All required assertions, file scope and baseline must pass.

## 17. Handoff contract

- Frozen DraftRepo/CatalogRepo/WorkflowReadRepo interfaces and temp-store API

- Exact artifacts: src-tauri/src/storage/draft_repo.rs, src-tauri/src/storage/catalog_repo.rs, src-tauri/src/storage/workflow_repo.rs, src-tauri/src/storage/temp_store.rs, src-tauri/tests/base_repo.rs; accepted integration SHA and `gate_phase2_sprint6_deep2` record. A successor must verify exports/ABI rather than infer from prose.

## 18. Resume and failure behavior

Not started: verify baseline and deps. Partial/test failure: inspect git status, changed paths and durable test DB, resume only safe incomplete steps. Implemented but unintegrated: do not dispatch successors. Interrupted agent/rejected patch: preserve branch, rebase only after review, rerun tests. Specification conflict: `CONTRACT_BLOCKED`, record evidence, request authority resolution; never edit SSOT in this sprint.


---

Subagent budget: 256K tokens maximum; start with the bounded references above, do not read entire SSOT by default. CLI-first with verified tool compatibility. Treat Jev/TypeSafe as optional advisory only when available and authorized; never send private repository content externally without consent.
