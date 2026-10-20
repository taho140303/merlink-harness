//! Prompt builders for the phase-2 lanes.
//!
//! The system text lives in `prompts/outline_*.md` so the wording can be edited
//! without touching Rust; the user text carries the numbers of the current plan
//! (arc budget, chapter range, scene budget) which the model must respect.

use super::contract::Lane;

pub const ARCS_SYSTEM: &str = include_str!("../../prompts/outline_arcs.md");
pub const CHAPTERS_SYSTEM: &str = include_str!("../../prompts/outline_chapters.md");

/// System prompt for a lane. Only implemented lanes have one.
pub fn system(lane: Lane) -> &'static str {
    match lane {
        Lane::ArcSmith => ARCS_SYSTEM,
        Lane::ChapterSmith => CHAPTERS_SYSTEM,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_prompts_state_the_prohibitions_and_the_contract() {
        for (lane, contract) in [
            (Lane::ArcSmith, "\"arcs\":[{\"position\""),
            (Lane::ChapterSmith, "\"chapters\":[{\"arc\""),
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
