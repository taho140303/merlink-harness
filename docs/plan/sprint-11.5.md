═══════════════════════════════════════════════════════
SPRINT 11.5 — FETCH MODEL THẬT + EDIT PROVIDER CHỈ 1 TAB
═══════════════════════════════════════════════════════

CONTEXT:
- Sprint 11 đã xong (agent phase stream). Đọc `docs/plan/sprint-11.md` để biết state gần nhất.
- Người dùng báo 2 lỗi ở Settings dialog, trích nguyên văn:
  1. *"sao chức năng fetch model không hoạt động vậy, nó chả gọi thằng nào đến bên provider của t cả"*
  2. *"Khi t ấn vào icon cây bút chì của các model provider có sẵn (đang được fake data trên UI) thì edit
     provider dialog hiện ra 2 tab, (có lẽ tái sử dụng từ dialog khi gọi add provider model) nhưng không
     được, chỉ cho một tab cho loại nào đã tạo thôi"*
- **Chẩn đoán đã xác minh bằng cách đọc code, không phải phỏng đoán:**
  - Lỗi 1 nằm ở `src/components/SettingsDialog.tsx`, hàm `fetchModels()` (~dòng 325): nó **không gọi
    backend**. Nó chỉ `setTimeout(..., 1500)` rồi tự chế 2 model giả `${form.id}-preview` và
    `${form.id}-reasoning`. **Backend không hề có command nào để lấy danh sách model** — `lib.rs` chỉ có
    `load_providers`, `save_providers`, `send_chat`, `execute_tool`, `super_json_read`,
    `super_json_progress`, `list_messages`. Nên đây là viết mới, không phải sửa.
  - Lỗi 2 nằm ở khối `.merlink-provider-modes` (~dòng 672-711): **luôn** render 2 nút tab
    "Third-party model provider" và "Custom model API", kể cả khi đang EDIT provider có sẵn
    (`editor.id !== null`). Tệ hơn: `onClick` của mỗi nút gọi `setEditor({ id: null, mode: ... })`,
    tức là **xoá mất id provider đang sửa** và reset form → bấm nhầm tab là biến thao tác "sửa" thành
    "tạo mới", dữ liệu đang nhập bị mất.
- Hàng tab đó nói về **LOẠI PROVIDER** (`editor.mode`: `"third-party" | "custom"`), không phải loại model.
  2 tab chỉ hợp lý ở luồng **ADD** (chưa chọn loại). Ở luồng **EDIT** thì loại đã cố định rồi.
- Endpoint test: `http://127.0.0.1:8045/v1`, api_key `local-dev-placeholder`. Config ở
  `/tmp/merlink-e2e/config.json`.
- Người dùng đã chốt: fetch xong thì **gộp vào danh sách đang có, dedupe theo `id`**, KHÔNG thay thế.

DECISIONS ĐÃ CHỐT (không hỏi lại, không tự đổi — đây là spec, không phải gợi ý):
- **D1 — Fetch model là việc của BACKEND, không phải frontend.** Thêm command Tauri
  `fetch_models(provider_kind, base_url, api_key) -> Vec<FetchedModel>`. Lý do: gọi từ webview sẽ vướng
  CORS, và API key phải nằm ở phía Rust. Frontend KHÔNG được `fetch()` trực tiếp tới provider.
- **D2 — `FetchedModel`** thêm vào `models.rs`: `{ id: String, name: String }`.
  `name` lấy từ field `display_name` nếu provider trả về, không thì lấy chính `id`.
- **D3 — Endpoint chuẩn hoá (hàm thuần, test được):**
  - OpenAI-compatible: `GET {base_url}/models`, header `Authorization: Bearer <api_key>`,
    đọc `data[].id` (+ `data[].name` / `data[].display_name` nếu có).
  - Anthropic: `GET {base}/v1/models` (dùng lại logic chuẩn hoá của `anthropic_endpoint` đang có —
    nhớ là nó trả `/v1/messages`, nên cần biến thể trả `/v1/models`), header `x-api-key` và
    `anthropic-version: 2023-06-01`.
  - `base_url` rỗng hoặc `api_key` rỗng → `Err` ngay, thông điệp rõ ràng (KHÔNG gọi mạng).
