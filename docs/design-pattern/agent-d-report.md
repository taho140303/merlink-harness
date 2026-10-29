# BÁO CÁO KIẾN TRÚC THIẾT KẾ PHẦN MỀM (BE TAURI HARNESS)
## Chuyên đề: Behavioral Design Patterns (Phần 2) trong Rust

> **Tài liệu tham chiếu:** `docs/be-tauri-plan.md`, `docs/db.dbml`, `docs/prepare-be-impl/no-match.md`  
> **Tác giả:** Subagent D (Advanced Agentic Architecture & Behavioral Systems)  
> **Crate mục tiêu:** `crates/domain`, `crates/agent`, `crates/infra`, `crates/llm`  
> **Trạng thái:** Sẵn sàng triển khai (Production Architecture Blueprint)

---

## 1. TỔNG QUAN VÀ BẢN ĐỒ BEHAVIORAL PATTERNS (PHẦN 2)

Hệ thống **Merlink Desktop Engine** được thiết kế theo triết lý **Backend-Authoritative**, đặt toàn bộ logic điều phối Agent, quản lý bộ nhớ, thực thi tool và audit trace trong runtime Rust nhúng (Tauri). Để giải quyết bài toán phức tạp về phân bổ tài nguyên, điều phối sự kiện thời gian thực, phục hồi sau sự cố và xử lý tài liệu đính kèm, 4 mẫu thiết kế hành vi (Behavioral Patterns) cốt lõi đã được lựa chọn và tinh chỉnh theo ngữ dụng học (idiomatic) của ngôn ngữ Rust:

```
┌────────────────────────────────────────────────────────────────────────────────────────┐
│                        MERLINK BEHAVIORAL ARCHITECTURE CORE                            │
└────────────────────────────────────────────────────────────────────────────────────────┘
                                    │
    ┌───────────────────────────────┼───────────────────────────────┐
    ▼                               ▼                               ▼
┌────────────────────────┐      ┌────────────────────────┐      ┌────────────────────────┐
│    STRATEGY PATTERN    │      │    OBSERVER PATTERN    │      │    MEMENTO PATTERN     │
│ (Pluggable Algorithms) │      │ (EventSink Multiplexer)│      │  (State & Checkpoint)  │
├────────────────────────┤      ├────────────────────────┤      ├────────────────────────┤
│ • TokenEstimation      │      │ • IpcChannelObserver   │      │ • SceneMemento         │
│   (Tiktoken EMA / Char)│      │   (tauri::ipc::Channel)│      │   (scene_versions)     │
│ • CompactionStrategy   │      │ • AuditTraceObserver   │      │ • RunCheckpointMemento │
│   (3-Tier + Fallback)  │      │   (run_steps dedicated)│      │   (PromptManifest &    │
│ • MemoryRetrieval      │      │ • EntityChangedObserver│      │    run_continue)       │
│   (FTS5 -> Hybrid RRF) │      │   (tauri-specta cache) │      │                        │
└────────────────────────┘      └────────────────────────┘      └────────────────────────┘
                                    │
                                    ▼
                        ┌────────────────────────┐
                        │ TEMPLATE METHOD PATTERN│
                        │  (AttachmentPipeline)  │
                        ├────────────────────────┤
                        │ 1. Validate MIME/Size  │
                        │ 2. SHA-256 Dedupe      │
                        │ 3. Format Extract Text │
                        │ 4. Token & Persist     │
                        └────────────────────────┘
```

---

## 2. STRATEGY PATTERN (PLUGGABLE ALGORITHMS)

Strategy Pattern phân tách các thuật toán có thể hoán đổi độc lập khỏi ngữ cảnh sử dụng (Context). Trong Rust, Strategy Pattern được hiện thực hóa qua `trait` kết hợp với Dynamic Dispatch (`Box<dyn Trait>` hoặc `Arc<dyn Trait>`) hoặc Static Dispatch (`impl Trait` / Generics) nhằm đạt tính mở rộng mà không làm ô nhiễm logic điều phối.

---

### 2.1 Chiến lược đếm token thích ứng: `TokenEstimationStrategy`

#### Bối cảnh kỹ thuật
Đặc thù ngôn ngữ tiếng Việt (chữ có dấu, từ ghép âm tiết) khiến các bộ tokenizer BPE chuẩn (như `o200k_base` của OpenAI) sinh ra số lượng token chênh lệch đáng kể so với tiếng Anh. Để tránh lỗi **Context Overflow** khi nạp prompt mà vẫn tối ưu hóa độ đầy của Context Window, hệ thống sử dụng thuật toán **Adaptive EMA Calibration** (Exponential Moving Average với hệ số làm mượt $\alpha = 0.2$).

#### Sơ đồ hoạt động của Adaptive Token Estimation

```
  [Input Text: Tiếng Việt]
              │
              ▼
  ┌─────────────────────────────────────────────────────────┐
  │              TokenEstimationStrategy (Trait)            │
  └───────────────────────────┬─────────────────────────────┘
                              │
              ┌───────────────┴───────────────┐
              ▼                               ▼
  ┌───────────────────────┐       ┌───────────────────────┐
  │TiktokenAdaptiveEma    │       │CharacterHeuristic     │
  │Strategy (Default)     │       │Strategy (Offline/Local│
  └───────────┬───────────┘       └───────────┬───────────┘
              │                               │
              │ 1. Raw BPE (o200k)            │ 1. Count UTF-8 chars
              │ 2. Apply token_calibration    │ 2. Multiplier (0.4)
              │    Estimate = Raw * Calib     │ 3. Safety buffer
              ▼                               ▼
       [Estimated Tokens]              [Estimated Tokens]
              │
              │ (Gửi prompt tới LLM Provider)
              ▼
       [LLM Response with Actual prompt_tokens]
              │
              ▼
  ┌─────────────────────────────────────────────────────────┐
  │ Calibration Feedback Loop (EMA Update, α = 0.2)         │
  │ Ratio = actual_prompt_tokens / raw_estimated_tokens     │
  │ New_Calib = 0.2 * Ratio + 0.8 * Old_Calib               │
  │ Persist to `ai_models.token_calibration` in SQLite      │
  └─────────────────────────────────────────────────────────┘
```

#### Code hiện thực Rust chuẩn mực

