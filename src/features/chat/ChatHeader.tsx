import { useState } from "react";
import { Info, Pencil, AlertCircle } from "lucide-react";
import { cn } from "@/shared/utils/cn";
import { ProcessDetailDialog } from "./ProcessDetailDialog";
import { Badge } from "@/shared/components/ui/badge";
import { useT } from "@/shared/i18n/useT";
import {
  type BusinessFlowData,
  isRequireComplete as checkRequireComplete,
  TOTAL_TRACKED_ATTRIBUTES,
} from "@/features/business-flow";

export interface ChatHeaderProps {
  onToggleDraft: () => void;
  isDraftOpen: boolean;
  progressPercent?: number;
  isRequireComplete?: boolean;
  businessFlowData?: BusinessFlowData;
  confirmedCount?: number;
  aiInferredCount?: number;
  totalAttributes?: number;
  readinessGatePassed?: boolean;
  backendStatus?: "available" | "unavailable";
  interruptedTurnAlert?: string | null;
  onDismissAlert?: () => void;
}

export function ChatHeader({
  onToggleDraft,
  isDraftOpen,
  progressPercent = 0,
  isRequireComplete: propIsRequireComplete,
  businessFlowData,
  confirmedCount,
  aiInferredCount,
  totalAttributes = TOTAL_TRACKED_ATTRIBUTES,
  readinessGatePassed,
  backendStatus = "available",
  interruptedTurnAlert,
  onDismissAlert,
}: ChatHeaderProps) {
  const [infoOpen, setInfoOpen] = useState(false);
  const t = useT();

  const isComplete =
    readinessGatePassed !== undefined
      ? readinessGatePassed
      : propIsRequireComplete !== undefined
        ? propIsRequireComplete
        : businessFlowData
          ? checkRequireComplete(businessFlowData)
          : false;

  const hasDual = confirmedCount !== undefined || aiInferredCount !== undefined;
  const confirmedPct = hasDual
    ? Math.round(((confirmedCount ?? 0) / totalAttributes) * 100)
    : progressPercent;
  const aiPct = hasDual
    ? Math.round(((aiInferredCount ?? 0) / totalAttributes) * 100)
    : 0;
  const displayTotalPct = Math.min(100, confirmedPct + aiPct);

  return (
    <>
      <header className="flex h-14 shrink-0 items-center justify-between border-b border-line bg-surface-panel/70 px-6">
        <div className="flex items-center gap-2.5">
          <h2 className="text-sm font-semibold tracking-tight text-fg">
            Merlink-harness
          </h2>
          {backendStatus === "unavailable" ? (
            <Badge
              variant="neutral"
              className="border-amber-500/40 text-amber-600 dark:text-amber-400 bg-amber-50/50 dark:bg-amber-950/30"
              title="Backend runtime bridge is unavailable"
            >
              Backend Unavailable
            </Badge>
          ) : (
            <Badge variant="neutral">{t("chat.preview")}</Badge>
          )}
          {interruptedTurnAlert && (
            <div
              role="alert"
              className="flex items-center gap-1.5 rounded-full border border-amber-500/30 bg-amber-50/80 px-2.5 py-0.5 text-[11px] font-medium text-amber-700 dark:bg-amber-950/40 dark:text-amber-300"
              title={interruptedTurnAlert}
            >
              <AlertCircle className="h-3 w-3 shrink-0 text-amber-500" />
              <span className="max-w-[200px] truncate">{interruptedTurnAlert}</span>
              {onDismissAlert && (
                <button
                  type="button"
                  onClick={onDismissAlert}
                  className="ml-1 text-xs opacity-60 hover:opacity-100"
                  aria-label="Dismiss alert"
                >
                  ×
                </button>
              )}
            </div>
          )}
        </div>

        <div className="flex items-center gap-3">
          <div className="flex items-center gap-2">
            <div
              role="progressbar"
              aria-valuenow={displayTotalPct}
              aria-valuemin={0}
              aria-valuemax={100}
              aria-label={t("chat.progress")}
              title={
                hasDual
                  ? `Đã xác nhận: ${confirmedCount ?? 0}/${totalAttributes} (${confirmedPct}%) | AI đề xuất: ${aiInferredCount ?? 0}/${totalAttributes} (${aiPct}%)`
                  : undefined
              }
              className="flex h-2 w-28 overflow-hidden rounded-full border border-line bg-surface-muted"
            >
              <div
                className="h-full bg-accent transition-all duration-300"
                style={{ width: `${confirmedPct}%` }}
              />
              {aiPct > 0 && (
                <div
                  className="h-full bg-accent/40 transition-all duration-300"
                  style={{ width: `${aiPct}%` }}
                />
              )}
            </div>
            <button
              type="button"
              onClick={() => setInfoOpen(true)}
              aria-label={t("chat.viewProgressDetail")}
              className="focus-ring rounded-lg p-1 text-fg-subtle hover:bg-surface-muted hover:text-fg"
              title={t("chat.viewProgressDetail")}
            >
              <Info aria-hidden="true" className="h-4 w-4" />
            </button>
          </div>
        </div>

        <div className="flex items-center gap-2">
          <div
            title={!isComplete ? t("chat.draftBuilding.disabledTooltip") : undefined}
            className="inline-flex"
          >
            <button
              type="button"
              disabled={!isComplete}
              onClick={onToggleDraft}
              aria-pressed={isDraftOpen}
              title={!isComplete ? t("chat.draftBuilding.disabledTooltip") : undefined}
              className={cn(
                "focus-ring flex items-center gap-1.5 rounded-xl border px-3 py-1.5 text-xs font-medium transition-colors",
                !isComplete
                  ? "cursor-not-allowed opacity-50 border-line bg-surface-card text-fg-muted"
                  : isDraftOpen
                    ? "border-blue-500 bg-blue-50 text-blue-700 dark:border-blue-500 dark:bg-blue-950/60 dark:text-blue-300"
                    : "border-line bg-surface-card text-fg hover:bg-surface-muted",
              )}
            >
              <Pencil className="h-3.5 w-3.5" />
              <span>{t("chat.draftBuilding")}</span>
            </button>
          </div>
        </div>
      </header>

      <ProcessDetailDialog
        open={infoOpen}
        onOpenChange={setInfoOpen}
        businessFlowData={businessFlowData}
        data={businessFlowData}
        confirmedCount={confirmedCount}
        aiInferredCount={aiInferredCount}
        totalAttributes={totalAttributes}
        isRequireComplete={isComplete}
      />
    </>
  );
}
