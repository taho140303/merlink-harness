/** Wave 0.3B & Phase 5 Sprint 3 — navigation rules and commit reconciliation for Draft -> Project. */

export type CommitResult =
  | { status: 'committed'; projectId: string; redirectPath?: string }
  | { status: 'failed'; reason: string; structuredError?: any }
  | { status: 'unknown'; reason?: string };

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
 * Route for navigating to the real committed project in View mode (REQ-FE-003, P5S3-P).
 * Normative contract: `/projects/${projectId}` in View mode.
 */
export function buildCommittedProjectViewPath(projectId: string): string {
  return `/projects/${projectId}`;
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

/* ---------- Phase 5 Sprint 3: Backend Commit & Reconciliation Integration ---------- */

export interface CommitResultDto {
  status: "committed" | "failed" | "unknown" | string;
  projectId?: string | null;
  redirectPath?: string | null;
  reason?: string | null;
  error?: any;
}

/**
 * Adapts backend commit response into normalized CommitResult.
 */
export function fromCommitResultDto(dto: any): CommitResult {
  if (!dto) return { status: 'unknown', reason: 'empty response' };
  if (dto.status === 'committed' && dto.projectId) {
    return {
      status: 'committed',
      projectId: dto.projectId,
      redirectPath: dto.redirectPath || buildCommittedProjectViewPath(dto.projectId),
    };
  }
  if (dto.status === 'failed' || dto.error) {
    return {
      status: 'failed',
      reason: dto.reason || (typeof dto.error === 'string' ? dto.error : (dto.error?.message || 'Commit failed')),
      structuredError: dto.error || dto,
    };
  }
  return { status: 'unknown', reason: dto.reason || 'unrecognized status' };
}

/**
 * Resolves real navigation for committed project.
 */
export function resolveRealNavigation(
  result: CommitResult | null,
  isPending: boolean
): NavigationDecision {
  if (isPending) return { ok: false, reason: 'pending' };
  if (!result || result.status !== 'committed') return { ok: false, reason: 'notCommitted' };
  if (!result.projectId || typeof result.projectId !== 'string' || result.projectId.trim() === '') {
    return { ok: false, reason: 'missingProjectId' };
  }
  return {
    ok: true,
    projectId: result.projectId,
    path: buildCommittedProjectViewPath(result.projectId),
  };
}

export interface ExecuteDraftCommitOptions {
  client: any;
  draftSessionId: string;
  onSuccess?: (projectId: string, path: string) => void;
  onError?: (error: any) => void;
}

/**
 * Executes draft commit with reconciliation against backend head / durable state.
 *
 * Requirements:
 * 1. Success (`P5S3-P`): calls onSuccess with projectId and `/projects/${projectId}` path.
 * 2. Failed TX-05 (`P5S3-B`): preserves draft state, reports structured backend error, no navigation.
 * 3. Lost response: queries durable backend (`getDraftWorkspaceState` / `listProjects`)
 *    before claiming failure or retrying, never double-committing.
 */
export async function executeDraftCommitWithReconciliation(
  options: ExecuteDraftCommitOptions
): Promise<CommitResult> {
  const { client, draftSessionId, onSuccess, onError } = options;
  try {
    const commitFn = client?.commitDraftToProject
      ? client.commitDraftToProject.bind(client)
      : (client?.commitDraft ? client.commitDraft.bind(client) : null);

    if (!commitFn) {
      const err = new Error("Backend client does not support commitDraft");
      onError?.(err);
      return { status: 'failed', reason: err.message, structuredError: err };
    }

    const rawResponse = await commitFn({ draftSessionId });
    const result = fromCommitResultDto(rawResponse);

    if (result.status === 'committed') {
      const path = result.redirectPath || buildCommittedProjectViewPath(result.projectId);
      onSuccess?.(result.projectId, path);
    } else if (result.status === 'failed') {
      onError?.(result.structuredError || result);
    }
    return result;
  } catch (err: any) {
    const isLostResponse =
      err?.code === 'NETWORK_ERROR' ||
      err?.code === 'TIMEOUT' ||
      err?.code === 'BACKEND_UNAVAILABLE' ||
      err?.name === 'BackendUnavailableError' ||
      err?.isLostResponse === true ||
      (typeof err?.message === 'string' && (
        err.message.toLowerCase().includes('network') ||
        err.message.toLowerCase().includes('timeout') ||
        err.message.toLowerCase().includes('unavailable') ||
        err.message.toLowerCase().includes('lost response')
      ));

    if (isLostResponse) {
      // Reconcile with durable backend before claiming failure
      try {
        const stateFn = client?.getDraftWorkspaceState ? client.getDraftWorkspaceState.bind(client) : null;
        if (stateFn) {
          const state = await stateFn({ draftSessionId });
          if (state && (state.status === 'committed' || state.projectId)) {
            const projectId = state.projectId || 'committed-project';
            const path = buildCommittedProjectViewPath(projectId);
            const reconciledResult: CommitResult = { status: 'committed', projectId, redirectPath: path };
            onSuccess?.(projectId, path);
            return reconciledResult;
          }
        }

        const listProjFn = client?.listProjects ? client.listProjects.bind(client) : null;
        if (listProjFn) {
          const listRes = await listProjFn({});
          const matching = listRes?.items?.find((p: any) => p.draftSessionId === draftSessionId);
          if (matching) {
            const path = buildCommittedProjectViewPath(matching.id);
            const reconciledResult: CommitResult = { status: 'committed', projectId: matching.id, redirectPath: path };
            onSuccess?.(matching.id, path);
            return reconciledResult;
          }
        }
      } catch {
        // Fall through to failure report
      }
    }

    const failedResult: CommitResult = {
      status: 'failed',
      reason: err?.message || 'Commit failed',
      structuredError: err,
    };
    onError?.(err);
    return failedResult;
  }
}

