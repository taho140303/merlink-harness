//! Code-keyed catalogue of shared, read-only reference data.
//!
//! Phase 2 lets a model *choose* a style preset, a narrator, a narrative
//! directive or a motif, but it never invents one: the generator prompts list
//! these codes and the commit step rejects anything that is not here.
//!
//! The JSON files live in `docs/schema/catalog/` and are generated from the
//! PostgreSQL seed scripts by `scripts/gen-catalog.mjs`. They are embedded with
//! `include_str!`, so the shipped binary parses no SQL and needs no database
//! table for reference data.

use std::collections::BTreeMap;
use std::sync::OnceLock;

use serde_json::Value;

/// One reference catalogue. Adding a kind means adding an arm in [`Kind::as_str`]
/// and a `include_str!` line in [`dataset`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Genres,
    Motifs,
    NarratorConfigs,
    NarrativeDirectives,
    RelationshipCategories,
    StylePresets,
}

impl Kind {
    /// The on-disk name and the key used by the prompt builder.
    pub fn as_str(self) -> &'static str {
        match self {
            Kind::Genres => "genres",
            Kind::Motifs => "motifs",
            Kind::NarratorConfigs => "narrator_configs",
            Kind::NarrativeDirectives => "narrative_directives",
            Kind::RelationshipCategories => "relationship_categories",
            Kind::StylePresets => "style_presets",
        }
    }

    /// Every kind. Tests iterate the whole catalogue with it.
    #[cfg(test)]
    pub const ALL: [Kind; 6] = [
        Kind::Genres,
        Kind::Motifs,
        Kind::NarratorConfigs,
        Kind::NarrativeDirectives,
        Kind::RelationshipCategories,
        Kind::StylePresets,
    ];
}

/// One catalogue row, flattened so callers never touch raw JSON for the common
/// fields. `data` keeps the rest (sensory weights, devices, notes, ...).
#[derive(Clone, Debug)]
pub struct Entry {
    pub code: String,
    pub name: String,
    /// Raw catalogue row. Sprint 17.5 reads the style weights, vocabulary and
    /// rhetorical devices from here; nothing consumes it before that.
    #[allow(dead_code)]
    pub data: Value,
}

static CATALOG: OnceLock<BTreeMap<&'static str, Vec<Entry>>> = OnceLock::new();

const GENRES_JSON: &str = include_str!("../../../docs/schema/catalog/genres.json");
const MOTIFS_JSON: &str = include_str!("../../../docs/schema/catalog/motifs.json");
const NARRATORS_JSON: &str = include_str!("../../../docs/schema/catalog/narrator_configs.json");
const DIRECTIVES_JSON: &str =
    include_str!("../../../docs/schema/catalog/narrative_directives.json");
const RELATIONSHIP_CATEGORIES_JSON: &str =
    include_str!("../../../docs/schema/catalog/relationship_categories.json");
const STYLE_PRESETS_JSON: &str = include_str!("../../../docs/schema/catalog/style_presets.json");

fn parse(raw: &str, kind: Kind) -> Vec<Entry> {
    let value: Value = serde_json::from_str(raw)
        .unwrap_or_else(|error| panic!("catalogue {} must be valid JSON: {error}", kind.as_str()));
    let array = value
        .as_array()
        .unwrap_or_else(|| panic!("catalogue {} must be a JSON array", kind.as_str()));

    let mut entries: Vec<Entry> = array
        .iter()
        .map(|item| {
            let code = item["code"].as_str().unwrap_or_default();
            assert!(
                !code.is_empty(),
                "catalogue {} has an entry without a code",
                kind.as_str()
            );
            Entry {
                code: code.to_owned(),
                name: item["name"].as_str().unwrap_or(code).to_owned(),
                data: item.clone(),
            }
        })
        .collect();
    entries.sort_by(|left, right| left.code.cmp(&right.code));
    entries
}

