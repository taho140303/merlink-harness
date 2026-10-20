import { useState, useEffect, useRef } from "react";
import { invokeCommand } from "../lib/ipc";
import {
  applyTextScale,
  readTextScale,
  TEXT_SCALE_OPTIONS,
} from "../lib/preferences";
import { applyTheme, readTheme, type Theme } from "../lib/theme";
import {
  Settings,
  Bot,
  X,
  Sun,
  Moon,
  Monitor,
  Pencil,
  Trash2,
  Plus,
  Eye,
  EyeOff,
  ChevronUp,
  ChevronDown,
} from "lucide-react";
import type { ProviderConfig, ProviderKind, ModelConfig } from "../types";

export interface SettingsDialogProps {
  open: boolean;
  onClose: () => void;
  initialTab?: "general" | "llm-provider";
  onProvidersChanged: () => void;
}

function createModelConfig(
  model: string | Partial<ModelConfig>,
): ModelConfig {
  const value =
    typeof model === "string"
      ? { id: model, name: model }
      : model && typeof model === "object"
        ? model
        : {};
  // Sprint 11.13 · D2: there is no preset table left, so every new model starts
  // with the plain defaults the author can then edit.
  return {
    id: typeof value.id === "string" ? value.id : "",
    name: typeof value.name === "string" ? value.name : String(value.id || ""),
    context_window: value.context_window || "128K",
    max_tokens: value.max_tokens || "4K",
    input_types: {
      text: value.input_types?.text ?? true,
      image: value.input_types?.image ?? false,
    },
    reasoning_level: value.reasoning_level ?? false,
  };
}

function isLocalOllamaURL(value: string): boolean {
  try {
    const host = new URL(value).hostname.toLowerCase();
    return host === "localhost" || host === "127.0.0.1" || host === "[::1]" || host === "::1";
  } catch {
    return false;
  }
}

interface FormState {
  kind: ProviderKind;
  id: string;
  name: string;
  baseURL: string;
  apiKey: string;
  models: ModelConfig[];
  defaultModel: string;
  showKey: boolean;
}

