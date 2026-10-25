import { Check, X } from "lucide-react";
import { cn } from "../../../shared/utils/cn";
import type { DraftStep } from "./types";

interface DraftHeaderProps {
  currentStep: DraftStep;
  onClose: () => void;
}

export function DraftHeader({ currentStep, onClose }: DraftHeaderProps) {
  const steps: { id: DraftStep; label: string }[] = [
    { id: "setup", label: "Setup" },
    { id: "character", label: "Character" },
    { id: "plan", label: "Plan" },
  ];

  const stepOrder: Record<DraftStep, number> = { setup: 1, character: 2, plan: 3 };
  const currentIdx = stepOrder[currentStep];

  return (
    <div className="flex h-14 shrink-0 items-center justify-between border-b border-slate-200 bg-white/70 px-5 backdrop-blur-xs dark:border-slate-800/80 dark:bg-slate-950/70">
      {/* Stepper Bar */}
      <div className="flex items-center gap-2">
        {steps.map((step, idx) => {
          const stepNum = idx + 1;
          const isDone = currentIdx > stepNum;
          const isActive = currentIdx === stepNum;

          return (
            <div key={step.id} className="flex items-center gap-2">
              <div className="flex items-center gap-1.5">
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
                    "text-xs font-medium transition-colors",
                    isActive
                      ? "text-slate-900 dark:text-slate-100 font-semibold"
                      : "text-slate-400 dark:text-slate-500"
                  )}
                >
                  {step.label}
                </span>
              </div>
              {idx < steps.length - 1 && (
                <span className="text-xs text-slate-300 dark:text-slate-700">›</span>
              )}
            </div>
          );
        })}
      </div>

      <button
        type="button"
        onClick={onClose}
        className="rounded-lg p-1.5 text-slate-400 hover:bg-slate-100 hover:text-slate-700 dark:hover:bg-slate-800 dark:hover:text-slate-200 transition-colors"
        title="Đóng Draft"
      >
        <X className="h-4 w-4" />
      </button>
    </div>
  );
}
