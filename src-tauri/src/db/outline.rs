//! Persistence for outline runs and their stage proposals.

use rusqlite::{Connection, params};
use serde::{Deserialize, Serialize};

use super::now;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OutlineRun {
    pub id: String,
    pub project_id: String,
    pub session_id: String,
    pub stage: String,
    pub status: String,
    pub calls: i64,
    pub tokens_in: i64,
    pub tokens_out: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OutlineStoredItem {
    pub id: String,
    pub run_id: String,
    pub stage: String,
    pub lane: String,
    pub shard: String,
    pub key: String,
    pub position: u32,
    pub payload_json: String,
    pub status: String,
    pub error: Option<String>,
}

pub fn create_run(
    connection: &Connection,
    project_id: &str,
    session_id: &str,
) -> Result<OutlineRun, String> {
    if let Some(run) = run_for_session(connection, session_id)? {
        return Ok(run);
    }
    let run = OutlineRun {
        id: format!("outline-{}-{}", session_id, now()),
        project_id: project_id.to_owned(),
        session_id: session_id.to_owned(),
        stage: "seed".into(),
        status: "active".into(),
        calls: 0,
        tokens_in: 0,
        tokens_out: 0,
    };
    connection.execute(
        "INSERT INTO outline_runs (id, project_id, session_id, stage, status, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?6)",
        params![run.id, run.project_id, run.session_id, run.stage, run.status, now()],
    ).map_err(|error| format!("Cannot create outline run: {error}"))?;
    Ok(run)
}

pub fn run_for_session(
    connection: &Connection,
    session_id: &str,
) -> Result<Option<OutlineRun>, String> {
    connection.query_row(
        "SELECT id, project_id, session_id, stage, status, calls, tokens_in, tokens_out FROM outline_runs WHERE session_id = ?1 ORDER BY created_at DESC, rowid DESC LIMIT 1",
        [session_id],
        |row| Ok(OutlineRun { id: row.get(0)?, project_id: row.get(1)?, session_id: row.get(2)?, stage: row.get(3)?, status: row.get(4)?, calls: row.get(5)?, tokens_in: row.get(6)?, tokens_out: row.get(7)? }),
    ).optional().map_err(|error| format!("Cannot read outline run: {error}"))
}

pub fn update_run_stage(connection: &Connection, run_id: &str, stage: &str) -> Result<(), String> {
    update_run_text(connection, "stage", run_id, stage)
}

pub fn update_run_status(
    connection: &Connection,
    run_id: &str,
    status: &str,
) -> Result<(), String> {
    update_run_text(connection, "status", run_id, status)
}

fn update_run_text(
    connection: &Connection,
    column: &str,
    run_id: &str,
    value: &str,
) -> Result<(), String> {
    let query = format!("UPDATE outline_runs SET {column} = ?1, updated_at = ?2 WHERE id = ?3");
    let changed = connection
        .execute(&query, params![value, now(), run_id])
        .map_err(|error| format!("Cannot update outline run: {error}"))?;
    if changed == 0 {
        return Err(format!("Outline run not found: {run_id}"));
    }
    Ok(())
}

pub fn update_run_usage(
    connection: &Connection,
    run_id: &str,
    calls: i64,
    tokens_in: i64,
    tokens_out: i64,
) -> Result<(), String> {
    connection.execute("UPDATE outline_runs SET calls = ?1, tokens_in = ?2, tokens_out = ?3, updated_at = ?4 WHERE id = ?5", params![calls, tokens_in, tokens_out, now(), run_id]).map_err(|error| format!("Cannot update outline usage: {error}"))?;
    Ok(())
}

pub fn upsert_items(connection: &Connection, items: &[OutlineStoredItem]) -> Result<(), String> {
    let transaction = connection
        .unchecked_transaction()
        .map_err(|error| format!("Cannot start outline item write: {error}"))?;
    for item in items {
        let changed = transaction.execute("INSERT INTO outline_items (id, run_id, stage, lane, shard, key, position, payload_json, status, error, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11) ON CONFLICT(run_id, stage, key) DO UPDATE SET lane=excluded.lane, shard=excluded.shard, position=excluded.position, payload_json=excluded.payload_json, status=excluded.status, error=excluded.error WHERE outline_items.lane = excluded.lane AND outline_items.shard = excluded.shard AND outline_items.status = 'pending'", params![item.id, item.run_id, item.stage, item.lane, item.shard, item.key, item.position, item.payload_json, item.status, item.error, now()]).map_err(|error| format!("Cannot save outline item: {error}"))?;
        if changed == 0 {
            return Err(format!(
                "Outline key '{}' is already used by a different lane or shard in stage '{}'",
                item.key, item.stage
            ));
        }
    }
    transaction
        .commit()
        .map_err(|error| format!("Cannot commit outline items: {error}"))
}

pub fn list_items(
    connection: &Connection,
    run_id: &str,
    stage: &str,
) -> Result<Vec<OutlineStoredItem>, String> {
    let mut statement = connection.prepare("SELECT id, run_id, stage, lane, shard, key, position, payload_json, status, error FROM outline_items WHERE run_id = ?1 AND stage = ?2 ORDER BY position, key").map_err(|error| format!("Cannot prepare outline item list: {error}"))?;
    statement
        .query_map(params![run_id, stage], |row| {
            Ok(OutlineStoredItem {
                id: row.get(0)?,
                run_id: row.get(1)?,
                stage: row.get(2)?,
                lane: row.get(3)?,
                shard: row.get(4)?,
                key: row.get(5)?,
                position: row.get(6)?,
                payload_json: row.get(7)?,
                status: row.get(8)?,
                error: row.get(9)?,
            })
        })
        .map_err(|error| format!("Cannot list outline items: {error}"))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| format!("Cannot read outline items: {error}"))
}

