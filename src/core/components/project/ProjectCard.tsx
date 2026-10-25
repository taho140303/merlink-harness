import { Eye, Check } from "lucide-react";
import { cn } from "../../../shared/utils/cn";
import type { ProjectItem } from "./types";

interface ProjectCardProps {
  project: ProjectItem;
  isSelected: boolean;
  onToggleSelect: (id: string) => void;
  onOverview: (project: ProjectItem) => void;
}

export function ProjectCard({
  project,
  isSelected,
  onToggleSelect,
  onOverview,
}: ProjectCardProps) {
  return (
    <div
      className={cn(
        "group relative flex rounded-2xl border bg-white p-4 shadow-xs transition-all duration-200 dark:bg-slate-900",
        isSelected
          ? "border-blue-500 ring-2 ring-blue-500/20"
          : "border-slate-200 hover:border-slate-300 hover:shadow-md dark:border-slate-800 dark:hover:border-slate-700"
      )}
    >
      {/* Radio chọn từng project */}
      <button
        type="button"
        onClick={() => onToggleSelect(project.id)}
        className={cn(
          "absolute right-3.5 top-3.5 z-10 flex h-4.5 w-4.5 items-center justify-center rounded-full border transition-all",
          isSelected
            ? "border-blue-600 bg-blue-600 text-white"
            : "border-slate-300 bg-white opacity-40 hover:opacity-100 dark:border-slate-600 dark:bg-slate-800"
        )}
        title="Chọn dự án"
      >
        {isSelected && <Check className="h-3 w-3 stroke-[3]" />}
      </button>

      {/* Cover SVG tỉ lệ 3x5 */}
      <div className="w-24 shrink-0 sm:w-28 aspect-[3/5] overflow-hidden rounded-xl border border-slate-100 bg-slate-50 dark:border-slate-800 dark:bg-slate-950">
        <img
          src={project.cover}
          alt={project.title}
          className="h-full w-full object-cover transition-transform duration-300 group-hover:scale-105"
        />
      </div>

      {/* Thông tin bên phải */}
      <div className="ml-4 flex flex-1 flex-col justify-between">
        <div>
          <div className="flex items-start justify-between gap-2 pr-6">
            <h3 className="text-sm font-semibold tracking-tight text-slate-900 dark:text-slate-100 line-clamp-1">
              {project.title}
            </h3>
          </div>

          <div className="mt-1">
            <span className="rounded-none bg-slate-100 px-2 py-0.5 text-[10px] font-semibold text-slate-700 dark:bg-slate-800 dark:text-slate-300">
              {project.status}
            </span>
          </div>

          <p className="mt-2 text-xs leading-relaxed text-slate-500 dark:text-slate-400 line-clamp-3">
            {project.description}
          </p>
        </div>

        {/* Nút Overview */}
        <div className="mt-4 flex items-center justify-end border-t border-slate-100 pt-2.5 dark:border-slate-800/80">
          <button
            type="button"
            onClick={() => onOverview(project)}
            className="flex items-center gap-1.5 rounded-xl border border-slate-200 bg-slate-50 px-3.5 py-1.5 text-xs font-medium text-slate-700 shadow-2xs hover:bg-slate-100 hover:text-slate-900 dark:border-slate-700 dark:bg-slate-800 dark:text-slate-300 dark:hover:bg-slate-750 transition-colors"
          >
            <Eye className="h-3.5 w-3.5" />
            <span>Overview</span>
          </button>
        </div>
      </div>
    </div>
  );
}
