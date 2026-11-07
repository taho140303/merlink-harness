---
sprint_id: phase4_sprint5_deep2
title: Stream scene generation under EditScope
phase: 4
sprint: 5
deep: 2
baseline_commit: 57a462efb953a665e3673b15e6cc33622a4f2d39
status: PLANNED
---

# phase4_sprint5_deep2 — Stream scene generation under EditScope

## 1. Sprint identity

Engineering phase 4 (Backend services and Writing Mode), within-phase depth 2; baseline is the Git commit above, not an uncommitted workspace. **PROPOSED, NOT IMPLEMENTED.**

## 2. Objective

Stream scene generation under EditScope. Observable handoff: WritingGeneration service, SceneGenerationStreamEvent and IPC #32–34.

## 3. Scope boundaries

**IN_SCOPE:** Implement RAM buffer, cancellation token and generation commands #32–34; no direct scene content/versions write.; Test lock-unlock-accept rejection and valid-completed-after-scope-expiry accept path via fake provider.

**OUT_OF_SCOPE / MUST NOT:** modify authoritative `docs/business/**`, `docs/db.dbml`, `docs/architecture/decisions/**`; use real application data; run production migration; add client request IDs to SF-06; invent business semantics; rewrite another sprint’s files.

## 4. Authoritative contract snapshot

- At most one active generation/chapter; scene must be unlocked and inside request scope; note trim only for current request.

- Lock during stream invalidates generation forever even after Unlock; completed valid output survives scope expiry for review/accept.

- **REQ-WRITE-002** `docs/business/core.md#L183-L222`: Scene stream/cancel/EditScope; Lock invalidates generation forever, Accept remains review-only. (planning coverage `CONTRACT_VALIDATION_REQUIRED`).

  Normative excerpt (verbatim baseline lines, truncated to 300 chars; use `sed` for the full range):

  > L183: #### Vòng đời sinh nội dung cảnh và Streaming Contract (`Q4.6` — APPROVED v2.4.4)

  > L185: ```text

  > L186: Generate / Regenerate

  > L187: ↓

- **REQ-CMD-032** `docs/business/be-core.md#L2149-L2149`: MUST implement typed IPC #32 `start_scene_generation` with the specified authorization, DTO, transaction and error behavior. (planning coverage `ASSIGNED`).

- **REQ-CMD-033** `docs/business/be-core.md#L2150-L2150`: MUST implement typed IPC #33 `cancel_scene_generation` with the specified authorization, DTO, transaction and error behavior. (planning coverage `ASSIGNED`).

- **REQ-CMD-034** `docs/business/be-core.md#L2151-L2151`: MUST implement typed IPC #34 `respond_edit_scope_expansion` with the specified authorization, DTO, transaction and error behavior. (planning coverage `ASSIGNED`).

- **REQ-SPECTEST-T-LOCK-01** `docs/business/be-core.md#L2536-L2536`: MUST implement executable BE acceptance vector T-LOCK-01, preserving scenario and expected result; old v1-only recovery claims remain legacy-only under B+. (planning coverage `ASSIGNED`).

**Targeted source references (read only these ranges unless a test proves broader context is necessary):**

- `docs/business/core.md#L183-L222` — focus: Generation metadata, invalidation, review; CLI: `sed -n "183,222p" docs/business/core.md` (or `rg -n "<symbol>" docs/business/core.md` for a changed anchor).

- `docs/business/be-core.md#L1741-L1800` — focus: EditScope and stream lifecycle; CLI: `sed -n "1741,1800p" docs/business/be-core.md` (or `rg -n "<symbol>" docs/business/be-core.md` for a changed anchor).

- `docs/business/be-core.md#L2238-L2240` — focus: IPC #32–34; CLI: `sed -n "2238,2240p" docs/business/be-core.md` (or `rg -n "<symbol>" docs/business/be-core.md` for a changed anchor).

**Owned IPC command contracts (verbatim narrow BE §10.2 rows; proposed, not currently implemented):**

