import { MessageSquareQuote } from 'lucide-react';

interface PlaceholderStepProps {
  title: string;
  description: string;
  onFeedback?: () => void;
}

export function PlaceholderStep({ title, description, onFeedback }: PlaceholderStepProps) {
  return (
    <div className="flex min-h-[50vh] flex-col items-center justify-center rounded-2xl border border-dashed border-slate-300 bg-white/50 p-8 text-center dark:border-slate-700 dark:bg-slate-900/50">
      <h3 className="text-lg font-bold text-slate-900 dark:text-slate-100">{title}</h3>
      <p className="mt-2 max-w-md text-sm text-slate-500 dark:text-slate-400">{description}</p>

      {onFeedback && (
        <button onClick={onFeedback} className="mt-4 flex items-center gap-1.5 text-xs font-medium text-blue-600 hover:underline dark:text-blue-400">
          <MessageSquareQuote className="h-4 w-4" /> Góp ý cho phần này
        </button>
      )}
    </div>
  );
}
