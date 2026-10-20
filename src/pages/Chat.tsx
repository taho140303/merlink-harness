import { useCallback, useEffect, useMemo, useRef, useState, type FormEvent } from "react";
import { useNavigate, useOutletContext, useParams } from "react-router-dom";
import { createAgentEventChannel } from "../lib/tauri";
import { invokeCommand } from "../lib/ipc";
import { getOutlineStatus, runOutline, startOutline as startOutlineRun } from "../lib/outline";
import { OutlineWorkspace } from "./OutlineWorkspace";
import { newMessageId, newSessionId } from "../lib/ids";
import { Markdown } from "../components/Markdown";
import { Dialog } from "../components/Dialog";
import { ContextDonut } from "../components/ContextDonut";
import { SelectBox } from "../components/SelectBox";
import {
  ToolLogDialog,
  type ToolLogEntry,
} from "../components/ToolLogDialog";
import {
  AlertCircle,
  Check,
  ChevronDown,
  ChevronRight,
  Copy,
  PenLine,
  RefreshCw,
  Send,
  Sparkles,
  Wrench,
} from "lucide-react";
import type {
  AgentEvent,
  MessageEnvelope,
  ProviderConfig,
  SuperJsonProgress,
  ContextUsage,
  ToolCall,
  OutlineStatus,
  OutlineEvent,
} from "../types";

/**
 * One step of the live frame (Sprint 11.12 · D1).
 *
 * A new `tool_started` appends a running step; the matching `tool_finished`
 * turns it into "done" or "failed". The frame renders the running step plus the
 * three most recent finished ones.
 */
interface LiveStep {
  key: string;
  label: string;
  state: "running" | "done" | "failed";
}

const WAITING_LABEL = "Đang chờ phản hồi từ model…";

/** D1: a finished step reads `Đã …` instead of `Đang …`. */
function doneLabel(label: string): string {
  return label.startsWith("Đang ") ? `Đã ${label.slice(5)}` : label;
}

/**
 * Folds one streamed `AgentEvent` into the live frame steps (D1).
 *
 * Rounds and tools are both steps, so the frame shows what Merlink is doing right
 * now plus the latest completed work, without any tool chip surviving the turn.
 */
function applyLiveEvent(steps: LiveStep[], event: AgentEvent): LiveStep[] {
  switch (event.type) {
    case "round_started":
      return [
        ...steps,
        { key: `round-${event.round}`, label: "Đang suy nghĩ", state: "running" },
      ];
    case "round_finished":
      return steps.map((step) =>
        step.key === `round-${event.round}` && step.state === "running"
          ? { ...step, label: doneLabel(step.label), state: "done" }
          : step,
      );
    case "tool_started":
      return [
        ...steps,
        { key: `tool-${event.id}`, label: event.status, state: "running" },
      ];
    case "tool_finished":
      return steps.map((step) =>
        step.key === `tool-${event.id}`
          ? {
              ...step,
              label: event.ok ? doneLabel(step.label) : step.label,
              state: event.ok ? "done" : "failed",
            }
          : step,
      );
    default:
      return steps;
  }
}

/** D1: `N giây` under a minute, `M phút S giây` after that. */
function formatElapsed(elapsedMs: number): string {
  const totalSeconds = Math.max(0, Math.floor(elapsedMs / 1000));
  if (totalSeconds < 60) return `${totalSeconds} giây`;
  return `${Math.floor(totalSeconds / 60)} phút ${totalSeconds % 60} giây`;
}

/** The footer keeps its `Tổng: X.Xs` format (D4). */
function formatSeconds(durationMs: number): string {
  return `${(durationMs / 1000).toFixed(1)}s`;
}

/** A tool result reports failure by carrying an `error` key. */
function resultFailed(content: string): boolean {
  try {
    const parsed: unknown = JSON.parse(content);
    return (
      typeof parsed === "object" &&
      parsed !== null &&
      !Array.isArray(parsed) &&
      "error" in (parsed as Record<string, unknown>)
    );
  } catch {
    return false;
  }
}

/**
 * Groups tool calls by the assistant message that closes their turn (D3).
 *
 * Reset on every `role: "user"` message, collect `tool_calls` from assistant
 * messages, match results through `tool_call_id`, and attach the list to the
 * message carrying `meta` (the final answer of that turn). A call with no result
 * message is treated as failed.
 */
function buildToolLogs(messages: MessageEnvelope[]): Map<string, ToolLogEntry[]> {
  const logs = new Map<string, ToolLogEntry[]>();
  let calls: ToolCall[] = [];
  let results = new Map<string, string>();

  for (const message of messages) {
    if (message.role === "user") {
      calls = [];
      results = new Map();
      continue;
    }
    if (message.role === "tool") {
      if (message.tool_call_id) results.set(message.tool_call_id, message.content);
      continue;
    }
    if (message.role !== "assistant") continue;

    if (message.tool_calls?.length) calls.push(...message.tool_calls);
    if (message.meta) {
      if (calls.length > 0) {
        logs.set(
          message.id,
          calls.map((call) => {
            const content = results.get(call.id) ?? null;
            return {
              call,
              result: content,
              ok: content !== null && !resultFailed(content),
            };
          }),
        );
      }
      calls = [];
      results = new Map();
    }
  }

  return logs;
}

