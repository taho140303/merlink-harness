import { useLocaleStore } from './store';
import { dict, type TKey } from './dict';

export function useT() {
  const locale = useLocaleStore((s) => s.locale);
  return (key: TKey): string => dict[locale][key] ?? dict.en[key] ?? key;
}
