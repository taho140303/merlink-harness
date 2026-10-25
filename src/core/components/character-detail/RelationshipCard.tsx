import type { Relationship, IntensityLevel } from './types';
import { ProgressBar } from './shared/ProgressBar';

const roleColor: Record<string, string> = {
  Superior: 'bg-purple-100 text-purple-700',
  Equal: 'bg-blue-100 text-blue-700',
  Inferior: 'bg-emerald-100 text-emerald-700',
};

const categoryColor: Record<string, string> = {
  Family: 'bg-rose-100 text-rose-700',
  Romantic: 'bg-pink-100 text-pink-700',
  Social: 'bg-amber-100 text-amber-700',
  Professional: 'bg-slate-100 text-slate-700',
  Rival: 'bg-red-100 text-red-700',
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

interface RelationshipCardProps {
  relationship: Relationship;
}

export function RelationshipCard({ relationship }: RelationshipCardProps) {
  return (
    <div className="rounded-xl border border-gray-200 bg-white shadow-sm p-4">
      <div className="flex items-start justify-between gap-2 mb-3">
        <div>
          <h4 className="font-semibold text-gray-900">{relationship.personName}</h4>
          <div className="flex flex-wrap gap-1.5 mt-1">
            <span
              className={`px-2 py-0.5 text-[10px] rounded-full font-medium ${roleColor[relationship.role] ?? 'bg-gray-100 text-gray-700'}`}
            >
              {relationship.role}
            </span>
            <span
              className={`px-2 py-0.5 text-[10px] rounded-full font-medium ${categoryColor[relationship.category] ?? 'bg-gray-100 text-gray-700'}`}
            >
              {relationship.category}
            </span>
          </div>
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
