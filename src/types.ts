export interface InputTypes {
  text: boolean;
  image: boolean;
}

export interface ModelConfig {
  id: string;
  name: string;
  context_window: string;
  max_tokens: string;
  input_types: InputTypes;
  reasoning_level: boolean;
}

export type ProviderKind = "openai" | "anthropic" | "custom";

export interface ProviderConfig {
  id: string;
  name: string;
  kind: string; // IPC/browser mock still produces unconstrained strings; editor narrows choices.
  api_key: string;
  base_url: string;
  models: ModelConfig[];
  default_model_id: string;
}

/**
 * Mirrors `FetchedModel` in src-tauri/src/models.rs (serde snake_case).
 *
 * One model as reported by the provider's own model-list endpoint. The fetch
 * itself happens in Rust (`fetch_models`) so the API key never reaches the
 * webview and no CORS preflight is needed (Sprint 11.5 · D1/D2).
 */
export interface FetchedModel {
  id: string;
  name: string;
}

/** Mirrors `TokenUsage` in src-tauri/src/models.rs (serde snake_case). */
export interface TokenUsage {
  input_tokens: number;
  output_tokens: number;
}

export interface ProjectSummary {
  id: string;
  title: string;
  genres: string[];
  created_at: string;
  session_count: number;
}

export interface SessionSummary {
  id: string;
  project_id: string;
  project_title: string | null;
  title: string;
  created_at: string;
}

export interface MessageMeta {
  model: string;
  provider: string;
  session_id: string;
  project_id: string;
  prompt_template_id: string;
  prompt_hash: string;
  tokens: TokenUsage;
  latency_ms: number;
  created_at: string;
}

/** Mirrors `ToolCall` in src-tauri/src/models.rs. */
export interface ToolCall {
  id: string;
  name: string;
  /** Free-form JSON: `serde_json::Value` on the Rust side. */
  arguments: unknown;
}

export interface MessageEnvelope {
  id: string;
  role: "user" | "assistant" | "system" | "tool";
  content: string;
  meta?: MessageMeta;
  /** Set on assistant messages that request tools. */
  tool_calls?: ToolCall[];
  /** Set on `role: "tool"` messages, pairing them with the request. */
  tool_call_id?: string;
  /** Tool name, for `role: "tool"` messages. */
  name?: string;
  /**
   * The model's own thinking, on the final assistant message of a turn
   * (Sprint 11.12 · D9). Display only; never sent back to the model.
   */
  reasoning?: string;
}

/**
 * Mirrors `ChatTurn` in src-tauri/src/models.rs.
 *
 * `project_id` is the project the session belongs to *after* the turn, so the
 * first message of a fresh chat can report the project it just created instead of
 * the frontend guessing one.
 */
export interface ChatTurn {
  project_id: string;
  messages: MessageEnvelope[];
}

/**
 * Mirrors `SuperJsonProgress` in src-tauri/src/models.rs (serde snake_case).
 *
 * The 12 required units cover 0 → 70% and the 35 optional units cover 70 → 100%.
 * `required_total`/`optional_total` are always 12/35; only the `filled_*` counts
 * drop to zero while the chat has no project.
 */
export interface ContextUsage {
  has_model: boolean; total_tokens: number; used_tokens: number; percent: number;
  system_tokens: number; tool_tokens: number; message_tokens: number;
  measured_input_tokens: number | null;
}

export interface OutlineAnchorResult {
  project_id: string;
  text: string;
  json: {
    project: {
      name: string | null;
      language: string | null;
      genre: string | null;
      genre_code: string | null;
      plan: {
        word: number | null;
        arc: number | null;
        chapter: number | null;
        scene: number | null;
      };
    };
    arcs: Array<{
      id: string;
      name: string | null;
      sequence: number | null;
      goal: string | null;
      outcome: string | null;
    }>;
    protagonist: Record<string, unknown> | null;
    cast: Array<Record<string, unknown>>;
    motif: Record<string, unknown> | null;
    theme: Record<string, unknown> | null;
    argument: Record<string, unknown> | null;
  };
}

export interface OutlineStatusResult {
  ok: boolean;
  stage: string;
  active: boolean;
  note: string;
  project_id: string;
}


export interface SuperJsonProgress {
  has_project: boolean;
  project_id: string;
  percent: number;
  label: string;
  filled_required: number;
  required_total: number;
  filled_optional: number;
  optional_total: number;
}

/**
 * Mirrors `AgentEvent` in src-tauri/src/models.rs.
 *
 * The Rust enum is serialised with an internal `type` tag (`serde(tag = "type",
 * rename_all = "snake_case")`), so this is a discriminated union the UI can
 * switch on. Events arrive over a Tauri `Channel` *while* the turn runs — they
 * are not buffered until `send_chat` resolves (D1/D2).
 */
export type AgentEvent =
  | { type: "turn_started"; session_id: string; round_budget: number }
  | { type: "round_started"; round: number }
  | {
      type: "tool_started";
      id: string;
      name: string;
      /** `null` for tools that do not address a super json path. */
      path: string | null;
      status: string;
    }
  | {
      type: "tool_finished";
      id: string;
      name: string;
      ok: boolean;
      duration_ms: number;
      status: string;
    }
  | { type: "round_finished"; round: number; duration_ms: number; tool_calls: number }
  | { type: "turn_finished"; project_id: string; duration_ms: number; rounds: number };

/* ------------------------------------------------------------------ */
/* Sample workspace data (mirrors the mock export in ref-ui/shared.js) */
/* ------------------------------------------------------------------ */

export interface Chapter {
  id: string;
  title: string;
  wordCount: number;
  status: string;
  content: string[];
}

export interface Project {
  id: string;
  title: string;
  genre: string;
  motifs: string[];
  cover: string;
  status: string;
  updatedAt: string;
  wordCount: number;
  description: string;
  chapters: Chapter[];
  author: string;
  createdAt: string;
}

export interface ChatMessage {
  id: string;
  role: string;
  content: string;
  time: string;
}

export interface Chat {
  id: string;
  title: string;
  projectId: string;
  updatedAt: string;
  messages: ChatMessage[];
}

export interface Relationship {
  name: string;
  relation: string;
}

export interface Character {
  id: string;
  name: string;
  role: string;
  /** Mock records use numbers, newly created records use strings. */
  age: string | number;
  gender: string;
  /** Project id this character belongs to. */
  story: string;
  tags: string[];
  appearance?: string;
  bio: string;
  backstory: string;
  /** May be plain strings in older saved data. */
  relationships: Array<Relationship | string>;
  dialogueCount: number;
  /** Base64 data URL chosen by the author. */
  uploadedImage?: string;
  /** Gradient artwork string used by the sample characters. */
  image?: string;
}
