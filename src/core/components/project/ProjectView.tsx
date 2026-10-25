import { useState, useMemo } from "react";
import { Search, ChevronDown, Trash2, Check } from "lucide-react";
import { cn } from "../../../shared/utils/cn";
import { MOCK_PROJECTS } from "./mockProjects";
import { ProjectCard } from "./ProjectCard";
import { ProjectDetailSidebar } from "./ProjectDetailSidebar";
import type { ProjectItem, SortOption } from "./types";

export default function ProjectView() {
  const [projects, setProjects] = useState<ProjectItem[]>(MOCK_PROJECTS);
  const [selectedIds, setSelectedIds] = useState<string[]>([]);
  const [searchTerm, setSearchTerm] = useState("");
  const [sortBy, setSortBy] = useState<SortOption>("a-z");
  const [selectedProject, setSelectedProject] = useState<ProjectItem | null>(null);

  const filteredAndSortedProjects = useMemo(() => {
    return projects.filter((p) => {
      const matchSearch =
        p.title.toLowerCase().includes(searchTerm.toLowerCase()) ||
        p.description.toLowerCase().includes(searchTerm.toLowerCase()) ||
        p.genres.some((g) => g.toLowerCase().includes(searchTerm.toLowerCase()));
      return matchSearch;
    }).sort((a, b) => {
      if (sortBy === "a-z") return a.title.localeCompare(b.title);
      if (sortBy === "z-a") return b.title.localeCompare(a.title);
      if (sortBy === "modified") return b.modifiedAt.localeCompare(a.modifiedAt);
      return 0;
    });
  }, [projects, searchTerm, sortBy]);

  const allSelected =
    filteredAndSortedProjects.length > 0 &&
    filteredAndSortedProjects.every((p) => selectedIds.includes(p.id));

  const toggleSelectAll = () => {
    if (allSelected) {
      setSelectedIds([]);
    } else {
      setSelectedIds(filteredAndSortedProjects.map((p) => p.id));
    }
  };

  const toggleSelectOne = (id: string) => {
    setSelectedIds((prev) =>
      prev.includes(id) ? prev.filter((i) => i !== id) : [...prev, id]
    );
  };

  const handleSoftDelete = () => {
    if (selectedIds.length === 0) return;
    setProjects((prev) => prev.filter((p) => !selectedIds.includes(p.id)));
    setSelectedIds([]);
  };

  return (
    <div className="relative flex h-full flex-col overflow-y-auto bg-slate-50/50 p-6 dark:bg-slate-950/20 sm:p-8">
      {/* Header trang Project */}
      <div className="flex flex-col gap-4 border-b border-slate-200 pb-6 dark:border-slate-800 sm:flex-row sm:items-center sm:justify-between">
        <div>
          <h1 className="text-xl font-bold tracking-tight text-slate-900 dark:text-slate-100 sm:text-2xl">
            Project
          </h1>
          <p className="mt-1 text-xs text-slate-500 dark:text-slate-400">
            Quản lý các kịch bản thế giới và tiến trình văn bản của bạn.
          </p>
        </div>

        {/* Action Controls: Radio Select All + Soft Delete + Search + Sort */}
        <div className="flex flex-wrap items-center gap-2.5">
          {/* Nút radio Select All / Deselect All */}
          <button
            type="button"
            onClick={toggleSelectAll}
            className={cn(
              "flex items-center gap-2 rounded-xl border px-3 py-1.5 text-xs font-medium shadow-2xs transition-colors",
              allSelected
                ? "border-blue-500 bg-blue-50 text-blue-700 dark:border-blue-600 dark:bg-blue-950/60 dark:text-blue-300"
                : "border-slate-200 bg-white text-slate-700 hover:bg-slate-50 dark:border-slate-800 dark:bg-slate-900 dark:text-slate-300 dark:hover:bg-slate-800"
            )}
            title={allSelected ? "Deselect All" : "Select All"}
          >
            <div
              className={cn(
                "flex h-4 w-4 items-center justify-center rounded-full border transition-all",
                allSelected
                  ? "border-blue-600 bg-blue-600 text-white"
                  : "border-slate-400 bg-transparent"
              )}
            >
              {allSelected && <Check className="h-2.5 w-2.5 stroke-[3]" />}
            </div>
            <span>{allSelected ? "Deselect All" : "Select All"}</span>
          </button>

          {/* Nút Soft Delete */}
          {selectedIds.length > 0 && (
            <button
              type="button"
              onClick={handleSoftDelete}
              className="flex items-center gap-1.5 rounded-xl border border-rose-200 bg-rose-50 px-3 py-1.5 text-xs font-medium text-rose-600 hover:bg-rose-100 dark:border-rose-900/60 dark:bg-rose-950/40 dark:text-rose-300 dark:hover:bg-rose-900/60 transition-colors animate-in fade-in-0 duration-200"
              title="Xóa mềm các dự án đã chọn"
            >
              <Trash2 className="h-3.5 w-3.5" />
              <span>Soft Delete ({selectedIds.length})</span>
            </button>
          )}

          {/* Search box */}
          <div className="relative w-full sm:w-56">
            <Search className="absolute left-3 top-1/2 h-3.5 w-3.5 -translate-y-1/2 text-slate-400" />
            <input
              type="text"
              value={searchTerm}
              onChange={(e) => setSearchTerm(e.target.value)}
              placeholder="Search projects or genres..."
              className="w-full rounded-xl border border-slate-200 bg-white pl-9 pr-3 py-1.5 text-xs text-slate-900 placeholder:text-slate-400 focus:outline-hidden focus:ring-1 focus:ring-slate-400 dark:border-slate-800 dark:bg-slate-900 dark:text-slate-100"
            />
          </div>

          {/* Sort selector */}
          <div className="relative">
            <select
              value={sortBy}
              onChange={(e) => setSortBy(e.target.value as SortOption)}
              className="w-full appearance-none rounded-xl border border-slate-200 bg-white pl-3 pr-8 py-1.5 text-xs font-medium text-slate-700 shadow-2xs focus:outline-hidden focus:ring-1 focus:ring-slate-400 dark:border-slate-800 dark:bg-slate-900 dark:text-slate-200"
            >
              <option value="a-z">Title (A - Z)</option>
              <option value="z-a">Title (Z - A)</option>
              <option value="modified">Modified Date</option>
            </select>
            <ChevronDown className="pointer-events-none absolute right-2.5 top-1/2 h-3.5 w-3.5 -translate-y-1/2 text-slate-400" />
          </div>
        </div>
      </div>

      {/* Grid danh sách Project */}
      <div className="mt-6 grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-2">
        {filteredAndSortedProjects.map((project) => (
          <ProjectCard
            key={project.id}
            project={project}
            isSelected={selectedIds.includes(project.id)}
            onToggleSelect={toggleSelectOne}
            onOverview={(p) => setSelectedProject(p)}
          />
        ))}

        {filteredAndSortedProjects.length === 0 && (
          <div className="col-span-full py-16 text-center text-xs text-slate-400">
            Không tìm thấy dự án phù hợp với từ khóa tìm kiếm.
          </div>
        )}
      </div>

      {/* Backdrop khi mở sidebar */}
      {selectedProject && (
        <div
          onClick={() => setSelectedProject(null)}
          className="fixed inset-0 z-30 bg-black/20 backdrop-blur-2xs transition-opacity"
        />
      )}

      {/* Sidebar chi tiết bên phải */}
      <ProjectDetailSidebar
        project={selectedProject}
        onClose={() => setSelectedProject(null)}
      />
    </div>
  );
}
