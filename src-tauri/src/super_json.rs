use std::{
    collections::HashMap,
    fs,
    path::PathBuf,
    sync::{Arc, Mutex, OnceLock},
};

use serde_json::{Map, Value};

use crate::{models::SuperJsonProgress, paths};

/// The canonical skeleton lives in the docs so the specification and the code
/// can never drift apart. `docs/design/super-json.md` explains every field.
const SKELETON_JSON: &str = include_str!("../../docs/schema/runtime/super.json");

/// Root-level keys that carry documentation, not data.
const NON_DATA_KEYS: [&str; 1] = ["$comment"];

/// Progress labels, worded exactly as `docs/design/idea.md` §progress describes
/// them. The header renders whatever the backend computes.
pub const LABEL_NO_PROJECT: &str = "Chưa có project";
pub const LABEL_COLLECTING: &str = "Đang thu thập thông tin";
pub const LABEL_ACCEPTABLE: &str = "Độ mạch lạc tạm chấp nhận";
pub const LABEL_GOOD: &str = "Độ mạch lạc: tốt";

/// Parsed skeleton, with documentation keys removed so the file we write to a
/// user's project directory contains data only.
pub fn skeleton() -> Value {
    let mut document: Value =
        serde_json::from_str(SKELETON_JSON).expect("embedded super.json must be valid JSON");
    if let Some(object) = document.as_object_mut() {
        for key in NON_DATA_KEYS {
            object.remove(key);
        }
    }
    document
}

fn collect_leaves(value: &Value, prefix: &str, out: &mut Vec<String>) {
    match value {
        Value::Object(map) => {
            for (key, child) in map {
                if key.starts_with('$') {
                    continue;
                }
                let path = if prefix.is_empty() {
                    key.clone()
                } else {
                    format!("{prefix}.{key}")
                };
                let is_empty_object = child.as_object().is_some_and(Map::is_empty);
                if is_empty_object {
                    out.push(path);
                } else {
                    collect_leaves(child, &path, out);
                }
            }
        }
        // nulls, scalars and arrays are all leaves.
        _ => out.push(prefix.to_string()),
    }
}

fn leaves_of(section: &str) -> Vec<String> {
    let document = skeleton();
    let mut out = Vec::new();
    if let Some(value) = document.get(section) {
        collect_leaves(value, section, &mut out);
    }
    out.sort();
    out
}

/// Every field the agent is allowed to touch, derived from the skeleton itself
/// rather than duplicated as a hand-written constant.
pub fn valid_paths() -> &'static [String] {
    static CACHE: OnceLock<Vec<String>> = OnceLock::new();
    CACHE.get_or_init(|| {
        let mut all = Vec::new();
        collect_leaves(&skeleton(), "", &mut all);
        all.sort();
        all
    })
}

/// Fields that block the "write outline" step until they are filled.
pub fn required_paths() -> &'static [String] {
    static CACHE: OnceLock<Vec<String>> = OnceLock::new();
    CACHE.get_or_init(|| leaves_of("require"))
}

/// The remaining fields: fillable at any time, and the ones that take the header
/// progress bar from 70% to 100%.
pub fn optional_paths() -> &'static [String] {
    static CACHE: OnceLock<Vec<String>> = OnceLock::new();
    CACHE.get_or_init(|| leaves_of("optional"))
}

#[derive(Clone, Copy)]
enum FieldKind {
    Text,
    PositiveInt,
    NonNegativeInt,
    Bool,
    TextArray,
    Choice(&'static [&'static str]),
}

/// Type rules per field. Values start as `null`, so the type cannot be inferred
/// from the skeleton — it has to be declared. `test_every_valid_path_has_a_type_rule`
/// keeps this table in sync with `super.json`.
fn field_kind(path: &str) -> Option<FieldKind> {
    use FieldKind::*;
    Some(match path {
        "require.overview.name"
        | "require.overview.logline.trigger-event"
        | "require.overview.logline.main-character"
        | "require.overview.logline.action"
        | "require.overview.logline.risk"
        | "require.overview.logline.major-loss"
        | "require.genre.primary"
        | "require.theme.question"
        | "optional.overview.description"
        | "optional.overview.author"
        | "optional.overview.target-audience"
        | "optional.overview.age-rating"
        | "optional.theme.resolution"
        | "optional.theme.message" => Text,

        "require.overview.plan.arc.count"
        | "require.overview.plan.chapter.count"
        | "require.overview.plan.word.count" => PositiveInt,

        "optional.overview.plan.arc.tolerance"
        | "optional.overview.plan.chapter.tolerance"
        | "optional.overview.plan.word.tolerance" => NonNegativeInt,

        "optional.overview.language" => Choice(&["VI", "EN", "JA", "ZH_HANT", "ZH_HANS"]),
        "optional.theme.resolution-type" => Choice(&["RESOLVED", "OPEN", "AMBIGUOUS"]),
        "optional.world.setting-scale" => Choice(&[
            "REALM", "WORLD", "REGION", "COUNTRY", "CITY", "TOWN", "FACILITY", "STREET", "ROOM",
        ]),

        "optional.world.has-power-system" => Bool,
        "optional.genre.secondary" => TextArray,

        _ => return None,
    })
}

fn validate(kind: FieldKind, path: &str, value: &Value) -> Result<(), String> {
    let type_error = |expected: &str| {
        format!(
            "Invalid value for {path}: expected {expected}, got {}",
            describe(value)
        )
    };
    match kind {
        FieldKind::Text => match value.as_str() {
            Some(text) if !text.trim().is_empty() => Ok(()),
            Some(_) => Err(format!("Invalid value for {path}: must not be empty")),
            None => Err(type_error("a non-empty string")),
        },
        FieldKind::PositiveInt => match value.as_i64() {
            Some(number) if number > 0 => Ok(()),
            // Deliberately rejects "5": models like to send numbers as strings.
            Some(_) => Err(format!("Invalid value for {path}: must be greater than 0")),
            None => Err(type_error("a positive integer")),
        },
        FieldKind::NonNegativeInt => match value.as_i64() {
            Some(number) if number >= 0 => Ok(()),
            Some(_) => Err(format!("Invalid value for {path}: must be 0 or greater")),
            None => Err(type_error("an integer >= 0")),
        },
        FieldKind::Bool => match value.as_bool() {
            Some(_) => Ok(()),
            None => Err(type_error("a boolean")),
        },
        FieldKind::TextArray => match value.as_array() {
            Some(items) => {
                if items.iter().all(|item| {
                    item.as_str().is_some_and(|text| !text.trim().is_empty())
                }) {
                    Ok(())
                } else {
                    Err(format!(
                        "Invalid value for {path}: every item must be a non-empty string"
                    ))
                }
            }
            None => Err(type_error("an array of strings")),
        },
        FieldKind::Choice(allowed) => match value.as_str() {
            Some(text) if allowed.contains(&text) => Ok(()),
            Some(text) => Err(format!(
                "Invalid value for {path}: {text:?} is not one of {}",
                allowed.join(", ")
            )),
            None => Err(type_error(&format!("one of {}", allowed.join(", ")))),
        },
    }
}

