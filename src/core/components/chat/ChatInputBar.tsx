import { useState, useRef, useEffect } from "react";
import { Sparkles, ChevronDown, Paperclip, ArrowUp, X, MessageSquareQuote } from "lucide-react";
import { ContextUsagePopover } from "./ContextUsagePopover";
import { ModelSelectorPopover } from "./ModelSelectorPopover";
import type { FeedbackContext } from "../draft/types";

interface ChatInputBarProps {
  feedbackContext?: FeedbackContext | null;
  onClearFeedback?: () => void;
}

export function ChatInputBar({ feedbackContext, onClearFeedback }: ChatInputBarProps) {
  const [prompt, setPrompt] = useState("");
  const inputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    if (feedbackContext) {
      inputRef.current?.focus();
    }
  }, [feedbackContext]);

  return (
    <div className="mx-auto w-full max-w-4xl rounded-3xl border border-slate-200 bg-white p-3 shadow-lg shadow-slate-200/40 dark:border-slate-800 dark:bg-slate-900 dark:shadow-none transition-all">
      {/* Component đính kèm khi ấn "Góp ý & sửa đổi" */}
      {feedbackContext && (
        <div className="mb-2 flex items-center justify-between rounded-xl border border-blue-200 bg-blue-50/70 px-3 py-1.5 text-xs text-blue-900 dark:border-blue-900/60 dark:bg-blue-950/40 dark:text-blue-200">
          <div className="flex items-center gap-2 truncate">
            <MessageSquareQuote className="h-4 w-4 shrink-0 text-blue-600 dark:text-blue-400" />
            <span className="rounded-md bg-blue-100 px-1.5 py-0.5 text-[10px] font-semibold uppercase text-blue-700 dark:bg-blue-900 dark:text-blue-300">
              {feedbackContext.tag}
            </span>
            <span className="truncate font-medium">{feedbackContext.title}</span>
          </div>

          <button
            type="button"
            onClick={onClearFeedback}
            className="rounded-md p-1 hover:bg-blue-100 dark:hover:bg-blue-900 transition-colors"
          >
            <X className="h-3.5 w-3.5" />
          </button>
        </div>
      )}

      <div className="flex items-center gap-2 px-2 pt-1 text-slate-400">
        <Sparkles className="h-4 w-4" />
        <input
          ref={inputRef}
          type="text"
          value={prompt}
          onChange={(e) => setPrompt(e.target.value)}
          placeholder={
            feedbackContext
              ? `Nhập nội dung chỉnh sửa hoặc góp ý cho [${feedbackContext.title}]...`
              : "Ask me anything..."
          }
          className="w-full bg-transparent text-sm text-slate-800 placeholder:text-slate-400 focus:outline-hidden dark:text-slate-100"
        />
      </div>

      <div className="mt-3 flex items-center justify-between border-t border-slate-100 pt-2.5 dark:border-slate-800/80">
        <div className="flex items-center gap-2">
          <button
            type="button"
            className="flex items-center gap-1 rounded-xl px-2.5 py-1.5 text-xs font-medium text-slate-500 hover:bg-slate-100 hover:text-slate-800 dark:text-slate-400 dark:hover:bg-slate-800 dark:hover:text-slate-200 transition-colors"
          >
            <Paperclip className="h-3.5 w-3.5" />
            <span>Attach</span>
          </button>

          <button
            type="button"
            className="flex items-center gap-1.5 rounded-xl border border-slate-200 bg-slate-50 px-3 py-1.5 text-xs font-medium text-slate-700 hover:bg-slate-100 dark:border-slate-700 dark:bg-slate-800 dark:text-slate-300 dark:hover:bg-slate-750 transition-colors"
          >
            <span>Select Source</span>
            <ChevronDown className="h-3 w-3 text-slate-400" />
          </button>
        </div>

        <div className="flex items-center gap-2">
          <ModelSelectorPopover />
          <ContextUsagePopover />
          <button
            type="button"
            className="flex h-8 w-8 items-center justify-center rounded-xl bg-slate-900 text-white shadow-xs hover:bg-slate-800 dark:bg-slate-100 dark:text-slate-900 dark:hover:bg-slate-200 transition-colors"
            title="Send"
          >
            <ArrowUp className="h-4 w-4" />
          </button>
        </div>
      </div>
    </div>
  );
}
