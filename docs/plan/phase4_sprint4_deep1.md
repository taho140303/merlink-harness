---
sprint_id: phase4_sprint4_deep1
title: Persist Writing read-model locks and chapter Save
phase: 4
sprint: 4
deep: 1
baseline_commit: 57a462efb953a665e3673b15e6cc33622a4f2d39
status: PLANNED
---

# phase4_sprint4_deep1 — Persist Writing read-model locks and chapter Save

## 1. Sprint identity

Engineering phase 4 (Backend services and Writing Mode), within-phase depth 1; baseline is the Git commit above, not an uncommitted workspace. **PROPOSED, NOT IMPLEMENTED.**

## 2. Objective

Persist Writing read-model locks and chapter Save. Observable handoff: WritingPersistence/read model and IPC #29–31,#35–37.

## 3. Scope boundaries

**IN_SCOPE:** Implement reading/locking/saving repositories and IPC #29–31,#35–37, no generation handler code.; Test clean locked scene does not block other dirty Save, dirty locked scene aborts all; receipt lost-response retry.

**OUT_OF_SCOPE / MUST NOT:** modify authoritative `docs/business/**`, `docs/db.dbml`, `docs/architecture/decisions/**`; use real application data; run production migration; add client request IDs to SF-06; invent business semantics; rewrite another sprint’s files.

## 4. Authoritative contract snapshot

- TX06 lock persists before UI success and generation invalidation callback; failed DB write cannot fake locked status.

- TX07 only dirty content scenes; expected MAX(scene_versions.version_number), lock and ownership checked in one BEGIN IMMEDIATE; same clientSaveId returns original.

- **REQ-WRITE-001** `docs/business/core.md#L162-L181`: Writing opens only on committed plan; chapter-first real scene IDs and ordered read model. (planning coverage `ASSIGNED`).

  Normative excerpt (verbatim baseline lines, truncated to 300 chars; use `sed` for the full range):

  > L162: ### 1.6 Hợp đồng Writing Mode: chương-first, cảnh là block

  > L164: > **Trạng thái:** APPROVED (v2.4..v2.4.4). Đây là business contract đã chốt đầy đủ; chi tiết giao diện và route thuộc `fe-core.md` §9; các đề xuất bổ sung cột schema tối thiểu nằm tại `[SCHEMA FOLLOW-UP]` (chờ phê duyệt riêng trước khi sửa `docs/db.dbml`).

  > L166: #### Mô hình chương và cảnh và Read-Model Identity (`Q4.5` — APPROVED v2.4.4)

  > L168: * Một chương là một dãy cảnh có thứ tự. Trong FE, mỗi block nội dung đại diện đúng một scene; block không phải paragraph và không gộp nhiều scene.

- **REQ-WRITE-003** `docs/business/core.md#L224-L335`: TX-06 immediate user-only lock; TX-07 atomic chapter Save OCC/idempotency/dirty-scene only. (planning coverage `CONTRACT_VALIDATION_REQUIRED`).

  Normative excerpt (verbatim baseline lines, truncated to 300 chars; use `sed` for the full range):

  > L224: #### Save cấp chương, OCC, Idempotency và Retry Recovery (`Q4.1`, `Q4.2`, `Q4.4`, `Q4.7` — APPROVED v2.4.4)

  > L226: * Save là hành động cấp chương do người dùng điều khiển; chỉ các scene có thay đổi nội dung chưa lưu (dirty content) được đưa vào Save. (Trạng thái khóa `scenes.is_locked` đã được persist ngay khi bấm Lock/Unlock theo `LOCK-01`, không chờ Save chương).

  > L227: * Backend tương lai phải thực hiện Save nguyên tử trong một SQLite transaction (`BEGIN IMMEDIATE`) (`LOCK-05`, `SCOPE-05`, `Q4.2`). Save không yêu cầu còn một AI request đang hoạt động và không yêu cầu kích hoạt lại `EditScope` đã hết hạn.

  > L228: * **Kiểm tra Optimistic Concurrency (OCC) trên lịch sử version hiện có (`Q4.2` — APPROVED v2.4.4):** Sử dụng lịch sử `scene_versions` hiện có làm chuẩn OCC mà không cần thêm cột `scenes.revision` cho V1. Với mỗi scene có nội dung dirty trong yêu cầu Save, bên trong cùng một transaction nguyên tử, ba

