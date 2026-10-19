# Creational GoF review — Reviewer B: boundaries and lifecycle

## Scope and method

Read the programme rules and the GoF skill, all five creational references (Abstract Factory, Builder, Factory Method, Prototype, Singleton), and the modernity/criticism reference. This is an independent, read-only source review: no application code was changed. I traced IPC commands through Rust persistence and provider adapters, then traced their results back into React state. Line counts below are approximate full-file sizes, not a claim that every test line was analyzed equally.

| Area | Source examined (approximate lines) | Boundary checked |
| --- | --- | --- |
| Native entry, data, and tools | `src-tauri/src/lib.rs` (207), `db.rs` (327), `store.rs` (326), `tools.rs` (879), `models.rs` (322), `paths.rs` (85) | Tauri commands, SQLite row construction, provider JSON, project creation. |
| Native agent and serialization | `src-tauri/src/runtime.rs` (1,809), `adapters.rs` (1,004), `super_json.rs` (1,513), plus test support and relevant IPC tests | Turn persistence, wire-format selection, filesystem read/modify/write and schema defaults. |
| Frontend and preview | `src/pages/Chat.tsx` (980), `Home.tsx` (222), `Characters.tsx` (231), `src/components/SettingsDialog.tsx` (1,008), `CharacterForm.tsx` (227), `src/lib/tauri.ts` (844), `characters.ts` (122), `src/data/mock.ts` (539), `src/types.ts` (214), `src/App.tsx` (301); also skimmed other pages/components and preferences | Session ownership, localStorage hydration, provider editing, native/preview differences. |

**Interpretation of the patterns.** A Rust helper returning a value or a TypeScript object literal is not automatically a GoF pattern. Factory Method can be a small, fallible boundary-construction function; Singleton describes the *lifetime problem* with unscoped mutable state, not a recommendation to introduce `getInstance()`. Neither Builder nor Prototype earns an implementation here: present construction is mostly named struct fields, spreads, and a JSON skeleton. Abstract Factory needs several compatible product categories across several variants; choosing one provider wire format or desktop versus browser preview does not meet that threshold. Findings explicitly marked **no pattern applies** should be fixed without pattern machinery.

## Ranked summary

| Rank | Severity | Pattern lens | Exact location | Impact in one line |
| ---: | --- | --- | --- | --- |
| 1 | **BUG** | Factory Method | `src-tauri/src/db.rs:182-210` | Replaying message IDs via SQLite `REPLACE` changes insertion order and can replace another session's message. |
| 2 | **BUG** | Singleton (avoid) | `src-tauri/src/super_json.rs:299-315` | Concurrent edits of different super-JSON fields lose a successfully reported write. |
| 3 | **BUG** | Factory Method | `src-tauri/src/tools.rs:295-321` | Racing project creates can silently reuse an ID; partial failure leaves orphaned state. |
| 4 | **BUG** | no pattern applies | `src/pages/Chat.tsx:394-410` | A late old-chat response can populate the newly reset conversation. |
| 5 | **BUG** | no pattern applies | `src/components/SettingsDialog.tsx:225-258` | Fresh installs cannot construct an Anthropic-kind provider through Settings. |
| 6 | **BUG** | no pattern applies | `src-tauri/src/store.rs:152-171` | Migration deletes edited legacy providers merely because their API key is blank. |
| 7 | **SMELL** | Factory Method | `src-tauri/src/super_json.rs:218-229` | Existing project JSON is never reconciled with a changing skeleton. |
| 8 | **BUG** | no pattern applies | `src-tauri/src/runtime.rs:343-375` | A failed later provider round persists an incomplete tool-request transcript. |
| 9 | **SMELL** | no pattern applies | `src-tauri/src/db.rs:5-30` | `CREATE TABLE IF NOT EXISTS` provides no upgrade path for older database shapes. |
| 10 | **BUG** | Factory Method | `src-tauri/src/db.rs:209-231` | Broken payload JSON is silently converted into fabricated, metadata-free messages. |
| 11 | **BUG** | no pattern applies | `src/pages/Chat.tsx:39-47` | Reload/navigation generates a new session with no way to find the persisted conversation. |
| 12 | **BUG** | no pattern applies | `src/lib/characters.ts:94-103` | An array-shaped but invalid stored character can crash search/filter rendering. |
| 13 | **SMELL** | no pattern applies | `src/components/SettingsDialog.tsx:356-362` | Whole-list provider saves overwrite another window's changes and credentials. |
| 14 | **OVER-ABSTRACTION** | Singleton (avoid) | `src/lib/tauri.ts:155-159` | Parallel browser-backend simulation has global cross-chat state and lies about missing fields. |
| 15 | **SMELL** | Singleton (avoid) | `src-tauri/src/adapters.rs:469-484` | Building a new HTTP client every round discards connection pooling and repeats setup. |
| 16 | **BUG** | no pattern applies | `src-tauri/src/lib.rs:125-137` | IPC callers can create/read/edit a project document without a matching project/session binding. |

