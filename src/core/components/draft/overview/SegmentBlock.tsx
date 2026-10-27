import { useState } from 'react';
import { List, LayoutGrid, ChevronDown, ChevronUp } from 'lucide-react';
import { cn } from '../../../../shared/utils/cn';
import type { DraftSegment } from '../types';

interface SegmentBlockProps {
  segments: DraftSegment[];
}

export function SegmentBlock({ segments }: SegmentBlockProps) {
  const [viewMode, setViewMode] = useState<'timeline' | 'grid'>('timeline');
  const [showAll, setShowAll] = useState(false);

  const displaySegments = showAll ? segments : segments.slice(0, 3);
  const hasMore = segments.length > 3;

  return (
    <div className="mt-4">
      <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-2 mb-3">
        <div className="flex items-center gap-2">
          <span className="text-[10px] font-semibold uppercase tracking-wider text-slate-500">Phân đoạn sự kiện</span>
          <span className="rounded-full bg-slate-100 px-2 py-0.5 text-[10px] font-medium text-slate-700 dark:bg-slate-800 dark:text-slate-300">
            {segments.length} giai đoạn
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
            <List className="h-3 w-3" /> Tiến trình
          </button>
          <button
            onClick={() => setViewMode('grid')}
            className={cn(
              "flex items-center gap-1.5 rounded-md px-2 py-1 text-[11px] font-medium transition-colors",
              viewMode === 'grid' ? "bg-white text-slate-900 shadow-xs dark:bg-slate-700 dark:text-white" : "text-slate-500 hover:text-slate-900 dark:text-slate-400 dark:hover:text-slate-200"
            )}
          >
            <LayoutGrid className="h-3 w-3" /> Dạng lưới
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
                  <span className="rounded bg-slate-100 px-1.5 py-0.5 text-[9px] font-bold uppercase text-slate-600 dark:bg-slate-800 dark:text-slate-300">{seg.chapterBadge}</span>
                  <span className="text-[10px] text-slate-400">Giai đoạn {seg.phaseNumber}</span>
                  <span className="text-[10px] text-slate-400">· Chương {seg.chapterNumber}</span>
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
                <span className="rounded bg-slate-100 px-1.5 py-0.5 text-[9px] font-bold uppercase text-slate-600 dark:bg-slate-800 dark:text-slate-300">{seg.chapterBadge}</span>
                <span className="text-[10px] text-slate-400">Giai đoạn {seg.phaseNumber}</span>
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
            {showAll ? <><ChevronUp className="h-3 w-3" /> Thu gọn phân đoạn</> : <><ChevronDown className="h-3 w-3" /> Xem thêm {segments.length - 3} phân đoạn</>}
          </button>
        </div>
      )}
    </div>
  );
}
