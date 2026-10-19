═══════════════════════════════════════════════════════
SPRINT 11.13 — CỠ CHỮ + BỎ CATALOG PROVIDER CỨNG + NEW CHAT Ở SIDEBAR + SELECT DESIGN SYSTEM
═══════════════════════════════════════════════════════

CONTEXT:
- Sprint 11.12 vừa xong: live frame trong khối phản hồi, nút cờ lê + dialog "Công cụ đã gọi",
  "Đã suy nghĩ N giây", luật hỏi ≤ 3 câu và không bịa field, tóm tắt trau lại; 97 test xanh.
  Đọc `docs/plan/sprint-11.12.md`.
- **Người dùng vừa nêu 6 yêu cầu, đây là nguồn sự thật của sprint này:**
  1. "Font chữ nhỏ quá" → cần **chỗ cấu hình cỡ chữ trong Settings**.
  2. "Bỏ cái fix cứng model provider đi" — và khi được hỏi lại, người dùng trả lời nguyên văn:
     **"Xóa sạch hết, file cấu hình json đó cũng bị chặn push lên github"**.
     Tức: **xoá toàn bộ catalog provider cứng** (10 provider seed + preset model + preset context window),
     chỉ giữ provider người dùng tự thêm; và `config.json` phải nằm trong `.gitignore` (sprint 12).
  3. "Nút new chat đặt trong sidebar trên cả home luôn, header không còn new chat nữa."
  4. "Vị trí của new chat thay thành viết dàn ý" → nút `Viết dàn ý` về cuối hàng action (chỗ New chat cũ).
  5. "Select box header chưa bắt theo design system của project."
- Trạng thái config thật của người dùng (`~/.merlink/config.json`): 10 provider seed **api_key rỗng**
  (openai, anthropic, deepseek, google, qwen, glm, zai, kimi, command-code, ollama) + 1 provider tự thêm
  `custom-1792378417139` ("sad", có key). Header vì thế vẫn hiện OpenAI/gpt-4o.
- Sprint 11.5 đã có nút "fetch models"; giữ nguyên năng lực đó.

DECISIONS ĐÃ CHỐT (không hỏi lại, không tự đổi):

- **D1 — Cỡ chữ là một setting thật, áp cho toàn app bằng zoom của webview.**
  - 4 mức: `100%` (Nhỏ), `110%` (Vừa — **mặc định**), `125%` (Lớn), `140%` (Rất lớn).
  - Lưu ở `localStorage["merlink-text-scale"]` (string số, ví dụ `"110"`).
  - Áp dụng: `src/lib/preferences.ts` xuất `TEXT_SCALE_OPTIONS`, `readTextScale() -> number` (0.9–1.4,
    mặc định 1.1), `applyTextScale(scale: number): Promise<void>`:
    - Trong Tauri: `getCurrentWebview().setZoom(scale)` (đã có `@tauri-apps/api/webview`, **không** thêm dep).
    - Browser mode: `document.documentElement.style.zoom = String(scale)`.
    - Nuốt lỗi + `console.error`, không được làm sập UI.
  - `App.tsx` gọi `applyTextScale(readTextScale())` khi mount (cạnh `applyStoredTheme`).
  - Settings tab **General** có khối "Cỡ chữ" dùng cùng khuôn UI với "Giao diện" (`merlink-theme-options`):
    4 nút, nút đang chọn có `is-active`, mỗi nút ghi nhãn + phần trăm.
  - `src-tauri/capabilities/default.json` thêm permission `"core:webview:allow-set-webview-zoom"`
    (đã xác nhận có trong schema). Không thêm command mới cho việc này.

