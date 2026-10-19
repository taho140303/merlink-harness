# Structural GoF review — Reviewer A: wiring and boundaries

## Scope examined

- Programme: `docs/refactor/README.md`; project GoF skill, all seven structural references (Adapter, Bridge, Composite, Decorator, Facade, Flyweight, Proxy), and `references/criticism-and-modernity.md`.
- Frontend: all 24 `src/**/*.ts` and `src/**/*.tsx` files, approximately **5,504 lines**. Closest inspection: `src/lib/tauri.ts` (844), `src/pages/Chat.tsx` (980), `src/components/SettingsDialog.tsx` (1,008), `src/App.tsx` (301), and shared types; also reviewed character, markdown, preferences, other pages/components, and mock data.
- Native: all 12 `src-tauri/src/*.rs` files, approximately **7,481 lines**. Closest inspection: `runtime.rs` (1,809, including tests), `adapters.rs` (1,004), `tools.rs` (879), `super_json.rs` (1,513), `db.rs` (327), `store.rs` (326), `lib.rs` (207), `paths.rs` (85), and representative IPC contract tests (916).
- Total inspected inventory: approximately **12,985 lines**; test-heavy sections were sampled for relevant contracts. This is source analysis, not a runtime reproduction. No implementation or test source changed.
- Interpretation: **Adapter** earns its name at provider-wire and UI/native conversions; **Facade** at cohesive agent/session APIs; **Proxy** only when an access boundary actually controls a service. Concurrency, migration and stale React state remain bugs even where **no pattern applies**. **Bridge**, **Composite**, **Decorator** and **Flyweight** are not justified by the observed dimensions/data volumes; see the final section.

## Ranked findings

| Rank | Classification | Structural pattern | Principal evidence | Impact in one line |
| ---: | --- | --- | --- | --- |
| 1 | **BUG** | Proxy (missing access control) | `src-tauri/src/lib.rs:125-133` | Public tool command can mutate a project unrelated to its session. |
| 2 | **BUG** | Adapter | `src/components/SettingsDialog.tsx:345-353` | New Anthropic providers are forced onto OpenAI-compatible wire format. |
| 3 | **BUG** | no pattern applies | `src/pages/Chat.tsx:394-411` | An old in-flight reply can repopulate a newly reset chat. |
| 4 | **BUG** | Facade (storage boundary) | `src-tauri/src/paths.rs:7-15` | Windows without `HOME` cannot open any persistent data. |
| 5 | **BUG** | no pattern applies | `src-tauri/src/tools.rs:249-261` | Concurrent project creation can bind separate chats to one document. |
| 6 | **BUG** | no pattern applies | `src-tauri/src/db.rs:19-25` | Globally keyed message upserts can delete another chat's message. |
| 7 | **BUG** | no pattern applies | `src/pages/Chat.tsx:316-318` | Saving a first provider leaves already-open Chat unable to send. |
| 8 | **BUG** | Adapter | `src/lib/tauri.ts:155-157` | Browser preview reuses one project across supposedly new sessions. |
| 9 | **BUG** | Adapter | `src-tauri/src/adapters.rs:285-314` | Malformed provider tool requests are silently discarded or passed broken. |
| 10 | **BUG** | Facade (read boundary) | `src-tauri/src/runtime.rs:528-536` | Reading an unknown project creates a ghost document. |
| 11 | **BUG** | Facade (migration boundary) | `src-tauri/src/store.rs:148-173` | Loading configuration may silently delete edited blank-key providers. |
| 12 | **BUG** | Adapter | `src-tauri/src/adapters.rs:101-140` | A valid chat-completions URL produces an invalid models URL. |
| 13 | **BUG** | Facade (persistence boundary) | `src-tauri/src/db.rs:223-233` | Corrupted stored messages silently lose tool/history metadata. |
| 14 | **SMELL** | Adapter | `src/lib/tauri.ts:788-843` | Generic IPC casts and hand-copied tests conceal contract drift. |
| 15 | **SMELL** | Adapter | `src/lib/tauri.ts:548-564` | Preview reports completeness despite invalid/missing project fields. |
| 16 | **OVER-ABSTRACTION** | Facade (false positive) | `src-tauri/src/runtime.rs:507-526` | Trivial pass-through methods widen runtime's surface for no benefit. |

