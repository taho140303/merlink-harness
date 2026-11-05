# FE CORE: CƠ CHẾ GIAO DIỆN MERLINK HARNESS

> **Phiên bản:** 0.1
> **Đối chiếu:** `docs/business/core.md` v2.3
> **Vai trò:** SSOT cơ chế giao diện và hợp đồng tương tác FE của Merlink Harness. Business Core quyết định hệ thống phải làm gì; tài liệu này mô tả giao diện hiện đang làm gì và cần làm gì. `docs/design-system.md` quyết định cách thể hiện thị giác.
> **Phạm vi kiểm tra:** `src/features/chat`, `src/features/draft`, `src/features/business-flow`, `src/features/projects`, `src/features/characters`, `src/features/worldbuilding`, router, Zustand store, i18n và mock data.
> **Thời điểm kiểm tra:** 2026-11-06.
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

Quy ước nguồn:

- **Core:** số mục trong `docs/business/core.md` v2.3.
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

1. Không có Working State thống nhất. Chat và Draft Workspace dùng hai nguồn mock khác nhau.
2. Không có `draft_session_id`, `session_id`, `head_version_seq`, `pending_patches` hoặc tool execution thật.
3. Không có draft lock, commit transaction, version restore, workflow execution, crash recovery hay Writing Mode thực tế.
4. Mọi tuyên bố về các cơ chế trên chỉ là `[TARGET]` hoặc `[GAP]`.

### 1.2 Nguồn dữ liệu mock

| Nguồn | Dùng cho | Đặc điểm |
| :--- | :--- | :--- |
| `src/features/business-flow/mockData.ts:3` | `SAMPLE_BUSINESS_FLOW_DATA` cho Chat, progress, ProcessDetailDialog | snake_case, không có `source`, không có `pending_patches` |
| `src/features/draft/mockOverview.ts:3` | `DRAFT_OVERVIEW` cho Draft Workspace | camelCase, có `versionHistory`, `segments`, `sceneConfig`; không phải `BusinessFlowData` |
| `src/features/projects/mockProjects.ts:16` | Project list, Project detail, Chapter editor | mock project, arc, chapter, block |
| `src/features/characters/mockData.ts` | Character library, Character detail | mock character, competence, memory, relationship |
| `src/features/system-prompt/mockData.ts` | System prompt/config | mock; ngoài phạm vi draft |

Hai nguồn `SAMPLE_BUSINESS_FLOW_DATA` và `DRAFT_OVERVIEW` không có adapter, không đồng bộ và dùng schema khác nhau. Vì vậy không được coi chúng là hai view của cùng một Working State.

### 1.3 State ownership hiện tại

| State | Nơi giữ | Persist | Ghi chú |
| :--- | :--- | :--- | :--- |
| Chat messages | `useState` trong `ChatView` | Không | mất khi unmount/refresh |
| Draft step, sub-step, version label, feedback mode | `useState` trong `DraftWorkspace` | Không | không có draft id |
| Feedback context | `useState` trong `ChatView` | Không | chỉ là chip trong input |
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
| `/project/:projectId` | `ProjectDetailView` | `[MOCK]` | project, arc, chapter, chat pane |
| `/project/:projectId/chapter/:chapterId` | `ChapterDetailView` | `[MOCK]` | chapter editor dạng block; không phải Writing Mode |
| `/character` | `CharacterListView` | `[MOCK]` | thư viện nhân vật, filter project/sex, tạo mới local |
| `/character/:characterId` | `CharacterDetailLayout` | `[MOCK]` | tab core/life/psych, edit local, chat pane |
| `/worldbuilding/:sub` | `WorldbuildingView` | `[MOCK]` | màn “Under Construction”; design-system loại trừ |
| `/config` | `ConfigPage` | `[MOCK]` | system prompt/config; ngoài phạm vi draft |

- Draft Workspace **không phải route**. Nó là pane phải trong `ChatView`, mở bằng nút Draft Building và đóng bằng nút collapse trong `DraftHeader`.
- `src/app/routes.ts` khai báo toàn bộ route.
- Breakpoint responsive: dưới `1280px` split pane chuyển thành một pane + overlay; nguồn `src/shared/hooks/useMediaQuery.ts:23`.
- Split pane chat/draft mặc định 50/50, min 545px/530px; nguồn `src/features/chat/ChatView.tsx:66` và `src/shared/hooks/useResizablePanel.ts`.

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
| DR-09 | `[MOCK]` | Start/“Xác nhận” chỉ xuất hiện ở bước 6; bấm chỉ đóng panel | `handleStart = onClose()` | 3.5, 3.7 | `DraftWorkspace.tsx:104-106`, `DraftToolbar.tsx:43-48` | không commit; xem C7 |
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

