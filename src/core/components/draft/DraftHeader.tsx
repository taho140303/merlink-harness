import { Check, PanelRightClose } from "lucide-react";
import { cn } from "../../../shared/utils/cn";
import { useT } from "../../../shared/i18n/useT";
import type { DraftStep, CharacterSubStep } from "./types";

interface DraftHeaderProps {
  currentStep: DraftStep;
  currentSubStep: CharacterSubStep;
  onClose: () => void;
}

export function DraftHeader({
  currentStep,
  currentSubStep,
  onClose,
}: DraftHeaderProps) {
  const t = useT();

  const steps: { id: DraftStep; label: string }[] = [
    { id: "overview", label: t("draft.step.overview") },
    { id: "chapter", label: t("draft.step.chapter") },
    { id: "world", label: t("draft.step.world") },
    { id: "scene", label: t("draft.step.scene") },
    { id: "character", label: t("draft.step.character") },
    { id: "relationship", label: t("draft.step.relationship") },
  ];

  const subSteps: { id: CharacterSubStep; label: string }[] = [
    { id: "appearance", label: t("draft.subStep.appearance") },
    { id: "psychological", label: t("draft.subStep.psychological") },
    { id: "lifeExperience", label: t("draft.subStep.lifeExperience") },
  ];

  const stepOrder: Record<DraftStep, number> = {
    overview: 1, chapter: 2, world: 3, scene: 4, character: 5, relationship: 6,
  };
  const currentIdx = stepOrder[currentStep];

  return (
    <div className="flex flex-col border-b border-line bg-surface-card/70 backdrop-blur-xs   select-none">
      {/* Main Progress Bar */}
      <div className="flex h-14 shrink-0 items-center justify-between px-4">
        <div className="flex items-center gap-1.5 overflow-x-auto [&::-webkit-scrollbar]:hidden [-ms-overflow-style:none] [scrollbar-width:none] pointer-events-none">
          {steps.map((step, idx) => {
            const stepNum = idx + 1;
            const isDone = currentIdx > stepNum;
            const isActive = currentIdx === stepNum;

            return (
              <div key={step.id} className="flex shrink-0 items-center gap-1.5">
                <div className="flex items-center gap-1.5">
                  <span
                    className={cn(
                      "flex h-5 w-5 items-center justify-center rounded-full text-xs font-semibold transition-colors",
                      isDone
                        ? "bg-emerald-500 text-white"
                        : isActive
                          ? "bg-accent text-white  "
                          : "bg-surface-muted text-fg-subtle  -subtle"
                    )}
                  >
                    {isDone ? <Check className="h-3 w-3 stroke-[3]" /> : stepNum}
                  </span>
                  <span
                    className={cn(
                      "text-xs font-medium transition-colors",
                      isActive
                        ? "font-semibold text-fg"
                        : "text-fg-subtle -subtle"
                    )}
                  >
                    {step.label}
                  </span>
                </div>
                {idx < steps.length - 1 && (
                  <span className="text-xs text-fg-subtle -muted">›</span>
                )}
              </div>
            );
          })}
        </div>

        <button
          type="button"
          onClick={onClose}
          className="ml-2 shrink-0 rounded-lg p-1.5 text-fg-subtle hover:bg-surface-muted hover:text-fg-muted   transition-colors pointer-events-auto"
          title={t('draft.nav.collapse')}
        >
          <PanelRightClose className="h-4 w-4" />
        </button>
      </div>

      {/* Sub Progress Bar (Chỉ hiện khi ở bước Character) */}
      {currentStep === "character" && (
        <div className="flex h-10 items-center gap-4 border-t border-line bg-surface-muted/50 px-6    -from-top-2 duration-200">
          <span className="text-xs font-bold uppercase tracking-wider text-fg-subtle pointer-events-none">
            5.1 - 5.3
          </span>
          <div className="flex items-center gap-3 pointer-events-none">
            {subSteps.map((sub, idx) => {
              const isActive = currentSubStep === sub.id;
              return (
                <div key={sub.id} className="flex items-center gap-3">
                  <div
                    className={cn(
                      "text-xs font-medium",
                      isActive
                        ? "text-blue-600 dark:text-blue-400 font-semibold"
                        : "text-fg-muted"
                    )}
                  >
                    5.{idx + 1} {sub.label}
                  </div>
                  {idx < subSteps.length - 1 && (
                    <span className="text-fg-subtle -muted text-xs">•</span>
                  )}
                </div>
              );
            })}
          </div>
        </div>
      )}
    </div>
  );
}
