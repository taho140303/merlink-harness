import { useState } from 'react';
import { LayoutGrid, Network } from 'lucide-react';
import { cn } from '../../../../shared/utils/cn';
import { useT } from '../../../../shared/i18n/useT';
import { InfoTooltip } from '../shared/InfoTooltip';
import { humanize } from '../shared/humanize';
import {
  components,
  constraints,
  emotionalRanges,
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
import type { MotifCategory } from '../types';

const CATEGORY_COLOR: Record<MotifCategory, string> = {
  character_arc: 'bg-indigo-100 text-indigo-700 dark:bg-indigo-950/60 dark:text-indigo-300',
  relationship: 'bg-rose-100 text-rose-700 dark:bg-rose-950/60 dark:text-rose-300',
  symbolic: 'bg-violet-100 text-violet-700 dark:bg-violet-950/60 dark:text-violet-300',
  plot: 'bg-emerald-100 text-emerald-700 dark:bg-emerald-950/60 dark:text-emerald-300',
};

function SectionTitle({ title, description }: { title: string; description: string }) {
  return (
    <div className="mb-3 flex items-start gap-2">
      <div>
        <h3 className="font-semibold text-slate-900 dark:text-slate-100">{title}</h3>
        <p className="mt-0.5 text-xs text-slate-500 dark:text-slate-400">{description}</p>
      </div>
      <InfoTooltip content={description} className="mt-1" />
    </div>
  );
}

export function MotifsTab() {
  const t = useT();
  const [viewMode, setViewMode] = useState<'default' | 'canvas'>('default');

  const genreById = new Map(genres.map((g) => [g.id, g]));
  const motifById = new Map(motifs.map((m) => [m.id, m]));
  const componentById = new Map(components.map((c) => [c.id, c]));
  const stateById = new Map(states.map((s) => [s.id, s]));
  const stateRuleById = new Map(stateRules.map((r) => [r.id, r]));
  const constraintById = new Map(constraints.map((c) => [c.id, c]));
  const eventBlockById = new Map(eventBlocks.map((e) => [e.id, e]));

  const genresForMotif = (motifId: string) =>
    motifApplicableGenres
      .filter((mg) => mg.motifId === motifId)
      .map((mg) => genreById.get(mg.genreId))
      .filter((g): g is (typeof genres)[number] => Boolean(g));

  return (
    <div className="space-y-8">
      {/* View mode switch */}
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-1 rounded-xl bg-slate-100 p-0.5 dark:bg-slate-800">
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
      </div>

      {viewMode === 'canvas' ? (
        <div className="rounded-2xl border border-dashed border-slate-300 bg-white/60 p-10 text-center dark:border-slate-700 dark:bg-slate-900/40">
          <Network className="mx-auto h-8 w-8 text-slate-300 dark:text-slate-600" />
          <p className="mt-3 text-sm font-medium text-slate-700 dark:text-slate-200">
            {t('sp.motifs.canvasPlaceholder')}
          </p>
          <p className="mt-1 text-xs text-slate-500 dark:text-slate-400">
            {t('sp.motifs.canvasHint')}
          </p>
        </div>
      ) : (
        <>
          {/* Motifs */}
          <section>
            <SectionTitle title={t('sp.motifs.title')} description={t('sp.motifs.desc')} />
            <div className="grid grid-cols-1 gap-3 md:grid-cols-2">
              {motifs.map((mo) => {
                const applicable = genresForMotif(mo.id);
                return (
                  <div
                    key={mo.id}
                    className="rounded-xl border border-slate-200 bg-white p-4 shadow-sm dark:border-slate-800 dark:bg-slate-900"
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

                    <p className="mt-2 text-xs leading-relaxed text-slate-700 dark:text-slate-300">
                      {mo.description}
                    </p>

                    <div className="mt-3 flex flex-wrap gap-1.5 border-t border-slate-100 pt-2.5 text-[10px] dark:border-slate-800">
                      {mo.isSystem ? (
                        <span className="rounded-full bg-slate-100 px-2 py-0.5 font-medium text-slate-700 dark:bg-slate-800 dark:text-slate-300">
                          {t('sp.common.system')}
                        </span>
                      ) : (
                        <span className="rounded-full bg-emerald-100 px-2 py-0.5 font-medium text-emerald-700 dark:bg-emerald-950/60 dark:text-emerald-300">
                          {t('sp.common.custom')}
                        </span>
                      )}
                      {mo.minChapter !== undefined && mo.maxChapter !== undefined && (
                        <span className="rounded-full bg-slate-100 px-2 py-0.5 font-medium text-slate-700 dark:bg-slate-800 dark:text-slate-300">
                          {t('sp.motifs.chapterRange')}: {mo.minChapter}–{mo.maxChapter}
                        </span>
                      )}
                      {applicable.length > 0 ? (
                        applicable.map((g) => (
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
                );
              })}
            </div>
          </section>

          {/* Components & States */}
          <div className="grid grid-cols-1 gap-6 md:grid-cols-2">
            <section>
              <SectionTitle title={t('sp.components.title')} description={t('sp.components.desc')} />
              <ul className="space-y-1.5">
                {components.map((c) => (
                  <li
                    key={c.id}
                    className="rounded-lg border border-slate-200 bg-white p-2.5 dark:border-slate-800 dark:bg-slate-900"
                  >
                    <p className="text-xs font-semibold text-slate-900 dark:text-slate-100">
                      {c.name}{' '}
                      <span className="font-normal text-slate-400">· {c.code}</span>
                    </p>
                    <p className="mt-0.5 text-[11px] text-slate-600 dark:text-slate-400">
                      {c.description}
                    </p>
                  </li>
                ))}
              </ul>
            </section>

            <section>
              <SectionTitle title={t('sp.states.title')} description={t('sp.states.desc')} />
              <ul className="space-y-1.5">
                {states.map((s) => (
                  <li
                    key={s.id}
                    className="rounded-lg border border-slate-200 bg-white p-2.5 dark:border-slate-800 dark:bg-slate-900"
                  >
                    <p className="text-xs font-semibold text-slate-900 dark:text-slate-100">
                      {s.name}{' '}
                      <span className="font-normal text-slate-400">· {s.code}</span>
                    </p>
                    <p className="mt-0.5 text-[11px] text-slate-600 dark:text-slate-400">
                      {s.description}
                    </p>
                  </li>
                ))}
              </ul>
            </section>
          </div>

          {/* State Rules */}
          <section>
            <SectionTitle title={t('sp.stateRules.title')} description={t('sp.stateRules.desc')} />
            <div className="flex flex-wrap gap-1.5">
              {stateRules.map((r) => (
                <span
                  key={r.id}
                  className="rounded-full border border-slate-200 bg-white px-2.5 py-1 text-[11px] font-medium text-slate-700 dark:border-slate-800 dark:bg-slate-900 dark:text-slate-300"
                >
                  {r.name} <span className="text-slate-400">· {r.code}</span>
                </span>
              ))}
            </div>
            <div className="mt-2 flex flex-wrap gap-1.5">
              {stateDownByRules.map((sdr) => {
                const st = stateById.get(sdr.stateId);
                const sr = stateRuleById.get(sdr.stateRuleId);
                return (
                  <span
                    key={sdr.id}
                    className={cn(
                      'rounded-full px-2.5 py-1 text-[11px] font-medium',
                      sdr.isAllowed
                        ? 'bg-emerald-100 text-emerald-700 dark:bg-emerald-950/60 dark:text-emerald-300'
                        : 'bg-rose-100 text-rose-700 dark:bg-rose-950/60 dark:text-rose-300',
                    )}
                  >
                    {st?.name} × {sr?.name} · {sdr.isAllowed ? '✓' : '✗'}
                  </span>
                );
              })}
            </div>
          </section>

          {/* Emotional Ranges */}
          <section>
            <SectionTitle
              title={t('sp.emotionalRanges.title')}
              description={t('sp.emotionalRanges.desc')}
            />
            <div className="flex flex-wrap gap-1.5">
              {emotionalRanges.map((er) => (
                <span
                  key={er.id}
                  className="rounded-full bg-amber-100 px-2.5 py-1 text-[11px] font-medium text-amber-800 dark:bg-amber-950/60 dark:text-amber-300"
                >
                  {er.name}
                </span>
              ))}
            </div>
          </section>

          {/* Event Blocks */}
          <section>
            <SectionTitle title={t('sp.eventBlocks.title')} description={t('sp.eventBlocks.desc')} />
            <div className="space-y-1.5">
              {eventBlocks.map((eb) => {
                const comp = componentById.get(eb.componentId);
                const st = stateById.get(eb.stateId);
                const mo = motifById.get(eb.motifId);
                return (
                  <div
                    key={eb.id}
                    className="flex flex-wrap items-center gap-2 rounded-lg border border-slate-200 bg-white px-3 py-2 text-[11px] dark:border-slate-800 dark:bg-slate-900"
                  >
                    <span className="font-mono font-semibold text-slate-700 dark:text-slate-300">
                      {eb.code}
                    </span>
                    <span className="text-slate-300">|</span>
                    <span className="rounded bg-indigo-100 px-1.5 py-0.5 font-medium text-indigo-700 dark:bg-indigo-950/60 dark:text-indigo-300">
                      {comp?.name}
                    </span>
                    <span className="text-slate-400">→</span>
                    <span className="rounded bg-emerald-100 px-1.5 py-0.5 font-medium text-emerald-700 dark:bg-emerald-950/60 dark:text-emerald-300">
                      {st?.name}
                    </span>
                    <span className="text-slate-300">|</span>
                    <span className="rounded bg-violet-100 px-1.5 py-0.5 font-medium text-violet-700 dark:bg-violet-950/60 dark:text-violet-300">
                      {mo?.name}
                    </span>
                  </div>
                );
              })}
            </div>
          </section>

          {/* Event Transitions */}
          <section>
            <SectionTitle
              title={t('sp.eventTransitions.title')}
              description={t('sp.eventTransitions.desc')}
            />
            <div className="space-y-1.5">
              {eventTransitions.map((et) => {
                const from = eventBlockById.get(et.fromEventBlockId);
                const to = eventBlockById.get(et.toEventBlockId);
                return (
                  <div
                    key={et.id}
                    className="flex flex-wrap items-center gap-2 rounded-lg border border-slate-200 bg-white px-3 py-2 text-[11px] dark:border-slate-800 dark:bg-slate-900"
                  >
                    <span className="font-mono text-slate-700 dark:text-slate-300">
                      {from?.code}
                    </span>
                    <span className="text-slate-400">→</span>
                    <span className="font-mono text-slate-700 dark:text-slate-300">
                      {to?.code}
                    </span>
                    <span className="ml-auto flex items-center gap-1 text-slate-500">
                      {t('sp.eventTransitions.priority')}: {et.priority}
                      <InfoTooltip content={et.name} />
                    </span>
                  </div>
                );
              })}
            </div>
          </section>

          {/* Constraints & Motif Constraints */}
          <div className="grid grid-cols-1 gap-6 md:grid-cols-2">
            <section>
              <SectionTitle
                title={t('sp.constraints.title')}
                description={t('sp.constraints.desc')}
              />
              <ul className="space-y-1.5">
                {constraints.map((c) => (
                  <li
                    key={c.id}
                    className="rounded-lg border border-slate-200 bg-white p-2.5 dark:border-slate-800 dark:bg-slate-900"
                  >
                    <p className="font-mono text-[11px] text-slate-500 dark:text-slate-400">
                      {c.code}
                    </p>
                    <p className="mt-1 text-xs font-semibold text-slate-900 dark:text-slate-100">
                      <span className="text-slate-400">{t('sp.constraints.require')}:</span>{' '}
                      {c.require}
                    </p>
                    <p className="mt-0.5 text-[11px] text-slate-600 dark:text-slate-400">
                      <span className="text-slate-400">{t('sp.constraints.reason')}:</span>{' '}
                      {c.reason}
                    </p>
                  </li>
                ))}
              </ul>
            </section>

            <section>
              <SectionTitle
                title={t('sp.motifConstraints.title')}
                description={t('sp.motifConstraints.desc')}
              />
              <ul className="space-y-1.5">
                {motifConstraints.map((mc) => {
                  const mo = motifById.get(mc.motifId);
                  const cs = constraintById.get(mc.constraintId);
                  return (
                    <li
                      key={mc.id}
                      className="rounded-lg border border-slate-200 bg-white px-3 py-2 text-[11px] dark:border-slate-800 dark:bg-slate-900"
                    >
                      <span className="font-semibold text-slate-800 dark:text-slate-200">
                        {mo?.name}
                      </span>
                      <span className="mx-1 text-slate-400">↔</span>
                      <span className="font-mono text-slate-600 dark:text-slate-400">
                        {cs?.code}
                      </span>
                    </li>
                  );
                })}
              </ul>
            </section>
          </div>
        </>
      )}
    </div>
  );
}
