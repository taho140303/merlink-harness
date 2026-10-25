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
      <span className="text-[11px] uppercase tracking-wide text-gray-500">{label}</span>
      <div className="flex items-center gap-2">
        {colorHex && (
          <span
            className="h-3.5 w-3.5 rounded-full border border-gray-300 shrink-0"
            style={{ backgroundColor: colorHex }}
          />
        )}
        <span className="text-sm font-medium text-gray-900">{value}</span>
      </div>
    </div>
  );
}
