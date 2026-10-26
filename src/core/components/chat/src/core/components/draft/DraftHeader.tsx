import { Check, PanelRightClose } from "lucide-react";
import { cn } from "../../../shared/utils/cn";
import type { DraftStep } from "./types";

interface DraftHeaderProps {
  currentStep: DraftStep;
  onClose: () => void;
  onStepClick: (step: DraftStep) => void;
}

export function DraftHeader({ currentStep, onClose, onStepClick }: DraftHeaderProps) {
  const steps: { id: DraftStep; label: string }[] = [
    { id: "overview", label: "Overview" },
    { id: "arcs", label: "Arcs" },
    { id: "chapter", label: "Chapter" },
    { id: "scene", label: "Scene" },
    { id: "config", label: "Config" },
    { id: "world", label: "World" },
    { id: "character", label: "Character" },
    { id: "characterDetail", label: "Detail" },
  ];

  const stepOrder: Record<DraftStep, number> = {
    overview: 1, arcs: 2, chapter: 3, scene: 4,
    config: 5, world: 6, character: 7, characterDetail: 8
  };
  const currentIdx = stepOrder[currentStep];

  return (
    <div className="flex h-14 shrink-0 items-center justify-between border-b border-slate-200 bg-white/70 px-4 backdrop-blur-xs select-none dark:border-slate-800/80 dark:bg-slate-950/70">
      
      {/* Scrollable Stepper */}
      <div className="flex items-center gap-1.5 overflow-x-auto [&::-webkit-scrollbar]:hidden [-ms-overflow-style:none] [scrollbar-width:none]">
        {steps.map((step, idx) => {
          const stepNum = idx + 1;
          const isDone = currentIdx > stepNum;
          const isActive = currentIdx === stepNum;

          return (
            <div key={step.id} className="flex items-center gap-1.5 shrink-0">
              <button
                type="button"
                onClick={() => onStepClick(step.id)}
                className="flex items-center gap-1.5 focus:outline-hidden"
              >
                <span
                  className={cn(
                    "flex h-5 w-5 items-center justify-center rounded-full text-[10px] font-semibold transition-colors",
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
        title="Thu gọn Draft panel"
      >
        <PanelRightClose className="h-4 w-4" />
      </button>
    </div>
  );
}
