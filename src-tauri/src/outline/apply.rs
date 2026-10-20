//! Writes an accepted proposal batch into the project document.
//!
//! Nothing here is allowed to run from a subagent: the facade calls `commit`
//! only after the author accepted the proposals, and every row is validated
//! against the schema before the document is touched.

use serde_json::{Value, json};

use super::{
    contract::{self, Lane},
    stage::{Entity, Stage, assert_order},
};
use crate::{db, super_json};

pub type CommitReport = crate::models::CommitReport;

/// Fields a proposal may carry per stage. Everything else the model returns
/// (arc numbers, shard bookkeeping) is dropped here instead of failing
/// validation on a key the schema does not know.
const ARC_FIELDS: &[&str] = &["name", "position", "code", "summarize", "goal", "outcome"];
const CHAPTER_FIELDS: &[&str] = &[
    "name",
    "position",
    "code",
    "summarize",
    "beats",
    "scene_count",
    "goal",
    "outcome",
];

fn table_for(stage: Stage) -> Result<&'static str, String> {
    match stage {
        Stage::Arcs => Ok("arcs"),
        Stage::Chapters => Ok("chapters"),
        other => Err(format!(
            "Giai đoạn {} chưa hỗ trợ commit trong sprint này",
            other.label()
        )),
    }
}

fn entity_for(stage: Stage) -> Entity {
    match stage {
        Stage::Chapters => Entity::Chapters,
        _ => Entity::Arcs,
    }
}

fn lane_for(stage: Stage) -> Result<Lane, String> {
    Lane::for_stage(stage).ok_or_else(|| format!("Giai đoạn {} chưa có contract", stage.label()))
}

fn fields_for(stage: Stage) -> &'static [&'static str] {
    match stage {
        Stage::Chapters => CHAPTER_FIELDS,
        _ => ARC_FIELDS,
    }
}

/// Turns one proposal into a row the schema accepts: known keys only, plus the
/// draft reference `<field>_json_index` that replaces the ARC's real id.
fn row_for(stage: Stage, proposal: &Value) -> Result<Value, String> {
    let object = proposal
        .as_object()
        .ok_or_else(|| "Đề xuất phải là một object".to_owned())?;
    let mut row = serde_json::Map::new();
    for field in fields_for(stage) {
        if let Some(value) = object.get(*field) {
            row.insert((*field).to_owned(), value.clone());
        }
    }
    if stage == Stage::Chapters {
        let arc = object
            .get("arc")
            .and_then(Value::as_u64)
            .ok_or_else(|| "Đề xuất chương thiếu số thứ tự arc".to_owned())?;
        if arc == 0 {
            return Err("Số thứ tự arc phải bắt đầu từ 1".into());
        }
        row.insert("arc_id_json_index".to_owned(), json!(arc - 1));
    }
    Ok(Value::Object(row))
}

fn rows_of(document: &Value, group: &str, table: &str) -> Vec<Value> {
    document
        .get(group)
        .and_then(|value| value.get("schema"))
        .and_then(Value::as_array)
        .and_then(|entries| entries.iter().find(|entry| entry["table"] == table))
        .and_then(|entry| entry["rows"].as_array())
        .cloned()
        .unwrap_or_default()
}

