# Mã giả tạo prompt

Bản nháp cơ chế ghép prompt; mọi quy tắc nghiệp vụ theo `docs/business/core.md`, khi lệch thì core thắng.

Cơ chế: mỗi **prompt block** nạp được `0..N` tool — block nào không cần tool thì registry không build schema nào; các row `prompt_block_tools` của block được nạp mới cấp tool cho run này. Block `tool_library` có thể nhắc tên tool khi cần, không lặp schema. Nhờ vậy bộ nhớ context chỉ chứa đoạn prompt + tool schema mà các block của flow yêu cầu, thay vì nạp cả kho tool.

## Working State, sealed version và audit

* Working State là trạng thái đang xử lý trong bộ nhớ backend; tool result `applied` chỉ xác nhận mutation đã áp dụng vào Working State, không phải durable commit.
* Sealed Draft Version là snapshot đã ghi bền vững trong `draft_versions`; đây là nguồn phục hồi Draft State.
* `messages.payload_json` là audit phục vụ lịch sử và phát hiện gián đoạn, không phải nguồn phục hồi State.
* Cuối lượt có mutation `applied`, backend phải niêm phong version; seal lỗi thì thử lại đúng một lần, cả hai lần lỗi trả `VersionSealFailed` và không trả kết quả thành công giả.
* Một lượt chat chỉ có tối đa một sealed version cho `(draft_session_id, turn_id)`; workflow round có đơn vị niêm phong riêng. Retry không tạo version trùng. Với SF-06 B+, chỉ header/receipts/evidence trong TX-04 chứng minh round đã commit (zero-net vẫn có header, không có Draft Version); `messages.payload_json`, prompt bytes, tool response hay report tạm không thay thế evidence. Artifact typed chỉ bền khi cùng TX-04 và thực sự cần cho downstream, không lưu mọi LLM response. Proposal protected-field ghi bền riêng TX-08 trước khi trả `patch_id`, không chứng minh round completed và không ở pure state patch. Resume chỉ skip sau khi historical verification và current-head applicability đều đạt; không chạy lại LLM/tool để vá evidence thiếu.

## Schema

Nguồn sự thật duy nhất cho `prompt_blocks`, `tools` và `prompt_block_tools` là `docs/db.dbml` (section `USECASE.BUSINESS.PROMPTS`); tài liệu này chỉ giải thích cơ chế ghép prompt và cấp tool, không định nghĩa schema.

## Ngôn ngữ giao tiếp và kho câu hỏi

Mỗi lượt, pipeline lấy `system_language` từ cài đặt giao diện (`vi` hoặc `en`) và bơm tên ngôn ngữ đầy đủ vào system prompt qua `{{system_language}}`. Đây là ngôn ngữ giao tiếp, không phải `project.language` của nội dung sáng tác. Backend giữ kho câu hỏi cố định theo ngôn ngữ, không có kho thì dùng `en`; AI không truyền ngôn ngữ vào tool và không cần tool tra câu hỏi riêng.

```text
// Mã giả
let system_language = settings.ui_language();   // "vi" | "en"; chỉ để giao tiếp, không phải project.language
let fields = get_field_null(ctx, limit = 3);    // [{ field, question }], backend tra kho theo system_language
// AI hỏi đúng các question trả về; không hỏi ngoài các câu này khi hỏi bổ sung trường thiếu
let elems = get_array_element_fields(ctx, array_path, code?);  // cho mảng, vì mảng rỗng không phải null
```

Ví dụ kết quả của `get_field_null` khi `system_language = "vi"`:

```json
[
  {"field": "require.project.display_name", "question": "Bạn muốn đặt tên truyện là gì?"},
  {"field": "require.project.language", "question": "Truyện được viết bằng ngôn ngữ nào?"},
  {"field": "require.project.target.word_per_chapter_count", "question": "Trung bình mỗi chương truyện có bao nhiêu từ?"}
]
```

## Script

getPrompt nạp 1 block theo `prompt_code` — unique nên đủ định danh. `heading_level` là **data của block** (CHECK 1..6), render ngay từ block, không truyền vào từ script để tránh 2 nguồn sự thật. Ký hiệu *(repo)* là chỗ nạp block từ DB — chỉ là pseudocode:

