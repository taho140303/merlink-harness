# FE CORE: CƠ CHẾ GIAO DIỆN MERLINK HARNESS

> **Phiên bản:** 0.3.2
> **Đối chiếu:** `docs/business/core.md` v2.4.1
> **Vai trò:** SSOT cơ chế giao diện và hợp đồng tương tác FE của Merlink Harness. Business Core quyết định hệ thống phải làm gì; tài liệu này mô tả giao diện hiện đang làm gì và cần làm gì. `docs/design-system.md` quyết định cách thể hiện thị giác.
> **Phạm vi kiểm tra:** `src/features/chat`, `src/features/draft`, `src/features/business-flow`, `src/features/projects` (gồm ChapterDetail, ProjectDetail, ProjectCreatorPanel, scenePreview), router, Zustand store, i18n và mock data.
> **Thời điểm kiểm tra:** 2026-11-06, sau Stage B / Wave 0.3B (điều hướng preview sau commit mô phỏng) và cleanup tiền-merge Stage B.
> **Giới hạn:** tài liệu này không sửa `src/`, `docs/db.dbml`, `docs/design-system.md`, `package.json` hoặc backend.

## 0. Cách đọc và trạng thái

Mỗi chức năng được đánh dấu bằng một hoặc nhiều nhãn:

| Nhãn | Nghĩa |
| :--- | :--- |
| `[IMPLEMENTED]` | Có mã nguồn và hành vi UI thực tế, chạy được ở mức giao diện. |
| `[MOCK]` | Có giao diện nhưng dữ liệu hoặc hành vi là giả, không có backend thật. |
| `[TARGET]` | Contract đích theo Business Core, chưa triển khai. |
| `[GAP]` | Business Core yêu cầu nhưng FE chưa hỗ trợ. |
| `[CONFLICT]` | Core và FE đang nói khác nhau, hoặc hai cách biểu diễn chưa thống nhất, cần người dùng quyết định. |

Một chức năng có thể có nhiều dòng với trạng thái khác nhau. Không dùng một nhãn `[IMPLEMENTED]` để bao phủ cả chức năng khi chỉ có giao diện mock.

Wave 0 dùng `[IMPLEMENTED]` cho hành vi UI chạy được trong trình duyệt, `[MOCK]` cho dữ liệu/hành vi preview không có backend. Các hợp đồng nghiệp vụ dùng thêm trạng thái `APPROVED`, `OPEN`, `SCHEMA FOLLOW-UP`, `IMPLEMENTATION PENDING`.

Quy ước nguồn:

- **Core:** số mục trong `docs/business/core.md` v2.4.
- **FE:** đường dẫn tương đối và số dòng trong mã nguồn tại thời điểm kiểm tra. Tên component/function được nêu kèm để tránh phụ thuộc số dòng.
- **Không** dùng số dòng của tài liệu này làm nguồn sự thật cho mã nguồn.

## 1. Kiến trúc frontend hiện tại

### 1.1 Stack và giới hạn

| Thành phần | Hiện trạng | Nguồn |
| :--- | :--- | :--- |
| Runtime | React 19, React Router 8, Tailwind CSS 4, Zustand 5 | `package.json` |
| Kiểu ứng dụng | SPA web, build bằng `react-router build` | `package.json`, `react-router.config.ts` |
| Backend | Không có Rust, Tauri, IPC, SQLite, HTTP API | không có `src-tauri`, không có `@tauri-apps/api` trong `package.json` |
| Dữ liệu | Mock data và local state | các file `mock*.ts`, `useState`, Zustand |
| Persistence | Chỉ theme và locale trong `localStorage` | `src/app/root.tsx:41`, `src/shared/i18n/store.ts:8`, `src/shared/hooks/useTheme.ts:14` |
| Dữ liệu nghiệp vụ | Không persist; refresh làm mất chat, draft, project, character | kiểm tra toàn bộ `src/features` |

Hệ quả:

1. Không có Working State thống nhất. Chat và Draft Workspace dùng hai nguồn mock khác nhau; Chapter Detail dùng nguồn mock thứ ba.
2. Không có `draft_session_id`, `session_id`, `head_version_seq`, `pending_patches` hoặc tool execution thật.
3. Không có draft lock, commit transaction, version restore, workflow execution, crash recovery hoặc Writing Mode backend.
4. Writing Mode mới có UI preview tại §9: generation buffer, version dialog và dirty state là local/preview, không có AI, persistence hay `scene_versions` thật.
5. Mọi tuyên bố về các cơ chế trên chỉ là `[TARGET]`, `[GAP]` hoặc `[IMPLEMENTATION PENDING]`.

### 1.2 Nguồn dữ liệu mock

| Nguồn | Dùng cho | Đặc điểm |
| :--- | :--- | :--- |
| `src/features/business-flow/mockData.ts:3` | `SAMPLE_BUSINESS_FLOW_DATA` cho Chat, progress, ProcessDetailDialog | snake_case, không có `source`, không có `pending_patches` |
| `src/features/draft/mockOverview.ts:3` | `DRAFT_OVERVIEW` cho Draft Workspace | camelCase, có `versionHistory`, `segments`, `sceneConfig`; không phải `BusinessFlowData` |
| `src/features/projects/mockProjects.ts:16` | Project list, Project detail, Chapter editor | mock project, arc, chapter, block; mỗi block preview được xem là một scene |
| `src/features/characters/mockData.ts` | Character library, Character detail | mock character, competence, memory, relationship |
| `src/features/system-prompt/mockData.ts` | System prompt/config | mock; ngoài phạm vi draft |

Hai nguồn `SAMPLE_BUSINESS_FLOW_DATA` và `DRAFT_OVERVIEW` không có adapter, không đồng bộ và dùng schema khác nhau. Vì vậy không được coi chúng là hai view của cùng một Working State.

Chapter Detail dùng `buildSceneBlocks(chapter)` (`scenePreview.ts`) để tạo `SceneBlock[]` từ `chapter.blocks`. `sceneId` preview có dạng `scn-{chapter.id}-{block.id}`, chỉ để demo; không phải `scenes.id` và không được persist.

### 1.3 State ownership hiện tại

| State | Nơi giữ | Persist | Ghi chú |
| :--- | :--- | :--- | :--- |
| Chat messages | `useState` trong `ChatView` | Không | mất khi unmount/refresh |
| Draft step, sub-step, version label, feedback mode | `useState` trong `DraftWorkspace` | Không | không có draft id |
| Feedback context | `useState` trong `ChatView` | Không | chỉ là chip trong input |
| Project Detail mode (`view`/`creator`) | URL search param `?mode=` | Có (URL) | mặc định `view`; giá trị lạ fallback `view` |
| Chapter scenes, dirty baseline, generation buffer, source preview | `useState` trong `ChapterDetailView` | Không | local state; refresh mất |
| Scene version dialog | state local + fixture `scenePreview.ts` | Không | dữ liệu mẫu, không phải `scene_versions` |
| Character list, character detail, edit snapshot | Zustand `useCharacterStore` | Không | mock data; `src/features/characters/store.ts` |
| Locale | Zustand + `localStorage` | Có | `merlink.locale` |
| Theme | Zustand + `localStorage` | Có | `theme` |
| Project data | hằng số `MOCK_PROJECTS` | Không | sửa/xóa chỉ đổi state cục bộ |

## 2. Routes và bản đồ màn hình

| Route | Component | Trạng thái | Ghi chú |
| :--- | :--- | :--- | :--- |
| `/` | `ChatView` | `[MOCK]` | chat mới, không có session list/history |
| `/chat/:id` | `ChatView` | `[MOCK]` | cùng component; param `id` chưa được dùng để nạp session |
| `/project` | `ProjectView` | `[MOCK]` | danh sách project, tìm kiếm, sort, xóa local |
| `/project/:projectId` | `ProjectDetailView` | `[MOCK]` | project, arc, chapter, chat pane; `?mode=view\|creator`, mặc định View |
| `/project/:projectId/chapter/:chapterId` | `ChapterDetailView` | `[MOCK]` | chapter-first scene blocks; bề mặt Writing Mode preview, không phải backend |
| `/character` | `CharacterListView` | `[MOCK]` | thư viện nhân vật, filter project/sex, tạo mới local |
| `/character/:characterId` | `CharacterDetailLayout` | `[MOCK]` | tab core/life/psych, edit local, chat pane |
| `/worldbuilding/:sub` | `WorldbuildingView` | `[MOCK]` | màn “Under Construction”; design-system loại trừ |
| `/config` | `ConfigPage` | `[MOCK]` | system prompt/config; ngoài phạm vi draft |

- Draft Workspace **không phải route**. Nó là pane phải trong `ChatView`, mở bằng nút Draft Building và đóng bằng nút collapse trong `DraftHeader`.
- `src/app/routes.ts` khai báo toàn bộ route. Wave 0 **không thêm route**; mode Creator dùng search param trên route project hiện có.
- `ProjectDetailView` đọc `?mode=view|creator`; thiếu hoặc giá trị lạ fallback `view`. Chuyển mode dùng `setSearchParams`, nên back/forward giữ đúng mode.
- `ChapterDetailPage` đọc `?mode=creator` để hiển thị badge Creator, nhưng **không tự bật edit**. Người dùng vẫn phải bấm Chỉnh sửa.
- Breakpoint responsive: dưới `1280px` split pane chuyển thành một pane + overlay; nguồn `src/shared/hooks/useMediaQuery.ts:23`.
- Split pane chat/draft mặc định 50/50, min 545px/530px; nguồn `src/features/chat/ChatView.tsx:66` và `src/shared/hooks/useResizablePanel.ts`.
- Chapter Detail đã có nhánh compact header (`compactHeader` trong `ChapterDetailView.tsx`, Wave 0.2); chi tiết và known issue còn lại tại §9.6.

## 3. Chat và Readiness Gate

### 3.1 Bảng hành vi

