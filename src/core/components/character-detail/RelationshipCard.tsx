import type { IntensityLevel, Relationship, RelationshipStates } from './types';
import { useCharacterDetailStore } from './store';
import { IntensitySlider } from './shared/IntensitySlider';
import { useT } from '../../../shared/i18n/useT';
import type { TKey } from '../../../shared/i18n/dict';

const roleColor: Record<string, string> = {
  Superior: 'bg-purple-100 text-purple-700 dark:bg-purple-950/60 dark:text-purple-300',
  Equal: 'bg-blue-100 text-blue-700 dark:bg-blue-950/60 dark:text-blue-300',
  Inferior: 'bg-emerald-100 text-emerald-700 dark:bg-emerald-950/60 dark:text-emerald-300',
};

const categoryColor: Record<string, string> = {
  Family: 'bg-rose-100 text-rose-700 dark:bg-rose-950/60 dark:text-rose-300',
  Romantic: 'bg-pink-100 text-pink-700 dark:bg-pink-950/60 dark:text-pink-300',
  Social: 'bg-amber-100 text-amber-700 dark:bg-amber-950/60 dark:text-amber-300',
  Professional: 'bg-slate-100 text-slate-700 dark:bg-slate-800 dark:text-slate-300',
  Rival: 'bg-red-100 text-red-700 dark:bg-red-950/60 dark:text-red-300',
};

const stateEntries: Array<[keyof RelationshipStates, TKey]> = [
  ['trust', 'detail.state.trust'],
  ['affection', 'detail.state.affection'],
  ['hostility', 'detail.state.hostility'],
  ['fear', 'detail.state.fear'],
  ['respect', 'detail.state.respect'],
  ['familiarity', 'detail.state.familiarity'],
  ['dependency', 'detail.state.dependency'],
];

export function RelationshipCard({ relationship }: { relationship: Relationship }) {
  const isEditing = useCharacterDetailStore((s) => s.isEditing);
  const patch = useCharacterDetailStore((s) => s.patchCharacter);
  const t = useT();

  const setState = (key: keyof RelationshipStates, level: IntensityLevel) =>
    patch((c) => {
      const rel = c.relationships.find((r) => r.id === relationship.id);
      if (rel) rel.states[key] = level;
    });

  return (
    <div className="rounded-xl border border-slate-200 bg-white p-4 shadow-sm dark:border-slate-800 dark:bg-slate-900">
      <div className="mb-3">
        <h4 className="font-semibold text-slate-900 dark:text-slate-100">
          {relationship.personName}
        </h4>
        <div className="mt-1 flex flex-wrap gap-1.5">
          <span
            className={`rounded-full px-2 py-0.5 text-[10px] font-medium ${
              roleColor[relationship.role] ?? 'bg-slate-100 text-slate-700'
            }`}
          >
            {relationship.role}
          </span>
          <span
            className={`rounded-full px-2 py-0.5 text-[10px] font-medium ${
              categoryColor[relationship.category] ?? 'bg-slate-100 text-slate-700'
            }`}
          >
            {relationship.category}
          </span>
        </div>
      </div>
      <div className="space-y-2">
        {stateEntries.map(([key, labelKey]) => (
          <IntensitySlider
            key={key}
            label={t(labelKey)}
            value={relationship.states[key]}
            onChange={isEditing ? (v) => setState(key, v) : undefined}
          />
        ))}
      </div>
    </div>
  );
}
