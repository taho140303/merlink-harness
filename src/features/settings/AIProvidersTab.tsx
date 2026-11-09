import { useEffect, useState } from "react";
import { Plus, AlertTriangle, AlertCircle, RefreshCw, CheckCircle2, ShieldAlert } from "lucide-react";
import { cn } from "@/shared/utils/cn";
import {
  ProviderForm,
  getProviderConfig,
  updateProviderConfig,
  testProviderConnection,
  type CredentialStorageType,
  type ProviderConfigStatusDto,
  type ProviderConfigResultDto,
  type UpdateProviderConfigInput,
  type TestProviderConnectionParams,
  type TestProviderConnectionResult,
  type AIProviderItemExtended,
} from "./ProviderForm";
import { CodingAgentCards } from "./CodingAgentCards";
import { DeleteConfirmDialog } from "@/shared/components/ui/delete-confirm-dialog";
import { toast } from "@/shared/components/ui/toast";
import { useT } from "@/shared/i18n/useT";
import type { AIProviderItem } from "./types";
import { useModelCatalogStore } from "./modelCatalogStore";
import { backendClient, type BackendClient } from "@/shared/backend/client";
import { redactSensitiveString } from "@/shared/backend/adapter";
import type { RuntimeAndProviderConfigDto } from "@/shared/backend/generated";

// Re-export public interfaces for external callers and tests
export {
  getProviderConfig,
  updateProviderConfig,
  testProviderConnection,
  type CredentialStorageType,
  type ProviderConfigStatusDto,
  type ProviderConfigResultDto,
  type UpdateProviderConfigInput,
  type TestProviderConnectionParams,
  type TestProviderConnectionResult,
  type AIProviderItemExtended,
};

const DEFAULT_FALLBACK_PROVIDERS: AIProviderItemExtended[] = [];

export interface AIProvidersTabProps {
  client?: BackendClient;
  initialConfig?: Partial<RuntimeAndProviderConfigDto>;
  initialProviders?: AIProviderItem[];
}

