# Structural GoF review — Reviewer B: composition and rendering trees

## Scope and approach

- Read `docs/refactor/README.md`, the project GoF skill, all seven structural references (Adapter, Bridge, Composite, Decorator, Facade, Flyweight, Proxy), and the criticism/modernity reference.
- Examined the 25 TypeScript/TSX files under `src/` (approximately 5,504 lines): routing, pages, dialogs, forms, mock data, IPC preview adapter, types, and supporting helpers.
- Examined the 12 Rust files under `src-tauri/src/` (approximately 7,481 lines): command registration, runtime, provider adapters, models, SQLite, persistence, tools, super-json, paths, and tests. Approximately 12,985 source lines overall; concentrated on production paths and sampled their tests.
- Lens: who owns a rendered subtree or session, whether IPC shapes retain identity, whether Rust layers add a boundary rather than merely forwarding, and whether a structural GoF name actually explains the problem. Findings are ranked by likely user impact, not by eagerness to add patterns.
- Read-only source review: no implementation edits or test execution. Reproduction steps below are deductions from the cited control flow, not claims of observed runtime reproduction.

## Ranked summary

| Rank | Class | Pattern | Primary evidence | Finding |
| ---: | --- | --- | --- | --- |
| 1 | **BUG** | no pattern applies | `src/pages/Chat.tsx:394-411` | A completed turn can write its transcript and project into a newly reset chat. |
| 2 | **BUG** | no pattern applies | `src-tauri/src/tools.rs:295-321` | Concurrent project creation can alias two sessions to one project. |
| 3 | **BUG** | no pattern applies | `src-tauri/src/db.rs:19-26` | A cross-session message-ID collision replaces somebody else's persisted message. |
| 4 | **BUG** | Adapter | `src/lib/tauri.ts:155-159` | Browser preview stores all sessions and projects in one global instance. |
| 5 | **BUG** | Facade | `src/pages/Chat.tsx:316-318` | Saving providers in Settings leaves the mounted chat unable to use them. |
| 6 | **BUG** | no pattern applies | `src/pages/Chat.tsx:252-305` | Late project-detail and progress reads can overwrite the current chat's display. |
| 7 | **BUG** | no pattern applies | `src/components/SettingsDialog.tsx:271-311` | A model-list fetch for provider A can update provider B's editor. |
| 8 | **BUG** | no pattern applies | `src/components/SettingsDialog.tsx:638-653` | Nested settings modal declares modality without keeping keyboard focus inside it. |
| 9 | **BUG** | Adapter | `src/pages/Home.tsx:13-35` | Dashboard and character page use fictional project data instead of real projects. |
| 10 | **SMELL** | Proxy | `src/lib/tauri.ts:792-844` | The IPC surrogate promises arbitrary return types and accepts unchecked argument keys. |
| 11 | **SMELL** | Adapter | `src/lib/tauri.ts:162-177` | Preview's writable schema and labels manually duplicate Rust-owned facts. |
| 12 | **BUG** | no pattern applies | `src-tauri/src/runtime.rs:474-494` | Round-cap response metadata can name a different project from the returned turn. |
| 13 | **SMELL** | Facade | `src/pages/Chat.tsx:203-245` | One 980-line component mixes session orchestration with all chat rendering. |
| 14 | **SMELL** | Composite | `src/components/SettingsDialog.tsx:65-112` | Settings' two panes and nested editor are one 1,008-line render/state tree. |
| 15 | **OVER-ABSTRACTION** | Facade | `src-tauri/src/runtime.rs:507-530` | Three no-policy Rust forwarding functions make module ownership less clear. |

## Detailed findings

### 1. Old turn populates a new session — **BUG**, no pattern applies

**Evidence.** `src/pages/Chat.tsx:394-411` clears messages, project, progress and live steps and assigns a new `sessionId`, but does not invalidate a running turn. Its event channel still calls `setLiveSteps` at `src/pages/Chat.tsx:458-461`; the awaited result unconditionally appends messages, assigns `turn.project_id`, and refreshes details at `src/pages/Chat.tsx:467-488`. Even the old request's `finally` changes live/loading state at `src/pages/Chat.tsx:498-506`. The sidebar emits this reset while Chat remains mounted (`src/App.tsx:134-142`).

**Why it matters.** Start a slow model reply and choose New chat. The old backend request correctly belongs to its original session, but its later UI callbacks repopulate the new session with that project's conversation. The new send can now receive an unrelated `projectId` and transcript; this is more than a cosmetic spinner issue. A React component tree is not a Composite that automatically isolates asynchronous work when its state is reset; no GoF structural pattern fixes missing request ownership.

