import ReactMarkdown from "react-markdown";
import remarkGfm from "remark-gfm";
import {
  Dialog,
  DialogContent,
  DialogTitle,
  DialogDescription,
} from "@/shared/components/ui/dialog";
import { Button } from "@/shared/components/ui/button";
import { useT } from "@/shared/i18n/useT";
import { useLocale } from "@/shared/i18n/store";
import { cn } from "@/shared/utils/cn";
import type { BusinessFlowData } from "@/features/business-flow";
import {
  SAMPLE_BUSINESS_FLOW_DATA,
  calculateBusinessFlowProgress,
  generateRequireMarkdown,
} from "@/features/business-flow";

export interface ProcessDetailDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  data?: BusinessFlowData;
  businessFlowData?: BusinessFlowData;
}

export function ProcessDetailDialog({
  open,
  onOpenChange,
  data,
  businessFlowData,
}: ProcessDetailDialogProps) {
  const t = useT();
  const { locale } = useLocale();
  const currentLang = locale;
  const lang = currentLang === "en" ? "en" : "vi";

  const flowData = businessFlowData ?? data ?? SAMPLE_BUSINESS_FLOW_DATA;
  const markdown = generateRequireMarkdown(flowData, lang === "en" ? "en" : "vi");
  const progress = calculateBusinessFlowProgress(flowData);

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-2xl p-6 sm:max-w-2xl max-h-[90vh] flex flex-col">
        {/* Header / Stats */}
        <div className="space-y-3">
          <div>
            <DialogTitle className="text-base font-semibold text-fg">
              {t("chat.processDetails.title")}
            </DialogTitle>
            <DialogDescription className="mt-1 text-xs text-fg-muted">
              {t("chat.processDetails.description")}
            </DialogDescription>
          </div>

          {/* Thanh tiến trình mini + Badge */}
          <div className="rounded-xl border border-line bg-surface-muted/40 p-3 space-y-2">
            <div className="flex items-center justify-between text-xs">
              <div className="flex items-center gap-2">
                <span className="font-semibold text-fg">{progress.percent}%</span>
                <span className="text-fg-muted">•</span>
                <span className="text-fg-muted">
                  {progress.filledCount}/{progress.totalCount}{" "}
                  {lang === "en" ? "attributes" : "thuộc tính"}
                </span>
              </div>
              <span
                className={cn(
                  "inline-flex items-center rounded-full px-2.5 py-0.5 text-xs font-medium border",
                  progress.isRequireComplete
                    ? "bg-emerald-500/15 text-emerald-700 dark:text-emerald-300 dark:bg-emerald-950/60 border-emerald-500/30"
                    : "bg-amber-500/15 text-amber-700 dark:text-amber-300 dark:bg-amber-950/60 border-amber-500/30"
                )}
              >
                {progress.isRequireComplete
                  ? (lang === "en" ? "Core: Completed" : "Cốt lõi: Đã hoàn tất")
                  : (lang === "en" ? "Core: Incomplete" : "Cốt lõi: Chưa hoàn tất")}
              </span>
            </div>
            <div className="h-1.5 w-full overflow-hidden rounded-full bg-surface-muted border border-line/40">
              <div
                className={cn(
                  "h-full rounded-full transition-all duration-300",
                  progress.isRequireComplete ? "bg-emerald-500" : "bg-primary"
                )}
                style={{ width: `${Math.min(100, Math.max(0, progress.percent))}%` }}
              />
            </div>
          </div>
        </div>

        {/* Render nội dung Markdown */}
        <div className="mt-2 max-h-[60vh] overflow-y-auto pr-2 select-text">
          <ReactMarkdown
            remarkPlugins={[remarkGfm]}
            components={{
              h2: ({ children }) => (
                <h2 className="mt-4 mb-2 flex items-center gap-2 border-b border-line pb-1.5 text-sm font-semibold text-fg first:mt-1">
                  <span className="h-3.5 w-1 rounded-full bg-primary" aria-hidden="true" />
                  <span>{children}</span>
                </h2>
              ),
              p: ({ children }) => (
                <p className="mb-2.5 text-xs leading-relaxed text-fg-muted last:mb-0">
                  {children}
                </p>
              ),
              ul: ({ children }) => (
                <ul className="mb-3 list-disc pl-5 space-y-1 text-xs text-fg-muted leading-relaxed">
                  {children}
                </ul>
              ),
              ol: ({ children }) => (
                <ol className="mb-3 list-decimal pl-5 space-y-1 text-xs text-fg-muted leading-relaxed">
                  {children}
                </ol>
              ),
              li: ({ children }) => (
                <li className="text-xs leading-relaxed">
                  {children}
                </li>
              ),
              strong: ({ children }) => (
                <strong className="font-semibold text-fg">
                  {children}
                </strong>
              ),
              code: ({ children, className }) => {
                const text = String(children);
                const isHighlighted = text.startsWith("<") && text.endsWith(">");
                return (
                  <code
                    className={cn(
                      "rounded px-1.5 py-0.5 font-mono text-xs font-semibold",
                      isHighlighted
                        ? "bg-purple-500/15 text-purple-700 dark:text-purple-300 dark:bg-purple-950/60 border border-purple-500/30"
                        : "bg-surface-muted text-fg",
                      className
                    )}
                  >
                    {children}
                  </code>
                );
              },
            }}
          >
            {markdown}
          </ReactMarkdown>
        </div>

        {/* Nút đóng Dialog dưới chân */}
        <div className="flex justify-end pt-3 border-t border-line mt-auto">
          <Button type="button" variant="primary" onClick={() => onOpenChange(false)}>
            {t("chat.close")}
          </Button>
        </div>
      </DialogContent>
    </Dialog>
  );
}