```rust
use std::collections::BTreeSet;
use std::fmt::Write as _;

#[derive(Debug, Clone, Copy)]
pub enum PromptBlockFormat {
    Json,
    Markdown,
}

#[derive(Debug, Clone)]
pub struct PromptBlock {
    pub id: uuid::Uuid,
    pub prompt_code: String,
    pub display_name: String,
    pub heading_level: u8, // 1..=6 — khớp CHECK ở DB
    pub content_format: PromptBlockFormat,
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct Tool {
    pub id: uuid::Uuid,
    pub code: String,
    pub display_name: String,
    pub description: Option<String>,
    pub input_schema: serde_json::Value,
}

/// Metadata phân loại của Tool Registry, không phải cột DBML.
/// Registry map tool code sang metadata này; không suy loại tool từ tiền tố tên.
#[derive(Debug, Clone, Copy)]
pub struct ToolMeta {
    pub writes_draft_state: bool,
}

/// Tool Registry cung cấp metadata qua `meta(tool_code)`.
/// Hàm minh họa; tên/chữ ký thực tế do implementation chốt.
fn is_draft_write(registry: &dyn ToolRegistry, call: &ToolCall) -> bool {
    registry.meta(&call.name).writes_draft_state
}

/// Nạp 1 block theo code — code unique là đủ định danh.
pub fn get_prompt(prompt_code: &str, repo: &dyn PromptRepo) -> Result<String, PromptError> {
    let block = repo.find_by_code(prompt_code)?;

    if !(1..=6).contains(&block.heading_level) {
        return Err(PromptError::InvalidHeadingLevel(block.heading_level));
    }

    let mut out = String::new();
    write!(out, "{} {}\n\n", "#".repeat(block.heading_level as usize), block.display_name)?;

    match block.content_format {
        PromptBlockFormat::Markdown => write!(out, "{}", block.content)?,
        PromptBlockFormat::Json => {
            write!(out, "```json\n{}\n```", block.content)?;
        }
    }

    Ok(out)
}
```

Chỉ cần đổi `script` là compose ra prompt khác: block nào không được khai báo thì không tốn token; tool gắn với bất kỳ block nào được nạp (qua `prompt_block_tools`) mới được đăng ký vào tool schema gửi lên API. `tool_library` không cần nhúng lại schema/mô tả tool.

## Khi nào load tool

Không phải prompt nào của block cũng có tool, nên cần 1 hàm query duy nhất thay cho viết từng biến thể `func_get_prompt_...`:

- Block **có tool** → có row trong `prompt_block_tools` (chỉ dùng để đăng ký function calling).
- Block **thuần văn bản** → không có row nào, `get_tools_by_prompt` trả `vec![]` — compose không rẽ nhánh nào.

Mô tả tool chỉ tồn tại ở `tools.description` + `input_schema` (schema gửi lên API). **Không interpolate danh sách/mô tả tool vào `content`** — nếu content lặp lại mô tả thì token bị tính hai lần; content muốn nhắc thì chỉ nhắc *tên* tool.

```rust
/// Nạp tools của 1 block: JOIN prompt_block_tools;
/// không có row = block thuần văn bản → trả vec![]
pub fn get_tools_by_prompt(
    prompt_code: &str,
    repo: &dyn PromptRepo,
) -> Result<Vec<Tool>, PromptError> {
    repo.find_tools_by_prompt(prompt_code)
}

/// Hàm compose duy nhất: prompt text (phân cách "\n\n") + tools dedupe theo code.
pub fn compose(script: &[&str], repo: &dyn PromptRepo) -> Result<RunSpec, PromptError> {
    let mut blocks = Vec::with_capacity(script.len());
    let mut seen = BTreeSet::new();
    let mut tools = Vec::new();

    for &code in script {
        // trim_end để mỗi block không mang newline thừa; join("\n\n") cho đúng 1 dòng trống giữa các block
        blocks.push(get_prompt(code, repo)?.trim_end().to_owned());

        for t in get_tools_by_prompt(code, repo)? {
            if seen.insert(t.code.clone()) {
                tools.push(t);
            }
        }
    }

    Ok(RunSpec { prompt: blocks.join("\n\n"), tools })
}
```

