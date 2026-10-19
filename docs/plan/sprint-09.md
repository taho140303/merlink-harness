═══════════════════════════════════════════════════════
SPRINT 9 — AGENT SỐNG TRONG APP: TOOL-CALL LOOP + SUPER JSON + SQLITE
═══════════════════════════════════════════════════════

CONTEXT:
- Trạng thái hiện tại (hết Sprint 8a): frontend React port xong Home/Characters; `src/pages/Chat.tsx` đã gọi
  `safeInvoke("send_chat")` với provider/model select hoạt động; `src/lib/tauri.ts` có mock cho browser.
- Backend Rust đã có sẵn: `store.rs` (provider config JSON tại `~/.merlink/config.json`, atomic write),
  `adapters.rs` (`call_openai` + `call_anthropic`, CHƯA có tools), `runtime.rs` (`send_chat` 1 lượt duy nhất,
  `execute_tool_internal` chỉ có `echo`), `models.rs`, `lib.rs` (4 command).
- CHƯA có: SQLite, super json, tool registry thật, agent loop, render tool call ở UI.
- **Super JSON ĐÃ HOÀN CHỈNH ở Sprint 8.5.** Nguồn sự thật:
  `docs/schema/runtime/super.json` (skeleton máy đọc) + `docs/design/super-json.md` (đặc tả + lý do
  require/optional + luật validate). ĐỌC CẢ HAI trước khi code. KHÔNG tự sáng tác field mới.
- Provider config VẪN là JSON — KHÔNG chuyển provider sang SQLite. SQLite CHỈ chứa dữ liệu runtime.

⚠️ ENDPOINT TEST ĐÃ CHẠY SẴN (không cần tự mò lại, nhưng phải verify lại 1 lần ở task 1):
- Endpoint: `http://127.0.0.1:8045/v1` (OpenAI-compatible), api_key `local-dev-placeholder`.
  Đây là dev key của localhost — KHÔNG commit file config chứa key này nếu sau này `git init`.
- `GET /v1/models` → 200, trả về list (gpt-4o, claude-sonnet-4-5, gemini-3-pro, ...). Dùng `gpt-4o` làm default.
- `POST /v1/chat/completions` với param `tools` → 200, trả `finish_reason:"tool_calls"` và `message.tool_calls`.
  → **Endpoint support tool-calling đầy đủ.**
- Vòng 2 (gửi lại assistant `tool_calls` + message `role:"tool"`) → 200, `finish_reason:"stop"`,
  content `"Đã đặt tên project thành công là **Nửa Đêm**."` → **Agent loop 2 vòng chạy được thật.**
- 3 điều bắt buộc rút ra từ test:
  (a) Khi KHÔNG đưa danh sách path hợp lệ, model bịa path `project.name` → tool trả Err.
      Vì vậy `path` của `super_json_set` PHẢI là JSON Schema `enum` gồm đúng 25 path hợp lệ (xem D4).
  (b) Response có field `reasoning_content` và `content` có thể rỗng `""` khi có tool_calls.
      Parser phải bỏ qua field lạ và KHÔNG được coi content rỗng là lỗi.
  (c) Model có xu hướng set `arc.count` thành chuỗi `"5"`. Validator phải ép đúng kiểu số nguyên.

DECISIONS ĐÃ CHỐT (không hỏi lại, không tự đổi — đây là spec, không phải gợi ý):
- **D1 — SQLite:** `rusqlite` (feature `bundled`). File `~/.merlink/merlink.db`. Chỉ 3 bảng:
  `projects`, `sessions`, `messages`. KHÔNG port `docs/schema/system.dbml` (Postgres/UUIDv7, sprint khác).
- **D2 — Root path override:** mọi đường dẫn dưới `~/.merlink` đi qua `paths::merlink_root()`, đọc env
  `MERLINK_HOME` trước, fallback `$HOME/.merlink`. Bắt buộc, để test + e2e không ghi vào `~/.merlink` thật.
