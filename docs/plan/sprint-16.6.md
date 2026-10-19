# SPRINT 16.6 — PROJECT TAB: 3×3 OVERVIEW GRID + RANDOM COVERS

> **Trạng thái: READY FOR SUBAGENT.**
> Branch: `sprint-16.6-project-tab` · Base: `sprint-16.5-sidebar-history` (sau khi 16.5 DONE).
> Nếu 16.5 chưa merge: rebase branch này lên `sprint-16.5-sidebar-history`, **không sửa** phần sidebar
> history. Baseline trên base 16.5: `cargo test` phải 0 failed / 3 ignored; `pnpm build` pass.
> Route mặc định: `openai-codex / gpt-6-sol / high`; fallback `commandcode / deepseek-v4.1-flash / high`.

## CONTEXT

- `src/pages/Projects.tsx` hiện chỉ là `ComingSoon`; sidebar có tab `Projects` nhưng chưa có màn hình thật.
- Backend chỉ có `db::list_projects` **test-only** trả `Vec<String>`; chưa có command list project.
- Metadata project nằm ở hai nguồn:
  - SQLite `projects(id, title, created_at)` — luôn có sau `project_create`.
  - `projects/<id>/super.json` — `require.project.name` và `require.schema.project_genres.rows`
    (`genre_id`, `is_primary`) có thể có hoặc chưa có.
- Yêu cầu mới:
  - Tab Projects để **vào xem** thư viện project.
  - Grid overview **3 cột** (3×3 khi có 9 project), card gồm cover + title + genres.
  - `ProjectCover` là component dùng chung, **hiển thị random** nhưng không nhấp nháy.
  - Genre trong card chỉ chiếm **tối đa 2 dòng**, **n cột**; nếu tràn, component cuối hiển thị `...`
    và `title` chứa các genre bị ẩn.
  - Nút **New Project** trong Projects điều hướng sang tab chat (tạo project qua hội thoại), **không**
    mở form tạo project.

## DECISIONS (chốt, không hỏi lại)

### D1 — `ProjectSummary` contract

```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProjectSummary {
    pub id: String,
    pub title: String,
    pub genres: Vec<String>,
    pub created_at: String,
    pub session_count: u32,
}
```

- `title`: `super.json` `require.project.name` nếu là string non-empty; ngược lại `projects.title`.
  (`project_create` có thể để title DB là `"Dự án chưa đặt tên"` rồi agent đặt tên sau.)
- `genres`: lấy từ `project_genres.rows[].genre_id`, trim, bỏ rỗng, dedupe không phân biệt hoa thường;
  row `is_primary == true` đứng trước, các row còn lại giữ thứ tự gốc.
- `session_count`: `COUNT(*) FROM sessions WHERE project_id = projects.id`.
- Sort list: `projects.created_at DESC, projects.rowid DESC`.
- Project không có `super.json` hoặc file hỏng: **không làm fail cả list**; dùng title DB, `genres: []`.

### D2 — Backend đọc file không tạo ghost

Thêm hàm chỉ đọc trong `super_json/io.rs`:

```rust
/// Reads an existing super json file. Missing file → Ok(None). Never creates
/// a project directory or a skeleton, unlike `load_or_init`.
pub fn load_existing(project_id: &str) -> Result<Option<Value>, String>;
```

Re-export ở `super_json/mod.rs`. `list_project_summaries`/`project_summary` dùng hàm này, không dùng
`load_or_init`.

### D3 — Command surface

Trong `commands/project.rs`:

```rust
#[tauri::command]
pub(crate) fn list_projects() -> Result<Vec<ProjectSummary>, String>;

#[tauri::command]
pub(crate) fn get_project(project_id: String) -> Result<Option<ProjectSummary>, String>;
```

- JS gọi `list_projects` với `{}`; `get_project` với `{ projectId }`.
- `list_projects` là read-only: không tạo project row, không tạo `super.json`.
- `get_project` trả `None` nếu project không tồn tại; không tạo gì.

### D4 — `ProjectCover` dùng chung, random ổn định

Tạo `src/components/ProjectCover.tsx`:

