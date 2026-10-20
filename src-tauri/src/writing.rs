//! Phase 3: turn the approved scene plan into prose, one scene at a time.
//!
//! Every scene is written into its own file, so five scenes can be written
//! concurrently and a rewrite never loses the previous draft. The state document
//! only records `content_path`, `word_count` and `status`; `scenes.content` stays
//! null forever, which keeps the JSON small and the prompt budget honest.

use std::{
    path::{Path, PathBuf},
    sync::{Arc, atomic::AtomicBool},
};

use serde_json::{Value, json};
use tokio::sync::mpsc;

use crate::{
    adapters::LlmBackend,
    catalog,
    models::{ModelConfig, OutlineEvent, ProviderConfig},
    paths,
    runtime::complete,
    super_json,
};

/// Prose rules every writing prompt carries. Kept in one place so the guardrail
/// and the prompt can never drift apart.
const PROSE_RULES: &str = "Cấm emoji. Cấm ký tự gạch ngang dài (en dash, em dash): dùng dấu phẩy \
                          hoặc dấu chấm. Không mở đầu bằng tiêu đề markdown. Không giải thích, \
                          không tóm tắt ở cuối. Viết tiếng Việt, thì quá khứ hoặc hiện tại, cùng \
                          một ngôi kể trong toàn cảnh.";

const SYSTEM_PROMPT: &str = "Bạn là người viết văn cho dự án tiểu thuyết này. Bạn nhận thiết lập đã \
                             chốt của một cảnh và viết đúng cảnh đó. Chỉ trả về văn bản thuần, không \
                             JSON, không markdown fence.";

/// One scene as it will be written.
#[derive(Clone, Debug)]
pub struct SceneBundle {
    pub name: String,
    /// Word budget the prompt asks for; the report compares against it.
    pub target_words: u64,
    pub system: String,
    pub user: String,
}

#[derive(Clone, Debug)]
pub struct WriteReport {
    pub scene: String,
    pub path: String,
    pub word_count: u64,
    /// `false` when the draft drifted more than 15% from the scene budget.
    pub within_budget: bool,
    pub clean: bool,
    pub violations: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct Violation {
    pub kind: &'static str,
    pub offset: usize,
}

impl Violation {
    fn label(&self) -> String {
        format!("{} tại vị trí {}", self.kind, self.offset)
    }
}

/// Finds emoji and long dashes. The check is deterministic: the model is retried
/// once, and a second failure is sanitised instead of shipped as is.
pub fn violations(text: &str) -> Vec<Violation> {
    let mut found = Vec::new();
    for (offset, character) in text.char_indices() {
        let code = character as u32;
        let is_dash = matches!(character, '\u{2013}' | '\u{2014}' | '\u{2012}' | '\u{2015}');
        let is_emoji = matches!(
            code,
            0x1F300..=0x1FAFF | 0x1F000..=0x1F2FF | 0x2600..=0x27BF | 0xFE0F
        );
        if is_dash {
            found.push(Violation {
                kind: "gạch ngang dài",
                offset,
            });
        } else if is_emoji {
            found.push(Violation {
                kind: "emoji",
                offset,
            });
        }
    }
    found
}

/// Last resort: replace what the model refused to drop.
pub fn sanitize(text: &str) -> String {
    let mut cleaned = String::with_capacity(text.len());
    for character in text.chars() {
        let code = character as u32;
        let is_dash = matches!(character, '\u{2013}' | '\u{2014}' | '\u{2012}' | '\u{2015}');
        let is_emoji = matches!(
            code,
            0x1F300..=0x1FAFF | 0x1F000..=0x1F2FF | 0x2600..=0x27BF | 0xFE0F
        );
        match (is_dash, is_emoji) {
            (true, _) => cleaned.push_str(", "),
            (_, true) => {}
            _ => cleaned.push(character),
        }
    }
    cleaned
}

fn word_count(text: &str) -> u64 {
    text.split_whitespace().count() as u64
}

/// `projects/<id>/content/chapter-001/scene-002.md`
pub fn scene_path(project_id: &str, chapter: u64, position: u64) -> Result<PathBuf, String> {
    let dir = paths::project_dir(project_id)?.join("content");
    Ok(dir
        .join(format!("chapter-{chapter:03}"))
        .join(format!("scene-{position:03}.md")))
}

/// `.../versions/scene-002-v3.md`, next to the scene it replaces.
pub fn version_path(
    project_id: &str,
    chapter: u64,
    position: u64,
    version: u32,
) -> Result<PathBuf, String> {
    let dir = paths::project_dir(project_id)?
        .join("content")
        .join(format!("chapter-{chapter:03}"))
        .join("versions");
    Ok(dir.join(format!("scene-{position:03}-v{version}.md")))
}

fn write_file(path: &Path, content: &str) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|error| format!("Không tạo được thư mục cảnh: {error}"))?;
    }
    std::fs::write(path, content).map_err(|error| format!("Không ghi được cảnh: {error}"))
}

