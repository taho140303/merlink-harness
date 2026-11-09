import { createContext, useContext } from 'react';
import { invoke } from '@tauri-apps/api/core';
import type * as T from './types';

export interface SystemPromptCatalog {
  motifs: T.Motif[];
  genres: T.Genre[];
  motifApplicableGenres: T.MotifApplicableGenre[];
  components: T.Component[];
  states: T.StateItem[];
  stateRules: T.StateRule[];
  stateDownByRules: T.StateDownByRule[];
  emotionalRanges: T.EmotionalRange[];
  eventBlocks: T.EventBlock[];
  eventTransitions: T.EventTransition[];
  constraints: T.Constraint[];
  motifConstraints: T.MotifConstraint[];
  eventBlockEmotionals: T.EventBlockEmotional[];
  stylePresets: T.StylePreset[];
  rhetoricalDevices: T.RhetoricalDevice[];
  stylePresetRhetoricalDevices: T.StylePresetRhetoricalDevice[];
  stylePresetVocabItems: T.StylePresetVocabItem[];
  stylePresetFocusItems: T.StylePresetFocusItem[];
  narratorConfigs: T.NarratorConfig[];
  narrativeDirectives: T.NarrativeDirective[];
  stylePresetNarratives: T.StylePresetNarrative[];
  themes: T.Theme[];
  themeArguments: T.ThemeArgument[];
}

const catalogKeys = [
  'motifs', 'genres', 'motifApplicableGenres', 'components', 'states',
  'stateRules', 'stateDownByRules', 'emotionalRanges', 'eventBlocks',
  'eventTransitions', 'constraints', 'motifConstraints', 'eventBlockEmotionals',
  'stylePresets', 'rhetoricalDevices', 'stylePresetRhetoricalDevices',
  'stylePresetVocabItems', 'stylePresetFocusItems', 'narratorConfigs',
  'narrativeDirectives', 'stylePresetNarratives', 'themes', 'themeArguments',
] as const satisfies readonly (keyof SystemPromptCatalog)[];

export const emptyCatalog: SystemPromptCatalog = Object.fromEntries(
  catalogKeys.map((key) => [key, []]),
) as unknown as SystemPromptCatalog;

const CatalogContext = createContext<SystemPromptCatalog>(emptyCatalog);
export const SystemPromptCatalogProvider = CatalogContext.Provider;
export function useSystemPromptCatalog() { return useContext(CatalogContext); }

function camelCase(key: string): string {
  return key.replace(/_([a-z])/g, (_, letter: string) => letter.toUpperCase());
}

function normalizeRow(row: Record<string, unknown>): Record<string, unknown> {
  const normalized: Record<string, unknown> = {};
  for (const [key, value] of Object.entries(row)) {
    const outputKey = key === 'display_name' ? 'name' : camelCase(key);
    normalized[outputKey] = ['is_system', 'is_allowed', 'is_reliable', 'breaks_fourth_wall'].includes(key)
      ? Boolean(value)
      : value;
  }
  return normalized;
}

export async function loadSystemPromptCatalog(): Promise<SystemPromptCatalog> {
  const raw = await invoke<Record<string, Record<string, unknown>[]>>('get_system_prompt_catalog');
  const result: Record<string, unknown> = {};
  for (const key of catalogKeys) {
    if (!Array.isArray(raw[key])) throw new Error(`Catalog response is missing ${key}`);
    result[key] = raw[key].map(normalizeRow);
  }
  // Theme scope is a display-only projection derived from its arc association.
  const catalog = result as unknown as SystemPromptCatalog;
  catalog.themes = catalog.themes.map((theme) => ({
    ...theme,
    scope: theme.arcId ? 'ARC' : 'PROJECT',
  }));
  return catalog;
}
