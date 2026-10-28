import { useState } from "react";
import { ChevronDown, Check } from "lucide-react";
import { cn } from "../../../shared/utils/cn";
import { Popover } from "../../../shared/components/ui/popover";
import { useT } from "../../../shared/i18n/useT";

interface ModelOption {
  id: string;
  name: string;
  provider: string;
}

const AVAILABLE_MODELS: Record<string, ModelOption[]> = {
  Anthropic: [
    { id: "claude-3-7-sonnet", name: "Claude 3.7 Sonnet", provider: "Anthropic" },
    { id: "claude-3-5-haiku", name: "Claude 3.5 Haiku", provider: "Anthropic" },
  ],
  OpenAI: [
    { id: "gpt-4o", name: "GPT-4o", provider: "OpenAI" },
    { id: "gpt-4o-mini", name: "GPT-4o mini", provider: "OpenAI" },
    { id: "o3-mini", name: "o3-mini", provider: "OpenAI" },
  ],
  DeepSeek: [
    { id: "deepseek-chat", name: "DeepSeek-V3", provider: "DeepSeek" },
    { id: "deepseek-reasoner", name: "DeepSeek-R1", provider: "DeepSeek" },
  ],
};

export function ModelSelectorPopover() {
  const t = useT();
  const [selectedModel, setSelectedModel] = useState<ModelOption>(
    AVAILABLE_MODELS.Anthropic[0]
  );

  return (
    <Popover
      label={t("chat.selectModel")}
      align="end"
      trigger={
        <span className="flex items-center gap-1.5 rounded-xl tier-input px-2.5 py-1.5 text-xs font-medium text-fg hover:bg-surface-muted transition-colors">
          <span className="max-w-[110px] truncate">{selectedModel.name}</span>
          <ChevronDown aria-hidden="true" className="h-3 w-3 text-fg-subtle" />
        </span>
      }
      triggerClassName="rounded-xl"
      contentClassName="no-scrollbar max-h-72 w-56 overflow-y-auto p-1.5"
    >
      {Object.entries(AVAILABLE_MODELS).map(([provider, models]) => (
        <div key={provider} className="py-1">
          <div className="px-2.5 py-1 text-xs font-semibold uppercase tracking-wider text-fg-subtle">
            {provider}
          </div>
          <div className="space-y-0.5">
            {models.map((m) => {
              const isSelected = selectedModel.id === m.id;
              return (
                <button
                  key={m.id}
                  type="button"
                  onClick={() => setSelectedModel(m)}
                  className={cn(
                    "focus-ring flex w-full items-center justify-between rounded-lg px-2.5 py-1.5 text-xs transition-colors text-left",
                    isSelected
                      ? "bg-surface-muted font-medium text-fg"
                      : "text-fg-muted hover:bg-surface-muted hover:text-fg"
                  )}
                >
                  <span className="truncate">{m.name}</span>
                  {isSelected && <Check aria-hidden="true" className="h-3.5 w-3.5 shrink-0 text-accent" />}
                </button>
              );
            })}
          </div>
        </div>
      ))}
    </Popover>
  );
}
