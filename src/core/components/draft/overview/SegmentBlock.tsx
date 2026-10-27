import { useState } from 'react';
import { List, LayoutGrid, ChevronDown, ChevronUp, GitBranch } from 'lucide-react';
import { cn } from '../../../../shared/utils/cn';
import { useT } from '../../../../shared/i18n/useT';
import type { DraftSegment } from '../types';

interface SegmentBlockProps {
  segments: DraftSegment[];
}

export function SegmentBlock({ segments }: SegmentBlockProps) {
  const t = useT();
  const [expanded, setExpanded] = useState(false);
  const [viewMode, setViewMode] = useState<'timeline' | 'grid'>('timeline');
  const [showAll, setShowAll] = useState(false);

  const displaySegments = showAll ? segments : segments.slice(0, 3);
  const hasMore = segments.length > 3;

  return (
    <div className="rounded-xl border border-slate-200 bg-white dark:border-slate-800 dark:bg-slate-900 overflow-hidden">
      {/* Toggle Header */}
      <button
        type="button"
        onClick={() => setExpanded(!expanded)}
        className="flex w-full items-center justify-between p-3 hover:bg-slate-50 dark:hover:bg-slate-800/50 transition-colors"
      >
        <div className="flex items-center gap-2">
          <GitBranch className="h-4 w-4 text-emerald-500" />
          <div className="flex flex-col items-start text-left">
            <span className="text-sm font-semibold text-slate-900 dark:text-slate-100">{t('draft.segment.title')}</span>
            <span className="text-[10px] text-slate-500 dark:text-slate-400">{t('draft.segment.subtitle')}</span>
          </div>
        </div>
        <div className="flex items-center gap-2">
          <span className="rounded-full bg-emerald-100 px-2 py-0.5 text-[10px] font-medium text-emerald-700 dark:bg-emerald-950/60 dark:text-emerald-300">
            {t('draft.segment.count', { count: segments.length })}
          </span>
          {expanded ? <ChevronUp className="h-4 w-4 text-slate-400" /> : <ChevronDown className="h-4 w-4 text-slate-400" />}
        </div>
      </button>

      {/* Toggle Body */}
      {expanded && (
        <div className="border-t border-slate-100 p-3 space-y-3 dark:border-slate-800">
          <div className="flex justify-end mb-2">
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
                  <div className="absolute -left-[21px] top-2 h-2.5 w-2.5 rounded-full border-2 border-white bg-emerald-500 dark:border-slate-900" />
                  <div className="rounded-lg border border-slate-200 bg-white p-2.5 shadow-xs dark:border-slate-800 dark:bg-slate-900">
                    <div className="flex items-center gap-2 mb-1">
                      <span className="rounded-md border border-emerald-200 bg-emerald-50 px-2 py-0.5 text-[10px] font-bold text-emerald-700 dark:border-emerald-800 dark:bg-emerald-950/60 dark:text-emerald-300">
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
                    <span className="rounded-md border border-emerald-200 bg-emerald-50 px-2 py-0.5 text-[10px] font-bold text-emerald-700 dark:border-emerald-800 dark:bg-emerald-950/60 dark:text-emerald-300">
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
      )}
    </div>
  );
}
