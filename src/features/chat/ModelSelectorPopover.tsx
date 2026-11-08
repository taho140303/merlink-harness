import { useEffect, useMemo, useState } from "react";
import { ChevronDown, Check } from "lucide-react";
import { cn } from "@/shared/utils/cn";
import { Popover } from "@/shared/components/ui/popover";
import { useT } from "@/shared/i18n/useT";
import { useModelCatalogStore } from "@/features/settings/modelCatalogStore";
import { backendClient } from "@/shared/backend/client";

interface ModelOption {
  id: string;
  name: string;
  provider: string;
}

export function ModelSelectorPopover() {
  const t = useT();
  const catalog = useModelCatalogStore((s) => s.providers);
  const selectedRef = useModelCatalogStore((s) => s.selected);
  const select = useModelCatalogStore((s) => s.select);
  const hydrate = useModelCatalogStore((s) => s.hydrate);

  useEffect(() => {
    hydrate();

    // Synchronize catalog with live backend providers to remove stale test leftovers
    if (backendClient && backendClient.status !== "unavailable") {
      backendClient
        .getRuntimeAndProviderConfig({})
        .then((res) => {
          if (!res || !Array.isArray(res.providers)) return;
          const liveIds = new Set(res.providers.map((p) => p.providerId));
          const currentStore = useModelCatalogStore.getState();
          const cleanProviders = currentStore.providers.filter((p) => liveIds.has(p.id));
          if (cleanProviders.length !== currentStore.providers.length) {
            useModelCatalogStore.setState({ providers: cleanProviders });
            try {
              if (typeof window !== "undefined") {
                window.localStorage.setItem(
                  "merlink.modelCatalog",
                  JSON.stringify({
                    providers: cleanProviders,
                    selected: currentStore.selected,
                  })
                );
              }
            } catch {
              /* ignore storage write failures */
            }
          }
        })
        .catch(() => {});
    }
  }, [hydrate]);

  // Only show configured providers with non-empty model IDs
  const groups = useMemo<Record<string, ModelOption[]>>(() => {
    const validProviders = catalog
      .map((p) => ({
        ...p,
        models: (p.models ?? []).filter(
          (m) => m && m.id && m.id.trim().length > 0
        ),
      }))
      .filter((p) => p.models.length > 0);

    return Object.fromEntries(
      validProviders.map((p) => [
        p.name || p.id,
        p.models.map((m) => ({
          id: `${p.id}::${m.id}`,
          name: m.name?.trim() || m.id,
          provider: p.name || p.id,
        })),
      ])
    );
  }, [catalog]);

  const allOptions = Object.values(groups).flat();
  const selectedId = selectedRef
    ? `${selectedRef.providerId}::${selectedRef.modelId}`
    : allOptions[0]?.id;
  const selectedModel =
    allOptions.find((o) => o.id === selectedId) ?? allOptions[0] ?? null;

  const handlePick = async (m: ModelOption) => {
    const [providerId, ...rest] = m.id.split("::");
    const modelId = rest.join("::");
    if (providerId && modelId) {
      select({ providerId, modelId });
      try {
        await backendClient.updateRuntimeAndProviderConfig({
          activeProviderId: providerId,
          modelName: modelId,
        });
      } catch (err) {
        console.error("Failed to sync active provider with backend:", err);
      }
    }
  };

  if (!selectedModel || allOptions.length === 0) {
    return (
      <Popover
        label={t("chat.selectModel")}
        align="end"
        trigger={
          <span className="flex items-center gap-1.5 rounded-xl tier-input px-2.5 py-1.5 text-xs font-medium text-fg-muted hover:bg-surface-muted transition-colors">
            <span className="max-w-[110px] truncate">{t("chat.selectModel")}</span>
            <ChevronDown aria-hidden="true" className="h-3 w-3 text-fg-subtle" />
          </span>
        }
        triggerClassName="rounded-xl"
        contentClassName="no-scrollbar max-h-72 w-60 overflow-y-auto p-3 text-center"
      >
        <p className="text-xs font-medium text-fg">{"Chưa có mô hình nào"}</p>
        <p className="mt-1 text-[11px] text-fg-subtle leading-relaxed">
          Vào mục Cài đặt &gt; AI Providers để thêm model của bạn.
        </p>
      </Popover>
    );
  }

  return (
    <Popover
      label={t("chat.selectModel")}
      align="end"
      trigger={
        <span className="flex items-center gap-1.5 rounded-xl tier-input px-2.5 py-1.5 text-xs font-medium text-fg hover:bg-surface-muted transition-colors">
          <span className="max-w-[110px] truncate">{selectedModel.name}</span>
          <ChevronDown aria-hidden="true" className="h-3 w-3 text-fg-subtle" />
        </span>
      }
      triggerClassName="rounded-xl"
      contentClassName="no-scrollbar max-h-72 w-60 overflow-y-auto p-1.5"
    >
      {Object.entries(groups).map(([provider, models]) => (
        <div key={provider} className="py-1">
          <div className="px-2.5 py-1 text-xs font-semibold uppercase tracking-wider text-fg-subtle">
            {provider}
          </div>
          <div className="space-y-0.5">
            {models.map((m) => {
              const isSelected = selectedModel.id === m.id;
              return (
                <button
                  key={m.id}
                  type="button"
                  onClick={() => handlePick(m)}
                  className={cn(
                    "focus-ring flex w-full items-center justify-between rounded-lg px-2.5 py-1.5 text-xs transition-colors text-left",
                    isSelected
                      ? "bg-surface-muted font-medium text-fg"
                      : "text-fg-muted hover:bg-surface-muted hover:text-fg"
                  )}
                >
                  <span className="truncate">{m.name}</span>
                  {isSelected && <Check aria-hidden="true" className="h-3.5 w-3.5 shrink-0 text-accent" />}
                </button>
              );
            })}
          </div>
        </div>
      ))}
    </Popover>
  );
}
