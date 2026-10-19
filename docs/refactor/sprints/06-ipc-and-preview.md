# S6 — IPC boundary and browser preview

- **Status:** blocked by S5
- **Branch:** `refactor/s6-ipc-preview`
- **Units:** F-13, F-14, F-15, F-16
- **Depends on:** S5
- **Code changes:** `src/lib/tauri.ts`, new `src/lib/ipc.ts`, new
  `src/lib/browser-mock.ts`, `src/pages/Chat.tsx`, `src/components/SettingsDialog.tsx`,
  `src-tauri/src/lib.rs`, `src-tauri/src/runtime.rs`, `src-tauri/src/models.rs`

## Goal

Make the IPC boundary honest and typed, make the browser preview a small
session-scoped test double instead of a parallel backend, guard the direct
`execute_tool` command, and delete policy-free forwarding/dead types. This is the
"delete more than you add" sprint.

## Evidence

- `src/lib/tauri.ts:792-844` exposes `safeInvoke<T>(cmd: string, args?: unknown)`
  and casts arbitrary results to `T`; every call site repeats the shape. A wrong
  command name or `project_id` vs `projectId` compiles.
- `src/lib/tauri.ts:48-790` is a second stateful backend: globals
  `browserValues`/`browserProjectId`/`browserMessages` (`:146-159`), a mock send
  loop (`:582-790`), and a `super_json_missing` that always returns `[]`
  (`:561-564`) despite `mockMissingRequired()` (`:161-177`).
- `src-tauri/src/lib.rs:125-133` exposes `execute_tool` with caller-supplied
  `project_id` and `session_id`; `src-tauri/src/tools.rs:221-230` checks only that
  the project string is non-empty.
- `src-tauri/src/runtime.rs:507-530` has three policy-free forwarding functions;
  `src-tauri/src/models.rs:69-81` defines `ToolInvocation`, used only to format one
  log line (`runtime.rs:439-451`).
- The contract tests (`src-tauri/src/ipc_contract_tests.rs`) are the authority
  for camelCase keys and must keep passing.

## Decisions (locked)

1. **One typed command map in `src/lib/ipc.ts`.** Keep `safeInvoke` as the raw
   wire edge (not exported to pages), and expose typed helpers:
   ```ts
   export interface CommandMap {
     load_providers: { args?: never; result: ProviderConfig[] };
     save_providers: { args: { providers: ProviderConfig[] }; result: void };
     send_chat: { args: SendChatArgs; result: ChatTurn };
     list_messages: { args: { sessionId: string }; result: MessageEnvelope[] };
     super_json_read: { args: { projectId: string }; result: unknown };
     super_json_progress: { args: { sessionId: string }; result: SuperJsonProgress };
     super_json_summary: { args: { projectId: string }; result: string };
     super_json_labels: { args?: never; result: Record<string, string> };
     context_usage: { args: { sessionId: string; providerId: string; modelId: string }; result: ContextUsage };
     fetch_models: { args: { providerKind: string; baseUrl: string; apiKey: string }; result: FetchedModel[] };
     execute_tool: { args: { toolName: string; args: Record<string, unknown>; projectId: string; sessionId: string }; result: unknown };
   }
   export function invokeCommand<K extends keyof CommandMap>(
     cmd: K,
     ...args: CommandMap[K]["args"] extends never ? [] : [CommandMap[K]["args"]]
   ): Promise<CommandMap[K]["result"]>;
   ```
   `SendChatArgs.channel` must be typed as `AgentEventChannel | Channel<AgentEvent>`.
   Migrate all `safeInvoke` call sites (`Chat.tsx`, `SettingsDialog.tsx`) to
   `invokeCommand`. Keep the raw `safeInvoke` only inside `ipc.ts`.
2. **Extract the preview.** Move the mock data/state/functions to
   `src/lib/browser-mock.ts`; export `browserInvoke(cmd, args): Promise<unknown>`
   and `isTauri` stays in `tauri.ts`. `tauri.ts` becomes transport + channel +
   dispatch only. No page imports `browser-mock`.
3. **Session-scoped preview.** Replace the three globals with
   `Map<sessionId, MockSession>` where `MockSession` holds `projectId`,
   `messages`, `values` (project path map), and `unnamedCount`. Pass the real IPC
   arguments through:
   - `send_chat` uses `args.sessionId`/`args.projectId`; a new chat gets its own
     session and project; creating a second project in the same mock session is
     rejected like the native path.
   - `list_messages` returns only that session's messages.
   - `super_json_read`/`summary` use `args.projectId`; an unknown project returns
     an empty document/placeholder, not another session's.
   - `super_json_progress` uses `args.sessionId`; unbound returns `has_project:
     false`.
   - `super_json_missing` uses `mockMissingRequired(session.values)`, never `[]`.
   - `execute_tool` uses the supplied session/project; `project_create` binds only
     that session.
   Do not recreate a second full validator; keep the existing mock path set and
   label it as a preview subset in a comment.
