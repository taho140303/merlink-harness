import { useState } from "react";
import { Sparkles, ChevronDown, Paperclip, ArrowUp } from "lucide-react";
import { ContextUsagePopover } from "./ContextUsagePopover";
import { ModelSelectorPopover } from "./ModelSelectorPopover";

export function ChatInputBar() {
  const [prompt, setPrompt] = useState("");

  return (
    <div className="mx-auto w-full max-w-4xl rounded-3xl border border-slate-200 bg-white p-3 shadow-lg shadow-slate-200/40 dark:border-slate-800 dark:bg-slate-900 dark:shadow-none">
      <div className="flex items-center gap-2 px-2 pt-1 text-slate-400">
        <Sparkles className="h-4 w-4" />
        <input
          type="text"
          value={prompt}
          onChange={(e) => setPrompt(e.target.value)}
          placeholder="Ask me anything..."
          className="w-full bg-transparent text-sm text-slate-800 placeholder:text-slate-400 focus:outline-hidden dark:text-slate-100"
        />
      </div>

      <div className="mt-3 flex items-center justify-between border-t border-slate-100 pt-2.5 dark:border-slate-800/80">
        {/* Left: Source selector + Context circle popover */}
        <div className="flex items-center gap-2">
          <button
            type="button"
            className="flex items-center gap-1.5 rounded-xl border border-slate-200 bg-slate-50 px-3 py-1.5 text-xs font-medium text-slate-700 hover:bg-slate-100 dark:border-slate-700 dark:bg-slate-800 dark:text-slate-300 dark:hover:bg-slate-750"
          >
            <span>Select Source</span>
            <ChevronDown className="h-3 w-3 text-slate-400" />
          </button>

          <ContextUsagePopover />
        </div>

        {/* Right: Attach + Model selectbox + Send button */}
        <div className="flex items-center gap-2">
          <button
            type="button"
            className="flex items-center gap-1 rounded-xl px-2.5 py-1.5 text-xs font-medium text-slate-500 hover:bg-slate-100 hover:text-slate-800 dark:text-slate-400 dark:hover:bg-slate-800 dark:hover:text-slate-200"
          >
            <Paperclip className="h-3.5 w-3.5" />
            <span>Attach</span>
          </button>

          <ModelSelectorPopover />

          <button
            type="button"
            className="flex h-8 w-8 items-center justify-center rounded-xl bg-slate-900 text-white shadow-xs hover:bg-slate-800 dark:bg-slate-100 dark:text-slate-900 dark:hover:bg-slate-200"
            title="Send"
          >
            <ArrowUp className="h-4 w-4" />
          </button>
        </div>
      </div>
    </div>
  );
}