- **REQ-CMD-029** `docs/business/be-core.md#L2146-L2146`: MUST implement typed IPC #29 `get_chapter_writing_state` with the specified authorization, DTO, transaction and error behavior. (planning coverage `ASSIGNED`).

- **REQ-CMD-030** `docs/business/be-core.md#L2147-L2147`: MUST implement typed IPC #30 `set_scene_lock` with the specified authorization, DTO, transaction and error behavior. (planning coverage `ASSIGNED`).

- **REQ-CMD-031** `docs/business/be-core.md#L2148-L2148`: MUST implement typed IPC #31 `set_chapter_scenes_lock_bulk` with the specified authorization, DTO, transaction and error behavior. (planning coverage `ASSIGNED`).

- **REQ-CMD-035** `docs/business/be-core.md#L2152-L2152`: MUST implement typed IPC #35 `list_scene_versions` with the specified authorization, DTO, transaction and error behavior. (planning coverage `ASSIGNED`).

- **REQ-CMD-036** `docs/business/be-core.md#L2153-L2153`: MUST implement typed IPC #36 `save_chapter` with the specified authorization, DTO, transaction and error behavior. (planning coverage `ASSIGNED`).

- **REQ-CMD-037** `docs/business/be-core.md#L2154-L2154`: MUST implement typed IPC #37 `get_chapter_save_receipt` with the specified authorization, DTO, transaction and error behavior. (planning coverage `ASSIGNED`).

- **REQ-TX-006** `docs/business/be-core.md#L504-L504`: MUST own TX-06 in one BEGIN IMMEDIATE boundary, with documented rollback and independent effects. (planning coverage `ASSIGNED`).

- **REQ-TX-007** `docs/business/be-core.md#L505-L505`: MUST own TX-07 in one BEGIN IMMEDIATE boundary, with documented rollback and independent effects. (planning coverage `ASSIGNED`).

- **REQ-SPECTEST-T-SAVE-01** `docs/business/be-core.md#L2537-L2537`: MUST implement executable BE acceptance vector T-SAVE-01, preserving scenario and expected result; old v1-only recovery claims remain legacy-only under B+. (planning coverage `ASSIGNED`).

**Targeted source references (read only these ranges unless a test proves broader context is necessary):**

- `docs/business/core.md#L162-L181` — focus: Committed-plan and real scene IDs; CLI: `sed -n "162,181p" docs/business/core.md` (or `rg -n "<symbol>" docs/business/core.md` for a changed anchor).

- `docs/business/core.md#L224-L335` — focus: Lock/OCC/Save idempotency; CLI: `sed -n "224,335p" docs/business/core.md` (or `rg -n "<symbol>" docs/business/core.md` for a changed anchor).

- `docs/business/be-core.md#L1681-L1739` — focus: Read model and TX06; CLI: `sed -n "1681,1739p" docs/business/be-core.md` (or `rg -n "<symbol>" docs/business/be-core.md` for a changed anchor).

- `docs/business/be-core.md#L1802-L2025` — focus: TX07; CLI: `sed -n "1802,2025p" docs/business/be-core.md` (or `rg -n "<symbol>" docs/business/be-core.md` for a changed anchor).

- `docs/business/be-core.md#L2231-L2243` — focus: Writing IPC #29–31,#35–37; CLI: `sed -n "2231,2243p" docs/business/be-core.md` (or `rg -n "<symbol>" docs/business/be-core.md` for a changed anchor).

**Owned IPC command contracts (verbatim narrow BE §10.2 rows; proposed, not currently implemented):**