## Detailed findings

### 1. Guard the native tool command, or remove it entirely — BUG / Proxy

**Evidence:** `src-tauri/src/lib.rs:125-133` registers `execute_tool` as IPC and directly forwards caller-supplied `project_id` and `session_id`. `src-tauri/src/runtime.rs:519-526` forwards them unchanged. `src-tauri/src/tools.rs:221-230` checks only that the project string is nonempty; `src-tauri/src/tools.rs:344-360` writes using precisely that string. In contrast, the agent path resolves the effective binding before each call at `src-tauri/src/runtime.rs:414-431`. `src-tauri/src/ipc_contract_tests.rs:254-256` says there is no frontend caller.

**Why it matters:** A stale/erroneous IPC caller, or script executing inside the app webview, can call `super_json_set` against another project's ID despite passing an unrelated session. Path sanitisation prevents directory traversal, not this session-to-project mismatch. This bypasses the domain ownership rule and can overwrite the wrong project document.

**Recommended change:** Prefer **deleting** this exported command if it exists only for tests; call `tools::execute_tool` from Rust tests instead. If a real UI caller needs it, make the native command a guarding Proxy: derive project from `db::project_for_session(session_id)`, reject a mismatch, and never trust an independently supplied project ID. Keep agent-internal calls on the resolved binding.

**Risk of change:** Existing IPC contract tests deliberately exercise direct tool calls; migrate them to service-level tests or create valid session bindings first. Do not mistake `paths::safe_segment` for authorization.

### 2. Make provider kind selectable independently of its user ID — BUG / Adapter

**Evidence:** A new provider form is always `custom` at `src/components/SettingsDialog.tsx:220-245`; `providerKind()` sends `custom` for model fetch at `src/components/SettingsDialog.tsx:253-258`, and save writes `kind: "custom"` at `src/components/SettingsDialog.tsx:345-353`. `src-tauri/src/store.rs:140-145` ships no seed catalog. Native dispatch only selects Anthropic for `kind == "anthropic"` at `src-tauri/src/adapters.rs:86-96`, while `src-tauri/src/adapters.rs:541-560` implements a genuinely different Messages API.

**Why it matters:** A fresh user adding Anthropic's URL/key cannot select Anthropic's wire protocol in Settings. Fetch sends an OpenAI Bearer request, and chat posts `/chat/completions` rather than `/v1/messages`; both fail. Hand-editing JSON is the only escape. Existing legacy `id == "anthropic"` entries may work, but the migration removes untouched blank-key seeds.

**Recommended change:** Add an explicit persisted provider `kind` choice (OpenAI-compatible versus Anthropic), independent of `id`, and use the same choice for model discovery and chat. Only the provider-specific Adapter needs to know headers and endpoints; keep friendly provider names user-defined.

**Risk of change:** Preserve old stored `kind` values and editing of already-configured entries; add a fresh-install UI-to-native contract test for both wire formats. Do not create one Adapter subclass per brand if their protocols coincide.

### 3. Invalidate in-flight work when New chat changes session — BUG / no pattern applies

**Evidence:** The global sidebar dispatches `merlink:new-chat` at `src/App.tsx:134-142` without checking activity. Its mounted Chat listener resets messages, session, project and live display at `src/pages/Chat.tsx:390-421`. Meanwhile `sendMessage` has already captured old `sessionId` and `projectId` at `src/pages/Chat.tsx:463-474`; the channel callback and response still set live steps, messages, project and detail at `src/pages/Chat.tsx:456-460,475-488`, and `finally` clears current loading state at `src/pages/Chat.tsx:498-506`.

**Why it matters:** Click New chat while a slow provider or preview turn runs: the old reply appears in the new empty conversation, and its project becomes the new chat's displayed project. A second send during the transition is also blocked by the previous turn's `loading` flag.

**Recommended change:** Introduce a local request generation/session guard; only a request still belonging to the visible session may emit UI events, append messages, update project/details, or clear that session's loading indicator. Cancellation is optional but should not be confused with preventing already-executed native tool writes.

