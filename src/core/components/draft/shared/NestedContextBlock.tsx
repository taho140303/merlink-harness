import { useState, type ReactNode } from 'react';
import { ChevronRight } from 'lucide-react';
import { cn } from '../../../../shared/utils/cn';

interface NestedContextBlockProps {
  title: string;
  subtitle?: string;
  badge?: string;
  icon?: ReactNode;
  defaultOpen?: boolean;
  children: ReactNode;
}

export function NestedContextBlock({
  title,
  subtitle,
  badge,
  icon,
  defaultOpen = false,
  children,
}: NestedContextBlockProps) {
  const [isOpen, setIsOpen] = useState(defaultOpen);

  return (
    <div className="overflow-hidden rounded-2xl border border-slate-200 bg-white shadow-xs dark:border-slate-800 dark:bg-slate-900 animate-in fade-in-50 duration-300">
      {/* Header Context (Cũ - Đóng) */}
      <div className="border-b border-slate-100 bg-slate-50/80 dark:border-slate-800 dark:bg-slate-800/40">
        <button
          type="button"
          onClick={() => setIsOpen(!isOpen)}
          className="flex w-full items-start justify-between p-4 text-left transition-colors hover:bg-slate-100/50 dark:hover:bg-slate-800/60"
        >
          <div className="flex-1">
            <div className="flex items-center gap-2">
              {icon}
              <h3 className="text-sm font-bold text-slate-900 dark:text-slate-100">{title}</h3>
              {badge && (
                <span className="rounded-md bg-slate-200 px-1.5 py-0.5 text-[10px] font-semibold uppercase text-slate-600 dark:bg-slate-700 dark:text-slate-300">
                  {badge}
                </span>
              )}
            </div>
            {subtitle && (
              <p className="mt-1 text-[11px] leading-relaxed text-slate-600 dark:text-slate-400 line-clamp-1">
                {subtitle}
              </p>
            )}
          </div>
          <div className="ml-4 flex shrink-0 items-center gap-2">
            <span className="text-[10px] font-medium text-slate-400 uppercase tracking-wider">
              {isOpen ? 'Thu gọn' : 'Mở rộng'}
            </span>
            <ChevronRight className={cn('h-4 w-4 text-slate-400 transition-transform', isOpen && 'rotate-90')} />
          </div>
        </button>

        {/* Nội dung Context mở rộng */}
        {isOpen && (
          <div className="animate-in fade-in-0 border-t border-slate-200 p-4 dark:border-slate-800">
             <p className="text-xs leading-relaxed text-slate-600 dark:text-slate-300">
               {subtitle || "Không có mô tả chi tiết."}
             </p>
          </div>
        )}
      </div>

      {/* Body Nội dung mới */}
      <div className="p-4 sm:p-5">{children}</div>
    </div>
  );
}
