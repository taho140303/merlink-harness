import type { ReactNode } from 'react';
import { cn } from '../../utils/cn';

/** Class dùng chung cho mọi control dạng input (Input/Textarea/Select). */
export const controlClass =
  'tier-input focus-ring w-full rounded-xl px-3 text-sm text-fg transition-colors disabled:cursor-not-allowed disabled:opacity-55';

interface FieldProps {
  id: string;
  label?: ReactNode;
  hint?: ReactNode;
  error?: ReactNode;
  required?: boolean;
  className?: string;
  children: ReactNode;
}

/**
 * Wrapper label + hint + error, đồng thời sinh id mô tả để control trỏ tới
 * bằng aria-describedby. Dùng bởi Input/Textarea/Select.
 */
export function Field({ id, label, hint, error, required, className, children }: FieldProps) {
  const description = error ?? hint;
  return (
    <div className={cn('flex flex-col gap-1', className)}>
      {label ? (
        <label htmlFor={id} className="text-xs font-medium text-fg-muted">
          {label}
          {required ? <span className="text-rose-500"> *</span> : null}
        </label>
      ) : null}
      {children}
      {description ? (
        <p id={`${id}-desc`} className={cn('text-xs', error ? 'text-rose-500' : 'text-fg-subtle')}>
          {description}
        </p>
      ) : null}
    </div>
  );
}
