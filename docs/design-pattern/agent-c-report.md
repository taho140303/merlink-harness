# BÁO CÁO KIẾN TRÚC CHUYÊN SÂU: BEHAVIORAL DESIGN PATTERNS TRONG RUST CHO BACKEND TAURI (MERLINK AGENT ENGINE)
## PHẦN 1: STATE, COMMAND, CHAIN OF RESPONSIBILITY, ITERATOR / ASYNC STREAM

---

**Tác giả:** Subagent C (AI Architecture Specialist - Advanced Agentic Coding)  
**Mục tiêu:** Hệ thống hóa và triển khai chuẩn mực các mẫu thiết kế hành vi (Behavioral Patterns) cho lõi thực thi AI Agent (Harness Engine) trên nền tảng Tauri v2 (Rust) + SQLite (`db.dbml`).  
**Tài liệu tham chiếu:** `docs/be-tauri-plan.md`, `docs/db.dbml`, `rust-unofficial.github.io`, `refactoring.guru/design-patterns/behavioral-patterns`.

---

## 1. TỔNG QUAN KIẾN TRÚC & NGUYÊN TẮC THIẾT KẾ CỐT LÕI

Trong bối cảnh xây dựng ứng dụng hỗ trợ sáng tác tiểu thuyết **Merlink**, AI Agent không chỉ đơn thuần là một chatbot hỏi-đáp mà là một hệ thống tự trị tác động trực tiếp vào cơ sở dữ liệu quan hệ phức tạp (chương hồi, phân cảnh, nhân vật, motif ràng buộc, dòng thời gian). Các yêu cầu khắt khe bao gồm:
1. **Tính tất định (Determinism) & An toàn bộ nhớ (Memory Safety):** Không cho phép rò rỉ trạng thái, race conditions hoặc crash ứng dụng khi xử lý streaming dữ liệu từ LLM.
2. **Human-in-the-Loop (HitL) & Phê duyệt hàng loạt (Batch Approval):** Người dùng có quyền xem trước, từ chối, hoặc can thiệp sửa đổi các tham số gọi công cụ (`tool_calls`) trước khi chúng được áp dụng vào SQLite.
3. **Khả năng hoàn tác (Undoability / Compensating Actions):** Mọi đột biến dữ liệu (như ghi đè nội dung cảnh, xóa nhân vật) phải có khả năng rollback ngay lập tức mà không phá vỡ tính nhất quán của cơ sở dữ liệu.
4. **Kiểm duyệt đa tầng (Multi-stage Verification Pipeline):** Ngăn chặn việc LLM vi phạm cấu trúc schema JSON, vượt quá phạm vi dự án (`project_id`), hoặc vi phạm các quy tắc ràng buộc văn học (`motif_constraints`, tính duy nhất của điểm nhìn POV).

Để hiện thực hóa các yêu cầu trên trong Rust, nhóm nghiên cứu chuẩn hóa **4 mẫu thiết kế hành vi (Behavioral Patterns)** theo tinh thần hiện đại, loại bỏ các tư duy hướng đối tượng cũ kỹ (như kế thừa sâu, lạm dụng `Box<dyn Any>` cấp phát heap) và thay thế bằng các đặc trưng bản địa của Rust: **Algebraic Data Types (Enum), Ownership/Borrowing, Zero-cost Abstractions, và Sans-I/O Functional Core**.

---

## 2. PATTERN 1: STATE PATTERN (SANS-I/O ENUM STATE MACHINE)

### 2.1. Triết lý: Tại sao loại bỏ OOP State Pattern truyền thống?

Mô hình State Pattern kinh điển trong OOP (Gang of Four) thường định nghĩa một abstract interface `State` với các phương thức như `handle()`, sau đó tạo các concrete class như `IdleState`, `StreamingState`. Trong Rust, cách tiếp cận này buộc lập trình viên phải sử dụng:
* Con trỏ trait đối tượng cấp phát heap `Box<dyn State>`.
* Kỹ thuật nội đột biến phức tạp (`Arc<Mutex<dyn State>>` hoặc `Rc<RefCell<dyn State>>`).
* Thiếu khả năng kiểm tra vét cạn (Exhaustive Pattern Matching) ở compile-time, dễ dẫn đến runtime error.
* Khó khăn trong việc serialize trạng thái để lưu checkpoint vào SQLite hoặc gửi qua Tauri IPC Event.

**Giải pháp tối ưu trong Rust: Sans-I/O Algebraic State Machine.**
Toàn bộ trạng thái của một vòng đời Agent Turn được biểu diễn bằng một Enum gắn kèm dữ liệu payload cụ thể (`RunPhase`). Toàn bộ logic nghiệp vụ được đóng gói trong một hàm chuyển trạng thái thuần túy (pure function):

$$\text{transition}: (\text{RunPhase}, \text{RunEvent}) \longrightarrow \text{Result}<(\text{RunPhase}, \text{Vec}<\text{DriverCommand}>), \text{TransitionError}>$$

* **Functional Core (Lõi thuần túy):** Không thực hiện bất kỳ thao tác I/O nào (không gọi mạng, không đọc DB, không emit Tauri event). Điều này cho phép viết unit test đạt độ bao phủ 100% một cách deterministic chỉ trong vài microsecond mà không cần mock thư viện I/O.
* **Imperative Shell (Vỏ điều phối I/O):** Một async event loop đơn giản nhận `DriverCommand` từ Functional Core để thực thi I/O thực tế (HTTP request, SQLite transaction, Tauri channel emit) rồi gửi `RunEvent` ngược trở lại Core.

### 2.2. Sơ đồ máy trạng thái tổng thể (ASCII State Lifecycle)

```text
                           +----------------------+
                           |         IDLE         |
                           +----------+-----------+
                                      | Event::StartRun(prompt, project_id)
                                      v
                           +----------------------+
                           |   BUILDING_CONTEXT   |
                           +----------+-----------+
                                      | Event::ContextReady(messages, tools)
                                      v
                           +----------------------+
                           |      STREAMING       | <---------------+
                           +----------+-----------+                 |
                                      | Event::StreamCompleted(tool_calls)
                                      v                             |
                           +----------------------+                 |
                           |     POLICY_CHECK     |                 |
                           +----------+-----------+                 |
                                      |                             |
                 +--------------------+--------------------+        |
                 | (Requires Approval)                     |        |
                 v                                         v        |
      +---------------------+                  +------------------+ |
      |  AWAITING_APPROVAL  |                  |  VERIFYING_PRE   | |
      +----------+----------+                  +--------+---------+ |
                 | Event::BatchApproved(commands)       |           |
                 +--------------------------------------+           |
                                      | Event::PreVerificationPassed|
                                      v                             |
                           +----------------------+                 |
                           |      EXECUTING       |                 |
                           +----------+-----------+                 |
                                      | Event::ExecutionSucceeded   |
                                      v                             |
                           +----------------------+                 |
                           |    VERIFYING_POST    |                 |
                           +----------+-----------+                 |
                                      |                             |
                 +--------------------+--------------------+        |
                 | Event::PostPassed                       | Event::RetryRequested
                 v                                         | (Feedback to LLM)
      +---------------------+                              +--------+
      |     FINALIZING      |
      +----------+----------+
                 | Event::Finalized
                 v
   ===================================================================
   TERMINAL STATES:
   * COMPLETED   : Hoàn tất toàn bộ yêu cầu, đã commit DB và cập nhật UI.
   * CANCELLED   : Người dùng chủ động hủy bỏ từ UI.
   * BLOCKED     : Vi phạm chính sách an toàn hoặc Domain Handler chặn cứng.
   * FAILED      : Lỗi kỹ thuật không thể khắc phục (Network, SQLite crash).
   * INTERRUPTED : Bị ngắt bởi tác vụ ưu tiên cao hơn hoặc timeout.
   ===================================================================
```

### 2.3. Cấu trúc dữ liệu & Code Rust chuẩn mực (Sans-I/O Implementation)

