import { MessageSquareQuote, ArrowRight, ArrowLeft, Play } from 'lucide-react';

interface PlaceholderStepProps {
  title: string;
  description: string;
  onFeedback?: () => void;
  onNext?: () => void;
  onBack?: () => void;
  onStart?: () => void;
}

export function PlaceholderStep({ title, description, onFeedback, onNext, onBack, onStart }: PlaceholderStepProps) {
  return (
    <div className="flex min-h-[50vh] flex-col items-center justify-center rounded-2xl border border-dashed border-slate-300 bg-white/50 p-8 text-center dark:border-slate-700 dark:bg-slate-900/50">
      <h3 className="text-lg font-bold text-slate-900 dark:text-slate-100">{title}</h3>
      <p className="mt-2 max-w-md text-sm text-slate-500 dark:text-slate-400">{description}</p>
      
      {onFeedback && (
        <button onClick={onFeedback} className="mt-4 flex items-center gap-1.5 text-xs font-medium text-blue-600 hover:underline dark:text-blue-400">
          <MessageSquareQuote className="h-4 w-4" /> Góp ý cho phần này
        </button>
      )}

      <div className="fixed bottom-6 right-6 z-20 flex items-center gap-2.5 animate-in fade-in-0 duration-300">
        {onBack && (
          <button onClick={onBack} className="flex items-center gap-1.5 rounded-full border border-slate-200 bg-white px-4 py-2.5 text-xs font-semibold text-slate-700 shadow-lg hover:bg-slate-50 dark:border-slate-800 dark:bg-slate-900 dark:text-slate-300 dark:hover:bg-slate-800 transition-all">
            <ArrowLeft className="h-4 w-4" /> Back
          </button>
        )}
        {onNext && (
          <button onClick={onNext} className="flex items-center gap-2 rounded-full bg-slate-900 px-5 py-2.5 text-xs font-semibold text-white shadow-lg shadow-slate-900/20 transition-all hover:scale-105 hover:bg-slate-800 dark:bg-slate-100 dark:text-slate-900 dark:hover:bg-slate-200">
            <span>Next Step</span>
            <ArrowRight className="h-4 w-4" />
          </button>
        )}
        {onStart && (
          <button onClick={onStart} className="flex items-center gap-2 rounded-full bg-blue-600 px-6 py-2.5 text-xs font-semibold text-white shadow-lg shadow-blue-500/25 transition-all hover:scale-105 hover:bg-blue-500">
            <Play className="h-4 w-4 fill-current" />
            <span>Start Generation</span>
          </button>
        )}
      </div>
    </div>
  );
}
