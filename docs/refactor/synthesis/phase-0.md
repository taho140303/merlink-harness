# Phase 0 synthesis — six independent GoF reviews

> Status: **complete** · 2026-10-19
> Inputs: 6 reviewer reports (2 per GoF group), 89 raw findings.
> Output: 21 merged units of work, 9 sprints (S0–S8), one deletion list, and a
> pattern verdict that is deliberately conservative.

## 1. What was done

| Group | Reviewer A | Reviewer B |
| --- | --- | --- |
| Creational | [a.md](../reports/creational/a.md) — construction/config lens | [b.md](../reports/creational/b.md) — boundaries/lifecycle lens |
| Structural | [a.md](../reports/structural/a.md) — wiring/boundaries lens | [b.md](../reports/structural/b.md) — composition/rendering lens |
| Behavioral | [a.md](../reports/behavioral/a.md) — agent loop/tools lens | [b.md](../reports/behavioral/b.md) — UI/domain interaction lens |

Each reviewer read the programme rules, the full GoF skill (including the
criticism/modernity reference), all pattern references for their group, and the
real source under `src/` and `src-tauri/src/`. Reviews were independent: no
reviewer read the other report for their group, and no application code was
changed.

The 89 raw findings collapse into **21 merged units**. A unit is merged when at
least two reviewers described the same failure mechanism, even if they named a
different pattern. The report count is not a score: several units are one bug
seen from three lenses.

## 2. Verdict in one screen

1. **The most dangerous defects are data-identity defects, not missing patterns.**
   A frontend turn can write into the wrong chat, a message ID can replace another
   session's row, and project creation can bind two chats to one document.
2. **The agent loop is honest about success but not about partial failure.** Tools
   can mutate a project, the following model call can fail, and the UI is told
   only "error" — the durable writes are invisible.
3. **Provider configuration has one real regression:** a fresh install cannot
   create an Anthropic-wire provider from Settings. The legacy cleanup can also
   delete a provider whose key the author deliberately cleared.
4. **The browser preview is a second, divergent backend.** It ignores session and
   project identity and reports false "all required fields filled" results. It
   should be a small session-scoped test double, not a parallel domain engine.
5. **Almost no new GoF ceremony is justified.** Session ownership, message
   identity, transactions and locks are correctness work. The patterns that earn
   their names are the existing `LlmBackend` seam (Strategy/test seam), the
   OpenAI/Anthropic adapters, `ProgressSink` (Observer), and a small typed IPC
   Adapter. Delete more than you add.

## 3. Merged findings, ranked

Severity is impact if left unfixed: **Critical** = durable cross-session loss;
**High** = wrong user-visible state or recoverable data loss; **Medium** =
misleading behavior or expensive drift; **Low** = maintenance cost.

