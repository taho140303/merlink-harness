<p align="center">
  <img src="public/logo.svg" alt="Merlink Harness logo" width="180" height="167" />
</p>

<h1 align="center">Merlink Harness</h1>

<p align="center">
  <strong>A local-first narrative harness for building a story with an AI agent.</strong>
</p>

<p align="center">
  <a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/Rust-000000?style=flat-square&logo=rust&logoColor=white" alt="Rust" /></a>
  <a href="https://react.dev/"><img src="https://img.shields.io/badge/React_19-20232A?style=flat-square&logo=react&logoColor=61DAFB" alt="React 19" /></a>
  <a href="https://vite.dev/"><img src="https://img.shields.io/badge/Vite_8-646CFF?style=flat-square&logo=vite&logoColor=white" alt="Vite 8" /></a>
  <a href="https://v2.tauri.app/"><img src="https://img.shields.io/badge/Tauri_v2-24C8D8?style=flat-square&logo=tauri&logoColor=white" alt="Tauri v2" /></a>
  <a href="https://www.microsoft.com/windows"><img src="https://img.shields.io/badge/Windows-0078D6?style=flat-square&logo=windows&logoColor=white" alt="Windows" /></a>
  <a href="https://www.linux.org/"><img src="https://img.shields.io/badge/Linux-FCC624?style=flat-square&logo=linux&logoColor=black" alt="Linux" /></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/License-MIT-8b5cf6?style=flat-square" alt="MIT License" /></a>
</p>

Merlink Harness is a desktop application for planning and drafting long-form fiction with an
AI agent. Every project lives in a single SQLite file on your machine; the model works through a
typed tool-calling surface instead of free-form JSON, and no prose reaches the manuscript until
you approve the commit.

The application runs as a Tauri v2 window: a Rust backend owns storage, migrations, provider
calls, tool dispatch and the draft state machine, while a React Router v8 frontend renders the
workspace.

## Status

Alpha (`0.1.0`). The project is feature-complete enough to plan a book end to end, but the
schema and the tool contract may still change between minor releases. The database baseline was
squashed before the first public release, so treat pre-release databases as disposable.

## What it does

- **Project workspace** — a project groups arcs, chapters and scenes. Drafts move through seven
  planning phases (project core, arcs, chapters, world, scene craft, character depth,
  relationships) before a writing phase drafts one approved chapter at a time.
- **Characters** — identity, appearance, psychology, memories, competences and directed
  relationships, all editable per field with a typed enum vocabulary.
- **Worldbuilding** — a `world_settings` tree plus lore entries (`CULTURE`, `HISTORY`, `FACTION`,
  `ITEM`) that scenes reference by code.
- **Prompt studio** — genres, motifs, event blocks, themes, narrative directives and style
  presets, each bound to the prompt blocks and tools that may use it.
- **Chat** — a ReAct-style loop with read/write tools, context-usage reporting and a model
  selector; inferred values are surfaced as proposals before they are written.
- **Providers** — Anthropic Messages and OpenAI-compatible APIs, plus Ollama and OpenCode for
  local or agent-backed models. Credentials go to the OS keyring, not to the repository.
- **Durable state** — WAL SQLite, integrity checks on start-up, `PRAGMA user_version`
  migrations with an automatic backup before any upgrade, and typed IPC errors.

## Stack

| Layer | Choice |
| --- | --- |
| Desktop shell | Tauri v2 (`2.12`) with a bundled window icon |
| Backend | Rust 2021, `rusqlite` with bundled SQLite, `reqwest` + rustls, `keyring`, `ts-rs` |
| Frontend | React 19, React Router v8 (framework mode), Vite 8, TypeScript 5.9, Tailwind v4 |
| State | Zustand stores plus a typed backend client |
| Storage | SQLite (WAL) at `~/.merlink/db/merlink.sqlite3`, migrations embedded in the binary |
| Tests | `node --test` for frontend contracts, `cargo test` for the Rust domain |

## Requirements

