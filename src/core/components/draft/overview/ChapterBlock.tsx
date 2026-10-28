import { useState } from 'react';
import { ChevronDown, ChevronUp, Layers, Play } from 'lucide-react';
import { useT } from '../../../../shared/i18n/useT';
import { cn } from '../../../../shared/utils/cn';
import type { DraftChapter } from '../types';

interface ChapterBlockProps {
  chapters: DraftChapter[];
  isFeedbackMode?: boolean;
  onAttachFeedback?: (id: string, tag: string, title: string) => void;
}

export function ChapterBlock({ chapters, isFeedbackMode, onAttachFeedback }: ChapterBlockProps) {
  const t = useT();
  const [expanded, setExpanded] = useState(false);
  const [expandedChapters, setExpandedChapters] = useState<Set<number>>(new Set());

  if (!chapters || chapters.length === 0) return null;

  const toggleChapter = (pos: number) => {
    setExpandedChapters(prev => {
      const next = new Set(prev);
      if (next.has(pos)) next.delete(pos);
      else next.add(pos);
      return next;
    });
  };

  const handleAttach = (e: React.MouseEvent, tag: string, title: string, id: string) => {
    if (isFeedbackMode && onAttachFeedback) {
      e.stopPropagation();
      onAttachFeedback(id, tag, title);
    }
  };

  return (
    <div 
      onClick={(e) => handleAttach(e, 'chapters', 'Chapters & Scenes', 'chapters')}
      className={cn(
        "rounded-xl border border-slate-200 bg-white dark:border-slate-800 dark:bg-slate-900 overflow-hidden transition-all duration-300",
        isFeedbackMode && "hover:!border-2 hover:!border-dashed hover:!border-purple-500 hover:bg-purple-500/5 cursor-none"
      )}
    >
      <button
        type="button"
        onClick={(e) => {
          if (isFeedbackMode) {
            handleAttach(e, 'chapters', 'Chapters & Scenes', 'chapters');
          } else {
            setExpanded(!expanded);
          }
        }}
        className="flex w-full items-center justify-between p-3 hover:bg-slate-50 dark:hover:bg-slate-800/50 transition-colors"
      >
        <div className="flex items-center gap-2">
          <Layers className="h-4 w-4 text-amber-500" />
          <div className="flex flex-col items-start text-left">
            <span className="text-base font-semibold text-slate-900 dark:text-slate-100">{t('draft.chapter.title')}</span>
            <span className="text-[11px] text-slate-500 dark:text-slate-400">{t('draft.chapter.subtitle')}</span>
          </div>
        </div>
        <div className="flex items-center gap-2">
          <span className="rounded-full bg-amber-100 px-2 py-0.5 text-xs font-medium text-amber-700 dark:bg-amber-950/60 dark:text-amber-300">
            {t('draft.chapter.count', { count: chapters.length })}
          </span>
          {expanded ? <ChevronUp className="h-4 w-4 text-slate-400" /> : <ChevronDown className="h-4 w-4 text-slate-400" />}
        </div>
      </button>

      {expanded && (
        <div className="border-t border-slate-100 p-3 space-y-3 dark:border-slate-800">
          {chapters.map((chap) => {
            const isChapExpanded = expandedChapters.has(chap.position);
            return (
              <div key={chap.position} className="rounded-lg border border-slate-200 bg-slate-50/50 dark:border-slate-700 dark:bg-slate-800/30 overflow-hidden">
                <button
                  type="button"
                  onClick={(e) => {
                    if (isFeedbackMode) {
                      handleAttach(e, 'chapter', chap.displayName, chap.position.toString());
                    } else {
                      toggleChapter(chap.position);
                    }
                  }}
                  className="flex w-full items-center justify-between p-2.5 hover:bg-slate-100 dark:hover:bg-slate-800 transition-colors"
                >
                  <div className="flex items-center gap-2">
                    <span className="flex h-5 w-5 items-center justify-center rounded bg-white text-xs font-bold text-slate-700 shadow-xs dark:bg-slate-900 dark:text-slate-300">
                      {chap.position}
                    </span>
                    <div className="flex flex-col items-start text-left">
                      <span className="text-sm font-bold text-slate-900 dark:text-slate-100">{chap.displayName}</span>
                      <span className="text-sm text-slate-500 dark:text-slate-400">{chap.summary}</span>
                    </div>
                  </div>
                  <div className="flex items-center gap-2">
                    <span className="flex items-center gap-1 text-xs font-medium text-slate-500">
                      <Play className="h-3 w-3" /> {chap.scenes.length} scenes
                    </span>
                    {isChapExpanded ? <ChevronUp className="h-3 w-3 text-slate-400" /> : <ChevronDown className="h-3 w-3 text-slate-400" />}
                  </div>
                </button>

                {isChapExpanded && (
                  <div className="border-t border-slate-200 p-2.5 dark:border-slate-700 space-y-2">
                    {chap.scenes.map(scene => (
                      <div key={scene.code} className="flex items-start gap-2.5 rounded-md bg-white p-2 shadow-xs dark:bg-slate-900">
                        <div className="flex h-5 w-5 shrink-0 items-center justify-center rounded bg-slate-100 text-[11px] font-mono text-slate-500 dark:bg-slate-800 dark:text-slate-400">
                          {scene.position}
                        </div>
                        <div>
                          <div className="flex items-center gap-2">
                            <span className="text-base font-bold text-slate-800 dark:text-slate-200">{scene.displayName}</span>
                            <span className="rounded bg-slate-100 px-1 py-0.5 text-[11px] font-mono text-slate-500 dark:bg-slate-800">{scene.code}</span>
                          </div>
                          <p className="mt-0.5 text-base text-slate-500 dark:text-slate-400">{scene.summary}</p>
                        </div>
                      </div>
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
}
