---
sprint_id: phase2_sprint1_deep0
title: Bootstrap Tauri workspace and module shells
phase: 2
sprint: 1
deep: 0
baseline_commit: 57a462efb953a665e3673b15e6cc33622a4f2d39
status: PLANNED
---

# phase2_sprint1_deep0 — Bootstrap Tauri workspace and module shells

## 1. Sprint identity

Engineering phase 2 (Foundation and frozen interfaces), within-phase depth 0; baseline is the Git commit above, not an uncommitted workspace. **PROPOSED, NOT IMPLEMENTED.**

## 2. Objective

Bootstrap Tauri workspace and module shells. Observable handoff: Compilable Cargo workspace; module-root scaffolding only (no downstream implementation files) and exact dependency lockfile.

## 3. Scope boundaries

**IN_SCOPE:** Check official Tauri CLI compatibility before scaffolding; inspect generated diff and restrict to allowed paths.; Use CLI to create crate and minimal compile smoke test; no second dev server or application DB.

**OUT_OF_SCOPE / MUST NOT:** modify authoritative `docs/business/**`, `docs/db.dbml`, `docs/architecture/decisions/**`; use real application data; run production migration; add client request IDs to SF-06; invent business semantics; rewrite another sprint’s files.

## 4. Authoritative contract snapshot

- Create a compilable minimal Tauri v2 shell; no DB, provider calls or production API effects.

- Freeze Cargo dependencies, module exports and generated binding ownership; any later dependency addition requires integration-owner change.

- Bootstrap must NOT create downstream implementation files or predeclare their modules. Each implementation sprint owns new .rs files and compiles them in its own integration test with `#[path = "..."] mod`; P6S2 alone adds production module declarations/registrations after acceptance. This avoids shared mod.rs writes and placeholder ownership.

- **REQ-ARCH-001** `docs/business/be-core.md#L35-L125`: Tauri v2 modular monolith; pure domain, one-way service/storage/provider dependency. (planning coverage `ASSIGNED`).

  Normative excerpt (verbatim baseline lines, truncated to 300 chars; use `sed` for the full range):

  > L35: ## §1. Backend Architecture (Kiến trúc tổng thể Backend)

  > L37: ### 1.1 Kiến trúc Modular Monolith trên Tauri v2

  > L39: Merlink Harness Backend `MUST` được tổ chức theo mô hình **Local-First Modular Monolith** viết bằng **Rust** bên trong tiến trình chính của **Tauri v2** (`src-tauri`). Hệ thống `MUST NOT` tách microservice, `MUST NOT` dùng message broker ngoài, `MUST NOT` dùng framework Event Sourcing hay CQRS phức

  > L41: ### 1.2 Cấu trúc thư mục và Module Rust (`src-tauri`)

**Targeted source references (read only these ranges unless a test proves broader context is necessary):**

- `docs/business/be-core.md#L35-L125` — focus: Module tree and one-way dependency direction; CLI: `sed -n "35,125p" docs/business/be-core.md` (or `rg -n "<symbol>" docs/business/be-core.md` for a changed anchor).

- `docs/business/be-core.md#L131-L174` — focus: Startup phases; no runtime claim; CLI: `sed -n "131,174p" docs/business/be-core.md` (or `rg -n "<symbol>" docs/business/be-core.md` for a changed anchor).

- `docs/business/fe-core.md#L34-L51` — focus: Current frontend-only baseline; CLI: `sed -n "34,51p" docs/business/fe-core.md` (or `rg -n "<symbol>" docs/business/fe-core.md` for a changed anchor).

- `package.json#L5-L11` — focus: Existing pnpm/RR scripts; CLI: `sed -n "5,11p" package.json` (or `rg -n "<symbol>" package.json` for a changed anchor).

## 5. Prerequisites

`depends_on: []`.

- None within the implementation DAG; first verify baseline SHA, clean/isolated worktree, platform tools and owner/ADR status.

## 6. Inputs

Required: accepted outputs of all predecessors; frozen committed SSOT ranges above; actual source paths listed below. Optional context: existing UI mocks for fixtures only. No `report/` input; no historical research.

## 7. File ownership

