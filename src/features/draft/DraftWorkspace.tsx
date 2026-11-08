import { useState, useMemo, useEffect, useRef } from 'react';
import { MessageSquareQuote, AlertTriangle, RefreshCw } from 'lucide-react';
import { useNavigate } from 'react-router';
import { DraftHeader } from './DraftHeader';
import { DraftOverviewBlock } from './DraftOverviewBlock';
import { DraftToolbar } from './DraftToolbar';
import { DraftLoading } from './DraftLoading';
import { CommitConfirmationDialog } from './CommitConfirmationDialog';
import {
  buildCommittedProjectViewPath,
  createOnceGuard,
  type SampleProjectRef,
} from './commitNavigation';
import {
  isTaskApplicable,
  WORKFLOW_BASELINE_UNAVAILABLE,
  REQUIRES_REPLAN,
  EXPIRED_BY_RESTORE,
  type TaskReceipt,
} from './commitPreview';
import { DRAFT_OVERVIEW } from './mockOverview';
import { ConfirmDialog } from '@/shared/components/ui/confirm-dialog';
import { useT } from '@/shared/i18n/useT';
import { cn } from '@/shared/utils/cn';
import type { DraftStep, CharacterSubStep, DraftOverview, FeedbackContext, DraftVersion } from './types';
import { BackendClient } from '@/shared/backend/client';
import type {
  DraftVersionSummaryDto,
  PendingPatchDto,
  DraftWorkspaceStateDto,
  RestoreVersionResponseDto,
  WorkflowExecutionSummaryDto,
  WorkflowProgressEvent,
} from '@/shared/backend/generated';

// ============================================================================
// Draft Backend State & Pure Controller
// ============================================================================

export interface DraftBackendState {
  draftSessionId: string;
  headVersionSeq: number | null;
  currentVersionLabel: string;
  versions: DraftVersionSummaryDto[];
  pendingPatches: PendingPatchDto[];
  workflowStatus: "idle" | "running" | "requires_replan" | "completed" | "cancelled" | "failed";
  workflowRunId: string | null;
  manualRepairMessage: string | null;
  lastError: any;
}

export class DraftBackendController {
  private client: BackendClient;
  private state: DraftBackendState;
  private listeners: Set<(state: DraftBackendState) => void> = new Set();

  constructor(client?: BackendClient, initialConfig?: Partial<DraftBackendState>) {
    this.client = client ?? new BackendClient();
    this.state = {
      draftSessionId: initialConfig?.draftSessionId ?? `draft-${Date.now()}`,
      headVersionSeq: initialConfig?.headVersionSeq ?? null,
      currentVersionLabel: initialConfig?.currentVersionLabel ?? "v.01",
      versions: initialConfig?.versions ?? [],
      pendingPatches: initialConfig?.pendingPatches ?? [],
      workflowStatus: initialConfig?.workflowStatus ?? "idle",
      workflowRunId: initialConfig?.workflowRunId ?? null,
      manualRepairMessage: initialConfig?.manualRepairMessage ?? null,
      lastError: null,
    };
  }

  public getClient(): BackendClient {
    return this.client;
  }

  public getState(): DraftBackendState {
    return { ...this.state };
  }

  public subscribe(listener: (state: DraftBackendState) => void): () => void {
    this.listeners.add(listener);
    return () => {
      this.listeners.delete(listener);
    };
  }

  private notify() {
    const snapshot = this.getState();
    for (const listener of this.listeners) {
      listener(snapshot);
    }
  }

  public updateState(partial: Partial<DraftBackendState>) {
    this.state = { ...this.state, ...partial };
    this.notify();
  }

  public clearManualRepairMessage() {
    this.updateState({ manualRepairMessage: null });
  }

