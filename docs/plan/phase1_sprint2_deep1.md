---
sprint_id: phase1_sprint2_deep1
title: Validate migration provenance and baseline
phase: 1
sprint: 2
deep: 1
baseline_commit: 57a462efb953a665e3673b15e6cc33622a4f2d39
status: PLANNED
---

# phase1_sprint2_deep1 — Validate migration provenance and baseline

## 1. Sprint identity

Engineering phase 1 (Contract validation), within-phase depth 1; baseline is the Git commit above, not an uncommitted workspace. **PROPOSED, NOT IMPLEMENTED.**

## 2. Objective

Validate migration provenance and baseline. Observable handoff: docs/plan/evidence/migration-contract.md with authorized version path, 65→66 manifest provenance or blocker.

## 3. Scope boundaries

**IN_SCOPE:** Inspect git tree and supported app-data/migration state read-only; propose two paths for owner review, never execute or modify DB.; Record exact target 66-table manifest and legacy compatibility conditions, no production data access.

**OUT_OF_SCOPE / MUST NOT:** modify authoritative `docs/business/**`, `docs/db.dbml`, `docs/architecture/decisions/**`; use real application data; run production migration; add client request IDs to SF-06; invent business semantics; rewrite another sprint’s files.

## 4. Authoritative contract snapshot

- There are no committed SQL migrations at baseline; never label invented 0001/0002 as historical.

- Owner-authorized baseline manifest/version and forward-only upgrade path are required before any migration sprint can be accepted.

- **REQ-CONTRACT-002** `docs/business/be-core.md#L2621-L2628`: Verify absent historical migration baseline and authorize version path before SQL. (planning coverage `CONTRACT_VALIDATION_REQUIRED`).

  Normative excerpt (verbatim baseline lines, truncated to 300 chars; use `sed` for the full range):

  > L2621: - **Phạm vi (Scope):**

  > L2622: - Khởi tạo crate `src-tauri`, cấu trúc module `ipc`, `service`, `domain`, `storage`, `providers`.

  > L2623: - Khi được cấp quyền implementation sau task này: giữ các migration lịch sử `0001_initial_schema.sql`/`0002_seed_reference_data.sql` bất biến; bổ sung migration **forward-only B+** cho bảng thứ 66, child rebuild và nullable eligibility column; fresh bootstrap chạy tuần tự các bước tới đúng schema ta

  > L2624: - Cài đặt `DbManager` tại đúng đường dẫn chuẩn `<app_data_dir>/merlink-harness/db/merlink.sqlite3` (PRAGMAs bắt buộc: `WAL`, `foreign_keys = ON`, `busy_timeout = 5000`, `synchronous = FULL`), `storage::migration` sử dụng duy nhất `PRAGMA user_version` kèm kiểm chứng cấu trúc schema `verify_schema_an

**Targeted source references (read only these ranges unless a test proves broader context is necessary):**

- `docs/business/be-core.md#L415-L459` — focus: user_version and backup protocol; CLI: `sed -n "415,459p" docs/business/be-core.md` (or `rg -n "<symbol>" docs/business/be-core.md` for a changed anchor).

- `docs/business/be-core.md#L471-L491` — focus: 65 legacy vs 66 target verifier; CLI: `sed -n "471,491p" docs/business/be-core.md` (or `rg -n "<symbol>" docs/business/be-core.md` for a changed anchor).

- `docs/business/be-core.md#L2621-L2628` — focus: Roadmap assumes nonexistent 0001/0002; CLI: `sed -n "2621,2628p" docs/business/be-core.md` (or `rg -n "<symbol>" docs/business/be-core.md` for a changed anchor).

- `docs/db.dbml#L1299-L1378` — focus: B+ physical target; CLI: `sed -n "1299,1378p" docs/db.dbml` (or `rg -n "<symbol>" docs/db.dbml` for a changed anchor).

## 5. Prerequisites