## Detailed findings

### 1. SQLite upsert destroys the transcript's ordering invariant — BUG; Factory Method

- **Evidence:** `src-tauri/src/runtime.rs:260-275` re-appends *every* client-provided historical message on each turn, and `src-tauri/src/runtime.rs:370-400` appends the final assistant message twice to attach metadata. `src-tauri/src/db.rs:173-195` implements append as `INSERT OR REPLACE` keyed by `messages.id`; `src-tauri/src/db.rs:198-210` promises insertion order but sorts by `rowid`. The id is a table-wide primary key (`src-tauri/src/db.rs:19-25`); the UI generates user ids from a millisecond clock (`src/pages/Chat.tsx:446-450`).
- **Why it matters:** SQLite `REPLACE` deletes the old row and inserts a new one: replay moves earlier messages to a later `rowid`. A subsequent `list_messages` can show an altered conversation, break user/tool/assistant order and feed misleading context estimates; duplicate IDs between sessions can *move* a message into the wrong session. This is not an idempotent append.
- **Recommended change:** Make persistence a narrowly specified **Factory Method** for one new, validated message: immutable `(session_id, message_id, ordinal)` identity; insert new IDs only, update payload in place when enriching metadata, and order by an explicit per-session sequence. Stop blindly replaying client history or verify that it agrees with stored session history. Test a two-turn transcript, metadata update, and duplicate IDs in separate sessions.
- **Risk of change:** Existing rowids have already shifted; migrating existing transcript order needs a conservative policy, not a claimed reconstruction of order that can no longer be inferred.

### 2. Atomic rename does not make super-JSON updates serializable — BUG; Singleton (avoid)

- **Evidence:** `src-tauri/src/super_json.rs:289-315` reads the whole document, mutates one field, and saves the whole document; `src-tauri/src/super_json.rs:233-267` atomically renames a temporary file. Tools and direct IPC can invoke this independently (`src-tauri/src/tools.rs:344-354`, `src-tauri/src/lib.rs:125-133`).
- **Why it matters:** Two commands can both read version A, set different fields, and rename A+X and A+Y. Both return success while one author's answer disappears. Atomic replacement prevents torn JSON, **not** lost writes; the per-project document is shared state with no defined synchronization lifetime.
- **Recommended change:** Inject a shared **per-project** lock into operations, or use optimistic version checks and retry, or move fields into a database transaction. Treat this as a Singleton-*scope* question: one coordinator for the same document, not an application-global `SuperJsonSingleton`. Cover concurrent writes to different required fields.
- **Risk of change:** A coarse app-wide lock would unnecessarily serialize unrelated projects; migrating the file into SQLite requires recovery and backup planning.

### 3. Project creation has no atomic commit point — BUG; Factory Method

