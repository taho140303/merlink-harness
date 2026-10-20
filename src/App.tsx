import { useEffect, useRef, useState } from "react";
import {
  Routes,
  Route,
  NavLink,
  Outlet,
  useLocation,
  useNavigate,
} from "react-router-dom";
import {
  BookOpen,
  MessageSquare,
  Plus,
  Settings2,
  Sliders,
  Users,
  PanelLeftClose,
  MoreVertical,
  Pencil,
  Trash2,
  ChevronRight,
  ChevronDown,
} from "lucide-react";
import { Home } from "./pages/Home";
import { Chat } from "./pages/Chat";
import { Characters } from "./pages/Characters";
import { Projects } from "./pages/Projects";
import { ProjectDetail } from "./pages/ProjectDetail";
import { StoryConfig } from "./pages/StoryConfig";
import { SettingsDialog } from "./components/SettingsDialog";
import { applyTextScale, readTextScale } from "./lib/preferences";
import { applyTheme, readTheme, subscribeSystemTheme } from "./lib/theme";
import { invokeCommand } from "./lib/ipc";
import type { SessionSummary } from "./types";
import { Dialog } from "./components/Dialog";

const NAV_ITEMS = [
  { to: "/chat", label: "Chat", icon: MessageSquare },
  { to: "/projects", label: "Projects", icon: BookOpen },
  { to: "/characters", label: "Characters", icon: Users },
  { to: "/story-config", label: "Story Config", icon: Sliders },
];

const SIDEBAR_KEY = "merlink-sidebar-collapsed";

function readStoredJson<T>(key: string, fallback: T): T {
  try {
    const raw = localStorage.getItem(key);
    return raw ? (JSON.parse(raw) as T) : fallback;
  } catch {
    return fallback;
  }
}

/** Decorative background blobs, ported from the ref-ui `Atmosphere` component. */
function Atmosphere() {
  return (
    <div className="merlink-atmosphere" aria-hidden="true">
      {[1, 2, 3, 4].map((index) => (
        <div key={index} className="merlink-blob" />
      ))}
      <svg
        className="merlink-grain"
        xmlns="http://www.w3.org/2000/svg"
        width="100%"
        height="100%"
        preserveAspectRatio="none"
      >
        <defs>
          <filter id="merlink-noise">
            <feTurbulence
              type="fractalNoise"
              baseFrequency=".78"
              numOctaves="3"
              stitchTiles="stitch"
            />
          </filter>
        </defs>
        <rect width="100%" height="100%" filter="url(#merlink-noise)" opacity=".64" />
      </svg>
    </div>
  );
}

