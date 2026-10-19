# Behavioral GoF review — Reviewer A: agent loop and tool execution

## Scope and review standard

Read the [programme rules](../../README.md), the
[GoF skill](../../../../.agents/skills/design-patterns-gof/SKILL.md), all eleven behavioral
references (Chain of Responsibility, Command, Interpreter, Iterator, Mediator, Memento, Observer,
State, Strategy, Template Method, Visitor), and
[criticism and modernity](../../../../.agents/skills/design-patterns-gof/references/criticism-and-modernity.md).
This is a source review, not a change proposal to implement every pattern.

Targeted frontend reading: [Chat.tsx](../../../../src/pages/Chat.tsx) (~980 lines),
[tauri.ts](../../../../src/lib/tauri.ts) (~844),
[SettingsDialog.tsx](../../../../src/components/SettingsDialog.tsx) (~1,008),
[ToolLogDialog.tsx](../../../../src/components/ToolLogDialog.tsx) (~141),
[App.tsx](../../../../src/App.tsx) (~301), [types.ts](../../../../src/types.ts) (~214), and
[preferences.ts](../../../../src/lib/preferences.ts) (~79). Targeted Rust reading:
[runtime.rs](../../../../src-tauri/src/runtime.rs) (~1,809, including tests),
[tools.rs](../../../../src-tauri/src/tools.rs) (~879, including tests),
[adapters.rs](../../../../src-tauri/src/adapters.rs) (~1,004, including tests),
[db.rs](../../../../src-tauri/src/db.rs) (~327), [models.rs](../../../../src-tauri/src/models.rs)
(~322), [super_json.rs](../../../../src-tauri/src/super_json.rs) (~1,513),
[lib.rs](../../../../src-tauri/src/lib.rs) (~207), [store.rs](../../../../src-tauri/src/store.rs)
(~326), and [paths.rs](../../../../src-tauri/src/paths.rs) (~85). Roughly 10,000 lines across these
files; emphasis was on the relevant execution paths rather than claiming every test or unrelated UI
branch received a line-by-line review.

Severity is relative impact, not an assertion that a remote attacker exists: **critical** means
durable cross-session data damage; **high** means incorrect user-visible state, data loss, or
unbounded work relative to the user's intended turn; **medium** means misleading behavior or
expensive drift. Findings are ranked by likely damage and practical remediation value.

## Ranked summary