/// Keeps the previous draft before the new one lands, numbered so nothing is lost.
fn archive_previous(project_id: &str, chapter: u64, position: u64) -> Result<Option<u32>, String> {
    let current = scene_path(project_id, chapter, position)?;
    if !current.exists() {
        return Ok(None);
    }
    let previous = std::fs::read_to_string(&current)
        .map_err(|error| format!("Không đọc được cảnh cũ: {error}"))?;
    let mut version = 1u32;
    while version_path(project_id, chapter, position, version)?.exists() {
        version += 1;
    }
    let target = version_path(project_id, chapter, position, version)?;
    write_file(&target, &previous)?;
    Ok(Some(version))
}

fn row_text(row: &Value, field: &str) -> Option<String> {
    row.get(field)
        .and_then(Value::as_str)
        .map(str::to_owned)
        .filter(|value| !value.trim().is_empty())
}

fn rows(document: &Value, table: &str) -> Vec<Value> {
    ["require", "optional"]
        .into_iter()
        .find_map(|group| {
            let entries = document.get(group)?.get("schema")?.as_array()?;
            let entry = entries.iter().find(|entry| entry["table"] == table)?;
            entry["rows"].as_array().cloned()
        })
        .unwrap_or_default()
}

fn compact_list(values: &[String], empty: &str) -> String {
    if values.is_empty() {
        empty.to_owned()
    } else {
        values.join("; ")
    }
}

