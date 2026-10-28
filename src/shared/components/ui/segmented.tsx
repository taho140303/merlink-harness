import { useRef, type KeyboardEvent, type ReactNode } from 'react';
import { cn } from '../../utils/cn';

export interface SegmentedItem {
  id: string;
  label: ReactNode;
  icon?: ReactNode;
}

export interface SegmentedProps {
  items: SegmentedItem[];
  value: string;
  onChange: (id: string) => void;
  ariaLabel: string;
  className?: string;
}

export function Segmented({ items, value, onChange, ariaLabel, className }: SegmentedProps) {
  const groupRef = useRef<HTMLDivElement>(null);
  const select = (id: string) => {
    onChange(id);
    groupRef.current?.querySelector<HTMLButtonElement>(`[data-segment-id="${id}"]`)?.focus();
  };
  const handleKeyDown = (event: KeyboardEvent<HTMLButtonElement>, currentId: string) => {
    if (!items.length) return;
    const index = items.findIndex((item) => item.id === currentId);
    let next: number | null = null;
    if (event.key === 'ArrowRight') next = (index + 1) % items.length;
    else if (event.key === 'ArrowLeft') next = (index - 1 + items.length) % items.length;
    else if (event.key === 'Home') next = 0;
    else if (event.key === 'End') next = items.length - 1;
    if (next !== null) {
      event.preventDefault();
      select(items[next].id);
    }
  };

  return (
    <div ref={groupRef} role="radiogroup" aria-label={ariaLabel} className={cn('inline-flex items-center gap-1 rounded-xl bg-surface-muted p-1', className)}>
      {items.map((item) => {
        const active = item.id === value;
        return (
          <button
            key={item.id}
            type="button"
            role="radio"
            data-segment-id={item.id}
            aria-checked={active}
            tabIndex={active ? 0 : -1}
            onClick={() => onChange(item.id)}
            onKeyDown={(event) => handleKeyDown(event, item.id)}
            className={cn('focus-ring inline-flex items-center justify-center gap-2 rounded-lg px-3 py-2 text-xs font-medium transition-colors', active ? 'bg-surface-card text-fg' : 'text-fg-muted hover:text-fg')}
          >
            {item.icon ? <span aria-hidden="true">{item.icon}</span> : null}
            {item.label}
          </button>
        );
      })}
    </div>
  );
}

