import { useEffect, type ReactNode } from "react";
import { useThemeStore, applyTheme } from "@/shared/hooks/useTheme";

export function ThemeProvider({ children }: { children: ReactNode }) {
  useEffect(() => {
    // Chỉ chạy 1 lần khi mount để áp dụng theme ban đầu và lắng nghe OS
    const initial = useThemeStore.getState().theme;
    applyTheme(initial);

    const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
    const handleChange = () => {
      if (useThemeStore.getState().theme === 'system') {
        applyTheme('system');
      }
    };
    mediaQuery.addEventListener('change', handleChange);
    return () => mediaQuery.removeEventListener('change', handleChange);
  }, []);

  return <>{children}</>;
}