/// Builds the prompt for one scene from the committed state.
///
/// The bundle carries everything phase 3 needs and nothing else, which is why it
/// never embeds the whole document.
pub fn bundle(project_id: &str, chapter: u64, position: u64) -> Result<SceneBundle, String> {
    let document = super_json::load_or_init(project_id)?;
    let scenes = rows(&document, "scenes");
    let scene = scenes
        .iter()
        .find(|row| {
            row.get("chapter_id_json_index").and_then(Value::as_u64) == Some(chapter - 1)
                && row.get("position").and_then(Value::as_u64) == Some(position)
        })
        .ok_or_else(|| format!("Không tìm thấy cảnh {position} của chương {chapter}"))?;

    let chapter_row = rows(&document, "chapters")
        .into_iter()
        .nth((chapter - 1) as usize)
        .ok_or_else(|| format!("Không tìm thấy chương {chapter}"))?;
    let name = row_text(scene, "name").unwrap_or_else(|| format!("Cảnh {position}"));
    let target_words = scene
        .get("target_words")
        .and_then(Value::as_u64)
        .unwrap_or(900);

    // Previous scenes of the same chapter, so the prose keeps its momentum.
    let previous: Vec<String> = scenes
        .iter()
        .filter(|row| {
            row.get("chapter_id_json_index").and_then(Value::as_u64) == Some(chapter - 1)
                && row.get("position").and_then(Value::as_u64) < Some(position)
                && row.get("status").and_then(Value::as_str) == Some("WRITTEN")
        })
        .filter_map(|row| {
            let position = row.get("position").and_then(Value::as_u64)?;
            let path = scene_path(project_id, chapter, position).ok()?;
            let text = std::fs::read_to_string(path).ok()?;
            Some(format!(
                "Cảnh {position} ({}): {}",
                row_text(row, "name").unwrap_or_default(),
                text.chars().take(600).collect::<String>()
            ))
        })
        .collect();

    let cast_refs: Vec<String> = rows(&document, "scene_characters")
        .into_iter()
        .filter(|row| {
            row.get("scene_id_json_index").and_then(Value::as_u64)
                == scene.get("_json_index").and_then(Value::as_u64)
        })
        .filter_map(|row| {
            let index = row.get("character_id_json_index").and_then(Value::as_u64)? as usize;
            let characters = rows(&document, "characters");
            let character = characters.get(index)?;
            let mut sheet = vec![format!(
                "{} ({})",
                row_text(character, "name").unwrap_or_default(),
                row_text(character, "role").unwrap_or_default()
            )];
            if let Some(summary) = row_text(character, "summary") {
                sheet.push(summary);
            }
            if let Some(motivation) = row_text(character, "motivation") {
                sheet.push(format!("Động lực: {motivation}"));
            }
            if let Some(flaw) = row_text(character, "flaw") {
                sheet.push(format!("Điểm yếu: {flaw}"));
            }
            if row.get("is_pov").and_then(Value::as_bool) == Some(true) {
                sheet.push("người kể chuyện của cảnh".into());
            }
            Some(sheet.join(". "))
        })
        .collect();

    let setting = scene
        .get("world_setting_id_json_index")
        .and_then(Value::as_u64)
        .and_then(|index| {
            let settings = rows(&document, "world_settings");
            let row = settings.get(index as usize)?;
            let mut parts = vec![row_text(row, "name").unwrap_or_default()];
            for field in [
                "description",
                "spatial_features",
                "sensory_visual",
                "sensory_auditory",
                "sensory_tactile",
                "sensory_olfactory",
                "sensory_gustatory",
            ] {
                if let Some(value) = row_text(row, field) {
                    parts.push(value);
                }
            }
            Some(parts.join(". "))
        })
        .unwrap_or_else(|| "Địa điểm chưa mô tả".into());

    let preset = scene
        .get("style_preset_code")
        .and_then(Value::as_str)
        .and_then(|code| {
            catalog::all(catalog::Kind::StylePresets)
                .iter()
                .find(|entry| entry.code == code)
                .map(|entry| {
                    let data = &entry.data;
                    let devices = data["devices"]
                        .as_array()
                        .map(|items| {
                            items
                                .iter()
                                .filter(|item| item["usage_type"] != "avoid")
                                .filter_map(|item| item["code"].as_str())
                                .collect::<Vec<_>>()
                                .join(", ")
                        })
                        .unwrap_or_default();
                    format!(
                        "{}: nhịp {}, cảm xúc {}, thủ pháp {}, từ vựng {}",
                        entry.name,
                        data["tone_pace"].as_str().unwrap_or("-"),
                        data["tone_emotion"].as_str().unwrap_or("-"),
                        if devices.is_empty() { "-" } else { &devices },
                        compact_list(
                            &data["vocab"]
                                .as_array()
                                .map(|items| {
                                    items
                                        .iter()
                                        .filter_map(Value::as_str)
                                        .map(str::to_owned)
                                        .collect::<Vec<String>>()
                                })
                                .unwrap_or_default(),
                            "-"
                        )
                    )
                })
        })
        .unwrap_or_else(|| "Không có preset".into());

    let narrator = scene
        .get("narrator_code")
        .and_then(Value::as_str)
        .and_then(|code| {
            catalog::all(catalog::Kind::NarratorConfigs)
                .iter()
                .find(|entry| entry.code == code)
                .map(|entry| {
                    format!(
                        "{}: {}, {}, {}",
                        entry.name,
                        entry.data["person"].as_str().unwrap_or("-"),
                        entry.data["person_subtype"].as_str().unwrap_or("-"),
                        entry.data["pov_scope"].as_str().unwrap_or("-")
                    )
                })
        })
        .unwrap_or_else(|| "Không có narrator".into());

    let directives = scene
        .get("directives")
        .and_then(Value::as_array)
        .map(|items| {
            items
                .iter()
                .filter_map(|item| {
                    Some(format!(
                        "- {} ({}): {}",
                        item["code"].as_str()?,
                        item["weight"].as_str().unwrap_or("-"),
                        item["instruction"].as_str().unwrap_or("")
                    ))
                })
                .collect::<Vec<_>>()
                .join("\n")
        })
        .unwrap_or_default();

    let rules = scene
        .get("rules")
        .and_then(Value::as_array)
        .map(|items| {
            items
                .iter()
                .filter_map(Value::as_str)
                .map(|rule| format!("- {rule}"))
                .collect::<Vec<_>>()
                .join("\n")
        })
        .unwrap_or_default();

    let beats = chapter_row
        .get("beats")
        .and_then(Value::as_array)
        .map(|items| {
            items
                .iter()
                .filter_map(Value::as_str)
                .collect::<Vec<_>>()
                .join("; ")
        })
        .unwrap_or_default();

    let user = format!(
        "Chương {chapter}: {}\nTóm tắt chương: {}\nBeats: {}\n\nCảnh {position}: {name}\nTóm tắt cảnh: {}\n\
         Độ dài mục tiêu: {target_words} từ\n\nNhân vật trong cảnh:\n{}\n\nĐịa điểm:\n{setting}\n\n\
         Phong cách:\n{preset}\n\nNgười kể chuyện:\n{narrator}\n\nDirective:\n{directives}\n\n\
         Luật riêng của cảnh:\n{rules}\n\nCác cảnh trước:\n{}\n\n{PROSE_RULES}\n\n\
         Viết cảnh này, chỉ trả về văn bản.",
        row_text(&chapter_row, "name").unwrap_or_default(),
        row_text(&chapter_row, "summarize").unwrap_or_default(),
        beats,
        row_text(scene, "summarize").unwrap_or_default(),
        if cast_refs.is_empty() {
            "Chưa có nhân vật".to_owned()
        } else {
            cast_refs.join("\n")
        },
        if previous.is_empty() {
            "Đây là cảnh đầu của chương".to_owned()
        } else {
            previous.join("\n")
        }
    );

    Ok(SceneBundle {
        name,
        target_words,
        system: SYSTEM_PROMPT.to_owned(),
        user,
    })
}

