---
sprint_id: phase5_sprint1_deep0
title: Freeze frontend transport and DTO adapters
phase: 5
sprint: 1
deep: 0
baseline_commit: 57a462efb953a665e3673b15e6cc33622a4f2d39
status: PLANNED
---

# phase5_sprint1_deep0 — Freeze frontend transport and DTO adapters

## 1. Sprint identity

Engineering phase 5 (Frontend integration), within-phase depth 0; baseline is the Git commit above, not an uncommitted workspace. **PROPOSED, NOT IMPLEMENTED.**

## 2. Objective

Freeze frontend transport and DTO adapters. Observable handoff: Typed backend client/adapter and FE fixture tests.

## 3. Scope boundaries

**IN_SCOPE:** Implement typed invoke/Channel adapter and pure snake_case→camelCase mapping against frozen DTO fixtures.; Test SSR no-window safety, error classes, event ordering, no plaintext key serialization.

**OUT_OF_SCOPE / MUST NOT:** modify authoritative `docs/business/**`, `docs/db.dbml`, `docs/architecture/decisions/**`; use real application data; run production migration; add client request IDs to SF-06; invent business semantics; rewrite another sprint’s files.

## 4. Authoritative contract snapshot

- Transport adapts existing SSR React Router app to Tauri IPC only in desktop environment; no fabricated persistent mock.

- Generated binding `src/shared/backend/generated.ts` belongs P2S5, never hand-edit; `types.ts` is a hand-written adapter/shim only, not a generated artifact. Global route/dict belong P6S2.

- **REQ-FE-001** `docs/business/fe-core.md#L495-L514`: Preserve UI layout/routes/breakpoints; additive integration and visual regression. (planning coverage `ASSIGNED`).

  Normative excerpt (verbatim baseline lines, truncated to 300 chars; use `sed` for the full range):

  > L495: ### 12.1 UI Preservation Invariant (Bất biến bảo toàn giao diện)

  > L497: Quy tắc này áp dụng cho mọi task có thay đổi FE, kể cả task mà mục tiêu chính là backend, Rust, IPC, API, persistence hoặc tích hợp nghiệp vụ. Đây là điều kiện nghiệm thu bắt buộc, không phải khuyến nghị. Nguyên tắc tối cao: **Functional correctness does not override visual preservation.**

  > L499: 1. **Preserve by default:** Layout, navigation, sidebar, split pane, tabs, stepper, toolbar, component placement, spacing, typography, styling và responsive breakpoints hiện tại phải được bảo toàn mặc định.

  > L500: 2. **Backend integration không đồng nghĩa UI redesign:** Việc triển khai Rust Backend, IPC, API, persistence hoặc bất kỳ nghiệp vụ mới nào không đồng nghĩa được phép thiết kế lại giao diện.

- **REQ-OPS-002** `docs/design-system.md#L8-L16`: Two-tier visual system and i18n chrome preserved (config/worldbuilding exceptions). (planning coverage `ASSIGNED`).

  Normative excerpt (verbatim baseline lines, truncated to 300 chars; use `sed` for the full range):

  > L8: ## 1. Quyết định đã chốt

  > L10: / # / Quyết định / Hệ quả /

  > L11: /:--/:--/:--/

  > L12: / 1 / **Mô hình surface 2 tầng** / Tầng 1 (shell/overlay) = neumorphism. Tầng 2 (nội dung dữ liệu) = flat-bordered glass. Không trộn lẫn. /

**Targeted source references (read only these ranges unless a test proves broader context is necessary):**

- `docs/business/be-core.md#L2100-L2108` — focus: serde/ts-rs and UI preservation; CLI: `sed -n "2100,2108p" docs/business/be-core.md` (or `rg -n "<symbol>" docs/business/be-core.md` for a changed anchor).

- `docs/business/fe-core.md#L495-L514` — focus: Visual preservation checklist; CLI: `sed -n "495,514p" docs/business/fe-core.md` (or `rg -n "<symbol>" docs/business/fe-core.md` for a changed anchor).

