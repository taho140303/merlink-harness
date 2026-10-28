import { Sliders, Cpu, Trash2 } from "lucide-react";
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
    { id: "recycle-bin" as SettingsTabId, label: t("settings.recycleBin"), icon: Trash2 },
  ];

  return (
    <aside className="w-52 shrink-0 border-r border-line bg-surface-muted/70 p-3">
      <div className="mb-4 px-2 pt-2">
        <h2 className="text-base font-semibold text-fg">
          {t("settings.title")}
        </h2>
        <p className="text-xs text-fg-muted">
          {t("settings.subtitle")}
        </p>
      </div>

      <nav className="space-y-1">
        {tabs.map(({ id, label, icon: Icon }) => (
          <button
            key={id}
            type="button"
            onClick={() => onSelectTab(id)}
            aria-current={activeTab === id ? 'page' : undefined}
            className={cn(
              "focus-ring flex w-full items-center gap-2.5 rounded-xl px-3 py-2 text-xs font-medium transition-colors",
              activeTab === id
                ? "bg-surface-card text-fg"
                : "text-fg-muted hover:bg-surface-muted hover:text-fg",
            )}
          >
            <Icon className="h-4 w-4" aria-hidden="true" />
            <span>{label}</span>
          </button>
        ))}
      </nav>
    </aside>
  );
}
