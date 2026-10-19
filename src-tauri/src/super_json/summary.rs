//! The super json rendered as six deterministic Vietnamese paragraphs.

use serde_json::Value;

use super::{progress::unit_counts, schema};

const PLACEHOLDER: &str = "[chưa có thông tin]";

const LANGUAGE_LABELS: &[(&str, &str)] = &[
    ("VI", "tiếng Việt"),
    ("EN", "tiếng Anh"),
    ("JA", "tiếng Nhật"),
    ("ZH_HANT", "tiếng Trung phồn thể"),
    ("ZH_HANS", "tiếng Trung giản thể"),
];

const RESOLUTION_LABELS: &[(&str, &str)] = &[
    ("RESOLVED", "giải quyết rõ ràng"),
    ("OPEN", "để ngỏ"),
    ("AMBIGUOUS", "mơ hồ"),
];

const SCALE_LABELS: &[(&str, &str)] = &[
    ("REALM", "cõi"),
    ("WORLD", "thế giới"),
    ("REGION", "vùng"),
    ("COUNTRY", "quốc gia"),
    ("CITY", "thành phố"),
    ("TOWN", "thị trấn"),
    ("FACILITY", "công trình"),
    ("STREET", "đường phố"),
    ("ROOM", "căn phòng"),
];

fn project_text(document: &Value, group: schema::Group, leaf: &str) -> Option<String> {
    let mut cursor = document.get(group.as_str())?.get("project")?;
    for segment in leaf.split('.') {
        cursor = cursor.get(segment)?;
    }
    cursor
        .as_str()
        .map(str::trim)
        .filter(|text| !text.is_empty())
        .map(str::to_owned)
}

fn project_int(document: &Value, group: schema::Group, leaf: &str) -> Option<i64> {
    let mut cursor = document.get(group.as_str())?.get("project")?;
    for segment in leaf.split('.') {
        cursor = cursor.get(segment)?;
    }
    cursor.as_i64()
}

fn rows(document: &Value, table: &str) -> Vec<Value> {
    schema::countable_entry(document, table)
        .and_then(|entry| entry.get("rows"))
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default()
}

fn row_text(row: &Value, field: &str) -> Option<String> {
    row.get(field)
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|text| !text.is_empty())
        .map(str::to_owned)
}

fn or_placeholder(value: Option<String>) -> String {
    value.unwrap_or_else(|| PLACEHOLDER.to_owned())
}

fn enum_label(value: Option<String>, table: &[(&str, &str)]) -> Option<String> {
    let value = value?;
    table
        .iter()
        .find(|(code, _)| *code == value)
        .map(|(_, label)| (*label).to_owned())
        .or(Some(value))
}

fn with_thousands(number: i64) -> String {
    let digits = number.to_string();
    let mut out = String::new();
    for (index, character) in digits.chars().rev().enumerate() {
        if index > 0 && index % 3 == 0 {
            out.push('.');
        }
        out.push(character);
    }
    out.chars().rev().collect()
}

fn count_or_placeholder(value: Option<i64>) -> String {
    value.map_or_else(|| PLACEHOLDER.to_owned(), with_thousands)
}