```ts
interface ProjectCoverProps {
  projectId: string;
  title: string;
  genres?: string[];
  variant?: "card" | "detail";
}
```

- Palette 6 gradient lấy đúng từ `ref-ui/projects.html` `fallbackCovers`:
  ```text
  linear-gradient(135deg, #8d829f, #dbc7b8 56%, #333d6f)
  linear-gradient(145deg, #183e65, #5ca6a3 56%, #d4ad7c)
  linear-gradient(135deg, #97647d, #d7a1a6 55%, #394562)
  linear-gradient(135deg, #343f70, #888ac1 56%, #d9b5a8)
  linear-gradient(135deg, #526774, #b4bbb3 52%, #bea486)
  linear-gradient(135deg, #435970, #88a4ad 54%, #efbd95)
  ```
- Chọn palette bằng hash ổn định của `projectId` (`hashString(id) % 6`). Đây là “random” về thị giác
  nhưng **cùng project luôn cùng cover** giữa grid/detail và giữa các lần render — tránh nhấp nháy.
- Component render:
  - `<div role="img" aria-label={`Cover artwork for ${title}`}>`.
  - Label nhỏ `MERLINK · STORY FILE`.
  - Caption: `variant === "detail" ? title : (genres?.[0] ?? title)`.
  - CSS class `project-cover project-cover--card|detail`.
- Dùng ở **hai nơi**: `ProjectOverviewCard` (grid) và `ProjectDetail` (trang xem). Không copy markup
  cover ở nơi khác.

### D5 — `GenreList`: tối đa 2 dòng, n cột, ellipsis cuối

Tạo `src/components/GenreList.tsx`:

```ts
interface GenreListProps {
  genres: string[];
  columns?: number;   // default 3
  maxRows?: number;   // default 2
}
```

- `budget = columns * maxRows`.
- Nếu `genres.length <= budget`: render tất cả.
- Nếu vượt: render `budget - 1` genre đầu, rồi một chip cuối:
  - text hiển thị: `...`
  - `title` attribute: các genre bị ẩn nối bằng `" · "` (ví dụ
    `title="Fantasy · Mystery · Romance"`)
  - `aria-label`: `"More genres: ..."` cùng danh sách.
- Không có genre: render `<span className="project-genre-empty">No genres yet</span>`.
- CSS grid: `grid-template-columns: repeat(var(--genre-columns), minmax(0, 1fr))`;
  `grid-auto-rows: 26px`; mỗi chip `min-width:0; overflow:hidden; text-overflow:ellipsis;
  white-space:nowrap`.
- `columns` mặc định 3; card overview dùng 3 → tối đa 6 slot; >6 genre thì 5 + `...`.
- “n cột” được thể hiện bằng prop `columns`; không hardcode số cột trong JSX.

### D6 — Projects page: 3 cột + detail route

- `/projects` thay `ComingSoon` bằng thư viện thật:
  - Header: breadcrumb `Workspace / Projects`, nút `New Project` (icon `Plus`).
  - Grid: `grid-template-columns: repeat(3, minmax(0, 1fr))`; 9 project đầu tạo đúng 3×3, nhiều hơn
    thì xuống hàng. Responsive: ≤1100px → 2 cột; ≤700px → 1 cột.
  - Card: `ProjectCover` + title + `GenreList`. Không hiện mô tả, word count, status, session list.
  - Loading: `Loading projects…`; error: message + `Retry`; empty: copy ngắn + nút `New Project`.
- `/projects/:projectId` — trang xem project:
  - Back link `Back to projects`.
  - Cover lớn (`variant="detail"`), title, `GenreList` (columns 3, maxRows 4), `created_at` đã format,
    `session_count` dạng `N conversations`.
  - Nút `Start a chat` → `/chat` + dispatch `merlink:new-chat` (cùng helper với New Project).
  - Project không tồn tại: `Project not found` + back link; không crash.
- Click card → `navigate(\`/projects/${encodeURIComponent(id)}\`)`.
- `New Project` (ở grid, empty state, detail):
  ```ts
  navigate("/chat");
  window.dispatchEvent(new Event("merlink:new-chat"));
  ```
  Không mở form tạo project; không gọi `project_create` trực tiếp.

### D7 — Browser mock

