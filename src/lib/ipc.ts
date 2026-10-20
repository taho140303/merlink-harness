import type { Channel } from "@tauri-apps/api/core";
import type {
  AgentEvent, ChatTurn, ContextUsage, FetchedModel, MessageEnvelope,
  OutlineAnchorResult, OutlineStatusResult,
  ProjectSummary, ProviderConfig, SessionSummary, SuperJsonProgress,
} from "../types";
import { safeInvoke, type AgentEventChannel } from "./tauri";

export interface SendChatArgs {
  providerId: string;
  modelId: string;
  messages: MessageEnvelope[];
  projectId: string;
  sessionId: string;
  channel: AgentEventChannel | Channel<AgentEvent>;
}

/** Command names, lower-camel-case IPC arguments, and returned wire shapes. */
export interface CommandMap {
  load_providers: { args: never; result: ProviderConfig[] };
  save_providers: { args: { providers: ProviderConfig[] }; result: void };
  send_chat: { args: SendChatArgs; result: ChatTurn };
  list_messages: { args: { sessionId: string }; result: MessageEnvelope[] };
  list_projects: { args: never; result: ProjectSummary[] };
  get_project: { args: { projectId: string }; result: ProjectSummary | null };
  list_sessions: { args: never; result: SessionSummary[] };
  get_session: { args: { sessionId: string }; result: SessionSummary | null };
  rename_session: { args: { sessionId: string; title: string }; result: SessionSummary };
  delete_session: { args: { sessionId: string }; result: boolean };
  super_json_read: { args: { projectId: string }; result: unknown };
  super_json_progress: { args: { sessionId: string }; result: SuperJsonProgress };
  super_json_summary: { args: { projectId: string }; result: string };
  super_json_labels: { args: never; result: Record<string, string> };
  outline_anchor: { args: { projectId: string }; result: OutlineAnchorResult };
  outline_status: { args: { sessionId: string }; result: OutlineStatusResult };
  outline_start: { args: { projectId: string; sessionId: string }; result: OutlineStatusResult };
  outline_run: { args: { sessionId: string; providerId: string; modelId: string; stage: string; range?: string; channel: Channel<unknown> }; result: unknown };
  outline_items: { args: { sessionId: string; stage: string }; result: unknown[] };
  outline_regenerate: { args: { sessionId: string; providerId: string; modelId: string; stage: string; key: string; note: string }; result: unknown };
  outline_commit: { args: { sessionId: string; stage: string; keys?: string[] }; result: unknown };
  outline_accept: { args: { sessionId: string; stage: string; keys?: string[] }; result: number };
  outline_reject: { args: { sessionId: string; stage: string; keys?: string[] }; result: number };
  outline_cancel: { args: { sessionId: string }; result: void };
  context_usage: { args: { sessionId: string; providerId: string; modelId: string }; result: ContextUsage };
  fetch_models: { args: { providerKind: string; baseUrl: string; apiKey: string }; result: FetchedModel[] };
  execute_tool: { args: { toolName: string; args: Record<string, unknown>; projectId: string; sessionId: string }; result: unknown };
}

type NoArgCommand = { [K in keyof CommandMap]: CommandMap[K]["args"] extends never ? K : never }[keyof CommandMap];
type ArgCommand = Exclude<keyof CommandMap, NoArgCommand>;

export function invokeCommand<K extends NoArgCommand>(cmd: K): Promise<CommandMap[K]["result"]>;
export function invokeCommand<K extends ArgCommand>(cmd: K, args: CommandMap[K]["args"]): Promise<CommandMap[K]["result"]>;
export function invokeCommand<K extends keyof CommandMap>(cmd: K, args?: CommandMap[K]["args"]): Promise<CommandMap[K]["result"]> {
  return safeInvoke<CommandMap[K]["result"]>(cmd, args);
}
