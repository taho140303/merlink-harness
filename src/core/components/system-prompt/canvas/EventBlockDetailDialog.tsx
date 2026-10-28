import { Dialog, DialogContent, DialogDescription, DialogTitle } from '../../../../shared/components/ui/dialog';
import { useT } from '../../../../shared/i18n/useT';
import type { Component, EmotionalRange, EventBlock, EventTransition, StateItem } from '../types';

interface EventBlockDetailDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  eventBlock: EventBlock | null;
  component?: Component;
  state?: StateItem;
  emotions: EmotionalRange[];
  outgoingTransitions: { transition: EventTransition; target?: EventBlock }[];
}

export function EventBlockDetailDialog({
  open,
  onOpenChange,
  eventBlock,
  component,
  state,
  emotions,
  outgoingTransitions,
}: EventBlockDetailDialogProps) {
  const t = useT();

  if (!eventBlock) return null;

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-lg p-6">
        <div>
          <p className="font-mono text-xs text-fg-muted">
            {eventBlock.code}
          </p>
          <DialogTitle className="mt-0.5 text-base font-semibold text-fg">
            {t('sp.canvas.detail.title')}
          </DialogTitle>
          <DialogDescription className="sr-only">{eventBlock.code}</DialogDescription>
        </div>

        <div className="mt-4 space-y-3">
          <div className="rounded-lg border border-indigo-100 bg-indigo-50/60 p-3 dark:border-indigo-900/60 dark:bg-indigo-950/30">
            <p className="text-xs font-semibold uppercase tracking-wide text-indigo-700 dark:text-indigo-300">
              {t('sp.eventBlock.component')}
            </p>
            <p className="mt-1 text-sm font-semibold text-fg">
              {component?.name}
            </p>
            <p className="mt-0.5 text-xs text-fg-muted">
              {component?.description}
            </p>
          </div>

          <div className="rounded-lg border border-emerald-100 bg-emerald-50/60 p-3 dark:border-emerald-900/60 dark:bg-emerald-950/30">
            <p className="text-xs font-semibold uppercase tracking-wide text-emerald-700 dark:text-emerald-300">
              {t('sp.eventBlock.state')}
            </p>
            <p className="mt-1 text-sm font-semibold text-fg">
              {state?.name}
            </p>
            <p className="mt-0.5 text-xs text-fg-muted">
              {state?.description}
            </p>
          </div>

          <div>
            <p className="mb-1.5 text-xs font-semibold uppercase tracking-wide text-fg-muted">
              {t('sp.eventBlock.emotionalRanges')}
            </p>
            {emotions.length === 0 ? (
              <p className="text-xs text-fg-subtle">{t('sp.eventBlock.noEmotionals')}</p>
            ) : (
              <div className="flex flex-wrap gap-1.5">
                {emotions.map((e) => (
                  <span
                    key={e.id}
                    className="rounded-full bg-amber-100 px-2 py-0.5 text-xs font-medium text-amber-800 dark:bg-amber-950/60 dark:text-amber-300"
                  >
                    {e.name}
                  </span>
                ))}
              </div>
            )}
          </div>

          <div>
            <p className="mb-1.5 text-xs font-semibold uppercase tracking-wide text-fg-muted">
              {t('sp.eventBlock.outgoingTransitions')}
            </p>
            {outgoingTransitions.length === 0 ? (
              <p className="text-xs text-fg-subtle">{t('sp.eventBlock.noTransitions')}</p>
            ) : (
              <ul className="space-y-1">
                {outgoingTransitions.map(({ transition, target }) => (
                  <li
                    key={transition.id}
                    className="flex items-center justify-between gap-2 rounded-md border border-line bg-surface-muted px-2.5 py-1.5 text-xs  "
                  >
                    <span className="font-mono font-semibold text-fg-muted">
                      {target?.code}
                    </span>
                    <span className="text-fg-muted">
                      {t('sp.eventTransitions.priority')}: {transition.priority}
                    </span>
                  </li>
                ))}
              </ul>
            )}
          </div>
        </div>

        <div className="mt-5 flex justify-end">
          <button
            type="button"
            onClick={() => onOpenChange(false)}
            className="rounded-xl bg-accent px-4 py-2 text-xs font-semibold text-white hover:opacity-90   "
          >
            {t('common.close')}
          </button>
        </div>
      </DialogContent>
    </Dialog>
  );
}