### 7.1 Contract đích

| ID | Trạng thái | Quy tắc UI | Dữ liệu/hành vi | Nguồn Core | Nguồn FE | Ghi chú |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| ST-01 | `[MOCK]` | Start/“Xác nhận” chỉ hiện ở bước 6; bấm đóng panel | `handleStart` gọi `onClose` | 3.5, 3.7 | `DraftWorkspace.tsx:104-106` | không commit |
| ST-02 | `[GAP]` | Start bị chặn khi còn `pending_patches`; UI yêu cầu xử lý patch trước | Không có patch UI | 3.7 | không có | |
| ST-03 | `[GAP]` | Trạng thái `editing` → `committing` → `committed`; hiển thị tiến độ commit | Không có | 3.6, 3.7 | không có | |
| ST-04 | `[GAP]` | Commit lỗi thì rollback về `editing`; draft giữ nguyên để thử lại | Không có | 3.7 | không có | |
| ST-05 | `[GAP]` | Start lặp idempotent; không tạo project mới | Không có | 3.7 | không có | |
| ST-06 | `[GAP]` | Sau commit draft read-only; chuyển sang Writing Mode theo plan đã commit | Không có | 3.6, 1.5 | không có | H2 còn mở điều hướng |

### 7.2 Hiện trạng

- Start chỉ là nút “Xác nhận” trong `DraftToolbar`; nhãn “Start” chỉ tồn tại trong `DraftStepNav` không dùng.
- Không có commit transaction, không có `committing`, không có progress, không có rollback, không có read-only sau commit.
- Sau khi đóng DraftWorkspace, ChatView vẫn giữ trạng thái local; không có project mới hoặc navigation.
- Không có kiểm tra `pending_patches` trước Start.

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

## 9. Writing Mode

### 9.1 Contract đích

| ID | Trạng thái | Quy tắc UI | Dữ liệu/hành vi | Nguồn Core | Nguồn FE | Ghi chú |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| WR-01 | `[GAP]` | Writing Mode chỉ mở sau khi plan đã commit | Không có plan commit | 1.5 | không có | |
| WR-02 | `[GAP]` | Người dùng chọn một chương; nạp context chương | Không có chọn chương từ plan | 1.5 | không có | |
| WR-03 | `[GAP]` | Viết/chỉnh nội dung các cảnh trong chương; kiểm tra continuity | Không có generation | 1.5 | không có | |
| WR-04 | `[GAP]` | Không viết nhiều chương, không tự chuyển chương, không background bulk | Không có Writing pipeline | 1.5 | không có | |
| WR-05 | `[MOCK]` | Chapter editor có block editing, lock/unlock từng block, save/cancel local | `ChapterDetailView` dùng state local | H1, 3.6 | `ChapterDetailView.tsx:70-89`, `ChapterDetailView.tsx:330-360` | không phải Writing Mode |
| WR-06 | `[MOCK]` | Project chat pane mở cạnh chapter/project | Chat pane giả, không inject context | 3.8 | `ProjectChatPane.tsx:15-80` | không có tool call thật |

### 9.2 Hiện trạng

- `ChapterDetailView` là màn hình đọc/sửa chapter dạng block; có `isEditing`, `editSnapshot`, block `locked`, save/cancel. Đây là editor local, không phải pipeline generation.
- Block lock là state UI local; không phải khóa backend, không gắn với `scenes.is_locked`, không tạo `scene_versions`.
- Không có nút “bắt đầu Writing Mode”, không có chọn chương từ plan, không có context builder, không có continuity check, không có version nội dung.
- H1 còn mở chi tiết generation/versioning/UI; H2 (điều hướng sau commit) và H3 (View/Creator) chưa có UI tương ứng.

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

Đây là danh sách dữ liệu/hành vi FE cần để thực hiện Core v2.3. Không đặt tên IPC command/event mới; tên cụ thể do backend và FE chốt sau.

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
| scene content/version (Writing Mode) | chapter editor, version nội dung | block local, không version |
| `system_language` và `project.language` tách biệt | chat và nội dung | chỉ locale UI, mock language |

## 14. Gap Matrix

