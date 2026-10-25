import { useState } from "react";
import { Outlet } from "react-router";
import Sidebar from "../components/Sidebar";

export interface OutletContextType {
  activeTab: string;
  setActiveTab: (tab: string) => void;
}

export default function FullLayout() {
  const [activeTab, setActiveTab] = useState("new-chat");

  return (
    <div className="flex min-h-screen bg-slate-50 text-slate-900 dark:bg-slate-900 dark:text-slate-100">
      <Sidebar activeTab={activeTab} onSelectTab={setActiveTab} />
      <main className="flex-1 overflow-hidden">
        <Outlet context={{ activeTab, setActiveTab } satisfies OutletContextType} />
      </main>
    </div>
  );
}
