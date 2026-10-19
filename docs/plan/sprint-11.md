═══════════════════════════════════════════════════════
SPRINT 11 — STREAMING PHASE + STATUS MESSAGE + ĐO THỜI GIAN TOOL
═══════════════════════════════════════════════════════

CONTEXT:
- Sprint 10 đã xong: agent tự tạo project, thanh tiến độ 70%/100%, gate nút "Viết dàn ý".
  Đọc `docs/plan/sprint-10.md` và `docs/plan/sprint-09.md` để biết state.
- Backend hiện có: `runtime::run_agent_turn(backend, providers, provider_id, model_id, messages,
  project_id, session_id) -> ChatTurn` chạy agent loop tối đa 5 vòng; `tools::execute_tool`;
  `adapters::{LlmBackend, HttpBackend, ScriptedBackend}`; `test_support::{TempHome, RequiredHome, ENV_LOCK}`.
- **VẤN ĐỀ NGƯỜI DÙNG BÁO (nguyên văn):** "nó bị lag á, có thể chia ra thành từng phase, rồi trên giao diện
  loading theo từng phase được ko, kiểu như đang ghi nhận tên truyện, việc gọi bất kì func sẽ kèm theo status
  message, nó cập nhật như thể đang stream vậy á, với phân mảnh thời gian gọi func ra, mỗi func chỉ ghi nhận
  1 thuộc tính, với lại đặc thử thời gian xem để t xem hàm nào phản hồi bao nhiêu giây"
- **Chẩn đoán, phải giải quyết CẢ HAI phần:**
  (a) *Perception:* `send_chat` là 1 request/response duy nhất. Suốt 20–30s UI chỉ hiện "Generating response",
      rồi đổ ra hết một lần. Người dùng không thấy tiến trình.
  (b) *Sự thật:* log thật cho thấy model gọi `super_json_missing` **lặp lại vô ích** — `super_json_set`
      đã trả `missing_required` ngay trong result. Mỗi vòng thừa là thêm 1 HTTP call tới model (~3–6s).
      Ví dụ log thật của người dùng trong 1 lượt: `project_create` → `super_json_missing` →
      `super_json_set(genre)` → `super_json_set(main-character)` → `super_json_missing` (thừa).
- **Nói thẳng trong báo cáo:** streaming KHÔNG làm lượt chat nhanh hơn, nó chỉ làm tiến trình nhìn thấy được.
  Thứ làm nhanh hơn thật là giảm số vòng gọi model (D8). Báo cáo phải tách bạch 2 việc này, không được
  tuyên bố "đã fix lag" nếu chỉ thêm streaming.
- Endpoint: `http://127.0.0.1:8045/v1`, api_key `local-dev-placeholder`, model `gpt-4o`.
  Config ở `/tmp/merlink-e2e/config.json` (JSON ARRAY đúng format `Vec<ProviderConfig>`).

DECISIONS ĐÃ CHỐT (không hỏi lại, không tự đổi — đây là spec, không phải gợi ý):
- **D1 — Transport: Tauri `Channel`.** `tauri::ipc::Channel<AgentEvent>` truyền thẳng qua tham số của
  command `send_chat`, KHÔNG dùng global event (`app.emit` + `listen`) — vì listener toàn cục dễ rò rỉ và
  nhân đôi handler khi React StrictMode mount 2 lần. `Channel` là core của Tauri v2, **không cần dependency
  mới** và không cần permission mới trong `capabilities/default.json`.
- **D2 — Event shape.** Thêm vào `models.rs`:
  ```rust
  #[derive(Clone, Debug, Serialize)]
  #[serde(tag = "type", rename_all = "snake_case")]
  pub enum AgentEvent {
      TurnStarted   { session_id: String, round_budget: u32 },
      RoundStarted  { round: u32 },
      ToolStarted   { id: String, name: String, path: Option<String>, status: String },
      ToolFinished  { id: String, name: String, ok: bool, duration_ms: u64, status: String },
      RoundFinished { round: u32, duration_ms: u64, tool_calls: u32 },
      TurnFinished  { project_id: String, duration_ms: u64, rounds: u32 },
  }
  ```
  Serialize phải ra `{"type":"tool_started", ...}` (tag nội bộ) để TS dựng discriminated union.