**Recommended change.** Capture a monotonically increasing session epoch when sending; increment it on New chat and guard **each** event, success, error, detail refresh and `finally` against the captured epoch. Decide separately whether to expose completed old work through session history. A temporary disabled New chat button would conceal rather than fix late callbacks when navigation or unmount changes.

**Risk of change.** Guarding only the success path leaves the event stream and `finally` racing; never cancel persistence for the original session solely because its UI is no longer visible. Test reset during event streaming, success and failure.

### 2. Project reservation is non-atomic — **BUG**, no pattern applies

**Evidence.** `src-tauri/src/tools.rs:295-310` separately checks the session's binding, lists existing IDs, and chooses a candidate. `src-tauri/src/db.rs:82-93` uses `INSERT OR IGNORE` rather than detecting an already-taken candidate. Only after filesystem work does `src-tauri/src/tools.rs:311-321` bind the session; `src-tauri/src/db.rs:158-169` upserts that binding regardless of a competing creator.

**Why it matters.** Two simultaneous turns creating similarly named projects can choose the same slug before either inserts. One insert is silently ignored, both initialize/update the same super-json path and each binds its session to that shared ID. Two unrelated authors' session trees can therefore converge on one document. Concurrent project creation **for one session** can also pass the initial check twice. This is a database invariant, not a reason to build a Composite project hierarchy.

**Recommended change.** Reserve a unique project ID with an insertion that reports conflicts and retries the suffix, and atomically assert that a session is still unbound before binding. Define the filesystem/DB rollback or reconciliation policy if document initialization fails. Add parallel tests with same-name sessions and same-session callers.

**Risk of change.** A SQLite transaction cannot atomically encompass filesystem writes; choose a documented ordering and recovery path rather than claiming a transaction alone fixes both stores. Preserve existing project IDs.

### 3. Global message primary key steals another session's row — **BUG**, no pattern applies

**Evidence.** `src-tauri/src/db.rs:19-26` declares `messages.id` as the sole primary key, while `src-tauri/src/db.rs:173-194` inserts each message with `INSERT OR REPLACE` and the current `session_id`. Frontend user IDs come from `msg-${Date.now()}` at `src/pages/Chat.tsx:446-450`. The runtime treats persistence as an idempotent replay of incoming history (`src-tauri/src/runtime.rs:260-275`); the backend's own IDs include a process-wide counter (`src-tauri/src/runtime.rs:133-142`), but that does not protect frontend IDs.

**Why it matters.** Two user messages submitted in the same millisecond in different sessions receive the same ID; persisting the second deletes the first row and inserts it under the second session. On history reload the first message disappears. Replaying an existing ID also replaces its rowid, disturbing `ORDER BY rowid` history order (`src-tauri/src/db.rs:198-210`).

**Recommended change.** Give messages an immutable storage key scoped to session (or globally unique IDs) and upsert only within that scope, preserving insertion order on replay. Migrate the existing SQLite key deliberately, and generate stronger frontend IDs as defense in depth.

**Risk of change.** Existing persisted history already relies on the single-column key and may contain legitimate replayed IDs. A migration must retain ordering and not reassign messages silently.

### 4. Preview adapter collapses all session/project state — **BUG**, Adapter

**Evidence.** `src/lib/tauri.ts:155-159` owns one `browserValues`, `browserProjectId`, and `browserMessages` for the entire module. `mockSendChat` has no `sessionId`/`projectId` input at `src/lib/tauri.ts:582-587` and creates a project only while the global ID is empty (`src/lib/tauri.ts:604-619`); it returns that global ID at `src/lib/tauri.ts:768-769`. Browser `super_json_read`, `super_json_progress`, and `super_json_summary` discard their supplied identities (`src/lib/tauri.ts:815-824`). New chat resets only React state (`src/pages/Chat.tsx:394-405`).

**Why it matters.** In browser preview, create a project, click New chat, and send again: the new session inherits the previous project's content and sees its progress. The mock claims to stand in for the Tauri IPC boundary, but breaks the backend's session-to-project contract. This can mask exactly the UI ownership bugs under review.

**Recommended change.** Implement the smallest session-keyed in-memory preview: session binding map, per-project values, and per-session messages. Pass through the existing IPC argument IDs and reject unknown projects instead of defaulting to one global document. Do not build a generic storage framework.

