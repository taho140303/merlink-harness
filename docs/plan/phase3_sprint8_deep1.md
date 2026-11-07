---
sprint_id: phase3_sprint8_deep1
title: Implement SF-06 pure patch/manifest codec
phase: 3
sprint: 8
deep: 1
baseline_commit: 57a462efb953a665e3673b15e6cc33622a4f2d39
status: PLANNED
---

# phase3_sprint8_deep1 — Implement SF-06 pure patch/manifest codec

## 1. Sprint identity

Engineering phase 3 (Draft domain and durable workflow), within-phase depth 1; baseline is the Git commit above, not an uncommitted workspace. **PROPOSED, NOT IMPLEMENTED.**

## 2. Objective

Implement SF-06 pure patch/manifest codec. Observable handoff: Pure codec/manifest API and GV1–12 fixtures.

## 3. Scope boundaries

**IN_SCOPE:** Implement typed v2 codec and golden/property tests (incl. insert+counter, cascade, order, missing/null).; Versioned decode rejects unknown codec; never normalize v1 evidence as v2.

**OUT_OF_SCOPE / MUST NOT:** modify authoritative `docs/business/**`, `docs/db.dbml`, `docs/architecture/decisions/**`; use real application data; run production migration; add client request IDs to SF-06; invent business semantics; rewrite another sprint’s files.

## 4. Authoritative contract snapshot

- Canonical plan digest from durable child tags and frozen P; request digest only durable header fields.

- All preconditions on S_before, apply clone atomically, validate final canonical bytes; no total op order or LLM fallback.