- **D3 — Status message tiếng Việt, có map cho từng path.** Thêm `tools::status_message(name, args) -> String`:
  - `project_create` → `"Đang ghi nhận tên truyện và tạo project"`
  - `super_json_missing` → `"Đang kiểm tra các trường còn thiếu"`
  - `super_json_get` → `"Đang đọc siêu JSON"`
  - `super_json_set` → `"Đang ghi nhận " + <nhãn của path>`
  - `echo` → `"Đang kiểm tra kết nối tool"`
  - tool lạ → `"Đang chạy <name>"`
  Map nhãn phải phủ **đủ 25 path** (test sẽ assert), ví dụ:
  `require.overview.name` → `"tên truyện"`, `require.overview.logline.trigger-event` → `"sự kiện khởi động"`,
  `require.genre.primary` → `"thể loại chính"`, `require.theme.question` → `"câu hỏi chủ đề"`,
  `require.overview.plan.arc.count` → `"số arc"`, `optional.world.has-power-system` → `"có hệ thống sức mạnh"`.
  Path không có trong map (không thể xảy ra với 25 path hợp lệ) → fallback chính path đó.
- **D4 — Mỗi lần gọi tool đều phát `ToolStarted` rồi `ToolFinished`** với `status` giống nhau, và
  `ToolFinished` mang `duration_ms` đo bằng `std::time::Instant`.
- **D5 — Đo thời gian ở cả 3 tầng:** mỗi tool call (`ToolFinished.duration_ms`), mỗi vòng gọi model
  (`RoundFinished.duration_ms`), và cả lượt (`TurnFinished.duration_ms`). Đây là dữ liệu người dùng yêu cầu
  ("xem hàm nào phản hồi bao nhiêu giây").
- **D6 — Song song giữ log stderr hiện có.** Dòng `[merlink] tool ...` phải được thêm `ms=<duration_ms>`
  để người dùng đọc log cũng thấy thời gian.
- **D7 — UI hiển thị phase trực tiếp trong lúc chờ.** Trong `loading`, `Chat.tsx` render danh sách phase
  lớn dần theo event: mỗi vòng là 1 mục `"Đang suy nghĩ (vòng N)"` kèm thời gian khi xong; mỗi tool là 1 dòng
  `status` + thời gian (ví dụ `1.2s`), dòng đang chạy có spinner. Khi `ChatTurn` trả về thì **xoá danh sách
  phase** và append message như hiện tại (tool chip đã có sẵn trong transcript, không render trùng 2 lần).
  Cảm giác phải giống streaming: event tới đâu, dòng hiện ra tới đó.
- **D8 — Cắt số vòng gọi model (đây mới là thứ làm nhanh hơn thật).** Sửa `harness_system.md`:
  - `super_json_set` **đã trả về `missing_required`** trong result → **CẤM** gọi `super_json_missing` ngay
    sau một `super_json_set`. Chỉ gọi `super_json_missing` khi thật sự chưa biết trạng thái (ví dụ ngay sau
    khi mở project).
  - Được phép gọi **nhiều `super_json_set` trong CÙNG một vòng** (parallel tool calls), mỗi call vẫn chỉ
    ghi **đúng 1 thuộc tính** — không được gộp nhiều property vào 1 call.
  - Khi người dùng đã cung cấp đủ thông tin trong 1 câu, ghi hết những gì suy ra được ngay trong vòng đó
    thay vì hỏi lại từng cái.
  `run_agent_turn` KHÔNG đổi giới hạn 5 vòng.
- **D9 — Không thêm dependency nào.** Vẫn chỉ `rusqlite` là dep được thêm cho feature area này.
- **D10 — Mock browser phải phát cùng chuỗi event**, có delay nhỏ (`setTimeout`/`await`) để demo được
  cảm giác streaming khi không có Tauri. Mock nhận channel qua arg và gọi `onmessage`.

GOAL:
Trong lúc chờ, UI hiện tiến trình theo từng phase ("Đang ghi nhận tên truyện", "Đang ghi nhận thể loại", …)
cập nhật dần như streaming, mỗi phase có thời gian chạy tính bằng giây; và số vòng gọi model cho cùng một
lượt giảm so với trước nhờ bỏ `super_json_missing` thừa.

