import { useState } from 'react';
import { Dialog, DialogContent, DialogTitle } from '../../../../shared/components/ui/dialog';
import { useT } from '../../../../shared/i18n/useT';
import type { DraftCharacter } from '../types';
import { User, Shield, Eye, Brain, Heart, BookOpen, AlertTriangle, CheckCircle2 } from 'lucide-react';
import { cn } from '../../../../shared/utils/cn';

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
        <div className="flex items-center justify-between border-b border-slate-200 p-6 dark:border-slate-800 shrink-0 bg-white dark:bg-slate-950 z-10 sticky top-0">
          <div className="flex items-center gap-3">
            <div className="flex h-12 w-12 items-center justify-center rounded-xl bg-indigo-100 text-indigo-600 dark:bg-indigo-950/60 dark:text-indigo-400">
              <User className="h-6 w-6" />
            </div>
            <div>
              <DialogTitle className="text-xl font-bold text-slate-900 dark:text-slate-100">
                {character.name}
              </DialogTitle>
              <p className="text-sm text-slate-500 dark:text-slate-400 mt-0.5">
                {character.motif.description}
              </p>
            </div>
          </div>
        </div>

        {/* Tabs */}
        <div className="flex border-b border-slate-200 px-6 bg-slate-50/50 dark:border-slate-800 dark:bg-slate-900/30 shrink-0">
          {[
            { id: 'appearance', label: t('character.detail.appearance'), icon: Eye },
            { id: 'life', label: t('character.detail.life'), icon: BookOpen },
            { id: 'psych', label: t('character.detail.psych'), icon: Brain },
          ].map((tab) => (
            <button
              key={tab.id}
              onClick={() => setActiveTab(tab.id as any)}
              className={cn(
                "flex items-center gap-2 px-4 py-3 text-sm font-semibold border-b-2 transition-colors",
                activeTab === tab.id
                  ? "border-indigo-600 text-indigo-600 dark:border-indigo-400 dark:text-indigo-400"
                  : "border-transparent text-slate-500 hover:text-slate-800 dark:text-slate-400 dark:hover:text-slate-200"
              )}
            >
              <tab.icon className="h-4 w-4" />
              {tab.label}
            </button>
          ))}
        </div>

        {/* Content Area */}
        <div className="flex-1 overflow-y-auto p-6 bg-slate-50/30 dark:bg-slate-950/20">
          {activeTab === 'appearance' && character.appearance && (
            <div className="space-y-6 animate-in fade-in slide-in-from-bottom-2">
              <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
                <Field label={t('draft.char.species')} value={character.appearance.species} />
                <Field label={t('draft.char.birth')} value={character.appearance.birth} />
                <Field label={t('draft.char.height')} value={`${character.appearance.heightCm} cm`} />
                <Field label={t('draft.char.weight')} value={`${character.appearance.weightKg} kg`} />
              </div>

              <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                <div className="space-y-4 rounded-xl border border-slate-200 bg-white p-4 dark:border-slate-800 dark:bg-slate-900">
                  <h3 className="text-sm font-bold text-slate-900 dark:text-slate-100 flex items-center gap-2">
                    <CheckCircle2 className="h-4 w-4 text-emerald-500" /> {t('draft.char.features')}
                  </h3>
                  <Field label={t('draft.char.hair')} value={`${character.appearance.hair.color}, ${character.appearance.hair.length}, ${character.appearance.hair.style} (${character.appearance.hair.texture})`} />
                  <Field label={t('draft.char.skin')} value={`${character.appearance.skin.tone}, ${character.appearance.skin.texture}`} />
                  <Field label={t('draft.char.eyes')} value={`Mắt ${character.appearance.eyes.iris.color}, tròng đen ${character.appearance.eyes.pupil.color}`} />
                </div>

                <div className="space-y-4 rounded-xl border border-slate-200 bg-white p-4 dark:border-slate-800 dark:bg-slate-900">
                  <h3 className="text-sm font-bold text-slate-900 dark:text-slate-100 flex items-center gap-2">
                    <AlertTriangle className="h-4 w-4 text-amber-500" /> {t('draft.char.marks')}
                  </h3>
                  <div className="space-y-2">
                    {character.appearance.distinguishingMarks.map((mark, i) => (
                      <p key={i} className="text-sm text-slate-700 dark:text-slate-300 pl-3 border-l-2 border-amber-300">
                        {mark}
                      </p>
                    ))}
                  </div>
                  <Field label={t('draft.char.outfit')} value={character.appearance.outfitStyle} />
                </div>
              </div>
              
              <div className="rounded-xl border border-slate-200 bg-white p-4 dark:border-slate-800 dark:bg-slate-900">
                <Field label={t('draft.char.summary')} value={character.appearance.summary || '-'} />
              </div>
            </div>
          )}

          {activeTab === 'life' && (
            <div className="space-y-6 animate-in fade-in slide-in-from-bottom-2">
              <div>
                <h3 className="text-base font-bold text-slate-900 dark:text-slate-100 mb-3 flex items-center gap-2">
                  <Shield className="h-5 w-5 text-blue-500" /> {t('draft.char.competences')}
                </h3>
                <div className="space-y-3">
                  {character.competences.map(comp => (
                    <div key={comp.id} className="rounded-xl border border-slate-200 bg-white p-4 dark:border-slate-800 dark:bg-slate-900">
                      <div className="flex items-center justify-between mb-2">
                        <h4 className="text-sm font-bold text-slate-900 dark:text-slate-100">{comp.name}</h4>
                        <span className="text-xs font-mono text-slate-500 bg-slate-100 dark:bg-slate-800 px-2 py-0.5 rounded">{comp.domain} · {comp.sceneLearned}</span>
                      </div>
                      <p className="text-sm text-slate-700 dark:text-slate-300 mb-3">{comp.description}</p>
                      <div className="grid grid-cols-2 gap-4">
                        <div>
                          <span className="text-[11px] font-bold uppercase text-emerald-600 dark:text-emerald-400">Capabilities</span>
                          <ul className="list-disc pl-4 mt-1 text-sm text-slate-700 dark:text-slate-300">
                            {comp.capabilities.map(c => <li key={c}>{c}</li>)}
                          </ul>
                        </div>
                        <div>
                          <span className="text-[11px] font-bold uppercase text-rose-600 dark:text-rose-400">Limitations</span>
                          <ul className="list-disc pl-4 mt-1 text-sm text-slate-700 dark:text-slate-300">
                            {comp.limitations.map(l => <li key={l}>{l}</li>)}
                          </ul>
                        </div>
                      </div>
                    </div>
                  ))}
                </div>
              </div>

              <div>
                <h3 className="text-base font-bold text-slate-900 dark:text-slate-100 mb-3 flex items-center gap-2">
                  <Heart className="h-5 w-5 text-rose-500" /> {t('draft.char.memories')}
                </h3>
                <div className="space-y-3">
                  {character.memories.map(mem => (
                    <div key={mem.id} className="rounded-xl border border-slate-200 bg-white p-4 dark:border-slate-800 dark:bg-slate-900">
                      <div className="flex items-center gap-2 mb-2">
                        <span className={cn(
                          "px-2 py-0.5 text-[10px] font-bold rounded uppercase",
                          mem.type === 'Episodic' ? "bg-purple-100 text-purple-700 dark:bg-purple-950/60 dark:text-purple-300" :
                          mem.type === 'Semantic' ? "bg-sky-100 text-sky-700 dark:bg-sky-950/60 dark:text-sky-300" :
                          "bg-teal-100 text-teal-700 dark:bg-teal-950/60 dark:text-teal-300"
                        )}>
                          {mem.type}
                        </span>
                        <span className="text-xs text-slate-400">Từ: {mem.sourceScene}</span>
                      </div>
                      <p className="text-sm italic text-slate-700 dark:text-slate-300 leading-relaxed">"{mem.content}"</p>
                    </div>
                  ))}
                </div>
              </div>
            </div>
          )}

          {activeTab === 'psych' && character.psychological && (
            <div className="space-y-6 animate-in fade-in slide-in-from-bottom-2">
              <div className="rounded-xl border border-slate-200 bg-white p-5 dark:border-slate-800 dark:bg-slate-900">
                <h3 className="text-base font-bold text-slate-900 dark:text-slate-100 mb-4">Core Trauma</h3>
                <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
                  <Field label="Vết thương lòng" value={character.psychological.coreTrauma.wound} />
                  <Field label="Lời dối trá" value={character.psychological.coreTrauma.lie} />
                  <Field label="Nỗi sợ" value={character.psychological.coreTrauma.fear} />
                </div>
              </div>

              <div className="rounded-xl border border-slate-200 bg-white p-5 dark:border-slate-800 dark:bg-slate-900">
                <h3 className="text-base font-bold text-slate-900 dark:text-slate-100 mb-4">Moral Framework</h3>
                <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
                  <Field label="Đạo đức" value={character.psychological.moralFramework.alignment} />
                  <Field label="Quy tắc" value={character.psychological.moralFramework.code} />
                  <Field label="Điểm gãy" value={character.psychological.moralFramework.breakingPoint} />
                </div>
              </div>

              <div className="rounded-xl border border-slate-200 bg-white p-5 dark:border-slate-800 dark:bg-slate-900">
                <h3 className="text-base font-bold text-slate-900 dark:text-slate-100 mb-4">Narrative Engine</h3>
                <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
                  <Field label="Mong muốn" value={character.psychological.narrativeEngine.want} />
                  <Field label="Nhu cầu" value={character.psychological.narrativeEngine.need} />
                  <Field label="Xung đột" value={character.psychological.narrativeEngine.conflict} />
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
      <p className="text-[11px] font-semibold uppercase tracking-wider text-slate-400 mb-1">{label}</p>
      <p className="text-sm text-slate-800 dark:text-slate-200 leading-relaxed">{value}</p>
    </div>
  );
}
