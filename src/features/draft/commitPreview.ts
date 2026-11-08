import type { DraftOverview } from './types';
import type {
  CommitPreviewDto,
} from '@/shared/backend/generated';

/** Wave 0.3A & Phase 5 Sprint 3 — presentation & DTO integration logic for the Phase 6 commit confirmation UI. */

export const WORKFLOW_BASELINE_UNAVAILABLE = 'WORKFLOW_BASELINE_UNAVAILABLE';
export const REQUIRES_REPLAN = 'requires_replan';
export const EXPIRED_BY_RESTORE = 'EXPIRED_BY_RESTORE';

export type ValidationStatus = 'ready' | 'blocked' | 'unknown';

export interface ValidationSummary {
  status: ValidationStatus;
  blockingReasons: string[];
}

export type PendingPatchStatus = { status: 'known'; count: number } | { status: 'unknown' };

/** Readiness data supplied by backend. Absent fields mean "not evaluated". */
export interface CommitReadinessInput {
  validation?: { ready?: boolean; blockingReasons?: string[] } | null;
  pendingPatches?: { count: number } | null;
}

export function deriveValidation(input?: CommitReadinessInput | null): ValidationSummary {
  const v = input?.validation;
  if (!v) return { status: 'unknown', blockingReasons: [] };
  const reasons = Array.isArray(v.blockingReasons) ? v.blockingReasons.filter(Boolean) : [];
  if (reasons.length > 0 || v.ready === false) return { status: 'blocked', blockingReasons: reasons };
  if (v.ready === true) return { status: 'ready', blockingReasons: [] };
  return { status: 'unknown', blockingReasons: [] };
}

export function derivePendingPatches(input?: CommitReadinessInput | null): PendingPatchStatus {
  const p = input?.pendingPatches;
  if (!p || typeof p.count !== 'number' || !Number.isInteger(p.count) || p.count < 0) {
    return { status: 'unknown' };
  }
  return { status: 'known', count: p.count };
}

export type CommitBlockReason =
  | 'validationBlocked'
  | 'validationUnknown'
  | 'patchesPending'
  | 'patchesUnknown'
  | 'operationPending';

export function getCommitBlockReasons(
  validation: ValidationSummary,
  patches: PendingPatchStatus,
  operationPending: boolean,
): CommitBlockReason[] {
  const reasons: CommitBlockReason[] = [];
  if (validation.status === 'blocked') reasons.push('validationBlocked');
  else if (validation.status === 'unknown') reasons.push('validationUnknown');
  if (patches.status === 'unknown') reasons.push('patchesUnknown');
  else if (patches.count > 0) reasons.push('patchesPending');
  if (operationPending) reasons.push('operationPending');
  return reasons;
}

export function isCommitEligible(
  validation: ValidationSummary,
  patches: PendingPatchStatus,
  operationPending: boolean,
): boolean {
  return getCommitBlockReasons(validation, patches, operationPending).length === 0;
}

/** `null` means unavailable — never fabricated. */
export interface PlanSummary {
  projectName: string | null;
  description: string | null;
  arcs: number | null;
  chapters: number | null;
  scenes: number | null;
  characters: number | null;
  relationships: number | null;
  worldbuildingAvailable: boolean;
}

export function summarizePlan(overview: DraftOverview): PlanSummary {
  const name = overview.displayName?.trim();
  const desc = overview.summary?.trim();
  const arcsList = Array.isArray(overview.arcs) ? overview.arcs : null;

  let chapters: number | null = null;
  let scenes: number | null = null;
  const sceneArcs = overview.sceneConfig?.arcs;
  if (Array.isArray(sceneArcs) && sceneArcs.length > 0) {
    chapters = sceneArcs.reduce((n, a) => n + a.chapters.length, 0);
    scenes = sceneArcs.reduce((n, a) => n + a.chapters.reduce((m, c) => m + c.scenes.length, 0), 0);
  } else if (arcsList && arcsList.length > 0 && arcsList.every((a) => Array.isArray(a.chapters))) {
    chapters = arcsList.reduce((n, a) => n + (a.chapters?.length ?? 0), 0);
    scenes = arcsList.reduce(
      (n, a) => n + (a.chapters ?? []).reduce((m, c) => m + (Array.isArray(c.scenes) ? c.scenes.length : 0), 0),
      0,
    );
  }

  return {
    projectName: name || null,
    description: desc || null,
    arcs: arcsList ? arcsList.length : null,
    chapters,
    scenes,
    characters: Array.isArray(overview.characters) ? overview.characters.length : null,
    relationships: Array.isArray(overview.relationships) ? overview.relationships.length : null,
    worldbuildingAvailable: !!overview.worldSettings,
  };
}

/* ---------- Simulated operation (preview only; never persists or navigates) ---------- */

export type CommitOpState =
  | { status: 'idle' }
  | { status: 'pending' }
  | { status: 'error' }
  | { status: 'unknown' }
  | { status: 'success-preview' };

export type CommitOpEvent = 'submit' | 'fail' | 'unknown' | 'succeed' | 'reset';

