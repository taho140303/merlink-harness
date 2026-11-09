import { useCallback, useState } from "react";
import { Outlet, useLocation } from "react-router";
import { Sidebar } from "@/features/navigation";
import { ToastViewport } from "@/shared/components/ui/toast";
import { AuroraBackground } from "@/shared/components/background/AuroraBackground";
import { useZoomHotkeys } from "@/shared/hooks/useZoomHotkeys";
import { ChatStartupLoading } from "./ChatStartupLoading";

export interface ChatStartupContext {
  finishChatStartup: () => void;
  beginChatStartup: () => void;
}

export default function FullLayout() {
  const location = useLocation();
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

      <div className="relative z-10 h-full shrink-0">
        <Sidebar />
      </div>

      {/* Main Container: Tier 1 — neumorphic shell */}
      <main className="relative z-10 flex-1 h-full overflow-hidden rounded-shell neu-flat">
        {/* Keep the chat mounted while its backend initialization runs. */}
        <div aria-hidden={isStarting} className={`h-full ${isStarting ? "invisible pointer-events-none" : ""}`}>
          <Outlet key={isChatRoute ? location.key : undefined} context={{ finishChatStartup, beginChatStartup } satisfies ChatStartupContext} />
        </div>
        {isStarting && <ChatStartupLoading />}
      </main>

      <ToastViewport />
    </div>
  );
}