| ID | Trạng thái | Quy tắc UI | Dữ liệu/hành vi | Nguồn Core | Nguồn FE | Ghi chú |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| CHAT-01 | `[IMPLEMENTED]` | Chat shell, header, input, message list, welcome, split pane | UI thật; dữ liệu giả | 3.1, 3.2 | `ChatView.tsx:189`, `ChatHeader.tsx:39`, `ChatInputBar.tsx:52` | không có backend |
| CHAT-02 | `[MOCK]` | Header hiển thị một progress bar + nút (i) | `progressPercent` từ `calculateBusinessFlowProgress` trên sample data | 3.1 yêu cầu hai phần trăm | `ChatHeader.tsx:50-61`, `ChatView.tsx:33-38` | xem C2 |
| CHAT-03 | `[IMPLEMENTED]` | Nút Draft Building disabled khi `!isRequireComplete`; tooltip khi disabled | `isRequireComplete` tính từ `SAMPLE_BUSINESS_FLOW_DATA` | 1.3, 2.1, 3.1 | `ChatHeader.tsx:30-36`, `ChatHeader.tsx:79-96`, `ChatView.tsx:39-42` | gate có UI; dữ liệu mock |
| CHAT-04 | `[MOCK]` | Bấm Draft Building mở `PreDraftSuggestionDialog`, không mở draft ngay | Dialog chỉ có confirm/cancel; không có input gợi ý dù i18n có key | 3.4 | `ChatView.tsx:44-64`, `PreDraftSuggestionDialog.tsx:32-52` | entry point đơn giản hóa |
| CHAT-05 | `[MOCK]` | Gửi tin tạo user message + assistant thinking + phản hồi canned + toolCalls giả | 4 giây thinking, tool `create_project`/`schema_audit` không tồn tại trong tool registry thật | 1.4, 3.6, 3.8 | `ChatView.tsx:79-171`, `ChatMessageItem.tsx:30-68` | không phải AI thực tế |
| CHAT-06 | `[MOCK]` | ProcessDetailDialog hiển thị markdown và một progress | `generateRequireMarkdown` chỉ từ `require.project` và `require.character` | 3.1 yêu cầu render từ `businessFlowData` | `ProcessDetailDialog.tsx:38-40`, `markdownGenerator.ts:38` | thiếu dữ liệu các phase sau |
| CHAT-07 | `[IMPLEMENTED]` | Split pane có thể kéo; dưới 1280px thành overlay | resize ratio, separator, keyboard | 3.2 | `ChatView.tsx:66-72`, `ChatView.tsx:240-272`, `useMediaQuery.ts:23` | không có backend |
| CHAT-08 | `[IMPLEMENTED]` | Feedback context hiển thị chip trong ChatInputBar; gửi xong clear | `FeedbackContext {id, tag, title}`; không có `content` | 3.4 | `ChatInputBar.tsx:54-71`, `DraftWorkspace.tsx:118-121`, `draft/types.ts:11-16` | xem C4 |
| CHAT-09 | `[GAP]` | Cần session/history, draft_session_id, plan state | Không có session store, không có route data | 3.6, 3.7 | toàn bộ `ChatView` dùng `useState` | refresh mất dữ liệu |
| CHAT-10 | `[MOCK]` | Model selector, context usage, attach/source button | Model list local; context usage hardcode 7%; attach/source không có hành vi | 3.8 | `ModelSelectorPopover.tsx:18`, `ContextUsagePopover.tsx:5`, `ChatInputBar.tsx:109-127` | không phải dữ liệu runtime |

### 3.2 Hành vi chi tiết

- `ChatHeader` hiển thị title “Merlink-harness” và badge “Preview”. Đây là dấu hiệu prototype, không phải trạng thái nghiệp vụ.
- Progress bar hiện tại là **một** giá trị phần trăm. Core 3.1 yêu cầu hai phần trăm riêng: “Đã xác nhận” (`source = user` hoặc `user_edited`) và “AI đề xuất” (`source = ai_generated`), mỗi phần chia cho tổng số thuộc tính. Đây là conflict C2.
- `PreDraftSuggestionDialog` hiện không có textarea/input; các i18n key `chat.preDraft.placeholder`, `chat.preDraft.skip`, `chat.preDraft.confirm` chưa được dùng đầy đủ. Confirm chỉ mở DraftWorkspace.
- Chat gửi tin không gọi backend, không chạy tool, không tạo version, không cập nhật `businessFlowData`. Các tool call trong message chỉ để minh họa popover.
- `ProcessDetailDialog` tính progress bằng cùng `calculateBusinessFlowProgress` và render markdown từ `generateRequireMarkdown`; không render toàn bộ cây `businessFlowData`.
- `ContextUsagePopover` hardcode 7% và các token ~2.3K/11.7K/133K; không phải context thật.
- Feedback chip chỉ giữ `id`, `tag`, `title`; không có `content` JSON, không gửi kèm message thật. `handleSend` chỉ đẩy text vào mảng local.
- Không có session list, session id, draft id, history persistence. Route `/chat/:id` không được dùng để phân biệt session.

## 4. Draft Workspace

### 4.1 Bảng hành vi

| ID | Trạng thái | Quy tắc UI | Dữ liệu/hành vi | Nguồn Core | Nguồn FE | Ghi chú |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| DR-01 | `[IMPLEMENTED]` | Stepper 6 bước: Overview, Chapter, World, Scene, Character, Relationship; bước 5 có 5.1-5.3 | step state local | 2.2-2.7, 3.2 | `DraftHeader.tsx:19-37`, `DraftWorkspace.tsx:21-25` | chỉ tiến, không quay lại |
| DR-02 | `[IMPLEMENTED]` | Next mở ConfirmDialog; confirm chạy fade-out + loading rồi sang bước kế | 400ms fade + 3s loading; không gọi backend | 3.3 | `DraftWorkspace.tsx:71-102`, `DraftWorkspace.tsx:239-259` | loading giả |
| DR-03 | `[MOCK]` | Mở Draft Workspace luôn có loading 3 giây | `transitionPhase = 'loading'` khởi tạo | 3.3 | `DraftWorkspace.tsx:44-69` | không phải nạp dữ liệu |
| DR-04 | `[MOCK]` | Dữ liệu mở dần theo bước; chapters bị ẩn ở Overview; World/Scene/Relationship mở theo bước | `phaseOverview` lọc client-side từ `DRAFT_OVERVIEW` | 3.2 | `DraftWorkspace.tsx:123-182` | dữ liệu mock |
| DR-05 | `[IMPLEMENTED]` | Tabs: Arcs, Characters; World từ bước 3; Scene từ bước 4; Relationship chỉ bước 6 | tab state auto-switch theo bước | 3.2 | `DraftOverviewBlock.tsx:62-76`, `DraftOverviewBlock.tsx:158-221` | tab cũ vẫn bấm lại được |
| DR-06 | `[MOCK]` | Version dropdown hiển thị v.03/v.02/v.01 | chọn chỉ đổi label `currentVersion`; không restore dữ liệu | 3.6 | `DraftToolbar.tsx:53-97`, `DraftWorkspace.tsx:227-229` | xem C6 |
| DR-07 | `[MOCK]` | Regenerate mở ConfirmDialog rồi chạy transition | `executeRegenerate` không đổi dữ liệu | 3.6 | `DraftWorkspace.tsx:108-116`, `DraftWorkspace.tsx:250-259` | chỉ là shell |
| DR-08 | `[IMPLEMENTED]` | Feedback Mode bật từ toolbar; click phần tử để gắn context; tắt khi rời panel | custom cursor; `{id, tag, title}`; không content | 3.4 | `DraftWorkspace.tsx:27-42`, `DraftWorkspace.tsx:118-121`, `DraftWorkspace.tsx:184-193` | xem C4 |
| DR-09 | `[MOCK]` | Start/“Xác nhận” chỉ xuất hiện ở bước 6; bấm mở Commit Confirmation Dialog preview (§7.2) | `handleStart = setCommitDialogOpen(true)` | 3.5, 3.7 | `DraftWorkspace.tsx:112` (`handleStart`), `DraftToolbar.tsx` | không commit; xem C7 |
| DR-10 | `[IMPLEMENTED]` | Character detail dialog 5.1/5.2/5.3; tab chưa có dữ liệu hiển thị khóa | lock dựa trên mock `appearance`, `psychological`, `competences`, `memories` | 2.6 | `CharacterDetailDialog.tsx:22-37`, `CharacterDetailDialog.tsx:92-130` | lock UI, không phải quyền backend |
| DR-11 | `[GAP]` | Cần Working State, phase/subphase, source, patch, version, lock, commit thật | không có các state trên | 1.3, 3.6, 3.7 | không có trong `src/features/draft` | xem GAP matrix |
| DR-12 | `[CONFLICT]` | Dữ liệu Draft dùng schema camelCase UI, không phải `BusinessFlowData` | `DRAFT_OVERVIEW` khác `SAMPLE_BUSINESS_FLOW_DATA` | 1.2, 2.2-2.7, 3.6 | `draft/types.ts:250`, `business-flow/types.ts:364` | xem C11 |

### 4.2 Thứ tự mở khóa dữ liệu

| Bước hiện tại | Arc/Chapter | World | Scene | Character | Relationship |
| :--- | :--- | :--- | :--- | :--- | :--- |
| Overview | Arc; ẩn chapters | Ẩn | Ẩn | Basic: name/nickname/gender/motif | Ẩn |
| Chapter | Arc + chapters + scenes | Ẩn | Ẩn | Basic | Ẩn |
| World | Arc + chapters | Tab World | Ẩn | Basic | Ẩn |
| Scene | Arc + chapters | Có | Tab Scene | Basic | Ẩn |
| Character | Có | Có | Có | Tab 5.1/5.2/5.3; dữ liệu mở dần | Ẩn |
| Relationship | Có | Có | Có | Đầy đủ theo mock | Tab Relationship |

- Dữ liệu được lọc trong `useMemo` `phaseOverview`, không phải dữ liệu thật bị khóa ở backend.
- `DraftHeader` stepper có `pointer-events-none`; không click để quay lại bước trước.
- `DraftStepNav` có nút Back/Next/Start nhưng không được dùng trong luồng hiện tại.
- Ở bước 6 toolbar dùng label `draft.toolbar.confirm` (“Xác nhận”), không phải label “Start” của `DraftStepNav`.

### 4.3 Dữ liệu và hành vi mock

- `DRAFT_OVERVIEW` có `currentPhase = 'chapter'` nhưng `DraftWorkspace` luôn bắt đầu ở `overview`; `currentPhase` không điều khiển UI.
- `versionHistory` là mảng mock; version không có `seq`, `phase`, `subphase`, `parent_seq`, `turn_id` hoặc hash.
- `segments` và `segmentsCount` trên Arc là dữ liệu UI-only, không có trong `BusinessFlowData`; không được dùng làm event block nghiệp vụ.
- `DraftCharacter` dùng `id`, `name`, `gender` và camelCase; `DraftChapter` không có `code`; `DraftCharacter.competences[].sceneLearned` là string (“Chương 3”) thay vì `scene_id`/`scene_code`.
- `DraftLoreEntry.data` là string; core 2.4 yêu cầu JSON object theo `LoreType`.
- `DraftWorldSettings` là một object; core 2.4 yêu cầu cây `world_settings` phân cấp.
- `DraftRelationship.states` là một object duy nhất; core 2.7 yêu cầu trạng thái thưa theo từng scene.
- Không có `source`, `pending_patches`, `draft_session_id`, `head_version_seq` trong bất kỳ mock draft nào.

## 5. Source và Pending Patches

### 5.1 Contract đích

