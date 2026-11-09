import { useState, useMemo, useEffect } from "react";
import { Search, ChevronDown } from "lucide-react";
import { ProjectCard } from "./ProjectCard";
import { ProjectDetailSidebar } from "./ProjectDetailSidebar";
import { DeleteConfirmDialog } from "@/shared/components/ui/delete-confirm-dialog";
import { toast } from "@/shared/components/ui/toast";
import { useT } from "@/shared/i18n/useT";
import { backendClient, type BackendClient } from "@/shared/backend/client";
import type { ProjectItemUiDto } from "@/shared/backend/generated";
import type { ProjectItem, SortOption } from "./types";

const ENTITY_CODE_PATTERN = /\b(CHAR|REL|FACTION|EVENT|LOC|ARC|CH|SC|SCN|THM|PROJ|COMP)_[A-Za-z0-9_-]+\b/gi;
const CODE_PREFIX_REGEX = /^(\[[A-Za-z0-9_-]+\]\s*|[A-Za-z0-9_-]+(?::|\s+[-–—])\s*)/i;

/**
 * Sanitizes entity names and titles to ensure internal codes (CHAR_001, ARC_001, etc.)
 * are never exposed as user-visible labels in rendered outputs.
 */
export function sanitizeDisplayLabel(
  nameOrTitle?: string | null,
  code?: string,
  fallback = "Chưa đặt tên"
): string {
  if (!nameOrTitle || typeof nameOrTitle !== "string") return fallback;
  let label = nameOrTitle.trim();
  if (code && typeof code === "string" && code.length > 0) {
    const escaped = code.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
    label = label.replace(new RegExp(`^\\[${escaped}\\]\\s*`, "i"), "");
    label = label.replace(new RegExp(`^${escaped}(?::|\\s+[-–—])\\s*`, "i"), "");
    if (label.trim().toLowerCase() === code.trim().toLowerCase()) return fallback;
    label = label.replace(new RegExp(`\\b${escaped}\\b`, "gi"), "").trim();
  }
  label = label.replace(CODE_PREFIX_REGEX, "");
  label = label.replace(ENTITY_CODE_PATTERN, "").trim();
  label = label.replace(/^[:\-–—\s]+/, "").trim();
  return label || fallback;
}

/**
 * Maps a ProjectItemUiDto from the backend transport into a clean ProjectItem.
 */
export function mapProjectDtoToItem(dto: ProjectItemUiDto): ProjectItem {
  const cleanTitle = sanitizeDisplayLabel(dto.title, dto.id, "Dự án");
  const validStatus: ProjectItem["status"] =
    dto.status === "Completed" ||
    dto.status === "Drafting" ||
    dto.status === "In Review" ||
    dto.status === "Ongoing" ||
    dto.status === "Planning"
      ? dto.status
      : "Drafting";

  return {
    id: dto.id,
    title: cleanTitle,
    status: validStatus,
    cover: dto.cover || "",
    description: sanitizeDisplayLabel(dto.description, undefined, dto.description || ""),
    genres: dto.genres || [],
    modifiedAt: dto.modifiedAt,
    details: {
      wordCount: dto.details?.wordCount ?? "0",
      chapters: dto.details?.chapters ?? "0",
      characters: dto.details?.characters ?? "0",
      worldLore: dto.details?.worldLore ?? "0",
      createdDate: dto.details?.createdDate ?? "",
    },
    arcs: dto.arcs?.map((a) => ({
      id: a.id,
      title: sanitizeDisplayLabel(a.title, a.id, "Hồi"),
      cover: a.cover ?? undefined,
      chapters: (a.chapters || []).map((c) => ({
        id: c.id,
        title: sanitizeDisplayLabel(c.title, c.id, "Chương"),
        date: c.date,
        isNew: c.isNew ?? false,
        blocks: (c.blocks || []).map((b) => ({
          id: b.id,
          sceneId: b.sceneId ?? undefined,
          text: b.text,
          locked: b.locked,
        })),
      })),
    })),
  };
}

/**
 * Sanitizes an existing ProjectItem to ensure no entity codes are leaked into labels.
 */
export function sanitizeProjectItem(project: ProjectItem): ProjectItem {
  return {
    ...project,
    title: sanitizeDisplayLabel(project.title, project.id, "Dự án"),
    description: sanitizeDisplayLabel(project.description, undefined, project.description),
    arcs: project.arcs?.map((a) => ({
      ...a,
      title: sanitizeDisplayLabel(a.title, a.id, "Hồi"),
      chapters: a.chapters.map((c) => ({
        ...c,
        title: sanitizeDisplayLabel(c.title, c.id, "Chương"),
      })),
    })),
  };
}

export interface ProjectViewProps {
  initialProjects?: ProjectItem[];
  client?: BackendClient;
}

