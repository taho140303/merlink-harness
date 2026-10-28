import { useState } from 'react';
import { BookOpen, ChevronDown, ChevronUp } from 'lucide-react';
import { useT } from '../../../../shared/i18n/useT';
import { cn } from '../../../../shared/utils/cn';
import type { DraftTheme } from '../types';

interface ThemeBlockProps {
  theme: DraftTheme;
  isFeedbackMode?: boolean;
  onAttachFeedback?: (id: string, tag: string, title: string) => void;
}

export function ThemeBlock({ theme, isFeedbackMode, onAttachFeedback }: ThemeBlockProps) {
  const t = useT();
  const [expanded, setExpanded] = useState(false);

  const handleAttach = (e: React.MouseEvent) => {
    if (isFeedbackMode && onAttachFeedback) {
      e.stopPropagation();
      onAttachFeedback('theme', 'theme', theme.question);
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
        className="flex w-full items-center justify-between p-3 hover:bg-surface-muted  transition-colors"
      >
        <div className="flex items-center gap-2">
          <BookOpen className="h-4 w-4 text-indigo-500" />
          <div className="flex flex-col items-start text-left">
            <span className="text-base font-semibold text-fg">{t('draft.theme.title')}</span>
            <span className="text-xs text-fg-muted">{t('draft.theme.subtitle')}</span>
          </div>
        </div>
        <div className="flex items-center gap-2">
          <span className="rounded-full bg-indigo-100 px-2 py-0.5 text-xs font-medium text-indigo-700 dark:bg-indigo-950/60 dark:text-indigo-300">
            {t('draft.theme.argumentsCount', { count: theme.arguments.length })}
          </span>
          {expanded ? <ChevronUp className="h-4 w-4 text-fg-subtle" /> : <ChevronDown className="h-4 w-4 text-fg-subtle" />}
        </div>
      </button>

      {expanded && (
        <div className="border-t border-line p-3 space-y-3 ">
          <div className="grid grid-cols-1 md:grid-cols-2 gap-3">
            <div className="rounded-lg bg-surface-muted p-2.5 ">
              <span className="text-xs font-semibold uppercase tracking-wider text-fg-subtle">{t('draft.theme.question')}</span>
              <p className="mt-1 text-lg font-serif text-fg ">{theme.question}</p>
            </div>
            <div className="rounded-lg bg-surface-muted p-2.5 ">
              <span className="text-xs font-semibold uppercase tracking-wider text-fg-subtle">{t('draft.theme.message')}</span>
              <p className="mt-1 text-lg font-serif text-fg ">{theme.message}</p>
            </div>
          </div>
          
          <div className="rounded-lg bg-surface-muted p-2.5 ">
            <span className="text-xs font-semibold uppercase tracking-wider text-fg-subtle">{t('draft.theme.resolution')}</span>
            <p className="mt-1 text-lg font-serif text-fg ">{theme.resolution}</p>
          </div>

          <div className="space-y-2">
            <span className="text-xs font-semibold uppercase tracking-wider text-fg-subtle">{t('draft.theme.characterArguments')}</span>
            {theme.arguments.map(arg => (
              <div key={arg.id} className="rounded-lg border border-line bg-surface-card p-2.5   space-y-2">
                <div className="flex flex-wrap items-center justify-between gap-2">
                  <span className="text-sm font-bold text-fg">{arg.characterName}</span>
                  <div className="flex items-center gap-1.5">
                    <span className="rounded bg-indigo-50 px-1.5 py-0.5 text-xs font-medium text-indigo-700 dark:bg-indigo-950/60 dark:text-indigo-300">
                      {arg.role === 'Chính' ? t('draft.theme.role.primary') : arg.role === 'Phản đề' ? t('draft.theme.role.antithesis') : arg.role}
                    </span>
                    <span className="rounded bg-emerald-50 px-1.5 py-0.5 text-xs font-medium text-emerald-700 dark:bg-emerald-950/60 dark:text-emerald-300">
                      {arg.reactionType === 'Hành động' ? t('draft.theme.reaction.action') : arg.reactionType === 'Lời nói' ? t('draft.theme.reaction.verbal') : arg.reactionType}
                    </span>
                  </div>
                </div>
                {arg.note && <p className="text-sm text-fg-muted -subtle">{arg.note}</p>}
                {(arg.requiredKnowledge || arg.requiredCompetence) && (
                  <div className="grid grid-cols-1 sm:grid-cols-2 gap-2 pt-1 border-t border-line ">
                    {arg.requiredKnowledge && (
                      <div>
                        <span className="text-xs uppercase text-fg-subtle">{t('draft.theme.requiredKnowledge')}</span>
                        <p className="text-sm text-fg-muted ">{arg.requiredKnowledge}</p>
                      </div>
                    )}
                    {arg.requiredCompetence && (
                      <div>
                        <span className="text-xs uppercase text-fg-subtle">{t('draft.theme.requiredCompetence')}</span>
                        <p className="text-sm text-fg-muted ">{arg.requiredCompetence}</p>
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
