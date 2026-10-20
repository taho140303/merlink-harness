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

function text(value: unknown): string | undefined {
  return typeof value === "string" && value.trim() !== "" ? value : undefined;
}

function number(value: unknown): number | undefined {
  return typeof value === "number" && Number.isFinite(value) ? value : undefined;
}

function list(value: unknown): unknown[] {
  return Array.isArray(value) ? value : [];
}

function record(value: unknown): Record<string, unknown> {
  return value !== null && typeof value === "object" && !Array.isArray(value)
    ? (value as Record<string, unknown>)
    : {};
}

function payloadOf(item: OutlineItem): Record<string, unknown> {
  return record(item.payload);
}

const SENSES: Array<[string, string]> = [
  ["visual", "Thị giác"],
  ["auditory", "Thính giác"],
  ["tactile", "Xúc giác"],
  ["olfactory", "Khứu giác"],
  ["gustatory", "Vị giác"],
];

const STATE_LABELS: Array<[string, string]> = [
  ["trust", "Tin"],
  ["affection", "Mến"],
  ["hostility", "Địch"],
  ["fear", "Sợ"],
  ["respect", "Nể"],
  ["familiarity", "Quen"],
  ["dependency", "Lệ thuộc"],
];

const WORLD_KIND_LABEL: Record<string, string> = {
  setting: "Địa điểm",
  lore: "Tri thức",
  power: "Hệ thống sức mạnh",
};

/**
 * Stage 2.2: the cast lane (characters and their inner state) next to the world
 * lane (settings, lore, power systems), with the relationship proposals below.
 *
 * Proposals arrive one row per character, per world entity and per bond, so each
 * card can be accepted or rewritten on its own before the stage is committed.
 */
