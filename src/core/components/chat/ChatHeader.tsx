import { useState } from "react";
import { Info, Pencil } from "lucide-react";
import { cn } from "../../../shared/utils/cn";
import { ProcessDetailDialog } from "./ProcessDetailDialog";
import { Badge } from "../../../shared/components/ui/badge";
import { useT } from "../../../shared/i18n/useT";

interface ChatHeaderProps {
  onToggleDraft: () => void;
  isDraftOpen: boolean;
  progressPercent?: number;
}

export function ChatHeader({
  onToggleDraft,
  isDraftOpen,
  progressPercent = 65,
}: ChatHeaderProps) {
  const [infoOpen, setInfoOpen] = useState(false);
  const t = useT();

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
            <div role="progressbar" aria-valuenow={progressPercent} aria-valuemin={0} aria-valuemax={100} aria-label={t("chat.progress")} className="h-2 w-28 overflow-hidden rounded-full border border-line bg-surface-muted">
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
          <button
            type="button"
            onClick={onToggleDraft}
            className={cn(
              "focus-ring flex items-center gap-1.5 rounded-xl border px-3 py-1.5 text-xs font-medium transition-colors",
              isDraftOpen
                ? "border-blue-500 bg-blue-50 text-blue-700 dark:border-blue-500 dark:bg-blue-950/60 dark:text-blue-300"
                : "border-line bg-surface-card text-fg hover:bg-surface-muted",

            )}
          >
            <Pencil className="h-3.5 w-3.5" />
            <span>{t("chat.draftBuilding")}</span>
          </button>
        </div>
      </header>

      <ProcessDetailDialog open={infoOpen} onOpenChange={setInfoOpen} />
    </>
  );
}
