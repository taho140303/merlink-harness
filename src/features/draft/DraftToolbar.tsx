import { useState, useRef, useEffect } from 'react';
import { Layers, ChevronDown, RotateCcw, ArrowRight, Check, MessageSquareQuote, CheckCircle2 } from 'lucide-react';
import { cn } from '@/shared/utils/cn';
import { useT } from '@/shared/i18n/useT';
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
  const [hoveredItem, setHoveredItem] = useState<string | null>(null);
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

  const actions = [
    { id: 'feedback', label: t('draft.toolbar.feedback'), icon: MessageSquareQuote, onClick: onToggleFeedback, active: isFeedbackMode },
    { id: 'regenerate', label: t('draft.toolbar.regenerate'), icon: RotateCcw, onClick: onRegenerate, active: false },
    { id: 'next', label: t('draft.toolbar.next'), icon: ArrowRight, onClick: onNext, active: false },
    { id: 'confirm', label: t('draft.toolbar.confirm'), icon: Check, onClick: onStart, active: false },
  ].filter(a => a.onClick !== undefined);

  return (
    <div className="flex items-center gap-2.5 rounded-full border border-line bg-surface-card/90 p-2  backdrop-blur-md    -from-bottom-4 duration-300 relative z-[60]">
      
      {/* Version Selector */}
      <div className="relative" ref={dropdownRef}>
        <button
          type="button"
          onClick={() => setVersionOpen(!versionOpen)}
          aria-expanded={versionOpen}
          className="focus-ring flex items-center gap-1.5 rounded-full bg-surface-muted px-3 py-2 text-sm font-semibold text-fg-muted transition-colors duration-200 hover:bg-surface-muted active:scale-95   "
        >
          <Layers className="h-3.5 w-3.5 text-fg-muted" />
          <span className="w-[30px] text-center font-mono">{currentVersion}</span>
          <ChevronDown className={cn("h-3.5 w-3.5 text-fg-subtle transition-transform duration-200", versionOpen && "rotate-180")} />
        </button>

        {versionOpen && (
          <div className="absolute bottom-full left-0 mb-3 w-56 overflow-hidden rounded-2xl border border-line bg-surface-card    z-50   -from-bottom-2">
            <div className="px-3 py-2 text-xs font-bold uppercase tracking-wider text-fg-subtle">
              {t('draft.toolbar.selectVersion')}
            </div>
            {versionHistory.map((v) => (
              <button
                key={v.version}
                type="button"
                onClick={() => {
                  onVersionChange(v.version);
                  setVersionOpen(false);
                }}
                className={cn(
                  "flex w-full items-center justify-between px-3.5 py-2.5 text-sm font-medium transition-colors hover:bg-surface-muted ",
                  currentVersion === v.version
                    ? "bg-surface-muted font-semibold text-fg  "
                    : "text-fg-muted -subtle"
                )}
              >
                <div className="flex items-center gap-5">
                  <span className="font-mono">{v.version}</span>
                </div>
                <div className="flex items-center gap-5">
                  <span className="text-xs font-normal text-fg-subtle">{v.updatedAt}</span>
                  {currentVersion === v.version && <CheckCircle2 className="h-3.5 w-3.5 text-fg " />}
                </div>
              </button>
            ))}
          </div>
        )}
      </div>

      <div className="h-6 w-[1.5px] bg-surface-muted  mx-2 rounded-full" />

      {/* Dynamic Expandable Actions */}
      <div className="flex items-center gap-5">
        {actions.map((action) => {
          const Icon = action.icon;
          const isHovered = hoveredItem === action.id;
          // Keep the feedback action expanded while feedback mode is active.
          const isExpanded = isHovered || (action.id === 'feedback' && action.active);

          return (
            <button
              key={action.id}
              type="button"
              onMouseEnter={() => setHoveredItem(action.id)}
              onMouseLeave={() => setHoveredItem(null)}
              onClick={action.onClick}
              className={cn(
                "focus-ring group relative flex h-9 sm:h-10 items-center justify-center rounded-full transition-colors duration-500 ease-[cubic-bezier(0.25,1,0.5,1)]",
                isExpanded
                  ? "bg-accent text-white px-4   "
                  : "bg-transparent text-fg-muted hover:text-fg w-9 sm:w-10 px-0 -subtle ",
                (action.id === 'next' || action.id === 'confirm') && !isExpanded && "bg-surface-muted "
              )}
              aria-label={action.label}
              aria-pressed={action.id === 'feedback' ? action.active : undefined}
            >
              <Icon
                className={cn(
                  "shrink-0 transition-transform duration-300",
                  isExpanded ? "w-5 h-5" : "w-5 h-5 group-hover:scale-110",
                  action.active && !isExpanded && "text-purple-600 dark:text-purple-400"
                )}
              />

              <div
                className={cn(
                  "grid transition-colors duration-500 ease-[cubic-bezier(0.25,1,0.5,1)] overflow-hidden",
                  isExpanded ? "grid-rows-[1fr] opacity-100 ml-2 max-w-[120px]" : "grid-rows-[0fr] opacity-0 ml-0 max-w-0"
                )}
              >
                <span className="overflow-hidden whitespace-nowrap text-sm font-semibold tracking-wide">
                  {action.label}
                </span>
              </div>

              {action.active && !isExpanded && (
                <span className="absolute bottom-1 w-1 h-1 rounded-full bg-purple-500" />
              )}
            </button>
          );
        })}
      </div>
    </div>
  );
}
