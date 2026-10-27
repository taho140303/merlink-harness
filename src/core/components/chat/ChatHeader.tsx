import { useState } from "react";
import { Info, Pencil } from "lucide-react";
import { cn } from "../../../shared/utils/cn";
import { ProcessDetailDialog } from "./ProcessDetailDialog";
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
      <header className="flex h-14 shrink-0 items-center justify-between border-b border-slate-200 bg-white/70 px-6  dark:border-slate-800/80 dark:bg-slate-950/70">
        <div className="flex items-center gap-2.5">
          <h2 className="text-sm font-semibold tracking-tight text-slate-900 dark:text-slate-100">
            Merlink-harness
          </h2>
          <span className="rounded-md border border-slate-200 bg-slate-100 px-1.5 py-0.5 text-[10px] font-medium text-slate-600 dark:border-slate-700 dark:bg-slate-800 dark:text-slate-300">
            {t("chat.preview")}
          </span>
        </div>

        <div className="flex items-center gap-3">
          <div className="flex items-center gap-2">
            <div className="h-2 w-28 overflow-hidden rounded-full border border-slate-200 bg-slate-100 dark:border-slate-700/80 dark:bg-slate-900">
              <div
                className="h-full rounded-full bg-slate-700 transition-all duration-300 dark:bg-slate-300"
                style={{ width: `${progressPercent}%` }}
              />
            </div>
            <button
              type="button"
              onClick={() => setInfoOpen(true)}
              className="rounded-lg p-1 text-slate-400 hover:bg-slate-100 hover:text-slate-700 dark:hover:bg-slate-800 dark:hover:text-slate-200"
              title="View progress detail"
            >
              <Info className="h-4 w-4" />
            </button>
          </div>
        </div>

        <div className="flex items-center gap-2">
          <button
            type="button"
            onClick={onToggleDraft}
            className={cn(
              "flex items-center gap-1.5 rounded-xl border px-3 py-1.5 text-xs font-medium shadow-xs transition-colors",
              isDraftOpen
                ? "border-blue-500 bg-blue-50 text-blue-700 dark:border-blue-500 dark:bg-blue-950/60 dark:text-blue-300"
                : "border-slate-200 bg-white text-slate-800 hover:bg-slate-50 dark:border-slate-800 dark:bg-slate-900 dark:text-slate-200 dark:hover:bg-slate-800",
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
