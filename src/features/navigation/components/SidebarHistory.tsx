import { useCallback, useEffect, useState } from "react";
import { NavLink, useLocation, useNavigate } from "react-router";
import { History as HistoryIcon, Folder, ChevronDown, Pencil, Trash2, Check, X } from "lucide-react";
import { cn } from "@/shared/utils/cn";
import { useT } from "@/shared/i18n/useT";
import { backendClient, type BackendClient } from "@/shared/backend/client";
import type { SessionSummaryDto } from "@/shared/backend/generated";
import type { ChatItem } from "../types";
import { toast } from "@/shared/components/ui/toast";
import { redactSensitiveString } from "@/shared/backend/adapter";

function formatSessionTime(value: string): string {
  const date = new Date(value);
  if (Number.isNaN(date.getTime())) return "";
  return new Intl.DateTimeFormat(undefined, { month: "numeric", day: "numeric" }).format(date);
}

export function isSidebarSessionVisible(session: SessionSummaryDto): boolean {
  return (session.messageCount ?? 0) >= 2;
}

export function mapSessionSummaryToChatItem(session: SessionSummaryDto): ChatItem {
  const storedTitle = session.displayName?.trim();
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
  const navigate = useNavigate();
  const location = useLocation();
  const [groupByProject, setGroupByProject] = useState(false);
  const [history, setHistory] = useState<ChatItem[]>([]);
  const [renamingId, setRenamingId] = useState<string | null>(null);
  const [renameValue, setRenameValue] = useState("");

  const loadHistory = useCallback(async () => {
    if (client.status === "unavailable") return;
    try {
      const result = await client.listChatSessions({ limit: 100, minMessageCount: 2 });
      const validItems = (result.items || []).filter(isSidebarSessionVisible);
      setHistory(validItems.map(mapSessionSummaryToChatItem));
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

  const handleSoftDelete = async (chat: ChatItem) => {
    try {
      await client.manageChatSessions({ action: "soft_delete", sessionIds: [chat.id] });
      setHistory((items) => items.filter((item) => item.id !== chat.id));
      window.dispatchEvent(new Event("merlink:chat-history-changed"));
      toast({ title: t("common.deleted"), variant: "success" });
      if (location.pathname === `/chat/${chat.id}`) navigate("/");
    } catch (error) {
      toast({ title: redactSensitiveString(String(error)), variant: "error" });
    }
  };

  const handleRename = async (chat: ChatItem) => {
    const name = renameValue.trim();
    if (!name || name.length > 120) return;
    try {
      await client.manageChatSessions({ action: "rename", sessionIds: [chat.id], displayName: name });
      setHistory((items) => items.map((item) => item.id === chat.id ? { ...item, title: name } : item));
      setRenamingId(null);
    } catch (error) {
      toast({ title: redactSensitiveString(String(error)), variant: "error" });
    }
  };

  const renderChatRow = (chat: ChatItem) => (
    <div key={chat.id} className="group flex w-full items-center rounded-lg">
      {renamingId === chat.id ? (
        <form className="flex min-w-0 flex-1 items-center gap-1" onSubmit={(event) => { event.preventDefault(); void handleRename(chat); }}>
          <input autoFocus value={renameValue} maxLength={120} onChange={(event) => setRenameValue(event.target.value)}
            onKeyDown={(event) => { if (event.key === "Escape") setRenamingId(null); }}
            aria-label={t("sidebar.renameSession")} className="tier-input focus-ring min-w-0 flex-1 rounded px-1.5 py-1 text-xs" />
          <button type="submit" aria-label={t("common.save")} className="focus-ring rounded p-1 text-emerald-600"><Check className="h-3.5 w-3.5" /></button>
          <button type="button" onClick={() => setRenamingId(null)} aria-label={t("common.cancel")} className="focus-ring rounded p-1 text-fg-muted"><X className="h-3.5 w-3.5" /></button>
        </form>
      ) : (
        <>
          <NavLink to={`/chat/${chat.id}`} className={({ isActive }) => cn(rowBase, "min-w-0 flex-1", isActive ? rowActive : rowIdle)}>
            <span className="truncate">{formatTitle(chat.title)}</span>
            <span className="shrink-0 text-xs text-fg-subtle">{chat.time}</span>
          </NavLink>
          <div className="flex shrink-0 opacity-100 transition-opacity sm:opacity-0 sm:group-hover:opacity-100 sm:group-focus-within:opacity-100">
            <button type="button" title={t("sidebar.renameSession")} aria-label={t("sidebar.renameSession")}
              onClick={() => { setRenamingId(chat.id); setRenameValue(chat.title); }}
              className="focus-ring rounded p-1 text-fg-muted hover:text-fg"><Pencil className="h-3.5 w-3.5" /></button>
            <button type="button" title={t("sidebar.deleteSession")} aria-label={t("sidebar.deleteSession")}
              onClick={() => void handleSoftDelete(chat)}
              className="focus-ring rounded p-1 text-fg-muted hover:text-rose-500"><Trash2 className="h-3.5 w-3.5" /></button>
          </div>
        </>
      )}
    </div>
  );

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
                    {group.chats.map(renderChatRow)}
                  </div>
                )}
              </div>
            );
          })
        ) : (
          history.map(renderChatRow)
        )}
      </div>
    </div>
  );
}
