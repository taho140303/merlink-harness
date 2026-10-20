import { useState, type ReactNode } from "react";
import type { OutlineItem } from "../../types";

interface Props {
  item: OutlineItem;
  children?: ReactNode;
  onAccept: () => void;
  onReject: () => void;
  /** `note` is empty for "viết lại", filled for "sửa". */
  onRegenerate: (note: string) => void;
  busy?: boolean;
}

const STATUS_LABEL: Record<string, string> = {
  pending: "Đề xuất",
  accepted: "Đã duyệt",
  committed: "Đã chốt",
  failed: "Lỗi",
};

/**
 * One reviewable proposal. The body is provided by the board, the shell owns the
 * status pill and the four actions every proposal shares.
 */
export function ItemCard({
  item,
  children,
  onAccept,
  onReject,
  onRegenerate,
  busy = false,
}: Props) {
  const [editing, setEditing] = useState(false);
  const [note, setNote] = useState("");
  const settled = item.status === "committed";

  function toggleEdit() {
    setEditing((value) => !value);
    setNote("");
  }

  return (
    <article className={`outline-item is-${item.status}`}>
      <header className="outline-item-head">
        <span className="outline-item-position">{item.position}</span>
        <h3>{item.name || item.title || item.key}</h3>
        <span className={`outline-pill is-${item.status}`}>
          {STATUS_LABEL[item.status] ?? item.status}
        </span>
      </header>

      {item.error && (
        <p className="outline-error" role="alert">
          {item.error}
        </p>
      )}

      {children}

      {editing && (
        <div className="outline-edit">
          <label>
            Ghi chú chỉnh sửa
            <textarea
              value={note}
              placeholder="Mô tả đúng chỗ cần đổi, ví dụ: chương 3 dồn dập hơn"
              onChange={(event) => setNote(event.target.value)}
            />
          </label>
          <button
            type="button"
            className="btn btn-primary"
            disabled={busy || !note.trim()}
            onClick={() => {
              onRegenerate(note.trim());
              setEditing(false);
            }}
          >
            Lưu chỉnh sửa
          </button>
        </div>
      )}

      <div className="outline-actions">
        <button
          type="button"
          className="btn"
          disabled={busy || settled}
          onClick={toggleEdit}
        >
          Sửa
        </button>
        <button
          type="button"
          className="btn"
          disabled={busy || settled}
          onClick={() => onRegenerate("")}
        >
          Viết lại
        </button>
        <button type="button" className="btn" disabled={busy || settled} onClick={onReject}>
          Bỏ
        </button>
        <button
          type="button"
          className="btn btn-primary"
          disabled={busy || item.status === "accepted" || settled}
          onClick={onAccept}
        >
          Duyệt
        </button>
      </div>
    </article>
  );
}
