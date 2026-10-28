import { useState } from "react";
import { ChevronRight, ChevronDown, Trash2, Plus, HelpCircle } from "lucide-react";
import { cn } from "../../../shared/utils/cn";
import { Input } from "../../../shared/components/ui/input";
import { Select } from "../../../shared/components/ui/select";
import { Button } from "../../../shared/components/ui/button";
import { Checkbox } from "../../../shared/components/ui/checkbox";
import { Switch } from "../../../shared/components/ui/switch";
import { useT } from "../../../shared/i18n/useT";
import type { AIProviderItem, ModelConfig } from "./types";

interface ProviderFormProps {
  initialData?: AIProviderItem | null;
  onCancel: () => void;
  onSave: (provider: AIProviderItem) => void;
}

export function ProviderForm({ initialData, onCancel, onSave }: ProviderFormProps) {
  const t = useT();
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
          {t("settings.provider.thirdParty")}
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
          {t("settings.provider.customApi")}
        </button>
      </div>

      <p className="text-xs text-fg-muted">
        {t("settings.provider.description")}
      </p>

      {/* Provider selection (Chỉ hiện ở third-party) */}
      {providerMode === "third-party" && (
        <div className="space-y-1.5">
          <Select
            label={t("settings.provider.label")}
            value={providerType}
            onChange={(e) => setProviderType(e.target.value)}
          >
            <option value="anthropic">anthropic</option>
            <option value="openai">openai</option>
            <option value="deepseek">deepseek</option>
            <option value="google">google gemini</option>
            <option value="kimi">kimi</option>
          </Select>
        </div>
      )}

      {/* API Key */}
      <div className="space-y-1.5">
        <div className="flex items-center gap-1.5">
          <label className="text-xs font-medium text-fg " htmlFor="api-key-field">{t("settings.provider.apiKey")}</label>
          <HelpCircle className="h-3.5 w-3.5 text-fg-subtle" />
        </div>
        <Input
          id="api-key-field"
          type="password"
          value={apiKey}
          onChange={(e) => setApiKey(e.target.value)}
          placeholder={t("settings.provider.apiKeyPlaceholder", { token: `${providerType.toUpperCase()}_AUTH_TOKEN` })}
          className="w-full text-xs"
        />
        <p className="text-xs text-amber-600 dark:text-amber-400">
          {t("settings.provider.envNotDetected", { token: `${providerType.toUpperCase()}_AUTH_TOKEN` })}
        </p>
      </div>

      {/* Base URL ra ngoài (Chỉ hiện ở custom) */}
      {providerMode === "custom" && (
        <div className="space-y-1.5">
          <label className="text-xs font-medium text-fg " htmlFor="custom-base-url">{t("settings.provider.baseUrl")}</label>
          <Input
            id="custom-base-url"
            type="text"
            value={baseUrl}
            onChange={(e) => setBaseUrl(e.target.value)}
            placeholder="http://127.0.0.1:2455/v1"
            className="w-full text-xs"
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
          <span>{t("settings.provider.customizedSettings")}</span>
        </button>

        {customizedOpen && (
          <div className="mt-3 space-y-4 rounded-xl border border-line bg-surface-muted/50 p-3.5  ">
            {/* Display Name */}
            <div className="space-y-1.5">
              <label className="text-xs font-medium text-fg " htmlFor="provider-display-name">{t("settings.provider.displayName")}</label>
              <Input
                id="provider-display-name"
                type="text"
                value={displayName}
                onChange={(e) => setDisplayName(e.target.value)}
                placeholder="codex-lb"
                className="w-full text-xs"
              />
            </div>

            {/* Base URL (Chỉ hiện ở third-party trong Customized settings) */}
            {providerMode === "third-party" && (
              <div className="space-y-1.5">
                <label className="text-xs font-medium text-fg " htmlFor="provider-base-url">{t("settings.provider.baseUrl")}</label>
                <Input
                  id="provider-base-url"
                  type="text"
                  value={baseUrl}
                  onChange={(e) => setBaseUrl(e.target.value)}
                  placeholder="http://127.0.0.1:2455/v1"
                  className="w-full text-xs"
                />
              </div>
            )}

            {/* API Protocol */}
            <div className="space-y-1.5">
              <Select
                label={t("settings.provider.apiProtocol")}
                value={apiProtocol}
                onChange={(e) => setApiProtocol(e.target.value)}
              >
                <option value="OpenAI Chat Completions">OpenAI Chat Completions</option>
                <option value="Anthropic Messages">Anthropic Messages</option>
              </Select>
            </div>

            {/* Models catalog */}
            <div className="space-y-3 pt-2">
              <div className="flex items-center justify-between">
                <div>
                  <h4 className="text-xs font-semibold text-fg">{t("settings.provider.models")}</h4>
                  <p className="text-xs text-fg-muted">{t("settings.provider.modelCatalog")}</p>
                </div>
                <div className="flex gap-2 text-xs">
                  <button type="button" className="text-fg-muted hover:text-fg ">
                    {t("settings.provider.restoreDefaults")}
                  </button>
                  <span className="text-fg-subtle ">•</span>
                  <button type="button" className="text-fg-muted hover:text-fg ">
                    {t("settings.provider.fetchModels")}
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
                      <Input
                        type="text"
                        aria-label={t("settings.provider.models")}
                        value={model.id}
                        onChange={(e) => {
                          const updated = [...models];
                          updated[idx].id = e.target.value;
                          setModels(updated);
                        }}
                        className="flex-1 text-xs"
                      />
                      <Input
                        type="text"
                        aria-label={t("settings.provider.displayName")}
                        value={model.name}
                        onChange={(e) => {
                          const updated = [...models];
                          updated[idx].name = e.target.value;
                          setModels(updated);
                        }}
                        className="flex-1 text-xs"
                      />
                      <Button
                        type="button"
                        aria-label={t("common.remove")}
                        variant="ghost"
                        size="icon"
                        onClick={() => removeModel(idx)}
                        className="text-fg-subtle hover:text-rose-500"
                      >
                        <Trash2 className="h-4 w-4" />
                      </Button>
                    </div>

                    <div className="grid grid-cols-2 gap-2">
                      <div>
                        <label className="text-xs text-fg-muted">{t("settings.provider.contextWindow")}</label>
                        <Input
                          type="text"
                          aria-label={t("settings.provider.contextWindow")}
                          value={model.contextWindow}
                          onChange={(e) => {
                            const updated = [...models];
                            updated[idx].contextWindow = e.target.value;
                            setModels(updated);
                          }}
                          className="w-full text-xs"
                        />
                      </div>
                      <div>
                        <label className="text-xs text-fg-muted">{t("settings.provider.maxOutputTokens")}</label>
                        <Input
                          type="text"
                          aria-label={t("settings.provider.maxOutputTokens")}
                          value={model.maxOutputTokens}
                          onChange={(e) => {
                            const updated = [...models];
                            updated[idx].maxOutputTokens = e.target.value;
                            setModels(updated);
                          }}
                          className="w-full text-xs"
                        />
                      </div>
                    </div>

                    <div className="flex items-center justify-between pt-1">
                      <div className="flex items-center gap-3 text-xs text-fg-muted">
                        <span className="text-xs text-fg-muted">{t("settings.provider.inputTypes")}</span>
                        <Checkbox
                          label={t("settings.provider.text")}
                          checked={model.inputTypes.text}
                          onChange={(e) => {
                            const updated = [...models];
                            updated[idx].inputTypes.text = e.target.checked;
                            setModels(updated);
                          }}
                        />
                        <Checkbox
                          label={t("settings.provider.image")}
                          checked={model.inputTypes.image}
                          onChange={(e) => {
                            const updated = [...models];
                            updated[idx].inputTypes.image = e.target.checked;
                            setModels(updated);
                          }}
                        />
                      </div>

                      <div className="flex items-center gap-1.5">
                        <span className="text-xs text-fg-muted">{t("settings.provider.reasoning")}</span>
                        <Switch
                          checked={model.reasoning}
                          ariaLabel={t("settings.provider.reasoning")}
                          onCheckedChange={(checked) => {
                            const updated = [...models];
                            updated[idx].reasoning = checked;
                            setModels(updated);
                          }}
                        />
                      </div>
                    </div>
                  </div>
                ))}

                <Button type="button" variant="secondary" size="sm" onClick={addModel} className="w-full border-dashed">
                  <Plus className="h-3.5 w-3.5" />
                  <span>{t("settings.provider.addModel")}</span>
                </Button>
              </div>
            </div>
          </div>
        )}
      </div>

      {/* Form Action Buttons */}
      <div className="flex items-center justify-end gap-2 pt-3 border-t border-line ">
        <Button type="button" variant="secondary" size="sm" onClick={onCancel}>
          {t("settings.provider.cancel")}
        </Button>
        <Button type="submit" variant="primary" size="sm">
          {t("common.save")}
        </Button>
      </div>
    </form>
  );
}