**Risk of change:** Ignoring a stale reply must not imply that the backend rolled back its old session or tool operations. Test New chat during success, rejection and streamed events.

### 4. Stop requiring a Unix-style `HOME` on Windows — BUG / Facade

**Evidence:** `src-tauri/src/main.rs:1-2` explicitly contains a Windows release target. Every persistent path goes through `src-tauri/src/paths.rs:7-23`, which reads only `MERLINK_HOME` or `HOME` and returns `"HOME is not set"` otherwise. Both SQLite opening at `src-tauri/src/db.rs:32-48` and provider config at `src-tauri/src/store.rs:156-160` use that function; neither considers Windows' `USERPROFILE`/known folders.

**Why it matters:** A normal Windows process need not have a `HOME` variable. In that environment the app cannot load providers, open SQLite, send a chat turn or create project files at all. This is a storage Facade portability bug, not a reason for more OO layers.

**Recommended change:** Resolve a platform user-data directory with Tauri/OS path APIs, retaining `MERLINK_HOME` as an explicit test override. Select a stable location and document migration or continued lookup of existing `~/.merlink` data where that path already exists.

**Risk of change:** Changing the root can strand existing installations or violate a previously documented location; include Windows integration coverage with `HOME` unset, plus migration/compatibility tests.

### 5. Allocate project IDs atomically in the database — BUG / no pattern applies

**Evidence:** `src-tauri/src/tools.rs:249-261` obtains all existing IDs, selects the first unused slug, then `src-tauri/src/tools.rs:295-321` writes project, file and session binding as separate steps. `src-tauri/src/db.rs:80-93` uses `INSERT OR IGNORE` for `create_project`, so a duplicate does not signal failure. `src-tauri/src/db.rs:152-170` also upserts the session binding without refusing an already-bound session.

**Why it matters:** Two simultaneously running chats asking for the same title can select the same ID. One insert is ignored, but both tools continue as if successful, write one shared `super.json`, and bind both sessions to it; a repeat `project_create` race in one session can also silently change its owner.

**Recommended change:** Reserve a unique ID with a real unique INSERT inside a transaction and retry suffixes on constraint failure; make session binding conditional on being empty and check affected rows. Coordinate the file write with reservation so a failure cannot leave an apparently successful binding to an incomplete document.

**Risk of change:** Existing duplicated bindings cannot automatically be disambiguated; plan a diagnostic rather than overwriting one project's work. A SQLite transaction does not make filesystem writes transactional.

### 6. Do not use a globally unique ID to replace arbitrary session messages — BUG / no pattern applies

**Evidence:** The `messages.id` column is the sole primary key, not `(session_id,id)`, at `src-tauri/src/db.rs:19-25`; `src-tauri/src/db.rs:173-194` applies `INSERT OR REPLACE`, replacing any previous row with that ID regardless of session. The native runtime trusts all incoming frontend message IDs at `src-tauri/src/runtime.rs:260-275`. React generates user IDs from `Date.now()` alone at `src/pages/Chat.tsx:435-453`, unlike the counter-backed native IDs at `src-tauri/src/runtime.rs:133-142`.

**Why it matters:** Two user sends in different chats in one millisecond, or replay of a reused client ID, can erase the old chat's message and insert the new one in its place. `REPLACE` also changes rowid, defeating the advertised stable insertion ordering.

**Recommended change:** Generate collision-resistant client IDs and enforce an upsert that *cannot* change a row's session ownership. Prefer a composite `(session_id,id)` key or conditional `UPDATE` plus `INSERT`; test duplicate IDs across sessions and stable ordering across re-sends.

**Risk of change:** Changing an existing SQLite primary key needs a data migration and careful handling of pre-existing duplicates. Do not simply drop idempotence: a resent frontend history currently depends on it.

### 7. Synchronize provider saves with the mounted chat — BUG / no pattern applies

**Evidence:** Chat loads providers once on mount at `src/pages/Chat.tsx:316-318,352-375`, and refuses sends when its local list is empty at `src/pages/Chat.tsx:435-438`. The Settings dialog is mounted separately at `src/App.tsx:274-283`; successful save only updates its own local state at `src/components/SettingsDialog.tsx:356-364`.

