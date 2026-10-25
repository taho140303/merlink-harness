import { useState } from 'react';
import { useCharacterDetailStore } from '../store';
import { MemoryCard } from '../MemoryCard';
import type { KnowledgeCertainty, LifeSubTab } from '../types';

const certaintyColor: Record<KnowledgeCertainty, string> = {
  Low: 'bg-red-100 text-red-700',
  Medium: 'bg-amber-100 text-amber-700',
  High: 'bg-emerald-100 text-emerald-700',
};

const subTabs: Array<{ id: LifeSubTab; label: string }> = [
  { id: 'knowledge', label: 'Knowledge' },
  { id: 'competence', label: 'Competence' },
  { id: 'memory', label: 'Memory' },
];

export function LifeExperienceTab() {
  const active = useCharacterDetailStore((s) => s.activeLifeTab);
  const setActive = useCharacterDetailStore((s) => s.setActiveLifeTab);
  const character = useCharacterDetailStore((s) => s.character);

  return (
    <div className="flex flex-col md:flex-row gap-4">
      <nav className="md:w-48 shrink-0 flex md:flex-col gap-1 overflow-x-auto md:overflow-visible">
        {subTabs.map((t) => (
          <button
            key={t.id}
            type="button"
            onClick={() => setActive(t.id)}
            className={`shrink-0 text-left px-3 py-2 rounded-lg text-sm font-medium transition-colors ${
              active === t.id
                ? 'bg-indigo-50 text-indigo-700'
                : 'text-gray-600 hover:bg-gray-100'
            }`}
          >
            {t.label}
          </button>
        ))}
      </nav>

      <div className="flex-1 min-w-0">
        {active === 'knowledge' && (
          <div className="space-y-2">
            {character.knowledge.map((k) => (
              <div
                key={k.id}
                className="rounded-xl border border-gray-200 bg-white shadow-sm px-4 py-3 flex items-center justify-between gap-3"
              >
                <div className="min-w-0">
                  <span className="text-[10px] uppercase tracking-wide text-gray-500">
                    {k.type}
                  </span>
                  <p className="text-sm text-gray-900 truncate">{k.path}</p>
                </div>
                <span
                  className={`px-2 py-0.5 text-[10px] rounded-full font-medium shrink-0 ${certaintyColor[k.certainty]}`}
                >
                  {k.certainty}
                </span>
              </div>
            ))}
          </div>
        )}

        {active === 'competence' && (
          <div className="space-y-3">
            {character.competences.map((c) => (
              <CompetenceCard key={c.id} competence={c} />
            ))}
          </div>
        )}

        {active === 'memory' && (
          <div className="space-y-3">
            {character.memories.map((m) => (
              <MemoryCard key={m.id} memory={m} />
            ))}
          </div>
        )}
      </div>
    </div>
  );
}

function CompetenceCard({ competence }: { competence: import('../types').Competence }) {
  const [expanded, setExpanded] = useState(false);
  return (
    <div className="rounded-xl border border-gray-200 bg-white shadow-sm">
      <div className="px-4 py-3">
        <div className="flex items-start justify-between gap-2">
          <div>
            <h4 className="font-semibold text-gray-900">{competence.name}</h4>
            <p className="text-xs text-gray-500">
              {competence.domain} · {competence.level}
            </p>
          </div>
          <span
            className={`px-2 py-0.5 text-[10px] rounded-full font-medium ${certaintyColor[competence.reliability]}`}
          >
            {competence.reliability}
          </span>
        </div>
        <p className="text-sm text-gray-700 mt-2">{competence.description}</p>
        <p className="text-xs text-gray-500 mt-1 italic">{competence.experience}</p>
        <button
          type="button"
          onClick={() => setExpanded(!expanded)}
          className="mt-2 text-xs text-indigo-600 hover:underline"
        >
          {expanded ? 'Hide details' : 'Show capabilities & limitations'}
        </button>
      </div>
      {expanded && (
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4 px-4 pb-4 border-t border-gray-100 pt-3">
          <div>
            <p className="text-[11px] uppercase tracking-wide text-emerald-700 mb-1.5">
              Capabilities
            </p>
            <ul className="space-y-1 text-sm text-gray-800 list-disc pl-4">
              {competence.capabilities.map((c, i) => (
                <li key={i}>{c}</li>
              ))}
            </ul>
          </div>
          <div>
            <p className="text-[11px] uppercase tracking-wide text-rose-700 mb-1.5">
              Limitations
            </p>
            <ul className="space-y-1 text-sm text-gray-800 list-disc pl-4">
              {competence.limitations.map((c, i) => (
                <li key={i}>{c}</li>
              ))}
            </ul>
          </div>
        </div>
      )}
    </div>
  );
}
