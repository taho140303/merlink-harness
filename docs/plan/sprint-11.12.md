═══════════════════════════════════════════════════════
SPRINT 11.12 — LIVE TRONG KHỐI PHẢN HỒI + NÚT CỜ LÊ + LUẬT GHI FIELD + TRAU TÓM TẮT
═══════════════════════════════════════════════════════

CONTEXT:
- Sprint 11.11 vừa xong: hàng nhập liệu `[input][donut][Send]` cùng hàng, dialog `Chi tiết` hiển thị
  tóm tắt 6 đoạn tiếng Việt sinh ở Rust (`super_json::summary`), 85 test xanh, `pnpm build` xanh.
  Đọc `docs/plan/sprint-11.11.md` để biết hợp đồng cũ.
- **Người dùng gửi 4 ảnh chụp màn hình + 1 danh sách lỗi. Đây là nguồn sự thật của sprint này:**
  1. "Đang suy nghĩ" hiện đang **nằm ở góc trên bên trái khung chat** (một dòng `position:absolute`),
     tách rời khỏi phản hồi. Người dùng muốn nó **nằm trong/trên chính khối phản hồi của Merlink**,
     dạng **khung mờ** cho thấy Merlink đang làm gì.
  2. Sau khi phản hồi xong, transcript vẫn phơi ra các chip `super_json_set ✓ {"path":…,"value":…}`.
     Người dùng: **"sau khi phản hồi xong không cần hiện tool nào được gọi trường nào được field"**.
  3. Người dùng muốn **dưới nút sao chép có thêm nút hình cờ lê**; bấm vào mở dialog liệt kê các tool đã
     gọi, dạng câu: **"<tên field> được ghi nhận với <data của field>"**.
  4. Người dùng muốn kiểu **"Thought for N seconds"** của DeepSeek web: hiện thời gian suy nghĩ gắn với
     khối phản hồi, gập/mở được.
  5. **Agent hỏi quá nhiều.** Tin nhắn thật trong ảnh: 3 câu hỏi đánh số, mỗi câu lại có 3–4 câu con kèm
     ví dụ dài (logline bị xé thành Trigger-event / Action / Risk / Major-loss). Người dùng:
     **"mỗi lần hỏi chỉ hỏi tối đa 3 câu để lấy được field đó thôi"**.
  6. **Agent ghi field không được phép.** Người dùng: *"có trường nào thì field trường đó, cấp dữ liệu
     nào thì cập nhật chính xác tuyệt đối dữ liệu đó chứ mới kêu viết truyện 120 chương mà dám gọi hàm
     đặt tên cho truyện thì quá láo"*.
  7. **Tóm tắt còn lỗi câu chữ** (do template của 11.11, người dùng đã đồng ý cho trau lại):
     - `… ký ức có làm nên con người?.` — dấu `?` bị thêm `.` → `?.`
     - `Nhân vật chính là một người thợ đồng hồ mù, người truy tìm người đã chế tạo ra nó.` — lặp "người".
     - `khoảng 90000 từ` — không phân cách nghìn.
     - `Dự kiến dài [chưa có thông tin] arc` — câu cụt khi thiếu dữ liệu.
- Repo **không có git**. `docs/` là của người dùng, **executor không được sửa** (file sprint này do
  assistant viết, đã nằm sẵn).

DECISIONS ĐÃ CHỐT (không hỏi lại, không tự đổi):

- **D1 — Live frame nằm TRONG khối phản hồi, chỉ tồn tại trong lúc chạy.**
  - Xoá hẳn cách hiện tại: dòng `.chat-live-status` `position:absolute;top:8px;left:26px`.
  - Khi `loading === true`, render **một khối là con cuối cùng của `[role="log"]`** (trong vùng cuộn),
    canh trái, `max-width:76%` (bằng bong bóng assistant), bo góc cùng họ với bong bóng.
  - Khối gồm:
    - Hàng đầu: spinner + nhãn bước đang chạy + **số giây đang trôi** (tabular-nums).
    - Dưới đó: tối đa **3 bước đã xong gần nhất**, mỗi bước `✓ <nhãn đã hoàn tất>`.
  - Nhãn "đang chạy" lấy từ `status` của `tool_started` ("Đang ghi nhận tên truyện", …); khi xong, đổi
    tiền tố `Đang ` → `Đã ` ("Đã ghi nhận tên truyện"). Nếu chưa có event nào: "Đang chờ phản hồi từ model…".
  - "Khung mờ": nền kính mờ (`var(--glass-soft)` + `backdrop-filter: blur(10px)`), viền nét đứt
    `var(--line)`, có nhịp thở rất nhẹ (pulse) để thấy nó đang sống. Người dùng dùng từ "khung mờ".
  - Số giây: đếm từ lúc bắt đầu lượt, tick 200ms, hiển thị `N giây` (dưới 60s) hoặc `M phút S giây`.
  - Khi lượt kết thúc (thành công **hoặc** lỗi) khối này **biến mất hoàn toàn**, không để lại dấu vết.

