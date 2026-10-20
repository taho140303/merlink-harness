import { invokeCommand } from "./ipc";
import { createAgentEventChannel } from "./tauri";
import type { OutlineEvent, OutlineItem, OutlineRunResult, OutlineStatus } from "../types";

export const getOutlineStatus = (sessionId: string): Promise<OutlineStatus> => invokeCommand("outline_status", { sessionId });
export const startOutline = (projectId: string, sessionId: string): Promise<OutlineStatus> => invokeCommand("outline_start", { projectId, sessionId });
export const loadOutlineItems = (sessionId: string, stage: string): Promise<OutlineItem[]> => invokeCommand("outline_items", { sessionId, stage });
export const acceptOutlineItems = (sessionId: string, stage: string, keys?: string[]): Promise<number> => invokeCommand("outline_accept", { sessionId, stage, keys });
export const rejectOutlineItems = (sessionId: string, stage: string, keys?: string[]): Promise<number> => invokeCommand("outline_reject", { sessionId, stage, keys });
export const commitOutline = (sessionId: string, stage: string) => invokeCommand("outline_commit", { sessionId, stage });
export const cancelOutline = (sessionId: string) => invokeCommand("outline_cancel", { sessionId });
export function runOutline(sessionId: string, providerId: string, modelId: string, stage: string, onEvent: (event: OutlineEvent) => void, range?: string): Promise<OutlineRunResult> {
  const channel = createAgentEventChannel<OutlineEvent>(onEvent);
  return invokeCommand("outline_run", { sessionId, providerId, modelId, stage, range, channel });
}
export const regenerateOutlineItem = (sessionId: string, providerId: string, modelId: string, stage: string, key: string, note: string) => invokeCommand("outline_regenerate", { sessionId, providerId, modelId, stage, key, note });
