import { useState } from 'react';
import { Dialog, DialogContent, DialogTitle, DialogDescription } from '@/shared/components/ui/dialog';
import { useT } from '@/shared/i18n/useT';
import type { DraftCharacter } from '../types';
import { User, Shield, Eye, Brain, Heart, BookOpen, AlertTriangle, CheckCircle2 } from 'lucide-react';
import { cn } from '@/shared/utils/cn';

interface CharacterDetailDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  character: DraftCharacter | null;
}

export function CharacterDetailDialog({ open, onOpenChange, character }: CharacterDetailDialogProps) {
  const t = useT();
  const [activeTab, setActiveTab] = useState<'appearance' | 'life' | 'psych'>('appearance');

  if (!character) return null;

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-4xl p-0 flex flex-col h-[85vh] overflow-hidden">
        {/* Header */}
        <div className="flex items-center justify-between border-b border-line p-6  shrink-0 bg-surface-card  z-10 sticky top-0">
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

        {/* Tabs */}
        <div role="tablist" aria-label={t('character.detail.appearance')} className="flex border-b border-line px-6 bg-surface-muted/50   shrink-0">
          {[
            { id: 'appearance', label: t('character.detail.appearance'), icon: Eye },
            { id: 'life', label: t('character.detail.life'), icon: BookOpen },
            { id: 'psych', label: t('character.detail.psych'), icon: Brain },
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
                  : "border-transparent text-fg-subtle hover:text-fg -subtle "
              )}
            >
              <tab.icon className="h-4 w-4" />
              {tab.label}
            </button>
          ))}
        </div>

        {/* Content Area */}
        <div className="flex-1 overflow-y-auto p-6 bg-surface-muted/30 ">
          {activeTab === 'appearance' && character.appearance && (
            <div className="space-y-6   -from-bottom-2">
              <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
                <Field label={t('draft.char.species')} value={character.appearance.species} />
                <Field label={t('draft.char.birth')} value={character.appearance.birth} />
                <Field label={t('draft.char.height')} value={`${character.appearance.heightCm} cm`} />
                <Field label={t('draft.char.weight')} value={`${character.appearance.weightKg} kg`} />
              </div>

              <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                <div className="space-y-4 tier-card rounded-card p-4  ">
                  <h3 className="text-sm font-bold text-fg flex items-center gap-2">
                    <CheckCircle2 className="h-4 w-4 text-emerald-500" /> {t('draft.char.features')}
                  </h3>
                  <Field label={t('draft.char.hair')} value={`${character.appearance.hair.color}, ${character.appearance.hair.length}, ${character.appearance.hair.style} (${character.appearance.hair.texture})`} />
                  <Field label={t('draft.char.skin')} value={`${character.appearance.skin.tone}, ${character.appearance.skin.texture}`} />
                  <Field label={t('draft.char.eyes')} value={t('draft.char.eyeDescription', { iris: character.appearance.eyes.iris.color, pupil: character.appearance.eyes.pupil.color })} />
                </div>

                <div className="space-y-4 tier-card rounded-card p-4  ">
                  <h3 className="text-sm font-bold text-fg flex items-center gap-2">
                    <AlertTriangle className="h-4 w-4 text-amber-500" /> {t('draft.char.marks')}
                  </h3>
                  <div className="space-y-2">
                    {character.appearance.distinguishingMarks.map((mark, i) => (
                      <p key={i} className="text-sm text-fg-muted  pl-3 border-l-2 border-amber-300">
                        {mark}
                      </p>
                    ))}
                  </div>
                  <Field label={t('draft.char.outfit')} value={character.appearance.outfitStyle} />
                </div>
              </div>
              
              <div className="tier-card rounded-card p-4  ">
                <Field label={t('draft.char.summary')} value={character.appearance.summary || '-'} />
              </div>
            </div>
          )}

          {activeTab === 'life' && (
            <div className="space-y-6   -from-bottom-2">
              <div>
                <h3 className="text-base font-bold text-fg mb-3 flex items-center gap-2">
                  <Shield className="h-5 w-5 text-blue-500" /> {t('draft.char.competences')}
                </h3>
                <div className="space-y-3">
                  {character.competences.map(comp => (
                    <div key={comp.id} className="tier-card rounded-card p-4  ">
                      <div className="flex items-center justify-between mb-2">
                        <h4 className="text-sm font-bold text-fg">{comp.name}</h4>
                        <span className="text-xs font-mono text-fg-subtle bg-surface-muted  px-2 py-0.5 rounded">{comp.domain} · {comp.sceneLearned}</span>
                      </div>
                      <p className="text-sm text-fg-muted  mb-3">{comp.description}</p>
                      <div className="grid grid-cols-2 gap-4">
                        <div>
                          <span className="text-xs font-bold uppercase text-emerald-600 dark:text-emerald-400">{t('draft.char.capabilities')}</span>
                          <ul className="list-disc pl-4 mt-1 text-sm text-fg-muted ">
                            {comp.capabilities.map(c => <li key={c}>{c}</li>)}
                          </ul>
                        </div>
                        <div>
                          <span className="text-xs font-bold uppercase text-rose-600 dark:text-rose-400">{t('draft.char.limitations')}</span>
                          <ul className="list-disc pl-4 mt-1 text-sm text-fg-muted ">
                            {comp.limitations.map(l => <li key={l}>{l}</li>)}
                          </ul>
                        </div>
                      </div>
                    </div>
                  ))}
                </div>
              </div>

              <div>
                <h3 className="text-base font-bold text-fg mb-3 flex items-center gap-2">
                  <Heart className="h-5 w-5 text-rose-500" /> {t('draft.char.memories')}
                </h3>
                <div className="space-y-3">
                  {character.memories.map(mem => (
                    <div key={mem.id} className="tier-card rounded-card p-4  ">
                      <div className="flex items-center gap-2 mb-2">
                        <span className={cn(
                          "px-2 py-0.5 text-xs font-bold rounded uppercase",
                          mem.type === 'Episodic' ? "bg-purple-100 text-purple-700 dark:bg-purple-950/60 dark:text-purple-300" :
                          mem.type === 'Semantic' ? "bg-sky-100 text-sky-700 dark:bg-sky-950/60 dark:text-sky-300" :
                          "bg-teal-100 text-teal-700 dark:bg-teal-950/60 dark:text-teal-300"
                        )}>
                          {mem.type}
                        </span>
                        <span className="text-xs text-fg-subtle">{t('draft.char.source', { source: mem.sourceScene })}</span>
                      </div>
                      <p className="text-sm italic text-fg-muted  leading-relaxed">"{mem.content}"</p>
                    </div>
                  ))}
                </div>
              </div>
            </div>
          )}

          {activeTab === 'psych' && character.psychological && (
            <div className="space-y-6   -from-bottom-2">
              <div className="tier-card rounded-card p-5  ">
                <h3 className="text-base font-bold text-fg mb-4">{t('draft.char.coreTrauma')}</h3>
                <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
                  <Field label={t('draft.char.wound')} value={character.psychological.coreTrauma.wound} />
                  <Field label={t('draft.char.lie')} value={character.psychological.coreTrauma.lie} />
                  <Field label={t('draft.char.fear')} value={character.psychological.coreTrauma.fear} />
                </div>
              </div>

              <div className="tier-card rounded-card p-5  ">
                <h3 className="text-base font-bold text-fg mb-4">{t('draft.char.moralFramework')}</h3>
                <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
                  <Field label={t('draft.char.moralAlignment')} value={character.psychological.moralFramework.alignment} />
                  <Field label={t('draft.char.moralCode')} value={character.psychological.moralFramework.code} />
                  <Field label={t('draft.char.breakingPoint')} value={character.psychological.moralFramework.breakingPoint} />
                </div>
              </div>

              <div className="tier-card rounded-card p-5  ">
                <h3 className="text-base font-bold text-fg mb-4">{t('draft.char.narrativeEngine')}</h3>
                <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
                  <Field label={t('draft.char.want')} value={character.psychological.narrativeEngine.want} />
                  <Field label={t('draft.char.need')} value={character.psychological.narrativeEngine.need} />
                  <Field label={t('draft.char.conflict')} value={character.psychological.narrativeEngine.conflict} />
                </div>
              </div>
            </div>
          )}
        </div>
      </DialogContent>
    </Dialog>
  );
}

function Field({ label, value }: { label: string; value: string }) {
  return (
    <div>
      <p className="text-xs font-semibold uppercase tracking-wider text-fg-subtle mb-1">{label}</p>
      <p className="text-sm text-fg  leading-relaxed">{value}</p>
    </div>
  );
}
