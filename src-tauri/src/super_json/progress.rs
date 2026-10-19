//! Missing/filled bookkeeping and the two-stop 70/30 progress calculation.

use serde_json::Value;

use super::{
    io::load_or_init,
    schema::{optional_paths, required_paths},
};
use crate::models::SuperJsonProgress;

/// Progress labels, worded exactly as `docs/design/idea.md` §progress describes
/// them. The header renders whatever the backend computes.
pub const LABEL_NO_PROJECT: &str = "Chưa có project";
pub const LABEL_COLLECTING: &str = "Đang thu thập thông tin";
pub const LABEL_ACCEPTABLE: &str = "Độ mạch lạc tạm chấp nhận";
pub const LABEL_GOOD: &str = "Độ mạch lạc: tốt";

/// Required paths still sitting at `null`. An empty result means the outline
/// button may light up.
pub fn missing_required(project_id: &str) -> Result<Vec<String>, String> {
    let document = load_or_init(project_id)?;
    let mut missing = Vec::new();
    for path in required_paths() {
        if !is_filled(&document, path) {
            missing.push(path.clone());
        }
    }
    Ok(missing)
}

pub(super) fn value_at<'a>(document: &'a Value, path: &str) -> Option<&'a Value> {
    let mut cursor = document;
    for segment in path.split('.') {
        cursor = cursor.get(segment)?;
    }
    Some(cursor)
}

/// A field counts as filled once it holds a value. `null` never counts, and an
/// empty array does not either: `optional.genre.secondary` starts as `[]` in the
/// skeleton, which means "no secondary genre named yet", not "answered".
fn is_filled(document: &Value, path: &str) -> bool {
    match value_at(document, path) {
        Some(Value::Null) | None => false,
        Some(Value::Array(items)) => !items.is_empty(),
        Some(_) => true,
    }
}

/// `(filled required, filled optional)` — the raw numbers behind the progress bar.
pub fn filled_counts(project_id: &str) -> Result<(u32, u32), String> {
    let document = load_or_init(project_id)?;
    let count = |paths: &[String]| {
        paths
            .iter()
            .filter(|path| is_filled(&document, path))
            .count() as u32
    };
    Ok((count(required_paths()), count(optional_paths())))
}

/// Two-stop progress (D6): the 11 required paths cover 0 → 70%, the 14 optional
/// paths cover 70 → 100%. Completing every required path lands on exactly 70.
pub fn progress(project_id: &str) -> Result<SuperJsonProgress, String> {
    let required_total = required_paths().len() as u32;
    let optional_total = optional_paths().len() as u32;

    if project_id.trim().is_empty() {
        return Ok(SuperJsonProgress {
            has_project: false,
            project_id: String::new(),
            percent: 0,
            label: LABEL_NO_PROJECT.into(),
            filled_required: 0,
            required_total,
            filled_optional: 0,
            optional_total,
        });
    }

    let (filled_required, filled_optional) = filled_counts(project_id)?;
    let percent = (70.0 * f64::from(filled_required) / f64::from(required_total)
        + 30.0 * f64::from(filled_optional) / f64::from(optional_total))
    .round()
    .clamp(0.0, 100.0) as u8;

    let label = if filled_required == required_total && filled_optional == optional_total {
        LABEL_GOOD
    } else if filled_required == required_total {
        LABEL_ACCEPTABLE
    } else {
        LABEL_COLLECTING
    };

    Ok(SuperJsonProgress {
        has_project: true,
        project_id: project_id.to_owned(),
        percent,
        label: label.into(),
        filled_required,
        required_total,
        filled_optional,
        optional_total,
    })
}

/// A valid value for every required path, so progress tests can complete the
/// 11 blocking fields without restating the type rules.
#[cfg(test)]
pub(super) fn required_value(path: &str) -> Value {
    use serde_json::json;
    match path {
        "require.overview.plan.arc.count" => json!(5),
        "require.overview.plan.chapter.count" => json!(120),
        "require.overview.plan.word.count" => json!(90000),
        _ => json!(format!("value for {path}")),
    }
}

