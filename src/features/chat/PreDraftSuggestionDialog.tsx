import { useState, useEffect } from 'react';
import { Dialog, DialogContent, DialogTitle, DialogDescription } from '@/shared/components/ui/dialog';
import { Button } from '@/shared/components/ui/button';
import { useT } from '@/shared/i18n/useT';

export interface PreDraftSuggestionDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  onConfirm: (suggestion?: string) => void;
}

export function PreDraftSuggestionDialog({
  open,
  onOpenChange,
  onConfirm,
}: PreDraftSuggestionDialogProps) {
  const t = useT();
  const [suggestion, setSuggestion] = useState('');

  useEffect(() => {
    if (!open) {
      setSuggestion('');
    }
  }, [open]);

  const handleSkip = () => {
    onConfirm();
    onOpenChange(false);
  };

  const handleConfirm = () => {
    const trimmed = suggestion.trim();
    onConfirm(trimmed || undefined);
    onOpenChange(false);
  };

  const handleCancel = () => {
    onOpenChange(false);
  };

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-lg p-6">
        <div>
          <DialogTitle className="text-base font-semibold text-fg">
            {t('chat.preDraft.title')}
          </DialogTitle>
          <DialogDescription className="mt-1.5 text-xs leading-relaxed text-fg-muted">
            {t('chat.preDraft.description')}
          </DialogDescription>
        </div>

        <div className="mt-3">
          <textarea
            value={suggestion}
            onChange={(e) => setSuggestion(e.target.value)}
            placeholder={t('chat.preDraft.placeholder')}
            rows={4}
            className="focus-ring w-full resize-none rounded-xl border border-line bg-surface-muted/60 p-3 text-xs leading-relaxed text-fg placeholder:text-fg-muted/60 focus:bg-surface focus:outline-none"
          />
        </div>

        <div className="mt-4 flex flex-wrap items-center justify-end gap-2">
          <Button type="button" variant="ghost" size="sm" onClick={handleCancel}>
            {t('common.cancel')}
          </Button>
          <Button type="button" variant="secondary" size="sm" onClick={handleSkip}>
            {t('chat.preDraft.skip')}
          </Button>
          <Button type="button" variant="primary" size="sm" onClick={handleConfirm}>
            {t('chat.preDraft.confirm')}
          </Button>
        </div>
      </DialogContent>
    </Dialog>
  );
}
