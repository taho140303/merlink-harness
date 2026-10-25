import { useState } from "react";
import { Dialog, DialogContent } from "../../../shared/components/ui/dialog";

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
          <h3 className="text-base font-semibold text-slate-900 dark:text-slate-100">
            Process Execution Details
          </h3>
          <p className="text-xs text-slate-500 dark:text-slate-400">
            Chi tiết tiến trình pipeline và các tham số nội suy trực tiếp.
          </p>
        </div>

        <div className="mt-4 space-y-4 text-xs leading-relaxed text-slate-600 dark:text-slate-300">
          <p className="rounded-xl border border-slate-100 bg-slate-50/70 p-3.5 dark:border-slate-800/80 dark:bg-slate-950/40">
            Hệ thống phân tách ngữ cảnh văn bản thành các khối chunk có độ dài tối đa là
            <input
              type="text"
              value={chunkSize}
              onChange={(e) => setChunkSize(e.target.value)}
              className="mx-1.5 inline-block w-16 rounded-md border border-slate-200 bg-white px-2 py-0.5 text-center text-xs font-semibold text-slate-900 focus:ring-1 focus:ring-slate-400 dark:border-slate-700 dark:bg-slate-800 dark:text-slate-100 focus:outline-hidden"
            />
            tokens, đồng thời áp dụng khoảng đệm chồng lấn là
            <input
              type="text"
              value={overlap}
              onChange={(e) => setOverlap(e.target.value)}
              className="mx-1.5 inline-block w-14 rounded-md border border-slate-200 bg-white px-2 py-0.5 text-center text-xs font-semibold text-slate-900 focus:ring-1 focus:ring-slate-400 dark:border-slate-700 dark:bg-slate-800 dark:text-slate-100 focus:outline-hidden"
            />
            tokens để đảm bảo tính liên kết dữ liệu giữa các luồng.
          </p>

          <p className="rounded-xl border border-slate-100 bg-slate-50/70 p-3.5 dark:border-slate-800/80 dark:bg-slate-950/40">
            Mức độ biến thiên ngẫu nhiên (Sampling Temperature) được gán ngưỡng
            <input
              type="text"
              value={temperature}
              onChange={(e) => setTemperature(e.target.value)}
              className="mx-1.5 inline-block w-14 rounded-md border border-slate-200 bg-white px-2 py-0.5 text-center text-xs font-semibold text-slate-900 focus:ring-1 focus:ring-slate-400 dark:border-slate-700 dark:bg-slate-800 dark:text-slate-100 focus:outline-hidden"
            />
            nhằm cân bằng giữa độ chính xác và sáng tạo. Khi gateway phát sinh độ trễ quá ngưỡng, tiến trình kích hoạt tối đa
            <input
              type="text"
              value={maxRetries}
              onChange={(e) => setMaxRetries(e.target.value)}
              className="mx-1.5 inline-block w-12 rounded-md border border-slate-200 bg-white px-2 py-0.5 text-center text-xs font-semibold text-slate-900 focus:ring-1 focus:ring-slate-400 dark:border-slate-700 dark:bg-slate-800 dark:text-slate-100 focus:outline-hidden"
            />
            lần thử lại (exponential backoff) trước khi timeout.
          </p>
        </div>

        <div className="flex justify-end pt-2">
          <button
            type="button"
            onClick={() => onOpenChange(false)}
            className="rounded-xl bg-slate-900 px-4 py-2 text-xs font-medium text-white shadow-xs hover:bg-slate-800 dark:bg-slate-100 dark:text-slate-900 dark:hover:bg-slate-200"
          >
            Đóng
          </button>
        </div>
      </DialogContent>
    </Dialog>
  );
}
