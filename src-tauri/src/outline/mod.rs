//! Outline generation engine for phase 2.
//!
//! One run per session, one proposal row per entity in `outline_items`, and a
//! fan-out that never touches the project state: only [`apply::commit`] writes,
//! and only the proposals the author accepted, in [`stage::STATE_ORDER`].

pub mod anchor;
pub mod apply;
pub mod contract;
pub mod fanout;
pub mod prompts;
pub mod seed;
pub mod stage;

pub use anchor::{anchor, anchor_compact, anchor_for_project, anchor_json};

use std::{
    collections::BTreeSet,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};

use serde_json::{Value, json};
use tokio::sync::mpsc;

use self::{
    contract::Lane,
    fanout::{Shard, ShardEvent},
    stage::Stage,
};
use crate::{
    db,
    models::{
        CommitReport, ModelConfig, OutlineEvent, OutlineItem, OutlineRunResult, OutlineStatus,
        ProviderConfig,
    },
    super_json,
};

/// Chapters per provider call: small enough to keep each prompt focused.
const CHAPTERS_PER_SHARD: u64 = 1;

/// One planned provider call. `shard` is the stable key used to store the
/// proposals it produced and to rebuild the very same call on regenerate.
struct ShardPlan {
    lane: Lane,
    shard: String,
    system: String,
    user: String,
    /// Set for the scene lane: the chapter this shard plans, injected into every
    /// proposal so the commit does not have to trust the model for it.
    chapter: Option<u64>,
}

pub fn start(project_id: &str, session_id: &str) -> Result<OutlineStatus, String> {
    if project_id.trim().is_empty() || session_id.trim().is_empty() {
        return Err("Cần mã dự án và phiên làm việc".into());
    }
    let connection = db::open()?;
    if !db::project_exists(&connection, project_id)? {
        return Err(format!("Không tìm thấy dự án: {project_id}"));
    }
    db::ensure_session(&connection, session_id, project_id)?;
    db::create_run(&connection, project_id, session_id)?;
    let _ = seed::seed_project(project_id)?;
    status(session_id)
}

pub fn status(session_id: &str) -> Result<OutlineStatus, String> {
    let connection = db::open()?;
    let Some(run) = db::run_for_session(&connection, session_id)? else {
        return Ok(OutlineStatus {
            ok: true,
            stage: Stage::Seed.as_str().into(),
            active: false,
            run_status: "idle".into(),
            stage_status: "empty".into(),
            counts: json!({"total":0,"pending":0,"accepted":0,"committed":0,"failed":0}),
            next_stage: Some(Stage::Arcs.as_str().into()),
            blockers: vec!["Chưa khởi tạo lượt dàn ý".into()],
            stages: Vec::new(),
            usage: json!({"calls":0,"tokens_in":0,"tokens_out":0}),
        });
    };
    let stage = Stage::parse(&run.stage).unwrap_or(Stage::Seed);
    let items = db::list_items(&connection, &run.id, stage.as_str())?;
    let total = items.len();
    let count = |wanted: &str| items.iter().filter(|item| item.status == wanted).count();
    let pending = count("pending");
    let accepted = count("accepted");
    let committed = count("committed");
    let failed = count("failed");

    let mut blockers = Vec::new();
    if total == 0 && stage != Stage::Seed {
        blockers.push(format!("Giai đoạn {} chưa có đề xuất", stage.label()));
    }
    if failed > 0 {
        blockers.push(format!("{failed} đề xuất lỗi cần chạy lại"));
    }
    if pending > 0 {
        blockers.push(format!("{pending} đề xuất chờ tác giả duyệt"));
    }
    let stage_status = if total == 0 {
        "empty"
    } else if committed == total {
        "committed"
    } else {
        "review"
    };

    Ok(OutlineStatus {
        ok: true,
        stage: stage.as_str().into(),
        active: run.status == "active" || run.status == "review",
        run_status: run.status,
        stage_status: stage_status.into(),
        counts: json!({
            "total": total,
            "pending": pending,
            "accepted": accepted,
            "committed": committed,
            "failed": failed,
        }),
        next_stage: stage.next().map(|next| next.as_str().to_owned()),
        blockers,
        stages: stage_states(&connection, &run.id)?,
        usage: json!({
            "calls": run.calls,
            "tokens_in": run.tokens_in,
            "tokens_out": run.tokens_out,
        }),
    })
}

/// Per-stage proposal counts for the rail. A stage is `committed` only when it
/// has proposals and every one of them reached `committed`.
fn stage_states(
    connection: &rusqlite::Connection,
    run_id: &str,
) -> Result<Vec<crate::models::OutlineStageState>, String> {
    let mut states = Vec::new();
    for stage in [
        Stage::Arcs,
        Stage::Chapters,
        Stage::Cast,
        Stage::World,
        Stage::Scenes,
    ] {
        let items = db::list_items(connection, run_id, stage.as_str())?;
        let total = items.len() as u32;
        let status = if total == 0 {
            "empty"
        } else if items.iter().all(|item| item.status == "committed") {
            "committed"
        } else {
            "review"
        };
        states.push(crate::models::OutlineStageState {
            stage: stage.as_str().to_owned(),
            status: status.to_owned(),
            total,
        });
    }
    Ok(states)
}

fn plan_counts(document: &Value) -> Result<(u64, u64, u64), String> {
    let read = |leaf: &str| {
        document
            .pointer(&format!("/require/project/plan/{leaf}/count"))
            .and_then(Value::as_u64)
            .filter(|value| *value > 0)
            .ok_or_else(|| format!("Thiếu require.project.plan.{leaf}.count"))
    };
    Ok((read("arc")?, read("chapter")?, read("scene")?))
}

fn arc_rows(document: &Value) -> Vec<Value> {
    document
        .pointer("/require/schema")
        .and_then(Value::as_array)
        .and_then(|entries| entries.iter().find(|entry| entry["table"] == "arcs"))
        .and_then(|entry| entry["rows"].as_array())
        .cloned()
        .unwrap_or_default()
}

