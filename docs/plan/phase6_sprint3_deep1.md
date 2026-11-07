---
sprint_id: phase6_sprint3_deep1
title: Verify end-to-end correctness and release gates
phase: 6
sprint: 3
deep: 1
baseline_commit: 57a462efb953a665e3673b15e6cc33622a4f2d39
status: PLANNED
---

# phase6_sprint3_deep1 — Verify end-to-end correctness and release gates

## 1. Sprint identity

Engineering phase 6 (Application assurance), within-phase depth 1; baseline is the Git commit above, not an uncommitted workspace. **PROPOSED, NOT IMPLEMENTED.**

## 2. Objective

Verify end-to-end correctness and release gates. Observable handoff: Release gate evidence and accepted integrated SHA only after all mandatory tests pass.

## 3. Scope boundaries

**IN_SCOPE:** Execute pnpm test/typecheck/lint/build, cargo fmt/clippy/test/check and application smoke after prerequisites exist.; Record exact commands/results/log hashes and integrated SHA; failures keep sprint unaccepted.

**OUT_OF_SCOPE / MUST NOT:** modify authoritative `docs/business/**`, `docs/db.dbml`, `docs/architecture/decisions/**`; use real application data; run production migration; add client request IDs to SF-06; invent business semantics; rewrite another sprint’s files.

## 4. Authoritative contract snapshot

- Unit PASS never implies end-to-end PASS; fault-injection and data-preservation tests use disposable DB only.

- No external LLM call/production DB; fake providers and isolated ports/temp roots; visual preservation compared against baseline.

- **REQ-TEST-001** `docs/business/be-core.md#L2515-L2611`: T-*, MIG/WF/IPC, WF-C/DB-P, REC matrices run on disposable databases. (planning coverage `ASSIGNED`).

  Normative excerpt (verbatim baseline lines, truncated to 300 chars; use `sed` for the full range):

  > L2515: ## §12. Testing & Acceptance Criteria (Ma trận Kiểm thử & Tiêu chí Nghiệm thu)

  > L2517: Các ma trận sau là tiêu chí kiểm thử dự kiến ở mức đặc tả; chưa khẳng định bất kỳ Rust integration test nào đã chạy hoặc PASS.

  > L2519: ### 12.1 Ma trận Kiểm thử Hệ thống Tổng thể (`T-*` Comprehensive Verification Matrix)

  > L2521: / Mã Test / Nhóm Kiểm thử / Bất biến / Yêu cầu Nghiệp vụ được Kiểm chứng / Kịch bản Thực thi Cụ thể / Kết quả Kỳ vọng (Pass Criteria) /

- **REQ-TEST-002** `docs/business/fe-core.md#L435-L440`: Existing 39 Node tests stay green; add independent IPC/FE integration and visual checks. (planning coverage `ASSIGNED`).

  Normative excerpt (verbatim baseline lines, truncated to 300 chars; use `sed` for the full range):

  > L435: ### 9.7 Test coverage (Wave 0.2)

  > L437: - Automated: `npm test` chạy glob `src/features/**/*.test.mjs` (Vite SSR + `node:test`). Trước cleanup Stage B: 34 test; sau cleanup: 39 test (`settings` 4, `commitPreview` C01–C12+, `commitNavigation` N01–N13, `scenePreview` 6 + 3 test F03 cho Unlock All khi generating/review). Đều là test logic th

  > L438: - Các SSR harness test đặt `server.ws: false`: Vite 8 vẫn tạo WebSocket server ở middleware mode dù `hmr: false`, khiến các file chạy song song tranh cổng 24678 (cảnh báo “Port 24678 is already in use”). Test không cần WebSocket nên tắt hẳn; không đổi isolation.

  > L439: - Manual browser checks: dirty Regenerate (discard/cancel/accept), multi-scene dirty, Save preview, navigation guard, lock/review, long scene 100/2.000/8.000 từ, Chat toggle, VI/EN, light/dark và 6 viewport.

**Targeted source references (read only these ranges unless a test proves broader context is necessary):**

- `docs/business/be-core.md#L2515-L2611` — focus: T/MIG/WF/REC acceptance matrices; CLI: `sed -n "2515,2611p" docs/business/be-core.md` (or `rg -n "<symbol>" docs/business/be-core.md` for a changed anchor).

- `docs/business/fe-core.md#L495-L514` — focus: Visual regression at 1280 and supported sizes; CLI: `sed -n "495,514p" docs/business/fe-core.md` (or `rg -n "<symbol>" docs/business/fe-core.md` for a changed anchor).

