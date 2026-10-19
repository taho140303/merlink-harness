import { useMemo } from "react";
import { Link } from "react-router-dom";
import {
  BookOpen,
  Users,
  Feather,
  Sparkles,
  ArrowUpRight,
  ArrowRight,
  MessageSquare,
  Settings2,
  Bot,
} from "lucide-react";
import { mockCharacters, mockProjects } from "../data/mock";
import { readCustomCharacters } from "../lib/characters";

export function Home() {
  // Project and word metrics remain demo fixtures until a real project read-model exists.
  const topProjects = mockProjects.slice(0, 3);
  const totalWords = mockProjects.reduce((sum, project) => sum + project.wordCount, 0);
  const totalCharacters = useMemo(
    () => readCustomCharacters().length + mockCharacters.length,
    [],
  );

  const cards = [
    {
      icon: BookOpen,
      value: String(mockProjects.length).padStart(2, "0"),
      label: "Stories in your library",
    },
    {
      icon: Users,
      value: String(totalCharacters).padStart(2, "0"),
      label: "Characters brought to life",
    },
    {
      icon: Feather,
      value: `${Math.round(totalWords / 1000)}k`,
      label: "Words across your worlds",
    },
  ];

  const links = [
    { icon: MessageSquare, label: "Write with AI", to: "/chat" },
    { icon: Users, label: "Meet your characters", to: "/characters" },
    { icon: Settings2, label: "Shape your writing style", to: "/story-config" },
    { icon: Bot, label: "Explore story projects", to: "/projects" },
  ];

  return (
    <div>
      <header className="dashboard-header">
        <div className="dashboard-heading">
          <p className="kicker">Your creative desk / Overview</p>
          <h1 className="page-title">Good to have you back.</h1>
          <p>Every great story starts with a moment to begin.</p>
        </div>
        <div className="dashboard-header-tools">
          <span className="dashboard-date surface-soft">
            ✦ &nbsp; Your writing space
          </span>
        </div>
      </header>

      <section className="dashboard-hero surface" aria-labelledby="hero-heading">
        <div className="dashboard-hero-copy">
          <span className="kicker">Pick up the thread</span>
          <h2 id="hero-heading">
            Make room for the <em>next chapter.</em>
          </h2>
          <p>
            A place for your ideas to wander and your words to find their way.
            Your worlds are waiting right where you left them.
          </p>
          <div className="dashboard-hero-actions">
            <Link className="btn btn-primary" to="/chat">
              <Sparkles size={17} /> Start writing <ArrowUpRight size={15} />
            </Link>
            <Link className="btn" to="/projects">
              <BookOpen size={17} /> Browse stories
            </Link>
          </div>
        </div>
        <div className="dashboard-hero-art" aria-hidden="true">
          <div className="dashboard-art-frame">
            <div className="dashboard-art-moon"></div>
            <div className="dashboard-art-horizon"></div>
            <div className="dashboard-art-orbit"></div>
          </div>
          <div className="dashboard-art-caption">
            <Feather size={15} /> Words become worlds
          </div>
        </div>
      </section>

      <section
        className="dashboard-metrics"
        aria-label="Your writing at a glance"
      >
        {cards.map((item) => {
          const IconComponent = item.icon;
          return (
            <div className="metric surface" key={item.label}>
              <div className="metric-icon">
                <IconComponent size={20} />
              </div>
              <div>
                <div className="metric-value">{item.value}</div>
                <div className="metric-label">{item.label}</div>
              </div>
            </div>
          );
        })}
      </section>

      <div className="dashboard-lower">
        <section aria-labelledby="recent-heading">
          <div className="dashboard-section-head">
            <div>
              <p className="kicker">Your library</p>
              <h2 id="recent-heading" className="section-title">
                Recently in the works
              </h2>
            </div>
            <Link className="dashboard-text-link" to="/projects">
              All projects <ArrowRight size={14} />
            </Link>
          </div>
          <div className="dashboard-projects">
            {topProjects.map((project) => (
              <Link
                key={project.id}
                to="/projects"
                className="dashboard-project surface-interactive"
              >
                <span
                  className="dashboard-project-cover"
                  style={{ background: project.cover }}
                  aria-hidden="true"
                ></span>
                <span className="dashboard-project-body">
                  <strong>{project.title}</strong>
                  <p>{project.description}</p>
                  <span className="dashboard-project-meta">
                    <span>{project.genre}</span>
                    <span>·</span>
                    <span>{project.chapters.length} chapters</span>
                    <span>·</span>
                    <span>{project.updatedAt}</span>
                  </span>
                </span>
                <ArrowUpRight size={17} className="dashboard-project-arrow" />
              </Link>
            ))}
          </div>
        </section>

        <aside className="dashboard-rail">
          <section className="dashboard-quick surface" aria-labelledby="quick-heading">
            <p className="kicker">Go somewhere</p>
            <h2 id="quick-heading" className="section-title">
              Your workspace
            </h2>
            <div className="dashboard-quick-list">
              {links.map((link) => {
                const IconComponent = link.icon;
                return (
                  <Link
                    key={link.to}
                    className="dashboard-quick-link"
                    to={link.to}
                  >
                    <IconComponent size={17} />
                    <span>{link.label}</span>
                    <ArrowUpRight size={14} />
                  </Link>
                );
              })}
            </div>
          </section>

          <section
            className="dashboard-activity surface"
            aria-labelledby="activity-heading"
          >
            <p className="kicker">The latest</p>
            <h2 id="activity-heading" className="section-title">
              Recent activity
            </h2>
            <div className="dashboard-activity-list">
              <div className="dashboard-activity-item">
                <span className="dashboard-activity-dot"></span>
                <div>
                  <p>
                    <strong>The Glass Tide</strong> is ready for its next scene.
                  </p>
                  <time>2 hours ago</time>
                </div>
              </div>
              <div className="dashboard-activity-item">
                <span className="dashboard-activity-dot"></span>
                <div>
                  <p>
                    You explored a new idea in <strong>AI Chat</strong>.
                  </p>
                  <time>Yesterday</time>
                </div>
              </div>
              <div className="dashboard-activity-item">
                <span className="dashboard-activity-dot"></span>
                <div>
                  <p>
                    <strong>The Midnight Orchard</strong> gained a chapter.
                  </p>
                  <time>2 days ago</time>
                </div>
              </div>
            </div>
          </section>
        </aside>
      </div>

      <p className="dashboard-footnote">
        Merlink Harness · A little more room for imagination · Local preview
      </p>
    </div>
  );
}
