import ChatView from "../../core/components/chat/ChatView";

export function meta() {
  return [
    { title: "Merlink Harness - Chat" },
    { name: "description", content: "Merlink Harness UI Workspace" },
  ];
}

export default function Home() {
  return <ChatView />;
}