export function reduceCommitOp(state: CommitOpState, event: CommitOpEvent): CommitOpState {
  switch (event) {
    case 'submit':
      // Duplicate submissions while pending are ignored; success preview is terminal until reset.
      return state.status === 'idle' || state.status === 'error' || state.status === 'unknown' ? { status: 'pending' } : state;
    case 'fail':
      return state.status === 'pending' ? { status: 'error' } : state;
    case 'unknown':
      return state.status === 'pending' ? { status: 'unknown' } : state;
    case 'succeed':
      return state.status === 'pending' ? { status: 'success-preview' } : state;
    case 'reset':
      return state.status === 'pending' ? state : { status: 'idle' };
  }
}

export function canDismissCommitDialog(state: CommitOpState): boolean {
  return state.status !== 'pending';
}

/* ---------- Phase 5 Sprint 3: Backend DTO Integration & Task Receipt Applicability ---------- */

export interface TaskReceipt {
  taskId: string;
  status: 'Verified' | 'Corrupt' | 'Indeterminate' | 'UnsupportedCodec' | string;
  headVersionSeq?: number | null;
  dependencyFingerprint?: string | null;
}

/**
 * Historical `Verified` receipt alone never marks current task applicable
 * without current head and dependency verification (REQ-FE-003, VER-06/VER-07).
 */
export function isTaskApplicable(
  receipt: TaskReceipt | null | undefined,
  currentHeadSeq: number,
  expectedDependencyFingerprint?: string | null
): boolean {
  if (!receipt) return false;
  if (receipt.status !== 'Verified') return false;
  if (typeof receipt.headVersionSeq === 'number' && receipt.headVersionSeq !== currentHeadSeq) {
    return false;
  }
  if (
    expectedDependencyFingerprint !== undefined &&
    expectedDependencyFingerprint !== null &&
    receipt.dependencyFingerprint !== expectedDependencyFingerprint
  ) {
    return false;
  }
  return true;
}

export interface DerivedCommitPreview {
  validation: ValidationSummary;
  pendingPatches: PendingPatchStatus;
  plan: PlanSummary;
  canConfirm: boolean;
  readiness: CommitReadinessInput;
  draftSessionId?: string;
  status?: string;
}

/**
 * Maps backend CommitPreviewDto into UI preview/readiness models.
 * Missing fields remain explicitly unavailable, never fabricated.
 */
export function fromCommitPreviewDto(dto?: CommitPreviewDto | null): DerivedCommitPreview {
  if (!dto) {
    const emptyValidation = deriveValidation(undefined);
    const emptyPatches = derivePendingPatches(undefined);
    return {
      validation: emptyValidation,
      pendingPatches: emptyPatches,
      plan: {
        projectName: null,
        description: null,
        arcs: null,
        chapters: null,
        scenes: null,
        characters: null,
        relationships: null,
        worldbuildingAvailable: false,
      },
      canConfirm: false,
      readiness: { validation: null, pendingPatches: null },
    };
  }

  let valStatus: ValidationStatus = 'unknown';
  const blockingReasons: string[] = [];
  if (dto.validation) {
    if (Array.isArray(dto.validation.issues)) {
      for (const issue of dto.validation.issues) {
        if (issue.severity === 'error' || !issue.severity) {
          blockingReasons.push(issue.message || `${issue.field}: error`);
        }
      }
    }
    if (dto.validation.status === 'blocked' || blockingReasons.length > 0) {
      valStatus = 'blocked';
    } else if (dto.validation.status === 'ready') {
      valStatus = 'ready';
    }
  }
  const validation: ValidationSummary = { status: valStatus, blockingReasons };

  let patches: PendingPatchStatus = { status: 'unknown' };
  if (dto.pendingPatches) {
    const count =
      typeof dto.pendingPatches.count === 'number' && Number.isInteger(dto.pendingPatches.count) && dto.pendingPatches.count >= 0
        ? dto.pendingPatches.count
        : Array.isArray(dto.pendingPatches.items)
          ? dto.pendingPatches.items.length
          : -1;
    if (count >= 0) {
      patches = { status: 'known', count };
    }
  }

  const p = dto.planSummary;
  const plan: PlanSummary = {
    projectName: p?.projectName?.trim() || null,
    description: null,
    arcs: typeof p?.arcCount === 'number' ? p.arcCount : (typeof p?.targetArcs === 'number' ? p.targetArcs : null),
    chapters: typeof p?.chapterCount === 'number' ? p.chapterCount : (typeof p?.targetChapters === 'number' ? p.targetChapters : null),
    scenes: null,
    characters: typeof p?.characterCount === 'number' ? p.characterCount : null,
    relationships: null,
    worldbuildingAvailable: Boolean(
      (p?.worldSettingCount && p.worldSettingCount > 0) ||
      (p?.loreEntryCount && p.loreEntryCount > 0)
    ),
  };

  const eligible = isCommitEligible(validation, patches, false);
  const canConfirm = typeof dto.canConfirm === 'boolean' ? (dto.canConfirm && eligible) : eligible;

  const readiness: CommitReadinessInput = {
    validation: {
      ready: validation.status === 'ready',
      blockingReasons: validation.blockingReasons,
    },
    pendingPatches: patches.status === 'known' ? { count: patches.count } : null,
  };

  return {
    validation,
    pendingPatches: patches,
    plan,
    canConfirm,
    readiness,
    draftSessionId: dto.draftSessionId,
    status: dto.status,
  };
}

