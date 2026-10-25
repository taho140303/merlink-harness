import { useState } from "react";
import { MessageSquare, Sliders, Info, Pencil } from "lucide-react";
import { cn } from "../../../shared/utils/cn";
import { ProcessDetailDialog } from "./ProcessDetailDialog";
import type { ChatMode } from "./types";

interface ChatHeaderProps {
  mode: ChatMode;
  onModeChange: (mode: ChatMode) => void;
  progressPercent?: number;
}

export function ChatHeader({ mode, onModeChange, progressPercent = 65 }: ChatHeaderProps) {
  const [infoOpen, setInfoOpen] = useState(false);

  return (
    <>
      <header className="flex h-14 shrink-0 items-center justify-between border-b border-slate-200 bg-white/70 px-6 backdrop-blur-xs dark:border-slate-800/80 dark:bg-slate-950/70">
        {/* Left: Title + Preview Badge */}
        <div className="flex items-center gap-2.5">
          <h2 className="text-sm font-semibold tracking-tight text-slate-900 dark:text-slate-100">
            Merlink-harness
          </h2>
          <span className="rounded-md border border-slate-200 bg-slate-100 px-1.5 py-0.5 text-[10px] font-medium text-slate-600 dark:border-slate-700 dark:bg-slate-800 dark:text-slate-300">
            preview
          </span>
        </div>

        {/* Center: Mode toggle + Process bar + Info icon */}
        <div className="flex items-center gap-3">
          {/* Toggle 2 mode: Chat & Manual */}
          <div className="flex items-center rounded-xl bg-slate-100 p-0.5 dark:bg-slate-800">
            <button
              type="button"
              onClick={() => onModeChange("chat")}
              className={cn(
                "flex items-center gap-1.5 rounded-lg px-3 py-1 text-xs font-medium transition-all",
                mode === "chat"
                  ? "bg-white text-slate-900 shadow-xs dark:bg-slate-700 dark:text-white"
                  : "text-slate-600 hover:text-slate-900 dark:text-slate-400 dark:hover:text-slate-200"
              )}
            >
              <MessageSquare className="h-3.5 w-3.5" />
              <span>Chat</span>
            </button>
            <button
              type="button"
              onClick={() => onModeChange("manual")}
              className={cn(
                "flex items-center gap-1.5 rounded-lg px-3 py-1 text-xs font-medium transition-all",
                mode === "manual"
                  ? "bg-white text-slate-900 shadow-xs dark:bg-slate-700 dark:text-white"
                  : "text-slate-600 hover:text-slate-900 dark:text-slate-400 dark:hover:text-slate-200"
              )}
            >
              <Sliders className="h-3.5 w-3.5" />
              <span>Manual</span>
            </button>
          </div>

          {/* Process bar */}
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
              title="Xem chi tiết tiến trình"
            >
              <Info className="h-4 w-4" />
            </button>
          </div>
        </div>

        {/* Right: Draft building button with pencil */}
        <div className="flex items-center gap-2">
          <button
            type="button"
            className="flex items-center gap-1.5 rounded-xl border border-slate-200 bg-white px-3 py-1.5 text-xs font-medium text-slate-800 shadow-xs transition-colors hover:bg-slate-50 dark:border-slate-800 dark:bg-slate-900 dark:text-slate-200 dark:hover:bg-slate-800"
          >
            <Pencil className="h-3.5 w-3.5" />
            <span>Draft building</span>
          </button>
        </div>
      </header>

      <ProcessDetailDialog open={infoOpen} onOpenChange={setInfoOpen} />
    </>
  );
}
