# SPRINT 16.5 — SIDEBAR CONVERSATION HISTORY (TITLE-ONLY ROWS)

> **Trạng thái: DONE — implemented on `sprint-16.5-sidebar-history`; root-verified.**
> Commits: `4468731` (implementation), `cf2df06` (spec), `fb00fff` (review fixes).
> Branch: `sprint-16.5-sidebar-history` · Base: `main` @ `125c77e` (sau merge Sprint 16.1).
> Baseline đã verify trên base: `cargo test` **155 passed / 3 ignored**, `pnpm build` pass.
> Kết quả sau sprint: `cargo test` **161 passed / 3 ignored**, `pnpm build`, fmt và clippy pass.
> Route mặc định: `openai-codex / gpt-6-sol / high`; fallback `commandcode / deepseek-v4.1-flash / high`.

## CONTEXT

- Sidebar hiện chỉ có brand, New chat, Workspace nav, Settings. **Không có lịch sử cuộc trò chuyện.**
- `Chat.tsx` tạo `newSessionId()` mỗi lần mount; không có route để mở lại session cũ. SQLite đã có
  bảng `sessions(id, project_id, title, created_at)` nhưng chưa có command list/rename/delete.
- `ref-ui/chat.html` có panel lịch sử với preview + thời gian. Yêu cầu mới **cố tình khác HTML**:
  mỗi session row chỉ có **title căn trái** + icon lucide `MoreVertical` (3 chấm dọc); menu cho
  **Rename** và **Delete**; không render preview, timestamp, project name hay metadata nào khác.
- Có switch **Group by project**, mặc định **OFF**. OFF = danh sách phẳng theo `sessions.created_at`
  giảm dần. ON = mỗi project là một toggle xổ ra/thu gọn các session của project đó.
- New chat đã nằm ở sidebar và phát event `merlink:new-chat`. Sprint này mở rộng nó thành lịch sử
  thật, có resume.

## DECISIONS (chốt, không hỏi lại)

### D1 — Data contract

Thêm model dùng chung:

```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SessionSummary {
    pub id: String,
    /// Empty string means "chưa gắn project" (DB đang lưu '' cho unbound).
    pub project_id: String,
    pub project_title: Option<String>,
    pub title: String,
    pub created_at: String,
}
```

- `title` là **display title đã fallback**: `sessions.title` nếu có; nếu không, lấy **user message
  đầu tiên** (collapse whitespace, tối đa 60 ký tự, thêm `…` nếu cắt); nếu vẫn không có,
  `"New conversation"`.
- `project_title` lấy từ `projects.title` qua `LEFT JOIN`; `None` khi unbound hoặc project row
  không còn. Không join ra `projects.created_at`.
- Sort: `sessions.created_at DESC, sessions.rowid DESC`.

### D2 — Backend functions và IPC

Các hàm DB (trong `db/sessions.rs`):

```rust
pub fn list_sessions(connection: &Connection) -> Result<Vec<SessionSummary>, String>;
pub fn get_session(connection: &Connection, session_id: &str) -> Result<Option<SessionSummary>, String>;
pub fn rename_session(connection: &Connection, session_id: &str, title: &str) -> Result<SessionSummary, String>;
pub fn delete_session(connection: &Connection, session_id: &str) -> Result<bool, String>;
```

Commands mới (trong `commands/sessions.rs`):

```rust
list_sessions() -> Result<Vec<SessionSummary>, String>
get_session(session_id: String) -> Result<Option<SessionSummary>, String>
rename_session(session_id: String, title: String) -> Result<SessionSummary, String>
delete_session(session_id: String) -> Result<bool, String>
```

- JS gửi **camelCase**: `{ sessionId }`, `{ sessionId, title }`.
- `rename_session`: trim; reject rỗng với `"Session title cannot be empty"`; tối đa 120 ký tự
  (`chars().count()`), reject với `"Session title is too long (max 120 characters)"`. Lưu đúng
  title đã trim. Trả `SessionSummary` mới (re-read) để frontend không tự đoán.
