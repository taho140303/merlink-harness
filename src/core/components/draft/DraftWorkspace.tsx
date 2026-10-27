import { useState, useMemo } from 'react';
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
    toast({ title: t('draft.toast.started'), variant: 'success' });
    onClose();
  };

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

  // Logic "phình to ra": Ở phase overview thì không có chapters, từ phase chapter trở đi mới có
  const phaseOverview = useMemo(() => {
    if (currentStep === 'overview') {
      return {
        ...overview,
        arcs: overview.arcs.map(arc => ({
          ...arc,
          chapters: undefined, // Ẩn chapters ở phase 1
        })),
      };
    }
    return overview;
  }, [overview, currentStep]);

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

      <div className="relative flex-1 overflow-hidden">
        <DraftOverviewBlock
          overview={phaseOverview}
          onFeedback={onFeedback}
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
