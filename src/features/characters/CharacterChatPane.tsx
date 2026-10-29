import { useState } from "react";
import { Sparkles, ArrowUp, X, User } from "lucide-react";
import { ModelSelectorPopover, ContextUsagePopover, ChatMessageItem } from "@/features/chat";
import { useT } from "@/shared/i18n/useT";
import { Input } from "@/shared/components/ui/input";
import type { ChatMessage } from "@/features/chat";

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
  const t = useT();
  const [prompt, setPrompt] = useState("");
  const [messages, setMessages] = useState<ChatMessage[]>([]);

  const handleSend = () => {
    if (!prompt.trim()) return;

    const userMsg: ChatMessage = {
      id: `usr-${Date.now()}`,
      role: "user",
      content: prompt,
      timestamp: "Just now",
    };

    const tempAssistantId = `ast-${Date.now()}`;
    const placeholderAssistant: ChatMessage = {
      id: tempAssistantId,
      role: "assistant",
      content: "",
      isThinking: true,
      timestamp: "Just now",
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
                thinkingDetails: `Analyzing biometric profile and relationship history of ${characterName}.`,
                content: `Analyzed the profile of **${characterName}** under **${activeSection}**:\n\n- Suggest adding war scars to highlight the cold personality.\n- The antagonistic relationships currently need further catalysts in Arc 1.`,
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
            : msg,
        ),
      );
    }, 2500);
  };

  return (
    <aside className="flex h-full w-full flex-col border-r border-line bg-surface-muted/60  ">
      <div className="flex h-14 shrink-0 items-center justify-between border-b border-line bg-surface-card/70 px-4 backdrop-blur-xs  ">
        <div className="flex items-center gap-2 truncate">
          <Sparkles className="h-4 w-4 shrink-0 text-blue-500" />
          <span className="truncate text-xs font-semibold text-fg ">
            {t("character.chat.title")}: {characterName}
          </span>
        </div>
        <button
          type="button"
          aria-label={t("common.close")}
          onClick={onClose}
          className="focus-ring rounded-lg p-1 text-fg-subtle transition-colors hover:bg-surface-muted hover:text-fg"
        >
          <X className="h-4 w-4" aria-hidden="true" />
        </button>
      </div>

      <div className="flex-1 space-y-3 overflow-y-auto p-4">
        {messages.length === 0 ? (
          <div className="flex h-full flex-col items-center justify-center p-6 text-center text-xs text-fg-subtle">
            <User className="mb-2 h-8 w-8 stroke-[1.5] text-fg-subtle " />
            <p className="font-medium text-fg-muted">
              {t("character.chat.emptyTitle")}
            </p>
            <p className="mt-1 text-xs">
              {t("character.chat.emptySubtitle").replace("{name}", characterName)}
            </p>
          </div>
        ) : (
          messages.map((m) => <ChatMessageItem key={m.id} message={m} />)
        )}
      </div>

      <div className="shrink-0 p-4">
        <div className="tier-card rounded-card p-3">
          <div className="mb-2 flex items-center gap-1.5 truncate rounded-lg bg-blue-50/70 px-2.5 py-1 text-xs font-medium text-blue-800 dark:bg-blue-950/50 dark:text-blue-300">
            <User className="h-3 w-3 shrink-0 text-blue-500" />
            <span className="truncate">
              {t("character.chat.sourceLabel")}: {characterName} ({activeSection})
            </span>
          </div>

          <Input
            type="text"
            aria-label={t("character.chat.placeholder").replace("{name}", characterName)}
            value={prompt}
            onChange={(e) => setPrompt(e.target.value)}
            onKeyDown={(e) => {
              if (e.key === "Enter") {
                e.preventDefault();
                handleSend();
              }
            }}
            placeholder={t("character.chat.placeholder").replace("{name}", characterName)}
            className="tier-input focus-ring w-full border-0 bg-transparent text-xs text-fg"
          />

          <div className="mt-3 flex items-center justify-between border-t border-line pt-2">
            <span className="max-w-[120px] truncate rounded-lg tier-input focus-ring px-2.5 py-1 text-xs font-medium text-fg   ">
              {characterName}
            </span>

            <div className="flex items-center gap-1.5">
              <ModelSelectorPopover />
              <ContextUsagePopover />
              <button
                type="button"
                aria-label={t("character.chat.send")}
                onClick={handleSend}
                className="focus-ring flex h-7 w-7 items-center justify-center rounded-lg bg-accent text-white transition-colors hover:opacity-90"
              >
                <ArrowUp className="h-3.5 w-3.5" aria-hidden="true" />
              </button>
            </div>
          </div>
        </div>
      </div>
    </aside>
  );
}