/// Same for the 14 optional paths, including the enum and boolean ones.
#[cfg(test)]
pub(super) fn optional_value(path: &str) -> Value {
    use serde_json::json;
    match path {
        "optional.overview.language" => json!("VI"),
        "optional.theme.resolution-type" => json!("OPEN"),
        "optional.world.setting-scale" => json!("CITY"),
        "optional.world.has-power-system" => json!(true),
        "optional.genre.secondary" => json!(["trinh thám"]),
        "optional.overview.plan.arc.tolerance"
        | "optional.overview.plan.chapter.tolerance"
        | "optional.overview.plan.word.tolerance" => json!(1),
        _ => json!(format!("value for {path}")),
    }
}

#[cfg(test)]
mod tests {
    use super::{
        super::io::set,
        *,
    };
    use crate::paths;
    use serde_json::json;

    fn fill_required(project_id: &str) {
        for path in required_paths() {
            set(project_id, path, required_value(path)).unwrap_or_else(|error| panic!("{path}: {error}"));
        }
    }

    fn fill_optional(project_id: &str) {
        for path in optional_paths() {
            set(project_id, path, optional_value(path)).unwrap_or_else(|error| panic!("{path}: {error}"));
        }
    }

    /// Acceptance: once all 11 required fields hold a value, nothing blocks the
    /// outline step any more.
    #[test]
    fn test_super_json_missing_is_empty_when_all_required_set() {
        let _home = crate::test_support::TempHome::new("super-complete");
        load_or_init("default").unwrap();

        let values: [(&str, Value); 11] = [
            ("require.overview.name", json!("Nửa Đêm")),
            (
                "require.overview.logline.trigger-event",
                json!("chiếc đồng hồ cuối cùng ngừng chạy"),
            ),
            (
                "require.overview.logline.main-character",
                json!("một người thợ đồng hồ mù"),
            ),
            (
                "require.overview.logline.action",
                json!("truy tìm người đã chế tạo ra nó"),
            ),
            ("require.overview.logline.risk", json!("đánh mất ký ức")),
            (
                "require.overview.logline.major-loss",
                json!("đứa con gái duy nhất"),
            ),
            ("require.overview.plan.arc.count", json!(5)),
            ("require.overview.plan.chapter.count", json!(120)),
            ("require.overview.plan.word.count", json!(90000)),
            ("require.genre.primary", json!("trinh thám")),
            (
                "require.theme.question",
                json!("ký ức có làm nên con người?"),
            ),
        ];

        for (path, value) in values {
            set("default", path, value).unwrap_or_else(|error| panic!("{path}: {error}"));
        }

        assert!(
            missing_required("default").unwrap().is_empty(),
            "all required fields were set, nothing should be missing"
        );

        // `null` is not an acceptable value for a text field, so the field stays
        // filled and the document is still complete.
        assert!(set("default", "require.overview.name", json!(null)).is_err());
        assert!(missing_required("default").unwrap().is_empty());
    }

    /// Acceptance: exactly the 11 required fields → exactly 70%.
    #[test]
    fn test_progress_is_seventy_when_required_complete() {
        let _home = crate::test_support::TempHome::new("progress-seventy");
        load_or_init("default").unwrap();
        fill_required("default");

        let progress = progress("default").unwrap();
        assert!(progress.has_project);
        assert_eq!(progress.project_id, "default");
        assert_eq!(progress.filled_required, 11);
        assert_eq!(progress.required_total, 11);
        assert_eq!(progress.filled_optional, 0, "no optional field was touched");
        assert_eq!(progress.optional_total, 14);
        assert_eq!(progress.percent, 70, "11/11 required must land on exactly 70%");
        assert_eq!(progress.label, "Độ mạch lạc tạm chấp nhận");

        assert_eq!(filled_counts("default").unwrap(), (11, 0));
        assert!(missing_required("default").unwrap().is_empty());
    }

