---
sprint_id: phase3_sprint4_deep1
title: Implement catalog queries and read tools
phase: 3
sprint: 4
deep: 1
baseline_commit: 57a462efb953a665e3673b15e6cc33622a4f2d39
status: PLANNED
---

# phase3_sprint4_deep1 — Implement catalog queries and read tools

## 1. Sprint identity

Engineering phase 3 (Draft domain and durable workflow), within-phase depth 1; baseline is the Git commit above, not an uncommitted workspace. **PROPOSED, NOT IMPLEMENTED.**

## 2. Objective

Implement catalog queries and read tools. Observable handoff: Frozen ToolRegistry metadata and read tool schema/results.

## 3. Scope boundaries

**IN_SCOPE:** Implement read tools and registry metadata, catalog repo interface from P2S6.; Test empty array vs null, pagination/catalog ranking, caller denial.

**OUT_OF_SCOPE / MUST NOT:** modify authoritative `docs/business/**`, `docs/db.dbml`, `docs/architecture/decisions/**`; use real application data; run production migration; add client request IDs to SF-06; invent business semantics; rewrite another sprint’s files.

## 4. Authoritative contract snapshot

- Registry freezes metadata and caller gates; read tools never create Draft Version.

- Null-field questions cap at three and system_language comes from trusted settings boundary, not model args.

- **REQ-TOOLS-003** `docs/business/harness/tools.md#L40-L50`: Read tools and catalog searches MUST use registry caller gate and no Draft Version. (planning coverage `ASSIGNED`).

  Normative excerpt (verbatim baseline lines, truncated to 300 chars; use `sed` for the full range):

  > L40: ### 3.1 Tool đọc và tra cứu

  > L42: / Tool (tên ví dụ) / Client / Loại / Bảng DBML / Tham số (theo DBML) / Ghi chú /

  > L43: / :--- / :--- / :--- / :--- / :--- / :--- /

  > L44: / `get_field_null` / AI / Đọc / State (không có bảng) / `limit?` (tối đa 3) / Trả tối đa 3 cặp `field`, `question` cố định theo `system_language` do backend biết; AI không truyền ngôn ngữ (core 3.1, 3.8). /

**Targeted source references (read only these ranges unless a test proves broader context is necessary):**

- `docs/business/harness/tools.md#L9-L24` — focus: Tool metadata and caller categories; CLI: `sed -n "9,24p" docs/business/harness/tools.md` (or `rg -n "<symbol>" docs/business/harness/tools.md` for a changed anchor).

- `docs/business/harness/tools.md#L38-L50` — focus: Null-field, state and lookup tools; CLI: `sed -n "38,50p" docs/business/harness/tools.md` (or `rg -n "<symbol>" docs/business/harness/tools.md` for a changed anchor).

- `docs/business/harness/tools.md#L76-L85` — focus: Phase3–6 catalog lookups; CLI: `sed -n "76,85p" docs/business/harness/tools.md` (or `rg -n "<symbol>" docs/business/harness/tools.md` for a changed anchor).

- `docs/business/prompts/overview.md#L276-L310` — focus: JIT retrieval guardrails; CLI: `sed -n "276,310p" docs/business/prompts/overview.md` (or `rg -n "<symbol>" docs/business/prompts/overview.md` for a changed anchor).

## 5. Prerequisites

`depends_on: ["phase3_sprint1_deep0", "phase2_sprint6_deep2"]`.

- `phase3_sprint1_deep0` [RUNTIME_DEPENDENCY] produces plan_outline_runs/split_balanced/plan_phase and progress/validation pure APIs; inspect its accepted commit, exported artifact/test and `gate_phase3_sprint1_deep0`. If absent, do not start.

- `phase2_sprint6_deep2` [INTEGRATION_DEPENDENCY] produces Frozen DraftRepo/CatalogRepo/WorkflowReadRepo interfaces and temp-store API; inspect its accepted commit, exported artifact/test and `gate_phase2_sprint6_deep2`. If absent, do not start.

## 6. Inputs

Required: accepted outputs of all predecessors; frozen committed SSOT ranges above; actual source paths listed below. Optional context: existing UI mocks for fixtures only. No `report/` input; no historical research.

## 7. File ownership

```yaml

allowed_write_paths:

  - src-tauri/src/domain/tools/registry.rs

  - src-tauri/src/domain/tools/read_tools.rs

  - src-tauri/tests/read_tools.rs

forbidden_write_paths:

  - docs/business/**

  - docs/db.dbml

  - docs/architecture/decisions/**

  - app-data/**

  - report/**

shared_read_only_paths:

  - docs/business/harness/tools.md

  - docs/business/prompts/overview.md

generated_paths: []

```

Any allowed path may be absent at baseline (not only `src-tauri/`; new FE adapter/shim files are also future targets). Verify with `test -e`/`git ls-files` before editing; existing files are read first. Any extra path requires central integration approval, not a silent scope expansion.

## 8. Public interfaces

PROPOSED_INTERFACE: ToolRegistry::lookup(meta), read_tools::{get_field_null,get_array_element_fields,get_state,search_*}; no direct mutation. Every consumed prerequisite interface is `PREREQUISITE_INTERFACE`, not an existing runtime guarantee. If a named signature conflicts with committed SSOT, stop `CONTRACT_BLOCKED`.

## 9. Implementation requirements

- Implement read tools and registry metadata, catalog repo interface from P2S6.

- Test empty array vs null, pagination/catalog ranking, caller denial.

