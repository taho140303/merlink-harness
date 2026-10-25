import type { Route } from "./+types/home";
import ChatView from "../../src/core/components/chat/ChatView";

export function meta({}: Route.MetaArgs) {
  return [
    { title: "Merlink Harness - Chat" },
    { name: "description", content: "Merlink Harness UI Workspace" },
  ];
}

export default function Home() {
  return <ChatView />;
}
