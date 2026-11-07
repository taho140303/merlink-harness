---
sprint_id: phase3_sprint3_deep0
title: Validate and encode Phase-3 artifacts
phase: 3
sprint: 3
deep: 0
baseline_commit: 57a462efb953a665e3673b15e6cc33622a4f2d39
status: PLANNED
---

# phase3_sprint3_deep0 — Validate and encode Phase-3 artifacts

## 1. Sprint identity

Engineering phase 3 (Draft domain and durable workflow), within-phase depth 0; baseline is the Git commit above, not an uncommitted workspace. **PROPOSED, NOT IMPLEMENTED.**

## 2. Objective

Validate and encode Phase-3 artifacts. Observable handoff: Versioned WorldProbeReportV1/WorldWriteInputV1 and executable fixture proof.

## 3. Scope boundaries

**IN_SCOPE:** Implement type/codec and fixture comparing producer→consumer exact JSON; if no actual consumer exists require owner-approved fixture before test.; Test absent budget, oversized, unknown version, mismatched ref, restart fingerprint without merger replay.

**OUT_OF_SCOPE / MUST NOT:** modify authoritative `docs/business/**`, `docs/db.dbml`, `docs/architecture/decisions/**`; use real application data; run production migration; add client request IDs to SF-06; invent business semantics; rewrite another sprint’s files.

## 4. Authoritative contract snapshot

- Build only after P1S3 approved consumer projection; store exact typed probe output and consumed write input, not all LLM responses.

- Both artifacts versioned/JCS/size-budgeted, probeRefs same draft/branch; historical verifier reads durable write input, not temp report or old merger.

