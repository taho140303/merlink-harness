import { useNavigate } from "react-router";
import { ChevronRight } from "lucide-react";
import { Badge } from "@/shared/components/ui/badge";
import { useT } from "@/shared/i18n/useT";
import { buildCreatorChapterPath } from "@/features/draft/commitNavigation";
import type { BackendClient } from "@/shared/backend/client";
import { sanitizeDisplayLabel } from "./ProjectView";
import type { ProjectItem } from "./types";

export interface ProjectCreatorPanelProps {
  project: ProjectItem;
  client?: BackendClient;
}

/**
 * Chapter-entry surface for Creator mode. It deliberately renders no scene
 * editor: choosing a chapter opens the existing Chapter Detail route.
 */
export function ProjectCreatorPanel({ project }: ProjectCreatorPanelProps) {
  const t = useT();
  const navigate = useNavigate();
  const arcs = (project.arcs ?? []).filter((arc) => arc.chapters.length > 0);
  const chapterNumberById = new Map(
    arcs
      .flatMap((arc) => arc.chapters)
      .map((chapter, index) => [chapter.id, index + 1] as const),
  );

  return (
    <div className="mt-6 space-y-6">
      <div>
        <h3 className="text-xs font-semibold uppercase tracking-wider text-fg-subtle">
          {t("project.creator.title")}
        </h3>
        <p className="mt-1 text-xs text-fg-muted">{t("project.creator.description")}</p>
      </div>

      {arcs.length === 0 ? (
        <div className="rounded-2xl border border-line bg-surface-card p-5">
          <p className="text-xs text-fg-muted">{t("project.creator.empty")}</p>
        </div>
      ) : (
        <div className="space-y-6">
          {arcs.map((arc) => {
            const cleanArcTitle = sanitizeDisplayLabel(arc.title, arc.id, "Hồi");
            return (
              <div
                key={arc.id}
                className="overflow-hidden rounded-2xl border border-line bg-surface-card"
              >
                <div className="border-b border-line bg-surface-muted/70 px-5 py-3">
                  <h4 className="text-sm font-bold text-fg">{cleanArcTitle}</h4>
                </div>

                <div className="space-y-0.5 p-3">
                  {arc.chapters.map((chapter) => {
                    const sceneCount = chapter.blocks?.length ?? 0;
                    const cleanChapterTitle = sanitizeDisplayLabel(
                      chapter.title,
                      chapter.id,
                      "Chương"
                    );

                    return (
                      <button
                        key={chapter.id}
                        type="button"
                        onClick={() =>
                          navigate(buildCreatorChapterPath(project.id, chapter.id))
                        }
                        className="focus-ring flex w-full items-center justify-between gap-3 rounded-xl border-b border-line px-3 py-2.5 text-left text-xs transition-colors last:border-b-0 hover:bg-surface-muted"
                      >
                        <span className="min-w-0 truncate text-fg">
                          {t("project.chapter.title", {
                            index: chapterNumberById.get(chapter.id) ?? 0,
                            title: cleanChapterTitle,
                          })}
                        </span>
                        <span className="flex shrink-0 items-center gap-2">
                          <Badge variant="neutral">
                            {t("project.creator.sceneCount", { count: sceneCount })}
                          </Badge>
                          <ChevronRight
                            className="h-3.5 w-3.5 text-fg-subtle"
                            aria-hidden="true"
                          />
                        </span>
                      </button>
                    );
                  })}
                </div>
              </div>
            );
          })}
        </div>
      )}
    </div>
  );
}
