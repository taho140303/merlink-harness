import { useDraftStream } from '../useDraftStream';
import { DraftStreamableBlock } from '../DraftStreamableBlock';
import { mockWorlds } from '../mockDraftData';
import type { FeedbackContext } from '../types';
import { MessageSquareQuote, ArrowRight, ArrowLeft, Globe2 } from 'lucide-react';

export function WorldStep({ onFeedback, onNext, onBack }: { onFeedback: (f: FeedbackContext) => void; onNext: () => void; onBack: () => void }) {
  const streamed = useDraftStream(mockWorlds.length, 450);
  const isLoaded = streamed >= mockWorlds.length;

  return (
    <div className="space-y-4">
      {mockWorlds.map((world, idx) => (
        <DraftStreamableBlock key={world.id} isLoaded={idx < streamed} loadingTitle="Constructing world settings..." tag="World">
          <div className="rounded-2xl border border-slate-200 bg-white p-4 shadow-xs dark:border-slate-800 dark:bg-slate-900">
            <div className="flex items-center gap-2">
              <Globe2 className="h-4 w-4 text-slate-400" />
              <h4 className="text-xs font-semibold text-slate-900 dark:text-slate-100">{world.locationName}</h4>
              <span className="rounded-md bg-slate-100 px-2 py-0.5 text-[10px] font-medium text-slate-600 dark:bg-slate-800 dark:text-slate-300">{world.locationType}</span>
            </div>
            <div className="mt-3 space-y-2 text-xs">
              <div>
                <span className="text-slate-400 block mb-0.5">Sensory details</span>
                <p className="text-slate-700 dark:text-slate-300">{world.sensory}</p>
              </div>
              <div className="flex flex-wrap gap-1.5 pt-1">
                <span className="text-slate-400">Lore references:</span>
                {world.lore.map(l => (
                  <span key={l} className="rounded-full bg-blue-50 px-2 py-0.5 text-[10px] font-medium text-blue-700 dark:bg-blue-950/60 dark:text-blue-300">
                    {l}
                  </span>
                ))}
              </div>
            </div>
            <div className="mt-3 flex justify-end">
              <button
                onClick={() => onFeedback({ id: world.id, tag: 'World', title: world.locationName })}
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
            <span>Next: Characters</span>
            <ArrowRight className="h-4 w-4" />
          </button>
        </div>
      )}
    </div>
  );
}