**Risk of change.** Existing preview screenshots may assume the always-populated singleton; update only intentionally scoped fixtures and exercise two sessions in one page.

### 5. Provider configuration has two unsynchronized owners — **BUG**, Facade

**Evidence.** Chat loads providers once on mount (`src/pages/Chat.tsx:316-318,352-375`). Settings owns another `providers` state and persists edits only into that local state (`src/components/SettingsDialog.tsx:92-105,356-364`); delete/clear similarly bypass Chat (`src/components/SettingsDialog.tsx:371-389`). Shell renders the Settings dialog alongside a persistent routed Chat (`src/App.tsx:274-294`), and Chat disables Send for its stale empty list (`src/pages/Chat.tsx:926-941`).

**Why it matters.** With no preconfigured provider, opening Settings from Chat and saving the first provider still leaves Send disabled until Chat unmounts/reloads. Deleting or changing a provider leaves Chat offering a stale model; the next send then fails backend validation. A narrow shared provider facade/state owner is warranted; React Context is not warranted merely to avoid a two-component callback.

**Recommended change.** Lift the provider list and refresh function to Shell, or notify Chat on successful save/delete and reload when Settings closes. Keep selected model valid after refresh. Route all provider mutations through one small owner, not a parallel Settings/Chat cache.

**Risk of change.** A refresh must not overwrite an in-progress user's deliberate model selection if that model still exists; avoid relying on an untyped global event with no change payload.

### 6. Late reads display a different project's details — **BUG**, no pattern applies

**Evidence.** `src/pages/Chat.tsx:252-285` starts independent project reads and assigns `superJson`/`superJsonSummary` without checking that `id` still matches the selected project. `src/pages/Chat.tsx:295-305` likewise sets progress from a captured `sessionId` without checking its current value. Reads start on project changes at `src/pages/Chat.tsx:343-350` and directly after turn completion at `src/pages/Chat.tsx:475-488`. New chat clears these fields (`src/pages/Chat.tsx:394-405`); the details dialog renders them under **the current** project title (`src/pages/Chat.tsx:942-969`).

**Why it matters.** A slow read for project A can finish after New chat or after project B's faster read, showing A's contents under B's heading. A late progress response can enable the outline button according to another session's required fields (`src/pages/Chat.tsx:520-527`). This race exists even if finding 1's turn-result guard is fixed.

**Recommended change.** Couple each request to a session/project generation and discard responses after identity changes; use one controller or keyed data query for the pair of detail reads. Also guard errors if displayed later.

**Risk of change.** A simple `cancelled` flag in the `useEffect` does not cover the direct refresh button and send-completion calls; preserve manual refresh and ensure same-project request ordering.

### 7. Fetch results cross provider-editor identities — **BUG**, no pattern applies

**Evidence.** `src/components/SettingsDialog.tsx:271-311` awaits `fetch_models`, then merges a closure-captured model list into whichever form `setForm(prev)` now represents. `openEditor` replaces `form`/`editor` (`src/components/SettingsDialog.tsx:225-237`); `closeEditor` resets visible fetch state but cannot cancel the pending request (`src/components/SettingsDialog.tsx:195-205`). The stale request can also write `fetchNotice`, `fetchError` and `fetching` after another editor opens.

**Why it matters.** Fetch A's models, close A, open B, and wait: A's models appear in B and may become B's default model, which can be saved as a valid-looking but wrong configuration. This is an asynchronous identity invariant, not a Decorator use case.

**Recommended change.** Tie each fetch to an editor generation plus requested provider kind/base URL/key; ignore all completion branches if that identity changed, and optionally abort the HTTP request. Merge against the current form's model IDs only once identity is confirmed.

**Risk of change.** Two fetches for the same editor may complete out of order; a boolean `fetching` state alone is insufficient if close/reopen resets it. Avoid storing the API key in logs while tracking request identity.

### 8. Settings' nested modal has no focus boundary — **BUG**, no pattern applies

**Evidence.** Settings focuses its outer close button on open and handles only Escape (`src/components/SettingsDialog.tsx:171-193`). Both outer and provider-editor sections claim `aria-modal="true"` (`src/components/SettingsDialog.tsx:405-411,638-653`), yet `editorRef` is never focused and no Tab loop or focus restoration exists. The existing shared dialog *does* implement topmost selection, focus trapping and restoration (`src/components/Dialog.tsx:23-79`).