- **REQ-ART-001** `docs/business/be-core.md#L1549-L1549`: WorldProbeReportV1 and exact WorldWriteInputV1 consumer projection with refs, budget and hashes. (planning coverage `CONTRACT_VALIDATION_REQUIRED`).

  Normative excerpt (verbatim baseline lines, truncated to 300 chars; use `sed` for the full range):

  > L1549: **Artifact OC-1B:** `world_probe` Phase 3 khi report của probe được gộp/tiêu thụ downstream bắt buộc persist `WorldProbeReportV1 = {schemaVersion:1, codecVersion, assignedScope, reportText}` (UTF-8 string đúng nội dung report địa hình/tổ chức/lore; JCS UTF-8 + SHA-256 với domain `merlink.sf06.artifa

**Targeted source references (read only these ranges unless a test proves broader context is necessary):**

- `docs/business/core.md#L943-L945` — focus: Probe→pure merge/tree→write; CLI: `sed -n "943,945p" docs/business/core.md` (or `rg -n "<symbol>" docs/business/core.md` for a changed anchor).

- `docs/business/be-core.md#L1200-L1207` — focus: world_write projection; CLI: `sed -n "1200,1207p" docs/business/be-core.md` (or `rg -n "<symbol>" docs/business/be-core.md` for a changed anchor).

- `docs/business/be-core.md#L1549-L1549` — focus: WorldProbeReportV1/WorldWriteInputV1; CLI: `sed -n "1549,1549p" docs/business/be-core.md` (or `rg -n "<symbol>" docs/business/be-core.md` for a changed anchor).

- `docs/architecture/decisions/sf06-owner-ratification.md#L81-L83` — focus: Consumer shape caveat; CLI: `sed -n "81,83p" docs/architecture/decisions/sf06-owner-ratification.md` (or `rg -n "<symbol>" docs/architecture/decisions/sf06-owner-ratification.md` for a changed anchor).

## 5. Prerequisites

`depends_on: ["phase1_sprint3_deep0", "phase2_sprint2_deep1"]`.

- `phase1_sprint3_deep0` [DATA_CONTRACT_DEPENDENCY] produces docs/plan/evidence/phase3-artifact-contract.md; optional approved fixture schema with owner signature/commit; inspect its accepted commit, exported artifact/test and `gate_phase1_sprint3_deep0`. If absent, do not start.

- `phase2_sprint2_deep1` [TYPE_DEPENDENCY] produces Frozen BusinessFlowData/TrackedField/CanonicalStateBytes APIs and pure test target; inspect its accepted commit, exported artifact/test and `gate_phase2_sprint2_deep1`. If absent, do not start.

## 6. Inputs

Required: accepted outputs of all predecessors; frozen committed SSOT ranges above; actual source paths listed below. Optional context: existing UI mocks for fixtures only. No `report/` input; no historical research.

## 7. File ownership

```yaml

allowed_write_paths:

  - src-tauri/src/domain/phase3_artifact.rs

  - src-tauri/tests/fixtures/phase3_artifacts.json

  - src-tauri/tests/phase3_artifact_contract.rs

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

PROPOSED_INTERFACE: WorldProbeReportV1 {schemaVersion,codecVersion,assignedScope,reportText}; WorldWriteInputV1 {schemaVersion,codecVersion,assignedScope,consumedInput,probeRefs}. Every consumed prerequisite interface is `PREREQUISITE_INTERFACE`, not an existing runtime guarantee. If a named signature conflicts with committed SSOT, stop `CONTRACT_BLOCKED`.

## 9. Implementation requirements

- Implement type/codec and fixture comparing producer→consumer exact JSON; if no actual consumer exists require owner-approved fixture before test.

- Test absent budget, oversized, unknown version, mismatched ref, restart fingerprint without merger replay.

- Prefer official compatible CLI/tooling over manual generated files; verify installed version and diff. No external LLM/Jev request without authorization; optional Jev advice never overrides SSOT.

## 10. Domain invariants

- Build only after P1S3 approved consumer projection; store exact typed probe output and consumed write input, not all LLM responses.

- Both artifacts versioned/JCS/size-budgeted, probeRefs same draft/branch; historical verifier reads durable write input, not temp report or old merger.

- No partial state escapes; protected provenance, explicit authorization, no external-effect recovery replay where applicable.

## 11. Failure matrix

| Trigger | Expected behavior / classification | Durable effects and recovery | Required test |

|---|---|---|---|

| Absent consumer contract | CONTRACT_BLOCKED, no invented schema; P3S3-N. | No unauthorized partial commit; re-read actual durable state before retry | phase3_sprint3_deep0-F1 |

| Missing artifact or budget | reject seal/replan, no fabricated completion; P3S3-C. | No unauthorized partial commit; re-read actual durable state before retry | phase3_sprint3_deep0-F2 |

| Missing accepted predecessor / baseline mismatch | `CONTRACT_BLOCKED`; do not run implementation | No user-data effects; record issue and owner in sprint evidence | phase3_sprint3_deep0-F0 |

## 12. Acceptance tests

Tests below name **future targets**, not tests executed during planning. Each row must become an executable test before the sprint can be accepted.

| Test ID | Initial state | Input / action | Expected output and state | Forbidden side effects / invariant |

|---|---|---|---|---|

| P3S3-P | pure Rust fixture (no DB/network/Tauri) + accepted predecessor types | In the owned test target, set up the fixture and run the scenario: consumedInput fixture byte-matches consumer input and hash. | consumedInput fixture byte-matches consumer input and hash. | No unrelated write; preserve exact expected state and listed invariant |

| P3S3-N | pure Rust fixture (no DB/network/Tauri) + accepted predecessor types | In the owned test target, set up the fixture and run the scenario: missing/oversize rejects with no committed round. | missing/oversize rejects with no committed round. | No unrelated write; preserve exact expected state and listed invariant |

| P3S3-B | pure Rust fixture (no DB/network/Tauri) + accepted predecessor types | In the owned test target, set up the fixture and run the scenario: restore-diverged probeRef cannot authorize world_write. | restore-diverged probeRef cannot authorize world_write. | No unrelated write; preserve exact expected state and listed invariant |

| phase3_sprint3_deep0-F1 | accepted prerequisite fixture / isolated worktree | Inject Absent consumer contract into owned operation | CONTRACT_BLOCKED, no invented schema; P3S3-N. | No unauthorized commit or external replay |

| phase3_sprint3_deep0-F2 | accepted prerequisite fixture / isolated worktree | Inject Missing artifact or budget into owned operation | reject seal/replan, no fabricated completion; P3S3-C. | No unauthorized commit or external replay |

| phase3_sprint3_deep0-F0 | predecessor gate absent or baseline SHA drift | Attempt dispatch before predecessor accepted | CONTRACT_BLOCKED; no implementation started | No source/DB modification |

Backend module wiring rule: this sprint must not edit any `mod.rs`/`lib.rs`. Its integration test loads each owned implementation file via `#[path = "..."] mod <name>;` and fixture modules via the same mechanism. Production registration occurs only in P6S2 after this sprint is accepted. A compile success is therefore scoped to this test target, not a claim that the whole crate is wired.

Future command after the target files exist: `cargo test --manifest-path src-tauri/Cargo.toml --test phase3_artifacts  # future target, after bootstrap`.

## 13. Database and migration safety

No persistent database in this sprint; fixture DB (if any) is disposable and uniquely named per worktree.

## 14. Concurrency contract

- Peer `phase3_sprint1_deep0`: candidate parallel only after frozen contracts/gates; NONE: Disjoint exact write paths; shared types frozen by predecessors; worktree-local test resources.

- Peer `phase3_sprint2_deep0`: candidate parallel only after frozen contracts/gates; NONE: Disjoint exact write paths; shared types frozen by predecessors; worktree-local test resources.

Isolation: own worktree/branch from baseline or accepted integration SHA; unique temp root, test DB and port; no shared mutable process/global fixture. Same depth alone is NOT permission to dispatch.

## 15. Deliverables

- [ ] Versioned WorldProbeReportV1/WorldWriteInputV1 and executable fixture proof

- [ ] Tests above and evidence including command exit codes, scope diff and accepted commit SHA.

## 16. Completion criteria

`SPRINT_IMPLEMENTED` requires artifacts, `SPRINT_TESTED` requires executed acceptance tests, `SPRINT_INTEGRATED` requires merged checks, and **`SPRINT_ACCEPTED` requires `gate_phase3_sprint3_deep0` accepted**. A contract-evidence sprint cannot be ACCEPTED for an affected implementation successor while its owner decision is unresolved. No agent declaration alone unlocks successors. All required assertions, file scope and baseline must pass.

## 17. Handoff contract

- Versioned WorldProbeReportV1/WorldWriteInputV1 and executable fixture proof

- Exact artifacts: src-tauri/src/domain/phase3_artifact.rs, src-tauri/tests/fixtures/phase3_artifacts.json, src-tauri/tests/phase3_artifact_contract.rs; accepted integration SHA and `gate_phase3_sprint3_deep0` record. A successor must verify exports/ABI rather than infer from prose.

## 18. Resume and failure behavior

Not started: verify baseline and deps. Partial/test failure: inspect git status, changed paths and durable test DB, resume only safe incomplete steps. Implemented but unintegrated: do not dispatch successors. Interrupted agent/rejected patch: preserve branch, rebase only after review, rerun tests. Specification conflict: `CONTRACT_BLOCKED`, record evidence, request authority resolution; never edit SSOT in this sprint.

**Explicit unblock requirements:** PHASE3_CONSUMER_SHAPE, CI-02, CONTRACT_REVIEW_REQUIRED:REQ-ART-001.


---

Subagent budget: 256K tokens maximum; start with the bounded references above, do not read entire SSOT by default. CLI-first with verified tool compatibility. Treat Jev/TypeSafe as optional advisory only when available and authorized; never send private repository content externally without consent.