- **D2 — Sau khi xong, transcript KHÔNG còn dấu vết tool.**
  - Xoá component `ToolChip`, `PendingToolCalls`, nhánh render `role === "tool"`, và các helper chỉ chúng
    dùng (`summarizeArguments`, `toolFailed`). Gỡ luôn class `.chat-tool-chip` nếu không còn dùng.
  - Message assistant **rỗng nội dung và chỉ có `tool_calls`** → không render gì.
  - Message `role:"tool"` → không render gì.
  - Nghĩa là sau một lượt, transcript chỉ còn: bong bóng user, các câu trả lời prose của assistant.

- **D3 — Nút cờ lê + dialog "Công cụ đã gọi".**
  - Trong footer của **message assistant cuối lượt** (message có `meta`), đặt cạnh nút Copy một nút icon
    `Wrench` (cùng kích thước/style với `.chat-copy`), `aria-label="Công cụ đã gọi"`.
  - Nút chỉ hiện khi lượt đó **thực sự gọi tool** (có ít nhất 1 tool call).
  - Bấm → mở `Dialog` (dùng `src/components/Dialog.tsx` sẵn có) tiêu đề **"Công cụ đã gọi"**.
  - Mỗi dòng đúng định dạng câu:
    - `super_json_set` thành công: `{Nhãn field} được ghi nhận với {giá trị}` — nhãn viết hoa chữ đầu.
      Ví dụ: `Tên truyện được ghi nhận với "Chuyển sinh thành sói"`.
    - `super_json_set` thất bại: `{Nhãn field} chưa được ghi nhận` + dòng lỗi nhỏ bên dưới.
    - `project_create`: `Dự án được tạo với tên "{name}"`; nếu tạo không kèm tên: `Dự án được tạo (chưa có tên)`.
    - Tool khác (`super_json_get`, `super_json_missing`, `echo`): `Đã gọi {tool}`.
  - Giá trị hiển thị: string → trong dấu ngoặc kép `"…"`; number/bool → in thô; array → nối `, `.
  - Nhãn field lấy từ **Rust** qua command mới `super_json_labels` (D8) — **không** hardcode map thứ hai
    trong `Chat.tsx` (đã có `MOCK_PATH_LABELS` cho browser mode, được phép dùng lại ở mock).
  - Gom nhóm theo lượt: reset khi gặp `role === "user"`; gom `tool_calls` của các assistant message;
    đối chiếu kết quả bằng `tool_call_id` của message `role:"tool"`; gắn vào message có `meta`.
    Tool call không có message kết quả → coi là thất bại.

- **D4 — Header "Đã suy nghĩ N giây" (kiểu DeepSeek web).**
  - Phía trên bong bóng assistant (chỉ message có `meta`), một dòng nhỏ:
    `Đã suy nghĩ {N} giây` — N = **tổng thời gian model** của lượt = tổng `duration_ms` của các
    `round_finished`. Nếu không có event (đường demo không API key) → fallback `meta.latency_ms`.
  - Nếu message có `reasoning` khác rỗng → kèm nút chevron; bấm để mở/đóng panel chứa nội dung suy nghĩ
    (chữ nhỏ, màu `var(--muted)`, `white-space:pre-wrap`). **Mặc định đóng.**
  - Không có `reasoning` → chỉ hiện dòng thời gian, không chevron, không bấm được.
  - Footer **giữ** `Tổng: X.Xs` = tổng thời gian lượt. Sửa lỗi cũ: footer phải đọc
    `message.meta.latency_ms` (đúng của từng lượt), **không** dùng state `turnDurationMs` toàn cục
    (state đó khiến mọi bong bóng cũ hiển thị thời gian của lượt mới nhất). Xoá state `turnDurationMs`.
  - Lưu thời gian model theo message: `turnStats: Record<string, number>` (messageId → modelMs), gán khi
    lượt resolve cho message có `meta`.

