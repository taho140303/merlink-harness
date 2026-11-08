import { useEffect, useState } from "react";
import { CheckCircle2, Loader2, Settings2 } from "lucide-react";
import { Button } from "@/shared/components/ui/button";
import { Input } from "@/shared/components/ui/input";
import { Select } from "@/shared/components/ui/select";
import {
  Dialog,
  DialogDescription,
  DialogContent,
  DialogTitle,
} from "@/shared/components/ui/dialog";
import { toast } from "@/shared/components/ui/toast";
import { useT } from "@/shared/i18n/useT";
import { backendClient } from "@/shared/backend/client";
import { useModelCatalogStore } from "./modelCatalogStore";
import ollamaLight from "@/shared/assets/model-provider/ollama-light.svg";
import ollamaDark from "@/shared/assets/model-provider/ollama-dark.svg";

const OLLAMA_PROVIDER_ID = "ollama";
const OLLAMA_STORAGE_KEY = "merlink.ollamaAgentConfig";
const DEFAULT_OLLAMA_ENDPOINT = "http://127.0.0.1:11434";

interface OllamaSettings {
  endpointUrl: string;
  modelName: string;
}

const DEFAULT_OLLAMA_SETTINGS: OllamaSettings = {
  endpointUrl: DEFAULT_OLLAMA_ENDPOINT,
  modelName: "",
};

function readStoredSettings(): OllamaSettings | null {
  if (typeof window === "undefined") return null;
  try {
    const value = window.localStorage.getItem(OLLAMA_STORAGE_KEY);
    if (!value) return null;
    const parsed = JSON.parse(value) as Partial<OllamaSettings>;
    if (typeof parsed.endpointUrl !== "string" || typeof parsed.modelName !== "string") {
      return null;
    }
    return { endpointUrl: parsed.endpointUrl, modelName: parsed.modelName };
  } catch {
    return null;
  }
}

function persistSettings(settings: OllamaSettings) {
  if (typeof window === "undefined") return;
  try {
    window.localStorage.setItem(OLLAMA_STORAGE_KEY, JSON.stringify(settings));
  } catch {
    // Keep the current session usable if browser storage is unavailable.
  }
}

