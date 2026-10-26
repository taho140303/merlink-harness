import { useDraftStream } from '../useDraftStream';
import { DraftStreamableBlock } from '../DraftStreamableBlock';
import { mockCharacters } from '../mockDraftData';
import type { FeedbackContext } from '../types';
import { MessageSquareQuote, ArrowRight, ArrowLeft, User } from 'lucide-react';

export function CharacterStep({ onFeedback, onNext, onBack }: { onFeedback: (f: FeedbackContext) => void; onNext: () => void; onBack: () => void }) {
  const streamed = useDraftStream(mockCharacters.length, 500);
  const isLoaded = streamed >= mockCharacters.length;

  return (
    <div className="space-y-4">
      {mockCharacters.map((char, idx) => (
        <DraftStreamableBlock key={char.id} isLoaded={idx < streamed} loadingTitle="Assembling character roster..." tag="Character">
          <div className="rounded-2xl border border-slate-200 bg-white p-4 shadow-xs dark:border-slate-800 dark:bg-slate-900">
            <div className="flex items-center gap-3">
              <div className="flex h-10 w-10 shrink-0 items-center justify-center rounded-xl bg-slate-100 text-slate-800 dark:bg-slate-800 dark:text-slate-200">
                <User className="h-5 w-5" />
              </div>
              <div>
                <h4 className="text-xs font-semibold text-slate-900 dark:text-slate-100">{char.name}</h4>
                <p className="text-[11px] text-slate-500 dark:text-slate-400">{char.role}</p>
              </div>
            </div>
            
            <div className="mt-3">
              <span className="text-[10px] uppercase tracking-wide text-slate-400 block mb-1">Appearances</span>
              <div className="flex flex-wrap gap-1.5">
                {char.appearances.map(a => (
                  <span key={a} className="rounded-md bg-slate-50 border border-slate-200 px-2 py-0.5 text-[10px] font-medium text-slate-700 dark:border-slate-700 dark:bg-slate-800 dark:text-slate-300">
                    {a}
                  </span>
                ))}
              </div>
            </div>

            <div className="mt-3 flex justify-end">
              <button
                onClick={() => onFeedback({ id: char.id, tag: 'Character', title: char.name })}
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
            <span>Next: Detail</span>
            <ArrowRight className="h-4 w-4" />
          </button>
        </div>
      )}
    </div>
  );
}
