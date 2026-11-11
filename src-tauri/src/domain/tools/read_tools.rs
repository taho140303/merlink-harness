//! Read Tools & Catalog Query Interface (`REQ-TOOLS-003`, `be-core.md` §5.1, §6.2, `harness/tools.md` §3.1).
//!
//! Provides read-only query tools:
//! 1. `get_field_null`: returns up to 3 null fields with deterministic fixed questions in `system_language`
//!    (strictly from backend trusted settings, AI cannot supply language; fallback to `"en"`).
//! 2. `get_array_element_fields`: distinguishes between empty array `[]` and missing/null fields (`P3S4-B`),
//!    returning priority creation fields when `code` is None or null fields of an existing element when `code` is Some.
//! 3. `get_state`: read-only working state snapshot with provenance and pending patches.
//! 4. Catalog search queries (`search_genres`, `search_motifs`, `search_lore`, `search_world_settings`):
//!    JIT retrieval via `CatalogRepo`. If an unknown code is queried, returns structured candidates
//!    without state mutation (`phase3_sprint4_deep1-F1`).
//! 5. Domain Invariant: Read tools NEVER mutate working state or create draft versions (`REQ-TOOLS-003`).

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::domain::business_flow::BusinessFlowData;
use crate::error::ToolRejection;

// ============================================================================
// CatalogRepo Interface & Records (matching P2S6 contracts)
// ============================================================================

/// Canonical error type for catalog queries.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum CatalogRepoError {
    #[error("Missing schema or table: {0}")]
    MissingSchema(String),

    #[error("Catalog entity not found: {0}")]
    NotFound(String),

    #[error("Database error: {0}")]
    Sqlite(String),

    #[error("Invalid data in catalog: {0}")]
    InvalidData(String),
}

/// Motif reference record (`motifs` table, docs/db.dbml:14).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MotifRecord {
    pub id: String,
    pub code: Option<String>,
    pub display_name: Option<String>,
    pub category: Option<String>,
    pub version: Option<String>,
    pub is_system: bool,
    pub description: Option<String>,
    pub min_chapter: Option<i64>,
    pub max_chapter: Option<i64>,
}

/// Genre reference record (`genres` table, docs/db.dbml:34).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GenreRecord {
    pub id: String,
    pub code: Option<String>,
    pub display_name: Option<String>,
    pub description: Option<String>,
}

/// Character reference record (`characters` table, docs/db.dbml:415).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CharacterRecord {
    pub id: String,
    pub code: Option<String>,
    pub display_name: Option<String>,
    pub nickname: Option<String>,
    pub birth: Option<String>,
    pub species: Option<String>,
    pub biological_sex: Option<String>,
    pub gender_identity: Option<String>,
    pub gender_expression: Option<String>,
    pub deleted_at: Option<String>,
}

/// World setting reference record (`world_settings` table, docs/db.dbml:336).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldSettingRecord {
    pub id: String,
    pub code: String,
    pub project_id: Option<String>,
    pub display_name: String,
    pub setting_type: String,
    pub parent_id: Option<String>,
    pub description: Option<String>,
    pub access: Option<String>,
    pub spatial_features: Option<String>,
    pub sensory_visual: Option<String>,
    pub sensory_auditory: Option<String>,
    pub sensory_tactile: Option<String>,
    pub sensory_olfactory: Option<String>,
    pub sensory_gustatory: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub deleted_at: Option<String>,
}

/// Lore entry reference record (`lore_entries` table, docs/db.dbml:363).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LoreEntryRecord {
    pub id: String,
    pub code: String,
    pub entry_type: String,
    pub display_name: String,
    pub description: Option<String>,
    pub world_setting_id: Option<String>,
    pub data: Option<String>,
    pub deleted_at: Option<String>,
}

/// Trait abstracting read queries on static/reference catalog entities (`be-core.md` §1.4, §6.2).
pub trait CatalogRepo {
    fn get_motif_by_code(&self, code: &str) -> Result<Option<MotifRecord>, CatalogRepoError>;
    fn list_motifs(&self, category: Option<&str>) -> Result<Vec<MotifRecord>, CatalogRepoError>;

    fn get_genre_by_code(&self, code: &str) -> Result<Option<GenreRecord>, CatalogRepoError>;
    fn list_genres(&self) -> Result<Vec<GenreRecord>, CatalogRepoError>;

