export type ChatMode = "chat" | "manual";

export interface ToolCallItem {
  id: string;
  name: string;
  parameters: Record<string, any>;
  result?: Record<string, any>;
  executionTimeMs?: number;
  status: "success" | "error" | "pending";
}

export interface ChatMessage {
  id: string;
  role: "user" | "assistant";
  content: string;
  thinkingSeconds?: number;
  thinkingDetails?: string;
  isThinking?: boolean;
  toolCalls?: ToolCallItem[];
  timestamp: string;
}
