import { useDraftStream } from '../useDraftStream';
import { DraftStreamableBlock } from '../DraftStreamableBlock';
import { NestedContextBlock } from '../shared/NestedContextBlock';
import { mockConfigs, mockArcs } from '../mockDraftData';
import type { FeedbackContext } from '../types';
import { MessageSquareQuote, ArrowRight, ArrowLeft, Layers, Settings2 } from 'lucide-react';

export function ConfigStep({ onFeedback, onNext, onBack }: { onFeedback: (f: FeedbackContext) => void; onNext: () => void; onBack: () => void }) {
  const streamed = useDraftStream(mockConfigs.length, 500);
  const isLoaded = streamed >= mockConfigs.length;

  let globalCfgIdx = 0;

  return (
    <div className="space-y-5">
      {mockArcs.map(arc => {
        // Mock Config mapping to Arc (in reality, config might be linked via ID)
        const arcConfigs = mockConfigs.filter(cfg => cfg.targetScope.includes(arc.name.split(':')[0]));
        if (arcConfigs.length === 0) return null;

        return (
          <NestedContextBlock
            key={arc.id}
            title={arc.name}
            subtitle={arc.summarize}
            badge="Arc Context"
            icon={<Layers className="h-4 w-4 text-indigo-500" />}
            defaultOpen={false}
          >
            <h4 className="mb-3 text-[11px] font-semibold uppercase tracking-wider text-slate-400 dark:text-slate-500">
              Configs & Themes
            </h4>
            <div className="space-y-3">
              {arcConfigs.map(cfg => {
                const currentIdx = globalCfgIdx++;
                const isCfgLoaded = currentIdx < streamed;

                return (
                  <DraftStreamableBlock key={cfg.id} isLoaded={isCfgLoaded} loadingTitle="Mapping themes and motifs..." tag="Config">
                    <div className="rounded-xl border border-slate-200 bg-white p-4 shadow-2xs dark:border-slate-800 dark:bg-slate-900">
                      <div className="flex items-center justify-between mb-3">
                        <div className="flex items-center gap-2">
                          <Settings2 className="h-4 w-4 text-slate-400" />
                          <h4 className="text-xs font-semibold text-slate-900 dark:text-slate-100">Scope: {cfg.targetScope}</h4>
                        </div>
                        <button
                          onClick={() => onFeedback({ id: cfg.id, tag: 'Config', title: cfg.targetScope })}
                          className="text-[10px] font-medium text-slate-500 hover:text-blue-600 dark:hover:text-blue-400 transition-colors"
                        >
                          Góp ý
                        </button>
                      </div>
                      <div className="space-y-2 text-xs">
                        <div className="rounded-lg bg-slate-50 p-2.5 dark:bg-slate-800/50">
                          <span className="text-slate-400 block mb-0.5">Theme</span>
                          <span className="font-serif italic text-slate-800 dark:text-slate-200">"{cfg.theme}"</span>
                        </div>
                        <div className="flex flex-wrap gap-1.5">
                          <span className="text-slate-400">Motifs:</span>
                          {cfg.motifs.map(m => (
                            <span key={m} className="rounded-full bg-indigo-50 px-2 py-0.5 text-[10px] font-medium text-indigo-700 dark:bg-indigo-950/60 dark:text-indigo-300">
                              {m}
                            </span>
                          ))}
                        </div>
                      </div>
                    </div>
                  </DraftStreamableBlock>
                );
              })}
            </div>
          </NestedContextBlock>
        );
      })}

      {isLoaded && (
        <div className="fixed bottom-6 right-6 z-20 flex items-center gap-2.5 animate-in fade-in-0 duration-300">
          <button onClick={onBack} className="flex items-center gap-1.5 rounded-full border border-slate-200 bg-white px-4 py-2.5 text-xs font-semibold text-slate-700 shadow-lg hover:bg-slate-50 dark:border-slate-800 dark:bg-slate-900 dark:text-slate-300 dark:hover:bg-slate-800 transition-all">
            <ArrowLeft className="h-4 w-4" /> Back
          </button>
          <button onClick={onNext} className="flex items-center gap-2 rounded-full bg-slate-900 px-5 py-2.5 text-xs font-semibold text-white shadow-lg shadow-slate-900/20 transition-all hover:scale-105 hover:bg-slate-800 dark:bg-slate-100 dark:text-slate-900 dark:hover:bg-slate-200">
            <span>Next: World</span>
            <ArrowRight className="h-4 w-4" />
          </button>
        </div>
      )}
    </div>
  );
}