- **REQ-CODEC-002** `docs/business/be-core.md#L1528-L1532`: Versioned pure atomic clone patch, provenance/counters/arrays/cascade/noop/zero-net; golden vectors. (planning coverage `ASSIGNED`).

  Normative excerpt (verbatim baseline lines, truncated to 300 chars; use `sed` for the full range):

  > L1528: **Evidence typed v2:** Mỗi child `evidence_json` là JCS object có `schemaVersion=2`, `codecVersion`, `plannerVersion`, `policyVersion`, `taskSpec` (phase, subphase nullable, kind, assignedScope, dependencies, deterministic task identity), `preStateHash`, `postStateHash`, `hadAppliedWrites`, `effecti

  > L1530: **Pure codec v2:** `canonical_state(S)=JCS(normalize_supported_BusinessFlowData(S) minus meta.updated_at)`; `S ≡ T` khi và chỉ khi canonical bytes bằng nhau (không suy logic byte equality chỉ từ hai hash bằng nhau). Normalize optional `TrackedField` thiếu thành `{value:null,source:incomplete}` theo

  > L1532: **Golden vectors đặc tả (CHƯA CHẠY):** GV1 create Arc + `SetCounter(1→2)` ⇒ final code/counter nhất quán; GV2 tracked `{value:old,source:ai_generated}`→`{value:new,source:ai_generated}` bảo toàn source; GV3 delete Scene + dependent links + set nullable memory scene ref null ⇒ final valid bất kể seri

- **REQ-CODEC-003** `docs/business/be-core.md#L1512-L1528`: Durable canonical plan/request digests and frozen P; reject unsupported codecs. (planning coverage `ASSIGNED`).

  Normative excerpt (verbatim baseline lines, truncated to 300 chars; use `sed` for the full range):

  > L1512: **Canonical identity/digest (không có `client_request_id` mới).** Mỗi header chứa `(draft_session_id,workflow_run_id,plan_generation,round_index,round_seal_key,task_count,parallel_limit,plan_digest,request_digest,base_version_seq,base_data_hash,final_data_hash,resulting_version_seq)`. `round_seal_ke

  > L1514: ```text

  > L1515: plan_digest = "sf06:plan:v2:" + hex(SHA256(UTF8("merlink.sf06.plan.v2\0") // JCS({

  > L1516: draft_session_id, workflow_run_id, plan_generation, round_index,

**Targeted source references (read only these ranges unless a test proves broader context is necessary):**

- `docs/business/be-core.md#L1512-L1532` — focus: Digest formulas, canonical patch, GV1–12; CLI: `sed -n "1512,1532p" docs/business/be-core.md` (or `rg -n "<symbol>" docs/business/be-core.md` for a changed anchor).

- `docs/business/core.md#L974-L986` — focus: S(i−1)/Si and zero-net; CLI: `sed -n "974,986p" docs/business/core.md` (or `rg -n "<symbol>" docs/business/core.md` for a changed anchor).

- `docs/db.dbml#L1299-L1355` — focus: Header/evidence shape; CLI: `sed -n "1299,1355p" docs/db.dbml` (or `rg -n "<symbol>" docs/db.dbml` for a changed anchor).

- `docs/architecture/decisions/sf06-owner-ratification.md#L38-L42` — focus: No transient input in request digest; CLI: `sed -n "38,42p" docs/architecture/decisions/sf06-owner-ratification.md` (or `rg -n "<symbol>" docs/architecture/decisions/sf06-owner-ratification.md` for a changed anchor).

## 5. Prerequisites

`depends_on: ["phase3_sprint1_deep0", "phase3_sprint3_deep0"]`.

- `phase3_sprint1_deep0` [RUNTIME_DEPENDENCY] produces plan_outline_runs/split_balanced/plan_phase and progress/validation pure APIs; inspect its accepted commit, exported artifact/test and `gate_phase3_sprint1_deep0`. If absent, do not start.

- `phase3_sprint3_deep0` [DATA_CONTRACT_DEPENDENCY] produces Versioned WorldProbeReportV1/WorldWriteInputV1 and executable fixture proof; inspect its accepted commit, exported artifact/test and `gate_phase3_sprint3_deep0`. If absent, do not start.

## 6. Inputs

Required: accepted outputs of all predecessors; frozen committed SSOT ranges above; actual source paths listed below. Optional context: existing UI mocks for fixtures only. No `report/` input; no historical research.

## 7. File ownership

```yaml

allowed_write_paths:

  - src-tauri/src/domain/sf06/codec.rs

  - src-tauri/src/domain/sf06/manifest.rs

  - src-tauri/tests/sf06_golden.rs

forbidden_write_paths:

  - docs/business/**

  - docs/db.dbml

  - docs/architecture/decisions/**

  - app-data/**

  - report/**

shared_read_only_paths:

  - docs/business/be-core.md

  - docs/business/core.md

  - docs/db.dbml

  - docs/architecture/decisions/sf06-owner-ratification.md

generated_paths: []

```

Any allowed path may be absent at baseline (not only `src-tauri/`; new FE adapter/shim files are also future targets). Verify with `test -e`/`git ls-files` before editing; existing files are read first. Any extra path requires central integration approval, not a silent scope expansion.

## 8. Public interfaces

PROPOSED_INTERFACE: canonical_diff(S0,Sn)->PatchV2; replay(S0,PatchV2)->State; plan_digest, request_digest, codecVersion. Every consumed prerequisite interface is `PREREQUISITE_INTERFACE`, not an existing runtime guarantee. If a named signature conflicts with committed SSOT, stop `CONTRACT_BLOCKED`.

## 9. Implementation requirements

- Implement typed v2 codec and golden/property tests (incl. insert+counter, cascade, order, missing/null).

- Versioned decode rejects unknown codec; never normalize v1 evidence as v2.

- Prefer official compatible CLI/tooling over manual generated files; verify installed version and diff. No external LLM/Jev request without authorization; optional Jev advice never overrides SSOT.

## 10. Domain invariants

- Canonical plan digest from durable child tags and frozen P; request digest only durable header fields.

- All preconditions on S_before, apply clone atomically, validate final canonical bytes; no total op order or LLM fallback.

- No partial state escapes; protected provenance, explicit authorization, no external-effect recovery replay where applicable.

## 11. Failure matrix

| Trigger | Expected behavior / classification | Durable effects and recovery | Required test |

|---|---|---|---|

| Unsupported valid transition | EvidenceCodecUnsupported, no state publish; P3S8-N. | No unauthorized partial commit; re-read actual durable state before retry | phase3_sprint8_deep1-F1 |

| Same-path write or update deleted entity | reject; independent ref-null cascade accepted; P3S8-C. | No unauthorized partial commit; re-read actual durable state before retry | phase3_sprint8_deep1-F2 |

| Missing accepted predecessor / baseline mismatch | `CONTRACT_BLOCKED`; do not run implementation | No user-data effects; record issue and owner in sprint evidence | phase3_sprint8_deep1-F0 |

## 12. Acceptance tests

Tests below name **future targets**, not tests executed during planning. Each row must become an executable test before the sprint can be accepted.

| Test ID | Initial state | Input / action | Expected output and state | Forbidden side effects / invariant |

|---|---|---|---|---|

| P3S8-P | pure Rust fixture (no DB/network/Tauri) + accepted predecessor types | In the owned test target, set up the fixture and run the scenario: property replay(before,diff) has equal canonical bytes to after. | property replay(before,diff) has equal canonical bytes to after. | No unrelated write; preserve exact expected state and listed invariant |

| P3S8-N | pure Rust fixture (no DB/network/Tauri) + accepted predecessor types | In the owned test target, set up the fixture and run the scenario: malformed/unknown v2 codec fails closed without tool call. | malformed/unknown v2 codec fails closed without tool call. | No unrelated write; preserve exact expected state and listed invariant |

| P3S8-B | pure Rust fixture (no DB/network/Tauri) + accepted predecessor types | In the owned test target, set up the fixture and run the scenario: T1 1→2, T2 2→1 has two patches and no net version. | T1 1→2, T2 2→1 has two patches and no net version. | No unrelated write; preserve exact expected state and listed invariant |

| phase3_sprint8_deep1-F1 | accepted prerequisite fixture / isolated worktree | Inject Unsupported valid transition into owned operation | EvidenceCodecUnsupported, no state publish; P3S8-N. | No unauthorized commit or external replay |

| phase3_sprint8_deep1-F2 | accepted prerequisite fixture / isolated worktree | Inject Same-path write or update deleted entity into owned operation | reject; independent ref-null cascade accepted; P3S8-C. | No unauthorized commit or external replay |

| phase3_sprint8_deep1-F0 | predecessor gate absent or baseline SHA drift | Attempt dispatch before predecessor accepted | CONTRACT_BLOCKED; no implementation started | No source/DB modification |

Backend module wiring rule: this sprint must not edit any `mod.rs`/`lib.rs`. Its integration test loads each owned implementation file via `#[path = "..."] mod <name>;` and fixture modules via the same mechanism. Production registration occurs only in P6S2 after this sprint is accepted. A compile success is therefore scoped to this test target, not a claim that the whole crate is wired.

Future command after the target files exist: `cargo test --manifest-path src-tauri/Cargo.toml --test sf06_golden  # future target, after bootstrap`.

## 13. Database and migration safety

No persistent database in this sprint; fixture DB (if any) is disposable and uniquely named per worktree.

## 14. Concurrency contract

- Peer `phase3_sprint4_deep1`: candidate parallel only after frozen contracts/gates; NONE: Disjoint exact write paths; shared types frozen by predecessors; worktree-local test resources.

Isolation: own worktree/branch from baseline or accepted integration SHA; unique temp root, test DB and port; no shared mutable process/global fixture. Same depth alone is NOT permission to dispatch.

## 15. Deliverables

- [ ] Pure codec/manifest API and GV1–12 fixtures

- [ ] Tests above and evidence including command exit codes, scope diff and accepted commit SHA.

## 16. Completion criteria

`SPRINT_IMPLEMENTED` requires artifacts, `SPRINT_TESTED` requires executed acceptance tests, `SPRINT_INTEGRATED` requires merged checks, and **`SPRINT_ACCEPTED` requires `gate_phase3_sprint8_deep1` accepted**. A contract-evidence sprint cannot be ACCEPTED for an affected implementation successor while its owner decision is unresolved. No agent declaration alone unlocks successors. All required assertions, file scope and baseline must pass.

## 17. Handoff contract

- Pure codec/manifest API and GV1–12 fixtures

- Exact artifacts: src-tauri/src/domain/sf06/codec.rs, src-tauri/src/domain/sf06/manifest.rs, src-tauri/tests/sf06_golden.rs; accepted integration SHA and `gate_phase3_sprint8_deep1` record. A successor must verify exports/ABI rather than infer from prose.

## 18. Resume and failure behavior

Not started: verify baseline and deps. Partial/test failure: inspect git status, changed paths and durable test DB, resume only safe incomplete steps. Implemented but unintegrated: do not dispatch successors. Interrupted agent/rejected patch: preserve branch, rebase only after review, rerun tests. Specification conflict: `CONTRACT_BLOCKED`, record evidence, request authority resolution; never edit SSOT in this sprint.


---

Subagent budget: 256K tokens maximum; start with the bounded references above, do not read entire SSOT by default. CLI-first with verified tool compatibility. Treat Jev/TypeSafe as optional advisory only when available and authorized; never send private repository content externally without consent.