| Rank | Class / severity | Pattern | Primary evidence | One-line finding |
| ---: | --- | --- | --- | --- |
| 1 | BUG / high | State | [Chat.tsx:394](../../../../src/pages/Chat.tsx#L394) | New Chat cannot cancel or isolate an in-flight agent turn. |
| 2 | BUG / critical | No pattern applies | [db.rs:19](../../../../src-tauri/src/db.rs#L19) | Millisecond user IDs plus global `REPLACE` can move a message across sessions. |
| 3 | BUG / high | Command | [runtime.rs:260](../../../../src-tauri/src/runtime.rs#L260) | The caller's whole transcript, not persisted session history, is authoritative. |
| 4 | BUG / high | No pattern applies | [super_json.rs:290](../../../../src-tauri/src/super_json.rs#L290) | Concurrent field writes can silently lose another successful field write. |
| 5 | BUG / high | No pattern applies | [tools.rs:249](../../../../src-tauri/src/tools.rs#L249) | Concurrent project creation can bind two chats to the same project. |
| 6 | BUG / high | State | [runtime.rs:370](../../../../src-tauri/src/runtime.rs#L370) | Failed later model rounds leave durable, invisible partial turns. |
| 7 | BUG / medium | State | [runtime.rs:474](../../../../src-tauri/src/runtime.rs#L474) | Reaching the round cap can append stale pre-tool prose as a final answer. |
| 8 | BUG / medium | Command | [runtime.rs:429](../../../../src-tauri/src/runtime.rs#L429) | Tool success streams before the result is durably recorded. |
| 9 | BUG / medium | Observer | [Chat.tsx:316](../../../../src/pages/Chat.tsx#L316) | Editing providers in the open Settings dialog leaves Chat's catalog stale. |
| 10 | BUG / medium | Observer | [Chat.tsx:295](../../../../src/pages/Chat.tsx#L295) | Old asynchronous progress reads can paint another chat's project state. |
| 11 | SMELL / medium | No pattern applies | [adapters.rs:531](../../../../src-tauri/src/adapters.rs#L531) | A transient provider error aborts a partially completed turn without a safe HTTP retry. |
| 12 | SMELL / medium | Command | [tools.rs:66](../../../../src-tauri/src/tools.rs#L66) | Tool dispatch, advertised schemas, and progress labels are parallel registries. |
| 13 | SMELL / medium | No pattern applies | [adapters.rs:285](../../../../src-tauri/src/adapters.rs#L285) | Malformed provider tool requests can be dropped or assigned empty IDs. |
| 14 | OVER-ABSTRACTION / medium | No pattern applies | [tauri.ts:608](../../../../src/lib/tauri.ts#L608) | Browser preview duplicates the agent state machine but violates its contracts. |
| 15 | OVER-ABSTRACTION / low | Command (unneeded wrapper) | [models.rs:69](../../../../src-tauri/src/models.rs#L69) | `ToolInvocation` exists only to format one log line and should be deleted. |

## Detailed findings

### 1. BUG / high — A new chat does not own or cancel its running turn — **State**

- **Evidence:** [Chat.tsx:394–411](../../../../src/pages/Chat.tsx#L394-L411) resets session,
  transcript, project, progress, and live steps but not `loading` or the running request.
  [Chat.tsx:435–474](../../../../src/pages/Chat.tsx#L435-L474) closes over the old session and
  installs an event callback with no session guard.
  [Chat.tsx:475–506](../../../../src/pages/Chat.tsx#L475-L506) subsequently appends its old reply,
  project binding, and errors to whichever chat is now displayed.
  [runtime.rs:48–55](../../../../src-tauri/src/runtime.rs#L48-L55) explicitly ignores channel
  closure, while [adapters.rs:469–474](../../../../src-tauri/src/adapters.rs#L469-L474) allows 120
  seconds per network request and [runtime.rs:320](../../../../src-tauri/src/runtime.rs#L320) allows
  five rounds.
- **Failure:** Press New Chat during a slow turn: the old assistant text and old project can appear
  in the new transcript. Sending remains disabled while the old request runs; navigating away does
  not cancel expensive provider calls or tool writes. Until it finishes, the fresh chat is blocked
  from sending its own first message.
- **Change:** Represent the active turn with a run ID and explicit `idle | running | cancelling |
  finished` transitions. Gate every channel event, reply, refresh, and `finally` against that ID;
  invalidate it immediately on New Chat/unmount. Add a native cancellation token or command for the
  backend; decide explicitly whether already-started tool side effects finish or are aborted.
- **Change risk:** Cancellation is not transaction rollback. Preserve completed tool results, and
  record an interrupted state rather than silently discarding durable work.

### 2. BUG / critical — IDs can overwrite another session's messages — **no pattern applies**

- **Evidence:** [Chat.tsx:446–450](../../../../src/pages/Chat.tsx#L446-L450) creates a user message
  ID from `Date.now()` alone. [db.rs:19–26](../../../../src-tauri/src/db.rs#L19-L26) declares
  `messages.id` the global primary key, while
  [db.rs:173–194](../../../../src-tauri/src/db.rs#L173-L194) performs `INSERT OR REPLACE` using the
  caller-supplied ID and session ID.
- **Failure:** Two chat sends within one millisecond, even in different windows/sessions, can share
  `msg-<timestamp>`. The second `REPLACE` deletes the first session's row and inserts it under the
  second session. Persisted transcript data crosses sessions or disappears; SQLite correctly follows
  this schema, so no race-specific database error warns the user.
- **Change:** Generate collision-resistant user IDs, constrain identity to `(session_id, id)` or use
  a server-generated row identity, and reject conflicting inserts rather than using `REPLACE` as an
  append operation. Migrate existing rows carefully; separately test same-ID sends in different
  sessions.
- **Change risk:** Key migration must preserve existing row order and metadata. This is relational
  identity and conflict semantics, **not** a reason to introduce GoF Command or Memento classes.

### 3. BUG / high — The client can rewrite history while the backend ignores its own transcript — **Command**

- **Evidence:** [Chat.tsx:452–473](../../../../src/pages/Chat.tsx#L452-L473) submits every currently
  rendered message. [runtime.rs:260–275](../../../../src-tauri/src/runtime.rs#L260-L275) upserts all
  of them into SQLite, then [runtime.rs:312–341](../../../../src-tauri/src/runtime.rs#L312-L341)
  builds the next model request from that supplied array rather than `db::list_messages`.
  [lib.rs:174–177](../../../../src-tauri/src/lib.rs#L174-L177) exposes a history-read command, but
  Chat's [mount path](../../../../src/pages/Chat.tsx#L314-L318) loads only context and providers;
  its own [comment](../../../../src/pages/Chat.tsx#L39-L42) says there is no resume flow yet.
- **Failure:** A stale or independently constructed caller can replace earlier transcript rows and
  decide what the model remembers. After a failed partial turn, SQLite and the in-memory context
  disagree; returning to a known session would not automatically reconstruct its context. Replacing
  a historical message also changes its row position because SQLite `REPLACE` deletes and reinserts.
- **Change:** Make the IPC request a small, explicit **Command**: session ID, provider/model
  selection, newly authored user content, and a request ID. The backend validates it, appends the
  new message once, and constructs model history from its own stored session messages. If editing
  historical messages becomes an intentional feature, make that a separate command with explicit
  revision rules.
- **Change risk:** Existing sessions may already contain partial or malformed turns; canonical
  replay requires a recovery policy before switching away from UI-supplied history. Do not make a
  separate class per message.

### 4. BUG / high — Whole-document JSON writes lose concurrent updates — **no pattern applies**

- **Evidence:** [super_json.rs:290–315](../../../../src-tauri/src/super_json.rs#L290-L315) loads the
  full document, changes one field, and saves it.
  [super_json.rs:233–267](../../../../src-tauri/src/super_json.rs#L233-L267) atomically renames a
  newly written whole-file copy but performs no per-project serialization or version check.
  [lib.rs:125–133](../../../../src-tauri/src/lib.rs#L125-L133) also exposes direct tool execution,
  separate from the turn loop.
- **Failure:** Two concurrently executing `super_json_set` commands can both read version A, then
  save A+field X and A+field Y. Both calls report success, but the last rename removes one field.
  Atomic rename prevents torn files, not lost updates; first-use `load_or_init` has the same
  read/create race.
- **Change:** Serialize read–modify–write per project, or use compare-and-swap versions and retry
  conflicts. Reuse the same guard from direct tool calls and agent turns. Add a concurrency test
  writing two different fields repeatedly.
- **Change risk:** Per-project locks must cover creation as well as mutation and must have a clear
  scope across possible process instances. A GoF Memento would snapshot data but would **not** fix
  the race; no behavioral pattern applies.

### 5. BUG / high — Check-then-create is not atomic for projects — **no pattern applies**

- **Evidence:** [tools.rs:249–261](../../../../src-tauri/src/tools.rs#L249-L261) lists existing IDs
  and chooses an available slug; [tools.rs:295–321](../../../../src-tauri/src/tools.rs#L295-L321)
  checks session binding, inserts the project, initializes its JSON, and finally binds the session
  as separate operations. [db.rs:80–93](../../../../src-tauri/src/db.rs#L80-L93) uses `INSERT OR
  IGNORE`, so losing an ID collision is silently accepted.
  [db.rs:158–169](../../../../src-tauri/src/db.rs#L158-L169) unconditionally overwrites a session's
  project binding.
- **Failure:** Two sessions requesting the same title at the same time can select the same slug,
  both return success, and both bind to one project. Two parallel requests from one unbound session
  can instead create an orphan project and let the last binding win. Tool call ordering inside one
  turn does not serialize separate turns/windows.
- **Change:** Reserve an ID using a transaction and conflict-aware insertion; retry suffix selection
  on an actual unique constraint conflict. Conditionally bind only an unbound session, and define
  cleanup/compensation when JSON initialization fails after insertion.
- **Change risk:** The filesystem and SQLite are not one atomic resource, so failure recovery needs
  an explicit policy. This is transaction/locking design, not a justification for introducing a
  Chain of Responsibility.

### 6. BUG / high — Errors strand partial assistant/tool messages — **State**

- **Evidence:** [runtime.rs:370–377](../../../../src-tauri/src/runtime.rs#L370-L377) persists an
  assistant message requesting all tool calls before executing any.
  [runtime.rs:414–465](../../../../src-tauri/src/runtime.rs#L414-L465) stores results one by one. On
  a later provider error, [runtime.rs:343–353](../../../../src-tauri/src/runtime.rs#L343-L353) emits
  `TurnFinished` and returns only `Err`, not partial messages or a failure status.
  [Chat.tsx:489–506](../../../../src/pages/Chat.tsx#L489-L506) displays an error and clears the
  frame without loading persisted partial work.
- **Failure:** Tools may have changed a project while the transcript claims only the user's prompt
  was sent. If persistence fails during a multi-call round, the stored assistant message can
  reference calls with no corresponding tool result. A future server-side replay of that history may
  be rejected by providers requiring paired tool responses.
- **Change:** Model persisted turn states (`running`, `interrupted`, `completed`) and resolve every
  requested call to either a real result or an explicit interrupted/error result. Return a partial
  outcome to the UI or reload canonical history after failure; differentiate provider error from
  storage error. Give retry a new request ID and never blindly rerun successful non-idempotent
  tools.
- **Change risk:** Existing partial histories need validation/migration before canonical replay.
  Memento-style rollback is appropriate only where side effects can really be undone; an arbitrary
  tool write cannot be rolled back merely by changing transcript state.

### 7. BUG / medium — The last tool round can masquerade as a finished answer — **State**

- **Evidence:** [runtime.rs:320–343](../../../../src-tauri/src/runtime.rs#L320-L343) spends a round
  budget on each model call, including calls that ask for tools.
  [runtime.rs:414–472](../../../../src-tauri/src/runtime.rs#L414-L472) executes tools even on the
  fifth round. [runtime.rs:474–493](../../../../src-tauri/src/runtime.rs#L474-L493) then prepends
  `last_content`, generated **before** those tools completed, to a synthetic stop marker; no final
  model observes their results.
- **Failure:** The UI can show an apparently complete answer such as “I saved the values” even when
  a fifth-round write failed. The marker explains the cap, but does not make the preceding prose
  truthful or give the model a chance to interpret final tool outcomes.
- **Change:** Treat `exhausted` as its own terminal state. Either reserve one tool-free final model
  response within a separate budget, or return a clearly system-authored cap message without
  recycling pre-tool prose; include last tool failures in the visible outcome.
- **Change risk:** Reserving an extra model call has cost and latency implications. Preserve the
  hard cap, and test precisely the final-round-with-tools case.

### 8. BUG / medium — “Tool finished successfully” precedes durable recording — **Command**

- **Evidence:** [runtime.rs:429–458](../../../../src-tauri/src/runtime.rs#L429-L458) invokes the
  tool, logs success, and emits `ToolFinished { ok: true }`. Only afterward does
  [runtime.rs:460–465](../../../../src-tauri/src/runtime.rs#L460-L465) append the tool's result to
  SQLite, with `?` able to return early.
- **Failure:** On a full/read-only/busy database, the streamed UI reports a successful step that has
  no durable result, while the overall command fails. For a write tool, the project JSON may have
  changed despite no transcript evidence; a retry can repeat non-idempotent behavior.
- **Change:** Treat each tool invocation as a **Command** with explicit executed/persisted status;
  only emit a durable-success event after the result has been stored. On persistence failure, emit
  an unrecorded-side-effect error and mark the run interrupted. Consider transactional recording for
  database-only tools, not a pretend transaction around arbitrary filesystem/network work.
- **Change risk:** Deferring the event slightly changes streaming timing. Do not delay `ToolStarted`
  or hide the real side effect when persistence cannot be repaired.

### 9. BUG / medium — Provider edits do not notify the mounted chat — **Observer**

- **Evidence:** [Chat.tsx:316–318](../../../../src/pages/Chat.tsx#L316-L318) calls `loadProviders()`
  only once on mount; [Chat.tsx:352–375](../../../../src/pages/Chat.tsx#L352-L375) stores its own
  provider list.
  [SettingsDialog.tsx:356–364](../../../../src/components/SettingsDialog.tsx#L356-L364) saves and
  updates only the dialog's state; deletion and key clearing do likewise at
  [SettingsDialog.tsx:371–391](../../../../src/components/SettingsDialog.tsx#L371-L391).
  [App.tsx:274–283](../../../../src/App.tsx#L274-L283) renders Settings alongside the mounted route,
  not in place of it.
- **Failure:** Starting with no providers, adding one in Settings still leaves Chat's send guard
  disabled. Removing or changing a selected provider leaves Chat showing stale options, and
  `send_chat` can then return `Provider not found` or silently use old selection behavior until
  route remount.
- **Change:** Use a minimal **Observer** notification on successful provider save/delete (or call
  `loadProviders` when Settings closes). Reconcile selected model against refreshed catalog. A React
  context is reasonable only if multiple consumers actually need it; one callback can suffice today.
- **Change risk:** A refresh while a turn runs must not change that turn's captured provider/model.
  Do not introduce a global event bus for this one relationship.

### 10. BUG / medium — Async reads can leak prior project state into the next chat — **Observer**

- **Evidence:** [Chat.tsx:295–312](../../../../src/pages/Chat.tsx#L295-L312) launches
  progress/context reads and unconditionally writes their eventual results.
  [Chat.tsx:394–405](../../../../src/pages/Chat.tsx#L394-L405) resets progress when starting a new
  chat but does not invalidate pending reads;
  [Chat.tsx:487–488](../../../../src/pages/Chat.tsx#L487-L488) can launch another refresh from an
  old send closure. [Chat.tsx:520–527](../../../../src/pages/Chat.tsx#L520-L527) enables the outline
  control solely from the last progress object.
- **Failure:** A slow prior-session `super_json_progress` answer can arrive after New Chat and show
  the prior project's completion percentage, potentially enabling the outline action in an unbound
  chat. The analogous context-usage result can display the wrong model/session usage.
- **Change:** Scope each async observation to the current session/provider/model snapshot and
  discard it when keys change; a monotonically increasing generation ref is enough. Move the refresh
  after a completed turn behind the same run-ID guard as finding 1.
- **Change risk:** An overly aggressive generation counter can drop the newest response when
  refreshes overlap; compare captured keys and request sequence, not only component mount state.

### 11. SMELL / medium — Provider transport errors are not retried in place — **no pattern applies**

- **Evidence:** [adapters.rs:444–467](../../../../src-tauri/src/adapters.rs#L444-L467) converts HTTP
  status and response to an undifferentiated error string;
  [adapters.rs:531–560](../../../../src-tauri/src/adapters.rs#L531-L560) sends one request for
  either provider kind. [runtime.rs:343–353](../../../../src-tauri/src/runtime.rs#L343-L353)
  terminates the run on that error, even after earlier rounds have performed tools.
- **Cost:** Transient rate limiting, 502s, and connection resets turn a partially updated project
  into an apparent failure. Asking the user to resend the entire chat is unsafe for `project_create`
  and future non-idempotent tools.
- **Change:** Preserve structured transport/status information, retry **only the same model HTTP
  request** for a small bounded set of transient failures, honor `Retry-After` where available, and
  show attempts in progress events. Never retry by replaying the whole tool loop. A simple shared
  helper is sufficient; use **Strategy** only if provider-specific retry algorithms actually emerge.
- **Change risk:** Retrying an ambiguous post-response connection failure may duplicate billed
  completions, though it must not duplicate locally executed tools. Bound latency and make
  cancellation effective during backoff.

### 12. SMELL / medium — Tool metadata is spread across parallel dispatch tables — **Command**

- **Evidence:** [tools.rs:8–14](../../../../src-tauri/src/tools.rs#L8-L14) lists names;
  [tools.rs:66–110](../../../../src-tauri/src/tools.rs#L66-L110) independently maps names to
  status/path; [tools.rs:117–219](../../../../src-tauri/src/tools.rs#L117-L219) advertises schemas;
  [tools.rs:264–375](../../../../src-tauri/src/tools.rs#L264-L375) executes by another name match.
  The UI adds its own after-the-fact presentation cases at
  [ToolLogDialog.tsx:66–109](../../../../src/components/ToolLogDialog.tsx#L66-L109).
- **Cost:** Adding or renaming a tool requires coordinated edits to several maps; the model may be
  offered a tool whose dispatcher rejects it, or a valid call may show a generic/misleading live
  status. Existing tests constrain some drift but do not make construction atomic.
- **Change:** If new tools are expected, group each tool's schema builder, validator/handler,
  status, and optional path extractor into a small data-driven **Command** registry, with functions
  rather than a separate class and trait per tool. Generate the advertised list and dispatcher from
  one registry; leave special-case UI prose near the UI unless it becomes actual shared domain data.
- **Change risk:** Centralizing now costs code and can make the variable per-tool JSON schema less
  readable. If these five tools are truly stable, retain the existing straightforward matches and
  strengthen contract tests instead.

### 13. SMELL / medium — Parsing silently alters tool-call intent — **no pattern applies**

- **Evidence:** [adapters.rs:285–314](../../../../src-tauri/src/adapters.rs#L285-L314) uses
  `filter_map` to drop malformed OpenAI tool items and substitutes an empty call ID when missing.
  [adapters.rs:340–367](../../../../src-tauri/src/adapters.rs#L340-L367) accepts Anthropic
  `tool_use` blocks with empty ID/name.
  [runtime.rs:370–376](../../../../src-tauri/src/runtime.rs#L370-L376) persists what remains, then
  [runtime.rs:414–465](../../../../src-tauri/src/runtime.rs#L414-L465) dispatches it and emits
  events keyed by those IDs.
- **Cost:** A provider/proxy response containing valid prose plus one malformed call can be treated
  as a final response without executing the requested action. Repeated missing IDs collide in the
  UI's step keys and can make subsequent provider tool results invalid.
- **Change:** Validate every returned call at the adapter boundary: nonempty unique ID and name,
  supported argument representation, and explicit failure if any item cannot be normalized. Report
  the provider protocol fault instead of pretending the call did not exist.
- **Change risk:** Some nonstandard OpenAI-compatible proxies are permissive today; collect
  representative responses before tightening acceptance. This is validation, not a missing
  Interpreter or Chain of Responsibility.

### 14. OVER-ABSTRACTION / medium — Browser preview is a second, inaccurate agent loop — **no pattern applies**

- **Evidence:** [tauri.ts:608–637](../../../../src/lib/tauri.ts#L608-L637) plans its own tool
  rounds, [tauri.ts:654–713](../../../../src/lib/tauri.ts#L654-L713) replays all live events, and
  [tauri.ts:733–742](../../../../src/lib/tauri.ts#L733-L742) assembles every round's calls into
  **one** assistant message, unlike the Rust one-assistant-per-round behavior at
  [runtime.rs:370–377](../../../../src-tauri/src/runtime.rs#L370-L377). The preview setter
  [tauri.ts:548–555](../../../../src/lib/tauri.ts#L548-L555) accepts unvalidated paths/values; the
  missing-fields tool [tauri.ts:561–564](../../../../src/lib/tauri.ts#L561-L564) reports an empty
  missing list even if required fields remain unset.
- **Cost:** Browser preview can show “all required fields filled” or a plausible tool log under
  conditions the native app would reject. Every new backend lifecycle fix must be mirrored in a
  large TS implementation that is expressly not a real model. This is false confidence, not useful
  reuse.
- **Change:** **Delete** the duplicated preview decision engine if it is not a product requirement;
  prefer explicit canned turn/event fixtures for visual previews and exercise the real Rust loop
  with scripted backends for behavior. If interactive browser previews are essential, share a
  contract schema and test the mocks against native error cases instead of claiming they mirror
  execution.
- **Change risk:** Removing interactive mock replies may reduce offline demo utility. Preserve a
  clearly labeled visual fixture path before deletion; do not introduce Template Method, State
  inheritance, or a cross-language Mediator to preserve fake parity.

### 15. OVER-ABSTRACTION / low — `ToolInvocation` is an unused nominal domain layer — **Command (unneeded wrapper)**

- **Evidence:** [models.rs:69–81](../../../../src-tauri/src/models.rs#L69-L81) defines serializable
  `ToolInvocation` despite explicitly stating that it is not persisted and that tool-result messages
  already hold the record. [runtime.rs:439–451](../../../../src-tauri/src/runtime.rs#L439-L451)
  constructs one solely to print `name`, `ok`, `arguments`, and `result` on the next line; its `id`
  and `error` are not used there.
- **Cost:** A nominal “invocation” model implies a durable command ledger or lifecycle when none
  exists, doubles fields already carried by `ToolCall`/`MessageEnvelope`, and adds
  maintenance/serialization surface without behavior.
- **Change:** **Delete** `ToolInvocation` and log the existing local variables directly. If a real
  retryable/persisted command record is later needed, introduce it with explicit state and
  idempotency semantics at that point, not as this empty wrapper.
- **Change risk:** Check for imports in external consumers before removing a public type; the
  examined runtime uses it only for logging. Removing it is not a replacement for the Command
  request/persistence changes above.

## What NOT to change

- Keep the simple, bounded model → tools → model loop at
  [runtime.rs:320–472](../../../../src-tauri/src/runtime.rs#L320-L472), with separate tool errors
  sent back to the model at [runtime.rs:433–437](../../../../src-tauri/src/runtime.rs#L433-L437).
  Fix terminal and interruption states; do **not** replace a clear loop with a Chain of
  Responsibility or Template Method hierarchy.
- Keep the existing streaming observer seam:
  [runtime.rs:31–55](../../../../src-tauri/src/runtime.rs#L31-L55) has a `ProgressSink` plus real
  Tauri channel, and [runtime.rs:63–75](../../../../src-tauri/src/runtime.rs#L63-L75) has a
  recording test sink. The backend interface is similarly exercised by
  [ScriptedBackend](../../../../src-tauri/src/adapters.rs#L567-L575). These have genuine multiple
  implementations; do not delete them merely because the production path uses one.
- Keep the backend-owned path whitelist at
  [super_json.rs:275–297](../../../../src-tauri/src/super_json.rs#L275-L297) and backend-to-UI
  labels at [tools.rs:58–64](../../../../src-tauri/src/tools.rs#L58-L64). Avoid an Interpreter for
  dotted paths or a Visitor over JSON; explicit validation and typed access are simpler and safer.
- Do **not** add Strategy classes for the existing provider switch, a Mediator for Chat/Settings, or
  Memento snapshots around every tool call. A notification/callback, a bounded retry helper, and
  transactional persistence solve the demonstrated problems with less machinery. State, Observer,
  and Command are useful here only where they clarify real transitions, subscriptions, and durable
  requests.