4. **Guard `execute_tool`.** In `runtime::execute_tool_internal`, for
   `super_json_get|set|missing`:
   - resolve the bound project with `db::project_for_session(session_id)`;
   - if unbound → `Err("This chat is not bound to a project...")`;
   - if the supplied `project_id` is non-empty and differs from the binding →
     `Err("Project does not belong to this session")`;
   - otherwise use the bound project ID.
   `project_create` and `echo` keep their current behavior (bootstrap must work
   for an unbound session). Do not delete the command: the IPC contract tests
   exercise it. Use `tools::execute_tool` unchanged.
5. **Delete policy-free wrappers.** `lib.rs` calls `adapters::fetch_models` and
   `super_json::load_or_init` directly; remove `runtime::fetch_models_internal`
   and `runtime::super_json_document`. Keep `execute_tool_internal` because it now
   owns the session guard. Move the two runtime tests that call
   `fetch_models_internal` to `adapters::fetch_models`.
6. **Delete `ToolInvocation`.** Log the local fields directly in `runtime.rs`;
   remove the struct from `models.rs` and any imports. Do not replace it with
   another nominal wrapper.
7. **No new dependency and no command rename.** The `merlink_handlers!` list and
   the camelCase contract stay unchanged.

## Tasks

1. Add `src/lib/ipc.ts` with the map, `invokeCommand`, and `SendChatArgs`.
2. Move mock code to `src/lib/browser-mock.ts`; rewrite the state to per-session;
   export `browserInvoke`; keep `BROWSER_FETCHED_MODELS`, `MOCK_PATH_LABELS`, and
   the preview providers. Remove the globals and the `safeInvoke` switch from
   `tauri.ts`.
3. Migrate `Chat.tsx`/`SettingsDialog.tsx` imports/calls; run `pnpm build` and fix
   every type error (this is the point of the typed map).
4. `runtime.rs`: add the guard, delete the two wrappers, delete `ToolInvocation`
   usage/log directly, and update tests.
5. `lib.rs`: call the owning modules directly.
6. Add Rust unit tests for the guard if a DB-only test is practical; otherwise
   extend `ipc_contract_tests.rs` with a case that passes a different
   `projectId` for a bound session and expects the guard error.
7. Manual preview trace (no test framework): two sessions in one page, each with
   its own project/messages/progress; `super_json_missing` reports real missing
   paths after a partial fill.

## Explicitly out of scope

- A code-generation pipeline, JSON-schema codegen, or a new IPC framework.
- Deleting the preview entirely; interactive browser preview stays.
- Persisted session list/resume (product feature).
- Changing the Rust command names, argument casing, or error strings that the
  contract tests assert (except the new guard error, which needs its own test).

## Acceptance checks

- [ ] `cargo test --manifest-path src-tauri/Cargo.toml` passes, including all IPC
      contract tests.
- [ ] `pnpm build` passes with the typed command map; no `safeInvoke` call remains
      outside `src/lib/ipc.ts` and `src/lib/tauri.ts`.
- [ ] `execute_tool("super_json_set", projectId=B, sessionId=bound-to-A)` returns
      the guard error and does not touch B.
- [ ] Browser preview: two sessions have independent project/message/progress
      state; `super_json_missing` is honest.
- [ ] `ToolInvocation` and the two policy-free wrappers no longer exist.
- [ ] `src/lib/tauri.ts` is transport/typing only (roughly < 120 lines).
- [ ] No new dependency.

## Commit and push

```bash
git switch -c refactor/s6-ipc-preview main
# edit
pnpm build
cargo test --manifest-path src-tauri/Cargo.toml
git add src/lib/ipc.ts src/lib/browser-mock.ts src/lib/tauri.ts \
        src/pages/Chat.tsx src/components/SettingsDialog.tsx \
        src-tauri/src/lib.rs src-tauri/src/runtime.rs src-tauri/src/models.rs \
        src-tauri/src/ipc_contract_tests.rs docs/refactor/sprints/README.md \
        docs/refactor/sprints/06-ipc-and-preview.md
git commit -m "refactor(ipc): type the command boundary and scope the preview by session"
git push -u origin refactor/s6-ipc-preview
```

Report the branch, commit SHA, both command results, the guard error text, the
new `tauri.ts` line count, and any preview behavior that intentionally remains
less complete than the native backend.
