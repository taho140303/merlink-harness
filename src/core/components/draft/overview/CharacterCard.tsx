import { User } from 'lucide-react';
import { cn } from '../../../../shared/utils/cn';
import { useT } from '../../../../shared/i18n/useT';
import type { DraftCharacter } from '../types';

interface CharacterCardProps {
  character: DraftCharacter;
}

export function CharacterCard({ character }: CharacterCardProps) {
  const t = useT();
  return (
    <div className="flex flex-col rounded-xl border border-slate-200 bg-white p-4 shadow-sm dark:border-slate-800 dark:bg-slate-900">
      <div className="flex items-start justify-between gap-2">
        <div className="flex items-center gap-2.5">
          <div className="flex h-8 w-8 items-center justify-center rounded-full bg-slate-100 dark:bg-slate-800">
            <User className="h-4 w-4 text-slate-500 dark:text-slate-400" />
          </div>
          <div>
            <h4 className="text-sm font-bold text-slate-900 dark:text-slate-100">{character.name}</h4>
            <p className="text-[11px] text-slate-500 dark:text-slate-400">{character.gender}</p>
          </div>
        </div>
        <span className={cn(
          "rounded-full px-2 py-0.5 text-[10px] font-medium",
          character.genderBadge === 'female' 
            ? "bg-pink-100 text-pink-700 dark:bg-pink-950/60 dark:text-pink-300"
            : "bg-blue-100 text-blue-700 dark:bg-blue-950/60 dark:text-blue-300"
        )}>
          {character.gender}
        </span>
      </div>
      <div className="mt-3 rounded-lg bg-slate-50 p-2.5 dark:bg-slate-800/50">
        <span className="text-[10px] font-semibold uppercase tracking-wider text-slate-400">{t('draft.character.archetype')}</span>
        <p className="mt-1 text-[11px] leading-relaxed text-slate-600 dark:text-slate-300">{character.archetype}</p>
      </div>
    </div>
  );
}