fn describe(value: &Value) -> String {
    match value {
        Value::Null => "null".into(),
        Value::Bool(inner) => format!("boolean {inner}"),
        Value::Number(inner) => format!("number {inner}"),
        Value::String(inner) => format!("string {inner:?}"),
        Value::Array(_) => "an array".into(),
        Value::Object(_) => "an object".into(),
    }
}

fn file_path(project_id: &str) -> Result<PathBuf, String> {
    paths::super_json_path(project_id)
}

/// Process-wide per-project serialization. This protects only threads in this app
/// process; a second app process can still race on the same filesystem document.
fn project_lock(project_id: &str) -> Arc<Mutex<()>> {
    static LOCKS: OnceLock<Mutex<HashMap<String, Arc<Mutex<()>>>>> = OnceLock::new();
    let registry = LOCKS.get_or_init(|| Mutex::new(HashMap::new()));
    let mut locks = registry.lock().unwrap_or_else(|poison| poison.into_inner());
    Arc::clone(
        locks
            .entry(project_id.to_owned())
            .or_insert_with(|| Arc::new(Mutex::new(()))),
    )
}

/// Reads the project's super json, creating it from the skeleton on first use.
pub fn load_or_init(project_id: &str) -> Result<Value, String> {
    let lock = project_lock(project_id);
    let _guard = lock.lock().unwrap_or_else(|poison| poison.into_inner());
    load_or_init_unlocked(project_id)
}

/// The caller must already hold the project's lock.
fn load_or_init_unlocked(project_id: &str) -> Result<Value, String> {
    let path = file_path(project_id)?;
    match fs::read_to_string(&path) {
        Ok(raw) => serde_json::from_str(&raw)
            .map_err(|error| format!("Invalid super json at {}: {error}", path.display())),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            let document = skeleton();
            save_unlocked(project_id, &document, false)?;
            Ok(document)
        }
        Err(error) => Err(format!("Cannot read super json: {error}")),
    }
}

/// The caller must already hold the project's lock. Exclusive saves publish via
/// hard link, which fails if `super.json` exists instead of replacing it.
fn save_unlocked(project_id: &str, document: &Value, exclusive: bool) -> Result<(), String> {
    let path = file_path(project_id)?;
    let parent = path.parent().ok_or("Invalid super json path")?;
    fs::create_dir_all(parent)
        .map_err(|error| format!("Cannot create project directory: {error}"))?;

    let bytes = serde_json::to_vec_pretty(document)
        .map_err(|error| format!("Cannot encode super json: {error}"))?;
    let tmp = parent.join(format!(
        ".super.{}.{}.tmp",
        std::process::id(),
        chrono::Utc::now().timestamp_nanos_opt().unwrap_or_default()
    ));

    let result = (|| -> Result<(), String> {
        #[cfg(unix)]
        {
            use std::{io::Write, os::unix::fs::OpenOptionsExt};
            let mut file = fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .mode(0o600)
                .open(&tmp)
                .map_err(|error| format!("Cannot create super json: {error}"))?;
            file.write_all(&bytes)
                .map_err(|error| format!("Cannot write super json: {error}"))?;
            file.sync_all()
                .map_err(|error| format!("Cannot sync super json: {error}"))?;
        }
        #[cfg(not(unix))]
        {
            fs::write(&tmp, &bytes).map_err(|error| format!("Cannot write super json: {error}"))?;
        }
        if exclusive {
            fs::hard_link(&tmp, &path).map_err(|error| {
                if error.kind() == std::io::ErrorKind::AlreadyExists {
                    format!("Super json already exists at {}", path.display())
                } else {
                    format!("Cannot initialize super json: {error}")
                }
            })?;
            let _ = fs::remove_file(&tmp);
            Ok(())
        } else {
            fs::rename(&tmp, &path).map_err(|error| format!("Cannot replace super json: {error}"))
        }
    })();

    if result.is_err() {
        let _ = fs::remove_file(&tmp);
    }
    result
}

/// Initialize once, in one atomic publish, without replacing an existing file.
/// A successful return means this call created the file; callers may compensate
/// a later transaction failure by removing only that file.
pub fn init_project(project_id: &str, name: Option<&str>) -> Result<(), String> {
    let lock = project_lock(project_id);
    let _guard = lock.lock().unwrap_or_else(|poison| poison.into_inner());
    let mut document = skeleton();
    if let Some(name) = name {
        validate(FieldKind::Text, "require.overview.name", &Value::String(name.to_owned()))?;
        document["require"]["overview"]["name"] = Value::String(name.to_owned());
    }
    save_unlocked(project_id, &document, true)
}

pub fn get(project_id: &str, path: &str) -> Result<Value, String> {
    if !valid_paths().iter().any(|known| known == path) {
        return Err(format!("Unknown super json path: {path}"));
    }
    let document = load_or_init(project_id)?;
    let mut cursor = &document;
    for segment in path.split('.') {
        cursor = cursor
            .get(segment)
            .ok_or_else(|| format!("Path not found: {path}"))?;
    }
    Ok(cursor.clone())
}

