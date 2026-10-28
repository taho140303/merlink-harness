import { useState } from 'react';
import { ChevronDown, ChevronUp, Film, User, BookOpen, Mic2 } from 'lucide-react';
import { cn } from '../../../../shared/utils/cn';
import type { SceneConfigPhase, SceneConfig } from '../types';

interface SceneConfigBlockProps {
  config: SceneConfigPhase;
  isFeedbackMode?: boolean;
  onAttachFeedback?: (id: string, tag: string, title: string) => void;
}

export function SceneConfigBlock({ config, isFeedbackMode, onAttachFeedback }: SceneConfigBlockProps) {
  const [expandedArcs, setExpandedArcs] = useState<Set<number>>(new Set([0]));
  const [expandedChapters, setExpandedChapters] = useState<Set<string>>(new Set());

  const toggleArc = (idx: number) => {
    setExpandedArcs(prev => {
      const next = new Set(prev);
      if (next.has(idx)) next.delete(idx); else next.add(idx);
      return next;
    });
  };

  const toggleChapter = (key: string) => {
    setExpandedChapters(prev => {
      const next = new Set(prev);
      if (next.has(key)) next.delete(key); else next.add(key);
      return next;
    });
  };

  const handleAttach = (e: React.MouseEvent, id: string, tag: string, title: string) => {
    if (isFeedbackMode && onAttachFeedback) {
      e.stopPropagation();
      onAttachFeedback(id, tag, title);
    }
  };

  return (
    <div className="w-full space-y-4">
      {config.arcs.map((arc, arcIdx) => {
        const isArcExpanded = expandedArcs.has(arcIdx);
        return (
          <div
            key={arcIdx}
            className={cn(
              "rounded-xl border border-slate-200 bg-white dark:border-slate-800 dark:bg-slate-900 overflow-hidden transition-all duration-300",
              isFeedbackMode && "hover:!border-2 hover:!border-dashed hover:!border-purple-500 hover:bg-purple-500/5 cursor-none"
            )}
            onClick={(e) => handleAttach(e, `arc-${arcIdx}`, 'arc', arc.displayName)}
          >
            <button
              type="button"
              onClick={(e) => { e.stopPropagation(); toggleArc(arcIdx); }}
              className="flex w-full items-center justify-between p-4 hover:bg-slate-50 dark:hover:bg-slate-800/50 transition-colors"
            >
              <div className="flex items-center gap-3">
                <span className="flex h-8 w-8 items-center justify-center rounded-lg bg-indigo-100 font-bold text-indigo-700 dark:bg-indigo-950/60 dark:text-indigo-300">
                  {arc.position}
                </span>
                <div className="text-left">
                  <h3 className="text-base font-bold text-slate-900 dark:text-slate-100">{arc.displayName}</h3>
                  <p className="text-sm text-slate-500 dark:text-slate-400">{arc.chapters.length} chương</p>
                </div>
              </div>
              {isArcExpanded ? <ChevronUp className="h-5 w-5 text-slate-400" /> : <ChevronDown className="h-5 w-5 text-slate-400" />}
            </button>

            {isArcExpanded && (
              <div className="border-t border-slate-100 p-4 dark:border-slate-800 space-y-3">
                {arc.chapters.map((chap, chapIdx) => {
                  const chapKey = `${arcIdx}-${chapIdx}`;
                  const isChapExpanded = expandedChapters.has(chapKey);
                  return (
                    <div
                      key={chapKey}
                      className="rounded-lg border border-slate-200 bg-slate-50/50 dark:border-slate-700 dark:bg-slate-800/30 overflow-hidden"
                    >
                      <button
                        type="button"
                        onClick={(e) => { e.stopPropagation(); toggleChapter(chapKey); }}
                        className="flex w-full items-center justify-between p-3 hover:bg-slate-100 dark:hover:bg-slate-800 transition-colors"
                      >
                        <div className="flex items-center gap-2">
                          <span className="flex h-6 w-6 items-center justify-center rounded bg-white text-xs font-bold text-slate-700 shadow-xs dark:bg-slate-900 dark:text-slate-300">
                            {chap.position}
                          </span>
                          <span className="text-sm font-semibold text-slate-900 dark:text-slate-100">{chap.displayName}</span>
                          <span className="text-xs text-slate-500 dark:text-slate-400">({chap.scenes.length} cảnh)</span>
                        </div>
                        {isChapExpanded ? <ChevronUp className="h-4 w-4 text-slate-400" /> : <ChevronDown className="h-4 w-4 text-slate-400" />}
                      </button>

                      {isChapExpanded && (
                        <div className="border-t border-slate-200 p-3 dark:border-slate-700 space-y-2">
                          {chap.scenes.map((scene) => (
                            <SceneCard
                              key={scene.code}
                              scene={scene}
                              isFeedbackMode={isFeedbackMode}
                              onAttachFeedback={onAttachFeedback}
                            />
                          ))}
                        </div>
                      )}
                    </div>
                  );
                })}
              </div>
            )}
          </div>
        );
      })}
    </div>
  );
}

