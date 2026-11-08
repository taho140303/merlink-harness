import { useParams } from "react-router";
import { ChatPage } from "@/pages";

export function meta() {
  return [
    { title: "Merlink Harness - Chat" },
    { name: "description", content: "Merlink Harness UI Workspace" },
  ];
}

export default function ChatRoute() {
  const { id } = useParams();
  return <ChatPage sessionId={id} />;
}
