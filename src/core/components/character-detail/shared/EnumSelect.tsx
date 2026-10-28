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
      <label className="text-xs uppercase tracking-wide text-fg-muted">
        {label}
      </label>
      {onChange ? (
        <select
          value={value}
          onChange={(e) => onChange(e.target.value)}
          className="mt-1 w-full rounded-lg tier-input focus-ring px-3 py-2 text-sm text-fg focus-ring   "
        >
          {options.map((o) => (
            <option key={o} value={o}>
              {o}
            </option>
          ))}
        </select>
      ) : (
        <span className="mt-1 text-sm font-medium text-fg">
          {value}
        </span>
      )}
    </div>
  );
}
