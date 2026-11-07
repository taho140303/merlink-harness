---
sprint_id: phase3_sprint9_deep2
title: Verify historical B+ rounds read-only
phase: 3
sprint: 9
deep: 2
baseline_commit: 57a462efb953a665e3673b15e6cc33622a4f2d39
status: PLANNED
---

# phase3_sprint9_deep2 — Verify historical B+ rounds read-only

## 1. Sprint identity

Engineering phase 3 (Draft domain and durable workflow), within-phase depth 2; baseline is the Git commit above, not an uncommitted workspace. **PROPOSED, NOT IMPLEMENTED.**

## 2. Objective

Verify historical B+ rounds read-only. Observable handoff: Read-only verify_historical_round API and classifier fixtures.

## 3. Scope boundaries

**IN_SCOPE:** Implement read-only verifier, exact index set, hash chain and scope projection; test legacy no evidence and unsupported version.; Never call provider, effectful tool, old policy or temp report.

**OUT_OF_SCOPE / MUST NOT:** modify authoritative `docs/business/**`, `docs/db.dbml`, `docs/architecture/decisions/**`; use real application data; run production migration; add client request IDs to SF-06; invent business semantics; rewrite another sprint’s files.

## 4. Authoritative contract snapshot

- Verification uses sealed S0, header, N children, optional version, supported codec and durable artifacts only.

- Separate Verified/Corrupt/Indeterminate from Attested/Unknown and current-head applicability; Restore seq alone never skips.

