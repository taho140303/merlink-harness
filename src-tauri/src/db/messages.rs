//! Message rows: idempotent append, transcript listing, and test-only counts.

use rusqlite::{Connection, params};

use crate::models::MessageEnvelope;

use super::now;

pub fn append_message(
    connection: &Connection,
    session_id: &str,
    message: &MessageEnvelope,
) -> Result<(), String> {
    let payload = serde_json::to_string(message)
        .map_err(|error| format!("Cannot encode message: {error}"))?;
    let changed = connection
        .execute(
            "INSERT INTO messages (id, session_id, role, content, payload_json, created_at) \
             VALUES (?1, ?2, ?3, ?4, ?5, ?6) \
             ON CONFLICT(id) DO UPDATE SET \
                 role = excluded.role, \
                 content = excluded.content, \
                 payload_json = excluded.payload_json \
             WHERE messages.session_id = excluded.session_id",
            params![
                message.id,
                session_id,
                message.role,
                message.content,
                payload,
                now()
            ],
        )
        .map_err(|error| format!("Cannot append message: {error}"))?;
    if changed == 0 {
        let stored_session: String = connection
            .query_row(
                "SELECT session_id FROM messages WHERE id = ?1",
                params![message.id],
                |row| row.get(0),
            )
            .map_err(|error| {
                format!("Cannot read conflicting message '{}': {error}", message.id)
            })?;
        return Err(format!(
            "Cannot append message '{}': ID already belongs to session '{}', not requested session '{}'",
            message.id, stored_session, session_id
        ));
    }
    Ok(())
}

/// Returns the transcript in insertion order.
///
/// `created_at` has millisecond resolution and a tool loop can append several
/// messages inside the same millisecond, so ordering uses SQLite's implicit
/// `rowid` instead.
pub fn list_messages(
    connection: &Connection,
    session_id: &str,
) -> Result<Vec<MessageEnvelope>, String> {
    let mut statement = connection
        .prepare(
            "SELECT id, role, content, payload_json FROM messages \
             WHERE session_id = ?1 ORDER BY rowid ASC",
        )
        .map_err(|error| format!("Cannot read messages: {error}"))?;
    let rows = statement
        .query_map(params![session_id], |row| {
            let id: String = row.get(0)?;
            let role: String = row.get(1)?;
            let content: String = row.get(2)?;
            let payload: Option<String> = row.get(3)?;
            Ok((id, role, content, payload))
        })
        .map_err(|error| format!("Cannot read messages: {error}"))?;

    let mut messages = Vec::new();
    for row in rows {
        let (id, role, content, payload) =
            row.map_err(|error| format!("Cannot read message row: {error}"))?;
        messages.push(match payload {
            Some(raw) => {
                let mut message =
                    serde_json::from_str::<MessageEnvelope>(&raw).map_err(|error| {
                        format!("Cannot decode message '{id}' in session '{session_id}': {error}")
                    })?;
                // The primary key, not the JSON copy, is authoritative.
                message.id = id;
                message
            }
            None => MessageEnvelope::new(id, role, content),
        });
    }
    Ok(messages)
}

/// Message count for a session. Only the tests need it so far; kept next to
/// `list_messages` because they share the same query shape.
#[cfg(test)]
pub fn count_messages(connection: &Connection, session_id: &str) -> Result<u64, String> {
    // rusqlite has no `FromSql` for u64, so read an i64 and widen.
    let count: i64 = connection
        .query_row(
            "SELECT COUNT(*) FROM messages WHERE session_id = ?1",
            params![session_id],
            |row| row.get(0),
        )
        .map_err(|error| format!("Cannot count messages: {error}"))?;
    Ok(u64::try_from(count).unwrap_or(0))
}

#[cfg(test)]
mod tests {
    use super::super::{ensure_project, ensure_session, open};
    use super::{append_message, count_messages, list_messages};
    use crate::models::{MessageEnvelope, ToolCall};
    use rusqlite::{Connection, params};
    use serde_json::json;

    fn table_names(connection: &Connection) -> Vec<String> {
        let mut statement = connection
            .prepare("SELECT name FROM sqlite_master WHERE type = 'table' ORDER BY name")
            .unwrap();
        let rows = statement
            .query_map([], |row| row.get::<_, String>(0))
            .unwrap();
        rows.map(Result::unwrap).collect()
    }

    #[test]
    fn test_db_message_round_trip_and_tables() {
        let _home = crate::test_support::TempHome::new("db");
        let connection = open().unwrap();

        let tables = table_names(&connection);
        for expected in ["projects", "sessions", "messages"] {
            assert!(tables.contains(&expected.to_string()), "missing {expected}");
        }

        ensure_project(&connection, "default", "Default project").unwrap();
        ensure_session(&connection, "sess-1", "default").unwrap();

        let user = MessageEnvelope::user("m1", "Đặt tên project là Nửa Đêm");
        let mut assistant = MessageEnvelope::new("m2", "assistant", "");
        assistant.tool_calls = Some(vec![ToolCall {
            id: "call_1".into(),
            name: "super_json_set".into(),
            arguments: json!({"path": "require.overview.name", "value": "Nửa Đêm"}),
        }]);
        let tool =
            MessageEnvelope::tool_result("m3", "call_1", "super_json_set", &json!({"ok": true}));
        let final_message = MessageEnvelope::new("m4", "assistant", "Đã đặt tên xong.");

        for message in [&user, &assistant, &tool, &final_message] {
            append_message(&connection, "sess-1", message).unwrap();
        }

        let listed = list_messages(&connection, "sess-1").unwrap();
        assert_eq!(listed.len(), 4);
        assert_eq!(
            listed.iter().map(|m| m.role.as_str()).collect::<Vec<_>>(),
            vec!["user", "assistant", "tool", "assistant"]
        );
        assert_eq!(count_messages(&connection, "sess-1").unwrap(), 4);

        // Tool wiring must survive the SQLite round trip.
        assert_eq!(
            listed[1].tool_calls.as_ref().unwrap()[0].name,
            "super_json_set"
        );
        assert_eq!(listed[2].tool_call_id.as_deref(), Some("call_1"));
        assert_eq!(listed[2].name.as_deref(), Some("super_json_set"));

        // Sessions are isolated from each other.
        assert!(list_messages(&connection, "sess-other").unwrap().is_empty());
    }