| ID | Trạng thái | Quy tắc UI | Dữ liệu/hành vi | Nguồn Core | Nguồn FE | Ghi chú |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| SRC-01 | `[GAP]` | Hiển thị nguồn từng trường/phần tử: `user`, `user_edited`, `ai_generated` | Không có `source` trong mock | 1.3 | `business-flow/types.ts:364`, `draft/types.ts:250` | không có badge |
| SRC-02 | `[GAP]` | Hai progress: Đã xác nhận và AI đề xuất, chia theo `source` | Một progress duy nhất | 3.1 | `ChatHeader.tsx:50-61`, `progressEngine.ts:430-453` | xem C1/C2 |
| SRC-03 | `[GAP]` | Patch preview: trường, giá trị cũ/mới, phần tử bị ảnh hưởng | Không có UI patch | 1.3 | không có trong `src/features/draft` | |
| SRC-04 | `[GAP]` | Confirm/reject từng patch; nút Áp dụng tất cả; trạng thái expired; idempotent | Không có UI | 1.3, 3.6 | không có | |
| SRC-05 | `[GAP]` | UI khóa nút khi đang gửi confirm/reject; hiển thị kết quả `already_confirmed` | Không có | 3.6 | không có | |
| SRC-06 | `[TARGET]` | Hiển thị “patch hết hiệu lực” khi `expired` | Không có | 1.3 | không có | |

### 5.2 Hiện trạng

- Không có `pending_patches` trong `BusinessFlowData` mock; không có màn hình patch.
- Không có source badge, không có hai progress, không có preview diff, không có apply/reject.
- `source` chỉ xuất hiện trong type `CharacterMemory.sourceScene` và `CharacterCompetence` UI, không phải provenance của core.
- `calculateBusinessFlowProgress` chỉ đếm thuộc tính có giá trị, không phân biệt `source`. Vì vậy cả hai progress của core không thể tính từ dữ liệu hiện tại.
- Draft panel không hiển thị patch count hoặc banner “có patch đang chờ”.
- Scene content source (`AI_GENERATED` / `HUMAN_EDITED` / `RESTORE`) là mô hình riêng của Writing Mode, không phải `source` cấp draft; xem §9. Mock draft hiện không có source và không có `scene_versions`.

## 6. Workflow, Lock và Cancellation

### 6.1 Contract đích

| ID | Trạng thái | Quy tắc UI | Dữ liệu/hành vi | Nguồn Core | Nguồn FE | Ghi chú |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| WF-01 | `[GAP]` | Khi workflow chạy: hiển thị phase hiện tại, vòng hiện tại, task đã xong/tổng task | Không có workflow runner | 3.9 | không có | |
| WF-02 | `[GAP]` | Khóa thao tác ghi draft; cho phép đọc dữ liệu | Không có lock/busy state | 3.6, 3.9 | không có | |
| WF-03 | `[GAP]` | Hiển thị trạng thái “workflow đang chạy”, “đang hủy”, “seal pending”, “seal failed” | Không có | 3.9 | không có | |
| WF-04 | `[GAP]` | Nút yêu cầu hủy; hủy có kiểm soát, không rollback mutation đã applied | Không có | 3.9 | không có | |
| WF-05 | `[GAP]` | Resume available, completed, failed; hiển thị trạng thái vòng chưa seal | Không có | 3.9 | không có | |
| WF-06 | `[MOCK]` | Có tab Settings cấu hình subagent: maxSubagents, rejectedThreshold, outlineCapacity, outlineLookback, contextMargin, units, model | UI-only, không persist, không chạy | 3.9 | `SubagentOrchestrationTab.tsx:1`, `subagentConfig.ts:1` | không phải workflow UI |

### 6.2 Ghi chú

- `DRAFT_LOCK_TIMEOUT` là thời gian chờ lấy khóa, không phải thời gian thực thi workflow. UI không được hiển thị “quá 30 giây là workflow bị dừng”.
- Không có draft lock, busy state, read-only snapshot, cancel request, seal state hoặc resume state trong FE.
- Tab Settings subagent chỉ là mock; comment tại `src/features/settings/subagentConfig.ts:1` ghi rõ “UI-only intent. Nothing in this module is sent to the backend or persisted.”

## 7. Start/Commit

### 7.1 Hợp đồng đích (H2 APPROVED)

| ID | Trạng thái | Quy tắc UI | Dữ liệu/hành vi | Nguồn Core | Nguồn FE | Ghi chú |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| ST-01 | `[MOCK]` | Action “Xác nhận” ở bước 6 mở Commit Confirmation Dialog; dialog đồng thời là plan review | `handleStart` mở dialog preview (Wave 0.3A) | 3.5, 3.7 | `DraftWorkspace.tsx` `handleStart`, `CommitConfirmationDialog.tsx` | H2 approved; commit thật chưa có |
| ST-02 | `[MOCK]` | Dialog tóm tắt plan, validation và pending patches; confirm disable khi còn điều kiện chặn | Dialog + read-model presentation; readiness thật chưa có | 3.5, 3.7 | `commitPreview.ts` | |
| ST-03 | `[MOCK]` | Chặn submit trùng; pending không thể đóng; backend idempotent; request gián đoạn không tự coi là thành công | Chỉ mô phỏng pending; idempotency backend chưa có | 3.7 | `commitPreview.ts` | |
| ST-04 | `[MOCK]` | Failure giữ Draft Workspace và draft `editing`; hiển thị lỗi; cho retry an toàn | Mô phỏng lỗi giữ draft, retry | 3.7 | `CommitConfirmationDialog.tsx` | |
| ST-05 | `[GAP]` | Success đóng Draft Workspace, điều hướng `/project/:projectId`, mặc định View | Không có navigation/project mới | 3.7, fe-core §9 | không có | H3: chọn chương ở Creator để viết |
| ST-06 | `[GAP]` | `phase_confirm` khác `commit`; không thêm màn/phase review riêng | Không có `phase_confirm` thật | 3.5, 3.6, 3.7 | `draft/toolbar` chỉ là mock | |

### 7.2 Hiện trạng (Wave 0.3A — UI preview)

**Đã triển khai ở mức UI preview `[MOCK]`:**

- Nút “Xác nhận” ở bước 6 mở `CommitConfirmationDialog` (`src/features/draft/CommitConfirmationDialog.tsx`); không còn đóng Draft Workspace. Dialog là additive: không đổi stepper, tab, toolbar, route hay CSS toàn cục. State dialog là local (`useState`), không thêm Zustand store.
- Logic thuần ở `src/features/draft/commitPreview.ts`, test tại `commitPreview.test.mjs` (C01–C12, automated, chỉ logic thuần). Tương tác DOM (Escape, click ngoài, nút X, responsive, light/dark) chỉ kiểm tra thủ công, không có DOM test.
- **Plan summary** suy ra từ `overview` hiện tại: tên, mô tả (nếu có), số arc, chương, cảnh (từ `sceneConfig`, fallback `arcs[].chapters`), nhân vật, quan hệ, worldbuilding có/không. Trường thiếu hiển thị “Không khả dụng”/“Chưa đánh giá”, không bịa số.
- **Validation:** `ready` chỉ khi dữ liệu validation xác nhận rõ; `blocked` khi có lý do chặn hoặc `ready === false`; thiếu dữ liệu → `unknown`. `unknown` không bao giờ được coi là `ready`; việc tới bước 6 không suy ra readiness.
- **Pending patches:** `known{count}` chỉ khi có dữ liệu hợp lệ; thiếu → `unknown`. `count>0` chặn commit; `unknown` cũng chặn commit.
- **Eligibility:** Confirm chỉ bật khi `validation=ready` AND `patches=known` AND `count=0` AND không có thao tác pending. Hiện không có nguồn dữ liệu readiness thật nên **Confirm luôn bị vô hiệu ở production** với lý do hiển thị.
- **Mô phỏng:** máy trạng thái `idle → pending → error | success-preview`. Pending chặn submit trùng, Escape, click ngoài, nút X và Cancel; hiển thị loading. Error giữ nguyên draft và cho retry. Success chỉ là “Kết quả xem trước (mô phỏng)”: không tạo project, không ghi dữ liệu, không điều hướng, không đóng Draft Workspace. Điều khiển “Dev simulation” (fixture readiness + outcome) chỉ render khi `import.meta.env.DEV`; fixture đi qua cùng logic eligibility, không bypass.
- Toàn bộ chuỗi i18n dùng khóa `commit.*` (vi/en).

**Wave 0.3B — thang kích thước và điều hướng preview `[MOCK]`:**

- Typography của `CommitConfirmationDialog` tăng đúng +2px, chỉ trong component này (`text-xs→text-sm`, `text-sm→text-base`, `text-base→text-lg`; badge/button/label Select được override cục bộ). Không đổi token toàn cục hay `Dialog` chung. Chiều rộng tăng từ `max-w-lg` (512px) lên `max-w-[44rem]` (704px); vẫn header cố định, body cuộn, footer ghim.
- `src/features/draft/commitNavigation.ts` định nghĩa `CommitResult` (`committed{projectId}` | `failed` | `unknown`) và `resolveSimulatedNavigation`. Chỉ kết quả **mô phỏng**, không pending, `committed` và `projectId` khớp một mock project có sẵn mới được điều hướng; không fallback sang project đầu tiên, không tạo/sinh ID.
- Mô phỏng thành công không tự điều hướng. Dialog hiển thị dự án mẫu đích (chọn trong panel Dev simulation; có lựa chọn “không tồn tại” để thử lỗi), cảnh báo đích chứa dữ liệu mẫu chứ không phải draft vừa duyệt và bản nháp có thể bị mất. Người dùng bấm “Xem Project Detail mẫu” → hộp xác nhận (`ConfirmDialog`) → đóng Draft Workspace và `navigate('/project/:projectId')` (không có `mode` ⇒ View Mode). Bấm lặp chỉ chạy một lần (once-guard).
- Failed / unknown / pending / thiếu target: giữ Draft Workspace và dữ liệu, không điều hướng. Chuyển sang Creator Mode và chọn chương là thao tác thủ công của người dùng trên route hiện có `/project/:projectId/chapter/:chapterId?mode=creator`; không có route mới, không tự sinh chương/cảnh.
- Draft chỉ là state local, không persist: sau điều hướng hoặc refresh không khôi phục. Đây **không phải commit thật**; không tạo project.
- `ProjectCreatorPanel` điều hướng chọn chương qua `buildCreatorChapterPath` của `commitNavigation.ts` (test N11/N12), nên component và test dùng chung một hợp đồng route; URL không đổi.

**Cleanup tiền-merge Stage B `[MOCK]`:**

- Khi pending, `CommitConfirmationDialog` ẩn nút X dùng chung (class cục bộ `[&>button:last-child]:hidden`, không đổi `Dialog` chung), chặn Escape/click ngoài, disable Cancel và `handleOpenChange` từ chối đóng (`canDismissCommitDialog`). Test N13 chỉ kiểm tra wiring ở mức source; tương tác DOM vẫn kiểm tra thủ công.
- Điều khiển “Dev simulation” chỉ render khi `import.meta.env.DEV`; production build không có panel này và kết quả luôn là `unknown` (không success, không điều hướng).
- Phân biệt: dialog, máy trạng thái, validation/patch presentation và điều hướng mẫu là **UI implementation `[MOCK]`**; validation thật, pending patches thật, commit bền vững, idempotency, tạo project và điều hướng tới project thật là **phụ thuộc backend `[GAP]`**.