| ID | Khu vực | Core yêu cầu | FE hiện tại | Trạng thái | Ảnh hưởng |
| :--- | :--- | :--- | :--- | :--- | :--- |
| GAP-01 | Progress | Hai progress theo source | Một progress | `[GAP]` | người dùng không thấy phần đã xác nhận so với AI đề xuất |
| GAP-02 | Phase 0 | Project Overview card | Không có card; header DraftOverviewBlock là Phase 1 | `[GAP]` | thiếu UI mapping Phase 0 |
| GAP-03 | Patch | Preview/confirm/reject/apply all/expired | Không có | `[GAP]` | không thể xử lý `pending_confirmation` |
| GAP-04 | Lock | Draft busy, read-only snapshot | Không có | `[GAP]` | không có phản hồi khi workflow giữ khóa |
| GAP-05 | Cancel | Hủy có kiểm soát | Không có | `[GAP]` | người dùng không thể dừng workflow |
| GAP-06 | Commit | Start transaction, progress, rollback, read-only | Start đóng panel | `[GAP]` | không commit được |
| GAP-07 | Version | Restore sealed version, phase/subphase | Dropdown mock | `[GAP]` | không quay lại được version |
| GAP-08 | Crash | Phát hiện gián đoạn, phục hồi sealed | Không có | `[GAP]` | refresh mất dữ liệu |
| GAP-09 | Writing | Một chương, context, continuity, version | Chapter editor mock | `[GAP]` | chưa có Writing Mode |
| GAP-10 | Character | Library reuse qua `project_characters` | Không có link UI | `[GAP]` | không tái sử dụng được |
| GAP-11 | Flat data | `scene_id`, `source_scene_id`, cho phép trùng | string `sceneLearned`/`sourceScene` | `[GAP]` | không commit đúng schema |
| GAP-12 | Code | Không hiển thị code | Hiển thị slug/code | `[CONFLICT]` | trái core 3.8 |
| GAP-13 | Cascade | Preview, source protection, atomic | Không có draft cascade UI | `[GAP]` | không xóa cha an toàn |
| GAP-14 | Identifiers | `draft_session_id`, `turn_id`, `head_version_seq` | Local state | `[GAP]` | không có persistence/version |
| GAP-15 | Process detail | Render từ toàn bộ `businessFlowData` | Chỉ require | `[GAP]` | thiếu ngữ cảnh tiến độ |

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
| C7 | Commit transaction, rollback, read-only (core 3.7) | Start đóng panel | `DraftWorkspace.tsx:104-106` | Cao: không commit | Implement commit flow; không coi đóng panel là commit | `[GAP]` |
| C8 | Pending patches chặn Start (core 3.7) | Không có patch UI | không có | Cao: Start có thể bỏ qua dữ liệu chờ | Implement patch UI và block Start | `[GAP]` |
| C9 | Workflow lock/cancel (core 3.6, 3.9) | Không có UI | không có | Cao: không kiểm soát được workflow | Implement trạng thái và cancel | `[GAP]` |
| C10 | Writing Mode sau commit (core 1.5) | Chapter editor mock | `ChapterDetailView.tsx:70-89` | Cao: chưa có writing | Giữ H1 mở; không coi editor là Writing Mode | `[GAP]` |
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

### Q-C6. Phạm vi Writing Mode và H1

- **Mô tả:** Core 1.5 đã chốt contract; H1 còn mở generation/versioning/UI. FE mới có chapter editor mock.
- **Nguồn Core:** 1.5, H1.
- **Nguồn FE:** `ChapterDetailView.tsx`.
- **Phương án A:** Giữ H1 mở; implement Writing Mode theo contract khi có backend.
- **Phương án B:** Mở rộng chapter editor thành Writing Mode ngay trong UI prototype.
- **Tác động:** scope FE, pipeline, version nội dung.
- **Đề xuất:** A; không tự thiết kế pipeline.
- **Trạng thái:** Chưa chốt.

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

## 17. Changelog

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
| Start chỉ đóng panel | `src/features/draft/DraftWorkspace.tsx:104-106` |
| Version dropdown mock | `src/features/draft/DraftToolbar.tsx:53-97` |
| Feedback context id/tag/title | `src/features/draft/types.ts:11-16` |
| Code hiển thị trên UI | `ChapterBlock.tsx:116`, `WorldSettingsBlock.tsx:89`, `SceneConfigBlock.tsx:144` |
| Chapter editor local | `src/features/projects/ChapterDetailView.tsx:70-89` |
| Character store mock | `src/features/characters/store.ts`, `CharacterListView.tsx:13-25` |
| Subagent settings UI-only | `src/features/settings/subagentConfig.ts:1` |
| Breakpoint 1280 | `src/shared/hooks/useMediaQuery.ts:23` |
| Chỉ localStorage theme/locale | `src/app/root.tsx:41`, `src/shared/i18n/store.ts:8`, `src/shared/hooks/useTheme.ts:14` |
