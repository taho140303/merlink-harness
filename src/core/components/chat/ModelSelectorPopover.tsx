import { useState, useRef, useEffect } from "react";
import { ChevronDown, Check } from "lucide-react";
import { cn } from "../../../shared/utils/cn";

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
  const [open, setOpen] = useState(false);
  const [selectedModel, setSelectedModel] = useState<ModelOption>(
    AVAILABLE_MODELS.Anthropic[0]
  );
  const containerRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    function handleClickOutside(e: MouseEvent) {
      if (containerRef.current && !containerRef.current.contains(e.target as Node)) {
        setOpen(false);
      }
    }
    if (open) document.addEventListener("mousedown", handleClickOutside);
    return () => document.removeEventListener("mousedown", handleClickOutside);
  }, [open]);

  return (
    <div className="relative inline-block" ref={containerRef}>
      {/* Selectbox Trigger */}
      <button
        type="button"
        onClick={() => setOpen(!open)}
        className="flex items-center gap-1.5 rounded-xl border border-slate-200 bg-white px-2.5 py-1.5 text-xs font-medium text-slate-700 shadow-2xs hover:bg-slate-50 dark:border-slate-800 dark:bg-slate-900 dark:text-slate-300 dark:hover:bg-slate-800 transition-colors"
      >
        <span className="max-w-[110px] truncate">{selectedModel.name}</span>
        <ChevronDown className={cn("h-3 w-3 text-slate-400 transition-transform", open && "rotate-180")} />
      </button>

      {/* Popover danh sách model theo provider */}
      {open && (
        <div className="absolute bottom-full right-0 mb-2 w-56 max-h-72 overflow-y-auto rounded-2xl border border-slate-200 bg-white p-1.5 shadow-xl dark:border-slate-800 dark:bg-slate-900 z-50 animate-in fade-in-0 zoom-in-95">
          {Object.entries(AVAILABLE_MODELS).map(([provider, models]) => (
            <div key={provider} className="py-1">
              <div className="px-2.5 py-1 text-[10px] font-semibold uppercase tracking-wider text-slate-400 dark:text-slate-500">
                {provider}
              </div>
              <div className="space-y-0.5">
                {models.map((m) => {
                  const isSelected = selectedModel.id === m.id;
                  return (
                    <button
                      key={m.id}
                      type="button"
                      onClick={() => {
                        setSelectedModel(m);
                        setOpen(false);
                      }}
                      className={cn(
                        "flex w-full items-center justify-between rounded-lg px-2.5 py-1.5 text-xs transition-colors text-left",
                        isSelected
                          ? "bg-slate-100 font-medium text-slate-900 dark:bg-slate-800 dark:text-white"
                          : "text-slate-600 hover:bg-slate-50 hover:text-slate-900 dark:text-slate-400 dark:hover:bg-slate-800/60 dark:hover:text-slate-200"
                      )}
                    >
                      <span className="truncate">{m.name}</span>
                      {isSelected && <Check className="h-3.5 w-3.5 shrink-0 text-blue-500" />}
                    </button>
                  );
                })}
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