export default function ProjectView({ initialProjects, client: propClient }: ProjectViewProps = {}) {
  const t = useT();
  const activeClient = propClient || backendClient;
  const [projects, setProjects] = useState<ProjectItem[]>(() => {
    if (initialProjects) return initialProjects.map(sanitizeProjectItem);
    return [];
  });
  const [searchTerm, setSearchTerm] = useState("");
  const [sortBy, setSortBy] = useState<SortOption>("a-z");
  const [selectedProject, setSelectedProject] = useState<ProjectItem | null>(null);
  const [deleteTarget, setDeleteTarget] = useState<ProjectItem | null>(null);
  const [deleteOpen, setDeleteOpen] = useState(false);

  useEffect(() => {
    let isMounted = true;
    let retryTimer: ReturnType<typeof setTimeout> | null = null;
    let attempts = 0;

    if (initialProjects) {
      setProjects(initialProjects.map(sanitizeProjectItem));
      return;
    }

    const loadProjects = () => {
      if (!isMounted) return;
      if (activeClient && activeClient.status !== "unavailable") {
        activeClient
          .listProjects()
          .then((res) => {
            if (!isMounted) return;
            const mapped = (res.items || []).map(mapProjectDtoToItem);
            setProjects(mapped);
          })
          .catch((err) => {
            console.warn("Failed to load projects from backend transport:", err);
          });
      } else if (attempts < 10) {
        attempts += 1;
        retryTimer = setTimeout(loadProjects, 150);
      }
    };

    loadProjects();

    return () => {
      isMounted = false;
      if (retryTimer) clearTimeout(retryTimer);
    };
  }, [activeClient, initialProjects]);

  const filteredAndSortedProjects = useMemo(() => {
    return projects
      .filter((p) => {
        const q = searchTerm.toLowerCase();
        return (
          p.title.toLowerCase().includes(q) ||
          p.description.toLowerCase().includes(q) ||
          p.genres.some((g) => g.toLowerCase().includes(q))
        );
      })
      .sort((a, b) => {
        if (sortBy === "a-z") return a.title.localeCompare(b.title);
        if (sortBy === "z-a") return b.title.localeCompare(a.title);
        if (sortBy === "modified") return b.modifiedAt.localeCompare(a.modifiedAt);
        return 0;
      });
  }, [projects, searchTerm, sortBy]);

  const requestDelete = (project: ProjectItem) => {
    setDeleteTarget(project);
    setDeleteOpen(true);
  };

  const confirmDelete = async () => {
    if (!deleteTarget) return;
    const targetId = deleteTarget.id;
    try {
      if (activeClient && activeClient.status !== "unavailable") {
        await activeClient.deleteProject({ projectId: targetId });
      }
      setProjects((prev) => prev.filter((p) => p.id !== targetId));
      if (selectedProject?.id === targetId) {
        setSelectedProject(null);
      }
      toast({ title: t("project.deletedToast"), variant: "success" });
    } catch (err) {
      console.error("deleteProject failed:", err);
      toast({ title: "Không thể xóa dự án", variant: "error" });
    } finally {
      setDeleteTarget(null);
      setDeleteOpen(false);
    }
  };

  return (
    <div className="relative flex h-full flex-col overflow-y-auto bg-surface-muted/50 p-6 sm:p-8">
      <div className="flex flex-col gap-4 border-b border-line pb-6 sm:flex-row sm:items-center sm:justify-between">
        <div>
          <h1 className="text-xl font-bold tracking-tight text-fg sm:text-2xl">
            {t("project.title")}
          </h1>
          <p className="mt-1 text-xs text-fg-muted">
            {t("project.subtitle")}
          </p>
        </div>

        <div className="flex flex-wrap items-center gap-2.5">
          <div className="relative w-full sm:w-56">
            <Search className="absolute left-3 top-1/2 h-3.5 w-3.5 -translate-y-1/2 text-fg-subtle" />
            <input
              type="text"
              value={searchTerm}
              onChange={(e) => setSearchTerm(e.target.value)}
              placeholder={t("project.searchPlaceholder")}
              className="w-full tier-card rounded-card py-1.5 pl-9 pr-3 text-xs text-fg placeholder:text-fg-subtle focus-ring"
            />
          </div>

          <div className="relative">
            <select
              value={sortBy}
              onChange={(e) => setSortBy(e.target.value as SortOption)}
              className="w-full appearance-none tier-card rounded-card py-1.5 pl-3 pr-8 text-xs font-medium text-fg-muted focus-ring"
            >
              <option value="a-z">{t("project.sort.a-z")}</option>
              <option value="z-a">{t("project.sort.z-a")}</option>
              <option value="modified">{t("project.sort.modified")}</option>
            </select>
            <ChevronDown className="pointer-events-none absolute right-2.5 top-1/2 h-3.5 w-3.5 -translate-y-1/2 text-fg-subtle" />
          </div>
        </div>
      </div>

      <div className="mt-6 grid grid-cols-1 gap-4 xl:grid-cols-2">
        {filteredAndSortedProjects.map((project) => (
          <ProjectCard
            key={project.id}
            project={project}
            onDelete={requestDelete}
            onOverview={(p) => setSelectedProject(p)}
          />
        ))}
        {filteredAndSortedProjects.length === 0 && (
          <div className="col-span-full py-16 text-center text-xs text-fg-subtle">
            {t("project.empty")}
          </div>
        )}
      </div>

      {selectedProject && (
        <div
          onClick={() => setSelectedProject(null)}
          className="fixed inset-0 z-30 bg-black/20 backdrop-blur-2xs transition-opacity"
        />
      )}

      <ProjectDetailSidebar
        project={selectedProject}
        onClose={() => setSelectedProject(null)}
      />

      <DeleteConfirmDialog
        open={deleteOpen}
        onOpenChange={setDeleteOpen}
        title={t("project.delete.title")}
        description={`${deleteTarget?.title ?? ""} — ${t("project.delete.description")}`}
        confirmLabel={t("delete.confirm")}
        cancelLabel={t("common.cancel")}
        onConfirm={confirmDelete}
      />
    </div>
  );
}
