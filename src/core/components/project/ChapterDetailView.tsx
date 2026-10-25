import { useState, useRef, useEffect } from "react";
import { Link } from "react-router";
import { Pencil, MessageSquare, Lock, Unlock, GripVertical } from "lucide-react";
import { cn } from "../../../shared/utils/cn";
import { truncate } from "../../../shared/utils/string";
import { useResizablePanel } from "../../../shared/hooks/useResizablePanel";
import { ProjectChatPane } from "./ProjectChatPane";
import type { ProjectItem, ChapterItem, ChapterBlock } from "./types";

interface ChapterDetailViewProps {
  project: ProjectItem;
  chapter: ChapterItem;
  chapterIndex: number;
}

// Component Textarea tự dãn theo nội dung chữ và tắt triệt để scrollbar
function AutoResizeTextarea({
  value,
  disabled,
  onChange,
}: {
  value: string;
  disabled?: boolean;
  onChange: (val: string) => void;
}) {
  const textareaRef = useRef<HTMLTextAreaElement>(null);

  useEffect(() => {
    if (textareaRef.current) {
      textareaRef.current.style.height = "auto";
      textareaRef.current.style.height = `${textareaRef.current.scrollHeight}px`;
    }
  }, [value]);

  return (
    <textarea
      ref={textareaRef}
      value={value}
      disabled={disabled}
      onChange={(e) => onChange(e.target.value)}
      rows={1}
      className={cn(
        "w-full resize-none overflow-hidden bg-transparent text-xs leading-relaxed focus:outline-hidden",
        disabled
          ? "cursor-not-allowed text-slate-400 select-none dark:text-slate-500"
          : "text-slate-800 dark:text-slate-200"
      )}
    />
  );
}

