import type { IntensityLevel } from '../types';

const levelConfig: Record<IntensityLevel, { pct: number; color: string; label: string }> = {
  NONE: { pct: 0, color: 'bg-gray-300', label: 'None' },
  LOW: { pct: 25, color: 'bg-green-400', label: 'Low' },
  MODERATE: { pct: 50, color: 'bg-yellow-400', label: 'Moderate' },
  HIGH: { pct: 75, color: 'bg-orange-500', label: 'High' },
  ABSOLUTE: { pct: 100, color: 'bg-red-600', label: 'Absolute' },
};

interface ProgressBarProps {
  label: string;
  level: IntensityLevel;
}

export function ProgressBar({ label, level }: ProgressBarProps) {
  const cfg = levelConfig[level];
  return (
    <div>
      <div className="flex justify-between text-[11px] text-gray-600 mb-1">
        <span>{label}</span>
        <span className="font-medium">{cfg.label}</span>
      </div>
      <div className="h-1.5 w-full bg-gray-100 rounded-full overflow-hidden">
        <div
          className={`h-full ${cfg.color} rounded-full transition-all`}
          style={{ width: `${cfg.pct}%` }}
        />
      </div>
    </div>
  );
}
