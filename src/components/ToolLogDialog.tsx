import { Dialog } from "./Dialog";
import type { ToolCall } from "../types";

/**
 * One executed tool call of a finished turn (Sprint 11.12 · D3).
 *
 * `result` is the raw JSON string carried by the matching `role: "tool"`
 * message, or `null` when no result message exists — which is treated as a
 * failure ("Tool call không có message kết quả → coi là thất bại").
 */
export interface ToolLogEntry {
  call: ToolCall;
  result: string | null;
  ok: boolean;
}

export interface ToolLogDialogProps {
  open: boolean;
  onClose: () => void;
  entries: ToolLogEntry[];
  /** `path -> Vietnamese label`, from the Rust `super_json_labels` command (D8). */
  labels: Record<string, string>;
}

function resultObject(result: unknown): Record<string, unknown> | null {
  return typeof result === "object" && result !== null && !Array.isArray(result)
    ? (result as Record<string, unknown>)
    : null;
}

function errorOf(result: unknown): string | null {
  const object = resultObject(result);
  const error = object?.error;
  return typeof error === "string" ? error : null;
}

function argumentsOf(call: ToolCall): Record<string, unknown> {
  return resultObject(call.arguments) ?? {};
}

/** D3: string → `"…"`; number/bool → raw; array → joined with `, `. */
function formatValue(value: unknown): string {
  if (typeof value === "string") return `"${value}"`;
  if (typeof value === "number" || typeof value === "boolean") {
    return String(value);
  }
  if (Array.isArray(value)) {
    return value
      .map((item) => (typeof item === "string" ? item : JSON.stringify(item)))
      .join(", ");
  }
  if (value === null || value === undefined) return "null";
  return JSON.stringify(value);
}

/** D3: the field label is written with a capital first letter. */
function capitalize(text: string): string {
  return text.length === 0 ? text : `${text[0].toLocaleUpperCase("vi")}${text.slice(1)}`;
}

interface ToolSentence {
  text: string;
  error: string | null;
}

function describe(entry: ToolLogEntry, labels: Record<string, string>): ToolSentence {
  const result = entry.result === null ? null : (() => {
    try {
      return JSON.parse(entry.result) as unknown;
    } catch {
      return null;
    }
  })();
  const resultRecord = resultObject(result);
  const args = argumentsOf(entry.call);
  const error = errorOf(result);

  if (entry.call.name === "super_json_set") {
    const path = typeof args.path === "string" ? args.path : "";
    const label = labels[path] ? capitalize(labels[path]) : path;
    if (entry.ok) {
      // The tool's own result is authoritative; the call arguments are the
      // fallback when an older transcript carries no result body.
      const value = resultRecord?.value ?? args.value;
      return {
        text: `${label} được ghi nhận với ${formatValue(value)}`,
        error: null,
      };
    }
    return {
      text: `${label} chưa được ghi nhận`,
      // D3: a call with no result message counts as failed, so it still needs a
      // small error line under the sentence.
      error: error ?? "Không nhận được kết quả từ công cụ.",
    };
  }

  if (entry.call.name === "super_json_add_row") {
    const table = typeof args.table === "string" ? args.table : "";
    return entry.ok
      ? { text: `Đã thêm một dòng vào ${table}`, error: null }
      : { text: `Chưa thêm được dòng vào ${table}`, error };
  }

  if (entry.call.name === "super_json_add_field") {
    const table = typeof args.table === "string" ? args.table : "";
    return entry.ok
      ? { text: `Đã cập nhật một trường của ${table}`, error: null }
      : { text: `Chưa cập nhật được trường của ${table}`, error };
  }

  if (entry.call.name === "project_create") {
    if (entry.ok) {
      // D7: a project created without a name reports no `name` key at all.
      const name = resultRecord?.name;
      return typeof name === "string" && name.trim().length > 0
        ? { text: `Dự án được tạo với tên "${name}"`, error: null }
        : { text: "Dự án được tạo (chưa có tên)", error: null };
    }
    return { text: "Dự án chưa được tạo", error };
  }

  return { text: `Đã gọi ${entry.call.name}`, error };
}

/**
 * The dialog behind the wrench button (D3): every tool a finished turn called,
 * phrased as a Vietnamese sentence, with failures showing their error line.
 */
export function ToolLogDialog({ open, onClose, entries, labels }: ToolLogDialogProps) {
  return (
    <Dialog open={open} onClose={onClose} title="Công cụ đã gọi">
      {entries.length === 0 ? (
        <p className="chat-tool-log-empty">Lượt này không gọi công cụ nào.</p>
      ) : (
        <ul className="chat-tool-log" data-testid="chat-tool-log">
          {entries.map((entry) => {
            const sentence = describe(entry, labels);
            return (
              <li
                key={entry.call.id}
                className={`chat-tool-log-item${entry.ok ? "" : " is-failed"}`}
              >
                <p className="chat-tool-log-sentence">{sentence.text}</p>
                {sentence.error && (
                  <p className="chat-tool-log-error">{sentence.error}</p>
                )}
              </li>
            );
          })}
        </ul>
      )}
    </Dialog>
  );
}
