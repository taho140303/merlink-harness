---
sprint_id: phase1_sprint1_deep0
title: Reconcile blocking SSOT contradictions
phase: 1
sprint: 1
deep: 0
baseline_commit: 57a462efb953a665e3673b15e6cc33622a4f2d39
status: PLANNED
---

# phase1_sprint1_deep0 — Reconcile blocking SSOT contradictions

## 1. Sprint identity

Engineering phase 1 (Contract validation), within-phase depth 0; baseline is the Git commit above, not an uncommitted workspace. **PROPOSED, NOT IMPLEMENTED.**

## 2. Objective

Reconcile blocking SSOT contradictions. Observable handoff: docs/plan/evidence/contract-ledger.md with issue IDs, accepted SSOT SHA or CONTRACT_BLOCKED and affected sprint IDs.

## 3. Scope boundaries

**IN_SCOPE:** Write a source-anchored decision ledger and a dependency-specific unblock map.; Verify each resolution against owner/Core/DBML and freeze named interfaces only after an authorized SSOT revision.

**OUT_OF_SCOPE / MUST NOT:** modify authoritative `docs/business/**`, `docs/db.dbml`, `docs/architecture/decisions/**`; use real application data; run production migration; add client request IDs to SF-06; invent business semantics; rewrite another sprint’s files.

## 4. Authoritative contract snapshot

- Inventory exact contradictions: seal_pending_retry, TX01 pending expiry, restore dedupe/nullable phase, ErrorCode, TX05 counts/order, TX06 failure ordering, theme OPEN, scene-version restore IPC, extra durable commands outside TX inventory, system_language source, provider configuration persistence.

- Do not decide business/schema policy: record owner disposition, amended committed authority SHA and compatibility evidence; unresolved entries remain CONTRACT_BLOCKED.

- **REQ-CONTRACT-001** `docs/architecture/decisions/sf06-owner-ratification.md#L70-L83`: Resolve listed cross-SSOT contradictions only through owner authority, never coding-agent invention. (planning coverage `CONTRACT_VALIDATION_REQUIRED`).

  Normative excerpt (verbatim baseline lines, truncated to 300 chars; use `sed` for the full range):

  > L71: ## 5. Contradictions resolved; conditions before implementation

  > L73: 1. D09 header-vs-version digest contradiction resolved without manifest table; no claim of same client request or exact original LLM outcome.

  > L74: 2. D10 RFC staging proposal is **obsolete historical research**; owner chose independent TX-08, and TX-08 never rolls back merely because TX-04 fails.

  > L75: 3. Old sealed-Sn output note corrected for **new v2** to Si; current applicability remains a separate check, especially on Restore.

**Targeted source references (read only these ranges unless a test proves broader context is necessary):**

- `docs/architecture/decisions/sf06-owner-ratification.md#L3-L17` — focus: Owner choices and specification-only status; CLI: `sed -n "3,17p" docs/architecture/decisions/sf06-owner-ratification.md` (or `rg -n "<symbol>" docs/architecture/decisions/sf06-owner-ratification.md` for a changed anchor).

- `docs/business/be-core.md#L175-L221` — focus: ErrorCode surface; CLI: `sed -n "175,221p" docs/business/be-core.md` (or `rg -n "<symbol>" docs/business/be-core.md` for a changed anchor).

- `docs/business/be-core.md#L493-L506` — focus: Eight TX inventory; CLI: `sed -n "493,506p" docs/business/be-core.md` (or `rg -n "<symbol>" docs/business/be-core.md` for a changed anchor).

- `docs/db.dbml#L958-L997` — focus: Theme enum/default mismatch; CLI: `sed -n "958,997p" docs/db.dbml` (or `rg -n "<symbol>" docs/db.dbml` for a changed anchor).

## 5. Prerequisites

`depends_on: []`.

- None within the implementation DAG; first verify baseline SHA, clean/isolated worktree, platform tools and owner/ADR status.

## 6. Inputs

Required: accepted outputs of all predecessors; frozen committed SSOT ranges above; actual source paths listed below. Optional context: existing UI mocks for fixtures only. No `report/` input; no historical research.

## 7. File ownership

```yaml

allowed_write_paths:

  - docs/plan/evidence/contract-ledger.md

forbidden_write_paths:

  - docs/business/**

  - docs/db.dbml

  - docs/architecture/decisions/**

  - app-data/**

  - report/**

shared_read_only_paths:

  - docs/architecture/decisions/sf06-owner-ratification.md

  - docs/business/be-core.md

  - docs/db.dbml

generated_paths: []

```