function CopyButton({ text }: { text: string }) {
  const [copied, setCopied] = useState(false);
  const [failed, setFailed] = useState(false);
  async function copy() {
    setFailed(false);
    try {
      await navigator.clipboard.writeText(text);
    } catch {
      try {
        const area = document.createElement("textarea"); area.value = text; area.style.position = "fixed"; area.style.opacity = "0";
        document.body.appendChild(area); area.select(); const ok = document.execCommand("copy"); area.remove();
        if (!ok) throw new Error("copy failed");
      } catch { setFailed(true); return; }
    }
    setCopied(true); window.setTimeout(() => setCopied(false), 1500);
  }
  return <span className="chat-copy-wrap"><button type="button" className="chat-copy" aria-label="Sao chép tin nhắn" onClick={() => void copy()}>{copied ? <Check size={14} /> : <Copy size={14} />}</button>{failed && <span role="alert" className="chat-copy-error">Không thể sao chép</span>}</span>;
}

export function Chat() {
  const { providersRevision } = useOutletContext<{ providersRevision: number }>();
  const { sessionId: routeSessionId } = useParams<{ sessionId?: string }>();
  const navigate = useNavigate();
  const [providers, setProviders] = useState<ProviderConfig[]>([]);
  const [selectedProviderId, setSelectedProviderId] = useState<string>("");
  const [selectedModelId, setSelectedModelId] = useState<string>("");
  const [messages, setMessages] = useState<MessageEnvelope[]>([]);
  const [input, setInput] = useState<string>("");
  const [loading, setLoading] = useState<boolean>(false);
  const [error, setError] = useState<string | null>(null);
  const [sessionId, setSessionId] = useState<string>(() => routeSessionId ?? newSessionId());
  /**
   * The project this chat belongs to. Empty until the agent creates one; the
   * backend is the source of truth and every turn reports it back (D1/D4).
   */
  const [projectId, setProjectId] = useState<string>("");
  const [progress, setProgress] = useState<SuperJsonProgress | null>(null);
  const [superJson, setSuperJson] = useState<unknown>(null);
  const [superJsonOpen, setSuperJsonOpen] = useState<boolean>(false);
  /**
   * The dialog's main content (Sprint 11.11 · D5): six paragraphs of Vietnamese
   * prose generated in Rust. The raw document is still one toggle away.
   */
  const [superJsonSummary, setSuperJsonSummary] = useState<string>("");
  const [showRawJson, setShowRawJson] = useState<boolean>(false);
  const [contextUsage, setContextUsage] = useState<ContextUsage | null>(null);
  /**
   * Live frame state (Sprint 11.12 · D1): the steps streamed over the `send_chat`
   * channel, plus the wall clock that drives the "N giây" readout. Everything is
   * cleared when the turn resolves, so the frame leaves no trace.
   */
  const [liveSteps, setLiveSteps] = useState<LiveStep[]>([]);
  const [liveStartedAt, setLiveStartedAt] = useState<number | null>(null);
  const [liveElapsedMs, setLiveElapsedMs] = useState<number>(0);
  /**
   * Model time per final assistant message (D4): the sum of `round_finished`
   * durations. It belongs to the message, not to a global "last turn" state.
   */
  const [turnStats, setTurnStats] = useState<Record<string, number>>({});
  /** Reasoning panel open/closed per message; closed by default (D4). */
  const [reasoningOpen, setReasoningOpen] = useState<Record<string, boolean>>({});
  /** `path -> Vietnamese label` from Rust's `super_json_labels` (D3/D8). */
  const [pathLabels, setPathLabels] = useState<Record<string, string>>({});
  /** The message whose tool log dialog is open, if any (D3). */
  const [toolLogFor, setToolLogFor] = useState<string | null>(null);
  const [outline, setOutline] = useState<OutlineStatus | null>(null);
  /** The workspace replaces the transcript while this is true. */
  const [outlineMode, setOutlineMode] = useState(false);

  const messagesEndRef = useRef<HTMLDivElement>(null);
  const liveFrameRef = useRef<HTMLDivElement>(null);
  const historyIndex = useRef(-1);
  const savedDraft = useRef("");

  // The epoch owns every async read and turn. New chat detaches the old turn;
  // it does not cancel the backend, whose tools may already have run.
  const sessionEpochRef = useRef(0);
  const activeTurnRef = useRef<{ epoch: number; sessionId: string } | null>(null);
  const mountedRef = useRef(true);
  const sessionRef = useRef(sessionId);
  const projectRef = useRef(projectId);
  const providerRef = useRef(selectedProviderId);
  const modelRef = useRef(selectedModelId);
  const loadedRouteRef = useRef<string | null>(routeSessionId ? null : "");
  const isCurrentRead = useCallback((epoch: number, session: string, project: string) =>
    sessionEpochRef.current === epoch &&
    sessionRef.current === session &&
    projectRef.current === project,
  []);
  const isCurrentTurn = useCallback((epoch: number, session: string) =>
    sessionEpochRef.current === epoch &&
    sessionRef.current === session &&
    activeTurnRef.current?.epoch === epoch &&
    activeTurnRef.current.sessionId === session,
  []);

  useEffect(() => {
    mountedRef.current = true;
    return () => {
      mountedRef.current = false;
      sessionEpochRef.current += 1;
      activeTurnRef.current = null;
    };
  }, []);

  const refreshSuperJson = useCallback(async (id: string) => {
    const epoch = sessionEpochRef.current;
    const session = sessionRef.current;
    const project = projectRef.current;
    if (id !== project) return;
    // Without a project there is no super json to read; asking would be an error.
    if (!id) {
      setSuperJson(null);
      return;
    }
    try {
      const document = await invokeCommand("super_json_read", {
        projectId: id,
      });
      if (!isCurrentRead(epoch, session, project)) return;
      setSuperJson(document);
    } catch (err) {
      if (!isCurrentRead(epoch, session, project)) return;
      console.error("super_json_read error:", err);
    }
  }, [isCurrentRead]);

  /**
   * Prose for the `Chi tiết` dialog (D2/D5). Generated in Rust from the super
   * json — the frontend only splits it into paragraphs.
   */
  const refreshSummary = useCallback(async (id: string) => {
    const epoch = sessionEpochRef.current;
    const session = sessionRef.current;
    const project = projectRef.current;
    if (id !== project) return;
    if (!id) {
      setSuperJsonSummary("");
      return;
    }
    try {
      const text = await invokeCommand("super_json_summary", {
        projectId: id,
      });
      if (!isCurrentRead(epoch, session, project)) return;
      setSuperJsonSummary(text);
    } catch (err) {
      if (!isCurrentRead(epoch, session, project)) return;
      console.error("super_json_summary error:", err);
    }
  }, [isCurrentRead]);

  /** Refresh keeps both views current (D5). */
  const refreshDetail = useCallback(
    async (id: string) => {
      await Promise.all([refreshSuperJson(id), refreshSummary(id)]);
    },
    [refreshSuperJson, refreshSummary],
  );

  /** Progress is computed in Rust; the header only renders it (D7). */
  const refreshProgress = useCallback(async () => {
    const epoch = sessionEpochRef.current;
    const session = sessionRef.current;
    const project = projectRef.current;
    try {
      const next = await invokeCommand("super_json_progress", {
        sessionId: session,
      });
      if (!isCurrentRead(epoch, session, project)) return;
      setProgress(next);
    } catch (err) {
      if (!isCurrentRead(epoch, session, project)) return;
      console.error("super_json_progress error:", err);
    }
  }, [isCurrentRead]);

  const refreshContextUsage = useCallback(async () => {
    const epoch = sessionEpochRef.current;
    const session = sessionRef.current;
    const project = projectRef.current;
    const provider = providerRef.current;
    const model = modelRef.current;
    try {
      const next = await invokeCommand("context_usage", { sessionId: session, providerId: provider, modelId: model });
      if (!isCurrentRead(epoch, session, project) || providerRef.current !== provider || modelRef.current !== model) return;
      setContextUsage(next);
    } catch (err) {
      if (!isCurrentRead(epoch, session, project) || providerRef.current !== provider || modelRef.current !== model) return;
      console.error("context_usage error:", err);
    }
  }, [isCurrentRead]);

  useEffect(() => { void refreshContextUsage(); }, [refreshContextUsage, sessionId, selectedProviderId, selectedModelId]);

  useEffect(() => {
    const routeId = routeSessionId ?? "";
    if (loadedRouteRef.current === routeId) return;
    loadedRouteRef.current = routeId;
    if (!routeId) {
      sessionEpochRef.current += 1; activeTurnRef.current = null;
      const fresh = newSessionId(); sessionRef.current = fresh; projectRef.current = "";
      setSessionId(fresh); setProjectId(""); setMessages([]); setError(null); setLoading(false);
      setProgress(null); setSuperJson(null); setSuperJsonSummary("");
      setLiveSteps([]); setLiveStartedAt(null); setLiveElapsedMs(0); setTurnStats({}); setReasoningOpen({}); setToolLogFor(null);
      return;
    }
    let cancelled = false;
    const epoch = ++sessionEpochRef.current;
    activeTurnRef.current = null;
    setLoading(false);
    setError(null);
    setLiveSteps([]); setLiveStartedAt(null); setLiveElapsedMs(0);
    setTurnStats({}); setReasoningOpen({}); setToolLogFor(null);
    void (async () => {
      try {
        const summary = await invokeCommand("get_session", { sessionId: routeId });
        if (cancelled || sessionEpochRef.current !== epoch) return;
        if (!summary) { navigate("/chat", { replace: true }); return; }
        const transcript = await invokeCommand("list_messages", { sessionId: routeId });
        if (cancelled || sessionEpochRef.current !== epoch) return;
        sessionRef.current = routeId; projectRef.current = summary.project_id;
        setSessionId(routeId); setProjectId(summary.project_id); setMessages(transcript);
        setProgress(null); setSuperJson(null); setSuperJsonSummary("");
        void refreshProgress(); void refreshDetail(summary.project_id);
      } catch (err) {
        if (!cancelled) { console.error("Could not load conversation:", err); navigate("/chat", { replace: true }); }
      }
    })();
    return () => { cancelled = true; };
  }, [routeSessionId, navigate]);

  useEffect(() => {
    let cancelled = false;
    void loadProviders(() => cancelled);
    return () => { cancelled = true; };
  }, [providersRevision]);

  // D3/D8: the tool log's Vietnamese labels come from Rust, never from a second
  // hardcoded map in the frontend. Browser mode answers from its mock.
  useEffect(() => {
    let cancelled = false;
    invokeCommand("super_json_labels")
      .then((labels) => {
        if (!cancelled && labels) setPathLabels(labels);
      })
      .catch((err) => console.error("super_json_labels error:", err));
    return () => {
      cancelled = true;
    };
  }, []);

  // D1: tick the live frame's "N giây" readout every 200ms while a turn runs.
  useEffect(() => {
    if (!loading || liveStartedAt === null) return;
    const tick = () => setLiveElapsedMs(Date.now() - liveStartedAt);
    tick();
    const timer = window.setInterval(tick, 200);
    return () => window.clearInterval(timer);
  }, [loading, liveStartedAt]);

  useEffect(() => {
    void refreshProgress();
  }, [refreshProgress, sessionId]);

  // Both dialog views follow the project the session is bound to.
  useEffect(() => {
    void refreshDetail(projectId);
  }, [projectId, refreshDetail]);

  async function loadProviders(isCancelled: () => boolean) {
    try {
      const list = await invokeCommand("load_providers");
      if (!mountedRef.current || isCancelled() || !Array.isArray(list)) return;
      setProviders(list);
      // Keep the live selection when still valid; otherwise prefer a provider
      // with a key (the original first-load rule). An in-flight turn already
      // captured its provider/model in send_chat and is not restarted here.
      const provider = list.find((item) => item.id === providerRef.current)
        ?? list.find((item) => item.api_key.trim().length > 0)
        ?? list[0];
      const modelId = provider?.models.find((model) => model.id === modelRef.current)?.id
        ?? provider?.models.find((model) => model.id === provider.default_model_id)?.id
        ?? provider?.models[0]?.id ?? "";
      providerRef.current = provider?.id ?? "";
      modelRef.current = modelId;
      setSelectedProviderId(providerRef.current);
      setSelectedModelId(modelId);
    } catch (err) {
      if (!isCancelled()) console.error("Error loading providers in Chat:", err);
    }
  }

  function handleProviderChange(providerId: string) {
    providerRef.current = providerId;
    setSelectedProviderId(providerId);
    const provider = providers.find((p) => p.id === providerId);
    if (provider) {
      const modelId = provider.models.find((m) => m.id === provider.default_model_id)?.id ||
        provider.models[0]?.id || "";
      modelRef.current = modelId;
      setSelectedModelId(modelId);
    }
  }

  /**
   * D4: New chat is now owned by the sidebar, which dispatches this event on
   * every page. A stable callback keeps the subscription below from re-binding.
   */
  const handleNewChat = useCallback(() => {
    sessionEpochRef.current += 1;
    activeTurnRef.current = null;
    const nextSession = newSessionId();
    loadedRouteRef.current = "";
    sessionRef.current = nextSession;
    projectRef.current = "";
    setMessages([]);
    setError(null);
    setLoading(false);
    setSessionId(nextSession);
    // A new chat has no project until the agent creates one.
    setProjectId("");
    setProgress(null);
    setSuperJson(null);
    setSuperJsonSummary("");
    setLiveSteps([]);
    setLiveStartedAt(null);
    setLiveElapsedMs(0);
    setTurnStats({});
    setReasoningOpen({});
    setToolLogFor(null);
    historyIndex.current = -1;
    savedDraft.current = "";
  }, []);

  // D4: the sidebar's New chat button, honoured on the already-mounted page.
  // `handleNewChat` is stable (`useCallback` with no deps), so subscribing once
  // is enough and the listener never goes stale.
  useEffect(() => {
    const onNewChat = () => handleNewChat();
    window.addEventListener("merlink:new-chat", onNewChat);
    return () => window.removeEventListener("merlink:new-chat", onNewChat);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  useEffect(() => {
    // D1: the frame lives at the bottom of the transcript, so scroll to it while
    // it exists and to the ordinary anchor once the turn is over.
    (liveFrameRef.current ?? messagesEndRef.current)?.scrollIntoView({
      behavior: "smooth",
    });
  }, [messages, loading]);

  /**
   * The one send path. Both the form and the outline button go through it, so the
   * outline request is an ordinary user message (D8).
   */
  async function sendMessage(raw: string) {
    const content = raw.trim();
    if (!content || activeTurnRef.current !== null) return;
    if (providers.length === 0) {
      setError("Add a provider in Settings before sending a message.");
      return;
    }

    sessionEpochRef.current += 1;
    const epoch = sessionEpochRef.current;
    const turnSessionId = sessionRef.current;
    activeTurnRef.current = { epoch, sessionId: turnSessionId };
    setError(null);
    setLiveSteps([]);
    setLiveStartedAt(Date.now());
    setLiveElapsedMs(0);
    // D4: sum the model time of every round into the final message's stats.
    let modelMs = 0;
    const userMessage: MessageEnvelope = {
      id: newMessageId("msg"),
      role: "user",
      content,
    };

    const nextMessages = [...messages, userMessage];
    setMessages(nextMessages);
    setLoading(true);

    // On a failed turn, only SQLite knows which tool calls/results survived.
    async function reloadSessionMessages(sessionId: string) {
      const stored = await invokeCommand("list_messages", { sessionId });
      if (isCurrentTurn(epoch, sessionId)) setMessages(stored);
    }

    // D1: register the channel *before* invoking, so no early event is missed.
    // Events arrive while the command runs and grow the live frame incrementally.
    const channel = createAgentEventChannel((event: AgentEvent) => {
      if (!isCurrentTurn(epoch, turnSessionId)) return;
      if (event.type === "round_finished") modelMs += event.duration_ms;
      setLiveSteps((previous) => applyLiveEvent(previous, event));
    });

    try {
      // One turn can append several messages: the assistant's tool request, the
      // tool results, then the final answer. `project_id` is the binding the
      // backend resolved, which may have been created by this very turn.
      const turn = await invokeCommand("send_chat", {
        providerId: selectedProviderId,
        modelId: selectedModelId,
        messages: nextMessages,
        projectId,
        sessionId: turnSessionId,
        channel,
      });
      if (!isCurrentTurn(epoch, turnSessionId)) return;
      projectRef.current = turn.project_id;
      setMessages((previous) => [...previous, ...turn.messages]);
      setProjectId(turn.project_id);
      // D4: store the model time against the final message that carries `meta`,
      // falling back to its measured latency on the no-event (demo) path.
      const final = [...turn.messages].reverse().find((message) => message.meta);
      if (final?.meta) {
        const meta = final.meta;
        setTurnStats((previous) => ({
          ...previous,
          [final.id]: modelMs > 0 ? modelMs : meta.latency_ms,
        }));
      }
      void refreshDetail(turn.project_id);
      void refreshProgress();
      loadedRouteRef.current = turnSessionId;
      if (routeSessionId !== turnSessionId) navigate(`/chat/${encodeURIComponent(turnSessionId)}`, { replace: true });
    } catch (err: unknown) {
      if (!isCurrentTurn(epoch, turnSessionId)) return;
      console.error("send_chat error:", err);
      setError(
        typeof err === "string"
          ? err
          : err instanceof Error
            ? err.message
            : "Failed to send chat message.",
      );
      try {
        await reloadSessionMessages(turnSessionId);
      } catch (reloadError) {
        // Keep the turn error and the current transcript when recovery fails.
        if (isCurrentTurn(epoch, turnSessionId)) console.error("list_messages error:", reloadError);
      }
      if (!isCurrentTurn(epoch, turnSessionId)) return;
      try {
        const next = await invokeCommand("super_json_progress", {
          sessionId: turnSessionId,
        });
        if (!isCurrentTurn(epoch, turnSessionId)) return;
        projectRef.current = next.project_id;
        setProjectId(next.project_id);
        setProgress(next);
        void refreshDetail(next.project_id);
      } catch (progressError) {
        if (isCurrentTurn(epoch, turnSessionId)) console.error("super_json_progress error:", progressError);
      }
    } finally {
      window.dispatchEvent(new Event("merlink:sessions-changed"));
      if (isCurrentTurn(epoch, turnSessionId)) {
        activeTurnRef.current = null;
        setLoading(false);
        // D1/D2: the turn is over (success or error), so the live frame disappears
        // completely; the transcript keeps only prose plus the message-level tool
        // log behind the wrench button (D3).
        setLiveSteps([]);
        setLiveStartedAt(null);
        setLiveElapsedMs(0);
        void refreshContextUsage();
      }
    }
  }

  useEffect(() => {
    let cancelled = false;
    void getOutlineStatus(sessionId).then((next) => { if (cancelled) return; setOutline(next.active ? next : null); setOutlineMode(next.active); }).catch(() => { if (!cancelled) { setOutline(null); setOutlineMode(false); } });
    return () => { cancelled = true; };
  }, [sessionId, projectId]);

  async function handleStartOutline() {
    if (!projectId || !selectedProviderId || !selectedModelId) return;
    setLoading(true);
    try {
      const started = await startOutlineRun(projectId, sessionId);
      setOutline(started);
      setOutlineMode(true);
      await runOutline(sessionId, selectedProviderId, selectedModelId, "arcs", (_event: OutlineEvent) => undefined);
      const current = await getOutlineStatus(sessionId);
      setOutline(current);
    } catch (err) { setError(String(err)); }
    finally { setLoading(false); }
  }

  async function handleSend(event?: FormEvent) {
    event?.preventDefault();
    const content = input.trim();
    if (!content || activeTurnRef.current !== null) return;
    if (providers.length === 0) {
      setError("Add a provider in Settings before sending a message.");
      return;
    }
    setInput("");
    historyIndex.current = -1;
    savedDraft.current = "";
    await sendMessage(content);
  }

  // 0 → 70% is the required half of the bar, 70 → 100% the optional half.
  const requiredFill = progress ? (Math.min(progress.percent, 70) / 70) * 100 : 0;
  const optionalFill = progress ? (Math.max(0, progress.percent - 70) / 30) * 100 : 0;
  // D8: the outline button only lights up once every required field is filled.
  const canOutline =
    !!progress &&
    progress.required_total > 0 &&
    progress.filled_required === progress.required_total;

  const currentProvider = providers.find((p) => p.id === selectedProviderId);
  const currentModel = currentProvider?.models.find(
    (m) => m.id === selectedModelId,
  );

  // D3: tool calls grouped per finished turn, keyed by the message carrying `meta`.
  const toolLogs = useMemo(() => buildToolLogs(messages), [messages]);
  const openToolLog = toolLogFor ? (toolLogs.get(toolLogFor) ?? []) : [];
  // D1: what the frame shows right now, and the three latest finished steps.
  const runningStep = [...liveSteps].reverse().find((step) => step.state === "running");
  // Only tool phases are listed as finished work: a finished "Đã suy nghĩ" round
  // repeats what the header already says and would crowd out the recorded fields.
  const completedSteps = liveSteps
    .filter((step) => step.state !== "running" && step.key.startsWith("tool-"))
    .slice(-3);

  return (
    <div
      style={{
        display: "flex",
        flexDirection: "column",
        height: "calc(100vh - 80px)",
        minHeight: "520px",
      }}
    >
      <div className="page-topbar" style={{ marginBottom: "18px", flex: "none" }}>
        <span className="page-breadcrumb">
          Workspace / <strong>Chat</strong>
        </span>

        <div className="page-topbar-actions">
          <div className="chat-progress">
            <div
              className="chat-progress-track"
              role="progressbar"
              aria-label="Super json progress"
              aria-valuemin={0}
              aria-valuemax={100}
              aria-valuenow={progress?.percent ?? 0}
            >
              <span className="chat-progress-segment is-required">
                <span
                  className="chat-progress-fill"
                  style={{ width: `${requiredFill}%` }}
                />
              </span>
              <span className="chat-progress-segment is-optional">
                <span
                  className="chat-progress-fill"
                  style={{ width: `${optionalFill}%` }}
                />
              </span>
            </div>
            <span className="chat-progress-caption">
              <strong className="chat-progress-label">
                {progress?.label ?? "…"}
              </strong>
              <span className="chat-progress-count">
                {progress?.filled_required ?? 0}/{progress?.required_total ?? 12}
                {" · "}
                {progress?.percent ?? 0}%
              </span>
            </span>
          </div>

          <button type="button" className="btn" aria-expanded={superJsonOpen} onClick={() => setSuperJsonOpen(true)}>
            <span>Chi tiết</span>
          </button>

          {/* D5/D6: order is Chi tiết → provider → model → Viết dàn ý, and both
              selects now wear the project's design system. D3: an empty list is
              a real state, not a broken one. */}
          <SelectBox
            aria-label="Provider"
            value={selectedProviderId}
            disabled={providers.length === 0}
            onChange={(event) => handleProviderChange(event.target.value)}
          >
            {providers.length === 0 ? (
              <option value="" disabled>
                Chưa cấu hình provider
              </option>
            ) : (
              providers.map((provider) => (
                <option key={provider.id} value={provider.id}>
                  {provider.name}
                </option>
              ))
            )}
          </SelectBox>

          <SelectBox
            aria-label="Model"
            value={selectedModelId}
            disabled={!currentProvider}
            onChange={(event) => { modelRef.current = event.target.value; setSelectedModelId(event.target.value); }}
          >
            {currentProvider?.models.length ? (
              currentProvider.models.map((model) => (
                <option key={model.id} value={model.id}>
                  {model.name || model.id}
                </option>
              ))
            ) : (
              <option value="" disabled>
                Chưa có model
              </option>
            )}
          </SelectBox>

          {outline ? (
            <button
              type="button"
              className="btn"
              title="Quay lại bàn dàn ý"
              disabled={loading}
              onClick={() => setOutlineMode(true)}
            >
              <PenLine size={16} />
              <span>Mở dàn ý</span>
            </button>
          ) : (
            <button
              type="button"
              className="btn"
              title={
                canOutline
                  ? "Bắt đầu dàn ý cho dự án này"
                  : "Cần điền đủ 11 trường bắt buộc trước"
              }
              disabled={!canOutline || loading}
              onClick={() => void handleStartOutline()}
            >
              <PenLine size={16} />
              <span>Viết dàn ý</span>
            </button>
          )}
        </div>
      </div>

      <div
        style={{
          position: "relative",
          display: "flex",
          flex: 1,
          minHeight: 0,
          marginBottom: "16px",
        }}
      >
        <div
          className="surface"
          role="log"
          aria-live="polite"
          aria-label="Chat conversation"
          style={{
            flex: 1,
            minHeight: 0,
            overflowY: "auto",
            padding: "26px",
            display: "flex",
            flexDirection: "column",
            gap: "18px",
          }}
        >
          {outlineMode ? <OutlineWorkspace sessionId={sessionId} providerId={selectedProviderId} modelId={selectedModelId} status={outline} onStatus={setOutline} onExit={() => setOutlineMode(false)} /> : <>
          {messages.length === 0 ? (
            <div
              style={{
                margin: "auto",
                textAlign: "center",
                color: "var(--muted)",
                maxWidth: "420px",
              }}
            >
              <div
                style={{
                  display: "inline-grid",
                  placeItems: "center",
                  width: "52px",
                  height: "52px",
                  borderRadius: "16px",
                  background: "var(--accent-soft)",
                  color: "var(--accent-strong)",
                  marginBottom: "14px",
                }}
              >
                <Sparkles size={24} />
              </div>
              <h3
                style={{
                  margin: "0 0 6px",
                  fontSize: "18px",
                  fontWeight: 700,
                  letterSpacing: "-.03em",
                  color: "var(--ink)",
                }}
              >
                Start writing together
              </h3>
              <p style={{ margin: 0, fontSize: "13px", lineHeight: 1.6 }}>
                Ask a question, outline a scene, or test ideas with{" "}
                {currentModel?.name || "your chosen model"}.
              </p>
            </div>
          ) : (
            messages.map((message, index) => {
              // D2: tool messages and tool-only assistant messages leave no
              // trace in the transcript after the turn is over.
              if (message.role === "tool" || message.role === "system") return null;

              const isUser = message.role === "user";
              const hasContent = message.content.trim().length > 0;
              if (!isUser && !hasContent) return null;

              const hasReasoning =
                !isUser && Boolean(message.reasoning?.trim());
              const reasoningShown = Boolean(reasoningOpen[message.id]);
              const modelMs = message.meta
                ? (turnStats[message.id] ?? message.meta.latency_ms)
                : 0;
              const logEntries = toolLogs.get(message.id);
              const hasToolLog = Boolean(logEntries && logEntries.length > 0);

              return (
                <div
                  key={message.id || index}
                  className="chat-message"
                  style={{
                    display: "flex",
                    flexDirection: "column",
                    alignItems: isUser ? "flex-end" : "flex-start",
                    maxWidth: "100%",
                  }}
                >
                  {/* D4: "Đã suy nghĩ N giây" above the bubble of every message
                      that closes a turn; the chevron only exists when the model
                      actually produced reasoning. */}
                  {!isUser && message.meta && (
                    <div className="chat-think">
                      {hasReasoning ? (
                        <button
                          type="button"
                          className="chat-think-toggle"
                          aria-expanded={reasoningShown}
                          aria-controls={`chat-reasoning-${message.id}`}
                          onClick={() =>
                            setReasoningOpen((previous) => ({
                              ...previous,
                              [message.id]: !previous[message.id],
                            }))
                          }
                        >
                          {reasoningShown ? (
                            <ChevronDown size={13} />
                          ) : (
                            <ChevronRight size={13} />
                          )}
                          <span>
                            Đã suy nghĩ {Math.max(1, Math.round(modelMs / 1000))} giây
                          </span>
                        </button>
                      ) : (
                        <span className="chat-think-label">
                          Đã suy nghĩ {Math.max(1, Math.round(modelMs / 1000))} giây
                        </span>
                      )}
                    </div>
                  )}

                  {hasReasoning && reasoningShown && (
                    <div
                      className="chat-think-panel"
                      id={`chat-reasoning-${message.id}`}
                      data-testid="chat-reasoning-panel"
                    >
                      {message.reasoning}
                    </div>
                  )}

                  <div
                    style={{
                      maxWidth: "76%",
                      padding: "12px 16px",
                      borderRadius: isUser
                        ? "16px 16px 4px 16px"
                        : "16px 16px 16px 4px",
                      background: isUser
                        ? "var(--accent)"
                        : "var(--glass-soft)",
                      border: isUser ? "1px solid transparent" : "1px solid var(--line)",
                      color: isUser ? "#fff" : "var(--ink)",
                      fontSize: "14px",
                      lineHeight: 1.65,
                      wordBreak: "break-word",
                    }}
                  >
                    {isUser ? <span style={{ whiteSpace: "pre-wrap" }}>{message.content}</span> : <Markdown content={message.content} />}
                  </div>

                  <div className="chat-message-footer">
                    <CopyButton text={message.content} />
                    {/* D3: the wrench sits next to Copy, only on a turn that
                        actually called a tool. */}
                    {!isUser && hasToolLog && (
                      <button
                        type="button"
                        className="chat-copy chat-tools"
                        aria-label="Công cụ đã gọi"
                        onClick={() => setToolLogFor(message.id)}
                      >
                        <Wrench size={14} />
                      </button>
                    )}
                    {!isUser && message.meta && <span className="chat-time" title={`${message.meta.model} · ${message.meta.provider} · ${(message.meta.latency_ms / 1000).toFixed(1)}s${message.meta.tokens ? ` · ${message.meta.tokens.input_tokens} in / ${message.meta.tokens.output_tokens} out` : ""}`}>Tổng: {formatSeconds(message.meta.latency_ms)}</span>}
                  </div>
                </div>
              );
            })
          )}

          {error && (
            <div
              role="alert"
              style={{
                display: "flex",
                alignItems: "flex-start",
                gap: "9px",
                padding: "12px 15px",
                borderRadius: "12px",
                background: "rgba(198,91,105,.12)",
                border: "1px solid rgba(198,91,105,.35)",
                color: "var(--error)",
                fontSize: "13px",
                lineHeight: 1.5,
              }}
            >
              <AlertCircle size={16} />
              <span>{error}</span>
            </div>
          )}

          <div ref={messagesEndRef} />

          {/* D1: the live frame is the LAST child of [role="log"], so it lives
              inside the scrolling transcript, left-aligned and as wide as an
              assistant bubble. It exists only while `loading` is true. */}
          {loading && (
            <div
              ref={liveFrameRef}
              className="chat-live-frame"
              data-testid="chat-live-frame"
              role="status"
              aria-live="polite"
              aria-label="Agent progress"
            >
              <div className="chat-live-frame-head">
                <span className="chat-phase-spinner" aria-hidden="true" />
                <span className="chat-live-frame-label">
                  {runningStep?.label ?? WAITING_LABEL}
                </span>
                <span className="chat-live-frame-time">
                  {formatElapsed(liveElapsedMs)}
                </span>
              </div>
              {completedSteps.length > 0 && (
                <ul className="chat-live-frame-steps">
                  {completedSteps.map((step) => (
                    <li
                      key={step.key}
                      className={`chat-live-frame-step${step.state === "failed" ? " is-failed" : ""}`}
                    >
                      <span className="chat-live-frame-mark" aria-hidden="true">
                        {step.state === "failed" ? "✕" : "✓"}
                      </span>
                      <span>{step.label}</span>
                    </li>
                  ))}
                </ul>
              )}
            </div>
          )}
          </>}
        </div>
      </div>

      {/* D1: exactly three parts on one line — input (flex:1), the context
          donut, then Send. The donut is a flex item now, not an overlay. */}
      <form className="chat-input-row" onSubmit={handleSend}>
        <input
          className="field chat-input"
          type="text"
          value={input}
          placeholder={outline?.active ? "Trao đổi thêm với Merlink" : "Nhập tin nhắn..."}
          aria-label="Your message"
          disabled={loading}
          onChange={(event) => { setInput(event.target.value); if (historyIndex.current === -1) savedDraft.current = event.target.value; }}
          onKeyDown={(event) => {
            if (event.key === "Enter" && providers.length === 0 && input.trim()) {
              event.preventDefault();
              setError("Add a provider in Settings before sending a message.");
              return;
            }
            if (event.key === "ArrowUp") {
              event.preventDefault();
              const sent = messages.filter((m) => m.role === "user").map((m) => m.content);
              if (!sent.length) return;
              if (historyIndex.current === -1) savedDraft.current = input;
              historyIndex.current = Math.min(historyIndex.current + 1, sent.length - 1);
              setInput(sent[sent.length - 1 - historyIndex.current]);
            } else if (event.key === "ArrowDown" && historyIndex.current >= 0) {
              event.preventDefault();
              historyIndex.current -= 1;
              setInput(historyIndex.current < 0 ? savedDraft.current : (messages.filter((m) => m.role === "user").map((m) => m.content)[messages.filter((m) => m.role === "user").length - 1 - historyIndex.current] ?? ""));
            }
          }}
        />
        <ContextDonut usage={contextUsage} />
        <button
          type="submit"
          className="btn btn-primary chat-send"
          disabled={!input.trim() || loading || providers.length === 0}
        >
          <Send size={16} />
          <span>Send</span>
        </button>
      </form>
      {/* D3: an empty provider list explains the disabled Send instead of
          leaving the author guessing. */}
      {providers.length === 0 && (
        <p className="chat-input-hint">
          Thêm provider trong Settings (góc dưới bên trái) để bắt đầu.
        </p>
      )}
      <Dialog open={superJsonOpen} onClose={() => setSuperJsonOpen(false)} title={`super.json · project ${projectId || "— chưa có project —"}`} wide>
        <div className="chat-detail-toolbar">
          <button
            type="button"
            className="btn chat-detail-json-toggle"
            aria-pressed={showRawJson}
            onClick={() => setShowRawJson((value) => !value)}
          >
            <span>JSON</span>
          </button>
          <button type="button" className="btn" onClick={() => void refreshDetail(projectId)}><RefreshCw size={14} /><span>Refresh</span></button>
        </div>
        {showRawJson ? (
          <pre className="chat-detail-json" data-testid="super-json-raw">
            {superJson === null ? (projectId ? "Chưa đọc được super json." : "Chưa có project — hãy gửi tin nhắn đầu tiên để agent tạo project.") : JSON.stringify(superJson, null, 2)}
          </pre>
        ) : superJsonSummary.trim() ? (
          <div className="chat-detail-summary" data-testid="super-json-summary">
            {superJsonSummary.split("\n\n").map((paragraph, index) => (
              <p key={index}>{paragraph}</p>
            ))}
          </div>
        ) : (
          <p className="chat-detail-empty">
            {projectId
              ? "Chưa đọc được tóm tắt."
              : "Chưa có project — hãy gửi tin nhắn đầu tiên để agent tạo project."}
          </p>
        )}
      </Dialog>
      <ToolLogDialog
        open={toolLogFor !== null}
        onClose={() => setToolLogFor(null)}
        entries={openToolLog}
        labels={pathLabels}
      />
    </div>
  );
}
