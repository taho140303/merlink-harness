import { useState } from "react";
import { Image as ImageIcon, Sparkles, BookOpen, Layers, Users, Clock, Calendar } from "lucide-react";
import { Dialog, DialogContent } from "../../../shared/components/ui/dialog";
import type { ProjectItem, ArcItem } from "./types";

interface ProjectFullDetailDialogProps {
  project: ProjectItem | null;
  open: boolean;
  onOpenChange: (open: boolean) => void;
}

export function ProjectFullDetailDialog({
  project,
  open,
  onOpenChange,
}: ProjectFullDetailDialogProps) {
  const [assetDialogOpen, setAssetDialogOpen] = useState(false);
  const [expandedArcs, setExpandedArcs] = useState<Record<string, boolean>>({});

  if (!project) return null;

  const arcs: ArcItem[] = project.arcs && project.arcs.length > 0 ? project.arcs : [
    {
      id: "arc-default",
      title: "Tập 1: Mở màn biên niên sử",
      chapters: [
        { id: "c-1", title: "Chương 1: Khởi nguyên", date: "18/04/2026", hasIllustration: true },
        { id: "c-2", title: "Chương 2: Dấu vết đầu tiên", date: "24/04/2026" },
        { id: "c-3", title: "Chương 3: Hội ngộ định mệnh", date: "02/05/2026", isNew: true },
      ],
    },
  ];

  const toggleArc = (arcId: string) => {
    setExpandedArcs((prev) => ({ ...prev, [arcId]: !prev[arcId] }));
  };

  return (
    <>
      <Dialog open={open} onOpenChange={onOpenChange}>
        <DialogContent className="max-w-4xl p-0 gap-0 overflow-hidden h-[85vh] flex flex-col">
          {/* Header Bar */}
          <div className="flex h-14 shrink-0 items-center justify-between border-b border-slate-200 px-6 dark:border-slate-800">
            <h2 className="text-sm font-semibold tracking-tight text-slate-900 dark:text-slate-100">
              Project Full Details & Arc Blueprint
            </h2>
          </div>

          {/* Scrollable Body */}
          <div className="flex-1 overflow-y-auto p-6 space-y-8">
            {/* 1. Thông tin tổng quan lần nữa */}
            <div className="flex flex-col gap-5 sm:flex-row rounded-2xl border border-slate-200 bg-slate-50/60 p-5 dark:border-slate-800 dark:bg-slate-900/40">
              <div className="w-28 aspect-[3/5] shrink-0 overflow-hidden rounded-xl border border-slate-200 bg-white shadow-xs dark:border-slate-800 dark:bg-slate-950">
                <img
                  src={project.cover}
                  alt={project.title}
                  className="h-full w-full object-cover"
                />
              </div>

              <div className="flex flex-1 flex-col justify-between">
                <div>
                  <div className="flex items-center gap-2.5">
                    <h3 className="text-base font-bold text-slate-900 dark:text-slate-100">
                      {project.title}
                    </h3>
                    <span className="rounded-none bg-slate-900 px-2 py-0.5 text-[10px] font-bold uppercase text-white dark:bg-slate-100 dark:text-slate-900">
                      {project.status}
                    </span>
                  </div>

                  <p className="mt-2 text-xs leading-relaxed text-slate-600 dark:text-slate-300">
                    {project.description}
                  </p>
                </div>

                <div className="mt-4 grid grid-cols-2 gap-2 text-xs sm:grid-cols-4">
                  <div className="rounded-lg bg-white p-2 border border-slate-200 dark:bg-slate-800 dark:border-slate-700">
                    <span className="text-[10px] text-slate-400 block uppercase">Words</span>
                    <span className="font-semibold text-slate-800 dark:text-slate-200">{project.details.wordCount}</span>
                  </div>
                  <div className="rounded-lg bg-white p-2 border border-slate-200 dark:bg-slate-800 dark:border-slate-700">
                    <span className="text-[10px] text-slate-400 block uppercase">Chapters</span>
                    <span className="font-semibold text-slate-800 dark:text-slate-200">{project.details.chapters}</span>
                  </div>
                  <div className="rounded-lg bg-white p-2 border border-slate-200 dark:bg-slate-800 dark:border-slate-700">
                    <span className="text-[10px] text-slate-400 block uppercase">Characters</span>
                    <span className="font-semibold text-slate-800 dark:text-slate-200">{project.details.characters}</span>
                  </div>
                  <div className="rounded-lg bg-white p-2 border border-slate-200 dark:bg-slate-800 dark:border-slate-700">
                    <span className="text-[10px] text-slate-400 block uppercase">Modified</span>
                    <span className="font-semibold text-slate-800 dark:text-slate-200">{project.modifiedAt}</span>
                  </div>
                </div>
              </div>
            </div>

            {/* 2. Container group theo Arc */}
            <div className="space-y-6">
              <h4 className="text-xs font-semibold uppercase tracking-wider text-slate-400 dark:text-slate-500">
                Story Arcs & Volumes
              </h4>

              {arcs.map((arc) => {
                const isExpanded = expandedArcs[arc.id] ?? false;
                const visibleChapters = isExpanded ? arc.chapters : arc.chapters.slice(0, 5);
                const hasMore = arc.chapters.length > 5;

                return (
                  <div
                    key={arc.id}
                    className="overflow-hidden rounded-2xl border border-slate-200 bg-white shadow-xs dark:border-slate-800 dark:bg-slate-900"
                  >
                    {/* Arc Header title */}
                    <div className="border-b border-slate-100 bg-slate-50/70 px-5 py-3 dark:border-slate-800 dark:bg-slate-950/50">
                      <h3 className="text-sm font-bold text-slate-900 dark:text-slate-100 flex items-center gap-1.5">
                        <span>{arc.title}</span>
                        <span className="text-rose-500 font-bold">*</span>
                      </h3>
                    </div>

                    {/* Arc Content Layout */}
                    <div className="flex flex-col sm:flex-row p-5 gap-5">
                      {/* Cột trái: Arc Cover (dùng chung book cover) + nút Assets bên dưới */}
                      <div className="flex flex-col items-center shrink-0 w-32 mx-auto sm:mx-0">
                        <div className="w-28 aspect-[3/5] overflow-hidden rounded-xl border border-slate-200 bg-slate-50 shadow-xs dark:border-slate-800 dark:bg-slate-950">
                          <img
                            src={arc.cover || project.cover}
                            alt={arc.title}
                            className="h-full w-full object-cover"
                          />
                        </div>

                        {/* Nút Assets nhỏ bên dưới cover */}
                        <button
                          type="button"
                          onClick={() => setAssetDialogOpen(true)}
                          className="mt-3 flex w-full items-center justify-center gap-1.5 rounded-lg border border-slate-200 bg-slate-50 py-1.5 text-xs font-medium text-slate-700 hover:bg-slate-100 hover:text-slate-900 dark:border-slate-700 dark:bg-slate-800 dark:text-slate-300 dark:hover:bg-slate-750 transition-colors"
                        >
                          <Sparkles className="h-3 w-3 text-blue-500" />
                          <span>Assets</span>
                        </button>
                      </div>

                      {/* Cột phải: Danh sách chương */}
                      <div className="flex-1 space-y-1">
                        {visibleChapters.map((chap) => (
                          <div
                            key={chap.id}
                            className="flex items-center justify-between py-2 border-b border-slate-100/70 dark:border-slate-800/60 text-xs hover:bg-slate-50 dark:hover:bg-slate-800/40 px-2 rounded-md transition-colors"
                          >
                            <div className="flex items-center gap-2 truncate pr-2">
                              {chap.hasIllustration && (
                                <ImageIcon className="h-3.5 w-3.5 shrink-0 text-slate-500 dark:text-slate-400" />
                              )}
                              {chap.isNew && (
                                <span className="font-semibold text-rose-500 shrink-0">Mới</span>
                              )}
                              <span className="truncate text-slate-800 dark:text-slate-200">
                                {chap.title}
                              </span>
                            </div>
                            <span className="text-[11px] text-slate-400 shrink-0 font-mono">
                              {chap.date}
                            </span>
                          </div>
                        ))}

                        {hasMore && (
                          <div className="pt-2">
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
        </DialogContent>
      </Dialog>

      {/* Dialog con khi ấn nút Assets */}
      <Dialog open={assetDialogOpen} onOpenChange={setAssetDialogOpen}>
        <DialogContent className="max-w-xs p-6 text-center">
          <div className="py-4">
            <p className="text-sm font-medium text-slate-600 dark:text-slate-300">nothing</p>
          </div>
          <button
            type="button"
            onClick={() => setAssetDialogOpen(false)}
            className="w-full rounded-xl bg-slate-900 py-2 text-xs font-medium text-white hover:bg-slate-800 dark:bg-slate-100 dark:text-slate-900 dark:hover:bg-slate-200"
          >
            Đóng
          </button>
        </DialogContent>
      </Dialog>
    </>
  );
}
