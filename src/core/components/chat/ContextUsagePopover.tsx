import { Popover } from "../../../shared/components/ui/popover";
import { useT } from "../../../shared/i18n/useT";

export function ContextUsagePopover() {
  const t = useT();
  const percent = 7;
  const radius = 6;
  const circumference = 2 * Math.PI * radius;
  const strokeDashoffset = circumference - (percent / 100) * circumference;

  return (
    <Popover
      label={t("chat.contextUsage")}
      align="end"
      side="top"
      trigger={
        <span className="flex items-center gap-1.5 rounded-lg px-2 py-1 text-xs font-medium text-fg-muted hover:bg-surface-muted hover:text-fg transition-colors">
          <svg aria-hidden="true" className="h-4 w-4 -rotate-90" viewBox="0 0 16 16">
            <circle cx="8" cy="8" r={radius} className="stroke-line" strokeWidth="2" fill="transparent" />
            <circle cx="8" cy="8" r={radius} className="stroke-accent transition-colors duration-300" strokeWidth="2" strokeDasharray={circumference} strokeDashoffset={strokeDashoffset} strokeLinecap="round" fill="transparent" />
          </svg>
          <span className="text-xs font-semibold">{percent}%</span>
        </span>
      }
    >
      <div className="w-72 p-1.5">
        <div className="flex items-center justify-between text-xs">
          <span className="font-semibold text-fg">
            {percent}% <span className="font-normal text-fg-muted">{t("chat.contextUsed")}</span>
          </span>
          <span className="text-xs text-fg-subtle">~67.7K / 1M</span>
        </div>

        <div className="mt-2.5 flex h-1.5 w-full overflow-hidden rounded-full bg-surface-muted">
          <div style={{ width: "1%" }} className="bg-fg-subtle" />
          <div style={{ width: "2%" }} className="bg-purple-500" />
          <div style={{ width: "4%" }} className="bg-blue-500" />
        </div>

        <div className="mt-3.5 space-y-2 text-xs">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-2"><span className="h-2 w-2 rounded-xs bg-fg-subtle" /><span className="text-fg-muted">{t("chat.context.systemPrompt")}</span></div>
            <span className="text-fg-subtle">~2.3K</span>
          </div>
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-2"><span className="h-2 w-2 rounded-xs bg-purple-500" /><span className="text-fg-muted">{t("chat.context.toolDefinitions")}</span></div>
            <span className="text-fg-subtle">~11.7K</span>
          </div>
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-2"><span className="h-2 w-2 rounded-xs bg-blue-500" /><span className="text-fg-muted">{t("chat.context.messages")}</span></div>
            <span className="text-fg-subtle">~133K</span>
          </div>
        </div>
      </div>
    </Popover>
  );
}
