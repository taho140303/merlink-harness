//! Deterministic anchor generator extracting core project narrative anchors.
//!
//! Generates Vietnamese text (exactly 6 paragraphs) and structured JSON for prompt injection.

use serde_json::{Value, json};

const PLACEHOLDER: &str = "[chưa có: ";

fn project_text(document: &Value, group: &str, leaf: &str) -> Option<String> {
    let mut cursor = document.get(group)?.get("project")?;
    for segment in leaf.split('.') {
        cursor = cursor.get(segment)?;
    }
    cursor
        .as_str()
        .map(str::trim)
        .filter(|t| !t.is_empty())
        .map(str::to_owned)
}

fn project_int(document: &Value, group: &str, leaf: &str) -> Option<i64> {
    let mut cursor = document.get(group)?.get("project")?;
    for segment in leaf.split('.') {
        cursor = cursor.get(segment)?;
    }
    cursor.as_i64()
}

fn rows(document: &Value, table: &str) -> Vec<Value> {
    crate::super_json::countable_entry(document, table)
        .and_then(|entry| entry.get("rows"))
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default()
}

fn row_text(row: &Value, field: &str) -> Option<String> {
    row.get(field)
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|t| !t.is_empty())
        .map(str::to_owned)
}

fn or_path(val: Option<String>, path: &str) -> String {
    val.unwrap_or_else(|| format!("{PLACEHOLDER}{path}]"))
}

fn or_path_int(val: Option<i64>, path: &str) -> String {
    val.map_or_else(|| format!("{PLACEHOLDER}{path}]"), |n| n.to_string())
}

fn or_path_num(val: Option<f64>, path: &str) -> String {
    val.map_or_else(|| format!("{PLACEHOLDER}{path}]"), |n| n.to_string())
}

