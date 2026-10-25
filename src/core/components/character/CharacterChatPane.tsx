import { useState } from "react";
import { Sparkles, ArrowUp, X, User } from "lucide-react";
import { ModelSelectorPopover } from "../chat/ModelSelectorPopover";
import { ContextUsagePopover } from "../chat/ContextUsagePopover";
import { ChatMessageItem } from "../chat/ChatMessageItem";
import type { ChatMessage } from "../chat/types";

interface CharacterChatPaneProps {
  characterName: string;
  activeSection: string;
  onClose: () => void;
}

export function CharacterChatPane({
  characterName,
  activeSection,
  onClose,
}: CharacterChatPaneProps) {
  const [prompt, setPrompt] = useState("");
  const [messages, setMessages] = useState<ChatMessage[]>([]);

  const handleSend = () => {
    if (!prompt.trim()) return;

    const userMsg: ChatMessage = {
      id: `usr-${Date.now()}`,
      role: "user",
      content: prompt,
      timestamp: "Vừa xong",
    };

    const tempAssistantId = `ast-${Date.now()}`;
    const placeholderAssistant: ChatMessage = {
      id: tempAssistantId,
      role: "assistant",
      content: "",
      isThinking: true,
      timestamp: "Vừa xong",
    };

    setMessages((prev) => [...prev, userMsg, placeholderAssistant]);
    setPrompt("");

    setTimeout(() => {
      setMessages((prev) =>
        prev.map((msg) =>
          msg.id === tempAssistantId
            ? {
                ...msg,
                isThinking: false,
                thinkingSeconds: 4,
                thinkingDetails: `Trích xuất thông số nhân trắc học và lịch sử quan hệ của nhân vật ${characterName}.`,
                content: `Đã phân tích hồ sơ của **${characterName}** theo mục **${activeSection}**:\n\n- Đề xuất bổ sung vết tích chiến tranh để làm nổi bật nét tính cách lạnh lùng.\n- Quan hệ đối kháng với các phe phái hiện tại đang cần thêm tình tiết xúc tác trong Arc 1.`,
                toolCalls: [
                  {
                    id: `tool-${Date.now()}`,
                    name: "character_schema_inspector",
                    parameters: { character: characterName, section: activeSection },
                    result: { status: "audited", consistencyScore: 94 },
                    status: "success",
                  },
                ],
              }
            : msg
        )
      );
    }, 2500);
  };

  return (
    <aside className="flex h-full w-full flex-col border-r border-slate-200 bg-slate-50/60 dark:border-slate-800 dark:bg-slate-950/40">
      {/* Header Chat Pane */}
      <div className="flex h-14 shrink-0 items-center justify-between border-b border-slate-200 bg-white/70 px-4 backdrop-blur-xs dark:border-slate-800 dark:bg-slate-900/70">
        <div className="flex items-center gap-2 truncate">
          <Sparkles className="h-4 w-4 text-blue-500 shrink-0" />
          <span className="text-xs font-semibold text-slate-800 dark:text-slate-200 truncate">
            Copilot: {characterName}
          </span>
        </div>
        <button
          type="button"
          onClick={onClose}
          className="rounded-lg p-1 text-slate-400 hover:bg-slate-100 hover:text-slate-700 dark:hover:bg-slate-800 dark:hover:text-slate-200 transition-colors"
          title="Đóng Chat"
        >
          <X className="h-4 w-4" />
        </button>
      </div>

      {/* Message List */}
      <div className="flex-1 overflow-y-auto p-4 space-y-3">
        {messages.length === 0 ? (
          <div className="flex h-full flex-col items-center justify-center p-6 text-center text-xs text-slate-400">
            <User className="h-8 w-8 stroke-[1.5] text-slate-300 dark:text-slate-700 mb-2" />
            <p className="font-medium text-slate-600 dark:text-slate-300">
              Character Copilot đang kích hoạt
            </p>
            <p className="mt-1 text-[11px]">
              Đặt câu hỏi hoặc yêu cầu AI hỗ trợ hoàn thiện hồ sơ cho {characterName}.
            </p>
          </div>
        ) : (
          messages.map((m) => <ChatMessageItem key={m.id} message={m} />)
        )}
      </div>

      {/* Input Chat */}
      <div className="shrink-0 p-4">
        <div className="rounded-2xl border border-slate-200 bg-white p-3 shadow-xs dark:border-slate-800 dark:bg-slate-900">
          <div className="mb-2 flex items-center gap-1.5 rounded-lg bg-blue-50/70 px-2.5 py-1 text-[11px] font-medium text-blue-800 dark:bg-blue-950/50 dark:text-blue-300 truncate">
            <User className="h-3 w-3 shrink-0 text-blue-500" />
            <span className="truncate">Select source: {characterName} ({activeSection})</span>
          </div>

          <input
            type="text"
            value={prompt}
            onChange={(e) => setPrompt(e.target.value)}
            onKeyDown={(e) => {
              if (e.key === "Enter") {
                e.preventDefault();
                handleSend();
              }
            }}
            placeholder={`Hỏi về ${characterName}...`}
            className="w-full bg-transparent text-xs text-slate-800 placeholder:text-slate-400 focus:outline-hidden dark:text-slate-100"
          />

          <div className="mt-3 flex items-center justify-between border-t border-slate-100 pt-2 dark:border-slate-800/80">
            <span className="rounded-lg border border-slate-200 bg-slate-50 px-2.5 py-1 text-[10px] font-medium text-slate-700 dark:border-slate-700 dark:bg-slate-800 dark:text-slate-300 max-w-[120px] truncate">
              {characterName}
            </span>

            <div className="flex items-center gap-1.5">
              <ModelSelectorPopover />
              <ContextUsagePopover />
              <button
                type="button"
                onClick={handleSend}
                className="flex h-7 w-7 items-center justify-center rounded-lg bg-slate-900 text-white hover:bg-slate-800 dark:bg-slate-100 dark:text-slate-900 dark:hover:bg-slate-200 transition-colors"
                title="Gửi"
              >
                <ArrowUp className="h-3.5 w-3.5" />
              </button>
            </div>
          </div>
        </div>
      </div>
    </aside>
  );
}