```rust
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// --- 1. RUN PHASE (CÁC TRẠNG THÁI HỢP LỆ) ---
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RunPhase {
    Idle,
    BuildingContext {
        run_id: Uuid,
        project_id: Uuid,
        user_prompt: String,
    },
    Streaming {
        run_id: Uuid,
        accumulated_text: String,
        pending_tool_calls: Vec<RawToolCall>,
    },
    PolicyCheck {
        run_id: Uuid,
        tool_commands: Vec<ToolCommand>,
    },
    AwaitingApproval {
        run_id: Uuid,
        batch_id: Uuid,
        commands: Vec<ToolCommand>,
    },
    VerifyingPre {
        run_id: Uuid,
        approved_commands: Vec<ToolCommand>,
    },
    Executing {
        run_id: Uuid,
        active_command_idx: usize,
        commands: Vec<ToolCommand>,
    },
    VerifyingPost {
        run_id: Uuid,
        executed_commands: Vec<ToolCommand>,
    },
    Finalizing {
        run_id: Uuid,
        summary: String,
    },
    Completed { run_id: Uuid },
    Cancelled { run_id: Uuid, reason: String },
    Blocked { run_id: Uuid, reason: String },
    Failed { run_id: Uuid, error_message: String },
    Interrupted { run_id: Uuid },
}

// --- 2. RUN EVENT (CÁC BIẾN CỐ ĐẦU VÀO TỪ SHELL) ---
#[derive(Debug, Clone, PartialEq)]
pub enum RunEvent {
    StartRun { run_id: Uuid, project_id: Uuid, prompt: String },
    ContextBuilt { system_prompt: String, messages: Vec<ChatMessage> },
    StreamChunkReceived(StreamItem),
    StreamFinished { tool_calls: Vec<RawToolCall> },
    PolicyEvaluated { auto_approved: bool, commands: Vec<ToolCommand> },
    BatchApproved { batch_id: Uuid, edited_commands: Vec<ToolCommand> },
    BatchRejected { batch_id: Uuid, reason: String },
    PreVerificationPassed,
    PreVerificationFailed { feedback: StructuredFeedback, retry_count: usize },
    ExecutionStepCompleted { step_idx: usize },
    ExecutionAllCompleted,
    PostVerificationPassed,
    PostVerificationFailed { feedback: StructuredFeedback },
    FinalizationDone,
    UserCancelRequested { reason: String },
    FatalErrorOccurred { error: String },
}

// --- 3. DRIVER COMMAND (CHỈ THỊ GỬI RA IMPERATIVE SHELL) ---
#[derive(Debug, Clone, PartialEq)]
pub enum DriverCommand {
    LoadContextFromDb { project_id: Uuid },
    InitiateLlmStream { messages: Vec<ChatMessage> },
    EmitUiChunk { text: String },
    EmitApprovalRequest { batch_id: Uuid, commands: Vec<ToolCommand> },
    RunPreVerificationPipeline { commands: Vec<ToolCommand> },
    ExecuteToolBatch { commands: Vec<ToolCommand> },
    RunPostVerificationPipeline { commands: Vec<ToolCommand> },
    CommitTransactionAndPersist,
    RollbackTransaction,
    NotifyUiRunStatus { phase_name: &'static str },
}

// --- 4. CÁC KIỂU DỮ LIỆU BỔ TRỢ ---
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RawToolCall {
    pub call_id: String,
    pub tool_name: String,
    pub raw_arguments: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToolCommand {
    pub call_id: String,
    pub tool_name: String,
    pub original_args: serde_json::Value,
    pub edited_args: serde_json::Value,
    pub requires_user_approval: bool,
    pub savepoint_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StreamItem {
    Token(String),
    ToolCallDelta(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructuredFeedback {
    pub failed_stage: &'static str,
    pub violations: Vec<String>,
}

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum TransitionError {
    #[error("Invalid event {event:?} received in phase {phase:?}")]
    UnexpectedEvent { phase: String, event: String },
    #[error("State invariant violated: {0}")]
    InvariantViolation(String),
}

// --- 5. HÀM CHUYỂN TRẠNG THÁI THUẦN TÚY (PURE FUNCTIONAL CORE) ---
pub fn transition(
    current: RunPhase,
    event: RunEvent,
) -> Result<(RunPhase, Vec<DriverCommand>), TransitionError> {
    // 5.1. Xử lý các sự kiện ngắt toàn cục (Global Interrupts)
    if let RunEvent::UserCancelRequested { reason } = &event {
        let run_id = get_run_id(&current).unwrap_or_else(Uuid::new_v4);
        return Ok((
            RunPhase::Cancelled { run_id, reason: reason.clone() },
            vec![DriverCommand::RollbackTransaction, DriverCommand::NotifyUiRunStatus { phase_name: "Cancelled" }],
        ));
    }

    if let RunEvent::FatalErrorOccurred { error } = &event {
        let run_id = get_run_id(&current).unwrap_or_else(Uuid::new_v4);
        return Ok((
            RunPhase::Failed { run_id, error_message: error.clone() },
            vec![DriverCommand::RollbackTransaction, DriverCommand::NotifyUiRunStatus { phase_name: "Failed" }],
        ));
    }

    // 5.2. Chuyển trạng thái theo từng giai đoạn
    match (current, event) {
        (RunPhase::Idle, RunEvent::StartRun { run_id, project_id, prompt }) => {
            let next = RunPhase::BuildingContext {
                run_id,
                project_id,
                user_prompt: prompt,
            };
            let cmds = vec![
                DriverCommand::NotifyUiRunStatus { phase_name: "BuildingContext" },
                DriverCommand::LoadContextFromDb { project_id },
            ];
            Ok((next, cmds))
        }

        (RunPhase::BuildingContext { run_id, .. }, RunEvent::ContextBuilt { messages, .. }) => {
            let next = RunPhase::Streaming {
                run_id,
                accumulated_text: String::new(),
                pending_tool_calls: Vec::new(),
            };
            let cmds = vec![
                DriverCommand::NotifyUiRunStatus { phase_name: "Streaming" },
                DriverCommand::InitiateLlmStream { messages },
            ];
            Ok((next, cmds))
        }

        (RunPhase::Streaming { run_id, mut accumulated_text, pending_tool_calls }, RunEvent::StreamChunkReceived(item)) => {
            let mut cmds = Vec::new();
            if let StreamItem::Token(t) = item {
                accumulated_text.push_str(&t);
                cmds.push(DriverCommand::EmitUiChunk { text: t });
            }
            let next = RunPhase::Streaming {
                run_id,
                accumulated_text,
                pending_tool_calls,
            };
            Ok((next, cmds))
        }

        (RunPhase::Streaming { run_id, .. }, RunEvent::StreamFinished { tool_calls }) => {
            if tool_calls.is_empty() {
                // Không gọi tool, chuyển thẳng đến Finalizing
                let next = RunPhase::Finalizing { run_id, summary: "Completed direct answer".into() };
                Ok((next, vec![DriverCommand::NotifyUiRunStatus { phase_name: "Finalizing" }]))
            } else {
                let commands = tool_calls.into_iter().map(|tc| ToolCommand {
                    call_id: tc.call_id,
                    tool_name: tc.tool_name,
                    original_args: serde_json::from_str(&tc.raw_arguments).unwrap_or(serde_json::Value::Null),
                    edited_args: serde_json::Value::Null,
                    requires_user_approval: true, // Mặc định bật kiểm tra
                    savepoint_name: None,
                }).collect();

                let next = RunPhase::PolicyCheck { run_id, tool_commands: commands };
                Ok((next, vec![DriverCommand::NotifyUiRunStatus { phase_name: "PolicyCheck" }]))
            }
        }

        (RunPhase::PolicyCheck { run_id, tool_commands }, RunEvent::PolicyEvaluated { auto_approved, commands }) => {
            if auto_approved {
                let next = RunPhase::VerifyingPre { run_id, approved_commands: commands.clone() };
                Ok((next, vec![
                    DriverCommand::NotifyUiRunStatus { phase_name: "VerifyingPre" },
                    DriverCommand::RunPreVerificationPipeline { commands },
                ]))
            } else {
                let batch_id = Uuid::new_v4();
                let next = RunPhase::AwaitingApproval { run_id, batch_id, commands: commands.clone() };
                Ok((next, vec![
                    DriverCommand::NotifyUiRunStatus { phase_name: "AwaitingApproval" },
                    DriverCommand::EmitApprovalRequest { batch_id, commands },
                ]))
            }
        }

        (RunPhase::AwaitingApproval { run_id, .. }, RunEvent::BatchApproved { edited_commands, .. }) => {
            let next = RunPhase::VerifyingPre { run_id, approved_commands: edited_commands.clone() };
            Ok((next, vec![
                DriverCommand::NotifyUiRunStatus { phase_name: "VerifyingPre" },
                DriverCommand::RunPreVerificationPipeline { commands: edited_commands },
            ]))
        }

        (RunPhase::AwaitingApproval { run_id, .. }, RunEvent::BatchRejected { reason, .. }) => {
            let next = RunPhase::Blocked { run_id, reason };
            Ok((next, vec![DriverCommand::NotifyUiRunStatus { phase_name: "Blocked" }]))
        }

        (RunPhase::VerifyingPre { run_id, approved_commands }, RunEvent::PreVerificationPassed) => {
            let next = RunPhase::Executing {
                run_id,
                active_command_idx: 0,
                commands: approved_commands.clone(),
            };
            Ok((next, vec![
                DriverCommand::NotifyUiRunStatus { phase_name: "Executing" },
                DriverCommand::ExecuteToolBatch { commands: approved_commands },
            ]))
        }

        (RunPhase::Executing { run_id, commands, .. }, RunEvent::ExecutionAllCompleted) => {
            let next = RunPhase::VerifyingPost { run_id, executed_commands: commands.clone() };
            Ok((next, vec![
                DriverCommand::NotifyUiRunStatus { phase_name: "VerifyingPost" },
                DriverCommand::RunPostVerificationPipeline { commands },
            ]))
        }

        (RunPhase::VerifyingPost { run_id, .. }, RunEvent::PostVerificationPassed) => {
            let next = RunPhase::Finalizing { run_id, summary: "Run executed and verified successfully".into() };
            Ok((next, vec![
                DriverCommand::NotifyUiRunStatus { phase_name: "Finalizing" },
                DriverCommand::CommitTransactionAndPersist,
            ]))
        }

        (RunPhase::Finalizing { run_id, .. }, RunEvent::FinalizationDone) => {
            let next = RunPhase::Completed { run_id };
            Ok((next, vec![DriverCommand::NotifyUiRunStatus { phase_name: "Completed" }]))
        }

        (invalid_phase, invalid_event) => Err(TransitionError::UnexpectedEvent {
            phase: format!("{:?}", invalid_phase),
            event: format!("{:?}", invalid_event),
        }),
    }
}

fn get_run_id(phase: &RunPhase) -> Option<Uuid> {
    match phase {
        RunPhase::Idle => None,
        RunPhase::BuildingContext { run_id, .. }
        | RunPhase::Streaming { run_id, .. }
        | RunPhase::PolicyCheck { run_id, .. }
        | RunPhase::AwaitingApproval { run_id, .. }
        | RunPhase::VerifyingPre { run_id, .. }
        | RunPhase::Executing { run_id, .. }
        | RunPhase::VerifyingPost { run_id, .. }
        | RunPhase::Finalizing { run_id, .. }
        | RunPhase::Completed { run_id }
        | RunPhase::Cancelled { run_id, .. }
        | RunPhase::Blocked { run_id, .. }
        | RunPhase::Failed { run_id, .. }
        | RunPhase::Interrupted { run_id } => Some(*run_id),
    }
}
```

