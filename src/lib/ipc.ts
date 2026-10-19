import type { Channel } from "@tauri-apps/api/core";
import type {
  AgentEvent, ChatTurn, ContextUsage, FetchedModel, MessageEnvelope,
  ProviderConfig, SuperJsonProgress,
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
  super_json_read: { args: { projectId: string }; result: unknown };
  super_json_progress: { args: { sessionId: string }; result: SuperJsonProgress };
  super_json_summary: { args: { projectId: string }; result: string };
  super_json_labels: { args: never; result: Record<string, string> };
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
