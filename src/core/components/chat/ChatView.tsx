import { useState } from "react";
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

  return (
    <div className="flex h-full w-full overflow-hidden">
      {/* Cột trái: Chat Area */}
      <div className="flex flex-1 flex-col overflow-hidden bg-slate-50/50 dark:bg-slate-950/20">
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

      {/* Cột phải: Draft Building Workspace */}
      {isDraftOpen && (
        <div className="flex flex-1 flex-col overflow-hidden">
          <DraftWorkspace
            onClose={() => setIsDraftOpen(false)}
            onFeedback={(feedback) => setFeedbackContext(feedback)}
          />
        </div>
      )}
    </div>
  );
}
