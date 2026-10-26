import { useDraftStream } from '../useDraftStream';
import { DraftStreamableBlock } from '../DraftStreamableBlock';
import { mockCharacterDetails } from '../mockDraftData';
import type { FeedbackContext } from '../types';
import { MessageSquareQuote, ArrowLeft, Play, CheckCircle2 } from 'lucide-react';

export function CharacterDetailStep({ onFeedback, onBack, onStart }: { onFeedback: (f: FeedbackContext) => void; onBack: () => void; onStart: () => void }) {
  const streamed = useDraftStream(mockCharacterDetails.length, 600);
  const isLoaded = streamed >= mockCharacterDetails.length;

  return (
    <div className="space-y-4">
      {mockCharacterDetails.map((cd, idx) => (
        <DraftStreamableBlock key={cd.id} isLoaded={idx < streamed} loadingTitle="Simulating psychological profiles..." tag="Detail">
          <div className="rounded-2xl border border-slate-200 bg-white p-4 shadow-xs dark:border-slate-800 dark:bg-slate-900">
            <h4 className="text-sm font-semibold text-slate-900 dark:text-slate-100 border-b border-slate-100 pb-2 dark:border-slate-800">
              {cd.name}
            </h4>
            
            <div className="mt-3 space-y-3 text-xs">
              <div>
                <span className="text-slate-400 block mb-0.5">Personality & Action Logic</span>
                <p className="text-slate-700 dark:text-slate-300"><span className="font-medium text-slate-900 dark:text-slate-100">Tính cách:</span> {cd.personality}</p>
                <p className="mt-1 text-slate-700 dark:text-slate-300"><span className="font-medium text-slate-900 dark:text-slate-100">Hành động:</span> {cd.actionLogic}</p>
              </div>
              <div className="rounded-lg bg-indigo-50/50 border border-indigo-100 p-2.5 dark:bg-indigo-950/30 dark:border-indigo-900/60">
                <span className="text-indigo-700 dark:text-indigo-300 font-semibold block mb-0.5">Theme Alignment</span>
                <p className="text-indigo-900 dark:text-indigo-100">{cd.themeAlignment}</p>
              </div>
              <div>
                <span className="text-slate-400 block mb-0.5">Scene Role</span>
                <p className="text-slate-700 dark:text-slate-300">{cd.sceneRole}</p>
              </div>
            </div>

            <div className="mt-3 flex justify-end">
              <button
                onClick={() => onFeedback({ id: cd.id, tag: 'Detail', title: cd.name })}
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
          <button onClick={onStart} className="flex items-center gap-2 rounded-full bg-blue-600 px-6 py-2.5 text-xs font-semibold text-white shadow-lg shadow-blue-500/25 transition-all hover:scale-105 hover:bg-blue-500">
            <Play className="h-4 w-4 fill-current" />
            <span>Start Generation</span>
          </button>
        </div>
      )}
    </div>
  );
}
