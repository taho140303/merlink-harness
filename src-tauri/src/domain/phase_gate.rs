//! Phase gating: a phase may only be left once its own optional payload is complete.
//!
//! `core.md` §2.8 lists cross-entity invariants, but the runtime previously only gated Phase 0
//! (`is_require_complete`); every later phase reported `can_advance_phase = true`. This module adds
//! an explicit completeness contract per phase, used by `confirm_phase`, the progress DTO and the
//! workflow launcher. Messages are user-facing Vietnamese hints, never raw field dumps.

use crate::domain::business_flow::{
    BusinessFlowData, CharacterAppearanceItem, CharacterLifeExperienceItem,
    CharacterPsychologicalItem, TrackedField,
};

fn blank(field: &TrackedField<String>) -> bool {
    field
        .value
        .as_deref()
        .map(str::trim)
        .map(str::is_empty)
        .unwrap_or(true)
}

fn scene_codes(flow: &BusinessFlowData) -> Vec<String> {
    flow.optional
        .chapter_phase
        .arcs
        .iter()
        .flat_map(|arc| arc.chapters.iter())
        .flat_map(|chapter| chapter.scenes.iter())
        .map(|scene| scene.code.clone())
        .collect()
}

fn world_setting_codes(flow: &BusinessFlowData) -> Vec<String> {
    flow.optional
        .world_building_phase
        .world_settings
        .iter()
        .map(|setting| setting.code.clone())
        .collect()
}

fn appearance_missing(flow: &BusinessFlowData) -> Vec<String> {
    let mut missing = Vec::new();
    for character in &flow.require.character {
        let label = character
            .display_name
            .value
            .clone()
            .or_else(|| character.nickname.value.clone())
            .unwrap_or_else(|| character.code.clone());
        let Some(appearance) = flow
            .optional
            .character_appearance_phase
            .character
            .iter()
            .find(|item| item.code == character.code)
        else {
            missing.push(format!("Nhân vật {label}: thiếu hồ sơ ngoại hình (5.1)"));
            continue;
        };
        if !appearance_missing_fields(appearance).is_empty() {
            missing.push(format!(
                "Nhân vật {label}: ngoại hình còn thiếu {}",
                appearance_missing_fields(appearance).join(", ")
            ));
        }
    }
    missing
}

fn appearance_missing_fields(appearance: &CharacterAppearanceItem) -> Vec<&'static str> {
    let mut fields = Vec::new();
    if blank(&appearance.default_summary) && blank(&appearance.species) {
        fields.push("default_summary/species");
    }
    fields
}

fn psychological_missing(flow: &BusinessFlowData) -> Vec<String> {
    let mut missing = Vec::new();
    for character in &flow.require.character {
        let label = character
            .display_name
            .value
            .clone()
            .or_else(|| character.nickname.value.clone())
            .unwrap_or_else(|| character.code.clone());
        let Some(psychology) = flow
            .optional
            .character_psychological_phase
            .character
            .iter()
            .find(|item| item.code == character.code)
        else {
            missing.push(format!("Nhân vật {label}: thiếu hồ sơ tâm lý (5.2)"));
            continue;
        };
        if let Some(problems) = psychological_missing_fields(psychology) {
            missing.push(format!("Nhân vật {label}: tâm lý còn thiếu {problems}"));
        }
    }
    missing
}

fn psychological_missing_fields(item: &CharacterPsychologicalItem) -> Option<&'static str> {
    if blank(&item.core_trauma_and_belief.the_wound)
        && blank(&item.narrative_engine.external_want)
        && blank(&item.narrative_engine.internal_need)
    {
        return Some("the_wound/external_want/internal_need");
    }
    None
}

