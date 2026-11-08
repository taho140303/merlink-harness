import { ChatView } from "@/features/chat";

export default function ChatPage({ sessionId }: { sessionId?: string }) {
  return <ChatView sessionId={sessionId} />;
}
