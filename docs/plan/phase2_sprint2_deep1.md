---
sprint_id: phase2_sprint2_deep1
title: Freeze pure domain types and canonical state
phase: 2
sprint: 2
deep: 1
baseline_commit: 57a462efb953a665e3673b15e6cc33622a4f2d39
status: PLANNED
---

# phase2_sprint2_deep1 — Freeze pure domain types and canonical state

## 1. Sprint identity

Engineering phase 2 (Foundation and frozen interfaces), within-phase depth 1; baseline is the Git commit above, not an uncommitted workspace. **PROPOSED, NOT IMPLEMENTED.**

## 2. Objective

Freeze pure domain types and canonical state. Observable handoff: Frozen BusinessFlowData/TrackedField/CanonicalStateBytes APIs and pure test target.

## 3. Scope boundaries

**IN_SCOPE:** Implement types and pure transformations; derive serde names from current Core, not mock FE types.; Write in-module unit/property tests for JCS bytes, missing/null, source, ordered arrays, code allocation and multilingual word count.

**OUT_OF_SCOPE / MUST NOT:** modify authoritative `docs/business/**`, `docs/db.dbml`, `docs/architecture/decisions/**`; use real application data; run production migration; add client request IDs to SF-06; invent business semantics; rewrite another sprint’s files.

## 4. Authoritative contract snapshot

- BusinessFlowData/TrackedField provenance canonical JSON excludes meta.updated_at but includes counters and semantic array order.

- Domain is pure: no Tauri/SQLite/HTTP; codes never reissued after deletion, Restore reads saved counters.

- **REQ-DOM-001** `docs/business/core.md#L24-L54`: Nullable collection, TrackedField provenance and no AI fabrication; protected-write enforcement is delegated to the tool-owner requirement. (planning coverage `ASSIGNED`).

  Normative excerpt (verbatim baseline lines, truncated to 300 chars; use `sed` for the full range):

  > L24: ### 1.2 Nguyên tắc thiết kế cốt lõi

  > L26: * **Tách bạch State Hội thoại và State Nghiệp vụ:** Lịch sử chat (`ChatMessage[]`) là nhật ký trao đổi để người dùng và AI tương tác; bản nháp cấu trúc (`BusinessFlowData`) là một tài liệu JSON sống được cập nhật liên tục qua các tool call (xem mục 1.4); AI không ghi trực tiếp vào JSON.

  > L27: * **Mọi trường đều Nullable trong quá trình thu thập:** Khi người dùng chưa nhắc tới một yếu tố, trường đó giữ giá trị `null` hoặc mảng rỗng. AI không tự bịa quyết định định lượng/định danh như số Arc, số chương, tên nhân vật hoặc thể loại; nội dung này tuân theo Mức 1 tại mục 1.3. Quy tắc này không

  > L28: * **Gating trước khi Dựng (Readiness Gate):** Chỉ cho phép kích hoạt quy trình sinh dàn ý chuyên sâu (`Draft Building`) khi tập thuộc tính cốt lõi (`require`) đã được lấp đầy hoàn chỉnh.

- **REQ-DOM-002** `docs/business/be-core.md#L512-L555`: Stable entity codes, per-parent monotone _counters, restore consistency. (planning coverage `ASSIGNED`).

  Normative excerpt (verbatim baseline lines, truncated to 300 chars; use `sed` for the full range):

  > L512: ### 4.1 Quyền sở hữu `BusinessFlowData` và Cấu trúc `WorkingState`

  > L514: Theo `core.md` §1.2 và §3.6:

  > L515: - Mỗi phiên nháp (`draft_session_id`) sở hữu đúng một `WorkingState` trong bộ nhớ Backend và chuỗi các bản chụp bất biến trong bảng `draft_versions`.

  > L516: - Cấu trúc JSON được lưu trong `draft_versions.data_json` `MUST` bao gồm:

- **REQ-DOM-006** `docs/business/be-core.md#L1681-L2025`: UAX #29/CJK deterministic word count for Writing Mode. (planning coverage `ASSIGNED`).

  Normative excerpt (verbatim baseline lines, truncated to 300 chars; use `sed` for the full range):

  > L1681: ## §8. Writing Mode Backend (Dịch vụ Backend cho Chế độ Sáng tác — Chương-First, Cảnh là Block)

  > L1683: Mục này thực thi đầy đủ hợp đồng Writing Mode đã phê duyệt tại `core.md` §1.5, §1.6 (`H1–H3`, `Decision 1B: LOCK-01..07`, `Decision 2A: SCOPE-01..06`, và `Q4.1–Q4.9`).

  > L1685: ### 8.1 Cổng Kiểm tra Plan Đã Commit (`Committed-Plan Gate`) & Mô hình Đọc Chương/Cảnh (`Q4.5`)

  > L1687: 1. **Điều kiện mở Writing Mode (`core.md` §1.5, §1.6):**

- **REQ-CODEC-001** `docs/business/be-core.md#L556-L561`: RFC8785 JCS state bytes exclude meta.updated_at; compare bytes not only hash. (planning coverage `ASSIGNED`).

  Normative excerpt (verbatim baseline lines, truncated to 300 chars; use `sed` for the full range):

  > L556: ### 4.3 Chuẩn hóa Băm Trạng thái (`data_hash`)

  > L558: Để phát hiện chính xác trường hợp dữ liệu không thay đổi (`data_hash == head_version.data_hash` nhằm chặn tạo version trùng theo `core.md` §3.6), Backend `MUST` tính `data_hash` theo quy trình tất định:

  > L559: 1. Chuẩn hóa cây JSON của `BusinessFlowData` (bao gồm `require`, `optional`, `user_constraints`, `decision_log`, `_counters` và toàn bộ trường `source`) theo chuẩn **RFC 8785 (JSON Canonicalization Scheme — JCS)**: sắp xếp toàn bộ khóa của object theo thứ tự mã điểm UTF-16/lexicographical, định dạng

  > L560: 2. Băm chuỗi byte UTF-8 đã chuẩn hóa bằng thuật toán **SHA-256** và xuất ra chuỗi hex 64 ký tự chữ thường (`sha256:<64_hex_chars>`).

- **REQ-SPECTEST-T-UNIT-02** `docs/business/be-core.md#L2524-L2524`: MUST implement executable BE acceptance vector T-UNIT-02, preserving scenario and expected result; old v1-only recovery claims remain legacy-only under B+. (planning coverage `ASSIGNED`).

- **REQ-SPECTEST-T-UNIT-05** `docs/business/be-core.md#L2527-L2527`: MUST implement executable BE acceptance vector T-UNIT-05, preserving scenario and expected result; old v1-only recovery claims remain legacy-only under B+. (planning coverage `ASSIGNED`).

**Targeted source references (read only these ranges unless a test proves broader context is necessary):**

- `docs/business/core.md#L24-L109` — focus: Nullable and provenance rules; CLI: `sed -n "24,109p" docs/business/core.md` (or `rg -n "<symbol>" docs/business/core.md` for a changed anchor).

- `docs/business/be-core.md#L512-L561` — focus: State, codes, JCS hash; CLI: `sed -n "512,561p" docs/business/be-core.md` (or `rg -n "<symbol>" docs/business/be-core.md` for a changed anchor).

- `docs/business/core.md#L791-L799` — focus: Stable code/counters; CLI: `sed -n "791,799p" docs/business/core.md` (or `rg -n "<symbol>" docs/business/core.md` for a changed anchor).

- `docs/business/be-core.md#L2519-L2527` — focus: Pure test vectors; CLI: `sed -n "2519,2527p" docs/business/be-core.md` (or `rg -n "<symbol>" docs/business/be-core.md` for a changed anchor).

## 5. Prerequisites

`depends_on: ["phase2_sprint1_deep0"]`.

- `phase2_sprint1_deep0` [BUILD_DEPENDENCY] produces Compilable Cargo workspace; module-root scaffolding only (no downstream implementation files) and exact dependency lockfile; inspect its accepted commit, exported artifact/test and `gate_phase2_sprint1_deep0`. If absent, do not start.

## 6. Inputs

Required: accepted outputs of all predecessors; frozen committed SSOT ranges above; actual source paths listed below. Optional context: existing UI mocks for fixtures only. No `report/` input; no historical research.

