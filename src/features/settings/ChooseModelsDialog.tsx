import { useEffect, useState } from "react";
import * as DialogPrimitive from "@radix-ui/react-dialog";
import { X } from "lucide-react";
import { cn } from "@/shared/utils/cn";
import { Input } from "@/shared/components/ui/input";
import { Button } from "@/shared/components/ui/button";
import {
  Dialog,
  DialogPortal,
  DialogOverlay,
  DialogTitle,
  DialogDescription,
  DialogClose,
} from "@/shared/components/ui/dialog";
import { useT } from "@/shared/i18n/useT";
import type { BackendClient } from "@/shared/backend/client";

export const AVAILABLE_PROVIDER_MODELS = [
  "claude-opus-4-6",
  "claude-opus-4-6-thinking",
  "claude-opus-5-5",
  "claude-opus-5-5-*",
  "claude-opus-5-5-high",
  "claude-opus-5-5-low",
  "claude-opus-5-5-medium",
  "claude-sonnet-4-6",
  "claude-sonnet-5-5",
  "claude-sonnet-5-5-*",
];

export interface ChooseModelsDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  onAddModels: (modelIds: string[]) => void;
  availableModels?: readonly string[];
  initialSelectedIds?: readonly string[];
  client?: BackendClient;
  providerId?: string;
  /** Current form values; when set, models are fetched live from the endpoint. */
  fetchParams?: {
    providerId: string;
    endpointUrl?: string | null;
    protocol?: string;
    apiKey?: string;
  };
}

