import { User, Eye } from 'lucide-react';
import { cn } from '@/shared/utils/cn';
import { useT } from '@/shared/i18n/useT';
import type { DraftCharacter } from '../types';

interface CharacterCardProps {
  character: DraftCharacter;
  onView: () => void;
  canViewDetail?: boolean;
  isFeedbackMode?: boolean;
  onAttachFeedback?: (id: string, tag: string, title: string) => void;
}

export function CharacterCard({ character, onView, canViewDetail = true, isFeedbackMode, onAttachFeedback }: CharacterCardProps) {
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
        "flex flex-col tier-card rounded-card p-4    transition-colors duration-300",
        isFeedbackMode && "hover:!border-2 hover:!border-dashed hover:!border-purple-500 hover:bg-purple-500/5 cursor-none"
      )}
    >
      <div className="flex items-start justify-between gap-2">
        <div className="flex items-center gap-2.5">
          <div className="flex h-8 w-8 items-center justify-center rounded-full bg-surface-muted ">
            <User className="h-4 w-4 text-fg-muted" />
          </div>
          <div>
            <h4 className="text-sm font-bold text-fg">{character.name}</h4>
            <p className="text-xs text-fg-muted">
              {t('character.nickname')}: {character.nickname?.trim() || t('common.unknown')}
            </p>
          </div>
        </div>
        <div className="flex items-center gap-1.5">
          <span className={cn(
            "rounded-full px-2 py-0.5 text-xs font-medium",
            character.genderBadge === 'female' 
              ? "bg-pink-100 text-pink-700 dark:bg-pink-950/60 dark:text-pink-300"
              : "bg-blue-100 text-blue-700 dark:bg-blue-950/60 dark:text-blue-300"
          )}>
            {character.gender}
          </span>
        </div>
      </div>
      <div className="mt-3 rounded-lg bg-surface-muted p-2.5  flex-1 flex flex-col justify-between">
        <div>
          <span className="text-xs font-semibold uppercase tracking-wider text-fg-subtle">{t('draft.character.motif')}</span>
          <p className="mt-1 text-sm leading-relaxed text-fg-muted">{character.motif.description}</p>
        </div>
        
        {canViewDetail && character.appearance && (
          <button
            onClick={(e) => { e.stopPropagation(); onView(); }}
            className="mt-3 flex w-full items-center justify-center gap-1.5 rounded-lg bg-surface-card py-1.5 text-xs font-medium text-indigo-600  hover:bg-surface-muted  dark:text-indigo-400  transition-colors"
          >
            <Eye className="h-3.5 w-3.5" />
            {t('draft.char.viewDetails')}
          </button>
        )}
      </div>
    </div>
  );
}