fn life_experience_missing(flow: &BusinessFlowData) -> Vec<String> {
    let mut missing = Vec::new();
    let scenes = scene_codes(flow);
    for character in &flow.require.character {
        let label = character
            .display_name
            .value
            .clone()
            .or_else(|| character.nickname.value.clone())
            .unwrap_or_else(|| character.code.clone());
        let Some(life) = flow
            .optional
            .character_life_experience_phase
            .character
            .iter()
            .find(|item| item.code == character.code)
        else {
            missing.push(format!("Nhân vật {label}: thiếu ký ức/năng lực (5.3)"));
            continue;
        };
        if let Some(problem) = life_experience_missing_fields(life, &scenes) {
            missing.push(format!("Nhân vật {label}: {problem}"));
        }
    }
    missing
}

fn life_experience_missing_fields(
    item: &CharacterLifeExperienceItem,
    scenes: &[String],
) -> Option<String> {
    if item.memories.is_empty() && item.competences.is_empty() {
        return Some("chưa có ký ức hoặc năng lực".to_string());
    }
    if let Some(competence) = item.competences.iter().find(|competence| {
        let code = competence.scene_code.value.clone().unwrap_or_default();
        code.trim().is_empty() || !scenes.iter().any(|scene| scene == &code)
    }) {
        return Some(format!(
            "năng lực {} chưa gắn cảnh thể hiện hợp lệ",
            competence
                .display_name
                .value
                .clone()
                .unwrap_or_else(|| competence.code.clone())
        ));
    }
    None
}

fn relationship_missing(flow: &BusinessFlowData) -> Vec<String> {
    let characters: Vec<&str> = flow
        .require
        .character
        .iter()
        .map(|character| character.code.as_str())
        .collect();
    if characters.len() < 2 {
        return Vec::new();
    }
    let relationships = &flow.optional.relationship_phase.relationship;
    if relationships.is_empty() {
        return vec!["Chưa khai báo quan hệ nào giữa các nhân vật".to_string()];
    }
    let mut missing = Vec::new();
    for relationship in relationships {
        let from = relationship.character.from_code.as_str();
        let to = relationship.character.to_code.as_str();
        if from.trim().is_empty() || to.trim().is_empty() {
            missing.push(format!(
                "Quan hệ {} thiếu from_code/to_code",
                relationship.code
            ));
            continue;
        }
        if !characters.contains(&from) || !characters.contains(&to) {
            missing.push(format!(
                "Quan hệ {} tham chiếu nhân vật không tồn tại",
                relationship.code
            ));
        }
    }
    missing
}

/// Remove duplicate Arc rows that share a `position`, keeping the first occurrence and dropping
/// the duplicated Arc plus its chapter/scene containers. Repair for drafts produced before
/// `add_arc` merged by position (live draft had 9 Arcs for a target of 5, positions 2..5 twice).
pub fn dedupe_arcs_by_position(flow: &mut BusinessFlowData) -> usize {
    let mut seen = std::collections::HashSet::new();
    let mut removed_codes: Vec<String> = Vec::new();
    let mut kept = Vec::new();
    for arc in std::mem::take(&mut flow.optional.overview_phase.arcs) {
        let duplicate = match arc.position.value {
            Some(position) => !seen.insert(position),
            None => false,
        };
        if duplicate {
            removed_codes.push(arc.code);
        } else {
            kept.push(arc);
        }
    }
    flow.optional.overview_phase.arcs = kept;
    if removed_codes.is_empty() {
        return 0;
    }
    flow.optional
        .chapter_phase
        .arcs
        .retain(|container| !removed_codes.contains(&container.code));
    flow.optional
        .scene_config_phase
        .arcs
        .retain(|container| !removed_codes.contains(&container.code));
    removed_codes.len()
}

