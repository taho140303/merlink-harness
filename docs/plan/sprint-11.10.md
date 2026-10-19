═══════════════════════════════════════════════════════
SPRINT 11.10 — SUPER JSON THÀNH DIALOG + DONUT CONTEXT
═══════════════════════════════════════════════════════

CONTEXT:
- Sprint 11.91 xong (markdown + copy + `Tổng: X.Xs` + history mũi tên). Sprint 11.92 xong (blob đứng yên,
  CPU 24–29% → 0%, xoá Local Mode). Đọc `docs/plan/sprint-11.91.md` và `sprint-11.92.md`.
- Hiện tại panel super json là **khung xổ xuống** ngay dưới topbar: state `superJsonOpen`, nút nhãn
  `"Super JSON"`, mở ra chiếm chỗ và đẩy nội dung xuống.
- Chưa có bất kỳ thông tin nào về **context window**: người dùng không biết model đang nhớ bao nhiêu,
  còn bao nhiêu chỗ. Ảnh tham chiếu người dùng gửi có dạng: `36% of context used · ~364K / 1M`, kèm 3 dòng
  `System prompt ~1.4K`, `Tool definitions ~8K`, `Messages ~299K`.
- Yêu cầu nguyên văn: *"cái view super json là dialog chứ ko phải cửa sổ xổ xuống nhé, với cái chữ super
  json đổi thành chi tiết và xếp nó kế thanh tiến độ nhé, với lại chưa xem được context, kiểu vòng tròn
  donut dưới khung chat hover vào có thể hiển thị được % context mà model đó nhớ tổng và 3 phân loại."*
- `src/components/Dialog.tsx` đã tồn tại từ Sprint 8a (backdrop click, Escape, focus trap, focus restore,
  variant `wide`) → dùng lại, không viết mới.
- Repo có IPC contract test với drift guard: `src-tauri/src/ipc_contract_tests.rs`. Thêm command mới mà
  không thêm dòng vào bảng contract → `test_contract_table_covers_every_registered_command` fail.

DECISIONS ĐÃ CHỐT (không hỏi lại, không tự đổi):
- **D1 — Super json thành DIALOG, dùng lại `Dialog.tsx`.** Bỏ hẳn khung xổ xuống (`superJsonOpen` +
  panel chèn dưới topbar). Nút đổi nhãn thành **`"Chi tiết"`**, đặt **ngay cạnh thanh tiến độ** trong
  `.page-topbar-actions` (thanh tiến độ trước, nút sau). Dialog dùng variant `wide`, nội dung: JSON
  pretty-print + nút Refresh. Escape và click backdrop đóng (đã có sẵn trong `Dialog.tsx`).
- **D2 — Context tính ở RUST, UI chỉ vẽ.** Thêm command
  `context_usage(session_id, provider_id, model_id) -> ContextUsage`:
  ```rust
  pub struct ContextUsage {
      pub has_model: bool,
      pub total_tokens: u64,      // context window của model
      pub used_tokens: u64,       // system + tools + messages
      pub percent: u8,            // 0..=100, làm tròn
      pub system_tokens: u64,
      pub tool_tokens: u64,
      pub message_tokens: u64,
      pub measured_input_tokens: Option<u64>, // số thật từ meta.tokens nếu có
  }
  ```
- **D3 — Cách ước lượng phải nói rõ, và phải nhất quán.** Token ước lượng = `chars / 4` (làm tròn lên),
  áp dụng cho cả 3 phần:
  - `system_tokens`: độ dài `assemble_prompt(...)` với provider/model/session/project hiện tại.
  - `tool_tokens`: độ dài `serde_json::to_string(&tools::tool_specs())`.
  - `message_tokens`: tổng `content` của các message trong session (đọc qua `db::list_messages`), **cộng**
    phần JSON của `tool_calls` nếu có.
  `used_tokens = system + tool + message`. `percent = round(100*used/total)`, kẹp 0..=100.
  `measured_input_tokens` = `meta.tokens.input_tokens` của assistant message cuối cùng nếu có (để UI hiện
  số thật bên cạnh số ước lượng). **Không trộn** số thật vào `used_tokens`.