  /**
   * Loads authoritative durable state from backend.
   */
  public async loadWorkspaceState(): Promise<DraftWorkspaceStateDto | null> {
    try {
      const getFn = this.client.getDraftWorkspaceState
        ? this.client.getDraftWorkspaceState.bind(this.client)
        : null;
      if (!getFn) return null;

      const wsState = await getFn({ draftSessionId: this.state.draftSessionId });
      if (wsState) {
        this.updateState({
          headVersionSeq: wsState.headVersionSeq ?? this.state.headVersionSeq,
          pendingPatches: wsState.pendingPatches ?? this.state.pendingPatches,
        });
      }
      return wsState;
    } catch (err: any) {
      this.updateState({ lastError: err });
      return null;
    }
  }

  /**
   * Loads versions from backend.
   */
  public async loadVersions(): Promise<DraftVersionSummaryDto[]> {
    try {
      const listFn = this.client.listDraftVersions
        ? this.client.listDraftVersions.bind(this.client)
        : null;
      if (!listFn) return this.state.versions;

      const res = await listFn({ draftSessionId: this.state.draftSessionId });
      const versions = res?.versions || [];
      this.updateState({
        versions,
        headVersionSeq: res?.headVersionSeq ?? this.state.headVersionSeq,
      });
      return versions;
    } catch (err: any) {
      this.updateState({ lastError: err });
      return this.state.versions;
    }
  }

  /**
   * Restores a draft version.
   *
   * Invariants:
   * 1. Backend head is authoritative: updates head to the newly created restore version.
   * 2. Restore expires pending patches (EXPIRED_BY_RESTORE).
   * 3. WorkflowBaselineUnavailable (phase5_sprint3_deep2-F1): surfaces explicit manual-repair
   *    message (WORKFLOW_BASELINE_UNAVAILABLE), NEVER fabricates a synthetic version.
   */
  public async restoreDraftVersion(
    targetSeq: number,
    clientRequestId?: string
  ): Promise<{ ok: boolean; version?: DraftVersionSummaryDto; error?: string }> {
    try {
      const restoreFn = this.client.restoreDraftVersion
        ? this.client.restoreDraftVersion.bind(this.client)
        : null;
      if (!restoreFn) {
        throw new Error("restoreDraftVersion not supported on client");
      }

      const res: RestoreVersionResponseDto = await restoreFn({
        draftSessionId: this.state.draftSessionId,
        targetSeq,
        clientRequestId: clientRequestId ?? `restore-${Date.now()}`,
      });

      // Restoring a version creates a new restore version and expires pending patches
      const newVersion = res.newVersion;
      const expiredIds = new Set(res.expiredPatchIds || []);

      const updatedPatches = this.state.pendingPatches.map((p) => {
        if (expiredIds.has(p.id)) {
          return { ...p, status: EXPIRED_BY_RESTORE };
        }
        return p;
      }).filter((p) => !expiredIds.has(p.id)); // filter out expired from active pending

      const updatedVersions = [...this.state.versions];
      if (newVersion && !updatedVersions.some((v) => v.seq === newVersion.seq)) {
        updatedVersions.unshift(newVersion);
      }

      this.updateState({
        headVersionSeq: newVersion.seq,
        currentVersionLabel: newVersion.displayLabel || `v.${String(newVersion.seq).padStart(2, '0')}`,
        versions: updatedVersions,
        pendingPatches: updatedPatches,
        manualRepairMessage: null,
      });

      return { ok: true, version: newVersion };
    } catch (err: any) {
      const isBaselineUnavailable =
        err?.code === WORKFLOW_BASELINE_UNAVAILABLE ||
        err?.code === 'WorkflowBaselineUnavailable' ||
        err?.message?.includes('WorkflowBaselineUnavailable') ||
        err?.message?.includes(WORKFLOW_BASELINE_UNAVAILABLE) ||
        err?.name === 'WorkflowBaselineUnavailable';

      if (isBaselineUnavailable) {
        // Explicit manual-repair message; never fabricate a synthetic version!
        this.updateState({
          manualRepairMessage: `${WORKFLOW_BASELINE_UNAVAILABLE}: Sealed baseline is missing or unavailable. Manual repair required.`,
          lastError: err,
        });
        return { ok: false, error: WORKFLOW_BASELINE_UNAVAILABLE };
      }

      this.updateState({ lastError: err });
      return { ok: false, error: err?.message || 'Restore failed' };
    }
  }

