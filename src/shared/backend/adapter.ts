/**
 * Pure mapping functions and DTO adapters between Backend DTOs (`generated.ts`)
 * and Frontend domain models.
 *
 * Implements:
 * - Pure snake_case <-> camelCase transformations
 * - Secret redaction and error normalization
 * - Provenance preservation and code-hiding display formatting
 * - Streaming event sequence validation and reducers
 */

import type {
  ErrorCode,
  IpcError,
  CharacterLibraryItemDto,
  SceneReadModelDto,
  ChapterItemUiDto,
  ArcItemUiDto,
  ProjectItemUiDto,
  AttributeEvaluationDetailDto,
  DraftWorkspaceStateDto,
  ChatMessageDto,
  WorkflowExecutionSummaryDto,
  ProviderStatusDto,
  ChatStreamEvent,
  WorkflowProgressEvent,
  SceneGenerationStreamEvent,
  GenerationStreamStatus,
} from "./generated";
import { IPC_COMMAND_NAMES } from "./generated";
import {
  TypedIpcError,
  BackendUnavailableError,
  type FieldSource,
  type CharacterViewModel,
  type SceneViewModel,
  type ChapterViewModel,
  type ArcViewModel,
  type DraftAttributeViewModel,
  type ProjectViewModel,
  type ProviderConfigViewModel,
} from "./types";

// ============================================================================
// 1. Case Conversions (Pure snake_case <-> camelCase)
// ============================================================================

/**
 * Converts a snake_case string to camelCase.
 */
export function snakeToCamel(str: string): string {
  return str.replace(/([-_][a-z0-9])/gi, (group) =>
    group.toUpperCase().replace("-", "").replace("_", "")
  );
}

/**
 * Converts a camelCase string to snake_case.
 */
export function camelToSnake(str: string): string {
  return str.replace(/[A-Z]/g, (letter) => `_${letter.toLowerCase()}`).replace(/^_/, "");
}

/**
 * Recursively converts all object keys from snake_case to camelCase.
 */
export function keysToCamelCase<T = any>(obj: any): T {
  if (obj === null || obj === undefined || typeof obj !== "object") {
    return obj;
  }
  if (obj instanceof Date || obj instanceof RegExp || obj instanceof Error) {
    return obj as unknown as T;
  }
  if (Array.isArray(obj)) {
    return obj.map((item) => keysToCamelCase(item)) as unknown as T;
  }

  const result: Record<string, any> = {};
  for (const key of Object.keys(obj)) {
    const camelKey = snakeToCamel(key);
    result[camelKey] = keysToCamelCase(obj[key]);
  }
  return result as T;
}

/**
 * Recursively converts all object keys from camelCase to snake_case.
 */
export function keysToSnakeCase<T = any>(obj: any): T {
  if (obj === null || obj === undefined || typeof obj !== "object") {
    return obj;
  }
  if (obj instanceof Date || obj instanceof RegExp || obj instanceof Error) {
    return obj as unknown as T;
  }
  if (Array.isArray(obj)) {
    return obj.map((item) => keysToSnakeCase(item)) as unknown as T;
  }

  const result: Record<string, any> = {};
  for (const key of Object.keys(obj)) {
    const snakeKey = camelToSnake(key);
    result[snakeKey] = keysToSnakeCase(obj[key]);
  }
  return result as T;
}

// ============================================================================
// 2. Secret Redaction & Sanitization
// ============================================================================

