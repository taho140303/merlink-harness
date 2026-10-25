import { useState } from "react";
import { GripVertical } from "lucide-react";
import { cn } from "../../../shared/utils/cn";
import { useResizablePanel } from "../../../shared/hooks/useResizablePanel";
import { ChatHeader } from "./ChatHeader";
import { ChatWelcome } from "./ChatWelcome";
import { ChatInputBar } from "./ChatInputBar";
import { ChatMessageItem } from "./ChatMessageItem";
import { DraftWorkspace } from "../draft/DraftWorkspace";
import type { ChatMode, ChatMessage } from "./types";
import type { FeedbackContext } from "../draft/types";

const INITIAL_MESSAGES: ChatMessage[] = [
  {
    id: "msg-sample",
    role: "assistant",
    content: `Project đã tạo xong. Bạn đã đặt tổng số chữ là 500. Còn thiếu một số quyết định cốt lõi. Cho mình hỏi:

1. **Tiêu đề** của truyện là gì (nếu chưa đặt tên, để trống cũng được)?
2. **Ngôn ngữ** viết truyện? (VI / EN / JA / ZH_HANT / ZH_HANS)
3. **Dàn bài**: bao nhiêu arc, chương, scene? (ví dụ: "1 arc, 1 chương, 3 scene"; với 500 chữ thì thường 1 arc, 1 chương là đủ)`,
    thinkingSeconds: 7,
    thinkingDetails: "Xác thực cấu hình dự án, kiểm tra định mức từ mục tiêu (500 chữ) và khởi tạo danh mục câu hỏi cốt lõi.",
    timestamp: "18:04",
    toolCalls: [
      {
        id: "tool-1",
        name: "create_project",
        parameters: { targetWords: 500, mode: "fast_draft" },
        result: { status: "created", projectId: "proj-sample-500" },
        status: "success",
      },
      {
        id: "tool-2",
        name: "schema_audit",
        parameters: { requireMeta: ["title", "language", "structure"] },
        result: { missingFields: ["title", "language", "outline"] },
        status: "success",
      },
    ],
  },
];

export default function ChatView() {
  const [mode, setMode] = useState<ChatMode>("chat");
  const [isDraftOpen, setIsDraftOpen] = useState(false);
  const [feedbackContext, setFeedbackContext] = useState<FeedbackContext | null>(null);
  const [messages, setMessages] = useState<ChatMessage[]>(INITIAL_MESSAGES);

  const { leftRatio, isDragging, containerRef, startDragging } = useResizablePanel({
    initialRatio: 50,
    minPx: 360,
  });

  const handleSendMessage = (text: string) => {
    const userMsg: ChatMessage = {
      id: `usr-${Date.now()}`,
      role: "user",
      content: text,
      timestamp: "Vừa xong",
    };

    const tempAssistantId = `ast-${Date.now()}`;
    const placeholderAssistant: ChatMessage = {
      id: tempAssistantId,
      role: "assistant",
      content: "",
      isThinking: true,
      timestamp: "Vừa xong",
    };

    setMessages((prev) => [...prev, userMsg, placeholderAssistant]);

    // Giả lập thinking 3 giây rồi nạp markdown response kèm tool calls
    setTimeout(() => {
      setMessages((prev) =>
        prev.map((msg) =>
          msg.id === tempAssistantId
            ? {
                ...msg,
                isThinking: false,
                thinkingSeconds: 3,
                thinkingDetails: "Phân tích yêu cầu kịch bản, tra cứu cơ sở dữ liệu các lore hiện có.",
                content: `Đã ghi nhận yêu cầu: "${text}".\n\n1. **Khởi tạo dữ liệu**: Bản nháp đã được lưu vào workspace.\n2. **Đề xuất kế tiếp**: Bạn có thể mở **Draft building** ở góc trên để tùy biến chi tiết các arc và nhân vật.`,
                toolCalls: [
                  {
                    id: `tool-${Date.now()}`,
                    name: "sync_workspace_state",
                    parameters: { input: text },
                    result: { success: true, changesApplied: 1 },
                    status: "success",
                  },
                ],
              }
            : msg
        )
      );
    }, 3000);
  };

  return (
    <div
      ref={containerRef}
      className={cn(
        "flex h-full w-full overflow-hidden",
        isDragging && "cursor-col-resize select-none"
      )}
    >
      {/* Cột trái: Chat Area */}
      <div
        style={{ width: isDraftOpen ? `${leftRatio}%` : "100%" }}
        className="flex flex-col overflow-hidden bg-slate-50/50 dark:bg-slate-950/20"
      >
        <ChatHeader
          mode={mode}
          onModeChange={setMode}
          isDraftOpen={isDraftOpen}
          onToggleDraft={() => setIsDraftOpen(!isDraftOpen)}
        />

        <div className="flex-1 overflow-y-auto px-6 py-4 space-y-4">
          {messages.length === 0 ? (
            <ChatWelcome />
          ) : (
            messages.map((m) => <ChatMessageItem key={m.id} message={m} />)
          )}
        </div>

        <div className="shrink-0 p-6">
          <ChatInputBar
            feedbackContext={feedbackContext}
            onClearFeedback={() => setFeedbackContext(null)}
            onSend={handleSendMessage}
          />
        </div>
      </div>

      {/* Ranh giới kéo thả */}
      {isDraftOpen && (
        <div
          onMouseDown={startDragging}
          className="group relative flex w-2 shrink-0 cursor-col-resize items-center justify-center bg-transparent transition-colors hover:bg-blue-500/10 dark:hover:bg-blue-500/20"
        >
          <div className="h-full w-px bg-slate-200 transition-colors group-hover:bg-blue-500 dark:bg-slate-800" />
          <div
            className={cn(
              "absolute z-30 flex h-8 w-4.5 items-center justify-center rounded-md border border-slate-300 bg-white shadow-xs transition-opacity dark:border-slate-700 dark:bg-slate-800",
              isDragging ? "opacity-100 ring-2 ring-blue-500" : "opacity-0 group-hover:opacity-100"
            )}
          >
            <GripVertical className="h-3.5 w-3.5 text-slate-500 dark:text-slate-400" />
          </div>
        </div>
      )}

      {/* Cột phải: Draft Building Workspace */}
      {isDraftOpen && (
        <div
          style={{ width: `${100 - leftRatio}%` }}
          className="flex flex-col overflow-hidden"
        >
          <DraftWorkspace
            onClose={() => setIsDraftOpen(false)}
            onFeedback={(feedback) => setFeedbackContext(feedback)}
          />
        </div>
      )}
    </div>
  );
}
