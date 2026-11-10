import { useState, useMemo, useEffect, useRef, useDeferredValue } from 'react';
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
  businessFlowData?: any;
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
      businessFlowData: initialConfig?.businessFlowData ?? null,
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
          businessFlowData: wsState.businessFlowData ?? this.state.businessFlowData,
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
// BusinessFlowData -> DraftOverview Mapper
// ============================================================================

function unwrapTracked<T = any>(val: any, fallback: T): T {
  if (val && typeof val === "object" && !Array.isArray(val) && "value" in val) {
    return val.value !== null && val.value !== undefined ? val.value : fallback;
  }
  return val !== null && val !== undefined ? val : fallback;
}

export function mapBusinessFlowDataToDraftOverview(
  bfd: any,
  baseOverview: DraftOverview = DRAFT_OVERVIEW
): DraftOverview {
  if (!bfd || typeof bfd !== "object" || !bfd.require) {
    return baseOverview;
  }

  const reqProj = bfd.require?.project ?? {};
  const displayName = String(unwrapTracked(reqProj.display_name, "") || "").trim();
  const summary = String(unwrapTracked(reqProj.summary, "") || "").trim();
  const language = String(unwrapTracked(reqProj.language, "vi") || "vi").trim();
  const arcCount = Number(unwrapTracked(reqProj.target?.arc_count, 0)) || 0;
  const chapterCount = Number(unwrapTracked(reqProj.target?.chapter_count, 0)) || 0;
  const wordPerChapter = Number(unwrapTracked(reqProj.target?.word_per_chapter_count, 0)) || 0;

  const rawGenres = Array.isArray(reqProj.genres) ? reqProj.genres : [];
  const genres = rawGenres
    .map((g: any) => ({
      name: String(
        unwrapTracked(g?.display_name, "") || unwrapTracked(g?.code, "") || ""
      ).trim(),
      isPrimary: Boolean(unwrapTracked(g?.is_primary, false)),
    }))
    .filter((g: any) => g.name.length > 0);

  const reqChars = Array.isArray(bfd.require?.character) ? bfd.require.character : [];
  const appChars = Array.isArray(bfd.optional?.character_appearance_phase?.character)
    ? bfd.optional.character_appearance_phase.character
    : [];
  const psychChars = Array.isArray(
    bfd.optional?.character_psychological_phase?.character ??
      bfd.optional?.charactar_psychological_phase?.character
  )
    ? bfd.optional?.character_psychological_phase?.character ??
      bfd.optional?.charactar_psychological_phase?.character
    : [];
  const lifeChars = Array.isArray(bfd.optional?.character_life_experience_phase?.character)
    ? bfd.optional.character_life_experience_phase.character
    : [];

  // If bfd is completely empty (no display_name, no characters, no arcs), fallback to baseOverview
  const optArcs = Array.isArray(bfd.optional?.overview_phase?.arcs)
    ? bfd.optional.overview_phase.arcs
    : [];
  if (!displayName && reqChars.length === 0 && optArcs.length === 0 && genres.length === 0) {
    return baseOverview;
  }

  const charMap = new Map<string, string>();
  const characters = reqChars.map((rc: any, idx: number) => {
    const code = String(rc?.code || `CHAR_${idx + 1}`);
    const name =
      String(unwrapTracked(rc?.display_name, "") || unwrapTracked(rc?.nickname, "") || `Nhân vật ${idx + 1}`).trim();
    charMap.set(code, name);
    const nickname = String(unwrapTracked(rc?.nickname, "") || "").trim() || null;
    const sex = String(unwrapTracked(rc?.biological_sex, "NONE") || "NONE").toUpperCase();
    const genderBadge = sex === "MALE" ? "MALE" : sex === "FEMALE" ? "FEMALE" : "NONE";
    const motifDesc =
      String(
        unwrapTracked(rc?.notes, "") ||
          unwrapTracked(rc?.role, "") ||
          unwrapTracked(rc?.motif_code, "") ||
          ""
      ).trim() || "Nhân vật cốt lõi";

    const app = appChars.find((a: any) => a?.code === code) ?? {};
    const psych = psychChars.find((p: any) => p?.code === code) ?? {};
    const life = lifeChars.find((l: any) => l?.code === code) ?? {};

    const rawMarks = Array.isArray(app?.distinguishing_marks) ? app.distinguishing_marks : [];
    const distinguishingMarks = rawMarks
      .map((m: any) => {
        if (typeof m === "string") return m;
        const desc = String(unwrapTracked(m?.description, "") || "").trim();
        const loc = String(unwrapTracked(m?.location, "") || "").trim();
        return loc && desc ? `${loc}: ${desc}` : desc || loc;
      })
      .filter(Boolean);

    const rawCompetences = Array.isArray(life?.competences) ? life.competences : [];
    const competences = rawCompetences.map((comp: any, cIdx: number) => ({
      id: String(comp?.code || `comp-${idx}-${cIdx}`),
      name: String(unwrapTracked(comp?.display_name, "") || `Kỹ năng ${cIdx + 1}`),
      domain: String(unwrapTracked(comp?.domain, "") || "Chung"),
      description: String(unwrapTracked(comp?.description, "") || ""),
      sceneLearned: String(unwrapTracked(comp?.acquired_at_scene_code, "") || ""),
      capabilities: Array.isArray(comp?.capabilities)
        ? comp.capabilities.map((cap: any) => String(unwrapTracked(cap, ""))).filter(Boolean)
        : [],
      limitations: Array.isArray(comp?.limitations)
        ? comp.limitations.map((lim: any) => String(unwrapTracked(lim, ""))).filter(Boolean)
        : [],
    }));

    const rawMemories = Array.isArray(life?.memories) ? life.memories : [];
    const memories = rawMemories.map((mem: any, mIdx: number) => ({
      id: String(mem?.code || `mem-${idx}-${mIdx}`),
      type: (String(unwrapTracked(mem?.memory_type, "Episodic")) as any) || "Episodic",
      status: (String(unwrapTracked(mem?.retrieval_status, "Active")) as any) || "Active",
      content: String(unwrapTracked(mem?.summary, "") || unwrapTracked(mem?.title, "") || ""),
      sourceScene: String(unwrapTracked(mem?.origin_scene_code, "") || ""),
    }));

    return {
      id: code,
      name,
      nickname,
      gender: genderBadge,
      genderBadge,
      motif: {
        description: motifDesc,
      },
      appearance: {
        species: String(unwrapTracked(app?.species, "Con người") || "Con người"),
        birth: String(unwrapTracked(app?.birth, "") || "Chưa rõ"),
        gender: {
          identity: String(unwrapTracked(app?.gender?.identity, genderBadge) || genderBadge),
          expression: String(unwrapTracked(app?.gender?.expression, genderBadge) || genderBadge),
        },
        heightCm: Number(unwrapTracked(app?.height_cm, 170)) || 170,
        weightKg: Number(unwrapTracked(app?.weight_kg, 60)) || 60,
        hair: {
          color: String(unwrapTracked(app?.hair?.color, "Đen") || "Đen"),
          style: String(unwrapTracked(app?.hair?.style, "Tự nhiên") || "Tự nhiên"),
          texture: String(unwrapTracked(app?.hair?.texture, "Mềm") || "Mềm"),
          length: String(unwrapTracked(app?.hair?.length, "Vừa") || "Vừa"),
        },
        skin: {
          tone: String(unwrapTracked(app?.skin?.tone, "Tự nhiên") || "Tự nhiên"),
          texture: String(unwrapTracked(app?.skin?.texture, "Bình thường") || "Bình thường"),
        },
        eyes: {
          pupil: {
            shape: String(unwrapTracked(app?.eyes?.pupil?.shape, "Tròn") || "Tròn"),
            color: String(unwrapTracked(app?.eyes?.pupil?.color, "Đen") || "Đen"),
          },
          iris: {
            texture: unwrapTracked(app?.eyes?.iris?.texture, null),
            color: String(unwrapTracked(app?.eyes?.iris?.color, "Nâu") || "Nâu"),
          },
          sclera: String(unwrapTracked(app?.eyes?.sclera, "Trắng") || "Trắng"),
        },
        outfitStyle: String(unwrapTracked(app?.outfit_style, "Giản dị") || "Giản dị"),
        distinguishingMarks,
        summary: String(unwrapTracked(app?.default_summary, "") || motifDesc),
      },
      competences,
      memories,
      psychological: {
        coreTrauma: {
          wound: String(unwrapTracked(psych?.core_trauma_and_belief?.the_wound, "") || "Chưa xác định"),
          lie: String(unwrapTracked(psych?.core_trauma_and_belief?.the_lie, "") || "Chưa xác định"),
          fear: String(unwrapTracked(psych?.core_trauma_and_belief?.fundamental_fear, "") || "Chưa xác định"),
        },
        moralFramework: {
          alignment: String(unwrapTracked(psych?.cognitive_moral_framework?.moral_alignment, "NEUTRAL") || "NEUTRAL"),
          code: String(unwrapTracked(psych?.cognitive_moral_framework?.moral_code, "") || "Chưa xác định"),
          breakingPoint: String(unwrapTracked(psych?.cognitive_moral_framework?.breaking_point, "") || "Chưa xác định"),
        },
        narrativeEngine: {
          want: String(unwrapTracked(psych?.narrative_engine?.external_want, "") || "Chưa xác định"),
          need: String(unwrapTracked(psych?.narrative_engine?.internal_need, "") || "Chưa xác định"),
          conflict: String(unwrapTracked(psych?.narrative_engine?.internal_conflict, "") || "Chưa xác định"),
        },
      },
    };
  });

  const chapArcs = Array.isArray(bfd.optional?.chapter_phase?.arcs)
    ? bfd.optional.chapter_phase.arcs
    : [];

  const arcs = optArcs.map((arc: any, idx: number) => {
    const arcCode = String(arc?.code || `ARC_${idx + 1}`);
    const arcTitle = String(unwrapTracked(arc?.display_name, "") || `Arc ${idx + 1}`);
    const arcPos = Number(unwrapTracked(arc?.position, idx + 1)) || idx + 1;
    const arcSummary = String(unwrapTracked(arc?.summary, "") || "");
    const motifCode = String(unwrapTracked(arc?.motif?.code, "") || "");
    const motifName = String(unwrapTracked(arc?.motif?.display_name, "") || motifCode || "Khởi đầu");
    const eventBlocks = Array.isArray(arc?.motif?.event_blocks) ? arc.motif.event_blocks : [];
    const segments = eventBlocks.map((eb: any, sIdx: number) => ({
      id: `${arcCode}-seg-${sIdx + 1}`,
      chapterRange: `${unwrapTracked(eb?.start_chapter, 1)}-${unwrapTracked(eb?.end_chapter, 1)}`,
      phaseNumber: Number(unwrapTracked(eb?.position, sIdx + 1)) || sIdx + 1,
      summary: String(unwrapTracked(eb?.summary, "") || ""),
    }));

    const themeArgs = Array.isArray(arc?.themes?.arguments) ? arc.themes.arguments : [];
    const chapArc = chapArcs.find((ca: any) => ca?.code === arcCode);
    const rawChapters = Array.isArray(chapArc?.chapters) ? chapArc.chapters : [];
    const chapters = rawChapters.map((ch: any, cIdx: number) => {
      const rawScenes = Array.isArray(ch?.scenes) ? ch.scenes : [];
      return {
        displayName: String(unwrapTracked(ch?.display_name, "") || `Chương ${cIdx + 1}`),
        position: Number(unwrapTracked(ch?.position, cIdx + 1)) || cIdx + 1,
        summary: String(unwrapTracked(ch?.summary, "") || ""),
        scenes: rawScenes.map((sc: any, scIdx: number) => ({
          code: String(sc?.code || `SC_${cIdx + 1}_${scIdx + 1}`),
          displayName: String(unwrapTracked(sc?.display_name, "") || `Cảnh ${scIdx + 1}`),
          position: Number(unwrapTracked(sc?.position, scIdx + 1)) || scIdx + 1,
          summary: String(unwrapTracked(sc?.summary, "") || ""),
        })),
      };
    });

    return {
      id: arcCode,
      number: arcPos,
      title: arcTitle,
      subnote: motifName,
      segmentsCount: segments.length,
      summary: arcSummary,
      motif: {
        code: motifCode,
        displayName: motifName,
        category: String(unwrapTracked(arc?.motif?.category, "") || "Cốt truyện"),
        note: String(unwrapTracked(arc?.motif?.note, "") || ""),
      },
      theme: {
        question: String(unwrapTracked(arc?.themes?.question, "") || ""),
        message: String(unwrapTracked(arc?.themes?.message, "") || ""),
        resolution: String(
          unwrapTracked(arc?.themes?.resolution?.content, "") ||
            unwrapTracked(arc?.themes?.resolution?.type, "") ||
            unwrapTracked(arc?.themes?.resolution, "") ||
            ""
        ),
        arguments: themeArgs.map((arg: any, aIdx: number) => {
          const cCode = String(unwrapTracked(arg?.character_code, "") || "");
          return {
            id: `${arcCode}-arg-${aIdx + 1}`,
            characterName: charMap.get(cCode) || cCode || "Nhân vật",
            role: (String(unwrapTracked(arg?.role, "PRIMARY")) as any) || "PRIMARY",
            reactionType: (String(unwrapTracked(arg?.reaction_type, "ACTION")) as any) || "ACTION",
            note: String(unwrapTracked(arg?.note, "") || ""),
          };
        }),
      },
      segments,
      chapters: chapters.length > 0 ? chapters : undefined,
    };
  });

  const rawWorldSettings = bfd.optional?.world_building_phase?.world_settings;
  const wsItem = Array.isArray(rawWorldSettings) ? rawWorldSettings[0] : rawWorldSettings;
  const worldSettings = wsItem
    ? {
        code: String(wsItem?.code || "WORLD_01"),
        displayName: String(unwrapTracked(wsItem?.display_name, "") || "Bối cảnh thế giới"),
        type: String(unwrapTracked(wsItem?.type, "") || "WORLD"),
        description: String(unwrapTracked(wsItem?.description, "") || ""),
        access: String(unwrapTracked(wsItem?.access, "") || "PUBLIC"),
        loreEntries: (Array.isArray(wsItem?.lore_entries) ? wsItem.lore_entries : []).map(
          (le: any, lIdx: number) => ({
            code: String(le?.code || `LORE_${lIdx + 1}`),
            displayName: String(unwrapTracked(le?.display_name, "") || `Tri thức ${lIdx + 1}`),
            type: (String(unwrapTracked(le?.category, "HISTORY")) as any) || "HISTORY",
            description: String(unwrapTracked(le?.description, "") || ""),
            data: String(unwrapTracked(le?.content, "") || unwrapTracked(le?.description, "") || ""),
          })
        ),
      }
    : baseOverview.worldSettings;

  const rawRels = Array.isArray(bfd.optional?.relationship_phase?.relationship)
    ? bfd.optional.relationship_phase.relationship
    : [];
  const relationships =
    rawRels.length > 0
      ? rawRels.map((rel: any, rIdx: number) => {
          const fromCode = String(unwrapTracked(rel?.from_character_code, "") || "");
          const toCode = String(unwrapTracked(rel?.to_character_code, "") || "");
          return {
            id: String(rel?.code || `rel-${rIdx + 1}`),
            fromCharacterName: charMap.get(fromCode) || fromCode || "Nhân vật A",
            toCharacterName: charMap.get(toCode) || toCode || "Nhân vật B",
            role: {
              hierarchy: "EQUAL" as const,
              category: (String(unwrapTracked(rel?.relationship_type, "SOCIAL")) as any) || "SOCIAL",
              subType: String(unwrapTracked(rel?.relationship_type, "Đồng hành") || "Đồng hành"),
              isPrimary: true,
            },
            states: {
              trust: "MODERATE",
              affection: "MODERATE",
              hostility: "NONE",
              fear: "NONE",
              respect: "MODERATE",
              familiarity: "MODERATE",
              dependency: "LOW",
            },
            motif: {
              code: String(unwrapTracked(rel?.motif?.code, "") || ""),
              description: String(unwrapTracked(rel?.summary, "") || ""),
            },
          };
        })
      : baseOverview.relationships;

  return {
    ...baseOverview,
    displayName: displayName || baseOverview.displayName,
    language: language || baseOverview.language,
    summary: summary || baseOverview.summary,
    target: {
      arcs: arcCount || baseOverview.target.arcs,
      chapters: chapterCount || baseOverview.target.chapters,
      wordPerChapter: wordPerChapter || baseOverview.target.wordPerChapter,
    },
    genres: genres.length > 0 ? genres : baseOverview.genres,
    arcs,
    characters,
    worldSettings,
    relationships,
  };
}