- **D4 — Parse tách thành hàm thuần để test offline:** `parse_models_response(body: &Value, kind: &str)
  -> Result<Vec<FetchedModel>, String>`. Chấp nhận cả `data[]` (chuẩn OpenAI/Anthropic mới) và mảng trần
  `[]`. Response không có `data` và không phải mảng → `Err`. Bỏ qua phần tử thiếu `id`. Nếu kết quả rỗng
  → `Err("Provider returned no models")`.
- **D5 — Lỗi fetch phải HIỆN LÊN UI**, không được chỉ `console.error`. Trạng thái trong dialog:
  `fetching` (đang chạy) / `fetchError` (chuỗi lỗi hiện dưới nút). Nút bị `disabled` khi đang fetch.
- **D6 — Gộp + dedupe theo `id` (người dùng đã chốt).** Model fetch về đi qua `createModelConfig` để có
  default hợp lý, chỉ append những `id` chưa tồn tại trong `form.models`; model đang có **không bị sửa**.
  Nếu không có gì mới → hiện thông báo kiểu "Không có model mới".
- **D7 — Xoá SẠCH code giả.** Toàn bộ `fetchTimer`, mọi `setTimeout` liên quan fetch, và 2 model bịa
  `-preview` / `-reasoning` phải biến mất khỏi repo. `grep -rn "preview\|reasoning\" *fake*"` không còn.
  Cụ thể: `grep -rn "\-preview\|\-reasoning" src/` phải KHÔNG ra kết quả nào từ SettingsDialog.
- **D8 — Edit provider chỉ hiện 1 tab.** Khi `editor.id !== null` (đang sửa provider có sẵn):
  chỉ render **đúng 1** control mode, tương ứng `editor.mode` của provider đó, và **không cho đổi loại**
  (render dạng nhãn tĩnh / nút không hành động). Khi `editor.id === null` (luồng ADD) thì giữ nguyên 2 tab
  để chọn loại. Tuyệt đối không được để bấm tab làm `setEditor({ id: null })` khi đang sửa.
- **D9 — `handleSave` khi edit phải giữ nguyên `id` provider đang sửa** (không sinh id mới), và
  `defaultModel` phải được kẹp về một model còn tồn tại sau khi gộp/restore.
- **D10 — Không thêm dependency nào** (cả Rust lẫn npm). `reqwest` đã có sẵn.

GOAL:
Bấm "Fetch available models" gọi thật tới provider và đổ danh sách model thật về (gộp, dedupe, báo lỗi
lên UI nếu thất bại); và khi sửa một provider đã tạo thì dialog chỉ hiện đúng 1 tab loại provider, không
còn đường nào làm mất id provider đang sửa.

TASKS:
1. Pre-flight: `curl` `GET http://127.0.0.1:8045/v1/models` với header `Authorization: Bearer
   local-dev-placeholder` → dán output thật (rút gọn) vào báo cáo. Endpoint chết → STOP, báo BLOCKED.
2. `src-tauri/src/models.rs`: thêm `FetchedModel { id, name }` (serde snake_case khớp TS).
3. `src-tauri/src/adapters.rs`: thêm `models_endpoint(base_url, kind)` (biến thể của
   `anthropic_endpoint` trả `/v1/models`), `parse_models_response(body, kind)` theo D4, và
   `fetch_models(kind, base_url, api_key) -> Result<Vec<FetchedModel>, String>` theo D3.
   Dùng `client()` sẵn có; timeout ngắn hơn (30s) cho endpoint danh sách.
4. `src-tauri/src/runtime.rs`: `fetch_models_internal(kind, base_url, api_key)` gọi `adapters::fetch_models`.
5. `src-tauri/src/lib.rs`: command `fetch_models(provider_kind, base_url, api_key) -> Vec<FetchedModel>`.
   **Nhớ D11 bên dưới về tên tham số phía JS.**
6. `src/components/SettingsDialog.tsx`:
   - `fetchModels` thành `async`, gọi command thật, gộp + dedupe theo D6, set `fetchError` theo D5.
   - Xoá `fetchTimer` + mọi `setTimeout` + code model giả (D7). Dọn cả các chỗ `clearTimeout(fetchTimer.current)`
     rải rác ở `closeEditor`, `switchProvider`, `restoreModels` và 2 `onClick` của tab.
   - Khối `.merlink-provider-modes`: render 1 tab khi `editor.id !== null`, 2 tab khi `editor.id === null` (D8).
   - `handleSave`: giữ id khi edit, kẹp `defaultModel` (D9).
