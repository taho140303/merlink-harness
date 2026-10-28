import { useState } from 'react';
import { ChevronDown, ChevronUp, Film } from 'lucide-react';
import { cn } from '../../../../shared/utils/cn';
import { useT } from '../../../../shared/i18n/useT';
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
                        <div className="border-t border-slate-200 p-3 dark:border-slate-700 space-y-3">
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
  const t = useT();
  
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
        "rounded-md border border-slate-200 bg-white p-4 dark:border-slate-700 dark:bg-slate-900 transition-all duration-300",
        isFeedbackMode && "hover:!border-2 hover:!border-dashed hover:!border-purple-500 hover:bg-purple-500/5 cursor-none"
      )}
    >
      <div className="flex items-center gap-2 mb-2">
        <Film className="h-4 w-4 text-blue-500" />
        <span className="text-sm font-bold text-slate-900 dark:text-slate-100">
          Cảnh {scene.position}: {scene.displayName}
        </span>
        <span className="rounded bg-slate-100 px-1.5 py-0.5 text-xs font-mono text-slate-500 dark:bg-slate-800 dark:text-slate-400">{scene.code}</span>
      </div>

      <p className="text-sm text-slate-600 dark:text-slate-400 mb-4 leading-relaxed">
        {scene.summary}
      </p>

      {/* Metadata: Title Group + Text */}
      <div className="grid grid-cols-1 sm:grid-cols-3 gap-4 pt-3 border-t border-slate-100 dark:border-slate-800">
        <div>
          <span className="text-[11px] font-bold uppercase tracking-wider text-slate-400">
            {t('draft.scene.stylePreset')}
          </span>
          <p className="mt-1 text-sm font-medium text-slate-800 dark:text-slate-200">
            {scene.stylePresetCode}
          </p>
        </div>
        <div>
          <span className="text-[11px] font-bold uppercase tracking-wider text-slate-400">
            {t('draft.scene.narrator')}
          </span>
          <p className="mt-1 text-sm font-medium text-slate-800 dark:text-slate-200">
            {scene.narratorConfigCode}
          </p>
        </div>
        <div>
          <span className="text-[11px] font-bold uppercase tracking-wider text-slate-400">
            {t('draft.scene.lore')}
          </span>
          <p className={cn(
            "mt-1 text-sm font-medium",
            scene.loreEntriesCode 
              ? "text-slate-800 dark:text-slate-200" 
              : "text-slate-400 italic"
          )}>
            {scene.loreEntriesCode || t('draft.scene.none')}
          </p>
        </div>
      </div>

      {/* Characters Group */}
      <div className="mt-4">
        <span className="text-[11px] font-bold uppercase tracking-wider text-slate-400">
          {t('draft.scene.characters')}
        </span>
        <div className="mt-1 flex flex-wrap gap-x-4 gap-y-1">
          {scene.characters.map((char, i) => (
            <span
              key={i}
              className={cn(
                "text-sm",
                char.isPov
                  ? "font-bold text-indigo-600 dark:text-indigo-400"
                  : "text-slate-700 dark:text-slate-300"
              )}
            >
              {char.displayName} {char.isPov && "(POV)"}
            </span>
          ))}
        </div>
      </div>
    </div>
  );
}