**Why it matters.** Keyboard focus remains behind the provider editor and can tab into Settings' underlying controls, even though assistive technology is told the inner dialog is modal. Closing the inner editor can strand focus on an element that no longer exists. The presence of a React wrapper does not make it the GoF Decorator; accessibility is the actual requirement.

**Recommended change.** Reuse the shared dialog's focus policy through one accessible modal primitive that can handle nested topmost dialogs, or simplify the editor to a single modal view. Focus the editor on entry, trap Tab inside the topmost dialog, and restore focus to its opener on exit.

**Risk of change.** Naively nesting the existing `Dialog` portals creates two competing body-scroll and Escape handlers; coordinate a modal stack rather than wrapping twice and hoping event order works.

### 9. Real projects never appear in the rendered workspace — **BUG**, Adapter

**Evidence.** Home's displayed project and character counts, word total and top projects all come from `mockProjects`/`mockCharacters` (`src/pages/Home.tsx:13-35`). Characters uses that same static project list for filters and association (`src/pages/Characters.tsx:7-16,115-127`), and the creation form defaults to its first sample project (`src/components/CharacterForm.tsx:62-79`). In contrast, project creation persists a SQLite record (`src-tauri/src/tools.rs:295-321`; `src-tauri/src/db.rs:82-108`), but registered IPC commands have no list-projects endpoint (`src-tauri/src/lib.rs:15-29`). The Projects route is explicitly a Coming Soon placeholder (`src/pages/Projects.tsx:4-17`).

**Why it matters.** Creating a real project in Chat never adds it to Home or the character story selector; a newly created character can be attached to a sample story instead. If these screens are intentionally demos, their production-sounding labels still claim false workspace totals. The existing `Project` view type (`src/types.ts:159-172`) includes mock-only fields, not the SQLite row's shape.

**Recommended change.** Decide whether these screens are explicitly demo-only or real. For real screens, expose a small persisted project/character read-model adapter and distinguish unavailable word counts from zero; for demos, label/isolate fixtures and never present them as the author's library. Do not force Rust rows into the mock-rich `Project` type by inventing cover art or chapters.

**Risk of change.** The persisted schema currently stores only project ID/title/time, not genre/cover/chapter metrics, so replacing mocks is a product/data migration as well as a component change.

### 10. The IPC proxy is typed only by a caller-chosen cast — **SMELL**, Proxy

**Evidence.** `src/lib/tauri.ts:792-844` exposes `safeInvoke<T>(cmd: string, args?: unknown): Promise<T>` and casts both real Tauri output and preview branches to arbitrary `T`. `src/types.ts:15-23,43-53,88-115` hand-mirrors Rust IPC models (`src-tauri/src/models.rs:19-28,46-57,148-156`); Rust Tauri command parameters use lower-camel argument keys (`src-tauri/src/lib.rs:105-113,179-190`). Existing Rust IPC tests exercise hand-entered command bodies (`src-tauri/src/ipc_contract_tests.rs:224-250,301-316`) but cannot type-check frontend call sites.

**Why it matters.** A misspelled command name or `project_id` in place of `projectId` compiles; browser preview may silently ignore those arguments. The project's IPC tests explicitly document an earlier snake-case regression (`src-tauri/src/ipc_contract_tests.rs:3-13`). This Proxy should control the boundary instead of allowing any claimed result shape.

**Recommended change.** Give the existing wrapper a small typed command/argument/result map (or typed per-command helpers) for actual frontend calls; keep `unknown` at the wire edge and validate critical decoded fields. Retain the valuable Rust integration tests and add a frontend command-shape check rather than replacing them with generated scaffolding by default.

**Risk of change.** Tauri `Channel` values are not JSON-serializable plain argument records; type it deliberately instead of requiring every command to fit a uniform string-to-JSON function.

### 11. Preview copies schema-derived facts by hand — **SMELL**, Adapter

**Evidence.** `src/lib/tauri.ts:162-177` hardcodes eleven required paths and `src/lib/tauri.ts:411-433` hardcodes eleven/fourteen counts and progress semantics. A separate label list spans `src/lib/tauri.ts:456-486`. Rust derives the authoritative leaf lists from the embedded skeleton through `OnceLock` (`src-tauri/src/super_json.rs:73-96`) and supplies production labels through `src-tauri/src/lib.rs:157-167`; a Rust test checks embedded and documented skeleton parity (`src-tauri/src/lib.rs:71-79`).