- IPC-29 `get_chapter_writing_state` — `docs/business/be-core.md#L2235`: **`get_chapter_writing_state`**  ·  `{ projectId: Uuid, chapterId: Uuid }`  ·  `ChapterWritingReadModelDto { projectId, chapterId, arcId, chapterTitle, position, updatedAt, scenes: SceneReadModelDto[] }`  ·  Kiểm tra **Committed-Plan Gate** (`WR-01`); trả danh sách cảnh sắp theo `scenes.position ASC` với đầy đủ `id` (`scenes.id` thật), `position`, `displayName`, `summary`, `content`, `wordCount`, `isLocked`, `headVersionNumber`, `currentSceneSource` (`Q4.5`).  ·  Read-only.  ·  `EntityNotFound`, `ValidationFailed`, `DatabaseError`  ·  Không áp dụng.

- IPC-30 `set_scene_lock` — `docs/business/be-core.md#L2236`: **`set_scene_lock`**  ·  `{ projectId: Uuid, chapterId: Uuid, sceneId: Uuid, isLocked: bool }`  ·  `SceneLockStateDto { sceneId: Uuid, isLocked: bool, invalidatedGenerationId?: Uuid }`  ·  **Độc quyền `CallerContext::UiAction`** (`LOCK-01`, `LOCK-02`). Nếu `isLocked == true` và cảnh đang streaming generation $\rightarrow$ hủy stream ngay và đánh dấu `invalidated_by_lock` (`authorization_valid = false` vĩnh viễn — `LOCK-04`).  ·  Thực thi **`TX-06` (`BEGIN IMMEDIATE`)**: cập nhật ngay `scenes.is_locked` trong SQLite; **không** tạo `scene_versions` (`LOCK-07`); bảo toàn mọi dirty state/buffer (`LOCK-03`).  ·  `EntityNotFound`, `UnauthorizedCaller`, `DatabaseError`  ·  Hoàn tất tức thời; nếu DB lỗi, trả lỗi để UI không đổi trạng thái khóa giả.

- IPC-31 `set_chapter_scenes_lock_bulk` — `docs/business/be-core.md#L2237`: **`set_chapter_scenes_lock_bulk`**  ·  `{ projectId: Uuid, chapterId: Uuid, isLocked: bool }`  ·  `BulkSceneLockStateDto { chapterId: Uuid, scenes: SceneLockStateDto[] }`  ·  Độc quyền `CallerContext::UiAction` (phục vụ Lock All / Unlock All trên `ChapterDetailView`). Nếu đang có generation active và yêu cầu Lock All $\rightarrow$ hủy generation đó theo `LOCK-04`.  ·  Thực thi **`TX-06` (`BEGIN IMMEDIATE`)**: cập nhật `scenes.is_locked = ?1 WHERE chapter_id = ?2` trong 1 giao dịch; không tạo `scene_versions`.  ·  `EntityNotFound`, `UnauthorizedCaller`, `DatabaseError`  ·  Hoàn tất tức thời.

- IPC-35 `list_scene_versions` — `docs/business/be-core.md#L2241`: **`list_scene_versions`**  ·  `{ projectId: Uuid, chapterId: Uuid, sceneId: Uuid }`  ·  `SceneVersionListDto { sceneId: Uuid, headVersionNumber: i32, versions: SceneVersionItemDto[] }`  ·  Xác minh `sceneId` thuộc `chapterId`.  ·  Read-only (`SELECT` trên `scene_versions WHERE parent_id = ?1 ORDER BY version_number DESC`).  ·  `EntityNotFound`, `DatabaseError`  ·  Không áp dụng.

