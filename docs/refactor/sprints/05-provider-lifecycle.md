# S5 — Provider lifecycle

- **Status:** blocked by S4
- **Branch:** `refactor/s5-provider-lifecycle`
- **Units:** F-08, F-09, F-10, F-11, F-12, F-17
- **Depends on:** S4
- **Code changes:** `src/components/SettingsDialog.tsx`, `src/pages/Chat.tsx`,
  `src/App.tsx`, `src/types.ts`, `src-tauri/src/store.rs`,
  `src-tauri/src/adapters.rs`, `src-tauri/src/models.rs` (derives only)

## Goal

Make provider configuration honest end to end: the author explicitly chooses the
wire protocol, an existing provider keeps its kind, a key-cleared legacy provider
survives the one-time cleanup, the mounted Chat sees settings changes, overlapping
saves/fetches cannot cross-contaminate, the model-list URL is derived correctly,
and the HTTP transport is reused.

## Evidence

- `src/components/SettingsDialog.tsx:225-258` always enters the add flow as
  `custom` and derives `kind` from `form.id`; `:325-353` repeats the inference on
  save. With no shipped catalog (`src-tauri/src/store.rs:140-145`), a fresh
  Anthropic provider is sent through the OpenAI adapter
  (`src-tauri/src/adapters.rs:86-96`).
- `src-tauri/src/store.rs:13-24,148-173` deletes any legacy-ID provider with an
  empty key and rewrites the file; `SettingsDialog.tsx:382-389` intentionally
  clears a key, so the next load deletes the configured provider.
- `src/pages/Chat.tsx:316-318,352-375` loads providers once on mount;
  `SettingsDialog.tsx:356-389` updates only its own state; Shell renders both
  (`src/App.tsx:274-294`).
- `SettingsDialog.tsx:271-311` merges a late fetch into whichever form is
  current; `:325-393` saves whole arrays from stale closures; `Apply` is not
  disabled while saving.
- `src-tauri/src/adapters.rs:101-140` accepts a chat URL ending
  `/chat/completions` but appends `/models`; `:469-484` builds a new client per
  request.
- `src-tauri/src/adapters.rs:285-314,340-367` drops malformed tool calls or
  assigns blank IDs.

## Decisions (locked)

1. **Explicit protocol, persisted `kind`.** Keep the existing string values
   (`"openai"`, `"anthropic"`, `"custom"`; `custom` is OpenAI-compatible). In the
   provider editor add a **Protocol** select:
   - `OpenAI-compatible` → `"custom"` for a new endpoint, `"openai"` preserved
     for an existing one;
   - `Anthropic` → `"anthropic"`.
   Initialize the form from `provider.kind`; save `form.kind` verbatim. Do not
   infer the protocol from `id`, `name`, or hostname. Existing `"openai"` and
   `"anthropic"` records must be editable without changing kind unless the author
   changes the select.
2. **Delete the `editor.mode` pseudo-state.** `editor` keeps `{ id, kind }` only
   if needed for new/existing; the UI branches on `form.kind` and
   `editor.id === null`. Remove the disabled "Custom/Third-party" toggle and its
   copy; keep the advanced-fields section for new/custom endpoints and for any
   provider whose base URL needs editing.
3. **Full-record legacy fingerprint.** Move the ten legacy seed definitions into
   a production `legacy_seed_catalog()` in `store.rs` (same data as the current
   `catalog_fixture()`), derive `PartialEq`/`Eq` for `ProviderConfig`,
   `ModelConfig`, and `InputTypes`, and treat a record as an untouched seed only
   when `api_key` is empty **and** it equals a seed record field-for-field. Keep
   `catalog_fixture()` as a thin test wrapper. This protects edited endpoints,
   names, models, and a deliberately cleared key. Add regression tests for:
   untouched seed pruned; edited keyless provider kept; key-cleared configured
   provider kept; custom provider kept.
4. **One provider revision in Shell.** `App.tsx`/`Shell` owns
   `providersRevision: number`; pass it to `<Chat providersRevision={...} />` and
   pass `onProvidersChanged={() => setProvidersRevision(r => r + 1)}` to
   `<SettingsDialog>`. Chat reloads on revision change; Settings calls the
   callback only after a successful save/delete/clear. Preserve the current
   provider/model selection if still present; otherwise fall back to the existing
   "first with a key, else first" rule.
5. **Serialize provider mutations.** Add a `saving` state and one
   `persistProviders(next, successMessage)` helper. Disable Apply/Delete/Clear
   while `saving`; ignore a second submit. Update local state and the revision
   callback only after the backend accepts the write. A failed write leaves the
   previous list and shows the error.
