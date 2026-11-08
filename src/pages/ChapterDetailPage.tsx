import { useParams, Link, useSearchParams } from "react-router";
import { ChapterDetailView, MOCK_PROJECTS } from "@/features/projects";
import { backendClient } from "@/shared/backend/client";
import { useT } from "@/shared/i18n/useT";

export default function ChapterDetailPage() {
  const { projectId, chapterId } = useParams();
  const [searchParams] = useSearchParams();
  const t = useT();
  const creatorEntry = searchParams.get("mode") === "creator";

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

  if (!projectId || !chapterId) {
    return (
      <div className="flex h-full flex-col items-center justify-center p-6 text-center">
        <p className="text-sm text-fg-muted">{t("chapter.notFound")}</p>
        <Link to="/project" className="mt-3 text-xs font-semibold text-blue-600 hover:underline">
          {t("project.backToList")}
        </Link>
      </div>
    );
  }

  if ((!project || !targetChapter) && (!backendClient || backendClient.status === "unavailable")) {
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
      key={`${projectId}-${chapterId}`}
      projectId={projectId}
      chapterId={chapterId}
      project={project}
      chapter={targetChapter || undefined}
      chapterIndex={chapterIndex}
      creatorEntry={creatorEntry}
      client={backendClient}
    />
  );
}
