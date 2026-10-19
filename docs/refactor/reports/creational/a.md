# Creational review — Reviewer A

## Scope and interpretation

- Read `docs/refactor/README.md`, the project GoF skill, its five creational references, and `references/criticism-and-modernity.md` before assessing source. This is a read-only review; no application or test code was changed.
- Examined the construction and lifetime paths in `src/` (25 TS/TSX files, approximately 5,500 lines): notably `src/pages/Chat.tsx` (980), `src/components/SettingsDialog.tsx` (1,008), `src/lib/tauri.ts` (844), `src/App.tsx` (301), `src/components/CharacterForm.tsx` (227), `src/types.ts` (214), the character helpers, preferences, Markdown parser, sample data, and the remaining components/pages.
- Examined `src-tauri/src/` (12 Rust files, approximately 7,500 lines including tests): notably `runtime.rs` (1,809), `super_json.rs` (1,513), `adapters.rs` (1,004), `tools.rs` (879), `db.rs` (327), `store.rs` (326), `models.rs` (322), `lib.rs` (207), `paths.rs` (85), and the IPC/test-support construction seams. Counts are whole-file sizes; the large files were sampled at relevant creation, persistence, and test paths rather than every test line.
- **Pattern bar:** a Rust `Clone`, JavaScript spread, config record, one-off `new`, or module variable is not automatically Prototype, Builder, Factory Method, or Singleton. The absence of an Abstract Factory/Prototype finding is deliberate: there is no demonstrated interchangeable *family* of related products or expensive, semantically necessary clone template. “No pattern applies” means fix the underlying lifecycle, race, or persistence semantics without a GoF wrapper.

## Ranked summary

| Rank | Class / impact | Creational pattern | Primary evidence | Finding |
| ---: | --- | --- | --- | --- |
| 1 | **BUG · high** | no pattern applies | `src-tauri/src/store.rs:152-171` | Legacy cleanup deletes a provider the author edited or explicitly cleared. |
| 2 | **BUG · high** | Factory Method | `src/pages/Chat.tsx:446-450` | Millisecond message IDs overwrite unrelated persisted messages. |
| 3 | **BUG · high** | no pattern applies | `src-tauri/src/super_json.rs:289-315` | Concurrent field updates lose one writer's entire document changes. |
| 4 | **BUG · high** | Factory Method | `src-tauri/src/tools.rs:249-261` | Project creation is non-atomic and can bind two sessions to one slug. |
| 5 | **BUG · high** | no pattern applies | `src/pages/Chat.tsx:394-410` | A finished old turn contaminates a newly constructed chat. |
| 6 | **BUG · medium** | Factory Method | `src/components/SettingsDialog.tsx:225-258` | Editing an Anthropic-kind provider can turn it into OpenAI. |
| 7 | **BUG · medium** | no pattern applies | `src/components/SettingsDialog.tsx:356-376` | Whole-list provider saves silently discard changes from another writer. |
| 8 | **BUG · medium** | Singleton | `src/lib/tauri.ts:155-159` | Browser-preview module state outlives the chat it is supposed to represent. |
| 9 | **BUG · medium** | no pattern applies | `src/components/SettingsDialog.tsx:271-310` | A late model-list fetch mutates a different provider's form. |
| 10 | **SMELL · medium** | Builder | `src-tauri/src/adapters.rs:469-484` | New HTTP clients are built on every model request and chat round. |
| 11 | **OVER-ABSTRACTION · medium** | no pattern applies | `src/lib/tauri.ts:509-565` | A second, hand-built tool backend creates demonstrably false preview results. |
| 12 | **BUG · low** | no pattern applies | `src/components/SettingsDialog.tsx:284-317` | A duplicated fetched model can make provider setup impossible to save. |

## Detailed findings

### 1. Deleting keyless legacy providers is not a safe migration

