═══════════════════════════════════════════════════════
SPRINT 11.91 — CHAT RENDERING: MARKDOWN + SAO CHÉP + NHÃN THỜI GIAN + HISTORY MŨI TÊN
═══════════════════════════════════════════════════════

CONTEXT:
- Sprint 11.8 đã xong (live status thu gọn thành 1 dòng mờ). Đọc `docs/plan/sprint-11.8.md` để biết state.
- **Sprint 11.9 gộp đã bị tách** thành 11.91 (file này) và 11.92 (`sprint-11.92.md`). Một lần chạy gộp đã bị
  dừng giữa chừng và **không để lại thay đổi thực chất nào** trong `src/pages/Chat.tsx` (đã kiểm: không có
  `Tổng:`, không copy, không markdown, vẫn còn `Tổng thời gian lượt` và meta line cũ). `pnpm build` xanh.
  ⇒ Bắt đầu từ cây sạch, nhưng **đọc lại `Chat.tsx` trước khi sửa** (11.8 vừa đổi vùng `.chat-live-status`).
- Người dùng báo nguyên văn: *"cập nhật thêm cái hiển thị md trên phản hồi nha, hiện tại nó hiện ra định
  dạng md nhưng chưa được render nhìn lỏ, với lại thêm nút sao chép tin nhắn của người dùng với tin phản
  hồi của merlink dưới từng phản hồi nữa, thời gian tổng phản hồi chỉ hiển thị tiếng việt: Tổng: .. s
  thôi. nút sao chép rồi tới cái dòng hiện thời gian nhé. Với người dùng khi ấn nút "top" trên bàn phím
  nó duyệt ngược lên được các tin nhắn trước đó"*
- Sprite hiện tại: tin nhắn render bằng text thuần với `whiteSpace: pre-wrap`, nên `**đậm**`, `#` , ``` ```
  hiện ra nguyên xi. `applyAgentEvent` đã fold về 1 phase (11.8).
- Repo **không có JS test runner** (không vitest/jest). Ràng buộc thật → xem D6.

DECISIONS ĐÃ CHỐT (không hỏi lại, không tự đổi):
- **D1 — Markdown do mình render, KHÔNG thêm dependency.** `src/lib/markdown.ts` (parser thuần → AST) +
  `src/components/Markdown.tsx` (AST → **React element**). **TUYỆT ĐỐI KHÔNG `dangerouslySetInnerHTML`** —
  render bằng React text node thì không có bề mặt XSS. Đây là lý do không dùng `marked` + `dompurify`.
- **D2 — Tập con markdown, đúng danh sách này, không hơn:**
  đoạn văn (ngăn bởi dòng trống) · xuống dòng đơn → `<br>` · `#`/`##`/`###` · code block có rào ``` ```
  (kèm ngôn ngữ tuỳ chọn) · list `-`/`*` một cấp · list `1.` · `>` · `---` · inline `**đậm**`,
  `*nghiêng*`, `` `code` ``, `[chữ](url)` → `<a target="_blank" rel="noreferrer noopener">`.
  Không bảng, không ảnh, không highlight. Cú pháp ngoài danh sách → hiện **nguyên văn**, không được nuốt chữ.
- **D3 — Nút sao chép dưới TỪNG tin** (cả user và Merlink): icon button, `aria-label="Sao chép tin nhắn"`,
  icon đổi sang dấu check ~1.5s. Copy **markdown gốc**, không phải text đã render.
  `navigator.clipboard.writeText` trước, fallback textarea + `document.execCommand("copy")`, cả hai fail thì
  **hiện lỗi nhìn thấy được**, không im lặng.
- **D4 — Nhãn thời gian: chỉ `"Tổng: X.Xs"`.** Thay hẳn `"Tổng thời gian lượt: ..."`.
  **Bỏ meta line cũ** (`model · provider · latency · tokens`) khỏi hiển thị, chuyển vào `title` của dòng
  thời gian để không mất thông tin. Thứ tự trong footer: **nút sao chép trước, dòng thời gian sau**.
  - Tin Merlink: `[sao chép]` + `Tổng: X.Xs`, X = tổng thời gian lượt nếu biết (`turn_finished`), else
    `meta.latency_ms`.
  - Tin người dùng: chỉ `[sao chép]`.
- **D5 — Mũi tên lên/xuống duyệt tin đã gửi**, chỉ khi input focus:
  `ArrowUp` lần đầu lưu bản nháp đang gõ rồi về tin gần nhất, các lần sau lùi tiếp, dừng ở tin cũ nhất
  (không vòng). `ArrowDown` tiến về tin mới hơn; vượt tin mới nhất → **khôi phục đúng bản nháp đã lưu**.
  Gửi xong reset con trỏ. Input là `<input>` một dòng nên không tranh chấp con trỏ.
