import { useEffect, useState, type FormEvent } from 'react';
import { Trash2 } from 'lucide-react';
import { Dialog, DialogContent } from './dialog';

interface DeleteConfirmDialogProps {
  open: boolean;
  onOpenChange: (o: boolean) => void;
  title: string;
  description: string;
  confirmLabel?: string;
  cancelLabel?: string;
  onConfirm: () => void;
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
  onConfirm,
}: DeleteConfirmDialogProps) {
  const [code, setCode] = useState<string>(() => generateCode());
  const [input, setInput] = useState('');

  useEffect(() => {
    if (open) {
      setCode(generateCode());
      setInput('');
    }
  }, [open]);

  const matches = input === code;
  const showMismatch = input.length === code.length && !matches;

  const handleSubmit = (e: FormEvent) => {
    e.preventDefault();
    if (!matches) return;
    onConfirm();
    onOpenChange(false);
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
              <Trash2 className="h-5 w-5" />
            </div>
            <div className="min-w-0">
              <h3 className="text-base font-semibold text-slate-900 dark:text-slate-100">
                {title}
              </h3>
              <p className="mt-1 text-xs text-slate-500 dark:text-slate-400">
                {description}
              </p>
            </div>
          </div>

          <div>
            <p className="mb-1.5 text-xs font-medium text-slate-600 dark:text-slate-300">
              Type the code below to confirm:
            </p>
            <div className="my-2 flex items-center justify-center gap-1.5">
              {code.split('').map((c, i) => (
                <span
                  key={i}
                  className="flex h-10 w-9 select-none items-center justify-center rounded-md border border-slate-200 bg-slate-50 font-mono text-base font-bold text-slate-800 dark:border-slate-700 dark:bg-slate-900 dark:text-slate-200"
                >
                  {c}
                </span>
              ))}
            </div>
            <input
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
              placeholder="Type the code here"
              className="w-full rounded-xl border border-slate-200 bg-white px-3 py-2 text-center font-mono text-sm uppercase tracking-[0.4em] text-slate-900 focus:outline-hidden focus:ring-2 focus:ring-rose-400 dark:border-slate-700 dark:bg-slate-950 dark:text-slate-100"
            />
            {showMismatch && (
              <p className="mt-1 text-[11px] font-medium text-rose-500">Code does not match</p>
            )}
          </div>

          <div className="flex justify-end gap-2 pt-1">
            <button
              type="button"
              onClick={() => handleOpenChange(false)}
              className="rounded-xl border border-slate-200 bg-white px-4 py-2 text-xs font-medium text-slate-700 hover:bg-slate-50 dark:border-slate-700 dark:bg-slate-900 dark:text-slate-300 dark:hover:bg-slate-800"
            >
              {cancelLabel}
            </button>
            <button
              type="submit"
              disabled={!matches}
              className="rounded-xl bg-rose-600 px-4 py-2 text-xs font-semibold text-white shadow-xs hover:bg-rose-500 disabled:cursor-not-allowed disabled:opacity-40"
            >
              {confirmLabel}
            </button>
          </div>
        </form>
      </DialogContent>
    </Dialog>
  );
}
