import { useState } from "react";
import {
  MessageSquarePlus,
  FolderGit2,
  Users,
  Globe,
  Wrench,
  Settings,
  ChevronDown,
  PanelLeftClose,
  PanelLeftOpen,
  BookOpen,
  Sliders,
  Sun,
  Moon,
  Laptop,
  History as HistoryIcon,
  Folder,
} from "lucide-react";
import { cn } from "../../shared/utils/cn";
import { Dialog, DialogContent } from "./ui/dialog";

type ThemeMode = "light" | "dark" | "system";

interface ChatItem {
  id: string;
  title: string;
  time: string;
  project: string;
}

const MOCK_HISTORY: ChatItem[] = [
  { id: "1", title: "Setup Docker and Kube", time: "21h", project: "Merlink Core" },
  { id: "2", title: "Refactor Tailwind v4", time: "1d", project: "Merlink Core" },
  { id: "3", title: "Fix layout route bug", time: "3d", project: "Merlink Harness" },
  { id: "4", title: "World lore brainstorming", time: "1w", project: "Merlink Story" },
  { id: "5", title: "Character stat rules", time: "3w", project: "Merlink Story" },
  { id: "6", title: "Release checklist", time: "2m", project: "Merlink Harness" },
];

function formatTitle(title: string): string {
  return title.length > 12 ? `${title.slice(0, 12)}...` : title;
}

