import { useCharacterDetailStore } from '../store';
import { FieldDisplay } from './FieldDisplay';
import { cn } from '../../../../shared/utils/cn';

interface EditableFieldProps {
  label: string;
  value: string;
  onChange?: (v: string) => void;
  multiline?: boolean;
  className?: string;
}

export function EditableField({
  label,
  value,
  onChange,
  multiline,
  className,
}: EditableFieldProps) {
  const isEditing = useCharacterDetailStore((s) => s.isEditing);

  if (!isEditing) {
    return (
      <FieldDisplay
        label={label}
        value={<span className="whitespace-pre-wrap">{value}</span>}
        className={className}
      />
    );
  }

  const inputCls =
    'mt-1 w-full text-sm rounded-lg border border-slate-200 bg-slate-50 px-3 py-2 text-slate-800 focus:outline-hidden focus:ring-2 focus:ring-slate-400 dark:border-slate-700 dark:bg-slate-950 dark:text-slate-200';

  return (
    <div className={cn('flex flex-col', className)}>
      <label className="text-[11px] uppercase tracking-wide text-slate-500 dark:text-slate-400">
        {label}
      </label>
      {multiline ? (
        <textarea
          rows={3}
          className={inputCls}
          value={value}
          onChange={(e) => onChange?.(e.target.value)}
        />
      ) : (
        <input
          className={inputCls}
          value={value}
          onChange={(e) => onChange?.(e.target.value)}
        />
      )}
    </div>
  );
}
