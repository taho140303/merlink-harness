import {
  useEffect,
  useLayoutEffect,
  useRef,
  useState,
  type ReactNode,
} from 'react';
import { createPortal } from 'react-dom';
import { cn } from '../../utils/cn';

const useIsomorphicLayoutEffect = typeof window !== 'undefined' ? useLayoutEffect : useEffect;

export interface PopoverProps {
  /** Nội dung bên trong nút trigger (icon hoặc text). */
  trigger: ReactNode;
  /** Nhãn a11y cho cả trigger và panel. */
  label: string;
  children: ReactNode;
  align?: 'start' | 'end';
  side?: 'bottom' | 'top';
  triggerClassName?: string;
  contentClassName?: string;
}

/**
 * Popover chuẩn duy nhất của app: portal ra body (không bị overflow-hidden cắt),
 * đóng bằng Esc/click ngoài, trả focus về trigger, có aria-haspopup/expanded.
 */
export function Popover({
  trigger,
  label,
  children,
  align = 'start',
  side = 'bottom',
  triggerClassName,
  contentClassName,
}: PopoverProps) {
  const [open, setOpen] = useState(false);
  const [position, setPosition] = useState<{ top: number; left: number } | null>(null);
  const triggerRef = useRef<HTMLButtonElement>(null);
  const panelRef = useRef<HTMLDivElement>(null);

  useIsomorphicLayoutEffect(() => {
    if (!open) return;

    const place = () => {
      const anchor = triggerRef.current?.getBoundingClientRect();
      if (!anchor) return;
      const panel = panelRef.current;
      const width = panel?.offsetWidth ?? 240;
      const height = panel?.offsetHeight ?? 200;

      let left = align === 'end' ? anchor.right - width : anchor.left;
      left = Math.max(8, Math.min(left, window.innerWidth - width - 8));

      let top = side === 'bottom' ? anchor.bottom + 8 : anchor.top - height - 8;
      if (top < 8) top = anchor.bottom + 8;
      if (top + height > window.innerHeight - 8) top = Math.max(8, anchor.top - height - 8);

      setPosition({ top, left });
    };

    place();
    window.addEventListener('resize', place);
    window.addEventListener('scroll', place, true);
    return () => {
      window.removeEventListener('resize', place);
      window.removeEventListener('scroll', place, true);
    };
  }, [open, align, side]);

  useEffect(() => {
    if (!open) return;

    const handlePointerDown = (event: MouseEvent) => {
      const target = event.target as Node;
      if (panelRef.current?.contains(target) || triggerRef.current?.contains(target)) return;
      setOpen(false);
    };

    const handleKeyDown = (event: KeyboardEvent) => {
      if (event.key !== 'Escape') return;
      setOpen(false);
      triggerRef.current?.focus();
    };

    document.addEventListener('mousedown', handlePointerDown);
    document.addEventListener('keydown', handleKeyDown);

    const focusTimer = window.setTimeout(() => {
      panelRef.current
        ?.querySelector<HTMLElement>(
          'button:not([disabled]), [href], input:not([disabled]), select:not([disabled]), textarea:not([disabled]), [tabindex]:not([tabindex="-1"])',
        )
        ?.focus();
    }, 0);

    return () => {
      document.removeEventListener('mousedown', handlePointerDown);
      document.removeEventListener('keydown', handleKeyDown);
      window.clearTimeout(focusTimer);
    };
  }, [open]);

  return (
    <>
      <button
        ref={triggerRef}
        type="button"
        aria-label={label}
        aria-haspopup="dialog"
        aria-expanded={open}
        onClick={() => setOpen((value) => !value)}
        className={cn('focus-ring rounded-lg', triggerClassName)}
      >
        {trigger}
      </button>

      {open && typeof document !== 'undefined'
        ? createPortal(
            <div
              ref={panelRef}
              role="dialog"
              aria-label={label}
              style={{ top: position?.top ?? -9999, left: position?.left ?? -9999 }}
              className={cn('tier-overlay fixed z-[80] rounded-panel p-2', contentClassName)}
            >
              {children}
            </div>,
            document.body,
          )
        : null}
    </>
  );
}
