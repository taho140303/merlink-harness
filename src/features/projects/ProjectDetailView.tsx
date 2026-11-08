import { useState, useEffect } from "react";
import { Link, useNavigate, useSearchParams, useParams } from "react-router";
import { Sparkles, MessageSquare, GripVertical } from "lucide-react";
import { cn } from "@/shared/utils/cn";
import { truncate } from "@/shared/utils/string";
import { useResizablePanel } from "@/shared/hooks/useResizablePanel";
import { useIsCompact } from "@/shared/hooks/useMediaQuery";
import { Dialog, DialogContent, DialogDescription, DialogTitle } from "@/shared/components/ui/dialog";
import { Segmented } from "@/shared/components/ui/segmented";
import { useT } from "@/shared/i18n/useT";
import { backendClient, type BackendClient } from "@/shared/backend/client";
import { ProjectChatPane } from "./ProjectChatPane";
import { ProjectCreatorPanel } from "./ProjectCreatorPanel";
import { sanitizeDisplayLabel, mapProjectDtoToItem, sanitizeProjectItem } from "./ProjectView";
import type { ProjectItem, ArcItem } from "./types";

export interface ProjectDetailViewProps {
  project?: ProjectItem;
  projectId?: string;
  client?: BackendClient;
}

export function ProjectDetailView({
  project: initialProject,
  projectId: propProjectId,
  client: propClient,
}: ProjectDetailViewProps = {}) {
  const t = useT();
  const isCompact = useIsCompact();
  const navigate = useNavigate();
  const { projectId: routeProjectId } = useParams();
  const [searchParams, setSearchParams] = useSearchParams();
  const mode = searchParams.get("mode") === "creator" ? "creator" : "view";
  const [isChatOpen, setIsChatOpen] = useState(false);
  const [assetDialogOpen, setAssetDialogOpen] = useState(false);
  const [expandedArcs, setExpandedArcs] = useState<Record<string, boolean>>({});

  const targetId = propProjectId || routeProjectId || initialProject?.id;
  const activeClient = propClient || backendClient;

  const [currentProject, setCurrentProject] = useState<ProjectItem | null>(() => {
    if (initialProject) return sanitizeProjectItem(initialProject);
    return null;
  });
  const [notFound, setNotFound] = useState(false);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    let isMounted = true;
    if (!targetId) {
      if (!initialProject) {
        setNotFound(true);
      }
      return;
    }

    if (activeClient && activeClient.status !== "unavailable") {
      setLoading(true);
      activeClient
        .getProjectDetail({ projectId: targetId })
        .then((res) => {
          if (!isMounted) return;
          if (!res?.project) {
            setNotFound(true);
            setCurrentProject(null);
          } else {
            setCurrentProject(mapProjectDtoToItem(res.project));
            setNotFound(false);
          }
        })
        .catch(() => {
          if (!isMounted) return;
          // Absent or NOT_FOUND: render explicit not-found state, never mock fallback!
          setNotFound(true);
          setCurrentProject(null);
        })
        .finally(() => {
          if (isMounted) setLoading(false);
        });
    } else if (initialProject) {
      setCurrentProject(sanitizeProjectItem(initialProject));
      setNotFound(false);
    } else {
      setNotFound(true);
    }

    return () => {
      isMounted = false;
    };
  }, [targetId, activeClient, initialProject]);

  const handleModeChange = (nextMode: string) => {
    const next = new URLSearchParams(searchParams);
    if (nextMode === "creator") next.set("mode", "creator");
    else next.delete("mode");
    setSearchParams(next);
  };

  const { leftRatio, setLeftRatio, isDragging, containerRef, startDragging } = useResizablePanel({
    initialRatio: 45,
    minPx: 340,
  });

  if (notFound || (!currentProject && !loading)) {
    return (
      <div
        data-testid="project-not-found"
        className="flex h-full flex-col items-center justify-center p-6 text-center"
      >
        <p className="text-sm text-fg-muted">{t("project.notFound")}</p>
        <Link to="/project" className="mt-3 text-xs font-semibold text-blue-600 hover:underline">
          {t("project.backToList")}
        </Link>
      </div>
    );
  }

  if (loading && !currentProject) {
    return (
      <div className="flex h-full items-center justify-center p-6 text-center text-xs text-fg-muted">
        Đang tải...
      </div>
    );
  }

  const project = currentProject!;
  const arcs: ArcItem[] = project.arcs && project.arcs.length > 0 ? project.arcs : [];

  const toggleArc = (arcId: string) => {
    setExpandedArcs((prev) => ({ ...prev, [arcId]: !prev[arcId] }));
  };

  return (
    <div
      ref={containerRef}
      className={cn(
        "relative flex h-full w-full overflow-hidden",
        isDragging && "cursor-col-resize select-none"
      )}
    >
      {/* Left pane displays the chat when opened */}
      {isChatOpen && (
        <div
          style={isCompact ? undefined : { width: `${leftRatio}%` }}
          className={cn('h-full overflow-hidden', isCompact && 'absolute inset-0 z-40 w-full bg-surface-panel')}
        >
          <ProjectChatPane
            projectName={project.title}
            onClose={() => setIsChatOpen(false)}
          />
        </div>
      )}

      {/* Draggable column divider */}
      {isChatOpen && !isCompact && (
        <div
          role="separator"
          tabIndex={0}
          aria-label={t('project.resizePanels')}
          aria-orientation="vertical"
          aria-valuenow={Math.round(leftRatio)}
          aria-valuemin={20}
          aria-valuemax={80}
          onMouseDown={startDragging}
          onKeyDown={(event) => {
            if (event.key !== 'ArrowLeft' && event.key !== 'ArrowRight') return;
            event.preventDefault();
            const delta = event.key === 'ArrowLeft' ? -5 : 5;
            setLeftRatio(Math.min(80, Math.max(20, leftRatio + delta)));
          }}
          className="group relative flex w-2 shrink-0 cursor-col-resize items-center justify-center bg-transparent transition-colors hover:bg-blue-500/10 dark:hover:bg-blue-500/20"
        >
          <div className="h-full w-px bg-surface-muted transition-colors group-hover:bg-blue-500" />
          <div
            className={cn(
              "absolute z-30 flex h-8 w-4.5 items-center justify-center rounded-md border border-line-strong bg-surface-card transition-opacity",
              isDragging ? "opacity-100 ring-2 ring-blue-500" : "opacity-0 group-hover:opacity-100"
            )}
          >
            <GripVertical className="h-3.5 w-3.5 text-fg-muted" />
          </div>
        </div>
      )}

      {/* Right pane displays project details */}
      <div
        style={{ width: isChatOpen && !isCompact ? `${100 - leftRatio}%` : "100%" }}
        className="flex h-full flex-col overflow-y-auto bg-surface-muted/50 p-6 sm:p-8"
      >
        {/* Project breadcrumb header */}
        <div className="flex h-12 shrink-0 items-center justify-between border-b border-line pb-4">
          <div className="flex items-center gap-2 text-xs font-semibold text-fg-muted">
            <Link to="/project" className="hover:text-fg">
              {t("project.title")}
            </Link>
            <span>›</span>
            <span className="text-fg font-bold" title={project.title}>
              {truncate(project.title, 15)}
            </span>
          </div>

          <div className="flex shrink-0 items-center gap-2">
            <Segmented
              items={[
                { id: "view", label: t("project.mode.view") },
                { id: "creator", label: t("project.mode.creator") },
              ]}
              value={mode}
              onChange={handleModeChange}
              ariaLabel={t("project.mode.aria")}
            />

            <button
              type="button"
              onClick={() => setIsChatOpen(!isChatOpen)}
              className={cn(
                "flex items-center gap-1.5 rounded-xl border px-3 py-1.5 text-xs font-medium transition-colors",
                isChatOpen
                  ? "border-blue-500 bg-blue-50 text-blue-700 dark:border-blue-600 dark:bg-blue-950/60 dark:text-blue-300"
                  : "border-line bg-surface-card text-fg-muted hover:bg-surface-muted"
              )}
            >
              <MessageSquare className="h-3.5 w-3.5" />
              <span>{t("project.chat.label")}</span>
            </button>
          </div>
        </div>

        {mode === "view" ? (
          <div className="mt-6 space-y-6">
            {/* Overview Card */}
            <div className="flex flex-col gap-5 sm:flex-row tier-card rounded-card p-5">
              <div className="w-28 aspect-[3/5] shrink-0 overflow-hidden rounded-xl border border-line bg-surface-muted">
                <img src={project.cover} alt={project.title} className="h-full w-full object-cover" />
              </div>

              <div className="flex flex-1 flex-col justify-between">
                <div>
                  <div className="flex items-center gap-2.5">
                    <h2 className="text-base font-bold text-fg">
                      {project.title}
                    </h2>
                    <span className="rounded-full bg-accent px-2 py-0.5 text-xs font-bold uppercase text-white">
                      {project.status}
                    </span>
                  </div>
                  <p className="mt-2 text-xs leading-relaxed text-fg-muted">
                    {project.description}
                  </p>
                </div>

                <div className="mt-4 grid grid-cols-2 gap-2 text-xs sm:grid-cols-4">
                  <div className="rounded-xl bg-surface-muted p-2.5 border border-line">
                    <span className="text-xs text-fg-subtle block uppercase">{t("project.words")}</span>
                    <span className="font-semibold text-fg">{project.details.wordCount}</span>
                  </div>
                  <div className="rounded-xl bg-surface-muted p-2.5 border border-line">
                    <span className="text-xs text-fg-subtle block uppercase">{t("project.chapters")}</span>
                    <span className="font-semibold text-fg">{project.details.chapters}</span>
                  </div>
                  <div className="rounded-xl bg-surface-muted p-2.5 border border-line">
                    <span className="text-xs text-fg-subtle block uppercase">{t("project.characters")}</span>
                    <span className="font-semibold text-fg">{project.details.characters}</span>
                  </div>
                  <div className="rounded-xl bg-surface-muted p-2.5 border border-line">
                    <span className="text-xs text-fg-subtle block uppercase">{t("project.modified")}</span>
                    <span className="font-semibold text-fg">{project.modifiedAt}</span>
                  </div>
                </div>
              </div>
            </div>

            {/* Arcs Container */}
            <div className="space-y-6">
              <h3 className="text-xs font-semibold uppercase tracking-wider text-fg-subtle">
                {t("project.arcsRegistry")}
              </h3>

              {arcs.map((arc) => {
                const isExpanded = expandedArcs[arc.id] ?? false;
                const visibleChapters = isExpanded ? arc.chapters : arc.chapters.slice(0, 5);
                const hasMore = arc.chapters.length > 5;

                return (
                  <div
                    key={arc.id}
                    className="overflow-hidden rounded-2xl border border-line bg-surface-card"
                  >
                    <div className="border-b border-line bg-surface-muted/70 px-5 py-3">
                      <h4 className="text-sm font-bold text-fg">
                        {arc.title}
                      </h4>
                    </div>

                    <div className="flex flex-col sm:flex-row p-5 gap-5">
                      {/* Left column: Arc cover and assets action */}
                      <div className="flex flex-col items-center shrink-0 w-32 mx-auto sm:mx-0">
                        <div className="w-28 aspect-[3/5] overflow-hidden rounded-xl border border-line bg-surface-muted">
                          <img
                            src={arc.cover || project.cover}
                            alt={arc.title}
                            className="h-full w-full object-cover"
                          />
                        </div>

                        <button
                          type="button"
                          onClick={() => setAssetDialogOpen(true)}
                          className="mt-3 flex w-full items-center justify-center gap-1.5 rounded-lg border border-line bg-surface-muted py-1.5 text-xs font-medium text-fg-muted hover:bg-surface-muted transition-colors"
                        >
                          <Sparkles className="h-3 w-3 text-blue-500" />
                          <span>{t("project.assetsButton")}</span>
                        </button>
                      </div>

                      {/* Right column: Chapter list */}
                      <div className="flex-1 space-y-0.5">
                        {visibleChapters.map((chap, idx) => (
                          <div
                            key={chap.id}
                            onClick={() => navigate(`/project/${project.id}/chapter/${chap.id}`)}
                            className="flex cursor-pointer items-center justify-between py-2.5 px-3 border-b border-line text-xs rounded-xl hover:bg-surface-muted transition-colors"
                          >
                            <div className="flex items-center gap-2 truncate pr-2">
                              {chap.isNew && (
                                <span className="font-semibold text-rose-500 shrink-0">{t("project.chapter.new")}</span>
                              )}
                              <span className="truncate text-fg">
                                {t("project.chapter.title", { index: idx + 1, title: chap.title })}
                              </span>
                            </div>
                            <span className="text-xs text-fg-subtle shrink-0 font-mono">
                              {chap.date}
                            </span>
                          </div>
                        ))}

                        {hasMore && (
                          <div className="pt-2 px-3">
                            <button
                              type="button"
                              onClick={() => toggleArc(arc.id)}
                              className="text-xs font-semibold text-fg hover:text-blue-600 dark:hover:text-blue-400 transition-colors"
                            >
                              {isExpanded
                                ? t("project.chapter.collapse")
                                : t("project.chapter.showMore", { count: arc.chapters.length - 5 })}
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
        ) : (
          <ProjectCreatorPanel project={project} client={activeClient} />
        )}

        {/* Dialog Assets */}
        <Dialog open={assetDialogOpen} onOpenChange={setAssetDialogOpen}>
          <DialogContent className="max-w-xs p-6 text-center">
            <DialogTitle className="sr-only">{t('project.assets')}</DialogTitle>
            <DialogDescription className="sr-only">{t('project.assetsEmpty')}</DialogDescription>
            <div className="py-4">
              <p className="text-sm font-medium text-fg-muted">{t("project.assets.emptyLabel")}</p>
            </div>
            <button
              type="button"
              onClick={() => setAssetDialogOpen(false)}
              className="w-full rounded-xl bg-accent py-2 text-xs font-medium text-white hover:opacity-90"
            >
              {t('common.close')}
            </button>
          </DialogContent>
        </Dialog>
      </div>
    </div>
  );
}
