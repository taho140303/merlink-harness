import type { ReactNode } from 'react';
import { Plus, Trash2 } from 'lucide-react';
import type {
  KnowledgeCertainty,
  Memory,
  MemoryStatus,
  MemoryType,
} from './types';
import { CERTAINTY_OPTIONS } from './types';
import { useCharacterDetailStore } from './store';
import { useT } from '../../../shared/i18n/useT';

const typeColor: Record<MemoryType, string> = {
  Episodic: 'bg-sky-100 text-sky-700 dark:bg-sky-950/60 dark:text-sky-300',
  Semantic: 'bg-violet-100 text-violet-700 dark:bg-violet-950/60 dark:text-violet-300',
  Procedural: 'bg-teal-100 text-teal-700 dark:bg-teal-950/60 dark:text-teal-300',
};

const statusColor: Record<MemoryStatus, string> = {
  Active: 'bg-emerald-100 text-emerald-700 dark:bg-emerald-950/60 dark:text-emerald-300',
  Suppressed: 'bg-amber-100 text-amber-700 dark:bg-amber-950/60 dark:text-amber-300',
  Forgotten: 'bg-slate-200 text-slate-700 dark:bg-slate-800 dark:text-slate-300',
  Distorted: 'bg-rose-100 text-rose-700 dark:bg-rose-950/60 dark:text-rose-300',
};

function CompactInput({
  label,
  value,
  onChange,
  multiline,
}: {
  label: string;
  value: string;
  onChange: (v: string) => void;
  multiline?: boolean;
}) {
  const cls =
    'mt-1 w-full rounded-md border border-slate-200 bg-slate-50 px-2 py-1 text-xs text-slate-800 focus:outline-hidden focus:ring-1 focus:ring-slate-400 dark:border-slate-700 dark:bg-slate-950 dark:text-slate-200';
  return (
    <div>
      <p className="text-[10px] uppercase tracking-wide text-slate-500 dark:text-slate-400">
        {label}
      </p>
      {multiline ? (
        <textarea rows={3} value={value} onChange={(e) => onChange(e.target.value)} className={cls} />
      ) : (
        <input value={value} onChange={(e) => onChange(e.target.value)} className={cls} />
      )}
    </div>
  );
}

function CompactSelect({
  label,
  value,
  options,
  onChange,
}: {
  label: string;
  value: string;
  options: readonly string[];
  onChange: (v: string) => void;
}) {
  return (
    <div>
      <p className="text-[10px] uppercase tracking-wide text-slate-500 dark:text-slate-400">
        {label}
      </p>
      <select
        value={value}
        onChange={(e) => onChange(e.target.value)}
        className="mt-1 w-full rounded-md border border-slate-200 bg-slate-50 px-2 py-1 text-xs text-slate-800 focus:outline-hidden focus:ring-1 focus:ring-slate-400 dark:border-slate-700 dark:bg-slate-950 dark:text-slate-200"
      >
        {options.map((o) => (
          <option key={o} value={o}>
            {o}
          </option>
        ))}
      </select>
    </div>
  );
}

