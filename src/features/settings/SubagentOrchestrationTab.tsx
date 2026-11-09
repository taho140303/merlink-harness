import { useEffect } from 'react';
import { Minus, Plus } from 'lucide-react';
import { Button } from '@/shared/components/ui/button';
import { Select } from '@/shared/components/ui/select';
import { useT } from '@/shared/i18n/useT';
import type { TKey } from '@/shared/i18n/dict';
import { useModelCatalogStore } from './modelCatalogStore';
import type { CatalogProvider } from './modelCatalogStore';
import {
  DEFAULT_SUBAGENT_CONFIG, NUMERIC_LIMITS, toConfig, toDraft,
  validateSubagentDraft,
  type NumericSetting, type SubagentConfig, type SubagentDraft, type ValidationError,
} from './subagentConfig';

type Row = { key: NumericSetting; label: TKey; description: TKey };
type Section = { title: TKey; rows: Row[] };

const SECTIONS: Section[] = [
  { title: 'settings.subagent.parallel', rows: [
    { key: 'maxSubagents', label: 'settings.subagent.maxSubagents', description: 'settings.subagent.maxSubagentsDesc' },
    { key: 'rejectedThreshold', label: 'settings.subagent.rejectedThreshold', description: 'settings.subagent.rejectedThresholdDesc' },
  ] },
  { title: 'settings.subagent.outline', rows: [
    { key: 'outlineCapacity', label: 'settings.subagent.outlineCapacity', description: 'settings.subagent.outlineCapacityDesc' },
    { key: 'outlineLookback', label: 'settings.subagent.outlineLookback', description: 'settings.subagent.outlineLookbackDesc' },
  ] },
  { title: 'settings.subagent.context', rows: [
    { key: 'contextMargin', label: 'settings.subagent.contextMargin', description: 'settings.subagent.contextMarginDesc' },
  ] },
  { title: 'settings.subagent.units', rows: [
    { key: 'chaptersPerSubagent', label: 'settings.subagent.chaptersPerSubagent', description: 'settings.subagent.chaptersPerSubagentDesc' },
    { key: 'loreRecordsPerSubagent', label: 'settings.subagent.loreRecordsPerSubagent', description: 'settings.subagent.loreRecordsPerSubagentDesc' },
    { key: 'charactersPerSubagent', label: 'settings.subagent.charactersPerSubagent', description: 'settings.subagent.charactersPerSubagentDesc' },
    { key: 'relationshipPairsPerSubagent', label: 'settings.subagent.relationshipPairsPerSubagent', description: 'settings.subagent.relationshipPairsPerSubagentDesc' },
  ] },
];

interface Props {
  draft: SubagentDraft;
  saved: SubagentConfig;
  onDraftChange: (draft: SubagentDraft) => void;
  onSave: (config: SubagentConfig) => void;
}

export function getSubagentModelOptions(providers: CatalogProvider[]) {
  return providers.flatMap((provider) => provider.models.map((model) => ({
    value: `${provider.id}/${model.id}`,
    label: `${provider.name} · ${model.name || model.id}`,
  })));
}

