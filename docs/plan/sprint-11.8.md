═══════════════════════════════════════════════════════
SPRINT 11.8 — FIX CRASH KHI REQUEST FAIL + THU GỌN LIVE STATUS
═══════════════════════════════════════════════════════

CONTEXT:
- Sprint 11.6 đã xong (71 test xanh). Đọc `docs/plan/sprint-11.md`, `sprint-11.5.md`, `sprint-11.6.md`
  để biết state, và `docs/plan/sprint-15.md` (đang LOCKED — refactor đã bị hoãn).
- **LỖI 1 — APP TỰ TẮT (ưu tiên cao nhất).** Người dùng báo nguyên văn:
  *"gửi request mà nó không thành công liền á (chưa có model), nó bị chuỗi gọi gì ấy, lag vãi xong dừng
  app luôn rồi, kiểm tra xem có thằng nào gọi đệ quy ko, chứ nó phản hồi liền mà lag nó văng ra luôn"*
  Tức là: gửi tin nhắn trong tình huống request **fail ngay lập tức** (ví dụ provider chưa có model /
  chưa chọn được model) → app rất lag → **thoát hẳn**.
- **Đã khảo sát sơ bộ, KHÔNG tìm ra vòng lặp hiển nhiên.** Đây là dữ kiện, không phải kết luận:
  - `src/pages/Chat.tsx` dài 800 dòng. `applyAgentEvent` (dòng ~87) là hàm thuần, `switch` bounded,
    **không đệ quy**.
  - `refreshSuperJson` (`useCallback`, deps `[]`) và `refreshProgress` (deps `[sessionId]`) → **không** tạo
    render loop. Effect ở dòng ~252 phụ thuộc `refreshProgress`; effect ở dòng ~257 phụ thuộc
    `[projectId, refreshSuperJson]`. Cả hai chỉ chạy lại khi khoá đổi.
  - `sendMessage` (dòng ~311) tạo channel **một lần mỗi lần gửi**, và `setLivePhases([])` nằm trong
    `finally` → không rò state qua các lần gửi.
  - `createAgentEventChannel` (`src/lib/tauri.ts` dòng ~36) sạch: Tauri thì `new Channel()`, browser thì
    `{ onmessage }`.
  Vì **không** tìm thấy nguyên nhân bằng đọc code, sprint này bắt buộc phải **tái hiện rồi mới sửa**.
- **LỖI 2 — Khung live status quá to.** Người dùng: *"cái khung hiển thị stream simulator lớn quá, chữ
  cũng rõ nữa, đang suy nghĩ được rồi, ko cần vòng mấy đâu, cái này là một màn hình với text hơi mờ chứ
  ko rõ đen, hiển thị nhỏ ở đầu góc trái thôi, độ lớn font và hiển thị thời gian giữ nguyên, nhưng đừng
  cho nó chiếm không gian vậy."*
- Endpoint test: `http://127.0.0.1:8045/v1`, api_key `local-dev-placeholder`, model `gpt-4o`.
  Config ở `/tmp/merlink-e2e/config.json`. Nhưng phần lớn việc của sprint này **không cần endpoint**.

DECISIONS ĐÃ CHỐT (không hỏi lại, không tự đổi):
- **D1 — ROOT CAUSE TRƯỚC, CẤM ĐOÁN.** Không được sửa theo phỏng đoán. Phải có:
  (a) một **cách tái hiện** ghi lại được (thao tác chính xác + hiện tượng quan sát),
  (b) **bằng chứng** cho nguyên nhân nhân quả (log, stack, `panicked at`, số đo CPU, hoặc counter được
  gắn tạm),
  (c) sau đó mới sửa.
  Nếu **không tái hiện được** sau 2 lần thử → DỪNG, báo NEED_INPUT kèm mọi thứ đã thử. Không được "sửa
  cho chắc" rồi tuyên bố đã fix.