- **D2 — Xoá sạch catalog provider cứng.**
  - `store::default_providers()` trả `Vec::new()`. Xoá helper `provider()` khỏi production code.
    `load_providers()` khi chưa có file trả `vec![]` (không seed gì).
  - **Migration một lần** trong `load_providers()`: nếu file tồn tại, bỏ mọi entry mà
    `id` thuộc danh sách 10 id seed cũ **VÀ** `api_key.trim()` rỗng (tức entry chưa ai động tới).
    Entry user tự thêm (`custom-<timestamp>`, "sad", …) **không đụng**. Nếu file thay đổi thì ghi lại
    bằng `save_providers` để lần sau không phải lọc lại. Danh sách id seed khai báo là hằng
    `LEGACY_SEEDED_IDS` với comment nói rõ đây là cleanup một lần, không phải catalog.
  - `SettingsDialog.tsx`: xoá hẳn `PROVIDER_CATALOG` và `MODEL_OVERRIDES`. Luồng "Thêm provider" mở form
    **trống** (id/name/baseURL/apiKey rỗng, kind mặc định `custom`, models rỗng); người dùng tự điền
    Base URL, key, rồi bấm "Fetch models" (giữ nguyên) hoặc thêm model tay. Model mới mặc định
    `context_window = "128K"`, `max_tokens = "4K"`, `reasoning_level = false`, sửa được.
  - Danh sách provider rỗng → Settings hiện empty state: "Chưa có provider nào. Thêm provider để bắt đầu."
  - `lib/tauri.ts` (mock browser) giữ **2 provider mẫu** để preview được (không còn mirror Rust);
    sửa comment cho đúng. `MODEL_PRESETS` của mock được phép giữ vì chỉ là preview.

- **D3 — Chat thích nghi với danh sách provider rỗng.**
  - `loadProviders()`: chọn provider mặc định = **provider đầu tiên có `api_key` khác rỗng**, nếu không
    có thì provider đầu tiên; model mặc định = `default_model_id` nếu còn tồn tại, ngược lại model đầu.
  - `providers.length === 0`: select provider hiển thị một option disabled "Chưa cấu hình provider";
    nút Send disabled; dưới hàng nhập liệu hiện dòng gợi ý nhỏ:
    "Thêm provider trong Settings (góc dưới bên trái) để bắt đầu."
  - Không đổi shape `send_chat`/`context_usage`.

- **D4 — New chat nằm ở sidebar, hiện trên MỌI trang (kể cả Home).**
  - Chèn nút `New chat` (icon `Plus`, style `btn btn-primary`, full width) ngay dưới brand, trên nav.
    Khi sidebar collapsed: chỉ còn icon vuông, tooltip `New chat` (dùng đúng cơ chế tooltip của nav link).
  - Bấm: `navigate("/chat")` **và** phát `window.dispatchEvent(new Event("merlink:new-chat"))`.
  - `Chat.tsx` nghe event đó (effect `[]`, callback ổn định qua `useCallback`) và gọi `handleNewChat()`.
  - Xoá nút `New chat` khỏi header Chat.

- **D5 — Header Chat: hàng action mới.**
  Thứ tự cuối cùng, từ trái sang phải:
  `[progress] [Chi tiết] [provider select] [model select] [Viết dàn ý]`
  - `Viết dàn ý` nằm ở **vị trí cuối** (chỗ New chat cũ), giữ nguyên logic `canOutline`/disabled.
  - Không còn nút New chat.

- **D6 — Select box header theo design system.**
  - Component mới `src/components/SelectBox.tsx`: wrapper `.merlink-select-wrap` + `<select class="merlink-select">`
    + icon `ChevronDown` tuyệt đối bên phải (`pointer-events:none`).
  - CSS khớp token của project: `border:1px solid var(--line)`, `border-radius:12px`,
    `background:var(--glass)`, `backdrop-filter:blur(12px)`, `box-shadow:var(--shadow)`,
    `min-height:40px` (bằng `.btn`), `padding:0 36px 0 14px`, `font-size:12.5px;font-weight:650`,
    `color:var(--ink)`, `appearance:none`, hover `var(--glass-strong)`, focus
    `border-color:var(--accent)` + `outline` theo `:focus-visible` toàn cục.
  - Dùng cho **provider** và **model** ở header. Giữ `aria-label` ("Provider", "Model").
  - Không đổi style các select khác (Settings) trong sprint này.

- **D7 — Không thêm dependency** (Rust lẫn npm). Không đổi shape `super.json`, 25 path, 70/30, 5 vòng.

