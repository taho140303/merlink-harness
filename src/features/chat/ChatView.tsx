import { useState, useRef, useEffect, useMemo, useCallback } from "react";
import { GripVertical } from "lucide-react";
import { useNavigate, useOutletContext } from "react-router";
import type { ChatStartupContext } from "@/app/layouts/FullLayout";
import { buildProjectPath } from "@/features/draft/commitNavigation";
import { cn } from "@/shared/utils/cn";
import { useResizablePanel } from "@/shared/hooks/useResizablePanel";
import { useIsCompact } from "@/shared/hooks/useMediaQuery";
import { ChatHeader } from "./ChatHeader";
import { ChatWelcome } from "./ChatWelcome";
import { ChatInputBar } from "./ChatInputBar";
import { ChatMessageItem } from "./ChatMessageItem";
import { DraftWorkspace } from "@/features/draft/DraftWorkspace";
import { FloatingOutline } from "@/shared/components/ui/floating-outline";
import { PreDraftSuggestionDialog } from "./PreDraftSuggestionDialog";
import {
  fromDraftProgressDto,
  reconcileDraftProgress,
  resolveSystemLanguage,
  type BusinessFlowData,
  type DraftProgressState,
} from "@/features/business-flow";
import {
  BackendClient,
  isTauriEnvironment,
} from "@/shared/backend/client";
import {
  BackendUnavailableError,
  type ChatMessageDto,
  type ChatStreamEvent,
  type SendChatTurnResponseDto,
  type CancelChatTurnResponseDto,
  type DraftWorkspaceStateDto,
  type DraftVersionSummaryDto,
  type ToolCallItemDto,
} from "@/shared/backend/types";
import type { ChatMessage } from "./types";
import type { FeedbackContext } from "@/features/draft/types";
import { useT } from "@/shared/i18n/useT";
import { useLocale } from "@/shared/i18n/store";

// ============================================================================
// Types & Pure State Controller for Chat + Progress Integration
// ============================================================================

export interface ChatBackendState {
  backendStatus: "available" | "unavailable";
  initialized: boolean;
  sessionId: string;
  draftSessionId: string | null;
  headVersionSeq: number | null;
  sealedVersion: DraftVersionSummaryDto | null;
  pendingPatchCount: number;
  interruptedTurnAlert: string | null;
  isStreaming: boolean;
  activeTurnId: string | null;
  progress: DraftProgressState;
  workingState: BusinessFlowData | null;
  messages: ChatMessage[];
  lastError: string | null;
}

function withStartupTimeout<T>(promise: Promise<T>, step: string): Promise<T> {
  return new Promise((resolve, reject) => {
    const timer = setTimeout(() => reject(new Error(`Chat startup timed out while ${step} (15s)`)), 15_000);
    promise.then(
      (value) => { clearTimeout(timer); resolve(value); },
      (error) => { clearTimeout(timer); reject(error); },
    );
  });
}

function mapDtoToChatMessage(dto: ChatMessageDto): ChatMessage {
  return {
    id: dto.id,
    role: dto.role === "assistant" ? "assistant" : "user",
    content: dto.content,
    thinkingSeconds: dto.thinkingSeconds ?? undefined,
    thinkingDetails: dto.thinkingDetails ?? undefined,
    isThinking: Boolean(dto.isThinking),
    toolCalls: dto.toolCalls?.map((tc: ToolCallItemDto) => ({
      id: tc.id,
      name: tc.name,
      parameters: tc.parameters ?? {},
      result: tc.result ?? undefined,
      executionTimeMs: tc.executionTimeMs ?? undefined,
      status: tc.status === "success" || tc.status === "applied"
        ? "success"
        : tc.status === "pending" || tc.status === "pending_confirmation"
          ? "pending"
          : "error",
    })),
    timestamp: dto.timestamp || "Vừa xong",
  };
}

/**
 * Pure orchestration controller managing Chat IPC commands, streaming events,
 * dual progress reconciliation, and controlled cancellation without React dependencies.
 */
export class ChatBackendController {
  private client: BackendClient;
  private state: ChatBackendState;
  private listeners: Set<(state: ChatBackendState) => void> = new Set();
  /** Debounced workspace refresh so an applied tool call reaches the open Draft Workspace. */
  private workspaceRefreshTimer: ReturnType<typeof setTimeout> | null = null;

  private initGeneration = 0;
  private turnGeneration = 0;
  private pendingInit: { key: string; promise: Promise<ChatBackendState> } | null = null;