- **D3 — Super JSON là JSON TOÀN CỤC CỦA PROJECT:**
  đường dẫn `~/.merlink/projects/{project_id}/super.json`. KHÔNG có `temp.json`, KHÔNG per-session.
  Skeleton KHÔNG được viết lại trong Rust — phải `include_str!("../../docs/schema/runtime/super.json")`
  để docs và code không bao giờ lệch nhau. AI KHÔNG bao giờ ghi file trực tiếp — chỉ đổi qua tool.
- **D4 — Bộ tool Sprint 9 (đúng 4):** `echo`, `super_json_get`, `super_json_set`, `super_json_missing`.
  Whitelist = **25 leaf path** lấy đúng từ `docs/schema/runtime/super.json`:
  **11 REQUIRED** (`super_json_missing` đếm theo nhóm này):
   1. `require.overview.name`
   2. `require.overview.logline.trigger-event`
   3. `require.overview.logline.main-character`
   4. `require.overview.logline.action`
   5. `require.overview.logline.risk`
   6. `require.overview.logline.major-loss`
   7. `require.overview.plan.arc.count`
   8. `require.overview.plan.chapter.count`
   9. `require.overview.plan.word.count`
   10. `require.genre.primary`
   11. `require.theme.question`
  **14 OPTIONAL:**
   12. `optional.overview.description`        19. `optional.genre.secondary`
   13. `optional.overview.author`             20. `optional.theme.resolution-type`
   14. `optional.overview.language`           21. `optional.theme.resolution`
   15. `optional.overview.target-audience`    22. `optional.theme.message`
   16. `optional.overview.age-rating`         23. `optional.world.has-power-system`
   17. `optional.overview.plan.arc.tolerance` 24. `optional.world.setting-scale`
   18. `optional.overview.plan.chapter.tolerance`  25. `optional.overview.plan.word.tolerance`
  Luật validate (bảng đầy đủ ở `docs/design/super-json.md` §5):
  - string non-empty: `overview.name`, 5 field logline, `genre.primary`, `theme.question`,
    `overview.description`, `overview.author`, `overview.target-audience`, `overview.age-rating`,
    `theme.resolution`, `theme.message`
  - số nguyên dương: `plan.arc.count`, `plan.chapter.count`, `plan.word.count`
  - số nguyên >= 0: `plan.arc.tolerance`, `plan.chapter.tolerance`, `plan.word.tolerance`
  - enum: `overview.language` ∈ {VI,EN,JA,ZH_HANT,ZH_HANS}; `theme.resolution-type` ∈ {RESOLVED,OPEN,AMBIGUOUS};
    `world.setting-scale` ∈ {REALM,WORLD,REGION,COUNTRY,CITY,TOWN,FACILITY,STREET,ROOM}
  - boolean: `world.has-power-system`
  - array<string>: `genre.secondary`
  Path lạ → `Err`, KHÔNG tạo field mới. Sai kiểu → `Err` (đặc biệt `"5"` phải bị từ chối, không auto-cast).
- **D5 — Trait `LlmBackend` trong `adapters.rs`:** `HttpBackend` cho production và `ScriptedBackend`
  CHỈ tồn tại dưới `#[cfg(test)]`. Nhánh "[Demo mode]" khi `api_key` rỗng GIỮ NGUYÊN hành vi (text, không tool) —
  KHÔNG ship AI giả vào app thật. Dùng `Pin<Box<dyn Future>>` thủ công, KHÔNG thêm `async-trait` / `futures`.
- **D6 — Wire format:** OpenAI → `tools` + `tool_calls` + message `role:"tool"`.
  Anthropic → block `tool_use` / `tool_result`. Chuẩn hoá cả hai về `ToolCall { id, name, arguments }`.
- **D7 — `send_chat(...)` trả `Vec<MessageEnvelope>`** = toàn bộ message append trong lượt đó
  (assistant tool_calls + message `role:"tool"` + assistant cuối). Frontend append cả mảng.
