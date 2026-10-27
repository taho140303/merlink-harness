import ProjectView from "../../core/components/project/ProjectView";

export function meta() {
  return [
    { title: "Merlink Harness - Project" },
    { name: "description", content: "Worldbuilding Projects Workspace" },
  ];
}

export default function ProjectRoute() {
  return <ProjectView />;
}
