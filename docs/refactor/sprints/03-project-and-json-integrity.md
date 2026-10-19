# S3 — Project creation and super-JSON integrity

- **Status:** pushed
- **Branch:** `refactor/s3-project-json-integrity`
- **Units:** F-03, F-04
- **Depends on:** S2
- **Code changes:** `src-tauri/src/db.rs`, `src-tauri/src/tools.rs`,
  `src-tauri/src/super_json.rs`

## Goal

Make project creation reserve one ID and one session binding atomically, and make
per-project super-JSON writes serializable so two concurrent field updates cannot
silently lose one another.

## Evidence

- `src-tauri/src/tools.rs:249-261` lists existing IDs, chooses a free slug, and
  returns it without reserving anything.
- `src-tauri/src/tools.rs:295-321` checks the session binding, inserts with
  `INSERT OR IGNORE`, initializes JSON, sets the name, and binds the session as
  separate operations. A duplicate insert is reported as success; a later failure
  leaves an orphan.
- `src-tauri/src/db.rs:82-93` uses `INSERT OR IGNORE`; `:158-169` overwrites any
  existing session binding.
- `src-tauri/src/super_json.rs:289-315` reads the whole document, changes one
  leaf, and saves the whole document. `:233-267` makes each replacement atomic,
  but there is no lock or version across read/mutate/write.
- `src-tauri/src/tools.rs:344-354` exposes super-JSON writes through tool
  execution; direct IPC (S6) can run them independently.

## Decisions (locked)

1. **Reserve, do not check-then-create.** Add
   `db::create_project(connection, project_id, title) -> Result<bool, String>`:
   plain `INSERT` (no `OR IGNORE`); return `Ok(true)` on insert, `Ok(false)` on a
   `ConstraintViolation` for the ID, and `Err` for any other database error.
   Keep the existing function name/signature only if all callers are updated;
   prefer returning the boolean.
2. **Suffix loop in `tools.rs`.** Remove `unique_project_id`'s read-then-pick
   loop. Compute the base slug once, then try `base`, `base-2`, … with
   `create_project`; stop on `Ok(true)`. Cap the attempts (for example 1,000) and
   return a clear error if exhausted. The slug must still pass
   `paths::project_dir` before insertion.
3. **Conditional bind.** Add `db::bind_session_to_project_if_unbound`:
   an `INSERT ... ON CONFLICT(id) DO UPDATE SET project_id = excluded.project_id
   WHERE sessions.project_id IS NULL OR sessions.project_id = ''`; return whether
   a row was written. `project_create` must fail if the session was already bound
   (the existing pre-check can stay for the error message, but the conditional
   bind is the authority).
4. **One project JSON write.** Add `super_json::init_project(project_id, name:
   Option<&str>)` that writes the skeleton plus the optional
   `require.overview.name` in **one** atomic save, and refuses to overwrite an
   existing file (`create_new`-style check or a per-project lock + existence
   check). Do not call `load_or_init` and then `set` as two separate writes.
5. **Transaction + compensation.** In `project_create`:
   - Open the connection as `mut`; begin a transaction.
   - Reserve the project ID with `create_project`; loop suffixes on `false`.
   - Initialize the JSON with `init_project`. If it fails, drop/rollback the
     transaction and remove only the file/directory this call just created; never
     remove a pre-existing directory.
   - Bind the session with the conditional bind. If it fails, rollback and remove
     the just-created JSON.
   - Commit.
   - A SQLite transaction cannot cover the filesystem; document the possible
     partial states and the cleanup order. Do not claim perfect atomicity.
6. **Per-project super-JSON lock.** Add a process-wide
   `OnceLock<Mutex<HashMap<String, Arc<Mutex<()>>>>>` registry in `super_json.rs`.
   Public `load_or_init`, `set`, and `init_project` acquire the project's lock.
   Internal functions (`load_or_init_unlocked`, `save_unlocked`) assume the lock
   is held; `set` must not deadlock by re-locking. `get`, `missing_required`,
   `filled_counts`, and `progress` may keep using the public locked
   `load_or_init`; reads are already protected by the atomic rename. Document
   that this is an in-process lock and does not coordinate a second app process.
7. **No new dependency.** Use `std::sync` only.

## Tasks

1. `db.rs`: change `create_project` to conflict-aware; add
   `bind_session_to_project_if_unbound`; keep `ensure_session`/`ensure_project`
   behavior for the turn loop. Add tests:
   - two `create_project("same")` calls → second returns `false` and the first
     row is unchanged;
   - conditional bind on a bound session returns `false`/does not overwrite;
   - bind on a missing session inserts it (the IPC `execute_tool` test relies on
     this).
2. `tools.rs`: replace `unique_project_id`; wire the transaction and
   compensation; update `project_create` result. Add tests:
   - two sequential creates with the same name in different sessions produce
     `nua-dem` and `nua-dem-2`;
   - a second create in one session returns the existing "already belongs"
     error;
   - the created `super.json` contains the name in one write;
   - conflict-aware insertion is exercised directly (test `create_project`).
3. `super_json.rs`: add the lock registry, internal unlocked functions, and
   `init_project`. Add a concurrency test using a `TempHome` and multiple
   `std::thread`s, each calling `set` for a different required path; join and
   assert every value is present in a fresh `load_or_init`. Use a barrier to
   maximize overlap. Do not create a `TempHome` inside each thread (it holds a
   global env lock); create one in the parent and move the project ID.
4. Keep the 25 paths, the 70/30 formula, and the 5-round cap unchanged.

## Explicitly out of scope

- Message identity/DB migrations (S2).
- Agent-loop partial turns/events/round cap (S4).
- Provider config (S5).
- Moving super JSON into SQLite, cross-process file locks, or a new dependency.
- Adding a `Builder`/`Factory` hierarchy; the fallible functions above are the
  whole abstraction.

## Acceptance checks

- [ ] `cargo test --manifest-path src-tauri/Cargo.toml` passes.
- [ ] `pnpm build` passes.
- [ ] Two same-named creates in different sessions get distinct IDs and distinct
      `super.json` files.
- [ ] A second create in the same session is refused and creates no second
      project row/file.
- [ ] Concurrent `set` on different required paths leaves both values present.
- [ ] A failed JSON initialization does not leave a committed project row (test
      the reachable failure path or document why it is not injectable).
- [ ] No frontend file changed.

## Commit and push

```bash
git switch -c refactor/s3-project-json-integrity main
# edit
cargo test --manifest-path src-tauri/Cargo.toml
pnpm build
git add src-tauri/src/db.rs src-tauri/src/tools.rs src-tauri/src/super_json.rs \
        docs/refactor/sprints/README.md \
        docs/refactor/sprints/03-project-and-json-integrity.md
git commit -m "fix(storage): reserve projects atomically and serialize super-json writes"
git push -u origin refactor/s3-project-json-integrity
```

Report the branch, commit SHA, both command results, the lock scope, the
compensation order, and any partial state that remains possible.
