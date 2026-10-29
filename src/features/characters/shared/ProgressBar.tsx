import type { IntensityLevel } from '../types';

const levelConfig: Record<IntensityLevel, { pct: number; color: string; label: string }> = {
  NONE: { pct: 0, color: 'bg-line-strong ', label: 'None' },
  LOW: { pct: 25, color: 'bg-emerald-400', label: 'Low' },
  MODERATE: { pct: 50, color: 'bg-yellow-400', label: 'Moderate' },
  HIGH: { pct: 75, color: 'bg-orange-500', label: 'High' },
  ABSOLUTE: { pct: 100, color: 'bg-rose-600', label: 'Absolute' },
};

interface ProgressBarProps {
  label: string;
  level: IntensityLevel;
}

export function ProgressBar({ label, level }: ProgressBarProps) {
  const cfg = levelConfig[level];
  return (
    <div>
      <div className="mb-1 flex justify-between text-xs text-fg-muted ">
        <span>{label}</span>
        <span className="font-medium">{cfg.label}</span>
      </div>
      <div className="h-1.5 w-full overflow-hidden rounded-full bg-surface-muted ">
        <div
          className={`h-full ${cfg.color} rounded-full transition-colors`}
          style={{ width: `${cfg.pct}%` }}
        />
      </div>
    </div>
  );
}
