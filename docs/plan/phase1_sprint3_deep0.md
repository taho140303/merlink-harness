---
sprint_id: phase1_sprint3_deep0
title: Validate Phase-3 producer-consumer artifact contract
phase: 1
sprint: 3
deep: 0
baseline_commit: 57a462efb953a665e3673b15e6cc33622a4f2d39
status: PLANNED
---

# phase1_sprint3_deep0 — Validate Phase-3 producer-consumer artifact contract

## 1. Sprint identity

Engineering phase 1 (Contract validation), within-phase depth 0; baseline is the Git commit above, not an uncommitted workspace. **PROPOSED, NOT IMPLEMENTED.**

## 2. Objective

Validate Phase-3 producer-consumer artifact contract. Observable handoff: docs/plan/evidence/phase3-artifact-contract.md; optional approved fixture schema with owner signature/commit.

## 3. Scope boundaries

**IN_SCOPE:** Define an executable fixture schema from approved consumer contract or record inability, without inventing lore fields.; Require source probe refs/hash/branch tests, budget absent/oversize fail-closed tests, byte-exact consumed input test.

**OUT_OF_SCOPE / MUST NOT:** modify authoritative `docs/business/**`, `docs/db.dbml`, `docs/architecture/decisions/**`; use real application data; run production migration; add client request IDs to SF-06; invent business semantics; rewrite another sprint’s files.

## 4. Authoritative contract snapshot

- WorldProbeReportV1.reportText and WorldWriteInputV1.consumedInput are not proven to match an existing consumer: none exists at HEAD.

- No temporary report, historical merger or LLM replay can stand in for missing durable input; preserve OC-1B minimum.

