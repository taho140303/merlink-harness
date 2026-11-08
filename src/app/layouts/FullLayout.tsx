import { useCallback, useState } from "react";
import { Outlet, useLocation } from "react-router";
import { Sidebar } from "@/features/navigation";
import { ToastViewport } from "@/shared/components/ui/toast";
import { AuroraBackground } from "@/shared/components/background/AuroraBackground";
import { useT } from "@/shared/i18n/useT";
import { useZoomHotkeys } from "@/shared/hooks/useZoomHotkeys";

export interface ChatStartupContext {
  finishChatStartup: () => void;
  beginChatStartup: () => void;
}

export default function FullLayout() {
  const location = useLocation();
  const t = useT();
  useZoomHotkeys();
  const isChatRoute = location.pathname === "/" || /^\/chat\/[^/]+\/?$/.test(location.pathname);
  // The location key makes a new chat navigation start behind the gate immediately,
  // without waiting for an effect (and without hiding unrelated routes).
  const [finishedKey, setFinishedKey] = useState<string | null>(null);
  const isStarting = isChatRoute && finishedKey !== location.key;
  const finishChatStartup = useCallback(() => setFinishedKey(location.key), [location.key]);
  const beginChatStartup = useCallback(() => setFinishedKey(null), []);

  return (
    <div className="relative flex h-screen w-full overflow-hidden p-4 gap-4 text-fg selection:bg-pink-200 dark:selection:bg-purple-900 z-0">
      <AuroraBackground />

      {/* Keep the chat mounted so its backend initialization can complete behind the gate. */}
      <div aria-hidden={isStarting} className={`relative z-10 h-full shrink-0 ${isStarting ? "invisible pointer-events-none" : ""}`}>
        <Sidebar />
      </div>

      {/* Main Container: Tier 1 — neumorphic shell */}
      <main aria-hidden={isStarting} className={`relative z-10 flex-1 h-full overflow-hidden rounded-shell neu-flat ${isStarting ? "invisible pointer-events-none" : ""}`}>
        <Outlet key={isChatRoute ? location.key : undefined} context={{ finishChatStartup, beginChatStartup } satisfies ChatStartupContext} />
      </main>

      {isStarting && (
        <div role="status" aria-live="polite" className="absolute inset-0 z-20 flex flex-col items-center justify-center bg-surface-muted/80 backdrop-blur-md">
          <img src="/logo.svg" alt="" aria-hidden="true" className="mb-6 h-24 w-24 object-contain animate-logo-float" />
          <h1 className="mb-2 px-6 text-center text-xl font-bold text-fg">{t("chat.startup.title")}</h1>
          <p className="px-6 text-center text-base font-medium text-indigo-600 dark:text-indigo-400 animate-pulse">{t("chat.startup.status")}</p>
        </div>
      )}

      <ToastViewport />
    </div>
  );
}
