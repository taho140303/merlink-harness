import { useState } from 'react';
import { ChevronRight, Settings2, Film, Globe2, User, ShieldAlert, CheckCircle } from 'lucide-react';
import { cn } from '../../../../shared/utils/cn';
import { DraftStreamableBlock } from '../DraftStreamableBlock';
import { ThemeDisplay } from './ThemeDisplay';
import type { DraftConfig, DraftScene, DraftWorld, DraftCharacter, DraftCharacterDetail, FeedbackContext } from '../types';

// --- Config Block ---
export function ConfigBlock({ config, isStreaming, onFeedback }: { config: DraftConfig, isStreaming: boolean, onFeedback: (f: FeedbackContext) => void }) {
  return (
    <DraftStreamableBlock isLoaded={!isStreaming} loadingTitle="Mapping themes and motifs..." tag="Config">
      <div className="rounded-xl border border-slate-200 bg-white p-3 shadow-2xs dark:border-slate-800 dark:bg-slate-900">
        <div className="flex items-center justify-between mb-2">
          <div className="flex items-center gap-2">
            <Settings2 className="h-3.5 w-3.5 text-slate-400" />
            <h4 className="text-[11px] font-semibold text-slate-900 dark:text-slate-100">Config: {config.targetScope}</h4>
          </div>
          <button onClick={() => onFeedback({ id: config.id, tag: 'Config', title: config.targetScope })} className="text-[10px] text-slate-500 hover:text-blue-600">Góp ý</button>
        </div>
        <div className="rounded-lg bg-slate-50 p-2 text-[11px] dark:bg-slate-800/50">
          <span className="text-slate-400 block mb-0.5">Theme</span>
          <span className="font-serif italic text-slate-800 dark:text-slate-200">"{config.theme}"</span>
        </div>
        <div className="mt-2 flex flex-wrap gap-1.5">
          {config.motifs.map(m => (
            <span key={m} className="rounded-full bg-indigo-50 px-2 py-0.5 text-[9px] font-medium text-indigo-700 dark:bg-indigo-950/60 dark:text-indigo-300">{m}</span>
          ))}
        </div>
      </div>
    </DraftStreamableBlock>
  );
}