**Vẫn `[GAP]` / `[IMPLEMENTATION PENDING]`:**

- Backend validation và nguồn `pending_patches` thật.
- Durable commit, idempotency, transaction rollback, tạo project.
- Điều hướng sau commit thật tới project vừa tạo (ST-05): chỉ có preview tới project mẫu (Wave 0.3B); project thật vẫn pending.
- Crash recovery, `phase_confirm` vs `commit` thật, read-only sau commit.

## 8. Persistence, Version và Crash Recovery

### 8.1 Contract đích

| ID | Trạng thái | Quy tắc UI | Dữ liệu/hành vi | Nguồn Core | Nguồn FE | Ghi chú |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| VER-01 | `[MOCK]` | Version dropdown hiển thị danh sách version | Dữ liệu mock `versionHistory` | 3.6 | `DraftToolbar.tsx:53-97` | không có seq |
| VER-02 | `[GAP]` | Chọn version phải restore dữ liệu, phase, subphase; append-only | Chọn chỉ đổi label | 3.6 | `DraftWorkspace.tsx:227-229` | xem C6 |
| VER-03 | `[GAP]` | Hiển thị head version, version history, lý do version | Không có | 3.6 | không có | |
| VER-04 | `[GAP]` | Restore làm patch pending thành expired; hiển thị patch hết hiệu lực | Không có | 3.6 | không có | |
| VER-05 | `[GAP]` | Seal pending, seal failed (`VersionSealFailed`), interrupted turn, crash recovery | Không có | 3.6 | không có | |
| VER-06 | `[GAP]` | Workflow resume từ sealed version cuối; task đã seal không chạy lại | Không có | 3.9 | không có | Q3 còn mở |

### 8.2 Hiện trạng

- `localStorage` chỉ lưu theme và locale; không lưu `BusinessFlowData`, draft, message hoặc version.
- Version dropdown dùng `DRAFT_OVERVIEW.versionHistory` mock; chọn version không thay đổi `overview` data.
- Không có head version, sealed version, version hash, `turn_id`, `chat_message_id`, `reason` hoặc `parent_seq` trong FE.
- Refresh làm mất toàn bộ chat/draft state.
- FE không có Working State hay tool result `applied`; vì vậy không thể hiển thị hoặc kiểm chứng khác biệt giữa `applied` và sealed version. Theo core 3.6, `applied` không phải durable và `VersionSealFailed` không được báo thành công giả.
- Core 3.6 có `[SCHEMA FOLLOW-UP]` cho idempotent sealing; FE hiện không có schema hoặc persistence để kiểm chứng.
- Scene version dialog ở §9 dùng fixture local; không phải `scene_versions`, không ghi DB và không liên quan `draft_versions`.
- Save nội dung cảnh chưa có backend; FE chỉ hiển thị cảnh báo và giữ working state trong bộ nhớ.

## 9. Writing Mode (Wave 0 preview)

### 9.1 Trạng thái hợp đồng

H1 đã `APPROVED` tại core 1.6. Wave 0/0.1 triển khai UI preview trên Chapter Detail; backend, persistence và committed-plan gate chưa có.

| ID | Trạng thái | Quy tắc UI | Dữ liệu/hành vi | Nguồn Core | Nguồn FE | Ghi chú |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| WR-01 | `[GAP]` | Writing Mode chỉ mở khi project có committed plan hợp lệ | Preview mở từ `MOCK_PROJECTS`; không có read-model/gate | 1.6 | `ProjectDetailPage.tsx`, `ChapterDetailPage.tsx` | phải chặn khi chưa có plan |
| WR-02 | `[IMPLEMENTED]` | Project Detail có switch View/Creator; Creator chỉ chọn chương, không nhúng scene editor | `?mode=view\|creator`, mock chapters | 1.6 | `ProjectDetailView.tsx:23-34`, `ProjectCreatorPanel.tsx` | UI only |
| WR-03 | `[IMPLEMENTED]` | Chapter Detail đọc/sửa từng scene; một block = một scene; label cảnh trong edit mode | `SceneBlock[]` local; `buildSceneBlocks` | 1.6 | `ChapterDetailView.tsx:120-121`, `scenePreview.ts:30-36` | dữ liệu mock |
| WR-04 | `[IMPLEMENTED]` | Generation controls theo scene; buffer; tối đa một generation/chương; review/accept/discard | Preview buffer local, không gọi AI | 1.6 | `ChapterDetailView.tsx:263-330` | preview, không phải AI thật |
| WR-05 | `[IMPLEMENTED]` | Version dialog theo scene; source/timestamp/content; restore local | Fixture `buildPreviewVersions` | 1.6 | `ChapterDetailView.tsx:730-800`, `scenePreview.ts:42-62` | dữ liệu mẫu, không persist |
| WR-06 | `[IMPLEMENTED]` | Dirty theo scene; lock change tính dirty; Save cảnh báo; guard điều hướng; Chat giữ buffer | Local state + `useBlocker` + `beforeunload` | 1.6 | `ChapterDetailView.tsx:150-177`, `:209-230` | không có persistence |
| WR-07 | `[IMPLEMENTED]` | Khóa scene chặn sửa/generate/versions; Lock All/Unlock All | `block.locked` local | 1.6 | `ChapterDetailView.tsx:520-560`, `:650-690` | chưa gắn `scenes.is_locked` |
| WR-08 | `[GAP]` | Save cấp chương persist content + lock + word_count + `scene_versions` | Backend chưa có | 1.6 | không có | `[IMPLEMENTATION PENDING]` |
| WR-09 | `[GAP]` | Continuity theo scene, advisory | Không có API | 1.6 | không có | |
| WR-10 | `[GAP]` | Generation stream/cancel thật | Không có tool sinh nội dung scene | 1.6 | không có | `[IMPLEMENTATION PENDING]` |
| WR-11 | `[IMPLEMENTED]` | Không tự generate scene/chương kế, không bulk | UI không có hành vi tự động | 1.6 | `ChapterDetailView.tsx` | chỉ là UI preview |
| WR-12 | `[IMPLEMENTED]` | Regenerate mở dialog feedback tùy chọn; note rỗng/khoảng trắng thành `null`; note chỉ thuộc scene đã chọn | Request local `{sceneId, note}`; note hiện trong review; AI không đọc | 1.6 | `ChapterDetailView.tsx:274-340`, `scenePreview.ts:17-28` | UI preview; AI consumption `[IMPLEMENTATION PENDING]` |

### 9.2 Mô hình chương-first, cảnh là block

- Một `ChapterBlock` đại diện đúng một scene; chapter là dãy scene, không phải một textarea gộp và không có scene page riêng.
- `ChapterBlock` có `sceneId` optional; `scenePreview.ts` tạo ID preview `scn-{chapter.id}-{block.id}`. Đây **không phải** `scenes.id` và không được persist.
- Array index chỉ dùng để hiển thị “Cảnh N”; identity dùng `sceneId`. Backend read-model phải trả `scenes.id` thật.
- Không có `chapter_versions`; version nội dung là `scene_versions` theo scene, sau khi backend Save thành công.
- Không có chapter editor page mới, scene route mới, giao diện sửa scene thứ hai hoặc switch View/Creator trong Chapter Detail.
- Source nội dung cảnh (`AI_GENERATED`, `HUMAN_EDITED`, `RESTORE`) tách biệt source cấp draft; UI preview track source trong local state, không ghi DB.

### 9.3 Hành vi Wave 0

**Project Detail**

- View mode giữ nguyên overview card, Arc registry, chapter registry, chat pane và layout hiện có.
- `Segmented` View/Creator nằm vùng phải header, trước nút Chat; đây là control duy nhất được thêm vào header.
- `?mode=view|creator` được đọc từ URL; thiếu hoặc giá trị lạ → View. Chuyển mode dùng `setSearchParams`; back/forward giữ mode.
- Creator chỉ render danh sách chương theo Arc, badge số cảnh; không render scene editor.
- Click chương điều hướng `/project/:projectId/chapter/:chapterId?mode=creator`.
- `?mode=creator` trên Chapter Detail chỉ hiện badge “Creator/Sáng tác”; **không tự bật edit**.
- Không có switch View/Creator trong Chapter Detail; Chapter Detail chỉ có Edit/Cancel/Save hiện hữu.

**Chapter Detail**

- View mode giữ nguyên cột đọc `max-w-2xl`, title/date, breadcrumb, Chat, FloatingOutline.
- Edit mode giữ card scene và `AutoResizeTextarea`; thêm label `Cảnh N`, badge source/dirty, hàng control.
- Generate chỉ bật khi scene rỗng và không khóa; Regenerate khi có nội dung mở dialog feedback; Cancel khi đang chạy; Versions khi có nội dung và không khóa.
- Buffer generation hiện trong card, có badge “Bản xem trước” và ghi chú không phải AI thật, không lưu. Accept đưa text vào working state và dirty; Discard bỏ buffer.
- Version dialog hiển thị fixture có nhãn “Dữ liệu mẫu”; Restore cập nhật working state và dirty, không persist.
- Save vẫn là nút hiện có; khi dirty hiện toast “Chưa thể lưu trong bản xem trước”, giữ buffer và dirty; khi sạch hiện “Không có thay đổi chưa lưu.”
- Lock All / Unlock All và lock từng scene giữ nguyên visual; lock chặn edit/generate/versions.
- Dirty guard dùng `useBlocker` cho điều hướng nội bộ và `beforeunload` cho refresh/close; ConfirmDialog `Rời chương này?`.
- Mở/đóng Chat không reset buffer; `AutoResizeTextarea` tính lại chiều cao khi pane đổi width để tránh cắt nội dung.

**Regenerate dialog (Wave 0.1)**

- Chỉ Regenerate mở dialog; Generate cho scene rỗng giữ nguyên hành vi cũ.
- Dialog dùng `Dialog`/`Textarea`/`Button` hiện có, title “Tạo lại cảnh”, description và placeholder theo i18n; textarea tùy chọn, tối đa 2.000 ký tự.
- Hủy, nút X và Escape đóng dialog, không sinh generation, không đánh dirty.
- Note rỗng hoặc chỉ khoảng trắng được chuẩn hóa thành `null`; note khác rỗng được trim; không yêu cầu độ dài tối thiểu.
- Confirm tạo request local `SceneRegenerationRequest {sceneId, note}` và bắt đầu preview cho đúng scene đã chọn; chặn submit trùng; scene khóa hoặc đang có generation không mở/không confirm.
- Note không được append vào scene text hay sample text, không gửi sang scene khác, không persist; review hiển thị “Ghi chú đã gửi (chỉ xem trước)”.
- Nội dung scene chỉ đổi sau Accept; Discard/Cancel giữ nguyên nội dung.
- AI không đọc note trong preview; tiêu thụ note thật là `[IMPLEMENTATION PENDING]`.

**Interaction hardening (Wave 0.2)**