GOAL:
Settings có mục "Cỡ chữ" áp ngay toàn app; không còn provider cứng nào (config cũ được dọn, chỉ giữ
provider tự thêm); New chat nằm ở sidebar và dùng được từ Home; header Chat còn
`Chi tiết | provider | model | Viết dàn ý` với select đúng design system.

TASKS:
1. `src-tauri/src/store.rs`: D2 (`default_providers` rỗng, `LEGACY_SEEDED_IDS`, migration + rewrite,
   `#[cfg(test)] pub fn catalog_fixture()` giữ catalog cũ cho test).
2. `src-tauri/src/lib.rs`: test `test_default_providers_are_empty` thay test cũ.
3. `src-tauri/src/runtime.rs` + `src-tauri/src/ipc_contract_tests.rs`: chuyển các chỗ đang dùng
   `default_providers()` làm fixture sang `catalog_fixture()`; `agtron_providers()` tự dựng provider
   thay vì `next().unwrap()`.
4. `src/lib/preferences.ts` (mới) + `src/App.tsx` (áp zoom, nút New chat sidebar) + `App` nhận
   `navigate`.
5. `src/components/SettingsDialog.tsx`: cỡ chữ (D1) + bỏ catalog (D2) + empty state.
6. `src/pages/Chat.tsx`: D3, D4 (nghe event), D5, D6.
7. `src/components/SelectBox.tsx` (mới).
8. `src/lib/tauri.ts`: mock 2 provider, comment đúng; `localStorage` mặc định cỡ chữ.
9. `src/index.css`: block `/* Sprint 11.13 · text scale + sidebar new chat + design-system select */`.
   Ghi file **nguyên tử**.
10. `src-tauri/capabilities/default.json`: thêm permission zoom.
11. Verify ACCEPTANCE, dọn process/port.

ACCEPTANCE:
- [ ] `cd src-tauri && cargo test` → PASS, 0 warning, có các test mới:
      `test_default_providers_are_empty`,
      `test_legacy_seeded_providers_are_pruned`,
      `test_custom_provider_survives_the_seed_cleanup`.
- [ ] `pnpm build` → PASS, 0 lỗi type.
- [ ] `Cargo.toml`/`package.json`/`pnpm-lock.yaml` **không đổi**.
- [ ] **Cỡ chữ (CDP, browser mode):** mở Settings → General → thấy 4 nút cỡ chữ; bấm `125%` thì
      `document.documentElement.style.zoom === "1.25"` và `localStorage["merlink-text-scale"] === "125"`;
      reload → vẫn `1.25`. Đo chiều cao một dòng chữ (`getBoundingClientRect().height` của bubble chat)
      trước/sau, dán cả hai số.
- [ ] **Catalog đã sạch (đo thật):**
      - [ ] `cargo test` chứng minh `default_providers().is_empty()`.
      - [ ] Migration: dựng một `MERLINK_HOME` tạm với config chứa 10 seed rỗng key + 1 `custom-*` có key →
            `load_providers()` trả **đúng 1** provider (`custom-*`), và file trên đĩa đã bị ghi lại
            (đọc lại file, không còn 10 seed).
      - [ ] Settings (browser mode) **không** còn danh sách preset nào để chọn khi thêm provider; form
            thêm mới bắt đầu trống. Dán screenshot.
      - [ ] Chat với danh sách rỗng: select hiện "Chưa cấu hình provider", Send disabled, có dòng gợi ý.
- [ ] **New chat ở sidebar (CDP):**
      - [ ] Có nút `New chat` trong `.merlink-sidebar` khi ở `/` (Home) và ở `/chat`.
      - [ ] Bấm từ Home → URL `#/chat`, transcript trống, `sessionId` mới (kiểm tra qua việc không còn
            message cũ / banner trống "Start writing together").
      - [ ] Header Chat **không** còn nút New chat: `document.querySelectorAll('.page-topbar-actions button')`
            không chứa chữ "New chat".
- [ ] **Thứ tự header (CDP):** text của `.page-topbar-actions` theo đúng thứ tự
      `Chi tiết` → provider → model → `Viết dàn ý`; nút `Viết dàn ý` là phần tử cuối.
