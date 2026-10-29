import type { IntensityLevel } from '../types';
import { INTENSITY_LEVELS } from '../types';
import { ProgressBar } from './ProgressBar';

interface IntensitySliderProps {
  label: string;
  value: IntensityLevel;
  onChange?: (v: IntensityLevel) => void;
}

export function IntensitySlider({ label, value, onChange }: IntensitySliderProps) {
  if (!onChange) return <ProgressBar label={label} level={value} />;

  const idx = INTENSITY_LEVELS.indexOf(value);
  return (
    <div>
      <div className="mb-1 flex justify-between text-xs text-fg-muted ">
        <span>{label}</span>
        <span className="font-semibold text-fg">{value}</span>
      </div>
      <input
        type="range"
        aria-label={label}
        aria-valuetext={value}
        min={0}
        max={4}
        step={1}
        value={idx < 0 ? 0 : idx}
        onChange={(e) => onChange(INTENSITY_LEVELS[Number(e.target.value)])}
        className="w-full accent-indigo-600"
      />
      <div className="mt-0.5 flex justify-between text-xs text-fg-subtle">
        {INTENSITY_LEVELS.map((l) => (
          <span key={l}>{l}</span>
        ))}
      </div>
    </div>
  );
}