- IPC-36 `save_chapter` — `docs/business/be-core.md#L2242`: **`save_chapter`**  ·  `SaveChapterRequest { projectId: Uuid, chapterId: Uuid, clientSaveId: String, dirtyScenes: DirtySceneSaveItem[] }`  ·  `SaveChapterResponse { chapterId: Uuid, clientSaveId: String, savedScenes: SavedSceneResultDto[], chapterScenes: SceneReadModelDto[], wasIdempotentReplay: bool }`  ·  Kiểm tra đủ 5 tính chất idempotency của `clientSaveId` (`Q4.1`, `SF-02`) và kiểm tra OCC (`expected_version_number == MAX(version_number)`) + `scenes.is_locked == false` trên mọi cảnh trong `dirtyScenes` (`LOCK-05`, `Q4.2`). Không yêu cầu còn `EditScope` đang mở (`SCOPE-05`).  ·  Thực thi **`TX-07` (`BEGIN IMMEDIATE`)**: cập nhật `scenes(content, word_count)`, chèn `scene_versions` (`parent_id NOT NULL` — `SF-07`), và chèn biên nhận bất biến vào `chapter_save_receipts` (`SF-02`). Rollback toàn bộ chương nếu bất kỳ cảnh dirty nào bị khóa hoặc lệch OCC.  ·  `SceneLocked`, `OccVersionMismatch`, `IdempotencyKeyPayloadMismatch`, `EntityNotFound`, `DatabaseError`  ·  Nguyên tử tuyệt đối; nếu kết quả mập mờ do đứt mạng, FE giữ nguyên `clientSaveId` và gọi lại hoặc gọi `get_chapter_save_receipt` (`Q4.7`).

- IPC-37 `get_chapter_save_receipt` — `docs/business/be-core.md#L2243`: **`get_chapter_save_receipt`**  ·  `{ chapterId: Uuid, clientSaveId: String }`  ·  `SaveReceiptLookupDto { found: bool, receipt?: SaveChapterResponse, currentChapterScenes: SceneReadModelDto[] }`  ·  Phục vụ giao thức khôi phục sau kết quả Save mập mờ (`Q4.7` — `core.md` §1.6).  ·  Read-only (`SELECT` trên `chapter_save_receipts` và `scenes`).  ·  `EntityNotFound`, `DatabaseError`  ·  Không áp dụng.

## 5. Prerequisites

`depends_on: ["phase4_sprint2_deep0", "phase2_sprint5_deep2"]`.

- `phase4_sprint2_deep0` [RUNTIME_DEPENDENCY] produces CommitService/CommitRepo, preview/commit/reopen IPC #19–21; inspect its accepted commit, exported artifact/test and `gate_phase4_sprint2_deep0`. If absent, do not start.

- `phase2_sprint5_deep2` [TYPE_DEPENDENCY] produces Frozen DTO/error/adapter Rust API plus generated TS binding path and schema fixture; inspect its accepted commit, exported artifact/test and `gate_phase2_sprint5_deep2`. If absent, do not start.

## 6. Inputs

Required: accepted outputs of all predecessors; frozen committed SSOT ranges above; actual source paths listed below. Optional context: existing UI mocks for fixtures only. No `report/` input; no historical research.

## 7. File ownership

