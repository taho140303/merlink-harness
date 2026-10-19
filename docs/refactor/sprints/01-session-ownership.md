# S1 — Session and turn ownership (frontend)

- **Status:** pushed
- **Branch:** `refactor/s1-session-ownership`
- **Units:** F-01 (frontend half), F-02
- **Depends on:** S0
- **Code changes:** `src/pages/Chat.tsx`, new `src/lib/ids.ts`

## Goal

Make every asynchronous chat operation belong to the session/turn that started
it. Pressing **New chat** must detach the old turn, not let it repaint the new
one, and every read/stream/completion must be discarded if its captured epoch is
no longer current.

## Evidence

- `src/App.tsx:134-142` dispatches `merlink:new-chat` regardless of activity.
- `src/pages/Chat.tsx:394-411` resets messages/session/project/live state but not
  the running request or `loading`.
- `src/pages/Chat.tsx:458-461` writes every channel event into the current live
  frame; `:467-488` unconditionally appends the old reply and old project;
  `:489-497` can show the old error; `:498-506` clears the current loading state.
- `src/pages/Chat.tsx:252-312` reads (`super_json_read`, `super_json_summary`,
  `super_json_progress`, `context_usage`) commit after their awaits without
  checking the session/project that started them.
- `src/pages/Chat.tsx:446-450` creates a user message ID from `Date.now()` only.

## Decisions (locked)

1. **One session epoch.** Add `sessionEpochRef: useRef(0)` and
   `activeTurnRef: useRef<{ epoch: number; sessionId: string } | null>(null)`.
   Increment the epoch and clear the active turn in `handleNewChat`, on unmount,
   and before starting a new send. A completion is current only when both the
   captured epoch and the captured session id still match.
2. **Detach, do not disable.** New chat remains usable while a turn is running.
   Set `loading` false on reset and let the old request finish in the background.
   Do not pretend the backend was cancelled; tools may already have run. The old
   `finally` must not clear the new turn's loading flag.
3. **Channel guard.** The channel handler checks the captured epoch before
   `setLiveSteps`; a stale event is dropped entirely (including `round_finished`
   accumulating `modelMs` for the new turn).
4. **Success/error/finally guard.** Before any `setMessages`, `setProjectId`,
   `setTurnStats`, `setError`, `refreshDetail`, or `refreshProgress`, compare the
   captured epoch/session with the current refs. The `finally` block only clears
   `loading` and live state when the turn is still current.
5. **Keyed reads.** `refreshSuperJson(id)`, `refreshSummary(id)`,
   `refreshProgress()`, and `refreshContextUsage()` capture the current
   session/project (and provider/model for context usage) at call time and check
   those values again after `await` before setting state. Use refs mirrored from
   state, not stale closures. A response for an old project/session is ignored.
6. **Message IDs.** Add `src/lib/ids.ts`:
   - `newSessionId()` — `sess-<uuid>` with a `crypto.randomUUID()` path and a
     `crypto.getRandomValues`/`Date.now`+random fallback.
   - `newMessageId(prefix = "msg")` — `<prefix>-<uuid>` with the same fallback.
   Use `newMessageId("msg")` for the user message in `sendMessage`; keep
   `newSessionId()` for the chat session. Remove the local `newSessionId` from
   `Chat.tsx`. Do not change the ID format of backend-generated messages.
7. **No provider refresh in this sprint.** Settings→Chat invalidation belongs to
   S5; do not add an event bus here.

## Tasks

1. Create `src/lib/ids.ts`; use the Web Crypto API, no dependency. The fallback
   must not use `Math.random()` alone: combine `Date.now()` with
   `crypto.getRandomValues` when available, and a counter as the last resort.
2. Refactor `sendMessage` in `src/pages/Chat.tsx`:
   - Capture `const epoch = sessionEpochRef.current` and `const turnSessionId =
     sessionId` before the first `setState`.
   - Set `activeTurnRef.current = { epoch, sessionId: turnSessionId }`.
   - Wrap the channel callback and every post-await branch with one helper
     `isCurrentTurn(epoch, turnSessionId)`.
   - On `handleNewChat`, increment the epoch, set `activeTurnRef.current = null`,
     `setLoading(false)`, and clear live state as today.
   - Keep the existing one-send-path behavior: form and outline both use
     `sendMessage`.
3. Add a `useEffect` cleanup that increments the epoch on unmount. Do not call
   `setState` after unmount.
4. Guard the four read functions with captured identity; if a request is
   superseded, ignore both success and error.
5. Keep the existing `merlink:new-chat` listener and its cleanup; do not replace
   it with a new global bus in this sprint.
6. Add a short comment block at the top of the ownership logic explaining the
   epoch rule and the "detach, do not cancel" decision.

## Explicitly out of scope

- Backend/DB changes (S2), project creation (S3), agent-loop error recovery (S4),
  provider synchronization (S5), IPC typing/preview (S6), component extraction.
- No new dependency, no test framework, no CSS/visual change.

## Acceptance checks

- [ ] `pnpm build` passes.
- [ ] `cargo test --manifest-path src-tauri/Cargo.toml` passes unchanged.
- [ ] `grep -n "Date.now()" src/pages/Chat.tsx` shows no user-message ID creation
      (other timing uses such as drafts/history may remain).
- [ ] Manual source trace: with a delayed `send_chat`, pressing New chat makes
      every old event/reply/error/`finally` a no-op on the new chat; a new send
      in the new chat has its own epoch and is not blocked by the old `loading`.
- [ ] Manual source trace: a slow `super_json_progress`/detail/context read from
      the old session cannot set state after New chat.
- [ ] No backend file changed.

## Commit and push

```bash
git switch -c refactor/s1-session-ownership main
# edit
pnpm build
cargo test --manifest-path src-tauri/Cargo.toml
git add src/pages/Chat.tsx src/lib/ids.ts docs/refactor/sprints/README.md \
        docs/refactor/sprints/01-session-ownership.md
git commit -m "fix(chat): scope async turns and reads to their session epoch"
git push -u origin refactor/s1-session-ownership
```

Report the branch, commit SHA, both command results, the exact epoch guard
locations, and any read path that still cannot be guarded without a backend
change (there should be none for the four listed reads).