- **D8 — Session: CHƯA CÓ HƯỚNG XỬ LÝ (user xác nhận).** Sprint 9 chỉ cần `session_id` là chuỗi opaque
  do frontend sinh ra, dùng làm khoá ngoài cho bảng `messages`. KHÔNG tạo thư mục riêng cho session,
  KHÔNG làm title / resume / list / delete session. Nếu gặp quyết định gì THÊM về session → DỪNG, hỏi.

GOAL:
Gõ một câu trong Chat → model thật (endpoint `127.0.0.1:8045`) gọi tool `super_json_set` →
`~/.merlink/projects/{project_id}/super.json` đổi đúng field đúng kiểu, và toàn bộ message/tool-call được lưu
vào `~/.merlink/merlink.db`, verify được bằng e2e thật + `cargo test` offline.

TASKS:
1. Pre-flight verify lại endpoint 1 lần: `curl` `/v1/models` và 1 lần `/v1/chat/completions` có `tools`,
   ghi lại output thật vào báo cáo. Nếu endpoint chết → STOP, báo BLOCKED.
2. Chạy `cargo add rusqlite --features bundled` trong `src-tauri/`. Không thêm dep nào khác.
   Nếu fail vì registry offline → STOP ngay, báo BLOCKED, KHÔNG tự chế storage giả.
3. Tạo `src-tauri/src/paths.rs`: `merlink_root()` (theo D2), `db_path()`, `config_path()`,
   `project_dir(project_id)`, `super_json_path(project_id)`. Unit test cho `MERLINK_HOME`.
4. Sửa `src-tauri/src/store.rs`: `get_config_path()` gọi `paths::merlink_root()`. Không sửa gì khác.
5. Tạo `src-tauri/src/db.rs`: `open()` (tạo thư mục cha, WAL, foreign_keys=ON, CREATE TABLE IF NOT EXISTS),
   `ensure_project`, `ensure_session`, `append_message`, `list_messages`, `count_messages`.
   Schema: `projects(id TEXT PK, title TEXT, created_at TEXT)`;
   `sessions(id TEXT PK, project_id TEXT, title TEXT, created_at TEXT)`;
   `messages(id TEXT PK, session_id TEXT, role TEXT, content TEXT, payload_json TEXT, created_at TEXT)`.
6. Tạo `src-tauri/src/super_json.rs`: `skeleton()` đọc bằng
   `include_str!("../../docs/schema/runtime/super.json")` + `serde_json::from_str`;
   `VALID_PATHS` (const 25 path, tách `REQUIRED_PATHS` 11 phần tử);
   `load_or_init(project_id)`, `get(project_id, path)`, `set(project_id, path, value)` (validate đủ bảng luật ở D4),
   `missing_required(project_id)`, `save_atomic()` (tmp + rename như `store.rs`).
7. Sửa `src-tauri/src/models.rs`: thêm `ToolCall { id, name, arguments: serde_json::Value }`,
   `ToolInvocation { id, name, arguments, result, ok, error }`; thêm vào `MessageEnvelope` các field optional
   `tool_calls: Option<Vec<ToolCall>>`, `tool_call_id: Option<String>`, `name: Option<String>` —
   tất cả `#[serde(default, skip_serializing_if = "Option::is_none")]` để payload cũ vẫn parse.
8. Tạo `src-tauri/src/tools.rs`: `tool_specs() -> Vec<serde_json::Value>` (JSON Schema kiểu OpenAI cho 4 tool,
   `path` của `super_json_set` là `enum` đủ 25 path, kèm `description` nói rõ required hay optional) và
   `execute_tool(name, args, project_id, session_id) -> Result<serde_json::Value, String>`.
   Tool lạ → `Err` kèm danh sách tool hợp lệ.
9. Sửa `src-tauri/src/adapters.rs`: trait `LlmBackend` + `HttpBackend` (D5); truyền `tools` vào request;
   parse `tool_calls` (OpenAI) và `tool_use` (Anthropic) thành `Vec<ToolCall>`; map history nội bộ sang wire
   format. `AdapterReply` thêm `tool_calls: Vec<ToolCall>`, cho phép `content` rỗng khi có tool call,
   bỏ qua field lạ như `reasoning_content` (đừng dùng `deny_unknown_fields`).
