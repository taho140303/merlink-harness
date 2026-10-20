//! Deterministic seed step: it records what the author already said about the
//! genre, so the generators can answer in catalogue codes instead of free text.

use serde_json::Value;

use crate::{
    catalog::{self, Kind},
    super_json,
};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SeedReport {
    pub resolved: usize,
    pub unresolved: Vec<usize>,
}

/// `project_genres` lives in one group's `schema` array, so its rows are found
/// by table name, never by index.
fn genre_group(document: &Value) -> Option<&'static str> {
    for group in ["require", "optional"] {
        let found = document
            .get(group)
            .and_then(|value| value.get("schema"))
            .and_then(Value::as_array)
            .is_some_and(|entries| {
                entries
                    .iter()
                    .any(|entry| entry["table"] == "project_genres")
            });
        if found {
            return Some(group);
        }
    }
    None
}

fn genre_rows_mut(document: &mut Value) -> Option<(&'static str, &mut Vec<Value>)> {
    let group = genre_group(document)?;
    let entries = document.get_mut(group)?.get_mut("schema")?.as_array_mut()?;
    let entry = entries
        .iter_mut()
        .find(|entry| entry["table"] == "project_genres")?;
    let rows = entry.get_mut("rows")?.as_array_mut()?;
    Some((group, rows))
}

/// Resolves `genre_code` for every `project_genres` row. Unmatched rows are
/// reported, never guessed: the author decides the genre, not us.
pub fn seed(document: &mut Value) -> SeedReport {
    let mut report = SeedReport::default();
    let Some((_, rows)) = genre_rows_mut(document) else {
        return report;
    };
    for (index, row) in rows.iter_mut().enumerate() {
        let genre = row
            .get("genre_id")
            .and_then(Value::as_str)
            .unwrap_or_default();
        let normalized = normalize(genre);
        if normalized.is_empty() {
            report.unresolved.push(index);
            continue;
        }
        let found = catalog::all(Kind::Genres).iter().find(|entry| {
            normalize(&entry.code) == normalized || normalize(&entry.name) == normalized
        });
        // The author writes free text ("trinh thám"), the catalogue holds a long
        // bilingual label ("Huyền bí / Trinh thám (Mystery)"), so a contained
        // match is the useful fallback. Shortest match wins to stay specific.
        let found = found.or_else(|| {
            catalog::all(Kind::Genres)
                .iter()
                .filter(|entry| normalize(&entry.name).contains(&normalized))
                .min_by_key(|entry| normalize(&entry.name).len())
        });
        match found {
            Some(entry) => {
                row["genre_code"] = Value::String(entry.code.clone());
                report.resolved += 1;
            }
            None => report.unresolved.push(index),
        }
    }
    report
}

/// Loads the project document, resolves the genre codes and writes them back
/// through the validated `super_json::set` path only when something changed.
pub fn seed_project(project_id: &str) -> Result<SeedReport, String> {
    let mut document = super_json::load_or_init(project_id)?;
    let report = seed(&mut document);
    if report.resolved == 0 {
        return Ok(report);
    }
    let (group, rows) = genre_rows_mut(&mut document)
        .ok_or_else(|| "Không đọc được project_genres để ghi genre_code".to_owned())?;
    let path = format!("{group}.schema.project_genres.rows");
    let rows = Value::Array(rows.clone());
    super_json::set(project_id, &path, rows)?;
    Ok(report)
}

/// Case- and diacritic-insensitive comparison key, so "Trinh thám" matches the
/// catalogue name and "trinh tham" matches its code.
fn normalize(text: &str) -> String {
    text.chars()
        .filter_map(|c| {
            let folded = match c.to_lowercase().next().unwrap_or(c) {
                'á' | 'à' | 'ả' | 'ã' | 'ạ' | 'ă' | 'ắ' | 'ằ' | 'ẳ' | 'ẵ' | 'ặ' | 'â' | 'ấ'
                | 'ầ' | 'ẩ' | 'ẫ' | 'ậ' => 'a',
                'é' | 'è' | 'ẻ' | 'ẽ' | 'ẹ' | 'ê' | 'ế' | 'ề' | 'ể' | 'ễ' | 'ệ' => {
                    'e'
                }
                'í' | 'ì' | 'ỉ' | 'ĩ' | 'ị' => 'i',
                'ó' | 'ò' | 'ỏ' | 'õ' | 'ọ' | 'ô' | 'ố' | 'ồ' | 'ổ' | 'ỗ' | 'ộ' | 'ơ' | 'ớ'
                | 'ờ' | 'ở' | 'ỡ' | 'ợ' => 'o',
                'ú' | 'ù' | 'ủ' | 'ũ' | 'ụ' | 'ư' | 'ứ' | 'ừ' | 'ử' | 'ữ' | 'ự' => {
                    'u'
                }
                'ý' | 'ỳ' | 'ỷ' | 'ỹ' | 'ỵ' => 'y',
                'đ' => 'd',
                other => other,
            };
            folded.is_ascii_alphanumeric().then_some(folded)
        })
        .collect()
}
