import { HomePage } from "@/pages";

export function meta() {
  return [
    { title: "Merlink Harness - Chat" },
    { name: "description", content: "Merlink Harness UI Workspace" },
  ];
}

export default function HomeRoute() {
  return <HomePage />;
}
