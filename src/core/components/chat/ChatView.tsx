import { useState } from "react";
import { GripVertical } from "lucide-react";
import { cn } from "../../../shared/utils/cn";
import { useResizablePanel } from "../../../shared/hooks/useResizablePanel";
import { ChatHeader } from "./ChatHeader";
import { ChatWelcome } from "./ChatWelcome";
import { ChatInputBar } from "./ChatInputBar";
import { DraftWorkspace } from "../draft/DraftWorkspace";
import type { ChatMode } from "./types";
import type { FeedbackContext } from "../draft/types";

export default function ChatView() {
  const [mode, setMode] = useState<ChatMode>("chat");
  const [isDraftOpen, setIsDraftOpen] = useState(false);
  const [feedbackContext, setFeedbackContext] = useState<FeedbackContext | null>(null);

  const { leftRatio, isDragging, containerRef, startDragging } = useResizablePanel({
    initialRatio: 50,
    minPx: 360,
  });

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

        <div className="flex-1 overflow-y-auto px-6">
          <ChatWelcome />
        </div>

        <div className="shrink-0 p-6">
          <ChatInputBar
            feedbackContext={feedbackContext}
            onClearFeedback={() => setFeedbackContext(null)}
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
