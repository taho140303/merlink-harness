import { useDraftStream } from '../useDraftStream';
import { DraftStreamableBlock } from '../DraftStreamableBlock';
import { NestedContextBlock } from '../shared/NestedContextBlock';
import { mockCharacterDetails, mockCharacters } from '../mockDraftData';
import type { FeedbackContext } from '../types';
import { MessageSquareQuote, ArrowLeft, Play, User } from 'lucide-react';

export function CharacterDetailStep({ onFeedback, onBack, onStart }: { onFeedback: (f: FeedbackContext) => void; onBack: () => void; onStart: () => void }) {
  const streamed = useDraftStream(mockCharacterDetails.length, 600);
  const isLoaded = streamed >= mockCharacterDetails.length;

  let globalDetailIdx = 0;

  return (
    <div className="space-y-5">
      {mockCharacters.map(char => {
        const charDetail = mockCharacterDetails.find(cd => cd.name === char.name);
        if (!charDetail) return null;

        const currentIdx = globalDetailIdx++;
        const isDetailLoaded = currentIdx < streamed;

        return (
          <NestedContextBlock
            key={char.id}
            title={char.name}
            subtitle={`Role: ${char.role}`}
            badge="Character Context"
            icon={<User className="h-4 w-4 text-indigo-500" />}
            defaultOpen={false}
          >
            <h4 className="mb-3 text-[11px] font-semibold uppercase tracking-wider text-slate-400 dark:text-slate-500">
              Psychological Profile
            </h4>
            <DraftStreamableBlock isLoaded={isDetailLoaded} loadingTitle="Simulating psychological profiles..." tag="Detail">
              <div className="rounded-xl border border-slate-200 bg-white p-4 shadow-2xs dark:border-slate-800 dark:bg-slate-900">
                <div className="flex items-center justify-between border-b border-slate-100 pb-2 dark:border-slate-800 mb-3">
                  <h4 className="text-xs font-semibold text-slate-900 dark:text-slate-100">Deep Dive Analysis</h4>
                  <button
                    onClick={() => onFeedback({ id: charDetail.id, tag: 'Detail', title: charDetail.name })}
                    className="text-[10px] font-medium text-slate-500 hover:text-blue-600 dark:hover:text-blue-400 transition-colors"
                  >
                    Góp ý
                  </button>
                </div>
                
                <div className="space-y-3 text-xs">
                  <div>
                    <span className="text-slate-400 block mb-0.5">Personality & Action Logic</span>
                    <p className="text-slate-700 dark:text-slate-300"><span className="font-medium text-slate-900 dark:text-slate-100">Tính cách:</span> {charDetail.personality}</p>
                    <p className="mt-1 text-slate-700 dark:text-slate-300"><span className="font-medium text-slate-900 dark:text-slate-100">Hành động:</span> {charDetail.actionLogic}</p>
                  </div>
                  <div className="rounded-lg bg-indigo-50/50 border border-indigo-100 p-2.5 dark:bg-indigo-950/30 dark:border-indigo-900/60">
                    <span className="text-indigo-700 dark:text-indigo-300 font-semibold block mb-0.5">Theme Alignment</span>
                    <p className="text-indigo-900 dark:text-indigo-100">{charDetail.themeAlignment}</p>
                  </div>
                  <div>
                    <span className="text-slate-400 block mb-0.5">Scene Role</span>
                    <p className="text-slate-700 dark:text-slate-300">{charDetail.sceneRole}</p>
                  </div>
                </div>
              </div>
            </DraftStreamableBlock>
          </NestedContextBlock>
        );
      })}

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