- `docs/architecture/decisions/sf06-owner-ratification.md#L3-L5` — focus: Implementation not previously verified; CLI: `sed -n "3,5p" docs/architecture/decisions/sf06-owner-ratification.md` (or `rg -n "<symbol>" docs/architecture/decisions/sf06-owner-ratification.md` for a changed anchor).

## 5. Prerequisites

`depends_on: ["phase6_sprint1_deep0", "phase6_sprint2_deep0"]`.

- `phase6_sprint1_deep0` [RUNTIME_DEPENDENCY] produces CI workflow and reproducible Docker/package scripts; inspect its accepted commit, exported artifact/test and `gate_phase6_sprint1_deep0`. If absent, do not start.

- `phase6_sprint2_deep0` [RUNTIME_DEPENDENCY] produces Integrated revision SHA, global registration, startup recovery and IPC inventory test; inspect its accepted commit, exported artifact/test and `gate_phase6_sprint2_deep0`. If absent, do not start.

## 6. Inputs

Required: accepted outputs of all predecessors; frozen committed SSOT ranges above; actual source paths listed below. Optional context: existing UI mocks for fixtures only. No `report/` input; no historical research.

## 7. File ownership

```yaml

allowed_write_paths:

  - tests/e2e/merlink_smoke.test.mjs

  - src-tauri/tests/full_recovery.rs

  - docs/plan/evidence/release-gate.md

forbidden_write_paths:

  - docs/business/**

  - docs/db.dbml

  - docs/architecture/decisions/**

  - app-data/**

  - report/**

shared_read_only_paths:

  - docs/business/be-core.md

  - docs/business/fe-core.md

  - docs/architecture/decisions/sf06-owner-ratification.md

generated_paths: []

```

Any allowed path may be absent at baseline (not only `src-tauri/`; new FE adapter/shim files are also future targets). Verify with `test -e`/`git ls-files` before editing; existing files are read first. Any extra path requires central integration approval, not a silent scope expansion.

## 8. Public interfaces

PROPOSED_INTERFACE: Release gate evidence and accepted integrated SHA only after all mandatory tests pass. Every consumed prerequisite interface is `PREREQUISITE_INTERFACE`, not an existing runtime guarantee. If a named signature conflicts with committed SSOT, stop `CONTRACT_BLOCKED`.

## 9. Implementation requirements

- Execute pnpm test/typecheck/lint/build, cargo fmt/clippy/test/check and application smoke after prerequisites exist.

- Record exact commands/results/log hashes and integrated SHA; failures keep sprint unaccepted.

- Prefer official compatible CLI/tooling over manual generated files; verify installed version and diff. No external LLM/Jev request without authorization; optional Jev advice never overrides SSOT.

## 10. Domain invariants

- Unit PASS never implies end-to-end PASS; fault-injection and data-preservation tests use disposable DB only.

- No external LLM call/production DB; fake providers and isolated ports/temp roots; visual preservation compared against baseline.

- No partial state escapes; protected provenance, explicit authorization, no external-effect recovery replay where applicable.

## 11. Failure matrix

| Trigger | Expected behavior / classification | Durable effects and recovery | Required test |

|---|---|---|---|

| Crash/fault injection or migration mismatch | release gate fails, no data repair; P6S3-N. | No unauthorized partial commit; re-read actual durable state before retry | phase6_sprint3_deep1-F1 |

| Missing browser/OS integration | mark unverified, not PASS; P6S3-C. | No unauthorized partial commit; re-read actual durable state before retry | phase6_sprint3_deep1-F2 |

| Missing accepted predecessor / baseline mismatch | `CONTRACT_BLOCKED`; do not run implementation | No user-data effects; record issue and owner in sprint evidence | phase6_sprint3_deep1-F0 |

## 12. Acceptance tests

Tests below name **future targets**, not tests executed during planning. Each row must become an executable test before the sprint can be accepted.

| Test ID | Initial state | Input / action | Expected output and state | Forbidden side effects / invariant |

|---|---|---|---|---|

| P6S3-MATRIX | accepted integrated worktree or isolated CI checkout | In the owned test target, set up the fixture and run the scenario: execute every named T-UNIT/T-DB/T-ROLL/T-CONC/T-IDEM/T-REST/T-PROV/T-CASC/T-WF/T-LOCK/T-SAVE/T-REC/T-IPC, MIG-01..05, WF-01..10, WF-C01..15, IPC-01..02 and DB-P01..02 vector on disposable DB; report each ID PASS/FAIL. | execute every named T-UNIT/T-DB/T-ROLL/T-CONC/T-IDEM/T-REST/T-PROV/T-CASC/T-WF/T-LOCK/T-SAVE/T-REC/T-IPC, MIG-01..05, WF-01..10, WF-C01..15, IPC-01..02 and DB-P01..02 vector on disposable DB; report each ID PASS/FAIL. | No unrelated write; preserve exact expected state and listed invariant |

