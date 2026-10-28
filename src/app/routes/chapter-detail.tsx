import { useParams, Link } from "react-router";
import { MOCK_PROJECTS } from "../../core/components/project/mockProjects";
import { ChapterDetailView } from "../../core/components/project/ChapterDetailView";
import { useT } from "../../shared/i18n/useT";

export function meta() {
  return [{ title: "Merlink Chapter Detail" }];
}

export default function ChapterDetailRoute() {
  const { projectId, chapterId } = useParams();
  const t = useT();
  const project = MOCK_PROJECTS.find((p) => p.id === projectId);

  let targetChapter = null;
  let chapterIndex = 1;

  if (project?.arcs) {
    let globalIndex = 0;
    for (const arc of project.arcs) {
      for (const chap of arc.chapters) {
        globalIndex += 1;
        if (chap.id === chapterId) {
          targetChapter = chap;
          chapterIndex = globalIndex;
          break;
        }
      }
      if (targetChapter) break;
    }
  }

  if (!project || !targetChapter) {
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
    <ChapterDetailView
      project={project}
      chapter={targetChapter}
      chapterIndex={chapterIndex}
    />
  );
}
