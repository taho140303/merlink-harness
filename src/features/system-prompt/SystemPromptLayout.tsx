import { useEffect, useState } from 'react';
import { ChevronRight, Lock } from 'lucide-react';
import { cn } from '@/shared/utils/cn';
import { Tabs } from '@/shared/components/ui/tabs';
import { truncate } from '@/shared/utils/string';
import { useT } from '@/shared/i18n/useT';
import type { TKey } from '@/shared/i18n/dict';
import { MotifsTab } from './tabs/MotifsTab';
import { StylePresetsTab } from './tabs/StylePresetsTab';
import { NarrativeDirectivesTab } from './tabs/NarrativeDirectivesTab';
import { ThemesTab } from './tabs/ThemesTab';
import { GenresTab } from './tabs/GenresTab';
import { emptyCatalog, loadSystemPromptCatalog, SystemPromptCatalogProvider } from './catalog';

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
  const [catalog, setCatalog] = useState(emptyCatalog);
  const [catalogError, setCatalogError] = useState<string | null>(null);
  const [catalogLoading, setCatalogLoading] = useState(true);
  const [refreshToken, setRefreshToken] = useState(0);

  useEffect(() => {
    let cancelled = false;
    loadSystemPromptCatalog().then((data) => {
      if (cancelled) return;
      setCatalog(data);
      setCatalogError(null);
    }).catch((error: unknown) => {
      if (!cancelled) setCatalogError(error instanceof Error ? error.message : String(error));
    }).finally(() => {
      if (!cancelled) setCatalogLoading(false);
    });
    return () => { cancelled = true; };
  }, [tab, refreshToken]);

  const handleTabChange = (newTab: SpTab) => {
    if (newTab === tab) setRefreshToken((n) => n + 1);
    else setTab(newTab);
    setActiveChild(null);
    setCatalogLoading(true);
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
        <button type="button" onClick={() => { setCatalogLoading(true); setRefreshToken((n) => n + 1); }} className="rounded-lg border border-line px-3 py-1.5 text-xs text-fg-muted hover:text-fg">Tải lại</button>
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
      <SystemPromptCatalogProvider value={catalog}>
      <div className="mt-6">
        {catalogLoading && <p role="status" className="mb-4 text-sm text-fg-muted">Đang tải dữ liệu tham chiếu…</p>}
        {catalogError && <p role="alert" className="mb-4 text-sm text-red-600">Không tải được dữ liệu tham chiếu: {catalogError}</p>}
        {tab === 'motifs' && (
          <MotifsTab activeChild={activeChild} onSelectChild={setActiveChild} />
        )}
        {tab === 'stylePresets' && <StylePresetsTab />}
        {tab === 'narrativeDirectives' && <NarrativeDirectivesTab />}
        {tab === 'themes' && <ThemesTab />}
        {tab === 'genres' && <GenresTab />}
      </div>
      </SystemPromptCatalogProvider>
    </div>
  );
}