### 2.4. Deterministic Unit Testing (Sans-I/O Verification)

Nhờ tách biệt I/O, kiểm thử trạng thái không cần khởi động runtime Tokio, mock database pool, hay thiết lập server HTTP giả lập. Mọi test chạy tuần tự với tốc độ nano-giây:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_agent_flow_without_approval() {
        let run_id = Uuid::new_v4();
        let project_id = Uuid::new_v4();

        // 1. Idle -> StartRun
        let (phase1, cmds1) = transition(
            RunPhase::Idle,
            RunEvent::StartRun { run_id, project_id, prompt: "Viết mở đầu chương 1".into() },
        ).expect("Transition to BuildingContext failed");

        assert!(matches!(phase1, RunPhase::BuildingContext { .. }));
        assert_eq!(cmds1, vec![
            DriverCommand::NotifyUiRunStatus { phase_name: "BuildingContext" },
            DriverCommand::LoadContextFromDb { project_id },
        ]);

        // 2. BuildingContext -> ContextBuilt
        let (phase2, cmds2) = transition(
            phase1,
            RunEvent::ContextBuilt {
                system_prompt: "Sys prompt".into(),
                messages: vec![ChatMessage { role: "user".into(), content: "Prompt".into() }],
            },
        ).expect("Transition to Streaming failed");

        assert!(matches!(phase2, RunPhase::Streaming { .. }));
        assert_eq!(cmds2.len(), 2);

        // 3. Streaming -> StreamFinished (with tools)
        let tool_call = RawToolCall {
            call_id: "call_abc123".into(),
            tool_name: "update_scene_content".into(),
            raw_arguments: r#"{"scene_id":"018f6c44-325b-7888-8219-cbf900f0aa11","content":"Màn đêm buông xuống..."}"#.into(),
        };
        let (phase3, _) = transition(
            phase2,
            RunEvent::StreamFinished { tool_calls: vec![tool_call] },
        ).expect("Transition to PolicyCheck failed");

        assert!(matches!(phase3, RunPhase::PolicyCheck { .. }));

        // 4. PolicyCheck -> PolicyEvaluated (Auto approved)
        if let RunPhase::PolicyCheck { tool_commands, .. } = &phase3 {
            let (phase4, cmds4) = transition(
                phase3.clone(),
                RunEvent::PolicyEvaluated { auto_approved: true, commands: tool_commands.clone() },
            ).expect("Transition to VerifyingPre failed");

            assert!(matches!(phase4, RunPhase::VerifyingPre { .. }));
            assert_eq!(cmds4[0], DriverCommand::NotifyUiRunStatus { phase_name: "VerifyingPre" });
        }
    }

    #[test]
    fn test_user_cancel_at_any_time() {
        let run_id = Uuid::new_v4();
        let phase = RunPhase::Executing {
            run_id,
            active_command_idx: 0,
            commands: vec![],
        };

        let (next_phase, cmds) = transition(
            phase,
            RunEvent::UserCancelRequested { reason: "User stopped generation".into() },
        ).expect("Cancel should always succeed");

        assert_eq!(next_phase, RunPhase::Cancelled { run_id, reason: "User stopped generation".into() });
        assert_eq!(cmds, vec![
            DriverCommand::RollbackTransaction,
            DriverCommand::NotifyUiRunStatus { phase_name: "Cancelled" },
        ]);
    }

    #[test]
    fn test_invalid_event_transition_returns_error() {
        let phase = RunPhase::Idle;
        let event = RunEvent::PreVerificationPassed; // Không thể pass verify khi đang Idle!
        let result = transition(phase, event);
        assert!(result.is_err());
    }
}
```

---

## 3. PATTERN 2: COMMAND PATTERN (REVERSIBLE TOOL INVOCATIONS & BATCH APPROVAL)

### 3.1. Khái niệm & Sự gắn kết với Domain Model (`db.dbml`)

Trong các hệ thống Agentic thông thường, khi mô hình phát sinh `tool_calls`, hệ thống thường gọi trực tiếp hàm xử lý (RPC invocation). Cách tiếp cận này phá vỡ khả năng kiểm soát dữ liệu trong Merlink:
* Không thể gom nhóm các hành vi để người dùng duyệt chung một lượt (HitL Batching).
* Khi người dùng phát hiện tham số không hợp lý (ví dụ: LLM tự ý sinh tên nhân vật sai quy ước), không thể chỉnh sửa tham số trước khi thực thi.
* Khi thao tác gặp lỗi ở giữa chừng, DB bị rơi vào trạng thái rác cục bộ.

**Command Pattern** giải quyết triệt để vấn đề này bằng cách:
1. Đóng gói mọi Tool Call thành một đối tượng độc lập (Command Object).
2. Hỗ trợ cơ chế **Compensating Actions** và **Snapshotting** dựa trên cấu trúc bảng của `docs/db.dbml`:
   * **Bảng `scene_versions` (Snapshotting):** Trước khi `scenes.content` bị sửa, Command tự động đọc nội dung hiện tại, chép một bản snapshot vào `scene_versions (id, parent_id, version_number, scene_source, content, word_count, created_at)`. Khi người dùng muốn Undo, hệ thống chỉ cần đọc lại phiên bản kề trước từ `scene_versions` để phục hồi nguyên trạng.
   * **Bảng `characters` & `project_characters` (Soft Delete):** Khi xóa một nhân vật khỏi dự án, hệ thống không bao giờ thực hiện `DELETE FROM characters` (gây mồ côi dữ liệu lịch sử). Thay vào đó, Command gán `deleted_at = CURRENT_TIMESTAMP`. Khi Undo, chỉ cần gán lại `deleted_at = NULL`.
3. Tận dụng tính năng **SQLite Savepoints** (`SAVEPOINT sp_name` và `ROLLBACK TO sp_name`) lồng nhau bên trong transaction chung của lượt chạy.

### 3.2. Sơ đồ thực thi Command, Batch Approval & Undo Tree

```text
       LLM Tool Invocations
                |
                v
     +---------------------+
     | RawToolCall [1..N]  |
     +----------+----------+
                | Pack into Commands
                v