6. **Fetch generation.** Add `editorGenerationRef`; increment on `openEditor` and
   `closeEditor`. Capture it in `fetchModels`; after `await`, ignore success,
   error, and `finally` if the generation changed. Dedupe fetched IDs against
   both the current form and the incoming batch (Set), so a provider returning
   the same ID twice cannot invalidate the form. Show the notice only for the
   active editor.
7. **URL normalization.** `models_endpoint` must derive from the API root:
   - OpenAI/custom: strip a trailing `/chat/completions`; if the result ends
     `/models`, use it; else append `/models`.
   - Anthropic: strip a trailing `/messages` (so `/v1/messages` → `/v1`); if the
     result ends `/v1`, append `/models`; if it already ends `/models`, use it;
     else append `/v1/models`.
   Add unit tests for base, `/v1`, completed chat URL, completed models URL, and
   a trailing slash, for both kinds.
8. **Reuse immutable clients.** Replace `client()`/`list_client()` with
   `OnceLock<Result<reqwest::Client, String>>` caches and return a cheap
   `Client` clone (or `&'static Client`). Keep the 120s chat and 30s list
   timeouts distinct. No new dependency.
9. **Adapters fail closed on malformed tool calls.**
   - OpenAI: every item in `tool_calls` must have a non-empty `id`, a non-empty
     `function.name`, and `arguments` that is an object/array/null or a JSON
     string that parses to JSON. A missing/blank required field or invalid JSON
     string is an `Err` naming the call. Duplicate IDs are an `Err`.
   - Anthropic: every `tool_use` block must have a non-empty `id` and `name`;
     missing `input` is `Null` only if the block explicitly has no input;
     otherwise accept any JSON value.
   - Keep tolerating object-valued `arguments` (some proxies send it).
   - Add unit tests for each rejection and for the tolerated object form.

## Tasks

1. `types.ts`: add `export type ProviderKind = "openai" | "anthropic" | "custom";`
   and narrow `ProviderConfig.kind` to it if all producers are updated. Keep the
   wire values unchanged.
2. `store.rs`: add `legacy_seed_catalog`, derives, the full fingerprint, and the
   four tests. Keep the one-time prune + rewrite behavior for genuine seeds.
3. `SettingsDialog.tsx`: protocol select, form `kind`, remove `mode`, typed
   `newFormState` (no `any`), generation guard, dedupe, serialized persist
   helper, callback, disabled controls while saving.
4. `App.tsx`/`Chat.tsx`: revision state/prop/effect; preserve selection.
5. `adapters.rs`: URL helper + tests; static clients; strict tool-call parsing +
   tests.
6. Update `catalog_fixture()` call sites if the name changes; keep the IPC
   contract fixtures working (they use seed shape, not the prune path).

## Explicitly out of scope

- Multi-window merge/versioning of `config.json`; this sprint serializes one app
  window and documents the limitation.
- Moving provider config to SQLite, encrypting keys, or a keychain integration.
- A provider registry/factory hierarchy; the protocol select + `match` is enough.
- New provider features (streaming, custom headers, proxy settings).
- IPC typing and browser-preview changes (S6).

## Acceptance checks

- [ ] `cargo test --manifest-path src-tauri/Cargo.toml` passes.
- [ ] `pnpm build` passes.
- [ ] A fresh Anthropic provider can be added, model-fetched, saved, and will
      route to `call_anthropic` (manual trace: `kind` saved as `"anthropic"`).
- [ ] Editing an existing `"anthropic"` provider does not change its kind.
- [ ] An edited keyless legacy provider and a key-cleared configured provider
      both survive `load_providers`.
- [ ] Saving the first provider in Settings makes the mounted Chat able to send
      without a route change (manual trace).
- [ ] Two Apply clicks / a delete during a pending save cannot write a stale
      whole-list snapshot.
- [ ] Fetch A → close → open B → resolve A leaves B's form/notices untouched.
- [ ] `models_endpoint` tests cover both protocols and completed URLs.
- [ ] Malformed tool calls are rejected, not dropped or assigned blank IDs.
- [ ] No new dependency.

## Commit and push

```bash
git switch -c refactor/s5-provider-lifecycle main
# edit
cargo test --manifest-path src-tauri/Cargo.toml
pnpm build
git add src/components/SettingsDialog.tsx src/pages/Chat.tsx src/App.tsx \
        src/types.ts src-tauri/src/store.rs src-tauri/src/adapters.rs \
        src-tauri/src/models.rs docs/refactor/sprints/README.md \
        docs/refactor/sprints/05-provider-lifecycle.md
git commit -m "fix(providers): persist protocol, protect legacy config, and sync settings"
git push -u origin refactor/s5-provider-lifecycle
```

Report the branch, commit SHA, both command results, the exact legacy predicate,
the new endpoint test cases, and any provider UI behavior that changed.