TASKS:
1. Pre-flight: `curl` `/v1/models` và 1 lần `/v1/chat/completions` có `tools` tới `127.0.0.1:8045/v1`,
   dán output thật vào báo cáo. Endpoint chết → STOP, báo BLOCKED.
2. `src-tauri/src/models.rs`: thêm `AgentEvent` đúng D2. Thêm test serialize khẳng định có `"type"`.
3. `src-tauri/src/tools.rs`: thêm `status_message(name, args)` + map nhãn 25 path (D3).
4. `src-tauri/src/runtime.rs`:
   - Thêm trait `ProgressSink: Send + Sync { fn emit(&self, event: AgentEvent); }` (cùng style thủ công như
     `LlmBackend`, không thêm `async-trait`), impl `NoopSink` và `RecordingSink` (chỉ `#[cfg(test)]`).
   - `run_agent_turn` nhận thêm `sink: &dyn ProgressSink`. Phát `TurnStarted` đầu lượt, `RoundStarted` trước
     mỗi lần gọi backend, `ToolStarted`/`ToolFinished` quanh mỗi `execute_tool`, `RoundFinished` cuối mỗi vòng,
     `TurnFinished` trước khi return (kể cả nhánh demo mode, nhánh chạm trần 5 vòng, và nhánh lỗi tool).
   - Đo `duration_ms` cho cả 3 tầng theo D5. Thêm `ms=` vào dòng `eprintln!` hiện có (D6).
   - `send_chat_internal` nhận thêm `channel: tauri::ipc::Channel<AgentEvent>` và bọc nó trong 1 `ChannelSink`.
5. `src-tauri/src/lib.rs`: `send_chat` nhận thêm tham số channel; cập nhật test cũ cho khớp signature.
6. `src-tauri/prompts/harness_system.md`: thêm mục quy tắc gọi tool theo D8 (cấm `super_json_missing` sau
   `super_json_set`; cho phép nhiều `super_json_set` trong 1 vòng, mỗi call 1 thuộc tính).
   **Giữ đủ 25 path** (test `test_system_prompt_documents_every_valid_path` sẽ fail nếu thiếu) và giữ 2 dòng identity.
7. `src/types.ts`: thêm discriminated union `AgentEvent` khớp D2.
8. `src/lib/tauri.ts`: mock `send_chat` nhận channel, phát chuỗi event y hệt backend (có delay nhỏ) rồi
   resolve `ChatTurn` (D10).
9. `src/pages/Chat.tsx`: state `livePhases`; đăng ký channel trước khi invoke; append phase khi event tới;
   render danh sách phase trong lúc loading (D7); xoá khi turn xong. Hiện tổng thời gian lượt khi xong.
10. `src/index.css`: block `/* Sprint 11 · agent phase stream */` cho danh sách phase + spinner.
11. Viết test (BẮT BUỘC) đúng các tên sau, tất cả dùng `crate::test_support::TempHome`:
    `test_agent_event_serializes_with_type_tag`,
    `test_status_message_covers_all_25_paths`,
    `test_status_message_for_meta_tools`,
    `test_agent_loop_emits_ordered_events`,
    `test_tool_finished_event_carries_duration`,
    `test_round_finished_event_carries_duration`,
    `test_turn_finished_reports_project_and_rounds`,
    `test_events_are_emitted_before_the_turn_returns`,
    `test_failed_tool_emits_tool_finished_with_ok_false`,
    `test_demo_mode_still_emits_turn_events`.
12. Chạy verify theo ACCEPTANCE, thu log thật, dọn process/port sau khi xong.

ACCEPTANCE:
- [ ] `cd src-tauri && cargo test` → PASS toàn bộ, 0 warning, gồm 10 test mới + test cũ đã cập nhật.
- [ ] `cargo test` vẫn KHÔNG ghi gì vào `~/.merlink` thật (diff trước/sau).
- [ ] `test_status_message_covers_all_25_paths`: với **mọi** path trong `super_json::valid_paths()`,
      `status_message("super_json_set", {"path": p, "value": "x"})` trả chuỗi non-empty, và với các path
      đã có nhãn thì chuỗi phải là tiếng Việt có nghĩa (assert chứa nhãn mong đợi cho ít nhất 5 path mẫu,
      ví dụ `require.overview.name` → chứa `"tên truyện"`).