/// Generates exactly 6 paragraphs summarizing the project narrative state.
/// All missing/null/empty values produce `[chưa có: <path>]`.
/// Output contains no emojis, and no en-dash / em-dash characters.
pub fn anchor(document: &Value) -> String {
    // ── Đoạn 1: Tên, ngôn ngữ, thể loại (+ genre_code), plan word/arc/chapter/scene
    let name = or_path(
        project_text(document, "require", "name"),
        "require.project.name",
    );
    let lang = or_path(
        project_text(document, "require", "language"),
        "require.project.language",
    );

    let genre_rows = rows(document, "project_genres");
    let primary_genre_row = genre_rows
        .iter()
        .find(|r| r.get("is_primary").and_then(Value::as_bool) == Some(true))
        .or_else(|| genre_rows.first());

    let genre_name = primary_genre_row
        .and_then(|r| row_text(r, "genre_id"))
        .unwrap_or_else(|| format!("{PLACEHOLDER}require.schema.project_genres.rows]"));

    let genre_code = primary_genre_row
        .and_then(|r| row_text(r, "genre_code"))
        .unwrap_or_else(|| format!("{PLACEHOLDER}require.schema.project_genres.rows.genre_code]"));

    let word_plan = or_path_int(
        project_int(document, "require", "plan.target-words"),
        "require.project.plan.target-words",
    );
    let arc_plan = or_path_int(
        project_int(document, "require", "plan.target-arcs"),
        "require.project.plan.target-arcs",
    );
    let chapter_plan = or_path_int(
        project_int(document, "require", "plan.target-chapters"),
        "require.project.plan.target-chapters",
    );
    let scene_plan = or_path_int(
        project_int(document, "optional", "plan.target-scenes"),
        "optional.project.plan.target-scenes",
    );

    let p1 = format!(
        "Dự án {name}, ngôn ngữ {lang}, thể loại {genre_name} (mã: {genre_code}). Kế hoạch quy mô gồm {word_plan} từ, {arc_plan} arc, {chapter_plan} chương, {scene_plan} cảnh."
    );

    // ── Đoạn 2: Diễn tiến arc (mở đầu + các arc còn lại)
    let arc_rows = rows(document, "arcs");
    let p2 = if arc_rows.is_empty() {
        format!("Diễn tiến arc: {PLACEHOLDER}require.schema.arcs.rows].")
    } else {
        let mut parts = Vec::new();
        for (i, row) in arc_rows.iter().enumerate() {
            let arc_name = row_text(row, "name")
                .unwrap_or_else(|| format!("{PLACEHOLDER}require.schema.arcs.rows.name]"));
            let goal = row_text(row, "goal")
                .unwrap_or_else(|| format!("{PLACEHOLDER}require.schema.arcs.rows.goal]"));
            let outcome = row_text(row, "outcome")
                .unwrap_or_else(|| format!("{PLACEHOLDER}require.schema.arcs.rows.outcome]"));
            let summarize = row_text(row, "summarize")
                .unwrap_or_else(|| format!("{PLACEHOLDER}require.schema.arcs.rows.summarize]"));
            parts.push(format!(
                "Arc {}: {} (Mục tiêu: {}. Kết quả: {}. Tóm tắt: {})",
                i + 1,
                arc_name,
                goal,
                outcome,
                summarize
            ));
        }
        format!("Diễn tiến arc: {}.", parts.join("; "))
    };

    // ── Đoạn 3: Nhân vật chính (role, archetype, summary, description) + tuyến đối trọng
    let character_rows = rows(document, "characters");
    let p3 = if character_rows.is_empty() {
        format!("Nhân vật chính và tuyến đối trọng: {PLACEHOLDER}require.schema.characters.rows].")
    } else {
        // Tìm protagonist (is_primary = true hoặc name khớp hoặc row đầu tiên)
        let protagonist = character_rows
            .iter()
            .find(|r| r.get("is_primary").and_then(Value::as_bool) == Some(true))
            .or_else(|| character_rows.first());

        let protag_str = if let Some(p) = protagonist {
            let name = row_text(p, "name")
                .unwrap_or_else(|| format!("{PLACEHOLDER}require.schema.characters.rows.name]"));
            let role = row_text(p, "role")
                .unwrap_or_else(|| format!("{PLACEHOLDER}require.schema.characters.rows.role]"));
            let archetype = row_text(p, "archetype").unwrap_or_else(|| {
                format!("{PLACEHOLDER}require.schema.characters.rows.archetype]")
            });
            let summary = row_text(p, "summary")
                .unwrap_or_else(|| format!("{PLACEHOLDER}require.schema.characters.rows.summary]"));
            let description = row_text(p, "description").unwrap_or_else(|| {
                format!("{PLACEHOLDER}require.schema.characters.rows.description]")
            });
            format!(
                "Nhân vật chính: {name} (Vai trò: {role}, Hình mẫu: {archetype}, Tóm lược: {summary}, Miêu tả: {description})"
            )
        } else {
            format!("Nhân vật chính: {PLACEHOLDER}require.schema.characters.rows]")
        };

        let counter_characters: Vec<_> = character_rows
            .iter()
            .filter(|r| {
                if protagonist.is_some_and(|p| std::ptr::eq(*r, p)) {
                    return false;
                }
                let role = row_text(r, "role").unwrap_or_default().to_lowercase();
                role.contains("đối trọng")
                    || role.contains("antagonist")
                    || role.contains("phản diện")
                    || role.contains("rival")
            })
            .collect();

        let counter_str = if counter_characters.is_empty() {
            let others: Vec<_> = character_rows
                .iter()
                .filter(|r| {
                    if let Some(p) = protagonist {
                        !std::ptr::eq(*r, p)
                    } else {
                        true
                    }
                })
                .take(2)
                .map(|r| {
                    let name = row_text(r, "name").unwrap_or_else(|| {
                        format!("{PLACEHOLDER}require.schema.characters.rows.name]")
                    });
                    let role = row_text(r, "role").unwrap_or_else(|| {
                        format!("{PLACEHOLDER}require.schema.characters.rows.role]")
                    });
                    format!("{name} ({role})")
                })
                .collect();
            if others.is_empty() {
                format!("Tuyến đối trọng: {PLACEHOLDER}optional.schema.relationships.rows]")
            } else {
                format!("Tuyến nhân vật khác: {}", others.join(", "))
            }
        } else {
            let rendered: Vec<_> = counter_characters
                .iter()
                .map(|r| {
                    let name = row_text(r, "name").unwrap_or_else(|| {
                        format!("{PLACEHOLDER}require.schema.characters.rows.name]")
                    });
                    let role = row_text(r, "role").unwrap_or_else(|| {
                        format!("{PLACEHOLDER}require.schema.characters.rows.role]")
                    });
                    let archetype = row_text(r, "archetype").unwrap_or_else(|| {
                        format!("{PLACEHOLDER}require.schema.characters.rows.archetype]")
                    });
                    format!("{name} (Vai trò: {role}, Hình mẫu: {archetype})")
                })
                .collect();
            format!("Tuyến đối trọng: {}", rendered.join(", "))
        };

        format!("{protag_str}. {counter_str}.")
    };

    // ── Đoạn 4: Motif chủ đạo (name, scope, description, significance)
    let motif_rows = rows(document, "project_motifs");
    let p4 = if motif_rows.is_empty() {
        format!("Motif chủ đạo: {PLACEHOLDER}require.schema.project_motifs.rows].")
    } else {
        let mut parts = Vec::new();
        for m in motif_rows.iter().take(3) {
            let name = row_text(m, "name")
                .or_else(|| row_text(m, "motif_id"))
                .unwrap_or_else(|| {
                    format!("{PLACEHOLDER}require.schema.project_motifs.rows.name]")
                });
            let scope = row_text(m, "scope").unwrap_or_else(|| {
                format!("{PLACEHOLDER}require.schema.project_motifs.rows.scope]")
            });
            let desc = row_text(m, "description").unwrap_or_else(|| {
                format!("{PLACEHOLDER}require.schema.project_motifs.rows.description]")
            });
            let sig = row_text(m, "significance").unwrap_or_else(|| {
                format!("{PLACEHOLDER}require.schema.project_motifs.rows.significance]")
            });
            parts.push(format!(
                "{name} (Phạm vi: {scope}, Miêu tả: {desc}, Ý nghĩa: {sig})"
            ));
        }
        format!("Motif chủ đạo: {}.", parts.join("; "))
    };

    // ── Đoạn 5: Theme (name, statement, weight)
    let theme_rows = rows(document, "themes");
    let p5 = if theme_rows.is_empty() {
        format!("Chủ đề: {PLACEHOLDER}require.schema.themes.rows].")
    } else {
        let mut parts = Vec::new();
        for t in theme_rows.iter().take(3) {
            let name = row_text(t, "name")
                .unwrap_or_else(|| format!("{PLACEHOLDER}require.schema.themes.rows.name]"));
            let statement = row_text(t, "statement")
                .unwrap_or_else(|| format!("{PLACEHOLDER}require.schema.themes.rows.statement]"));
            let weight = t.get("weight").and_then(Value::as_f64);
            let weight_str = or_path_num(weight, "require.schema.themes.rows.weight");
            parts.push(format!(
                "{name} (Thông điệp: {statement}, Trọng số: {weight_str})"
            ));
        }
        format!("Chủ đề: {}.", parts.join("; "))
    };

    // ── Đoạn 6: Xung đột tư tưởng (character, claim, polarity, counter_claim)
    let arg_rows = rows(document, "theme_arguments");
    let p6 = if arg_rows.is_empty() {
        format!("Xung đột tư tưởng: {PLACEHOLDER}require.schema.theme_arguments.rows].")
    } else {
        let mut parts = Vec::new();
        for a in arg_rows.iter().take(3) {
            let character = row_text(a, "character_id").unwrap_or_else(|| {
                format!("{PLACEHOLDER}require.schema.theme_arguments.rows.character_id]")
            });
            let claim = row_text(a, "claim").unwrap_or_else(|| {
                format!("{PLACEHOLDER}require.schema.theme_arguments.rows.claim]")
            });
            let polarity = row_text(a, "polarity").unwrap_or_else(|| {
                format!("{PLACEHOLDER}require.schema.theme_arguments.rows.polarity]")
            });
            let counter = row_text(a, "counter_claim").unwrap_or_else(|| {
                format!("{PLACEHOLDER}require.schema.theme_arguments.rows.counter_claim]")
            });
            parts.push(format!(
                "Nhân vật {character}: Luận điểm \"{claim}\" ({polarity}) đối kháng \"{counter}\""
            ));
        }
        format!("Xung đột tư tưởng: {}.", parts.join("; "))
    };

    format!("{p1}\n\n{p2}\n\n{p3}\n\n{p4}\n\n{p5}\n\n{p6}")
}