  /**
   * Loads pending patches.
   */
  public async loadPendingPatches(): Promise<PendingPatchDto[]> {
    try {
      const listFn = this.client.listPendingPatches
        ? this.client.listPendingPatches.bind(this.client)
        : null;
      if (!listFn) return this.state.pendingPatches;

      const res = await listFn({ draftSessionId: this.state.draftSessionId });
      const patches = res?.items || [];
      this.updateState({ pendingPatches: patches });
      return patches;
    } catch (err: any) {
      this.updateState({ lastError: err });
      return this.state.pendingPatches;
    }
  }

  /**
   * Confirms a single pending patch.
   */
  public async confirmPendingPatch(patchId: string): Promise<any> {
    try {
      const confirmFn = this.client.confirmPendingPatch
        ? this.client.confirmPendingPatch.bind(this.client)
        : null;
      if (!confirmFn) return null;

      const res = await confirmFn({ draftSessionId: this.state.draftSessionId, patchId });
      const remaining = this.state.pendingPatches.filter((p) => p.id !== patchId);

      const partial: Partial<DraftBackendState> = { pendingPatches: remaining };
      if (res?.sealedVersion) {
        partial.headVersionSeq = res.sealedVersion.seq;
        partial.currentVersionLabel = res.sealedVersion.displayLabel;
        if (!this.state.versions.some((v) => v.seq === res.sealedVersion!.seq)) {
          partial.versions = [res.sealedVersion, ...this.state.versions];
        }
      }
      this.updateState(partial);
      return res;
    } catch (err: any) {
      this.updateState({ lastError: err });
      throw err;
    }
  }

  /**
   * Discards/rejects a single pending patch.
   */
  public async discardPendingPatch(patchId: string): Promise<any> {
    try {
      const rejectFn = this.client.rejectPendingPatch
        ? this.client.rejectPendingPatch.bind(this.client)
        : null;
      if (!rejectFn) return null;

      const res = await rejectFn({ draftSessionId: this.state.draftSessionId, patchId });
      const remaining = this.state.pendingPatches.filter((p) => p.id !== patchId);
      this.updateState({ pendingPatches: remaining });
      return res;
    } catch (err: any) {
      this.updateState({ lastError: err });
      throw err;
    }
  }

  /**
   * Confirms all pending patches in batch.
   */
  public async confirmAllPendingPatches(clientRequestId?: string): Promise<any> {
    try {
      const confirmAllFn = this.client.confirmAllPendingPatches
        ? this.client.confirmAllPendingPatches.bind(this.client)
        : null;
      if (!confirmAllFn) return null;

      const res = await confirmAllFn({
        draftSessionId: this.state.draftSessionId,
        clientRequestId: clientRequestId ?? `batch-${Date.now()}`,
      });

      const appliedIds = new Set(res?.applied || []);
      const remaining = this.state.pendingPatches.filter((p) => !appliedIds.has(p.id));

      const partial: Partial<DraftBackendState> = { pendingPatches: remaining };
      if (res?.sealedVersion) {
        partial.headVersionSeq = res.sealedVersion.seq;
        partial.currentVersionLabel = res.sealedVersion.displayLabel;
        if (!this.state.versions.some((v) => v.seq === res.sealedVersion!.seq)) {
          partial.versions = [res.sealedVersion, ...this.state.versions];
        }
      }
      this.updateState(partial);
      return res;
    } catch (err: any) {
      this.updateState({ lastError: err });
      throw err;
    }
  }

