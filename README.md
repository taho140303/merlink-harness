# Merlink Harness

Merlink Harness is an early-stage, local-first narrative harness for developing a story with an agent. A model can call tools to create a project and update its structured "super JSON"; messages are backed by SQLite. The desktop application uses Tauri 2, React 19, TypeScript, and Rust.

## Status

Merlink Harness is early/alpha software, not a finished writing suite. Chat, provider settings, super JSON editing through agent tools, characters, and a browser preview are available. Projects and Story Config are placeholders, and there is no session-list or resume UI yet. The browser preview uses a mock backend; use the Tauri desktop app to exercise the real Rust IPC and persistence paths.

## How it works

A chat turn sends the conversation to the configured model, runs any requested tools, and sends the tool results back to the model (model → tools → model), with a maximum of five tool rounds. The project "super JSON" has 81 writable paths (13 project leaves + 34 tables × count/rows), split into 18 required and 29 optional units. A chat session can be bound to a project; the project document is shared at the project level rather than stored separately for each session. SQLite stores sessions, projects, and messages. Provider configuration and each project's super JSON are files under `~/.merlink`.

An API provider and key are needed to use the real model-backed desktop chat. Browser preview is useful for viewing the frontend, but its mock responses are not a substitute for testing desktop behavior.

## Requirements

- Node.js 20 or later and pnpm.
- A stable Rust toolchain.
- The [Tauri 2 system prerequisites](https://v2.tauri.app/start/prerequisites/) for your platform. Linux needs the distribution's native WebKitGTK and build packages; macOS needs Xcode Command Line Tools; Windows needs the supported Microsoft C++ tools and WebView2. Follow the official prerequisites for current package names and versions.
- Desktop storage resolves from `MERLINK_HOME`, then `HOME`, then on Windows `USERPROFILE` (or `HOMEDRIVE` + `HOMEPATH`), with `.merlink` under that root. Native platform data-directory discovery is not implemented; `MERLINK_HOME` always wins.

## Run

From the repository root:

```sh
pnpm install
pnpm dev          # browser preview (mock backend)
pnpm tauri dev    # desktop app (real Tauri/Rust backend)
```

Use the local address printed by Vite for browser preview. Configure your model provider and API key in Settings before using model-backed chat in the desktop app.

## Test and build

```sh
cargo test --manifest-path src-tauri/Cargo.toml
pnpm build
```

The Rust baseline is 127 passed and 3 ignored; the ignored tests require online provider access. Tests use an isolated `MERLINK_HOME` and must never write to a contributor's real `~/.merlink`. Set `MERLINK_HOME` to a disposable directory if you run a manual storage test; do not point it at real user data. `pnpm build` type-checks and builds the frontend; it does not exercise the desktop IPC boundary. See [CONTRIBUTING.md](CONTRIBUTING.md) for IPC and storage rules.

## Data and privacy

There is no telemetry: app data stays local except for model requests you explicitly make to a configured API provider. By default, provider settings (including API keys) are stored in `~/.merlink/config.json` (file mode `0600` on Unix), the SQLite database is `~/.merlink/merlink.db`, and each project document is `~/.merlink/projects/<id>/super.json`. `MERLINK_HOME` overrides the whole data root. Keep config files and test data containing keys out of commits and bug reports.

## Repository map

- `src/` — React/TypeScript frontend and browser preview bridge.
- `src-tauri/src/` — Rust backend, Tauri commands, tool loop, and storage.
- `docs/` — design notes, plans, schemas, and the refactor programme.
- `.agents/skills/` — project-scoped agent skills (when installed locally).

## Refactor programme

Read the [refactor programme](docs/refactor/README.md) and its [phase-0 synthesis](docs/refactor/synthesis/phase-0.md) for the review record and known limitations. Phase 1 (S0–S8) is complete; the [sprint index](docs/refactor/sprints/README.md) records the scope and status of every unit. Product gaps such as session resume and a real project read-model are listed in the synthesis, not implemented here.

## Contributing and license

Contributions are welcome; start with [CONTRIBUTING.md](CONTRIBUTING.md). Please report vulnerabilities using [SECURITY.md](SECURITY.md) and follow the [Code of Conduct](CODE_OF_CONDUCT.md). Merlink Harness is licensed under the [MIT License](LICENSE).
