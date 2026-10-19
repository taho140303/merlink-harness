# S7 — UI correctness and accessibility

- **Status:** pushed
- **Branch:** `refactor/s7-ui-correctness`
- **Units:** F-18, F-19
- **Depends on:** S6
- **Code changes:** `src/lib/characters.ts`, `src/components/CharacterForm.tsx`,
  `src/pages/Characters.tsx`, `src/pages/Home.tsx`,
  `src/components/SettingsDialog.tsx`, `src/App.tsx`,
  `src/lib/preferences.ts` (or a new `src/lib/theme.ts`)

## Goal

Fix the small but user-visible correctness and accessibility defects that remain
after the data/session/provider work: untrusted character storage, a silent
story assignment, a portrait read race, a stale Home count, duplicated theme
logic, and a nested settings modal that claims modality without a focus boundary.

## Evidence

- `src/lib/characters.ts:94-103` checks only `Array.isArray` and casts to
  `Character[]`; `Characters.tsx:37-63` then reads fields and spreads
  `item.tags`, so an older/hand-edited record can crash rendering.
- `src/components/CharacterForm.tsx:62-69` replaces an empty story with
  `projects[0]?.id`, although `characters.ts:83-91` explicitly supports
  "Unassigned story".
- `src/components/CharacterForm.tsx:38-53` lets two `FileReader`s resolve in any
  order; the last `onload` wins even if it is the older selection.
- `src/pages/Home.tsx:13-35` counts only `mockCharacters`, while
  `Characters.tsx:12-26` includes custom characters.
- `src/App.tsx:75-89` and `src/components/SettingsDialog.tsx:137-155` duplicate
  theme read/apply logic; `preferences.ts` already owns text scale, so theme is
  the odd one out.
- `src/components/SettingsDialog.tsx:171-193` focuses the outer close button;
  `:638-653` marks the nested provider editor `aria-modal="true"` but never
  focuses/traps inside it or restores focus. `src/components/Dialog.tsx:23-79`
  already has a working trap that can be extracted/reused.

## Decisions (locked)

1. **Validated character storage, no destructive rewrite.** Add a small
   `normalizeCharacter(raw: unknown): Character | null` in `characters.ts`:
   - require a non-empty string `id`, `name`, `role`;
   - default `story` to `""`, `age`/`gender`/`appearance`/`bio`/`backstory` to
     `""`, `tags`/`relationships` to `[]`, `dialogueCount` to `0`;
   - coerce `tags` only when it is an array of strings; otherwise `[]`;
   - preserve unknown fields via a shallow spread so future data is not erased.
   `readCustomCharacters()` maps through it and drops `null`s. Do not write the
   normalized array back to localStorage on read.
2. **Story is optional.** Use `story: form.story` in the created character (may
   be `""`); keep the "Unassigned story" display path. Do not change the form's
   empty option label. Existing records are not migrated.
3. **Portrait race.** Keep a `readerRef`/`uploadGenerationRef`; abort the previous
   `FileReader` when a new file is chosen; in `onload`, commit only when the
   generation still matches and the form is mounted. Keep the existing MIME/size
   validation before reading.
4. **Home count is shared.** Derive the character metric from
   `readCustomCharacters().length + mockCharacters.length` (memoized) so a newly
   created character is counted. Leave the project/word metrics on the demo
   fixtures, but add a short comment that Projects/Home metrics are demo data
   until a real project read-model exists. Do not add a project-list command in
   this sprint.
5. **One theme module.** Extract `readTheme`, `applyTheme`, and a
   `subscribeSystemTheme` helper into `src/lib/theme.ts` (or extend
   `preferences.ts`). `App.tsx` and `SettingsDialog.tsx` both use it; remove the
   duplicate DOM/localStorage code. Preserve `localStorage["merlink-theme"]`
   values (`light|dark|system`) and the `data-theme` + `dark` class behavior.
6. **Nested modal focus.** Implement a local focus boundary for the provider
   editor:
   - store the opener element; focus the first focusable element (or the panel)
     on open;
   - trap Tab within `editorRef` while it is the topmost layer;
   - restore focus to the opener on close;
   - keep Escape closing only the editor when it is open;
   - do not change `document.body.style.overflow` from the nested editor (the
     parent Settings already owns it).
   If extracting the existing `Dialog` trap into a shared hook is cleaner, do it
   without changing `Dialog`'s behavior.
7. **No component rewrite.** `Chat.tsx`/`SettingsDialog.tsx` remain large in this
   sprint; a full decomposition is deferred because there is no frontend test
   runner. Fix behavior, not file length.

## Tasks

1. `characters.ts`: normalizer + safe reader; add a small pure test only if a TS
   test runner exists (it does not — do not add one). Verify by `pnpm build` and
   a manual bad-record trace.
2. `CharacterForm.tsx`: story + portrait generation/abort.
3. `Home.tsx`: shared character count + demo comment.
4. `theme.ts`: extract and wire App/Settings; delete duplicates.
5. `SettingsDialog.tsx`: focus trap/restore for the editor; ensure the Escape
   handler does not close both layers.
6. `Chat.tsx`: if a send is attempted with no provider (Enter path), set a short
   error instead of returning silently. Do not add a new visual component.

## Explicitly out of scope

- `useChatSession`/Settings component extraction (F-20) — deferred until frontend
  behavior has a test seam.
- A project list/read-model, real Home metrics, or session resume.
- New dependencies, a test framework, i18n, or a visual redesign.
- Changing the character data schema or migrating stored records.

## Acceptance checks

- [ ] `pnpm build` passes.
- [ ] `cargo test --manifest-path src-tauri/Cargo.toml` passes unchanged.
- [ ] A stored array containing `null`, `{}`, `tags: {}`, or a missing `role` is
      filtered/defaulted without a render crash.
- [ ] Creating a character with Story left empty shows "Unassigned story" and is
      not attached to the first demo project.
- [ ] Selecting portrait A then B, with B resolving first, leaves B selected.
- [ ] Adding a custom character increments Home's character metric after
      navigation.
- [ ] Theme changes from Settings still update the document immediately and
      survive reload; system preference still works.
- [ ] Keyboard-only: opening the provider editor focuses inside it, Tab stays in
      it, Escape closes only the editor, and focus returns to the opener.
- [ ] No new dependency and no backend file changed.

## Commit and push

```bash
git switch -c refactor/s7-ui-correctness main
# edit
pnpm build
cargo test --manifest-path src-tauri/Cargo.toml
git add src/lib/characters.ts src/components/CharacterForm.tsx \
        src/pages/Characters.tsx src/pages/Home.tsx \
        src/components/SettingsDialog.tsx src/App.tsx \
        src/lib/theme.ts src/pages/Chat.tsx \
        docs/refactor/sprints/README.md docs/refactor/sprints/07-ui-correctness.md
git commit -m "fix(ui): validate stored characters, keep story optional, and trap modal focus"
git push -u origin refactor/s7-ui-correctness
```

Report the branch, commit SHA, both command results, the focus-trap behavior,
and the exact Home metric source.