/// Six fixed paragraphs: identity, cast, theme, plan, world, status.
pub fn summary(document: &Value) -> String {
    // ── 1. Identity ──────────────────────────────────────────────────────
    let primary_genre = rows(document, "project_genres")
        .into_iter()
        .find(|row| row["is_primary"].as_bool() == Some(true))
        .and_then(|row| row_text(&row, "genre_id"));
    let paragraph_one = format!(
        "Tên dự án: {}. Ngôn ngữ: {}. Thể loại chính: {}. Tác giả: {}. Đối tượng độc giả: {}.",
        or_placeholder(project_text(document, schema::Group::Require, "name")),
        or_placeholder(enum_label(
            project_text(document, schema::Group::Require, "language"),
            LANGUAGE_LABELS
        )),
        or_placeholder(primary_genre),
        or_placeholder(project_text(document, schema::Group::Optional, "author")),
        or_placeholder(project_text(
            document,
            schema::Group::Optional,
            "target-audience"
        )),
    );

    // ── 2. Cast and relationships ────────────────────────────────────────
    let cast = rows(document, "characters");
    let cast_text = if cast.is_empty() {
        PLACEHOLDER.to_owned()
    } else {
        cast.iter()
            .map(|row| {
                let name = row_text(row, "name").unwrap_or_else(|| PLACEHOLDER.to_owned());
                if row["is_primary"].as_bool() == Some(true) {
                    format!("{name} (chính)")
                } else {
                    name
                }
            })
            .collect::<Vec<_>>()
            .join(", ")
    };
    let relationship_count = rows(document, "relationships").len();
    let paragraph_two =
        format!("Dàn nhân vật: {cast_text}. Số quan hệ đã khai báo: {relationship_count}.");

    // ── 3. Theme ─────────────────────────────────────────────────────────
    let theme = rows(document, "themes")
        .into_iter()
        .next()
        .unwrap_or(Value::Null);
    let theme_question = row_text(&theme, "question");
    let theme_message = row_text(&theme, "message");
    let theme_resolution = row_text(&theme, "resolution");
    let theme_mode = enum_label(row_text(&theme, "resolution_type"), RESOLUTION_LABELS);
    let paragraph_three = format!(
        "Chủ đề: {}. Thông điệp: {}. Cách giải quyết: {}, với nội dung: {}. Số luận điểm chủ đề: {}.",
        or_placeholder(theme_question),
        or_placeholder(theme_message),
        or_placeholder(theme_mode),
        or_placeholder(theme_resolution),
        rows(document, "theme_arguments").len(),
    );

    // ── 4. Plan ──────────────────────────────────────────────────────────
    let arc = project_int(document, schema::Group::Require, "plan.arc.count");
    let chapter = project_int(document, schema::Group::Require, "plan.chapter.count");
    let scene = project_int(document, schema::Group::Require, "plan.scene.count");
    let word = project_int(document, schema::Group::Require, "plan.word.count");
    let arc_tolerance = project_int(document, schema::Group::Optional, "plan.arc.tolerance");
    let chapter_tolerance =
        project_int(document, schema::Group::Optional, "plan.chapter.tolerance");
    let scene_tolerance = project_int(document, schema::Group::Optional, "plan.scene.tolerance");
    let word_tolerance = project_int(document, schema::Group::Optional, "plan.word.tolerance");
    let tolerance = if arc_tolerance.is_some()
        || chapter_tolerance.is_some()
        || scene_tolerance.is_some()
        || word_tolerance.is_some()
    {
        format!(
            " Sai số chấp nhận: {} arc, {} chương, {} cảnh, {} từ.",
            count_or_placeholder(arc_tolerance),
            count_or_placeholder(chapter_tolerance),
            count_or_placeholder(scene_tolerance),
            count_or_placeholder(word_tolerance),
        )
    } else {
        String::new()
    };
    let paragraph_four = format!(
        "Kế hoạch: {} arc, {} chương, {} cảnh, {} từ.{} Đã khai báo: {} arc, {} chương, {} cảnh.",
        count_or_placeholder(arc),
        count_or_placeholder(chapter),
        count_or_placeholder(scene),
        count_or_placeholder(word),
        tolerance,
        rows(document, "arcs").len(),
        rows(document, "chapters").len(),
        rows(document, "scenes").len(),
    );

    // ── 5. World ─────────────────────────────────────────────────────────
    let world = rows(document, "world_settings")
        .into_iter()
        .next()
        .unwrap_or(Value::Null);
    let world_name = row_text(&world, "name");
    let world_scale = enum_label(row_text(&world, "type"), SCALE_LABELS);
    let powers = rows(document, "power_systems");
    let power_text = match powers.first() {
        Some(row) => {
            let name = row_text(row, "name").unwrap_or_else(|| PLACEHOLDER.to_owned());
            let source = row_text(row, "source").unwrap_or_else(|| PLACEHOLDER.to_owned());
            format!("{name} (nguồn: {source})")
        }
        None => "không có".to_owned(),
    };
    let paragraph_five = format!(
        "Bối cảnh: {}, quy mô {}. Hệ thống sức mạnh: {}. Số lore: {}.",
        or_placeholder(world_name),
        or_placeholder(world_scale),
        power_text,
        rows(document, "lore_entries").len(),
    );

    // ── 6. Status ────────────────────────────────────────────────────────
    let (filled_required, filled_optional) = unit_counts(document);
    let paragraph_six = format!(
        "Tóm tắt: {}. Đã hoàn thành {filled_required}/{} mục bắt buộc và {filled_optional}/{} mục tùy chọn.",
        or_placeholder(project_text(document, schema::Group::Optional, "summarize")),
        schema::required_paths().len(),
        schema::optional_paths().len(),
    );

    [
        paragraph_one,
        paragraph_two,
        paragraph_three,
        paragraph_four,
        paragraph_five,
        paragraph_six,
    ]
    .join("\n\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::super_json::io::set;
    use serde_json::json;

    fn filled() -> Value {
        let mut document = schema::skeleton();
        let project = "require.project";
        for (path, value) in [
            (format!("{project}.name"), json!("Nửa Đêm")),
            (format!("{project}.language"), json!("VI")),
            (format!("{project}.plan.arc.count"), json!(5)),
            (format!("{project}.plan.chapter.count"), json!(120)),
            (format!("{project}.plan.scene.count"), json!(360)),
            (format!("{project}.plan.word.count"), json!(90000)),
        ] {
            assign(&mut document, &path, value);
        }
        assign_table(
            &mut document,
            "require",
            "project_genres",
            "count",
            json!(1),
        );
        assign_table(
            &mut document,
            "require",
            "project_genres",
            "rows",
            json!([{ "genre_id": "trinh thám", "is_primary": true }]),
        );
        assign_table(&mut document, "require", "characters", "count", json!(3));
        assign_table(
            &mut document,
            "require",
            "characters",
            "rows",
            json!([
                { "name": "Hoàng", "is_primary": true },
                { "name": "Phong", "is_primary": false },
                { "name": "Linh", "is_primary": false }
            ]),
        );
        assign_table(&mut document, "require", "themes", "count", json!(1));
        assign_table(
            &mut document,
            "require",
            "themes",
            "rows",
            json!([{ "scope": "PROJECT", "question": "ký ức có làm nên con người?" }]),
        );
        document
    }

    fn assign_table(document: &mut Value, group: &str, table: &str, key: &str, value: Value) {
        let entry = document[group]["schema"]
            .as_array_mut()
            .unwrap()
            .iter_mut()
            .find(|entry| entry["table"] == table)
            .unwrap();
        entry[key] = value;
    }

    fn assign(document: &mut Value, path: &str, value: Value) {
        let segments: Vec<&str> = path.split('.').collect();
        let (last, parents) = segments.split_last().unwrap();
        let mut cursor = document;
        for segment in parents {
            cursor = cursor.get_mut(*segment).unwrap();
        }
        cursor[*last] = value;
    }

    #[test]
    fn test_summary_has_six_paragraphs_and_placeholders() {
        let text = summary(&schema::skeleton());
        assert_eq!(text.split("\n\n").count(), 6, "{text}");
        assert!(text.contains(PLACEHOLDER));
        assert!(!text.contains("null"));
    }

    #[test]
    fn test_summary_lists_all_three_characters() {
        let text = summary(&filled());
        assert!(text.contains("Hoàng (chính)"), "{text}");
        assert!(text.contains("Phong"), "{text}");
        assert!(text.contains("Linh"), "{text}");
        assert!(text.contains("trinh thám"));
        assert!(text.contains("ký ức có làm nên con người?"));
        assert!(text.contains("90.000"));
    }

    #[test]
    fn test_summary_is_deterministic() {
        assert_eq!(summary(&filled()), summary(&filled()));
    }

    #[test]
    fn test_summary_uses_new_schema_paths() {
        let _home = crate::test_support::TempHome::new("summary-rows");
        crate::super_json::load_or_init("p").unwrap();
        set("p", "require.project.name", json!("A")).unwrap();
        let document = crate::super_json::load_or_init("p").unwrap();
        assert!(summary(&document).contains("Tên dự án: A"));
    }
}
