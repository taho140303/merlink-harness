import { Globe2, Shield, BookOpen } from 'lucide-react';
import { useT } from '../../../../shared/i18n/useT';
import { cn } from '../../../../shared/utils/cn';
import type { DraftWorldSettings } from '../types';

interface WorldSettingsBlockProps {
  settings: DraftWorldSettings;
  isFeedbackMode?: boolean;
  onAttachFeedback?: (id: string, tag: string, title: string) => void;
}

export function WorldSettingsBlock({ settings, isFeedbackMode, onAttachFeedback }: WorldSettingsBlockProps) {
  const t = useT();

  const handleAttach = (e: React.MouseEvent, tag: string, title: string, id: string) => {
    if (isFeedbackMode && onAttachFeedback) {
      e.stopPropagation();
      onAttachFeedback(id, tag, title);
    }
  };

  return (
    <div className="space-y-4">
      <div 
        onClick={(e) => handleAttach(e, 'world', settings.displayName, settings.code)}
        className={cn(
          "rounded-xl border border-slate-200 bg-white p-4 shadow-sm dark:border-slate-800 dark:bg-slate-900 transition-all duration-300",
          isFeedbackMode && "hover:!border-2 hover:!border-dashed hover:!border-purple-500 hover:bg-purple-500/5 cursor-none"
        )}
      >
        <div className="flex items-center justify-between border-b border-slate-100 pb-3 dark:border-slate-800">
          <div className="flex items-center gap-3">
            <div className="flex h-10 w-10 items-center justify-center rounded-xl bg-indigo-100 text-indigo-600 dark:bg-indigo-950/60 dark:text-indigo-400">
              <Globe2 className="h-5 w-5" />
            </div>
            <div>
              <h3 className="text-sm font-bold text-slate-900 dark:text-slate-100">
                {settings.displayName}
              </h3>
              <div className="flex items-center gap-2 text-[11px] font-medium text-slate-500 dark:text-slate-400">
                <span className="rounded bg-slate-100 px-1.5 py-0.5 dark:bg-slate-800">{settings.type}</span>
                {settings.parentCode && <span>Parent: {settings.parentCode}</span>}
              </div>
            </div>
          </div>
        </div>

        <div className="mt-3 grid grid-cols-1 md:grid-cols-2 gap-4">
          <div className="space-y-3">
            <div>
              <span className="text-[11px] font-semibold uppercase tracking-wider text-slate-400">{t('draft.world.desc')}</span>
              <p className="mt-1 text-sm leading-relaxed text-slate-700 dark:text-slate-300">
                {settings.description}
              </p>
            </div>
            <div>
              <span className="text-[11px] font-semibold uppercase tracking-wider text-slate-400">{t('draft.world.access')}</span>
              <p className="mt-1 text-sm leading-relaxed text-slate-700 dark:text-slate-300 flex items-center gap-1.5">
                <Shield className="h-3 w-3 text-emerald-500" />
                {settings.access}
              </p>
            </div>
          </div>
        </div>
      </div>

      <div className="rounded-xl border border-slate-200 bg-white p-4 shadow-sm dark:border-slate-800 dark:bg-slate-900">
        <div className="flex items-center gap-2 mb-3">
          <BookOpen className="h-4 w-4 text-amber-500" />
          <h4 className="text-sm font-bold text-slate-900 dark:text-slate-100">{t('draft.world.lore')}</h4>
          <span className="rounded-full bg-amber-100 px-2 py-0.5 text-[10px] font-medium text-amber-700 dark:bg-amber-950/60 dark:text-amber-300">
            {settings.loreEntries.length} entries
          </span>
        </div>

        <div className="space-y-2">
          {settings.loreEntries.map((lore) => (
            <div 
              key={lore.code} 
              onClick={(e) => handleAttach(e, 'lore', lore.displayName, lore.code)}
              className={cn(
                "rounded-lg border border-slate-200 bg-slate-50 p-2.5 dark:border-slate-700 dark:bg-slate-800/50 transition-all duration-300",
                isFeedbackMode && "hover:!border-2 hover:!border-dashed hover:!border-purple-500 hover:bg-purple-500/5 cursor-none"
              )}
            >
              <div className="flex items-center gap-2 mb-1">
                <span className="rounded bg-white px-1.5 py-0.5 text-[9px] font-bold text-slate-600 shadow-2xs dark:bg-slate-900 dark:text-slate-300">
                  {lore.type}
                </span>
                <span className="text-xs font-bold text-slate-900 dark:text-slate-100">{lore.displayName}</span>
                <span className="text-[9px] font-mono text-slate-400">{lore.code}</span>
              </div>
              <p className="text-xs text-slate-600 dark:text-slate-400">{lore.description}</p>
              {lore.data && (
                <p className="mt-1 text-xs italic text-slate-500 dark:text-slate-500 border-l-2 border-slate-300 pl-2 dark:border-slate-600">
                  {lore.data}
                </p>
              )}
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}