/// Generates a compact 1-paragraph summary under 600 characters for subagent prompt injection.
pub fn anchor_compact(document: &Value) -> String {
    let name = project_text(document, "require", "name").unwrap_or_else(|| "Dự án".into());
    let genre_rows = rows(document, "project_genres");
    let primary_genre = genre_rows
        .iter()
        .find(|r| r.get("is_primary").and_then(Value::as_bool) == Some(true))
        .or_else(|| genre_rows.first())
        .and_then(|r| row_text(r, "genre_id"))
        .unwrap_or_else(|| "Chưa định hình thể loại".into());

    let char_rows = rows(document, "characters");
    let protag_name = char_rows
        .first()
        .and_then(|r| row_text(r, "name"))
        .unwrap_or_else(|| "Nhân vật chính".into());

    let theme_rows = rows(document, "themes");
    let theme_name = theme_rows
        .first()
        .and_then(|r| row_text(r, "name"))
        .unwrap_or_else(|| "Chủ đề trung tâm".into());

    let arc_count = project_int(document, "require", "plan.target-arcs").unwrap_or(0);
    let chap_count = project_int(document, "require", "plan.target-chapters").unwrap_or(0);

    let mut compact = format!(
        "Dự án {name}, thể loại {primary_genre}, nhân vật chính {protag_name}, chủ đề {theme_name}. Quy mô dự kiến: {arc_count} arc, {chap_count} chương."
    );

    if compact.chars().count() > 600 {
        compact = compact.chars().take(597).collect::<String>() + "...";
    }
    compact
}