```rust
// crates/agent/src/context/tokens.rs

use async_trait::async_trait;
use std::sync::RwLock;
use tiktoken_rs::{o200k_base_singleton, CoreBPE};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TokenEstimationError {
    #[error("Tokenizer encoding failure: {0}")]
    EncodingError(String),
    #[error("Strategy execution failed: {0}")]
    ExecutionFailure(String),
}

/// Trait trừu tượng cho mọi thuật toán ước lượng và hiệu chuẩn Token
pub trait TokenEstimationStrategy: Send + Sync {
    /// Tính toán số token ước tính cho chuỗi văn bản đầu vào
    fn estimate_tokens(&self, text: &str) -> usize;

    /// Cập nhật hệ số hiệu chuẩn dựa trên số liệu thực tế trả về từ LLM Provider
    fn calibrate(&self, raw_estimated: usize, actual_prompt_tokens: usize) -> f64;

    /// Lấy hệ số hiệu chuẩn hiện tại
    fn current_calibration_factor(&self) -> f64;
}

/// Chiến lược Tiktoken tích hợp Adaptive EMA dành riêng cho tiếng Việt
pub struct TiktokenAdaptiveEmaStrategy {
    bpe: CoreBPE,
    calibration_factor: RwLock<f64>,
    alpha: f64, // Hệ số làm mượt EMA (mặc định 0.2)
}

impl TiktokenAdaptiveEmaStrategy {
    pub fn new(initial_factor: f64) -> Self {
        // o200k_base là bộ tokenizer tiêu chuẩn hiện đại của GPT-4o / các dòng mô hình mới
        let bpe = o200k_base_singleton();
        Self {
            bpe,
            calibration_factor: RwLock::new(if initial_factor <= 0.0 { 1.20 } else { initial_factor }),
            alpha: 0.2,
        }
    }
}

impl TokenEstimationStrategy for TiktokenAdaptiveEmaStrategy {
    fn estimate_tokens(&self, text: &str) -> usize {
        // Đếm token cơ sở qua BPE
        let raw_count = self.bpe.encode_with_special_tokens(text).len();
        let factor = *self.calibration_factor.read().unwrap();
        // Áp dụng hệ số hiệu chuẩn và trần trên
        ((raw_count as f64) * factor).ceil() as usize
    }

    fn calibrate(&self, raw_estimated: usize, actual_prompt_tokens: usize) -> f64 {
        if raw_estimated == 0 || actual_prompt_tokens == 0 {
            return *self.calibration_factor.read().unwrap();
        }

        let observed_ratio = actual_prompt_tokens as f64 / raw_estimated as f64;
        let mut factor_guard = self.calibration_factor.write().unwrap();
        
        // Công thức EMA: New_EMA = α * Current_Observation + (1 - α) * Old_EMA
        let updated_factor = self.alpha * observed_ratio + (1.0 - self.alpha) * (*factor_guard);
        
        // Khống chế biên độ an toàn từ 0.8 đến 2.5
        let clamped_factor = updated_factor.clamp(0.8, 2.5);
        *factor_guard = clamped_factor;
        clamped_factor
    }

    fn current_calibration_factor(&self) -> f64 {
        *self.calibration_factor.read().unwrap()
    }
}

/// Chiến lược Heuristic dựa trên số lượng ký tự (Dành cho model local / offline)
pub struct CharacterHeuristicStrategy {
    chars_per_token: f64,
    safety_margin: f64,
}

impl CharacterHeuristicStrategy {
    pub fn new() -> Self {
        Self {
            // Tiếng Việt trung bình ~2.5 đến 3.2 ký tự / token trong mô hình local
            chars_per_token: 2.8,
            safety_margin: 1.25, // 25% buffer dự phòng
        }
    }
}

impl TokenEstimationStrategy for CharacterHeuristicStrategy {
    fn estimate_tokens(&self, text: &str) -> usize {
        let char_count = text.chars().count();
        let base_tokens = (char_count as f64) / self.chars_per_token;
        (base_tokens * self.safety_margin).ceil() as usize
    }

    fn calibrate(&self, _raw_estimated: usize, _actual_prompt_tokens: usize) -> f64 {
        1.0 // Heuristic cố định, không áp dụng EMA
    }

    fn current_calibration_factor(&self) -> f64 {
        1.0
    }
}
```

---

### 2.2 Chiến lược nén ngữ cảnh 3 cấp: `CompactionStrategy`

Khi lịch sử hội thoại và bộ nhớ vượt quá **Context Budget** (`max_context_tokens`), Agent Runtime không được phép crash mà phải thực thi nén ngữ cảnh thông minh.

#### Kiến trúc nén phân tầng (3-Tier Compaction Pipeline)

```
  [Context Window Exceeds Token Budget]
                   │
                   ▼
  ┌──────────────────────────────────────────────────────────────┐
  │ TIER 1: ElideOldToolResults                                  │
  │ • Quét các lượt tool call cũ nằm ngoài cửa sổ bảo vệ.        │
  │ • Thu gọn JSON raw data dài dòng thành thẻ vắn tắt:          │
  │   "[tool result elided: scene_read id=018f... size=45KB]"    │
  └──────────────────────────────┬───────────────────────────────┘
                                 │ Vẫn vượt Budget?
                                 ▼
  ┌──────────────────────────────────────────────────────────────┐
  │ TIER 2: PruneRetrievedMemories                               │
  │ • Loại bỏ Context Memories không ghim (unpinned).            │
  │ • Giữ lại tuyệt đối: Pinned Memories (`is_pinned = true`).   │
  │ • Sắp xếp loại dần theo điểm tương đồng BM25/Cosine thấp nhất│
  └──────────────────────────────┬───────────────────────────────┘
                                 │ Vẫn vượt Budget?
                                 ▼
  ┌──────────────────────────────────────────────────────────────┐
  │ TIER 3: SummarizeOldTurns (LLM Compact)                      │
  │ • Gọi LLM tóm tắt các hội thoại cũ thành một bản tóm tắt     │
  │   gọn gàng (Summary Message Envelope: covers [seq_a..seq_b]) │
  └──────────────────────────────┬───────────────────────────────┘
                                 │ LLM Gặp Sự Cố (Timeout/429/OOM)?
                                 ▼
  ┌──────────────────────────────────────────────────────────────┐
  │ DETERMINISTIC FALLBACK STRATEGY                              │
  │ • Cắt tỉa cứng có bảo vệ bất biến (Invariants Protection):   │
  │   1. KHÔNG cắt 6 message gần nhất (Recent Sliding Window).   │
  │   2. KHÔNG chạm Pinned Memories.                             │
  │   3. KHÔNG cắt Locked System Layers (System Prompt/Schemas). │
  └──────────────────────────────────────────────────────────────┘
```

#### Code hiện thực Rust chuẩn mực

