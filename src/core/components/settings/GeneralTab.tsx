import { Sun, Moon, Laptop } from "lucide-react";
import { cn } from "../../../shared/utils/cn";
import { useTheme } from "../../../shared/hooks/useTheme";

export function GeneralTab() {
  const { theme, setTheme } = useTheme();

  return (
    <div className="space-y-6">
      <div>
        <h3 className="text-base font-semibold text-slate-900 dark:text-slate-100">General Settings</h3>
        <p className="text-xs text-slate-500 dark:text-slate-400">Quản lý giao diện và trải nghiệm hiển thị.</p>
      </div>

      <div className="space-y-2">
        <label className="text-xs font-semibold uppercase tracking-wider text-slate-500 dark:text-slate-400">
          Appearance
        </label>
        <div className="grid grid-cols-3 gap-2 rounded-xl bg-slate-100 p-1 dark:bg-slate-800 max-w-sm">
          <button
            type="button"
            onClick={() => setTheme("light")}
            className={cn(
              "flex items-center justify-center gap-2 rounded-lg py-2 text-xs font-medium transition-all",
              theme === "light"
                ? "bg-white text-slate-900 shadow-xs dark:bg-slate-700 dark:text-white"
                : "text-slate-600 hover:text-slate-900 dark:text-slate-400 dark:hover:text-slate-100"
            )}
          >
            <Sun className="h-4 w-4" />
            <span>Light</span>
          </button>

          <button
            type="button"
            onClick={() => setTheme("dark")}
            className={cn(
              "flex items-center justify-center gap-2 rounded-lg py-2 text-xs font-medium transition-all",
              theme === "dark"
                ? "bg-white text-slate-900 shadow-xs dark:bg-slate-700 dark:text-white"
                : "text-slate-600 hover:text-slate-900 dark:text-slate-400 dark:hover:text-slate-100"
            )}
          >
            <Moon className="h-4 w-4" />
            <span>Dark</span>
          </button>

          <button
            type="button"
            onClick={() => setTheme("system")}
            className={cn(
              "flex items-center justify-center gap-2 rounded-lg py-2 text-xs font-medium transition-all",
              theme === "system"
                ? "bg-white text-slate-900 shadow-xs dark:bg-slate-700 dark:text-white"
                : "text-slate-600 hover:text-slate-900 dark:text-slate-400 dark:hover:text-slate-100"
            )}
          >
            <Laptop className="h-4 w-4" />
            <span>System</span>
          </button>
        </div>
      </div>
    </div>
  );
}