/// Sets exactly one whitelisted field. Never creates new fields.
pub fn set(project_id: &str, path: &str, value: Value) -> Result<Value, String> {
    let kind = field_kind(path).ok_or_else(|| {
        format!(
            "Unknown super json path: {path}. Valid paths are: {}",
            valid_paths().join(", ")
        )
    })?;
    validate(kind, path, &value)?;

    let lock = project_lock(project_id);
    let _guard = lock.lock().unwrap_or_else(|poison| poison.into_inner());
    let mut document = load_or_init_unlocked(project_id)?;
    let segments: Vec<&str> = path.split('.').collect();
    let (last, parents) = segments.split_last().ok_or("Empty path")?;

    let mut cursor = &mut document;
    for segment in parents {
        cursor = cursor
            .get_mut(*segment)
            .ok_or_else(|| format!("Path not found: {path}"))?;
    }
    let slot = cursor
        .get_mut(*last)
        .ok_or_else(|| format!("Path not found: {path}"))?;
    *slot = value.clone();

    save_unlocked(project_id, &document, false)?;
    Ok(value)
}

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

fn value_at<'a>(document: &'a Value, path: &str) -> Option<&'a Value> {
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

// ─────────────────────────────────────────────────────────────────────────────
// Sprint 11.11 · D2/D3/D4 — the super json rendered as Vietnamese prose
// ─────────────────────────────────────────────────────────────────────────────

/// What every unanswered field prints, verbatim (D3).
const SUMMARY_PLACEHOLDER: &str = "[chưa có thông tin]";

fn placeholder() -> &'static str {
    SUMMARY_PLACEHOLDER
}

/// `[chưa có thông tin]` when the field is missing.
fn or_placeholder(value: Option<String>) -> String {
    value.unwrap_or_else(|| placeholder().to_owned())
}

/// Reads one leaf as display text. `null`, a blank string, an empty array and a
/// path that does not exist all mean "not answered yet" (D3).
fn text_at(document: &Value, path: &str) -> Option<String> {
    match value_at(document, path)? {
        Value::Null => None,
        Value::String(text) => {
            let trimmed = text.trim();
            (!trimmed.is_empty()).then(|| trimmed.to_owned())
        }
        // `optional.genre.secondary` is the only leaf that is an array; it reads
        // as a comma-separated list. `[]` is "no answer", exactly like progress.
        Value::Array(items) => {
            let joined = items
                .iter()
                .filter_map(Value::as_str)
                .map(str::trim)
                .filter(|item| !item.is_empty())
                .collect::<Vec<_>>()
                .join(", ");
            (!joined.is_empty()).then_some(joined)
        }
        Value::Bool(value) => Some(value.to_string()),
        // D3: integers print whole, with no thousands separator.
        Value::Number(number) => Some(
            number
                .as_i64()
                .map_or_else(|| number.to_string(), |value| value.to_string()),
        ),
        Value::Object(_) => None,
    }
}

/// Integer leaves (`plan.*.count`, `plan.*.tolerance`) as raw numbers, so the
/// caller can decide between a natural sentence and a label sentence (D5).
fn int_at(document: &Value, path: &str) -> Option<i64> {
    value_at(document, path).and_then(Value::as_i64)
}

/// Ends a sentence with exactly one full stop (D5): a value that already ends in
/// `.`, `?`, `!` or `…` is not given a second punctuation mark, which is what
/// produced `con người?.` in Sprint 11.11.
fn terminate(text: &str) -> String {
    let trimmed = text.trim_end();
    match trimmed.chars().last() {
        Some('.') | Some('?') | Some('!') | Some('…') => trimmed.to_owned(),
        _ => format!("{trimmed}."),
    }
}

/// `90000` → `90.000`, `5000` → `5.000`, `120` → `120` (D5).
fn with_thousands(value: i64) -> String {
    let negative = value < 0;
    let digits = value.unsigned_abs().to_string();
    let mut out = String::with_capacity(digits.len() + digits.len() / 3 + 1);
    for (index, character) in digits.chars().enumerate() {
        if index > 0 && (digits.len() - index) % 3 == 0 {
            out.push('.');
        }
        out.push(character);
    }
    if negative {
        format!("-{out}")
    } else {
        out
    }
}

/// `Some(90000)` → `"90.000"`, `None` → the placeholder.
fn thousands_or_placeholder(value: Option<i64>) -> String {
    value.map_or_else(|| placeholder().to_owned(), with_thousands)
}

/// D4: enum codes never reach the dialog. An unknown code (only possible in a
/// hand-edited file) falls back to the placeholder rather than printing raw.
fn enum_label(value: Option<String>, table: &[(&str, &str)]) -> Option<String> {
    let code = value?;
    table
        .iter()
        .find(|(key, _)| *key == code)
        .map(|(_, label)| (*label).to_owned())
}

const LANGUAGE_LABELS: &[(&str, &str)] = &[
    ("VI", "tiếng Việt"),
    ("EN", "tiếng Anh"),
    ("JA", "tiếng Nhật"),
    ("ZH_HANT", "chữ Hán phồn thể"),
    // The skeleton only ever writes ZH_HANS, but ZH_CN is accepted as the same
    // language (D4 names both spellings).
    ("ZH_HANS", "chữ Hán giản thể"),
    ("ZH_CN", "chữ Hán giản thể"),
];