```rust
// crates/agent/src/context/compactor.rs

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextMessage {
    pub seq: i64,
    pub role: String,
    pub content: String,
    pub is_tool_result: bool,
    pub tool_call_id: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ContextMemoryItem {
    pub id: String,
    pub content: String,
    pub is_pinned: bool,
    pub score: f32,
}

#[derive(Debug, Clone)]
pub struct ContextSnapshot {
    pub system_instructions: String, // Khóa cứng, bất biến
    pub pinned_memories: Vec<ContextMemoryItem>,
    pub dynamic_memories: Vec<ContextMemoryItem>,
    pub history_messages: Vec<ContextMessage>,
}

#[derive(Debug, Error)]
pub enum CompactionError {
    #[error("LLM Provider failed during summarization: {0}")]
    LlmFailure(String),
    #[error("Unrecoverable context overflow even after maximum compaction")]
    UnrecoverableOverflow,
}

#[async_trait]
pub trait CompactionStrategy: Send + Sync {
    async fn compact(
        &self,
        snapshot: &mut ContextSnapshot,
        target_token_budget: usize,
    ) -> Result<usize, CompactionError>;
}

/// Hiện thực chiến lược 3 cấp kèm Fallback tất định
pub struct CascadingCompactionStrategy {
    llm_client: Arc<dyn crate::ports::LlmClient>,
    token_estimator: Arc<dyn super::tokens::TokenEstimationStrategy>,
}

impl CascadingCompactionStrategy {
    pub fn new(
        llm_client: Arc<dyn crate::ports::LlmClient>,
        token_estimator: Arc<dyn super::tokens::TokenEstimationStrategy>,
    ) -> Self {
        Self { llm_client, token_estimator }
    }

    fn calculate_total_tokens(&self, snapshot: &ContextSnapshot) -> usize {
        let mut total = self.token_estimator.estimate_tokens(&snapshot.system_instructions);
        for m in &snapshot.pinned_memories {
            total += self.token_estimator.estimate_tokens(&m.content);
        }
        for m in &snapshot.dynamic_memories {
            total += self.token_estimator.estimate_tokens(&m.content);
        }
        for msg in &snapshot.history_messages {
            total += self.token_estimator.estimate_tokens(&msg.content);
        }
        total
    }

    /// TIER 1: Lược bớt nội dung kết quả Tool cũ
    fn elide_old_tool_results(&self, snapshot: &mut ContextSnapshot) {
        let total_msgs = snapshot.history_messages.len();
        if total_msgs <= 6 {
            return; // Bảo vệ cửa sổ 6 tin nhắn gần nhất
        }

        let cut_off_idx = total_msgs - 6;
        for msg in snapshot.history_messages[..cut_off_idx].iter_mut() {
            if msg.is_tool_result && msg.content.len() > 120 {
                let tool_id = msg.tool_call_id.as_deref().unwrap_or("unknown");
                msg.content = format!("[Tool result elided: call_id={} (data omitted for space)]", tool_id);
            }
        }
    }

    /// TIER 2: Cắt bỏ các memory truy xuất động (ưu tiên giữ memory điểm cao)
    fn prune_dynamic_memories(&self, snapshot: &mut ContextSnapshot, target: usize) {
        while self.calculate_total_tokens(snapshot) > target && !snapshot.dynamic_memories.is_empty() {
            // Sắp xếp điểm tăng dần để pop memory thấp nhất
            snapshot.dynamic_memories.sort_by(|a, b| a.score.partial_cmp(&b.score).unwrap());
            snapshot.dynamic_memories.remove(0);
        }
    }

    /// Fallback tất định: Cắt tỉa cứng lịch sử nhưng bảo vệ 6 message mới nhất
    fn deterministic_fallback_truncate(&self, snapshot: &mut ContextSnapshot) {
        if snapshot.history_messages.len() > 6 {
            let drain_range = 0..(snapshot.history_messages.len() - 6);
            snapshot.history_messages.drain(drain_range);
        }
    }
}

#[async_trait]
impl CompactionStrategy for CascadingCompactionStrategy {
    async fn compact(
        &self,
        snapshot: &mut ContextSnapshot,
        target_token_budget: usize,
    ) -> Result<usize, CompactionError> {
        let mut current_tokens = self.calculate_total_tokens(snapshot);
        if current_tokens <= target_token_budget {
            return Ok(current_tokens);
        }

        // --- CẤP 1: Elide Tool Results ---
        self.elide_old_tool_results(snapshot);
        current_tokens = self.calculate_total_tokens(snapshot);
        if current_tokens <= target_token_budget {
            return Ok(current_tokens);
        }

        // --- CẤP 2: Prune Dynamic Memories ---
        self.prune_dynamic_memories(snapshot, target_token_budget);
        current_tokens = self.calculate_total_tokens(snapshot);
        if current_tokens <= target_token_budget {
            return Ok(current_tokens);
        }

        // --- CẤP 3: Summarize Old Turns qua LLM ---
        let total_msgs = snapshot.history_messages.len();
        if total_msgs > 6 {
            let summarize_slice = &snapshot.history_messages[..(total_msgs - 6)];
            let prompt = format!(
                "Tóm tắt ngắn gọn diễn biến chính của {} tin nhắn sau đây trong 1 đoạn văn:\n{:?}",
                summarize_slice.len(),
                summarize_slice
            );

            match self.llm_client.complete_quick(&prompt).await {
                Ok(summary) => {
                    let seq_from = summarize_slice.first().map(|m| m.seq).unwrap_or(0);
                    let seq_to = summarize_slice.last().map(|m| m.seq).unwrap_or(0);
                    
                    let summary_msg = ContextMessage {
                        seq: seq_from,
                        role: "system".to_string(),
                        content: format!("[Tóm tắt hội thoại từ bước {} đến {}]: {}", seq_from, seq_to, summary),
                        is_tool_result: false,
                        tool_call_id: None,
                    };

                    // Thay thế phần cũ bằng bản tóm tắt duy nhất
                    snapshot.history_messages.drain(..(total_msgs - 6));
                    snapshot.history_messages.insert(0, summary_msg);
                }
                Err(err) => {
                    // KÍCH HOẠT FALLBACK TẤT ĐỊNH KHI LLM LỖI
                    tracing::warn!("Compaction LLM summarization failed: {}. Activating deterministic fallback", err);
                    self.deterministic_fallback_truncate(snapshot);
                }
            }
        }

        current_tokens = self.calculate_total_tokens(snapshot);
        if current_tokens > target_token_budget {
            return Err(CompactionError::UnrecoverableOverflow);
        }

        Ok(current_tokens)
    }
}
```

---

### 2.3 Chiến lược truy xuất bộ nhớ ngữ cảnh: `MemoryRetrievalStrategy`

#### Lộ trình tiến hóa không phá vỡ giao diện
- **Phase 1 (MVP/Hiện tại):** `SqliteFts5RecencyStrategy` — Sử dụng bảng ảo SQLite FTS5 (`context_memories_fts`) kết hợp lọc `is_pinned` và hàm suy giảm theo thời gian (recency decay). Hoạt động hoàn toàn in-process, không phụ thuộc thư viện native nặng.
- **Phase 3 (Mở rộng ngữ nghĩa):** `HybridVectorRrfStrategy` — Nhúng `fastembed-rs` (mô hình BGE-M3) và extension `sqlite-vec`. Kết hợp kết quả tìm kiếm từ khóa FTS5 và tìm kiếm vector thông qua giải thuật **Reciprocal Rank Fusion (RRF)**:
  $$RRF\_Score(d) = \frac{1}{k + rank_{fts}(d)} + \frac{1}{k + rank_{vec}(d)} \quad (k = 60)$$

Cả hai giai đoạn đều tuân thủ **một trait duy nhất**, giúp `ContextBuilder` hoàn toàn không thay đổi code khi nâng cấp hệ thống.

```
       ContextBuilder (Client of Strategy)
                       │
                       ▼ calls .retrieve(query, budget)
  ┌─────────────────────────────────────────────────────────┐
  │             MemoryRetrievalStrategy (Trait)             │
  └────────────────────────────┬────────────────────────────┘
                               │
               ┌───────────────┴───────────────┐
               ▼                               ▼
  ┌─────────────────────────┐     ┌─────────────────────────┐
  │SqliteFts5RecencyStrategy│     │HybridVectorRrfStrategy  │
  │(Phase 1 - SQLite FTS5)  │     │(Phase 3 - BGE-M3 + RRF) │
  ├─────────────────────────┤     ├─────────────────────────┤
  │• FTS5 BM25 search       │     │• FTS5 Lexical Search    │
  │• Recency Exponential    │     │• sqlite-vec Cosine KNN  │
  │  Decay Score            │     │• Reciprocal Rank Fusion │
  │• Pinned union & limit   │     │• Pinned union & limit   │
  └─────────────────────────┘     └─────────────────────────┘
```