**Why it matters:** On a fresh install with no providers, opening Settings from Chat and saving the first provider leaves Chat's list empty and Send disabled until the route remounts or the app reloads. Changes to models/keys likewise leave the chat selector stale.

**Recommended change:** Lift provider data to the common app owner or publish a narrowly scoped settings-saved notification and reload Chat. This is ordinary shared state, **not** an excuse for a Proxy, global event bus or new storage service.

**Risk of change:** If reloading resets the selected provider/model, preserve a valid current selection when possible; clear it only when removed.

### 8. Give the browser-preview Adapter actual session boundaries — BUG / Adapter

**Evidence:** Preview storage consists of one global `browserValues` map, `browserProjectId` and `browserMessages` at `src/lib/tauri.ts:142-159`; `mockSendChat` accepts no session/project arguments at `src/lib/tauri.ts:582-587`, and uses the global project to decide whether to call `project_create` at `src/lib/tauri.ts:604-619`. The `list_messages` and `super_json_read` mocks ignore request arguments at `src/lib/tauri.ts:810-818`. New chat resets only React state at `src/pages/Chat.tsx:394-411`.

**Why it matters:** In `pnpm dev`, start a story, click New chat and send a second title: the preview continues editing the first project and can display the first document in the new chat. Its return metadata even hard-codes `session_id: "browser-preview"` at `src/lib/tauri.ts:718-723`, so the preview does not exercise the desktop ownership contract.

**Recommended change:** Pass `sessionId` through the mock, maintain small per-session project/message maps and per-project values, or explicitly disable project creation/edits in browser preview. Keep it a small test double, not a second production domain engine.

**Risk of change:** Existing visual fixtures rely on current canned providers/transcripts; update browser checks deliberately and avoid persisting mock data into a real user's store.

### 9. Fail closed on malformed provider tool calls — BUG / Adapter

**Evidence:** `src-tauri/src/adapters.rs:285-314` silently `filter_map`s OpenAI tool calls missing function/name and uses blank IDs; string arguments that are invalid JSON become a raw string at `src-tauri/src/adapters.rs:294-301`. `src-tauri/src/adapters.rs:340-368` accepts Anthropic `tool_use` blocks with blank IDs/names and null input. `src-tauri/src/runtime.rs:370-375,414-465` persists and executes whatever survived parsing.

**Why it matters:** A provider/proxy response with both explanatory text and a malformed tool call can appear to succeed while never writing requested changes. Blank tool IDs break matching on the next round; invalid arguments fail later at the tool boundary with misleading messages instead of exposing a malformed provider response.

**Recommended change:** Distinguish legitimate optional content/usage from required tool-call fields. Parse each declared call as `Result`, reject missing/blank IDs or names and malformed JSON argument strings at the provider Adapter, and surface a clear response error. Keep the documented object-valued proxy-argument compatibility if useful.

**Risk of change:** Some nonconforming providers currently limp through; add fixture tests for tolerated dialects and malformed calls before tightening acceptance.

### 10. Do not let a project read create an unregistered project — BUG / Facade

**Evidence:** IPC `super_json_read` and `super_json_summary` take caller-supplied `project_id` at `src-tauri/src/lib.rs:135-138,147-155`; their runtime wrappers call `load_or_init` at `src-tauri/src/runtime.rs:528-536`. On missing file, `src-tauri/src/super_json.rs:218-230` creates a skeleton. `src-tauri/src/paths.rs:26-53` validates a safe path segment but does not confirm an existing database project or a session binding.

**Why it matters:** A typo, stale UI state or direct read creates a project directory and document even if no SQLite project exists. The UI receives apparently valid empty details, masking the incorrect ID; a later actual project with that slug can inherit the ghost file.

**Recommended change:** Separate read-existing from create-on-project-creation. Validate the requested project exists, preferably use a session-scoped read Facade where ownership is intended, and return NotFound on a missing file instead of creating it.

**Risk of change:** Existing tests (`src-tauri/src/ipc_contract_tests.rs:426-439`) explicitly expect read-side initialization; update them and decide how to handle already-created orphan files.

### 11. Match the legacy seed's shape, not only ID plus empty key — BUG / Facade

