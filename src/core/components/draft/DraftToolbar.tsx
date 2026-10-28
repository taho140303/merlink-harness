import { useState, useRef, useEffect } from 'react';
import { Layers, ChevronDown, RotateCcw, ArrowRight, Check, MessageSquareQuote } from 'lucide-react';
import { cn } from '../../../shared/utils/cn';
import { useT } from '../../../shared/i18n/useT';
import type { DraftVersion } from './types';

interface DraftToolbarProps {
  currentVersion: string;
  versionHistory: DraftVersion[];
  onVersionChange: (v: string) => void;
  onRegenerate: () => void;
  onNext?: () => void;
  onStart?: () => void;
  isFeedbackMode: boolean;
  onToggleFeedback: () => void;
}

export function DraftToolbar({
  currentVersion,
  versionHistory,
  onVersionChange,
  onRegenerate,
  onNext,
  onStart,
  isFeedbackMode,
  onToggleFeedback,
}: DraftToolbarProps) {
  const t = useT();
  const [versionOpen, setVersionOpen] = useState(false);
  const dropdownRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    function handleClickOutside(e: MouseEvent) {
      if (dropdownRef.current && !dropdownRef.current.contains(e.target as Node)) {
        setVersionOpen(false);
      }
    }
    if (versionOpen) document.addEventListener("mousedown", handleClickOutside);
    return () => document.removeEventListener("mousedown", handleClickOutside);
  }, [versionOpen]);

  return (
    <div className="flex items-center gap-1.5 rounded-full border border-slate-200/60 bg-white/90 p-1.5 shadow-xl backdrop-blur-md dark:border-slate-800/60 dark:bg-slate-900/90 animate-in slide-in-from-bottom-4 duration-300 relative z-[60]">
      
      {/* Version Selector */}
      <div className="relative" ref={dropdownRef}>
        <button
          type="button"
          onClick={() => setVersionOpen(!versionOpen)}
          className="flex items-center gap-1.5 rounded-full bg-slate-100 px-3 py-2 text-xs font-semibold text-slate-700 transition-all duration-300 hover:bg-slate-200 dark:bg-slate-800 dark:text-slate-300 dark:hover:bg-slate-700"
        >
          <Layers className="h-4 w-4 text-indigo-500" />
          <span className="w-[30px] text-center font-mono">{currentVersion}</span>
          <ChevronDown className={cn("h-3 w-3 text-slate-400 transition-transform", versionOpen && "rotate-180")} />
        </button>

        {versionOpen && (
          <div className="absolute bottom-full left-0 mb-3 w-64 overflow-hidden rounded-2xl border border-slate-200 bg-white shadow-xl dark:border-slate-800 dark:bg-slate-900 z-50 animate-in fade-in slide-in-from-bottom-2">
            {versionHistory.map((v) => (
              <button
                key={v.version}
                type="button"
                onClick={() => {
                  onVersionChange(v.version);
                  setVersionOpen(false);
                }}
                className={cn(
                  "flex w-full items-center justify-between px-4 py-2.5 text-xs transition-colors hover:bg-slate-50 dark:hover:bg-slate-800/60",
                  currentVersion === v.version
                    ? "bg-slate-50 font-semibold text-slate-900 dark:bg-slate-800/60 dark:text-white"
                    : "text-slate-600 dark:text-slate-400"
                )}
              >
                <div className="flex items-center gap-2">
                  <span className="font-mono">{v.version}</span>
                </div>
                <span className="text-[10px] font-normal text-slate-400">{v.updatedAt}</span>
              </button>
            ))}
          </div>
        )}
      </div>

      <div className="h-5 w-px bg-slate-200 dark:bg-slate-700 mx-1" />

      {/* Action Buttons */}
      <button
        type="button"
        onClick={onToggleFeedback}
        className={cn(
          "group flex h-9 items-center rounded-full px-2.5 transition-all duration-300",
          isFeedbackMode
            ? "bg-purple-100 text-purple-700 dark:bg-purple-900/50 dark:text-purple-300"
            : "text-slate-500 hover:bg-slate-100 hover:text-slate-900 dark:text-slate-400 dark:hover:bg-slate-800 dark:hover:text-slate-100"
        )}
        title={t('draft.toolbar.feedback')}
      >
        <MessageSquareQuote className="h-4 w-4 shrink-0 transition-transform duration-300 group-hover:scale-110" />
        <span className="max-w-0 overflow-hidden whitespace-nowrap opacity-0 transition-all duration-300 group-hover:ml-1.5 group-hover:max-w-[100px] group-hover:opacity-100">
          {t('draft.toolbar.feedback')}
        </span>
      </button>

      <button
        type="button"
        onClick={onRegenerate}
        className="flex h-9 w-9 items-center justify-center rounded-full text-slate-500 transition-all duration-300 hover:bg-slate-100 hover:text-slate-900 dark:text-slate-400 dark:hover:bg-slate-800 dark:hover:text-slate-100 group"
        title={t('draft.toolbar.regenerate')}
      >
        <RotateCcw className="h-4 w-4 transition-transform duration-500 group-hover:-rotate-180" />
      </button>

      {onNext && (
        <button
          type="button"
          onClick={onNext}
          className="flex items-center gap-1.5 rounded-full bg-slate-900 px-4 py-2 text-xs font-semibold text-white shadow-md transition-all duration-300 hover:bg-slate-800 hover:shadow-lg dark:bg-slate-100 dark:text-slate-900 dark:hover:bg-white group"
        >
          <span>{t('draft.toolbar.next')}</span>
          <ArrowRight className="h-4 w-4 transition-transform duration-300 group-hover:translate-x-1" />
        </button>
      )}

      {onStart && (
        <button
          type="button"
          onClick={onStart}
          className="flex items-center gap-1.5 rounded-full bg-blue-600 px-4 py-2 text-xs font-semibold text-white shadow-md transition-all duration-300 hover:bg-blue-500 hover:shadow-lg group"
        >
          <Check className="h-4 w-4 transition-transform duration-300 group-hover:scale-110" />
          <span>{t('draft.toolbar.confirm')}</span>
        </button>
      )}
    </div>
  );
}