- **Node.js 22.22+** (CI runs Node 24); **pnpm 11**
- **Rust stable** (`rustup` recommended, with `rustfmt` and `clippy`)
- **Tauri v2 system dependencies.** On Debian/Ubuntu:

  ```bash
  sudo apt update
  sudo apt install libwebkit2gtk-4.1-dev build-essential curl wget file \
    libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev patchelf
  ```

  `libssl-dev` and `pkg-config` are required by `openssl-sys`; `patchelf` is only needed to build
  the AppImage bundle. Windows builds need no extra runtime dependency because WebView2 ships
  with Windows 10 (1803+) and Windows 11.

## Run

```bash
pnpm install --frozen-lockfile
pnpm tauri dev          # native window with hot reload
pnpm dev                # browser-only frontend on http://localhost:5173
```

The browser-only mode is useful for UI work; anything that touches the backend needs
`pnpm tauri dev` because the IPC bridge only exists inside the Tauri window.

## Test and build

```bash
pnpm test               # node --test over src/features/**/*.test.mjs
pnpm typecheck          # react-router typegen && tsc
pnpm lint               # eslint + packaging contract checks
pnpm build              # react-router build -> build/client
pnpm test:rust          # cargo test in src-tauri (nice -n 10, two jobs)
pnpm tauri build        # bundle deb, rpm, AppImage and the Windows installers
```

CI runs the same commands in `.github/workflows/ci.yml`, plus `cargo clippy -- -D warnings` and
the full Rust test suite. The design and planning documents that used to live in `docs/` are kept
outside this checkout; the repository only carries what is needed to build, test and release the
app.

## Release

Releases are cut from tags, and the tag must match the version in `package.json`,
`src-tauri/tauri.conf.json` and `src-tauri/Cargo.toml`:

```bash
# bump the three version fields first, then
git tag v0.1.0
git push origin v0.1.0
```

`.github/workflows/release.yml` validates the tag, builds on `ubuntu-22.04` (glibc 2.35, the
oldest supported base) and `windows-latest`, and publishes a draft release with `tauri-action`
v1. The workflow never publishes silently: a maintainer reviews the draft and its assets.

## Repository map

```text
src/
  app/                 routes, layout shell, providers (theme, i18n)
  features/            feature slices: chat, draft, projects, characters, worldbuilding,
                       settings, system-prompt, navigation
  shared/              design-system components, backend client, i18n, assets
src-tauri/
  src/ipc/             #[tauri::command] handlers and DTOs
  src/service/         application services (chat orchestration, workflow, commits)
  src/domain/          invariants, tools, progress engine, artifact contracts
  src/storage/         SQLite repositories, connection pragmas, migration runner
  migrations/          0001_schema.sql and 0002_seed_system_source.sql
  icons/               bundle icons generated from public/logo.svg
public/                logo.svg and favicon
tests/e2e/             end-to-end smoke script
```

## Data and privacy

Merlink Harness is local-first: no account, no telemetry and no hosted storage.

- `~/.merlink/db/merlink.sqlite3` — the project database (WAL mode)
- `~/.merlink/backups/` — automatic pre-migration snapshots
- `~/.merlink/tmp/` — workflow artifacts, cleaned by the app
- `~/.merlink/provider-config.json` — provider endpoints and model choices

API keys are stored in the operating-system keyring, are never written to the repository, and
are redacted from logs. Provider requests leave your machine only when you send a chat turn.

## Database and migrations

The migration history was squashed to two forward-only files ahead of the first public release:

| File | Contents |
| --- | --- |
| `src-tauri/migrations/0001_schema.sql` | complete physical schema: 66 tables, constraints and indexes |
| `src-tauri/migrations/0002_seed_system_source.sql` | system catalogs and prompt seeds |

Both files are `include_str!`-embedded, applied inside one `BEGIN IMMEDIATE` transaction per
file, and tracked with `PRAGMA user_version`. New changes always append `0003_*.sql`,
`0004_*.sql`, and so on; see [CONTRIBUTING.md](CONTRIBUTING.md#database-migrations) before you
touch either file.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for the setup, the commit convention, the IPC contract and
the review checklist. Security issues should be reported privately rather than as a public issue.

## License

[MIT](LICENSE) © 2026 Merlink Harness contributors.
