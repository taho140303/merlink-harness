# Phase 2 — module decomposition plan

> Status: **planned** · 2026-10-19
> Input: user proposal for `commands/ adapters/ runtime/ tools/ db/`, plus measured
> production-line counts (not total file size).
> Rule: this is a **behavior-preserving move**. No feature, no UI change, no new
> dependency, no public API rename.

## 0. Status

| Sprint | Scope | Status | Branch tip |
| --- | --- | --- | --- |
| P2-S1 | `commands/` | done | `a3fc179` |
| P2-S2 | `adapters/` | done | `928bcf4` |
| P2-S3 | `tools/` | done | `cd5a0da` |
| P2-S4 | `super_json/` | done | `e06844a` |
| P2-S5 | `db/` | ready | — |
| P2-S6 | `runtime/` | pending | — |

## 1. Why the files look big

Most of the line count is `#[cfg(test)]`. The production surface is smaller:

| File | Total | Production | Tests | Decision |
| --- | ---: | ---: | ---: | --- |
| `super_json.rs` | 1,602 | **784** | 818 | split |
| `adapters.rs` | 1,066 | **606** | 460 | split |
| `runtime.rs` | 2,057 | **584** | 1,473 | split |
| `tools.rs` | 950 | **424** | 526 | split |
| `db.rs` | 643 | **325** | 318 | split |
| `models.rs` | 308 | 217 | 91 | keep |
| `store.rs` | 341 | 203 | 138 | keep |
| `paths.rs` | 151 | 70 | 81 | keep |
| `lib.rs` | 207 | 113 commands + 36 wiring | 58 | split commands |
| `ipc_contract_tests.rs` | 977 | 0 | 977 | keep for now |

`models.rs`, `store.rs`, and `paths.rs` stay flat: they are single-concern files
under ~220 production lines. Splitting them would add files without reducing the
number of things a reader must hold in their head. `ipc_contract_tests.rs` is
test-only; split it only if it keeps growing.

## 2. Target tree

```text
src-tauri/src/
├── lib.rs                    # builder, merlink_handlers!, re-exports, tiny tests
├── main.rs
├── commands/
│   ├── mod.rs                # pub(crate) re-exports
│   ├── chat.rs               # send_chat, list_messages, context_usage
│   ├── project.rs            # execute_tool (project/tool command boundary)
│   ├── providers.rs          # load_providers, save_providers, fetch_models
│   └── super_json.rs         # read, progress, summary, labels
├── adapters/
│   ├── mod.rs                # LlmBackend, ChatRequest, AdapterReply, fetch_models, re-exports
│   ├── endpoints.rs          # openai/anthropic/models URL normalisation
│   ├── openai.rs             # openai_turns, parse_openai_response, call_openai
│   ├── anthropic.rs          # anthropic_turns/tools, parse_anthropic_response, call_anthropic
│   ├── http.rs               # clients, checked_json
│   └── scripted.rs           # #[cfg(test)] ScriptedBackend
├── runtime/
│   ├── mod.rs                # re-exports
│   ├── agent.rs              # run_agent_turn, send_chat_internal, turn helpers
│   ├── events.rs             # ProgressSink, Noop/Channel/Recording sinks
│   ├── prompt.rs             # SYSTEM_TEMPLATE, assemble_prompt
│   └── session.rs            # execute_tool_internal guard, document/summary/session/context queries
├── tools/
│   ├── mod.rs                # TOOL_NAMES, re-exports
│   ├── labels.rs             # PATH_LABELS, field_labels, status_message, tool_path
│   ├── specs.rs              # tool_specs
│   ├── project.rs            # project_or_error, slugify, unique_project_id
│   └── execute.rs            # execute_tool dispatch
├── super_json/
│   ├── mod.rs                # re-exports
│   ├── schema.rs             # skeleton, path lists, FieldKind, validate
│   ├── io.rs                 # lock, load/init/save/get/set
│   ├── progress.rs           # missing/filled/progress/value_at
│   └── summary.rs            # Vietnamese prose template and label tables
├── db/
│   ├── mod.rs                # SCHEMA, open/migrate, re-exports
│   ├── projects.rs           # ensure/create/exists/list
│   ├── sessions.rs           # ensure/bind/project_for_session
│   └── messages.rs           # append/list/count
├── models.rs
├── store.rs
├── paths.rs
├── test_support.rs
└── ipc_contract_tests.rs
```

## 3. Invariants

1. **Public call sites do not change.** `crate::adapters::{...}`,
   `crate::runtime::{...}`, `crate::tools::{...}`, `crate::db::{...}`, and
   `crate::super_json::{...}` keep working through `mod.rs` re-exports.
2. **No behavior change.** Same SQL, same commands, same JSON, same error
   strings, same 25 paths, 70/30 formula, 5-round cap.
3. **No dependency.** No `async-trait`, no `thiserror`, no new crate.
4. **Tests move with the code they exercise.** Do not rewrite assertions; only
   adjust module paths and visibility. If a test needs a private helper, use
   `pub(super)`/`pub(crate)` rather than duplicating it.
5. **`include_str!` paths are audited.** Moving a file changes relative depth:
   - `runtime/mod.rs` → `../../prompts/harness_system.md`
   - `super_json/mod.rs` → `../../../docs/schema/runtime/super.json`
   - `lib.rs` stays `../../docs/schema/runtime/super.json`
6. **Tauri macro stays honest.** `merlink_handlers!` and the camelCase IPC
   contract tests must keep passing. If command functions move, the macro and
   tests must resolve them through explicit imports/re-exports; no duplicated
   handler list.
