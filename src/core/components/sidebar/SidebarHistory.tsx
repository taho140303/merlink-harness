import { useState } from "react";
import { NavLink } from "react-router";
import { History as HistoryIcon, Folder, ChevronDown } from "lucide-react";
import { cn } from "../../../shared/utils/cn";
import { useT } from "../../../shared/i18n/useT";
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

const rowBase =
  "focus-ring flex w-full items-center justify-between rounded-lg px-2.5 py-1.5 text-xs transition-colors text-left";
const rowActive = "bg-surface-muted font-medium text-fg";
const rowIdle = "text-fg-muted hover:bg-surface-muted hover:text-fg";

export function SidebarHistory({ collapsed, onExpand }: SidebarHistoryProps) {
  const t = useT();
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
          aria-label={t("sidebar.history")}
          className="focus-ring flex w-full justify-center rounded-xl p-2.5 text-fg-muted transition-colors hover:bg-surface-muted hover:text-fg"
          title={t("sidebar.history")}
        >
          <HistoryIcon className="h-5 w-5" aria-hidden="true" />
        </button>
      </div>
    );
  }

  return (
    <div className="pt-3">
      <div className="flex items-center justify-between px-2.5 py-1.5">
        <span className="text-xs font-semibold uppercase tracking-wider text-fg-subtle">
          {t("sidebar.history")}
        </span>
        <button
          type="button"
          role="switch"
          aria-checked={groupByProject}
          aria-label={t("sidebar.groupByProject")}
          onClick={() => setGroupByProject(!groupByProject)}
          className={cn(
            "focus-ring relative inline-flex h-4 w-7 shrink-0 cursor-pointer items-center rounded-full transition-colors duration-200 ease-in-out",
            groupByProject ? "bg-accent" : "bg-line",
          )}
          title={t("sidebar.groupByProject")}
        >
          <span
            aria-hidden="true"
            className={cn(
              "pointer-events-none inline-block h-3 w-3 transform rounded-full bg-white transition-transform duration-200 ease-in-out",
              groupByProject ? "translate-x-3.5" : "translate-x-0.5",
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
                  aria-expanded={isOpen}
                  className="focus-ring flex w-full items-center justify-between rounded-lg px-2.5 py-1.5 text-xs font-medium text-fg-muted transition-colors hover:bg-surface-muted hover:text-fg"
                >
                  <div className="flex items-center gap-1.5 truncate">
                    <Folder className="h-3.5 w-3.5 shrink-0 text-fg-subtle" aria-hidden="true" />
                    <span className="truncate">{projectName}</span>
                  </div>
                  <ChevronDown
                    aria-hidden="true"
                    className={cn(
                      "h-3.5 w-3.5 shrink-0 text-fg-subtle transition-transform duration-200",
                      !isOpen && "-rotate-90",
                    )}
                  />
                </button>

                {isOpen && (
                  <div className="ml-3 space-y-0.5 border-l border-line pl-2">
                    {chats.map((chat) => (
                      <NavLink
                        key={chat.id}
                        to={`/chat/${chat.id}`}
                        className={({ isActive }) =>
                          cn(rowBase, isActive ? rowActive : rowIdle)
                        }
                      >
                        <span className="truncate">{formatTitle(chat.title)}</span>
                        <span className="shrink-0 text-xs text-fg-subtle">{chat.time}</span>
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
              className={({ isActive }) => cn(rowBase, isActive ? rowActive : rowIdle)}
            >
              <span className="truncate">{formatTitle(chat.title)}</span>
              <span className="shrink-0 text-xs text-fg-subtle">{chat.time}</span>
            </NavLink>
          ))
        )}
      </div>
    </div>
  );
}
