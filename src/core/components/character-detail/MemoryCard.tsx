import type { ReactNode } from 'react';
import type { Memory, MemoryStatus, MemoryType } from './types';

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

export function MemoryCard({ memory }: { memory: Memory }) {
  return (
    <details className="group rounded-xl border border-slate-200 bg-white shadow-sm dark:border-slate-800 dark:bg-slate-900">
      <summary className="flex cursor-pointer list-none items-start justify-between gap-3 px-4 py-3">
        <div className="min-w-0 flex-1">
          <p className="truncate font-semibold text-slate-900 dark:text-slate-100">
            {memory.summary}
          </p>
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
          <div>
            <p className="mb-1 text-[11px] uppercase tracking-wide text-slate-500 dark:text-slate-400">Recollection</p>
            <p className="leading-relaxed text-slate-800 dark:text-slate-200">{memory.recollection}</p>
          </div>
          <div>
            <p className="mb-1 text-[11px] uppercase tracking-wide text-slate-500 dark:text-slate-400">Interpretation</p>
            <p className="leading-relaxed text-slate-800 dark:text-slate-200">{memory.interpretation}</p>
          </div>
        </div>

        <div className="grid grid-cols-2 gap-3 text-xs md:grid-cols-4">
          <Info label="Certainty" value={memory.certainty} />
          <Info label="Knowledge Source" value={memory.knowledgeSource} />
          <Info label="Occurred At" value={memory.occurredAt} />
          <Info label="Perceived At" value={memory.perceivedAt} />
        </div>

        {memory.references.length > 0 && (
          <Section title="References">
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
          </Section>
        )}

        {memory.emotions.length > 0 && (
          <Section title="Emotions">
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
          </Section>
        )}

        {memory.connections.length > 0 && (
          <Section title="Connections">
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
          <Section title="Revision History">
            <ol className="relative ml-1 space-y-3 border-l-2 border-slate-200 pl-4 dark:border-slate-700">
              {memory.revisions.map((rev) => (
                <li key={rev.revisionNumber} className="relative">
                  <span className="absolute -left-[21px] top-1 h-2.5 w-2.5 rounded-full border-2 border-white bg-indigo-500 dark:border-slate-900" />
                  <p className="text-xs font-medium text-slate-800 dark:text-slate-200">
                    Rev {rev.revisionNumber} — {rev.reason}
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
      <p className="text-[10px] uppercase tracking-wide text-slate-500 dark:text-slate-400">{label}</p>
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
