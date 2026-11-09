import { useState, useEffect } from "react";
import ReactMarkdown from "react-markdown";
import remarkGfm from "remark-gfm";
import { Copy, Check, ChevronDown, ChevronUp, Loader2, Paperclip } from "lucide-react";
import { cn } from "@/shared/utils/cn";
import { ToolCallsPopover } from "./ToolCallsPopover";
import type { ChatMessage } from "./types";
import { useT } from "@/shared/i18n/useT";
import { splitChatAttachmentContent } from "./attachmentContent";

interface ChatMessageItemProps {
  message: ChatMessage;
}

export function ChatMessageItem({ message }: ChatMessageItemProps) {
  const t = useT();
  const [copied, setCopied] = useState(false);
  const [thinkingExpanded, setThinkingExpanded] = useState(false);
  const [liveSeconds, setLiveSeconds] = useState(message.thinkingSeconds ?? 1);

  const isAssistant = message.role === "assistant";
  const userContent = isAssistant ? null : splitChatAttachmentContent(message.content);

  useEffect(() => {
    if (!message.isThinking) {
      setLiveSeconds(message.thinkingSeconds ?? 1);
      return;
    }

    const start = Date.now();
    const interval = setInterval(() => {
      setLiveSeconds(Math.max(1, Math.floor((Date.now() - start) / 1000) + 1));
    }, 1000);

    return () => clearInterval(interval);
  }, [message.isThinking]);

  const handleCopy = () => {
    navigator.clipboard.writeText(message.content);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div id={message.id} className="mx-auto w-full max-w-4xl py-3 space-y-2 select-text scroll-mt-6">
      {/* 1. Trạng thái Đang suy nghĩ: Đồng hồ đếm giây + Stream dòng suy nghĩ */}
      {isAssistant && message.isThinking && (
        <div className="space-y-2 duration-200">
          <div className="flex items-center gap-2 text-xs font-medium text-fg-muted">
            <Loader2 className="h-3.5 w-3.5 animate-spin text-blue-500" />
            <span>{t("chat.thinking", { seconds: liveSeconds })}</span>
          </div>

          {/* Khối log suy nghĩ streaming trực tiếp */}
          {message.thinkingDetails && (
            <div className="rounded-xl border border-dashed border-line-strong bg-surface-muted/70 p-3 font-mono text-xs italic leading-relaxed text-fg-muted   ">
              <span className="whitespace-pre-line">{message.thinkingDetails}</span>
              <span className="inline-block h-3 w-1.5 ml-1 animate-pulse bg-blue-500 align-middle" />
            </div>
          )}
        </div>
      )}

      {/* 2. Trạng thái Đã suy nghĩ xong: Collapsible button */}
      {isAssistant && !message.isThinking && message.thinkingSeconds !== undefined && (
        <div className="flex flex-col space-y-1">
          <button
            type="button"
            onClick={() => setThinkingExpanded(!thinkingExpanded)}
            aria-expanded={thinkingExpanded}
            className="flex items-center gap-1.5 text-xs font-medium text-fg-subtle hover:text-fg-muted   transition-colors w-fit"
          >
            <span>{t("chat.thoughtForSeconds", { seconds: message.thinkingSeconds })}</span>
            {message.thinkingDetails && (
              thinkingExpanded ? <ChevronUp aria-hidden="true" className="h-3 w-3" /> : <ChevronDown aria-hidden="true" className="h-3 w-3" />
            )}
          </button>

          {thinkingExpanded && message.thinkingDetails && (
            <div className="rounded-xl border border-line bg-surface-muted/50 p-3 font-mono text-xs italic leading-relaxed text-fg-muted   ">
              <p className="whitespace-pre-line">{message.thinkingDetails}</p>
            </div>
          )}
        </div>
      )}

      {/* 3. Khung chat Markdown: Chỉ render khi có content (không render khung rỗng lúc thinking) */}
      {Boolean(message.content) && (
        <div
          className={cn(
            "group relative rounded-card p-4 transition-colors duration-200",
            isAssistant
              ? "tier-card w-full"
              : "ml-auto w-fit max-w-[85%] border border-accent/20 bg-accent-soft text-fg sm:max-w-[75%]"
          )}
        >
          <div className="relative">
            {/* Div ẩn dự phòng overlay tương lai */}
            <div
              className="hidden pointer-events-none absolute inset-0 z-0 select-none opacity-0"
              aria-hidden="true"
            />

            <div className="relative z-10 text-xs leading-relaxed text-fg sm:text-sm">
              {!isAssistant ? (
                <>
                  {userContent?.text && <p className="whitespace-pre-wrap break-words">{userContent.text}</p>}
                  {Boolean(userContent?.names.length) && (
                    <div className="mt-2 flex flex-wrap gap-1.5">
                      {userContent?.names.map((name, index) => (
                        <span key={`${name}-${index}`} className="inline-flex max-w-full items-center gap-1 rounded-lg border border-accent/20 bg-surface-card px-2 py-1 text-xs">
                          <Paperclip aria-hidden="true" className="h-3 w-3 shrink-0" />
                          <span className="truncate">{name}</span>
                        </span>
                      ))}
                    </div>
                  )}
                </>
              ) : <ReactMarkdown
                remarkPlugins={[remarkGfm]}
                components={{
                  p: ({ children }) => <p className="mb-3 last:mb-0">{children}</p>,
                  ol: ({ children }) => <ol className="mb-3 list-decimal pl-5 space-y-1.5 text-fg">{children}</ol>,
                  pre: ({ children }) => <pre className="my-3 overflow-x-auto rounded-lg border border-line bg-surface-muted p-3 text-fg">{children}</pre>,
                  table: ({ children }) => <table className="my-3 w-full border-collapse border border-line text-left">{children}</table>,
                  blockquote: ({ children }) => <blockquote className="my-3 border-l-2 border-line-strong pl-3 text-fg-muted">{children}</blockquote>,
                  a: ({ children, href }) => <a className="focus-ring text-accent underline" href={href}>{children}</a>,
                  ul: ({ children }) => <ul className="mb-3 list-disc pl-5 space-y-1.5 text-fg">{children}</ul>,
                  strong: ({ children }) => (
                    <strong className="font-semibold text-fg">{children}</strong>
                  ),
                  code: ({ children }) => (
                    <code className="rounded-md bg-surface-muted px-1.5 py-0.5 text-xs font-mono text-fg">
                      {children}
                    </code>
                  ),
                }}
              >
                {message.content}
              </ReactMarkdown>}
            </div>
          </div>
        </div>
      )}

      {/* 4. Action bar cho assistant khi đã có nội dung */}
      {isAssistant && !message.isThinking && (Boolean(message.content) || Boolean(message.toolCalls?.length)) && (
        <div className="flex items-center gap-2 pt-0.5 pl-1">
          <button
            type="button"
            onClick={handleCopy}
            className="focus-ring flex h-7 w-7 items-center justify-center rounded-lg tier-card text-fg-muted transition-colors"
            aria-label={t("chat.copyContent")}
            title={t("chat.copyContent")}
          >
            {copied ? <Check aria-hidden="true" className="h-3.5 w-3.5 text-emerald-500" /> : <Copy aria-hidden="true" className="h-3.5 w-3.5" />}
          </button>

          {message.toolCalls && message.toolCalls.length > 0 && (
            <ToolCallsPopover toolCalls={message.toolCalls} />
          )}
        </div>
      )}
    </div>
  );
}
