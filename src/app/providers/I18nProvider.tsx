import type { ReactNode } from "react";

export function I18nProvider({ children }: { children: ReactNode }) {
  // Không cần subscribe toàn bộ cây component ở tầng root vì các component
  // dùng văn bản đã tự động subscribe thông qua hook useT() cục bộ.
  return <>{children}</>;
}