- **Evidence:** `src-tauri/src/tools.rs:295-321` separately checks session binding, selects a slug, inserts a project, initializes the JSON file, sets its name, and binds the session. The slug check is an unlocked snapshot (`src-tauri/src/tools.rs:249-261`); `src-tauri/src/db.rs:82-93` uses `INSERT OR IGNORE`, so a conflict is reported as success rather than a retry condition.
- **Why it matters:** Two requests for the same title can choose the same ID and each report successful creation of the one row; two creates in one session can race past the one-project check. A file initialization or binding failure leaves a DB project or filesystem document without the promised pair. Retrying can create a new suffixed project and leave the first orphan.
- **Recommended change:** Make `project_create` the sole fallible **Factory Method** that commits or reports a recoverable failure. Use a transaction with a uniqueness-enforcing insert (retry suffix collisions), serialize creation for one session, and stage/reconcile the JSON file when coordinating SQLite and filesystem. Test duplicate-title and same-session concurrent creates plus injected file failure.
- **Risk of change:** SQLite and filesystem do not share a transaction; recovery policy for pre-existing partial projects must be explicit.

### 4. New chat does not retire an in-flight turn — BUG; no pattern applies

- **Evidence:** `src/pages/Chat.tsx:394-410` clears the transcript and generates another session id but does not invalidate the outstanding send/channel. `src/pages/Chat.tsx:458-476` continues to apply events and the old response with unconditional state setters; `src/pages/Chat.tsx:488-495` likewise can surface the old error. The request captures the previous `sessionId` at `src/pages/Chat.tsx:467-474`.
- **Why it matters:** Press New chat while a request runs and its assistant answer/project binding appears inside the new chat despite having been saved under the old session in SQLite. The UI now sends subsequent messages using a new session id but an old project id/transcript: a boundary/lifecycle split, not merely a cosmetic progress glitch.
- **Recommended change:** Give each turn a captured session/epoch; ignore completion and stream events unless it is still the active epoch. Optionally cancel the transport, but still guard late completions. Decide whether a new chat may begin immediately while an old one is loading.
- **Risk of change:** Canceling a UI wait does not undo tools already committed on the backend; surface their recoverable session identity rather than pretending they never ran.

### 5. The editor cannot create the Anthropic wire-format variant — BUG; no pattern applies

- **Evidence:** `src/components/SettingsDialog.tsx:220-258` always starts added providers as `custom` and derives kind `custom` from that mode. `src/components/SettingsDialog.tsx:325-353` saves every new provider with that kind. `src-tauri/src/store.rs:140-145` ships no provider catalog; `src-tauri/src/adapters.rs:92-96` and `src-tauri/src/adapters.rs:502-509` select Anthropic messages and `x-api-key` only when kind is `anthropic`.
- **Why it matters:** On a fresh install a user can enter an Anthropic URL/key but the app calls OpenAI-style `/models` and `/chat/completions` with Bearer auth; fetching models and chatting fail. Imported legacy entries can work, making the regression easy to miss.
- **Recommended change:** Let the author explicitly choose an API protocol when adding a provider, store that kind and use the same kind for both model discovery and chat. Existing default can remain OpenAI-compatible. A discriminated union is sufficient; **no Abstract Factory applies** because this chooses one wire protocol, not related families of constructed products.
- **Risk of change:** Do not auto-relabel existing `custom` entries based on hostname: compatible proxies intentionally use arbitrary endpoints.

### 6. Provider cleanup mistakes a blank key for an untouched seed — BUG; no pattern applies

- **Evidence:** `src-tauri/src/store.rs:13-24` lists legacy ids. `src-tauri/src/store.rs:148-171` drops *any* matching id with an empty key, then immediately persists the reduced list, despite the comment promising to retain configured providers. There is no comparison of name, URL or model choices.
- **Why it matters:** A user who customized an older seeded provider's endpoint or models but had not entered a key permanently loses that configuration on load. Once the cleanup writes the reduced file, the intent cannot be recovered from ordinary application state.
- **Recommended change:** Delete only entries matching the full old seed fingerprint, or use a one-time versioned migration that preserves ambiguous records. Back up the original config before a destructive migration and add a customized-without-key regression test. This is a data-migration rule; **no pattern applies**.
- **Risk of change:** Preserving ambiguous seeds may leave some unwanted legacy entries; that is safer than silently deleting author-owned settings.

### 7. An existing super-JSON document has no schema-evolution path — SMELL; Factory Method

