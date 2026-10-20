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
    if matches!(stage, Stage::Cast | Stage::World) {
        return commit_cast_world(run_id, stage, keys);
    }
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

fn schema_rows(document: &Value, table: &str) -> Vec<Value> {
    rows_of(document, "optional", table).into_iter().collect()
}

fn stage_rows_path(table: &str) -> String {
    let group = if super_json::schema_table_group(table).is_some_and(|g| g.as_str() == "require") {
        "require"
    } else {
        "optional"
    };
    format!("{group}.schema.{table}")
}

fn commit_cast_world(
    run_id: &str,
    stage: Stage,
    keys: Option<&[String]>,
) -> Result<CommitReport, String> {
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
    if schema_rows(&document, "chapters").is_empty() {
        return Err("Cần chốt giai đoạn Chương trước".into());
    }
    let stored = db::list_items(&connection, run_id, stage.as_str())?;
    let mut selected = Vec::new();
    let mut skipped = 0;
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

    let mut characters = rows_of(&document, "require", "characters");
    let char_offset = characters.len();
    let mut refs = std::collections::HashMap::<String, usize>::new();
    let mut proposals = Vec::<(Lane, Value)>::new();
    for item in &selected {
        let lane = match item.lane.as_str() {
            "cast_smith" => Lane::CastSmith,
            "world_smith" => Lane::WorldSmith,
            "bond_smith" => Lane::BondSmith,
            other => return Err(format!("Lane không hợp lệ: {other}")),
        };
        let value: Value = serde_json::from_str(&item.payload_json)
            .map_err(|error| format!("Đề xuất {} có JSON không hợp lệ: {error}", item.key))?;
        contract::validate_entity(lane, &value)?;
        if lane == Lane::CastSmith {
            let reference = value["ref"]
                .as_str()
                .ok_or("Thiếu ref nhân vật")?
                .to_owned();
            if refs
                .insert(
                    reference.clone(),
                    char_offset + characters.len() - char_offset,
                )
                .is_some()
            {
                return Err(format!("Tham chiếu nhân vật trùng: {reference}"));
            }
            let mut row = serde_json::Map::new();
            for field in [
                "name",
                "is_primary",
                "role",
                "archetype",
                "motivation",
                "want",
                "need",
                "flaw",
                "summary",
                "description",
            ] {
                if let Some(value) = value.get(field) {
                    row.insert(field.into(), value.clone());
                }
            }
            characters.push(Value::Object(row));
        }
        proposals.push((lane, value));
    }
    // Character draft indices are assigned by the ordered character batch.
    for (index, row) in characters.iter().enumerate().skip(char_offset) {
        let _ = row;
        if let Some((_, proposal)) = proposals
            .iter()
            .filter(|(lane, _)| *lane == Lane::CastSmith)
            .nth(index - char_offset)
        {
            refs.insert(
                proposal["ref"].as_str().unwrap_or_default().to_owned(),
                index,
            );
        }
    }

    if stage == Stage::World {
        let mut statement = connection.prepare(
            "SELECT i.payload_json FROM outline_items i JOIN outline_runs r ON r.id=i.run_id WHERE r.project_id=?1 AND i.stage='cast' AND i.status='committed' ORDER BY r.created_at DESC, i.position, i.key"
        ).map_err(|error| format!("Không đọc được tham chiếu nhân vật: {error}"))?;
        let cast_payloads = statement
            .query_map([&project_id], |row| row.get::<_, String>(0))
            .map_err(|error| format!("Không đọc được tham chiếu nhân vật: {error}"))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| format!("Không đọc được tham chiếu nhân vật: {error}"))?;
        let mut seen = std::collections::BTreeSet::new();
        for payload in cast_payloads {
            let proposal: Value =
                serde_json::from_str(&payload).map_err(|error| error.to_string())?;
            if let Some(reference) = proposal["ref"].as_str()
                && seen.insert(reference.to_owned())
            {
                refs.insert(reference.to_owned(), seen.len() - 1);
            }
        }
    }
    let mut writes: Vec<(String, Vec<Value>)> = Vec::new();
    if stage == Stage::Cast {
        let mut appearances = Vec::new();
        let mut marks = Vec::new();

        let mut competences = Vec::new();
        let mut domains = Vec::new();
        let mut capabilities = Vec::new();
        let mut limitations = Vec::new();
        let mut memories = Vec::new();
        let mut knowledge = Vec::new();
        for (lane, proposal) in &proposals {
            if *lane != Lane::CastSmith {
                continue;
            }
            let index = refs[proposal["ref"].as_str().unwrap_or_default()];
            if let Some(appearance) = proposal.get("appearance").and_then(Value::as_object) {
                let mut row = json!({"character_id_json_index": index});
                for (source, target) in [
                    ("height", "height"),
                    ("height_unit", "height_unit"),
                    ("outfit_style", "outfit_style"),
                    ("hair", "hair_note"),
                    ("eyes", "eyes_note"),
                ] {
                    if let Some(value) = appearance.get(source) {
                        row[target] = value.clone();
                    }
                }
                appearances.push(row);
                for mark in appearance
                    .get("marks")
                    .and_then(Value::as_array)
                    .into_iter()
                    .flatten()
                {
                    marks.push(json!({"character_id_json_index":index,"content":mark}));
                }
            }
            // The document has no per-character motif table, so the codes ride
            // on the character row itself (still 81 writable paths).
            if let Some(motifs) = proposal["motifs"].as_array()
                && let Some(row) = characters.get_mut(index)
            {
                row["motifs"] = Value::Array(motifs.clone());
            }
            if let Some(items) = proposal["competences"].as_array() {
                for competence in items {
                    let domain = competence["domain"].as_str().unwrap_or("general");
                    let domain_index = domains
                        .iter()
                        .position(|r: &Value| r["name"] == domain)
                        .unwrap_or_else(|| {
                            domains.push(json!({
                                "code": domain.to_lowercase().replace(' ', "-"),
                                "name": domain,
                                "description": format!("Lĩnh vực {domain}"),
                                "is_system": false
                            }));
                            domains.len() - 1
                        });
                    let competence_index = competences.len();
                    competences.push(json!({"character_id_json_index":index,"domain_id_json_index":domain_index,"name":competence["name"],"level":competence["level"]}));
                    for content in competence["capabilities"].as_array().into_iter().flatten() {
                        capabilities.push(
                            json!({"competence_id_json_index":competence_index,"content":content}),
                        );
                    }
                    for content in competence["limitations"].as_array().into_iter().flatten() {
                        limitations.push(
                            json!({"competence_id_json_index":competence_index,"content":content}),
                        );
                    }
                }
            }
            if let Some(items) = proposal["memories"].as_array() {
                for memory in items {
                    memories.push(json!({"owner_id_json_index":index,"type":memory["type"],"summary":memory["summary"],"certainty":memory["certainty"],"knowledge_source":memory["source"],"significance":memory["significance"],"source_scene_id":null}));
                }
            }
            if let Some(items) = proposal["knowledge"].as_array() {
                for item in items {
                    knowledge.push(json!({"character_id_json_index":index,"type":item["type"],"path":item["path"],"certainty":item["certainty"]}));
                }
            }
        }
        writes.push(("characters".into(), characters));
        for (table, rows) in [
            ("character_appearances", appearances),
            ("character_distinguishing_marks", marks),
            ("competence_domains", domains),
            ("character_competences", competences),
            ("competence_capabilities", capabilities),
            ("competence_limitations", limitations),
            ("memories", memories),
            ("memory_emotions", Vec::new()),
            ("character_knowledge_sources", knowledge),
        ] {
            writes.push((table.into(), rows));
        }
    } else {
        let mut settings = Vec::new();
        let mut lore = Vec::new();
        let mut powers = Vec::new();
        let mut tiers = Vec::new();
        let mut abilities = Vec::new();
        let mut relationships = Vec::new();
        let mut relationship_motifs = Vec::new();
        let mut states = Vec::new();
        let mut roles = Vec::new();
        let mut role_categories = Vec::new();
        let existing_categories = schema_rows(&document, "relationship_categories");
        let mut setting_refs = std::collections::HashMap::<String, usize>::new();
        let existing_settings = schema_rows(&document, "world_settings");
        for (index, row) in existing_settings.iter().enumerate() {
            if let Some(code) = row["code"].as_str() {
                setting_refs.insert(code.to_owned(), index);
            }
        }
        let mut pending: Vec<&Value> = proposals
            .iter()
            .filter_map(|(_, p)| (p["kind"] == "setting").then_some(p))
            .collect();
        while !pending.is_empty() {
            let before = pending.len();
            let mut next = Vec::new();
            for p in pending {
                let parent = p["parent"].as_str();
                if parent.is_some_and(|reference| !setting_refs.contains_key(reference)) {
                    next.push(p);
                    continue;
                }
                let reference = p["ref"].as_str().unwrap_or_default().to_owned();
                let index = existing_settings.len() + settings.len();
                setting_refs.insert(reference, index);
                let sensory = &p["sensory"];
                let mut row = json!({"name":p["name"],"type":p["type"],"code":p["ref"],"description":p["description"],"access":p["access"],"spatial_features":p["spatial_features"],"sensory_visual":sensory["visual"],"sensory_auditory":sensory["auditory"],"sensory_tactile":sensory["tactile"],"sensory_olfactory":sensory["olfactory"],"sensory_gustatory":sensory["gustatory"]});
                if let Some(parent_index) = parent.and_then(|reference| setting_refs.get(reference))
                {
                    row["parent_id_json_index"] = json!(parent_index);
                }
                settings.push(row);
            }
            if next.len() == before {
                return Err("Không thể sắp xếp world_settings theo quan hệ cha-con".into());
            }
            pending = next;
        }
        for (_, p) in &proposals {
            match p["kind"].as_str().unwrap_or_default() {
                "lore" => {
                    let mut row = json!({"code":p["code"],"type":p["type"],"scope":p["scope"],"name":p["name"],"description":p["description"],"data":p["data"]});
                    if let Some(index) = p["world_setting"]
                        .as_str()
                        .and_then(|r| setting_refs.get(r))
                    {
                        row["world_setting_id_json_index"] = json!(index);
                    }
                    lore.push(row);
                }
                "power" => {
                    let power_index = powers.len();
                    powers.push(json!({"name":p["name"],"source":p["source"],"code":p["ref"],"scope":p["scope"],"description":p["description"],"rules":p["rules"],"cost":p["cost"],"limitations":p["limitations"]}));
                    if let Some(items) = p["tiers"].as_array() {
                        for tier in items {
                            let ti = tiers.len();
                            tiers.push(json!({"power_system_id_json_index":power_index,"code":tier["code"],"name":tier["name"],"rank":tier["rank"],"description":tier["description"]}));
                            for ability in p["abilities"]
                                .as_array()
                                .into_iter()
                                .flatten()
                                .filter(|a| a["tier"] == tier["code"])
                            {
                                abilities.push(json!({"power_system_id_json_index":power_index,"tier_id_json_index":ti,"name":ability["name"],"description":ability["description"]}));
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        let mut bond_values = Vec::new();
        for (lane, p) in &proposals {
            if *lane == Lane::BondSmith {
                bond_values.push(p.clone());
            }
        }
        for p in bond_values {
            let Some(from) = refs.get(p["from"].as_str().unwrap_or_default()) else {
                return Err(format!("Không tìm thấy nhân vật {}", p["from"]));
            };
            let Some(to) = refs.get(p["to"].as_str().unwrap_or_default()) else {
                return Err(format!("Không tìm thấy nhân vật {}", p["to"]));
            };
            let rel = relationships.len();
            relationships.push(json!({"from_character_id_json_index":from,"to_character_id_json_index":to,"summary":p["summary"],"origin":p["origin"],"is_primary":p["is_primary"]}));
            let mut state = json!({"parent_id_json_index":rel});
            for k in [
                "trust",
                "affection",
                "hostility",
                "fear",
                "respect",
                "familiarity",
                "dependency",
            ] {
                state[k] = p["states"][k].clone();
            }
            states.push(state);
            for motif in p["motifs"].as_array().into_iter().flatten() {
                relationship_motifs
                    .push(json!({"relationship_id_json_index":rel,"motif_id":motif}));
            }
            for role in p["roles"].as_array().into_iter().flatten() {
                let category = role["category"].as_str().unwrap_or_default();
                let sub_type = role["sub_type"].as_str().unwrap_or_default();
                let category_index = existing_categories
                    .iter()
                    .position(|row| row["category"] == category && row["sub_type"] == sub_type)
                    .or_else(|| {
                        role_categories
                            .iter()
                            .position(|row: &Value| {
                                row["category"] == category && row["sub_type"] == sub_type
                            })
                            .map(|index| existing_categories.len() + index)
                    })
                    .unwrap_or_else(|| {
                        role_categories.push(json!({"category":category,"sub_type":sub_type}));
                        existing_categories.len() + role_categories.len() - 1
                    });
                roles.push(json!({"relationship_id_json_index":rel,"category_id_json_index":category_index,"is_primary":role["is_primary"],"hierarchy":role["hierarchy"]}));
            }
        }
        writes.extend([
            ("world_settings".into(), settings),
            ("lore_entries".into(), lore),
            ("power_systems".into(), powers),
            ("power_tiers".into(), tiers),
            ("power_abilities".into(), abilities),
            ("relationships".into(), relationships),
            ("relationship_states".into(), states),
            ("relationship_categories".into(), role_categories),
            ("relationship_roles".into(), roles),
            ("relationship_motifs".into(), relationship_motifs),
        ]);
    }
    let entities = if stage == Stage::Cast {
        vec![
            Entity::Characters,
            Entity::Appearances,
            Entity::Competences,
            Entity::Memories,
            Entity::MemoryLinks,
        ]
    } else {
        vec![
            Entity::WorldSettings,
            Entity::LoreEntries,
            Entity::PowerSystems,
            Entity::Relationships,
            Entity::RelationshipStates,
            Entity::RelationshipRoles,
            Entity::RelationshipMotifs,
        ]
    };
    assert_order(&entities)?;
    let total = writes
        .iter()
        .find(|(t, _)| t == "characters")
        .map(|(_, r)| r.len())
        .unwrap_or_else(|| rows_of(&document, "require", "characters").len());
    if stage == Stage::Cast && total == 0 {
        return Err("Bảng characters bắt buộc phải có ít nhất một hàng".into());
    }
    let committed = selected.len();
    let mut warnings = Vec::new();
    for (table, incoming) in writes {
        super_json::schema_table_group(&table)
            .ok_or_else(|| format!("Không tìm thấy bảng {table}"))?;
        let rows = if table == "characters" {
            incoming
        } else {
            let mut existing = schema_rows(&document, &table);
            existing.extend(incoming);
            existing
        };
        let base = stage_rows_path(&table);
        let rows_path = format!("{base}.rows");
        let count_path = format!("{base}.count");
        guard_path(&rows_path)?;
        guard_path(&count_path)?;
        let count = rows.len();
        super_json::set(&project_id, &rows_path, json!(rows))?;
        super_json::set(&project_id, &count_path, json!(count))?;
    }
    if stage == Stage::World && selected.iter().all(|item| item.lane != "bond_smith") {
        warnings.push("Không có đề xuất mối quan hệ; relationships được giữ nguyên".into());
    }
    for item in &selected {
        db::set_item_status(&connection, run_id, stage.as_str(), &item.key, "committed")?;
    }
    db::update_run_status(&connection, run_id, "committed")?;
    Ok(CommitReport {
        committed,
        skipped,
        missing: Vec::new(),
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
