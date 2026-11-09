import { useState } from "react";
import { Wrench, CheckCircle2, CircleX, Clock3, Terminal } from "lucide-react";
import { cn } from "@/shared/utils/cn";
import { Dialog, DialogContent, DialogDescription, DialogTitle } from "@/shared/components/ui/dialog";
import type { ToolCallItem } from "./types";
import { useT } from "@/shared/i18n/useT";

interface ToolCallsPopoverProps {
  toolCalls: ToolCallItem[];
}

export function ToolCallsPopover({ toolCalls }: ToolCallsPopoverProps) {
  const translate = useT();
  const [open, setOpen] = useState(false);
  const [activeToolId, setActiveToolId] = useState<string>(toolCalls[0]?.id || "");

  if (!toolCalls || toolCalls.length === 0) return null;

  const activeTool = toolCalls.find((t) => t.id === activeToolId) || toolCalls[0];

  return (
    <>
      <button type="button" onClick={() => setOpen(true)} aria-label={translate("chat.executedTools")}
        className="focus-ring flex h-7 items-center gap-1.5 rounded-lg border border-line px-2 text-xs font-medium text-fg-muted transition-colors hover:bg-surface-muted">
        <Wrench aria-hidden="true" className="h-3.5 w-3.5" />
        <span className="text-xs font-semibold">{translate("chat.toolCount", { count: toolCalls.length })}</span>
        {toolCalls.some((tool) => tool.status === "error") && <CircleX aria-hidden="true" className="h-3 w-3 text-rose-500" />}
      </button>
      <Dialog open={open} onOpenChange={setOpen}>
      <DialogContent className="w-[min(90vw,36rem)] max-w-xl p-4">
        <DialogTitle className="sr-only">{translate("chat.executedTools")}</DialogTitle>
        <DialogDescription className="sr-only">{translate("chat.toolCount", { count: toolCalls.length })}</DialogDescription>
      <div className="flex items-center justify-between border-b border-line pb-2 px-1 pr-8">
        <span className="flex items-center gap-1.5 text-xs font-semibold text-fg">
          <Terminal aria-hidden="true" className="h-3.5 w-3.5 text-accent" />
          <span>{translate("chat.executedTools")} ({toolCalls.length})</span>
        </span>
      </div>

      <div className="mt-2.5 grid grid-cols-5 gap-2">
        <div className="col-span-2 space-y-1 border-r border-line pr-1.5">
          {toolCalls.map((tool) => (
            <button
              key={tool.id}
              type="button"
              onClick={() => setActiveToolId(tool.id)}
              aria-label={translate("chat.selectTool", { name: tool.name })}
              className={cn(
                "focus-ring flex w-full items-center justify-between rounded-lg px-2 py-1.5 text-xs font-mono transition-colors text-left",
                activeTool.id === tool.id ? "bg-surface-muted font-semibold text-fg" : "text-fg-muted hover:bg-surface-muted"
              )}
            >
              <span className="truncate">{tool.name}</span>
              {tool.status === "error"
                ? <CircleX aria-hidden="true" className="h-3 w-3 shrink-0 text-rose-500" />
                : tool.status === "pending"
                  ? <Clock3 aria-hidden="true" className="h-3 w-3 shrink-0 text-amber-500" />
                  : <CheckCircle2 aria-hidden="true" className="h-3 w-3 shrink-0 text-emerald-500" />}
            </button>
          ))}
        </div>

        <div className="col-span-3 max-h-48 overflow-y-auto space-y-2 text-xs font-mono pl-1">
          <div>
            <span className="block mb-0.5 text-fg-subtle">{translate("chat.input")}</span>
            <pre className="overflow-x-auto rounded-lg bg-surface-muted p-2 text-fg-muted">{JSON.stringify(activeTool.parameters, null, 2)}</pre>
          </div>
          <p className={cn("font-sans font-medium", activeTool.status === "error" ? "text-rose-500" : "text-fg-muted")}>
            {activeTool.status === "error" ? translate("chat.toolFailed") : activeTool.status === "pending" ? translate("chat.toolPending") : translate("chat.toolSucceeded")}
          </p>
          {activeTool.result && (
            <div>
              <span className="block mb-0.5 text-fg-subtle">{translate("chat.output")}</span>
              <pre className="overflow-x-auto rounded-lg bg-surface-muted p-2 text-fg-muted">{JSON.stringify(activeTool.result, null, 2)}</pre>
            </div>
          )}
        </div>
      </div>
      </DialogContent>
      </Dialog>
    </>
  );
}
