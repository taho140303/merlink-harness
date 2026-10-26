import { useState } from 'react';
import { Target, User, ChevronRight } from 'lucide-react';
import { cn } from '../../../../shared/utils/cn';
import type { DraftTheme } from '../types';

export function ThemeDisplay({ themes, defaultOpen = true }: { themes: DraftTheme[]; defaultOpen?: boolean }) {
  const [isOpen, setIsOpen] = useState(defaultOpen);

  if (!themes || themes.length === 0) return null;

  return (
    <div className="mt-3 overflow-hidden rounded-xl border border-slate-200 bg-slate-50/50 dark:border-slate-800 dark:bg-slate-950/30">
      {/* Toggle Header */}
      <button
        type="button"
        onClick={() => setIsOpen(!isOpen)}
        className="flex w-full items-center justify-between p-3 text-left transition-colors hover:bg-slate-100/50 dark:hover:bg-slate-800/30"
      >
        <div className="flex items-center gap-2 text-xs font-semibold text-slate-700 dark:text-slate-300">
          <Target className="h-4 w-4 text-rose-500" />
          Themes ({themes.length})
        </div>
        <ChevronRight
          className={cn(
            'h-4 w-4 text-slate-400 transition-transform duration-200',
            isOpen && 'rotate-90',
          )}
        />
      </button>

      {/* Toggle Content */}
      {isOpen && (
        <div className="animate-in fade-in-0 duration-200 border-t border-slate-200 p-3 dark:border-slate-800 space-y-3">
          {themes.map(theme => (
            <div key={theme.id} className="rounded-lg border border-slate-200 bg-white p-3 shadow-2xs dark:border-slate-700 dark:bg-slate-900">
              <div className="flex items-start justify-between gap-2">
                <div className="flex items-center gap-1.5 text-xs font-semibold text-slate-900 dark:text-slate-100">
                  {theme.question}
                </div>
                <div className="flex flex-wrap gap-1">
                  <span className="rounded bg-slate-100 px-1.5 py-0.5 text-[9px] font-bold text-slate-600 uppercase dark:bg-slate-800 dark:text-slate-300">
                    {theme.scope}
                  </span>
                  <span className="rounded bg-blue-100 px-1.5 py-0.5 text-[9px] font-bold text-blue-700 uppercase dark:bg-blue-950/60 dark:text-blue-300">
                    {theme.resolutionType}
                  </span>
                </div>
              </div>

              <div className="mt-2 text-[11px] text-slate-700 dark:text-slate-300">
                <p><span className="font-medium text-slate-500 dark:text-slate-400">Resolution:</span> {theme.resolution}</p>
                {theme.message && (
                  <p className="mt-1 italic text-slate-500 dark:text-slate-400">Message: "{theme.message}"</p>
                )}
              </div>

              <div className="mt-3 border-t border-slate-100 pt-2 dark:border-slate-800">
                <p className="text-[10px] font-semibold uppercase tracking-wide text-slate-500 dark:text-slate-400 mb-1.5 flex items-center gap-1">
                  <User className="h-3 w-3" />
                  Character Arguments
                </p>
                {theme.arguments.length === 0 ? (
                  <p className="text-[10px] italic text-slate-400">Chưa có nhân vật nào trả lời theme này (Có thể bổ sung sau).</p>
                ) : (
                  <div className="space-y-1.5">
                    {theme.arguments.map(arg => (
                      <div key={arg.id} className="flex flex-col rounded bg-slate-50 p-2 text-[10px] border border-slate-100 dark:border-slate-800 dark:bg-slate-950/50">
                        <div className="flex items-center justify-between mb-0.5">
                          <span className="font-semibold text-slate-800 dark:text-slate-200">{arg.characterName}</span>
                          <div className="flex gap-1">
                            <span className="text-emerald-600 dark:text-emerald-400 font-medium">{arg.role}</span>
                            <span className="text-slate-400">•</span>
                            <span className="text-slate-500">{arg.answerMode}</span>
                          </div>
                        </div>
                        <span className="text-slate-600 dark:text-slate-400">{arg.note}</span>
                      </div>
                    ))}
                  </div>
                )}
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
