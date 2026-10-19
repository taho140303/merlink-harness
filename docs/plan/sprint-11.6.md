═══════════════════════════════════════════════════════
SPRINT 11.6 — IPC CONTRACT TEST + DỌN PROMPT DIR CHẾT
═══════════════════════════════════════════════════════

CONTEXT:
- Sprint 11.5 đã xong (fetch model thật + edit provider 1 tab). Đọc `docs/plan/sprint-11.5.md`,
  `docs/plan/sprint-11.md`, `docs/plan/sprint-10.md` để biết state.
- **Lỗ hổng cần bịt:** đã 3 sprint liên tiếp việc verify chỉ chạy ở browser mode (mock) chứ **chưa bao giờ
  chạy qua tầng IPC thật của Tauri**. Sprint 9 đã trả giá cho đúng lỗ hổng này: frontend gọi `send_chat`
  với key `session_id` trong khi Tauri v2 chờ `sessionId`, khiến chat trên desktop **không gửi được tin
  nhắn nào** mà không ai phát hiện ra.
- Bằng chứng bug đó (đã verify từ source crate): `tauri-macros/src/lib.rs` ghi
  *"Either `camelCase` (default) or `snake_case`"*, và `command/wrapper.rs` gọi `key.to_lower_camel_case()`.
- `src-tauri/prompt/` (KHÔNG có "s") là thư mục chết: 413 bytes, mtime cũ hơn `prompts/`, và
  `runtime.rs:20` dùng `include_str!("../prompts/harness_system.md")` (7683 bytes). Grep toàn repo không có
  tham chiếu nào tới `prompt/`. Cần xoá.
- `tauri::test` CÓ SẴN trong tauri 2.12.3 sau feature `test`, cung cấp `mock_builder`, `mock_app`,
  `mock_context`, `noop_assets`, `get_ipc_response`, `assert_ipc_response`
  (xem `~/.cargo/registry/src/*/tauri-2.12.3/src/test/mod.rs`).
- **Người dùng đã HOÃN refactor lớn** (tụ JSON về 1 chỗ, tách `tools.rs`/`runtime.rs`, chọn design pattern
  cho tool/provider) vì repo **không có git**. Sprint này vì vậy phải **nhỏ, khép kín, KHÔNG tái cấu trúc
  module**. Không được "tiện tay" dọn thêm.

DECISIONS ĐÃ CHỐT (không hỏi lại, không tự đổi):
- **D1 — Chỉ một thay đổi manifest duy nhất được phép:** thêm
  `[dev-dependencies] tauri = { version = "2", features = ["test"] }` vào `src-tauri/Cargo.toml`.
  Dev-dependency KHÔNG được biên dịch vào app release. Không thêm crate nào khác, không đổi
  `package.json`, không đổi `capabilities/default.json`.
- **D2 — Danh sách command phải chia sẻ giữa prod và test bằng `macro_rules!`**, ví dụ
  `macro_rules! merlink_handlers { () => { tauri::generate_handler![...] } }`, dùng ở cả `run()` và ở
  app mock của test. Cấm chép tay danh sách command ra 2 nơi — đó chính là kiểu drift mà sprint này sinh
  ra để chặn.
- **D3 — Test khẳng định HÌNH DẠNG LỖI, không khẳng định thành công**, với các command cần provider/DB sống:
  - Gọi bằng key **camelCase** (đúng) → **KHÔNG** được lỗi `missing required key`.
  - Gọi bằng key **snake_case** (sai) → **PHẢI** lỗi `missing required key`.
    Đây là bài test ghi lại cái bẫy đã gây bug Sprint 9.
- **D4 — Ít nhất 3 command phải chạy end-to-end qua tầng IPC thật và trả giá trị thật**:
  `load_providers`, `super_json_progress`, `list_messages` (đều là tham số đơn giản, chạy được dưới
  `TempHome`). Không được chỉ assert lỗi cho tất cả.
- **D5 — Test chống drift:** đọc chính source của `lib.rs` bằng `include_str!`, trích ra danh sách command
  trong `merlink_handlers!`, và assert bảng contract phủ **đủ** danh sách đó (cả 2 chiều: không thiếu,
  không thừa). Mục đích: thêm command mới mà quên test contract là `cargo test` fail ngay.
- **D6 — Xoá `src-tauri/prompt/`** sau khi đã chứng minh không ai tham chiếu. **KHÔNG** đụng
  `src-tauri/prompts/` (đang sống) và **KHÔNG** đụng `docs/prompts/` (thuộc về người dùng).
- **D7 — Không tái cấu trúc module.** Không tách file, không đổi tên module, không đổi vị trí hàm, không
  đổi hành vi runtime của app. Ngoại lệ duy nhất: hằng/macro `merlink_handlers` ở D2.