- [ ] **Select design system (CDP):** 2 select header có class `merlink-select`; computed style:
      `border-radius:12px`, `min-height:40px`, và background chứa `rgba` của `--glass`; có icon chevron
      (`.merlink-select-wrap svg`). Dán computed style thật.
- [ ] Browser mode: 0 console error / 0 exception.
- [ ] Ảnh chụp: (a) sidebar có New chat ở Home, (b) Settings cỡ chữ, (c) header mới với select.
- [ ] `docs/` không đổi; chỉ file trong allow-list bị sửa.
- [ ] Port 1420/1421 free, không còn process vite/chromium.

───────────────────────────────────────────────────────
🛑 STOP CONDITIONS
───────────────────────────────────────────────────────
- Tất cả ACCEPTANCE pass → DỪNG, báo "SPRINT DONE"
- Retry 1 task quá 2 lần vẫn fail → DỪNG, escalate
- Cần dependency mới → DỪNG, hỏi (D7)
- Permission `core:webview:allow-set-webview-zoom` không tồn tại → DỪNG, báo
- Task vượt 40 phút ước tính → DỪNG, chia nhỏ lại

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
4. Test cũ mâu thuẫn với D2 mà không nằm trong danh sách được phép sửa

───────────────────────────────────────────────────────
🚫 OUT OF SCOPE
───────────────────────────────────────────────────────
- File được sửa/tạo: `src-tauri/src/{store,lib,runtime}.rs`, `src-tauri/src/ipc_contract_tests.rs`,
  `src-tauri/capabilities/default.json`, `src/lib/preferences.ts` (mới),
  `src/components/SelectBox.tsx` (mới), `src/components/SettingsDialog.tsx`, `src/pages/Chat.tsx`,
  `src/App.tsx`, `src/lib/tauri.ts`, `src/index.css`.
- KHÔNG sửa `docs/`.
- KHÔNG đổi luồng fetch models / edit provider ngoài việc bỏ catalog.
- KHÔNG đổi `super_json`, agent loop, markdown, donut, dialog `Chi tiết`.
- KHÔNG làm theme mới, không i18n, không đổi ngôn ngữ.
- KHÔNG refactor module (`sprint-15.md` LOCKED).

───────────────────────────────────────────────────────
📤 HANDOFF FORMAT
───────────────────────────────────────────────────────

STATUS: [DONE | PARTIAL | BLOCKED | NEED_INPUT]
SPRINT: 11.13

✅ COMPLETED / ❌ FAILED / 🤔 NEED_INPUT / 📋 NEXT ACTION

═══════════════════════════════════════════════════════

GHI CHÚ KỸ THUẬT (bắt buộc đọc):
- **Bạn ĐƯỢC PHÉP và PHẢI tự khởi động `pnpm dev` / `MERLINK_HOME=/tmp/merlink-e2e pnpm tauri dev`**
  như background job, chờ bằng poll (`ss -ltnp | grep 1420`), không sleep mù.
- **Tauri v2 chờ key camelCase ở phía JS.**
- Browser mode verify: `pnpm dev` + Chromium headless (`--no-sandbox`), `http://localhost:1420/#/chat`,
  CDP. Script mẫu còn ở `/tmp/cdp-sprint1112.mjs` (copy rồi sửa).
- **ĐỪNG dùng `gnome-screenshot`.**
- Sửa `src/index.css` phải ghi **nguyên tử** (file tạm + `os.replace`).
- `cmd | tail` che exit code → `echo "EXIT=${PIPESTATUS[0]}"`.
- `pkill -f "pattern"` tự khớp shell → `pkill -f "[p]attern"`.
- Test chạm storage phải dùng `crate::test_support::TempHome` / `RequiredHome`.
- **ĐỪNG sửa `~/.merlink/config.json` thật của người dùng.** Mọi test migration phải dùng home tạm.
- `test_real_endpoint_agent_loop` là test flaky có sẵn — không sửa.
- Máy thiếu RAM: đừng chạy song song nhiều build/webview.
