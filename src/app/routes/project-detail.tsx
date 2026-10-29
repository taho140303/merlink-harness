import { ProjectDetailPage } from "@/pages";
import type { Route } from "./+types/project-detail";
import { MOCK_PROJECTS } from "@/features/projects";

export function meta({ params }: Route.MetaArgs) {
  const proj = MOCK_PROJECTS.find((p) => p.id === params.projectId);
  return [{ title: proj ? `Merlink - ${proj.title}` : "Merlink Project Detail" }];
}

export default function ProjectDetailRoute() {
  return <ProjectDetailPage />;
}