  /**
   * Discards all pending patches.
   */
  public async discardAllPendingPatches(): Promise<void> {
    const patches = [...this.state.pendingPatches];
    for (const patch of patches) {
      try {
        await this.discardPendingPatch(patch.id);
      } catch {
        // continue discarding
      }
    }
    this.updateState({ pendingPatches: [] });
  }

  /**
   * Starts or resumes segmented workflow run.
   *
   * Invariants:
   * 1. WorkflowBaselineUnavailable (phase5_sprint3_deep2-F1): surfaces manual-repair message,
   *    never fabricates a synthetic version.
   * 2. Indeterminate receipt (phase5_sprint3_deep2-F2): surfaces requires_replan UI, NEVER completed badge.
   */
  public async startOrResumeWorkflow(
    phase: number,
    subphase?: number | null,
    onProgress?: (event: WorkflowProgressEvent) => void
  ): Promise<WorkflowExecutionSummaryDto> {
    this.updateState({ workflowStatus: 'running' });
    try {
      const startFn = this.client.startOrResumeWorkflow
        ? this.client.startOrResumeWorkflow.bind(this.client)
        : null;
      if (!startFn) {
        throw new Error("startOrResumeWorkflow not supported on client");
      }

      const res: WorkflowExecutionSummaryDto = await startFn(
        {
          draftSessionId: this.state.draftSessionId,
          phase,
          subphase,
          systemLanguage: 'vi',
        },
        onProgress
      );

      this.applyWorkflowExecutionResult(res);
      return res;
    } catch (err: any) {
      const isBaselineUnavailable =
        err?.code === WORKFLOW_BASELINE_UNAVAILABLE ||
        err?.code === 'WorkflowBaselineUnavailable' ||
        err?.message?.includes('WorkflowBaselineUnavailable') ||
        err?.message?.includes(WORKFLOW_BASELINE_UNAVAILABLE) ||
        err?.name === 'WorkflowBaselineUnavailable';

      if (isBaselineUnavailable) {
        this.updateState({
          manualRepairMessage: `${WORKFLOW_BASELINE_UNAVAILABLE}: Sealed baseline is missing or unavailable. Manual repair required.`,
          workflowStatus: 'failed',
          lastError: err,
        });
      } else if (
        err?.status === REQUIRES_REPLAN ||
        err?.message?.includes(REQUIRES_REPLAN) ||
        err?.message?.includes('INDETERMINATE')
      ) {
        this.updateState({
          workflowStatus: 'requires_replan',
          lastError: err,
        });
      } else {
        this.updateState({
          workflowStatus: 'failed',
          lastError: err,
        });
      }
      throw err;
    }
  }

  /**
   * Applies workflow execution summary into controller state.
   */
  public applyWorkflowExecutionResult(res: WorkflowExecutionSummaryDto) {
    if (
      res.status === REQUIRES_REPLAN ||
      res.status === 'INDETERMINATE' ||
      res.recoveryReason === 'insufficient_task_evidence'
    ) {
      // Indeterminate receipt surfaces requires_replan UI, never completed badge!
      this.updateState({
        workflowRunId: res.workflowRunId,
        workflowStatus: 'requires_replan',
        pendingPatches: res.pendingPatches ?? this.state.pendingPatches,
      });
      return;
    }

    if (res.status === 'completed') {
      const partial: Partial<DraftBackendState> = {
        workflowRunId: res.workflowRunId,
        workflowStatus: 'completed',
        pendingPatches: res.pendingPatches ?? this.state.pendingPatches,
      };
      if (res.lastSealedVersion) {
        partial.headVersionSeq = res.lastSealedVersion.seq;
        partial.currentVersionLabel = res.lastSealedVersion.displayLabel;
        if (!this.state.versions.some((v) => v.seq === res.lastSealedVersion!.seq)) {
          partial.versions = [res.lastSealedVersion, ...this.state.versions];
        }
      }
      this.updateState(partial);
      return;
    }

    this.updateState({
      workflowRunId: res.workflowRunId,
      workflowStatus: (res.status as any) || 'idle',
      pendingPatches: res.pendingPatches ?? this.state.pendingPatches,
    });
  }

