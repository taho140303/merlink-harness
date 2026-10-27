import { ChevronDown, ChevronUp } from 'lucide-react';
import { useT } from '../../../../shared/i18n/useT';
import { ThemeBlock } from './ThemeBlock';
import { SegmentBlock } from './SegmentBlock';
import type { DraftArc } from '../types';

interface ArcCardProps {
  arc: DraftArc;
  isExpanded: boolean;
  onToggle: () => void;
}

export function ArcCard({ arc, isExpanded, onToggle }: ArcCardProps) {
  const t = useT();
  return (
    <div className="rounded-xl border border-slate-200 bg-white shadow-sm dark:border-slate-800 dark:bg-slate-900 overflow-hidden">
      <button
        type="button"
        onClick={onToggle}
        className="flex w-full items-center justify-between p-4 hover:bg-slate-50 dark:hover:bg-slate-800/50 transition-colors"
      >
        <div className="flex items-center gap-3">
          <span className="flex h-8 w-8 items-center justify-center rounded-lg bg-indigo-100 font-bold text-indigo-700 dark:bg-indigo-950/60 dark:text-indigo-300">
            {arc.number}
          </span>
          <div className="flex flex-col items-start text-left">
            <h4 className="text-sm font-bold text-slate-900 dark:text-slate-100">{arc.title}</h4>
            <span className="text-[11px] text-slate-500 dark:text-slate-400">{arc.subnote}</span>
          </div>
        </div>
        <div className="flex items-center gap-3">
          <span className="rounded-full bg-slate-100 px-2 py-0.5 text-[10px] font-medium text-slate-700 dark:bg-slate-800 dark:text-slate-300">
            {t('draft.arc.segmentsCount', { count: arc.segmentsCount })}
          </span>
          {isExpanded ? <ChevronUp className="h-4 w-4 text-slate-400" /> : <ChevronDown className="h-4 w-4 text-slate-400" />}
        </div>
      </button>

      {isExpanded && (
        <div className="border-t border-slate-100 p-4 dark:border-slate-800 space-y-4">
          <p className="text-xs leading-relaxed text-slate-600 dark:text-slate-300">{arc.summary}</p>
          
          <ThemeBlock theme={arc.theme} />
          
          <SegmentBlock segments={arc.segments} />
        </div>
      )}
    </div>
  );
}
