import { useState } from 'react';
import { cn } from '@/shared/utils/cn';
import { useT } from '@/shared/i18n/useT';
import { InfoTooltip } from '../shared/InfoTooltip';
import { humanize } from '../shared/humanize';
import { useDbEnumLabel } from '@/shared/i18n/dbEnums';
import { useSystemPromptCatalog } from '../catalog';
import type { NarrativeWeight, TonePace } from '../types';
import { sortRhetoricalDevices, STYLE_PRESET_CATEGORY_KEYS } from '../stylePresetUtils';
import { Dialog, DialogContent, DialogDescription, DialogTitle } from '@/shared/components/ui/dialog';
import { Info } from 'lucide-react';

const WEIGHT_PCT: Record<NarrativeWeight, number> = { NONE: 0, LOW: 33, MEDIUM: 66, HIGH: 100 };
const PACE_COLOR: Record<TonePace, string> = {
  VERY_LOW: 'bg-surface-muted text-fg-muted  ',
  LOW: 'bg-sky-100 text-sky-700 dark:bg-sky-950/60 dark:text-sky-300',
  MODERATE: 'bg-amber-100 text-amber-700 dark:bg-amber-950/60 dark:text-amber-300',
  FAST: 'bg-orange-100 text-orange-700 dark:bg-orange-950/60 dark:text-orange-300',
  VERY_FAST: 'bg-rose-100 text-rose-700 dark:bg-rose-950/60 dark:text-rose-300',
};

function WeightMeter({ label, weight }: { label: string; weight: NarrativeWeight }) {
  const enumLabel = useDbEnumLabel();
  return (
    <div>
      <div className="mb-1 flex items-center justify-between text-xs text-fg-muted">
        <span>{label}</span>
        <span className="font-semibold text-fg-muted">{enumLabel('NarrativeWeight', weight)}</span>
      </div>
      <div className="h-1.5 w-full overflow-hidden rounded-full bg-surface-muted ">
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
        <h3 className="font-semibold text-fg">{title}</h3>
        <p className="mt-0.5 text-xs text-fg-muted">{description}</p>
      </div>
      <InfoTooltip content={description} className="mt-1" />
    </div>
  );
}

