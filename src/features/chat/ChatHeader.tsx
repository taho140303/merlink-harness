import { useState } from "react";
import { Info, Pencil } from "lucide-react";
import { cn } from "@/shared/utils/cn";
import { ProcessDetailDialog } from "./ProcessDetailDialog";
import { Badge } from "@/shared/components/ui/badge";
import { useT } from "@/shared/i18n/useT";
import {
  type BusinessFlowData,
  isRequireComplete as checkRequireComplete,
} from "@/features/business-flow";

export interface ChatHeaderProps {
  onToggleDraft: () => void;
  isDraftOpen: boolean;
  progressPercent?: number;
  isRequireComplete?: boolean;
  businessFlowData?: BusinessFlowData;
}

export function ChatHeader({
  onToggleDraft,
  isDraftOpen,
  progressPercent = 65,
  isRequireComplete: propIsRequireComplete,
  businessFlowData,
}: ChatHeaderProps) {
  const [infoOpen, setInfoOpen] = useState(false);
  const t = useT();

  const isComplete =
    propIsRequireComplete !== undefined
      ? propIsRequireComplete
      : businessFlowData
        ? checkRequireComplete(businessFlowData)
        : true;

  return (
    <>
      <header className="flex h-14 shrink-0 items-center justify-between border-b border-line bg-surface-panel/70 px-6">
        <div className="flex items-center gap-2.5">
          <h2 className="text-sm font-semibold tracking-tight text-fg">
            Merlink-harness
          </h2>
          <Badge variant="neutral">{t("chat.preview")}</Badge>
        </div>

        <div className="flex items-center gap-3">
          <div className="flex items-center gap-2">
            <div
              role="progressbar"
              aria-valuenow={progressPercent}
              aria-valuemin={0}
              aria-valuemax={100}
              aria-label={t("chat.progress")}
              className="h-2 w-28 overflow-hidden rounded-full border border-line bg-surface-muted"
            >
              <div
                className="h-full rounded-full bg-accent transition-colors duration-300"
                style={{ width: `${progressPercent}%` }}
              />
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
      />
    </>
  );
}