- `list_projects`: trả summary cho:
  - các `mockProjects` trong `src/data/mock.ts`;
  - các project do mock `project_create` tạo trong `sessions` (projectId khác rỗng, chưa có trong mock).
  - Sort `created_at DESC`.
- `get_project`: tìm trong cùng tập; trả `null` nếu không có.
- Genres: mock project dùng `[project.genre]`; project tạo qua chat đọc `project_genres` values nếu có.
- `session_count`: đếm sessions có cùng `projectId`.
- Không cần cover field; cover do component tự hash.

### D8 — Không thêm dependency

Dùng `lucide-react` sẵn có (`Plus`, `BookOpen`, `ArrowLeft`, `ChevronRight` nếu cần). Không thêm
router mới (React Router đã có), không thêm thư viện ảnh/gradient.

---

## TASKS

### T1 — `super_json` read-only helper

1. `src-tauri/src/super_json/io.rs`: thêm `pub fn load_existing(project_id: &str) -> Result<Option<Value>, String>`.
   - `paths::super_json_path`; `NotFound` → `Ok(None)`; lỗi đọc/parse khác → `Err` kèm path.
   - Không tạo directory/file.
2. `src-tauri/src/super_json/mod.rs`: re-export `load_existing`.
3. Test trong `io.rs`: `test_load_existing_does_not_create_a_missing_project` và
   `test_load_existing_reads_an_existing_document`.

### T2 — Project summaries trong DB

1. `src-tauri/src/models.rs`: thêm `ProjectSummary` đúng D1.
2. `src-tauri/src/db/projects.rs`:
   - `pub fn list_project_summaries(connection: &Connection) -> Result<Vec<ProjectSummary>, String>`.
   - `pub fn project_summary(connection: &Connection, project_id: &str) -> Result<Option<ProjectSummary>, String>`.
   - Query projects; session counts bằng một `GROUP BY project_id` (không N+1).
   - Đọc `super_json::load_existing`; lỗi/parse hỏng → fallback DB title + `genres: []`.
   - Helper `fn extract_genres(document: &Value) -> Vec<String>` tìm `project_genres` trong
     `require.schema` hoặc `optional.schema`, đọc rows, dedupe, primary-first.
   - Helper `fn preferred_title(document: &Value, db_title: &str) -> String`.
3. Tests:
   - `test_list_project_summaries_orders_newest_first`
   - `test_project_summary_prefers_super_json_name_and_primary_genres`
   - `test_project_summary_falls_back_when_super_json_is_missing`
   - `test_project_summary_counts_sessions`
   - `test_project_summary_returns_none_for_unknown_project`
4. `src-tauri/src/db/mod.rs`: re-export hai hàm mới. Giữ nguyên `#[cfg(test)] list_projects` cũ cho
   test hiện có.

### T3 — Commands + IPC contract

1. `src-tauri/src/commands/project.rs`: thêm `list_projects`, `get_project` (D3).
2. `src-tauri/src/commands/mod.rs`: re-export.
3. `src-tauri/src/lib.rs`: import + thêm 2 handler vào `merlink_handlers!`.
4. `src-tauri/src/ipc_contract_tests.rs`:
   - `body_list_projects` = `{}`; `body_get_project` = `{ "projectId": "contract-project" }`.
   - 2 `ContractEntry` mới.
   - End-to-end: tạo project bằng tool `project_create` (hoặc seed DB + super.json), gọi
     `list_projects`/`get_project`, assert title/genres/count.
   - `test_contract_table_covers_every_registered_command` pass.

### T4 — Frontend types, IPC map, browser mock

1. `src/types.ts`: thêm `ProjectSummary` mirror D1.
2. `src/lib/ipc.ts`:
   - `list_projects: { args: never; result: ProjectSummary[] }`
   - `get_project: { args: { projectId: string }; result: ProjectSummary | null }`
3. `src/lib/browser-mock.ts`: implement D7; import `mockProjects` từ `../data/mock`.
4. Không đổi `Project` mock type cũ; `ProjectSummary` là type mới.

### T5 — Components dùng chung

