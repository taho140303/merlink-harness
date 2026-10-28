import { useState, useEffect, type RefObject } from 'react';
import { cn } from '../../utils/cn';

export interface OutlineItem {
  id: string;
  title: string;
}

interface FloatingOutlineProps {
  items: OutlineItem[];
  containerRef?: RefObject<HTMLElement>;
  onItemClick: (id: string) => void;
}

const MAX_INDICATORS = 10;

export function FloatingOutline({ items, containerRef, onItemClick }: FloatingOutlineProps) {
  const [isHovered, setIsHovered] = useState(false);
  const [activeId, setActiveId] = useState<string | undefined>(items[0]?.id);

  // Logic theo dõi scroll để cập nhật active item
  useEffect(() => {
    const container = containerRef?.current;
    if (!container || items.length === 0) return;

    let ticking = false;

    const handleScroll = () => {
      if (!ticking) {
        window.requestAnimationFrame(() => {
          const offset = 120; // Khoảng cách từ đỉnh container
          let currentActive = items[0].id;

          for (const item of items) {
            const el = document.getElementById(item.id);
            if (el) {
              const rect = el.getBoundingClientRect();
              const containerRect = container.getBoundingClientRect();
              
              // Nếu đỉnh của item vượt qua ngưỡng offset so với đỉnh container
              if (rect.top - containerRect.top <= offset) {
                currentActive = item.id;
              } else {
                break;
              }
            }
          }
          setActiveId(currentActive);
          ticking = false;
        });
        ticking = true;
      }
    };

    container.addEventListener('scroll', handleScroll, { passive: true });
    // Gọi lần đầu để set active chuẩn xác
    setTimeout(handleScroll, 100);

    return () => container.removeEventListener('scroll', handleScroll);
  }, [items, containerRef]);

  if (items.length === 0) return null;

  // Logic Cửa sổ trượt (Sliding Window) cho các chỉ báo bên ngoài
  let displayIndicators = items;
  if (items.length > MAX_INDICATORS) {
    const activeIndex = items.findIndex((i) => i.id === activeId);
    if (activeIndex !== -1) {
      const start = Math.max(
        0,
        Math.min(activeIndex - Math.floor(MAX_INDICATORS / 2), items.length - MAX_INDICATORS)
      );
      displayIndicators = items.slice(start, start + MAX_INDICATORS);
    } else {
      displayIndicators = items.slice(0, MAX_INDICATORS);
    }
  }

  return (
    <div
      className="absolute right-2 top-1/2 -translate-y-1/2 z-40 flex items-center justify-end group"
      onMouseEnter={() => setIsHovered(true)}
      onMouseLeave={() => setIsHovered(false)}
    >
      {/* Vùng Trigger (Bắt sự kiện hover) */}
      <div className="absolute right-0 top-1/2 -translate-y-1/2 w-10 h-3/4 cursor-pointer z-0" />

      {/* Trigger Indicator (Các dấu gạch ngang) */}
      <div 
        className={cn(
          "flex flex-col items-end gap-4 py-8 transition-opacity duration-200 z-10 pointer-events-none",
          isHovered ? "opacity-0" : "opacity-100"
        )}
      >
        {displayIndicators.map((item) => (
          <div
            key={`indicator-${item.id}`}
            className={cn(
              "h-1.5 rounded-full transition-all duration-300 shrink-0",
              activeId === item.id 
                ? "w-6 bg-[#e61380]" 
                : "w-3 bg-slate-300 dark:bg-slate-700"
            )}
          />
        ))}
      </div>

      {/* Popup List khi Hover (Hiển thị full danh sách, có scroll) */}
      <div 
        className={cn(
          "absolute right-0 top-1/2 -translate-y-1/2 w-52 max-h-[400px] overflow-y-auto no-scrollbar rounded-xl border border-slate-200 bg-white/95 p-2 shadow-xl backdrop-blur-md dark:border-slate-800 dark:bg-slate-900/95 z-20 transition-all duration-200",
          isHovered ? "opacity-100 visible scale-100 pointer-events-auto" : "opacity-0 invisible scale-95 pointer-events-none"
        )}
      >
        <div className="flex flex-col gap-0.5">
          {items.map((item) => (
            <button
              key={item.id}
              onClick={() => {
                setActiveId(item.id);
                onItemClick(item.id);
              }}
              className={cn(
                "w-full text-left px-3 py-2 text-xs rounded-lg transition-colors truncate",
                activeId === item.id
                  ? "bg-[#e61380]/10 text-[#e61380] font-medium"
                  : "text-slate-600 hover:bg-slate-100 dark:text-slate-400 dark:hover:bg-slate-800 dark:hover:text-slate-200"
              )}
              title={item.title}
            >
              {item.title.length > 20 ? `${item.title.substring(0, 20)}...` : item.title}
            </button>
          ))}
        </div>
      </div>
    </div>
  );
}