/// Missing items for a whole phase. Phase 0 delegates to the 46-attribute evaluator; phases 1–6
/// check their own optional payload. The list is empty when the phase may be left.
pub fn missing_for_phase(flow: &BusinessFlowData, phase: i32) -> Vec<String> {
    match phase {
        0 => {
            let eval = crate::domain::progress_engine::evaluate_progress(flow);
            if eval.is_require_complete {
                Vec::new()
            } else {
                vec!["Phase 0: còn trường require chưa hoàn tất".to_string()]
            }
        }
        1 => {
            let mut missing = Vec::new();
            let target = flow.require.project.target.arc_count.value.unwrap_or(0);
            let arcs = &flow.optional.overview_phase.arcs;
            if arcs.len() as u32 != target {
                missing.push(format!(
                    "Phase 1: cần {} Arc, hiện có {}",
                    target,
                    arcs.len()
                ));
            }
            for arc in arcs {
                let label = arc
                    .display_name
                    .value
                    .clone()
                    .unwrap_or_else(|| arc.code.clone());
                if blank(&arc.display_name) || blank(&arc.summary) {
                    missing.push(format!("Arc {label}: thiếu tên hoặc tóm tắt"));
                }
                if blank(&arc.motif.code) {
                    missing.push(format!("Arc {label}: thiếu motif xung đột"));
                }
                if arc.motif.event_blocks.is_empty() {
                    missing.push(format!("Arc {label}: thiếu khối biến cố"));
                }
                if blank(&arc.themes.question) || blank(&arc.themes.message) {
                    missing.push(format!("Arc {label}: thiếu câu hỏi chủ đề hoặc thông điệp"));
                }
            }
            missing
        }
        2 => {
            let mut missing = Vec::new();
            let target = flow.require.project.target.chapter_count.value.unwrap_or(0);
            let chapters: Vec<_> = flow
                .optional
                .chapter_phase
                .arcs
                .iter()
                .flat_map(|arc| arc.chapters.iter())
                .collect();
            if chapters.len() as u32 != target {
                missing.push(format!(
                    "Phase 2: cần {} chương, hiện có {}",
                    target,
                    chapters.len()
                ));
            }
            for chapter in chapters {
                let label = chapter
                    .display_name
                    .value
                    .clone()
                    .unwrap_or_else(|| chapter.code.clone());
                if blank(&chapter.display_name) || blank(&chapter.summary) {
                    missing.push(format!("Chương {label}: thiếu tên hoặc tóm tắt"));
                }
                if chapter.scenes.is_empty() {
                    missing.push(format!("Chương {label}: chưa có cảnh nào"));
                }
                for scene in &chapter.scenes {
                    if blank(&scene.display_name) || blank(&scene.summary) {
                        missing.push(format!(
                            "Cảnh {} (chương {label}): thiếu tên hoặc tóm tắt",
                            scene.code
                        ));
                    }
                }
            }
            missing
        }
        3 => {
            let mut missing = Vec::new();
            let settings = &flow.optional.world_building_phase.world_settings;
            if settings.is_empty() {
                missing.push("Phase 3: chưa có bối cảnh thế giới nào".to_string());
            }
            let codes = world_setting_codes(flow);
            for setting in settings {
                let label = setting
                    .display_name
                    .value
                    .clone()
                    .unwrap_or_else(|| setting.code.clone());
                if blank(&setting.display_name) || blank(&setting.setting_type) {
                    missing.push(format!("Bối cảnh {label}: thiếu tên hoặc loại địa hình"));
                }
                let parent = setting.parent_code.value.clone().unwrap_or_default();
                if !parent.trim().is_empty() && !codes.iter().any(|code| code == &parent) {
                    missing.push(format!("Bối cảnh {label}: parent_code không tồn tại"));
                }
            }
            missing
        }
        4 => {
            let mut missing = Vec::new();
            let worlds = world_setting_codes(flow);
            let mut configured: Vec<String> = Vec::new();
            for arc in &flow.optional.scene_config_phase.arcs {
                for chapter in &arc.chapters {
                    for scene in &chapter.scenes {
                        configured.push(scene.code.clone());
                        if blank(&scene.world_setting_code) {
                            missing.push(format!("Cảnh {}: chưa gán địa điểm", scene.code));
                        } else {
                            let code = scene.world_setting_code.value.clone().unwrap_or_default();
                            if !worlds.iter().any(|world| world == &code) {
                                missing.push(format!(
                                    "Cảnh {}: world_setting_code không tồn tại",
                                    scene.code
                                ));
                            }
                        }
                        if blank(&scene.narrator_config_code) {
                            missing.push(format!("Cảnh {}: chưa gán người kể chuyện", scene.code));
                        }
                        if blank(&scene.style_preset_code) {
                            missing.push(format!("Cảnh {}: chưa gán phong cách", scene.code));
                        }
                    }
                }
            }
            for scene in scene_codes(flow) {
                if !configured.iter().any(|code| code == &scene) {
                    missing.push(format!("Cảnh {scene}: chưa có cấu hình phân cảnh (4)"));
                }
            }
            missing
        }
        5 => {
            let mut missing = appearance_missing(flow);
            missing.extend(psychological_missing(flow));
            missing.extend(life_experience_missing(flow));
            missing
        }
        6 => relationship_missing(flow),
        _ => Vec::new(),
    }
}

