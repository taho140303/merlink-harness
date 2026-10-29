import { useState } from "react";
import { Settings } from "lucide-react";
import { cn } from "@/shared/utils/cn";
import { useT } from "@/shared/i18n/useT";
import { SidebarHeader } from "./SidebarHeader";
import { SidebarNav } from "./SidebarNav";
import { SidebarHistory } from "./SidebarHistory";
import { SettingsDialog } from "./SettingsDialog";

export default function Sidebar() {
  const t = useT();
  const [collapsed, setCollapsed] = useState(false);
  const [settingsOpen, setSettingsOpen] = useState(false);

  return (
    <>
      <aside
        className={cn(
          "relative flex h-full flex-col rounded-shell-sm neu-flat transition-all duration-300 ease-in-out select-none",
          collapsed ? "w-19 px-2 py-4" : "w-64 px-3 py-4"
        )}
      >
        <SidebarHeader
          collapsed={collapsed}
          onToggle={() => setCollapsed(!collapsed)}
        />

        <nav className="no-scrollbar mt-4 flex-1 space-y-1 overflow-y-auto pr-0.5">
          <SidebarNav collapsed={collapsed} />
          <SidebarHistory
            collapsed={collapsed}
            onExpand={() => setCollapsed(false)}
          />
        </nav>

        <div className="shrink-0 pt-3">
          <button
            onClick={() => setSettingsOpen(true)}
            className={cn(
              "focus-ring flex w-full items-center gap-3 rounded-xl px-3 py-2.5 text-sm font-medium text-fg-muted transition-colors hover:text-fg hover:neu-pressed",
              collapsed && "justify-center px-0"
            )}
            title={t("sidebar.settings")}
          >
            <Settings className="h-5 w-5 shrink-0" />
            {!collapsed && <span>{t("sidebar.settings")}</span>}
          </button>
        </div>
      </aside>

      <SettingsDialog open={settingsOpen} onOpenChange={setSettingsOpen} />
    </>
  );
}