- **D4 — Parse context window từ `ModelConfig.context_window`** ("128K", "200K", "1M", "128000").
  Viết hàm thuần `parse_context_window(&str) -> u64`, fallback **128_000** khi không parse được hoặc = 0.
  Không có provider/model (hoặc model không tồn tại) → `has_model = false`, các số = 0, `total_tokens = 0`.
- **D5 — Donut ở DƯỚI khung chat**, cạnh hàng input (không phải trong header). Kích thước ~32–40px,
  ring hiển thị phần đã dùng, số `%` ở giữa. Hover **và** focus (bàn phím) mở popover gồm:
  `~used / total`, và 3 dòng `System prompt`, `Tool definitions`, `Messages`. Số hiển thị có tiền tố `~`
  vì là ước lượng; nếu có `measured_input_tokens` thì thêm một dòng "đo thật: n".
  `has_model = false` → donut rỗng, popover ghi "Chưa chọn model".
- **D6 — Màu donut dùng biến theme sẵn có** (`--accent`, `--accent-soft`, `--muted`, `--line`), không
  hardcode màu mới. Phải đọc được ở cả theme sáng và tối.
- **D7 — Không thêm dependency** (Rust lẫn npm). Không dùng thư viện chart.
- **D8 — Cập nhật IPC contract test**: thêm `context_usage` vào `merlink_handlers!` (đã có macro) **và**
  thêm dòng tương ứng vào bảng contract trong `ipc_contract_tests.rs`, nếu không drift guard sẽ fail.

GOAL:
Super json mở bằng **dialog** (nhãn "Chi tiết", nằm cạnh thanh tiến độ, không đẩy layout); và có **donut
context** dưới khung chat, hover/focus hiện `%` đã dùng, tổng `used/total`, cùng 3 phân loại
system prompt / tool definitions / messages.

TASKS:
1. `src-tauri/src/models.rs`: thêm `ContextUsage` (serde snake_case khớp D2).
2. `src-tauri/src/adapters.rs`: thêm `parse_context_window(&str) -> u64` (D4) — hàm thuần, có test.
3. `src-tauri/src/runtime.rs`: `context_usage_for_session(session_id, provider_id, model_id)` theo D2/D3.
   Dùng `db::list_messages`, `store::load_providers`, `tools::tool_specs`, `assemble_prompt`.
4. `src-tauri/src/lib.rs`: command `context_usage(session_id, provider_id, model_id)`; thêm vào
   `merlink_handlers!`.
5. `src-tauri/src/ipc_contract_tests.rs`: thêm dòng contract cho `context_usage` với key camelCase
   (`sessionId`, `providerId`, `modelId`).
6. `src/pages/Chat.tsx`: bỏ panel xổ xuống; nút "Chi tiết" mở `Dialog` (variant wide) chứa JSON + Refresh;
   đặt nút ngay sau thanh tiến độ. Thêm donut dưới khung chat + popover theo D5; gọi `context_usage` khi
   mount, sau mỗi lượt, và khi đổi provider/model.
7. `src/components/ContextDonut.tsx` (mới): SVG donut + popover. Không dep.
8. `src/types.ts` + `src/lib/tauri.ts`: mirror `ContextUsage`, mock `context_usage` cho browser mode
   (số giả định nhưng **phải** chia đúng 3 phần và percent khớp).
9. `src/index.css`: block `/* Sprint 11.10 · detail dialog + context donut */`.
10. Viết test (BẮT BUỘC) đúng các tên:
    `test_parse_context_window_k_and_m`,
    `test_parse_context_window_falls_back_to_128k`,
    `test_context_usage_splits_into_three_parts`,
    `test_context_usage_percent_matches_window`,
    `test_context_usage_without_model_is_zero`,
    `test_context_usage_grows_after_a_turn`,
    `test_ipc_context_usage_end_to_end`.
