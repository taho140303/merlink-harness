# Sprint 5 hotfix: composer focus ring

## Root cause

In `src/app/app.css`, `.focus-ring:focus-visible` set `border-radius: inherit`. This forced the focused element to inherit its parent’s radius (often zero), making the outline appear square even when the composer itself has rounded corners. The rule was unnecessary because the outline follows the focused element’s own border radius.

## Change

Removed only `border-radius: inherit;` from `.focus-ring:focus-visible`. Kept `outline: 2px solid var(--color-accent);` and `outline-offset: 2px;` unchanged. No component or other styling changes.

## Verification

- `npx tsc --noEmit`: PASS (exit 0)
- `npm run lint`: PASS (exit 0)
- `npm run build`: PASS (exit 0; client and SSR builds completed)
- Visual check: opened `http://localhost:5199/`, focused the chat textbox (`Tin nhắn`), and captured `/tmp/sprint5-focus-ring.png`. Screenshot capture succeeded; direct visual inspection was unavailable in this agent environment because the active model does not support image input.