/// Writes one scene: provider call, guardrail, retry, file, state.
///
/// The argument list mirrors what a call needs: backend, provider, model,
/// project, chapter, position, author note and the event sink.
#[allow(clippy::too_many_arguments)]
pub async fn write_scene(
    backend: Arc<dyn LlmBackend>,
    provider: &ProviderConfig,
    model: &ModelConfig,
    project_id: &str,
    chapter: u64,
    position: u64,
    note: Option<&str>,
    sink: mpsc::UnboundedSender<OutlineEvent>,
) -> Result<WriteReport, String> {
    let document = super_json::load_or_init(project_id)?;
    let scenes = rows(&document, "scenes");
    let Some(scene) = scenes.iter().find(|row| {
        row.get("chapter_id_json_index").and_then(Value::as_u64) == Some(chapter - 1)
            && row.get("position").and_then(Value::as_u64) == Some(position)
    }) else {
        return Err(format!(
            "Không tìm thấy cảnh {position} của chương {chapter}"
        ));
    };
    if scene.get("status").and_then(Value::as_str) == Some("LOCKED") {
        return Err("Cảnh này đã bị khoá, mở khoá trước khi viết lại".into());
    }

    let bundle = bundle(project_id, chapter, position)?;
    let mut user = bundle.user.clone();
    if let Some(note) = note.filter(|note| !note.trim().is_empty()) {
        user.push_str(&format!("\nGhi chú của tác giả cho lần viết lại: {note}"));
    }
    let _ = sink.send(OutlineEvent::ShardStarted {
        key: format!("scene-{chapter:03}-{position:03}"),
        lane: "prose_writer".into(),
    });

    let mut retried = false;
    let mut sanitised: Vec<String> = Vec::new();
    let text;
    loop {
        let completion = complete(backend.as_ref(), provider, model, &bundle.system, &user).await?;
        let found = violations(&completion.text);
        if found.is_empty() {
            text = completion.text;
            break;
        }
        if retried {
            text = sanitize(&completion.text);
            sanitised = found.iter().map(Violation::label).collect();
            break;
        }
        retried = true;
        user = format!(
            "{user}\nLần trước bạn vi phạm: {}. Viết lại, tuyệt đối không emoji và không gạch ngang dài.",
            found
                .iter()
                .map(Violation::label)
                .collect::<Vec<_>>()
                .join(", ")
        );
    }

    // `clean` answers "did the model comply", so a draft we had to sanitise is
    // reported as dirty even though the shipped text is now free of violations.
    let clean = sanitised.is_empty() && violations(&text).is_empty();
    let words = word_count(&text);
    let archived = archive_previous(project_id, chapter, position)?;
    let path = scene_path(project_id, chapter, position)?;
    write_file(&path, &text)?;

    let relative = format!("content/chapter-{chapter:03}/scene-{position:03}.md");
    let index = scene
        .get("_json_index")
        .and_then(Value::as_u64)
        .ok_or("Cảnh thiếu _json_index")?;
    super_json::add_field(
        project_id,
        "scenes",
        index as i64,
        "content_path",
        json!(relative),
    )?;
    super_json::add_field(
        project_id,
        "scenes",
        index as i64,
        "word_count",
        json!(words),
    )?;
    super_json::add_field(
        project_id,
        "scenes",
        index as i64,
        "status",
        json!("WRITTEN"),
    )?;

    let _ = sink.send(OutlineEvent::ShardFinished {
        key: format!("scene-{chapter:03}-{position:03}"),
        lane: "prose_writer".into(),
        ok: true,
        error: None,
    });
    if let Some(version) = archived {
        let _ = version;
    }
    let budget = bundle.target_words.max(1);
    let within_budget = words.abs_diff(budget) * 100 <= budget * 15;
    Ok(WriteReport {
        scene: bundle.name,
        path: relative,
        word_count: words,
        within_budget,
        clean,
        violations: sanitised,
    })
}