// --- Scene Block ---
export function SceneBlock({ 
  scene, 
  isStreaming, 
  phase, 
  onFeedback, 
  onCharacterClick,
  children 
}: { 
  scene: DraftScene; 
  isStreaming: boolean; 
  phase: string; 
  onFeedback: (f: FeedbackContext) => void; 
  onCharacterClick: (charName: string) => void;
  children?: React.ReactNode;
}) {
  const [isOpen, setIsOpen] = useState(false);

  return (
    <DraftStreamableBlock isLoaded={!isStreaming} loadingTitle="Configuring scene parameters..." tag="Scene">
      <div className="rounded-lg border border-slate-200 bg-white shadow-2xs dark:border-slate-800 dark:bg-slate-900 overflow-hidden">
        <div 
          className="flex items-center justify-between p-3 cursor-pointer hover:bg-slate-50 dark:hover:bg-slate-800/50"
          onClick={() => setIsOpen(!isOpen)}
        >
          <div className="flex items-center gap-2">
            <ChevronRight className={cn("h-3.5 w-3.5 text-slate-400 transition-transform", isOpen && "rotate-90")} />
            <Film className="h-3.5 w-3.5 text-slate-500" />
            <span className="text-[11px] font-semibold text-slate-900 dark:text-slate-100">{scene.name}</span>
          </div>
          <button onClick={(e) => { e.stopPropagation(); onFeedback({ id: scene.id, tag: 'Scene', title: scene.name }); }} className="text-[10px] text-slate-500 hover:text-blue-600">Góp ý</button>
        </div>

        {isOpen && (
          <div className="border-t border-slate-100 p-3 dark:border-slate-800 space-y-3">
            <p className="text-[11px] text-slate-600 dark:text-slate-400">{scene.summarize}</p>
            
            <div className="grid grid-cols-2 gap-2 text-[10px]">
              <div className="rounded bg-slate-50 p-2 dark:bg-slate-800/50">
                <span className="text-slate-400 block mb-0.5">Style Preset</span>
                <span className="font-medium text-slate-700 dark:text-slate-300">{scene.stylePreset}</span>
              </div>
              <div className="rounded bg-slate-50 p-2 dark:bg-slate-800/50">
                <span className="text-slate-400 block mb-0.5">POV</span>
                <span className="font-medium text-slate-700 dark:text-slate-300">{scene.narratorConfig}</span>
              </div>
            </div>

            {/* Scene Characters (Clickable) */}
            <div>
              <span className="text-[10px] uppercase tracking-wide text-slate-400 block mb-1">Scene Characters</span>
              <div className="flex flex-wrap gap-1.5">
                {scene.characters.map(c => (
                  <button 
                    key={c.name}
                    onClick={() => onCharacterClick(c.name)}
                    className={cn(
                      "px-2 py-1 rounded text-[10px] font-medium border flex items-center gap-1 transition-colors cursor-pointer hover:shadow-sm",
                      c.isPov 
                        ? "bg-amber-100 border-amber-200 text-amber-800 dark:bg-amber-950/60 dark:border-amber-900/60 dark:text-amber-300 hover:bg-amber-200 dark:hover:bg-amber-900" 
                        : "bg-slate-100 border-slate-200 text-slate-700 dark:bg-slate-800 dark:border-slate-700 dark:text-slate-300 hover:bg-slate-200 dark:hover:bg-slate-700"
                    )}
                  >
                    <User className="h-3 w-3 opacity-50" />
                    {c.name} 
                    {c.isPov && <span className="text-[8px] opacity-70 font-bold uppercase ml-0.5">(POV)</span>}
                  </button>
                ))}
              </div>
            </div>

            <div className="grid grid-cols-2 gap-2 text-[9px]">
              <div className="rounded border border-rose-100 bg-rose-50 p-2 dark:border-rose-900/60 dark:bg-rose-950/30">
                <span className="font-semibold text-rose-700 dark:text-rose-300 flex items-center gap-1 mb-1"><ShieldAlert className="h-2.5 w-2.5" /> Forbidden</span>
                <ul className="list-disc pl-3 text-rose-900 dark:text-rose-100">
                  {scene.forbidden.map(f => <li key={f}>{f}</li>)}
                </ul>
              </div>
              <div className="rounded border border-emerald-100 bg-emerald-50 p-2 dark:border-emerald-900/60 dark:bg-emerald-950/30">
                <span className="font-semibold text-emerald-700 dark:text-emerald-300 flex items-center gap-1 mb-1"><CheckCircle className="h-2.5 w-2.5" /> Allowed</span>
                <ul className="list-disc pl-3 text-emerald-900 dark:text-emerald-100">
                  {scene.allowed.map(a => <li key={a}>{a}</li>)}
                </ul>
              </div>
            </div>
            {children && <div className="mt-3 space-y-2">{children}</div>}
          </div>
        )}
      </div>
    </DraftStreamableBlock>
  );
}