11. Verify theo ACCEPTANCE, dọn process/port.

ACCEPTANCE:
- [ ] `cd src-tauri && cargo test` → PASS toàn bộ, 0 warning, gồm 7 test mới + **drift guard vẫn xanh**
      (tức `context_usage` đã có dòng contract).
- [ ] `pnpm build` (tsc + vite) → PASS, 0 lỗi type.
- [ ] `package.json` và `Cargo.toml` **không đổi** (D7).
- [ ] **Dialog, đo bằng DOM/CDP:**
      - [ ] Nhãn nút là `Chi tiết`; **không còn** text `Super JSON` ở header.
      - [ ] Bấm nút → có dialog (`[role="dialog"]` hoặc phần tử `.dialog-panel`), có JSON pretty-print.
      - [ ] **Escape đóng**, click backdrop đóng.
      - [ ] Nút nằm **sau** thanh tiến độ trong `.page-topbar-actions` (dán thứ tự DOM thật).
      - [ ] Khi dialog mở, **không** đẩy layout: chiều cao vùng transcript không đổi (dán số đo).
- [ ] **Donut, đo bằng DOM/CDP:**
      - [ ] Donut render dưới khung chat (dán selector + toạ độ/rect thật).
      - [ ] Hover (dispatch chuột) → popover hiện `%`, `~used / total`, và **đủ 3 dòng** system prompt /
            tool definitions / messages. Dán text thật của popover.
      - [ ] `system + tool + message == used` (dán 4 số).
      - [ ] `percent == round(100*used/total)` (dán phép tính).
      - [ ] `has_model = false` (khi chưa có model) → popover ghi "Chưa chọn model", donut rỗng, **không crash**.
- [ ] **E2E thật**: chạy `MERLINK_HOME=/tmp/merlink-e2e cargo test -- --ignored --nocapture real_endpoint`
      (hoặc một test online riêng) gọi `context_usage` sau 1 lượt thật và dán số: tổng, 3 phần, percent.
      Nếu không chạy được endpoint thì nói rõ và chỉ dùng số từ mock — **không bịa**.
- [ ] Browser mode: **0 console error / 0 exception**; ảnh chụp có dialog mở và ảnh chụp có donut + popover.
- [ ] `MERLINK_HOME=/tmp/merlink-e2e pnpm tauri dev` → compile, cửa sổ sống > 30s, không panic.
- [ ] Sau khi xong: port 1420/1421 free, không còn process `merlink-harness`/`vite`/`chromium`.
- [ ] `docs/` không đổi; chỉ file trong allow-list bị sửa.
- [ ] Báo cáo trung thực, dán số đo thật. KHÔNG bịa.

───────────────────────────────────────────────────────
🛑 STOP CONDITIONS
───────────────────────────────────────────────────────
- Tất cả ACCEPTANCE pass → DỪNG, báo "SPRINT DONE"
- Retry 1 task quá 2 lần vẫn fail → DỪNG, escalate
- Cần thêm dependency (chart lib, markdown lib, …) → DỪNG, hỏi (D7)
- Drift guard fail vì bạn không muốn thêm contract row → DỪNG, báo (không được nới lỏng test)
- Task vượt 30 phút ước tính → DỪNG, chia nhỏ lại

───────────────────────────────────────────────────────
🔁 RETRY BUDGET
───────────────────────────────────────────────────────
- Mỗi task tối đa 2 lần thử, lần sau phải đổi approach
- Sau lần 2 fail: DỪNG task đó, chuyển task khác
- Cuối sprint: liệt kê task chưa xong + lý do

