import { useState } from 'react';
import { cn } from '../../../../shared/utils/cn';
import { useT } from '../../../../shared/i18n/useT';
import { InfoTooltip } from '../shared/InfoTooltip';
import { humanize } from '../shared/humanize';
import {
  narrativeDirectives,
  rhetoricalDevices,
  stylePresetFocusItems,
  stylePresetNarratives,
  stylePresetRhetoricalDevices,
  stylePresetVocabItems,
  stylePresets,
} from '../mockData';
import type { NarrativeWeight, TonePace } from '../types';

const WEIGHT_PCT: Record<NarrativeWeight, number> = { NONE: 0, LOW: 33, MEDIUM: 66, HIGH: 100 };

const PACE_COLOR: Record<TonePace, string> = {
  VERY_LOW: 'bg-slate-200 text-slate-700 dark:bg-slate-800 dark:text-slate-300',
  LOW: 'bg-sky-100 text-sky-700 dark:bg-sky-950/60 dark:text-sky-300',
  MODERATE: 'bg-amber-100 text-amber-700 dark:bg-amber-950/60 dark:text-amber-300',
  FAST: 'bg-orange-100 text-orange-700 dark:bg-orange-950/60 dark:text-orange-300',
  VERY_FAST: 'bg-rose-100 text-rose-700 dark:bg-rose-950/60 dark:text-rose-300',
};

// Extract unique categories for the filter
const CATEGORIES = Array.from(new Set(stylePresets.map(p => p.category))).sort();

function WeightMeter({ label, weight }: { label: string; weight: NarrativeWeight }) {
  return (
    <div>
      <div className="mb-1 flex items-center justify-between text-[10px] text-slate-500 dark:text-slate-400">
        <span>{label}</span>
        <span className="font-semibold text-slate-700 dark:text-slate-300">{weight}</span>
      </div>
      <div className="h-1.5 w-full overflow-hidden rounded-full bg-slate-100 dark:bg-slate-800">
        <div
          className="h-full rounded-full bg-indigo-500"
          style={{ width: `${WEIGHT_PCT[weight]}%` }}
        />
      </div>
    </div>
  );
}

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

