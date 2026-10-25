import { Outlet } from "react-router";
import Sidebar from "../../core/components/Sidebar";
import { ToastViewport } from "../../shared/components/ui/toast";

export default function FullLayout() {
  return (
    <div className="flex h-screen w-full overflow-hidden bg-slate-50 text-slate-900 dark:bg-slate-900 dark:text-slate-100">
      <Sidebar />
      <main className="flex-1 h-full overflow-hidden">
        <Outlet />
      </main>
      <ToastViewport />
    </div>
  );
}
