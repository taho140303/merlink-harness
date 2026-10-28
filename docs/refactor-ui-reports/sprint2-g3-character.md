# Sprint 2 — G3 Character UI report

## Changes

| File(s) | Main changes |
|---|---|
| `src/core/components/character/CharacterCard.tsx` | Content-card surface, focusable keyboard navigation, token text and badges. |
| `CharacterListView.tsx` | Search/filter use shared Input/Select, token empty state. |
| `CharacterChatPane.tsx` | Composer tokenization, close/send aria-labels, chat card token. |
| `CreateCharacterDialog.tsx` | Shared Dialog retained; controls and accessible description/title migrated. |
| `character-detail/shared/*` | Input treatment, focus ring, slider/progress semantics, token text. |
| `character-detail/MemoryCard.tsx`, `RelationshipCard.tsx` | Card surface, text and control tokens; delete stays immediate (no confirmation added). |
| `character-detail/tabs/*`, `CharacterDetailLayout.tsx` | Content surfaces, token colors and tabs/accessibility updates. |

## Pattern counts (before → after)

| Pattern | Before | After |
|---|---:|---:|
| `text-[...px]` | 24 | 0 |
| `rounded-[` | 0 | 0 |
| `shadow-` | 21 | 0 |
| `bg-white` | 20 | 0 |
| `border-slate-` | 59 | 0 |
| `focus:outline-hidden` | 20 | 0 |
| `neu-` | 0 | 0 |

## Hardcoded strings / primitive gaps

- Toast title uses `t('common.saved')`; no hardcoded “Changes saved” string changed.
- The chat send label is temporarily `Send message`; there is no existing `character.chat.send` i18n key. Proposed key: `character.chat.send`.
- Confirmed shared `DialogDescription`, `Input`, `Select`, `Textarea`, `Button`, `Tabs`, and `Badge` primitives exist. CharacterDetailLayout tab migration remains to be completed if not visible in this report.
- Existing MemoryCard deletion is immediate; behavior was intentionally preserved without adding confirmation.

## TypeScript

`npx tsc --noEmit`: pending execution; report update required with result.

## Notes

No mock data, dictionary, shared UI primitive, or app stylesheet was changed.