- **Class / pattern:** **BUG · high**; **no pattern applies**. This is provenance-sensitive data migration, not a factory/default-provider opportunity.
- **Evidence:** `src-tauri/src/store.rs:13-24` hard-codes ten historical IDs; `src-tauri/src/store.rs:148-154` calls a provider “untouched” solely when its key is empty and its ID is in that set. `src-tauri/src/store.rs:156-173` filters those records and immediately persists the smaller list.
- **Reproduction / cost:** An author may edit the endpoint, name, models, or limits of an old `anthropic`/`openai` record without keeping a key. More directly, Settings exposes **Clear API key** for precisely those entries at `src/components/SettingsDialog.tsx:977-989`; it writes `api_key: ""` at `src/components/SettingsDialog.tsx:382-389`. On the next load, the migration permanently deletes that configured provider. Other open dialogs can still display it until reload.
- **Recommended change:** Stop guessing ownership from the ID and key. Remove seeds only if the *whole record* matches a known shipped seed (accounting for versioned fixtures), or mark one-time migration explicitly. Test an edited keyless legacy provider and the UI clear-key transition. Retain the intentional empty default list (`src-tauri/src/store.rs:140-145`).
- **Risk of change:** Strict fixture matching leaves some truly untouched old presets in place. Prefer a harmless leftover preset to silently deleting authored configuration; document the migration boundary.

### 2. Clock time is not a message identity factory

- **Class / pattern:** **BUG · high**; **Factory Method** is justified for a small semantic ID-creation function, not a hierarchy of message classes.
- **Evidence:** `src/pages/Chat.tsx:446-450` emits `msg-${Date.now()}`; `src-tauri/src/db.rs:19-25` makes `messages.id` globally unique, and `src-tauri/src/db.rs:173-194` uses `INSERT OR REPLACE`, not a collision error. Every incoming message is inserted in `src-tauri/src/runtime.rs:260-275`.
- **Reproduction / cost:** Two chats sending in the same millisecond create identical IDs. The later insert **replaces** the first chat's row and changes its `session_id`; the first transcript loses a message without an error. Repeat submits within one millisecond have the same hazard. Backend-generated IDs already acknowledge this collision mode in `src-tauri/src/runtime.rs:133-142`.
- **Recommended change:** Give user-message creation one collision-resistant `newMessageId()` factory (reuse the `crypto.randomUUID()` strategy of `src/pages/Chat.tsx:43-48`, with a well-scoped fallback). Consider rejecting conflicting `(id, different session/payload)` inserts rather than masking them with `REPLACE`; preserve intentional replay/idempotency explicitly.
- **Risk of change:** Previously stored IDs and history must remain readable. A database constraint change should be migration-tested against legitimate repeat insertion of an unchanged message.

### 3. Immutable-looking super-JSON writes are lost-update writes

- **Class / pattern:** **BUG · high**; **no pattern applies**. This requires synchronization or transactional storage, not Prototype or a Builder.
- **Evidence:** `src-tauri/src/super_json.rs:289-315` loads an entire project document, mutates one path, and saves the entire document. `src-tauri/src/super_json.rs:233-266` atomically renames a *whole* file, without locking the read–modify–write sequence. `src-tauri/src/tools.rs:344-354` exposes this write through tool execution; `src-tauri/src/runtime.rs:190-210` admits independently running chat commands.
- **Reproduction / cost:** Two sessions bound to one project can both read document D, write different paths, then independently rename their D+one-field snapshots. The second rename discards the first answer. Atomic rename protects against a torn file, **not** lost updates. The UI's completion/progress may then claim a field is answered when the file no longer contains it.
- **Recommended change:** Serialize per-project `set` operations across load/mutate/save, or put the fields in a store with atomic updates; test concurrent writes to different required paths. Keep independent project writes concurrent. Coordinate first-use `load_or_init` with the same per-project boundary.
- **Risk of change:** A coarse process-wide mutex serializes unrelated projects; a per-project lock has a lifecycle/cleanup cost, and an in-process lock does not coordinate other app processes. Choose the actual concurrency contract explicitly.

### 4. Choosing an available slug does not reserve it

- **Class / pattern:** **BUG · high**; **Factory Method** applies to creation of a project identity/resource, but the essential fix is atomic reservation and cleanup.
- **Evidence:** `src-tauri/src/tools.rs:249-261` checks the current project ID list and constructs a free slug; `src-tauri/src/tools.rs:295-323` later inserts a row, initializes a file, writes a title, then binds the session. `src-tauri/src/db.rs:80-93` inserts with `INSERT OR IGNORE` and never reports that a proposed ID was already taken.
- **Reproduction / cost:** Two sessions naming a project alike may both select `same-name`. One insert is silently ignored; both can write the same project's JSON and bind to it, unexpectedly sharing story content. Independently, failure after the DB insert but before JSON initialization or session binding leaves an orphan row; retry gets a suffixed slug and the user sees duplicate projects.
- **Recommended change:** Let one project-creation function reserve IDs using a unique insert that **returns collision** and retries suffixes atomically. Make initialization/binding failure observable and compensating (or stage file creation and DB commit with a clear recovery rule); add a parallel-create regression test and a failed-initialization test.
- **Risk of change:** SQLite and filesystem creation do not share a native transaction. Do not promise perfect atomicity; design and test a recoverable partial state and avoid deleting a concurrently claimed directory.