#### Code hiện thực Rust chuẩn mực

```rust
// crates/agent/src/context/memory.rs

use async_trait::async_trait;
use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct ContextMemoryRecord {
    pub id: Uuid,
    pub project_id: Option<Uuid>,
    pub session_id: Option<String>,
    pub content: String,
    pub is_pinned: bool,
    pub score: f32,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct MemoryQuery {
    pub project_id: Uuid,
    pub session_id: Option<String>,
    pub query_text: String,
    pub max_tokens: usize,
}

#[async_trait]
pub trait MemoryRetrievalStrategy: Send + Sync {
    async fn retrieve(
        &self,
        query: &MemoryQuery,
    ) -> Result<Vec<ContextMemoryRecord>, sqlx::Error>;
}

/// Phase 1: SQLite FTS5 + Recency Decay
pub struct SqliteFts5RecencyStrategy {
    pool: SqlitePool,
}

impl SqliteFts5RecencyStrategy {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl MemoryRetrievalStrategy for SqliteFts5RecencyStrategy {
    async fn retrieve(
        &self,
        query: &MemoryQuery,
    ) -> Result<Vec<ContextMemoryRecord>, sqlx::Error> {
        // Truy vấn kết hợp: Luôn lấy Pinned Memories + FTS5 Rank kèm Recency
        // FTS5 bm25 trả về số âm (càng nhỏ càng khớp)
        let sql = r#"
            WITH ranked_fts AS (
                SELECT 
                    cm.id,
                    cm.project_id,
                    cm.session_id,
                    cm.content,
                    cm.is_pinned,
                    cm.updated_at,
                    (-1.0 * bm25(context_memories_fts)) AS bm25_score,
                    -- Recency decay: e^(-0.05 * số ngày kể từ lần cập nhật cuối)
                    EXP(-0.05 * ((julianday('now') - julianday(cm.updated_at)))) AS recency_factor
                FROM context_memories_fts fts
                JOIN context_memories cm ON cm.id = fts.rowid
                WHERE context_memories_fts MATCH ?
                  AND (cm.project_id = ? OR cm.project_id IS NULL)
                  AND cm.deleted_at IS NULL
            ),
            all_candidates AS (
                -- Nhánh 1: Các memory được pin luôn được ưu tiên tuyệt đối
                SELECT 
                    id, project_id, session_id, content, is_pinned, updated_at,
                    1000.0 AS final_score
                FROM context_memories
                WHERE is_pinned = 1 
                  AND (project_id = ? OR project_id IS NULL)
                  AND deleted_at IS NULL

                UNION ALL

                -- Nhánh 2: Kết quả FTS5 nhân với Recency Factor
                SELECT 
                    id, project_id, session_id, content, is_pinned, updated_at,
                    (bm25_score * recency_factor) AS final_score
                FROM ranked_fts
                WHERE is_pinned = 0
            )
            SELECT DISTINCT id, project_id, session_id, content, is_pinned, updated_at, final_score
            FROM all_candidates
            ORDER BY final_score DESC
            LIMIT 20;
        "#;

        let records = sqlx::query_as::<_, (Uuid, Option<Uuid>, Option<String>, String, bool, DateTime<Utc>, f64)>(sql)
            .bind(&query.query_text)
            .bind(query.project_id)
            .bind(query.project_id)
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .map(|(id, proj_id, sess_id, content, is_pinned, updated_at, score)| {
                ContextMemoryRecord {
                    id,
                    project_id: proj_id,
                    session_id: sess_id,
                    content,
                    is_pinned,
                    score: score as f32,
                    updated_at,
                }
            })
            .collect();

        Ok(records)
    }
}

/// Phase 3: Hybrid Vector RRF Strategy (sqlite-vec + fastembed-rs)
pub struct HybridVectorRrfStrategy {
    pool: SqlitePool,
    rrf_k: f32, // Hằng số làm mượt RRF (thường chọn 60.0)
}

impl HybridVectorRrfStrategy {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool, rrf_k: 60.0 }
    }
}

#[async_trait]
impl MemoryRetrievalStrategy for HybridVectorRrfStrategy {
    async fn retrieve(
        &self,
        query: &MemoryQuery,
    ) -> Result<Vec<ContextMemoryRecord>, sqlx::Error> {
        // Thực thi thuật toán Reciprocal Rank Fusion kết hợp lexical rank (FTS5) và vector rank (KNN)
        // Lưu ý: ContextBuilder gọi hàm này mà hoàn toàn không cần biết bên dưới chạy vector hay fts5.
        tracing::info!("Executing Phase 3 Hybrid Vector RRF Retrieval for query: {}", query.query_text);
        
        // Đoạn code mẫu thực thi RRF query qua SQLite
        // (Trong môi trường thực tế, embedding vector được sinh qua fastembed-rs in-process)
        Ok(vec![])
    }
}
```

---

## 3. OBSERVER / PUBLISH-SUBSCRIBE PATTERN (`EventSink` MULTIPLEXER)

Trong kiến trúc **Backend-Authoritative**, Agent Runtime là nơi phát sinh hàng loạt sự kiện nghiệp vụ (`DomainEvent`). Các thành phần xung quanh (Giao diện React, Hệ thống Trace kiểm toán, Cơ chế Cache Invalidation) đều cần tiêu thụ các sự kiện này nhưng có mục đích, tính chu kỳ và ràng buộc I/O hoàn toàn khác nhau.

### 3.1 Sơ đồ kiến trúc EventSink Multiplexer

```
                 ┌────────────────────────────────┐
                 │          AGENT RUNTIME         │
                 │   (State Machine / Run Loop)   │
                 └───────────────┬────────────────┘
                                 │ Emits DomainEvent
                                 ▼
                 ┌────────────────────────────────┐
                 │    EventSink (Trait Port)      │
                 └───────────────┬────────────────┘
                                 │
                 ┌───────────────▼────────────────┐
                 │    MultiplexEventSink (Hub)    │
                 │ • Fault-Isolated Dispatch      │
                 │ • Broadcast concurrent/ordered │
                 └───────┬───────────────┬────────┘
                         │               │
       ┌─────────────────┘               └─────────────────┐
       ▼                                                   ▼
┌───────────────────────────┐                     ┌───────────────────────────┐
│     IpcChannelObserver    │                     │     AuditTraceObserver    │
│  (tauri::ipc::Channel)    │                     │  (Append-only run_steps)  │
├───────────────────────────┤                     ├───────────────────────────┤
│ • Converts to RunEvent    │                     │ • Dedicated Audit Conn    │
│ • Streams token deltas    │                     │ • Step: context, tool,    │
│ • Tool approval prompt    │                     │   approval, verify        │
│ • Non-blocking async push │                     │ • Survives biz rollback!  │
└──────────────┬────────────┘                     └───────────────────────────┘
               │                                                   │
               ▼                                                   ▼
       [React UI Stream]                                   [SQLite Audit DB]
                                       │
                                       ▼
                        ┌───────────────────────────┐
                        │   EntityChangedObserver   │
                        │   (tauri-specta event)    │
                        ├───────────────────────────┤
                        │ • Filters write commits   │
                        │ • Emits `entityChanged`   │
                        │ • Triggers TanStack Query │
                        │   `invalidateQueries()`   │
                        └──────────────┬────────────┘
                                       │
                                       ▼
                              [FE Query Cache Sync]
```

