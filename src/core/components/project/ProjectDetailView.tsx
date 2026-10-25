import { useState } from "react";
import { Link, useNavigate } from "react-router";
import { Sparkles, MessageSquare, GripVertical } from "lucide-react";
import { cn } from "../../../shared/utils/cn";
import { truncate } from "../../../shared/utils/string";
import { useResizablePanel } from "../../../shared/hooks/useResizablePanel";
import { Dialog, DialogContent } from "../../../shared/components/ui/dialog";
import { ProjectChatPane } from "./ProjectChatPane";
import type { ProjectItem, ArcItem } from "./types";

interface ProjectDetailViewProps {
  project: ProjectItem;
}

export function ProjectDetailView({ project }: ProjectDetailViewProps) {
  const navigate = useNavigate();
  const [isChatOpen, setIsChatOpen] = useState(false);
  const [assetDialogOpen, setAssetDialogOpen] = useState(false);
  const [expandedArcs, setExpandedArcs] = useState<Record<string, boolean>>({});

  const { leftRatio, isDragging, containerRef, startDragging } = useResizablePanel({
    initialRatio: 45,
    minPx: 340,
  });

  const arcs: ArcItem[] = project.arcs && project.arcs.length > 0 ? project.arcs : [];

  const toggleArc = (arcId: string) => {
    setExpandedArcs((prev) => ({ ...prev, [arcId]: !prev[arcId] }));
  };

  return (
    <div
      ref={containerRef}
      className={cn(
        "flex h-full w-full overflow-hidden",
        isDragging && "cursor-col-resize select-none"
      )}
    >
      {/* Pane Trái: Chat Layout khi bật Chat */}
      {isChatOpen && (
        <div style={{ width: `${leftRatio}%` }} className="h-full overflow-hidden">
          <ProjectChatPane
            projectName={project.title}
            onClose={() => setIsChatOpen(false)}
          />
        </div>
      )}

      {/* Thanh chia cột có thể kéo */}
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

      {/* Pane Phải: Nội dung chi tiết Project */}
      <div
        style={{ width: isChatOpen ? `${100 - leftRatio}%` : "100%" }}
        className="flex h-full flex-col overflow-y-auto bg-slate-50/50 p-6 dark:bg-slate-950/20 sm:p-8"
      >
        {/* Header Breadcrumbs: Project > [Tên Project (<=15 ký tự)] */}
        <div className="flex h-12 shrink-0 items-center justify-between border-b border-slate-200 pb-4 dark:border-slate-800">
          <div className="flex items-center gap-2 text-xs font-semibold text-slate-500 dark:text-slate-400">
            <Link to="/project" className="hover:text-slate-900 dark:hover:text-slate-100">
              Project
            </Link>
            <span>›</span>
            <span className="text-slate-900 dark:text-slate-100 font-bold" title={project.title}>
              {truncate(project.title, 15)}
            </span>
          </div>

          <button
            type="button"
            onClick={() => setIsChatOpen(!isChatOpen)}
            className={cn(
              "flex items-center gap-1.5 rounded-xl border px-3 py-1.5 text-xs font-medium shadow-xs transition-colors",
              isChatOpen
                ? "border-blue-500 bg-blue-50 text-blue-700 dark:border-blue-600 dark:bg-blue-950/60 dark:text-blue-300"
                : "border-slate-200 bg-white text-slate-700 hover:bg-slate-50 dark:border-slate-800 dark:bg-slate-900 dark:text-slate-300 dark:hover:bg-slate-800"
            )}
          >
            <MessageSquare className="h-3.5 w-3.5" />
            <span>Chat</span>
          </button>
        </div>

        <div className="mt-6 space-y-6">
          {/* Overview Card */}
          <div className="flex flex-col gap-5 sm:flex-row rounded-2xl border border-slate-200 bg-white p-5 shadow-xs dark:border-slate-800 dark:bg-slate-900">
            <div className="w-28 aspect-[3/5] shrink-0 overflow-hidden rounded-xl border border-slate-100 bg-slate-50 dark:border-slate-800 dark:bg-slate-950">
              <img src={project.cover} alt={project.title} className="h-full w-full object-cover" />
            </div>

            <div className="flex flex-1 flex-col justify-between">
              <div>
                <div className="flex items-center gap-2.5">
                  <h2 className="text-base font-bold text-slate-900 dark:text-slate-100">
                    {project.title}
                  </h2>
                  <span className="rounded-none bg-slate-900 px-2 py-0.5 text-[10px] font-bold uppercase text-white dark:bg-slate-100 dark:text-slate-900">
                    {project.status}
                  </span>
                </div>
                <p className="mt-2 text-xs leading-relaxed text-slate-600 dark:text-slate-300">
                  {project.description}
                </p>
              </div>

              <div className="mt-4 grid grid-cols-2 gap-2 text-xs sm:grid-cols-4">
                <div className="rounded-xl bg-slate-50 p-2.5 dark:bg-slate-800/60 border border-slate-100 dark:border-slate-800">
                  <span className="text-[10px] text-slate-400 block uppercase">Words</span>
                  <span className="font-semibold text-slate-800 dark:text-slate-200">{project.details.wordCount}</span>
                </div>
                <div className="rounded-xl bg-slate-50 p-2.5 dark:bg-slate-800/60 border border-slate-100 dark:border-slate-800">
                  <span className="text-[10px] text-slate-400 block uppercase">Chapters</span>
                  <span className="font-semibold text-slate-800 dark:text-slate-200">{project.details.chapters}</span>
                </div>
                <div className="rounded-xl bg-slate-50 p-2.5 dark:bg-slate-800/60 border border-slate-100 dark:border-slate-800">
                  <span className="text-[10px] text-slate-400 block uppercase">Characters</span>
                  <span className="font-semibold text-slate-800 dark:text-slate-200">{project.details.characters}</span>
                </div>
                <div className="rounded-xl bg-slate-50 p-2.5 dark:bg-slate-800/60 border border-slate-100 dark:border-slate-800">
                  <span className="text-[10px] text-slate-400 block uppercase">Modified</span>
                  <span className="font-semibold text-slate-800 dark:text-slate-200">{project.modifiedAt}</span>
                </div>
              </div>
            </div>
          </div>

          {/* Arcs Container */}
          <div className="space-y-6">
            <h3 className="text-xs font-semibold uppercase tracking-wider text-slate-400 dark:text-slate-500">
              Arcs & Chapter Registry
            </h3>

            {arcs.map((arc) => {
              const isExpanded = expandedArcs[arc.id] ?? false;
              const visibleChapters = isExpanded ? arc.chapters : arc.chapters.slice(0, 5);
              const hasMore = arc.chapters.length > 5;

              return (
                <div
                  key={arc.id}
                  className="overflow-hidden rounded-2xl border border-slate-200 bg-white shadow-xs dark:border-slate-800 dark:bg-slate-900"
                >
                  {/* Bỏ dấu * đỏ theo đúng yêu cầu */}
                  <div className="border-b border-slate-100 bg-slate-50/70 px-5 py-3 dark:border-slate-800 dark:bg-slate-950/50">
                    <h4 className="text-sm font-bold text-slate-900 dark:text-slate-100">
                      {arc.title}
                    </h4>
                  </div>

                  <div className="flex flex-col sm:flex-row p-5 gap-5">
                    {/* Cột trái: Cover Arc + Nút Assets */}
                    <div className="flex flex-col items-center shrink-0 w-32 mx-auto sm:mx-0">
                      <div className="w-28 aspect-[3/5] overflow-hidden rounded-xl border border-slate-200 bg-slate-50 shadow-xs dark:border-slate-800 dark:bg-slate-950">
                        <img
                          src={arc.cover || project.cover}
                          alt={arc.title}
                          className="h-full w-full object-cover"
                        />
                      </div>

                      <button
                        type="button"
                        onClick={() => setAssetDialogOpen(true)}
                        className="mt-3 flex w-full items-center justify-center gap-1.5 rounded-lg border border-slate-200 bg-slate-50 py-1.5 text-xs font-medium text-slate-700 hover:bg-slate-100 dark:border-slate-700 dark:bg-slate-800 dark:text-slate-300 transition-colors"
                      >
                        <Sparkles className="h-3 w-3 text-blue-500" />
                        <span>Assets</span>
                      </button>
                    </div>

                    {/* Cột phải: Danh sách chương (bỏ picture icon, app tự render "Chương <i>:") */}
                    <div className="flex-1 space-y-0.5">
                      {visibleChapters.map((chap, idx) => (
                        <div
                          key={chap.id}
                          onClick={() => navigate(`/project/${project.id}/chapter/${chap.id}`)}
                          className="flex cursor-pointer items-center justify-between py-2.5 px-3 border-b border-slate-100/70 dark:border-slate-800/60 text-xs rounded-xl hover:bg-slate-50 dark:hover:bg-slate-800/50 transition-colors"
                        >
                          <div className="flex items-center gap-2 truncate pr-2">
                            {chap.isNew && (
                              <span className="font-semibold text-rose-500 shrink-0">Mới</span>
                            )}
                            <span className="truncate text-slate-800 dark:text-slate-200">
                              Chương {idx + 1}: {chap.title}
                            </span>
                          </div>
                          <span className="text-[11px] text-slate-400 shrink-0 font-mono">
                            {chap.date}
                          </span>
                        </div>
                      ))}

                      {hasMore && (
                        <div className="pt-2 px-3">
                          <button
                            type="button"
                            onClick={() => toggleArc(arc.id)}
                            className="text-xs font-semibold text-slate-900 hover:text-blue-600 dark:text-slate-100 dark:hover:text-blue-400 transition-colors"
                          >
                            {isExpanded
                              ? "Thu gọn danh sách"
                              : `Xem tiếp (${arc.chapters.length - 5} chương)`}
                          </button>
                        </div>
                      )}
                    </div>
                  </div>
                </div>
              );
            })}
          </div>
        </div>

        {/* Dialog Assets */}
        <Dialog open={assetDialogOpen} onOpenChange={setAssetDialogOpen}>
          <DialogContent className="max-w-xs p-6 text-center">
            <div className="py-4">
              <p className="text-sm font-medium text-slate-600 dark:text-slate-300">nothing</p>
            </div>
            <button
              type="button"
              onClick={() => setAssetDialogOpen(false)}
              className="w-full rounded-xl bg-slate-900 py-2 text-xs font-medium text-white hover:bg-slate-800 dark:bg-slate-100 dark:text-slate-900"
            >
              Đóng
            </button>
          </DialogContent>
        </Dialog>
      </div>
    </div>
  );
}
