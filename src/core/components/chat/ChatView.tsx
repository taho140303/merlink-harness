import { useState } from "react";
import { ChatHeader } from "./ChatHeader";
import { ChatWelcome } from "./ChatWelcome";
import { ChatInputBar } from "./ChatInputBar";
import type { ChatMode } from "./types";

export default function ChatView() {
  const [mode, setMode] = useState<ChatMode>("chat");

  return (
    <div className="flex h-full flex-col bg-slate-50/50 dark:bg-slate-950/20">
      <ChatHeader mode={mode} onModeChange={setMode} />
      
      <div className="flex-1 overflow-y-auto px-6">
        <ChatWelcome />
      </div>

      <div className="shrink-0 p-6">
        <ChatInputBar />
      </div>
    </div>
  );
}
