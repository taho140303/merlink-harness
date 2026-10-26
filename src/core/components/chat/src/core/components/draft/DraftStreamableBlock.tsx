import { Loader2 } from 'lucide-react';
import type { ReactNode } from 'react';

interface DraftStreamableBlockProps {
  isLoaded: boolean;
  loadingTitle: string;
  tag: string;
  children: ReactNode;
}

export function DraftStreamableBlock({ isLoaded, loadingTitle, tag, children }: DraftStreamableBlockProps) {
  if (!isLoaded) {
    return (
      <div className="rounded-2xl border border-slate-200 bg-white p-4 shadow-xs dark:border-slate-800 dark:bg-slate-900/60 animate-pulse">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-2">
            <Loader2 className="h-3.5 w-3.5 animate-spin text-blue-500" />
            <span className="text-xs font-medium text-slate-500 dark:text-slate-400">
              {loadingTitle}
            </span>
          </div>
          <span className="rounded-md bg-slate-100 px-2 py-0.5 text-[10px] text-slate-400 dark:bg-slate-800">
            {tag}
          </span>
        </div>
        <div className="mt-3.5 space-y-2">
          <div className="h-3 w-5/6 rounded-md bg-slate-100 dark:bg-slate-800" />
          <div className="h-3 w-2/3 rounded-md bg-slate-100 dark:bg-slate-800" />
        </div>
      </div>
    );
  }

  return (
    <div className="animate-in fade-in-50 zoom-in-98">
      {children}
    </div>
  );
}
