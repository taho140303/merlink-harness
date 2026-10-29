import { useState } from 'react';
import { Plus, Trash2 } from 'lucide-react';
import { useCharacterStore } from '../store';
import { MemoryCard } from '../MemoryCard';
import { useT } from '@/shared/i18n/useT';
import type { CharacterCompetence, LifeSubTab } from '../types';

export function LifeExperienceTab() {
  const t = useT();
  const active = useCharacterStore((s) => s.activeLifeTab);
  const setActive = useCharacterStore((s) => s.setActiveLifeTab);
  const character = useCharacterStore((s) => s.character);
  const isEditing = useCharacterStore((s) => s.isEditing);
  const patch = useCharacterStore((s) => s.patchCharacter);

  // 2.1: Thu gọn theo DBML: Competence bao hàm cả Knowledge, và Memory phẳng
  const subTabs: Array<{ id: LifeSubTab; label: string }> = [
    { id: 'competence', label: t('detail.life.competence') },
    { id: 'memory', label: t('detail.life.memory') },
  ];

  const addMemory = () => {
    patch((c) => {
      c.memories.push({
        id: `mem-${Date.now()}`,
        content: 'Ký ức mới...',
        sourceScene: 'Cảnh hiện tại',
        createdAt: 'Vừa xong',
      });
    });
  };

  const addCompetence = () => {
    patch((c) => {
      c.competences.push({
        id: `comp-${Date.now()}`,
        name: 'Kỹ năng / Tri thức mới',
        domain: 'Lĩnh vực',
        level: 'Sơ cấp',
        description: 'Mô tả năng lực hoặc kiến thức sở hữu...',
        capabilities: ['Khả năng 1'],
        limitations: ['Giới hạn 1'],
      });
    });
  };

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
                ? 'bg-indigo-50 text-indigo-700 dark:bg-indigo-950/60 dark:text-indigo-300 font-semibold'
                : 'text-fg-muted hover:bg-surface-muted'
            }`}
          >
            {st.label}
          </button>
        ))}
      </nav>

      <div className="min-w-0 flex-1">
        {active === 'competence' && (
          <div className="space-y-3">
            {character.competences.map((c) => (
              <CompetenceCard key={c.id} competence={c} />
            ))}
            {isEditing && (
              <button
                type="button"
                onClick={addCompetence}
                className="flex items-center gap-1.5 text-sm font-medium text-indigo-600 hover:underline dark:text-indigo-400 pt-2"
              >
                <Plus className="h-4 w-4" />
                Thêm Năng lực / Tri thức
              </button>
            )}
          </div>
        )}

        {active === 'memory' && (
          <div className="space-y-3">
            {character.memories.map((m) => (
              <MemoryCard key={m.id} memory={m} />
            ))}
            {isEditing && (
              <button
                type="button"
                onClick={addMemory}
                className="flex items-center gap-1.5 text-sm font-medium text-indigo-600 hover:underline dark:text-indigo-400 pt-2"
              >
                <Plus className="h-4 w-4" />
                Thêm Ký ức
              </button>
            )}
          </div>
        )}
      </div>
    </div>
  );
}

function CompetenceCard({ competence }: { competence: CharacterCompetence }) {
  const t = useT();
  const [expanded, setExpanded] = useState(false);
  const isEditing = useCharacterStore((s) => s.isEditing);
  const patch = useCharacterStore((s) => s.patchCharacter);

  const update = (fn: (c: CharacterCompetence) => void) =>
    patch((c) => {
      const x = c.competences.find((y) => y.id === competence.id);
      if (x) fn(x);
    });

  const removeCompetence = () => {
    patch((c) => {
      c.competences = c.competences.filter((x) => x.id !== competence.id);
    });
  };

  const inputCls =
    'mt-1 w-full rounded-md tier-input focus-ring px-2 py-1 text-xs text-fg';

  // 2.1: Nếu không có Capabilities thì đóng vai trò là Knowledge/Tri thức
  const isKnowledge = competence.capabilities.length === 0;

  return (
    <div className="rounded-xl border border-line bg-surface-card p-4 transition-colors">
      <div className="flex items-start justify-between gap-2">
        <div className="min-w-0 flex-1">
          {isEditing ? (
            <>
              <input
                value={competence.name}
                onChange={(e) => update((c) => (c.name = e.target.value))}
                className={inputCls}
                placeholder="Tên năng lực / tri thức"
              />
              <div className="mt-1 grid grid-cols-2 gap-2">
                <input
                  value={competence.domain}
                  onChange={(e) => update((c) => (c.domain = e.target.value))}
                  className={inputCls}
                  placeholder="Lĩnh vực / Domain"
                />
                <input
                  value={competence.level || ''}
                  onChange={(e) => update((c) => (c.level = e.target.value))}
                  className={inputCls}
                  placeholder="Cấp bậc / Level"
                />
              </div>
            </>
          ) : (
            <>
              <div className="flex items-center gap-2">
                <h4 className="font-semibold text-fg">
                  {competence.name}
                </h4>
                {isKnowledge && (
                  <span className="rounded-full bg-surface-muted px-2 py-0.5 text-[11px] font-medium text-fg-muted">
                    Tri thức / Lore
                  </span>
                )}
              </div>
              <p className="text-xs text-fg-muted">
                {competence.domain} {competence.level && `· ${competence.level}`}
              </p>
            </>
          )}
        </div>

        {isEditing && (
          <button
            type="button"
            onClick={removeCompetence}
            className="text-rose-500 hover:text-rose-600 p-1"
            title={t('common.remove')}
          >
            <Trash2 className="h-4 w-4" />
          </button>
        )}
      </div>

      {isEditing ? (
        <>
          <textarea
            rows={2}
            value={competence.description}
            onChange={(e) => update((c) => (c.description = e.target.value))}
            className={`${inputCls} mt-2`}
            placeholder="Mô tả"
          />
          <input
            value={competence.experience || ''}
            onChange={(e) => update((c) => (c.experience = e.target.value))}
            className={inputCls}
            placeholder="Kinh nghiệm / Nguồn gốc"
          />
        </>
      ) : (
        <>
          <p className="mt-2 text-sm text-fg-muted">
            {competence.description}
          </p>
          {competence.experience && (
            <p className="mt-1 text-xs italic text-fg-muted">
              {competence.experience}
            </p>
          )}
        </>
      )}

      <button
        type="button"
        onClick={() => setExpanded(!expanded)}
        className="mt-2 text-xs text-indigo-600 hover:underline dark:text-indigo-400 font-medium"
      >
        {expanded ? t('detail.life.hideDetails') : t('detail.life.showDetails')}
      </button>

      {expanded && (
        <div className="grid grid-cols-1 gap-4 border-t border-line mt-3 pt-3 md:grid-cols-2">
          <div>
            <p className="mb-1.5 text-xs font-semibold uppercase tracking-wide text-emerald-700 dark:text-emerald-400">
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
              <ul className="list-disc space-y-1 pl-4 text-xs text-fg">
                {competence.capabilities.length > 0 ? (
                  competence.capabilities.map((cap, i) => <li key={i}>{cap}</li>)
                ) : (
                  <li className="text-fg-subtle italic">Không có hành động đặc biệt</li>
                )}
              </ul>
            )}
          </div>
          <div>
            <p className="mb-1.5 text-xs font-semibold uppercase tracking-wide text-rose-700 dark:text-rose-400">
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
              <ul className="list-disc space-y-1 pl-4 text-xs text-fg">
                {competence.limitations.length > 0 ? (
                  competence.limitations.map((lim, i) => <li key={i}>{lim}</li>)
                ) : (
                  <li className="text-fg-subtle italic">Chưa xác định điểm yếu</li>
                )}
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
            className="flex-1 rounded-md tier-input focus-ring px-2 py-1 text-xs"
          />
          <button type="button" onClick={() => onRemove(i)} className="text-rose-500 p-1">
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
