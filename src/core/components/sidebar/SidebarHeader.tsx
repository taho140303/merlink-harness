import { PanelLeftClose, PanelLeftOpen } from "lucide-react";
import { useT } from "../../../shared/i18n/useT";

interface SidebarHeaderProps {
  collapsed: boolean;
  onToggle: () => void;
}

export function SidebarHeader({ collapsed, onToggle }: SidebarHeaderProps) {
  const t = useT();

  if (collapsed) {
    return (
      <div className="flex h-14 shrink-0 items-center justify-center px-2">
        <button
          onClick={onToggle}
          aria-label={t("sidebar.expand")}
          className="focus-ring group relative flex h-9 w-9 items-center justify-center rounded-xl neu-pressed text-fg-muted transition-colors hover:text-accent"
          title={t("sidebar.expand")}
        >
          <div className="flex items-center justify-center transition-opacity duration-200 group-hover:opacity-0">
            <img src="/logo.svg" alt="" className="h-6 w-6 object-contain" />
          </div>
          <div className="absolute inset-0 flex items-center justify-center opacity-0 transition-opacity duration-200 group-hover:opacity-100">
            <PanelLeftOpen className="h-5 w-5" aria-hidden="true" />
          </div>
        </button>
      </div>
    );
  }

  return (
    <div className="flex h-14 shrink-0 items-center justify-between px-1">
      <div className="flex items-center gap-2.5 overflow-hidden">
        <div className="flex h-9 w-9 shrink-0 items-center justify-center rounded-xl neu-pressed">
          <img src="/logo.svg" alt="" className="h-6 w-6 object-contain" />
        </div>
        <span className="whitespace-nowrap text-sm font-extrabold tracking-tight text-fg">
          merlink-harness
        </span>
      </div>
      <button
        onClick={onToggle}
        aria-label={t("sidebar.collapse")}
        className="focus-ring shrink-0 rounded-lg p-1.5 text-fg-muted transition-colors hover:neu-pressed hover:text-fg"
        title={t("sidebar.collapse")}
      >
        <PanelLeftClose className="h-5 w-5" aria-hidden="true" />
      </button>
    </div>
  );
}
