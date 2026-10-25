import { useState, useRef, useEffect } from "react";
import { cn } from "../../../shared/utils/cn";

export function ContextUsagePopover() {
  const [open, setOpen] = useState(false);
  const containerRef = useRef<HTMLDivElement>(null);

  // Đóng khi click ngoài
  useEffect(() => {
    function handleClickOutside(e: MouseEvent) {
      if (containerRef.current && !containerRef.current.contains(e.target as Node)) {
        setOpen(false);
      }
    }
    if (open) document.addEventListener("mousedown", handleClickOutside);
    return () => document.removeEventListener("mousedown", handleClickOutside);
  }, [open]);

  const percent = 7;
  const radius = 6;
  const circumference = 2 * Math.PI * radius;
  const strokeDashoffset = circumference - (percent / 100) * circumference;

  return (
    <div className="relative inline-block" ref={containerRef}>
      {/* Vòng tròn Trigger context */}
      <button
        type="button"
        onClick={() => setOpen(!open)}
        className="flex items-center gap-1.5 rounded-lg px-2 py-1 text-xs font-medium text-slate-600 hover:bg-slate-100 hover:text-slate-900 dark:text-slate-400 dark:hover:bg-slate-800 dark:hover:text-slate-200 transition-colors"
        title="Context usage"
      >
        <svg className="h-4 w-4 -rotate-90" viewBox="0 0 16 16">
          <circle
            cx="8"
            cy="8"
            r={radius}
            className="stroke-slate-200 dark:stroke-slate-700"
            strokeWidth="2"
            fill="transparent"
          />
          <circle
            cx="8"
            cy="8"
            r={radius}
            className="stroke-blue-500 transition-all duration-300"
            strokeWidth="2"
            strokeDasharray={circumference}
            strokeDashoffset={strokeDashoffset}
            strokeLinecap="round"
            fill="transparent"
          />
        </svg>
        <span className="text-[11px] font-semibold">{percent}%</span>
      </button>

      {/* Popover chi tiết */}
      {open && (
        <div className="absolute bottom-full right-0 mb-2 w-72 rounded-2xl border border-slate-200 bg-white p-3.5 shadow-xl dark:border-slate-800 dark:bg-slate-900 z-50 animate-in fade-in-0 zoom-in-95">
          <div className="flex items-center justify-between text-xs">
            <span className="font-semibold text-slate-900 dark:text-slate-100">
              {percent}% <span className="font-normal text-slate-500">of context used</span>
            </span>
            <span className="text-[11px] text-slate-400 dark:text-slate-400">~67.7K / 1M</span>
          </div>

          {/* Thanh phân khúc context */}
          <div className="mt-2.5 flex h-1.5 w-full overflow-hidden rounded-full bg-slate-100 dark:bg-slate-800">
            <div style={{ width: "1%" }} className="bg-slate-400" />
            <div style={{ width: "2%" }} className="bg-purple-500" />
            <div style={{ width: "4%" }} className="bg-blue-500" />
          </div>

          {/* Breakdown items */}
          <div className="mt-3.5 space-y-2 text-xs">
            <div className="flex items-center justify-between">
              <div className="flex items-center gap-2">
                <span className="h-2 w-2 rounded-xs bg-slate-400" />
                <span className="text-slate-600 dark:text-slate-300">System prompt</span>
              </div>
              <span className="text-slate-400">~2.3K</span>
            </div>

            <div className="flex items-center justify-between">
              <div className="flex items-center gap-2">
                <span className="h-2 w-2 rounded-xs bg-purple-500" />
                <span className="text-slate-600 dark:text-slate-300">Tool definitions</span>
              </div>
              <span className="text-slate-400">~11.7K</span>
            </div>

            <div className="flex items-center justify-between">
              <div className="flex items-center gap-2">
                <span className="h-2 w-2 rounded-xs bg-blue-500" />
                <span className="text-slate-600 dark:text-slate-300">Messages</span>
              </div>
              <span className="text-slate-400">~133K</span>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
