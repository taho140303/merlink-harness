═══════════════════════════════════════════════════════
SPRINT 10 — AUTO-CREATE PROJECT + THANH TIẾN ĐỘ + GATE NÚT VIẾT DÀN Ý
═══════════════════════════════════════════════════════

CONTEXT:
- Sprint 9 đã xong và đang chạy thật: agent loop gọi tool, super json project-global, SQLite.
  Đọc `docs/plan/sprint-09.md` để biết state, và `docs/design/super-json.md` cho shape của super json.
- Backend hiện có: `paths.rs` (`MERLINK_HOME` override), `db.rs` (3 bảng projects/sessions/messages),
  `super_json.rs` (25 leaf path, 11 required, validator), `tools.rs` (4 tool: echo, super_json_get,
  super_json_set, super_json_missing), `adapters.rs` (`LlmBackend` + `HttpBackend` + `ScriptedBackend`),
  `runtime.rs` (agent loop 5 vòng, `run_agent_turn`), `lib.rs` (6 command), `test_support.rs`
  (`TempHome` + `RequiredHome` cho test, `ENV_LOCK` chống race vì `MERLINK_HOME` là biến toàn process).
- Frontend: `Chat.tsx` hardcode `PROJECT_ID = "default"`, đã render tool chip + panel Super JSON,
  sinh `session_id` mỗi session bằng `crypto.randomUUID()`.
- Endpoint test: `http://127.0.0.1:8045/v1`, api_key `local-dev-placeholder`, model `gpt-4o`.
  Config đã có sẵn ở `/tmp/merlink-e2e/config.json` (JSON ARRAY đúng format `Vec<ProviderConfig>`).
- Đọc `docs/design/idea.md` §đoạn 1 và §đoạn 5 trước khi code — đó là nguồn yêu cầu của sprint này.

DECISIONS ĐÃ CHỐT (không hỏi lại, không tự đổi — đây là spec, không phải gợi ý):
- **D1 — Session là nguồn sự thật duy nhất cho "chat này thuộc project nào".**
  Thêm `db::project_for_session(session_id) -> Result<Option<String>, String>` (đọc `sessions.project_id`)
  và `db::bind_session_to_project(session_id, project_id)` (UPDATE).
  Session chưa gắn project → `sessions.project_id = ''` (chuỗi rỗng, cột là NOT NULL).
  `ensure_session` phải nhận project_id rỗng được, và khi rỗng thì **KHÔNG** gọi `ensure_project`.
- **D2 — Project chưa gắn thì không có super json.** Khi session chưa bind project, gọi tool
  `super_json_*` phải trả `Err` với thông điệp bắt đầu bằng
  `No project yet. Call project_create first.`
- **D3 — Tool thứ 5: `project_create`.** Args: `{"name": string}` (tên tác phẩm, non-empty).
  Hành vi, theo đúng thứ tự:
  1. reject nếu name rỗng/sai kiểu.
  2. **reject nếu session này đã bind project rồi** → `Err("This chat already belongs to project <id>")`.
     (Chống tạo project trùng khi model gọi lại.)
  3. sinh `project_id` = slug của name: lowercase, mọi ký tự không phải `[a-z0-9]` → `-`, gộp `-` liền nhau,
     cắt `-` ở hai đầu; rỗng thì dùng `"project"`. Nếu id đã tồn tại trong bảng `projects` thì thêm hậu tố
     `-2`, `-3`, ... cho tới khi duy nhất. Slug phải qua được `paths::safe_segment` (chỉ `[a-z0-9-]`).
  4. `db::create_project(id, name)` + tạo `super.json` từ skeleton cho project đó.
  5. set `require.overview.name = name` luôn (để không hỏi lại tên tác phẩm lần nữa).
  6. `bind_session_to_project(session_id, id)`.
  7. trả `{"ok": true, "project_id": id, "name": name, "missing_required": [...]}`.