    fn get_character_by_code(
        &self,
        code: &str,
    ) -> Result<Option<CharacterRecord>, CatalogRepoError>;
    fn list_characters(
        &self,
        include_deleted: bool,
    ) -> Result<Vec<CharacterRecord>, CatalogRepoError>;

    fn get_world_setting_by_code(
        &self,
        code: &str,
    ) -> Result<Option<WorldSettingRecord>, CatalogRepoError>;
    fn list_world_settings(
        &self,
        project_id: Option<&str>,
        include_deleted: bool,
    ) -> Result<Vec<WorldSettingRecord>, CatalogRepoError>;

    fn get_lore_entry_by_code(
        &self,
        code: &str,
    ) -> Result<Option<LoreEntryRecord>, CatalogRepoError>;
    fn list_lore_entries(
        &self,
        world_setting_id: Option<&str>,
        include_deleted: bool,
    ) -> Result<Vec<LoreEntryRecord>, CatalogRepoError>;
}

// ============================================================================
// Tool 1: get_field_null
// ============================================================================

/// Pair of field path and fixed question in the active system language (`prompts/overview.md` §2.1).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FieldNullQuestion {
    pub field: String,
    pub question: String,
}

/// Returns up to 3 null/missing fields with deterministic questions (`REQ-TOOLS-003`, `P3S4-P`, `phase3_sprint4_deep1-F2`).
///
/// Invariants:
/// - Max questions returned is capped at 3 (`limit.unwrap_or(3).min(3)`).
/// - `system_language` is obtained from the trusted UI settings boundary, never from model args.
/// - If `system_language` is missing, unknown, or unsupported, cleanly falls back to `"en"`.
/// - Empty arrays `[]` are NOT reported as null fields (`core.md` §3.1, `P3S4-B`).
pub fn get_field_null(
    data: &BusinessFlowData,
    limit: Option<usize>,
    system_language: &str,
) -> Vec<FieldNullQuestion> {
    let effective_limit = limit.unwrap_or(3).min(3);
    if effective_limit == 0 {
        return Vec::new();
    }

    let is_vi = system_language.trim().eq_ignore_ascii_case("vi");

    let mut questions = Vec::new();

    // 1. require.project.display_name
    if data.require.project.display_name.value.is_none() {
        questions.push(FieldNullQuestion {
            field: "require.project.display_name".to_string(),
            question: if is_vi {
                "Bạn muốn đặt tên truyện là gì?".to_string()
            } else {
                "What is the title of your story?".to_string()
            },
        });
    }

    // 2. require.project.summary
    if data.require.project.summary.value.is_none() {
        questions.push(FieldNullQuestion {
            field: "require.project.summary".to_string(),
            question: if is_vi {
                "Tóm tắt nội dung chính của truyện là gì?".to_string()
            } else {
                "What is the summary of the story?".to_string()
            },
        });
    }

    // 3. require.project.language
    if data
        .require
        .project
        .language
        .value
        .as_deref()
        .map_or(true, |value| value.trim().is_empty())
        || !matches!(
            data.require.project.language.source,
            Some(
                crate::domain::business_flow::ProvenanceSource::User
                    | crate::domain::business_flow::ProvenanceSource::UserEdited
                    | crate::domain::business_flow::ProvenanceSource::AiGenerated
            )
        )
    {
        questions.push(FieldNullQuestion {
            field: "require.project.language".to_string(),
            question: if is_vi {
                "Truyện được viết bằng ngôn ngữ nào?".to_string()
            } else {
                "What language is the story written in?".to_string()
            },
        });
    }

    // 4. require.project.target.word_per_chapter_count
    if data
        .require
        .project
        .target
        .word_per_chapter_count
        .value
        .is_none()
    {
        questions.push(FieldNullQuestion {
            field: "require.project.target.word_per_chapter_count".to_string(),
            question: if is_vi {
                "Trung bình mỗi chương truyện có bao nhiêu từ?".to_string()
            } else {
                "How many words on average per chapter?".to_string()
            },
        });
    }

    // 5. require.project.target.chapter_count
    if data.require.project.target.chapter_count.value.is_none() {
        questions.push(FieldNullQuestion {
            field: "require.project.target.chapter_count".to_string(),
            question: if is_vi {
                "Tổng số chương dự kiến của truyện là bao nhiêu?".to_string()
            } else {
                "What is the total planned chapter count?".to_string()
            },
        });
    }

    // 6. require.project.target.arc_count
    if data.require.project.target.arc_count.value.is_none() {
        questions.push(FieldNullQuestion {
            field: "require.project.target.arc_count".to_string(),
            question: if is_vi {
                "Truyện dự kiến gồm bao nhiêu Arc (phần lớn)?".to_string()
            } else {
                "How many arcs are planned for the story?".to_string()
            },
        });
    }

    questions.truncate(effective_limit);
    questions
}

