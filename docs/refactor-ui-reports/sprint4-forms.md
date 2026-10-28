# Sprint 4 — Forms primitives

## API
- `Checkbox`: native checkbox with `label`, optional `description`, native input attributes including `checked`, `onChange`, and `disabled`; styled with `tier-input focus-ring`, 4×4 sizing, rounded corners, and accent color.
- `Switch`: `checked`, `onCheckedChange`, required `ariaLabel`, optional `label`, `disabled`, and `className`; button switch with `aria-checked`, focus ring, 5×9 track, and surface-card knob.
- `Segmented`: `items: { id, label, icon? }[]`, `value`, `onChange`, `ariaLabel`, optional `className`; radiogroup/radio semantics, roving tabindex, ArrowLeft/Right and Home/End navigation.

## Files changed
- Added `src/shared/components/ui/checkbox.tsx`, `switch.tsx`, and `segmented.tsx`.
- Updated `src/core/components/settings/ProviderForm.tsx` to use Checkbox for text/image and Switch for reasoning, preserving existing state updates and visible labels.
- Updated `src/core/components/settings/GeneralTab.tsx` to use Segmented for theme and language with existing icons and translation keys.

## Validation
- `npx tsc --noEmit`: **failed** due to pre-existing syntax errors in out-of-scope `src/core/components/draft/overview/SegmentBlock.tsx` (lines 129–139: TS1003, TS1382, TS17002, TS1005, TS1128, TS1109). No errors were reported in Sprint 4 files before compilation stopped.
- `npm run lint`: **failed** due to the same pre-existing parsing error in out-of-scope `src/core/components/draft/overview/SegmentBlock.tsx:131`. Initial run found the opaque-white knob token in `switch.tsx`; it was changed to `bg-surface-card`.

## Notes
No dict changes or dependencies added. No build/dev server run.