// --- World Block ---
export function WorldBlock({ world, isStreaming, onFeedback }: { world: DraftWorld, isStreaming: boolean, onFeedback: (f: FeedbackContext) => void }) {
  return (
    <DraftStreamableBlock isLoaded={!isStreaming} loadingTitle="Constructing world settings..." tag="World">
      <div className="rounded-lg border border-slate-200 bg-white p-3 shadow-2xs dark:border-slate-800 dark:bg-slate-900">
        <div className="flex items-center justify-between mb-2">
          <div className="flex items-center gap-2">
            <Globe2 className="h-3.5 w-3.5 text-slate-400" />
            <h4 className="text-[11px] font-semibold text-slate-900 dark:text-slate-100">{world.locationName}</h4>
          </div>
          <button onClick={() => onFeedback({ id: world.id, tag: 'World', title: world.locationName })} className="text-[10px] text-slate-500 hover:text-blue-600">Góp ý</button>
        </div>
        <p className="text-[10px] text-slate-600 dark:text-slate-400"><span className="font-medium">Sensory:</span> {world.sensory}</p>
        <div className="mt-2 flex flex-wrap gap-1">
          {world.lore.map(l => <span key={l} className="rounded bg-blue-50 px-1.5 py-0.5 text-[9px] text-blue-700 dark:bg-blue-950/60 dark:text-blue-300">{l}</span>)}
        </div>
      </div>
    </DraftStreamableBlock>
  );
}

// --- Character Block ---
export function CharacterBlock({ char, isStreaming, phase, onFeedback, children }: { char: DraftCharacter, isStreaming: boolean, phase: string, onFeedback: (f: FeedbackContext) => void, children?: React.ReactNode }) {
  const [isOpen, setIsOpen] = useState(false);
  return (
    <DraftStreamableBlock isLoaded={!isStreaming} loadingTitle="Assembling character roster..." tag="Character">
      <div className="rounded-lg border border-slate-200 bg-white shadow-2xs dark:border-slate-800 dark:bg-slate-900 overflow-hidden">
        <div 
          className="flex items-center justify-between p-2 cursor-pointer hover:bg-slate-50 dark:hover:bg-slate-800/50"
          onClick={() => setIsOpen(!isOpen)}
        >
          <div className="flex items-center gap-2">
            <ChevronRight className={cn("h-3.5 w-3.5 text-slate-400 transition-transform", isOpen && "rotate-90")} />
            <User className="h-3.5 w-3.5 text-indigo-500" />
            <span className="text-[11px] font-semibold text-slate-900 dark:text-slate-100">{char.name}</span>
            <span className="rounded bg-slate-100 px-1.5 py-0.5 text-[9px] text-slate-600 dark:bg-slate-800 dark:text-slate-300">{char.role}</span>
          </div>
          <button onClick={(e) => { e.stopPropagation(); onFeedback({ id: char.id, tag: 'Character', title: char.name }); }} className="text-[10px] text-slate-500 hover:text-blue-600">Góp ý</button>
        </div>
        {isOpen && children && <div className="border-t border-slate-100 p-2 dark:border-slate-800">{children}</div>}
      </div>
    </DraftStreamableBlock>
  );
}

// --- Character Detail Block ---
export function CharacterDetailBlock({ detail, isStreaming, onFeedback }: { detail: DraftCharacterDetail, isStreaming: boolean, onFeedback: (f: FeedbackContext) => void }) {
  return (
    <DraftStreamableBlock isLoaded={!isStreaming} loadingTitle="Simulating psychological profiles..." tag="Detail">
      <div className="rounded border border-slate-200 bg-slate-50 p-2.5 dark:border-slate-700 dark:bg-slate-950/50">
        <div className="flex justify-between items-center mb-1">
          <span className="text-[10px] font-semibold text-slate-700 dark:text-slate-300">Deep Dive Analysis</span>
          <button onClick={() => onFeedback({ id: detail.id, tag: 'Detail', title: detail.name })} className="text-[9px] text-slate-500 hover:text-blue-600">Góp ý</button>
        </div>
        <div className="space-y-1.5 text-[10px]">
          <p><span className="font-medium text-slate-500">Tính cách:</span> {detail.personality}</p>
          <p><span className="font-medium text-slate-500">Hành động:</span> {detail.actionLogic}</p>
          <p className="text-indigo-700 dark:text-indigo-300"><span className="font-medium">Theme Role:</span> {detail.themeAlignment}</p>
        </div>
      </div>
    </DraftStreamableBlock>
  );
}
