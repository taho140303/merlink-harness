import { useState } from 'react';
import { cn } from '../../utils/cn';

export interface OutlineItem {
  id: string;
  title: string;
}

interface FloatingOutlineProps {
  items: OutlineItem[];
  activeId?: string;
  onItemClick: (id: string) => void;
}

export function FloatingOutline({ items, activeId, onItemClick }: FloatingOutlineProps) {
  const [isHovered, setIsHovered] = useState(false);

  if (items.length === 0) return null;

  return (
    <div
      className="absolute right-4 top-1/2 -translate-y-1/2 z-40 flex flex-col items-end group"
      onMouseEnter={() => setIsHovered(true)}
      onMouseLeave={() => setIsHovered(false)}
    >
      {/* Trigger Indicator (Các dấu gạch ngang nhỏ) */}
      <div className="flex flex-col gap-1.5 py-4 px-2 cursor-pointer">
        {items.map((item) => (
          <div
            key={`indicator-${item.id}`}
            className={cn(
              "h-1 rounded-full transition-all duration-300",
              activeId === item.id 
                ? "w-6 bg-blue-500" 
                : "w-3 bg-slate-300 group-hover:bg-slate-400 dark:bg-slate-700 dark:group-hover:bg-slate-600"
            )}
          />
        ))}
      </div>

      {/* Popup List khi Hover */}
      {isHovered && (
        <div className="absolute right-full mr-2 top-1/2 -translate-y-1/2 w-64 max-h-96 overflow-y-auto no-scrollbar rounded-xl border border-slate-200 bg-white/95 p-2 shadow-xl backdrop-blur-md dark:border-slate-800 dark:bg-slate-900/95 animate-in fade-in zoom-in-95 duration-200">
          <div className="flex flex-col gap-0.5">
            {items.map((item) => (
              <button
                key={item.id}
                onClick={() => onItemClick(item.id)}
                className={cn(
                  "w-full text-left px-3 py-2 text-xs rounded-lg transition-colors truncate",
                  activeId === item.id
                    ? "bg-blue-50 text-blue-700 font-medium dark:bg-blue-950/60 dark:text-blue-300"
                    : "text-slate-600 hover:bg-slate-100 dark:text-slate-400 dark:hover:bg-slate-800 dark:hover:text-slate-200"
                )}
                title={item.title}
              >
                {item.title.length > 20 ? `${item.title.substring(0, 20)}...` : item.title}
              </button>
            ))}
          </div>
        </div>
      )}
    </div>
  );
}