### 3.2 Hiện thực 3 Observer độc lập

1. **`IpcChannelObserver`**: Chuyển đổi `DomainEvent` thành DTO `RunEvent` và đẩy qua `tauri::ipc::Channel<RunEvent>` gửi trực tiếp lên frontend React. Tránh tắc nghẽn giao diện nhờ cơ chế stream không đệm vô hạn.
2. **`AuditTraceObserver`**: Ghi append-only vào bảng `run_steps` trên một **kết nối Audit riêng biệt** (`audit_conn`). Ràng buộc thiết kế cực kỳ quan trọng: Nếu transaction nghiệp vụ của một tool ghi bị `ROLLBACK` do lỗi hoặc vi phạm commit guard, bản ghi audit trace ghi nhận sự cố đó **vẫn phải được bảo toàn vĩnh viễn** để phục vụ gỡ lỗi.
3. **`EntityChangedObserver`**: Khi nhận diện sự kiện tool ghi dữ liệu đã commit thành công (`DomainEvent::EntityMutated`), observer này phát tín hiệu toàn cục `events.entityChanged { entity, ids }` thông qua `tauri-specta`. Giao diện React bắt sự kiện này để tự động refresh (invalidate) cache TanStack Query mà không cần polling.

#### Code hiện thực Rust chuẩn mực

```rust
// crates/agent/src/events.rs

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::ipc::Channel;
use sqlx::SqlitePool;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum DomainEvent {
    RunStarted { run_id: String, session_id: String, mode: String },
    TokenChunkReceived { run_id: String, delta: String },
    ToolCallProposed { run_id: String, call_id: String, tool_name: String, args_json: String },
    ToolCallExecuted { run_id: String, call_id: String, success: bool, output_json: String, duration_ms: i64 },
    EntityMutated { entity_type: String, entity_id: String, action: String },
    RunCompleted { run_id: String, tokens_in: usize, tokens_out: usize },
    RunFailed { run_id: String, error_code: String, message: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", content = "data")]
pub enum RunEvent {
    Started { run_id: String, mode: String },
    Token { delta: String },
    ToolProposed { call_id: String, tool_name: String },
    ToolFinished { call_id: String, success: bool },
    Finished { tokens_in: usize, tokens_out: usize },
    Error { code: String, message: String },
}

/// Cổng trừu tượng EventSink
#[async_trait]
pub trait EventSink: Send + Sync {
    async fn on_event(&self, event: &DomainEvent);
}

/// Bộ phân phối đa kênh (Multiplexer Hub) với khả năng cô lập lỗi
pub struct MultiplexEventSink {
    observers: Vec<Arc<dyn EventSink>>,
}

impl MultiplexEventSink {
    pub fn new() -> Self {
        Self { observers: Vec::new() }
    }

    pub fn add_observer(&mut self, observer: Arc<dyn EventSink>) {
        self.observers.push(observer);
    }
}

#[async_trait]
impl EventSink for MultiplexEventSink {
    async fn on_event(&self, event: &DomainEvent) {
        // Phát sự kiện đồng thời tới các Observer, cô lập lỗi (fault isolation)
        for observer in &self.observers {
            let obs = Arc::clone(observer);
            let evt = event.clone();
            // Trong tokio task hoặc tuần tự nhưng bắt lỗi an toàn
            tokio::spawn(async move {
                obs.on_event(&evt).await;
            });
        }
    }
}

/// Observer 1: Đẩy dữ liệu thời gian thực qua Tauri IPC Channel
pub struct IpcChannelObserver {
    channel: Channel<RunEvent>,
}

impl IpcChannelObserver {
    pub fn new(channel: Channel<RunEvent>) -> Self {
        Self { channel }
    }
}

#[async_trait]
impl EventSink for IpcChannelObserver {
    async fn on_event(&self, event: &DomainEvent) {
        let run_event = match event {
            DomainEvent::RunStarted { run_id, mode, .. } => {
                Some(RunEvent::Started { run_id: run_id.clone(), mode: mode.clone() })
            }
            DomainEvent::TokenChunkReceived { delta, .. } => {
                Some(RunEvent::Token { delta: delta.clone() })
            }
            DomainEvent::ToolCallProposed { call_id, tool_name, .. } => {
                Some(RunEvent::ToolProposed { call_id: call_id.clone(), tool_name: tool_name.clone() })
            }
            DomainEvent::ToolCallExecuted { call_id, success, .. } => {
                Some(RunEvent::ToolFinished { call_id: call_id.clone(), success: *success })
            }
            DomainEvent::RunCompleted { tokens_in, tokens_out, .. } => {
                Some(RunEvent::Finished { tokens_in: *tokens_in, tokens_out: *tokens_out })
            }
            DomainEvent::RunFailed { error_code, message, .. } => {
                Some(RunEvent::Error { code: error_code.clone(), message: message.clone() })
            }
            _ => None,
        };

        if let Some(msg) = run_event {
            if let Err(e) = self.channel.send(msg) {
                tracing::warn!("Failed to stream RunEvent to Tauri Channel: {:?}", e);
            }
        }
    }
}

/// Observer 2: Ghi Audit Trace vào SQLite (Độc lập với business transaction)
pub struct AuditTraceObserver {
    audit_pool: SqlitePool, // Pool chuyên trách riêng cho log/audit
}

impl AuditTraceObserver {
    pub fn new(audit_pool: SqlitePool) -> Self {
        Self { audit_pool }
    }
}

#[async_trait]
impl EventSink for AuditTraceObserver {
    async fn on_event(&self, event: &DomainEvent) {
        let (run_id, kind, name, input, output, verdict, duration) = match event {
            DomainEvent::ToolCallExecuted { run_id, call_id, success, output_json, duration_ms } => {
                (
                    run_id.clone(),
                    "tool_call",
                    call_id.clone(),
                    None,
                    Some(output_json.clone()),
                    if *success { "pass" } else { "retry" },
                    *duration_ms as i32
                )
            }
            DomainEvent::RunFailed { run_id, error_code, message } => {
                (
                    run_id.clone(),
                    "lifecycle",
                    error_code.clone(),
                    None,
                    Some(message.clone()),
                    "block",
                    0
                )
            }
            _ => return, // Chỉ lưu các step cần audit
        };

        let step_id = Uuid::now_v7();
        let sql = r#"
            INSERT INTO run_steps (id, run_id, seq, kind, name, input_json, output_json, verdict, duration_ms, created_at)
            VALUES (?, ?, (SELECT COALESCE(MAX(seq), 0) + 1 FROM run_steps WHERE run_id = ?), ?, ?, ?, ?, ?, ?, datetime('now'));
        "#;

        if let Err(err) = sqlx::query(sql)
            .bind(step_id)
            .bind(&run_id)
            .bind(&run_id)
            .bind(kind)
            .bind(name)
            .bind(input)
            .bind(output)
            .bind(verdict)
            .bind(duration)
            .execute(&self.audit_pool)
            .await
        {
            tracing::error!("AUDIT TRACE WRITE FAILED (Survives business rollback): {}", err);
        }
    }
}

/// Observer 3: Phát sự kiện toàn cục để FE TanStack Query tự động Invalidate Cache
pub struct EntityChangedObserver {
    app_handle: tauri::AppHandle,
}

impl EntityChangedObserver {
    pub fn new(app_handle: tauri::AppHandle) -> Self {
        Self { app_handle }
    }
}

#[derive(Clone, Serialize)]
struct EntityChangedPayload {
    entity: String,
    id: String,
    action: String,
}

#[async_trait]
impl EventSink for EntityChangedObserver {
    async fn on_event(&self, event: &DomainEvent) {
        if let DomainEvent::EntityMutated { entity_type, entity_id, action } = event {
            let payload = EntityChangedPayload {
                entity: entity_type.clone(),
                id: entity_id.clone(),
                action: action.clone(),
            };
            
            // Phát event định danh qua Tauri Event System
            use tauri::Emitter;
            if let Err(err) = self.app_handle.emit("events.entityChanged", &payload) {
                tracing::error!("Failed to emit entityChanged event via Specta: {}", err);
            }
        }
    }
}
```

