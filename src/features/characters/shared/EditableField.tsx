import { useCharacterDetailStore } from '../store';
import { FieldDisplay } from './FieldDisplay';
import { cn } from '@/shared/utils/cn';

interface EditableFieldProps {
  label: string;
  value: string;
  displayValue?: string;
  onChange?: (v: string) => void;
  multiline?: boolean;
  className?: string;
}

export function EditableField({
  label,
  value,
  displayValue,
  onChange,
  multiline,
  className,
}: EditableFieldProps) {
  const isEditing = useCharacterDetailStore((s) => s.isEditing);

  if (!isEditing) {
    return (
      <FieldDisplay
        label={label}
        value={<span className="whitespace-pre-wrap">{displayValue ?? value}</span>}
        className={className}
      />
    );
  }

  const inputCls =
    'mt-1 w-full text-sm rounded-lg tier-input focus-ring px-3 py-2 text-fg focus-ring   ';

  return (
    <div className={cn('flex flex-col', className)}>
      <label className="text-xs uppercase tracking-wide text-fg-muted">
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
