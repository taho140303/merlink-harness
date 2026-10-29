import { useState, useEffect } from 'react';
import { Dialog, DialogContent, DialogTitle, DialogDescription } from '@/shared/components/ui/dialog';
import { useT } from '@/shared/i18n/useT';
import type { DraftCharacter, DraftStep, CharacterSubStep } from '../types';
import { User, Eye, Brain, BookOpen, Lock, Shield } from 'lucide-react';
import { cn } from '@/shared/utils/cn';

interface CharacterDetailDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  character: DraftCharacter | null;
  currentStep?: DraftStep;
  currentSubStep?: CharacterSubStep;
}

export function CharacterDetailDialog({
  open,
  onOpenChange,
  character,
  currentStep,
  currentSubStep,
}: CharacterDetailDialogProps) {
  const t = useT();
  const [activeTab, setActiveTab] = useState<'appearance' | 'psych' | 'life'>('appearance');

  // Tự động chuyển tab theo sub-step hiện tại khi mở dialog
  useEffect(() => {
    if (currentStep === 'character') {
      if (currentSubStep === 'appearance') setActiveTab('appearance');
      else if (currentSubStep === 'psychological') setActiveTab('psych');
      else if (currentSubStep === 'lifeExperience') setActiveTab('life');
    }
  }, [currentStep, currentSubStep, open]);

  if (!character) return null;

  const isAppearanceUnlocked = !!character.appearance;
  const isPsychUnlocked = !!character.psychological;
  const isLifeUnlocked = (character.competences && character.competences.length > 0) || (character.memories && character.memories.length > 0);

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-4xl p-0 flex flex-col h-[85vh] overflow-hidden">
        {/* Header */}
        <div className="flex items-center justify-between border-b border-line p-6 shrink-0 bg-surface-card z-10 sticky top-0">
          <div className="flex items-center gap-3">
            <div className="flex h-12 w-12 items-center justify-center rounded-xl bg-indigo-100 text-indigo-600 dark:bg-indigo-950/60 dark:text-indigo-400">
              <User className="h-6 w-6" />
            </div>
            <div>
              <DialogTitle className="text-xl font-bold text-fg">
                {character.name}
              </DialogTitle>
              <DialogDescription className="text-sm text-fg-muted mt-0.5">
                {character.motif.description}
              </DialogDescription>
            </div>
          </div>
        </div>

        {/* Tabs — Theo tiến trình 5.1 -> 5.2 -> 5.3 */}
        <div role="tablist" aria-label={t('character.detail.appearance')} className="flex border-b border-line px-6 bg-surface-muted/50 shrink-0">
          {[
            { id: 'appearance', label: '5.1 ' + t('character.detail.appearance'), icon: Eye, unlocked: isAppearanceUnlocked },
            { id: 'psych', label: '5.2 ' + t('character.detail.psych'), icon: Brain, unlocked: isPsychUnlocked },
            { id: 'life', label: '5.3 ' + t('character.detail.life'), icon: BookOpen, unlocked: isLifeUnlocked },
          ].map((tab) => (
            <button
              key={tab.id}
              type="button"
              role="tab"
              aria-selected={activeTab === tab.id}
              tabIndex={activeTab === tab.id ? 0 : -1}
              onClick={() => setActiveTab(tab.id as any)}
              className={cn(
                "flex items-center gap-2 px-4 py-3 text-sm font-semibold border-b-2 transition-colors",
                activeTab === tab.id
                  ? "border-indigo-600 text-indigo-600 dark:border-indigo-400 dark:text-indigo-400"
                  : "border-transparent text-fg-subtle hover:text-fg"
              )}
            >
              <tab.icon className="h-4 w-4" />
              <span>{tab.label}</span>
              {!tab.unlocked && <Lock className="h-3 w-3 text-fg-subtle ml-0.5" />}
            </button>
          ))}
        </div>

        {/* Content Area */}
        <div className="flex-1 overflow-y-auto p-6 bg-surface-muted/30">
          {/* Tab 5.1: Appearance */}
          {activeTab === 'appearance' && (
            character.appearance ? (
              <div className="space-y-6">
                <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
                  <Field label={t('draft.char.species')} value={character.appearance.species} />
                  <Field label={t('draft.char.birth')} value={character.appearance.birth} />
                  <Field label={t('draft.char.height')} value={`${character.appearance.heightCm} cm`} />
                  <Field label={t('draft.char.weight')} value={`${character.appearance.weightKg} kg`} />
                </div>

                <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                  <div className="space-y-4 tier-card rounded-card p-4">
                    <h4 className="text-sm font-bold text-fg border-b border-line pb-2">{t('draft.char.features')}</h4>
                    <Field label={t('draft.char.hair')} value={`${character.appearance.hair.color} (${character.appearance.hair.style}, ${character.appearance.hair.texture})`} />
                    <Field label={t('draft.char.skin')} value={`${character.appearance.skin.tone} (${character.appearance.skin.texture})`} />
                  </div>

                  <div className="space-y-4 tier-card rounded-card p-4">
                    <h4 className="text-sm font-bold text-fg border-b border-line pb-2">{t('draft.char.eyes')}</h4>
                    <Field label={t('draft.char.eyes')} value={`Con ngươi: ${character.appearance.eyes.pupil.color} (${character.appearance.eyes.pupil.shape}), Tròng mắt: ${character.appearance.eyes.iris.color}`} />
                    <Field label="Củng mạc (tròng trắng)" value={character.appearance.eyes.sclera} />
                  </div>
                </div>

                <div className="space-y-4 tier-card rounded-card p-4">
                  <h4 className="text-sm font-bold text-fg border-b border-line pb-2">{t('draft.char.outfit')} & {t('draft.char.marks')}</h4>
                  <Field label={t('draft.char.outfit')} value={character.appearance.outfitStyle} />
                  <div>
                    <p className="text-xs font-semibold uppercase tracking-wider text-fg-subtle mb-1">{t('draft.char.marks')}</p>
                    <div className="flex flex-wrap gap-1.5">
                      {character.appearance.distinguishingMarks.map((mark, i) => (
                        <span key={i} className="rounded-md bg-surface-muted px-2 py-1 text-xs text-fg">
                          {mark}
                        </span>
                      ))}
                    </div>
                  </div>
                  <Field label={t('draft.char.summary')} value={character.appearance.summary} />
                </div>
              </div>
            ) : (
              <LockedNotice step="5.1" title="Chưa sinh dữ liệu ngoại hình" subtitle="Dữ liệu ngoại hình sẽ được sinh ở Bước 5.1 (Ngoại hình nhân vật)." />
            )
          )}

          {/* Tab 5.2: Psychological */}
          {activeTab === 'psych' && (
            character.psychological ? (
              <div className="space-y-6">
                <div className="tier-card rounded-card p-5">
                  <h3 className="text-base font-bold text-fg mb-4">{t('draft.char.coreTrauma')}</h3>
                  <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
                    <Field label={t('draft.char.wound')} value={character.psychological.coreTrauma.wound} />
                    <Field label={t('draft.char.lie')} value={character.psychological.coreTrauma.lie} />
                    <Field label={t('draft.char.fear')} value={character.psychological.coreTrauma.fear} />
                  </div>
                </div>

                <div className="tier-card rounded-card p-5">
                  <h3 className="text-base font-bold text-fg mb-4">{t('draft.char.moralFramework')}</h3>
                  <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
                    <Field label={t('draft.char.moralAlignment')} value={character.psychological.moralFramework.alignment} />
                    <Field label={t('draft.char.moralCode')} value={character.psychological.moralFramework.code} />
                    <Field label={t('draft.char.breakingPoint')} value={character.psychological.moralFramework.breakingPoint} />
                  </div>
                </div>

                <div className="tier-card rounded-card p-5">
                  <h3 className="text-base font-bold text-fg mb-4">{t('draft.char.narrativeEngine')}</h3>
                  <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
                    <Field label={t('draft.char.want')} value={character.psychological.narrativeEngine.want} />
                    <Field label={t('draft.char.need')} value={character.psychological.narrativeEngine.need} />
                    <Field label={t('draft.char.conflict')} value={character.psychological.narrativeEngine.conflict} />
                  </div>
                </div>
              </div>
            ) : (
              <LockedNotice step="5.2" title="Chưa sinh dữ liệu tâm lý" subtitle="Khung tâm lý (vết thương lòng, nỗi sợ, động cơ) sẽ được sinh ở Bước 5.2 (Tâm lý nhân vật)." />
            )
          )}

          {/* Tab 5.3: Life Experience (Memories & Competences) */}
          {activeTab === 'life' && (
            isLifeUnlocked ? (
              <div className="space-y-6">
                {/* Competences */}
                {character.competences && character.competences.length > 0 && (
                  <div className="tier-card rounded-card p-5">
                    <div className="flex items-center gap-2 mb-4">
                      <Shield className="h-5 w-5 text-indigo-500" />
                      <h3 className="text-base font-bold text-fg">{t('draft.char.competences')}</h3>
                    </div>
                    <div className="space-y-4">
                      {character.competences.map((comp) => (
                        <div key={comp.id} className="rounded-lg border border-line bg-surface-muted/50 p-4">
                          <div className="flex items-center justify-between mb-2">
                            <div>
                              <h4 className="font-bold text-fg text-sm">{comp.name}</h4>
                              <span className="text-xs text-fg-subtle">{comp.domain} {comp.sceneLearned && `• ${comp.sceneLearned}`}</span>
                            </div>
                          </div>
                          <p className="text-xs text-fg-muted mb-3">{comp.description}</p>
                          <div className="grid grid-cols-1 md:grid-cols-2 gap-3 text-xs">
                            <div>
                              <span className="font-semibold text-emerald-600 dark:text-emerald-400 block mb-1">Khả năng</span>
                              <ul className="list-disc pl-4 space-y-0.5 text-fg-muted">
                                {comp.capabilities.map((c, i) => <li key={i}>{c}</li>)}
                              </ul>
                            </div>
                            <div>
                              <span className="font-semibold text-rose-600 dark:text-rose-400 block mb-1">Giới hạn</span>
                              <ul className="list-disc pl-4 space-y-0.5 text-fg-muted">
                                {comp.limitations.map((l, i) => <li key={i}>{l}</li>)}
                              </ul>
                            </div>
                          </div>
                        </div>
                      ))}
                    </div>
                  </div>
                )}

                {/* Memories */}
                {character.memories && character.memories.length > 0 && (
                  <div className="tier-card rounded-card p-5">
                    <div className="flex items-center gap-2 mb-4">
                      <BookOpen className="h-5 w-5 text-indigo-500" />
                      <h3 className="text-base font-bold text-fg">{t('draft.char.memories')}</h3>
                    </div>
                    <div className="space-y-3">
                      {character.memories.map((mem) => (
                        <div key={mem.id} className="rounded-lg border border-line bg-surface-muted/50 p-4">
                          <div className="flex items-center justify-between mb-2">
                            <span className="px-2 py-0.5 text-xs font-bold rounded uppercase bg-indigo-100 text-indigo-700 dark:bg-indigo-950/60 dark:text-indigo-300">
                              {mem.type}
                            </span>
                            <span className="text-xs text-fg-subtle">{mem.sourceScene}</span>
                          </div>
                          <p className="text-sm italic text-fg-muted leading-relaxed">"{mem.content}"</p>
                        </div>
                      ))}
                    </div>
                  </div>
                )}
              </div>
            ) : (
              <LockedNotice step="5.3" title="Chưa sinh ký ức & năng lực" subtitle="Ký ức đầu đời và kho năng lực/tri thức sẽ được sinh ở Bước 5.3 (Ký ức & Năng lực)." />
            )
          )}
        </div>
      </DialogContent>
    </Dialog>
  );
}

function LockedNotice({ step, title, subtitle }: { step: string; title: string; subtitle: string }) {
  return (
    <div className="flex flex-col items-center justify-center py-20 text-center">
      <div className="flex h-12 w-12 items-center justify-center rounded-xl bg-surface-muted text-fg-subtle mb-3">
        <Lock className="h-6 w-6" />
      </div>
      <span className="rounded-full bg-surface-muted px-2.5 py-0.5 text-xs font-bold uppercase tracking-wider text-fg-subtle mb-2">
        Bước {step}
      </span>
      <h4 className="text-sm font-bold text-fg">{title}</h4>
      <p className="text-xs text-fg-muted mt-1 max-w-sm leading-relaxed">
        {subtitle}
      </p>
    </div>
  );
}

function Field({ label, value }: { label: string; value: string }) {
  return (
    <div>
      <p className="text-xs font-semibold uppercase tracking-wider text-fg-subtle mb-1">{label}</p>
      <p className="text-sm text-fg leading-relaxed">{value}</p>
    </div>
  );
}