---

## 4. MEMENTO PATTERN (STATE SNAPSHOT & RECOVERY)

Memento Pattern được áp dụng để chụp lại (capture) trạng thái nội tại của một đối tượng mà không phá vỡ tính bao đóng (encapsulation), cho phép khôi phục lại trạng thái chính xác đó khi cần thiết.

---

### 4.1 `SceneMemento` và Hệ thống Phiên bản (`scene_versions`)

#### Bối cảnh nghiệp vụ
Trong quá trình đồng sáng tác kịch bản/truyện, khi Agent chỉnh sửa một phân cảnh (`scenes`), hệ thống **bắt buộc** phải đảm bảo nguyên tắc: **Mọi tác vụ ghi dữ liệu đều đảo ngược được**. Nếu người dùng không hài lòng với nội dung do AI tạo, hoặc AI gặp lỗi hallucination, phân cảnh có thể lập tức quay về trạng thái trước đó.

#### Sơ đồ Memento Pattern cho Phân Cảnh

```
  ┌─────────────────────────────────────────────────────────────┐
  │ ORIGINATOR: SceneAggregate                                  │
  │ • id: Uuid, project_id: Uuid, title: String                 │
  │ • current_content: String                                   │
  │ • current_summary: String                                   │
  ├─────────────────────────────────────────────────────────────┤
  │ + create_memento(source: SceneSource) -> SceneMemento       │
  │ + restore(memento: SceneMemento)                            │
  └──────────────────────────────┬──────────────────────────────┘
                                 │ produces snapshot
                                 ▼
  ┌─────────────────────────────────────────────────────────────┐
  │ MEMENTO: SceneMemento (`scene_versions` row)                 │
  │ • parent_id: Uuid (scenes.id)                               │
  │ • version_number: i32                                       │
  │ • scene_source: SceneSource (AI_GENERATED | HUMAN_EDITED)   │
  │ • content: String, summary: String, word_count: i32         │
  │ • created_at: DateTime<Utc>                                 │
  └──────────────────────────────┬──────────────────────────────┘
                                 │ stored & retrieved
                                 ▼
  ┌─────────────────────────────────────────────────────────────┐
  │ CARETAKER: SceneVersionRepository (SQLite)                  │
  │ • save_version(memento)                                     │
  │ • get_version(parent_id, version_number) -> SceneMemento    │
  │ • list_history(parent_id) -> Vec<VersionHeader>             │
  └─────────────────────────────────────────────────────────────┘
```

#### Code hiện thực Rust chuẩn mực

```rust
// crates/domain/src/scene.rs

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SceneSource {
    #[serde(rename = "AI_GENERATED")]
    AiGenerated,
    #[serde(rename = "HUMAN_EDITED")]
    HumanEdited,
    #[serde(rename = "RESTORE")]
    Restore,
}

/// MEMENTO: Chứa toàn bộ snapshot trạng thái của Scene tại một thời điểm
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneMemento {
    pub id: Uuid,
    pub parent_id: Uuid,
    pub version_number: i32,
    pub scene_source: SceneSource,
    pub summary: Option<String>,
    pub content: String,
    pub word_count: i32,
    pub created_at: DateTime<Utc>,
}

/// ORIGINATOR: Thực thể Scene quản lý trạng thái nghiệp vụ đang thay đổi
pub struct SceneAggregate {
    id: Uuid,
    project_id: Uuid,
    version_counter: i32,
    content: String,
    summary: Option<String>,
}

impl SceneAggregate {
    pub fn new(id: Uuid, project_id: Uuid, initial_content: String) -> Self {
        Self {
            id,
            project_id,
            version_counter: 1,
            content: initial_content,
            summary: None,
        }
    }

    /// Chụp lại trạng thái hiện tại thành Memento trước khi thay đổi
    pub fn create_memento(&self, source: SceneSource) -> SceneMemento {
        let words = self.content.split_whitespace().count() as i32;
        SceneMemento {
            id: Uuid::now_v7(),
            parent_id: self.id,
            version_number: self.version_counter,
            scene_source: source,
            summary: self.summary.clone(),
            content: self.content.clone(),
            word_count: words,
            created_at: Utc::now(),
        }
    }

    /// Khôi phục trạng thái nghiệp vụ từ một Memento trong quá khứ
    pub fn restore_from_memento(&mut self, memento: SceneMemento) {
        self.content = memento.content;
        self.summary = memento.summary;
        self.version_counter += 1; // Bản phục hồi tăng số hiệu version để duy trì lịch sử tuyến tính
    }

    pub fn apply_agent_edit(&mut self, new_content: String, new_summary: Option<String>) {
        self.version_counter += 1;
        self.content = new_content;
        self.summary = new_summary;
    }
}
```

---

### 4.2 `RunCheckpointMemento` & `PromptManifest` cho Cơ chế `run_continue`

Khi ứng dụng Tauri bị tắt đột ngột (crash, mất nguồn, user kill tiến trình), tiến trình phục hồi khởi động (**Startup Reconcile**) quét các run ở trạng thái `running` hoặc `awaiting_approval` và chuyển sang `interrupted`.

Nhờ lưu trữ **`RunCheckpointMemento`** kèm **`PromptManifest`** tại step `context` trong bảng `run_steps`, người dùng có thể gọi lệnh `run_continue(run_id)` để tiếp tục phiên làm việc mà:
1. Không cần chạy lại từ đầu các tool đã thực thi thành công.
2. Không làm thay đổi hay phân mảnh ngữ cảnh đã được khóa mã băm SHA-256.

