import { useState } from 'react';
import { ChevronRight, LayoutGrid, Network } from 'lucide-react';
import { cn } from '../../../../shared/utils/cn';
import { truncate } from '../../../../shared/utils/string';
import { useT } from '../../../../shared/i18n/useT';
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
  states,
} from '../mockData';
import type { Motif, MotifCategory } from '../types';
import { MotifCanvas } from '../canvas/MotifCanvas';

const CATEGORY_COLOR: Record<MotifCategory, string> = {
  character_arc: 'bg-indigo-100 text-indigo-700 dark:bg-indigo-950/60 dark:text-indigo-300',
  relationship: 'bg-rose-100 text-rose-700 dark:bg-rose-950/60 dark:text-rose-300',
  symbolic: 'bg-violet-100 text-violet-700 dark:bg-violet-950/60 dark:text-violet-300',
  plot: 'bg-emerald-100 text-emerald-700 dark:bg-emerald-950/60 dark:text-emerald-300',
};

export function MotifsTab() {
  const [selectedMotifId, setSelectedMotifId] = useState<string | null>(null);

  if (selectedMotifId) {
    const motif = motifs.find((m) => m.id === selectedMotifId);
    if (motif) {
      return (
        <MotifDetailView motif={motif} onBack={() => setSelectedMotifId(null)} />
      );
    }
  }

  return <MotifListView onSelect={(id) => setSelectedMotifId(id)} />;
}

/* ============================================================
 * LIST VIEW
 * ============================================================ */