`depends_on: ["phase1_sprint1_deep0"]`.

- `phase1_sprint1_deep0` [RUNTIME_DEPENDENCY] produces docs/plan/evidence/contract-ledger.md with issue IDs, accepted SSOT SHA or CONTRACT_BLOCKED and affected sprint IDs; inspect its accepted commit, exported artifact/test and `gate_phase1_sprint1_deep0`. If absent, do not start.

## 6. Inputs

Required: accepted outputs of all predecessors; frozen committed SSOT ranges above; actual source paths listed below. Optional context: existing UI mocks for fixtures only. No `report/` input; no historical research.

## 7. File ownership

```yaml

allowed_write_paths:

  - docs/plan/evidence/migration-contract.md

forbidden_write_paths:

  - docs/business/**

  - docs/db.dbml

  - docs/architecture/decisions/**

  - app-data/**

  - report/**

shared_read_only_paths:

  - docs/business/be-core.md

  - docs/db.dbml

generated_paths: []

```

Any allowed path may be absent at baseline (not only `src-tauri/`; new FE adapter/shim files are also future targets). Verify with `test -e`/`git ls-files` before editing; existing files are read first. Any extra path requires central integration approval, not a silent scope expansion.

## 8. Public interfaces

MIGRATION_DECISION {baseline_manifest_sha, version_path, backup_policy, authorized_revision}; absent approval = CONTRACT_BLOCKED. Every consumed prerequisite interface is `PREREQUISITE_INTERFACE`, not an existing runtime guarantee. If a named signature conflicts with committed SSOT, stop `CONTRACT_BLOCKED`.

## 9. Implementation requirements

- Inspect git tree and supported app-data/migration state read-only; propose two paths for owner review, never execute or modify DB.

- Record exact target 66-table manifest and legacy compatibility conditions, no production data access.

- Prefer official compatible CLI/tooling over manual generated files; verify installed version and diff. No external LLM/Jev request without authorization; optional Jev advice never overrides SSOT.

## 10. Domain invariants

- There are no committed SQL migrations at baseline; never label invented 0001/0002 as historical.

- Owner-authorized baseline manifest/version and forward-only upgrade path are required before any migration sprint can be accepted.

- No partial state escapes; protected provenance, explicit authorization, no external-effect recovery replay where applicable.

## 11. Failure matrix

| Trigger | Expected behavior / classification | Durable effects and recovery | Required test |

|---|---|---|---|

| Absent decision/legacy sample | CONTRACT_BLOCKED, no DB write; test P1S2-N. | No unauthorized partial commit; re-read actual durable state before retry | phase1_sprint2_deep1-F1 |

| Version mismatch | stop before migration and retain backup/user data; test P1S2-C. | No unauthorized partial commit; re-read actual durable state before retry | phase1_sprint2_deep1-F2 |

| Missing accepted predecessor / baseline mismatch | `CONTRACT_BLOCKED`; do not run implementation | No user-data effects; record issue and owner in sprint evidence | phase1_sprint2_deep1-F0 |

## 12. Acceptance tests

Tests below name **future targets**, not tests executed during planning. Each row must become an executable test before the sprint can be accepted.

| Test ID | Initial state | Input / action | Expected output and state | Forbidden side effects / invariant |

|---|---|---|---|---|

| P1S2-P | accepted baseline + owner evidence worktree; no prior sprint acceptance | In the owned test target, set up the fixture and run the scenario: verified git tree contains zero SQL files; review packet explicitly says missing historical baseline. | verified git tree contains zero SQL files; review packet explicitly says missing historical baseline. | No unrelated write; preserve exact expected state and listed invariant |

| P1S2-N | accepted baseline + owner evidence worktree; no prior sprint acceptance | In the owned test target, set up the fixture and run the scenario: cannot produce authorized baseline version from docs alone | cannot produce authorized baseline version from docs alone => CONTRACT_BLOCKED. | No unrelated write; preserve exact expected state and listed invariant |

