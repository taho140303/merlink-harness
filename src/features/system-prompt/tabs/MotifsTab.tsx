import { useState } from 'react';
import { ChevronRight, LayoutGrid, Network } from 'lucide-react';
import { cn } from '@/shared/utils/cn';
import { useT } from '@/shared/i18n/useT';
import { InfoTooltip } from '../shared/InfoTooltip';
import { humanize } from '../shared/humanize';
import {
  components,
  constraints,
  emotionalRanges,
  eventBlockEmotionals,
  eventBlocks,
  eventTransitions,
  genres,
  motifApplicableGenres,
  motifConstraints,
  motifs,
  stateDownByRules,
  stateRules,
  states,
} from '../mockData';
import { MotifCanvas } from '../canvas/MotifCanvas';
import type { Motif, MotifCategory } from '../types';

const CATEGORY_COLOR: Record<MotifCategory, string> = {
  character_arc: 'bg-indigo-100 text-indigo-700 dark:bg-indigo-950/60 dark:text-indigo-300',
  relationship: 'bg-rose-100 text-rose-700 dark:bg-rose-950/60 dark:text-rose-300',
  symbolic: 'bg-violet-100 text-violet-700 dark:bg-violet-950/60 dark:text-violet-300',
  plot: 'bg-emerald-100 text-emerald-700 dark:bg-emerald-950/60 dark:text-emerald-300',
};

interface MotifsTabProps {
  activeChild: { id: string; name: string } | null;
  onSelectChild: (child: { id: string; name: string } | null) => void;
}

export function MotifsTab({ activeChild, onSelectChild }: MotifsTabProps) {
  if (activeChild) {
    const motif = motifs.find((m) => m.id === activeChild.id);
    if (motif) {
      return <MotifDetailView motif={motif} />;
    }
  }

  return <MotifListView onSelect={(mo) => onSelectChild({ id: mo.id, name: mo.name })} />;
}

/* ============================================================
 * LIST VIEW
 * ============================================================ */

function MotifListView({ onSelect }: { onSelect: (motif: Motif) => void }) {
  const t = useT();
  const genreById = new Map(genres.map((g) => [g.id, g]));

  const genresForMotif = (motifId: string) =>
    motifApplicableGenres
      .filter((mg) => mg.motifId === motifId)
      .map((mg) => genreById.get(mg.genreId))
      .filter((g): g is (typeof genres)[number] => Boolean(g));

  return (
    <div className="space-y-4">
      <p className="text-xs text-fg-muted">{t('sp.motifs.clickHint')}</p>

      <div className="grid grid-cols-1 gap-3 md:grid-cols-2">
        {motifs.map((mo) => {
          const applicable = genresForMotif(mo.id);
          return (
            <button
              key={mo.id}
              type="button"
              onClick={() => onSelect(mo)}
              className="group rounded-xl border border-line bg-surface-card p-4 text-left  transition-colors hover:border-line-strong hover:   "
            >
              <div className="flex items-start justify-between gap-2">
                <div className="min-w-0">
                  <h4 className="font-semibold text-fg">
                    {mo.name}
                  </h4>
                  <p className="mt-0.5 truncate font-mono text-xs text-fg-muted">
                    {mo.code} · v{mo.version}
                  </p>
                </div>
                <span
                  className={cn(
                    'shrink-0 rounded-full px-2 py-0.5 text-xs font-medium',
                    CATEGORY_COLOR[mo.category],
                  )}
                >
                  {humanize(mo.category)}
                </span>
              </div>

              <p className="mt-2 line-clamp-2 text-xs leading-relaxed text-fg-muted">
                {mo.description}
              </p>

              <div className="mt-3 flex flex-wrap items-center gap-1.5 border-t border-line pt-2.5 text-xs ">
                {mo.isSystem ? (
                  <span className="rounded-full bg-surface-muted px-2 py-0.5 font-medium text-fg-muted  ">
                    {t('sp.common.system')}
                  </span>
                ) : (
                  <span className="rounded-full bg-emerald-100 px-2 py-0.5 font-medium text-emerald-700 dark:bg-emerald-950/60 dark:text-emerald-300">
                    {t('sp.common.custom')}
                  </span>
                )}
                {applicable.map((g) => (
                  <span
                    key={g.id}
                    className="rounded-full bg-blue-100 px-2 py-0.5 font-medium text-blue-700 dark:bg-blue-950/60 dark:text-blue-300"
                  >
                    {g.name}
                  </span>
                ))}
                <span className="ml-auto flex items-center gap-1 text-fg-subtle transition-colors group-hover:text-fg-muted ">
                  {t('sp.motif.viewDetail')}
                  <ChevronRight className="h-3 w-3" />
                </span>
              </div>
            </button>
          );
        })}
      </div>
    </div>
  );
}

