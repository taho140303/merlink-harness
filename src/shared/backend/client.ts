/**
 * SSR-safe Tauri transport and typed client for all 40 IPC commands.
 *
 * Requirements:
 * - Detects SSR / non-desktop environments safely (`typeof window !== "undefined"`).
 * - Fails with explicit `BackendUnavailableError` ("unavailable") when Tauri is absent.
 *   TUYỆT ĐỐI KHÔNG trả về dữ liệu mock giả tạo thành công!
 * - Pluggable transport (`BackendTransport` DI) for testing with mock IPC backends.
 * - Redacts secrets and protects sensitive data from unauthorized serialization/logging.
 */

import type {
  IpcCommandName,
  // Group 1
  CreateOrLoadChatSessionReq,
  ChatSessionInitDto,
  ListChatSessionsReq,
  ChatSessionListDto,
  SendChatMessageReq,
  SendChatTurnResponseDto,
  CancelChatTurnReq,
  CancelChatTurnResponseDto,
  // Group 2
  GetDraftWorkspaceStateReq,
  DraftWorkspaceStateDto,
  GetProgressDetailReq,
  ProgressDetailDto,
  ApplyManualDraftEditReq,
  ManualEditResponseDto,
  ConfirmPhaseReq,
  PhaseConfirmResponseDto,
  RegenerateDraftPhaseReq,
  RegeneratePhaseResponseDto,
  // Group 3
  ListDraftVersionsReq,
  DraftVersionListDto,
  RestoreDraftVersionReq,
  RestoreVersionResponseDto,
  // Group 4
  ListPendingPatchesReq,
  PendingPatchListDto,
  ConfirmPendingPatchReq,
  RejectPendingPatchReq,
  PatchResolutionDto,
  ConfirmAllPendingPatchesReq,
  ConfirmAllPatchesResponseDto,
  // Group 5
  StartOrResumeWorkflowReq,
  WorkflowExecutionSummaryDto,
  CancelWorkflowReq,
  CancelWorkflowResponseDto,
  GetWorkflowStatusReq,
  WorkflowStatusDto,
  // Group 6
  GetCommitConfirmationPreviewReq,
  CommitPreviewDto,
  CommitDraftReq,
  CommitDraftResponseDto,
  ReopenCommittedDraftReq,
  ReopenCommittedDraftResponseDto,
  // Group 7
  ListProjectsReq,
  ProjectListDto,
  GetProjectDetailReq,
  ProjectDetailDto,
  DeleteProjectReq,
  DeleteProjectResponseDto,
  // Group 8
  ListCharactersReq,
  CharacterListDto,
  GetCharacterDetailReq,
  CharacterDetailDto,
  CreateOrUpdateLibraryCharacterReq,
  LinkLibraryCharacterToDraftReq,
  LinkLibraryCharacterResponseDto,
  // Group 9
  GetChapterWritingStateReq,
  ChapterWritingReadModelDto,
  SetSceneLockReq,
  SceneLockStateDto,
  SetChapterScenesLockBulkReq,
  BulkSceneLockStateDto,
  StartSceneGenerationReq,
  SceneGenerationResultDto,
  CancelSceneGenerationReq,
  CancelSceneGenerationResponseDto,
  RespondEditScopeExpansionReq,
  RespondEditScopeExpansionResponseDto,
  ListSceneVersionsReq,
  SceneVersionListDto,
  SaveChapterReq,
  SaveChapterResp,
  GetChapterSaveReceiptReq,
  SaveReceiptLookupDto,
  // Group 10
  GetRuntimeConfigReq,
  RuntimeAndProviderConfigDto,
  UpdateRuntimeConfigReq,
  SetProviderApiKeyReq,
  SetProviderApiKeyResponseDto,
  // Streaming Events
  ChatStreamEvent,
  WorkflowProgressEvent,
  SceneGenerationStreamEvent,
} from "./generated";

import {
  BackendUnavailableError,
  type BackendTransport,
  type ChannelAdapter,
  type TransportStatus,
} from "./types";
import { normalizeBackendError, redactSensitiveData } from "./adapter";

/**
 * Checks whether the current runtime environment is a Tauri desktop application.
 * SSR-safe: returns false if `window` is undefined or Tauri bridge is missing.
 */
export function isTauriEnvironment(): boolean {
  if (typeof window === "undefined") {
    return false;
  }
  const win = window as any;
  return Boolean(win.__TAURI_INTERNALS__ || win.__TAURI__ || win.__TAURI_IPC__);
}

