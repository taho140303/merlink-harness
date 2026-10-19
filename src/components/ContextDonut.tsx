import { useState } from "react";
import type { ContextUsage } from "../types";

function compact(value: number): string {
  if (value >= 1_000_000) return `${(value / 1_000_000).toFixed(value % 1_000_000 ? 1 : 0)}M`;
  if (value >= 1_000) return `${(value / 1_000).toFixed(value % 1_000 ? 1 : 0)}K`;
  return String(value);
}

/**
 * The context ring (Sprint 11.10) rendered as an ordinary flex item (11.11 · D1).
 *
 * It carries no positioning of its own: the input row places it between the text
 * field and Send, and the wrapper is the anchor its popover opens upward from.
 * Rendering it inline is the whole point — it used to sit below the chat.
 */
export function ContextDonut({ usage }: { usage: ContextUsage | null }) {
  const [open, setOpen] = useState(false);
  const hasModel = usage?.has_model ?? false;
  const percent = hasModel ? usage?.percent ?? 0 : 0;
  return <div className="context-donut-wrap" onMouseEnter={() => setOpen(true)} onMouseLeave={() => setOpen(false)}>
    <button type="button" className="context-donut" aria-label="Context usage" aria-describedby={open ? "context-usage-popover" : undefined} onFocus={() => setOpen(true)} onBlur={() => setOpen(false)}>
      <svg viewBox="0 0 40 40" aria-hidden="true"><circle className="context-donut-track" cx="20" cy="20" r="16"/><circle className="context-donut-value" cx="20" cy="20" r="16" strokeDasharray={`${percent} 100`}/></svg>
      <span>{hasModel ? `${percent}%` : "—"}</span>
    </button>
    {open && <div className="context-popover" id="context-usage-popover" role="tooltip">
      {!hasModel ? <strong>Chưa chọn model</strong> : <>
        <strong>{percent}% context used · ~{compact(usage!.used_tokens)} / {compact(usage!.total_tokens)}</strong>
        <div><span>System prompt</span><span>~{compact(usage!.system_tokens)}</span></div>
        <div><span>Tool definitions</span><span>~{compact(usage!.tool_tokens)}</span></div>
        <div><span>Messages</span><span>~{compact(usage!.message_tokens)}</span></div>
        {usage!.measured_input_tokens != null && <div><span>Đo thật</span><span>{compact(usage!.measured_input_tokens)}</span></div>}
      </>}
    </div>}
  </div>;
}