function MotifListView({ onSelect }: { onSelect: (id: string) => void }) {
  const t = useT();
  const genreById = new Map(genres.map((g) => [g.id, g]));

  const genresForMotif = (motifId: string) =>
    motifApplicableGenres
      .filter((mg) => mg.motifId === motifId)
      .map((mg) => genreById.get(mg.genreId))
      .filter((g): g is (typeof genres)[number] => Boolean(g));

  return (
    <div className="space-y-8">
      <section>
        <h3 className="mb-2 text-sm font-semibold text-slate-900 dark:text-slate-100">
          {t('sp.motifs.title')}
        </h3>
        <p className="mb-4 text-xs text-slate-500 dark:text-slate-400">{t('sp.motifs.clickHint')}</p>
        <div className="grid grid-cols-1 gap-3 md:grid-cols-2">
          {motifs.map((mo) => {
          const applicable = genresForMotif(mo.id);
          return (
            <button
              key={mo.id}
              type="button"
              onClick={() => onSelect(mo.id)}
              className="group rounded-xl border border-slate-200 bg-white p-4 text-left shadow-sm transition-all hover:border-slate-300 hover:shadow-md dark:border-slate-800 dark:bg-slate-900 dark:hover:border-slate-700"
            >
              <div className="flex items-start justify-between gap-2">
                <div className="min-w-0">
                  <h4 className="font-semibold text-slate-900 dark:text-slate-100">
                    {mo.name}
                  </h4>
                  <p className="mt-0.5 truncate font-mono text-[11px] text-slate-500 dark:text-slate-400">
                    {mo.code} · v{mo.version}
                  </p>
                </div>
                <span
                  className={cn(
                    'shrink-0 rounded-full px-2 py-0.5 text-[10px] font-medium',
                    CATEGORY_COLOR[mo.category],
                  )}
                >
                  {humanize(mo.category)}
                </span>
              </div>

              <p className="mt-2 line-clamp-2 text-xs leading-relaxed text-slate-700 dark:text-slate-300">
                {mo.description}
              </p>

              <div className="mt-3 flex flex-wrap items-center gap-1.5 border-t border-slate-100 pt-2.5 text-[10px] dark:border-slate-800">
                {mo.isSystem ? (
                  <span className="rounded-full bg-slate-100 px-2 py-0.5 font-medium text-slate-700 dark:bg-slate-800 dark:text-slate-300">
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
                <span className="ml-auto flex items-center gap-1 text-slate-400 transition-colors group-hover:text-slate-700 dark:group-hover:text-slate-200">
                  {t('sp.motif.viewDetail')}
                  <ChevronRight className="h-3 w-3" />
                </span>
              </div>
            </button>
          );
          })}
        </div>
      </section>

      {/* State Mechanics Section */}
      <section>
        <div className="mb-3 flex items-start justify-between">
          <div>
            <h3 className="font-semibold text-slate-900 dark:text-slate-100">State Mechanics</h3>
            <p className="mt-0.5 text-xs text-slate-500 dark:text-slate-400">
              Các quy tắc (rules) có thể hạ trạng thái (state) xuống.
            </p>
          </div>
          <InfoTooltip content="state_down_by_rules: Ánh xạ giữa State, Rule và việc Rule đó có làm hạ State không (is_allowed)." />
        </div>
        
        <div className="grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-3">
          {states.map((st) => {
            const downRules = stateDownByRules.filter((r) => r.stateId === st.id);
            return (
              <div key={st.id} className="rounded-xl border border-slate-200 bg-white shadow-sm dark:border-slate-800 dark:bg-slate-900">
                <div className="border-b border-slate-100 px-4 py-3 dark:border-slate-800">
                  <h4 className="font-semibold text-slate-900 dark:text-slate-100">{st.name}</h4>
                  <p className="mt-0.5 text-[11px] text-slate-500 dark:text-slate-400">{st.description}</p>
                </div>
                <div className="p-4 space-y-2">
                  <p className="text-[10px] font-semibold uppercase tracking-wide text-slate-500 dark:text-slate-400 mb-1">
                    State Rules
                  </p>
                  {downRules.length === 0 ? (
                    <p className="text-xs text-slate-400">Không có rule nào.</p>
                  ) : (
                    downRules.map((dr) => {
                      const rule = stateRules.find((r) => r.id === dr.stateRuleId);
                      return (
                        <div key={dr.id} className="flex items-center justify-between gap-2 rounded-lg bg-slate-50 px-2.5 py-1.5 dark:bg-slate-800/60">
                          <span className="text-xs font-medium text-slate-700 dark:text-slate-300">
                            {rule?.name}
                          </span>
                          <span className={cn(
                            "rounded-full px-2 py-0.5 text-[10px] font-medium",
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
    </div>
  );
}

/* ============================================================
 * DETAIL VIEW
 * ============================================================ */

function MotifDetailView({ motif, onBack }: { motif: Motif; onBack: () => void }) {
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

  return (
    <div className="space-y-4">
      {/* Breadcrumb */}
      <div className="flex flex-wrap items-center gap-1.5 text-xs font-semibold text-slate-500 dark:text-slate-400">
        <span>{t('sp.title')}</span>
        <span>›</span>
        <button
          type="button"
          onClick={onBack}
          className="hover:text-slate-900 dark:hover:text-slate-100"
        >
          {t('sp.tab.motifs')}
        </button>
        <span>›</span>
        <span
          className="max-w-[200px] truncate font-bold text-slate-900 dark:text-slate-100"
          title={motif.name}
        >
          {truncate(motif.name, 15)}
        </span>
      </div>

      {/* Header card */}
      <div className="rounded-xl border border-slate-200 bg-white p-5 shadow-sm dark:border-slate-800 dark:bg-slate-900">
        <div className="flex items-start justify-between gap-3">
          <div className="min-w-0">
            <h2 className="text-lg font-bold text-slate-900 dark:text-slate-100">
              {motif.name}
            </h2>
            <p className="mt-0.5 font-mono text-[11px] text-slate-500 dark:text-slate-400">
              {motif.code} · v{motif.version}
            </p>
          </div>
          <div className="flex flex-wrap items-center gap-1.5">
            <span
              className={cn(
                'rounded-full px-2 py-0.5 text-[10px] font-medium',
                CATEGORY_COLOR[motif.category],
              )}
            >
              {humanize(motif.category)}
            </span>
            {motif.isSystem && (
              <span className="rounded-full bg-slate-100 px-2 py-0.5 text-[10px] font-medium text-slate-700 dark:bg-slate-800 dark:text-slate-300">
                {t('sp.common.system')}
              </span>
            )}
          </div>
        </div>

        <p className="mt-3 text-xs leading-relaxed text-slate-700 dark:text-slate-300">
          {motif.description}
        </p>

        <div className="mt-3 flex flex-wrap items-center gap-1.5 border-t border-slate-100 pt-3 text-[11px] dark:border-slate-800">
          {motif.minChapter !== undefined && motif.maxChapter !== undefined && (
            <span className="rounded-full bg-slate-100 px-2 py-0.5 font-medium text-slate-700 dark:bg-slate-800 dark:text-slate-300">
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
            <span className="text-slate-400">{t('sp.motifs.noGenres')}</span>
          )}
        </div>
      </div>

      {/* View mode switch (only inside motif detail) */}
      <div className="flex w-fit items-center gap-1 rounded-xl bg-slate-100 p-0.5 dark:bg-slate-800">
        <button
          type="button"
          onClick={() => setViewMode('default')}
          className={cn(
            'flex items-center gap-1.5 rounded-lg px-3 py-1.5 text-xs font-medium transition-all',
            viewMode === 'default'
              ? 'bg-white text-slate-900 shadow-xs dark:bg-slate-700 dark:text-white'
              : 'text-slate-600 hover:text-slate-900 dark:text-slate-400 dark:hover:text-slate-200',
          )}
        >
          <LayoutGrid className="h-3.5 w-3.5" />
          <span>{t('sp.motifs.viewDefault')}</span>
        </button>
        <button
          type="button"
          onClick={() => setViewMode('canvas')}
          className={cn(
            'flex items-center gap-1.5 rounded-lg px-3 py-1.5 text-xs font-medium transition-all',
            viewMode === 'canvas'
              ? 'bg-white text-slate-900 shadow-xs dark:bg-slate-700 dark:text-white'
              : 'text-slate-600 hover:text-slate-900 dark:text-slate-400 dark:hover:text-slate-200',
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
          emotionsForEventBlock={(ebId) =>
            eventBlockEmotionals
              .filter((ebe) => ebe.eventBlockId === ebId)
              .map((ebe) => emotionalById.get(ebe.emotionalRangeId))
              .filter((e): e is (typeof emotionalRanges)[number] => Boolean(e))
          }
        />
      ) : (
        <>
          {/* Motif Constraints */}
          <section className="rounded-xl border border-slate-200 bg-white p-4 shadow-sm dark:border-slate-800 dark:bg-slate-900">
            <div className="mb-2 flex items-start gap-1.5">
              <h3 className="text-sm font-semibold text-slate-900 dark:text-slate-100">
                {t('sp.motifConstraints.title')}
              </h3>
              <InfoTooltip content={t('sp.motifConstraints.desc')} className="mt-0.5" />
            </div>
            {motifConstraintRows.length === 0 ? (
              <p className="text-[11px] text-slate-400">{t('sp.motifs.noConstraints')}</p>
            ) : (
              <ul className="space-y-1.5">
                {motifConstraintRows.map(({ mc, constraint }) => (
                  <li
                    key={mc.id}
                    className="rounded-lg border border-slate-200 bg-slate-50/70 p-2.5 dark:border-slate-800 dark:bg-slate-950/40"
                  >
                    <p className="font-mono text-[11px] text-slate-500 dark:text-slate-400">
                      {constraint!.code}
                    </p>
                    <p className="mt-1 text-xs font-semibold text-slate-900 dark:text-slate-100">
                      <span className="text-slate-400">{t('sp.constraints.require')}:</span>{' '}
                      {constraint!.require}
                    </p>
                    <p className="mt-0.5 text-[11px] text-slate-600 dark:text-slate-400">
                      <span className="text-slate-400">{t('sp.constraints.reason')}:</span>{' '}
                      {constraint!.reason}
                    </p>
                  </li>
                ))}
              </ul>
            )}
          </section>

          {/* Event Blocks */}
          <section>
            <div className="mb-2 flex items-center gap-1.5">
              <h3 className="text-sm font-semibold text-slate-900 dark:text-slate-100">
                {t('sp.eventBlocks.title')}
              </h3>
              <span className="rounded-full bg-slate-100 px-2 py-0.5 text-[10px] font-medium text-slate-700 dark:bg-slate-800 dark:text-slate-300">
                {motifEventBlocks.length}
              </span>
              <InfoTooltip content={t('sp.eventBlocks.desc')} />
            </div>

            {motifEventBlocks.length === 0 ? (
              <div className="rounded-xl border border-dashed border-slate-300 bg-white/60 p-8 text-center text-xs text-slate-400 dark:border-slate-700 dark:bg-slate-900/40">
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
                      className="group rounded-xl border border-slate-200 bg-white shadow-sm dark:border-slate-800 dark:bg-slate-900"
                    >
                      <summary className="flex cursor-pointer list-none items-center justify-between gap-3 px-4 py-3">
                        <div className="min-w-0">
                          <p className="truncate font-mono text-[11px] font-semibold text-slate-800 dark:text-slate-200">
                            {eb.code}
                          </p>
                          <p className="mt-0.5 truncate text-[11px] text-slate-500 dark:text-slate-400">
                            <span className="font-medium text-indigo-600 dark:text-indigo-400">
                              {comp?.name}
                            </span>
                            <span className="mx-1 text-slate-300 dark:text-slate-600">→</span>
                            <span className="font-medium text-emerald-600 dark:text-emerald-400">
                              {st?.name}
                            </span>
                          </p>
                        </div>
                        <div className="flex shrink-0 items-center gap-2 text-[10px] text-slate-500 dark:text-slate-400">
                          <span>
                            {emotions.length} {t('sp.eventBlock.emotionalRanges').toLowerCase()}
                          </span>
                          <span className="text-slate-300 dark:text-slate-600">·</span>
                          <span>
                            {transitions.length}{' '}
                            {t('sp.eventBlock.outgoingTransitions').toLowerCase()}
                          </span>
                          <span className="ml-1 text-slate-400 transition-transform group-open:rotate-180">
                            ▾
                          </span>
                        </div>
                      </summary>

                      <div className="space-y-3 border-t border-slate-100 px-4 pb-4 pt-3 dark:border-slate-800">
                        <p className="text-[10px] italic text-slate-500 dark:text-slate-400">
                          {t('sp.eventBlock.childHint')}
                        </p>

                        {/* Component + State */}
                        <div className="grid grid-cols-1 gap-3 md:grid-cols-2">
                          <div className="rounded-lg border border-indigo-100 bg-indigo-50/50 p-2.5 dark:border-indigo-900/60 dark:bg-indigo-950/30">
                            <p className="text-[10px] font-semibold uppercase tracking-wide text-indigo-700 dark:text-indigo-300">
                              {t('sp.eventBlock.component')}
                            </p>
                            <p className="mt-1 text-xs font-semibold text-slate-900 dark:text-slate-100">
                              {comp?.name}{' '}
                              <span className="font-mono text-[10px] text-slate-400">
                                · {comp?.code}
                              </span>
                            </p>
                            <p className="mt-0.5 text-[11px] text-slate-600 dark:text-slate-400">
                              {comp?.description}
                            </p>
                          </div>
                          <div className="rounded-lg border border-emerald-100 bg-emerald-50/50 p-2.5 dark:border-emerald-900/60 dark:bg-emerald-950/30">
                            <p className="text-[10px] font-semibold uppercase tracking-wide text-emerald-700 dark:text-emerald-300">
                              {t('sp.eventBlock.state')}
                            </p>
                            <p className="mt-1 text-xs font-semibold text-slate-900 dark:text-slate-100">
                              {st?.name}{' '}
                              <span className="font-mono text-[10px] text-slate-400">
                                · {st?.code}
                              </span>
                            </p>
                            <p className="mt-0.5 text-[11px] text-slate-600 dark:text-slate-400">
                              {st?.description}
                            </p>
                          </div>
                        </div>

                        {/* Emotional ranges */}
                        <div>
                          <p className="mb-1.5 flex items-center gap-1 text-[10px] font-semibold uppercase tracking-wide text-slate-500 dark:text-slate-400">
                            {t('sp.eventBlock.emotionalRanges')}
                            <InfoTooltip content="Resolved via event_block_emotionals → emotional_ranges (-N)." />
                          </p>
                          {emotions.length === 0 ? (
                            <p className="text-[11px] text-slate-400">
                              {t('sp.eventBlock.noEmotionals')}
                            </p>
                          ) : (
                            <div className="flex flex-wrap gap-1.5">
                              {emotions.map((e) => (
                                <span
                                  key={e.id}
                                  className="rounded-full bg-amber-100 px-2 py-0.5 text-[11px] font-medium text-amber-800 dark:bg-amber-950/60 dark:text-amber-300"
                                >
                                  {e.name}
                                </span>
                              ))}
                            </div>
                          )}
                        </div>

                        {/* Outgoing transitions */}
                        <div>
                          <p className="mb-1.5 flex items-center gap-1 text-[10px] font-semibold uppercase tracking-wide text-slate-500 dark:text-slate-400">
                            {t('sp.eventBlock.outgoingTransitions')}
                            <InfoTooltip content="Resolved via event_transitions.from_event_block_id (-N)." />
                          </p>
                          {transitions.length === 0 ? (
                            <p className="text-[11px] text-slate-400">
                              {t('sp.eventBlock.noTransitions')}
                            </p>
                          ) : (
                            <ul className="space-y-1">
                              {transitions.map(({ et, target }) => (
                                <li
                                  key={et.id}
                                  className="flex flex-wrap items-center gap-1.5 rounded-lg border border-slate-200 bg-slate-50/70 px-2.5 py-1.5 text-[11px] dark:border-slate-800 dark:bg-slate-950/40"
                                >
                                  <span className="text-slate-400">→</span>
                                  <span className="font-mono font-semibold text-slate-700 dark:text-slate-300">
                                    {target!.code}
                                  </span>
                                  <span className="ml-auto flex items-center gap-1 text-slate-500 dark:text-slate-400">
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