// ============================================================================
// Tool 2: get_array_element_fields
// ============================================================================

/// Pair of element field and question for array inspection (`harness/tools.md` §3.1).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArrayElementFieldQuestion {
    pub field: String,
    pub question: String,
}

/// Returns priority fields to add an element or missing fields of an existing element (`P3S4-B`).
///
/// Invariants:
/// - Distinguishes between empty array `[]` (valid array with 0 items) and missing fields (`P3S4-B`).
/// - When `code` is None: returns prioritized fields required to create a new element in the array.
/// - When `code` is Some: searches for the element by code and returns its null fields.
/// - Returns `ToolRejection` with structured candidates if the code does not exist.
pub fn get_array_element_fields(
    data: &BusinessFlowData,
    array_path: &str,
    code: Option<&str>,
    limit: Option<usize>,
    system_language: &str,
) -> Result<Vec<ArrayElementFieldQuestion>, ToolRejection> {
    let effective_limit = limit.unwrap_or(3).min(3);
    let is_vi = system_language.trim().eq_ignore_ascii_case("vi");

    match array_path {
        "require.character" | "character" | "characters" => {
            if let Some(target_code) = code {
                let found = data
                    .require
                    .character
                    .iter()
                    .find(|c| c.code == target_code);

                match found {
                    Some(char_item) => {
                        let mut questions = Vec::new();
                        if char_item
                            .display_name
                            .value
                            .as_deref()
                            .map_or(true, |value| value.trim().is_empty())
                            && char_item
                                .nickname
                                .value
                                .as_deref()
                                .map_or(true, |value| value.trim().is_empty())
                        {
                            questions.push(ArrayElementFieldQuestion {
                                field: "display_name".to_string(),
                                question: if is_vi {
                                    "Tên hiển thị của nhân vật là gì?".to_string()
                                } else {
                                    "What is the character's display name?".to_string()
                                },
                            });
                        }
                        if char_item.biological_sex.value.is_none() {
                            questions.push(ArrayElementFieldQuestion {
                                field: "biological_sex".to_string(),
                                question: if is_vi {
                                    "Giới tính sinh học của nhân vật là gì?".to_string()
                                } else {
                                    "What is the character's biological sex?".to_string()
                                },
                            });
                        }
                        if char_item.motif_code.value.is_none() {
                            questions.push(ArrayElementFieldQuestion {
                                field: "motif_code".to_string(),
                                question: if is_vi {
                                    "Mô-típ tính cách (motif) của nhân vật là gì?".to_string()
                                } else {
                                    "What is the character's archetype motif code?".to_string()
                                },
                            });
                        }
                        questions.truncate(effective_limit);
                        Ok(questions)
                    }
                    None => {
                        let candidates = data
                            .require
                            .character
                            .iter()
                            .map(|c| c.code.clone())
                            .collect::<Vec<_>>();
                        Err(ToolRejection::with_candidates(
                            "character_not_found",
                            Value::String(target_code.to_string()),
                            candidates,
                        ))
                    }
                }
            } else {
                // Priority fields to create a new character
                let mut questions = vec![
                    ArrayElementFieldQuestion {
                        field: "display_name".to_string(),
                        question: if is_vi {
                            "Tên của nhân vật mới là gì?".to_string()
                        } else {
                            "What is the new character's name?".to_string()
                        },
                    },
                    ArrayElementFieldQuestion {
                        field: "biological_sex".to_string(),
                        question: if is_vi {
                            "Giới tính sinh học của nhân vật là gì?".to_string()
                        } else {
                            "What is the character's biological sex?".to_string()
                        },
                    },
                    ArrayElementFieldQuestion {
                        field: "motif_code".to_string(),
                        question: if is_vi {
                            "Mô-típ của nhân vật là gì?".to_string()
                        } else {
                            "What is the character's motif?".to_string()
                        },
                    },
                ];
                questions.truncate(effective_limit);
                Ok(questions)
            }
        }
        "require.project.genres" | "genres" | "project_genres" => {
            if let Some(target_code) = code {
                let found = data
                    .require
                    .project
                    .genres
                    .iter()
                    .find(|g| g.code.value.as_deref() == Some(target_code));

                match found {
                    Some(genre_item) => {
                        let mut questions = Vec::new();
                        if genre_item.is_primary.value.is_none() {
                            questions.push(ArrayElementFieldQuestion {
                                field: "is_primary".to_string(),
                                question: if is_vi {
                                    "Thể loại này có phải là thể loại chính không?".to_string()
                                } else {
                                    "Is this the primary genre of the project?".to_string()
                                },
                            });
                        }
                        questions.truncate(effective_limit);
                        Ok(questions)
                    }
                    None => {
                        let candidates = data
                            .require
                            .project
                            .genres
                            .iter()
                            .filter_map(|g| g.code.value.clone())
                            .collect::<Vec<_>>();
                        Err(ToolRejection::with_candidates(
                            "genre_item_not_found",
                            Value::String(target_code.to_string()),
                            candidates,
                        ))
                    }
                }
            } else {
                let mut questions = vec![
                    ArrayElementFieldQuestion {
                        field: "code".to_string(),
                        question: if is_vi {
                            "Bạn muốn truyện thuộc thể loại nào?".to_string()
                        } else {
                            "What genre would you like for the story?".to_string()
                        },
                    },
                    ArrayElementFieldQuestion {
                        field: "is_primary".to_string(),
                        question: if is_vi {
                            "Thể loại này có phải thể loại chính của truyện không?".to_string()
                        } else {
                            "Is this the primary genre of the story?".to_string()
                        },
                    },
                ];
                questions.truncate(effective_limit);
                Ok(questions)
            }
        }
        "optional.overview_phase.arcs" | "arcs" => {
            if let Some(target_code) = code {
                let found = data
                    .optional
                    .overview_phase
                    .arcs
                    .iter()
                    .find(|a| a.code == target_code);

                match found {
                    Some(arc_item) => {
                        let mut questions = Vec::new();
                        if arc_item.display_name.value.is_none() {
                            questions.push(ArrayElementFieldQuestion {
                                field: "display_name".to_string(),
                                question: if is_vi {
                                    "Tên của Arc này là gì?".to_string()
                                } else {
                                    "What is the title of this arc?".to_string()
                                },
                            });
                        }
                        if arc_item.summary.value.is_none() {
                            questions.push(ArrayElementFieldQuestion {
                                field: "summary".to_string(),
                                question: if is_vi {
                                    "Tóm tắt nội dung chính của Arc là gì?".to_string()
                                } else {
                                    "What is the summary of this arc?".to_string()
                                },
                            });
                        }
                        if arc_item.position.value.is_none() {
                            questions.push(ArrayElementFieldQuestion {
                                field: "position".to_string(),
                                question: if is_vi {
                                    "Thứ tự của Arc trong tác phẩm là bao nhiêu?".to_string()
                                } else {
                                    "What is the position index of this arc?".to_string()
                                },
                            });
                        }
                        questions.truncate(effective_limit);
                        Ok(questions)
                    }
                    None => {
                        let candidates = data
                            .optional
                            .overview_phase
                            .arcs
                            .iter()
                            .map(|a| a.code.clone())
                            .collect::<Vec<_>>();
                        Err(ToolRejection::with_candidates(
                            "arc_not_found",
                            Value::String(target_code.to_string()),
                            candidates,
                        ))
                    }
                }
            } else {
                let mut questions = vec![
                    ArrayElementFieldQuestion {
                        field: "display_name".to_string(),
                        question: if is_vi {
                            "Tên của Arc mới là gì?".to_string()
                        } else {
                            "What is the title of the new arc?".to_string()
                        },
                    },
                    ArrayElementFieldQuestion {
                        field: "summary".to_string(),
                        question: if is_vi {
                            "Tóm tắt cốt truyện của Arc mới là gì?".to_string()
                        } else {
                            "What is the summary of the new arc?".to_string()
                        },
                    },
                    ArrayElementFieldQuestion {
                        field: "position".to_string(),
                        question: if is_vi {
                            "Thứ tự xuất hiện của Arc là số mấy?".to_string()
                        } else {
                            "What is the position index of the new arc?".to_string()
                        },
                    },
                ];
                questions.truncate(effective_limit);
                Ok(questions)
            }
        }
        _ => Err(ToolRejection::new(format!(
            "unsupported_or_unknown_array_path: {array_path}"
        ))),
    }
}

