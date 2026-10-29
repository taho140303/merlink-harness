import type { CharacterRelationship, IntensityLevel, RelationshipStates } from './types';
import { useCharacterStore } from './store';
import { IntensitySlider } from './shared/IntensitySlider';
import { useT } from '@/shared/i18n/useT';
import type { TKey } from '@/shared/i18n/dict';

const roleColor: Record<string, string> = {
  SUPERIOR: 'bg-purple-100 text-purple-700 dark:bg-purple-950/60 dark:text-purple-300',
  EQUAL: 'bg-blue-100 text-blue-700 dark:bg-blue-950/60 dark:text-blue-300',
  INFERIOR: 'bg-emerald-100 text-emerald-700 dark:bg-emerald-950/60 dark:text-emerald-300',
};

const categoryColor: Record<string, string> = {
  FAMILY: 'bg-rose-100 text-rose-700 dark:bg-rose-950/60 dark:text-rose-300',
  ROMANTIC: 'bg-pink-100 text-pink-700 dark:bg-pink-950/60 dark:text-pink-300',
  SOCIAL: 'bg-amber-100 text-amber-700 dark:bg-amber-950/60 dark:text-amber-300',
  PROFESSIONAL: 'bg-surface-muted text-fg',
  ANTAGONISTIC: 'bg-red-100 text-red-700 dark:bg-red-950/60 dark:text-red-300',
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

export function RelationshipCard({ relationship }: { relationship: CharacterRelationship }) {
  const isEditing = useCharacterStore((s) => s.isEditing);
  const patch = useCharacterStore((s) => s.patchCharacter);
  const t = useT();

  const setState = (key: keyof RelationshipStates, level: IntensityLevel) =>
    patch((c) => {
      const rel = c.relationships.find((r) => r.id === relationship.id);
      if (rel) rel.states[key] = level;
    });

  return (
    <div className="rounded-xl border border-line bg-surface-card p-4">
      <div className="mb-3">
        <h4 className="font-semibold text-fg">
          {relationship.targetName}
        </h4>
        <div className="mt-1 flex flex-wrap gap-1.5">
          <span
            className={`rounded-full px-2 py-0.5 text-xs font-medium ${
              roleColor[relationship.role] || 'bg-surface-muted text-fg'
            }`}
          >
            {relationship.role}
          </span>
          <span
            className={`rounded-full px-2 py-0.5 text-xs font-medium ${
              categoryColor[relationship.category] || 'bg-surface-muted text-fg'
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
            onChange={isEditing ? (lvl) => setState(key, lvl) : undefined}
          />
        ))}
      </div>
    </div>
  );
}