1. `src/components/ProjectCover.tsx` đúng D4.
2. `src/components/GenreList.tsx` đúng D5.
3. `src/components/ProjectOverviewCard.tsx`:
   - `article` role button/tabIndex, click + Enter/Space → `onOpen(project.id)`.
   - `ProjectCover` card + title + `GenreList columns={3} maxRows={2}`.
   - Không hiện metadata khác.

### T6 — Pages + route

1. `src/pages/Projects.tsx`: thay `ComingSoon` bằng D6.
   - `useEffect` load `list_projects`; loading/error/empty.
   - `New Project` dùng helper D6.
   - Grid + cards.
2. `src/pages/ProjectDetail.tsx`: route detail D6.
   - `useParams`; `get_project`; loading/not-found.
   - `ProjectCover variant="detail"`, title, `GenreList columns={3} maxRows={4}`, created date,
     session count, back link, `Start a chat`.
   - Format ngày bằng `Intl.DateTimeFormat` (không thêm dependency).
3. `src/App.tsx`: thêm route `projects/:projectId`; giữ `projects` route. Không đổi sidebar 16.5.
4. New Project / Start a chat helper: có thể đặt hàm nhỏ trong từng page hoặc `src/lib/newChat.ts`
   (nếu tạo file mới, thêm vào allow-list). Ưu tiên hàm local trong page để tránh file mới.

### T7 — CSS `src/styles/projects.css`

Tạo file mới, import trong `Projects.tsx` và `ProjectDetail.tsx`:

```text
.projects-page
.projects-header
.projects-grid
.project-overview-card
.project-overview-body
.project-overview-title
.project-cover
.project-cover--card
.project-cover--detail
.project-cover-label
.project-cover-caption
.project-genre-grid
.project-genre-chip
.project-genre-empty
.project-detail-page
.project-detail-cover-wrap
.project-detail-meta
.projects-empty
```

- Grid 3 cột, gap 22px; responsive 2/1.
- Card `surface`, padding 10px, radius 19px, hover translateY(-3px); focus-visible ring.
- Cover aspect 16/9 (card), 1.28 (detail), radius 13/19px, overflow hidden; decorative ring bằng
  `::before` như ref-ui; gradient lấy từ inline style.
- Genre grid: `display:grid; grid-template-columns:repeat(var(--genre-columns,3),minmax(0,1fr));
  gap:6px;` chip 26px, ellipsis.
- Detail: 2 cột trên desktop (cover trái, info phải), 1 cột ≤900px; cover sticky optional.
- Dark theme chỉ dùng CSS variables; không hardcode nền sáng.
- Không đụng `.project-*` cũ nếu có trong `src/index.css`; class mới nằm trong `projects.css`.

### T8 — Verify + commit + push

```bash
cargo test --manifest-path src-tauri/Cargo.toml   # 0 failed / 3 ignored
pnpm build                                        # exit 0
git diff --stat
git add <ALLOW-LIST>
git -c user.name=taho140303 -c user.email="202986530+taho140303@users.noreply.github.com" \
    commit -m "feat(projects): add project overview grid with shared random covers"
git push -u origin sprint-16.6-project-tab
```

Không `git add -A`; không add `.agents/`, `dist/`, `src-tauri/target/`, `docs/schema/system.dbdiagram`.

---

## ACCEPTANCE

- [ ] `list_projects` trả `ProjectSummary[]` sort mới nhất trước; `get_project` trả đúng project hoặc
      `null`; cả hai read-only, không tạo `super.json`/project row.
- [ ] Title ưu tiên `require.project.name`; fallback DB title. Genres dedupe, primary-first; thiếu file
      → `genres: []`, không fail list.
- [ ] `session_count` đúng; project không có session → `0`.
- [ ] 2 command có trong `merlink_handlers!`, `CommandMap`, browser mock, IPC contract; contract
      coverage test pass.
- [ ] `/projects` render grid 3 cột; 9 project tạo 3×3; responsive 2/1.
- [ ] Card dùng `ProjectCover` + title + `GenreList`; không render mô tả/status/word count.
- [ ] `ProjectCover` dùng chung ở card và detail; cùng project luôn cùng gradient (stable hash).
- [ ] `GenreList`: tối đa 2 dòng × `columns` cột; > budget thì chip cuối là `...` với `title` chứa
      genre bị ẩn; không tràn layout.
