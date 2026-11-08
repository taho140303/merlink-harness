/**
 * Frontend backend integration types and shim interfaces.
 * Re-exports frozen DTOs from `./generated` and provides FE transport/adapter contracts.
 *
 * DO NOT EDIT `generated.ts`; this file defines FE view models, transport shims,
 * and error types that consume generated bindings.
 */

import type {
  ErrorCode,
  IpcError,
  ToolRejection,
  IpcCommandName,
} from "./generated";

export * from "./generated";

/**
 * Transport availability status across SSR and Tauri desktop runtime.
 */
export type TransportStatus = "available" | "unavailable";

/**
 * Cleanup subscription handle for streaming channels and event listeners.
 */
export interface StreamSubscription {
  unsubscribe: () => void;
}

/**
 * Channel callback adapter compatible with Tauri v2 Channel<T>.
 */
export interface ChannelAdapter<T> {
  onEvent: (event: T) => void;
  channelId?: number | string;
}

/**
 * Pluggable backend transport interface allowing dependency injection (DI).
 * Allows test suites to inject fake typed transports while desktop runs Tauri IPC.
 */
export interface BackendTransport {
  readonly status: TransportStatus;
  getStatus(): TransportStatus;
  invoke<T = unknown, R = unknown>(cmd: IpcCommandName, req?: T): Promise<R>;
  createChannel<T = unknown>(
    onEvent: (event: T) => void
  ): ChannelAdapter<T> | string | number | ((event: T) => void);
}

/**
 * Explicit error raised when backend transport is unavailable (e.g. SSR, browser, disconnected).
 * Functional requirement: NEVER return fake mock successful data when transport is unavailable.
 */
export class BackendUnavailableError extends Error {
  public readonly status = "unavailable" as const;
  public readonly code: ErrorCode = "provider_unreachable";
  public readonly retryable = false;

  constructor(
    message: string = "Backend transport unavailable in current environment (requires desktop Tauri runtime)"
  ) {
    super(message);
    this.name = "BackendUnavailableError";
    Object.setPrototypeOf(this, new.target.prototype);
  }
}

/**
 * Structured typed IPC error with sanitized details and standard error code.
 */
export class TypedIpcError extends Error implements IpcError {
  public readonly code: ErrorCode;
  public readonly details?: Record<string, any> | null;
  public readonly retryable: boolean;

  constructor(error: IpcError) {
    super(error.message);
    this.name = "TypedIpcError";
    this.code = error.code;
    this.details = error.details ?? null;
    this.retryable = Boolean(error.retryable);
    Object.setPrototypeOf(this, new.target.prototype);
  }
}

/**
 * Business rejection signal raised during AI Tool execution (`be-core.md` §1.5).
 */
export class ToolRejectionError extends Error {
  public readonly status = "rejected" as const;
  public readonly input?: Record<string, any> | null;
  public readonly candidates?: string[] | null;

  constructor(rejection: ToolRejection) {
    super(rejection.error);
    this.name = "ToolRejectionError";
    this.input = rejection.input ?? null;
    this.candidates = rejection.candidates ?? null;
    Object.setPrototypeOf(this, new.target.prototype);
  }
}

/**
 * Provenance tracking for user/AI authoring.
 */
export type FieldSource = "user" | "user_edited" | "ai_generated";

/**
 * Character View Model: retains stable internal `code` while exposing clean `displayLabel`.
 */
export interface CharacterViewModel {
  id: string;
  code: string;
  displayName: string;
  displayLabel: string;
  source: FieldSource;
  nickname?: string | null;
  species: string;
  biologicalSex: "NONE" | "MALE" | "FEMALE" | string;
  genderIdentity: string;
  genderExpression: string;
  inProject: boolean;
  avatarUrl?: string | null;
  defaultSummary?: string | null;
}

/**
 * Scene View Model: keeps stable `code` and `source` while hiding internal code from UI display.
 */
export interface SceneViewModel {
  id: string;
  code: string;
  chapterId: string;
  position: number;
  displayName: string;
  displayLabel: string;
  source: FieldSource;
  summary?: string | null;
  content?: string | null;
  wordCount: number;
  isLocked: boolean;
  headVersionNumber: number;
  currentSceneSource: "AI" | "USER" | "RESTORE" | string;
}

/**
 * Chapter View Model with nested scenes.
 */
export interface ChapterViewModel {
  id: string;
  code: string;
  displayName: string;
  displayLabel: string;
  source: FieldSource;
  position: number;
  date?: string;
  isNew?: boolean | null;
  scenes: SceneViewModel[];
}

/**
 * Arc View Model with nested chapters.
 */
export interface ArcViewModel {
  id: string;
  code: string;
  displayName: string;
  displayLabel: string;
  source: FieldSource;
  cover?: string | null;
  chapters: ChapterViewModel[];
}

/**
 * Draft Attribute View Model with 46-attribute progress tracking.
 */
export interface DraftAttributeViewModel {
  code: string;
  displayName: string;
  displayLabel: string;
  phase: number;
  isRequired: boolean;
  status: "complete" | "incomplete" | "conflict" | string;
  currentValueSummary?: string | null;
  source: FieldSource;
}

/**
 * Project View Model: clean UI DTO matching project overview.
 */
export interface ProjectViewModel {
  id: string;
  title: string;
  displayLabel: string;
  status: "Drafting" | "Completed" | "In Review" | "Ongoing" | "Planning" | string;
  cover: string;
  description: string;
  genres: string[];
  modifiedAt: string;
  details: {
    wordCount: string;
    chapters: string;
    characters: string;
    worldLore: string;
    createdDate: string;
  };
  arcs?: ArcViewModel[] | null;
}

/**
 * Provider Configuration View Model: hides raw secrets from UI rendering.
 */
export interface ProviderConfigViewModel {
  providerId: string;
  name: string;
  displayLabel: string;
  isActive: boolean;
  hasApiKey: boolean;
  endpointUrl?: string | null;
  modelName?: string | null;
}