export function ChooseModelsDialog({
  open,
  onOpenChange,
  onAddModels,
  availableModels = AVAILABLE_PROVIDER_MODELS,
  initialSelectedIds = [],
  client,
  providerId,
  fetchParams,
}: ChooseModelsDialogProps) {
  const t = useT();
  const [modelSearch, setModelSearch] = useState("");
  const [selectedModelIds, setSelectedModelIds] = useState<string[]>([]);
  const [modelsList, setModelsList] = useState<readonly string[]>(availableModels);
  const [fetching, setFetching] = useState(false);
  const [fetchError, setFetchError] = useState<string | null>(null);

  useEffect(() => {
    if (open) {
      // eslint-disable-next-line react-hooks/set-state-in-effect
      setModelSearch("");
      setSelectedModelIds([...initialSelectedIds]);

      setFetchError(null);

      // Live discovery from the endpoint currently in the form (DeepSeek-Harness style)
      if (client && client.status === "available" && fetchParams?.endpointUrl) {
        let cancelled = false;
        setFetching(true);
        client.transport
          .invoke("fetch_provider_models" as any, fetchParams)
          .then((res: any) => {
            if (cancelled) return;
            if (res?.ok && Array.isArray(res.models) && res.models.length > 0) {
              setModelsList(res.models);
            } else {
              setModelsList([]);
              setFetchError(
                res?.error || "Endpoint returned no models. Add model ids manually."
              );
            }
          })
          .catch((err: any) => {
            if (cancelled) return;
            setModelsList([]);
            setFetchError(String(err?.message || err));
          })
          .finally(() => {
            if (!cancelled) setFetching(false);
          });
        return () => {
          cancelled = true;
        };
      }

      // If live backend client is provided, query runtime provider models
      if (client && client.status === "available") {
        client
          .getRuntimeAndProviderConfig()
          .then((cfg) => {
            const target = providerId
              ? cfg.providers.find((p) => p.providerId === providerId)
              : cfg.providers.find((p) => p.isActive);
            if (target?.modelName) {
              const unique = Array.from(new Set([target.modelName, ...availableModels]));
              setModelsList(unique);
            }
          })
          .catch(() => {
            // Keep default available models if backend query fails
            setModelsList(availableModels);
          });
      } else {
        setModelsList(availableModels);
      }
    }
  }, [open, initialSelectedIds, availableModels, client, providerId, fetchParams?.endpointUrl, fetchParams?.protocol, fetchParams?.providerId]);

  const filteredAvailableModels = modelsList.filter((modelId) =>
    modelId.toLowerCase().includes(modelSearch.trim().toLowerCase())
  );

  const allFilteredSelected =
    filteredAvailableModels.length > 0 &&
    filteredAvailableModels.every((id) => selectedModelIds.includes(id));

  const toggleModelSelection = (modelId: string) => {
    setSelectedModelIds((prev) =>
      prev.includes(modelId)
        ? prev.filter((id) => id !== modelId)
        : [...prev, modelId]
    );
  };

  const handleToggleSelectAllVisible = () => {
    if (allFilteredSelected) {
      setSelectedModelIds((prev) =>
        prev.filter((id) => !filteredAvailableModels.includes(id))
      );
    } else {
      setSelectedModelIds((prev) => {
        const next = new Set(prev);
        for (const id of filteredAvailableModels) {
          next.add(id);
        }
        return Array.from(next);
      });
    }
  };

  const handleAddSelected = () => {
    if (selectedModelIds.length === 0) return;
    onAddModels(selectedModelIds);
    onOpenChange(false);
  };

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogPortal>
        <DialogOverlay />
        <DialogPrimitive.Content className="dialog-content tier-overlay fixed left-1/2 top-1/2 z-50 flex w-full max-w-md flex-col gap-3.5 rounded-panel p-5 text-fg focus:outline-none">
          {/* Block 1: Header row (Title left, Close X button right) */}
          <div className="flex items-center justify-between gap-3">
            <DialogTitle className="text-base font-semibold leading-snug tracking-tight text-fg">
              {t("settings.provider.fetchDialog.title")}
            </DialogTitle>
            <DialogClose
              aria-label={t("common.close")}
              className="focus-ring inline-flex h-7 w-7 shrink-0 items-center justify-center rounded-lg text-fg-muted transition-colors hover:bg-surface-muted hover:text-fg"
            >
              <X className="h-4 w-4" aria-hidden="true" />
            </DialogClose>
          </div>

          {/* Block 2: Subtitle */}
          <DialogDescription className="text-xs leading-relaxed text-fg-muted sm:text-sm">
            {t("settings.provider.fetchDialog.subtitle")}
          </DialogDescription>

          {/* Block 3: Realtime search input + borderless/backgroundless Select all / Deselect all button */}
          <div className="flex items-center justify-between gap-3">
            <div className="flex-1">
              <Input
                type="text"
                value={modelSearch}
                onChange={(e) => setModelSearch(e.target.value)}
                placeholder={t("settings.provider.fetchDialog.searchPlaceholder")}
                aria-label={t("settings.provider.fetchDialog.searchPlaceholder")}
                className="h-8 text-xs"
              />
            </div>
            <button
              type="button"
              onClick={handleToggleSelectAllVisible}
              disabled={filteredAvailableModels.length === 0}
              className="focus-ring shrink-0 rounded bg-transparent px-2 py-1 text-xs font-medium text-fg transition-colors hover:text-fg-muted disabled:pointer-events-none disabled:opacity-50"
            >
              {allFilteredSelected
                ? t("settings.provider.fetchDialog.deselectAll")
                : t("settings.provider.fetchDialog.selectAll")}
            </button>
          </div>

          {/* Block 4: Vertically scrollable container of model items */}
          <div
            role="group"
            aria-label={t("settings.provider.fetchDialog.title")}
            className="max-h-64 space-y-1 overflow-y-auto pr-1"
          >
            {filteredAvailableModels.length === 0 ? (
              <div className="py-8 text-center text-xs text-fg-muted">
                {fetching ? "…" : fetchError ?? t("settings.provider.fetchDialog.empty")}
              </div>
            ) : (
              filteredAvailableModels.map((modelId) => {
                const isChecked = selectedModelIds.includes(modelId);
                return (
                  <label
                    key={modelId}
                    className={cn(
                      "flex cursor-pointer items-center justify-start gap-2.5 rounded-lg px-2.5 py-2 transition-colors hover:bg-surface-muted/60",
                      isChecked && "bg-surface-muted/40"
                    )}
                  >
                    <input
                      type="checkbox"
                      checked={isChecked}
                      onChange={() => toggleModelSelection(modelId)}
                      className="tier-input focus-ring h-4 w-4 shrink-0 rounded accent-accent"
                    />
                    <span className="truncate font-mono text-xs text-fg">
                      {modelId}
                    </span>
                  </label>
                );
              })
            )}
          </div>

          {/* Block 5: Footer with 2 right-aligned buttons (Cancel + Add selected) */}
          <div className="flex items-center justify-end gap-2 pt-2">
            <Button
              type="button"
              variant="secondary"
              size="sm"
              onClick={() => onOpenChange(false)}
            >
              {t("settings.provider.cancel")}
            </Button>
            <Button
              type="button"
              variant="primary"
              size="sm"
              disabled={selectedModelIds.length === 0}
              onClick={handleAddSelected}
            >
              {t("settings.provider.fetchDialog.addSelected")}
            </Button>
          </div>
        </DialogPrimitive.Content>
      </DialogPortal>
    </Dialog>
  );
}
