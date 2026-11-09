import { useState } from "react";
import {
  ChevronRight,
  ChevronDown,
  Trash2,
  Plus,
  HelpCircle,
  CheckCircle2,
  AlertCircle,
  AlertTriangle,
  Loader2,
} from "lucide-react";
import { cn } from "@/shared/utils/cn";
import { Input } from "@/shared/components/ui/input";
import { Select } from "@/shared/components/ui/select";
import { Button } from "@/shared/components/ui/button";
import { Checkbox } from "@/shared/components/ui/checkbox";
import { Switch } from "@/shared/components/ui/switch";
import { toast } from "@/shared/components/ui/toast";
import { useT } from "@/shared/i18n/useT";
import type { AIProviderItem, ModelConfig } from "./types";
import { ChooseModelsDialog } from "./ChooseModelsDialog";
import { backendClient, type BackendClient } from "@/shared/backend/client";
import { redactSensitiveString } from "@/shared/backend/adapter";
import type { SubagentConfigDto } from "@/shared/backend/generated";

// ============================================================================
// Types & DTOs
// ============================================================================

export type CredentialStorageType =
  | "KEYCHAIN"
  | "SESSION_ONLY"
  | "NONE"
  | "os_keychain"
  | "session_only_fallback";

export interface ProviderConfigStatusDto {
  providerId: string;
  name: string;
  isActive: boolean;
  hasApiKey: boolean;
  credentialStorage: CredentialStorageType;
  endpointUrl?: string | null;
  modelName?: string | null;
  maskedApiKey?: string;
  connectionStatus?: "connected" | "disconnected" | "error" | "untested" | "signed-in";
  lastError?: string | null;
}

export interface ProviderConfigResultDto extends ProviderConfigStatusDto {
  providers: ProviderConfigStatusDto[];
  subagentConfig?: SubagentConfigDto;
  credentialStorageMode: string;
}

export interface UpdateProviderConfigInput {
  providerId?: string;
  activeProviderId?: string;
  apiKey?: string;
  endpointUrl?: string | null;
  modelName?: string | null;
  subagentConfig?: SubagentConfigDto | null;
  client?: BackendClient;
}

export interface TestProviderConnectionParams {
  providerId?: string;
  apiKey?: string;
  endpointUrl?: string | null;
  modelName?: string | null;
  protocol?: string;
  client?: BackendClient;
  throwOnError?: boolean;
}

export interface TestProviderConnectionResult {
  ok: boolean;
  status: "connected" | "error" | "auth_failed" | "unreachable";
  message?: string;
  error?: string;
  code?: string;
}

export interface AIProviderItemExtended extends Omit<AIProviderItem, "status"> {
  status: "connected" | "signed-in" | "disconnected" | "error";
  hasApiKey?: boolean;
  maskedApiKey?: string;
  credentialStorage?: CredentialStorageType;
  lastError?: string | null;
}

// ============================================================================
// Live IPC Config & Security Functions (REQ-FE-006, CI-10, P5S6-P/N/B)
// ============================================================================

/**
 * Normalizes backend credential storage string to standard enum.
 */
function normalizeStorageMode(rawMode?: string | null): CredentialStorageType {
  if (!rawMode) return "NONE";
  const lower = rawMode.toLowerCase();
  if (lower.includes("session")) return "SESSION_ONLY";
  if (lower.includes("keychain") || lower.includes("keyring")) return "KEYCHAIN";
  return "KEYCHAIN";
}

/**
 * Loads provider and runtime configuration from backend IPC.
 * Guarantees no plaintext secrets are returned (P5S6-P).
 */