  /**
   * Cancels running workflow run.
   */
  public async cancelWorkflowRun(workflowRunId?: string): Promise<any> {
    try {
      const cancelFn = this.client.cancelWorkflow
        ? this.client.cancelWorkflow.bind(this.client)
        : null;
      if (!cancelFn) return null;

      const runId = workflowRunId || this.state.workflowRunId || 'active-run';
      const res = await cancelFn({
        draftSessionId: this.state.draftSessionId,
        workflowRunId: runId,
      });

      this.updateState({ workflowStatus: 'cancelled' });
      return res;
    } catch (err: any) {
      this.updateState({ lastError: err });
      throw err;
    }
  }

  /**
   * Confirms workflow stage replan.
   */
  public confirmWorkflowStageReplan() {
    this.updateState({ workflowStatus: 'idle' });
  }

  /**
   * Enforces domain invariant: Historical `Verified` receipt alone never marks
   * current task applicable without current head and dependency verification.
   */
  public verifyTaskApplicability(
    receipt: TaskReceipt | null | undefined,
    expectedDependencyFingerprint?: string | null
  ): boolean {
    return isTaskApplicable(
      receipt,
      this.state.headVersionSeq ?? 0,
      expectedDependencyFingerprint
    );
  }
}

// ============================================================================
// DraftWorkspace Component
// ============================================================================

export interface DraftWorkspaceProps {
  onClose: () => void;
  onFeedback: (feedback: FeedbackContext) => void;
  overview?: DraftOverview;
  sampleProjects?: SampleProjectRef[];
  onViewSampleProject?: (projectId: string) => void;
  client?: BackendClient;
  draftSessionId?: string;
  onCommitSuccess?: (projectId: string, path: string) => void;
  onNavigate?: (path: string) => void;
}