- **Evidence:** `src-tauri/src/super_json.rs:25-35` constructs new projects from the embedded skeleton, while `src-tauri/src/super_json.rs:218-229` returns any parseable *existing* JSON unchanged. Allowed/required paths are computed from the *current* skeleton (`src-tauri/src/super_json.rs:73-96`); a set into a missing parent or leaf fails (`src-tauri/src/super_json.rs:299-311`).
- **Why it matters:** Adding a field to the skeleton allows tools to request it but cannot populate it in older project documents; missing-required/progress can count it indefinitely and writes to the new path return `Path not found`. Also, parseable but structurally invalid documents pass hydration without validation.
- **Recommended change:** Treat `load_or_init` as a fallible **Factory Method** for a usable *current-version* document: record a schema version, merge new default leaves without overwriting author data, validate known field types, and preserve unknown fields for forward compatibility. Add a fixture from the previous skeleton version.
- **Risk of change:** Naively replacing existing documents with fresh skeletons destroys creative work; a forward-only, non-destructive migration needs backups.

### 8. Provider failure after a tool-request round strands a partial turn — BUG; no pattern applies

- **Evidence:** `src-tauri/src/runtime.rs:370-377` persists an assistant message containing requested tool calls and `src-tauri/src/runtime.rs:460-464` persists tool results before another model round. If that subsequent request fails, `src-tauri/src/runtime.rs:343-353` returns only an error, never the already appended messages. The UI catches the error without loading partial messages (`src/pages/Chat.tsx:463-495`).
- **Why it matters:** The database contains tool side effects and transcript rows that the visible chat does not. Retrying with the UI's incomplete `messages` replays an incompatible history and can request the same tool again; a late failure following `project_create` may leave a bound project the UI does not know about.
- **Recommended change:** Define partial-turn recovery semantics: return persisted messages/project with an error status, or reload the authoritative session transcript on failure and mark the round interrupted. Idempotency for mutating tools must be considered separately. **No creational pattern applies** to partial transaction/result semantics.
- **Risk of change:** A returned partial assistant message with unanswered tool calls may be rejected by a provider; recovery must either complete tool results or deliberately truncate to a valid protocol boundary.

### 9. Existing SQLite files never receive schema migrations — SMELL; no pattern applies

- **Evidence:** `src-tauri/src/db.rs:5-30` defines tables with `CREATE TABLE IF NOT EXISTS`; `src-tauri/src/db.rs:47-59` re-runs that script on every open. There is no recorded database version or `ALTER TABLE` path. `src-tauri/src/db.rs:181-194` assumes the current `payload_json` column exists.
- **Why it matters:** An older database whose `messages` table lacks a later column retains its old shape; current inserts and selects fail instead of upgrading. Re-running an index creation on a nonexistent old column can also prevent startup. This is a concrete upgrade hazard even if the current schema works on a fresh install.
- **Recommended change:** Use `PRAGMA user_version` with ordered, transactional migrations and fixtures representing each released schema. Keep connection creation separate from migration decisions; **no GoF construction pattern applies**.
- **Risk of change:** Never mark a version as migrated until its transaction commits; back up real-user databases before destructive alterations.

### 10. Corrupt message payloads are silently fabricated as new messages — BUG; Factory Method

- **Evidence:** `src-tauri/src/db.rs:209-219` queries role/content/payload but not the stored id. `src-tauri/src/db.rs:222-231` catches every JSON deserialization failure and constructs `MessageEnvelope::new("recovered-{index}", role, content)`; the same fabricated identity is used for rows with a null payload.
- **Why it matters:** Corruption, a new required serialization field, and a genuinely legacy row all look identical. Tool-call IDs, metadata and reasoning disappear without warning; generated IDs depend on query position and can collide with other sessions when replayed through the globally keyed messages table. A restored transcript is therefore not faithfully replayable.
- **Recommended change:** Use a fallible row-to-envelope **Factory Method** that preserves the DB id. Treat a known legacy `NULL` payload explicitly; quarantine malformed non-null payloads or return an actionable error with row identity. Test corrupted payload and old-schema fixtures.
- **Risk of change:** Returning errors instead of partial history can make old sessions temporarily unavailable; provide a transparent repair/export path.