- `docs/business/fe-core.md#L516-L539` — focus: FE data needs; CLI: `sed -n "516,539p" docs/business/fe-core.md` (or `rg -n "<symbol>" docs/business/fe-core.md` for a changed anchor).

- `package.json#L5-L12` — focus: Existing test commands; CLI: `sed -n "5,12p" package.json` (or `rg -n "<symbol>" package.json` for a changed anchor).

## 5. Prerequisites

`depends_on: ["phase2_sprint5_deep2"]`.

- `phase2_sprint5_deep2` [TYPE_DEPENDENCY] produces Frozen DTO/error/adapter Rust API plus generated TS binding path and schema fixture; inspect its accepted commit, exported artifact/test and `gate_phase2_sprint5_deep2`. If absent, do not start.

## 6. Inputs

Required: accepted outputs of all predecessors; frozen committed SSOT ranges above; actual source paths listed below. Optional context: existing UI mocks for fixtures only. No `report/` input; no historical research.

## 7. File ownership

```yaml

allowed_write_paths:

  - src/shared/backend/client.ts

  - src/shared/backend/adapter.ts

  - src/shared/backend/types.ts

  - src/features/integration/contractFixtures.test.mjs

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

Any allowed path may be absent at baseline (not only `src-tauri/`; new FE adapter/shim files are also future targets). Verify with `test -e`/`git ls-files` before editing; existing files are read first. Any extra path requires central integration approval, not a silent scope expansion.

## 8. Public interfaces

PROPOSED_INTERFACE: typed desktop invoke/channel adapter; SSR-safe failure; generated DTOs remain read-only. Every consumed prerequisite interface is `PREREQUISITE_INTERFACE`, not an existing runtime guarantee. If a named signature conflicts with committed SSOT, stop `CONTRACT_BLOCKED`.

## 9. Implementation requirements

- Implement typed invoke/Channel adapter and pure snake_case→camelCase mapping against frozen DTO fixtures.

- Test SSR no-window safety, error classes, event ordering, no plaintext key serialization.

- Prefer official compatible CLI/tooling over manual generated files; verify installed version and diff. No external LLM/Jev request without authorization; optional Jev advice never overrides SSOT.

## 10. Domain invariants

- Transport adapts existing SSR React Router app to Tauri IPC only in desktop environment; no fabricated persistent mock.

- Generated binding `src/shared/backend/generated.ts` belongs P2S5, never hand-edit; `types.ts` is a hand-written adapter/shim only, not a generated artifact. Global route/dict belong P6S2.

- No partial state escapes; protected provenance, explicit authorization, no external-effect recovery replay where applicable.

## 11. Failure matrix

| Trigger | Expected behavior / classification | Durable effects and recovery | Required test |

|---|---|---|---|

| Tauri unavailable | explicit unavailable state, not success from mock; P5S1-N. | No unauthorized partial commit; re-read actual durable state before retry | phase5_sprint1_deep0-F1 |

| Generated DTO drift | typecheck fails, stop rather than changing schema; P5S1-C. | No unauthorized partial commit; re-read actual durable state before retry | phase5_sprint1_deep0-F2 |

| Missing accepted predecessor / baseline mismatch | `CONTRACT_BLOCKED`; do not run implementation | No user-data effects; record issue and owner in sprint evidence | phase5_sprint1_deep0-F0 |

## 12. Acceptance tests

Tests below name **future targets**, not tests executed during planning. Each row must become an executable test before the sprint can be accepted.

| Test ID | Initial state | Input / action | Expected output and state | Forbidden side effects / invariant |

|---|---|---|---|---|

| P5S1-P | accepted backend gate + fake typed Tauri transport fixture | In the owned test target, set up the fixture and run the scenario: fixture adapter preserves source and stable code but hides code label. | fixture adapter preserves source and stable code but hides code label. | No unrelated write; preserve exact expected state and listed invariant |

| P5S1-N | accepted backend gate + fake typed Tauri transport fixture | In the owned test target, set up the fixture and run the scenario: SSR imports without window/Tauri. | SSR imports without window/Tauri. | No unrelated write; preserve exact expected state and listed invariant |

| P5S1-B | accepted backend gate + fake typed Tauri transport fixture | In the owned test target, set up the fixture and run the scenario: unknown IPC error maps to safe status, no raw secrets. | unknown IPC error maps to safe status, no raw secrets. | No unrelated write; preserve exact expected state and listed invariant |

| P5S1-VISUAL | accepted backend gate + fake typed Tauri transport fixture | In the owned test target, set up the fixture and run the scenario: add only adapter/test files; rendered routes at 1280/1279px and declared viewports match baseline pixel/layout review; missing browser makes sprint BLOCKED_VISUAL_VERIFICATION. | add only adapter/test files; rendered routes at 1280/1279px and declared viewports match baseline pixel/layout review; missing browser makes sprint BLOCKED_VISUAL_VERIFICATION. | No unrelated write; preserve exact expected state and listed invariant |

| phase5_sprint1_deep0-F1 | accepted prerequisite fixture / isolated worktree | Inject Tauri unavailable into owned operation | explicit unavailable state, not success from mock; P5S1-N. | No unauthorized commit or external replay |

| phase5_sprint1_deep0-F2 | accepted prerequisite fixture / isolated worktree | Inject Generated DTO drift into owned operation | typecheck fails, stop rather than changing schema; P5S1-C. | No unauthorized commit or external replay |

| phase5_sprint1_deep0-F0 | predecessor gate absent or baseline SHA drift | Attempt dispatch before predecessor accepted | CONTRACT_BLOCKED; no implementation started | No source/DB modification |

Future command after the target files exist: `pnpm test && pnpm typecheck && pnpm lint && pnpm build  # future target, after dependencies exist`. For FE changes, visual comparison at 1280/1279px and each declared viewport is mandatory; if tooling is unavailable the sprint is `BLOCKED_VISUAL_VERIFICATION` and cannot be accepted. `VISUAL_NOT_VERIFIED` is not a waiver.