- **REQ-SF06-001** `docs/db.dbml#L1299-L1355`: Round header, exact task membership, composite FKs; legacy child fields retained. (planning coverage `CONTRACT_VALIDATION_REQUIRED`).

  Normative excerpt (verbatim baseline lines, truncated to 300 chars; use `sed` for the full range):

  > L1299: // B+ SF-06: hợp đồng vật lý đích; chưa phải migration đã chạy. Header bất biến theo protocol trusted backend.

  > L1300: Table workflow_round_receipts {

  > L1301: id uuid [pk, note: 'UUIDv7; commit identity kể cả zero-net']

  > L1302: draft_session_id uuid [ref: > draft_sessions.id, not null]

- **REQ-SF06-002** `docs/business/be-core.md#L1534-L1536`: Durable historical verifier separates integrity, attestation, current applicability; lookup before OCC. (planning coverage `ASSIGNED`).

  Normative excerpt (verbatim baseline lines, truncated to 300 chars; use `sed` for the full range):

  > L1534: **Historical verifier** (proposed Rust interface, không phải implementation): `verify_historical_round(read_tx, header_id, supported_codecs, current_head) -> {integrity, authorization_attestation, applicability}`. Chỉ đọc header, S0 `draft_versions.data_json`, N child/evidence, optional final versio

  > L1536: **Seal/retry** (proposed `seal_or_retry_round(conn, intent, authorized_plan, authenticated_effects)`): orchestrator giữ **một** `operation_lock(draft)` trong round và chuyển guard/token tới seal (nếu gọi độc lập mới lấy lock; tuyệt đối không lấy lại khóa đang giữ), rồi `BEGIN IMMEDIATE`; lookup **cả

- **REQ-SF06-004** `docs/business/be-core.md#L1534-L1549`: Restore divergence, legacy Unknown, lost response and artifact integrity without LLM/tool replay. (planning coverage `ASSIGNED`).

  Normative excerpt (verbatim baseline lines, truncated to 300 chars; use `sed` for the full range):

  > L1534: **Historical verifier** (proposed Rust interface, không phải implementation): `verify_historical_round(read_tx, header_id, supported_codecs, current_head) -> {integrity, authorization_attestation, applicability}`. Chỉ đọc header, S0 `draft_versions.data_json`, N child/evidence, optional final versio

  > L1536: **Seal/retry** (proposed `seal_or_retry_round(conn, intent, authorized_plan, authenticated_effects)`): orchestrator giữ **một** `operation_lock(draft)` trong round và chuyển guard/token tới seal (nếu gọi độc lập mới lấy lock; tuyệt đối không lấy lại khóa đang giữ), rồi `BEGIN IMMEDIATE`; lookup **cả

  > L1538: **Proposal TX-08 và eligibility:** Tool executor tạo/reuse proposal dưới **guard `operation_lock(draft)` đã thuộc chat turn/workflow orchestrator**, không re-lock mutex không reentrant; guard tồn tại xuyên tool call, SQLite TX-08 chỉ mở quanh DB mutation và không chờ network. TX-08 commit trước resp

  > L1540: / Case / DB / lifecycle sau boundary / Data validity riêng / Confirmation eligibility riêng / Status / schema necessity /

- **REQ-SPECTEST-REC-01** `docs/business/be-core.md#L2595-L2595`: MUST implement executable BE acceptance vector REC-01, preserving scenario and expected result; old v1-only recovery claims remain legacy-only under B+. (planning coverage `ASSIGNED`).

- **REQ-SPECTEST-REC-02** `docs/business/be-core.md#L2596-L2596`: MUST implement executable BE acceptance vector REC-02, preserving scenario and expected result; old v1-only recovery claims remain legacy-only under B+. (planning coverage `ASSIGNED`).

- **REQ-SPECTEST-REC-03** `docs/business/be-core.md#L2597-L2597`: MUST implement executable BE acceptance vector REC-03, preserving scenario and expected result; old v1-only recovery claims remain legacy-only under B+. (planning coverage `ASSIGNED`).

- **REQ-SPECTEST-REC-04** `docs/business/be-core.md#L2598-L2598`: MUST implement executable BE acceptance vector REC-04, preserving scenario and expected result; old v1-only recovery claims remain legacy-only under B+. (planning coverage `ASSIGNED`).

- **REQ-SPECTEST-REC-05** `docs/business/be-core.md#L2599-L2599`: MUST implement executable BE acceptance vector REC-05, preserving scenario and expected result; old v1-only recovery claims remain legacy-only under B+. (planning coverage `ASSIGNED`).

- **REQ-SPECTEST-REC-06** `docs/business/be-core.md#L2600-L2600`: MUST implement executable BE acceptance vector REC-06, preserving scenario and expected result; old v1-only recovery claims remain legacy-only under B+. (planning coverage `ASSIGNED`).

- **REQ-SPECTEST-REC-07** `docs/business/be-core.md#L2601-L2601`: MUST implement executable BE acceptance vector REC-07, preserving scenario and expected result; old v1-only recovery claims remain legacy-only under B+. (planning coverage `ASSIGNED`).

- **REQ-SPECTEST-REC-08** `docs/business/be-core.md#L2602-L2602`: MUST implement executable BE acceptance vector REC-08, preserving scenario and expected result; old v1-only recovery claims remain legacy-only under B+. (planning coverage `ASSIGNED`).

- **REQ-SPECTEST-REC-09** `docs/business/be-core.md#L2603-L2603`: MUST implement executable BE acceptance vector REC-09, preserving scenario and expected result; old v1-only recovery claims remain legacy-only under B+. (planning coverage `ASSIGNED`).

- **REQ-SPECTEST-REC-10** `docs/business/be-core.md#L2604-L2604`: MUST implement executable BE acceptance vector REC-10, preserving scenario and expected result; old v1-only recovery claims remain legacy-only under B+. (planning coverage `ASSIGNED`).

- **REQ-SPECTEST-REC-11** `docs/business/be-core.md#L2605-L2605`: MUST implement executable BE acceptance vector REC-11, preserving scenario and expected result; old v1-only recovery claims remain legacy-only under B+. (planning coverage `ASSIGNED`).

- **REQ-SPECTEST-REC-12** `docs/business/be-core.md#L2606-L2606`: MUST implement executable BE acceptance vector REC-12, preserving scenario and expected result; old v1-only recovery claims remain legacy-only under B+. (planning coverage `ASSIGNED`).

- **REQ-SPECTEST-REC-13** `docs/business/be-core.md#L2607-L2607`: MUST implement executable BE acceptance vector REC-13, preserving scenario and expected result; old v1-only recovery claims remain legacy-only under B+. (planning coverage `ASSIGNED`).

- **REQ-SPECTEST-REC-14** `docs/business/be-core.md#L2608-L2608`: MUST implement executable BE acceptance vector REC-14, preserving scenario and expected result; old v1-only recovery claims remain legacy-only under B+. (planning coverage `ASSIGNED`).

- **REQ-SPECTEST-REC-15** `docs/business/be-core.md#L2609-L2609`: MUST implement executable BE acceptance vector REC-15, preserving scenario and expected result; old v1-only recovery claims remain legacy-only under B+. (planning coverage `ASSIGNED`).

**Targeted source references (read only these ranges unless a test proves broader context is necessary):**

- `docs/business/be-core.md#L1534-L1536` — focus: Integrity/attestation/applicability and retry; CLI: `sed -n "1534,1536p" docs/business/be-core.md` (or `rg -n "<symbol>" docs/business/be-core.md` for a changed anchor).

- `docs/business/be-core.md#L1549-L1549` — focus: Artifact refs and hash; CLI: `sed -n "1549,1549p" docs/business/be-core.md` (or `rg -n "<symbol>" docs/business/be-core.md` for a changed anchor).

- `docs/db.dbml#L1299-L1355` — focus: Exact child membership/FKs; CLI: `sed -n "1299,1355p" docs/db.dbml` (or `rg -n "<symbol>" docs/db.dbml` for a changed anchor).

- `docs/architecture/decisions/sf06-owner-ratification.md#L38-L42` — focus: Trusted-writer limit; CLI: `sed -n "38,42p" docs/architecture/decisions/sf06-owner-ratification.md` (or `rg -n "<symbol>" docs/architecture/decisions/sf06-owner-ratification.md` for a changed anchor).

## 5. Prerequisites

`depends_on: ["phase3_sprint8_deep1", "phase3_sprint2_deep0", "phase3_sprint3_deep0"]`.

- `phase3_sprint8_deep1` [RUNTIME_DEPENDENCY] produces Pure codec/manifest API and GV1–12 fixtures; inspect its accepted commit, exported artifact/test and `gate_phase3_sprint8_deep1`. If absent, do not start.

- `phase3_sprint2_deep0` [RUNTIME_DEPENDENCY] produces DraftService, DraftRuntimeRegistry guard, draft/version IPC #5–8,#10–11, TX01/03 fixtures; inspect its accepted commit, exported artifact/test and `gate_phase3_sprint2_deep0`. If absent, do not start.

- `phase3_sprint3_deep0` [DATA_CONTRACT_DEPENDENCY] produces Versioned WorldProbeReportV1/WorldWriteInputV1 and executable fixture proof; inspect its accepted commit, exported artifact/test and `gate_phase3_sprint3_deep0`. If absent, do not start.

## 6. Inputs

Required: accepted outputs of all predecessors; frozen committed SSOT ranges above; actual source paths listed below. Optional context: existing UI mocks for fixtures only. No `report/` input; no historical research.

## 7. File ownership

```yaml

allowed_write_paths:

  - src-tauri/src/service/receipt_verifier.rs

  - src-tauri/tests/sf06_verifier.rs

forbidden_write_paths:

  - docs/business/**

  - docs/db.dbml

  - docs/architecture/decisions/**

  - app-data/**

  - report/**

shared_read_only_paths:

  - docs/business/be-core.md

  - docs/db.dbml

  - docs/architecture/decisions/sf06-owner-ratification.md

generated_paths: []

```

Any allowed path may be absent at baseline (not only `src-tauri/`; new FE adapter/shim files are also future targets). Verify with `test -e`/`git ls-files` before editing; existing files are read first. Any extra path requires central integration approval, not a silent scope expansion.

## 8. Public interfaces

PROPOSED_INTERFACE: verify_historical_round(read_tx,id,codecs,head)->{integrity,authorization_attestation,applicability}. Every consumed prerequisite interface is `PREREQUISITE_INTERFACE`, not an existing runtime guarantee. If a named signature conflicts with committed SSOT, stop `CONTRACT_BLOCKED`.

## 9. Implementation requirements

- Implement read-only verifier, exact index set, hash chain and scope projection; test legacy no evidence and unsupported version.

- Never call provider, effectful tool, old policy or temp report.

- Prefer official compatible CLI/tooling over manual generated files; verify installed version and diff. No external LLM/Jev request without authorization; optional Jev advice never overrides SSOT.

## 10. Domain invariants

- Verification uses sealed S0, header, N children, optional version, supported codec and durable artifacts only.

- Separate Verified/Corrupt/Indeterminate from Attested/Unknown and current-head applicability; Restore seq alone never skips.

- No partial state escapes; protected provenance, explicit authorization, no external-effect recovery replay where applicable.

## 11. Failure matrix

| Trigger | Expected behavior / classification | Durable effects and recovery | Required test |

|---|---|---|---|

| Index gap/N+1/hash mismatch | Corrupt and no skip; P3S9-N. | No unauthorized partial commit; re-read actual durable state before retry | phase3_sprint9_deep2-F1 |

| Restore-diverged | historical Verified but inapplicable; P3S9-C. | No unauthorized partial commit; re-read actual durable state before retry | phase3_sprint9_deep2-F2 |

| Missing accepted predecessor / baseline mismatch | `CONTRACT_BLOCKED`; do not run implementation | No user-data effects; record issue and owner in sprint evidence | phase3_sprint9_deep2-F0 |

## 12. Acceptance tests

Tests below name **future targets**, not tests executed during planning. Each row must become an executable test before the sprint can be accepted.

| Test ID | Initial state | Input / action | Expected output and state | Forbidden side effects / invariant |

|---|---|---|---|---|

| P3S9-P | disposable uniquely-named SQLite S0 + accepted predecessor fixture | In the owned test target, set up the fixture and run the scenario: persisted zero-net header+children verifies after restart. | persisted zero-net header+children verifies after restart. | No unrelated write; preserve exact expected state and listed invariant |

| P3S9-N | disposable uniquely-named SQLite S0 + accepted predecessor fixture | In the owned test target, set up the fixture and run the scenario: index N or missing child fails closed despite SQLite FK pass. | index N or missing child fails closed despite SQLite FK pass. | No unrelated write; preserve exact expected state and listed invariant |

| P3S9-B | disposable uniquely-named SQLite S0 + accepted predecessor fixture | In the owned test target, set up the fixture and run the scenario: old legacy NULL evidence | old legacy NULL evidence => Indeterminate/RequiresReplan, no backfill. | No unrelated write; preserve exact expected state and listed invariant |

| phase3_sprint9_deep2-F1 | accepted prerequisite fixture / isolated worktree | Inject Index gap/N+1/hash mismatch into owned operation | Corrupt and no skip; P3S9-N. | No unauthorized commit or external replay |

| phase3_sprint9_deep2-F2 | accepted prerequisite fixture / isolated worktree | Inject Restore-diverged into owned operation | historical Verified but inapplicable; P3S9-C. | No unauthorized commit or external replay |

| REC-01 | disposable/synthetic fixture described by BE | T2 phụ thuộc T1 (`A=1→2`, `B=A+1`) | Fingerprint input T2 dùng S1, không dùng S0. | Only owner-authorized effects; old v1-only statements interpreted as legacy, no B+ regression |

| REC-02 | disposable/synthetic fixture described by BE | Không có trạng thái trung gian lịch sử ở legacy hoặc thiếu B+ evidence | `Indeterminate`; B+ hợp lệ thì tái dựng từ patch bền, không dựng verification giả. | Only owner-authorized effects; old v1-only statements interpreted as legacy, no B+ regression |

| REC-03 | disposable/synthetic fixture described by BE | Net-zero round có writes | B+ header + N evidence, không version; A/B `Known(true)` sau restart khi evidence verified; legacy thiếu evidence `Unknown`. | Only owner-authorized effects; old v1-only statements interpreted as legacy, no B+ regression |

| REC-04 | disposable/synthetic fixture described by BE | Receipt `had_mutations=0` | Không suy ra tool chưa ghi, không suy ra effective change=false. | Only owner-authorized effects; old v1-only statements interpreted as legacy, no B+ regression |

| REC-05 | disposable/synthetic fixture described by BE | Same-process retry với trusted outcomes | Kiểm chứng tuần tự deterministically; chỉ skip nếu mọi kiểm tra đạt. | Only owner-authorized effects; old v1-only statements interpreted as legacy, no B+ regression |

| REC-06 | disposable/synthetic fixture described by BE | Lost IPC response với request lặp | Tra cứu trước, kiểm tra độc lập; request strings không đủ bằng chứng. | Only owner-authorized effects; old v1-only statements interpreted as legacy, no B+ regression |

| REC-07 | disposable/synthetic fixture described by BE | Crash sau TX-04 commit | B+ verify header/evidence committed từ SQLite; legacy/thiếu lịch sử task → recovery/replan. | Only owner-authorized effects; old v1-only statements interpreted as legacy, no B+ regression |

| REC-08 | disposable/synthetic fixture described by BE | Crash trước TX-04 commit | Rollback; không fabricated completion. | Only owner-authorized effects; old v1-only statements interpreted as legacy, no B+ regression |

| REC-09 | disposable/synthetic fixture described by BE | Output receipt bị task sau ghi đè | Không false historical invalidation; chứng minh downstream hoặc `Indeterminate`. | Only owner-authorized effects; old v1-only statements interpreted as legacy, no B+ regression |

| REC-10 | disposable/synthetic fixture described by BE | Restore về snapshot cũ | Không reuse receipt trên nhánh đã bị cắt. | Only owner-authorized effects; old v1-only statements interpreted as legacy, no B+ regression |

| REC-11 | disposable/synthetic fixture described by BE | Existing prefix + new suffix | Chỉ chấp nhận prefix có chứng cứ đủ và suffix backend xác thực. | Only owner-authorized effects; old v1-only statements interpreted as legacy, no B+ regression |

| REC-12 | disposable/synthetic fixture described by BE | Interleaved existing/new | Reject bảo thủ, rollback. | Only owner-authorized effects; old v1-only statements interpreted as legacy, no B+ regression |

| REC-13 | disposable/synthetic fixture described by BE | Caller giả mutations | Reject, không chấp nhận hash/fingerprint tự khai làm bằng chứng. | Only owner-authorized effects; old v1-only statements interpreted as legacy, no B+ regression |

| REC-14 | disposable/synthetic fixture described by BE | Sau restart thiếu intermediate evidence | `RequiresReplan`/can thiệp an toàn, không tự skip/re-execute receipt. | Only owner-authorized effects; old v1-only statements interpreted as legacy, no B+ regression |

| REC-15 | disposable/synthetic fixture described by BE | Full-round retry không có raw mutations | Legacy thiếu durable patch không hứa replay; B+ đã có evidence patch để verify từ SQLite, không duplicate Draft Version. | Only owner-authorized effects; old v1-only statements interpreted as legacy, no B+ regression |

| phase3_sprint9_deep2-F0 | predecessor gate absent or baseline SHA drift | Attempt dispatch before predecessor accepted | CONTRACT_BLOCKED; no implementation started | No source/DB modification |

Backend module wiring rule: this sprint must not edit any `mod.rs`/`lib.rs`. Its integration test loads each owned implementation file via `#[path = "..."] mod <name>;` and fixture modules via the same mechanism. Production registration occurs only in P6S2 after this sprint is accepted. A compile success is therefore scoped to this test target, not a claim that the whole crate is wired.

Future command after the target files exist: `cargo test --manifest-path src-tauri/Cargo.toml --test sf06_verifier  # future target, after bootstrap`.

## 13. Database and migration safety

Use `BEGIN IMMEDIATE` only for the assigned TX; WAL/FULL/FK ON/busy_timeout as specified. Use unique disposable DB/temp root per worktree. Never open a user application DB; no destructive migration without separate authorization. Backup/rollback and PRAGMA user_version are owned by P2S3. TX04 never rolls back committed TX08.

## 14. Concurrency contract

- Peer `phase3_sprint5_deep2`: candidate parallel only after frozen contracts/gates; NONE: Disjoint exact write paths; shared types frozen by predecessors; worktree-local test resources.

Isolation: own worktree/branch from baseline or accepted integration SHA; unique temp root, test DB and port; no shared mutable process/global fixture. Same depth alone is NOT permission to dispatch.

## 15. Deliverables

- [ ] Read-only verify_historical_round API and classifier fixtures

- [ ] Tests above and evidence including command exit codes, scope diff and accepted commit SHA.

## 16. Completion criteria

`SPRINT_IMPLEMENTED` requires artifacts, `SPRINT_TESTED` requires executed acceptance tests, `SPRINT_INTEGRATED` requires merged checks, and **`SPRINT_ACCEPTED` requires `gate_phase3_sprint9_deep2` accepted**. A contract-evidence sprint cannot be ACCEPTED for an affected implementation successor while its owner decision is unresolved. No agent declaration alone unlocks successors. All required assertions, file scope and baseline must pass.

## 17. Handoff contract

- Read-only verify_historical_round API and classifier fixtures

- Exact artifacts: src-tauri/src/service/receipt_verifier.rs, src-tauri/tests/sf06_verifier.rs; accepted integration SHA and `gate_phase3_sprint9_deep2` record. A successor must verify exports/ABI rather than infer from prose.

## 18. Resume and failure behavior

Not started: verify baseline and deps. Partial/test failure: inspect git status, changed paths and durable test DB, resume only safe incomplete steps. Implemented but unintegrated: do not dispatch successors. Interrupted agent/rejected patch: preserve branch, rebase only after review, rerun tests. Specification conflict: `CONTRACT_BLOCKED`, record evidence, request authority resolution; never edit SSOT in this sprint.

**Explicit unblock requirements:** CI-02, CI-11, CONTRACT_REVIEW_REQUIRED:REQ-SF06-001.


---

Subagent budget: 256K tokens maximum; start with the bounded references above, do not read entire SSOT by default. CLI-first with verified tool compatibility. Treat Jev/TypeSafe as optional advisory only when available and authorized; never send private repository content externally without consent.