**Evidence:** `src-tauri/src/store.rs:13-24` lists old IDs. `src-tauri/src/store.rs:148-154` considers every entry with one of those IDs and no API key untouched; `src-tauri/src/store.rs:156-173` filters it and immediately saves the shortened list. The old fixture at `src-tauri/src/store.rs:67-76` contains more distinguishing fields (name, endpoint, models).

**Why it matters:** A user-modified legacy `openai`/`anthropic` entry with edited URL, name or models but a temporarily cleared API key is permanently deleted on next read. The read Facade silently mutates config and cannot distinguish an edited record using the current predicate.

**Recommended change:** Compare the complete known seed shape, or use a migration/version marker that only removes verified pristine records. Back up existing config before one-shot destructive migration and make ambiguous cases visible rather than dropping data.

**Risk of change:** Preserving more legacy records can show unwanted untouched entries for older installations; choose possible extra clutter over deleting authored configuration.

### 12. Normalize model-list URLs consistently with chat URLs — BUG / Adapter

**Evidence:** `src-tauri/src/adapters.rs:101-108` accepts a configured OpenAI-compatible endpoint already ending `/chat/completions`; `src-tauri/src/adapters.rs:126-140` instead appends `/models` to that same URL. `src-tauri/src/adapters.rs:491-518` uses the latter for Settings' Fetch models action.

**Why it matters:** Chat works against `https://host/v1/chat/completions`, while Fetch models calls `https://host/v1/chat/completions/models` and fails. Users see contradictory diagnostics for one provider configuration.

**Recommended change:** Parse and normalize the configured API root once; strip recognized chat endpoint suffix before deriving `/models`, with tests for root URL, `/v1`, completed endpoint and trailing slash. Do not assume arbitrary providers expose model discovery at all.

**Risk of change:** A custom server may intentionally place models beneath an unusual path; document the supported URL forms or allow an explicit models URL only when needed.

### 13. Surface damaged persisted message payloads instead of fabricating history — BUG / Facade

**Evidence:** `src-tauri/src/db.rs:203-233` reads `payload_json` but, on JSON decode failure, silently substitutes `MessageEnvelope::new("recovered-{index}", role, content)`; the same fallback applies to legitimate older rows with no payload. `src-tauri/src/runtime.rs:260-275,312-314` persists and reuses incoming history, including tool calls/results.

**Why it matters:** A truncated or schema-incompatible payload loses tool IDs, call arguments, metadata and stable message identity, but `list_messages` returns a plausible-looking transcript instead of an error. `context_usage` also reads that substituted data at `src-tauri/src/runtime.rs:550-560`; a later history reload/replay would pass incomplete tool history to a provider. Real legacy null payloads and corrupt modern JSON require different treatment.

**Recommended change:** Preserve the explicit fallback for known legacy `NULL` payloads, but return a contextual error for present-but-invalid JSON or run an explicit, observable migration. Include row/session identifiers in diagnostics without exposing secrets.

**Risk of change:** Existing damaged databases that previously limped through will now fail to load; offer a repair/export path rather than hiding the corruption.

### 14. Make IPC's command/argument/return pairing type-safe — SMELL / Adapter

**Evidence:** `src/lib/tauri.ts:788-844` defines `safeInvoke<T>(cmd: string, args?: unknown)` and repeatedly coerces arbitrary values `as T`; for example, Chat passes an object to `safeInvoke<ChatTurn>("send_chat", …)` at `src/pages/Chat.tsx:463-474`. The Rust command signature fixes exact camelCase arguments at `src-tauri/src/lib.rs:105-123`; the tests manually copy frontend-shaped bodies at `src-tauri/src/ipc_contract_tests.rs:237-261` and only scan text for keys at `src-tauri/src/ipc_contract_tests.rs:851-887`.

**Why it matters:** TypeScript permits a wrong command name, misspelled argument or fabricated return type while tests may keep validating an old hand-copied request. A new IPC command has at least three representations to maintain. This is a seam that merits a small Adapter, not a large generated framework.

**Recommended change:** Export command-specific typed functions or one typed command map linking literal name, argument and response; have UI call that boundary and keep Tauri's real IPC integration tests. Extract the 600+ lines of mock behavior away from the transport module so neither side hides the other.