export function CodingAgentCards() {
  const t = useT();
  const [settings, setSettings] = useState<OllamaSettings>(DEFAULT_OLLAMA_SETTINGS);
  const [configured, setConfigured] = useState(false);
  const [connected, setConnected] = useState(false);
  const [configOpen, setConfigOpen] = useState(false);
  const [availableModels, setAvailableModels] = useState<string[]>([]);
  const [isFetchingModels, setIsFetchingModels] = useState(false);
  const [isSaving, setIsSaving] = useState(false);
  const [isTesting, setIsTesting] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    let active = true;
    const storedSettings = readStoredSettings();
    if (storedSettings) {
      setSettings(storedSettings);
      setConfigured(Boolean(storedSettings.modelName.trim()));
    }

    if (backendClient.status === "available") {
      backendClient
        .getRuntimeAndProviderConfig({})
        .then(async (config) => {
          if (!active) return;
          const provider = config.providers.find((item) => item.providerId === OLLAMA_PROVIDER_ID);
          if (provider?.endpointUrl || provider?.modelName) {
            const liveSettings = {
              endpointUrl: provider.endpointUrl || DEFAULT_OLLAMA_ENDPOINT,
              modelName: provider.modelName || "",
            };
            setSettings(liveSettings);
            setConfigured(Boolean(liveSettings.modelName.trim()));
            persistSettings(liveSettings);
            return;
          }

          // Restore the user's non-secret local Ollama settings into this runtime.
          if (storedSettings?.modelName.trim()) {
            await backendClient.updateRuntimeAndProviderConfig({
              activeProviderId: OLLAMA_PROVIDER_ID,
              endpointUrl: storedSettings.endpointUrl,
              modelName: storedSettings.modelName,
            });
          }
        })
        .catch(() => {
          // The form remains usable; the backend connection can be retried later.
        });
    }

    return () => {
      active = false;
    };
  }, []);

  const fetchModels = async () => {
    const endpointUrl = settings.endpointUrl.trim();
    if (!endpointUrl) {
      setError(t("settings.provider.ollamaEndpointRequired"));
      return;
    }
    if (backendClient.status !== "available") {
      setError(t("settings.provider.ollamaBackendUnavailable"));
      return;
    }

    setIsFetchingModels(true);
    setError(null);
    try {
      const result = await backendClient.transport.invoke("fetch_provider_models" as any, {
        providerId: OLLAMA_PROVIDER_ID,
        endpointUrl,
      }) as { ok?: boolean; models?: string[]; error?: string };
      if (!result?.ok || !Array.isArray(result.models) || result.models.length === 0) {
        setAvailableModels([]);
        setError(result?.error || t("settings.provider.ollamaFetchFailed"));
        return;
      }
      setAvailableModels(result.models);
      setSettings((current) => ({
        ...current,
        modelName: result.models!.includes(current.modelName) ? current.modelName : "",
      }));
    } catch (err: any) {
      setAvailableModels([]);
      setError(err?.message || t("settings.provider.ollamaFetchFailed"));
    } finally {
      setIsFetchingModels(false);
    }
  };

  const saveSettings = async () => {
    const next = {
      endpointUrl: settings.endpointUrl.trim(),
      modelName: settings.modelName.trim(),
    };
    if (!next.endpointUrl || !next.modelName) {
      setError(t("settings.provider.ollamaRequired"));
      return;
    }

    setIsSaving(true);
    setError(null);
    try {
      if (backendClient.status === "available") {
        await backendClient.updateRuntimeAndProviderConfig({
          activeProviderId: OLLAMA_PROVIDER_ID,
          endpointUrl: next.endpointUrl,
          modelName: next.modelName,
        });
      }

      persistSettings(next);
      const catalog = useModelCatalogStore.getState();
      catalog.hydrate();
      catalog.upsertProvider({
        id: OLLAMA_PROVIDER_ID,
        name: "Ollama",
        models: [{ id: next.modelName, name: next.modelName }],
      });
      catalog.select({ providerId: OLLAMA_PROVIDER_ID, modelId: next.modelName });
      setSettings(next);
      setConfigured(true);
      setConnected(false);
      setConfigOpen(false);
      toast({ title: t("common.saved"), variant: "success" });
    } catch (err: any) {
      setError(err?.message || t("settings.provider.ollamaSaveFailed"));
    } finally {
      setIsSaving(false);
    }
  };

  const testConnection = async () => {
    const endpointUrl = settings.endpointUrl.trim();
    const modelName = settings.modelName.trim();
    if (!endpointUrl || !modelName) {
      setConfigOpen(true);
      setError(t("settings.provider.ollamaRequired"));
      return;
    }
    if (backendClient.status !== "available") {
      setError(t("settings.provider.ollamaBackendUnavailable"));
      setConfigOpen(true);
      return;
    }

    setIsTesting(true);
    setError(null);
    try {
      const result = await backendClient.transport.invoke("test_provider_connection" as any, {
        providerId: OLLAMA_PROVIDER_ID,
        endpointUrl,
        modelName,
      }) as { ok?: boolean; error?: string };
      if (!result?.ok) throw new Error(result?.error || t("settings.provider.ollamaTestFailed"));
      setConnected(true);
      toast({ title: t("settings.provider.ollamaTestSuccess"), variant: "success" });
    } catch (err: any) {
      setConnected(false);
      setError(err?.message || t("settings.provider.ollamaTestFailed"));
      setConfigOpen(true);
    } finally {
      setIsTesting(false);
    }
  };

  return (
    <section aria-label={t("settings.provider.codingAgent")} className="space-y-3">
      <p className="text-xs leading-relaxed text-fg-muted">
        {t("settings.provider.agentDemoHint")}
      </p>
      <ul className="space-y-2.5">
        <li className="grid grid-cols-[2.75rem_minmax(0,1fr)_auto] items-center gap-3 rounded-xl border border-line bg-surface-card p-3 sm:gap-4 sm:px-4">
          <div className="flex h-11 w-11 items-center justify-center rounded-lg border border-line bg-surface-muted p-2">
            <img src={ollamaLight} alt="" className="h-full w-full object-contain dark:hidden" />
            <img src={ollamaDark} alt="" className="hidden h-full w-full object-contain dark:block" />
          </div>
          <div className="min-w-0 space-y-1">
            <p className="text-[11px] font-medium text-fg-muted">{t("settings.provider.codingAgent")}</p>
            <h4 className="truncate text-sm font-semibold text-fg">Ollama</h4>
            <p className="flex items-center gap-1.5 text-xs text-fg-muted" role="status">
              {connected ? (
                <CheckCircle2 aria-hidden="true" className="h-3.5 w-3.5 text-emerald-500" />
              ) : (
                <span aria-hidden="true" className={`h-2 w-2 shrink-0 rounded-full ${configured ? "bg-amber-500" : "bg-fg-subtle"}`} />
              )}
              {connected
                ? t("settings.provider.ollamaConnected")
                : configured
                  ? t("settings.provider.ollamaConfigured")
                  : t("settings.provider.agentDisconnected")}
            </p>
          </div>
          <div className="flex flex-col items-stretch gap-1.5">
            <Button type="button" size="sm" variant="secondary" onClick={() => { setError(null); setConfigOpen(true); }} className="min-w-20 rounded-lg px-2">
              <Settings2 aria-hidden="true" className="mr-1 h-3.5 w-3.5" />
              {t("settings.provider.agentConfig")}
            </Button>
            <Button type="button" size="sm" variant={connected ? "secondary" : "primary"} onClick={testConnection} disabled={isTesting} className="min-w-20 rounded-lg px-2">
              {isTesting ? <Loader2 aria-hidden="true" className="mr-1 h-3.5 w-3.5 animate-spin" /> : null}
              {t("settings.provider.ollamaTest")}
            </Button>
          </div>
        </li>
      </ul>

      <Dialog open={configOpen} onOpenChange={setConfigOpen}>
        <DialogContent className="max-w-md p-5 text-fg">
          <div className="space-y-1 pr-6">
            <DialogTitle className="text-base font-semibold">{t("settings.provider.ollamaTitle")}</DialogTitle>
            <DialogDescription className="text-xs text-fg-muted">{t("settings.provider.ollamaDescription")}</DialogDescription>
          </div>
          <div className="space-y-3">
              <Input
                label={t("settings.provider.baseUrl")}
                value={settings.endpointUrl}
                onChange={(event) => {
                  setSettings((current) => ({ ...current, endpointUrl: event.target.value, modelName: "" }));
                  setAvailableModels([]);
                }}
                placeholder={DEFAULT_OLLAMA_ENDPOINT}
              />
              <div className="space-y-2">
                <Select
                  label={t("settings.provider.ollamaModelId")}
                  value={settings.modelName}
                  onChange={(event) => setSettings((current) => ({ ...current, modelName: event.target.value }))}
                  disabled={availableModels.length === 0}
                >
                  <option value="">{t("settings.provider.ollamaSelectModel")}</option>
                  {settings.modelName && !availableModels.includes(settings.modelName) && (
                    <option value={settings.modelName}>{settings.modelName}</option>
                  )}
                  {availableModels.map((model) => <option key={model} value={model}>{model}</option>)}
                </Select>
                <Button type="button" variant="secondary" size="sm" onClick={fetchModels} disabled={isFetchingModels}>
                  {isFetchingModels ? t("settings.provider.ollamaFetchingModels") : t("settings.provider.ollamaFetchModels")}
                </Button>
              </div>
              {error && <p role="alert" className="text-xs text-rose-500">{error}</p>}
              <div className="flex justify-end gap-2 pt-1">
                <Button type="button" variant="secondary" size="sm" onClick={() => setConfigOpen(false)}>
                  {t("settings.provider.cancel")}
                </Button>
                <Button type="button" variant="primary" size="sm" onClick={saveSettings} disabled={isSaving}>
                  {isSaving ? t("settings.provider.saving") : t("common.save")}
                </Button>
              </div>
          </div>
        </DialogContent>
      </Dialog>
    </section>
  );
}