- `delete_session`: transaction; `DELETE FROM messages WHERE session_id = ?1` rồi
  `DELETE FROM sessions WHERE id = ?1`; trả `false` nếu session không tồn tại. Không xóa project,
  không xóa super.json, không đụng session khác.
- `get_session` trả `None` nếu không có; không tạo row.

### D3 — Route và resume

- Thêm route `/chat/:sessionId` trong `App.tsx` (HashRouter).
- `Chat.tsx` đọc `useParams<{ sessionId?: string }>()`.
- Khi mount với `sessionId`:
  1. `get_session` để lấy `project_id` và title.
  2. `list_messages` để lấy transcript.
  3. Set `sessionId`, `projectId`, `messages`; reset live frame/tool log/reasoning state; gọi
     `refreshProgress()` và `refreshDetail(project_id)`.
  4. Nếu `get_session` trả `None`: `navigate("/chat", { replace: true })` và bắt đầu chat mới.
- Khi `/chat` không có param: hành vi cũ (session mới). Nếu chuyển từ `/chat/:id` về `/chat`
  (back/forward), phải reset về chat mới **một lần**, không tạo hai session.
- Sidebar session click điều hướng `/chat/${encodeURIComponent(id)}`.
- Sau lượt gửi đầu tiên thành công, `navigate(\`/chat/${encodeURIComponent(turnSessionId)}\`,
  { replace: true })` để URL phản ánh session vừa tạo. Dùng ref để effect load-session **bỏ qua**
  lần đổi route này (không reload transcript, không reset).
- Không đổi shape `send_chat` hay `ChatTurn`.

### D4 — Sidebar UI

- Vị trí: ngay dưới `Workspace` nav, trên `merlink-sidebar-bottom`; `flex:1; min-height:0`.
- Header nhỏ `Conversations` + switch `Group by project` (label + `role="switch"`,
  `aria-checked`). Switch mặc định **OFF**; lưu `localStorage` key
  `merlink-session-group-by-project` (`"true"`/`"false"`).
- Row phẳng:
  - `NavLink`/button title **một dòng**, căn trái, `text-overflow: ellipsis`, không preview/time.
  - Nút `MoreVertical` (lucide) bên phải, `aria-label="Session actions for <title>"`,
    `aria-haspopup="menu"`, `aria-expanded`.
  - Menu chỉ có 2 item: `Rename`, `Delete`. Click ngoài hoặc `Escape` đóng menu.
  - Row active (URL khớp session) có background `var(--glass-strong)` + border trái accent.
- Group mode:
  - Mỗi project là một toggle row: `ChevronRight`/`ChevronDown` + project title. Không hiện session
    preview. Toggle xổ ra/thu gọn danh sách session con.
  - Group order: project có session mới nhất trước; unbound (`project_id === ""`) xuống cuối.
  - Mặc định tất cả expanded. Lưu các project đang collapsed vào
    `localStorage` key `merlink-collapsed-project-groups` (array project id; unbound dùng `""`).
  - Active session nằm trong group đang collapsed → tự expand group đó khi mount.
- Empty state: `"No conversations yet."` (không thêm CTA).
- Sidebar collapsed: ẩn toàn bộ history + switch. Không thêm nút history riêng.
- Rename:
  - Chọn `Rename` → row đổi thành `<input>` với title hiện tại, autoFocus, select all.
  - `Enter` hoặc blur → gọi `rename_session`; `Escape` → hủy.
  - Lỗi backend hiển thị inline dưới row (text `var(--error)`), giữ input để sửa lại.
- Delete:
  - Mở `Dialog` có sẵn: tiêu đề `Delete conversation?`, copy ngắn, nút `Keep conversation` /
    `Delete conversation`.
  - Xác nhận → `delete_session`; xóa row khỏi state; nếu là session đang mở → navigate `/chat` +
    dispatch `merlink:new-chat`.
- Không render session count, project name, date, preview, avatar trong row.

### D5 — Refresh và active state

