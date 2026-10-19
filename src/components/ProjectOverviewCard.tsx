import type { ProjectSummary } from "../types";
import { GenreList } from "./GenreList";
import { ProjectCover } from "./ProjectCover";

interface ProjectOverviewCardProps {
  project: ProjectSummary;
  onOpen: (projectId: string) => void;
}

export function ProjectOverviewCard({ project, onOpen }: ProjectOverviewCardProps) {
  const open = () => onOpen(project.id);
  return (
    <article className="project-overview-card surface-interactive" role="button" tabIndex={0} onClick={open} onKeyDown={(event) => {
      if (event.key === "Enter" || event.key === " ") { event.preventDefault(); open(); }
    }}>
      <ProjectCover projectId={project.id} title={project.title} genres={project.genres} />
      <div className="project-overview-body">
        <h2 className="project-overview-title">{project.title}</h2>
        <GenreList genres={project.genres} columns={2} maxRows={2} />
      </div>
    </article>
  );
}