**Why it matters.** Adding or renaming one writable path in Rust can leave browser preview's completion bar, missing-fields result and translated tool log out of sync without any TS compile error. The preview Adapter needs a demonstrable wire contract, not a second independently maintained schema tree.

**Recommended change.** Prefer a tiny generated preview fixture from the documented skeleton/label definitions, or a CI parity test that compares mock path and label keys with Rust's exported values. Keep only truly preview-specific response prose as hardcoded fixture data.

**Risk of change.** A runtime browser preview cannot call Rust to obtain metadata; do not replace this small offline mock with an elaborate renderer or network dependency merely to avoid generated constants.

### 12. Round-cap message's project metadata is stale — **BUG**, no pattern applies

**Evidence.** Within a tool round, the runtime refreshes `effective_project_id` **before** each call (`src-tauri/src/runtime.rs:414-431`); `project_create` changes the DB binding during that call (`src-tauri/src/tools.rs:311-321`). If the fifth round uses that call, the stop message's metadata uses the pre-call ID (`src-tauri/src/runtime.rs:474-489`), while the emitted event and returned `ChatTurn` reread the post-call ID (`src-tauri/src/runtime.rs:492-504`).

**Why it matters.** The same terminal assistant message records an empty/old `meta.project_id` while its turn reports a new project. Consumers reconciling message metadata to a project get contradictory state in a legitimate round-budget path.

**Recommended change.** Resolve effective project once after processing the final round and reuse that value for stop-message metadata, event and turn; cover project creation in the last allowed round with a scripted backend test.

**Risk of change.** The prompt hash legitimately describes the prompt **before** the last tool call; do not recompute or relabel it as the new project's prompt just to align the project ID.

### 13. Chat renders every concern in one component — **SMELL**, Facade

**Evidence.** `src/pages/Chat.tsx:203-245` declares provider, session, project, detail, progress, modal, history, and live-event states together. Async detail/progress orchestration follows at `src/pages/Chat.tsx:252-312`, session reset/send at `src/pages/Chat.tsx:390-507`, and the same function renders through the detail and tool-log dialogs at `src/pages/Chat.tsx:942-977`. The source is 980 lines, and findings 1, 5 and 6 cross these responsibilities.

**Why it matters.** Adding another toolbar or message affordance makes it harder to see which callback owns a session epoch, which read belongs to a project, and which provider selection is current. The cost is debugging and regression surface, not component length in isolation. A narrow session-controller hook would act as a Facade for rendering without a class hierarchy.

**Recommended change.** After fixing ownership bugs, extract a `useChatSession`-style controller that owns turn identity and server reads; render transcript, composer and detail panels as small focused children. Pass explicit state/actions to the few direct children; do **not** introduce app-wide Context or a generic `ChatNode` Composite to eliminate local props.

**Risk of change.** Moving existing state into a hook without defining epoch/project ownership only relocates the bugs. Preserve callback lifetimes and the event channel's early registration.

### 14. Settings' rendering tree has no clear owner boundary — **SMELL**, Composite

**Evidence.** `src/components/SettingsDialog.tsx:65-112` owns general-preference state, providers, the editor form and fetch state in one component. It loads provider data on modal open (`src/components/SettingsDialog.tsx:118-135`), fetches models (`src/components/SettingsDialog.tsx:263-311`), saves providers (`src/components/SettingsDialog.tsx:325-393`), and renders both settings sections plus a nested provider editor (`src/components/SettingsDialog.tsx:398-411,638-669`) across 1,008 lines.

**Why it matters.** The nested editor is a genuine part of a UI tree, but all parts share one mutable bag, making focus and fetch lifetimes surprisingly coupled. This is a useful **native React composition** problem; a GoF Composite interface with uniform leaf/group methods would add ceremony and make the event/identity bugs harder to see.

**Recommended change.** Split general preferences, provider list and provider editor into ordinary child components with the minimal explicit callbacks; keep one provider-data owner (finding 5) and one modal-focus policy (finding 8). Remove dead state/helpers while splitting rather than creating abstract field wrappers for one form.

**Risk of change.** An eager split can duplicate provider data and accidentally reset an unsaved editor form on each parent rerender. Define persistence and draft ownership before rearranging JSX.

### 15. Delete policy-free forwarding layers in Rust — **OVER-ABSTRACTION**, Facade

