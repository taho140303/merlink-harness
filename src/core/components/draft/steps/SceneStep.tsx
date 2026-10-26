import { useDraftStream } from '../useDraftStream';
import { DraftStreamableBlock } from '../DraftStreamableBlock';
import { NestedContextBlock } from '../shared/NestedContextBlock';
import { mockScenes, mockChapters } from '../mockDraftData';
import type { FeedbackContext } from '../types';
import { MessageSquareQuote, ArrowRight, ArrowLeft, BookOpen, Film, ShieldAlert, CheckCircle } from 'lucide-react';

export function SceneStep({ onFeedback, onNext, onBack }: { onFeedback: (f: FeedbackContext) => void; onNext: () => void; onBack: () => void }) {
  const streamed = useDraftStream(mockScenes.length, 450);
  const isLoaded = streamed >= mockScenes.length;

  let globalSceneIdx = 0;

  return (
    <div className="space-y-5">
      {mockChapters.map(chapter => {
        const chapterScenes = mockScenes.filter(s => s.chapterId === chapter.id);
        if (chapterScenes.length === 0) return null;

        return (
          <NestedContextBlock
            key={chapter.id}
            title={chapter.name}
            subtitle={chapter.description}
            badge="Chapter"
            icon={<BookOpen className="h-4 w-4 text-indigo-500" />}
            defaultOpen={false}
          >
            <h4 className="mb-3 text-[11px] font-semibold uppercase tracking-wider text-slate-400 dark:text-slate-500">
              Scenes
            </h4>
            <div className="space-y-3">
              {chapterScenes.map(scene => {
                const currentIdx = globalSceneIdx++;
                const isSceneLoaded = currentIdx < streamed;

                return (
                  <DraftStreamableBlock key={scene.id} isLoaded={isSceneLoaded} loadingTitle="Configuring scene parameters..." tag="Scene">
                    <div className="rounded-xl border border-slate-200 bg-white p-4 shadow-2xs dark:border-slate-800 dark:bg-slate-900">
                      <div className="flex items-center justify-between">
                        <h4 className="text-xs font-semibold text-slate-900 dark:text-slate-100 flex items-center gap-2">
                          <Film className="h-3.5 w-3.5 text-slate-400" />
                          {scene.name}
                        </h4>
                        <button
                          onClick={() => onFeedback({ id: scene.id, tag: 'Scene', title: scene.name })}
                          className="text-[10px] font-medium text-slate-500 hover:text-blue-600 dark:hover:text-blue-400 transition-colors"
                        >
                          Góp ý
                        </button>
                      </div>
                      <p className="mt-1.5 text-xs text-slate-600 dark:text-slate-400">{scene.summarize}</p>
                      
                      <div className="mt-3 grid grid-cols-2 gap-2 text-[11px]">
                        <div className="rounded-lg bg-slate-50 p-2 dark:bg-slate-800/50">
                          <span className="text-slate-400 block mb-0.5">Style Preset</span>
                          <span className="font-medium text-slate-700 dark:text-slate-300">{scene.stylePreset}</span>
                        </div>
                        <div className="rounded-lg bg-slate-50 p-2 dark:bg-slate-800/50">
                          <span className="text-slate-400 block mb-0.5">Narrator / POV</span>
                          <span className="font-medium text-slate-700 dark:text-slate-300">{scene.narratorConfig}</span>
                        </div>
                      </div>

                      <div className="mt-3 flex flex-wrap gap-1.5">
                        {scene.characters.map(c => (
                          <span key={c} className="rounded-md bg-blue-100 px-2 py-0.5 text-[10px] font-medium text-blue-700 dark:bg-blue-950/60 dark:text-blue-300">
                            {c}
                          </span>
                        ))}
                      </div>

                      <div className="mt-3 grid grid-cols-2 gap-2 text-[10px]">
                        <div className="rounded-lg border border-rose-100 bg-rose-50 p-2 dark:border-rose-900/60 dark:bg-rose-950/30">
                          <span className="font-semibold text-rose-700 dark:text-rose-300 flex items-center gap-1 mb-1"><ShieldAlert className="h-3 w-3" /> Forbidden</span>
                          <ul className="list-disc pl-3 text-rose-900 dark:text-rose-100">
                            {scene.forbidden.map(f => <li key={f}>{f}</li>)}
                          </ul>
                        </div>
                        <div className="rounded-lg border border-emerald-100 bg-emerald-50 p-2 dark:border-emerald-900/60 dark:bg-emerald-950/30">
                          <span className="font-semibold text-emerald-700 dark:text-emerald-300 flex items-center gap-1 mb-1"><CheckCircle className="h-3 w-3" /> Allowed</span>
                          <ul className="list-disc pl-3 text-emerald-900 dark:text-emerald-100">
                            {scene.allowed.map(a => <li key={a}>{a}</li>)}
                          </ul>
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
            <span>Next: Config</span>
            <ArrowRight className="h-4 w-4" />
          </button>
        </div>
      )}
    </div>
  );
}
