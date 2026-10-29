# Báo cáo đối chiếu mock data & tổ chức dữ liệu trên UI với `docs/db.dbml`

- **Ngày thực hiện**: 2026-10-29 (lần 1) · **cập nhật lần 5**: 2026-10-29
- **Phạm vi**: toàn bộ mock data trong `src/`, các `types.ts` định nghĩa shape dữ liệu, và cách các màn hình/tab tổ chức dữ liệu đó; đối chiếu với `docs/db.dbml`.
- **Trạng thái sửa**: Đã áp dụng toàn diện Cụm 1, 2, 3: cập nhật `db.dbml` (1.088 dòng, 55 bảng, 25 enum), tái cấu trúc toàn bộ `src/` theo Feature-Sliced Design (FSD / Bulletproof React), dọn sạch `src/core/`.
- **Kết luận ngắn**: Hệ thống đã được chuẩn hóa đồng bộ giữa DBML và UI, giải quyết dứt điểm các điểm xung đột cốt lõi.

---

## 0. TL;DR

| # | Kết luận | Mức |
|---|---|---|
| 1 | ~~Tồn tại hai model nhân vật không tương thích...~~ — **đã fix (1.1)**: Hợp nhất `CharacterProfile`, `CharacterDetailLayout` load nhân vật theo param ID từ URL `/character/:characterId`, có đủ 4 mock profile | ✅ P0-01 |
| 2 | ~~`projects.id` varchar vs `uuid` refs~~ — **đã fix**: `projects.id`, `sessions.project_id` đều `uuid`; `sessions.id`/`messages.id` giữ `varchar` | ✅ S-01 |
| 3 | ~~Enum `gender` thiếu...~~ — **đã fix (1.2)**: DBML `enum gender` có `NONE, MALE, FEMALE`. UI chỉ dùng 3 giá trị này, loại bỏ `Intersex, Unknown` | ✅ P0-03 |
| 4 | ~~`themes` thiếu `project_id`~~ — **đã chốt (3.4)**: Theme chỉ gắn với arc, không lưu `project_id` | ✅ P0-05 |
| 5 | ~~`characters` / `world_settings`...~~ — **đã fix (1.3 & Vấn đề 2)**: `world_settings` gắn `project_id`. `characters` độc lập, tái sử dụng qua `scene_characters` | ✅ P0-06 |
| 6 | ~~`project.details.*` là chuỗi format~~ — **đã chốt**: detail là view, không cần cột | ✅ P0-02 |
| 7 | ~~`sessions`/`messages` bị UI làm phẳng~~ — **đã fix (3.1)**: `SidebarHistory` lưu `projectId`, chỉ render `projectName` | ✅ P0-07 |
| 8 | ~~6 enum không bảng nào dùng~~ — **đã xoá** (28 → 22, sau đó +2 enum ở S-07 và +1 ContentStatus thành 25) | ✅ S-02 |
| 9 | ~~`character_memories` quá phẳng~~ — **đã fix (2.1)**: Theo DBML, ký ức phẳng và competence bao hàm knowledge, UI thu gọn lại | ✅ P1-07 |
| 10 | ~~Không có cơ chế soft-delete~~ — **đã fix (3.3)**: DBML đã thêm `deleted_at timestamp` vào các bảng chính (`projects`, `arcs`, `chapters`, `scenes`, `characters`, `world_settings`, `lore_entries`, `themes`) | ✅ P1-15 |

---

## 1. Nguồn & phương pháp đối chiếu

### 1.1. Phía schema — `docs/db.dbml`

| Chỉ số | Giá trị |
|---|---|
| Số dòng | 1.102 |
| Số bảng (`Table`) | **55** |
| Số enum (`enum` / `Enum`) | **26** (xoá 6 enum chết ở S-02; thêm `LoreUsageType`, `RevealDepth`, `ContentStatus`, `RhetoricalUsage`) |

Các phân vùng theo comment trong file:

| Phân vùng | Bảng |
|---|---|
| `SYSTEM_PROMPT.MOTIF` | `motifs`, `genres`, `motif_applicable_genres`, `components`, `states`, `state_rules`, `state_down_by_rules`, `emotional_ranges`, `event_blocks`, `event_transitions`, `event_block_emotionals`, `constraints`, `motif_constraints` |
| `SYSTEM_PROMPT.STYLE_PRESET` | `style_presets`, `rhetorical_devices`, `style_preset_rhetorical_devices`, `style_preset_vocab_items`, `style_preset_focus_items` |
| `SYSTEM_PROMPT.NARRATIVE_DIRECTIVES` | `narrator_configs`, `narrative_directives`, `style_preset_narratives` |
| `VFILL (TÍNH SAU)` | `symbolic_motifs`, `symbolics` (`character_motifs` đã chuyển lên trước `DATA.CONTENT`) |
| `DATA.CONTENT` | `projects`, `sessions`, `messages`, `project_genres`, `arcs`, `arc_motifs`, `arc_event_blocks`, `chapters`, `scenes`, `scene_characters`, `scene_versions` |
| `DATA.META` | `characters`, `appearance_option_groups`, `appearance_options`, `character_appearances`, `character_distinguishing_marks`, `relationships`, `relationship_categories`, `relationship_roles`, `relationship_states` |
| `DATA.META - WORLD BUILDING` | `world_settings`, `lore_entries` |
| `DATA.META - THEME` | `themes`, `theme_arguments` |
| `MEMORY SYSTEM` | `character_memories` (+ 4 enum không dùng) |
| `COMPETENCE` | `competence_domains`, `domain_level`, `character_competences`, `competence_capabilities`, `competence_limitations` |

`docs/db.dbdiagram` (5.006 dòng) chỉ là file layout toạ độ của dbdiagram.io, **không chứa thông tin schema** → không dùng để đối chiếu.

### 1.2. Phía UI — mock + types + component

| File | Dòng | Vai trò | Entity DBML liên quan |
|---|---|---|---|
| [`src/core/components/system-prompt/mockData.ts`](../../src/core/components/system-prompt/mockData.ts) | 768 | Thư viện system prompt: 4 motif, 3 genre, 11 event block, 7 transition, 10 style preset, 27 narrator config, 2 theme… | 23 bảng vùng SYSTEM_PROMPT + THEME |
| [`src/core/components/draft/mockOverview.ts`](../../src/core/components/draft/mockOverview.ts) | 252 | 1 project draft đầy đủ: arc → theme → segment → chapter → scene, character, world, lore | `projects`, `arcs`, `arc_motifs`, `arc_event_blocks`, `chapters`, `scenes`, `scene_characters`, `themes`, `characters`, `world_settings`, `lore_entries` |
| [`src/core/components/character-detail/mockData.ts`](../../src/core/components/character-detail/mockData.ts) | 366 | 1 hồ sơ nhân vật đầy đủ (Rimuru Tempest) | `characters`, `character_appearances`, `character_distinguishing_marks`, `relationships`, `character_memories`, `character_competences` |
| [`src/core/components/project/mockProjects.ts`](../../src/core/components/project/mockProjects.ts) | 311 | 14 project + 2 arc mẫu + block nội dung chương | `projects`, `arcs`, `chapters`, `scenes` |
| [`src/core/components/character/mockCharacters.ts`](../../src/core/components/character/mockCharacters.ts) | 142 | 3 nhân vật cho list/card/create | `characters`, `character_appearances`, `relationships` |
| [`src/core/components/sidebar/SidebarHistory.tsx`](../../src/core/components/sidebar/SidebarHistory.tsx) | 6 item | Lịch sử chat | `sessions` |
| [`src/core/components/chat/ChatView.tsx`](../../src/core/components/chat/ChatView.tsx) | — | Message sinh ra trong `useState`, không persist | `messages` |
| [`src/core/components/settings/RecycleBinTab.tsx`](../../src/core/components/settings/RecycleBinTab.tsx) | 4 item | Thùng rác | **không có bảng** |
| [`src/core/components/settings/types.ts`](../../src/core/components/settings/types.ts) | 25 | AI provider + model config | **không có bảng** |

### 1.3. Cách đối chiếu

Mỗi entity được so theo 3 trục:

1. **Trường**: cột DBML ↔ field trong `types.ts` / mock.
2. **Kiểu & enum**: kiểu DBML ↔ union type của UI, giá trị enum ↔ option UI.
3. **Tổ chức**: bảng DBML ↔ route/tab/khối UI đang hiển thị (một bảng có thể bị UI gộp hoặc tách).

Ký hiệu dùng trong báo cáo: ✅ khớp · ⚠️ khớp một phần / cần adapter · ❌ thiếu hoặc sai · 🧟 bảng/enum "chết" (không được dùng ở đâu).

---

## 2. Bản đồ tổ chức dữ liệu: DBML ↔ UI

### 2.1. Route ↔ miền dữ liệu

| Route | Component | Nguồn dữ liệu hiện tại | Miền DBML |
|---|---|---|---|
| `/` | `routes/home.tsx` | — | — |
| `/chat/:id` | `ChatView` + `DraftWorkspace` | `useState` (message) + `DRAFT_OVERVIEW` | `sessions`, `messages`, `projects` |
| `/project` | `ProjectView` | `MOCK_PROJECTS` | `projects`, `project_genres` |
| `/project/:projectId` | `ProjectDetailView` | `MOCK_PROJECTS` (tra theo id) | `projects`, `arcs`, `chapters` |
| `/project/:projectId/chapter/:chapterId` | `ChapterDetailView` | `MOCK_PROJECTS` → `chapter.blocks` | `chapters` + **`scenes` (không dùng)** |
| `/character` | `CharacterListView` | `MOCK_CHARACTERS` | `characters`, `character_appearances` |
| `/character/:characterId` | `CharacterDetailLayout` | store, khởi tạo bằng `rimuruProfile` — **không đọc `characterId`** | toàn bộ `DATA.META` |
| `/worldbuilding/lore` | `WorldbuildingPage` | — **"Under Construction"** | `lore_entries` |
| `/worldbuilding/settings` | `WorldbuildingPage` | — **"Under Construction"** | `world_settings` |
| `/config` | `SystemPromptLayout` (5 tab) | `system-prompt/mockData.ts` | 23 bảng SYSTEM_PROMPT + THEME |
| Settings dialog | `GeneralTab` / `AIProvidersTab` / `RecycleBinTab` | hardcode | **không có bảng** |

### 2.2. Ba "vũ trụ" dữ liệu song song trong UI

Đây là phát hiện quan trọng nhất về *tổ chức* dữ liệu: cùng một entity nhưng tồn tại 3 bản mock độc lập, không có khoá chung.