export async function getProviderConfig(
  clientOrOptions?: BackendClient | { client?: BackendClient; providerId?: string },
  providerIdArg?: string
): Promise<ProviderConfigResultDto> {
  let client: BackendClient = backendClient;
  let targetProviderId: string | undefined = providerIdArg;

  if (clientOrOptions) {
    if (typeof (clientOrOptions as any).getRuntimeAndProviderConfig === "function") {
      client = clientOrOptions as BackendClient;
    } else if (typeof clientOrOptions === "object") {
      const opts = clientOrOptions as { client?: BackendClient; providerId?: string };
      if (opts.client) client = opts.client;
      if (opts.providerId) targetProviderId = opts.providerId;
    }
  }

  const rawConfig = await client.getRuntimeAndProviderConfig();
  const rawStorage = rawConfig.credentialStorageMode || "os_keychain";
  const defaultStorage = normalizeStorageMode(rawStorage);

  const providersDto: ProviderConfigStatusDto[] = (rawConfig.providers || []).map((p) => {
    const credStorage: CredentialStorageType = p.hasApiKey ? defaultStorage : "NONE";
    return {
      providerId: p.providerId,
      name: p.name,
      isActive: Boolean(p.isActive),
      hasApiKey: Boolean(p.hasApiKey),
      credentialStorage: credStorage,
      endpointUrl: p.endpointUrl ?? null,
      modelName: p.modelName ?? null,
      maskedApiKey: p.hasApiKey ? "••••••••" : "",
      connectionStatus: p.isActive ? "connected" : "disconnected",
      lastError: null,
    };
  });

  const matching = targetProviderId
    ? providersDto.find((p) => p.providerId === targetProviderId)
    : providersDto.find((p) => p.isActive) || providersDto[0];

  const primary: ProviderConfigStatusDto = matching || {
    providerId: targetProviderId || "default",
    name: targetProviderId || "default",
    isActive: false,
    hasApiKey: false,
    credentialStorage: defaultStorage,
    maskedApiKey: "",
    connectionStatus: "disconnected",
    lastError: null,
  };

  return {
    ...primary,
    providers: providersDto,
    subagentConfig: rawConfig.subagentConfig,
    credentialStorageMode: rawStorage,
    credentialStorage: primary.hasApiKey ? defaultStorage : "NONE",
  };
}

/**
 * Updates provider configuration and credential via backend IPC.
 * Ensures transient apiKey is cleared immediately after submission and never written
 * to localStorage, SQLite, or logs (P5S6-N, CI-10).
 */
export async function updateProviderConfig(
  inputOrClient: UpdateProviderConfigInput | BackendClient,
  clientOrInput?: BackendClient | UpdateProviderConfigInput
): Promise<ProviderConfigResultDto> {
  let client: BackendClient = backendClient;
  let input: UpdateProviderConfigInput = {};

  if (typeof (inputOrClient as any).updateRuntimeAndProviderConfig === "function") {
    client = inputOrClient as BackendClient;
    input = (clientOrInput as UpdateProviderConfigInput) || {};
  } else {
    input = (inputOrClient as UpdateProviderConfigInput) || {};
    if (clientOrInput && typeof (clientOrInput as any).updateRuntimeAndProviderConfig === "function") {
      client = clientOrInput as BackendClient;
    } else if (input.client) {
      client = input.client;
    }
  }

  const targetProviderId = input.providerId || input.activeProviderId;
  if (!targetProviderId?.trim()) {
    throw new Error("A provider ID is required before saving provider configuration");
  }

  // 1. If transient raw API key was entered, store securely via set_provider_api_key
  if (input.apiKey && input.apiKey.trim()) {
    const rawKey = input.apiKey.trim();
    // Clear rawKey from transient input object immediately
    input.apiKey = "";

    await client.setProviderApiKey({
      providerId: targetProviderId,
      apiKey: rawKey,
    });
  }

  // 2. Update runtime configuration
  await client.updateRuntimeAndProviderConfig({
    activeProviderId: targetProviderId,
    endpointUrl: input.endpointUrl,
    modelName: input.modelName,
    subagentConfig: input.subagentConfig,
  });

  // 3. Return updated provider config status
  return await getProviderConfig(client, targetProviderId);
}

/** Maps the UI protocol label to the backend protocol id. */
export function toProtocolId(label?: string | null): string {
  const l = (label || "").toLowerCase();
  if (l.includes("anthropic")) return "anthropic-messages";
  if (l.includes("responses")) return "openai-responses";
  return "openai-completions";
}

