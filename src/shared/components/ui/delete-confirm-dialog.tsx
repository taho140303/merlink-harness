import { useEffect, useId, useState, type FormEvent } from 'react';
import { Trash2 } from 'lucide-react';
import { Dialog, DialogContent, DialogDescription, DialogTitle } from './dialog';
import { Button } from './button';
import { useT } from '../../i18n/useT';

interface DeleteConfirmDialogProps {
  open: boolean;
  onOpenChange: (o: boolean) => void;
  title: string;
  description: string;
  confirmLabel?: string;
  cancelLabel?: string;
  errorMessage?: string;
  onConfirm: () => void | Promise<void>;
}

const CODE_CHARS = 'ABCDEFGHJKLMNPQRSTUVWXYZ23456789';

function generateCode(len = 6): string {
  let s = '';
  for (let i = 0; i < len; i++) {
    s += CODE_CHARS[Math.floor(Math.random() * CODE_CHARS.length)];
  }
  return s;
}

export function DeleteConfirmDialog({
  open,
  onOpenChange,
  title,
  description,
  confirmLabel = 'Delete',
  cancelLabel = 'Cancel',
  errorMessage,
  onConfirm,
}: DeleteConfirmDialogProps) {
  const t = useT();
  const inputId = useId();
  const errorId = `${inputId}-error`;
  const [code, setCode] = useState<string>(() => generateCode());
  const [input, setInput] = useState('');
  const [submitting, setSubmitting] = useState(false);
  const [submitError, setSubmitError] = useState(false);

  useEffect(() => {
    if (open) {
      // eslint-disable-next-line react-hooks/set-state-in-effect
      setCode(generateCode());
      setInput('');
      setSubmitError(false);
    }
  }, [open]);

  const matches = input === code;
  const showMismatch = input.length === code.length && !matches;

  const handleSubmit = async (e: FormEvent) => {
    e.preventDefault();
    if (!matches || submitting) return;
    setSubmitting(true);
    setSubmitError(false);
    try {
      await onConfirm();
      onOpenChange(false);
    } catch {
      setSubmitError(true);
    } finally {
      setSubmitting(false);
    }
  };

  const handleOpenChange = (o: boolean) => {
    if (!o) setInput('');
    onOpenChange(o);
  };

  return (
    <Dialog open={open} onOpenChange={handleOpenChange}>
      <DialogContent className="max-w-md p-6">
        <form onSubmit={handleSubmit} className="space-y-4">
          <div className="flex items-start gap-3">
            <div className="flex h-10 w-10 shrink-0 items-center justify-center rounded-xl bg-rose-100 text-rose-600 dark:bg-rose-950/60 dark:text-rose-300">
              <Trash2 className="h-5 w-5" aria-hidden="true" />
            </div>
            <div className="min-w-0">
              <DialogTitle className="text-base">{title}</DialogTitle>
              <DialogDescription className="mt-1 text-xs">{description}</DialogDescription>
            </div>
          </div>

          <div>
            <label htmlFor={inputId} className="mb-1.5 block text-xs font-medium text-fg-muted">
              {t('delete.codeLabel')}
            </label>
            <div className="my-2 flex items-center justify-center gap-1.5">
              {code.split('').map((c, i) => (
                <span
                  key={i}
                  aria-hidden="true"
                  className="tier-input flex h-10 w-9 select-none items-center justify-center rounded-md font-mono text-base font-bold"
                >
                  {c}
                </span>
              ))}
            </div>
            <input
              id={inputId}
              type="text"
              value={input}
              onChange={(e) => setInput(e.target.value.toUpperCase())}
              onPaste={(e) => e.preventDefault()}
              onCopy={(e) => e.preventDefault()}
              onCut={(e) => e.preventDefault()}
              onDrop={(e) => e.preventDefault()}
              onContextMenu={(e) => e.preventDefault()}
              autoComplete="off"
              spellCheck={false}
              maxLength={code.length}
              placeholder={t('delete.codePlaceholder')}
              aria-invalid={showMismatch || undefined}
              aria-describedby={showMismatch ? errorId : undefined}
              className="tier-input focus-ring w-full rounded-xl px-3 py-2 text-center font-mono text-sm uppercase tracking-[0.4em] text-fg"
            />
            {showMismatch ? (
              <p id={errorId} className="mt-1 text-xs font-medium text-rose-500">
                {t('delete.codeMismatch')}
              </p>
            ) : null}
            {submitError && <p role="alert" className="mt-1 text-xs font-medium text-rose-500">{errorMessage || t('delete.failed')}</p>}
          </div>

          <div className="flex justify-end gap-2 pt-1">
            <Button variant="ghost" size="sm" onClick={() => handleOpenChange(false)}>
              {cancelLabel}
            </Button>
            <Button type="submit" variant="danger" size="sm" disabled={!matches || submitting}>
              {confirmLabel}
            </Button>
          </div>
        </form>
      </DialogContent>
    </Dialog>
  );
}