### 5. Constructing a new chat does not retire the old turn

- **Class / pattern:** **BUG · high**; **no pattern applies**. A session lifetime/cancellation rule is needed, not Singleton/Abstract Factory.
- **Evidence:** The always-enabled sidebar action at `src/App.tsx:198-211` dispatches `merlink:new-chat`. The listener at `src/pages/Chat.tsx:394-419` clears UI state and creates a fresh session ID, but `src/pages/Chat.tsx:456-475` still has a live event channel and an awaited request for the preceding session. The old result appends messages and binds a project at `src/pages/Chat.tsx:475-488`; old `finally` updates current loading state at `src/pages/Chat.tsx:498-506`.
- **Reproduction / cost:** Click New chat while a model/tool loop is running. Its old answer, old project ID, tool events, and possibly errors appear in the fresh chat; the next user turn may continue the old project. On unmount/remount, late state updates are still undesirable even if React drops some of them.
- **Recommended change:** Give each turn an immutable session/generation token; event callbacks, success, error, progress refresh, and `finally` must check the current generation. Decide separately whether to cancel backend work or merely detach its UI; disabling New chat until completion is a simpler but worse UX fallback.
- **Risk of change:** Suppressing display must not accidentally discard persisted turns; backend cancellation and project-creation cleanup require a deliberate policy.

### 6. Provider kind is rebuilt from its ID, not its configuration

- **Class / pattern:** **BUG · medium**; **Factory Method** for a *typed, single-point* provider-config construction decision; do not introduce an Abstract Factory.
- **Evidence:** Opening a provider stores only `id` and `mode` at `src/components/SettingsDialog.tsx:225-236`; fetch construction infers `kind` from `form.id === "anthropic"` at `src/components/SettingsDialog.tsx:253-258`. Saving repeats that inference at `src/components/SettingsDialog.tsx:325-354`. Backend dispatch actually follows `request.kind`, not `id`, at `src-tauri/src/adapters.rs:86-96`.
- **Reproduction / cost:** A valid stored provider `{ id: "my-claude", kind: "anthropic", ... }` opens as third-party, but a model fetch uses OpenAI bearer-auth and a subsequent save permanently changes it to OpenAI kind. Calls then use the wrong endpoint and headers. IDs are user-config data, not protocol discriminants.
- **Recommended change:** Carry `ProviderConfig.kind` as a typed, preserved field in the editor; one `makeProviderConfig(form, existing)` helper can use `existing.kind` for edits and `custom` for new providers. Fetch should use the same preserved kind. Keep the explicit backend `kind` dispatch.
- **Risk of change:** Existing incorrectly saved configurations may need user repair or a narrow migration; no inference can reliably recover their original protocol from a renamed ID.

### 7. Whole-array provider saves are vulnerable to stale construction

- **Class / pattern:** **BUG · medium**; **no pattern applies**. A snapshot and spread are not Prototype; the concern is lost writes.
- **Evidence:** Settings loads a provider array when opened at `src/components/SettingsDialog.tsx:118-135`; edit/remove builds a new *whole array* from local state at `src/components/SettingsDialog.tsx:356-376`. `src-tauri/src/store.rs:180-216` replaces the whole config on save without comparing revisions or merging records.
- **Reproduction / cost:** Two app windows open Settings, A adds a provider, then B edits an older provider and saves its stale array. A's new provider (including its configured API key) disappears. An external config edit has the same outcome. Atomic rename prevents partial bytes, not stale-snapshot overwrites.
- **Recommended change:** Offer backend `upsert_provider`/`remove_provider` operations that load, mutate, and persist under serialization; or expose a file revision and reject stale whole-array saves so the UI can reload/merge intentionally. Add two-editor tests around add/edit/delete.
- **Risk of change:** Conflicting concurrent edits to the *same* provider still need a policy; naive per-provider merges can resurrect deleted entries. Keep key-handling and file permissions from the existing save path.

