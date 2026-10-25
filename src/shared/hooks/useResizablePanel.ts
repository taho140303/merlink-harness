import { useState, useRef, useEffect, useCallback } from "react";

interface UseResizablePanelOptions {
  initialRatio?: number;
  minPx?: number;
}

export function useResizablePanel({
  initialRatio = 50,
  minPx = 360,
}: UseResizablePanelOptions = {}) {
  const [leftRatio, setLeftRatio] = useState(initialRatio);
  const [isDragging, setIsDragging] = useState(false);
  const containerRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (!isDragging) return;

    const handleMouseMove = (e: MouseEvent) => {
      if (!containerRef.current) return;
      const rect = containerRef.current.getBoundingClientRect();
      const currentPx = e.clientX - rect.left;
      const totalPx = rect.width;

      const clampedPx = Math.min(Math.max(currentPx, minPx), totalPx - minPx);
      const newRatio = (clampedPx / totalPx) * 100;
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
  }, [isDragging, minPx]);

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