  constructor(client?: BackendClient, initialConfig?: Partial<ChatBackendState>) {
    this.client = client ?? new BackendClient();
    const isAvail = isTauriEnvironment() || this.client.status === "available";

    this.state = {
      backendStatus: isAvail ? "available" : "unavailable",
      initialized: initialConfig?.initialized ?? Boolean(initialConfig?.sessionId),
      sessionId: initialConfig?.sessionId ?? "",
      draftSessionId: initialConfig?.draftSessionId ?? null,
      headVersionSeq: initialConfig?.headVersionSeq ?? null,
      sealedVersion: initialConfig?.sealedVersion ?? null,
      pendingPatchCount: initialConfig?.pendingPatchCount ?? 0,
      interruptedTurnAlert: initialConfig?.interruptedTurnAlert ?? null,
      isStreaming: false,
      activeTurnId: null,
      progress: initialConfig?.progress ?? fromDraftProgressDto(null),
      workingState: initialConfig?.workingState ?? null,
      messages: initialConfig?.messages ?? [],
      lastError: null,
    };
  }

  public getState(): ChatBackendState {
    return { ...this.state };
  }

  public subscribe(listener: (state: ChatBackendState) => void): () => void {
    this.listeners.add(listener);
    return () => {
      this.listeners.delete(listener);
    };
  }

  private notify() {
    const snapshot = this.getState();
    for (const listener of this.listeners) {
      listener(snapshot);
    }
  }

  public updateState(partial: Partial<ChatBackendState>) {
    this.state = { ...this.state, ...partial };
    this.notify();
  }

  public dismissAlert() {
    this.updateState({ interruptedTurnAlert: null });
  }

  /**
   * Initializes or reloads a chat session using `create_or_load_chat_session`.
   */
  public initSession(options?: {
    sessionId?: string;
    projectId?: string;
    trustedLanguage?: string;
  }, forceNew = false): Promise<ChatBackendState> {
    const key = JSON.stringify([options?.sessionId ?? null, options?.projectId ?? null]);
    // StrictMode can replay the mount effect; share the same pending request so
    // a new chat is allocated only once rather than creating duplicate sessions.
    if (!forceNew && this.pendingInit?.key === key) return this.pendingInit.promise;
    const promise = this.performInitSession(options);
    this.pendingInit = { key, promise };
    void promise.finally(() => {
      if (this.pendingInit?.promise === promise) this.pendingInit = null;
    });
    return promise;
  }

  private async performInitSession(options?: {
    sessionId?: string;
    projectId?: string;
    trustedLanguage?: string;
  }): Promise<ChatBackendState> {
    const generation = ++this.initGeneration;
    ++this.turnGeneration;
    const isAvail = isTauriEnvironment() || this.client.status === "available";
    this.updateState({
      backendStatus: isAvail ? "available" : "unavailable",
      initialized: false,
      sessionId: "",
      draftSessionId: null,
      headVersionSeq: null,
      sealedVersion: null,
      pendingPatchCount: 0,
      interruptedTurnAlert: null,
      isStreaming: false,
      activeTurnId: null,
      progress: fromDraftProgressDto(null),
      workingState: null,
      messages: [],
      lastError: null,
    });
    if (!isAvail) {
      this.updateState({ backendStatus: "unavailable", lastError: "BACKEND_UNAVAILABLE" });
      return this.getState();
    }

    try {
      // Only an explicit route ID resumes a session; the backend allocates new IDs.
      const initDto = await this.client.createOrLoadChatSession({
        ...(options?.sessionId ? { sessionId: options.sessionId } : {}),
        ...(options?.projectId ? { projectId: options.projectId } : {}),
      });
      if (generation !== this.initGeneration) return this.getState();
      if (!initDto.sessionId || !initDto.draftSessionId) {
        throw new Error("Backend returned an invalid chat/draft session ID");
      }
      this.updateState({
        backendStatus: "available",
        initialized: true,
        sessionId: initDto.sessionId,
        draftSessionId: initDto.draftSessionId,
        headVersionSeq: initDto.headVersionSeq ?? null,
        messages: (initDto.messages ?? []).map(mapDtoToChatMessage),
        workingState: initDto.workingState as BusinessFlowData,
        progress: fromDraftProgressDto(initDto.progress),
        interruptedTurnAlert: initDto.interruptedTurnAlert ?? null,
        lastError: null,
      });
      return this.getState();
    } catch (err: any) {
      if (generation !== this.initGeneration) return this.getState();
      this.updateState({
        backendStatus: err instanceof BackendUnavailableError ? "unavailable" : this.state.backendStatus,
        lastError: err instanceof BackendUnavailableError ? "BACKEND_UNAVAILABLE" : err?.message ?? "Failed to initialize chat session",
      });
      return this.getState();
    }
  }

