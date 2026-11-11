//! Pure Phase 3 Stage 2 merger (`core.md` §3.9, `be-core.md` §6.4.7 owner-ratified shape).
//!
//! Consumes validated `ProbeCandidatesV1` reports and produces the canonical candidate set that
//! `world_write` tasks consume. The merger only joins candidates that are provably identical;
//! semantic look-alikes and conflicting payloads stop the merge as `Ambiguous` so the user can
//! decide. It never allocates draft entity `code`s and never writes state.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::domain::phase3_artifact::validate_probe_candidates;

/// A canonical world candidate. `temp_key` is stable inside the merged set only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MergedWorldCandidate {
    pub temp_key: String,
    pub parent_temp_key: Option<String>,
    pub existing_parent_code: Option<String>,
    pub display_name: String,
    pub setting_type: String,
    pub description: Option<String>,
    pub access: Option<String>,
}

/// A canonical lore candidate attached to a world candidate or an existing world code.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MergedLoreCandidate {
    pub temp_key: String,
    pub world_temp_key: Option<String>,
    pub existing_world_code: Option<String>,
    pub display_name: String,
    pub lore_type: String,
    pub description: Option<String>,
    pub data: Value,
}

/// Canonical merged output consumed by `world_write`.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MergedCandidates {
    pub world_settings: Vec<MergedWorldCandidate>,
    pub lore_entries: Vec<MergedLoreCandidate>,
}

impl MergedCandidates {
    /// Hierarchical candidate slice for one depth layer, parents before children.
    pub fn world_layer(&self, depth: u32) -> Vec<&MergedWorldCandidate> {
        self.world_settings
            .iter()
            .filter(|candidate| self.depth_of(&candidate.temp_key) == Some(depth))
            .collect()
    }

    /// Depth of a canonical world candidate (`0` = root or attached to an existing node).
    pub fn depth_of(&self, temp_key: &str) -> Option<u32> {
        let by_key: BTreeMap<&str, &MergedWorldCandidate> = self
            .world_settings
            .iter()
            .map(|candidate| (candidate.temp_key.as_str(), candidate))
            .collect();
        let mut current = by_key.get(temp_key)?;
        let mut depth = 0;
        let mut visited = std::collections::BTreeSet::new();
        while let Some(parent_key) = &current.parent_temp_key {
            if !visited.insert(current.temp_key.as_str()) {
                return None;
            }
            current = by_key.get(parent_key.as_str())?;
            depth += 1;
        }
        Some(depth)
    }
}

/// Merge failure taxonomy.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum MergeError {
    #[error("invalid probe report: {report}: {reason}")]
    InvalidReport { report: String, reason: String },

    #[error("ambiguous candidate '{display_name}' ({kind}) from reports {first_report} and {second_report}: payload differs without proof of identity")]
    Ambiguous {
        kind: &'static str,
        display_name: String,
        first_report: String,
        second_report: String,
    },
}

fn normalize_name(raw: &str) -> String {
    raw.trim().to_lowercase()
}

fn optional_string(value: Option<&Value>) -> Option<String> {
    value
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|text| !text.is_empty())
        .map(str::to_string)
}

