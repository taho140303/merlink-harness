import { useState } from "react";
import { NavLink, useLocation } from "react-router";
import {
  MessageSquarePlus,
  FolderGit2,
  Users,
  Globe,
  Braces,
  ChevronDown,
  BookOpen,
  Sliders,
  Trash2,
} from "lucide-react";
import { cn } from "../../../shared/utils/cn";

interface SidebarNavProps {
  collapsed: boolean;
}

export function SidebarNav({ collapsed }: SidebarNavProps) {
  const [worldbuildingOpen, setWorldbuildingOpen] = useState(true);
  const location = useLocation();
  const isWorldbuildingActive = location.pathname.startsWith("/worldbuilding");

  return (
    <div className="space-y-1">
      <NavLink
        to="/"
        end
        className={({ isActive }) =>
          cn(
            "flex w-full items-center gap-3 rounded-xl px-3 py-2.5 text-sm font-medium transition-colors",
            isActive
              ? "bg-slate-100 text-slate-900 dark:bg-slate-800 dark:text-white"
              : "text-slate-600 hover:bg-slate-50 hover:text-slate-900 dark:text-slate-400 dark:hover:bg-slate-900 dark:hover:text-slate-200",
            collapsed && "justify-center px-0"
          )
        }
        title={collapsed ? "New chat" : undefined}
      >
        <MessageSquarePlus className="h-5 w-5 shrink-0" />
        {!collapsed && <span>New chat</span>}
      </NavLink>

      <NavLink
        to="/project"
        className={({ isActive }) =>
          cn(
            "flex w-full items-center gap-3 rounded-xl px-3 py-2.5 text-sm font-medium transition-colors",
            isActive
              ? "bg-slate-100 text-slate-900 dark:bg-slate-800 dark:text-white"
              : "text-slate-600 hover:bg-slate-50 hover:text-slate-900 dark:text-slate-400 dark:hover:bg-slate-900 dark:hover:text-slate-200",
            collapsed && "justify-center px-0"
          )
        }
        title={collapsed ? "Project" : undefined}
      >
        <FolderGit2 className="h-5 w-5 shrink-0" />
        {!collapsed && <span>Project</span>}
      </NavLink>

      <NavLink
        to="/character"
        className={({ isActive }) =>
          cn(
            "flex w-full items-center gap-3 rounded-xl px-3 py-2.5 text-sm font-medium transition-colors",
            isActive
              ? "bg-slate-100 text-slate-900 dark:bg-slate-800 dark:text-white"
              : "text-slate-600 hover:bg-slate-50 hover:text-slate-900 dark:text-slate-400 dark:hover:bg-slate-900 dark:hover:text-slate-200",
            collapsed && "justify-center px-0"
          )
        }
        title={collapsed ? "Character" : undefined}
      >
        <Users className="h-5 w-5 shrink-0" />
        {!collapsed && <span>Character</span>}
      </NavLink>

      {/* Worldbuilding Accordion */}
      <div>
        <button
          type="button"
          onClick={() => setWorldbuildingOpen((prev) => !prev)}
          className={cn(
            "flex w-full items-center justify-between rounded-xl px-3 py-2.5 text-sm font-medium transition-colors",
            isWorldbuildingActive
              ? "text-slate-900 dark:text-slate-100 font-semibold"
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
            <NavLink
              to="/worldbuilding/lore"
              className={({ isActive }) =>
                cn(
                  "flex w-full items-center gap-2 rounded-lg px-2.5 py-2 text-xs font-medium transition-colors",
                  isActive
                    ? "bg-slate-100 font-semibold text-slate-900 dark:bg-slate-800 dark:text-white"
                    : "text-slate-500 hover:bg-slate-50 hover:text-slate-800 dark:text-slate-400 dark:hover:bg-slate-900 dark:hover:text-slate-200"
                )
              }
            >
              <BookOpen className="h-4 w-4" />
              <span>Lore</span>
            </NavLink>
            <NavLink
              to="/worldbuilding/settings"
              className={({ isActive }) =>
                cn(
                  "flex w-full items-center gap-2 rounded-lg px-2.5 py-2 text-xs font-medium transition-colors",
                  isActive
                    ? "bg-slate-100 font-semibold text-slate-900 dark:bg-slate-800 dark:text-white"
                    : "text-slate-500 hover:bg-slate-50 hover:text-slate-800 dark:text-slate-400 dark:hover:bg-slate-900 dark:hover:text-slate-200"
                )
              }
            >
              <Sliders className="h-4 w-4" />
              <span>Settings</span>
            </NavLink>
          </div>
        )}
      </div>

      <NavLink
        to="/config"
        className={({ isActive }) =>
          cn(
            "flex w-full items-center gap-3 rounded-xl px-3 py-2.5 text-sm font-medium transition-colors",
            isActive
              ? "bg-slate-100 text-slate-900 dark:bg-slate-800 dark:text-white"
              : "text-slate-600 hover:bg-slate-50 hover:text-slate-900 dark:text-slate-400 dark:hover:bg-slate-900 dark:hover:text-slate-200",
            collapsed && "justify-center px-0"
          )
        }
        title={collapsed ? "System Prompt" : undefined}
      >
        <Braces className="h-5 w-5 shrink-0" />
        {!collapsed && <span>System Prompt</span>}
      </NavLink>

      <NavLink
        to="/recycle-bin"
        className={({ isActive }) =>
          cn(
            "flex w-full items-center gap-3 rounded-xl px-3 py-2.5 text-sm font-medium transition-colors",
            isActive
              ? "bg-rose-50 text-rose-700 dark:bg-rose-950/40 dark:text-rose-300"
              : "text-slate-600 hover:bg-slate-50 hover:text-slate-900 dark:text-slate-400 dark:hover:bg-slate-900 dark:hover:text-slate-200",
            collapsed && "justify-center px-0"
          )
        }
        title={collapsed ? "Recycle Bin" : undefined}
      >
        <Trash2 className="h-5 w-5 shrink-0" />
        {!collapsed && <span>Recycle bin</span>}
      </NavLink>
    </div>
  );
}
