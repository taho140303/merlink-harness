import { useState, useMemo } from "react";
import { Search, ChevronDown } from "lucide-react";
import { MOCK_PROJECTS } from "./mockProjects";
import { ProjectCard } from "./ProjectCard";
import { ProjectDetailSidebar } from "./ProjectDetailSidebar";
import { DeleteConfirmDialog } from "../../../shared/components/ui/delete-confirm-dialog";
import { toast } from "../../../shared/components/ui/toast";
import { useT } from "../../../shared/i18n/useT";
import type { ProjectItem, SortOption } from "./types";

export default function ProjectView() {
  const t = useT();
  const [projects, setProjects] = useState<ProjectItem[]>(MOCK_PROJECTS);
  const [searchTerm, setSearchTerm] = useState("");
  const [sortBy, setSortBy] = useState<SortOption>("a-z");
  const [selectedProject, setSelectedProject] = useState<ProjectItem | null>(null);
  const [deleteTarget, setDeleteTarget] = useState<ProjectItem | null>(null);
  const [deleteOpen, setDeleteOpen] = useState(false);

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

  const confirmDelete = () => {
    if (!deleteTarget) return;
    setProjects((prev) => prev.filter((p) => p.id !== deleteTarget.id));
    toast({ title: t("project.deletedToast"), variant: "success" });
    setDeleteTarget(null);
  };

  return (
    <div className="relative flex h-full flex-col overflow-y-auto bg-slate-50/50 p-6 dark:bg-slate-950/20 sm:p-8">
      <div className="flex flex-col gap-4 border-b border-slate-200 pb-6 dark:border-slate-800 sm:flex-row sm:items-center sm:justify-between">
        <div>
          <h1 className="text-xl font-bold tracking-tight text-slate-900 dark:text-slate-100 sm:text-2xl">
            {t("project.title")}
          </h1>
          <p className="mt-1 text-xs text-slate-500 dark:text-slate-400">
            {t("project.subtitle")}
          </p>
        </div>

        <div className="flex flex-wrap items-center gap-2.5">
          <div className="relative w-full sm:w-56">
            <Search className="absolute left-3 top-1/2 h-3.5 w-3.5 -translate-y-1/2 text-slate-400" />
            <input
              type="text"
              value={searchTerm}
              onChange={(e) => setSearchTerm(e.target.value)}
              placeholder={t("project.searchPlaceholder")}
              className="w-full rounded-xl border border-slate-200 bg-white py-1.5 pl-9 pr-3 text-xs text-slate-900 placeholder:text-slate-400 focus:outline-hidden focus:ring-1 focus:ring-slate-400 dark:border-slate-800 dark:bg-slate-900 dark:text-slate-100"
            />
          </div>

          <div className="relative">
            <select
              value={sortBy}
              onChange={(e) => setSortBy(e.target.value as SortOption)}
              className="w-full appearance-none rounded-xl border border-slate-200 bg-white py-1.5 pl-3 pr-8 text-xs font-medium text-slate-700 shadow-2xs focus:outline-hidden focus:ring-1 focus:ring-slate-400 dark:border-slate-800 dark:bg-slate-900 dark:text-slate-200"
            >
              <option value="a-z">{t("project.sort.a-z")}</option>
              <option value="z-a">{t("project.sort.z-a")}</option>
              <option value="modified">{t("project.sort.modified")}</option>
            </select>
            <ChevronDown className="pointer-events-none absolute right-2.5 top-1/2 h-3.5 w-3.5 -translate-y-1/2 text-slate-400" />
          </div>
        </div>
      </div>

      <div className="mt-6 grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-2">
        {filteredAndSortedProjects.map((project) => (
          <ProjectCard
            key={project.id}
            project={project}
            onDelete={requestDelete}
            onOverview={(p) => setSelectedProject(p)}
          />
        ))}
        {filteredAndSortedProjects.length === 0 && (
          <div className="col-span-full py-16 text-center text-xs text-slate-400">
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