/// Completeness of a single Phase-5 sub-phase (5.1, 5.2, 5.3). Used when moving inside Phase 5.
pub fn missing_for_subphase(flow: &BusinessFlowData, subphase: i32) -> Vec<String> {
    match subphase {
        1 => appearance_missing(flow),
        2 => psychological_missing(flow),
        3 => life_experience_missing(flow),
        _ => Vec::new(),
    }
}

/// True when the draft may leave `phase` (or, inside Phase 5, the current sub-phase).
pub fn can_advance(flow: &BusinessFlowData, phase: i32, subphase: Option<i32>) -> bool {
    if phase == 5 {
        let current = subphase.unwrap_or(1);
        return missing_for_subphase(flow, current).is_empty()
            && missing_for_phase(flow, 5).is_empty();
    }
    missing_for_phase(flow, phase).is_empty()
}

#[cfg(test)]
#[allow(clippy::field_reassign_with_default)]
mod tests {
    use super::*;
    use crate::domain::business_flow::{
        ArcEventBlockItem, ArcThemeSection, ChapterPhaseArcItem, ChapterPhaseChapterItem,
        ChapterPhaseSceneItem, OverviewArcItem, RequireCharacterItem,
    };

    fn complete_require() -> BusinessFlowData {
        let mut flow = BusinessFlowData::new();
        let project = &mut flow.require.project;
        project.display_name = TrackedField::user("Truyện".into());
        project.summary = TrackedField::user("Tóm tắt".into());
        project.language = TrackedField::user("VI".into());
        project.target.arc_count = TrackedField::user(5);
        project.target.chapter_count = TrackedField::user(150);
        project.target.word_per_chapter_count = TrackedField::user(2000);
        flow.require
            .project
            .genres
            .push(crate::domain::business_flow::RequireGenreItem {
                code: TrackedField::user("romance".into()),
                is_primary: TrackedField::user(true),
                display_name: TrackedField::user("Lãng mạn".into()),
            });
        let mut character = RequireCharacterItem::default();
        character.code = "hero".into();
        character.display_name = TrackedField::user("Anh hùng".into());
        character.biological_sex = TrackedField::user("MALE".into());
        character.motif_code = TrackedField::user("positive_change_arc".into());
        flow.require.character.push(character);
        flow
    }

    fn with_arcs(mut flow: BusinessFlowData) -> BusinessFlowData {
        let mut arc = OverviewArcItem::default();
        arc.code = "arc-1".into();
        arc.position = TrackedField::user(1);
        arc.display_name = TrackedField::user("Arc 1".into());
        arc.summary = TrackedField::user("Tóm tắt".into());
        arc.motif.code = TrackedField::user("positive_change_arc".into());
        arc.motif.event_blocks.push(ArcEventBlockItem::default());
        arc.themes = ArcThemeSection {
            question: TrackedField::user("Câu hỏi?".into()),
            message: TrackedField::user("Thông điệp".into()),
            ..Default::default()
        };
        flow.optional.overview_phase.arcs.push(arc);
        for index in 0..5u32 {
            let mut arc = OverviewArcItem::default();
            arc.code = format!("arc-{}", index + 1);
            arc.position = TrackedField::user(index + 1);
            arc.display_name = TrackedField::user(format!("Arc {}", index + 1));
            arc.summary = TrackedField::user("Tóm tắt".into());
            arc.motif.code = TrackedField::user("positive_change_arc".into());
            arc.motif.event_blocks.push(ArcEventBlockItem::default());
            arc.themes.question = TrackedField::user("Câu hỏi?".into());
            arc.themes.message = TrackedField::user("Thông điệp".into());
            if index > 0 {
                flow.optional.overview_phase.arcs.push(arc);
            }
        }
        flow
    }

