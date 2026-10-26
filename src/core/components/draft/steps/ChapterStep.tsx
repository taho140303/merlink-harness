import { useDraftStream } from '../useDraftStream';
import { DraftStreamableBlock } from '../DraftStreamableBlock';
import { mockChapters } from '../mockDraftData';
import type { FeedbackContext } from '../types';
import { MessageSquareQuote, ArrowRight, ArrowLeft, BookOpen } from 'lucide-react';

export function ChapterStep({ onFeedback, onNext, onBack }: { onFeedback: (f: FeedbackContext) => void; onNext: () => void; onBack: () => void }) {
  const streamed = useDraftStream(mockChapters.length, 400);
  const isLoaded = streamed >= mockChapters.length;

  return (
    <div className="space-y-4">
      {mockChapters.map((chap, idx) => (
        <DraftStreamableBlock key={chap.id} isLoaded={idx < streamed} loadingTitle="Drafting chapter descriptions..." tag="Chapter">
          <div className="rounded-2xl border border-slate-200 bg-white p-4 shadow-xs dark:border-slate-800 dark:bg-slate-900">
            <div className="flex items-center gap-2">
              <BookOpen className="h-4 w-4 text-slate-400" />
              <h4 className="text-xs font-semibold text-slate-900 dark:text-slate-100">{chap.name}</h4>
            </div>
            <p className="mt-2 text-xs leading-relaxed text-slate-600 dark:text-slate-400">{chap.description}</p>
            <div className="mt-3 flex justify-end">
              <button
                onClick={() => onFeedback({ id: chap.id, tag: 'Chapter', title: chap.name })}
                className="flex items-center gap-1.5 rounded-lg px-2 py-1 text-xs font-medium text-slate-500 hover:bg-slate-100 hover:text-blue-600 dark:text-slate-400 dark:hover:bg-slate-800 transition-colors"
              >
                <MessageSquareQuote className="h-3.5 w-3.5" /> Góp ý
              </button>
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
            <span>Next: Scenes</span>
            <ArrowRight className="h-4 w-4" />
          </button>
        </div>
      )}
    </div>
  );
}