fn dataset() -> &'static BTreeMap<&'static str, Vec<Entry>> {
    CATALOG.get_or_init(|| {
        let mut map = BTreeMap::new();
        map.insert(Kind::Genres.as_str(), parse(GENRES_JSON, Kind::Genres));
        map.insert(Kind::Motifs.as_str(), parse(MOTIFS_JSON, Kind::Motifs));
        map.insert(
            Kind::NarratorConfigs.as_str(),
            parse(NARRATORS_JSON, Kind::NarratorConfigs),
        );
        map.insert(
            Kind::NarrativeDirectives.as_str(),
            parse(DIRECTIVES_JSON, Kind::NarrativeDirectives),
        );
        map.insert(
            Kind::RelationshipCategories.as_str(),
            parse(RELATIONSHIP_CATEGORIES_JSON, Kind::RelationshipCategories),
        );
        map.insert(
            Kind::StylePresets.as_str(),
            parse(STYLE_PRESETS_JSON, Kind::StylePresets),
        );
        map
    })
}

/// Every entry of one catalogue, sorted by code.
pub fn all(kind: Kind) -> &'static [Entry] {
    dataset()
        .get(kind.as_str())
        .map(Vec::as_slice)
        .unwrap_or(&[])
}

#[cfg(test)]
mod tests {
    use super::*;

    fn find(kind: Kind, code: &str) -> Option<&'static Entry> {
        all(kind).iter().find(|entry| entry.code == code)
    }

    #[test]
    fn test_catalog_files_parse_and_are_non_empty() {
        for kind in Kind::ALL {
            assert!(
                !all(kind).is_empty(),
                "catalogue {} must not be empty; regenerate with \
                 `node scripts/gen-catalog.mjs`",
                kind.as_str()
            );
        }
    }

    #[test]
    fn test_catalog_codes_are_unique_and_slug_safe() {
        for kind in Kind::ALL {
            let mut seen = std::collections::BTreeSet::new();
            for entry in all(kind) {
                assert!(
                    !entry.code.is_empty(),
                    "{} has an empty code",
                    kind.as_str()
                );
                assert!(
                    !entry.code.chars().any(char::is_whitespace),
                    "{} code {:?} contains whitespace",
                    kind.as_str(),
                    entry.code
                );
                assert!(
                    seen.insert(entry.code.as_str()),
                    "{} has a duplicate code {:?}",
                    kind.as_str(),
                    entry.code
                );
            }
        }
    }

    #[test]
    fn test_catalog_is_sorted_and_addressable_by_code() {
        for kind in Kind::ALL {
            let entries = all(kind);
            assert!(
                entries.windows(2).all(|pair| pair[0].code <= pair[1].code),
                "{} must stay sorted by code",
                kind.as_str()
            );
            for entry in entries.iter().take(3) {
                let found = find(kind, &entry.code).expect("entry must be addressable by code");
                assert_eq!(found.name, entry.name);
            }
            assert!(
                find(kind, "definitely_not_a_code").is_none(),
                "{} must not resolve an invented code",
                kind.as_str()
            );
        }
    }

    #[test]
    fn test_catalog_known_codes_and_counts() {
        assert!(find(Kind::Genres, "fantasy").is_some());
        assert!(find(Kind::Genres, "mystery").is_some());
        assert!(find(Kind::NarratorConfigs, "first_person_protagonist").is_some());
        assert!(find(Kind::NarrativeDirectives, "show_dont_tell").is_some());
        assert!(find(Kind::StylePresets, "action.battle").is_some());
        assert!(find(Kind::RelationshipCategories, "FAMILY.father").is_some());
        assert_eq!(all(Kind::Genres).len(), 20);
        assert_eq!(all(Kind::Motifs).len(), 125);
        assert_eq!(all(Kind::NarratorConfigs).len(), 14);
        assert_eq!(all(Kind::NarrativeDirectives).len(), 21);
        assert_eq!(all(Kind::RelationshipCategories).len(), 43);
        assert_eq!(all(Kind::StylePresets).len(), 54);
    }

    #[test]
    fn test_style_preset_carries_its_writing_material() {
        let preset = find(Kind::StylePresets, "action.battle").expect("action.battle");
        assert_eq!(preset.data["tone_pace"], "FAST");
        assert!(
            preset.data["vocab"]
                .as_array()
                .is_some_and(|items| !items.is_empty()),
            "action.battle must carry vocabulary hints"
        );
        let devices = preset.data["devices"].as_array().expect("devices array");
        assert!(!devices.is_empty());
        assert!(devices.iter().any(|device| device["usage_type"] == "avoid"));
    }
}
