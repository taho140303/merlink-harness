# Phase 1 sprint index

One sprint = one branch = one subagent = one push. Sprints are sequential
because they share `src/pages/Chat.tsx`, `src-tauri/src/runtime.rs`, and the IPC
contract. Every sprint must leave `cargo test` and `pnpm build` green, add no
dependency, and update its own status line here.

| Sprint | Spec | Status | Branch | Units |
| --- | --- | --- | --- | --- |
| S0 | [00-oss-baseline.md](00-oss-baseline.md) | pushed | `refactor/s0-oss-baseline` | OSS docs |
| S1 | [01-session-ownership.md](01-session-ownership.md) | pushed | `refactor/s1-session-ownership` | F-01 frontend, F-02 |
| S2 | [02-message-integrity.md](02-message-integrity.md) | blocked by S1 | `refactor/s2-message-integrity` | F-01 backend, F-20 DB |
| S3 | [03-project-and-json-integrity.md](03-project-and-json-integrity.md) | blocked by S2 | `refactor/s3-project-json-integrity` | F-03, F-04 |
| S4 | [04-agent-loop-truth.md](04-agent-loop-truth.md) | blocked by S3 | `refactor/s4-agent-loop-truth` | F-05, F-06, F-07 |
| S5 | [05-provider-lifecycle.md](05-provider-lifecycle.md) | blocked by S4 | `refactor/s5-provider-lifecycle` | F-08–F-12, F-17 |
| S6 | [06-ipc-and-preview.md](06-ipc-and-preview.md) | blocked by S5 | `refactor/s6-ipc-preview` | F-13–F-16 |
| S7 | [07-ui-correctness.md](07-ui-correctness.md) | blocked by S6 | `refactor/s7-ui-correctness` | F-18, F-19 |

## Rules for every sprint subagent

1. Read `docs/refactor/README.md`, this index, your spec, and
   `docs/refactor/synthesis/phase-0.md` before editing.
2. Work on the branch named in the spec, created from the current `main`.
   Never commit directly to `main`, never force-push, never rewrite history.
3. Implement exactly the sprint scope. If a task is impossible without changing
   behaviour outside the scope, stop and report `BLOCKED` with the evidence.
4. No new Rust or npm dependency. No file moves under `docs/` that break an
   `include_str!`. No secrets in the repository.
5. Run `cargo test --manifest-path src-tauri/Cargo.toml` and `pnpm build` before
   committing. Both must pass.
6. Commit with a conventional message, push the branch to `origin`, and report
   the branch, commit SHA, commands run, results, and any deferred item.
7. Update the sprint's status line in this file to `pushed` (or `blocked`) in the
   same commit. Do not mark a sprint `done`; only the root agent merges and marks
   it done after verification.

## Definition of done (root agent)

- Branch exists on `origin` and contains exactly the sprint scope.
- `cargo test` and `pnpm build` pass on the branch.
- The spec's acceptance checks are satisfied or explicitly waived with a reason.
- The branch is merged into `main` with `--no-ff`, `main` is verified again, and
  this table is updated to `done`.
