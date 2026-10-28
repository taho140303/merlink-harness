import type { ReactNode } from 'react';
import { cn } from '../../utils/cn';

export interface SwitchProps {
  checked: boolean;
  onCheckedChange: (checked: boolean) => void;
  label?: ReactNode;
  ariaLabel: string;
  disabled?: boolean;
  className?: string;
}

export function Switch({ checked, onCheckedChange, label, ariaLabel, disabled, className }: SwitchProps) {
  return (
    <button
      type="button"
      role="switch"
      aria-checked={checked}
      aria-label={ariaLabel}
      disabled={disabled}
      onClick={() => onCheckedChange(!checked)}
      className={cn('focus-ring relative inline-flex h-5 w-9 shrink-0 items-center rounded-full transition-colors disabled:cursor-not-allowed disabled:opacity-55', checked ? 'bg-accent' : 'bg-line', className)}
    >
      <span aria-hidden="true" className={cn('inline-block h-3.5 w-3.5 rounded-full bg-surface-card transition-transform', checked ? 'translate-x-4' : 'translate-x-1')} />
      {label ? <span className="sr-only">{label}</span> : null}
    </button>
  );
}