// ============================================================================
// DraftWorkspace Component
// ============================================================================

export interface DraftWorkspaceProps {
  onClose: () => void;
  onFeedback: (feedback: FeedbackContext) => void;
  overview?: DraftOverview;
  businessFlowData?: any;
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
  overview: propOverview,
  businessFlowData: propBusinessFlowData,
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
      currentVersionLabel: (propOverview ?? DRAFT_OVERVIEW).version || "v.01",
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

  // Load workspace state, versions and pending patches on mount
  useEffect(() => {
    controller.loadWorkspaceState();
    controller.loadVersions();
    controller.loadPendingPatches();
  }, [controller]);

  // Defer the heavy JSON so a completed phase does not block input or animation frames.
  const deferredBusinessFlowData = useDeferredValue(
    controllerState.businessFlowData ?? propBusinessFlowData
  );
  const overview = useMemo(() => {
    if (propOverview) return propOverview;
    if (deferredBusinessFlowData) {
      return mapBusinessFlowDataToDraftOverview(deferredBusinessFlowData, DRAFT_OVERVIEW);
    }
    return DRAFT_OVERVIEW;
  }, [propOverview, deferredBusinessFlowData]);

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

  // Transitions are instant: phase data comes from the backend, not a timed animation.
  const [transitionPhase, setTransitionPhase] = useState<'idle' | 'fade-out' | 'loading'>('idle');
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