/**
 * Default Tauri runtime transport.
 * Interacts with `window.__TAURI_INTERNALS__` or fails explicitly.
 */
class DefaultTauriTransport implements BackendTransport {
  public get status(): TransportStatus {
    return isTauriEnvironment() ? "available" : "unavailable";
  }

  public getStatus(): TransportStatus {
    return this.status;
  }

  public async invoke<T = unknown, R = unknown>(
    cmd: IpcCommandName,
    req?: T
  ): Promise<R> {
    if (!isTauriEnvironment()) {
      throw new BackendUnavailableError(
        `Cannot execute IPC command '${cmd}': Tauri desktop runtime bridge unavailable`
      );
    }

    const win = window as any;
    const invoker =
      win.__TAURI_INTERNALS__?.invoke ??
      win.__TAURI__?.core?.invoke ??
      win.__TAURI__?.invoke;

    if (typeof invoker !== "function") {
      throw new BackendUnavailableError(
        `Tauri IPC bridge function unavailable for command '${cmd}'`
      );
    }

    try {
      return (await invoker(cmd, req)) as R;
    } catch (err) {
      throw normalizeBackendError(err);
    }
  }

  public createChannel<T = unknown>(
    onEvent: (event: T) => void
  ): ChannelAdapter<T> | any {
    if (!isTauriEnvironment()) {
      throw new BackendUnavailableError(
        "Cannot create streaming channel: Tauri desktop runtime bridge unavailable"
      );
    }

    const win = window as any;
    if (win.__TAURI_INTERNALS__?.Channel) {
      return new win.__TAURI_INTERNALS__.Channel((raw: T) => onEvent(raw));
    }

    return {
      onEvent,
      channelId: Date.now(),
    };
  }
}

let activeTransport: BackendTransport | null = null;

/**
 * Sets a custom backend transport (Dependency Injection for tests).
 */
export function setBackendTransport(transport: BackendTransport | null): void {
  activeTransport = transport;
}

/**
 * Returns the currently active backend transport.
 */
export function getBackendTransport(): BackendTransport {
  return activeTransport ?? new DefaultTauriTransport();
}

/**
 * Creates a helper streaming channel compatible with Tauri Channel<T>.
 */
export function createStreamChannel<T>(
  onEvent: (event: T) => void,
  transport: BackendTransport = getBackendTransport()
): ChannelAdapter<T> | any {
  return transport.createChannel(onEvent);
}

/**
 * Strongly-typed client covering all 40 commands in `IPC_COMMAND_NAMES`.
 */
export class BackendClient {
  private transportOverride: BackendTransport | null = null;

  constructor(transport?: BackendTransport) {
    if (transport) {
      this.transportOverride = transport;
    }
  }

  public get transport(): BackendTransport {
    return this.transportOverride ?? getBackendTransport();
  }

  public get status(): TransportStatus {
    return this.transport.status;
  }

  public getStatus(): TransportStatus {
    return this.transport.getStatus();
  }

  /**
   * Internal generic invocation with secret protection.
   */
  private async execute<T, R>(cmd: IpcCommandName, req?: T): Promise<R> {
    try {
      // Ensure plaintext secrets are never inadvertently logged or leaked
      if (cmd !== "set_provider_api_key") {
        redactSensitiveData(req);
      }
      return await this.transport.invoke<T, R>(cmd, req);
    } catch (err) {
      if (err instanceof BackendUnavailableError) {
        throw err;
      }
      throw normalizeBackendError(err);
    }
  }

  // ============================================================================
  // Group 1: Chat & Session (Commands 1..4)
  // ============================================================================

  public async createOrLoadChatSession(
    req: CreateOrLoadChatSessionReq = {}
  ): Promise<ChatSessionInitDto> {
    return this.execute("create_or_load_chat_session", req);
  }

  public async listChatSessions(
    req: ListChatSessionsReq = {}
  ): Promise<ChatSessionListDto> {
    return this.execute("list_chat_sessions", req);
  }

  public async sendChatMessage(
    req: SendChatMessageReq,
    onEvent?: (event: ChatStreamEvent) => void
  ): Promise<SendChatTurnResponseDto> {
    const payload: any = { ...req };
    if (onEvent) {
      payload.channel = this.transport.createChannel(onEvent);
    }
    return this.execute("send_chat_message", payload);
  }

