import { Info } from 'lucide-react';
import { cn } from '../../../../shared/utils/cn';

interface InfoTooltipProps {
  content: string;
  className?: string;
}

export function InfoTooltip({ content, className }: InfoTooltipProps) {
  return (
    <span className={cn('group/info relative inline-flex items-center', className)}>
      <Info className="h-3.5 w-3.5 cursor-help text-slate-400 hover:text-slate-600 dark:text-slate-500 dark:hover:text-slate-300" />
      <span className="pointer-events-none absolute left-1/2 top-full z-50 mt-1.5 w-64 -translate-x-1/2 rounded-lg bg-slate-900 px-2.5 py-1.5 text-[11px] font-normal leading-relaxed text-white opacity-0 shadow-lg transition-opacity duration-150 group-hover/info:opacity-100 dark:bg-slate-100 dark:text-slate-900">
        {content}
      </span>
    </span>
  );
}
