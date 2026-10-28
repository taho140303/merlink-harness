import { useState } from 'react';
import { Bookmark, Tags, BookOpen, Users, Globe2 } from 'lucide-react';
import { cn } from '../../../shared/utils/cn';
import { Dialog, DialogContent, DialogTitle } from '../../../shared/components/ui/dialog';
import { useT } from '../../../shared/i18n/useT';
import { ArcCard } from './overview/ArcCard';
import { CharacterCard } from './overview/CharacterCard';
import { WorldSettingsBlock } from './overview/WorldSettingsBlock';
import type { DraftOverview, FeedbackContext, DraftStep } from './types';

const MAX_VISIBLE_GENRES = 9;

interface DraftOverviewBlockProps {
  overview: DraftOverview;
  currentStep: DraftStep;
}

export function DraftOverviewBlock({ overview, currentStep }: DraftOverviewBlockProps) {
  const t = useT();
  const [allGenresOpen, setAllGenresOpen] = useState(false);
  
  const [activeTab, setActiveTab] = useState<'arcs' | 'characters' | 'world'>('arcs');
  const [arcFilter, setArcFilter] = useState<'all' | number>('all');
  const [expandedArcs, setExpandedArcs] = useState<Set<string>>(new Set());

  const visibleGenres = overview.genres.slice(0, MAX_VISIBLE_GENRES);
  const remainingGenres = overview.genres.length - MAX_VISIBLE_GENRES;

  const targets = [
    { key: 'arcs', label: t('draft.overview.arcs'), value: overview.target.arcs, hashClass: 'text-indigo-500', valueClass: 'text-indigo-600' },
    { key: 'chapters', label: t('draft.overview.chapters'), value: overview.target.chapters, hashClass: 'text-emerald-500', valueClass: 'text-emerald-600' },
    { key: 'words', label: t('draft.overview.wordsPerChapter'), value: overview.target.wordPerChapter, hashClass: 'text-violet-500', valueClass: 'text-violet-600' },
  ];

  const toggleArc = (id: string) => {
    setExpandedArcs(prev => {
      const next = new Set(prev);
      if (next.has(id)) next.delete(id);
      else next.add(id);
      return next;
    });
  };

  const expandAll = () => setExpandedArcs(new Set(overview.arcs.map(a => a.id)));
  const collapseAll = () => setExpandedArcs(new Set());

  const filteredArcs = arcFilter === 'all' ? overview.arcs : overview.arcs.filter(a => a.number === arcFilter);


  const showWorldTab = ['world', 'scene', 'character', 'relationship'].includes(currentStep);

  return (
    <div className="no-scrollbar relative flex h-full w-full flex-col overflow-y-auto bg-slate-50/40 dark:bg-slate-950/40">
      {/* Background Orbs */}
      <div className="pointer-events-none fixed inset-0 z-0 overflow-hidden">
        <div className="absolute -top-40 -left-40 h-96 w-96 rounded-full bg-indigo-300/30 blur-[100px] dark:bg-indigo-900/20" />
        <div className="absolute top-1/3 -right-40 h-96 w-96 rounded-full bg-pink-300/30 blur-[100px] dark:bg-pink-900/20" />
        <div className="absolute -bottom-40 left-1/3 h-96 w-96 rounded-full bg-violet-300/30 blur-[100px] dark:bg-violet-900/20" />
      </div>

      <div className="relative z-10 mx-auto flex w-full max-w-5xl flex-col gap-6 p-6 pb-24">
        {/* Header */}
        <header className="rounded-3xl p-5 neu-flat">
          <div className="flex flex-col gap-3 md:flex-row md:items-center md:justify-between">
            <div className="flex min-w-0 items-center gap-3">
              <h2 className="min-w-0 text-2xl font-black leading-tight tracking-tight text-slate-900 dark:text-slate-100">
                {overview.displayName}
              </h2>
              <span className="shrink-0 rounded-xl px-2.5 py-1 font-mono text-[11px] font-bold uppercase tracking-widest text-indigo-600 neu-pressed dark:text-indigo-400">
                {overview.language}
              </span>
            </div>


          </div>

          <p className="mt-4 max-w-4xl text-sm leading-relaxed text-slate-600 dark:text-slate-300">
            {overview.summary}
          </p>

          <div className="mt-4 flex flex-col justify-between gap-4 lg:flex-row lg:flex-wrap lg:items-center">
            <div className="flex w-fit shrink-0 flex-wrap items-center gap-2 rounded-2xl p-1.5 neu-pressed">
              {targets.map((target) => (
                <div
                  key={target.key}
                  className="flex items-center gap-1.5 whitespace-nowrap rounded-xl border border-slate-200/70 bg-white/70 px-3 py-1.5 font-mono text-xs font-bold text-slate-700 shadow-2xs dark:border-slate-700/70 dark:bg-slate-900/70 dark:text-slate-200"
                >
                  <span className={cn('font-black', target.hashClass)}>#</span>
                  <span className="text-slate-400 dark:text-slate-500">{target.label}:</span>
                  <span className={cn('text-sm font-extrabold', target.valueClass)}>{target.value}</span>
                </div>
              ))}
            </div>

            <div className="flex flex-wrap items-center gap-2">
              {visibleGenres.map((genre) => (
                <span
                  key={genre.name}
                  className={cn(
                    'flex items-center gap-1.5 whitespace-nowrap rounded-xl px-3 py-1.5 text-xs',
                    genre.isPrimary
                      ? 'border border-indigo-500/30 bg-indigo-500/10 font-bold text-indigo-700 dark:text-indigo-300'
                      : 'border border-slate-200/80 bg-white/60 font-medium text-slate-600 dark:border-slate-700/80 dark:bg-slate-900/40 dark:text-slate-300'
                  )}
                >
                  {genre.isPrimary && <Bookmark className="h-3 w-3 fill-current" />}
                  <span>{genre.name}</span>
                </span>
              ))}
              {remainingGenres > 0 && (
                <button
                  type="button"
                  onClick={() => setAllGenresOpen(true)}
                  className="flex items-center gap-1.5 rounded-xl px-3 py-1.5 text-xs font-black text-indigo-600 transition-all hover:neu-pressed dark:text-indigo-400"
                >
                  <span>...</span>
                  <span className="rounded-lg px-1.5 py-0.5 font-mono text-[10px] font-bold text-slate-500 neu-pressed dark:text-slate-300">
                    +{remainingGenres}
                  </span>
                </button>
              )}
            </div>
          </div>
        </header>

        {/* Nav Bar */}
        <div className="flex items-center gap-1 rounded-xl bg-slate-100 p-1 dark:bg-slate-800 w-fit">
          <button
            onClick={() => setActiveTab('arcs')}
            className={cn(
              "flex items-center gap-2 rounded-lg px-4 py-1.5 text-xs font-semibold transition-all",
              activeTab === 'arcs' ? "bg-white text-slate-900 shadow-xs dark:bg-slate-700 dark:text-white" : "text-slate-500 hover:text-slate-900 dark:text-slate-400 dark:hover:text-slate-200"
            )}
          >
            <BookOpen className="h-3.5 w-3.5" /> {t('draft.overview.tabArcs')}
          </button>
          <button
            onClick={() => setActiveTab('characters')}
            className={cn(
              "flex items-center gap-2 rounded-lg px-4 py-1.5 text-xs font-semibold transition-all",
              activeTab === 'characters' ? "bg-white text-slate-900 shadow-xs dark:bg-slate-700 dark:text-white" : "text-slate-500 hover:text-slate-900 dark:text-slate-400 dark:hover:text-slate-200"
            )}
          >
            <Users className="h-3.5 w-3.5" /> {t('draft.overview.tabCharacters')}
            <span className="rounded-full bg-slate-200 px-1.5 py-0.5 text-[9px] text-slate-600 dark:bg-slate-600 dark:text-slate-200">{overview.characters.length}</span>
          </button>
          {showWorldTab && (
            <button
              onClick={() => setActiveTab('world')}
              className={cn(
                "flex items-center gap-2 rounded-lg px-4 py-1.5 text-xs font-semibold transition-all animate-in fade-in slide-in-from-left-2",
                activeTab === 'world' ? "bg-white text-slate-900 shadow-xs dark:bg-slate-700 dark:text-white" : "text-slate-500 hover:text-slate-900 dark:text-slate-400 dark:hover:text-slate-200"
              )}
            >
              <Globe2 className="h-3.5 w-3.5" /> {t('draft.overview.tabWorld')}
            </button>
          )}
        </div>

        {/* Panels */}
        {activeTab === 'arcs' && (
          <div className="w-full space-y-3">
            <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-3 w-full">
              <div className="flex items-center gap-2">
                <span className="text-[10px] font-semibold uppercase tracking-wider text-slate-500">{t('draft.overview.filterArc')}</span>
                <select
                  value={arcFilter}
                  onChange={(e) => setArcFilter(e.target.value === 'all' ? 'all' : Number(e.target.value))}
                  className="rounded-xl border border-slate-200 bg-white px-3 py-1.5 text-xs font-medium text-slate-700 shadow-2xs focus:outline-hidden focus:ring-1 focus:ring-slate-400 dark:border-slate-800 dark:bg-slate-900 dark:text-slate-200"
                >
                  <option value="all">{t('draft.overview.filterAll')}</option>
                  {overview.arcs.map(arc => (
                    <option key={arc.id} value={arc.number}>
                      {t('draft.overview.arcNum', { num: arc.number })}
                    </option>
                  ))}
                </select>

              </div>
              <div className="flex items-center gap-2">
                <button onClick={expandAll} className="text-[11px] font-medium text-slate-500 hover:text-slate-900 dark:text-slate-400 dark:hover:text-slate-200">{t('draft.overview.expandAll')}</button>
                <span className="text-slate-300 dark:text-slate-600">|</span>
                <button onClick={collapseAll} className="text-[11px] font-medium text-slate-500 hover:text-slate-900 dark:text-slate-400 dark:hover:text-slate-200">{t('draft.overview.collapseAll')}</button>
              </div>
            </div>
            {filteredArcs.map(arc => (
              <ArcCard
                key={arc.id}
                arc={arc}
                isExpanded={expandedArcs.has(arc.id)}
                onToggle={() => toggleArc(arc.id)}
              />
            ))}
          </div>
        )}

        {activeTab === 'characters' && (
          <div className="space-y-3">
            <p className="text-xs text-slate-500 dark:text-slate-400">{t('draft.overview.charDesc')}</p>
            <div className="grid grid-cols-1 md:grid-cols-2 gap-3">
              {overview.characters.map(char => (
                <CharacterCard key={char.id} character={char} />
              ))}
            </div>
          </div>
        )}

        {activeTab === 'world' && overview.worldSettings && (
          <div className="space-y-3 animate-in fade-in duration-300">
            <p className="text-xs text-slate-500 dark:text-slate-400">{t('draft.overview.worldDesc')}</p>
            <WorldSettingsBlock settings={overview.worldSettings} />
          </div>
        )}
      </div>

      {/* Genre Modal */}
      <Dialog open={allGenresOpen} onOpenChange={setAllGenresOpen}>
        <DialogContent className="max-w-md p-6">
          <div className="flex items-center gap-2 border-b border-slate-200 pb-3 dark:border-slate-800">
            <Tags className="h-4 w-4 text-indigo-500" />
            <DialogTitle className="text-sm font-bold text-slate-900 dark:text-slate-100">
              {t('draft.overview.allGenres')}
            </DialogTitle>
          </div>
          <div className="flex max-h-[50vh] flex-wrap gap-2 overflow-y-auto py-4">
            {overview.genres.map((genre) => (
              <span
                key={genre.name}
                className={cn(
                  'flex items-center gap-1.5 whitespace-nowrap rounded-xl px-3 py-1.5 text-xs',
                  genre.isPrimary
                    ? 'border border-indigo-500/30 bg-indigo-500/10 font-bold text-indigo-700 dark:text-indigo-300'
                    : 'border border-slate-200/80 bg-white/60 font-medium text-slate-600 dark:border-slate-700/80 dark:bg-slate-900/40 dark:text-slate-300'
                )}
              >
                {genre.isPrimary && <Bookmark className="h-3 w-3 fill-current" />}
                <span>{genre.name}</span>
              </span>
            ))}
          </div>
          <div className="flex justify-end border-t border-slate-200 pt-3 dark:border-slate-800">
            <button
              type="button"
              onClick={() => setAllGenresOpen(false)}
              className="rounded-xl px-4 py-1.5 text-xs font-bold text-slate-700 transition-all hover:neu-pressed dark:text-slate-200"
            >
              {t('common.close')}
            </button>
          </div>
        </DialogContent>
      </Dialog>
    </div>
  );
}