/// Merge validated reports into one canonical candidate set.
pub fn merge_probe_reports(
    reports: &[(String, String)],
    existing_world_codes: &[String],
) -> Result<MergedCandidates, MergeError> {
    let mut worlds: BTreeMap<String, MergedWorldCandidate> = BTreeMap::new();
    // canonical key -> (report label, original description/access signature)
    // key -> (report, description?, access?)
    let mut world_signatures: BTreeMap<String, (String, Option<String>, Option<String>)> =
        BTreeMap::new();
    let mut lore: BTreeMap<String, MergedLoreCandidate> = BTreeMap::new();
    let mut lore_signatures: BTreeMap<String, (String, Option<String>, Option<Value>)> =
        BTreeMap::new();

    for (report_label, report_text) in reports {
        let report =
            validate_probe_candidates(report_text, existing_world_codes).map_err(|error| {
                MergeError::InvalidReport {
                    report: report_label.clone(),
                    reason: error.to_string(),
                }
            })?;

        // Canonical keys resolve temporary parents to the merged world identity.
        let mut local_to_canonical: BTreeMap<String, String> = BTreeMap::new();
        let owned_worlds = report["worldSettings"]
            .as_array()
            .cloned()
            .unwrap_or_default();
        let mut queue: Vec<Value> = owned_worlds;
        let mut progress = true;
        while let Some(item) = queue.first().cloned() {
            let object = item.as_object().cloned().unwrap_or_default();
            let temp_key = optional_string(object.get("tempKey")).unwrap_or_default();
            let parent_temp = optional_string(object.get("parentTempKey"));
            let display_name = optional_string(object.get("displayName")).unwrap_or_default();
            let setting_type = optional_string(object.get("type")).unwrap_or_default();
            let description = optional_string(object.get("description"));
            let access = optional_string(object.get("access"));
            let existing_parent = optional_string(object.get("existingParentCode"));

            let parent_canonical = match &parent_temp {
                Some(parent) => local_to_canonical.get(parent).cloned(),
                None => None,
            };
            let resolvable = parent_temp.is_none() || parent_canonical.is_some();
            if !resolvable {
                if !progress {
                    return Err(MergeError::InvalidReport {
                        report: report_label.clone(),
                        reason: format!("unresolvable parentTempKey '{parent_temp:?}'"),
                    });
                }
                queue.rotate_left(1);
                progress = false;
                continue;
            }
            progress = true;

            // Identity is the normalized display name + type, not the parent. Probes often
            // describe the same real place once as a root and once as a child of another probe's
            // hierarchy ("Thành phố Hồ Chí Minh" appeared both ways); keying by parent produced
            // duplicate world settings. Parent information is merged below, preferring a resolved
            // parent over a root.
            let key = format!("{}|{}", normalize_name(&display_name), setting_type);
            local_to_canonical.insert(temp_key.clone(), key.clone());

            match world_signatures.get_mut(&key) {
                Some((_first_report, first_description, first_access)) => {
                    // A probe that omits an optional field does not conflict with one that
                    // provides it: keep the known value, only differ when both are known.
                    for (existing, incoming, label) in [
                        (&mut *first_description, &description, "description"),
                        (&mut *first_access, &access, "access"),
                    ] {
                        match (&*existing, incoming) {
                            // Probes describe the same real location with different prose. That is
                            // not an identity conflict: keep the richer description and the first
                            // access value instead of aborting the whole Phase 3 merge (which left
                            // the draft confirmed at Phase 3 with zero world settings).
                            (Some(known), Some(candidate)) if known != candidate
                                && label == "description" && candidate.len() > known.len() => {
                                    *existing = Some(candidate.clone());
                                }
                            (None, Some(candidate)) => *existing = Some(candidate.clone()),
                            _ => {}
                        }
                    }
                    let stored = worlds.get_mut(&key).expect("merged world exists");
                    // A resolved parent wins over a root variant; conflicting resolved parents keep
                    // the first one (the report order is deterministic).
                    if stored.parent_temp_key.is_none() {
                        stored.parent_temp_key = parent_canonical.clone();
                    }
                    if stored.existing_parent_code.is_none() {
                        stored.existing_parent_code = existing_parent.clone();
                    }
                    // The signature keeps the richest description/known access; mirror it into the
                    // candidate that will be written.
                    stored.description = first_description.clone();
                    if stored.access.is_none() {
                        stored.access = first_access.clone();
                    }
                }
                None => {
                    world_signatures.insert(
                        key.clone(),
                        (report_label.clone(), description.clone(), access.clone()),
                    );
                    worlds.entry(key.clone()).or_insert(MergedWorldCandidate {
                        temp_key: key,
                        parent_temp_key: parent_canonical,
                        existing_parent_code: existing_parent,
                        display_name,
                        setting_type,
                        description,
                        access,
                    });
                }
            }

            queue.remove(0);
        }

        let owned_lore = report["loreEntries"]
            .as_array()
            .cloned()
            .unwrap_or_default();
        for item in owned_lore {
            let object = item.as_object().cloned().unwrap_or_default();
            let display_name = optional_string(object.get("displayName")).unwrap_or_default();
            let lore_type = optional_string(object.get("type")).unwrap_or_default();
            let description = optional_string(object.get("description"));
            let data = object
                .get("data")
                .cloned()
                .unwrap_or(Value::Object(Default::default()));
            let parent_temp = optional_string(object.get("worldTempKey"));
            let existing_world = optional_string(object.get("existingWorldCode"));

            let parent_canonical = match &parent_temp {
                Some(parent) => local_to_canonical.get(parent).cloned(),
                None => None,
            };
            if parent_temp.is_some() && parent_canonical.is_none() {
                return Err(MergeError::InvalidReport {
                    report: report_label.clone(),
                    reason: format!("unresolvable lore worldTempKey '{parent_temp:?}'"),
                });
            }
            let key = match (&parent_canonical, &existing_world) {
                (Some(parent), _) => format!(
                    "tmp:{parent}|{}|{}",
                    normalize_name(&display_name),
                    lore_type
                ),
                (None, Some(existing)) => {
                    format!(
                        "ex:{existing}|{}|{}",
                        normalize_name(&display_name),
                        lore_type
                    )
                }
                (None, None) => format!("none|{}|{}", normalize_name(&display_name), lore_type),
            };

            match lore_signatures.get_mut(&key) {
                Some((first_report, first_description, first_data)) => {
                    let empty_data = data
                        .as_object()
                        .map(|object| object.is_empty())
                        .unwrap_or(false);
                    if let Some(known) = &*first_data {
                        if !empty_data && known != &data {
                            return Err(MergeError::Ambiguous {
                                kind: "lore_entry",
                                display_name,
                                first_report: first_report.clone(),
                                second_report: report_label.clone(),
                            });
                        }
                    } else if !empty_data {
                        *first_data = Some(data.clone());
                    }
                    if first_description.is_none() {
                        *first_description = description.clone();
                    }
                    let stored = lore.get_mut(&key).expect("merged lore exists");
                    if stored.description.is_none() {
                        stored.description = description.clone();
                    }
                    if stored.data.is_null()
                        || stored
                            .data
                            .as_object()
                            .map(|o| o.is_empty())
                            .unwrap_or(false)
                    {
                        stored.data = data.clone();
                    }
                }
                None => {
                    lore_signatures.insert(
                        key.clone(),
                        (
                            report_label.clone(),
                            description.clone(),
                            Some(data.clone()),
                        ),
                    );
                    lore.entry(key.clone()).or_insert(MergedLoreCandidate {
                        temp_key: key,
                        world_temp_key: parent_canonical,
                        existing_world_code: existing_world,
                        display_name,
                        lore_type,
                        description,
                        data,
                    });
                }
            }
        }
    }

    Ok(MergedCandidates {
        world_settings: worlds.into_values().collect(),
        lore_entries: lore.into_values().collect(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn report(worlds: &str, lore: &str) -> String {
        format!(r#"{{"schemaVersion":1,"worldSettings":[{worlds}],"loreEntries":[{lore}]}}"#)
    }

    #[test]
    fn test_merge_identical_candidates_collapses_to_one() {
        let worlds = r#"{"tempKey":"w1","displayName":"City","type":"CITY","description":"Port"}"#;
        let reports = vec![
            ("probe-a".to_string(), report(worlds, "")),
            ("probe-b".to_string(), report(worlds, "")),
        ];
        let merged = merge_probe_reports(&reports, &[]).expect("identical candidates must merge");
        assert_eq!(merged.world_settings.len(), 1);
        assert_eq!(merged.world_settings[0].display_name, "City");
    }

    #[test]
    fn test_merge_keeps_richest_description_for_the_same_location() {
        // Independent chapter probes naturally describe "City" differently. The merger must keep
        // one candidate (longest description) instead of failing Phase 3 entirely.
        let first = r#"{"tempKey":"w1","displayName":"City","type":"CITY","description":"Port"}"#;
        let second = r#"{"tempKey":"w2","displayName":"City","type":"CITY","description":"Port and mountain district"}"#;
        let reports = vec![
            ("probe-a".to_string(), report(first, "")),
            ("probe-b".to_string(), report(second, "")),
        ];
        let merged = merge_probe_reports(&reports, &[]).expect("same location must merge");
        assert_eq!(merged.world_settings.len(), 1);
        assert_eq!(merged.world_settings[0].display_name, "City");
        assert_eq!(
            merged.world_settings[0].description.as_deref(),
            Some("Port and mountain district")
        );
    }

    #[test]
    fn test_merge_collapses_root_and_child_variants_of_the_same_place() {
        // Same real location described as a root by one probe and as a child by another.
        let root = r#"{"tempKey":"hcm-root","displayName":"Thành phố Hồ Chí Minh","type":"CITY","description":"Southern hub"}"#;
        let child = r#"{"tempKey":"vn","displayName":"Việt Nam","type":"COUNTRY"},{"tempKey":"hcm-child","parentTempKey":"vn","displayName":"Thành phố Hồ Chí Minh","type":"CITY","description":"Southern hub and startup district"}"#;
        let reports = vec![
            ("probe-a".to_string(), report(root, "")),
            ("probe-b".to_string(), report(child, "")),
        ];
        let merged = merge_probe_reports(&reports, &[]).expect("root/child variants must merge");
        assert_eq!(
            merged.world_settings.len(),
            2,
            "Vietnam + one Ho Chi Minh City"
        );
        let hcm = merged
            .world_settings
            .iter()
            .find(|candidate| candidate.display_name == "Thành phố Hồ Chí Minh")
            .unwrap();
        assert!(
            hcm.parent_temp_key.is_some(),
            "resolved parent must win over root"
        );
        assert_eq!(
            hcm.description.as_deref(),
            Some("Southern hub and startup district")
        );
    }

    #[test]
    fn test_merge_preserves_parent_chain_and_lore_attachment() {
        let worlds = r#"{"tempKey":"w1","displayName":"Realm","type":"REALM"},{"tempKey":"w2","parentTempKey":"w1","displayName":"City","type":"CITY"}"#;
        let lore = r#"{"tempKey":"l1","worldTempKey":"w2","displayName":"Fire","type":"HISTORY","data":{}}"#;
        let merged = merge_probe_reports(&[("probe-a".to_string(), report(worlds, lore))], &[])
            .expect("valid report merges");
        assert_eq!(merged.world_settings.len(), 2);
        assert_eq!(merged.lore_entries.len(), 1);
        let city = merged
            .world_settings
            .iter()
            .find(|candidate| candidate.display_name == "City")
            .unwrap();
        assert_eq!(merged.depth_of(&city.temp_key), Some(1));
        assert_eq!(
            merged.lore_entries[0].world_temp_key.as_deref(),
            Some(city.temp_key.as_str())
        );
        assert_eq!(merged.world_layer(0).len(), 1);
        assert_eq!(merged.world_layer(1).len(), 1);
    }
}