───────────────────────────────────────────────────────
🚨 ESCALATION TRIGGERS
───────────────────────────────────────────────────────
1. Lỗi không rõ nguyên nhân sau 2 lần debug
2. Cần chọn giữa 2 approach mà spec không nói
3. Cần đổi kiến trúc ngoài allow-list
4. Bug sprint trước ảnh hưởng sprint này
5. Cần user quyết định thẩm mỹ

───────────────────────────────────────────────────────
🚫 OUT OF SCOPE
───────────────────────────────────────────────────────
- File được sửa/tạo: `src-tauri/src/{models,adapters,runtime,lib}.rs`,
  `src-tauri/src/ipc_contract_tests.rs`, `src/pages/Chat.tsx`, `src/components/ContextDonut.tsx` (mới),
  `src/types.ts`, `src/lib/tauri.ts`, `src/index.css`. `src/components/Dialog.tsx` chỉ được sửa nếu thật
  sự cần (phải báo lý do). KHÔNG sửa `docs/`.
- KHÔNG làm phần session (list/resume/rename/delete).
- KHÔNG làm dialog gợi ý option (skip/submit), KHÔNG viết logic sinh dàn ý.
- KHÔNG đổi shape `super.json` / 25 path / công thức tiến độ 70%/30% / giới hạn 5 vòng.
- KHÔNG đổi markdown renderer, nút sao chép, nhãn `Tổng:` hay history mũi tên (vừa xong ở 11.91).
- KHÔNG đụng blob/atmosphere (vừa chốt ở 11.92).
- KHÔNG tái cấu trúc module / tụ JSON / design pattern (`sprint-15.md` LOCKED).
- KHÔNG thêm feature ngoài GOAL, KHÔNG update README/docs.

───────────────────────────────────────────────────────
📤 HANDOFF FORMAT
───────────────────────────────────────────────────────

STATUS: [DONE | PARTIAL | BLOCKED | NEED_INPUT]
SPRINT: 11.10

✅ COMPLETED / ❌ FAILED / 🤔 NEED_INPUT / 📋 NEXT ACTION (như các sprint trước)

═══════════════════════════════════════════════════════

GHI CHÚ KỸ THUẬT (bắt buộc đọc):
- **Tauri v2 chờ key camelCase ở phía JS.** `context_usage(session_id, provider_id, model_id)` phải gọi
  bằng `{ sessionId, providerId, modelId }`. Sai kiểu snake_case → "missing required key".
  Đã có test IPC bắt lỗi này; đừng làm nó đỏ.
- `cmd | tail` che mất exit code → `echo "EXIT=${PIPESTATUS[0]}"`.
- `pkill -f "pattern"` tự khớp shell đang chạy lệnh đó → `pkill -f "[p]attern"`.
- App dùng `HashRouter`; browser mode URL `http://localhost:1420/#/chat`.
- `agent-browser` KHÔNG có. CDP driver có sẵn `/tmp/cdp-check.mjs` (+ `cdp-phases.mjs`,
  `cdp-sprint115.mjs`); chromium cần `--no-sandbox`. Dùng chúng để đo DOM, dispatch hover, chụp ảnh.
- **ĐỪNG dùng `gnome-screenshot`** để chụp cửa sổ app: trên máy này `-w` có lúc chụp toàn màn hình, tức là
  lọt desktop riêng tư của người dùng. Luôn chụp bằng headless browser.
- HMR của Vite có thể để lại stylesheet stale nếu ghi file kiểu truncate. Nếu sửa CSS, ghi **nguyên tử**
  (ghi file tạm rồi `os.replace`), và verify layout bằng ảnh headless trước khi tin kết quả.
- `cargo test` chạy song song, `MERLINK_HOME` là biến toàn process → test chạm storage phải dùng
  `crate::test_support::TempHome`.
- Đo phần trăm: dùng số nguyên `u8`, làm tròn, kẹp 0..=100; đừng để chia cho 0 khi `total_tokens == 0`.
