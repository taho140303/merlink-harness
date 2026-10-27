import { useState } from 'react';
import { BookOpen, ChevronDown, ChevronUp } from 'lucide-react';
import { useT } from '../../../../shared/i18n/useT';
import type { DraftTheme } from '../types';

interface ThemeBlockProps {
  theme: DraftTheme;
}

export function ThemeBlock({ theme }: ThemeBlockProps) {
  const t = useT();
  const [expanded, setExpanded] = useState(false);

  return (
    <div className="rounded-xl border border-slate-200 bg-white dark:border-slate-800 dark:bg-slate-900 overflow-hidden">
      <button
        type="button"
        onClick={() => setExpanded(!expanded)}
        className="flex w-full items-center justify-between p-3 hover:bg-slate-50 dark:hover:bg-slate-800/50 transition-colors"
      >
        <div className="flex items-center gap-2">
          <BookOpen className="h-4 w-4 text-indigo-500" />
          <div className="flex flex-col items-start text-left">
            <span className="text-sm font-semibold text-slate-900 dark:text-slate-100">{t('draft.theme.title')}</span>
            <span className="text-[10px] text-slate-500 dark:text-slate-400">{t('draft.theme.subtitle')}</span>
          </div>
        </div>
        <div className="flex items-center gap-2">
          <span className="rounded-full bg-indigo-100 px-2 py-0.5 text-[10px] font-medium text-indigo-700 dark:bg-indigo-950/60 dark:text-indigo-300">
            {t('draft.theme.argumentsCount', { count: theme.arguments.length })}
          </span>
          {expanded ? <ChevronUp className="h-4 w-4 text-slate-400" /> : <ChevronDown className="h-4 w-4 text-slate-400" />}
        </div>
      </button>

      {expanded && (
        <div className="border-t border-slate-100 p-3 space-y-3 dark:border-slate-800">
          <div className="grid grid-cols-1 md:grid-cols-2 gap-3">
            <div className="rounded-lg bg-slate-50 p-2.5 dark:bg-slate-800/50">
              <span className="text-[10px] font-semibold uppercase tracking-wider text-slate-500">{t('draft.theme.question')}</span>
              <p className="mt-1 text-xs font-serif text-slate-800 dark:text-slate-200">{theme.question}</p>
            </div>
            <div className="rounded-lg bg-slate-50 p-2.5 dark:bg-slate-800/50">
              <span className="text-[10px] font-semibold uppercase tracking-wider text-slate-500">{t('draft.theme.message')}</span>
              <p className="mt-1 text-xs font-serif text-slate-800 dark:text-slate-200">{theme.message}</p>
            </div>
          </div>
          
          <div className="rounded-lg bg-slate-50 p-2.5 dark:bg-slate-800/50">
            <span className="text-[10px] font-semibold uppercase tracking-wider text-slate-500">{t('draft.theme.resolution')}</span>
            <p className="mt-1 text-xs font-serif text-slate-800 dark:text-slate-200">{theme.resolution}</p>
          </div>

          <div className="space-y-2">
            <span className="text-[10px] font-semibold uppercase tracking-wider text-slate-500">{t('draft.theme.characterArguments')}</span>
            {theme.arguments.map(arg => (
              <div key={arg.id} className="rounded-lg border border-slate-200 bg-white p-2.5 dark:border-slate-700 dark:bg-slate-900 space-y-2">
                <div className="flex flex-wrap items-center justify-between gap-2">
                  <span className="text-xs font-bold text-slate-900 dark:text-slate-100">{arg.characterName}</span>
                  <div className="flex items-center gap-1.5">
                    <span className="rounded bg-indigo-50 px-1.5 py-0.5 text-[10px] font-medium text-indigo-700 dark:bg-indigo-950/60 dark:text-indigo-300">
                      {arg.role === 'Chính' ? t('draft.theme.role.primary') : arg.role === 'Phản đề' ? t('draft.theme.role.antithesis') : arg.role}
                    </span>
                    <span className="rounded bg-emerald-50 px-1.5 py-0.5 text-[10px] font-medium text-emerald-700 dark:bg-emerald-950/60 dark:text-emerald-300">
                      {arg.reactionType === 'Hành động' ? t('draft.theme.reaction.action') : arg.reactionType === 'Lời nói' ? t('draft.theme.reaction.verbal') : arg.reactionType}
                    </span>
                  </div>
                </div>
                {arg.note && <p className="text-[11px] text-slate-600 dark:text-slate-400">{arg.note}</p>}
                {(arg.requiredKnowledge || arg.requiredCompetence) && (
                  <div className="grid grid-cols-1 sm:grid-cols-2 gap-2 pt-1 border-t border-slate-100 dark:border-slate-800">
                    {arg.requiredKnowledge && (
                      <div>
                        <span className="text-[9px] uppercase text-slate-400">{t('draft.theme.requiredKnowledge')}</span>
                        <p className="text-[11px] text-slate-700 dark:text-slate-300">{arg.requiredKnowledge}</p>
                      </div>
                    )}
                    {arg.requiredCompetence && (
                      <div>
                        <span className="text-[9px] uppercase text-slate-400">{t('draft.theme.requiredCompetence')}</span>
                        <p className="text-[11px] text-slate-700 dark:text-slate-300">{arg.requiredCompetence}</p>
                      </div>
                    )}
                  </div>
                )}
              </div>
            ))}
          </div>
        </div>
      )}
    </div>
  );
}