```yaml

allowed_write_paths:

  - src-tauri/Cargo.toml

  - src-tauri/Cargo.lock

  - src-tauri/tauri.conf.json

  - src-tauri/capabilities/default.json

  - src-tauri/src/main.rs

  - src-tauri/src/lib.rs

  - src-tauri/src/domain/mod.rs

  - src-tauri/src/storage/mod.rs

  - src-tauri/src/service/mod.rs

  - src-tauri/src/ipc/mod.rs

  - src-tauri/src/providers/mod.rs

  - src-tauri/tests/scaffold_smoke.rs

  - src-tauri/build.rs

  - src-tauri/.gitignore

  - src-tauri/icons/*

forbidden_write_paths:

  - docs/business/**

  - docs/db.dbml

  - docs/architecture/decisions/**

  - app-data/**

  - report/**

shared_read_only_paths:

  - docs/business/be-core.md

  - docs/business/fe-core.md

  - package.json

generated_paths: []

```

The bootstrap may create only empty compile-only placeholders for the generated stub paths above; each belongs to its future sprint owner, contains no business behavior, and must be replaced there. Bootstrap may not fill any other sprint’s implementation.

Any allowed path may be absent at baseline (not only `src-tauri/`; new FE adapter/shim files are also future targets). Verify with `test -e`/`git ls-files` before editing; existing files are read first. Any extra path requires central integration approval, not a silent scope expansion.

## 8. Public interfaces

EXISTING_INTERFACE: React Router build; PROPOSED_INTERFACE: minimal Tauri Builder and crate modules, no registered business command yet. Every consumed prerequisite interface is `PREREQUISITE_INTERFACE`, not an existing runtime guarantee. If a named signature conflicts with committed SSOT, stop `CONTRACT_BLOCKED`.

## 9. Implementation requirements

- Check official Tauri CLI compatibility before scaffolding; inspect generated diff and restrict to allowed paths.

- Use CLI to create crate and minimal compile smoke test; no second dev server or application DB.

- Prefer official compatible CLI/tooling over manual generated files; verify installed version and diff. No external LLM/Jev request without authorization; optional Jev advice never overrides SSOT.

## 10. Domain invariants

- Create a compilable minimal Tauri v2 shell; no DB, provider calls or production API effects.

- Freeze Cargo dependencies, module exports and generated binding ownership; any later dependency addition requires integration-owner change.

- Bootstrap must NOT create downstream implementation files or predeclare their modules. Each implementation sprint owns new .rs files and compiles them in its own integration test with `#[path = "..."] mod`; P6S2 alone adds production module declarations/registrations after acceptance. This avoids shared mod.rs writes and placeholder ownership.

- No partial state escapes; protected provenance, explicit authorization, no external-effect recovery replay where applicable.

## 11. Failure matrix

| Trigger | Expected behavior / classification | Durable effects and recovery | Required test |

|---|---|---|---|

| Unsupported platform/dependency | CONTRACT_BLOCKED with CLI error; no ad-hoc tool install; P2S1-N. | No unauthorized partial commit; re-read actual durable state before retry | phase2_sprint1_deep0-F1 |

| Generated out-of-scope file | reject generated patch; P2S1-C. | No unauthorized partial commit; re-read actual durable state before retry | phase2_sprint1_deep0-F2 |

| Missing accepted predecessor / baseline mismatch | `CONTRACT_BLOCKED`; do not run implementation | No user-data effects; record issue and owner in sprint evidence | phase2_sprint1_deep0-F0 |

## 12. Acceptance tests

Tests below name **future targets**, not tests executed during planning. Each row must become an executable test before the sprint can be accepted.

| Test ID | Initial state | Input / action | Expected output and state | Forbidden side effects / invariant |

|---|---|---|---|---|

| P2S1-P | clean scaffold worktree with no src-tauri and no application data | In the owned test target, set up the fixture and run the scenario: from disposable test target `cargo check --manifest-path src-tauri/Cargo.toml` succeeds after scaffold. | from disposable test target `cargo check --manifest-path src-tauri/Cargo.toml` succeeds after scaffold. | No unrelated write; preserve exact expected state and listed invariant |

