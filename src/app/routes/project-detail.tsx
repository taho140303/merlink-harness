import { useParams, Link } from "react-router";
import type { Route } from "./+types/project-detail";
import { MOCK_PROJECTS } from "../../core/components/project/mockProjects";
import { ProjectDetailView } from "../../core/components/project/ProjectDetailView";

export function meta({ params }: Route.MetaArgs) {
  const proj = MOCK_PROJECTS.find((p) => p.id === params.projectId);
  return [{ title: proj ? `Merlink - ${proj.title}` : "Merlink Project Detail" }];
}

export default function ProjectDetailRoute() {
  const { projectId } = useParams();
  const project = MOCK_PROJECTS.find((p) => p.id === projectId);

  if (!project) {
    return (
      <div className="flex h-full flex-col items-center justify-center p-6 text-center">
        <p className="text-sm text-fg-muted">Không tìm thấy dự án.</p>
        <Link to="/project" className="mt-3 text-xs font-semibold text-blue-600 hover:underline">
          Quay lại danh sách Project
        </Link>
      </div>
    );
  }

  return <ProjectDetailView project={project} />;
}
