import { Channel, invoke } from "@tauri-apps/api/core";
import type { AgentEvent } from "../types";
import { browserInvoke } from "./browser-mock";

declare global {
  interface Window {
    __TAURI_INTERNALS__?: unknown;
  }
}

export function isTauri(): boolean {
  return typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
}

/** The minimal shape the browser mock needs from a channel. */
export interface AgentEventChannel {
  onmessage: (event: AgentEvent) => void;
}

/** Tauri's live event channel, or a plain callback in browser preview. */
export function createAgentEventChannel(
  handler: (event: AgentEvent) => void,
): AgentEventChannel | Channel<AgentEvent> {
  if (isTauri()) {
    const channel = new Channel<AgentEvent>();
    channel.onmessage = handler;
    return channel;
  }
  return { onmessage: handler };
}

/** Low-level transport; callers use the typed invokeCommand boundary in ipc.ts. */
export async function safeInvoke<T>(cmd: string, args?: unknown): Promise<T> {
  if (isTauri()) return invoke<T>(cmd, args as Record<string, unknown> | undefined);
  return browserInvoke(cmd, args) as Promise<T>;
}
