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
    <div className="space-y-4 w-full">
      <div 
        onClick={(e) => handleAttach(e, 'world', settings.displayName, settings.code)}
        className={cn(
          "w-full rounded-xl border border-slate-200 bg-white p-5 shadow-sm dark:border-slate-800 dark:bg-slate-900 transition-all duration-300",
          isFeedbackMode && "hover:!border-2 hover:!border-dashed hover:!border-purple-500 hover:bg-purple-500/5 cursor-none"
        )}
      >
        <div className="flex items-center justify-between border-b border-slate-100 pb-4 dark:border-slate-800">
          <div className="flex items-center gap-3">
            <div className="flex h-10 w-10 shrink-0 items-center justify-center rounded-xl bg-indigo-100 text-indigo-600 dark:bg-indigo-950/60 dark:text-indigo-400">
              <Globe2 className="h-5 w-5" />
            </div>
            <div>
              <h3 className="text-base font-bold text-slate-900 dark:text-slate-100">
                {settings.displayName}
              </h3>
              <div className="flex items-center gap-2 text-xs font-medium text-slate-500 dark:text-slate-400 mt-0.5">
                <span className="rounded bg-slate-100 px-2 py-0.5 dark:bg-slate-800">{settings.type}</span>
                {settings.parentCode && <span>Parent: {settings.parentCode}</span>}
              </div>
            </div>
          </div>
        </div>

        <div className="mt-4 flex flex-col gap-4 w-full">
          <div className="w-full">
            <span className="text-[11px] font-semibold uppercase tracking-wider text-slate-400">{t('draft.world.desc')}</span>
            <p className="mt-1 text-sm leading-relaxed text-slate-700 dark:text-slate-300 w-full">
              {settings.description}
            </p>
          </div>
          <div className="w-full">
            <span className="text-[11px] font-semibold uppercase tracking-wider text-slate-400">{t('draft.world.access')}</span>
            <div className="mt-1 flex items-start gap-2 text-sm leading-relaxed text-slate-700 dark:text-slate-300 w-full">
              <Shield className="h-4 w-4 shrink-0 text-emerald-500 mt-0.5" />
              <p className="flex-1">{settings.access}</p>
            </div>
          </div>
        </div>
      </div>

      <div className="w-full rounded-xl border border-slate-200 bg-white p-5 shadow-sm dark:border-slate-800 dark:bg-slate-900">
        <div className="flex items-center gap-2 mb-4">
          <BookOpen className="h-4 w-4 text-amber-500" />
          <h4 className="text-base font-bold text-slate-900 dark:text-slate-100">{t('draft.world.lore')}</h4>
          <span className="rounded-full bg-amber-100 px-2.5 py-0.5 text-[11px] font-medium text-amber-700 dark:bg-amber-950/60 dark:text-amber-300">
            {settings.loreEntries.length} entries
          </span>
        </div>

        <div className="space-y-3 w-full">
          {settings.loreEntries.map((lore) => (
            <div 
              key={lore.code} 
              onClick={(e) => handleAttach(e, 'lore', lore.displayName, lore.code)}
              className={cn(
                "w-full rounded-lg border border-slate-200 bg-slate-50 p-4 dark:border-slate-700 dark:bg-slate-800/50 transition-all duration-300",
                isFeedbackMode && "hover:!border-2 hover:!border-dashed hover:!border-purple-500 hover:bg-purple-500/5 cursor-none"
              )}
            >
              <div className="flex flex-wrap items-center gap-2 mb-2">
                <span className="rounded bg-white px-2 py-0.5 text-[11px] font-bold text-slate-600 shadow-2xs dark:bg-slate-900 dark:text-slate-300">
                  {lore.type}
                </span>
                <span className="text-sm font-bold text-slate-900 dark:text-slate-100">{lore.displayName}</span>
                <span className="text-xs font-mono text-slate-400">{lore.code}</span>
              </div>
              <p className="text-sm text-slate-600 dark:text-slate-400 leading-relaxed">{lore.description}</p>
              {lore.data && (
                <p className="mt-2 text-sm italic text-slate-500 dark:text-slate-500 border-l-2 border-slate-300 pl-3 dark:border-slate-600 leading-relaxed">
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
