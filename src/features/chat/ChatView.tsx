import { useState, useRef, useEffect, useMemo } from "react";
import { GripVertical } from "lucide-react";
import { useNavigate } from "react-router";
import { MOCK_PROJECTS } from "@/features/projects/mockProjects";
import { buildProjectPath } from "@/features/draft/commitNavigation";
import { cn } from "@/shared/utils/cn";
import { useResizablePanel } from "@/shared/hooks/useResizablePanel";
import { useIsCompact } from "@/shared/hooks/useMediaQuery";
import { ChatHeader } from "./ChatHeader";
import { ChatWelcome } from "./ChatWelcome";
import { ChatInputBar } from "./ChatInputBar";
import { ChatMessageItem } from "./ChatMessageItem";
import { DraftWorkspace } from "@/features/draft/DraftWorkspace";
import { FloatingOutline } from "@/shared/components/ui/floating-outline";
import { PreDraftSuggestionDialog } from "./PreDraftSuggestionDialog";
import {
  SAMPLE_BUSINESS_FLOW_DATA,
  calculateBusinessFlowProgress,
  isRequireComplete,
} from "@/features/business-flow";
import type { ChatMessage } from "./types";
import type { FeedbackContext } from "@/features/draft/types";
import { useT } from "@/shared/i18n/useT";