function Shell() {
  const location = useLocation();
  const navigate = useNavigate();
  const [settingsOpen, setSettingsOpen] = useState(false);
  const [providersRevision, setProvidersRevision] = useState(0);
  const [collapsed, setCollapsed] = useState(() => readStoredJson<boolean>(SIDEBAR_KEY, false));
  const [sessions, setSessions] = useState<SessionSummary[]>([]);
  const [sessionsError, setSessionsError] = useState(false);
  const [groupByProject, setGroupByProject] = useState(() => {
    try { return localStorage.getItem("merlink-session-group-by-project") === "true"; } catch { return false; }
  });
  const [collapsedGroups, setCollapsedGroups] = useState<string[]>(() => readStoredJson<string[]>("merlink-collapsed-project-groups", []));
  const [menuFor, setMenuFor] = useState<string | null>(null);
  const [renamingId, setRenamingId] = useState<string | null>(null);
  const [renameValue, setRenameValue] = useState("");
  const [renameError, setRenameError] = useState<string | null>(null);
  const [deleteTarget, setDeleteTarget] = useState<SessionSummary | null>(null);
  const renameInputRef = useRef<HTMLInputElement>(null);
  const cancelRenameRef = useRef(false);

  const loadSessions = async () => {
    try { setSessions(await invokeCommand("list_sessions")); setSessionsError(false); }
    catch (error) { console.error("Could not load conversations:", error); setSessionsError(true); }
  };

  useEffect(() => {
    void loadSessions();
    // Trigger native provider discovery at desktop startup; Ollama is added to
    // the shared local config when its default local endpoint is responding.
    void invokeCommand("load_providers").catch((error) =>
      console.error("Could not load model providers:", error),
    );
    const refresh = () => void loadSessions();
    window.addEventListener("merlink:sessions-changed", refresh);
    return () => window.removeEventListener("merlink:sessions-changed", refresh);
  }, []);

  useEffect(() => {
    try { localStorage.setItem("merlink-session-group-by-project", String(groupByProject)); } catch { /* storage unavailable */ }
  }, [groupByProject]);
  useEffect(() => {
    try { localStorage.setItem("merlink-collapsed-project-groups", JSON.stringify(collapsedGroups)); } catch { /* storage unavailable */ }
  }, [collapsedGroups]);
  useEffect(() => {
    if (renamingId) { renameInputRef.current?.focus(); renameInputRef.current?.select(); }
  }, [renamingId]);
  useEffect(() => {
    const closeOnEscape = (event: KeyboardEvent) => { if (event.key === "Escape") setMenuFor(null); };
    const closeOnOutsideClick = (event: MouseEvent) => {
      const target = event.target;
      if (target instanceof Element && !target.closest(".merlink-session-menu, .merlink-session-more")) setMenuFor(null);
    };
    window.addEventListener("keydown", closeOnEscape);
    window.addEventListener("mousedown", closeOnOutsideClick);
    return () => { window.removeEventListener("keydown", closeOnEscape); window.removeEventListener("mousedown", closeOnOutsideClick); };
  }, []);

  useEffect(() => {
    applyTheme(readTheme());
    // D1: the text scale is a real setting, applied as soon as the shell mounts
    // (next to the theme). `applyTextScale` never rejects, so there is nothing
    // to await here.
    void applyTextScale(readTextScale());
    return subscribeSystemTheme();
  }, []);

  const toggleSidebar = () => {
    setCollapsed((previous) => {
      const next = !previous;
      try {
        localStorage.setItem(SIDEBAR_KEY, JSON.stringify(next));
      } catch {
        /* storage unavailable */
      }
      return next;
    });
  };

  const isActive = (to: string) =>
    to === "/" ? location.pathname === "/" : location.pathname.startsWith(to);

  /**
   * D4: New chat lives in the sidebar and works from every page, including Home.
   * On `/chat` the event resets the mounted page; from anywhere else the route
   * change mounts a brand-new Chat (which starts empty anyway).
   */
  const startNewChat = () => {
    if (location.pathname === "/chat") window.dispatchEvent(new Event("merlink:new-chat"));
    else navigate("/chat");
  };
  const activeSessionId = location.pathname.startsWith("/chat/") ? decodeURIComponent(location.pathname.slice("/chat/".length)) : "";
  const autoExpandedActiveRef = useRef<string | null>(null);
  useEffect(() => {
    const active = sessions.find((session) => session.id === activeSessionId);
    if (active && autoExpandedActiveRef.current !== activeSessionId) {
      autoExpandedActiveRef.current = activeSessionId;
      setCollapsedGroups((current) => current.filter((key) => key !== active.project_id));
    }
    if (!activeSessionId) autoExpandedActiveRef.current = null;
  }, [activeSessionId, sessions]);
  const beginRename = (session: SessionSummary) => {
    cancelRenameRef.current = false;
    setMenuFor(null); setRenamingId(session.id); setRenameValue(session.title); setRenameError(null);
  };
  const saveRename = async (sessionId: string) => {
    if (renamingId !== sessionId) return;
    try {
      const updated = await invokeCommand("rename_session", { sessionId, title: renameValue });
      setSessions((current) => current.map((item) => item.id === sessionId ? updated : item));
      setRenamingId(null); setRenameError(null);
    } catch (error) { setRenameError(error instanceof Error ? error.message : String(error)); }
  };
  const confirmDelete = async () => {
    if (!deleteTarget) return;
    const target = deleteTarget;
    try {
      await invokeCommand("delete_session", { sessionId: target.id });
      setSessions((current) => current.filter((item) => item.id !== target.id));
      setDeleteTarget(null);
      if (activeSessionId === target.id) { navigate("/chat"); window.dispatchEvent(new Event("merlink:new-chat")); }
    } catch (error) { console.error("Could not delete conversation:", error); }
  };
  const grouped = new Map<string, SessionSummary[]>();
  for (const session of sessions) grouped.set(session.project_id, [...(grouped.get(session.project_id) ?? []), session]);
  const groupKeys = [...grouped.keys()].sort((a, b) => {
    if (!a) return b ? 1 : 0; if (!b) return -1;
    return (grouped.get(b)?.[0]?.created_at ?? "").localeCompare(grouped.get(a)?.[0]?.created_at ?? "");
  });
  const toggleGroup = (key: string) => setCollapsedGroups((current) => current.includes(key) ? current.filter((value) => value !== key) : [...current, key]);
  const renderSession = (session: SessionSummary) => (
    <div className={`merlink-session-row${activeSessionId === session.id ? " merlink-session-open" : ""}`} key={session.id}>
      {renamingId === session.id ? <div className="merlink-session-rename-wrap">
        <input ref={renameInputRef} className="merlink-session-rename" value={renameValue} aria-label={`Rename ${session.title}`} onChange={(event) => { setRenameValue(event.target.value); setRenameError(null); }} onKeyDown={(event) => { if (event.key === "Enter") { event.preventDefault(); event.currentTarget.blur(); } if (event.key === "Escape") { event.preventDefault(); cancelRenameRef.current = true; event.currentTarget.blur(); setRenamingId(null); setRenameError(null); } }} onBlur={() => { if (cancelRenameRef.current) { cancelRenameRef.current = false; return; } void saveRename(session.id); }} />
        {renameError && <div className="merlink-session-error" role="alert">{renameError}</div>}
      </div> : <NavLink className="merlink-session-title" to={`/chat/${encodeURIComponent(session.id)}`} onClick={() => setMenuFor(null)}>{session.title}</NavLink>}
      <button type="button" className="merlink-session-more" aria-label={`Session actions for ${session.title}`} aria-haspopup="menu" aria-expanded={menuFor === session.id} onClick={() => setMenuFor(menuFor === session.id ? null : session.id)}><MoreVertical size={17} /></button>
      {menuFor === session.id && <div className="merlink-session-menu" role="menu" onMouseLeave={() => undefined}>
        <button type="button" className="merlink-session-menu-item" role="menuitem" onClick={() => beginRename(session)}><Pencil size={14} />Rename</button>
        <button type="button" className="merlink-session-menu-item" role="menuitem" onClick={() => { setMenuFor(null); setDeleteTarget(session); }}><Trash2 size={14} />Delete</button>
      </div>}
    </div>
  );

  return (
    <div className={`merlink-app${collapsed ? " is-collapsed" : ""}`}>
      <Atmosphere />

      <aside className="merlink-sidebar surface" aria-label="Primary navigation">
        <div className="merlink-brand">
          <NavLink
            className="merlink-brand-home"
            to="/"
            title="Merlink Harness home"
            aria-label="Merlink Harness home"
          >
            <span className="merlink-brand-mark">
              <img src="/logo.svg" width="36" height="36" alt="" />
            </span>
            <span className="merlink-brand-text">
              <span className="merlink-brand-title">Merlink Harness</span>
              <span className="merlink-brand-version">v1.0 STORY TOOL</span>
            </span>
          </NavLink>
          <button
            type="button"
            className="merlink-collapse"
            onClick={toggleSidebar}
            title="Collapse sidebar"
            aria-label="Collapse sidebar"
          >
            <PanelLeftClose size={17} />
          </button>
          <button
            type="button"
            className="merlink-expand"
            onClick={toggleSidebar}
            title="Expand sidebar"
            aria-label="Expand sidebar"
          >
            {/* Explicit panel-right glyph shown on hover of the collapsed mark. */}
            <svg
              width="24"
              height="24"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              strokeWidth="2"
              strokeLinecap="round"
              strokeLinejoin="round"
              aria-hidden="true"
            >
              <rect x="3" y="3" width="18" height="18" rx="2" />
              <path d="M15 3v18" />
            </svg>
          </button>
        </div>

        {/* D4: a full-width primary action directly under the brand, above the
            nav, so it is available on Home too. Collapsed, it keeps the icon and
            the same `data-tooltip` mechanism the nav links use. */}
        <button
          type="button"
          className="btn btn-primary merlink-new-chat"
          data-tooltip="New chat"
          title="New chat"
          aria-label="New chat"
          onClick={startNewChat}
        >
          <Plus size={17} />
          <span>New chat</span>
        </button>

        <span className="merlink-nav-label">Workspace</span>
        <nav className="merlink-nav" aria-label="Workspace">
          <NavLink
            to="/"
            className="merlink-nav-link"
            data-tooltip="Home"
            aria-label="Home"
            title="Home"
            aria-current={isActive("/") ? "page" : undefined}
          >
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="18"
              height="18"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              strokeWidth="2"
              strokeLinecap="round"
              strokeLinejoin="round"
              aria-hidden="true"
            >
              <path d="m3 9 9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z" />
              <polyline points="9 22 9 12 15 12 15 22" />
            </svg>
            <span>Home</span>
          </NavLink>

          {NAV_ITEMS.map((item) => {
            const IconComponent = item.icon;
            return (
              <NavLink
                key={item.to}
                to={item.to}
                className="merlink-nav-link"
                data-tooltip={item.label}
                aria-label={item.label}
                title={item.label}
                aria-current={isActive(item.to) ? "page" : undefined}
              >
                <IconComponent size={18} />
                <span>{item.label}</span>
              </NavLink>
            );
          })}
        </nav>

        {!collapsed && <section className="merlink-history" aria-label="Conversation history">
          <div className="merlink-history-head"><strong>Conversations</strong><label className="merlink-history-switch">Group by project <button type="button" role="switch" aria-checked={groupByProject} onClick={() => setGroupByProject((value) => !value)} /></label></div>
          {sessionsError ? <div className="merlink-history-empty">Could not load conversations. <button type="button" onClick={() => void loadSessions()}>Retry</button></div> : sessions.length === 0 ? <div className="merlink-history-empty">No conversations yet.</div> : <div className="merlink-session-list">
            {!groupByProject ? sessions.map(renderSession) : groupKeys.map((key) => {
              const expanded = !collapsedGroups.includes(key);
              const groupSessions = grouped.get(key) ?? [];
              const activeInGroup = groupSessions.some((session) => session.id === activeSessionId);
              const title = key ? groupSessions[0]?.project_title ?? key : "Unbound";
              return <div className="merlink-project-group" key={key}>
                <button type="button" className="merlink-project-toggle" aria-expanded={expanded || activeInGroup} onClick={() => toggleGroup(key)}>{(expanded || activeInGroup) ? <ChevronDown size={14} /> : <ChevronRight size={14} />}<span>{title}</span></button>
                {(expanded || activeInGroup) && <div className="merlink-project-children">{groupSessions.map(renderSession)}</div>}
              </div>;
            })}
          </div>}
        </section>}

        <div className="merlink-sidebar-bottom">
          <button
            type="button"
            className="merlink-nav-link merlink-settings-link"
            title="Settings"
            aria-label="Settings"
            onClick={() => setSettingsOpen(true)}
          >
            <Settings2 size={18} />
            <span>Settings</span>
          </button>
        </div>
      </aside>

      <main className={`merlink-content${isActive("/chat") ? " is-wide" : ""}`}>
        <div className="merlink-content-inner">
          <Outlet context={{ providersRevision }} />
        </div>
      </main>

      <Dialog open={!!deleteTarget} onClose={() => setDeleteTarget(null)} title="Delete conversation?">
        <p>Delete this conversation and its messages?</p>
        <div className="merlink-session-delete-actions"><button type="button" className="btn" onClick={() => setDeleteTarget(null)}>Keep conversation</button><button type="button" className="btn btn-primary" onClick={() => void confirmDelete()}>Delete conversation</button></div>
      </Dialog>
      <SettingsDialog
        open={settingsOpen}
        onClose={() => setSettingsOpen(false)}
        onProvidersChanged={() => setProvidersRevision((revision) => revision + 1)}
      />
    </div>
  );
}

export default function App() {
  return (
    <Routes>
      <Route path="/" element={<Shell />}>
        <Route index element={<Home />} />
        <Route path="chat" element={<Chat />} />
        <Route path="chat/:sessionId" element={<Chat />} />
        <Route path="characters" element={<Characters />} />
        <Route path="projects" element={<Projects />} />
        <Route path="projects/:projectId" element={<ProjectDetail />} />
        <Route path="story-config" element={<StoryConfig />} />
        <Route path="*" element={<Home />} />
      </Route>
    </Routes>
  );
}
