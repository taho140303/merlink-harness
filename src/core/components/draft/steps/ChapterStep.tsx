import { useDraftStream } from '../useDraftStream';
import { DraftStreamableBlock } from '../DraftStreamableBlock';
import { mockChapters, mockArcs } from '../mockDraftData';
import type { FeedbackContext } from '../types';
import { MessageSquareQuote, ArrowRight, ArrowLeft } from 'lucide-react';
import { ThemeDisplay } from '../shared/ThemeDisplay';

export function ChapterStep({ onFeedback, onNext, onBack }: { onFeedback: (f: FeedbackContext) => void; onNext: () => void; onBack: () => void }) {
  const streamed = useDraftStream(mockChapters.length, 400);
  const isLoaded = streamed >= mockChapters.length;

  let globalChapterIdx = 0;

  return (
    <div className="space-y-6">
      {mockArcs.map((arc) => {
        const arcChapters = mockChapters.filter((c) => c.arcId === arc.id);
        if (arcChapters.length === 0) return null;

        return (
          <div
            key={arc.id}
            className="overflow-hidden rounded-2xl border border-slate-200 bg-white shadow-xs dark:border-slate-800 dark:bg-slate-900 animate-in fade-in-50 duration-300"
          >
            {/* Arc Header & Cumulative Theme Context */}
            <div className="border-b border-slate-100 bg-slate-50/80 px-5 py-4 dark:border-slate-800 dark:bg-slate-800/40">
              <div className="mb-2 flex items-center justify-between">
                <div>
                  <h3 className="text-sm font-bold text-slate-900 dark:text-slate-100">{arc.name}</h3>
                  <p className="mt-1 text-[11px] leading-relaxed text-slate-600 dark:text-slate-400">{arc.summarize}</p>
                </div>
                <button
                  onClick={() => onFeedback({ id: arc.id, tag: 'Arc', title: arc.name })}
                  className="flex items-center gap-1.5 rounded-lg px-2 py-1 text-[11px] font-medium text-slate-500 hover:bg-slate-200/60 hover:text-blue-600 dark:text-slate-400 dark:hover:bg-slate-800 transition-colors shrink-0"
                >
                  <MessageSquareQuote className="h-3.5 w-3.5" /> Góp ý Arc
                </button>
              </div>
              
              {/* Hiển thị Theme cộng dồn */}
              <ThemeDisplay themes={arc.themes} />
            </div>

            {/* Danh sách Chapter (Layout Project View: Không Assets) */}
            <div className="p-5">
              <h4 className="mb-3 text-[11px] font-semibold uppercase tracking-wider text-slate-400 dark:text-slate-500">
                Chapters
              </h4>
              <div className="space-y-0.5">
                {arcChapters.map((chap, idx) => {
                  const currentIdx = globalChapterIdx++;
                  const isChapLoaded = currentIdx < streamed;

                  return (
                    <DraftStreamableBlock
                      key={chap.id}
                      isLoaded={isChapLoaded}
                      loadingTitle="Drafting chapter descriptions..."
                      tag="Chapter"
                    >
                      <div className="flex items-center justify-between py-2.5 px-3 border-b border-slate-100/70 last:border-0 dark:border-slate-800/60 rounded-xl hover:bg-slate-50 dark:hover:bg-slate-800/50 transition-colors group">
                        <div className="flex flex-col min-w-0 pr-4">
                          <span className="text-xs font-medium text-slate-800 dark:text-slate-200 truncate">
                            Chapter {idx + 1}: {chap.name}
                          </span>
                          {chap.description && (
                            <span className="text-[11px] text-slate-500 dark:text-slate-400 line-clamp-1 mt-0.5">
                              {chap.description}
                            </span>
                          )}
                        </div>
                        
                        <div className="flex items-center gap-3 shrink-0">
                          <span className="text-[10px] text-slate-400 font-mono">
                            Pos: {chap.position || idx + 1}
                          </span>
                          <button
                            onClick={() => onFeedback({ id: chap.id, tag: 'Chapter', title: chap.name })}
                            className="opacity-0 group-hover:opacity-100 flex items-center gap-1 text-[10px] font-medium text-slate-500 hover:text-blue-600 dark:text-slate-400 dark:hover:text-blue-400 transition-all"
                          >
                            <MessageSquareQuote className="h-3 w-3" /> Góp ý
                          </button>
                        </div>
                      </div>
                    </DraftStreamableBlock>
                  );
                })}
              </div>
            </div>
          </div>
        );
      })}

      {isLoaded && (
        <div className="fixed bottom-6 right-6 z-20 flex items-center gap-2.5 animate-in fade-in-0 duration-300">
          <button onClick={onBack} className="flex items-center gap-1.5 rounded-full border border-slate-200 bg-white px-4 py-2.5 text-xs font-semibold text-slate-700 shadow-lg hover:bg-slate-50 dark:border-slate-800 dark:bg-slate-900 dark:text-slate-300 dark:hover:bg-slate-800 transition-all">
            <ArrowLeft className="h-4 w-4" /> Back
          </button>
          <button onClick={onNext} className="flex items-center gap-2 rounded-full bg-slate-900 px-5 py-2.5 text-xs font-semibold text-white shadow-lg shadow-slate-900/20 transition-all hover:scale-105 hover:bg-slate-800 dark:bg-slate-100 dark:text-slate-900 dark:hover:bg-slate-200">
            <span>Next: Scenes</span>
            <ArrowRight className="h-4 w-4" />
          </button>
        </div>
      )}
    </div>
  );
}