  /**
   * Dispatches a chat message, processes stream events, and reconciles dual progress.
   */
  public async sendMessage(
    content: string,
    options?: {
      feedbackContext?: FeedbackContext | null;
      systemLanguageOverride?: string | null;
      trustedLanguage?: string | null;
    },
  ): Promise<SendChatTurnResponseDto | null> {
    const trimmed = content.trim();
    if (!trimmed) return null;

    if (this.state.backendStatus === "unavailable") {
      const errorMsg = "BACKEND_UNAVAILABLE: Cannot execute chat turn without desktop Tauri runtime";
      this.updateState({ lastError: errorMsg });
      throw new BackendUnavailableError(errorMsg);
    }
    if (!this.state.initialized || !this.state.sessionId || this.state.isStreaming) {
      const errorMsg = "Chat session is not ready or a turn is already in progress";
      this.updateState({ lastError: errorMsg });
      throw new Error(errorMsg);
    }
    const generation = this.turnGeneration;

    // CI-09: Trusted UI settings language strictly takes precedence
    const systemLanguage = resolveSystemLanguage(
      options?.trustedLanguage,
      options?.systemLanguageOverride,
    );

    const startTime = Date.now();
    const userMsgId = `usr-${startTime}`;
    const assistantMsgId = `ast-${startTime}`;

    const userMsg: ChatMessage = {
      id: userMsgId,
      role: "user",
      content: trimmed,
      timestamp: "Vừa xong",
    };

    const placeholderAssistant: ChatMessage = {
      id: assistantMsgId,
      role: "assistant",
      content: "",
      isThinking: true,
      thinkingSeconds: 1,
      thinkingDetails: "Khởi động mô hình và phân tích ngữ cảnh đầu vào...",
      timestamp: "Vừa xong",
    };

    this.updateState({
      messages: [...this.state.messages, userMsg, placeholderAssistant],
      isStreaming: true,
      lastError: null,
    });

    const feedbackPayload = options?.feedbackContext
      ? {
          code: options.feedbackContext.tag || options.feedbackContext.id,
          tag: options.feedbackContext.tag,
          title: options.feedbackContext.title,
          content: (options.feedbackContext as any).content ?? JSON.stringify(options.feedbackContext),
        }
      : null;

    try {
      const response = await this.client.sendChatMessage(
        {
          sessionId: this.state.sessionId,
          content: trimmed,
          systemLanguage,
          feedbackContext: feedbackPayload,
        },
        (event: ChatStreamEvent) => {
          if (generation === this.turnGeneration) this.handleStreamEvent(event, assistantMsgId, startTime);
        },
      );

      if (generation !== this.turnGeneration) return response;
      const finalSeconds = Math.max(1, Math.round((Date.now() - startTime) / 1000));
      this.updateState({
        isStreaming: false,
        activeTurnId: null,
        messages: this.state.messages.map((m) =>
          m.id === assistantMsgId
            ? {
                ...m,
                isThinking: false,
                thinkingSeconds: m.thinkingSeconds ?? finalSeconds,
                content: m.content || response.assistantText,
              }
            : m,
        ),
        progress: reconcileDraftProgress(this.state.progress, response.progress),
        sealedVersion: response.sealedVersion ?? this.state.sealedVersion,
        headVersionSeq: response.sealedVersion?.seq ?? this.state.headVersionSeq,
        pendingPatchCount: response.pendingPatches
          ? response.pendingPatches.filter((p) => p.status === "pending").length
          : this.state.pendingPatchCount,
      });

      await this.refreshDurableChat();
      return response;
    } catch (err: any) {
      if (generation !== this.turnGeneration) throw err;
      // A failed provider turn may still contain a durable tool audit.
      const errMsg = err?.message ?? "Chat turn execution failed";
      const finalSeconds = Math.max(1, Math.round((Date.now() - startTime) / 1000));
      this.updateState({
        isStreaming: false,
        activeTurnId: null,
        lastError: errMsg,
        messages: this.state.messages.map((m) =>
          m.id === assistantMsgId
            ? {
                ...m,
                isThinking: false,
                thinkingSeconds: m.thinkingSeconds ?? finalSeconds,
                content: m.content
                  ? `${m.content}\n\n[Lỗi kết nối nhà cung cấp: ${errMsg}]`
                  : `[Không thể lưu lượt trò chuyện do lỗi kết nối: ${errMsg}]`,
              }
            : m,
        ),
        // Notice: sealedVersion remains unchanged/null; never fabricate a sealed version on failure!
      });
      // A lost IPC response may have persisted a turn: reload before claiming durable state.
      await this.refreshDurableChat();
      throw err;
    }
  }