- **D4 — `send_chat` đổi kiểu trả về.** Thay `Vec<MessageEnvelope>` bằng struct `ChatTurn`:
  `{ project_id: String, messages: Vec<MessageEnvelope> }`, trong đó `project_id` là project HIỆN TẠI
  của session sau lượt đó (chuỗi rỗng nếu vẫn chưa có). Frontend dùng nó để rebind, không đoán.
- **D5 — Project hiệu lực được tính LẠI TRƯỚC MỖI TOOL CALL trong cùng một lượt**, không tính một lần
  cho cả lượt. Nếu không thì `project_create` ở vòng 1 sẽ không có tác dụng cho các tool ở vòng 2.
- **D6 — Công thức tiến độ (đúng theo `idea.md` §đoạn 5, hai chặn 70% và 100%):**
  `filled_required` = số path trong 11 required có giá trị khác `null`.
  `filled_optional` = số path trong 14 optional có giá trị khác `null`; **mảng rỗng `[]` tính là CHƯA điền**.
  `percent = round(70 * filled_required / 11 + 30 * filled_optional / 14)`.
  Nhãn (`label`):
  - `filled_required == 11` và `filled_optional == 14` → `"Độ mạch lạc: tốt"`
  - `filled_required == 11` → `"Độ mạch lạc tạm chấp nhận"`
  - còn lại → `"Đang thu thập thông tin"`
  - chưa có project → `percent = 0`, label `"Chưa có project"`, mọi count = 0.
  Như vậy đúng 11/11 required mà 0 optional → **percent = 70** và nhãn "Độ mạch lạc tạm chấp nhận".
- **D7 — Command mới `super_json_progress(session_id) -> SuperJsonProgress`**, tính ở Rust
  (nguồn sự thật duy nhất), frontend chỉ render. Struct trả về:
  `{ has_project: bool, project_id: String, percent: u8, label: String,
     filled_required: u32, required_total: u32, filled_optional: u32, optional_total: u32 }`.
- **D8 — Nút "Viết dàn ý"** sáng khi `filled_required == required_total` (tức percent >= 70).
  Bấm vào thì **gửi một user message y hệt như người dùng tự gõ**: `"Bắt đầu viết dàn ý."` qua đúng
  pipeline chat hiện có. KHÔNG viết logic sinh dàn ý thật ở sprint này.
- **D9 — Thanh tiến độ nằm ở header của trang Chat** (trong `.page-topbar`), 2 chặn như idea.md:
  phần 0→70% và phần 70→100% phải nhìn phân biệt được, kèm nhãn `label` và số `filled_required/11`.
  Style đặt trong `src/index.css` theo đúng pattern comment `/* Sprint N · ... */` đang dùng.
- **D10 — Không thêm dependency nào.** `rusqlite` vẫn là dep duy nhất được thêm ở Sprint 9.

GOAL:
Người dùng mở app, gõ tin nhắn đầu tiên khi chưa có project nào → agent tự gọi `project_create` và đặt tên
project từ chính prompt đó; thanh tiến độ ở header chạy từ 0% lên đúng 70% khi đủ 11 required và lên 100%
khi đủ cả 25; nút "Viết dàn ý" chỉ sáng khi đủ 11 required.

TASKS:
1. Pre-flight: `curl` `/v1/models` và 1 lần `/v1/chat/completions` có `tools` tới `127.0.0.1:8045/v1`,
   dán output thật vào báo cáo. Endpoint chết → STOP, báo BLOCKED.
2. `src-tauri/src/db.rs`: thêm `create_project`, `project_for_session`, `bind_session_to_project`,
   `list_projects`. Sửa `ensure_session` để nhận `project_id` rỗng và bỏ qua `ensure_project` khi rỗng.
   (Hiện `ensure_project` đang được gọi trong `run_agent_turn` — xem task 5.)