/// Rewrites a selection of scenes, one call per scene, sequential inside a
/// chapter so the prose keeps its momentum.
pub async fn rewrite_selection(
    backend: Arc<dyn LlmBackend>,
    provider: &ProviderConfig,
    model: &ModelConfig,
    project_id: &str,
    scenes: &[(u64, u64)],
    note: Option<&str>,
    cancel: Arc<AtomicBool>,
) -> Result<Vec<WriteReport>, String> {
    let mut reports = Vec::new();
    for (chapter, position) in scenes {
        if cancel.load(std::sync::atomic::Ordering::Relaxed) {
            return Err("Đã hủy lượt viết".into());
        }
        let (sender, _receiver) = mpsc::unbounded_channel();
        reports.push(
            write_scene(
                Arc::clone(&backend),
                provider,
                model,
                project_id,
                *chapter,
                *position,
                note,
                sender,
            )
            .await?,
        );
    }
    Ok(reports)
}

/// Locks or unlocks a scene so a rewrite cannot touch approved prose.
pub fn set_locked(
    project_id: &str,
    chapter: u64,
    position: u64,
    locked: bool,
) -> Result<(), String> {
    let document = super_json::load_or_init(project_id)?;
    let scenes = rows(&document, "scenes");
    let scene = scenes
        .iter()
        .find(|row| {
            row.get("chapter_id_json_index").and_then(Value::as_u64) == Some(chapter - 1)
                && row.get("position").and_then(Value::as_u64) == Some(position)
        })
        .ok_or_else(|| format!("Không tìm thấy cảnh {position} của chương {chapter}"))?;
    let index = scene
        .get("_json_index")
        .and_then(Value::as_u64)
        .ok_or("Cảnh thiếu _json_index")? as i64;
    let status = if locked { "LOCKED" } else { "REVISED" };
    super_json::add_field(project_id, "scenes", index, "status", json!(status))
}