  const triggerTransition = (action: () => void) => {
    action();
    setTransitionPhase('idle');
  };

  // Phase number for the current workspace step (`core.md` §3.2); running a phase
  // executes the segmented workflow with the seeded phase prompt.
  const phaseForStep: Record<DraftStep, number> = {
    overview: 1,
    chapter: 2,
    world: 3,
    scene: 4,
    character: 5,
    relationship: 6,
  };

  const [runProgress, setRunProgress] = useState<string | null>(null);

  const handleRunPhase = () => {
    if (controllerState.workflowStatus === 'running') return;
    const phase = phaseForStep[currentStep];
    const subphase = currentStep === 'character'
      ? ({ appearance: 1, psychological: 2, lifeExperience: 3 } as const)[currentSubStep]
      : null;
    setRunProgress(null);
    controller
      .startOrResumeWorkflow(phase, subphase, (event) => {
        // Only small progress strings cross into React state; no outline re-mapping per event.
        if (event.event === 'taskCompleted') {
          setRunProgress(`${event.data.completedTasks}/${event.data.totalTasks}`);
        }
      })
      .then(() => controller.loadWorkspaceState())
      .catch(() => {
        /* controller state already exposes the failure */
      })
      .finally(() => setRunProgress(null));
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

  // Reveal arcs in small batches instead of mounting the whole outline in one commit.
  const ARC_BATCH_SIZE = 2;
  const [visibleArcCount, setVisibleArcCount] = useState(ARC_BATCH_SIZE);
  useEffect(() => {
    setVisibleArcCount(ARC_BATCH_SIZE);
  }, [overview.arcs.length, currentStep]);
  useEffect(() => {
    if (visibleArcCount >= overview.arcs.length) return;
    const timer = setTimeout(
      () => setVisibleArcCount((count) => Math.min(count + ARC_BATCH_SIZE, overview.arcs.length)),
      16,
    );
    return () => clearTimeout(timer);
  }, [visibleArcCount, overview.arcs.length]);

  const phaseOverview = useMemo(() => {
    const isStepOverview = currentStep === 'overview';
    const isStepRelationship = currentStep === 'relationship';

    const revealedArcs = overview.arcs.slice(0, visibleArcCount);

    return {
      ...overview,
      arcs: isStepOverview
        ? revealedArcs.map((arc) => ({
            ...arc,
            chapters: undefined,
          }))
        : revealedArcs,
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
  }, [overview, currentStep, currentSubStep, visibleArcCount]);

  // Keep typing/scrolling responsive while a large outline tree is committed.
  const deferredPhaseOverview = useDeferredValue(phaseOverview);

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
            overview={deferredPhaseOverview}
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
            onRunPhase={handleRunPhase}
            runProgress={runProgress}
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

