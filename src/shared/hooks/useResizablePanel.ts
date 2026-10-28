import { useState, useRef, useEffect, useCallback } from "react";

interface UseResizablePanelOptions {
  initialRatio?: number;
  minPx?: number;
  minPxLeft?: number;
  minPxRight?: number;
}

export function useResizablePanel({
  initialRatio = 50,
  minPx = 360,
  minPxLeft,
  minPxRight,
}: UseResizablePanelOptions = {}) {
  const [leftRatio, setLeftRatio] = useState(initialRatio);
  const [isDragging, setIsDragging] = useState(false);
  const containerRef = useRef<HTMLDivElement>(null);

  // Ưu tiên minPxLeft/minPxRight nếu được truyền, nếu không fallback về minPx
  const actualMinLeft = minPxLeft ?? minPx;
  const actualMinRight = minPxRight ?? minPx;

  useEffect(() => {
    if (!isDragging) return;

    const handleMouseMove = (e: MouseEvent) => {
      if (!containerRef.current) return;
      const rect = containerRef.current.getBoundingClientRect();
      const currentPx = e.clientX - rect.left;
      const totalPx = rect.width;

      // Đảm bảo left không nhỏ hơn actualMinLeft và right không nhỏ hơn actualMinRight
      const clampedPx = Math.min(Math.max(currentPx, actualMinLeft), totalPx - actualMinRight);
      
      // Đề phòng trường hợp màn hình quá nhỏ, đảm bảo ratio không bị âm
      const safePx = Math.max(0, clampedPx);
      
      const newRatio = (safePx / totalPx) * 100;
      setLeftRatio(newRatio);
    };

    const handleMouseUp = () => {
      setIsDragging(false);
    };

    window.addEventListener("mousemove", handleMouseMove);
    window.addEventListener("mouseup", handleMouseUp);
    return () => {
      window.removeEventListener("mousemove", handleMouseMove);
      window.removeEventListener("mouseup", handleMouseUp);
    };
  }, [isDragging, actualMinLeft, actualMinRight]);

  const startDragging = useCallback((e: React.MouseEvent) => {
    e.preventDefault();
    setIsDragging(true);
  }, []);

  return {
    leftRatio,
    isDragging,
    containerRef,
    startDragging,
  };
}
