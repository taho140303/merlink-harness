//! Outline generation engine for phase 2.
//!
//! One run per session, one proposal row per entity in `outline_items`, and a
//! fan-out that never touches the project state: only [`apply::commit`] writes,
//! and only the proposals the author accepted, in [`stage::STATE_ORDER`].

pub mod anchor;
pub mod apply;
pub mod contract;
pub mod fanout;
pub mod seed;
pub mod stage;

pub use anchor::{anchor, anchor_compact, anchor_for_project, anchor_json};

use std::{
    collections::BTreeSet,
    sync::{Arc, atomic::AtomicBool},
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

/// Chapters per subagent call: small enough to keep one prompt focused, big
/// enough that a 120-chapter plan becomes 12 shards for five workers.
const CHAPTERS_PER_SHARD: u64 = 10;

/// Default fan-out width. Five is the number the phase-2 plan promises.
const DEFAULT_CONCURRENCY: usize = 5;

/// One planned provider call. `shard` is the stable key used to store the
/// proposals it produced and to rebuild the very same call on regenerate.
struct ShardPlan {
    lane: Lane,
    shard: String,
    system: String,
    user: String,
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
        usage: json!({
            "calls": run.calls,
            "tokens_in": run.tokens_in,
            "tokens_out": run.tokens_out,
        }),
    })
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

fn system_prompt(lane: Lane) -> String {
    format!(
        "Bạn là trợ lý dàn ý của Merlink, trả lời bằng tiếng Việt. Chỉ trả về đúng một object JSON \
         theo hợp đồng, không kèm giải thích và không markdown fence. Cấm emoji. Cấm ký tự gạch \
         ngang dài. Mỗi code là slug ASCII duy nhất, không khoảng trắng. Không bịa mã style preset, \
         narrator hay directive ngoài catalogue. Bạn không ghi state: chỉ đề xuất, tác giả duyệt \
         rồi backend mới ghi. Hợp đồng đầu ra: {}",
        lane.shape()
    )
}

fn arc_plan(document: &Value) -> Result<ShardPlan, String> {
    let (arc_total, chapter_total, scene_total) = plan_counts(document)?;
    Ok(ShardPlan {
        lane: Lane::ArcSmith,
        shard: "arcs".to_owned(),
        system: system_prompt(Lane::ArcSmith),
        user: format!(
            "Khối anchor:\n{}\n\nNgân sách: đúng {arc_total} arc, tổng {chapter_total} chương, \
             tổng {scene_total} cảnh. Trường chapters của mỗi arc là số chương của arc đó và tổng \
             phải bằng {chapter_total}.",
            anchor_compact(document)
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
            system: system_prompt(Lane::ChapterSmith),
            user: format!(
                "Khối anchor:\n{compact}\n\nCác arc đã chốt:\n{brief}\n\nViết chương {start} đến \
                 {end} trong tổng {chapter_total} chương. Trường arc là số thứ tự arc (bắt đầu từ \
                 1). Trường position là vị trí chương trong toàn truyện. Tổng scene_count của các \
                 chương bạn viết phải bằng {scene_budget}.",
            ),
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

fn build_plan(
    stage: Stage,
    document: &Value,
    range: Option<&str>,
) -> Result<Vec<ShardPlan>, String> {
    match stage {
        Stage::Arcs => Ok(vec![arc_plan(document)?]),
        Stage::Chapters => chapter_plans(document, range),
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
        let key = entry
            .get("code")
            .and_then(Value::as_str)
            .ok_or_else(|| format!("{array_key} có phần tử thiếu code"))?;
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
    });

    let (stats, results) = fanout::run_shards(
        backend,
        provider.clone(),
        model.clone(),
        shards,
        DEFAULT_CONCURRENCY,
        Arc::clone(&cancel),
        forward_events(sink.clone()),
    )
    .await;

    let mut stored = Vec::new();
    for (shard_key, result) in results {
        let Some(plan) = plans.iter().find(|plan| plan.shard == shard_key) else {
            continue;
        };
        match result {
            Ok(value) => stored.extend(proposals_from(&run.id, stage, plan, &value)?),
            Err(_) => continue,
        }
    }
    if stored.is_empty() {
        let error = "Không có đề xuất hợp lệ nào được sinh ra".to_owned();
        let _ = sink.send(OutlineEvent::RunFailed {
            error: error.clone(),
        });
        return Err(error);
    }
    db::upsert_items(&connection, &stored)?;
    db::update_run_stage(&connection, &run.id, stage.as_str())?;
    db::update_run_status(&connection, &run.id, "review")?;
    db::update_run_usage(
        &connection,
        &run.id,
        run.calls + stats.calls as i64,
        run.tokens_in + stats.tokens_in as i64,
        run.tokens_out + stats.tokens_out as i64,
    )?;
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
            Ok(OutlineItem {
                key: item.key,
                lane: item.lane,
                position: item.position,
                status: item.status,
                title: value
                    .get("name")
                    .and_then(Value::as_str)
                    .unwrap_or_default()
                    .to_owned(),
                error: item.error,
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
        let selected = keys.is_none_or(|wanted| wanted.iter().any(|key| key == &item.key));
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

        let chapters_backend = Arc::new(ScriptedBackend::new(vec![
            AdapterReply::text(chapters_reply(1, 10)),
            AdapterReply::text(chapters_reply(11, 2)),
        ]));
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
        assert_eq!(result.items, 12, "twelve chapters across two shards");
        assert!(result.shards >= 2);
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
