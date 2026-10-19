═══════════════════════════════════════════════════════
SPRINT 11.11 — HÀNG NHẬP LIỆU 3 PHẦN + SUPER JSON THÀNH CÂU
═══════════════════════════════════════════════════════

CONTEXT:
- Sprint 11.10 vừa xong: `Chi tiết` mở dialog (thay khung xổ xuống), donut context 38×38 có popover, command
  `context_usage`, 78 test xanh. Đọc `docs/plan/sprint-11.10.md`.
- **Lỗi layout người dùng báo:** donut đang nằm **dưới** khung chat (đo được rect `x=1318, y=769`), tách
  khỏi hàng nhập liệu. Người dùng muốn: **khung nhập liệu + ô context + nút gửi nằm NGANG HÀNG**.
- **Yêu cầu mới về dialog `Chi tiết`:** hiện đang đổ JSON thô. Người dùng muốn nó **parse thành câu văn tự
  nhiên**, kiểu một bản tóm tắt, với chỗ thiếu dữ liệu hiển thị `[chưa có thông tin]`.
  Người dùng đưa một mẫu (hồ sơ nhân vật) để chỉ **văn phong**, và nói rõ: *"này đương nhiên là ví dụ, mẫu
  theo superjson"* → tức nội dung phải theo **25 field của super.json**, không phải theo hồ sơ nhân vật.
- Repo có IPC contract test với drift guard (`src-tauri/src/ipc_contract_tests.rs`): thêm command mà không
  thêm dòng contract → `test_contract_table_covers_every_registered_command` fail.

DECISIONS ĐÃ CHỐT (không hỏi lại, không tự đổi):
- **D1 — Hàng nhập liệu gồm đúng 3 phần, cùng một hàng, theo thứ tự:**
  `[khung nhập liệu (flex:1)] [ô context 38×38] [nút Gửi]`.
  - Donut **không** còn `position:absolute` / `fixed`; nó là phần tử trong cùng flex row với input và nút.
  - Căn giữa theo trục dọc (`align-items:center`), khoảng cách đều (`gap`).
  - Popover của donut mở **lên trên** (vì donut ở đáy màn hình) và **không bị cắt** bởi container.
  - Nút Gửi giữ nguyên kích thước/nhãn hiện tại.
- **D2 — Tóm tắt super json sinh ở RUST, theo template cố định, KHÔNG gọi model.** Thêm hàm thuần
  `super_json::summary(document: &Value) -> String` + command
  `super_json_summary(project_id: String) -> String`.
  Lý do: tất định, test được, tức thời, không tốn token. Không dùng LLM để "viết lại".
- **D3 — Nội dung tóm tắt: đúng 6 đoạn, theo thứ tự sau.** Mọi giá trị thiếu (`null`, chuỗi rỗng, mảng
  rỗng) → in ra **`[chưa có thông tin]`**. Không được bỏ đoạn nào; đoạn nào thiếu hết thì vẫn in câu với
  placeholder.
  ```
  1. "Dự án truyện có tên là {name}, thuộc thể loại {genre.primary}{, kèm {genre.secondary nối bằng ", "}}.
      Tác giả là {author}. Truyện viết bằng {language}, hướng tới độc giả {target-audience},
      độ tuổi {age-rating}."
  2. "Câu chuyện bắt đầu khi {logline.trigger-event}. Nhân vật chính là {logline.main-character},
      người {logline.action}. Mối nguy là {logline.risk}, và cái giá có thể phải chịu là
      {logline.major-loss}."
  3. "Bộ truyện đặt ra câu hỏi: {theme.question}. Thông điệp hướng tới là {theme.message}.
      Cách trả lời được chọn là {theme.resolution-type}, với nội dung {theme.resolution}."
  4. "Dự kiến dài {plan.arc.count} arc, {plan.chapter.count} chương, khoảng {plan.word.count} từ.
      Sai số chấp nhận: {plan.arc.tolerance} arc, {plan.chapter.tolerance} chương,
      {plan.word.tolerance} từ."
  5. "Thế giới {có/không có} hệ thống sức mạnh. Phạm vi không gian sâu nhất cần dựng là
      {world.setting-scale}."
  6. "Tóm tắt nội dung: {overview.description}."
  ```
  Quy ước in số: in nguyên (`5`, `120`, `90000` → `90000`, không thêm dấu phân cách nghìn).
- **D4 — Map enum sang tiếng Việt** (không in mã thô):
  - `language`: `VI`→`tiếng Việt`, `EN`→`tiếng Anh`, `JA`→`tiếng Nhật`, `ZH_HANT`→`chữ Hán phồn thể`,
    `ZH_CN`/`ZH_HANS`→`chữ Hán giản thể`.
  - `theme.resolution-type`: `RESOLVED`→`kết thúc rõ ràng`, `OPEN`→`để ngỏ`, `AMBIGUOUS`→`mơ hồ`.
  - `world.setting-scale`: `REALM`→`cõi`, `WORLD`→`thế giới`, `REGION`→`vùng`, `COUNTRY`→`quốc gia`,
    `CITY`→`thành phố`, `TOWN`→`thị trấn`, `FACILITY`→`công trình`, `STREET`→`đường phố`, `ROOM`→`căn phòng`.
  - `world.has-power-system`: `true`→`có`, `false`→`không có`, `null`→placeholder.