- **D2 — Danh sách nghi phạm phải kiểm tra, theo thứ tự. Kiểm tra = chứng minh có/không, không phải suy đoán:**
  1. **Panic phía Rust** làm chết process. Kiểm log terminal của `pnpm tauri dev` xem có `panicked at`.
     Lưu ý `src-tauri/Cargo.toml` có `[profile.release] panic = "abort"` — kiểm xem dev profile có bị ảnh
     hưởng không, và panic trong `#[tauri::command]` async có bị nuốt hay không.
  2. **Vòng lặp không bounded phía Rust** (mọi `while`/`loop`/đệ quy) — nếu có, CPU bị đốt → "lag" →
     bị OS kill. Soi `unique_project_id`, `slugify`, agent loop, và mọi vòng lặp trong `db.rs`,
     `super_json.rs`, `runtime.rs`, `tools.rs`, `adapters.rs`.
  3. **Rò `Channel`**: mỗi lần gửi tạo một `Channel` mới. Kiểm xem Tauri/`transformCallback` có giữ
     callback trong một map không bao giờ được xoá → gửi nhiều lần là phình. Đo bằng cách gửi N lần
     request fail và theo dõi bộ nhớ.
  4. **Đệ quy ở đường lỗi**: có chỗ nào tự gọi lại chính nó khi fail không (retry không có điều kiện dừng)?
  5. **Render loop chỉ xuất hiện ở trạng thái lỗi**: đọc lại `Chat.tsx` nhánh `catch`, và mọi effect phụ
     thuộc state bị `catch` thay đổi (`error`).
- **D3 — Phải có test chống tái phát.** Test phải **fail trước khi sửa** và **pass sau khi sửa**:
  - Nếu nguyên nhân ở Rust → unit test trong Rust.
  - Nếu nguyên nhân ở frontend/React → không có test runner JS trong repo, nên dùng **script CDP**
    (xem ghi chú kỹ thuật) gửi một request fail rồi assert trang **vẫn phản hồi** (ví dụ: hẹn giờ
    `setTimeout` 500ms vẫn bắn, hoặc đếm số lần một hàm bị gọi qua `performance`/counter, hoặc
    `requestAnimationFrame` vẫn chạy). Ghi rõ trong báo cáo vì sao không thể test bằng unit test.
  - Bắt buộc chứng minh: chạy trên code **trước** khi sửa thấy nó fail (hoặc treo), dán output.
- **D4 — Thu gọn live status (đặc tả cụ thể):**
  - Bỏ panel full-width đang lớn dần. Thay bằng **một dòng trạng thái duy nhất**, neo ở **đầu góc trái**
    vùng tin nhắn.
  - Chỉ hiển thị **phase đang chạy**. Các phase đã xong **không xếp chồng** thành danh sách.
  - **Bỏ số vòng**: nhãn là `"Đang suy nghĩ"`, KHÔNG còn `"Đang suy nghĩ (vòng N)"`.
  - **Giữ nguyên cỡ chữ** và **giữ hiển thị thời gian** (ví dụ `0.3s`) như hiện tại.
  - **Chữ mờ, không đậm, không đen**: dùng màu muted + opacity giảm. Không dùng màu ink chính, không bold.
  - **Không chiếm không gian**: chiều cao một dòng (xem ngưỡng ở ACCEPTANCE), không đẩy danh sách tin nhắn.
- **D5 — Không thêm dependency nào** (Rust lẫn npm).
- **D6 — Không đụng phần khác.** Sprint này KHÔNG làm: dialog Super JSON, donut context, session,
  dialog gợi ý option, sinh dàn ý. Ba việc đó dồn sang **Sprint 11.9** — xem mục "Sprint 11.9" ở cuối.

GOAL:
Tái hiện được, tìm ra nguyên nhân gốc, và sửa lỗi app tự tắt khi request fail (kèm test chống tái phát);
đồng thời thu live status thành một dòng mờ ở góc trái, bỏ số vòng, giữ cỡ chữ và thời gian.

TASKS:
1. **Tái hiện lỗi 1 trước khi sửa bất cứ thứ gì.** Dựng kịch bản xác định: ví dụ xoá/đặt sai
   model id, hoặc chọn provider có `models: []`, hoặc chặn endpoint bằng base URL sai, rồi gửi tin nhắn.
   Ghi lại: thao tác, hiện tượng, thời gian tới khi treo, và log terminal. Dán vào báo cáo.
2. Chạy qua danh sách nghi phạm D2 theo thứ tự, ghi rõ **cái nào bị loại trừ bằng bằng chứng gì**.
3. Sửa nguyên nhân gốc đã tìm được.
4. Thêm test chống tái phát theo D3; chứng minh nó fail trước khi sửa.
5. `src/pages/Chat.tsx` + `src/index.css`: thu gọn live status theo D4 (bỏ `"vòng N"` khỏi
   `applyAgentEvent`, chỉ render phase đang chạy, style mờ + một dòng).
6. Verify theo ACCEPTANCE, thu log/ảnh thật, dọn process/port.

ACCEPTANCE:
- [ ] **Có bằng chứng tái hiện TRƯỚC khi sửa**: dán log/số đo cho thấy app treo hoặc thoát. Không có
      bằng chứng này thì sprint coi như chưa xong, dù code đã đổi.
