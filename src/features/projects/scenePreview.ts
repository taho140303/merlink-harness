import type { ChapterBlock, ChapterItem } from "./types";
import type { BackendClient } from "@/shared/backend/client";
import type {
  ChapterWritingReadModelDto,
  SceneLockStateDto,
  BulkSceneLockStateDto,
  CancelSceneGenerationResponseDto,
  SceneVersionListDto,
  SceneVersionItemDto,
  SaveChapterResponse,
  DirtySceneSaveItem,
  SaveReceiptLookupDto,
  SceneGenerationStreamEvent,
} from "@/shared/backend/generated";

export type SceneSource = "AI_GENERATED" | "HUMAN_EDITED" | "RESTORE";

export interface SceneBlock extends ChapterBlock {
  /** One scene per block. Preview data derives this from the chapter and block id, or real scenes.id from backend. */
  sceneId: string;
  headVersionNumber?: number;
  currentSceneSource?: "AI" | "USER" | "RESTORE" | string;
  position?: number;
  wordCount?: number;
  displayName?: string;
  summary?: string | null;
}

export interface SceneVersionPreview {
  versionNumber: number;
  source: SceneSource;
  createdAt: string;
  content: string;
}

/** Frontend-only request model for one scene regeneration. Never persisted. */
export interface SceneRegenerationRequest {
  sceneId: string;
  note: string | null;
}

/**
 * In-memory generation candidate maintaining SCOPE-06 provenance:
 * request_id, generation_id, scene_id, baseline_version_number, authorization_valid, status.
 */
export interface SceneGenerationCandidate {
  requestId: string;
  generationId: string;
  sceneId: string;
  baselineVersionNumber: number;
  authorizationValid: boolean;
  status: "streaming" | "completed" | "review" | "cancelled" | "invalidated_by_lock" | "failed";
  text: string;
  note?: string | null;
}

export interface SaveChapterScenesReq {
  projectId: string;
  chapterId: string;
  clientSaveId: string;
  expectedChapterRevision?: number;
  dirtyScenes: Array<{
    sceneId: string;
    expectedVersionNumber?: number;
    content: string;
    sceneSource?: "AI" | "USER" | "RESTORE" | string;
    summary?: string | null;
    isLocked?: boolean;
  }>;
}

/** Empty or whitespace-only feedback means "no additional guidance". */
export function normalizeNote(value: string): string | null {
  const trimmed = value.trim();
  return trimmed.length > 0 ? trimmed : null;
}

export function buildSceneRegenerationRequest(
  sceneId: string,
  noteInput: string,
): SceneRegenerationRequest {
  return { sceneId, note: normalizeNote(noteInput) };
}

export type SceneActionSource = "accept" | "edit" | "restore";

/** Source attribution is determined by the final action on the working content. */
export function sourceForSceneAction(action: SceneActionSource): SceneSource {
  switch (action) {
    case "accept":
      return "AI_GENERATED";
    case "restore":
      return "RESTORE";
    default:
      return "HUMAN_EDITED";
  }
}

/** Dirty means the working content/lock differs from the scene baseline. Source alone never counts. */
export function isSceneDirty(
  working: Pick<SceneBlock, "text" | "locked">,
  baseline: Pick<SceneBlock, "text" | "locked"> | undefined,
): boolean {
  if (!baseline) return true;
  return working.text !== baseline.text || working.locked !== baseline.locked;
}

/**
 * Content-dirty check: Lock is NOT content-dirty (REQ-FE-005).
 * Locking/unlocking does not mark scene content dirty and preserves dirty content buffers.
 */
export function isSceneContentDirty(
  working: Pick<SceneBlock, "text">,
  baseline: Pick<SceneBlock, "text"> | undefined,
): boolean {
  if (!baseline) return true;
  return working.text !== baseline.text;
}

/**
 * Bulk lock changes (Lock All / Unlock All) are only safe while no scene
 * generation or review is active. Per-scene lock rules are handled separately.
 */
export function canBulkChangeLocks(
  generation: { status: "generating" | "review" | string } | null,
): boolean {
  return generation === null;
}

/** Applies a bulk lock state, or returns the blocks untouched when blocked. */
export function applyBulkLock<T extends Pick<SceneBlock, "locked">>(
  blocks: readonly T[],
  locked: boolean,
  generation: { status: "generating" | "review" | string } | null,
): T[] {
  if (!canBulkChangeLocks(generation)) return blocks as T[];
  return blocks.map((block) => ({ ...block, locked }));
}

const FALLBACK_BLOCK: ChapterBlock = {
  id: "blk-fallback",
  text: "Nội dung chương đang được soạn thảo...",
  locked: false,
};

