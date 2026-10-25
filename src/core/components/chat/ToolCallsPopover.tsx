import { useState, useRef, useEffect } from "react";
import { Wrench, CheckCircle2, ChevronRight, Terminal } from "lucide-react";
import { cn } from "../../../shared/utils/cn";
import type { ToolCallItem } from "./types";

interface ToolCallsPopoverProps {
  toolCalls: ToolCallItem[];
}

export function ToolCallsPopover({ toolCalls }: ToolCallsPopoverProps) {
  const [open, setOpen] = useState(false);
  const [activeToolId, setActiveToolId] = useState<string>(toolCalls[0]?.id || "");
  const containerRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    function handleClickOutside(e: MouseEvent) {
      if (containerRef.current && !containerRef.current.contains(e.target as Node)) {
        setOpen(false);
      }
    }
    if (open) document.addEventListener("mousedown", handleClickOutside);
    return () => document.removeEventListener("mousedown", handleClickOutside);
  }, [open]);

  if (!toolCalls || toolCalls.length === 0) return null;

  const activeTool = toolCalls.find((t) => t.id === activeToolId) || toolCalls[0];

  return (
    <div className="relative inline-block" ref={containerRef}>
      <button
        type="button"
        onClick={() => setOpen(!open)}
        className={cn(
          "flex h-7 items-center gap-1.5 rounded-lg border px-2 text-xs font-medium transition-colors",
          open
            ? "border-blue-500 bg-blue-50 text-blue-600 dark:border-blue-800 dark:bg-blue-950/60 dark:text-blue-300"
            : "border-slate-200 bg-white text-slate-500 hover:border-slate-300 hover:text-slate-800 dark:border-slate-800 dark:bg-slate-900 dark:text-slate-400 dark:hover:text-slate-200"
        )}
        title="Xem các công cụ đã thực thi"
      >
        <Wrench className="h-3.5 w-3.5" />
        <span className="text-[11px] font-semibold">{toolCalls.length} tools</span>
      </button>

      {open && (
        <div className="absolute bottom-full left-0 mb-2 w-96 rounded-2xl border border-slate-200 bg-white p-3 shadow-xl dark:border-slate-800 dark:bg-slate-900 z-50 animate-in fade-in-0 zoom-in-95">
          <div className="flex items-center justify-between border-b border-slate-100 pb-2 px-1 dark:border-slate-800">
            <span className="text-xs font-semibold text-slate-900 dark:text-slate-100 flex items-center gap-1.5">
              <Terminal className="h-3.5 w-3.5 text-blue-500" />
              <span>Executed Tools ({toolCalls.length})</span>
            </span>
          </div>

          <div className="mt-2.5 grid grid-cols-5 gap-2">
            {/* List tools bên trái */}
            <div className="col-span-2 space-y-1 border-r border-slate-100 pr-1.5 dark:border-slate-800">
              {toolCalls.map((t) => (
                <button
                  key={t.id}
                  type="button"
                  onClick={() => setActiveToolId(t.id)}
                  className={cn(
                    "flex w-full items-center justify-between rounded-lg px-2 py-1.5 text-[11px] font-mono transition-colors text-left",
                    activeTool.id === t.id
                      ? "bg-slate-100 font-semibold text-slate-900 dark:bg-slate-800 dark:text-white"
                      : "text-slate-500 hover:bg-slate-50 dark:text-slate-400 dark:hover:bg-slate-800/50"
                  )}
                >
                  <span className="truncate">{t.name}</span>
                  <CheckCircle2 className="h-3 w-3 text-emerald-500 shrink-0" />
                </button>
              ))}
            </div>

            {/* Chi tiết parameter/result bên phải */}
            <div className="col-span-3 max-h-48 overflow-y-auto space-y-2 text-[10px] font-mono pl-1">
              <div>
                <span className="text-slate-400 block mb-0.5">Input:</span>
                <pre className="rounded-lg bg-slate-50 p-2 text-slate-700 dark:bg-slate-950 dark:text-slate-300 overflow-x-auto">
                  {JSON.stringify(activeTool.parameters, null, 2)}
                </pre>
              </div>

              {activeTool.result && (
                <div>
                  <span className="text-slate-400 block mb-0.5">Output:</span>
                  <pre className="rounded-lg bg-slate-50 p-2 text-slate-700 dark:bg-slate-950 dark:text-slate-300 overflow-x-auto">
                    {JSON.stringify(activeTool.result, null, 2)}
                  </pre>
                </div>
              )}
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
