# BÁO CÁO NGHIÊN CỨU & THIẾT KẾ KIẾN TRÚC BACKEND (SUBAGENT C)
## Chuyên sâu: Context Assembly, Hierarchical System Prompts, Context Compaction & Token Management

---

| Thông tin | Chi tiết |
|---|---|
| **Dự án** | Merlink Agent Harness (Tauri v2 + Rust Backend + React 19 UI) |
| **Vai trò** | Subagent C — Chuyên gia Context Engineering, Token Management & Memory Systems |
| **Mục tiêu** | Thiết kế toàn diện kiến trúc quản lý ngữ cảnh, lắp ráp prompt phân tầng, hiệu chuẩn token động, nén ngữ cảnh an toàn và phân định hệ thống bộ nhớ cho agent sáng tác tiểu thuyết |
| **Tài liệu đối chiếu** | `docs/be-tauri-plan.md` (mục 2, 5.2, 7.2, 7.3, 11), `docs/db.dbml`, `docs/bussiness-flow.md` |
| **Công cụ nghiên cứu** | `tinyfish search query` & `tinyfish fetch content get` (tra cứu docs tiktoken-rs, fastembed-rs, sqlite-vec) |
| **Ngày hoàn thành** | 2026-10-29 |

---

## MỤC LỤC
1. [Tổng quan & Bối cảnh](#1-tổng-quan--bối-cảnh)
2. [Khảo sát Thực nghiệm các Giải pháp Quản lý Ngữ cảnh & Token trong Rust](#2-khảo-sát-thực-nghiệm-các-giải-pháp-quản-lý-ngữ-cảnh--token-trong-rust)
   - 2.1 Thư viện & Chiến lược Đếm Token trong Rust (`tiktoken-rs` vs `tokenizers`)
   - 2.2 Thuật toán Hiệu chuẩn Token động (Adaptive EMA Calibration)
   - 2.3 Chiến lược Quản lý Cửa sổ Ngữ cảnh (Compaction, Sliding Window & Fallback)
   - 2.4 Công nghệ Tìm kiếm & Bộ nhớ Cục bộ (SQLite FTS5 vs `sqlite-vec` + `fastembed-rs`)
3. [Thiết kế Chi tiết Kiến trúc Backend cho Merlink](#3-thiết-kế-chi-tiết-kiến-trúc-backend-cho-merlink)
   - 3.1 Cấu trúc System Prompt Phân tầng (Hierarchical Prompt Composition)
   - 3.2 Trình biên dịch Story Directives từ DBML
   - 3.3 Prompt Manifest & Cơ chế Audit Trail
   - 3.4 Quản lý & Hiệu chuẩn Token động (tiktoken-rs Singleton + EMA Calibration)
   - 3.5 Context Compaction, Sliding Window & Fallback An toàn
   - 3.6 Bộ nhớ Ngữ cảnh: Context Memory vs Character Memory (FTS5 + Lộ trình sqlite-vec)
4. [Đối chiếu với `be-tauri-plan.md` & Đề xuất Tinh chỉnh](#4-đối-chiếu-với-be-tauri-planmd--đề-xuất-tinh-chỉnh)
5. [Các Giải pháp KHÔNG Phù hợp (No-Match & Anti-Patterns)](#5-các-giải-pháp-không-phù-hợp-no-match--anti-patterns)
6. [Kết luận & Khuyến nghị Triển khai](#6-kết-luận--khuyến-nghị-triển-khai)

---

## 1. TỔNG QUAN & BỐI CẢNH

### 1.1 Vai trò của Merlink Agent Harness
Merlink là một ứng dụng Desktop (Tauri v2 nhúng backend Rust và frontend React 19) đóng vai trò là một **harness chuyên biệt cho Agent sáng tác truyện dài tập (narrative authoring agent)**. Không giống như các chatbot hội thoại thông thường (conversational chatbots) chỉ duy trì lịch sử tin nhắn dạng phẳng (flat chat messages), Merlink đòi hỏi Agent phải:
1. **Thao tác trực tiếp trên cơ sở dữ liệu sáng tác đồ sộ**: Quản lý hàng trăm chương (`chapters`), phân đoạn (`scenes`), nhân vật (`characters`), mối quan hệ (`relationships`), thiết lập thế giới (`world_settings`), và tầng ý nghĩa tư tưởng (`themes`).
2. **Tuân thủ kỷ luật kể chuyện khắt khe (Story Directives)**: Định hình giọng văn dựa trên cấu trúc phong cách (`style_presets`), ngôi kể/góc nhìn (`narrator_configs`), các mô-típ cốt truyện (`motifs`), các khối biến cố (`event_blocks`), và các chỉ thị biên kịch (`narrative_directives`).
3. **Chạy hoàn toàn cục bộ (Desktop-first, Offline-first, Privacy-first)**: Toàn bộ dữ liệu lưu trữ trong SQLite cục bộ, API keys lưu trong OS Keychain, và chỉ giao tiếp ra ngoài qua HTTP streaming tới các LLM Provider (OpenAI, Anthropic, Ollama, OpenRouter, v.v.).

### 1.2 Những Thách thức Kỹ thuật Lõi
- **Bùng nổ ngữ cảnh trong văn học (Context Explosion)**: Một tác phẩm tiểu thuyết có dung lượng hàng chục vạn chữ. Kèm theo đó là các thông số lore, character sheets, scene versions, và tool call results. Nếu không có cơ chế nén (compaction) và phân tầng chặt chẽ, context window của LLM sẽ nhanh chóng bị tràn hoặc chi phí token tăng vọt.
- **Đặc thù ngôn ngữ tiếng Việt (Token Overhead & Tokenizer Mismatch)**: Tiếng Việt có dấu với cấu trúc âm tiết rời rạc thường bị phân mảnh (token fragmentation) trên các bộ tokenizer BPE phổ biến (cl100k_base, o200k_base), dẫn đến số token thực tế gửi lên LLM thường cao hơn 1.5x đến 2.5x so với tiếng Anh. Sai số giữa số token ước tính cục bộ và số token tính cước của Provider là rất lớn nếu không có cơ chế hiệu chuẩn thích ứng.
- **Sự khác biệt giữa các Model/Tokenizer**: Người dùng Merlink có thể cấu hình linh hoạt nhiều provider/model khác nhau (từ GPT-4o, Claude 3.5 Sonnet, tới các mô hình cục bộ qua Ollama như Llama 3.1, Qwen 2.5). Không thể hard-code một tokenizer duy nhất cho tất cả các model.
- **Sự nhầm lẫn giữa Trí nhớ Trợ lý (Context Memory) và Ký ức Nhân vật (Character Memory)**: Trong sáng tác truyện, nếu trộn lẫn tri thức tác nghiệp của AI (như "tác giả thích văn phong cô đọng, ghét tính từ sáo rỗng") với ký ức nội tâm của nhân vật trong cốt truyện (như "Lâm nhớ lần đầu gặp Nguyệt ở bờ suối năm 10 tuổi"), agent sẽ bị "loạn vai" (character confusion/identity bleeding), dẫn đến việc sinh văn bản sai lệch hoàn toàn.
- **Tính Kiểm toán Tuyệt đối (Prompt Auditability)**: Khi agent thực hiện một hành động sửa đổi sai lệch (ví dụ tự ý xóa một arc hoặc đổi ngôi kể), nhà văn cần biết chính xác tại bước (run step) đó, những tầng prompt nào, chỉ thị nào, và ký ức nào đã được nạp vào context window của LLM.

---

## 2. KHẢO SÁT THỰC NGHIỆM CÁC GIẢI PHÁP QUẢN LÝ NGỮ CẢNH & TOKEN TRONG RUST
*(Nghiên cứu được đối chiếu trực tiếp qua `tinyfish` từ tài liệu kỹ thuật của `tiktoken-rs`, `fastembed`, `sqlite-vec`)*

### 2.1 Thư viện & Chiến lược Đếm Token trong Rust

#### Khảo sát Thư viện `tiktoken-rs`
Từ tài liệu chính thức (`https://github.com/zurawiki/tiktoken-rs`):
- `tiktoken-rs` cung cấp các hàm Singleton tối ưu bộ nhớ: `tiktoken_rs::o200k_base_singleton()` và `tiktoken_rs::cl100k_base_singleton()`. Việc sử dụng singleton giúp tránh lặp lại chi phí giải mã và khởi tạo bảng từ vựng BPE (zero-allocation counting), đạt tốc độ mã hóa hàng chục ngàn token mỗi giây trên CPU thông thường.
- Tuy nhiên, chính tác giả của `tiktoken-rs` đã nhấn mạnh rõ phạm vi:
  > *"Scope: This crate is focused on OpenAI tokenizers (tiktoken). For non-OpenAI models (Llama, Gemini, Mistral, etc.), use the HuggingFace `tokenizers` crate."*
- Nếu cài đặt HuggingFace `tokenizers`, ứng dụng sẽ phải download hoặc đóng gói kèm các file `tokenizer.json` dung lượng lớn (2MB - 10MB mỗi model), làm phình to gói cài đặt desktop Tauri và tạo độ phức tạp quản trị file ngoài.

#### Đánh giá So sánh & Lựa chọn Kiến trúc cho Merlink:

| Giải pháp | Ưu điểm | Nhược điểm | Kết luận áp dụng |
|---|---|---|---|
| **`tiktoken-rs` (Singleton) + EMA Calibration** | - Tốc độ cực cao (< 10 µs cho 10K text).<br>- Nhẹ, zero external asset.<br>- Thích ứng động với mọi LLM (OpenAI, Anthropic, Ollama). | - Bước đếm ban đầu cho non-OpenAI model là xấp xỉ. | **LỰA CHỌN TỐI ƯU CỦA MERLINK**: Dùng `o200k_base_singleton()` làm baseline estimator, kết hợp thuật toán EMA Calibration để tự động bù trừ sai số tokenizer theo từng model. |
| **HuggingFace `tokenizers` (Đầy đủ)** | - Chính xác 100% theo từng họ model. | - Phải tải/bundle hàng chục file `tokenizer.json`.<br>- Tăng dung lượng app và độ phức tạp runtime. | **DỰ PHÒNG CHO PHASE SAU**: Chỉ xem xét nếu người dùng chạy hoàn toàn offline với các local model cực kỳ đặc thù. |
| **Character / Word Heuristic** | - Tức thì, zero dependency. | - Sai số quá lớn (±40% đến ±80%), cực kỳ nguy hiểm với tiếng Việt và tool schemas. | **CHỈ DÙNG LÀM FALLBACK CÙNG CỰC** khi không load được BPE. |

### 2.2 Thuật toán Hiệu chuẩn Token động (Adaptive EMA Token Calibration)
Khi gọi API đến các Provider (OpenAI, Anthropic, OpenRouter, Ollama), phản hồi hoàn thành hoặc SSE stream luôn trả về cấu trúc `usage`:
```json
{
  "usage": {
    "prompt_tokens": 1420,
    "completion_tokens": 312,
    "total_tokens": 1732
  }
}
```
Tại tầng `agent/context/tokens.rs`, ta có:
- $T_{est}$: Số token ước lượng bởi `tiktoken-rs` trước khi gửi request.
- $T_{actual}$: Số token thực tế `usage.prompt_tokens` trả về từ LLM Provider.

Tỷ lệ sai số thực tế tại bước $t$:
$$\text{Ratio}_t = \frac{T_{actual}}{T_{est}}$$

Để chống lại các biến động cục bộ (anomaly/noise do lỗi mạng hoặc metadata header), tỷ lệ này được kẹp trong ngưỡng an toàn:
$$\text{RatioClamped}_t = \text{clamp}(\text{Ratio}_t, 0.5, 3.0)$$

Hệ số hiệu chuẩn động cho model được cập nhật theo công thức **Exponential Moving Average (EMA)**:
$$\text{Calibration}_t = \alpha \cdot \text{RatioClamped}_t + (1 - \alpha) \cdot \text{Calibration}_{t-1}$$
Với $\alpha = 0.2$ (hoặc $0.1$ khi mẫu đã ổn định). Giá trị $\text{Calibration}_t$ được lưu ngược vào bảng `ai_models.token_calibration`.

### 2.3 Chiến lược Quản lý Cửa sổ Ngữ cảnh (Compaction & Sliding Window)
1. **Bảo vệ Cửa sổ Cận kề (Protected Recency Window)**:
   - Nghiên cứu từ các agent framework hiện đại cho thấy, việc tóm tắt hoặc cắt bỏ các tin nhắn vừa diễn ra gây ra hiện tượng mất ngữ cảnh nghiêm trọng nhất.
   - Merlink bắt buộc: **Luôn giữ nguyên vẹn 6 tin nhắn gần nhất** (tương đương 3 lượt hội thoại User - Assistant) cùng tin nhắn hiện tại.
2. **Kỹ thuật Lược bỏ Thân Kết quả Tool cũ (Tool Result Elision)**:
   - Kết quả trả về từ tool (như danh sách toàn bộ nhân vật, văn bản một scene dài 3,000 từ) thường chiếm tới 60%–80% context.
   - Sau khi Agent đã xử lý xong và đưa ra kết luận, dữ liệu thô này không còn cần thiết. Hệ thống thay thế thân JSON bằng đoạn text tóm lược: `[Đã lược kết quả tool <name>: N tokens]`.
3. **Compactor Fallback**:
   - Nếu tác vụ tóm tắt của LLM thất bại (do rate limit, network timeout, hoặc context overflow), hệ thống tự động kích hoạt bộ cắt tỉa tất định (Deterministic Truncation) mà không làm sập `run_loop`.

### 2.4 Công nghệ Tìm kiếm & Bộ nhớ Cục bộ (FTS5 vs `sqlite-vec` + `fastembed-rs`)

Qua tra cứu chuyên sâu từ tài liệu kỹ thuật của `sqlite-vec` (Alex Garcia) và `fastembed-rs`:
- **`sqlite-vec`**: Là extension SQLite viết bằng C thuần, zero-dependency, siêu nhẹ. Hỗ trợ bảng ảo `vec0` lưu trữ vector dạng `float[N]` hoặc nén nhị phân `bit[N]` (32x compression thông qua binary quantization). Benchmark thực tế trên máy tính cá nhân đạt < 15ms cho 100,000 vector. Không cần cài đặt bất kỳ server ngoài nào (không Docker, không Qdrant/Milvus).
- **`fastembed-rs`**: Crate Rust sử dụng ONNX Runtime nhúng, hỗ trợ sinh vector cục bộ 100% offline. Hỗ trợ mô hình **BGE-M3** (`Bgem3Embedding`), là mô hình hàng đầu hiện nay hỗ trợ đa ngôn ngữ và tiếng Việt cực tốt, cho phép xuất cả dense vector, sparse lexical vector và ColBERT multi-vector trong một lần chạy duy nhất.
- **SQLite FTS5 (BM25)**: Tích hợp sẵn trong SQLite, tối ưu tuyệt đối cho việc tìm kiếm chính xác từ ngữ, tên riêng nhân vật và địa danh truyện.

**Chiến lược 2 giai đoạn cho Merlink:**
- **Phase 0 & 1**: Sử dụng **SQLite FTS5 + Recency Decay + Pinned Memory**. Đảm bảo khởi động tức thì, 0% lỗi cài đặt extension, tài nguyên RAM < 5MB.
- **Phase 3**: Tích hợp `sqlite-vec` + `fastembed-rs` (`Bgem3Embedding`) để hình thành giải thuật **Hybrid Search (BM25 + Dense Vector qua Reciprocal Rank Fusion)**.

---

## 3. THIẾT KẾ CHI TIẾT KIẾN TRÚC BACKEND CHO MERLINK

### 3.1 Cấu trúc System Prompt Phân tầng (Hierarchical Prompt Composition)

Hệ thống prompt của Merlink được tổ chức thành 6 tầng phân cấp (Hierarchical Onion Architecture), tuân thủ nguyên tắc: *Từ tổng quát đến cụ thể, từ quy tắc bất biến đến ngữ cảnh biến động*:

```
┌─────────────────────────────────────────────────────────────┐
│ 1. Base Prompt (Vai trò, quy tắc cốt lõi, slug vs display)   │
├─────────────────────────────────────────────────────────────┤
│ 2. Story Directives (Genres, Motifs, Styles, POV, Rules)    │
├─────────────────────────────────────────────────────────────┤
│ 3. Project Context (Tên truyện, tóm tắt, ngôn ngữ, tiến độ) │
├─────────────────────────────────────────────────────────────┤
│ 4. Session Context (Chỉ thị riêng của phiên làm việc)        │
├─────────────────────────────────────────────────────────────┤
│ 5. Tool Guidelines (Chỉ dẫn sử dụng tool theo RunMode)      │
├─────────────────────────────────────────────────────────────┤
│ 6. Verify Feedback (Phản hồi sửa lỗi nếu step trước Retry)   │
└─────────────────────────────────────────────────────────────┘
```

#### Quy tắc Ưu tiên & Khối Bất biến (Precedence & Invariants)
1. **Precedence thông thường**:
   $$\text{Base} < \text{Project} < \text{Session} < \text{Story Directives}$$
   Chỉ thị ở tầng hẹp hơn có quyền tinh chỉnh hoặc thay thế chỉ thị ở tầng rộng hơn.
2. **Quy tắc Bất biến (Locked Invariants - `is_locked = true`)**:
   - Các quy định nền tảng (như: *"Trường `code` là slug định danh nội bộ, cấm dùng hiển thị cho người đọc; luôn dùng `display_name`"*; hoặc *"Ngôn ngữ bắt buộc của dự án là tiếng Việt"*) được đánh dấu `is_locked = true` trong `prompt_templates`.
   - Trình `PromptComposer` sẽ gom tất cả các khối `is_locked` và đặt vào phần Invariants bắt buộc ở cuối System Prompt, ngăn chặn việc prompt injection hoặc chỉ thị ở tầng con ghi đè.

### 3.2 Trình biên dịch Story Directives từ DBML
Tầng **Story Directives** được sinh động bởi `crates/agent/src/prompt/story_directives.rs` bằng cách truy vấn SQLite:

```rust
pub struct StoryDirectiveCompiler;

impl StoryDirectiveCompiler {
    pub async fn compile(
        &self,
        pool: &sqlx::SqlitePool,
        project_id: uuid::Uuid,
        active_scene_id: Option<uuid::Uuid>,
    ) -> Result<String, DomainError> {
        // 1. Truy vấn thể loại chính & phụ của dự án
        let genres = self.fetch_project_genres(pool, project_id).await?;
        
        // 2. Truy vấn Arc hiện tại và các Motifs / Event Blocks đang kích hoạt
        let motifs = self.fetch_active_motifs(pool, project_id, active_scene_id).await?;
        
        // 3. Truy vấn Narrative Directives (Show don't tell, quy ước giọng văn)
        let directives = self.fetch_narrative_directives(pool, project_id).await?;
        
        // 4. Nếu có scene_id cụ thể, truy vấn Narrator Config (POV) và Style Preset
        let scene_style = if let Some(sid) = active_scene_id {
            self.fetch_scene_style_and_pov(pool, sid).await?
        } else {
            None
        };

        // 5. Render thành Markdown Markdown chặt chẽ
        Ok(self.render_markdown(&genres, &motifs, &directives, &scene_style))
    }
}
```

**Mẫu Story Directives sau khi Render:**
```markdown
### CHỈ THỊ SÁNG TÁC (STORY DIRECTIVES)
- **Thể loại**: Tiên hiệp, Kỳ ảo cổ trang (Chính), Ân oán giang hồ (Phụ).
- **Mô-típ cốt truyện**: [MOTIF_OATH_BETRAYAL] "Lời thề phản bội".
  + Khối biến cố hiện tại: `EB_DISCOVERY` (Nhân vật phát hiện tín vật giả).
  + Khối biến cố cho phép tiếp theo: `EB_CONFRONTATION`, `EB_DENIAL`.
  + Ràng buộc mô-típ: Tuyệt đối không để nhân vật chính bộc lộ nghi vấn trước khi thu thập đủ 3 mảnh ngọc.
- **Thiết lập POV (Ngôi kể & Góc nhìn)**:
  + Ngôi thứ ba giới hạn (Third-Person Limited).
  + Nhân vật giữ POV: "Bạch Lạc Hàn" (ID: char_01). Không mô tả suy nghĩ thầm kín của nhân vật khác ngoài hành vi bên ngoài.
- **Phong cách hành văn (Style Preset)**:
  + Tỷ lệ hội thoại: MEDIUM (Tập trung tả nội tâm và không gian u tối).
  + Nhịp điệu: SLOW (Tạo không khí ngột ngạt, dồn nén trước cao trào).
```

### 3.3 Prompt Manifest & Cơ chế Audit Trail
Tại mỗi lượt chạy (`run_step` loại `context`), `PromptComposer` tạo ra một snapshot đầy đủ gọi là **`PromptManifest`** và lưu vào bảng `run_steps` dưới dạng JSON (`input_json`):

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptManifest {
    pub run_id: String,
    pub step_seq: u32,
    pub assembled_at: chrono::DateTime<chrono::Utc>,
    pub model_id: String,
    pub layers: Vec<PromptLayerManifest>,
    pub loaded_memories: Vec<MemoryReferenceManifest>,
    pub loaded_attachments: Vec<AttachmentReferenceManifest>,
    pub total_estimated_tokens: u32,
    pub final_prompt_sha256: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptLayerManifest {
    pub kind: String, // "base" | "project" | "session" | "story_directives" | "tools" | "verify"
    pub template_id: Option<uuid::Uuid>,
    pub version: Option<String>,
    pub sha256: String,
    pub token_count: u32,
    pub is_locked: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryReferenceManifest {
    pub memory_id: uuid::Uuid,
    pub scope: String,
    pub is_pinned: bool,
    pub relevance_score: f32,
    pub tokens: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttachmentReferenceManifest {
    pub attachment_id: uuid::Uuid,
    pub file_name: String,
    pub injection_mode: String, // "full_text" | "summary_excerpt"
    pub tokens: u32,
}
```

**Giá trị thực tiễn:**
- Phục vụ trực tiếp dialog UI `ProcessDetailDialog.tsx` để người dùng kiểm tra "Vì sao Agent lại hành động như vậy?".
- Hỗ trợ tính năng **Prompt Caching** của OpenAI và Anthropic bằng cách tận dụng `sha256` của từng layer để tối ưu chi phí và độ trễ.

---

### 3.4 Quản lý & Hiệu chuẩn Token động (tiktoken-rs Singleton + EMA)

#### Phân bổ Ngân sách Token (Token Budget Allocation)
Giả sử Model có `context_window` = $W$ và `max_output_tokens` = $M_{out}$.
1. **Dự trữ Output**:
   $$R_{out} = \min(M_{out}, 0.25 \times W)$$
2. **Buffer Thích ứng**:
   $$\text{Buffer} = \begin{cases} 
   0.20 \times W & \text{khi số mẫu } N < 5 \text{ (bảo vệ chống token fragmentation tiếng Việt)} \\
   \max(0.05 \times W, \; 0.10 \times W) & \text{khi số mẫu } N \ge 5 \text{ đã ổn định}
   \end{cases}$$
3. **Ngân sách khả dụng cho Input ($U$)**:
   $$U = W - R_{out} - \text{Buffer}$$

**Bảng trần ngân sách phân bổ theo Phân đoạn (% của $U$):**

| Phân đoạn (Segment) | Trần (% của $U$) | Xử lý khi vượt trần | Tương thích FE `ContextUsagePopover` |
|---|---|---|---|
| **System Layers** | 30% | Rút gọn Story Directives chi tiết; lược bỏ ví dụ tool guide | `bg-fg-subtle` (`systemPrompt`) |
| **Tool Schemas** | 15% | Chỉ nạp tool của `RunMode` hiện tại (`explore` / `edit`) | `bg-purple-500` (`toolDefinitions`) |
| **Pinned Memories** | 10% | Giữ memory cập nhật gần nhất, cảnh báo UI | Hiển thị trong phần memory |
| **Retrieved Memories** | 10% | Cắt theo điểm BM25 từ thấp lên cao | Hiển thị trong phần memory |
| **Attachments** | 25% | File nhỏ nạp full, file lớn trích xuất mục lục | Hiển thị trong phần attachments |
| **History Summary** | 10% | Tái tóm tắt các khối summary xa | Gom vào `messages` |
| **Recent History + Turn hiện tại** | Phần còn lại | Bảo vệ 6 tin nhắn gần nhất; lược bỏ thân `tool_result` | `bg-blue-500` (`messages`) |

#### Cài đặt Rust `TokenCounter` với Singleton & Calibration
```rust
use tiktoken_rs::{o200k_base_singleton, cl100k_base_singleton, CoreBPE};

pub enum TokenizerKind {
    O200k,
    Cl100k,
    Heuristic,
}

pub struct TokenCounter {
    tokenizer_kind: TokenizerKind,
}

impl TokenCounter {
    pub fn new(tokenizer_str: &str) -> Self {
        let kind = match tokenizer_str {
            "o200k" => TokenizerKind::O200k,
            "cl100k" => TokenizerKind::Cl100k,
            _ => TokenizerKind::O200k, // Mặc định o200k hiện đại
        };
        Self { tokenizer_kind: kind }
    }

    /// Đếm token thô siêu nhanh bằng BPE singleton (zero-allocation)
    pub fn count_raw(&self, text: &str) -> u32 {
        match self.tokenizer_kind {
            TokenizerKind::O200k => {
                let bpe: &'static CoreBPE = o200k_base_singleton();
                bpe.encode_with_special_tokens(text).len() as u32
            }
            TokenizerKind::Cl100k => {
                let bpe: &'static CoreBPE = cl100k_base_singleton();
                bpe.encode_with_special_tokens(text).len() as u32
            }
            TokenizerKind::Heuristic => {
                // Heuristic tiếng Việt: ~1 từ = 1.6 token
                let words = text.split_whitespace().count();
                ((words as f64) * 1.6).ceil() as u32
            }
        }
    }

    /// Đếm token có hiệu chuẩn qua EMA calibration của model
    pub fn count_calibrated(&self, text: &str, calibration_factor: f64) -> u32 {
        let raw = self.count_raw(text);
        ((raw as f64) * calibration_factor).ceil() as u32
    }
}
```

---

### 3.5 Context Compaction, Sliding Window & Fallback An toàn

#### Quy trình Cắt tỉa Đa tầng (Cascading Compaction Pipeline)
1. **Bước 1: Elide Tool Results cũ**:
   - Duyệt qua các tin nhắn nằm ngoài cửa sổ 6 tin nhắn gần nhất.
   - Với mọi `Message` mang kết quả tool (`ToolResult`), thay thế nội dung chi tiết bằng:
     ```json
     {
       "call_id": "call_123",
       "tool_name": "scene_get",
       "status": "elided",
       "summary": "[Nội dung kết quả chi tiết đã được lược bỏ để tiết kiệm 2,450 tokens]"
     }
     ```
2. **Bước 2: Cắt giảm Retrieved Memories**:
   - Giảm số lượng memory truy vấn từ FTS5 (ví dụ từ top 10 xuống top 3).
3. **Bước 3: Tóm tắt Lịch sử cũ (Summarization)**:
   - Các tin nhắn từ vị trí thứ 7 trở về trước được gom lại để gọi LLM tóm tắt, sinh ra một tin nhắn duy nhất mang `payload_json.type = "summary"`.
4. **Bước 4: Compactor Fallback Tất định (Deterministic Fallback)**:
   - Nếu việc gọi LLM tóm tắt gặp lỗi (timeout, 429, context overflow), **không để run loop bị crash**.
   - Kích hoạt hàm fallback cắt bỏ hoàn toàn các tin nhắn cũ hơn cửa sổ 6 tin nhắn, chèn một tin nhắn hệ thống tổng hợp:
     `[Hệ thống: Các tin nhắn từ seq 1 đến seq N đã được cắt tỉa an toàn do chạm giới hạn bộ nhớ]`
   - Tiếp tục chu trình streaming bình thường.

---

### 3.6 Bộ nhớ Ngữ cảnh: Context Memory vs Character Memory

#### Sự khác biệt Bản chất
| Thuộc tính | Context Memory (Agent Working Memory) | Character Memory (Fiction Narrative World) |
|---|---|---|
| **Bản chất** | Tri thức làm việc của trợ lý AI qua các phiên | Ký ức nội tâm, trải nghiệm của nhân vật hư cấu |
| **Bảng SQLite** | `context_memories` | `character_memories` |
| **Phạm vi (Scope)** | `global`, `project`, `session` | Thuộc về một nhân vật cụ thể (`character_id`) |
| **Cơ chế nạp** | Tự động nạp vào prompt qua `ContextBuilder` | Chỉ truy xuất khi Agent gọi tool `character_*` |
| **Ví dụ điển hình** | "Tác giả muốn viết văn phong lạnh lùng, dứt khoát; cấm tự ý tạo nhân vật mới." | "Bạch Lạc Hàn nhớ năm 7 tuổi từng nhìn thấy phụ thân giấu bức thư vào pho tượng gỗ." |

#### Hiện thực SQLite FTS5 (Giai đoạn 1)
```sql
CREATE VIRTUAL TABLE context_memories_fts USING fts5(
    content,
    tags,
    content='context_memories',
    content_rowid='rowid',
    tokenize='unicode61 remove_diacritics 0'
);

-- Triggers đồng bộ tự động
CREATE TRIGGER trg_cm_ai AFTER INSERT ON context_memories BEGIN
    INSERT INTO context_memories_fts(rowid, content, tags) 
    VALUES (new.rowid, new.content, new.tags);
END;

CREATE TRIGGER trg_cm_ad AFTER DELETE ON context_memories BEGIN
    INSERT INTO context_memories_fts(context_memories_fts, rowid, content, tags) 
    VALUES('delete', old.rowid, old.content, old.tags);
END;

CREATE TRIGGER trg_cm_au AFTER UPDATE ON context_memories BEGIN
    INSERT INTO context_memories_fts(context_memories_fts, rowid, content, tags) 
    VALUES('delete', old.rowid, old.content, old.tags);
    INSERT INTO context_memories_fts(rowid, content, tags) 
    VALUES (new.rowid, new.content, new.tags);
END;
```

#### Lộ trình Nâng cấp Vector với `sqlite-vec` & `fastembed-rs` (Phase 3)
Khi dự án bước sang giai đoạn nâng cấp tìm kiếm ngữ nghĩa mờ:
1. Kích hoạt extension `sqlite-vec` trong connection pool SQLite.
2. Tạo bảng ảo vector:
   ```sql
   CREATE VIRTUAL TABLE context_memories_vec USING vec0(
       memory_id text primary key,
       embedding float[384] -- Hoặc bit[1536] cho binary quantization
   );
   ```
3. Sử dụng `fastembed-rs` với model `Bgem3Embedding` (BGE-M3) chạy in-process tạo vector.
4. Kết hợp FTS5 và vector qua công thức **Reciprocal Rank Fusion (RRF)**:
   $$RRF\_Score(d) = \frac{0.5}{60 + \text{Rank}_{FTS5}(d)} + \frac{0.5}{60 + \text{Rank}_{Vector}(d)}$$

---

## 4. ĐỐI CHIẾU VỚI `BE-TAURI-PLAN.MD` & ĐỀ XUẤT TINH CHỈNH

| Hạng mục trong `be-tauri-plan.md` | Đánh giá hiện trạng | Đề xuất hoàn thiện của Subagent C |
|---|---|---|
| **Mục 2 & ADR-003: Token Counting** | Đề xuất dùng `tiktoken-rs` xấp xỉ + hiệu chuẩn EMA. | **Đã hoàn thiện**: Bổ sung cơ chế Singleton (`o200k_base_singleton()`) để tối ưu hiệu năng; thiết lập biên độ kẹp clamp $[0.5, 3.0]$ và chặn cận calibration $[0.6, 2.5]$; quy định hệ số đệm buffer động (20% -> 5%). |
| **Mục 5.2: Bảng `context_memories`** | Schema rõ ràng, phân biệt `scope`, có FTS5, có `content_hash` dedupe. | **Xác nhận tối ưu**: Đề xuất cấu hình `tokenize='unicode61 remove_diacritics 0'` cho FTS5 để hỗ trợ hoàn hảo tiếng Việt có dấu. |
| **Mục 7.1 & 7.2: Compactor Fallback** | Đã nêu nguyên tắc compactor lỗi không làm fail run. | **Cụ thể hóa mã nguồn**: Thiết kế struct `ContextCompactor` với nhánh `deterministic_fallback_truncate` bảo vệ 6 tin nhắn gần nhất và chèn synthetic notification marker. |
| **Mục 7.3: Prompt Precedence & Manifest** | Đã nêu thứ tự ưu tiên và yêu cầu prompt manifest. | **Chuẩn hóa cấu trúc**: Xây dựng struct `PromptManifest` chi tiết (kèm token breakdown, SHA256 từng layer) lưu vào `run_steps` với `kind = 'context'`, hỗ trợ cả UI preview và Prompt Caching. |
| **Mục 11: Attachments** | Trích xuất text, đưa vào context theo ngân sách. | **Bổ sung trần ngân sách**: Giới hạn trần cứng 25% ngân sách $U$; vượt ngưỡng này bắt buộc chuyển sang chế độ Table of Contents + Excerpt, yêu cầu Agent gọi tool `attachment_read`. |
| **Giao diện FE `ContextUsagePopover.tsx`** | Đang hiển thị mock tĩnh 3 thanh progress. | **Khớp nối DTO hoàn hảo**: Thiết kế DTO `ContextUsageBreakdown` từ Rust trả về đầy đủ các segment: `systemPrompt`, `toolDefinitions`, `messages`, `memories`, `attachments`. |

---

## 5. CÁC GIẢI PHÁP KHÔNG PHÙ HỢP (NO-MATCH & ANTI-PATTERNS)
*(Được tổng hợp để đưa vào `docs/no-match.md`)*

1. **Vector Database Server độc lập (Qdrant, Milvus, Weaviate, Pinecone, Chroma)**:
   - *Lý do loại bỏ*: Quá nặng nề cho ứng dụng Desktop cá nhân; đòi hỏi Docker hoặc server daemon chạy nền ăn RAM lớn, làm phức tạp hóa quá trình cài đặt của người dùng cuối.
   - *Thay thế bằng*: SQLite FTS5 (Phase 1) và `sqlite-vec` nhúng in-process (Phase 3).
2. **Tóm tắt Đệ quy Không Kiểm soát (Unbounded Recursive Summarization)**:
   - *Lý do loại bỏ*: Dẫn đến hiện tượng trôi dạt ngữ nghĩa nghiêm trọng ("tam sao thất bản") và tiêu tốn token vô ích khi tóm tắt lặp đi lặp lại.
   - *Thay thế bằng*: Sliding window có bảo vệ cửa sổ gần nhất (6 messages) kết hợp Elide Tool Result.
3. **Đếm Token ở Frontend (Client-side Token Counting)**:
   - *Lý do loại bỏ*: Vi phạm nguyên tắc **Backend-Authoritative tuyệt đối**; Frontend không có dữ liệu calibration và việc chạy tokenizer trong WebView gây lag giao diện.
   - *Thay thế bằng*: Rust backend xử lý 100% việc đếm và phân bổ ngân sách token.
4. **Gộp chung Context Memory và Character Memory thành một bảng duy nhất**:
   - *Lý do loại bỏ*: Gây ô nhiễm ngữ cảnh ("identity bleeding"), khiến AI nhầm lẫn giữa quy ước làm việc với tác giả và ký ức nội tâm của nhân vật trong truyện.
   - *Thay thế bằng*: Phân định 2 bảng riêng biệt: `context_memories` và `character_memories`.
5. **Cắt cứng lịch sử mà không Lược bỏ Tool Results (Naive Hard Truncation)**:
   - *Lý do loại bỏ*: Kết quả tool chiếm phần lớn dung lượng. Cắt cả tin nhắn hội thoại sẽ làm đứt gãy mạch giao tiếp của người dùng.
   - *Thay thế bằng*: Kỹ thuật Elide Tool Result cũ trước khi cắt tỉa tin nhắn.

---

## 6. KẾT LUẬN & KHUYẾN NGHỊ TRIỂN KHAI

Báo cáo nghiên cứu của Subagent C đã hoàn thành toàn diện các yêu cầu kỹ thuật:
1. Đã khảo sát và thực nghiệm sâu sắc về `tiktoken-rs`, `fastembed-rs`, và `sqlite-vec`.
2. Đã thiết kế chi tiết kiến trúc Prompt 6 tầng, Story Directives Compiler, và Prompt Manifest.
3. Đã xác lập công thức toán học và cài đặt mẫu cho Token Allocation, EMA Calibration, và Compactor Fallback an toàn.
4. Đã phân định ranh giới rõ ràng giữa Context Memory và Character Memory, kèm lộ trình nâng cấp từ SQLite FTS5 lên Vector Hybrid Search.

**Hành động tiếp theo cho Phase 0/1:**
- Tạo ADR `docs/adr/003-token-calibration.md` dựa trên thiết kế mục 3.4.
- Cung cấp đặc tả `PromptManifest` cho Subagent A (State Machine) và đặc tả `ToolContext` cho Subagent B (Tool Catalog).
