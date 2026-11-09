import { Eye, Trash2 } from "lucide-react";
import type { ProjectItem } from "./types";
import { useT } from "@/shared/i18n/useT";
import { useDbEnumLabel } from "@/shared/i18n/dbEnums";

interface ProjectCardProps {
  project: ProjectItem;
  onDelete: (project: ProjectItem) => void;
  onOverview: (project: ProjectItem) => void;
}

export function ProjectCard({ project, onDelete, onOverview }: ProjectCardProps) {
  const t = useT();
  const enumLabel = useDbEnumLabel();
  return (
    <div className="group relative flex rounded-shell-sm p-5 neu-flat transition-colors duration-300">
      <button
        type="button"
        onClick={() => onDelete(project)}
        title={t("project.trashTooltip")}
        className="absolute right-4 top-4 z-10 flex h-8 w-8 items-center justify-center rounded-xl text-fg-subtle transition-colors hover:neu-pressed hover:text-rose-500"
      >
        <Trash2 className="h-4 w-4" />
      </button>

      <div className="aspect-[3/5] w-24 shrink-0 overflow-hidden rounded-2xl ">
        <img
          src={project.cover}
          alt={project.title}
          className="h-full w-full object-cover transition-transform duration-300 group-hover:scale-105"
        />
      </div>

      <div className="ml-5 flex flex-1 flex-col justify-between">
        <div>
          <div className="flex items-start justify-between gap-2 pr-8">
            <h3 className="line-clamp-1 text-sm font-bold text-fg">
              {project.title}
            </h3>
          </div>
          <div className="mt-2">
            <span className="rounded-lg px-2 py-0.5 text-xs font-bold uppercase tracking-wider text-indigo-600 neu-pressed dark:text-indigo-400">
              {enumLabel('ContentStatus', project.status.toUpperCase().replaceAll(' ', '_'))}
            </span>
          </div>
          <p className="mt-3 line-clamp-3 text-xs leading-relaxed text-fg-muted -subtle">
            {project.description}
          </p>
        </div>

        <div className="mt-4 flex items-center justify-end">
          <button
            type="button"
            onClick={() => onOverview(project)}
            className="flex items-center gap-1.5 rounded-xl px-4 py-2 text-xs font-bold text-fg-muted transition-colors hover:neu-pressed "
          >
            <Eye className="h-4 w-4" />
            <span>{t("project.overview")}</span>
          </button>
        </div>
      </div>
    </div>
  );
}
