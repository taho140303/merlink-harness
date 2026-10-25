import { cn } from '../../../../shared/utils/cn';

interface EnumSelectProps {
  label: string;
  value: string;
  options: readonly string[];
  onChange?: (v: string) => void;
  className?: string;
}

export function EnumSelect({ label, value, options, onChange, className }: EnumSelectProps) {
  return (
    <div className={cn('flex flex-col', className)}>
      <label className="text-[11px] uppercase tracking-wide text-slate-500 dark:text-slate-400">
        {label}
      </label>
      {onChange ? (
        <select
          value={value}
          onChange={(e) => onChange(e.target.value)}
          className="mt-1 w-full rounded-lg border border-slate-200 bg-slate-50 px-3 py-2 text-sm text-slate-800 focus:outline-hidden focus:ring-2 focus:ring-slate-400 dark:border-slate-700 dark:bg-slate-950 dark:text-slate-200"
        >
          {options.map((o) => (
            <option key={o} value={o}>
              {o}
            </option>
          ))}
        </select>
      ) : (
        <span className="mt-1 text-sm font-medium text-slate-900 dark:text-slate-100">
          {value}
        </span>
      )}
    </div>
  );
}