- `App.tsx` gọi `list_sessions` khi mount và khi nghe event `merlink:sessions-changed`.
- `Chat.tsx` dispatch `merlink:sessions-changed`:
  - sau khi `send_chat` kết thúc (success **hoặc** error, vì row session có thể đã được tạo),
  - sau khi rename/delete không cần (App tự cập nhật state),
  - sau khi load session xong không cần.
- Active session lấy từ `location.pathname` (`/chat/<id>`), không lưu state riêng.
- Nếu `list_sessions` lỗi, hiện `"Could not load conversations."` + nút retry nhỏ; không làm sập shell.

### D6 — Browser mock

- `src/lib/browser-mock.ts` phải mock đủ 4 command trên.
- `MockSession` thêm `title: string | null`, `createdAt: string`.
- `mockSendChat` khi tạo session mới set `createdAt`; title vẫn `null` để list dùng fallback từ
  user message đầu tiên.
- Seed sẵn **3 session demo** (chỉ browser preview) để thấy flat/group ngay:
  - 2 session gắn với 2 `mockProjects` khác nhau, 1 session unbound.
  - Có ít nhất 1 session có user message để test title fallback.
  - Seed idempotent (chỉ tạo nếu `sessions` rỗng).
- `rename_session`/`delete_session` cập nhật map; `delete` xóa messages trong session.
- `get_session` trả đúng shape; `list_sessions` sort `createdAt DESC`.

### D7 — Không đổi schema, không migration

Bảng `sessions` đã có đủ `title` + `created_at`; **không** thêm cột, không tăng `SCHEMA_VERSION`,
không migration. `title` nullable là hợp lệ.

### D8 — Không thêm dependency

Dùng `lucide-react` sẵn có (`MoreVertical`, `Pencil`, `Trash2`, `ChevronRight`, `ChevronDown`).
Không thêm thư viện menu/popover.

---

## TASKS

### T1 — Model + DB layer

1. `src-tauri/src/models.rs`: thêm `SessionSummary` đúng D1 (serde snake_case mặc định).
2. `src-tauri/src/db/sessions.rs`:
   - helper `fn display_title(stored: Option<String>, first_user: Option<String>) -> String`
     (trim, collapse whitespace, 60 chars + `…`).
   - `list_sessions`, `get_session`, `rename_session`, `delete_session` đúng D1/D2.
   - Query list/get dùng `LEFT JOIN projects` + correlated subquery lấy user message đầu tiên
     (`ORDER BY m.rowid ASC LIMIT 1`). Không N+1.
   - `delete_session` dùng transaction và xóa messages trước.
3. `src-tauri/src/db/mod.rs`: `pub use sessions::{...}` các hàm mới.
4. Tests trong `sessions.rs`:
   - `test_list_sessions_orders_by_created_at_desc_and_falls_back_to_first_user_message`
   - `test_list_sessions_includes_project_title_and_unbound_group`
   - `test_rename_session_trims_and_rejects_empty_or_too_long`
   - `test_delete_session_removes_messages_and_returns_false_when_missing`
   - `test_get_session_returns_none_for_unknown_without_creating_a_row`

### T2 — Commands + IPC tests

1. Tạo `src-tauri/src/commands/sessions.rs` với 4 command D2 (dùng `#[tauri::command]`).
2. `src-tauri/src/commands/mod.rs`: thêm module + re-export bốn command.
3. `src-tauri/src/lib.rs`: import + thêm 4 handler vào `merlink_handlers!`.
4. `src-tauri/src/ipc_contract_tests.rs`:
   - thêm 4 `ContractEntry` + body builder (`body_list_sessions`, `body_get_session`,
     `body_rename_session`, `body_delete_session`).
   - end-to-end test: `list_sessions` → `rename_session` → `get_session` → `delete_session`;
     assert title/project/order và `delete` xóa messages.
   - `test_contract_table_covers_every_registered_command` phải pass.

### T3 — Frontend types, IPC map, browser mock

