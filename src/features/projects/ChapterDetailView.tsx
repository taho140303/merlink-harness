import { useState, useRef, useEffect, useMemo, useCallback } from "react";
import { Link, useBlocker, useParams } from "react-router";
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
import { backendClient, type BackendClient } from "@/shared/backend/client";
import {
  applyBulkLock,
  canBulkChangeLocks,
  buildPreviewVersions,
  buildSceneBlocks,
  buildSceneBlocksFromReadModel,
  buildSceneRegenerationRequest,
  isSceneDirty,
  isSceneContentDirty,
  sourceForSceneAction,
  loadChapterWritingReadModel,
  setSceneLock,
  setChapterScenesLockBulk,
  generateSceneDraft,
  cancelSceneGeneration,
  reviewGeneratedScene,
  listSceneVersions,
  executeSaveWithReceiptReconciliation,
  restoreSceneVersion,
  type SceneBlock,
  type SceneRegenerationRequest,
  type SceneSource,
  type SceneGenerationCandidate,
} from "./scenePreview";
import type { ProjectItem, ChapterItem } from "./types";
import type { SceneVersionItemDto } from "@/shared/backend/generated";

const SOURCE_LABEL_KEY = {
  AI_GENERATED: "project.scene.source.ai",
  HUMAN_EDITED: "project.scene.source.human",
  RESTORE: "project.scene.source.restore",
} as const;

