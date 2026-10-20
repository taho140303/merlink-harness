import { useEffect, useState } from "react";
import { ItemCard } from "./ItemCard";
import {
  acceptOutlineItems,
  loadOutlineItems,
  regenerateOutlineItem,
  rejectOutlineItems,
  runOutline,
} from "../../lib/outline";
import type { OutlineEvent, OutlineItem } from "../../types";

interface Props {
  sessionId: string;
  providerId: string;
  modelId: string;
  busy: boolean;
  reloadToken: number;
  onEvent: (event: OutlineEvent) => void;
  onChanged: () => void;
}

const RANGES = [
  { value: "1-3", label: "Chương 1-3" },
  { value: "1-6", label: "Chương 1-6" },
  { value: "all", label: "Tất cả" },
];

function text(value: unknown): string | undefined {
  return typeof value === "string" && value.trim() !== "" ? value : undefined;
}

function list(value: unknown): unknown[] {
  return Array.isArray(value) ? value : [];
}

function record(value: unknown): Record<string, unknown> {
  return value !== null && typeof value === "object" && !Array.isArray(value)
    ? (value as Record<string, unknown>)
    : {};
}

/**
 * Stage 2.3: one row group per chapter, each scene carrying the preset, the
 * setting, the cast with its point of view, the directives and the writing rules
 * that phase 3 will follow.
 */