Kết quả: `compose()` trả ra prompt text + `0..N` tool xuất hiện trong context của run đó — yêu cầu bộ nhớ theo block, không nạp toàn kho tool. `chatplan` đã bỏ: nó trùng `compose` nhưng không trả tools (dễ chạy thiếu schema) và dùng phân cách khác.

## Tool loop & just-in-time retrieval

Prompt không phải one-shot: mỗi request của user mở một **vòng lặp**, model chỉ *xin* gọi tool, harness chạy rồi append kết quả vào history và gọi model lại. Model không bao giờ tự chạy `search_genre` (tên ví dụ, chưa chốt) — nó phát `tool_call`, harness thực thi, rồi lượt sau model đọc observation và quyết định tiếp (ReAct loop: reason → act → observe).

```rust
struct TurnState {
    has_applied_write: bool,
    consecutive_rejected_count: usize,
}

/// Hàm trong: chỉ chạy vòng lặp và trả kết quả lượt; không tự niêm phong.
async fn run_turn_loop(
    ctx: &TurnContext,
    composed: &RunSpec,
    state: &mut TurnState,
) -> Result<String, PromptError> {
    let mut messages = vec![Message::system(composed.prompt.clone())];
    // Pipeline service bơm State, event log, vài lượt chat cuối vào messages trước request.
    messages.push(Message::user(ctx.goal.clone()));
    let rejected_limit = ctx.runtime_config.consecutive_rejected_limit; // ví dụ N = 3; độc lập MAX_ITER

    for _ in 0..MAX_ITER {
        let resp = llm.chat(&messages, &composed.tools).await?;
        messages.push(resp.message.clone());

        if resp.tool_calls.is_empty() {
            // Cuối lượt: hàm ngoài niêm phong version nếu has_applied_write = true.
            return Ok(resp.text);
        }

        let mut remaining_calls = resp.tool_calls.into_iter();
        while let Some(call) = remaining_calls.next() {
            // Lỗi nghiệp vụ trả Ok(json!({ "status": "rejected", ... })) → observation.
            // Err chỉ dành cho lỗi hạ tầng (DB down, timeout) → bubble ra ngoài.
            // Nhánh ? của registry.execute: không lưu phản hồi có tool_call thiếu tool_result vào lịch sử nạp lại (core 3.8).
            // Phân loại tool ghi Draft State lấy từ metadata của Tool Registry, không suy từ tiền tố tên.
            // `add_scene_version` là tool Writing Mode, không ghi Draft State và không được làm has_applied_write = true.
            let is_write = is_draft_write(registry, &call);
            let out = registry.execute(&call, ctx).await?;
            let status = out.get("status").and_then(|v| v.as_str());
            // Tool ghi nhiều trường: có ít nhất một results[i].status = applied.
            // Tool ghi một trường, tool xóa hoặc tool liên kết: status tổng = applied.
            let applied = out.get("results")
                .and_then(|v| v.as_array())
                .map(|rows| rows.iter().any(|r| r.get("status").and_then(|v| v.as_str()) == Some("applied")))
                .unwrap_or_else(|| status == Some("applied"));
            if is_write && applied { state.has_applied_write = true; }
            if status == Some("rejected") { state.consecutive_rejected_count += 1; }
            else { state.consecutive_rejected_count = 0; } // status tổng khác rejected
            messages.push(Message::tool_result(call.id, out));

            if state.consecutive_rejected_count > rejected_limit {
                // Phản hồi này còn tool_call chưa chạy: trả đủ tool_result trước khi dừng.
                for skipped in remaining_calls {
                    messages.push(Message::tool_result(skipped.id, json!({
                        "status": "rejected", "error": "turn_aborted"
                    })));
                }
                // Giữ State; hàm ngoài niêm phong version nếu has_applied_write.
                // Lượt cuối không cấp tool; model mời người dùng nhắc lại theo {{system_language}}.
                let final_resp = llm.chat(&messages, &[]).await?;
                return Ok(final_resp.text);
            }
        }
    }

    Err(PromptError::MaxIterations)
}

/// Hàm ngoài: mọi đường thoát của run_turn_loop, kể cả lỗi hạ tầng và MAX_ITER,
/// đều đi qua bước niêm phong khi has_applied_write = true.
pub async fn run_ai_turn(
    ctx: &TurnContext,
    composed: &RunSpec,
) -> Result<String, PromptError> {
    // Mỗi draft có một working State và một khóa ghi; lượt đến sau chờ tối đa
    // DRAFT_LOCK_TIMEOUT (mặc định 30 giây), quá hạn trả lỗi "draft đang bận".
    let _draft_guard = ctx.draft_lock
        .acquire(&ctx.draft_session_id, ctx.runtime_config.draft_lock_timeout)
        .await?;
    let mut state = TurnState {
        has_applied_write: false,
        consecutive_rejected_count: 0,
    };
    let result = run_turn_loop(ctx, composed, &mut state).await;

    if state.has_applied_write {
        // seal_turn_version ghi turn_id và chat_message_id = tin nhắn user khởi phát lượt.
        // Idempotent theo (draft_session_id, turn_id): retry trả version hiện có, không tạo snapshot thứ hai.
        // Workflow round có đơn vị niêm phong riêng; không dùng chung turn_id cho nhiều vòng.
        if seal_turn_version(ctx).await.is_err() {
            // Lỗi DB khi niêm phong: thử lại đúng một lần rồi báo người dùng.
            if let Err(err) = seal_turn_version(ctx).await {
                notify_seal_failed(ctx, err);
                // Pseudocode: VersionSealFailed là variant minh họa, chưa có kiểu Rust đã triển khai.
                // Không trả Ok của lượt chat khi State đã đổi nhưng version chưa được niêm phong.
                return Err(PromptError::VersionSealFailed);
            }
        }
    }

    result
}
```