- [ ] **Nguyên nhân gốc được phát biểu kèm bằng chứng** (file:line + log trích dẫn hoặc số đo).
      Nếu nguyên nhân là "nhiều thứ cộng lại" thì phải nói rõ từng cái và cái nào quyết định.
- [ ] Cùng kịch bản tái hiện đó, chạy lại **sau khi sửa** → app **không treo, không thoát**, UI hiện
      thông báo lỗi đọc được. Dán bằng chứng (ảnh + log + thời gian phản hồi).
- [ ] Test chống tái phát (D3) **fail trên code cũ** và **pass trên code mới**. Dán cả hai output.
      Nếu không thể test tự động được thì giải thích rõ vì sao, và bù bằng script CDP có assert.
- [ ] `cd src-tauri && cargo test` → PASS toàn bộ (hiện 71 passed / 3 ignored), 0 warning, không rơi test nào.
- [ ] `cargo test` vẫn KHÔNG ghi gì vào `~/.merlink` thật.
- [ ] `pnpm build` (tsc + vite) → PASS, 0 lỗi type.
- [ ] **Live status (D4) — đo bằng DOM qua CDP**, dán số đo thật:
      - [ ] Không còn text khớp `/vòng\s*\d+/` ở khu vực status.
      - [ ] Element status có chiều cao **≤ 28px** và là một dòng (`scrollHeight - clientHeight <= 2`).
      - [ ] Màu chữ **không** phải màu ink chính, và có độ mờ (opacity < 1 hoặc dùng token muted).
      - [ ] Trong lúc chạy vẫn thấy chuỗi thời gian khớp `/\d+\.\d+s/`.
      - [ ] Ảnh chụp trước/sau để so bằng mắt.
- [ ] App khởi động bằng `MERLINK_HOME=/tmp/merlink-e2e pnpm tauri dev` → sống > 30s, không panic.
- [ ] Sau khi xong: port 1420/1421 free, không còn process `merlink-harness`/`vite`/`chromium`.
- [ ] KHÔNG có file nào ngoài allow-list bị sửa; `docs/` không đổi.
- [ ] Báo cáo trung thực: nếu không tái hiện được, hoặc chỉ sửa được một phần, nói thẳng. KHÔNG bịa.

───────────────────────────────────────────────────────
🛑 STOP CONDITIONS (DỪNG NGAY khi đạt 1 trong các điều này)
───────────────────────────────────────────────────────
- Tất cả ACCEPTANCE pass → DỪNG, báo "SPRINT DONE"
- Retry 1 task quá 2 lần mà vẫn fail → DỪNG, escalate
- **Không tái hiện được lỗi 1 sau 2 lần thử** → DỪNG, báo NEED_INPUT kèm mọi thứ đã thử và hỏi người
  dùng cách tái hiện chính xác. KHÔNG được sửa mò.
- Nguyên nhân gốc nằm ngoài allow-list file (ví dụ phải sửa `adapters.rs`, `runtime.rs`, `db.rs`) → DỪNG,
  báo NEED_INPUT nêu rõ file cần sửa và lý do. (Xem allow-list ở OUT OF SCOPE — có mở sẵn cho nhóm Rust
  nếu nguyên nhân ở đó, nhưng phải khai báo.)
- Cần thêm dependency → DỪNG, hỏi
- Phát hiện phải tái cấu trúc module mới sửa được → DỪNG, hỏi (refactor đang LOCKED ở sprint 15)
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
3. Cần thay đổi kiến trúc ngoài allow-list (folder structure, store shape, tách module)
4. Không tái hiện được lỗi 1
5. Cần user test thủ công (visual, UX, performance)

───────────────────────────────────────────────────────
🚫 OUT OF SCOPE (KHÔNG được làm)
───────────────────────────────────────────────────────
- File được phép sửa: `src/pages/Chat.tsx`, `src/index.css`, và **chỉ khi nguyên nhân gốc nằm ở đó**:
  `src-tauri/src/{runtime,tools,db,super_json,adapters,models,lib}.rs`, `src/lib/tauri.ts`,
  `src/types.ts`. Mọi file Rust sửa được phải **khai báo rõ trong báo cáo kèm lý do**.
  KHÔNG sửa gì trong `docs/`.
- KHÔNG làm dialog Super JSON, KHÔNG làm donut context, KHÔNG đổi tên nút thành "Chi tiết" — đó là
  **Sprint 11.9**. Xem mục cuối file.
