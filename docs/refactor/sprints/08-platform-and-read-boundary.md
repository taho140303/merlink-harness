# S8 — Platform data root and read boundary

- **Status:** ready
- **Branch:** `refactor/s8-platform-read-boundary`
- **Units:** F-21 (residual findings from structural/a #4 and #10)
- **Depends on:** S7
- **Code changes:** `src-tauri/src/paths.rs`, `src-tauri/src/db.rs`,
  `src-tauri/src/runtime.rs`, `src-tauri/src/lib.rs`,
  `src-tauri/src/ipc_contract_tests.rs`

## Goal

Close two residual boundary defects that the eight earlier sprints did not own:

1. On Windows, `HOME` is normally unset; the app currently cannot resolve its
   data root, so config, SQLite, and projects all fail before the UI starts.
2. `super_json_read` and `super_json_summary` call `load_or_init`, which creates a
   project directory + `super.json` for any syntactically valid `project_id` —
   even when no SQLite project exists. A typo or stale UI id therefore creates a
   ghost project document and returns plausible empty data.

## Evidence

- `src-tauri/src/paths.rs:7-15` resolves only `MERLINK_HOME` or `HOME` and errors
  otherwise. `src-tauri/src/main.rs:1-2` targets Windows. Windows sets
  `USERPROFILE`, not `HOME`.
- `src-tauri/src/lib.rs:135-138,147-155` call `super_json::load_or_init` for
  read/summary. `src-tauri/src/super_json.rs:218-230` creates the skeleton on a
  missing file. `src-tauri/src/paths.rs:26-53` sanitises the path segment but
  does not prove a project row exists.
- `src-tauri/src/ipc_contract_tests.rs:430-439` currently relies on the ghost
  creation for its "empty project" summary assertion.

## Decisions (locked)

1. **Home resolution order:** `MERLINK_HOME` (test/override) → `HOME` →
   `USERPROFILE` → `HOMEDRIVE` + `HOMEPATH` → error listing the variables tried.
   Keep the default directory name `.merlink`. Do not move existing data.
2. **Reads do not create projects.** Add `db::project_exists(connection,
   project_id) -> Result<bool, String>` and a small policy function in
   `runtime.rs` (for example `super_json_document(project_id)`) that:
   - rejects an empty/blank project id;
   - returns `Err("Project not found: <id>")` when the row is absent;
   - only then calls `super_json::load_or_init`.
   `super_json_summary_for_project` must use the same check. `lib.rs` calls the
   policy functions; do not duplicate the check in the command layer.
3. **Creation still works.** `project_create` continues to use
   `super_json::init_project` directly; this sprint does not change creation.
4. **No new dependency, no schema change, no UI change.** Error strings are new;
   the frontend already logs/propagates `Err`.
5. **Contract tests move with the behavior.** Update the "empty project" summary
   test to create a project first, then read it. Add a test that an unknown
   project id is rejected by both `super_json_read` and `super_json_summary` and
   that no `projects/<ghost>/super.json` is created.

## Tasks

1. `paths.rs`: implement the fallback chain. Keep the existing
   `MERLINK_HOME`-wins test. Add a pure helper test for the fallback order if
   practical; do not mutate process-global env in a parallel test without the
   `TempHome`/`ENV_LOCK` discipline.
2. `db.rs`: add `project_exists` (a `SELECT EXISTS(...)` or `query_row`).
3. `runtime.rs`: add the read-policy function(s), reuse in summary. Keep the S6
   `execute_tool` guard unchanged.
4. `lib.rs`: route `super_json_read`/`super_json_summary` through the policy.
5. `ipc_contract_tests.rs`: update the summary test and add the ghost-rejection
   test. Use the existing `Harness` and `TempHome`; do not touch real storage.
6. Update `docs/refactor/synthesis/phase-0.md`: add F-21 to the table and S8 to
   the sprint map, and record the final execution log. Update this spec and the
   sprint index status. (The root agent may do this after the branch is pushed;
   if you do it, keep the edits limited to status/rows.)

## Explicitly out of scope

- Moving the data root to a platform data directory (a migration/product
  decision); only the fallback is in scope.
- Session resume/listing, project listing, or changing `super_json_progress`.
- Changing `load_or_init` itself; it remains the create-on-use helper for
  `project_create`.
- Any frontend, CSS, provider, IPC-map, or dependency change.

## Acceptance checks

- [ ] `cargo test --manifest-path src-tauri/Cargo.toml` passes (124 + new tests).
- [ ] `pnpm build` passes unchanged.
- [ ] A missing `HOME` with `USERPROFILE` set resolves `<USERPROFILE>/.merlink`;
      `MERLINK_HOME` still wins; a fully empty environment errors with a clear
      message.
- [ ] `super_json_read("missing")` and `super_json_summary("missing")` return
      "Project not found" and create no directory/file.
- [ ] A real project created through `project_create` can still be read and
      summarized.
- [ ] No frontend file changed; no new dependency.

## Commit and push

```bash
git switch -c refactor/s8-platform-read-boundary main
# edit
cargo test --manifest-path src-tauri/Cargo.toml
pnpm build
git -c user.name=taho140303 \
    -c user.email="202986530+taho140303@users.noreply.github.com" \
    commit -am "fix(storage): resolve a Windows home and reject unknown-project reads"
git push -u origin refactor/s8-platform-read-boundary
```

Report: branch, commit SHA, author identity, both command results, the home
fallback order, the ghost-read error text, and whether any contract test needed
its expectation changed.
