import { useState } from 'react';
import { GitMerge, ChevronRight, Activity, Target, Book, Sparkles, ShieldAlert, ArrowRight } from 'lucide-react';
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
    <div className="space-y-2 mt-4">
      <div className="flex items-center gap-1.5 text-[10px] font-semibold uppercase tracking-wider text-slate-400 dark:text-slate-500">
        <Target className="h-3.5 w-3.5" /> Applied Motifs
      </div>
      <div className="grid gap-2">
        {motifs.map(mo => {
          const isExpanded = expandedId === mo.id;
          return (
            <div key={mo.id} className="overflow-hidden rounded-lg border border-slate-200 bg-white shadow-2xs dark:border-slate-700 dark:bg-slate-900">
              
              {/* Header */}
              <div 
                className="flex items-center justify-between p-3 cursor-pointer hover:bg-slate-50 dark:hover:bg-slate-800/50 transition-colors"
                onClick={() => setExpandedId(isExpanded ? null : mo.id)}
              >
                <div className="flex items-center gap-2 min-w-0">
                  <ChevronRight className={cn("h-4 w-4 text-slate-400 transition-transform shrink-0", isExpanded && "rotate-90")} />
                  <GitMerge className="h-4 w-4 text-slate-500 shrink-0" />
                  <span className="text-xs font-semibold text-slate-900 dark:text-slate-100 truncate">{mo.name}</span>
                  <span className={cn("rounded px-2 py-0.5 text-[9px] font-bold uppercase shrink-0", CATEGORY_COLOR[mo.category])}>
                    {mo.category.replace('_', ' ')}
                  </span>
                  <span className="rounded bg-slate-100 px-2 py-0.5 text-[9px] font-bold text-slate-600 dark:bg-slate-800 dark:text-slate-300 shrink-0">
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
                <div className="border-t border-slate-100 p-4 dark:border-slate-800 bg-slate-50/40 dark:bg-slate-950/20 space-y-4">
                  {/* Meta info */}
                  <div>
                    <p className="text-[11px] text-slate-700 dark:text-slate-300 leading-relaxed mb-2">{mo.description}</p>
                    <div className="flex flex-wrap gap-x-4 gap-y-1 text-[10px] text-slate-500 dark:text-slate-400 mb-3">
                      <span><strong className="font-medium text-slate-700 dark:text-slate-300">Code:</strong> {mo.code}</span>
                      <span><strong className="font-medium text-slate-700 dark:text-slate-300">From:</strong> {mo.from}</span>
                      <span><strong className="font-medium text-slate-700 dark:text-slate-300">Version:</strong> v{mo.version}</span>
                      {mo.minChapter !== undefined && (
                        <span><strong className="font-medium text-slate-700 dark:text-slate-300">Chapters:</strong> {mo.minChapter} - {mo.maxChapter || '∞'}</span>
                      )}
                    </div>
                  </div>

                  {/* Genres & Symbolics */}
                  <div className="grid grid-cols-2 gap-3">
                    {mo.genres.length > 0 && (
                      <div>
                        <span className="text-[9px] font-semibold uppercase tracking-wider text-slate-400 flex items-center gap-1 mb-1"><Book className="h-3 w-3"/> Genres</span>
                        <div className="flex flex-wrap gap-1">
                          {mo.genres.map(g => <span key={g} className="rounded bg-blue-50 px-1.5 py-0.5 text-[10px] font-medium text-blue-700 dark:bg-blue-950/60 dark:text-blue-300">{g}</span>)}
                        </div>
                      </div>
                    )}
                    {mo.symbolics.length > 0 && (
                      <div>
                        <span className="text-[9px] font-semibold uppercase tracking-wider text-slate-400 flex items-center gap-1 mb-1"><Sparkles className="h-3 w-3"/> Symbolics</span>
                        <div className="flex flex-wrap gap-1">
                          {mo.symbolics.map(s => <span key={s} className="rounded bg-violet-50 px-1.5 py-0.5 text-[10px] font-medium text-violet-700 dark:bg-violet-950/60 dark:text-violet-300">{s}</span>)}
                        </div>
                      </div>
                    )}
                  </div>

                  {/* Constraints */}
                  {mo.constraints.length > 0 && (
                    <div className="rounded-lg border border-amber-200 bg-amber-50 p-2.5 dark:border-amber-900/50 dark:bg-amber-950/20">
                      <span className="text-[9px] font-semibold uppercase tracking-wider text-amber-700 dark:text-amber-400 flex items-center gap-1 mb-1.5"><ShieldAlert className="h-3 w-3"/> Constraints</span>
                      <ul className="space-y-1.5">
                        {mo.constraints.map(c => (
                          <li key={c.code} className="text-[10px]">
                            <span className="font-mono text-amber-600 dark:text-amber-500 bg-white/50 dark:bg-black/20 px-1 py-0.5 rounded">{c.code}</span>
                            <span className="font-semibold text-amber-900 dark:text-amber-200 ml-1.5">{c.require}</span>
                            <p className="text-amber-700 dark:text-amber-300/80 mt-0.5 italic">Reason: {c.reason}</p>
                          </li>
                        ))}
                      </ul>
                    </div>
                  )}

                  {/* Event Blocks Flow */}
                  <div>
                    <span className="text-[9px] font-semibold uppercase tracking-wider text-slate-400 flex items-center gap-1 mb-2"><Activity className="h-3 w-3"/> Event Blocks Flow</span>
                    <div className="space-y-3 relative before:absolute before:inset-0 before:ml-2.5 before:-translate-x-px md:before:mx-auto md:before:translate-x-0 before:h-full before:w-0.5 before:bg-gradient-to-b before:from-transparent before:via-slate-200 before:to-transparent dark:before:via-slate-700">
                      {mo.eventBlocks.map((eb, index) => (
                        <div key={eb.id} className="relative flex items-start gap-3">
                          {/* Timeline dot */}
                          <div className="absolute left-0 top-1.5 h-5 w-5 -translate-x-1/2 rounded-full border-2 border-white bg-slate-200 dark:border-slate-900 dark:bg-slate-600 z-10" />
                          
                          <div className="ml-6 flex-1 rounded-lg border border-slate-200 bg-white p-2.5 shadow-2xs dark:border-slate-700 dark:bg-slate-900">
                            <div className="flex items-center gap-2 mb-1.5">
                              <span className="font-mono text-[10px] font-bold text-slate-600 dark:text-slate-300">{eb.code}</span>
                              <span className="text-slate-300">|</span>
                              <span className="text-[10px] font-medium text-indigo-600 dark:text-indigo-400">Component: {eb.component}</span>
                              <ArrowRight className="h-3 w-3 text-slate-400" />
                              <span className="text-[10px] font-medium text-emerald-600 dark:text-emerald-400">State: {eb.state}</span>
                            </div>
                            
                            <div className="flex flex-wrap gap-x-3 gap-y-1 text-[9px] text-slate-500">
                              {eb.emotionalRanges.length > 0 && (
                                <span><strong className="font-medium text-rose-500">Emotions:</strong> {eb.emotionalRanges.join(', ')}</span>
                              )}
                              {eb.downByRules && eb.downByRules.length > 0 && (
                                <span><strong className="font-medium text-orange-500">Down By:</strong> {eb.downByRules.map(r => r.ruleName).join(', ')}</span>
                              )}
                            </div>

                            {/* Transitions (Edges) */}
                            {eb.transitions.length > 0 && (
                              <div className="mt-2 space-y-1">
                                {eb.transitions.map(tr => (
                                  <div key={tr.id} className="flex items-center gap-1.5 text-[9px] bg-slate-50 px-2 py-1 rounded dark:bg-slate-800/50">
                                    <GitMerge className="h-2.5 w-2.5 text-slate-400" />
                                    <span className="text-slate-600 dark:text-slate-300">{tr.name}</span>
                                    <span className="text-slate-400">→</span>
                                    <span className="font-mono text-indigo-600 dark:text-indigo-400 font-semibold">{tr.targetEventCode}</span>
                                    <span className="ml-auto text-slate-400">Priority: {tr.priority}</span>
                                  </div>
                                ))}
                              </div>
                            )}
                          </div>
                        </div>
                      ))}
                    </div>
                  </div>

                </div>
              )}
            </div>
          );
        })}
      </div>
    </div>
  );
}
