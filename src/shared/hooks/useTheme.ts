import { useEffect } from 'react';
import { create } from 'zustand';

export type Theme = 'light' | 'dark' | 'system';

interface ThemeState {
  theme: Theme;
  setTheme: (theme: Theme) => void;
}

// Hàm lấy theme ban đầu từ localStorage
const getInitialTheme = (): Theme => {
  if (typeof window === 'undefined') return 'system';
  const saved = window.localStorage.getItem('theme') as Theme | null;
  if (saved === 'light' || saved === 'dark') return saved;
  return 'system';
};

// Hàm thực thi thay đổi class trên thẻ html
export function applyTheme(theme: Theme) {
  if (typeof window === 'undefined') return;
  const root = window.document.documentElement;
  root.classList.remove('light', 'dark');

  let activeTheme = theme;
  if (theme === 'system') {
    activeTheme = window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
  }
  root.classList.add(activeTheme);
}

export const useThemeStore = create<ThemeState>((set) => ({
  theme: getInitialTheme(), // Khởi tạo đúng giá trị từ localStorage
  setTheme: (theme) => {
    localStorage.setItem('theme', theme);
    set({ theme });
    applyTheme(theme);
  },
}));

export function useTheme() {
  const theme = useThemeStore((s) => s.theme);
  const setTheme = useThemeStore((s) => s.setTheme);

  // Đồng bộ class khi component mount
  useEffect(() => {
    applyTheme(theme);

    // Lắng nghe sự thay đổi của OS nếu đang ở chế độ System
    if (theme !== 'system') return;
    const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
    const handleChange = () => applyTheme('system');
    mediaQuery.addEventListener('change', handleChange);
    return () => mediaQuery.removeEventListener('change', handleChange);
  }, [theme]);

  return { theme, setTheme };
}