pub fn set_item_status(
    connection: &Connection,
    run_id: &str,
    stage: &str,
    key: &str,
    status: &str,
) -> Result<(), String> {
    connection
        .execute(
            "UPDATE outline_items SET status=?1 WHERE run_id=?2 AND stage=?3 AND key=?4",
            params![status, run_id, stage, key],
        )
        .map_err(|error| format!("Cannot update outline item: {error}"))?;
    Ok(())
}

/// Drops every item a shard produced, so a regeneration replaces the batch
/// instead of leaving stale keys behind (codes can change between proposals).
pub fn delete_items_for_shard(
    connection: &Connection,
    run_id: &str,
    stage: &str,
    shard: &str,
) -> Result<(), String> {
    connection
        .execute(
            "DELETE FROM outline_items WHERE run_id=?1 AND stage=?2 AND shard=?3 AND status='pending'",
            params![run_id, stage, shard],
        )
        .map_err(|error| format!("Cannot clear outline items: {error}"))?;
    Ok(())
}

use rusqlite::OptionalExtension;

#[cfg(test)]
mod tests {
    use super::run_for_session;
    use crate::db::{ensure_project, ensure_session, open};

    #[test]
    fn upsert_items_rejects_key_collision_and_preserves_accepted_proposal() {
        use super::{OutlineStoredItem, list_items, set_item_status, upsert_items};
        use serde_json::json;

        let _home = crate::test_support::TempHome::new("outline-key-lane-conflict");
        let connection = open().unwrap();
        ensure_project(&connection, "outline-key-project", "Project").unwrap();
        ensure_session(&connection, "outline-key-session", "outline-key-project").unwrap();
        super::super::create_run(&connection, "outline-key-project", "outline-key-session")
            .unwrap();
        let run = super::super::run_for_session(&connection, "outline-key-session")
            .unwrap()
            .unwrap();
        let item = |lane: &str, shard: &str, status: &str| OutlineStoredItem {
            id: format!("item-{lane}"),
            run_id: run.id.clone(),
            stage: "world".into(),
            lane: lane.into(),
            shard: shard.into(),
            key: "entity-01".into(),
            position: 1,
            payload_json: json!({"code":"entity-01"}).to_string(),
            status: status.into(),
            error: None,
        };
        upsert_items(&connection, &[item("world_smith", "world-01", "pending")]).unwrap();
        set_item_status(&connection, &run.id, "world", "entity-01", "accepted").unwrap();
        let conflict =
            upsert_items(&connection, &[item("bond_smith", "world-03", "pending")]).unwrap_err();
        assert!(conflict.contains("different lane or shard"));
        let stored = list_items(&connection, &run.id, "world").unwrap();
        assert_eq!(stored.len(), 1);
        assert_eq!(stored[0].lane, "world_smith");
        assert_eq!(stored[0].status, "accepted");
    }

    #[test]
    fn run_for_session_breaks_same_timestamp_ties_by_newest_rowid() {
        let _home = crate::test_support::TempHome::new("outline-run-order-tie");
        let connection = open().unwrap();
        ensure_project(&connection, "outline-order-project", "Project").unwrap();
        ensure_session(
            &connection,
            "outline-order-session",
            "outline-order-project",
        )
        .unwrap();
        connection.execute(
            "INSERT INTO outline_runs (id, project_id, session_id, stage, status, created_at, updated_at) VALUES ('old-run','outline-order-project','outline-order-session','arcs','review','same-time','same-time'), ('new-run','outline-order-project','outline-order-session','arcs','review','same-time','same-time')",
            [],
        ).unwrap();

        let latest = run_for_session(&connection, "outline-order-session")
            .unwrap()
            .unwrap();
        assert_eq!(latest.id, "new-run");
    }
}
