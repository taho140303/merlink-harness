import { useState, useEffect } from 'react';
import { Bookmark, Tags, BookOpen, Users, Globe2, Film, HeartHandshake } from 'lucide-react';
import { cn } from '@/shared/utils/cn';
import { Dialog, DialogContent, DialogTitle, DialogDescription } from '@/shared/components/ui/dialog';
import { useT } from '@/shared/i18n/useT';
import { ArcCard } from './overview/ArcCard';
import { CharacterCard } from './overview/CharacterCard';
import { WorldSettingsBlock } from './overview/WorldSettingsBlock';
import { SceneConfigBlock } from './overview/SceneConfigBlock';
import { RelationshipBlock } from './overview/RelationshipBlock';
import { CharacterDetailDialog } from './overview/CharacterDetailDialog';
import type { DraftOverview, DraftStep, CharacterSubStep, DraftCharacter } from './types';

const MAX_VISIBLE_GENRES = 9;

interface DraftOverviewBlockProps {
  overview: DraftOverview;
  currentStep: DraftStep;
  currentSubStep?: CharacterSubStep;
  isFeedbackMode?: boolean;
  onAttachFeedback?: (id: string, tag: string, title: string) => void;
}

export function DraftOverviewBlock({
  overview,
  currentStep,
  currentSubStep,
  isFeedbackMode,
  onAttachFeedback,
}: DraftOverviewBlockProps) {
  const t = useT();
  const [allGenresOpen, setAllGenresOpen] = useState(false);
  
  const [activeTab, setActiveTab] = useState<'arcs' | 'characters' | 'world' | 'scene' | 'relationship'>('arcs');
  const [arcFilter, setArcFilter] = useState<'all' | number>('all');
  const [expandedArcs, setExpandedArcs] = useState<Set<string>>(new Set());
  const [selectedCharacter, setSelectedCharacter] = useState<DraftCharacter | null>(null);

  const visibleGenres = overview.genres.slice(0, MAX_VISIBLE_GENRES);
  const remainingGenres = overview.genres.length - MAX_VISIBLE_GENRES;

  const targets = [
    { key: 'arcs', label: t('draft.overview.targetArc'), value: overview.target.arcs, hashClass: 'text-indigo-500', valueClass: 'text-indigo-600' },
    { key: 'chapters', label: t('draft.overview.targetChapter'), value: overview.target.chapters, hashClass: 'text-emerald-500', valueClass: 'text-emerald-600' },
    { key: 'words', label: t('draft.overview.targetWord'), value: overview.target.wordPerChapter, hashClass: 'text-violet-500', valueClass: 'text-violet-600' },
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

  // Điều kiện mở tab theo Phase 1 -> Phase 6 của business-flow
  const showWorldTab = ['world', 'scene', 'character', 'relationship'].includes(currentStep);
  const showSceneTab = ['scene', 'character', 'relationship'].includes(currentStep);
  const showRelationshipTab = currentStep === 'relationship';

  // Nút "Xem chi tiết" của nhân vật CHỈ mở ra từ Phase 5 (character) và Phase 6 (relationship)
  const canViewCharacterDetail = ['character', 'relationship'].includes(currentStep);

  // Tự động chuyển tab tương ứng khi tiến trình bước sang giai đoạn mới
  useEffect(() => {
    if (currentStep === 'world') setActiveTab('world');
    else if (currentStep === 'scene') setActiveTab('scene');
    else if (currentStep === 'character') setActiveTab('characters');
    else if (currentStep === 'relationship') setActiveTab('relationship');
  }, [currentStep]);

  return (
    <div className="no-scrollbar relative flex h-full w-full flex-col overflow-y-auto bg-surface-muted/40">
      {/* Background Orbs */}
      <div className="pointer-events-none fixed inset-0 z-0 overflow-hidden">
        <div className="absolute -top-40 -left-40 h-96 w-96 rounded-full bg-indigo-300/30 blur-[100px] dark:bg-indigo-900/20" />
        <div className="absolute top-1/3 -right-40 h-96 w-96 rounded-full bg-pink-300/30 blur-[100px] dark:bg-pink-900/20" />
        <div className="absolute -bottom-40 left-1/3 h-96 w-96 rounded-full bg-violet-300/30 blur-[100px] dark:bg-violet-900/20" />
      </div>

      <div className="relative z-10 mx-auto flex w-full max-w-5xl flex-col gap-6 p-6 pb-24">
        {/* Header */}
        <header 
          onClick={() => isFeedbackMode && onAttachFeedback?.('overview', 'overview', overview.displayName)}
          className={cn(
            "tier-panel rounded-panel p-5 transition-colors duration-300",
            isFeedbackMode && "hover:!border-2 hover:!border-dashed hover:!border-purple-500 hover:bg-purple-500/5 cursor-none"
          )}
        >
          <div className="flex flex-col gap-3 md:flex-row md:items-center md:justify-between">
            <div className="flex min-w-0 items-center gap-3">
              <h2 className="min-w-0 text-2xl font-black leading-tight tracking-tight text-fg">
                {overview.displayName}
              </h2>
              <span className="shrink-0 rounded-xl px-2.5 py-1 font-mono text-xs font-bold uppercase tracking-widest bg-surface-muted text-indigo-600 dark:text-indigo-400">
                {overview.language}
              </span>
            </div>
          </div>

          <p className="mt-4 max-w-4xl text-base leading-relaxed text-fg-muted">
            {overview.summary}
          </p>

          <div className="mt-4 flex flex-col justify-between gap-4 lg:flex-row lg:flex-wrap lg:items-center">
            <div className="flex w-fit shrink-0 flex-wrap items-center gap-2 rounded-xl bg-surface-muted p-1.5">
              {targets.map((target) => (
                <div
                  key={target.key}
                  className="flex items-center gap-1.5 whitespace-nowrap tier-card rounded-card px-3 py-1.5 font-mono text-sm font-bold text-fg-muted"
                >
                  <span className={cn('font-black', target.hashClass)}>#</span>
                  <span className="text-fg-subtle">{target.label}:</span>
                  <span className={cn('text-base font-extrabold', target.valueClass)}>{target.value}</span>
                </div>
              ))}
            </div>

            <div className="flex flex-wrap items-center gap-2">
              {visibleGenres.map((genre) => (
                <span
                  key={genre.name}
                  className={cn(
                    'flex items-center gap-1.5 whitespace-nowrap rounded-xl px-3 py-1.5 text-sm',
                    genre.isPrimary
                      ? 'border border-indigo-500/30 bg-indigo-500/10 font-bold text-indigo-700 dark:text-indigo-300'
                      : 'border border-line bg-surface-card/60 font-medium text-fg-muted'
                  )}
                >
                  {genre.isPrimary && <Bookmark className="h-3.5 w-3.5 fill-current" />}
                  <span>{genre.name}</span>
                </span>
              ))}
              {remainingGenres > 0 && (
                <button
                  type="button"
                  onClick={(e) => { e.stopPropagation(); if (!isFeedbackMode) setAllGenresOpen(true); }}
                  aria-label={t('draft.overview.allGenres')}
                  className="flex items-center gap-1.5 rounded-xl px-3 py-1.5 text-sm font-black text-indigo-600 transition-colors hover:bg-surface-muted dark:text-indigo-400"
                >
                  <span>...</span>
                  <span className="rounded-lg px-1.5 py-0.5 font-mono text-xs font-bold text-fg-subtle bg-surface-muted">
                    +{remainingGenres}
                  </span>
                </button>
              )}
            </div>
          </div>
        </header>

        {/* Nav Bar */}
        <div className="flex items-center gap-1 rounded-xl bg-surface-muted p-1 w-fit flex-wrap">
          <button
            onClick={(e) => { e.stopPropagation(); setActiveTab('arcs'); }}
            aria-pressed={activeTab === 'arcs'}
            className={cn(
              "flex items-center gap-2 rounded-lg px-4 py-1.5 text-sm font-semibold transition-colors",
              activeTab === 'arcs' ? "bg-surface-card text-fg" : "text-fg-subtle hover:text-fg"
            )}
          >
            <BookOpen className="h-3.5 w-3.5" /> {t('draft.overview.tabArcs')}
          </button>
          <button
            onClick={(e) => { e.stopPropagation(); setActiveTab('characters'); }}
            aria-pressed={activeTab === 'characters'}
            className={cn(
              "flex items-center gap-2 rounded-lg px-4 py-1.5 text-sm font-semibold transition-colors",
              activeTab === 'characters' ? "bg-surface-card text-fg" : "text-fg-subtle hover:text-fg"
            )}
          >
            <Users className="h-3.5 w-3.5" /> {t('draft.overview.tabCharacters')}
            <span className="rounded-full bg-surface-muted px-1.5 py-0.5 text-xs text-fg-muted">{overview.characters.length}</span>
          </button>

          {/* Phase 3: World building */}
          {showWorldTab && (
            <button
              onClick={(e) => { e.stopPropagation(); setActiveTab('world'); }}
              aria-pressed={activeTab === 'world'}
              className={cn(
                "flex items-center gap-2 rounded-lg px-4 py-1.5 text-sm font-semibold transition-colors -from-left-2",
                activeTab === 'world' ? "bg-surface-card text-fg" : "text-fg-subtle hover:text-fg"
              )}
            >
              <Globe2 className="h-3.5 w-3.5" /> {t('draft.overview.tabWorld')}
            </button>
          )}

          {/* Phase 4: Scene config */}
          {showSceneTab && (
            <button
              onClick={(e) => { e.stopPropagation(); setActiveTab('scene'); }}
              aria-pressed={activeTab === 'scene'}
              className={cn(
                "flex items-center gap-2 rounded-lg px-4 py-1.5 text-sm font-semibold transition-colors -from-left-2",
                activeTab === 'scene' ? "bg-surface-card text-fg" : "text-fg-subtle hover:text-fg"
              )}
            >
              <Film className="h-3.5 w-3.5" /> {t('draft.overview.tabScene')}
            </button>
          )}

          {/* Phase 6: Relationship */}
          {showRelationshipTab && (
            <button
              onClick={(e) => { e.stopPropagation(); setActiveTab('relationship'); }}
              aria-pressed={activeTab === 'relationship'}
              className={cn(
                "flex items-center gap-2 rounded-lg px-4 py-1.5 text-sm font-semibold transition-colors -from-left-2",
                activeTab === 'relationship' ? "bg-surface-card text-fg" : "text-fg-subtle hover:text-fg"
              )}
            >
              <HeartHandshake className="h-3.5 w-3.5" /> Quan hệ (Phase 6)
            </button>
          )}
        </div>

        {/* Panels */}
        {activeTab === 'arcs' && (
          <div className="w-full space-y-3">
            <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-3 w-full">
              <div className="flex items-center gap-2">
                <span className="text-xs font-semibold uppercase tracking-wider text-fg-subtle">{t('draft.overview.filterArc')}</span>
                <select
                  value={arcFilter}
                  onChange={(e) => setArcFilter(e.target.value === 'all' ? 'all' : Number(e.target.value))}
                  onClick={(e) => e.stopPropagation()}
                  className="tier-card rounded-card px-3 py-1.5 text-sm font-medium text-fg-muted focus-ring"
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
                <button onClick={(e) => { e.stopPropagation(); expandAll(); }} className="text-xs font-medium text-fg-subtle hover:text-fg">{t('draft.overview.expandAll')}</button>
                <span className="text-fg-subtle">|</span>
                <button onClick={(e) => { e.stopPropagation(); collapseAll(); }} className="text-xs font-medium text-fg-subtle hover:text-fg">{t('draft.overview.collapseAll')}</button>
              </div>
            </div>
            {filteredArcs.map(arc => (
              <ArcCard
                key={arc.id}
                arc={arc}
                isExpanded={expandedArcs.has(arc.id)}
                onToggle={() => toggleArc(arc.id)}
                isFeedbackMode={isFeedbackMode}
                onAttachFeedback={onAttachFeedback}
              />
            ))}
          </div>
        )}

        {activeTab === 'characters' && (
          <div className="space-y-3">
            <p className="text-sm text-fg-muted">{t('draft.overview.charDesc')}</p>
            <div className="grid grid-cols-1 md:grid-cols-2 gap-3">
              {overview.characters.map(char => (
                <CharacterCard 
                  key={char.id} 
                  character={char} 
                  canViewDetail={canViewCharacterDetail}
                  onView={() => setSelectedCharacter(char)}
                  isFeedbackMode={isFeedbackMode}
                  onAttachFeedback={onAttachFeedback}
                />
              ))}
            </div>
          </div>
        )}

        {activeTab === 'world' && overview.worldSettings && (
          <div className="space-y-3 duration-300">
            <p className="text-sm text-fg-muted">{t('draft.overview.worldDesc')}</p>
            <WorldSettingsBlock 
              settings={overview.worldSettings} 
              isFeedbackMode={isFeedbackMode}
              onAttachFeedback={onAttachFeedback}
            />
          </div>
        )}

        {activeTab === 'scene' && overview.sceneConfig && (
          <div className="space-y-3 duration-300">
            <p className="text-sm text-fg-muted">{t('draft.overview.sceneDesc')}</p>
            <SceneConfigBlock
              config={overview.sceneConfig}
              isFeedbackMode={isFeedbackMode}
              onAttachFeedback={onAttachFeedback}
            />
          </div>
        )}

        {activeTab === 'relationship' && overview.relationships && (
          <div className="space-y-3 duration-300">
            <RelationshipBlock
              relationships={overview.relationships}
              isFeedbackMode={isFeedbackMode}
              onAttachFeedback={onAttachFeedback}
            />
          </div>
        )}
      </div>

      {/* Genre Modal */}
      <Dialog open={allGenresOpen} onOpenChange={setAllGenresOpen}>
        <DialogContent className="max-w-md p-6">
          <div className="flex items-center gap-2 border-b border-line pb-3">
            <Tags className="h-4 w-4 text-indigo-500" />
            <DialogTitle className="text-base font-bold text-fg">
              {t('draft.overview.allGenres')}
            </DialogTitle>
            <DialogDescription className="sr-only">{t('draft.overview.allGenres')}</DialogDescription>
          </div>
          <div className="flex max-h-[50vh] flex-wrap gap-2 overflow-y-auto py-4">
            {overview.genres.map((genre) => (
              <span
                key={genre.name}
                className={cn(
                  'flex items-center gap-1.5 whitespace-nowrap rounded-xl px-3 py-1.5 text-sm',
                  genre.isPrimary
                    ? 'border border-indigo-500/30 bg-indigo-500/10 font-bold text-indigo-700 dark:text-indigo-300'
                    : 'border border-line bg-surface-card/60 font-medium text-fg-muted'
                )}
              >
                {genre.isPrimary && <Bookmark className="h-3.5 w-3.5 fill-current" />}
                <span>{genre.name}</span>
              </span>
            ))}
          </div>
          <div className="flex justify-end border-t border-line pt-3">
            <button
              type="button"
              onClick={() => setAllGenresOpen(false)}
              className="rounded-xl px-4 py-1.5 text-sm font-bold text-fg-muted transition-colors hover:bg-surface-muted"
            >
              {t('common.close')}
            </button>
          </div>
        </DialogContent>
      </Dialog>

      {/* Character Detail Dialog */}
      <CharacterDetailDialog 
        open={!!selectedCharacter} 
        onOpenChange={(open) => !open && setSelectedCharacter(null)} 
        character={selectedCharacter} 
        currentStep={currentStep}
        currentSubStep={currentSubStep}
      />
    </div>
  );
}