10. Sửa `src-tauri/src/runtime.rs`: agent loop tối đa 5 vòng — gọi backend → có tool_calls thì persist assistant
    message, execute từng tool, persist message `role:"tool"` (lỗi tool trả JSON `{"error": ...}`, KHÔNG fail cả
    lượt), lặp → không còn tool_calls thì persist assistant cuối và return `Vec<MessageEnvelope>`.
    Chạm trần → content cuối kết thúc bằng `[stopped: reached 5 tool rounds]`. Persist cả user message.
    `execute_tool_internal` route sang `tools.rs`.
11. Sửa `src-tauri/src/lib.rs`: `send_chat(provider_id, model_id, messages, project_id, session_id)
    -> Vec<MessageEnvelope>`; thêm command `super_json_read(project_id)` và `list_messages(session_id)`;
    giữ `load_providers` / `save_providers` / `execute_tool`. Cập nhật test cũ cho khớp signature mới.
12. Sửa `src-tauri/prompts/harness_system.md`: thêm tool contract — tên 4 tool, **inject đủ 25 path kèm nhãn
    required/optional và kiểu dữ liệu**, nhấn mạnh `plan.*.count` phải là SỐ không phải chuỗi, luôn gọi
    `super_json_set` ngay khi user cung cấp giá trị (không trả lời suông), cấm bịa field ngoài whitelist,
    khi user nói "không có ý tưởng" thì đề xuất 3 phương án trong text.
    Giữ nguyên 2 dòng identity `Runtime: {provider_name} / {model_name}` và `Project: {project_id}`.
13. Sửa frontend: `src/types.ts` mirror type mới + `role` thêm `"tool"`; `src/lib/tauri.ts` mock `send_chat`
    (mảng gồm 1 tool-call + 1 tool result + 1 assistant cuối), `super_json_read`, `list_messages`;
    `src/pages/Chat.tsx` sinh `session_id` một lần cho mỗi session (`crypto.randomUUID()` + fallback,
    New chat → id mới), truyền `session_id`, append cả mảng trả về, render message `role:"tool"` thành
    chip gọn `tool · super_json_set → ok`, thêm panel collapsible "Super JSON" gọi `super_json_read` + nút
    refresh. Dùng inline style / class có sẵn, KHÔNG tạo file CSS mới.
14. Viết config test: tạo `/tmp/merlink-e2e/config.json` là **JSON ARRAY** (đúng format `Vec<ProviderConfig>`
    mà `store.rs` đọc) chứa 1 provider:
    `{"id":"agtron","name":"Agtron (local)","kind":"openai","api_key":"<LOCAL_DEV_KEY>",
      "base_url":"http://127.0.0.1:8045/v1","models":[{"id":"gpt-4o","name":"gpt-4o",
      "context_window":"128K","max_tokens":"4K","input_types":{"text":true,"image":false},
      "reasoning_level":false}],"default_model_id":"gpt-4o"}`
15. Viết test (BẮT BUỘC, override rule "không viết test") đúng tên:
    `test_merlink_root_honours_env`, `test_super_json_skeleton_and_set_round_trip`,
    `test_super_json_rejects_unknown_path`, `test_super_json_required_path_count_is_eleven`,
    `test_super_json_rejects_wrong_type`, `test_super_json_enum_paths_reject_invalid_value`,
    `test_db_message_round_trip_and_tables`, `test_agent_loop_executes_tool_then_finishes`,
    `test_agent_loop_caps_at_max_rounds`, `test_openai_parses_tool_calls_with_empty_content`.
    Tất cả set `MERLINK_HOME` vào thư mục tạm.
16. Chạy verify theo ACCEPTANCE, thu log thật, dọn process/port sau khi xong.

