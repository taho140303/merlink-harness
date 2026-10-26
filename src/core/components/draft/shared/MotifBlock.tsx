import { useState } from 'react';
import { GitMerge, ChevronRight, Activity, Target } from 'lucide-react';
import { cn } from '../../../../shared/utils/cn';
import type { DraftMotif, MotifCategory, FeedbackContext } from '../types';

const CATEGORY_COLOR: Record<MotifCategory, string> = {
  character_arc: 'bg-indigo-100 text-indigo-700 dark:bg-indigo-950/60 dark:text-indigo-300',
  relationship: 'bg-rose-100 text-rose-700 dark:bg-rose-950/60 dark:text-rose-300',
  symbolic: 'bg-violet-100 text-violet-700 dark:bg-violet-950/60 dark:text-violet-300',
  plot: 'bg-emerald-100 text-emerald-700 dark:bg-emerald-950/60 dark:text-emerald-300',
};

export function MotifBlock({ motifs, onFeedback }: { motifs: DraftMotif[]; onFeedback?: (f: FeedbackContext) => void }) {
  const [expandedId, setExpandedId] = useState<string | null>(null);

  if (!motifs || motifs.length === 0) return null;

  return (
    <div className="space-y-2 mt-3">
      <div className="flex items-center gap-1.5 text-[10px] font-semibold uppercase tracking-wider text-slate-400 dark:text-slate-500">
        <Target className="h-3 w-3" /> Applied Motifs
      </div>
      <div className="grid gap-2">
        {motifs.map(mo => {
          const isExpanded = expandedId === mo.id;
          return (
            <div key={mo.id} className="overflow-hidden rounded-lg border border-slate-200 bg-white shadow-2xs dark:border-slate-700 dark:bg-slate-900">
              {/* Header */}
              <div 
                className="flex items-center justify-between p-2.5 cursor-pointer hover:bg-slate-50 dark:hover:bg-slate-800/50 transition-colors"
                onClick={() => setExpandedId(isExpanded ? null : mo.id)}
              >
                <div className="flex items-center gap-2 min-w-0">
                  <ChevronRight className={cn("h-3.5 w-3.5 text-slate-400 transition-transform", isExpanded && "rotate-90")} />
                  <GitMerge className="h-3.5 w-3.5 text-slate-500 shrink-0" />
                  <span className="text-[11px] font-semibold text-slate-900 dark:text-slate-100 truncate">{mo.name}</span>
                  <span className={cn("rounded px-1.5 py-0.5 text-[9px] font-medium uppercase shrink-0", CATEGORY_COLOR[mo.category])}>
                    {mo.category.replace('_', ' ')}
                  </span>
                  <span className="rounded bg-slate-100 px-1.5 py-0.5 text-[9px] font-bold text-slate-600 dark:bg-slate-800 dark:text-slate-300 shrink-0">
                    {mo.scope.toUpperCase()}
                  </span>
                </div>
                {onFeedback && (
                  <button 
                    onClick={(e) => { e.stopPropagation(); onFeedback({ id: mo.id, tag: 'Motif', title: mo.name }); }}
                    className="text-[10px] text-slate-400 hover:text-blue-600 transition-colors shrink-0 ml-2"
                  >
                    Góp ý
                  </button>
                )}
              </div>

              {/* Body */}
              {isExpanded && (
                <div className="border-t border-slate-100 p-3 dark:border-slate-800 bg-slate-50/30 dark:bg-slate-950/20 space-y-3">
                  <div>
                    <p className="text-[11px] text-slate-700 dark:text-slate-300">{mo.description}</p>
                    <div className="mt-1.5 flex flex-wrap gap-x-3 gap-y-1 text-[10px] text-slate-500 dark:text-slate-400">
                      <span><strong className="font-medium text-slate-600 dark:text-slate-300">Code:</strong> {mo.code}</span>
                      <span><strong className="font-medium text-slate-600 dark:text-slate-300">From:</strong> {mo.from}</span>
                      {mo.minChapter !== undefined && <span><strong className="font-medium text-slate-600 dark:text-slate-300">Chapters:</strong> {mo.minChapter} - {mo.maxChapter || '∞'}</span>}
                    </div>
                  </div>

                  {/* Nested Event Blocks */}
                  {mo.eventBlocks && mo.eventBlocks.length > 0 && (
                    <div>
                      <p className="text-[10px] font-semibold uppercase tracking-wide text-slate-500 dark:text-slate-400 mb-1.5 flex items-center gap-1">
                        <Activity className="h-3 w-3" /> Event Blocks
                      </p>
                      <div className="space-y-1">
                        {mo.eventBlocks.map(eb => (
                          <div key={eb.id} className="flex items-center gap-2 rounded bg-white p-1.5 text-[10px] shadow-2xs border border-slate-100 dark:border-slate-800 dark:bg-slate-900">
                            <span className="font-mono text-slate-500">{eb.code}</span>
                            <span className="text-slate-300">|</span>
                            <span className="font-medium text-indigo-600 dark:text-indigo-400">{eb.component}</span>
                            <span className="text-slate-400">→</span>
                            <span className="font-medium text-emerald-600 dark:text-emerald-400">{eb.state}</span>
                          </div>
                        ))}
                      </div>
                    </div>
                  )}
                </div>
              )}
            </div>
          );
        })}
      </div>
    </div>
  );
}
