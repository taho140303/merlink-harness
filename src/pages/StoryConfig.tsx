import { Sliders } from "lucide-react";
import { ComingSoon } from "../components/ComingSoon";

export function StoryConfig() {
  return (
    <div>
      <div className="page-topbar">
        <span className="page-breadcrumb">
          Workspace / <strong>Story configuration</strong>
        </span>
      </div>
      <ComingSoon
        icon={<Sliders size={21} />}
        title="Story configuration"
        description="Shape how each chapter comes to life: writing style, ground rules, tone, and output format."
        note="Coming soon in Sprint 8"
      />
    </div>
  );
}