export function MemoryCard({ memory }: { memory: Memory }) {
  const t = useT();
  const isEditing = useCharacterDetailStore((s) => s.isEditing);
  const patch = useCharacterDetailStore((s) => s.patchCharacter);

  const update = (fn: (m: Memory) => void) =>
    patch((c) => {
      const m = c.memories.find((x) => x.id === memory.id);
      if (m) fn(m);
    });

  return (
    <details className="group rounded-xl border border-slate-200 bg-white shadow-sm dark:border-slate-800 dark:bg-slate-900">
      <summary className="flex cursor-pointer list-none items-start justify-between gap-3 px-4 py-3">
        <div className="min-w-0 flex-1">
          {isEditing ? (
            <input
              value={memory.summary}
              onChange={(e) => update((m) => (m.summary = e.target.value))}
              className="w-full rounded-md border border-slate-200 bg-slate-50 px-2 py-1 text-sm font-semibold text-slate-900 dark:border-slate-700 dark:bg-slate-950 dark:text-slate-100"
            />
          ) : (
            <p className="truncate font-semibold text-slate-900 dark:text-slate-100">
              {memory.summary}
            </p>
          )}
          <div className="mt-1.5 flex flex-wrap gap-1.5">
            <span className={`rounded-full px-2 py-0.5 text-[10px] font-medium ${typeColor[memory.type]}`}>
              {memory.type}
            </span>
            <span className={`rounded-full px-2 py-0.5 text-[10px] font-medium ${statusColor[memory.status]}`}>
              {memory.status}
            </span>
          </div>
        </div>
        <span className="mt-1 text-xs text-slate-400 transition-transform group-open:rotate-180">
          ▾
        </span>
      </summary>

      <div className="space-y-4 border-t border-slate-100 px-4 pb-4 pt-4 text-sm dark:border-slate-800">
        <div className="grid grid-cols-1 gap-3 md:grid-cols-2">
          {isEditing ? (
            <>
              <CompactInput
                label={t('detail.life.recollection')}
                value={memory.recollection}
                onChange={(v) => update((m) => (m.recollection = v))}
                multiline
              />
              <CompactInput
                label={t('detail.life.interpretation')}
                value={memory.interpretation}
                onChange={(v) => update((m) => (m.interpretation = v))}
                multiline
              />
            </>
          ) : (
            <>
              <div>
                <p className="mb-1 text-[11px] uppercase tracking-wide text-slate-500 dark:text-slate-400">
                  {t('detail.life.recollection')}
                </p>
                <p className="leading-relaxed text-slate-800 dark:text-slate-200">
                  {memory.recollection}
                </p>
              </div>
              <div>
                <p className="mb-1 text-[11px] uppercase tracking-wide text-slate-500 dark:text-slate-400">
                  {t('detail.life.interpretation')}
                </p>
                <p className="leading-relaxed text-slate-800 dark:text-slate-200">
                  {memory.interpretation}
                </p>
              </div>
            </>
          )}
        </div>

        {isEditing ? (
          <div className="grid grid-cols-2 gap-3 md:grid-cols-4">
            <CompactSelect
              label={t('detail.life.certainty')}
              value={memory.certainty}
              options={CERTAINTY_OPTIONS}
              onChange={(v) => update((m) => (m.certainty = v as KnowledgeCertainty))}
            />
            <CompactInput
              label={t('detail.life.source')}
              value={memory.knowledgeSource}
              onChange={(v) => update((m) => (m.knowledgeSource = v))}
            />
            <CompactInput
              label={t('detail.life.occurredAt')}
              value={memory.occurredAt}
              onChange={(v) => update((m) => (m.occurredAt = v))}
            />
            <CompactInput
              label={t('detail.life.perceivedAt')}
              value={memory.perceivedAt}
              onChange={(v) => update((m) => (m.perceivedAt = v))}
            />
          </div>
        ) : (
          <div className="grid grid-cols-2 gap-3 text-xs md:grid-cols-4">
            <Info label={t('detail.life.certainty')} value={memory.certainty} />
            <Info label={t('detail.life.knowledgeSource')} value={memory.knowledgeSource} />
            <Info label={t('detail.life.occurredAt')} value={memory.occurredAt} />
            <Info label={t('detail.life.perceivedAt')} value={memory.perceivedAt} />
          </div>
        )}

        {(isEditing || memory.references.length > 0) && (
          <Section title={t('detail.life.references')}>
            {isEditing ? (
              <div className="space-y-2">
                {memory.references.map((r, i) => (
                  <div key={i} className="flex items-center gap-2">
                    <input
                      value={r.role}
                      onChange={(e) => update((m) => (m.references[i].role = e.target.value))}
                      placeholder={t('detail.life.rolePlaceholder')}
                      className="w-28 rounded-md border border-slate-200 bg-slate-50 px-2 py-1 text-xs dark:border-slate-700 dark:bg-slate-950 dark:text-slate-100"
                    />
                    <input
                      value={r.name}
                      onChange={(e) => update((m) => (m.references[i].name = e.target.value))}
                      placeholder={t('detail.life.namePlaceholder')}
                      className="flex-1 rounded-md border border-slate-200 bg-slate-50 px-2 py-1 text-xs dark:border-slate-700 dark:bg-slate-950 dark:text-slate-100"
                    />
                    <button
                      type="button"
                      onClick={() => update((m) => m.references.splice(i, 1))}
                      className="text-rose-500"
                    >
                      <Trash2 className="h-3.5 w-3.5" />
                    </button>
                  </div>
                ))}
                <button
                  type="button"
                  onClick={() =>
                    update((m) => m.references.push({ role: 'Unknown', name: '' }))
                  }
                  className="flex items-center gap-1 text-xs text-indigo-600 hover:underline dark:text-indigo-400"
                >
                  <Plus className="h-3 w-3" /> {t('detail.life.addReference')}
                </button>
              </div>
            ) : (
              <div className="flex flex-wrap gap-1.5">
                {memory.references.map((r, i) => (
                  <span
                    key={i}
                    className="rounded-full bg-slate-100 px-2 py-0.5 text-[11px] text-slate-700 dark:bg-slate-800 dark:text-slate-300"
                  >
                    <span className="font-medium">{r.role}:</span> {r.name}
                  </span>
                ))}
              </div>
            )}
          </Section>
        )}

        {(isEditing || memory.emotions.length > 0) && (
          <Section title={t('detail.life.emotions')}>
            {isEditing ? (
              <div className="space-y-2">
                {memory.emotions.map((e, i) => (
                  <div key={i} className="flex items-center gap-2">
                    <input
                      value={e.name}
                      onChange={(ev) => update((m) => (m.emotions[i].name = ev.target.value))}
                      placeholder={t('detail.life.emotionPlaceholder')}
                      className="w-32 rounded-md border border-slate-200 bg-slate-50 px-2 py-1 text-xs dark:border-slate-700 dark:bg-slate-950 dark:text-slate-100"
                    />
                    <input
                      type="range"
                      min={0}
                      max={100}
                      value={e.intensity}
                      onChange={(ev) =>
                        update((m) => (m.emotions[i].intensity = Number(ev.target.value)))
                      }
                      className="flex-1 accent-rose-600"
                    />
                    <span className="w-10 text-right text-xs font-semibold text-slate-700 dark:text-slate-300">
                      {e.intensity}%
                    </span>
                    <button
                      type="button"
                      onClick={() => update((m) => m.emotions.splice(i, 1))}
                      className="text-rose-500"
                    >
                      <Trash2 className="h-3.5 w-3.5" />
                    </button>
                  </div>
                ))}
                <button
                  type="button"
                  onClick={() =>
                    update((m) => m.emotions.push({ name: 'Emotion', intensity: 50 }))
                  }
                  className="flex items-center gap-1 text-xs text-indigo-600 hover:underline dark:text-indigo-400"
                >
                  <Plus className="h-3 w-3" /> {t('detail.life.addEmotion')}
                </button>
              </div>
            ) : (
              <div className="flex flex-wrap gap-1.5">
                {memory.emotions.map((e) => (
                  <span
                    key={e.name}
                    className="rounded-full border border-rose-100 bg-rose-50 px-2 py-0.5 text-[11px] text-rose-700 dark:border-rose-900/60 dark:bg-rose-950/40 dark:text-rose-300"
                  >
                    {e.name} <span className="font-semibold">{e.intensity}%</span>
                  </span>
                ))}
              </div>
            )}
          </Section>
        )}

        {memory.connections.length > 0 && (
          <Section title={t('detail.life.connections')}>
            <ul className="space-y-1">
              {memory.connections.map((c, i) => (
                <li key={i} className="text-xs text-slate-700 dark:text-slate-300">
                  <span className="font-medium">{c.label}</span>{' '}
                  <span className="text-indigo-600 underline-offset-2 hover:underline dark:text-indigo-400">
                    {c.targetSummary}
                  </span>
                </li>
              ))}
            </ul>
          </Section>
        )}

        {memory.revisions.length > 0 && (
          <Section title={t('detail.life.revisionHistory')}>
            <ol className="relative ml-1 space-y-3 border-l-2 border-slate-200 pl-4 dark:border-slate-700">
              {memory.revisions.map((rev) => (
                <li key={rev.revisionNumber} className="relative">
                  <span className="absolute -left-[21px] top-1 h-2.5 w-2.5 rounded-full border-2 border-white bg-indigo-500 dark:border-slate-900" />
                  <p className="text-xs font-medium text-slate-800 dark:text-slate-200">
                    {t('detail.life.revisionShort')} {rev.revisionNumber} — {rev.reason}
                  </p>
                  <p className="text-xs text-slate-600 dark:text-slate-400">{rev.change}</p>
                </li>
              ))}
            </ol>
          </Section>
        )}
      </div>
    </details>
  );
}

function Info({ label, value }: { label: string; value: string }) {
  return (
    <div>
      <p className="text-[10px] uppercase tracking-wide text-slate-500 dark:text-slate-400">
        {label}
      </p>
      <p className="font-medium text-slate-800 dark:text-slate-200">{value}</p>
    </div>
  );
}

function Section({ title, children }: { title: string; children: ReactNode }) {
  return (
    <div>
      <p className="mb-1.5 text-[11px] uppercase tracking-wide text-slate-500 dark:text-slate-400">
        {title}
      </p>
      {children}
    </div>
  );
}
