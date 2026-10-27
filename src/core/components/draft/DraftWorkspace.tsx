import { useState } from 'react';
import { DraftHeader } from './DraftHeader';
import { PlaceholderStep } from './steps/PlaceholderStep';
import { toast } from '../../../shared/components/ui/toast';
import { useT } from '../../../shared/i18n/useT';
import type { DraftStep, CharacterSubStep, FeedbackContext } from './types';

interface DraftWorkspaceProps {
  onClose: () => void;
  onFeedback: (feedback: FeedbackContext) => void;
}

export function DraftWorkspace({ onClose, onFeedback }: DraftWorkspaceProps) {
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
    if (currentStep === 'chapter') setCurrentStep('overview');
    else if (currentStep === 'world') setCurrentStep('chapter');
    else if (currentStep === 'scene') setCurrentStep('world');
    else if (currentStep === 'character') setCurrentStep('scene');
    else if (currentStep === 'relationship') {
      setCurrentStep('character');
      setCurrentSubStep('lifeExperience');
    } else if (currentStep === 'character') {
      if (currentSubStep === 'lifeExperience') setCurrentSubStep('psychological');
      else if (currentSubStep === 'psychological') setCurrentSubStep('appearance');
      else setCurrentStep('scene');
    }
  };

  const getStepProps = () => {
    const base = {
      title: '',
      description: '',
      onFeedback: () => onFeedback({ id: 'temp', tag: currentStep, title: 'Góp ý' }),
      onBack: currentStep !== 'overview' ? handleBack : undefined,
      onNext: currentStep !== 'relationship' ? handleNext : undefined,
      onStart: currentStep === 'relationship' ? handleStart : undefined,
    };

    switch (currentStep) {
      case 'overview':
        return { ...base, title: t('draft.step.overview'), description: 'Thông tin tổng quan về dự án, tóm tắt cốt truyện và mục tiêu.' };
      case 'chapter':
        return { ...base, title: t('draft.step.chapter'), description: 'Dàn ý các chương, cấu trúc Arc và mô tả nội dung từng chương.' };
      case 'world':
        return { ...base, title: t('draft.step.world'), description: 'Xây dựng bối cảnh thế giới, văn hóa, lịch sử và các địa điểm chính.' };
      case 'scene':
        return { ...base, title: t('draft.step.scene'), description: 'Cấu hình chi tiết các cảnh, nhân vật xuất hiện, POV và style preset.' };
      case 'character':
        const subTitle = currentSubStep === 'appearance' ? t('draft.subStep.appearance') : currentSubStep === 'psychological' ? t('draft.subStep.psychological') : t('draft.subStep.lifeExperience');
        return { ...base, title: `5. ${t('draft.step.character')} - ${subTitle}`, description: `Chi tiết nhân vật: ${subTitle}.` };
      case 'relationship':
        return { ...base, title: t('draft.step.relationship'), description: 'Thiết lập mối quan hệ, tương tác và mức độ liên kết giữa các nhân vật.' };
      default:
        return base;
    }
  };

  return (
    <aside className="flex h-full w-full flex-col border-l border-slate-200 bg-slate-50/40 dark:border-slate-800 dark:bg-slate-950/40 overflow-hidden">
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
        <PlaceholderStep {...getStepProps()} />
      </div>
    </aside>
  );
}
