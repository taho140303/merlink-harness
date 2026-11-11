//! Deterministic entity `code` allocation and monotone per-parent `_counters` preservation
//! (`REQ-DOM-002`, `core.md` §3.8, `be-core.md` §4.2, `T-UNIT-02`, `P2S2-B`, `phase2_sprint2_deep1-F2`).
//!
//! Invariants enforced:
//! 1. Name-based slugs (`project`, `character`) normalize Vietnamese & Latin text to `[a-z0-9_]`
//!    (`đ/Đ -> d`, strip diacritics, collapse non-alphanumeric runs to `_`). The hyphen `-` is
//!    strictly reserved as the hierarchy and entity-type separator (`core.md` §3.8).
//! 2. Hierarchical child codes combine `<parent_code>-<type_prefix><N>` where `<N>` is drawn from
//!    the monotone counter `_counters[counter_key]`:
//!    - Arc under Project: `<project_code>-a<N>` (counter key `<project_code>`)
//!    - Chapter under Arc: `<arc_code>-c<N>` (counter key `<arc_code>`)
//!    - Scene under Chapter: `<chapter_code>-s<N>` (counter key `<chapter_code>`)
//!    - World Setting under Project: `<project_code>-w<N>` (counter key `<project_code>:w`)
//!    - Lore Entry under Project: `<project_code>-l<N>` (counter key `<project_code>:l`)
//!    - Theme under Arc: `<arc_code>-t<N>` (counter key `<arc_code>:t`)
//!    - Relationship under Project: `<project_code>-r<N>` (counter key `<project_code>:r`)
//!    - Competence under Character: `<character_code>-k<N>` (counter key `<character_code>:k`)
//!    - Standalone prefix allocation (`char-1`, `ws-1`, `lore-1`, `thm-1`, etc.) is also supported
//!      via `EntityCodeSpec::Prefixed { prefix }`.
//! 3. Once allocated, an entity's `code` never changes when its `display_name` or `position` changes.
//! 4. Deleting an entity from `BusinessFlowData` never decrements or reclaims its counter in `_counters`;
//!    subsequent allocations always advance to the next integer (`-c4` after deleting `-c2` when `-c3` existed).
//! 5. Any attempt to rewind a counter or publish a colliding `code` is rejected with `DomainError`
//!    without mutating the caller's `BusinessFlowData` (`phase2_sprint2_deep1-F2`).

use std::collections::BTreeSet;

use super::business_flow::{BusinessFlowData, DomainError, EntityCounters};