- IPC-32 `start_scene_generation` — `docs/business/be-core.md#L2238`: **`start_scene_generation`**  ·  `{ projectId: Uuid, chapterId: Uuid, sceneId: Uuid, mode: "generate" \ ·  "regenerate", note?: String, currentWorkingContent?: String, onEvent: Channel<SceneGenerationStreamEvent> }`  ·  `SceneGenerationResultDto { requestId: Uuid, generationId: Uuid, sceneId: Uuid, baselineVersionNumber: i32, status: GenerationStreamStatus, authorizationValid: bool, generatedText: String, normalizedNote?: String }`  ·  Kiểm tra chương chưa có generation nào đang chạy (`core.md` §1.6: tối đa 1 generation/chương). Kiểm tra `scenes.is_locked == false` trong DB (`LOCK-04`). Khởi tạo `EditScope = {sceneId}` (`SCOPE-01`, `Q4.9`). Chuẩn hóa `note`: trim, rỗng/khoảng trắng $\rightarrow$ `None`, tối đa 2.000 ký tự (`core.md` §1.6).  ·  **Không** ghi vào `scenes.content` hay `scene_versions` (`core.md` §1.6). Kết quả chỉ lưu trong `SceneGenerationBuffer` của phiên làm việc để chờ người dùng Review $\rightarrow$ Accept/Discard.  ·  `SceneLocked`, `OutOfEditScope`, `ValidationFailed`, `ProviderUnreachable`, `ProviderAuthFailed`  ·  Hỗ trợ hủy qua `cancel_scene_generation` hoặc khi người dùng bấm `set_scene_lock(true)` (`LOCK-04`).

- IPC-33 `cancel_scene_generation` — `docs/business/be-core.md#L2239`: **`cancel_scene_generation`**  ·  `{ chapterId: Uuid, generationId: Uuid }`  ·  `{ generationId: Uuid, status: "cancelled" }`  ·  Xác minh `generationId` đang chạy trong `chapterId`.  ·  Kích hoạt `scene_generation_token.cancel()`, đặt `status = Cancelled` trong RAM buffer.  ·  `EntityNotFound`  ·  Idempotent khi gọi lặp.

- IPC-34 `respond_edit_scope_expansion` — `docs/business/be-core.md#L2240`: **`respond_edit_scope_expansion`**  ·  `{ chapterId: Uuid, requestId: Uuid, targetSceneId: Uuid, approved: bool }`  ·  `{ requestId: Uuid, allowedSceneIds: Uuid[], targetSceneIsLocked: bool }`  ·  Xác thực quyết định của người dùng khi AI trong Chat của chương xin mở rộng `EditScope` sang `targetSceneId` (`SL-09`, `SL-10`, `Q4.9`).  ·  Cập nhật `RequestEditScope` trong RAM; **tuyệt đối không** tự động mở khóa nếu `targetSceneId` đang có `is_locked == true` (`SL-10`).  ·  `EntityNotFound`, `SceneLocked`  ·  Không áp dụng.

## 5. Prerequisites

`depends_on: ["phase4_sprint4_deep1", "phase4_sprint3_deep1", "phase2_sprint4_deep3"]`.

- `phase4_sprint4_deep1` [RUNTIME_DEPENDENCY] produces WritingPersistence/read model and IPC #29–31,#35–37; inspect its accepted commit, exported artifact/test and `gate_phase4_sprint4_deep1`. If absent, do not start.

- `phase4_sprint3_deep1` [RUNTIME_DEPENDENCY] produces Project/Character services, repos and IPC #22–28; inspect its accepted commit, exported artifact/test and `gate_phase4_sprint3_deep1`. If absent, do not start.

- `phase2_sprint4_deep3` [INTEGRATION_DEPENDENCY] produces LlmProvider trait, adapters, credential/redaction APIs and config IPC #38–40; inspect its accepted commit, exported artifact/test and `gate_phase2_sprint4_deep3`. If absent, do not start.

