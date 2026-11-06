import type { DraftOverview } from './types';

/** Wave 0.3A — pure presentation logic for the Phase 6 commit confirmation UI. No backend, no persistence. */

export type ValidationStatus = 'ready' | 'blocked' | 'unknown';

export interface ValidationSummary {
  status: ValidationStatus;
  blockingReasons: string[];
}

export type PendingPatchStatus = { status: 'known'; count: number } | { status: 'unknown' };

/** Readiness data supplied by a (future) backend. Absent fields mean "not evaluated". */
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
  | { status: 'success-preview' };

export type CommitOpEvent = 'submit' | 'fail' | 'succeed' | 'reset';

export function reduceCommitOp(state: CommitOpState, event: CommitOpEvent): CommitOpState {
  switch (event) {
    case 'submit':
      // Duplicate submissions while pending are ignored; success preview is terminal until reset.
      return state.status === 'idle' || state.status === 'error' ? { status: 'pending' } : state;
    case 'fail':
      return state.status === 'pending' ? { status: 'error' } : state;
    case 'succeed':
      return state.status === 'pending' ? { status: 'success-preview' } : state;
    case 'reset':
      return state.status === 'pending' ? state : { status: 'idle' };
  }
}

export function canDismissCommitDialog(state: CommitOpState): boolean {
  return state.status !== 'pending';
}
