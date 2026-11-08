import { useEffect } from 'react';
import { getCurrentWebview } from '@tauri-apps/api/webview';
import { isTauriEnvironment } from '@/shared/backend/client';

const STORAGE_KEY = 'merlink.zoom';
const MIN = 0.8;
const MAX = 1.5;
const STEP = 0.1;

function clamp(level: number): number {
  // Round away floating point drift (e.g. 1.0000000000000002) before persisting.
  return Math.round(Math.min(MAX, Math.max(MIN, level)) * 100) / 100;
}

function readStoredZoom(): number {
  if (typeof window === 'undefined') return 1;
  try {
    const raw = window.localStorage.getItem(STORAGE_KEY);
    if (!raw) return 1;
    const parsed = Number(raw);
    return Number.isFinite(parsed) ? clamp(parsed) : 1;
  } catch {
    return 1;
  }
}

function persistZoom(level: number) {
  try {
    window.localStorage.setItem(STORAGE_KEY, String(level));
  } catch {
    /* storage unavailable: keep the in-memory zoom only */
  }
}

async function applyZoom(level: number): Promise<void> {
  if (typeof document === 'undefined') return;
  persistZoom(level);

  // Prefer the native webview zoom so `100vh` layouts are not clipped; fall back
  // to CSS zoom for browser dev runs where the Tauri bridge is absent.
  if (isTauriEnvironment()) {
    try {
      await getCurrentWebview().setZoom(level);
      document.documentElement.style.zoom = '';
      return;
    } catch {
      /* permission or runtime unavailable: use the CSS fallback below */
    }
  }
  document.documentElement.style.zoom = String(level);
}

/**
 * Ctrl/⌘ + `+`, `-`, `0` page zoom.
 *
 * The desktop webview has no native zoom shortcuts, so the application owns
 * them and persists the chosen level across restarts.
 */
export function useZoomHotkeys(): void {
  useEffect(() => {
    if (typeof window === 'undefined') return;

    let current = readStoredZoom();
    void applyZoom(current);

    const onKeyDown = (event: KeyboardEvent) => {
      if (!(event.ctrlKey || event.metaKey) || event.altKey) return;

      const { key, code } = event;
      const isZoomIn = key === '+' || key === '=' || code === 'NumpadAdd';
      const isZoomOut = key === '-' || key === '_' || code === 'NumpadSubtract';
      const isReset = key === '0' || code === 'Numpad0';
      if (!isZoomIn && !isZoomOut && !isReset) return;

      event.preventDefault();
      if (isReset) current = 1;
      else current = clamp(current + (isZoomIn ? STEP : -STEP));
      void applyZoom(current);
    };

    // `capture` runs before app handlers so the shortcut always wins.
    window.addEventListener('keydown', onKeyDown, true);
    return () => window.removeEventListener('keydown', onKeyDown, true);
  }, []);
}