### 8. Browser-preview state is an accidental application Singleton

- **Class / pattern:** **BUG · medium**; **Singleton**, in its problematic module-global mutable form. Do not replace it with a formal `Singleton` class.
- **Evidence:** `src/lib/tauri.ts:155-159` holds one global values map, project ID, message list, and unnamed-project counter. `src/lib/tauri.ts:515-546` rejects a second `project_create` whenever that global ID is set. The preview `mockSendChat` signature at `src/lib/tauri.ts:582-590` has no `sessionId`; `src/lib/tauri.ts:768-769` appends messages globally, and `src/lib/tauri.ts:805-811` returns those messages without session scoping. Meanwhile New chat creates a new session at `src/pages/Chat.tsx:394-399`.
- **Reproduction / cost:** In plain-browser preview, create a project and click New chat: the new UI is empty, but `mockSendChat` skips `project_create` (`src/lib/tauri.ts:610-620`) because the old global project ID still exists. An explicit `execute_tool(project_create)` instead receives an already-bound error (`src/lib/tauri.ts:525-527`). Subsequent tool updates hit the old project and mock transcript queries can mix histories. A browser refresh resets everything, proving lifetime is the mismatch.
- **Recommended change:** Scope preview state by session ID (pass it through the mock bridge), or build an explicit preview session object and reset it on New chat. If preview does not promise multiple sessions, clearly make it a **small stateless stub** rather than a misleading persistent clone.
- **Risk of change:** Adjust the preview's currently scripted fixtures and tests; do not accidentally use the mock's local state as the desktop source of truth.

### 9. Model fetching can attach a previous provider's models to a new one

- **Class / pattern:** **BUG · medium**; **no pattern applies**. This is an async editor lifetime bug, not Builder.
- **Evidence:** `src/components/SettingsDialog.tsx:271-282` starts `fetch_models` from the current form; its success path calls `setForm(prev => ...)` without checking which editor is active at `src/components/SettingsDialog.tsx:284-310`. Changing/closing the editor resets the `fetching` flag but does not cancel or invalidate the old request at `src/components/SettingsDialog.tsx:195-235`.
- **Reproduction / cost:** Start a slow fetch for provider A, close that editor, then open provider B and fetch or edit it. A's late response appends A-models to B and may set B's default to an incompatible model. The late `finally` can also clear the loading indicator for B's newer request.
- **Recommended change:** Track request generation/editor identity, invalidate on close/switch, and apply success/error/finally only to the matching generation. Dedupe against `prev.models` inside that guarded state update, not a closure snapshot.
- **Risk of change:** A cancelled response should be silent rather than shown as a provider error; ensure the current editor's fetch state is still cleared on genuine failure.

### 10. The existing client Builder runs once per call, not once per lifetime

- **Class / pattern:** **SMELL · medium**; **Builder** is already the right tool for assembling `reqwest::Client` options; a GoF-style *new* Builder is not warranted.
- **Evidence:** `src-tauri/src/adapters.rs:469-484` builds two separately timed clients; `src-tauri/src/adapters.rs:502-515`, `src-tauri/src/adapters.rs:531-537`, and `src-tauri/src/adapters.rs:552-560` call those constructors on every fetch/LLM request. One agent turn explicitly loops over rounds in `src-tauri/src/runtime.rs:320-343`, and production passes a new unit `HttpBackend` at `src-tauri/src/runtime.rs:190-210`.
- **Failure / cost:** Each tool round creates a new connection pool instead of reusing TCP/TLS sessions and HTTP/2 connections; multi-round interactions incur avoidable handshakes and allocations. Keeping model-list's 30-second timeout distinct from chat's 120-second timeout is correct.
- **Recommended change:** Construct two `reqwest::Client`s once at app/backend initialization and hold them in an injected `HttpBackend { chat, model_list }` (or equivalent app-managed state). Pass the chat client to both provider implementations, retain backend injection for fake tests, and avoid a process-global mutable client registry.
- **Risk of change:** Client settings become long-lived; refresh behavior, proxy configuration, and timeouts must remain explicit. Test both provider protocols and the 30-second list-client policy.

### 11. Delete preview-backend imitation rather than abstracting it further

