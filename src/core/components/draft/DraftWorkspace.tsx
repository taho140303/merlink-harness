import { useState } from "react";
import { DraftHeader } from "./DraftHeader";
import { DraftSetupStep } from "./DraftSetupStep";
import { DraftCharacterStep } from "./DraftCharacterStep";
import type { DraftStep, FeedbackContext } from "./types";

interface DraftWorkspaceProps {
  onClose: () => void;
  onFeedback: (feedback: FeedbackContext) => void;
}

export function DraftWorkspace({ onClose, onFeedback }: DraftWorkspaceProps) {
  const [currentStep, setCurrentStep] = useState<DraftStep>("setup");

  return (
    <aside className="flex h-full w-full flex-col border-l border-slate-200 bg-slate-50/40 dark:border-slate-800 dark:bg-slate-950/40 overflow-hidden">
      <DraftHeader currentStep={currentStep} onClose={onClose} />

      <div className="flex-1 overflow-y-auto">
        {currentStep === "setup" && (
          <DraftSetupStep
            onFeedback={onFeedback}
            onNext={() => setCurrentStep("character")}
          />
        )}

        {currentStep === "character" && (
          <DraftCharacterStep
            onFeedback={onFeedback}
            onNext={() => setCurrentStep("plan")}
          />
        )}

        {currentStep === "plan" && (
          <div className="flex h-full items-center justify-center p-6 text-center text-xs text-slate-400">
            Plan stage under construction.
          </div>
        )}
      </div>
    </aside>
  );
}