export function ChapterDetailView({
  project,
  chapter,
  chapterIndex,
}: ChapterDetailViewProps) {
  const [isEditing, setIsEditing] = useState(false);
  const [isChatOpen, setIsChatOpen] = useState(false);
  const [blocks, setBlocks] = useState<ChapterBlock[]>(
    chapter.blocks || [
      { id: "b1", text: "Nội dung chương đang được soạn thảo...", locked: false },
    ]
  );

  const { leftRatio, isDragging, containerRef, startDragging } = useResizablePanel({
    initialRatio: 45,
    minPx: 340,
  });

  const toggleLock = (blockId: string) => {
    setBlocks((prev) =>
      prev.map((b) => (b.id === blockId ? { ...b, locked: !b.locked } : b))
    );
  };

  const updateBlockText = (blockId: string, newText: string) => {
    setBlocks((prev) =>
      prev.map((b) => (b.id === blockId ? { ...b, text: newText } : b))
    );
  };

  const fullChapterTitle = `Chương ${chapterIndex}: ${chapter.title}`;

  return (
    <div
      ref={containerRef}
      className={cn(
        "flex h-full w-full overflow-hidden",
        isDragging && "cursor-col-resize select-none"
      )}
    >
      {/* Pane Trái: Chat Layout */}
      {isChatOpen && (
        <div style={{ width: `${leftRatio}%` }} className="h-full overflow-hidden">
          <ProjectChatPane
            projectName={project.title}
            subContext={fullChapterTitle}
            onClose={() => setIsChatOpen(false)}
          />
        </div>
      )}

      {/* Ranh giới kéo cột */}
      {isChatOpen && (
        <div
          onMouseDown={startDragging}
          className="group relative flex w-2 shrink-0 cursor-col-resize items-center justify-center bg-transparent transition-colors hover:bg-blue-500/10 dark:hover:bg-blue-500/20"
        >
          <div className="h-full w-px bg-slate-200 transition-colors group-hover:bg-blue-500 dark:bg-slate-800" />
          <div
            className={cn(
              "absolute z-30 flex h-8 w-4.5 items-center justify-center rounded-md border border-slate-300 bg-white shadow-xs transition-opacity dark:border-slate-700 dark:bg-slate-800",
              isDragging ? "opacity-100 ring-2 ring-blue-500" : "opacity-0 group-hover:opacity-100"
            )}
          >
            <GripVertical className="h-3.5 w-3.5 text-slate-500 dark:text-slate-400" />
          </div>
        </div>
      )}

      {/* Pane Phải: Nội dung chương */}
      <div
        style={{ width: isChatOpen ? `${100 - leftRatio}%` : "100%" }}
        className="flex h-full flex-col overflow-y-auto bg-slate-50/50 p-6 dark:bg-slate-950/20 sm:p-8"
      >
        {/* Header Breadcrumbs */}
        <div className="flex h-12 shrink-0 items-center justify-between border-b border-slate-200 pb-4 dark:border-slate-800">
          <div className="flex items-center gap-2 truncate text-xs font-semibold text-slate-500 dark:text-slate-400">
            <Link to="/project" className="shrink-0 hover:text-slate-900 dark:hover:text-slate-100">
              Project
            </Link>
            <span>›</span>
            <Link
              to={`/project/${project.id}`}
              className="truncate hover:text-slate-900 dark:hover:text-slate-100"
              title={project.title}
            >
              {truncate(project.title, 15)}
            </Link>
            <span>›</span>
            <span
              className="truncate font-bold text-slate-900 dark:text-slate-100"
              title={fullChapterTitle}
            >
              {truncate(fullChapterTitle, 15)}
            </span>
          </div>

          <div className="flex shrink-0 items-center gap-2">
            <button
              type="button"
              onClick={() => setIsEditing(!isEditing)}
              className={cn(
                "flex items-center gap-1.5 rounded-xl border px-3 py-1.5 text-xs font-medium shadow-xs transition-colors",
                isEditing
                  ? "border-amber-500 bg-amber-50 text-amber-800 dark:border-amber-600 dark:bg-amber-950/60 dark:text-amber-300"
                  : "border-slate-200 bg-white text-slate-700 hover:bg-slate-50 dark:border-slate-800 dark:bg-slate-900 dark:text-slate-300"
              )}
            >
              <Pencil className="h-3.5 w-3.5" />
              <span>{isEditing ? "Viewing" : "Edit"}</span>
            </button>

            <button
              type="button"
              onClick={() => setIsChatOpen(!isChatOpen)}
              className={cn(
                "flex items-center gap-1.5 rounded-xl border px-3 py-1.5 text-xs font-medium shadow-xs transition-colors",
                isChatOpen
                  ? "border-blue-500 bg-blue-50 text-blue-700 dark:border-blue-600 dark:bg-blue-950/60 dark:text-blue-300"
                  : "border-slate-200 bg-white text-slate-700 hover:bg-slate-50 dark:border-slate-800 dark:bg-slate-900 dark:text-slate-300"
              )}
            >
              <MessageSquare className="h-3.5 w-3.5" />
              <span>Chat</span>
            </button>
          </div>
        </div>

        {/* Nội dung chương (thêm pr-14 để chừa khoảng trống cho icon Lock nằm ngoài mà không bị cấn lề) */}
        <div className="mx-auto mt-6 w-full max-w-3xl pr-14 space-y-5">
          <div className="border-b border-slate-100 pb-4 dark:border-slate-800">
            <h1 className="text-xl font-bold tracking-tight text-slate-900 dark:text-slate-100 sm:text-2xl">
              {fullChapterTitle}
            </h1>
            <p className="mt-1 text-xs text-slate-400">Đăng ngày: {chapter.date}</p>
          </div>

          <div className="space-y-4">
            {blocks.map((block) => (
              <div
                key={block.id}
                className={cn(
                  "group relative rounded-2xl transition-all duration-200",
                  isEditing
                    ? "border border-slate-200 bg-white p-4 shadow-xs dark:border-slate-800 dark:bg-slate-900"
                    : "text-sm leading-relaxed text-slate-700 dark:text-slate-300"
                )}
              >
                {isEditing ? (
                  <>
                    {/* Icon Lock nằm ngoài khối ở mép phải, căn giữa theo chiều dọc */}
                    <div
                      className={cn(
                        "absolute -right-11 top-1/2 z-10 -translate-y-1/2 transition-all duration-200",
                        block.locked
                          ? "opacity-100"
                          : "opacity-0 group-hover:opacity-100"
                      )}
                    >
                      <button
                        type="button"
                        onClick={() => toggleLock(block.id)}
                        className={cn(
                          "flex h-7 w-7 items-center justify-center rounded-lg border shadow-xs transition-colors",
                          block.locked
                            ? "border-rose-300 bg-rose-50 text-rose-600 dark:border-rose-800 dark:bg-rose-950 dark:text-rose-300"
                            : "border-slate-200 bg-white text-slate-400 hover:border-slate-300 hover:text-slate-800 dark:border-slate-700 dark:bg-slate-800 dark:text-slate-400 dark:hover:text-slate-200"
                        )}
                        title={block.locked ? "Khối đã khóa (Click mở)" : "Khóa khối này"}
                      >
                        {block.locked ? (
                          <Lock className="h-3.5 w-3.5" />
                        ) : (
                          <Unlock className="h-3.5 w-3.5" />
                        )}
                      </button>
                    </div>

                    <AutoResizeTextarea
                      value={block.text}
                      disabled={block.locked}
                      onChange={(val) => updateBlockText(block.id, val)}
                    />

                    {block.locked && (
                      <span className="mt-1.5 inline-flex items-center gap-1 text-[10px] font-medium text-rose-500">
                        <Lock className="h-3 w-3" />
                        <span>Khối nội dung đã bị khóa</span>
                      </span>
                    )}
                  </>
                ) : (
                  <p className="text-sm leading-relaxed">{block.text}</p>
                )}
              </div>
            ))}
          </div>
        </div>
      </div>
    </div>
  );
}
