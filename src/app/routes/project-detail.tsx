import { ProjectDetailPage } from "@/pages";
import type { Route } from "./+types/project-detail";

export function meta({ params }: Route.MetaArgs) {
  void params;
  return [{ title: "Merlink Project Detail" }];
}

export default function ProjectDetailRoute() {
  return <ProjectDetailPage />;
}
