import { useId, type InputHTMLAttributes, type ReactNode } from 'react';
import { cn } from '../../utils/cn';

export interface CheckboxProps extends Omit<InputHTMLAttributes<HTMLInputElement>, 'type'> {
  label: ReactNode;
  description?: ReactNode;
}

export function Checkbox({ id, label, description, className, ...props }: CheckboxProps) {
  const generatedId = useId();
  const inputId = id ?? generatedId;
  const descriptionId = description ? `${inputId}-description` : undefined;

  return (
    <label htmlFor={inputId} className={cn('inline-flex cursor-pointer items-center gap-1.5', props.disabled && 'cursor-not-allowed opacity-55')}>
      <input
        {...props}
        id={inputId}
        type="checkbox"
        aria-describedby={descriptionId}
        className={cn('tier-input focus-ring h-4 w-4 rounded accent-accent disabled:cursor-not-allowed', className)}
      />
      <span className="inline-flex flex-col">
        <span>{label}</span>
        {description ? <span id={descriptionId} className="text-xs text-fg-subtle">{description}</span> : null}
      </span>
    </label>
  );
}
