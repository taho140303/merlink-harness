import { Eye } from "lucide-react";
import type { ProjectItem } from "./types";

interface ProjectCardProps {
  project: ProjectItem;
  onView: (project: ProjectItem) => void;
}

export function ProjectCard({ project, onView }: ProjectCardProps) {
  return (
    <div className="group flex rounded-2xl border border-slate-200 bg-white p-4 shadow-xs transition-all duration-200 hover:border-slate-300 hover:shadow-md dark:border-slate-800 dark:bg-slate-900 dark:hover:border-slate-700">
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
          <div className="flex items-start justify-between gap-2">
            <h3 className="text-sm font-semibold tracking-tight text-slate-900 dark:text-slate-100 line-clamp-1">
              {project.title}
            </h3>
            {/* Tag chữ nhật không radius */}
            <span className="shrink-0 rounded-none bg-slate-100 px-2 py-0.5 text-[10px] font-semibold text-slate-700 dark:bg-slate-800 dark:text-slate-300">
              {project.status}
            </span>
          </div>

          <p className="mt-2 text-xs leading-relaxed text-slate-500 dark:text-slate-400 line-clamp-3">
            {project.description}
          </p>
        </div>

        {/* Nút View thay thế share và read more */}
        <div className="mt-4 flex items-center justify-end border-t border-slate-100 pt-2.5 dark:border-slate-800/80">
          <button
            type="button"
            onClick={() => onView(project)}
            className="flex items-center gap-1.5 rounded-xl border border-slate-200 bg-slate-50 px-3.5 py-1.5 text-xs font-medium text-slate-700 shadow-2xs hover:bg-slate-100 hover:text-slate-900 dark:border-slate-700 dark:bg-slate-800 dark:text-slate-300 dark:hover:bg-slate-750 transition-colors"
          >
            <Eye className="h-3.5 w-3.5" />
            <span>View</span>
          </button>
        </div>
      </div>
    </div>
  );
}
