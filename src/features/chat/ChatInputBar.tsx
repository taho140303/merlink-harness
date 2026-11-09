import { useState, useRef, useEffect } from "react";
import { Button } from "@/shared/components/ui/button";
import { Sparkles, ChevronDown, Paperclip, ArrowUp, Square, X, MessageSquareQuote } from "lucide-react";
import { ContextUsagePopover } from "./ContextUsagePopover";
import { ModelSelectorPopover } from "./ModelSelectorPopover";
import { useT } from "@/shared/i18n/useT";
import { toast } from "@/shared/components/ui/toast";
import type { FeedbackContext } from "@/features/draft/types";
import { serializeChatAttachments } from "./attachmentContent";

interface ChatInputBarProps {
  feedbackContext?: FeedbackContext | null;
  onClearFeedback?: () => void;
  onSend?: (text: string) => void;
  disabled?: boolean;
  isGenerating?: boolean;
  onStop?: () => void;
}

interface AttachedTextFile { name: string; content: string; size: number }
const MAX_ATTACHMENT_BYTES = 2 * 1024 * 1024;
const MAX_ATTACHMENTS = 5;

export function acceptsChatAttachment(file: Pick<File, "name" | "size">): boolean {
  return /\.(md|json)$/i.test(file.name) && file.size <= MAX_ATTACHMENT_BYTES;
}

export function ChatInputBar({
  feedbackContext,
  onClearFeedback,
  onSend,
  disabled = false,
  isGenerating = false,
  onStop,
}: ChatInputBarProps) {
  const t = useT();
  const [prompt, setPrompt] = useState("");
  const [attachments, setAttachments] = useState<AttachedTextFile[]>([]);
  const [isReadingFile, setIsReadingFile] = useState(false);
  const inputRef = useRef<HTMLTextAreaElement>(null);
  const fileRef = useRef<HTMLInputElement>(null);
  const MAX_HEIGHT = 208;

  useEffect(() => {
    const textarea = inputRef.current;
    if (!textarea) return;

    textarea.style.height = "auto";
    const nextHeight = Math.min(textarea.scrollHeight, MAX_HEIGHT);
    textarea.style.height = `${nextHeight}px`;
    textarea.style.overflowY = textarea.scrollHeight > MAX_HEIGHT ? "auto" : "hidden";
  }, [prompt]);

  useEffect(() => {
    if (feedbackContext) {
      inputRef.current?.focus();
    }
  }, [feedbackContext]);

  const handleSend = () => {
    if (disabled || isGenerating || isReadingFile || (!prompt.trim() && attachments.length === 0)) return;
    onSend?.(serializeChatAttachments(prompt, attachments.map(({ name, content }) => ({ name, content }))));
    setPrompt("");
    setAttachments([]);
    if (inputRef.current) {
      inputRef.current.style.height = "auto";
      inputRef.current.style.overflowY = "hidden";
    }
    onClearFeedback?.();
  };

  const handleFiles = async (files: FileList | null) => {
    const picked = Array.from(files ?? []);
    if (!picked.length) return;
    const remaining = MAX_ATTACHMENTS - attachments.length;
    if (picked.length > remaining || picked.some((file) => !acceptsChatAttachment(file))) {
      toast({ title: t("chat.attachInvalid"), variant: "error" });
      return;
    }
    setIsReadingFile(true);
    try {
      const next = await Promise.all(picked.map(async (file) => ({
        name: file.name.replace(/[\r\n]/g, " "),
        content: await file.text(),
        size: file.size,
      })));
      setAttachments((current) => [...current, ...next]);
    } catch {
      toast({ title: t("chat.attachReadFailed"), variant: "error" });
    } finally {
      setIsReadingFile(false);
    }
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
            aria-label={t("chat.clearFeedbackContext")}
            className="focus-ring rounded-md p-1 hover:bg-blue-100 dark:hover:bg-blue-900 transition-colors"
          >
            <X aria-hidden="true" className="h-3.5 w-3.5" />
          </button>
        </div>
      )}

      <div className="flex items-start gap-2 px-2 pt-1 text-fg-subtle">
        <Sparkles className="mt-2 h-4 w-4" />
        <textarea
          ref={inputRef}
          disabled={disabled || isGenerating}
          aria-label={t("chat.message")}
          rows={1}
          value={prompt}
          onChange={(e) => setPrompt(e.target.value)}
          onKeyDown={(e) => {
            if (e.nativeEvent.isComposing || e.key !== "Enter") return;
            if (e.altKey || e.shiftKey || e.ctrlKey || e.metaKey) {
              e.preventDefault();
              const textarea = e.currentTarget;
              const start = textarea.selectionStart;
              const end = textarea.selectionEnd;
              const pos = start;
              const nextPrompt = `${prompt.slice(0, start)}\n${prompt.slice(end)}`;
              setPrompt(() => nextPrompt);
              requestAnimationFrame(() => {
                textarea.selectionStart = textarea.selectionEnd = pos + 1;
              });
              return;
            }
            e.preventDefault();
            handleSend();
          }}
          placeholder={
            feedbackContext
              ? t("chat.feedbackPlaceholder", { title: feedbackContext.title })
              : t("chat.messagePlaceholder")
          }
          className="tier-input focus-ring max-h-52 w-full resize-none overflow-hidden rounded-xl bg-transparent px-3 py-2 text-sm leading-relaxed text-fg placeholder:text-fg-subtle"
        />
      </div>

      {attachments.length > 0 && (
        <div className="mt-2 flex flex-wrap gap-1.5 px-2">
          {attachments.map((file, index) => (
            <span key={`${file.name}-${index}`} className="inline-flex items-center gap-1 rounded-lg border border-line bg-surface-muted px-2 py-1 text-xs text-fg-muted">
              <span className="max-w-36 truncate">{file.name}</span>
              <button type="button" onClick={() => setAttachments((current) => current.filter((_, i) => i !== index))}
                aria-label={`${t("common.remove")}: ${file.name}`} className="focus-ring rounded p-0.5 hover:text-fg">
                <X aria-hidden="true" className="h-3 w-3" />
              </button>
            </span>
          ))}
        </div>
      )}

      <div className="mt-3 flex items-center justify-between border-t border-line pt-2.5">
        <div className="flex items-center gap-2">
          <input ref={fileRef} type="file" accept=".md,.json,text/markdown,application/json" multiple
            className="sr-only" tabIndex={-1} aria-hidden="true"
            onChange={(event) => { void handleFiles(event.currentTarget.files); event.currentTarget.value = ""; }} />
          <button
            type="button"
            onClick={() => fileRef.current?.click()}
            disabled={disabled || isGenerating || isReadingFile || attachments.length >= MAX_ATTACHMENTS}
            title={t("chat.attachTooltip")}
            aria-label={t("chat.attachTooltip")}
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
          {isGenerating ? (
            <Button type="button" variant="danger" size="icon" onClick={onStop} disabled={!onStop} aria-label={t("chat.stopGenerating")} title={t("chat.stopGenerating")}>
              <Square aria-hidden="true" className="h-3.5 w-3.5 fill-current" />
            </Button>
          ) : (
            <Button type="button" variant="primary" size="icon" onClick={handleSend} disabled={disabled || isReadingFile || (!prompt.trim() && attachments.length === 0)} aria-label={t("chat.send")} title={t("chat.send")}>
              <ArrowUp aria-hidden="true" className="h-4 w-4" />
            </Button>
          )}
        </div>
      </div>
    </div>
  );
}
