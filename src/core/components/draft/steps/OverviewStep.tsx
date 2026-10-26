import { useDraftStream } from '../useDraftStream';
import { DraftStreamableBlock } from '../DraftStreamableBlock';
import { mockOverview } from '../mockDraftData';
import type { FeedbackContext } from '../types';
import { MessageSquareQuote, CheckCircle2 } from 'lucide-react';

export function OverviewStep({ onFeedback, onNext }: { onFeedback: (f: FeedbackContext) => void; onNext: () => void }) {
  const streamed = useDraftStream(1);
  const isLoaded = streamed >= 1;

  return (
    <div className="space-y-4">
      <DraftStreamableBlock isLoaded={isLoaded} loadingTitle="Analyzing core project parameters..." tag="Overview">
        <div className="rounded-2xl border border-slate-200 bg-white p-4 shadow-xs dark:border-slate-800 dark:bg-slate-900">
          <div className="flex items-center justify-between">
            <h4 className="text-sm font-semibold text-slate-900 dark:text-slate-100">
              {mockOverview.title}
            </h4>
            <span className="rounded-md bg-slate-100 px-2 py-0.5 text-[10px] font-medium text-slate-600 dark:bg-slate-800 dark:text-slate-300">
              {mockOverview.language}
            </span>
          </div>
          <p className="mt-1 text-[11px] text-slate-500 dark:text-slate-400">
            Author: {mockOverview.author} • Target: {mockOverview.targetWords}
          </p>
          <p className="mt-3 text-xs leading-relaxed text-slate-700 dark:text-slate-300">
            {mockOverview.summarize}
          </p>
          <div className="mt-3 flex justify-end">
            <button
              onClick={() => onFeedback({ id: 'ov1', tag: 'Overview', title: mockOverview.title })}
              className="flex items-center gap-1.5 rounded-lg px-2 py-1 text-xs font-medium text-slate-500 hover:bg-slate-100 hover:text-blue-600 dark:text-slate-400 dark:hover:bg-slate-800 transition-colors"
            >
              <MessageSquareQuote className="h-3.5 w-3.5" />
              <span>Góp ý</span>
            </button>
          </div>
        </div>
      </DraftStreamableBlock>

      {isLoaded && (
        <div className="fixed bottom-6 right-6 z-20 flex items-center justify-center animate-in fade-in-0 duration-300">
          <button
            onClick={onNext}
            className="flex items-center gap-2 rounded-full bg-slate-900 px-5 py-2.5 text-xs font-semibold text-white shadow-lg shadow-slate-900/20 transition-all hover:scale-105 hover:bg-slate-800 dark:bg-slate-100 dark:text-slate-900 dark:hover:bg-slate-200"
          >
            <span>Next: Arcs</span>
            <CheckCircle2 className="h-4 w-4" />
          </button>
        </div>
      )}
    </div>
  );
}
