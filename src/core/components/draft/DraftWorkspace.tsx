import { useState } from 'react';
import { DraftHeader } from './DraftHeader';
import { OverviewStep } from './steps/OverviewStep';
import { ArcsStep } from './steps/ArcsStep';
import { ChapterStep } from './steps/ChapterStep';
import { SceneStep } from './steps/SceneStep';
import { ConfigStep } from './steps/ConfigStep';
import { WorldStep } from './steps/WorldStep';
import { CharacterStep } from './steps/CharacterStep';
import { CharacterDetailStep } from './steps/CharacterDetailStep';
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
        {currentStep === 'chapter' && (
          <ChapterStep onFeedback={onFeedback} onNext={() => setCurrentStep('scene')} onBack={() => setCurrentStep('arcs')} />
        )}
        {currentStep === 'scene' && (
          <SceneStep onFeedback={onFeedback} onNext={() => setCurrentStep('config')} onBack={() => setCurrentStep('chapter')} />
        )}
        {currentStep === 'config' && (
          <ConfigStep onFeedback={onFeedback} onNext={() => setCurrentStep('world')} onBack={() => setCurrentStep('scene')} />
        )}
        {currentStep === 'world' && (
          <WorldStep onFeedback={onFeedback} onNext={() => setCurrentStep('character')} onBack={() => setCurrentStep('config')} />
        )}
        {currentStep === 'character' && (
          <CharacterStep onFeedback={onFeedback} onNext={() => setCurrentStep('characterDetail')} onBack={() => setCurrentStep('world')} />
        )}
        {currentStep === 'characterDetail' && (
          <CharacterDetailStep onFeedback={onFeedback} onStart={handleStart} onBack={() => setCurrentStep('character')} />
        )}
      </div>
    </aside>
  );
}
