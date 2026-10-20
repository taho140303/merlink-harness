import { useCallback, useEffect, useState } from "react";
import "../styles/outline.css";
import { ArcsBoard } from "../components/outline/ArcsBoard";
import { CastWorldBoard } from "../components/outline/CastWorldBoard";
import { ScenesBoard } from "../components/outline/ScenesBoard";
import {
  cancelOutline,
  commitOutline,
  getOutlineStatus,
  runOutline,
} from "../lib/outline";
import type { OutlineEvent, OutlineStatus } from "../types";

interface Props {
  sessionId: string;
  providerId: string;
  modelId: string;
  status: OutlineStatus | null;
  onStatus: (status: OutlineStatus) => void;
  /** Leaves the workspace without touching the run; chat comes back. */
  onExit: () => void;
}

interface StageTab {
  id: string;
  label: string;
  /** Stages after 2.1 need a later sprint; they render disabled with a reason. */
  available: boolean;
  hint: string;
}

const STAGES: StageTab[] = [
  { id: "arcs", label: "2.1 Tóm tắt", available: true, hint: "" },
  {
    id: "cast",
    label: "2.2 Nhân vật & Thế giới",
    available: true,
    hint: "Cần chốt giai đoạn chương trước",
  },
  {
    id: "scenes",
    label: "2.3 Phân cảnh",
    available: true,
    hint: "Cần chốt nhân vật và thế giới trước",
  },
];

const STATE_LABEL: Record<string, string> = {
  empty: "Chưa chạy",
  review: "Đề xuất",
  committed: "Đã chốt",
};

/**
 * Phase 2 workspace: the author runs a stage, reads every proposal, and commits
 * only what they approved. Generation happens in the backend; this screen shows
 * the run state and the proposal board.
 */