// ============================================================================
// Tool 3: get_state
// ============================================================================

/// Structured output returned by `get_state` (`harness/tools.md` §3.1).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StateSnapshotOutput {
    pub status: String,
    pub target_type: Option<String>,
    pub code: Option<String>,
    pub state: Value,
    pub pending_patches: Vec<Value>,
}

/// Reads the working state snapshot or a specific entity slice (`REQ-TOOLS-003`).
///
/// Invariant: Pure read-only operation; never creates draft versions.
pub fn get_state(
    data: &BusinessFlowData,
    target_type: Option<&str>,
    code: Option<&str>,
) -> StateSnapshotOutput {
    let state_value = match target_type {
        None => serde_json::to_value(data).unwrap_or(Value::Null),
        Some("project") => serde_json::to_value(&data.require.project).unwrap_or(Value::Null),
        Some("character") => {
            if let Some(target_code) = code {
                let found = data
                    .require
                    .character
                    .iter()
                    .find(|c| c.code == target_code);
                serde_json::to_value(found).unwrap_or(Value::Null)
            } else {
                serde_json::to_value(&data.require.character).unwrap_or(Value::Null)
            }
        }
        Some("arc") => {
            if let Some(target_code) = code {
                let found = data
                    .optional
                    .overview_phase
                    .arcs
                    .iter()
                    .find(|a| a.code == target_code);
                serde_json::to_value(found).unwrap_or(Value::Null)
            } else {
                serde_json::to_value(&data.optional.overview_phase.arcs).unwrap_or(Value::Null)
            }
        }
        Some(other) => Value::String(format!("unrecognized_target_type: {other}")),
    };

    StateSnapshotOutput {
        status: "applied".to_string(),
        target_type: target_type.map(str::to_string),
        code: code.map(str::to_string),
        state: state_value,
        pending_patches: Vec::new(),
    }
}

