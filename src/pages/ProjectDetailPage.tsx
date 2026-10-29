import { useParams, Link } from "react-router";
import { ProjectDetailView, MOCK_PROJECTS } from "@/features/projects";
import { useT } from "@/shared/i18n/useT";

export default function ProjectDetailPage() {
  const { projectId } = useParams();
  const t = useT();
  const project = MOCK_PROJECTS.find((p) => p.id === projectId);

  if (!project) {
    return (
      <div className="flex h-full flex-col items-center justify-center p-6 text-center">
        <p className="text-sm text-fg-muted">{t("project.notFound")}</p>
        <Link to="/project" className="mt-3 text-xs font-semibold text-blue-600 hover:underline">
          {t("project.backToList")}
        </Link>
      </div>
    );
  }

  return <ProjectDetailView project={project} />;
}
