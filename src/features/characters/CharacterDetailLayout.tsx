import { useState, useEffect } from 'react';
import { Link, useParams } from 'react-router';
import { Pencil, MessageSquare, GripVertical, X, Check } from 'lucide-react';
import { cn } from '@/shared/utils/cn';
import { truncate } from '@/shared/utils/string';
import { useResizablePanel } from '@/shared/hooks/useResizablePanel';
import { useIsCompact } from '@/shared/hooks/useMediaQuery';
import { CharacterChatPane } from './CharacterChatPane';
import { ConfirmDialog } from '@/shared/components/ui/confirm-dialog';
import { toast } from '@/shared/components/ui/toast';
import { useT } from '@/shared/i18n/useT';
import { Tabs } from '@/shared/components/ui/tabs';
import type { TKey } from '@/shared/i18n/dict';
import { backendClient, type BackendClient } from '@/shared/backend/client';
import type { CharacterDetailDto } from '@/shared/backend/generated';
import { sanitizeDisplayLabel } from '@/features/projects/ProjectView';
import { useCharacterStore } from './store';
import { MOCK_CHARACTERS_MAP } from './mockData';
import { CoreIdentityTab } from './tabs/CoreIdentityTab';
import { LifeExperienceTab } from './tabs/LifeExperienceTab';
import { PsychologicalTab } from './tabs/PsychologicalTab';
import { sanitizeCharacterProfile } from './CharacterListView';
import type {
  CharacterProfile,
  CharacterAppearance,
  CharacterCompetence,
  CharacterMemory,
  CharacterRelationship,
  DetailTab,
  DistinguishingMark,
  Gender,
  PsychologicalProfile,
} from './types';

const tabKeys: Record<DetailTab, TKey> = {
  core: 'character.detail.core',
  life: 'character.detail.life',
  psych: 'character.detail.psych',
};

const lifeSubLabels = {
  competence: 'Competence',
  memory: 'Memory',
} as const;

const validGender = (g?: string | null): Gender => {
  if (g === 'MALE' || g === 'FEMALE' || g === 'NONE') return g;
  return 'NONE';
};

/**
 * Maps a CharacterDetailDto from the backend transport into a UI CharacterProfile.
 * Sanitizes all names and internal code slugs.
 */
export function mapCharacterDetailDtoToProfile(
  dto: CharacterDetailDto,
  projectName?: string
): CharacterProfile {
  const c = dto.character;
  const cleanName = sanitizeDisplayLabel(c.name, c.code, 'Nhân vật');
  const cleanNickname = c.nickname ? sanitizeDisplayLabel(c.nickname, undefined, '') : undefined;

  const app = dto.appearance;
  const appearance: CharacterAppearance = {
    height: app?.height ?? 170,
    weight: app?.weight ?? 60,
    hair: app?.hair ?? 'Đen',
    eyes: app?.eyes ?? 'Nâu',
    skin: app?.skin ?? 'Tự nhiên',
    hairNote: app?.hairNote ?? '',
    eyesNote: app?.eyesNote ?? '',
    skinNote: app?.skinNote ?? '',
    outfitStyle: app?.outfitStyle ?? 'Giản dị',
    defaultSummary: app?.defaultSummary ?? c.defaultSummary ?? '',
  };

  const distinguishingMarks: DistinguishingMark[] = [];

  const memories: CharacterMemory[] = (dto.memories ?? []).map((m) => ({
    id: m.id,
    content: sanitizeDisplayLabel(
      m.title ? `${m.title}: ${m.description}` : m.description,
      undefined,
      m.description || m.id
    ),
  }));

  const competences: CharacterCompetence[] = (dto.competences ?? []).map((comp) => ({
    id: comp.id,
    code: comp.id,
    name: sanitizeDisplayLabel(comp.competenceName, comp.id, 'Kỹ năng'),
    domain: comp.domain || 'Chung',
    level: comp.level || 'Cơ bản',
    description: comp.competenceName,
    capabilities: [],
    limitations: [],
  }));

  const relationships: CharacterRelationship[] = (dto.relationships ?? []).map((r, idx) => ({
    id: `rel-${r.targetCharacterId}-${idx}`,
    targetId: r.targetCharacterId,
    targetName: sanitizeDisplayLabel(r.targetCharacterId, undefined, 'Người liên hệ'),
    role: 'EQUAL' as const,
    category: 'SOCIAL' as const,
    states: {
      trust: 'MODERATE' as const,
      affection: 'MODERATE' as const,
      hostility: 'NONE' as const,
      fear: 'NONE' as const,
      respect: 'MODERATE' as const,
      familiarity: 'MODERATE' as const,
      dependency: 'LOW' as const,
    },
  }));

  let psychological: PsychologicalProfile | undefined = undefined;
  if (dto.psychological) {
    psychological = {
      coreTrauma: {
        theWound: dto.psychological.summary || '',
        theLie: '',
        fundamentalFear: '',
      },
      cognitiveMoral: {
        moralAlignment: 'NEUTRAL',
        moralCode: (dto.psychological.coreValues || []).join(', '),
        cognitiveBias: '',
        breakingPoint: (dto.psychological.traumaTriggers || []).join(', '),
      },
      narrativeEngine: {
        externalWant: '',
        internalNeed: '',
        internalConflict: '',
      },
      behavioralAdaptation: {
        socialMask: {
          personaType: '',
          verbalStyle: (dto.psychological.behavioralQuirks || []).join(', '),
        },
        defenseMechanisms: {
          primaryReaction: '',
          stressTrigger: '',
          copingStrategy: '',
        },
      },
    };
  }

  return {
    id: c.id,
    code: c.code,
    projectId: undefined,
    projectName: projectName,
    basicInfo: {
      name: cleanName,
      nickname: cleanNickname,
      species: c.species,
      biologicalSex: validGender(c.biologicalSex),
      genderIdentity: validGender(c.genderIdentity),
      genderExpression: validGender(c.genderExpression),
    },
    avatarBg: 'bg-indigo-500',
    appearance,
    distinguishingMarks,
    relationships,
    competences,
    memories,
    psychological,
  };
}