  /** Reload only the working state (cheap) so the Draft Workspace mirrors applied tool calls. */
  private async refreshWorkingState(): Promise<void> {
    const draftSessionId = this.state.draftSessionId;
    if (!draftSessionId) return;
    try {
      const dto = await this.client.getDraftWorkspaceState({ draftSessionId });
      if (!dto) return;
      this.updateState({
        workingState: (dto.businessFlowData as unknown as BusinessFlowData) ?? this.state.workingState,
        progress: fromDraftProgressDto(dto.progress),
      });
    } catch {
      // Non-fatal: the next turn reload refreshes the authoritative snapshot.
    }
  }

  /** Coalesce bursts of applied writes into one workspace reload (at most every 400 ms). */
  private scheduleWorkingStateRefresh(): void {
    if (this.workspaceRefreshTimer) return;
    this.workspaceRefreshTimer = setTimeout(() => {
      this.workspaceRefreshTimer = null;
      void this.refreshWorkingState();
    }, 400);
  }

  private async refreshDurableChat(): Promise<void> {
    if (!this.state.initialized || !this.state.sessionId) return;
    const generation = this.initGeneration;
    try {
      const dto = await this.client.createOrLoadChatSession({ sessionId: this.state.sessionId });
      if (generation !== this.initGeneration || dto.sessionId !== this.state.sessionId) return;
      this.updateState({
        messages: dto.messages.map(mapDtoToChatMessage),
        workingState: dto.workingState as BusinessFlowData,
        progress: fromDraftProgressDto(dto.progress),
        headVersionSeq: dto.headVersionSeq ?? null,
        interruptedTurnAlert: dto.interruptedTurnAlert ?? this.state.interruptedTurnAlert,
      });
    } catch {
      // Keep the explicit turn error; stream content is provisional until reload succeeds.
    }
  }

  private handleStreamEvent(event: ChatStreamEvent, assistantMsgId: string, startTime?: number) {
    switch (event.event) {
      case "turnStarted":
        this.updateState({ activeTurnId: event.data.turnId });
        break;

      case "assistantDelta": {
        const elapsed = startTime ? Math.max(1, Math.round((Date.now() - startTime) / 1000)) : undefined;
        this.updateState({
          messages: this.state.messages.map((m) =>
            m.id === assistantMsgId
              ? {
                  ...m,
                  content: (m.content || "") + event.data.delta,
                  isThinking: false,
                  thinkingSeconds: m.thinkingSeconds ?? elapsed,
                }
              : m,
          ),
        });
        break;
      }

      case "toolCallStarted":
        this.updateState({
          messages: this.state.messages.map((m) => {
            if (m.id !== assistantMsgId) return m;
            const currentTools = m.toolCalls || [];
            if (currentTools.some((t) => t.id === event.data.toolCallId)) return m;
            return {
              ...m,
              toolCalls: [
                ...currentTools,
                {
                  id: event.data.toolCallId,
                  name: event.data.toolCode,
                  parameters: {},
                  status: "pending",
                },
              ],
            };
          }),
        });
        break;

      case "toolCallCompleted": {
        // P5S2-P: Streamed tool result updates pending patch count independently of sealed DraftVersion!
        const isPending =
          event.data.status === "pending_confirmation" || event.data.status === "pending";
        const newCount = isPending
          ? this.state.pendingPatchCount + 1
          : this.state.pendingPatchCount;

        this.updateState({
          pendingPatchCount: newCount,
          messages: this.state.messages.map((m) => {
            if (m.id !== assistantMsgId) return m;
            const currentTools = m.toolCalls || [];
            return {
              ...m,
              toolCalls: currentTools.map((tc) =>
                tc.id === event.data.toolCallId
                  ? {
                      ...tc,
                      status: event.data.status === "applied"
                        ? "success"
                        : event.data.status === "pending" || event.data.status === "pending_confirmation"
                          ? "pending"
                          : "error",
                      result: {
                        status: event.data.status,
                        hasAppliedWrite: event.data.hasAppliedWrite,
                      },
                    }
                  : tc,
              ),
            };
          }),
        });
        break;
      }

      case "workingStateUpdated":
        this.updateState({
          progress: reconcileDraftProgress(this.state.progress, event.data.progress),
          pendingPatchCount: event.data.pendingPatchCount,
        });
        // The event only carries counters; refresh the snapshot so the Draft Workspace (and the
        // tool panel) reflect the mutation that just happened, not the state from turn start.
        this.scheduleWorkingStateRefresh();
        break;

      case "turnCompleted":
        this.updateState({
          isStreaming: false,
          activeTurnId: null,
          sealedVersion: event.data.sealedVersion ?? this.state.sealedVersion,
          headVersionSeq: event.data.sealedVersion?.seq ?? this.state.headVersionSeq,
          messages: this.state.messages.map((m) =>
            m.id === assistantMsgId ? { ...m, isThinking: false } : m,
          ),
        });
        break;

      case "turnError":
        // Preserve the provider error while the durable tool audit is reloaded.
        this.updateState({
          isStreaming: false,
          activeTurnId: null,
          lastError: event.data.error.message,
          messages: this.state.messages.map((m) =>
            m.id === assistantMsgId
              ? {
                  ...m,
                  isThinking: false,
                  content: m.content
                    ? `${m.content}\n\n[Lỗi: ${event.data.error.message}]`
                    : `[Lỗi lượt trò chuyện: ${event.data.error.message}]`,
                }
              : m,
          ),
        });
        break;
    }
  }