const RESOLUTION_LABELS: &[(&str, &str)] = &[
    ("RESOLVED", "kết thúc rõ ràng"),
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

/// The dialog's main content (D2): six fixed paragraphs, filled from the 25
/// super json leaves, with every gap printed as `[chưa có thông tin]`.
///
/// Deterministic on purpose: same document in, byte-identical text out. No model
/// is called and no locale is consulted.
///
/// Sprint 11.12 · D5 reworked the wording: sentence punctuation is never
/// doubled, a missing field falls back to a `Label: value.` sentence instead of
/// producing a broken clause (`dài [chưa có thông tin] arc`), and counts print
/// with `.` thousands separators.
pub fn summary(document: &Value) -> String {
    // `{, kèm ...}` is the one conditional clause in the template: with no
    // secondary genre there is nothing to append, so it is omitted rather than
    // printed with a placeholder.
    let secondary = text_at(document, "optional.genre.secondary")
        .map(|value| format!(", kèm {value}"))
        .unwrap_or_default();

    // ── 1. Identity ──────────────────────────────────────────────────────
    let author = match text_at(document, "optional.overview.author") {
        // D5: natural sentence when the value exists, label sentence when it does not.
        Some(value) => format!("Tác giả là {}", terminate(&value)),
        None => format!("Tác giả: {}.", placeholder()),
    };
    let paragraph_one = format!(
        "Dự án truyện có tên là {}, thuộc thể loại {}{}. {} Truyện viết bằng {}, hướng tới độc giả {}, độ tuổi {}",
        or_placeholder(text_at(document, "require.overview.name")),
        or_placeholder(text_at(document, "require.genre.primary")),
        secondary,
        author,
        or_placeholder(enum_label(
            text_at(document, "optional.overview.language"),
            LANGUAGE_LABELS
        )),
        or_placeholder(text_at(document, "optional.overview.target-audience")),
        terminate(&or_placeholder(text_at(
            document,
            "optional.overview.age-rating"
        ))),
    );

    // ── 2. Logline ───────────────────────────────────────────────────────
    let trigger = match text_at(document, "require.overview.logline.trigger-event") {
        Some(value) => format!("Câu chuyện bắt đầu khi {}", terminate(&value)),
        None => format!("Sự kiện mở đầu: {}.", placeholder()),
    };
    // D5: no `, người {action}` — that repeated "người" whenever the author's
    // action already started with it.
    let cast = match (
        text_at(document, "require.overview.logline.main-character"),
        text_at(document, "require.overview.logline.action"),
    ) {
        (Some(character), Some(action)) => {
            terminate(&format!("Nhân vật chính là {character}, {action}"))
        }
        (Some(character), None) => format!(
            "Nhân vật chính là {} Hành động chính: {}.",
            terminate(&character),
            placeholder()
        ),
        (None, Some(action)) => format!(
            "Nhân vật chính: {}. Hành động chính: {}",
            placeholder(),
            terminate(&action)
        ),
        (None, None) => format!(
            "Nhân vật chính: {}. Hành động chính: {}.",
            placeholder(),
            placeholder()
        ),
    };
    let stake = match (
        text_at(document, "require.overview.logline.risk"),
        text_at(document, "require.overview.logline.major-loss"),
    ) {
        (Some(risk), Some(loss)) => terminate(&format!(
            "Mối nguy là {risk}, và cái giá có thể phải chịu là {loss}"
        )),
        (Some(risk), None) => format!(
            "Mối nguy là {} Mất mát lớn: {}.",
            terminate(&risk),
            placeholder()
        ),
        (None, Some(loss)) => format!(
            "Rủi ro: {}. Cái giá có thể phải chịu là {}",
            placeholder(),
            terminate(&loss)
        ),
        (None, None) => format!("Rủi ro: {}. Mất mát lớn: {}.", placeholder(), placeholder()),
    };
    let paragraph_two = format!("{trigger} {cast} {stake}");

    // ── 3. Theme ─────────────────────────────────────────────────────────
    let message = match text_at(document, "optional.theme.message") {
        Some(value) => format!("Thông điệp hướng tới là {}", terminate(&value)),
        None => format!("Thông điệp hướng tới: {}.", placeholder()),
    };
    let resolution = or_placeholder(text_at(document, "optional.theme.resolution"));
    let answer = match enum_label(
        text_at(document, "optional.theme.resolution-type"),
        RESOLUTION_LABELS,
    ) {
        Some(kind) => format!(
            "Cách trả lời được chọn là {kind}, với nội dung {}",
            terminate(&resolution)
        ),
        None => format!(
            "Cách trả lời được chọn: {}, với nội dung: {}",
            placeholder(),
            terminate(&resolution)
        ),
    };
    let paragraph_three = format!(
        "Bộ truyện đặt ra câu hỏi: {} {} {}",
        terminate(&or_placeholder(text_at(document, "require.theme.question"))),
        message,
        answer
    );

    // ── 4. Plan ──────────────────────────────────────────────────────────
    let arc = int_at(document, "require.overview.plan.arc.count");
    let chapter = int_at(document, "require.overview.plan.chapter.count");
    let word = int_at(document, "require.overview.plan.word.count");
    let plan = match (arc, chapter, word) {
        // D5: the natural sentence only when all three counts exist; otherwise
        // label sentences, never a broken `dài [chưa có thông tin] arc`.
        (Some(arc), Some(chapter), Some(word)) => format!(
            "Dự kiến dài {} arc, {} chương, khoảng {} từ.",
            with_thousands(arc),
            with_thousands(chapter),
            with_thousands(word)
        ),
        _ => format!(
            "Số arc dự kiến: {}. Số chương dự kiến: {}. Tổng số từ mục tiêu: {}.",
            thousands_or_placeholder(arc),
            thousands_or_placeholder(chapter),
            thousands_or_placeholder(word)
        ),
    };
    let arc_tolerance = int_at(document, "optional.overview.plan.arc.tolerance");
    let chapter_tolerance = int_at(document, "optional.overview.plan.chapter.tolerance");
    let word_tolerance = int_at(document, "optional.overview.plan.word.tolerance");
    // D5: the tolerance sentence only exists once at least one tolerance is set.
    let tolerance = if arc_tolerance.is_some()
        || chapter_tolerance.is_some()
        || word_tolerance.is_some()
    {
        format!(
            " Sai số chấp nhận: {} arc, {} chương, {} từ.",
            thousands_or_placeholder(arc_tolerance),
            thousands_or_placeholder(chapter_tolerance),
            thousands_or_placeholder(word_tolerance)
        )
    } else {
        String::new()
    };
    let paragraph_four = format!("{plan}{tolerance}");

    // ── 5. World ─────────────────────────────────────────────────────────
    let power = match value_at(document, "optional.world.has-power-system") {
        Some(Value::Bool(true)) => "Thế giới có hệ thống sức mạnh.".to_owned(),
        Some(Value::Bool(false)) => "Thế giới không có hệ thống sức mạnh.".to_owned(),
        _ => format!(
            "Thế giới có hệ thống sức mạnh hay không: {}.",
            placeholder()
        ),
    };
    let scale = match enum_label(
        text_at(document, "optional.world.setting-scale"),
        SCALE_LABELS,
    ) {
        Some(value) => format!("Phạm vi không gian sâu nhất cần dựng là {}", terminate(&value)),
        None => format!(
            "Phạm vi không gian sâu nhất cần dựng: {}.",
            placeholder()
        ),
    };
    let paragraph_five = format!("{power} {scale}");

    // ── 6. Synopsis ──────────────────────────────────────────────────────
    let paragraph_six = format!(
        "Tóm tắt nội dung: {}",
        terminate(&or_placeholder(text_at(
            document,
            "optional.overview.description"
        )))
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
    use serde_json::json;

    #[test]
    fn test_concurrent_sets_preserve_all_required_values() {
        use std::sync::{Arc, Barrier};

        let _home = crate::test_support::TempHome::new("super-concurrent");
        let project_id = "concurrent".to_owned();
        let paths = required_paths().to_vec();
        let barrier = Arc::new(Barrier::new(paths.len()));
        let threads: Vec<_> = paths
            .iter()
            .cloned()
            .map(|path| {
                let barrier = Arc::clone(&barrier);
                let project_id = project_id.clone();
                std::thread::spawn(move || {
                    barrier.wait();
                    set(&project_id, &path, required_value(&path)).unwrap();
                })
            })
            .collect();
        for thread in threads {
            thread.join().unwrap();
        }
        let document = load_or_init(&project_id).unwrap();
        for path in &paths {
            assert_eq!(value_at(&document, path), Some(&required_value(path)), "{path}");
        }
    }

    #[test]
    fn test_init_project_refuses_to_overwrite_existing_json() {
        let _home = crate::test_support::TempHome::new("super-no-overwrite");
        init_project("existing", Some("First")).unwrap();
        let error = init_project("existing", Some("Second")).unwrap_err();
        assert!(error.contains("already exists"), "{error}");
        assert_eq!(get("existing", "require.overview.name").unwrap(), json!("First"));
    }

    #[test]
    fn test_super_json_required_path_count_is_eleven() {
        let _home = crate::test_support::TempHome::new("super-count");
        assert_eq!(valid_paths().len(), 25, "valid path count drifted");
        assert_eq!(
            required_paths().len(),
            11,
            "required path count drifted: {:?}",
            required_paths()
        );
        assert_eq!(
            missing_required("default").unwrap().len(),
            11,
            "a fresh project must report all 11 required paths as missing"
        );
        // Every required path must also be a valid path.
        for path in required_paths() {
            assert!(valid_paths().iter().any(|known| known == path), "{path}");
        }
    }

    /// Guards the hand-written type table against `super.json` edits.
    #[test]
    fn test_every_valid_path_has_a_type_rule() {
        for path in valid_paths() {
            assert!(
                field_kind(path).is_some(),
                "no type rule declared for {path} — super.json and field_kind() drifted"
            );
        }
        assert_eq!(valid_paths().len(), 25);
    }

    #[test]
    fn test_super_json_skeleton_and_set_round_trip() {
        let _home = crate::test_support::TempHome::new("super-round-trip");

        let fresh = load_or_init("default").unwrap();
        assert_eq!(fresh, skeleton(), "file must start as the skeleton");

        let on_disk: Value =
            serde_json::from_str(&fs::read_to_string(paths::super_json_path("default").unwrap()).unwrap())
                .unwrap();
        assert_eq!(on_disk, skeleton());
        // Documentation keys must not leak into the user's data file.
        assert!(on_disk.get("$comment").is_none());

        let returned = set("default", "require.overview.name", json!("Nửa Đêm")).unwrap();
        assert_eq!(returned, json!("Nửa Đêm"));

        set("default", "require.overview.plan.arc.count", json!(5)).unwrap();
        set("default", "optional.genre.secondary", json!(["trinh thám"])).unwrap();

        assert_eq!(
            get("default", "require.overview.name").unwrap(),
            json!("Nửa Đêm")
        );
        assert_eq!(
            get("default", "require.overview.plan.arc.count").unwrap(),
            json!(5)
        );

        // A different project must not see the first project's edits.
        assert!(get("other", "require.overview.name").unwrap().is_null());

        // Re-reading from disk preserves the edits.
        assert_eq!(
            load_or_init("default").unwrap()["require"]["overview"]["name"],
            json!("Nửa Đêm")
        );

        let missing = missing_required("default").unwrap();
        assert_eq!(missing.len(), 9, "two required fields were filled");
        assert!(!missing.contains(&"require.overview.name".to_string()));
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

    #[test]
    fn test_super_json_rejects_unknown_path() {
        let _home = crate::test_support::TempHome::new("super-unknown");

        for bad in [
            "require.overview.bogus",
            "project.name", // the path the model invented before the enum existed
            "",
            "require",
        ] {
            let error = set("default", bad, json!("x")).unwrap_err();
            assert!(
                error.contains("Unknown super json path"),
                "unexpected error for {bad}: {error}"
            );
        }

        // A rejected write must not add anything to the document.
        let document = load_or_init("default").unwrap();
        assert_eq!(document, skeleton());
        assert!(document["require"]["overview"].get("bogus").is_none());
    }

    #[test]
    fn test_super_json_rejects_wrong_type() {
        let _home = crate::test_support::TempHome::new("super-type");
        load_or_init("default").unwrap();

        // Models routinely send numbers as strings; that must be rejected.
        assert!(set("default", "require.overview.plan.arc.count", json!("5")).is_err());
        assert!(set("default", "require.overview.plan.arc.count", json!(0)).is_err());
        assert!(set("default", "require.overview.plan.arc.count", json!(-1)).is_err());
        assert!(set("default", "require.overview.plan.arc.count", json!(true)).is_err());
        assert!(set("default", "require.overview.plan.arc.count", json!(5.5)).is_err());
        assert!(set("default", "require.overview.plan.arc.count", json!(5)).is_ok());

        assert!(set("default", "optional.overview.plan.arc.tolerance", json!(0)).is_ok());
        assert!(set("default", "optional.overview.plan.arc.tolerance", json!(-1)).is_err());

        assert!(set("default", "require.overview.name", json!("   ")).is_err());
        assert!(set("default", "require.overview.name", json!(7)).is_err());
        assert!(set("default", "require.overview.name", json!(null)).is_err());

        assert!(set("default", "optional.world.has-power-system", json!(true)).is_ok());
        assert!(set("default", "optional.world.has-power-system", json!("yes")).is_err());

        assert!(set("default", "optional.genre.secondary", json!(["a", "b"])).is_ok());
        assert!(set("default", "optional.genre.secondary", json!([])).is_ok());
        assert!(set("default", "optional.genre.secondary", json!([""])).is_err());
        assert!(set("default", "optional.genre.secondary", json!("a")).is_err());

        // Failed writes must leave the file untouched.
        assert_eq!(load_or_init("default").unwrap()["require"]["overview"]["plan"]["arc"]
            ["count"], json!(5));
    }

    /// A valid value for every required path, so progress tests can complete the
    /// 11 blocking fields without restating the type rules.
    fn required_value(path: &str) -> Value {
        match path {
            "require.overview.plan.arc.count" => json!(5),
            "require.overview.plan.chapter.count" => json!(120),
            "require.overview.plan.word.count" => json!(90000),
            _ => json!(format!("value for {path}")),
        }
    }

    /// Same for the 14 optional paths, including the enum and boolean ones.
    fn optional_value(path: &str) -> Value {
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

    #[test]
    fn test_super_json_enum_paths_reject_invalid_value() {
        let _home = crate::test_support::TempHome::new("super-enum");
        load_or_init("default").unwrap();

        assert!(set("default", "optional.theme.resolution-type", json!("MAYBE")).is_err());
        assert!(set("default", "optional.theme.resolution-type", json!("OPEN")).is_ok());

        assert!(set("default", "optional.world.setting-scale", json!("PLANET")).is_err());
        assert!(set("default", "optional.world.setting-scale", json!("CITY")).is_ok());

        assert!(set("default", "optional.overview.language", json!("VI")).is_ok());
        assert!(set("default", "optional.overview.language", json!("vi")).is_err());
        assert!(set("default", "optional.overview.language", json!("FR")).is_err());

        let error =
            set("default", "optional.world.setting-scale", json!("PLANET")).unwrap_err();
        assert!(error.contains("is not one of"), "unhelpful error: {error}");
    }

    // ─────────────────────────────────────────────────────────────────────
    // Sprint 11.11 · D3/D4 — the prose summary
    // ─────────────────────────────────────────────────────────────────────

    /// Writes `value` at a dotted path in an in-memory document.
    fn assign(document: &mut Value, path: &str, value: Value) {
        let segments: Vec<&str> = path.split('.').collect();
        let (last, parents) = segments.split_last().expect("a path has segments");
        let mut cursor = document;
        for segment in parents {
            cursor = cursor
                .get_mut(*segment)
                .unwrap_or_else(|| panic!("path {path} is missing segment {segment}"));
        }
        cursor
            .as_object_mut()
            .expect("super json nodes are objects")
            .insert((*last).to_owned(), value);
    }

    /// Every one of the 25 leaves answered, with the enum values D4 cares about.
    fn filled_document() -> Value {
        json!({
            "require": {
                "overview": {
                    "name": "Nửa Đêm",
                    "logline": {
                        "trigger-event": "chiếc đồng hồ cuối cùng ngừng chạy",
                        "main-character": "một người thợ đồng hồ mù",
                        "action": "truy tìm người đã chế tạo ra nó",
                        "risk": "đánh mất ký ức",
                        "major-loss": "đứa con gái duy nhất"
                    },
                    "plan": {
                        "arc": { "count": 5 },
                        "chapter": { "count": 120 },
                        "word": { "count": 90000 }
                    }
                },
                "genre": { "primary": "trinh thám" },
                "theme": { "question": "ký ức có làm nên con người?" }
            },
            "optional": {
                "overview": {
                    "description": "Một người thợ đồng hồ mù đi tìm ký ức đã mất",
                    "author": "Nguyễn Văn A",
                    "language": "VI",
                    "target-audience": "người lớn",
                    "age-rating": "16+",
                    "plan": {
                        "arc": { "tolerance": 1 },
                        "chapter": { "tolerance": 10 },
                        "word": { "tolerance": 5000 }
                    }
                },
                "genre": { "secondary": ["huyền bí", "gia đình"] },
                "theme": {
                    "resolution-type": "OPEN",
                    "resolution": "người đọc tự quyết định",
                    "message": "ký ức định nghĩa con người"
                },
                "world": { "has-power-system": true, "setting-scale": "CITY" }
            }
        })
    }

    #[test]
    fn test_summary_marks_missing_fields_as_placeholder() {
        let text = summary(&skeleton());
        let paragraphs: Vec<&str> = text.split("\n\n").collect();
        assert_eq!(paragraphs.len(), 6);
        for (index, paragraph) in paragraphs.iter().enumerate() {
            assert!(
                paragraph.contains("[chưa có thông tin]"),
                "paragraph {} printed no placeholder: {paragraph}",
                index + 1
            );
        }
        // 21 slots in Sprint 11.12 · D5 (it was 24 in 11.11): 25 leaves, minus
        // `genre.secondary` (its `, kèm …` clause is conditional) and minus the
        // three tolerances (the "Sai số chấp nhận" sentence is omitted entirely
        // while all three are unset).
        assert_eq!(
            text.matches("[chưa có thông tin]").count(),
            21,
            "unexpected placeholder count:\n{text}"
        );
        // D5: no broken `dài [chưa có thông tin] arc` clause.
        assert!(
            !text.contains("dài [chưa có thông tin] arc"),
            "a count is missing and still printed as a broken clause:\n{text}"
        );
        assert!(
            !text.contains("Sai số chấp nhận"),
            "the tolerance sentence must not print while every tolerance is unset:\n{text}"
        );
        assert!(!text.contains("null"), "raw JSON leaked into the prose:\n{text}");
        assert!(
            !text.contains("require.") && !text.contains("optional."),
            "a path leaked into the prose:\n{text}"
        );
        // Printed on demand (`--nocapture`) as hand-off evidence.
        println!("skeleton summary ->\n{text}");
    }

    #[test]
    fn test_summary_includes_all_filled_values() {
        let document = filled_document();
        let text = summary(&document);
        for expected in [
            "Dự án truyện có tên là Nửa Đêm, thuộc thể loại trinh thám, kèm huyền bí, gia đình.",
            "Tác giả là Nguyễn Văn A.",
            "Truyện viết bằng tiếng Việt, hướng tới độc giả người lớn, độ tuổi 16+.",
            "Câu chuyện bắt đầu khi chiếc đồng hồ cuối cùng ngừng chạy.",
            "Nhân vật chính là một người thợ đồng hồ mù, truy tìm người đã chế tạo ra nó.",
            "Mối nguy là đánh mất ký ức, và cái giá có thể phải chịu là đứa con gái duy nhất.",
            "Bộ truyện đặt ra câu hỏi: ký ức có làm nên con người?",
            "Thông điệp hướng tới là ký ức định nghĩa con người.",
            "Cách trả lời được chọn là để ngỏ, với nội dung người đọc tự quyết định.",
            "Dự kiến dài 5 arc, 120 chương, khoảng 90.000 từ.",
            "Sai số chấp nhận: 1 arc, 10 chương, 5.000 từ.",
            "Thế giới có hệ thống sức mạnh.",
            "Phạm vi không gian sâu nhất cần dựng là thành phố.",
            "Tóm tắt nội dung: Một người thợ đồng hồ mù đi tìm ký ức đã mất.",
        ] {
            assert!(text.contains(expected), "summary is missing {expected:?}:\n{text}");
        }
        assert!(
            !text.contains("[chưa có thông tin]"),
            "a fully answered document still prints a placeholder:\n{text}"
        );
        // Printed on demand (`--nocapture`) as hand-off evidence.
        println!("filled super json summary ->\n{text}");
    }

    #[test]
    fn test_summary_maps_enums_to_vietnamese() {
        let mut document = filled_document();

        // Language, every code D4 names (including both Hán spellings).
        for (code, expected) in [
            ("VI", "tiếng Việt"),
            ("EN", "tiếng Anh"),
            ("JA", "tiếng Nhật"),
            ("ZH_HANT", "chữ Hán phồn thể"),
            ("ZH_HANS", "chữ Hán giản thể"),
            ("ZH_CN", "chữ Hán giản thể"),
        ] {
            assign(&mut document, "optional.overview.language", json!(code));
            let text = summary(&document);
            assert!(
                text.contains(&format!("Truyện viết bằng {expected},")),
                "language {code} did not map to {expected}:\n{text}"
            );
            assert!(!text.contains(code), "raw language code {code} leaked:\n{text}");
        }

        // Resolution type.
        for (code, expected) in [
            ("RESOLVED", "kết thúc rõ ràng"),
            ("OPEN", "để ngỏ"),
            ("AMBIGUOUS", "mơ hồ"),
        ] {
            assign(&mut document, "optional.theme.resolution-type", json!(code));
            let text = summary(&document);
            assert!(
                text.contains(&format!("Cách trả lời được chọn là {expected},")),
                "resolution {code} did not map to {expected}:\n{text}"
            );
            assert!(!text.contains(code), "raw resolution code {code} leaked:\n{text}");
        }

        // Setting scale, all nine stops.
        for (code, expected) in [
            ("REALM", "cõi"),
            ("WORLD", "thế giới"),
            ("REGION", "vùng"),
            ("COUNTRY", "quốc gia"),
            ("CITY", "thành phố"),
            ("TOWN", "thị trấn"),
            ("FACILITY", "công trình"),
            ("STREET", "đường phố"),
            ("ROOM", "căn phòng"),
        ] {
            assign(&mut document, "optional.world.setting-scale", json!(code));
            let text = summary(&document);
            assert!(
                text.contains(&format!("cần dựng là {expected}.")),
                "scale {code} did not map to {expected}:\n{text}"
            );
            assert!(!text.contains(code), "raw scale code {code} leaked:\n{text}");
        }

        // The boolean is prose too, never `true`/`false`.
        assign(&mut document, "optional.world.has-power-system", json!(false));
        let text = summary(&document);
        assert!(text.contains("Thế giới không có hệ thống sức mạnh."), "{text}");
        assert!(!text.contains("Thế giới true") && !text.contains("Thế giới false"), "{text}");

        // An unknown code cannot print itself either.
        assign(&mut document, "optional.overview.language", json!("FR"));
        let text = summary(&document);
        assert!(
            text.contains("Truyện viết bằng [chưa có thông tin],"),
            "unknown language did not fall back to the placeholder:\n{text}"
        );
        assert!(!text.contains("FR"), "raw unknown code leaked:\n{text}");
    }

    #[test]
    fn test_summary_has_six_paragraphs() {
        for document in [skeleton(), filled_document()] {
            let text = summary(&document);
            let paragraphs: Vec<&str> = text.split("\n\n").collect();
            assert_eq!(paragraphs.len(), 6, "expected six paragraphs:\n{text}");
            assert!(
                paragraphs.iter().all(|paragraph| !paragraph.trim().is_empty()),
                "one of the six paragraphs is empty:\n{text}"
            );
            assert!(
                !text.trim_start().starts_with('{'),
                "the dialog content must be prose, not JSON:\n{text}"
            );
            assert_eq!(text, text.trim(), "the summary must not carry padding");
        }
    }

    #[test]
    fn test_summary_handles_partial_data() {
        let mut document = skeleton();
        assign(&mut document, "require.overview.name", json!("Nửa Đêm"));
        assign(&mut document, "optional.overview.language", json!("JA"));
        assign(&mut document, "require.overview.plan.arc.count", json!(3));
        assign(&mut document, "optional.genre.secondary", json!(["huyền bí"]));
        assign(&mut document, "optional.world.has-power-system", json!(true));
        assign(&mut document, "optional.world.setting-scale", json!("ROOM"));

        let text = summary(&document);
        // Filled values are in place, with no placeholder next to them ...
        assert!(
            text.contains("Dự án truyện có tên là Nửa Đêm, thuộc thể loại [chưa có thông tin], kèm huyền bí."),
            "{text}"
        );
        assert!(text.contains("Truyện viết bằng tiếng Nhật,"), "{text}");
        // D5: only one of the three counts is set, so the label sentence is used
        // instead of the broken `dài 3 arc, [chưa có thông tin] chương …` clause.
        assert!(
            text.contains(
                "Số arc dự kiến: 3. Số chương dự kiến: [chưa có thông tin]. Tổng số từ mục tiêu: [chưa có thông tin]."
            ),
            "{text}"
        );
        assert!(
            text.contains("Thế giới có hệ thống sức mạnh. Phạm vi không gian sâu nhất cần dựng là căn phòng."),
            "{text}"
        );
        // ... while the untouched fields still say so, in label form (D5).
        assert!(text.contains("Tác giả: [chưa có thông tin]."), "{text}");
        assert!(text.contains("Sự kiện mở đầu: [chưa có thông tin]."), "{text}");
        assert!(text.contains("Rủi ro: [chưa có thông tin]."), "{text}");
        assert!(text.contains("Tóm tắt nội dung: [chưa có thông tin]."), "{text}");
        assert_eq!(text.split("\n\n").count(), 6, "partial data must not drop a paragraph");
        assert!(!text.contains("null"), "{text}");
        println!("partial super json summary ->\n{text}");
    }

    /// D5: a value that already ends a sentence must not collect a second mark.
    #[test]
    fn test_summary_does_not_double_question_mark() {
        let text = summary(&filled_document());
        assert!(
            text.contains("Bộ truyện đặt ra câu hỏi: ký ức có làm nên con người?"),
            "{text}"
        );
        assert!(!text.contains("?."), "a question mark was doubled:\n{text}");
        assert!(!text.contains("!!") && !text.contains(". ."), "{text}");

        // The same rule covers every value that can end a sentence.
        let mut document = filled_document();
        assign(
            &mut document,
            "require.overview.logline.trigger-event",
            json!("một tiếng súng nổ?"),
        );
        assign(
            &mut document,
            "optional.overview.description",
            json!("Anh ta biến mất…"),
        );
        assign(
            &mut document,
            "optional.theme.resolution",
            json!("không ai biết!"),
        );
        let text = summary(&document);
        assert!(!text.contains("?."), "{text}");
        assert!(!text.contains("…."), "{text}");
        assert!(!text.contains("!."), "{text}");
    }

    /// D5: the old template's `, người {action}` repeated "người" whenever the
    /// author's action already began with it.
    #[test]
    fn test_summary_action_has_no_repeated_nguoi() {
        let text = summary(&filled_document());
        assert!(
            !text.contains("người truy tìm người"),
            "the repeated connector is back:\n{text}"
        );
        assert!(
            text.contains("Nhân vật chính là một người thợ đồng hồ mù, truy tìm người đã chế tạo ra nó."),
            "{text}"
        );
        assert!(!text.contains(", người "), "the `, người` connector leaked back:\n{text}");
    }

    /// D5: counts print with `.` as the thousands separator.
    #[test]
    fn test_summary_formats_thousands_with_dots() {
        assert_eq!(with_thousands(90000), "90.000");
        assert_eq!(with_thousands(5000), "5.000");
        assert_eq!(with_thousands(240000), "240.000");
        assert_eq!(with_thousands(120), "120");
        assert_eq!(with_thousands(500), "500");
        assert_eq!(with_thousands(1000), "1.000");
        assert_eq!(with_thousands(0), "0");

        let text = summary(&filled_document());
        assert!(text.contains("khoảng 90.000 từ"), "{text}");
        assert!(text.contains("5.000 từ"), "{text}");
        assert!(!text.contains("90000"), "raw count leaked:\n{text}");
        assert!(!text.contains("5000"), "raw tolerance leaked:\n{text}");

        // The tolerance numbers go through the same formatter.
        let mut document = filled_document();
        assign(
            &mut document,
            "optional.overview.plan.word.tolerance",
            json!(240000),
        );
        let text = summary(&document);
        assert!(text.contains("240.000 từ"), "{text}");
    }

    /// D5: incomplete counts use three label sentences instead of one broken one.
    #[test]
    fn test_summary_uses_labels_when_counts_incomplete() {
        let empty = summary(&skeleton());
        assert!(
            !empty.contains("dài [chưa có thông tin] arc"),
            "the broken clause is back:\n{empty}"
        );
        assert!(
            empty.contains("Số arc dự kiến: [chưa có thông tin]. Số chương dự kiến: [chưa có thông tin]. Tổng số từ mục tiêu: [chưa có thông tin]."),
            "{empty}"
        );

        // Two of three still fall back to labels, with the known values in place.
        let mut document = skeleton();
        assign(&mut document, "require.overview.plan.arc.count", json!(5));
        assign(
            &mut document,
            "require.overview.plan.chapter.count",
            json!(120),
        );
        let text = summary(&document);
        assert!(
            text.contains("Số arc dự kiến: 5. Số chương dự kiến: 120. Tổng số từ mục tiêu: [chưa có thông tin]."),
            "{text}"
        );
        assert!(!text.contains("Dự kiến dài"), "{text}");

        // All three: the natural sentence, with thousands separators.
        let filled = summary(&filled_document());
        assert!(
            filled.contains("Dự kiến dài 5 arc, 120 chương, khoảng 90.000 từ."),
            "{filled}"
        );
    }

    /// D5: the tolerance sentence appears only once a tolerance is actually set.
    #[test]
    fn test_summary_omits_tolerance_sentence_when_unset() {
        let empty = summary(&skeleton());
        assert!(!empty.contains("Sai số chấp nhận"), "{empty}");

        // One tolerance is enough to bring the sentence back; the other two print
        // their placeholder in place.
        let mut document = skeleton();
        assign(
            &mut document,
            "optional.overview.plan.chapter.tolerance",
            json!(10),
        );
        let text = summary(&document);
        assert!(
            text.contains("Sai số chấp nhận: [chưa có thông tin] arc, 10 chương, [chưa có thông tin] từ."),
            "{text}"
        );

        // All three set: every number is printed.
        let filled = summary(&filled_document());
        assert!(
            filled.contains("Sai số chấp nhận: 1 arc, 10 chương, 5.000 từ."),
            "{filled}"
        );
    }

    #[test]
    fn test_summary_is_deterministic() {
        let document = filled_document();
        let first = summary(&document);
        for _ in 0..5 {
            assert_eq!(summary(&document), first, "the same document printed differently");
        }

        // A structurally equal but separately built document must also match: the
        // output depends on values only, never on iteration order or state.
        let rebuilt = serde_json::from_str::<Value>(&serde_json::to_string(&document).unwrap()).unwrap();
        assert_eq!(summary(&rebuilt), first);

        // And the empty document is just as stable.
        let empty = summary(&skeleton());
        assert_eq!(summary(&skeleton()), empty);
    }
}
