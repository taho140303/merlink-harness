import { useEffect, useState } from "react";
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
} from "lucide-react";
import { Home } from "./pages/Home";
import { Chat } from "./pages/Chat";
import { Characters } from "./pages/Characters";
import { Projects } from "./pages/Projects";
import { StoryConfig } from "./pages/StoryConfig";
import { SettingsDialog } from "./components/SettingsDialog";
import { applyTextScale, readTextScale } from "./lib/preferences";
import { applyTheme, readTheme, subscribeSystemTheme } from "./lib/theme";

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
  const [collapsed, setCollapsed] = useState(() =>
    readStoredJson<boolean>(SIDEBAR_KEY, false),
  );

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
    navigate("/chat");
    window.dispatchEvent(new Event("merlink:new-chat"));
  };

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
              <img src="logo.png" width="36" height="36" alt="" />
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
        <Route path="characters" element={<Characters />} />
        <Route path="projects" element={<Projects />} />
        <Route path="story-config" element={<StoryConfig />} />
        <Route path="*" element={<Home />} />
      </Route>
    </Routes>
  );
}
