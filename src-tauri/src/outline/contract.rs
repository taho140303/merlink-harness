//! Output contracts for the phase-2 lanes.
//!
//! Every subagent returns exactly one JSON object; the shape is enforced here so
//! a malformed proposal never reaches the author's review screen.

use serde_json::Value;

use super::stage::Stage;
use crate::runtime::extract_json;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Only the lanes that have an implementation live here; `cast_smith`,
/// `world_smith`, `bond_smith` and `scene_binder` join in sprints 17.4 and 17.5,
/// together with their contracts.
pub enum Lane {
    ArcSmith,
    ChapterSmith,
}

impl Lane {
    pub fn key(self) -> &'static str {
        match self {
            Self::ArcSmith => "arc_smith",
            Self::ChapterSmith => "chapter_smith",
        }
    }

    /// The lane that owns a stage, so a stored proposal can be re-validated
    /// without trusting the row it came from.
    pub fn for_stage(stage: Stage) -> Option<Self> {
        match stage {
            Stage::Arcs => Some(Self::ArcSmith),
            Stage::Chapters => Some(Self::ChapterSmith),
            _ => None,
        }
    }

    /// The JSON array key this lane answers with.
    pub fn array_key(self) -> &'static str {
        match self {
            Self::ArcSmith => "arcs",
            Self::ChapterSmith => "chapters",
        }
    }
}

pub fn parse(lane: Lane, text: &str) -> Result<Value, String> {
    let value = extract_json(text)?;
    validate(lane, &value)?;
    Ok(value)
}

/// Validates one entity of a lane, the unit the author reviews and accepts.
pub fn validate_entity(lane: Lane, entity: &Value) -> Result<(), String> {
    let object = entity
        .as_object()
        .ok_or_else(|| format!("{} phải là một object", lane.key()))?;
    let (required, text_fields) = match lane {
        Lane::ArcSmith => (
            &[
                "position",
                "code",
                "name",
                "goal",
                "outcome",
                "summarize",
                "chapters",
            ][..],
            &["name", "goal", "outcome", "summarize"][..],
        ),
        Lane::ChapterSmith => (
            &[
                "arc",
                "position",
                "code",
                "name",
                "summarize",
                "beats",
                "scene_count",
                "goal",
                "outcome",
            ][..],
            &["name", "summarize", "goal", "outcome"][..],
        ),
    };
    for key in required {
        if !object.contains_key(*key) {
            return Err(format!("{}.{key} là bắt buộc", lane.key()));
        }
    }
    for key in text_fields {
        if !object.get(*key).is_some_and(Value::is_string) {
            return Err(format!("{}.{key} phải là chuỗi", lane.key()));
        }
    }
    let position = object
        .get("position")
        .and_then(Value::as_u64)
        .ok_or_else(|| format!("{}.position phải là số nguyên", lane.key()))?;
    if position == 0 {
        return Err(format!("{}.position phải bắt đầu từ 1", lane.key()));
    }
    let code = object
        .get("code")
        .and_then(Value::as_str)
        .ok_or_else(|| format!("{}.code phải là chuỗi", lane.key()))?;
    if code.is_empty()
        || !code
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
    {
        return Err(format!("{}.code phải là slug ASCII", lane.key()));
    }
    match lane {
        Lane::ArcSmith => {
            if object
                .get("chapters")
                .and_then(Value::as_u64)
                .is_none_or(|count| count == 0)
            {
                return Err("arcs.chapters phải là số nguyên dương".into());
            }
        }
        Lane::ChapterSmith => {
            let arc = object
                .get("arc")
                .and_then(Value::as_u64)
                .ok_or_else(|| "chapters.arc phải là số nguyên".to_owned())?;
            if arc == 0 {
                return Err("chapters.arc phải bắt đầu từ 1".into());
            }
            if !object.get("beats").is_some_and(Value::is_array) {
                return Err("chapters.beats phải là mảng".into());
            }
            if object
                .get("scene_count")
                .and_then(Value::as_u64)
                .is_none_or(|count| count == 0)
            {
                return Err("chapters.scene_count phải là số nguyên dương".into());
            }
        }
    }
    Ok(())
}

