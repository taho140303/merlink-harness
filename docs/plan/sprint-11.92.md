═══════════════════════════════════════════════════════
SPRINT 11.92 — DỌN "LOCAL MODE" + TRUY NGUYÊN & GIẢM CPU CỦA ATMOSPHERE
═══════════════════════════════════════════════════════

CONTEXT (dữ kiện đã verify, không phải phỏng đoán):
1. **Vì sao app biến mất (đã đóng, không phải việc của sprint này):** kernel OOM killer giết `node` của
   `pnpm tauri dev` lúc `11:22:14`, đúng 11s sau tin nhắn cuối của người dùng; swap còn 44 kB trống.
   Người dùng xác nhận lúc đó **đang chạy dev trong terminal**. App là con của tiến trình đó nên chết
   theo. ⇒ Không có fix code nào cho việc này.
2. **Nhưng lag thì thật, và app có phần.** Người dùng nói rõ: *"nhưng mà nó đang thật sự lag đấy"*.
   Đo được `WebKitWebProcess` ăn **36–40% CPU khi idle** (và ~70–100% khi render bằng software).
3. **Hai nghi phạm trong CSS, chưa xác định cái nào chiếm phần lớn:**
   - 4 × `.merlink-blob` `width/height:470px`, `filter:blur(64px)`, `will-change:transform`, keyframes
     animate **cả `scale`** → scale một lớp đã blur buộc engine **tính lại blur mỗi frame**.
   - `.surface`, `.btn`, `.field`, `.pill`, `.dialog-panel` dùng `backdrop-filter: blur(12px)`. Khi phía
     sau có nền **động**, backdrop-filter phải composite lại mỗi frame trên diện tích lớn — đây có thể
     còn tốn hơn blob.
   ⇒ **Không được tối ưu trước khi biết cái nào là thủ phạm.** Xem D2 bước "truy nguyên".
4. **Máy đang cạn RAM thật:** 16 GB, swap 4,0/4,0 GB dùng, còn ~2 MB. Không có thay đổi CSS nào cứu được
   việc đó; nhưng 36% CPU idle vẫn là thứ đáng sửa và là thứ làm "lag" nặng thêm khi đang swap.

---

## ⚠️ CẬP NHẬT PHƯƠNG PHÁP ĐO — ĐỌC TRƯỚC KHI LÀM D2

**Dữ kiện mới (2026-10-19):** một lượt chạy trước đã đo baseline thật và bị chặn ở bước truy nguyên:
`WebKitWebProcess` PID 636410 cho **36,9%** (mẫu 2, `top -b -n 2 -d 5`). Nhưng **WebKitGTK KHÔNG có CDP**,
nên không thể `Runtime.evaluate` để chèn style vào cửa sổ app đang chạy. Các script `/tmp/cdp-*.mjs` mở
**Chromium headless riêng** → dùng chúng để đo CPU của app thật là **không hợp lệ**.

**Phương pháp thay thế (thay hẳn D2 bước 2) — A/B bằng sửa file + Vite HMR:**

App dev chạy qua Vite, nên **sửa `src/index.css` là HMR đẩy thay đổi vào app đang chạy ngay**, không cần
devtools. Vì vậy mỗi điều kiện đo = một lần sửa CSS tạm → chờ HMR áp → đo 2 mẫu → **revert**.

Thứ tự điều kiện phải đo (mỗi cái 2 mẫu, cùng route, cửa sổ hiển thị + focus, không gõ gì, ~10s/mẫu):

| # | Sửa tạm trong `src/index.css` | Trả lời câu hỏi gì |
|---|---|---|
| 1 | (không sửa) | baseline |
| 2 | bỏ `will-change:transform` khỏi `.merlink-blob` | fix miễn phí có tác dụng không |
| 3 | `.merlink-blob{display:none}` | trần chi phí của blob |
| 4 | `backdrop-filter:none` + `-webkit-backdrop-filter:none` trên các surface | backdrop-filter có phải thủ phạm chính |
| 5 | bỏ `scale()` khỏi 4 keyframes | phương án (i) |
| 6 | `blur(64px)` → `blur(40px)` | phương án (ii) |
| 7 | tắt hẳn animation của blob | phương án (iii) |

**Bắt buộc chứng minh CSS tạm đã thật sự áp vào app**, vì không có DOM để kiểm: chụp **đúng cửa sổ app**
bằng `xwininfo -root -tree` (tìm cửa sổ `merlink-harness`) rồi `import -window <id> <png>`. Ở điều kiện 3
blob phải **biến mất khỏi ảnh**; ở điều kiện 4 các tấm kính phải mất độ mờ. Ảnh nào không đổi = HMR chưa ăn,
phải chờ và chụp lại — **không được đo khi chưa xác nhận**.