## 13. Database and migration safety

No persistent database in this sprint; fixture DB (if any) is disposable and uniquely named per worktree.

## 14. Concurrency contract

- No same-phase/same-depth peer. Cross-phase scheduling still requires exact predecessor gates and conflict scan.

Isolation: own worktree/branch from baseline or accepted integration SHA; unique temp root, test DB and port; no shared mutable process/global fixture. Same depth alone is NOT permission to dispatch.

## 15. Deliverables

- [ ] Typed backend client/adapter and FE fixture tests

- [ ] Tests above and evidence including command exit codes, scope diff and accepted commit SHA.

## 16. Completion criteria

`SPRINT_IMPLEMENTED` requires artifacts, `SPRINT_TESTED` requires executed acceptance tests, `SPRINT_INTEGRATED` requires merged checks, and **`SPRINT_ACCEPTED` requires `gate_phase5_sprint1_deep0` accepted**. A contract-evidence sprint cannot be ACCEPTED for an affected implementation successor while its owner decision is unresolved. No agent declaration alone unlocks successors. All required assertions, file scope and baseline must pass.

## 17. Handoff contract

- Typed backend client/adapter and FE fixture tests

- Exact artifacts: src/shared/backend/client.ts, src/shared/backend/adapter.ts, src/shared/backend/types.ts, src/features/integration/contractFixtures.test.mjs; accepted integration SHA and `gate_phase5_sprint1_deep0` record. A successor must verify exports/ABI rather than infer from prose.

## 18. Resume and failure behavior

Not started: verify baseline and deps. Partial/test failure: inspect git status, changed paths and durable test DB, resume only safe incomplete steps. Implemented but unintegrated: do not dispatch successors. Interrupted agent/rejected patch: preserve branch, rebase only after review, rerun tests. Specification conflict: `CONTRACT_BLOCKED`, record evidence, request authority resolution; never edit SSOT in this sprint.


---

Subagent budget: 256K tokens maximum; start with the bounded references above, do not read entire SSOT by default. CLI-first with verified tool compatibility. Treat Jev/TypeSafe as optional advisory only when available and authorized; never send private repository content externally without consent.
