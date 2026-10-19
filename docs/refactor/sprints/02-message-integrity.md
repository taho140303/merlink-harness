# S2 — Message identity and database integrity

- **Status:** pushed
- **Branch:** `refactor/s2-message-integrity`
- **Units:** F-01 (backend half), F-20 (database part)
- **Depends on:** S1
- **Code changes:** `src-tauri/src/db.rs`, `src-tauri/src/runtime.rs` (only where
  it calls the DB), `src-tauri/src/lib.rs` (only if a test needs it)

## Goal

Make message persistence append-safe: a message ID may only be replayed inside
its own session, a legitimate replay must not change transcript order, a stored
message must come back with its real ID, and a corrupt payload must be reported
instead of silently replaced by a fabricated message.

## Evidence

- `src-tauri/src/db.rs:19-26` makes `messages.id` the sole primary key.
- `src-tauri/src/db.rs:173-194` writes with `INSERT OR REPLACE`, which deletes
  and re-inserts the row and can steal an ID from another session.
- `src-tauri/src/runtime.rs:260-275` replays the caller's whole history on every
  turn; `:370-400` appends the final assistant message twice (once without meta,
  once with it).
- `src-tauri/src/db.rs:198-210` promises insertion order via `rowid`; `REPLACE`
  breaks that promise.
- `src-tauri/src/db.rs:203-233` selects no `id`, then fabricates
  `recovered-{index}` for both a legitimate `NULL` payload and corrupt JSON.
- `src-tauri/src/db.rs:5-59` has no `PRAGMA user_version` or migration hook.
- The frontend half (UUID message IDs) is S1; this sprint still enforces the
  invariant at the storage boundary, because an old client or a stale tab can
  send any ID.

## Decisions (locked)

1. **Keep the single-column primary key.** Do not rebuild the `messages` table in
   this phase. A composite `(session_id, id)` key is a later migration with its
   own release plan.
2. **Conditional upsert.** Replace `INSERT OR REPLACE` with:

   ```sql
   INSERT INTO messages (id, session_id, role, content, payload_json, created_at)
   VALUES (?1, ?2, ?3, ?4, ?5, ?6)
   ON CONFLICT(id) DO UPDATE SET
       role = excluded.role,
       content = excluded.content,
       payload_json = excluded.payload_json
   WHERE messages.session_id = excluded.session_id
   ```

   Do **not** overwrite `created_at`. A legitimate replay in the same session
   updates in place and keeps the rowid. A conflict with a different session
   updates zero rows; `append_message` must detect that and return a clear error
   containing the message ID and both session IDs (no secrets).
3. **Read the real ID.** `list_messages` selects `id, role, content,
   payload_json`. For a present payload, deserialize it and **override/verify**
   its `id` with the stored ID (or reject a mismatch) so history cannot invent
   identities. For `payload_json IS NULL` (known legacy), construct a
   `MessageEnvelope` with the stored ID, role, and content. For a present but
   invalid payload, return an error naming the message ID and session; do not
   fabricate a row.
4. **Ordering.** Keep `ORDER BY rowid ASC`. The conditional upsert must not
   change rowids on replay. Add a test that replays an existing message and
   asserts both the row count and the order are unchanged.
5. **Schema version.** Add `const SCHEMA_VERSION: i64 = 1;` and a migration
   function in `db::open()`:
   - Read `PRAGMA user_version`.
   - If 0: run the existing schema creation and `PRAGMA user_version = 1` inside
     one transaction; commit.
   - If 1: run `CREATE TABLE IF NOT EXISTS`/index defensively (idempotent).
   - If greater than 1: return an error ("database was created by a newer
     version") rather than modifying it.
   - Do not use `execute_batch` with a transaction opened by `Connection::transaction`
     in a way that nests; pick one style and test opening twice.
6. **No backfill of old data.** Existing rows with shifted rowids stay as they
   are; document this in a comment. Do not delete or renumber rows.

## Tasks

1. `db.rs`:
   - Add the schema-version constant and `migrate(connection)` helper.
   - Change `append_message` to the conditional upsert + zero-row detection.
   - Change `list_messages` to select and preserve `id`; separate the legacy
     `NULL` path from the corrupt-JSON path.
   - Add a `MessageStore` test module covering:
     * same ID, same session, replayed twice → one row, same rowid/order;
     * same ID, different sessions → second call errors, first row unchanged;
     * `NULL` payload → stored ID/role/content returned;
     * invalid JSON payload → `Err` containing the message ID;
     * `user_version` 0 → 1 on open; reopening is idempotent.
2. `runtime.rs`: adjust only call sites/tests if the new error path or row
   construction changes behavior. Do not redesign the loop here (S4).
3. Update any test fixture that inserts a raw payload, if needed. Keep the
   existing `test_message_envelope_tolerates_legacy_payload` and
   `test_tool_call_round_trip_with_empty_content` passing.
4. If `append_message` now rejects a cross-session ID, add a regression test that
   goes through `runtime::send_chat_internal` or `run_agent_turn` with two
   sessions' histories, not only the DB unit test.

## Explicitly out of scope

- Project creation, session binding, super-JSON locks (S3).
- Partial-turn recovery, event ordering, round cap (S4).
- IPC typing, preview, `execute_tool` (S6).
- A composite primary key, rowid renumbering, or a data-repair UI.

## Acceptance checks

- [ ] `cargo test --manifest-path src-tauri/Cargo.toml` passes.
- [ ] `pnpm build` passes.
- [ ] A duplicate ID from another session is rejected and does not modify the
      original row.
- [ ] Replaying an existing message leaves `COUNT(*)` and `ORDER BY rowid`
      unchanged.
- [ ] A malformed payload is an observable error, not `recovered-{index}`.
- [ ] `PRAGMA user_version` is 1 after the first open and stays 1 after a second
      open.
- [ ] No frontend file changed.

## Commit and push

```bash
git switch -c refactor/s2-message-integrity main
# edit
cargo test --manifest-path src-tauri/Cargo.toml
pnpm build
git add src-tauri/src/db.rs src-tauri/src/runtime.rs \
        docs/refactor/sprints/README.md docs/refactor/sprints/02-message-integrity.md
git commit -m "fix(db): make message replay session-safe and preserve transcript order"
git push -u origin refactor/s2-message-integrity
```

Report the branch, commit SHA, both command results, the exact conflict error
text, and the migration/version behavior.
