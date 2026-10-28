import { useRef, type KeyboardEvent, type ReactNode } from 'react';
import { cn } from '../../utils/cn';

export interface TabItem {
  id: string;
  label: ReactNode;
  icon?: ReactNode;
  disabled?: boolean;
}

export interface TabsProps {
  items: TabItem[];
  value: string;
  onChange: (id: string) => void;
  /** Nhãn cho tablist (bắt buộc cho a11y). */
  ariaLabel: string;
  variant?: 'underline' | 'pill';
  className?: string;
}

/**
 * Tab bar chuẩn: role=tablist/tab, aria-selected, roving tabindex,
 * điều hướng ArrowLeft/ArrowRight/Home/End.
 */
export function Tabs({ items, value, onChange, ariaLabel, variant = 'underline', className }: TabsProps) {
  const listRef = useRef<HTMLDivElement>(null);

  const focusTab = (id: string) => {
    onChange(id);
    listRef.current?.querySelector<HTMLButtonElement>(`[data-tab-id="${id}"]`)?.focus();
  };

  const handleKeyDown = (event: KeyboardEvent<HTMLButtonElement>, currentId: string) => {
    const enabled = items.filter((item) => !item.disabled);
    if (enabled.length === 0) return;
    const position = enabled.findIndex((item) => item.id === currentId);

    let nextPosition: number | null = null;
    if (event.key === 'ArrowRight' || event.key === 'ArrowDown') {
      nextPosition = (position + 1) % enabled.length;
    } else if (event.key === 'ArrowLeft' || event.key === 'ArrowUp') {
      nextPosition = (position - 1 + enabled.length) % enabled.length;
    } else if (event.key === 'Home') {
      nextPosition = 0;
    } else if (event.key === 'End') {
      nextPosition = enabled.length - 1;
    }

    if (nextPosition === null) return;
    event.preventDefault();
    focusTab(enabled[nextPosition].id);
  };

  return (
    <div
      ref={listRef}
      role="tablist"
      aria-label={ariaLabel}
      className={cn(
        'flex items-center gap-1',
        variant === 'underline' && 'border-b border-line',
        className,
      )}
    >
      {items.map((item) => {
        const selected = item.id === value;
        return (
          <button
            key={item.id}
            type="button"
            role="tab"
            data-tab-id={item.id}
            aria-selected={selected}
            tabIndex={selected ? 0 : -1}
            disabled={item.disabled}
            onClick={() => onChange(item.id)}
            onKeyDown={(event) => handleKeyDown(event, item.id)}
            className={cn(
              'focus-ring inline-flex items-center gap-1.5 whitespace-nowrap px-3 py-2 text-xs font-semibold transition-colors disabled:pointer-events-none disabled:opacity-40',
              variant === 'underline'
                ? cn('-mb-px border-b-2', selected ? 'border-accent text-fg' : 'border-transparent text-fg-muted hover:text-fg')
                : cn('rounded-full', selected ? 'bg-accent-soft text-accent' : 'text-fg-muted hover:bg-surface-muted hover:text-fg'),
            )}
          >
            {item.icon ? <span aria-hidden="true">{item.icon}</span> : null}
            {item.label}
          </button>
        );
      })}
    </div>
  );
}
