# Behavioral GoF review — Reviewer B (UI/domain interaction)

## Scope and method

- Read the programme rules, the GoF skill, all eleven behavioral references, and the criticism/modernity reference. This was a source-only review; no UI or backend was executed, and no production file was changed.
- Frontend: approximately **5,500 lines** across all 25 TypeScript/TSX files under `src/`. Concentrated on [Chat.tsx](<src/pages/Chat.tsx>) (~980 lines), [SettingsDialog.tsx](<src/components/SettingsDialog.tsx>) (~1,008), [Characters.tsx](<src/pages/Characters.tsx>) (~231), [CharacterForm.tsx](<src/components/CharacterForm.tsx>) (~227), [Dialog.tsx](<src/components/Dialog.tsx>) (~115), [App.tsx](<src/App.tsx>) (~301), [Home.tsx](<src/pages/Home.tsx>) (~222), [tauri.ts](<src/lib/tauri.ts>) (~844), [preferences.ts](<src/lib/preferences.ts>) (~79), and [characters.ts](<src/lib/characters.ts>) (~122); also inspected the remaining page, component, type, and mock-data modules.
- Backend: approximately **7,480 lines** across 12 Rust files under `src-tauri/src/`, with focused inspection of [runtime.rs](<src-tauri/src/runtime.rs>) (~1,809), [super_json.rs](<src-tauri/src/super_json.rs>) (~1,513), [adapters.rs](<src-tauri/src/adapters.rs>) (~1,004), [tools.rs](<src-tauri/src/tools.rs>) (~879), [store.rs](<src-tauri/src/store.rs>) (~326), [db.rs](<src-tauri/src/db.rs>) (~327), [models.rs](<src-tauri/src/models.rs>) (~322), and the registered IPC surface in [lib.rs](<src-tauri/src/lib.rs>) (~207). Test-only sections were skimmed rather than treated as production flows.
- Ranking prioritizes lost or misattributed author work, then persistence correctness, then stale/contradictory UI and accessibility. **BUG** means an observable wrong result under the stated trigger; **SMELL** means a maintenance or operational risk; **OVER-ABSTRACTION** means remove a layer rather than add a pattern. GoF labels are a design lens, not a demand for class hierarchies.

## Summary