**Risk of change:** A broad rewrite can disturb working channel serialization and preview flows. Migrate one command at a time and retain the native contract tests' negative camelCase coverage.

### 15. Trim browser mock validation instead of pretending it is native validation — SMELL / Adapter

**Evidence:** Browser `super_json_set` accepts any nonempty path and value at `src/lib/tauri.ts:548-555`, while native `src-tauri/src/super_json.rs:289-315` whitelists and type-checks every field. Mock `super_json_missing` always returns an empty missing list at `src/lib/tauri.ts:561-564`, despite a separate real mock-required-path calculation at `src/lib/tauri.ts:161-177`.

**Why it matters:** Browser manual checks can claim all eleven required fields are complete or show a successful write for a path the native app rejects. The preview acts as a false-positive test of agent/tool workflows.

**Recommended change:** At minimum reuse `mockMissingRequired()` for missing fields and reject unknown keys/types for exercised preview paths. If maintaining validation parity costs too much, label tool simulation as intentionally incomplete and keep native contract tests authoritative. **Delete** any mock-only domain rules not needed to render the UI.

**Risk of change:** A full TypeScript port of the Rust schema would create more drift; choose a minimal demonstrable subset or contract-fixture-based preview, not another 1,500-line validator.

### 16. Remove thin wrappers that do not form a meaningful facade — OVER-ABSTRACTION / Facade

**Evidence:** `src-tauri/src/lib.rs:185-190` calls `runtime::fetch_models_internal`, which only calls `adapters::fetch_models` at `src-tauri/src/runtime.rs:507-517`. Likewise `src-tauri/src/lib.rs:125-133` calls `runtime::execute_tool_internal`, which only calls `tools::execute_tool` at `src-tauri/src/runtime.rs:519-526`. Unlike `run_agent_turn` at `src-tauri/src/runtime.rs:224-233,260-275`, these wrappers compose no policy, conversion, lifecycle or state.

**Why it matters:** Putting every native command through `runtime` makes the module look like an all-service Facade but hides the actual owner of provider fetching and tool execution. It adds call surfaces and encourages more unrelated forwarding methods, while obscuring the missing guard in finding 1.

**Recommended change:** **Delete** pure forwarding wrappers and call their service modules from command handlers; retain `run_agent_turn`/`send_chat_internal`, which coordinate provider loading, persistence, streaming and tool rounds. If `execute_tool` stays exported, its replacement must implement the session guard from finding 1 rather than remain a pass-through.

**Risk of change:** Online tests may import `fetch_models_internal`; update call sites deliberately and keep native command behavior/camelCase signatures unchanged.

## What NOT to change

- Keep the real provider Adapters: `src-tauri/src/adapters.rs:86-96,521-560` converts **two actually incompatible** OpenAI/Anthropic formats; `src-tauri/src/adapters.rs:567-589` and `src-tauri/src/runtime.rs:224-233` show a useful scripted test seam. A separate class per vendor is unnecessary; **Bridge** does not earn two independent hierarchies here.
- Keep a single narrow `run_agent_turn` orchestration boundary and `ProgressSink` at `src-tauri/src/runtime.rs:31-55,224-233`: Tauri channels and recording tests legitimately differ. The special `NoopSink` at `src-tauri/src/runtime.rs:39-46` is only test-used and is optional to delete, not grounds for a general sink framework.
- Keep project IDs path-safe at `src-tauri/src/paths.rs:26-53`, but do not claim validation substitutes for session ownership. Keep SQLite WAL/busy-timeout and per-operation connections at `src-tauri/src/db.rs:32-59`; the project ID race needs a transaction, not a connection pool abstraction.
- Keep small React presentation composition such as `src/components/Dialog.tsx:17-23` and `src/components/SelectBox.tsx:14-26`; these are UI reuse, **not** evidence that GoF Composite or Decorator should be imposed on the domain.
- Do **not** introduce Flyweight for 25 schema leaves or a handful of providers, Proxy for simple provider fetching, Composite for flat tool specs, or Decorator wrappers around a single HTTP backend. There is no measured scale or independent dimension warranting them. Where a finding is labelled **no pattern applies**, fix the bug directly.
