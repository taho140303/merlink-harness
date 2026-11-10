import { useState, useRef, useEffect } from 'react';
import { Layers, ChevronDown, RotateCcw, ArrowRight, Check, MessageSquareQuote, CheckCircle2, AlertTriangle, XCircle, RefreshCw } from 'lucide-react';
import { cn } from '@/shared/utils/cn';
import { useT } from '@/shared/i18n/useT';
import { useDbEnumLabel } from '@/shared/i18n/dbEnums';
import type { DraftVersion } from './types';

export interface DraftToolbarProps {
  currentVersion: string;
  versionHistory: DraftVersion[];
  onVersionChange: (v: string) => void;
  onRegenerate: () => void;
  onRunPhase?: () => void;
  runProgress?: string | null;
  workflowError?: string | null;
  onNext?: () => void;
  onStart?: () => void;
  isFeedbackMode: boolean;
  onToggleFeedback: () => void;
  // Phase 5 Sprint 3 real backend wiring props
  headVersionSeq?: number | null;
  onRestoreVersion?: (seq: number) => void;
  workflowStatus?: "idle" | "running" | "requires_replan" | "completed" | "cancelled" | "failed";
  onCancelWorkflow?: () => void;
  onReplanWorkflow?: () => void;
  pendingPatchesCount?: number;
  onConfirmAllPatches?: () => void;
  onDiscardAllPatches?: () => void;
  manualRepairMessage?: string | null;
}

