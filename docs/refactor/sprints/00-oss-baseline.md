# S0 — Open-source baseline

- **Status:** done
- **Branch:** `refactor/s0-oss-baseline`
- **Units:** programme deliverable (no defect ID)
- **Depends on:** nothing
- **Code changes:** **none**

## Goal

Make the public repository understandable and legally usable as an open-source
project: an honest README, a contribution guide that encodes the IPC/test rules,
and an MIT license. The current root `README.md` is the unmodified Tauri template
and there is no `LICENSE` or `CONTRIBUTING.md`.

## Why now

The repository is public (`https://github.com/taho140303/merlink-harness`) but
its front page says "Tauri + React + Typescript — This template should help get
you started". That is false for this product and gives a contributor no way to
run, test, or safely change it. The refactor programme is about to touch the IPC
and persistence boundaries; the contribution rules must exist before those
changes land.

## Scope

Create or replace exactly these files:

1. **`README.md`** — replace the template. Required sections:
   - Product name and one-paragraph description: local-first narrative harness;
     agent tool-calling over a project "super JSON"; SQLite-backed; Tauri 2 +
     React 19 + TypeScript + Rust.
   - **Status** — honest: early/alpha; Chat, provider settings, super JSON,
     characters, and browser preview work; Projects and Story Config are
     placeholders; there is no session-list/resume UI yet.
   - **How it works** — the turn loop (model → tools → model, max 5 rounds),
     the 25-leaf super JSON, the project/session binding, SQLite for messages and
     `~/.merlink` for config + project JSON.
   - **Requirements** — Node 20+, pnpm, Rust stable, Tauri 2 system prerequisites
     (link to the official Tauri prerequisites page), platform notes.
   - **Run** — `pnpm install`, `pnpm dev` (browser preview), `pnpm tauri dev`
     (desktop).
   - **Test / build** — `cargo test --manifest-path src-tauri/Cargo.toml`,
     `pnpm build`; note the 3 ignored online tests and the `MERLINK_HOME` rule.
   - **Data and privacy** — everything stays local; API keys live in
     `~/.merlink/config.json` (mode 0600 on Unix); no telemetry; `MERLINK_HOME`
     overrides the data root.
   - **Repository map** — `src/` (React), `src-tauri/src/` (Rust), `docs/`
     (design, plans, refactor), `.agents/skills/` (project-scoped skills).
   - **Refactor programme** — link to `docs/refactor/README.md` and the synthesis.
   - **License** — MIT, link to `LICENSE`.
   - Do not claim features that do not exist. Do not include a screenshot unless
     one already exists in `ref-ui/` and is clearly a mock-up; if used, label it.

2. **`CONTRIBUTING.md`** — required sections:
   - Ground rules: local-first, no secrets, no new dependency without agreement,
     no behaviour change in refactor PRs.
   - Setup and the two verification commands.
   - **IPC contract rule** (critical): Tauri lower-camel-cases Rust parameter
     names; JS must send `sessionId`/`projectId`/`providerId`, never
     `session_id`. Every new/changed command needs a test in
     `src-tauri/src/ipc_contract_tests.rs` and registration through
     `merlink_handlers!` only.
   - **Storage rules**: `MERLINK_HOME` for tests; never touch a real `~/.merlink`;
     config is a JSON file, runtime data is SQLite; super JSON has exactly 25
     writable paths and the 70/30 progress formula is fixed.
   - **Design-pattern rule**: name the problem first; small functions and native
     language features beat GoF ceremony; a new abstraction needs a real second
     implementation or a concrete consequence.
   - Branch/commit/PR workflow; conventional commits; one concern per PR.
   - Reporting bugs and security issues; link `SECURITY.md`.
   - License of contributions (MIT, inbound = outbound).

3. **`LICENSE`** — MIT, `Copyright (c) 2026 Merlink Harness contributors`.
   Use the standard SPDX text, no placeholders.

4. **`SECURITY.md`** — short: supported versions (0.1.x), report privately via
   GitHub Security Advisories, do not open a public issue for a key leak, scope
   (local desktop app; API keys, path traversal, IPC boundary).

5. **`CODE_OF_CONDUCT.md`** — Contributor Covenant 2.1, contact placeholder
   `maintainers@example.invalid` is **not** acceptable; use the repository
   owner's GitHub profile URL (`https://github.com/taho140303`) as the contact
   path instead of inventing an email.

Optional, only if cheap: `.github/ISSUE_TEMPLATE/bug_report.yml` and
`pull_request_template.md`. Do not add a CI workflow in this sprint; there is no
verified CI matrix yet.

## Explicitly out of scope

- No source, test, manifest, lockfile, or `.gitignore` changes.
- No repository rename, no GitHub settings, no release/tag, no npm publish.
- No screenshots or marketing copy that overstate the current app.
- No code of conduct enforcement tooling.

## Acceptance checks

- [ ] `README.md` describes the actual app and links to `LICENSE`,
      `CONTRIBUTING.md`, `docs/refactor/README.md`, and the synthesis.
- [ ] `CONTRIBUTING.md` states the camelCase IPC rule, `MERLINK_HOME`, the fixed
      25 paths/70-30/5-round invariants, and the no-new-dependency rule.
- [ ] `LICENSE` is the unmodified MIT text with the copyright line above.
- [ ] `SECURITY.md` and `CODE_OF_CONDUCT.md` exist and contain no fake email.
- [ ] `git diff --stat` touches only the files listed above.
- [ ] `pnpm build` and `cargo test` still pass (they must, since no code changed;
      run them anyway to record the baseline).
- [ ] No secret, API key, personal path, or private planning note is copied into
      the new docs.

## Commit and push

```bash
git switch -c refactor/s0-oss-baseline main
# write the files
pnpm build
cargo test --manifest-path src-tauri/Cargo.toml
git add README.md CONTRIBUTING.md LICENSE SECURITY.md CODE_OF_CONDUCT.md \
        docs/refactor/sprints/README.md docs/refactor/sprints/00-oss-baseline.md
git commit -m "docs: add open-source baseline (README, contributing, MIT license)"
git push -u origin refactor/s0-oss-baseline
```

Report: branch, commit SHA, files changed, both command results, and the exact
README status paragraph so the root agent can review the honesty of the claims.
