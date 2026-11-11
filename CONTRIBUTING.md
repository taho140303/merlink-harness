# Contributing to Merlink Harness

Thanks for taking the time to contribute. This document covers how to set up the project, what
the code contracts are, and what a good pull request looks like.

## Ground rules

- Be direct and technical. Review comments are about the change, not the person.
- Name a concrete problem before proposing a design pattern. Add an abstraction only when there
  is a real second implementation.
- Keep the diff focused. A refactor and a behaviour change travel in separate commits, and
  usually in separate pull requests.
- Every behaviour change ships with a test that would fail without it.
- Never commit local state: `.env`, `app-data/`, `~/.merlink`, provider keys, or generated
  database files.

## Setup and verification

Prerequisites: Node.js 22.22+ (CI uses 24), pnpm, Rust stable with `rustfmt` and `clippy`, and
the Tauri v2 Linux packages listed in the README.

```bash
pnpm install --frozen-lockfile

# what CI runs, in the same order
pnpm test                 # node --test over src/features/**/*.test.mjs
pnpm typecheck            # react-router typegen && tsc
pnpm lint                 # eslint + packaging contract checks
pnpm build                # react-router build -> build/client

cd src-tauri
cargo clippy --all-targets --locked -- -D warnings
cargo test --locked
cargo build --bundles deb # optional: verify the Tauri bundle locally
```

The tree is not rustfmt-normalized yet: run `cargo fmt` on the modules you touch, but do not
reformat unrelated files in the same pull request. Formatting is not a CI gate until that
baseline cleanup happens.

Two rules keep CI green:

1. **Run `pnpm build` before any `cargo` command that compiles the app.** `tauri-build` and
   `generate_context!` read `frontendDist` (`build/client`) at compile time, so a missing
   frontend build looks like a Rust error.
2. **Keep the lockfiles in sync.** `pnpm-lock.yaml` and `src-tauri/Cargo.lock` are committed and
   CI installs with `--frozen-lockfile` / `--locked`. Run `pnpm install` and `cargo update` when
   you change a manifest, and commit the lockfile in the same change.

## IPC contract

The frontend and the Rust backend meet only through `#[tauri::command]` handlers.

- Tauri lower-camel-cases command arguments: send `sessionId`, not `session_id`, and `projectId`,
  not `project_id`.
- Register new commands in the invoke handler next to their siblings; an unregistered command
  fails only at runtime.
- Return typed results. IPC errors carry an `ErrorCode` from `src-tauri/src/error.rs` so the UI
  can branch on the code instead of matching message text.
- Test the invocation shape, not just the service underneath: add or extend a test next to the
  command module, or in `src-tauri/tests/ipc_contract.rs` when the payload crosses a feature
  boundary.

Frontend calls go through the typed client in `src/shared/backend/client.ts`; do not call
`invoke` directly from components.

## Storage and invariants

- The database is a single SQLite file at `~/.merlink/db/merlink.sqlite3`. Open it through
  `DbManager`, never with an ad-hoc connection: the manager applies WAL, `synchronous = FULL`,
  `foreign_keys = ON`, a busy timeout and `temp_store = MEMORY`.
- `PRAGMA user_version` is the only schema version authority. There is no `_schema_migrations`
  table, and start-up verifies the table manifest, indexes and foreign keys.
- Draft content is versioned. A write that does not go through the commit/version path is a bug.
- Enum values in the frontend (`src/shared/i18n/dbEnums.ts`) mirror the database check
  constraints. When you add a value, change both and keep a test for the mapping.

## Database migrations

The forward-only history is two files, and both are embedded in the binary with `include_str!`:

| File | Role |
| --- | --- |
| `src-tauri/migrations/0001_schema.sql` | complete physical schema |
| `src-tauri/migrations/0002_seed_system_source.sql` | system catalogs and prompt seeds |

Rules:

1. **Never edit an applied migration.** `0001` and `0002` are baked into every shipped build; a
   local edit silently diverges from released databases and the start-up schema verifier will
   reject the result.
2. **Append.** New schema or seed changes go to `0003_<topic>.sql`, then `0004`, and so on. Add
   the `include_str!` constant, a `get_migration_sql` arm, and bump `MAX_SUPPORTED_SCHEMA_VERSION`
   in `src-tauri/src/storage/migration.rs` in the same commit.
3. **One transaction per file.** The runner wraps each file in `BEGIN IMMEDIATE ... COMMIT` and
   sets `user_version` inside that transaction. A file must be valid on a fresh database *and* on
   the previous version.
4. **Seed data is system source.** Only catalog and prompt rows belong in a seed file. User rows
   (projects, sessions, characters, drafts) never do.
5. **Preserve user edits on upgrade.** Seeds use `INSERT OR IGNORE` or
   `ON CONFLICT ... DO UPDATE` so re-running never overwrites an author's edited prompt.
6. **Test both paths.** Every migration commit adds a test that boots an empty database and one
   that upgrades the previous version, then asserts `user_version` and the affected rows.

## Branches, commits, and pull requests

- Branch from the current `main` with a topic name, for example `feat/draft-scene-lock` or
  `fix/chat-history-order`.
- Use Conventional Commits: `feat:`, `fix:`, `refactor:`, `perf:`, `docs:`, `test:`, `build:`,
  `ci:`, `chore:`. Add a scope when it helps: `fix(draft): reopen at the committed phase`.
- Keep the subject in the imperative mood and under ~72 characters; explain the *why* in the
  body, not the *what*.
- One logical change per commit, and a commit must build on its own.

A pull request is ready when:

- [ ] `pnpm test`, `pnpm typecheck`, `pnpm lint`, `pnpm build` pass locally
- [ ] `cargo clippy -- -D warnings` and `cargo test` pass in `src-tauri`
- [ ] new behaviour has a test, and the test fails without the change
- [ ] the migration rules above are respected, if the schema or seed changed
- [ ] user-visible copy exists in the i18n dictionary, not only in a component
- [ ] no secrets, local paths, database files or `app-data/` are staged
- [ ] the PR description states the problem, the approach, and how it was verified

## Releases

Release only from a green `main`:

1. Bump the version in `package.json`, `src-tauri/tauri.conf.json` and `src-tauri/Cargo.toml`.
2. Run `pnpm install` so the lockfile records the new version if it is referenced there.
3. Commit, then tag `vX.Y.Z` and push the tag. The release workflow refuses a tag that does not
   match the package version.
4. Review the draft release, its notes and its assets (`.deb`, `.rpm`, `.AppImage`, `.msi`,
   `.exe`), then publish it.

## Bugs and security

- Open an issue with the version, the operating system, the exact steps, and any log line from
  `~/.merlink/logs`. Redact prompts and API keys before pasting.
- Do not open a public issue for a security problem: report it privately to the maintainers and
  give them a reasonable window to ship a fix before disclosure.

## License of contributions

By contributing you agree that your contribution is licensed under the [MIT License](LICENSE),
and that you have the right to submit it.