// ============================================================================
// Tool 4: Catalog Search Tools (genres, motifs, lore, world_settings)
// ============================================================================

/// Result container for catalog search tools (`prompts/overview.md` §2.1).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchOutput<T> {
    pub status: String,
    pub items: Vec<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub candidates: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Searches reference genre catalog via `CatalogRepo` (`REQ-TOOLS-003`, `phase3_sprint4_deep1-F1`).
///
/// If an unknown code or non-matching query is passed, returns structured candidates without mutation.
pub fn search_genres(
    catalog: &dyn CatalogRepo,
    query: Option<&str>,
    limit: Option<usize>,
) -> Result<SearchOutput<GenreRecord>, CatalogRepoError> {
    let all = catalog.list_genres()?;
    let effective_limit = limit.unwrap_or(10);

    let filtered: Vec<GenreRecord> = if let Some(q) = query {
        let q_clean = q.trim().to_lowercase();
        all.iter()
            .filter(|g| {
                g.code
                    .as_deref()
                    .map(|c| c.to_lowercase().contains(&q_clean))
                    .unwrap_or(false)
                    || g.display_name
                        .as_deref()
                        .map(|d| d.to_lowercase().contains(&q_clean))
                        .unwrap_or(false)
            })
            .cloned()
            .collect()
    } else {
        all.clone()
    };

    if filtered.is_empty() {
        let candidates = all
            .into_iter()
            .filter_map(|g| g.code)
            .take(effective_limit)
            .collect();

        Ok(SearchOutput {
            status: "rejected".to_string(),
            items: Vec::new(),
            candidates: Some(candidates),
            error: Some("unknown_genre".to_string()),
        })
    } else {
        let mut items = filtered;
        items.truncate(effective_limit);
        Ok(SearchOutput {
            status: "applied".to_string(),
            items,
            candidates: None,
            error: None,
        })
    }
}

