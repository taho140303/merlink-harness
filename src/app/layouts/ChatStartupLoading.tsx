import { useT } from "@/shared/i18n/useT";

export function ChatStartupLoading() {
  const t = useT();
  return (
    <div role="status" aria-live="polite" className="absolute inset-0 z-20 flex flex-col items-center justify-center rounded-shell bg-surface-muted/80 backdrop-blur-md">
      <img src="/logo.svg" alt="" aria-hidden="true" className="mb-6 h-24 w-24 object-contain animate-logo-float" />
      <h1 className="mb-2 px-6 text-center text-xl font-bold text-fg">{t("chat.startup.title")}</h1>
      <p className="px-6 text-center text-base font-medium text-indigo-600 dark:text-indigo-400 animate-pulse">{t("chat.startup.status")}</p>
    </div>
  );
}