- **D5 — Trau lại `super_json::summary`.** Vẫn đúng **6 đoạn**, vẫn `[chưa có thông tin]` cho dữ liệu
  thiếu, vẫn map enum sang tiếng Việt (giữ nguyên bảng ở 11.11 · D4). Sửa 4 lỗi câu chữ + các ca thiếu
  dữ liệu. Template mục tiêu (đây là hợp đồng, test phải khớp):

  **Khi rỗng hoàn toàn** (skeleton) — đúng 21 placeholder:
  ```
  Dự án truyện có tên là [chưa có thông tin], thuộc thể loại [chưa có thông tin]. Tác giả: [chưa có thông tin]. Truyện viết bằng [chưa có thông tin], hướng tới độc giả [chưa có thông tin], độ tuổi [chưa có thông tin].

  Sự kiện mở đầu: [chưa có thông tin]. Nhân vật chính: [chưa có thông tin]. Hành động chính: [chưa có thông tin]. Rủi ro: [chưa có thông tin]. Mất mát lớn: [chưa có thông tin].

  Bộ truyện đặt ra câu hỏi: [chưa có thông tin]. Thông điệp hướng tới: [chưa có thông tin]. Cách trả lời được chọn: [chưa có thông tin], với nội dung: [chưa có thông tin].

  Số arc dự kiến: [chưa có thông tin]. Số chương dự kiến: [chưa có thông tin]. Tổng số từ mục tiêu: [chưa có thông tin].

  Thế giới có hệ thống sức mạnh hay không: [chưa có thông tin]. Phạm vi không gian sâu nhất cần dựng: [chưa có thông tin].

  Tóm tắt nội dung: [chưa có thông tin].
  ```

  **Khi đầy đủ** (`filled_document()`) — đúng các câu sau:
  ```
  Dự án truyện có tên là Nửa Đêm, thuộc thể loại trinh thám, kèm huyền bí, gia đình. Tác giả là Nguyễn Văn A. Truyện viết bằng tiếng Việt, hướng tới độc giả người lớn, độ tuổi 16+.

  Câu chuyện bắt đầu khi chiếc đồng hồ cuối cùng ngừng chạy. Nhân vật chính là một người thợ đồng hồ mù, truy tìm người đã chế tạo ra nó. Mối nguy là đánh mất ký ức, và cái giá có thể phải chịu là đứa con gái duy nhất.

  Bộ truyện đặt ra câu hỏi: ký ức có làm nên con người? Thông điệp hướng tới là ký ức định nghĩa con người. Cách trả lời được chọn là để ngỏ, với nội dung người đọc tự quyết định.

  Dự kiến dài 5 arc, 120 chương, khoảng 90.000 từ. Sai số chấp nhận: 1 arc, 10 chương, 5.000 từ.

  Thế giới có hệ thống sức mạnh. Phạm vi không gian sâu nhất cần dựng là thành phố.

  Tóm tắt nội dung: Một người thợ đồng hồ mù đi tìm ký ức đã mất.
  ```

  **Quy tắc câu (bắt buộc):**
  - **Không lặp từ nối**: bỏ hẳn `, người {action}`. Khi có cả nhân vật và hành động:
    `Nhân vật chính là {character}, {action}.` (xem câu mẫu ở trên).
  - **Không nhân đôi dấu câu**: giá trị đã kết thúc bằng `.`/`?`/`!`/`…` thì **không** thêm `.` nữa.
    Áp dụng cho mọi giá trị đứng cuối câu (trigger, câu hỏi chủ đề, mô tả, …).
  - **Số có phân cách nghìn bằng dấu `.`**: `90000` → `90.000`, `5000` → `5.000`, `240000` → `240.000`.
    Áp dụng cho arc/chapter/word/tolerance.
  - **Không in câu cụt kiểu "dài [chưa có thông tin] arc"**:
    - Có đủ cả 3 số (arc, chương, từ) → `Dự kiến dài {arc} arc, {chapter} chương, khoảng {word} từ.`
    - Thiếu bất kỳ số nào → tách thành 3 câu nhãn: `Số arc dự kiến: {arc}. Số chương dự kiến: {chapter}. Tổng số từ mục tiêu: {word}.` (chỗ thiếu vẫn là placeholder).
    - Câu "Sai số chấp nhận: …" **chỉ in khi có ít nhất một tolerance**; tolerance nào thiếu thì placeholder tại đúng vị trí đó.
  - **Ngôi kể nhất quán**: dùng dạng nhãn `X: giá trị.` khi thiếu, dạng câu tự nhiên khi có:
    - `Tác giả là X.` / thiếu: `Tác giả: [chưa có thông tin].`
    - `Sự kiện mở đầu: …` khi thiếu; `Câu chuyện bắt đầu khi X.` khi có.
    - `Rủi ro: …` khi thiếu; `Mối nguy là X.` khi có.
    - `Mất mát lớn: …` khi thiếu; `Cái giá có thể phải chịu là X.` khi có. Khi có **cả** risk và major-loss thì dùng câu gộp `Mối nguy là X, và cái giá có thể phải chịu là Y.`
    - `Thông điệp hướng tới là X.` / thiếu: `Thông điệp hướng tới: [chưa có thông tin].`
    - Power system: `Thế giới có hệ thống sức mạnh.` (true) / `Thế giới không có hệ thống sức mạnh.` (false) / thiếu: `Thế giới có hệ thống sức mạnh hay không: [chưa có thông tin].`
    - Scale: `Phạm vi không gian sâu nhất cần dựng là X.` / thiếu: `Phạm vi không gian sâu nhất cần dựng: [chưa có thông tin].`
  - `optional.genre.secondary` vẫn là mệnh đề điều kiện `, kèm A, B` (bỏ khi rỗng) — giữ nguyên.
  - Giữ nguyên: 6 đoạn, không có path thô, không có `null`, không padding, tất định.
  - **Chấp nhận thay đổi số placeholder ở test cũ**: 24 → **21** (vì câu "Sai số chấp nhận" không còn in khi cả 3 tolerance đều rỗng). Ghi comment giải thích ngay trong test.

