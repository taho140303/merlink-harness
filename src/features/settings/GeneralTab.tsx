import { Sun, Moon, Laptop, Languages } from "lucide-react";
import { Segmented } from "@/shared/components/ui/segmented";
import { useTheme } from "@/shared/hooks/useTheme";
import { useLocale } from "@/shared/i18n/store";
import { useT } from "@/shared/i18n/useT";

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
        <Segmented
          className="grid max-w-sm grid-cols-3"
          ariaLabel={t("settings.appearance")}
          value={theme}
          onChange={(value) => setTheme(value as typeof theme)}
          items={[
            { id: "light", icon: <Sun className="h-4 w-4" />, label: <span>{t("settings.appearance.light")}</span> },
            { id: "dark", icon: <Moon className="h-4 w-4" />, label: <span>{t("settings.appearance.dark")}</span> },
            { id: "system", icon: <Laptop className="h-4 w-4" />, label: <span>{t("settings.appearance.system")}</span> },
          ]}
        />
      </div>

      <div className="space-y-2">
        <label className="text-xs font-semibold uppercase tracking-wider text-fg-muted">
          {t("settings.language")}
        </label>
        <Segmented
          className="grid max-w-sm grid-cols-2"
          ariaLabel={t("settings.language")}
          value={locale}
          onChange={(value) => setLocale(value as typeof locale)}
          items={[
            { id: "en", icon: <Languages className="h-4 w-4" />, label: <span>{t("settings.language.en")}</span> },
            { id: "vi", icon: <Languages className="h-4 w-4" />, label: <span>{t("settings.language.vi")}</span> },
          ]}
        />
      </div>
    </div>
  );
}
