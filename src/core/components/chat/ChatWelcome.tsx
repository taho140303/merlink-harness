import { OrbSphere } from "./OrbSphere";
import { useT } from "../../../shared/i18n/useT";

export function ChatWelcome() {
  const t = useT();

  return (
    <div className="flex h-full min-h-[55vh] flex-col items-center justify-center text-center select-none">
      <div className="flex flex-col items-center space-y-6">
        <OrbSphere />
        <h1 className="max-w-md text-xl font-semibold tracking-tight text-slate-800 dark:text-slate-100 sm:text-2xl">
          {t("chat.welcomeTitle")}
        </h1>
      </div>
    </div>
  );
}
