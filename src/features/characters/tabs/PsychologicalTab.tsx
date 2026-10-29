import type { ReactNode } from 'react';
import { useCharacterStore } from '../store';
import { useT } from '@/shared/i18n/useT';
import type { PsychologicalProfile } from '../types';

function Block({ title, children }: { title: string; children: ReactNode }) {
  return (
    <section className="rounded-xl border border-line bg-surface-card p-5   ">
      <h3 className="mb-3 font-semibold text-fg">{title}</h3>
      {children}
    </section>
  );
}

function ProseField({
  label,
  value,
  onChange,
}: {
  label: string;
  value: string;
  onChange?: (v: string) => void;
}) {
  if (onChange) {
    return (
      <div>
        <p className="mb-1 text-xs uppercase tracking-wide text-fg-muted">
          {label}
        </p>
        <textarea
          rows={4}
          value={value}
          onChange={(e) => onChange(e.target.value)}
          className="w-full resize-y rounded-lg tier-input focus-ring px-3 py-2 font-serif text-sm leading-7 text-fg focus-ring   "
        />
      </div>
    );
  }
  return (
    <div>
      <p className="mb-1 text-xs uppercase tracking-wide text-fg-muted">
        {label}
      </p>
      <p className="whitespace-pre-wrap font-serif text-sm leading-7 text-fg ">
        {value}
      </p>
    </div>
  );
}