- **D6 — Luật hỏi: tối đa 3 câu mỗi lượt.**
  Thêm mục `# Asking the author` vào `src-tauri/prompts/harness_system.md` với đúng nội dung:
  ```md
  # Asking the author

  - Ask **at most three questions per reply**. If more required fields are missing, ask the three most
    important ones now and leave the rest for the next turn.
  - One question per field. Do not turn one field into a menu of sub-questions (no Trigger-event /
    Action / Risk / Major-loss breakdown in one message), and do not put a list of examples under it:
    at most one short example, and only when it genuinely helps.
  - Never re-ask a field that is already set, and never re-ask something the author answered earlier.
  - After recording fields, confirm in one short line, then ask the next questions.
  ```
  Đồng thời **sửa** mục "Starting a project" đang khuyến khích bịa tên: bỏ câu "Derive `name` from what
  the author just said" và thay bằng luật ở D7.

- **D7 — Luật "chỉ ghi đúng thứ người dùng nói".**
  Thêm mục `# Only write what the author actually said`:
  ```md
  # Only write what the author actually said

  - `super_json_set` writes **exactly one** field with **exactly the value the author stated**. Do not
    round it, translate it, expand it or derive other fields from it.
  - Never invent a value to fill a field, and never write a field the author has not talked about. If the
    author says "viết truyện 120 chương", the only thing you may write is
    `require.overview.plan.chapter.count` = 120 — not a title, not a genre, not a logline.
  - When the author has not named the work, call `project_create` **without** a name (see below). It still
    creates the project, and `require.overview.name` stays empty until the author gives a title.
    **Never make one up.**
  - When it is unclear which field the author meant, ask instead of writing.
  ```
  **Đổi hành vi `project_create`** (đây là gốc rễ của lỗi "tự đặt tên"):
  - Tham số `name` thành **tuỳ chọn** (`required: []` trong schema).
  - Có `name` → hành vi như cũ: slug id + ghi `require.overview.name`.
  - Không có `name`/rỗng → tạo project với id lấy từ base `du-an` (`du-an`, `du-an-2`, …), tên hiển thị
    "Dự án chưa đặt tên", và **không** ghi `require.overview.name`.
  - Kết quả tool: `{"ok":true,"project_id":…,"name":<chỉ khi có>,"missing_required":[…]}`.
  - Mock browser mode phải mô phỏng đúng hành vi này.