    /// Acceptance: all 25 fields → 100%.
    #[test]
    fn test_progress_is_hundred_when_all_fields_filled() {
        let _home = crate::test_support::TempHome::new("progress-hundred");
        load_or_init("default").unwrap();
        fill_required("default");
        fill_optional("default");

        let progress = progress("default").unwrap();
        assert_eq!(progress.filled_required, 11);
        assert_eq!(progress.filled_optional, 14);
        assert_eq!(progress.percent, 100);
        assert_eq!(progress.label, "Độ mạch lạc: tốt");
        assert_eq!(filled_counts("default").unwrap(), (11, 14));
    }

    /// Acceptance: `optional.genre.secondary = []` is not an answer. The skeleton
    /// ships it as `[]`, so counting it would start every project at 2%.
    #[test]
    fn test_progress_counts_empty_array_as_unfilled() {
        let _home = crate::test_support::TempHome::new("progress-empty-array");
        load_or_init("default").unwrap();
        fill_required("default");

        let baseline = progress("default").unwrap();
        assert_eq!(baseline.filled_optional, 0, "the skeleton's [] is not filled");
        assert_eq!(baseline.percent, 70);

        // The validator accepts an empty array (it is a legal value) ...
        set("default", "optional.genre.secondary", json!([])).unwrap();
        let empty = progress("default").unwrap();
        assert_eq!(empty.filled_optional, 0, "an empty array is still not filled");
        assert_eq!(empty.percent, 70);

        // ... and a non-empty one does count.
        set("default", "optional.genre.secondary", json!(["hành động"])).unwrap();
        let filled = progress("default").unwrap();
        assert_eq!(filled.filled_optional, 1);
        assert_eq!(filled.percent, 72, "70 + round(30 * 1 / 14)");
    }

    /// Every label and both stops, driven through the real numbers.
    #[test]
    fn test_progress_label_thresholds() {
        let _home = crate::test_support::TempHome::new("progress-labels");

        // No project at all: the header still knows the totals, but nothing is filled.
        let none = progress("").unwrap();
        assert!(!none.has_project);
        assert_eq!(none.project_id, "");
        assert_eq!(none.percent, 0);
        assert_eq!(none.label, "Chưa có project");
        assert_eq!(none.filled_required, 0);
        assert_eq!(none.filled_optional, 0);
        assert_eq!(none.required_total, 11);
        assert_eq!(none.optional_total, 14);
        // Asking about a project that does not exist must not create a directory.
        assert!(!paths::merlink_root().unwrap().join("projects").exists());

        // A project with nothing filled yet.
        let empty = progress("default").unwrap();
        assert!(empty.has_project);
        assert_eq!(empty.percent, 0);
        assert_eq!(empty.label, "Đang thu thập thông tin");

        // One required field short of the stop.
        let required: Vec<String> = required_paths().to_vec();
        for path in &required[..10] {
            set("default", path, required_value(path)).unwrap();
        }
        let partial = progress("default").unwrap();
        assert_eq!(partial.filled_required, 10);
        assert_eq!(partial.percent, 64, "round(70 * 10 / 11)");
        assert_eq!(partial.label, "Đang thu thập thông tin");

        // Exactly 11/11: the outline button threshold.
        set("default", &required[10], required_value(&required[10])).unwrap();
        let threshold = progress("default").unwrap();
        assert_eq!(threshold.percent, 70);
        assert_eq!(threshold.label, "Độ mạch lạc tạm chấp nhận");

        // All 25.
        fill_optional("default");
        let full = progress("default").unwrap();
        assert_eq!(full.percent, 100);
        assert_eq!(full.label, "Độ mạch lạc: tốt");
    }
}
