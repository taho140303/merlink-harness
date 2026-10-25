import { create } from 'zustand';
import type { Locale } from './dict';

const STORAGE_KEY = 'merlink.locale';

function loadInitial(): Locale {
  if (typeof window === 'undefined') return 'vi';
  const saved = window.localStorage.getItem(STORAGE_KEY);
  return saved === 'en' || saved === 'vi' ? saved : 'vi';
}

interface LocaleState {
  locale: Locale;
  setLocale: (l: Locale) => void;
}

export const useLocaleStore = create<LocaleState>((set) => ({
  locale: loadInitial(),
  setLocale: (locale) => {
    if (typeof window !== 'undefined') window.localStorage.setItem(STORAGE_KEY, locale);
    set({ locale });
  },
}));

export function useLocale() {
  const locale = useLocaleStore((s) => s.locale);
  const setLocale = useLocaleStore((s) => s.setLocale);
  return { locale, setLocale };
}