  public async cancelChatTurn(
    req: CancelChatTurnReq
  ): Promise<CancelChatTurnResponseDto> {
    return this.execute("cancel_chat_turn", req);
  }

  // ============================================================================
  // Group 2: Draft, Working State & Progress Engine (Commands 5..9)
  // ============================================================================

  public async getDraftWorkspaceState(
    req: GetDraftWorkspaceStateReq
  ): Promise<DraftWorkspaceStateDto> {
    return this.execute("get_draft_workspace_state", req);
  }

  public async getProgressDetail(
    req: GetProgressDetailReq
  ): Promise<ProgressDetailDto> {
    return this.execute("get_progress_detail", req);
  }

  public async applyManualDraftEdit(
    req: ApplyManualDraftEditReq
  ): Promise<ManualEditResponseDto> {
    return this.execute("apply_manual_draft_edit", req);
  }

  public async confirmPhase(
    req: ConfirmPhaseReq
  ): Promise<PhaseConfirmResponseDto> {
    return this.execute("confirm_phase", req);
  }

  public async regenerateDraftPhase(
    req: RegenerateDraftPhaseReq
  ): Promise<RegeneratePhaseResponseDto> {
    return this.execute("regenerate_draft_phase", req);
  }

  // ============================================================================
  // Group 3: Versioning & Restore (Commands 10..11)
  // ============================================================================

  public async listDraftVersions(
    req: ListDraftVersionsReq
  ): Promise<DraftVersionListDto> {
    return this.execute("list_draft_versions", req);
  }

  public async restoreDraftVersion(
    req: RestoreDraftVersionReq
  ): Promise<RestoreVersionResponseDto> {
    return this.execute("restore_draft_version", req);
  }

  // ============================================================================
  // Group 4: Pending Patches (Commands 12..15)
  // ============================================================================

  public async listPendingPatches(
    req: ListPendingPatchesReq
  ): Promise<PendingPatchListDto> {
    return this.execute("list_pending_patches", req);
  }

  public async confirmPendingPatch(
    req: ConfirmPendingPatchReq
  ): Promise<PatchResolutionDto> {
    return this.execute("confirm_pending_patch", req);
  }

  public async rejectPendingPatch(
    req: RejectPendingPatchReq
  ): Promise<PatchResolutionDto> {
    return this.execute("reject_pending_patch", req);
  }

  public async confirmAllPendingPatches(
    req: ConfirmAllPendingPatchesReq
  ): Promise<ConfirmAllPatchesResponseDto> {
    return this.execute("confirm_all_pending_patches", req);
  }

  // ============================================================================
  // Group 5: Segmented Workflow (Commands 16..18)
  // ============================================================================

  public async startOrResumeWorkflow(
    req: StartOrResumeWorkflowReq,
    onProgress?: (event: WorkflowProgressEvent) => void
  ): Promise<WorkflowExecutionSummaryDto> {
    const payload: any = { ...req };
    if (onProgress) {
      payload.channel = this.transport.createChannel(onProgress);
    }
    return this.execute("start_or_resume_workflow", payload);
  }

  public async cancelWorkflow(
    req: CancelWorkflowReq
  ): Promise<CancelWorkflowResponseDto> {
    return this.execute("cancel_workflow", req);
  }

  public async getWorkflowStatus(
    req: GetWorkflowStatusReq
  ): Promise<WorkflowStatusDto> {
    return this.execute("get_workflow_status", req);
  }

  // ============================================================================
  // Group 6: Start / Commit Draft (Commands 19..21)
  // ============================================================================

  public async getCommitConfirmationPreview(
    req: GetCommitConfirmationPreviewReq
  ): Promise<CommitPreviewDto> {
    return this.execute("get_commit_confirmation_preview", req);
  }

  public async commitDraft(
    req: CommitDraftReq
  ): Promise<CommitDraftResponseDto> {
    return this.execute("commit_draft", req);
  }

  public async reopenCommittedDraft(
    req: ReopenCommittedDraftReq
  ): Promise<ReopenCommittedDraftResponseDto> {
    return this.execute("reopen_committed_draft", req);
  }

  // ============================================================================
  // Group 7: Projects & Creator Mode (Commands 22..24)
  // ============================================================================

  public async listProjects(
    req: ListProjectsReq = {}
  ): Promise<ProjectListDto> {
    return this.execute("list_projects", req);
  }

  public async getProjectDetail(
    req: GetProjectDetailReq
  ): Promise<ProjectDetailDto> {
    return this.execute("get_project_detail", req);
  }

