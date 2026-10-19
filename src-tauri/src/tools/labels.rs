//! Vietnamese labels for the super json paths and the tool log.

use serde_json::Value;

/// Vietnamese label for every writable super json path (D3). The list is
/// exhaustive over the 25 leaf paths; `test_status_message_covers_all_25_paths`
/// guards that it stays that way.
const PATH_LABELS: [(&str, &str); 25] = [
    ("require.overview.name", "tên truyện"),
    ("require.overview.logline.trigger-event", "sự kiện khởi động"),
    ("require.overview.logline.main-character", "nhân vật chính"),
    ("require.overview.logline.action", "hành động chính"),
    ("require.overview.logline.risk", "rủi ro"),
    ("require.overview.logline.major-loss", "mất mát lớn"),
    ("require.overview.plan.arc.count", "số arc"),
    ("require.overview.plan.chapter.count", "số chương"),
    ("require.overview.plan.word.count", "số từ"),
    ("require.genre.primary", "thể loại chính"),
    ("require.theme.question", "câu hỏi chủ đề"),
    ("optional.overview.description", "mô tả truyện"),
    ("optional.overview.author", "tác giả"),
    ("optional.overview.language", "ngôn ngữ"),
    ("optional.overview.target-audience", "đối tượng độc giả"),
    ("optional.overview.age-rating", "độ tuổi"),
    ("optional.overview.plan.arc.tolerance", "sai lệch số arc"),
    ("optional.overview.plan.chapter.tolerance", "sai lệch số chương"),
    ("optional.overview.plan.word.tolerance", "sai lệch số từ"),
    ("optional.genre.secondary", "thể loại phụ"),
    ("optional.theme.resolution-type", "kiểu giải quyết chủ đề"),
    ("optional.theme.resolution", "câu trả lời chủ đề"),
    ("optional.theme.message", "thông điệp"),
    ("optional.world.has-power-system", "có hệ thống sức mạnh"),
    ("optional.world.setting-scale", "quy mô thế giới"),
];

/// The label for one super json path, or `None` when it is outside the map.
fn path_label(path: &str) -> Option<&'static str> {
    PATH_LABELS
        .iter()
        .find(|(known, _)| *known == path)
        .map(|(_, label)| *label)
}

/// The single source of truth for field labels (Sprint 11.12 · D8).
///
/// `lib.rs` exposes this through the `super_json_labels` command so the frontend
/// can phrase the tool log in Vietnamese without keeping a second copy of the map.
pub fn field_labels() -> Vec<(&'static str, &'static str)> {
    PATH_LABELS.to_vec()
}

/// A ready-to-render Vietnamese phase message for one tool call (D3). It is the
/// `status` carried by both `ToolStarted` and `ToolFinished`, so the UI line never
/// changes wording while the tool runs.
pub fn status_message(name: &str, args: &Value) -> String {
    match name {
        // Sprint 11.12 · D7: an unnamed project must not claim to record a title
        // it never received — the live frame has to stay literally true.
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
        "super_json_missing" => "Đang kiểm tra các trường còn thiếu".to_owned(),
        "super_json_get" => "Đang đọc siêu JSON".to_owned(),
        "super_json_set" => {
            let path = args.get("path").and_then(Value::as_str).unwrap_or("");
            match path_label(path) {
                Some(label) => format!("Đang ghi nhận {label}"),
                // Unreachable for the 25 whitelisted paths; fall back to the path
                // itself rather than inventing a label.
                None => format!("Đang ghi nhận {path}"),
            }
        }
        "echo" => "Đang kiểm tra kết nối tool".to_owned(),
        other => format!("Đang chạy {other}"),
    }
}