Tool impl ký kiểu `async fn execute(&self, call: &ToolCall, ctx: &ToolContext) -> Result<serde_json::Value, ToolInfraError>`. Khung kết quả nghiệp vụ thống nhất với mục 1.4 của `docs/business/core.md`: tool ghi nhiều trường trả `{ "status": <tổng>, "results": [{ "field": "...", "status": "applied" | "pending_confirmation" | "rejected", "patch_id"?: "...", "error"?: "..." }] }`; `status` tổng là `applied` nếu có ít nhất một trường `applied`, nếu không là `pending_confirmation` nếu có ít nhất một trường `pending_confirmation`, nếu không là `rejected`. Tool ghi một trường, tool xóa và tool liên kết trả `{ "status": "applied" | "pending_confirmation" | "rejected", ... }` và có thể bỏ `results`. Tool đọc thành công dùng `applied` nhưng không tạo version: chỉ tool có metadata `writes_draft_state = true` và có ít nhất một kết quả `applied` mới góp vào version, không suy từ `status` đơn lẻ và không suy từ tiền tố tên. Ghi đè dữ liệu được bảo vệ dùng `pending_confirmation` kèm `patch_id`; lỗi nghiệp vụ/validate trả `Ok(json!({ "status": "rejected", ... }))` để AI nhận observation và tự sửa khi phù hợp, không trả `Err`. `Err` chỉ dành cho lỗi hạ tầng như DB down hoặc timeout.

### Phân loại tool ghi Draft State

Tool Registry giữ metadata `writes_draft_state` cho từng tool; pipeline dùng metadata này để quyết định `has_applied_write`, không dùng tiền tố `add_`, `set_`, `delete_`, cũng không dùng tên tool làm quy tắc. Các thao tác Writing Mode (`write_scene_content`, `lock_scene`, `unlock_scene`, `add_scene_version`) có metadata này là `false` và **không được cấp cho AI** (theo `core.md` §1.6 `SL-01..12`, `LOCK-01..07`, `SCOPE-01..06`: chỉ người dùng mới được thao tác Lock/Unlock — được persist tức thời xuống SQLite qua IPC độc quyền của người dùng theo `LOCK-01`, không chờ `Save Chapter` — trong khi nội dung scene được persist nguyên tử qua `Save Chapter` theo `LOCK-05`); tool đọc cũng `false`. Các tool Planning ghi State nháp của thực thể project/arc/chapter/scene/character/world/lore/relationship có metadata `true`. Việc đăng ký metadata là một phần của Tool Registry; không thêm cột DBML cho metadata này.

