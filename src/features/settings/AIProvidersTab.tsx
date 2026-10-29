import { useState } from "react";
import { Plus } from "lucide-react";
import { ProviderForm } from "./ProviderForm";
import { DeleteConfirmDialog } from "@/shared/components/ui/delete-confirm-dialog";
import { toast } from "@/shared/components/ui/toast";
import { useT } from "@/shared/i18n/useT";
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
  const t = useT();
  const [providers, setProviders] = useState<AIProviderItem[]>(INITIAL_PROVIDERS);
  const [editingProvider, setEditingProvider] = useState<AIProviderItem | null>(null);
  const [isAdding, setIsAdding] = useState(false);
  const [deleteTarget, setDeleteTarget] = useState<AIProviderItem | null>(null);
  const [deleteOpen, setDeleteOpen] = useState(false);

  const handleSave = (provider: AIProviderItem) => {
    if (editingProvider) {
      setProviders(providers.map((p) => (p.id === provider.id ? provider : p)));
    } else {
      setProviders([...providers, provider]);
    }
    setIsAdding(false);
    setEditingProvider(null);
  };

  const requestDelete = (provider: AIProviderItem) => {
    setDeleteTarget(provider);
    setDeleteOpen(true);
  };

  const confirmDelete = () => {
    if (!deleteTarget) return;
    setProviders((prev) => prev.filter((p) => p.id !== deleteTarget.id));
    toast({ title: t("common.deleted"), variant: "success" });
    setDeleteTarget(null);
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
        <h3 className="text-base font-semibold text-fg">
          {t("providers.models")}
        </h3>
        <p className="text-xs text-fg-muted">
          {t("providers.subtitle")}
        </p>
      </div>

      <div className="space-y-2">
        {providers.map((provider) => (
          <div
            key={provider.id}
            className="flex items-center justify-between rounded-2xl border border-line bg-surface-card px-4 py-2.5 transition-colors  "
          >
            <div className="flex items-center gap-2.5 truncate">
              <span className="truncate text-xs font-medium text-fg">
                {provider.name}
              </span>
              {provider.isCustom && (
                <span className="rounded-md bg-surface-muted px-1.5 py-0.5 text-xs font-medium text-fg-muted  ">
                  {t("providers.custom")}
                </span>
              )}
              <span className="inline-block h-2 w-2 shrink-0 rounded-full bg-emerald-500" />
              {provider.status === "signed-in" && (
                <span className="text-xs font-normal text-emerald-600 dark:text-emerald-400">
                  {t("providers.signedIn")}
                </span>
              )}
            </div>

            <div className="flex shrink-0 items-center gap-1">
              <button
                type="button"
                onClick={() => setEditingProvider(provider)}
                className="rounded-lg px-2.5 py-1 text-xs font-medium text-fg-muted hover:bg-surface-muted hover:text-fg   "
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
          className="flex w-full items-center justify-center gap-2 rounded-2xl border border-dashed border-line-strong py-3 text-xs font-medium text-fg-muted transition-colors hover:border-line-strong hover:text-fg   "
        >
          <Plus className="h-4 w-4" />
          <span>{t("providers.add")}</span>
        </button>
      </div>

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