/// Searches reference motif catalog via `CatalogRepo` (`REQ-TOOLS-003`, `phase3_sprint4_deep1-F1`).
pub fn search_motifs(
    catalog: &dyn CatalogRepo,
    category: Option<&str>,
    query: Option<&str>,
    limit: Option<usize>,
) -> Result<SearchOutput<MotifRecord>, CatalogRepoError> {
    let all = catalog.list_motifs(category)?;
    let effective_limit = limit.unwrap_or(10);

    let filtered: Vec<MotifRecord> = if let Some(q) = query {
        let q_clean = q.trim().to_lowercase();
        all.iter()
            .filter(|m| {
                m.code
                    .as_deref()
                    .map(|c| c.to_lowercase().contains(&q_clean))
                    .unwrap_or(false)
                    || m.display_name
                        .as_deref()
                        .map(|d| d.to_lowercase().contains(&q_clean))
                        .unwrap_or(false)
            })
            .cloned()
            .collect()
    } else {
        all.clone()
    };

    if filtered.is_empty() {
        let candidates = all
            .into_iter()
            .filter_map(|m| m.code)
            .take(effective_limit)
            .collect();

        Ok(SearchOutput {
            status: "rejected".to_string(),
            items: Vec::new(),
            candidates: Some(candidates),
            error: Some("unknown_motif".to_string()),
        })
    } else {
        let mut items = filtered;
        items.truncate(effective_limit);
        Ok(SearchOutput {
            status: "applied".to_string(),
            items,
            candidates: None,
            error: None,
        })
    }
}

/// Searches world settings in catalog via `CatalogRepo` (`REQ-TOOLS-003`, `phase3_sprint4_deep1-F1`).
pub fn search_world_settings(
    catalog: &dyn CatalogRepo,
    project_id: Option<&str>,
    query: Option<&str>,
    limit: Option<usize>,
) -> Result<SearchOutput<WorldSettingRecord>, CatalogRepoError> {
    let all = catalog.list_world_settings(project_id, false)?;
    let effective_limit = limit.unwrap_or(10);

    let filtered: Vec<WorldSettingRecord> = if let Some(q) = query {
        let q_clean = q.trim().to_lowercase();
        all.iter()
            .filter(|w| {
                w.code.to_lowercase().contains(&q_clean)
                    || w.display_name.to_lowercase().contains(&q_clean)
            })
            .cloned()
            .collect()
    } else {
        all.clone()
    };

    if filtered.is_empty() {
        let candidates = all
            .into_iter()
            .map(|w| w.code)
            .take(effective_limit)
            .collect();

        Ok(SearchOutput {
            status: "rejected".to_string(),
            items: Vec::new(),
            candidates: Some(candidates),
            error: Some("unknown_world_setting".to_string()),
        })
    } else {
        let mut items = filtered;
        items.truncate(effective_limit);
        Ok(SearchOutput {
            status: "applied".to_string(),
            items,
            candidates: None,
            error: None,
        })
    }
}

/// Searches lore entries in catalog via `CatalogRepo` (`REQ-TOOLS-003`, `phase3_sprint4_deep1-F1`).
pub fn search_lore(
    catalog: &dyn CatalogRepo,
    world_setting_id: Option<&str>,
    query: Option<&str>,
    limit: Option<usize>,
) -> Result<SearchOutput<LoreEntryRecord>, CatalogRepoError> {
    let all = catalog.list_lore_entries(world_setting_id, false)?;
    let effective_limit = limit.unwrap_or(10);

    let filtered: Vec<LoreEntryRecord> = if let Some(q) = query {
        let q_clean = q.trim().to_lowercase();
        all.iter()
            .filter(|l| {
                l.code.to_lowercase().contains(&q_clean)
                    || l.display_name.to_lowercase().contains(&q_clean)
            })
            .cloned()
            .collect()
    } else {
        all.clone()
    };

    if filtered.is_empty() {
        let candidates = all
            .into_iter()
            .map(|l| l.code)
            .take(effective_limit)
            .collect();

        Ok(SearchOutput {
            status: "rejected".to_string(),
            items: Vec::new(),
            candidates: Some(candidates),
            error: Some("unknown_lore_entry".to_string()),
        })
    } else {
        let mut items = filtered;
        items.truncate(effective_limit);
        Ok(SearchOutput {
            status: "applied".to_string(),
            items,
            candidates: None,
            error: None,
        })
    }
}

/// Resolves a single catalog code or returns structured candidates with rejection (`phase3_sprint4_deep1-F1`).
pub fn resolve_catalog_genre(
    catalog: &dyn CatalogRepo,
    code: &str,
) -> Result<Result<GenreRecord, ToolRejection>, CatalogRepoError> {
    match catalog.get_genre_by_code(code)? {
        Some(record) => Ok(Ok(record)),
        None => {
            let candidates = catalog
                .list_genres()?
                .into_iter()
                .filter_map(|g| g.code)
                .take(5)
                .collect::<Vec<_>>();
            Ok(Err(ToolRejection::with_candidates(
                "unknown_genre",
                Value::String(code.to_string()),
                candidates,
            )))
        }
    }
}
