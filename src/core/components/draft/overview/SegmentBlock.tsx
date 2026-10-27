import { useState } from 'react';
import { List, LayoutGrid, ChevronDown, ChevronUp } from 'lucide-react';
import { cn } from '../../../../shared/utils/cn';
import { useT } from '../../../../shared/i18n/useT';
import type { DraftSegment } from '../types';

interface SegmentBlockProps {
  segments: DraftSegment[];
}

export function SegmentBlock({ segments }: SegmentBlockProps) {
  const t = useT();
  const [viewMode, setViewMode] = useState<'timeline' | 'grid'>('timeline');
  const [showAll, setShowAll] = useState(false);

  const displaySegments = showAll ? segments : segments.slice(0, 3);
  const hasMore = segments.length > 3;

  return (
    <div className="mt-4">
      <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-2 mb-3">
        <div className="flex items-center gap-2">
          <span className="text-[10px] font-semibold uppercase tracking-wider text-slate-500">{t('draft.segment.title')}</span>
          <span className="rounded-full bg-slate-100 px-2 py-0.5 text-[10px] font-medium text-slate-700 dark:bg-slate-800 dark:text-slate-300">
            {t('draft.segment.count', { count: segments.length })}
          </span>
        </div>
        <div className="flex items-center gap-1 rounded-lg bg-slate-100 p-0.5 dark:bg-slate-800 w-fit">
          <button
            onClick={() => setViewMode('timeline')}
            className={cn(
              "flex items-center gap-1.5 rounded-md px-2 py-1 text-[11px] font-medium transition-colors",
              viewMode === 'timeline' ? "bg-white text-slate-900 shadow-xs dark:bg-slate-700 dark:text-white" : "text-slate-500 hover:text-slate-900 dark:text-slate-400 dark:hover:text-slate-200"
            )}
          >
            <List className="h-3 w-3" /> {t('draft.segment.viewTimeline')}
          </button>
          <button
            onClick={() => setViewMode('grid')}
            className={cn(
              "flex items-center gap-1.5 rounded-md px-2 py-1 text-[11px] font-medium transition-colors",
              viewMode === 'grid' ? "bg-white text-slate-900 shadow-xs dark:bg-slate-700 dark:text-white" : "text-slate-500 hover:text-slate-900 dark:text-slate-400 dark:hover:text-slate-200"
            )}
          >
            <LayoutGrid className="h-3 w-3" /> {t('draft.segment.viewGrid')}
          </button>
        </div>
      </div>

      {viewMode === 'timeline' ? (
        <div className="relative border-l border-slate-200 dark:border-slate-700 ml-2 pl-4 space-y-3 pb-1">
          {displaySegments.map(seg => (
            <div key={seg.id} className="relative">
              <div className="absolute -left-[21px] top-2 h-2.5 w-2.5 rounded-full border-2 border-white bg-indigo-500 dark:border-slate-900" />
              <div className="rounded-lg border border-slate-200 bg-white p-2.5 shadow-xs dark:border-slate-800 dark:bg-slate-900">
                <div className="flex items-center gap-2 mb-1">
                  <span className="rounded-md border border-indigo-200 bg-indigo-50 px-2 py-0.5 text-[10px] font-bold text-indigo-700 dark:border-indigo-800 dark:bg-indigo-950/60 dark:text-indigo-300">
                    {seg.chapterRange}
                  </span>
                  <span className="text-[10px] font-medium text-slate-500 dark:text-slate-400">{t('draft.segment.phase', { num: seg.phaseNumber })}</span>
                </div>
                <p className="text-[11px] text-slate-700 dark:text-slate-300">{seg.summary}</p>
              </div>
            </div>
          ))}
        </div>
      ) : (
        <div className="grid grid-cols-1 md:grid-cols-2 gap-2">
          {displaySegments.map(seg => (
            <div key={seg.id} className="rounded-lg border border-slate-200 bg-white p-2.5 shadow-xs dark:border-slate-800 dark:bg-slate-900">
              <div className="flex items-center gap-2 mb-1">
                <span className="rounded-md border border-indigo-200 bg-indigo-50 px-2 py-0.5 text-[10px] font-bold text-indigo-700 dark:border-indigo-800 dark:bg-indigo-950/60 dark:text-indigo-300">
                  {seg.chapterRange}
                </span>
                <span className="text-[10px] font-medium text-slate-500 dark:text-slate-400">{t('draft.segment.phase', { num: seg.phaseNumber })}</span>
              </div>
              <p className="text-[11px] text-slate-700 dark:text-slate-300">{seg.summary}</p>
            </div>
          ))}
        </div>
      )}

      {hasMore && (
        <div className="mt-3 flex justify-center">
          <button
            onClick={() => setShowAll(!showAll)}
            className="flex items-center gap-1 text-[11px] font-medium text-slate-500 hover:text-slate-900 dark:text-slate-400 dark:hover:text-slate-200"
          >
            {showAll ? <><ChevronUp className="h-3 w-3" /> {t('draft.segment.collapse')}</> : <><ChevronDown className="h-3 w-3" /> {t('draft.segment.showMore', { count: segments.length - 3 })}</>}
          </button>
        </div>
      )}
    </div>
  );
}
