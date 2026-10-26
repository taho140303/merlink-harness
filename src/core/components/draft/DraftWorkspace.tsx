import { useState } from 'react';
import { DraftHeader } from './DraftHeader';
import { OverviewStep } from './steps/OverviewStep';
import { ArcsStep } from './steps/ArcsStep';
import { CumulativeMasterLayout } from './CumulativeMasterLayout';
import { toast } from '../../../shared/components/ui/toast';
import type { DraftStep, FeedbackContext } from './types';

interface DraftWorkspaceProps {
  onClose: () => void;
  onFeedback: (feedback: FeedbackContext) => void;
}

export function DraftWorkspace({ onClose, onFeedback }: DraftWorkspaceProps) {
  const [currentStep, setCurrentStep] = useState<DraftStep>('overview');

  const handleStart = () => {
    toast({ title: 'Bắt đầu sinh nội dung truyện...', variant: 'success' });
    onClose();
  };

  // Các phase liên tiếp cho Cumulative Layout
  const cumulativePhases: DraftStep[] = ['chapter', 'scene', 'config', 'world', 'character', 'characterDetail'];
  const isCumulative = cumulativePhases.includes(currentStep);

  const getNextStep = (current: DraftStep): DraftStep => {
    const idx = cumulativePhases.indexOf(current);
    if (idx >= 0 && idx < cumulativePhases.length - 1) return cumulativePhases[idx + 1];
    return current;
  };

  const getPrevStep = (current: DraftStep): DraftStep => {
    const idx = cumulativePhases.indexOf(current);
    if (idx > 0) return cumulativePhases[idx - 1];
    if (idx === 0) return 'arcs';
    return current;
  };

  return (
    <aside className="flex h-full w-full flex-col border-l border-slate-200 bg-slate-50/40 dark:border-slate-800 dark:bg-slate-950/40 overflow-hidden">
      <DraftHeader
        currentStep={currentStep}
        onClose={onClose}
        onStepClick={setCurrentStep}
      />

      <div className="flex-1 overflow-y-auto p-6 pb-24">
        {currentStep === 'overview' && (
          <OverviewStep onFeedback={onFeedback} onNext={() => setCurrentStep('arcs')} />
        )}
        
        {currentStep === 'arcs' && (
          <ArcsStep onFeedback={onFeedback} onNext={() => setCurrentStep('chapter')} onBack={() => setCurrentStep('overview')} />
        )}

        {isCumulative && (
          <CumulativeMasterLayout
            phase={currentStep}
            onFeedback={onFeedback}
            onNext={currentStep !== 'characterDetail' ? () => setCurrentStep(getNextStep(currentStep)) : undefined}
            onBack={() => setCurrentStep(getPrevStep(currentStep))}
            onStart={currentStep === 'characterDetail' ? handleStart : undefined}
          />
        )}
      </div>
    </aside>
  );
}