1. `src/types.ts`: thêm `SessionSummary` mirror D1.
2. `src/lib/ipc.ts`: thêm 4 entry vào `CommandMap`:
   - `list_sessions: { args: never; result: SessionSummary[] }`
   - `get_session: { args: { sessionId: string }; result: SessionSummary | null }`
   - `rename_session: { args: { sessionId: string; title: string }; result: SessionSummary }`
   - `delete_session: { args: { sessionId: string }; result: boolean }`
3. `src/lib/browser-mock.ts`: implement D6; export/seed demo sessions; đảm bảo
   `browserInvoke` xử lý 4 case mới.

### T4 — Sidebar history trong `src/App.tsx`

1. State + effects:
   - `sessions`, `sessionsError`, `groupByProject`, `collapsedGroups`, `menuFor`, `renamingId`,
     `renameValue`, `renameError`, `deleteTarget`.
   - `loadSessions` dùng `invokeCommand("list_sessions")`; mount + event
     `merlink:sessions-changed`.
   - Persist/read 2 localStorage key ở D4; parse lỗi thì fallback an toàn.
2. Render `<Conversations>` dưới nav, trên bottom:
   - Header + switch.
   - Flat list hoặc grouped list đúng D4.
   - Session row component nội bộ (có thể tách `SessionRow` trong cùng file) dùng `NavLink` cho
     title và button `MoreVertical` riêng; không lồng button trong link.
   - Menu Rename/Delete; rename input; delete `Dialog`.
3. Active state từ `location.pathname`; auto-expand group chứa active session.
4. Khi delete active: `navigate("/chat")` + `window.dispatchEvent(new Event("merlink:new-chat"))`.
5. Ẩn history khi `collapsed`.

### T5 — Chat resume trong `src/pages/Chat.tsx`

1. Import `useNavigate`, `useParams`.
2. `sessionId` khởi tạo từ route param nếu có, nếu không `newSessionId()`.
3. Effect load session theo D3:
   - guard `cancelled`, `sessionEpochRef`, `loadedRouteRef`.
   - set `sessionRef.current`/`projectRef.current` **trước** khi gọi các refresh phụ thuộc.
   - clear live frame, tool log, reasoning, turn stats.
4. Sau `send_chat` thành công:
   - `loadedRouteRef.current = turnSessionId`;
   - `navigate(\`/chat/${turnSessionId}\`, { replace: true })` nếu URL chưa có id;
   - dispatch `merlink:sessions-changed` trong `finally` (cả success/error).
5. Xử lý `/chat` → `/chat/:id` và ngược lại không tạo hai session, không reload đè transcript.
6. Không đổi `send_chat` payload, `ChatTurn`, hay luồng tool.

### T6 — CSS trong `src/index.css`

Thêm class mới (giữ prefix `merlink-`):

```text
.merlink-history
.merlink-history-head
.merlink-history-switch
.merlink-session-list
.merlink-session-row
.merlink-session-open
.merlink-session-title
.merlink-session-more
.merlink-session-menu
.merlink-session-menu-item
.merlink-session-rename
.merlink-session-error
.merlink-project-group
.merlink-project-toggle
.merlink-project-children
.merlink-history-empty
```

- Title 1 dòng, ellipsis; row min-height ~36px; padding trái 10–12px; 3 chấm sát phải.
- Menu absolute, z-index trên row, background `var(--glass-strong)`, border `var(--line)`.
- Active row: border-left 2px `var(--accent)`, background `var(--glass-strong)`.
- Group toggle: full width, title ellipsis, chevron 14px.
- History list `overflow-y:auto; min-height:0;` scrollbar mảnh.
- Dark theme dùng CSS variables sẵn có; không hardcode màu mới ngoài `--error` cho lỗi.
- `@media (max-width:600px)`: ẩn history (sidebar mobile đang 56px).

### T7 — Verify + commit + push

