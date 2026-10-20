import { useEffect, useState } from "react";
import { ArrowLeft, Plus } from "lucide-react";
import { Link, useNavigate, useParams } from "react-router-dom";
import { ChapterWorkspace } from "../components/write/ChapterWorkspace";
import { GenreList } from "../components/GenreList";
import { ProjectCover } from "../components/ProjectCover";
import { invokeCommand } from "../lib/ipc";
import type { ProjectSummary } from "../types";
import "../styles/projects.css";

function startNewChat(navigate: ReturnType<typeof useNavigate>) {
  navigate("/chat");
  window.dispatchEvent(new Event("merlink:new-chat"));
}

export function ProjectDetail() {
  const { projectId = "" } = useParams();
  const navigate = useNavigate();
  const [project, setProject] = useState<ProjectSummary | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [tab, setTab] = useState<"overview" | "write">("overview");
  useEffect(() => {
    let active = true;
    setLoading(true); setError(null);
    invokeCommand("get_project", { projectId: decodeURIComponent(projectId) })
      .then((result) => { if (active) setProject(result); })
      .catch((reason) => { if (active) setError(reason instanceof Error ? reason.message : String(reason)); })
      .finally(() => { if (active) setLoading(false); });
    return () => { active = false; };
  }, [projectId]);

  return (
    <div className="project-detail-page">
      <Link to="/projects" className="project-back-link"><ArrowLeft size={16} /> Back to projects</Link>
      {loading ? <p className="projects-state">Loading project…</p> : error ? (
        <section className="projects-empty" role="alert"><h1>Could not load project</h1><p>{error}</p><Link to="/projects" className="btn">Back to projects</Link></section>
      ) : !project ? (
        <section className="projects-empty"><h1>Project not found</h1><p>This project may have been removed or is unavailable.</p><Link to="/projects" className="btn">Back to projects</Link></section>
      ) : (
        tab === "write" ? (
          <>
            <nav className="project-tabs" aria-label="Project sections">
              <button type="button" className="project-tab" onClick={() => setTab("overview")}>
                Tổng quan
              </button>
              <button type="button" className="project-tab is-active" aria-current="page">
                Viết
              </button>
            </nav>
            <ChapterWorkspace projectId={decodeURIComponent(projectId)} />
          </>
        ) : (
        <>
        <nav className="project-tabs" aria-label="Project sections">
          <button type="button" className="project-tab is-active" aria-current="page">
            Tổng quan
          </button>
          <button type="button" className="project-tab" onClick={() => setTab("write")}>
            Viết
          </button>
        </nav>
        <section className="project-detail-layout">
          <div className="project-detail-cover-wrap"><ProjectCover projectId={project.id} title={project.title} genres={project.genres} variant="detail" /></div>
          <div className="project-detail-info">
            <p className="project-detail-eyebrow">PROJECT</p>
            <h1>{project.title}</h1>
            <GenreList genres={project.genres} columns={3} maxRows={4} />
            <div className="project-detail-meta"><span>Created <strong>{new Intl.DateTimeFormat(undefined, { dateStyle: "medium" }).format(new Date(project.created_at))}</strong></span><span><strong>{project.session_count}</strong> {project.session_count === 1 ? "conversation" : "conversations"}</span></div>
            <button className="btn btn-primary" onClick={() => startNewChat(navigate)}><Plus size={16} /> Start a chat</button>
          </div>
        </section>
        </>
        )
      )}
    </div>
  );
}
