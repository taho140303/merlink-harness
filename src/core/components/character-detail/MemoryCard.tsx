import type { ReactNode } from 'react';
import type { Memory, MemoryStatus, MemoryType } from './types';

const typeColor: Record<MemoryType, string> = {
  Episodic: 'bg-sky-100 text-sky-700',
  Semantic: 'bg-violet-100 text-violet-700',
  Procedural: 'bg-teal-100 text-teal-700',
};

const statusColor: Record<MemoryStatus, string> = {
  Active: 'bg-emerald-100 text-emerald-700',
  Suppressed: 'bg-amber-100 text-amber-700',
  Forgotten: 'bg-gray-200 text-gray-700',
  Distorted: 'bg-red-100 text-red-700',
};

export function MemoryCard({ memory }: { memory: Memory }) {
  return (
    <details className="rounded-xl border border-gray-200 bg-white shadow-sm group">
      <summary className="cursor-pointer list-none px-4 py-3 flex items-start justify-between gap-3">
        <div className="flex-1 min-w-0">
          <p className="font-semibold text-gray-900 truncate">{memory.summary}</p>
          <div className="flex flex-wrap gap-1.5 mt-1.5">
            <span className={`px-2 py-0.5 text-[10px] rounded-full font-medium ${typeColor[memory.type]}`}>
              {memory.type}
            </span>
            <span className={`px-2 py-0.5 text-[10px] rounded-full font-medium ${statusColor[memory.status]}`}>
              {memory.status}
            </span>
          </div>
        </div>
        <span className="text-gray-400 text-xs mt-1 group-open:rotate-180 transition-transform">▾</span>
      </summary>

      <div className="px-4 pb-4 border-t border-gray-100 pt-4 space-y-4 text-sm">
        <div className="grid grid-cols-1 md:grid-cols-2 gap-3">
          <div>
            <p className="text-[11px] uppercase tracking-wide text-gray-500 mb-1">Recollection</p>
            <p className="text-gray-800 leading-relaxed">{memory.recollection}</p>
          </div>
          <div>
            <p className="text-[11px] uppercase tracking-wide text-gray-500 mb-1">Interpretation</p>
            <p className="text-gray-800 leading-relaxed">{memory.interpretation}</p>
          </div>
        </div>

        <div className="grid grid-cols-2 md:grid-cols-4 gap-3 text-xs">
          <Info label="Certainty" value={memory.certainty} />
          <Info label="Knowledge Source" value={memory.knowledgeSource} />
          <Info label="Occurred At" value={memory.occurredAt} />
          <Info label="Perceived At" value={memory.perceivedAt} />
        </div>

        {memory.references.length > 0 && (
          <Section title="References">
            <div className="flex flex-wrap gap-1.5">
              {memory.references.map((r, i) => (
                <span key={i} className="px-2 py-0.5 text-[11px] rounded-full bg-gray-100 text-gray-700">
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
                  className="px-2 py-0.5 text-[11px] rounded-full bg-rose-50 text-rose-700 border border-rose-100"
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
                <li key={i} className="text-xs text-gray-700">
                  <span className="font-medium">{c.label}</span>{' '}
                  <span className="text-indigo-600 underline-offset-2 hover:underline">
                    {c.targetSummary}
                  </span>
                </li>
              ))}
            </ul>
          </Section>
        )}

        {memory.revisions.length > 0 && (
          <Section title="Revision History">
            <ol className="relative border-l-2 border-gray-200 ml-1 space-y-3 pl-4">
              {memory.revisions.map((rev) => (
                <li key={rev.revisionNumber} className="relative">
                  <span className="absolute -left-[21px] top-1 h-2.5 w-2.5 rounded-full bg-indigo-500 border-2 border-white" />
                  <p className="text-xs font-medium text-gray-800">
                    Rev {rev.revisionNumber} — {rev.reason}
                  </p>
                  <p className="text-xs text-gray-600">{rev.change}</p>
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
      <p className="text-[10px] uppercase tracking-wide text-gray-500">{label}</p>
      <p className="text-gray-800 font-medium">{value}</p>
    </div>
  );
}

function Section({ title, children }: { title: string; children: ReactNode }) {
  return (
    <div>
      <p className="text-[11px] uppercase tracking-wide text-gray-500 mb-1.5">{title}</p>
      {children}
    </div>
  );
}
