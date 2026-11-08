import { useState, useRef, useEffect, useMemo, useCallback } from "react";
import { GripVertical, StopCircle } from "lucide-react";
import { useNavigate } from "react-router";
import { MOCK_PROJECTS } from "@/features/projects/mockProjects";
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
  SAMPLE_BUSINESS_FLOW_DATA,
  fromDraftProgressDto,
  reconcileDraftProgress,
  computeDraftProgress,
  resolveSystemLanguage,
  type DraftProgressState,
  type DraftProgressDto,
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
import type { ChatMessage, ToolCallItem } from "./types";
import type { FeedbackContext } from "@/features/draft/types";
import { useT } from "@/shared/i18n/useT";
import { useLocale } from "@/shared/i18n/store";

// ============================================================================
// Types & Pure State Controller for Chat + Progress Integration
// ============================================================================

export interface ChatBackendState {
  backendStatus: "available" | "unavailable";
  sessionId: string;
  draftSessionId: string | null;
  headVersionSeq: number | null;
  sealedVersion: DraftVersionSummaryDto | null;
  pendingPatchCount: number;
  interruptedTurnAlert: string | null;
  isStreaming: boolean;
  activeTurnId: string | null;
  progress: DraftProgressState;
  messages: ChatMessage[];
  lastError: string | null;
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
      status: tc.status === "error" ? "error" : "success",
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

