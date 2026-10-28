import { useState } from 'react';
import { ChevronRight, Lock } from 'lucide-react';
import { cn } from '../../../shared/utils/cn';
import { Tabs } from '../../../shared/components/ui/tabs';
import { truncate } from '../../../shared/utils/string';
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
  const [activeChild, setActiveChild] = useState<{ id: string; name: string } | null>(null);

  const handleTabChange = (newTab: SpTab) => {
    setTab(newTab);
    setActiveChild(null);
  };

  return (
    <div className="flex h-full flex-col overflow-y-auto bg-surface-muted p-6  sm:p-8">
      {/* Header: Dynamic Breadcrumb as Title */}
      <div className="flex items-start justify-between gap-3 border-b border-line pb-4 ">
        <div>
          <div className="flex items-center gap-1.5 text-xl font-bold tracking-tight text-fg sm:text-2xl">
            <button
              type="button"
              onClick={() => setActiveChild(null)}
              className={cn(
                'transition-colors hover:underline',
                tab === 'motifs' && !activeChild
                  ? 'text-fg'
                  : 'text-fg-muted hover:text-fg -subtle ',
              )}
            >
              {t('sp.title')}
            </button>
            <ChevronRight className="h-5 w-5 shrink-0 text-fg-subtle -muted" />
            <button
              type="button"
              onClick={() => setActiveChild(null)}
              className={cn(
                'transition-colors hover:underline',
                activeChild
                  ? 'text-fg-muted hover:text-fg -subtle '
                  : 'text-fg',
              )}
            >
              {t(TAB_LABELS[tab])}
            </button>
            {activeChild && (
              <>
                <ChevronRight className="h-5 w-5 shrink-0 text-fg-subtle -muted" />
                <span
                  className="max-w-[200px] truncate text-fg"
                  title={activeChild.name}
                >
                  {truncate(activeChild.name, 15)}
                </span>
              </>
            )}
            <span className="ml-2 flex shrink-0 items-center gap-1 rounded-full bg-surface-muted px-2 py-0.5 text-xs font-medium text-fg-muted  ">
              <Lock className="h-2.5 w-2.5" />
              {t('sp.readonly')}
            </span>
          </div>
          <p className="mt-1 text-xs text-fg-muted">{t('sp.subtitle')}</p>
        </div>
      </div>

      {/* Tabs */}
      <Tabs
        className="mt-6 flex-wrap"
        items={(Object.keys(TAB_LABELS) as SpTab[]).map((key) => ({ id: key, label: t(TAB_LABELS[key]) }))}
        value={tab}
        onChange={(key) => handleTabChange(key as SpTab)}
        ariaLabel={t('sp.title')}
        variant="underline"
      />

      {/* Content */}
      <div className="mt-6">
        {tab === 'motifs' && (
          <MotifsTab activeChild={activeChild} onSelectChild={setActiveChild} />
        )}
        {tab === 'stylePresets' && <StylePresetsTab />}
        {tab === 'narrativeDirectives' && <NarrativeDirectivesTab />}
        {tab === 'themes' && <ThemesTab />}
        {tab === 'genres' && <GenresTab />}
      </div>
    </div>
  );
}