export default function Sidebar() {
  const [collapsed, setCollapsed] = useState(false);
  const [worldbuildingOpen, setWorldbuildingOpen] = useState(true);
  const [activeTab, setActiveTab] = useState("new-chat");
  const [settingsOpen, setSettingsOpen] = useState(false);
  const [theme, setTheme] = useState<ThemeMode>("light");

  // History state
  const [groupByProject, setGroupByProject] = useState(false);
  const [openProjects, setOpenProjects] = useState<Record<string, boolean>>({
    "Merlink Core": true,
    "Merlink Harness": true,
    "Merlink Story": true,
  });

  const toggleProject = (project: string) => {
    setOpenProjects((prev) => ({ ...prev, [project]: !prev[project] }));
  };

  const groupedChats = MOCK_HISTORY.reduce<Record<string, ChatItem[]>>((acc, item) => {
    if (!acc[item.project]) acc[item.project] = [];
    acc[item.project].push(item);
    return acc;
  }, {});

  return (
    <>
      <aside
        className={cn(
          "relative flex h-screen flex-col border-r border-slate-200 bg-white transition-all duration-300 ease-in-out select-none",
          collapsed ? "w-18 px-2" : "w-64 px-3"
        )}
      >
        {/* Header */}
        <div className="flex h-16 shrink-0 items-center justify-between border-b border-slate-100 px-2">
          {collapsed ? (
            <button
              onClick={() => setCollapsed(false)}
              className="group relative mx-auto flex h-10 w-10 items-center justify-center rounded-xl bg-slate-900 text-white shadow-xs transition-colors hover:bg-slate-800"
              title="Mở sidebar"
            >
              <div className="flex items-center justify-center transition-opacity duration-200 group-hover:opacity-0">
                <span className="text-base font-bold">M</span>
              </div>
              <div className="absolute inset-0 flex items-center justify-center opacity-0 transition-opacity duration-200 group-hover:opacity-100">
                <PanelLeftOpen className="h-5 w-5 text-white" />
              </div>
            </button>
          ) : (
            <>
              <div className="flex items-center gap-3 overflow-hidden">
                <div className="flex h-10 w-10 shrink-0 items-center justify-center rounded-xl bg-slate-900 text-white shadow-xs">
                  <span className="text-base font-bold">M</span>
                </div>
                <span className="truncate font-semibold tracking-tight text-slate-900">
                  merlink-harness
                </span>
              </div>
              <button
                onClick={() => setCollapsed(true)}
                className="rounded-lg p-2 text-slate-400 hover:bg-slate-100 hover:text-slate-700"
                title="Thu gọn sidebar"
              >
                <PanelLeftClose className="h-5 w-5" />
              </button>
            </>
          )}
        </div>

        {/* Navigation & History */}
        <nav className="mt-4 flex-1 space-y-1 overflow-y-auto pr-0.5">
          {/* New Chat */}
          <button
            onClick={() => setActiveTab("new-chat")}
            className={cn(
              "flex w-full items-center gap-3 rounded-xl px-3 py-2.5 text-sm font-medium transition-colors",
              activeTab === "new-chat"
                ? "bg-slate-100 text-slate-900"
                : "text-slate-600 hover:bg-slate-50 hover:text-slate-900",
              collapsed && "justify-center px-0"
            )}
            title={collapsed ? "New chat" : undefined}
          >
            <MessageSquarePlus className="h-5 w-5 shrink-0" />
            {!collapsed && <span>New chat</span>}
          </button>

          {/* Project */}
          <button
            onClick={() => setActiveTab("project")}
            className={cn(
              "flex w-full items-center gap-3 rounded-xl px-3 py-2.5 text-sm font-medium transition-colors",
              activeTab === "project"
                ? "bg-slate-100 text-slate-900"
                : "text-slate-600 hover:bg-slate-50 hover:text-slate-900",
              collapsed && "justify-center px-0"
            )}
            title={collapsed ? "Project" : undefined}
          >
            <FolderGit2 className="h-5 w-5 shrink-0" />
            {!collapsed && <span>Project</span>}
          </button>

          {/* Character */}
          <button
            onClick={() => setActiveTab("character")}
            className={cn(
              "flex w-full items-center gap-3 rounded-xl px-3 py-2.5 text-sm font-medium transition-colors",
              activeTab === "character"
                ? "bg-slate-100 text-slate-900"
                : "text-slate-600 hover:bg-slate-50 hover:text-slate-900",
              collapsed && "justify-center px-0"
            )}
            title={collapsed ? "Character" : undefined}
          >
            <Users className="h-5 w-5 shrink-0" />
            {!collapsed && <span>Character</span>}
          </button>

          {/* Worldbuilding (Accordion) */}
          <div>
            <button
              onClick={() => {
                if (collapsed) setCollapsed(false);
                setWorldbuildingOpen(!worldbuildingOpen);
              }}
              className={cn(
                "flex w-full items-center justify-between rounded-xl px-3 py-2.5 text-sm font-medium transition-colors",
                activeTab.startsWith("worldbuilding")
                  ? "text-slate-900"
                  : "text-slate-600 hover:bg-slate-50 hover:text-slate-900",
                collapsed && "justify-center px-0"
              )}
              title={collapsed ? "Worldbuilding" : undefined}
            >
              <div className="flex items-center gap-3">
                <Globe className="h-5 w-5 shrink-0" />
                {!collapsed && <span>Worldbuilding</span>}
              </div>
              {!collapsed && (
                <ChevronDown
                  className={cn(
                    "h-4 w-4 shrink-0 transition-transform duration-200",
                    worldbuildingOpen && "rotate-180"
                  )}
                />
              )}
            </button>

            {!collapsed && worldbuildingOpen && (
              <div className="ml-5 mt-1 space-y-1 border-l-2 border-slate-100 pl-3">
                <button
                  onClick={() => setActiveTab("worldbuilding-lore")}
                  className={cn(
                    "flex w-full items-center gap-2 rounded-lg px-2.5 py-2 text-xs font-medium transition-colors",
                    activeTab === "worldbuilding-lore"
                      ? "bg-slate-100 font-semibold text-slate-900"
                      : "text-slate-500 hover:bg-slate-50 hover:text-slate-800"
                  )}
                >
                  <BookOpen className="h-4 w-4" />
                  <span>Lore</span>
                </button>
                <button
                  onClick={() => setActiveTab("worldbuilding-settings")}
                  className={cn(
                    "flex w-full items-center gap-2 rounded-lg px-2.5 py-2 text-xs font-medium transition-colors",
                    activeTab === "worldbuilding-settings"
                      ? "bg-slate-100 font-semibold text-slate-900"
                      : "text-slate-500 hover:bg-slate-50 hover:text-slate-800"
                  )}
                >
                  <Sliders className="h-4 w-4" />
                  <span>Settings</span>
                </button>
              </div>
            )}
          </div>

          {/* Config */}
          <button
            onClick={() => setActiveTab("config")}
            className={cn(
              "flex w-full items-center gap-3 rounded-xl px-3 py-2.5 text-sm font-medium transition-colors",
              activeTab === "config"
                ? "bg-slate-100 text-slate-900"
                : "text-slate-600 hover:bg-slate-50 hover:text-slate-900",
              collapsed && "justify-center px-0"
            )}
            title={collapsed ? "Config" : undefined}
          >
            <Wrench className="h-5 w-5 shrink-0" />
            {!collapsed && <span>Config</span>}
          </button>

          {/* History Section */}
          <div className="pt-3">
            {collapsed ? (
              <button
                onClick={() => setCollapsed(false)}
                className="flex w-full justify-center rounded-xl p-2.5 text-slate-500 hover:bg-slate-100 hover:text-slate-900 transition-colors"
                title="History"
              >
                <HistoryIcon className="h-5 w-5" />
              </button>
            ) : (
              <>
                <div className="flex items-center justify-between px-2.5 py-1.5">
                  <span className="text-xs font-semibold uppercase tracking-wider text-slate-400">
                    History
                  </span>
                  <button
                    type="button"
                    role="switch"
                    aria-checked={groupByProject}
                    onClick={() => setGroupByProject(!groupByProject)}
                    className={cn(
                      "relative inline-flex h-4 w-7 shrink-0 cursor-pointer items-center rounded-full transition-colors duration-200 ease-in-out focus:outline-hidden",
                      groupByProject ? "bg-slate-900" : "bg-slate-200"
                    )}
                    title="Nhóm theo project"
                  >
                    <span
                      className={cn(
                        "pointer-events-none inline-block h-3 w-3 transform rounded-full bg-white shadow-xs transition duration-200 ease-in-out",
                        groupByProject ? "translate-x-3.5" : "translate-x-0.5"
                      )}
                    />
                  </button>
                </div>

                {/* History Items Container */}
                <div className="mt-1 space-y-0.5">
                  {groupByProject ? (
                    Object.entries(groupedChats).map(([projectName, chats]) => {
                      const isOpen = openProjects[projectName] ?? false;
                      return (
                        <div key={projectName} className="space-y-0.5">
                          <button
                            type="button"
                            onClick={() => toggleProject(projectName)}
                            className="flex w-full items-center justify-between rounded-lg px-2.5 py-1.5 text-xs font-medium text-slate-600 hover:bg-slate-50 hover:text-slate-900 transition-colors"
                          >
                            <div className="flex items-center gap-1.5 truncate">
                              <Folder className="h-3.5 w-3.5 shrink-0 text-slate-400" />
                              <span className="truncate">{projectName}</span>
                            </div>
                            <ChevronDown
                              className={cn(
                                "h-3.5 w-3.5 shrink-0 text-slate-400 transition-transform duration-200",
                                !isOpen && "-rotate-90"
                              )}
                            />
                          </button>

                          {isOpen && (
                            <div className="ml-3 space-y-0.5 border-l border-slate-100 pl-2">
                              {chats.map((chat) => (
                                <button
                                  key={chat.id}
                                  onClick={() => setActiveTab(`chat-${chat.id}`)}
                                  className={cn(
                                    "flex w-full items-center justify-between rounded-md px-2 py-1.5 text-xs transition-colors text-left",
                                    activeTab === `chat-${chat.id}`
                                      ? "bg-slate-100 font-medium text-slate-900"
                                      : "text-slate-500 hover:bg-slate-50 hover:text-slate-900"
                                  )}
                                >
                                  <span className="truncate">{formatTitle(chat.title)}</span>
                                  <span className="shrink-0 text-[10px] text-slate-400">
                                    {chat.time}
                                  </span>
                                </button>
                              ))}
                            </div>
                          )}
                        </div>
                      );
                    })
                  ) : (
                    MOCK_HISTORY.map((chat) => (
                      <button
                        key={chat.id}
                        onClick={() => setActiveTab(`chat-${chat.id}`)}
                        className={cn(
                          "flex w-full items-center justify-between rounded-lg px-2.5 py-1.5 text-xs transition-colors text-left",
                          activeTab === `chat-${chat.id}`
                            ? "bg-slate-100 font-medium text-slate-900"
                            : "text-slate-500 hover:bg-slate-50 hover:text-slate-900"
                        )}
                      >
                        <span className="truncate">{formatTitle(chat.title)}</span>
                        <span className="shrink-0 text-[10px] text-slate-400">{chat.time}</span>
                      </button>
                    ))
                  )}
                </div>
              </>
            )}
          </div>
        </nav>

        {/* Bottom Pinned Settings */}
        <div className="shrink-0 border-t border-slate-100 py-3">
          <button
            onClick={() => setSettingsOpen(true)}
            className={cn(
              "flex w-full items-center gap-3 rounded-xl px-3 py-2.5 text-sm font-medium text-slate-600 transition-colors hover:bg-slate-100 hover:text-slate-900",
              collapsed && "justify-center px-0"
            )}
            title="Settings"
          >
            <Settings className="h-5 w-5 shrink-0" />
            {!collapsed && <span>Settings</span>}
          </button>
        </div>
      </aside>

      {/* Settings Dialog */}
      <Dialog open={settingsOpen} onOpenChange={setSettingsOpen}>
        <DialogContent>
          <div>
            <h2 className="text-lg font-semibold text-slate-900">Settings</h2>
            <p className="text-xs text-slate-500">Tùy chỉnh cấu hình hệ thống của bạn.</p>
          </div>

          <div className="mt-4 space-y-3">
            <label className="text-sm font-medium text-slate-700">Appearance</label>
            <div className="grid grid-cols-3 gap-2 rounded-xl bg-slate-100 p-1">
              <button
                type="button"
                onClick={() => setTheme("light")}
                className={cn(
                  "flex items-center justify-center gap-2 rounded-lg py-2 text-xs font-medium transition-all",
                  theme === "light"
                    ? "bg-white text-slate-900 shadow-xs"
                    : "text-slate-600 hover:text-slate-900"
                )}
              >
                <Sun className="h-4 w-4" />
                <span>Light</span>
              </button>

              <button
                type="button"
                onClick={() => setTheme("dark")}
                className={cn(
                  "flex items-center justify-center gap-2 rounded-lg py-2 text-xs font-medium transition-all",
                  theme === "dark"
                    ? "bg-white text-slate-900 shadow-xs"
                    : "text-slate-600 hover:text-slate-900"
                )}
              >
                <Moon className="h-4 w-4" />
                <span>Dark</span>
              </button>

              <button
                type="button"
                onClick={() => setTheme("system")}
                className={cn(
                  "flex items-center justify-center gap-2 rounded-lg py-2 text-xs font-medium transition-all",
                  theme === "system"
                    ? "bg-white text-slate-900 shadow-xs"
                    : "text-slate-600 hover:text-slate-900"
                )}
              >
                <Laptop className="h-4 w-4" />
                <span>System</span>
              </button>
            </div>
          </div>
        </DialogContent>
      </Dialog>
    </>
  );
}