7. **One sprint = one branch = one subagent = one push.** Root merges with
   `--no-ff` after re-running the commands.

## 3a. Line budgets: production vs total

Sprint targets below are **production-line** budgets. Test modules move with the
code they exercise, so a file's `wc -l` can be larger than its production size.
This is deliberate: keeping tests next to the code preserves private access and
avoids duplicating helpers. If total file size becomes the priority, a later
phase can move each `#[cfg(test)] mod tests` into a sibling test module, but that
is a separate refactor.

## 4. Sprint sequence (low risk → high risk)

### P2-S1 — `commands/` extraction

- **Files:** `src-tauri/src/lib.rs`, new `src-tauri/src/commands/{mod,chat,project,providers,super_json}.rs`,
  `src-tauri/src/ipc_contract_tests.rs` (imports only).
- **Work:** move the 11 `#[tauri::command]` functions out of `lib.rs` by domain;
  keep `merlink_handlers!`, `run()`, and the lib tests in `lib.rs`; re-export the
  commands at crate root or import them in the test module so the macro expands.
- **Target:** `lib.rs` ≤ 100 lines; no command file > 180 lines; command names,
  argument casing, and error strings unchanged.
- **Risk:** Tauri's generated `__cmd__*` macros across modules. If a module
  layout cannot satisfy the macro after 2 attempts, stop and escalate (do not
  duplicate the handler list).

### P2-S2 — `adapters/` split

- **Files:** `src-tauri/src/adapters.rs` → `adapters/{mod,endpoints,openai,anthropic,http,scripted}.rs`.
- **Work:** move URL helpers, OpenAI wire code, Anthropic wire code, HTTP client
  construction, and the test-only `ScriptedBackend`; split the tests with them.
- **Target:** `adapters/mod.rs` ≤ 120 lines; each submodule ≤ 250 production lines.
- **Risk:** `#[cfg(test)]` re-export of `ScriptedBackend`; `AdapterReply` test
  constructors.

### P2-S3 — `tools/` split

- **Files:** `src-tauri/src/tools.rs` → `tools/{mod,labels,specs,project,execute}.rs`.
- **Work:** separate UI labels, JSON schemas, project-ID creation, and dispatch.
  Keep `TOOL_NAMES` in `mod.rs` and re-export the public surface.
- **Target:** `tools/mod.rs` ≤ 80 lines; each submodule ≤ 220 lines.
- **Risk:** `PATH_LABELS` private helpers shared by status/tool-path; keep one
  owner in `labels.rs` and expose `pub(super)`.

### P2-S4 — `super_json/` split

- **Files:** `src-tauri/src/super_json.rs` → `super_json/{mod,schema,io,progress,summary}.rs`.
- **Work:** move schema/path validation, file locking + IO, progress/missing
  logic, and the large Vietnamese summary template. Update `include_str!` depth.
- **Target:** `summary.rs` ≤ 420 production lines (the prose template is the
  largest block); other submodules ≤ 320; `mod.rs` only constants + re-exports.
- **Risk:** `summary.rs` is the largest prose block; move it mechanically and
  keep every test. `include_str!` path is the most likely build break.

### P2-S5 — `db/` split

- **Files:** `src-tauri/src/db.rs` → `db/{mod,projects,sessions,messages}.rs`.
- **Work:** schema/migration/open in `mod.rs`; project, session, and message
  operations in their modules; move tests with each.
- **Target:** `db/mod.rs` ≤ 160 lines; each submodule ≤ 180 lines.
- **Risk:** tests use private `SCHEMA`/`now`; expose `pub(super)` only if needed.
  `PRAGMA user_version` must remain in one place.

### P2-S6 — `runtime/` split

- **Files:** `src-tauri/src/runtime.rs` → `runtime/{mod,agent,events,prompt,session}.rs`.
- **Work:** move the agent loop, progress sinks, prompt assembly, and session
  queries; update `include_str!`; split the large test module with the code.
- **Target:** `runtime/agent.rs` ≤ 340 production lines; `mod.rs` ≤ 80; no
  behavior change in event ordering, partial-turn handling, or the cap.
- **Risk:** highest. The 1,473 test lines include scripted-backend, streaming,
  cap, and partial-failure tests. Move them with `git mv`-style mechanical edits;
  do not rewrite. If a test cannot compile without a design change, stop.

## 5. Verification for every sprint

```bash
cargo test --manifest-path src-tauri/Cargo.toml   # 127 passed, 3 ignored
pnpm build
rustfmt --check <changed rust files>              # repo-wide fmt is pre-existing red
git diff --stat                                   # only the sprint allow-list
```

Plus:

- `grep -R "include_str!"` and confirm every path compiles.
- `cargo test ... ipc_contract` and confirm camelCase/negative-key tests still
  pass.
- No change in the output of `git diff --stat` outside the sprint files.

## 6. Non-goals

- No feature work, no UI/CSS change, no new commands, no API rename.
- No dependency changes.
- No `models.rs`/`store.rs`/`paths.rs` split (single concern, small).
- No `ipc_contract_tests.rs` split in this round (test-only; revisit if >1,200
  lines).
- No move of `docs/` files and no change to `include_str!` assets.
- No test assertion rewrite; moves only.

## 7. Exit criteria for Phase 2

- All six sprints merged; `main` green.
- `src-tauri/src/lib.rs` is wiring only; commands live under `commands/`.
- The four large production modules are folders with focused submodules.
- No public call site changed without a re-export.
- `AGENTS.md` records the sprint contract and the model fallback.