3. `src-tauri/src/super_json.rs`: thêm `filled_counts(project_id) -> Result<(u32, u32), String>`
   (required đã điền, optional đã điền; mảng rỗng = chưa điền) và `progress(project_id) -> Result<SuperJsonProgress, String>`.
   Định nghĩa `SuperJsonProgress` trong `models.rs` (serde snake_case, khớp D7).
4. `src-tauri/src/tools.rs`: thêm `project_create` vào `TOOL_NAMES` (thành 5) và `tool_specs()`;
   implement đúng D3. `super_json_*` trả lỗi theo D2 khi chưa có project.
5. `src-tauri/src/runtime.rs`:
   - `run_agent_turn` tính `effective_project_id` **trước mỗi lần gọi `tools::execute_tool`** (D5),
     và dùng nó cho mọi việc liên quan project (system prompt, kết quả trả về).
   - Không gọi `ensure_project` khi session chưa bind project.
   - `assemble_prompt` thêm placeholder `{project_state}`; khi chưa có project thì inject
     `Project: (none yet - call project_create once the author says what they want to write)`,
     khi có rồi thì inject `Project: <id>`.
   - `send_chat_internal` trả `ChatTurn` (D4).
   - Thêm `super_json_progress_for_session(session_id)` cho command ở task 6.
6. `src-tauri/src/lib.rs`: `send_chat` trả `models::ChatTurn`; thêm command
   `super_json_progress(session_id) -> models::SuperJsonProgress`. Cập nhật test cũ cho khớp.
7. `src-tauri/prompts/harness_system.md`: thêm contract cho `project_create` — khi nào gọi (khi chưa có
   project VÀ tác giả đã nói muốn viết gì), tự đặt tên project từ chính điều tác giả vừa nói, chỉ gọi MỘT lần,
   và mô tả thanh tiến độ 70%/100%. Giữ nguyên 2 dòng identity và **giữ đủ 25 path** đang liệt kê
   (test `test_system_prompt_documents_every_valid_path` sẽ fail nếu thiếu).
8. `src/types.ts`: thêm `ChatTurn`, `SuperJsonProgress`; `send_chat` giờ trả `ChatTurn`.
9. `src/lib/tauri.ts`: mock `send_chat` trả `ChatTurn`, thêm mock `super_json_progress`, và cho mock
   `project_create` hoạt động trong browser mode (bind project giả, đổi `browserSuperJson`).
10. `src/pages/Chat.tsx`: bỏ `PROJECT_ID = "default"` hardcode → state `projectId` khởi tạo `""`,
    cập nhật từ `ChatTurn.project_id` sau mỗi lượt; gọi `super_json_progress` khi mount và sau mỗi lượt;
    render thanh tiến độ 2 chặn ở header; nút "Viết dàn ý" disabled tới khi đủ 11 required, bấm thì gửi
    user message `"Bắt đầu viết dàn ý."` qua đúng `handleSend`.
11. `src/index.css`: thêm block `/* Sprint 10 · super json progress */` cho thanh tiến độ 2 chặn.
12. Viết test (BẮT BUỘC) đúng các tên sau, tất cả dùng `crate::test_support::TempHome`:
    `test_project_create_tool_binds_session_and_seeds_super_json`,
    `test_project_create_rejects_second_project_for_same_session`,
    `test_project_create_slug_is_unique_and_path_safe`,
    `test_super_json_tools_fail_without_project`,
    `test_progress_is_seventy_when_required_complete`,
    `test_progress_is_hundred_when_all_fields_filled`,
    `test_progress_counts_empty_array_as_unfilled`,
    `test_progress_label_thresholds`,
    `test_agent_loop_creates_project_on_first_message`,
    `test_agent_loop_second_project_creation_is_rejected`.
13. Chạy verify theo ACCEPTANCE, thu log thật, dọn process/port sau khi xong.

