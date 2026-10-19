import { getCurrentWebview } from "@tauri-apps/api/webview";
import { isTauri } from "./tauri";

/**
 * Text scale (Sprint 11.13 · D1).
 *
 * The whole app zooms, because the complaint was "font chữ nhỏ quá" — not "make
 * one panel's font bigger". `100%` is the small end, `110%` is the default, and
 * the setting survives a restart in `localStorage["merlink-text-scale"]` as a
 * plain string number (`"125"`).
 *
 * Applying it has two transports: Tauri's native webview zoom (no dependency,
 * no new command — `core:webview:allow-set-webview-zoom` in capabilities), and
 * CSS `zoom` for the plain-browser preview.
 */
export interface TextScaleOption {
  /** Percentage as shown in the UI and stored in localStorage. */
  value: number;
  label: string;
}

export const TEXT_SCALE_STORAGE_KEY = "merlink-text-scale";

export const DEFAULT_TEXT_SCALE = 1.1;

export const MIN_TEXT_SCALE = 0.9;
export const MAX_TEXT_SCALE = 1.4;

export const TEXT_SCALE_OPTIONS: TextScaleOption[] = [
  { value: 100, label: "Nhỏ" },
  { value: 110, label: "Vừa" },
  { value: 125, label: "Lớn" },
  { value: 140, label: "Rất lớn" },
];

/** Keep the webview usable even if storage holds nonsense. */
function clampScale(scale: number): number {
  return Math.min(MAX_TEXT_SCALE, Math.max(MIN_TEXT_SCALE, scale));
}

/**
 * The persisted scale as a ratio (`1.1`, not `110`).
 *
 * Anything unreadable falls back to the default; the value is clamped so a bad
 * entry cannot zoom the app into or out of existence.
 */
export function readTextScale(): number {
  try {
    const raw = localStorage.getItem(TEXT_SCALE_STORAGE_KEY);
    if (raw === null || raw.trim() === "") return DEFAULT_TEXT_SCALE;
    const parsed = Number(raw);
    if (!Number.isFinite(parsed) || parsed <= 0) return DEFAULT_TEXT_SCALE;
    // The stored format is a percentage ("110"); tolerating a ratio ("1.1") costs
    // nothing and keeps an older preference readable.
    const ratio = parsed > 2 ? parsed / 100 : parsed;
    return clampScale(ratio);
  } catch {
    return DEFAULT_TEXT_SCALE;
  }
}

/**
 * Apply the scale to the entire app (D1).
 *
 * Errors are swallowed (and logged) on purpose: a failed zoom must never take
 * the UI down. The promise resolves either way so callers can fire and forget.
 */
export async function applyTextScale(scale: number): Promise<void> {
  const normalized = clampScale(scale);
  try {
    if (isTauri()) {
      await getCurrentWebview().setZoom(normalized);
    } else {
      document.documentElement.style.zoom = String(normalized);
    }
  } catch (error) {
    console.error("applyTextScale error:", error);
  }
}
