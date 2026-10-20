import { invokeCommand } from "./ipc";
import type { SceneRef, WritingStatus, WriteReport } from "../types";

/** Scene writing for phase 3: one file per scene, versions kept on rewrite. */
export const getWritingStatus = (projectId: string): Promise<WritingStatus> =>
  invokeCommand("writing_status", { projectId });

export const writeScene = (
  projectId: string,
  providerId: string,
  modelId: string,
  chapter: number,
  position: number,
  note?: string,
): Promise<WriteReport> =>
  invokeCommand("write_scene", { projectId, providerId, modelId, chapter, position, note });

export const rewriteScenes = (
  projectId: string,
  providerId: string,
  modelId: string,
  scenes: SceneRef[],
  note?: string,
): Promise<WriteReport[]> =>
  invokeCommand("rewrite_scenes", { projectId, providerId, modelId, scenes, note });

export const lockScene = (
  projectId: string,
  chapter: number,
  position: number,
  locked: boolean,
): Promise<void> =>
  invokeCommand("lock_scene", { projectId, chapter, position, locked });