ACCEPTANCE:
- [ ] `cd src-tauri && cargo test` → PASS toàn bộ, 0 warning, gồm 10 test mới + test cũ đã cập nhật.
- [ ] `cargo test` vẫn KHÔNG tạo file nào trong `~/.merlink` thật.
- [ ] `test_project_create_slug_is_unique_and_path_safe`: tạo 2 project cùng tên → id thứ hai có hậu tố,
      và cả hai id đều qua được `paths::safe_segment`. Tên có ký tự tiếng Việt/dấu câu → slug sạch.
- [ ] `test_progress_is_seventy_when_required_complete`: điền đúng 11 required, 0 optional →
      `percent == 70`, `label == "Độ mạch lạc tạm chấp nhận"`.
- [ ] `test_progress_is_hundred_when_all_fields_filled`: điền đủ 25 → `percent == 100`,
      `label == "Độ mạch lạc: tốt"`.
- [ ] `test_progress_counts_empty_array_as_unfilled`: `optional.genre.secondary = []` không được tính là đã điền.
- [ ] `test_super_json_tools_fail_without_project`: `super_json_set` khi session chưa bind project →
      `Err` bắt đầu bằng `No project yet. Call project_create first.`
- [ ] `test_agent_loop_creates_project_on_first_message`: ScriptedBackend vòng 1 gọi `project_create`,
      vòng 2 trả text → `ChatTurn.project_id` khác rỗng, `sessions.project_id` đã bind, và
      `projects/<id>/super.json` tồn tại trên đĩa với `require.overview.name` đã set.
- [ ] `pnpm build` (tsc + vite) → PASS, 0 lỗi type.
- [ ] `MERLINK_HOME=/tmp/merlink-e2e pnpm tauri dev` → compile, cửa sổ sống > 30s, log không panic.
- [ ] **E2E THẬT**: xoá `/tmp/merlink-e2e/projects` và `merlink.db*` trước, rồi chạy test online
      `MERLINK_HOME=/tmp/merlink-e2e cargo test -- --ignored --nocapture real_endpoint` với một test
      được cập nhật/mới: gõ `Tôi muốn viết truyện trinh thám về một thợ đồng hồ mù` khi session CHƯA có project
      → model gọi `project_create` → có thư mục project mới dưới `/tmp/merlink-e2e/projects/`
      với `super.json` chứa `require.overview.name` khác null. Dán output thật vào báo cáo.
- [ ] **E2E THẬT phần tiến độ**: chạy thêm các lượt để điền nốt required, sau đó
      `super_json_progress(session_id)` trả `filled_required == 11` và `percent == 70`
      với label `"Độ mạch lạc tạm chấp nhận"`. Nếu model không điền hết trong 5 lượt thì ghi rõ số lượt
      thực tế và giá trị đạt được, KHÔNG được bịa.
- [ ] Browser mode: `pnpm dev` + mở `http://localhost:1420/#/chat`, gõ 1 tin nhắn → có tool chip,
      thanh tiến độ render, **0 console error / 0 exception**. Chụp ảnh màn hình vào báo cáo.
- [ ] Nút "Viết dàn ý" disabled khi chưa đủ 11 required (kiểm tra bằng DOM `disabled`), enabled sau khi đủ.
- [ ] Sau khi xong: port 1420/1421 free, không còn process `merlink-harness`/`vite` treo.
- [ ] Trong báo cáo có log e2e thật + nội dung `super.json` trên đĩa. KHÔNG bịa kết quả.

───────────────────────────────────────────────────────
🛑 STOP CONDITIONS (DỪNG NGAY khi đạt 1 trong các điều này)
───────────────────────────────────────────────────────
- Tất cả ACCEPTANCE pass → DỪNG, báo "SPRINT DONE"
- Retry 1 task quá 2 lần mà vẫn fail → DỪNG, escalate
- Phát hiện task cần design decision không có trong spec → DỪNG, hỏi
- Cần thêm dependency ngoài list (D10: KHÔNG được thêm gì) → DỪNG, hỏi
- Endpoint `127.0.0.1:8045` không phản hồi → DỪNG, báo BLOCKED
- Model liên tục không chịu gọi `project_create` sau khi đã sửa system prompt 2 lần → DỪNG, báo NEED_INPUT
  kèm raw response
