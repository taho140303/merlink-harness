import { useState } from "react";
import { Dialog, DialogContent, DialogTitle, DialogDescription } from "../../../shared/components/ui/dialog";
import { Button } from "../../../shared/components/ui/button";

interface ProcessDetailDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
}

export function ProcessDetailDialog({ open, onOpenChange }: ProcessDetailDialogProps) {
  const [chunkSize, setChunkSize] = useState("1024");
  const [overlap, setOverlap] = useState("128");
  const [temperature, setTemperature] = useState("0.7");
  const [maxRetries, setMaxRetries] = useState("3");

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-2xl p-6">
        <div>
          <DialogTitle className="text-base font-semibold text-fg">Process Execution Details</DialogTitle>
          <DialogDescription className="text-xs text-fg-muted">Chi tiết tiến trình pipeline và các tham số nội suy trực tiếp.</DialogDescription>
        </div>

        <div className="mt-4 space-y-4 text-xs leading-relaxed text-fg-muted">
          <p className="rounded-xl border border-line bg-surface-muted p-3.5">
            Hệ thống phân tách ngữ cảnh văn bản thành các khối chunk có độ dài tối đa là
            <input
              type="text"
              value={chunkSize}
              onChange={(e) => setChunkSize(e.target.value)}
              className="tier-input focus-ring mx-1.5 inline-block w-16 rounded-md px-2 py-0.5 text-center text-xs font-semibold text-fg"
            />
            tokens, đồng thời áp dụng khoảng đệm chồng lấn là
            <input
              type="text"
              value={overlap}
              onChange={(e) => setOverlap(e.target.value)}
              className="tier-input focus-ring mx-1.5 inline-block w-14 rounded-md px-2 py-0.5 text-center text-xs font-semibold text-fg"
            />
            tokens để đảm bảo tính liên kết dữ liệu giữa các luồng.
          </p>

          <p className="rounded-xl border border-line bg-surface-muted p-3.5">
            Mức độ biến thiên ngẫu nhiên (Sampling Temperature) được gán ngưỡng
            <input
              type="text"
              value={temperature}
              onChange={(e) => setTemperature(e.target.value)}
              className="tier-input focus-ring mx-1.5 inline-block w-14 rounded-md px-2 py-0.5 text-center text-xs font-semibold text-fg"
            />
            nhằm cân bằng giữa độ chính xác và sáng tạo. Khi gateway phát sinh độ trễ quá ngưỡng, tiến trình kích hoạt tối đa
            <input
              type="text"
              value={maxRetries}
              onChange={(e) => setMaxRetries(e.target.value)}
              className="tier-input focus-ring mx-1.5 inline-block w-12 rounded-md px-2 py-0.5 text-center text-xs font-semibold text-fg"
            />
            lần thử lại (exponential backoff) trước khi timeout.
          </p>
        </div>

        <div className="flex justify-end pt-2">
          <Button type="button" variant="primary" onClick={() => onOpenChange(false)}>Đóng</Button>
        </div>
      </DialogContent>
    </Dialog>
  );
}
