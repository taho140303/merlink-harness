//! Vietnamese labels for the super json paths and the tool log.

use serde_json::Value;

/// Project leaves (13). The schema half is composed from table + operation.
const PROJECT_LABELS: [(&str, &str); 13] = [
    ("require.project.name", "tên truyện"),
    ("require.project.language", "ngôn ngữ"),
    ("require.project.plan.arc.count", "số arc"),
    ("require.project.plan.chapter.count", "số chương"),
    ("require.project.plan.scene.count", "số cảnh"),
    ("require.project.plan.word.count", "số từ"),
    ("optional.project.author", "tác giả"),
    ("optional.project.summarize", "tóm tắt"),
    ("optional.project.target-audience", "đối tượng độc giả"),
    ("optional.project.plan.arc.tolerance", "sai lệch số arc"),
    (
        "optional.project.plan.chapter.tolerance",
        "sai lệch số chương",
    ),
    ("optional.project.plan.scene.tolerance", "sai lệch số cảnh"),
    ("optional.project.plan.word.tolerance", "sai lệch số từ"),
];

/// The 34 schema tables.
const TABLE_LABELS: [(&str, &str); 34] = [
    ("relationship_motifs", "motif quan hệ"),
    ("project_motifs", "motif dự án"),
    ("project_genres", "thể loại dự án"),
    ("arcs", "arc"),
    ("chapters", "chương"),
    ("scenes", "cảnh"),
    ("scene_characters", "nhân vật trong cảnh"),
    ("scene_versions", "phiên bản cảnh"),
    ("characters", "nhân vật"),
    ("appearance_option_groups", "nhóm ngoại hình"),
    ("appearance_options", "lựa chọn ngoại hình"),
    ("character_appearances", "ngoại hình nhân vật"),
    ("character_distinguishing_marks", "dấu đặc biệt"),
    ("relationships", "quan hệ"),
    ("relationship_categories", "loại quan hệ"),
    ("relationship_roles", "vai trò quan hệ"),
    ("relationship_states", "trạng thái quan hệ"),
    ("world_settings", "bối cảnh"),
    ("lore_entries", "lore"),
    ("power_systems", "hệ thống sức mạnh"),
    ("power_tiers", "bậc sức mạnh"),
    ("power_abilities", "năng lực"),
    ("themes", "chủ đề"),
    ("theme_arguments", "luận điểm chủ đề"),
    ("memories", "ký ức"),
    ("memory_references", "tham chiếu ký ức"),
    ("memory_emotions", "cảm xúc ký ức"),
    ("memory_connections", "liên kết ký ức"),
    ("memory_revisions", "bản sửa ký ức"),
    ("character_knowledge_sources", "nguồn tri thức"),
    ("competence_domains", "lĩnh vực năng lực"),
    ("character_competences", "năng lực nhân vật"),
    ("competence_capabilities", "khả năng năng lực"),
    ("competence_limitations", "giới hạn năng lực"),
];

fn table_label(table: &str) -> Option<&'static str> {
    TABLE_LABELS
        .iter()
        .find(|(known, _)| *known == table)
        .map(|(_, label)| *label)
}

fn split_schema(path: &str) -> Option<(&str, &str, &str)> {
    let (group, rest) = path.split_once('.')?;
    let tail = rest.strip_prefix("schema.")?;
    if group != "require" && group != "optional" {
        return None;
    }
    let (table, kind) = tail.rsplit_once('.')?;
    if kind != "count" && kind != "rows" {
        return None;
    }
    Some((group, table, kind))
}

/// The Vietnamese label for one writable path, or `None` outside the map.
fn path_label(path: &str) -> Option<String> {
    if let Some(label) = PROJECT_LABELS
        .iter()
        .find(|(known, _)| *known == path)
        .map(|(_, label)| *label)
    {
        return Some(label.to_owned());
    }
    let (_, table, kind) = split_schema(path)?;
    let table = table_label(table)?;
    Some(match kind {
        "count" => format!("số lượng {table}"),
        "rows" => format!("danh sách {table}"),
        _ => return None,
    })
}

/// The single source of truth for field labels (Sprint 11.12 · D8).
///
/// `lib.rs` exposes this through the `super_json_labels` command so the frontend
/// can phrase the tool log in Vietnamese without keeping a second copy of the map.
pub fn field_labels() -> Vec<(String, String)> {
    crate::super_json::valid_paths()
        .iter()
        .filter_map(|path| path_label(path).map(|label| (path.clone(), label)))
        .collect()
}