- **D6 — Verify không cần JS test runner:** cho **mock browser** (`src/lib/tauri.ts`) trả fixture markdown
  phủ đủ D2 → fixture là test cố định; assert DOM bằng CDP.
- **D7 — Không thêm dependency nào; không đụng Rust.** Thuần frontend.

GOAL:
Phản hồi Merlink render markdown đúng (không còn `**`, `#` thô), mỗi tin có nút sao chép và dòng
`Tổng: X.Xs` cho phản hồi, và mũi tên lên/xuống duyệt lại được tin đã gửi mà không mất bản nháp.

TASKS:
1. `src/lib/markdown.ts`: `parseMarkdown(text: string): MdBlock[]` theo đúng D2. Hàm thuần, không đụng DOM.
2. `src/components/Markdown.tsx`: render AST → React element. Không `dangerouslySetInnerHTML`.
3. `src/pages/Chat.tsx`: dùng `<Markdown>` cho nội dung tin Merlink (tin user giữ text thuần);
   footer mỗi tin theo D3/D4 (đúng thứ tự, bỏ meta line cũ); handler `ArrowUp`/`ArrowDown` theo D5.
4. `src/lib/tauri.ts`: mock browser trả fixture markdown phủ đủ D2.
5. `src/index.css`: block `/* Sprint 11.91 · markdown + message footer */`. Style phải ăn khớp theme
   sáng/tối (dùng `--ink`, `--muted`, `--line`, `--glass-soft`); code block cuộn ngang được, không phá layout.
6. Verify theo ACCEPTANCE (bắt buộc CDP để đo DOM + đọc clipboard + chụp ảnh), dọn process/port.

ACCEPTANCE:
- [ ] `pnpm build` (tsc + vite) → PASS, 0 lỗi type.
- [ ] `grep -rn "dangerouslySetInnerHTML" src/` → **KHÔNG có kết quả**.
- [ ] `package.json` không đổi; `grep -n "marked\|dompurify\|react-markdown" package.json` → không có.
- [ ] **Markdown render (đo DOM qua CDP)**, dán kết quả: sau khi gửi 1 tin, DOM có đủ `<strong>`, `<em>`,
      `<code>`, `<pre>`, `<li>` (cả `ul` và `ol`), `<blockquote>`, `<h1|h2|h3>`, `<hr>`, và `<a>` có
      `target="_blank"` + `rel` chứa `noreferrer`.
