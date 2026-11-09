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
  calculateBusinessFlowProgress,
  generateRequireMarkdown,
  TOTAL_TRACKED_ATTRIBUTES,
} from "@/features/business-flow";

const EMPTY_BUSINESS_FLOW_DATA: BusinessFlowData = {
  meta: {
    phase: 0,
    version: "v.01",
    updated_at: "",
  },
  require: {
    project: {
      display_name: "",
      summary: "",
      language: "",
      target: {
        arc_count: 0,
        chapter_count: 0,
        word_per_chapter_count: 0,
      },
      genres: [],
    },
    character: [],
  },
  optional: {},
};

export interface ProcessDetailDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  data?: BusinessFlowData;
  businessFlowData?: BusinessFlowData;
  confirmedCount?: number;
  aiInferredCount?: number;
  totalAttributes?: number;
  isRequireComplete?: boolean;
}

export function ProcessDetailDialog({
  open,
  onOpenChange,
  data,
  businessFlowData,
  confirmedCount,
  aiInferredCount,
  totalAttributes = TOTAL_TRACKED_ATTRIBUTES,
  isRequireComplete: propIsRequireComplete,
}: ProcessDetailDialogProps) {
  const t = useT();
  const { locale } = useLocale();
  const currentLang = locale;
  const lang = currentLang === "en" ? "en" : "vi";

  const flowData = businessFlowData ?? data ?? EMPTY_BUSINESS_FLOW_DATA;
  const markdown = generateRequireMarkdown(flowData, lang === "en" ? "en" : "vi");
  const computedProgress = calculateBusinessFlowProgress(flowData);
  const hasBackendCounts = confirmedCount !== undefined || aiInferredCount !== undefined;
  const filledCount = hasBackendCounts
    ? Math.max(computedProgress.filledCount, (confirmedCount ?? 0) + (aiInferredCount ?? 0))
    : computedProgress.filledCount;
  const totalCount = totalAttributes || computedProgress.totalCount;
  const percent = totalCount > 0 ? Math.min(100, Math.round((filledCount / totalCount) * 100)) : 0;
  const isRequireComplete =
    propIsRequireComplete !== undefined
      ? propIsRequireComplete || computedProgress.isRequireComplete
      : computedProgress.isRequireComplete;
  const progress = {
    percent,
    filledCount,
    totalCount,
    isRequireComplete,
  };

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-2xl p-6 sm:max-w-2xl max-h-[90vh] flex flex-col">
        {/* Header / Stats */}
        <div className="space-y-3">
          <div>
            <DialogTitle className="text-lg font-semibold text-fg sm:text-xl">
              {t("chat.processDetails.title")}
            </DialogTitle>
            <DialogDescription className="mt-1 text-sm text-fg-muted">
              {t("chat.processDetails.description")}
            </DialogDescription>
          </div>

          {/* Thanh tiến trình mini + Badge */}
          <div className="rounded-xl border border-line bg-surface-muted/40 p-3 space-y-2">
            <div className="flex items-center justify-between">
              <div className="flex items-center gap-2 text-sm font-semibold">
                <span className="text-fg">{progress.percent}%</span>
                <span className="text-fg-muted">•</span>
                <span className="text-fg-muted">
                  {progress.filledCount}/{progress.totalCount}{" "}
                  {lang === "en" ? "attributes" : "thuộc tính"}
                </span>
              </div>
              <span
                className={cn(
                  "inline-flex items-center rounded-full px-3 py-1 text-xs font-semibold border",
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
                <h2 className="text-base font-bold text-fg sm:text-lg border-b border-line pb-2 mt-5 mb-3 flex items-center gap-2 first:mt-1">
                  <span className="h-3.5 w-1 rounded-full bg-primary" aria-hidden="true" />
                  <span>{children}</span>
                </h2>
              ),
              p: ({ children }) => (
                <p className="text-sm sm:text-base leading-relaxed text-fg-muted mb-3 last:mb-0">
                  {children}
                </p>
              ),
              ul: ({ children }) => (
                <ul className="text-sm sm:text-base leading-relaxed space-y-1.5 mb-3.5 list-disc pl-5 text-fg-muted">
                  {children}
                </ul>
              ),
              ol: ({ children }) => (
                <ol className="text-sm sm:text-base leading-relaxed space-y-1.5 mb-3.5 list-decimal pl-5 text-fg-muted">
                  {children}
                </ol>
              ),
              li: ({ children }) => (
                <li className="text-sm sm:text-base leading-relaxed">
                  {children}
                </li>
              ),
              strong: ({ children }) => (
                <strong className="text-sm sm:text-base font-semibold text-fg">
                  {children}
                </strong>
              ),
              code: ({ children, className }) => {
                const text = String(children).replace(/^<|>$/g, "");
                return (
                  <code
                    className={cn(
                      "inline-block rounded-md bg-purple-500/15 px-2 py-0.5 font-sans text-sm font-semibold text-purple-700 border border-purple-500/30 dark:bg-purple-950/60 dark:text-purple-300 dark:border-purple-800/60 align-baseline mx-0.5",
                      className
                    )}
                  >
                    {text}
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
          <Button type="button" variant="primary" size="md" onClick={() => onOpenChange(false)}>
            {t("chat.close")}
          </Button>
        </div>
      </DialogContent>
    </Dialog>
  );
}