**Evidence.** `src-tauri/src/lib.rs:125-137,179-190` delegates three commands into `runtime`, but `src-tauri/src/runtime.rs:507-530` merely passes `fetch_models_internal` to `adapters::fetch_models`, `execute_tool_internal` to `tools::execute_tool`, and `super_json_document` to `super_json::load_or_init`. The model-fetch wrapper also has direct test callers (`src-tauri/src/runtime.rs:1715,1794`), so deletion requires relocating those calls, not ignoring them.

**Why it matters.** These functions add no invariants, aggregation or stable simplified API. They suggest that `runtime` owns network discovery, tools and document reads when the underlying modules already own those responsibilities. This is the negative case for Facade: a layer that only hides where the real code lives.

**Recommended change.** **Delete** the three pass-throughs and call their owning modules from the IPC command; move model-fetch test calls to `adapters` or invoke the command directly as appropriate. Keep runtime functions that truly assemble multi-module behavior, such as `run_agent_turn` and session-specific progress.

**Risk of change.** Preserve command names, parameter casing and error strings while removing delegation; adjust the existing runtime tests' invocation path and then run the IPC contract suite in the implementation phase.

## Suggested acceptance checks for the implementation phase

These are test scenarios, not code changes in this review:

1. While a turn streams tool events, start New chat; no old event, reply, project ID, error or `finally` state enters the new session.
2. Repeat the reset when the original turn rejects; its error is not displayed in the replacement chat.
3. Create two same-named projects concurrently in distinct sessions; both bindings resolve to distinct persisted IDs and documents.
4. Attempt two concurrent `project_create` calls in one session; exactly one binding survives and no orphan project is presented as successful.
5. Persist distinct sessions' same-ID messages; neither session loses a row or borrows the other's content.
6. Replay a session's existing message ID; transcript order remains stable after reload.
7. In browser preview create two sessions; each reports only its own project, message list and completion bar.
8. Add, edit and delete a provider while Chat remains mounted; its selector and Send availability update without a route reload.
9. Resolve A's super-json/progress requests after B's newer requests; B's title, raw data, summary and outline gate remain consistent.
10. Fetch provider A's models, close its editor, open B, then resolve A's fetch; B's form and notice stay untouched.
11. Open the nested provider editor with a keyboard; initial focus, Tab cycle, Escape and focus restoration belong to the topmost modal.
12. Create a real project, revisit Home and Characters, and verify either real data or an unmistakably labelled demo—not mixed identities.
13. Make the last allowed tool round create a project; the stop-message metadata, turn result and finished event agree on its ID.

## What NOT to change

- **Adapter:** Keep the real OpenAI/Anthropic wire transformations in `src-tauri/src/adapters.rs:162-249,273-386`; translating incompatible provider protocols is earned indirection. Fix the preview contract, not the production adapters' existence.
- **Bridge:** The `LlmBackend`/`HttpBackend`/test-only `ScriptedBackend` seam (`src-tauri/src/adapters.rs:72-99,567-605`; `src-tauri/src/runtime.rs:223-233`) permits offline agent-loop tests. Preserve that injection; it does **not** justify parallel provider-class and UI-class hierarchies. Calling it a test seam is more precise than insisting it is a textbook two-hierarchy Bridge.
- **Composite:** React already supplies children and nested elements. The character page's ordinary `<Dialog>` children (`src/pages/Characters.tsx:193-221`) need no `Component`/`Leaf`/`Composite` interface; the two dialogs have distinct content and responsibilities.
- **Decorator:** CSS containers and a modal enclosing children are not automatically behavior decorators. Do not stack higher-order components or modal wrappers to fix session identity or keyboard focus; those need explicit lifetimes and topmost-modal policy (`src/components/SettingsDialog.tsx:638-653`).
- **Facade / Proxy:** Keep the one Tauri command boundary (`src-tauri/src/lib.rs:15-29`) and the browser/desktop switch (`src/lib/tauri.ts:788-844`). Make them honest and typed; do not add another generic service layer. The handler-list macro prevents production/test registration drift (`src-tauri/src/lib.rs:10-29`) and should stay.
- **Flyweight:** `OnceLock`-shared schema path lists (`src-tauri/src/super_json.rs:73-96`) are already proportionate. A few dozen JSX nodes, form objects or per-turn provider requests (`src-tauri/src/runtime.rs:328-342`) do not warrant pooling, mutable global caches or reused objects carrying API keys.
- **When no pattern applies:** Database atomicity, message identity, asynchronous stale-response guards and focus handling are concrete correctness obligations. Label them as such instead of inventing structural participants to reach all seven pattern names.