/* ============================================================
 * DETAIL VIEW
 * ============================================================ */

function MotifDetailView({ motif }: { motif: Motif }) {
  const t = useT();
  const [viewMode, setViewMode] = useState<'default' | 'canvas'>('default');

  const genreById = new Map(genres.map((g) => [g.id, g]));
  const componentById = new Map(components.map((c) => [c.id, c]));
  const stateById = new Map(states.map((s) => [s.id, s]));
  const constraintById = new Map(constraints.map((c) => [c.id, c]));
  const emotionalById = new Map(emotionalRanges.map((e) => [e.id, e]));
  const eventBlockById = new Map(eventBlocks.map((e) => [e.id, e]));

  const applicableGenres = motifApplicableGenres
    .filter((mg) => mg.motifId === motif.id)
    .map((mg) => genreById.get(mg.genreId))
    .filter((g): g is (typeof genres)[number] => Boolean(g));

  const motifConstraintRows = motifConstraints
    .filter((mc) => mc.motifId === motif.id)
    .map((mc) => ({ mc, constraint: constraintById.get(mc.constraintId) }))
    .filter((r) => Boolean(r.constraint));

  const motifEventBlocks = eventBlocks.filter((eb) => eb.motifId === motif.id);

  const emotionsForEventBlock = (ebId: string) =>
    eventBlockEmotionals
      .filter((ebe) => ebe.eventBlockId === ebId)
      .map((ebe) => emotionalById.get(ebe.emotionalRangeId))
      .filter((e): e is (typeof emotionalRanges)[number] => Boolean(e));

  const transitionsForEventBlock = (ebId: string) =>
    eventTransitions
      .filter((et) => et.fromEventBlockId === ebId)
      .map((et) => ({ et, target: eventBlockById.get(et.toEventBlockId) }))
      .filter((r) => Boolean(r.target));

  // State Mechanics: lấy các state xuất hiện trong event blocks của motif
  const usedStateIds = Array.from(new Set(motifEventBlocks.map(eb => eb.stateId)));
  const relevantStates = states.filter(st => usedStateIds.includes(st.id));

  return (
    <div className="space-y-4">
      {/* Header card */}
      <div className="tier-card rounded-card p-5">
        <div className="flex items-start justify-between gap-3">
          <div className="min-w-0">
            <h2 className="text-lg font-bold text-fg">
              {motif.name}
            </h2>
            <p className="mt-0.5 font-mono text-xs text-fg-muted">
              {motif.code} · v{motif.version}
            </p>
          </div>
          <div className="flex flex-wrap items-center gap-1.5">
            <span
              className={cn(
                'rounded-full px-2 py-0.5 text-xs font-medium',
                CATEGORY_COLOR[motif.category],
              )}
            >
              {humanize(motif.category)}
            </span>
            {motif.isSystem && (
              <span className="rounded-full bg-surface-muted px-2 py-0.5 text-xs font-medium text-fg-muted  ">
                {t('sp.common.system')}
              </span>
            )}
          </div>
        </div>

        <p className="mt-3 text-xs leading-relaxed text-fg-muted">
          {motif.description}
        </p>

        <div className="mt-3 flex flex-wrap items-center gap-1.5 border-t border-line pt-3 text-xs ">
          {motif.minChapter !== undefined && motif.maxChapter !== undefined && (
            <span className="rounded-full bg-surface-muted px-2 py-0.5 font-medium text-fg-muted  ">
              {t('sp.motifs.chapterRange')}: {motif.minChapter}–{motif.maxChapter}
            </span>
          )}
          {applicableGenres.length > 0 ? (
            applicableGenres.map((g) => (
              <span
                key={g.id}
                className="rounded-full bg-blue-100 px-2 py-0.5 font-medium text-blue-700 dark:bg-blue-950/60 dark:text-blue-300"
              >
                {g.name}
              </span>
            ))
          ) : (
            <span className="text-fg-subtle">{t('sp.motifs.noGenres')}</span>
          )}
        </div>
      </div>

      {/* View mode switch */}
      <div className="flex w-fit items-center gap-1 rounded-xl bg-surface-muted p-0.5 ">
        <button
          type="button"
          onClick={() => setViewMode('default')}
          className={cn(
            'flex items-center gap-1.5 rounded-lg px-3 py-1.5 text-xs font-medium transition-colors',
            viewMode === 'default'
              ? 'bg-surface-card text-fg   '
              : 'text-fg-muted hover:text-fg -subtle ',
          )}
        >
          <LayoutGrid className="h-3.5 w-3.5" />
          <span>{t('sp.motifs.viewDefault')}</span>
        </button>
        <button
          type="button"
          onClick={() => setViewMode('canvas')}
          className={cn(
            'flex items-center gap-1.5 rounded-lg px-3 py-1.5 text-xs font-medium transition-colors',
            viewMode === 'canvas'
              ? 'bg-surface-card text-fg   '
              : 'text-fg-muted hover:text-fg -subtle ',
          )}
        >
          <Network className="h-3.5 w-3.5" />
          <span>{t('sp.motifs.viewCanvas')}</span>
        </button>
      </div>

      {viewMode === 'canvas' ? (
        <MotifCanvas
          motif={motif}
          eventBlocks={motifEventBlocks}
          eventTransitions={eventTransitions.filter((et) =>
            motifEventBlocks.some((eb) => eb.id === et.fromEventBlockId),
          )}
          componentById={componentById}
          stateById={stateById}
          emotionsForEventBlock={emotionsForEventBlock}
        />
      ) : (
        <>
          {/* Motif Constraints */}
          <section className="tier-card rounded-card p-4">
            <div className="mb-2 flex items-start gap-1.5">
              <h3 className="text-sm font-semibold text-fg">
                {t('sp.motifConstraints.title')}
              </h3>
              <InfoTooltip content={t('sp.motifConstraints.desc')} className="mt-0.5" />
            </div>
            {motifConstraintRows.length === 0 ? (
              <p className="text-xs text-fg-subtle">{t('sp.motifs.noConstraints')}</p>
            ) : (
              <ul className="space-y-1.5">
                {motifConstraintRows.map(({ mc, constraint }) => (
                  <li
                    key={mc.id}
                    className="rounded-lg border border-line bg-surface-muted p-2.5  "
                  >
                    <p className="font-mono text-xs text-fg-muted">
                      {constraint!.code}
                    </p>
                    <p className="mt-1 text-xs font-semibold text-fg">
                      <span className="text-fg-subtle">{t('sp.constraints.require')}:</span>{' '}
                      {constraint!.require}
                    </p>
                    <p className="mt-0.5 text-xs text-fg-muted">
                      <span className="text-fg-subtle">{t('sp.constraints.reason')}:</span>{' '}
                      {constraint!.reason}
                    </p>
                  </li>
                ))}
              </ul>
            )}
          </section>

          {/* State Mechanics (Chỉ relevantStates của motif này) */}
          {relevantStates.length > 0 && (
            <section className="tier-card rounded-card p-4">
              <div className="mb-3 flex items-start justify-between">
                <div className="flex items-start gap-1.5">
                  <div>
                    <h3 className="text-sm font-semibold text-fg">
                      State Mechanics
                    </h3>
                    <p className="mt-0.5 text-xs text-fg-muted">
                      Các quy tắc (rules) có thể hạ trạng thái (state) xuống cho motif này.
                    </p>
                  </div>
                  <InfoTooltip content="state_down_by_rules: Ánh xạ giữa State, Rule và việc Rule đó có làm hạ State không (is_allowed)." className="mt-0.5" />
                </div>
              </div>
              
              <div className="grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-3">
                {relevantStates.map((st) => {
                  const downRules = stateDownByRules.filter((r) => r.stateId === st.id);
                  return (
                    <div key={st.id} className="rounded-xl border border-line bg-surface-muted   ">
                      <div className="border-b border-line px-3 py-2 ">
                        <h4 className="font-semibold text-fg text-xs">{st.name}</h4>
                        <p className="mt-0.5 text-xs text-fg-muted">{st.description}</p>
                      </div>
                      <div className="p-3 space-y-1.5">
                        <p className="text-xs font-semibold uppercase tracking-wide text-fg-muted mb-1">
                          State Rules
                        </p>
                        {downRules.length === 0 ? (
                          <p className="text-xs text-fg-subtle">Không có rule nào.</p>
                        ) : (
                          downRules.map((dr) => {
                            const rule = stateRules.find((r) => r.id === dr.stateRuleId);
                            return (
                              <div key={dr.id} className="flex items-center justify-between gap-2 rounded-lg bg-surface-card px-2 py-1.5  ">
                                <span className="text-xs font-medium text-fg-muted">
                                  {rule?.name}
                                </span>
                                <span className={cn(
                                  "rounded-full px-1.5 py-0.5 text-xs font-medium",
                                  dr.isAllowed 
                                    ? "bg-emerald-100 text-emerald-700 dark:bg-emerald-950/60 dark:text-emerald-300"
                                    : "bg-rose-100 text-rose-700 dark:bg-rose-950/60 dark:text-rose-300"
                                )}>
                                  {dr.isAllowed ? "Allowed" : "Forbidden"}
                                </span>
                              </div>
                            );
                          })
                        )}
                      </div>
                    </div>
                  );
                })}
              </div>
            </section>
          )}

          {/* Event Blocks */}
          <section>
            <div className="mb-2 flex items-center gap-1.5">
              <h3 className="text-sm font-semibold text-fg">
                {t('sp.eventBlocks.title')}
              </h3>
              <span className="rounded-full bg-surface-muted px-2 py-0.5 text-xs font-medium text-fg-muted  ">
                {motifEventBlocks.length}
              </span>
              <InfoTooltip content={t('sp.eventBlocks.desc')} />
            </div>

            {motifEventBlocks.length === 0 ? (
              <div className="rounded-xl border border-dashed border-line-strong bg-surface-card/60 p-8 text-center text-xs text-fg-subtle -strong ">
                {t('sp.motifs.noEventBlocks')}
              </div>
            ) : (
              <div className="space-y-2">
                {motifEventBlocks.map((eb) => {
                  const comp = componentById.get(eb.componentId);
                  const st = stateById.get(eb.stateId);
                  const emotions = emotionsForEventBlock(eb.id);
                  const transitions = transitionsForEventBlock(eb.id);

                  return (
                    <details
                      key={eb.id}
                      className="group tier-card rounded-card"
                    >
                      <summary className="flex cursor-pointer list-none items-center justify-between gap-3 px-4 py-3">
                        <div className="min-w-0">
                          <p className="truncate font-mono text-xs font-semibold text-fg">
                            {eb.code}
                          </p>
                          <p className="mt-0.5 truncate text-xs text-fg-muted">
                            <span className="font-medium text-indigo-600 dark:text-indigo-400">
                              {comp?.name}
                            </span>
                            <span className="mx-1 text-fg-subtle -muted">→</span>
                            <span className="font-medium text-emerald-600 dark:text-emerald-400">
                              {st?.name}
                            </span>
                          </p>
                        </div>
                        <div className="flex shrink-0 items-center gap-2 text-xs text-fg-muted">
                          <span>
                            {emotions.length} {t('sp.eventBlock.emotionalRanges').toLowerCase()}
                          </span>
                          <span className="text-fg-subtle -muted">·</span>
                          <span>
                            {transitions.length}{' '}
                            {t('sp.eventBlock.outgoingTransitions').toLowerCase()}
                          </span>
                          <span className="ml-1 text-fg-subtle transition-transform group-open:rotate-180">
                            ▾
                          </span>
                        </div>
                      </summary>

                      <div className="space-y-3 border-t border-line px-4 pb-4 pt-3 ">
                        <p className="text-xs italic text-fg-muted">
                          {t('sp.eventBlock.childHint')}
                        </p>

                        {/* Component + State */}
                        <div className="grid grid-cols-1 gap-3 md:grid-cols-2">
                          <div className="rounded-lg border border-indigo-100 bg-indigo-50/50 p-2.5 dark:border-indigo-900/60 dark:bg-indigo-950/30">
                            <p className="text-xs font-semibold uppercase tracking-wide text-indigo-700 dark:text-indigo-300">
                              {t('sp.eventBlock.component')}
                            </p>
                            <p className="mt-1 text-xs font-semibold text-fg">
                              {comp?.name}{' '}
                              <span className="font-mono text-xs text-fg-subtle">
                                · {comp?.code}
                              </span>
                            </p>
                            <p className="mt-0.5 text-xs text-fg-muted">
                              {comp?.description}
                            </p>
                          </div>
                          <div className="rounded-lg border border-emerald-100 bg-emerald-50/50 p-2.5 dark:border-emerald-900/60 dark:bg-emerald-950/30">
                            <p className="text-xs font-semibold uppercase tracking-wide text-emerald-700 dark:text-emerald-300">
                              {t('sp.eventBlock.state')}
                            </p>
                            <p className="mt-1 text-xs font-semibold text-fg">
                              {st?.name}{' '}
                              <span className="font-mono text-xs text-fg-subtle">
                                · {st?.code}
                              </span>
                            </p>
                            <p className="mt-0.5 text-xs text-fg-muted">
                              {st?.description}
                            </p>
                          </div>
                        </div>

                        {/* Emotional ranges */}
                        <div>
                          <p className="mb-1.5 flex items-center gap-1 text-xs font-semibold uppercase tracking-wide text-fg-muted">
                            {t('sp.eventBlock.emotionalRanges')}
                            <InfoTooltip content="Resolved via event_block_emotionals → emotional_ranges (-N)." />
                          </p>
                          {emotions.length === 0 ? (
                            <p className="text-xs text-fg-subtle">
                              {t('sp.eventBlock.noEmotionals')}
                            </p>
                          ) : (
                            <div className="flex flex-wrap gap-1.5">
                              {emotions.map((e) => (
                                <span
                                  key={e.id}
                                  className="rounded-full bg-amber-100 px-2 py-0.5 text-xs font-medium text-amber-800 dark:bg-amber-950/60 dark:text-amber-300"
                                >
                                  {e.name}
                                </span>
                              ))}
                            </div>
                          )}
                        </div>

                        {/* Outgoing transitions */}
                        <div>
                          <p className="mb-1.5 flex items-center gap-1 text-xs font-semibold uppercase tracking-wide text-fg-muted">
                            {t('sp.eventBlock.outgoingTransitions')}
                            <InfoTooltip content="Resolved via event_transitions.from_event_block_id (-N)." />
                          </p>
                          {transitions.length === 0 ? (
                            <p className="text-xs text-fg-subtle">
                              {t('sp.eventBlock.noTransitions')}
                            </p>
                          ) : (
                            <ul className="space-y-1">
                              {transitions.map(({ et, target }) => (
                                <li
                                  key={et.id}
                                  className="flex flex-wrap items-center gap-1.5 rounded-lg border border-line bg-surface-muted px-2.5 py-1.5 text-xs  "
                                >
                                  <span className="text-fg-subtle">→</span>
                                  <span className="font-mono font-semibold text-fg-muted">
                                    {target!.code}
                                  </span>
                                  <span className="ml-auto flex items-center gap-1 text-fg-muted">
                                    {t('sp.eventTransitions.priority')}: {et.priority}
                                    <InfoTooltip content={et.name} />
                                  </span>
                                </li>
                              ))}
                            </ul>
                          )}
                        </div>
                      </div>
                    </details>
                  );
                })}
              </div>
            )}
          </section>
        </>
      )}
    </div>
  );
}