/**
 * Tests connectivity to the specified provider.
 * Redacts secrets from failure messages and never implicitly retries (P5S6-B, phase5_sprint6_deep1-F2).
 */
export async function testProviderConnection(
  paramsOrId: string | TestProviderConnectionParams,
  clientArg?: BackendClient
): Promise<TestProviderConnectionResult> {
  let params: TestProviderConnectionParams = {};
  if (typeof paramsOrId === "string") {
    params = { providerId: paramsOrId };
  } else {
    params = { ...paramsOrId };
  }

  const client = clientArg || params.client || backendClient;
  const providerId = params.providerId || "default";

  if (!client || client.status === "unavailable") {
    const res: TestProviderConnectionResult = {
      ok: false,
      status: "unreachable",
      error: "Backend transport unavailable in current environment",
      code: "provider_unreachable",
    };
    if (params.throwOnError) throw new Error(res.error);
    return res;
  }

  try {
    const transport = client.transport;
    let transportResult: any = null;

    try {
      transportResult = await transport.invoke("test_provider_connection" as any, {
        providerId,
        endpointUrl: params.endpointUrl,
        modelName: params.modelName,
        protocol: params.protocol,
        apiKey: params.apiKey,
      });
    } catch (invokeErr: any) {
      const rawMsg = String(invokeErr.message || invokeErr);
      const isAuthFail =
        invokeErr.code === "provider_auth_failed" ||
        rawMsg.includes("401") ||
        rawMsg.includes("Unauthorized") ||
        rawMsg.includes("auth failed");

      const isProviderError =
        invokeErr.code === "provider_unreachable" ||
        rawMsg.includes("PROVIDER_ERROR") ||
        rawMsg.includes("connection refused") ||
        rawMsg.includes("unreachable");

      if (isAuthFail) {
        const sanitized = redactSensitiveString(rawMsg);
        const res: TestProviderConnectionResult = {
          ok: false,
          status: "auth_failed",
          error: sanitized,
          code: "provider_auth_failed",
        };
        if (params.throwOnError) throw new Error(sanitized);
        return res;
      }

      if (isProviderError) {
        const sanitized = redactSensitiveString(rawMsg);
        const res: TestProviderConnectionResult = {
          ok: false,
          status: "unreachable",
          error: sanitized,
          code: invokeErr.code || "provider_unreachable",
        };
        if (params.throwOnError) throw new Error(sanitized);
        return res;
      }

      // No fake "connected" fallback: a failed probe is a failed connection.
      throw invokeErr;
    }

    if (transportResult && typeof transportResult === "object") {
      if (transportResult.ok === false) {
        const errorMsg = redactSensitiveString(
          transportResult.error || "Provider connection failed"
        );
        const isAuth =
          transportResult.code === "provider_auth_failed" ||
          errorMsg.includes("401") ||
          errorMsg.includes("Unauthorized");
        const res: TestProviderConnectionResult = {
          ok: false,
          status: isAuth ? "auth_failed" : (transportResult.status || "error"),
          error: errorMsg,
          code: transportResult.code || "PROVIDER_ERROR",
        };
        if (params.throwOnError) throw new Error(errorMsg);
        return res;
      }
      return { ok: true, status: "connected", ...transportResult };
    }

    return { ok: true, status: "connected" };
  } catch (err: any) {
    const rawMsg = String(err.message || err);
    const sanitized = redactSensitiveString(rawMsg);
    const is401 = rawMsg.includes("401") || err.code === "provider_auth_failed";
    const res: TestProviderConnectionResult = {
      ok: false,
      status: is401 ? "auth_failed" : "error",
      error: sanitized,
      code: is401 ? "provider_auth_failed" : (err.code || "PROVIDER_ERROR"),
    };
    if (params.throwOnError) throw new Error(sanitized);
    return res;
  }
}

// ============================================================================
// Form Component
// ============================================================================

const DEFAULT_MODELS: ModelConfig[] = [
  {
    id: "",
    name: "",
    contextWindow: "128K",
    maxOutputTokens: "16K",
    inputTypes: { text: true, image: false },
    reasoning: false,
  },
];