- **Class / pattern:** **OVER-ABSTRACTION · medium**; **no pattern applies**. In particular, an Abstract Factory for real/mock tool families would formalize a false equivalence.
- **Evidence:** `src/lib/tauri.ts:146-177` hand-maintains preview super-JSON required paths, `src/lib/tauri.ts:509-569` hand-implements a second tool executor, and `src/lib/tauri.ts:792-838` hand-routes IPC names. Yet `src/lib/tauri.ts:561-564` responds to `super_json_missing` with `missing: []` even when `mockMissingRequired()` at `src/lib/tauri.ts:161-178` reports unanswered paths; the Rust authority computes actual missing fields at `src-tauri/src/super_json.rs:318-329`.
- **Failure / cost:** Browser preview can declare an incomplete outline complete when callers use the tool result, while the desktop app refuses it. Every schema/tool addition requires synchronized edits across two languages; an elaborate interface cannot make invented preview results authoritative.
- **Recommended change:** **Delete** duplicated business rules where possible. Retain only deliberately labelled UI fixtures and a narrow fake boundary for rendering; use Rust contract tests for behavior. If `super_json_missing` remains in preview, derive it from the existing `mockMissingRequired()` instead of returning fabricated success.
- **Risk of change:** Browser-only demos may lose fake agent interactions. Preserve the essential UI demonstration explicitly, but do not represent it as backend correctness coverage.

### 12. Fetch merge does not dedupe duplicates in the incoming response

- **Class / pattern:** **BUG · low**; **no pattern applies**. This is set membership/normalization, not Prototype or a model-object Builder.
- **Evidence:** `src/components/SettingsDialog.tsx:284-290` builds `existing` solely from models *already on the form*; it filters all incoming rows against that unchanged set. `src/components/SettingsDialog.tsx:301-305` appends the resulting batch. `src/components/SettingsDialog.tsx:314-323` then forbids saving duplicate IDs.
- **Reproduction / cost:** If `/models` returns the same ID twice, both copies are considered new. A previously valid provider form becomes invalid immediately after Fetch models; the author must manually remove one before Save becomes available. An in-flight form edit has the same stale-set risk.
- **Recommended change:** Normalize nonblank fetched IDs with a Set seeded from the **current** form and add each unseen ID once; leave existing metadata untouched. Add duplicate-response and concurrent-form-edit tests.
- **Risk of change:** If a provider supplies conflicting names for the same ID, choose a deterministic first/last-wins rule and describe it; do not silently rewrite existing model metadata.

## Proposed regression checks (not executed)

These are review acceptance criteria, not claims that any test was run or any source was modified.
The cases are intentionally about observable state, not whether a GoF class exists.

1. **Edited legacy record survives** (`src-tauri/src/store.rs:152-173`).
   - Arrange a config containing the former `anthropic` fixture with a changed Base URL and no API key.
   - Call `load_providers` twice, including after the migration's rewrite opportunity.
   - Assert the modified record and its endpoint survive both reads.
   - Negative control: an exactly untouched fixture can still be removed if that is the chosen policy.

2. **Clearing a key does not delete the provider** (`src/components/SettingsDialog.tsx:382-390`).
   - Arrange a named legacy-ID provider with a saved key and edited models.
   - Invoke the Settings clear-key action, close Settings, and reopen the application/config.
   - Assert the same provider and models remain, now with an empty key.
   - Confirm no auto-created default provider reappears in an unrelated fresh installation.

3. **Message identities survive clock collisions** (`src/pages/Chat.tsx:446-450`).
   - Arrange two different sessions with user-message creation at the same frozen millisecond.
   - Persist both and retrieve each session through `list_messages`.
   - Assert the IDs differ and neither session loses a row.
   - Resubmit an unchanged message once and assert intentional idempotency remains.

4. **Two writes to one story both survive** (`src-tauri/src/super_json.rs:289-315`).
   - Arrange one project document and two separate required leaf paths.
   - Force two `set` calls to overlap their read/write windows.
   - Assert both final values are present, and `missing_required` agrees with the file.
   - Run the same scenario for two projects; unrelated projects should not share a coarse lock.

5. **Two projects with one title stay distinct** (`src-tauri/src/tools.rs:249-323`).
   - Arrange two as-yet-unbound sessions both requesting `project_create` with one title.
   - Launch both calls concurrently, then query project IDs and session bindings.
   - Assert each session binds to a different created row and its own JSON path.
   - Inject a JSON-initialization failure and assert there is a defined retry/recovery state.