/**
 * Adds a stable scene identity without relying on the array index.
 * The future read model supplies `sceneId`; preview fixtures derive it from
 * the chapter id and the block id, both stable for the lifetime of the mock.
 */
export function buildSceneBlocks(chapter: ChapterItem): SceneBlock[] {
  const source =
    chapter.blocks && chapter.blocks.length > 0 ? chapter.blocks : [FALLBACK_BLOCK];

  return source.map((block) => ({
    ...block,
    sceneId: block.sceneId ?? `scn-${chapter.id}-${block.id}`,
  }));
}

/**
 * Maps authoritative ChapterWritingReadModelDto scenes into SceneBlocks using real scenes.id.
 */
export function buildSceneBlocksFromReadModel(
  readModel: ChapterWritingReadModelDto,
): SceneBlock[] {
  return (readModel.scenes || []).map((s) => ({
    id: s.id,
    sceneId: s.id, // REAL UUIDv7 scenes.id, NOT scn-*
    text: s.content ?? "",
    locked: s.isLocked,
    headVersionNumber: s.headVersionNumber,
    currentSceneSource: s.currentSceneSource,
    position: s.position,
    wordCount: s.wordCount,
    displayName: s.displayName,
    summary: s.summary,
  }));
}

/**
 * Preview-only version fixtures. The dialog labels them as sample data; they
 * are never persisted and never pretend to be database records.
 */
export function buildPreviewVersions(
  scene: SceneBlock,
  sampleContent: string,
): SceneVersionPreview[] {
  if (!scene.text.trim()) return [];

  return [
    {
      versionNumber: 2,
      source: "HUMAN_EDITED",
      createdAt: "2026-04-18 21:30",
      content: scene.text,
    },
    {
      versionNumber: 1,
      source: "AI_GENERATED",
      createdAt: "2026-04-18 21:12",
      content: sampleContent,
    },
  ];
}

/**
 * Review generated scene candidate (P5S5-P, P5S5-N).
 * Valid completed stream remains reviewable after request scope expires.
 * Lock-unlock cannot accept invalidated generation (throws CONFLICT_INVALIDATED).
 */
export function reviewGeneratedScene(params: {
  action: "accept" | "discard";
  candidate: SceneGenerationCandidate;
  scene: SceneBlock;
}): { accepted: boolean; updatedText?: string; source?: SceneSource } {
  const { action, candidate, scene } = params;
  if (action === "discard") {
    return { accepted: false };
  }

  // P5S5-N: Lock-unlock cannot accept invalidated generation
  if (
    !candidate.authorizationValid ||
    candidate.status === "invalidated_by_lock" ||
    scene.locked ||
    (scene.headVersionNumber !== undefined &&
      candidate.baselineVersionNumber !== scene.headVersionNumber)
  ) {
    throw new Error(
      "CONFLICT_INVALIDATED: Candidate generation has been invalidated and cannot be accepted"
    );
  }

  return {
    accepted: true,
    updatedText: candidate.text,
    source: "AI_GENERATED",
  };
}

/**
 * Scene version restore pure state transition (CI-08 / REQ-WRITE-004).
 * Never invents a 41st IPC command. Restoring a version loads its content into the
 * unlocked editor dirty buffer so it is persisted through the authoritative save_chapter (#36).
 * Locked scene restore is blocked with an explicit contract notice.
 */
export function restoreSceneVersion(
  scene: SceneBlock,
  version:
    | Pick<SceneVersionItemDto, "content" | "versionNumber">
    | Pick<SceneVersionPreview, "content" | "versionNumber">,
): { success: boolean; newText?: string; source?: SceneSource; error?: string } {
  if (scene.locked) {
    throw new Error(
      "SCENE_LOCKED_RESTORE_BLOCKED: Cannot restore historical version into a locked scene"
    );
  }

  return {
    success: true,
    newText: version.content,
    source: "RESTORE",
  };
}

// ============================================================================
// Durable Service Adapters using BackendClient (Commands #29..#37)
// ============================================================================

/**
 * Loads chapter writing read model (#29 get_chapter_writing_state).
 * Enforces Committed-Plan Gate (WR-01).
 */
export async function loadChapterWritingReadModel(
  client: BackendClient,
  req: { projectId: string; chapterId: string },
): Promise<ChapterWritingReadModelDto> {
  return client.getChapterWritingState(req);
}

/**
 * Persists scene lock state immediately (#30 set_scene_lock).
 * Exclusive UI action, executes TX-06 immediately, does NOT mark scene dirty.
 */
export async function setSceneLock(
  client: BackendClient,
  req: { projectId: string; chapterId: string; sceneId: string; desiredLocked: boolean },
): Promise<SceneLockStateDto> {
  return client.setSceneLock(req);
}

/**
 * Bulk locks/unlocks chapter scenes (#31 set_chapter_scenes_lock_bulk).
 */