export function SettingsDialog({
  open,
  onClose,
  initialTab = "general",
  onProvidersChanged,
}: SettingsDialogProps) {
  const [tab, setTab] = useState<"general" | "llm-provider">(initialTab);
  const [language, setLanguage] = useState(() => {
    try {
      return localStorage.getItem("merlink-language") || "en";
    } catch {
      return "en";
    }
  });

  const [theme, setThemeState] = useState<Theme>(readTheme);

  // D1: the active button is the persisted scale, rounded to its percentage.
  const [textScalePercent, setTextScalePercent] = useState<number>(() =>
    Math.round(readTextScale() * 100),
  );

  const [providers, setProviders] = useState<ProviderConfig[]>([]);
  const [editor, setEditor] = useState<{ id: string | null } | null>(null);
  const editorGenerationRef = useRef(0);
  const savingRef = useRef(false);
  const [saving, setSaving] = useState(false);
  const [form, setForm] = useState<FormState>({
    kind: "custom",
    id: "",
    name: "",
    baseURL: "",
    apiKey: "",
    models: [],
    defaultModel: "",
    showKey: false,
  });
  const formRef = useRef(form);
  formRef.current = form;
  const [notice, setNotice] = useState("");
  const [expandedModel, setExpandedModel] = useState<number>(-1);
  const [fetching, setFetching] = useState(false);
  /** D5: fetch failures are shown under the button, never only in the console. */
  const [fetchError, setFetchError] = useState("");
  /** Result of a successful fetch: how many models were new (D6). */
  const [fetchNotice, setFetchNotice] = useState("");

  const closeRef = useRef<HTMLButtonElement>(null);
  const editorRef = useRef<HTMLDivElement>(null);
  const editorOpenerRef = useRef<HTMLElement | null>(null);
  const panelRef = useRef<HTMLDivElement>(null);

  // Sync initial tab when opening and invalidate an editor closed with Settings.
  useEffect(() => {
    if (open) {
      setTab(initialTab);
      void loadProvidersFromBackend();
    } else {
      editorGenerationRef.current += 1;
      setEditor(null);
      resetFetchState();
    }
  }, [open, initialTab]);

  const loadProvidersFromBackend = async () => {
    try {
      const data = await invokeCommand("load_providers");
      if (Array.isArray(data)) {
        setProviders(data);
      }
    } catch (err) {
      console.error("Failed to load providers from backend:", err);
    }
  };

  const chooseTheme = (nextTheme: Theme) => {
    setThemeState(nextTheme);
    applyTheme(nextTheme, true);
  };

  /**
   * D1: persist the percentage (`"125"`) and apply the matching ratio (`1.25`)
   * immediately — no reload and no restart needed.
   */
  const applyTextScalePreference = (percent: number) => {
    setTextScalePercent(percent);
    try {
      localStorage.setItem("merlink-text-scale", String(percent));
    } catch (e) {
      console.error(e);
    }
    void applyTextScale(percent / 100);
  };

  // The outer Settings dialog owns scroll locking, not its nested editor.
  useEffect(() => {
    if (!open) return;
    const oldOverflow = document.body.style.overflow;
    document.body.style.overflow = "hidden";
    closeRef.current?.focus();
    return () => { document.body.style.overflow = oldOverflow; };
  }, [open]);

  useEffect(() => {
    if (!open) return;
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === "Escape") {
        e.preventDefault();
        if (editor) closeEditor();
        else onClose();
      }
    };
    document.addEventListener("keydown", handleKeyDown);
    return () => document.removeEventListener("keydown", handleKeyDown);
  }, [open, editor, onClose]);

  useEffect(() => {
    if (!open || !editor) return;
    const panel = editorRef.current;
    if (!panel) return;
    const opener = editorOpenerRef.current;
    const selectors =
      'a[href], button:not([disabled]), input:not([disabled]), select:not([disabled]), textarea:not([disabled]), [tabindex]:not([tabindex="-1"])';
    const focusable = () => [...panel.querySelectorAll<HTMLElement>(selectors)].filter(
      (element) => element.getClientRects().length &&
        getComputedStyle(element).visibility !== "hidden",
    );
    (focusable()[0] ?? panel).focus();

    const trapTab = (event: KeyboardEvent) => {
      if (event.key !== "Tab") return;
      // A generic Dialog portal may be above Settings; only the top layer traps.
      const layers = document.querySelectorAll(
        ".merlink-provider-backdrop, .merlink-dialog-backdrop",
      );
      if (layers[layers.length - 1] !== panel.parentElement) return;
      const elements = focusable();
      if (!elements.length) {
        event.preventDefault();
        panel.focus();
        return;
      }
      const first = elements[0];
      const last = elements[elements.length - 1];
      if (!panel.contains(document.activeElement) || document.activeElement === panel) {
        event.preventDefault();
        (event.shiftKey ? last : first).focus();
      } else if (event.shiftKey && document.activeElement === first) {
        event.preventDefault();
        last.focus();
      } else if (!event.shiftKey && document.activeElement === last) {
        event.preventDefault();
        first.focus();
      }
    };
    document.addEventListener("keydown", trapTab);
    return () => {
      document.removeEventListener("keydown", trapTab);
      if (opener?.isConnected) opener.focus();
    };
  }, [open, editor]);

  /** Clears the fetch banner whenever the editor moves to another form. */
  const resetFetchState = () => {
    setFetching(false);
    setFetchError("");
    setFetchNotice("");
  };

  const closeEditor = () => {
    if (savingRef.current) return;
    editorGenerationRef.current += 1;
    resetFetchState();
    setEditor(null);
  };

  const newFormState = (source: ProviderConfig | null): FormState => {
    const models = (source?.models ?? []).map(createModelConfig);
    return {
      kind: source?.kind === "anthropic" || source?.kind === "openai" || source?.kind === "ollama"
        ? source.kind : "custom",
      id: source?.id ?? "",
      name: source?.name ?? "",
      baseURL: source?.base_url ?? "",
      apiKey: source?.api_key ?? "",
      models,
      defaultModel: source?.default_model_id || models[0]?.id || "",
      showKey: false,
    };
  };

  /**
   * D2: adding a provider opens an **empty** form. There is no catalog to pick a
   * preset from any more, so the author fills in the Base URL, the key, then
   * fetches models (or adds them by hand). A new provider is `custom` by default.
   */
  const openEditor = (provider: ProviderConfig | null = null) => {
    if (savingRef.current) return;
    editorOpenerRef.current = document.activeElement instanceof HTMLElement
      ? document.activeElement : null;
    editorGenerationRef.current += 1;
    resetFetchState();
    setForm(newFormState(provider));
    setExpandedModel(-1);
    setEditor({ id: provider?.id ?? null });
  };

  const showAdvanced = editor?.id === null || form.kind === "custom" || Boolean(form.baseURL);

  const updateModel = (index: number, patch: Partial<ModelConfig>) => {
    setForm((prev) => ({
      ...prev,
      models: prev.models.map((m, i) => (i === index ? { ...m, ...patch } : m)),
    }));
  };

  const providerBaseURL = () => form.baseURL.trim();

  /**
   * Asks the provider for its real model list (Sprint 11.5 · D1/D5/D6).
   *
   * The request is made by the Rust `fetch_models` command, not from the webview:
   * the API key stays out of the browser context and there is no CORS preflight.
   * Results are **merged** into the model list, deduped by `id`, and existing rows
   * are left untouched; failures are rendered under the button instead of being
   * logged away.
   */
  const fetchModels = async () => {
    if (fetching || !editor) return;
    const generation = editorGenerationRef.current;
    setFetching(true);
    setFetchError("");
    setFetchNotice("");
    try {
      // D11: Tauri expects camelCase keys for Rust parameter names.
      const fetched = await invokeCommand("fetch_models", {
        providerKind: form.kind,
        baseUrl: providerBaseURL(),
        apiKey: form.apiKey.trim(),
      });
      if (generation !== editorGenerationRef.current) return;

      const current = formRef.current;
      const seen = new Set(current.models.map((model) => model.id.trim()));
      const incoming = Array.isArray(fetched) ? fetched : [];
      const added: ModelConfig[] = [];
      for (const item of incoming) {
        if (!item || typeof item.id !== "string" || !item.id.trim()) continue;
        const id = item.id.trim();
        if (seen.has(id)) continue;
        seen.add(id);
        added.push(createModelConfig({ id, name: item.name?.trim() || id }));
      }
      if (added.length === 0) {
        setFetchNotice(
          incoming.length === 0
            ? "Provider returned no models."
            : `Không có model mới (${current.models.length} model đang có).`,
        );
        return;
      }

      setForm((prev) => {
        const existing = new Set(prev.models.map((model) => model.id.trim()));
        const unique = added.filter((model) => !existing.has(model.id));
        return {
          ...prev,
          models: [...prev.models, ...unique],
          defaultModel: prev.defaultModel || unique[0]?.id || "",
        };
      });
      setFetchNotice(`Đã thêm ${added.length} model mới.`);
    } catch (err) {
      if (generation === editorGenerationRef.current) {
        setFetchError(err instanceof Error ? err.message : String(err));
      }
    } finally {
      if (generation === editorGenerationRef.current) setFetching(false);
    }
  };

  const validModels =
    form.models.length > 0 &&
    form.models.every((m) => m.id.trim() && m.name.trim()) &&
    new Set(form.models.map((m) => m.id.trim())).size === form.models.length;

  const ollamaLocal = form.kind === "ollama" && isLocalOllamaURL(form.baseURL);
  const valid = Boolean(
    (ollamaLocal || form.apiKey.trim()) &&
      validModels &&
      form.name.trim() &&
      /^https?:\/\/\S+/i.test(form.baseURL),
  );

  // The native IPC writes the entire list; serialize mutations in this window.
  // Other windows still have no cross-window merge/versioning.
  const persistProviders = async (next: ProviderConfig[], successMessage: string): Promise<boolean> => {
    if (savingRef.current) return false;
    savingRef.current = true;
    setSaving(true);
    try {
      await invokeCommand("save_providers", { providers: next });
      setProviders(next);
      setNotice(successMessage);
      onProvidersChanged();
      return true;
    } catch (err) {
      console.error("Failed to save providers:", err);
      setNotice(`Failed to save providers: ${err instanceof Error ? err.message : String(err)}`);
      return false;
    } finally {
      savingRef.current = false;
      setSaving(false);
    }
  };

  const handleSave = async () => {
    if (!valid || savingRef.current || !editor) return;
    // Editing keeps its id; a new id never determines the wire protocol.
    const id = editor.id ?? `custom-${Date.now()}`;
    if (!id) return;
    const existing = providers.find((item) => item.id === id);
    const cleaned = form.models.map((m) =>
      createModelConfig({
        ...m,
        id: m.id.trim(),
        name: m.name.trim(),
      }),
    );
    // D9: the default model must survive the merge — clamp it to a model that is
    // still in the list after trimming.
    const default_model_id = cleaned.some((m) => m.id === form.defaultModel.trim())
      ? form.defaultModel.trim()
      : cleaned[0]?.id || "";

    // D2: with no catalog left, the form is the only source of these values.
    const provider: ProviderConfig = {
      id,
      name: form.name.trim() || id,
      kind: form.kind,
      api_key: form.apiKey.trim(),
      base_url: form.baseURL.trim(),
      models: cleaned,
      default_model_id,
    };

    const nextProviders = existing
      ? providers.map((item) => (item.id === id ? provider : item))
      : [...providers, provider];

    if (await persistProviders(nextProviders, "Provider saved")) closeEditor();
  };

  const handleRemoveProvider = async (id: string) => {
    if (savingRef.current) return;
    const nextProviders = providers.filter((item) => item.id !== id);
    await persistProviders(nextProviders, "Provider deleted");
  };

  const handleClearKey = async () => {
    if (savingRef.current || !editor?.id) return;
    const nextProviders = providers.map((item) =>
      item.id === editor.id ? { ...item, api_key: "" } : item,
    );
    if (await persistProviders(nextProviders, "Saved API key cleared")) closeEditor();
  };

  if (!open) return null;

  return (
    <div
      className="merlink-settings-backdrop"
      onMouseDown={(e) => {
        if (e.target === e.currentTarget && !editor) onClose();
      }}
    >
      <section
        ref={panelRef}
        tabIndex={-1}
        role="dialog"
        aria-modal="true"
        aria-label="Settings"
        className="merlink-settings-panel"
      >
        <nav className="merlink-settings-tabs" aria-label="Settings sections">
          <div className="merlink-settings-brand">
            <Settings size={20} />
            <span>Settings</span>
          </div>
          <button
            type="button"
            className={`merlink-settings-tab ${tab === "general" ? "is-active" : ""}`}
            onClick={() => setTab("general")}
          >
            <Settings size={18} />
            <span>General</span>
          </button>
          <button
            type="button"
            className={`merlink-settings-tab ${tab === "llm-provider" ? "is-active" : ""}`}
            onClick={() => setTab("llm-provider")}
          >
            <Bot size={18} />
            <span>LLM Provider</span>
          </button>
        </nav>

        <div className="merlink-settings-content">
          <header className="merlink-settings-head">
            <span>{tab === "general" ? "GENERAL" : "LLM PROVIDER"}</span>
            <button
              ref={closeRef}
              type="button"
              className="merlink-settings-icon"
              onClick={onClose}
              aria-label="Close settings"
            >
              <X size={19} />
            </button>
          </header>

          <div className="merlink-settings-body">
            {tab === "general" ? (
              <>
                <h2>Appearance</h2>
                <p>Make your writing space feel like yours.</p>
                <label className="merlink-settings-field">
                  <span>Theme</span>
                  <div className="merlink-theme-options">
                    <button
                      type="button"
                      className={theme === "light" ? "is-active" : ""}
                      onClick={() => chooseTheme("light")}
                    >
                      <Sun size={17} />
                      <span>Light</span>
                    </button>
                    <button
                      type="button"
                      className={theme === "dark" ? "is-active" : ""}
                      onClick={() => chooseTheme("dark")}
                    >
                      <Moon size={17} />
                      <span>Dark</span>
                    </button>
                    <button
                      type="button"
                      className={theme === "system" ? "is-active" : ""}
                      onClick={() => chooseTheme("system")}
                    >
                      <Monitor size={17} />
                      <span>System</span>
                    </button>
                  </div>
                </label>

                {/* D1: same option-button pattern as the theme picker above. */}
                <h2>Cỡ chữ</h2>
                <p>Phóng to toàn bộ giao diện cho dễ đọc. Áp dụng ngay, không cần khởi động lại.</p>
                <label className="merlink-settings-field">
                  <div
                    className="merlink-theme-options merlink-text-scale-options"
                    role="group"
                    aria-label="Cỡ chữ"
                  >
                    {TEXT_SCALE_OPTIONS.map((option) => (
                      <button
                        key={option.value}
                        type="button"
                        className={textScalePercent === option.value ? "is-active" : ""}
                        aria-pressed={textScalePercent === option.value}
                        onClick={() => applyTextScalePreference(option.value)}
                      >
                        <span>
                          {option.label} {option.value}%
                        </span>
                      </button>
                    ))}
                  </div>
                </label>

                <h2>Language</h2>
                <label className="merlink-settings-field">
                  <span>Language</span>
                  <select
                    value={language}
                    onChange={(e) => {
                      setLanguage(e.target.value);
                      try {
                        localStorage.setItem("merlink-language", e.target.value);
                      } catch {}
                    }}
                  >
                    <option value="en">English</option>
                    <option value="vi">Tiếng Việt</option>
                    <option value="ja">日本語</option>
                    <option value="zh-Hans">简体中文</option>
                    <option value="zh-Hant">繁體中文</option>
                  </select>
                </label>
                <p className="merlink-settings-hint">
                  Language preference is saved; translation is coming later.
                </p>

                <div className="merlink-settings-about">
                  <strong>Merlink Harness · 0.1.0-alpha</strong>
                  <span>A local-first writing studio.</span>
                </div>
              </>
            ) : (
              <>
                <h2>Model providers</h2>
                <p>
                  Manage connections on this device. Configured providers are saved directly to your local filesystem.
                </p>
                {/* D2: nothing is seeded any more, so the empty list is a real,
                    expected state with a next step instead of a blank panel. */}
                {providers.length === 0 ? (
                  <p className="merlink-provider-empty" role="status">
                    Chưa có provider nào. Thêm provider để bắt đầu.
                  </p>
                ) : (
                  <div className="merlink-provider-list">
                    {providers.map((provider) => (
                    <div
                      key={provider.id}
                      className="merlink-provider-row"
                      role="button"
                      tabIndex={0}
                      onClick={() => openEditor(provider)}
                      onKeyDown={(e) => {
                        if (e.key === "Enter" || e.key === " ") {
                          e.preventDefault();
                          openEditor(provider);
                        }
                      }}
                    >
                      <span className="merlink-provider-avatar">
                        {provider.name.slice(0, 1)}
                      </span>
                      <span className="merlink-provider-details">
                        <strong>
                          {provider.name}
                          {provider.kind === "custom" && <small>Custom</small>}
                        </strong>
                        <span>
                          {provider.models.length} models configured ·{" "}
                          {provider.models.find(
                            (m) => m.id === provider.default_model_id,
                          )?.name ||
                            provider.models[0]?.name ||
                            "No model"}
                        </span>
                      </span>
                      <span
                        className={`merlink-provider-status ${
                          provider.api_key ? "is-configured" : ""
                        }`}
                        title={provider.api_key ? "Configured" : "Available"}
                      />
                      <button
                        type="button"
                        className="merlink-settings-icon"
                        onClick={(e) => {
                          e.stopPropagation();
                          openEditor(provider);
                        }}
                        aria-label={`Edit ${provider.name}`}
                      >
                        <Pencil size={17} />
                      </button>
                      <button
                        type="button"
                        className="merlink-settings-icon"
                        disabled={saving}
                        onClick={(e) => {
                          e.stopPropagation();
                          void handleRemoveProvider(provider.id);
                        }}
                        aria-label={`Delete ${provider.name}`}
                      >
                        <Trash2 size={17} />
                      </button>
                    </div>
                  ))}
                  </div>
                )}

                <button
                  type="button"
                  className="merlink-provider-add"
                  onClick={() => openEditor()}
                >
                  <Plus size={17} />
                  <span>Add model provider</span>
                </button>

                {notice && (
                  <p role="status" className="merlink-settings-hint">
                    {notice}
                  </p>
                )}
              </>
            )}
          </div>
        </div>
      </section>

      {/* Provider Edit Dialog (Nested Modal) */}
      {editor && (
        <div
          className="merlink-provider-backdrop"
          onMouseDown={(e) => {
            if (e.target === e.currentTarget) closeEditor();
          }}
        >
          <section
            ref={editorRef}
            tabIndex={-1}
            role="dialog"
            aria-modal="true"
            aria-label="Edit model provider"
            className="merlink-provider-editor"
          >
            <header className="merlink-settings-head">
              <strong>
                {editor.id ? "Edit model provider" : "Add model provider"}
              </strong>
              <button
                type="button"
                className="merlink-settings-icon"
                aria-label="Close provider editor"
                onClick={closeEditor}
              >
                <X size={18} />
              </button>
            </header>

            <div className="merlink-provider-editor-body">
              <label className="merlink-settings-field">
                <span>Protocol</span>
                <select
                  value={form.kind === "anthropic" ? "anthropic" : form.kind === "ollama" ? (isLocalOllamaURL(form.baseURL) ? "ollama-local" : "ollama-cloud") : "openai-compatible"}
                  onChange={(e) => setForm((prev) => {
                    const protocol = e.target.value;
                    if (protocol === "anthropic") return { ...prev, kind: "anthropic" };
                    if (protocol === "ollama-local") return { ...prev, kind: "ollama", name: prev.name || "Ollama (local)", baseURL: "http://127.0.0.1:11434/v1", apiKey: "" };
                    if (protocol === "ollama-cloud") return { ...prev, kind: "ollama", name: prev.name || "Ollama Cloud", baseURL: "https://ollama.com/v1" };
                    return { ...prev, kind: prev.kind === "anthropic" || prev.kind === "ollama" ? "custom" : prev.kind };
                  })}
                >
                  <option value="openai-compatible">OpenAI-compatible</option>
                  <option value="anthropic">Anthropic</option>
                  <option value="ollama-local">Ollama (local)</option>
                  <option value="ollama-cloud">Ollama Cloud</option>
                </select>
              </label>

              <p>{form.kind === "ollama" && ollamaLocal ? "Local Ollama detected/configured. API key is not required." : form.kind === "ollama" ? "Ollama Cloud uses your cloud API key and the OpenAI-compatible endpoint." : "Choose the provider's wire protocol, then enter its API key and endpoint."}</p>

              <label className="merlink-settings-field">
                <span>API key</span>
                <span className="merlink-key-input">
                  <input
                    type={form.showKey ? "text" : "password"}
                    value={form.apiKey}
                    required={!ollamaLocal}
                    placeholder={ollamaLocal ? "Not required for local Ollama" : "Enter an API key"}
                    onChange={(e) =>
                      setForm((prev) => ({ ...prev, apiKey: e.target.value }))
                    }
                  />
                  <button
                    type="button"
                    aria-label={form.showKey ? "Hide API key" : "Reveal API key"}
                    onClick={() =>
                      setForm((prev) => ({ ...prev, showKey: !prev.showKey }))
                    }
                  >
                    {form.showKey ? <EyeOff size={17} /> : <Eye size={17} />}
                  </button>
                </span>
              </label>

              <details
                className="merlink-provider-advanced"
                open={showAdvanced ? true : undefined}
              >
                <summary>Customized settings</summary>
                <div className="merlink-provider-custom-fields">
                  <label className="merlink-settings-field">
                    <span>Display name</span>
                    <input
                      value={form.name}
                      required
                      placeholder="Provider name"
                      onChange={(e) =>
                        setForm((prev) => ({ ...prev, name: e.target.value }))
                      }
                    />
                  </label>
                  <label className="merlink-settings-field">
                    <span>Base URL</span>
                    <input
                      type="url"
                      value={form.baseURL}
                      required
                      placeholder="https://api.example.com/v1"
                      onChange={(e) =>
                        setForm((prev) => ({
                          ...prev,
                          baseURL: e.target.value,
                        }))
                      }
                    />
                  </label>
                </div>

                <label className="merlink-settings-field">
                  <span>Default model</span>
                  <select
                    value={
                      form.models.some((m) => m.id === form.defaultModel)
                        ? form.defaultModel
                        : form.models[0]?.id || ""
                    }
                    onChange={(e) =>
                      setForm((prev) => ({
                        ...prev,
                        defaultModel: e.target.value,
                      }))
                    }
                  >
                    {form.models
                      .filter((m) => m.id)
                      .map((m, index) => (
                        <option key={`${index}-${m.id}`} value={m.id}>
                          {m.name || m.id}
                        </option>
                      ))}
                  </select>
                </label>

                {/* Models Section */}
                <section
                  className="merlink-model-section"
                  aria-label="Customized model catalog"
                >
                  <div className="merlink-model-heading">
                    <div>
                      <h3>Models</h3>
                      <span className="merlink-settings-hint">
                        Customized model catalog
                      </span>
                    </div>
                    <div className="merlink-model-tools">
                      {/* D2: no preset table left, so there is nothing to
                          "restore"; only the real fetch remains (Sprint 11.5). */}
                      <button
                        type="button"
                        disabled={fetching}
                        onClick={fetchModels}
                      >
                        {fetching ? "Fetching models…" : "Fetch available models"}
                      </button>
                    </div>
                  </div>

                  <div className="merlink-model-list">
                    {form.models.map((model, index) => (
                      <div key={index} className="merlink-model-row">
                        <div className="merlink-model-row-top">
                          <div className="merlink-model-inputs">
                            <label className="merlink-settings-field">
                              <span>Model ID</span>
                              <input
                                value={model.id}
                                placeholder="deepseek-v4"
                                onChange={(e) =>
                                  updateModel(index, { id: e.target.value })
                                }
                              />
                            </label>
                            <label className="merlink-settings-field">
                              <span>Display Name</span>
                              <input
                                value={model.name}
                                placeholder="DeepSeek V4"
                                onChange={(e) =>
                                  updateModel(index, { name: e.target.value })
                                }
                              />
                            </label>
                          </div>
                          <div className="merlink-model-row-actions">
                            <button
                              type="button"
                              aria-label={`${
                                expandedModel === index ? "Collapse" : "Expand"
                              } model ${model.id || index + 1}`}
                              aria-expanded={expandedModel === index}
                              onClick={() =>
                                setExpandedModel(
                                  expandedModel === index ? -1 : index,
                                )
                              }
                            >
                              {expandedModel === index ? (
                                <ChevronUp size={17} />
                              ) : (
                                <ChevronDown size={17} />
                              )}
                            </button>
                            <button
                              type="button"
                              aria-label={`Delete model ${model.id || index + 1}`}
                              onClick={() => {
                                setForm((prev) => ({
                                  ...prev,
                                  models: prev.models.filter(
                                    (_, pos) => pos !== index,
                                  ),
                                }));
                                setExpandedModel(-1);
                              }}
                            >
                              <Trash2 size={17} />
                            </button>
                          </div>
                        </div>

                        {expandedModel === index && (
                          <div className="merlink-model-expanded">
                            <div className="merlink-provider-custom-fields">
                              <label className="merlink-settings-field">
                                <span>Context window</span>
                                <input
                                  value={model.context_window}
                                  placeholder="128K"
                                  onChange={(e) =>
                                    updateModel(index, {
                                      context_window: e.target.value,
                                    })
                                  }
                                />
                              </label>
                              <label className="merlink-settings-field">
                                <span>Max output tokens</span>
                                <input
                                  value={model.max_tokens}
                                  placeholder="4K"
                                  onChange={(e) =>
                                    updateModel(index, {
                                      max_tokens: e.target.value,
                                    })
                                  }
                                />
                              </label>
                            </div>
                            <div className="merlink-model-checks">
                              <strong>Input types</strong>
                              <label>
                                <input
                                  type="checkbox"
                                  checked={Boolean(model.input_types.text)}
                                  onChange={(e) =>
                                    updateModel(index, {
                                      input_types: {
                                        ...model.input_types,
                                        text: e.target.checked,
                                      },
                                    })
                                  }
                                />
                                <span>Text</span>
                              </label>
                              <label>
                                <input
                                  type="checkbox"
                                  checked={Boolean(model.input_types.image)}
                                  onChange={(e) =>
                                    updateModel(index, {
                                      input_types: {
                                        ...model.input_types,
                                        image: e.target.checked,
                                      },
                                    })
                                  }
                                />
                                <span>Image</span>
                              </label>
                            </div>
                            <label className="merlink-model-switch">
                              <input
                                type="checkbox"
                                role="switch"
                                checked={Boolean(model.reasoning_level)}
                                onChange={(e) =>
                                  updateModel(index, {
                                    reasoning_level: e.target.checked,
                                  })
                                }
                              />
                              <span>Reasoning levels</span>
                            </label>
                          </div>
                        )}
                      </div>
                    ))}
                  </div>

                  <button
                    type="button"
                    className="merlink-model-add"
                    onClick={() => {
                      setForm((prev) => ({
                        ...prev,
                        models: [
                          ...prev.models,
                          createModelConfig({ id: "", name: "" }),
                        ],
                      }));
                      setExpandedModel(form.models.length);
                    }}
                  >
                    <Plus size={16} />
                    <span>Add model</span>
                  </button>

                  {/* D5: the fetch result belongs on screen, not in the console. */}
                  {fetching && (
                    <p className="merlink-model-fetch-status" role="status">
                      Đang tải danh sách model từ provider…
                    </p>
                  )}
                  {!fetching && fetchError && (
                    <p className="merlink-model-fetch-error" role="alert">
                      Không tải được danh sách model: {fetchError}
                    </p>
                  )}
                  {!fetching && !fetchError && fetchNotice && (
                    <p className="merlink-model-fetch-status" role="status">
                      {fetchNotice}
                    </p>
                  )}
                  {!validModels && (
                    <p className="merlink-model-fetch-status">
                      Add at least one model with a unique ID and display name.
                    </p>
                  )}
                </section>
              </details>

              <footer className="merlink-provider-editor-actions">
                {editor.id !== null &&
                  providers.some(
                    (item) => item.id === editor.id && item.api_key,
                  ) && (
                    <button
                      type="button"
                      className="btn"
                      disabled={saving}
                      onClick={handleClearKey}
                    >
                      Clear API key
                    </button>
                  )}
                <button type="button" className="btn" onClick={closeEditor}>
                  Cancel
                </button>
                <button
                  type="button"
                  className="btn btn-primary"
                  disabled={!valid || saving}
                  onClick={handleSave}
                >
                  Apply
                </button>
              </footer>
            </div>
          </section>
        </div>
      )}
    </div>
  );
}
