import { useCallback, useEffect, useState } from "react";
import "../styles/outline.css";
import { ArcsBoard } from "../components/outline/ArcsBoard";
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
    available: false,
    hint: "Giai đoạn 2.2 mở ở sprint 17.4",
  },
  {
    id: "scenes",
    label: "2.3 Phân cảnh",
    available: false,
    hint: "Giai đoạn 2.3 mở ở sprint 17.5",
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
  const locked = !activeStage.available || (selected !== "arcs" && !isCommitted("arcs"));

  function onEvent(event: OutlineEvent) {
    setEvents((previous) => [...previous.slice(-6), event]);
  }

  async function run(stage: string) {
    if (!providerId || !modelId) {
      setMessage("Chọn provider và model trước khi chạy dàn ý.");
      return;
    }
    setBusy(true);
    setEvents([]);
    setMessage("");
    try {
      await runOutline(sessionId, providerId, modelId, stage, onEvent);
      await refresh();
    } catch (reason) {
      setMessage(String(reason));
    } finally {
      setBusy(false);
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
          const disabled = !stage.available || (stage.id !== "arcs" && !isCommitted("arcs"));
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
              {stage.available && stage.id === "arcs" && (
                <button
                  type="button"
                  className="btn"
                  disabled={busy}
                  onClick={() => void run("arcs")}
                >
                  {busy ? "Đang chạy…" : stageStatus("arcs") === "empty" ? "Chạy" : "Chạy lại"}
                </button>
              )}
            </div>
          );
        })}
      </nav>

      <div className="outline-body">
        {locked ? (
          <div className="outline-locked">
            <h2>Giai đoạn chưa mở</h2>
            <p>{activeStage.hint || "Cần chốt giai đoạn trước."}</p>
          </div>
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

        {events.length > 0 && (
          <section className="outline-progress" aria-live="polite">
            <strong>Tiến độ</strong>
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

function describe(event: OutlineEvent): string {
  switch (event.type) {
    case "run_started":
      return "Bắt đầu chạy";
    case "shard_started":
      return `Đang xử lý ${event.key}`;
    case "shard_finished":
      return `${event.key}: ${event.ok ? "hoàn tất" : (event.error ?? "thất bại")}`;
    case "items_ready":
      return `Đã tạo ${event.count} đề xuất`;
    case "run_finished":
      return `Hoàn tất, ${event.failures} lỗi`;
    case "run_failed":
      return event.error;
  }
}
