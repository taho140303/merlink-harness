import { PanelRightClose, BookOpen, Layers, Users, Calendar, Clock } from "lucide-react";
import type { ProjectItem } from "./types";

interface ProjectDetailSidebarProps {
  project: ProjectItem | null;
  onClose: () => void;
}

// Xử lý tạo hình tháp 3 2 1 cho genres (tối đa 9 tags, nếu > 9 thì tag 9 là "...")
function getPyramidRows(genres: string[]): string[][] {
  const maxTags = 9;
  let items: string[];
  if (genres.length > maxTags) {
    items = [...genres.slice(0, maxTags - 1), "..."];
  } else {
    items = [...genres];
  }

  const rows: string[][] = [];
  const pattern = [3, 2, 1, 2, 1];
  let currentIdx = 0;

  for (const count of pattern) {
    if (currentIdx >= items.length) break;
    rows.push(items.slice(currentIdx, currentIdx + count));
    currentIdx += count;
  }

  if (currentIdx < items.length) {
    rows.push(items.slice(currentIdx));
  }

  return rows;
}

export function ProjectDetailSidebar({ project, onClose }: ProjectDetailSidebarProps) {
  if (!project) return null;

  const pyramidRows = getPyramidRows(project.genres);

  return (
    <aside className="fixed inset-y-0 right-0 z-40 flex w-full max-w-sm flex-col border-l border-slate-200 bg-white shadow-2xl transition-all duration-300 dark:border-slate-800 dark:bg-slate-950 sm:max-w-md">
      {/* Header bar */}
      <div className="flex h-14 shrink-0 items-center justify-between border-b border-slate-100 px-5 dark:border-slate-800/80">
        <span className="text-xs font-semibold uppercase tracking-wider text-slate-400 dark:text-slate-500">
          Project Overview
        </span>
        <button
          type="button"
          onClick={onClose}
          className="rounded-lg p-1.5 text-slate-400 hover:bg-slate-100 hover:text-slate-700 dark:hover:bg-slate-800 dark:hover:text-slate-200 transition-colors"
          title="Đóng sidebar"
        >
          <PanelRightClose className="h-4 w-4" />
        </button>
      </div>

      {/* Main scrollable content */}
      <div className="flex-1 overflow-y-auto p-6 space-y-6">
        {/* Cover SVG căn center, tỉ lệ 3x5 */}
        <div className="flex flex-col items-center">
          <div className="w-40 aspect-[3/5] overflow-hidden rounded-xl border border-slate-200 bg-slate-50 shadow-md dark:border-slate-800 dark:bg-slate-900">
            <img
              src={project.cover}
              alt={project.title}
              className="h-full w-full object-cover"
            />
          </div>

          {/* Title */}
          <h3 className="mt-4 text-center text-base font-bold tracking-tight text-slate-900 dark:text-slate-100">
            {project.title}
          </h3>

          {/* Status chữ nhật không radius */}
          <div className="mt-2">
            <span className="rounded-none bg-slate-900 px-2.5 py-1 text-[10px] font-bold uppercase tracking-wider text-white dark:bg-slate-100 dark:text-slate-900 shadow-2xs">
              {project.status}
            </span>
          </div>
        </div>

        {/* Hàng loạt tag genres xếp hình tháp 3 2 1 */}
        <div className="flex flex-col items-center gap-1.5 w-full">
          {pyramidRows.map((row, rIdx) => (
            <div key={rIdx} className="flex flex-wrap items-center justify-center gap-1.5">
              {row.map((tag, tIdx) => (
                <span
                  key={tIdx}
                  className="rounded-md border border-slate-200 bg-slate-50 px-2 py-0.5 text-[11px] font-medium text-slate-600 dark:border-slate-800 dark:bg-slate-900 dark:text-slate-300"
                >
                  {tag}
                </span>
              ))}
            </div>
          ))}
        </div>

        {/* Description */}
        <div className="space-y-1.5">
          <h4 className="text-xs font-semibold text-slate-900 dark:text-slate-100">
            Description
          </h4>
          <p className="text-xs leading-relaxed text-slate-600 dark:text-slate-300">
            {project.description}
          </p>
        </div>

        {/* Details section */}
        <div className="border-t border-slate-100 pt-5 dark:border-slate-800/80 space-y-3">
          <h4 className="text-xs font-semibold text-slate-900 dark:text-slate-100">
            Details
          </h4>

          <div className="grid grid-cols-2 gap-2.5 text-xs">
            <div className="rounded-xl border border-slate-100 bg-slate-50/70 p-3 dark:border-slate-800/60 dark:bg-slate-900/40">
              <div className="flex items-center gap-1.5 text-slate-400 mb-1">
                <BookOpen className="h-3.5 w-3.5" />
                <span className="text-[10px] uppercase">Word Count</span>
              </div>
              <span className="font-semibold text-slate-800 dark:text-slate-200">
                {project.details.wordCount}
              </span>
            </div>

            <div className="rounded-xl border border-slate-100 bg-slate-50/70 p-3 dark:border-slate-800/60 dark:bg-slate-900/40">
              <div className="flex items-center gap-1.5 text-slate-400 mb-1">
                <Layers className="h-3.5 w-3.5" />
                <span className="text-[10px] uppercase">Chapters</span>
              </div>
              <span className="font-semibold text-slate-800 dark:text-slate-200">
                {project.details.chapters}
              </span>
            </div>

            <div className="rounded-xl border border-slate-100 bg-slate-50/70 p-3 dark:border-slate-800/60 dark:bg-slate-900/40">
              <div className="flex items-center gap-1.5 text-slate-400 mb-1">
                <Users className="h-3.5 w-3.5" />
                <span className="text-[10px] uppercase">Characters</span>
              </div>
              <span className="font-semibold text-slate-800 dark:text-slate-200">
                {project.details.characters}
              </span>
            </div>

            <div className="rounded-xl border border-slate-100 bg-slate-50/70 p-3 dark:border-slate-800/60 dark:bg-slate-900/40">
              <div className="flex items-center gap-1.5 text-slate-400 mb-1">
                <Clock className="h-3.5 w-3.5" />
                <span className="text-[10px] uppercase">World Lore</span>
              </div>
              <span className="font-semibold text-slate-800 dark:text-slate-200">
                {project.details.worldLore}
              </span>
            </div>

            <div className="col-span-2 flex items-center justify-between rounded-xl border border-slate-100 bg-slate-50/70 p-3 dark:border-slate-800/60 dark:bg-slate-900/40">
              <div className="flex items-center gap-1.5 text-slate-400">
                <Calendar className="h-3.5 w-3.5" />
                <span className="text-[10px] uppercase">Last Modified</span>
              </div>
              <span className="font-medium text-slate-700 dark:text-slate-300">
                {project.modifiedAt}
              </span>
            </div>
          </div>
        </div>
      </div>
    </aside>
  );
}