- **D8 — Command mới `super_json_labels`.**
  - `super_json_labels() -> BTreeMap<String, String>` trả đủ **25** cặp path → nhãn tiếng Việt.
  - Nguồn duy nhất: `tools::PATH_LABELS`. Thêm `pub fn field_labels() -> Vec<(&'static str, &'static str)>`
    trong `tools.rs` (hoặc `pub const`) rồi `lib.rs` map sang `BTreeMap`.
  - Thêm vào `merlink_handlers!` và **một dòng contract** trong `ipc_contract_tests.rs`
    (`body: json!({})`), cộng test end-to-end `test_ipc_super_json_labels_end_to_end` (đủ 25, có
    `require.overview.name` → `tên truyện`).

- **D9 — Bắt `reasoning_content` (để header "Đã suy nghĩ" mở ra được nội dung thật).**
  - `AdapterReply` thêm `pub reasoning: Option<String>`; `parse_openai_response` đọc
    `choices[0].message.reasoning_content` (string, trim, rỗng → `None`).
    Anthropic: để `None` trong sprint này (không bật extended thinking).
  - `MessageEnvelope` thêm `#[serde(default, skip_serializing_if = "Option::is_none")] pub reasoning: Option<String>`;
    cập nhật 2 constructor (`new`, `tool_result`) → `None`.
  - `run_agent_turn` cộng dồn reasoning của **mọi round** vào `turn_reasoning`; gắn vào **message cuối
    lượt** (message có `meta`), gồm cả nhánh chạm trần 5 vòng. Message trung gian không mang reasoning.
  - **Không** đưa `reasoning` lên wire gửi model (hàm `openai_turns`/`anthropic_turns` đã dựng JSON thủ
    công nên an toàn — chỉ cần kiểm tra, không sửa nếu không cần).

- **D10 — Không thêm dependency** (Rust lẫn npm). Không đổi shape `super.json`, 25 path, công thức
  70/30, giới hạn 5 vòng.

GOAL:
Một lượt chat trông như sau: khung mờ "Đang suy nghĩ · N giây" nằm **trong** vùng phản hồi (kèm các bước
đã ghi nhận), khi xong thì khung biến mất và chỉ còn câu trả lời + dòng "Đã suy nghĩ N giây" (mở ra được
nội dung suy nghĩ) + nút cờ lê mở dialog "Công cụ đã gọi". Agent hỏi tối đa 3 câu/lượt và **không bao giờ
tự bịa** giá trị field, kể cả tên truyện. Tóm tắt 6 đoạn đọc tự nhiên, số có phân cách nghìn, không lặp
từ, không nhân đôi dấu câu.

TASKS:
1. `src-tauri/src/super_json.rs`: viết lại `summary()` theo D5; thêm helper `terminate()` (thêm `.` nếu
   cần) và `with_thousands(i64)`. Cập nhật 3 test cũ (`..._marks_missing_fields_as_placeholder`,
   `..._includes_all_filled_values`, `..._handles_partial_data`) và thêm test mới (xem ACCEPTANCE).
2. `src-tauri/src/tools.rs`: `pub fn field_labels()`; `project_create` cho phép thiếu `name` (D7);
   cập nhật `tool_specs()` (bỏ `required: ["name"]`, sửa description).
3. `src-tauri/src/adapters.rs`: `AdapterReply.reasoning` + parse `reasoning_content` (D9).
4. `src-tauri/src/models.rs`: `MessageEnvelope.reasoning` (D9).
5. `src-tauri/src/runtime.rs`: cộng dồn reasoning, gắn vào message cuối (D9).
6. `src-tauri/src/lib.rs`: command `super_json_labels` + thêm vào `merlink_handlers!`.
7. `src-tauri/src/ipc_contract_tests.rs`: dòng contract + test end-to-end cho `super_json_labels`.
8. `src-tauri/prompts/harness_system.md`: D6 + D7 (sửa cả mục "Starting a project").
9. `src/pages/Chat.tsx`: D1 (live frame trong transcript), D2 (bỏ toàn bộ chip tool), D3 (nút cờ lê +
   gom log theo lượt), D4 (header "Đã suy nghĩ", bỏ state `turnDurationMs`), sửa footer dùng
   `meta.latency_ms`.
10. `src/components/ToolLogDialog.tsx` (mới): dialog danh sách log theo D3.
11. `src/types.ts`: `MessageEnvelope.reasoning?: string`.
12. `src/lib/tauri.ts`: mock `super_json_labels` (dùng `MOCK_PATH_LABELS`); mock `project_create` không
    tên; mock gắn `reasoning` cho message cuối để browser mode thấy được panel.
