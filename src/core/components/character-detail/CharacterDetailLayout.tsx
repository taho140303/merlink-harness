import { useState } from 'react';
import { Link } from 'react-router';
import { Pencil, MessageSquare, GripVertical, X, Check } from 'lucide-react';
import { cn } from '../../../shared/utils/cn';
import { truncate } from '../../../shared/utils/string';
import { useResizablePanel } from '../../../shared/hooks/useResizablePanel';
import { CharacterChatPane } from '../character/CharacterChatPane';
import { ConfirmDialog } from '../../../shared/components/ui/confirm-dialog';
import { toast } from '../../../shared/components/ui/toast';
import { useT } from '../../../shared/i18n/useT';
import { useCharacterDetailStore } from './store';
import { CoreIdentityTab } from './tabs/CoreIdentityTab';
import { LifeExperienceTab } from './tabs/LifeExperienceTab';
import { PsychologicalTab } from './tabs/PsychologicalTab';
import type { DetailTab } from './types';

const tabLabels: Record<DetailTab, string> = {
  core: 'Core Identity',
  life: 'Life Experience',
  psych: 'Psychological',
};

const lifeSubLabels = {
  knowledge: 'Knowledge',
  competence: 'Competence',
  memory: 'Memory',
} as const;

export function CharacterDetailLayout() {
  const t = useT();
  const character = useCharacterDetailStore((s) => s.character);
  const isEditing = useCharacterDetailStore((s) => s.isEditing);
  const startEditing = useCharacterDetailStore((s) => s.startEditing);
  const saveEditing = useCharacterDetailStore((s) => s.saveEditing);
  const cancelEditing = useCharacterDetailStore((s) => s.cancelEditing);
  const isChatOpen = useCharacterDetailStore((s) => s.isChatOpen);
  const toggleChat = useCharacterDetailStore((s) => s.toggleChat);
  const activeTab = useCharacterDetailStore((s) => s.activeTab);
  const setActiveTab = useCharacterDetailStore((s) => s.setActiveTab);
  const activeLifeTab = useCharacterDetailStore((s) => s.activeLifeTab);

  const [discardOpen, setDiscardOpen] = useState(false);

  const { leftRatio, isDragging, containerRef, startDragging } = useResizablePanel({
    initialRatio: 45,
    minPx: 340,
  });

  const subSectionName =
    activeTab === 'life'
      ? `${tabLabels.life} › ${lifeSubLabels[activeLifeTab]}`
      : tabLabels[activeTab];

  const handleSave = () => {
    saveEditing();
    toast({ title: t('common.saved'), variant: 'success' });
  };
  const handleConfirmDiscard = () => {
    cancelEditing();
    setDiscardOpen(false);
  };

  return (
    <div
      ref={containerRef}
      className={cn(
        'flex h-full w-full overflow-hidden',
        isDragging && 'cursor-col-resize select-none',
      )}
    >
      {isChatOpen && (
        <div style={{ width: `${leftRatio}%` }} className="h-full overflow-hidden">
          <CharacterChatPane
            characterName={character.basicInfo.name}
            activeSection={tabLabels[activeTab]}
            onClose={() => toggleChat()}
          />
        </div>
      )}

      {isChatOpen && (
        <div
          onMouseDown={startDragging}
          className="group relative flex w-2 shrink-0 cursor-col-resize items-center justify-center bg-transparent transition-colors hover:bg-blue-500/10 dark:hover:bg-blue-500/20"
        >
          <div className="h-full w-px bg-slate-200 transition-colors group-hover:bg-blue-500 dark:bg-slate-800" />
          <div
            className={cn(
              'absolute z-30 flex h-8 w-4.5 items-center justify-center rounded-md border border-slate-300 bg-white shadow-xs transition-opacity dark:border-slate-700 dark:bg-slate-800',
              isDragging
                ? 'opacity-100 ring-2 ring-blue-500'
                : 'opacity-0 group-hover:opacity-100',
            )}
          >
            <GripVertical className="h-3.5 w-3.5 text-slate-500 dark:text-slate-400" />
          </div>
        </div>
      )}

      <div
        style={{ width: isChatOpen ? `${100 - leftRatio}%` : '100%' }}
        className="flex h-full flex-col overflow-y-auto bg-slate-50/50 p-6 dark:bg-slate-950/20 sm:p-8"
      >
        <div className="flex h-12 shrink-0 items-center justify-between border-b border-slate-200 pb-4 dark:border-slate-800">
          <div className="flex items-center gap-2 truncate text-xs font-semibold text-slate-500 dark:text-slate-400">
            <Link to="/character" className="shrink-0 hover:text-slate-900 dark:hover:text-slate-100">
              Character
            </Link>
            <span>›</span>
            <span
              className="truncate text-slate-500 dark:text-slate-400"
              title={character.basicInfo.name}
            >
              {truncate(character.basicInfo.name, 15)}
            </span>
            <span>›</span>
            <span className="truncate font-bold text-slate-900 dark:text-slate-100">
              {truncate(subSectionName, 20)}
            </span>
          </div>

          <div className="flex shrink-0 items-center gap-2">
            {!isEditing ? (
              <button
                type="button"
                onClick={startEditing}
                className="flex items-center gap-1.5 rounded-xl border border-slate-200 bg-white px-3 py-1.5 text-xs font-medium text-slate-700 shadow-xs transition-colors hover:bg-slate-50 dark:border-slate-800 dark:bg-slate-900 dark:text-slate-300 dark:hover:bg-slate-800"
              >
                <Pencil className="h-3.5 w-3.5" />
                <span>{t('common.edit')}</span>
              </button>
            ) : (
              <>
                <button
                  type="button"
                  onClick={() => setDiscardOpen(true)}
                  className="flex items-center gap-1.5 rounded-xl border border-slate-200 bg-white px-3 py-1.5 text-xs font-medium text-slate-700 shadow-xs transition-colors hover:bg-slate-50 dark:border-slate-800 dark:bg-slate-900 dark:text-slate-300 dark:hover:bg-slate-800"
                >
                  <X className="h-3.5 w-3.5" />
                  <span>{t('common.cancel')}</span>
                </button>
                <button
                  type="button"
                  onClick={handleSave}
                  className="flex items-center gap-1.5 rounded-xl bg-slate-900 px-3 py-1.5 text-xs font-semibold text-white shadow-xs transition-colors hover:bg-slate-800 dark:bg-slate-100 dark:text-slate-900 dark:hover:bg-slate-200"
                >
                  <Check className="h-3.5 w-3.5" />
                  <span>{t('common.save')}</span>
                </button>
              </>
            )}

            <button
              type="button"
              onClick={toggleChat}
              className={cn(
                'flex items-center gap-1.5 rounded-xl border px-3 py-1.5 text-xs font-medium shadow-xs transition-colors',
                isChatOpen
                  ? 'border-blue-500 bg-blue-50 text-blue-700 dark:border-blue-600 dark:bg-blue-950/60 dark:text-blue-300'
                  : 'border-slate-200 bg-white text-slate-700 hover:bg-slate-50 dark:border-slate-800 dark:bg-slate-900 dark:text-slate-300 dark:hover:bg-slate-800',
              )}
            >
              <MessageSquare className="h-3.5 w-3.5" />
              <span>Chat</span>
            </button>
          </div>
        </div>

        <div className="mt-6 flex flex-wrap gap-2 border-b border-slate-200 pb-3 dark:border-slate-800">
          {(Object.keys(tabLabels) as DetailTab[]).map((tab) => (
            <button
              key={tab}
              type="button"
              onClick={() => setActiveTab(tab)}
              className={cn(
                'rounded-xl px-3.5 py-1.5 text-xs font-semibold transition-all',
                activeTab === tab
                  ? 'bg-slate-900 text-white shadow-xs dark:bg-slate-100 dark:text-slate-900'
                  : 'bg-white text-slate-600 hover:bg-slate-100 dark:bg-slate-900 dark:text-slate-400 dark:hover:bg-slate-800',
              )}
            >
              {tabLabels[tab]}
            </button>
          ))}
        </div>

        <div className="mt-6">
          {activeTab === 'core' && <CoreIdentityTab />}
          {activeTab === 'life' && <LifeExperienceTab />}
          {activeTab === 'psych' && <PsychologicalTab />}
        </div>
      </div>

      <ConfirmDialog
        open={discardOpen}
        onOpenChange={setDiscardOpen}
        title={t('discard.title')}
        description={t('discard.description')}
        confirmLabel={t('discard.confirm')}
        cancelLabel={t('discard.cancel')}
        variant="danger"
        onConfirm={handleConfirmDiscard}
      />
    </div>
  );
}
