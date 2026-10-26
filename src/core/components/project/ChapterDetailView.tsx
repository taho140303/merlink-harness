import { useState, useRef, useEffect } from "react";
import { Link } from "react-router";
import { Pencil, MessageSquare, Lock, Unlock, GripVertical, X, Check } from "lucide-react";
import { cn } from "../../../shared/utils/cn";
import { truncate } from "../../../shared/utils/string";
import { useResizablePanel } from "../../../shared/hooks/useResizablePanel";
import { ProjectChatPane } from "./ProjectChatPane";
import { ConfirmDialog } from "../../../shared/components/ui/confirm-dialog";
import { toast } from "../../../shared/components/ui/toast";
import { useT } from "../../../shared/i18n/useT";
import { FloatingOutline } from "../../../shared/components/ui/floating-outline";
import type { ProjectItem, ChapterItem, ChapterBlock } from "./types";

interface ChapterDetailViewProps {
  project: ProjectItem;
  chapter: ChapterItem;
  chapterIndex: number;
}

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
          : "text-slate-800 dark:text-slate-200",
      )}
    />
  );
}

export function ChapterDetailView({ project, chapter, chapterIndex }: ChapterDetailViewProps) {
  const t = useT();
  const [isEditing, setIsEditing] = useState(false);
  const [editSnapshot, setEditSnapshot] = useState<ChapterBlock[] | null>(null);
  const [discardOpen, setDiscardOpen] = useState(false);
  const [isChatOpen, setIsChatOpen] = useState(false);
  const [blocks, setBlocks] = useState<ChapterBlock[]>(
    chapter.blocks || [
      { id: "b1", text: "Nội dung chương đang được soạn thảo...", locked: false },
    ],
  );

  const { leftRatio, isDragging, containerRef, startDragging } = useResizablePanel({
    initialRatio: 45,
    minPx: 340,
  });

  const handleStartEdit = () => {
    setEditSnapshot(structuredClone(blocks));
    setIsEditing(true);
  };
  const handleSave = () => {
    setIsEditing(false);
    setEditSnapshot(null);
    toast({ title: t("common.saved"), variant: "success" });
  };
  const handleCancelConfirm = () => {
    if (editSnapshot) setBlocks(editSnapshot);
    setIsEditing(false);
    setEditSnapshot(null);
    setDiscardOpen(false);
  };

  const toggleLock = (blockId: string) => {
    setBlocks((prev) =>
      prev.map((b) => (b.id === blockId ? { ...b, locked: !b.locked } : b)),
    );
  };
  const updateBlockText = (blockId: string, newText: string) => {
    setBlocks((prev) =>
      prev.map((b) => (b.id === blockId ? { ...b, text: newText } : b)),
    );
  };

  const fullChapterTitle = `Chương ${chapterIndex}: ${chapter.title}`;

  const scrollToBlock = (id: string) => {
    const el = document.getElementById(id);
    if (el) el.scrollIntoView({ behavior: 'smooth', block: 'center' });
  };

  const outlineItems = blocks.map((b, idx) => ({
    id: b.id,
    title: b.text || `Block ${idx + 1}`
  }));

  return (
    <div
      ref={containerRef}
      className={cn(
        "relative flex h-full w-full overflow-hidden",
        isDragging && "cursor-col-resize select-none",
      )}
    >
      {isChatOpen && (
        <div style={{ width: `${leftRatio}%` }} className="h-full overflow-hidden">
          <ProjectChatPane
            projectName={project.title}
            subContext={fullChapterTitle}
            onClose={() => setIsChatOpen(false)}
          />
        </div>
      )}

      {isChatOpen && (
        <div
          onMouseDown={startDragging}
          className="group relative flex w-2 shrink-0 cursor-col-resize items-center justify-center bg-transparent transition-colors hover:bg-blue-500/10 dark:hover:bg-blue-500/20"
        >
          <div className="h-full w-px bg-slate-200 transition-colors group-hover:bg-blue-500 dark:bg-slate-800" />
          <div
            className={cn(
              "absolute z-30 flex h-8 w-4.5 items-center justify-center rounded-md border border-slate-300 bg-white shadow-xs transition-opacity dark:border-slate-700 dark:bg-slate-800",
              isDragging ? "opacity-100 ring-2 ring-blue-500" : "opacity-0 group-hover:opacity-100",
            )}
          >
            <GripVertical className="h-3.5 w-3.5 text-slate-500 dark:text-slate-400" />
          </div>
        </div>
      )}

      <div
        style={{ width: isChatOpen ? `${100 - leftRatio}%` : "100%" }}
        className="relative flex h-full flex-col overflow-y-auto bg-transparent p-6 dark:bg-transparent sm:p-8"
      >
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
            {!isEditing ? (
              <button
                type="button"
                onClick={handleStartEdit}
                className="flex items-center gap-1.5 rounded-xl border border-slate-200 bg-white px-3 py-1.5 text-xs font-medium text-slate-700 shadow-xs transition-colors hover:bg-slate-50 dark:border-slate-800 dark:bg-slate-900 dark:text-slate-300 dark:hover:bg-slate-800"
              >
                <Pencil className="h-3.5 w-3.5" />
                <span>{t("common.edit")}</span>
              </button>
            ) : (
              <>
                <button
                  type="button"
                  onClick={() => setDiscardOpen(true)}
                  className="flex items-center gap-1.5 rounded-xl border border-slate-200 bg-white px-3 py-1.5 text-xs font-medium text-slate-700 shadow-xs transition-colors hover:bg-slate-50 dark:border-slate-800 dark:bg-slate-900 dark:text-slate-300 dark:hover:bg-slate-800"
                >
                  <X className="h-3.5 w-3.5" />
                  <span>{t("common.cancel")}</span>
                </button>
                <button
                  type="button"
                  onClick={handleSave}
                  className="flex items-center gap-1.5 rounded-xl bg-slate-900 px-3 py-1.5 text-xs font-semibold text-white shadow-xs transition-colors hover:bg-slate-800 dark:bg-slate-100 dark:text-slate-900 dark:hover:bg-slate-200"
                >
                  <Check className="h-3.5 w-3.5" />
                  <span>{t("common.save")}</span>
                </button>
              </>
            )}

            <button
              type="button"
              onClick={() => setIsChatOpen(!isChatOpen)}
              className={cn(
                "flex items-center gap-1.5 rounded-xl border px-3 py-1.5 text-xs font-medium shadow-xs transition-colors",
                isChatOpen
                  ? "border-blue-500 bg-blue-50 text-blue-700 dark:border-blue-600 dark:bg-blue-950/60 dark:text-blue-300"
                  : "border-slate-200 bg-white text-slate-700 hover:bg-slate-50 dark:border-slate-800 dark:bg-slate-900 dark:text-slate-300",
              )}
            >
              <MessageSquare className="h-3.5 w-3.5" />
              <span>Chat</span>
            </button>
          </div>
        </div>

        <div className="mx-auto mt-6 w-full max-w-3xl space-y-5 pr-14">
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
                id={block.id}
                className={cn(
                  "group relative rounded-2xl transition-all duration-200 scroll-mt-6",
                  isEditing
                    ? "border border-slate-200 bg-white p-4 shadow-xs dark:border-slate-800 dark:bg-slate-900"
                    : "text-sm leading-relaxed text-slate-700 dark:text-slate-300",
                )}
              >
                {isEditing ? (
                  <>
                    <div
                      className={cn(
                        "absolute -right-11 top-1/2 z-10 -translate-y-1/2 transition-all duration-200",
                        block.locked ? "opacity-100" : "opacity-0 group-hover:opacity-100",
                      )}
                    >
                      <button
                        type="button"
                        onClick={() => toggleLock(block.id)}
                        className={cn(
                          "flex h-7 w-7 items-center justify-center rounded-lg border shadow-xs transition-colors",
                          block.locked
                            ? "border-rose-300 bg-rose-50 text-rose-600 dark:border-rose-800 dark:bg-rose-950 dark:text-rose-300"
                            : "border-slate-200 bg-white text-slate-400 hover:border-slate-300 hover:text-slate-800 dark:border-slate-700 dark:bg-slate-800 dark:text-slate-400 dark:hover:text-slate-200",
                        )}
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

      {/* Thanh Outline (Minimap) - Nằm cố định bên ngoài khung cuộn */}
      {blocks.length > 0 && (
        <FloatingOutline 
          items={outlineItems} 
          onItemClick={scrollToBlock} 
        />
      )}

      <ConfirmDialog
        open={discardOpen}
        onOpenChange={setDiscardOpen}
        title={t("discard.title")}
        description={t("discard.description")}
        confirmLabel={t("discard.confirm")}
        cancelLabel={t("discard.cancel")}
        variant="danger"
        onConfirm={handleCancelConfirm}
      />
    </div>
  );
}
