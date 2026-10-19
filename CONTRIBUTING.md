# Contributing to Merlink Harness

Thank you for helping improve this early-stage local-first narrative harness. Keep changes small, reviewable, and honest about what the app currently does. Follow the [Code of Conduct](CODE_OF_CONDUCT.md).

## Ground rules

- Preserve local-first behavior; do not add telemetry or leak project content or provider keys.
- Never commit secrets, API keys, user data, personal paths, or real `~/.merlink` contents. Redact logs and example reports.
- Do not add an npm or Rust dependency without agreement from the maintainers.
- Keep refactor PRs behavior-preserving unless their scoped issue explicitly calls for a bug fix; do not bundle new features into refactors.
- Name a concrete problem before proposing a design pattern. Prefer a small function or native language feature to GoF ceremony. Add an abstraction only when there is a real second implementation or a concrete consequence of not adding it.

## Setup and verification

Install Node.js 20+, pnpm, stable Rust, and your platform's [Tauri 2 prerequisites](https://v2.tauri.app/start/prerequisites/). From the repository root:

```sh
pnpm install
pnpm dev          # browser preview, mock backend
pnpm tauri dev    # desktop app, real IPC and storage
```

Before opening a PR, run both checks:

```sh
cargo test --manifest-path src-tauri/Cargo.toml
pnpm build
```

Three Rust tests require online access and are ignored by default. A browser-preview check cannot replace the Rust IPC contract tests or a desktop check for behavior affecting Tauri commands.

## IPC contract

Tauri lower-camel-cases Rust command parameter names when it deserializes JavaScript invokes. Send `sessionId`, `projectId`, and `providerId` from JS, **not** `session_id`, `project_id`, or `provider_id`. For every added or changed Tauri command, add or update an invocation test in `src-tauri/src/ipc_contract_tests.rs` that exercises the actual argument names. Register commands through `merlink_handlers!` only so the app and contract tests share the same command list. Keep the agent loop's maximum at **five tool rounds**.

## Storage and invariants

`MERLINK_HOME` overrides the default `~/.merlink` data root; use a temporary, isolated value for tests and manual experiments. Never test against real `~/.merlink` data. Provider configuration is a JSON file (`config.json`), runtime session/project/message data is SQLite (`merlink.db`), and each project's super JSON lives at `projects/<id>/super.json`. Do not silently change those formats or locations. The super JSON has exactly **25 writable leaf paths**, and the required/optional progress calculation uses a fixed **70/30** weighting; preserve both invariants.

## Branches, commits, and PRs

Create a topic branch from current `main`; use conventional commit subjects such as `fix: guard chat session updates` or `docs: clarify storage`. Keep one concern per PR. Explain the user-visible behavior, storage or IPC impact, verification commands and results, and any deferred work. For a refactor sprint, follow its [sprint spec and index](docs/refactor/sprints/README.md); never merge your own sprint branch to `main` as part of a scoped contribution.

## Bugs and security

Open a GitHub issue with reproduction steps, expected versus actual results, platform, and redacted logs for ordinary bugs. For vulnerabilities or leaked keys, do **not** open a public issue; follow [SECURITY.md](SECURITY.md) to report privately.

## License of contributions

This repository is [MIT licensed](LICENSE). By contributing, you agree that your contributions are licensed under the same MIT terms (inbound = outbound).