export function SubagentOrchestrationTab({ draft, saved, onDraftChange, onSave }: Props) {
  const t = useT();
  const providers = useModelCatalogStore((state) => state.providers);
  const hydrateCatalog = useModelCatalogStore((state) => state.hydrate);
  useEffect(() => { hydrateCatalog(); }, [hydrateCatalog]);
  const modelOptions = getSubagentModelOptions(providers);
  const selectedModel = draft.model === 'chat' || modelOptions.some((option) => option.value === draft.model)
    ? draft.model : 'chat';
  const errors = validateSubagentDraft(draft);
  const isDirty = Object.entries(saved).some(([key, value]) =>
    (key === 'model' ? selectedModel : draft[key as keyof SubagentDraft]) !== String(value));
  const setField = (key: keyof SubagentDraft, value: string) => onDraftChange({ ...draft, [key]: value });
  const errorText = (error: ValidationError, key: NumericSetting) => {
    if (error === 'minimum') return t('settings.subagent.error.minimum', { min: NUMERIC_LIMITS[key].min });
    if (error === 'maximum') return t('settings.subagent.error.maximum', { max: NUMERIC_LIMITS[key].max ?? 0 });
    return t(`settings.subagent.error.${error}`);
  };

  return (
    <div className="space-y-6 pb-2">
      <header>
        <h3 className="text-base font-semibold text-fg">{t('settings.subagent.title')}</h3>
        <p className="mt-1 text-xs leading-relaxed text-fg-muted">{t('settings.subagent.subtitle')}</p>
      </header>

      {SECTIONS.map((section) => (
        <section key={section.title} aria-label={t(section.title)} className="space-y-2">
          <h4 className="text-xs font-semibold uppercase tracking-wider text-fg-muted">{t(section.title)}</h4>
          <div className="divide-y divide-line overflow-hidden rounded-xl border border-line bg-surface-card">
            {section.rows.map(({ key, label, description }) => {
              const { min, max } = NUMERIC_LIMITS[key];
              const numeric = Number(draft[key]);
              const validInteger = /^-?\d+$/.test(draft[key]) && Number.isSafeInteger(numeric);
              const inputId = `subagent-${key}`;
              const error = errors[key];
              return (
                <div key={key} className="px-3 py-3 sm:px-4">
                  <div className="flex flex-wrap items-center justify-between gap-x-4 gap-y-2">
                    <div className="min-w-[170px] flex-1">
                      <label htmlFor={inputId} className="block text-sm font-medium text-fg">{t(label)}</label>
                      <p id={`${inputId}-description`} className="mt-0.5 text-xs leading-relaxed text-fg-muted">{t(description)}</p>
                    </div>
                    <div role="group" aria-label={t(label)} className="flex h-9 shrink-0 items-stretch overflow-hidden rounded-xl border border-line bg-surface-input">
                      <button type="button" onClick={() => setField(key, String(numeric - 1))}
                        disabled={!validInteger || numeric <= min}
                        aria-label={t('settings.subagent.decrease', { label: t(label) })}
                        className="focus-ring flex w-9 items-center justify-center text-fg-muted hover:bg-surface-muted hover:text-fg disabled:cursor-not-allowed disabled:opacity-40">
                        <Minus aria-hidden="true" className="h-3.5 w-3.5" />
                      </button>
                      <input id={inputId} type="number" inputMode="numeric" step={1} min={min} max={max}
                        value={draft[key]} onChange={(event) => setField(key, event.target.value)}
                        aria-invalid={error ? true : undefined}
                        aria-describedby={`${inputId}-description${error ? ` ${inputId}-error` : ''}`}
                        className="focus-ring w-14 border-x border-line bg-surface-card px-1 text-center text-sm font-medium text-fg [appearance:textfield] [&::-webkit-inner-spin-button]:appearance-none [&::-webkit-outer-spin-button]:appearance-none" />
                      <button type="button" onClick={() => setField(key, String(numeric + 1))}
                        disabled={!validInteger || (max !== undefined && numeric >= max)}
                        aria-label={t('settings.subagent.increase', { label: t(label) })}
                        className="focus-ring flex w-9 items-center justify-center text-fg-muted hover:bg-surface-muted hover:text-fg disabled:cursor-not-allowed disabled:opacity-40">
                        <Plus aria-hidden="true" className="h-3.5 w-3.5" />
                      </button>
                    </div>
                  </div>
                  {error && <p id={`${inputId}-error`} role="alert" className="mt-2 text-xs text-rose-500">{errorText(error, key)}</p>}
                </div>
              );
            })}
          </div>
        </section>
      ))}

      <section aria-label={t('settings.subagent.model')} className="space-y-2">
        <h4 className="text-xs font-semibold uppercase tracking-wider text-fg-muted">{t('settings.subagent.model')}</h4>
        <div className="flex flex-wrap items-center justify-between gap-3 rounded-xl border border-line bg-surface-card px-3 py-3 sm:px-4">
          <div className="min-w-[170px] flex-1">
            <label htmlFor="subagent-model" className="block text-sm font-medium text-fg">{t('settings.subagent.modelLabel')}</label>
            <p id="subagent-model-description" className="mt-0.5 text-xs leading-relaxed text-fg-muted">{t('settings.subagent.modelDesc')}</p>
          </div>
          <Select id="subagent-model" value={selectedModel} onChange={(event) => setField('model', event.target.value)}
            aria-describedby="subagent-model-description" fieldClassName="w-full min-w-0 sm:w-48" className="text-xs">
            <option value="chat">{t('settings.subagent.useChatModel')}</option>
            {modelOptions.map((option) => <option key={option.value} value={option.value}>{option.label}</option>)}
          </Select>
        </div>
      </section>

      <footer className="flex flex-wrap items-center justify-end gap-2 border-t border-line pt-4">
        <Button size="sm" variant="secondary" onClick={() => onDraftChange(toDraft(DEFAULT_SUBAGENT_CONFIG))}>
          {t('settings.subagent.restoreDefaults')}
        </Button>
        <Button size="sm" variant="primary" disabled={!isDirty || Object.keys(errors).length > 0}
          onClick={() => onSave({ ...toConfig(draft), model: selectedModel })}>{t('settings.subagent.save')}</Button>
      </footer>
    </div>
  );
}