ACCEPTANCE:
- [ ] `cd src-tauri && cargo test` → PASS toàn bộ, gồm 10 test mới + test cũ đã cập nhật.
- [ ] `cargo test` KHÔNG tạo/ghi file nào trong `~/.merlink` thật.
- [ ] `test_super_json_required_path_count_is_eleven` assert `missing_required()` trên file mới tạo trả ĐÚNG
      11 path, khớp đúng danh sách ở D4.
- [ ] `test_super_json_rejects_unknown_path`: `require.overview.bogus` → `Err`; thêm case `project.name`
      (path model đã từng bịa) → `Err`; sau lỗi file không có field mới.
- [ ] `test_super_json_rejects_wrong_type`: `plan.arc.count` nhận `"5"` (chuỗi) → `Err`; nhận `5` → OK;
      nhận `-1` → `Err`.
- [ ] `test_super_json_enum_paths_reject_invalid_value`: `theme.resolution-type = "MAYBE"` → `Err`;
      `world.setting-scale = "PLANET"` → `Err`; `overview.language = "VI"` → OK.
- [ ] `test_openai_parses_tool_calls_with_empty_content`: content `""` + có `tool_calls` → vẫn parse ra
      `ToolCall`, không trả Err.
- [ ] `skeleton()` trong Rust == `docs/schema/runtime/super.json` (assert round-trip serialize khớp file).
- [ ] `pnpm build` (tsc + vite) → PASS, 0 lỗi type.
- [ ] `pnpm tauri dev` (chạy với `MERLINK_HOME=/tmp/merlink-e2e`) → compile được, cửa sổ sống > 30s, không panic.
- [ ] **E2E THẬT**: provider Agtron / model `gpt-4o`, gõ `Đặt tên project là Nửa Đêm` → chip tool
      `super_json_set`; panel Super JSON hiện `require.overview.name = "Nửa Đêm"`;
      `/tmp/merlink-e2e/projects/default/super.json` trên đĩa chứa đúng giá trị. Dán response thô vào báo cáo.
- [ ] **E2E THẬT, field mới 10**: gõ `Thể loại là trinh thám` → `require.genre.primary` được set.
- [ ] **E2E THẬT, field mới 11**: gõ `Câu hỏi chủ đề là sức mạnh có tha hoá con người không` →
      `require.theme.question` được set.
- [ ] **E2E THẬT, kiểu số**: gõ `Chia truyện thành 5 arc` → `require.overview.plan.arc.count == 5`
      **dạng number**, không phải chuỗi `"5"`.
- [ ] **E2E THẬT, tool lỗi không làm chết lượt**: gọi `execute_tool` trực tiếp với path không hợp lệ →
      trả `{"error": ...}`, lượt chat vẫn kết thúc bằng 1 assistant message bình thường, app không crash.
- [ ] `super_json_missing` sau khi fill 1 required → trả ĐÚNG 10 path còn thiếu.
- [ ] Reload trang trong lúc app chạy → `list_messages` trả đúng transcript đã lưu trong SQLite
      (số message + role khớp, gồm cả message `role:"tool"`).
- [ ] Browser mode (`pnpm dev`, không Tauri) Chat vẫn chạy bằng mock, không lỗi console.
- [ ] Sau khi fill hết 11 required, `super_json_missing` trả mảng rỗng.
- [ ] Trong báo cáo có log e2e thật (curl output + nội dung `super.json` trên đĩa). Nếu endpoint chết giữa
      chừng thì ghi rõ ở NEED_INPUT, KHÔNG được bịa kết quả.

───────────────────────────────────────────────────────
🛑 STOP CONDITIONS (DỪNG NGAY khi đạt 1 trong các điều này)
───────────────────────────────────────────────────────
- Tất cả ACCEPTANCE pass → DỪNG, báo "SPRINT DONE"
- Retry 1 task quá 2 lần mà vẫn fail → DỪNG, escalate
- Phát hiện task cần design decision không có trong spec → DỪNG, hỏi
- Cần thêm dependency ngoài list đã cho (`rusqlite` là dependency DUY NHẤT được phép thêm) → DỪNG, hỏi
- `cargo add rusqlite` fail vì registry offline → DỪNG ngay, báo BLOCKED.
  KHÔNG được thay bằng JSON-file-giả-sqlite, KHÔNG tự viết storage engine.
