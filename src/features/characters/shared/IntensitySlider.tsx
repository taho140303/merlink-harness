import type { IntensityLevel } from '../types';
import { INTENSITY_LEVELS } from '../types';
import { ProgressBar } from './ProgressBar';
import { useDbEnumLabel } from '@/shared/i18n/dbEnums';

interface IntensitySliderProps {
  label: string;
  value: IntensityLevel;
  onChange?: (v: IntensityLevel) => void;
}

export function IntensitySlider({ label, value, onChange }: IntensitySliderProps) {
  const enumLabel = useDbEnumLabel();
  if (!onChange) return <ProgressBar label={label} level={value} />;

  const idx = INTENSITY_LEVELS.indexOf(value);
  return (
    <div>
      <div className="mb-1 flex justify-between text-xs text-fg-muted ">
        <span>{label}</span>
        <span className="font-semibold text-fg">{enumLabel('RelationDimensionLevel', value)}</span>
      </div>
      <input
        type="range"
        aria-label={label}
        aria-valuetext={enumLabel('RelationDimensionLevel', value)}
        min={0}
        max={4}
        step={1}
        value={idx < 0 ? 0 : idx}
        onChange={(e) => onChange(INTENSITY_LEVELS[Number(e.target.value)])}
        className="w-full accent-indigo-600"
      />
      <div className="mt-0.5 flex justify-between text-xs text-fg-subtle">
        {INTENSITY_LEVELS.map((l) => (
          <span key={l}>{enumLabel('RelationDimensionLevel', l)}</span>
        ))}
      </div>
    </div>
  );
}