**Ảnh để so "look" thì chụp ở browser mode headless** (cùng CSS, cùng DOM, tất định, và không chụp màn hình
riêng tư của người dùng). Ảnh cửa sổ thật chỉ dùng để chứng minh thay đổi đã áp.

**Revert sạch:** cuối sprint, file chỉ được chứa đúng 2 fix không-đổi-hình. Chứng minh bằng grep rằng
`scale(` vẫn còn trong keyframes và `blur(64px)` vẫn còn — tức các phương án 5/6/7 **không** bị để lại.

Nếu điều kiện 4 (backdrop-filter) là mức giảm lớn nhất → **DỪNG, báo NEED_INPUT** kèm số: bỏ blur khỏi
surface là đổi hình, phải để người dùng quyết.

---

DECISIONS ĐÃ CHỐT (không hỏi lại, không tự đổi):
- **D1 — Bỏ hẳn khối "Local Mode / All data stored on device".**
  - `src/App.tsx` (~dòng 232-241): xoá **toàn bộ** `<div className="merlink-local-mode">…</div>`, gồm
    `merlink-local-dot` + `merlink-local-copy` + `<strong>Local Mode</strong>` +
    `<span>All data stored on device</span>`. Xoá cả khối, không chỉ hai dòng chữ — một chấm xanh không
    nhãn là rác thị giác.
  - `src/index.css`: xoá **mọi** rule `.merlink-local-*` (`.merlink-local-mode`, `-dot`, `-copy`,
    `-copy strong`, `-copy span`, và các biến thể trong `.merlink-app.is-collapsed` và media query).
    CSS bị minify trên một dòng dài → gỡ bằng chuỗi chính xác rồi verify bằng grep.
  - Không được để sidebar vỡ layout: nút Settings vẫn phải nằm sát đáy.
- **D2 — Truy nguyên TRƯỚC, tối ưu SAU. Đúng thứ tự này, không đảo:**
  1. **Baseline.** Mở app, để yên ở trang Chat (không gõ gì), **cửa sổ hiển thị và có focus**, ~10 giây.
     Đo `%CPU` của tiến trình webview (`ps -eo pid,pcpu,rss,comm | grep -i webkit`, lấy tiến trình RSS lớn
     nhất; nếu nghi ngờ thì báo cả process tree). Ghi lại `matchMedia('(prefers-reduced-motion: reduce)').matches`
     — nếu là `true` thì animation đã bị rule toàn cục tắt sẵn và số baseline sẽ nói lên điều khác.
     **Dán số.**
  2. **Truy nguyên bằng override tạm trong DevTools/CDP (KHÔNG sửa file):**
     (a) `.merlink-blob{display:none}` → đo
     (b) `backdrop-filter:none` + `-webkit-backdrop-filter:none` trên `.surface,.surface-soft,
         .surface-interactive,.btn,.field,.pill,.dialog-panel` → đo
     (c) cả (a) và (b) → đo
     ⇒ Báo cáo **cái nào chiếm phần lớn**. Đây là kết luận chính của sprint.
  3. **Chỉ áp những thay đổi KHÔNG đổi hình, thuộc về thủ phạm đã xác định** (ví dụ, nếu blob chiếm phần
     lớn): `animation-play-state: paused` khi cửa sổ ẩn/mất focus (qua `document.visibilitychange` +
     `window` `blur`/`focus`), và bỏ `will-change:transform` khỏi `.merlink-blob`. Nếu backdrop-filter mới
     là thủ phạm thì **DỪNG, báo NEED_INPUT** — bỏ blur khỏi `.surface` là đổi hình rõ rệt, cần người dùng
     quyết.
  4. **Đo lại** sau thay đổi, dán số.
  5. **Đo + chụp 3 phương án ĐỔI HÌNH, KHÔNG áp vào code:**
     (i) bỏ `scale()` khỏi keyframes, chỉ animate `translate`
     (ii) `blur(64px)` → `blur(40px)` + bù gradient mềm hơn
     (iii) blob tĩnh hoàn toàn (không animation)
     Với mỗi cái: `%CPU` idle + 1 ảnh. Trình bày **bảng** để người dùng chọn. **Cấm tự chọn.**
- **D3 — Bước 3 phải giữ nguyên hình.** Computed style của `.merlink-blob` (width/height/filter/opacity/
  background/position) **giống hệt** trước–sau; chỉ `will-change` được phép khác.