export async function setChapterScenesLockBulk(
  client: BackendClient,
  req: { projectId: string; chapterId: string; isLocked: boolean },
): Promise<BulkSceneLockStateDto> {
  return client.setChapterScenesLockBulk(req);
}

/**
 * Starts scene draft generation (#32 start_scene_generation).
 * Stream completes and transitions candidate to review state (P5S5-P).
 */
export async function generateSceneDraft(
  client: BackendClient,
  req: {
    projectId: string;
    chapterId: string;
    sceneId: string;
    mode?: string;
    note?: string | null;
    currentWorkingContent?: string | null;
  },
  onEvent?: (event: SceneGenerationStreamEvent) => void,
): Promise<SceneGenerationCandidate> {
  const res = await client.startSceneGeneration(
    {
      projectId: req.projectId,
      chapterId: req.chapterId,
      sceneId: req.sceneId,
      mode: req.mode || "generate",
      note: req.note,
      currentWorkingContent: req.currentWorkingContent,
    },
    onEvent,
  );

  return {
    requestId: res.requestId,
    generationId: res.generationId,
    sceneId: res.sceneId,
    baselineVersionNumber: res.baselineVersionNumber,
    authorizationValid: res.authorizationValid,
    status: "review", // Completed stream remains in Review state (P5S5-P)
    text: res.generatedText,
    note: res.normalizedNote,
  };
}

/**
 * Cancels active scene generation (#33 cancel_scene_generation).
 */
export async function cancelSceneGeneration(
  client: BackendClient,
  req: { chapterId: string; generationId: string },
): Promise<CancelSceneGenerationResponseDto> {
  return client.cancelSceneGeneration(req);
}

/**
 * Lists scene historical versions (#35 list_scene_versions).
 * Contains full content for client-side restoration (CI-08).
 */
export async function listSceneVersions(
  client: BackendClient,
  req: { projectId: string; chapterId: string; sceneId: string },
): Promise<SceneVersionListDto> {
  return client.listSceneVersions(req);
}

/**
 * Saves chapter scenes atomically with OCC (#36 save_chapter).
 * P5S5-B: Only dirty scenes are submitted; locked clean scenes do NOT block saving.
 * Reject entire save if any dirty scene is locked (LOCK-05).
 */
export async function saveChapterScenes(
  client: BackendClient,
  req: SaveChapterScenesReq,
): Promise<SaveChapterResponse> {
  for (const scene of req.dirtyScenes) {
    if (scene.isLocked) {
      throw new Error("SceneLocked: Cannot save locked scene with dirty content");
    }
  }

  const dirtyScenesPayload: DirtySceneSaveItem[] = req.dirtyScenes.map((s) => ({
    sceneId: s.sceneId,
    expectedVersionNumber: s.expectedVersionNumber ?? req.expectedChapterRevision ?? 0,
    content: s.content,
    sceneSource: (s.sceneSource === "AI_GENERATED" ? "AI" : s.sceneSource) || "USER",
    summary: s.summary ?? null,
  }));

  return client.saveChapter({
    projectId: req.projectId,
    chapterId: req.chapterId,
    clientSaveId: req.clientSaveId,
    dirtyScenes: dirtyScenesPayload,
  });
}

/**
 * Looks up chapter save receipt (#37 get_chapter_save_receipt).
 * Used for lost save response reconciliation (Q4.7 / phase5_sprint5_deep3-F2).
 */
export async function getChapterSaveReceipt(
  client: BackendClient,
  req: { chapterId: string; clientSaveId: string },
): Promise<SaveReceiptLookupDto> {
  return client.getChapterSaveReceipt(req);
}

/**
 * Reconciles lost save response via receipt lookup (phase5_sprint5_deep3-F2).
 * Reuses the SAME clientSaveId and never blindly overwrites.
 */
export async function executeSaveWithReceiptReconciliation(
  client: BackendClient,
  req: SaveChapterScenesReq,
): Promise<{
  success: boolean;
  response: SaveChapterResponse;
  reconciledFromReceipt: boolean;
}> {
  try {
    const res = await saveChapterScenes(client, req);
    return { success: true, response: res, reconciledFromReceipt: false };
  } catch (err) {
    // phase5_sprint5_deep3-F2: Query receipt with the SAME clientSaveId
    try {
      const receiptLookup = await getChapterSaveReceipt(client, {
        chapterId: req.chapterId,
        clientSaveId: req.clientSaveId,
      });
      if (receiptLookup && receiptLookup.found && receiptLookup.receipt) {
        return {
          success: true,
          response: receiptLookup.receipt,
          reconciledFromReceipt: true,
        };
      }
    } catch {
      // Propagate original error if receipt lookup also fails
    }
    throw err;
  }
}
