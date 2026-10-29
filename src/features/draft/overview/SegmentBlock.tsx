import { useState } from 'react';
import { List, LayoutGrid, ChevronDown, ChevronUp, GitBranch } from 'lucide-react';
import { cn } from '@/shared/utils/cn';
import { useT } from '@/shared/i18n/useT';
import type { DraftSegment } from '../types';

interface SegmentBlockProps {
  segments: DraftSegment[];
  isFeedbackMode?: boolean;
  onAttachFeedback?: (id: string, tag: string, title: string) => void;
}

export function SegmentBlock({ segments, isFeedbackMode, onAttachFeedback }: SegmentBlockProps) {
  const t = useT();
  const [expanded, setExpanded] = useState(false);
  const [viewMode, setViewMode] = useState<'timeline' | 'grid'>('timeline');
  const [showAll, setShowAll] = useState(false);

  const displaySegments = showAll ? segments : segments.slice(0, 3);
  const hasMore = segments.length > 3;

  const handleAttach = (e: React.MouseEvent) => {
    if (isFeedbackMode && onAttachFeedback) {
      e.stopPropagation();
      onAttachFeedback('segments', 'segments', 'Event Segments');
    }
  };

  return (
    <div 
      onClick={handleAttach}
      className={cn(
        "tier-card rounded-card overflow-hidden transition-colors duration-300",
        isFeedbackMode && "hover:!border-2 hover:!border-dashed hover:!border-purple-500 hover:bg-purple-500/5 cursor-none"
      )}
    >
      <button
        type="button"
        onClick={(e) => {
          if (isFeedbackMode) {
            handleAttach(e);
          } else {
            setExpanded(!expanded);
          }
        }}
        aria-expanded={expanded}
        className="focus-ring flex w-full items-center justify-between p-3 hover:bg-surface-muted  transition-colors"
      >
        <div className="flex items-center gap-2">
          <GitBranch className="h-4 w-4 text-emerald-500" />
          <div className="flex flex-col items-start text-left">
            <span className="text-base font-semibold text-fg">{t('draft.segment.title')}</span>
            <span className="text-xs text-fg-muted">{t('draft.segment.subtitle')}</span>
          </div>
        </div>
        <div className="flex items-center gap-2">
          <span className="rounded-full bg-emerald-100 px-2 py-0.5 text-xs font-medium text-emerald-700 dark:bg-emerald-950/60 dark:text-emerald-300">
            {t('draft.segment.count', { count: segments.length })}
          </span>
          {expanded ? <ChevronUp aria-hidden="true" className="h-4 w-4 text-fg-subtle" /> : <ChevronDown aria-hidden="true" className="h-4 w-4 text-fg-subtle" />}
        </div>
      </button>

      {expanded && (
        <div className="border-t border-line p-3 space-y-3 ">
          <div className="flex justify-end mb-2">
            <div className="flex items-center gap-1 rounded-lg bg-surface-muted p-0.5  w-fit">
              <button
                onClick={() => setViewMode('timeline')}
                aria-pressed={viewMode === 'timeline'}
                className={cn(
                  "focus-ring",
                  "flex items-center gap-1.5 rounded-md px-2 py-1 text-sm font-medium transition-colors",
                  viewMode === 'timeline' ? "bg-surface-card text-fg   " : "text-fg-subtle hover:text-fg -subtle "
                )}
              >
                <List className="h-3 w-3" /> {t('draft.segment.viewTimeline')}
              </button>
              <button
                onClick={() => setViewMode('grid')}
                aria-pressed={viewMode === 'grid'}
                className={cn(
                  "focus-ring",
                  "flex items-center gap-1.5 rounded-md px-2 py-1 text-sm font-medium transition-colors",
                  viewMode === 'grid' ? "bg-surface-card text-fg   " : "text-fg-subtle hover:text-fg -subtle "
                )}
              >
                <LayoutGrid className="h-3 w-3" /> {t('draft.segment.viewGrid')}
              </button>
            </div>
          </div>

          {viewMode === 'timeline' ? (
            <div className="relative border-l border-line  ml-2 pl-4 space-y-3 pb-1">
              {displaySegments.map(seg => (
                <div key={seg.id} className="relative">
                  <div className="absolute -left-[21px] top-2 h-2.5 w-2.5 rounded-full border-2 border-line bg-emerald-500 " />
                  <div className="rounded-lg border border-line bg-surface-card p-2.5   ">
                    <div className="flex items-center gap-2 mb-1">
                      <span className="rounded-md border border-emerald-200 bg-emerald-50 px-2 py-0.5 text-xs font-bold text-emerald-700 dark:border-emerald-800 dark:bg-emerald-950/60 dark:text-emerald-300">
                        {seg.chapterRange}
                      </span>
                      <span className="text-xs font-medium text-fg-muted">{t('draft.segment.phase', { num: seg.phaseNumber })}</span>
                    </div>
                    <p className="text-base text-fg-muted ">{seg.summary}</p>
                  </div>
                </div>
              ))}
            </div>
          ) : (
            <div className="grid grid-cols-1 md:grid-cols-2 gap-2">
              {displaySegments.map(seg => (
                <div key={seg.id} className="rounded-lg border border-line bg-surface-card p-2.5   ">
                  <div className="flex items-center gap-2 mb-1">
                    <span className="rounded-md border border-emerald-200 bg-emerald-50 px-2 py-0.5 text-xs font-bold text-emerald-700 dark:border-emerald-800 dark:bg-emerald-950/60 dark:text-emerald-300">
                      {seg.chapterRange}
                    </span>
                    <span className="text-xs font-medium text-fg-muted">{t('draft.segment.phase', { num: seg.phaseNumber })}</span>
                  </div>
                  <p className="text-base text-fg-muted ">{seg.summary}</p>
                </div>
              ))}
            </div>
          )}

          {hasMore && (
            <div className="mt-3 flex justify-center">
              <button
                onClick={() => setShowAll(!showAll)}
                aria-expanded={showAll}
                className="focus-ring flex items-center gap-1 text-sm font-medium text-fg-subtle hover:text-fg -subtle "
              >
                {showAll ? <><ChevronUp className="h-3 w-3" /> {t('draft.segment.collapse')}</> : <><ChevronDown className="h-3 w-3" /> {t('draft.segment.showMore', { count: segments.length - 3 })}</>}
              </button>
            </div>
          )}
        </div>
      )}
    </div>
  );
}