| ID | Sev. | Unit | Pattern lens | Primary evidence | Reviewers | Sprint |
| --- | --- | --- | --- | --- | --- | --- |
| F-01 | Critical | Message identity can replace another session's row; replay moves rowids | no pattern applies | `src/pages/Chat.tsx:446-450`; `src-tauri/src/db.rs:19-26,173-210` | 6/6 | S1, S2 |
| F-02 | Critical | New Chat does not retire an in-flight turn; old reply/project/events land in the new chat | State / Observer | `src/pages/Chat.tsx:394-411,458-506`; `src/App.tsx:134-142` | 6/6 | S1 |
| F-03 | High | Concurrent super-JSON field writes lose one successful update | no pattern applies | `src-tauri/src/super_json.rs:289-315`; `src-tauri/src/tools.rs:344-354` | 4/6 | S3 |
| F-04 | High | Project creation is check-then-create across SQLite and the filesystem | Factory Method (small) | `src-tauri/src/tools.rs:249-321`; `src-tauri/src/db.rs:80-93,158-169` | 6/6 | S3 |
| F-05 | High | A failed later model round strands durable tool effects and messages | State | `src-tauri/src/runtime.rs:343-353,370-465`; `src/pages/Chat.tsx:489-506` | 5/6 | S4 |
| F-06 | Medium | `ToolFinished { ok: true }` is emitted before the result is persisted | Command | `src-tauri/src/runtime.rs:429-465` | 3/6 | S4 |
| F-07 | Medium | The 5-round cap recycles pre-tool prose and can stamp stale project metadata | State | `src-tauri/src/runtime.rs:474-504` | 3/6 | S4 |
| F-08 | High | Fresh installs cannot create an Anthropic-wire provider; edits can flip kind | Adapter / Factory Method | `src/components/SettingsDialog.tsx:225-258,325-353`; `src-tauri/src/adapters.rs:86-96` | 4/6 | S5 |
| F-09 | High | Legacy provider cleanup deletes edited or deliberately key-cleared providers | no pattern applies | `src-tauri/src/store.rs:13-24,148-173` | 5/6 | S5 |
| F-10 | High | Settings saves do not refresh the already-mounted Chat | Observer / Facade | `src/pages/Chat.tsx:316-318,352-375`; `src/components/SettingsDialog.tsx:356-389`; `src/App.tsx:274-294` | 4/6 | S5 |
| F-11 | Medium | Provider saves and model fetches race across stale snapshots/editors | Command / State | `src/components/SettingsDialog.tsx:271-311,325-393` | 4/6 | S5 |
| F-12 | Medium | Model-list URLs are derived inconsistently; HTTP clients are rebuilt per request | Adapter / Builder | `src-tauri/src/adapters.rs:101-140,469-484` | 2/6 | S5 |
| F-13 | Medium | IPC is a stringly typed cast; command/argument/result drift is unchecked in TS | Proxy / Adapter | `src/lib/tauri.ts:788-844`; `src/pages/Chat.tsx:463-474` | 2/6 | S6 |
| F-14 | Medium | Browser preview ignores session/project identity and fakes missing-field results | Adapter / Singleton (avoid) | `src/lib/tauri.ts:146-177,509-569,582-619,815-824` | 6/6 | S6 |
| F-15 | Medium | Public `execute_tool` IPC can mutate a project unrelated to its session | Proxy (missing guard) | `src-tauri/src/lib.rs:125-133`; `src-tauri/src/tools.rs:221-230` | 3/6 | S6 |
| F-16 | Low | Policy-free Rust wrappers and `ToolInvocation` widen the surface | Facade / Command (unneeded) | `src-tauri/src/runtime.rs:507-530`; `src-tauri/src/models.rs:69-81` | 3/6 | S6 |
| F-17 | Medium | Malformed provider tool calls are dropped or given blank IDs | no pattern applies | `src-tauri/src/adapters.rs:285-314,340-367` | 3/6 | S5 (adapter hardening) |
| F-18 | Medium | Character/browser storage is trusted; story defaults silently to the first project; portrait reads race | no pattern applies | `src/lib/characters.ts:94-103`; `src/components/CharacterForm.tsx:38-69` | 2/6 | S7 |
| F-19 | Medium | Settings nested editor claims modality without a focus boundary; theme logic is duplicated | no pattern applies | `src/components/SettingsDialog.tsx:171-193,638-653`; `src/App.tsx:75-89` | 2/6 | S7 |
| F-20 | Low | `Chat.tsx` (980) and `SettingsDialog.tsx` (1,008) mix session orchestration, rendering, and persistence | Facade / Composite (native React) | `src/pages/Chat.tsx:203-245`; `src/components/SettingsDialog.tsx:65-112` | 2/6 | backlog |
| F-21 | High on Windows / Medium else | Windows `HOME` is unset and unknown-project reads create ghost documents | no pattern applies | `src-tauri/src/paths.rs:7-15`; `src-tauri/src/lib.rs:135-155`; `src-tauri/src/super_json.rs:218-230` | 1/6 | S8 |

