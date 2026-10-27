import { useState } from "react";
import { Sparkles, ArrowUp, BookOpen, X } from "lucide-react";
import { ModelSelectorPopover } from "../chat/ModelSelectorPopover";
import { ContextUsagePopover } from "../chat/ContextUsagePopover";

interface ProjectChatPaneProps {
  projectName: string;
  subContext?: string;
  onClose: () => void;
}

export function ProjectChatPane({
  projectName,
  subContext,
  onClose,
}: ProjectChatPaneProps) {
  const [prompt, setPrompt] = useState("");

  return (
    <aside className="flex h-full w-full flex-col border-r border-slate-200 bg-slate-50/60 dark:border-slate-800 dark:bg-slate-950/40">
      {/* Header Chat Pane */}
      <div className="flex h-14 shrink-0 items-center justify-between border-b border-slate-200 bg-white/70 px-4 backdrop-blur-xs dark:border-slate-800 dark:bg-slate-900/70">
        <div className="flex items-center gap-2 truncate">
          <Sparkles className="h-4 w-4 text-blue-500 shrink-0" />
          <span className="text-xs font-semibold text-slate-800 dark:text-slate-200 truncate">
            Merlink: {projectName}
          </span>
        </div>
        <button
          type="button"
          onClick={onClose}
          className="rounded-lg p-1 text-slate-400 hover:bg-slate-100 hover:text-slate-700 dark:hover:bg-slate-800 dark:hover:text-slate-200 transition-colors"
          title="Đóng Chat"
        >
          <X className="h-4 w-4" />
        </button>
      </div>

      {/* Tin nhắn mẫu */}
      <div className="flex-1 overflow-y-auto p-4 space-y-4 text-xs">
        <div className="rounded-2xl border border-slate-200 bg-white p-3.5 shadow-2xs dark:border-slate-800 dark:bg-slate-900 leading-relaxed text-slate-600 dark:text-slate-300">
          <p className="font-semibold text-slate-800 dark:text-slate-100 mb-1">
            Đã liên kết ngữ cảnh dự án:
          </p>
          <p>
            Toàn bộ thông tin thiết lập thế giới, danh sách Arc và nội dung của{" "}
            <span className="font-semibold text-blue-600 dark:text-blue-400">{projectName}</span>{" "}
            đã được tải vào bộ nhớ ngữ cảnh.
          </p>
          {subContext && (
            <p className="mt-2 rounded-lg bg-blue-50 p-2 text-blue-800 dark:bg-blue-950/60 dark:text-blue-300">
              Đang làm việc tại: <strong>{subContext}</strong>
            </p>
          )}
        </div>
      </div>

      {/* Input Chat với Source là Project Name */}
      <div className="shrink-0 p-4">
        <div className="rounded-2xl border border-slate-200 bg-white p-3 shadow-sm dark:border-slate-800 dark:bg-slate-900">
          {/* Tag đính kèm ngữ cảnh project */}
          <div className="mb-2 flex items-center gap-1.5 rounded-lg bg-blue-50/70 px-2.5 py-1 text-[11px] font-medium text-blue-800 dark:bg-blue-950/50 dark:text-blue-300">
            <BookOpen className="h-3 w-3 shrink-0 text-blue-500" />
            <span className="truncate">Context: {projectName} {subContext ? `› ${subContext}` : ""}</span>
          </div>

          <input
            type="text"
            value={prompt}
            onChange={(e) => setPrompt(e.target.value)}
            placeholder="Hỏi hoặc yêu cầu AI hỗ trợ kịch bản..."
            className="w-full bg-transparent text-xs text-slate-800 placeholder:text-slate-400 focus:outline-hidden dark:text-slate-100"
          />

          <div className="mt-3 flex items-center justify-between border-t border-slate-100 pt-2 dark:border-slate-800/80">
            {/* Select Source cố định là Tên Project */}
            <div className="flex items-center gap-1.5">
              <span className="rounded-lg border border-slate-200 bg-slate-50 px-2.5 py-1 text-[10px] font-medium text-slate-700 dark:border-slate-700 dark:bg-slate-800 dark:text-slate-300 max-w-[130px] truncate">
                {projectName}
              </span>
            </div>

            <div className="flex items-center gap-1.5">
              <ModelSelectorPopover />
              <ContextUsagePopover />
              <button
                type="button"
                className="flex h-7 w-7 items-center justify-center rounded-lg bg-slate-900 text-white hover:bg-slate-800 dark:bg-slate-100 dark:text-slate-900 dark:hover:bg-slate-200 transition-colors"
                title="Gửi"
              >
                <ArrowUp className="h-3.5 w-3.5" />
              </button>
            </div>
          </div>
        </div>
      </div>
    </aside>
  );
}
