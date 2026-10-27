import { useState } from 'react';
import { DraftHeader } from './DraftHeader';
import { DraftOverviewBlock } from './DraftOverviewBlock';
import { DraftStepNav } from './DraftStepNav';
import { PlaceholderStep } from './steps/PlaceholderStep';
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

  const getStepProps = () => {
    switch (currentStep) {
      case 'chapter':
        return {
          title: t('draft.step.chapter'),
          description: 'Dàn ý các chương, cấu trúc Arc và mô tả nội dung từng chương.',
        };
      case 'world':
        return {
          title: t('draft.step.world'),
          description: 'Xây dựng bối cảnh thế giới, văn hóa, lịch sử và các địa điểm chính.',
        };
      case 'scene':
        return {
          title: t('draft.step.scene'),
          description: 'Cấu hình chi tiết các cảnh, nhân vật xuất hiện, POV và style preset.',
        };
      case 'character': {
        const subTitle =
          currentSubStep === 'appearance'
            ? t('draft.subStep.appearance')
            : currentSubStep === 'psychological'
              ? t('draft.subStep.psychological')
              : t('draft.subStep.lifeExperience');
        return {
          title: `5. ${t('draft.step.character')} - ${subTitle}`,
          description: `Chi tiết nhân vật: ${subTitle}.`,
        };
      }
      case 'relationship':
        return {
          title: t('draft.step.relationship'),
          description: 'Thiết lập mối quan hệ, tương tác và mức độ liên kết giữa các nhân vật.',
        };
      case 'overview':
      default:
        return { title: t('draft.step.overview'), description: '' };
    }
  };

  const stepProps = getStepProps();
  const handleStepFeedback = () =>
    onFeedback({ id: 'temp', tag: currentStep, title: stepProps.title });

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

        {currentStep !== 'overview' && (
          <div className="mt-5">
            <PlaceholderStep
              title={stepProps.title}
              description={stepProps.description}
              onFeedback={handleStepFeedback}
            />
          </div>
        )}

        <DraftStepNav
          onBack={currentStepNumber > 1 ? handleBack : undefined}
          onNext={currentStep !== 'relationship' ? handleNext : undefined}
          onStart={currentStep === 'relationship' ? handleStart : undefined}
        />
      </div>
    </aside>
  );
}
