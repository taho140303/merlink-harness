import { useCallback, useEffect, useState } from "react";
import { Plus } from "lucide-react";
import { useNavigate } from "react-router-dom";
import { ProjectOverviewCard } from "../components/ProjectOverviewCard";
import { invokeCommand } from "../lib/ipc";
import type { ProjectSummary } from "../types";
import "../styles/projects.css";

function newChat(navigate: ReturnType<typeof useNavigate>) {
  navigate("/chat");
  window.dispatchEvent(new Event("merlink:new-chat"));
}

export function Projects() {
  const navigate = useNavigate();
  const [projects, setProjects] = useState<ProjectSummary[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const load = useCallback(async () => {
    setLoading(true); setError(null);
    try { setProjects(await invokeCommand("list_projects")); }
    catch (reason) { setError(reason instanceof Error ? reason.message : String(reason)); }
    finally { setLoading(false); }
  }, []);
  useEffect(() => { void load(); }, [load]);

  return (
    <main className="projects-page">
      <header className="projects-header">
        <span className="page-breadcrumb">Workspace / <strong>Projects</strong></span>
        <button className="btn btn-primary" onClick={() => newChat(navigate)}><Plus size={16} /> New Project</button>
      </header>
      {loading ? <p className="projects-state">Loading projects…</p> : error ? (
        <div className="projects-state" role="alert"><p>Could not load projects: {error}</p><button className="btn" onClick={() => void load()}>Retry</button></div>
      ) : projects.length === 0 ? (
        <section className="projects-empty"><h1>No projects yet</h1><p>Start a chat to create your first project.</p><button className="btn btn-primary" onClick={() => newChat(navigate)}><Plus size={16} /> New Project</button></section>
      ) : (
        <section className="projects-grid" aria-label="Projects">
          {projects.map((project) => <ProjectOverviewCard key={project.id} project={project} onOpen={(id) => navigate(`/projects/${encodeURIComponent(id)}`)} />)}
        </section>
      )}
    </main>
  );
}