  /**
   * Controlled cancellation of active chat turn via `cancel_chat_turn`.
   */
  public async cancelActiveTurn(): Promise<CancelChatTurnResponseDto | null> {
    const turnId = this.state.activeTurnId;
    if (!turnId) return null;

    try {
      const res = await this.client.cancelChatTurn({
        sessionId: this.state.sessionId,
        turnId,
      });

      this.updateState({
        isStreaming: false,
        activeTurnId: null,
        interruptedTurnAlert: "Lượt hội thoại đã bị dừng theo yêu cầu của người dùng.",
        sealedVersion: res.sealedVersion ?? this.state.sealedVersion,
        headVersionSeq: res.sealedVersion?.seq ?? this.state.headVersionSeq,
      });

      return res;
    } catch {
      this.updateState({
        isStreaming: false,
        activeTurnId: null,
        interruptedTurnAlert: "Đã gửi yêu cầu dừng lượt hội thoại.",
      });
      return null;
    }
  }

  /**
   * Reconciles durable backend state (load_draft_session / get_draft_workspace_state)
   * after a lost response before claiming any version (phase5_sprint2_deep1-F1).
   */
  public async reconcileDurableState(): Promise<DraftWorkspaceStateDto | null> {
    const draftSessionId = this.state.draftSessionId;
    if (!draftSessionId) {
      if (this.state.sessionId) {
        await this.initSession({ sessionId: this.state.sessionId });
      }
      return null;
    }

    try {
      const wsState = await this.client.getDraftWorkspaceState({ draftSessionId });
      const pendingCount = wsState.pendingPatches
        ? wsState.pendingPatches.filter((p) => p.status === "pending").length
        : 0;

      this.updateState({
        headVersionSeq: wsState.headVersionSeq ?? null,
        progress: fromDraftProgressDto(wsState.progress),
        workingState: wsState.businessFlowData as BusinessFlowData,
        pendingPatchCount: pendingCount,
        isStreaming: false,
        activeTurnId: null,
      });

      return wsState;
    } catch (err: any) {
      this.updateState({
        lastError: err?.message ?? "Failed to reconcile durable draft state",
      });
      return null;
    }
  }
}

// ============================================================================
// ChatView Component Props
// ============================================================================

export interface ChatViewProps {
  client?: BackendClient;
  sessionId?: string;
  draftSessionId?: string;
  initialMessages?: ChatMessage[];
  systemLanguage?: "vi" | "en";
  onToggleDraft?: () => void;
  onProgressUpdate?: (progress: DraftProgressState) => void;
  onVersionSealed?: (version: DraftVersionSummaryDto) => void;
  onPendingPatchesChange?: (count: number) => void;
}