### Cross-review agreement

- **6/6 reviewers:** F-01, F-02, F-04, F-14. These are not taste; they are
  independently reproduced control-flow defects.
- **5/6:** F-05, F-09.
- **4/6:** F-03, F-08, F-10, F-11.
- **2–3/6:** F-06, F-07, F-12, F-13, F-15, F-16, F-17, F-18, F-19, F-20. F-21
  (Windows home fallback + ghost read) is structural/a only and was scheduled as a
  post-synthesis hardening sprint. These
  are real but lower leverage; a sprint may land only part of one if doing so
  keeps the diff reviewable.

## 4. The five defect clusters in detail

### 4.1 Turn and session ownership (F-01, F-02)

Chat creates a new `sessionId` and clears state on New Chat
(`src/pages/Chat.tsx:394-411`) but never invalidates the outstanding
`send_chat` call. The channel callback (`:458-461`), success path (`:467-488`),
error path (`:489-497`), and `finally` (`:498-506`) all write unconditionally.
The result is an old answer or project appearing in a new chat, and the new chat
being blocked by an old `loading` flag.

The same class of bug exists on reads: `refreshSuperJson`/`refreshSummary`
(`:252-285`), `refreshProgress` (`:295-305`), and `refreshContextUsage`
(`:307-312`) commit after their awaits without checking that the captured
session/project is still current. A slow old read can repaint the new chat and
even enable the outline button from another project's progress.

**Direction:** one monotonically increasing session/turn epoch, captured by every
async operation; guard channel events, success, error, `finally`, and all reads.
Do not disable New Chat — detach the old turn instead. Backend cancellation is a
separate product decision; already-executed tools must not be pretended away.

### 4.2 Message identity and database integrity (F-01)

`messages.id` is the sole primary key (`src-tauri/src/db.rs:19-26`) and
`append_message` is `INSERT OR REPLACE` (`:173-194`). The frontend generates user
IDs from `Date.now()` (`src/pages/Chat.tsx:446-450`). Two sends in the same
millisecond in different sessions can therefore delete one row and reinsert it
under the other session. `REPLACE` also deletes and re-inserts the row, so
replaying an existing ID changes its `rowid` and defeats the `ORDER BY rowid`
transcript order (`:198-210`). `list_messages` also discards the stored ID and
fabricates `recovered-{index}` identities for corrupt or legacy payloads
(`:203-233`), and there is no `PRAGMA user_version` migration path (`:5-59`).

**Direction:** collision-resistant frontend IDs (UUID) *and* a storage-level
invariant. Keep the existing single-column key to avoid a risky table rebuild;
make the upsert conditional on the same `session_id`, detect a zero-row
conflict, and preserve the rowid for a legitimate replay. Preserve stored IDs on
read; treat a known `NULL` payload as legacy and a present-but-invalid payload as
an error with the message ID. Add a versioned migration hook even if v1 has no
schema change.

### 4.3 Project and super-JSON integrity (F-03, F-04)

Project creation checks the session binding, lists IDs, inserts with
`INSERT OR IGNORE`, initializes the JSON, sets the title, and binds the session
as separate operations (`src-tauri/src/tools.rs:295-321`). Two same-named
creates can pick the same slug, one insert is silently ignored, and both sessions
can bind to the same document. A failure after the DB insert leaves an orphan
project. The JSON itself is a whole-file read/modify/write with an atomic rename
(`src-tauri/src/super_json.rs:289-315`); atomic rename prevents a torn file but
not a lost update when two sessions set different fields concurrently.

**Direction:** reserve the project ID with a real conflict-aware `INSERT` inside
a transaction; retry suffixes on conflict; bind only an unbound session; add a
compensating cleanup for the filesystem/DB split and document that SQLite and
the filesystem are not one transaction. Serialize per-project JSON writes with an
in-process per-project lock (no new dependency); expose the limitation that
another process is not coordinated.

