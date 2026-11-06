/** Wave 0.3B — pure rules for the simulated post-commit navigation preview. No backend, no project creation. */

export type CommitResult =
  | { status: 'committed'; projectId: string }
  | { status: 'failed'; reason: string }
  | { status: 'unknown' };

export interface SampleProjectRef {
  id: string;
  title: string;
}

export type NavigationBlockReason =
  | 'pending'
  | 'notSimulated'
  | 'notCommitted'
  | 'missingProjectId'
  | 'invalidProject';

export type NavigationDecision =
  | { ok: true; projectId: string; path: string }
  | { ok: false; reason: NavigationBlockReason };

/** Existing route `/project/:projectId`; no `mode` param → Project Detail opens in View Mode. */
export function buildProjectPath(projectId: string): string {
  return `/project/${projectId}`;
}

/** Existing route used by ProjectCreatorPanel after the user manually selects a chapter. */
export function buildCreatorChapterPath(projectId: string, chapterId: string): string {
  return `/project/${projectId}/chapter/${chapterId}?mode=creator`;
}

/**
 * Only a simulated, non-pending `committed` result whose projectId resolves to an
 * existing sample project may navigate. Never falls back to another project.
 */
export function resolveSimulatedNavigation(input: {
  result: CommitResult | null;
  pending: boolean;
  simulationActive: boolean;
  projects: readonly SampleProjectRef[];
}): NavigationDecision {
  if (input.pending) return { ok: false, reason: 'pending' };
  if (!input.simulationActive) return { ok: false, reason: 'notSimulated' };
  const r = input.result;
  if (!r || r.status !== 'committed') return { ok: false, reason: 'notCommitted' };
  if (typeof r.projectId !== 'string' || r.projectId.trim() === '') return { ok: false, reason: 'missingProjectId' };
  const project = input.projects.find((p) => p.id === r.projectId);
  if (!project) return { ok: false, reason: 'invalidProject' };
  return { ok: true, projectId: project.id, path: buildProjectPath(project.id) };
}

/** Runs the action at most once. */
export function createOnceGuard(): (action: () => void) => boolean {
  let used = false;
  return (action) => {
    if (used) return false;
    used = true;
    action();
    return true;
  };
}
