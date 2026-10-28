# Sprint 4 — Accessibility

**Status:** Partial completion · **Scope:** chat (excluding `ChatView.tsx`), draft, project (excluding `ProjectDetailView.tsx`), character, character-detail (excluding `CharacterDetailLayout.tsx`), sidebar.

## Changes

| File | Changes |
|---|---|
| `chat/ChatHeader.tsx` | Draft panel toggle reports `aria-pressed`. Progress meter already had accessible progressbar role/name/value. |
| `chat/ChatMessageItem.tsx` | Thinking disclosure reports `aria-expanded`; chevron icons marked decorative. Copy button already had accessible name/title. |
| `draft/DraftHeader.tsx` | Close/collapse icon button gets accessible name. |
| `draft/DraftToolbar.tsx` | Version selector reports `aria-expanded`; action buttons have accessible labels. |
| `draft/DraftOverviewBlock.tsx` | Genre overflow button has accessible name; filter tabs report pressed state; genre dialog includes a description. |
| `draft/overview/ArcCard.tsx` | Accordion state exposed with `aria-expanded`; focus ring added to trigger. |
| `draft/overview/ChapterBlock.tsx` | Chapter and nested scene disclosures report `aria-expanded`; focus rings added to triggers. |
| `draft/overview/SceneConfigBlock.tsx` | Arc/chapter disclosure states exposed with `aria-expanded`; focus rings added. |
| `draft/overview/SegmentBlock.tsx` | Main disclosure, view toggle pressed states, and show-more expanded state exposed; focus rings added. |
| `draft/overview/CharacterDetailDialog.tsx` | Dialog description added; tablist, tabs, selected state and roving tabindex added. |
| `character/CreateCharacterDialog.tsx` | Proper `DialogTitle` and `DialogDescription` replace plain heading/paragraph. |
| `character-detail/MemoryCard.tsx` | Emotion range control named and value text provided. |
| `character-detail/shared/IntensitySlider.tsx` | Slider accessible name and value text added. |
| `character-detail/tabs/CoreIdentityTab.tsx` | Accordion expanded state exposed; plus/minus ornament hidden from assistive technology. |

## Accessible names observed

Existing named controls include ChatHeader progress-details icon, ChatInputBar clear/send controls and labeled message input, ChatMessageItem copy action, character chat close/send, project chat close/send, project sidebar close, sidebar expand/collapse/history controls, sidebar project-group switch, worldbuilding disclosure, and tool/model popovers (which use the shared Popover label). Text-labelled controls in draft, project and character flows retain their visible names. Images found in reviewed scope already have meaningful alt text or `alt=""` when decorative. Dialogs reviewed now include titles; relevant dialogs also include descriptions.

## Remaining / limitations

- This pass did not eliminate every missing focus ring or add state semantics to every filter/toggle; further audit is recommended. In particular some older character detail and project controls remain untouched.
- Existing icon-only buttons in several in-scope files appear to have title-only accessible text or need follow-up review for aria-label, such as project card delete, character detail remove controls, and some legacy project/character actions.
- `SidebarHistory` uses the correct switch `role="switch"` with `aria-checked`; no `aria-pressed` needed.
- No UI text, state logic, or layout was intentionally changed.

## Checks

- `npx tsc --noEmit` → **0 errors** (latest run).
- `npm run lint` → **0 errors** (run after initial accessibility edits; rerun recommended after latest dialog/tab edits).
- No build or dev server started.