- [ ] **Không còn markdown thô:** text hiển thị **không** chứa `**` hay ``` ``` ```.
- [ ] **XSS:** gửi tin chứa `<img src=x onerror="window.__xss=1">` → **trong `.chat-markdown`** không có
      `img` nào, `window.__xss === undefined`, chuỗi đó hiện ra dưới dạng **chữ**. Dán bằng chứng.
      (Đếm toàn document thì vẫn thấy 1 `img` là `logo.png` của shell — ảnh nền có sẵn, không tính.
      Tiêu chí là *payload không tạo ra element*, không phải *document không có ảnh*.)
- [ ] **Nút sao chép:** click nút của 1 tin Merlink → clipboard chứa **markdown gốc** (dùng
      `Browser.grantPermissions` với `clipboardReadWrite` rồi `navigator.clipboard.readText()`).
      Dán nội dung đọc được. Không đọc được clipboard thì nói rõ và chứng minh cách khác.
- [ ] **Thứ tự footer:** DOM cho thấy nút sao chép đứng **trước** dòng thời gian.
- [ ] **Nhãn thời gian:** DOM chỉ có `Tổng: \d+\.\d+s`; **không** còn `Tổng thời gian lượt`, **không** còn
      `in /` `out`. Dán text footer thật.
- [ ] **History mũi tên** (CDP, dán giá trị input từng bước):
      gửi `A` rồi `B`; `ArrowUp`→`B`; `ArrowUp`→`A`; `ArrowUp`→vẫn `A`; `ArrowDown`→`B`;
      `ArrowDown`→rỗng; và gõ nháp `nhap-dang-do` rồi `ArrowUp`,`ArrowDown` → quay lại đúng `nhap-dang-do`.
- [ ] Browser mode: **0 console error / 0 exception** xuyên suốt.
- [ ] Ảnh chụp: markdown đã render (có code block + list + link) và footer có nút sao chép + `Tổng:`.
- [ ] `cd src-tauri && cargo test` vẫn 71 passed / 3 ignored (không đụng Rust, chỉ là chốt chặn).
- [ ] Chỉ file trong allow-list bị sửa; `docs/` không đổi. Port 1420/1421 free sau khi xong.
- [ ] Báo cáo trung thực, dán số đo thật. KHÔNG bịa.

───────────────────────────────────────────────────────
🛑 STOP CONDITIONS
───────────────────────────────────────────────────────
- Tất cả ACCEPTANCE pass → DỪNG, báo "SPRINT DONE"
- Retry 1 task quá 2 lần mà vẫn fail → DỪNG, escalate
- Thấy cần dependency cho markdown/clipboard → DỪNG, hỏi (D1/D7)
- Thấy cần sửa Rust → DỪNG, hỏi (D7)
- Phát hiện bug sprint trước ảnh hưởng → DỪNG, báo NEED_INPUT
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
5. Cần user test thủ công

───────────────────────────────────────────────────────
🚫 OUT OF SCOPE
───────────────────────────────────────────────────────
- File được sửa/tạo: `src/lib/markdown.ts` (mới), `src/components/Markdown.tsx` (mới),
  `src/pages/Chat.tsx`, `src/lib/tauri.ts` (chỉ mock fixture), `src/index.css` (chỉ style markdown + footer).
  KHÔNG sửa Rust. KHÔNG sửa `docs/`.
- **KHÔNG làm phần của 11.92**: KHÔNG xoá khối Local Mode trong `App.tsx`, KHÔNG đụng `App.tsx`,
  KHÔNG tối ưu blob. Đó là sprint riêng (`sprint-11.92.md`).
- KHÔNG làm donut context, KHÔNG làm dialog Super JSON (11.10).
- KHÔNG bảng markdown, KHÔNG render ảnh, KHÔNG highlight cú pháp.
- KHÔNG session list/resume/delete, KHÔNG dialog gợi ý option, KHÔNG logic sinh dàn ý.
- KHÔNG đổi shape `super.json`, công thức tiến độ, giới hạn 5 vòng agent loop.
- KHÔNG tái cấu trúc module / tụ JSON / design pattern (`sprint-15.md` LOCKED).
- KHÔNG thêm feature ngoài GOAL, KHÔNG tối ưu premature, KHÔNG update README/docs.

───────────────────────────────────────────────────────
📤 HANDOFF FORMAT
───────────────────────────────────────────────────────

STATUS: [DONE | PARTIAL | BLOCKED | NEED_INPUT]
SPRINT: 11.91

✅ COMPLETED:
- [task N]: [file changed]

❌ FAILED (nếu có):
- [task]: [lỗi gì] | [đã thử gì] | [cần gì từ user]

🤔 NEED_INPUT (nếu có):
- ...

📋 NEXT ACTION:
- DONE → "Ready for sprint 11.92"
- BLOCKED → "Waiting for user response on X"
- PARTIAL → "Continue task Y after user confirms"

═══════════════════════════════════════════════════════

GHI CHÚ KỸ THUẬT (bắt buộc đọc):
- `cmd | tail` che mất exit code → `echo "EXIT=${PIPESTATUS[0]}"`.
- `pkill -f "pattern"` tự khớp shell đang chạy lệnh đó → `pkill -f "[p]attern"`.
- App dùng `HashRouter`; URL browser mode `http://localhost:1420/#/chat`.
- `agent-browser` KHÔNG có. CDP driver có sẵn: `/tmp/cdp-check.mjs`, `/tmp/cdp-phases.mjs`,
  `/tmp/cdp-sprint115.mjs`. Chromium cần `--no-sandbox`.
- Bấm phím thật: focus input bằng `Runtime.evaluate`, rồi `Input.dispatchKeyEvent` với
  `type:"rawKeyDown"`, `key:"ArrowUp"`, `code:"ArrowUp"`, `windowsVirtualKeyCode:38` (ArrowDown: 40).
  Set `input.value` trực tiếp sẽ KHÔNG tới được React state.
- Đọc clipboard: `Browser.grantPermissions` với
  `{origin:"http://localhost:1420", permissions:["clipboardReadWrite","clipboardSanitizedWrite"]}`
  trước, rồi `navigator.clipboard.readText()`.
- Fixture markdown nằm ở mock trong `src/lib/tauri.ts` — đó là test fixture của bạn, sửa nó cho phủ đủ D2.
- 11.8 vừa sửa vùng live-status trong `Chat.tsx`; đọc lại file trước khi sửa để không revert.
