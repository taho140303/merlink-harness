import { ChevronDown, ChevronUp, Tags } from 'lucide-react';
import { useT } from '../../../../shared/i18n/useT';
import { cn } from '../../../../shared/utils/cn';
import { ThemeBlock } from './ThemeBlock';
import { SegmentBlock } from './SegmentBlock';
import { ChapterBlock } from './ChapterBlock';
import type { DraftArc } from '../types';

interface ArcCardProps {
  arc: DraftArc;
  isExpanded: boolean;
  onToggle: () => void;
  isFeedbackMode?: boolean;
  onAttachFeedback?: (id: string, tag: string, title: string) => void;
}

export function ArcCard({ arc, isExpanded, onToggle, isFeedbackMode, onAttachFeedback }: ArcCardProps) {
  const t = useT();
  
  const handleAttach = (e: React.MouseEvent) => {
    if (isFeedbackMode && onAttachFeedback) {
      e.stopPropagation();
      onAttachFeedback(arc.id, 'arc', arc.title);
    }
  };

  return (
    <div 
      onClick={handleAttach}
      className={cn(
        "tier-card rounded-card    overflow-hidden transition-colors duration-300",
        isFeedbackMode && "hover:!border-2 hover:!border-dashed hover:!border-purple-500 hover:bg-purple-500/5 cursor-none"
      )}
    >
      <button
        type="button"
        onClick={(e) => {
          if (isFeedbackMode) {
            handleAttach(e);
          } else {
            onToggle();
          }
        }}
        className="flex w-full items-center justify-between p-4 hover:bg-surface-muted  transition-colors"
      >
        <div className="flex items-center gap-3">
          <span className="flex h-8 w-8 items-center justify-center rounded-lg bg-indigo-100 font-bold text-indigo-700 dark:bg-indigo-950/60 dark:text-indigo-300">
            {arc.number}
          </span>
          <div className="flex flex-col items-start text-left">
            <h4 className="text-base font-bold text-fg">{arc.title}</h4>
            <span className="text-xs text-fg-muted">{arc.subnote}</span>
          </div>
        </div>
        <div className="flex items-center gap-3">
          <span className="rounded-full bg-surface-muted px-2 py-0.5 text-xs font-medium text-fg-muted  ">
            {t('draft.arc.segmentsCount', { count: arc.segmentsCount })}
          </span>
          {isExpanded ? <ChevronUp className="h-4 w-4 text-fg-subtle" /> : <ChevronDown className="h-4 w-4 text-fg-subtle" />}
        </div>
      </button>

      {isExpanded && (
        <div className="border-t border-line p-4  space-y-4">
          <div className="flex flex-col gap-2">
            <p className="text-base leading-relaxed text-fg-muted">{arc.summary}</p>
            {arc.motif && (
              <div className="flex flex-wrap items-center gap-1.5 pt-1">
                <Tags className="h-3.5 w-3.5 text-fg-subtle" />
                <span className="text-xs font-semibold uppercase tracking-wider text-fg-subtle">Motif:</span>
                <span className="rounded-md bg-indigo-50 px-2 py-0.5 text-sm font-medium text-indigo-700 dark:bg-indigo-950/60 dark:text-indigo-300">
                  {arc.motif.displayName} ({arc.motif.category})
                </span>
                {arc.motif.note && (
                  <span className="text-sm text-fg-subtle italic">— {arc.motif.note}</span>
                )}
              </div>
            )}
          </div>
          
          <ThemeBlock 
            theme={arc.theme} 
            isFeedbackMode={isFeedbackMode} 
            onAttachFeedback={onAttachFeedback ? () => onAttachFeedback(arc.id, 'theme', arc.title) : undefined} 
          />
          
          <SegmentBlock 
            segments={arc.segments} 
            isFeedbackMode={isFeedbackMode} 
            onAttachFeedback={onAttachFeedback ? () => onAttachFeedback(arc.id, 'segment', arc.title) : undefined} 
          />
          
          {arc.chapters && arc.chapters.length > 0 && (
            <ChapterBlock 
              chapters={arc.chapters} 
              isFeedbackMode={isFeedbackMode} 
              onAttachFeedback={onAttachFeedback ? () => onAttachFeedback(arc.id, 'chapter', arc.title) : undefined} 
            />
          )}
        </div>
      )}
    </div>
  );
}
