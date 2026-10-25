import { PanelLeftClose, PanelLeftOpen } from "lucide-react";

interface SidebarHeaderProps {
  collapsed: boolean;
  onToggle: () => void;
}

export function SidebarHeader({ collapsed, onToggle }: SidebarHeaderProps) {
  if (collapsed) {
    return (
      <div className="flex h-16 shrink-0 items-center justify-center border-b border-slate-100 px-2 dark:border-slate-800/80">
        <button
          onClick={onToggle}
          className="group relative flex h-10 w-10 items-center justify-center rounded-xl bg-slate-900 text-white shadow-xs transition-colors hover:bg-slate-800 dark:bg-slate-100 dark:text-slate-900 dark:hover:bg-slate-200"
          title="Mở sidebar"
        >
          <div className="flex items-center justify-center transition-opacity duration-200 group-hover:opacity-0">
            <span className="text-base font-bold">M</span>
          </div>
          <div className="absolute inset-0 flex items-center justify-center opacity-0 transition-opacity duration-200 group-hover:opacity-100">
            <PanelLeftOpen className="h-5 w-5" />
          </div>
        </button>
      </div>
    );
  }

  return (
    <div className="flex h-16 shrink-0 items-center justify-between border-b border-slate-100 px-2 dark:border-slate-800/80">
      <div className="flex items-center gap-3 overflow-hidden">
        <div className="flex h-10 w-10 shrink-0 items-center justify-center rounded-xl bg-slate-900 text-white shadow-xs dark:bg-slate-100 dark:text-slate-900">
          <span className="text-base font-bold">M</span>
        </div>
        <span className="truncate font-semibold tracking-tight text-slate-900 dark:text-slate-100">
          merlink-harness
        </span>
      </div>
      <button
        onClick={onToggle}
        className="rounded-lg p-2 text-slate-400 hover:bg-slate-100 hover:text-slate-700 dark:text-slate-400 dark:hover:bg-slate-800 dark:hover:text-slate-200"
        title="Thu gọn sidebar"
      >
        <PanelLeftClose className="h-5 w-5" />
      </button>
    </div>
  );
}