/// Writing progress for the workspace header.
pub fn status(project_id: &str) -> Result<Value, String> {
    let document = super_json::load_or_init(project_id)?;
    let scenes = rows(&document, "scenes");
    let written = scenes
        .iter()
        .filter(|row| {
            matches!(
                row["status"].as_str(),
                Some("WRITTEN") | Some("REVISED") | Some("LOCKED")
            )
        })
        .count();
    let words: u64 = scenes
        .iter()
        .filter_map(|row| row.get("word_count").and_then(Value::as_u64))
        .sum();
    Ok(json!({
        "ok": true,
        "scenes": scenes.len(),
        "written": written,
        "words": words,
        "locked": scenes.iter().filter(|row| row["status"] == json!("LOCKED")).count(),
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapters::{AdapterReply, ScriptedBackend};
    use crate::models::InputTypes;
    use serde_json::json;

    fn provider() -> ProviderConfig {
        ProviderConfig {
            id: "p-prose".into(),
            name: "Prose provider".into(),
            kind: "openai".into(),
            api_key: "test-key".into(),
            base_url: "https://example.invalid".into(),
            models: Vec::new(),
            default_model_id: "m-prose".into(),
        }
    }

    fn model() -> ModelConfig {
        ModelConfig {
            id: "m-prose".into(),
            name: "Prose model".into(),
            context_window: "8192".into(),
            max_tokens: "2048".into(),
            input_types: InputTypes {
                text: true,
                image: false,
            },
            reasoning_level: false,
        }
    }

    #[test]
    fn test_violations_finds_emoji_and_long_dashes() {
        let found = violations("Trời mưa – rất lạnh 😀");
        assert_eq!(found.len(), 2, "{found:?}");
        assert!(found.iter().any(|violation| violation.kind == "emoji"));
        assert!(
            found
                .iter()
                .any(|violation| violation.kind == "gạch ngang dài")
        );
        assert!(violations("Trời mưa, rất lạnh.").is_empty());
    }

    #[test]
    fn test_sanitize_replaces_dashes_and_drops_emoji() {
        let cleaned = sanitize("Mưa – lạnh 😀 rồi tạnh");
        assert!(!cleaned.contains('\u{2013}'));
        assert!(!cleaned.contains('😀'));
        assert!(cleaned.contains("Mưa"));
        assert!(cleaned.contains("rạnh") || cleaned.contains("rồi"));
    }

    /// Minimal committed state: one chapter, one scene, one character, one place.
    fn project_with_one_scene(tag: &str) -> String {
        let session = format!("sess-{tag}");
        let connection = crate::db::open().unwrap();
        crate::db::ensure_session(&connection, &session, "").unwrap();
        let created = crate::tools::execute_tool(
            "project_create",
            &json!({ "name": "Tro Than" }),
            "",
            &session,
        )
        .unwrap();
        let project = created["project_id"].as_str().unwrap().to_owned();
        for (path, value) in [
            (
                "optional.schema.chapters.rows",
                json!([{
                    "name": "Mở màn", "position": 1, "summarize": "An trở về làng",
                    "beats": ["về làng", "gặp Bình"], "scene_count": 1
                }]),
            ),
            ("optional.schema.chapters.count", json!(1)),
            (
                "require.schema.characters.rows",
                json!([{
                    "name": "An", "is_primary": true, "role": "protagonist",
                    "summary": "Thợ rèn trẻ", "motivation": "Đòi lại nhà", "flaw": "Cứng đầu"
                }]),
            ),
            ("require.schema.characters.count", json!(1)),
            (
                "optional.schema.world_settings.rows",
                json!([{
                    "name": "Làng Tro", "type": "TOWN", "access": "PUBLIC",
                    "description": "Làng ven sông", "sensory_visual": "xám",
                    "sensory_auditory": "gió"
                }]),
            ),
            ("optional.schema.world_settings.count", json!(1)),
        ] {
            crate::super_json::set(&project, path, value).unwrap();
        }
        crate::super_json::set(
            &project,
            "optional.schema.scenes.rows",
            json!([{
                "name": "Tro tàn", "position": 1, "chapter_id_json_index": 0,
                "world_setting_id_json_index": 0, "style_preset_code": "action.battle",
                "narrator_code": "first_person_protagonist",
                "summarize": "An về làng trong mưa", "target_words": 900,
                "directives": [{"code": "show_dont_tell", "weight": "HIGH",
                    "instruction": "Cho thấy cái lạnh qua bàn tay"}],
                "rules": ["Không miêu tả quá hai giác quan mỗi đoạn"],
                "status": "PLANNED"
            }]),
        )
        .unwrap();
        crate::super_json::set(&project, "optional.schema.scenes.count", json!(1)).unwrap();
        crate::super_json::set(
            &project,
            "optional.schema.scene_characters.rows",
            json!([{ "scene_id_json_index": 0, "character_id_json_index": 0, "is_pov": true }]),
        )
        .unwrap();
        crate::super_json::set(&project, "optional.schema.scene_characters.count", json!(1))
            .unwrap();
        project
    }

    fn write_with(project: &str, replies: Vec<&str>) -> Result<WriteReport, String> {
        let backend = Arc::new(ScriptedBackend::new(
            replies.into_iter().map(AdapterReply::text).collect(),
        ));
        let (sender, _receiver) = mpsc::unbounded_channel();
        tauri::async_runtime::block_on(write_scene(
            backend,
            &provider(),
            &model(),
            project,
            1,
            1,
            None,
            sender,
        ))
    }

    #[test]
    fn test_write_scene_creates_file_and_updates_state() {
        let _home = crate::test_support::TempHome::new("prose-write");
        let project = project_with_one_scene("write");
        let report = write_with(
            &project,
            vec!["Mưa xuống trên mái lá. An đứng im, tay lạnh."],
        )
        .unwrap();
        assert!(report.clean);
        assert!(report.word_count > 0);
        assert!(
            !report.within_budget,
            "a nine word draft is far below 900 words"
        );
        let path = scene_path(&project, 1, 1).unwrap();
        assert!(path.exists());
        let document = super_json::load_or_init(&project).unwrap();
        let scene = &rows(&document, "scenes")[0];
        assert_eq!(
            scene["content_path"],
            json!("content/chapter-001/scene-001.md")
        );
        assert_eq!(scene["status"], json!("WRITTEN"));
        assert_eq!(scene["content"], Value::Null);
        assert_eq!(scene["word_count"], json!(report.word_count));
    }

    #[test]
    fn test_write_scene_retries_on_emoji_and_dash() {
        let _home = crate::test_support::TempHome::new("prose-retry");
        let project = project_with_one_scene("retry");
        let report = write_with(
            &project,
            vec!["Trời mưa 😀 rất lạnh – An đứng im.", "Trời mưa, rất lạnh."],
        )
        .unwrap();
        assert!(report.clean, "the second draft is clean");
        let text = std::fs::read_to_string(scene_path(&project, 1, 1).unwrap()).unwrap();
        assert_eq!(text, "Trời mưa, rất lạnh.");
    }

    #[test]
    fn test_write_scene_sanitises_a_second_violation() {
        let _home = crate::test_support::TempHome::new("prose-sanitise");
        let project = project_with_one_scene("sanitise");
        let report = write_with(
            &project,
            vec!["Trời mưa 😀 rất lạnh.", "Vẫn mưa 😀 và lạnh – thật."],
        )
        .unwrap();
        assert!(!report.clean, "the second draft still had violations");
        assert!(!report.violations.is_empty());
        let text = std::fs::read_to_string(scene_path(&project, 1, 1).unwrap()).unwrap();
        assert!(!text.contains('😀'));
        assert!(!text.contains('\u{2013}'));
    }

    #[test]
    fn test_rewrite_keeps_the_previous_version() {
        let _home = crate::test_support::TempHome::new("prose-version");
        let project = project_with_one_scene("version");
        write_with(&project, vec!["Bản một, trời mưa."]).unwrap();
        write_with(&project, vec!["Bản hai, trời tạnh."]).unwrap();
        let version = version_path(&project, 1, 1, 1).unwrap();
        assert!(version.exists(), "the first draft must be archived");
        assert_eq!(
            std::fs::read_to_string(version).unwrap(),
            "Bản một, trời mưa."
        );
        assert_eq!(
            std::fs::read_to_string(scene_path(&project, 1, 1).unwrap()).unwrap(),
            "Bản hai, trời tạnh."
        );
    }

    #[test]
    fn test_locked_scene_is_not_rewritten() {
        let _home = crate::test_support::TempHome::new("prose-locked");
        let project = project_with_one_scene("locked");
        write_with(&project, vec!["Bản đã duyệt."]).unwrap();
        set_locked(&project, 1, 1, true).unwrap();
        let error = write_with(&project, vec!["Bản khác."]).unwrap_err();
        assert!(error.contains("khoá"), "{error}");
        assert_eq!(
            std::fs::read_to_string(scene_path(&project, 1, 1).unwrap()).unwrap(),
            "Bản đã duyệt."
        );
    }

    #[test]
    fn test_scene_paths_are_inside_the_project() {
        let _home = crate::test_support::TempHome::new("prose-paths");
        let path = scene_path("du-an", 2, 3).unwrap();
        assert!(path.ends_with("content/chapter-002/scene-003.md"));
        let version = version_path("du-an", 2, 3, 4).unwrap();
        assert!(version.ends_with("content/chapter-002/versions/scene-003-v4.md"));
    }
}