### 4.4 Agent-loop truth (F-05, F-06, F-07)

When a later model call fails, the runtime returns only `Err`
(`src-tauri/src/runtime.rs:343-353`) even though tool results and assistant
messages are already persisted (`:370-377,460-465`). The UI shows an error and
keeps only the user bubble; the next turn sends an incomplete transcript. The
`ToolFinished` event is emitted before the result is appended (`:429-465`), so a
persistence failure can show a successful step that has no durable result. On the
5th round, the loop executes tools, then appends the *pre-tool* `last_content`
plus a stop marker (`:474-493`); the stop message's metadata can also carry the
pre-call project ID while the turn result carries the post-call ID.

**Direction:** on provider failure after any write, reload the authoritative
session transcript in the UI (existing `list_messages`) and mark the turn
interrupted; emit `ToolFinished` only after the result is durably stored; make
the cap path system-authored and truthful (no recycled prose), and re-read the
effective project once after the final tool round for the stop message, event,
and return value. Do not add retry policy in this sprint.

### 4.5 Provider lifecycle (F-08–F-12, F-17)

Settings always enters the "add provider" flow as `custom` and derives `kind`
from the ID (`src/components/SettingsDialog.tsx:225-258,325-353`). On a fresh
install there is no catalog (`src-tauri/src/store.rs:140-145`), so an Anthropic
URL/key is sent through the OpenAI adapter. Editing an existing
`kind: "anthropic"` provider can flip it to OpenAI. The legacy cleanup deletes
any legacy-ID provider with a blank key (`src-tauri/src/store.rs:148-173`), which
includes a provider whose key the author intentionally cleared. Chat loads
providers once (`src/pages/Chat.tsx:316-318`), so the first provider saved in
Settings does not reach the mounted chat. Settings saves the whole array from a
stale closure and model fetches are not tied to the editor that started them.
`models_endpoint` appends `/models` to a URL that may already end
`/chat/completions` (`src-tauri/src/adapters.rs:101-140`), and every request
builds a new `reqwest::Client` (`:469-484`).

**Direction:** persist an explicit protocol choice (OpenAI-compatible vs
Anthropic), preserve an existing provider's kind on edit, and use the same kind
for model discovery and chat. Replace the ID+blank-key cleanup predicate with a
full-record fingerprint (and test edited + key-cleared legacy records). Lift a
provider revision from Settings to Shell and have Chat reload on change. Disable
and serialize provider mutations while one is pending; attach each model fetch to
an editor generation. Normalize the model-list URL from the chat endpoint and
reuse two immutable `reqwest::Client`s per timeout policy. Harden adapter parsing
of malformed tool calls at the same time.

## 5. Pattern verdict

The skill's own test is "does the language already have this?" and "have I seen
this three times?". Applying it to the evidence:

| Pattern | Verdict | Why |
| --- | --- | --- |
| **Adapter** | **Keep / strengthen** | OpenAI and Anthropic wire formats are genuinely incompatible; the production adapters and the typed IPC boundary are earned. |
| **Strategy** | **Keep the existing seam** | `LlmBackend` + `ScriptedBackend` is a real test seam, not a provider-class hierarchy. Do not add per-brand UI strategies. |
| **Observer** | **Use narrowly** | `ProgressSink` is justified. For provider refresh, one callback/revision is enough; no event bus. |
| **Command** | **Only as a verb** | Encapsulate the *new user message + request ID*, not a class per button. The durable unit is a turn, not a `Command` hierarchy. |
| **State** | **Only as a discriminated union** | `idle | running(requestId) | failed` for the UI and `running | interrupted | completed` for a turn. No State classes. |
| **Facade** | **Delete false ones** | Several `runtime` wrappers forward with no policy. Keep `run_agent_turn`; remove pass-throughs. |
| **Factory Method** | **Small helpers only** | A fallible `create_project` / `new_message_id` function is enough. No Abstract Factory. |
| **Builder / Prototype / Composite / Decorator / Bridge / Flyweight / Visitor / Interpreter / Memento / Mediator / Template Method / Chain of Responsibility / Singleton** | **Not justified** | No measured family of products, no clone template, no tree traversal, no undo feature, no multi-stage pipeline. React composition and SQLite records already cover the real needs. |