```yaml

allowed_write_paths:

  - src-tauri/src/storage/writing_repo.rs

  - src-tauri/src/service/writing_persistence.rs

  - src-tauri/src/ipc/writing_persistence_commands.rs

  - src-tauri/tests/writing_persistence.rs

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

PROPOSED_INTERFACE: WritingPersistence::{get_chapter,set_scene_lock,set_bulk_lock,save_chapter,get_receipt}; IPC #29–31,#35–37. Every consumed prerequisite interface is `PREREQUISITE_INTERFACE`, not an existing runtime guarantee. If a named signature conflicts with committed SSOT, stop `CONTRACT_BLOCKED`.

## 9. Implementation requirements

- Implement reading/locking/saving repositories and IPC #29–31,#35–37, no generation handler code.

- Test clean locked scene does not block other dirty Save, dirty locked scene aborts all; receipt lost-response retry.

- Prefer official compatible CLI/tooling over manual generated files; verify installed version and diff. No external LLM/Jev request without authorization; optional Jev advice never overrides SSOT.

## 10. Domain invariants

- TX06 lock persists before UI success and generation invalidation callback; failed DB write cannot fake locked status.

- TX07 only dirty content scenes; expected MAX(scene_versions.version_number), lock and ownership checked in one BEGIN IMMEDIATE; same clientSaveId returns original.

- No partial state escapes; protected provenance, explicit authorization, no external-effect recovery replay where applicable.

## 11. Failure matrix

| Trigger | Expected behavior / classification | Durable effects and recovery | Required test |

|---|---|---|---|

| Unresolved lock persist/invalidation sequencing | CONTRACT_BLOCKED; P4S4-N. | No unauthorized partial commit; re-read actual durable state before retry | phase4_sprint4_deep1-F1 |

| Any dirty scene stale/locked | rollback whole chapter and FE keeps dirty buffer; P4S4-C. | No unauthorized partial commit; re-read actual durable state before retry | phase4_sprint4_deep1-F2 |

| Missing accepted predecessor / baseline mismatch | `CONTRACT_BLOCKED`; do not run implementation | No user-data effects; record issue and owner in sprint evidence | phase4_sprint4_deep1-F0 |

## 12. Acceptance tests

Tests below name **future targets**, not tests executed during planning. Each row must become an executable test before the sprint can be accepted.

| Test ID | Initial state | Input / action | Expected output and state | Forbidden side effects / invariant |

|---|---|---|---|---|

| P4S4-P | disposable uniquely-named SQLite S0 + accepted predecessor fixture | In the owned test target, set up the fixture and run the scenario: save two dirty scenes creates two scene_versions and one chapter receipt. | save two dirty scenes creates two scene_versions and one chapter receipt. | No unrelated write; preserve exact expected state and listed invariant |

| P4S4-N | disposable uniquely-named SQLite S0 + accepted predecessor fixture | In the owned test target, set up the fixture and run the scenario: same ID different payload | same ID different payload => mismatch and no new version. | No unrelated write; preserve exact expected state and listed invariant |

| P4S4-B | disposable uniquely-named SQLite S0 + accepted predecessor fixture | In the owned test target, set up the fixture and run the scenario: Lock→Unlock without content change does not bump OCC; lock-only not content dirty. | Lock→Unlock without content change does not bump OCC; lock-only not content dirty. | No unrelated write; preserve exact expected state and listed invariant |

| phase4_sprint4_deep1-F1 | accepted prerequisite fixture / isolated worktree | Inject Unresolved lock persist/invalidation sequencing into owned operation | CONTRACT_BLOCKED; P4S4-N. | No unauthorized commit or external replay |

| phase4_sprint4_deep1-F2 | accepted prerequisite fixture / isolated worktree | Inject Any dirty scene stale/locked into owned operation | rollback whole chapter and FE keeps dirty buffer; P4S4-C. | No unauthorized commit or external replay |

| IPC-CMD-029 | isolated authorized and unauthorized caller fixtures | Invoke `get_chapter_writing_state` with valid request and invalid/stale variant; use frozen DTO | /  **`get_chapter_writing_state`**  /  `{ projectId: Uuid, chapterId: Uuid }`  /  `ChapterWritingReadModelDto { projectId, chapterId, arcId, chapterTitle, position, updatedAt, scenes: SceneReadModelDto[] }`  /  Kiểm tra **Committed-Plan Gate** (`WR-01`); trả danh sách cảnh sắp theo `scenes.position ASC` với đầy đủ `id` (`scenes.id` thật), `position`, `displayName`, `summary`, `content`, `wordCount`, `isLocked`, `headVersionNumber`, `currentSceneSource` (`Q4.5`).  /  Read-only.  /  `EntityNotFound`, `ValidationFailed`, `DatabaseError`  /  Không áp dụng.  / | No unlisted command, no unauthorized write; assert event/error/transaction result |

| IPC-CMD-030 | isolated authorized and unauthorized caller fixtures | Invoke `set_scene_lock` with valid request and invalid/stale variant; use frozen DTO | /  **`set_scene_lock`**  /  `{ projectId: Uuid, chapterId: Uuid, sceneId: Uuid, isLocked: bool }`  /  `SceneLockStateDto { sceneId: Uuid, isLocked: bool, invalidatedGenerationId?: Uuid }`  /  **Độc quyền `CallerContext::UiAction`** (`LOCK-01`, `LOCK-02`). Nếu `isLocked == true` và cảnh đang streaming generation $\rightarrow$ hủy stream ngay và đánh dấu `invalidated_by_lock` (`authorization_valid = false` vĩnh viễn — `LOCK-04`).  /  Thực thi **`TX-06` (`BEGIN IMMEDIATE`)**: cập nhật ngay `scenes.is_locked` trong SQLite; **không** tạo `scene_versions` (`LOCK-07`); bảo toàn mọi dirty state/buffer (`LOCK-03`).  /  `EntityNotFound`, `UnauthorizedCaller`, `DatabaseError`  /  Hoàn tất tức thời; nếu DB lỗi, trả lỗi để UI không đổi trạng thái khóa giả.  / | No unlisted command, no unauthorized write; assert event/error/transaction result |

| IPC-CMD-031 | isolated authorized and unauthorized caller fixtures | Invoke `set_chapter_scenes_lock_bulk` with valid request and invalid/stale variant; use frozen DTO | /  **`set_chapter_scenes_lock_bulk`**  /  `{ projectId: Uuid, chapterId: Uuid, isLocked: bool }`  /  `BulkSceneLockStateDto { chapterId: Uuid, scenes: SceneLockStateDto[] }`  /  Độc quyền `CallerContext::UiAction` (phục vụ Lock All / Unlock All trên `ChapterDetailView`). Nếu đang có generation active và yêu cầu Lock All $\rightarrow$ hủy generation đó theo `LOCK-04`.  /  Thực thi **`TX-06` (`BEGIN IMMEDIATE`)**: cập nhật `scenes.is_locked = ?1 WHERE chapter_id = ?2` trong 1 giao dịch; không tạo `scene_versions`.  /  `EntityNotFound`, `UnauthorizedCaller`, `DatabaseError`  /  Hoàn tất tức thời.  / | No unlisted command, no unauthorized write; assert event/error/transaction result |

| IPC-CMD-035 | isolated authorized and unauthorized caller fixtures | Invoke `list_scene_versions` with valid request and invalid/stale variant; use frozen DTO | /  **`list_scene_versions`**  /  `{ projectId: Uuid, chapterId: Uuid, sceneId: Uuid }`  /  `SceneVersionListDto { sceneId: Uuid, headVersionNumber: i32, versions: SceneVersionItemDto[] }`  /  Xác minh `sceneId` thuộc `chapterId`.  /  Read-only (`SELECT` trên `scene_versions WHERE parent_id = ?1 ORDER BY version_number DESC`).  /  `EntityNotFound`, `DatabaseError`  /  Không áp dụng.  / | No unlisted command, no unauthorized write; assert event/error/transaction result |

| IPC-CMD-036 | isolated authorized and unauthorized caller fixtures | Invoke `save_chapter` with valid request and invalid/stale variant; use frozen DTO | /  **`save_chapter`**  /  `SaveChapterRequest { projectId: Uuid, chapterId: Uuid, clientSaveId: String, dirtyScenes: DirtySceneSaveItem[] }`  /  `SaveChapterResponse { chapterId: Uuid, clientSaveId: String, savedScenes: SavedSceneResultDto[], chapterScenes: SceneReadModelDto[], wasIdempotentReplay: bool }`  /  Kiểm tra đủ 5 tính chất idempotency của `clientSaveId` (`Q4.1`, `SF-02`) và kiểm tra OCC (`expected_version_number == MAX(version_number)`) + `scenes.is_locked == false` trên mọi cảnh trong `dirtyScenes` (`LOCK-05`, `Q4.2`). Không yêu cầu còn `EditScope` đang mở (`SCOPE-05`).  /  Thực thi **`TX-07` (`BEGIN IMMEDIATE`)**: cập nhật `scenes(content, word_count)`, chèn `scene_versions` (`parent_id NOT NULL` — `SF-07`), và chèn biên nhận bất biến vào `chapter_save_receipts` (`SF-02`). Rollback toàn bộ chương nếu bất kỳ cảnh dirty nào bị khóa hoặc lệch OCC.  /  `SceneLocked`, `OccVersionMismatch`, `IdempotencyKeyPayloadMismatch`, `EntityNotFound`, `DatabaseError`  /  Nguyên tử tuyệt đối; nếu kết quả mập mờ do đứt mạng, FE giữ nguyên `clientSaveId` và gọi lại hoặc gọi `get_chapter_save_receipt` (`Q4.7`).  / | No unlisted command, no unauthorized write; assert event/error/transaction result |

| IPC-CMD-037 | isolated authorized and unauthorized caller fixtures | Invoke `get_chapter_save_receipt` with valid request and invalid/stale variant; use frozen DTO | /  **`get_chapter_save_receipt`**  /  `{ chapterId: Uuid, clientSaveId: String }`  /  `SaveReceiptLookupDto { found: bool, receipt?: SaveChapterResponse, currentChapterScenes: SceneReadModelDto[] }`  /  Phục vụ giao thức khôi phục sau kết quả Save mập mờ (`Q4.7` — `core.md` §1.6).  /  Read-only (`SELECT` trên `chapter_save_receipts` và `scenes`).  /  `EntityNotFound`, `DatabaseError`  /  Không áp dụng.  / | No unlisted command, no unauthorized write; assert event/error/transaction result |

| TX-006 | disposable S0 + failure injection at every write step | Execute TX-06 then inject error/lose response | /  **TX-06**  /  **Immediate Scene Lock Toggle** (`set_scene_lock`, `set_chapter_scenes_lock_bulk`)  /  Đọc và cập nhật trực tiếp `scenes.is_locked` cho một cảnh (hoặc các cảnh thuộc chương khi Lock All / Unlock All).  /  Rollback; trả lỗi cho FE; FE giữ nguyên trạng thái hiển thị cũ (`LOCK-01`).  / | Atomic rollback, no unrelated durable effect; retry checks receipt |

| TX-007 | disposable S0 + failure injection at every write step | Execute TX-07 then inject error/lose response | /  **TX-07**  /  **Atomic Save Chapter** (`save_chapter`)  /  Kiểm tra/chèn `chapter_save_receipts` (`SF-02`); kiểm tra OCC (`MAX(scene_versions.version_number)`) và `scenes.is_locked == 0`; cập nhật `scenes(content, word_count)` và chèn `scene_versions` (`SF-07`) cho các cảnh dirty.  /  Nếu bất kỳ cảnh dirty nào bị khóa hoặc lệch version OCC hoặc lỗi ghi: Rollback toàn bộ chương; bảo toàn nguyên vẹn dirty state trên FE (`LOCK-05`, `Q4.1`, `Q4.2`).  / | Atomic rollback, no unrelated durable effect; retry checks receipt |

| T-SAVE-01 | disposable/synthetic fixture described by BE | Trong chương có 3 cảnh: (1) Cảnh 1 dirty (mở khóa), Cảnh 2 clean (đang khóa) $\rightarrow$ gọi `save_chapter(C_1)`; (2) Gọi lại `save_chapter(C_1)` với cùng payload; (3) Gọi `save_chapter(C_1)` với payload khác; (4) Gọi `save_chapter(C_2)` khi Cảnh 2 đang khóa nhưng nằm trong `dirtyScenes`; (5) Gọi `save_chapter(C_3)` với `expected_version_number` cũ. | (1) Thành công nguyên tử (Cảnh 2 clean đang khóa không chặn Save — Case F), tạo đúng 1 `scene_versions` cho Cảnh 1 và 1 biên nhận `chapter_save_receipts`. (2) Trả lại kết quả gốc của `C_1`, không tạo `scene_versions` thứ 2. (3) Trả `IdempotencyKeyPayloadMismatch`. (4) Rollback toàn bộ chương với `SceneLocked` (Case B/E). (5) Rollback toàn bộ chương với `OccVersionMismatch`. | Only owner-authorized effects; old v1-only statements interpreted as legacy, no B+ regression |

| phase4_sprint4_deep1-F0 | predecessor gate absent or baseline SHA drift | Attempt dispatch before predecessor accepted | CONTRACT_BLOCKED; no implementation started | No source/DB modification |

Backend module wiring rule: this sprint must not edit any `mod.rs`/`lib.rs`. Its integration test loads each owned implementation file via `#[path = "..."] mod <name>;` and fixture modules via the same mechanism. Production registration occurs only in P6S2 after this sprint is accepted. A compile success is therefore scoped to this test target, not a claim that the whole crate is wired.

