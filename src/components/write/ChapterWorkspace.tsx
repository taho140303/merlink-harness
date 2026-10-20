import { useCallback, useEffect, useMemo, useState } from "react";
import { ItemCard } from "../outline/ItemCard";
import { Markdown } from "../Markdown";
import { SelectBox } from "../SelectBox";
import { invokeCommand } from "../../lib/ipc";
import { lockScene, rewriteScenes, writeScene } from "../../lib/writing";
import type {
  OutlineItem,
  ProviderConfig,
  SceneRef,
  WritingStatus,
} from "../../types";

interface Props {
  projectId: string;
}

interface SceneRow {
  chapter: number;
  position: number;
  name: string;
  summary: string;
  status: string;
  words: number;
  contentPath?: string;
  targetWords: number;
}

function text(value: unknown): string {
  return typeof value === "string" ? value : "";
}

/**
 * The writing workspace: chapters on the left, scene blocks on the right. A
 * chapter is a list of scene blocks, each one either empty or already written,
 * and any selection of blocks can be rewritten with one note.
 */
export function ChapterWorkspace({ projectId }: Props) {
  const [providers, setProviders] = useState<ProviderConfig[]>([]);
  const [providerId, setProviderId] = useState("");
  const [modelId, setModelId] = useState("");
  const [scenes, setScenes] = useState<SceneRow[]>([]);
  const [status, setStatus] = useState<WritingStatus | null>(null);
  const [selected, setSelected] = useState<SceneRef[]>([]);
  const [note, setNote] = useState("");
  const [chapter, setChapter] = useState<number | null>(null);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState("");

  const reload = useCallback(async () => {
    const [document, writing] = await Promise.all([
      invokeCommand("super_json_read", { projectId }),
      invokeCommand("writing_status", { projectId }),
    ]);
    const doc = document as Record<string, unknown>;
    const rows = ((doc.optional as Record<string, unknown>)?.schema as unknown[]) ?? [];
    const entries = new Map<string, Record<string, unknown>>();
    for (const entry of rows as Record<string, unknown>[]) {
      entries.set(String(entry.table), entry);
    }
    const chapters = (entries.get("chapters")?.rows as Record<string, unknown>[]) ?? [];
    const sceneRows = (entries.get("scenes")?.rows as Record<string, unknown>[]) ?? [];
    setScenes(
      sceneRows.map((row) => {
        const chapterIndex = Number(row.chapter_id_json_index ?? 0);
        return {
          chapter: chapterIndex + 1,
          position: Number(row.position ?? 0),
          name: text(row.name),
          summary: text(row.summarize),
          status: text(row.status) || "PLANNED",
          words: Number(row.word_count ?? 0),
          contentPath: text(row.content_path) || undefined,
          targetWords: Number(row.target_words ?? 0),
        };
      }),
    );
    setStatus(writing);
    setChapter((current) => current ?? (chapters.length > 0 ? 1 : null));
  }, [projectId]);

  useEffect(() => {
    let active = true;
    void invokeCommand("load_providers")
      .then((list) => {
        if (!active) return;
        setProviders(list);
        const first = list[0];
        if (first) {
          setProviderId((current) => current || first.id);
          setModelId((current) => current || first.default_model_id || first.models[0]?.id || "");
        }
      })
      .catch(() => undefined);
    return () => {
      active = false;
    };
  }, []);

  useEffect(() => {
    void reload().catch((reason: unknown) => setError(String(reason)));
  }, [reload]);

  const chapters = useMemo(() => {
    const map = new Map<number, SceneRow[]>();
    for (const scene of scenes) {
      map.set(scene.chapter, [...(map.get(scene.chapter) ?? []), scene]);
    }
    return [...map.entries()].sort((a, b) => a[0] - b[0]);
  }, [scenes]);

  const current = chapters.find(([position]) => position === chapter)?.[1] ?? [];
  const selectedKeys = new Set(selected.map((scene) => `${scene.chapter}:${scene.position}`));

  function toggle(scene: SceneRow) {
    const key = `${scene.chapter}:${scene.position}`;
    setSelected((previous) =>
      previous.some((entry) => `${entry.chapter}:${entry.position}` === key)
        ? previous.filter((entry) => `${entry.chapter}:${entry.position}` !== key)
        : [...previous, { chapter: scene.chapter, position: scene.position }],
    );
  }

  async function writeOne(scene: SceneRow) {
    if (!providerId || !modelId) {
      setError("Chọn provider và model trước khi viết.");
      return;
    }
    setBusy(true);
    setError("");
    try {
      await writeScene(projectId, providerId, modelId, scene.chapter, scene.position, note || undefined);
      await reload();
    } catch (reason) {
      setError(String(reason));
    } finally {
      setBusy(false);
    }
  }

  async function rewriteSelected() {
    if (!providerId || !modelId) {
      setError("Chọn provider và model trước khi viết.");
      return;
    }
    if (selected.length === 0) {
      setError("Chọn ít nhất một cảnh để viết lại.");
      return;
    }
    setBusy(true);
    setError("");
    try {
      await rewriteScenes(projectId, providerId, modelId, selected, note || undefined);
      setSelected([]);
      await reload();
    } catch (reason) {
      setError(String(reason));
    } finally {
      setBusy(false);
    }
  }

  async function toggleLock(scene: SceneRow) {
    setBusy(true);
    try {
      await lockScene(projectId, scene.chapter, scene.position, scene.status !== "LOCKED");
      await reload();
    } catch (reason) {
      setError(String(reason));
    } finally {
      setBusy(false);
    }
  }

  const provider = providers.find((entry) => entry.id === providerId);

  return (
    <div className="outline-board write-workspace">
      {error && (
        <p className="outline-message is-error" role="alert">
          {error}
        </p>
      )}

      <header className="outline-section-head">
        <h2>Viết chương</h2>
        {status && (
          <span className="outline-count">
            {status.written}/{status.scenes} cảnh · {status.words} từ
          </span>
        )}
        <div className="outline-section-actions">
          <SelectBox
            aria-label="Provider"
            value={providerId}
            disabled={providers.length === 0 || busy}
            onChange={(event) => {
              const next = providers.find((entry) => entry.id === event.target.value);
              setProviderId(event.target.value);
              setModelId(next?.default_model_id ?? next?.models[0]?.id ?? "");
            }}
          >
            {providers.length === 0 ? (
              <option value="">Chưa cấu hình provider</option>
            ) : (
              providers.map((entry) => (
                <option key={entry.id} value={entry.id}>
                  {entry.name}
                </option>
              ))
            )}
          </SelectBox>
          <SelectBox
            aria-label="Model"
            value={modelId}
            disabled={!provider || busy}
            onChange={(event) => setModelId(event.target.value)}
          >
            {provider?.models.map((entry) => (
              <option key={entry.id} value={entry.id}>
                {entry.name || entry.id}
              </option>
            )) ?? <option value="">Chưa có model</option>}
          </SelectBox>
          <button
            type="button"
            className="btn btn-primary"
            disabled={busy || selected.length === 0}
            onClick={() => void rewriteSelected()}
          >
            Viết lại {selected.length} cảnh đã chọn
          </button>
        </div>
      </header>

      <div className="outline-columns">
        <section className="outline-section">
          <header className="outline-section-head">
            <h3>Chương</h3>
          </header>
          <ol className="write-chapter-list">
            {chapters.map(([position, rows]) => {
              const written = rows.filter((row) => row.status !== "PLANNED").length;
              return (
                <li key={position}>
                  <button
                    type="button"
                    className={`write-chapter${chapter === position ? " is-active" : ""}`}
                    onClick={() => setChapter(position)}
                  >
                    <strong>Chương {position}</strong>
                    <span>
                      {written}/{rows.length} cảnh
                    </span>
                  </button>
                </li>
              );
            })}
          </ol>
          <label className="outline-inline-field">
            Ghi chú cho lần viết
            <textarea
              value={note}
              placeholder="Ví dụ: dồn dập hơn, bớt miêu tả"
              onChange={(event) => setNote(event.target.value)}
            />
          </label>
        </section>

        <section className="outline-section">
          <header className="outline-section-head">
            <h3>Cảnh của chương {chapter ?? "-"}</h3>
            <span className="outline-count">{current.length} cảnh</span>
          </header>
          {current.length === 0 ? (
            <p className="outline-empty">
              Chương này chưa có cảnh. Chốt phân cảnh ở bàn dàn ý trước.
            </p>
          ) : (
            <ol className="write-scene-list">
              {current.map((scene) => {
                const key = `${scene.chapter}:${scene.position}`;
                const item: OutlineItem = {
                  id: key,
                  key,
                  lane: "prose_writer",
                  position: scene.position,
                  status:
                    scene.status === "LOCKED"
                      ? "committed"
                      : scene.status === "PLANNED"
                        ? "pending"
                        : "accepted",
                  title: scene.name,
                  error: null,
                  name: scene.name,
                  summarize: scene.summary,
                  scene_count: scene.targetWords,
                };
                return (
                  <li key={key}>
                    <label className="write-scene-select">
                      <input
                        type="checkbox"
                        checked={selectedKeys.has(key)}
                        disabled={busy || scene.status === "LOCKED"}
                        onChange={() => toggle(scene)}
                      />
                      chọn để viết lại
                    </label>
                    <ItemCard
                      item={item}
                      busy={busy}
                      onAccept={() => void writeOne(scene)}
                      onReject={() => void toggleLock(scene)}
                      onRegenerate={() => void writeOne(scene)}
                    >
                      <p className="outline-meta">
                        {scene.status} · {scene.words} từ
                        {scene.contentPath ? ` · ${scene.contentPath}` : ""}
                      </p>
                      {scene.summary && (
                        <div className="outline-summary">
                          <Markdown content={scene.summary} />
                        </div>
                      )}
                    </ItemCard>
                  </li>
                );
              })}
            </ol>
          )}
        </section>
      </div>
    </div>
  );
}