/// Returns a structured JSON object with stable keys matching the contract.
pub fn anchor_json(document: &Value) -> Value {
    let name = project_text(document, "require", "name");
    let language = project_text(document, "require", "language");

    let genre_rows = rows(document, "project_genres");
    let primary_genre_row = genre_rows
        .iter()
        .find(|r| r.get("is_primary").and_then(Value::as_bool) == Some(true))
        .or_else(|| genre_rows.first());

    let genre = primary_genre_row.and_then(|r| row_text(r, "genre_id"));
    let genre_code = primary_genre_row.and_then(|r| row_text(r, "genre_code"));

    let word = project_int(document, "require", "plan.target-words");
    let arc = project_int(document, "require", "plan.target-arcs");
    let chapter = project_int(document, "require", "plan.target-chapters");
    let scene = project_int(document, "optional", "plan.target-scenes");

    let arc_rows = rows(document, "arcs");
    let arcs_json: Vec<Value> = arc_rows
        .iter()
        .map(|r| {
            json!({
                "name": r.get("name").unwrap_or(&Value::Null),
                "goal": r.get("goal").unwrap_or(&Value::Null),
                "outcome": r.get("outcome").unwrap_or(&Value::Null),
                "summarize": r.get("summarize").unwrap_or(&Value::Null),
            })
        })
        .collect();

    let char_rows = rows(document, "characters");
    let protag_row = char_rows
        .iter()
        .find(|r| r.get("is_primary").and_then(Value::as_bool) == Some(true))
        .or_else(|| char_rows.first());

    let protagonist_json = match protag_row {
        Some(p) => json!({
            "name": p.get("name").unwrap_or(&Value::Null),
            "role": p.get("role").unwrap_or(&Value::Null),
            "archetype": p.get("archetype").unwrap_or(&Value::Null),
            "summary": p.get("summary").unwrap_or(&Value::Null),
            "description": p.get("description").unwrap_or(&Value::Null),
        }),
        None => json!({
            "name": Value::Null,
            "role": Value::Null,
            "archetype": Value::Null,
            "summary": Value::Null,
            "description": Value::Null,
        }),
    };

    let cast_json: Vec<Value> = char_rows
        .iter()
        .map(|r| {
            json!({
                "name": r.get("name").unwrap_or(&Value::Null),
                "role": r.get("role").unwrap_or(&Value::Null),
                "archetype": r.get("archetype").unwrap_or(&Value::Null),
                "summary": r.get("summary").unwrap_or(&Value::Null),
                "description": r.get("description").unwrap_or(&Value::Null),
            })
        })
        .collect();

    let motif_rows = rows(document, "project_motifs");
    let motif_json = match motif_rows.first() {
        Some(m) => json!({
            "name": m.get("name").unwrap_or(&Value::Null),
            "scope": m.get("scope").unwrap_or(&Value::Null),
            "description": m.get("description").unwrap_or(&Value::Null),
            "significance": m.get("significance").unwrap_or(&Value::Null),
        }),
        None => json!({
            "name": Value::Null,
            "scope": Value::Null,
            "description": Value::Null,
            "significance": Value::Null,
        }),
    };

    let theme_rows = rows(document, "themes");
    let theme_json = match theme_rows.first() {
        Some(t) => json!({
            "name": t.get("name").unwrap_or(&Value::Null),
            "statement": t.get("statement").unwrap_or(&Value::Null),
            "weight": t.get("weight").unwrap_or(&Value::Null),
        }),
        None => json!({
            "name": Value::Null,
            "statement": Value::Null,
            "weight": Value::Null,
        }),
    };

    let arg_rows = rows(document, "theme_arguments");
    let argument_json = match arg_rows.first() {
        Some(a) => json!({
            "character_id": a.get("character_id").unwrap_or(&Value::Null),
            "claim": a.get("claim").unwrap_or(&Value::Null),
            "polarity": a.get("polarity").unwrap_or(&Value::Null),
            "counter_claim": a.get("counter_claim").unwrap_or(&Value::Null),
        }),
        None => json!({
            "character_id": Value::Null,
            "claim": Value::Null,
            "polarity": Value::Null,
            "counter_claim": Value::Null,
        }),
    };

    json!({
        "project": {
            "name": name,
            "language": language,
            "genre": genre,
            "genre_code": genre_code,
            "plan": {
                "word": word,
                "arc": arc,
                "chapter": chapter,
                "scene": scene,
            }
        },
        "arcs": arcs_json,
        "protagonist": protagonist_json,
        "cast": cast_json,
        "motif": motif_json,
        "theme": theme_json,
        "argument": argument_json,
    })
}