GOAL:
`cargo test` bắt được lỗi sai tên tham số IPC (kiểu bug Sprint 9) mà không cần webview, và
`src-tauri/prompt/` biến mất khỏi repo.

TASKS:
1. Chứng minh `src-tauri/prompt/` không được tham chiếu: chạy grep trên toàn repo (kể cả `build.rs`,
   `tauri.conf.json`, `docs/`), dán output thật vào báo cáo. Sau đó `rm -rf src-tauri/prompt/`.
   Kiểm tra lại `src-tauri/prompts/harness_system.md` vẫn còn và `cargo build` vẫn chạy.
2. Thêm dev-dependency theo D1 vào `src-tauri/Cargo.toml`.
3. `src-tauri/src/lib.rs`: thêm `macro_rules! merlink_handlers` (D2) và dùng nó trong `run()`.
4. Tạo `src-tauri/src/ipc_contract_tests.rs` gated `#[cfg(test)]`, khai báo từ `lib.rs` bằng
   `#[cfg(test)] mod ipc_contract_tests;`. Trong đó:
   - một hàm dựng app mock bằng `tauri::test::mock_builder()` + `merlink_handlers!()`;
   - một **bảng contract** liệt kê mỗi command với đúng bộ key camelCase mà frontend đang dùng
     (đối chiếu `src/pages/Chat.tsx` và `src/components/SettingsDialog.tsx` để lấy key thật, không đoán);
   - harness gọi command qua `tauri::test::get_ipc_response` với `InvokeRequest` dựng từ
     `cmd` + body JSON. Đọc source `tauri-2.12.3/src/test/mod.rs` để dùng đúng API.
   - mọi test chạm storage phải dùng `crate::test_support::TempHome`.
5. Viết test (BẮT BUỘC) đúng các tên sau:
   `test_ipc_load_providers_end_to_end`,
   `test_ipc_super_json_progress_end_to_end`,
   `test_ipc_list_messages_end_to_end`,
   `test_ipc_execute_tool_end_to_end`,
   `test_ipc_send_chat_requires_camel_case_session_id`,
   `test_ipc_snake_case_keys_are_rejected`,
   `test_contract_table_covers_every_registered_command`,
   `test_every_contract_entry_uses_camel_case_keys`.
6. `src-tauri/src/lib.rs`: nếu `send_chat` có tham số channel và việc dựng `Channel` phía Rust gây khó,
   thì với riêng `send_chat` chỉ assert **hình dạng lỗi** theo D3 (không cần chạy thành công). Ghi rõ
   trong báo cáo là đã làm vậy và vì sao.
7. Chạy verify theo ACCEPTANCE, thu log thật, dọn process/port.

ACCEPTANCE:
- [ ] `cd src-tauri && cargo test` → PASS toàn bộ, 0 warning, gồm 8 test mới + **63 test cũ vẫn pass**
      (không được để rơi test nào).
- [ ] `cargo test` vẫn KHÔNG ghi gì vào `~/.merlink` thật (diff md5 config.json + mtime trước/sau).
- [ ] `test_ipc_snake_case_keys_are_rejected` chứng minh key snake_case bị từ chối, dán thông điệp lỗi thật.
- [ ] `test_ipc_send_chat_requires_camel_case_session_id` chứng minh `sessionId` đúng thì KHÔNG lỗi
      missing-key, `session_id` thì CÓ. Dán cả 2 output.
- [ ] `test_contract_table_covers_every_registered_command` fail nếu thêm command mới mà không khai báo:
      chứng minh bằng cách **tạm** thêm một command giả vào `merlink_handlers!`, chạy test thấy fail,
      dán output, rồi hoàn nguyên. (Bắt buộc làm bước chứng minh này — test chống drift mà chưa từng thấy
      nó fail thì chưa chứng minh được gì.)
- [ ] `grep -rn "tauri::test" src-tauri/src/` → chỉ xuất hiện trong code `#[cfg(test)]` (hoặc trong
      `ipc_contract_tests.rs`). Không có chỗ nào trong đường production.
- [ ] `cargo build` (không phải test profile) → PASS. App release không kéo feature `test`.
- [ ] `pnpm build` → PASS, 0 lỗi type (không đổi frontend, nhưng phải chứng minh không vỡ).
- [ ] `MERLINK_HOME=/tmp/merlink-e2e pnpm tauri dev` → compile, cửa sổ sống > 30s, không panic.
- [ ] `src-tauri/prompt/` KHÔNG còn tồn tại; `src-tauri/prompts/harness_system.md` vẫn còn và vẫn được
      `include_str!`; `docs/prompts/` vẫn nguyên vẹn (so `find docs/prompts -type f | wc -l` trước/sau).
- [ ] Không file nào ngoài allow-list bị sửa. Đặc biệt `docs/` không đổi.
- [ ] Sau khi xong: port 1420/1421 free, không còn process `merlink-harness`/`vite`/`chromium`.
- [ ] Báo cáo có log thật cho từng mục. KHÔNG bịa kết quả.