export function DraftToolbar({
  currentVersion,
  versionHistory,
  onVersionChange,
  onRegenerate,
  onRunPhase,
  runProgress,
  workflowError,
  onNext,
  onStart,
  isFeedbackMode,
  onToggleFeedback,
  headVersionSeq,
  onRestoreVersion,
  workflowStatus = 'idle',
  onCancelWorkflow,
  onReplanWorkflow,
  pendingPatchesCount = 0,
  onConfirmAllPatches,
  onDiscardAllPatches,
  manualRepairMessage,
}: DraftToolbarProps) {
  const t = useT();
  const enumLabel = useDbEnumLabel();
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
    <div className="flex items-center gap-2.5 rounded-full border border-line bg-surface-card/90 p-2 backdrop-blur-md -from-bottom-4 duration-300 relative z-[60]">
      
      {/* Version Selector */}
      <div className="relative" ref={dropdownRef}>
        <button
          type="button"
          onClick={() => setVersionOpen(!versionOpen)}
          aria-expanded={versionOpen}
          className="focus-ring flex items-center gap-1.5 rounded-full bg-surface-muted px-3 py-2 text-sm font-semibold text-fg-muted transition-colors duration-200 hover:bg-surface-muted active:scale-95"
        >
          <Layers className="h-3.5 w-3.5 text-fg-muted" />
          <span className="w-[30px] text-center font-mono">{currentVersion}</span>
          <ChevronDown className={cn("h-3.5 w-3.5 text-fg-subtle transition-transform duration-200", versionOpen && "rotate-180")} />
        </button>

        {versionOpen && (
          <div className="absolute bottom-full left-0 mb-3 flex max-h-80 w-64 flex-col overflow-hidden rounded-2xl border border-line bg-surface-card z-50 -from-bottom-2 shadow-lg">
            <div className="px-3 py-2 text-xs font-bold uppercase tracking-wider text-fg-subtle">
              {t('draft.toolbar.selectVersion')}
            </div>
            <div className="min-h-0 flex-1 overflow-y-auto">
            {versionHistory.map((v) => {
              const seq = (v as any).seq;
              const isSelected = currentVersion === v.version || (headVersionSeq !== undefined && headVersionSeq !== null && seq === headVersionSeq);
              return (
                <div
                  key={v.version}
                  className={cn(
                    "flex w-full items-center gap-2 px-3.5 py-2.5 text-sm font-medium transition-colors hover:bg-surface-muted",
                    isSelected ? "bg-surface-muted font-semibold text-fg" : "text-fg-muted"
                  )}
                >
                  <button
                    type="button"
                    onClick={() => {
                      // Selecting only previews; restoring is an explicit action so opening the
                      // list no longer creates a restore version on every click.
                      onVersionChange(v.version);
                      setVersionOpen(false);
                    }}
                    className="focus-ring flex min-w-0 flex-1 flex-col text-left"
                  >
                    <span className="font-mono">{v.version}</span>
                    {(v as any).reason && (
                      <span className="text-[10px] text-fg-subtle line-clamp-1">{enumLabel('DraftVersionReason', (v as any).reason)}</span>
                    )}
                  </button>
                  <div className="flex shrink-0 items-center gap-1.5">
                    <span className="text-[10px] font-normal text-fg-subtle">{v.updatedAt}</span>
                    {isSelected && <CheckCircle2 className="h-3.5 w-3.5 text-accent" />}
                    {onRestoreVersion && typeof seq === 'number' && !isSelected && (
                      <button
                        type="button"
                        title={t('draft.toolbar.restoreVersion')}
                        aria-label={`${t('draft.toolbar.restoreVersion')} ${v.version}`}
                        onClick={() => {
                          onRestoreVersion(seq);
                          onVersionChange(v.version);
                          setVersionOpen(false);
                        }}
                        className="focus-ring rounded-md p-1 text-fg-subtle hover:bg-surface-card hover:text-fg"
                      >
                        <RotateCcw className="h-3.5 w-3.5" />
                      </button>
                    )}
                  </div>
                </div>
              );
            })}
            </div>
          </div>
        )}
      </div>

      {/* Manual Repair Banner / Notice (phase5_sprint3_deep2-F1) */}
      {manualRepairMessage && (
        <div
          role="alert"
          data-testid="manual-repair-notice"
          className="flex items-center gap-1.5 rounded-full bg-rose-500/20 px-3 py-1 text-xs font-medium text-rose-700 dark:text-rose-300"
        >
          <AlertTriangle className="h-3.5 w-3.5 shrink-0" />
          <span className="max-w-[160px] truncate">{manualRepairMessage}</span>
        </div>
      )}

      {/* Workflow Status Indicators (phase5_sprint3_deep2-F2) */}
      {workflowStatus === 'failed' && workflowError && (
        <div role="alert" className="mt-1 max-w-[420px] truncate text-[11px] text-rose-500" title={workflowError}>
          {workflowError}
        </div>
      )}

      {workflowStatus === 'requires_replan' && (
        <div
          role="status"
          data-testid="requires-replan-badge"
          className="flex items-center gap-1.5 rounded-full bg-amber-500/20 px-3 py-1 text-xs font-semibold text-amber-700 dark:text-amber-300"
        >
          <AlertTriangle className="h-3.5 w-3.5 shrink-0" />
          <span>REQUIRES_REPLAN</span>
          {onReplanWorkflow && (
            <button
              type="button"
              onClick={onReplanWorkflow}
              className="ml-1 rounded-full bg-amber-600 px-2 py-0.5 text-[11px] font-semibold text-white hover:bg-amber-700"
            >
              Replan
            </button>
          )}
        </div>
      )}

      {workflowStatus === 'running' && (
        <div
          role="status"
          data-testid="workflow-running-badge"
          className="flex items-center gap-1.5 rounded-full bg-blue-500/20 px-3 py-1 text-xs font-medium text-blue-700 dark:text-blue-300"
        >
          <RefreshCw className="h-3.5 w-3.5 animate-spin shrink-0" />
          <span>Workflow running...</span>
          {onCancelWorkflow && (
            <button
              type="button"
              onClick={onCancelWorkflow}
              className="ml-1 rounded-full bg-rose-600 px-2 py-0.5 text-[11px] font-semibold text-white hover:bg-rose-700"
            >
              Cancel
            </button>
          )}
        </div>
      )}

      {/* Completed badge: ONLY shown when workflowStatus is 'completed', NEVER on requires_replan */}
      {workflowStatus === 'completed' && (
        <div
          role="status"
          data-testid="workflow-completed-badge"
          className="flex items-center gap-1.5 rounded-full bg-emerald-500/20 px-3 py-1 text-xs font-semibold text-emerald-700 dark:text-emerald-300"
        >
          <CheckCircle2 className="h-3.5 w-3.5 shrink-0" />
          <span>Completed</span>
        </div>
      )}

      {/* Pending Patches Count Badge */}
      {pendingPatchesCount > 0 && (
        <div
          role="status"
          data-testid="pending-patches-badge"
          className="flex items-center gap-1.5 rounded-full bg-purple-500/20 px-2.5 py-1 text-xs font-semibold text-purple-700 dark:text-purple-300"
        >
          <span>{pendingPatchesCount} pending</span>
          {onConfirmAllPatches && (
            <button
              type="button"
              onClick={onConfirmAllPatches}
              title="Confirm all pending patches"
              className="rounded bg-purple-600 px-1.5 py-0.5 text-[10px] text-white hover:bg-purple-700"
            >
              Confirm All
            </button>
          )}
          {onDiscardAllPatches && (
            <button
              type="button"
              onClick={onDiscardAllPatches}
              title="Discard all pending patches"
              className="rounded bg-surface-muted px-1.5 py-0.5 text-[10px] text-fg-muted hover:text-fg"
            >
              Discard
            </button>
          )}
        </div>
      )}

      <div className="h-6 w-[1.5px] bg-surface-muted mx-2 rounded-full" />

      {/* Dynamic Expandable Actions */}
      <div className="flex items-center gap-5">
        {actions.map((action) => {
          const Icon = action.icon;
          const isHovered = hoveredItem === action.id;
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
                  ? "bg-accent text-white px-4"
                  : "bg-transparent text-fg-muted hover:text-fg w-9 sm:w-10 px-0 -subtle",
                (action.id === 'next' || action.id === 'confirm') && !isExpanded && "bg-surface-muted"
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