export function PsychologicalTab() {
  const t = useT();
  const character = useCharacterStore((s) => s.character);
  const p = character.psychological;
  const isEditing = useCharacterStore((s) => s.isEditing);
  const patch = useCharacterStore((s) => s.patchCharacter);

  if (!p) {
    return (
      <div className="rounded-xl border border-line bg-surface-card p-8 text-center text-sm text-fg-muted">
        Chưa có dữ liệu tâm lý cho nhân vật này.
      </div>
    );
  }

  const upd = (fn: (draft: PsychologicalProfile) => void) =>
    patch((c) => {
      if (c.psychological) fn(c.psychological);
    });

  return (
    <div className="space-y-4">
      <Block title={t('detail.psych.coreTrauma')}>
        <div className="grid gap-4 md:grid-cols-3">
          <ProseField
            label={t('detail.psych.theWound')}
            value={p.coreTrauma.theWound}
            onChange={isEditing ? (v) => upd((d) => (d.coreTrauma.theWound = v)) : undefined}
          />
          <ProseField
            label={t('detail.psych.theLie')}
            value={p.coreTrauma.theLie}
            onChange={isEditing ? (v) => upd((d) => (d.coreTrauma.theLie = v)) : undefined}
          />
          <ProseField
            label={t('detail.psych.fundamentalFear')}
            value={p.coreTrauma.fundamentalFear}
            onChange={isEditing ? (v) => upd((d) => (d.coreTrauma.fundamentalFear = v)) : undefined}
          />
        </div>
      </Block>

      <Block title={t('detail.psych.cognitiveMoral')}>
        <div className="mb-4 grid gap-4 md:grid-cols-2">
          <div>
            <p className="mb-1 text-xs uppercase tracking-wide text-fg-muted">
              {t('detail.psych.moralAlignment')}
            </p>
            {isEditing ? (
              <input
                value={p.cognitiveMoral.moralAlignment}
                onChange={(e) => upd((d) => (d.cognitiveMoral.moralAlignment = e.target.value))}
                className="w-full rounded-lg tier-input focus-ring px-3 py-1.5 text-xs   "
              />
            ) : (
              <span className="inline-block rounded-full bg-emerald-100 px-2.5 py-0.5 text-xs font-medium text-emerald-700 dark:bg-emerald-950/60 dark:text-emerald-300">
                {p.cognitiveMoral.moralAlignment}
              </span>
            )}
          </div>
          <ProseField
            label={t('detail.psych.cognitiveBias')}
            value={p.cognitiveMoral.cognitiveBias}
            onChange={
              isEditing ? (v) => upd((d) => (d.cognitiveMoral.cognitiveBias = v)) : undefined
            }
          />
        </div>
        <div className="grid gap-4 md:grid-cols-2">
          <ProseField
            label={t('detail.psych.moralCode')}
            value={p.cognitiveMoral.moralCode}
            onChange={isEditing ? (v) => upd((d) => (d.cognitiveMoral.moralCode = v)) : undefined}
          />
          <div className="rounded-lg border border-rose-100 bg-rose-50 p-3 dark:border-rose-900/60 dark:bg-rose-950/30">
            <p className="mb-1 text-xs uppercase tracking-wide text-rose-700 dark:text-rose-300">
              {t('detail.psych.breakingPoint')}
            </p>
            {isEditing ? (
              <textarea
                rows={4}
                value={p.cognitiveMoral.breakingPoint}
                onChange={(e) => upd((d) => (d.cognitiveMoral.breakingPoint = e.target.value))}
                className="w-full resize-y rounded-lg border border-rose-200 bg-surface-card px-2 py-1.5 font-serif text-sm leading-7 text-rose-900 focus-ring focus:ring-2 focus:ring-rose-400 dark:border-rose-900  dark:text-rose-100"
              />
            ) : (
              <p className="whitespace-pre-wrap font-serif text-sm leading-7 text-rose-900 dark:text-rose-100">
                {p.cognitiveMoral.breakingPoint}
              </p>
            )}
          </div>
        </div>
      </Block>

      <Block title={t('detail.psych.narrativeEngine')}>
        <div className="grid gap-4 md:grid-cols-3">
          <ProseField
            label={t('detail.psych.externalWant')}
            value={p.narrativeEngine.externalWant}
            onChange={
              isEditing ? (v) => upd((d) => (d.narrativeEngine.externalWant = v)) : undefined
            }
          />
          <ProseField
            label={t('detail.psych.internalNeed')}
            value={p.narrativeEngine.internalNeed}
            onChange={
              isEditing ? (v) => upd((d) => (d.narrativeEngine.internalNeed = v)) : undefined
            }
          />
          <ProseField
            label={t('detail.psych.internalConflict')}
            value={p.narrativeEngine.internalConflict}
            onChange={
              isEditing ? (v) => upd((d) => (d.narrativeEngine.internalConflict = v)) : undefined
            }
          />
        </div>
      </Block>

      <Block title={t('detail.psych.behavioralAdaptation')}>
        <div className="space-y-4">
          <div>
            <h4 className="mb-2 text-xs uppercase tracking-wide text-fg-muted">
              {t('detail.psych.socialMask')}
            </h4>
            <div className="grid gap-4 md:grid-cols-2">
              <ProseField
                label={t('detail.psych.personaType')}
                value={p.behavioralAdaptation.socialMask.personaType}
                onChange={
                  isEditing
                    ? (v) => upd((d) => (d.behavioralAdaptation.socialMask.personaType = v))
                    : undefined
                }
              />
              <ProseField
                label={t('detail.psych.verbalStyle')}
                value={p.behavioralAdaptation.socialMask.verbalStyle}
                onChange={
                  isEditing
                    ? (v) => upd((d) => (d.behavioralAdaptation.socialMask.verbalStyle = v))
                    : undefined
                }
              />
            </div>
          </div>
          <div>
            <h4 className="mb-2 text-xs uppercase tracking-wide text-fg-muted">
              {t('detail.psych.defenseMechanisms')}
            </h4>
            <div className="grid gap-4 md:grid-cols-3">
              <ProseField
                label={t('detail.psych.primaryReaction')}
                value={p.behavioralAdaptation.defenseMechanisms.primaryReaction}
                onChange={
                  isEditing
                    ? (v) =>
                        upd((d) => (d.behavioralAdaptation.defenseMechanisms.primaryReaction = v))
                    : undefined
                }
              />
              <ProseField
                label={t('detail.psych.stressTrigger')}
                value={p.behavioralAdaptation.defenseMechanisms.stressTrigger}
                onChange={
                  isEditing
                    ? (v) =>
                        upd((d) => (d.behavioralAdaptation.defenseMechanisms.stressTrigger = v))
                    : undefined
                }
              />
              <ProseField
                label={t('detail.psych.copingStrategy')}
                value={p.behavioralAdaptation.defenseMechanisms.copingStrategy}
                onChange={
                  isEditing
                    ? (v) =>
                        upd((d) => (d.behavioralAdaptation.defenseMechanisms.copingStrategy = v))
                    : undefined
                }
              />
            </div>
          </div>
        </div>
      </Block>
    </div>
  );
}