### Ranh giới Planning/Writing

* Planning Pipeline chạy đến bước commit plan; không tự động sinh nội dung chương. Việc lập kế hoạch 500 chương không phải yêu cầu viết 500 chương.
* Writing Pipeline chỉ chạy sau khi plan đã commit; một phiên chỉ tập trung một chương, không tự chuyển chương và không chạy hàng loạt.
* Trong Writing Mode (`core.md` §1.6 v2.4.4):
  - **Lock/Unlock tức thời (`LOCK-01..04`):** Chỉ người dùng mới được Lock/Unlock scene; thao tác này persist `scenes.is_locked` tức thời xuống SQLite qua IPC chuyên biệt (không qua `Save Chapter`, không xóa dirty content/buffer, và không tạo `scene_versions`).
  - **Generation & Content Save (`SCOPE-01..06`, `LOCK-05`, `Q4.1..Q4.9`):** AI chỉ được sinh nội dung khi `scene.is_locked == false` **và** `scene.id ∈ active_request.EditScope`; kết quả sinh đi vào buffer tạm (`started -> delta* -> completed | cancelled | failed`). Nếu bị Lock giữa chừng khi đang stream, generation đó bị vô hiệu hóa vĩnh viễn (giữ text tạm để xem nhưng cấm Accept kể cả sau khi Unlock). Khi AI request kết thúc, `EditScope` hết hạn đối với các lần sinh mới nhưng nội dung đã hoàn tất hợp lệ vẫn giữ nguyên hiệu lực để người dùng Review/Accept/Save mà không cần AI request đang chạy (`SCOPE-02..05`), miễn là scene đang mở khóa tại thời điểm Accept/Save. AI không có tool đổi `is_locked` hay ghi trực tiếp vào `scenes.content` / `scene_versions`.
* Hai pipeline có prompt block, context injection, tập tool và quy tắc versioning riêng. Không tái sử dụng workflow Phase 3-6 để viết nội dung.
* Toàn bộ quy tắc nghiệp vụ của Writing Mode (`Q4.1..Q4.9`) đã được chốt tại `core.md` §1.6 và mục Q4 (v2.4.4).

Khi nạp lại vài lượt chat cuối, mỗi `tool_call` phải đi kèm `tool_result` tương ứng. Nếu vượt ngưỡng giữa một phản hồi có nhiều call, các call còn lại nhận kết quả `rejected` với `error = turn_aborted` trước khi gửi lịch sử vào lượt model cuối không cấp tool. Lượt cuối tạo lời mời nhắc lại theo `{{system_language}}` do pipeline bơm từ cài đặt giao diện, không hardcode tiếng Việt và không dùng ngôn ngữ sáng tác `project.language` cho lời mời.

### Enum lớn thì query tại chỗ, không preload

JIT retrieval danh mục chỉ áp dụng cho dữ liệu tĩnh/phẳng như genres, motifs, tropes: không nhét toàn bộ danh mục vào prompt vì đó là data, không phải instruction. Prompt chỉ cần nhắc tên tool đọc khi cần; tool trả dữ liệu rút gọn như `code` + `display_name`, có phân trang. Cây Arc, chương, cảnh không có tool `search_*`: AI phân giải tham chiếu từ State đã có trong context theo mục 3.8 "Phân giải tham chiếu thực thể" của `docs/business/core.md`. Ngoại lệ giới hạn: khi mục lục trong context bị cắt (cờ `outline_truncated = true`) và tên hoặc vị trí người dùng nhắc không khớp, AI gọi tool đọc `lookup_outline` (tên ví dụ, chưa chốt; xem mục 3.8 của `docs/business/core.md` và `docs/business/harness/tools.md`). Tool này không tạo version. Khi kết quả tra cứu chưa đầy đủ, AI không được kết luận thực thể không tồn tại hoặc tự tạo thực thể thay thế.

