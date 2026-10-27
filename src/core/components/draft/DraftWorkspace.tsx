import { useState } from 'react';
import { DraftHeader } from './DraftHeader';
import { DraftOverviewBlock } from './DraftOverviewBlock';
import { DraftStepNav } from './DraftStepNav';
import { DRAFT_OVERVIEW } from './mockOverview';
import { toast } from '../../../shared/components/ui/toast';
import { useT } from '../../../shared/i18n/useT';
import type { DraftStep, CharacterSubStep, FeedbackContext, DraftOverview } from './types';

interface DraftWorkspaceProps {
  onClose: () => void;
  onFeedback: (feedback: FeedbackContext) => void;
  overview?: DraftOverview;
}

const STEP_ORDER: DraftStep[] = ['overview', 'chapter', 'world', 'scene', 'character', 'relationship'];

export function DraftWorkspace({ onClose, onFeedback, overview = DRAFT_OVERVIEW }: DraftWorkspaceProps) {
  const t = useT();
  const [currentStep, setCurrentStep] = useState<DraftStep>('overview');
  const [currentSubStep, setCurrentSubStep] = useState<CharacterSubStep>('appearance');

  const handleStart = () => {
    toast({ title: 'Bắt đầu sinh nội dung truyện...', variant: 'success' });
    onClose();
  };

  // Logic chuyển bước tự động (hỗ trợ đi qua sub-step)
  const handleNext = () => {
    if (currentStep === 'overview') setCurrentStep('chapter');
    else if (currentStep === 'chapter') setCurrentStep('world');
    else if (currentStep === 'world') setCurrentStep('scene');
    else if (currentStep === 'scene') setCurrentStep('character');
    else if (currentStep === 'character') {
      if (currentSubStep === 'appearance') setCurrentSubStep('psychological');
      else if (currentSubStep === 'psychological') setCurrentSubStep('lifeExperience');
      else if (currentSubStep === 'lifeExperience') setCurrentStep('relationship');
    }
  };

  const handleBack = () => {
    if (currentStep === 'relationship') {
      setCurrentStep('character');
      setCurrentSubStep('lifeExperience');
      return;
    }

    if (currentStep === 'character') {
      if (currentSubStep === 'lifeExperience') setCurrentSubStep('psychological');
      else if (currentSubStep === 'psychological') setCurrentSubStep('appearance');
      else setCurrentStep('scene');
      return;
    }

    const currentIdx = STEP_ORDER.indexOf(currentStep);
    if (currentIdx > 0) setCurrentStep(STEP_ORDER[currentIdx - 1]);
  };

  const currentStepNumber = STEP_ORDER.indexOf(currentStep) + 1;

  return (
    <aside className="flex h-full w-full flex-col overflow-hidden border-l border-slate-200 bg-slate-50/40 dark:border-slate-800 dark:bg-slate-950/40">
      <DraftHeader
        currentStep={currentStep}
        currentSubStep={currentSubStep}
        onStepClick={(step) => {
          setCurrentStep(step);
          if (step === 'character') setCurrentSubStep('appearance');
        }}
        onSubStepClick={setCurrentSubStep}
        onClose={onClose}
      />

      <div className="flex-1 overflow-y-auto p-6 pb-24">
        {/* Khối tổng quan dự án: giữ nguyên toàn bộ layout và hiện ở mọi phase */}
        <DraftOverviewBlock
          overview={overview}
          onFeedback={() =>
            onFeedback({ id: 'temp', tag: 'overview', title: t('draft.step.overview') })
          }
        />

        <DraftStepNav
          onBack={currentStepNumber > 1 ? handleBack : undefined}
          onNext={currentStep !== 'relationship' ? handleNext : undefined}
          onStart={currentStep === 'relationship' ? handleStart : undefined}
        />
      </div>
    </aside>
  );
}