- [ ] Click card mở `/projects/:id`; back link về `/projects`; project không tồn tại có empty state.
- [ ] `New Project` ở grid/empty/detail → `/chat` + dispatch `merlink:new-chat`; không mở form.
- [ ] Loading/error/empty states hoạt động; browser preview cũng chạy.
- [ ] `cargo test` 0 failed / 3 ignored; `pnpm build` exit 0.
- [ ] `git diff --stat` chỉ chứa ALLOW-LIST; commit author `taho140303`.

## ALLOW-LIST

```text
docs/plan/sprint-16.6.md
src-tauri/src/super_json/io.rs
src-tauri/src/super_json/mod.rs
src-tauri/src/models.rs
src-tauri/src/db/projects.rs
src-tauri/src/db/mod.rs
src-tauri/src/commands/project.rs
src-tauri/src/commands/mod.rs
src-tauri/src/lib.rs
src-tauri/src/ipc_contract_tests.rs
src/lib/ipc.ts
src/lib/browser-mock.ts
src/types.ts
src/App.tsx
src/components/ProjectCover.tsx
src/components/GenreList.tsx
src/components/ProjectOverviewCard.tsx
src/pages/Projects.tsx
src/pages/ProjectDetail.tsx
src/styles/projects.css
```

Ngoài danh sách: **KHÔNG sửa**. Đặc biệt không đụng sidebar history của 16.5, `src/index.css`,
`docs/schema/*`, `AGENTS.md`, `src/pages/Home.tsx`.

───────────────────────────────────────────────────────
🛑 STOP CONDITIONS
───────────────────────────────────────────────────────
- ACCEPTANCE pass hết → DỪNG, báo `SPRINT DONE`.
- Retry 1 task quá 2 lần vẫn fail → DỪNG, escalate kèm output.
- Cần đổi `ProjectSummary` shape / route / IPC ngoài spec → DỪNG, hỏi.
- Cần thêm dependency → DỪNG, hỏi.
- Cần binding chat vào project có sẵn / form tạo project → DỪNG, hỏi (ngoài sprint).
- Task vượt 30 phút ước tính → DỪNG, chia nhỏ.

───────────────────────────────────────────────────────
🔁 RETRY BUDGET
───────────────────────────────────────────────────────
- Mỗi task tối đa 2 lần; lần 2 phải đổi approach, không lặp y hệt.
- Sau 2 lần fail: dừng task đó, chuyển task khác; cuối sprint liệt kê.

───────────────────────────────────────────────────────
🚨 ESCALATION TRIGGERS
───────────────────────────────────────────────────────
1. Lỗi không rõ nguyên nhân sau 2 lần debug.
2. Phải chọn giữa 2 approach mà spec không nói.
3. Phải đổi kiến trúc (route, IPC shape, DB schema).
4. Phát hiện bug sprint trước ảnh hưởng sprint này.
5. Cần user test thủ công (visual/UX/perf).

───────────────────────────────────────────────────────
🚫 OUT OF SCOPE
───────────────────────────────────────────────────────
- Edit/delete project, upload cover, form tạo project, project settings.
- Binding chat vào project có sẵn; tạo session từ project detail.
- Session list trong project detail (sidebar 16.5 đã có).
- Reuse cover ở Home/dashboard; thay mock data toàn cục.
- Search/filter/sort UI ngoài sort backend created_at.
- Sửa `docs/schema/system.dbdiagram`, `.agents/`, `dist/`, `target/`, `node_modules/`.
- Merge `main`, force-push, rewrite history.

───────────────────────────────────────────────────────
📤 HANDOFF FORMAT
───────────────────────────────────────────────────────

STATUS: [DONE | PARTIAL | BLOCKED | NEED_INPUT]
SPRINT: 16.6
ROUTE: [provider/model/reasoning_effort thực tế]

✅ COMPLETED:
- [task N]: [file changed] + [output thật]

❌ FAILED:
- [task]: [lỗi] | [đã thử] | [cần gì]

🤔 NEED_INPUT:
- [câu hỏi] | Option A | Option B

