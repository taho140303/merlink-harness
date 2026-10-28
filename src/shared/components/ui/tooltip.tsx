import {
  Children,
  cloneElement,
  isValidElement,
  useId,
  useState,
  type ReactElement,
  type ReactNode,
} from 'react';
import { cn } from '../../utils/cn';

export interface TooltipProps {
  content: ReactNode;
  children: ReactElement;
  side?: 'top' | 'bottom';
  className?: string;
}

/**
 * Tooltip nhẹ: hiển thị khi hover hoặc focus bàn phím, gắn aria-describedby
 * vào chính phần tử con (không bọc thêm phần tử trung gian).
 */
export function Tooltip({ content, children, side = 'top', className }: TooltipProps) {
  const [open, setOpen] = useState(false);
  const id = useId();

  const child = Children.only(children);
  if (!isValidElement(child)) return child;

  const describedBy = open ? id : undefined;
  const enhanced = cloneElement(child as ReactElement<{ 'aria-describedby'?: string }>, {
    'aria-describedby': describedBy,
  });

  return (
    <span
      className="relative inline-flex"
      onMouseEnter={() => setOpen(true)}
      onMouseLeave={() => setOpen(false)}
      onFocusCapture={() => setOpen(true)}
      onBlurCapture={() => setOpen(false)}
    >
      {enhanced}
      {open ? (
        <span
          role="tooltip"
          id={id}
          className={cn(
            'tier-overlay pointer-events-none absolute left-1/2 z-[90] w-max max-w-64 -translate-x-1/2 rounded-lg px-2 py-1 text-xs text-fg',
            side === 'top' ? 'bottom-full mb-2' : 'top-full mt-2',
            className,
          )}
        >
          {content}
        </span>
      ) : null}
    </span>
  );
}
