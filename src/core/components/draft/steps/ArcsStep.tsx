import { useDraftStream } from '../useDraftStream';
import { DraftStreamableBlock } from '../DraftStreamableBlock';
import { mockArcs } from '../mockDraftData';
import type { FeedbackContext } from '../types';
import { MessageSquareQuote, ArrowRight, ArrowLeft, Target, HelpCircle, CheckCircle2, User } from 'lucide-react';

export function ArcsStep({ onFeedback, onNext, onBack }: { onFeedback: (f: FeedbackContext) => void; onNext: () => void; onBack: () => void }) {
  const streamed = useDraftStream(mockArcs.length, 500);
  const isLoaded = streamed >= mockArcs.length;

  return (
    <div className="space-y-6">
      {mockArcs.map((arc, idx) => (
        <DraftStreamableBlock key={arc.id} isLoaded={idx < streamed} loadingTitle="Structuring narrative arcs & themes..." tag="Arc">
          <div className="rounded-2xl border border-slate-200 bg-white p-4 shadow-xs dark:border-slate-800 dark:bg-slate-900">
            <div className="flex items-center justify-between border-b border-slate-100 pb-2 dark:border-slate-800">
              <div>
                <h4 className="text-sm font-semibold text-slate-900 dark:text-slate-100">{arc.name}</h4>
                <span className="text-[10px] font-medium text-indigo-600 dark:text-indigo-400 uppercase tracking-wider">Arc {idx + 1}</span>
              </div>
              <button
                onClick={() => onFeedback({ id: arc.id, tag: 'Arc', title: arc.name })}
                className="flex items-center gap-1.5 rounded-lg px-2 py-1 text-[11px] font-medium text-slate-500 hover:bg-slate-100 hover:text-blue-600 dark:text-slate-400 dark:hover:bg-slate-800 transition-colors"
              >
                <MessageSquareQuote className="h-3.5 w-3.5" /> Góp ý Arc
              </button>
            </div>

            <p className="mt-2 text-xs leading-relaxed text-slate-600 dark:text-slate-400">{arc.summarize}</p>

            {/* Themes trong Arc */}
            <div className="mt-4 space-y-3">
              {arc.themes.map(theme => (
                <div key={theme.id} className="rounded-xl border border-slate-200 bg-slate-50/50 p-3 dark:border-slate-800 dark:bg-slate-950/30">
                  <div className="flex items-start justify-between gap-2">
                    <div className="flex items-center gap-1.5 text-xs font-semibold text-slate-900 dark:text-slate-100">
                      <Target className="h-3.5 w-3.5 text-rose-500" />
                      {theme.question}
                    </div>
                    <div className="flex flex-wrap gap-1">
                      <span className="rounded bg-slate-200 px-1.5 py-0.5 text-[9px] font-bold text-slate-700 uppercase dark:bg-slate-800 dark:text-slate-300">
                        {theme.scope}
                      </span>
                      <span className="rounded bg-blue-100 px-1.5 py-0.5 text-[9px] font-bold text-blue-700 uppercase dark:bg-blue-950/60 dark:text-blue-300">
                        {theme.resolutionType}
                      </span>
                    </div>
                  </div>

                  <div className="mt-2 text-[11px] text-slate-700 dark:text-slate-300">
                    <p><span className="font-medium text-slate-500 dark:text-slate-400">Resolution:</span> {theme.resolution}</p>
                    {theme.message && (
                      <p className="mt-1 italic text-slate-500 dark:text-slate-400">Message: "{theme.message}"</p>
                    )}
                  </div>

                  {/* Theme Arguments */}
                  <div className="mt-3 border-t border-slate-200 pt-2 dark:border-slate-800">
                    <p className="text-[10px] font-semibold uppercase tracking-wide text-slate-500 dark:text-slate-400 mb-1.5 flex items-center gap-1">
                      <User className="h-3 w-3" />
                      Character Arguments
                    </p>
                    {theme.arguments.length === 0 ? (
                      <p className="text-[10px] italic text-slate-400">Chưa có nhân vật nào trả lời theme này (Có thể bổ sung sau).</p>
                    ) : (
                      <div className="space-y-1.5">
                        {theme.arguments.map(arg => (
                          <div key={arg.id} className="flex flex-col rounded bg-white p-2 text-[10px] shadow-2xs border border-slate-100 dark:border-slate-700 dark:bg-slate-900">
                            <div className="flex items-center justify-between mb-0.5">
                              <span className="font-semibold text-slate-800 dark:text-slate-200">{arg.characterName}</span>
                              <div className="flex gap-1">
                                <span className="text-emerald-600 dark:text-emerald-400 font-medium">{arg.role}</span>
                                <span className="text-slate-400">•</span>
                                <span className="text-slate-500">{arg.answerMode}</span>
                              </div>
                            </div>
                            <span className="text-slate-600 dark:text-slate-400">{arg.note}</span>
                          </div>
                        ))}
                      </div>
                    )}
                  </div>
                </div>
              ))}
            </div>

          </div>
        </DraftStreamableBlock>
      ))}

      {isLoaded && (
        <div className="fixed bottom-6 right-6 z-20 flex items-center gap-2.5 animate-in fade-in-0 duration-300">
          <button onClick={onBack} className="flex items-center gap-1.5 rounded-full border border-slate-200 bg-white px-4 py-2.5 text-xs font-semibold text-slate-700 shadow-lg hover:bg-slate-50 dark:border-slate-800 dark:bg-slate-900 dark:text-slate-300 dark:hover:bg-slate-800 transition-all">
            <ArrowLeft className="h-4 w-4" /> Back
          </button>
          <button onClick={onNext} className="flex items-center gap-2 rounded-full bg-slate-900 px-5 py-2.5 text-xs font-semibold text-white shadow-lg shadow-slate-900/20 transition-all hover:scale-105 hover:bg-slate-800 dark:bg-slate-100 dark:text-slate-900 dark:hover:bg-slate-200">
            <span>Next: Chapters</span>
            <ArrowRight className="h-4 w-4" />
          </button>
        </div>
      )}
    </div>
  );
}