**Deletion list (do this before adding anything):**

- `ToolInvocation` (`src-tauri/src/models.rs:69-81`) — used only to format one log line.
- `runtime::execute_tool_internal`, `runtime::fetch_models_internal`,
  `runtime::super_json_document` — policy-free forwarding.
- The `execute_tool` IPC command, unless a real caller appears; the agent loop
  calls `tools::execute_tool` directly.
- The separate stateful browser backend simulation, or at minimum its global
  cross-session state and false missing-field path.
- The `editor.mode` pseudo-state in Settings once `kind` is explicit.

## 6. Sprint map

Each sprint is one unit of work, one branch, one subagent, one push. Sprints are
sequential because they share `Chat.tsx`, `runtime.rs`, and the IPC contract.
Every sprint must leave `cargo test` and `pnpm build` green and must not add a
dependency.

| Sprint | Spec | Units | Primary files | Depends on |
| --- | --- | --- | --- | --- |
| S0 | [00-oss-baseline.md](../sprints/00-oss-baseline.md) | programme deliverable | `README.md`, `CONTRIBUTING.md`, `LICENSE`, `.github/` | — |
| S1 | [01-session-ownership.md](../sprints/01-session-ownership.md) | F-01 (frontend), F-02 | `src/pages/Chat.tsx` | S0 |
| S2 | [02-message-integrity.md](../sprints/02-message-integrity.md) | F-01 (backend), schema-version hook | `src-tauri/src/db.rs`, `runtime.rs` | S1 |
| S3 | [03-project-and-json-integrity.md](../sprints/03-project-and-json-integrity.md) | F-03, F-04 | `tools.rs`, `db.rs`, `super_json.rs` | S2 |
| S4 | [04-agent-loop-truth.md](../sprints/04-agent-loop-truth.md) | F-05, F-06, F-07 | `runtime.rs`, `Chat.tsx` | S3 |
| S5 | [05-provider-lifecycle.md](../sprints/05-provider-lifecycle.md) | F-08, F-09, F-10, F-11, F-12, F-17 | `SettingsDialog.tsx`, `store.rs`, `adapters.rs`, `Chat.tsx` | S4 |
| S6 | [06-ipc-and-preview.md](../sprints/06-ipc-and-preview.md) | F-13, F-14, F-15, F-16 | `lib/tauri.ts`, `lib/ipc.ts`, `lib.rs`, `runtime.rs` | S5 |
| S7 | [07-ui-correctness.md](../sprints/07-ui-correctness.md) | F-18, F-19 | `Characters.tsx`, `CharacterForm.tsx`, `characters.ts`, `SettingsDialog.tsx`, `App.tsx` | S6 |

**Deliberately deferred:** F-20 (the 980-line Chat and 1,008-line Settings
components). It is a real maintenance cost, but there is no frontend test runner
in this repository and a large extraction would move the very async lifetimes
that S1/S5/S7 are fixing. The programme fixes the behavior first; a later sprint
can extract hooks once there is a way to observe regressions. This is the same
"do not add abstraction before the second real implementation" rule the skill
applies to GoF patterns.

The sprint index and status table live in
[sprints/README.md](../sprints/README.md). A sprint is "done" only when its
branch is pushed and its acceptance checks are recorded in the spec.

## 7. What this programme will NOT change

- **No behaviour changes beyond the listed defects.** No new features, no
  session list/resume feature, no project browser, no outline generator.
- **No dependency additions** (Rust or npm) unless the author explicitly agrees.
- **No file moves inside `docs/`** that break `include_str!`; the only new docs
  are under `docs/refactor/` plus the root open-source files.