```text
user: "tạo project fantasy"
  → assistant: search_genre { query: "fantasy", limit: 10 }     [call_1]
  → tool: { "status": "applied", "candidates": [{ "code": "fantasy", "display_name": "Fantasy" }] }
  → assistant: add_project_genre { genre_code: "fantasy", is_primary: true, declared_source: "user" } [call_2]
  → tool: { "status": "applied" }
  → assistant: "Đã gắn thể loại fantasy làm thể loại chính của project"
```

Ví dụ minh họa bước gắn thể loại cho project: `search_genre` trả `code = "fantasy"`, rồi `add_project_genre` dùng đúng giá trị đó làm `genre_code`, đặt `is_primary = true` và khai báo `declared_source = "user"` để handler gán `source`. `add_project_genre` là tool liên kết một payload nguyên tử nên giữ `declared_source` scalar; tool ghi nhiều trường như `set_character` dùng `declared_sources` theo từng trường (core 1.3). Bước tạo project không được minh họa ở đây. Tên tool trong ví dụ chưa chốt; contract thực tế do `tools.input_schema` trong `docs/db.dbml` cấp qua function calling. Tên minh họa theo quy ước `<động từ>_<thực thể>` ở mục 1.4.5 của `docs/business/core.md`.

Quy ước:

- Enum nhỏ, ổn định (< ~20: `gender`, `status`) → nhét thẳng vào input schema là được.
- Danh mục tĩnh/phẳng lớn (genres, motifs, tropes) → tra qua tool đọc `search_*`/`list_*` theo thực thể; không dùng JIT retrieval cho Arc/chương/cảnh thuộc State nháp. Ngoại lệ duy nhất là `lookup_outline` khi mục lục bị cắt.
- Tool ghi nhận `code` đã chuẩn hóa; nếu AI gửi code lạ của danh mục tĩnh, tool trả `Ok` với `status = rejected`, kèm `error`, `input` và `candidates` để AI sửa ở lượt sau (không trả `Err`):

```json
{ "status": "rejected", "error": "unknown_genre", "input": "isekai", "candidates": ["portal_fantasy", "other_world"] }
```

### Guardrails trong harness

- `MAX_ITER` 20–30 là giới hạn tổng số vòng lặp; độc lập với ngưỡng `consecutive_rejected_count`.
- Backend giữ `consecutive_rejected_count` in-memory trong lượt, chỉ tăng khi `status` tổng của tool call là `rejected` và về 0 trong các trường hợp còn lại, kể cả call có ít nhất một trường `applied` hoặc `pending_confirmation`. Ngưỡng runtime cho phép tối đa N call `rejected` liên tiếp (ví dụ N = 3); call thứ N+1 thì dừng lượt, giữ State đã có và niêm phong phần tool ghi được `applied` trước đó vào version cuối lượt. Lời mời nhắc lại được model sinh ở lượt cuối không cấp tool theo `{{system_language}}`, không lộ lỗi kỹ thuật.
- Cap output mỗi tool (~10–20 dòng / 1–2k token), có cursor để phân trang.
- Truncate/compact history khi gần context limit.
- Cache kết quả `search_genre` (tên ví dụ, chưa chốt) theo session nếu danh mục ít đổi.
- Viết tool description kỹ — model dựa vào đó để chọn tool, quan trọng ngang prompt.

Các tool danh mục tĩnh và tool ghi minh họa ở trên chỉ là row trong `tools`, gắn vào block tương ứng qua `prompt_block_tools`; không cần bảng riêng cho từng tool. Tên tool trong ví dụ chưa chốt.

Tham khảo: [ReAct loop — ramonchancay](https://www.ramonchancay.me/blog/react-loop-agent-tools) · [Context engineering — Mastra](https://mastra.ai/articles/context-engineering)