```bash
cargo test --manifest-path src-tauri/Cargo.toml   # 0 failed / 3 ignored
pnpm build                                        # exit 0
git diff --stat
git add <ALLOW-LIST>
git -c user.name=taho140303 -c user.email="202986530+taho140303@users.noreply.github.com" \
    commit -m "feat(sidebar): conversation history with rename/delete and project grouping"
git push -u origin sprint-16.5-sidebar-history
```

Không `git add -A`; không add `.agents/`, `dist/`, `src-tauri/target/`, `docs/schema/system.dbdiagram`.

---

## ACCEPTANCE

- [ ] `list_sessions` trả `SessionSummary[]` sort `created_at DESC`; session không có title dùng user
      message đầu tiên (≤60 ký tự, whitespace collapsed); không có message → `"New conversation"`.
- [ ] `project_title` đúng cho session có project; `null`/unbound cho `"''"`.
- [ ] `get_session` không tạo row; session lạ trả `None`.
- [ ] `rename_session` trim, reject rỗng và >120 ký tự, persist, trả summary mới.
- [ ] `delete_session` xóa cả `messages` và `sessions` trong transaction; session lạ trả `false`.
- [ ] 4 command có mặt trong `merlink_handlers!`, `CommandMap`, browser mock, IPC contract; contract
      coverage test pass.
- [ ] Sidebar flat mode: chỉ title trái + `MoreVertical`; **không** preview/time/project; click title
      mở `/chat/:id` và load đúng transcript/project.
- [ ] Switch Group by project mặc định OFF; ON hiển thị project toggle; collapse/expand hoạt động;
      unbound group cuối; active session tự expand group.
- [ ] Rename inline bằng menu; Enter/blur lưu; Escape hủy; lỗi hiển thị inline.
- [ ] Delete mở confirm; xóa row; nếu đang mở thì về chat mới.
- [ ] New chat vẫn tạo session mới; sau tin nhắn đầu tiên session xuất hiện trong sidebar và URL
      đổi thành `/chat/:id` mà không reload transcript.
- [ ] Sidebar collapsed ẩn history.
- [ ] `cargo test` 155+ passed / 0 failed / 3 ignored; `pnpm build` exit 0.
- [ ] `git diff --stat` chỉ chứa ALLOW-LIST; commit author `taho140303`.

## ALLOW-LIST

```text
docs/plan/sprint-16.5.md
src-tauri/src/models.rs
src-tauri/src/db/sessions.rs
src-tauri/src/db/mod.rs
src-tauri/src/commands/sessions.rs
src-tauri/src/commands/mod.rs
src-tauri/src/lib.rs
src-tauri/src/ipc_contract_tests.rs
src/lib/ipc.ts
src/lib/browser-mock.ts
src/types.ts
src/App.tsx
src/pages/Chat.tsx
src/index.css
```

Ngoài danh sách: **KHÔNG sửa**. Đặc biệt không đụng `docs/schema/system.dbdiagram` (đang có thay đổi
cục bộ của root), `src/pages/Projects.tsx` (Sprint 16.6), `src-tauri/src/super_json/*`, `AGENTS.md`.

───────────────────────────────────────────────────────
🛑 STOP CONDITIONS
───────────────────────────────────────────────────────
- ACCEPTANCE pass hết → DỪNG, báo `SPRINT DONE`.
- Retry 1 task quá 2 lần vẫn fail → DỪNG, escalate kèm output.
- Cần đổi schema DB / thêm cột / migration → DỪNG, hỏi.
- Cần thêm dependency → DỪNG, hỏi.
- Cần quyết định UX ngoài spec (ví dụ search, date group, preview, project detail) → DỪNG, hỏi.
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
3. Phải đổi kiến trúc (route shape, IPC shape, DB schema).
4. Phát hiện bug sprint trước ảnh hưởng sprint này.
5. Cần user test thủ công (visual/UX/perf).

