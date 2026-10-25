import { useState } from 'react';
import { Plus, Trash2 } from 'lucide-react';
import { useCharacterDetailStore } from '../store';
import { MemoryCard } from '../MemoryCard';
import type { Competence, KnowledgeCertainty, LifeSubTab } from '../types';

const certaintyColor: Record<KnowledgeCertainty, string> = {
  Low: 'bg-rose-100 text-rose-700 dark:bg-rose-950/60 dark:text-rose-300',
  Medium: 'bg-amber-100 text-amber-700 dark:bg-amber-950/60 dark:text-amber-300',
  High: 'bg-emerald-100 text-emerald-700 dark:bg-emerald-950/60 dark:text-emerald-300',
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
  const isEditing = useCharacterDetailStore((s) => s.isEditing);
  const addKnowledge = useCharacterDetailStore((s) => s.addKnowledge);
  const updateKnowledgePath = useCharacterDetailStore((s) => s.updateKnowledgePath);
  const removeKnowledge = useCharacterDetailStore((s) => s.removeKnowledge);

  return (
    <div className="flex flex-col gap-4 md:flex-row">
      <nav className="flex shrink-0 gap-1 overflow-x-auto md:w-48 md:flex-col md:overflow-visible">
        {subTabs.map((t) => (
          <button
            key={t.id}
            type="button"
            onClick={() => setActive(t.id)}
            className={`shrink-0 rounded-lg px-3 py-2 text-left text-sm font-medium transition-colors ${
              active === t.id
                ? 'bg-indigo-50 text-indigo-700 dark:bg-indigo-950/60 dark:text-indigo-300'
                : 'text-slate-600 hover:bg-slate-100 dark:text-slate-400 dark:hover:bg-slate-800'
            }`}
          >
            {t.label}
          </button>
        ))}
      </nav>

      <div className="min-w-0 flex-1">
        {active === 'knowledge' && (
          <div className="space-y-2">
            {character.knowledge.map((k) => (
              <div
                key={k.id}
                className="flex items-center gap-3 rounded-xl border border-slate-200 bg-white px-4 py-3 shadow-sm dark:border-slate-800 dark:bg-slate-900"
              >
                <div className="min-w-0 flex-1">
                  <span className="text-[10px] uppercase tracking-wide text-slate-500 dark:text-slate-400">
                    {k.type}
                  </span>
                  {isEditing ? (
                    <input
                      className="mt-1 w-full rounded-lg border border-slate-200 bg-slate-50 px-2 py-1 text-sm text-slate-800 dark:border-slate-700 dark:bg-slate-950 dark:text-slate-200"
                      value={k.path}
                      onChange={(e) => updateKnowledgePath(k.id, e.target.value)}
                    />
                  ) : (
                    <p className="truncate text-sm text-slate-900 dark:text-slate-100">
                      {k.path}
                    </p>
                  )}
                </div>
                <span
                  className={`shrink-0 rounded-full px-2 py-0.5 text-[10px] font-medium ${certaintyColor[k.certainty]}`}
                >
                  {k.certainty}
                </span>
                {isEditing && (
                  <button
                    type="button"
                    onClick={() => removeKnowledge(k.id)}
                    className="shrink-0 text-rose-500 hover:text-rose-600"
                    title="Remove"
                  >
                    <Trash2 className="h-4 w-4" />
                  </button>
                )}
              </div>
            ))}
            {isEditing && (
              <button
                type="button"
                onClick={() =>
                  addKnowledge({ type: 'Initial', path: 'New source', certainty: 'Medium' })
                }
                className="flex items-center gap-1.5 text-sm text-indigo-600 hover:underline dark:text-indigo-400"
              >
                <Plus className="h-3.5 w-3.5" />
                Add Knowledge
              </button>
            )}
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

function CompetenceCard({ competence }: { competence: Competence }) {
  const [expanded, setExpanded] = useState(false);
  return (
    <div className="rounded-xl border border-slate-200 bg-white shadow-sm dark:border-slate-800 dark:bg-slate-900">
      <div className="px-4 py-3">
        <div className="flex items-start justify-between gap-2">
          <div>
            <h4 className="font-semibold text-slate-900 dark:text-slate-100">{competence.name}</h4>
            <p className="text-xs text-slate-500 dark:text-slate-400">
              {competence.domain} · {competence.level}
            </p>
          </div>
          <span
            className={`rounded-full px-2 py-0.5 text-[10px] font-medium ${certaintyColor[competence.reliability]}`}
          >
            {competence.reliability}
          </span>
        </div>
        <p className="mt-2 text-sm text-slate-700 dark:text-slate-300">
          {competence.description}
        </p>
        <p className="mt-1 text-xs italic text-slate-500 dark:text-slate-400">
          {competence.experience}
        </p>
        <button
          type="button"
          onClick={() => setExpanded(!expanded)}
          className="mt-2 text-xs text-indigo-600 hover:underline dark:text-indigo-400"
        >
          {expanded ? 'Hide details' : 'Show capabilities & limitations'}
        </button>
      </div>
      {expanded && (
        <div className="grid grid-cols-1 gap-4 border-t border-slate-100 px-4 pb-4 pt-3 dark:border-slate-800 md:grid-cols-2">
          <div>
            <p className="mb-1.5 text-[11px] uppercase tracking-wide text-emerald-700 dark:text-emerald-400">
              Capabilities
            </p>
            <ul className="list-disc space-y-1 pl-4 text-sm text-slate-800 dark:text-slate-200">
              {competence.capabilities.map((c, i) => (
                <li key={i}>{c}</li>
              ))}
            </ul>
          </div>
          <div>
            <p className="mb-1.5 text-[11px] uppercase tracking-wide text-rose-700 dark:text-rose-400">
              Limitations
            </p>
            <ul className="list-disc space-y-1 pl-4 text-sm text-slate-800 dark:text-slate-200">
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