- [ ] `test_events_are_emitted_before_the_turn_returns`: `RecordingSink` ghi kèm thời điểm;
      `ToolStarted` của tool ĐẦU TIÊN phải được ghi **trước** khi `run_agent_turn` return. (Đây là bằng chứng
      "streaming thật", không phải gom rồi phát một lần cuối.)
- [ ] `test_agent_loop_emits_ordered_events`: thứ tự đúng `TurnStarted → RoundStarted → ToolStarted →
      ToolFinished → RoundFinished → … → TurnFinished`, và số `ToolStarted` == số tool call thực tế.
- [ ] `test_tool_finished_event_carries_duration`: `duration_ms` là số đo thật (>= 0 và với tool có sleep
      giả lập thì > 0) — không được hardcode 0.
- [ ] `test_failed_tool_emits_tool_finished_with_ok_false`: tool lỗi vẫn phát `ToolFinished { ok: false }`
      và lượt chat vẫn kết thúc bình thường.
- [ ] `pnpm build` (tsc + vite) → PASS, 0 lỗi type.
- [ ] `MERLINK_HOME=/tmp/merlink-e2e pnpm tauri dev` → compile, cửa sổ sống > 30s, không panic.
- [ ] **E2E THẬT + ĐO THỜI GIAN**: chạy test online
      `MERLINK_HOME=/tmp/merlink-e2e cargo test -- --ignored --nocapture real_endpoint` với một test
      được cập nhật để in ra **bảng thời gian thật**: mỗi tool call `name / status / ms`, mỗi vòng
      `round / ms / số tool call`, và tổng lượt. Dán nguyên bảng đó vào báo cáo.
- [ ] **ĐO ĐƯỢC MỨC GIẢM SỐ VÒNG (D8)**: chạy cùng một câu lệnh mở project
      (`Tôi muốn viết truyện trinh thám về một thợ đồng hồ mù`) và báo cáo **số vòng gọi model** thực tế
      sau khi sửa prompt, so với baseline 5 tool call / 3 vòng ghi trong CONTEXT ở trên.
      Nếu số vòng KHÔNG giảm thì nói thẳng là không giảm và đưa số liệu — KHÔNG được tô hồng.
- [ ] Browser mode: `pnpm dev` + CDP script (dùng lại `/tmp/cdp-check.mjs`, xem gotcha bên dưới), chụp DOM
      **trong lúc đang chờ** để chứng minh danh sách phase lớn dần (ít nhất 2 ảnh ở 2 thời điểm khác nhau
      cho thấy số dòng phase tăng), và `0 console error / 0 exception`.
- [ ] Verify có ít nhất 1 ảnh cho thấy phase đang chạy có spinner + ảnh sau khi xong cho thấy thời gian từng dòng.
- [ ] Sau khi xong: port 1420/1421 free, không còn process `merlink-harness`/`vite` treo.
- [ ] Báo cáo phải tách rõ: "streaming làm tiến trình nhìn thấy được" vs "giảm số vòng làm nhanh hơn thật",
      kèm số đo cho cả hai. KHÔNG bịa kết quả.

───────────────────────────────────────────────────────
🛑 STOP CONDITIONS (DỪNG NGAY khi đạt 1 trong các điều này)
───────────────────────────────────────────────────────
- Tất cả ACCEPTANCE pass → DỪNG, báo "SPRINT DONE"
- Retry 1 task quá 2 lần mà vẫn fail → DỪNG, escalate
- Phát hiện task cần design decision không có trong spec → DỪNG, hỏi
- Cần thêm dependency ngoài list (D9: KHÔNG được thêm gì, kể cả npm package) → DỪNG, hỏi
- Cần thêm permission vào `capabilities/default.json` → DỪNG, hỏi (Channel core lẽ ra không cần)
- Endpoint `127.0.0.1:8045` không phản hồi → DỪNG, báo BLOCKED
- Phát hiện Tauri `Channel` không dùng được với `safeInvoke` wrapper hiện tại → DỪNG, báo NEED_INPUT kèm
  lỗi thật, KHÔNG tự chuyển sang global event mà không hỏi
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
6. Nảy sinh yêu cầu gì THÊM về session (list/resume/delete/rename) — user CHƯA chốt phần này