───────────────────────────────────────────────────────
🚫 OUT OF SCOPE
───────────────────────────────────────────────────────
- Không làm project tab / project grid / ProjectCover (Sprint 16.6).
- Không search, filter, date grouping, preview, timestamp, session count trong row.
- Không auto-generate title bằng AI; chỉ fallback từ user message đầu tiên.
- Không đổi `send_chat`, `ChatTurn`, tool loop, super json, progress.
- Không sửa `docs/schema/system.dbdiagram`, `.agents/`, `dist/`, `target/`, `node_modules/`.
- Không merge `main`, không force-push, không rewrite history.

───────────────────────────────────────────────────────
📤 HANDOFF FORMAT
───────────────────────────────────────────────────────

STATUS: [DONE | PARTIAL | BLOCKED | NEED_INPUT]
SPRINT: 16.5
ROUTE: [provider/model/reasoning_effort thực tế]

✅ COMPLETED:
- [task N]: [file changed] + [output thật]

❌ FAILED:
- [task]: [lỗi] | [đã thử] | [cần gì]

🤔 NEED_INPUT:
- [câu hỏi] | Option A | Option B

📋 NEXT ACTION:
- DONE → "Ready for Sprint 16.6"
- BLOCKED → "Waiting for user response on X"

```

---

## SUBAGENT CALL BRIEF (dùng đúng format AGENTS.md)

```text
═══════════════════════════════════════════════════════
SPRINT 16.5 — SIDEBAR CONVERSATION HISTORY
═══════════════════════════════════════════════════════

CONTEXT:
Repo /home/hyanhasta05/workspace/personal/pet-project/merlink-harness, base main @ 125c77e.
Sidebar chưa có history; SQLite sessions đã có title/created_at nhưng chưa có list/rename/delete.
Spec đầy đủ: docs/plan/sprint-16.5.md. Baseline: cargo test 155 passed / 3 ignored; pnpm build pass.

GOAL:
Sidebar có danh sách session title-only, resume được, rename/delete được, switch group-by-project
mặc định OFF, và toàn bộ cargo test + pnpm build xanh.

TASKS:
1. Đọc docs/plan/sprint-16.5.md và làm đúng T1–T7 trong ALLOW-LIST.
2. Không sửa file ngoài ALLOW-LIST; không đụng docs/schema/system.dbdiagram.
3. Chạy cargo test + pnpm build; commit bằng identity taho140303; push branch
   sprint-16.5-sidebar-history.

ACCEPTANCE:
- Toàn bộ checklist trong docs/plan/sprint-16.5.md pass.
- cargo test 0 failed / 3 ignored; pnpm build exit 0.
- git diff --stat chỉ còn ALLOW-LIST; commit author taho140303.

───────────────────────────────────────────────────────
🛑 STOP CONDITIONS
───────────────────────────────────────────────────────
- ACCEPTANCE pass hết → DỪNG, báo "SPRINT DONE".
- Retry 1 task quá 2 lần vẫn fail → DỪNG, escalate.
- Cần đổi DB schema/route shape/IPC shape ngoài spec → DỪNG, hỏi.
- Cần thêm dependency → DỪNG, hỏi.
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
- Projects tab/grid/ProjectCover (Sprint 16.6).
- Search, date grouping, preview, timestamp, session count trong row.
- Auto-title bằng AI; đổi send_chat/ChatTurn/tool loop/super json.
- Sửa docs/schema/system.dbdiagram, .agents/, dist/, target/, node_modules/.
- Merge main, force-push, rewrite history.

───────────────────────────────────────────────────────
📤 HANDOFF FORMAT
───────────────────────────────────────────────────────

STATUS: [DONE | PARTIAL | BLOCKED | NEED_INPUT]
SPRINT: 16.5
ROUTE: [provider/model/reasoning_effort thực tế]

✅ COMPLETED:
- [task N]: [file changed] + [output thật]

❌ FAILED:
- [task]: [lỗi] | [đã thử] | [cần gì]

🤔 NEED_INPUT:
- [câu hỏi] | Option A | Option B

📋 NEXT ACTION:
- DONE → "Ready for Sprint 16.6"
- BLOCKED → "Waiting for user response on X"

═══════════════════════════════════════════════════════
```
