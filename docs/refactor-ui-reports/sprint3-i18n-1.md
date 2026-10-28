# Sprint 3 i18n — Batch 1/3

## Status: PARTIAL

### Completed
- Localized chat UI chrome and ProviderForm labels/placeholders using `t(key)`.
- Added `useT()` to ProviderForm.
- Reconciled `en` and `vi` dictionaries with a script: 492 unique keys in each locale; no empty keys, duplicates, or parity differences.
- TypeScript check: `npx tsc --noEmit` — PASS (exit code 0).
- Dictionary parity check — PASS: en=492, vi=492, duplicates=0 in both, missing en=[], missing vi=[].

### Preserved data (not UI chrome)
- ChatView mock conversation content, thinking logs, and tool output.
- `MOCK_HISTORY` titles and other mock project/history content.
- Provider/model names, model IDs, protocol/API names, and environment-token identifiers.
- Numeric values (progress, context/token counts and limits).
- Product name `Merlink-harness`.

### Remaining
- Perform a complete manual inventory of every user-facing string in all scoped chat/settings/sidebar TSX files and verify route metadata. Existing provider options, endpoint examples, protocol names, and identifiers were intentionally retained as technical/configuration data; scope-wide completion has not been independently audited. Therefore this report is PARTIAL despite passing compilation and locale parity.
- No build or dev server was run.

### Scope files changed
Chat components, `ProviderForm.tsx`, `src/shared/i18n/dict.ts`, and this report. `src/app/routes/chat.tsx` was inspected and left unchanged (metadata is route/SEO metadata, not UI chrome).