| P2S1-N | clean scaffold worktree with no src-tauri and no application data | In the owned test target, set up the fixture and run the scenario: grep generated capabilities shows no blanket filesystem/shell permission. | grep generated capabilities shows no blanket filesystem/shell permission. | No unrelated write; preserve exact expected state and listed invariant |

| P2S1-B | clean scaffold worktree with no src-tauri and no application data | In the owned test target, set up the fixture and run the scenario: `git diff --name-only` contains only declared paths. | `git diff --name-only` contains only declared paths. | No unrelated write; preserve exact expected state and listed invariant |

| phase2_sprint1_deep0-F1 | accepted prerequisite fixture / isolated worktree | Inject Unsupported platform/dependency into owned operation | CONTRACT_BLOCKED with CLI error; no ad-hoc tool install; P2S1-N. | No unauthorized commit or external replay |

| phase2_sprint1_deep0-F2 | accepted prerequisite fixture / isolated worktree | Inject Generated out-of-scope file into owned operation | reject generated patch; P2S1-C. | No unauthorized commit or external replay |

| phase2_sprint1_deep0-F0 | predecessor gate absent or baseline SHA drift | Attempt dispatch before predecessor accepted | CONTRACT_BLOCKED; no implementation started | No source/DB modification |

Future command after the target files exist: `cargo test --manifest-path src-tauri/Cargo.toml --test scaffold_smoke  # future target, after bootstrap`.

## 13. Database and migration safety

Use `BEGIN IMMEDIATE` only for the assigned TX; WAL/FULL/FK ON/busy_timeout as specified. Use unique disposable DB/temp root per worktree. Never open a user application DB; no destructive migration without separate authorization. Backup/rollback and PRAGMA user_version are owned by P2S3. TX04 never rolls back committed TX08.

## 14. Concurrency contract

- No same-phase/same-depth peer. Cross-phase scheduling still requires exact predecessor gates and conflict scan.

Isolation: own worktree/branch from baseline or accepted integration SHA; unique temp root, test DB and port; no shared mutable process/global fixture. Same depth alone is NOT permission to dispatch.

## 15. Deliverables

- [ ] Compilable Cargo workspace; module-root scaffolding only (no downstream implementation files) and exact dependency lockfile

- [ ] Tests above and evidence including command exit codes, scope diff and accepted commit SHA.

## 16. Completion criteria

`SPRINT_IMPLEMENTED` requires artifacts, `SPRINT_TESTED` requires executed acceptance tests, `SPRINT_INTEGRATED` requires merged checks, and **`SPRINT_ACCEPTED` requires `gate_phase2_sprint1_deep0` accepted**. A contract-evidence sprint cannot be ACCEPTED for an affected implementation successor while its owner decision is unresolved. No agent declaration alone unlocks successors. All required assertions, file scope and baseline must pass.

## 17. Handoff contract

- Compilable Cargo workspace; module-root scaffolding only (no downstream implementation files) and exact dependency lockfile

- Exact artifacts: src-tauri/Cargo.toml, src-tauri/Cargo.lock, src-tauri/tauri.conf.json, src-tauri/capabilities/default.json, src-tauri/src/main.rs, src-tauri/src/lib.rs, src-tauri/src/domain/mod.rs, src-tauri/src/storage/mod.rs, src-tauri/src/service/mod.rs, src-tauri/src/ipc/mod.rs, src-tauri/src/providers/mod.rs, src-tauri/tests/scaffold_smoke.rs, src-tauri/build.rs, src-tauri/.gitignore, src-tauri/icons/*; accepted integration SHA and `gate_phase2_sprint1_deep0` record. A successor must verify exports/ABI rather than infer from prose.

## 18. Resume and failure behavior

Not started: verify baseline and deps. Partial/test failure: inspect git status, changed paths and durable test DB, resume only safe incomplete steps. Implemented but unintegrated: do not dispatch successors. Interrupted agent/rejected patch: preserve branch, rebase only after review, rerun tests. Specification conflict: `CONTRACT_BLOCKED`, record evidence, request authority resolution; never edit SSOT in this sprint.


---

Subagent budget: 256K tokens maximum; start with the bounded references above, do not read entire SSOT by default. CLI-first with verified tool compatibility. Treat Jev/TypeSafe as optional advisory only when available and authorized; never send private repository content externally without consent.
