import { useEffect, useState } from 'react';

/** Theo dõi một media query, cập nhật khi viewport đổi. */
export function useMediaQuery(query: string): boolean {
  const [matches, setMatches] = useState(() =>
    typeof window === 'undefined' ? false : window.matchMedia(query).matches,
  );

  useEffect(() => {
    const mql = window.matchMedia(query);
    const onChange = () => setMatches(mql.matches);
    onChange();
    mql.addEventListener('change', onChange);
    return () => mql.removeEventListener('change', onChange);
  }, [query]);

  return matches;
}

/**
 * Dưới 1280px, các split-pane (chat/draft, character chat, project chat)
 * không đủ chỗ cho 2 cột tối thiểu → chuyển sang chế độ 1 pane + overlay.
 */
export const COMPACT_QUERY = '(max-width: 1279px)';

export function useIsCompact(): boolean {
  return useMediaQuery(COMPACT_QUERY);
}
