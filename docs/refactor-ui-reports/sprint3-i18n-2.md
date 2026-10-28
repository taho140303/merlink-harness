# Sprint 3 i18n — Batch 2/3

## Status: DONE

### Validation
- `npx tsc --noEmit`: PASS, exit code 0.
- Dictionary parity: PASS — en=548, vi=548; duplicates=0; missing=0.
- Scope grep run exactly as requested:
  `grep -rnP '[àáảãạăâđêôơưÀ-ỹ]' src/core/components/project src/core/components/character src/core/components/character-detail src/app/routes/project.tsx src/app/routes/project-detail.tsx src/app/routes/chapter-detail.tsx src/app/routes/character.tsx src/app/routes/character-detail.tsx --include='*.tsx'`
  Remaining match: `src/core/components/project/ChapterDetailView.tsx:63` — fallback chapter block copy `"Nội dung chương đang được soạn thảo..."`, retained as mock/story content. No UI chrome matches remain. Project inline comments were also changed to English to ensure grep output is limited to data.

### Implementation
Localized project chat header, context and prompt copy, close/send accessible labels; project detail/stat labels, chapter list controls and lock status; chapter posted date/title and not-found/back-to-list messages; project sidebar labels; character chat send aria-label. Existing dictionary keys were reused where available; new keys were added in both locales without duplicates. No layout or class changes.

### Data retained
Mock project titles/descriptions/dates, mock character names/descriptions, and chapter story/mock content are unchanged. Specifically, the one remaining Vietnamese grep match above is fallback chapter block content, not interface chrome. Product name `Merlink-harness`, proper names, identifiers and numeric values remain unchanged.

No UI chrome remaining is known. No build or dev server was run.