/// Specification for allocating a stable entity `code` (`be-core.md` §4.2).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntityCodeSpec<'a> {
    /// Root project code derived from `display_name` slug (e.g. `"te_nu_nguu_nhan"`), or fallback `"p1"`.
    Project { display_name: &'a str },
    /// Character code derived from `display_name` or `nickname` slug (e.g. `"minh_hoang"`, `"minh_hoang_2"`).
    Character { display_name_or_nickname: &'a str },
    /// Arc belonging to `project_code`: `<project_code>-a<N>` (counter key: `<project_code>`).
    Arc { project_code: &'a str },
    /// Chapter belonging to `arc_code`: `<arc_code>-c<N>` (counter key: `<arc_code>`).
    Chapter { arc_code: &'a str },
    /// Scene belonging to `chapter_code`: `<chapter_code>-s<N>` (counter key: `<chapter_code>`).
    Scene { chapter_code: &'a str },
    /// World setting belonging to `project_code`: `<project_code>-w<N>` (counter key: `<project_code>:w`).
    WorldSetting { project_code: &'a str },
    /// Lore entry belonging to `project_code`: `<project_code>-l<N>` (counter key: `<project_code>:l`).
    LoreEntry { project_code: &'a str },
    /// Theme belonging to `arc_code`: `<arc_code>-t<N>` (counter key: `<arc_code>:t`).
    Theme { arc_code: &'a str },
    /// Event block belonging to `arc_code`: `<arc_code>-eb<N>` (counter key: `<arc_code>:eb`).
    ArcEventBlock { arc_code: &'a str },
    /// Relationship belonging to `project_code`: `<project_code>-r<N>` (counter key: `<project_code>:r`).
    Relationship { project_code: &'a str },
    /// Character competence belonging to `character_code`: `<character_code>-k<N>` (counter key: `<character_code>:k`).
    Competence { character_code: &'a str },
    /// Generic prefix-based monotone code (`<prefix>-<N>`, e.g. `char-1`, `ws-1`, `lore-1`).
    Prefixed { prefix: &'a str },
}

/// Pure, stateless allocator for stable entity codes and monotone `_counters`.
pub struct CodeAllocator;

impl CodeAllocator {
    /// Normalizes a human-readable display name (including Vietnamese diacritics) into a valid
    /// base slug containing only `[a-z0-9_]` (`core.md` §3.8, `be-core.md` §4.2).
    ///
    /// Hyphens (`-`) are converted to `_` so a name slug can never forge hierarchy suffixes like `-a1` or `-c2`.
    pub fn slugify_name(input: &str) -> String {
        let mut out = String::with_capacity(input.len());
        let mut prev_underscore = false;

        for ch in input.chars() {
            let mapped = fold_vietnamese_and_latin_char(ch);
            if let Some(ascii_ch) = mapped {
                if ascii_ch.is_ascii_alphanumeric() {
                    out.push(ascii_ch.to_ascii_lowercase());
                    prev_underscore = false;
                } else if !prev_underscore && !out.is_empty() {
                    out.push('_');
                    prev_underscore = true;
                }
            } else if (ch.is_whitespace() || ch == '-' || ch == '_' || ch.is_ascii_punctuation())
                && !prev_underscore
                && !out.is_empty()
            {
                out.push('_');
                prev_underscore = true;
            }
        }

        while out.ends_with('_') {
            out.pop();
        }
        out
    }

    /// Allocates a new globally unique entity `code` within `state`, advancing `state.counters`
    /// monotonically when applicable (`REQ-DOM-002`, `be-core.md` §4.2).
    ///
    /// Performs all validation on a cloned candidate counter map before committing so that on any
    /// error (`DomainError`), `state` remains completely untouched (`phase2_sprint2_deep1-F2`).
    pub fn allocate(
        state: &mut BusinessFlowData,
        spec: EntityCodeSpec<'_>,
    ) -> Result<String, DomainError> {
        Self::allocate_with_committed_codes(state, spec, &BTreeSet::new())
    }

    /// Allocates a new entity `code` checking both `state` (WorkingState) and `committed_codes`
    /// (already committed database codes when provided) for collisions (`core.md` §3.8).
    pub fn allocate_with_committed_codes(
        state: &mut BusinessFlowData,
        spec: EntityCodeSpec<'_>,
        committed_codes: &BTreeSet<String>,
    ) -> Result<String, DomainError> {
        let mut occupied = state.collect_unique_entity_codes()?;
        for c in committed_codes {
            occupied.insert(c.clone());
        }

        let mut candidate_counters = state.counters.clone();
        let allocated_code = Self::allocate_on_parts(&mut candidate_counters, &occupied, spec)?;

        // Verify monotonicity before committing candidate counters back to state.
        Self::verify_counter_monotonicity(&state.counters, &candidate_counters)?;
        state.counters = candidate_counters;
        Ok(allocated_code)
    }

    /// Validates that `proposed` counters never rewind or drop any existing counter in `baseline`
    /// (`be-core.md` §4.2, `phase2_sprint2_deep1-F2`).
    pub fn verify_counter_monotonicity(
        baseline: &EntityCounters,
        proposed: &EntityCounters,
    ) -> Result<(), DomainError> {
        for (key, &current_val) in baseline {
            let proposed_val = proposed.get(key).copied().unwrap_or(0);
            if proposed_val < current_val {
                return Err(DomainError::CounterRewind {
                    counter_key: key.clone(),
                    current: current_val,
                    proposed: proposed_val,
                });
            }
        }
        Ok(())
    }

    /// Attempts to apply a counter update and register `new_code` into `state` atomically,
    /// rejecting any counter rewind or `code` collision without mutating `state` (`phase2_sprint2_deep1-F2`).
    pub fn validate_and_apply_counter_transition(
        state: &mut BusinessFlowData,
        proposed_counters: EntityCounters,
        new_code: Option<&str>,
    ) -> Result<(), DomainError> {
        let occupied = state.collect_unique_entity_codes()?;
        if let Some(code) = new_code {
            let trimmed = code.trim();
            if trimmed.is_empty() {
                return Err(DomainError::InvalidEntityCode {
                    code: code.to_string(),
                    reason: "entity code cannot be empty".to_string(),
                });
            }
            if occupied.contains(trimmed) {
                return Err(DomainError::CodeCollision {
                    code: trimmed.to_string(),
                });
            }
        }
        Self::verify_counter_monotonicity(&state.counters, &proposed_counters)?;
        state.counters = proposed_counters;
        Ok(())
    }

    fn allocate_on_parts(
        counters: &mut EntityCounters,
        occupied: &BTreeSet<String>,
        spec: EntityCodeSpec<'_>,
    ) -> Result<String, DomainError> {
        match spec {
            EntityCodeSpec::Project { display_name } => {
                let base = Self::slugify_name(display_name);
                if base.is_empty() {
                    Self::allocate_monotone_child(counters, occupied, "__root_project", "p", "")
                } else {
                    Ok(Self::allocate_unique_slug(&base, occupied))
                }
            }
            EntityCodeSpec::Character {
                display_name_or_nickname,
            } => {
                let base = Self::slugify_name(display_name_or_nickname);
                if base.is_empty() {
                    return Err(DomainError::InvalidEntityCode {
                        code: String::new(),
                        reason: "character display_name_or_nickname produced an empty slug"
                            .to_string(),
                    });
                }
                Ok(Self::allocate_unique_slug(&base, occupied))
            }
            EntityCodeSpec::Arc { project_code } => {
                let parent = Self::require_non_empty_parent(project_code, "project_code")?;
                Self::allocate_monotone_child(counters, occupied, parent, parent, "-a")
            }
            EntityCodeSpec::Chapter { arc_code } => {
                let parent = Self::require_non_empty_parent(arc_code, "arc_code")?;
                Self::allocate_monotone_child(counters, occupied, parent, parent, "-c")
            }
            EntityCodeSpec::Scene { chapter_code } => {
                let parent = Self::require_non_empty_parent(chapter_code, "chapter_code")?;
                Self::allocate_monotone_child(counters, occupied, parent, parent, "-s")
            }
            EntityCodeSpec::WorldSetting { project_code } => {
                let parent = Self::require_non_empty_parent(project_code, "project_code")?;
                let counter_key = format!("{parent}:w");
                Self::allocate_monotone_child(counters, occupied, &counter_key, parent, "-w")
            }
            EntityCodeSpec::LoreEntry { project_code } => {
                let parent = Self::require_non_empty_parent(project_code, "project_code")?;
                let counter_key = format!("{parent}:l");
                Self::allocate_monotone_child(counters, occupied, &counter_key, parent, "-l")
            }
            EntityCodeSpec::Theme { arc_code } => {
                let parent = Self::require_non_empty_parent(arc_code, "arc_code")?;
                let counter_key = format!("{parent}:t");
                Self::allocate_monotone_child(counters, occupied, &counter_key, parent, "-t")
            }
            EntityCodeSpec::ArcEventBlock { arc_code } => {
                let parent = Self::require_non_empty_parent(arc_code, "arc_code")?;
                let counter_key = format!("{parent}:eb");
                Self::allocate_monotone_child(counters, occupied, &counter_key, parent, "-eb")
            }
            EntityCodeSpec::Relationship { project_code } => {
                let parent = Self::require_non_empty_parent(project_code, "project_code")?;
                let counter_key = format!("{parent}:r");
                Self::allocate_monotone_child(counters, occupied, &counter_key, parent, "-r")
            }
            EntityCodeSpec::Competence { character_code } => {
                let parent = Self::require_non_empty_parent(character_code, "character_code")?;
                let counter_key = format!("{parent}:k");
                Self::allocate_monotone_child(counters, occupied, &counter_key, parent, "-k")
            }
            EntityCodeSpec::Prefixed { prefix } => {
                let clean = Self::require_non_empty_parent(prefix, "prefix")?;
                Self::allocate_monotone_child(counters, occupied, clean, clean, "-")
            }
        }
    }

    fn require_non_empty_parent<'a>(raw: &'a str, label: &str) -> Result<&'a str, DomainError> {
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            return Err(DomainError::InvalidEntityCode {
                code: raw.to_string(),
                reason: format!("{label} must not be empty"),
            });
        }
        Ok(trimmed)
    }

    fn allocate_unique_slug(base_slug: &str, occupied: &BTreeSet<String>) -> String {
        if !occupied.contains(base_slug) {
            return base_slug.to_string();
        }
        let mut suffix: u64 = 2;
        loop {
            let candidate = format!("{base_slug}_{suffix}");
            if !occupied.contains(&candidate) {
                return candidate;
            }
            suffix += 1;
        }
    }

    fn allocate_monotone_child(
        counters: &mut EntityCounters,
        occupied: &BTreeSet<String>,
        counter_key: &str,
        code_prefix: &str,
        separator_and_tag: &str,
    ) -> Result<String, DomainError> {
        let current = counters.get(counter_key).copied().unwrap_or(0);
        let mut next = current
            .checked_add(1)
            .ok_or_else(|| DomainError::InvalidEntityCode {
                code: code_prefix.to_string(),
                reason: format!("counter overflow for key '{counter_key}'"),
            })?;

        loop {
            let candidate = format!("{code_prefix}{separator_and_tag}{next}");
            if !occupied.contains(&candidate) {
                counters.insert(counter_key.to_string(), next);
                return Ok(candidate);
            }
            next = next
                .checked_add(1)
                .ok_or_else(|| DomainError::InvalidEntityCode {
                    code: code_prefix.to_string(),
                    reason: format!("counter overflow for key '{counter_key}'"),
                })?;
        }
    }
}

/// Maps Vietnamese and common Latin accented characters to their base ASCII character (`be-core.md` §4.2).
fn fold_vietnamese_and_latin_char(ch: char) -> Option<char> {
    if ch.is_ascii_alphanumeric() {
        return Some(ch);
    }
    match ch {
        'à' | 'á' | 'ả' | 'ã' | 'ạ' | 'ă' | 'ằ' | 'ắ' | 'ẳ' | 'ẵ' | 'ặ' | 'â' | 'ầ' | 'ấ' | 'ẩ'
        | 'ẫ' | 'ậ' | 'À' | 'Á' | 'Ả' | 'Ã' | 'Ạ' | 'Ă' | 'Ằ' | 'Ắ' | 'Ẳ' | 'Ẵ' | 'Ặ' | 'Â'
        | 'Ầ' | 'Ấ' | 'Ẩ' | 'Ẫ' | 'Ậ' | 'ä' | 'å' | 'æ' | 'Ä' | 'Å' | 'Æ' => {
            Some('a')
        }
        'đ' | 'Đ' => Some('d'),
        'è' | 'é' | 'ẻ' | 'ẽ' | 'ẹ' | 'ê' | 'ề' | 'ế' | 'ể' | 'ễ' | 'ệ' | 'È' | 'É' | 'Ẻ' | 'Ẽ'
        | 'Ẹ' | 'Ê' | 'Ề' | 'Ế' | 'Ể' | 'Ễ' | 'Ệ' | 'ë' | 'Ë' => Some('e'),
        'ì' | 'í' | 'ỉ' | 'ĩ' | 'ị' | 'Ì' | 'Í' | 'Ỉ' | 'Ĩ' | 'Ị' | 'î' | 'ï' | 'Î' | 'Ï' => {
            Some('i')
        }
        'ò' | 'ó' | 'ỏ' | 'õ' | 'ọ' | 'ô' | 'ồ' | 'ố' | 'ổ' | 'ỗ' | 'ộ' | 'ơ' | 'ờ' | 'ớ' | 'ở'
        | 'ỡ' | 'ợ' | 'Ò' | 'Ó' | 'Ỏ' | 'Õ' | 'Ọ' | 'Ô' | 'Ồ' | 'Ố' | 'Ổ' | 'Ỗ' | 'Ộ' | 'Ơ'
        | 'Ờ' | 'Ớ' | 'Ở' | 'Ỡ' | 'Ợ' | 'ö' | 'ø' | 'Ö' | 'Ø' => Some('o'),
        'ù' | 'ú' | 'ủ' | 'ũ' | 'ụ' | 'ư' | 'ừ' | 'ứ' | 'ử' | 'ữ' | 'ự' | 'Ù' | 'Ú' | 'Ủ' | 'Ũ'
        | 'Ụ' | 'Ư' | 'Ừ' | 'Ứ' | 'Ử' | 'Ữ' | 'Ự' | 'ü' | 'û' | 'Ü' | 'Û' => {
            Some('u')
        }
        'ỳ' | 'ý' | 'ỷ' | 'ỹ' | 'ỵ' | 'Ỳ' | 'Ý' | 'Ỷ' | 'Ỹ' | 'Ỵ' | 'ÿ' | 'Ÿ' => {
            Some('y')
        }
        'ç' | 'Ç' => Some('c'),
        'ñ' | 'Ñ' => Some('n'),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::business_flow::{
        ChapterPhaseArcItem, ChapterPhaseChapterItem, OverviewArcItem, TrackedField,
    };

    #[test]
    fn vietnamese_slugify_replaces_diacritics_and_strips_hyphens() {
        assert_eq!(
            CodeAllocator::slugify_name("Tế Nữ Ngưu Nhân - Phần 1"),
            "te_nu_nguu_nhan_phan_1"
        );
        assert_eq!(
            CodeAllocator::slugify_name("Độc Cô Cầu Bại"),
            "doc_co_cau_bai"
        );
    }

    #[test]
    fn allocate_arc_chapter_scene_monotone_after_delete() {
        let mut state = BusinessFlowData::new();
        let proj_code = CodeAllocator::allocate(
            &mut state,
            EntityCodeSpec::Project {
                display_name: "Tế Nữ Ngưu Nhân",
            },
        )
        .unwrap();
        assert_eq!(proj_code, "te_nu_nguu_nhan");
        state.require.project.code = Some(proj_code.clone());

        let arc1 = CodeAllocator::allocate(
            &mut state,
            EntityCodeSpec::Arc {
                project_code: &proj_code,
            },
        )
        .unwrap();
        assert_eq!(arc1, "te_nu_nguu_nhan-a1");
        state.optional.overview_phase.arcs.push(OverviewArcItem {
            code: arc1.clone(),
            display_name: TrackedField::user("Hồi 1".to_string()),
            ..Default::default()
        });
        state.optional.chapter_phase.arcs.push(ChapterPhaseArcItem {
            code: arc1.clone(),
            chapters: vec![],
        });

        let c1 = CodeAllocator::allocate(&mut state, EntityCodeSpec::Chapter { arc_code: &arc1 })
            .unwrap();
        let c2 = CodeAllocator::allocate(&mut state, EntityCodeSpec::Chapter { arc_code: &arc1 })
            .unwrap();
        let c3 = CodeAllocator::allocate(&mut state, EntityCodeSpec::Chapter { arc_code: &arc1 })
            .unwrap();
        assert_eq!(c1, "te_nu_nguu_nhan-a1-c1");
        assert_eq!(c2, "te_nu_nguu_nhan-a1-c2");
        assert_eq!(c3, "te_nu_nguu_nhan-a1-c3");

        state.optional.chapter_phase.arcs[0].chapters = vec![
            ChapterPhaseChapterItem {
                code: c1,
                ..Default::default()
            },
            ChapterPhaseChapterItem {
                code: c2.clone(),
                ..Default::default()
            },
            ChapterPhaseChapterItem {
                code: c3,
                ..Default::default()
            },
        ];

        // Physical removal of c2
        state.optional.chapter_phase.arcs[0]
            .chapters
            .retain(|ch| ch.code != c2);

        // Next chapter must receive -c4, never reusing -c2
        let c4 = CodeAllocator::allocate(&mut state, EntityCodeSpec::Chapter { arc_code: &arc1 })
            .unwrap();
        assert_eq!(c4, "te_nu_nguu_nhan-a1-c4");
        assert_eq!(state.counters.get(&arc1).copied(), Some(4));
    }
}