- **D5 — Dialog `Chi tiết` hiển thị TÓM TẮT là nội dung chính**, và có một nút nhỏ `JSON` để bật/tắt xem
  raw JSON (giữ khả năng debug, không mất dữ liệu). Mặc định là tóm tắt.
  Nút Refresh hiện có vẫn giữ, refresh cả hai.
- **D6 — Cập nhật IPC contract test**: thêm `super_json_summary` vào `merlink_handlers!` và thêm dòng
  contract (`{ projectId }`) trong `ipc_contract_tests.rs`. Không được nới lỏng drift guard.
- **D7 — Không thêm dependency** (Rust lẫn npm). Không gọi model để sinh tóm tắt.

GOAL:
Hàng nhập liệu có đúng 3 phần ngang hàng `[input][context][Gửi]`; và dialog `Chi tiết` hiển thị một bản tóm
tắt tiếng Việt theo 6 đoạn, chỗ thiếu dữ liệu là `[chưa có thông tin]`, kèm nút xem raw JSON.

TASKS:
1. `src-tauri/src/super_json.rs`: `pub fn summary(document: &Value) -> String` theo D3/D4.
   Thêm helper `fn text_at(doc, path) -> Option<String>` và `fn placeholder()` để tránh lặp.
2. `src-tauri/src/runtime.rs`: `super_json_summary_for_project(project_id)` gọi `load_or_init` + `summary`.
3. `src-tauri/src/lib.rs`: command `super_json_summary(project_id)`; thêm vào `merlink_handlers!`.
4. `src-tauri/src/ipc_contract_tests.rs`: thêm dòng contract `super_json_summary` với `{ projectId }`.
5. `src/pages/Chat.tsx`: đưa donut vào cùng flex row với input và nút Gửi (D1); bỏ positioning tuyệt đối
   của donut; popover mở lên trên. Dialog `Chi tiết`: hiển thị tóm tắt (gọi `super_json_summary`), thêm nút
   toggle `JSON` (D5).
6. `src/components/ContextDonut.tsx`: bỏ style absolute/fixed nếu có; nhận prop để render inline.
7. `src/types.ts` + `src/lib/tauri.ts`: mock `super_json_summary` cho browser mode (trả một câu tóm tắt
   mẫu có placeholder để UI test được).
8. `src/index.css`: block `/* Sprint 11.11 · input row + summary */`; chỉnh style hàng nhập liệu và popover.
9. Viết test (BẮT BUỘC) đúng các tên:
   `test_summary_marks_missing_fields_as_placeholder`,
   `test_summary_includes_all_filled_values`,
   `test_summary_maps_enums_to_vietnamese`,
   `test_summary_has_six_paragraphs`,
   `test_summary_handles_partial_data`,
   `test_summary_is_deterministic`,
   `test_ipc_super_json_summary_end_to_end`.
10. Verify theo ACCEPTANCE, dọn process/port.

ACCEPTANCE:
- [ ] `cd src-tauri && cargo test` → PASS, 0 warning, gồm 7 test mới + drift guard xanh.
- [ ] `pnpm build` → PASS, 0 lỗi type.
- [ ] `Cargo.toml` và `package.json` **không đổi** (D7).
- [ ] **Layout hàng nhập liệu (đo bằng CDP, dán số thật):**
      - [ ] `input`, `context donut`, `nút Gửi` có **cùng tâm y** (chênh lệch ≤ 4px).
      - [ ] Thứ tự trong DOM: input → donut → nút Gửi.
      - [ ] Donut **không** còn `position:absolute`/`fixed`; `offsetParent` của nó là hàng nhập liệu
            (hoặc dùng `getComputedStyle(...).position === "static"`).
      - [ ] Donut nằm **giữa** input và nút (so sánh `getBoundingClientRect().left` của 3 phần tử).
      - [ ] Hover donut → popover mở **lên trên** (rect của popover có `bottom` ≤ rect của donut `top`)
            và **không bị cắt** (popover hiện đủ 3 dòng, `getBoundingClientRect().height > 0`).
- [ ] **Tóm tắt (đo bằng DOM/CDP + dán text thật):**
      - [ ] Mở dialog `Chi tiết` → nội dung là **câu văn**, không phải JSON (không bắt đầu bằng `{`).
      - [ ] Có **đúng 6 đoạn** (đếm `.split("\n\n")` hoặc số `<p>`).
      - [ ] Với super json rỗng: mọi chỗ thiếu đều in `[chưa có thông tin]`; dán toàn bộ text.
      - [ ] Sau khi điền vài field (dùng `super_json_set` qua IPC hoặc mock): text chứa đúng giá trị đã
            điền và **không** còn placeholder ở vị trí đó. Dán text trước/sau.
      - [ ] Enum in tiếng Việt: `VI` → `tiếng Việt`, `CITY` → `thành phố`, `OPEN` → `để ngỏ`. Dán bằng chứng.
      - [ ] Nút toggle `JSON` bật/tắt được, và khi bật thì thấy JSON thô.
