//! Prompt builders for the phase-2 lanes.
//!
//! The system text lives in `prompts/outline_*.md` so the wording can be edited
//! without touching Rust; the user text carries the numbers of the current plan
//! (arc budget, chapter range, scene budget) which the model must respect.

use super::contract::Lane;

pub const ARCS_SYSTEM: &str = include_str!("../../prompts/outline_arcs.md");
pub const CHAPTERS_SYSTEM: &str = include_str!("../../prompts/outline_chapters.md");
pub const CAST_SYSTEM: &str = include_str!("../../prompts/outline_cast.md");
pub const WORLD_SYSTEM: &str = include_str!("../../prompts/outline_world.md");
pub const BONDS_SYSTEM: &str = include_str!("../../prompts/outline_bonds.md");
pub const SCENES_SYSTEM: &str = include_str!("../../prompts/outline_scenes.md");

/// System prompt for a lane. Only implemented lanes have one.
pub fn system(lane: Lane) -> &'static str {
    match lane {
        Lane::ArcSmith => ARCS_SYSTEM,
        Lane::ChapterSmith => CHAPTERS_SYSTEM,
        Lane::CastSmith => CAST_SYSTEM,
        Lane::WorldSmith => WORLD_SYSTEM,
        Lane::BondSmith => BONDS_SYSTEM,
        Lane::SceneBinder => SCENES_SYSTEM,
    }
}

/// The arc brief: the whole plan in one call, so the arcs can be balanced.
pub fn arcs_user(anchor: &str, arc_total: u64, chapter_total: u64, scene_total: u64) -> String {
    format!(
        "Khối anchor:\n{anchor}\n\n\
         Ngân sách: đúng {arc_total} arc, tổng {chapter_total} chương, tổng {scene_total} cảnh.\n\
         Trường `chapters` của mỗi arc là số chương của arc đó và tổng phải bằng {chapter_total}.\n\
         Trả về đúng một object JSON theo định dạng đã nêu."
    )
}

/// One chapter shard: a closed range plus the arcs it has to fit into.
pub fn chapters_user(
    anchor: &str,
    arcs_brief: &str,
    start: u64,
    end: u64,
    chapter_total: u64,
    scene_budget: u64,
) -> String {
    format!(
        "Khối anchor:\n{anchor}\n\n\
         Các arc đã chốt:\n{arcs_brief}\n\n\
         Viết chương {start} đến {end} trong tổng {chapter_total} chương.\n\
         Tổng `scene_count` của các chương này phải bằng {scene_budget}.\n\
         `position` chạy từ {start} đến {end}. Trả về đúng một object JSON theo định dạng đã nêu."
    )
}

pub fn cast_user(
    anchor: &str,
    chapters_brief: &str,
    character_range: &str,
    motif_codes: &[String],
    skill: &str,
) -> String {
    format!(
        "Anchor:\n{anchor}\n\nTóm tắt chương đã chốt:\n{chapters_brief}\n\nSố nhân vật cần tạo: {character_range}.\nMã motif hợp lệ: {}\nMạch truyện: {skill}",
        motif_codes.join(", ")
    )
}
pub fn world_user(anchor: &str, chapters_brief: &str, genre_code: &str) -> String {
    format!("Anchor:\n{anchor}\n\nTóm tắt chương:\n{chapters_brief}\n\nMã thể loại: {genre_code}")
}
pub fn bonds_user(anchor: &str, cast_brief: &str) -> String {
    format!("Anchor:\n{anchor}\n\nNhân vật đã chốt theo chỉ số:\n{cast_brief}")
}

/// One chapter of the scene stage: everything the binder must respect is in the
/// text, so it never has to guess a preset, a place or a character.
#[allow(clippy::too_many_arguments)]
pub fn scenes_user(
    anchor: &str,
    chapter_brief: &str,
    cast_brief: &str,
    settings_brief: &str,
    preset_codes: &str,
    narrator_codes: &str,
    directive_codes: &str,
    scene_budget: u64,
    word_budget: u64,
) -> String {
    format!(
        "Khối anchor:\n{anchor}\n\n\
         Chương cần chia cảnh:\n{chapter_brief}\n\n\
         Nhân vật đã chốt (dùng tên làm tham chiếu):\n{cast_brief}\n\n\
         Địa điểm đã chốt (dùng tên làm tham chiếu):\n{settings_brief}\n\n\
         style_preset_code hợp lệ: {preset_codes}\n\
         narrator_code hợp lệ: {narrator_codes}\n\
         directive code hợp lệ: {directive_codes}\n\n\
         Trả đúng {scene_budget} cảnh cho chương này, tổng target_words khoảng {word_budget}.\n\
         Trả về đúng một object JSON theo định dạng đã nêu."
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_prompts_state_the_prohibitions_and_the_contract() {
        for (lane, contract) in [
            (Lane::ArcSmith, "\"arcs\":[{\"position\""),
            (Lane::ChapterSmith, "\"chapters\":[{\"arc\""),
            (Lane::SceneBinder, "\"scenes\":[{\"position\""),
        ] {
            let text = system(lane);
            assert!(
                text.contains(contract),
                "{} misses its contract",
                lane.key()
            );
            assert!(text.contains("Cấm emoji"), "{} allows emoji", lane.key());
            assert!(
                text.contains("gạch ngang dài"),
                "{} allows long dashes",
                lane.key()
            );
            assert!(
                text.contains("không ghi vào state"),
                "{} must forbid writing state",
                lane.key()
            );
            assert!(
                !text.contains('\u{2013}') && !text.contains('\u{2014}'),
                "{} contains an en or em dash",
                lane.key()
            );
        }
    }

    #[test]
    fn test_user_prompts_carry_the_budget_numbers() {
        let arcs = arcs_user("anchor", 3, 12, 36);
        assert!(arcs.contains("đúng 3 arc"));
        assert!(arcs.contains("tổng 12 chương"));
        assert!(arcs.contains("tổng 36 cảnh"));

        let chapters = chapters_user("anchor", "1. Mưa", 11, 20, 120, 30);
        assert!(chapters.contains("chương 11 đến 20"));
        assert!(chapters.contains("bằng 30"));
        assert!(chapters.contains("từ 11 đến 20"));
    }
}