- **D4 — Không thêm dependency, không đụng Rust, không đụng `Chat.tsx`.** Sprint này chỉ `App.tsx` +
  `index.css`.
- **D5 — Không đụng `prefers-reduced-motion`**: rule toàn cục hiện có phải giữ nguyên hành vi.

GOAL:
Khối "Local Mode" biến mất khỏi sidebar (không vỡ layout), và có **số đo** xác định cái nào thật sự đốt
CPU idle (blob hay backdrop-filter), kèm mức giảm đạt được từ các thay đổi không đổi hình và một bảng
3 phương án đổi hình để người dùng chọn.

TASKS:
1. Đo baseline CPU như D2.1, dán số + giá trị `prefers-reduced-motion`.
2. Xoá khối Local Mode (D1) trong `App.tsx` + toàn bộ CSS `.merlink-local-*`. Chụp ảnh sidebar trước/sau.
3. Đo 7 điều kiện theo bảng ở mục "CẬP NHẬT PHƯƠNG PHÁP ĐO" (A/B bằng sửa CSS tạm + HMR), dán 7 cặp số
   và kết luận **cái nào chiếm phần lớn**. Mỗi điều kiện phải có ảnh cửa sổ chứng minh CSS đã áp.
4. Áp các thay đổi **không-đổi-hình** (pause khi ẩn/mất focus + bỏ `will-change`), đo lại, dán số.
5. Trình bày bảng 3 phương án đổi hình (i/ii/iii) với số đo + ảnh browser mode, **không để lại trong code**.
6. Verify theo ACCEPTANCE, dọn process/port.

ACCEPTANCE:
- [ ] `grep -rn "merlink-local\|Local Mode\|All data stored on device" src/` → **0 kết quả**.
- [ ] Ảnh sidebar: dưới nút Settings **không còn** khối đó; nút vẫn sát đáy, không vỡ layout. Kèm ảnh
      trạng thái sidebar **thu gọn** (`.is-collapsed`) để chứng minh cũng không vỡ.
- [ ] **Có 7 cặp số `%CPU`** theo bảng phương pháp đo (baseline · bỏ will-change · ẩn blob · tắt
      backdrop-filter · bỏ scale · blur 40 · tĩnh). Dán bảng đầy đủ, kèm cách đo đã dùng.
- [ ] **Có kết luận truy nguyên** bằng lời: blob hay backdrop-filter chiếm phần lớn, kèm số chứng minh.
- [ ] **Mỗi điều kiện đo có 1 ảnh cửa sổ** chứng minh CSS tạm đã thật sự áp (điều kiện ẩn blob: blob phải
      biến mất khỏi ảnh). Ảnh không đổi = HMR chưa ăn → không được tính là đã đo.
- [ ] **Revert sạch:** cuối sprint `grep -c "scale(" src/index.css` vẫn > 0 và `grep -c "blur(64px)"` vẫn > 0
      (tức phương án 5/6/7 không bị để lại), và chỉ có đúng 2 fix không-đổi-hình trong diff.
- [ ] `%CPU` sau khi áp thay đổi không-đổi-hình: **giảm được bao nhiêu**, dán số trước/sau.
- [ ] **Computed style của `.merlink-blob` không đổi** (trừ `will-change`): dán giá trị
      width/height/filter/opacity/background trước và sau.
- [ ] **Bảng 3 phương án đổi hình** (i/ii/iii) có `%CPU` + ảnh từng cái, và **không phương án nào** đã được
      áp vào code (`grep "scale" src/index.css` vẫn còn `scale` trong keyframes nếu chưa được duyệt).
- [ ] `pnpm build` → PASS, 0 lỗi type.
- [ ] `cd src-tauri && cargo test` → vẫn 71 passed / 3 ignored.
- [ ] `MERLINK_HOME=/tmp/merlink-e2e pnpm tauri dev` → compile, cửa sổ sống > 30s, không panic.
- [ ] Chỉ `src/App.tsx` + `src/index.css` bị sửa; `docs/` không đổi. Port 1420/1421 free sau khi xong.
- [ ] Báo cáo trung thực, dán số thật. Nếu không đo được `%CPU` thì nói rõ và dùng cách khác có số.

───────────────────────────────────────────────────────
🛑 STOP CONDITIONS
───────────────────────────────────────────────────────
- Tất cả ACCEPTANCE pass → DỪNG, báo "SPRINT DONE"
- Retry 1 task quá 2 lần vẫn fail → DỪNG, escalate
- **Truy nguyên cho ra backdrop-filter là thủ phạm chính** → DỪNG ở bước 3, báo NEED_INPUT kèm số
  (giảm blur của `.surface` là đổi hình, phải để người dùng quyết)