export function AIProvidersTab({
  client,
  initialConfig,
  initialProviders,
}: AIProvidersTabProps = {}) {
  const t = useT();
  const [activeSubTab, setActiveSubTab] = useState<"api" | "coding-agent">("api");

  // Determine initial credential storage and providers
  const initialStorageMode: CredentialStorageType =
    initialConfig?.credentialStorageMode === "session_only_fallback" ||
    initialConfig?.credentialStorageMode === "SESSION_ONLY"
      ? "SESSION_ONLY"
      : initialConfig?.credentialStorageMode === "os_keychain" ||
        initialConfig?.credentialStorageMode === "KEYCHAIN"
      ? "KEYCHAIN"
      : "KEYCHAIN";

  const initialItems: AIProviderItemExtended[] = initialProviders
    ? (initialProviders as AIProviderItemExtended[]).filter((p) => p.id !== "ollama")
    : initialConfig?.providers
    ? initialConfig.providers.filter((p) => p.providerId !== "ollama").map((p) => ({
        id: p.providerId,
        name: p.name,
        status: p.isActive ? "connected" : "disconnected",
        hasApiKey: Boolean(p.hasApiKey),
        maskedApiKey: p.hasApiKey ? "••••••••" : undefined,
        credentialStorage: p.hasApiKey ? initialStorageMode : "NONE",
        baseUrl: p.endpointUrl || undefined,
        modelName: p.modelName || undefined,
        isCustom: p.providerId !== "ollama" && p.providerId !== "opencode" && p.providerId !== "deepseek",
      }))
    : DEFAULT_FALLBACK_PROVIDERS;

  const [providers, setProviders] = useState<AIProviderItemExtended[]>(initialItems);
  const [credentialStorage, setCredentialStorage] = useState<CredentialStorageType>(initialStorageMode);
  const [isBackendAvailable, setIsBackendAvailable] = useState<boolean>(true);
  const [testingProviderId, setTestingProviderId] = useState<string | null>(null);

  const [editingProvider, setEditingProvider] = useState<AIProviderItemExtended | null>(null);
  const [isAdding, setIsAdding] = useState(false);
  const [deleteTarget, setDeleteTarget] = useState<AIProviderItemExtended | null>(null);
  const [deleteOpen, setDeleteOpen] = useState(false);

  // Fetch live IPC state via BackendClient (REQ-FE-006, CI-10)
  useEffect(() => {
    let isMounted = true;
    const activeClient = client || backendClient;

    async function loadLiveConfig() {
      useModelCatalogStore.getState().hydrate();
      const savedNames = new Map(
        useModelCatalogStore.getState().providers.map((provider) => [provider.id, provider.name])
      );

      if (initialConfig?.providers) {
        setProviders((current) => current
          .filter((provider) => provider.id !== "ollama")
          .map((provider) => ({
            ...provider,
            name: savedNames.get(provider.id) || provider.name,
          })));
        return;
      }

      try {
        const config = await getProviderConfig(activeClient);
        if (!isMounted) return;

        setCredentialStorage(config.credentialStorage);
        setIsBackendAvailable(true);

        if (config.providers) {
          const liveList: AIProviderItemExtended[] = config.providers
            .filter((p) => p.providerId !== "ollama")
            .map((p) => ({
            id: p.providerId,
            name: savedNames.get(p.providerId) || p.name,
            status: p.isActive ? "connected" : "disconnected",
            hasApiKey: p.hasApiKey,
            maskedApiKey: p.maskedApiKey,
            credentialStorage: p.credentialStorage,
            baseUrl: p.endpointUrl || undefined,
            isCustom:
              p.providerId !== "ollama" &&
              p.providerId !== "opencode" &&
              p.providerId !== "deepseek",
          }));
          setProviders(liveList);
        }
      } catch (err) {
        if (!isMounted) return;
        // In unavailable / browser / SSR environments, mark backend as unavailable
        setIsBackendAvailable(false);
      }
    }

    loadLiveConfig();
    return () => {
      isMounted = false;
    };
  }, [client, initialConfig]);

  const handleTestProvider = async (provider: AIProviderItemExtended) => {
    setTestingProviderId(provider.id);
    try {
      const activeClient = client || backendClient;
      const res = await testProviderConnection(
        {
          providerId: provider.id,
          endpointUrl: provider.baseUrl,
        },
        activeClient
      );

      if (res.ok) {
        setProviders((prev) =>
          prev.map((p) =>
            p.id === provider.id
              ? { ...p, status: "connected", lastError: null }
              : p
          )
        );
        toast({ title: t("common.saved") || "Connection successful", variant: "success" });
      } else {
        setProviders((prev) =>
          prev.map((p) =>
            p.id === provider.id
              ? { ...p, status: "error", lastError: res.error }
              : p
          )
        );
        toast({
          title: `Connection failed: ${res.error || "PROVIDER_ERROR"}`,
          variant: "error",
        });
      }
    } catch (err: any) {
      setProviders((prev) =>
        prev.map((p) =>
          p.id === provider.id
            ? { ...p, status: "error", lastError: err.message || "PROVIDER_ERROR" }
            : p
        )
      );
      toast({
        title: `Connection failed: ${err.message || "PROVIDER_ERROR"}`,
        variant: "error",
      });
    } finally {
      setTestingProviderId(null);
    }
  };

  const handleSave = async (savedItem: AIProviderItem) => {
    // Secret safety: ensure raw apiKey is never saved in frontend state
    const safeItem: AIProviderItemExtended = {
      ...savedItem,
      status: "connected",
      hasApiKey: Boolean(savedItem.apiKey || (editingProvider && editingProvider.hasApiKey)),
      maskedApiKey: "••••••••",
      credentialStorage: credentialStorage,
      apiKey: undefined,
    };

    const cleanModels = (safeItem.models ?? [])
      .filter((m) => m && m.id && m.id.trim().length > 0)
      .map((m) => ({ id: m.id.trim(), name: m.name?.trim() || m.id.trim() }));

    useModelCatalogStore.getState().upsertProvider({
      id: safeItem.id,
      name: safeItem.name,
      models: cleanModels,
    });

    if (editingProvider) {
      setProviders(providers.map((p) => (p.id === safeItem.id ? safeItem : p)));
    } else {
      setProviders([...providers, safeItem]);
    }
    setIsAdding(false);
    setEditingProvider(null);
    toast({ title: t("common.saved") || "Provider saved", variant: "success" });
  };

  const requestDelete = (provider: AIProviderItemExtended) => {
    setDeleteTarget(provider);
    setDeleteOpen(true);
  };

  const confirmDelete = async () => {
    if (!deleteTarget) return;
    try {
      await (client || backendClient).updateRuntimeAndProviderConfig({ removeProviderId: deleteTarget.id });
    } catch (error) {
      toast({ title: redactSensitiveString(String(error)), variant: "error" });
      return;
    }
    useModelCatalogStore.getState().removeProvider(deleteTarget.id);
    setProviders((prev) => prev.filter((p) => p.id !== deleteTarget.id));
    toast({ title: t("common.deleted"), variant: "success" });
    setDeleteTarget(null);
  };

  if (isAdding || editingProvider) {
    return (
      <ProviderForm
        initialData={editingProvider}
        client={client}
        credentialStorage={credentialStorage}
        onCancel={() => {
          setIsAdding(false);
          setEditingProvider(null);
        }}
        onSave={handleSave}
      />
    );
  }

  const isSessionOnly =
    credentialStorage === "SESSION_ONLY" || credentialStorage === "session_only_fallback";

  return (
    <div className="space-y-4">
      <div>
        <h3 className="text-base font-semibold text-fg">
          {t("providers.models")}
        </h3>
        <p className="text-xs text-fg-muted">
          {t("providers.subtitle")}
        </p>
      </div>

      {/* Session-Only Warning (phase5_sprint6_deep1-F1) */}
      {isSessionOnly && (
        <div
          role="alert"
          className="flex items-start gap-2.5 rounded-xl border border-amber-500/30 bg-amber-500/10 p-3 text-xs text-amber-700 dark:text-amber-400"
        >
          <AlertTriangle className="mt-0.5 h-4 w-4 shrink-0 text-amber-600 dark:text-amber-400" />
          <div>
            <span className="font-semibold">
              SESSION_ONLY: Non-durable credential notice.{" "}
            </span>
            <span>
              OS Keychain is unavailable. Credentials are stored in-memory for this session
              only and will not persist after restart.
            </span>
          </div>
        </div>
      )}

      {/* Backend Unavailable Notice (CI-10) */}
      {!isBackendAvailable && (
        <div
          role="status"
          className="flex items-start gap-2.5 rounded-xl border border-line bg-surface-muted/60 p-3 text-xs text-fg-muted"
        >
          <ShieldAlert className="mt-0.5 h-4 w-4 shrink-0 text-fg-muted" />
          <div>
            <span className="font-semibold">Backend unavailable (offline mode). </span>
            <span>
              Tauri IPC is not connected. Showing simulated local provider state; changes will
              not be durably persisted to OS Keychain.
            </span>
          </div>
        </div>
      )}

      {/* Top-level segmented switch: API vs AI coding agent */}
      <div
        role="group"
        aria-label={t("settings.provider.modeLabel")}
        className="flex w-full flex-col rounded-xl bg-surface-muted p-1 sm:inline-flex sm:w-auto sm:flex-row"
      >
        <button
          type="button"
          onClick={() => setActiveSubTab("api")}
          aria-pressed={activeSubTab === "api"}
          className={cn(
            "focus-ring rounded-lg px-3.5 py-1.5 text-xs font-medium transition-colors",
            activeSubTab === "api"
              ? "bg-surface-card text-fg"
              : "text-fg-muted hover:text-fg"
          )}
        >
          {t("providers.tab.api")}
        </button>
        <button
          type="button"
          onClick={() => setActiveSubTab("coding-agent")}
          aria-pressed={activeSubTab === "coding-agent"}
          className={cn(
            "focus-ring rounded-lg px-3.5 py-1.5 text-xs font-medium transition-colors",
            activeSubTab === "coding-agent"
              ? "bg-surface-card text-fg"
              : "text-fg-muted hover:text-fg"
          )}
        >
          {t("settings.provider.codingAgent")}
        </button>
      </div>

      {activeSubTab === "coding-agent" ? (
        <CodingAgentCards />
      ) : (
        <div className="space-y-2">
          {providers.map((provider) => (
            <div
              key={provider.id}
              className="flex items-center justify-between rounded-2xl border border-line bg-surface-card px-4 py-2.5 transition-colors"
            >
              <div className="flex items-center gap-2.5 truncate">
                <span className="truncate text-xs font-medium text-fg">
                  {provider.name}
                </span>
                {provider.isCustom && (
                  <span className="rounded-md bg-surface-muted px-1.5 py-0.5 text-xs font-medium text-fg-muted">
                    {t("providers.custom")}
                  </span>
                )}

                {/* Status dot indicator (P5S6-B) */}
                {provider.status === "connected" || provider.status === "signed-in" ? (
                  <span
                    className="inline-block h-2 w-2 shrink-0 rounded-full bg-emerald-500"
                    title="Connected"
                  />
                ) : provider.status === "error" ? (
                  <span
                    className="inline-block h-2 w-2 shrink-0 rounded-full bg-rose-500"
                    title="Connection failed"
                  />
                ) : (
                  <span
                    className="inline-block h-2 w-2 shrink-0 rounded-full bg-zinc-400 dark:bg-zinc-600"
                    title="Disconnected"
                  />
                )}

                {provider.status === "signed-in" && (
                  <span className="text-xs font-normal text-emerald-600 dark:text-emerald-400">
                    {t("providers.signedIn")}
                  </span>
                )}

                {provider.status === "error" && (
                  <span className="text-xs font-normal text-rose-500">
                    Connection failed
                  </span>
                )}

                {/* Masked API Key & Storage Indicator (P5S6-P, phase5_sprint6_deep1-F1) */}
                {provider.hasApiKey && (
                  <div className="flex items-center gap-1.5">
                    <span className="font-mono text-xs text-fg-muted">
                      {provider.maskedApiKey || "••••••••"}
                    </span>
                    {provider.credentialStorage && (
                      <span className="rounded bg-surface-muted px-1.5 py-0.5 text-[10px] font-mono font-medium text-fg-muted">
                        {provider.credentialStorage}
                      </span>
                    )}
                  </div>
                )}
              </div>

              <div className="flex shrink-0 items-center gap-1">
                {/* Test Connection Button */}
                <button
                  type="button"
                  onClick={() => handleTestProvider(provider)}
                  disabled={testingProviderId === provider.id}
                  className="rounded-lg px-2.5 py-1 text-xs font-medium text-fg-muted hover:bg-surface-muted hover:text-fg disabled:opacity-50"
                >
                  {testingProviderId === provider.id ? "Testing..." : "Test"}
                </button>

                <button
                  type="button"
                  onClick={() => setEditingProvider(provider)}
                  className="rounded-lg px-2.5 py-1 text-xs font-medium text-fg-muted hover:bg-surface-muted hover:text-fg"
                >
                  {t("providers.edit")}
                </button>

                {provider.name !== "DeepSeek Account" && (
                  <button
                    type="button"
                    onClick={() => requestDelete(provider)}
                    className="rounded-lg px-2.5 py-1 text-xs font-medium text-rose-500 hover:bg-rose-50 hover:text-rose-600 dark:hover:bg-rose-950/30"
                  >
                    {t("providers.delete")}
                  </button>
                )}
              </div>
            </div>
          ))}

          <button
            type="button"
            onClick={() => setIsAdding(true)}
            className="flex w-full items-center justify-center gap-2 rounded-2xl border border-dashed border-line-strong py-3 text-xs font-medium text-fg-muted transition-colors hover:border-line-strong hover:text-fg"
          >
            <Plus className="h-4 w-4" />
            <span>{t("providers.add")}</span>
          </button>
        </div>
      )}

      <DeleteConfirmDialog
        open={deleteOpen}
        onOpenChange={setDeleteOpen}
        title={t("providers.delete.title")}
        description={`${deleteTarget?.name ?? ""} — ${t("providers.delete.description")}`}
        confirmLabel={t("delete.confirm")}
        cancelLabel={t("common.cancel")}
        onConfirm={confirmDelete}
      />
    </div>
  );
}
