import { Eye, Trash2 } from "lucide-react";
import type { ProjectItem } from "./types";
import { useT } from "../../../shared/i18n/useT";

interface ProjectCardProps {
  project: ProjectItem;
  onDelete: (project: ProjectItem) => void;
  onOverview: (project: ProjectItem) => void;
}

export function ProjectCard({ project, onDelete, onOverview }: ProjectCardProps) {
  const t = useT();
  return (
    <div className="group relative flex rounded-2xl border border-slate-200 bg-white p-4 shadow-xs transition-all duration-200 hover:border-slate-300 hover:shadow-md dark:border-slate-800 dark:bg-slate-900 dark:hover:border-slate-700">
      <button
        type="button"
        onClick={() => onDelete(project)}
        title={t("project.trashTooltip")}
        className="absolute right-3.5 top-3.5 z-10 flex h-7 w-7 items-center justify-center rounded-lg border border-slate-200 bg-white text-slate-400 shadow-2xs transition-colors hover:border-rose-200 hover:bg-rose-50 hover:text-rose-500 dark:border-slate-700 dark:bg-slate-800 dark:text-slate-400 dark:hover:border-rose-900/60 dark:hover:bg-rose-950/40 dark:hover:text-rose-300"
      >
        <Trash2 className="h-3.5 w-3.5" />
      </button>

      <div className="aspect-[3/5] w-24 shrink-0 overflow-hidden rounded-xl border border-slate-100 bg-slate-50 dark:border-slate-800 dark:bg-slate-950 sm:w-28">
        <img
          src={project.cover}
          alt={project.title}
          className="h-full w-full object-cover transition-transform duration-300 group-hover:scale-105"
        />
      </div>

      <div className="ml-4 flex flex-1 flex-col justify-between">
        <div>
          <div className="flex items-start justify-between gap-2 pr-8">
            <h3 className="line-clamp-1 text-sm font-semibold tracking-tight text-slate-900 dark:text-slate-100">
              {project.title}
            </h3>
          </div>
          <div className="mt-1">
            <span className="rounded-none bg-slate-100 px-2 py-0.5 text-[10px] font-semibold text-slate-700 dark:bg-slate-800 dark:text-slate-300">
              {project.status}
            </span>
          </div>
          <p className="mt-2 line-clamp-3 text-xs leading-relaxed text-slate-500 dark:text-slate-400">
            {project.description}
          </p>
        </div>

        <div className="mt-4 flex items-center justify-end border-t border-slate-100 pt-2.5 dark:border-slate-800/80">
          <button
            type="button"
            onClick={() => onOverview(project)}
            className="flex items-center gap-1.5 rounded-xl border border-slate-200 bg-slate-50 px-3.5 py-1.5 text-xs font-medium text-slate-700 shadow-2xs transition-colors hover:bg-slate-100 hover:text-slate-900 dark:border-slate-700 dark:bg-slate-800 dark:text-slate-300 dark:hover:bg-slate-750"
          >
            <Eye className="h-3.5 w-3.5" />
            <span>{t("project.overview")}</span>
          </button>
        </div>
      </div>
    </div>
  );
}