function SceneCard({ scene, isFeedbackMode, onAttachFeedback }: { scene: SceneConfig; isFeedbackMode?: boolean; onAttachFeedback?: (id: string, tag: string, title: string) => void }) {
  const handleAttach = (e: React.MouseEvent) => {
    if (isFeedbackMode && onAttachFeedback) {
      e.stopPropagation();
      onAttachFeedback(scene.code, 'scene', scene.displayName);
    }
  };

  return (
    <div
      onClick={handleAttach}
      className={cn(
        "rounded-md border border-slate-200 bg-white p-3 dark:border-slate-700 dark:bg-slate-900 transition-all duration-300",
        isFeedbackMode && "hover:!border-2 hover:!border-dashed hover:!border-purple-500 hover:bg-purple-500/5 cursor-none"
      )}
    >
      <div className="flex items-start justify-between gap-3">
        <div className="flex-1">
          <div className="flex items-center gap-2 mb-2">
            <Film className="h-4 w-4 text-blue-500" />
            <span className="text-sm font-bold text-slate-900 dark:text-slate-100">
              Cảnh {scene.position}: {scene.displayName}
            </span>
            <span className="rounded bg-slate-100 px-1.5 py-0.5 text-xs font-mono text-slate-500 dark:bg-slate-800 dark:text-slate-400">{scene.code}</span>
          </div>

          <div className="flex flex-wrap gap-2 mt-2">
            <span className="flex items-center gap-1 rounded-full bg-indigo-50 px-2 py-0.5 text-xs font-medium text-indigo-700 dark:bg-indigo-950/60 dark:text-indigo-300">
              <BookOpen className="h-3 w-3" /> {scene.stylePresetCode}
            </span>
            <span className="flex items-center gap-1 rounded-full bg-emerald-50 px-2 py-0.5 text-xs font-medium text-emerald-700 dark:bg-emerald-950/60 dark:text-emerald-300">
              <Mic2 className="h-3 w-3" /> {scene.narratorConfigCode}
            </span>
            <span className="flex items-center gap-1 rounded-full bg-amber-50 px-2 py-0.5 text-xs font-medium text-amber-700 dark:bg-amber-950/60 dark:text-amber-300">
              <BookOpen className="h-3 w-3" /> {scene.loreEntriesCode}
            </span>
          </div>

          <div className="mt-2 flex flex-wrap gap-1.5">
            {scene.characters.map((char, i) => (
              <span
                key={i}
                className={cn(
                  "flex items-center gap-1 rounded-full px-2 py-0.5 text-xs",
                  char.isPov
                    ? "bg-purple-100 text-purple-700 dark:bg-purple-950/60 dark:text-purple-300 font-semibold"
                    : "bg-slate-100 text-slate-600 dark:bg-slate-800 dark:text-slate-300"
                )}
              >
                {char.isPov && <User className="h-3 w-3" />}
                {char.displayName}
              </span>
            ))}
          </div>
        </div>
      </div>
    </div>
  );
}