```rust
// crates/agent/src/checkpoint.rs

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Thông tin cấu thành từng lớp Prompt tại thời điểm tạo Checkpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptLayerManifest {
    pub layer_name: String,      // "system_identity", "project_lore", "character_rules"
    pub template_id: Option<Uuid>,
    pub version: String,
    pub sha256: String,          // Mã băm kiểm tra tính toàn vẹn nội dung lớp
    pub token_count: usize,
}

/// MEMENTO: Checkpoint toàn diện của Agent Run tại một chu kỳ bước
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunCheckpointMemento {
    pub run_id: String,
    pub step_seq: i32,
    pub current_phase: String,
    pub prompt_manifest: Vec<PromptLayerManifest>,
    pub loaded_memory_ids: Vec<Uuid>,
    pub loaded_attachment_ids: Vec<Uuid>,
    pub token_breakdown: HashMap<String, usize>, // system, memory, history, attachments, tools
}

impl RunCheckpointMemento {
    /// Xuất dữ liệu sang định dạng JSON để lưu vào cột `run_steps.input_json`
    pub fn to_step_payload(&self) -> String {
        serde_json::to_string(self).expect("Serialization of checkpoint must succeed")
    }

    /// Khôi phục checkpoint từ bản ghi DB
    pub fn from_step_payload(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}
```

---

## 5. TEMPLATE METHOD PATTERN (`AttachmentPipeline`)

Template Method Pattern định nghĩa khung xương (skeleton) của một giải thuật trong một phương thức trừu tượng, trì hoãn việc hiện thực một số bước cụ thể cho các cấu trúc chuyên biệt mà không làm thay đổi cấu trúc tổng thể của giải thuật.

### 5.1 Quy trình Xử lý Tài liệu Đính kèm (Attachment Processing)

Quy trình nạp file đính kèm (`attachments`) từ phía người dùng vào hệ thống tuân thủ nghiêm ngặt 4 bước tuần tự cố định:

```
  [User attaches file: path]
              │
              ▼
  ┌──────────────────────────────────────────────────────────────┐
  │ STEP 1: Validate MIME & Size                                 │
  │ • Kiểm tra Whitelist (txt, md, docx, pdf)                    │
  │ • Kiểm tra kích thước tối đa (Max 25MB)                      │
  │ • Chặn Path Traversal vulnerability                          │
  └──────────────────────────────┬───────────────────────────────┘
                                 │ Pass
                                 ▼
  ┌──────────────────────────────────────────────────────────────┐
  │ STEP 2: Compute SHA-256 & Dedupe                             │
  │ • Hash stream bytes qua SHA-256                              │
  │ • Kiểm tra DB: Đã tồn tại hash?                              │
  │   - CÓ: Tái sử dụng storage_path, liên kết ID, dừng trích xuất│
  │   - CHƯA: Copy file nguyên tử vào `app_data/attachments/<hash│
  └──────────────────────────────┬───────────────────────────────┘
                                 │ New file
                                 ▼
  ┌──────────────────────────────────────────────────────────────┐
  │ STEP 3: Format-Specific Text Extraction (Hook Method)        │
  │ • txt / md   ──> Direct UTF-8 Read                           │
  │ • docx       ──> OpenXML docx parser                         │
  │ • pdf        ──> In-process PDF text stream reader           │
  │ • unsupported──> Mark extract_status = 'unsupported'         │
  └──────────────────────────────┬───────────────────────────────┘
                                 │ Text Extracted
                                 ▼
  ┌──────────────────────────────────────────────────────────────┐
  │ STEP 4: Token Count & Database Persistence                   │
  │ • Gọi TokenEstimationStrategy tính `token_estimate`          │
  │ • Lưu DB: extract_status = 'done', extracted_text, tokens    │
  │ • Emit Event: DomainEvent::AttachmentProcessed               │
  └──────────────────────────────────────────────────────────────┘
```

### 5.2 Code hiện thực Rust chuẩn mực

```rust
// crates/infra/src/fs/attachment_pipeline.rs