  public async deleteProject(
    req: DeleteProjectReq
  ): Promise<DeleteProjectResponseDto> {
    return this.execute("delete_project", req);
  }

  // ============================================================================
  // Group 8: Character Library (Commands 25..28)
  // ============================================================================

  public async listCharacters(
    req: ListCharactersReq = {}
  ): Promise<CharacterListDto> {
    return this.execute("list_characters", req);
  }

  public async getCharacterDetail(
    req: GetCharacterDetailReq
  ): Promise<CharacterDetailDto> {
    return this.execute("get_character_detail", req);
  }

  public async createOrUpdateLibraryCharacter(
    req: CreateOrUpdateLibraryCharacterReq
  ): Promise<CharacterDetailDto> {
    return this.execute("create_or_update_library_character", req);
  }

  public async linkLibraryCharacterToDraft(
    req: LinkLibraryCharacterToDraftReq
  ): Promise<LinkLibraryCharacterResponseDto> {
    return this.execute("link_library_character_to_draft", req);
  }

  // ============================================================================
  // Group 9: Writing Mode, Scene Lock, Generation & Save Chapter (Commands 29..37)
  // ============================================================================

  public async getChapterWritingState(
    req: GetChapterWritingStateReq
  ): Promise<ChapterWritingReadModelDto> {
    return this.execute("get_chapter_writing_state", req);
  }

  public async setSceneLock(
    req: SetSceneLockReq
  ): Promise<SceneLockStateDto> {
    return this.execute("set_scene_lock", req);
  }

  public async setChapterScenesLockBulk(
    req: SetChapterScenesLockBulkReq
  ): Promise<BulkSceneLockStateDto> {
    return this.execute("set_chapter_scenes_lock_bulk", req);
  }

  public async startSceneGeneration(
    req: StartSceneGenerationReq,
    onEvent?: (event: SceneGenerationStreamEvent) => void
  ): Promise<SceneGenerationResultDto> {
    const payload: any = { ...req };
    if (onEvent) {
      payload.channel = this.transport.createChannel(onEvent);
    }
    return this.execute("start_scene_generation", payload);
  }

  public async cancelSceneGeneration(
    req: CancelSceneGenerationReq
  ): Promise<CancelSceneGenerationResponseDto> {
    return this.execute("cancel_scene_generation", req);
  }

  public async respondEditScopeExpansion(
    req: RespondEditScopeExpansionReq
  ): Promise<RespondEditScopeExpansionResponseDto> {
    return this.execute("respond_edit_scope_expansion", req);
  }

  public async listSceneVersions(
    req: ListSceneVersionsReq
  ): Promise<SceneVersionListDto> {
    return this.execute("list_scene_versions", req);
  }

  public async saveChapter(
    req: SaveChapterReq
  ): Promise<SaveChapterResp> {
    return this.execute("save_chapter", req);
  }

  public async getChapterSaveReceipt(
    req: GetChapterSaveReceiptReq
  ): Promise<SaveReceiptLookupDto> {
    return this.execute("get_chapter_save_receipt", req);
  }

  // ============================================================================
  // Group 10: Model Configuration & Credentials (Commands 38..40)
  // ============================================================================

  public async getRuntimeAndProviderConfig(
    req: GetRuntimeConfigReq = {}
  ): Promise<RuntimeAndProviderConfigDto> {
    return this.execute("get_runtime_and_provider_config", req);
  }

  public async updateRuntimeAndProviderConfig(
    req: UpdateRuntimeConfigReq
  ): Promise<RuntimeAndProviderConfigDto> {
    return this.execute("update_runtime_and_provider_config", req);
  }

  public async setProviderApiKey(
    req: SetProviderApiKeyReq
  ): Promise<SetProviderApiKeyResponseDto> {
    return this.execute("set_provider_api_key", req);
  }

  /**
   * Dedicated secure alias for setting provider secret.
   */
  public async saveProviderSecret(
    req: SetProviderApiKeyReq
  ): Promise<SetProviderApiKeyResponseDto> {
    return this.setProviderApiKey(req);
  }
}

/**
 * Singleton typed backend client using the active or default transport.
 */
export const backendClient: BackendClient = new BackendClient();

/**
 * Factory helper for creating new client instances with a custom transport.
 */
export function createBackendClient(transport?: BackendTransport): BackendClient {
  return new BackendClient(transport);
}