- Dirty được tính lại từ `workingContent !== baselineContent || workingLocked !== baselineLocked`; source metadata không tạo dirty. Sửa rồi khôi phục đúng baseline, hoặc lock/unlock về baseline, thì scene sạch.
- Source được quyết định bởi thao tác cuối: Accept → `AI_GENERATED`; sửa tay → `HUMAN_EDITED`; Restore → `RESTORE`; Restore rồi sửa tay → `HUMAN_EDITED`. Mở/đóng dialog, cancel, discard và lock không đổi source.
- Generation có `requestId`; cancel/discard/unmount vô hiệu hóa callback cũ. Double Confirm bị chặn; một chapter chỉ có một generation active.
- Trong lúc generating/review, lock của scene đang active, Lock All và Unlock All đều bị disable (handler cũng chặn qua `canBulkChangeLocks`/`applyBulkLock` trong `scenePreview.ts`); lock scene khác vẫn an toàn. Accept bị chặn nếu scene đã khóa.
- Nội dung scene dài được giới hạn chiều cao editor ở `70vh` với scroll trong editor; review sample giới hạn `50vh`. Controls và Accept/Discard vẫn tiếp cận được, không overlap, không horizontal overflow.
- Dưới breakpoint 1279 hiện có, Chat của Chapter Detail chuyển thành overlay toàn pane (dùng `useIsCompact` hiện có), không còn split-pane hẹp.
- Header dùng container width: ẩn label Lock All/Unlock All khi header < 510px; xếp breadcrumb + controls thành hai hàng khi < 420px. Breadcrumb được truncate, không bị ẩn toàn bộ. Không thêm breakpoint toàn cục.

**UI preservation của Wave 0:** không đổi sidebar/shell, navigation hierarchy, breadcrumb, title/date, cột `max-w-2xl`, card scene, `AutoResizeTextarea`, FloatingOutline, Chat split-pane/tỷ lệ, design tokens, typography hoặc breakpoint hiện có. Các phần thêm nằm trong header Project Detail, header Chapter Detail (responsive), card scene và dialog; xem §12.1.

### 9.4 Preview và production

**Đã có trong Wave 0/0.1 (UI/local):**

- Project View/Creator switch và chapter selection.
- Chapter Detail scene blocks, scene labels, edit/save local.
- Scene generation controls, preview buffer, review/accept/discard.
- Regenerate dialog với feedback tùy chọn; note được giữ trong request local và hiện ở review.
- Sample version history và local restore.
- Dirty tracking, lock tracking, dirty navigation guard, Chat preservation.

**Chưa có (không được mô tả là production):**

- AI generation thật, stream/cancel.
- AI đọc/hiểu note regeneration; preview chỉ lưu note để kiểm tra.
- Durable Save và `scene_versions` thật.
- Backend lock/unlock và `scenes.is_locked`.
- Committed-plan gate.
- H2 commit dialog/transaction/navigation.
- Continuity API.
- Crash recovery/autosave.

Mọi dữ liệu generation/version trong preview là fixture hoặc local state; không phải bản ghi DB, không tạo `scene_versions`, không được dùng làm bằng chứng backend đã hoạt động. Note regeneration cũng chỉ là local state; sample generation không phản ánh nội dung note.

### 9.5 Hợp đồng backend cần

- Read-model scene: `scenes.id`, `chapter_id`, `position`, `content`, `word_count`, `is_locked`, quyền truy cập theo project đã commit.
- Generation một scene: stream/cancel, không ghi DB; nhận nội dung scene hiện tại + note tùy chọn; buffer contract.
- Save cấp chương: atomic content + lock + word_count + `scene_versions`; trả baseline/version head.
- Version list/restore: append-only; restore tạo version `RESTORE` sau Save.
- Lock/unlock persist.
- Continuity advisory.
- Committed-plan gate cho Creator/Writing.
- Commit Confirmation Dialog/transaction/navigation (H2).

Chi tiết dữ liệu và gap ở §13–§15; chi tiết schema follow-up ở core Q4.

### 9.6 Responsive header và compact Chat (Wave 0.2)

**Tái hiện trước khi sửa:** ở 1280px, edit mode, Chat mở: header `clientWidth` 453, controls 464, `scrollWidth` 464 (overflow 11px), breadcrumb width 0.

**Nguyên nhân:** năm control trong header có tổng bề rộng cố định 464px; breadcrumb là flex item bị co về 0; Chapter Detail chưa dùng nhánh compact 1279px nên Chat vẫn chia pane ở các width nhỏ.

**Đã sửa trong Chapter Detail:**

- Dùng `useIsCompact` hiện có: từ 1279px trở xuống, Chat chuyển thành overlay toàn pane; separator và split ratio không còn áp dụng.
- Header đo container width: ẩn label Lock All/Unlock All khi header < 510px (icon vẫn có `title` và `aria-label`); xếp breadcrumb + controls thành hai hàng khi < 420px.
- Breadcrumb có `min-w-0`, `flex-1` và truncate; controls `shrink-0`; không có overflow ngang.

**Đo sau khi sửa (edit mode):**

| Viewport | Chat | Header width | Overflow | Breadcrumb | Ghi chú |
| ---: | :--- | ---: | :--- | ---: | :--- |
| 1440 | mở | 541 | Không | 76px | label hiện |
| 1366 | mở | 500 | Không | 188px | label ẩn |
| 1280 | mở | 453 | Không | 141px | label ẩn; trước fix overflow 11px |
| 1279 | mở | 894 | Không | 430px | Chat overlay |
| 1024 | mở | 639 | Không | 175px | Chat overlay |
| 768 | đóng/mở | 383 | Không | 383px | header hai hàng |

Các width 1440/1366/1280 khi Chat đóng, và 1024/768 khi Chat đóng, cũng không overflow. Không đổi global breakpoint, sidebar, design token hoặc layout ngoài Chapter Detail.

### 9.7 Test coverage (Wave 0.2)

- Automated: `npm test` chạy glob `src/features/**/*.test.mjs` (Vite SSR + `node:test`). Trước cleanup Stage B: 34 test; sau cleanup: 39 test (`settings` 4, `commitPreview` C01–C12+, `commitNavigation` N01–N13, `scenePreview` 6 + 3 test F03 cho Unlock All khi generating/review). Đều là test logic thuần/source-wiring, không phải DOM test.
- Các SSR harness test đặt `server.ws: false`: Vite 8 vẫn tạo WebSocket server ở middleware mode dù `hmr: false`, khiến các file chạy song song tranh cổng 24678 (cảnh báo “Port 24678 is already in use”). Test không cần WebSocket nên tắt hẳn; không đổi isolation.
- Manual browser checks: dirty Regenerate (discard/cancel/accept), multi-scene dirty, Save preview, navigation guard, lock/review, long scene 100/2.000/8.000 từ, Chat toggle, VI/EN, light/dark và 6 viewport.
- Giới hạn: repo không có DOM/component test harness (không có jsdom/testing-library); các tương tác component T01-T04, T11-T18 được kiểm tra thủ công, không được tuyên bố là automated coverage.

## 10. Character Library, Memory và Competence

### 10.1 Bảng hành vi

| ID | Trạng thái | Quy tắc UI | Dữ liệu/hành vi | Nguồn Core | Nguồn FE | Ghi chú |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| CHR-01 | `[MOCK]` | Character library list, search, filter project/sex, create | `useCharacterStore` + mock list; create local | 2.6 | `CharacterListView.tsx:13-25`, `CreateCharacterDialog.tsx` | không persist |
| CHR-02 | `[IMPLEMENTED]` | Character detail tab core/life/psych; edit/save/cancel local | Zustand edit snapshot | 2.6 | `CharacterDetailLayout.tsx:92-125` | dữ liệu mock |
| CHR-03 | `[MOCK]` | Competence/memory hiển thị, thêm/xóa local; memory có `sourceScene` string | `LifeExperienceTab`/`MemoryCard` | 2.6 | `LifeExperienceTab.tsx:1-110`, `MemoryCard.tsx` | không có `scene_id` |
| CHR-04 | `[GAP]` | Tái sử dụng nhân vật thư viện qua `project_characters` | Không có UI liên kết; library chỉ filter theo `projectId` mock | 3.8 | `CharacterListView.tsx:17-33` | xem GAP-10 |
| CHR-05 | `[GAP]` | Năng lực/ký ức phẳng: `scene_id NOT NULL`, `source_scene_id` nullable, cho phép trùng | FE dùng `sceneLearned`/`sourceScene` string; không có source, không validate | 2.6, 2.8 | `draft/types.ts:100-116`, `characters/types.ts` | xem C11 |
| CHR-06 | `[GAP]` | Continuity check và phân biệt dữ liệu trùng với mâu thuẫn | Không có | 1.5, 2.6 | không có | |
| CHR-07 | `[GAP]` | Project-character link có vai trò/notes sau commit | Không có UI | 3.8 | không có | |

### 10.2 Ghi chú

- Character library là global mock; project filter chỉ dựa trên `projectId`/`projectName` có sẵn trong mock, không phải `project_characters`.
- Character detail sửa trực tiếp mock profile; không có `source`, không có patch, không có version.
- `CharacterCompetence` UI có `sceneLearned` string, `CharacterMemory` UI có `sourceScene` string; core yêu cầu tham chiếu scene bằng `scene_id`/`scene_code` và `source_scene_id` nullable.
- Không có merge/normalize dữ liệu trùng; UI chỉ render danh sách. Đây là hành vi phù hợp với việc cho phép dữ liệu trùng, nhưng FE chưa có continuity check hoặc source.

## 11. Code Collision và Cascade Delete ở góc nhìn UI

### 11.1 Code

| ID | Trạng thái | Quy tắc UI | Dữ liệu/hành vi | Nguồn Core | Nguồn FE | Ghi chú |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| CODE-01 | `[CONFLICT]` | Người dùng không thấy `code` trên UI; chỉ thấy display name/vị trí | FE hiển thị slug/code ở một số block | 3.8 | `ChapterBlock.tsx:116`, `WorldSettingsBlock.tsx:89`, `SceneConfigBlock.tsx:144` | xem C5 |
| CODE-02 | `[GAP]` | Handler cấp code cuối; UI không quyết định code | Không có UI code; chỉ hiển thị | 3.8 | không có | |
| CODE-03 | `[GAP]` | Collision khi tạo/commit hiển thị lỗi có cấu trúc và giữ draft | Không có create/commit thật | 3.7, 3.8 | không có | |
| CODE-04 | `[GAP]` | Tái sử dụng nhân vật thư viện; không tạo bản sao | Không có UI liên kết | 3.8 | `CharacterListView` | xem CHR-04 |

### 11.2 Cascade Delete