───────────────────────────────────────────────────────
🛑 STOP CONDITIONS (DỪNG NGAY khi đạt 1 trong các điều này)
───────────────────────────────────────────────────────
- Tất cả ACCEPTANCE pass → DỪNG, báo "SPRINT DONE"
- Retry 1 task quá 2 lần mà vẫn fail → DỪNG, escalate
- Phát hiện task cần design decision không có trong spec → DỪNG, hỏi
- Cần thêm dependency ngoài `tauri` feature `test` (D1) → DỪNG, hỏi
- `tauri::test::get_ipc_response` không dùng được với `Channel` và điều đó chặn cả 3 command ở D4
  (không chỉ `send_chat`) → DỪNG, báo NEED_INPUT kèm lỗi thật
- Có ý muốn tách file / đổi cấu trúc module để test dễ hơn → DỪNG, hỏi (D7 cấm, refactor đã bị hoãn)
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
3. Cần thay đổi kiến trúc ngoài D1–D7 (folder structure, store shape, tách module) — refactor ĐÃ BỊ HOÃN
4. Phát hiện bug ở sprint trước ảnh hưởng sprint này
5. Cần user test thủ công (visual, UX, performance)

───────────────────────────────────────────────────────
🚫 OUT OF SCOPE (KHÔNG được làm)
───────────────────────────────────────────────────────
- File được phép sửa/tạo: `src-tauri/Cargo.toml` (chỉ thêm dev-dependency ở D1),
  `src-tauri/src/lib.rs`, `src-tauri/src/ipc_contract_tests.rs` (mới). XOÁ: `src-tauri/prompt/` (cả thư mục).
  KHÔNG sửa file nào khác. KHÔNG sửa gì trong `docs/`.
- KHÔNG tụ JSON về một chỗ, KHÔNG tách `tools.rs`/`runtime.rs`/`adapters.rs`, KHÔNG chọn design pattern
  cho tool/provider, KHÔNG đổi `tools.rs`. Người dùng đã hoãn toàn bộ nhóm việc này.
- KHÔNG `git init` (người dùng đã quyết định không dùng git lúc này).
- KHÔNG xoá `src-tauri/prompts/`, KHÔNG xoá `docs/prompts/`.
- KHÔNG đổi hành vi runtime của app, KHÔNG đổi UI/UX, KHÔNG sửa frontend.
- KHÔNG viết test ngoài 8 test ở task 5, KHÔNG optimize premature, KHÔNG update README/docs.

───────────────────────────────────────────────────────
📤 HANDOFF FORMAT (khi DỪNG, báo cáo theo format này)
───────────────────────────────────────────────────────

STATUS: [DONE | PARTIAL | BLOCKED | NEED_INPUT]
SPRINT: 11.6

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

GHI CHÚ KỸ THUẬT CHO NGƯỜI THỰC THI (bắt buộc đọc):
- `tauri::test` chỉ có sau feature `test`; source nằm ở
  `~/.cargo/registry/src/*/tauri-2.12.3/src/test/mod.rs`. Đọc chữ ký `get_ipc_response` và
  `InvokeRequest` trước khi viết test, đừng đoán.
- **Tauri v2 chờ key camelCase ở phía JS** (`tauri-macros` gọi `to_lower_camel_case`). Key thật đang dùng:
  `send_chat` → `{ providerId, modelId, messages, projectId, sessionId }`;
  `fetch_models` → `{ providerKind, baseUrl, apiKey }`;
  `super_json_read` → `{ projectId }`; `super_json_progress` → `{ sessionId }`;
  `list_messages` → `{ sessionId }`; `execute_tool` → `{ toolName, args, projectId, sessionId }`;
  `save_providers` → `{ providers }`. Đối chiếu lại `src/pages/Chat.tsx` và
  `src/components/SettingsDialog.tsx` để chắc, đừng chép từ đây mà không kiểm.
- `cargo test` chạy song song; `MERLINK_HOME` là biến toàn process → test nào chạm storage phải dùng
  `crate::test_support::TempHome` (đã có `ENV_LOCK` bên trong).
- `cmd | tail` che mất exit code → dùng `echo "EXIT=${PIPESTATUS[0]}"`.
- `pkill -f "pattern"` tự khớp shell đang chạy lệnh đó → dùng bracket trick `pkill -f "[p]attern"`.
- App dùng `HashRouter`; URL browser mode là `http://localhost:1420/#/chat`.
- `agent-browser` KHÔNG có sẵn; CDP driver ở `/tmp/cdp-check.mjs`; chromium cần `--no-sandbox`.
- Trước khi xoá `src-tauri/prompt/`, kiểm tra kỹ `src-tauri/build.rs` và `tauri.conf.json` xem có tham
  chiếu đường dẫn nào tới nó không (kể cả trong `bundle.resources`).
