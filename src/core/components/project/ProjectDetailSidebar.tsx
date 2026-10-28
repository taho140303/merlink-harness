import { useNavigate } from "react-router";
import { PanelRightClose, BookOpen, Layers, Users, Calendar, Clock, ExternalLink } from "lucide-react";
import type { ProjectItem } from "./types";
import { useT } from "../../../shared/i18n/useT";

interface ProjectDetailSidebarProps {
  project: ProjectItem | null;
  onClose: () => void;
}

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
  const navigate = useNavigate();
  const t = useT();

  if (!project) return null;

  const pyramidRows = getPyramidRows(project.genres);

  const handleGoToDetail = () => {
    onClose();
    navigate(`/project/${project.id}`);
  };

  return (
    <aside className="fixed inset-y-0 right-0 z-40 flex w-full max-w-sm flex-col border-l border-line bg-surface-card transition-colors duration-300 sm:max-w-md">
      {/* Header bar */}
      <div className="flex h-14 shrink-0 items-center justify-between border-b border-line px-5 ">
        <span className="text-xs font-semibold uppercase tracking-wider text-fg-subtle">
          {t("project.overview")}
        </span>
        <button
          type="button"
          onClick={onClose}
          className="rounded-lg p-1.5 text-fg-subtle hover:bg-surface-muted hover:text-fg-muted   transition-colors"
          title={t("project.sidebar.close")} aria-label={t("project.sidebar.close")}
        >
          <PanelRightClose className="h-4 w-4" />
        </button>
      </div>

      {/* Main scrollable content */}
      <div className="flex-1 overflow-y-auto p-6 space-y-6">
        <div className="flex flex-col items-center">
          <div className="w-40 aspect-[3/5] overflow-hidden rounded-xl border border-line bg-surface-muted   ">
            <img
              src={project.cover}
              alt={project.title}
              className="h-full w-full object-cover"
            />
          </div>

          <h3 className="mt-4 text-center text-base font-bold tracking-tight text-fg">
            {project.title}
          </h3>

          <div className="mt-2">
            <span className="rounded-full bg-accent px-2.5 py-1 text-xs font-bold uppercase tracking-wider text-white   ">
              {project.status}
            </span>
          </div>
        </div>

        {/* Pyramid-shaped genre tags */}
        <div className="flex flex-col items-center gap-1.5 w-full">
          {pyramidRows.map((row, rIdx) => (
            <div key={rIdx} className="flex flex-wrap items-center justify-center gap-1.5">
              {row.map((tag, tIdx) => (
                <span
                  key={tIdx}
                  className="rounded-md border border-line bg-surface-muted px-2 py-0.5 text-xs font-medium text-fg-muted   "
                >
                  {tag}
                </span>
              ))}
            </div>
          ))}
        </div>

        {/* Description */}
        <div className="space-y-1.5">
          <h4 className="text-xs font-semibold text-fg">
            {t("project.detail.description")}
          </h4>
          <p className="text-xs leading-relaxed text-fg-muted">
            {project.description}
          </p>
        </div>

        {/* Details section */}
        <div className="border-t border-line pt-5  space-y-3">
          <h4 className="text-xs font-semibold text-fg">
            {t("project.detail.details")}
          </h4>

          <div className="grid grid-cols-2 gap-2.5 text-xs">
            <div className="rounded-xl border border-line bg-surface-muted/70 p-3  ">
              <div className="flex items-center gap-1.5 text-fg-subtle mb-1">
                <BookOpen className="h-3.5 w-3.5" />
                <span className="text-xs uppercase">{t("project.words")}</span>
              </div>
              <span className="font-semibold text-fg ">
                {project.details.wordCount}
              </span>
            </div>

            <div className="rounded-xl border border-line bg-surface-muted/70 p-3  ">
              <div className="flex items-center gap-1.5 text-fg-subtle mb-1">
                <Layers className="h-3.5 w-3.5" />
                <span className="text-xs uppercase">{t("project.chapters")}</span>
              </div>
              <span className="font-semibold text-fg ">
                {project.details.chapters}
              </span>
            </div>

            <div className="rounded-xl border border-line bg-surface-muted/70 p-3  ">
              <div className="flex items-center gap-1.5 text-fg-subtle mb-1">
                <Users className="h-3.5 w-3.5" />
                <span className="text-xs uppercase">{t("project.characters")}</span>
              </div>
              <span className="font-semibold text-fg ">
                {project.details.characters}
              </span>
            </div>

            <div className="rounded-xl border border-line bg-surface-muted/70 p-3  ">
              <div className="flex items-center gap-1.5 text-fg-subtle mb-1">
                <Clock className="h-3.5 w-3.5" />
                <span className="text-xs uppercase">{t("project.worldLore")}</span>
              </div>
              <span className="font-semibold text-fg ">
                {project.details.worldLore}
              </span>
            </div>

            <div className="col-span-2 flex items-center justify-between rounded-xl border border-line bg-surface-muted/70 p-3  ">
              <div className="flex items-center gap-1.5 text-fg-subtle">
                <Calendar className="h-3.5 w-3.5" />
                <span className="text-xs uppercase">{t("project.modified")}</span>
              </div>
              <span className="font-medium text-fg-muted ">
                {project.modifiedAt}
              </span>
            </div>
          </div>
        </div>
      </div>

      {/* Detail action navigates to the full page */}
      <div className="shrink-0 border-t border-line p-4 ">
        <button
          type="button"
          onClick={handleGoToDetail}
          className="flex w-full items-center justify-center gap-2 rounded-xl bg-accent py-2.5 text-xs font-semibold text-white  hover:opacity-90    transition-colors"
        >
          <ExternalLink className="h-3.5 w-3.5" />
          <span>{t("project.detail")}</span>
        </button>
      </div>
    </aside>
  );
}
