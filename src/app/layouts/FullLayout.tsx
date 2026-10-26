import { Outlet } from "react-router";
import Sidebar from "../../core/components/Sidebar";
import { ToastViewport } from "../../shared/components/ui/toast";
import { AuroraBackground } from "../../shared/components/background/AuroraBackground";

export default function FullLayout() {
  return (
    <div className="relative flex h-screen w-full overflow-hidden p-4 gap-4 text-slate-900 dark:text-slate-100 selection:bg-pink-200 dark:selection:bg-purple-900">
      <AuroraBackground />
      <Sidebar />
      <main className="flex-1 h-full overflow-hidden">
        <div className="h-full w-full overflow-hidden rounded-3xl neu-flat">
          <Outlet />
        </div>
      </main>
      <ToastViewport />
    </div>
  );
}