export function ScenesBoard({
  sessionId,
  providerId,
  modelId,
  busy,
  reloadToken,
  onEvent,
  onChanged,
}: Props) {
  const [items, setItems] = useState<OutlineItem[]>([]);
  const [range, setRange] = useState("1-3");
  const [error, setError] = useState("");
  const [pendingKey, setPendingKey] = useState<string | null>(null);
  const [selectedIds, setSelectedIds] = useState<Set<string>>(() => new Set());
  const [bulkPending, setBulkPending] = useState(false);
  const [status, setStatus] = useState("");

  useEffect(() => {
    let active = true;
    setError("");
    void loadOutlineItems(sessionId, "scenes")
      .then((value) => {
        if (active) setItems(value);
      })
      .catch((reason: unknown) => {
        if (active) setError(String(reason));
      });
    return () => {
      active = false;
    };
  }, [sessionId, reloadToken]);

  const chapters = new Map<number, OutlineItem[]>();
  for (const item of items) {
    const chapter = Number(record(item.payload).chapter ?? 0);
    const bucket = chapters.get(chapter) ?? [];
    bucket.push(item);
    chapters.set(chapter, bucket);
  }
  const ordered = [...chapters.entries()].sort((a, b) => a[0] - b[0]);
  const reviewable = items.filter((item) => item.status === "pending");
  const selected = reviewable.filter((item) => selectedIds.has(item.id));
  const selectionDisabled = busy || pendingKey !== null || bulkPending;

  async function bulkAccept() {
    const ids = selected.filter((item) => item.status === "pending").map((item) => item.id);
    if (ids.length === 0) return;
    setBulkPending(true);
    setError("");
    setStatus("");
    try {
      const count = await acceptOutlineItems(sessionId, "scenes", ids);
      setSelectedIds(new Set());
      setStatus(`${count} mục đã được duyệt.`);
      onChanged();
    } catch (reason) {
      setError(String(reason));
    } finally {
      setBulkPending(false);
    }
  }

  async function decide(
    item: OutlineItem,
    kind: "accept" | "reject" | "regenerate",
    note = "",
  ) {
    setPendingKey(item.key);
    setError("");
    try {
      if (kind === "accept") await acceptOutlineItems(sessionId, "scenes", [item.id]);
      else if (kind === "reject") await rejectOutlineItems(sessionId, "scenes", [item.id]);
      else await regenerateOutlineItem(sessionId, providerId, modelId, "scenes", item.key, note);
      setSelectedIds((current) => {
        const next = new Set(current);
        next.delete(item.id);
        return next;
      });
      onChanged();
    } catch (reason) {
      setError(String(reason));
    } finally {
      setPendingKey(null);
    }
  }

  async function generate() {
    setError("");
    try {
      await runOutline(sessionId, providerId, modelId, "scenes", onEvent, range);
      onChanged();
    } catch (reason) {
      setError(String(reason));
    }
  }

  return (
    <div className="outline-board">
      {error && (
        <p className="outline-message is-error" role="alert">
          {error}
        </p>
      )}
      {status && <p className="outline-message" role="status">{status}</p>}

      <header className="outline-section-head">
        <h2>Phân cảnh</h2>
        <span className="outline-count">{items.length} cảnh</span>
        <div className="outline-section-actions">
          <span className="outline-count">{selected.length} đã chọn / {reviewable.length} chờ duyệt</span>
          <button type="button" className="btn" disabled={selectionDisabled || reviewable.length === 0}
            onClick={() => setSelectedIds(new Set([...selectedIds, ...reviewable.map((item) => item.id)]))}>
            Chọn tất cả
          </button>
          <button type="button" className="btn" disabled={selectionDisabled || selected.length === 0}
            onClick={() => setSelectedIds(new Set())}>
            Bỏ chọn
          </button>
          <button type="button" className="btn btn-primary" disabled={selectionDisabled || selected.length === 0}
            onClick={() => void bulkAccept()}>
            Duyệt đã chọn ({selected.length})
          </button>
          <label className="outline-inline-field">
            Phạm vi chương
            <select
              className="field"
              value={range}
              disabled={busy}
              onChange={(event) => setRange(event.target.value)}
            >
              {RANGES.map((option) => (
                <option key={option.value} value={option.value}>
                  {option.label}
                </option>
              ))}
            </select>
          </label>
          <button
            type="button"
            className="btn btn-primary"
            disabled={busy}
            onClick={() => void generate()}
          >
            {items.length === 0 ? "Sinh phân cảnh" : "Sinh thêm"}
          </button>
        </div>
      </header>

      {ordered.length === 0 ? (
        <p className="outline-empty">
          Chưa có cảnh nào. Chốt nhân vật và thế giới trước, rồi sinh phân cảnh theo từng chương.
        </p>
      ) : (
        ordered.map(([chapter, scenes]) => (
          <section className="outline-section" key={chapter}>
            <header className="outline-section-head">
              <h3>Chương {chapter}</h3>
              <span className="outline-count">{scenes.length} cảnh</span>
            </header>
            <ol className="outline-chapter-list">
              {scenes.map((item) => {
                const data = record(item.payload);
                const cast = list(data.cast);
                const directives = list(data.directives);
                const rules = list(data.rules);
                const pov = cast.find((entry) => record(entry).is_pov === true);
                return (
                  <li key={item.key}>
                    <label className="outline-selection">
                      <input type="checkbox" checked={selectedIds.has(item.id)}
                        disabled={selectionDisabled || item.status !== "pending"}
                        onChange={(event) => setSelectedIds((current) => {
                          const next = new Set(current);
                          if (event.target.checked) next.add(item.id);
                          else next.delete(item.id);
                          return next;
                        })} />
                      Chọn mục để duyệt
                    </label>
                    <ItemCard
                      item={item}
                      busy={busy || pendingKey === item.key || bulkPending}
                      onAccept={() => void decide(item, "accept")}
                      onReject={() => void decide(item, "reject")}
                      onRegenerate={(note) => void decide(item, "regenerate", note)}
                    >
                      <dl className="outline-facts">
                        <div>
                          <dt>Người kể</dt>
                          <dd>{text(record(pov).character) ?? "Chưa có"}</dd>
                        </div>
                        <div>
                          <dt>Địa điểm</dt>
                          <dd>{text(data.world_setting) ?? "Chưa có"}</dd>
                        </div>
                        <div>
                          <dt>Phong cách</dt>
                          <dd>{text(data.style_preset_code) ?? "Chưa có"}</dd>
                        </div>
                        <div>
                          <dt>Người kể chuyện</dt>
                          <dd>{text(data.narrator_code) ?? "Chưa có"}</dd>
                        </div>
                        <div>
                          <dt>Độ dài</dt>
                          <dd>{Number(data.target_words ?? 0)} từ</dd>
                        </div>
                      </dl>
                      {text(data.summarize) && (
                        <p className="outline-meta">{text(data.summarize)}</p>
                      )}
                      {directives.length > 0 && (
                        <div className="outline-chips">
                          {directives.map((entry, index) => {
                            const directive = record(entry);
                            return (
                              <span className="outline-chip" key={index}>
                                {text(directive.code) ?? "directive"} ·{" "}
                                {text(directive.weight) ?? "-"}
                              </span>
                            );
                          })}
                        </div>
                      )}
                      {rules.length > 0 && (
                        <ul className="outline-detail-list">
                          {rules.map((rule, index) => (
                            <li key={index}>{String(rule)}</li>
                          ))}
                        </ul>
                      )}
                    </ItemCard>
                  </li>
                );
              })}
            </ol>
          </section>
        ))
      )}
    </div>
  );
}