- KHÔNG làm session list/resume/rename/delete.
- KHÔNG làm dialog gợi ý option (skip/submit), KHÔNG viết logic sinh dàn ý.
- KHÔNG đổi shape `super.json` / 25 path / công thức tiến độ 70%/30% / giới hạn 5 vòng agent loop.
- KHÔNG tái cấu trúc module, KHÔNG tụ JSON về một chỗ, KHÔNG chọn design pattern (`sprint-15.md` LOCKED).
- KHÔNG `git init`.
- KHÔNG thêm feature ngoài GOAL, KHÔNG đổi UI ngoài D4, KHÔNG optimize premature,
  KHÔNG viết test ngoài test ở D3, KHÔNG update README/docs.

───────────────────────────────────────────────────────
📤 HANDOFF FORMAT (khi DỪNG, báo cáo theo format này)
───────────────────────────────────────────────────────

STATUS: [DONE | PARTIAL | BLOCKED | NEED_INPUT]
SPRINT: 11.8

✅ COMPLETED:
- [task 1]: [file changed]

❌ FAILED (nếu có):
- [task]: [lỗi gì] | [đã thử gì] | [cần gì từ user]

🤔 NEED_INPUT (nếu có):
- Câu hỏi 1: [mô tả] | Option A: [...] | Option B: [...]

📋 NEXT ACTION:
- Nếu DONE → "Ready for sprint 11.9, paste next prompt"
- Nếu BLOCKED → "Waiting for user response on X"
- Nếu PARTIAL → "Continue task Y after user confirms"

═══════════════════════════════════════════════════════

SPRINT 11.9 — ĐÃ ĐƯỢC HOÃN SANG FILE RIÊNG (không làm ở sprint này):
Người dùng yêu cầu 2 việc nữa, cố tình tách ra để sprint này tập trung vào crash:
1. View super json thành **dialog** (không phải khung xổ xuống), đổi chữ `"Super JSON"` → `"Chi tiết"`,
   và **xếp cạnh thanh tiến độ**. Gợi ý: đã có sẵn `src/components/Dialog.tsx` từ Sprint 8a.
2. **Donut context** dưới khung chat: hover hiện `%` context model nhớ được, tổng + **3 phân loại**
   (System prompt / Tool definitions / Messages). Cần command Rust mới tính token (nguồn sự thật ở backend),
   ước lượng theo ký tự và ưu tiên số `meta.tokens` thật khi có.
Lý do tách: nếu crash nằm ở chính đoạn render live-status thì thiết kế donut (một chỗ cập nhật định kỳ
nữa) phải chờ biết nguyên nhân, nếu không sẽ lặp lại đúng lỗi đó.

GHI CHÚ KỸ THUẬT CHO NGƯỜI THỰC THI (bắt buộc đọc):
- `tauri-macros` chuyển tên tham số Rust bằng `to_lower_camel_case`: mọi `invoke` từ JS phải dùng key
  camelCase (`sessionId`, `projectId`, `providerKind`…). Đã có test IPC bắt lỗi này ở
  `src-tauri/src/ipc_contract_tests.rs` — nếu bạn sửa/thêm command thì phải cập nhật bảng contract ở đó,
  không thì `test_contract_table_covers_every_registered_command` fail.
- `cargo test` chạy song song; `MERLINK_HOME` là biến toàn process → test chạm storage phải dùng
  `crate::test_support::TempHome`.
- `cmd | tail` che mất exit code → dùng `echo "EXIT=${PIPESTATUS[0]}"`.
- `pkill -f "pattern"` tự khớp shell đang chạy lệnh đó → dùng bracket trick `pkill -f "[p]attern"`.
- App dùng `HashRouter`; URL browser mode là `http://localhost:1420/#/chat`.
- `agent-browser` KHÔNG có sẵn. CDP driver có sẵn: `/tmp/cdp-check.mjs`, `/tmp/cdp-phases.mjs`,
  `/tmp/cdp-sprint115.mjs`. Chromium cần `--no-sandbox`. Dùng chúng để đo DOM (chiều cao element, màu
  computed style, sự hiện diện của chuỗi thời gian) và chụp ảnh.
- Để đo "trang còn phản hồi hay không" trong browser mode: cài một counter bằng `Runtime.evaluate`
  (ví dụ `window.__probe = 0; setInterval(() => window.__probe++, 50)`) rồi sau khi gửi request fail, đọc
  lại counter — nếu nó ngừng tăng thì event loop đã bị chặn.
- Trong browser mode, `createAgentEventChannel` trả `{ onmessage }` và mock phát event; đường Tauri thật
  dùng `new Channel()`. Lỗi treo có thể chỉ xuất hiện ở một trong hai đường — phải ghi rõ đã tái hiện ở
  đường nào.
