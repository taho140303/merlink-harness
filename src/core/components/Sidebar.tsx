import { useState } from "react";
import { Settings } from "lucide-react";
import { cn } from "../../shared/utils/cn";
import { SidebarHeader } from "./sidebar/SidebarHeader";
import { SidebarNav } from "./sidebar/SidebarNav";
import { SidebarHistory } from "./sidebar/SidebarHistory";
import { SettingsDialog } from "./sidebar/SettingsDialog";

export default function Sidebar() {
  const [collapsed, setCollapsed] = useState(false);
  const [settingsOpen, setSettingsOpen] = useState(false);

  return (
    <>
      <aside
        className={cn(
          "relative flex h-screen flex-col border-r border-slate-200 bg-white transition-all duration-300 ease-in-out select-none dark:border-slate-800 dark:bg-slate-950",
          collapsed ? "w-18 px-2" : "w-64 px-3"
        )}
      >
        <SidebarHeader
          collapsed={collapsed}
          onToggle={() => setCollapsed(!collapsed)}
        />

        <nav className="mt-4 flex-1 space-y-1 overflow-y-auto pr-0.5">
          <SidebarNav collapsed={collapsed} />
          <SidebarHistory
            collapsed={collapsed}
            onExpand={() => setCollapsed(false)}
          />
        </nav>

        <div className="shrink-0 border-t border-slate-100 py-3 dark:border-slate-800/80">
          <button
            onClick={() => setSettingsOpen(true)}
            className={cn(
              "flex w-full items-center gap-3 rounded-xl px-3 py-2.5 text-sm font-medium text-slate-600 transition-colors hover:bg-slate-100 hover:text-slate-900 dark:text-slate-400 dark:hover:bg-slate-900 dark:hover:text-slate-200",
              collapsed && "justify-center px-0"
            )}
            title="Settings"
          >
            <Settings className="h-5 w-5 shrink-0" />
            {!collapsed && <span>Settings</span>}
          </button>
        </div>
      </aside>

      <SettingsDialog open={settingsOpen} onOpenChange={setSettingsOpen} />
    </>
  );
}
