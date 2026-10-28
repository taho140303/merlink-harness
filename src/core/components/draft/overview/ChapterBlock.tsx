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
        "tier-card rounded-card overflow-hidden transition-colors duration-300",
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
        aria-expanded={expanded}
        className="focus-ring flex w-full items-center justify-between p-3 hover:bg-surface-muted  transition-colors"
      >
        <div className="flex items-center gap-2">
          <Layers className="h-4 w-4 text-amber-500" />
          <div className="flex flex-col items-start text-left">
            <span className="text-base font-semibold text-fg">{t('draft.chapter.title')}</span>
            <span className="text-xs text-fg-muted">{t('draft.chapter.subtitle')}</span>
          </div>
        </div>
        <div className="flex items-center gap-2">
          <span className="rounded-full bg-amber-100 px-2 py-0.5 text-xs font-medium text-amber-700 dark:bg-amber-950/60 dark:text-amber-300">
            {t('draft.chapter.count', { count: chapters.length })}
          </span>
          {expanded ? <ChevronUp aria-hidden="true" className="h-4 w-4 text-fg-subtle" /> : <ChevronDown aria-hidden="true" className="h-4 w-4 text-fg-subtle" />}
        </div>
      </button>

      {expanded && (
        <div className="border-t border-line p-3 space-y-3 ">
          {chapters.map((chap) => {
            const isChapExpanded = expandedChapters.has(chap.position);
            return (
              <div key={chap.position} className="rounded-lg border border-line bg-surface-muted/50   overflow-hidden">
                <button
                  type="button"
                  onClick={(e) => {
                    if (isFeedbackMode) {
                      handleAttach(e, 'chapter', chap.displayName, chap.position.toString());
                    } else {
                      toggleChapter(chap.position);
                    }
                  }}
                  aria-expanded={isChapExpanded}
                  className="focus-ring flex w-full items-center justify-between p-2.5 hover:bg-surface-muted  transition-colors"
                >
                  <div className="flex items-center gap-2">
                    <span className="flex h-5 w-5 items-center justify-center rounded bg-surface-card text-xs font-bold text-fg-muted   ">
                      {chap.position}
                    </span>
                    <div className="flex flex-col items-start text-left">
                      <span className="text-sm font-bold text-fg">{chap.displayName}</span>
                      <span className="text-sm text-fg-muted">{chap.summary}</span>
                    </div>
                  </div>
                  <div className="flex items-center gap-2">
                    <span className="flex items-center gap-1 text-xs font-medium text-fg-subtle">
                      <Play className="h-3 w-3" /> {chap.scenes.length} scenes
                    </span>
                    {isChapExpanded ? <ChevronUp className="h-3 w-3 text-fg-subtle" /> : <ChevronDown className="h-3 w-3 text-fg-subtle" />}
                  </div>
                </button>

                {isChapExpanded && (
                  <div className="border-t border-line p-2.5  space-y-2">
                    {chap.scenes.map(scene => (
                      <div key={scene.code} className="flex items-start gap-2.5 rounded-md bg-surface-card p-2  ">
                        <div className="flex h-5 w-5 shrink-0 items-center justify-center rounded bg-surface-muted text-xs font-mono text-fg-subtle  -subtle">
                          {scene.position}
                        </div>
                        <div>
                          <div className="flex items-center gap-2">
                            <span className="text-base font-bold text-fg ">{scene.displayName}</span>
                            <span className="rounded bg-surface-muted px-1 py-0.5 text-xs font-mono text-fg-subtle ">{scene.code}</span>
                          </div>
                          <p className="mt-0.5 text-base text-fg-muted">{scene.summary}</p>
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