export default function ChatView() {
  const t = useT();
  const isCompact = useIsCompact();
  const navigate = useNavigate();
  const [isDraftOpen, setIsDraftOpen] = useState(false);
  const sampleProjects = useMemo(() => MOCK_PROJECTS.map((p) => ({ id: p.id, title: p.title })), []);
  // Wave 0.3B: explicit, simulated preview transition. Not a real commit; no project is created.
  const handleViewSampleProject = (projectId: string) => {
    setIsDraftOpen(false);
    navigate(buildProjectPath(projectId));
  };
  const [isSuggestionOpen, setIsSuggestionOpen] = useState(false);
  const [feedbackContext, setFeedbackContext] = useState<FeedbackContext | null>(null);
  const [messages, setMessages] = useState<ChatMessage[]>([]);
  const [keyboardRatio, setKeyboardRatio] = useState<number | null>(null);
  const scrollBottomRef = useRef<HTMLDivElement>(null);
  const scrollContainerRef = useRef<HTMLDivElement>(null);

  const businessFlowData = SAMPLE_BUSINESS_FLOW_DATA;
  const progressResult = useMemo(
    () => calculateBusinessFlowProgress(businessFlowData),
    [businessFlowData]
  );
  const progressPercent = progressResult.percent;
  const requireComplete = useMemo(
    () => isRequireComplete(businessFlowData),
    [businessFlowData]
  );

  const handleToggleDraft = () => {
    if (isDraftOpen) {
      setIsDraftOpen(false);
    } else {
      setIsSuggestionOpen(true);
    }
  };

  const handleSuggestionConfirm = (suggestion?: string) => {
    setIsSuggestionOpen(false);
    setIsDraftOpen(true);
    if (suggestion && suggestion.trim()) {
      const userMsg: ChatMessage = {
        id: `usr-${Date.now()}`,
        role: "user",
        content: suggestion.trim(),
        timestamp: "Vừa xong",
      };
      setMessages((prev) => [...prev, userMsg]);
    }
  };

  const { leftRatio, isDragging, containerRef, startDragging } = useResizablePanel({
    initialRatio: 50,
    minPxLeft: 545,
    minPxRight: 530,
  });

  const splitRatio = keyboardRatio ?? leftRatio;

  // Tự động cuộn xuống đáy khi có message mới hoặc đang streaming
  useEffect(() => {
    scrollBottomRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [messages]);

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
      thinkingSeconds: 1,
      thinkingDetails: "Khởi động mô hình và phân tích ngữ cảnh đầu vào...",
      timestamp: "Vừa xong",
    };

    setMessages((prev) => [...prev, userMsg, placeholderAssistant]);

    // Các mẩu log suy nghĩ stream theo thời gian
    const thinkingSteps = [
      "Kiểm tra các thực thể (Worldbuilding, Arcs, Characters) hiện có trong workspace...",
      "Truy vấn thông số cấu hình và định mức mục tiêu văn bản...",
      "Phát hiện thiếu hụt các tham số cốt lõi. Khởi tạo câu hỏi định hướng cho người dùng...",
    ];

    let currentSeconds = 1;
    let stepIdx = 0;

    // Timer đếm giây và streaming thinking logs
    const thinkInterval = setInterval(() => {
      currentSeconds += 1;
      const currentDetail = thinkingSteps.slice(0, stepIdx + 1).join("\n");

      setMessages((prev) =>
        prev.map((msg) =>
          msg.id === tempAssistantId
            ? {
                ...msg,
                thinkingSeconds: currentSeconds,
                thinkingDetails: currentDetail,
              }
            : msg
        )
      );

      stepIdx += 1;

      // Khi suy nghĩ đủ 4 giây thì kết thúc thinking và stream markdown response
      if (currentSeconds >= 4) {
        clearInterval(thinkInterval);

        const finalResponse = `Project đã tạo xong. Bạn đã đặt tổng số chữ là 500. Còn thiếu một số quyết định cốt lõi. Cho mình hỏi:

1. **Tiêu đề** của truyện là gì (nếu chưa đặt tên, để trống cũng được)?
2. **Ngôn ngữ** viết truyện? (VI / EN / JA / ZH_HANT / ZH_HANS)
3. **Dàn bài**: bao nhiêu arc, chương, scene? (ví dụ: "1 arc, 1 chương, 3 scene"; với 500 chữ thì thường 1 arc, 1 chương là đủ)`;

        // Kết thúc thinking, bắt đầu stream nội dung phản hồi
        setMessages((prev) =>
          prev.map((msg) =>
            msg.id === tempAssistantId
              ? {
                  ...msg,
                  isThinking: false,
                  thinkingSeconds: currentSeconds,
                  thinkingDetails: thinkingSteps.join("\n"),
                  content: finalResponse,
                  toolCalls: [
                    {
                      id: "tool-1",
                      name: "create_project",
                      parameters: { targetWords: 500, prompt: text },
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
                }
              : msg
          )
        );
      }
    }, 1000);
  };

  const scrollToMessage = (id: string) => {
    const el = document.getElementById(id);
    if (el) el.scrollIntoView({ behavior: 'smooth', block: 'start' });
  };

  const outlineItems = useMemo(
    () =>
      messages
        .filter((m) => m.role === "user")
        .map((m) => ({
          id: m.id,
          title: m.content || "Message",
        })),
    [messages]
  );

  return (
    <div
      ref={containerRef}
      className={cn(
        "relative flex h-full w-full overflow-hidden rounded-shell-sm bg-transparent",
        isDragging && "cursor-col-resize select-none"
      )}
    >
      {/* Cột trái: Chat Area */}
      <div
        style={{ width: isDraftOpen && !isCompact ? `${splitRatio}%` : "100%" }}
        className="flex flex-col overflow-hidden "
      >
        <ChatHeader
          isDraftOpen={isDraftOpen}
          onToggleDraft={handleToggleDraft}
          progressPercent={progressPercent}
          isRequireComplete={requireComplete}
          businessFlowData={businessFlowData}
        />

        <div className="relative flex-1 overflow-hidden">
          <div ref={scrollContainerRef} className="no-scrollbar h-full overflow-y-auto px-6 py-4 space-y-4">
            {messages.length === 0 ? (
              <ChatWelcome />
            ) : (
              messages.map((m) => <ChatMessageItem key={m.id} message={m} />)
            )}
            <div ref={scrollBottomRef} />
          </div>
          
          {/* Thanh Outline (Minimap) - Nằm cố định bên trong khung chứa tin nhắn */}
          {messages.length > 0 && (
            <FloatingOutline 
              items={outlineItems} 
              containerRef={scrollContainerRef}
              onItemClick={scrollToMessage} 
            />
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
      {isDraftOpen && !isCompact && (
        <div
          role="separator"
          tabIndex={0}
          aria-label={t("chat.resizePanels")}
          aria-orientation="vertical"
          aria-valuenow={Math.round(splitRatio)}
          aria-valuemin={10}
          aria-valuemax={90}
          onMouseDown={(event) => {
            setKeyboardRatio(null);
            startDragging(event);
          }}
          onKeyDown={(event) => {
            if (event.key === "ArrowLeft" || event.key === "ArrowRight") {
              event.preventDefault();
              const adjustment = event.key === "ArrowLeft" ? -5 : 5;
              setKeyboardRatio(Math.min(90, Math.max(10, splitRatio + adjustment)));
            }
          }}
          className="focus-ring group relative flex w-2 shrink-0 cursor-col-resize items-center justify-center bg-transparent transition-colors hover:bg-accent-soft"
        >
          <div className="h-full w-px bg-line transition-colors group-hover:bg-accent" />
          <div
            className={cn(
              "absolute z-30 flex h-8 w-4.5 items-center justify-center rounded-md tier-card transition-opacity",
              isDragging ? "opacity-100 ring-2 ring-blue-500" : "opacity-0 group-hover:opacity-100"
            )}
          >
            <GripVertical aria-hidden="true" className="h-3.5 w-3.5 text-fg-subtle" />
          </div>
        </div>
      )}

      {/* Cột phải: Draft Building Workspace */}
      {isDraftOpen && (
        <div
          style={isCompact ? undefined : { width: `${100 - splitRatio}%` }}
          className={cn(
            "flex flex-col overflow-hidden",
            isCompact && "absolute inset-0 z-40 bg-surface-panel",
          )}
        >
          <DraftWorkspace
            onClose={() => setIsDraftOpen(false)}
            onFeedback={(feedback) => setFeedbackContext(feedback)}
            sampleProjects={sampleProjects}
            onViewSampleProject={handleViewSampleProject}
          />
        </div>
      )}

      <PreDraftSuggestionDialog
        open={isSuggestionOpen}
        onOpenChange={setIsSuggestionOpen}
        onConfirm={handleSuggestionConfirm}
      />
    </div>
  );
}
