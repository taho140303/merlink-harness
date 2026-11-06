import { useState, useRef, useEffect, useMemo } from "react";
import { Link, useBlocker } from "react-router";
import {
  Pencil,
  MessageSquare,
  Lock,
  Unlock,
  GripVertical,
  X,
  Check,
  Sparkles,
  RotateCcw,
  Loader2,
  History,
} from "lucide-react";
import { cn } from "@/shared/utils/cn";
import { truncate } from "@/shared/utils/string";
import { useResizablePanel } from "@/shared/hooks/useResizablePanel";
import { useIsCompact } from "@/shared/hooks/useMediaQuery";
import { ProjectChatPane } from "./ProjectChatPane";
import { ConfirmDialog } from "@/shared/components/ui/confirm-dialog";
import { Badge } from "@/shared/components/ui/badge";
import { Button } from "@/shared/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogTitle,
} from "@/shared/components/ui/dialog";
import { Textarea } from "@/shared/components/ui/textarea";
import { toast } from "@/shared/components/ui/toast";
import { useT } from "@/shared/i18n/useT";
import { FloatingOutline } from "@/shared/components/ui/floating-outline";
import {
  applyBulkLock,
  canBulkChangeLocks,
  buildPreviewVersions,
  buildSceneBlocks,
  buildSceneRegenerationRequest,
  isSceneDirty,
  sourceForSceneAction,
  type SceneBlock,
  type SceneRegenerationRequest,
  type SceneSource,
  type SceneVersionPreview,
} from "./scenePreview";
import type { ProjectItem, ChapterItem } from "./types";

const SOURCE_LABEL_KEY = {
  AI_GENERATED: "project.scene.source.ai",
  HUMAN_EDITED: "project.scene.source.human",
  RESTORE: "project.scene.source.restore",
} as const;

interface ChapterDetailViewProps {
  project: ProjectItem;
  chapter: ChapterItem;
  chapterIndex: number;
  creatorEntry?: boolean;
}

interface PreviewGeneration {
  requestId: number;
  sceneId: string;
  status: "generating" | "review";
  attempt: number;
  text: string;
  note: string | null;
}

interface RegenerateDialogState {
  sceneId: string;
  noteInput: string;
}

function AutoResizeTextarea({
  value,
  disabled,
  onChange,
  ariaLabel,
}: {
  value: string;
  disabled?: boolean;
  onChange: (val: string) => void;
  ariaLabel?: string;
}) {
  const textareaRef = useRef<HTMLTextAreaElement>(null);

  useEffect(() => {
    const textarea = textareaRef.current;
    if (!textarea) return;

    let lastWidth = 0;
    const resize = () => {
      if (textarea.clientWidth === lastWidth) return;
      lastWidth = textarea.clientWidth;
      textarea.style.height = "auto";
      textarea.style.height = `${textarea.scrollHeight}px`;
    };

    resize();
    const observer = new ResizeObserver(resize);
    observer.observe(textarea);
    return () => observer.disconnect();
  }, [value]);

  return (
    <textarea
      ref={textareaRef}
      value={value}
      disabled={disabled}
      aria-label={ariaLabel}
      onChange={(e) => onChange(e.target.value)}
      rows={1}
      className={cn(
        "w-full max-h-[70vh] resize-none overflow-y-auto bg-transparent text-sm leading-relaxed focus-ring",
        disabled
          ? "cursor-not-allowed text-fg-subtle select-none -subtle"
          : "text-fg ",
      )}
    />
  );
}

