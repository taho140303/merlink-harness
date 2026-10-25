import { useState } from "react";
import { NavLink } from "react-router";
import { History as HistoryIcon, Folder, ChevronDown } from "lucide-react";
import { cn } from "../../../shared/utils/cn";
import type { ChatItem } from "./types";

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

interface SidebarHistoryProps {
  collapsed: boolean;
  onExpand: () => void;
}

export function SidebarHistory({ collapsed, onExpand }: SidebarHistoryProps) {
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

  if (collapsed) {
    return (
      <div className="pt-3">
        <button
          onClick={onExpand}
          className="flex w-full justify-center rounded-xl p-2.5 text-slate-500 hover:bg-slate-100 hover:text-slate-900 transition-colors dark:text-slate-400 dark:hover:bg-slate-900 dark:hover:text-slate-100"
          title="History"
        >
          <HistoryIcon className="h-5 w-5" />
        </button>
      </div>
    );
  }

  return (
    <div className="pt-3">
      <div className="flex items-center justify-between px-2.5 py-1.5">
        <span className="text-xs font-semibold uppercase tracking-wider text-slate-400 dark:text-slate-500">
          History
        </span>
        <button
          type="button"
          role="switch"
          aria-checked={groupByProject}
          onClick={() => setGroupByProject(!groupByProject)}
          className={cn(
            "relative inline-flex h-4 w-7 shrink-0 cursor-pointer items-center rounded-full transition-colors duration-200 ease-in-out focus:outline-hidden",
            groupByProject ? "bg-slate-900 dark:bg-slate-100" : "bg-slate-200 dark:bg-slate-700"
          )}
          title="Nhóm theo project"
        >
          <span
            className={cn(
              "pointer-events-none inline-block h-3 w-3 transform rounded-full bg-white shadow-xs transition duration-200 ease-in-out dark:bg-slate-900",
              groupByProject ? "translate-x-3.5" : "translate-x-0.5"
            )}
          />
        </button>
      </div>

      <div className="mt-1 space-y-0.5">
        {groupByProject ? (
          Object.entries(groupedChats).map(([projectName, chats]) => {
            const isOpen = openProjects[projectName] ?? false;
            return (
              <div key={projectName} className="space-y-0.5">
                <button
                  type="button"
                  onClick={() => toggleProject(projectName)}
                  className="flex w-full items-center justify-between rounded-lg px-2.5 py-1.5 text-xs font-medium text-slate-600 hover:bg-slate-50 hover:text-slate-900 transition-colors dark:text-slate-400 dark:hover:bg-slate-900 dark:hover:text-slate-200"
                >
                  <div className="flex items-center gap-1.5 truncate">
                    <Folder className="h-3.5 w-3.5 shrink-0 text-slate-400 dark:text-slate-500" />
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
                  <div className="ml-3 space-y-0.5 border-l border-slate-100 pl-2 dark:border-slate-800">
                    {chats.map((chat) => (
                      <NavLink
                        key={chat.id}
                        to={`/chat/${chat.id}`}
                        className={({ isActive }) =>
                          cn(
                            "flex w-full items-center justify-between rounded-md px-2 py-1.5 text-xs transition-colors text-left",
                            isActive
                              ? "bg-slate-100 font-medium text-slate-900 dark:bg-slate-800 dark:text-white"
                              : "text-slate-500 hover:bg-slate-50 hover:text-slate-900 dark:text-slate-400 dark:hover:bg-slate-900 dark:hover:text-slate-200"
                          )
                        }
                      >
                        <span className="truncate">{formatTitle(chat.title)}</span>
                        <span className="shrink-0 text-[10px] text-slate-400 dark:text-slate-500">
                          {chat.time}
                        </span>
                      </NavLink>
                    ))}
                  </div>
                )}
              </div>
            );
          })
        ) : (
          MOCK_HISTORY.map((chat) => (
            <NavLink
              key={chat.id}
              to={`/chat/${chat.id}`}
              className={({ isActive }) =>
                cn(
                  "flex w-full items-center justify-between rounded-lg px-2.5 py-1.5 text-xs transition-colors text-left",
                  isActive
                    ? "bg-slate-100 font-medium text-slate-900 dark:bg-slate-800 dark:text-white"
                    : "text-slate-500 hover:bg-slate-50 hover:text-slate-900 dark:text-slate-400 dark:hover:bg-slate-900 dark:hover:text-slate-200"
                )
              }
            >
              <span className="truncate">{formatTitle(chat.title)}</span>
              <span className="shrink-0 text-[10px] text-slate-400 dark:text-slate-500">
                {chat.time}
              </span>
            </NavLink>
          ))
        )}
      </div>
    </div>
  );
}
