import { useState } from 'react';
import { Lock } from 'lucide-react';
import { cn } from '../../../shared/utils/cn';
import { useT } from '../../../shared/i18n/useT';
import type { TKey } from '../../../shared/i18n/dict';
import { MotifsTab } from './tabs/MotifsTab';
import { StylePresetsTab } from './tabs/StylePresetsTab';
import { NarrativeDirectivesTab } from './tabs/NarrativeDirectivesTab';
import { ThemesTab } from './tabs/ThemesTab';
import { GenresTab } from './tabs/GenresTab';

type SpTab = 'motifs' | 'stylePresets' | 'narrativeDirectives' | 'themes' | 'genres';

const TAB_LABELS: Record<SpTab, TKey> = {
  motifs: 'sp.tab.motifs',
  stylePresets: 'sp.tab.stylePresets',
  narrativeDirectives: 'sp.tab.narrativeDirectives',
  themes: 'sp.tab.themes',
  genres: 'sp.tab.genres',
};

export function SystemPromptLayout() {
  const t = useT();
  const [tab, setTab] = useState<SpTab>('motifs');

  return (
    <div className="flex h-full flex-col overflow-y-auto bg-slate-50/50 p-6 dark:bg-slate-950/20 sm:p-8">
      <div className="flex items-start justify-between gap-3 border-b border-slate-200 pb-4 dark:border-slate-800">
        <div>
          <div className="flex items-center gap-2">
            <h1 className="text-xl font-bold tracking-tight text-slate-900 dark:text-slate-100 sm:text-2xl">
              {t('sp.title')}
            </h1>
            <span className="flex items-center gap-1 rounded-full bg-slate-100 px-2 py-0.5 text-[10px] font-medium text-slate-600 dark:bg-slate-800 dark:text-slate-300">
              <Lock className="h-2.5 w-2.5" />
              {t('sp.readonly')}
            </span>
          </div>
          <p className="mt-1 text-xs text-slate-500 dark:text-slate-400">{t('sp.subtitle')}</p>
        </div>
      </div>

      <div className="mt-6 flex flex-wrap gap-2 border-b border-slate-200 pb-3 dark:border-slate-800">
        {(Object.keys(TAB_LABELS) as SpTab[]).map((key) => (
          <button
            key={key}
            type="button"
            onClick={() => setTab(key)}
            className={cn(
              'rounded-xl px-3.5 py-1.5 text-xs font-semibold transition-all',
              tab === key
                ? 'bg-slate-900 text-white shadow-xs dark:bg-slate-100 dark:text-slate-900'
                : 'bg-white text-slate-600 hover:bg-slate-100 dark:bg-slate-900 dark:text-slate-400 dark:hover:bg-slate-800',
            )}
          >
            {t(TAB_LABELS[key])}
          </button>
        ))}
      </div>

      <div className="mt-6">
        {tab === 'motifs' && <MotifsTab />}
        {tab === 'stylePresets' && <StylePresetsTab />}
        {tab === 'narrativeDirectives' && <NarrativeDirectivesTab />}
        {tab === 'themes' && <ThemesTab />}
        {tab === 'genres' && <GenresTab />}
      </div>
    </div>
  );
}
