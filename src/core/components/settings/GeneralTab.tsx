import { Sun, Moon, Laptop, Languages } from "lucide-react";
import { cn } from "../../../shared/utils/cn";
import { useTheme } from "../../../shared/hooks/useTheme";
import { useLocale } from "../../../shared/i18n/store";
import { useT } from "../../../shared/i18n/useT";

export function GeneralTab() {
  const { theme, setTheme } = useTheme();
  const { locale, setLocale } = useLocale();
  const t = useT();

  return (
    <div className="space-y-6">
      <div>
        <h3 className="text-base font-semibold text-fg">
          {t("settings.general.title")}
        </h3>
        <p className="text-xs text-fg-muted">
          {t("settings.general.subtitle")}
        </p>
      </div>

      <div className="space-y-2">
        <label className="text-xs font-semibold uppercase tracking-wider text-fg-muted">
          {t("settings.appearance")}
        </label>
        <div className="grid max-w-sm grid-cols-3 gap-2 rounded-xl bg-surface-muted p-1">
          <button
            type="button"
            onClick={() => setTheme("light")}
            className={cn(
              "focus-ring flex items-center justify-center gap-2 rounded-lg py-2 text-xs font-medium transition-colors",
              theme === "light"
                ? "bg-surface-card text-fg"
                : "text-fg-muted hover:text-fg",
            )}
          >
            <Sun className="h-4 w-4" />
            <span>{t("settings.appearance.light")}</span>
          </button>
          <button
            type="button"
            onClick={() => setTheme("dark")}
            className={cn(
              "focus-ring flex items-center justify-center gap-2 rounded-lg py-2 text-xs font-medium transition-colors",
              theme === "dark"
                ? "bg-surface-card text-fg"
                : "text-fg-muted hover:text-fg",
            )}
          >
            <Moon className="h-4 w-4" />
            <span>{t("settings.appearance.dark")}</span>
          </button>
          <button
            type="button"
            onClick={() => setTheme("system")}
            className={cn(
              "focus-ring flex items-center justify-center gap-2 rounded-lg py-2 text-xs font-medium transition-colors",
              theme === "system"
                ? "bg-surface-card text-fg"
                : "text-fg-muted hover:text-fg",
            )}
          >
            <Laptop className="h-4 w-4" />
            <span>{t("settings.appearance.system")}</span>
          </button>
        </div>
      </div>

      <div className="space-y-2">
        <label className="text-xs font-semibold uppercase tracking-wider text-fg-muted">
          {t("settings.language")}
        </label>
        <div className="grid max-w-sm grid-cols-2 gap-2 rounded-xl bg-surface-muted p-1">
          <button
            type="button"
            onClick={() => setLocale("en")}
            className={cn(
              "focus-ring flex items-center justify-center gap-2 rounded-lg py-2 text-xs font-medium transition-colors",
              locale === "en"
                ? "bg-surface-card text-fg"
                : "text-fg-muted hover:text-fg",
            )}
          >
            <Languages className="h-4 w-4" />
            <span>{t("settings.language.en")}</span>
          </button>
          <button
            type="button"
            onClick={() => setLocale("vi")}
            className={cn(
              "focus-ring flex items-center justify-center gap-2 rounded-lg py-2 text-xs font-medium transition-colors",
              locale === "vi"
                ? "bg-surface-card text-fg"
                : "text-fg-muted hover:text-fg",
            )}
          >
            <Languages className="h-4 w-4" />
            <span>{t("settings.language.vi")}</span>
          </button>
        </div>
      </div>
    </div>
  );
}