- **REQ-CONTRACT-003** `docs/business/be-core.md#L1549-L1549`: Verify actual Phase3 consumer projection and artifact budget before freezing codec. (planning coverage `CONTRACT_VALIDATION_REQUIRED`).

  Normative excerpt (verbatim baseline lines, truncated to 300 chars; use `sed` for the full range):

  > L1549: **Artifact OC-1B:** `world_probe` Phase 3 khi report của probe được gộp/tiêu thụ downstream bắt buộc persist `WorldProbeReportV1 = {schemaVersion:1, codecVersion, assignedScope, reportText}` (UTF-8 string đúng nội dung report địa hình/tổ chức/lore; JCS UTF-8 + SHA-256 với domain `merlink.sf06.artifa

**Targeted source references (read only these ranges unless a test proves broader context is necessary):**

- `docs/business/core.md#L943-L945` — focus: Probe, pure merge/tree, parent-before-child write; CLI: `sed -n "943,945p" docs/business/core.md` (or `rg -n "<symbol>" docs/business/core.md` for a changed anchor).

- `docs/business/be-core.md#L1200-L1207` — focus: world_write scope projection; CLI: `sed -n "1200,1207p" docs/business/be-core.md` (or `rg -n "<symbol>" docs/business/be-core.md` for a changed anchor).

- `docs/business/be-core.md#L1549-L1549` — focus: Two typed artifacts and actual consumer caveat; CLI: `sed -n "1549,1549p" docs/business/be-core.md` (or `rg -n "<symbol>" docs/business/be-core.md` for a changed anchor).

- `docs/architecture/decisions/sf06-owner-ratification.md#L81-L83` — focus: Unverified consumer projection; CLI: `sed -n "81,83p" docs/architecture/decisions/sf06-owner-ratification.md` (or `rg -n "<symbol>" docs/architecture/decisions/sf06-owner-ratification.md` for a changed anchor).

## 5. Prerequisites

`depends_on: []`.

- None within the implementation DAG; first verify baseline SHA, clean/isolated worktree, platform tools and owner/ADR status.

## 6. Inputs

Required: accepted outputs of all predecessors; frozen committed SSOT ranges above; actual source paths listed below. Optional context: existing UI mocks for fixtures only. No `report/` input; no historical research.

## 7. File ownership

```yaml

allowed_write_paths:

  - docs/plan/evidence/phase3-artifact-contract.md

forbidden_write_paths:

  - docs/business/**

  - docs/db.dbml

  - docs/architecture/decisions/**

  - app-data/**

  - report/**

shared_read_only_paths:

  - docs/business/core.md

  - docs/business/be-core.md

  - docs/architecture/decisions/sf06-owner-ratification.md

generated_paths: []

```

Any allowed path may be absent at baseline (not only `src-tauri/`; new FE adapter/shim files are also future targets). Verify with `test -e`/`git ls-files` before editing; existing files are read first. Any extra path requires central integration approval, not a silent scope expansion.

## 8. Public interfaces

ARTIFACT_CONSUMER_FIXTURE {WorldProbeReportV1, WorldWriteInputV1, exact_input_hash, probe_refs, approved_revision}. Every consumed prerequisite interface is `PREREQUISITE_INTERFACE`, not an existing runtime guarantee. If a named signature conflicts with committed SSOT, stop `CONTRACT_BLOCKED`.

## 9. Implementation requirements

- Define an executable fixture schema from approved consumer contract or record inability, without inventing lore fields.

- Require source probe refs/hash/branch tests, budget absent/oversize fail-closed tests, byte-exact consumed input test.

- Prefer official compatible CLI/tooling over manual generated files; verify installed version and diff. No external LLM/Jev request without authorization; optional Jev advice never overrides SSOT.

## 10. Domain invariants

- WorldProbeReportV1.reportText and WorldWriteInputV1.consumedInput are not proven to match an existing consumer: none exists at HEAD.

- No temporary report, historical merger or LLM replay can stand in for missing durable input; preserve OC-1B minimum.

- No partial state escapes; protected provenance, explicit authorization, no external-effect recovery replay where applicable.

## 11. Failure matrix

| Trigger | Expected behavior / classification | Durable effects and recovery | Required test |

|---|---|---|---|

| No implemented consumer/approved fixture | CONTRACT_BLOCKED; no guessed report shape; test P1S3-N. | No unauthorized partial commit; re-read actual durable state before retry | phase1_sprint3_deep0-F1 |

| Unknown artifact codec | RequiresReplan, never reinterpret; test P1S3-C. | No unauthorized partial commit; re-read actual durable state before retry | phase1_sprint3_deep0-F2 |

| Missing accepted predecessor / baseline mismatch | `CONTRACT_BLOCKED`; do not run implementation | No user-data effects; record issue and owner in sprint evidence | phase1_sprint3_deep0-F0 |

## 12. Acceptance tests

Tests below name **future targets**, not tests executed during planning. Each row must become an executable test before the sprint can be accepted.

| Test ID | Initial state | Input / action | Expected output and state | Forbidden side effects / invariant |

|---|---|---|---|---|

| P1S3-P | accepted baseline + owner evidence worktree; no prior sprint acceptance | In the owned test target, set up the fixture and run the scenario: fixture round-trips exact consumed JSON and JCS digest against declared contract. | fixture round-trips exact consumed JSON and JCS digest against declared contract. | No unrelated write; preserve exact expected state and listed invariant |

| P1S3-N | accepted baseline + owner evidence worktree; no prior sprint acceptance | In the owned test target, set up the fixture and run the scenario: absent consumer evidence marks CONTRACT_BLOCKED, not PASS. | absent consumer evidence marks CONTRACT_BLOCKED, not PASS. | No unrelated write; preserve exact expected state and listed invariant |

| P1S3-B | accepted baseline + owner evidence worktree; no prior sprint acceptance | In the owned test target, set up the fixture and run the scenario: missing/oversize artifact has no completed-round claim. | missing/oversize artifact has no completed-round claim. | No unrelated write; preserve exact expected state and listed invariant |

| phase1_sprint3_deep0-F1 | accepted prerequisite fixture / isolated worktree | Inject No implemented consumer/approved fixture into owned operation | CONTRACT_BLOCKED; no guessed report shape; test P1S3-N. | No unauthorized commit or external replay |

| phase1_sprint3_deep0-F2 | accepted prerequisite fixture / isolated worktree | Inject Unknown artifact codec into owned operation | RequiresReplan, never reinterpret; test P1S3-C. | No unauthorized commit or external replay |

| phase1_sprint3_deep0-F0 | predecessor gate absent or baseline SHA drift | Attempt dispatch before predecessor accepted | CONTRACT_BLOCKED; no implementation started | No source/DB modification |

Contract evidence must include `CONTRACT_STATUS: ACCEPTED` (or `INVENTORY_ACCEPTED` only for inventory; descendants still blocked on affected issues), `AUTHORITY_SHA`, `ISSUE_ID`, `SOURCE_ANCHOR`, and `DISPOSITION`; the validator checks format, but a human/owner must verify the approval SHA. A ledger alone cannot unblock SQL, artifact codec or changed product policy.

Future command after the target files exist: `python3 docs/plan/validate_plan.py --contract-evidence docs/plan/evidence/phase3-artifact-contract.md  # future target after authority/evidence file exists`.

## 13. Database and migration safety

No persistent database in this sprint; fixture DB (if any) is disposable and uniquely named per worktree.

## 14. Concurrency contract

- Peer `phase1_sprint1_deep0`: candidate parallel only after frozen contracts/gates; NONE: Disjoint exact write paths; shared types frozen by predecessors; worktree-local test resources.

Isolation: own worktree/branch from baseline or accepted integration SHA; unique temp root, test DB and port; no shared mutable process/global fixture. Same depth alone is NOT permission to dispatch.

## 15. Deliverables

- [ ] docs/plan/evidence/phase3-artifact-contract.md; optional approved fixture schema with owner signature/commit

- [ ] Tests above and evidence including command exit codes, scope diff and accepted commit SHA.

## 16. Completion criteria

`SPRINT_IMPLEMENTED` requires artifacts, `SPRINT_TESTED` requires executed acceptance tests, `SPRINT_INTEGRATED` requires merged checks, and **`SPRINT_ACCEPTED` requires `gate_phase1_sprint3_deep0` accepted**. A contract-evidence sprint cannot be ACCEPTED for an affected implementation successor while its owner decision is unresolved. No agent declaration alone unlocks successors. All required assertions, file scope and baseline must pass.

## 17. Handoff contract

- docs/plan/evidence/phase3-artifact-contract.md; optional approved fixture schema with owner signature/commit

- Exact artifacts: docs/plan/evidence/phase3-artifact-contract.md; accepted integration SHA and `gate_phase1_sprint3_deep0` record. A successor must verify exports/ABI rather than infer from prose.

## 18. Resume and failure behavior

Not started: verify baseline and deps. Partial/test failure: inspect git status, changed paths and durable test DB, resume only safe incomplete steps. Implemented but unintegrated: do not dispatch successors. Interrupted agent/rejected patch: preserve branch, rebase only after review, rerun tests. Specification conflict: `CONTRACT_BLOCKED`, record evidence, request authority resolution; never edit SSOT in this sprint.

**Explicit unblock requirements:** PHASE3_CONSUMER_SHAPE, CI-02, CONTRACT_REVIEW_REQUIRED:REQ-CONTRACT-003.


---

Subagent budget: 256K tokens maximum; start with the bounded references above, do not read entire SSOT by default. CLI-first with verified tool compatibility. Treat Jev/TypeSafe as optional advisory only when available and authorized; never send private repository content externally without consent.