| P1S2-B | accepted baseline + owner evidence worktree; no prior sprint acceptance | In the owned test target, set up the fixture and run the scenario: target manifest counts 66 names; no migration executed. | target manifest counts 66 names; no migration executed. | No unrelated write; preserve exact expected state and listed invariant |

| phase1_sprint2_deep1-F1 | accepted prerequisite fixture / isolated worktree | Inject Absent decision/legacy sample into owned operation | CONTRACT_BLOCKED, no DB write; test P1S2-N. | No unauthorized commit or external replay |

| phase1_sprint2_deep1-F2 | accepted prerequisite fixture / isolated worktree | Inject Version mismatch into owned operation | stop before migration and retain backup/user data; test P1S2-C. | No unauthorized commit or external replay |

| phase1_sprint2_deep1-F0 | predecessor gate absent or baseline SHA drift | Attempt dispatch before predecessor accepted | CONTRACT_BLOCKED; no implementation started | No source/DB modification |

Contract evidence must include `CONTRACT_STATUS: ACCEPTED` (or `INVENTORY_ACCEPTED` only for inventory; descendants still blocked on affected issues), `AUTHORITY_SHA`, `ISSUE_ID`, `SOURCE_ANCHOR`, and `DISPOSITION`; the validator checks format, but a human/owner must verify the approval SHA. A ledger alone cannot unblock SQL, artifact codec or changed product policy.

Future command after the target files exist: `python3 docs/plan/validate_plan.py --contract-evidence docs/plan/evidence/migration-contract.md  # future target after authority/evidence file exists`.

## 13. Database and migration safety

No persistent database in this sprint; fixture DB (if any) is disposable and uniquely named per worktree.

## 14. Concurrency contract

- No same-phase/same-depth peer. Cross-phase scheduling still requires exact predecessor gates and conflict scan.

Isolation: own worktree/branch from baseline or accepted integration SHA; unique temp root, test DB and port; no shared mutable process/global fixture. Same depth alone is NOT permission to dispatch.

## 15. Deliverables

- [ ] docs/plan/evidence/migration-contract.md with authorized version path, 65→66 manifest provenance or blocker

- [ ] Tests above and evidence including command exit codes, scope diff and accepted commit SHA.

## 16. Completion criteria

`SPRINT_IMPLEMENTED` requires artifacts, `SPRINT_TESTED` requires executed acceptance tests, `SPRINT_INTEGRATED` requires merged checks, and **`SPRINT_ACCEPTED` requires `gate_phase1_sprint2_deep1` accepted**. A contract-evidence sprint cannot be ACCEPTED for an affected implementation successor while its owner decision is unresolved. No agent declaration alone unlocks successors. All required assertions, file scope and baseline must pass.

## 17. Handoff contract

- docs/plan/evidence/migration-contract.md with authorized version path, 65→66 manifest provenance or blocker

- Exact artifacts: docs/plan/evidence/migration-contract.md; accepted integration SHA and `gate_phase1_sprint2_deep1` record. A successor must verify exports/ABI rather than infer from prose.

## 18. Resume and failure behavior

Not started: verify baseline and deps. Partial/test failure: inspect git status, changed paths and durable test DB, resume only safe incomplete steps. Implemented but unintegrated: do not dispatch successors. Interrupted agent/rejected patch: preserve branch, rebase only after review, rerun tests. Specification conflict: `CONTRACT_BLOCKED`, record evidence, request authority resolution; never edit SSOT in this sprint.

**Explicit unblock requirements:** OWNER_MIGRATION_PROVENANCE, CI-01, CONTRACT_REVIEW_REQUIRED:REQ-CONTRACT-002.


---

Subagent budget: 256K tokens maximum; start with the bounded references above, do not read entire SSOT by default. CLI-first with verified tool compatibility. Treat Jev/TypeSafe as optional advisory only when available and authorized; never send private repository content externally without consent.
