import { useState } from 'react';
import { Plus, Trash2 } from 'lucide-react';
import { useCharacterDetailStore } from '../store';
import { MemoryCard } from '../MemoryCard';
import { useT } from '../../../../shared/i18n/useT';
import type { Competence, KnowledgeCertainty, KnowledgeType, LifeSubTab } from '../types';
import { CERTAINTY_OPTIONS, KNOWLEDGE_TYPE_OPTIONS } from '../types';

const certaintyColor: Record<KnowledgeCertainty, string> = {
  Low: 'bg-rose-100 text-rose-700 dark:bg-rose-950/60 dark:text-rose-300',
  Medium: 'bg-amber-100 text-amber-700 dark:bg-amber-950/60 dark:text-amber-300',
  High: 'bg-emerald-100 text-emerald-700 dark:bg-emerald-950/60 dark:text-emerald-300',
};

export function LifeExperienceTab() {
  const t = useT();
  const active = useCharacterDetailStore((s) => s.activeLifeTab);
  const setActive = useCharacterDetailStore((s) => s.setActiveLifeTab);
  const character = useCharacterDetailStore((s) => s.character);
  const isEditing = useCharacterDetailStore((s) => s.isEditing);
  const addKnowledge = useCharacterDetailStore((s) => s.addKnowledge);
  const removeKnowledge = useCharacterDetailStore((s) => s.removeKnowledge);
  const patch = useCharacterDetailStore((s) => s.patchCharacter);

  const subTabs: Array<{ id: LifeSubTab; label: string }> = [
    { id: 'knowledge', label: t('detail.life.knowledge') },
    { id: 'competence', label: t('detail.life.competence') },
    { id: 'memory', label: t('detail.life.memory') },
  ];

  return (
    <div className="flex flex-col gap-4 md:flex-row">
      <nav className="flex shrink-0 gap-1 overflow-x-auto md:w-48 md:flex-col md:overflow-visible">
        {subTabs.map((st) => (
          <button
            key={st.id}
            type="button"
            onClick={() => setActive(st.id)}
            className={`shrink-0 rounded-lg px-3 py-2 text-left text-sm font-medium transition-colors ${
              active === st.id
                ? 'bg-indigo-50 text-indigo-700 dark:bg-indigo-950/60 dark:text-indigo-300'
                : 'text-fg-muted hover:bg-surface-muted  '
            }`}
          >
            {st.label}
          </button>
        ))}
      </nav>

      <div className="min-w-0 flex-1">
        {active === 'knowledge' && (
          <div className="space-y-2">
            {character.knowledge.map((k) => (
              <div
                key={k.id}
                className="flex items-start gap-3 rounded-xl border border-line bg-surface-card px-4 py-3   "
              >
                <div className="min-w-0 flex-1 space-y-1.5">
                  {isEditing ? (
                    <>
                      <select
                        value={k.type}
                        onChange={(e) =>
                          patch((c) => {
                            const x = c.knowledge.find((y) => y.id === k.id);
                            if (x) x.type = e.target.value as KnowledgeType;
                          })
                        }
                        className="rounded-md tier-input focus-ring px-2 py-1 text-xs uppercase tracking-wide   "
                      >
                        {KNOWLEDGE_TYPE_OPTIONS.map((o) => (
                          <option key={o} value={o}>
                            {o}
                          </option>
                        ))}
                      </select>
                      <input
                        value={k.path}
                        onChange={(e) =>
                          patch((c) => {
                            const x = c.knowledge.find((y) => y.id === k.id);
                            if (x) x.path = e.target.value;
                          })
                        }
                        className="mt-1 w-full rounded-lg tier-input focus-ring px-2 py-1 text-sm   "
                      />
                    </>
                  ) : (
                    <>
                      <span className="text-xs uppercase tracking-wide text-fg-muted">
                        {k.type}
                      </span>
                      <p className="truncate text-sm text-fg">
                        {k.path}
                      </p>
                    </>
                  )}
                </div>
                {isEditing ? (
                  <select
                    value={k.certainty}
                    onChange={(e) =>
                      patch((c) => {
                        const x = c.knowledge.find((y) => y.id === k.id);
                        if (x) x.certainty = e.target.value as KnowledgeCertainty;
                      })
                    }
                    className="shrink-0 rounded-md tier-input focus-ring px-2 py-1 text-xs font-medium   "
                  >
                    {CERTAINTY_OPTIONS.map((o) => (
                      <option key={o} value={o}>
                        {o}
                      </option>
                    ))}
                  </select>
                ) : (
                  <span
                    className={`shrink-0 rounded-full px-2 py-0.5 text-xs font-medium ${certaintyColor[k.certainty]}`}
                  >
                    {k.certainty}
                  </span>
                )}
                {isEditing && (
                  <button
                    type="button"
                    onClick={() => removeKnowledge(k.id)}
                    className="shrink-0 text-rose-500 hover:text-rose-600"
                    title={t('common.remove')}
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
                  addKnowledge({
                    type: 'Initial',
                    path: t('detail.life.newSource'),
                    certainty: 'Medium',
                  })
                }
                className="flex items-center gap-1.5 text-sm text-indigo-600 hover:underline dark:text-indigo-400"
              >
                <Plus className="h-3.5 w-3.5" />
                {t('detail.life.addKnowledge')}
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
  const t = useT();
  const [expanded, setExpanded] = useState(false);
  const isEditing = useCharacterDetailStore((s) => s.isEditing);
  const patch = useCharacterDetailStore((s) => s.patchCharacter);

  const update = (fn: (c: Competence) => void) =>
    patch((c) => {
      const x = c.competences.find((y) => y.id === competence.id);
      if (x) fn(x);
    });

  const inputCls =
    'mt-1 w-full rounded-md tier-input focus-ring px-2 py-1 text-xs text-fg focus-ring   ';

  return (
    <div className="rounded-xl border border-line bg-surface-card   ">
      <div className="px-4 py-3">
        <div className="flex items-start justify-between gap-2">
          <div className="min-w-0 flex-1">
            {isEditing ? (
              <>
                <input
                  value={competence.name}
                  onChange={(e) => update((c) => (c.name = e.target.value))}
                  className={inputCls}
                />
                <div className="mt-1 grid grid-cols-2 gap-2">
                  <input
                    value={competence.domain}
                    onChange={(e) => update((c) => (c.domain = e.target.value))}
                    className={inputCls}
                    placeholder="Domain"
                  />
                  <input
                    value={competence.level}
                    onChange={(e) => update((c) => (c.level = e.target.value))}
                    className={inputCls}
                    placeholder="Level"
                  />
                </div>
              </>
            ) : (
              <>
                <h4 className="font-semibold text-fg">
                  {competence.name}
                </h4>
                <p className="text-xs text-fg-muted">
                  {competence.domain} · {competence.level}
                </p>
              </>
            )}
          </div>
          {isEditing ? (
            <select
              value={competence.reliability}
              onChange={(e) =>
                update((c) => (c.reliability = e.target.value as KnowledgeCertainty))
              }
              className="shrink-0 rounded-md tier-input focus-ring px-2 py-1 text-xs font-medium   "
            >
              {CERTAINTY_OPTIONS.map((o) => (
                <option key={o} value={o}>
                  {o}
                </option>
              ))}
            </select>
          ) : (
            <span
              className={`shrink-0 rounded-full px-2 py-0.5 text-xs font-medium ${certaintyColor[competence.reliability]}`}
            >
              {competence.reliability}
            </span>
          )}
        </div>

        {isEditing ? (
          <>
            <textarea
              rows={2}
              value={competence.description}
              onChange={(e) => update((c) => (c.description = e.target.value))}
              className={`${inputCls} mt-2`}
              placeholder="Description"
            />
            <input
              value={competence.experience}
              onChange={(e) => update((c) => (c.experience = e.target.value))}
              className={inputCls}
              placeholder="Experience"
            />
          </>
        ) : (
          <>
            <p className="mt-2 text-sm text-fg-muted">
              {competence.description}
            </p>
            <p className="mt-1 text-xs italic text-fg-muted">
              {competence.experience}
            </p>
          </>
        )}

        <button
          type="button"
          onClick={() => setExpanded(!expanded)}
          className="mt-2 text-xs text-indigo-600 hover:underline dark:text-indigo-400"
        >
          {expanded ? t('detail.life.hideDetails') : t('detail.life.showDetails')}
        </button>
      </div>
      {expanded && (
        <div className="grid grid-cols-1 gap-4 border-t border-line px-4 pb-4 pt-3  md:grid-cols-2">
          <div>
            <p className="mb-1.5 text-xs uppercase tracking-wide text-emerald-700 dark:text-emerald-400">
              {t('detail.life.capabilities')}
            </p>
            {isEditing ? (
              <EditableList
                items={competence.capabilities}
                addLabel={t('detail.life.addItem')}
                newValue={t('detail.life.newCapability')}
                onChange={(i, v) => update((c) => (c.capabilities[i] = v))}
                onAdd={() => update((c) => c.capabilities.push(t('detail.life.newCapability')))}
                onRemove={(i) => update((c) => c.capabilities.splice(i, 1))}
              />
            ) : (
              <ul className="list-disc space-y-1 pl-4 text-sm text-fg ">
                {competence.capabilities.map((c, i) => (
                  <li key={i}>{c}</li>
                ))}
              </ul>
            )}
          </div>
          <div>
            <p className="mb-1.5 text-xs uppercase tracking-wide text-rose-700 dark:text-rose-400">
              {t('detail.life.limitations')}
            </p>
            {isEditing ? (
              <EditableList
                items={competence.limitations}
                addLabel={t('detail.life.addItem')}
                newValue={t('detail.life.newLimitation')}
                onChange={(i, v) => update((c) => (c.limitations[i] = v))}
                onAdd={() => update((c) => c.limitations.push(t('detail.life.newLimitation')))}
                onRemove={(i) => update((c) => c.limitations.splice(i, 1))}
              />
            ) : (
              <ul className="list-disc space-y-1 pl-4 text-sm text-fg ">
                {competence.limitations.map((c, i) => (
                  <li key={i}>{c}</li>
                ))}
              </ul>
            )}
          </div>
        </div>
      )}
    </div>
  );
}

function EditableList({
  items,
  addLabel,
  newValue,
  onChange,
  onAdd,
  onRemove,
}: {
  items: string[];
  addLabel: string;
  newValue: string;
  onChange: (i: number, v: string) => void;
  onAdd: () => void;
  onRemove: (i: number) => void;
}) {
  void newValue;
  return (
    <div className="space-y-1.5">
      {items.map((item, i) => (
        <div key={i} className="flex items-center gap-1.5">
          <input
            value={item}
            onChange={(e) => onChange(i, e.target.value)}
            className="flex-1 rounded-md tier-input focus-ring px-2 py-1 text-xs   "
          />
          <button type="button" onClick={() => onRemove(i)} className="text-rose-500">
            <Trash2 className="h-3.5 w-3.5" />
          </button>
        </div>
      ))}
      <button
        type="button"
        onClick={onAdd}
        className="flex items-center gap-1 text-xs text-indigo-600 hover:underline dark:text-indigo-400"
      >
        <Plus className="h-3 w-3" /> {addLabel}
      </button>
    </div>
  );
}