## 6. Inputs

Required: accepted outputs of all predecessors; frozen committed SSOT ranges above; actual source paths listed below. Optional context: existing UI mocks for fixtures only. No `report/` input; no historical research.

## 7. File ownership

```yaml

allowed_write_paths:

  - src-tauri/src/service/writing_generation.rs

  - src-tauri/src/ipc/writing_generation_commands.rs

  - src-tauri/tests/writing_generation.rs

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

PROPOSED_INTERFACE: WritingGeneration::{start,cancel,expand_scope}; IPC #32–34, SceneGenerationStreamEvent. Every consumed prerequisite interface is `PREREQUISITE_INTERFACE`, not an existing runtime guarantee. If a named signature conflicts with committed SSOT, stop `CONTRACT_BLOCKED`.

## 9. Implementation requirements

- Implement RAM buffer, cancellation token and generation commands #32–34; no direct scene content/versions write.

- Test lock-unlock-accept rejection and valid-completed-after-scope-expiry accept path via fake provider.

- Prefer official compatible CLI/tooling over manual generated files; verify installed version and diff. No external LLM/Jev request without authorization; optional Jev advice never overrides SSOT.

## 10. Domain invariants

- At most one active generation/chapter; scene must be unlocked and inside request scope; note trim only for current request.

- Lock during stream invalidates generation forever even after Unlock; completed valid output survives scope expiry for review/accept.

- No partial state escapes; protected provenance, explicit authorization, no external-effect recovery replay where applicable.

## 11. Failure matrix

| Trigger | Expected behavior / classification | Durable effects and recovery | Required test |

|---|---|---|---|

| Provider error/disconnect | failed event, partial buffer non-accept; P4S5-N. | No unauthorized partial commit; re-read actual durable state before retry | phase4_sprint5_deep2-F1 |

| Out-of-scope/locked scene | reject before provider call; P4S5-C. | No unauthorized partial commit; re-read actual durable state before retry | phase4_sprint5_deep2-F2 |

| Missing accepted predecessor / baseline mismatch | `CONTRACT_BLOCKED`; do not run implementation | No user-data effects; record issue and owner in sprint evidence | phase4_sprint5_deep2-F0 |

## 12. Acceptance tests

Tests below name **future targets**, not tests executed during planning. Each row must become an executable test before the sprint can be accepted.

| Test ID | Initial state | Input / action | Expected output and state | Forbidden side effects / invariant |

|---|---|---|---|---|

| P4S5-P | disposable uniquely-named SQLite S0 + accepted predecessor fixture | In the owned test target, set up the fixture and run the scenario: started→delta→completed carries generationId and valid metadata. | started→delta→completed carries generationId and valid metadata. | No unrelated write; preserve exact expected state and listed invariant |

| P4S5-N | disposable uniquely-named SQLite S0 + accepted predecessor fixture | In the owned test target, set up the fixture and run the scenario: Lock midstream emits invalidated_by_lock, Unlock never revives old result. | Lock midstream emits invalidated_by_lock, Unlock never revives old result. | No unrelated write; preserve exact expected state and listed invariant |

| P4S5-B | disposable uniquely-named SQLite S0 + accepted predecessor fixture | In the owned test target, set up the fixture and run the scenario: generation canceled leaves persisted content unchanged. | generation canceled leaves persisted content unchanged. | No unrelated write; preserve exact expected state and listed invariant |

| phase4_sprint5_deep2-F1 | accepted prerequisite fixture / isolated worktree | Inject Provider error/disconnect into owned operation | failed event, partial buffer non-accept; P4S5-N. | No unauthorized commit or external replay |

| phase4_sprint5_deep2-F2 | accepted prerequisite fixture / isolated worktree | Inject Out-of-scope/locked scene into owned operation | reject before provider call; P4S5-C. | No unauthorized commit or external replay |

| IPC-CMD-032 | isolated authorized and unauthorized caller fixtures | Invoke `start_scene_generation` with valid request and invalid/stale variant; use frozen DTO | /  **`start_scene_generation`**  /  `{ projectId: Uuid, chapterId: Uuid, sceneId: Uuid, mode: "generate" \ /  "regenerate", note?: String, currentWorkingContent?: String, onEvent: Channel<SceneGenerationStreamEvent> }`  /  `SceneGenerationResultDto { requestId: Uuid, generationId: Uuid, sceneId: Uuid, baselineVersionNumber: i32, status: GenerationStreamStatus, authorizationValid: bool, generatedText: String, normalizedNote?: String }`  /  Kiểm tra chương chưa có generation nào đang chạy (`core.md` §1.6: tối đa 1 generation/chương). Kiểm tra `scenes.is_locked == false` trong DB (`LOCK-04`). Khởi tạo `EditScope = {sceneId}` (`SCOPE-01`, `Q4.9`). Chuẩn hóa `note`: trim, rỗng/khoảng trắng $\rightarrow$ `None`, tối đa 2.000 ký tự (`core.md` §1.6).  /  **Không** ghi vào `scenes.content` hay `scene_versions` (`core.md` §1.6). Kết quả chỉ lưu trong `SceneGenerationBuffer` của phiên làm việc để chờ người dùng Review $\rightarrow$ Accept/Discard.  /  `SceneLocked`, `OutOfEditScope`, `ValidationFailed`, `ProviderUnreachable`, `ProviderAuthFailed`  /  Hỗ trợ hủy qua `cancel_scene_generation` hoặc khi người dùng bấm `set_scene_lock(true)` (`LOCK-04`).  / | No unlisted command, no unauthorized write; assert event/error/transaction result |

| IPC-CMD-033 | isolated authorized and unauthorized caller fixtures | Invoke `cancel_scene_generation` with valid request and invalid/stale variant; use frozen DTO | /  **`cancel_scene_generation`**  /  `{ chapterId: Uuid, generationId: Uuid }`  /  `{ generationId: Uuid, status: "cancelled" }`  /  Xác minh `generationId` đang chạy trong `chapterId`.  /  Kích hoạt `scene_generation_token.cancel()`, đặt `status = Cancelled` trong RAM buffer.  /  `EntityNotFound`  /  Idempotent khi gọi lặp.  / | No unlisted command, no unauthorized write; assert event/error/transaction result |

| IPC-CMD-034 | isolated authorized and unauthorized caller fixtures | Invoke `respond_edit_scope_expansion` with valid request and invalid/stale variant; use frozen DTO | /  **`respond_edit_scope_expansion`**  /  `{ chapterId: Uuid, requestId: Uuid, targetSceneId: Uuid, approved: bool }`  /  `{ requestId: Uuid, allowedSceneIds: Uuid[], targetSceneIsLocked: bool }`  /  Xác thực quyết định của người dùng khi AI trong Chat của chương xin mở rộng `EditScope` sang `targetSceneId` (`SL-09`, `SL-10`, `Q4.9`).  /  Cập nhật `RequestEditScope` trong RAM; **tuyệt đối không** tự động mở khóa nếu `targetSceneId` đang có `is_locked == true` (`SL-10`).  /  `EntityNotFound`, `SceneLocked`  /  Không áp dụng.  / | No unlisted command, no unauthorized write; assert event/error/transaction result |

| T-LOCK-01 | disposable/synthetic fixture described by BE | (Case A) Bắt đầu streaming generation trên Cảnh 1 $\rightarrow$ bấm `set_scene_lock(true)` giữa chừng $\rightarrow$ bấm `set_scene_lock(false)` (Unlock) $\rightarrow$ cố bấm Accept generation đó. (Case C/D) Sinh hoàn tất hợp lệ trên Cảnh 2 $\rightarrow$ `EditScope` hết hạn khi request kết thúc $\rightarrow$ bấm Accept và Save. (Case G/H) AI cố sinh trên cảnh ngoài `EditScope` hoặc cảnh đang khóa. | (Case A) `scenes.is_locked = true` được persist ngay xuống SQLite mà không tạo `scene_versions` và không mất dirty text; stream bị hủy ngay với `status = InvalidatedByLock, authorization_valid = false`; kể cả sau khi Unlock lại Cảnh 1, lệnh Accept cho `generation_id` đó vẫn bị chặn vĩnh viễn (`GenerationInvalidatedByLock`). (Case C/D) Cảnh 2 vẫn Accept và Save thành công sau khi request kết thúc. (Case G/H) Bị chặn ngay bởi `OutOfEditScope` / `SceneLocked`. | Only owner-authorized effects; old v1-only statements interpreted as legacy, no B+ regression |

| phase4_sprint5_deep2-F0 | predecessor gate absent or baseline SHA drift | Attempt dispatch before predecessor accepted | CONTRACT_BLOCKED; no implementation started | No source/DB modification |

Backend module wiring rule: this sprint must not edit any `mod.rs`/`lib.rs`. Its integration test loads each owned implementation file via `#[path = "..."] mod <name>;` and fixture modules via the same mechanism. Production registration occurs only in P6S2 after this sprint is accepted. A compile success is therefore scoped to this test target, not a claim that the whole crate is wired.