### 11. Chat session identity is persisted only in a component lifetime — BUG; no pattern applies

- **Evidence:** `src/pages/Chat.tsx:39-47` documents the missing resume flow. `src/pages/Chat.tsx:203-216` initializes messages empty and a fresh session id on mount. Native code persists sessions and messages (`src-tauri/src/db.rs:12-25`, `src-tauri/src/lib.rs:174-177`), but the UI has no saved current session id or list/resume interaction.
- **Why it matters:** Reload or navigate away/back: the author sees an empty chat while all prior messages and project binding remain in SQLite under an unreferenced opaque id. A second send creates a different session, making otherwise recoverable work effectively inaccessible through the app.
- **Recommended change:** Persist the active session reference or offer an explicit session list backed by SQLite; on mount load its messages and project binding. The session should be an explicit value passed across boundaries, **not a Singleton** or global mutable field; no GoF pattern applies to the UI restoration requirement.
- **Risk of change:** Old anonymous sessions may need a one-time discovery UI; never attach them automatically to an unrelated project by guesswork.

### 12. localStorage array shape is mistaken for valid Character models — BUG; no pattern applies

- **Evidence:** `src/lib/characters.ts:94-103` checks only `Array.isArray` before casting untrusted JSON to `Character[]`. `src/pages/Characters.tsx:37-63` immediately reads fields and spreads `...(item.tags || [])`. The expected new-record shape is constructed at `src/components/CharacterForm.tsx:62-79` but is never enforced when rehydrating old records.
- **Why it matters:** An older or hand-edited character with `tags: {}` or a null entry causes a rendering exception instead of a partial character list; missing fields can silently produce broken portraits/search results. TypeScript's cast provides no runtime validation at this boundary.
- **Recommended change:** Parse individual rows with a small, version-aware validator/normalizer, applying defaults only to optional fields and quarantining invalid entries. It can be a plain function: **no GoF Factory Method hierarchy applies** to a simple localStorage DTO.
- **Risk of change:** Aggressive normalization can delete unfamiliar future fields; retain the raw record for export or recovery instead of overwriting the source on read.

### 13. Whole-list provider saves have no revision or merge protection — SMELL; no pattern applies

- **Evidence:** `src/components/SettingsDialog.tsx:356-362` saves a complete locally cached `nextProviders` array; `src-tauri/src/lib.rs:100-103` accepts it wholesale; `src-tauri/src/store.rs:180-221` atomically replaces the entire config without comparing the previous version. The startup cleanup can also rewrite that same file (`src-tauri/src/store.rs:156-171`).
- **Why it matters:** Two windows or a stale dialog can each edit different providers, and the last save drops the other's change or API key. The atomic rename guarantees a complete file but not a correct merge or deterministic ownership of the config snapshot.
- **Recommended change:** Mutate a named provider with an expected revision, or reject a stale whole-list save and ask to reload; keep sensitive credentials out of unnecessary cross-window round trips. **No pattern applies**: this is concurrency control, not an object-creation family.
- **Risk of change:** Conflict handling needs intelligible UI; automatically merging two edits to the *same* provider may silently discard one author's intent.

### 14. Browser-preview "backend" is a cross-chat global simulation — OVER-ABSTRACTION; Singleton (avoid)

- **Evidence:** `src/lib/tauri.ts:155-159` keeps one process-global field map, project id, messages array and unnamed-project counter, while `src/lib/tauri.ts:525-555` lets that global project determine future creation and field writes. `src/lib/tauri.ts:561-564` claims no required fields are missing despite `mockMissingRequired()` computing them at `src/lib/tauri.ts:176-178`. `src/lib/tauri.ts:810-811` returns all browser messages regardless of requested session.
- **Why it matters:** New chat resets React state (`src/pages/Chat.tsx:394-410`) but not the mock's old project; the next preview chat can reject project creation as already bound, mutate the previous project's fields, or show a false "ready" status. This full parallel domain implementation has diverged from the real backend and makes browser testing misleading.
- **Recommended change:** **Delete** the duplicate stateful backend simulator in favor of a small explicit preview fixture or a session-keyed, shared pure model with parity tests. If keeping it, scope state by session/project and faithfully report required fields. Do **not** add a Singleton class or an Abstract Factory around two inconsistent implementations.
- **Risk of change:** Marketing/demo preview content may need replacement fixtures; do not accidentally remove the real Tauri transport path.

