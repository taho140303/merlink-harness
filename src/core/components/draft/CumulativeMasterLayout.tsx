import { useDraftStream } from './useDraftStream';
import { mockArcs, mockChapters, mockScenes, mockConfigs, mockWorlds, mockCharacters, mockCharacterDetails } from './mockDraftData';
import { ThemeDisplay } from './shared/ThemeDisplay';
import { ConfigBlock, SceneBlock, WorldBlock, CharacterBlock, CharacterDetailBlock } from './shared/ToggleBlocks';
import type { DraftStep, FeedbackContext } from './types';
import { BookOpen, MessageSquareQuote, ArrowLeft, ArrowRight, CheckCircle2, Play } from 'lucide-react';

export function CumulativeMasterLayout({
  phase,
  onFeedback,
  onNext,
  onBack,
  onStart,
}: {
  phase: DraftStep;
  onFeedback: (f: FeedbackContext) => void;
  onNext?: () => void;
  onBack?: () => void;
  onStart?: () => void;
}) {
  // Xác định tổng số item cần stream cho phase hiện tại
  const streamCount =
    phase === 'chapter' ? mockChapters.length :
    phase === 'scene' ? mockScenes.length :
    phase === 'config' ? mockConfigs.length :
    phase === 'world' ? mockWorlds.length :
    phase === 'character' ? mockCharacters.length :
    phase === 'characterDetail' ? mockCharacterDetails.length : 0;

  const streamedCount = useDraftStream(streamCount, 400);
  const isLoaded = streamedCount >= streamCount;

  // Hàm đệ quy render cây
  let chapterIndex = 0;
  let sceneIndex = 0;
  let configIndex = 0;
  let worldIndex = 0;
  let charIndex = 0;
  let detailIndex = 0;

  return (
    <div className="space-y-6 pb-20">
      {mockArcs.map((arc) => {
        const arcChapters = mockChapters.filter(c => c.arcId === arc.id);
        const arcConfig = mockConfigs.find(c => c.targetScope.includes(arc.name.split(':')[0]));

        return (
          <div key={arc.id} className="overflow-hidden rounded-2xl border border-slate-200 bg-white shadow-xs dark:border-slate-800 dark:bg-slate-900">
            {/* ARC HEADER */}
            <div className="border-b border-slate-100 bg-slate-50/80 p-4 dark:border-slate-800 dark:bg-slate-800/40">
              <div className="flex justify-between items-start mb-2">
                <div>
                  <h3 className="text-sm font-bold text-slate-900 dark:text-slate-100">{arc.name}</h3>
                  <p className="mt-1 text-[11px] leading-relaxed text-slate-600 dark:text-slate-400">{arc.summarize}</p>
                </div>
                <button onClick={() => onFeedback({ id: arc.id, tag: 'Arc', title: arc.name })} className="flex items-center gap-1.5 rounded-lg px-2 py-1 text-[11px] font-medium text-slate-500 hover:bg-slate-200/60 hover:text-blue-600 dark:text-slate-400 dark:hover:bg-slate-800 transition-colors shrink-0">
                  <MessageSquareQuote className="h-3.5 w-3.5" /> Góp ý
                </button>
              </div>

              <ThemeDisplay themes={arc.themes} defaultOpen={false} />

              {/* INJECT: ARC CONFIG */}
              {phase === 'config' && arcConfig && (
                <div className="mt-3">
                  <ConfigBlock
                    config={arcConfig}
                    isStreaming={configIndex++ >= streamedCount}
                    onFeedback={onFeedback}
                  />
                </div>
              )}
            </div>

            {/* CHAPTER LIST */}
            <div className="p-4 sm:p-5 space-y-3">
              <h4 className="text-[11px] font-semibold uppercase tracking-wider text-slate-400 dark:text-slate-500">Chapters</h4>
              {arcChapters.map((chap, idx) => {
                const chapScenes = mockScenes.filter(s => s.chapterId === chap.id);
                const isChapStreaming = phase === 'chapter' && chapterIndex++ >= streamedCount;

                return (
                  <div key={chap.id} className="rounded-xl border border-slate-200 bg-slate-50/50 p-3 dark:border-slate-800 dark:bg-slate-950/20">
                    <div className="flex justify-between items-start">
                      <div>
                        <h5 className="text-xs font-semibold text-slate-800 dark:text-slate-200">Chapter {idx + 1}: {chap.name}</h5>
                        <p className="text-[11px] text-slate-500 dark:text-slate-400 mt-0.5">{chap.description}</p>
                      </div>
                      <button onClick={() => onFeedback({ id: chap.id, tag: 'Chapter', title: chap.name })} className="text-[10px] text-slate-400 hover:text-blue-600 transition-colors">Góp ý</button>
                    </div>

                    {/* INJECT: CHAPTER CONFIG */}
                    {phase === 'config' && mockConfigs.find(c => c.targetScope.includes(chap.name.split(':')[0])) && (
                      <div className="mt-3">
                        <ConfigBlock
                          config={mockConfigs.find(c => c.targetScope.includes(chap.name.split(':')[0]))!}
                          isStreaming={configIndex++ >= streamedCount}
                          onFeedback={onFeedback}
                        />
                      </div>
                    )}

                    {/* INJECT: SCENES */}
                    {chapScenes.length > 0 && phase !== 'config' && phase !== 'chapter' && (
                      <div className="mt-3 space-y-2">
                        {chapScenes.map(scene => {
                          const isSceneStreaming = phase === 'scene' && sceneIndex++ >= streamedCount;
                          const sceneWorld = mockWorlds.find(w => w.id === `w${sceneIndex}`); // Mock mapping
                          const sceneChars = mockCharacters.filter(c => c.appearances.some(a => a.includes(scene.name.split(':')[0])));

                          return (
                            <SceneBlock
                              key={scene.id}
                              scene={scene}
                              isStreaming={isSceneStreaming}
                              phase={phase}
                              onFeedback={onFeedback}
                            >
                              {/* INJECT: WORLD */}
                              {phase === 'world' && sceneWorld && (
                                <WorldBlock
                                  world={sceneWorld}
                                  isStreaming={worldIndex++ >= streamedCount}
                                  onFeedback={onFeedback}
                                />
                              )}

                              {/* INJECT: CHARACTERS */}
                              {(phase === 'character' || phase === 'characterDetail') && sceneChars.map(char => {
                                const isCharStreaming = phase === 'character' && charIndex++ >= streamedCount;
                                const charDetail = mockCharacterDetails.find(cd => cd.name === char.name);

                                return (
                                  <CharacterBlock
                                    key={char.id}
                                    char={char}
                                    isStreaming={isCharStreaming}
                                    phase={phase}
                                    onFeedback={onFeedback}
                                  >
                                    {/* INJECT: DETAIL */}
                                    {phase === 'characterDetail' && charDetail && (
                                      <CharacterDetailBlock
                                        detail={charDetail}
                                        isStreaming={detailIndex++ >= streamedCount}
                                        onFeedback={onFeedback}
                                      />
                                    )}
                                  </CharacterBlock>
                                );
                              })}
                            </SceneBlock>
                          );
                        })}
                      </div>
                    )}
                  </div>
                );
              })}
            </div>
          </div>
        );
      })}

      {/* BOTTOM ACTIONS */}
      {isLoaded && (
        <div className="fixed bottom-6 right-6 z-20 flex items-center gap-2.5 animate-in fade-in-0 duration-300">
          {onBack && (
            <button onClick={onBack} className="flex items-center gap-1.5 rounded-full border border-slate-200 bg-white px-4 py-2.5 text-xs font-semibold text-slate-700 shadow-lg hover:bg-slate-50 dark:border-slate-800 dark:bg-slate-900 dark:text-slate-300 dark:hover:bg-slate-800 transition-all">
              <ArrowLeft className="h-4 w-4" /> Back
            </button>
          )}
          {onNext && (
            <button onClick={onNext} className="flex items-center gap-2 rounded-full bg-slate-900 px-5 py-2.5 text-xs font-semibold text-white shadow-lg shadow-slate-900/20 transition-all hover:scale-105 hover:bg-slate-800 dark:bg-slate-100 dark:text-slate-900 dark:hover:bg-slate-200">
              <span>Next Phase</span>
              <ArrowRight className="h-4 w-4" />
            </button>
          )}
          {onStart && (
            <button onClick={onStart} className="flex items-center gap-2 rounded-full bg-blue-600 px-6 py-2.5 text-xs font-semibold text-white shadow-lg shadow-blue-500/25 transition-all hover:scale-105 hover:bg-blue-500">
              <Play className="h-4 w-4 fill-current" />
              <span>Start Generation</span>
            </button>
          )}
        </div>
      )}
    </div>
  );
}