| ID | Trạng thái | Quy tắc UI | Dữ liệu/hành vi | Nguồn Core | Nguồn FE | Ghi chú |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| CAS-01 | `[GAP]` | Preview đầy đủ dữ liệu bị ảnh hưởng trước khi xóa cha | Không có draft delete UI | 1.3 | không có | |
| CAS-02 | `[GAP]` | Nếu có `user`/`user_edited`, trả `pending_confirmation` và cho confirm/reject | Không có patch UI | 1.3 | không có | |
| CAS-03 | `[GAP]` | Cascade nguyên tử; một version `ai_patch` cho toàn bộ | Không có version | 1.3, 3.6 | không có | |
| CAS-04 | `[MOCK]` | DeleteConfirmDialog cho project/character là local, không phải cascade draft | `ProjectView`/`CharacterList` xóa local | 1.3 | `ProjectView.tsx`, `DeleteConfirmDialog` | không liên quan draft cascade |

## 12. Cross-cutting Concerns

| ID | Trạng thái | Quy tắc UI | Dữ liệu/hành vi | Nguồn Core | Nguồn FE | Ghi chú |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| XC-01 | `[IMPLEMENTED]` | UI chrome dùng i18n `vi`/`en` | `dict.ts` + `useT` | 3.8 | `src/shared/i18n/dict.ts`, `useT.ts` | mock data không dịch |
| XC-02 | `[MOCK]` | `system_language` cho giao tiếp; `project.language` cho nội dung | Chưa có pipeline tách hai ngôn ngữ; mock data có `language` | 3.8 | `ChatView`/mock data | chỉ là dữ liệu |
| XC-03 | `[IMPLEMENTED]` | Responsive: split pane dưới 1280px thành overlay | `COMPACT_QUERY = '(max-width: 1279px)'` | 3.2 | `useMediaQuery.ts:23` | breakpoint thực tế |
| XC-04 | `[IMPLEMENTED]` | Một số primitive a11y: progressbar, separator, tab, dialog title | `aria-*`, `role`, `focus-ring` | không thuộc core | `ChatHeader`, `ChatView`, `DraftOverviewBlock` | không audit toàn bộ |
| XC-05 | `[GAP]` | Không hiển thị `code`; dùng display label | FE hiển thị code/slug | 3.8 | 3 block ở mục 11 | xem C5 |
| XC-06 | `[GAP]` | Loading/empty/error state cho patch, version, commit, workflow | Hầu như không có | 1.3, 3.6, 3.7, 3.9 | chỉ có mock loading Draft | |
| XC-07 | `[GAP]` | Phân biệt technical identifier và display label | FE trộn `id`, `code`, `displayName` | 3.8 | `draft/types.ts` | |

### 12.1 UI Preservation Invariant (Bất biến bảo toàn giao diện)

Quy tắc này áp dụng cho mọi task có thay đổi FE, kể cả task mà mục tiêu chính là backend, Rust, IPC, API, persistence hoặc tích hợp nghiệp vụ. Đây là điều kiện nghiệm thu bắt buộc, không phải khuyến nghị. Nguyên tắc tối cao: **Functional correctness does not override visual preservation.**

1. **Preserve by default:** Layout, navigation, sidebar, split pane, tabs, stepper, toolbar, component placement, spacing, typography, styling và responsive breakpoints hiện tại phải được bảo toàn mặc định.
2. **Backend integration không đồng nghĩa UI redesign:** Việc triển khai Rust Backend, IPC, API, persistence hoặc bất kỳ nghiệp vụ mới nào không đồng nghĩa được phép thiết kế lại giao diện.
3. **Additive changes only:** Chức năng mới phải ưu tiên tích hợp bằng thay đổi bổ sung tối thiểu, chẳng hạn thêm button, dialog, tooltip, badge hoặc trạng thái vào component hiện có. Không tự thay thế cấu trúc giao diện.
4. **Explicit approval:** Mọi thay đổi cấu trúc, bố cục hoặc thiết kế ngoài phạm vi người dùng phê duyệt đều phải được xin phép trước.
5. **No implicit authorization:** Các lý do như tối ưu UX, refactor, clean code, dễ tích hợp backend hoặc cải thiện responsive không được xem là sự cho phép thay đổi layout.
6. **Strict file scope:** Agent chỉ được chỉnh sửa các file nằm trong allowlist của task. Nếu cần sửa file khác, phải dừng và yêu cầu phê duyệt.
7. **Subagent inheritance:** Mọi subagent phải tuân thủ cùng giới hạn. Agent chính chịu trách nhiệm kiểm tra diff của subagent.
8. **Visual regression:** Với task có thay đổi FE, phải kiểm tra diff và so sánh giao diện trước/sau tại các viewport liên quan khi có công cụ hỗ trợ. Không được tuyên bố đã kiểm tra trực quan nếu chưa thực hiện.
9. **Failure condition:** Một implementation đúng chức năng nhưng tự ý làm thay đổi giao diện ngoài phạm vi được phê duyệt phải được đánh giá là **FAILED**.
10. **Exception:** Chỉ được thay đổi layout khi người dùng yêu cầu hoặc phê duyệt rõ ràng. Quyền sửa một file component không đồng nghĩa quyền thay đổi toàn bộ layout của component đó.

**Ghi chú triển khai FE:**

- Checklist nghiệm thu của mọi task FE phải có mục kiểm tra invariant này; thiếu mục đó thì task chưa đủ điều kiện hoàn thành.
- Kiểm chứng tối thiểu: diff file, đối chiếu component bị sửa với mục 12.1, và so sánh trước/sau tại breakpoint 1280px (XC-03) cùng viewport mà task tuyên bố hỗ trợ.
- Nếu không có công cụ so sánh trực quan, phải ghi rõ "chưa kiểm tra trực quan"; không được mô tả là đã kiểm tra.

## 13. Hợp đồng dữ liệu FE cần từ backend

Đây là danh sách dữ liệu/hành vi FE cần để thực hiện Core v2.4. Không đặt tên IPC command/event mới; tên cụ thể do backend và FE chốt sau.

| Nhu cầu | Dùng cho | Hiện trạng FE |
| :--- | :--- | :--- |
| `draft_session_id`, trạng thái draft (`editing`/`committing`/`committed`) | mở draft, khóa ghi, Start | không có |
| `phase`, `subphase` hiện tại | stepper, restore, read-only | mock `currentPhase` string, không dùng |
| `head_version_seq`, danh sách version (seq, reason, phase, subphase, created_at) | version dropdown, restore | mock `versionHistory` string |
| snapshot `businessFlowData` kèm `source` từng trường | render dữ liệu, hai progress | mock camelCase, không source |
| `pending_patches[]` (target_code, field, operation, previous/new value, status) | patch UI, Start block | không có |
| trạng thái khóa/workflow (running, round, task, cancel, seal) | workflow UI | không có |
| lỗi có cấu trúc (`rejected`, `pending_confirmation`, `VersionSealFailed`, collision) | toast/dialog/badge | chỉ có mock toast |
| `project_characters` / character library link | tái sử dụng nhân vật | mock filter, không link |
| committed-plan gate và `project_id` sau commit | Creator/Writing entry, H2 navigation | không có; mock project luôn mở |
| scene read-model: `id`, `chapter_id`, `position`, `content`, `word_count`, `is_locked` | Creator chapter/scene list, Writing surface | block local; `sceneId` preview `scn-*` |
| generation một scene: stream, cancel, buffer, lỗi | Generate/Regenerate/Cancel/Retry | chưa có; preview dùng sample text |
| regeneration request note: `{sceneId, note}` với `note` nullable | Regenerate feedback và review | local state; AI không đọc, không persist |
| Save cấp chương: content + lock + word_count + `scene_versions` | Save, dirty baseline, version head | chưa có; Save chỉ cảnh báo |
| danh sách version scene và restore | version dialog, restore | fixture `buildPreviewVersions`; không persist |
| dirty/navigation contract từ backend | guard, refresh/close | local state + `useBlocker`/`beforeunload` |
| commit confirmation/status/navigation | H2 commit dialog và post-commit View | chỉ UI preview: Start mở Commit Confirmation Dialog, kết quả và điều hướng là mô phỏng DEV-only tới project mẫu; không có backend |
| `system_language` và `project.language` tách biệt | chat và nội dung | chỉ locale UI, mock language |

## 14. Gap Matrix

| ID | Khu vực | Core yêu cầu | FE hiện tại | Trạng thái | Ảnh hưởng |
| :--- | :--- | :--- | :--- | :--- | :--- |
| GAP-01 | Progress | Hai progress theo source | Một progress | `[GAP]` | người dùng không thấy phần đã xác nhận so với AI đề xuất |
| GAP-02 | Phase 0 | Project Overview card | Không có card; header DraftOverviewBlock là Phase 1 | `[GAP]` | thiếu UI mapping Phase 0 |
| GAP-03 | Patch | Preview/confirm/reject/apply all/expired | Không có | `[GAP]` | không thể xử lý `pending_confirmation` |
| GAP-04 | Lock | Draft busy, read-only snapshot | Không có | `[GAP]` | không có phản hồi khi workflow giữ khóa |
| GAP-05 | Cancel | Hủy có kiểm soát | Không có | `[GAP]` | người dùng không thể dừng workflow |
| GAP-06 | Commit | Start transaction, progress, rollback, read-only | Start mở dialog preview; pending/error/success chỉ mô phỏng | `[GAP]` | không commit được |
| GAP-07 | Version | Restore sealed version, phase/subphase | Dropdown mock | `[GAP]` | không quay lại được version |
| GAP-08 | Crash | Phát hiện gián đoạn, phục hồi sealed | Không có | `[GAP]` | refresh mất dữ liệu |
| GAP-09 | Writing | Một chương, context, continuity, version | UI preview Chapter Detail; chưa có committed gate/backend/continuity | `[GAP]` | UI có nhưng chưa dùng được với plan thật |
| GAP-10 | Character | Library reuse qua `project_characters` | Không có link UI | `[GAP]` | không tái sử dụng được |
| GAP-11 | Flat data | `scene_id`, `source_scene_id`, cho phép trùng | string `sceneLearned`/`sourceScene` | `[GAP]` | không commit đúng schema |
| GAP-12 | Code | Không hiển thị code | Hiển thị slug/code | `[CONFLICT]` | trái core 3.8 |
| GAP-13 | Cascade | Preview, source protection, atomic | Không có draft cascade UI | `[GAP]` | không xóa cha an toàn |
| GAP-14 | Identifiers | `draft_session_id`, `turn_id`, `head_version_seq` | Local state | `[GAP]` | không có persistence/version |
| GAP-15 | Process detail | Render từ toàn bộ `businessFlowData` | Chỉ require | `[GAP]` | thiếu ngữ cảnh tiến độ |
| GAP-16 | Scene Save | Atomic content + lock + word_count + `scene_versions` | Save chỉ cảnh báo, giữ local | `[GAP]` | không persist nội dung scene |
| GAP-17 | Scene Generation | Stream/cancel, buffer, lỗi, note tùy chọn | Preview sample text; note local không được AI đọc | `[GAP]` | không sinh nội dung thật theo note |
| GAP-18 | H2 Commit | Dialog, validation, pending, idempotent, navigation | Dialog UI `[MOCK]`; validation/patch từ fixture; điều hướng mô phỏng tới project mẫu; idempotency/commit thật chưa có | `[GAP]` | chưa commit/điều hướng |
| GAP-19 | Scene Read-model | `scenes.id`, `position`, `content`, `word_count`, `is_locked` | `sceneId` preview `scn-*` | `[GAP]` | không có identity/persistence thật |
| GAP-20 | Scene Lock | Persist `scenes.is_locked` | Local `block.locked` | `[GAP]` | khóa không bền |
| GAP-21 | Responsive | Chapter Detail compact 1279px | Chat overlay từ 1279 trở xuống; header không overflow; breadcrumb truncate; label ẩn <510, stack <420 | `[IMPLEMENTED]` | sửa trong Wave 0.2; không đổi breakpoint toàn cục |