/// Validates a whole lane answer: one non-empty array, unique codes and
/// contiguous positions *inside the payload*. A shard may start at any position
/// (chapters 11-20 is a legitimate shard), so the check is relative.
pub fn validate(lane: Lane, value: &Value) -> Result<(), String> {
    let array_key = lane.array_key();
    let rows = value
        .get(array_key)
        .and_then(Value::as_array)
        .ok_or_else(|| format!("{array_key} phải là mảng"))?;
    if rows.is_empty() {
        return Err(format!("{array_key} không được rỗng"));
    }
    let mut codes = std::collections::BTreeSet::new();
    let mut previous: Option<u64> = None;
    for (index, row) in rows.iter().enumerate() {
        let prefix = format!("{array_key}[{index}]");
        validate_entity(lane, row).map_err(|error| format!("{prefix}: {error}"))?;
        let position = row
            .get("position")
            .and_then(Value::as_u64)
            .ok_or_else(|| format!("{prefix}.position phải là số nguyên"))?;
        if let Some(previous) = previous
            && position != previous + 1
        {
            return Err(format!("{prefix}.position phải liên tục từ vị trí trước"));
        }
        previous = Some(position);
        let code = row
            .get("code")
            .and_then(Value::as_str)
            .ok_or_else(|| format!("{prefix}.code phải là chuỗi"))?;
        if !codes.insert(code.to_owned()) {
            return Err(format!("{prefix}.code bị trùng"));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn arc(position: u64, code: &str) -> Value {
        json!({
            "position": position, "code": code, "name": "Tên", "goal": "Mục tiêu",
            "outcome": "Kết quả", "summarize": "Tóm tắt", "chapters": 4,
        })
    }

    #[test]
    fn test_contract_rejects_non_contiguous_positions() {
        let value = json!({"arcs": [arc(1, "arc-01"), arc(3, "arc-02")]});
        let error = validate(Lane::ArcSmith, &value).unwrap_err();
        assert!(error.contains("arcs[1].position"), "{error}");
    }

    #[test]
    fn test_contract_accepts_shard_starting_after_one() {
        let value = json!({"arcs": [arc(11, "arc-11"), arc(12, "arc-12")]});
        assert!(validate(Lane::ArcSmith, &value).is_ok());
    }

    #[test]
    fn test_contract_rejects_duplicate_codes_and_bad_slug() {
        let duplicate = json!({"arcs": [arc(1, "arc-01"), arc(2, "arc-01")]});
        assert!(validate(Lane::ArcSmith, &duplicate).is_err());
        let bad_slug = json!({"arcs": [arc(1, "Arc 01")]});
        assert!(validate(Lane::ArcSmith, &bad_slug).is_err());
    }

    #[test]
    fn test_contract_rejects_wrong_array_key_and_empty_payload() {
        let wrong_key = json!({"arcs": [arc(1, "arc-01")]});
        assert!(validate(Lane::ChapterSmith, &wrong_key).is_err());
        assert!(validate(Lane::ArcSmith, &json!({"arcs": []})).is_err());
    }

    #[test]
    fn test_chapter_entity_requires_beats_and_positive_scene_count() {
        let mut chapter = json!({
            "arc": 1, "position": 1, "code": "ch-001", "name": "Mở màn",
            "summarize": "Tóm tắt", "beats": ["a"], "scene_count": 3,
            "goal": "g", "outcome": "o",
        });
        assert!(validate_entity(Lane::ChapterSmith, &chapter).is_ok());
        chapter["beats"] = json!("không phải mảng");
        assert!(validate_entity(Lane::ChapterSmith, &chapter).is_err());
        chapter["beats"] = json!(["a"]);
        chapter["scene_count"] = json!(0);
        assert!(validate_entity(Lane::ChapterSmith, &chapter).is_err());
    }
}
