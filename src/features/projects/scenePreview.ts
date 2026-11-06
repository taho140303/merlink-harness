import type { ChapterBlock, ChapterItem } from "./types";

export type SceneSource = "AI_GENERATED" | "HUMAN_EDITED" | "RESTORE";

export interface SceneBlock extends ChapterBlock {
  /** One scene per block. Preview data derives this from the chapter and block id. */
  sceneId: string;
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