fn table_for_rows(args: &Value) -> Option<&str> {
    let path = args.get("path")?.as_str()?;
    split_schema(path).map(|(_, table, _)| table)
}

/// A ready-to-render Vietnamese phase message for one tool call.
pub fn status_message(name: &str, args: &Value) -> String {
    match name {
        "project_create" => {
            let named = args
                .get("name")
                .and_then(Value::as_str)
                .map(str::trim)
                .is_some_and(|value| !value.is_empty());
            if named {
                "Đang ghi nhận tên truyện và tạo project".to_owned()
            } else {
                "Đang tạo project".to_owned()
            }
        }
        "super_json_missing" => "Đang kiểm tra các mục còn thiếu".to_owned(),
        "super_json_get" => "Đang đọc siêu JSON".to_owned(),
        "super_json_add_row" => {
            let table = args.get("table").and_then(Value::as_str).unwrap_or("");
            let label = table_label(table).unwrap_or(table);
            format!("Đang thêm 1 {label}")
        }
        "super_json_add_field" => {
            let table = args.get("table").and_then(Value::as_str).unwrap_or("");
            let label = table_label(table).unwrap_or(table);
            format!("Đang cập nhật {label}")
        }
        "super_json_set" => {
            let path = args.get("path").and_then(Value::as_str).unwrap_or("");
            if let Some(label) = path_label(path) {
                if split_schema(path).map(|(_, _, kind)| kind) == Some("rows") {
                    let count = args
                        .get("value")
                        .and_then(Value::as_array)
                        .map_or(0, Vec::len);
                    let table = table_for_rows(args)
                        .and_then(table_label)
                        .unwrap_or(label.as_str());
                    return format!("Đang ghi nhận {count} {table}");
                }
                return format!("Đang ghi nhận {label}");
            }
            format!("Đang ghi nhận {path}")
        }
        "echo" => "Đang kiểm tra kết nối tool".to_owned(),
        "outline" => "Đang đọc khối dàn ý".to_owned(),
        other => format!("Đang chạy {other}"),
    }
}

/// The super json path a tool call addresses, when it has one. Used for the
/// `path` field of `ToolStarted` so the UI can label the phase precisely.
pub fn tool_path(name: &str, args: &Value) -> Option<String> {
    match name {
        "super_json_set" | "super_json_get" => {
            args.get("path").and_then(Value::as_str).map(str::to_owned)
        }
        "super_json_add_row" | "super_json_add_field" => {
            let table = args.get("table").and_then(Value::as_str)?;
            crate::super_json::schema_table_group(table)
                .map(|group| format!("{}.schema.{table}", group.as_str()))
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_status_message_covers_all_81_paths() {
        let _home = crate::test_support::TempHome::new("tools-status-paths");
        let paths = crate::super_json::valid_paths();
        assert_eq!(paths.len(), 81, "the whitelist must stay at 81 paths");
        for path in paths {
            assert!(path_label(path).is_some(), "no label for {path}");
        }
        assert_eq!(table_label("characters"), Some("nhân vật"));
    }

    #[test]
    fn test_rows_status_reports_count_without_dumping_array() {
        let message = status_message(
            "super_json_set",
            &json!({
                "path": "require.schema.characters.rows",
                "value": [
                    { "name": "Hoàng", "is_primary": true },
                    { "name": "Phong", "is_primary": false },
                    { "name": "Linh", "is_primary": false }
                ]
            }),
        );
        assert_eq!(message, "Đang ghi nhận 3 nhân vật");
        assert!(!message.contains("Hoàng"));
    }

    #[test]
    fn test_add_tools_have_status_and_table_path() {
        assert_eq!(
            status_message("super_json_add_row", &json!({"table": "characters"})),
            "Đang thêm 1 nhân vật"
        );
        assert_eq!(
            status_message("super_json_add_field", &json!({"table": "scenes"})),
            "Đang cập nhật cảnh"
        );
        assert_eq!(
            tool_path("super_json_add_row", &json!({"table": "characters"})),
            Some("require.schema.characters".to_owned())
        );
        assert_eq!(
            tool_path("super_json_add_field", &json!({"table": "scenes"})),
            Some("optional.schema.scenes".to_owned())
        );
        assert_eq!(tool_path("super_json_add_row", &json!({})), None);
    }

    #[test]
    fn test_labels_command_covers_all_81_paths() {
        let _home = crate::test_support::TempHome::new("tools-labels-path");
        let labels = field_labels();
        assert_eq!(labels.len(), 81);
        for path in crate::super_json::valid_paths() {
            assert!(
                labels.iter().any(|(known, _)| known == path),
                "missing label for {path}"
            );
        }
    }
}
