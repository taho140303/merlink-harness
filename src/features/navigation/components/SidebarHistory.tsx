import { useCallback, useEffect, useState } from "react";
import { NavLink } from "react-router";
import { History as HistoryIcon, Folder, ChevronDown } from "lucide-react";
import { cn } from "@/shared/utils/cn";
import { useT } from "@/shared/i18n/useT";
import { backendClient, type BackendClient } from "@/shared/backend/client";
import type { SessionSummaryDto } from "@/shared/backend/generated";
import type { ChatItem } from "../types";

function formatSessionTime(value: string): string {
  const date = new Date(value);
  if (Number.isNaN(date.getTime())) return "";
  return new Intl.DateTimeFormat(undefined, { month: "numeric", day: "numeric" }).format(date);
}

/** `displayName` is returned by the backend session list but is not yet in generated.ts. */
type SessionSummaryWithTitle = SessionSummaryDto & { displayName?: string | null };

export function mapSessionSummaryToChatItem(session: SessionSummaryDto): ChatItem {
  const storedTitle = (session as SessionSummaryWithTitle).displayName?.trim();
  return {
    id: session.sessionId,
    title: storedTitle || `Chat ${session.sessionId.slice(0, 6)}`,
    time: formatSessionTime(session.updatedAt),
    projectId: session.projectId || "",
    projectName: "Cuộc trò chuyện",
  };
}

function formatTitle(title: string): string {
  return title.length > 12 ? `${title.slice(0, 12)}...` : title;
}

interface SidebarHistoryProps {
  collapsed: boolean;
  onExpand: () => void;
  client?: BackendClient;
}

const rowBase =
  "focus-ring flex w-full items-center justify-between rounded-lg px-2.5 py-1.5 text-xs transition-colors text-left";
const rowActive = "bg-surface-muted font-medium text-fg";
const rowIdle = "text-fg-muted hover:bg-surface-muted hover:text-fg";

export function SidebarHistory({ collapsed, onExpand, client = backendClient }: SidebarHistoryProps) {
  const t = useT();
  const [groupByProject, setGroupByProject] = useState(false);
  const [history, setHistory] = useState<ChatItem[]>([]);

  const loadHistory = useCallback(async () => {
    if (client.status === "unavailable") return;
    try {
      const result = await client.listChatSessions({ limit: 100 });
      setHistory((result.items || []).map(mapSessionSummaryToChatItem));
    } catch (error) {
      console.warn("Failed to load chat history from backend:", error);
    }
  }, [client]);

  useEffect(() => {
    const initialLoad = window.setTimeout(() => void loadHistory(), 0);
    window.addEventListener("merlink:chat-history-changed", loadHistory);
    return () => {
      window.clearTimeout(initialLoad);
      window.removeEventListener("merlink:chat-history-changed", loadHistory);
    };
  }, [loadHistory]);
  const [openProjects, setOpenProjects] = useState<Record<string, boolean>>({
    "proj-1": true,
    "proj-2": true,
    "proj-3": true,
  });

  const toggleProject = (projectId: string) => {
    setOpenProjects((prev) => ({ ...prev, [projectId]: !prev[projectId] }));
  };

  // Nhóm theo projectId nhưng hiển thị projectName
  const projectGroups = history.reduce<Record<string, { projectName: string; chats: ChatItem[] }>>(
    (acc, item) => {
      if (!acc[item.projectId]) {
        acc[item.projectId] = { projectName: item.projectName, chats: [] };
      }
      acc[item.projectId].chats.push(item);
      return acc;
    },
    {}
  );

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
          Object.entries(projectGroups).map(([projectId, group]) => {
            const isOpen = openProjects[projectId] ?? false;
            return (
              <div key={projectId} className="space-y-0.5">
                <button
                  type="button"
                  onClick={() => toggleProject(projectId)}
                  aria-expanded={isOpen}
                  className="focus-ring flex w-full items-center justify-between rounded-lg px-2.5 py-1.5 text-xs font-medium text-fg-muted transition-colors hover:bg-surface-muted hover:text-fg"
                >
                  <div className="flex items-center gap-1.5 truncate">
                    <Folder className="h-3.5 w-3.5 shrink-0 text-fg-subtle" aria-hidden="true" />
                    {/* 3.1: Chỉ hiển thị project name */}
                    <span className="truncate">{group.projectName}</span>
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
                    {group.chats.map((chat) => (
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
          history.map((chat) => (
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
