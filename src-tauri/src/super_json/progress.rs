//! Missing/filled bookkeeping and the two-stop 70/30 progress calculation.
//!
//! A unit is one decision: a `project` leaf, or one schema table (its `count`
//! plus a complete `rows` array). 12 required units cover 0 → 70%; 35 optional
//! units cover 70 → 100%.

use serde_json::Value;

use super::{
    io::load_or_init,
    schema::{self, Group, PathRef},
    tables,
};
use crate::models::SuperJsonProgress;

/// Progress labels, worded exactly as `docs/design/idea.md` §progress describes
/// them. The header renders whatever the backend computes.
pub const LABEL_NO_PROJECT: &str = "Chưa có project";
pub const LABEL_COLLECTING: &str = "Đang thu thập thông tin";
pub const LABEL_ACCEPTABLE: &str = "Độ mạch lạc tạm chấp nhận";
pub const LABEL_GOOD: &str = "Độ mạch lạc: tốt";

/// Required units still unfilled. An empty result means the outline button may
/// light up.
pub fn missing_required(project_id: &str) -> Result<Vec<String>, String> {
    let document = load_or_init(project_id)?;
    let mut missing = Vec::new();
    for unit in schema::required_paths() {
        if !is_unit_filled(&document, unit) {
            missing.push(unit.clone());
        }
    }
    Ok(missing)
}

fn value_at<'a>(document: &'a Value, path: &str) -> Option<&'a Value> {
    let mut cursor = document;
    for segment in path.split('.') {
        cursor = cursor.get(segment)?;
    }
    Some(cursor)
}

/// `null` never counts; an empty array does not either. Scalars and non-empty
/// arrays do.
fn is_filled_value(value: &Value) -> bool {
    match value {
        Value::Null => false,
        Value::Array(items) => !items.is_empty(),
        _ => true,
    }
}

/// One unit is filled when its project leaf has a value, or when the table has a
/// declared count and a matching, valid `rows` array.
pub(super) fn is_unit_filled(document: &Value, unit: &str) -> bool {
    if unit.contains(".project.") {
        return value_at(document, unit).is_some_and(is_filled_value);
    }
    let Some(PathRef::TableSelector { group, table }) = schema::parse_path(unit) else {
        return false;
    };
    let result = match group {
        Group::Require => tables::is_table_complete(document, table),
        Group::Optional => tables::is_optional_table_complete(document, table),
    };
    result.unwrap_or(false)
}

/// `(filled required, filled optional)` — the raw numbers behind the progress bar.
pub fn filled_counts(project_id: &str) -> Result<(u32, u32), String> {
    let document = load_or_init(project_id)?;
    let count = |units: &[String]| {
        units
            .iter()
            .filter(|unit| is_unit_filled(&document, unit))
            .count() as u32
    };
    Ok((
        count(schema::required_paths()),
        count(schema::optional_paths()),
    ))
}

/// Counts filled units for a document the caller already loaded (used by the
/// deterministic prose summary).
pub(super) fn unit_counts(document: &Value) -> (u32, u32) {
    let count = |units: &[String]| {
        units
            .iter()
            .filter(|unit| is_unit_filled(document, unit))
            .count() as u32
    };
    (
        count(schema::required_paths()),
        count(schema::optional_paths()),
    )
}

