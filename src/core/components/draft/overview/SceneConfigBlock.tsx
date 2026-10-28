import { useState } from 'react';
import { ChevronDown, ChevronUp, Film } from 'lucide-react';
import { cn } from '../../../../shared/utils/cn';
import { useT } from '../../../../shared/i18n/useT';
import type { SceneConfigPhase, SceneConfig } from '../types';

interface SceneConfigBlockProps {
  config: SceneConfigPhase;
  isFeedbackMode?: boolean;
  onAttachFeedback?: (id: string, tag: string, title: string) => void;
}

export function SceneConfigBlock({ config, isFeedbackMode, onAttachFeedback }: SceneConfigBlockProps) {
  const t = useT();
  const [expandedArcs, setExpandedArcs] = useState<Set<number>>(new Set([0]));
  const [expandedChapters, setExpandedChapters] = useState<Set<string>>(new Set());

  const toggleArc = (idx: number) => {
    setExpandedArcs(prev => {
      const next = new Set(prev);
      if (next.has(idx)) next.delete(idx); else next.add(idx);
      return next;
    });
  };

  const toggleChapter = (key: string) => {
    setExpandedChapters(prev => {
      const next = new Set(prev);
      if (next.has(key)) next.delete(key); else next.add(key);
      return next;
    });
  };

  const handleAttach = (e: React.MouseEvent, id: string, tag: string, title: string) => {
    if (isFeedbackMode && onAttachFeedback) {
      e.stopPropagation();
      onAttachFeedback(id, tag, title);
    }
  };

  return (
    <div className="w-full space-y-4">
      {config.arcs.map((arc, arcIdx) => {
        const isArcExpanded = expandedArcs.has(arcIdx);
        return (
          <div
            key={arcIdx}
            className={cn(
              "tier-card rounded-card overflow-hidden transition-colors duration-300",
              isFeedbackMode && "hover:!border-2 hover:!border-dashed hover:!border-purple-500 hover:bg-purple-500/5 cursor-none"
            )}
            onClick={(e) => handleAttach(e, `arc-${arcIdx}`, 'arc', arc.displayName)}
          >
            <button
              type="button"
              onClick={(e) => { e.stopPropagation(); toggleArc(arcIdx); }}
              className="flex w-full items-center justify-between p-4 hover:bg-surface-muted  transition-colors"
            >
              <div className="flex items-center gap-3">
                <span className="flex h-8 w-8 items-center justify-center rounded-lg bg-indigo-100 font-bold text-indigo-700 dark:bg-indigo-950/60 dark:text-indigo-300">
                  {arc.position}
                </span>
                <div className="text-left">
                  <h3 className="text-base font-bold text-fg">{arc.displayName}</h3>
                  <p className="text-sm text-fg-muted">{t('draft.scene.chapterCount', { count: arc.chapters.length })}</p>
                </div>
              </div>
              {isArcExpanded ? <ChevronUp className="h-5 w-5 text-fg-subtle" /> : <ChevronDown className="h-5 w-5 text-fg-subtle" />}
            </button>

            {isArcExpanded && (
              <div className="border-t border-line p-4  space-y-3">
                {arc.chapters.map((chap, chapIdx) => {
                  const chapKey = `${arcIdx}-${chapIdx}`;
                  const isChapExpanded = expandedChapters.has(chapKey);
                  return (
                    <div
                      key={chapKey}
                      className="rounded-lg border border-line bg-surface-muted/50   overflow-hidden"
                    >
                      <button
                        type="button"
                        onClick={(e) => { e.stopPropagation(); toggleChapter(chapKey); }}
                        className="flex w-full items-center justify-between p-3 hover:bg-surface-muted  transition-colors"
                      >
                        <div className="flex items-center gap-2">
                          <span className="flex h-6 w-6 items-center justify-center rounded bg-surface-card text-xs font-bold text-fg-muted   ">
                            {chap.position}
                          </span>
                          <span className="text-sm font-semibold text-fg">{chap.displayName}</span>
                          <span className="text-xs text-fg-muted">({t('draft.scene.sceneCount', { count: chap.scenes.length })})</span>
                        </div>
                        {isChapExpanded ? <ChevronUp className="h-4 w-4 text-fg-subtle" /> : <ChevronDown className="h-4 w-4 text-fg-subtle" />}
                      </button>

                      {isChapExpanded && (
                        <div className="border-t border-line p-3  space-y-3">
                          {chap.scenes.map((scene) => (
                            <SceneCard
                              key={scene.code}
                              scene={scene}
                              isFeedbackMode={isFeedbackMode}
                              onAttachFeedback={onAttachFeedback}
                            />
                          ))}
                        </div>
                      )}
                    </div>
                  );
                })}
              </div>
            )}
          </div>
        );
      })}
    </div>
  );
}