Any allowed path may be absent at baseline (not only `src-tauri/`; new FE adapter/shim files are also future targets). Verify with `test -e`/`git ls-files` before editing; existing files are read first. Any extra path requires central integration approval, not a silent scope expansion.

## 8. Public interfaces

CONTRACT_EVIDENCE {issue_id, old_authority, proposed_disposition, owner_approval_sha, affected_sprints}; no implementation interface. Every consumed prerequisite interface is `PREREQUISITE_INTERFACE`, not an existing runtime guarantee. If a named signature conflicts with committed SSOT, stop `CONTRACT_BLOCKED`.

## 9. Implementation requirements

- Write a source-anchored decision ledger and a dependency-specific unblock map.

- Verify each resolution against owner/Core/DBML and freeze named interfaces only after an authorized SSOT revision.

- Prefer official compatible CLI/tooling over manual generated files; verify installed version and diff. No external LLM/Jev request without authorization; optional Jev advice never overrides SSOT.

## 10. Domain invariants

- Inventory exact contradictions: seal_pending_retry, TX01 pending expiry, restore dedupe/nullable phase, ErrorCode, TX05 counts/order, TX06 failure ordering, theme OPEN, scene-version restore IPC, extra durable commands outside TX inventory, system_language source, provider configuration persistence.

- Do not decide business/schema policy: record owner disposition, amended committed authority SHA and compatibility evidence; unresolved entries remain CONTRACT_BLOCKED.

- No partial state escapes; protected provenance, explicit authorization, no external-effect recovery replay where applicable.

## 11. Failure matrix

| Trigger | Expected behavior / classification | Durable effects and recovery | Required test |

|---|---|---|---|

| Missing owner disposition | CONTRACT_BLOCKED; zero implementation writes; resume with new SSOT SHA; test P1S1-N. | No unauthorized partial commit; re-read actual durable state before retry | phase1_sprint1_deep0-F1 |

| Conflicting SSOT revision | no silent precedence beyond authority hierarchy; preserve both claims; test P1S1-C. | No unauthorized partial commit; re-read actual durable state before retry | phase1_sprint1_deep0-F2 |

| Missing accepted predecessor / baseline mismatch | `CONTRACT_BLOCKED`; do not run implementation | No user-data effects; record issue and owner in sprint evidence | phase1_sprint1_deep0-F0 |

## 12. Acceptance tests

Tests below name **future targets**, not tests executed during planning. Each row must become an executable test before the sprint can be accepted.

| Test ID | Initial state | Input / action | Expected output and state | Forbidden side effects / invariant |

|---|---|---|---|---|

| P1S1-P | accepted baseline + owner evidence worktree; no prior sprint acceptance | In the owned test target, set up the fixture and run the scenario: given two conflicting source ranges, ledger lists old/new contract, owner and accepted revision; no SSOT edits. | given two conflicting source ranges, ledger lists old/new contract, owner and accepted revision; no SSOT edits. | No unrelated write; preserve exact expected state and listed invariant |

| P1S1-N | accepted baseline + owner evidence worktree; no prior sprint acceptance | In the owned test target, set up the fixture and run the scenario: unresolved theme OPEN or seal_pending_retry yields CONTRACT_BLOCKED and prevents only affected successors. | unresolved theme OPEN or seal_pending_retry yields CONTRACT_BLOCKED and prevents only affected successors. | No unrelated write; preserve exact expected state and listed invariant |

| P1S1-B | accepted baseline + owner evidence worktree; no prior sprint acceptance | In the owned test target, set up the fixture and run the scenario: clean baseline unchanged; evidence file can be regenerated idempotently from same SHA. | clean baseline unchanged; evidence file can be regenerated idempotently from same SHA. | No unrelated write; preserve exact expected state and listed invariant |

| phase1_sprint1_deep0-F1 | accepted prerequisite fixture / isolated worktree | Inject Missing owner disposition into owned operation | CONTRACT_BLOCKED; zero implementation writes; resume with new SSOT SHA; test P1S1-N. | No unauthorized commit or external replay |

| phase1_sprint1_deep0-F2 | accepted prerequisite fixture / isolated worktree | Inject Conflicting SSOT revision into owned operation | no silent precedence beyond authority hierarchy; preserve both claims; test P1S1-C. | No unauthorized commit or external replay |

| phase1_sprint1_deep0-F0 | predecessor gate absent or baseline SHA drift | Attempt dispatch before predecessor accepted | CONTRACT_BLOCKED; no implementation started | No source/DB modification |