/// Commit an accepted proposal batch. Phase 2 may fill required table rows, but
/// never the required project leaves the author owns.
pub fn commit(run_id: &str, stage: Stage, keys: Option<&[String]>) -> Result<CommitReport, String> {
    let table = table_for(stage)?;
    let connection = db::open()?;
    let (project_id, session_id): (String, String) = connection
        .query_row(
            "SELECT project_id, session_id FROM outline_runs WHERE id=?1",
            [run_id],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .map_err(|error| format!("Không tìm thấy lượt chạy: {error}"))?;
    let run = db::run_for_session(&connection, &session_id)?.ok_or("Không tìm thấy lượt chạy")?;
    if run.id != run_id {
        return Err("Lượt chạy không còn là lượt hiện tại".into());
    }
    let document = super_json::load_or_init(&project_id)?;
    let group = super_json::schema_table_group(table)
        .ok_or_else(|| format!("Không tìm thấy bảng {table} trong schema"))?
        .as_str();

    if stage == Stage::Chapters && rows_of(&document, "require", "arcs").is_empty() {
        return Err("Cần chốt giai đoạn Cốt truyện trước".into());
    }

    // Without explicit keys, commit what the author already accepted. With keys
    // ("chốt arc-01 và arc-02"), the named proposals are the approval itself, so
    // the chat flow does not need a separate accept call per item.
    let stored = db::list_items(&connection, run_id, stage.as_str())?;
    let mut selected = Vec::new();
    let mut skipped = 0usize;
    for item in stored {
        let approved = match keys {
            Some(requested) => {
                requested.iter().any(|key| key == &item.key) && item.status != "committed"
            }
            None => item.status == "accepted",
        };
        if approved {
            selected.push(item);
        } else {
            skipped += 1;
        }
    }
    if selected.is_empty() {
        return Err("Không có đề xuất được chấp nhận để commit".into());
    }

    // Validate the whole batch before anything is written: a single bad row must
    // leave the document untouched.
    let mut incoming = Vec::new();
    for item in &selected {
        let proposal: Value = serde_json::from_str(&item.payload_json)
            .map_err(|error| format!("Đề xuất {} có JSON không hợp lệ: {error}", item.key))?;
        contract::validate_entity(lane_for(stage)?, &proposal)?;
        incoming.push(row_for(stage, &proposal)?);
    }

    let plan_count = |leaf: &str| -> Result<usize, String> {
        document
            .pointer(&format!("/require/project/plan/{leaf}/count"))
            .and_then(Value::as_u64)
            .map(|value| value as usize)
            .ok_or_else(|| format!("Thiếu require.project.plan.{leaf}.count"))
    };

    let existing = rows_of(&document, group, table);
    let mut rows = existing.clone();
    for row in incoming {
        let position = row
            .get("position")
            .and_then(Value::as_u64)
            .ok_or_else(|| format!("{table} có đề xuất thiếu position"))?;
        match rows
            .iter_mut()
            .find(|known| known.get("position").and_then(Value::as_u64) == Some(position))
        {
            Some(slot) => *slot = row,
            None => rows.push(row),
        }
    }
    rows.sort_by_key(|row| {
        row.get("position")
            .and_then(Value::as_u64)
            .unwrap_or(u64::MAX)
    });

    let target = plan_count(if stage == Stage::Arcs {
        "arc"
    } else {
        "chapter"
    })?;
    let mut missing = Vec::new();
    for expected in 1..=target {
        if !rows
            .iter()
            .any(|row| row.get("position").and_then(Value::as_u64) == Some(expected as u64))
        {
            missing.push(format!("{table}[{expected}]"));
        }
    }
    if stage == Stage::Arcs && !missing.is_empty() {
        return Err(format!(
            "arcs.rows cần đủ {target} arc nhưng còn thiếu {}",
            missing.join(", ")
        ));
    }
    for (index, row) in rows.iter().enumerate() {
        if row.get("position").and_then(Value::as_u64) != Some((index + 1) as u64) {
            return Err(format!("{table}.rows[{index}].position phải liên tục từ 1"));
        }
    }

    let scene_target = plan_count("scene")?;
    let scene_sum: usize = rows
        .iter()
        .map(|row| row.get("scene_count").and_then(Value::as_u64).unwrap_or(0) as usize)
        .sum();
    let mut warnings: Vec<String> = Vec::new();
    if stage == Stage::Chapters {
        if missing.is_empty() && scene_sum != scene_target {
            return Err(format!(
                "chapters.scene_count cần tổng {scene_target} nhưng có {scene_sum} \
                 (thiếu/thừa {})",
                scene_sum.abs_diff(scene_target)
            ));
        }
        if !missing.is_empty() {
            warnings.push(format!(
                "Còn {} chương chưa sinh, tổng cảnh hiện tại là {scene_sum}/{scene_target}",
                missing.len()
            ));
        }
    }
    for row in &existing {
        let Some(name) = row.get("name").and_then(Value::as_str) else {
            continue;
        };
        if name.trim().is_empty() {
            continue;
        }
        let replaced = !incoming_contains(&rows, row);
        if replaced {
            warnings.push(format!(
                "Thay {} người dùng đã đặt: {name}",
                if stage == Stage::Arcs {
                    "arc"
                } else {
                    "chương"
                }
            ));
        }
    }

    let rows_path = format!("{group}.schema.{table}.rows");
    let count_path = format!("{group}.schema.{table}.count");
    guard_path(&rows_path)?;
    guard_path(&count_path)?;
    assert_order(&[entity_for(stage)])?;

    let committed = rows.len();
    super_json::set(&project_id, &rows_path, json!(rows))?;
    super_json::set(&project_id, &count_path, json!(committed))?;
    for item in &selected {
        db::set_item_status(&connection, run_id, stage.as_str(), &item.key, "committed")?;
    }
    db::update_run_status(&connection, run_id, "committed")?;
    Ok(CommitReport {
        committed,
        skipped,
        missing,
        warnings,
    })
}

/// A row the author named survives only when the new batch still carries it.
fn incoming_contains(rows: &[Value], row: &Value) -> bool {
    let Some(name) = row.get("name").and_then(Value::as_str) else {
        return true;
    };
    rows.iter().any(|candidate| {
        candidate
            .get("name")
            .and_then(Value::as_str)
            .is_some_and(|other| other.eq_ignore_ascii_case(name))
    })
}

/// Required project leaves belong to the author. Required table rows are
/// intentionally writable by phase 2 when the schema classifies that table as
/// required, which is how the outline expands `arcs` and `characters`.
fn guard_path(path: &str) -> Result<(), String> {
    if path.starts_with("require.project.") {
        return Err(
            "Trường này do tác giả quyết định ở giai đoạn 1, không ghi đè ở giai đoạn 2".into(),
        );
    }
    if let Some(tail) = path.strip_prefix("require.schema.") {
        let mut parts = tail.split('.');
        let table = parts.next().unwrap_or_default();
        let field = parts.next().unwrap_or_default();
        if parts.next().is_none()
            && matches!(field, "count" | "rows")
            && super_json::schema_table_group(table)
                .is_some_and(|group| group.as_str() == "require")
        {
            return Ok(());
        }
    } else if let Some(tail) = path.strip_prefix("optional.") {
        let parts: Vec<_> = tail.split('.').collect();
        if (parts.len() == 2 && parts[0] == "project")
            || (parts.len() == 3 && parts[0] == "schema" && matches!(parts[2], "count" | "rows"))
        {
            return Ok(());
        }
    }
    Err(format!("Không được ghi đường dẫn state: {path}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_guard_allows_required_table_rows_but_never_project_leaves() {
        assert!(guard_path("require.schema.arcs.rows").is_ok());
        assert!(guard_path("require.schema.characters.count").is_ok());
        assert!(guard_path("optional.schema.chapters.rows").is_ok());
        assert!(guard_path("require.project.name").is_err());
        assert!(guard_path("require.project.plan.arc.count").is_err());
    }

    #[test]
    fn test_chapter_row_turns_arc_number_into_draft_reference() {
        let proposal = json!({
            "code": "ch-001", "name": "Mở màn", "position": 1,
            "summarize": "Tóm tắt", "beats": ["a"], "scene_count": 3,
            "goal": "g", "outcome": "o", "arc": 2,
        });
        let row = row_for(Stage::Chapters, &proposal).unwrap();
        assert_eq!(row["arc_id_json_index"], json!(1));
        assert!(row.get("arc").is_none(), "arc is not a schema column");
        assert_eq!(row["scene_count"], json!(3));
    }

    #[test]
    fn test_arc_row_drops_bookkeeping_fields() {
        let proposal = json!({
            "code": "arc-01", "name": "Mưa", "position": 1,
            "summarize": "s", "goal": "g", "outcome": "o", "chapters": 12,
        });
        let row = row_for(Stage::Arcs, &proposal).unwrap();
        assert!(row.get("chapters").is_none());
        assert_eq!(row["code"], json!("arc-01"));
    }
}
