import { useState } from "react";
import { Wrench, CheckCircle2, Terminal } from "lucide-react";
import { cn } from "../../../shared/utils/cn";
import { Popover } from "../../../shared/components/ui/popover";
import type { ToolCallItem } from "./types";

interface ToolCallsPopoverProps {
  toolCalls: ToolCallItem[];
}

export function ToolCallsPopover({ toolCalls }: ToolCallsPopoverProps) {
  const [activeToolId, setActiveToolId] = useState<string>(toolCalls[0]?.id || "");

  if (!toolCalls || toolCalls.length === 0) return null;

  const activeTool = toolCalls.find((t) => t.id === activeToolId) || toolCalls[0];

  return (
    <Popover
      label="Executed tools"
      align="start"
      side="top"
      trigger={<span className="flex h-7 items-center gap-1.5 rounded-lg border border-line px-2 text-xs font-medium text-fg-muted transition-colors"><Wrench aria-hidden="true" className="h-3.5 w-3.5" /><span className="text-xs font-semibold">{toolCalls.length} tools</span></span>}
      triggerClassName="rounded-lg"
      contentClassName="w-96 p-3"
    >
      <div className="flex items-center justify-between border-b border-line pb-2 px-1">
        <span className="flex items-center gap-1.5 text-xs font-semibold text-fg">
          <Terminal aria-hidden="true" className="h-3.5 w-3.5 text-accent" />
          <span>Executed Tools ({toolCalls.length})</span>
        </span>
      </div>

      <div className="mt-2.5 grid grid-cols-5 gap-2">
        <div className="col-span-2 space-y-1 border-r border-line pr-1.5">
          {toolCalls.map((t) => (
            <button
              key={t.id}
              type="button"
              onClick={() => setActiveToolId(t.id)}
              aria-label={`Select tool ${t.name}`}
              className={cn(
                "focus-ring flex w-full items-center justify-between rounded-lg px-2 py-1.5 text-xs font-mono transition-colors text-left",
                activeTool.id === t.id ? "bg-surface-muted font-semibold text-fg" : "text-fg-muted hover:bg-surface-muted"
              )}
            >
              <span className="truncate">{t.name}</span>
              <CheckCircle2 aria-hidden="true" className="h-3 w-3 text-emerald-500 shrink-0" />
            </button>
          ))}
        </div>

        <div className="col-span-3 max-h-48 overflow-y-auto space-y-2 text-xs font-mono pl-1">
          <div>
            <span className="block mb-0.5 text-fg-subtle">Input:</span>
            <pre className="overflow-x-auto rounded-lg bg-surface-muted p-2 text-fg-muted">{JSON.stringify(activeTool.parameters, null, 2)}</pre>
          </div>
          {activeTool.result && (
            <div>
              <span className="block mb-0.5 text-fg-subtle">Output:</span>
              <pre className="overflow-x-auto rounded-lg bg-surface-muted p-2 text-fg-muted">{JSON.stringify(activeTool.result, null, 2)}</pre>
            </div>
          )}
        </div>
      </div>
    </Popover>
  );
}