7. `src/types.ts`: thêm `FetchedModel`.
8. `src/lib/tauri.ts`: mock `fetch_models` trả danh sách cố định nhỏ (kèm comment nói rõ đây là preview
   của browser mode, bản thật nằm ở Rust), để luồng UI merge/dedupe chạy được khi không có Tauri.
9. `src/index.css`: style cho dòng lỗi fetch nếu cần (block `/* Sprint 11.5 · fetch models */`).
10. Viết test (BẮT BUỘC) đúng các tên sau:
    `test_parse_openai_models_response`,
    `test_parse_models_response_prefers_display_name`,
    `test_parse_models_response_accepts_bare_array`,
    `test_parse_models_response_rejects_garbage`,
    `test_parse_models_response_skips_entries_without_id`,
    `test_parse_models_response_errors_on_empty_list`,
    `test_models_endpoint_for_openai_and_anthropic`,
    `test_fetch_models_rejects_blank_inputs`.
    (Các test parse/endpoint là hàm thuần, KHÔNG cần `TempHome` vì không chạm storage.)
11. Chạy verify theo ACCEPTANCE, thu log thật, dọn process/port sau khi xong.

ACCEPTANCE:
- [ ] `cd src-tauri && cargo test` → PASS toàn bộ, 0 warning, gồm 8 test mới + test cũ.
- [ ] `cargo test` vẫn KHÔNG ghi gì vào `~/.merlink` thật.
- [ ] `pnpm build` (tsc + vite) → PASS, 0 lỗi type.
- [ ] `grep -rn "preview\|reasoning" src/components/SettingsDialog.tsx` → KHÔNG còn model giả nào.
      `grep -n "setTimeout\|fetchTimer" src/components/SettingsDialog.tsx` → KHÔNG còn dòng nào (D7).
- [ ] `grep -rn "fetch(" src/components/SettingsDialog.tsx src/lib/tauri.ts` → không có `fetch()` trực
      tiếp tới provider (D1).
- [ ] **E2E THẬT fetch_models**: thêm test `#[ignore]` (chạy bằng
      `MERLINK_HOME=/tmp/merlink-e2e cargo test -- --ignored --nocapture real_endpoint`) gọi
      `fetch_models_internal("openai", "http://127.0.0.1:8045/v1", "local-dev-placeholder")` → trả danh sách
      KHÁC RỖNG và chứa `gpt-4o`. Dán output thật vào báo cáo.
- [ ] **E2E THẬT lỗi**: gọi với `api_key` sai → trả `Err` có nội dung HTTP status / thông điệp provider,
      KHÔNG panic, KHÔNG treo. Dán output thật.
- [ ] `MERLINK_HOME=/tmp/merlink-e2e pnpm tauri dev` → compile, cửa sổ sống > 30s, không panic.
- [ ] **CDP — EDIT chỉ 1 tab**: `pnpm dev` + CDP script, mở Settings → tab LLM PROVIDER → bấm nút sửa
      (icon bút chì) của provider OpenAI → đếm `document.querySelectorAll('.merlink-provider-modes button').length`
      → **== 1**. Chụp ảnh. Dán số đếm thật.
- [ ] **CDP — ADD vẫn 2 tab**: bấm nút thêm provider mới → cùng selector đếm → **== 2**. Dán số đếm thật.
- [ ] **CDP — không mất id khi sửa**: khi đang sửa OpenAI, lưu lại → provider vẫn là `openai` (không sinh
      provider mới). Kiểm bằng cách đọc lại danh sách provider sau khi lưu (hoặc đếm số provider trước/sau
      — phải bằng nhau).
- [ ] **CDP — merge/dedupe**: ở luồng ADD (custom), bấm "Fetch available models" 2 lần liên tiếp → số
      model tăng ở lần 1 và **không tăng** ở lần 2 (dedupe), không có model trùng `id`.
- [ ] Browser mode: `0 console error / 0 exception` trong toàn bộ các bước trên.
- [ ] Sau khi xong: port 1420/1421 free, không còn process `merlink-harness`/`vite`/`chromium` treo.
- [ ] Báo cáo có log thật cho từng mục + ảnh chụp màn hình. KHÔNG bịa kết quả.