## 7. File ownership

```yaml

allowed_write_paths:

  - src-tauri/src/domain/business_flow.rs

  - src-tauri/src/domain/code_allocator.rs

  - src-tauri/src/domain/fingerprint.rs

  - src-tauri/src/domain/word_count.rs

  - src-tauri/tests/domain_core.rs

forbidden_write_paths:

  - docs/business/**

  - docs/db.dbml

  - docs/architecture/decisions/**

  - app-data/**

  - report/**

shared_read_only_paths:

  - docs/business/core.md

  - docs/business/be-core.md

generated_paths: []

```

Any allowed path may be absent at baseline (not only `src-tauri/`; new FE adapter/shim files are also future targets). Verify with `test -e`/`git ls-files` before editing; existing files are read first. Any extra path requires central integration approval, not a silent scope expansion.

## 8. Public interfaces

PROPOSED_INTERFACE: BusinessFlowData, TrackedField<T>, ProvenanceSource, canonical_state(&State)->Result<Vec<u8>>, CodeAllocator::allocate. Every consumed prerequisite interface is `PREREQUISITE_INTERFACE`, not an existing runtime guarantee. If a named signature conflicts with committed SSOT, stop `CONTRACT_BLOCKED`.

## 9. Implementation requirements

- Implement types and pure transformations; derive serde names from current Core, not mock FE types.

- Write in-module unit/property tests for JCS bytes, missing/null, source, ordered arrays, code allocation and multilingual word count.

- Prefer official compatible CLI/tooling over manual generated files; verify installed version and diff. No external LLM/Jev request without authorization; optional Jev advice never overrides SSOT.

## 10. Domain invariants

- BusinessFlowData/TrackedField provenance canonical JSON excludes meta.updated_at but includes counters and semantic array order.

- Domain is pure: no Tauri/SQLite/HTTP; codes never reissued after deletion, Restore reads saved counters.

- No partial state escapes; protected provenance, explicit authorization, no external-effect recovery replay where applicable.

## 11. Failure matrix

| Trigger | Expected behavior / classification | Durable effects and recovery | Required test |

|---|---|---|---|

| Unknown State shape | typed validation error and no mutation; P2S2-N. | No unauthorized partial commit; re-read actual durable state before retry | phase2_sprint2_deep1-F1 |

| Collision/counter rewind | reject without publishing candidate; P2S2-C. | No unauthorized partial commit; re-read actual durable state before retry | phase2_sprint2_deep1-F2 |

| Missing accepted predecessor / baseline mismatch | `CONTRACT_BLOCKED`; do not run implementation | No user-data effects; record issue and owner in sprint evidence | phase2_sprint2_deep1-F0 |

## 12. Acceptance tests

Tests below name **future targets**, not tests executed during planning. Each row must become an executable test before the sprint can be accepted.

| Test ID | Initial state | Input / action | Expected output and state | Forbidden side effects / invariant |

|---|---|---|---|---|

| P2S2-P | pure Rust fixture (no DB/network/Tauri) + accepted predecessor types | In the owned test target, set up the fixture and run the scenario: canonical bytes identical for reordered object keys but not reordered semantic arrays. | canonical bytes identical for reordered object keys but not reordered semantic arrays. | No unrelated write; preserve exact expected state and listed invariant |

| P2S2-N | pure Rust fixture (no DB/network/Tauri) + accepted predecessor types | In the owned test target, set up the fixture and run the scenario: protected source and `meta.updated_at` exclusion verified; no DB writes. | protected source and `meta.updated_at` exclusion verified; no DB writes. | No unrelated write; preserve exact expected state and listed invariant |

| P2S2-B | pure Rust fixture (no DB/network/Tauri) + accepted predecessor types | In the owned test target, set up the fixture and run the scenario: delete then allocate yields next suffix, never reuses deleted code. | delete then allocate yields next suffix, never reuses deleted code. | No unrelated write; preserve exact expected state and listed invariant |

| phase2_sprint2_deep1-F1 | accepted prerequisite fixture / isolated worktree | Inject Unknown State shape into owned operation | typed validation error and no mutation; P2S2-N. | No unauthorized commit or external replay |

