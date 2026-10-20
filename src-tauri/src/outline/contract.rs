//! Output contracts for the phase-2 lanes.
//!
//! Every subagent returns exactly one JSON object; the shape is enforced here so
//! a malformed proposal never reaches the author's review screen.

use serde_json::Value;

use super::stage::Stage;
use crate::runtime::extract_json;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
// The public lane names are part of the persisted contract and intentionally share this suffix.
#[allow(clippy::enum_variant_names)]
/// Only the lanes that have an implementation live here; `cast_smith`,
/// `world_smith`, `bond_smith` and `scene_binder` join in sprints 17.4 and 17.5,
/// together with their contracts.
pub enum Lane {
    ArcSmith,
    ChapterSmith,
    CastSmith,
    WorldSmith,
    BondSmith,
}

impl Lane {
    pub fn key(self) -> &'static str {
        match self {
            Self::ArcSmith => "arc_smith",
            Self::ChapterSmith => "chapter_smith",
            Self::CastSmith => "cast_smith",
            Self::WorldSmith => "world_smith",
            Self::BondSmith => "bond_smith",
        }
    }

    /// The lane that owns a stage, so a stored proposal can be re-validated
    /// without trusting the row it came from.
    pub fn for_stage(stage: Stage) -> Option<Self> {
        match stage {
            Stage::Arcs => Some(Self::ArcSmith),
            Stage::Chapters => Some(Self::ChapterSmith),
            Stage::Cast => Some(Self::CastSmith),
            Stage::World => Some(Self::WorldSmith),
            _ => None,
        }
    }

    /// The JSON array key this lane answers with.
    pub fn array_key(self) -> &'static str {
        match self {
            Self::ArcSmith => "arcs",
            Self::ChapterSmith => "chapters",
            Self::CastSmith => "characters",
            Self::WorldSmith => "world",
            Self::BondSmith => "relationships",
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
        Lane::CastSmith => (
            &[
                "ref",
                "name",
                "role",
                "archetype",
                "motivation",
                "want",
                "need",
                "flaw",
                "summary",
                "description",
                "is_primary",
            ][..],
            &[
                "name",
                "role",
                "archetype",
                "motivation",
                "want",
                "need",
                "flaw",
                "summary",
                "description",
            ][..],
        ),
        Lane::WorldSmith => (&["ref", "kind"][..], &[][..]),
        Lane::BondSmith => (
            &[
                "from",
                "to",
                "summary",
                "origin",
                "is_primary",
                "states",
                "roles",
            ][..],
            &["summary", "origin"][..],
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
    if matches!(lane, Lane::ArcSmith | Lane::ChapterSmith) {
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
    }
    match lane {
        Lane::ArcSmith
            if object
                .get("chapters")
                .and_then(Value::as_u64)
                .is_none_or(|count| count == 0) =>
        {
            return Err("arcs.chapters phải là số nguyên dương".into());
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
        _ => {}
    }
    match lane {
        Lane::CastSmith => {
            slug(object, "ref", lane)?;
            if !object.get("is_primary").is_some_and(Value::is_boolean) {
                return Err("cast_smith.is_primary phải là boolean".into());
            }
            if let Some(appearance) = object.get("appearance") {
                for key in ["height_unit", "hair", "eyes", "outfit_style"] {
                    if appearance.get(key).is_some_and(|v| !v.is_string()) {
                        return Err(format!("appearance.{key} phải là chuỗi"));
                    }
                }
                if appearance.get("height").is_some_and(|v| !v.is_number())
                    || appearance.get("marks").is_some_and(|v| !string_array(v))
                {
                    return Err("appearance không hợp lệ".into());
                }
                if let Some(unit) = appearance.get("height_unit").and_then(Value::as_str)
                    && !["MM", "CM", "M"].contains(&unit)
                {
                    return Err(format!("height_unit không hợp lệ: {unit}"));
                }
            }
            if let Some(motifs) = object.get("motifs") {
                for code in motifs
                    .as_array()
                    .ok_or("motifs phải là mảng")?
                    .iter()
                    .filter_map(Value::as_str)
                {
                    if !crate::catalog::all(crate::catalog::Kind::Motifs)
                        .iter()
                        .any(|entry| entry.code == code)
                    {
                        return Err(format!("Mã motif không tồn tại: {code}"));
                    }
                }
            }
            for field in ["competences", "memories", "knowledge"] {
                if object.get(field).is_some_and(|v| !v.is_array()) {
                    return Err(format!("{field} phải là mảng"));
                }
            }
            validate_records(
                object,
                "competences",
                &["domain", "name", "level"],
                &["capabilities", "limitations"],
            )?;
            validate_records(
                object,
                "memories",
                &["type", "summary", "certainty", "source", "significance"],
                &[],
            )?;
            validate_records(object, "knowledge", &["type", "path", "certainty"], &[])?;
        }
        Lane::WorldSmith => {
            slug(object, "ref", lane)?;
            let kind = string(object, "kind")?;
            match kind {
                "setting" => {
                    for key in [
                        "name",
                        "type",
                        "description",
                        "access",
                        "spatial_features",
                        "sensory",
                    ] {
                        if !object.contains_key(key) {
                            return Err(format!("world.{key} là bắt buộc"));
                        }
                    }
                    if ![
                        "REALM", "WORLD", "REGION", "COUNTRY", "CITY", "TOWN", "FACILITY",
                        "STREET", "ROOM",
                    ]
                    .contains(&string(object, "type")?)
                    {
                        return Err("setting.type không hợp lệ".into());
                    }
                    if ![
                        "PUBLIC",
                        "PRIVATE",
                        "SECRET",
                        "LOCKED",
                        "GUARDED",
                        "RESTRICTED",
                    ]
                    .contains(&string(object, "access")?)
                    {
                        return Err("setting.access không hợp lệ".into());
                    }
                    if !object["sensory"].is_object() {
                        return Err("setting.sensory phải là object".into());
                    }
                    for key in ["visual", "auditory", "tactile", "olfactory", "gustatory"] {
                        if !object["sensory"].get(key).is_some_and(Value::is_string) {
                            return Err(format!("sensory.{key} phải là chuỗi"));
                        }
                    }
                }
                "lore" => {
                    for key in ["code", "type", "scope", "name", "description"] {
                        if !object.get(key).is_some_and(Value::is_string) {
                            return Err(format!("lore.{key} phải là chuỗi"));
                        }
                    }
                    if !["CULTURE", "HISTORY", "FACTION", "ITEM"].contains(&string(object, "type")?)
                    {
                        return Err("lore.type không hợp lệ".into());
                    }
                }
                "power" => {
                    for key in [
                        "name",
                        "source",
                        "scope",
                        "description",
                        "rules",
                        "cost",
                        "limitations",
                    ] {
                        if !object.get(key).is_some_and(Value::is_string) {
                            return Err(format!("power.{key} phải là chuỗi"));
                        }
                    }
                    for field in ["tiers", "abilities"] {
                        if object.get(field).is_some_and(|v| !v.is_array()) {
                            return Err(format!("{field} phải là mảng"));
                        }
                    }
                }
                _ => return Err(format!("world.kind không hợp lệ: {kind}")),
            }
        }
        Lane::BondSmith => {
            slug(object, "from", lane)?;
            slug(object, "to", lane)?;
            if !object["is_primary"].is_boolean() {
                return Err("is_primary phải là boolean".into());
            }
            for key in [
                "trust",
                "affection",
                "hostility",
                "fear",
                "respect",
                "familiarity",
                "dependency",
            ] {
                // The schema stores a qualitative level per dimension, not a
                // score, so the contract must not ask the model for numbers.
                const LEVELS: [&str; 5] = ["NONE", "LOW", "MODERATE", "HIGH", "ABSOLUTE"];
                if !object["states"]
                    .get(key)
                    .and_then(Value::as_str)
                    .is_some_and(|value| LEVELS.contains(&value))
                {
                    return Err(format!(
                        "states.{key} phải là một trong NONE, LOW, MODERATE, HIGH, ABSOLUTE"
                    ));
                }
            }
            let roles = object["roles"].as_array().ok_or("roles phải là mảng")?;
            for role in roles {
                if ![
                    "FAMILY",
                    "ROMANTIC",
                    "SOCIAL",
                    "PROFESSIONAL",
                    "ANTAGONISTIC",
                ]
                .contains(&role["category"].as_str().unwrap_or(""))
                    || !["SUPERIOR", "EQUAL", "INFERIOR"]
                        .contains(&role["hierarchy"].as_str().unwrap_or(""))
                    || !role["sub_type"].is_string()
                    || !role["is_primary"].is_boolean()
                {
                    return Err("roles có phần tử không hợp lệ".into());
                }
            }
        }
        _ => {}
    }
    Ok(())
}

fn string<'a>(object: &'a serde_json::Map<String, Value>, key: &str) -> Result<&'a str, String> {
    object
        .get(key)
        .and_then(Value::as_str)
        .ok_or_else(|| format!("{key} phải là chuỗi"))
}
fn slug(object: &serde_json::Map<String, Value>, key: &str, lane: Lane) -> Result<(), String> {
    let value = string(object, key)?;
    if value.is_empty()
        || !value
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
    {
        return Err(format!("{}.{} phải là slug", lane.key(), key));
    }
    Ok(())
}
fn string_array(value: &Value) -> bool {
    value
        .as_array()
        .is_some_and(|a| a.iter().all(Value::is_string))
}
fn validate_records(
    object: &serde_json::Map<String, Value>,
    field: &str,
    required: &[&str],
    arrays: &[&str],
) -> Result<(), String> {
    if let Some(items) = object.get(field).and_then(Value::as_array) {
        for item in items {
            for key in required {
                if item.get(*key).is_none_or(Value::is_null) {
                    return Err(format!("{field}.{key} là bắt buộc"));
                }
            }
            for key in arrays {
                if item.get(*key).is_some_and(|v| !string_array(v)) {
                    return Err(format!("{field}.{key} phải là mảng chuỗi"));
                }
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
        if matches!(lane, Lane::ArcSmith | Lane::ChapterSmith) {
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
        }
        let reference = match lane {
            Lane::ArcSmith | Lane::ChapterSmith => row
                .get("code")
                .and_then(Value::as_str)
                .unwrap_or("")
                .to_owned(),
            Lane::CastSmith | Lane::WorldSmith => row
                .get("ref")
                .and_then(Value::as_str)
                .unwrap_or("")
                .to_owned(),
            Lane::BondSmith => format!(
                "{}:{}",
                row["from"].as_str().unwrap_or(""),
                row["to"].as_str().unwrap_or("")
            ),
        };
        if !codes.insert(reference) {
            return Err(format!("{prefix}.ref bị trùng"));
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
    fn test_cast_contract_accepts_and_rejects_payload() {
        let valid = json!({"characters":[{"ref":"linh","name":"Linh","role":"hero","archetype":"seeker","motivation":"find","want":"truth","need":"trust","flaw":"proud","summary":"s","description":"d","is_primary":true}]});
        assert!(validate(Lane::CastSmith, &valid).is_ok());
        let mut invalid = valid.clone();
        invalid["characters"][0]["is_primary"] = json!("yes");
        assert!(validate(Lane::CastSmith, &invalid).is_err());
        let mut unknown_motif = valid.clone();
        unknown_motif["characters"][0]["motifs"] = json!(["khong-co"]);
        let error = validate(Lane::CastSmith, &unknown_motif).unwrap_err();
        assert!(error.contains("khong-co"), "{error}");
    }

    #[test]
    fn test_world_contract_accepts_and_rejects_payload() {
        let valid = json!({"world":[{"ref":"city","kind":"setting","name":"Thành phố","type":"CITY","parent":null,"description":"d","access":"PUBLIC","spatial_features":"s","sensory":{"visual":"v","auditory":"a","tactile":"t","olfactory":"o","gustatory":"g"}}]});
        assert!(validate(Lane::WorldSmith, &valid).is_ok());
        let mut invalid = valid.clone();
        invalid["world"][0]["type"] = json!("MOON");
        assert!(validate(Lane::WorldSmith, &invalid).is_err());
    }

    #[test]
    fn test_bond_contract_accepts_and_rejects_payload() {
        let valid = json!({"relationships":[{"from":"linh","to":"minh","summary":"s","origin":"o","is_primary":true,"states":{"trust":"MODERATE","affection":"MODERATE","hostility":"NONE","fear":"NONE","respect":"MODERATE","familiarity":"MODERATE","dependency":"NONE"},"roles":[]}]});
        assert!(validate(Lane::BondSmith, &valid).is_ok());
        let mut invalid = valid.clone();
        invalid["relationships"][0]["states"]["trust"] = json!(101);
        assert!(validate(Lane::BondSmith, &invalid).is_err());
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