| P6S3-P | accepted integrated worktree or isolated CI checkout | In the owned test target, set up the fixture and run the scenario: fresh app creates DB and executes chat→draft→commit→Writing Save with fake provider. | fresh app creates DB and executes chat→draft→commit→Writing Save with fake provider. | No unrelated write; preserve exact expected state and listed invariant |

| P6S3-N | accepted integrated worktree or isolated CI checkout | In the owned test target, set up the fixture and run the scenario: fault after TX08 before TX04 retains proposal without round completion. | fault after TX08 before TX04 retains proposal without round completion. | No unrelated write; preserve exact expected state and listed invariant |

| P6S3-B | accepted integrated worktree or isolated CI checkout | In the owned test target, set up the fixture and run the scenario: lock during stream then Unlock never enables old Accept; dirty Save OCC conflict preserves buffer. | lock during stream then Unlock never enables old Accept; dirty Save OCC conflict preserves buffer. | No unrelated write; preserve exact expected state and listed invariant |

| phase6_sprint3_deep1-F1 | accepted prerequisite fixture / isolated worktree | Inject Crash/fault injection or migration mismatch into owned operation | release gate fails, no data repair; P6S3-N. | No unauthorized commit or external replay |

| phase6_sprint3_deep1-F2 | accepted prerequisite fixture / isolated worktree | Inject Missing browser/OS integration into owned operation | mark unverified, not PASS; P6S3-C. | No unauthorized commit or external replay |

| phase6_sprint3_deep1-F0 | predecessor gate absent or baseline SHA drift | Attempt dispatch before predecessor accepted | CONTRACT_BLOCKED; no implementation started | No source/DB modification |

Future command after the target files exist: `pnpm test && pnpm typecheck && pnpm lint && pnpm build && cargo test --manifest-path src-tauri/Cargo.toml  # future target`. For FE changes, visual comparison at 1280/1279px and each declared viewport is mandatory; if tooling is unavailable the sprint is `BLOCKED_VISUAL_VERIFICATION` and cannot be accepted. `VISUAL_NOT_VERIFIED` is not a waiver.

## 13. Database and migration safety

No persistent database in this sprint; fixture DB (if any) is disposable and uniquely named per worktree.

## 14. Concurrency contract

- No same-phase/same-depth peer. Cross-phase scheduling still requires exact predecessor gates and conflict scan.

Isolation: own worktree/branch from baseline or accepted integration SHA; unique temp root, test DB and port; no shared mutable process/global fixture. Same depth alone is NOT permission to dispatch.

## 15. Deliverables

- [ ] Release gate evidence and accepted integrated SHA only after all mandatory tests pass

- [ ] Tests above and evidence including command exit codes, scope diff and accepted commit SHA.

## 16. Completion criteria

`SPRINT_IMPLEMENTED` requires artifacts, `SPRINT_TESTED` requires executed acceptance tests, `SPRINT_INTEGRATED` requires merged checks, and **`SPRINT_ACCEPTED` requires `gate_phase6_sprint3_deep1` accepted**. A contract-evidence sprint cannot be ACCEPTED for an affected implementation successor while its owner decision is unresolved. No agent declaration alone unlocks successors. All required assertions, file scope and baseline must pass.

## 17. Handoff contract

- Release gate evidence and accepted integrated SHA only after all mandatory tests pass

- Exact artifacts: tests/e2e/merlink_smoke.test.mjs, src-tauri/tests/full_recovery.rs, docs/plan/evidence/release-gate.md; accepted integration SHA and `gate_phase6_sprint3_deep1` record. A successor must verify exports/ABI rather than infer from prose.

## 18. Resume and failure behavior

Not started: verify baseline and deps. Partial/test failure: inspect git status, changed paths and durable test DB, resume only safe incomplete steps. Implemented but unintegrated: do not dispatch successors. Interrupted agent/rejected patch: preserve branch, rebase only after review, rerun tests. Specification conflict: `CONTRACT_BLOCKED`, record evidence, request authority resolution; never edit SSOT in this sprint.


---

Subagent budget: 256K tokens maximum; start with the bounded references above, do not read entire SSOT by default. CLI-first with verified tool compatibility. Treat Jev/TypeSafe as optional advisory only when available and authorized; never send private repository content externally without consent.