| Vũ trụ | Nơi dùng | Project | Nhân vật | ID |
|---|---|---|---|---|
| **A. Project/Chapter** | `/project`, `/project/:id`, `/chapter/:id` | Chronicles of Aethelgard, The Sunken Leyline… (14 project) | chỉ đếm `"18 entities"` | `proj-1`, `c-1`… |
| **B. Character** | `/character`, `/character/:id` | — | Kaelen Vance, Lyra Dawnseeker, Vaelin Grey, **Rimuru Tempest** | `char-1`…, `rimuru-tempest` |
| **C. Draft** | `DraftWorkspace` (mở từ `/` và `/chat/:id`) | **Tế nữ ngưu nhân** | Phù Thanh Cơ, Hạc Minh, Trưởng lão Hạc | `c-1`, `c-2`, `c-3` |
| **D. Sidebar history** | `SidebarHistory` | Merlink Core / Merlink Harness / Merlink Story | — | `1`…`6` |

Hệ quả:

- `SidebarHistory` dùng `chat.project` là **tên project dạng chuỗi**, không phải `project_id`; các project này ("Merlink Core") không tồn tại trong bảng `projects` của cả 3 vũ trụ còn lại.
- ID trùng giữa các vũ trụ: `c-1` vừa là chapter trong `mockProjects` vừa là character `Phù Thanh Cơ` trong draft. Nếu nối API mà không đổi ID sẽ va chạm khi normalize store.
- `/character/char-1` (Kaelen) và `/character/c-1` (Phù Thanh Cơ) đều render **Rimuru** vì `CharacterDetailLayout` đọc `useCharacterDetailStore` — store này khởi tạo cứng bằng `rimuruProfile` ([store.ts:37](../../src/core/components/character-detail/store.ts#L37)) và không có action `loadById`.

---

## 3. Đối chiếu chi tiết theo phân vùng

### 3.1. `SYSTEM_PROMPT.MOTIF` — ✅ gần như 1:1

| Bảng DBML | Type UI | Kết quả |
|---|---|---|
| `motifs` | `Motif` | ✅ giữ `display_name`; UI map `Motif.name` → `displayName`. `minChapter`/`maxChapter` là field UI-only, chưa có cột — hoặc bỏ khỏi UI, hoặc thêm cột |
| `genres` | `Genre` | ✅ giữ `display_name` (UI map `Genre.name`) |
| `motif_applicable_genres` | `MotifApplicableGenre` | ✅ |
| `components` | `Component` | ✅ (lệch `display_name`) |
| `states` | `StateItem` | ✅ |
| `state_rules` | `StateRule` | ✅ |
| `state_down_by_rules` | `StateDownByRule` | ✅ (`is_allowed` ↔ `isAllowed`) |
| `emotional_ranges` | `EmotionalRange` | ✅ |
| `event_blocks` | `EventBlock` | ✅ |
| `event_transitions` | `EventTransition` | ✅ |
| `event_block_emotionals` | `EventBlockEmotional` | ✅ |
| `constraints` | `Constraint` | ✅ (`require`/`reason`) |
| `motif_constraints` | `MotifConstraint` | ✅ đã bổ sung `syntax` + `constrain` trên bảng nối (theo Option 1b). `code` unique quy ước `<motif.code>_<constraint.code>` |

Ghi chú: `MotifsTab` + `MotifCanvas` render đầy đủ event block, transition, emotional range, constraint, state mechanics → **vùng này đã đủ dữ liệu để chốt contract API**. `MotifCanvas` tự tính toạ độ node, không cần bảng layout.

### 3.2. `SYSTEM_PROMPT.STYLE_PRESET` — ✅ tốt nhất trong toàn bộ dự án

`style_presets` có 16 cột, `StylePresetsTab` render đúng 16 trường đó (`tonePace`, `toneEmotion`, `visual`, `auditory`, `tactile`, `olfactory`, `gustatory`, `languageAbstract`, `syntaxSentence`, `syntaxVariation`, `syntaxStructure`, `category`, `isSystem`, `code`) → **không cần adapter**.

Sai lệch duy nhất:

| Vấn đề | Chi tiết | Mức |
|---|---|---|
| `usage_type` là `varchar` | DBML ghi `varchar [note: 'primary \| secondary \| avoid']`; UI đã có union `RhetoricalUsage` → nên đổi thành enum | P2 |
| `style_presets` không có `display_name` | UI cũng chỉ hiển thị `code` → nhất quán, giữ nguyên | — |

### 3.3. `SYSTEM_PROMPT.NARRATIVE_DIRECTIVES`

| Bảng | Kết quả |
|---|---|
| `narrator_configs` | ✅ DBML đã thêm `description text` — khớp 27 narrator đang có mô tả trong mock |
| `narrative_directives` | ✅ (`code`, `description`, `category`, `is_system`) |
| `style_preset_narratives` | ✅ (`weight` dùng `NarrativeWeight`) |

`NarrativeDirectivesTab` lọc narrator theo `person` và nhóm directive theo `category` → map thẳng được vào query, không cần bảng phụ.

### 3.4. `DATA.META - THEME`

| Bảng | Kết quả |
|---|---|
| `themes` | ✅ **Theme chỉ gắn với arc.** Đã bỏ `project_id` khỏi DBML: `themes.arc_id` → `arcs.project_id` → `projects`. UI `Theme.scope: 'PROJECT' \| 'ARC'` + `projectId` phải bỏ; theme cấp project = hợp các theme của mọi arc thuộc project (`JOIN arcs`). |
| `theme_arguments` | ✅ khớp đủ: `theme_id`, `character_id`, `role`, `answer_mode`, `note`, `required_knowledge`, `required_competence`. UI thêm `characterName` (denormalize để hiển thị) — chấp nhận được cho read model |

Lệch ở tầng UI: draft dùng **nhãn tiếng Việt** thay vì enum code — `role: 'Chính' | 'Phản đề'` và `reactionType: 'Hành động' | 'Lời nói'` ([mockOverview.ts:57-70](../../src/core/components/draft/mockOverview.ts#L57-L70)), rồi `ThemeBlock` mới map ngược sang nhãn (`'Chính' ? primary : 'Phản đề' ? antithesis`) ([ThemeBlock.tsx:84-87](../../src/core/components/draft/overview/ThemeBlock.tsx#L84-L87)). Đây là **hard-code i18n vào dữ liệu**, phải thay bằng `PRIMARY/ANTITHESIS/…` + `ACTION/VERBAL/…`.

### 3.5. `DATA.CONTENT` — projects / arcs / chapters / scenes

#### `projects`

| DBML (hiện tại) | UI `ProjectItem` | Kết quả |
|---|---|---|
| `id uuid [pk]` | `id` | ✅ |
| `code varchar [unique]` | — | ⚠️ **`code` chỉ dùng cho truy vấn ngữ nghĩa + làm khoá i18n; không hiển thị trên UI.** UI không cần field này, nhưng nên dùng `code` cho deep-link thay vì `id` |
| `display_name text` | `title` | ⚠️ rename `title` → `displayName` |
| `summary text` | `description` | ⚠️ rename `description` → `summary` |
| `cover varchar` | `cover: string` (import SVG) | ✅ đã có cột |
| `status ContentStatus` | `status: 'Drafting' \| 'Completed' \| 'In Review' \| 'Ongoing' \| 'Planning'` | ✅ enum mới `PLANNING / DRAFTING / ONGOING / IN_REVIEW / COMPLETED` khớp đúng 5 giá trị UI |
| `language Language` | — (chỉ `DraftOverview.language` có) | ⚠️ phải thêm `language` vào `ProjectItem` |
| `created_at`, `updated_at` | `modifiedAt: "2026-10-24"` | ⚠️ phải là timestamp ISO, không phải chuỗi ngày; format ở view |
| — | `details: { wordCount, chapters, characters, worldLore }` | ✅ **detail là view** — không cần cột. Tính bằng query aggregate (xem §6.2(d)) |
| — | `genres: string[]` | ❌ phải qua bảng nối (xem dưới) |

#### `project_genres` (bảng nối tiêm thể loại vào project)

DBML: `project_id uuid`, `genre_id uuid`, `is_primary boolean`, unique `(project_id, genre_id)`.
UI hiện tại: `DraftGenre { name: string; isPrimary: boolean }` ([`draft/types.ts:18-21`](../../src/core/components/draft/types.ts#L18-L21)) và `ProjectItem.genres: string[]` ([`project/types.ts:30`](../../src/core/components/project/types.ts#L30)) — lưu **tên**, không có primary.

→ UI phải đổi thành `genres: { genreId, code, displayName, isPrimary }[]`; `code` ẩn, `displayName` lấy theo ngôn ngữ.

> ✅ **Đã chốt (Option 2a)**: Dùng luôn bảng nối `project_genres` sẵn có (`project_id`, `genre_id`, `is_primary`) để tiêm thể loại vào project, không thêm bảng mới. UI cần bỏ mảng chuỗi `genres: string[]` và chuẩn hoá sang bảng nối này.

#### `arcs` / `arc_motifs` / `arc_event_blocks` — theo DBML

UI phải theo DBML (không thêm cột cho UI):

| DBML | UI hiện tại (đường dẫn) | Việc cần làm ở UI |
|---|---|---|
| `arcs(code, display_name, project_id, position, summary)` | `ArcItem { id, title, cover?, chapters }` — [`project/types.ts:17-22`](../../src/core/components/project/types.ts#L17-L22) | `title` → `displayName`; `code` ẩn; thêm `position`, `summary`; **bỏ `cover`** (dùng `projects.cover`) |
| `arc_motifs(arc_id unique, motif_id, note)` | `DraftArc.motif { code, displayName, category, note }` — [`draft/types.ts:68-73`](../../src/core/components/draft/types.ts#L68-L73) | `code/displayName/category` là dữ liệu join từ `motifs` → UI chỉ giữ `motifId` + `note` |
| `arc_event_blocks(arc_motif_id, event_block_id, start_chapter_id, end_chapter_id, summary)` | `DraftSegment { chapterRange: 'Chương 1 - 3', phaseNumber, summary }` — [`draft/types.ts:23-28`](../../src/core/components/draft/types.ts#L23-L28) | thay `chapterRange` (chuỗi) bằng `startChapterId`/`endChapterId`; thêm `eventBlockId`; `phaseNumber` suy ra từ thứ tự |
| — | `DraftArc.segmentsCount` — [`draft/types.ts:66`](../../src/core/components/draft/types.ts#L66) | bỏ — đếm `segments.length` |

#### `chapters` — theo DBML

| DBML | UI `ChapterItem` ([`project/types.ts:9-15`](../../src/core/components/project/types.ts#L9-L15)) | Việc cần làm ở UI |
|---|---|---|
| `id`, `code`, `display_name`, `arc_id`, `position`, `summary` | `id`, `title`, `date`, `isNew?`, `blocks?` | `title` → `displayName`; `code` ẩn (chỉ semantic); thêm `position`, `summary` |
| `created_at`, `updated_at` | `date: "18/04/2026"` | **bỏ `date` khỏi model** — suy từ `created_at`, format ở view |
| — | `isNew?: boolean` | **bỏ** — không cần nữa |
| `scenes` (N-1) | `blocks: { id, text, locked }[]` | xem bảng scene bên dưới |

#### `scenes` — UI đang khác ở đâu, field nào

| DBML `scenes` | UI field hiện tại (đường dẫn) | Khác thế nào / việc cần làm |
|---|---|---|
| `id`, `code`, `display_name` | `ChapterBlock.id` — [`project/types.ts:3-7`](../../src/core/components/project/types.ts#L3-L7); `DraftScene.code`+`displayName` — [`draft/types.ts:47-52`](../../src/core/components/draft/types.ts#L47-L52) | UI đang dùng `id` dạng `"blk-1"`. `code` (`sc-1`) chỉ để truy vấn + i18n, **không render** |
| `content` | `ChapterBlock.text` — [`project/types.ts:5`](../../src/core/components/project/types.ts#L5) | `text` ↔ `content`; UI **không có** `summary` |
| `is_locked` *(đã fix)* | `ChapterBlock.locked` — [`project/types.ts:6`](../../src/core/components/project/types.ts#L6) | ✅ Khớp 1:1. Khóa sửa tay / chống AI ghi đè khi regenerate lại nội dung |
| `position` | `DraftScene.position` | ✅ |
| `word_count` | — | UI không hiển thị; dùng cho `details.wordCount` (view) |
| `chapter_id` | ẩn — UI lồng `arcs[].chapters[].blocks` | không có field tương ứng; phải phẳng hoá hoặc giữ FK khi gọi API |
| `style_preset_id`, `narrator_config_id` | `SceneConfig.stylePresetCode`, `narratorConfigCode` — [`draft/types.ts:170-171`](../../src/core/components/draft/types.ts#L170-L171) | UI lưu **`code`** thay vì `id`; nếu API nhận code thì phải khai báo rõ đây là "semantic ref", không phải FK |
| `world_setting_id` | `SceneConfig.worldSettingCode` / `worldSettingId` — [`draft/types.ts:168-169`](../../src/core/components/draft/types.ts#L168-L169) | ✅ Sinh ra từ bước 3 (World building), gắn vào scene ở bước 4 (Scene config). Đã thêm `world_settings.project_id` vào DBML và `worldSettingId`/`worldSettingCode` vào UI |
| `scene_lore_entries` (N-N) | `SceneConfig.loreEntriesCode` — [`draft/types.ts:172`](../../src/core/components/draft/types.ts#L172) | 1 chuỗi → mảng `{ loreId, usage, revealToReader, revealToPov }[]` (P1-05) |
| `scene_characters` | `SceneCharacter { characterId?, characterCode?, displayName, isPov }` — [`draft/types.ts:158-163`](../../src/core/components/draft/types.ts#L158-L163) | ✅ Đã bổ sung `characterId` & `characterCode` dùng cho deep-link và query ngữ nghĩa; `displayName` dùng để hiển thị trên UI |
| `scene_versions` | — | UI chưa có; `DRAFT_OVERVIEW.versionHistory` là cấp **project**, không phải scene |

**Trùng lặp trong chính draft mock**: scene xuất hiện 2 lần trong cùng object `DraftOverview`:

- `arcs[].chapters[].scenes[]` → `DraftScene { code, displayName, position, summary }`
- `sceneConfig.arcs[].chapters[].scenes[]` → `SceneConfig { code, displayName, position, summary, stylePresetCode, narratorConfigCode, loreEntriesCode, characters[] }`

Hai bản này phải đồng bộ bằng tay (`sc-1`…`sc-5` khớp nhau trong mock hiện tại), và bản trong `arcs` bị `DraftWorkspace` cắt bỏ khi ở step `overview` ([DraftWorkspace.tsx:108-119](../../src/core/components/draft/DraftWorkspace.tsx#L108-L119)) → dễ sinh lệch dữ liệu.

Ngoài ra `draft/types.ts` trước đây khai báo trùng 2 lần các interface `SceneCharacter`, `SceneConfig`, `ChapterConfig`, `ArcSceneConfig`, `SceneConfigPhase` — nay đã dọn dẹp xong.

#### 3.5.1. Kiểu ID trong schema (đã cập nhật)

```
projects.id           uuid     ✅
arcs.project_id       uuid     ✅ ref → projects.id
project_genres.project_id uuid ✅ ref → projects.id
themes.arc_id         uuid     ✅ ref → arcs.id  (theme không còn project_id)
sessions.id           varchar  ✅ ID do runtime sinh
sessions.project_id   uuid     ✅ ref → projects.id
messages.id           varchar  ✅
messages.session_id   varchar  ✅ ref → sessions.id
```

55 bảng: **53 dùng `uuid` PK**, chỉ `sessions`/`messages` giữ `varchar`. `code` **không phải** khoá — nó là định danh ngữ nghĩa (slug) dùng cho truy vấn + i18n key và không bao giờ hiển thị.

#### 3.5.2. Quy ước `code` / `display_name` và cách hiển thị đa ngôn ngữ

**Bất biến (áp dụng toàn schema):** `code` chỉ dùng cho truy vấn ngữ nghĩa và làm khoá i18n — **không bao giờ render**. Mọi thứ người dùng nhìn thấy đến từ `display_name` (hoặc bản dịch của nó). Quy ước này đã ghi ở đầu [`db.dbml`](../db.dbml).

| Lớp dữ liệu | Quy ước | Ví dụ |
|---|---|---|
| Catalog hệ thống (genres, motifs, states, style presets…) | `code` = slug ổn định; `display_name` = nhãn tiếng Việt mặc định | `genres.code = 'high_fantasy'` · `display_name = 'High Fantasy'` |
| Bản dịch | `i18n key = <bảng>.<code>.<trường>` | `genre.high_fantasy.display_name` → `"High Fantasy"` / `"高空奇幻"` |
| Enum | `note` = nhãn ngắn; key `<Enum>.<VALUE>` | `LoreUsageType.FORESHADOW` → `"Gieo mầm"` |
| Nội dung người dùng viết (projects, chapters, scenes) | `display_name` là **nội dung thật**, không dịch | `projects.display_name = "Tế nữ ngưu nhân"` |

**Luồng hiển thị để UI thống nhất một chỗ:**

```ts
// API luôn trả cặp code + displayName
type Ref = { code: string; displayName: string };

// helper duy nhất — không rải logic i18n khắp component
const label = (t: TFn, table: string, r: Ref) =>
  t(`${table}.${r.code}.display_name`, { defaultValue: r.displayName });
```

- `code` được phép xuất hiện trong response và URL (`/project/chronicles-of-aethelgard`) nhưng **không** render thành text.
- Với enum: `t('LoreUsageType.FORESHADOW')` fallback về `note` trong DBML — vì vậy cả 120 giá trị enum đều có note ngắn.
- `ProjectItem.title`, `ChapterItem.title`, `ChapterBlock` là nội dung tác giả viết → dùng thẳng `display_name`, không tra dictionary.

### 3.6. `sessions` / `messages`

| DBML | UI | Kết quả |
|---|---|---|
| `sessions(id, project_id, display_name, created_at)` | `ChatItem { id, title, time: '21h', project: string }` | ❌ `project` là **tên** project; `time` là chuỗi tương đối đã format; thiếu `created_at` |
| `messages(id, session_id, role, content, payload_json, created_at)` | `ChatMessage { id, role: 'user'\|'assistant', content, thinkingSeconds, thinkingDetails, isThinking, toolCalls[], timestamp }` | ❌ thiếu `session_id`; `thinking*` và `toolCalls` phải nằm trong `payload_json`; `timestamp: 'Vừa xong'` là chuỗi hiển thị, không phải thời gian |

Ghi chú tích cực: `payload_json` là thiết kế đúng cho `toolCalls`/`thinking` (UI đã có `ToolCallItem { name, parameters, result, executionTimeMs, status }` → serialize thẳng vào `payload_json`). Vấn đề chỉ là UI chưa có type "envelope" và chưa có `session_id`.

`messages.role` là `varchar` với note `user, assistant, tool,...` trong khi type UI chỉ có `user | assistant` → cần enum `MessageRole`.

### 3.7. `DATA.META` — characters & appearance

#### `characters`

| DBML | UI `CharacterItem` (list) | UI `CharacterFullProfile` (detail) | Kết quả |
|---|---|---|---|
| `id uuid` | `id: 'char-1'` | `id: 'rimuru-tempest'` | ❌ cả 2 đều không phải UUID; 2 vũ trụ ID khác nhau |
| `code varchar [unique]` | — | — | ❌ không có |
| `display_name` | `name` | `basicInfo.name` | ⚠️ cùng ý nghĩa, 2 đường dẫn khác nhau |
| `birth timestamp` | — | — (draft có `appearance.birth: '10/10/1998'`) | ❌ UI lưu chuỗi ngày sinh, không phải timestamp |
| `species varchar` | `species` | `basicInfo.species` | ✅ |
| `biological_sex gender` | `'Male' \| 'Female' \| 'Intersex' \| 'Unknown'` | `'male' \| 'female' \| 'intersex' \| 'none' \| 'unknown'` | ❌ **enum chỉ có MALE/FEMALE** + casing khác nhau giữa 2 model |
| `gender_identity gender` | `'NONE' \| 'MALE' \| 'FEMALE'` | như trên | ⚠️ khớp giá trị nhưng DBML thiếu `NONE` |
| `gender_expression gender` | như trên | như trên | ⚠️ như trên |

#### `character_appearances`

DBML chuẩn hoá 9 trường ngoại hình về FK `appearance_options` + `height_unit`/`weight_unit`:

```
hair_color_id, hair_length_id, hair_style_id, hair_texture_id,
pupil_shape_id, iris_color_id, sclera_color_id, skin_tone_id, skin_texture_id,
height, height_unit, weight, weight_unit, hair_note, eyes_note, skin_note,
outfit_style, default_summary
```

UI thì **không dùng** mô hình này:

| UI | Dạng | Kết quả |
|---|---|---|
| `CharacterAppearance` (list) | `hairColor: 'Silver Ash'`, `hairHex: '#c0c6cf'`, `eyeColor`, `eyeHex`, `skinTone`, `skinHex` + 3 note + outfit + summary | ❌ mất `hair_length/style/texture`, `pupil_shape`, `sclera`, `skin_texture`; lưu **hex trực tiếp** thay vì trỏ `appearance_options` |
| `Appearance` (detail) | `hair/eyes/skin: { value, hex }` | ❌ phẳng hơn nữa, không có style/texture/length |
| `CharacterAppearance` (draft) | `hair{color,style,texture,length}`, `skin{tone,texture}`, `eyes{pupil{shape,color}, iris{texture,color}, sclera}` | ⚠️ **gần DBML nhất** nhưng vẫn là free text, không phải FK |

→ `appearance_option_groups` và `appearance_options` hiện là **2 bảng 🧟** đối với UI. Đây cũng là lý do UI phải tự khai báo hex ở 3 nơi khác nhau, dễ lệch màu giữa card / detail / draft.

`MeasurementUnit`: DBML gộp đơn vị độ dài và khối lượng vào 1 enum (`MM…KG`), UI dùng `'cm' | 'm' | 'ft' | 'in' | 'kg' | 'lbs'`. Giao = `{CM, M, KG}`; DBML thiếu `FT/IN/LBS`, UI không dùng `MM/HA/KM/ML/L/MG/G`.

#### `character_distinguishing_marks`

DBML: `character_id`, `content text not null` (2 cột).
UI list: `{ id, location: 'Cẳng tay phải', description: '…' }` — **có `location`**.
UI detail: `{ id, content }` — **không có `location`**.
→ 2 model lệch nhau, DBML thiếu `location`.

### 3.8. `relationships` / `relationship_roles` / `relationship_states`

UI gộp 3 bảng thành **1 card**:

```ts
CharacterRelationship {
  id, targetName,                       // targetName ❌ không phải to_character_id
  category: string,                     // ❌ free text
  role: 'superior'|'equal'|'inferior',  // ⚠️ casing
  states: { trust, affection, hostility, fear, respect, familiarity, dependency }
}
```

| DBML | UI | Kết quả |
|---|---|---|
| `relationships(motif_id, from_character_id, to_character_id)` | `targetName` | ❌ không có `motif_id`; `targetName` thay vì FK |
| `relationship_categories(category CategoryGroup, sub_type)` | `category: 'Rivalry / Former Alliance'`, `'Imperial Hierarchy'`, `'Investigative Focus'` | ❌ chuỗi tự do, **không map được** vào `FAMILY/ROMANTIC/SOCIAL/PROFESSIONAL/ANTAGONISTIC`; type `RelationshipCategory` khai báo `'Rival'` — giá trị không tồn tại trong enum DBML |
| `relationship_roles(hierarchy, category_id, is_primary)` | `role`, `category` | ❌ UI không có `is_primary`; hierarchy casing: `'equal'` (list) vs `'Equal'` (detail) |
| `relationship_states(parent_id, scene_id, 7 chiều, created_at)` | `states` (1 snapshot) | ⚠️ UI chỉ hiển thị **trạng thái mới nhất**, DBML là **chuỗi thời gian theo scene** — chấp nhận được nhưng cần documented read model: `relationship detail = relationship + role primary + state mới nhất` |

Giá trị 7 chiều khớp hoàn hảo: `NONE/LOW/MODERATE/HIGH/ABSOLUTE` ↔ `RelationDimensionLevel`.

### 3.9. World building — `world_settings` / `lore_entries`

| DBML | UI `DraftWorldSettings` | Kết quả |
|---|---|---|
| `code`, `display_name`, `description` | `code`, `displayName`, `description` | ✅ |
| `type LocationType` | `type: string` (`'REALM'`) | ⚠️ UI khai báo `string` thay vì union 9 giá trị → không có validate |
| `parent_id uuid` | `parentCode?: string` | ❌ **tham chiếu bằng code thay vì id** |
| `access LocationAccess` | `access: 'Tự do đi lại (ngoài trừ khu vực Tế Đàn bị phong tỏa)'` | ❌ enum bị thay bằng câu văn |
| `spatial_features`, `sensory_visual/auditory/tactile/olfactory/gustatory` | — | ❌ UI không hiển thị (6 trường bị bỏ) |
| `created_at`, `updated_at` | — | ❌ |
| — | `loreEntries: DraftLoreEntry[]` (lồng con) | ⚠️ DBML là bảng **độc lập** có `world_setting_id` → UI đang mô hình hoá sai quan hệ (1 world chứa N lore là đúng về ý niệm, nhưng aggregate trong cùng object làm mất khả năng dùng lore ở nơi khác) |

`lore_entries`:

| DBML | UI `DraftLoreEntry` | Kết quả |
|---|---|---|
| `code`, `display_name`, `type`, `description`, `world_setting_id` | `code`, `displayName`, `type`, `description`, (lồng trong world) | ✅ về giá trị, ⚠️ về quan hệ |
| `data json` | `data: string` | ⚠️ UI dùng string (`'Có thể phát sáng khi đến gần khu vực tế đàn.'`) thay vì JSON có cấu trúc |

**UI chưa có chỗ cho domain này**: `/worldbuilding/lore` và `/worldbuilding/settings` đều là trang "Under Construction"; cây không gian `realm > world > region > country > city > town > facility > street > room` (9 cấp, tự tham chiếu) **chưa có UI duyệt cây**. Hiện chỉ thấy 1 world duy nhất (`ws-1`) trong draft preview.

### 3.10. Memory — `character_memories`

Đây là khoảng cách lớn nhất giữa UI và schema.

DBML (8 dòng, note *"Kho ký ức phẳng của nhân vật"*):

```
character_memories(id, character_id, source_scene_id, content, created_at)
```

UI `Memory` (14 trường):

```
id, summary, type, status, recollection, interpretation, certainty, knowledgeSource,
occurredAt, perceivedAt, references[], emotions[], connections[], revisions[]
```

| UI field | DBML | Kết quả |
|---|---|---|
| `content` / `recollection` / `summary` | `content` | ⚠️ gộp được, nhưng UI tách 3 ý (tiêu đề, nội dung, diễn giải) |
| `type: 'Episodic'\|'Semantic'\|'Procedural'` | enum `MemoryType` 🧟 | ❌ enum có nhưng **cột không có** |
| `status: 'Active'\|'Suppressed'\|'Forgotten'\|'Distorted'` | enum `MemoryStatus` 🧟 | ❌ |
| `certainty: 'Low'\|'Medium'\|'High'` | enum `MemoryCertainty` = `CERTAIN\|PROBABLE\|UNCERTAIN` 🧟 | ❌ **khác ngữ nghĩa**, không chỉ khác nhãn |
| `knowledgeSource: 'Personal experience'` | enum `MemorySource` = `FIRSTHAND\|HEARSAY\|INFERENCE` 🧟 | ❌ |
| `occurredAt`, `perceivedAt` | — | ❌ thiếu cả 2 cột (rất quan trọng cho memory bị bóp méo) |
| `references: { role, name }[]` | — | ❌ cần bảng `memory_references` |
| `emotions: { name, intensity: 0-100 }[]` | — | ❌ cần bảng `memory_emotions` (hoặc dùng `emotional_ranges` + cột `intensity`) |
| `connections: { label, targetMemoryId }[]` | — | ❌ cần bảng `memory_connections` |
| `revisions: { revisionNumber, reason, change }[]` | — | ❌ cần bảng `memory_revisions` |
| — | `source_scene_id` | ✅ UI không có nhưng đây là cột tốt |

Ngoài ra UI còn khái niệm **Knowledge** (`KnowledgeSource { type: 'Initial'|'Lore', path, certainty }`) — không có bảng nào trong DBML. Đây là loại dữ liệu khác với memory (kiến thức nhân vật *sở hữu*), cần bảng `character_knowledges` hoặc map sang `lore_entries` + trạng thái biết/không biết.

### 3.11. Competence

| DBML | UI `Competence` | Kết quả |
|---|---|---|
| `character_competences(character_id, domain_id, scene_id [not null], display_name, description)` | `{ name, domain: string, level: string, reliability, description, experience }` | ❌ thiếu `level`, `reliability`, `experience`; `domain` là text thay vì FK |
| `competence_domains(code, display_name, description, is_system)` | `domain: 'Unique Skill'`, `'Kiếm thuật'` | ⚠️ dùng text, không phải FK |
| `domain_level(competence_domain_id, code, display_name, description)` | `level: 'Ultimate'` | ❌ **bảng `domain_level` không được bảng nào tham chiếu** (🧟); UI cũng chỉ lưu text |
| `competence_capabilities / competence_limitations(content)` | `capabilities[]`, `limitations[]` | ✅ map tốt (mỗi item = 1 row) |
| `scene_id not null` | `sceneLearned: 'Chương 3'` | ❌ UI ghi **chương**, DBML bắt buộc **scene** và `not null` → mọi năng lực phải gắn scene, kể cả năng lực bẩm sinh |

`reliability` dùng chung `KnowledgeCertainty` (Low/Medium/High) — không có cột trong DBML.

### 3.12. `VFILL` — `character_motifs`, `symbolic_motifs`, `symbolics`

| Bảng | UI | Kết quả |
|---|---|---|
| `character_motifs(motif_id, character_id, description)` | `DraftCharacter.motif: { description: 'strong girl' }` | ❌ UI chỉ giữ `description`, **không có `motif_id`/`code`** → không thể ghi vào bảng; `biological_sex` + `motif_code` là 2 trường bắt buộc theo [bussiness-flow.md](../bussiness-flow.md#L42-L53) nhưng draft mock không có `motif.code` |
| `symbolic_motifs(symbolic_id, motif_id)` | — | 🧟 không có UI |
| `symbolics(id)` | — | 🧟 bảng rỗng nghĩa (chỉ có PK) |

### 3.13. Ngoài schema — Settings / Recycle Bin / AI Provider

| UI | Dữ liệu | DBML |
|---|---|---|
| `GeneralTab` (theme, locale) | local (zustand) | — (đúng, không cần DB) |
| `AIProvidersTab` + `ProviderForm` | `AIProviderItem { name, status, isCustom, providerType, apiKey, baseUrl, apiProtocol, models[] }`, `ModelConfig { contextWindow, maxOutputTokens, inputTypes{text,image}, reasoning }` | ❌ không có bảng. Nếu cần persist: `ai_providers`, `model_configs`; **không lưu `apiKey` plaintext** |
| `RecycleBinTab` | `DeletedItem { id, name, type: 'PROJECT'\|'CHARACTER', deletedAt, details }` | ❌ không bảng nào có `deleted_at` → soft-delete chưa được thiết kế |

---

## 4. Lỗi nội tại của `docs/db.dbml`

Các vấn đề dưới đây **không liên quan UI** — schema tự mâu thuẫn, nên sửa trước khi sinh migration.

> **Cập nhật lần 4 — 2026-10-29.** Đã đối chiếu lại [`db.dbml`](../db.dbml): **1.071 dòng · 55 bảng · 25 enum**, 120/120 giá trị enum có `note` là nhãn ngắn dùng cho i18n. Bảng dưới đây là *trạng thái thực tế*, không còn là danh sách đề xuất.
> Đợt 3 giải quyết nốt S-07 theo hướng **Nhóm 1** (§4.1).
>
> ✅ đã xử lý xong

| ID | Vấn đề (bản gốc) | Trạng thái | Đã làm / còn lại |
|---|---|---|---|
| S-01 | `projects.id` varchar vs `uuid` refs | ✅ | `projects.id` → `uuid`; `sessions.project_id` → `uuid`; `sessions.id` & `messages.id` giữ **`varchar`** (ID do runtime sinh, không phải UUID) nên `messages.session_id varchar` là hợp lệ. Bản sửa trung gian đã đổi `sessions.id`/`messages.id` sang `uuid` nhưng để `sessions.project_id` varchar → sinh 2 mismatch mới; đã revert theo recommend *“uuid cho projects, varchar cho session/message”* |
| S-02 | 6 enum 🧟 không bảng nào dùng | ✅ | Đã **xoá** cả 6 (`ContentStatus`, `SceneStatus`, `MemoryType`, `MemoryCertainty`, `MemorySource`, `MemoryStatus`) — 28 → 22 enum (nay 24 sau khi thêm 2 enum ở S-07). **Hệ quả cần biết**: `projects` và `scenes` nay không còn cột `status` ⇒ vòng đời project/scene chưa biểu diễn được. Nhu cầu này chuyển sang mục 5 (P0-02, P1-02); nếu muốn dùng lại thì phải thêm enum **kèm cột sử dụng**, không để enum mồ côi |
| S-03 | `domain_level` không được bảng nào tham chiếu | ✅ | `character_competences.level_id uuid [ref: > domain_level.id]`; đồng thời `domain_level.code` bỏ unique toàn cục → unique theo `(competence_domain_id, code)` để `BEGINNER` dùng được ở nhiều domain |
| S-04 | `symbolics` chỉ có `id`; `symbolic_motifs` không unique | ✅ | `symbolics` có thêm `code`, `display_name`, `description`, `is_system`; `symbolic_motifs` thêm unique `(symbolic_id, motif_id)` + index `motif_id` |
| S-05 | `themes` thiếu `project_id` | ✅ | **Đổi hướng theo chốt của bạn**: theme **chỉ gắn với arc** — đã bỏ `project_id` (không thêm), `arc_id` là quan hệ duy nhất, project suy ra qua `arcs.project_id`. Ghi chú cũ "project -> arc" cũng đã bỏ |
| S-06 | `relationship_states.parent_id` đặt tên sai | ✅ | Đã đổi thành `relationship_id` |
| S-07 | `scenes.lore_entries` là 1 uuid thay vì N-N | ✅ | Đã xoá cột `scenes.lore_entries`; thêm bảng nối `scene_lore_entries` + 2 enum (`LoreUsageType`, `RevealDepth`). Áp dụng **Nhóm 1** (3 thuộc tính) — chi tiết + backlog Nhóm 2/3 ở §4.1 |
| S-08 | `relationship_categories` unique `sub_type` toàn cục, thiếu `code` | ✅ | Thêm `code varchar [unique]`; `sub_type` hết unique toàn cục → unique `(category, sub_type)` |
| S-09 | `SceneStatus.DRAFF` typo | ✅ | Hết hiệu lực vì `SceneStatus` đã bị xoá ở S-02 |
| S-10 | *(đính chính)* `character_competences` index `(character_id, domain_id)` | ✅ | **Đánh giá ban đầu của tôi sai**: cặp này **không nên** unique vì một nhân vật có nhiều năng lực cùng domain (Rimuru: Predator / Great Sage / Universal Shapeshift / Gluttony đều `Ultimate Skill`). Đã giữ index non-unique + comment giải thích, và thêm `code varchar [unique]` để định danh năng lực |
| S-11 | `chapters` không có timestamp | ✅ | Đã có `created_at` / `updated_at`. Ghi chú: phần *“thiếu `project_id`”* **không phải lỗi** — suy ra qua `arcs` là chuẩn hoá đúng |
| S-12 | `arc_motifs.id`, `arc_event_blocks.id` thiếu note `UUIDv7` | ✅ | Đã thêm — giờ toàn bộ 53 bảng dùng `uuid` PK đều có note |
| S-13 | `motif_constraints.code` tự ghép chuỗi | ✅ | Giữ cột, thêm note quy ước `<motif.code>_<constraint.code>` để seed/import không lệch; đã bổ sung `syntax` và `constrain` cho bảng nối (Option 1b) |

**Kết quả**: **13/13 mục đã đóng**. Schema không còn FK lệch kiểu, không còn enum/bảng mồ côi, và quan hệ scene ↔ lore đã là N-N có ngữ nghĩa.

### 4.1. S-07 — `scene_lore_entries` (đã áp dụng Nhóm 1)

**Vấn đề ban đầu**: `scenes.lore_entries uuid [ref: > lore_entries.id]` — một scene chỉ trỏ được **một** lore, trong khi thực tế một cảnh thường dùng 0–5 lore (bối cảnh + phe phái + vật phẩm + điều cấm). Tên cột số nhiều nhưng kiểu là 1 giá trị.

**Điểm cốt lõi**: bảng nối **không nên** chỉ có `(scene_id, lore_entry_id)`. Với chỉ 2 khoá ngoài, nó chỉ trả lời *“cảnh này có nhắc lore Y”* — đúng thứ DB đã làm được, nhưng không giúp gì cho việc kể chuyện. Giá trị nằm ở chỗ mô tả **lore đó đang làm nhiệm vụ gì trong cảnh**.

**Đã chốt (đợt 3): áp dụng Nhóm 1.** `scenes.lore_entries` đã bị xoá; thêm bảng nối + 2 enum:

```dbml
enum LoreUsageType {
    SETTING     [note: 'Dựng bối cảnh / không khí']
    REFERENCE   [note: 'Nhắc tới thoáng qua']
    FORESHADOW  [note: 'Gieo mầm cho tình tiết sau']
    PAYOFF      [note: 'Trả nợ một FORESHADOW trước đó']
    REVEAL      [note: 'Tiết lộ thông tin mới']
    CONFLICT    [note: 'Là nguyên nhân / xung đột của cảnh']
    CONSTRAINT  [note: 'Ràng buộc hành vi: luật, điều cấm, giới hạn sức mạnh']
    RED_HERRING [note: 'Đánh lạc hướng người đọc']
}

enum RevealDepth { NONE HINT PARTIAL FULL }

Table scene_lore_entries {
    id uuid [pk, note: 'UUIDv7']
    scene_id uuid [ref: > scenes.id, not null]
    lore_entry_id uuid [ref: > lore_entries.id, not null]

    usage LoreUsageType [default: 'REFERENCE']
    reveal_to_reader RevealDepth [default: 'PARTIAL']
    reveal_to_pov RevealDepth [default: 'NONE']

    indexes {
        (scene_id, lore_entry_id) [unique]
        lore_entry_id
        usage
    }
}
```

Nhóm 2 và Nhóm 3 **chưa** đưa vào — giữ làm backlog; cả hai đều là cột thêm vào bảng đã tồn tại nên bổ sung sau bằng migration không phá vỡ dữ liệu.

#### Nhóm 1 — ✅ đã áp dụng (3 thuộc tính)

| Thuộc tính | Kiểu đề xuất | Vì sao cần cho kể chuyện |
|---|---|---|
| `usage` | enum `LoreUsageType` | Phân biệt lore đang **dựng bối cảnh**, **gieo mầm**, **trả nợ**, **tiết lộ**, **gây xung đột**, **ràng buộc hành vi**, hay **đánh lạc hướng**. Đây là thứ biến “danh sách tham chiếu” thành “dàn cảnh có chủ đích” |
| `reveal_to_reader` | enum `RevealDepth` (NONE/HINT/PARTIAL/FULL) | Kiểm soát lượng thông tin nhả cho **người đọc** theo từng cảnh — chống dump lore |
| `reveal_to_pov` | enum `RevealDepth` | Kiểm soát lượng thông tin **nhân vật POV** biết. `reader > pov` = **dramatic irony**; `pov > reader` = bí mật nội tâm. Thiếu cặp này thì AI rất dễ cho POV “biết tuốt” |

Ba thuộc tính này đủ để chạy 3 kiểm tra tự động mà hiện tại không thể:

1. *Lore nào đã gieo (`FORESHADOW`) mà chưa có `PAYOFF`?* → lỗi tuyến truyện.
2. *Cảnh nào nhả `FULL` quá sớm?* → lỗi nhịp exposition.
3. *Cảnh nào `reveal_to_pov = NONE` nhưng lời thoại vẫn giải thích lore?* → lỗi POV.

#### Nhóm 2 — chưa áp dụng (3 thuộc tính)

| Thuộc tính | Kiểu | Vì sao |
|---|---|---|
| `prominence` | `NarrativeWeight` (tái dùng enum sẵn có) | Lore xuất hiện **thoáng qua** hay là **trọng tâm** cảnh. Dùng để cân bằng mật độ chi tiết và quyết định cảnh có cần viết mô tả riêng không |
| `is_required` | boolean | Cảnh **bắt buộc** phải dùng lore này (vd cảnh phá phong ấn buộc nhắc “Thánh tích Ngưu Nhân”) hay chỉ tùy chọn. Ảnh hưởng trực tiếp tới prompt generate |
| `note` | text | Hướng dẫn cụ thể cho AI: *“chỉ nhắc tên, không giải thích”*, *“nhân vật kể sai để che giấu”*. Thay cho `loreEntriesCode` dạng chuỗi trong UI draft |

#### Nhóm 3 — chưa áp dụng (4 thuộc tính)

| Thuộc tính | Kiểu | Vì sao |
|---|---|---|
| `pays_off_id` | self-FK → `scene_lore_entries.id` | Dòng `PAYOFF` trỏ về dòng `FORESHADOW` tương ứng → biến foreshadow/payoff từ quy ước trong đầu thành **ràng buộc kiểm tra được** |
| `pov_character_id` | FK → `characters.id` | Chốt POV của cảnh để đánh giá `reveal_to_pov` (vì `scene_characters.is_pov` có thể nhiều hơn 1) |
| `narrated_by_character_id` | FK → `characters.id` | Nếu lore được kể qua **lời thoại**: ai là người kể → truy vấn *“lore này được kể bởi ai, ở đâu”* và phát hiện nhân vật nói điều mình không thể biết |
| `position` | int | Thứ tự xuất hiện khi một cảnh có nhiều lore — cần để dựng lại mạch cảnh |

#### Nhóm 4 — cân nhắc nhưng **không** khuyến nghị

- `canon_status` (CANON/VARIANT/CONTRADICTS): hấp dẫn nhưng trùng phần lớn với `reveal_to_reader` + `note`; chỉ thêm nếu thực sự có quy trình “dị bản lore”.
- Gắn bảng nối vào `scene_versions`: mỗi lần sửa cảnh phải clone cả tập lore → phức tạp, trong khi `scene_versions` chưa có UI.
- `created_at` trên bảng nối: chỉ hữu ích cho audit, không giúp kể chuyện.

#### Bản đầy đủ Nhóm 1+2+3 (chỉ tham chiếu — chưa áp dụng hết)

```dbml
enum LoreUsageType {
  SETTING      [note: 'Dựng bối cảnh / không khí']
  REFERENCE    [note: 'Nhắc tới thoáng qua']
  FORESHADOW   [note: 'Gieo mầm cho tình tiết sau']
  PAYOFF       [note: 'Trả nợ một FORESHADOW trước đó']
  REVEAL       [note: 'Tiết lộ thông tin mới']
  CONFLICT     [note: 'Là nguyên nhân / xung đột của cảnh']
  CONSTRAINT   [note: 'Ràng buộc hành vi: luật, điều cấm, giới hạn sức mạnh']
  RED_HERRING  [note: 'Đánh lạc hướng người đọc']
}

enum RevealDepth { NONE HINT PARTIAL FULL }

Table scene_lore_entries {
  id uuid [pk, note: 'UUIDv7']
  scene_id uuid [ref: > scenes.id, not null]
  lore_entry_id uuid [ref: > lore_entries.id, not null]

  usage LoreUsageType [default: 'REFERENCE']
  reveal_to_reader RevealDepth [default: 'PARTIAL']
  reveal_to_pov RevealDepth [default: 'NONE']

  prominence NarrativeWeight [default: 'MEDIUM']
  is_required boolean [default: false]
  position int

  pays_off_id uuid [ref: > scene_lore_entries.id, note: 'PAYOFF trỏ về FORESHADOW']
  pov_character_id uuid [ref: > characters.id]
  narrated_by_character_id uuid [ref: > characters.id]

  note text

  indexes {
    (scene_id, lore_entry_id) [unique]
    lore_entry_id
    usage
    pays_off_id
  }
}
```

> **Đã làm**: xoá `scenes.lore_entries`, thêm bảng nối với 3 thuộc tính Nhóm 1 (snippet ở trên). **Còn lại (P1-05, phạm vi UI)**: đổi `SceneConfig.loreEntriesCode` (1 chuỗi) thành mảng `sceneLoreEntries: { loreId, usage, revealToReader, revealToPov }[]`.

**Vì sao dừng ở Nhóm 1**: 3 thuộc tính này đã đủ để chạy các kiểm tra foreshadow/payoff và POV — tức là phần *không thể bổ sung sau mà không mất dữ liệu lịch sử*. Nhóm 2/3 chỉ là cột thêm, có thể làm sau khi vận hành thực tế cho thấy cần.

---

## 5. Tổng hợp mức độ ưu tiên

### P0 — phải chốt trước khi viết lớp API (Toàn bộ 7 P0 đã giải quyết xong)

| ID | Nội dung | Nơi sửa |
|---|---|---|
| P0-01 | ~~Hai model nhân vật không tương thích; route detail bỏ qua :characterId~~ — **đã fix (1.1)**: Hợp nhất `CharacterProfile`, `CharacterDetailLayout` nhận `characterId`, store `loadCharacter` tra cứu đúng ID, đầy đủ 4 mock profile | `@/features/characters` |
| P0-02 | ~~`project.details.*` là chuỗi format~~ — **detail là view**, không cần cột; chỉ cần UI format số ở tầng view | `ProjectDetailView` |
| P0-03 | ~~`gender` enum thiếu...~~ — **đã fix (1.2)**: DBML `enum gender` có `NONE, MALE, FEMALE`. UI chỉ dùng 3 giá trị này cho cả 3 trường giới, bỏ `Intersex, Unknown` | `db.dbml`, `@/features/characters` |
| P0-04 | ~~`projects.id varchar` vs `uuid` refs~~ — **đã fix** (S-01) | — |
| P0-05 | ~~`themes` thiếu `project_id`~~ — **chốt: không thêm**, theme chỉ gắn arc (S-05) | — |
| P0-06 | ~~`characters`/`world_settings` không có `project_id`~~ — **đã chốt (1.3 & Vấn đề 2)**: `world_settings` đã có `project_id` (kết nối World building ở Bước 3 vào project); `characters` độc lập tái sử dụng qua `scene_characters` | `db.dbml` |
| P0-07 | ~~`sessions`/`messages`: `project` là tên~~ — **đã fix (3.1)**: `SidebarHistory` lưu `projectId`, chỉ render `projectName` | `@/features/navigation` |

### P1 — lệch cấu trúc, cần quyết định (Đã giải quyết phần lớn)

| ID | Nội dung |
|---|---|
| P1-01 | ~~`chapters` thiếu timestamp~~ — **đã fix** (S-11); `isNew` **bỏ khỏi UI**, `date` suy từ `created_at` |
| P1-02 | ~~"block" ≠ scene & locked~~ — **đã fix**: `scenes.is_locked` khớp `ChapterBlock.locked` 1:1 (khóa sửa tay / chống AI ghi đè) |
| P1-03 | Scene bị lưu 2 lần trong `DraftOverview` (`arcs[].chapters[].scenes` vs `sceneConfig`) |
| P1-04 | ~~`scene_characters` dùng displayName~~ — **đã fix**: bổ sung `characterId` & `characterCode` dùng cho deep-link/query, `displayName` hiển thị |
| P1-05 | ~~`scenes.lore_entries` 1 uuid & UI loreEntriesCode~~ — **đã fix**: schema có `scene_lore_entries` N-N; UI `SceneConfig.loreEntries` đã chuyển thành mảng có `usage`, `revealToReader`, `revealToPov` |
| P1-06 | `scene_versions`/`SceneSource` không có UI; `versionHistory` cấp project không có bảng |
| P1-07 | ~~`character_memories` quá phẳng~~ — **đã fix (2.1)**: UI thu gọn theo DBML phẳng (`id`, `content`, `sourceScene`, `createdAt`), competence bao hàm knowledge |
| P1-08 | ~~UI `certainty` / Knowledge~~ — **đã fix (2.1)**: Knowledge được biểu diễn qua competence (nếu không có capabilities thì là knowledge) |
| P1-09 | ~~`character_competences` thiếu level/reliability/experience~~ — **đã fix**: DBML đã bổ sung `level_id`, `reliability`, `experience`. `scene_id not null` là **chủ ý thiết kế chuẩn** (mọi năng lực/kỹ năng dù bình dị nhất cũng phải gắn cảnh thể hiện) |
| P1-10 | ~~`appearance_options`...~~ — **đã fix (2.2)**: UI bỏ `hairHex`, `eyeHex`, `skinHex`, theo DBML |
| P1-11 | ~~UI gộp `relationships`...~~ — **đã fix (2.3)**: UI dùng đúng `CategoryGroup` và `Hierarchy` theo DBML |
| P1-12 | `world_settings.access` enum bị thay bằng câu văn; 6 trường `spatial/sensory` không có UI; chưa có UI cây không gian |
| P1-13 | `lore_entries.data json` ↔ UI `data: string`; `/worldbuilding/*` là placeholder |
| P1-14 | ~~`project_genres` bị thay bằng string[]~~ — **đã chốt (2a / 3.2)**: Dùng `project_genres` N-N |
| P1-15 | ~~Recycle Bin không có `deleted_at`~~ — **đã fix (3.3)**: Đã thêm `deleted_at timestamp` vào các bảng chính trong DBML |
| P1-16 | ~~Draft theme argument dùng nhãn tiếng Việt~~ — **đã fix**: chuyển thành enum `PRIMARY`, `ANTITHESIS`, `ACTION`, `VERBAL` kèm badge màu và i18n |

### P2 — naming / cosmetic / mock hygiene (12)

| ID | Nội dung |
|---|---|
| P2-01 | `display_name` ↔ `name` ở ~16 bảng (motifs, genres, components, states, state_rules, emotional_ranges, narrator_configs, arcs, chapters, scenes, characters, competence_domains, domain_level, world_settings, lore_entries, projects) |
| P2-02 | Casing enum không thống nhất trong UI (`'equal'` vs `'Equal'`, `'Male'` vs `'none'`) |
| P2-03 | ~~`motifs.minChapter/maxChapter`~~ — **đã fix**: thêm `min_chapter` và `max_chapter` vào `motifs` |
| P2-04 | ~~`narrator_configs` thiếu `description`~~ — **đã fix**: thêm `description text` vào DBML |
| P2-05 | ~~`character_distinguishing_marks` location~~ — **đã fix**: thêm `location varchar` vào DBML |
| P2-06 | `characters.birth` timestamp ↔ UI chuỗi `'10/10/1998'` |
| P2-07 | ~~`MeasurementUnit` thiếu FT/IN/LBS~~ — **đã fix**: đã thêm `FT, IN, LBS` vào enum |
| P2-08 | ~~`usage_type` là varchar~~ — **đã fix**: chuyển sang enum `RhetoricalUsage` (`primary`, `secondary`, `avoid`) |
| P2-09 | ~~`symbolics` stub; `symbolic_motifs` thiếu unique~~ — **đã fix (S-04)**: thêm `code/display_name/description` và unique |
| P2-10 | `/config` read-only + badge "custom" nhưng không có đường tạo custom item |
| P2-11 | ~~14 project dùng chung SAMPLE_ARCS~~ — **đã fix**: mỗi project dùng bản clone `getSampleArcs()` độc lập |
| P2-12 | ~~`draft/types.ts` khai báo trùng 5 interface~~ — **đã dọn dẹp xong** |

---

## 6. Đề xuất

### 6.1. Patch DBML (ưu tiên theo P0 → P1)

> **Ghi chú đợt 3**: các patch dưới đây là **bản gốc của lần 1**. Toàn bộ S-01…S-13 (kể cả `scene_lore_entries` bản Nhóm 1) đã áp dụng vào [`db.dbml`](../db.dbml), với cách làm rút gọn hơn — xem §4 và §4.1. Các mục còn lại (`assets`, `memory_*`, `project_versions`, `deleted_at`) **vẫn là đề xuất**, chưa áp dụng.

**(1) Chốt kiểu ID — dùng uuid cho `projects`, giữ varchar cho session/message**

```dbml
Table projects {
    id uuid [pk, note: 'UUIDv7']
    code varchar [unique, note: 'slug dùng cho URL: chronicles-of-aethelgard']
    display_name text
    summary text
    status ContentStatus [default: 'DRAFT']
    language Language [default: 'VI']
    cover_asset_id uuid [ref: > assets.id]
    word_count int [note: 'cache, recompute từ scenes']
    deleted_at timestamp
    created_at timestamp
    updated_at timestamp
}
```

> `assets` là bảng **mới** cần bổ sung (chưa có trong DBML): `id`, `owner_type` (`project`/`arc`/`character`), `owner_id`, `url`, `mime`, `created_at`.

**(2) Tách enum giới tính**

```dbml
enum BiologicalSex { MALE FEMALE INTERSEX NONE UNKNOWN }
enum GenderIdentity { MALE FEMALE NON_BINARY NONE }

Table characters {
    ...
    biological_sex BiologicalSex
    gender_identity GenderIdentity
    gender_expression GenderIdentity
}
```

**(3) `themes` thêm project scope**

```dbml
Table themes {
    id uuid [pk]
    code varchar [unique]
    project_id uuid [ref: > projects.id, not null]   // thêm
    arc_id uuid [ref: > arcs.id]                      // null = theme cấp project
    parent_theme_id uuid [ref: > themes.id]
    question text [not null]
    resolution_type ThemeResolutionType [default: 'OPEN']
    resolution text
    message text
    ...
}
```

**(4) `chapters` có timestamp + scene có status/lock**

```dbml
Table chapters {
    ...
    created_at timestamp
    updated_at timestamp
}

Table scenes {
    ...
    status SceneStatus [default: 'DRAFF']
    is_locked boolean [default: false, note: 'Khoá sửa tay — tương ứng UI block.locked']
    // bỏ: lore_entries uuid
}

Table scene_lore_entries {
    id uuid [pk]
    scene_id uuid [ref: > scenes.id, not null]
    lore_entry_id uuid [ref: > lore_entries.id, not null]
    indexes { (scene_id, lore_entry_id) [unique] }
}
```

**(5) Mở rộng memory theo đúng UI**

```dbml
Table character_memories {
    id uuid [pk]
    character_id uuid [not null, ref: > characters.id]
    source_scene_id uuid [ref: > scenes.id]
    summary text
    content text [not null]
    interpretation text
    type MemoryType [default: 'EPISODIC']
    status MemoryStatus [default: 'ACTIVE']
    certainty MemoryCertainty [default: 'PROBABLE']
    source MemorySource [default: 'FIRSTHAND']
    occurred_at timestamp [note: 'thời điểm xảy ra trong truyện']
    perceived_at timestamp [note: 'thời điểm chủ thể ghi nhận']
    created_at timestamp
}

Table memory_emotions {
    id uuid [pk]
    memory_id uuid [ref: > character_memories.id, not null]
    emotional_range_id uuid [ref: > emotional_ranges.id]
    intensity int [note: '0-100']
    indexes { (memory_id, emotional_range_id) [unique] }
}

Table memory_references {
    id uuid [pk]
    memory_id uuid [ref: > character_memories.id, not null]
    character_id uuid [ref: > characters.id]
    role varchar [note: 'friend, enemy, unknown…']
    name text
}

Table memory_connections {
    id uuid [pk]
    memory_id uuid [ref: > character_memories.id, not null]
    target_memory_id uuid [ref: > character_memories.id, not null]
    label varchar
    indexes { (memory_id, target_memory_id) [unique] }
}

Table memory_revisions {
    id uuid [pk]
    memory_id uuid [ref: > character_memories.id, not null]
    revision_number int
    reason text
    change text
    created_at timestamp
    indexes { (memory_id, revision_number) [unique] }
}
```

**(6) Version cho nội dung (thay `versionHistory` cấp project)**

```dbml
Table project_versions {
    id uuid [pk]
    project_id uuid [ref: > projects.id, not null]
    version_number int
    phase varchar [note: 'overview | chapter | world | scene | character']
    snapshot json
    created_at timestamp
    indexes { (project_id, version_number) [unique] }
}
```

**(7) Soft delete dùng chung**

Thêm `deleted_at timestamp` + index cho: `projects`, `characters`, `arcs`, `chapters`, `scenes`, `world_settings`, `lore_entries`, `themes`.

**(8) Sửa các lỗi nhỏ**

- `relationship_states.parent_id` → `relationship_id`
- `scenes.lore_entries` → xoá (đã thay bằng bảng nối)
- `SceneStatus.DRAFF` → `DRAFT`
- `symbolics` → bổ sung `code`, `display_name`, `description`, `is_system`; hoặc xoá nếu chưa dùng
- `character_competences` → thêm `level_id uuid [ref: > domain_level.id]`, `reliability`, `experience`; đổi `scene_id` thành nullable
- `narrator_configs` → thêm `description text`
- `motifs` → thêm `min_chapter int`, `max_chapter int` (hoặc bỏ khỏi UI)
- `style_preset_rhetorical_devices.usage_type` → enum `RhetoricalUsage { primary secondary avoid }`

### 6.2. Kiến trúc mới của `src/` (Feature-Sliced Design / Bulletproof React)

Đã tái cấu trúc toàn diện `src/` thành các module độc lập, loại bỏ hoàn toàn `src/core/`:

```
src/
├── app/                    # Khởi tạo ứng dụng & routing wrapper
│   ├── layouts/            # FullLayout
│   ├── providers/          # ThemeProvider, I18nProvider, AppProviders
│   ├── router/             # Route config / helpers
│   └── routes/             # Route modules React Router v7
├── features/               # Mỗi feature là 1 module độc lập (components, stores, types)
│   ├── characters/         # Model hợp nhất CharacterProfile, loadById, tabs, cards
│   ├── chat/               # ChatView, popovers, input bar, message item
│   ├── draft/              # DraftWorkspace, overview blocks, steps, mockOverview
│   ├── navigation/         # Sidebar, SidebarNav, SidebarHistory (lưu projectId), SettingsDialog
│   ├── projects/           # ProjectView, ProjectDetailView, ChapterDetailView (is_locked)
│   ├── settings/           # GeneralTab, AIProvidersTab, ProviderForm, RecycleBinTab (deletedAt)
│   ├── system-prompt/      # SystemPromptLayout, canvas, tabs
│   └── worldbuilding/      # WorldbuildingView
├── pages/                  # Ghép feature thành màn hình mỏng (mỏng)
│   ├── HomePage.tsx
│   ├── ChatPage.tsx
│   ├── ProjectListPage.tsx
│   ├── ProjectDetailPage.tsx
│   ├── ChapterDetailPage.tsx
│   ├── CharacterListPage.tsx
│   ├── CharacterDetailPage.tsx
│   ├── WorldbuildingPage.tsx
│   └── ConfigPage.tsx
├── shared/                 # Dùng chung, không chứa business logic
│   ├── assets/             # Book covers, avatar images
│   ├── components/         # ui (design system primitives), background
│   ├── hooks/              # useMediaQuery, useResizablePanel, useTheme
│   ├── i18n/               # dict, store, useT
│   └── utils/              # cn, string
└── config/                 # app config, constants
```

**(a) Một model nhân vật duy nhất.** Hợp nhất thành `CharacterProfile` tại `@/features/characters/types.ts`. Giữ đầy đủ `basicInfo`, `appearance` (đã bỏ hex), `distinguishingMarks`, `relationships` (theo DBML), `competences` và `memories` phẳng.

**(b) Sửa route detail.** `CharacterDetailPage` và `CharacterDetailLayout` nhận `characterId` từ URL `useParams().characterId`, tự động load đúng profile qua `useCharacterStore.loadCharacter(id)`.

**(c) Quy ước mapping.**
- `display_name` ↔ `displayName` (hoặc `title` cho projects/chapters).
- `is_locked` ↔ `locked` (trên `ChapterBlock`).
- `projectId` lưu mã thực tế, chỉ render `projectName` trên UI.

| DBML | UI |
|---|---|
| `display_name` | `displayName` (thay cho `name`/`title`) |
| `summary` | `summary` (thay cho `description`) |
| `created_at` / `updated_at` | `createdAt` / `updatedAt` (**Date**, không phải string) |
| `is_system` | `isSystem` |
| `is_allowed` | `isAllowed` |
| `is_primary` | `isPrimary` |

**(d) Số liệu phải là số.** `ProjectDetails` nên là:

```ts
details: {
  wordCount: number;      // sum(scenes.word_count)
  chapterCount: number;   // count(chapters)
  characterCount: number; // count(distinct scene_characters.character_id)
  loreCount: number;      // count(distinct scene_lore_entries.lore_entry_id)
}
```

và format ở tầng view (`Intl.NumberFormat` + đơn vị i18n), không format trong mock.

**(e) Bỏ hard-code i18n trong dữ liệu.** `DraftThemeArgument.role` → `ThemeArgumentRole`; `reactionType` → `ThemeAnswerMode`; `DraftWorldSettings.access` → `LocationAccess`.

**(f) Enum dùng chung.** Sinh union type từ 24 enum DBML, đặt tại `src/core/domain/enums.ts`, thay toàn bộ union khai báo tay trong `types.ts` (UI đang có 5 bộ trùng: `IntensityLevel` vs `RelationDimensionLevel`, `KnowledgeCertainty`, `Gender` ×2, `MemoryType` ×2, `MemoryStatus` ×2 — 4 enum memory giờ chỉ còn tồn tại ở phía UI, xem S-02). Mọi giá trị enum đã có `note` là **nhãn ngắn (≤ 19 ký tự)** để dùng làm label mặc định; quy ước khoá i18n là `<Enum>.<VALUE>` (vd `LoreUsageType.FORESHADOW`), bản dịch en/ja/zh đặt trong `dict.ts`.

**(g) Mock hygiene.** Tách `DEFAULT_BLOCKS` theo từng chapter (không share reference), và bỏ `SAMPLE_ARCS` dùng chung cho 14 project (P2-11).

### 6.3. Lộ trình đề xuất

| Sprint | Nội dung | Đầu ra |
|---|---|---|
| **A — Chốt schema** | S-01 → S-13 **đã xong toàn bộ**; còn P0-03, P0-06 | `db.dbml` v2 + migration đầu tiên |
| **B — Bổ sung bảng thiếu** | `memory_*` (5 bảng), `project_versions`, `deleted_at`, `assets`; mở rộng `character_competences` | `db.dbml` v3 |
| **C — Đồng bộ UI** | Hợp nhất model nhân vật, adapter layer, bỏ chuỗi format, sửa route detail, nối `/worldbuilding/*` | UI chạy trên dữ liệu typed từ schema |

---

## 7. Câu hỏi cần chốt (decision log)

| # | Câu hỏi | Ảnh hưởng |
|---|---|---|
| Q1 | `projects.id` dùng `uuid` hay giữ `varchar` (slug)? | S-01, P0-04 |
| Q2 | `characters` / `world_settings` / `lore_entries` là **thư viện dùng chung** (liên kết qua `scenes`) hay **thuộc project**? | P0-06 — quyết định toàn bộ query của Project Detail |
| Q3 | ~~"Block" trong Chapter Detail có phải là scenes?~~ → **Đã chốt**: Đã thêm `is_locked` vào `scenes` khớp hoàn toàn với `ChapterBlock.locked`. | Đã chốt |
| Q4 | Memory giữ "phẳng" (theo note DBML) hay mở rộng theo UI? | P1-07, P1-08 |
| Q5 | ~~Có cần theme cấp project không?~~ → **Không**: theme chỉ gắn arc; theme cấp project = JOIN `arcs` | Đã chốt |
| Q6 | Có cần soft-delete cho Recycle Bin không? | P1-15 |
| Q7 | UI `certainty` (Low/Medium/High) là *độ tin cậy của nguồn* hay *mức chắc chắn của chủ thể*? | P1-08 |
| Q8 | ~~Năng lực bẩm sinh lưu thế nào?~~ → **Đã chốt**: Bắt buộc gắn cảnh (`scene_id NOT NULL`), vì trong kể chuyện bất kỳ năng lực/kỹ năng nào (kể cả bà lão biết may đồ) đều được thể hiện/bộc lộ qua một cảnh cụ thể | Đã chốt |

---

## 8. Checklist hành động

**Schema (`docs/db.dbml`)** — Đã giải quyết toàn diện
- [x] S-01: thống nhất kiểu ID (`projects.id` + `sessions.project_id` = `uuid`; session/message id = `varchar`)
- [x] S-02: xoá 6 enum 🧟 (28 → 22, sau đó +2 enum ở S-07 và ContentStatus)
- [x] S-03: nối `domain_level` qua `character_competences.level_id`
- [x] S-04: bổ sung cột cho `symbolics` + unique `symbolic_motifs`
- [x] S-05: `themes` **chỉ gắn arc** (bỏ `project_id`, giữ `arc_id`)
- [x] S-06: `relationship_states.relationship_id`
- [x] S-08: `relationship_categories` có `code`, unique theo `(category, sub_type)`
- [x] S-09: hết hiệu lực (enum bị xoá)
- [x] S-10: `character_competences` thêm `code`, index non-unique kèm comment
- [x] S-11: `chapters` có `created_at` / `updated_at`
- [x] S-12: thêm note `UUIDv7`
- [x] S-13: note quy ước code cho `motif_constraints` + syntax/constrain (Option 1b)
- [x] S-07: `scene_lore_entries` (Nhóm 1: `usage` + `reveal_to_reader` + `reveal_to_pov`), xoá `scenes.lore_entries`
- [x] P0-03: chuẩn hóa enum `gender` (`NONE`, `MALE`, `FEMALE`)
- [x] P1-15: thêm `deleted_at timestamp` vào các bảng chính (`projects`, `arcs`, `chapters`, `scenes`, `characters`, `world_settings`, `lore_entries`, `themes`)
- [x] P2-04: thêm `narrator_configs.description`
- [x] P1-02: `scenes` có `is_locked` khớp `ChapterBlock.locked`

**UI & Refactor Kiến trúc (`src/`)**
- [x] P0-01: Hợp nhất model nhân vật + sửa lỗi fix cứng route `/character/:id`
- [x] P0-06: `world_settings` gắn `project_id`, `characters` độc lập tái sử dụng qua `scene_characters`
- [x] P0-07: `SidebarHistory` lưu `projectId`, chỉ render `projectName`
- [x] P1-04: `SceneCharacter` có `characterId` / `characterCode`
- [x] P1-05: `SceneConfig` có mảng `loreEntries` chuẩn N-N theo `scene_lore_entries`
- [x] P1-07: `character_memories` thu gọn phẳng theo DBML
- [x] P1-10: bỏ `hairHex`, `eyeHex`, `skinHex` trên UI theo DBML
- [x] P1-11: relationships chuẩn hóa theo `CategoryGroup` và `Hierarchy`
- [x] P1-14: `project_genres` theo Option 2a
- [x] P1-16: Theme arguments chuẩn hóa sang enum (`PRIMARY`, `ANTITHESIS`, `ACTION`, `VERBAL`)
- [x] P2-11: mỗi project clone `SAMPLE_ARCS` riêng biệt
- [x] P2-12: dọn dẹp interface trùng trong `draft/types.ts`
- [x] Tái cấu trúc toàn bộ `src/` thành `features/`, `pages/`, `app/`, `shared/`, `config/` và dọn sạch `src/core/`

**Tài liệu**
- [ ] Ghi lại Q1–Q8 vào decision log chung
- [ ] Sinh `src/core/domain/enums.ts` từ DBML và thay union khai báo tay

---

## Phụ lục A — Bảng đối chiếu enum

| Enum DBML | Giá trị DBML | UI tương ứng | Kết quả |
|---|---|---|---|
| `MotifsType` | character_arc / relationship / symbolic / plot | `MotifCategory` | ✅ |
| `NarrativeWeight` | NONE…HIGH | `NarrativeWeight` | ✅ |
| `SentenceLength` | VERY_SHORT…LONG | `SentenceLength` | ✅ |
| `SentenceStructure` | SIMPLE…MIXED | `SentenceStructure` | ✅ |
| `TonePace` | VERY_LOW…VERY_FAST | `TonePace` | ✅ |
| `GrammaticalPerson` | FIRST/SECOND/THIRD | `GrammaticalPerson` | ✅ |
| `PersonSubtype` | 9 giá trị | `PersonSubtype` | ✅ |
| `PovScope` | 3 giá trị | `PovScope` | ✅ |
| `DirectiveCategory` | 3 giá trị | `DirectiveCategory` | ✅ |
| `ContentStatus` | PLANNING / DRAFTING / ONGOING / IN_REVIEW / COMPLETED | `ProjectItem.status` (5 giá trị, casing khác) | ✅ khớp giá trị, cần chuẩn hoá casing |
| `Language` | VI / EN / JA / ZH_HANT / ZH_HANS | `DraftOverview.language: string` | ⚠️ |
| ~~`SceneStatus`~~ | GENERATING / DRAFF / REVISING / APPROVED / FAILED | — | 🗑 đã xoá (S-02) |
| `SceneSource` | AI_GENERATED / HUMAN_EDITED / RESTORE | — | ❌ (bảng `scene_versions` không có UI) |
| `gender` | MALE / FEMALE | `Gender` = NONE/MALE/FEMALE; `BiologicalSex` 4-5 giá trị | ❌ |
| `MeasurementUnit` | MM…KG (11) | cm/m/ft/in/kg/lbs (6) | ⚠️ |
| `Hierarchy` | SUPERIOR / EQUAL / INFERIOR | `'superior'\|'equal'\|'inferior'` và `'Superior'\|'Equal'\|'Inferior'` | ⚠️ casing |
| `RelationDimensionLevel` | NONE…ABSOLUTE | `IntensityLevel` / `RelationshipStateLevel` | ✅ |
| `CategoryGroup` | FAMILY / ROMANTIC / SOCIAL / PROFESSIONAL / ANTAGONISTIC | `'Family'\|'Romantic'\|'Social'\|'Professional'\|'Rival'` | ❌ `Rival` |
| `LocationType` | 9 cấp không gian | `type: string` | ⚠️ |
| `LocationAccess` | PUBLIC…RESTRICTED | free text | ❌ |
| `LoreType` | CULTURE / HISTORY / FACTION / ITEM | `DraftLoreEntry.type` | ✅ |
| `ThemeResolutionType` | RESOLVED / OPEN / AMBIGUOUS | `ThemeResolutionType` | ✅ |
| `ThemeArgumentRole` | PRIMARY / ANTITHESIS / CAUTIONARY / WITNESS | `ThemeArgumentRole` (system-prompt) + `'Chính'\|'Phản đề'` (draft) | ⚠️ |
| `ThemeAnswerMode` | VERBAL / ACTION / OUTCOME / IMPLICIT | `ThemeAnswerMode` + `'Hành động'\|'Lời nói'` (draft) | ⚠️ |
| `LoreUsageType` | 8 giá trị (SETTING…RED_HERRING) | — chưa có UI | ✅ schema-native (mới ở S-07) |
| `RevealDepth` | NONE / HINT / PARTIAL / FULL | — chưa có UI | ✅ schema-native (mới ở S-07) |
| ~~`MemoryType`~~ | EPISODIC / SEMANTIC / PROCEDURAL | `MemoryType` (UI vẫn dùng) | 🗑 đã xoá (S-02) |
| ~~`MemoryCertainty`~~ | CERTAIN / PROBABLE / UNCERTAIN | `KnowledgeCertainty` Low/Medium/High | 🗑 đã xoá (S-02) |
| ~~`MemorySource`~~ | FIRSTHAND / HEARSAY / INFERENCE | `knowledgeSource: string` | 🗑 đã xoá (S-02) |
| ~~`MemoryStatus`~~ | ACTIVE / SUPPRESSED / FORGOTTEN / DISTORTED | `MemoryStatus` (UI vẫn dùng) | 🗑 đã xoá (S-02) |

## Phụ lục B — Mức độ phủ theo bảng

| Mức | Số bảng | Danh sách |
|---|---|---|
| ✅ Khớp gần 1:1 | 24 | `motifs`*, `genres`*, `motif_applicable_genres`, `components`*, `states`*, `state_rules`*, `state_down_by_rules`, `emotional_ranges`*, `event_blocks`, `event_transitions`, `event_block_emotionals`, `constraints`, `motif_constraints`, `style_presets`, `rhetorical_devices`, `style_preset_rhetorical_devices`, `style_preset_vocab_items`, `style_preset_focus_items`, `narrative_directives`, `style_preset_narratives`, `narrator_configs`, `theme_arguments`, `competence_capabilities`, `competence_limitations` |
| ⚠️ Có UI nhưng lệch | 25 | `projects`, `project_genres`, `arcs`, `arc_motifs`, `arc_event_blocks`, `chapters`, `scenes`, `scene_characters`, `sessions`, `messages`, `characters`, `character_appearances`, `character_distinguishing_marks`, `relationships`, `relationship_categories`, `relationship_roles`, `relationship_states`, `world_settings`, `lore_entries`, `themes`, `character_memories`, `competence_domains`, `character_competences`, `character_motifs`, `scene_lore_entries` |
| ❌ Không có UI | 6 | `scene_versions`, `appearance_option_groups`, `appearance_options`, `domain_level`, `symbolic_motifs`, `symbolics` |

\* chỉ lệch tên `display_name` ↔ `name`. Tổng: 24 + 25 + 6 = **55 bảng**, khớp số `Table` khai báo trong [`db.dbml`](../db.dbml).

---

*Hết báo cáo. Mọi kết luận đều trích từ file trong repo tại thời điểm 2026-10-29; không có mã nguồn nào bị thay đổi.*
