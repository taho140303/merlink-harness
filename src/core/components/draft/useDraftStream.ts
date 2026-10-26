import { useState, useEffect } from 'react';

export function useDraftStream(totalItems: number, delayMs = 400) {
  const [streamedCount, setStreamedCount] = useState(0);

  useEffect(() => {
    let current = 0;
    const interval = setInterval(() => {
      current += 1;
      setStreamedCount(current);
      if (current >= totalItems) clearInterval(interval);
    }, delayMs);
    return () => clearInterval(interval);
  }, [totalItems, delayMs]);

  return streamedCount;
}
