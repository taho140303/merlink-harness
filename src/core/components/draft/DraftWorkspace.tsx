import { useState, useMemo, useEffect } from 'react';
import { DraftHeader } from './DraftHeader';
import { DraftOverviewBlock } from './DraftOverviewBlock';
import { DraftToolbar } from './DraftToolbar';
import { DraftLoading } from './DraftLoading';
import { DRAFT_OVERVIEW } from './mockOverview';
import { ConfirmDialog } from '../../../shared/components/ui/confirm-dialog';
import { toast } from '../../../shared/components/ui/toast';
import { useT } from '../../../shared/i18n/useT';
import { cn } from '../../../shared/utils/cn';
import type { DraftStep, CharacterSubStep, DraftOverview } from './types';

interface DraftWorkspaceProps {
  onClose: () => void;
  overview?: DraftOverview;
}

const STEP_ORDER: DraftStep[] = ['overview', 'chapter', 'world', 'scene', 'character', 'relationship'];

export function DraftWorkspace({ onClose, overview = DRAFT_OVERVIEW }: DraftWorkspaceProps) {
  const t = useT();
  const [currentStep, setCurrentStep] = useState<DraftStep>('overview');
  const [currentSubStep, setCurrentSubStep] = useState<CharacterSubStep>('appearance');
  const [currentVersion, setCurrentVersion] = useState(overview.version);
  
  const [confirmOpen, setConfirmOpen] = useState(false);

  // Transition states
  const [transitionPhase, setTransitionPhase] = useState<'idle' | 'fade-out' | 'loading' | 'fade-in'>('idle');
  const [loadingIdx, setLoadingIdx] = useState(0);

  const loadingMessages = [
    t('draft.loading.msg1'),
    t('draft.loading.msg2'),
    t('draft.loading.msg3'),
    t('draft.loading.msg4'),
  ];

  useEffect(() => {
    if (transitionPhase !== 'loading') return;
    const interval = setInterval(() => {
      setLoadingIdx(prev => (prev + 1) % loadingMessages.length);
    }, 3000);
    return () => clearInterval(interval);
  }, [transitionPhase, loadingMessages.length]);

  const triggerTransition = (action: () => void) => {
    if (transitionPhase !== 'idle') return;
    setTransitionPhase('fade-out');
    
    setTimeout(() => {
      action();
      setLoadingIdx(0); 
      setTransitionPhase('loading');
      
      setTimeout(() => {
        setTransitionPhase('fade-in');
        setTimeout(() => setTransitionPhase('idle'), 600);
      }, 3000 * loadingMessages.length);
    }, 600);
  };

  const executeNext = () => {
    setConfirmOpen(false);
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

  const handleStart = () => {
    toast({ title: t('draft.toast.started'), variant: 'success' });
    onClose();
  };

  const handleRegenerate = () => {
    triggerTransition(() => {
      toast({ title: 'Regenerated data for this phase.', variant: 'success' });
    });
  };

  const currentStepNumber = STEP_ORDER.indexOf(currentStep) + 1;

  const phaseOverview = useMemo(() => {
    if (currentStep === 'overview') {
      return {
        ...overview,
        arcs: overview.arcs.map(arc => ({
          ...arc,
          chapters: undefined, 
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
        {transitionPhase === 'loading' && (
          <DraftLoading message={loadingMessages[loadingIdx]} />
        )}

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
          />
        </div>

        {/* Toolbar */}
        <div className={cn(
          "absolute bottom-6 left-1/2 -translate-x-1/2 z-20 transition-all duration-600",
          isFading ? "opacity-0 translate-y-4 pointer-events-none" : "opacity-100 translate-y-0"
        )}>
          <DraftToolbar
            currentVersion={currentVersion}
            versionHistory={overview.versionHistory || []}
            onVersionChange={setCurrentVersion}
            onRegenerate={handleRegenerate}
            onNext={currentStep !== 'relationship' ? () => setConfirmOpen(true) : undefined}
            onStart={currentStep === 'relationship' ? handleStart : undefined}
          />
        </div>
      </div>

      <ConfirmDialog
        open={confirmOpen}
        onOpenChange={setConfirmOpen}
        title={t('draft.dialog.nextTitle')}
        description={t('draft.dialog.nextDesc')}
        confirmLabel={t('draft.dialog.confirmNext')}
        cancelLabel={t('common.cancel')}
        onConfirm={executeNext}
      />
    </aside>
  );
}