/// The super json path a tool call addresses, when it has one. Used for the
/// `path` field of `ToolStarted` so the UI can label the phase precisely.
pub fn tool_path(name: &str, args: &Value) -> Option<String> {
    match name {
        "super_json_set" | "super_json_get" => args
            .get("path")
            .and_then(Value::as_str)
            .map(str::to_owned),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    /// D3: every one of the 25 whitelisted paths must yield a non-empty phase
    /// message, and the labelled ones must read like real Vietnamese.
    #[test]
    fn test_status_message_covers_all_25_paths() {
        let _home = crate::test_support::TempHome::new("tools-status-paths");
        let paths = crate::super_json::valid_paths();
        assert_eq!(paths.len(), 25, "the whitelist must stay at 25 paths");
        assert_eq!(PATH_LABELS.len(), 25, "the label map must cover the whitelist");

        // Every label in the map is non-empty and belongs to a real path.
        for (path, label) in PATH_LABELS {
            assert!(!label.trim().is_empty(), "{path} has an empty label");
            assert!(
                paths.iter().any(|known| known == path),
                "label map mentions a non-whitelisted path: {path}"
            );
        }

        for path in paths {
            let message =
                status_message("super_json_set", &json!({"path": path, "value": "x"}));
            assert!(!message.trim().is_empty(), "{path} produced no message");
            assert!(
                message.starts_with("Đang ghi nhận "),
                "{path} produced an unexpected message: {message}"
            );
            // A whitelisted path must never fall back to itself.
            assert_ne!(
                message,
                format!("Đang ghi nhận {path}"),
                "{path} is missing from PATH_LABELS"
            );
        }

        // At least these five labels must be the expected, meaningful Vietnamese.
        for (path, expected) in [
            ("require.overview.name", "tên truyện"),
            ("require.overview.logline.trigger-event", "sự kiện khởi động"),
            ("require.genre.primary", "thể loại chính"),
            ("require.theme.question", "câu hỏi chủ đề"),
            ("require.overview.plan.arc.count", "số arc"),
            ("optional.world.has-power-system", "có hệ thống sức mạnh"),
        ] {
            let message = status_message("super_json_set", &json!({"path": path, "value": "x"}));
            assert!(
                message.contains(expected),
                "{path} message {message:?} must contain {expected:?}"
            );
        }

        // A path outside the map would fall back to itself (cannot happen with the
        // 25 valid paths, but the fallback must still be non-empty).
        let fallback = status_message("super_json_set", &json!({"path": "require.bogus"}));
        assert_eq!(fallback, "Đang ghi nhận require.bogus");
    }

    /// D3: the non-`super_json_set` tools have fixed messages, and an unknown
    /// tool names itself.
    #[test]
    fn test_status_message_for_meta_tools() {
        let _home = crate::test_support::TempHome::new("tools-status-meta");
        assert_eq!(
            status_message("project_create", &json!({"name": "Nửa Đêm"})),
            "Đang ghi nhận tên truyện và tạo project"
        );
        // Sprint 11.12 · D7: unnamed creation says only what it does.
        assert_eq!(
            status_message("project_create", &json!({})),
            "Đang tạo project"
        );
        assert_eq!(
            status_message("project_create", &json!({"name": "   "})),
            "Đang tạo project"
        );
        assert_eq!(
            status_message("super_json_missing", &json!({})),
            "Đang kiểm tra các trường còn thiếu"
        );
        assert_eq!(
            status_message("super_json_get", &json!({"path": "require.overview.name"})),
            "Đang đọc siêu JSON"
        );
        assert_eq!(
            status_message("echo", &json!({"message": "hi"})),
            "Đang kiểm tra kết nối tool"
        );
        assert_eq!(
            status_message("some_future_tool", &json!({})),
            "Đang chạy some_future_tool"
        );

        // `tool_path` only reports a path for the tools that address one.
        assert_eq!(
            tool_path("super_json_set", &json!({"path": "require.overview.name"})).as_deref(),
            Some("require.overview.name")
        );
        assert_eq!(
            tool_path("super_json_get", &json!({"path": "require.genre.primary"})).as_deref(),
            Some("require.genre.primary")
        );
        assert!(tool_path("project_create", &json!({"name": "x"})).is_none());
        assert!(tool_path("super_json_missing", &json!({})).is_none());
    }

    /// Sprint 11.12 · D8: the command source covers every one of the 25 paths.
    #[test]
    fn test_labels_command_covers_all_25_paths() {
        let _home = crate::test_support::TempHome::new("tools-labels");
        let labels = field_labels();
        assert_eq!(labels.len(), 25, "the label map must cover all 25 paths");
        assert_eq!(
            labels.len(),
            crate::super_json::valid_paths().len(),
            "the label map and the whitelist drifted"
        );

        // Every path is present exactly once, and every label is real Vietnamese.
        for path in crate::super_json::valid_paths() {
            let matches: Vec<&str> = labels
                .iter()
                .filter(|(known, _)| known == path)
                .map(|(_, label)| *label)
                .collect();
            assert_eq!(matches.len(), 1, "{path} must have exactly one label");
            assert!(!matches[0].trim().is_empty(), "{path} has an empty label");
        }
        let by_path: std::collections::BTreeMap<&str, &str> = labels.iter().copied().collect();
        assert_eq!(by_path["require.overview.name"], "tên truyện");
        assert_eq!(by_path["require.overview.plan.word.count"], "số từ");
        assert_eq!(by_path["optional.world.setting-scale"], "quy mô thế giới");
    }
}
