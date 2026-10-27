import { ArrowLeft, ArrowRight, Play } from 'lucide-react';
import { useT } from '../../../shared/i18n/useT';

interface DraftStepNavProps {
  onBack?: () => void;
  onNext?: () => void;
  onStart?: () => void;
}

export function DraftStepNav({ onBack, onNext, onStart }: DraftStepNavProps) {
  const t = useT();
  return (
    <div className="fixed bottom-6 right-6 z-20 flex items-center gap-2.5">
      {onBack && (
        <button
          type="button"
          onClick={onBack}
          className="flex items-center gap-1.5 rounded-full border border-slate-200 bg-white px-4 py-2.5 text-xs font-semibold text-slate-700 shadow-lg transition-all hover:bg-slate-50 dark:border-slate-800 dark:bg-slate-900 dark:text-slate-300 dark:hover:bg-slate-800"
        >
          <ArrowLeft className="h-4 w-4" /> {t('draft.nav.back')}
        </button>
      )}
      {onNext && (
        <button
          type="button"
          onClick={onNext}
          className="flex items-center gap-2 rounded-full bg-slate-900 px-5 py-2.5 text-xs font-semibold text-white shadow-lg shadow-slate-900/20 transition-all hover:scale-105 hover:bg-slate-800 dark:bg-slate-100 dark:text-slate-900 dark:hover:bg-slate-200"
        >
          <span>{t('draft.nav.next')}</span>
          <ArrowRight className="h-4 w-4" />
        </button>
      )}
      {onStart && (
        <button
          type="button"
          onClick={onStart}
          className="flex items-center gap-2 rounded-full bg-blue-600 px-6 py-2.5 text-xs font-semibold text-white shadow-lg shadow-blue-500/25 transition-all hover:scale-105 hover:bg-blue-500"
        >
          <Play className="h-4 w-4 fill-current" />
          <span>{t('draft.nav.start')}</span>
        </button>
      )}
    </div>
  );
}