13. `src/index.css`: block `/* Sprint 11.12 · live frame + thinking header + tool log */`. Xoá
    `.chat-live-status*` cũ và `.chat-tool-chip` nếu không còn dùng. Ghi file **nguyên tử**.
14. Verify theo ACCEPTANCE, dọn process/port.

ACCEPTANCE:
- [ ] `cd src-tauri && cargo test` → PASS, 0 warning, gồm các test mới:
      `test_summary_does_not_double_question_mark`,
      `test_summary_action_has_no_repeated_nguoi`,
      `test_summary_formats_thousands_with_dots`,
      `test_summary_uses_labels_when_counts_incomplete`,
      `test_summary_omits_tolerance_sentence_when_unset`,
      `test_labels_command_covers_all_25_paths`,
      `test_system_prompt_limits_questions_per_turn`,
      `test_system_prompt_forbids_inventing_values`,
      `test_project_create_without_name_leaves_title_unset`,
      `test_openai_parses_reasoning_content`,
      `test_agent_loop_attaches_reasoning_to_final_message`,
      `test_ipc_super_json_labels_end_to_end`.
- [ ] `pnpm build` → PASS, 0 lỗi type (kể cả `noUnusedLocals`: xoá sạch helper chết).
- [ ] `Cargo.toml` và `package.json` **không đổi** (D10).
- [ ] **Live frame (đo bằng CDP, dán số thật):**
      - [ ] Trong lúc chạy: tồn tại phần tử `.chat-live-frame` (hoặc `[data-testid="chat-live-frame"]`)
            và `frame.closest('[role="log"]') !== null` → nó nằm **trong** vùng transcript.
            Dán: `frame.getBoundingClientRect()` + `getComputedStyle(frame).position` (`static|relative`,
            **không** `absolute|fixed`).
      - [ ] Không còn phần tử nào khớp `.chat-live-status` / `position:absolute;top:8px`.
      - [ ] Ngay sau khi lượt xong: `document.querySelector('[data-testid="chat-live-frame"]') === null`
            và số phần tử chứa chữ `super_json_set` trong `[role="log"]` = **0**.
      - [ ] Khối hiện ít nhất 1 bước `✓ Đã …` sau khi một tool chạy xong; dán text.
      - [ ] Số giây tăng: đo text ở 2 thời điểm cách nhau ≥ 1s, dán cả hai.
- [ ] **"Đã suy nghĩ N giây" (đo bằng CDP):**
      - [ ] Mỗi message assistant có `meta` hiện đúng 1 dòng khớp `/Đã suy nghĩ \d+ giây/`.
      - [ ] Khi message có `reasoning`: có nút chevron; bấm → panel mở, `aria-expanded="true"`, text
            panel = nội dung reasoning; bấm lần nữa → đóng. Dán text.
      - [ ] Khi message không có `reasoning`: **không** có chevron.
- [ ] **Nút cờ lê + dialog (đo bằng CDP):**
      - [ ] Mỗi lượt có tool: footer có đúng 1 nút `[aria-label="Công cụ đã gọi"]`.
      - [ ] Bấm → dialog mở, tiêu đề "Công cụ đã gọi".
      - [ ] Nội dung dialog chứa đúng câu dạng `Tên truyện được ghi nhận với "…"` (dán toàn bộ text).
      - [ ] Lượt không gọi tool → **không** có nút cờ lê.
- [ ] **Tóm tắt (đo bằng DOM + `cargo test -- --nocapture`):**
      - [ ] Không còn `?.` trong output; câu hỏi chủ đề kết thúc đúng 1 dấu `?`.
      - [ ] Không còn chuỗi `người truy tìm người`.
      - [ ] `90000` → `90.000`; `5000` → `5.000`.
      - [ ] Skeleton: không còn câu `dài [chưa có thông tin] arc`; đúng **21** placeholder; vẫn 6 đoạn.
      - [ ] Dán nguyên văn output skeleton + output đầy đủ.
- [ ] Browser mode: **0 console error / 0 exception**.
- [ ] Ảnh chụp: (a) đang chạy với khung mờ + bước, (b) sau khi xong với header "Đã suy nghĩ", (c) dialog
      "Công cụ đã gọi".
- [ ] `docs/` không đổi; chỉ file trong allow-list bị sửa.

