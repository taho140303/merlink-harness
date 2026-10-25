import {
  MessageSquarePlus,
  FolderGit2,
  Users,
  Globe,
  Wrench,
  ChevronDown,
  BookOpen,
  Sliders,
} from "lucide-react";
import { cn } from "../../../shared/utils/cn";

interface SidebarNavProps {
  collapsed: boolean;
  activeTab: string;
  onSelectTab: (tab: string) => void;
  worldbuildingOpen: boolean;
  onToggleWorldbuilding: () => void;
}

export function SidebarNav({
  collapsed,
  activeTab,
  onSelectTab,
  worldbuildingOpen,
  onToggleWorldbuilding,
}: SidebarNavProps) {
  const primaryNavItems = [
    { id: "new-chat", label: "New chat", icon: MessageSquarePlus },
    { id: "project", label: "Project", icon: FolderGit2 },
    { id: "character", label: "Character", icon: Users },
  ];

  return (
    <div className="space-y-1">
      {primaryNavItems.map(({ id, label, icon: Icon }) => (
        <button
          key={id}
          onClick={() => onSelectTab(id)}
          className={cn(
            "flex w-full items-center gap-3 rounded-xl px-3 py-2.5 text-sm font-medium transition-colors",
            activeTab === id
              ? "bg-slate-100 text-slate-900 dark:bg-slate-800 dark:text-white"
              : "text-slate-600 hover:bg-slate-50 hover:text-slate-900 dark:text-slate-400 dark:hover:bg-slate-900 dark:hover:text-slate-200",
            collapsed && "justify-center px-0"
          )}
          title={collapsed ? label : undefined}
        >
          <Icon className="h-5 w-5 shrink-0" />
          {!collapsed && <span>{label}</span>}
        </button>
      ))}

      {/* Worldbuilding Accordion */}
      <div>
        <button
          onClick={onToggleWorldbuilding}
          className={cn(
            "flex w-full items-center justify-between rounded-xl px-3 py-2.5 text-sm font-medium transition-colors",
            activeTab.startsWith("worldbuilding")
              ? "text-slate-900 dark:text-slate-100"
              : "text-slate-600 hover:bg-slate-50 hover:text-slate-900 dark:text-slate-400 dark:hover:bg-slate-900 dark:hover:text-slate-200",
            collapsed && "justify-center px-0"
          )}
          title={collapsed ? "Worldbuilding" : undefined}
        >
          <div className="flex items-center gap-3">
            <Globe className="h-5 w-5 shrink-0" />
            {!collapsed && <span>Worldbuilding</span>}
          </div>
          {!collapsed && (
            <ChevronDown
              className={cn(
                "h-4 w-4 shrink-0 transition-transform duration-200",
                worldbuildingOpen && "rotate-180"
              )}
            />
          )}
        </button>

        {!collapsed && worldbuildingOpen && (
          <div className="ml-5 mt-1 space-y-1 border-l-2 border-slate-100 pl-3 dark:border-slate-800">
            <button
              onClick={() => onSelectTab("worldbuilding-lore")}
              className={cn(
                "flex w-full items-center gap-2 rounded-lg px-2.5 py-2 text-xs font-medium transition-colors",
                activeTab === "worldbuilding-lore"
                  ? "bg-slate-100 font-semibold text-slate-900 dark:bg-slate-800 dark:text-white"
                  : "text-slate-500 hover:bg-slate-50 hover:text-slate-800 dark:text-slate-400 dark:hover:bg-slate-900 dark:hover:text-slate-200"
              )}
            >
              <BookOpen className="h-4 w-4" />
              <span>Lore</span>
            </button>
            <button
              onClick={() => onSelectTab("worldbuilding-settings")}
              className={cn(
                "flex w-full items-center gap-2 rounded-lg px-2.5 py-2 text-xs font-medium transition-colors",
                activeTab === "worldbuilding-settings"
                  ? "bg-slate-100 font-semibold text-slate-900 dark:bg-slate-800 dark:text-white"
                  : "text-slate-500 hover:bg-slate-50 hover:text-slate-800 dark:text-slate-400 dark:hover:bg-slate-900 dark:hover:text-slate-200"
              )}
            >
              <Sliders className="h-4 w-4" />
              <span>Settings</span>
            </button>
          </div>
        )}
      </div>

      {/* Config */}
      <button
        onClick={() => onSelectTab("config")}
        className={cn(
          "flex w-full items-center gap-3 rounded-xl px-3 py-2.5 text-sm font-medium transition-colors",
          activeTab === "config"
            ? "bg-slate-100 text-slate-900 dark:bg-slate-800 dark:text-white"
            : "text-slate-600 hover:bg-slate-50 hover:text-slate-900 dark:text-slate-400 dark:hover:bg-slate-900 dark:hover:text-slate-200",
          collapsed && "justify-center px-0"
        )}
        title={collapsed ? "Config" : undefined}
      >
        <Wrench className="h-5 w-5 shrink-0" />
        {!collapsed && <span>Config</span>}
      </button>
    </div>
  );
}