| phase2_sprint2_deep1-F2 | accepted prerequisite fixture / isolated worktree | Inject Collision/counter rewind into owned operation | reject without publishing candidate; P2S2-C. | No unauthorized commit or external replay |

| T-UNIT-02 | disposable/synthetic fixture described by BE | Tạo Project, 2 Arc, 3 Chapter, xóa Chapter 2 rồi tạo Chapter mới; đổi `display_name` của Arc 1. | `code` của Arc 1 giữ nguyên khi đổi `display_name`. Chapter mới sau khi xóa Chapter 2 nhận hậu tố `-c4` (không tái sử dụng `-c2`). `_counters` tăng đơn điệu. | Only owner-authorized effects; old v1-only statements interpreted as legacy, no B+ regression |

| T-UNIT-05 | disposable/synthetic fixture described by BE | Đếm văn bản Tiếng Việt, Tiếng Anh, Tiếng Nhật và Tiếng Trung. | Trả về số lượng từ tất định khớp quy tắc UAX #29 + CJK. | Only owner-authorized effects; old v1-only statements interpreted as legacy, no B+ regression |

| phase2_sprint2_deep1-F0 | predecessor gate absent or baseline SHA drift | Attempt dispatch before predecessor accepted | CONTRACT_BLOCKED; no implementation started | No source/DB modification |

Backend module wiring rule: this sprint must not edit any `mod.rs`/`lib.rs`. Its integration test loads each owned implementation file via `#[path = "..."] mod <name>;` and fixture modules via the same mechanism. Production registration occurs only in P6S2 after this sprint is accepted. A compile success is therefore scoped to this test target, not a claim that the whole crate is wired.

Future command after the target files exist: `cargo test --manifest-path src-tauri/Cargo.toml --test domain_core  # future target, after bootstrap`.

## 13. Database and migration safety

No persistent database in this sprint; fixture DB (if any) is disposable and uniquely named per worktree.

## 14. Concurrency contract

- Peer `phase2_sprint3_deep1`: candidate parallel only after frozen contracts/gates; NONE: Disjoint exact write paths; shared types frozen by predecessors; worktree-local test resources.

Isolation: own worktree/branch from baseline or accepted integration SHA; unique temp root, test DB and port; no shared mutable process/global fixture. Same depth alone is NOT permission to dispatch.

## 15. Deliverables

- [ ] Frozen BusinessFlowData/TrackedField/CanonicalStateBytes APIs and pure test target

- [ ] Tests above and evidence including command exit codes, scope diff and accepted commit SHA.

## 16. Completion criteria

`SPRINT_IMPLEMENTED` requires artifacts, `SPRINT_TESTED` requires executed acceptance tests, `SPRINT_INTEGRATED` requires merged checks, and **`SPRINT_ACCEPTED` requires `gate_phase2_sprint2_deep1` accepted**. A contract-evidence sprint cannot be ACCEPTED for an affected implementation successor while its owner decision is unresolved. No agent declaration alone unlocks successors. All required assertions, file scope and baseline must pass.

## 17. Handoff contract

- Frozen BusinessFlowData/TrackedField/CanonicalStateBytes APIs and pure test target

- Exact artifacts: src-tauri/src/domain/business_flow.rs, src-tauri/src/domain/code_allocator.rs, src-tauri/src/domain/fingerprint.rs, src-tauri/src/domain/word_count.rs, src-tauri/tests/domain_core.rs; accepted integration SHA and `gate_phase2_sprint2_deep1` record. A successor must verify exports/ABI rather than infer from prose.

## 18. Resume and failure behavior

Not started: verify baseline and deps. Partial/test failure: inspect git status, changed paths and durable test DB, resume only safe incomplete steps. Implemented but unintegrated: do not dispatch successors. Interrupted agent/rejected patch: preserve branch, rebase only after review, rerun tests. Specification conflict: `CONTRACT_BLOCKED`, record evidence, request authority resolution; never edit SSOT in this sprint.


---

Subagent budget: 256K tokens maximum; start with the bounded references above, do not read entire SSOT by default. CLI-first with verified tool compatibility. Treat Jev/TypeSafe as optional advisory only when available and authorized; never send private repository content externally without consent.