export function StylePresetsTab() {
  const t = useT();
  const [categoryFilter, setCategoryFilter] = useState<string>('ALL');

  const filteredPresets = categoryFilter === 'ALL'
    ? stylePresets
    : stylePresets.filter(p => p.category === categoryFilter);

  const rdevById = new Map(rhetoricalDevices.map((r) => [r.id, r]));
  const ndById = new Map(narrativeDirectives.map((n) => [n.id, n]));

  return (
    <div className="space-y-6">
      <div className="mb-3 flex flex-wrap items-start justify-between gap-3">
        <SectionTitle title={t('sp.style.title')} description={t('sp.style.desc')} />
        
        {/* Category Filter */}
        <div className="flex items-center gap-1 rounded-xl bg-slate-100 p-0.5 dark:bg-slate-800 overflow-x-auto max-w-full">
          <button
            type="button"
            onClick={() => setCategoryFilter('ALL')}
            className={cn(
              'flex items-center gap-1 rounded-lg px-2.5 py-1.5 text-xs font-medium transition-all whitespace-nowrap',
              categoryFilter === 'ALL'
                ? 'bg-white text-slate-900 shadow-xs dark:bg-slate-700 dark:text-white'
                : 'text-slate-600 hover:text-slate-900 dark:text-slate-400 dark:hover:text-slate-200'
            )}
          >
            All
          </button>
          {CATEGORIES.map(cat => (
            <button
              key={cat}
              type="button"
              onClick={() => setCategoryFilter(cat)}
              className={cn(
                'flex items-center gap-1 rounded-lg px-2.5 py-1.5 text-xs font-medium transition-all whitespace-nowrap',
                categoryFilter === cat
                  ? 'bg-white text-slate-900 shadow-xs dark:bg-slate-700 dark:text-white'
                  : 'text-slate-600 hover:text-slate-900 dark:text-slate-400 dark:hover:text-slate-200'
              )}
            >
              {humanize(cat)}
            </button>
          ))}
        </div>
      </div>

      {filteredPresets.length === 0 ? (
        <div className="rounded-xl border border-dashed border-slate-300 bg-white/60 p-10 text-center text-xs text-slate-400 dark:border-slate-700 dark:bg-slate-900/40">
          Không có style preset nào thuộc thể loại này.
        </div>
      ) : (
        filteredPresets.map((preset) => {
          const rds = stylePresetRhetoricalDevices.filter((r) => r.presetId === preset.id);
          const vocab = stylePresetVocabItems.filter((v) => v.presetId === preset.id);
          const focus = stylePresetFocusItems.filter((f) => f.presetId === preset.id);
          const narratives = stylePresetNarratives.filter((n) => n.stylePresetId === preset.id);

          return (
            <details
              key={preset.id}
              className="group rounded-xl border border-slate-200 bg-white shadow-sm dark:border-slate-800 dark:bg-slate-900"
            >
              <summary className="flex cursor-pointer list-none items-center justify-between gap-2 px-4 py-3">
                <div className="min-w-0">
                  <div className="flex flex-wrap items-center gap-2">
                    <h4 className="font-semibold text-slate-900 dark:text-slate-100">
                      {preset.code}
                    </h4>
                    <span className="rounded-full bg-slate-100 px-2 py-0.5 text-[10px] font-medium text-slate-700 dark:bg-slate-800 dark:text-slate-300">
                      {preset.category}
                    </span>
                    <span
                      className={cn(
                        'rounded-full px-2 py-0.5 text-[10px] font-medium',
                        PACE_COLOR[preset.tonePace],
                      )}
                    >
                      {humanize(preset.tonePace)}
                    </span>
                    {preset.isSystem && (
                      <span className="rounded-full bg-slate-100 px-2 py-0.5 text-[10px] font-medium text-slate-700 dark:bg-slate-800 dark:text-slate-300">
                        {t('sp.common.system')}
                      </span>
                    )}
                  </div>
                  <p className="mt-1 text-[11px] text-slate-500 dark:text-slate-400">
                    {t('sp.style.toneEmotion')}: {preset.toneEmotion}
                  </p>
                </div>
                <span className="text-xs text-slate-400 transition-transform group-open:rotate-180">
                  ▾
                </span>
              </summary>

              <div className="space-y-5 border-t border-slate-100 px-4 pb-4 pt-4 dark:border-slate-800">
                {/* Sensory */}
                <div>
                  <div className="mb-2 flex items-center gap-1.5">
                    <h5 className="text-[11px] font-semibold uppercase tracking-wide text-slate-600 dark:text-slate-300">
                      {t('sp.style.sensory')}
                    </h5>
                    <InfoTooltip content="Relative weight of each sensory channel in the generated prose." />
                  </div>
                  <div className="grid grid-cols-2 gap-3 md:grid-cols-3">
                    <WeightMeter label={t('sp.style.visual')} weight={preset.visual} />
                    <WeightMeter label={t('sp.style.auditory')} weight={preset.auditory} />
                    <WeightMeter label={t('sp.style.tactile')} weight={preset.tactile} />
                    <WeightMeter label={t('sp.style.olfactory')} weight={preset.olfactory} />
                    <WeightMeter label={t('sp.style.gustatory')} weight={preset.gustatory} />
                    <WeightMeter label={t('sp.style.languageAbstract')} weight={preset.languageAbstract} />
                  </div>
                </div>

                {/* Syntax */}
                <div>
                  <div className="mb-2 flex items-center gap-1.5">
                    <h5 className="text-[11px] font-semibold uppercase tracking-wide text-slate-600 dark:text-slate-300">
                      {t('sp.style.syntax')}
                    </h5>
                    <InfoTooltip content="Sentence shape and rhythm constraints for this preset." />
                  </div>
                  <div className="grid grid-cols-2 gap-3 md:grid-cols-3">
                    <div>
                      <p className="text-[10px] text-slate-500 dark:text-slate-400">
                        {t('sp.style.sentenceLength')}
                      </p>
                      <p className="text-xs font-semibold text-slate-800 dark:text-slate-200">
                        {humanize(preset.syntaxSentence)}
                      </p>
                    </div>
                    <div>
                      <p className="text-[10px] text-slate-500 dark:text-slate-400">
                        {t('sp.style.sentenceStructure')}
                      </p>
                      <p className="text-xs font-semibold text-slate-800 dark:text-slate-200">
                        {humanize(preset.syntaxStructure)}
                      </p>
                    </div>
                    <WeightMeter label={t('sp.style.sentenceVariation')} weight={preset.syntaxVariation} />
                  </div>
                </div>

                {/* Rhetorical devices */}
                {rds.length > 0 && (
                  <div>
                    <div className="mb-2 flex items-center gap-1.5">
                      <h5 className="text-[11px] font-semibold uppercase tracking-wide text-slate-600 dark:text-slate-300">
                        {t('sp.style.rhetorical')}
                      </h5>
                      <InfoTooltip content="Which rhetorical devices to lean into, use lightly, or avoid." />
                    </div>
                    <div className="space-y-1.5">
                      {rds.map((r) => {
                        const dev = rdevById.get(r.rhetoricalDeviceId);
                        const usageColor =
                          r.usageType === 'primary'
                            ? 'bg-emerald-100 text-emerald-700 dark:bg-emerald-950/60 dark:text-emerald-300'
                            : r.usageType === 'secondary'
                              ? 'bg-sky-100 text-sky-700 dark:bg-sky-950/60 dark:text-sky-300'
                              : 'bg-rose-100 text-rose-700 dark:bg-rose-950/60 dark:text-rose-300';
                        return (
                          <div
                            key={r.id}
                            className="flex flex-wrap items-start gap-2 rounded-lg border border-slate-200 bg-slate-50/70 px-2.5 py-2 text-[11px] dark:border-slate-800 dark:bg-slate-950/40"
                          >
                            <span className={cn('rounded-full px-2 py-0.5 font-medium', usageColor)}>
                              {t(`sp.style.usage.${r.usageType}`)}
                            </span>
                            <span className="font-semibold text-slate-800 dark:text-slate-200">
                              {humanize(dev?.code ?? '')}
                            </span>
                            {r.customInstruction && (
                              <span className="text-slate-500 dark:text-slate-400">
                                — {r.customInstruction}
                              </span>
                            )}
                          </div>
                        );
                      })}
                    </div>
                  </div>
                )}

                {/* Vocab + Focus */}
                <div className="grid grid-cols-1 gap-4 md:grid-cols-2">
                  <div>
                    <div className="mb-2 flex items-center gap-1.5">
                      <h5 className="text-[11px] font-semibold uppercase tracking-wide text-slate-600 dark:text-slate-300">
                        {t('sp.style.vocab')}
                      </h5>
                      <InfoTooltip content="Suggested vocabulary tokens to bias the model's output." />
                    </div>
                    <div className="flex flex-wrap gap-1.5">
                      {vocab.map((v) => (
                        <span
                          key={v.id}
                          className="rounded-full bg-slate-100 px-2 py-0.5 text-[11px] text-slate-700 dark:bg-slate-800 dark:text-slate-300"
                        >
                          {v.content}
                        </span>
                      ))}
                    </div>
                  </div>
                  <div>
                    <div className="mb-2 flex items-center gap-1.5">
                      <h5 className="text-[11px] font-semibold uppercase tracking-wide text-slate-600 dark:text-slate-300">
                        {t('sp.style.focus')}
                      </h5>
                      <InfoTooltip content="Thematic / sensory focus items to prioritise." />
                    </div>
                    <div className="flex flex-wrap gap-1.5">
                      {focus.map((f) => (
                        <span
                          key={f.id}
                          className="rounded-full bg-indigo-100 px-2 py-0.5 text-[11px] text-indigo-700 dark:bg-indigo-950/60 dark:text-indigo-300"
                        >
                          {f.content}
                        </span>
                      ))}
                    </div>
                  </div>
                </div>

                {/* Narrative directives */}
                {narratives.length > 0 && (
                  <div>
                    <div className="mb-2 flex items-center gap-1.5">
                      <h5 className="text-[11px] font-semibold uppercase tracking-wide text-slate-600 dark:text-slate-300">
                        {t('sp.style.directives')}
                      </h5>
                      <InfoTooltip content="Directives applied with the given weight." />
                    </div>
                    <div className="space-y-1.5">
                      {narratives.map((sn) => {
                        const nd = ndById.get(sn.narrativeDirectiveId);
                        return (
                          <div
                            key={sn.id}
                            className="flex items-start justify-between gap-3 rounded-lg border border-slate-200 bg-white px-2.5 py-2 dark:border-slate-800 dark:bg-slate-900"
                          >
                            <div className="min-w-0">
                              <p className="font-mono text-[11px] font-semibold text-slate-800 dark:text-slate-200">
                                {nd?.code}
                              </p>
                              <p className="mt-0.5 text-[11px] text-slate-500 dark:text-slate-400">
                                {nd?.description}
                              </p>
                            </div>
                            <span className="shrink-0 rounded-full bg-slate-100 px-2 py-0.5 text-[10px] font-medium text-slate-700 dark:bg-slate-800 dark:text-slate-300">
                              {sn.weight}
                            </span>
                          </div>
                        );
                      })}
                    </div>
                  </div>
                )}
              </div>
            </details>
          );
        })
      )}
    </div>
  );
}
