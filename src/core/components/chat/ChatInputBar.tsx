import { useState, useRef, useEffect } from "react";
import { Button } from "../../../shared/components/ui/button";
import { Sparkles, ChevronDown, Paperclip, ArrowUp, X, MessageSquareQuote } from "lucide-react";
import { ContextUsagePopover } from "./ContextUsagePopover";
import { ModelSelectorPopover } from "./ModelSelectorPopover";
import { useT } from "../../../shared/i18n/useT";
import type { FeedbackContext } from "../draft/types";

interface ChatInputBarProps {
  feedbackContext?: FeedbackContext | null;
  onClearFeedback?: () => void;
  onSend?: (text: string) => void;
}

export function ChatInputBar({
  feedbackContext,
  onClearFeedback,
  onSend,
}: ChatInputBarProps) {
  const t = useT();
  const [prompt, setPrompt] = useState("");
  const inputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    if (feedbackContext) {
      inputRef.current?.focus();
    }
  }, [feedbackContext]);

  const handleSend = () => {
    if (!prompt.trim()) return;
    onSend?.(prompt);
    setPrompt("");
    onClearFeedback?.();
  };

  return (
    <div className="tier-panel mx-auto w-full max-w-4xl rounded-panel p-3 transition-colors">
      {feedbackContext && (
        <div className="mb-2 flex items-center justify-between rounded-xl border border-blue-200 bg-blue-50/70 px-3 py-1.5 text-xs text-blue-900 dark:border-blue-900/60 dark:bg-blue-950/40 dark:text-blue-200">
          <div className="flex items-center gap-2 truncate">
            <MessageSquareQuote className="h-4 w-4 shrink-0 text-blue-600 dark:text-blue-400" />
            <span className="rounded-md bg-blue-100 px-1.5 py-0.5 text-xs font-semibold uppercase text-blue-700 dark:bg-blue-900 dark:text-blue-300">
              {feedbackContext.tag}
            </span>
            <span className="truncate font-medium">{feedbackContext.title}</span>
          </div>
          <button
            type="button"
            onClick={onClearFeedback}
            aria-label="Clear feedback context"
            className="focus-ring rounded-md p-1 hover:bg-blue-100 dark:hover:bg-blue-900 transition-colors"
          >
            <X aria-hidden="true" className="h-3.5 w-3.5" />
          </button>
        </div>
      )}

      <div className="flex items-center gap-2 px-2 pt-1 text-fg-subtle">
        <Sparkles className="h-4 w-4" />
        <input
          ref={inputRef}
          aria-label="Message"
          type="text"
          value={prompt}
          onChange={(e) => setPrompt(e.target.value)}
          onKeyDown={(e) => {
            if (e.key === "Enter") {
              e.preventDefault();
              handleSend();
            }
          }}
          placeholder={
            feedbackContext
              ? `Nhập nội dung chỉnh sửa hoặc góp ý cho [${feedbackContext.title}]...`
              : "Ask me anything..."
          }
          className="tier-input focus-ring w-full rounded-xl bg-transparent px-3 py-2 text-sm text-fg placeholder:text-fg-subtle"
        />
      </div>

      <div className="mt-3 flex items-center justify-between border-t border-line pt-2.5">
        <div className="flex items-center gap-2">
          <button
            type="button"
            className="focus-ring flex items-center gap-1 rounded-xl px-2.5 py-1.5 text-xs font-medium text-fg-muted hover:bg-surface-muted hover:text-fg transition-colors"
          >
            <Paperclip aria-hidden="true" className="h-3.5 w-3.5" />
            <span>{t("chat.attach")}</span>
          </button>

          {/* Select Source Button - Đồng bộ 100% với Model Selector */}
          <button
            type="button"
            className="focus-ring flex items-center gap-1.5 rounded-xl tier-input px-2.5 py-1.5 text-xs font-medium text-fg-muted hover:bg-surface-muted transition-colors"
          >
            <span>{t("chat.selectSource")}</span>
            <ChevronDown aria-hidden="true" className="h-3 w-3 text-fg-subtle" />
          </button>
        </div>

        <div className="flex items-center gap-2">
          <ModelSelectorPopover />
          <ContextUsagePopover />
          <Button type="button" variant="primary" size="icon" onClick={handleSend} disabled={!prompt.trim()} aria-label="Send" title="Send">
            <ArrowUp aria-hidden="true" className="h-4 w-4" />
          </Button>
        </div>
      </div>
    </div>
  );
}
