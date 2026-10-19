export type Theme = "light" | "dark" | "system";

const THEME_KEY = "merlink-theme";
const SYSTEM_DARK = "(prefers-color-scheme: dark)";

export function readTheme(): Theme {
  try {
    const stored = localStorage.getItem(THEME_KEY);
    return stored === "light" || stored === "dark" ? stored : "system";
  } catch {
    return "system";
  }
}

export function applyTheme(theme: Theme, persist = false): void {
  if (persist) {
    try {
      localStorage.setItem(THEME_KEY, theme);
    } catch {
      // The selected theme still applies for this session if storage is unavailable.
    }
  }
  const dark = theme === "dark" ||
    (theme === "system" && window.matchMedia(SYSTEM_DARK).matches);
  document.documentElement.dataset.theme = dark ? "dark" : "light";
  document.documentElement.classList.toggle("dark", dark);
}

export function subscribeSystemTheme(): () => void {
  const media = window.matchMedia(SYSTEM_DARK);
  const onChange = () => {
    if (readTheme() === "system") applyTheme("system");
  };
  media.addEventListener("change", onChange);
  return () => media.removeEventListener("change", onChange);
}