- **No change to the super JSON shape, the 25 paths, the 70/30 progress formula,
  or the 5-round cap.** Fix the truthfulness of the cap, not the cap itself.
- **No table rebuild of `messages`** in this phase. The conditional upsert and
  UUID IDs close the observed hole with a reversible migration; a composite key
  can be a later, separately tested migration.
- **No generic state framework.** React state, a few refs, and discriminated
  unions are enough.

## 8. Residual risks and open questions

1. **Cross-process writes.** A per-project in-process lock does not coordinate a
   second app instance. The desktop app is single-instance today; document the
   assumption. A file lock would need a dependency or platform-specific code.
2. **SQLite + filesystem atomicity.** A project row and its `super.json` cannot
   be committed together. S3 must define the recoverable partial state and test
   at least one injected failure; it must not claim perfect atomicity.
3. **Existing data.** Old rows may already have shifted rowids or partial turns.
   S2/S4 should preserve what exists, not attempt to reconstruct an order that
   cannot be inferred. A repair/export path is future work.
4. **Product gaps found but not scheduled.** Persisted session resume
   (`list_messages` exists, no UI), real projects on Home/Characters, and
   project listing are product decisions, not refactors. They are recorded here
   and intentionally left out of S0–S8.
5. **Windows data root.** S8 added `USERPROFILE` and `HOMEDRIVE` + `HOMEPATH`
   fallbacks while keeping `~/.merlink`. A migration to a platform data directory
   (for example `%APPDATA%`) is a separate product decision; it would need a
   relocation plan for existing data.
6. **Reviewer claims not reproduced at runtime.** These reports are static
   source reviews. Every sprint must turn its target defect into an executable
   test or a recorded manual check; a finding that cannot be reproduced should
   be reported as such rather than "fixed" speculatively.

## 9. Acceptance gate for Phase 1

Phase 1 is complete when all of the following are true:

- [x] Six reports exist at the paths mandated by the programme README, each with
      `file:line` evidence, severity, pattern lens, and a "what NOT to change"
      section.
- [x] This synthesis ranks the merged findings and maps every scheduled unit to
      a sprint.
- [x] All nine sprint specs exist and each has a branch pushed with a green
      `cargo test` and `pnpm build`.
- [x] The root repository has a real README, CONTRIBUTING, and LICENSE.
- [x] `main` contains the merged sprint work, and the final tip is verified with
      `cargo test` and `pnpm build`.

## 10. Execution log

All branches were cut from the preceding `main`, merged with `--no-ff`, and
verified again on `main`. Commits from S3 onward use the repository owner's
`taho140303 <202986530+taho140303@users.noreply.github.com>` identity; S0–S2
were already published under the machine's previous git identity before that
account switch and were **not** rewritten (public history is append-only by
policy).

| Sprint | Branch | Commit | Merge into `main` |
| --- | --- | --- | --- |
| S0 | `refactor/s0-oss-baseline` | `63f1627` | `90e5301` |
| S1 | `refactor/s1-session-ownership` | `80b9acd` | `8f94dba` |
| S2 | `refactor/s2-message-integrity` | `f1f43f7` | `5e7833c` |
| S3 | `refactor/s3-project-json-integrity` | `e128ad5` | `a6a06cb` |
| S4 | `refactor/s4-agent-loop-truth` | `b7a38f1` | `faa1d04` |
| S5 | `refactor/s5-provider-lifecycle` | `eef6ff0` | `9f80f20` |
| S6 | `refactor/s6-ipc-preview` | `0fe928a` | `78612aa` |
| S7 | `refactor/s7-ui-correctness` | `03edce2` | `6a0e949` |
| S8 | `refactor/s8-platform-read-boundary` | `9430077` | `ff40e30` |

Final verification on `main`: `cargo test --manifest-path src-tauri/Cargo.toml`
→ 127 passed, 0 failed, 3 ignored; `pnpm build` → tsc + Vite pass.