+-------------------------------------------------------------+
| ApprovalBatchCommand (batch_id: Uuid)                       |
|  - Command 1: UpdateSceneContent (scene_id, content)        |
|  - Command 2: SoftDeleteCharacter (character_id)            |
|  - Command 3: CreateMotifConstraint (motif_id, syntax)      |
+------------------------------+------------------------------+
                               |
                               v
                       [ UI APPROVAL MODAL ]
            +---------------------------------------+
            | Cmd 1: [ APPROVE ] [ EDIT ARGS ]      |
            | Cmd 2: [ REJECT  ]                    |
            | Cmd 3: [ APPROVE ]                    |
            +-------------------+-------------------+
                                | User confirms batch
                                v
           +-----------------------------------------+
           | Filtered & Edited Commands              |
           |  - Cmd 1 (with edited_args if modified) |
           |  - Cmd 3                                |
           +--------------------+--------------------+
                                |
             +------------------v------------------+
             |    SQLite Transaction (BEGIN)       |
             |                                     |
             | SAVEPOINT sp_cmd_1;                 |
             |  1. Read current scenes.content     |
             |  2. INSERT INTO scene_versions(...) | <--- Snapshot Created
             |  3. UPDATE scenes SET content=...   |
             |                                     |
             | SAVEPOINT sp_cmd_3;                 |
             |  1. Verify motif_constraints        |
             |  2. INSERT INTO motif_constraints   |
             |                                     |
             | If any error / post-check fails:    |
             |    ROLLBACK TO sp_cmd_x;            |
             | If user clicks [ UNDO ]:            |
             |    Call cmd.undo(tx) -> Revert DB   |
             | COMMIT;                             |
             +-------------------------------------+
