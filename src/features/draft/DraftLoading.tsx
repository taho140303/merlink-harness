import { useT } from '@/shared/i18n/useT';

interface DraftLoadingProps {
  message: string;
}

export function DraftLoading({ message }: DraftLoadingProps) {
  const t = useT();
  
  return (
    <div className="absolute inset-0 z-40 flex flex-col items-center justify-center bg-surface-muted/80  backdrop-blur-md   duration-300">
      <div className="relative mb-6 flex items-center justify-center">
        <img src="/logo.svg" alt="Loading" className="h-24 w-24 object-contain animate-logo-float" />
      </div>
      <h3 className="mb-2 text-xl font-bold text-fg">
        {t('draft.loading.title')}
      </h3>
      <p className="h-6 text-base font-medium text-indigo-600 dark:text-indigo-400 animate-pulse">
        {message}
      </p>
    </div>
  );
}