───────────────────────────────────────────────────────
🚫 OUT OF SCOPE (KHÔNG được làm)
───────────────────────────────────────────────────────
- File được phép sửa/tạo: `src-tauri/src/{models,tools,runtime,lib}.rs`,
  `src-tauri/prompts/harness_system.md`, `src/types.ts`, `src/lib/tauri.ts`, `src/pages/Chat.tsx`,
  `src/index.css`. KHÔNG sửa file nào trong `docs/`. KHÔNG sửa `src-tauri/capabilities/default.json`,
  `src-tauri/Cargo.toml`, `package.json` (trừ khi STOP condition cho phép hỏi).
- KHÔNG đổi shape `super.json`, danh sách 25 path, hay công thức tiến độ 70%/30%.
- KHÔNG đổi giới hạn 5 vòng của agent loop.
- KHÔNG làm session list/resume/rename/delete.
- KHÔNG làm dialog gợi ý option (skip/submit).
- KHÔNG viết logic sinh dàn ý / arc / chapter thật.
- KHÔNG port `docs/schema/system.dbml` sang SQLite.
- KHÔNG thêm feature ngoài GOAL, KHÔNG đổi UI/UX ngoài D7, KHÔNG optimize premature
  (chỉ đo và cắt vòng theo D8), KHÔNG viết test ngoài 10 test ở task 11, KHÔNG update README/docs.

───────────────────────────────────────────────────────
📤 HANDOFF FORMAT (khi DỪNG, báo cáo theo format này)
───────────────────────────────────────────────────────

STATUS: [DONE | PARTIAL | BLOCKED | NEED_INPUT]
SPRINT: 11

✅ COMPLETED:
- [task 1]: [file changed]
- [task 2]: [file changed]

❌ FAILED (nếu có):
- [task]: [lỗi gì] | [đã thử gì] | [cần gì từ user]

🤔 NEED_INPUT (nếu có):
- Câu hỏi 1: [mô tả] | Option A: [...] | Option B: [...]

📋 NEXT ACTION:
- Nếu DONE → "Ready for sprint 12, paste next prompt"
- Nếu BLOCKED → "Waiting for user response on X"
- Nếu PARTIAL → "Continue task Y after user confirms"

═══════════════════════════════════════════════════════

GHI CHÚ KỸ THUẬT CHO NGƯỜI THỰC THI (không phải phần của spec gửi user):
- `cargo test` chạy song song; `MERLINK_HOME` là biến toàn process → mọi test chạm storage phải giữ
  `crate::test_support::TempHome`.
- `cmd | tail` che mất exit code → dùng `echo "EXIT=${PIPESTATUS[0]}"`.
- `pkill -f "pattern"` tự khớp shell đang chạy lệnh đó → dùng bracket trick `pkill -f "[p]attern"`.
- App dùng `HashRouter`; URL browser mode là `http://localhost:1420/#/chat`.
- `agent-browser` KHÔNG có sẵn. Có sẵn CDP driver ở `/tmp/cdp-check.mjs` (Node 22, không cần dep):
  `node /tmp/cdp-check.mjs "http://localhost:1420/#/chat" /tmp/out.png "tin nhắn"`.
  Nó chụp ảnh + in console error. Muốn bắt ảnh giữa chừng thì sửa script để chụp nhiều lần theo thời gian.
- chromium cần `--no-sandbox` (bản snap).
- SQLite: `rowid` không được xuất hiện trong `CREATE INDEX`; rusqlite không có `FromSql` cho `u64`.
- `name`/`tool_call_id`/`tool_calls` nằm trong cột `payload_json`, không có cột riêng.
- Tauri v2 mặc định chờ key **camelCase** ở phía JS (`tauri-macros` gọi `to_lower_camel_case`). Mọi tham số
  invoke mới phải là camelCase (`sessionId`, `projectId`…) nếu không sẽ lỗi "missing required key".
