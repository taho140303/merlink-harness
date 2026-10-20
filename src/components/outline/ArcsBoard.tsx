import { useEffect, useState } from "react";
import { Markdown } from "../Markdown";
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
  /** Bumped by the workspace after every run or commit, to re-read proposals. */
  reloadToken: number;
  onEvent: (event: OutlineEvent) => void;
  onChanged: () => void;
}

const RANGES = [
  { value: "1-10", label: "1-10" },
  { value: "1-30", label: "1-30" },
  { value: "all", label: "Tất cả" },
];

/**
 * Stage 2.1: the arc summaries, and the chapter summaries the author asked for.
 *
 * Proposals arrive one row per arc and one row per chapter, so every card can be
 * accepted, rejected or rewritten on its own.
 */
export function ArcsBoard({
  sessionId,
  providerId,
  modelId,
  busy,
  reloadToken,
  onEvent,
  onChanged,
}: Props) {
  const [items, setItems] = useState<OutlineItem[]>([]);
  const [range, setRange] = useState("1-10");
  const [error, setError] = useState("");
  const [pendingKey, setPendingKey] = useState<string | null>(null);

  useEffect(() => {
    let active = true;
    setError("");
    void Promise.all([
      loadOutlineItems(sessionId, "arcs"),
      loadOutlineItems(sessionId, "chapters"),
    ])
      .then(([arcs, chapters]) => {
        if (active) setItems([...arcs, ...chapters]);
      })
      .catch((reason: unknown) => {
        if (active) setError(String(reason));
      });
    return () => {
      active = false;
    };
  }, [sessionId, reloadToken]);

  const arcs = items.filter((item) => item.lane === "arc_smith");
  const chapters = items.filter((item) => item.lane === "chapter_smith");

  async function decide(
    item: OutlineItem,
    kind: "accept" | "reject" | "regenerate",
    note = "",
  ) {
    const stage = item.lane === "chapter_smith" ? "chapters" : "arcs";
    setPendingKey(item.key);
    setError("");
    try {
      if (kind === "accept") await acceptOutlineItems(sessionId, stage, [item.key]);
      else if (kind === "reject") await rejectOutlineItems(sessionId, stage, [item.key]);
      else {
        await regenerateOutlineItem(
          sessionId,
          providerId,
          modelId,
          stage,
          item.key,
          note,
        );
      }
      onChanged();
    } catch (reason) {
      setError(String(reason));
    } finally {
      setPendingKey(null);
    }
  }

  async function expandChapters() {
    setError("");
    try {
      await runOutline(sessionId, providerId, modelId, "chapters", onEvent, range);
      onChanged();
    } catch (reason) {
      setError(String(reason));
    }
  }

  const rangeSelect = (
    <label className="outline-inline-field">
      Mở rộng chương
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
  );

  return (
    <div className="outline-board">
      {error && (
        <p className="outline-message is-error" role="alert">
          {error}
        </p>
      )}

      <section className="outline-section">
        <header className="outline-section-head">
          <h2>Tóm tắt các cung truyện</h2>
          <span className="outline-count">{arcs.length} arc</span>
        </header>
        {arcs.length === 0 ? (
          <p className="outline-empty">
            Chưa có đề xuất. Bấm Chạy ở cột bên trái để Merlink chia cung truyện.
          </p>
        ) : (
          <div className="outline-arc-list">
            {arcs.map((item) => (
              <ItemCard
                key={item.key}
                item={item}
                busy={busy || pendingKey === item.key}
                onAccept={() => void decide(item, "accept")}
                onReject={() => void decide(item, "reject")}
                onRegenerate={(note) => void decide(item, "regenerate", note)}
              >
                <dl className="outline-facts">
                  <div>
                    <dt>Mục tiêu</dt>
                    <dd>{item.goal ?? "Chưa có"}</dd>
                  </div>
                  <div>
                    <dt>Kết quả</dt>
                    <dd>{item.outcome ?? "Chưa có"}</dd>
                  </div>
                  <div>
                    <dt>Ngân sách</dt>
                    <dd>{item.chapters ?? "?"} chương</dd>
                  </div>
                </dl>
                {item.summarize && (
                  <div className="outline-summary">
                    <Markdown content={item.summarize} />
                  </div>
                )}
              </ItemCard>
            ))}
          </div>
        )}
      </section>

      <section className="outline-section">
        <header className="outline-section-head">
          <h2>Chương</h2>
          <span className="outline-count">{chapters.length} chương</span>
          <div className="outline-section-actions">
            {rangeSelect}
            <button
              type="button"
              className="btn btn-primary"
              disabled={busy || arcs.length === 0}
              onClick={() => void expandChapters()}
            >
              Mở rộng chương
            </button>
          </div>
        </header>
        {chapters.length === 0 ? (
          <p className="outline-empty">
            Chốt các cung truyện trước, rồi mở rộng chương theo khoảng bạn muốn đọc.
          </p>
        ) : (
          <ol className="outline-chapter-list">
            {chapters.map((item) => (
              <li key={item.key}>
                <ItemCard
                  item={item}
                  busy={busy || pendingKey === item.key}
                  onAccept={() => void decide(item, "accept")}
                  onReject={() => void decide(item, "reject")}
                  onRegenerate={(note) => void decide(item, "regenerate", note)}
                >
                  {item.summarize && (
                    <div className="outline-summary">
                      <Markdown content={item.summarize} />
                    </div>
                  )}
                  {item.beats && item.beats.length > 0 && (
                    <div className="outline-chips">
                      {item.beats.map((beat) => (
                        <span className="outline-chip" key={beat}>
                          {beat}
                        </span>
                      ))}
                    </div>
                  )}
                  <p className="outline-meta">
                    {item.scene_count ?? 0} cảnh
                    {item.arc ? ` · arc ${item.arc}` : ""}
                  </p>
                </ItemCard>
              </li>
            ))}
          </ol>
        )}
      </section>
    </div>
  );
}
