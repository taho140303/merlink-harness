//! Sprint 11.11 · D2/D3/D4 — the super json rendered as Vietnamese prose.

use serde_json::Value;

use super::progress::value_at;

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
    use crate::super_json::skeleton;
    use serde_json::json;

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
