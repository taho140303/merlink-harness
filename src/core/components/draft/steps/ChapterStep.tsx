import { useDraftStream } from '../useDraftStream';
import { DraftStreamableBlock } from '../DraftStreamableBlock';
import { mockChapters, mockArcs } from '../mockDraftData';
import type { FeedbackContext } from '../types';
import { MessageSquareQuote, ArrowRight, ArrowLeft, BookOpen, CornerDownRight, Layers } from 'lucide-react';

export function ChapterStep({ onFeedback, onNext, onBack }: { onFeedback: (f: FeedbackContext) => void; onNext: () => void; onBack: () => void }) {
  const streamed = useDraftStream(mockChapters.length, 400);
  const isLoaded = streamed >= mockChapters.length;

  // Biến đếm toàn cục để check tiến độ stream của từng chapter
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
            {/* Cột mốc Arc (Cumulative Context) */}
            <div className="border-b border-slate-100 bg-slate-50/80 px-4 py-3 dark:border-slate-800 dark:bg-slate-800/40">
              <div className="flex items-center gap-2 mb-1">
                <Layers className="h-4 w-4 text-indigo-500" />
                <h3 className="text-sm font-bold text-slate-900 dark:text-slate-100">
                  {arc.name}
                </h3>
              </div>
              <p className="text-[11px] leading-relaxed text-slate-600 dark:text-slate-400">
                {arc.summarize}
              </p>
            </div>

            {/* Danh sách Chapter thuộc Arc này */}
            <div className="divide-y divide-slate-100 dark:divide-slate-800/60">
              {arcChapters.map((chap, idx) => {
                const currentIdx = globalChapterIdx++;
                const isChapLoaded = currentIdx < streamed;

                return (
                  <div key={chap.id} className="p-4">
                    <DraftStreamableBlock
                      isLoaded={isChapLoaded}
                      loadingTitle="Drafting chapter descriptions..."
                      tag="Chapter"
                    >
                      <div className="flex items-start gap-3">
                        <CornerDownRight className="mt-0.5 h-4 w-4 shrink-0 text-slate-300 dark:text-slate-600" />
                        <div className="flex-1">
                          <div className="flex items-center justify-between">
                            <div className="flex items-center gap-2">
                              <BookOpen className="h-3.5 w-3.5 text-slate-400" />
                              <h4 className="text-xs font-semibold text-slate-900 dark:text-slate-100">
                                {chap.name}
                              </h4>
                            </div>
                            <span className="rounded-md bg-slate-100 px-1.5 py-0.5 text-[10px] font-medium text-slate-500 dark:bg-slate-800 dark:text-slate-400">
                              Pos: {idx + 1}
                            </span>
                          </div>
                          
                          <p className="mt-1.5 text-xs leading-relaxed text-slate-600 dark:text-slate-400">
                            {chap.description}
                          </p>
                          
                          <div className="mt-2 flex justify-end">
                            <button
                              onClick={() => onFeedback({ id: chap.id, tag: 'Chapter', title: chap.name })}
                              className="flex items-center gap-1.5 rounded-lg px-2 py-1 text-[11px] font-medium text-slate-500 hover:bg-slate-100 hover:text-blue-600 dark:text-slate-400 dark:hover:bg-slate-800 transition-colors"
                            >
                              <MessageSquareQuote className="h-3.5 w-3.5" /> Góp ý
                            </button>
                          </div>
                        </div>
                      </div>
                    </DraftStreamableBlock>
                  </div>
                );
              })}
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