    #[test]
    fn test_append_message_replay_preserves_count_order_and_created_at() {
        let _home = crate::test_support::TempHome::new("db-message-replay");
        let connection = open().unwrap();
        for id in ["first", "middle", "last"] {
            append_message(&connection, "session-a", &MessageEnvelope::user(id, id)).unwrap();
        }
        connection
            .execute(
                "UPDATE messages SET created_at = 'original-timestamp' WHERE id = 'middle'",
                [],
            )
            .unwrap();
        let before: Vec<(i64, String)> = connection
            .prepare("SELECT rowid, id FROM messages ORDER BY rowid ASC")
            .unwrap()
            .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))
            .unwrap()
            .map(Result::unwrap)
            .collect();

        for _ in 0..2 {
            append_message(
                &connection,
                "session-a",
                &MessageEnvelope::user("middle", "updated"),
            )
            .unwrap();
        }

        let after: Vec<(i64, String)> = connection
            .prepare("SELECT rowid, id FROM messages ORDER BY rowid ASC")
            .unwrap()
            .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))
            .unwrap()
            .map(Result::unwrap)
            .collect();
        assert_eq!(count_messages(&connection, "session-a").unwrap(), 3);
        assert_eq!(
            after, before,
            "replay must keep each rowid and transcript order"
        );
        assert_eq!(
            list_messages(&connection, "session-a")
                .unwrap()
                .iter()
                .map(|message| message.id.as_str())
                .collect::<Vec<_>>(),
            vec!["first", "middle", "last"]
        );
        let (created_at, content): (String, String) = connection
            .query_row(
                "SELECT created_at, content FROM messages WHERE id = 'middle'",
                [],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .unwrap();
        assert_eq!(created_at, "original-timestamp");
        assert_eq!(content, "updated");
    }

    #[test]
    fn test_append_message_rejects_cross_session_duplicate() {
        let _home = crate::test_support::TempHome::new("db-cross-session-id");
        let connection = open().unwrap();
        append_message(
            &connection,
            "session-a",
            &MessageEnvelope::user("shared-id", "original"),
        )
        .unwrap();

        let error = append_message(
            &connection,
            "session-b",
            &MessageEnvelope::user("shared-id", "replacement"),
        )
        .unwrap_err();

        assert_eq!(
            error,
            "Cannot append message 'shared-id': ID already belongs to session 'session-a', not requested session 'session-b'"
        );
        assert_eq!(count_messages(&connection, "session-a").unwrap(), 1);
        assert_eq!(count_messages(&connection, "session-b").unwrap(), 0);
        assert_eq!(
            list_messages(&connection, "session-a").unwrap()[0].content,
            "original"
        );
    }

    #[test]
    fn test_list_messages_reconstructs_null_payload_with_stored_id() {
        let _home = crate::test_support::TempHome::new("db-legacy-null-payload");
        let connection = open().unwrap();
        connection
            .execute(
                "INSERT INTO messages (id, session_id, role, content, payload_json, created_at) \
                 VALUES ('legacy-id', 'session-a', 'tool', 'legacy content', NULL, 'original')",
                [],
            )
            .unwrap();

        let listed = list_messages(&connection, "session-a").unwrap();
        assert_eq!(listed.len(), 1);
        assert_eq!(listed[0].id, "legacy-id");
        assert_eq!(listed[0].role, "tool");
        assert_eq!(listed[0].content, "legacy content");
    }

    #[test]
    fn test_list_messages_reports_corrupt_payload_instead_of_recovery() {
        let _home = crate::test_support::TempHome::new("db-corrupt-payload");
        let connection = open().unwrap();
        connection
            .execute(
                "INSERT INTO messages (id, session_id, role, content, payload_json, created_at) \
                 VALUES ('corrupt-id', 'session-a', 'user', 'original', '{invalid', 'original')",
                [],
            )
            .unwrap();

        let error = list_messages(&connection, "session-a").unwrap_err();
        assert!(error.contains("corrupt-id"), "{error}");
        assert!(error.contains("session-a"), "{error}");
    }

    #[test]
    fn test_list_messages_uses_stored_id_not_payload_id() {
        let _home = crate::test_support::TempHome::new("db-mismatched-payload-id");
        let connection = open().unwrap();
        append_message(
            &connection,
            "session-a",
            &MessageEnvelope::user("stored-id", "original"),
        )
        .unwrap();
        let payload =
            serde_json::to_string(&MessageEnvelope::user("wrong-id", "original")).unwrap();
        connection
            .execute(
                "UPDATE messages SET payload_json = ?1 WHERE id = 'stored-id'",
                params![payload],
            )
            .unwrap();
        assert_eq!(
            list_messages(&connection, "session-a").unwrap()[0].id,
            "stored-id"
        );
    }
}