export function OutlineWorkspace({
  sessionId,
  providerId,
  modelId,
  status,
  onStatus,
  onExit,
}: Props) {
  const [selected, setSelected] = useState("arcs");
  const [busy, setBusy] = useState(false);
  const [events, setEvents] = useState<OutlineEvent[]>([]);
  const [message, setMessage] = useState("");
  const [runLabel, setRunLabel] = useState("");
  /** Bumped after a run or a commit so the board re-reads its proposals. */
  const [reloadToken, setReloadToken] = useState(0);

  const refresh = useCallback(async () => {
    const next = await getOutlineStatus(sessionId);
    onStatus(next);
    setReloadToken((token) => token + 1);
  }, [sessionId, onStatus]);

  useEffect(() => {
    void refresh().catch(() => undefined);
  }, [refresh]);

  const stageStatus = (stage: string) =>
    status?.stages.find((entry) => entry.stage === stage)?.status ?? "empty";

  const isCommitted = (stage: string) => stageStatus(stage) === "committed";

  const activeStage = STAGES.find((stage) => stage.id === selected) ?? STAGES[0];
  const prerequisitesMet = (stage: string) => {
    if (stage === "arcs") return true;
    if (stage === "cast") return isCommitted("chapters");
    if (stage === "scenes") return isCommitted("cast") && isCommitted("world");
    return false;
  };
  const locked = !activeStage.available || !prerequisitesMet(selected);

  function onEvent(event: OutlineEvent) {
    setEvents((previous) => [...previous, event]);
    if (event.type === "run_started") {
      setRunLabel(`Đang chạy ${stageName(event.stage)} · ${event.shards_total} lô tuần tự`);
    } else if (event.type === "shard_started") {
      setRunLabel(`Đang xử lý ${event.key}…`);
    } else if (event.type === "shard_finished") {
      setRunLabel(event.ok
        ? `Đã lưu bản nháp ${event.key}; chuẩn bị lô tiếp theo…`
        : `Lô ${event.key} gặp lỗi; đang tiếp tục các lô còn lại…`);
    } else if (event.type === "run_finished") {
      setRunLabel("Đã hoàn tất sinh đề xuất.");
    } else if (event.type === "run_failed") {
      setRunLabel("Lượt sinh dàn ý thất bại.");
    }
  }

  async function run(stage: string) {
    if (!providerId || !modelId) {
      setMessage("Chọn provider và model trước khi chạy dàn ý.");
      return;
    }
    setBusy(true);
    setEvents([]);
    setRunLabel("Đang chuẩn bị các lô công việc…");
    setMessage("");
    try {
      await runOutline(sessionId, providerId, modelId, stage, onEvent);
      await refresh();
    } catch (reason) {
      setMessage(String(reason));
    } finally {
      setBusy(false);
      setRunLabel("");
    }
  }

  async function commit() {
    setBusy(true);
    setMessage("");
    try {
      const result = await commitOutline(sessionId, selected);
      if (result.missing.length > 0) {
        setMessage(`Đã chốt ${result.committed} mục. Còn thiếu: ${result.missing.join(", ")}`);
      } else if (result.warnings.length > 0) {
        setMessage(`Đã chốt ${result.committed} mục. ${result.warnings.join("; ")}`);
      } else {
        setMessage(`Đã chốt ${result.committed} mục.`);
      }
      await refresh();
    } catch (reason) {
      setMessage(String(reason));
    } finally {
      setBusy(false);
    }
  }

  async function stop() {
    try {
      await cancelOutline(sessionId);
      setMessage("Đã dừng lượt chạy.");
      await refresh();
    } catch (reason) {
      setMessage(String(reason));
    } finally {
      setBusy(false);
    }
  }

  return (
    <div className="outline-workspace">
      <nav className="outline-rail" aria-label="Giai đoạn dàn ý">
        {STAGES.map((stage) => {
          const committed = isCommitted(stage.id);
          const label = committed ? STATE_LABEL.committed : STATE_LABEL[stageStatus(stage.id)];
          const disabled = !stage.available || !prerequisitesMet(stage.id);
          return (
            <div
              key={stage.id}
              className={`outline-stage${selected === stage.id ? " is-active" : ""}${
                disabled ? " is-locked" : ""
              }`}
              title={disabled ? stage.hint : undefined}
            >
              <button
                type="button"
                disabled={disabled}
                aria-current={selected === stage.id}
                onClick={() => setSelected(stage.id)}
              >
                <strong>{stage.label}</strong>
                <span
                  className={`outline-pill${
                    committed ? " is-committed" : label === STATE_LABEL.review ? " is-pending" : ""
                  }`}
                >
                  {label}
                </span>
              </button>
              {stage.available && (stage.id === "arcs" || stage.id === "cast") && (
                <button
                  type="button"
                  className="btn"
                  disabled={busy}
                  title={
                    stage.id === "cast"
                      ? "Sinh nhân vật, thế giới và quan hệ"
                      : "Sinh tóm tắt cung truyện"
                  }
                  onClick={() => void run(stage.id)}
                >
                  {busy ? "Đang chạy…" : stageStatus(stage.id) === "empty" ? "Chạy" : "Chạy lại"}
                </button>
              )}
            </div>
          );
        })}
      </nav>

      <div className="outline-body">
        {(busy || events.length > 0) && (
          <section className="outline-progress" aria-live="polite" aria-atomic="false">
            <strong>{busy ? "Merlink đang làm việc" : "Tiến độ gần nhất"}</strong>
            {runLabel && <p role="status">{runLabel}</p>}
            {(() => {
              const start = [...events].reverse().find((event) => event.type === "run_started");
              const finished = events.filter((event) => event.type === "shard_finished").length;
              return start?.type === "run_started" ? (
                <span>{finished}/{start.shards_total} lô đã xử lý · bản nháp được lưu sau từng lô</span>
              ) : null;
            })()}
            <ul>
              {events.map((event, index) => (
                <li
                  key={`${event.type}-${index}`}
                  className={event.type === "shard_finished" && !event.ok ? "is-failed" : ""}
                >
                  {describe(event)}
                </li>
              ))}
            </ul>
          </section>
        )}
        {locked ? (
          <div className="outline-locked">
            <h2>Giai đoạn chưa mở</h2>
            <p>{activeStage.hint || "Cần chốt giai đoạn trước."}</p>
          </div>
        ) : selected === "scenes" ? (
          <ScenesBoard
            sessionId={sessionId}
            providerId={providerId}
            modelId={modelId}
            busy={busy}
            reloadToken={reloadToken}
            onEvent={onEvent}
            onChanged={() => void refresh()}
          />
        ) : selected === "cast" ? (
          <CastWorldBoard
            sessionId={sessionId}
            providerId={providerId}
            modelId={modelId}
            busy={busy}
            reloadToken={reloadToken}
            onEvent={onEvent}
            onChanged={() => void refresh()}
          />
        ) : (
          <ArcsBoard
            sessionId={sessionId}
            providerId={providerId}
            modelId={modelId}
            busy={busy}
            reloadToken={reloadToken}
            onEvent={onEvent}
            onChanged={() => void refresh()}
          />
        )}

        {message && (
          <p className="outline-message" role="status">
            {message}
          </p>
        )}
      </div>

      <footer className="outline-footer">
        <button
          type="button"
          className="btn btn-primary"
          disabled={busy || locked}
          onClick={() => void commit()}
        >
          Chốt giai đoạn
        </button>
        <button type="button" className="btn" disabled={!busy} onClick={() => void stop()}>
          Dừng
        </button>
        <button type="button" className="btn" onClick={onExit}>
          Về chat
        </button>
      </footer>
    </div>
  );
}

function stageName(stage: string): string {
  return ({ arcs: "tóm tắt", chapters: "chương", cast: "nhân vật", world: "thế giới", scenes: "phân cảnh" } as Record<string, string>)[stage] ?? stage;
}

function describe(event: OutlineEvent): string {
  switch (event.type) {
    case "run_started":
      return `Bắt đầu ${stageName(event.stage)} · ${event.shards_total} lô`;
    case "shard_started":
      return `Đang xử lý ${event.key}`;
    case "shard_finished":
      return `${event.key}: ${event.ok ? "đã lưu bản nháp" : (event.error ?? "thất bại")}`;
    case "items_ready":
      return `Đã tạo ${event.count} đề xuất`;
    case "run_finished":
      return `Hoàn tất, ${event.failures} lỗi`;
    case "run_failed":
      return event.error;
  }
}
