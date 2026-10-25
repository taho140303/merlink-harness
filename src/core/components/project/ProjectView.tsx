import { useState, useMemo } from "react";
import { Search, ChevronDown } from "lucide-react";
import { MOCK_PROJECTS } from "./mockProjects";
import { ProjectCard } from "./ProjectCard";
import { ProjectDetailSidebar } from "./ProjectDetailSidebar";
import type { ProjectItem, SortOption } from "./types";

export default function ProjectView() {
  const [searchTerm, setSearchTerm] = useState("");
  const [sortBy, setSortBy] = useState<SortOption>("a-z");
  const [selectedProject, setSelectedProject] = useState<ProjectItem | null>(null);

  const filteredAndSortedProjects = useMemo(() => {
    return MOCK_PROJECTS.filter((p) => {
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
  }, [searchTerm, sortBy]);

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

        {/* Input Search nằm kế Select Box sort */}
        <div className="flex flex-col gap-2.5 sm:flex-row sm:items-center">
          <div className="relative w-full sm:w-64">
            <Search className="absolute left-3 top-1/2 h-3.5 w-3.5 -translate-y-1/2 text-slate-400" />
            <input
              type="text"
              value={searchTerm}
              onChange={(e) => setSearchTerm(e.target.value)}
              placeholder="Search projects or genres..."
              className="w-full rounded-xl border border-slate-200 bg-white pl-9 pr-3 py-1.5 text-xs text-slate-900 placeholder:text-slate-400 focus:outline-hidden focus:ring-1 focus:ring-slate-400 dark:border-slate-800 dark:bg-slate-900 dark:text-slate-100"
            />
          </div>

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

      {/* Grid danh sách Project (2 cột trên màn hình rộng) */}
      <div className="mt-6 grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-2">
        {filteredAndSortedProjects.map((project) => (
          <ProjectCard
            key={project.id}
            project={project}
            onView={(p) => setSelectedProject(p)}
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