## 15. Conflict Matrix

Phân loại ban đầu lấy từ báo cáo local agent; đã kiểm tra lại mã nguồn. Không tự đổi phân loại để giảm số conflict.

| ID | Business Core | FE | File/dòng | Mức ảnh hưởng | Đề xuất | Trạng thái |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| C1 | 46 thuộc tính Master list (core 3.1) | 50 thuộc tính (progressEngine) | `src/features/business-flow/progressEngine.ts:11-79`, core 3.1 | Cao: sai mẫu số progress | Người dùng xác nhận danh sách chuẩn; nếu FE đúng thì core cần cập nhật 46 → 50, nếu core đúng thì FE phải bỏ 4 thuộc tính | `[CONFLICT]` |
| C2 | Hai progress: Đã xác nhận / AI đề xuất (core 3.1) | Một progress duy nhất | `ChatHeader.tsx:50-61`, `progressEngine.ts:430-453` | Cao: thiếu thông tin provenance | Ưu tiên implement hai progress theo core; cần dữ liệu `source` | `[CONFLICT]` |
| C3 | Project Overview card ở Phase 0 (core 2.1) | Không có card; header DraftOverviewBlock là Phase 1 | `DraftOverviewBlock.tsx:89-155`, core 2.1 | Trung bình: thiếu UI mapping Phase 0 | Thêm card Phase 0 hoặc chốt lại core nếu không còn cần | `[GAP]` |
| C4 | `feedbackContext {code, tag, title, content}` (core 3.4) | `{id, tag, title}`, không `content` | `draft/types.ts:11-16`, `DraftWorkspace.tsx:118-121` | Cao: AI không nhận đúng scope/payload | FE đổi sang `code` ổn định và bổ sung `content`; giữ tag/title | `[CONFLICT]` |
| C5 | Không hiển thị `code` trên UI (core 3.8) | Hiển thị slug/code ở ChapterBlock, WorldSettingsBlock, SceneConfigBlock | `ChapterBlock.tsx:116`, `WorldSettingsBlock.tsx:89`, `SceneConfigBlock.tsx:144` | Cao: trái quy tắc định danh | Sửa FE để ẩn code; giữ code trong data/tool | `[CONFLICT]` |
| C6 | Restore version thật, phase/subphase (core 3.6) | Dropdown mock, chỉ đổi label | `DraftToolbar.tsx:53-97` | Cao: không quay lại được | Implement restore sau khi có backend | `[GAP]` |
| C7 | Commit transaction + H2 dialog/navigation (core 3.5, 3.7) | Start mở dialog preview (0.3A); không commit thật | `DraftWorkspace.tsx:112`, `CommitConfirmationDialog.tsx`, `commitNavigation.ts` | Cao: không commit/điều hướng thật | Wave 0.3B đã có điều hướng **mô phỏng** tới project mẫu; còn cần commit backend + điều hướng tới project thật; không coi đóng panel hay success-preview là commit | `[GAP]` |
| C8 | Pending patches chặn Start (core 3.7) | Không có patch UI | không có | Cao: Start có thể bỏ qua dữ liệu chờ | Implement patch UI và block Start | `[GAP]` |
| C9 | Workflow lock/cancel (core 3.6, 3.9) | Không có UI | không có | Cao: không kiểm soát được workflow | Implement trạng thái và cancel | `[GAP]` |
| C10 | Writing Mode chapter-first (core 1.6, H1 APPROVED) | Preview surface đã có; backend/committed gate chưa | `ChapterDetailView.tsx`, `ProjectCreatorPanel.tsx` | Cao: UI có nhưng chưa dùng được với plan thật | Hoàn thiện backend/gate; không coi preview là production | `[GAP]` |
| C11 | State có `source` từng trường (core 1.3) | Mock draft không có source | `business-flow/types.ts:364`, `draft/types.ts:250` | Cao: không tính được progress/patch | Bổ sung source vào schema FE khi có backend | `[GAP]` |
| C12 | `draft_session_id`, `turn_id`, `head_version_seq` (core 3.6) | Local state, không id | `ChatView.tsx:25-28`, `DraftWorkspace.tsx:21-25` | Cao: không persistence/version | Bổ sung contract dữ liệu, không tự tạo schema | `[GAP]` |

## 16. Xung đột và câu hỏi cần người dùng chốt

### Q-C1. Danh sách Master attributes chuẩn là 46 hay 50?

- **Mô tả:** Core 3.1 ghi 46; FE `MASTER_ATTRIBUTES_LIST` có 50. Cần xác định đâu là danh sách chuẩn.
- **Nguồn Core:** 3.1.
- **Nguồn FE:** `src/features/business-flow/progressEngine.ts:11-79`.
- **Phương án A:** Core đúng 46; FE phải rà và bỏ 4 thuộc tính thừa.
- **Phương án B:** FE đúng 50; core cập nhật 46 → 50 và giải thích 4 thuộc tính bổ sung.
- **Tác động:** mẫu số progress, điều kiện readiness, tài liệu test.
- **Đề xuất:** chưa chốt; cần đối chiếu danh sách nghiệp vụ gốc. Không tự sửa core hoặc FE.
- **Trạng thái:** Chưa chốt.

### Q-C2. Progress hiển thị một hay hai phần trăm?

- **Mô tả:** Core 3.1 yêu cầu hai progress (đã xác nhận/AI đề xuất); FE đang một progress.
- **Nguồn Core:** 3.1.
- **Nguồn FE:** `ChatHeader.tsx:50-61`, `progressEngine.ts:430-453`.
- **Phương án A:** Implement hai progress theo core; cần `source` trong State.
- **Phương án B:** Giữ một progress và sửa core; cần quyết định nghiệp vụ.
- **Tác động:** UI header, ProcessDetailDialog, logic tính progress.
- **Đề xuất:** ưu tiên A vì core đã chốt provenance.
- **Trạng thái:** Chưa chốt.

### Q-C3. Project Overview Phase 0 có cần card riêng?

- **Mô tả:** Core 2.1 mô tả Card Project Overview; FE không có card này, header DraftOverviewBlock là Phase 1.
- **Nguồn Core:** 2.1, 3.4.
- **Nguồn FE:** `DraftOverviewBlock.tsx:89-155`.
- **Phương án A:** Thêm card Phase 0 và attach `feedbackContext.code = require.project`.
- **Phương án B:** Bỏ card khỏi core nếu UI không cần.
- **Tác động:** feedback mode, Phase 0 mapping, điều hướng.
- **Đề xuất:** giữ core, bổ sung UI khi làm Phase 0.
- **Trạng thái:** Chưa chốt.

### Q-C4. `feedbackContext` dùng `code` hay `id`?

- **Mô tả:** Core yêu cầu `{code, tag, title, content}`; FE dùng `{id, tag, title}`, không content.
- **Nguồn Core:** 3.4.
- **Nguồn FE:** `draft/types.ts:11-16`, `DraftWorkspace.tsx:118-121`.
- **Phương án A:** FE đổi sang `code` ổn định và gửi `content` JSON.
- **Phương án B:** Core đổi sang `id`; không nên vì `id` có thể là UI id không ổn định.
- **Tác động:** scope tool call, bảo vệ source, Project Overview attach.
- **Đề xuất:** A.
- **Trạng thái:** Chưa chốt.

### Q-C5. Có tiếp tục hiển thị `code` trên UI không?

- **Mô tả:** Core 3.8 cấm; FE hiển thị scene code, lore code, parent code, style/narrator code.
- **Nguồn Core:** 3.8.
- **Nguồn FE:** `ChapterBlock.tsx:116`, `WorldSettingsBlock.tsx:89`, `SceneConfigBlock.tsx:144`.
- **Phương án A:** Ẩn toàn bộ code; dùng display name/vị trí.
- **Phương án B:** Chỉ cho phép code ở chế độ debug, không cho người dùng thường.
- **Tác động:** UI draft, copy/feedback, debugging.
- **Đề xuất:** A; đây là quy tắc đã chốt, không cần mở lại.
- **Trạng thái:** Đề xuất xử lý, không chờ quyết định nghiệp vụ.

### Q-C6. Writing Mode: H1 đã chốt, còn backend contract

- **Mô tả:** Core 1.6 đã `APPROVED` mô hình chương-first, cảnh là block; generation buffer/review/accept; Save cấp chương; source/restore/lock/dirty/continuity theo scene. Wave 0 đã có UI preview tại §9.
- **Nguồn Core:** 1.6, Q4.
- **Nguồn FE:** `ChapterDetailView.tsx`, `ProjectCreatorPanel.tsx`, `scenePreview.ts`.
- **Đã chốt:** H1 business contract; không mở lại scene page/paragraph model/chapter_versions.
- **Còn mở:** idempotency Save, stale write, source hiện tại của scene, audit lock, read-model scene identity, generation stream/cancel, commit retry/recovery.
- **Đề xuất:** giữ preview là UI-only; backend làm theo Q4; không coi fixture là `scene_versions`.
- **Trạng thái:** `APPROVED` cho business; `OPEN` cho chi tiết kỹ thuật.

### Q-C7. Schema follow-up cho idempotent sealing

- **Mô tả:** Core 3.6 ghi `[SCHEMA FOLLOW-UP]` cho `draft_versions`; FE không có persistence nên chưa thể xác minh.
- **Nguồn Core:** 3.6.
- **Nguồn FE:** không có.
- **Phương án A:** Chốt `seal_key` + unique `(draft_session_id, seal_key)` ở DBML sau.
- **Phương án B:** Dùng unique `(draft_session_id, turn_id)` và cột workflow/round riêng.
- **Tác động:** chống version trùng, migration, backend.
- **Đề xuất:** để backend chốt cùng Q3; chưa sửa DBML.
- **Trạng thái:** Chưa chốt.

### Q-C8. Task identity/checkpoint lưu ở đâu?

- **Mô tả:** Core 3.9 đã chốt nguyên tắc resume nhưng chưa chốt cấu trúc task/checkpoint.
- **Nguồn Core:** 3.9, Q3.
- **Nguồn FE:** không có.
- **Phương án A:** Dùng report + State + sealed version, không thêm bảng.
- **Phương án B:** Thêm task ledger/checkpoint schema tối thiểu.
- **Tác động:** resume, duplicate prevention, schema.
- **Đề xuất:** giữ Q3 mở; không tự thêm task ledger.
- **Trạng thái:** Chưa chốt.

