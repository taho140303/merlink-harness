import type { RhetoricalUsage, StylePresetRhetoricalDevice } from './types';
import type { TKey } from '@/shared/i18n/dict';

const RHETORICAL_ORDER: Record<RhetoricalUsage, number> = { primary: 0, secondary: 1, avoid: 2 };

export function sortRhetoricalDevices(items: StylePresetRhetoricalDevice[]): StylePresetRhetoricalDevice[] {
  return [...items].sort((a, b) => RHETORICAL_ORDER[a.usageType] - RHETORICAL_ORDER[b.usageType]);
}

export const STYLE_PRESET_CATEGORY_KEYS: Record<string, TKey> = {
  action: 'sp.style.category.action',
  exploration_discovery: 'sp.style.category.exploration_discovery',
  fantasy_scifi_mechanics: 'sp.style.category.fantasy_scifi_mechanics',
  mystery: 'sp.style.category.mystery',
  political_social: 'sp.style.category.political_social',
  psychological_drama: 'sp.style.category.psychological_drama',
  romance_relationship: 'sp.style.category.romance_relationship',
  slice_of_life_comedy: 'sp.style.category.slice_of_life_comedy',
  transition: 'sp.style.category.transition',
};