### 15. HTTP client construction uses request lifetime, not app lifetime — SMELL; Singleton (avoid)

- **Evidence:** `src-tauri/src/adapters.rs:469-484` builds fresh `reqwest::Client`s; `src-tauri/src/adapters.rs:531-559` calls that builder on every model completion, potentially several times per turn; model-list fetches do the same (`src-tauri/src/adapters.rs:502-509`). `src-tauri/src/lib.rs:193-200` does not manage an application-lifetime transport.
- **Why it matters:** Repeated client construction forfeits TLS/connection reuse, introduces avoidable latency and resource churn across rounds, and makes transport configuration less deterministic than it needs to be.
- **Recommended change:** Construct one reusable client per timeout policy at startup or in an injected backend value. Tauri-managed state or a clonable `reqwest::Client` is enough; **do not** introduce a classic mutable Singleton/get-instance abstraction. Keep `LlmBackend` injection for scripted tests.
- **Risk of change:** Model-list requests intentionally have a shorter timeout; keep distinct timeout policies rather than sharing one unsuitable client.

### 16. Project documents can be constructed without a real project owner — BUG; no pattern applies

- **Evidence:** `src-tauri/src/lib.rs:125-137` exposes `execute_tool` with caller-provided project/session ids and `super_json_read` with only a project id. `src-tauri/src/runtime.rs:519-529` delegates without checking the SQLite project row or session binding; `src-tauri/src/super_json.rs:218-229` creates a missing file for any syntactically valid project id. `src-tauri/src/tools.rs:344-353` mutates the supplied project's document without reconciling it to that session.
- **Why it matters:** A stale UI project id or a direct command call can create ghost files and change another session's document, while DB-backed navigation/session ownership disagrees. This is a trust/identity invariant inside the local desktop app, not necessarily an external attacker vulnerability.
- **Recommended change:** At the IPC boundary, distinguish privileged UI edits from agent tool execution; require a known project and, where relevant, a matching session binding before creating/mutating a document. Do not silently create project files during a read of an unknown id. **No pattern applies**; validation and authorization are not object factories.
- **Risk of change:** Existing orphan files produced by earlier commands may need an explicit import/reconcile path rather than becoming invisible after stricter checks.

## Suggested verification matrix (for later implementation sprints, not code changes now)

These checks clarify the intended object and session lifetime. They are proposals, not a claim that tests were run during this read-only review.

**1 — Transcript replay and identity.**
- Persist a user/assistant pair, send a second turn carrying the previous pair, and list messages: the original pair retains its order and identity.
- Enrich the final assistant's metadata without creating a second record or changing its sequence; send equal message ids in separate sessions and assert isolation.

**2 — Concurrent super-JSON writes.**
- Launch two setters of different paths in the same project against the same starting document; both acknowledged values must survive.
- Repeat for independent projects and assert they do not share an unnecessary global write bottleneck.

**3 — Project creation atomicity.**
- Race equal-title project creation from two sessions: expect two distinct ids or one explicit, retriable collision, never two successful references to an unintended common project.
- Race creation in one session and inject a file-write failure: enforce one binding and show a recoverable orphan/rollback policy.

**4 — New chat during an old turn.**
- Start a delayed provider reply, press New chat, and allow the old reply and live channel to finish: the fresh transcript/project remain untouched.
- Confirm that the old turn remains discoverable by its original session id if backend writes already committed.

**5 — Fresh Anthropic configuration.**
- Start from an empty config, add an Anthropic endpoint in Settings and fetch models: assert `/v1/models` and `x-api-key` instead of OpenAI auth.
- Send a chat and assert the saved kind survives restart while OpenAI-compatible custom providers still use their previous wire format.

