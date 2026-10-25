import { Sliders, Cpu } from "lucide-react";
import { cn } from "../../../shared/utils/cn";
import { useT } from "../../../shared/i18n/useT";
import type { SettingsTabId } from "./types";

interface SettingsSidebarProps {
  activeTab: SettingsTabId;
  onSelectTab: (tab: SettingsTabId) => void;
}

export function SettingsSidebar({ activeTab, onSelectTab }: SettingsSidebarProps) {
  const t = useT();
  const tabs = [
    { id: "general" as SettingsTabId, label: t("settings.general"), icon: Sliders },
    { id: "ai-providers" as SettingsTabId, label: t("settings.aiProviders"), icon: Cpu },
  ];

  return (
    <aside className="w-52 shrink-0 border-r border-slate-200 bg-slate-50/70 p-3 dark:border-slate-800 dark:bg-slate-950/50">
      <div className="mb-4 px-2 pt-2">
        <h2 className="text-base font-semibold text-slate-900 dark:text-slate-100">
          {t("settings.title")}
        </h2>
        <p className="text-xs text-slate-500 dark:text-slate-400">
          {t("settings.subtitle")}
        </p>
      </div>

      <nav className="space-y-1">
        {tabs.map(({ id, label, icon: Icon }) => (
          <button
            key={id}
            type="button"
            onClick={() => onSelectTab(id)}
            className={cn(
              "flex w-full items-center gap-2.5 rounded-xl px-3 py-2 text-xs font-medium transition-colors",
              activeTab === id
                ? "bg-white text-slate-900 shadow-xs dark:bg-slate-800 dark:text-white"
                : "text-slate-600 hover:bg-slate-200/60 hover:text-slate-900 dark:text-slate-400 dark:hover:bg-slate-900 dark:hover:text-slate-200",
            )}
          >
            <Icon className="h-4 w-4" />
            <span>{label}</span>
          </button>
        ))}
      </nav>
    </aside>
  );
}
