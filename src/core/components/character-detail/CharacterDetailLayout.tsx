import { useState } from 'react';
import { Link } from 'react-router';
import { Pencil, MessageSquare, GripVertical, X, Check } from 'lucide-react';
import { cn } from '../../../shared/utils/cn';
import { truncate } from '../../../shared/utils/string';
import { useResizablePanel } from '../../../shared/hooks/useResizablePanel';
import { useIsCompact } from '../../../shared/hooks/useMediaQuery';
import { CharacterChatPane } from '../character/CharacterChatPane';
import { ConfirmDialog } from '../../../shared/components/ui/confirm-dialog';
import { toast } from '../../../shared/components/ui/toast';
import { useT } from '../../../shared/i18n/useT';
import { Tabs } from '../../../shared/components/ui/tabs';
import type { TKey } from '../../../shared/i18n/dict';
import { useCharacterDetailStore } from './store';
import { CoreIdentityTab } from './tabs/CoreIdentityTab';
import { LifeExperienceTab } from './tabs/LifeExperienceTab';
import { PsychologicalTab } from './tabs/PsychologicalTab';
import type { DetailTab } from './types';

const tabKeys: Record<DetailTab, TKey> = {
  core: 'character.detail.core',
  life: 'character.detail.life',
  psych: 'character.detail.psych',
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

  const isCompact = useIsCompact();
  const { leftRatio, setLeftRatio, isDragging, containerRef, startDragging } = useResizablePanel({
    initialRatio: 45,
    minPx: 340,
  });

  const tabLabel = (tab: DetailTab) => t(tabKeys[tab]);
  const subSectionName =
    activeTab === 'life'
      ? `${tabLabel('life')} › ${lifeSubLabels[activeLifeTab]}`
      : tabLabel(activeTab);

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
        'relative flex h-full w-full overflow-hidden',
        isDragging && 'cursor-col-resize select-none',
      )}
    >
      {isChatOpen && (
        <div
          style={isCompact ? undefined : { width: `${leftRatio}%` }}
          className={cn('h-full overflow-hidden', isCompact && 'absolute inset-0 z-40 w-full bg-surface-panel')}
        >
          <CharacterChatPane
            characterName={character.basicInfo.name}
            activeSection={tabLabel(activeTab)}
            onClose={() => toggleChat()}
          />
        </div>
      )}

      {isChatOpen && !isCompact && (
        <div
          role="separator"
          tabIndex={0}
          aria-label={t('character.detail.resizePanels')}
          aria-orientation="vertical"
          aria-valuenow={Math.round(leftRatio)}
          aria-valuemin={20}
          aria-valuemax={80}
          onMouseDown={startDragging}
          onKeyDown={(event) => {
            if (event.key !== 'ArrowLeft' && event.key !== 'ArrowRight') return;
            event.preventDefault();
            const delta = event.key === 'ArrowLeft' ? -5 : 5;
            setLeftRatio(Math.min(80, Math.max(20, leftRatio + delta)));
          }}
          className="focus-ring group relative flex w-2 shrink-0 cursor-col-resize items-center justify-center bg-transparent transition-colors hover:bg-accent-soft"
        >
          <div className="h-full w-px bg-line transition-colors group-hover:bg-accent" />
          <div
            className={cn(
              'absolute z-30 flex h-8 w-4.5 items-center justify-center rounded-md tier-card transition-opacity',
              isDragging
                ? 'opacity-100 ring-2 ring-accent'
                : 'opacity-0 group-hover:opacity-100',
            )}
          >
            <GripVertical aria-hidden="true" className="h-3.5 w-3.5 text-fg-muted" />
          </div>
        </div>
      )}

      <div
        style={{ width: isChatOpen && !isCompact ? `${100 - leftRatio}%` : '100%' }}
        className="flex h-full flex-col overflow-y-auto bg-surface-muted/50 p-6  sm:p-8"
      >
        <div className="flex h-12 shrink-0 items-center justify-between border-b border-line pb-4 ">
          <div className="flex items-center gap-2 truncate text-xs font-semibold text-fg-muted">
            <Link to="/character" className="shrink-0 hover:text-fg ">
              Character
            </Link>
            <span>›</span>
            <span
              className="truncate text-fg-muted"
              title={character.basicInfo.name}
            >
              {truncate(character.basicInfo.name, 15)}
            </span>
            <span>›</span>
            <span className="truncate font-bold text-fg">
              {truncate(subSectionName, 20)}
            </span>
          </div>

          <div className="flex shrink-0 items-center gap-2">
            {!isEditing ? (
              <button
                type="button"
                onClick={startEditing}
                className="flex items-center gap-1.5 rounded-xl border border-line bg-surface-card px-3 py-1.5 text-xs font-medium text-fg  transition-colors hover:bg-surface-muted    "
              >
                <Pencil className="h-3.5 w-3.5" />
                <span>{t('common.edit')}</span>
              </button>
            ) : (
              <>
                <button
                  type="button"
                  onClick={() => setDiscardOpen(true)}
                  className="flex items-center gap-1.5 rounded-xl border border-line bg-surface-card px-3 py-1.5 text-xs font-medium text-fg  transition-colors hover:bg-surface-muted    "
                >
                  <X className="h-3.5 w-3.5" />
                  <span>{t('common.cancel')}</span>
                </button>
                <button
                  type="button"
                  onClick={handleSave}
                  className="flex items-center gap-1.5 rounded-xl bg-accent px-3 py-1.5 text-xs font-semibold text-white  transition-colors hover:opacity-90   "
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
                'flex items-center gap-1.5 rounded-xl border px-3 py-1.5 text-xs font-medium  transition-colors',
                isChatOpen
                  ? 'border-blue-500 bg-blue-50 text-blue-700 dark:border-blue-600 dark:bg-blue-950/60 dark:text-blue-300'
                  : 'border-line bg-surface-card text-fg hover:bg-surface-muted    ',
              )}
            >
              <MessageSquare className="h-3.5 w-3.5" />
              <span>Chat</span>
            </button>
          </div>
        </div>

        <Tabs
          className="mt-6"
          ariaLabel={t('character.detail.tabs')}
          value={activeTab}
          onChange={(id) => setActiveTab(id as DetailTab)}
          items={(Object.keys(tabKeys) as DetailTab[]).map((tab) => ({
            id: tab,
            label: t(tabKeys[tab]),
          }))}
        />

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