- [ ] Browser mode: **0 console error / 0 exception**.
- [ ] Ảnh chụp: (a) hàng nhập liệu 3 phần + popover đang mở, (b) dialog tóm tắt với placeholder,
      (c) dialog tóm tắt đã có dữ liệu.
- [ ] `MERLINK_HOME=/tmp/merlink-e2e pnpm tauri dev` → compile, cửa sổ sống > 30s, không panic.
- [ ] Port 1420/1421 free; không còn process `merlink-harness`/`vite`/`chromium`.
- [ ] `docs/` không đổi; chỉ file trong allow-list bị sửa.
- [ ] Báo cáo trung thực, dán số/text thật. KHÔNG bịa.

───────────────────────────────────────────────────────
🛑 STOP CONDITIONS
───────────────────────────────────────────────────────
- Tất cả ACCEPTANCE pass → DỪNG, báo "SPRINT DONE"
- Retry 1 task quá 2 lần vẫn fail → DỪNG, escalate
- Cần dependency hoặc cần gọi model để tóm tắt → DỪNG, hỏi (D2/D7)
- Drift guard fail vì không muốn thêm contract row → DỪNG, báo (không được nới lỏng test)
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
5. Cần user quyết định thẩm mỹ/câu chữ

───────────────────────────────────────────────────────
🚫 OUT OF SCOPE
───────────────────────────────────────────────────────
- File được sửa/tạo: `src-tauri/src/{super_json,runtime,lib}.rs`, `src-tauri/src/ipc_contract_tests.rs`,
  `src/pages/Chat.tsx`, `src/components/ContextDonut.tsx`, `src/types.ts`, `src/lib/tauri.ts`,
  `src/index.css`. KHÔNG sửa `docs/`.
- KHÔNG đổi công thức `context_usage` hay 3 phân loại (vừa chốt ở 11.10).
- KHÔNG đổi markdown renderer / nút sao chép / `Tổng:` / history mũi tên (11.91).
- KHÔNG đụng blob/atmosphere (11.92).
- KHÔNG làm session, KHÔNG dialog gợi ý option, KHÔNG sinh dàn ý.
- KHÔNG đổi shape `super.json` / 25 path / công thức tiến độ 70%/30% / giới hạn 5 vòng.
- KHÔNG tái cấu trúc module / tụ JSON / design pattern (`sprint-15.md` LOCKED).
- KHÔNG thêm feature ngoài GOAL, KHÔNG update README/docs.

───────────────────────────────────────────────────────
📤 HANDOFF FORMAT
───────────────────────────────────────────────────────

STATUS: [DONE | PARTIAL | BLOCKED | NEED_INPUT]
SPRINT: 11.11

✅ COMPLETED / ❌ FAILED / 🤔 NEED_INPUT / 📋 NEXT ACTION (như các sprint trước)

═══════════════════════════════════════════════════════

GHI CHÚ KỸ THUẬT (bắt buộc đọc):
- **Bạn ĐƯỢC PHÉP và PHẢI tự khởi động `pnpm dev` / `MERLINK_HOME=/tmp/merlink-e2e pnpm tauri dev`**
  như background job, chờ bằng poll (`ss -ltnp | grep 1420`), không sleep mù. Đừng dừng để xin phép.
- **Tauri v2 chờ key camelCase ở phía JS**: `super_json_summary(project_id)` → gọi bằng `{ projectId }`.
  Sai snake_case → "missing required key".
- **ĐỪNG dùng `gnome-screenshot`** (trên máy này `-w` có lúc chụp cả desktop, lọt dữ liệu riêng tư).
  Chỉ dùng CDP driver trong `/tmp` (`cdp-check.mjs`, `cdp-phases.mjs`, `cdp-sprint115.mjs`),
  chromium cần `--no-sandbox`. URL: `http://localhost:1420/#/chat`.
- Sửa `src/index.css` phải ghi **nguyên tử** (file tạm + `os.replace`). Ghi kiểu truncate từng làm Vite phục
  vụ CSS rỗng, trang mất sạch style và làm sai lệch cả một phép đo.
- `cmd | tail` che exit code → `echo "EXIT=${PIPESTATUS[0]}"`.
- `pkill -f "pattern"` tự khớp shell → `pkill -f "[p]attern"`.
- Test chạm storage phải dùng `crate::test_support::TempHome`; `MERLINK_HOME` là biến toàn process.
- `test_real_endpoint_agent_loop` (ignored) là test **flaky có sẵn**: nó assert model chọn đúng field, mà
  model không tất định. Nếu nó fail, ghi rõ là flaky và chạy lại 1 lần — **không** coi là regression của
  sprint này, và **không** sửa nó trong sprint này.