function SceneCard({ scene, isFeedbackMode, onAttachFeedback }: { scene: SceneConfig; isFeedbackMode?: boolean; onAttachFeedback?: (id: string, tag: string, title: string) => void }) {
  const t = useT();
  const handleAttach = (e: React.MouseEvent) => {
    if (isFeedbackMode && onAttachFeedback) {
      e.stopPropagation();
      onAttachFeedback(scene.code, 'scene', scene.displayName);
    }
  };

  return (
    <div
      onClick={handleAttach}
      className={cn(
        "tier-card rounded-card p-4 transition-colors duration-300",
        isFeedbackMode && "hover:!border-2 hover:!border-dashed hover:!border-purple-500 hover:bg-purple-500/5 cursor-none"
      )}
    >
      <div className="flex items-center gap-2 mb-2">
        <Film className="h-4 w-4 text-blue-500" />
        <span className="text-sm font-bold text-fg">
          {t('draft.scene.sceneTitle', { position: scene.position, displayName: scene.displayName })}
        </span>
        <span className="rounded bg-surface-muted px-1.5 py-0.5 text-xs font-mono text-fg-subtle">{scene.code}</span>
      </div>

      <p className="text-sm text-fg-muted mb-4 leading-relaxed">
        {scene.summary}
      </p>

      {/* Metadata: Title Group + Text */}
      <div className="grid grid-cols-1 sm:grid-cols-3 gap-4 pt-3 border-t border-line ">
        <div>
          <span className="text-xs font-bold uppercase tracking-wider text-fg-subtle">
            {t('draft.scene.stylePreset')}
          </span>
          <p className="mt-1 text-sm font-medium text-fg ">
            {scene.stylePresetCode}
          </p>
        </div>
        <div>
          <span className="text-xs font-bold uppercase tracking-wider text-fg-subtle">
            {t('draft.scene.narrator')}
          </span>
          <p className="mt-1 text-sm font-medium text-fg ">
            {scene.narratorConfigCode}
          </p>
        </div>
        <div>
          <span className="text-xs font-bold uppercase tracking-wider text-fg-subtle">
            {t('draft.scene.lore')}
          </span>
          <p className={cn(
            "mt-1 text-sm font-medium",
            scene.loreEntriesCode 
              ? "text-fg " 
              : "text-fg-subtle italic"
          )}>
            {scene.loreEntriesCode || t('draft.scene.none')}
          </p>
        </div>
      </div>

      {/* Characters Group */}
      <div className="mt-4">
        <span className="text-xs font-bold uppercase tracking-wider text-fg-subtle">
          {t('draft.scene.characters')}
        </span>
        <div className="mt-1 flex flex-wrap gap-x-4 gap-y-1">
          {scene.characters.map((char, i) => (
            <span
              key={i}
              className={cn(
                "text-sm",
                char.isPov
                  ? "font-bold text-indigo-600 dark:text-indigo-400"
                  : "text-fg-muted "
              )}
            >
              {char.displayName} {char.isPov && "(POV)"}
            </span>
          ))}
        </div>
      </div>
    </div>
  );
}