export interface CharacterDetailLayoutProps {
  characterId?: string;
  client?: BackendClient;
}

export function CharacterDetailLayout({
  characterId: propId,
  client: propClient,
}: CharacterDetailLayoutProps = {}) {
  const t = useT();
  const { characterId: paramId } = useParams();
  const targetId = propId || paramId;
  const activeClient = propClient || backendClient;

  const charactersList = useCharacterStore((s) => s.charactersList);
  const storeCharacter = useCharacterStore((s) => s.character);
  const loadCharacter = useCharacterStore((s) => s.loadCharacter);
  const setStoreCharacter = useCharacterStore((s) => s.setCharacter);
  const isEditing = useCharacterStore((s) => s.isEditing);
  const startEditing = useCharacterStore((s) => s.startEditing);
  const saveEditing = useCharacterStore((s) => s.saveEditing);
  const cancelEditing = useCharacterStore((s) => s.cancelEditing);
  const isChatOpen = useCharacterStore((s) => s.isChatOpen);
  const toggleChat = useCharacterStore((s) => s.toggleChat);
  const activeTab = useCharacterStore((s) => s.activeTab);
  const setActiveTab = useCharacterStore((s) => s.setActiveTab);
  const activeLifeTab = useCharacterStore((s) => s.activeLifeTab);

  const [backendCharacter, setBackendCharacter] = useState<CharacterProfile | null>(null);
  const [notFound, setNotFound] = useState(false);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    let isMounted = true;
    if (!targetId) {
      setNotFound(true);
      return;
    }

    if (activeClient && activeClient.status !== 'unavailable') {
      setLoading(true);
      activeClient
        .getCharacterDetail({ characterId: targetId })
        .then((res) => {
          if (!isMounted) return;
          if (!res?.character) {
            setNotFound(true);
            setBackendCharacter(null);
          } else {
            const mapped = mapCharacterDetailDtoToProfile(res);
            setBackendCharacter(mapped);
            setNotFound(false);
            setStoreCharacter(mapped);
          }
        })
        .catch(() => {
          if (!isMounted) return;
          // Absent from backend: explicit not-found, never mock fallback!
          setNotFound(true);
          setBackendCharacter(null);
        })
        .finally(() => {
          if (isMounted) setLoading(false);
        });
    } else {
      // Backend unavailable: look in local store or mock map
      const found =
        charactersList.find((c) => c.id === targetId || c.code === targetId) ||
        MOCK_CHARACTERS_MAP[targetId];

      if (found) {
        const sanitized = sanitizeCharacterProfile(found);
        setBackendCharacter(sanitized);
        setNotFound(false);
        loadCharacter(targetId);
      } else {
        // If absent locally as well, render explicit not found!
        setNotFound(true);
        setBackendCharacter(null);
      }
    }

    return () => {
      isMounted = false;
    };
  }, [targetId, activeClient, charactersList, loadCharacter, setStoreCharacter]);

  const [discardOpen, setDiscardOpen] = useState(false);

  const isCompact = useIsCompact();
  const { leftRatio, setLeftRatio, isDragging, containerRef, startDragging } = useResizablePanel({
    initialRatio: 45,
    minPx: 340,
  });

  if (notFound || (!backendCharacter && !loading)) {
    return (
      <div
        data-testid="character-not-found"
        className="flex h-full flex-col items-center justify-center p-6 text-center"
      >
        <p className="text-sm text-fg-muted">
          Không tìm thấy nhân vật.
        </p>
        <Link
          to="/character"
          className="mt-3 text-xs font-semibold text-blue-600 hover:underline"
        >
          Quay lại danh sách nhân vật
        </Link>
      </div>
    );
  }

  if (loading && !backendCharacter) {
    return (
      <div className="flex h-full items-center justify-center p-6 text-center text-xs text-fg-muted">
        Đang tải...
      </div>
    );
  }

  const character = backendCharacter || storeCharacter;

  const tabLabel = (tab: DetailTab) => t(tabKeys[tab]);
  const subSectionName =
    activeTab === 'life'
      ? `${tabLabel('life')} › ${lifeSubLabels[activeLifeTab] || 'Competence'}`
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
        className="flex h-full flex-col overflow-y-auto bg-surface-muted/50 p-6 sm:p-8"
      >
        <div className="flex h-12 shrink-0 items-center justify-between border-b border-line pb-4">
          <div className="flex items-center gap-2 truncate text-xs font-semibold text-fg-muted">
            <Link to="/character" className="shrink-0 hover:text-fg">
              {t('character.title')}
            </Link>
            <span>›</span>
            <span
              className="truncate text-fg-muted font-medium"
              title={`${character.basicInfo.name} (${t('character.nickname')}: ${character.basicInfo.nickname?.trim() || t('common.unknown')})`}
            >
              {truncate(character.basicInfo.name, 18)}{' '}
              <span className="text-fg-subtle font-normal">
                ({character.basicInfo.nickname?.trim() || t('common.unknown')})
              </span>
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
                className="flex items-center gap-1.5 rounded-xl border border-line bg-surface-card px-3 py-1.5 text-xs font-medium text-fg transition-colors hover:bg-surface-muted"
              >
                <Pencil className="h-3.5 w-3.5" />
                <span>{t('common.edit')}</span>
              </button>
            ) : (
              <>
                <button
                  type="button"
                  onClick={() => setDiscardOpen(true)}
                  className="flex items-center gap-1.5 rounded-xl border border-line bg-surface-card px-3 py-1.5 text-xs font-medium text-fg transition-colors hover:bg-surface-muted"
                >
                  <X className="h-3.5 w-3.5" />
                  <span>{t('common.cancel')}</span>
                </button>
                <button
                  type="button"
                  onClick={handleSave}
                  className="flex items-center gap-1.5 rounded-xl bg-accent px-3 py-1.5 text-xs font-semibold text-white transition-colors hover:opacity-90"
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
                'flex items-center gap-1.5 rounded-xl border px-3 py-1.5 text-xs font-medium transition-colors',
                isChatOpen
                  ? 'border-accent bg-accent-soft text-accent'
                  : 'border-line bg-surface-card text-fg-muted hover:bg-surface-muted',
              )}
            >
              <MessageSquare className="h-3.5 w-3.5" />
              <span>Chat</span>
            </button>
          </div>
        </div>

        <div className="mx-auto mt-6 w-full max-w-4xl space-y-6">
          <Tabs
            items={[
              { id: 'core', label: tabLabel('core') },
              { id: 'life', label: tabLabel('life') },
              { id: 'psych', label: tabLabel('psych') },
            ]}
            value={activeTab}
            onChange={(key) => setActiveTab(key as DetailTab)}
            ariaLabel={t('character.detail.tabs')}
            variant="underline"
          />

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
