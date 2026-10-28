import { useState } from "react";
import { Dialog, DialogContent, DialogTitle, DialogDescription } from "../../../shared/components/ui/dialog";
import { Button } from "../../../shared/components/ui/button";
import { useT } from "../../../shared/i18n/useT";

interface ProcessDetailDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
}

export function ProcessDetailDialog({ open, onOpenChange }: ProcessDetailDialogProps) {
  const t = useT();
  const [chunkSize, setChunkSize] = useState("1024");
  const [overlap, setOverlap] = useState("128");
  const [temperature, setTemperature] = useState("0.7");
  const [maxRetries, setMaxRetries] = useState("3");

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-2xl p-6">
        <div>
          <DialogTitle className="text-base font-semibold text-fg">{t("chat.processDetails.title")}</DialogTitle>
          <DialogDescription className="text-xs text-fg-muted">{t("chat.processDetails.description")}</DialogDescription>
        </div>

        <div className="mt-4 space-y-4 text-xs leading-relaxed text-fg-muted">
          <p className="rounded-xl border border-line bg-surface-muted p-3.5">
            {t("chat.processDetails.chunkBefore")}
            <input
              type="text"
              value={chunkSize}
              onChange={(e) => setChunkSize(e.target.value)}
              className="tier-input focus-ring mx-1.5 inline-block w-16 rounded-md px-2 py-0.5 text-center text-xs font-semibold text-fg"
            />
            {t("chat.processDetails.chunkMid")}
            <input
              type="text"
              value={overlap}
              onChange={(e) => setOverlap(e.target.value)}
              className="tier-input focus-ring mx-1.5 inline-block w-14 rounded-md px-2 py-0.5 text-center text-xs font-semibold text-fg"
            />
            {t("chat.processDetails.chunkAfter")}
          </p>

          <p className="rounded-xl border border-line bg-surface-muted p-3.5">
            {t("chat.processDetails.samplingBefore")}
            <input
              type="text"
              value={temperature}
              onChange={(e) => setTemperature(e.target.value)}
              className="tier-input focus-ring mx-1.5 inline-block w-14 rounded-md px-2 py-0.5 text-center text-xs font-semibold text-fg"
            />
            {t("chat.processDetails.samplingMid")}
            <input
              type="text"
              value={maxRetries}
              onChange={(e) => setMaxRetries(e.target.value)}
              className="tier-input focus-ring mx-1.5 inline-block w-12 rounded-md px-2 py-0.5 text-center text-xs font-semibold text-fg"
            />
            {t("chat.processDetails.samplingAfter")}
          </p>
        </div>

        <div className="flex justify-end pt-2">
          <Button type="button" variant="primary" onClick={() => onOpenChange(false)}>{t("chat.close")}</Button>
        </div>
      </DialogContent>
    </Dialog>
  );
}