6. **New chat detaches the old turn** (`src/pages/Chat.tsx:394-410`).
   - Arrange a pending `send_chat` response whose first session has a known project.
   - Click New chat before resolving the old event channel and result.
   - Assert the new chat stays empty, unbound, and free of old progress/errors.
   - Start a fresh turn and assert the old `finally` does not unlock or clear its live frame.

7. **Editing preserves provider protocol** (`src/components/SettingsDialog.tsx:225-258`).
   - Arrange a configured `{ id: "my-claude", kind: "anthropic" }` provider.
   - Open its editor, fetch models, then save an unrelated field.
   - Assert fetch uses Anthropic authentication and saved `kind` remains `anthropic`.
   - Reopen and send a chat; the adapter must still dispatch to the Anthropic path.

8. **Stale config saves do not erase another writer** (`src/components/SettingsDialog.tsx:356-376`).
   - Arrange two Settings views that load the same initial provider revision.
   - Add a provider in A; in B, edit a different provider using its stale snapshot.
   - Assert both edits survive or B receives an explicit conflict to resolve.
   - Repeat with a deletion to ensure merging does not resurrect removed records.

9. **Preview chat state belongs to its session** (`src/lib/tauri.ts:155-159`).
   - Arrange browser preview with a created project in chat A.
   - Click New chat and create another project in chat B without refreshing the page.
   - Assert B gets a distinct project and no transcript from A.
   - Return to A if navigation exists, or assert an intentional reset if preview is stateless.

10. **Late model fetch cannot mutate another editor** (`src/components/SettingsDialog.tsx:271-310`).
    - Arrange a deferred models response for provider A, then close A and open B.
    - Resolve A's request while B's editor is active.
    - Assert B's models, default, fetch banner, and busy state remain unchanged.
    - Resolve a genuine B request and assert B's own loading indicator is cleared.

11. **Client lifetime and timeouts are explicit** (`src-tauri/src/adapters.rs:469-484`).
    - Arrange an injected production backend and a scripted two-round tool exchange.
    - Assert both chat requests borrow one shared chat client, not newly built clients.
    - Assert model-list traffic uses the separate 30-second-configured client.
    - Run existing scripted backend tests to ensure the injection seam still works.

12. **Preview and model-list results tell the truth** (`src/lib/tauri.ts:561-564`).
    - Arrange an unfinished preview project; call `super_json_missing` and compare its required paths.
    - Assert the mock does not claim zero missing while required fields are unset.
    - Feed two fetched models with the same ID into Settings (`src/components/SettingsDialog.tsx:284-317`).
    - Assert precisely one new model appears and Save remains enabled when other inputs are valid.

## What NOT to change

- **Do not add an Abstract Factory.** `src/lib/tauri.ts:37-46` already creates either a Tauri event channel or a small preview-compatible object with one function. `src-tauri/src/adapters.rs:86-99` has a two-branch protocol dispatch, not two coordinated product families that must be swapped together. TypeScript discriminants and Rust `match` are clearer.
- **Do not add a Prototype API.** `src-tauri/src/runtime.rs:328-342` clones request-owned strings/messages, and `src/components/SettingsDialog.tsx:246-250` spreads immutable form records; these are ordinary ownership/React operations. `src/data/mock.ts:214-241` builds one set of sample projects, not a configurable template-cloning mechanism.
- **Keep small factories, delete ceremony.** `src-tauri/src/models.rs:105-136` has useful static constructors `MessageEnvelope::new`/`tool_result`, and `src/pages/Chat.tsx:43-48` has a local UUID session-ID factory. Neither needs subclassing, fluent setters, a configurable Builder, or a provider registry. A semantic message-ID creator would be similarly small.
- **Keep test seams and narrow ownership.** `src-tauri/src/adapters.rs:74-99` and `src-tauri/src/runtime.rs:224-233` inject an LLM backend and event sink for scripted tests; `src-tauri/src/ipc_contract_tests.rs:38-65` builds a real command harness under a temporary home. `src-tauri/src/db.rs:32-60` opens per-operation connections rather than hiding a mutable global database. Do not replace these with a Singleton service locator.
- **Keep intentional immutable process caches.** `src-tauri/src/super_json.rs:73-95` memoizes schema path metadata; that is a bounded immutable cache, not the mutable chat/project Singleton criticized in finding 8. Keep the application state at session/project scope instead.