export function ChapterDetailView({
  project,
  chapter,
  chapterIndex,
  creatorEntry = false,
}: ChapterDetailViewProps) {
  const t = useT();
  const isCompact = useIsCompact();
  const [isEditing, setIsEditing] = useState(false);
  const [discardOpen, setDiscardOpen] = useState(false);
  const [isChatOpen, setIsChatOpen] = useState(false);
  const headerRef = useRef<HTMLDivElement>(null);
  const [compactHeader, setCompactHeader] = useState(false);
  const [stackHeader, setStackHeader] = useState(false);
  const [blocks, setBlocks] = useState<SceneBlock[]>(() => buildSceneBlocks(chapter));
  const [savedBlocks] = useState<SceneBlock[]>(() => buildSceneBlocks(chapter));
  const [sources, setSources] = useState<Record<string, SceneSource>>({});
  const [generation, setGeneration] = useState<PreviewGeneration | null>(null);
  const [regenerateDialog, setRegenerateDialog] =
    useState<RegenerateDialogState | null>(null);
  const [versionSceneId, setVersionSceneId] = useState<string | null>(null);
  const generationTimer = useRef<number | null>(null);
  const generationRequestId = useRef(0);
  const regenerateSubmitting = useRef(false);

  const scrollContainerRef = useRef<HTMLDivElement>(null);
  const { leftRatio, isDragging, containerRef, startDragging } = useResizablePanel({
    initialRatio: 45,
    minPx: 340,
  });

  useEffect(() => {
    const header = headerRef.current;
    if (!header) return;

    const update = () => {
      const width = header.clientWidth;
      setCompactHeader(width > 0 && width < 510);
      setStackHeader(width > 0 && width < 420);
    };

    update();
    const observer = new ResizeObserver(update);
    observer.observe(header);
    return () => observer.disconnect();
  }, []);

  const savedByScene = useMemo(
    () => new Map(savedBlocks.map((block) => [block.sceneId, block])),
    [savedBlocks],
  );

  const dirtyByScene = useMemo(() => {
    const dirty = new Map<string, boolean>();
    for (const block of blocks) {
      const saved = savedByScene.get(block.sceneId);
      dirty.set(block.sceneId, isSceneDirty(block, saved));
    }
    return dirty;
  }, [blocks, savedByScene]);

  const dirtyCount = useMemo(
    () => [...dirtyByScene.values()].filter(Boolean).length,
    [dirtyByScene],
  );

  const hasUnsavedWork = dirtyCount > 0 || generation !== null;

  const sceneIndexById = useMemo(
    () => new Map(blocks.map((block, index) => [block.sceneId, index])),
    [blocks],
  );

  const blocker = useBlocker(
    ({ currentLocation, nextLocation }) =>
      hasUnsavedWork && currentLocation.pathname !== nextLocation.pathname,
  );

  useEffect(() => {
    if (!hasUnsavedWork) return;

    const handleBeforeUnload = (event: BeforeUnloadEvent) => {
      event.preventDefault();
      event.returnValue = "";
    };

    window.addEventListener("beforeunload", handleBeforeUnload);
    return () => window.removeEventListener("beforeunload", handleBeforeUnload);
  }, [hasUnsavedWork]);

  useEffect(
    () => () => {
      generationRequestId.current += 1;
      if (generationTimer.current !== null) {
        window.clearTimeout(generationTimer.current);
      }
    },
    [],
  );

  const clearGenerationTimer = () => {
    generationRequestId.current += 1;
    if (generationTimer.current !== null) {
      window.clearTimeout(generationTimer.current);
      generationTimer.current = null;
    }
  };

  const handleLockAll = () => {
    if (generation) return;
    setRegenerateDialog(null);
    setBlocks((prev) => prev.map((b) => ({ ...b, locked: true })));
  };

  const handleUnlockAll = () => {
    if (!canBulkChangeLocks(generation)) return;
    setBlocks((prev) => applyBulkLock(prev, false, generation));
  };

  const handleStartEdit = () => {
    setIsEditing(true);
  };

  const handleSave = () => {
    if (dirtyCount === 0) {
      toast({ title: t("project.save.noChanges") });
      return;
    }

    toast({
      title: t("project.save.previewTitle"),
      description: t("project.save.previewDescription"),
    });
  };

  const handleCancel = () => {
    if (hasUnsavedWork) {
      setDiscardOpen(true);
      return;
    }

    clearGenerationTimer();
    setGeneration(null);
    setRegenerateDialog(null);
    setVersionSceneId(null);
    setIsEditing(false);
  };

  const handleCancelConfirm = () => {
    setBlocks(savedBlocks.map((block) => ({ ...block })));
    setSources({});
    clearGenerationTimer();
    setGeneration(null);
    setRegenerateDialog(null);
    setVersionSceneId(null);
    setIsEditing(false);
    setDiscardOpen(false);
  };

  const toggleLock = (sceneId: string) => {
    if (generation?.sceneId === sceneId) return;

    setBlocks((prev) =>
      prev.map((b) => (b.sceneId === sceneId ? { ...b, locked: !b.locked } : b)),
    );
  };

  const updateBlockText = (sceneId: string, newText: string) => {
    setBlocks((prev) =>
      prev.map((b) => (b.sceneId === sceneId ? { ...b, text: newText } : b)),
    );
    setSources((prev) => ({
      ...prev,
      [sceneId]: sourceForSceneAction("edit"),
    }));
  };

  const startPreviewGeneration = (
    block: SceneBlock,
    attemptBase: number,
    note: string | null = null,
  ) => {
    if (block.locked) return;
    if (generation && generation.sceneId !== block.sceneId) return;
    if (generation?.status === "generating") return;

    clearGenerationTimer();
    const requestId = ++generationRequestId.current;
    const attempt = attemptBase + 1;
    const sceneNumber = (sceneIndexById.get(block.sceneId) ?? 0) + 1;

    setGeneration({
      requestId,
      sceneId: block.sceneId,
      status: "generating",
      attempt,
      text: "",
      note,
    });

    generationTimer.current = window.setTimeout(() => {
      setGeneration((prev) =>
        prev && prev.requestId === requestId
          ? {
              ...prev,
              status: "review",
              text: t("project.scene.previewSample", {
                index: sceneNumber,
                attempt,
              }),
            }
          : prev,
      );
      generationTimer.current = null;
    }, 900);
  };

  const openRegenerateDialog = (block: SceneBlock) => {
    if (block.locked || generation) return;
    regenerateSubmitting.current = false;
    setRegenerateDialog({ sceneId: block.sceneId, noteInput: "" });
  };

  const closeRegenerateDialog = () => {
    setRegenerateDialog(null);
  };

  const confirmRegeneration = () => {
    if (!regenerateDialog || generation || regenerateSubmitting.current) return;
    regenerateSubmitting.current = true;

    const block = blocks.find((item) => item.sceneId === regenerateDialog.sceneId);
    if (!block || block.locked) {
      setRegenerateDialog(null);
      return;
    }

    const request: SceneRegenerationRequest = buildSceneRegenerationRequest(
      block.sceneId,
      regenerateDialog.noteInput,
    );

    setRegenerateDialog(null);
    startPreviewGeneration(block, 0, request.note);
  };

  const cancelGeneration = () => {
    clearGenerationTimer();
    setGeneration(null);
  };

  const acceptGeneration = () => {
    if (!generation || generation.status !== "review") return;
    const target = blocks.find((block) => block.sceneId === generation.sceneId);
    if (!target || target.locked) return;

    generationRequestId.current += 1;
    setBlocks((prev) =>
      prev.map((b) =>
        b.sceneId === generation.sceneId ? { ...b, text: generation.text } : b,
      ),
    );
    setSources((prev) => ({
      ...prev,
      [generation.sceneId]: sourceForSceneAction("accept"),
    }));
    setGeneration(null);
  };

  const discardGeneration = () => {
    clearGenerationTimer();
    setGeneration(null);
  };

  const restoreVersion = (version: SceneVersionPreview) => {
    if (!versionSceneId) return;

    setBlocks((prev) =>
      prev.map((b) =>
        b.sceneId === versionSceneId ? { ...b, text: version.content } : b,
      ),
    );
    setSources((prev) => ({
      ...prev,
      [versionSceneId]: sourceForSceneAction("restore"),
    }));
    setVersionSceneId(null);
    toast({
      title: t("project.scene.versionRestored"),
      description: t("project.scene.versionRestoredHint"),
    });
  };

  const fullChapterTitle = t("chapter.title", { index: chapterIndex, title: chapter.title });

  const versionBlock = versionSceneId
    ? blocks.find((block) => block.sceneId === versionSceneId)
    : undefined;
  const versionIndex = versionBlock
    ? (sceneIndexById.get(versionBlock.sceneId) ?? 0) + 1
    : 0;
  const versions = versionBlock
    ? buildPreviewVersions(
        versionBlock,
        t("project.scene.versionSample", { index: versionIndex, version: 1 }),
      )
    : [];

  const scrollToBlock = (id: string) => {
    const el = document.getElementById(id);
    if (el) el.scrollIntoView({ behavior: 'smooth', block: 'start' });
  };

  const outlineItems = useMemo(
    () =>
      blocks.map((b, idx) => ({
        id: b.id,
        title: isEditing
          ? t("project.scene.label", { index: idx + 1 })
          : b.text || t("project.chapter.blockTitle", { index: idx + 1 }),
      })),
    [blocks, isEditing, t]
  );

  return (
    <div
      ref={containerRef}
      className={cn(
        "relative flex h-full w-full overflow-hidden",
        isDragging && "cursor-col-resize select-none",
      )}
    >
      {isChatOpen && (
        <div
          style={isCompact ? undefined : { width: `${leftRatio}%` }}
          className={cn(
            "h-full overflow-hidden",
            isCompact && "absolute inset-0 z-40 w-full bg-surface-panel",
          )}
        >
          <ProjectChatPane
            projectName={project.title}
            subContext={fullChapterTitle}
            onClose={() => setIsChatOpen(false)}
          />
        </div>
      )}

      {isChatOpen && !isCompact && (
        <div
          onMouseDown={startDragging}
          className="group relative flex w-2 shrink-0 cursor-col-resize items-center justify-center bg-transparent transition-colors hover:bg-blue-500/10 dark:hover:bg-blue-500/20"
        >
          <div className="h-full w-px bg-surface-muted transition-colors group-hover:bg-blue-500 " />
          <div
            className={cn(
              "absolute z-30 flex h-8 w-4.5 items-center justify-center rounded-md border border-line-strong bg-surface-card  transition-opacity  ",
              isDragging ? "opacity-100 ring-2 ring-blue-500" : "opacity-0 group-hover:opacity-100",
            )}
          >
            <GripVertical className="h-3.5 w-3.5 text-fg-muted" />
          </div>
        </div>
      )}

      <div
        ref={scrollContainerRef}
        style={{
          width: isChatOpen && !isCompact ? `${100 - leftRatio}%` : "100%",
        }}
        className="relative flex h-full flex-col overflow-y-auto bg-transparent p-6 dark:bg-transparent sm:p-8"
      >
        <div
          ref={headerRef}
          className={cn(
            "flex shrink-0 items-center justify-between gap-y-2 border-b border-line pb-4",
            stackHeader ? "min-h-12 flex-wrap" : "h-12",
          )}
        >
          <div
            className={cn(
              "flex min-w-0 items-center gap-2 truncate text-xs font-semibold text-fg-muted",
              stackHeader ? "w-full" : "flex-1",
            )}
          >
            <Link to="/project" className="shrink-0 hover:text-fg ">
              {t("project.title")}
            </Link>
            <span>›</span>
            <Link
              to={`/project/${project.id}`}
              className="truncate hover:text-fg "
              title={project.title}
            >
              {truncate(project.title, 15)}
            </Link>
            <span>›</span>
            <span
              className="truncate font-bold text-fg"
              title={fullChapterTitle}
            >
              {truncate(fullChapterTitle, 15)}
            </span>
          </div>

          <div
            className={cn(
              "flex shrink-0 items-center justify-end gap-2",
              stackHeader && "ml-auto",
              compactHeader && "flex-wrap",
            )}
          >
            {isEditing && (
              <div className="mr-2 flex items-center gap-1 border-r border-line pr-3 ">
                <button
                  type="button"
                  onClick={handleLockAll}
                  disabled={!!generation}
                  aria-label={t("project.lockAll")}
                  className="flex items-center gap-1.5 rounded-lg px-2 py-1.5 text-xs font-medium text-rose-600 transition-colors duration-100 hover:bg-rose-50 active:scale-95 disabled:pointer-events-none disabled:opacity-50 dark:text-rose-400 dark:hover:bg-rose-950/50"
                  title={t("project.lockAll")}
                >
                  <Lock className="h-3.5 w-3.5" />
                  {!compactHeader && (
                    <span className="hidden sm:inline">{t("project.lockAll")}</span>
                  )}
                </button>
                <button
                  type="button"
                  onClick={handleUnlockAll}
                  disabled={!canBulkChangeLocks(generation)}
                  aria-label={t("project.unlockAll")}
                  className="flex items-center gap-1.5 rounded-lg px-2 py-1.5 text-xs font-medium text-emerald-600 transition-colors duration-100 hover:bg-emerald-50 active:scale-95 disabled:pointer-events-none disabled:opacity-50 dark:text-emerald-400 dark:hover:bg-emerald-950/50"
                  title={t("project.unlockAll")}
                >
                  <Unlock className="h-3.5 w-3.5" />
                  {!compactHeader && (
                    <span className="hidden sm:inline">{t("project.unlockAll")}</span>
                  )}
                </button>
              </div>
            )}

            {!isEditing ? (
              <button
                type="button"
                onClick={handleStartEdit}
                className="flex items-center gap-1.5 tier-card rounded-card px-3 py-1.5 text-xs font-medium text-fg-muted  transition-colors hover:bg-surface-muted    "
              >
                <Pencil className="h-3.5 w-3.5" />
                <span>{t("common.edit")}</span>
              </button>
            ) : (
              <>
                <button
                  type="button"
                  onClick={handleCancel}
                  className="flex items-center gap-1.5 tier-card rounded-card px-3 py-1.5 text-xs font-medium text-fg-muted  transition-colors hover:bg-surface-muted    "
                >
                  <X className="h-3.5 w-3.5" />
                  <span>{t("common.cancel")}</span>
                </button>
                <button
                  type="button"
                  onClick={handleSave}
                  title={
                    dirtyCount > 0
                      ? t("project.scene.unsaved")
                      : t("project.save.noChanges")
                  }
                  className="relative flex items-center gap-1.5 rounded-xl bg-accent px-3 py-1.5 text-xs font-semibold text-white  transition-colors hover:opacity-90   "
                >
                  <Check className="h-3.5 w-3.5" />
                  <span>{t("common.save")}</span>
                  {dirtyCount > 0 && (
                    <span
                      className="absolute right-1.5 top-1.5 h-1.5 w-1.5 rounded-full bg-amber-300"
                      aria-hidden="true"
                    />
                  )}
                </button>
              </>
            )}

            <button
              type="button"
              onClick={() => setIsChatOpen(!isChatOpen)}
              className={cn(
                "focus-ring flex items-center gap-1.5 rounded-xl border px-3 py-1.5 text-xs font-medium  transition-colors",
                isChatOpen
                  ? "border-blue-500 bg-blue-50 text-blue-700 dark:border-blue-600 dark:bg-blue-950/60 dark:text-blue-300"
                  : "border-line bg-surface-card text-fg-muted hover:bg-surface-muted   ",
              )}
            >
              <MessageSquare className="h-3.5 w-3.5" />
              <span>{t("project.chat.label")}</span>
            </button>
          </div>
        </div>

        <div className="mx-auto mt-6 w-full max-w-2xl space-y-5 pr-10">
          <div className="border-b border-line pb-4 ">
            <h1 className="text-xl font-bold tracking-tight text-fg sm:text-2xl">
              {fullChapterTitle}
              {creatorEntry && (
                <Badge variant="accent" className="ml-2 align-middle">
                  {t("project.mode.creator")}
                </Badge>
              )}
            </h1>
            <p className="mt-1 text-xs text-fg-subtle">{t("project.chapter.postedAt", { date: chapter.date })}</p>
          </div>

          <div className="space-y-4">
            {blocks.map((block, blockIndex) => {
              const sceneNumber = blockIndex + 1;
              const dirty = dirtyByScene.get(block.sceneId) ?? false;
              const activeGeneration =
                generation?.sceneId === block.sceneId ? generation : null;
              const source = sources[block.sceneId];
              const hasContent = block.text.trim().length > 0;

              return (
                <div
                  key={block.sceneId}
                  id={block.id}
                  className={cn(
                    "group relative rounded-2xl transition-colors duration-200 scroll-mt-24",
                    isEditing
                      ? "border border-line bg-surface-card p-4   "
                      : "text-sm leading-relaxed text-fg-muted ",
                  )}
                >
                  {isEditing ? (
                    <>
                      <div
                        className={cn(
                          "absolute -right-11 top-1/2 z-10 -translate-y-1/2 transition-colors duration-200",
                          block.locked ? "opacity-100" : "opacity-0 group-hover:opacity-100",
                        )}
                      >
                        <button
                          type="button"
                          onClick={() => toggleLock(block.sceneId)}
                          disabled={!!activeGeneration}
                          aria-label={
                            block.locked
                              ? t("project.scene.unlock")
                              : t("project.scene.lock")
                          }
                          title={
                            block.locked
                              ? t("project.scene.unlock")
                              : t("project.scene.lock")
                          }
                          className={cn(
                            "focus-ring flex h-7 w-7 items-center justify-center rounded-lg border  transition-colors disabled:pointer-events-none disabled:opacity-50",
                            block.locked
                              ? "border-rose-300 bg-rose-50 text-rose-600 dark:border-rose-800 dark:bg-rose-950 dark:text-rose-300"
                              : "border-line bg-surface-card text-fg-subtle hover:border-line-strong hover:text-fg   -subtle ",
                          )}
                        >
                          {block.locked ? (
                            <Lock className="h-3.5 w-3.5" />
                          ) : (
                            <Unlock className="h-3.5 w-3.5" />
                          )}
                        </button>
                      </div>

                      <div className="mb-2 flex flex-wrap items-center justify-between gap-2">
                        <span className="text-xs font-semibold uppercase tracking-wider text-fg-subtle">
                          {t("project.scene.label", { index: sceneNumber })}
                        </span>
                        <span className="flex items-center gap-1.5">
                          {source && (
                            <Badge
                              variant={
                                source === "AI_GENERATED"
                                  ? "accent"
                                  : source === "RESTORE"
                                    ? "warning"
                                    : "neutral"
                              }
                            >
                              {t(SOURCE_LABEL_KEY[source])}
                            </Badge>
                          )}
                          <Badge variant={dirty ? "warning" : "neutral"}>
                            {dirty
                              ? t("project.scene.unsaved")
                              : t("project.scene.clean")}
                          </Badge>
                        </span>
                      </div>

                      <AutoResizeTextarea
                        value={block.text}
                        disabled={
                          block.locked || activeGeneration?.status === "generating"
                        }
                        ariaLabel={t("project.scene.label", { index: sceneNumber })}
                        onChange={(val) => updateBlockText(block.sceneId, val)}
                      />

                      {block.locked && (
                        <span className="mt-1.5 inline-flex items-center gap-1 text-xs font-medium text-rose-500">
                          <Lock className="h-3 w-3" />
                          <span>{t("project.chapter.locked")}</span>
                        </span>
                      )}

                      {activeGeneration?.status === "generating" && (
                        <div
                          role="status"
                          className="mt-3 flex items-center gap-2 rounded-xl border border-line bg-surface-muted/60 px-3 py-2 text-xs text-fg-muted"
                        >
                          <Loader2 className="h-3.5 w-3.5 animate-spin" aria-hidden="true" />
                          <span>{t("project.scene.generating")}</span>
                        </div>
                      )}

                      {activeGeneration?.status === "review" && (
                        <div className="mt-3 rounded-xl border border-amber-300/70 bg-amber-50/60 p-3 dark:border-amber-800 dark:bg-amber-950/20">
                          <div className="flex flex-wrap items-center gap-2">
                            <Badge variant="warning">
                              {t("project.scene.previewBadge")}
                            </Badge>
                            <p className="text-xs text-fg-muted">
                              {t("project.scene.previewNote")}
                            </p>
                          </div>
                          {activeGeneration.note && (
                            <div className="mt-2 max-h-24 overflow-y-auto rounded-lg border border-line bg-surface-card/70 px-2.5 py-2 text-xs text-fg-muted">
                              <span className="font-semibold text-fg">
                                {t("project.scene.regenerateNotePreview")}
                              </span>{" "}
                              <span className="whitespace-pre-wrap break-words">
                                {activeGeneration.note}
                              </span>
                            </div>
                          )}
                          <p className="mt-2 max-h-[50vh] overflow-y-auto whitespace-pre-wrap pr-1 text-sm leading-relaxed text-fg">
                            {activeGeneration.text}
                          </p>
                          <div className="mt-3 flex flex-wrap items-center gap-2">
                            <Button
                              size="sm"
                              variant="primary"
                              disabled={block.locked}
                              onClick={acceptGeneration}
                            >
                              {t("project.scene.accept")}
                            </Button>
                            <Button size="sm" variant="ghost" onClick={discardGeneration}>
                              {t("project.scene.discard")}
                            </Button>
                          </div>
                        </div>
                      )}

                      <div className="mt-3 flex flex-wrap items-center gap-2 border-t border-line pt-3">
                        <Button
                          size="sm"
                          variant="secondary"
                          disabled={block.locked || hasContent || !!generation}
                          onClick={() => startPreviewGeneration(block, 0)}
                        >
                          <Sparkles className="h-3.5 w-3.5" aria-hidden="true" />
                          <span>{t("project.scene.generate")}</span>
                        </Button>
                        <Button
                          size="sm"
                          variant="secondary"
                          disabled={block.locked || !hasContent || !!generation}
                          onClick={() => openRegenerateDialog(block)}
                        >
                          <RotateCcw className="h-3.5 w-3.5" aria-hidden="true" />
                          <span>{t("project.scene.regenerate")}</span>
                        </Button>
                        {activeGeneration?.status === "generating" && (
                          <Button size="sm" variant="ghost" onClick={cancelGeneration}>
                            <X className="h-3.5 w-3.5" aria-hidden="true" />
                            <span>{t("common.cancel")}</span>
                          </Button>
                        )}
                        <Button
                          size="sm"
                          variant="ghost"
                          disabled={!hasContent || block.locked || !!generation}
                          onClick={() => setVersionSceneId(block.sceneId)}
                        >
                          <History className="h-3.5 w-3.5" aria-hidden="true" />
                          <span>{t("project.scene.versions")}</span>
                        </Button>
                      </div>
                    </>
                  ) : (
                    <p className="whitespace-pre-wrap text-base leading-relaxed">{block.text}</p>
                  )}
                </div>
              );
            })}
          </div>
        </div>
      </div>

      {/* Chapter outline minimap */}
      {blocks.length > 0 && (
        <FloatingOutline 
          items={outlineItems} 
          containerRef={scrollContainerRef}
          onItemClick={scrollToBlock} 
        />
      )}

      <ConfirmDialog
        open={discardOpen}
        onOpenChange={setDiscardOpen}
        title={t("discard.title")}
        description={t("discard.description")}
        confirmLabel={t("discard.confirm")}
        cancelLabel={t("discard.cancel")}
        variant="danger"
        onConfirm={handleCancelConfirm}
      />

      <ConfirmDialog
        open={blocker.state === "blocked"}
        onOpenChange={(open) => {
          if (!open) blocker.reset?.();
        }}
        title={t("project.leave.title")}
        description={t("project.leave.description")}
        confirmLabel={t("project.leave.confirm")}
        cancelLabel={t("project.leave.cancel")}
        variant="danger"
        onConfirm={() => blocker.proceed?.()}
      />

      <Dialog
        open={!!regenerateDialog}
        onOpenChange={(open) => {
          if (!open) closeRegenerateDialog();
        }}
      >
        <DialogContent className="max-w-lg">
          <div>
            <DialogTitle className="text-base">
              {t("project.scene.regenerateTitle")}
            </DialogTitle>
            <DialogDescription className="mt-1 text-xs">
              {t("project.scene.regenerateDescription")}
            </DialogDescription>
          </div>

          <Textarea
            label={t("project.scene.regenerateNoteLabel")}
            hint={t("project.scene.regenerateMaxHint")}
            value={regenerateDialog?.noteInput ?? ""}
            onChange={(event) =>
              setRegenerateDialog((prev) =>
                prev ? { ...prev, noteInput: event.target.value } : prev,
              )
            }
            placeholder={t("project.scene.regeneratePlaceholder")}
            maxLength={2000}
            rows={4}
          />

          <div className="mt-4 flex justify-end gap-2">
            <Button variant="ghost" size="sm" onClick={closeRegenerateDialog}>
              {t("common.cancel")}
            </Button>
            <Button
              variant="primary"
              size="sm"
              disabled={!!generation}
              onClick={confirmRegeneration}
            >
              {t("project.scene.regenerateConfirm")}
            </Button>
          </div>
        </DialogContent>
      </Dialog>

      {versionBlock && (
        <Dialog
          open={!!versionSceneId}
          onOpenChange={(open) => {
            if (!open) setVersionSceneId(null);
          }}
        >
          <DialogContent className="max-w-lg">
            <div>
              <DialogTitle className="text-base">
                {t("project.scene.versionTitle", { index: versionIndex })}
              </DialogTitle>
              <DialogDescription className="mt-1 text-xs">
                {t("project.scene.versionPreview")}
              </DialogDescription>
            </div>

            <div className="mt-2 max-h-[55vh] space-y-2 overflow-y-auto pr-1">
              {versions.length === 0 ? (
                <p className="text-xs text-fg-muted">
                  {t("project.scene.versionEmpty")}
                </p>
              ) : (
                versions.map((version) => (
                  <div
                    key={version.versionNumber}
                    className="rounded-xl border border-line bg-surface-muted/40 p-3"
                  >
                    <div className="flex flex-wrap items-center justify-between gap-2">
                      <div className="flex flex-wrap items-center gap-2">
                        <span className="font-mono text-xs font-semibold text-fg">
                          v.{version.versionNumber}
                        </span>
                        <Badge
                          variant={
                            version.source === "AI_GENERATED"
                              ? "accent"
                              : version.source === "RESTORE"
                                ? "warning"
                                : "neutral"
                          }
                        >
                          {t(SOURCE_LABEL_KEY[version.source])}
                        </Badge>
                        <span className="text-xs text-fg-subtle">
                          {version.createdAt}
                        </span>
                      </div>
                      <Button
                        size="sm"
                        variant="secondary"
                        onClick={() => restoreVersion(version)}
                      >
                        {t("project.scene.versionRestore")}
                      </Button>
                    </div>
                    <p className="mt-2 line-clamp-3 whitespace-pre-wrap text-xs text-fg-muted">
                      {version.content}
                    </p>
                  </div>
                ))
              )}
            </div>
          </DialogContent>
        </Dialog>
      )}
    </div>
  );
}