fn arcs_brief(document: &Value) -> String {
    let rows = arc_rows(document);
    if rows.is_empty() {
        return "Chưa có cung truyện nào được chốt.".to_owned();
    }
    rows.iter()
        .enumerate()
        .map(|(index, row)| {
            format!(
                "{}. {}: {}",
                index + 1,
                row.get("name")
                    .and_then(Value::as_str)
                    .unwrap_or("(chưa đặt tên)"),
                row.get("goal")
                    .and_then(Value::as_str)
                    .or_else(|| row.get("summarize").and_then(Value::as_str))
                    .unwrap_or("(chưa có mục tiêu)")
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn arc_plan(document: &Value) -> Result<ShardPlan, String> {
    let (arc_total, chapter_total, scene_total) = plan_counts(document)?;
    Ok(ShardPlan {
        lane: Lane::ArcSmith,
        shard: "arcs".to_owned(),
        system: prompts::system(Lane::ArcSmith).to_owned(),
        chapter: None,
        user: prompts::arcs_user(
            &anchor_compact(document),
            arc_total,
            chapter_total,
            scene_total,
        ),
    })
}

fn chapter_plans(document: &Value, range: Option<&str>) -> Result<Vec<ShardPlan>, String> {
    let (_, chapter_total, scene_total) = plan_counts(document)?;
    let brief = arcs_brief(document);
    let compact = anchor_compact(document);
    let mut plans = Vec::new();
    for (start, end) in chapter_range(range, chapter_total)? {
        let index = (start - 1) / CHAPTERS_PER_SHARD;
        let chapter_span = end - start + 1;
        let scene_budget = (scene_total * chapter_span).div_ceil(chapter_total);
        plans.push(ShardPlan {
            lane: Lane::ChapterSmith,
            shard: format!("chapters-{:02}", index + 1),
            system: prompts::system(Lane::ChapterSmith).to_owned(),
            chapter: None,
            user: prompts::chapters_user(&compact, &brief, start, end, chapter_total, scene_budget),
        });
    }
    Ok(plans)
}

/// `range` accepts `1-10`, `1-30`, `all`, or empty for the whole plan. It snaps
/// to whole shards so a rerun never splits a batch in half.
fn chapter_range(range: Option<&str>, total: u64) -> Result<Vec<(u64, u64)>, String> {
    let text = range.unwrap_or("").trim();
    let (start, end) = if text.is_empty() || text.eq_ignore_ascii_case("all") {
        (1, total)
    } else if let Some((left, right)) = text.split_once('-') {
        let start: u64 = left
            .trim()
            .parse()
            .map_err(|_| format!("Khoảng chương không hợp lệ: {text}"))?;
        let end: u64 = right
            .trim()
            .parse()
            .map_err(|_| format!("Khoảng chương không hợp lệ: {text}"))?;
        (start, end)
    } else {
        return Err(format!("Khoảng chương không hợp lệ: {text}"));
    };
    if start == 0 || end < start || end > total {
        return Err(format!(
            "Khoảng chương phải nằm trong 1-{total} và tăng dần: {text}"
        ));
    }
    let mut bounds = Vec::new();
    let mut cursor = start;
    while cursor <= end {
        let shard_end = (cursor + CHAPTERS_PER_SHARD - 1).min(end);
        bounds.push((cursor, shard_end));
        cursor = shard_end + 1;
    }
    Ok(bounds)
}

fn chapter_brief(document: &Value) -> String {
    rows_of_document(document, "chapters")
        .iter()
        .enumerate()
        .map(|(i, row)| {
            format!(
                "{}. {}: {}",
                i + 1,
                row["name"].as_str().unwrap_or(""),
                row["summarize"].as_str().unwrap_or("")
            )
        })
        .collect::<Vec<_>>()
        .join("\\n")
}
fn rows_of_document(document: &Value, table: &str) -> Vec<Value> {
    ["require", "optional"]
        .into_iter()
        .find_map(|group| {
            document
                .get(group)?
                .get("schema")?
                .as_array()?
                .iter()
                .find(|entry| entry["table"] == table)?
                .get("rows")?
                .as_array()
                .cloned()
        })
        .unwrap_or_default()
}
fn cast_plans(document: &Value) -> Result<Vec<ShardPlan>, String> {
    let chapters = rows_of_document(document, "chapters");
    let total = 6usize.max(chapters.len() / 3);
    let count = total.div_ceil(6);
    let brief = chapter_brief(document);
    Ok((0..count)
        .map(|index| ShardPlan {
            lane: Lane::CastSmith,
            shard: format!("cast-{:02}", index + 1),
            system: prompts::system(Lane::CastSmith).into(),
            chapter: None,
            user: prompts::cast_user(
                &anchor_compact(document),
                &brief,
                &format!("{} characters", 6.min(total - index * 6)),
                &Vec::new(),
                &brief,
            ),
        })
        .collect())
}
fn world_plans(document: &Value) -> Vec<ShardPlan> {
    let brief = chapter_brief(document);
    let anchor = anchor_compact(document);
    let genre = anchor_json(document)["project"]["genre_code"]
        .as_str()
        .unwrap_or("")
        .to_owned();
    let mut plans: Vec<ShardPlan> = ["settings top down", "lore and power systems"]
        .iter()
        .enumerate()
        .map(|(i, task)| ShardPlan {
            lane: Lane::WorldSmith,
            shard: format!("world-{:02}", i + 1),
            system: prompts::system(Lane::WorldSmith).into(),
            chapter: None,
            user: format!(
                "{}\n\nPhạm vi: {task}.",
                prompts::world_user(&anchor, &brief, &genre)
            ),
        })
        .collect();
    let characters = rows_of_document(document, "characters");
    if !characters.is_empty() {
        let cast_brief = characters
            .iter()
            .enumerate()
            .map(|(i, row)| {
                format!(
                    "{}. {}: {}",
                    i,
                    row["name"].as_str().unwrap_or(""),
                    row["summary"].as_str().unwrap_or("")
                )
            })
            .collect::<Vec<_>>()
            .join("\\n");
        plans.push(ShardPlan {
            lane: Lane::BondSmith,
            shard: "bonds-01".into(),
            system: prompts::system(Lane::BondSmith).into(),
            chapter: None,
            user: prompts::bonds_user(&anchor, &cast_brief),
        });
    }
    plans
}

/// One shard per chapter: the binder needs the whole chapter to choose consistent
/// presets, and five chapters run in parallel.
fn scene_plans(document: &Value, range: Option<&str>) -> Result<Vec<ShardPlan>, String> {
    let chapters = rows_of_document(document, "chapters");
    if chapters.is_empty() {
        return Err("Cần chốt giai đoạn Chương trước".into());
    }
    let (_, _, scene_total) = plan_counts(document)?;
    let word_total = document
        .pointer("/require/project/plan/word/count")
        .and_then(Value::as_u64)
        .unwrap_or(0);
    let bounds = chapter_range(range, chapters.len() as u64)?;
    let anchor = anchor_compact(document);
    let cast_brief = rows_of_document(document, "characters")
        .iter()
        .map(|row| {
            format!(
                "- {}: {} / {}",
                row["name"].as_str().unwrap_or(""),
                row["role"].as_str().unwrap_or(""),
                row["summary"].as_str().unwrap_or("")
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    let settings_brief = rows_of_document(document, "world_settings")
        .iter()
        .map(|row| {
            format!(
                "- {}: {} / {}",
                row["name"].as_str().unwrap_or(""),
                row["type"].as_str().unwrap_or(""),
                row["description"].as_str().unwrap_or("")
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    let preset_codes = catalog_codes(crate::catalog::Kind::StylePresets);
    let narrator_codes = catalog_codes(crate::catalog::Kind::NarratorConfigs);
    let directive_codes = catalog_codes(crate::catalog::Kind::NarrativeDirectives);

    let mut plans = Vec::new();
    for (start, end) in bounds {
        for position in start..=end {
            let Some(chapter) = chapters.get((position - 1) as usize) else {
                continue;
            };
            let scene_budget = chapter["scene_count"].as_u64().unwrap_or(3).max(1);
            let word_budget = if word_total == 0 || scene_total == 0 {
                0
            } else {
                (word_total / scene_total.max(1)) * scene_budget
            };
            let chapter_brief = format!(
                "Chương {position}: {}\nTóm tắt: {}\nBeats: {}\nSố cảnh: {scene_budget}",
                chapter["name"].as_str().unwrap_or(""),
                chapter["summarize"].as_str().unwrap_or(""),
                chapter["beats"]
                    .as_array()
                    .map(|beats| beats
                        .iter()
                        .filter_map(Value::as_str)
                        .collect::<Vec<_>>()
                        .join("; "))
                    .unwrap_or_default(),
            );
            plans.push(ShardPlan {
                lane: Lane::SceneBinder,
                shard: format!("scenes-ch-{position:03}"),
                system: prompts::system(Lane::SceneBinder).to_owned(),
                chapter: Some(position),
                user: prompts::scenes_user(
                    &anchor,
                    &chapter_brief,
                    &cast_brief,
                    &settings_brief,
                    &preset_codes,
                    &narrator_codes,
                    &directive_codes,
                    scene_budget,
                    word_budget,
                ),
            });
        }
    }
    Ok(plans)
}

/// Compact `code: label` list for a catalogue, so the binder picks a real code.
fn catalog_codes(kind: crate::catalog::Kind) -> String {
    crate::catalog::all(kind)
        .iter()
        .map(|entry| format!("{} ({})", entry.code, entry.name))
        .collect::<Vec<_>>()
        .join("; ")
}

fn build_plan(
    stage: Stage,
    document: &Value,
    range: Option<&str>,
) -> Result<Vec<ShardPlan>, String> {
    match stage {
        Stage::Arcs => Ok(vec![arc_plan(document)?]),
        Stage::Chapters => chapter_plans(document, range),
        Stage::Cast => cast_plans(document),
        Stage::World => Ok(world_plans(document)),
        Stage::Scenes => scene_plans(document, range),
        other => Err(format!(
            "Giai đoạn {} chưa được dùng trong sprint này",
            other.label()
        )),
    }
}

/// Splits one lane result into one proposal per entity so the author can review,
/// regenerate and accept a single arc or chapter.
fn proposals_from(
    run_id: &str,
    stage: Stage,
    plan: &ShardPlan,
    value: &Value,
) -> Result<Vec<db::OutlineStoredItem>, String> {
    let array_key = plan.lane.array_key();
    let entries = value
        .get(array_key)
        .and_then(Value::as_array)
        .ok_or_else(|| format!("Thiếu mảng {array_key} trong đề xuất"))?;
    let mut items = Vec::new();
    for entry in entries {
        // The scene lane is planned per chapter, so the shard owns that number.
        let entry = if let Some(chapter) = plan.chapter {
            let mut object = entry
                .as_object()
                .cloned()
                .ok_or_else(|| "Đề xuất cảnh phải là object".to_owned())?;
            object.insert("chapter".to_owned(), json!(chapter));
            Value::Object(object)
        } else {
            entry.clone()
        };
        let entry = &entry;
        let bond_key;
        let key = if plan.lane == Lane::BondSmith {
            let from = entry
                .get("from")
                .and_then(Value::as_str)
                .ok_or("Mối quan hệ thiếu from")?;
            let to = entry
                .get("to")
                .and_then(Value::as_str)
                .ok_or("Mối quan hệ thiếu to")?;
            bond_key = format!("{from}-{to}");
            bond_key.as_str()
        } else {
            entry
                .get("code")
                .or_else(|| entry.get("ref"))
                .and_then(Value::as_str)
                .ok_or_else(|| format!("{array_key} có phần tử thiếu mã tham chiếu"))?
        };
        let position = entry.get("position").and_then(Value::as_u64).unwrap_or(0) as u32;
        items.push(db::OutlineStoredItem {
            id: format!("{run_id}-{}-{key}", stage.as_str()),
            run_id: run_id.to_owned(),
            stage: stage.as_str().to_owned(),
            lane: plan.lane.key().to_owned(),
            shard: plan.shard.clone(),
            key: key.to_owned(),
            position,
            payload_json: serde_json::to_string(entry)
                .map_err(|error| format!("Không lưu được đề xuất {key}: {error}"))?,
            status: "pending".to_owned(),
            error: None,
        });
    }
    Ok(items)
}

fn forward_events(sink: mpsc::UnboundedSender<OutlineEvent>) -> mpsc::UnboundedSender<ShardEvent> {
    let (sender, mut receiver) = mpsc::unbounded_channel();
    tokio::spawn(async move {
        while let Some(event) = receiver.recv().await {
            let forwarded = match event {
                ShardEvent::Started { key, lane } => OutlineEvent::ShardStarted {
                    key,
                    lane: lane.key().into(),
                },
                ShardEvent::Finished { key, lane, result } => OutlineEvent::ShardFinished {
                    key,
                    lane: lane.key().into(),
                    ok: result.is_ok(),
                    error: result.err(),
                },
            };
            let _ = sink.send(forwarded);
        }
    });
    sender
}

/// A stage counts as committed when it has at least one proposal and every one
/// of them reached `committed`. `seed` is committed by definition once the run
/// exists, because `start` performs it synchronously.
fn committed_stages(
    connection: &rusqlite::Connection,
    run_id: &str,
) -> Result<BTreeSet<Stage>, String> {
    let mut committed = BTreeSet::new();
    committed.insert(Stage::Seed);
    for stage in [
        Stage::Arcs,
        Stage::Chapters,
        Stage::Cast,
        Stage::World,
        Stage::Scenes,
    ] {
        let items = db::list_items(connection, run_id, stage.as_str())?;
        if !items.is_empty() && items.iter().all(|item| item.status == "committed") {
            committed.insert(stage);
        }
    }
    Ok(committed)
}

pub async fn run(
    session_id: &str,
    provider: &ProviderConfig,
    model: &ModelConfig,
    stage: Stage,
    range: Option<&str>,
    sink: mpsc::UnboundedSender<OutlineEvent>,
    cancel: Arc<AtomicBool>,
) -> Result<OutlineRunResult, String> {
    run_with_backend(
        Arc::new(crate::adapters::HttpBackend),
        session_id,
        provider,
        model,
        stage,
        range,
        sink,
        cancel,
    )
    .await
}

/// Same as [`run`] with an injected backend, which is how the tests drive the
/// whole pipeline through `ScriptedBackend` without a network. The argument
/// count mirrors the call it wraps: session, provider, model, stage, range,
/// event sink and cancel flag.
#[allow(clippy::too_many_arguments)]
pub async fn run_with_backend(
    backend: Arc<dyn crate::adapters::LlmBackend>,
    session_id: &str,
    provider: &ProviderConfig,
    model: &ModelConfig,
    stage: Stage,
    range: Option<&str>,
    sink: mpsc::UnboundedSender<OutlineEvent>,
    cancel: Arc<AtomicBool>,
) -> Result<OutlineRunResult, String> {
    let connection = db::open()?;
    let run = db::run_for_session(&connection, session_id)?.ok_or("Chưa khởi tạo lượt dàn ý")?;
    let document = super_json::load_or_init(&run.project_id)?;
    stage::gate(stage, &committed_stages(&connection, &run.id)?)?;
    let plans = build_plan(stage, &document, range)?;
    let shards: Vec<Shard> = plans
        .iter()
        .map(|plan| Shard {
            lane: plan.lane,
            key: plan.shard.clone(),
            system: plan.system.clone(),
            user: plan.user.clone(),
        })
        .collect();
    let _ = sink.send(OutlineEvent::RunStarted {
        stage: stage.as_str().into(),
        shards_total: plans.len() as u32,
    });

    // Process one shard at a time. Each successful shard is committed to the
    // proposal store before the next provider call begins, so partial work is
    // visible immediately and survives an app restart.
    let mut stats = fanout::RunStats::default();
    let mut stored = Vec::new();
    let mut first_error: Option<String> = None;
    for (index, (plan, shard)) in plans.iter().zip(shards).enumerate() {
        let _ = sink.send(OutlineEvent::ShardStarted {
            key: plan.shard.clone(),
            lane: plan.lane.key().into(),
        });
        let (shard_stats, results) = fanout::run_shards(
            Arc::clone(&backend),
            provider.clone(),
            model.clone(),
            vec![shard],
            1,
            Arc::clone(&cancel),
            mpsc::unbounded_channel().0,
        )
        .await;
        stats.calls += shard_stats.calls;
        stats.failures += shard_stats.failures;
        stats.tokens_in += shard_stats.tokens_in;
        stats.tokens_out += shard_stats.tokens_out;

        let result = results
            .into_iter()
            .next()
            .map(|(_, result)| result)
            .unwrap_or_else(|| Err("Đã hủy lượt chạy".to_owned()));
        let (ok, error) = match result {
            Ok(value) => match proposals_from(&run.id, stage, plan, &value) {
                Ok(items) => {
                    if let Err(error) = db::upsert_items(&connection, &items) {
                        stats.failures += 1;
                        if first_error.is_none() {
                            first_error = Some(error.clone());
                        }
                        (false, Some(error))
                    } else {
                        stored.extend(items);
                        (true, None)
                    }
                }
                Err(error) => {
                    stats.failures += 1;
                    if first_error.is_none() {
                        first_error = Some(error.clone());
                    }
                    (false, Some(error))
                }
            },
            Err(error) => {
                if first_error.is_none() {
                    first_error = Some(error.clone());
                }
                (false, Some(error))
            }
        };
        let _ = sink.send(OutlineEvent::ShardFinished {
            key: plan.shard.clone(),
            lane: plan.lane.key().into(),
            ok,
            error,
        });
        db::update_run_usage(
            &connection,
            &run.id,
            run.calls + stats.calls as i64,
            run.tokens_in + stats.tokens_in as i64,
            run.tokens_out + stats.tokens_out as i64,
        )?;
        if cancel.load(Ordering::Relaxed) || index + 1 == plans.len() {
            break;
        }
    }

    if stored.is_empty() {
        let cause = first_error.unwrap_or_else(|| "không rõ nguyên nhân".to_owned());
        let error = format!("Không có đề xuất hợp lệ nào được sinh ra: {cause}");
        let _ = sink.send(OutlineEvent::RunFailed {
            error: error.clone(),
        });
        return Err(error);
    }
    db::update_run_stage(&connection, &run.id, stage.as_str())?;
    db::update_run_status(&connection, &run.id, "review")?;
    let _ = sink.send(OutlineEvent::ItemsReady {
        stage: stage.as_str().into(),
        count: stored.len() as u32,
    });
    let _ = sink.send(OutlineEvent::RunFinished {
        stage: stage.as_str().into(),
        failures: stats.failures as u32,
    });
    Ok(OutlineRunResult {
        stage: stage.as_str().into(),
        shards: plans.len() as u32,
        calls: stats.calls as u32,
        items: stored.len() as u32,
        failures: stats.failures as u32,
        tokens_in: stats.tokens_in,
        tokens_out: stats.tokens_out,
    })
}

pub fn items(session_id: &str, stage: Stage) -> Result<Vec<OutlineItem>, String> {
    let connection = db::open()?;
    let run = db::run_for_session(&connection, session_id)?.ok_or("Chưa khởi tạo lượt dàn ý")?;
    db::list_items(&connection, &run.id, stage.as_str())?
        .into_iter()
        .map(|item| {
            let value: Value = serde_json::from_str(&item.payload_json)
                .map_err(|error| format!("Đề xuất {} không hợp lệ: {error}", item.key))?;
            let text = |field: &str| value.get(field).and_then(Value::as_str).map(str::to_owned);
            let number = |field: &str| value.get(field).and_then(Value::as_u64);
            Ok(OutlineItem {
                id: item.id,
                key: item.key,
                lane: item.lane,
                position: item.position,
                status: item.status,
                title: text("name").unwrap_or_default(),
                error: item.error,
                name: text("name"),
                goal: text("goal"),
                outcome: text("outcome"),
                summarize: text("summarize"),
                chapters: number("chapters"),
                beats: value.get("beats").and_then(Value::as_array).map(|beats| {
                    beats
                        .iter()
                        .filter_map(Value::as_str)
                        .map(str::to_owned)
                        .collect()
                }),
                arc: number("arc"),
                scene_count: number("scene_count"),
                payload: value.clone(),
            })
        })
        .collect()
}

pub async fn regenerate(
    session_id: &str,
    provider: &ProviderConfig,
    model: &ModelConfig,
    stage: Stage,
    key: &str,
    note: &str,
) -> Result<OutlineRunResult, String> {
    regenerate_with_backend(
        Arc::new(crate::adapters::HttpBackend),
        session_id,
        provider,
        model,
        stage,
        key,
        note,
    )
    .await
}

pub async fn regenerate_with_backend(
    backend: Arc<dyn crate::adapters::LlmBackend>,
    session_id: &str,
    provider: &ProviderConfig,
    model: &ModelConfig,
    stage: Stage,
    key: &str,
    note: &str,
) -> Result<OutlineRunResult, String> {
    let connection = db::open()?;
    let run = db::run_for_session(&connection, session_id)?.ok_or("Chưa khởi tạo lượt dàn ý")?;
    let stored = db::list_items(&connection, &run.id, stage.as_str())?;
    let target = stored
        .iter()
        .find(|item| item.key == key)
        .ok_or_else(|| format!("Không tìm thấy đề xuất {key}"))?;
    if target.status == "committed" {
        return Err("Đề xuất này đã được ghi vào state, không tạo lại được".into());
    }
    let shard_key = target.shard.clone();
    let document = super_json::load_or_init(&run.project_id)?;
    let plans = build_plan(stage, &document, None)?;
    let plan = plans
        .iter()
        .find(|plan| plan.shard == shard_key)
        .ok_or_else(|| format!("Không dựng lại được lô {shard_key}"))?;
    let shard = Shard {
        lane: plan.lane,
        key: plan.shard.clone(),
        system: plan.system.clone(),
        user: format!("{}\nGhi chú sửa của tác giả: {note}", plan.user),
    };
    let (stats, results) = fanout::run_shards(
        backend,
        provider.clone(),
        model.clone(),
        vec![shard],
        1,
        Arc::new(AtomicBool::new(false)),
        forward_events(mpsc::unbounded_channel().0),
    )
    .await;
    let mut produced = Vec::new();
    for (_, result) in results {
        if let Ok(value) = result {
            produced.extend(proposals_from(&run.id, stage, plan, &value)?);
        }
    }
    if produced.is_empty() {
        return Err("Lần tạo lại không trả về đề xuất hợp lệ".into());
    }
    db::delete_items_for_shard(&connection, &run.id, stage.as_str(), &shard_key)?;
    db::upsert_items(&connection, &produced)?;
    db::update_run_usage(
        &connection,
        &run.id,
        run.calls + stats.calls as i64,
        run.tokens_in + stats.tokens_in as i64,
        run.tokens_out + stats.tokens_out as i64,
    )?;
    Ok(OutlineRunResult {
        stage: stage.as_str().into(),
        shards: 1,
        calls: stats.calls as u32,
        items: produced.len() as u32,
        failures: stats.failures as u32,
        tokens_in: stats.tokens_in,
        tokens_out: stats.tokens_out,
    })
}

pub fn accept(session_id: &str, stage: Stage, keys: Option<&[String]>) -> Result<u32, String> {
    decide(session_id, stage, keys, "accepted")
}

pub fn reject(session_id: &str, stage: Stage, keys: Option<&[String]>) -> Result<u32, String> {
    decide(session_id, stage, keys, "rejected")
}

fn decide(
    session_id: &str,
    stage: Stage,
    keys: Option<&[String]>,
    status: &str,
) -> Result<u32, String> {
    let connection = db::open()?;
    let run = db::run_for_session(&connection, session_id)?.ok_or("Chưa khởi tạo lượt dàn ý")?;
    let items = db::list_items(&connection, &run.id, stage.as_str())?;
    let mut changed = 0u32;
    for item in items {
        let selected = keys.is_none_or(|wanted| {
            wanted
                .iter()
                .any(|identity| identity == &item.id || identity == &item.key)
        });
        if !selected || item.status == "committed" {
            continue;
        }
        db::set_item_status(&connection, &run.id, stage.as_str(), &item.key, status)?;
        changed += 1;
    }
    if changed == 0 {
        return Err("Không có đề xuất nào đổi trạng thái".into());
    }
    Ok(changed)
}

pub fn commit(
    session_id: &str,
    stage: Stage,
    keys: Option<&[String]>,
) -> Result<CommitReport, String> {
    let connection = db::open()?;
    let run = db::run_for_session(&connection, session_id)?.ok_or("Chưa khởi tạo lượt dàn ý")?;
    apply::commit(&run.id, stage, keys)
}

pub fn cancel(session_id: &str) -> Result<(), String> {
    let connection = db::open()?;
    let run = db::run_for_session(&connection, session_id)?.ok_or("Chưa khởi tạo lượt dàn ý")?;
    db::update_run_status(&connection, &run.id, "cancelled")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, atomic::AtomicBool};

    use serde_json::{Value, json};

    use super::*;
    use crate::{
        adapters::{AdapterReply, ScriptedBackend},
        db,
        models::{InputTypes, ModelConfig, ProviderConfig},
        test_support::TempHome,
        tools,
    };

    fn provider() -> ProviderConfig {
        ProviderConfig {
            id: "p-outline".into(),
            name: "Outline provider".into(),
            kind: "openai".into(),
            api_key: "test-key".into(),
            base_url: "https://example.invalid".into(),
            models: Vec::new(),
            default_model_id: "m-outline".into(),
        }
    }

    fn model() -> ModelConfig {
        ModelConfig {
            id: "m-outline".into(),
            name: "Outline model".into(),
            context_window: "8192".into(),
            max_tokens: "2048".into(),
            input_types: InputTypes {
                text: true,
                image: false,
            },
            reasoning_level: false,
        }
    }

    /// Creates a project whose plan is 3 arcs / 12 chapters / 36 scenes and
    /// returns `(project_id, session_id)`. The caller owns the `TempHome`.
    fn project_with_plan(tag: &str) -> (String, String) {
        let session = format!("sess-{tag}");
        let connection = db::open().unwrap();
        db::ensure_session(&connection, &session, "").unwrap();
        let created = tools::execute_tool(
            "project_create",
            &json!({ "name": "Tro Than" }),
            "",
            &session,
        )
        .unwrap();
        let project = created["project_id"].as_str().unwrap().to_owned();
        for (path, value) in [
            ("require.project.language", json!("VI")),
            ("require.project.plan.arc.count", json!(3)),
            ("require.project.plan.chapter.count", json!(12)),
            ("require.project.plan.scene.count", json!(36)),
        ] {
            super_json::set(&project, path, value).unwrap();
        }
        (project, session)
    }

    fn arcs_reply() -> String {
        json!({"arcs": [
            {"position":1,"code":"arc-01","name":"Mưa","goal":"Mở màn","outcome":"Mất nhà","summarize":"Bốn chương mở màn","chapters":4},
            {"position":2,"code":"arc-02","name":"Tro","goal":"Tìm đồng minh","outcome":"Phản bội","summarize":"Bốn chương giữa","chapters":4},
            {"position":3,"code":"arc-03","name":"Lửa","goal":"Đối đầu","outcome":"Tái thiết","summarize":"Bốn chương cuối","chapters":4}
        ]})
        .to_string()
    }

    fn chapters_reply(start: u64, count: u64) -> String {
        let chapters: Vec<Value> = (0..count)
            .map(|offset| {
                let position = start + offset;
                json!({
                    "arc": ((position - 1) / 4) + 1,
                    "position": position,
                    "code": format!("ch-{position:03}"),
                    "name": format!("Chương {position}"),
                    "summarize": "Tóm tắt chương",
                    "beats": ["mở", "biến cố", "kết"],
                    "scene_count": 3,
                    "goal": "Mục tiêu chương",
                    "outcome": "Kết quả chương",
                })
            })
            .collect();
        json!({ "chapters": chapters }).to_string()
    }

    fn drain(receiver: &mut mpsc::UnboundedReceiver<OutlineEvent>) -> Vec<OutlineEvent> {
        let mut events = Vec::new();
        while let Ok(event) = receiver.try_recv() {
            events.push(event);
        }
        events
    }

    struct PersistCheckingBackend {
        session_id: String,
        calls: std::sync::atomic::AtomicUsize,
    }

    impl crate::adapters::LlmBackend for PersistCheckingBackend {
        fn complete<'a>(
            &'a self,
            _request: crate::adapters::ChatRequest,
        ) -> crate::adapters::BoxedFuture<'a, Result<AdapterReply, String>> {
            Box::pin(async move {
                let call = self.calls.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                if call > 0 {
                    let connection = db::open().unwrap();
                    let run = db::run_for_session(&connection, &self.session_id)
                        .unwrap()
                        .unwrap();
                    assert_eq!(
                        db::list_items(&connection, &run.id, Stage::Chapters.as_str())
                            .unwrap()
                            .len(),
                        call,
                        "every earlier chapter must be persisted before the next call"
                    );
                }
                Ok(AdapterReply::text(chapters_reply(call as u64 + 1, 1)))
            })
        }
    }

    #[test]
    fn chapter_shards_run_sequentially_and_persist_each_draft_before_next_call() {
        let _home = TempHome::new("outline-sequential-persist");
        let (project, session) = project_with_plan("sequential");
        start(&project, &session).unwrap();
        tauri::async_runtime::block_on(run_with_backend(
            Arc::new(ScriptedBackend::new(vec![AdapterReply::text(arcs_reply())])),
            &session,
            &provider(),
            &model(),
            Stage::Arcs,
            None,
            mpsc::unbounded_channel().0,
            Arc::new(AtomicBool::new(false)),
        ))
        .unwrap();
        accept(&session, Stage::Arcs, None).unwrap();
        commit(&session, Stage::Arcs, None).unwrap();
        let backend = Arc::new(PersistCheckingBackend {
            session_id: session.clone(),
            calls: std::sync::atomic::AtomicUsize::new(0),
        });

        let result = tauri::async_runtime::block_on(run_with_backend(
            backend.clone(),
            &session,
            &provider(),
            &model(),
            Stage::Chapters,
            None,
            mpsc::unbounded_channel().0,
            Arc::new(AtomicBool::new(false)),
        ))
        .unwrap();

        assert_eq!(backend.calls.load(std::sync::atomic::Ordering::SeqCst), 12);
        assert_eq!(result.items, 12);
        let connection = db::open().unwrap();
        let run = db::run_for_session(&connection, &session).unwrap().unwrap();
        assert_eq!(
            db::list_items(&connection, &run.id, Stage::Chapters.as_str())
                .unwrap()
                .len(),
            12
        );
    }

    #[test]
    fn test_outline_engine_end_to_end_scripted() {
        let _home = TempHome::new("outline-e2e");
        let (project, session) = project_with_plan("e2e");
        let before = super_json::load_or_init(&project).unwrap();
        start(&project, &session).unwrap();

        let arcs_backend = Arc::new(ScriptedBackend::new(vec![AdapterReply::text(arcs_reply())]));
        let (sender, mut receiver) = mpsc::unbounded_channel();
        let result = tauri::async_runtime::block_on(run_with_backend(
            arcs_backend,
            &session,
            &provider(),
            &model(),
            Stage::Arcs,
            None,
            sender,
            Arc::new(AtomicBool::new(false)),
        ))
        .unwrap();
        assert_eq!(result.items, 3, "three arcs become three proposals");
        assert!(
            drain(&mut receiver)
                .iter()
                .any(|event| matches!(event, OutlineEvent::ItemsReady { .. }))
        );

        assert_eq!(accept(&session, Stage::Arcs, None).unwrap(), 3);
        let report = commit(&session, Stage::Arcs, None).unwrap();
        assert_eq!(report.committed, 3);
        assert!(report.missing.is_empty());

        let chapters_backend = Arc::new(ScriptedBackend::new(
            (1..=12)
                .map(|chapter| AdapterReply::text(chapters_reply(chapter, 1)))
                .collect(),
        ));
        let (sender, mut receiver) = mpsc::unbounded_channel();
        let result = tauri::async_runtime::block_on(run_with_backend(
            chapters_backend,
            &session,
            &provider(),
            &model(),
            Stage::Chapters,
            None,
            sender,
            Arc::new(AtomicBool::new(false)),
        ))
        .unwrap();
        assert_eq!(result.items, 12, "twelve chapters across twelve calls");
        assert_eq!(result.shards, 12);
        assert!(
            drain(&mut receiver)
                .iter()
                .any(|event| matches!(event, OutlineEvent::ShardFinished { .. }))
        );

        assert_eq!(accept(&session, Stage::Chapters, None).unwrap(), 12);
        let report = commit(&session, Stage::Chapters, None).unwrap();
        assert_eq!(report.committed, 12);
        assert!(report.missing.is_empty(), "{:?}", report.missing);

        let after = super_json::load_or_init(&project).unwrap();
        assert_eq!(
            after["require"]["project"], before["require"]["project"],
            "phase 2 never rewrites the author's project leaves"
        );
        let arcs = after["require"]["schema"]
            .as_array()
            .unwrap()
            .iter()
            .find(|entry| entry["table"] == "arcs")
            .unwrap();
        assert_eq!(arcs["count"], json!(3));
        assert_eq!(arcs["rows"].as_array().unwrap().len(), 3);
        let chapters = after["optional"]["schema"]
            .as_array()
            .unwrap()
            .iter()
            .find(|entry| entry["table"] == "chapters")
            .unwrap();
        assert_eq!(chapters["rows"].as_array().unwrap().len(), 12);
        assert_eq!(chapters["count"], json!(12));
        let scene_sum: u64 = chapters["rows"]
            .as_array()
            .unwrap()
            .iter()
            .map(|row| row["scene_count"].as_u64().unwrap())
            .sum();
        assert_eq!(scene_sum, 36);
        assert_eq!(
            chapters["rows"][0]["arc_id_json_index"],
            json!(0),
            "the arc number becomes a draft reference"
        );
    }

    /// Runs and commits arcs + chapters so a cast/world test starts from a
    /// document that satisfies the 2.2 gate.
    fn prepare_through_chapters(project: &str, session: &str) {
        start(project, session).unwrap();
        tauri::async_runtime::block_on(run_with_backend(
            Arc::new(ScriptedBackend::new(vec![AdapterReply::text(arcs_reply())])),
            session,
            &provider(),
            &model(),
            Stage::Arcs,
            None,
            mpsc::unbounded_channel().0,
            Arc::new(AtomicBool::new(false)),
        ))
        .unwrap();
        accept(session, Stage::Arcs, None).unwrap();
        commit(session, Stage::Arcs, None).unwrap();

        tauri::async_runtime::block_on(run_with_backend(
            Arc::new(ScriptedBackend::new(
                (1..=12)
                    .map(|chapter| AdapterReply::text(chapters_reply(chapter, 1)))
                    .collect(),
            )),
            session,
            &provider(),
            &model(),
            Stage::Chapters,
            None,
            mpsc::unbounded_channel().0,
            Arc::new(AtomicBool::new(false)),
        ))
        .unwrap();
        accept(session, Stage::Chapters, None).unwrap();
        commit(session, Stage::Chapters, None).unwrap();
    }

    fn cast_reply() -> String {
        json!({"characters": [
            {
                "ref": "char-a", "name": "An", "role": "protagonist", "archetype": "seeker",
                "motivation": "Đòi lại nhà", "want": "Công lý", "need": "Học cách tin người",
                "flaw": "Cứng đầu", "summary": "Thợ rèn trẻ mất nhà", "description": "Gầy, mắt nâu",
                "is_primary": true,
                "appearance": {"height": 170, "height_unit": "CM", "hair": "đen", "eyes": "nâu",
                    "outfit_style": "áo vải", "marks": ["sẹo tay trái"]},
                "motifs": ["bildung_through_experience"],
                "competences": [{"domain": "kiếm", "name": "Kiếm pháp", "level": "cao",
                    "capabilities": ["chém nhanh"], "limitations": ["mất thăng bằng"]}],
                "memories": [{"type": "EPISODIC", "summary": "Mất nhà", "certainty": "CERTAIN",
                    "source": "FIRSTHAND", "significance": 0.8}],
                "knowledge": [{"type": "LORE", "path": "lore:den-bao", "certainty": "HIGH"}]
            },
            {
                "ref": "char-b", "name": "Bình", "role": "ally", "archetype": "guardian",
                "motivation": "Bảo vệ em gái", "want": "Yên ổn", "need": "Dám nói không",
                "flaw": "Sợ xung đột", "summary": "Lính gác làng", "description": "Cao, tóc ngắn",
                "is_primary": false,
                "competences": [{"domain": "khiên", "name": "Thủ khiên", "level": "trung bình",
                    "capabilities": ["chắn đòn"], "limitations": ["chậm"]}]
            }
        ]})
        .to_string()
    }

    fn world_reply() -> String {
        json!({"world": [
            {"ref": "ws-02", "kind": "setting", "name": "Nhà An", "type": "ROOM", "parent": "ws-01",
             "description": "Căn bếp nhỏ", "access": "PRIVATE", "spatial_features": "hẹp",
             "sensory": {"visual": "đèn dầu", "auditory": "mưa", "tactile": "lạnh",
                 "olfactory": "khói", "gustatory": "nhạt"}},
            {"ref": "ws-01", "kind": "setting", "name": "Làng Tro", "type": "TOWN", "parent": null,
             "description": "Làng ven sông", "access": "PUBLIC", "spatial_features": "dốc",
             "sensory": {"visual": "xám", "auditory": "gió", "tactile": "ẩm",
                 "olfactory": "tro", "gustatory": "mặn"}},
            {"ref": "lore-01", "kind": "lore", "code": "den-bao", "type": "ITEM", "scope": "project",
             "name": "Đèn bão", "description": "Vật gia truyền", "world_setting": "ws-01"},
            {"ref": "pw-01", "kind": "power", "name": "Linh khí", "source": "Di vật",
             "scope": "project", "description": "Năng lượng cổ", "rules": "Cần hơi thở đều",
             "cost": "Mỏi", "limitations": "Không dùng ban ngày",
             "tiers": [{"code": "t1", "name": "Sơ", "rank": 1, "description": "Mới thức tỉnh"}],
             "abilities": [{"tier": "t1", "name": "Đốt", "description": "Bén lửa"}]}
        ]})
        .to_string()
    }

    fn bonds_reply() -> String {
        json!({"relationships": [{
            "from": "char-a", "to": "char-b", "summary": "Đồng minh chớm nở",
            "origin": "Cùng giữ làng", "is_primary": true,
            "states": {"trust": "MODERATE", "affection": "LOW", "hostility": "LOW", "fear": "NONE",
                "respect": "MODERATE", "familiarity": "HIGH", "dependency": "NONE"},
            "roles": [{"category": "SOCIAL", "sub_type": "friend", "is_primary": true,
                "hierarchy": "EQUAL"}],
            "motifs": ["bildung_through_experience"]
        }]})
        .to_string()
    }

    /// Answers by lane instead of by call order: the world stage runs its three
    /// shards concurrently, so a scripted queue would be flaky.
    struct LaneBackend;

    impl crate::adapters::LlmBackend for LaneBackend {
        fn complete<'a>(
            &'a self,
            request: crate::adapters::ChatRequest,
        ) -> crate::adapters::BoxedFuture<'a, Result<AdapterReply, String>> {
            Box::pin(async move {
                let system = request.system.clone();
                let body = if system.contains("dàn nhân vật") {
                    cast_reply()
                } else if system.contains("mối quan hệ") {
                    bonds_reply()
                } else if system.contains("chia cảnh") {
                    scenes_reply()
                } else if system.contains("thế giới") || system.contains("địa điểm") {
                    world_reply()
                } else {
                    return Err(format!("unexpected lane prompt: {system}"));
                };
                Ok(AdapterReply::text(body))
            })
        }
    }

    fn scenes_reply() -> String {
        json!({"scenes": [
            {
                "position": 1, "code": "scene-001", "name": "Tro tàn",
                "summarize": "An trở về làng trong mưa. Bình chặn đường. Cảnh kết ở cánh cửa đóng.",
                "world_setting": "Làng Tro", "style_preset_code": "action.battle",
                "narrator_code": "first_person_protagonist", "target_words": 900,
                "directives": [
                    {"code": "show_dont_tell", "weight": "HIGH", "instruction": "Cho thấy cái lạnh qua bàn tay"},
                    {"code": "clarity_first", "weight": "MODERATE", "instruction": "Câu ngắn, rõ chủ thể"}
                ],
                "rules": ["Không miêu tả quá hai giác quan mỗi đoạn", "Không dùng lời thoại dài"],
                "cast": [{"character": "An", "is_pov": true}, {"character": "Bình", "is_pov": false}],
                "relationships": [{"a": "An", "b": "Bình", "beat": "Chạm mặt ở cổng, chưa ai nói"}]
            },
            {
                "position": 2, "code": "scene-002", "name": "Bếp lạnh",
                "summarize": "Hai người vào bếp. Bình đặt đèn bão xuống. An nhận ra món nợ cũ.",
                "world_setting": "Nhà An", "style_preset_code": "action.battle",
                "narrator_code": "first_person_protagonist", "target_words": 800,
                "directives": [
                    {"code": "show_dont_tell", "weight": "MODERATE", "instruction": "Để im lặng làm việc"}
                ],
                "rules": ["Giữ nhịp chậm", "Không giải thích quá khứ"],
                "cast": [{"character": "An", "is_pov": true}, {"character": "Bình", "is_pov": false}]
            },
            {
                "position": 3, "code": "scene-003", "name": "Lời hẹn",
                "summarize": "An nhận lời giúp Bình. Cảnh kết khi trời sáng.",
                "world_setting": "Làng Tro", "style_preset_code": "action.battle",
                "narrator_code": "first_person_protagonist", "target_words": 700,
                "directives": [
                    {"code": "technique_rotation", "weight": "LOW", "instruction": "Đổi kỹ thuật giữa các đoạn"}
                ],
                "rules": ["Không kết bằng câu hỏi"],
                "cast": [{"character": "An", "is_pov": true}, {"character": "Bình", "is_pov": false}]
            }
        ]})
        .to_string()
    }

    fn run_stage_with(session: &str, stage: Stage, backend: Arc<dyn crate::adapters::LlmBackend>) {
        tauri::async_runtime::block_on(run_with_backend(
            backend,
            session,
            &provider(),
            &model(),
            stage,
            None,
            mpsc::unbounded_channel().0,
            Arc::new(AtomicBool::new(false)),
        ))
        .unwrap();
    }

    fn run_stage(session: &str, stage: Stage, payload: String) {
        tauri::async_runtime::block_on(run_with_backend(
            Arc::new(ScriptedBackend::new(vec![AdapterReply::text(payload)])),
            session,
            &provider(),
            &model(),
            stage,
            None,
            mpsc::unbounded_channel().0,
            Arc::new(AtomicBool::new(false)),
        ))
        .unwrap();
    }

    fn table_rows<'a>(document: &'a Value, table: &str) -> &'a Vec<Value> {
        for group in ["require", "optional"] {
            if let Some(entry) = document[group]["schema"]
                .as_array()
                .and_then(|entries| entries.iter().find(|entry| entry["table"] == table))
            {
                return entry["rows"].as_array().unwrap();
            }
        }
        panic!("table {table} missing");
    }

    #[test]
    fn test_stage_23_run_stores_one_proposal_per_scene() {
        let _home = TempHome::new("outline-23-run");
        let (project, session) = project_with_plan("run23");
        prepare_through_chapters(&project, &session);
        run_stage_with(&session, Stage::Cast, Arc::new(LaneBackend));
        accept(&session, Stage::Cast, None).unwrap();
        commit(&session, Stage::Cast, None).unwrap();
        run_stage_with(&session, Stage::World, Arc::new(LaneBackend));
        accept(&session, Stage::World, None).unwrap();
        commit(&session, Stage::World, None).unwrap();

        // Only chapter 1, so the run is a single shard.
        tauri::async_runtime::block_on(run_with_backend(
            Arc::new(LaneBackend),
            &session,
            &provider(),
            &model(),
            Stage::Scenes,
            Some("1-1"),
            mpsc::unbounded_channel().0,
            Arc::new(AtomicBool::new(false)),
        ))
        .unwrap();

        let items = items(&session, Stage::Scenes).unwrap();
        assert_eq!(items.len(), 3, "three scenes for one chapter");
        assert_eq!(items[0].payload["chapter"], json!(1));
        assert_eq!(
            items[0].payload["style_preset_code"],
            json!("action.battle")
        );
    }

    #[test]
    fn test_stage_23_commit_writes_scenes_and_cast() {
        let _home = TempHome::new("outline-23-commit");
        let (project, session) = project_with_plan("commit23");
        prepare_through_chapters(&project, &session);
        run_stage_with(&session, Stage::Cast, Arc::new(LaneBackend));
        accept(&session, Stage::Cast, None).unwrap();
        commit(&session, Stage::Cast, None).unwrap();
        run_stage_with(&session, Stage::World, Arc::new(LaneBackend));
        accept(&session, Stage::World, None).unwrap();
        commit(&session, Stage::World, None).unwrap();

        tauri::async_runtime::block_on(run_with_backend(
            Arc::new(LaneBackend),
            &session,
            &provider(),
            &model(),
            Stage::Scenes,
            Some("1-1"),
            mpsc::unbounded_channel().0,
            Arc::new(AtomicBool::new(false)),
        ))
        .unwrap();
        accept(&session, Stage::Scenes, None).unwrap();
        let report = commit(&session, Stage::Scenes, None).unwrap();
        assert_eq!(report.committed, 3);

        let document = super_json::load_or_init(&project).unwrap();
        let scenes = table_rows(&document, "scenes");
        assert_eq!(scenes.len(), 3);
        assert_eq!(scenes[0]["position"], json!(1));
        assert_eq!(scenes[0]["name"], json!("Tro tàn"));
        assert_eq!(scenes[0]["chapter_id_json_index"], json!(0));
        assert_eq!(scenes[0]["status"], json!("PLANNED"));
        assert_eq!(scenes[0]["style_preset_code"], json!("action.battle"));
        assert_eq!(
            scenes[0]["narrator_code"],
            json!("first_person_protagonist")
        );
        assert_eq!(
            scenes[0]["content"],
            Value::Null,
            "prose arrives in phase 3"
        );
        assert!(scenes[0]["directives"].as_array().unwrap().len() == 2);
        assert!(scenes[0]["rules"].as_array().unwrap().len() == 2);
        let setting = scenes[0]["world_setting_id_json_index"].as_u64().unwrap();
        assert!(setting < table_rows(&document, "world_settings").len() as u64);

        let cast = table_rows(&document, "scene_characters");
        assert_eq!(cast.len(), 6, "two characters in each of the three scenes");
        let pov: Vec<&Value> = cast
            .iter()
            .filter(|row| row["is_pov"] == json!(true))
            .collect();
        assert_eq!(pov.len(), 3, "one point of view per scene");
        assert_eq!(pov[0]["character_id_json_index"], json!(0));
    }

    #[test]
    fn test_stage_23_rejects_preset_code_not_in_catalog() {
        let mut scenes: Value = serde_json::from_str(&scenes_reply()).unwrap();
        scenes["scenes"][0]["style_preset_code"] = json!("khong-co-preset");
        let error = contract::validate(Lane::SceneBinder, &scenes).unwrap_err();
        assert!(error.contains("khong-co-preset"), "{error}");
    }

    #[test]
    fn test_stage_23_requires_exactly_one_pov() {
        let mut scenes: Value = serde_json::from_str(&scenes_reply()).unwrap();
        scenes["scenes"][0]["cast"][1]["is_pov"] = json!(true);
        let error = contract::validate(Lane::SceneBinder, &scenes).unwrap_err();
        assert!(
            error.contains("is_pov") || error.contains("người kể"),
            "{error}"
        );

        let mut nobody: Value = serde_json::from_str(&scenes_reply()).unwrap();
        nobody["scenes"][0]["cast"][0]["is_pov"] = json!(false);
        assert!(contract::validate(Lane::SceneBinder, &nobody).is_err());
    }

    #[test]
    fn test_stage_23_rejects_unknown_directive_code() {
        let mut scenes: Value = serde_json::from_str(&scenes_reply()).unwrap();
        scenes["scenes"][0]["directives"][0]["code"] = json!("khong-co-directive");
        let error = contract::validate(Lane::SceneBinder, &scenes).unwrap_err();
        assert!(error.contains("khong-co-directive"), "{error}");
    }

    #[test]
    fn test_stage_22_cast_and_world_store_proposals() {
        let _home = TempHome::new("outline-22-run");
        let (project, session) = project_with_plan("run22");
        prepare_through_chapters(&project, &session);

        run_stage(&session, Stage::Cast, cast_reply());
        let cast_items = items(&session, Stage::Cast).unwrap();
        assert_eq!(cast_items.len(), 2, "one proposal per character");

        run_stage(&session, Stage::World, world_reply());
        let world_items = items(&session, Stage::World).unwrap();
        assert_eq!(world_items.len(), 4, "two settings, one lore, one power");

        let status = status(&session).unwrap();
        let cast_state = status
            .stages
            .iter()
            .find(|entry| entry.stage == "cast")
            .unwrap();
        assert_eq!(cast_state.status, "review");
        assert_eq!(cast_state.total, 2);
    }

    #[test]
    fn test_stage_22_commit_writes_characters_then_relationships() {
        let _home = TempHome::new("outline-22-commit");
        let (project, session) = project_with_plan("commit22");
        prepare_through_chapters(&project, &session);

        run_stage_with(&session, Stage::Cast, Arc::new(LaneBackend));
        accept(&session, Stage::Cast, None).unwrap();
        let cast_report = commit(&session, Stage::Cast, None).unwrap();
        assert_eq!(cast_report.committed, 2);

        run_stage_with(&session, Stage::World, Arc::new(LaneBackend));
        accept(&session, Stage::World, None).unwrap();
        commit(&session, Stage::World, None).unwrap();

        let document = super_json::load_or_init(&project).unwrap();
        let characters = table_rows(&document, "characters");
        assert_eq!(characters.len(), 2);
        assert_eq!(characters[0]["name"], json!("An"));
        assert_eq!(characters[0]["is_primary"], json!(true));

        let relationships = table_rows(&document, "relationships");
        assert_eq!(relationships.len(), 1);
        let from = relationships[0]["from_character_id_json_index"]
            .as_u64()
            .unwrap();
        let to = relationships[0]["to_character_id_json_index"]
            .as_u64()
            .unwrap();
        assert!(from < characters.len() as u64 && to < characters.len() as u64);
        assert_ne!(from, to);

        let states = table_rows(&document, "relationship_states");
        assert_eq!(states.len(), 1);
        assert_eq!(states[0]["trust"], json!("MODERATE"));

        // The document has no per-character motif table; the codes ride on the
        // character row and stay addressable from the review board.
        assert_eq!(
            characters[0]["motifs"],
            json!(["bildung_through_experience"])
        );
    }

    #[test]
    fn test_stage_22_memories_commit_with_null_source_scene() {
        let _home = TempHome::new("outline-22-memory");
        let (project, session) = project_with_plan("memory22");
        prepare_through_chapters(&project, &session);
        run_stage(&session, Stage::Cast, cast_reply());
        accept(&session, Stage::Cast, None).unwrap();
        commit(&session, Stage::Cast, None).unwrap();

        let document = super_json::load_or_init(&project).unwrap();
        let memories = table_rows(&document, "memories");
        assert_eq!(memories.len(), 1);
        assert_eq!(memories[0]["source_scene_id"], Value::Null);
        assert_eq!(memories[0]["owner_id_json_index"], json!(0));
    }

    #[test]
    fn test_stage_22_world_settings_parents_come_first() {
        let _home = TempHome::new("outline-22-settings");
        let (project, session) = project_with_plan("settings22");
        prepare_through_chapters(&project, &session);
        run_stage_with(&session, Stage::Cast, Arc::new(LaneBackend));
        accept(&session, Stage::Cast, None).unwrap();
        commit(&session, Stage::Cast, None).unwrap();

        // The child (ws-02) arrives before its parent (ws-01) on purpose.
        run_stage_with(&session, Stage::World, Arc::new(LaneBackend));
        accept(&session, Stage::World, None).unwrap();
        commit(&session, Stage::World, None).unwrap();

        let document = super_json::load_or_init(&project).unwrap();
        let settings = table_rows(&document, "world_settings");
        let parent = settings
            .iter()
            .position(|row| row["name"] == json!("Làng Tro"))
            .unwrap();
        let child = settings
            .iter()
            .position(|row| row["name"] == json!("Nhà An"))
            .unwrap();
        assert!(parent < child, "parent must be written before its child");
        assert_eq!(
            settings[child]["parent_id_json_index"],
            json!(parent as u64)
        );

        let lore = table_rows(&document, "lore_entries");
        assert_eq!(lore.len(), 1);
        let powers = table_rows(&document, "power_systems");
        assert_eq!(powers.len(), 1);
        let tiers = table_rows(&document, "power_tiers");
        assert_eq!(tiers.len(), 1);
        let abilities = table_rows(&document, "power_abilities");
        assert_eq!(abilities.len(), 1);
        assert_eq!(abilities[0]["tier_id_json_index"], json!(0));
    }

    #[test]
    fn test_stage_22_rejects_unknown_motif_code() {
        let _home = TempHome::new("outline-22-motif");
        let (project, session) = project_with_plan("motif22");
        prepare_through_chapters(&project, &session);
        let payload = json!({"characters": [{
            "ref": "char-x", "name": "X", "role": "r", "archetype": "a", "motivation": "m",
            "want": "w", "need": "n", "flaw": "f", "summary": "s", "description": "d",
            "is_primary": true, "motifs": ["khong-co-motif-nay"]
        }]})
        .to_string();
        let error = tauri::async_runtime::block_on(run_with_backend(
            Arc::new(ScriptedBackend::new(vec![
                AdapterReply::text(payload.clone()),
                AdapterReply::text(payload.clone()),
                AdapterReply::text(payload),
            ])),
            &session,
            &provider(),
            &model(),
            Stage::Cast,
            None,
            mpsc::unbounded_channel().0,
            Arc::new(AtomicBool::new(false)),
        ))
        .unwrap_err();
        assert!(error.contains("khong-co-motif-nay"), "{error}");
    }

    #[test]
    fn test_outline_items_expose_proposal_content_for_the_board() {
        let _home = TempHome::new("outline-items-board");
        let (project, session) = project_with_plan("board");
        start(&project, &session).unwrap();
        tauri::async_runtime::block_on(run_with_backend(
            Arc::new(ScriptedBackend::new(vec![AdapterReply::text(arcs_reply())])),
            &session,
            &provider(),
            &model(),
            Stage::Arcs,
            None,
            mpsc::unbounded_channel().0,
            Arc::new(AtomicBool::new(false)),
        ))
        .unwrap();

        let items = items(&session, Stage::Arcs).unwrap();
        assert_eq!(items.len(), 3);
        let first = &items[0];
        assert_eq!(first.key, "arc-01");
        assert_eq!(first.name.as_deref(), Some("Mưa"));
        assert_eq!(first.goal.as_deref(), Some("Mở màn"));
        assert_eq!(first.outcome.as_deref(), Some("Mất nhà"));
        assert_eq!(first.summarize.as_deref(), Some("Bốn chương mở màn"));
        assert_eq!(first.chapters, Some(4));
        assert_eq!(first.position, 1);
        assert!(first.beats.is_none(), "arcs carry no beats");
    }

    #[test]
    fn commit_accepted_items_uses_latest_run_and_preserves_acceptance() {
        let _home = TempHome::new("outline-commit-accepted-latest");
        let (project, session) = project_with_plan("accepted-latest");
        start(&project, &session).unwrap();
        tauri::async_runtime::block_on(run_with_backend(
            Arc::new(ScriptedBackend::new(vec![AdapterReply::text(arcs_reply())])),
            &session,
            &provider(),
            &model(),
            Stage::Arcs,
            None,
            mpsc::unbounded_channel().0,
            Arc::new(AtomicBool::new(false)),
        ))
        .unwrap();
        let accepted = accept(&session, Stage::Arcs, None).unwrap();
        assert_eq!(accepted, 3);

        // A second outline start currently returns the existing run but updates
        // its timestamp. Make a second run too, to exercise historical-session
        // rows while commit resolves the run internally.
        let connection = db::open().unwrap();
        let first_run = db::run_for_session(&connection, &session).unwrap().unwrap();
        let second_run_id = format!("{}-new", first_run.id);
        connection.execute(
            "INSERT INTO outline_runs (id, project_id, session_id, stage, status, created_at, updated_at) VALUES (?1, ?2, ?3, 'arcs', 'review', '2099-01-01', '2099-01-01')",
            rusqlite::params![second_run_id, project, session],
        ).unwrap();
        let mut copied = db::list_items(&connection, &first_run.id, Stage::Arcs.as_str()).unwrap();
        for item in &mut copied {
            item.id = format!("{}-copy", item.id);
            item.run_id = second_run_id.clone();
            item.status = "pending".into();
        }
        db::upsert_items(&connection, &copied).unwrap();

        let report = apply::commit(&first_run.id, Stage::Arcs, None).unwrap();
        assert_eq!(report.committed, 3);
        let current =
            db::list_items(&db::open().unwrap(), &first_run.id, Stage::Arcs.as_str()).unwrap();
        assert!(current.iter().all(|item| item.status == "committed"));
    }

    #[test]
    fn cast_and_world_proposals_with_same_key_remain_independent_and_commit() {
        let _home = TempHome::new("outline-cast-world-key-collision");
        let (project, session) = project_with_plan("cast-world-collision");
        prepare_through_chapters(&project, &session);

        run_stage(&session, Stage::Cast, cast_reply());
        run_stage(&session, Stage::World, world_reply());

        let cast = items(&session, Stage::Cast).unwrap();
        let world = items(&session, Stage::World).unwrap();
        assert_eq!(cast.len(), 2);
        assert_eq!(world.len(), 4);
        assert!(cast.iter().any(|item| item.key == "char-a"));
        assert_eq!(
            world.len(),
            4,
            "world lane contains settings, lore, and two powers"
        );

        assert_eq!(accept(&session, Stage::Cast, None).unwrap(), 2);
        assert_eq!(accept(&session, Stage::World, None).unwrap(), 4);
        assert_eq!(commit(&session, Stage::Cast, None).unwrap().committed, 2);
        assert_eq!(commit(&session, Stage::World, None).unwrap().committed, 4);
    }

    #[test]
    fn accept_world_proposals_then_commit_commits_every_accepted_item() {
        let _home = TempHome::new("outline-accept-world-commit");
        let (project, session) = project_with_plan("accept-world-commit");
        prepare_through_chapters(&project, &session);
        run_stage(&session, Stage::Cast, cast_reply());
        accept(&session, Stage::Cast, None).unwrap();
        commit(&session, Stage::Cast, None).unwrap();
        run_stage(&session, Stage::World, world_reply());

        let accepted = accept(&session, Stage::World, None).unwrap();
        assert_eq!(accepted, 4);
        let status = status(&session).unwrap();
        assert_eq!(status.counts["accepted"], accepted);
        assert_eq!(status.counts["pending"], 0);

        let report = commit(&session, Stage::World, None).unwrap();
        assert_eq!(report.committed, accepted as usize);
        assert_eq!(
            items(&session, Stage::World)
                .unwrap()
                .iter()
                .filter(|item| item.status == "committed")
                .count(),
            accepted as usize
        );
    }

    #[test]
    fn accept_all_then_commit_accepts_all_arc_proposals() {
        let _home = TempHome::new("outline-accept-commit-all");
        let (project, session) = project_with_plan("accept-commit-all");
        start(&project, &session).unwrap();
        run_stage(&session, Stage::Arcs, arcs_reply());

        let accepted = accept(&session, Stage::Arcs, None).unwrap();
        assert_eq!(accepted, 3);
        let status = status(&session).unwrap();
        assert_eq!(status.counts["accepted"], 3);
        assert_eq!(status.counts["pending"], 0);

        let report = commit(&session, Stage::Arcs, None).unwrap();
        assert_eq!(report.committed, 3);
        assert_eq!(
            items(&session, Stage::Arcs)
                .unwrap()
                .iter()
                .filter(|item| item.status == "committed")
                .count(),
            3
        );
    }

    #[test]
    fn test_outline_commit_refuses_out_of_order() {
        let _home = TempHome::new("outline-order");
        let (project, session) = project_with_plan("order");
        start(&project, &session).unwrap();
        let error = tauri::async_runtime::block_on(run_with_backend(
            Arc::new(ScriptedBackend::new(vec![AdapterReply::text(
                chapters_reply(1, 2),
            )])),
            &session,
            &provider(),
            &model(),
            Stage::Chapters,
            Some("1-2"),
            mpsc::unbounded_channel().0,
            Arc::new(AtomicBool::new(false)),
        ))
        .unwrap_err();
        assert!(error.contains("Cần chốt giai đoạn"), "{error}");
    }

    #[test]
    fn test_outline_commit_rejects_count_mismatch() {
        let _home = TempHome::new("outline-count");
        let (project, session) = project_with_plan("count");
        start(&project, &session).unwrap();
        let short = json!({"arcs": [
            {"position":1,"code":"arc-01","name":"Mưa","goal":"g","outcome":"o","summarize":"s","chapters":6},
            {"position":2,"code":"arc-02","name":"Tro","goal":"g","outcome":"o","summarize":"s","chapters":6}
        ]})
        .to_string();
        tauri::async_runtime::block_on(run_with_backend(
            Arc::new(ScriptedBackend::new(vec![AdapterReply::text(short)])),
            &session,
            &provider(),
            &model(),
            Stage::Arcs,
            None,
            mpsc::unbounded_channel().0,
            Arc::new(AtomicBool::new(false)),
        ))
        .unwrap();
        accept(&session, Stage::Arcs, None).unwrap();
        let error = commit(&session, Stage::Arcs, None).unwrap_err();
        assert!(error.contains("cần đủ 3 arc"), "{error}");
    }

    #[test]
    fn test_outline_fanout_isolates_failures_and_is_resumable() {
        let _home = TempHome::new("outline-fanout");
        let shards = vec![
            Shard {
                lane: contract::Lane::ChapterSmith,
                key: "chapters-01".into(),
                system: "system".into(),
                user: "user".into(),
            },
            Shard {
                lane: contract::Lane::ChapterSmith,
                key: "chapters-02".into(),
                system: "system".into(),
                user: "user".into(),
            },
        ];
        let backend = Arc::new(ScriptedBackend::new(vec![
            AdapterReply::text(chapters_reply(1, 2)),
            AdapterReply::text("không phải JSON"),
            AdapterReply::text("không phải JSON"),
            AdapterReply::text("không phải JSON"),
        ]));
        let (stats, results) = tauri::async_runtime::block_on(fanout::run_shards(
            backend,
            provider(),
            model(),
            shards,
            1,
            Arc::new(AtomicBool::new(false)),
            mpsc::unbounded_channel().0,
        ));
        assert_eq!(results.len(), 2);
        assert_eq!(stats.failures, 1, "the broken shard is retried then failed");
        assert!(results.iter().any(|(_, result)| result.is_ok()));

        // Resuming only the failed shard fills the gap without touching the rest.
        let retry = Arc::new(ScriptedBackend::new(vec![AdapterReply::text(
            chapters_reply(3, 2),
        )]));
        let (stats, results) = tauri::async_runtime::block_on(fanout::run_shards(
            retry,
            provider(),
            model(),
            vec![Shard {
                lane: contract::Lane::ChapterSmith,
                key: "chapters-02".into(),
                system: "system".into(),
                user: "user".into(),
            }],
            1,
            Arc::new(AtomicBool::new(false)),
            mpsc::unbounded_channel().0,
        ));
        assert_eq!(stats.failures, 0);
        assert!(results[0].1.is_ok());
    }

    #[test]
    fn test_state_order_is_sorted_and_complete() {
        use stage::Entity;
        assert_eq!(stage::STATE_ORDER.len(), 26);
        let rank = |entity: Entity| {
            stage::STATE_ORDER
                .iter()
                .position(|known| *known == entity)
                .unwrap()
        };
        assert!(rank(Entity::Arcs) < rank(Entity::Chapters));
        assert!(rank(Entity::Chapters) < rank(Entity::Characters));
        assert!(rank(Entity::Characters) < rank(Entity::Relationships));
        assert!(rank(Entity::Memories) < rank(Entity::Relationships));
        assert!(rank(Entity::ProjectMotifs) < rank(Entity::Scenes));
        assert!(rank(Entity::SceneCharacters) < rank(Entity::MemorySourceLinks));
        assert_eq!(
            stage::STATE_ORDER.last(),
            Some(&Entity::SceneContent),
            "prose is written last"
        );
        assert!(stage::assert_order(&[Entity::Arcs, Entity::Chapters]).is_ok());
        assert!(stage::assert_order(&[Entity::Chapters, Entity::Arcs]).is_err());
    }

    #[test]
    fn test_outline_seed_resolves_genre_code() {
        let _home = TempHome::new("outline-seed");
        let (project, _session) = project_with_plan("seed");
        super_json::set(
            &project,
            "require.schema.project_genres.rows",
            json!([{ "genre_id": "Trinh thám", "is_primary": true }]),
        )
        .unwrap();
        let report = seed::seed_project(&project).unwrap();
        assert_eq!(report.resolved, 1);
        let document = super_json::load_or_init(&project).unwrap();
        let row = &document["require"]["schema"]
            .as_array()
            .unwrap()
            .iter()
            .find(|entry| entry["table"] == "project_genres")
            .unwrap()["rows"][0];
        assert_eq!(row["genre_code"], json!("mystery"));
    }
}
