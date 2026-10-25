import type { ReactNode } from 'react';
import { useCharacterDetailStore } from '../store';

function Block({
  title,
  children,
}: {
  title: string;
  children: ReactNode;
}) {
  return (
    <section className="rounded-xl border border-gray-200 bg-white shadow-sm p-5">
      <h3 className="font-semibold text-gray-900 mb-3">{title}</h3>
      {children}
    </section>
  );
}

function ProseField({ label, value }: { label: string; value: string }) {
  return (
    <div>
      <p className="text-[11px] uppercase tracking-wide text-gray-500 mb-1">{label}</p>
      <p className="font-serif text-[15px] leading-7 text-gray-800 whitespace-pre-wrap">
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
        <div className="grid gap-4 md:grid-cols-2 mb-4">
          <div>
            <p className="text-[11px] uppercase tracking-wide text-gray-500 mb-1">
              Moral Alignment
            </p>
            <span className="inline-block px-2.5 py-0.5 text-xs rounded-full bg-emerald-100 text-emerald-700 font-medium">
              {p.cognitiveMoral.moralAlignment}
            </span>
          </div>
          <ProseField label="Cognitive Bias" value={p.cognitiveMoral.cognitiveBias} />
        </div>
        <div className="grid gap-4 md:grid-cols-2">
          <ProseField label="Moral Code" value={p.cognitiveMoral.moralCode} />
          <div className="rounded-lg bg-red-50 border border-red-100 p-3">
            <p className="text-[11px] uppercase tracking-wide text-red-700 mb-1">
              Breaking Point
            </p>
            <p className="font-serif text-[15px] leading-7 text-red-900 whitespace-pre-wrap">
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
            <h4 className="text-xs uppercase tracking-wide text-gray-500 mb-2">
              Social Mask
            </h4>
            <div className="grid gap-4 md:grid-cols-2">
              <ProseField label="Persona Type" value={p.behavioralAdaptation.socialMask.personaType} />
              <ProseField label="Verbal Style" value={p.behavioralAdaptation.socialMask.verbalStyle} />
            </div>
          </div>
          <div>
            <h4 className="text-xs uppercase tracking-wide text-gray-500 mb-2">
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
