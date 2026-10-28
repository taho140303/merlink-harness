import { useState, useMemo, useEffect, useRef } from 'react';
import { MessageSquareQuote } from 'lucide-react';
import { DraftHeader } from './DraftHeader';
import { DraftOverviewBlock } from './DraftOverviewBlock';
import { DraftToolbar } from './DraftToolbar';
import { DraftLoading } from './DraftLoading';
import { DRAFT_OVERVIEW } from './mockOverview';
import { ConfirmDialog } from '../../../shared/components/ui/confirm-dialog';
import { useT } from '../../../shared/i18n/useT';
import { cn } from '../../../shared/utils/cn';
import type { DraftStep, CharacterSubStep, DraftOverview, FeedbackContext } from './types';

interface DraftWorkspaceProps {
  onClose: () => void;
  onFeedback: (feedback: FeedbackContext) => void;
  overview?: DraftOverview;
}

export function DraftWorkspace({ onClose, onFeedback, overview = DRAFT_OVERVIEW }: DraftWorkspaceProps) {
  const t = useT();
  const [currentStep, setCurrentStep] = useState<DraftStep>('overview');
  const [currentSubStep, setCurrentSubStep] = useState<CharacterSubStep>('appearance');
  const [currentVersion, setCurrentVersion] = useState(overview.version);
  const [confirmOpen, setConfirmOpen] = useState(false);
  
  // Feedback Mode State & Custom Cursor
  const [isFeedbackMode, setIsFeedbackMode] = useState(false);
  const [cursorPos, setCursorPos] = useState({ x: 0, y: 0 });
  const containerRef = useRef<HTMLElement>(null);

  useEffect(() => {
    if (!isFeedbackMode) return;
    const handleMouseMove = (e: MouseEvent) => {
      if (containerRef.current) {
        const rect = containerRef.current.getBoundingClientRect();
        setCursorPos({ x: e.clientX - rect.left, y: e.clientY - rect.top });
      }
    };
    window.addEventListener('mousemove', handleMouseMove);
    return () => window.removeEventListener('mousemove', handleMouseMove);
  }, [isFeedbackMode]);

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
    onClose();
  };

  const handleRegenerate = () => {
    triggerTransition(() => {
      // Logic regenerate data
    });
  };

  const handleAttachFeedback = (id: string, tag: string, title: string) => {
    onFeedback({ id, tag, title });
    setIsFeedbackMode(false); // Tắt mode sau khi chọn
  };

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
    <aside 
      ref={containerRef}
      onMouseLeave={() => setIsFeedbackMode(false)}
      className={cn(
        "flex h-full w-full flex-col overflow-hidden border-l border-slate-200 bg-slate-50/40 dark:border-slate-800 dark:bg-slate-950/40 relative",
        isFeedbackMode && "cursor-none [&_*]:cursor-none"
      )}
    >
      <DraftHeader
        currentStep={currentStep}
        currentSubStep={currentSubStep}
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
            isFeedbackMode={isFeedbackMode}
            onAttachFeedback={handleAttachFeedback}
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
            isFeedbackMode={isFeedbackMode}
            onToggleFeedback={() => setIsFeedbackMode(!isFeedbackMode)}
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

      {/* Custom Feedback Cursor */}
      {isFeedbackMode && (
        <div
          className="pointer-events-none absolute z-[9999] flex h-8 w-8 -translate-x-1/2 -translate-y-1/2 items-center justify-center rounded-full border-2 border-purple-500 bg-purple-500/20 shadow-[0_0_15px_rgba(168,85,247,0.5)] backdrop-blur-sm transition-transform duration-75 ease-out"
          style={{ left: cursorPos.x, top: cursorPos.y }}
        >
          <MessageSquareQuote className="h-4 w-4 text-purple-600 dark:text-purple-400" />
        </div>
      )}
    </aside>
  );
}