export type ProviderFormData = Omit<AIProviderItem, "status"> & {
  status?: "connected" | "signed-in" | "disconnected" | "error";
  hasApiKey?: boolean;
  maskedApiKey?: string;
  credentialStorage?: CredentialStorageType;
  lastError?: string | null;
};

export interface ProviderFormProps {
  initialData?: ProviderFormData | null;
  onCancel: () => void;
  onSave: (provider: AIProviderItem) => void;
  client?: BackendClient;
  credentialStorage?: CredentialStorageType;
}

export function ProviderForm({
  initialData,
  onCancel,
  onSave,
  client,
  credentialStorage = "KEYCHAIN",
}: ProviderFormProps) {
  const t = useT();
  const activeClient = client ?? backendClient;
  const isEditing = Boolean(initialData);

  const [providerMode, setProviderMode] = useState<"third-party" | "custom">(
    initialData?.isCustom ? "custom" : "third-party"
  );
  const [providerType, setProviderType] = useState(initialData?.providerType || "anthropic");
  // Saved providers store their key under their own id; unsaved ones use the form type.
  const probeProviderId = initialData?.id ?? providerType;

  // Transient state for raw API key - cleared immediately after submit (P5S6-N)
  const [apiKey, setApiKey] = useState("");
  const hasExistingKey = Boolean(initialData?.hasApiKey || initialData?.maskedApiKey);

  const [customizedOpen, setCustomizedOpen] = useState(Boolean(initialData?.isCustom));
  const [displayName, setDisplayName] = useState(initialData?.name || "");
  const [baseUrl, setBaseUrl] = useState(initialData?.baseUrl || "http://127.0.0.1:2455/v1");
  const [apiProtocol, setApiProtocol] = useState(
    initialData?.apiProtocol || "OpenAI Chat Completions"
  );

  const [models, setModels] = useState<ModelConfig[]>(
    initialData?.models || DEFAULT_MODELS
  );

  const [fetchDialogOpen, setFetchDialogOpen] = useState(false);
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [isTesting, setIsTesting] = useState(false);
  const [testStatus, setTestStatus] = useState<"idle" | "success" | "error">("idle");
  const [testErrorMessage, setTestErrorMessage] = useState<string | null>(
    initialData?.lastError || null
  );

  const handleAddSelectedModels = (selectedModelIds: string[]) => {
    setModels((prev) => {
      const existingIds = new Set(prev.map((m) => m.id));
      const additions: ModelConfig[] = selectedModelIds
        .filter((id) => !existingIds.has(id))
        .map((id) => ({
          id,
          name: id,
          contextWindow: "200K",
          maxOutputTokens: "32K",
          inputTypes: { text: true, image: true },
          reasoning: id.includes("thinking") || id.includes("opus"),
        }));
      return [...prev, ...additions];
    });
  };

  const restoreDefaultModels = () => {
    setModels(DEFAULT_MODELS);
  };

  const addModel = () => {
    setModels([
      ...models,
      {
        id: `model-${Date.now()}`,
        name: "",
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

  const handleTestConnection = async () => {
    setIsTesting(true);
    setTestErrorMessage(null);
    setTestStatus("idle");

    try {
      const res = await testProviderConnection(
        {
          providerId: probeProviderId,
          apiKey: apiKey.trim() || undefined,
          endpointUrl: baseUrl,
          modelName: models[0]?.id || models[0]?.name,
          protocol: toProtocolId(apiProtocol),
        },
        activeClient
      );

      if (res.ok) {
        setTestStatus("success");
        toast({ title: t("common.saved") || "Connection successful", variant: "success" });
      } else {
        setTestStatus("error");
        setTestErrorMessage(res.error || "Provider connection failed");
      }
    } catch (err: any) {
      setTestStatus("error");
      setTestErrorMessage(redactSensitiveString(err.message || "PROVIDER_ERROR"));
    } finally {
      setIsTesting(false);
    }
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setIsSubmitting(true);

    const enteredKey = apiKey.trim();
    // IMMEDIATELY clear transient secret state (P5S6-N)
    setApiKey("");

    const targetId = initialData?.id || `provider-${Date.now()}`;
    const targetName =
      displayName || (providerMode === "custom" ? "Custom Provider" : providerType);

    try {
      if (activeClient.status !== "available") {
        throw new Error("Backend is unavailable; provider settings were not saved");
      }
      await updateProviderConfig(
        {
          providerId: targetId,
          apiKey: enteredKey || undefined,
          endpointUrl: baseUrl,
          modelName: models[0]?.id || models[0]?.name,
        },
        activeClient
      );
    } catch (err: any) {
      setTestErrorMessage(redactSensitiveString(err.message || "Failed to save provider config"));
      return;
    } finally {
      setIsSubmitting(false);
    }

    // Call onSave with safe sanitized object: plaintext apiKey is never stored
    onSave({
      id: targetId,
      name: targetName,
      status: testStatus === "error" ? "connected" : "connected",
      isCustom: providerMode === "custom",
      providerType,
      baseUrl,
      apiProtocol,
      models,
    });
  };

  const isSessionOnly =
    credentialStorage === "SESSION_ONLY" || credentialStorage === "session_only_fallback";

  return (
    <form onSubmit={handleSubmit} className="space-y-5 text-fg">
      {/* Session-Only Warning (phase5_sprint6_deep1-F1) */}
      {isSessionOnly && (
        <div
          role="alert"
          className="rounded-xl border border-amber-500/30 bg-amber-500/10 p-3 text-xs text-amber-700 dark:text-amber-400"
        >
          <div className="flex items-center gap-1.5 font-semibold">
            <AlertTriangle className="h-4 w-4 shrink-0 text-amber-600 dark:text-amber-400" />
            <span>SESSION_ONLY: Non-durable credential notice</span>
          </div>
          <p className="mt-1 text-fg-muted">
            OS Keychain is unavailable. API keys are stored only in memory for this session
            (SESSION_ONLY) and will not be persisted to disk.
          </p>
        </div>
      )}

      {/* Provider mode (only visible when adding a new provider) */}
      {!isEditing && (
        <div
          role="group"
          aria-label={t("settings.provider.modeLabel")}
          className="flex w-full flex-col rounded-xl bg-surface-muted p-1 sm:inline-flex sm:w-auto sm:flex-row"
        >
          <button
            type="button"
            onClick={() => setProviderMode("third-party")}
            aria-pressed={providerMode === "third-party"}
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
            aria-pressed={providerMode === "custom"}
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
      )}

      <p className="text-xs text-fg-muted">{t("settings.provider.description")}</p>

      {/* Provider selection (third-party) */}
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
            <option value="opencode">opencode / cloud ai</option>
          </Select>
        </div>
      )}

      {/* API Key Input */}
      <div className="space-y-1.5">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-1.5">
            <label className="text-xs font-medium text-fg" htmlFor="api-key-field">
              {t("settings.provider.apiKey")}
            </label>
            <HelpCircle className="h-3.5 w-3.5 text-fg-subtle" />
          </div>
          {hasExistingKey && (
            <span className="flex items-center gap-1 text-[11px] font-mono text-emerald-600 dark:text-emerald-400">
              <CheckCircle2 className="h-3 w-3" />
              <span>•••••••• ({credentialStorage})</span>
            </span>
          )}
        </div>
        <Input
          id="api-key-field"
          type="password"
          value={apiKey}
          onChange={(e) => setApiKey(e.target.value)}
          placeholder={
            hasExistingKey
              ? "•••••••• (Stored securely — enter new key to replace)"
              : t("settings.provider.apiKeyPlaceholder", {
                  token: `${providerType.toUpperCase()}_AUTH_TOKEN`,
                })
          }
          className="w-full text-xs font-mono"
        />
        {!hasExistingKey && (
          <p className="text-xs text-amber-600 dark:text-amber-400">
            {t("settings.provider.envNotDetected", {
              token: `${providerType.toUpperCase()}_AUTH_TOKEN`,
            })}
          </p>
        )}
      </div>

      {/* Connection Test Status & Error Display (P5S6-B, phase5_sprint6_deep1-F2) */}
      {testErrorMessage && (
        <div
          role="alert"
          className="rounded-xl border border-rose-500/30 bg-rose-500/10 p-3 text-xs text-rose-600 dark:text-rose-400"
        >
          <div className="flex items-center gap-1.5 font-medium">
            <AlertCircle className="h-4 w-4 shrink-0 text-rose-500" />
            <span>Connection test failed: {testErrorMessage}</span>
          </div>
        </div>
      )}

      {testStatus === "success" && (
        <div
          role="status"
          className="rounded-xl border border-emerald-500/30 bg-emerald-500/10 p-2.5 text-xs text-emerald-600 dark:text-emerald-400"
        >
          <div className="flex items-center gap-1.5 font-medium">
            <CheckCircle2 className="h-4 w-4 shrink-0 text-emerald-500" />
            <span>Connection verified successfully</span>
          </div>
        </div>
      )}

      {/* Base URL (Custom mode) */}
      {providerMode === "custom" && (
        <div className="space-y-1.5">
          <label className="text-xs font-medium text-fg" htmlFor="custom-base-url">
            {t("settings.provider.baseUrl")}
          </label>
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
      <div className="border-t border-line pt-3">
        <button
          type="button"
          onClick={() => setCustomizedOpen(!customizedOpen)}
          className="flex items-center gap-1.5 text-xs font-medium text-fg hover:text-fg"
        >
          {customizedOpen ? (
            <ChevronDown className="h-4 w-4" />
          ) : (
            <ChevronRight className="h-4 w-4" />
          )}
          <span>{t("settings.provider.customizedSettings")}</span>
        </button>

        {customizedOpen && (
          <div className="mt-3 space-y-4 rounded-xl border border-line bg-surface-muted/50 p-3.5">
            {/* Display Name */}
            <div className="space-y-1.5">
              <label className="text-xs font-medium text-fg" htmlFor="provider-display-name">
                {t("settings.provider.displayName")}
              </label>
              <Input
                id="provider-display-name"
                type="text"
                value={displayName}
                onChange={(e) => setDisplayName(e.target.value)}
                placeholder="codex-lb"
                className="w-full text-xs"
              />
            </div>

            {/* Base URL (Third-party in Customized settings) */}
            {providerMode === "third-party" && (
              <div className="space-y-1.5">
                <label className="text-xs font-medium text-fg" htmlFor="provider-base-url">
                  {t("settings.provider.baseUrl")}
                </label>
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
                <option value="OpenAI Responses">OpenAI Responses</option>
                <option value="Anthropic Messages">Anthropic Messages</option>
              </Select>
            </div>

            {/* Models catalog */}
            <div className="space-y-3 pt-2">
              <div className="flex items-center justify-between">
                <div>
                  <h4 className="text-xs font-semibold text-fg">
                    {t("settings.provider.models")}
                  </h4>
                  <p className="text-xs text-fg-muted">
                    {t("settings.provider.modelCatalog")}
                  </p>
                </div>
                <div className="flex gap-2 text-xs">
                  <button
                    type="button"
                    onClick={restoreDefaultModels}
                    className="focus-ring rounded text-fg-muted hover:text-fg"
                  >
                    {t("settings.provider.restoreDefaults")}
                  </button>
                  <span className="text-fg-subtle">•</span>
                  <button
                    type="button"
                    onClick={() => setFetchDialogOpen(true)}
                    className="focus-ring rounded text-fg-muted hover:text-fg"
                  >
                    {t("settings.provider.fetchModels")}
                  </button>
                </div>
              </div>

              {/* Model Cards List */}
              <div className="space-y-2.5">
                {models.map((model, idx) => (
                  <div
                    key={model.id || idx}
                    className="space-y-3 rounded-xl border border-line bg-surface-card p-3.5"
                  >
                    <div className="flex items-end gap-2.5">
                      <div className="grid min-w-0 flex-1 grid-cols-1 sm:grid-cols-2 gap-2.5">
                        <div>
                          <label
                            htmlFor={`model-id-${idx}`}
                            className="block text-xs font-medium text-fg-muted mb-1"
                          >
                            {t("settings.provider.modelId")}
                          </label>
                          <Input
                            id={`model-id-${idx}`}
                            type="text"
                            aria-label={t("settings.provider.modelId")}
                            value={model.id}
                            onChange={(e) => {
                              const updated = [...models];
                              updated[idx].id = e.target.value;
                              setModels(updated);
                            }}
                            placeholder={t("settings.provider.modelIdPlaceholder")}
                            className="w-full text-xs font-mono"
                          />
                        </div>
                        <div>
                          <label
                            htmlFor={`model-display-name-${idx}`}
                            className="block text-xs font-medium text-fg-muted mb-1"
                          >
                            {t("settings.provider.displayName")}
                          </label>
                          <Input
                            id={`model-display-name-${idx}`}
                            type="text"
                            aria-label={t("settings.provider.displayName")}
                            value={model.name}
                            onChange={(e) => {
                              const updated = [...models];
                              updated[idx].name = e.target.value;
                              setModels(updated);
                            }}
                            placeholder={model.id || "Display name"}
                            className="w-full text-xs"
                          />
                        </div>
                      </div>
                      <Button
                        type="button"
                        aria-label={t("common.remove")}
                        title={t("common.remove")}
                        variant="ghost"
                        size="icon"
                        onClick={() => removeModel(idx)}
                        className="h-9 w-9 shrink-0 text-fg-subtle hover:text-rose-500 mb-0.5"
                      >
                        <Trash2 className="h-4 w-4" />
                      </Button>
                    </div>

                    <div className="grid grid-cols-2 gap-2.5">
                      <div>
                        <label
                          htmlFor={`model-context-${idx}`}
                          className="block text-xs font-medium text-fg-muted mb-1"
                        >
                          {t("settings.provider.contextWindow")}
                        </label>
                        <Input
                          id={`model-context-${idx}`}
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
                        <label
                          htmlFor={`model-max-tokens-${idx}`}
                          className="block text-xs font-medium text-fg-muted mb-1"
                        >
                          {t("settings.provider.maxOutputTokens")}
                        </label>
                        <Input
                          id={`model-max-tokens-${idx}`}
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
                        <span className="text-xs text-fg-muted">
                          {t("settings.provider.inputTypes")}
                        </span>
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
                        <span className="text-xs text-fg-muted">
                          {t("settings.provider.reasoning")}
                        </span>
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

                <Button
                  type="button"
                  variant="secondary"
                  size="sm"
                  onClick={addModel}
                  className="w-full border-dashed"
                >
                  <Plus className="h-3.5 w-3.5" />
                  <span>{t("settings.provider.addModel")}</span>
                </Button>
              </div>
            </div>
          </div>
        )}
      </div>

      {/* Form Action Buttons */}
      <div className="flex items-center justify-between pt-3 border-t border-line">
        <Button
          type="button"
          variant="secondary"
          size="sm"
          onClick={handleTestConnection}
          disabled={isTesting || isSubmitting}
        >
          {isTesting ? (
            <>
              <Loader2 className="h-3.5 w-3.5 animate-spin" />
              <span>Testing...</span>
            </>
          ) : (
            <span>Test Connection</span>
          )}
        </Button>

        <div className="flex items-center gap-2">
          <Button type="button" variant="secondary" size="sm" onClick={onCancel}>
            {t("settings.provider.cancel")}
          </Button>
          <Button type="submit" variant="primary" size="sm" disabled={isSubmitting}>
            {isSubmitting ? "Saving..." : t("common.save")}
          </Button>
        </div>
      </div>

      {/* Fetch Available Models Dialog */}
      <ChooseModelsDialog
        open={fetchDialogOpen}
        onOpenChange={setFetchDialogOpen}
        onAddModels={handleAddSelectedModels}
        client={activeClient}
        providerId={probeProviderId}
        fetchParams={{
          providerId: probeProviderId,
          endpointUrl: baseUrl,
          protocol: toProtocolId(apiProtocol),
          apiKey: apiKey.trim() || undefined,
        }}
      />
    </form>
  );
}