  constructor(client?: BackendClient, initialConfig?: Partial<ChatBackendState>) {
    this.client = client ?? new BackendClient();
    const isAvail = isTauriEnvironment() || this.client.status === "available";

    this.state = {
      backendStatus: isAvail ? "available" : "unavailable",
      sessionId: initialConfig?.sessionId ?? `sess-${Date.now()}`,
      draftSessionId: initialConfig?.draftSessionId ?? null,
      headVersionSeq: initialConfig?.headVersionSeq ?? null,
      sealedVersion: initialConfig?.sealedVersion ?? null,
      pendingPatchCount: initialConfig?.pendingPatchCount ?? 0,
      interruptedTurnAlert: initialConfig?.interruptedTurnAlert ?? null,
      isStreaming: false,
      activeTurnId: null,
      progress: initialConfig?.progress ?? computeDraftProgress(SAMPLE_BUSINESS_FLOW_DATA),
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
  public async initSession(options?: {
    sessionId?: string;
    projectId?: string;
    trustedLanguage?: string;
  }): Promise<ChatBackendState> {
    if (!isTauriEnvironment() && this.client.status === "unavailable") {
      this.updateState({
        backendStatus: "unavailable",
        lastError: "BACKEND_UNAVAILABLE",
      });
      return this.getState();
    }

    try {
      const initDto = await this.client.createOrLoadChatSession({
        sessionId: options?.sessionId ?? this.state.sessionId,
        projectId: options?.projectId,
      });

      const loadedMessages = (initDto.messages || []).map(mapDtoToChatMessage);
      const reconciledProgress = fromDraftProgressDto(initDto.progress);

      this.updateState({
        backendStatus: "available",
        sessionId: initDto.sessionId,
        draftSessionId: initDto.draftSessionId,
        headVersionSeq: initDto.headVersionSeq ?? null,
        messages: loadedMessages.length > 0 ? loadedMessages : this.state.messages,
        progress: reconciledProgress,
        interruptedTurnAlert: initDto.interruptedTurnAlert ?? null,
        lastError: null,
      });

      return this.getState();
    } catch (err: any) {
      if (err instanceof BackendUnavailableError || err?.code === "provider_unreachable") {
        this.updateState({
          backendStatus: "unavailable",
          lastError: "BACKEND_UNAVAILABLE",
        });
      } else {
        this.updateState({
          lastError: err?.message ?? "Failed to initialize chat session",
        });
      }
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

    // CI-09: Trusted UI settings language strictly takes precedence
    const systemLanguage = resolveSystemLanguage(
      options?.trustedLanguage,
      options?.systemLanguageOverride,
    );

    const userMsgId = `usr-${Date.now()}`;
    const assistantMsgId = `ast-${Date.now()}`;

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
          this.handleStreamEvent(event, assistantMsgId);
        },
      );

      this.updateState({
        isStreaming: false,
        activeTurnId: null,
        messages: this.state.messages.map((m) =>
          m.id === assistantMsgId
            ? {
                ...m,
                isThinking: false,
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

      return response;
    } catch (err: any) {
      // P5S2-N: Provider failure NEVER marks chat turn saved or sealed!
      const errMsg = err?.message ?? "Chat turn execution failed";
      this.updateState({
        isStreaming: false,
        activeTurnId: null,
        lastError: errMsg,
        messages: this.state.messages.map((m) =>
          m.id === assistantMsgId
            ? {
                ...m,
                isThinking: false,
                content: m.content
                  ? `${m.content}\n\n[Lỗi kết nối nhà cung cấp: ${errMsg}]`
                  : `[Không thể lưu lượt trò chuyện do lỗi kết nối: ${errMsg}]`,
              }
            : m,
        ),
        // Notice: sealedVersion remains unchanged/null; never fabricate a sealed version on failure!
      });
      throw err;
    }
  }

  private handleStreamEvent(event: ChatStreamEvent, assistantMsgId: string) {
    switch (event.event) {
      case "turnStarted":
        this.updateState({ activeTurnId: event.data.turnId });
        break;

      case "assistantDelta":
        this.updateState({
          messages: this.state.messages.map((m) =>
            m.id === assistantMsgId
              ? { ...m, content: (m.content || "") + event.data.delta, isThinking: false }
              : m,
          ),
        });
        break;

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
                  status: "success",
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
                      status: event.data.status === "rejected" ? "error" : "success",
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
        // P5S2-N: Provider failure NEVER marks turn saved
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
      await this.initSession({ sessionId: this.state.sessionId });
      return null;
    }

    try {
      const wsState = await this.client.getDraftWorkspaceState({ draftSessionId });
      const pendingCount = wsState.pendingPatches
        ? wsState.pendingPatches.filter((p) => p.status === "pending").length
        : 0;

      this.updateState({
        headVersionSeq: wsState.headVersionSeq ?? null,
        progress: reconcileDraftProgress(this.state.progress, wsState.progress),
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
  draftSessionId,
  initialMessages,
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

  // Pure controller instance
  const controllerRef = useRef<ChatBackendController | null>(null);
  if (!controllerRef.current) {
    controllerRef.current = new ChatBackendController(client, {
      sessionId,
      draftSessionId,
      messages: initialMessages,
    });
  }
  const controller = controllerRef.current;

  const [controllerState, setControllerState] = useState<ChatBackendState>(() =>
    controller.getState(),
  );

  useEffect(() => {
    return controller.subscribe((next) => {
      setControllerState(next);
      if (onProgressUpdate) onProgressUpdate(next.progress);
      if (onVersionSealed && next.sealedVersion) onVersionSealed(next.sealedVersion);
      if (onPendingPatchesChange) onPendingPatchesChange(next.pendingPatchCount);
    });
  }, [controller, onProgressUpdate, onVersionSealed, onPendingPatchesChange]);

  // Load session on mount
  useEffect(() => {
    controller.initSession({
      sessionId,
      trustedLanguage: systemLanguage ?? currentLocale,
    });
  }, [controller, sessionId, systemLanguage, currentLocale]);

  const [isDraftOpen, setIsDraftOpen] = useState(false);
  const sampleProjects = useMemo(
    () => MOCK_PROJECTS.map((p) => ({ id: p.id, title: p.title })),
    [],
  );

  const handleViewSampleProject = (projectId: string) => {
    setIsDraftOpen(false);
    navigate(buildProjectPath(projectId));
  };

  const [isSuggestionOpen, setIsSuggestionOpen] = useState(false);
  const [feedbackContext, setFeedbackContext] = useState<FeedbackContext | null>(null);
  const [keyboardRatio, setKeyboardRatio] = useState<number | null>(null);
  const scrollBottomRef = useRef<HTMLDivElement>(null);
  const scrollContainerRef = useRef<HTMLDivElement>(null);

  const businessFlowData = SAMPLE_BUSINESS_FLOW_DATA;

  const handleToggleDraft = () => {
    if (propOnToggleDraft) {
      propOnToggleDraft();
      return;
    }
    if (isDraftOpen) {
      setIsDraftOpen(false);
    } else {
      setIsSuggestionOpen(true);
    }
  };

  const handleSuggestionConfirm = (suggestion?: string) => {
    setIsSuggestionOpen(false);
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
          progressPercent={controllerState.progress.totalPercent}
          confirmedCount={controllerState.progress.confirmedCount}
          aiInferredCount={controllerState.progress.aiInferredCount}
          totalAttributes={controllerState.progress.totalAttributes}
          readinessGatePassed={controllerState.progress.readinessGatePassed}
          isRequireComplete={controllerState.progress.isRequireComplete}
          backendStatus={controllerState.backendStatus}
          interruptedTurnAlert={controllerState.interruptedTurnAlert}
          onDismissAlert={() => controller.dismissAlert()}
          businessFlowData={businessFlowData}
        />

        {controllerState.backendStatus === "unavailable" && (
          <div
            role="status"
            className="mx-6 mt-3 flex items-center justify-between rounded-lg border border-amber-500/30 bg-amber-500/10 px-3.5 py-2 text-xs text-amber-700 dark:text-amber-300"
          >
            <span>
              {currentLocale === "vi"
                ? "Backend Tauri chưa khả dụng. Dữ liệu thử nghiệm hiện tại không được lưu trữ vào cơ sở dữ liệu thật."
                : "Backend runtime bridge is unavailable. Running in preview mode without durable storage."}
            </span>
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
          {controllerState.isStreaming && (
            <div className="mb-2 flex items-center justify-end">
              <button
                type="button"
                onClick={handleCancel}
                className="focus-ring flex items-center gap-1.5 rounded-lg border border-line bg-surface-card px-2.5 py-1 text-xs font-medium text-fg-subtle hover:bg-surface-muted hover:text-fg"
              >
                <StopCircle className="h-3.5 w-3.5 text-red-500" />
                <span>{currentLocale === "vi" ? "Dừng tạo" : "Stop generating"}</span>
              </button>
            </div>
          )}

          <ChatInputBar
            feedbackContext={feedbackContext}
            onClearFeedback={() => setFeedbackContext(null)}
            onSend={handleSendMessage}
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
      {isDraftOpen && (
        <div
          style={isCompact ? undefined : { width: `${100 - splitRatio}%` }}
          className={cn(
            "flex flex-col overflow-hidden",
            isCompact && "absolute inset-0 z-40 bg-surface-panel",
          )}
        >
          <DraftWorkspace
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
