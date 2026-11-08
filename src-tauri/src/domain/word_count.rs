//! Deterministic multilingual word count (`REQ-DOM-006`, `be-core.md` §8.5, `T-UNIT-05`).
//!
//! Rules (`be-core.md` §8.5.2):
//! - Uses **Unicode Standard Annex #29 (`unicode-segmentation`)** combined with CJK character rules:
//!   - Every character belonging to the Unicode **Han (CJK Unified Ideographs)**, **Hiragana**,
//!     or **Katakana** script blocks counts as `1` word unit.
//!   - Every contiguous non-CJK alphanumeric word segment split by UAX #29 (`unicode_words()`)
//!     counts as `1` word unit (each whitespace-separated Vietnamese syllable or English/Latin/numeric
//!     word is `1` word unit).
//!   - Punctuation, whitespace, and symbols are ignored.

use unicode_segmentation::UnicodeSegmentation;

use super::business_flow::Language;

/// Counts words deterministically across Vietnamese, English, Japanese, and Chinese text
/// using UAX #29 word boundaries plus per-character CJK counting (`REQ-DOM-006`, `T-UNIT-05`).
pub fn count_words(text: &str) -> u32 {
    if text.trim().is_empty() {
        return 0;
    }

    let mut total: u32 = 0;
    let mut non_cjk_buffer = String::with_capacity(text.len());

    for ch in text.chars() {
        if is_cjk_word_char(ch) {
            total = total.saturating_add(1);
            // Push a separator so UAX #29 treats Latin/numeric runs on either side of a CJK character
            // as distinct words (e.g. "AI東京2026" -> "AI" (1) + '東' (1) + '京' (1) + "2026" (1) = 4).
            non_cjk_buffer.push(' ');
        } else {
            non_cjk_buffer.push(ch);
        }
    }

    for token in non_cjk_buffer.unicode_words() {
        if token.chars().any(|c| c.is_alphanumeric()) {
            total = total.saturating_add(1);
        }
    }

    total
}

/// Language-aware word counter matching the call signature referenced in `be-core.md` §8.4.2 / §8.5.2
/// (`count_words_with_language(&item.content, project_language)`).
///
/// Because `count_words` already handles both UAX #29 Latin/Vietnamese tokens and Han/Hiragana/Katakana
/// characters deterministically, this produces identical results regardless of whether text is pure
/// or mixed-script.
pub fn count_words_for_language(text: &str, _language: Language) -> u32 {
    count_words(text)
}

/// Returns `true` if `ch` belongs to a CJK ideograph or Japanese syllabary block where each
/// character represents one word unit (`be-core.md` §8.5.2):
/// - Hiragana (`U+3040..=U+309F`, excluding non-letter punctuation marks `U+3099..=U+309C` / `U+309B..=U+309C`)
/// - Katakana (`U+30A0..=U+30FF`, excluding middle dot `U+30FB`)
/// - Katakana Phonetic Extensions (`U+31F0..=U+31FF`)
/// - Halfwidth Katakana (`U+FF66..=U+FF9F`)
/// - CJK Unified Ideographs (`U+4E00..=U+9FFF`)
/// - CJK Unified Ideographs Extension A (`U+3400..=U+4DBF`)
/// - CJK Unified Ideographs Extensions B–I (`U+20000..=U+323AF`)
/// - CJK Compatibility Ideographs (`U+F900..=U+FAFF`, `U+2F800..=U+2FA1F`)
pub fn is_cjk_word_char(ch: char) -> bool {
    let code = ch as u32;
    matches!(
        code,
        // Hiragana letters (excluding combining voiced marks 3099..309C)
        0x3041..=0x3096
        | 0x309D..=0x309F
        // Katakana letters (excluding katakana double hyphen 30A0 and middle dot 30FB)
        | 0x30A1..=0x30FA
        | 0x30FC..=0x30FF
        | 0x31F0..=0x31FF
        | 0xFF66..=0xFF9F
        // CJK Unified Ideographs Extension A
        | 0x3400..=0x4DBF
        // CJK Unified Ideographs Main Block
        | 0x4E00..=0x9FFF
        // CJK Compatibility Ideographs
        | 0xF900..=0xFAFF
        // CJK Unified Ideographs Extensions B, C, D, E, F, I, G, H + Compatibility Supplement
        | 0x20000..=0x323AF
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_vietnamese_english_japanese_and_chinese_deterministically() {
        // Vietnamese: 6 syllables separated by whitespace
        assert_eq!(count_words("Trăng đêm chiếu sáng đại ngàn."), 6);
        // English: 5 words
        assert_eq!(count_words("The quick brown fox jumps!"), 5);
        // Japanese: "東京" (2 Han) + "の" (1 Hiragana) + "夜" (1 Han) + "は" (1 Hiragana) + "静" (1 Han) + "か" (1 Hiragana) + "だ" (1 Hiragana) = 8
        assert_eq!(count_words("東京の夜は静かだ。"), 8);
        // Chinese: "江湖" (2) + "風雲" (2) + "再起" (2) = 6
        assert_eq!(count_words("江湖風雲再起，"), 6);
        // Mixed CJK + Latin/Numbers: "Chương" (1) + "1" (1) + "東京" (2) + "2026" (1) = 5
        assert_eq!(count_words("Chương 1: 東京 2026!"), 5);
    }
}