/// Two-stop progress: the 12 required units cover 0 → 70%, the 35 optional
/// units cover 70 → 100%. Completing every required unit lands on exactly 70.
pub fn progress(project_id: &str) -> Result<SuperJsonProgress, String> {
    let required_total = schema::required_paths().len() as u32;
    let optional_total = schema::optional_paths().len() as u32;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::super_json::io::set;
    use serde_json::json;

    fn fill_required(project_id: &str) {
        set(project_id, "require.project.name", json!("Nửa Đêm")).unwrap();
        set(project_id, "require.project.language", json!("VI")).unwrap();
        for (leaf, value) in [
            ("plan.arc.count", 5),
            ("plan.chapter.count", 120),
            ("plan.scene.count", 360),
            ("plan.word.count", 90000),
        ] {
            set(project_id, &format!("require.project.{leaf}"), json!(value)).unwrap();
        }
        let rows = [
            (
                "project_genres",
                json!([{ "genre_id": "trinh thám", "is_primary": true }]),
            ),
            ("arcs", json!([{ "name": "Mưa", "position": 1 }])),
            (
                "characters",
                json!([
                    { "name": "Hoàng", "is_primary": true },
                    { "name": "Phong", "is_primary": false },
                    { "name": "Linh", "is_primary": false }
                ]),
            ),
            (
                "project_motifs",
                json!([{ "motif_id": "mo-tif", "scope": "project" }]),
            ),
            (
                "themes",
                json!([{ "scope": "PROJECT", "question": "ký ức có làm nên con người?" }]),
            ),
            (
                "theme_arguments",
                json!([{ "theme_id": "t1", "character_id": "c1", "role": "PRIMARY" }]),
            ),
        ];
        for (table, rows) in rows {
            let count = rows.as_array().unwrap().len() as i64;
            set(
                project_id,
                &format!("require.schema.{table}.count"),
                json!(count),
            )
            .unwrap();
            set(project_id, &format!("require.schema.{table}.rows"), rows).unwrap();
        }
    }

    #[test]
    fn test_progress_is_seventy_when_required_complete() {
        let _home = crate::test_support::TempHome::new("progress-70");
        crate::super_json::load_or_init("p").unwrap();
        fill_required("p");
        let progress = progress("p").unwrap();
        assert_eq!(progress.filled_required, 12, "{progress:?}");
        assert_eq!(progress.required_total, 12);
        assert_eq!(progress.percent, 70);
        assert_eq!(progress.label, LABEL_ACCEPTABLE);
        assert!(missing_required("p").unwrap().is_empty());
    }

    #[test]
    fn test_progress_counts_table_only_when_count_and_rows_agree() {
        let _home = crate::test_support::TempHome::new("progress-unit");
        crate::super_json::load_or_init("p").unwrap();
        set("p", "require.schema.characters.count", json!(3)).unwrap();
        assert_eq!(filled_counts("p").unwrap().0, 0);
        set(
            "p",
            "require.schema.characters.rows",
            json!([
                { "name": "Hoàng", "is_primary": true },
                { "name": "Phong", "is_primary": false },
                { "name": "Linh", "is_primary": false }
            ]),
        )
        .unwrap();
        assert_eq!(filled_counts("p").unwrap().0, 1);
    }

    #[test]
    fn test_progress_is_hundred_when_all_units_filled() {
        let _home = crate::test_support::TempHome::new("progress-100");
        crate::super_json::load_or_init("p").unwrap();
        fill_required("p");
        for leaf in [
            "author",
            "summarize",
            "target-audience",
            "plan.arc.tolerance",
            "plan.chapter.tolerance",
            "plan.scene.tolerance",
            "plan.word.tolerance",
        ] {
            let value = if leaf.contains("tolerance") {
                json!(1)
            } else {
                json!("x")
            };
            set("p", &format!("optional.project.{leaf}"), value).unwrap();
        }
        for table in [
            "relationship_motifs",
            "chapters",
            "scenes",
            "scene_characters",
            "scene_versions",
            "appearance_option_groups",
            "appearance_options",
            "character_appearances",
            "character_distinguishing_marks",
            "relationships",
            "relationship_categories",
            "relationship_roles",
            "relationship_states",
            "world_settings",
            "lore_entries",
            "power_systems",
            "power_tiers",
            "power_abilities",
            "memories",
            "memory_references",
            "memory_emotions",
            "memory_connections",
            "memory_revisions",
            "character_knowledge_sources",
            "competence_domains",
            "character_competences",
            "competence_capabilities",
            "competence_limitations",
        ] {
            set("p", &format!("optional.schema.{table}.count"), json!(0)).unwrap();
        }
        let progress = progress("p").unwrap();
        assert_eq!(progress.filled_required, 12);
        assert_eq!(progress.filled_optional, 35);
        assert_eq!(progress.percent, 100);
        assert_eq!(progress.label, LABEL_GOOD);
    }
}
