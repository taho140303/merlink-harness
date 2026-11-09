import { Globe2, Shield, BookOpen } from 'lucide-react';
import { useT } from '@/shared/i18n/useT';
import { useDbEnumLabel } from '@/shared/i18n/dbEnums';
import { cn } from '@/shared/utils/cn';
import type { DraftWorldSettings } from '../types';

interface WorldSettingsBlockProps {
  settings: DraftWorldSettings;
  isFeedbackMode?: boolean;
  onAttachFeedback?: (id: string, tag: string, title: string) => void;
}

export function WorldSettingsBlock({ settings, isFeedbackMode, onAttachFeedback }: WorldSettingsBlockProps) {
  const t = useT();
  const enumLabel = useDbEnumLabel();

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
          "w-full tier-card rounded-card p-5    transition-colors duration-300",
          isFeedbackMode && "hover:!border-2 hover:!border-dashed hover:!border-purple-500 hover:bg-purple-500/5 cursor-none"
        )}
      >
        <div className="flex items-center justify-between border-b border-line pb-4 ">
          <div className="flex items-center gap-3">
            <div className="flex h-10 w-10 shrink-0 items-center justify-center rounded-xl bg-indigo-100 text-indigo-600 dark:bg-indigo-950/60 dark:text-indigo-400">
              <Globe2 className="h-5 w-5" />
            </div>
            <div>
              <h3 className="text-lg font-bold text-fg">
                {settings.displayName}
              </h3>
              <div className="flex items-center gap-2 text-sm font-medium text-fg-muted mt-1">
                <span className="rounded bg-surface-muted px-2 py-0.5 ">{settings.type}</span>
                {settings.parentCode && <span>Parent: {settings.parentCode}</span>}
              </div>
            </div>
          </div>
        </div>

        <div className="mt-4 flex flex-col gap-5 w-full">
          <div className="w-full">
            <span className="text-sm font-semibold uppercase tracking-wider text-fg-subtle">{t('draft.world.desc')}</span>
            <p className="mt-1 text-lg leading-relaxed text-fg-muted  w-full">
              {settings.description}
            </p>
          </div>
          <div className="w-full">
            <span className="text-sm font-semibold uppercase tracking-wider text-fg-subtle">{t('draft.world.access')}</span>
            <div className="mt-1 flex items-start gap-2 text-lg leading-relaxed text-fg-muted  w-full">
              <Shield className="h-5 w-5 shrink-0 text-emerald-500 mt-1" />
              <p className="flex-1">{enumLabel('LocationAccess', settings.access)}</p>
            </div>
          </div>
        </div>
      </div>

      <div className="w-full tier-card rounded-card p-5   ">
        <div className="flex items-center gap-2 mb-4">
          <BookOpen className="h-5 w-5 text-amber-500" />
          <h4 className="text-lg font-bold text-fg">{t('draft.world.lore')}</h4>
          <span className="rounded-full bg-amber-100 px-2.5 py-0.5 text-sm font-medium text-amber-700 dark:bg-amber-950/60 dark:text-amber-300">
            {settings.loreEntries.length} entries
          </span>
        </div>

        <div className="space-y-3 w-full">
          {settings.loreEntries.map((lore) => (
            <div 
              key={lore.code} 
              onClick={(e) => handleAttach(e, 'lore', lore.displayName, lore.code)}
              className={cn(
                "w-full rounded-lg border border-line bg-surface-muted p-4   transition-colors duration-300",
                isFeedbackMode && "hover:!border-2 hover:!border-dashed hover:!border-purple-500 hover:bg-purple-500/5 cursor-none"
              )}
            >
              <div className="flex flex-wrap items-center gap-2 mb-2">
                <span className="rounded bg-surface-card px-2 py-0.5 text-xs font-bold text-fg-muted   ">
                  {lore.type}
                </span>
                <span className="text-base font-bold text-fg">{lore.displayName}</span>
                <span className="text-sm font-mono text-fg-subtle">{lore.code}</span>
              </div>
              <p className="text-base text-fg-muted -subtle leading-relaxed">{lore.description}</p>
              {lore.data && (
                <p className="mt-2 text-base italic text-fg-subtle -subtle border-l-2 border-line-strong pl-3  leading-relaxed">
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
