import { useState, useMemo, useEffect } from 'react';
import { DraftHeader } from './DraftHeader';
import { DraftOverviewBlock } from './DraftOverviewBlock';
import { DraftStepNav } from './DraftStepNav';
import { DraftLoading } from './DraftLoading';
import { DRAFT_OVERVIEW } from './mockOverview';
import { toast } from '../../../shared/components/ui/toast';
import { useT } from '../../../shared/i18n/useT';
import { cn } from '../../../shared/utils/cn';
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
  
  // Transition states
  const [transitionPhase, setTransitionPhase] = useState<'idle' | 'fade-out' | 'loading' | 'fade-in'>('idle');
  const [loadingIdx, setLoadingIdx] = useState(0);

  const loadingMessages = [
    t('draft.loading.msg1'),
    t('draft.loading.msg2'),
    t('draft.loading.msg3'),
    t('draft.loading.msg4'),
  ];

  // Xử lý đổi text mỗi 500ms khi loading
  useEffect(() => {
    if (transitionPhase !== 'loading') return;
    const interval = setInterval(() => {
      setLoadingIdx(prev => (prev + 1) % loadingMessages.length);
    }, 500);
    return () => clearInterval(interval);
  }, [transitionPhase, loadingMessages.length]);

  const triggerTransition = (action: () => void) => {
    if (transitionPhase !== 'idle') return;
    setTransitionPhase('fade-out');
    
    // Fade out 0.6s
    setTimeout(() => {
      action();
      setLoadingIdx(0); // Reset index before entering loading phase
      setTransitionPhase('loading');
      
      // Loading 2s (4 messages x 500ms)
      setTimeout(() => {
        setTransitionPhase('fade-in');
        
        // Fade in 0.6s
        setTimeout(() => {
          setTransitionPhase('idle');
        }, 600);
      }, 2000);
    }, 600);
  };

  const handleStart = () => {
    toast({ title: t('draft.toast.started'), variant: 'success' });
    onClose();
  };

  const handleNext = () => {
    triggerTransition(() => {
      if (currentStep === 'overview') setCurrentStep('chapter');
      else if (currentStep === 'chapter') setCurrentStep('world');
      else if (currentStep === 'world') setCurrentStep('scene');
      else if (currentStep === 'scene') setCurrentStep('character');
      else if (currentStep === 'character') {
        if (currentSubStep === 'appearance') setCurrentSubStep('psychological');
        else if (currentSubStep === 'psychological') setCurrentSubStep('lifeExperience');
        else if (currentSubStep === 'lifeExperience') setCurrentStep('relationship');
      }
    });
  };

  const handleBack = () => {
    triggerTransition(() => {
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
    });
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

  const isFading = transitionPhase === 'fade-out' || transitionPhase === 'loading';

  return (
    <aside className="flex h-full w-full flex-col overflow-hidden border-l border-slate-200 bg-slate-50/40 dark:border-slate-800 dark:bg-slate-950/40">
      <DraftHeader
        currentStep={currentStep}
        currentSubStep={currentSubStep}
        onStepClick={(step) => {
          if (step === currentStep) return;
          triggerTransition(() => {
            setCurrentStep(step);
            if (step === 'character') setCurrentSubStep('appearance');
          });
        }}
        onSubStepClick={setCurrentSubStep}
        onClose={onClose}
      />

      <div className="relative flex-1 overflow-hidden">
        {/* Loading Overlay */}
        {transitionPhase === 'loading' && (
          <DraftLoading message={loadingMessages[loadingIdx]} />
        )}

        {/* Main Content Area */}
        <div className={cn(
          "h-full w-full transition-all duration-600",
          transitionPhase === 'fade-out' && "opacity-0 scale-[0.98]",
          transitionPhase === 'loading' && "hidden",
          transitionPhase === 'fade-in' && "opacity-100 scale-100 animate-in fade-in duration-600",
          transitionPhase === 'idle' && "opacity-100 scale-100"
        )}>
          <DraftOverviewBlock
            overview={phaseOverview}
            currentStep={currentStep}
            onFeedback={onFeedback}
          />
        </div>

        {/* Navigation Buttons */}
        <div className={cn(
          "absolute bottom-6 right-6 z-20 transition-opacity duration-600",
          isFading ? "opacity-0 pointer-events-none" : "opacity-100"
        )}>
          <DraftStepNav
            onBack={currentStepNumber > 1 ? handleBack : undefined}
            onNext={currentStep !== 'relationship' ? handleNext : undefined}
            onStart={currentStep === 'relationship' ? handleStart : undefined}
          />
        </div>
      </div>
    </aside>
  );
}