Future command after the target files exist: `cargo test --manifest-path src-tauri/Cargo.toml --test writing_persistence  # future target, after bootstrap`.

## 13. Database and migration safety

Use `BEGIN IMMEDIATE` only for the assigned TX; WAL/FULL/FK ON/busy_timeout as specified. Use unique disposable DB/temp root per worktree. Never open a user application DB; no destructive migration without separate authorization. Backup/rollback and PRAGMA user_version are owned by P2S3. TX04 never rolls back committed TX08.

## 14. Concurrency contract

- Peer `phase4_sprint3_deep1`: candidate parallel only after frozen contracts/gates; NONE: Disjoint exact write paths; shared types frozen by predecessors; worktree-local test resources.

Isolation: own worktree/branch from baseline or accepted integration SHA; unique temp root, test DB and port; no shared mutable process/global fixture. Same depth alone is NOT permission to dispatch.

## 15. Deliverables

- [ ] WritingPersistence/read model and IPC #29–31,#35–37

- [ ] Tests above and evidence including command exit codes, scope diff and accepted commit SHA.

## 16. Completion criteria

`SPRINT_IMPLEMENTED` requires artifacts, `SPRINT_TESTED` requires executed acceptance tests, `SPRINT_INTEGRATED` requires merged checks, and **`SPRINT_ACCEPTED` requires `gate_phase4_sprint4_deep1` accepted**. A contract-evidence sprint cannot be ACCEPTED for an affected implementation successor while its owner decision is unresolved. No agent declaration alone unlocks successors. All required assertions, file scope and baseline must pass.

## 17. Handoff contract

- WritingPersistence/read model and IPC #29–31,#35–37

- Exact artifacts: src-tauri/src/storage/writing_repo.rs, src-tauri/src/service/writing_persistence.rs, src-tauri/src/ipc/writing_persistence_commands.rs, src-tauri/tests/writing_persistence.rs; accepted integration SHA and `gate_phase4_sprint4_deep1` record. A successor must verify exports/ABI rather than infer from prose.

## 18. Resume and failure behavior

Not started: verify baseline and deps. Partial/test failure: inspect git status, changed paths and durable test DB, resume only safe incomplete steps. Implemented but unintegrated: do not dispatch successors. Interrupted agent/rejected patch: preserve branch, rebase only after review, rerun tests. Specification conflict: `CONTRACT_BLOCKED`, record evidence, request authority resolution; never edit SSOT in this sprint.

**Explicit unblock requirements:** TX06_FAILURE_ORDER, CI-06, CONTRACT_REVIEW_REQUIRED:REQ-WRITE-003.


---

Subagent budget: 256K tokens maximum; start with the bounded references above, do not read entire SSOT by default. CLI-first with verified tool compatibility. Treat Jev/TypeSafe as optional advisory only when available and authorized; never send private repository content externally without consent.