- Endpoint `127.0.0.1:8045` không phản hồi → DỪNG, báo BLOCKED (đừng tự thay bằng StubBackend rồi coi là pass)
- Model liên tục trả lời suông mà không gọi tool sau khi đã sửa system prompt 2 lần → DỪNG, báo NEED_INPUT
  kèm raw response (đây là vấn đề prompt, không phải vấn đề code)
- Phát hiện super json thiếu field so với `docs/design/super-json.md` → DỪNG, hỏi. KHÔNG tự thêm field.
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
3. Cần thay đổi kiến trúc hiện tại (folder structure, store shape) — đặc biệt nếu phải đổi shape của super.json
   khác với `docs/design/super-json.md`
4. Phát hiện bug ở sprint trước ảnh hưởng sprint này
5. Cần user test thủ công (visual, UX, performance)
6. Nảy sinh bất kỳ yêu cầu gì THÊM về session (title, resume, list, delete, đổi scope) — user chưa chốt phần này

───────────────────────────────────────────────────────
🚫 OUT OF SCOPE (KHÔNG được làm)
───────────────────────────────────────────────────────
- KHÔNG refactor code ngoài file list ở TASKS. File được phép sửa/tạo:
  `src-tauri/Cargo.toml`, `src-tauri/src/{paths,db,super_json,tools}.rs` (mới),
  `src-tauri/src/{store,models,adapters,runtime,lib}.rs`, `src-tauri/prompts/harness_system.md`,
  `src/types.ts`, `src/lib/tauri.ts`, `src/pages/Chat.tsx`, `/tmp/merlink-e2e/config.json`.
  KHÔNG sửa `docs/schema/runtime/super.json` và `docs/design/super-json.md` (đã chốt ở Sprint 8.5).
  KHÔNG sửa file nào khác trong `docs/`.
- KHÔNG port `docs/schema/system.dbml` / `docs/schema/seed/*` sang SQLite (sprint khác).
- KHÔNG làm bộ tool random tên (`geneRandFirstName` / `geneRandLastName` / bảng `FirstNameCharacter`) — sprint khác.
- KHÔNG làm dialog gợi ý option (skip/submit) — sprint khác.
- KHÔNG làm thanh tiến độ 70% / 100% ở header — sprint khác.
- KHÔNG làm auto create project ở tin nhắn đầu tiên — sprint khác (Sprint 9 chỉ chạy trên `project_id = "default"`).
- KHÔNG sinh `characters` / `world_settings` / `lore_entries` / `power_systems` từ super json — sprint khác.
- KHÔNG làm bất cứ thứ gì về session ngoài D8.
- KHÔNG thêm feature ngoài GOAL, KHÔNG đổi UI/UX ngoài spec, KHÔNG optimize premature,
  KHÔNG viết test ngoài 10 test ở task 15, KHÔNG update README/docs.

───────────────────────────────────────────────────────
📤 HANDOFF FORMAT (khi DỪNG, báo cáo theo format này)
───────────────────────────────────────────────────────

STATUS: [DONE | PARTIAL | BLOCKED | NEED_INPUT]
SPRINT: 9

✅ COMPLETED:
- [task 1]: [file changed]
- [task 2]: [file changed]

❌ FAILED (nếu có):
- [task]: [lỗi gì] | [đã thử gì] | [cần gì từ user]

🤔 NEED_INPUT (nếu có):
- Câu hỏi 1: [mô tả] | Option A: [...] | Option B: [...]
- Câu hỏi 2: ...

📋 NEXT ACTION:
- Nếu DONE → "Ready for sprint 10, paste next prompt"
- Nếu BLOCKED → "Waiting for user response on X"
- Nếu PARTIAL → "Continue task Y after user confirms"

═══════════════════════════════════════════════════════