export function ChatView({
  client,
  sessionId,
  systemLanguage,
  onToggleDraft: propOnToggleDraft,
  onProgressUpdate,
  onVersionSealed,
  onPendingPatchesChange,
}: ChatViewProps = {}) {
  const t = useT();
  const { locale: currentLocale } = useLocale();
  const isCompact = useIsCompact();
  const navigate = useNavigate();
  const startup = useOutletContext<ChatStartupContext | undefined>();
  const finishChatStartup = startup?.finishChatStartup;
  const beginChatStartup = startup?.beginChatStartup;
  const backendClient = useMemo(() => client ?? new BackendClient(), [client]);

  // One controller per mounted view. A lazy state initializer keeps the construction out of
  // the render path (the React Compiler rules reject writing to a ref during render).
  const [controller] = useState(() => new ChatBackendController(backendClient));

  const [controllerState, setControllerState] = useState<ChatBackendState>(() => ({
    ...controller.getState(),
    // The server cannot detect the desktop bridge. Hydrate with the same
    // snapshot, then publish the actual runtime status in the effect below.
    backendStatus: "unavailable",
  }));

  useEffect(() => {
    setControllerState(controller.getState());
    return controller.subscribe((next) => {
      setControllerState(next);
      if (onProgressUpdate) onProgressUpdate(next.progress);
      if (onVersionSealed && next.sealedVersion) onVersionSealed(next.sealedVersion);
      if (onPendingPatchesChange) onPendingPatchesChange(next.pendingPatchCount);
    });
  }, [controller, onProgressUpdate, onVersionSealed, onPendingPatchesChange]);

  // Route ID is the only resume authority. Locale changes must not create sessions.
  // Retry briefly on mount in case WebKitGTK/Tauri IPC bridge attaches after SSR hydration.
  useEffect(() => {
    let cancelled = false;
    let retryTimer: ReturnType<typeof setTimeout> | null = null;
    let attempts = 0;
    const maxAttempts = 5;
    // Never expose the previous route's historical message blocks if loading
    // configuration for the next route fails before session hydration begins.
    controller.updateState({
      initialized: false,
      sessionId: "",
      draftSessionId: null,
      messages: [],
      lastError: null,
    });

    const attemptInit = () => {
      if (cancelled) return;
      attempts += 1;
      if (!isTauriEnvironment() && backendClient.status === "unavailable") {
        if (attempts < maxAttempts) {
          retryTimer = setTimeout(attemptInit, 200);
        } else {
          controller.updateState({ backendStatus: "unavailable", lastError: "BACKEND_UNAVAILABLE" });
          finishChatStartup?.();
        }
        return;
      }
      // Configuration and persisted messages are both required before revealing
      // the chat. Fetch configuration first so a failed setup cannot create a
      // throwaway session on every retry.
      void withStartupTimeout(backendClient.getRuntimeAndProviderConfig({}), "loading runtime configuration").then(() => {
        if (cancelled) return null;
        return withStartupTimeout(controller.initSession({ sessionId }), "loading the chat session");
      }).then((state) => {
        if (cancelled || !state) return;
        if (state.initialized) window.dispatchEvent(new Event("merlink:chat-history-changed"));
        finishChatStartup?.();
      }).catch((error: unknown) => {
        if (cancelled) return;
        controller.updateState({ lastError: error instanceof Error ? error.message : String(error) });
        finishChatStartup?.();
      });
    };

    attemptInit();

    return () => {
      cancelled = true;
      if (retryTimer) clearTimeout(retryTimer);
    };
  }, [controller, sessionId, backendClient, finishChatStartup]);

  const retryInitialization = useCallback((forceNew = false) => {
    beginChatStartup?.();
    void withStartupTimeout(backendClient.getRuntimeAndProviderConfig({}), "loading runtime configuration").then(() =>
      withStartupTimeout(controller.initSession({ sessionId }, forceNew), "loading the chat session"),
    ).then((state) => {
      if (state.initialized) window.dispatchEvent(new Event("merlink:chat-history-changed"));
      finishChatStartup?.();
    }).catch((error: unknown) => {
      controller.updateState({ lastError: error instanceof Error ? error.message : String(error) });
      finishChatStartup?.();
    });
  }, [backendClient, controller, sessionId, beginChatStartup, finishChatStartup]);

  const [isDraftOpen, setIsDraftOpen] = useState(false);

  useEffect(() => {
    if (sessionId) return;
    const newChat = () => {
      setIsDraftOpen(false);
      retryInitialization(true);
    };
    window.addEventListener("merlink:new-chat", newChat);
    return () => window.removeEventListener("merlink:new-chat", newChat);
  }, [sessionId, retryInitialization]);

  const sampleProjects = useMemo(() => [] as { id: string; title: string }[], []);

  const handleViewSampleProject = (projectId: string) => {
    setIsDraftOpen(false);
    navigate(buildProjectPath(projectId));
  };

  const [isSuggestionOpen, setIsSuggestionOpen] = useState(false);
  const [suggestionSeen, setSuggestionSeen] = useState(() => {
    if (typeof window === 'undefined') return false;
    try {
      return window.localStorage.getItem('merlink.draftSuggestionSeen') === '1';
    } catch {
      return false;
    }
  });
  const [feedbackContext, setFeedbackContext] = useState<FeedbackContext | null>(null);
  const [keyboardRatio, setKeyboardRatio] = useState<number | null>(null);
  const scrollBottomRef = useRef<HTMLDivElement>(null);
  const scrollContainerRef = useRef<HTMLDivElement>(null);

  const businessFlowData = controllerState.initialized ? controllerState.workingState : null;
  const draftReady = controllerState.initialized && Boolean(controllerState.draftSessionId);

  const handleToggleDraft = () => {
    if (!draftReady) return;
    if (propOnToggleDraft) {
      propOnToggleDraft();
      return;
    }
    if (isDraftOpen) {
      setIsDraftOpen(false);
    } else if (suggestionSeen) {
      // The suggestion dialog is a one-time onboarding step.
      setIsDraftOpen(true);
    } else {
      setIsSuggestionOpen(true);
    }
  };

  const handleSuggestionConfirm = (suggestion?: string) => {
    setIsSuggestionOpen(false);
    setSuggestionSeen(true);
    try {
      window.localStorage.setItem('merlink.draftSuggestionSeen', '1');
    } catch {
      /* storage unavailable: keep the dialog for this session only */
    }
    if (!draftReady) return;
    setIsDraftOpen(true);
    if (suggestion && suggestion.trim()) {
      handleSendMessage(suggestion.trim());
    }
  };

  const { leftRatio, isDragging, containerRef, startDragging } = useResizablePanel({
    initialRatio: 50,
    minPxLeft: 545,
    minPxRight: 530,
  });

  const splitRatio = keyboardRatio ?? leftRatio;

  useEffect(() => {
    scrollBottomRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [controllerState.messages]);

  const handleSendMessage = useCallback(
    async (text: string) => {
      try {
        await controller.sendMessage(text, {
          feedbackContext,
          trustedLanguage: systemLanguage ?? currentLocale,
        });
        window.dispatchEvent(new Event("merlink:chat-history-changed"));
      } catch {
        // Error state already recorded in controllerState.lastError
      } finally {
        if (feedbackContext) {
          setFeedbackContext(null);
        }
      }
    },
    [controller, feedbackContext, systemLanguage, currentLocale],
  );

  const handleCancel = useCallback(() => {
    controller.cancelActiveTurn();
  }, [controller]);

  const scrollToMessage = (id: string) => {
    const el = document.getElementById(id);
    if (el) el.scrollIntoView({ behavior: "smooth", block: "start" });
  };

  const outlineItems = useMemo(
    () =>
      controllerState.messages
        .filter((m) => m.role === "user")
        .map((m) => ({
          id: m.id,
          title: m.content || "Message",
        })),
    [controllerState.messages],
  );

  return (
    <div
      ref={containerRef}
      className={cn(
        "relative flex h-full w-full overflow-hidden rounded-shell-sm bg-transparent",
        isDragging && "cursor-col-resize select-none",
      )}
    >
      {/* Cột trái: Chat Area */}
      <div
        style={{ width: isDraftOpen && !isCompact ? `${splitRatio}%` : "100%" }}
        className="flex flex-col overflow-hidden"
      >
        <ChatHeader
          isDraftOpen={isDraftOpen}
          onToggleDraft={handleToggleDraft}
          draftExists={Boolean(controllerState.headVersionSeq)}
          progressPercent={controllerState.progress.totalPercent}
          confirmedCount={controllerState.progress.confirmedCount}
          aiInferredCount={controllerState.progress.aiInferredCount}
          totalAttributes={controllerState.progress.totalAttributes}
          readinessGatePassed={draftReady && controllerState.progress.readinessGatePassed}
          isRequireComplete={draftReady && controllerState.progress.isRequireComplete}
          backendStatus={controllerState.backendStatus}
          interruptedTurnAlert={controllerState.interruptedTurnAlert}
          onDismissAlert={() => controller.dismissAlert()}
          businessFlowData={businessFlowData ?? undefined}
        />

        {!controllerState.initialized && (
          <div
            role="status"
            className="mx-6 mt-3 flex items-center justify-between rounded-lg border border-amber-500/30 bg-amber-500/10 px-3.5 py-2 text-xs text-amber-700 dark:text-amber-300"
          >
            <span>
              {currentLocale === "vi"
                ? "Chat chưa kết nối backend. Chế độ xem trước không lưu dữ liệu."
                : "Chat is not connected. Offline preview is non-durable; messages and drafts cannot be saved."}
            </span>
          </div>
        )}

        {controllerState.lastError && (
          <div role="alert" className="mx-6 mt-2 flex items-center justify-between gap-3 rounded-lg border border-red-500/30 bg-red-500/10 px-3 py-2 text-xs text-red-700 dark:text-red-300">
            <span>{controllerState.lastError}</span>
            {!controllerState.initialized && (
              <button type="button" onClick={() => retryInitialization()} className="focus-ring shrink-0 rounded-lg px-2 py-1 font-semibold underline underline-offset-2 hover:text-fg">
                {t("chat.startup.retry")}
              </button>
            )}
          </div>
        )}

        {controllerState.pendingPatchCount > 0 && (
          <div
            role="status"
            aria-live="polite"
            className="mx-6 mt-2 flex items-center gap-2 rounded-lg border border-blue-500/30 bg-blue-500/10 px-3.5 py-1.5 text-xs text-blue-700 dark:text-blue-300"
          >
            <span className="inline-block h-2 w-2 rounded-full bg-blue-500 animate-pulse" />
            <span>
              {currentLocale === "vi"
                ? `Có ${controllerState.pendingPatchCount} đề xuất sửa đổi đang chờ xem xét`
                : `${controllerState.pendingPatchCount} pending patches awaiting review`}
            </span>
          </div>
        )}

        <div className="relative flex-1 overflow-hidden">
          <div
            ref={scrollContainerRef}
            className="no-scrollbar h-full overflow-y-auto px-6 py-4 space-y-4"
          >
            {controllerState.messages.length === 0 ? (
              <ChatWelcome />
            ) : (
              controllerState.messages.map((m) => (
                <ChatMessageItem key={m.id} message={m} />
              ))
            )}
            <div ref={scrollBottomRef} />
          </div>

          {/* Thanh Outline (Minimap) */}
          {controllerState.messages.length > 0 && (
            <FloatingOutline
              items={outlineItems}
              containerRef={scrollContainerRef}
              onItemClick={scrollToMessage}
            />
          )}
        </div>

        <div className="shrink-0 p-6">
          <ChatInputBar
            feedbackContext={feedbackContext}
            onClearFeedback={() => setFeedbackContext(null)}
            onSend={handleSendMessage}
            disabled={!controllerState.initialized}
            isGenerating={controllerState.isStreaming}
            onStop={handleCancel}
          />
        </div>
      </div>

      {/* Ranh giới kéo thả */}
      {isDraftOpen && !isCompact && (
        <div
          role="separator"
          tabIndex={0}
          aria-label={t("chat.resizePanels")}
          aria-orientation="vertical"
          aria-valuenow={Math.round(splitRatio)}
          aria-valuemin={10}
          aria-valuemax={90}
          onMouseDown={(event) => {
            setKeyboardRatio(null);
            startDragging(event);
          }}
          onKeyDown={(event) => {
            if (event.key === "ArrowLeft" || event.key === "ArrowRight") {
              event.preventDefault();
              const adjustment = event.key === "ArrowLeft" ? -5 : 5;
              setKeyboardRatio(Math.min(90, Math.max(10, splitRatio + adjustment)));
            }
          }}
          className="focus-ring group relative flex w-2 shrink-0 cursor-col-resize items-center justify-center bg-transparent transition-colors hover:bg-accent-soft"
        >
          <div className="h-full w-px bg-line transition-colors group-hover:bg-accent" />
          <div
            className={cn(
              "absolute z-30 flex h-8 w-4.5 items-center justify-center rounded-md tier-card transition-opacity",
              isDragging ? "opacity-100 ring-2 ring-blue-500" : "opacity-0 group-hover:opacity-100",
            )}
          >
            <GripVertical aria-hidden="true" className="h-3.5 w-3.5 text-fg-subtle" />
          </div>
        </div>
      )}

      {/* Cột phải: Draft Building Workspace */}
      {isDraftOpen && draftReady && (
        <div
          style={isCompact ? undefined : { width: `${100 - splitRatio}%` }}
          className={cn(
            "flex flex-col overflow-hidden",
            isCompact && "absolute inset-0 z-40 bg-surface-panel",
          )}
        >
          <DraftWorkspace
            key={controllerState.draftSessionId}
            client={backendClient}
            draftSessionId={controllerState.draftSessionId!}
            businessFlowData={controllerState.workingState}
            onClose={() => setIsDraftOpen(false)}
            onFeedback={(feedback) => setFeedbackContext(feedback)}
            sampleProjects={sampleProjects}
            onViewSampleProject={handleViewSampleProject}
          />
        </div>
      )}

      <PreDraftSuggestionDialog
        open={isSuggestionOpen}
        onOpenChange={setIsSuggestionOpen}
        onConfirm={handleSuggestionConfirm}
      />
    </div>
  );
}

export default ChatView;