### Q-C9. Đồng bộ schema dữ liệu FE với `BusinessFlowData`

- **Mô tả:** FE draft dùng camelCase/`DRAFT_OVERVIEW`; core dùng `BusinessFlowData` snake_case, source, patch, version.
- **Nguồn Core:** 1.2, 2.2-2.7, 3.6.
- **Nguồn FE:** `draft/types.ts`, `business-flow/types.ts`.
- **Phương án A:** Tạo adapter FE → `BusinessFlowData` trước khi nối backend.
- **Phương án B:** Đổi toàn bộ component draft sang schema core.
- **Tác động:** lớn cho component, test, mapping.
- **Đề xuất:** A trước, B sau khi backend ổn định.
- **Trạng thái:** Chưa chốt.

### Q-C10. Backend contract cho Save và version nội dung cảnh

- **Mô tả:** Core 1.6 chốt Save cấp chương chỉ gồm scene dirty, một `scene_versions` cho mỗi scene có nội dung thay đổi, lock-only Save không tạo content version. Backend contract chưa có.
- **Nguồn Core:** 1.6, Q4; DBML `scenes`, `scene_versions`.
- **Nguồn FE:** `ChapterDetailView.tsx` (Save chỉ cảnh báo), `scenePreview.ts` (fixture).
- **Cần chốt:** idempotency của Save; optimistic concurrency/stale write; source hiện tại của scene; audit lock; read-model `scenes.id`/`position`; generation stream/cancel; cách generation nhận nội dung scene + note tùy chọn.
- **SCHEMA FOLLOW-UP:** `scene_versions` chưa có khóa idempotency, `parent_id` nullable; `scenes` chưa có timestamp/source.
- **Đề xuất:** backend chốt API/schema cùng core Q4; FE chỉ hiển thị trạng thái thật, không tự tạo version.
- **Trạng thái:** `OPEN` / `SCHEMA FOLLOW-UP` / `IMPLEMENTATION PENDING`.

## 17. Changelog

### v0.3.2 - 2026-11-06

- Cleanup tiền-merge Stage B: Unlock All bị chặn khi generating/review (`canBulkChangeLocks`, `applyBulkLock`); nhãn Chat Chapter Detail dùng i18n `project.chat.label` và có `focus-ring`.
- Commit dialog: ẩn nút X khi pending (cục bộ), ghi rõ DEV-only simulation và ranh giới UI `[MOCK]` vs backend `[GAP]`.
- `ProjectCreatorPanel` dùng `buildCreatorChapterPath`; thêm test N12/N13 và 3 test F03; `npm test` 39 test.
- Sửa mô tả lỗi thời “Start đóng panel” ở §13/§14/§15 và tham chiếu `DraftWorkspace.tsx`.

### v0.3.1 - 2026-11-06

- Wave 0.3B: dialog +2px typography, `max-w-[44rem]`; điều hướng mô phỏng sau commit tới project mẫu (`commitNavigation.ts`, N01–N11).

### v0.3 - 2026-11-06

- Wave 0.3A: nút “Xác nhận” bước 6 mở Commit Confirmation Dialog preview (`commitPreview.ts`, C01–C12); test mặc định gộp mọi feature.

### v0.2.2 - 2026-11-06

- Hardening dirty: tính lại từ content/lock so với baseline; source metadata không tạo dirty; sửa-rồi-hoàn-tác và lock/unlock về baseline đều sạch.
- Hardening source: Accept → AI_GENERATED, sửa tay → HUMAN_EDITED, Restore → RESTORE, Restore rồi sửa → HUMAN_EDITED; dialog/cancel/discard/lock không đổi source.
- Hardening generation: thêm `requestId`, cancel/discard/unmount vô hiệu hóa callback cũ; chặn double Confirm; giữ một generation/chương.
- Hardening lock: khóa scene active và Lock All bị disable khi generating/review; lock scene khác vẫn an toàn; Accept bị chặn nếu scene khóa.
- Long content: editor cap `70vh`, review sample cap `50vh`; internal scroll, controls và Accept/Discard vẫn tiếp cận được.
- Responsive: Chat Chapter Detail dùng overlay từ 1279 trở xuống; header ẩn label khi <510, xếp hai hàng khi <420; breadcrumb truncate; hết overflow 11px tại 1280.
- Thêm 6 automated test tại `scenePreview.test.mjs`; cập nhật §9.6/§9.7, GAP-21.

### v0.2.1 - 2026-11-06

- Thêm Regenerate dialog tại §9: feedback tùy chọn, tối đa 2.000 ký tự, `note` rỗng/khoảng trắng chuẩn hóa `null`, note khác rỗng được trim.
- Confirm tạo request local `{sceneId, note}` cho đúng scene; Hủy/X/Escape không sinh generation; note không persist, không append vào scene/sample, không gửi sang scene khác.
- Review preview hiển thị “Ghi chú đã gửi (chỉ xem trước)”; AI không đọc note trong preview, tiêu thụ note thật là `[IMPLEMENTATION PENDING]`.
- Giữ nguyên Generate cho scene rỗng, Save cấp chương, version, lock, dirty/guard, layout và UI Preservation Invariant.
- Thêm key i18n VI/EN; cập nhật §13, GAP-17 và Q-C10.

### v0.2 - 2026-11-06

- Đối chiếu Business Core v2.4; chuyển H1 sang hợp đồng chương-first, cảnh là block và ghi nhận Wave 0 preview.
- Cập nhật §9: Project View/Creator, Chapter Detail là Writing Mode surface, scene label/control/buffer/review, version dialog, dirty/lock/guard, Save cảnh báo; tách rõ preview và production.
- Cập nhật §7: H2 Commit Confirmation Dialog, `phase_confirm` khác `commit`, failure/success/navigation; FE vẫn `[GAP]`/`IMPLEMENTATION PENDING`.
- Cập nhật §13–§15: scene read-model, generation, Save/version, lock, committed gate, H2 commit; thêm GAP-16…GAP-21 và cập nhật C7/C10.
- Cập nhật Q-C6 (H1 đã chốt) và thêm Q-C10 (backend Save/version scene); giữ Q-C1…Q-C5, Q-C7…Q-C9.
- Ghi nhận known issue 1280px Chat/header overflow tại §9.6; không coi là layout change được duyệt.
- Giữ nguyên UI Preservation Invariant §12.1; không sửa `src/`, `docs/db.dbml` hoặc `docs/design-system.md` trong task tài liệu này.

### v0.1 - 2026-11-06

- Tạo FE Core lần đầu, đối chiếu Business Core v2.3.
- Ghi nhận kiến trúc FE prototype: React Router + Zustand + mock, không backend/persistence.
- Ghi nhận trạng thái `[IMPLEMENTED]`, `[MOCK]`, `[TARGET]`, `[GAP]`, `[CONFLICT]` cho Chat, Draft, Source/Patch, Workflow, Start/Commit, Persistence/Version, Writing Mode, Character Library.
- Lập Gap Matrix và Conflict Matrix C1-C12.
- Xác định các câu hỏi cần người dùng chốt: master attributes 46/50, một/hai progress, Project Overview, feedbackContext code/id, code visibility, Writing Mode, schema follow-up, task checkpoint, adapter schema.
- Bổ sung UI Preservation Invariant tại 12.1: bảo toàn giao diện mặc định, chỉ thay đổi bổ sung, cần phê duyệt rõ ràng cho thay đổi layout, áp dụng cho cả agent chính và subagent, vi phạm bị đánh giá FAILED.
- Không sửa `src/`, `docs/db.dbml` hoặc `docs/design-system.md`.

## Phụ lục A. Bằng chứng kiểm tra chính

| Nhận định | Bằng chứng |
| :--- | :--- |
| Không có backend/Tauri | `package.json`; không có `src-tauri`; không có `@tauri-apps/api` |
| Chat dùng sample data | `src/features/chat/ChatView.tsx:33` |
| Chat gửi tin giả | `src/features/chat/ChatView.tsx:79-171` |
| Một progress bar | `src/features/chat/ChatHeader.tsx:50-61` |
| 50 attributes | `src/features/business-flow/progressEngine.ts:11-79` |
| Draft data mock camelCase | `src/features/draft/mockOverview.ts:3`, `draft/types.ts:250` |
| Draft 6 bước | `src/features/draft/DraftHeader.tsx:19-37` |
| Start mở dialog preview | `src/features/draft/DraftWorkspace.tsx:112` (`handleStart`) |
| Version dropdown mock | `src/features/draft/DraftToolbar.tsx:53-97` |
| Feedback context id/tag/title | `src/features/draft/types.ts:11-16` |
| Code hiển thị trên UI | `ChapterBlock.tsx:116`, `WorldSettingsBlock.tsx:89`, `SceneConfigBlock.tsx:144` |
| Chapter writing preview | `src/features/projects/ChapterDetailView.tsx:120-121`, `:263-330` |
| Scene identity preview | `src/features/projects/scenePreview.ts:30-36`, `types.ts:6` |
| Creator switch/route | `src/features/projects/ProjectDetailView.tsx:23-34`, `:117`; `ProjectCreatorPanel.tsx:19-72` |
| Scene labels/generation/version UI | `ChapterDetailView.tsx:580-690`, `:730-800` |
| Regenerate feedback dialog | `ChapterDetailView.tsx:316-336`, `:780-840`; `scenePreview.ts:17-28`; key i18n `project.scene.regenerate*` |
| Dirty helper | `scenePreview.ts:isSceneDirty`; `ChapterDetailView.tsx` dùng helper cho từng scene |
| Source helper | `scenePreview.ts:sourceForSceneAction` |
| Generation request identity | `ChapterDetailView.tsx:generationRequestId`; timer callback kiểm tra `requestId` |
| Long content cap | `ChapterDetailView.tsx` textarea `max-h-[70vh]`, review `max-h-[50vh]` |
| Responsive header | `ChapterDetailView.tsx` `headerRef` + `ResizeObserver`; compact Chat `useIsCompact` |
| Automated tests | `npm test` → `src/features/**/*.test.mjs` (39 test sau cleanup Stage B) |
| Dirty guard | `ChapterDetailView.tsx:150-177`, `:209-230`; `useBlocker`, `beforeunload` |
| Save preview warning | `ChapterDetailView.tsx:209-217` |
| Không có `scene_versions` thật | Không có API/DB; fixture `buildPreviewVersions` |
| Known 1280 header gap | Đo tại 1280px với Chat mở: `scrollWidth` 464 / `clientWidth` 453 |
| Character store mock | `src/features/characters/store.ts`, `CharacterListView.tsx:13-25` |
| Subagent settings UI-only | `src/features/settings/subagentConfig.ts:1` |
| Breakpoint 1280 | `src/shared/hooks/useMediaQuery.ts:23` |
| Chỉ localStorage theme/locale | `src/app/root.tsx:41`, `src/shared/i18n/store.ts:8`, `src/shared/hooks/useTheme.ts:14` |
