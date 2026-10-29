import {
  Dialog,
  DialogContent,
  DialogTitle,
  DialogDescription,
} from '@/shared/components/ui/dialog';
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

  const handleCancel = () => {
    onOpenChange(false);
  };

  const handleConfirm = () => {
    onConfirm();
    onOpenChange(false);
  };

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-lg p-6">
        <div>
          <DialogTitle className="text-base font-semibold text-fg">
            {t('chat.preDraft.title')}
          </DialogTitle>
          <DialogDescription className="mt-2 text-sm leading-relaxed text-fg-muted">
            {t('chat.preDraft.description')}
          </DialogDescription>
        </div>

        <div className="mt-6 flex items-center justify-between gap-3">
          <Button type="button" variant="primary" onClick={handleCancel}>
            {t('common.cancel')}
          </Button>
          <Button type="button" variant="secondary" onClick={handleConfirm}>
            {t('common.confirm')}
          </Button>
        </div>
      </DialogContent>
    </Dialog>
  );
}
