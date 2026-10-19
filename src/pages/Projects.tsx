import { BookOpen } from "lucide-react";
import { ComingSoon } from "../components/ComingSoon";

export function Projects() {
  return (
    <div>
      <div className="page-topbar">
        <span className="page-breadcrumb">
          Workspace / <strong>Projects</strong>
        </span>
      </div>
      <ComingSoon
        icon={<BookOpen size={21} />}
        title="Story projects"
        description="Every world starts with a first page. Keep your stories close and make space for what comes next."
        note="Coming soon in Sprint 8"
      />
    </div>
  );
}
