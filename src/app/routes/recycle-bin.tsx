import type { Route } from "./+types/recycle-bin";
import { Trash2 } from "lucide-react";

export function meta({}: Route.MetaArgs) {
  return [{ title: "Merlink Harness - Recycle Bin" }];
}

export default function RecycleBinPage() {
  return (
    <div className="flex h-full min-h-[85vh] flex-col items-center justify-center p-6 text-center">
      <div className="w-full max-w-sm rounded-2xl border border-slate-200 bg-white p-8 shadow-xs dark:border-slate-800 dark:bg-slate-950">
        <div className="mx-auto flex h-12 w-12 items-center justify-center rounded-2xl bg-slate-100 text-slate-500 dark:bg-slate-800 dark:text-slate-400">
          <Trash2 className="h-6 w-6" />
        </div>
        <h2 className="mt-4 text-base font-bold tracking-tight text-slate-900 dark:text-slate-100">
          Recycle Bin
        </h2>
        <p className="mt-1 text-xs text-slate-500 dark:text-slate-400">
          Các dự án đã bị xóa mềm sẽ được lưu trữ tạm thời tại đây.
        </p>
      </div>
    </div>
  );
}