───────────────────────────────────────────────────────
🛑 STOP CONDITIONS (DỪNG NGAY khi đạt 1 trong các điều này)
───────────────────────────────────────────────────────
- Tất cả ACCEPTANCE pass → DỪNG, báo "SPRINT DONE"
- Retry 1 task quá 2 lần mà vẫn fail → DỪNG, escalate
- Phát hiện task cần design decision không có trong spec → DỪNG, hỏi
- Cần thêm dependency ngoài list (D10: KHÔNG được thêm gì) → DỪNG, hỏi
- Endpoint `127.0.0.1:8045` không phản hồi → DỪNG, báo BLOCKED
- Phát hiện endpoint `/v1/models` trả format khác hẳn `data[].id` → DỪNG, dán raw response và hỏi,
  KHÔNG tự đoán thêm format
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
6. Nảy sinh yêu cầu gì THÊM về session — user CHƯA chốt phần này

───────────────────────────────────────────────────────
🚫 OUT OF SCOPE (KHÔNG được làm)
───────────────────────────────────────────────────────
- File được phép sửa/tạo: `src-tauri/src/{models,adapters,runtime,lib}.rs`,
  `src/components/SettingsDialog.tsx`, `src/types.ts`, `src/lib/tauri.ts`, `src/index.css`.
  KHÔNG sửa file nào trong `docs/`. KHÔNG đụng `Cargo.toml`, `package.json`,
  `capabilities/default.json`.
- KHÔNG làm picker/dialog chọn từng model khi fetch (D6 đã chốt là gộp thẳng).
- KHÔNG đổi format `~/.merlink/config.json` (vẫn là JSON array `Vec<ProviderConfig>`).
- KHÔNG đổi shape `super.json` / 25 path / công thức tiến độ.
- KHÔNG làm session list/resume/rename/delete.
- KHÔNG làm dialog gợi ý option (skip/submit), KHÔNG viết logic sinh dàn ý.
- KHÔNG port `docs/schema/system.dbml` sang SQLite.
- KHÔNG thêm feature ngoài GOAL, KHÔNG đổi UI/UX ngoài D5/D8, KHÔNG optimize premature,
  KHÔNG viết test ngoài 8 test ở task 10 + test online ở ACCEPTANCE, KHÔNG update README/docs.

───────────────────────────────────────────────────────
📤 HANDOFF FORMAT (khi DỪNG, báo cáo theo format này)
───────────────────────────────────────────────────────

STATUS: [DONE | PARTIAL | BLOCKED | NEED_INPUT]
SPRINT: 11.5

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

D11 — GHI CHÚ KỸ THUẬT CHO NGƯỜI THỰC THI (bắt buộc đọc):
- **Tauri v2 chờ key camelCase ở phía JS.** `tauri-macros` chuyển mọi tên tham số Rust bằng
  `to_lower_camel_case`. Command `fetch_models(provider_kind, base_url, api_key)` phải được gọi từ JS bằng
  `{ providerKind, baseUrl, apiKey }`. Gọi sai kiểu snake_case sẽ lỗi "missing required key".
  Đây là bug đã từng xảy ra ở Sprint 9 — đừng lặp lại.
- `cargo test` chạy song song và `MERLINK_HOME` là biến toàn process → test nào chạm storage phải dùng
  `crate::test_support::TempHome`.
- `cmd | tail` che mất exit code → dùng `echo "EXIT=${PIPESTATUS[0]}"`.
- `pkill -f "pattern"` tự khớp shell đang chạy lệnh đó → dùng bracket trick `pkill -f "[p]attern"`.
- App dùng `HashRouter`; URL browser mode là `http://localhost:1420/#/chat`. Settings là dialog trong app,
  mở bằng nút "Settings" ở sidebar dưới cùng.
- `agent-browser` KHÔNG có sẵn. Có CDP driver ở `/tmp/cdp-check.mjs` (Node 22, không cần dep) và
  `/tmp/cdp-phases.mjs` (bản Sprint 11, chụp nhiều thời điểm). Chromium cần `--no-sandbox`.
- rusqlite không có `FromSql` cho `u64`; `rowid` không được xuất hiện trong `CREATE INDEX`.
- `reqwest` client hiện có timeout 120s trong `adapters::client()`; cho endpoint danh sách nên dùng
  timeout ngắn hơn (30s) để UI không treo lâu khi provider sai địa chỉ.