```

### 3.3. Trait `ReversibleCommand` & Implementation chi tiết trong Rust

```rust
use async_trait::async_trait;
use rusqlite::{params, Connection, Transaction};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(thiserror::Error, Debug)]
pub enum CommandError {
    #[error("Database error: {0}")]
    Db(#[from] rusqlite::Error),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("Entity not found: {0}")]
    NotFound(String),
    #[error("Command execution pre-condition failed: {0}")]
    PreconditionFailed(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExecutionReceipt {
    pub command_id: Uuid,
    pub affected_rows: usize,
    pub snapshot_id: Option<Uuid>,
}

#[async_trait]
pub trait ReversibleCommand: Send + Sync {
    /// Định danh duy nhất của Command
    fn id(&self) -> Uuid;
    /// Tên đại diện cho công cụ (khớp với LLM function name)
    fn tool_name(&self) -> &'static str;
    /// Tên điểm lưu SQLite Savepoint phục vụ rollback cục bộ
    fn savepoint_name(&self) -> String {
        format!("sp_{}", self.id().simple())
    }
    /// Thực thi có khả năng hoàn tác trong phạm vi SQLite Transaction
    async fn execute(&mut self, tx: &Transaction<'_>) -> Result<ExecutionReceipt, CommandError>;
    /// Hoàn tác trạng thái về trước khi execute
    async fn undo(&mut self, tx: &Transaction<'_>) -> Result<(), CommandError>;
}

// ============================================================================
// COMMAND 1: UPDATE SCENE CONTENT (VỚI SNAPSHOT VÀO SCENE_VERSIONS)
// ============================================================================
#[derive(Debug, Clone)]
pub struct UpdateSceneContentCommand {
    pub id: Uuid,
    pub scene_id: Uuid,
    pub new_content: String,
    pub scene_source: String, // e.g., "AI_GENERATED", "AI_EDITED"
    // Lưu trữ nội dung cũ để thực hiện Revert/Undo
    pub previous_content: Option<String>,
    pub previous_word_count: Option<i64>,
    pub generated_version_id: Option<Uuid>,
}

impl UpdateSceneContentCommand {
    pub fn new(scene_id: Uuid, new_content: String, scene_source: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            scene_id,
            new_content,
            scene_source: scene_source.into(),
            previous_content: None,
            previous_word_count: None,
            generated_version_id: None,
        }
    }
}

#[async_trait]
impl ReversibleCommand for UpdateSceneContentCommand {
    fn id(&self) -> Uuid { self.id }
    fn tool_name(&self) -> &'static str { "update_scene_content" }

    async fn execute(&mut self, tx: &Transaction<'_>) -> Result<ExecutionReceipt, CommandError> {
        let sp = self.savepoint_name();
        tx.execute_batch(&format!("SAVEPOINT {sp};"))?;

        // 1. Đọc nội dung hiện tại của scene
        let mut stmt = tx.prepare("SELECT content, word_count FROM scenes WHERE id = ?1")?;
        let (old_content, old_wc): (String, i64) = stmt.query_row(params![self.scene_id.to_string()], |row| {
            Ok((row.get(0)?, row.get(1)?))
        }).map_err(|e| match e {
            rusqlite::Error::QueryReturnedNoRows => CommandError::NotFound(format!("Scene {}", self.scene_id)),
            other => CommandError::Db(other),
        })?;

        self.previous_content = Some(old_content.clone());
        self.previous_word_count = Some(old_wc);

        // 2. Tính toán version_number tiếp theo trong scene_versions
        let next_version: i64 = tx.query_row(
            "SELECT COALESCE(MAX(version_number), 0) + 1 FROM scene_versions WHERE parent_id = ?1",
            params![self.scene_id.to_string()],
            |row| row.get(0),
        )?;

        // 3. Tạo snapshot vào scene_versions (Bảo toàn lịch sử sáng tác)
        let version_id = Uuid::new_v4();
        let word_count = self.new_content.split_whitespace().count() as i64;

        tx.execute(
            r#"
            INSERT INTO scene_versions (
                id, parent_id, version_number, scene_source,
                summary, content, word_count, created_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, CURRENT_TIMESTAMP)
            "#,
            params![
                version_id.to_string(),
                self.scene_id.to_string(),
                next_version,
                self.scene_source,
                "Auto snapshot before AI update",
                old_content, // Lưu lại nội dung TRƯỚC KHI sửa vào lịch sử
                old_wc,
            ],
        )?;

        self.generated_version_id = Some(version_id);

        // 4. Cập nhật scenes với nội dung mới
        let rows = tx.execute(
            "UPDATE scenes SET content = ?1, word_count = ?2 WHERE id = ?3",
            params![self.new_content, word_count, self.scene_id.to_string()],
        )?;

        tx.execute_batch(&format!("RELEASE SAVEPOINT {sp};"))?;

        Ok(ExecutionReceipt {
            command_id: self.id,
            affected_rows: rows,
            snapshot_id: Some(version_id),
        })
    }

    async fn undo(&mut self, tx: &Transaction<'_>) -> Result<(), CommandError> {
        let prev_content = self.previous_content.as_ref()
            .ok_or_else(|| CommandError::PreconditionFailed("No previous state captured for undo".into()))?;
        let prev_wc = self.previous_word_count.unwrap_or(0);

        // 1. Revert nội dung scene về previous_content
        tx.execute(
            "UPDATE scenes SET content = ?1, word_count = ?2 WHERE id = ?3",
            params![prev_content, prev_wc, self.scene_id.to_string()],
        )?;

        // 2. Xóa bản ghi snapshot vừa tạo nếu muốn xóa sạch dấu vết hoặc đánh dấu soft-delete
        if let Some(vid) = self.generated_version_id {
            tx.execute("DELETE FROM scene_versions WHERE id = ?1", params![vid.to_string()])?;
        }

        Ok(())
    }
}

// ============================================================================
// COMMAND 2: SOFT DELETE CHARACTER (SOFT-DELETE QUA DELETED_AT)
// ============================================================================
#[derive(Debug, Clone)]
pub struct SoftDeleteCharacterCommand {
    pub id: Uuid,
    pub project_id: Uuid,
    pub character_id: Uuid,
    pub previous_deleted_at: Option<String>,
}

#[async_trait]
impl ReversibleCommand for SoftDeleteCharacterCommand {
    fn id(&self) -> Uuid { self.id }
    fn tool_name(&self) -> &'static str { "soft_delete_character" }

    async fn execute(&mut self, tx: &Transaction<'_>) -> Result<ExecutionReceipt, CommandError> {
        let sp = self.savepoint_name();
        tx.execute_batch(&format!("SAVEPOINT {sp};"))?;

        // 1. Kiểm tra tồn tại trong bảng nối project_characters
        let exists: bool = tx.query_row(
            "SELECT EXISTS(SELECT 1 FROM project_characters WHERE project_id = ?1 AND character_id = ?2 AND deleted_at IS NULL)",
            params![self.project_id.to_string(), self.character_id.to_string()],
            |row| row.get(0),
        )?;

        if !exists {
            tx.execute_batch(&format!("ROLLBACK TO {sp};"))?;
            return Err(CommandError::NotFound(format!("Character {} not active in project {}", self.character_id, self.project_id)));
        }

        // 2. Soft-delete trong bảng nối project_characters
        let rows = tx.execute(
            "UPDATE project_characters SET deleted_at = CURRENT_TIMESTAMP WHERE project_id = ?1 AND character_id = ?2",
            params![self.project_id.to_string(), self.character_id.to_string()],
        )?;

        tx.execute_batch(&format!("RELEASE SAVEPOINT {sp};"))?;

        Ok(ExecutionReceipt {
            command_id: self.id,
            affected_rows: rows,
            snapshot_id: None,
        })
    }

    async fn undo(&mut self, tx: &Transaction<'_>) -> Result<(), CommandError> {
        // Phục hồi lại liên kết trong project_characters bằng cách set deleted_at = NULL
        tx.execute(
            "UPDATE project_characters SET deleted_at = NULL WHERE project_id = ?1 AND character_id = ?2",
            params![self.project_id.to_string(), self.character_id.to_string()],
        )?;
        Ok(())
    }
}
```

---

## 4. PATTERN 3: CHAIN OF RESPONSIBILITY PATTERN (4-STAGE VERIFICATION PIPELINE)

### 4.1. Kiến trúc Pipeline Xác thực & Vòng phản hồi (Feedback Loop)

Trước khi bất kỳ một tập lệnh nào được thực thi hoặc commit vào SQLite, nó phải vượt qua một chuỗi xác thực gồm 4 tầng độc lập. Thiết kế theo **Chain of Responsibility** phân tách hoàn toàn các mối bận tâm (Separation of Concerns):
1. **Stage 1 - SchemaValidationHandler:** Kiểm tra tính hợp lệ cú pháp JSON Schema (sử dụng metadata từ crate `schemars` và kiểu dữ liệu strong typing của Rust). Loại bỏ ngay các payload thiếu trường hoặc sai định dạng UUID.
2. **Stage 2 - ScopeOwnershipHandler:** Đảm bảo tính cô lập dự án (Tenant/Project Boundary). Mọi entity (`scene_id`, `character_id`, `chapter_id`) mà tool tác động bắt buộc phải thuộc quyền sở hữu của `ctx.project_id`. Đặc biệt kiểm tra bảng liên kết N-N `project_characters`.
3. **Stage 3 - DomainRuleHandler:** Kiểm tra các quy tắc nghiệp vụ văn học quy định trong `docs/db.dbml`:
   * Kiểm tra ràng buộc motif (`motif_constraints` liên kết `motifs` và `constraints`).
   * Kiểm tra tính duy nhất của điểm nhìn POV trong `scene_characters` (`is_pov = true` chỉ được phép gán cho tối đa 1 nhân vật trong cùng một phân cảnh).
   * Kiểm tra thứ tự hợp lệ của `chapters.sort_order` và `scenes.sort_order`.
4. **Stage 4 - PostConditionHandler:** Chạy sau khi các command đã thực thi trong SQLite Savepoint nhưng TRƯỚC KHI commit transaction:
   * Kiểm tra độ dài văn bản không vượt ngưỡng quy định (`word_count`).
   * Kiểm tra tính toàn vẹn của các bảng snapshot `scene_versions`.

```text
               +---------------------------------------------+
               |  ToolCommand Batch from LLM / User Approval |
               +----------------------+----------------------+
                                      |
                                      v
                      [ STAGE 1: SchemaValidation ]
                      - Cú pháp JSON, kiểu dữ liệu,
                        ràng buộc regex UUIDv7.
                                      |
                     +----------------+----------------+
              [PASS] |                                 | [FAIL] -> Verdict::Retry(SchemaError)
                     v                                 |           (Gửi lại LLM sửa cú pháp)
                      [ STAGE 2: ScopeOwnership ]      |
                      - Thực thể thuộc project_id?     |
                      - Tồn tại trong project_chars?   |
                                      |                |
                     +----------------+----------------+
              [PASS] |                                 | [FAIL] -> Verdict::Block(SecurityBreach)
                     v                                 |           (Chặn đứng: LLM truy cập lậu)
                      [ STAGE 3: DomainRule ]          |
                      - Ràng buộc motif_constraints?   |
                      - Duy nhất 1 POV per scene?      |
                      - Thứ tự sort_order hợp lệ?      |
                                      |                |
                     +----------------+----------------+
              [PASS] |                                 | [FAIL] -> Verdict::Retry(LiteraryFeedback)
                     v                                 |           (Gửi phản hồi văn học cho LLM)
             { RUN IN SQLITE SAVEPOINT TRANSACTION }   |
                                      |                |
                                      v                |
                      [ STAGE 4: PostCondition ]       |
                      - Word count invariants          |
                      - Snapshot integrity check       |
                                      |                |
                     +----------------+----------------+
              [PASS] |                                 | [FAIL] -> ROLLBACK SAVEPOINT &
                     v                                             Verdict::Retry(InvariantError)
           [ COMMIT & FINALIZE ]
```

### 4.2. Trait & Enum định nghĩa Pipeline

```rust
use async_trait::async_trait;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationContext {
    pub project_id: Uuid,
    pub run_id: Uuid,
    pub user_id: Option<Uuid>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Verdict {
    /// Vượt qua bài kiểm tra thành công, chuyển tiếp sang mắt xích tiếp theo
    Pass,
    /// Phát hiện lỗi logic/văn học có thể khắc phục được: Yêu cầu LLM điều chỉnh
    Retry {
        stage_name: &'static str,
        structured_feedback: String,
        suggested_correction: Option<serde_json::Value>,
    },
    /// Vi phạm nghiêm trọng (bảo mật, phá hoại dữ liệu): Chặn đứng ngay lập tức
    Block {
        stage_name: &'static str,
        reason: String,
    },
}

#[async_trait]
pub trait VerificationHandler: Send + Sync {
    fn name(&self) -> &'static str;
    async fn verify(
        &self,
        ctx: &VerificationContext,
        command: &ToolCommand,
        conn: &Connection,
    ) -> Result<Verdict, anyhow::Error>;
}

pub struct VerificationPipeline {
    handlers: Vec<Box<dyn VerificationHandler>>,
}

impl VerificationPipeline {
    pub fn new() -> Self {
        Self { handlers: Vec::new() }
    }

    pub fn add_handler(mut self, handler: Box<dyn VerificationHandler>) -> Self {
        self.handlers.push(handler);
        self
    }

    pub async fn execute(
        &self,
        ctx: &VerificationContext,
        command: &ToolCommand,
        conn: &Connection,
    ) -> Result<Verdict, anyhow::Error> {
        for handler in &self.handlers {
            let verdict = handler.verify(ctx, command, conn).await?;
            match verdict {
                Verdict::Pass => continue, // Đi tiếp đến mắt xích kế tiếp
                blocking_or_retry => return Ok(blocking_or_retry), // Dừng chuỗi ngay lập tức
            }
        }
        Ok(Verdict::Pass)
    }
}
```

### 4.3. Cài đặt chi tiết 4 Handler chuẩn mực

```rust
// ============================================================================
// STAGE 1: SCHEMA VALIDATION HANDLER (Kiểm tra JSON Schema với schemars)
// ============================================================================
pub struct SchemaValidationHandler;

#[async_trait]
impl VerificationHandler for SchemaValidationHandler {
    fn name(&self) -> &'static str { "Stage1_SchemaValidation" }

    async fn verify(
        &self,
        _ctx: &VerificationContext,
        command: &ToolCommand,
        _conn: &Connection,
    ) -> Result<Verdict, anyhow::Error> {
        let args = if !command.edited_args.is_null() {
            &command.edited_args
        } else {
            &command.original_args
        };

        match command.tool_name.as_str() {
            "update_scene_content" => {
                let scene_id = args.get("scene_id").and_then(|v| v.as_str());
                let content = args.get("content").and_then(|v| v.as_str());

                if scene_id.is_none() || content.is_none() {
                    return Ok(Verdict::Retry {
                        stage_name: self.name(),
                        structured_feedback: "Missing required fields: 'scene_id' (UUID) and 'content' (string) are mandatory.".into(),
                        suggested_correction: None,
                    });
                }

                if Uuid::parse_str(scene_id.unwrap()).is_err() {
                    return Ok(Verdict::Retry {
                        stage_name: self.name(),
                        structured_feedback: format!("Invalid UUID format for 'scene_id': '{}'.", scene_id.unwrap()),
                        suggested_correction: None,
                    });
                }
            }
            "soft_delete_character" => {
                let char_id = args.get("character_id").and_then(|v| v.as_str());
                if char_id.is_none() || Uuid::parse_str(char_id.unwrap()).is_err() {
                    return Ok(Verdict::Retry {
                        stage_name: self.name(),
                        structured_feedback: "Tool 'soft_delete_character' requires valid UUID 'character_id'.".into(),
                        suggested_correction: None,
                    });
                }
            }
            _ => {}
        }

        Ok(Verdict::Pass)
    }
}

// ============================================================================
// STAGE 2: SCOPE OWNERSHIP HANDLER (Bảo mật phạm vi dự án / Tenant Isolation)
// ============================================================================
pub struct ScopeOwnershipHandler;

#[async_trait]
impl VerificationHandler for ScopeOwnershipHandler {
    fn name(&self) -> &'static str { "Stage2_ScopeOwnership" }

    async fn verify(
        &self,
        ctx: &VerificationContext,
        command: &ToolCommand,
        conn: &Connection,
    ) -> Result<Verdict, anyhow::Error> {
        let args = if !command.edited_args.is_null() { &command.edited_args } else { &command.original_args };

        // 1. Kiểm tra nếu thao tác trên scene: Scene phải thuộc project_id của context
        if let Some(scene_id_str) = args.get("scene_id").and_then(|v| v.as_str()) {
            let scene_id = Uuid::parse_str(scene_id_str)?;
            let belongs_to_project: bool = conn.query_row(
                r#"
                SELECT EXISTS(
                    SELECT 1 FROM scenes s
                    JOIN chapters c ON s.chapter_id = c.id
                    JOIN volumes v ON c.volume_id = v.id
                    WHERE s.id = ?1 AND v.project_id = ?2
                )
                "#,
                params![scene_id.to_string(), ctx.project_id.to_string()],
                |row| row.get(0),
            )?;

            if !belongs_to_project {
                return Ok(Verdict::Block {
                    stage_name: self.name(),
                    reason: format!("Security violation: Scene {} does not belong to Project {}.", scene_id, ctx.project_id),
                });
            }
        }

        // 2. Kiểm tra nếu thao tác trên nhân vật: Phải thuộc bảng nối project_characters
        if let Some(char_id_str) = args.get("character_id").and_then(|v| v.as_str()) {
            let char_id = Uuid::parse_str(char_id_str)?;
            let is_linked: bool = conn.query_row(
                "SELECT EXISTS(SELECT 1 FROM project_characters WHERE project_id = ?1 AND character_id = ?2 AND deleted_at IS NULL)",
                params![ctx.project_id.to_string(), char_id.to_string()],
                |row| row.get(0),
            )?;

            if !is_linked {
                return Ok(Verdict::Block {
                    stage_name: self.name(),
                    reason: format!("Access denied: Character {} is not part of Project {}.", char_id, ctx.project_id),
                });
            }
        }

        Ok(Verdict::Pass)
    }
}

// ============================================================================
// STAGE 3: DOMAIN RULE HANDLER (Quy tắc văn học từ db.dbml)
// ============================================================================
pub struct DomainRuleHandler;

#[async_trait]
impl VerificationHandler for DomainRuleHandler {
    fn name(&self) -> &'static str { "Stage3_DomainRule" }

    async fn verify(
        &self,
        ctx: &VerificationContext,
        command: &ToolCommand,
        conn: &Connection,
    ) -> Result<Verdict, anyhow::Error> {
        let args = if !command.edited_args.is_null() { &command.edited_args } else { &command.original_args };

        // Quy tắc 1: Kiểm tra tính duy nhất của điểm nhìn POV trong scene_characters
        if command.tool_name == "assign_scene_pov" {
            let scene_id = args.get("scene_id").and_then(|v| v.as_str()).unwrap();
            let is_pov = args.get("is_pov").and_then(|v| v.as_bool()).unwrap_or(false);

            if is_pov {
                let existing_pov_count: i64 = conn.query_row(
                    "SELECT COUNT(*) FROM scene_characters WHERE scene_id = ?1 AND is_pov = 1",
                    params![scene_id],
                    |row| row.get(0),
                )?;

                if existing_pov_count >= 1 {
                    return Ok(Verdict::Retry {
                        stage_name: self.name(),
                        structured_feedback: format!(
                            "Literary rule violation: Scene {} already has a primary POV assigned. A scene cannot have multiple active POV characters simultaneously.",
                            scene_id
                        ),
                        suggested_correction: None,
                    });
                }
            }
        }

        // Quy tắc 2: Kiểm tra ràng buộc Motif Constraints (motif_constraints table)
        if command.tool_name == "update_scene_content" {
            let content = args.get("content").and_then(|v| v.as_str()).unwrap_or("");
            
            // Đọc các ràng buộc cấm kỵ (negative constraints) của project
            let mut stmt = conn.prepare(
                r#"
                SELECT c.code, c.require, c.reason
                FROM motif_constraints mc
                JOIN constraints c ON mc.constraint_id = c.id
                JOIN project_motifs pm ON mc.motif_id = pm.motif_id
                WHERE pm.project_id = ?1
                "#
            )?;

            let constraints = stmt.query_map(params![ctx.project_id.to_string()], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?, row.get::<_, String>(2)?))
            })?;

            for item in constraints {
                let (code, requirement, reason) = item?;
                // Ví dụ: Kiểm tra nếu ràng buộc cấm dùng từ ngữ hiện đại trong bối cảnh cổ trang
                if code == "NO_MODERN_SLANG" && (content.contains("smartphone") || content.contains("internet")) {
                    return Ok(Verdict::Retry {
                        stage_name: self.name(),
                        structured_feedback: format!(
                            "Motif Constraint '{}' violated: {}. Found modern slang in ancient setting content.",
                            code, reason
                        ),
                        suggested_correction: None,
                    });
                }
            }
        }

        Ok(Verdict::Pass)
    }
}

// ============================================================================
// STAGE 4: POST-CONDITION HANDLER (Kiểm tra tính toàn vẹn sau khi execute)
// ============================================================================
pub struct PostConditionHandler;

#[async_trait]
impl VerificationHandler for PostConditionHandler {
    fn name(&self) -> &'static str { "Stage4_PostCondition" }

    async fn verify(
        &self,
        _ctx: &VerificationContext,
        command: &ToolCommand,
        conn: &Connection,
    ) -> Result<Verdict, anyhow::Error> {
        let args = if !command.edited_args.is_null() { &command.edited_args } else { &command.original_args };

        if command.tool_name == "update_scene_content" {
            let scene_id = args.get("scene_id").and_then(|v| v.as_str()).unwrap();
            
            // Đảm bảo sau khi cập nhật, word_count phải đồng bộ chuẩn xác với content thực tế
            let (content, word_count): (String, i64) = conn.query_row(
                "SELECT content, word_count FROM scenes WHERE id = ?1",
                params![scene_id],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )?;

            let calculated_wc = content.split_whitespace().count() as i64;
            if calculated_wc != word_count {
                return Ok(Verdict::Retry {
                    stage_name: self.name(),
                    structured_feedback: format!(
                        "Invariant violated: Stored word_count ({}) does not match token length ({}).",
                        word_count, calculated_wc
                    ),
                    suggested_correction: None,
                });
            }
        }

        Ok(Verdict::Pass)
    }
}
```

---

## 5. PATTERN 4: ITERATOR / ASYNC STREAM PATTERN (LLM STREAMING & TOOL ACCUMULATOR)

### 5.1. Thách thức Streaming trong AI Novel Engine

Khi LLM sinh văn bản, giao thức Server-Sent Events (SSE) trả về các đoạn byte rời rạc. Đối với câu trả lời bình thường, các chunk là các từ ngữ tự nhiên (`text_delta`). Tuy nhiên, khi LLM gọi công cụ (`tool_calls`), tham số JSON bị xé nhỏ thành hàng chục mảnh delta:
```json
// Chunk 1: {"tool_calls": [{"index": 0, "id": "call_1", "function": {"name": "update_scene"}}]}
// Chunk 2: {"tool_calls": [{"index": 0, "function": {"arguments": "{\"scene_id\":"}}]}
// Chunk 3: {"tool_calls": [{"index": 0, "function": {"arguments": " \"018f6c44\""}}]}
// Chunk 4: {"tool_calls": [{"index": 0, "function": {"arguments": ", \"content\": \"Đêm tối...\"}"}}]}
```

Nếu không có một bộ lọc chuẩn hóa luồng (Stream Adapter) và một bộ tích lũy trạng thái (Accumulator), hệ thống sẽ gặp các vấn đề:
1. UI bị giật lag nếu bắn từng byte qua Tauri IPC.
2. Không thể parse JSON nếu thử deserialize một chuỗi chưa hoàn chỉnh.
3. Rò rỉ bộ nhớ nếu kết nối mạng bị rớt giữa chừng.

**Giải pháp: Async Stream Adapter + `ToolCallAccumulator`.**
* Chuẩn hóa mọi phản hồi từ bất kỳ nhà cung cấp LLM nào thành một kiểu stream thống nhất trong Rust:
  $$\text{Pin}<\text{Box}<\text{dyn Stream}<\text{Item} = \text{Result}<\text{StreamItem}, \text{LlmError}>> + \text{Send}>>$$
* Tích hợp `ToolCallAccumulator` để gom và ghép các mảnh JSON delta theo `index` của từng tool call, chỉ phát ra `RawToolCall` hoàn chỉnh khi stream báo kết thúc hoặc chuyển sang token khác.

### 5.2. Sơ đồ luồng dữ liệu Async Stream (ASCII Dataflow)

```text
       HTTP SSE Network Stream (bytes)
                      |
                      v
     +----------------------------------+
     | reqwest-eventsource / SSE Parser |
     +----------------+-----------------+
                      | Raw Event Strings
                      v
     +----------------------------------+
     |   Custom Stream Adapter          |
     |   Pin<Box<dyn Stream<...>>>      |
     +----------------+-----------------+
                      | Yields StreamItem
                      |
        +-------------+-------------+
        |                           |
        v (TextDelta)               v (ToolCallDelta)
+---------------+           +-------------------------------+
| Tauri Channel |           | ToolCallAccumulator           |
| (IPC to WebUI)|           |  - index: 0                   |
| 60fps throttle|           |  - name: "update_scene"       |
+---------------+           |  - buffer: "{"scene_id":..."  |
                            +---------------+---------------+
                                            |
                                            | Stream Finished / All Collected
                                            v
                                    +---------------+
                                    |  RawToolCall  | (Valid JSON String)
                                    +---------------+
```

### 5.3. Triển khai Rust Stream Adapter & `ToolCallAccumulator`

```rust
use futures_core::Stream;
use futures_util::{StreamExt, pin_mut};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::pin::Pin;

#[derive(thiserror::Error, Debug)]
pub enum LlmError {
    #[error("Network connection error: {0}")]
    Network(String),
    #[error("Stream interrupted prematurely")]
    PrematureAbort,
    #[error("Malformed JSON in tool argument: {0}")]
    MalformedJson(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum StreamItem {
    TextDelta(String),
    ToolCallDelta {
        index: usize,
        call_id: Option<String>,
        tool_name: Option<String>,
        args_delta: String,
    },
    Finished {
        finish_reason: String,
    },
}

pub type BoxedLlmStream = Pin<Box<dyn Stream<Item = Result<StreamItem, LlmError>> + Send>>;

// ============================================================================
// BỘ TÍCH LŨY THAM SỐ GỌI CÔNG CỤ (TOOL CALL ACCUMULATOR)
// ============================================================================
#[derive(Debug, Default)]
pub struct ToolCallAccumulator {
    in_flight_calls: HashMap<usize, InFlightCall>,
}

#[derive(Debug, Default)]
struct InFlightCall {
    call_id: String,
    tool_name: String,
    arguments_buffer: String,
}

impl ToolCallAccumulator {
    pub fn new() -> Self {
        Self::default()
    }

    /// Tiếp nhận một mảnh delta từ LLM stream
    pub fn ingest_delta(&mut self, index: usize, id: Option<String>, name: Option<String>, delta: String) {
        let entry = self.in_flight_calls.entry(index).or_default();
        if let Some(id) = id {
            entry.call_id = id;
        }
        if let Some(name) = name {
            entry.tool_name = name;
        }
        entry.arguments_buffer.push_str(&delta);
    }

    /// Gom toàn bộ các tool call đã hoàn chỉnh và validate cấu trúc JSON cơ bản
    pub fn finalize(self) -> Result<Vec<RawToolCall>, LlmError> {
        let mut sorted_indices: Vec<_> = self.in_flight_calls.keys().copied().collect();
        sorted_indices.sort_unstable();

        let mut results = Vec::with_capacity(sorted_indices.len());

        for idx in sorted_indices {
            let call = self.in_flight_calls.get(&idx).unwrap();
            
            // Kiểm tra cú pháp JSON sơ bộ bằng cách parse vào serde_json::Value
            serde_json::from_str::<serde_json::Value>(&call.arguments_buffer).map_err(|e| {
                LlmError::MalformedJson(format!(
                    "Tool '{}' received invalid JSON arguments '{}': {}",
                    call.tool_name, call.arguments_buffer, e
                ))
            })?;

            results.push(RawToolCall {
                call_id: call.call_id.clone(),
                tool_name: call.tool_name.clone(),
                raw_arguments: call.arguments_buffer.clone(),
            });
        }

        Ok(results)
    }
}

// ============================================================================
// STREAM CONSUMER: ĐIỀU PHỐI VỪA GỬI UI VỪA TÍCH LŨY TOOL
// ============================================================================
pub struct LlmStreamConsumer;

impl LlmStreamConsumer {
    pub async fn consume_stream(
        stream: BoxedLlmStream,
        ui_token_tx: tokio::sync::mpsc::Sender<String>,
    ) -> Result<(String, Vec<RawToolCall>), LlmError> {
        pin_mut!(stream);

        let mut full_text = String::new();
        let mut accumulator = ToolCallAccumulator::new();

        while let Some(item_res) = stream.next().await {
            let item = item_res?;
            match item {
                StreamItem::TextDelta(token) => {
                    full_text.push_str(&token);
                    // Bắn sang giao diện qua Tauri channel (bỏ qua nếu UI đóng)
                    let _ = ui_token_tx.send(token).await;
                }
                StreamItem::ToolCallDelta { index, call_id, tool_name, args_delta } => {
                    accumulator.ingest_delta(index, call_id, tool_name, args_delta);
                }
                StreamItem::Finished { finish_reason: _ } => {
                    break;
                }
            }
        }

        let completed_tools = accumulator.finalize()?;
        Ok((full_text, completed_tools))
    }
}
```

---

## 6. PATTERN 5: TỔNG HỢP KIẾN TRÚC HỆ THỐNG (ARCHITECTURAL SYNTHESIS)

### 6.1. Sơ đồ phối hợp tổng thể trong một lượt chạy (Turn Lifecycle)

Sơ đồ tuần tự dưới đây minh họa sự tương tác đồng bộ giữa 4 Pattern bên trong Backend Tauri của Merlink:

```text
[TAURI UI IPC]        [IMPERATIVE SHELL]        [SANS-I/O STATE]     [VERIFICATION PIPELINE]     [COMMAND & SQLITE]
      |                       |                        |                         |                       |
      | 1. invoke("start_run")|                        |                         |                       |
      +---------------------->|                        |                         |                       |
      |                       | 2. transition(StartRun)|                         |                       |
      |                       +----------------------->|                         |                       |
      |                       | 3. (Phase, [LoadContext])                        |                       |
      |                       |<-----------------------+                         |                       |
      |                       |                                                  |                       |
      |                       | 4. Read DB -> ContextBuilt                       |                       |
      |                       +----------------------->|                         |                       |
      |                       | 5. [InitiateLlmStream] |                         |                       |
      |                       |<-----------------------+                         |                       |
      |                       |                                                  |                       |
      |                       | 6. Async Stream Item (Text & Tool Delta)         |                       |
      | 7. emit(Token)        |    Accumulate via ToolCallAccumulator            |                       |
      |<----------------------+                                                  |                       |
      |                       |                                                  |                       |
      |                       | 8. StreamCompleted -> transition                 |                       |
      |                       +----------------------->|                         |                       |
      |                       | 9. [EmitApprovalRequest]                         |                       |
      | 10. Open Batch Modal  |<-----------------------+                         |                       |
      |<----------------------+                                                  |                       |
      |                       |                                                  |                       |
      | 11. User Edits & OK   |                                                  |                       |
      +---------------------->|                                                  |                       |
      |                       | 12. BatchApproved -> transition                  |                       |
      |                       +----------------------->|                         |                       |
      |                       | 13. [RunPreVerificationPipeline]                 |                       |
      |                       |<-----------------------+                         |                       |
      |                       |                                                  |                       |
      |                       | 14. Execute 4 Stages                             |                       |
      |                       +------------------------------------------------->|                       |
      |                       |     Stage 1: SchemaValidation                    |                       |
      |                       |     Stage 2: ScopeOwnership                      |                       |
      |                       |     Stage 3: DomainRule (POV, Motif)             |                       |
      |                       | 15. Verdict::Pass                                |                       |
      |                       |<-------------------------------------------------+                       |
      |                       |                                                  |                       |
      |                       | 16. PreVerificationPassed -> transition          |                       |
      |                       +----------------------->|                         |                       |
      |                       | 17. [ExecuteToolBatch] |                         |                       |
      |                       |<-----------------------+                         |                       |
      |                       |                                                                          |
      |                       | 18. Savepoint -> Snapshot scene_versions -> UPDATE scenes ------------->|
      |                       |                                                                          |
      |                       | 19. RunPostVerificationPipeline                                          |
      |                       +------------------------------------------------->|                       |
      |                       |     Stage 4: PostCondition (WordCount Invariant) |                       |
      |                       | 20. Verdict::Pass                                |                       |
      |                       |<-------------------------------------------------+                       |
      |                       |                                                                          |
      |                       | 21. CommitTransactionAndPersist ---------------------------------------->|
      | 22. emit("run_done")  |                                                                          |
      |<----------------------+                                                                          |
```

### 6.2. Cấu trúc thư mục module đề xuất (`src-tauri/src/agent/`)

Để đảm bảo khả năng bảo trì, mở rộng và tuân thủ nguyên tắc Clean Architecture, mã nguồn Rust của Agent Engine được tổ chức như sau:

```text
src-tauri/src/agent/
├── mod.rs                     # Entrypoint module export
├── state/
│   ├── mod.rs
│   ├── phase.rs               # Enum RunPhase & RunContext
│   ├── event.rs               # Enum RunEvent
│   ├── command.rs             # Enum DriverCommand
│   └── machine.rs             # Pure transition() function & unit tests (Sans-I/O)
├── commands/
│   ├── mod.rs                 # Trait ReversibleCommand & ExecutionReceipt
│   ├── batch.rs               # ApprovalBatchCommand & HitL editing logic
│   ├── scene_content.rs       # UpdateSceneContentCommand (Snapshotting scene_versions)
│   ├── character.rs           # SoftDeleteCharacterCommand (Soft-delete project_characters)
│   └── motif.rs               # AttachMotifConstraintCommand
├── verification/
│   ├── mod.rs                 # Trait VerificationHandler, Verdict & Pipeline
│   ├── schema.rs              # Stage 1: SchemaValidationHandler (schemars)
│   ├── ownership.rs           # Stage 2: ScopeOwnershipHandler (Tenant isolation)
│   ├── literary_rules.rs      # Stage 3: DomainRuleHandler (POV & Motif constraints)
│   └── post_invariants.rs     # Stage 4: PostConditionHandler (WordCount & DB consistency)
├── stream/
│   ├── mod.rs
│   ├── accumulator.rs         # ToolCallAccumulator (Streaming JSON delta stitcher)
│   └── adapter.rs             # BoxedLlmStream & SSE conversion
└── shell/
    ├── mod.rs
    ├── runner.rs              # Imperative Shell Async Loop (Tokio runner)
    └── tauri_emitter.rs       # IPC Event Bridge nối với Svelte/React Frontend
```

### 6.3. Ma trận so sánh & Trade-off Phân tích

| Tiêu chí | Tiếp cận OOP Kinh điển (Dynamic Dispatch) | Tiếp cận Rust Bản địa (Sans-I/O + ADT + Stream) | Lợi ích trong Merlink Engine |
| :--- | :--- | :--- | :--- |
| **Cấp phát bộ nhớ** | Nặng: Heap allocation liên tục (`Box<dyn State>`, `Box<dyn Command>`). | Cực nhẹ: Hầu hết nằm trên Stack qua Enum kích thước cố định (`RunPhase`). | Không gây phân mảnh bộ nhớ khi chạy lâu trên máy trạm của tác giả. |
| **Tính an toàn luồng** | Dễ deadlock khi dùng `Arc<Mutex<State>>` giữa các luồng Tokio. | Tuyệt đối an toàn: Single Ownership trong Core, shell trao đổi qua Channels (`mpsc`). | Không bao giờ xảy ra Race Condition khi vừa stream vừa bấm nút Hủy. |
| **Khả năng kiểm thử** | Phức tạp: Phải mock toàn bộ Database connection và HTTP Client. | Đơn giản: Test hàm `transition` thuần túy bằng unit test không mock. | Tốc độ chạy CI/CD siêu tốc, bao phủ 100% case biên mà không cần DB rác. |
| **Kiểm soát đột biến dữ liệu** | Khó khăn: Lệnh gọi tool thường ghi trực tiếp vào DB, khó hoàn tác. | Toàn diện: Command Pattern lưu `scene_versions` snapshot + SQLite savepoints. | Tác giả an tâm để AI sửa truyện vì luôn bấm [UNDO] được ngay lập tức. |
| **Bảo vệ toàn vẹn văn học** | Phân mảnh: Code kiểm tra rule rải rác trong các hàm controller. | Tập trung: 4-Stage Verification Pipeline chặn đứng từ schema tới motif. | Đảm bảo tiểu thuyết không bao giờ bị đa điểm nhìn (loạn POV) hay lệch motif. |

---

## 7. KẾT LUẬN & KHUYẾN NGHỊ TRIỂN KHAI

Báo cáo này đã thiết lập nền tảng lý thuyết và mã nguồn mẫu chuẩn xác cho **Hệ thống Hành vi (Behavioral Patterns Phần 1)** của Merlink Novel Agent:
1. **Sans-I/O State Pattern** đảm bảo khả năng quản lý vòng đời Agent Turn hoàn toàn tất định, loại bỏ race conditions và giúp test logic dễ dàng.
2. **Reversible Command Pattern** gắn kết chặt chẽ với cơ sở dữ liệu `db.dbml`, khai thác triệt để sức mạnh của `scene_versions` cho snapshot rollback và `deleted_at` cho soft delete, mang lại trải nghiệm HitL Batch Approval mượt mà cho tác giả.
3. **Chain of Responsibility Pattern** tạo thành một lá chắn 4 lớp kiên cố, vừa bảo vệ tính an toàn dữ liệu, vừa duy trì tính logic nghệ thuật của tác phẩm văn học.
4. **Iterator / Async Stream Pattern** giải quyết triệt để vấn đề phân mảnh JSON của LLM SSE Stream, mang lại trải nghiệm hiển thị 60fps mượt mà trên frontend.

**Khuyến nghị bước tiếp theo:**
Đưa toàn bộ thiết kế này vào pha lập trình mã nguồn chính thức tại nhánh backend `merlink-core-agent`, tích hợp trực tiếp vào module IPC Tauri v2 để kiểm thử thực tế với các model LLM hàng đầu.
