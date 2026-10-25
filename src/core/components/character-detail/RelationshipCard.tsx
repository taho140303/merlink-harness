import type { IntensityLevel, Relationship } from './types';
import { ProgressBar } from './shared/ProgressBar';

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

const stateLabels: Array<[keyof Relationship['states'], string]> = [
  ['trust', 'Trust'],
  ['affection', 'Affection'],
  ['hostility', 'Hostility'],
  ['fear', 'Fear'],
  ['respect', 'Respect'],
  ['familiarity', 'Familiarity'],
  ['dependency', 'Dependency'],
];

export function RelationshipCard({ relationship }: { relationship: Relationship }) {
  return (
    <div className="rounded-xl border border-slate-200 bg-white p-4 shadow-sm dark:border-slate-800 dark:bg-slate-900">
      <div className="mb-3">
        <h4 className="font-semibold text-slate-900 dark:text-slate-100">
          {relationship.personName}
        </h4>
        <div className="mt-1 flex flex-wrap gap-1.5">
          <span
            className={`rounded-full px-2 py-0.5 text-[10px] font-medium ${
              roleColor[relationship.role] ?? 'bg-slate-100 text-slate-700 dark:bg-slate-800 dark:text-slate-300'
            }`}
          >
            {relationship.role}
          </span>
          <span
            className={`rounded-full px-2 py-0.5 text-[10px] font-medium ${
              categoryColor[relationship.category] ?? 'bg-slate-100 text-slate-700 dark:bg-slate-800 dark:text-slate-300'
            }`}
          >
            {relationship.category}
          </span>
        </div>
      </div>
      <div className="space-y-2">
        {stateLabels.map(([key, label]) => (
          <ProgressBar
            key={key}
            label={label}
            level={relationship.states[key] as IntensityLevel}
          />
        ))}
      </div>
    </div>
  );
}