| Rank | Severity | Pattern lens | Exact source evidence | One-line finding |
| ---: | --- | --- | --- | --- |
| 1 | BUG · critical | State + Observer | [src/pages/Chat.tsx:394](<src/pages/Chat.tsx#L394>), [src/pages/Chat.tsx:458](<src/pages/Chat.tsx#L458>) | New Chat during an active turn accepts the old turn into the new transcript. |
| 2 | BUG · high | Memento (restore), no class | [src/pages/Chat.tsx:211](<src/pages/Chat.tsx#L211>), [src-tauri/src/lib.rs:174](<src-tauri/src/lib.rs#L174>) | Navigation/restart loses the UI's access to previously persisted sessions. |
| 3 | BUG · high | State (failed/partial turn) | [src-tauri/src/runtime.rs:273](<src-tauri/src/runtime.rs#L273>), [src-tauri/src/runtime.rs:343](<src-tauri/src/runtime.rs#L343>) | A failed later model round leaves durable tool effects/messages invisible to the frontend. |
| 4 | BUG · high | no pattern applies | [src-tauri/src/super_json.rs:290](<src-tauri/src/super_json.rs#L290>) | Parallel writes to distinct super-json fields silently erase one another. |
| 5 | BUG · high | no pattern applies | [src/components/SettingsDialog.tsx:382](<src/components/SettingsDialog.tsx#L382>), [src-tauri/src/store.rs:152](<src-tauri/src/store.rs#L152>) | Clearing an old-ID provider's key deletes the entire configured provider on next load. |
| 6 | BUG · high | Observer (shared change notification) | [src/pages/Chat.tsx:316](<src/pages/Chat.tsx#L316>), [src/components/SettingsDialog.tsx:360](<src/components/SettingsDialog.tsx#L360>) | Adding a provider in Settings leaves the already-mounted Chat unable to send. |
| 7 | BUG · high | State (request identity), no class | [src/components/SettingsDialog.tsx:271](<src/components/SettingsDialog.tsx#L271>), [src/components/SettingsDialog.tsx:301](<src/components/SettingsDialog.tsx#L301>) | A late model-list response updates a different provider's editor. |
| 8 | BUG · medium | Command (serialized mutations) | [src/components/SettingsDialog.tsx:325](<src/components/SettingsDialog.tsx#L325>), [src/components/SettingsDialog.tsx:371](<src/components/SettingsDialog.tsx#L371>) | Concurrent whole-list provider saves race and can lose updates. |
| 9 | BUG · medium | no pattern applies | [src/components/CharacterForm.tsx:68](<src/components/CharacterForm.tsx#L68>) | Leaving Story unselected silently assigns the first, unrelated story. |
| 10 | BUG · medium | State (session isolation) | [src/lib/tauri.ts:155](<src/lib/tauri.ts#L155>), [src/lib/tauri.ts:610](<src/lib/tauri.ts#L610>) | Browser-preview New Chat inherits the prior project's document and progress. |
| 11 | BUG · medium | State (request identity), no class | [src/pages/Chat.tsx:252](<src/pages/Chat.tsx#L252>), [src/pages/Chat.tsx:295](<src/pages/Chat.tsx#L295>) | Unordered progress/detail responses overwrite the active project's view. |
| 12 | BUG · medium | no pattern applies | [src/components/SettingsDialog.tsx:171](<src/components/SettingsDialog.tsx#L171>), [src/components/SettingsDialog.tsx:638](<src/components/SettingsDialog.tsx#L638>) | The Settings editor claims modal behavior without focus isolation or restoration. |
| 13 | BUG · low | Observer (shared source of truth) | [src/pages/Home.tsx:26](<src/pages/Home.tsx#L26>), [src/pages/Characters.tsx:16](<src/pages/Characters.tsx#L16>) | Home's character count ignores characters the user has created. |
| 14 | BUG · low | no pattern applies | [src/components/CharacterForm.tsx:50](<src/components/CharacterForm.tsx#L50>) | An older portrait upload can replace a newer selection. |
| 15 | OVER-ABSTRACTION · low | State explicitly **not** warranted | [src/components/SettingsDialog.tsx:233](<src/components/SettingsDialog.tsx#L233>) | Delete the redundant two-value provider editor mode; do not build State classes. |

## Detailed findings

### 1. New Chat does not own or cancel an in-flight turn — BUG · critical · State / Observer

**Evidence.** [src/App.tsx:139–142](<src/App.tsx#L139-L142>) dispatches `merlink:new-chat` regardless of whether Chat is busy. [src/pages/Chat.tsx:394–411](<src/pages/Chat.tsx#L394-L411>) clears the transcript and replaces `sessionId` but leaves `loading` and the active request untouched. Its channel callback still updates the live frame at [src/pages/Chat.tsx:458–461](<src/pages/Chat.tsx#L458-L461>); completion unconditionally appends the former session's messages and project ID at [src/pages/Chat.tsx:467–488](<src/pages/Chat.tsx#L467-L488>), followed by unconditional cleanup at [src/pages/Chat.tsx:498–506](<src/pages/Chat.tsx#L498-L506>).

**Failure.** Click Send, then New Chat before the model replies. The new chat displays old streamed events and later receives the old transcript/project; its new `sessionId` no longer matches the displayed project's backend binding. If a second turn is allowed after the old one finishes, messages from one session can be submitted under another. This is not a leaking window listener: [src/pages/Chat.tsx:416–420](<src/pages/Chat.tsx#L416-L420>) does unsubscribe; it is a missing *request ownership* check.

**Recommended change.** Model each turn as an explicit request identity associated with its session (`idle | running(requestId, sessionId) | failed | complete` can be a discriminated union, not classes). New Chat should either remain disabled while sending, or cancel/retire that request. Gate the event callback, success, error, and `finally` against the identity; keep an old backend completion attached only to its original persisted session. Pass the action through props or a route-level callback rather than expanding the window event bus.

**Risk of change.** Dropping frontend callbacks must not pretend to cancel the backend: its tool writes can still finish. If cancellation is implemented, define whether the native request and tool side effects are cancellable before exposing a Cancel affordance. Exercise delayed responses plus New Chat and navigation.

### 2. Persisted sessions cannot be restored in the UI — BUG · high · Memento (conceptual)

**Evidence.** [src/pages/Chat.tsx:207–216](<src/pages/Chat.tsx#L207-L216>) initializes an empty transcript and random session ID every mount. Its mount effects fetch providers, labels, context, and progress, not messages or available sessions ([src/pages/Chat.tsx:307–350](<src/pages/Chat.tsx#L307-L350>)). Yet the backend persists incoming and assistant messages at [src-tauri/src/runtime.rs:260–275](<src-tauri/src/runtime.rs#L260-L275>) and [src-tauri/src/runtime.rs:370–400](<src-tauri/src/runtime.rs#L370-L400>), and already exposes `list_messages(sessionId)` at [src-tauri/src/lib.rs:174–177](<src-tauri/src/lib.rs#L174-L177>).

**Failure.** Open Chat, create a project, then navigate to Home and back (or restart). The interface creates a different session, shows an empty transcript, and cannot discover or return to the old session/project despite records in SQLite. History is durable but functionally inaccessible; future user input starts unrelated work.

**Recommended change.** Persist/select the active session ID and expose a list of stored sessions with their project binding; on selecting a session, hydrate messages and project/progress from the backend as one coherent restore operation. This is Memento's *restore* concern; the SQLite record is already the snapshot. Do not introduce an `Originator/Memento/Caretaker` class trio around plain immutable messages.

**Risk of change.** Existing DB rows may include partial turns; decide which records to display and how to mark interrupted sessions. Concurrent restore and send must share the request-identity guard from finding 1.

### 3. Failure after tool calls splits durable and visible conversation — BUG · high · State

**Evidence.** [src-tauri/src/runtime.rs:273–275](<src-tauri/src/runtime.rs#L273-L275>) writes incoming history, [src-tauri/src/runtime.rs:370–377](<src-tauri/src/runtime.rs#L370-L377>) writes an assistant tool-call message, and [src-tauri/src/runtime.rs:460–464](<src-tauri/src/runtime.rs#L460-L464>) writes tool results. A later model request can fail and returns only `Err` at [src-tauri/src/runtime.rs:343–353](<src-tauri/src/runtime.rs#L343-L353>), discarding `appended`. Chat only appends backend messages on `Ok` ([src/pages/Chat.tsx:467–475](<src/pages/Chat.tsx#L467-L475>)); on error it retains the user bubble and displays only an alert ([src/pages/Chat.tsx:489–507](<src/pages/Chat.tsx#L489-L507>)). A later send upserts that *incomplete UI history* at [src-tauri/src/db.rs:173–194](<src-tauri/src/db.rs#L173-L194>), while DB listing uses row order at [src-tauri/src/db.rs:198–211](<src-tauri/src/db.rs#L198-L211>).

**Failure.** The model successfully creates a project or sets fields, then the follow-up provider request times out: those effects are durable, but the user sees no tool result and cannot know whether retry is safe. The next turn omits the partial assistant/tool messages from its provider conversation while their SQLite records remain. `INSERT OR REPLACE` also moves replayed records to new rowids, so a restored transcript can have misleading ordering around the old partial messages.

**Recommended change.** Represent `partial/failed` explicitly, return or reload the messages already committed on failure, and render a truthful interrupted turn with retry semantics. Prefer a backend response carrying `appended`, project ID, and error status to a bare `Err` once writes have begun. Decide whether retries resume, repeat, or compensate non-idempotent tools; do not silently retry project creation.

**Risk of change.** Making old partial messages visible changes current transcript assumptions (`buildToolLogs` and provider wire formatting). Preserve correct tool-call/result pairing and DB ordering when migrating.

### 4. Super-json set is an unguarded read/modify/write — BUG · high · no pattern applies

**Evidence.** [src-tauri/src/super_json.rs:290–315](<src-tauri/src/super_json.rs#L290-L315>) calls `load_or_init`, updates one leaf in an in-memory document, then replaces the whole file. [src-tauri/src/super_json.rs:233–266](<src-tauri/src/super_json.rs#L233-L266>) makes each *individual replacement* atomic, but holds no per-project lock/version across the read and write. `super_json_set` is accessible from [src-tauri/src/tools.rs:344–360](<src-tauri/src/tools.rs#L344-L360>) and through the IPC tool command at [src-tauri/src/lib.rs:125–133](<src-tauri/src/lib.rs#L125-L133>).

**Failure.** Two concurrent sessions (or direct tool invocations) modify different fields of the same project; both read version A, writer 1 commits A+X, writer 2 commits A+Y, and X vanishes. The atomic rename protects against torn files, not lost updates. A UI showing progress immediately after X can then regress.

**Recommended change.** Serialize each project's read/modify/write under one lock (or use versioned compare-and-swap with retry). Keep `set` as the single mutation entry point and test two parallel updates to distinct paths. **No GoF behavioral object pattern is the fix**: this is synchronization/transactional integrity, not Visitor, Command, or Memento ceremony.

**Risk of change.** Cross-process writers need a file lock or shared database transaction rather than only an in-process mutex. Guard initialization as well as updates, and avoid holding a global lock across unrelated projects.

### 5. “Clear API key” can delete the complete provider — BUG · high · no pattern applies

**Evidence.** The Settings editor exposes Clear API key for non-custom configured providers at [src/components/SettingsDialog.tsx:977–989](<src/components/SettingsDialog.tsx#L977-L989>). It saves the same provider list with an empty key at [src/components/SettingsDialog.tsx:382–390](<src/components/SettingsDialog.tsx#L382-L390>). On the next `load_providers`, [src-tauri/src/store.rs:148–171](<src-tauri/src/store.rs#L148-L171>) deletes **every** provider with a legacy-seeded ID and blank key, and rewrites the resulting list. The migration tests only check seeded blank and configured nonblank cases ([src-tauri/src/store.rs:239–325](<src-tauri/src/store.rs#L239-L325>)).

**Failure.** An upgraded author's customized `openai`/`anthropic` legacy entry keeps its name, endpoint, and models while they remove a compromised key. The next Settings open/app restart erases the whole entry without a delete confirmation. This is not a Memento gap: the migration predicate cannot distinguish “old untouched seed” from “key deliberately cleared.”

**Recommended change.** Make migration versioned and run it once on legacy configuration, or compare the *entire* entry to an untouched seeded fixture before pruning. Add a regression covering customized legacy ID → clear key → reload, and ensure deleting a provider remains an explicit user action.

**Risk of change.** Simply removing cleanup may resurrect previously seeded blank entries for installations that have not migrated. Preserve backward compatibility with a migration marker or exact legacy-shape match.

### 6. Settings edits never notify the mounted Chat — BUG · high · Observer

**Evidence.** Shell renders the route outlet and Settings as siblings at [src/App.tsx:274–283](<src/App.tsx#L274-L283>), so opening Settings does not unmount Chat. Chat loads providers only once at [src/pages/Chat.tsx:316–318](<src/pages/Chat.tsx#L316-L318>) and disables sending with an empty list at [src/pages/Chat.tsx:926–940](<src/pages/Chat.tsx#L926-L940>). Settings writes its own `providers` state after save or delete at [src/components/SettingsDialog.tsx:360–376](<src/components/SettingsDialog.tsx#L360-L376>), without invalidating Chat.

**Failure.** First install: open Chat (no provider), add one in Settings, close Settings; Chat still says no provider and cannot send until navigation or reload. Deleting or modifying a provider likewise leaves Chat's selected credentials/models stale; a later send may fail backend validation even while the UI offers that model.

**Recommended change.** Lift provider state/loading into Shell and pass it through props/context to Chat and Settings, or use a minimal shared query cache invalidated when Settings successfully saves. This is a practical Observer subscription, **not** justification for a custom event-bus/mediator framework. Reconcile the selection against new IDs and default models atomically.

**Risk of change.** A provider edit during an active turn should affect only future turns; snapshot the provider/model selected when the request begins. Handle failed save without publishing an uncommitted list.

### 7. Fetched models can contaminate another editor — BUG · high · State (request identity)

**Evidence.** [src/components/SettingsDialog.tsx:271–311](<src/components/SettingsDialog.tsx#L271-L311>) starts an awaited model fetch and later merges the result into whichever `form` is current via `setForm(prev => ...)`. [src/components/SettingsDialog.tsx:195–205](<src/components/SettingsDialog.tsx#L195-L205>) clears only flags when the editor closes. [src/components/SettingsDialog.tsx:225–237](<src/components/SettingsDialog.tsx#L225-L237>) opens another provider and replaces `form`; the original fetch has no cancellation/identity check. The native fetch may run for 30 seconds ([src-tauri/src/adapters.rs:476–483](<src-tauri/src/adapters.rs#L476-L483>)).

**Failure.** Start Fetch models for provider A, close its editor, open provider B, then A's request resolves. Its model IDs are appended to B and may become B's default, creating a plausible but invalid configuration if Apply is pressed. Late errors/notices can also appear under B's provider.

**Recommended change.** Capture a monotonically increasing request generation and editor/provider identity; ignore success, error, and `finally` if it is not the active request. Abort the request on close/change when supported. Deduplicate against the *current* form inside the guarded state update rather than against captured `form.models`. A two-state flag is insufficient; a State **concept** is useful, not a class hierarchy.

**Risk of change.** Do not disable the editor permanently after an ignored response; reset pending state only for the matching generation. Test A→B and rapid changes to B's model rows while fetch is pending.

### 8. Provider writes use stale whole-list snapshots — BUG · medium · Command (serialize mutations)

**Evidence.** Save computes `nextProviders` from the render's `providers` before awaiting IPC ([src/components/SettingsDialog.tsx:325–363](<src/components/SettingsDialog.tsx#L325-L363>)); removal and key clearing do the same ([src/components/SettingsDialog.tsx:371–389](<src/components/SettingsDialog.tsx#L371-L389>)). Apply is disabled only by form validity, not by an in-flight write ([src/components/SettingsDialog.tsx:993–1000](<src/components/SettingsDialog.tsx#L993-L1000>)); deletion remains enabled at [src/components/SettingsDialog.tsx:600–610](<src/components/SettingsDialog.tsx#L600-L610>). Rust unconditionally replaces the entire file at [src-tauri/src/store.rs:180–216](<src-tauri/src/store.rs#L180-L216>).

**Failure.** Double-click Apply or delete another provider while Apply is pending: overlapping calls can write incompatible snapshots; the later completion overwrites both disk and local state with an older list, resurrecting a deleted entry or losing a new one. A successful toast does not guarantee the final on-disk state is that save.

**Recommended change.** Disable mutation controls while a save is pending and route save/delete/clear through a single serialized mutation function with functional state updates after confirmed persistence. If multiwindow support is required, implement backend version checks or targeted CRUD operations. Command describes serializing *operations*, not a need for one class per button.

**Risk of change.** Naively updating React state optimistically can show an entry that failed to persist; use rollback or await persistence. Avoid holding UI disabled while unrelated model fetching is in progress.

### 9. Optional story selection is not actually optional — BUG · medium · no pattern applies

**Evidence.** The form initializes `story: ""` ([src/lib/characters.ts:60–71](<src/lib/characters.ts#L60-L71>)) and offers `Select a story` as the empty value at [src/components/CharacterForm.tsx:133–142](<src/components/CharacterForm.tsx#L133-L142>). On submit, [src/components/CharacterForm.tsx:62–69](<src/components/CharacterForm.tsx#L62-L69>) replaces that choice with `projects[0]?.id`. A separate display helper explicitly handles an empty story as `Unassigned story` at [src/lib/characters.ts:83–91](<src/lib/characters.ts#L83-L91>).

**Failure.** Creating an unassigned character silently attaches them to the first demo story, skewing story and genre filters ([src/pages/Characters.tsx:37–63](<src/pages/Characters.tsx#L37-L63>)). The detail screen asserts a relationship the author never selected.

**Recommended change.** Keep `story: form.story` when the selection is optional, and show the supported unassigned label; alternatively require a selection and mark the field required. This is a plain correctness fix; no Strategy, State, or Iterator abstraction applies.

**Risk of change.** Existing saved characters with first-story IDs cannot be distinguished from intentional assignments and should not be migrated automatically.

### 10. Browser-preview projects are global rather than session-scoped — BUG · medium · State

**Evidence.** The mock stores one module-global `browserValues`, `browserProjectId`, and `browserMessages` at [src/lib/tauri.ts:146–159](<src/lib/tauri.ts#L146-L159>). Its `mockSendChat` argument has no session ID ([src/lib/tauri.ts:582–587](<src/lib/tauri.ts#L582-L587>)) and calls `project_create` only when the global project is empty ([src/lib/tauri.ts:610–620](<src/lib/tauri.ts#L610-L620>)). Chat's New Chat resets only its own local state at [src/pages/Chat.tsx:394–411](<src/pages/Chat.tsx#L394-L411>), while the mock returns the global project at [src/lib/tauri.ts:768–770](<src/lib/tauri.ts#L768-L770>).

**Failure.** In browser preview, click New Chat after creating a project: the next supposedly independent chat inherits the previous project and its 25-field progress. The mock can therefore conceal or fabricate session-binding bugs during UI testing; the native backend does not share this global design.

**Recommended change.** Index mock state by the same session ID as the native command, and keep project and messages together per session. Resetting *all* globals on New Chat would erase history and would not mirror native behavior. State is the lifecycle lens; a class-based State implementation is unnecessary.

**Risk of change.** Existing browser tests may assume one singleton mock project; update them to pass/assert session IDs. Keep the native backend as the source of truth for semantics.

### 11. Unkeyed detail/progress refresh can show the wrong project — BUG · medium · State

**Evidence.** `refreshSuperJson(id)` and `refreshSummary(id)` set shared state after their awaits, without checking the currently selected project ([src/pages/Chat.tsx:252–285](<src/pages/Chat.tsx#L252-L285>)). `refreshProgress` likewise commits regardless of whether its captured session ID is still current ([src/pages/Chat.tsx:295–305](<src/pages/Chat.tsx#L295-L305>)). An effect refreshes detail on project changes ([src/pages/Chat.tsx:343–350](<src/pages/Chat.tsx#L343-L350>)), while a completed turn separately triggers detail/progress requests ([src/pages/Chat.tsx:475–488](<src/pages/Chat.tsx#L475-L488>)); New Chat clears these values but cannot stop previous requests ([src/pages/Chat.tsx:394–405](<src/pages/Chat.tsx#L394-L405>)).

**Failure.** If a slow request for project A resolves after New Chat or after project B becomes active, A's prose/JSON/progress appears under B's title. The outline button derives enabled state from possibly stale required fields at [src/pages/Chat.tsx:520–528](<src/pages/Chat.tsx#L520-L528>), so it can enable/disable contrary to the active session.

**Recommended change.** Key each refresh to a current `{sessionId, projectId, generation}` and ignore obsolete completions; avoid duplicate detail fetches by giving the effect one responsibility and keeping explicit Refresh only for the same active project. This is request-state coordination, not a requirement for another observer bus.

**Risk of change.** Backend progress may be newer than cached detail within the same session; do not reject fresh same-session results solely because a component rerendered. Test out-of-order resolution deliberately.

### 12. Settings nested editor is not an accessible modal — BUG · medium · no pattern applies

**Evidence.** Settings focuses its *parent* close button each time its effect runs at [src/components/SettingsDialog.tsx:171–193](<src/components/SettingsDialog.tsx#L171-L193>); opening an editor changes that effect dependency at [src/components/SettingsDialog.tsx:225–237](<src/components/SettingsDialog.tsx#L225-L237>). The child advertises `role="dialog" aria-modal="true"` at [src/components/SettingsDialog.tsx:638–653](<src/components/SettingsDialog.tsx#L638-L653>) but never focuses/traps inside it or restores focus on close. The shared Dialog does implement a topmost focus trap and restoration at [src/components/Dialog.tsx:23–79](<src/components/Dialog.tsx#L23-L79>).

**Failure.** A keyboard user opens Edit and stays focused on the underlying Settings close button; Tab can traverse the parent or background instead of the provider editor. Escape is handled, but it does not make the active modal accessible. A screen reader is told two layers are modal while focus sits outside the upper one.

**Recommended change.** Reuse one dialog/focus-management primitive that understands nested layers, or implement child initial focus, topmost-only Tab trap, parent inertness, and focus restoration to the Edit trigger. **No GoF behavioral pattern applies**: this is a dialog accessibility contract, not a reason to add a Mediator.

**Risk of change.** Two independent document-level Escape listeners can close both layers if reused naively. Preserve topmost-only handling and test keyboard navigation with both Settings and the editor open.

### 13. Home count and Characters archive have different sources — BUG · low · Observer

**Evidence.** The dashboard's metric uses only `mockCharacters.length` at [src/pages/Home.tsx:13–29](<src/pages/Home.tsx#L13-L29>). The archive includes persisted custom characters at [src/pages/Characters.tsx:12–26](<src/pages/Characters.tsx#L12-L26>) and saves additions to local storage at [src/pages/Characters.tsx:72–80](<src/pages/Characters.tsx#L72-L80>).

**Failure.** Create a character, return Home: its “Characters brought to life” figure stays fixed while the archive contains one more. This is a small but user-visible contradiction in cross-page domain state.

**Recommended change.** Have both pages read a single character source, including persisted custom entries, and refresh it when created. A tiny shared hook/store or route-level state is enough; Observer does not imply Redux, a custom event bus, or a mediator.

**Risk of change.** Local-storage read can fail or contain malformed data; continue using the existing guarded reader and do not count entries the archive rejected.

### 14. Portrait FileReaders can finish in reverse order — BUG · low · no pattern applies

**Evidence.** Each upload starts a new `FileReader` at [src/components/CharacterForm.tsx:38–53](<src/components/CharacterForm.tsx#L38-L53>) and its `onload` always replaces `uploadedImage` without checking whether the user has already selected a newer file. The input offers replacement while a previous read may still be pending ([src/components/CharacterForm.tsx:185–208](<src/components/CharacterForm.tsx#L185-L208>)).

**Failure.** Pick image A, quickly pick B; if B loads first and A loads last, the preview and eventual saved character unexpectedly contain A. A slow read can also update a closed form unnecessarily.

**Recommended change.** Abort the previous reader or attach a per-selection generation to `onload`; only the latest selection may commit. Keep MIME/size validation before reading. **No pattern applies** to this one-element async race.

**Risk of change.** Cleanup must not abort the currently selected image before it finishes; include two rapid selections and close-during-upload in tests.

### 15. Provider editor `mode` is derived, not a State machine — OVER-ABSTRACTION · low

**Evidence.** Opening an editor computes `mode` solely from `provider?.kind !== "custom"` at [src/components/SettingsDialog.tsx:225–244](<src/components/SettingsDialog.tsx#L225-L244>). Newly added providers always start as custom; the mode switch is rendered as a disabled button at [src/components/SettingsDialog.tsx:668–675](<src/components/SettingsDialog.tsx#L668-L675>). Fetch and save then rederive kind from `custom` and provider ID at [src/components/SettingsDialog.tsx:257–258](<src/components/SettingsDialog.tsx#L257-L258>) and [src/components/SettingsDialog.tsx:345–350](<src/components/SettingsDialog.tsx#L345-L350>).

**Cost.** `editor.mode` appears to model transitions the UI no longer supports and forces two sources of truth (`mode`, `form.id`/persisted `kind`). A future change can make fetch select a different wire protocol than save. Calling this a GoF State candidate would make the unnecessary branch even harder to remove.

**Recommended change.** **Delete the redundant mode state and disabled pseudo-toggle**. Derive the immutable provider kind directly from the selected persisted entry (new entry defaults to custom), then share that one value between Fetch and Save. Keep `editor.id` only if needed to distinguish new from existing. **State is explicitly not applicable** to two immutable display choices.

**Risk of change.** Existing `openai`/`anthropic` entries must preserve their actual stored `kind`; do not infer an arbitrary non-custom provider's protocol from its ID alone. Regression-test editing upgraded legacy providers.

## Focused regression scenarios for implementation sprints

These are proposed tests, **not** claims that the review executed them. Keep the assertions at the UI/domain boundary rather than testing the existence of design-pattern classes.

1. Delay a chat response, press New Chat, then resolve the old response: the new transcript, project, live frame, context, and progress remain unrelated to the old session.
2. Start a chat and create a project; navigate away/back and restart: the original session can be selected with the same transcript and project binding.
3. Script a successful tool call followed by a model timeout: the UI says what persisted, the next provider request receives complete tool-call/result history, and retry does not create a duplicate project.
4. Set two separate super-json paths concurrently on one project: both final values remain present, including after reload.
5. Customize a legacy-ID provider, clear its key, close/reopen Settings: name, endpoint, models, and provider identity remain intact.
6. Open Chat before configuring a provider; save one in Settings and return: Chat offers it without route change. Delete the selected provider and verify selection reconciles safely.
7. Fetch models for provider A, close and edit B, then resolve A's delayed request: neither B's model list nor B's status notice changes.
8. Delay provider save while triggering another save/delete: disk and UI converge on one serialized operation order rather than an older snapshot.
9. Create a character with Story left on “Select a story”: the archive shows “Unassigned story,” and no specific-story filter includes them.
10. In browser preview, create a project, press New Chat, send again: the second mock session has an independent project and field values.
11. Resolve detail/progress requests in reverse order across New Chat: a stale response never restores the old project's content or outline eligibility.
12. Navigate Settings and its nested editor only with keyboard: opening focuses the child, Tab remains inside the topmost modal, Escape closes exactly one layer, and closing restores the trigger.
13. Add a custom character then navigate to Home: its metric matches the Characters archive without a reload.
14. Select portrait A then B and finish their reads B→A: B remains the preview and the saved portrait.
15. Edit old OpenAI/Anthropic entries after deleting editor `mode`: fetch and save use the same stored protocol kind, and newly added endpoints remain custom.

**Suggested sprint boundary:** fix session/turn identity and history recovery together (1–3, 11); treat file mutation and provider persistence separately (4–8); then address character/browser/a11y mismatches (9–10, 12–14), with the deletion-only cleanup (15) folded into the provider sprint. Avoid a single cross-app behavioral framework.

## What NOT to change

- **Keep the narrowly scoped backend Strategy seam.** [src-tauri/src/adapters.rs:74–99](<src-tauri/src/adapters.rs#L74-L99>) has one `LlmBackend` interface used to inject scripted replies into the agent loop; it separates network adapters from orchestration. Do not create new per-provider UI strategies simply because this backend Strategy is justified.
- **Keep one honest send path.** [src/pages/Chat.tsx:431–435](<src/pages/Chat.tsx#L431-L435>) and [src/pages/Chat.tsx:639–649](<src/pages/Chat.tsx#L639-L649>) route ordinary text and outline through the same `sendMessage` operation; fix its lifecycle, do not fork it into separate Command subclasses. The [src/pages/Chat.tsx:416–420](<src/pages/Chat.tsx#L416-L420>) listener already cleans up correctly; do not report a nonexistent leak.
- **Keep simple iteration and props.** [src/pages/Characters.tsx:24–63](<src/pages/Characters.tsx#L24-L63>) filters one normalized character array; [src/components/CharacterDetail.tsx:70–87](<src/components/CharacterDetail.tsx#L70-L87>) handles the few relationship shapes locally. Neither needs Iterator or Visitor objects. Shared provider/character ownership can be a small parent prop or hook rather than a global Mediator.
- **Keep atomic replacement, but strengthen its surrounding transaction.** [src-tauri/src/super_json.rs:239–266](<src-tauri/src/super_json.rs#L239-L266>) and [src-tauri/src/store.rs:191–216](<src-tauri/src/store.rs#L191-L216>) avoid torn files; the findings above concern read/modify/write isolation and stale whole-list input, not replacing atomic writes with direct writes.
- **Do not add Memento classes for immutable React state or generic undo machinery without an undo feature.** Restore the actual saved chat session first; simple snapshots, existing SQLite records, and explicit request IDs suffice. Chain of Responsibility, Interpreter, Template Method, and Visitor have no compelling UI/domain use in the examined flows.