export function DraftWorkspace({
  onClose,
  onFeedback,
  overview = DRAFT_OVERVIEW,
  sampleProjects,
  onViewSampleProject,
  client,
  draftSessionId,
  onCommitSuccess,
  onNavigate,
}: DraftWorkspaceProps) {
  const t = useT();
  let navigate: ((path: string) => void) | undefined;
  try {
    navigate = useNavigate();
  } catch {
    // router context unavailable in isolated test environments
  }

  // Pure controller instance
  const controllerRef = useRef<DraftBackendController | null>(null);
  if (!controllerRef.current) {
    controllerRef.current = new DraftBackendController(client, {
      draftSessionId: draftSessionId ?? `draft-${Date.now()}`,
      currentVersionLabel: overview.version || "v.01",
    });
  }
  const controller = controllerRef.current;
  const [controllerState, setControllerState] = useState<DraftBackendState>(() =>
    controller.getState()
  );

  useEffect(() => {
    return controller.subscribe((nextState) => {
      setControllerState(nextState);
    });
  }, [controller]);

  // Load versions and pending patches on mount
  useEffect(() => {
    controller.loadVersions();
    controller.loadPendingPatches();
  }, [controller]);

  const [currentStep, setCurrentStep] = useState<DraftStep>('overview');
  const [currentSubStep, setCurrentSubStep] = useState<CharacterSubStep>('appearance');
  const [currentVersion, setCurrentVersion] = useState(
    controllerState.currentVersionLabel || overview.version
  );
  const [confirmOpen, setConfirmOpen] = useState(false);
  const [regenerateConfirmOpen, setRegenerateConfirmOpen] = useState(false);
  const [commitDialogOpen, setCommitDialogOpen] = useState(false);

  // Synchronize version label from controller
  useEffect(() => {
    if (controllerState.currentVersionLabel) {
      setCurrentVersion(controllerState.currentVersionLabel);
    }
  }, [controllerState.currentVersionLabel]);

  // Feedback Mode State & Custom Cursor
  const [isFeedbackMode, setIsFeedbackMode] = useState(false);
  const [cursorPos, setCursorPos] = useState({ x: 0, y: 0 });
  const containerRef = useRef<HTMLElement>(null);

  useEffect(() => {
    if (!isFeedbackMode) return;
    const handleMouseMove = (e: MouseEvent) => {
      if (containerRef.current) {
        const rect = containerRef.current.getBoundingClientRect();
        setCursorPos({ x: e.clientX - rect.left, y: e.clientY - rect.top });
      }
    };
    window.addEventListener('mousemove', handleMouseMove);
    return () => window.removeEventListener('mousemove', handleMouseMove);
  }, [isFeedbackMode]);

  // Transition states — loading 3s on initial overview
  const [transitionPhase, setTransitionPhase] = useState<'idle' | 'fade-out' | 'loading'>('loading');
  const [loadingIdx, setLoadingIdx] = useState(0);

  const loadingMessages = [
    t('draft.loading.msg1'),
    t('draft.loading.msg2'),
    t('draft.loading.msg3'),
    t('draft.loading.msg4'),
  ];

  useEffect(() => {
    if (transitionPhase !== 'loading') return;
    const interval = setInterval(() => {
      setLoadingIdx((prev) => (prev + 1) % loadingMessages.length);
    }, 1500);
    return () => clearInterval(interval);
  }, [transitionPhase, loadingMessages.length]);

  useEffect(() => {
    const timer = setTimeout(() => {
      setTransitionPhase('idle');
    }, 3000);
    return () => clearTimeout(timer);
  }, []);

  const triggerTransition = (action: () => void) => {
    if (transitionPhase !== 'idle') return;
    setTransitionPhase('fade-out');

    setTimeout(() => {
      action();
      setLoadingIdx(0);
      setTransitionPhase('loading');

      setTimeout(() => {
        setTransitionPhase('idle');
      }, 3000);
    }, 400);
  };

  const executeNext = () => {
    setConfirmOpen(false);
    triggerTransition(() => {
      if (currentStep === 'overview') setCurrentStep('chapter');
      else if (currentStep === 'chapter') setCurrentStep('world');
      else if (currentStep === 'world') setCurrentStep('scene');
      else if (currentStep === 'scene') {
        setCurrentStep('character');
        setCurrentSubStep('appearance');
      } else if (currentStep === 'character') {
        if (currentSubStep === 'appearance') setCurrentSubStep('psychological');
        else if (currentSubStep === 'psychological') setCurrentSubStep('lifeExperience');
        else if (currentSubStep === 'lifeExperience') setCurrentStep('relationship');
      }
    });
  };

  const handleStart = () => {
    setCommitDialogOpen(true);
  };

  const handleRegenerate = () => {
    setRegenerateConfirmOpen(true);
  };

  const executeRegenerate = () => {
    triggerTransition(() => {
      // Regenerate logic
    });
  };

  const handleAttachFeedback = (id: string, tag: string, title: string) => {
    onFeedback({ id, tag, title });
    setIsFeedbackMode(false);
  };

  // Merge backend versions with mock overview versions
  const mergedVersionHistory: DraftVersion[] = useMemo(() => {
    if (controllerState.versions.length > 0) {
      return controllerState.versions.map((v) => ({
        version: v.displayLabel || `v.${String(v.seq).padStart(2, '0')}`,
        updatedAt: v.createdAt ? new Date(v.createdAt).toLocaleDateString() : 'Vừa xong',
        seq: v.seq,
        reason: v.reason,
      } as any));
    }
    return overview.versionHistory || [];
  }, [controllerState.versions, overview.versionHistory]);

  const phaseOverview = useMemo(() => {
    const isStepOverview = currentStep === 'overview';
    const isStepRelationship = currentStep === 'relationship';

    return {
      ...overview,
      arcs: isStepOverview
        ? overview.arcs.map((arc) => ({
            ...arc,
            chapters: undefined,
          }))
        : overview.arcs,
      worldSettings: ['world', 'scene', 'character', 'relationship'].includes(currentStep)
        ? overview.worldSettings
        : undefined,
      sceneConfig: ['scene', 'character', 'relationship'].includes(currentStep)
        ? overview.sceneConfig
        : undefined,
      characters: overview.characters.map((char) => {
        if (['overview', 'chapter', 'world', 'scene'].includes(currentStep)) {
          return {
            ...char,
            appearance: undefined as any,
            psychological: undefined as any,
            competences: [],
            memories: [],
          };
        }
        if (currentStep === 'character') {
          if (currentSubStep === 'appearance') {
            return {
              ...char,
              psychological: undefined as any,
              competences: [],
              memories: [],
            };
          }
          if (currentSubStep === 'psychological') {
            return {
              ...char,
              competences: [],
              memories: [],
            };
          }
        }
        return char;
      }),
      relationships: isStepRelationship ? overview.relationships : undefined,
    };
  }, [overview, currentStep, currentSubStep]);

  const isFading = transitionPhase === 'fade-out' || transitionPhase === 'loading';

  // Commit success handler: navigates once to real project in View mode (P5S3-P)
  const commitNavGuard = useRef(createOnceGuard());
  const handleCommitSuccess = (projectId: string, path: string) => {
    onClose();
    commitNavGuard.current(() => {
      onCommitSuccess?.(projectId, path);
      if (onNavigate) {
        onNavigate(path);
      } else if (navigate) {
        navigate(path);
      }
    });
  };

  return (
    <aside
      ref={containerRef}
      onMouseLeave={() => setIsFeedbackMode(false)}
      className={cn(
        "flex h-full w-full flex-col overflow-hidden border-l border-line bg-surface-muted/40 relative",
        isFeedbackMode && "cursor-none [&_*]:cursor-none"
      )}
    >
      <DraftHeader
        currentStep={currentStep}
        currentSubStep={currentSubStep}
        onClose={onClose}
      />

      {/* Manual Repair Banner (phase5_sprint3_deep2-F1) */}
      {controllerState.manualRepairMessage && (
        <div
          role="alert"
          data-testid="workspace-manual-repair-alert"
          className="mx-4 mt-3 flex items-center justify-between rounded-xl bg-rose-100 px-4 py-2 text-sm font-medium text-rose-700 dark:bg-rose-950/50 dark:text-rose-300"
        >
          <div className="flex items-center gap-2">
            <AlertTriangle className="h-4 w-4 shrink-0 text-rose-600" />
            <span>{controllerState.manualRepairMessage}</span>
          </div>
          <button
            type="button"
            onClick={() => controller.clearManualRepairMessage()}
            className="text-xs font-semibold hover:underline"
          >
            {t('common.close')}
          </button>
        </div>
      )}

      {/* Workflow Requires Replan Banner (phase5_sprint3_deep2-F2) */}
      {controllerState.workflowStatus === 'requires_replan' && (
        <div
          role="status"
          data-testid="workspace-requires-replan-banner"
          className="mx-4 mt-3 flex items-center justify-between rounded-xl bg-amber-100 px-4 py-2 text-sm font-medium text-amber-700 dark:bg-amber-950/50 dark:text-amber-300"
        >
          <div className="flex items-center gap-2">
            <AlertTriangle className="h-4 w-4 shrink-0 text-amber-600" />
            <span>REQUIRES_REPLAN: Trạng thái quy trình không xác định, cần lập kế hoạch lại.</span>
          </div>
          <button
            type="button"
            onClick={() => controller.confirmWorkflowStageReplan()}
            className="rounded-lg bg-amber-700 px-3 py-1 text-xs font-semibold text-white hover:bg-amber-800"
          >
            Lập kế hoạch lại
          </button>
        </div>
      )}

      <div className="relative flex-1 overflow-hidden">
        {transitionPhase === 'loading' && (
          <DraftLoading message={loadingMessages[loadingIdx]} />
        )}

        <div
          className={cn(
            "h-full w-full transition-all duration-300",
            transitionPhase === 'fade-out' && "opacity-0 scale-[0.98]",
            transitionPhase === 'loading' && "hidden",
            transitionPhase === 'idle' && "opacity-100 scale-100"
          )}
        >
          <DraftOverviewBlock
            overview={phaseOverview}
            currentStep={currentStep}
            currentSubStep={currentSubStep}
            isFeedbackMode={isFeedbackMode}
            onAttachFeedback={handleAttachFeedback}
          />
        </div>

        {/* Toolbar */}
        <div
          className={cn(
            "absolute bottom-6 left-1/2 -translate-x-1/2 z-20 transition-colors duration-300",
            isFading ? "opacity-0 translate-y-4 pointer-events-none" : "opacity-100 translate-y-0"
          )}
        >
          <DraftToolbar
            currentVersion={currentVersion}
            headVersionSeq={controllerState.headVersionSeq}
            versionHistory={mergedVersionHistory}
            onVersionChange={setCurrentVersion}
            onRestoreVersion={(seq) => controller.restoreDraftVersion(seq)}
            onRegenerate={handleRegenerate}
            onNext={currentStep !== 'relationship' ? () => setConfirmOpen(true) : undefined}
            onStart={currentStep === 'relationship' ? handleStart : undefined}
            isFeedbackMode={isFeedbackMode}
            onToggleFeedback={() => setIsFeedbackMode(!isFeedbackMode)}
            workflowStatus={controllerState.workflowStatus}
            onCancelWorkflow={() => controller.cancelWorkflowRun()}
            onReplanWorkflow={() => controller.confirmWorkflowStageReplan()}
            pendingPatchesCount={controllerState.pendingPatches.length}
            onConfirmAllPatches={() => controller.confirmAllPendingPatches()}
            onDiscardAllPatches={() => controller.discardAllPendingPatches()}
            manualRepairMessage={controllerState.manualRepairMessage}
          />
        </div>
      </div>

      <ConfirmDialog
        open={confirmOpen}
        onOpenChange={setConfirmOpen}
        title={t('draft.dialog.nextTitle')}
        description={t('draft.dialog.nextDesc')}
        confirmLabel={t('draft.dialog.confirmNext')}
        cancelLabel={t('common.cancel')}
        cancelEmphasized
        onConfirm={executeNext}
      />

      <CommitConfirmationDialog
        open={commitDialogOpen}
        onOpenChange={setCommitDialogOpen}
        overview={overview}
        sampleProjects={sampleProjects}
        onViewSampleProject={onViewSampleProject}
        client={controller.getClient()}
        draftSessionId={controllerState.draftSessionId}
        onCommitSuccess={handleCommitSuccess}
      />

      <ConfirmDialog
        open={regenerateConfirmOpen}
        onOpenChange={setRegenerateConfirmOpen}
        title={t('draft.dialog.regenerateTitle')}
        description={t('draft.dialog.regenerateDesc')}
        confirmLabel={t('draft.dialog.confirmRegenerate')}
        cancelLabel={t('common.cancel')}
        cancelEmphasized
        onConfirm={executeRegenerate}
      />

      {/* Custom Feedback Cursor */}
      {isFeedbackMode && (
        <div
          className="pointer-events-none absolute z-[9999] flex h-8 w-8 -translate-x-1/2 -translate-y-1/2 items-center justify-center rounded-full border-2 border-purple-500 bg-purple-500/20 backdrop-blur-sm transition-transform duration-150 ease-out"
          style={{ left: cursorPos.x, top: cursorPos.y }}
        >
          <MessageSquareQuote className="h-4 w-4 text-purple-600 dark:text-purple-400" />
        </div>
      )}
    </aside>
  );
}

