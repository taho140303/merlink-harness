import { User, Eye } from 'lucide-react';
import { cn } from '../../../../shared/utils/cn';
import { useT } from '../../../../shared/i18n/useT';
import type { DraftCharacter } from '../types';

interface CharacterCardProps {
  character: DraftCharacter;
  onView: () => void;
  isFeedbackMode?: boolean;
  onAttachFeedback?: (id: string, tag: string, title: string) => void;
}

export function CharacterCard({ character, onView, isFeedbackMode, onAttachFeedback }: CharacterCardProps) {
  const t = useT();
  
  const handleAttach = (e: React.MouseEvent) => {
    if (isFeedbackMode && onAttachFeedback) {
      e.stopPropagation();
      onAttachFeedback(character.id, 'character', character.name);
    }
  };

  return (
    <div 
      onClick={handleAttach}
      className={cn(
        "flex flex-col rounded-xl border border-slate-200 bg-white p-4 shadow-sm dark:border-slate-800 dark:bg-slate-900 transition-all duration-300",
        isFeedbackMode && "hover:!border-2 hover:!border-dashed hover:!border-purple-500 hover:bg-purple-500/5 cursor-none"
      )}
    >
      <div className="flex items-start justify-between gap-2">
        <div className="flex items-center gap-2.5">
          <div className="flex h-8 w-8 items-center justify-center rounded-full bg-slate-100 dark:bg-slate-800">
            <User className="h-4 w-4 text-slate-500 dark:text-slate-400" />
          </div>
          <div>
            <h4 className="text-sm font-bold text-slate-900 dark:text-slate-100">{character.name}</h4>
            <p className="text-xs text-slate-500 dark:text-slate-400">{character.gender}</p>
          </div>
        </div>
        <div className="flex items-center gap-1.5">
          <span className={cn(
            "rounded-full px-2 py-0.5 text-[10px] font-medium",
            character.genderBadge === 'female' 
              ? "bg-pink-100 text-pink-700 dark:bg-pink-950/60 dark:text-pink-300"
              : "bg-blue-100 text-blue-700 dark:bg-blue-950/60 dark:text-blue-300"
          )}>
            {character.gender}
          </span>
        </div>
      </div>
      <div className="mt-3 rounded-lg bg-slate-50 p-2.5 dark:bg-slate-800/50 flex-1 flex flex-col justify-between">
        <div>
          <span className="text-[10px] font-semibold uppercase tracking-wider text-slate-400">{t('draft.character.motif')}</span>
          <p className="mt-1 text-sm leading-relaxed text-slate-600 dark:text-slate-300">{character.motif.description}</p>
        </div>
        
        {character.appearance && (
          <button
            onClick={(e) => { e.stopPropagation(); onView(); }}
            className="mt-3 flex w-full items-center justify-center gap-1.5 rounded-lg bg-white py-1.5 text-xs font-medium text-indigo-600 shadow-2xs hover:bg-slate-50 dark:bg-slate-900 dark:text-indigo-400 dark:hover:bg-slate-800 transition-colors"
          >
            <Eye className="h-3.5 w-3.5" />
            {t('draft.char.viewDetails') || 'Xem chi tiết'}
          </button>
        )}
      </div>
    </div>
  );
}
