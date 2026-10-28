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
    <form onSubmit={handleSubmit} className="space-y-5 text-fg ">
      {/* Tab Switcher: Third-party vs Custom */}
      <div className="inline-flex rounded-xl bg-surface-muted p-1 ">
        <button
          type="button"
          onClick={() => setProviderMode("third-party")}
          className={cn(
            "focus-ring rounded-lg px-3.5 py-1.5 text-xs font-medium transition-colors",
            providerMode === "third-party"
              ? "bg-surface-card text-fg"
              : "text-fg-muted hover:text-fg"
          )}
        >
          Third-party model provider
        </button>
        <button
          type="button"
          onClick={() => setProviderMode("custom")}
          className={cn(
            "focus-ring rounded-lg px-3.5 py-1.5 text-xs font-medium transition-colors",
            providerMode === "custom"
              ? "bg-surface-card text-fg"
              : "text-fg-muted hover:text-fg"
          )}
        >
          Custom model API
        </button>
      </div>

      <p className="text-xs text-fg-muted">
        Pick OpenAI, Anthropic, Kimi, or another provider from the built-in catalog and enter its API key.
      </p>

      {/* Provider selection (Chỉ hiện ở third-party) */}
      {providerMode === "third-party" && (
        <div className="space-y-1.5">
          <label className="text-xs font-medium text-fg ">Provider</label>
          <select
            value={providerType}
            onChange={(e) => setProviderType(e.target.value)}
            className="tier-input focus-ring w-full rounded-xl px-3 py-2 text-xs text-fg"
          >
            <option value="anthropic">anthropic</option>
            <option value="openai">openai</option>
            <option value="deepseek">deepseek</option>
            <option value="google">google gemini</option>
            <option value="kimi">kimi</option>
          </select>
        </div>
      )}

      {/* API Key */}
      <div className="space-y-1.5">
        <div className="flex items-center gap-1.5">
          <label className="text-xs font-medium text-fg ">API key</label>
          <HelpCircle className="h-3.5 w-3.5 text-fg-subtle" />
        </div>
        <input
          type="password"
          value={apiKey}
          onChange={(e) => setApiKey(e.target.value)}
          placeholder={`Leave blank to use env as auth (${providerType.toUpperCase()}_AUTH_TOKEN)`}
          className="w-full rounded-xl tier-input focus-ring w-full rounded-xl px-3 py-2 text-xs text-fg"
        />
        <p className="text-xs text-amber-600 dark:text-amber-400">
          Env var not detected. Enter an API key, or configure {providerType.toUpperCase()}_AUTH_TOKEN in your env.
        </p>
      </div>

      {/* Base URL ra ngoài (Chỉ hiện ở custom) */}
      {providerMode === "custom" && (
        <div className="space-y-1.5">
          <label className="text-xs font-medium text-fg ">Base URL</label>
          <input
            type="text"
            value={baseUrl}
            onChange={(e) => setBaseUrl(e.target.value)}
            placeholder="http://127.0.0.1:2455/v1"
            className="w-full rounded-xl tier-input focus-ring w-full rounded-xl px-3 py-2 text-xs text-fg"
          />
        </div>
      )}

      {/* Collapsible: Customized settings */}
      <div className="border-t border-line pt-3 ">
        <button
          type="button"
          onClick={() => setCustomizedOpen(!customizedOpen)}
          className="flex items-center gap-1.5 text-xs font-medium text-fg hover:text-fg  "
        >
          {customizedOpen ? <ChevronDown className="h-4 w-4" /> : <ChevronRight className="h-4 w-4" />}
          <span>Customized settings</span>
        </button>

        {customizedOpen && (
          <div className="mt-3 space-y-4 rounded-xl border border-line bg-surface-muted/50 p-3.5  ">
            {/* Display Name */}
            <div className="space-y-1.5">
              <label className="text-xs font-medium text-fg ">Display name</label>
              <input
                type="text"
                value={displayName}
                onChange={(e) => setDisplayName(e.target.value)}
                placeholder="codex-lb"
                className="w-full rounded-xl tier-input focus-ring w-full rounded-xl px-3 py-2 text-xs text-fg"
              />
            </div>

            {/* Base URL (Chỉ hiện ở third-party trong Customized settings) */}
            {providerMode === "third-party" && (
              <div className="space-y-1.5">
                <label className="text-xs font-medium text-fg ">Base URL</label>
                <input
                  type="text"
                  value={baseUrl}
                  onChange={(e) => setBaseUrl(e.target.value)}
                  placeholder="http://127.0.0.1:2455/v1"
                  className="w-full rounded-xl tier-input focus-ring w-full rounded-xl px-3 py-2 text-xs text-fg"
                />
              </div>
            )}

            {/* API Protocol */}
            <div className="space-y-1.5">
              <label className="text-xs font-medium text-fg ">API protocol</label>
              <select
                value={apiProtocol}
                onChange={(e) => setApiProtocol(e.target.value)}
                className="tier-input focus-ring w-full rounded-xl px-3 py-2 text-xs text-fg"
              >
                <option value="OpenAI Chat Completions">OpenAI Chat Completions</option>
                <option value="Anthropic Messages">Anthropic Messages</option>
              </select>
            </div>

            {/* Models catalog */}
            <div className="space-y-3 pt-2">
              <div className="flex items-center justify-between">
                <div>
                  <h4 className="text-xs font-semibold text-fg">Models</h4>
                  <p className="text-xs text-fg-muted">Customized model catalog</p>
                </div>
                <div className="flex gap-2 text-xs">
                  <button type="button" className="text-fg-muted hover:text-fg ">
                    Restore defaults
                  </button>
                  <span className="text-fg-subtle ">•</span>
                  <button type="button" className="text-fg-muted hover:text-fg ">
                    Fetch available models
                  </button>
                </div>
              </div>

              {/* Model Cards List */}
              <div className="space-y-2.5">
                {models.map((model, idx) => (
                  <div
                    key={idx}
                    className="space-y-2 rounded-xl border border-line bg-surface-card p-3  "
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
                        className="flex-1 rounded-lg border border-line bg-surface-muted px-2.5 py-1.5 text-xs text-fg   "
                      />
                      <input
                        type="text"
                        value={model.name}
                        onChange={(e) => {
                          const updated = [...models];
                          updated[idx].name = e.target.value;
                          setModels(updated);
                        }}
                        className="flex-1 rounded-lg border border-line bg-surface-muted px-2.5 py-1.5 text-xs text-fg   "
                      />
                      <button
                        type="button"
                        onClick={() => removeModel(idx)}
                        className="rounded-lg p-1.5 text-fg-subtle hover:bg-surface-muted hover:text-rose-500 "
                      >
                        <Trash2 className="h-4 w-4" />
                      </button>
                    </div>

                    <div className="grid grid-cols-2 gap-2">
                      <div>
                        <label className="text-xs text-fg-muted">Context window</label>
                        <input
                          type="text"
                          value={model.contextWindow}
                          onChange={(e) => {
                            const updated = [...models];
                            updated[idx].contextWindow = e.target.value;
                            setModels(updated);
                          }}
                          className="w-full rounded-lg border border-line bg-surface-muted px-2 py-1 text-xs text-fg   "
                        />
                      </div>
                      <div>
                        <label className="text-xs text-fg-muted">Max output tokens</label>
                        <input
                          type="text"
                          value={model.maxOutputTokens}
                          onChange={(e) => {
                            const updated = [...models];
                            updated[idx].maxOutputTokens = e.target.value;
                            setModels(updated);
                          }}
                          className="w-full rounded-lg border border-line bg-surface-muted px-2 py-1 text-xs text-fg   "
                        />
                      </div>
                    </div>

                    <div className="flex items-center justify-between pt-1">
                      <div className="flex items-center gap-3 text-xs text-fg-muted">
                        <span className="text-xs text-fg-muted">Input types:</span>
                        <label className="flex items-center gap-1 cursor-pointer">
                          <input
                            type="checkbox"
                            checked={model.inputTypes.text}
                            onChange={(e) => {
                              const updated = [...models];
                              updated[idx].inputTypes.text = e.target.checked;
                              setModels(updated);
                            }}
                            className="rounded border-line-strong"
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
                            className="rounded border-line-strong"
                          />
                          <span>Image</span>
                        </label>
                      </div>

                      <div className="flex items-center gap-1.5">
                        <span className="text-xs text-fg-muted">Reasoning</span>
                        <button
                          type="button"
                          onClick={() => {
                            const updated = [...models];
                            updated[idx].reasoning = !updated[idx].reasoning;
                            setModels(updated);
                          }}
                          className={cn(
                            "relative inline-flex h-4 w-7 shrink-0 cursor-pointer items-center rounded-full transition-colors",
                            model.reasoning ? "bg-accent " : "bg-surface-muted "
                          )}
                        >
                          <span
                            className={cn(
                              "inline-block h-3 w-3 transform rounded-full bg-surface-card  transition duration-200 ",
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
                  className="flex w-full items-center justify-center gap-1.5 rounded-xl border border-dashed border-line-strong py-2 text-xs font-medium text-fg-muted hover:border-line-strong hover:text-fg   "
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
      <div className="flex items-center justify-end gap-2 pt-3 border-t border-line ">
        <button
          type="button"
          onClick={onCancel}
          className="rounded-xl border border-line px-4 py-2 text-xs font-medium text-fg hover:bg-surface-muted   "
        >
          Cancel
        </button>
        <button
          type="submit"
          className="rounded-xl bg-accent px-4 py-2 text-xs font-medium text-white  hover:opacity-90   "
        >
          Apply
        </button>
      </div>
    </form>
  );
}
