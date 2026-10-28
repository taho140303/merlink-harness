import { MessageSquareQuote } from 'lucide-react';
import { useT } from '../../../../shared/i18n/useT';

interface PlaceholderStepProps {
  title: string;
  description: string;
  onFeedback?: () => void;
}

export function PlaceholderStep({ title, description, onFeedback }: PlaceholderStepProps) {
  const t = useT();
  return (
    <div className="flex min-h-[50vh] flex-col items-center justify-center rounded-2xl border border-dashed border-line-strong bg-surface-card/50 p-8 text-center  ">
      <h3 className="text-lg font-bold text-fg">{title}</h3>
      <p className="mt-2 max-w-md text-base text-fg-muted">{description}</p>

      {onFeedback && (
        <button onClick={onFeedback} className="mt-4 flex items-center gap-1.5 text-sm font-medium text-blue-600 hover:underline dark:text-blue-400">
          <MessageSquareQuote className="h-4 w-4" /> {t('draft.overview.feedbackForSection')}
        </button>
      )}
    </div>
  );
}
