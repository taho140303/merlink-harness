# Sprint 3 i18n — Batch 3/3

## Status: PARTIAL

### Implementation
- Routed remaining draft UI chrome through `t(key)` and added bilingual keys for the feedback action, scene/chapter counts and scene title, character detail headings/labels, eye description, memory source, version selector, and draft-panel collapse title.
- Reused existing keys for `draft.char.viewDetails`; removed the hardcoded fallback.
- `src/app/routes/chat.tsx` contains route metadata only; left unchanged.

### Validation
- `npx tsc --noEmit`: FAIL (exit code 2), unrelated pre-existing syntax error in `src/core/components/settings/ProviderForm.tsx:373`: expected corresponding JSX closing tag for `Button`.
- Dictionary parity: PASS — en=570, vi=570; duplicates=0; missing=0.
- Draft TSX Vietnamese grep: remaining matches are comments and enum/data values; the data enum values in `ThemeBlock.tsx` are compared to map existing translated labels and were left intact.
- UI-copy grep found only the parameterized `draft.char.source` call; no other English/Vietnamese UI hardcode in listed areas.

### Preserved data
- Mock overview and story content, character names, descriptions/lore, scene/chapter names and summaries, values such as gender, memory type and character attributes remain data and were not translated.
- Existing enum strings `Chính`, `Phản đề`, `Hành động`, and `Lời nói` remain comparison inputs; displayed labels use existing `t()` mappings.

### Scope caveat
- Worktree already contained unrelated modifications before this batch, including `eslint.config.js`, `package.json`, files outside the authorized scope, and reports from prior batches. This batch changed only the draft TSX files listed above plus `src/shared/i18n/dict.ts` and this report.