- Không đo được `%CPU` của webview sau 2 cách thử → DỪNG, báo NEED_INPUT (đừng tối ưu mù)
- Thấy cần dependency hoặc cần sửa Rust → DỪNG, hỏi
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
2. Truy nguyên chỉ ra backdrop-filter (xem STOP)
3. Cần đổi kiến trúc ngoài allow-list
4. Bug sprint trước ảnh hưởng sprint này
5. Cần user quyết định thẩm mỹ

───────────────────────────────────────────────────────
🚫 OUT OF SCOPE
───────────────────────────────────────────────────────
- File được sửa: `src/App.tsx`, `src/index.css`. KHÔNG sửa file nào khác. KHÔNG sửa `docs/`.
- **KHÔNG làm phần của 11.91**: KHÔNG đụng `src/pages/Chat.tsx`, KHÔNG markdown, KHÔNG nút sao chép,
  KHÔNG nhãn `Tổng:`, KHÔNG history mũi tên. Đó là `sprint-11.91.md`.
- KHÔNG làm donut context, KHÔNG làm dialog Super JSON (11.10).
- KHÔNG đổi màu/kích thước/vị trí/opacity của blob ở bước 3 (D3).
- KHÔNG đổi `backdrop-filter` của surface khi chưa được duyệt.
- KHÔNG session list/resume/delete, KHÔNG dialog gợi ý option, KHÔNG logic sinh dàn ý.
- KHÔNG tái cấu trúc module / tụ JSON / design pattern (`sprint-15.md` LOCKED).
- KHÔNG thêm feature ngoài GOAL, KHÔNG update README/docs.

───────────────────────────────────────────────────────
📤 HANDOFF FORMAT
───────────────────────────────────────────────────────

STATUS: [DONE | PARTIAL | BLOCKED | NEED_INPUT]
SPRINT: 11.92

✅ COMPLETED:
- [task N]: [việc đã làm + output thật]

❌ FAILED / 🤔 NEED_INPUT:
- [câu hỏi kèm số liệu]

📋 NEXT ACTION:
- DONE → "Chọn phương án đổi hình i/ii/iii (hoặc không đổi)"
- BLOCKED → "Waiting for user response on X"

═══════════════════════════════════════════════════════

GHI CHÚ KỸ THUẬT (bắt buộc đọc):
- `cmd | tail` che mất exit code → `echo "EXIT=${PIPESTATUS[0]}"`.
- `pkill -f "pattern"` tự khớp shell đang chạy lệnh đó → `pkill -f "[p]attern"`.
- Đo CPU phải **cùng điều kiện** cho mọi lần: cùng route (`http://localhost:1420/#/chat` hoặc app Tauri
  ở trang Chat), cửa sổ **hiển thị + focus**, không gõ gì, ~10 giây, lấy trung bình 2 lần đo.
  Nếu cửa sổ bị ẩn/không focus thì chính cái pause-when-hidden sẽ làm số liệu vô nghĩa.
- **Đo bằng A/B sửa file + HMR, KHÔNG bằng devtools.** WebKitGTK không có CDP nên không chèn style vào app
  đang chạy được. Sửa `src/index.css` → Vite HMR đẩy vào app → chờ → đo → revert. Mỗi lần revert phải
  chờ HMR áp lại rồi mới đo điều kiện kế tiếp, nếu không số sẽ dính điều kiện trước.
- Chụp cửa sổ app: `xwininfo -root -tree | grep -i merlink` để lấy window id, rồi
  `import -window <id> /tmp/x.png`. Nếu `import` fail (Wayland), thử `grim -g "$(xwininfo -id <id> | grep -o '[0-9]*x[0-9]*+[0-9]*+[0-9]*' | head -1)"`.
  Ảnh cửa sổ chỉ để chứng minh CSS đã áp — **không** dùng để so look.
- App dùng `HashRouter`. CDP driver có sẵn ở `/tmp/*.mjs`; chromium cần `--no-sandbox`.
- `ps -eo pcpu` trả về **trung bình từ lúc process sinh ra**, không phải tức thời — với app vừa khởi động
  thì số này sai lệch. Ưu tiên `top -b -n 2 -d 5` (lấy mẫu thứ 2) hoặc đọc `/proc/<pid>/stat` hai lần
  cách nhau ~10s rồi tự tính. Nói rõ bạn dùng cách nào.
- Máy đang swap 100%: số đo có thể dao động mạnh. Đo 2 lần và báo cả hai, đừng chọn số đẹp.