**6 — Legacy provider cleanup.**
- Fixture: seeded id, edited endpoint/model list, empty key; load providers twice and verify the edited record and its fields survive.
- Fixture: exact historical seed, empty key; verify any cleanup is idempotent and does not delete a matching user-created record by name alone.

**7 — JSON schema evolution.**
- Persist a document from an older skeleton, introduce one required leaf in a test fixture, load it and verify that its old answers are retained and the new field is editable.
- Pass a structurally invalid but parseable document and require explicit validation, not a successful return of an unusable object.

**8 — Partial tool round failure.**
- Script a tool-using first round and a network failure in the second: the UI must display or recover the project binding, assistant request and tool results.
- Resume and verify that no previously successful mutating tool is accidentally applied twice solely because the UI lost partial-turn messages.

**9 — Database migration.**
- Open fixtures at each previously shipped table shape and require both successful migration and a readable message transcript after reopen.
- Force a migration failure mid-transaction; reopening must see the previous usable version, not a misleading new version marker.

**10 — Malformed message payload.**
- Store a malformed non-null `payload_json`; require an actionable error containing the stored row id rather than an invented message with missing tool metadata.
- Store a known legacy null payload and verify deliberate, stable reconstruction using the database id and role/content columns.

**11 — Session restoration.**
- Create a bound project with chat history, navigate away and return, and verify the same session id, project id and messages can be resumed.
- Reload the application and verify an intentional new session is distinguishable from a restored old one.

**12 — localStorage character hydration.**
- Load an array containing null, a row with object-shaped tags, and an older valid row lacking optional fields; the screen must still render valid rows.
- Confirm no hydration/normalization read overwrites raw unsupported records before the user has a way to recover them.

**13 — Concurrent provider edits.**
- Have two editors open with identical initial provider snapshots, modify separate providers, save in sequence and assert no silent loss of either change.
- Save conflicting edits to the same provider and require an explicit conflict instead of a silent last-writer-wins API-key replacement.

**14 — Browser preview parity and isolation.**
- Create project A, start new chat, create project B, and ensure both chats have independent project ids, messages and per-project required-path progress.
- Set one required field and query `super_json_missing`: it must agree with the preview document rather than always claiming zero missing.

**15 — Reusable HTTP clients.**
- Instrument backend rounds with the same connection-capable test server; construction should happen once per timeout policy rather than once per round.
- Preserve the model-list short timeout while chat continues to allow the longer configured completion timeout.

**16 — IPC ownership checks.**
- Read an unknown but well-formed project id and confirm it does not create a document or a phantom database project.
- Attempt a tool write using a real project id paired with a different session id and confirm it is rejected without modifying either document.

## What NOT to change

- **Do not introduce an Abstract Factory just because OpenAI and Anthropic exist.** One protocol-dependent request/response path is a branch, not multiple co-varying categories; `src-tauri/src/adapters.rs:86-99` already centralizes the choice. Likewise, desktop IPC versus browser preview is not a coherent family warranting more classes.
- **Do not add a Builder for `MessageEnvelope`, provider config, or Tauri startup.** Plain struct literals and `serde` defaults remain clearer at `src-tauri/src/models.rs:83-100` and `src-tauri/src/lib.rs:193-200`; staged construction is not the issue. Validate boundary values and define commits instead.
- **Do not add Prototype machinery for project skeletons or React copies.** `src-tauri/src/super_json.rs:25-35` already parses an explicit template; the missing behavior is *migration of existing documents*, not virtual cloning. `src/components/SettingsDialog.tsx:246-250` uses ordinary immutable updates appropriately.
- **Keep** the injected `LlmBackend` test seam (`src-tauri/src/adapters.rs:75-99` and `src-tauri/src/adapters.rs:567-571`); it tests real agent behavior without live providers. Refactor its HTTP client's lifetime, not the trait into a product-family hierarchy.
- **Keep** checked project paths and atomic file replacement (`src-tauri/src/tools.rs:249-261`, `src-tauri/src/super_json.rs:247-272`, `src-tauri/src/store.rs:191-221`). They solve traversal and torn writes; add concurrency/version protection *alongside* them, rather than replacing them with a process-global Singleton.