- Task vượt quá 30 phút ước tính → DỪNG, chia nhỏ lại

───────────────────────────────────────────────────────
🔁 RETRY BUDGET (giới hạn số lần thử)
───────────────────────────────────────────────────────
- Mỗi task: tối đa 2 lần thử
- Mỗi lần thử: phải thay đổi approach, KHÔNG được lặp y hệt
- Sau lần 2 fail: DỪNG task đó, chuyển sang task khác
- Cuối sprint: list ra các task chưa xong + lý do

───────────────────────────────────────────────────────
🚨 ESCALATION TRIGGERS (khi nào hỏi user)
───────────────────────────────────────────────────────
DỪNG và hỏi user khi:
1. Lỗi không rõ nguyên nhân sau 2 lần debug
2. Cần chọn giữa 2 approach (A vs B) mà spec không nói
3. Cần thay đổi kiến trúc ngoài D1–D10 (folder structure, store shape, đổi tên bảng/cột)
4. Phát hiện bug ở sprint trước ảnh hưởng sprint này
5. Cần user test thủ công (visual, UX, performance)
6. Nảy sinh yêu cầu gì THÊM về session (list/resume/delete/rename) — user CHƯA chốt phần này, sprint này
   chỉ dùng session như khoá bind project

───────────────────────────────────────────────────────
🚫 OUT OF SCOPE (KHÔNG được làm)
───────────────────────────────────────────────────────
- File được phép sửa/tạo: `src-tauri/src/{db,super_json,tools,runtime,lib,models}.rs`,
  `src-tauri/prompts/harness_system.md`, `src/types.ts`, `src/lib/tauri.ts`, `src/pages/Chat.tsx`,
  `src/index.css`. KHÔNG sửa file nào trong `docs/`.
- KHÔNG viết logic sinh dàn ý / sinh arc / sinh chapter thật — nút chỉ gửi 1 user message.
- KHÔNG làm dialog gợi ý option (skip/submit) — sprint khác.
- KHÔNG làm UI quản lý danh sách project, không thêm route/trang mới.
- KHÔNG làm session list/resume/rename/delete.
- KHÔNG port `docs/schema/system.dbml` sang SQLite.
- KHÔNG làm bộ tool random tên (`geneRandFirstName` / `geneRandLastName`).
- KHÔNG đổi shape của `docs/schema/runtime/super.json` hay danh sách 25 path.
- KHÔNG thêm feature ngoài GOAL, KHÔNG đổi UI/UX ngoài D9, KHÔNG optimize premature,
  KHÔNG viết test ngoài 10 test ở task 12 + test online ở ACCEPTANCE, KHÔNG update README/docs.

───────────────────────────────────────────────────────
📤 HANDOFF FORMAT (khi DỪNG, báo cáo theo format này)
───────────────────────────────────────────────────────

STATUS: [DONE | PARTIAL | BLOCKED | NEED_INPUT]
SPRINT: 10

✅ COMPLETED:
- [task 1]: [file changed]
- [task 2]: [file changed]

❌ FAILED (nếu có):
- [task]: [lỗi gì] | [đã thử gì] | [cần gì từ user]

🤔 NEED_INPUT (nếu có):
- Câu hỏi 1: [mô tả] | Option A: [...] | Option B: [...]
- Câu hỏi 2: ...

📋 NEXT ACTION:
- Nếu DONE → "Ready for sprint 11, paste next prompt"
- Nếu BLOCKED → "Waiting for user response on X"
- Nếu PARTIAL → "Continue task Y after user confirms"

═══════════════════════════════════════════════════════
