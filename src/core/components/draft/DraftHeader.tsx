import { Check, PanelRightClose } from "lucide-react";
import { cn } from "../../../shared/utils/cn";
import { useT } from "../../../shared/i18n/useT";
import type { DraftStep, CharacterSubStep } from "./types";

interface DraftHeaderProps {
  currentStep: DraftStep;
  currentSubStep: CharacterSubStep;
  onStepClick: (step: DraftStep) => void;
  onSubStepClick: (subStep: CharacterSubStep) => void;
  onClose: () => void;
}

export function DraftHeader({
  currentStep,
  currentSubStep,
  onStepClick,
  onSubStepClick,
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
    <div className="flex flex-col border-b border-slate-200 bg-white/70 backdrop-blur-xs dark:border-slate-800/80 dark:bg-slate-950/70 select-none">
      {/* Main Progress Bar */}
      <div className="flex h-14 shrink-0 items-center justify-between px-4">
        <div className="flex items-center gap-1.5 overflow-x-auto [&::-webkit-scrollbar]:hidden [-ms-overflow-style:none] [scrollbar-width:none]">
          {steps.map((step, idx) => {
            const stepNum = idx + 1;
            const isDone = currentIdx > stepNum;
            const isActive = currentIdx === stepNum;

            return (
              <div key={step.id} className="flex shrink-0 items-center gap-1.5">
                <button
                  type="button"
                  onClick={() => onStepClick(step.id)}
                  className="flex items-center gap-1.5 focus:outline-hidden"
                >
                  <span
                    className={cn(
                      "flex h-5 w-5 items-center justify-center rounded-full text-[11px] font-semibold transition-colors",
                      isDone
                        ? "bg-emerald-500 text-white"
                        : isActive
                          ? "bg-slate-900 text-white dark:bg-slate-100 dark:text-slate-900"
                          : "bg-slate-100 text-slate-400 dark:bg-slate-800 dark:text-slate-500"
                    )}
                  >
                    {isDone ? <Check className="h-3 w-3 stroke-[3]" /> : stepNum}
                  </span>
                  <span
                    className={cn(
                      "text-[11px] font-medium transition-colors hover:text-slate-900 dark:hover:text-slate-100",
                      isActive
                        ? "font-semibold text-slate-900 dark:text-slate-100"
                        : "text-slate-400 dark:text-slate-500"
                    )}
                  >
                    {step.label}
                  </span>
                </button>
                {idx < steps.length - 1 && (
                  <span className="text-[10px] text-slate-300 dark:text-slate-700">›</span>
                )}
              </div>
            );
          })}
        </div>

        <button
          type="button"
          onClick={onClose}
          className="ml-2 shrink-0 rounded-lg p-1.5 text-slate-400 hover:bg-slate-100 hover:text-slate-700 dark:hover:bg-slate-800 dark:hover:text-slate-200 transition-colors"
          title="Thu gọn"
        >
          <PanelRightClose className="h-4 w-4" />
        </button>
      </div>

      {/* Sub Progress Bar (Chỉ hiện khi ở bước Character) */}
      {currentStep === "character" && (
        <div className="flex h-10 items-center gap-4 border-t border-slate-100 bg-slate-50/50 px-6 dark:border-slate-800/60 dark:bg-slate-900/30 animate-in slide-in-from-top-2 duration-200">
          <span className="text-[10px] font-bold uppercase tracking-wider text-slate-400">
            5.1 - 5.3
          </span>
          <div className="flex items-center gap-3">
            {subSteps.map((sub, idx) => {
              const isActive = currentSubStep === sub.id;
              return (
                <div key={sub.id} className="flex items-center gap-3">
                  <button
                    type="button"
                    onClick={() => onSubStepClick(sub.id)}
                    className={cn(
                      "text-[11px] font-medium transition-colors hover:text-slate-900 dark:hover:text-slate-100",
                      isActive
                        ? "text-blue-600 dark:text-blue-400 font-semibold"
                        : "text-slate-500 dark:text-slate-400"
                    )}
                  >
                    5.{idx + 1} {sub.label}
                  </button>
                  {idx < subSteps.length - 1 && (
                    <span className="text-slate-300 dark:text-slate-700 text-[11px]">•</span>
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