Future command after the target files exist: `cargo test --manifest-path src-tauri/Cargo.toml --test writing_generation  # future target, after bootstrap`.

## 13. Database and migration safety

Use `BEGIN IMMEDIATE` only for the assigned TX; WAL/FULL/FK ON/busy_timeout as specified. Use unique disposable DB/temp root per worktree. Never open a user application DB; no destructive migration without separate authorization. Backup/rollback and PRAGMA user_version are owned by P2S3. TX04 never rolls back committed TX08.

## 14. Concurrency contract

- No same-phase/same-depth peer. Cross-phase scheduling still requires exact predecessor gates and conflict scan.

Isolation: own worktree/branch from baseline or accepted integration SHA; unique temp root, test DB and port; no shared mutable process/global fixture. Same depth alone is NOT permission to dispatch.

## 15. Deliverables

- [ ] WritingGeneration service, SceneGenerationStreamEvent and IPC #32–34

- [ ] Tests above and evidence including command exit codes, scope diff and accepted commit SHA.

## 16. Completion criteria

`SPRINT_IMPLEMENTED` requires artifacts, `SPRINT_TESTED` requires executed acceptance tests, `SPRINT_INTEGRATED` requires merged checks, and **`SPRINT_ACCEPTED` requires `gate_phase4_sprint5_deep2` accepted**. A contract-evidence sprint cannot be ACCEPTED for an affected implementation successor while its owner decision is unresolved. No agent declaration alone unlocks successors. All required assertions, file scope and baseline must pass.

## 17. Handoff contract

- WritingGeneration service, SceneGenerationStreamEvent and IPC #32–34

- Exact artifacts: src-tauri/src/service/writing_generation.rs, src-tauri/src/ipc/writing_generation_commands.rs, src-tauri/tests/writing_generation.rs; accepted integration SHA and `gate_phase4_sprint5_deep2` record. A successor must verify exports/ABI rather than infer from prose.

## 18. Resume and failure behavior

Not started: verify baseline and deps. Partial/test failure: inspect git status, changed paths and durable test DB, resume only safe incomplete steps. Implemented but unintegrated: do not dispatch successors. Interrupted agent/rejected patch: preserve branch, rebase only after review, rerun tests. Specification conflict: `CONTRACT_BLOCKED`, record evidence, request authority resolution; never edit SSOT in this sprint.

**Explicit unblock requirements:** CONTRACT_REVIEW_REQUIRED:REQ-WRITE-002.


---

Subagent budget: 256K tokens maximum; start with the bounded references above, do not read entire SSOT by default. CLI-first with verified tool compatibility. Treat Jev/TypeSafe as optional advisory only when available and authorized; never send private repository content externally without consent.