- Prefer official compatible CLI/tooling over manual generated files; verify installed version and diff. No external LLM/Jev request without authorization; optional Jev advice never overrides SSOT.

## 10. Domain invariants

- Registry freezes metadata and caller gates; read tools never create Draft Version.

- Null-field questions cap at three and system_language comes from trusted settings boundary, not model args.

- No partial state escapes; protected provenance, explicit authorization, no external-effect recovery replay where applicable.

## 11. Failure matrix

| Trigger | Expected behavior / classification | Durable effects and recovery | Required test |

|---|---|---|---|

| Unknown catalog code | structured candidates, no mutation; P3S4-N. | No unauthorized partial commit; re-read actual durable state before retry | phase3_sprint4_deep1-F1 |

| Missing translator locale | documented en fallback, no provider call; P3S4-C. | No unauthorized partial commit; re-read actual durable state before retry | phase3_sprint4_deep1-F2 |

| Missing accepted predecessor / baseline mismatch | `CONTRACT_BLOCKED`; do not run implementation | No user-data effects; record issue and owner in sprint evidence | phase3_sprint4_deep1-F0 |

## 12. Acceptance tests

Tests below name **future targets**, not tests executed during planning. Each row must become an executable test before the sprint can be accepted.

| Test ID | Initial state | Input / action | Expected output and state | Forbidden side effects / invariant |

|---|---|---|---|---|

| P3S4-P | pure Rust fixture (no DB/network/Tauri) + accepted predecessor types | In the owned test target, set up the fixture and run the scenario: get_field_null returns <=3 deterministic questions. | get_field_null returns <=3 deterministic questions. | No unrelated write; preserve exact expected state and listed invariant |

| P3S4-N | pure Rust fixture (no DB/network/Tauri) + accepted predecessor types | In the owned test target, set up the fixture and run the scenario: AI cannot request UI-only confirm/lock via registry. | AI cannot request UI-only confirm/lock via registry. | No unrelated write; preserve exact expected state and listed invariant |

| P3S4-B | pure Rust fixture (no DB/network/Tauri) + accepted predecessor types | In the owned test target, set up the fixture and run the scenario: empty array and missing field differ. | empty array and missing field differ. | No unrelated write; preserve exact expected state and listed invariant |

| phase3_sprint4_deep1-F1 | accepted prerequisite fixture / isolated worktree | Inject Unknown catalog code into owned operation | structured candidates, no mutation; P3S4-N. | No unauthorized commit or external replay |

| phase3_sprint4_deep1-F2 | accepted prerequisite fixture / isolated worktree | Inject Missing translator locale into owned operation | documented en fallback, no provider call; P3S4-C. | No unauthorized commit or external replay |

| phase3_sprint4_deep1-F0 | predecessor gate absent or baseline SHA drift | Attempt dispatch before predecessor accepted | CONTRACT_BLOCKED; no implementation started | No source/DB modification |

Backend module wiring rule: this sprint must not edit any `mod.rs`/`lib.rs`. Its integration test loads each owned implementation file via `#[path = "..."] mod <name>;` and fixture modules via the same mechanism. Production registration occurs only in P6S2 after this sprint is accepted. A compile success is therefore scoped to this test target, not a claim that the whole crate is wired.

Future command after the target files exist: `cargo test --manifest-path src-tauri/Cargo.toml --test read_tools  # future target, after bootstrap`.

## 13. Database and migration safety

No persistent database in this sprint; fixture DB (if any) is disposable and uniquely named per worktree.

## 14. Concurrency contract

- Peer `phase3_sprint8_deep1`: candidate parallel only after frozen contracts/gates; NONE: Disjoint exact write paths; shared types frozen by predecessors; worktree-local test resources.

Isolation: own worktree/branch from baseline or accepted integration SHA; unique temp root, test DB and port; no shared mutable process/global fixture. Same depth alone is NOT permission to dispatch.

## 15. Deliverables

- [ ] Frozen ToolRegistry metadata and read tool schema/results

- [ ] Tests above and evidence including command exit codes, scope diff and accepted commit SHA.

## 16. Completion criteria

`SPRINT_IMPLEMENTED` requires artifacts, `SPRINT_TESTED` requires executed acceptance tests, `SPRINT_INTEGRATED` requires merged checks, and **`SPRINT_ACCEPTED` requires `gate_phase3_sprint4_deep1` accepted**. A contract-evidence sprint cannot be ACCEPTED for an affected implementation successor while its owner decision is unresolved. No agent declaration alone unlocks successors. All required assertions, file scope and baseline must pass.

## 17. Handoff contract

- Frozen ToolRegistry metadata and read tool schema/results

- Exact artifacts: src-tauri/src/domain/tools/registry.rs, src-tauri/src/domain/tools/read_tools.rs, src-tauri/tests/read_tools.rs; accepted integration SHA and `gate_phase3_sprint4_deep1` record. A successor must verify exports/ABI rather than infer from prose.

## 18. Resume and failure behavior

Not started: verify baseline and deps. Partial/test failure: inspect git status, changed paths and durable test DB, resume only safe incomplete steps. Implemented but unintegrated: do not dispatch successors. Interrupted agent/rejected patch: preserve branch, rebase only after review, rerun tests. Specification conflict: `CONTRACT_BLOCKED`, record evidence, request authority resolution; never edit SSOT in this sprint.


---

Subagent budget: 256K tokens maximum; start with the bounded references above, do not read entire SSOT by default. CLI-first with verified tool compatibility. Treat Jev/TypeSafe as optional advisory only when available and authorized; never send private repository content externally without consent.
