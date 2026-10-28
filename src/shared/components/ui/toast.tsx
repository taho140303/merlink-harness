import { create } from 'zustand';
import { X } from 'lucide-react';
import { cn } from '../../utils/cn';
import { useT } from '../../i18n/useT';

export interface ToastItem {
  id: string;
  title: string;
  description?: string;
  variant?: 'default' | 'success' | 'error';
}

interface ToastState {
  toasts: ToastItem[];
  push: (t: Omit<ToastItem, 'id'>) => void;
  dismiss: (id: string) => void;
}

export const useToastStore = create<ToastState>((set) => ({
  toasts: [],
  push: (t) => {
    const id = `t-${Date.now()}-${Math.random().toString(36).slice(2, 8)}`;
    set((s) => ({ toasts: [...s.toasts, { id, ...t }] }));
    setTimeout(() => {
      set((s) => ({ toasts: s.toasts.filter((x) => x.id !== id) }));
    }, 3800);
  },
  dismiss: (id) => set((s) => ({ toasts: s.toasts.filter((x) => x.id !== id) })),
}));

export const toast = (t: Omit<ToastItem, 'id'>) => useToastStore.getState().push(t);

export function ToastViewport() {
  const toasts = useToastStore((s) => s.toasts);
  const dismiss = useToastStore((s) => s.dismiss);
  const tr = useT();

  return (
    <div className="pointer-events-none fixed bottom-6 right-6 z-[100] flex flex-col gap-3">
      {toasts.map((t) => (
        <div
          key={t.id}
          className={cn(
            'toast-item pointer-events-auto flex w-80 items-start gap-2 rounded-panel p-4 neu-flat',
            t.variant === 'error' ? 'text-rose-500' : t.variant === 'success' ? 'text-emerald-500' : 'text-fg'
          )}
        >
          <div className="min-w-0 flex-1">
            <p className="text-xs font-semibold">{t.title}</p>
            {t.description && (
              <p className="mt-0.5 text-xs opacity-80">{t.description}</p>
            )}
          </div>
          <button
            type="button"
            aria-label={tr('common.close')}
            onClick={() => dismiss(t.id)}
            className="focus-ring rounded-md p-1 opacity-60 hover:bg-surface-muted"
          >
            <X className="h-3.5 w-3.5" aria-hidden="true" />
          </button>
        </div>
      ))}
    </div>
  );
}
