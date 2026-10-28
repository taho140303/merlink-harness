import { OrbSphere } from "./OrbSphere";
import { useT } from "../../../shared/i18n/useT";

export function ChatWelcome() {
  const t = useT();

  return (
    <div className="flex h-full min-h-[55vh] flex-col items-center justify-center text-center select-none">
      <div className="flex flex-col items-center space-y-8">
        <OrbSphere />
        
        <div className="flex flex-col items-center space-y-3">
          {/* Dòng chào mừng */}
          <span className="text-xs font-bold uppercase tracking-[0.2em] text-indigo-500 dark:text-indigo-400">
            {t("chat.welcomeGreeting")}
          </span>
          
          {/* Tiêu đề chính */}
          <h1 className="max-w-md text-2xl font-semibold tracking-tight text-fg  sm:text-3xl">
            {t("chat.welcomeTitle")}
          </h1>
        </div>
      </div>
    </div>
  );
}