const SENSITIVE_PATTERNS: RegExp[] = [
  // OpenAI / generic API keys
  /sk-[A-Za-z0-9_-]{8,}/g,
  // Bearer tokens
  /Bearer\s+[A-Za-z0-9._-]{8,}/gi,
  // Key / secret / token assignments in JSON / text
  /(["']?(?:api[_-]?key|secret|token|password|credential)["']?\s*[:=]\s*["'])([^"'\s,;]+)(["'])/gi,
];

/**
 * Strips secret tokens and passwords from strings.
 */
export function redactSensitiveString(text: string): string {
  if (!text || typeof text !== "string") return text;
  let sanitized = text;

  // Mask known API key patterns
  sanitized = sanitized.replace(/sk-[A-Za-z0-9_-]{8,}/g, "sk-***[REDACTED]***");
  sanitized = sanitized.replace(/Bearer\s+[A-Za-z0-9._-]{8,}/gi, "Bearer [REDACTED]");

  // Mask key=value assignments
  for (const pattern of SENSITIVE_PATTERNS) {
    sanitized = sanitized.replace(
      pattern,
      (_match, prefix, _val, suffix) => `${prefix}[REDACTED]${suffix}`
    );
  }

  return sanitized;
}

const SENSITIVE_KEY_NAMES = new Set([
  "apikey",
  "api_key",
  "secret",
  "password",
  "token",
  "accesstoken",
  "access_token",
  "credential",
  "credentials",
]);

/**
 * Deeply redacts sensitive keys and values from data structures before logging or IPC serialization.
 */
export function redactSensitiveData<T = any>(data: T): T {
  if (data === null || data === undefined) return data;
  if (typeof data === "string") {
    return redactSensitiveString(data) as unknown as T;
  }
  if (typeof data !== "object") return data;
  if (data instanceof Date || data instanceof RegExp) return data;

  if (Array.isArray(data)) {
    return data.map((item) => redactSensitiveData(item)) as unknown as T;
  }

  const result: Record<string, any> = {};
  for (const [key, value] of Object.entries(data)) {
    const lowerKey = key.toLowerCase().replace(/[-_]/g, "");
    if (SENSITIVE_KEY_NAMES.has(lowerKey)) {
      result[key] = "[REDACTED]";
    } else {
      result[key] = redactSensitiveData(value);
    }
  }
  return result as T;
}

// ============================================================================
// 3. Error Normalization (`normalizeBackendError`)
// ============================================================================

const KNOWN_ERROR_CODES: Set<string> = new Set([
  "draft_busy",
  "draft_not_editable",
  "draft_reopen_forbidden",
  "pending_patches_exist",
  "validation_failed",
  "code_collision_on_commit",
  "version_seal_failed",
  "seal_key_payload_mismatch",
  "idempotency_key_payload_mismatch",
  "stale_workflow_receipt",
  "scene_locked",
  "out_of_edit_scope",
  "generation_invalidated_by_lock",
  "occ_version_mismatch",
  "unauthorized_caller",
  "entity_not_found",
  "provider_unreachable",
  "provider_auth_failed",
  "unsupported_schema_version",
  "migration_failed",
  "database_error",
  "internal_error",
  "workflow_baseline_unavailable",
  "evidence_codec_unsupported",
  "unsupported_evidence_codec",
  "unauthorized_transition",
  "patch_precondition_failed",
  "patch_expired",
  "proposal_eligibility_conflict",
  "proposal_eligibility_indeterminate",
  "artifact_budget_unavailable",
  "artifact_budget_exceeded",
  "workflow_invariant_violation",
  "confirmation_not_authorized",
  "requires_replan",
]);

/**
 * Normalizes any caught error into a safe, redacted `TypedIpcError` or `BackendUnavailableError`.
 * Strips raw secrets (`sk-...`, `Bearer ...`) from message and details,
 * and maps unrecognized errors to safe `internal_error` or `provider_unreachable`.
 */
export function normalizeBackendError(
  error: unknown
): TypedIpcError | BackendUnavailableError {
  if (error instanceof BackendUnavailableError) {
    return error;
  }

  if (error instanceof TypedIpcError) {
    return new TypedIpcError({
      code: error.code,
      message: redactSensitiveString(error.message),
      details: redactSensitiveData(error.details ?? null),
      retryable: error.retryable,
    });
  }

  if (typeof error === "object" && error !== null) {
    const obj = error as Record<string, any>;
    const rawCode = typeof obj.code === "string" ? obj.code : "";
    const code: ErrorCode = KNOWN_ERROR_CODES.has(rawCode)
      ? (rawCode as ErrorCode)
      : "internal_error";

    const rawMessage = typeof obj.message === "string" ? obj.message : "Unknown backend error";
    const message = redactSensitiveString(rawMessage);
    const details = redactSensitiveData(obj.details ?? null);
    const retryable = Boolean(obj.retryable);

    return new TypedIpcError({
      code,
      message,
      details,
      retryable,
    });
  }

  // String or unknown primitive error
  const rawMsg = typeof error === "string" ? error : String(error);
  return new TypedIpcError({
    code: "internal_error",
    message: redactSensitiveString(rawMsg),
    details: null,
    retryable: false,
  });
}

// ============================================================================
// 4. UI Display Label Helper (Code-Hiding Invariant)
// ============================================================================

/**
 * Regex patterns identifying raw internal code prefixes:
 * Examples: `[CHAR_001] Alice` -> `Alice`
 *           `ARC_01: Hồi 1` -> `Hồi 1`
 *           `SCN_05 - Gặp gỡ` -> `Gặp gỡ`
 */
const CODE_PREFIX_REGEX = /^(\[[A-Za-z0-9_-]+\]\s*|[A-Za-z0-9_-]+(?::|\s+[-–—])\s*)/;

/**
 * Returns a human-friendly display label, stripping raw internal codes (`CHAR_...`, `ARC_...`).
 * Preserves the internal code on the data layer while keeping the UI clean.
 */
export function formatDisplayLabel(nameOrTitle?: string | null, code?: string): string {
  if (!nameOrTitle || typeof nameOrTitle !== "string") {
    return "Chưa đặt tên";
  }

  let label = nameOrTitle.trim();

  // If a specific code was passed and appears as a prefix, strip it
  if (code && typeof code === "string" && code.length > 0) {
    const escaped = code.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
    const codeRegex = new RegExp(`^(?:\\[${escaped}\\]|${escaped}(?::|\\s+[-–—]))\\s*`, "i");
    label = label.replace(codeRegex, "");
  }

  // Strip general technical prefix patterns
  label = label.replace(CODE_PREFIX_REGEX, "");

  return label.trim() || nameOrTitle.trim();
}

/**
 * Maps raw backend scene source ("AI" | "USER" | "RESTORE") to FE `FieldSource`.
 */
export function mapSceneSource(
  rawSource?: string | null,
  fallback: FieldSource = "user"
): FieldSource {
  if (!rawSource) return fallback;
  const upper = rawSource.toUpperCase();
  if (upper === "AI") return "ai_generated";
  if (upper === "USER" || upper === "RESTORE") return "user";
  return fallback;
}

// ============================================================================
// 5. Domain View-Model Adapters
// ============================================================================

/**
 * Transforms Character DTO into CharacterViewModel with hidden code label.
 */
export function toCharacterViewModel(
  dto: CharacterLibraryItemDto,
  source: FieldSource = "user"
): CharacterViewModel {
  return {
    id: dto.id,
    code: dto.code,
    displayName: dto.name,
    displayLabel: formatDisplayLabel(dto.name, dto.code),
    source,
    nickname: dto.nickname ?? null,
    species: dto.species,
    biologicalSex: dto.biologicalSex,
    genderIdentity: dto.genderIdentity,
    genderExpression: dto.genderExpression,
    inProject: dto.inProject,
    avatarUrl: dto.avatarUrl ?? null,
    defaultSummary: dto.defaultSummary ?? null,
  };
}

/**
 * Transforms Scene Read Model DTO to SceneViewModel.
 */
export function toSceneViewModel(
  dto: SceneReadModelDto,
  sourceOverride?: FieldSource
): SceneViewModel {
  const source = sourceOverride ?? mapSceneSource(dto.currentSceneSource);
  return {
    id: dto.id,
    code: dto.id,
    chapterId: dto.chapterId,
    position: dto.position,
    displayName: dto.displayName,
    displayLabel: formatDisplayLabel(dto.displayName, dto.id),
    source,
    summary: dto.summary ?? null,
    content: dto.content ?? null,
    wordCount: dto.wordCount,
    isLocked: dto.isLocked,
    headVersionNumber: dto.headVersionNumber,
    currentSceneSource: dto.currentSceneSource,
  };
}

/**
 * Transforms Chapter Item UI DTO to ChapterViewModel.
 */
export function toChapterViewModel(
  dto: ChapterItemUiDto,
  source: FieldSource = "user"
): ChapterViewModel {
  const scenes: SceneViewModel[] = (dto.blocks ?? []).map((b, idx) => ({
    id: b.id,
    code: b.sceneId ?? b.id,
    chapterId: dto.id,
    position: idx + 1,
    displayName: `Cảnh ${idx + 1}`,
    displayLabel: `Cảnh ${idx + 1}`,
    source,
    content: b.text,
    wordCount: b.text ? b.text.trim().split(/\s+/).length : 0,
    isLocked: b.locked,
    headVersionNumber: 1,
    currentSceneSource: "USER",
  }));

  return {
    id: dto.id,
    code: dto.id,
    displayName: dto.title,
    displayLabel: formatDisplayLabel(dto.title, dto.id),
    source,
    position: 1,
    date: dto.date,
    isNew: dto.isNew ?? false,
    scenes,
  };
}

/**
 * Transforms Arc Item UI DTO to ArcViewModel.
 */
export function toArcViewModel(
  dto: ArcItemUiDto,
  source: FieldSource = "user"
): ArcViewModel {
  return {
    id: dto.id,
    code: dto.id,
    displayName: dto.title,
    displayLabel: formatDisplayLabel(dto.title, dto.id),
    source,
    cover: dto.cover ?? null,
    chapters: dto.chapters.map((c) => toChapterViewModel(c, source)),
  };
}

/**
 * Transforms Project Item UI DTO to ProjectViewModel.
 */
export function toProjectViewModel(dto: ProjectItemUiDto): ProjectViewModel {
  return {
    id: dto.id,
    title: dto.title,
    displayLabel: formatDisplayLabel(dto.title, dto.id),
    status: dto.status,
    cover: dto.cover,
    description: dto.description,
    genres: dto.genres,
    modifiedAt: dto.modifiedAt,
    details: {
      wordCount: dto.details.wordCount,
      chapters: dto.details.chapters,
      characters: dto.details.characters,
      worldLore: dto.details.worldLore,
      createdDate: dto.details.createdDate,
    },
    arcs: dto.arcs ? dto.arcs.map((a) => toArcViewModel(a)) : null,
  };
}

/**
 * Transforms Attribute Evaluation Detail DTO to DraftAttributeViewModel.
 */
export function toDraftAttributeViewModel(
  dto: AttributeEvaluationDetailDto
): DraftAttributeViewModel {
  const source: FieldSource =
    dto.source === "ai" || dto.source === "ai_generated"
      ? "ai_generated"
      : dto.source === "user_edited"
      ? "user_edited"
      : "user";

  return {
    code: dto.code,
    displayName: dto.displayName,
    displayLabel: formatDisplayLabel(dto.displayName, dto.code),
    phase: dto.phase,
    isRequired: dto.isRequired,
    status: dto.status,
    currentValueSummary: dto.currentValueSummary ?? null,
    source,
  };
}

/**
 * Transforms Provider Status DTO to ProviderConfigViewModel.
 */
export function toProviderConfigViewModel(dto: ProviderStatusDto): ProviderConfigViewModel {
  return {
    providerId: dto.providerId,
    name: dto.name,
    displayLabel: formatDisplayLabel(dto.name, dto.providerId),
    isActive: dto.isActive,
    hasApiKey: dto.hasApiKey,
    endpointUrl: dto.endpointUrl ?? null,
    modelName: dto.modelName ?? null,
  };
}

// ============================================================================
// 6. Round-Trip Mapping Adapters
// ============================================================================

export function adaptDraftToBackend(model: Record<string, any>): Record<string, any> {
  return keysToSnakeCase(model);
}

export function adaptBackendToDraft(dto: DraftWorkspaceStateDto): Record<string, any> {
  return keysToCamelCase(dto);
}

export function adaptChatToBackend(model: Record<string, any>): Record<string, any> {
  return keysToSnakeCase(model);
}

export function adaptBackendToChat(dto: ChatMessageDto): Record<string, any> {
  return keysToCamelCase(dto);
}

export function adaptWorkflowToBackend(model: Record<string, any>): Record<string, any> {
  return keysToSnakeCase(model);
}

export function adaptBackendToWorkflow(
  dto: WorkflowExecutionSummaryDto
): Record<string, any> {
  return keysToCamelCase(dto);
}

export function adaptProjectToBackend(
  model: Partial<ProjectViewModel>
): Partial<ProjectItemUiDto> {
  return keysToCamelCase(model);
}

export function adaptBackendToProject(dto: ProjectItemUiDto): ProjectViewModel {
  return toProjectViewModel(dto);
}

export function adaptCharacterToBackend(
  model: Partial<CharacterViewModel>
): Partial<CharacterLibraryItemDto> {
  return {
    id: model.id,
    code: model.code,
    name: model.displayName,
    nickname: model.nickname,
    species: model.species,
    biologicalSex: model.biologicalSex,
    genderIdentity: model.genderIdentity,
    genderExpression: model.genderExpression,
    inProject: model.inProject,
    avatarUrl: model.avatarUrl,
    defaultSummary: model.defaultSummary,
  };
}

export function adaptBackendToCharacter(
  dto: CharacterLibraryItemDto,
  source: FieldSource = "user"
): CharacterViewModel {
  return toCharacterViewModel(dto, source);
}

export function adaptWritingToBackend(
  model: Partial<SceneViewModel>
): Partial<SceneReadModelDto> {
  return {
    id: model.id,
    chapterId: model.chapterId,
    position: model.position,
    displayName: model.displayName,
    summary: model.summary,
    content: model.content,
    wordCount: model.wordCount,
    isLocked: model.isLocked,
    headVersionNumber: model.headVersionNumber,
    currentSceneSource:
      model.source === "ai_generated" ? "AI" : model.source === "user" ? "USER" : "RESTORE",
  };
}

export function adaptBackendToWriting(
  dto: SceneReadModelDto,
  source?: FieldSource
): SceneViewModel {
  return toSceneViewModel(dto, source);
}

export function adaptProviderConfigToBackend(
  model: Partial<ProviderConfigViewModel>
): Partial<ProviderStatusDto> {
  return {
    providerId: model.providerId,
    name: model.name,
    isActive: model.isActive,
    hasApiKey: model.hasApiKey,
    endpointUrl: model.endpointUrl,
    modelName: model.modelName,
  };
}

export function adaptBackendToProviderConfig(dto: ProviderStatusDto): ProviderConfigViewModel {
  return toProviderConfigViewModel(dto);
}

// ============================================================================
// 7. Streaming Event Order & Reducers
// ============================================================================

export interface ChatStreamAccumulator {
  turnId: string;
  userMessageId?: string;
  assistantMessageId?: string;
  text: string;
  toolCalls: Array<{ id: string; code: string; status: string }>;
  isComplete: boolean;
  error: IpcError | null;
}

/**
 * Validates the event sequence of ChatStreamEvent.
 * Must start with `turnStarted` and finish with `turnCompleted` or `turnError`.
 */
export function validateChatStreamSequence(
  events: ChatStreamEvent[]
): { valid: boolean; error?: string } {
  if (events.length === 0) {
    return { valid: true };
  }
  if (events[0].event !== "turnStarted") {
    return { valid: false, error: "Stream must begin with turnStarted" };
  }

  let hasEnded = false;
  for (let i = 1; i < events.length; i++) {
    if (hasEnded) {
      return { valid: false, error: "Event received after terminal turn event" };
    }
    const evt = events[i].event;
    if (evt === "turnCompleted" || evt === "turnError") {
      hasEnded = true;
    }
  }

  return { valid: true };
}

/**
 * Reducer for ChatStreamEvent into accumulated turn state.
 */
export function reduceChatStream(events: ChatStreamEvent[]): ChatStreamAccumulator {
  const acc: ChatStreamAccumulator = {
    turnId: "",
    text: "",
    toolCalls: [],
    isComplete: false,
    error: null,
  };

  for (const e of events) {
    switch (e.event) {
      case "turnStarted":
        acc.turnId = e.data.turnId;
        acc.userMessageId = e.data.userMessageId;
        break;
      case "assistantDelta":
        acc.text += e.data.delta;
        break;
      case "toolCallStarted":
        acc.toolCalls.push({
          id: e.data.toolCallId,
          code: e.data.toolCode,
          status: "started",
        });
        break;
      case "toolCallCompleted": {
        const target = acc.toolCalls.find((tc) => tc.id === e.data.toolCallId);
        if (target) {
          target.status = e.data.status;
        }
        break;
      }
      case "turnCompleted":
        acc.assistantMessageId = e.data.assistantMessageId;
        acc.isComplete = true;
        break;
      case "turnError":
        acc.error = e.data.error;
        acc.isComplete = true;
        break;
    }
  }

  return acc;
}

export interface WorkflowStreamAccumulator {
  workflowRunId: string;
  planGeneration: number;
  completedTasks: number;
  totalTasks: number;
  currentRound: number;
  isComplete: boolean;
  status: string;
}

/**
 * Validates WorkflowProgressEvent sequence.
 */
export function validateWorkflowStreamSequence(
  events: WorkflowProgressEvent[]
): { valid: boolean; error?: string } {
  if (events.length === 0) return { valid: true };
  if (events[0].event !== "workflowStarted") {
    return { valid: false, error: "Workflow stream must begin with workflowStarted" };
  }

  let hasEnded = false;
  for (let i = 1; i < events.length; i++) {
    if (hasEnded) {
      return { valid: false, error: "Event received after workflowFinished" };
    }
    if (events[i].event === "workflowFinished") {
      hasEnded = true;
    }
  }
  return { valid: true };
}

/**
 * Reducer for WorkflowProgressEvent.
 */
export function reduceWorkflowStream(
  events: WorkflowProgressEvent[]
): WorkflowStreamAccumulator {
  const acc: WorkflowStreamAccumulator = {
    workflowRunId: "",
    planGeneration: 0,
    completedTasks: 0,
    totalTasks: 0,
    currentRound: 0,
    isComplete: false,
    status: "running",
  };

  for (const e of events) {
    switch (e.event) {
      case "workflowStarted":
        acc.workflowRunId = e.data.workflowRunId;
        acc.planGeneration = e.data.planGeneration;
        acc.totalTasks = e.data.totalTasks;
        break;
      case "roundStarted":
        acc.currentRound = e.data.roundIndex;
        break;
      case "taskCompleted":
        acc.completedTasks = e.data.completedTasks;
        acc.totalTasks = e.data.totalTasks;
        break;
      case "workflowFinished":
        acc.status = e.data.status;
        acc.isComplete = true;
        break;
    }
  }
  return acc;
}

export interface SceneGenerationAccumulator {
  requestId: string;
  generationId: string;
  sceneId: string;
  text: string;
  status: GenerationStreamStatus;
  isComplete: boolean;
  error: IpcError | null;
}

/**
 * Validates SceneGenerationStreamEvent sequence.
 */
export function validateSceneGenerationStreamSequence(
  events: SceneGenerationStreamEvent[]
): { valid: boolean; error?: string } {
  if (events.length === 0) return { valid: true };
  if (events[0].event !== "started") {
    return { valid: false, error: "Scene generation must start with 'started'" };
  }

  let hasEnded = false;
  for (let i = 1; i < events.length; i++) {
    if (hasEnded) {
      return { valid: false, error: "Event received after terminal generation event" };
    }
    const evt = events[i].event;
    if (
      evt === "completed" ||
      evt === "cancelled" ||
      evt === "invalidatedByLock" ||
      evt === "failed"
    ) {
      hasEnded = true;
    }
  }
  return { valid: true };
}

/**
 * Reducer for SceneGenerationStreamEvent.
 */
export function reduceSceneGenerationStream(
  events: SceneGenerationStreamEvent[]
): SceneGenerationAccumulator {
  const acc: SceneGenerationAccumulator = {
    requestId: "",
    generationId: "",
    sceneId: "",
    text: "",
    status: "streaming",
    isComplete: false,
    error: null,
  };

  for (const e of events) {
    switch (e.event) {
      case "started":
        acc.requestId = e.data.requestId;
        acc.generationId = e.data.generationId;
        acc.sceneId = e.data.sceneId;
        acc.status = e.data.status;
        break;
      case "delta":
        acc.text += e.data.textDelta;
        break;
      case "completed":
        acc.text = e.data.fullText;
        acc.status = e.data.status;
        acc.isComplete = true;
        break;
      case "cancelled":
        acc.text = e.data.partialText;
        acc.status = e.data.status;
        acc.isComplete = true;
        break;
      case "invalidatedByLock":
        acc.text = e.data.partialText;
        acc.status = e.data.status;
        acc.isComplete = true;
        break;
      case "failed":
        acc.error = e.data.error;
        acc.status = e.data.status;
        acc.isComplete = true;
        break;
    }
  }

  return acc;
}

// Verification invariant check: master commands must have exactly 40 elements
if (IPC_COMMAND_NAMES.length !== 40) {
  throw new Error(`Master IPC command count drift: expected 40, found ${IPC_COMMAND_NAMES.length}`);
}