export function StylePresetsTab() {
  const enumLabel = useDbEnumLabel();
  const { narrativeDirectives, rhetoricalDevices, stylePresetFocusItems, stylePresetNarratives, stylePresetRhetoricalDevices, stylePresetVocabItems, stylePresets } = useSystemPromptCatalog();
  const t = useT();
  const [categoryFilter, setCategoryFilter] = useState<string>('ALL');
  const [expandedPresetId, setExpandedPresetId] = useState<string | null>(null);
  const categories = Array.from(new Set(stylePresets.map((preset) => preset.category))).sort();
  const categoryLabel = (category: string) => {
    const key = STYLE_PRESET_CATEGORY_KEYS[category];
    return key ? t(key) : t('sp.common.category');
  };

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
        <div className="flex items-center gap-1 rounded-xl bg-surface-muted p-0.5  overflow-x-auto max-w-full">
          <button
            type="button"
            onClick={() => setCategoryFilter('ALL')}
            className={cn(
              'flex items-center gap-1 rounded-lg px-2.5 py-1.5 text-xs font-medium transition-colors whitespace-nowrap',
              categoryFilter === 'ALL'
                ? 'bg-surface-card text-fg   '
                : 'text-fg-muted hover:text-fg -subtle '
            )}
          >
            {t('sp.style.allCategories')}
          </button>
          {categories.map(cat => (
            <button
              key={cat}
              type="button"
              onClick={() => setCategoryFilter(cat)}
              className={cn(
                'flex items-center gap-1 rounded-lg px-2.5 py-1.5 text-xs font-medium transition-colors whitespace-nowrap',
                categoryFilter === cat
                  ? 'bg-surface-card text-fg   '
                  : 'text-fg-muted hover:text-fg -subtle '
              )}
            >
              {categoryLabel(cat)}
            </button>
          ))}
        </div>
      </div>

      {filteredPresets.length === 0 ? (
        <div className="rounded-xl border border-dashed border-line-strong bg-surface-card/60 p-10 text-center text-xs text-fg-subtle -strong ">
          Không có style preset nào thuộc thể loại này.
        </div>
      ) : (
        filteredPresets.map((preset) => {
          const expanded = expandedPresetId === preset.id;
          const rds = expanded ? sortRhetoricalDevices(stylePresetRhetoricalDevices.filter((r) => r.presetId === preset.id)) : [];
          const vocab = expanded ? stylePresetVocabItems.filter((v) => v.presetId === preset.id) : [];
          const focus = expanded ? stylePresetFocusItems.filter((f) => f.presetId === preset.id) : [];
          const narratives = expanded ? stylePresetNarratives.filter((n) => n.stylePresetId === preset.id) : [];

          return (
            <div key={preset.id} className="tier-card rounded-card">
              <div className="flex items-center justify-between gap-2 px-4 py-3">
                <div className="min-w-0">
                  <div className="flex flex-wrap items-center gap-2">
                    <h4 className="font-semibold text-fg">
                      {humanize(preset.code.split('/').pop() ?? preset.code)}
                    </h4>
                    <span className="rounded-full bg-surface-muted px-2 py-0.5 text-xs font-medium text-fg-muted  ">
                      {categoryLabel(preset.category)}
                    </span>
                    <span
                      className={cn(
                        'rounded-full px-2 py-0.5 text-xs font-medium',
                        PACE_COLOR[preset.tonePace],
                      )}
                    >
                      {enumLabel('TonePace', preset.tonePace)}
                    </span>
                    {preset.isSystem && (
                      <span className="rounded-full bg-surface-muted px-2 py-0.5 text-xs font-medium text-fg-muted  ">
                        {t('sp.common.system')}
                      </span>
                    )}
                  </div>
                  <p className="mt-1 text-xs text-fg-muted">
                    {t('sp.style.toneEmotion')}: {preset.toneEmotion}
                  </p>
                </div>
                <button type="button" onClick={() => setExpandedPresetId(preset.id)}
                  title={t('sp.style.details')} aria-label={`${t('sp.style.details')}: ${humanize(preset.code.split('/').pop() ?? preset.code)}`}
                  className="focus-ring shrink-0 rounded-lg p-2 text-fg-muted hover:bg-surface-muted hover:text-fg">
                  <Info aria-hidden="true" className="h-4 w-4" />
                </button>
              </div>

              {expanded && <Dialog open onOpenChange={(open) => { if (!open) setExpandedPresetId(null); }}>
                <DialogContent className="max-h-[85vh] max-w-3xl overflow-y-auto p-0">
                  <div className="border-b border-line px-5 py-4 pr-12">
                    <DialogTitle className="text-base font-semibold text-fg">{humanize(preset.code.split('/').pop() ?? preset.code)}</DialogTitle>
                    <DialogDescription className="mt-1 text-xs text-fg-muted">{categoryLabel(preset.category)} · {preset.toneEmotion}</DialogDescription>
                  </div>
              <div className="space-y-5 px-5 pb-5 pt-4">
                {/* Sensory */}
                <div>
                  <div className="mb-2 flex items-center gap-1.5">
                    <h5 className="text-xs font-semibold uppercase tracking-wide text-fg-muted ">
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
                    <h5 className="text-xs font-semibold uppercase tracking-wide text-fg-muted ">
                      {t('sp.style.syntax')}
                    </h5>
                    <InfoTooltip content="Sentence shape and rhythm constraints for this preset." />
                  </div>
                  <div className="grid grid-cols-2 gap-3 md:grid-cols-3">
                    <div>
                      <p className="text-xs text-fg-muted">
                        {t('sp.style.sentenceLength')}
                      </p>
                      <p className="text-xs font-semibold text-fg">
                        {enumLabel('SentenceLength', preset.syntaxSentence)}
                      </p>
                    </div>
                    <div>
                      <p className="text-xs text-fg-muted">
                        {t('sp.style.sentenceStructure')}
                      </p>
                      <p className="text-xs font-semibold text-fg">
                        {enumLabel('SentenceStructure', preset.syntaxStructure)}
                      </p>
                    </div>
                    <WeightMeter label={t('sp.style.sentenceVariation')} weight={preset.syntaxVariation} />
                  </div>
                </div>

                {/* Rhetorical devices */}
                {rds.length > 0 && (
                  <div>
                    <div className="mb-2 flex items-center gap-1.5">
                      <h5 className="text-xs font-semibold uppercase tracking-wide text-fg-muted ">
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
                            className="flex flex-wrap items-start gap-2 rounded-lg border border-line bg-surface-muted px-2.5 py-2 text-xs  "
                          >
                            <span className={cn('rounded-full px-2 py-0.5 font-medium', usageColor)}>
                              {enumLabel('RhetoricalUsage', r.usageType)}
                            </span>
                            <span className="font-semibold text-fg">
                              {dev?.description || humanize(dev?.code ?? '')}
                            </span>
                            {r.customInstruction && (
                              <span className="text-fg-muted">
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
                      <h5 className="text-xs font-semibold uppercase tracking-wide text-fg-muted ">
                        {t('sp.style.vocab')}
                      </h5>
                      <InfoTooltip content="Suggested vocabulary tokens to bias the model's output." />
                    </div>
                    <div className="flex flex-wrap gap-1.5">
                      {vocab.map((v) => (
                        <span
                          key={v.id}
                          className="rounded-full bg-surface-muted px-2 py-0.5 text-xs text-fg-muted  "
                        >
                          {v.content}
                        </span>
                      ))}
                    </div>
                  </div>
                  <div>
                    <div className="mb-2 flex items-center gap-1.5">
                      <h5 className="text-xs font-semibold uppercase tracking-wide text-fg-muted ">
                        {t('sp.style.focus')}
                      </h5>
                      <InfoTooltip content="Thematic / sensory focus items to prioritise." />
                    </div>
                    <div className="flex flex-wrap gap-1.5">
                      {focus.map((f) => (
                        <span
                          key={f.id}
                          className="rounded-full bg-indigo-100 px-2 py-0.5 text-xs text-indigo-700 dark:bg-indigo-950/60 dark:text-indigo-300"
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
                      <h5 className="text-xs font-semibold uppercase tracking-wide text-fg-muted ">
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
                            className="flex items-start justify-between gap-3 tier-card rounded-lg px-2.5 py-2"
                          >
                            <div className="min-w-0">
                              <p className="font-mono text-xs font-semibold text-fg">
                                {nd?.code}
                              </p>
                              <p className="mt-0.5 text-xs text-fg-muted">
                                {nd?.description}
                              </p>
                            </div>
                            <span className="shrink-0 rounded-full bg-surface-muted px-2 py-0.5 text-xs font-medium text-fg-muted  ">
                              {enumLabel('NarrativeWeight', sn.weight)}
                            </span>
                          </div>
                        );
                      })}
                    </div>
                  </div>
                )}
              </div>
                </DialogContent>
              </Dialog>}
            </div>
          );
        })
      )}
    </div>
  );
}
