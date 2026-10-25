import type { ReactNode } from 'react';
import { cn } from '../../../../shared/utils/cn';

interface FieldDisplayProps {
  label: string;
  value: ReactNode;
  colorHex?: string;
  className?: string;
}

export function FieldDisplay({ label, value, colorHex, className }: FieldDisplayProps) {
  return (
    <div className={cn('flex flex-col gap-1', className)}>
      <span className="text-[11px] uppercase tracking-wide text-slate-500 dark:text-slate-400">
        {label}
      </span>
      <div className="flex items-center gap-2">
        {colorHex && (
          <span
            className="h-3.5 w-3.5 shrink-0 rounded-full border border-slate-300 dark:border-slate-600"
            style={{ backgroundColor: colorHex }}
          />
        )}
        <span className="text-sm font-medium text-slate-900 dark:text-slate-100">{value}</span>
      </div>
    </div>
  );
}