export function CastWorldBoard({
  sessionId,
  providerId,
  modelId,
  busy,
  reloadToken,
  onEvent,
  onChanged,
}: Props) {
  const [items, setItems] = useState<OutlineItem[]>([]);
  const [error, setError] = useState("");
  const [pendingKey, setPendingKey] = useState<string | null>(null);
  const [selectedIds, setSelectedIds] = useState<Set<string>>(() => new Set());
  const [bulkPending, setBulkPending] = useState(false);
  const [status, setStatus] = useState("");

  useEffect(() => {
    let active = true;
    setError("");
    void Promise.all([
      loadOutlineItems(sessionId, "cast"),
      loadOutlineItems(sessionId, "world"),
    ])
      .then(([cast, world]) => {
        if (active) setItems([...cast, ...world]);
      })
      .catch((reason: unknown) => {
        if (active) setError(String(reason));
      });
    return () => {
      active = false;
    };
  }, [sessionId, reloadToken]);

  const characters = items.filter((item) => item.lane === "cast_smith");
  const world = items.filter((item) => item.lane === "world_smith");
  const bonds = items.filter((item) => item.lane === "bond_smith");

  function reviewControls(list: OutlineItem[], stage: "cast" | "world") {
    const reviewable = list.filter((item) => item.status === "pending");
    const selected = reviewable.filter((item) => selectedIds.has(item.id));
    const disabled = busy || pendingKey !== null || bulkPending;
    return (
      <div className="outline-section-actions">
        <span className="outline-count">{selected.length} đã chọn / {reviewable.length} chờ duyệt</span>
        <button type="button" className="btn" disabled={disabled || reviewable.length === 0}
          onClick={() => setSelectedIds((current) => new Set([...current, ...reviewable.map((item) => item.id)]))}>
          Chọn tất cả
        </button>
        <button type="button" className="btn" disabled={disabled || selected.length === 0}
          onClick={() => setSelectedIds((current) => {
            const next = new Set(current);
            reviewable.forEach((item) => next.delete(item.id));
            return next;
          })}>
          Bỏ chọn
        </button>
        <button type="button" className="btn btn-primary" disabled={disabled || selected.length === 0}
          onClick={() => void bulkAccept(stage, selected)}>
          Duyệt đã chọn ({selected.length})
        </button>
      </div>
    );
  }

  async function bulkAccept(stage: "cast" | "world", selected: OutlineItem[]) {
    const ids = selected.filter((item) => item.status === "pending").map((item) => item.id);
    if (ids.length === 0) return;
    setBulkPending(true);
    setError("");
    setStatus("");
    try {
      const count = await acceptOutlineItems(sessionId, stage, ids);
      setSelectedIds(new Set());
      setStatus(`${count} mục đã được duyệt.`);
      onChanged();
    } catch (reason) {
      setError(String(reason));
    } finally {
      setBulkPending(false);
    }
  }

  function selection(item: OutlineItem) {
    return (
      <label className="outline-selection">
        <input type="checkbox" checked={selectedIds.has(item.id)}
          disabled={busy || pendingKey !== null || bulkPending || item.status !== "pending"}
          onChange={(event) => setSelectedIds((current) => {
            const next = new Set(current);
            if (event.target.checked) next.add(item.id);
            else next.delete(item.id);
            return next;
          })} />
        Chọn mục để duyệt
      </label>
    );
  }

  const nameOf = new Map<string, string>();
  for (const item of characters) {
    nameOf.set(item.key, text(payloadOf(item).name) ?? item.key);
  }

  async function decide(
    item: OutlineItem,
    kind: "accept" | "reject" | "regenerate",
    note = "",
  ) {
    const stage = item.lane === "cast_smith" ? "cast" : "world";
    setPendingKey(item.key);
    setError("");
    try {
      if (kind === "accept") await acceptOutlineItems(sessionId, stage, [item.id]);
      else if (kind === "reject") await rejectOutlineItems(sessionId, stage, [item.id]);
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

  async function expandWorld() {
    setError("");
    try {
      await runOutline(sessionId, providerId, modelId, "world", onEvent);
      onChanged();
    } catch (reason) {
      setError(String(reason));
    }
  }

  const actions = (item: OutlineItem) => ({
    busy: busy || pendingKey === item.key || bulkPending,
    onAccept: () => void decide(item, "accept"),
    onReject: () => void decide(item, "reject"),
    onRegenerate: (note: string) => void decide(item, "regenerate", note),
  });

  return (
    <div className="outline-board">
      {error && (
        <p className="outline-message is-error" role="alert">
          {error}
        </p>
      )}
      {status && <p className="outline-message" role="status">{status}</p>}

      <div className="outline-columns">
        <section className="outline-section">
          <header className="outline-section-head">
            <h2>Nhân vật</h2>
            <span className="outline-count">{characters.length} đề xuất</span>
            {reviewControls(characters, "cast")}
          </header>
          {characters.length === 0 ? (
            <p className="outline-empty">
              Chưa có nhân vật nào. Chạy giai đoạn Nhân vật ở cột bên trái.
            </p>
          ) : (
            <div className="outline-arc-list">
              {characters.map((item) => {
                const data = payloadOf(item);
                const appearance = record(data.appearance);
                const competences = list(data.competences);
                const memories = list(data.memories);
                const motifs = list(data.motifs);
                return (
                  <div key={item.key}>
                    {selection(item)}
                    <ItemCard item={item} {...actions(item)}>
                    <p className="outline-meta">
                      {[text(data.role), text(data.archetype)]
                        .filter(Boolean)
                        .join(" · ") || "Chưa có vai trò"}
                      {data.is_primary === true ? " · nhân vật chính" : ""}
                    </p>
                    {text(data.summary) && <p className="outline-meta">{text(data.summary)}</p>}
                    <dl className="outline-facts">
                      <div>
                        <dt>Động lực</dt>
                        <dd>{text(data.motivation) ?? text(data.want) ?? "Chưa có"}</dd>
                      </div>
                      <div>
                        <dt>Điểm yếu</dt>
                        <dd>{text(data.flaw) ?? "Chưa có"}</dd>
                      </div>
                    </dl>
                    {Object.keys(appearance).length > 0 && (
                      <p className="outline-meta">
                        Ngoại hình:{" "}
                        {[
                          text(appearance.hair) && `tóc ${text(appearance.hair)}`,
                          text(appearance.eyes) && `mắt ${text(appearance.eyes)}`,
                          text(appearance.outfit_style),
                          number(appearance.height) &&
                            `${number(appearance.height)} ${text(appearance.height_unit) ?? ""}`,
                        ]
                          .filter(Boolean)
                          .join(", ") || "chưa mô tả"}
                      </p>
                    )}
                    {motifs.length > 0 && (
                      <div className="outline-chips">
                        {motifs.map((motif) => (
                          <span className="outline-chip" key={String(motif)}>
                            {String(motif)}
                          </span>
                        ))}
                      </div>
                    )}
                    {competences.length > 0 && (
                      <ul className="outline-detail-list">
                        {competences.map((entry, index) => {
                          const skill = record(entry);
                          return (
                            <li key={index}>
                              {text(skill.name) ?? "Năng lực"}
                              {text(skill.level) ? ` · ${text(skill.level)}` : ""}
                            </li>
                          );
                        })}
                      </ul>
                    )}
                    {memories.length > 0 && (
                      <ul className="outline-detail-list">
                        {memories.map((entry, index) => (
                          <li key={index}>{text(record(entry).summary) ?? "Ký ức"}</li>
                        ))}
                      </ul>
                    )}
                    </ItemCard>
                  </div>
                );
              })}
            </div>
          )}
        </section>

        <section className="outline-section">
          <header className="outline-section-head">
            <h2>Thế giới</h2>
            <span className="outline-count">{world.length} đề xuất</span>
            {reviewControls(world, "world")}
            <div className="outline-section-actions">
              <button
                type="button"
                className="btn"
                disabled={busy || characters.length === 0}
                title="Sinh địa điểm, tri thức, hệ thống sức mạnh và quan hệ"
                onClick={() => void expandWorld()}
              >
                {world.length === 0 ? "Sinh thế giới" : "Sinh lại thế giới"}
              </button>
            </div>
          </header>
          {world.length === 0 ? (
            <p className="outline-empty">
              Chưa có địa điểm hay tri thức nào. Chạy giai đoạn Thế giới ở cột bên trái.
            </p>
          ) : (
            <div className="outline-arc-list">
              {world.map((item) => {
                const data = payloadOf(item);
                const kind = text(data.kind) ?? "setting";
                const sensory = record(data.sensory);
                return (
                  <div key={item.key}>
                    {selection(item)}
                    <ItemCard item={item} {...actions(item)}>
                    <p className="outline-meta">
                      {WORLD_KIND_LABEL[kind] ?? kind}
                      {text(data.type) ? ` · ${text(data.type)}` : ""}
                      {text(data.access) ? ` · ${text(data.access)}` : ""}
                    </p>
                    {text(data.description) && (
                      <p className="outline-meta">{text(data.description)}</p>
                    )}
                    {text(data.spatial_features) && (
                      <p className="outline-meta">Không gian: {text(data.spatial_features)}</p>
                    )}
                    {kind === "power" && (
                      <dl className="outline-facts">
                        <div>
                          <dt>Nguồn</dt>
                          <dd>{text(data.source) ?? "Chưa có"}</dd>
                        </div>
                        <div>
                          <dt>Cái giá</dt>
                          <dd>{text(data.cost) ?? "Chưa có"}</dd>
                        </div>
                      </dl>
                    )}
                    {SENSES.some(([key]) => text(sensory[key])) && (
                      <ul className="outline-detail-list">
                        {SENSES.map(([key, label]) =>
                          text(sensory[key]) ? (
                            <li key={key}>
                              {label}: {text(sensory[key])}
                            </li>
                          ) : null,
                        )}
                      </ul>
                    )}
                    </ItemCard>
                  </div>
                );
              })}
            </div>
          )}
        </section>
      </div>

      <section className="outline-section">
        <header className="outline-section-head">
          <h2>Quan hệ</h2>
          <span className="outline-count">{bonds.length} đề xuất</span>
          {reviewControls(bonds, "world")}
        </header>
        {bonds.length === 0 ? (
          <p className="outline-empty">
            Chưa có quan hệ nào. Quan hệ chỉ sinh sau khi nhân vật được chốt.
          </p>
        ) : (
          <ol className="outline-chapter-list">
            {bonds.map((item) => {
              const data = payloadOf(item);
              const states = record(data.states);
              const from = nameOf.get(text(data.from) ?? "") ?? text(data.from) ?? "?";
              const to = nameOf.get(text(data.to) ?? "") ?? text(data.to) ?? "?";
              return (
                <li key={item.key}>
                  {selection(item)}
                  <ItemCard item={item} {...actions(item)}>
                    <p className="outline-meta">
                      <strong>
                        {from} và {to}
                      </strong>
                      {text(data.summary) ? ` · ${text(data.summary)}` : ""}
                    </p>
                    <ul className="outline-detail-list">
                      {STATE_LABELS.map(([key, label]) => {
                        const value = text(states[key]);
                        return value ? (
                          <li key={key}>
                            {label}: {value}
                          </li>
                        ) : null;
                      })}
                    </ul>
                  </ItemCard>
                </li>
              );
            })}
          </ol>
        )}
      </section>
    </div>
  );
}
