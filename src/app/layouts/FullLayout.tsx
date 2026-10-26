import { Outlet } from "react-router";
import Sidebar from "../../core/components/Sidebar";
import { ToastViewport } from "../../shared/components/ui/toast";
import { AuroraBackground } from "../../shared/components/background/AuroraBackground";

export default function FullLayout() {
  return (
    <div className="relative flex h-screen w-full overflow-hidden bg-transparent p-6 gap-8 text-slate-900 dark:text-slate-100 selection:bg-pink-200 dark:selection:bg-purple-900 z-0">
      <AuroraBackground />
      
      {/* Sidebar Container */}
      <div className="relative z-10 h-full shrink-0">
        <Sidebar />
      </div>
      
      {/* Main Container */}
      <main className="relative z-10 flex-1 h-full overflow-hidden">
        <div className="h-full w-full overflow-hidden rounded-3xl neu-flat">
          <Outlet />
        </div>
      </main>
      
      <ToastViewport />
    </div>
  );
}
