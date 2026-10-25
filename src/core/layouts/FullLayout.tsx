import { Outlet } from "react-router";
import Sidebar from "../components/Sidebar";

export default function FullLayout() {
  return (
    <div className="flex min-h-screen bg-slate-50 text-slate-900">
      <Sidebar />
      <main className="flex-1 p-6 overflow-auto">
        <Outlet />
      </main>
    </div>
  );
}