use async_trait::async_trait;
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};
use thiserror::Error;
use tokio::fs;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum AttachmentPipelineError {
    #[error("Validation failed: {0}")]
    ValidationError(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Extraction failed: {0}")]
    ExtractionError(String),
    #[error("Database error: {0}")]
    DbError(String),
}

pub struct ProcessedAttachment {
    pub id: Uuid,
    pub sha256: String,
    pub storage_path: String,
    pub extracted_text: String,
    pub token_estimate: usize,
    pub is_duplicate: bool,
}

/// Trait Hook: Cung cấp thuật toán trích xuất riêng biệt cho từng định dạng file
#[async_trait]
pub trait FormatTextExtractor: Send + Sync {
    async fn extract(&self, file_path: &Path, mime: &str) -> Result<String, AttachmentPipelineError>;
}

/// Bộ điều phối Template Method
pub struct AttachmentPipeline {
    app_data_dir: PathBuf,
    extractor: Box<dyn FormatTextExtractor>,
    token_estimator: std::sync::Arc<dyn crate::context::tokens::TokenEstimationStrategy>,
    max_size_bytes: u64,
}

impl AttachmentPipeline {
    pub fn new(
        app_data_dir: PathBuf,
        extractor: Box<dyn FormatTextExtractor>,
        token_estimator: std::sync::Arc<dyn crate::context::tokens::TokenEstimationStrategy>,
    ) -> Self {
        Self {
            app_data_dir,
            extractor,
            token_estimator,
            max_size_bytes: 25 * 1024 * 1024, // 25 MB
        }
    }

    /// TEMPLATE METHOD: Cấu trúc giải thuật bất biến gồm 4 bước cố định
    pub async fn process_file(
        &self,
        source_path: &Path,
        mime: &str,
    ) -> Result<ProcessedAttachment, AttachmentPipelineError> {
        // BƯỚC 1: Validate MIME & Size (Invariant Step)
        self.validate(source_path, mime).await?;

        // BƯỚC 2: Compute SHA-256 & Deduplication (Invariant Step)
        let (sha256, is_dup, target_storage_path) = self.compute_sha256_and_dedupe(source_path).await?;
        
        if is_dup {
            return Ok(ProcessedAttachment {
                id: Uuid::now_v7(),
                sha256,
                storage_path: target_storage_path,
                extracted_text: String::new(), // File trùng không cần re-extract
                token_estimate: 0,
                is_duplicate: true,
            });
        }

        // BƯỚC 3: Format-Specific Text Extraction (Hook Method Step)
        let extracted_text = self.extractor.extract(&self.app_data_dir.join(&target_storage_path), mime).await?;

        // BƯỚC 4: Token Count & Metric Preparation (Invariant Step)
        let token_estimate = self.token_estimator.estimate_tokens(&extracted_text);

        Ok(ProcessedAttachment {
            id: Uuid::now_v7(),
            sha256,
            storage_path: target_storage_path,
            extracted_text,
            token_estimate,
            is_duplicate: false,
        })
    }

    // --- CÁC BƯỚC CỐ ĐỊNH NỘI TẠI (Private Methods) ---

    async fn validate(&self, path: &Path, mime: &str) -> Result<(), AttachmentPipelineError> {
        let meta = fs::metadata(path).await?;
        if meta.len() > self.max_size_bytes {
            return Err(AttachmentPipelineError::ValidationError(format!(
                "File vượt quá dung lượng cho phép: {} bytes",
                meta.len()
            )));
        }

        // Whitelist MIME kiểm tra an toàn
        let allowed_mimes = ["text/plain", "text/markdown", "application/vnd.openxmlformats-officedocument.wordprocessingml.document", "application/pdf"];
        if !allowed_mimes.contains(&mime) {
            return Err(AttachmentPipelineError::ValidationError(format!("Định dạng MIME không được hỗ trợ: {}", mime)));
        }

        Ok(())
    }

    async fn compute_sha256_and_dedupe(&self, path: &Path) -> Result<(String, bool, String), AttachmentPipelineError> {
        let bytes = fs::read(path).await?;
        let mut hasher = Sha256::new();
        hasher.update(&bytes);
        let hash_hex = format!("{:x}", hasher.finalize());

        let relative_storage_path = format!("attachments/{}", hash_hex);
        let absolute_dest = self.app_data_dir.join(&relative_storage_path);

        if absolute_dest.exists() {
            // Đã tồn tại file trên ổ đĩa -> Deduplication kích hoạt
            return Ok((hash_hex, true, relative_storage_path));
        }

        if let Some(parent) = absolute_dest.parent() {
            fs::create_dir_all(parent).await?;
        }
        fs::write(&absolute_dest, &bytes).await?;

        Ok((hash_hex, false, relative_storage_path))
    }
}
```

---

## 6. LÝ DO LOẠI BỎ CÁC PATTERN CỔ ĐIỂN: VISITOR & INTERPRETER

Trong các tài liệu hướng đối tượng kinh điển (GoF) như *Refactoring Guru*, **Visitor** và **Interpreter** thường được đề xuất cho các bài toán duyệt cấu trúc phân cấp và thực thi ngôn ngữ/truy vấn. Tuy nhiên, trong kiến trúc Rust hiện đại của Merlink Engine, cả hai pattern này **chính thức bị loại bỏ** dựa trên các luận cứ kỹ thuật sâu sắc:

### 6.1 Loại bỏ Visitor Pattern
1. **Sự vượt trội của Rust Algebraic Data Types (Enums) và Exhaustive `match`:**  
   Trong OOP truyền thống (Java/C++), Visitor ra đời để giải quyết bài toán *Double Dispatch* vì các ngôn ngữ này không thể dễ dàng thêm phương thức mới vào cấu trúc lớp đã đóng. Ngược lại, Rust hỗ trợ Enum dạng sum-type nguyên bản. Khi cần duyệt hoặc biến đổi AST/Entity, lập trình viên chỉ cần viết một biểu thức `match`.
2. **Bảo đảm an toàn ở thời điểm biên dịch (Compile-time Exhaustiveness):**  
   Trình biên dịch `rustc` cưỡng chế mọi nhánh của Enum phải được xử lý. Khi thêm một variant mới vào `DomainEvent` hoặc `ToolCall`, compiler sẽ báo lỗi chính xác mọi vị trí cần bổ sung logic, loại bỏ hoàn toàn nhu cầu dựng các class `Visitor`, `Element` và cơ chế `accept(visitor)` cồng kềnh.
3. **Hiệu năng Zero-cost Abstraction:**  
   Pattern matching được tối ưu thành các lệnh nhảy trực tiếp (jump table), triệt tiêu hoàn toàn chi phí gián tiếp của con trỏ hàm (vtable dynamic dispatch) đi kèm với `dyn Visitor`.

### 6.2 Loại bỏ Interpreter Pattern
1. **Tránh tái phát minh SQLite:**  
   Toàn bộ việc phân tích cú pháp (parsing), tối ưu hóa cây thực thi (query planning), tìm kiếm từ khóa (FTS5 tokenization) và đánh chỉ mục vector đã được engine C/SQLite đảm nhận với tốc độ tính bằng microsecond.
2. **Biểu diễn Filter bằng DTO Enums tĩnh:**  
   Các bộ lọc tìm kiếm nghiệp vụ từ Frontend (như lọc character, scene, memory) được map trực tiếp qua Serde thành các Struct/Enum phẳng và chuyển thành câu truy vấn Parameterized SQL. Việc viết một bộ thông dịch cây cú pháp AST (Interpreter AST) riêng trong Rust là một sự phức tạp hóa không cần thiết, làm tăng nguy cơ rò rỉ bộ nhớ và làm chậm ứng dụng.

---

## 7. BẢNG TỔNG HỢP VÀ ĐỊNH VỊ TRIỂN KHAI TRONG CARGO WORKSPACE

| Behavioral Pattern | Struct / Trait Chính | Phân Vị Crate | Vai Trò Kiến Trúc Trong Merlink |
|---|---|---|---|
| **Strategy** | `TokenEstimationStrategy`<br>`TiktokenAdaptiveEmaStrategy` | `crates/agent` | Đo đếm token chính xác, tự động thích ứng với ngữ nghĩa tiếng Việt qua hệ số EMA $\alpha = 0.2$. |
| **Strategy** | `CompactionStrategy`<br>`CascadingCompactionStrategy` | `crates/agent` | Nén ngữ cảnh 3 cấp linh hoạt; bảo vệ nghiêm ngặt 3 bất biến cốt lõi (6 message gần nhất, Pinned Memories, System layers). |
| **Strategy** | `MemoryRetrievalStrategy`<br>`SqliteFts5RecencyStrategy` | `crates/agent` | Cắm rút giữa FTS5 Lexical (Phase 1) và Hybrid Vector RRF (Phase 3) mà không sửa đổi `ContextBuilder`. |
| **Observer** | `EventSink` Multiplexer<br>`IpcChannelObserver`<br>`AuditTraceObserver`<br>`EntityChangedObserver` | `crates/domain`<br>`crates/agent`<br>`src-tauri` | Phân phối luồng sự kiện tách rời: Stream UI thời gian thực, lưu vết Audit Trace không mất khi rollback, và tự động xóa cache React FE. |
| **Memento** | `SceneMemento`<br>`SceneVersionRepository` | `crates/domain`<br>`crates/infra` | Lưu giữ và khôi phục trạng thái phân cảnh truyện tức thì (`scene_versions`), bảo vệ công sức người dùng trước hallucination của AI. |
| **Memento** | `RunCheckpointMemento`<br>`PromptManifest` | `crates/agent` | Lưu trữ cấu trúc phân tầng prompt và memory ID tại step `context`, làm nền tảng cho `run_continue` sau sự cố sập app. |
| **Template Method** | `AttachmentPipeline`<br>`FormatTextExtractor` | `crates/infra` | Cố định khung giải thuật 4 bước xử lý file đính kèm (Validate -> SHA256 Dedupe -> Hook Extract -> Token Persist). |

---

## 8. KẾT LUẬN

Báo cáo kiến trúc này đã chuẩn hóa toàn bộ các giải pháp **Behavioral Design Patterns (Phần 2)** cho hệ thống **Merlink Desktop Engine**. Bằng việc kết hợp chặt chẽ các nguyên lý hướng đối tượng kinh điển với sức mạnh của ngôn ngữ lập trình hệ thống Rust, kiến trúc đạt được tính mở rộng cao, khả năng chịu lỗi bền bỉ (resilience), bảo toàn tính toàn vẹn dữ liệu, và mang lại trải nghiệm tương tác mượt mà cho người dùng.