📋 NEXT ACTION:
- DONE → "Ready for root review/merge"
- BLOCKED → "Waiting for user response on X"

```

---

## SUBAGENT CALL BRIEF (dùng đúng format AGENTS.md)

```text
═══════════════════════════════════════════════════════
SPRINT 16.6 — PROJECT TAB: 3×3 OVERVIEW GRID + RANDOM COVERS
═══════════════════════════════════════════════════════

CONTEXT:
Repo /home/hyanhasta05/workspace/personal/pet-project/merlink-harness.
Base: branch sprint-16.5-sidebar-history (sau khi 16.5 DONE); nếu chưa có thì rebase lên branch đó.
Spec đầy đủ: docs/plan/sprint-16.6.md. Projects page hiện là ComingSoon; chưa có list/get project IPC.

GOAL:
Tab Projects có thư viện project 3 cột với ProjectCover dùng chung (random ổn định), GenreList tối đa
2 dòng/n cột với ellipsis, detail route để xem project, New Project điều hướng sang chat; cargo test
và pnpm build xanh.

TASKS:
1. Đọc docs/plan/sprint-16.6.md và làm đúng T1–T8 trong ALLOW-LIST.
2. Không sửa file ngoài ALLOW-LIST; không đụng sidebar history 16.5 hoặc docs/schema/system.dbdiagram.
3. Chạy cargo test + pnpm build; commit bằng identity taho140303; push branch
   sprint-16.6-project-tab.

ACCEPTANCE:
- Toàn bộ checklist trong docs/plan/sprint-16.6.md pass.
- cargo test 0 failed / 3 ignored; pnpm build exit 0.
- git diff --stat chỉ còn ALLOW-LIST; commit author taho140303.

───────────────────────────────────────────────────────
🛑 STOP CONDITIONS
───────────────────────────────────────────────────────
- ACCEPTANCE pass hết → DỪNG, báo "SPRINT DONE".
- Retry 1 task quá 2 lần vẫn fail → DỪNG, escalate.
- Cần đổi ProjectSummary/route/IPC shape ngoài spec → DỪNG, hỏi.
- Cần thêm dependency → DỪNG, hỏi.
- Cần form tạo project hoặc binding chat vào project có sẵn → DỪNG, hỏi.
- Task vượt quá 30 phút ước tính → DỪNG, chia nhỏ.

───────────────────────────────────────────────────────
🔁 RETRY BUDGET
───────────────────────────────────────────────────────
- Mỗi task tối đa 2 lần; lần 2 phải đổi approach.
- Sau 2 lần fail: dừng task đó, báo PARTIAL.

───────────────────────────────────────────────────────
🚨 ESCALATION TRIGGERS
───────────────────────────────────────────────────────
1. Lỗi không rõ nguyên nhân sau 2 lần debug.
2. Cần chọn A/B mà spec không nói.
3. Cần đổi kiến trúc hiện tại.
4. Phát hiện bug sprint trước ảnh hưởng sprint này.
5. Cần user test thủ công visual/UX.

───────────────────────────────────────────────────────
🚫 OUT OF SCOPE
───────────────────────────────────────────────────────
- Edit/delete/upload cover/form tạo project; binding chat vào project có sẵn.
- Session list trong project detail; reuse cover ở Home.
- Search/filter/sort UI ngoài sort backend.
- Sửa docs/schema/system.dbdiagram, .agents/, dist/, target/, node_modules/.
- Merge main, force-push, rewrite history.

───────────────────────────────────────────────────────
📤 HANDOFF FORMAT
───────────────────────────────────────────────────────

STATUS: [DONE | PARTIAL | BLOCKED | NEED_INPUT]
SPRINT: 16.6
ROUTE: [provider/model/reasoning_effort thực tế]

✅ COMPLETED:
- [task N]: [file changed] + [output thật]

❌ FAILED:
- [task]: [lỗi] | [đã thử] | [cần gì]

🤔 NEED_INPUT:
- [câu hỏi] | Option A | Option B

📋 NEXT ACTION:
- DONE → "Ready for root review/merge"
- BLOCKED → "Waiting for user response on X"

═══════════════════════════════════════════════════════
```
