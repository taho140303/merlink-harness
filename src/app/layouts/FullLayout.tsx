import { Outlet } from "react-router";
import Sidebar from "../../core/components/Sidebar";
import { ToastViewport } from "../../shared/components/ui/toast";
import { AuroraBackground } from "../../shared/components/background/AuroraBackground";

export default function FullLayout() {
  return (
    <div className="relative flex h-screen w-full overflow-hidden p-4 gap-4 text-fg selection:bg-pink-200 dark:selection:bg-purple-900 z-0">
      <AuroraBackground />
      
      {/* Sidebar Container */}
      <div className="relative z-10 h-full shrink-0">
        <Sidebar />
      </div>
      
      {/* Main Container: Tier 1 — neumorphic shell */}
      <main className="relative z-10 flex-1 h-full overflow-hidden rounded-shell neu-flat">
        <Outlet />
      </main>
      
      <ToastViewport />
    </div>
  );
}