Contract evidence must include `CONTRACT_STATUS: ACCEPTED` (or `INVENTORY_ACCEPTED` only for inventory; descendants still blocked on affected issues), `AUTHORITY_SHA`, `ISSUE_ID`, `SOURCE_ANCHOR`, and `DISPOSITION`; the validator checks format, but a human/owner must verify the approval SHA. A ledger alone cannot unblock SQL, artifact codec or changed product policy.

Future command after the target files exist: `python3 docs/plan/validate_plan.py --contract-evidence docs/plan/evidence/contract-ledger.md  # future target after authority/evidence file exists`.

## 13. Database and migration safety

No persistent database in this sprint; fixture DB (if any) is disposable and uniquely named per worktree.

## 14. Concurrency contract

- Peer `phase1_sprint3_deep0`: candidate parallel only after frozen contracts/gates; NONE: Disjoint exact write paths; shared types frozen by predecessors; worktree-local test resources.

Isolation: own worktree/branch from baseline or accepted integration SHA; unique temp root, test DB and port; no shared mutable process/global fixture. Same depth alone is NOT permission to dispatch.

## 15. Deliverables

- [ ] docs/plan/evidence/contract-ledger.md with issue IDs, accepted SSOT SHA or CONTRACT_BLOCKED and affected sprint IDs

- [ ] Tests above and evidence including command exit codes, scope diff and accepted commit SHA.

## 16. Completion criteria

`SPRINT_IMPLEMENTED` requires artifacts, `SPRINT_TESTED` requires executed acceptance tests, `SPRINT_INTEGRATED` requires merged checks, and **`SPRINT_ACCEPTED` requires `gate_phase1_sprint1_deep0` accepted**. A contract-evidence sprint cannot be ACCEPTED for an affected implementation successor while its owner decision is unresolved. No agent declaration alone unlocks successors. All required assertions, file scope and baseline must pass.

**Contract issue ledger (open):** CI-01 = Migration baseline 0001/0002 absent; version path unapproved → phase1_sprint2_deep1, phase2_sprint3_deep1; CI-02 = Phase-3 merger/consumer exact projection unproven → phase1_sprint3_deep0, phase3_sprint3_deep0, phase3_sprint9_deep2, phase3_sprint10_deep4, phase4_sprint1_deep0; CI-03 = seal_pending_retry/TX01 pending expiry/restore idempotency/nullable phase → phase3_sprint2_deep0; CI-04 = ErrorCode variants incomplete for IPC/error contracts → phase2_sprint4_deep3, phase2_sprint5_deep2; CI-05 = TX-05 table count/insert order differs from Core → phase4_sprint2_deep0; CI-06 = TX-06 cancel-before-persist failure ordering → phase4_sprint4_deep1; CI-07 = themes.resolution_type default OPEN not in enum → phase2_sprint3_deep1; CI-08 = No authoritative scene-version restore IPC → phase5_sprint5_deep3; CI-09 = system_language request-vs-settings precedence → phase5_sprint2_deep1; CI-10 = Provider config persistence semantics missing → phase2_sprint4_deep3, phase5_sprint6_deep1; CI-11 = B+ target represented as unconditional vs spec-only authority → phase2_sprint3_deep1, phase3_sprint9_deep2. This sprint may be gate-accepted as `INVENTORY_ACCEPTED` after the ledger and owner-escalation map exist; that acceptance does **not** clear any issue. Each listed successor also carries the issue IDs in its manifest `contract_blockers` and must stop until the owner decision is recorded.

## 17. Handoff contract

- docs/plan/evidence/contract-ledger.md with issue IDs, accepted SSOT SHA or CONTRACT_BLOCKED and affected sprint IDs

- Exact artifacts: docs/plan/evidence/contract-ledger.md; accepted integration SHA and `gate_phase1_sprint1_deep0` record. A successor must verify exports/ABI rather than infer from prose.

## 18. Resume and failure behavior

Not started: verify baseline and deps. Partial/test failure: inspect git status, changed paths and durable test DB, resume only safe incomplete steps. Implemented but unintegrated: do not dispatch successors. Interrupted agent/rejected patch: preserve branch, rebase only after review, rerun tests. Specification conflict: `CONTRACT_BLOCKED`, record evidence, request authority resolution; never edit SSOT in this sprint.

**Explicit unblock requirements:** CONTRACT_REVIEW_REQUIRED:REQ-CONTRACT-001.


---

Subagent budget: 256K tokens maximum; start with the bounded references above, do not read entire SSOT by default. CLI-first with verified tool compatibility. Treat Jev/TypeSafe as optional advisory only when available and authorized; never send private repository content externally without consent.
