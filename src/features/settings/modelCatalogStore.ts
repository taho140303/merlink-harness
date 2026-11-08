import { create } from "zustand";

/**
 * Client-side catalog of configured providers/models shared by Settings and the chat
 * model selector. Never stores secrets (no API keys), only ids and display names.
 */
const STORAGE_KEY = "merlink.modelCatalog";

export interface CatalogModel {
  id: string;
  name: string;
}

export interface CatalogProvider {
  id: string;
  name: string;
  models: CatalogModel[];
}

export interface SelectedModel {
  providerId: string;
  modelId: string;
}

interface Persisted {
  providers: CatalogProvider[];
  selected: SelectedModel | null;
}

interface ModelCatalogState extends Persisted {
  hydrated: boolean;
  hydrate: () => void;
  upsertProvider: (provider: CatalogProvider) => void;
  removeProvider: (providerId: string) => void;
  select: (selected: SelectedModel) => void;
}

function persist(state: Persisted) {
  if (typeof window === "undefined") return;
  try {
    window.localStorage.setItem(
      STORAGE_KEY,
      JSON.stringify({ providers: state.providers, selected: state.selected })
    );
  } catch {
    /* storage unavailable: keep in-memory only */
  }
}

export const useModelCatalogStore = create<ModelCatalogState>((set, get) => ({
  providers: [],
  selected: null,
  hydrated: false,
  hydrate: () => {
    if (get().hydrated || typeof window === "undefined") return;
    try {
      const raw = window.localStorage.getItem(STORAGE_KEY);
      if (raw) {
        const parsed = JSON.parse(raw) as Partial<Persisted>;
        set({
          providers: Array.isArray(parsed.providers) ? parsed.providers : [],
          selected: parsed.selected ?? null,
        });
      }
    } catch {
      /* ignore corrupt data */
    }
    set({ hydrated: true });
  },
  upsertProvider: (provider) => {
    const providers = get().providers.some((p) => p.id === provider.id)
      ? get().providers.map((p) => (p.id === provider.id ? provider : p))
      : [...get().providers, provider];
    let selected = get().selected;
    if (!selected && provider.models[0]) {
      selected = { providerId: provider.id, modelId: provider.models[0].id };
    }
    set({ providers, selected });
    persist({ providers, selected });
  },
  removeProvider: (providerId) => {
    const providers = get().providers.filter((p) => p.id !== providerId);
    const cur = get().selected;
    const selected = cur && cur.providerId === providerId ? null : cur;
    set({ providers, selected });
    persist({ providers, selected });
  },
  select: (selected) => {
    set({ selected });
    persist({ providers: get().providers, selected });
  },
}));