export interface ChapterDetailViewProps {
  project?: ProjectItem;
  chapter?: ChapterItem;
  chapterIndex?: number;
  creatorEntry?: boolean;
  projectId?: string;
  chapterId?: string;
  client?: BackendClient;
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
  chapterIndex = 1,
  creatorEntry = false,
  projectId: propProjectId,
  chapterId: propChapterId,
  client: propClient,
}: ChapterDetailViewProps) {
  const t = useT();
  const isCompact = useIsCompact();
  const routeParams = useParams();
  const activeClient = propClient || backendClient;

  const targetProjectId = propProjectId || routeParams.projectId || project?.id;
  const targetChapterId = propChapterId || routeParams.chapterId || chapter?.id;

  const [backendGateError, setBackendGateError] = useState<
    "UNCOMMITTED_PLAN" | "NOT_FOUND" | "BACKEND_ERROR" | null
  >(null);
  const [readModelTitle, setReadModelTitle] = useState<string | null>(null);
  const [readModelIndex, setReadModelIndex] = useState<number | null>(null);

  const [isEditing, setIsEditing] = useState(false);
  const [discardOpen, setDiscardOpen] = useState(false);
  const [isChatOpen, setIsChatOpen] = useState(false);
  const headerRef = useRef<HTMLDivElement>(null);
  const [compactHeader, setCompactHeader] = useState(false);
  const [stackHeader, setStackHeader] = useState(false);

  const initialBlocks = useMemo(() => {
    if (chapter) return buildSceneBlocks(chapter);
    return [];
  }, [chapter]);

  const [blocks, setBlocks] = useState<SceneBlock[]>(initialBlocks);
  const [savedBlocks, setSavedBlocks] = useState<SceneBlock[]>(initialBlocks);
  const [sources, setSources] = useState<Record<string, SceneSource>>({});

  const [generation, setGeneration] = useState<SceneGenerationCandidate | null>(null);
  const [regenerateDialog, setRegenerateDialog] =
    useState<RegenerateDialogState | null>(null);
  const [versionSceneId, setVersionSceneId] = useState<string | null>(null);
  const [backendVersions, setBackendVersions] = useState<SceneVersionItemDto[] | null>(null);

  const generationTimer = useRef<number | null>(null);
  const generationRequestId = useRef(0);
  const regenerateSubmitting = useRef(false);
  const currentSaveIdRef = useRef<string | null>(null);

  const scrollContainerRef = useRef<HTMLDivElement>(null);
  const { leftRatio, isDragging, containerRef, startDragging } = useResizablePanel({
    initialRatio: 45,
    minPx: 340,
  });

  // Load Chapter Writing Read Model via BackendClient (#29 get_chapter_writing_state)
  const loadWritingReadModel = useCallback(async () => {
    if (
      !activeClient ||
      activeClient.status === "unavailable" ||
      !targetProjectId ||
      !targetChapterId
    ) {
      return;
    }

    try {
      const readModel = await loadChapterWritingReadModel(activeClient, {
        projectId: targetProjectId,
        chapterId: targetChapterId,
      });

      setBackendGateError(null);
      setReadModelTitle(readModel.chapterTitle);
      if (typeof readModel.position === "number") {
        setReadModelIndex(readModel.position);
      }

      const sceneBlocks = buildSceneBlocksFromReadModel(readModel);
      setBlocks(sceneBlocks);
      setSavedBlocks(sceneBlocks.map((b) => ({ ...b })));

      const initialSources: Record<string, SceneSource> = {};
      for (const s of sceneBlocks) {
        if (s.currentSceneSource === "AI") {
          initialSources[s.sceneId] = "AI_GENERATED";
        } else if (s.currentSceneSource === "RESTORE") {
          initialSources[s.sceneId] = "RESTORE";
        } else if (s.currentSceneSource === "USER") {
          initialSources[s.sceneId] = "HUMAN_EDITED";
        }
      }
      setSources(initialSources);
    } catch (err: any) {
      const errMsg = String(err?.message || err);
      if (
        errMsg.includes("EntityNotFound") ||
        errMsg.includes("not found") ||
        errMsg.includes("NotFound")
      ) {
        setBackendGateError("NOT_FOUND");
      } else if (
        errMsg.includes("ValidationFailed") ||
        errMsg.includes("Committed") ||
        errMsg.includes("committed") ||
        errMsg.includes("plan")
      ) {
        // Committed-Plan Gate (WR-01)
        setBackendGateError("UNCOMMITTED_PLAN");
      } else {
        setBackendGateError("BACKEND_ERROR");
      }
    }
  }, [activeClient, targetProjectId, targetChapterId]);

  useEffect(() => {
    loadWritingReadModel();
  }, [loadWritingReadModel]);

  // Header responsive measurements
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

  // Content-dirty check: Lock is NOT content-dirty (REQ-FE-005)
  const dirtyByScene = useMemo(() => {
    const dirty = new Map<string, boolean>();
    for (const block of blocks) {
      const saved = savedByScene.get(block.sceneId);
      dirty.set(block.sceneId, isSceneContentDirty(block, saved));
    }
    return dirty;
  }, [blocks, savedByScene]);

  const dirtyCount = useMemo(
    () => [...dirtyByScene.values()].filter(Boolean).length,
    [dirtyByScene],
  );

  const hasUnsavedWork = dirtyCount > 0 || (generation !== null && generation.status !== "failed");

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

  // Lock All (#31 set_chapter_scenes_lock_bulk)
  const handleLockAll = async () => {
    if (generation && generation.status === "streaming") return;
    setRegenerateDialog(null);

    if (
      activeClient &&
      activeClient.status !== "unavailable" &&
      targetProjectId &&
      targetChapterId
    ) {
      try {
        await setChapterScenesLockBulk(activeClient, {
          projectId: targetProjectId,
          chapterId: targetChapterId,
          isLocked: true,
        });

        // Lock change is immediate and not content dirty; update both blocks and savedBlocks lock state
        setBlocks((prev) => prev.map((b) => ({ ...b, locked: true })));
        setSavedBlocks((prev) => prev.map((b) => ({ ...b, locked: true })));

        // Invalidate generation candidate if present (P5S5-N)
        if (generation) {
          setGeneration((prev) =>
            prev ? { ...prev, authorizationValid: false, status: "invalidated_by_lock" } : null,
          );
        }
      } catch (err) {
        console.error("Lock all failed:", err);
        toast({ title: "Không thể khóa tất cả các cảnh." });
      }
      return;
    }

    setBlocks((prev) => prev.map((b) => ({ ...b, locked: true })));
    setSavedBlocks((prev) => prev.map((b) => ({ ...b, locked: true })));
  };

  // Unlock All (#31 set_chapter_scenes_lock_bulk)
  const handleUnlockAll = async () => {
    if (!canBulkChangeLocks(generation)) return;

    if (
      activeClient &&
      activeClient.status !== "unavailable" &&
      targetProjectId &&
      targetChapterId
    ) {
      try {
        await setChapterScenesLockBulk(activeClient, {
          projectId: targetProjectId,
          chapterId: targetChapterId,
          isLocked: false,
        });

        setBlocks((prev) => prev.map((b) => ({ ...b, locked: false })));
        setSavedBlocks((prev) => prev.map((b) => ({ ...b, locked: false })));
      } catch (err) {
        console.error("Unlock all failed:", err);
        toast({ title: "Không thể mở khóa các cảnh." });
      }
      return;
    }

    setBlocks((prev) => applyBulkLock(prev, false, generation));
    setSavedBlocks((prev) => applyBulkLock(prev, false, generation));
  };

  const handleStartEdit = () => {
    setIsEditing(true);
  };

  // Save Chapter scenes (#36 save_chapter, with OCC and #37 get_chapter_save_receipt reconciliation)
  const handleSave = async () => {
    if (dirtyCount === 0) {
      toast({ title: t("project.save.noChanges") });
      return;
    }

    // Identify dirty scenes
    const dirtyScenesToSave = blocks.filter((b) =>
      isSceneContentDirty(b, savedByScene.get(b.sceneId)),
    );

    // LOCK-05 / P5S5-B: Reject save if any dirty scene is locked; locked clean scenes do NOT block saving!
    const lockedDirtyScene = dirtyScenesToSave.find((b) => b.locked);
    if (lockedDirtyScene) {
      toast({
        title: "Không thể lưu cảnh bị khóa",
        description: "Có cảnh bị khóa đang chứa thay đổi chưa lưu. Vui lòng mở khóa cảnh trước khi lưu chương.",
      });
      return;
    }

    if (
      activeClient &&
      activeClient.status !== "unavailable" &&
      targetProjectId &&
      targetChapterId
    ) {
      // Retain or mint clientSaveId for idempotent retry/reconciliation (phase5_sprint5_deep3-F2)
      if (!currentSaveIdRef.current) {
        currentSaveIdRef.current =
          typeof crypto !== "undefined" && typeof crypto.randomUUID === "function"
            ? crypto.randomUUID()
            : `save-${Date.now()}-${Math.random().toString(36).substring(2, 9)}`;
      }
      const clientSaveId = currentSaveIdRef.current;

      try {
        const result = await executeSaveWithReceiptReconciliation(activeClient, {
          projectId: targetProjectId,
          chapterId: targetChapterId,
          clientSaveId,
          dirtyScenes: dirtyScenesToSave.map((b) => ({
            sceneId: b.sceneId,
            expectedVersionNumber: b.headVersionNumber ?? 0,
            content: b.text,
            sceneSource:
              sources[b.sceneId] === "AI_GENERATED"
                ? "AI"
                : sources[b.sceneId] === "RESTORE"
                  ? "RESTORE"
                  : "USER",
            summary: b.summary ?? null,
            isLocked: b.locked,
          })),
        });

        // Save successfully persisted / reconciled!
        currentSaveIdRef.current = null;

        if (result.response.chapterScenes && result.response.chapterScenes.length > 0) {
          const updatedBlocks = buildSceneBlocksFromReadModel({
            projectId: targetProjectId,
            chapterId: targetChapterId,
            chapterTitle: readModelTitle || "",
            position: readModelIndex ?? chapterIndex,
            scenes: result.response.chapterScenes,
          });
          setBlocks(updatedBlocks);
          setSavedBlocks(updatedBlocks.map((b) => ({ ...b })));
        } else {
          setSavedBlocks(blocks.map((b) => ({ ...b })));
        }

        toast({
          title: "Đã lưu chương thành công",
          description: result.reconciledFromReceipt
            ? "Đã đối soát biên nhận lưu thành công từ dịch vụ."
            : "Toàn bộ thay đổi nội dung đã được lưu bền vững.",
        });
      } catch (err: any) {
        console.error("Save chapter failed:", err);
        // phase5_sprint5_deep3-F2: Retain currentSaveIdRef.current and dirty buffer intact!
        toast({
          title: "Lỗi lưu chương",
          description: "Không thể lưu chương. Toàn bộ nội dung đang sửa được giữ nguyên để thử lại.",
        });
      }
      return;
    }

    // Local / preview fallback
    setSavedBlocks(blocks.map((b) => ({ ...b })));
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

  // Toggle single scene lock (#30 set_scene_lock)
  // Lock is NOT content-dirty. Lock failure keeps UI unlocked and dirty buffer intact (phase5_sprint5_deep3-F1).
  const toggleLock = async (sceneId: string) => {
    if (generation?.sceneId === sceneId && generation?.status === "streaming") return;

    const targetBlock = blocks.find((b) => b.sceneId === sceneId);
    if (!targetBlock) return;

    const desiredLocked = !targetBlock.locked;

    if (
      activeClient &&
      activeClient.status !== "unavailable" &&
      targetProjectId &&
      targetChapterId
    ) {
      try {
        await setSceneLock(activeClient, {
          projectId: targetProjectId,
          chapterId: targetChapterId,
          sceneId,
          desiredLocked,
        });

        // Persist lock in UI state & baseline without touching dirty content buffer
        setBlocks((prev) =>
          prev.map((b) => (b.sceneId === sceneId ? { ...b, locked: desiredLocked } : b)),
        );
        setSavedBlocks((prev) =>
          prev.map((b) => (b.sceneId === sceneId ? { ...b, locked: desiredLocked } : b)),
        );

        // LOCK-04 / P5S5-N: Locking invalidates any active or completed generation candidate
        if (desiredLocked && generation?.sceneId === sceneId) {
          setGeneration((prev) =>
            prev ? { ...prev, authorizationValid: false, status: "invalidated_by_lock" } : null,
          );
        }
      } catch (err) {
        // phase5_sprint5_deep3-F1: UI remains in previous lock state, dirty buffer intact!
        console.error("Lock toggle failed on backend:", err);
        toast({
          title: "Lỗi cập nhật khóa cảnh",
          description: "Không thể cập nhật khóa cảnh. Trạng thái và nội dung soạn thảo được giữ nguyên.",
        });
      }
      return;
    }

    setBlocks((prev) =>
      prev.map((b) => (b.sceneId === sceneId ? { ...b, locked: desiredLocked } : b)),
    );
    setSavedBlocks((prev) =>
      prev.map((b) => (b.sceneId === sceneId ? { ...b, locked: desiredLocked } : b)),
    );
    if (desiredLocked && generation?.sceneId === sceneId) {
      setGeneration((prev) =>
        prev ? { ...prev, authorizationValid: false, status: "invalidated_by_lock" } : null,
      );
    }
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

  // Start scene generation (#32 start_scene_generation)
  const startPreviewGeneration = async (
    block: SceneBlock,
    attemptBase: number,
    note: string | null = null,
  ) => {
    if (block.locked) return;
    if (generation && generation.sceneId !== block.sceneId) return;
    if (generation?.status === "streaming") return;

    clearGenerationTimer();
    const requestId = String(++generationRequestId.current);
    const attempt = attemptBase + 1;
    const sceneNumber = (sceneIndexById.get(block.sceneId) ?? 0) + 1;

    if (
      activeClient &&
      activeClient.status !== "unavailable" &&
      targetProjectId &&
      targetChapterId
    ) {
      setGeneration({
        requestId,
        generationId: "",
        sceneId: block.sceneId,
        baselineVersionNumber: block.headVersionNumber ?? 0,
        authorizationValid: true,
        status: "streaming",
        text: "",
        note,
      });

      try {
        const candidate = await generateSceneDraft(
          activeClient,
          {
            projectId: targetProjectId,
            chapterId: targetChapterId,
            sceneId: block.sceneId,
            mode: block.text.trim().length > 0 ? "regenerate" : "generate",
            note,
            currentWorkingContent: block.text,
          },
          (event) => {
            if (event.event === "started") {
              setGeneration((prev) =>
                prev
                  ? {
                      ...prev,
                      generationId: event.data.generationId,
                      baselineVersionNumber: event.data.baselineVersionNumber,
                      authorizationValid: event.data.authorizationValid,
                    }
                  : null,
              );
            } else if (event.event === "delta") {
              setGeneration((prev) =>
                prev ? { ...prev, text: prev.text + event.data.textDelta } : null,
              );
            } else if (event.event === "completed") {
              setGeneration((prev) =>
                prev
                  ? {
                      ...prev,
                      text: event.data.fullText,
                      status: "review", // Completed stream remains reviewable (P5S5-P)
                    }
                  : null,
              );
            } else if (
              event.event === "cancelled" ||
              event.event === "invalidatedByLock"
            ) {
              setGeneration((prev) =>
                prev
                  ? {
                      ...prev,
                      text: event.data.partialText,
                      status: "invalidated_by_lock",
                      authorizationValid: false,
                    }
                  : null,
              );
            }
          },
        );

        // P5S5-P: Valid completed stream candidate remains in Review state after stream completes
        setGeneration(candidate);
      } catch (err: any) {
        console.error("Scene generation error:", err);
        toast({
          title: "Lỗi sinh nội dung cảnh",
          description: String(err?.message || err),
        });
        setGeneration(null);
      }
      return;
    }

    // Local / simulated generation fallback
    setGeneration({
      requestId,
      generationId: `gen-local-${Date.now()}`,
      sceneId: block.sceneId,
      status: "streaming",
      baselineVersionNumber: block.headVersionNumber ?? 0,
      authorizationValid: true,
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

  const cancelGeneration = async () => {
    clearGenerationTimer();
    if (
      activeClient &&
      activeClient.status !== "unavailable" &&
      targetChapterId &&
      generation?.generationId
    ) {
      try {
        await cancelSceneGeneration(activeClient, {
          chapterId: targetChapterId,
          generationId: generation.generationId,
        });
      } catch (err) {
        console.warn("Cancel generation warning:", err);
      }
    }
    setGeneration(null);
  };

  // Accept generation (P5S5-P, P5S5-N)
  const acceptGeneration = () => {
    if (!generation || generation.status !== "review") return;
    const target = blocks.find((block) => block.sceneId === generation.sceneId);
    if (!target) return;

    try {
      const reviewResult = reviewGeneratedScene({
        action: "accept",
        candidate: generation,
        scene: target,
      });

      if (reviewResult.accepted && reviewResult.updatedText) {
        setBlocks((prev) =>
          prev.map((b) =>
            b.sceneId === generation.sceneId
              ? { ...b, text: reviewResult.updatedText! }
              : b,
          ),
        );
        setSources((prev) => ({
          ...prev,
          [generation.sceneId]: sourceForSceneAction("accept"),
        }));
        setGeneration(null);
      }
    } catch (err: any) {
      // P5S5-N: lock-unlock cannot accept invalidated generation (CONFLICT_INVALIDATED)
      toast({
        title: "Không thể chấp nhận bản sinh",
        description: "Bản sinh đã bị vô hiệu hóa do cảnh bị khóa hoặc nội dung đã thay đổi.",
      });
    }
  };

  const discardGeneration = () => {
    clearGenerationTimer();
    setGeneration(null);
  };

  // Open versions dialog (#35 list_scene_versions)
  const handleOpenVersions = async (sceneId: string) => {
    setVersionSceneId(sceneId);
    if (
      activeClient &&
      activeClient.status !== "unavailable" &&
      targetProjectId &&
      targetChapterId
    ) {
      try {
        const res = await listSceneVersions(activeClient, {
          projectId: targetProjectId,
          chapterId: targetChapterId,
          sceneId,
        });
        setBackendVersions(res.versions);
      } catch (err) {
        console.error("List scene versions error:", err);
        setBackendVersions(null);
      }
      return;
    }
    setBackendVersions(null);
  };

  // Restore scene version (CI-08 / REQ-WRITE-004)
  // Loads content into dirty buffer; persisted via save_chapter (#36). Zero 41st IPC command.
  const restoreVersion = (version: { versionNumber: number; content: string }) => {
    if (!versionSceneId) return;
    const target = blocks.find((b) => b.sceneId === versionSceneId);
    if (!target) return;

    try {
      const res = restoreSceneVersion(target, version);
      if (res.success && res.newText !== undefined) {
        setBlocks((prev) =>
          prev.map((b) =>
            b.sceneId === versionSceneId ? { ...b, text: res.newText! } : b,
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
      }
    } catch (err: any) {
      toast({
        title: "Không thể khôi phục phiên bản",
        description: "Cảnh đang bị khóa. Mở khóa cảnh trước khi khôi phục nội dung phiên bản lịch sử.",
      });
    }
  };

  const currentProjectId = project?.id || targetProjectId || "";
  const currentProjectTitle = project?.title || "Dự án";
  const currentChapterIndex = readModelIndex ?? chapterIndex;
  const currentChapterTitle = readModelTitle || chapter?.title || t("project.chapter.blockTitle", { index: currentChapterIndex });
  const fullChapterTitle = t("chapter.title", { index: currentChapterIndex, title: currentChapterTitle });

  const versionBlock = versionSceneId
    ? blocks.find((block) => block.sceneId === versionSceneId)
    : undefined;
  const versionIndex = versionBlock
    ? (sceneIndexById.get(versionBlock.sceneId) ?? 0) + 1
    : 0;

  const previewVersions = useMemo(() => {
    if (backendVersions) {
      return backendVersions.map((v) => ({
        versionNumber: v.versionNumber,
        source: (v.sceneSource === "AI" ? "AI_GENERATED" : v.sceneSource === "RESTORE" ? "RESTORE" : "HUMAN_EDITED") as SceneSource,
        createdAt: v.createdAt,
        content: v.content,
      }));
    }
    if (versionBlock) {
      return buildPreviewVersions(
        versionBlock,
        t("project.scene.versionSample", { index: versionIndex, version: 1 }),
      );
    }
    return [];
  }, [backendVersions, versionBlock, versionIndex, t]);

  const scrollToBlock = (id: string) => {
    const el = document.getElementById(id);
    if (el) el.scrollIntoView({ behavior: "smooth", block: "start" });
  };

  const outlineItems = useMemo(
    () =>
      blocks.map((b, idx) => ({
        id: b.id,
        title: isEditing
          ? t("project.scene.label", { index: idx + 1 })
          : b.text || t("project.chapter.blockTitle", { index: idx + 1 }),
      })),
    [blocks, isEditing, t],
  );

  // Committed-Plan Gate UI rendering (WR-01)
  if (backendGateError === "UNCOMMITTED_PLAN") {
    return (
      <div className="flex h-full flex-col items-center justify-center p-6 text-center">
        <div className="max-w-md rounded-2xl border border-amber-300/60 bg-amber-50/60 p-6 dark:border-amber-800 dark:bg-amber-950/30">
          <h2 className="text-base font-bold text-fg">
            Chưa có kịch bản đã cam kết
          </h2>
          <p className="mt-2 text-xs leading-relaxed text-fg-muted">
            Writing Mode chỉ khả dụng sau khi kịch bản đã được xác nhận và cam kết (Committed Plan). Vui lòng hoàn thành giai đoạn lập kế hoạch trước khi soạn thảo.
          </p>
          <Link
            to={`/project/${currentProjectId}`}
            className="mt-4 inline-flex items-center rounded-xl bg-accent px-4 py-2 text-xs font-semibold text-white hover:opacity-90"
          >
            {t("project.backToList")}
          </Link>
        </div>
      </div>
    );
  }

  // Not Found UI
  if (backendGateError === "NOT_FOUND") {
    return (
      <div className="flex h-full flex-col items-center justify-center p-6 text-center">
        <p className="text-sm text-fg-muted">{t("chapter.notFound")}</p>
        <Link to="/project" className="mt-3 text-xs font-semibold text-blue-600 hover:underline">
          {t("project.backToList")}
        </Link>
      </div>
    );
  }

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
            projectName={currentProjectTitle}
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
              to={`/project/${currentProjectId}`}
              className="truncate hover:text-fg "
              title={currentProjectTitle}
            >
              {truncate(currentProjectTitle, 15)}
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
                  disabled={!!generation && generation.status === "streaming"}
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
            <p className="mt-1 text-xs text-fg-subtle">
              {chapter?.date
                ? t("project.chapter.postedAt", { date: chapter.date })
                : "Chế độ soạn thảo trực tiếp"}
            </p>
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
                          disabled={!!activeGeneration && activeGeneration.status === "streaming"}
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
                          block.locked || activeGeneration?.status === "streaming"
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

                      {activeGeneration?.status === "streaming" && (
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
                        {activeGeneration?.status === "streaming" && (
                          <Button size="sm" variant="ghost" onClick={cancelGeneration}>
                            <X className="h-3.5 w-3.5" aria-hidden="true" />
                            <span>{t("common.cancel")}</span>
                          </Button>
                        )}
                        <Button
                          size="sm"
                          variant="ghost"
                          disabled={!hasContent || block.locked || !!generation}
                          onClick={() => handleOpenVersions(block.sceneId)}
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
              {previewVersions.length === 0 ? (
                <p className="text-xs text-fg-muted">
                  {t("project.scene.versionEmpty")}
                </p>
              ) : (
                previewVersions.map((version) => (
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
                        disabled={versionBlock.locked}
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