/// Loads project super.json and produces anchor JSON object containing both text and json views.
pub fn anchor_for_project(project_id: &str) -> Result<Value, String> {
    let document = crate::super_json::load_or_init(project_id)?;
    let text = anchor(&document);
    let compact = anchor_compact(&document);
    let json_val = anchor_json(&document);
    Ok(json!({
        "ok": true,
        "project_id": project_id,
        "text": text,
        "compact": compact,
        "json": json_val,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anchor_has_six_paragraphs_and_placeholders() {
        let skeleton = crate::super_json::skeleton();
        let text = anchor(&skeleton);
        let paragraphs: Vec<&str> = text.split("\n\n").collect();
        assert_eq!(paragraphs.len(), 6, "Must have exactly 6 paragraphs");
        assert!(text.contains("[chưa có: "));
        assert!(!text.contains("–"));
        assert!(!text.contains("—"));
    }

    #[test]
    fn test_anchor_compact_length() {
        let skeleton = crate::super_json::skeleton();
        let compact = anchor_compact(&skeleton);
        assert!(compact.chars().count() <= 600);
        assert!(!compact.contains("–"));
        assert!(!compact.contains("—"));
    }

    #[test]
    fn test_anchor_json_has_stable_keys() {
        let skeleton = crate::super_json::skeleton();
        let res = anchor_json(&skeleton);
        assert!(res.get("project").is_some());
        assert!(res["project"].get("plan").is_some());
        assert!(res.get("arcs").is_some());
        assert!(res.get("protagonist").is_some());
        assert!(res.get("cast").is_some());
        assert!(res.get("motif").is_some());
        assert!(res.get("theme").is_some());
        assert!(res.get("argument").is_some());
    }
}
