import type { ReactNode } from 'react';
import { useCharacterDetailStore } from '../store';

function Block({ title, children }: { title: string; children: ReactNode }) {
  return (
    <section className="rounded-xl border border-slate-200 bg-white p-5 shadow-sm dark:border-slate-800 dark:bg-slate-900">
      <h3 className="mb-3 font-semibold text-slate-900 dark:text-slate-100">{title}</h3>
      {children}
    </section>
  );
}

function ProseField({ label, value }: { label: string; value: string }) {
  return (
    <div>
      <p className="mb-1 text-[11px] uppercase tracking-wide text-slate-500 dark:text-slate-400">
        {label}
      </p>
      <p className="whitespace-pre-wrap font-serif text-[15px] leading-7 text-slate-800 dark:text-slate-200">
        {value}
      </p>
    </div>
  );
}

export function PsychologicalTab() {
  const p = useCharacterDetailStore((s) => s.character).psychological;

  return (
    <div className="space-y-4">
      <Block title="Core Trauma & Belief">
        <div className="grid gap-4 md:grid-cols-3">
          <ProseField label="The Wound" value={p.coreTrauma.theWound} />
          <ProseField label="The Lie" value={p.coreTrauma.theLie} />
          <ProseField label="Fundamental Fear" value={p.coreTrauma.fundamentalFear} />
        </div>
      </Block>

      <Block title="Cognitive & Moral Framework">
        <div className="mb-4 grid gap-4 md:grid-cols-2">
          <div>
            <p className="mb-1 text-[11px] uppercase tracking-wide text-slate-500 dark:text-slate-400">
              Moral Alignment
            </p>
            <span className="inline-block rounded-full bg-emerald-100 px-2.5 py-0.5 text-xs font-medium text-emerald-700 dark:bg-emerald-950/60 dark:text-emerald-300">
              {p.cognitiveMoral.moralAlignment}
            </span>
          </div>
          <ProseField label="Cognitive Bias" value={p.cognitiveMoral.cognitiveBias} />
        </div>
        <div className="grid gap-4 md:grid-cols-2">
          <ProseField label="Moral Code" value={p.cognitiveMoral.moralCode} />
          <div className="rounded-lg border border-rose-100 bg-rose-50 p-3 dark:border-rose-900/60 dark:bg-rose-950/30">
            <p className="mb-1 text-[11px] uppercase tracking-wide text-rose-700 dark:text-rose-300">
              Breaking Point
            </p>
            <p className="whitespace-pre-wrap font-serif text-[15px] leading-7 text-rose-900 dark:text-rose-100">
              {p.cognitiveMoral.breakingPoint}
            </p>
          </div>
        </div>
      </Block>

      <Block title="Narrative Engine">
        <div className="grid gap-4 md:grid-cols-3">
          <ProseField label="External Want" value={p.narrativeEngine.externalWant} />
          <ProseField label="Internal Need" value={p.narrativeEngine.internalNeed} />
          <ProseField label="Internal Conflict" value={p.narrativeEngine.internalConflict} />
        </div>
      </Block>

      <Block title="Behavioral Adaptation">
        <div className="space-y-4">
          <div>
            <h4 className="mb-2 text-xs uppercase tracking-wide text-slate-500 dark:text-slate-400">
              Social Mask
            </h4>
            <div className="grid gap-4 md:grid-cols-2">
              <ProseField
                label="Persona Type"
                value={p.behavioralAdaptation.socialMask.personaType}
              />
              <ProseField
                label="Verbal Style"
                value={p.behavioralAdaptation.socialMask.verbalStyle}
              />
            </div>
          </div>
          <div>
            <h4 className="mb-2 text-xs uppercase tracking-wide text-slate-500 dark:text-slate-400">
              Defense Mechanisms
            </h4>
            <div className="grid gap-4 md:grid-cols-3">
              <ProseField
                label="Primary Reaction"
                value={p.behavioralAdaptation.defenseMechanisms.primaryReaction}
              />
              <ProseField
                label="Stress Trigger"
                value={p.behavioralAdaptation.defenseMechanisms.stressTrigger}
              />
              <ProseField
                label="Coping Strategy"
                value={p.behavioralAdaptation.defenseMechanisms.copingStrategy}
              />
            </div>
          </div>
        </div>
      </Block>
    </div>
  );
}