───────────────────────────────────────────────────────
🛑 STOP CONDITIONS
───────────────────────────────────────────────────────
- Tất cả ACCEPTANCE pass → DỪNG, báo "SPRINT DONE"
- Retry 1 task quá 2 lần vẫn fail → DỪNG, escalate
- Cần dependency mới → DỪNG, hỏi (D10)
- Phải sửa `docs/` → DỪNG (trừ ghi chú trong báo cáo)
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
4. Bug sprint trước ảnh hưởng sprint này
5. Test cũ mâu thuẫn với D5 mà không nằm trong danh sách được phép sửa

───────────────────────────────────────────────────────
🚫 OUT OF SCOPE
───────────────────────────────────────────────────────
- File được sửa/tạo: `src-tauri/src/{super_json,tools,adapters,models,runtime,lib}.rs`,
  `src-tauri/src/ipc_contract_tests.rs`, `src-tauri/prompts/harness_system.md`,
  `src/pages/Chat.tsx`, `src/components/ToolLogDialog.tsx`, `src/types.ts`, `src/lib/tauri.ts`,
  `src/index.css`. KHÔNG sửa `docs/` (kể cả file sprint này).
- KHÔNG đổi markdown renderer, history mũi tên, popover donut, dialog `Chi tiết`/`super_json_summary`,
  `context_usage`, blob/atmosphere.
- KHÔNG làm session/resume, KHÔNG sinh dàn ý, KHÔNG streaming từng token (SSE).
- KHÔNG đổi shape `super.json` / 25 path / công thức 70/30 / giới hạn 5 vòng.
- KHÔNG refactor module (`sprint-15.md` LOCKED).
- KHÔNG thêm feature ngoài GOAL, KHÔNG update README.

───────────────────────────────────────────────────────
📤 HANDOFF FORMAT
───────────────────────────────────────────────────────

STATUS: [DONE | PARTIAL | BLOCKED | NEED_INPUT]
SPRINT: 11.12

✅ COMPLETED / ❌ FAILED / 🤔 NEED_INPUT / 📋 NEXT ACTION (như các sprint trước)

═══════════════════════════════════════════════════════

GHI CHÚ KỸ THUẬT (bắt buộc đọc):
- **Bạn ĐƯỢC PHÉP và PHẢI tự khởi động `pnpm dev` / `MERLINK_HOME=/tmp/merlink-e2e pnpm tauri dev`**
  như background job, chờ bằng poll (`ss -ltnp | grep 1420`), không sleep mù. Đừng dừng để xin phép.
- **Tauri v2 chờ key camelCase ở phía JS**: `super_json_labels` không có tham số → gọi
  `safeInvoke("super_json_labels")`; các command khác giữ `{ projectId }` / `{ sessionId }`.
- **Bật browser mode để verify UI**: `pnpm dev` + Chromium headless (`--no-sandbox`),
  URL `http://localhost:1420/#/chat`. Mock nằm trong `src/lib/tauri.ts` — **phải cập nhật mock** thì mới
  thấy được log/reasoning khi không có Tauri.
- **ĐỪNG dùng `gnome-screenshot`** (trên máy này có lúc chụp cả desktop, lọt dữ liệu riêng tư).
  Chỉ dùng CDP driver trong `/tmp` (`cdp-check.mjs`, `cdp-phases.mjs`, `cdp-sprint115.mjs` hoặc viết mới),
  chromium cần `--no-sandbox`.
- Sửa `src/index.css` phải ghi **nguyên tử** (file tạm + `os.replace`). Ghi truncate từng làm Vite phục vụ
  CSS rỗng và làm sai lệch cả một phép đo.
- `cmd | tail` che exit code → `echo "EXIT=${PIPESTATUS[0]}"`.
- `pkill -f "pattern"` tự khớp shell → `pkill -f "[p]attern"`.
- Test chạm storage phải dùng `crate::test_support::TempHome`; `MERLINK_HOME` là biến toàn process.
- `test_real_endpoint_agent_loop` (ignored) là test **flaky có sẵn**: nó assert model chọn đúng field, mà
  model không tất định. Nếu fail, ghi rõ là flaky và chạy lại 1 lần — **không** coi là regression của
  sprint này, và **không** sửa nó trong sprint này.
- Máy thiếu RAM (16GB, swap đầy) — đừng chạy song song nhiều tiến trình build/webview.