    fn with_chapters(mut flow: BusinessFlowData) -> BusinessFlowData {
        flow.optional.chapter_phase.arcs.clear();
        for arc in &flow.optional.overview_phase.arcs {
            let mut chapter_arc = ChapterPhaseArcItem {
                code: arc.code.clone(),
                chapters: Vec::new(),
            };
            for position in 0..30u32 {
                let mut scene = ChapterPhaseSceneItem::default();
                scene.code = format!("{}-c{}-s1", arc.code, position + 1);
                scene.display_name = TrackedField::user("Cảnh".into());
                scene.summary = TrackedField::user("Diễn biến".into());
                let mut chapter = ChapterPhaseChapterItem::default();
                chapter.code = format!("{}-c{}", arc.code, position + 1);
                chapter.display_name = TrackedField::user("Chương".into());
                chapter.summary = TrackedField::user("Tóm tắt".into());
                chapter.scenes.push(scene);
                chapter_arc.chapters.push(chapter);
            }
            flow.optional.chapter_phase.arcs.push(chapter_arc);
        }
        flow
    }

    #[test]
    fn phase_zero_is_gated_by_require_completeness() {
        let mut flow = BusinessFlowData::new();
        assert!(!can_advance(&flow, 0, None));
        flow = complete_require();
        assert!(can_advance(&flow, 0, None));
    }

    #[test]
    fn phase_one_requires_the_configured_arc_count_and_arc_fields() {
        let flow = complete_require();
        let missing = missing_for_phase(&flow, 1);
        assert!(!missing.is_empty());
        let flow = with_arcs(flow);
        assert!(
            missing_for_phase(&flow, 1).is_empty(),
            "{:?}",
            missing_for_phase(&flow, 1)
        );
        let mut broken = flow.clone();
        broken.optional.overview_phase.arcs[0].summary = TrackedField::default();
        assert!(!missing_for_phase(&broken, 1).is_empty());
    }

    #[test]
    fn duplicate_arc_positions_are_collapsed_to_the_first_occurrence() {
        let mut flow = with_arcs(complete_require());
        // Simulate the live corruption: positions 2..5 generated a second time with new codes.
        for (index, position) in [2u32, 3, 4, 5].iter().enumerate() {
            let mut duplicate = OverviewArcItem::default();
            duplicate.code = format!("arc-{}", index + 6);
            duplicate.position = TrackedField::user(*position);
            duplicate.display_name = TrackedField::user("Duplicate".into());
            flow.optional.overview_phase.arcs.push(duplicate);
        }
        assert_eq!(flow.optional.overview_phase.arcs.len(), 9);
        let removed = dedupe_arcs_by_position(&mut flow);
        assert_eq!(removed, 4);
        assert_eq!(flow.optional.overview_phase.arcs.len(), 5);
        let positions: Vec<u32> = flow
            .optional
            .overview_phase
            .arcs
            .iter()
            .filter_map(|arc| arc.position.value)
            .collect();
        let mut unique = positions.clone();
        unique.sort_unstable();
        unique.dedup();
        assert_eq!(positions.len(), unique.len(), "positions must stay unique");
        assert!(flow
            .optional
            .overview_phase
            .arcs
            .iter()
            .all(|arc| !arc.code.starts_with("arc-6")));
    }

    #[test]
    fn phase_two_requires_exact_chapter_count_and_scene_summaries() {
        let flow = with_chapters(with_arcs(complete_require()));
        assert!(missing_for_phase(&flow, 2).is_empty());
        let mut broken = flow.clone();
        broken.optional.chapter_phase.arcs[0].chapters[0].scenes[0].summary =
            TrackedField::default();
        assert!(!missing_for_phase(&broken, 2).is_empty());
    }
}
