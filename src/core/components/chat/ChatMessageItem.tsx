import { useState } from "react";
import ReactMarkdown from "react-markdown";
import remarkGfm from "remark-gfm";
import { Copy, Check, ChevronDown, ChevronUp, Loader2 } from "lucide-react";
import { cn } from "../../../shared/utils/cn";
import { ToolCallsPopover } from "./ToolCallsPopover";
import type { ChatMessage } from "./types";

interface ChatMessageItemProps {
  message: ChatMessage;
}

export function ChatMessageItem({ message }: ChatMessageItemProps) {
  const [copied, setCopied] = useState(false);
  const [thinkingExpanded, setThinkingExpanded] = useState(false);

  const handleCopy = () => {
    navigator.clipboard.writeText(message.content);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  const isAssistant = message.role === "assistant";

  return (
    <div className="mx-auto w-full max-w-4xl py-3 space-y-1.5 select-text">
      {/* 1. Dòng thời gian suy nghĩ ở TRÊN đầu (chỉ dành cho assistant) */}
      {isAssistant && message.isThinking && (
        <div className="flex items-center gap-2 text-xs font-medium text-slate-400 dark:text-slate-400 animate-pulse">
          <Loader2 className="h-3.5 w-3.5 animate-spin text-blue-500" />
          <span>Đang suy nghĩ...</span>
        </div>
      )}

      {isAssistant && !message.isThinking && message.thinkingSeconds !== undefined && (
        <div className="flex flex-col space-y-1">
          <button
            type="button"
            onClick={() => setThinkingExpanded(!thinkingExpanded)}
            className="flex items-center gap-1.5 text-xs text-slate-400 hover:text-slate-600 dark:text-slate-400 dark:hover:text-slate-200 transition-colors"
          >
            <span>Đã suy nghĩ {message.thinkingSeconds} giây</span>
            {message.thinkingDetails && (
              thinkingExpanded ? <ChevronUp className="h-3 w-3" /> : <ChevronDown className="h-3 w-3" />
            )}
          </button>

          {/* Chi tiết nội dung suy nghĩ nếu có */}
          {thinkingExpanded && message.thinkingDetails && (
            <div className="rounded-xl border border-slate-200/60 bg-slate-50/50 p-3 text-xs italic text-slate-500 dark:border-slate-800/60 dark:bg-slate-900/40 dark:text-slate-400">
              {message.thinkingDetails}
            </div>
          )}
        </div>
      )}

      {/* 2. Khung chat container: Markdown render xếp chồng lên một thẻ div ẩn */}
      <div
        className={cn(
          "group relative rounded-2xl border transition-all duration-200",
          isAssistant
            ? "border-slate-200 bg-white/95 p-4 shadow-xs dark:border-slate-800/80 dark:bg-[#151921]"
            : "border-blue-100 bg-blue-50/70 p-4 dark:border-blue-900/50 dark:bg-blue-950/30"
        )}
      >
        <div className="relative">
          {/* Thẻ div xếp chồng bên dưới - hiện tại ẩn để dành cho canvas/tool overlay tương lai */}
          <div
            className="hidden pointer-events-none absolute inset-0 z-0 select-none opacity-0"
            aria-hidden="true"
          />

          {/* Lớp Markdown Render chính */}
          <div className="relative z-10 text-xs sm:text-sm leading-relaxed text-slate-800 dark:text-slate-200">
            <ReactMarkdown
              remarkPlugins={[remarkGfm]}
              components={{
                p: ({ children }) => <p className="mb-3 last:mb-0">{children}</p>,
                ol: ({ children }) => <ol className="mb-3 list-decimal pl-5 space-y-1.5">{children}</ol>,
                ul: ({ children }) => <ul className="mb-3 list-disc pl-5 space-y-1.5">{children}</ul>,
                strong: ({ children }) => (
                  <strong className="font-semibold text-slate-900 dark:text-slate-100">{children}</strong>
                ),
                code: ({ children }) => (
                  <code className="rounded-md bg-slate-100 px-1.5 py-0.5 text-xs font-mono text-slate-800 dark:bg-slate-800 dark:text-slate-200">
                    {children}
                  </code>
                ),
              }}
            >
              {message.content}
            </ReactMarkdown>
          </div>
        </div>
      </div>

      {/* 3. Action bar dưới tin nhắn assistant: Nút Copy + Nút Cờ lê (Tool calls) */}
      {isAssistant && !message.isThinking && (
        <div className="flex items-center gap-2 pt-1 pl-1">
          <button
            type="button"
            onClick={handleCopy}
            className="flex h-7 w-7 items-center justify-center rounded-lg border border-slate-200 bg-white text-slate-500 hover:border-slate-300 hover:text-slate-800 dark:border-slate-800 dark:bg-slate-900 dark:text-slate-400 dark:hover:text-slate-200 transition-colors"
            title="Sao chép nội dung"
          >
            {copied ? <Check className="h-3.5 w-3.5 text-emerald-500" /> : <Copy className="h-3.5 w-3.5" />}
          </button>

          {message.toolCalls && message.toolCalls.length > 0 && (
            <ToolCallsPopover toolCalls={message.toolCalls} />
          )}
        </div>
      )}
    </div>
  );
}
