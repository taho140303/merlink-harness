# S4 — Agent-loop truth

- **Status:** pushed
- **Branch:** `refactor/s4-agent-loop-truth`
- **Units:** F-05, F-06, F-07
- **Depends on:** S3
- **Code changes:** `src-tauri/src/runtime.rs`, `src-tauri/src/models.rs` (only if
  an event shape must change), `src/pages/Chat.tsx` (error reload only)

## Goal

Never tell the user something the durable transcript does not support. A turn
that wrote tools and then failed must become visible as an interrupted turn; a
tool step must only be reported finished after its result is stored; and the
5-round cap must not recycle pre-tool prose or stale project metadata.

## Evidence

- `src-tauri/src/runtime.rs:343-353` returns only `Err` on a later model failure,
  although `:370-377` and `:460-465` have already persisted messages/tool results.
- `src/pages/Chat.tsx:489-506` shows an error and keeps only the user bubble; the
  next turn sends an incomplete UI transcript.
- `src-tauri/src/runtime.rs:429-465` emits `ToolFinished { ok: true }` before the
  result is appended; a persistence failure leaves a "successful" UI step with no
  durable result.
- `src-tauri/src/runtime.rs:474-493` appends the pre-tool `last_content` plus
  `STOP_MARKER` after the 5th round; `:481-489` uses the pre-call
  `effective_project_id` for the stop message's metadata while `:492-504` re-reads
  the project for the event and return value.

## Decisions (locked)

1. **Keep the 5-round cap.** Do not add a 6th model call, a retry loop, or a new
   cancellation API in this sprint.
2. **Interrupted turns are reloaded, not invented.** On a provider error after
   the turn has started, the backend keeps the existing `Err` return (the IPC
   shape does not change). The frontend, on error, reloads the authoritative
   transcript with the existing `list_messages` command and refreshes
   progress/context. If the reload fails, keep the current error and do not
   fabricate messages.
3. **Event after persistence.** Move the tool-result `append_message` before the
   `ToolFinished` emit. On append failure, emit `ToolFinished { ok: false }` with
   the same status and return an error that says the tool ran but its result
   could not be stored. Do not claim the tool failed silently.
4. **Cap message is system-authored.** Remove `last_content` recycling. The stop
   message content is exactly `STOP_MARKER` (keep the constant string so existing
   clients/tests recognize it). The accumulated reasoning may still attach.
5. **Project metadata is resolved once after the final round.** After the tool
   loop, call `effective_project` once and use that value for:
   - the stop message `build_meta(... project_id ...)`,
   - the `TurnFinished` event,
   - the returned `ChatTurn.project_id`.
   The prompt hash still describes the prompt that was actually sent; do not
   recompute it.
6. **No new `ChatTurn` field.** Do not add `interrupted`/`error` to the IPC shape
   in this sprint; `list_messages` is the recovery path.
7. **Tool failures remain tool results.** A failing `tools::execute_tool` still
   produces a `role: tool` result with an `error` key and `ok: false`; do not turn
   it into a turn-level error.

## Tasks

1. `runtime.rs`:
   - In the per-call loop, build the tool result, append it, then emit
     `ToolFinished`; handle the append error as described.
   - Remove `last_content` and the pre-tool prose concatenation.
   - Re-read `effective_project_id` after the loop and use it consistently for
     the stop message/event/return.
   - Keep `RoundFinished` after the tools; keep `TurnFinished` last on every
     non-error path.
2. `Chat.tsx`:
   - Add `reloadSessionMessages(sessionId)` using
     `safeInvoke<MessageEnvelope[]>("list_messages", { sessionId })`.
   - In the `catch` of `sendMessage`, after setting the error, reload the
     transcript for the current turn's session, and refresh progress/context. All
     of this must respect S1's epoch guard.
   - Do not clear the error just because the reload succeeded; the turn still
     failed.
3. Tests in `runtime.rs`:
   - **Partial failure:** scripted backend returns one tool call, then an error.
     Assert `run_agent_turn` returns `Err`, and `db::list_messages` contains the
     user message, the assistant tool-call message, and the tool result (all
     persisted) for that session.
   - **Cap truth:** scripted 5 tool-call rounds whose last reply has prose.
     Assert the final message content is exactly `STOP_MARKER`, does **not**
     contain the last reply's prose, and the final message `meta.project_id`
     equals the returned `ChatTurn.project_id` after a `project_create` in the
     last round. Use a scripted backend/tool arrangement that can create a
     project in round 5.
   - **Event order:** with a recording sink, assert the `ToolFinished` event for
     a call arrives after a `db::list_messages` read can see its result. If that
     is hard to observe without a hook, assert the implementation order in a
     comment and keep the existing event-count tests.
4. Update existing tests that assert the cap message contains prior text.

## Explicitly out of scope

- Provider HTTP retries/backoff, cancellation, a turn-state table, or a
  `Command`/`Memento` hierarchy.
- Changing `MAX_TOOL_ROUNDS`, the prompt, the tool set, or the 70/30 formula.
- UI redesign of the error banner; only the reload behavior is in scope.

## Acceptance checks

- [ ] `cargo test --manifest-path src-tauri/Cargo.toml` passes.
- [ ] `pnpm build` passes.
- [ ] A tool-then-provider-error turn leaves the tool result in SQLite and the UI
      reloads it (manual trace for the UI half).
- [ ] The cap message is exactly `STOP_MARKER` and carries the post-tool project
      ID.
- [ ] No new IPC field and no new dependency.
- [ ] `ToolFinished` is emitted only after the result append on the success path.

## Commit and push

```bash
git switch -c refactor/s4-agent-loop-truth main
# edit
cargo test --manifest-path src-tauri/Cargo.toml
pnpm build
git add src-tauri/src/runtime.rs src/pages/Chat.tsx \
        docs/refactor/sprints/README.md docs/refactor/sprints/04-agent-loop-truth.md
git commit -m "fix(agent): report only durable turns and stop recycling capped prose"
git push -u origin refactor/s4-agent-loop-truth
```

Report the branch, commit SHA, both command results, the partial-failure test
output, and the exact cap message text.
