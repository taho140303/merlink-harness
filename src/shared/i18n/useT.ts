import { useLocaleStore } from './store';
import { dict, type TKey } from './dict';

export function useT() {
  const locale = useLocaleStore((s) => s.locale);
  return (key: TKey, params?: Record<string, string | number>): string => {
    let text = dict[locale][key] ?? dict.en[key] ?? key;
    if (params) {
      Object.entries(params).forEach(([k, v]) => {
        text = text.replace(new RegExp(`\\{${k}\\}`, 'g'), String(v));
      });
    }
    return text;
  };
}
