import { useState } from "react";
import { Plus } from "lucide-react";
import { cn } from "../../../shared/utils/cn";
import { ProviderForm } from "./ProviderForm";
import type { AIProviderItem } from "./types";

const INITIAL_PROVIDERS: AIProviderItem[] = [
  { id: "1", name: "DeepSeek Account", status: "signed-in" },
  { id: "2", name: "DeepSeek", status: "connected" },
  { id: "3", name: "Command Code", status: "connected" },
  { id: "4", name: "ChatGPT", status: "connected" },
  { id: "5", name: "vercel-ai-gateway", status: "connected" },
  { id: "6", name: "antigravity", status: "connected", isCustom: true },
  { id: "7", name: "codex-lb", status: "connected", isCustom: true },
];

export function AIProvidersTab() {
  const [providers, setProviders] = useState<AIProviderItem[]>(INITIAL_PROVIDERS);
  const [editingProvider, setEditingProvider] = useState<AIProviderItem | null>(null);
  const [isAdding, setIsAdding] = useState(false);

  const handleSave = (provider: AIProviderItem) => {
    if (editingProvider) {
      setProviders(providers.map((p) => (p.id === provider.id ? provider : p)));
    } else {
      setProviders([...providers, provider]);
    }
    setIsAdding(false);
    setEditingProvider(null);
  };

  const handleDelete = (id: string) => {
    setProviders(providers.filter((p) => p.id !== id));
  };

  if (isAdding || editingProvider) {
    return (
      <ProviderForm
        initialData={editingProvider}
        onCancel={() => {
          setIsAdding(false);
          setEditingProvider(null);
        }}
        onSave={handleSave}
      />
    );
  }

  return (
    <div className="space-y-4">
      <div>
        <h3 className="text-base font-semibold text-slate-900 dark:text-slate-100">Models</h3>
        <p className="text-xs text-slate-500 dark:text-slate-400">
          Enter your API keys to use models from the following providers.
        </p>
      </div>

      {/* Providers List Container */}
      <div className="space-y-2">
        {providers.map((provider) => (
          <div
            key={provider.id}
            className="flex items-center justify-between rounded-2xl border border-slate-200 bg-white px-4 py-2.5 transition-colors dark:border-slate-800 dark:bg-slate-900"
          >
            <div className="flex items-center gap-2.5 truncate">
              <span className="truncate text-xs font-medium text-slate-900 dark:text-slate-100">
                {provider.name}
              </span>

              {provider.isCustom && (
                <span className="rounded-md bg-slate-100 px-1.5 py-0.5 text-[10px] font-medium text-slate-600 dark:bg-slate-800 dark:text-slate-300">
                  Custom
                </span>
              )}

              <span className="inline-block h-2 w-2 rounded-full bg-emerald-500 shrink-0" />

              {provider.status === "signed-in" && (
                <span className="text-xs font-normal text-emerald-600 dark:text-emerald-400">
                  Signed in
                </span>
              )}
            </div>

            <div className="flex items-center gap-1 shrink-0">
              <button
                type="button"
                onClick={() => setEditingProvider(provider)}
                className="rounded-lg px-2.5 py-1 text-xs font-medium text-slate-600 hover:bg-slate-100 hover:text-slate-900 dark:text-slate-400 dark:hover:bg-slate-800 dark:hover:text-slate-200"
              >
                Edit
              </button>
              {provider.name !== "DeepSeek Account" && (
                <button
                  type="button"
                  onClick={() => handleDelete(provider.id)}
                  className="rounded-lg px-2.5 py-1 text-xs font-medium text-rose-500 hover:bg-rose-50 hover:text-rose-600 dark:hover:bg-rose-950/30"
                >
                  Delete
                </button>
              )}
            </div>
          </div>
        ))}

        {/* Add Provider Button */}
        <button
          type="button"
          onClick={() => setIsAdding(true)}
          className="flex w-full items-center justify-center gap-2 rounded-2xl border border-dashed border-slate-300 py-3 text-xs font-medium text-slate-600 transition-colors hover:border-slate-400 hover:text-slate-900 dark:border-slate-800 dark:text-slate-400 dark:hover:text-slate-200"
        >
          <Plus className="h-4 w-4" />
          <span>Add model provider</span>
        </button>
      </div>
    </div>
  );
}
