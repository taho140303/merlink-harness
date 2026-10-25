import { useState } from "react";
import { ChevronRight, ChevronDown, Trash2, Plus, HelpCircle } from "lucide-react";
import { cn } from "../../../shared/utils/cn";
import type { AIProviderItem, ModelConfig } from "./types";

interface ProviderFormProps {
  initialData?: AIProviderItem | null;
  onCancel: () => void;
  onSave: (provider: AIProviderItem) => void;
}

export function ProviderForm({ initialData, onCancel, onSave }: ProviderFormProps) {
  const [providerMode, setProviderMode] = useState<"third-party" | "custom">(
    initialData?.isCustom ? "custom" : "third-party"
  );
  const [providerType, setProviderType] = useState(initialData?.providerType || "anthropic");
  const [apiKey, setApiKey] = useState(initialData?.apiKey || "");
  const [customizedOpen, setCustomizedOpen] = useState(Boolean(initialData?.isCustom));

  // Custom parameters
  const [displayName, setDisplayName] = useState(initialData?.name || "");
  const [baseUrl, setBaseUrl] = useState(initialData?.baseUrl || "http://127.0.0.1:2455/v1");
  const [apiProtocol, setApiProtocol] = useState(initialData?.apiProtocol || "OpenAI Chat Completions");

  // Models catalog
  const [models, setModels] = useState<ModelConfig[]>(
    initialData?.models || [
      {
        id: "gpt-6-luna",
        name: "gpt-6-luna",
        contextWindow: "272K",
        maxOutputTokens: "32K",
        inputTypes: { text: true, image: false },
        reasoning: false,
      },
    ]
  );

  const addModel = () => {
    setModels([
      ...models,
      {
        id: `model-${Date.now()}`,
        name: "new-model",
        contextWindow: "128K",
        maxOutputTokens: "16K",
        inputTypes: { text: true, image: false },
        reasoning: false,
      },
    ]);
  };

  const removeModel = (index: number) => {
    setModels(models.filter((_, i) => i !== index));
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    onSave({
      id: initialData?.id || `provider-${Date.now()}`,
      name: displayName || (providerMode === "custom" ? "Custom Provider" : providerType),
      status: "connected",
      isCustom: providerMode === "custom",
      providerType,
      apiKey,
      baseUrl,
      apiProtocol,
      models,
    });
  };

  return (
    <form onSubmit={handleSubmit} className="space-y-5 text-slate-800 dark:text-slate-100">
      {/* Tab Switcher: Third-party vs Custom */}
      <div className="inline-flex rounded-xl bg-slate-100 p-1 dark:bg-slate-800">
        <button
          type="button"
          onClick={() => setProviderMode("third-party")}
          className={cn(
            "rounded-lg px-3.5 py-1.5 text-xs font-medium transition-all",
            providerMode === "third-party"
              ? "bg-white text-slate-900 shadow-xs dark:bg-slate-700 dark:text-white"
              : "text-slate-600 hover:text-slate-900 dark:text-slate-400 dark:hover:text-slate-100"
          )}
        >
          Third-party model provider
        </button>
        <button
          type="button"
          onClick={() => setProviderMode("custom")}
          className={cn(
            "rounded-lg px-3.5 py-1.5 text-xs font-medium transition-all",
            providerMode === "custom"
              ? "bg-white text-slate-900 shadow-xs dark:bg-slate-700 dark:text-white"
              : "text-slate-600 hover:text-slate-900 dark:text-slate-400 dark:hover:text-slate-100"
          )}
        >
          Custom model API
        </button>
      </div>

      <p className="text-xs text-slate-500 dark:text-slate-400">
        Pick OpenAI, Anthropic, Kimi, or another provider from the built-in catalog and enter its API key.
      </p>

      {/* Provider selection */}
      <div className="space-y-1.5">
        <label className="text-xs font-medium text-slate-700 dark:text-slate-300">Provider</label>
        <select
          value={providerType}
          onChange={(e) => setProviderType(e.target.value)}
          className="w-full rounded-xl border border-slate-200 bg-white px-3 py-2 text-xs text-slate-900 focus:outline-hidden focus:ring-2 focus:ring-slate-400 dark:border-slate-800 dark:bg-slate-900 dark:text-slate-100"
        >
          <option value="anthropic">anthropic</option>
          <option value="openai">openai</option>
          <option value="deepseek">deepseek</option>
          <option value="google">google gemini</option>
          <option value="kimi">kimi</option>
        </select>
      </div>

      {/* API Key */}
      <div className="space-y-1.5">
        <div className="flex items-center gap-1.5">
          <label className="text-xs font-medium text-slate-700 dark:text-slate-300">API key</label>
          <HelpCircle className="h-3.5 w-3.5 text-slate-400" />
        </div>
        <input
          type="password"
          value={apiKey}
          onChange={(e) => setApiKey(e.target.value)}
          placeholder={`Leave blank to use env as auth (${providerType.toUpperCase()}_AUTH_TOKEN)`}
          className="w-full rounded-xl border border-slate-200 bg-white px-3 py-2 text-xs text-slate-900 placeholder:text-slate-400 focus:outline-hidden focus:ring-2 focus:ring-slate-400 dark:border-slate-800 dark:bg-slate-900 dark:text-slate-100"
        />
        <p className="text-[11px] text-amber-600 dark:text-amber-400">
          Env var not detected. Enter an API key, or configure {providerType.toUpperCase()}_AUTH_TOKEN in your env.
        </p>
      </div>

      {/* Collapsible: Customized settings */}
      <div className="border-t border-slate-100 pt-3 dark:border-slate-800/80">
        <button
          type="button"
          onClick={() => setCustomizedOpen(!customizedOpen)}
          className="flex items-center gap-1.5 text-xs font-medium text-slate-700 hover:text-slate-900 dark:text-slate-300 dark:hover:text-white"
        >
          {customizedOpen ? <ChevronDown className="h-4 w-4" /> : <ChevronRight className="h-4 w-4" />}
          <span>Customized settings</span>
        </button>

        {customizedOpen && (
          <div className="mt-3 space-y-4 rounded-xl border border-slate-100 bg-slate-50/50 p-3.5 dark:border-slate-800/60 dark:bg-slate-950/40">
            {/* Display Name */}
            <div className="space-y-1.5">
              <label className="text-xs font-medium text-slate-700 dark:text-slate-300">Display name</label>
              <input
                type="text"
                value={displayName}
                onChange={(e) => setDisplayName(e.target.value)}
                placeholder="codex-lb"
                className="w-full rounded-xl border border-slate-200 bg-white px-3 py-2 text-xs text-slate-900 placeholder:text-slate-400 focus:outline-hidden focus:ring-2 focus:ring-slate-400 dark:border-slate-800 dark:bg-slate-900 dark:text-slate-100"
              />
            </div>

            {/* Base URL */}
            <div className="space-y-1.5">
              <label className="text-xs font-medium text-slate-700 dark:text-slate-300">Base URL</label>
              <input
                type="text"
                value={baseUrl}
                onChange={(e) => setBaseUrl(e.target.value)}
                placeholder="http://127.0.0.1:2455/v1"
                className="w-full rounded-xl border border-slate-200 bg-white px-3 py-2 text-xs text-slate-900 placeholder:text-slate-400 focus:outline-hidden focus:ring-2 focus:ring-slate-400 dark:border-slate-800 dark:bg-slate-900 dark:text-slate-100"
              />
            </div>

            {/* API Protocol */}
            <div className="space-y-1.5">
              <label className="text-xs font-medium text-slate-700 dark:text-slate-300">API protocol</label>
              <select
                value={apiProtocol}
                onChange={(e) => setApiProtocol(e.target.value)}
                className="w-full rounded-xl border border-slate-200 bg-white px-3 py-2 text-xs text-slate-900 focus:outline-hidden focus:ring-2 focus:ring-slate-400 dark:border-slate-800 dark:bg-slate-900 dark:text-slate-100"
              >
                <option value="OpenAI Chat Completions">OpenAI Chat Completions</option>
                <option value="Anthropic Messages">Anthropic Messages</option>
              </select>
            </div>

            {/* Models catalog */}
            <div className="space-y-3 pt-2">
              <div className="flex items-center justify-between">
                <div>
                  <h4 className="text-xs font-semibold text-slate-900 dark:text-slate-100">Models</h4>
                  <p className="text-[11px] text-slate-500 dark:text-slate-400">Customized model catalog</p>
                </div>
                <div className="flex gap-2 text-xs">
                  <button type="button" className="text-slate-500 hover:text-slate-800 dark:hover:text-slate-300">
                    Restore defaults
                  </button>
                  <span className="text-slate-300 dark:text-slate-700">•</span>
                  <button type="button" className="text-slate-500 hover:text-slate-800 dark:hover:text-slate-300">
                    Fetch available models
                  </button>
                </div>
              </div>

              {/* Model Cards List */}
              <div className="space-y-2.5">
                {models.map((model, idx) => (
                  <div
                    key={idx}
                    className="space-y-2 rounded-xl border border-slate-200 bg-white p-3 dark:border-slate-800 dark:bg-slate-900"
                  >
                    <div className="flex items-center gap-2">
                      <input
                        type="text"
                        value={model.id}
                        onChange={(e) => {
                          const updated = [...models];
                          updated[idx].id = e.target.value;
                          setModels(updated);
                        }}
                        className="flex-1 rounded-lg border border-slate-200 bg-slate-50 px-2.5 py-1.5 text-xs text-slate-900 dark:border-slate-800 dark:bg-slate-800 dark:text-slate-100"
                      />
                      <input
                        type="text"
                        value={model.name}
                        onChange={(e) => {
                          const updated = [...models];
                          updated[idx].name = e.target.value;
                          setModels(updated);
                        }}
                        className="flex-1 rounded-lg border border-slate-200 bg-slate-50 px-2.5 py-1.5 text-xs text-slate-900 dark:border-slate-800 dark:bg-slate-800 dark:text-slate-100"
                      />
                      <button
                        type="button"
                        onClick={() => removeModel(idx)}
                        className="rounded-lg p-1.5 text-slate-400 hover:bg-slate-100 hover:text-rose-500 dark:hover:bg-slate-800"
                      >
                        <Trash2 className="h-4 w-4" />
                      </button>
                    </div>

                    <div className="grid grid-cols-2 gap-2">
                      <div>
                        <label className="text-[10px] text-slate-500">Context window</label>
                        <input
                          type="text"
                          value={model.contextWindow}
                          onChange={(e) => {
                            const updated = [...models];
                            updated[idx].contextWindow = e.target.value;
                            setModels(updated);
                          }}
                          className="w-full rounded-lg border border-slate-200 bg-slate-50 px-2 py-1 text-xs text-slate-900 dark:border-slate-800 dark:bg-slate-800 dark:text-slate-100"
                        />
                      </div>
                      <div>
                        <label className="text-[10px] text-slate-500">Max output tokens</label>
                        <input
                          type="text"
                          value={model.maxOutputTokens}
                          onChange={(e) => {
                            const updated = [...models];
                            updated[idx].maxOutputTokens = e.target.value;
                            setModels(updated);
                          }}
                          className="w-full rounded-lg border border-slate-200 bg-slate-50 px-2 py-1 text-xs text-slate-900 dark:border-slate-800 dark:bg-slate-800 dark:text-slate-100"
                        />
                      </div>
                    </div>

                    <div className="flex items-center justify-between pt-1">
                      <div className="flex items-center gap-3 text-xs text-slate-600 dark:text-slate-300">
                        <span className="text-[10px] text-slate-500">Input types:</span>
                        <label className="flex items-center gap-1 cursor-pointer">
                          <input
                            type="checkbox"
                            checked={model.inputTypes.text}
                            onChange={(e) => {
                              const updated = [...models];
                              updated[idx].inputTypes.text = e.target.checked;
                              setModels(updated);
                            }}
                            className="rounded border-slate-300"
                          />
                          <span>Text</span>
                        </label>
                        <label className="flex items-center gap-1 cursor-pointer">
                          <input
                            type="checkbox"
                            checked={model.inputTypes.image}
                            onChange={(e) => {
                              const updated = [...models];
                              updated[idx].inputTypes.image = e.target.checked;
                              setModels(updated);
                            }}
                            className="rounded border-slate-300"
                          />
                          <span>Image</span>
                        </label>
                      </div>

                      <div className="flex items-center gap-1.5">
                        <span className="text-[10px] text-slate-500">Reasoning</span>
                        <button
                          type="button"
                          onClick={() => {
                            const updated = [...models];
                            updated[idx].reasoning = !updated[idx].reasoning;
                            setModels(updated);
                          }}
                          className={cn(
                            "relative inline-flex h-4 w-7 shrink-0 cursor-pointer items-center rounded-full transition-colors",
                            model.reasoning ? "bg-slate-900 dark:bg-slate-100" : "bg-slate-200 dark:bg-slate-700"
                          )}
                        >
                          <span
                            className={cn(
                              "inline-block h-3 w-3 transform rounded-full bg-white shadow-xs transition duration-200 dark:bg-slate-900",
                              model.reasoning ? "translate-x-3.5" : "translate-x-0.5"
                            )}
                          />
                        </button>
                      </div>
                    </div>
                  </div>
                ))}

                <button
                  type="button"
                  onClick={addModel}
                  className="flex w-full items-center justify-center gap-1.5 rounded-xl border border-dashed border-slate-300 py-2 text-xs font-medium text-slate-600 hover:border-slate-400 hover:text-slate-900 dark:border-slate-800 dark:text-slate-400 dark:hover:text-slate-200"
                >
                  <Plus className="h-3.5 w-3.5" />
                  <span>Add model</span>
                </button>
              </div>
            </div>
          </div>
        )}
      </div>

      {/* Form Action Buttons */}
      <div className="flex items-center justify-end gap-2 pt-3 border-t border-slate-100 dark:border-slate-800">
        <button
          type="button"
          onClick={onCancel}
          className="rounded-xl border border-slate-200 px-4 py-2 text-xs font-medium text-slate-700 hover:bg-slate-100 dark:border-slate-700 dark:text-slate-300 dark:hover:bg-slate-800"
        >
          Cancel
        </button>
        <button
          type="submit"
          className="rounded-xl bg-slate-900 px-4 py-2 text-xs font-medium text-white shadow-xs hover:bg-slate-800 dark:bg-slate-100 dark:text-slate-900 dark:hover:bg-slate-200"
        >
          Apply
        </button>
      </div>
    </form>
  );
}
