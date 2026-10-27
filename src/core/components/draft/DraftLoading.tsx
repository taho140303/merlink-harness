import { Sparkles } from 'lucide-react';
import { useT } from '../../../shared/i18n/useT';

interface DraftLoadingProps {
  message: string;
}

export function DraftLoading({ message }: DraftLoadingProps) {
  const t = useT();
  
  return (
    <div className="absolute inset-0 z-40 flex flex-col items-center justify-center bg-slate-50/80 dark:bg-slate-950/80 backdrop-blur-md animate-in fade-in duration-300">
      <div className="relative mb-6 flex h-16 w-16 items-center justify-center">
        <div className="absolute inset-0 rounded-full border-4 border-indigo-100 dark:border-indigo-900/50" />
        <div className="absolute inset-0 animate-spin rounded-full border-4 border-indigo-600 border-t-transparent dark:border-indigo-400 dark:border-t-transparent" />
        <Sparkles className="h-6 w-6 text-indigo-600 dark:text-indigo-400 animate-pulse" />
      </div>
      <h3 className="mb-2 text-lg font-bold text-slate-900 dark:text-slate-100">
        {t('draft.loading.title')}
      </h3>
      <p className="h-6 text-sm font-medium text-indigo-600 dark:text-indigo-400 animate-pulse">
        {message}
      </p>
    </div>
  );
}
