import { useParams, Link, useSearchParams } from "react-router";
import { ChapterDetailView } from "@/features/projects";
import { backendClient } from "@/shared/backend/client";
import { useT } from "@/shared/i18n/useT";

export default function ChapterDetailPage() {
  const { projectId, chapterId } = useParams();
  const [searchParams] = useSearchParams();
  const t = useT();
  const creatorEntry = searchParams.get("mode") === "creator";

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

  return (
    <ChapterDetailView
      key={`${projectId}-${chapterId}`}
      projectId={projectId}
      chapterId={chapterId}
      creatorEntry={creatorEntry}
      client={backendClient}
    />
  );
}
