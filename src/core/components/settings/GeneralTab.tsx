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
        <h3 className="text-base font-semibold text-slate-900 dark:text-slate-100">
          {t("settings.general.title")}
        </h3>
        <p className="text-xs text-slate-500 dark:text-slate-400">
          {t("settings.general.subtitle")}
        </p>
      </div>

      <div className="space-y-2">
        <label className="text-xs font-semibold uppercase tracking-wider text-slate-500 dark:text-slate-400">
          {t("settings.appearance")}
        </label>
        <div className="grid max-w-sm grid-cols-3 gap-2 rounded-xl bg-slate-100 p-1 dark:bg-slate-800">
          <button
            type="button"
            onClick={() => setTheme("light")}
            className={cn(
              "flex items-center justify-center gap-2 rounded-lg py-2 text-xs font-medium transition-all",
              theme === "light"
                ? "bg-white text-slate-900 shadow-xs dark:bg-slate-700 dark:text-white"
                : "text-slate-600 hover:text-slate-900 dark:text-slate-400 dark:hover:text-slate-100",
            )}
          >
            <Sun className="h-4 w-4" />
            <span>{t("settings.appearance.light")}</span>
          </button>
          <button
            type="button"
            onClick={() => setTheme("dark")}
            className={cn(
              "flex items-center justify-center gap-2 rounded-lg py-2 text-xs font-medium transition-all",
              theme === "dark"
                ? "bg-white text-slate-900 shadow-xs dark:bg-slate-700 dark:text-white"
                : "text-slate-600 hover:text-slate-900 dark:text-slate-400 dark:hover:text-slate-100",
            )}
          >
            <Moon className="h-4 w-4" />
            <span>{t("settings.appearance.dark")}</span>
          </button>
          <button
            type="button"
            onClick={() => setTheme("system")}
            className={cn(
              "flex items-center justify-center gap-2 rounded-lg py-2 text-xs font-medium transition-all",
              theme === "system"
                ? "bg-white text-slate-900 shadow-xs dark:bg-slate-700 dark:text-white"
                : "text-slate-600 hover:text-slate-900 dark:text-slate-400 dark:hover:text-slate-100",
            )}
          >
            <Laptop className="h-4 w-4" />
            <span>{t("settings.appearance.system")}</span>
          </button>
        </div>
      </div>

      <div className="space-y-2">
        <label className="text-xs font-semibold uppercase tracking-wider text-slate-500 dark:text-slate-400">
          {t("settings.language")}
        </label>
        <div className="grid max-w-sm grid-cols-2 gap-2 rounded-xl bg-slate-100 p-1 dark:bg-slate-800">
          <button
            type="button"
            onClick={() => setLocale("en")}
            className={cn(
              "flex items-center justify-center gap-2 rounded-lg py-2 text-xs font-medium transition-all",
              locale === "en"
                ? "bg-white text-slate-900 shadow-xs dark:bg-slate-700 dark:text-white"
                : "text-slate-600 hover:text-slate-900 dark:text-slate-400 dark:hover:text-slate-100",
            )}
          >
            <Languages className="h-4 w-4" />
            <span>{t("settings.language.en")}</span>
          </button>
          <button
            type="button"
            onClick={() => setLocale("vi")}
            className={cn(
              "flex items-center justify-center gap-2 rounded-lg py-2 text-xs font-medium transition-all",
              locale === "vi"
                ? "bg-white text-slate-900 shadow-xs dark:bg-slate-700 dark:text-white"
                : "text-slate-600 hover:text-slate-900 dark:text-slate-400 dark:hover:text-slate-100",
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
