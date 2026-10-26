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
} from "lucide-react";
import { cn } from "../../../shared/utils/cn";
import { useT } from "../../../shared/i18n/useT";

interface SidebarNavProps {
  collapsed: boolean;
}

export function SidebarNav({ collapsed }: SidebarNavProps) {
  const t = useT();
  const [worldbuildingOpen, setWorldbuildingOpen] = useState(true);
  const location = useLocation();
  const isWorldbuildingActive = location.pathname.startsWith("/worldbuilding");

  const activeClass = "neu-pressed text-slate-900 dark:text-white font-semibold";
  const inactiveClass = "text-slate-600 hover:neu-flat hover:text-slate-900 dark:text-slate-400 dark:hover:text-slate-200";

  return (
    <div className="space-y-2">
      <NavLink
        to="/"
        end
        className={({ isActive }) =>
          cn(
            "flex w-full items-center gap-3 rounded-xl px-3 py-2.5 text-sm font-medium transition-all",
            isActive ? activeClass : inactiveClass,
            collapsed && "justify-center px-0"
          )
        }
        title={collapsed ? t("sidebar.newChat") : undefined}
      >
        <MessageSquarePlus className="h-5 w-5 shrink-0" />
        {!collapsed && <span>{t("sidebar.newChat")}</span>}
      </NavLink>

      <NavLink
        to="/project"
        className={({ isActive }) =>
          cn(
            "flex w-full items-center gap-3 rounded-xl px-3 py-2.5 text-sm font-medium transition-all",
            isActive ? activeClass : inactiveClass,
            collapsed && "justify-center px-0"
          )
        }
        title={collapsed ? t("sidebar.project") : undefined}
      >
        <FolderGit2 className="h-5 w-5 shrink-0" />
        {!collapsed && <span>{t("sidebar.project")}</span>}
      </NavLink>

      <NavLink
        to="/character"
        className={({ isActive }) =>
          cn(
            "flex w-full items-center gap-3 rounded-xl px-3 py-2.5 text-sm font-medium transition-all",
            isActive ? activeClass : inactiveClass,
            collapsed && "justify-center px-0"
          )
        }
        title={collapsed ? t("sidebar.character") : undefined}
      >
        <Users className="h-5 w-5 shrink-0" />
        {!collapsed && <span>{t("sidebar.character")}</span>}
      </NavLink>

      {/* Worldbuilding Accordion */}
      <div>
        <button
          type="button"
          onClick={() => setWorldbuildingOpen((prev) => !prev)}
          className={cn(
            "flex w-full items-center justify-between rounded-xl px-3 py-2.5 text-sm font-medium transition-all",
            isWorldbuildingActive ? activeClass : inactiveClass,
            collapsed && "justify-center px-0"
          )}
          title={collapsed ? t("sidebar.worldbuilding") : undefined}
        >
          <div className="flex items-center gap-3">
            <Globe className="h-5 w-5 shrink-0" />
            {!collapsed && <span>{t("sidebar.worldbuilding")}</span>}
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
                  "flex w-full items-center gap-2 rounded-lg px-2.5 py-2 text-xs font-medium transition-all",
                  isActive ? activeClass : inactiveClass
                )
              }
            >
              <BookOpen className="h-4 w-4" />
              <span>{t("sidebar.lore")}</span>
            </NavLink>
            <NavLink
              to="/worldbuilding/settings"
              className={({ isActive }) =>
                cn(
                  "flex w-full items-center gap-2 rounded-lg px-2.5 py-2 text-xs font-medium transition-all",
                  isActive ? activeClass : inactiveClass
                )
              }
            >
              <Sliders className="h-4 w-4" />
              <span>{t("sidebar.worldSettings")}</span>
            </NavLink>
          </div>
        )}
      </div>

      <NavLink
        to="/config"
        className={({ isActive }) =>
          cn(
            "flex w-full items-center gap-3 rounded-xl px-3 py-2.5 text-sm font-medium transition-all",
            isActive ? activeClass : inactiveClass,
            collapsed && "justify-center px-0"
          )
        }
        title={collapsed ? t("sidebar.systemPrompt") : undefined}
      >
        <Braces className="h-5 w-5 shrink-0" />
        {!collapsed && <span>{t("sidebar.systemPrompt")}</span>}
      </NavLink>
    </div>
  );
}
